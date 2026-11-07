//! The Jamming Array's hints **0x1268** ([`update_1268`], `0x408698`, one placed; the decomp's
//! `func_L26_004070F8`): two help texts (crate::help, `0x2f5550` = L00 `0x2f5270`).
//!
//! * **0**: updated at any distance (`+0x30` 0xff), its words `+0x04` / `+0x10` 0 (read by nothing); 1.
//! * **1**, each tick:
//!   * with the sphere's frames on (`0x1a8ff0`, crate::gravity) and the panel idle (`0x27d548` 0, nothing asked
//!     `0x27d56c` −1): the gravity boots' text 0x9c5 (slot 0x1c) unless the slot's level bits (`0x18c958 + 0xc·0x1c`,
//!     `+8`) hold the level's;
//!   * with the stat `0x18c8f4` 0 (or more than 0x1a5e0 frames since `0x18c8f8`), Ratchet idle (`0x18c0b4` 0) and
//!     past 0x4b0 frames on the level (`0x1b16c8`): the text 0x9b5 (slot 0xc) unless its level bits hold the level's.
//!
//! Both tests shift the level the wrong way (`level << 1`, not `1 << level`: the game's `sllv` takes its operands
//! swapped), so the bits they read are those of the levels set in `2·level`. The stats `0x18c8f4` / `0x18c8f8` are not
//! ported (G-HERO-005): read as 0, the second text's first test always passes.

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x1268;

/// 0x1268 (module docs).
pub fn update_1268(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.update_dist = 0xff;
            m.set_pv_i(0x10, 0);
            m.set_pv_i(0x04, 0);
            m.set_state(1, -1);
        }
        1 => {
            let g = &mut *ctx.game;
            let bits = (g.level.max(0) as u32).wrapping_shl(1);
            let seen = |g: &crate::game::GameState, slot: usize| g.help_levels.get(slot).is_some_and(|&b| b & bits != 0);
            if ctx.hero.gravity.on && g.help_state == 0 && g.help_req.is_none() && !seen(g, 0x1c) { g.help(0x9c5, 0x1c); }
            if ctx.hero.state == 0 && 0x4b1 <= g.frame && !seen(g, 0xc) { g.help(0x9b5, 0xc); }
        }
        _ => {}
    }
}
