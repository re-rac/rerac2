//! Tabora's geysers 0x08f6..0x08f9 (`func_L08_003CCD80`, L08 `0x3ce1d0`, one update for the four): a column on the
//! liquid grid (crate::water::grid) that sprays while the liquid runs and caps over when it freezes. The placed one is
//! 0x08f9 (2 placed); at its start it makes its three layers 0x08f8, 0x08f7 and 0x08f6 at its place (`0x3ce080`:
//! [`layer`]).
//!
//! The pvars: `+0x00` the grid's mover it stands on (a moby; −1 none), `+0x04` its texture's turn speed, `+0x06` the
//! turn, `+0x08` the cap (0 running, 1 capped), `+0x0c` the mover's height, `+0x10..+0x18` the master's layers, `+0x1c`
//! capped, `+0x1d` the texture turns, `+0x1e` its alpha when running, `+0x1f` the master's loop's slot.
//!
//! * **0** (once): 0x08f9: the turn speed 200 (`$gp+0xee4`), the alpha 0xff, the cap `+0x1c`'s, and its three layers
//!   (speeds 270 / 240 / 300, alphas 0x80 / 0x70 / 0x80, `$gp+0xee6..`). Every class: the mover's height
//!   (`0x3ce1a8`: its z, 0 with none), 0.19 up (`$gp+0xf88`), mode | 0x2008, no slot, 1.
//! * **Every tick**: its z a tenth of the way to the mover's height + 0.2 + 0.19 × (1 − cap). The mover frozen
//!   (`0x3ce178`: its `+0x14`; crate::water::grid) and not capped: the cap up 0.01 (`$gp+0xef4`) a tick, capped at 1;
//!   thawed and capped: down, running at 0. Its alpha (`+0x23`) `+0x1e + (255 − +0x1e) × cap`.
//! * **Not fully capped**: with the turns on, its class's texture turned by `−speed × (1 − cap)` in 1/4096
//!   (`0x31ea80` = L00 `0x311a80`, crate::moby::MobyWorld::scroll_class; the turn kept in 0..4096); one time in
//!   `16 + 20·cap`, a splash: a point `±2` along its first row and `±0.8` along its second from it, at the mover's
//!   height + 0.21, a drop in the liquid there (`0x3d1468` → `0x38c308`: crate::water::grid::Grid::disturb on the
//!   level's seven records, radius 2..3, height 0.2..0.4, kept) and a ring on it (`0x3484c0` = L00 `0x339b60`,
//!   crate::particles::water::ring: size 2..3, growing 13000).
//! * **Running** (not capped): no collision (`+0x98` = 0); 0x08f9 keeps its loop (class sound 0, looped: `0x36eb38`
//!   = L00 `0x3605f0`) and puffs every tick (crate::particles::staged: from a point 0.5 up and ±3 / ±1 along its
//!   rows, or half the time one toward the camera in its frame (x × −1..3.5, y × 0..3, z 0..0.5), drifting up to
//!   0.01 a second either way and 0.1..0.2 up, then 0..1 up; 0x80f0f0a0 → 0x00f0f0a0, sizes 1..2 → 2..4; 10 ticks in,
//!   40 eased, none out). **Capped**: solid (its class's collision); 0x08f9's loop stopped (`0x36e838` = L00
//!   `0x3602f0`).

use crate::hero::physics::{add, set_len, sub, V3};
use crate::moby::{Ctx, MobyWorld};

pub const MASTER: i16 = 0x08f9;
pub const CLASSES: [i16; 4] = [0x08f6, 0x08f7, 0x08f8, MASTER];

mod pv {
    pub const MOVER: usize = 0x00;
    pub const SPEED: usize = 0x04;
    pub const TURN: usize = 0x06;
    pub const CAP: usize = 0x08;
    pub const HEIGHT: usize = 0x0c;
    pub const LAYERS: usize = 0x10;
    pub const CAPPED: usize = 0x1c;
    pub const TURNS: usize = 0x1d;
    pub const ALPHA: usize = 0x1e;
    pub const SLOT: usize = 0x1f;
}

/// The level's words (`$gp+0xee4..`, L08).
const SPEED: i16 = 200;
const ALPHA: u8 = 0xff;
/// The layers: class, turn speed, alpha.
const LAYERS: [(i16, i16, u8); 3] = [(0x08f8, 270, 0x80), (0x08f7, 240, 0x70), (0x08f6, 300, 0x80)];
const STEP: u32 = 0x3c23_d70a;
const RISE: u32 = 0x3e42_8f5c;
const SPLASH_Z: u32 = 0x3e57_0a3d;
/// The grid's records on L08 (crate::water::grid's site: 7 at `0x2a0d80`).
const RECORDS: usize = 7;

/// The mover's height (`0x3ce1a8`).
fn height(w: &MobyWorld, mover: i32) -> f32 {
    if mover == -1 { return 0.0; }
    usize::try_from(mover).ok().and_then(|i| w.mobys.get(i)).map_or(0.0, |m| m.pos[2])
}

/// The mover frozen (`0x3ce178`).
fn frozen(w: &MobyWorld, mover: i32) -> bool {
    if mover == -1 { return false; }
    usize::try_from(mover).ok().and_then(|i| w.mobys.get(i)).is_some_and(|m| m.pv_b(0x14) != 0)
}

/// `0x3ce080(master, class, speed, capped, mover, alpha, turns)`: a layer made at the master's place, drawn, its
/// update and draw distances the master's, its yaw alone of its angles, its pvars set so, rebuilt, state 0.
#[allow(clippy::too_many_arguments)]
fn layer(w: &mut MobyWorld, ctx: &mut Ctx, master: usize, class: i16, speed: i16, capped: u8, mover: i32, alpha: u8, turns: u8) -> usize {
    let d = w.create(class);
    let s = w.mobys[master].clone();
    let m = &mut w.mobys[d];
    m.update_dist = s.update_dist;
    m.draw_dist = s.draw_dist;
    m.drawn = 1;
    m.pos = s.pos;
    m.rot = [0.0, 0.0, s.rot[2]];
    m.set_pv_s(pv::SPEED, speed);
    m.set_pv_b(pv::CAPPED, capped);
    m.set_pv_f(pv::CAP, capped as f32);
    m.set_pv_b(pv::TURNS, turns);
    m.set_pv_i(pv::MOVER, mover);
    m.set_pv_b(pv::ALPHA, alpha);
    m.set_pv_s(pv::TURN, 0);
    w.rebuild(d, ctx.host);
    w.mobys[d].state = 0;
    d
}

/// `0x3ce1d0` (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let class = w.mobys[id].o_class;
    if w.mobys[id].state == 0 {
        if class == MASTER {
            let m = &mut w.mobys[id];
            m.set_pv_s(pv::SPEED, SPEED);
            m.set_pv_b(pv::ALPHA, ALPHA);
            let capped = m.pv_b(pv::CAPPED);
            m.set_pv_f(pv::CAP, capped as f32);
            let (mover, turns) = (m.pv_i(pv::MOVER), m.pv_b(pv::TURNS));
            for (k, (c, speed, alpha)) in LAYERS.into_iter().enumerate() {
                let d = if w.class(c).is_some() { layer(w, ctx, id, c, speed, capped, mover, alpha, turns) as i32 } else { 0 };
                w.mobys[id].set_pv_i(pv::LAYERS + 4 * k, d);
            }
        }
        let h = height(w, w.mobys[id].pv_i(pv::MOVER));
        let m = &mut w.mobys[id];
        m.set_pv_f(pv::HEIGHT, h);
        m.pos[2] += f32::from_bits(RISE);
        m.mode |= 0x2008;
        m.set_pv_b(pv::SLOT, 0xff);
        m.state = 1;
    }
    let mover = w.mobys[id].pv_i(pv::MOVER);
    let ice = frozen(w, mover);
    let m = &mut w.mobys[id];
    let cap = m.pv_f(pv::CAP);
    m.pos[2] += ((m.pv_f(pv::HEIGHT) + 0.2 + f32::from_bits(RISE) * (1.0 - cap)) - m.pos[2]) * 0.1;
    if ice {
        if m.pv_b(pv::CAPPED) == 0 {
            let c = m.pv_f(pv::CAP) + f32::from_bits(STEP);
            m.set_pv_f(pv::CAP, c);
            if 1.0 < c {
                m.set_pv_f(pv::CAP, 1.0);
                m.set_pv_b(pv::CAPPED, 1);
            }
        }
    } else if m.pv_b(pv::CAPPED) != 0 {
        let c = m.pv_f(pv::CAP) - f32::from_bits(STEP);
        m.set_pv_f(pv::CAP, c);
        if c < 0.0 {
            m.set_pv_f(pv::CAP, 0.0);
            m.set_pv_b(pv::CAPPED, 0);
        }
    }
    let a = m.pv_b(pv::ALPHA);
    let cap = m.pv_f(pv::CAP);
    m.alpha = a.wrapping_add(((255 - a as i32) as f32 * cap) as i32 as u8);
    if cap < 1.0 {
        if m.pv_b(pv::TURNS) != 0 {
            let mut d = -((m.pv_s(pv::SPEED) as f32 * (1.0 - cap)) as i32);
            let mut t = (m.pv_s(pv::TURN) as i32).wrapping_add(d) as i16;
            if 0x1000 < t {
                t -= 0x1000;
                d -= 0x1000;
            } else if t < 0 {
                t += 0x1000;
                d += 0x1000;
            }
            m.set_pv_s(pv::TURN, t);
            w.scroll_class(class, 0, d);
        }
        let m = &w.mobys[id];
        let n = 0x10 + (f32::from_bits(0x41a0_0000) * cap) as i32;
        if ctx.game.rng.randi(n - 1) == 0 {
            let rng = &mut ctx.game.rng;
            let a = rng.randf(-2.0, 2.0);
            let mut at = add(m.pos, set_len(m.rows[0], a));
            let b = rng.randf(-f32::from_bits(0x3f4c_cccd), f32::from_bits(0x3f4c_cccd));
            at = add(at, set_len(m.rows[1], b));
            at[2] = m.pv_f(pv::HEIGHT) + f32::from_bits(SPLASH_Z);
            let r = rng.randf(2.0, 3.0);
            let h = rng.randf(f32::from_bits(0x3e4c_cccd), f32::from_bits(0x3ecc_cccd));
            w.liquid.disturb(at[0], at[1], r, h, 0..RECORDS, false);
            let size = ctx.game.rng.randf(2.0, 3.0);
            if let Some(s) = ctx.parts.as_deref_mut() { crate::particles::water::ring(s, &mut ctx.game.rng, size, 13000.0, at, false, None); }
        }
    }
    if w.mobys[id].pv_b(pv::CAPPED) == 0 {
        if class == MASTER {
            let slot = w.mobys[id].pv_b(pv::SLOT) as i8 as i32;
            if slot == -1 || !ctx.host.sound_playing(slot) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_b(pv::SLOT, h as i8 as u8);
            }
            // `rand($gp+0xf80 − 1)`: 2 − 1, so every tick.
            if ctx.game.rng.randi(1) == 0 { puff(w, id, ctx); }
        }
        w.mobys[id].has_collision = false;
    } else {
        if class == MASTER {
            let slot = w.mobys[id].pv_b(pv::SLOT) as i8 as i32;
            if slot != -1 {
                if ctx.host.sound_playing(slot) { ctx.host.stop_sound(slot); }
                w.mobys[id].set_pv_b(pv::SLOT, 0xff);
            }
        }
        w.mobys[id].has_collision = w.class(class).is_some_and(|c| c.has_collision);
    }
}

/// The master's puff (module docs; `0x340df0` = L00 `0x3327e8`).
fn puff(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let rng = &mut ctx.game.rng;
    let s0 = rng.randf(1.0, 2.0);
    let s1 = rng.randf(2.0, 4.0);
    let at = if rng.randi(2) == 0 {
        let mut p = m.pos;
        p[2] += 0.5;
        let a = rng.randf(-3.0, 3.0);
        p = add(p, set_len(m.rows[0], a));
        let b = rng.randf(-1.0, 1.0);
        add(p, set_len(m.rows[1], b))
    } else {
        let mut d = set_len(sub(ctx.host.camera_pos(), m.pos), 1.0);
        d[2] = 0.0;
        // Into its frame (`0x2fb580` the transpose, `0x2faee8`), scaled, and back (`0x2faee8` through its rows).
        let r = &m.rows;
        let mut l: V3 = std::array::from_fn(|k| d[0] * r[k][0] + d[1] * r[k][1] + d[2] * r[k][2]);
        l[0] *= rng.randf(-1.0, 3.5);
        l[1] *= rng.randf(0.0, 3.0);
        l[2] = rng.randf(0.0, 0.5);
        let back: V3 = std::array::from_fn(|k| l[0] * r[0][k] + l[1] * r[1][k] + l[2] * r[2][k]);
        add(back, m.pos)
    };
    let k = f32::from_bits(0x3c88_8889);
    let spread = 0.01 * k;
    let a = rng.rand_angle();
    let vx = a.cos() * rng.randf(0.0, spread);
    let a = rng.rand_angle();
    let vy = a.sin() * rng.randf(0.0, spread);
    let vz = rng.randf(0.1, 0.2) * k;
    let a = rng.rand_angle();
    let ax = a.cos() * rng.randf(0.0, spread);
    let a = rng.rand_angle();
    let ay = a.sin() * rng.randf(0.0, spread);
    let az = rng.randf(0.0, 1.0) * k;
    if let Some(s) = ctx.parts.as_deref_mut() {
        crate::particles::staged(s, &mut ctx.game.rng, at, [vx, vy, vz, s0], [ax, ay, az, s1], 0x80f0_f0a0, 0x00f0_f0a0, 10, 40, 0, -1);
    }
}
