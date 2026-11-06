//! Smolg's props (L17 addresses).
//!
//! * **0x0fbd** ([`update_0fbd`], `0x41a0f8`, 43 placed; the decomp's `func_L17_00418A68`): a conveyor, a platform
//!   (its block at `+0x50`, its flags 4 and `+0x3e` bit 8) that moves its riders along its facing.
//!   * Its speed (`+0x9c`) is `+0x98` a second. If it sits on another platform (`+0x90`), its speed is instead how far
//!     that platform carries it along its facing this tick.
//!   * Its animation runs at 15 × its speed.
//!   * Its loop sound 0 (mode 4, `+0xa0`) plays while that is 0.1 or more, and stops below.
//! * **0x0fd7** ([`update_0fd7`], `0x41a3a0`, 12 placed): a hit with the flags 0x4010001 breaks it.
//!   * All its pieces fly (`0x32e570` = L00 `0x32b580`; the ranges `$gp+0x2d90` / `+0x2d88` / `+0x2d80`, every piece
//!     a shard, sound 0, the level's bolts).
//!   * A blast goes off at it (size 10 × its scale, sprites 12 / 10 / 10, 10 puffs, its flash ball).
//!   * Then it goes (`$gp+0x2db0`, the piece left standing, is −1).

use crate::hero::physics::{dot, sub, V3};
use crate::moby::{blast, platform, shards, Ctx, MobyWorld};

pub const CONVEYOR: i16 = 0x0fbd;
pub const CRATE: i16 = 0x0fd7;

pub fn update_0fbd(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        w.mobys[id].set_pv_i(0xa0, -1);
        w.mobys[id].set_state(1, -1);
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(0x8c, m.pv_i(0x8c) | 4);
    m.set_pv_s(0x3e, m.pv_s(0x3e) | 8);
    let yaw = m.rot[2];
    let dir: V3 = [yaw.cos(), yaw.sin(), 0.0];
    let under = usize::try_from(m.pv_i(0x90)).ok().filter(|&p| w.mobys.get(p).is_some_and(|q| platform::block(q).is_some()));
    let speed = match under {
        Some(p) => {
            let m = &w.mobys[id];
            match platform::ride(w, p, m.pos, m.rot) {
                Some((to, _)) => {
                    let mut d = sub(to, m.pos);
                    d[2] = 0.0;
                    dot(d, dir)
                }
                None => 0.0,
            }
        }
        None => w.mobys[id].pv_f(0x98) / 60.0,
    };
    let m = &mut w.mobys[id];
    m.set_pv_f(0x9c, speed);
    let rot = m.rot;
    platform::write(m, 0x50, [dir[0] * speed, dir[1] * speed, 0.0], rot, rot);
    let rate = speed * 15.0;
    ctx.host.set_anim_speed(&w.mobys[id], rate);
    let h = w.mobys[id].pv_i(0xa0);
    if rate.abs() < 0.1 {
        if 0 <= h && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(0xa0, -1);
    } else if h < 0 || !ctx.host.sound_playing(h) {
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(0xa0, h);
    }
}

const RANGE: [f32; 8] = [-3.0, -3.0, 4.0, 0.0, 3.0, 3.0, 8.0, 0.0];
const SPIN: (f32, f32) = (90.0, 360.0);
const GRAVITY: (f32, f32) = (10.0, 20.0);

pub fn update_0fd7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0001 != 0) {
            w.mobys[id].set_state(2, -1);
            w.clear_hit(id);
        },
        2 => {
            shards::throw_all(w, id, ctx, RANGE, SPIN, GRAVITY, 0xffff, 0, -1, 0, 0xffff);
            let (pos, s) = (w.mobys[id].pos, w.mobys[id].scale);
            blast::burst(w, ctx, pos, &blast::Blast { size: s * 10.0, bursts: [12, 10, 10], puffs: 10, streaks: 0, light: true, shake: false });
            w.delete(id);
        }
        _ => {}
    }
}

pub const LAMP: i16 = 0x10ec;

/// 0x10ec (`0x41bb58`, 8 placed; the decomp's `func_L17_0041A4C0`): a lamp.
/// * **0**: a glow 20 up its frame (`0x33e220`: 0x10008080, 336000 across, pulled 0.75, far 0x60, while it lives);
///   then 1.
/// * **1**: a hit with the flags 0x4010000, while `+0x68`[0] is 0, moves it to 2. The hit record is let go every
///   tick.
/// * **2**: it breaks. Its pieces fly as 0x0fd7's (the same values at `$gp+0x2e90..+0x2ec0`). The same blast goes off 20 up
///   its scaled frame. Then it goes.
pub fn update_10ec(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (pos, s) = (m.pos, m.scale);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow(sys, &mut ctx.game.rng, 336_000.0, 0.75, [0.0, 0.0, 20.0 * s], id, pos, 0x60, false, 0x1000_8080, -1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) && w.mobys[id].pv_i(0) == 0 {
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            shards::throw_all(w, id, ctx, RANGE, SPIN, GRAVITY, 0xffff, 0, -1, 0, 0xffff);
            let m = &w.mobys[id];
            let s = m.scale;
            let at = std::array::from_fn(|k| m.rows[2][k] * 20.0 * s + m.pos[k]);
            blast::burst(w, ctx, at, &blast::Blast { size: s * 10.0, bursts: [12, 10, 10], puffs: 10, streaks: 0, light: true, shake: false });
            w.delete(id);
        }
        _ => {}
    }
}
