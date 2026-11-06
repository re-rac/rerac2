//! The orbit levels' Giant Clank (L22 / L23: the same code, the decomp's `func_L22_003A8BA0`).
//!
//! * **0x027d** ([`update_027d`], L22 `0x3a9fd0`, L23 `0x3ae5c8`; one placed on each level): Giant Clank's body.
//!   Ratchet within 4 of it becomes Giant Clank on it (`0x2b7200(2, 0x56, it)`, crate::hero::body and
//!   crate::hero::giant).

use crate::hero::physics::dist;
use crate::moby::{Ctx, MobyWorld};

pub const BODY: i16 = 0x027d;

/// 0x027d (`0x3a9fd0`): the body switch within 4 of it, as Ratchet.
pub fn update_027d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.hero.mode != 0 { return; }
    if dist(ctx.hero.pos, w.mobys[id].pos) < 4.0 { ctx.hero.switch_body(2, 0x56, id); }
}
