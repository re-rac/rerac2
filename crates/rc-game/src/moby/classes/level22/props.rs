//! Dobbo orbit's small props (L22 addresses).
//!
//! * **0x0a6c** ([`update_0a6c`], `0x3d1c88`, 13 placed; the decomp's v1.01 `func_L22_003D0840`, no C): on its first
//!   update it hides (mode |= 0x41) and goes to state 1 (`0x320e30` = L00 `0x31b8d0`), which does nothing.

use crate::moby::{Ctx, MobyWorld};

pub const HIDDEN: i16 = 0x0a6c;

pub fn update_0a6c(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state != 0 { return; }
    m.mode |= 0x41;
    m.set_state(1, -1);
}
