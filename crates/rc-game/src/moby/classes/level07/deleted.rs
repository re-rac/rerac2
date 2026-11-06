//! Level 7's **0x0d21** (`0x3e74f8`, 2 placed; the decomp's `func_L07_003E61B8`): deleted on its first update while
//! the word `$gp+0x1aa0` is set (`0x3078e8`). The word is 1 in the level's data and nothing on the level writes it,
//! so the class never stays.

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0d21;

/// `$gp+0x1aa0` (`0x1b0a90`): 1, never written.
const GONE: bool = true;

/// `0x3e74f8` (module docs).
pub fn update_0d21(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    if GONE { w.delete(id); }
}
