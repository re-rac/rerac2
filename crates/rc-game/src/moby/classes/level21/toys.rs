//! The museum's small exhibits (L21 addresses, the decomp's L30).
//!
//! * **0x0bd5** ([`update_0bd5`], `0x3da600`, 2 placed; the decomp's `func_L30_003DA040`): a piece that plays its
//!   sequence 1 once when struck.
//!   * Its pvar `+0x100` keeps its place.
//!   * **0**: its place kept; sequence 1 from frame 0 (blend 1), stopped (`+0x48` = 0); then 1.
//!   * **1**: a hit with the flags 0x10000 (`0x2a9640`) → 2, the sequence running (`+0x48` = 1).
//!   * **2**: at the sequence's end it is put back at its place, sequence 0, stopped, 1.
//!   * Every tick its hit record is let go (`+0xa8` = −1).
//! * **0x08d7** ([`update_08d7`], `0x3be9f8`, 4 placed; the decomp's `func_US2_L30_003BE9F8`): a teleport pad,
//!   paired with another (`+0x00`).
//!   * Its pvars: `+0x04` set at the start (else it starts ready), `+0x08` its prompt (0 0x12ee, 1 0x12ef; in the
//!     museum, levels 0x15 / 0x1e, always 0x133f), `+0x0c` a record of the table `0x1b1844` it resets.
//!   * **0**: with `+0x04` it waits (1, sequence 0); else ready (3, sequence 2).
//!   * **3** (ready): Ratchet standing on it gets the prompt (owner 8). △ sends him: the partner is toggled
//!     ([`toggle`], `0x3be678`) and he is put at the partner's place and angles in his state 0x6f (`0x2b9e68`).
//!   * **2**: at the end of its sequence it arrives: 5, sequence 2, its record reset.
//!   * **5** (arrived): Ratchet in 0x6f is let go there (state 0); then 3.
//!   * The states are written as bytes (no state change: their ticks run on).
//!
//! Not ported for 0x08d7: Ratchet's byte `+0x22b5` set during the trip; the reset of its record in `0x1b1844`
//! (`0x42a500`: `+0x26` and `+0x20` of the record's target cleared), whose table the port has not got.

use crate::moby::classes::props::{set_seq, wrapped};
use crate::moby::{Ctx, MobyWorld};

pub const STRUCK: i16 = 0x0bd5;
pub const PAD: i16 = 0x08d7;

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

/// `0x3be678`: the partner pad toggled: waiting (1) it starts its sequence 1 (2); else it shows sequence 2 at once
/// (5).
fn toggle(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 1 {
        set_seq(w, id, ctx, 1, 0, 5);
        w.mobys[id].state = 2;
    } else {
        set_seq(w, id, ctx, 2, 0, 5);
        w.mobys[id].state = 5;
    }
}

pub fn update_08d7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if w.mobys[id].pv_i(4) != 0 {
                w.mobys[id].state = 1;
                set_seq(w, id, ctx, 0, 0, 5);
            } else {
                w.mobys[id].state = 3;
                set_seq(w, id, ctx, 2, 0, 5);
            }
        }
        2 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].state = 5;
                set_seq(w, id, ctx, 2, 0, 5);
            }
        }
        3 => {
            let text: i32 = if matches!(ctx.game.level, 0x15 | 0x1e) {
                0x133f
            } else {
                match w.mobys[id].pv_i(8) { 0 => 0x12ee, 1 => 0x12ef, _ => -1 }
            };
            if ctx.hero.ground_moby == Some(id) {
                ctx.game.prompt.request(8, text as u32, 1);
                if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 {
                    let Some(other) = usize::try_from(w.mobys[id].pv_i(0)).ok().filter(|&o| o < w.mobys.len()) else { return };
                    toggle(w, other, ctx);
                    let (p, r) = (w.mobys[other].pos, w.mobys[other].rot);
                    ctx.hero.teleport(p, r);
                    ctx.hero.request_state(0x6f, true);
                }
            }
        }
        5 => {
            if ctx.hero.state == 0x6f {
                let (p, r) = (ctx.hero.pos, ctx.hero.rot);
                ctx.hero.teleport(p, r);
                ctx.hero.request_state(0, false);
            }
            w.mobys[id].state = 3;
        }
        _ => {}
    }
}
