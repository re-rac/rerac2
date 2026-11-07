//! The museum's item zone, class **0x12e7** ([`update_12e7`], `0x3854f0`, one placed; the decomp's
//! `func_L30_003842E0`): while the camera (`0x1b9c40`) is in its area (`+0x00`, `0x32d238` = L00 `0x32a848`,
//! crate::level::LevelData::in_area) Ratchet's items are kept away (`0x18c0dc`, crate::hero::items::ItemBlock::no_use).
//!
//! * **0**: without an area it is deleted (`0x317e40`); else 1.
//! * **1**: the camera in the area: the items kept away, 2.
//! * **2**: the camera in the area: kept away still; out of it, the items let back (`0x18c0dc` 0), 1.

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x12e7;

pub fn update_12e7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let area = w.mobys[id].pv_i(0);
    match w.mobys[id].state {
        0 => {
            if area == -1 { w.delete(id) } else { w.mobys[id].set_state(1, -1) }
        }
        1 => {
            if ctx.level.in_area(area, ctx.host.camera_pos()) {
                ctx.hero.items.no_use = 1;
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            if ctx.level.in_area(area, ctx.host.camera_pos()) {
                ctx.hero.items.no_use = 1;
            } else {
                ctx.hero.items.no_use = 0;
                w.mobys[id].set_state(1, -1);
            }
        }
        _ => {}
    }
}
