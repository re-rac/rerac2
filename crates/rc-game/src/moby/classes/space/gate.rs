//! The gate race's gates, class 0x114a (Feltzin L05 `0x426508`, Hrugis L10 `0x422618`, Gorn L15 `0x4294a8`: one code,
//! the decomp's `func_L05_00424780`; its draw L05 `0x426550`, `func_L05_004247C8`). The race ([`super::mission`]'s
//! gate race, `0x422e88` / `0x426260` / `0x432ba8`) makes them two at a time at its route's points, their frame along
//! the route (rows: forward, right, up; kept), and sets their words each tick (`0x424980(gate, 1)` the draw on `+0xc`,
//! `0x424970(gate, colour)` the colour `+8`).
//!
//! **The update**: with `+0xc` set its draw callback (`0x3000c0` = L00 `0x2e7920`, [`MobyWorld::register_draw`]);
//! hidden and not advanced (mode |= 0x41): the model is never drawn.
//!
//! **The draw** ([`draw`]): a ring of radius 1 across the route, 30 points at random angles each frame (`(0, sin a,
//! −cos a)` in the gate's frame), at each two sprites of FX 0x13 (additive, 0.1 toward the camera, each at a random
//! roll: `0x3064d0` = L00 `0x2edf78`, [`crate::fx::sprite`]): 0.25 across in 0x80808080, then 0.5 across in the gate's
//! colour at alpha 0x40. Then its phase `+0` turned by `+4` = 0.1338 (read by nothing).

use crate::fx::{sprite, FxQuad};
use crate::hero::physics::{add, add_rot, scale, V3};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x114a;

/// The sprites' effect texture.
const FX: usize = 0x13;
/// The ring's points a frame.
const POINTS: usize = 30;
/// `+4`: the phase's step (0x3e09051f).
const STEP: f32 = 0.133_808_6;

/// `0x426508`.
pub fn update(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    if w.mobys[id].pv_i(0xc) != 0 { w.register_draw(id); }
    w.mobys[id].mode |= 0x41;
}

/// `0x426550`: the ring (module docs).
pub fn draw(w: &mut MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let (pos, rows, colour) = (m.pos, m.rows, m.pv_i(8) as u32);
    for _ in 0..POINTS {
        let a = rng.rand_angle();
        let v: V3 = [0.0, a.sin(), -a.cos()];
        let p = add(add(add(scale(rows[0], v[0]), scale(rows[1], v[1])), scale(rows[2], v[2])), pos);
        let roll = rng.rand_angle();
        sprite(out, 0.25, 0.1, roll, p, cam, FX, 0x8080_8080, true);
        let roll = rng.rand_angle();
        sprite(out, 0.5, 0.1, roll, p, cam, FX, (colour & 0x00ff_ffff) | 0x40 << 24, true);
    }
    let m = &mut w.mobys[id];
    m.set_pv_f(4, STEP);
    let phase = add_rot(m.pv_f(0), STEP);
    m.set_pv_f(0, phase);
}
