//! Damosel's hint and skill point watcher **0x0c34** (`0x3a1fd8`, one placed; the decomp's `func_US2_L18_003A1FD8`).
//! Its pvars: the cuboids `+0x0` / `+0x4`, its watch `+0x8`, `+0xc` (a half-word, 0), the help's wait `+0xe`.
//! * **0**: updated at any distance, the wait 40, `+0xc` 0; 1.
//! * Every tick while the help slot 0x7e was never shown (`0x18cf40` = 0) and with item 5 had (`0x1a7b05`): the wait
//!   counted down (`0x3092e0` = L00 `0x2eeae0`), and out, the help text 0xa27 (slot 0x7e, `0x30fa60` = L00 `0x2f5270`).
//! * **1**, the watch (crate::skill_points::watch, as Boldan's 0x0c2f): **0** Ratchet in `+0x0` in his movement group
//!   0xf: 1. **1** in his state 0x3e: 0; else in `+0x4`: skill point 7 (`0x1a7a6f`; its level sound 1 `0x37ca38`, the
//!   message 0x1233 `0x341f68`, `0x1a903c` 1).

use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c34;

const WATCH: usize = 0x8;
const WAIT: usize = 0xe;
/// The help text, its slot and the item it waits for.
const HELP: (i32, usize) = (0xa27, 0x7e);
const ITEM: usize = 5;

/// 0x0c34 (module docs).
pub fn update_0c34(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.update_dist = 0xff;
        m.set_pv_s(WAIT, 0x28);
        m.set_pv_s(0xc, 0);
        m.state = 1;
    }
    let never = ctx.game.help_counts.get(HELP.1).copied().unwrap_or(0) == 0;
    if never && ctx.game.items.get(ITEM).is_some_and(|&b| b != 0) {
        let mut t = w.mobys[id].pv_s(WAIT) as u16 as i32;
        let out = countdown(&mut t);
        w.mobys[id].set_pv_s(WAIT, t as i16);
        if out { ctx.game.help(HELP.0, HELP.1 as i32); }
    }
    if w.mobys[id].state != 1 { return; }
    let m = &w.mobys[id];
    let (start, end, watch) = (m.pv_i(0), m.pv_i(4), m.pv_i(WATCH));
    let watch = crate::skill_points::watch(ctx, watch, start, end, crate::skill_points::point::DAMOSEL);
    w.mobys[id].set_pv_i(WATCH, watch);
}
