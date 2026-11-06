//! Level 7's snow **0x12ec** (`0x379d50`, 1 placed; the decomp's `func_L07_00378AE0`) and its flakes, the particle
//! type 0x68 ([`flake`] `0x337a30`, [`update_flake`] `0x337bc0`).
//!
//! Its pvars: `[0]` the flakes' speed across (a second), `[1]` their fall (a second), `[3]` the drift's share, `[4]`
//! an area no flake starts in (`0x320ca8`), `+0x14` its path along the convoy's, `+0x20..+0x50` its rows there and
//! `+0x50` its origin (both set by the convoy, crate::moby::classes::level07::train), `+0x60` the point the drift is
//! taken from.
//!
//! * **0**: updated from any distance (`+0x30` 0xff), mode |= 0x100; on level 7 past the first frame (`0x1b1d88` ≠ 0) the view filled ([`fill`],
//!   `0x37a290`) and 1. Its speed 1.5, fall 2.5, `[2]` 1; it is the convoy's snow (`$gp−0x430`).
//! * **1**: on level 7 with the sky chunk in place (`0x1c9700` = 1): 2. Else flakes while `0x311bb0(2)` (drawn again
//!   each time) beats the count: within 15 across ([`disc`]) and 7 up of the point 4 ahead of the camera, moving
//!   within `[0]`/60 across and falling `[1]`/60 × 0.8..1.2, none in area `[4]`. The drift (`0x1b24d0`): that point's
//!   move since the last tick, times its length × 4 (at most 1, `[3]`).
//! * **2**: its first tick the drift's z down 60. On level 7 out of the sky chunk: 1. With a car ridden
//!   (crate::moby::classes::level07::train::Globals::riding): six flakes a tick of the second kind on its lane — the
//!   origin ±25 along its second row and the car's range ([`LANES`]) along its third, across by the camera's offset
//!   from the car — moving within 1/24 across and falling `[1]` × 0.8..1.2 / 120. The drift: the camera's move since
//!   the last tick plus the car's first row.
//!
//! **A flake** (`0x337a30(pos, vel, kind, shown)`): its fade's top alpha 32..80 (`+0x30`), colour 0x808080 (alpha
//! that top when `shown`), `+0x09` 0xf2, ALPHA 0x48, frame `*def[0x49]` or `*def[0x60]` (one in two), size
//! 0.1..0.24 × 210000, a random roll, `pos` and `vel`; kind 0 sways over `1 << 6..10` frames (`+0x0a`), kind 1
//! (`+0x34` 1) lives 120 ticks; `+0x2c` 0..100. (`+0x34` is left as the record had it for kind 0, as the original
//! leaves it.) The flakes' count `$gp+0x34c4` (nothing reads it) is not kept.
//!
//! **Each tick** ([`update_flake`]): more than 7 under the camera (`0x1b9188`), or a kind-1 flake's life out: its top
//! one down (gone at 0) and its alpha that; else its alpha up by one to the top. It rolls 2 a tick (back on records
//! whose address has bit 0x80: here the index's bit 1), moves by its velocity and the drift; a kind-0 flake sways
//! `sin(2π · (frame mod period) / period) / 60` along the camera's left row (`0x1b93c0`).

use crate::hero::physics::{add, len, scale, sub, V3};
use crate::moby::{Ctx, MobyWorld};
use crate::particles::{rec, Particles};
use crate::rng::Rng;

pub const CLASS: i16 = 0x12ec;
/// The flakes' particle type.
pub const FLAKE: u8 = 0x68;

/// `$gp−0x428 + 8·k`: the second kind's lane along the origin's third row, by the ridden car (0x11d9 + k).
const LANES: [(f32, f32); 5] = [(19.0, 27.0), (10.0, 25.0), (17.0, 30.0), (20.0, 38.0), (0.0, 10.0)];

/// `0x311e28(r, &v)`: a random point of a disc of radius `r` (its distance `(1 − u²)·r`), z 0.
fn disc(rng: &mut Rng, r: f32) -> V3 {
    let a = rng.rand_angle();
    let u = ((rng.rand() >> 16) & 0xfff) as f32 * 0.000_244_140_62;
    let d = (1.0 - u * u) * r;
    [a.cos() * d, a.sin() * d, 0.0]
}

/// The point 4 ahead of the camera (`0x1b9180` + 4 × its forward row `0x1b93b0`).
fn ahead(ctx: &Ctx) -> V3 { add(scale(ctx.host.camera_rows()[0], 4.0), ctx.host.camera_pos()) }

/// `0x379d50` (module docs).
pub fn update_12ec(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.mode |= 0x100;
            if ctx.game.level == 7 && ctx.game.frame != 0 {
                fill(w, id, ctx);
                w.mobys[id].set_state(1, -1);
            }
            let m = &mut w.mobys[id];
            m.set_pv_i(8, 1);
            m.set_pv_f(0, 1.5);
            m.set_pv_f(4, 2.5);
            w.convoy.snow = Some(id);
        }
        1 => {
            if ctx.game.level == 7 && ctx.game.variant == 1 {
                w.mobys[id].set_state(2, -1);
                return;
            }
            let (speed, fall, area) = { let m = &w.mobys[id]; (m.pv_f(0), m.pv_f(4), m.pv_i(0x10)) };
            let mut i = 0;
            while i < ctx.game.rng.randi(2) {
                let mut p = disc(&mut ctx.game.rng, 15.0);
                p[2] += 7.0;
                p = add(ahead(ctx), p);
                let mut v = disc(&mut ctx.game.rng, speed * 0.016_666_668);
                v[2] = -(fall * 0.016_666_668) * ctx.game.rng.randf(0.8, 1.2);
                if !ctx.level.in_area(area, p) {
                    if let Some(parts) = ctx.parts.as_deref_mut() { flake(parts, &mut ctx.game.rng, p, v, false, false); }
                }
                i += 1;
            }
            let at = ahead(ctx);
            let m = &mut w.mobys[id];
            let d = sub(at, m.pv_v(0x60));
            let share = (len(d) * 4.0).clamp(0.0, 1.0);
            m.set_pv_f(0xc, share);
            m.set_pv_v(0x60, at);
            w.convoy.drift = scale(d, share);
        }
        2 => {
            if w.mobys[id].first() { w.convoy.drift[2] -= 60.0; }
            if ctx.game.level == 7 && ctx.game.variant != 1 {
                w.mobys[id].set_state(1, -1);
                return;
            }
            let Some(r) = w.convoy.riding.filter(|&r| r < w.mobys.len()) else { return };
            let (rp, rf, k) = { let c = &w.mobys[r]; (c.pos, c.rows[0], (c.o_class - super::train::LEAD) as usize) };
            let (lo, hi) = LANES.get(k).copied().unwrap_or((0.0, 0.0));
            let cam = ctx.host.camera_pos();
            let m = &w.mobys[id];
            let (a_row, b_row, origin, fall) = (m.pv_v(0x30), m.pv_v(0x40), m.pv_v(0x50), m.pv_f(4));
            for _ in 0..6 {
                let a = ctx.game.rng.randf(-25.0, 25.0);
                let mut p = add(scale(a_row, a), origin);
                let b = ctx.game.rng.randf(lo, hi);
                p = add(scale(b_row, b), p);
                p = sub(add(p, cam), rp);
                p[2] -= cam[2] - rp[2];
                let mut v = disc(&mut ctx.game.rng, f32::from_bits(0x3d2a_aaab));
                v[2] = fall * ctx.game.rng.randf(0.8, 1.2) * -0.008_333_334;
                if let Some(parts) = ctx.parts.as_deref_mut() { flake(parts, &mut ctx.game.rng, p, v, true, false); }
            }
            let m = &mut w.mobys[id];
            w.convoy.drift = add(rf, sub(cam, m.pv_v(0x60)));
            m.set_pv_v(0x60, cam);
        }
        _ => {}
    }
}

/// `0x37a290`: 200 flakes shown at once within 15 across and ±7 up of the point 4 ahead of the camera (the first
/// kind's speeds), and the drift's point taken.
fn fill(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (speed, fall) = (w.mobys[id].pv_f(0), w.mobys[id].pv_f(4));
    for _ in 0..200 {
        let mut p = disc(&mut ctx.game.rng, 15.0);
        p[2] = ctx.game.rng.randf(-7.0, 7.0);
        p = add(ahead(ctx), p);
        let mut v = disc(&mut ctx.game.rng, speed * 0.016_666_668);
        v[2] = -(fall * 0.016_666_668) * ctx.game.rng.randf(0.8, 1.2);
        if let Some(parts) = ctx.parts.as_deref_mut() { flake(parts, &mut ctx.game.rng, p, v, false, true); }
    }
    let at = ahead(ctx);
    w.mobys[id].set_pv_v(0x60, at);
}

/// `0x337a30(pos, vel, kind, shown)` (module docs).
pub fn flake(sys: &mut Particles, rng: &mut Rng, pos: V3, vel: V3, kind1: bool, shown: bool) -> Option<usize> {
    let i = sys.create_part(FLAKE)?;
    let top = rng.rand_range(0x20, 0x50);
    let def = if rng.randi(2) != 0 { sys.def_first(0x49) } else { sys.def_first(0x60) };
    let size = rng.randf(f32::from_bits(0x3dcc_cccd), f32::from_bits(0x3e75_c28f)) * 210_000.0;
    let roll = rng.randi(0xff) as u8;
    let period = 1i16 << rng.rand_range(6, 10);
    let r = &mut sys.pool.recs[i];
    rec::set_u32(r, 0x30, top as u32);
    rec::set_u32(r, 4, 0x80_8080);
    r[1] = 0;
    r[9] = 2u8.wrapping_sub(0x10);
    r[3] = 0x48;
    r[2] = def;
    rec::set_ff(r, 0xc, size);
    r[8] = roll;
    rec::set_v3(r, 0x10, pos);
    rec::set_ff(r, 0x1c, 0.0);
    rec::set_v3(r, 0x20, vel);
    rec::set_i16(r, 0xa, period);
    if kind1 {
        rec::set_i16(r, 0xa, 0x78);
        rec::set_u32(r, 0x34, 1);
    }
    if shown { rec::set_u32(r, 4, 0x80_8080u32.wrapping_add((top as u32) << 24)); }
    let k = rng.rand_range(0, 100);
    rec::set_ff(&mut sys.pool.recs[i], 0x2c, k as f32);
    Some(i)
}

/// `0x337bc0` (module docs): `cam` the camera (`0x1b9180`), `mobys` for its rows, the frame and the drift.
pub fn update_flake(sys: &mut Particles, i: usize, mobys: &MobyWorld, cam: V3) {
    let r = &mut sys.pool.recs[i];
    let kind1 = rec::u32(r, 0x34) != 0;
    let mut out = rec::ff(r, 0x18) - cam[2] < -7.0;
    if !out && kind1 {
        let mut t = rec::i16(r, 0xa) as i32;
        out = crate::moby::countdown(&mut t);
        rec::set_i16(r, 0xa, t as i16);
    }
    let c = rec::u32(r, 4);
    if out {
        let top = rec::u32(r, 0x30) as i32 - 1;
        rec::set_u32(r, 0x30, top as u32);
        if top < 1 {
            sys.kill_part(i);
            return;
        }
        rec::set_u32(r, 4, (c & 0xff_ffff).wrapping_add((top as u32) << 24));
    } else if ((c as i32) >> 24) < rec::u32(r, 0x30) as i32 {
        rec::set_u32(r, 4, c.wrapping_add(0x100_0000));
    }
    r[8] = if (i >> 1) & 1 == 0 { r[8].wrapping_add(2) } else { r[8].wrapping_sub(2) };
    let mut p = add(add(rec::v3(r, 0x10), rec::v3(r, 0x20)), mobys.convoy.drift);
    if !kind1 {
        let period = rec::i16(r, 0xa) as i32;
        if period != 0 {
            let f = (mobys.frame & (period - 1)) as f32 / period as f32;
            let s = (f * 6.283_185_5).sin();
            p = add(p, scale(mobys.cam_rows[1], s * 0.016_666_668));
        }
    }
    rec::set_v3(r, 0x10, p);
}
