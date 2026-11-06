//! 0x0eac (`0x4009a8`; the decomp's `func_L07_003FF618`): Oozla's mover between two poses (this level's copy
//! `0x4009e0` of L01 `0x3ca620`, crate::moby::classes::oozla::update_mover), stopped for good (mode 2, no update) once
//! planet 8 is unlocked (`0x1a7bd8`). The copy sets mode 0x800 on its first tick only for Oozla's four mover classes
//! (`0x400a60`: 0x0a6a / 0x0a6b / 0x0aa0 / 0x0aa1), so this class keeps its own.

use crate::moby::classes::oozla::update_mover;
use crate::moby::{mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0eac;

/// `0x1a7bd8` = `0x1a7bd0` + 8: the planet whose unlock stops it.
const PLANET: usize = 8;

pub fn update_0eac(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.planets.get(PLANET).is_some_and(|&b| b != 0) {
        w.mobys[id].mode |= mode::NO_UPDATE;
        return;
    }
    let (first, had) = (w.mobys[id].state == 0, w.mobys[id].mode & 0x800);
    update_mover(w, id, ctx);
    if first {
        let m = &mut w.mobys[id];
        m.mode = (m.mode & !0x800) | had;
    }
}
