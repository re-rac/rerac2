//! Boldan's hint and skill point watcher **0x0c2f** (`0x3e9d48`, one placed; the decomp's `func_US2_L13_003E9D48`).
//! Its pvars: the cuboids `+0x0` / `+0x4` / `+0x8`, the hint shown `+0xc` (a half-word), its watch `+0xe`.
//! * **0**: updated at any distance, the hint not shown; 1.
//! * **1**, the watch: **0** Ratchet in `+0x4` in movement group 0xf (`0x18c0bc`): 1. **1** in his state 0x3e: 0;
//!   else in `+0x8` without skill point 4 (`0x1a7a6c`): the point awarded (crate::game::GameState::award_skill_point:
//!   `0x36a270(1, 0, 0)`, the message 0x1233, `0x1a903c` 1).
//! * Then, Ratchet in `+0x0`: unless the stat `0x18c90c` (counted, its time `0x18c910` within 0x69780 ticks; G-HERO-005,
//!   read as none) holds it, with the flag byte 0x7e (`0x139636`) under 2 and the hint not yet shown here: the help text
//!   0x9e3 (slot 0x3a, `0x2ff938` = L00 `0x2f5270`) while that slot was never shown (`0x18cc18` ≥ 0), the byte up by
//!   one, the hint shown.

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c2f;

const SHOWN: usize = 0xc;
const WATCH: usize = 0xe;
const FLAG: usize = 0x7e;

/// 0x0c2f (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.update_dist = 0xff;
        m.set_pv_s(SHOWN, 0);
        m.state = 1;
    }
    if w.mobys[id].state != 1 { return; }
    let pv = |k: usize| w.mobys[id].pv_i(k);
    let (c0, c1, c2) = (pv(0), pv(4), pv(8));
    match w.mobys[id].pv_s(WATCH) {
        0 => {
            if ctx.level.in_cuboid(c1, ctx.hero.pos) && ctx.hero.group == 0xf { w.mobys[id].set_pv_s(WATCH, 1); }
        }
        1 => {
            if ctx.hero.state == 0x3e {
                w.mobys[id].set_pv_s(WATCH, 0);
            } else if ctx.level.in_cuboid(c2, ctx.hero.pos) && !ctx.game.has_skill_point(4) {
                ctx.game.award_skill_point(4);
            }
        }
        _ => {}
    }
    if !ctx.level.in_cuboid(c0, ctx.hero.pos) { return; }
    let n = ctx.game.flags.get(FLAG).copied().unwrap_or(0);
    if n < 2 && w.mobys[id].pv_s(SHOWN) == 0 {
        if (ctx.game.help_levels.get(0x3a).copied().unwrap_or(0) as i32) >= 0 { ctx.game.help(0x9e3, 0x3a); }
        if let Some(b) = ctx.game.flags.get_mut(FLAG) { *b = n.wrapping_add(1); }
        w.mobys[id].set_pv_s(SHOWN, 1);
    }
}
