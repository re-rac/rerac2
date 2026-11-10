//! The orbit planetoids' surface helpers both orbit fights share.

use crate::hero::physics::{add, scale, set_len, sub, V3};
use crate::moby::Ctx;

/// `0x3a3db8(p, h)` (L23 `0x3a83f0`; the decomp's `func_L22_003A2988`): `p` moved along its line from the sphere's
/// centre to `h` above the surface (radius + `h` from the centre; the radius L22 `0x1b2a24`, L23 `0x1b30a4`).
pub fn surface(ctx: &Ctx, radius: f32, p: V3, h: f32) -> V3 {
    let c = ctx.hero.gravity.centre;
    add(set_len(sub(p, c), h + radius), c)
}

/// `0x32b440(p, out, 0)` (L00 `0x326448`): the down at `p` (world −z with the frames off).
pub fn down_at(ctx: &Ctx, p: V3) -> V3 { scale(ctx.hero.gravity.up_at(p), -1.0) }

/// `0x32b9d0(m, v, 0)` (L22; L23 its own place): the turn from the moby's facing to `v` (its yaw in the moby's frame,
/// made positive).
pub fn turn_to(rows: &[V3; 3], v: V3) -> f32 {
    let l = crate::gravity::to_local(v, rows);
    crate::hero::physics::atan2(l[0], l[1]).abs()
}
