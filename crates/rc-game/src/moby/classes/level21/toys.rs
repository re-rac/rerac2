//! The museum's small exhibits (L21 addresses, the decomp's L30).
//!
//! * **0x0bd5** ([`update_0bd5`], `0x3da600`, 2 placed; the decomp's `func_L30_003DA040`): a piece that plays its
//!   sequence 1 once when struck.
//!   * Its pvar `+0x100` keeps its place.
//!   * **0**: its place kept; sequence 1 from frame 0 (blend 1), stopped (`+0x48` = 0); then 1.
//!   * **1**: a hit with the flags 0x10000 (`0x2a9640`) → 2, the sequence running (`+0x48` = 1).
//!   * **2**: at the sequence's end it is put back at its place, sequence 0, stopped, 1.
//!   * Every tick its hit record is let go (`+0xa8` = −1).

use crate::moby::classes::props::{set_seq, wrapped};
use crate::moby::{Ctx, MobyWorld};

pub const STRUCK: i16 = 0x0bd5;

pub fn update_0bd5(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let p = w.mobys[id].pos;
            w.mobys[id].set_pv_v(0x100, p);
            w.mobys[id].set_state(1, -1);
            set_seq(w, id, ctx, 1, 0, 1);
            ctx.host.set_anim_speed(&w.mobys[id], 0.0);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x10000 != 0) {
                w.mobys[id].set_state(2, -1);
                set_seq(w, id, ctx, 1, 0, 1);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
        }
        2 => {
            if wrapped(w, id, ctx) {
                let p = w.mobys[id].pv_v(0x100);
                w.mobys[id].pos = p;
                w.mobys[id].set_state(1, -1);
                set_seq(w, id, ctx, 0, 0, 1);
                ctx.host.set_anim_speed(&w.mobys[id], 0.0);
            }
        }
        _ => {}
    }
    w.clear_hit(id);
}
