//! The Jamming Array's props (L26 addresses).
//!
//! * **0x0af4** ([`update_0af4`], `0x40de48`, 10 placed; the decomp's `func_L26_0040C8A8`): a floating platform
//!   (crate::moby::floater, its block at `+0x60`). Its set-up (`0x32bd98`) puts the pivot `+0x120` × 1.7 along its y.
//!   With a pivot it has no y or z response to Ratchet and no y wobble (`+0xd0`, `+0xd4`, `+0x100` = 0). Then each
//!   tick is a step (`0x32c0b0`).

use crate::moby::{floater, Ctx, MobyWorld};

pub const FLOATER: i16 = 0x0af4;
const BLOCK: usize = 0x60;

pub fn update_0af4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            floater::init(w, id, ctx, BLOCK, &floater::Params::default());
            let m = &mut w.mobys[id];
            let n = m.pv_i(0x120);
            if n != 0 {
                m.set_pv_v(BLOCK + 0x10, [0.0, n as f32 * 1.7, 0.0]);
                m.set_pv_f(0xd0, 0.0);
                m.set_pv_f(0x100, 0.0);
                m.set_pv_f(0xd4, 0.0);
            }
            m.set_state(1, -1);
        }
        1 => floater::step(w, id, ctx, BLOCK),
        _ => {}
    }
}
