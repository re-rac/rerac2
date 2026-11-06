//! The texture scroller **0x0d06** (L09 `0x3f7a70`, 1 placed; L16 `0x3eb260`, 1 placed; the decomp's
//! `func_L09_003F65E0`): it slides one of the 64 tie texture matrices (`$gp−0x1ba0`, L09 `0x1ad450`;
//! crate::moby::MobyWorld::tex_matrices) along s and t. Its pvars: `+0x00` the entry, `+0x04` / `+0x08` the s / t
//! rates a second.
//!
//! * **0**: alpha 0xff; 1.
//! * **1**: a rate above 0 moves its offset (the entry's `+0x30` / `+0x34`) by rate / 60; s is then held to −1..1
//!   (less 1 while above 1, plus 1 while below −1), t less 1 while above 1 and then plus 1 while below **1** (the
//!   game's own loop: t ends in 1..2).
//!
//! L09's scrolls entry 1 along s at 0.2 a second; L16's rates are 0.

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0d06;

/// `0x3f7a70` (module docs).
pub fn update_0d06(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.alpha = 0xff;
            m.state = 1;
        }
        1 => {
            let (k, rs, rt) = (m.pv_i(0).clamp(0, 63) as usize, m.pv_f(4), m.pv_f(8));
            if w.tex_matrices.len() <= k { w.tex_matrices.resize(64, [0.0; 2]); }
            let e = &mut w.tex_matrices[k];
            if 0.0 < rs {
                e[0] += rs * 0.016_666_668;
                while 1.0 < e[0] { e[0] -= 1.0; }
                while e[0] < -1.0 { e[0] += 1.0; }
            }
            if 0.0 < rt {
                e[1] += rt * 0.016_666_668;
                while 1.0 < e[1] { e[1] -= 1.0; }
                while e[1] < 1.0 { e[1] += 1.0; }
            }
        }
        _ => {}
    }
}
