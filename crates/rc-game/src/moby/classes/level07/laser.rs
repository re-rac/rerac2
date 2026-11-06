//! 0x0e28 (`0x3f5d10`; the decomp's `func_L07_003F4970`): the boss's laser (crate::moby::classes::level07::boss),
//! made by [`spawn`] (`0x3f5ea8`) between two points, kept in the frame of the platform the boss rides when it has
//! one. **0**: from its second tick its strength (`+0x24`) up to 1 by `$gp+0x1e68` a second; then 1. **1**: down to
//! 0 by `$gp+0x1e6c` a second; then deleted. Every tick its spin (`+0x28`) on by `$gp+0x1e70`° a second, its draw
//! registered ([`draw`], `0x3f6060`), and past 0.333 strength the line hits (`0x313618`, crate::moby::ai::
//! attack_line: flags 1, its damage `+0x2c`, push 1, the boss's own line ignored).
//!
//! pvars: `+0x00` / `+0x10` the ends (in the platform's frame with one), `+0x20` the platform, `+0x24` strength,
//! `+0x28` spin, `+0x2c` damage. The boss is its owner (`+0xb8`, the port's link at `+0x30`).

use crate::fx::FxQuad;
use crate::hero::physics::{add, cross, len, scale, set_len, sub, V3};
use crate::moby::{ai, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0e28;

/// `$gp+0x1e68` / `+0x1e6c`: the strength's rise and fall, a second.
const RISE: f32 = 4.0;
const FALL: f32 = 4.0;
/// `$gp+0x1e70`: the spin, degrees a second.
const SPIN: f32 = 300.0;
/// `$gp+0x1e78` / `+0x1e80`: the inner and outer colours of the two strips, `+0x1e80` their FX.
const STRIP_RGBA: [u32; 2] = [0x6000_4060, 0x6000_8000];
const STRIP_FX: usize = 5;
/// `$gp+0x1e88` / `+0x1e8c`: the strips' half widths.
const HALF: [f32; 2] = [0.25, 0.05];
/// `$gp+0x1e90`: the beads' spacing; `+0x1e98` their two sizes; `+0x1ea0` / `+0x1ea4` their two colours; `+0x1ea8`
/// the second's roll ahead; `+0x1eac` the roll turned per bead, degrees.
const STEP: f32 = 0.75;
const BEAD: [f32; 2] = [0.8, 0.333];
const BEAD_RGBA: [u32; 2] = [0x5000_00ff, 0x7000_00ff];
const BEAD_ROLL: f32 = 0.2;
const BEAD_TURN: f32 = 40.0;

/// `0x3f5ea8(owner, platform, a, b, damage)`: a laser at the owner from `a` to `b`, drawn from any distance, its spin
/// random; with a platform its ends into the platform's frame.
pub fn spawn(w: &mut MobyWorld, ctx: &mut Ctx, owner: usize, platform: Option<usize>, a: V3, b: V3, damage: f32) -> Option<usize> {
    w.class(CLASS)?;
    let id = w.create(CLASS);
    let spin = ctx.game.rng.rand_angle();
    let pos = w.mobys[owner].pos;
    let local = |p: V3| match platform.and_then(|q| w.mobys.get(q)) {
        Some(q) => {
            let d = sub(p, q.pos);
            [crate::hero::physics::dot(d, q.rows[0]), crate::hero::physics::dot(d, q.rows[1]), crate::hero::physics::dot(d, q.rows[2])]
        }
        None => p,
    };
    let (a, b) = (local(a), local(b));
    let m = &mut w.mobys[id];
    m.pos = pos;
    m.draw_dist = 0xff;
    m.update_dist = 0xff;
    m.set_pv_v(0, a);
    m.set_pv_v(0x10, b);
    ai::set_link(m, 0x20, platform);
    m.set_pv_f(0x24, 0.0);
    m.set_pv_f(0x28, spin);
    m.set_pv_f(0x2c, damage);
    ai::set_link(m, 0x30, Some(owner));
    w.changed = true;
    Some(id)
}

/// `0x3f5fb8`: the ends in the world.
fn ends(w: &MobyWorld, id: usize) -> (V3, V3) {
    let m = &w.mobys[id];
    let (a, b) = (m.pv_v(0), m.pv_v(0x10));
    match ai::link(m, 0x20).and_then(|q| w.mobys.get(q)) {
        Some(q) => {
            let world = |p: V3| add(std::array::from_fn(|i| p[0] * q.rows[0][i] + p[1] * q.rows[1][i] + p[2] * q.rows[2][i]), q.pos);
            (world(a), world(b))
        }
        None => (a, b),
    }
}

pub fn update_0e28(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.state_ticks = m.state_ticks.wrapping_add(1);
            if m.state_ticks != 1 {
                let mut s = m.pv_f(0x24);
                crate::hero::physics::approach(1.0, RISE / 60.0, &mut s);
                m.set_pv_f(0x24, s);
                if 1.0 <= s { m.set_state(1, -1); }
            }
        }
        1 => {
            let m = &mut w.mobys[id];
            let mut s = m.pv_f(0x24);
            crate::hero::physics::approach(0.0, FALL / 60.0, &mut s);
            m.set_pv_f(0x24, s);
            if s <= 0.0 { return w.delete(id); }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    m.set_pv_f(0x28, crate::hero::physics::add_rot(m.pv_f(0x28), SPIN * 0.000_290_888_25));
    w.register_draw(id);
    if 0.333 < w.mobys[id].pv_f(0x24) {
        let (a, b) = ends(w, id);
        let Some(owner) = ai::link(&w.mobys[id], 0x30).filter(|&o| o < w.mobys.len()) else { return };
        let damage = w.mobys[id].pv_f(0x2c);
        ai::attack_line(w, owner, ctx, a, b, 1, damage, 1.0, 0);
    }
}

/// `0x3f6060`: two strips along the line facing the camera (half widths [`HALF`], their colours' alpha by the
/// strength), then beads every [`STEP`] (in flight only as far as twice the strength reaches), each two glows rolled
/// apart, the roll turned and flipped per bead.
pub fn draw(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let (a, b) = ends(w, id);
    let d = sub(b, a);
    let normal = set_len(cross(sub(cam, m.pos), d), 1.0);
    let s = m.pv_f(0x24);
    for (k, half) in HALF.iter().enumerate() {
        let c = crate::hud::lerp_rgba(s, STRIP_RGBA[k] & 0xff_ffff, STRIP_RGBA[k]);
        let side = scale(normal, *half);
        let corners = [sub(a, side), add(a, side), sub(b, side), add(b, side)];
        out.push(FxQuad { fx: STRIP_FX, additive: true, corners, st: [[0.5, 0.0], [0.5, 1.0], [0.5, 0.0], [0.5, 1.0]], rgba: [c; 4] });
    }
    let length = len(d) / STEP;
    let (alpha, count) = if m.state == 0 { (1.0, (length * s * 2.0).min(length + 1.0)) } else { (s * s, length + 1.0) };
    let step = set_len(d, STEP);
    let mut at = a;
    let mut roll = m.pv_f(0x28);
    let mut i = 0;
    while (i as f32) < count {
        let size = if i as f32 == count - 1.0 || i & 1 != 0 { BEAD[0] } else { BEAD[1] };
        let c0 = crate::hud::lerp_rgba(alpha, BEAD_RGBA[0] & 0xff_ffff, BEAD_RGBA[0]);
        let c1 = crate::hud::lerp_rgba(alpha, BEAD_RGBA[1] & 0xff_ffff, BEAD_RGBA[1]);
        crate::fx::sprite(out, size, 0.0, roll, at, cam, 0x31, c0, true);
        crate::fx::sprite(out, size, 0.0, crate::hero::physics::add_rot(roll, BEAD_ROLL), at, cam, 0x31, c1, true);
        roll = -crate::hero::physics::add_rot(roll, BEAD_TURN * 0.017_453_294);
        at = if (i as f32) < length - 1.0 { add(at, step) } else { b };
        i += 1;
    }
}
