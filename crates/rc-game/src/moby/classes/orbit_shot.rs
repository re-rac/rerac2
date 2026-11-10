//! The orbit fights' drone shot (its update [`update_0f5d`], L22 `0x40f240` for Dobbo orbit's drones' 0x0f5d, L23
//! `0x423980` for Damosel orbit's turret shot 0x118e; the decomp's `func_L22_0040DD40`): 80 ticks (its maker's count)
//! on along its velocity, its level's hit on the line it moved each tick (L22 `0x40f428`,
//! crate::moby::classes::level22::drone::shot_hit; L23 `0x423b68`, crate::moby::classes::level23::shots::turret_hit),
//! then gone. Its makers: L22 `0x40f300` (crate::moby::classes::level22::drone), L23 `0x423a40`
//! (crate::moby::classes::level23::shots::turret_shot).

use crate::hero::physics::add;
use crate::moby::{countdown, Ctx, MobyWorld};

/// 0x0f5d / 0x118e (L22 `0x40f240`, module docs).
pub fn update_0f5d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0x40 { w.mobys[id].pvars.resize(0x40, 0); }
    match w.mobys[id].state {
        0 | 1 => {
            w.mobys[id].state = 1;
            let m = &mut w.mobys[id];
            let from = m.pos;
            m.pos = add(m.pos, m.pv_v(0));
            if w.mobys[id].o_class == super::level23::shots::TURRET {
                super::level23::shots::turret_hit(w, id, ctx, from);
            } else {
                super::level22::drone::shot_hit(w, id, ctx, from);
            }
            let m = &mut w.mobys[id];
            let mut k = m.pv_i(0x10);
            if countdown(&mut k) { m.state = 2; }
            m.set_pv_i(0x10, k);
        }
        2 => w.delete(id),
        _ => {}
    }
}
