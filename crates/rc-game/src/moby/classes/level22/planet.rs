//! The orbit fight's planetoid helpers (L22; the decomp's `func_L22_003A2988`, shared with L23's copies): the sphere
//! the fight is on is the gravity frames' (crate::gravity, its centre `0x189ed0`); its radius is the opponent's
//! distance from that centre at its set-up (`0x1b2a24`, crate::moby::classes::level22::boss).
//!
//! Not ported: the walk test `0x3a3bb0` (from → to in steps of 8 on the surface lifted by the mover's `+0x110 +
//! +0x114`, no step blocked by a line 0x12, at most 50 apart) and the nearest path point `0x3a3fc0` (over the
//! opponent's 32 paths `+0x2a0`, those the skip table lets through with its flag): their users, the opponent's AI
//! and the small enemies 0x0edd, are not ported yet.

use crate::hero::physics::{add, scale, set_len, sub, V3};
use crate::moby::Ctx;

/// `0x3a3db8(p, h)`: `p` moved along its line from the sphere's centre to `h` above the surface (radius + `h` from
/// the centre).
pub fn surface(ctx: &Ctx, radius: f32, p: V3, h: f32) -> V3 {
    let c = ctx.hero.gravity.centre;
    add(set_len(sub(p, c), h + radius), c)
}

/// `0x32b440(p, out, 0)` (L00 `0x326448`): the down at `p` (world −z with the frames off).
pub fn down_at(ctx: &Ctx, p: V3) -> V3 { scale(ctx.hero.gravity.up_at(p), -1.0) }
