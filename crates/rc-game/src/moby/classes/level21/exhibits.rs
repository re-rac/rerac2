//! The museum's exhibits.
//!
//! * **0x134e** ([`update_134e`], `0x387510`, 29 placed; the decomp's `func_L30_00386300`): a turning, bobbing
//!   exhibit that shows one of its 13 sub-models. Its pick is `+0x00`; out of range, a random one is chosen. The
//!   pick is drawn over the whole model (`+0x7c` = 1 << pick). It keeps its height (`+0xc`), then bobs 0.25 at 80° a
//!   second (`0x32a138` = L00 `0x324430`, `$gp−0x1b0` / `−0x1ac`; phase `+4`, last `+8`) and turns 1° a tick.

use crate::moby::{Ctx, MobyWorld};

pub const EXHIBIT: i16 = 0x134e;

pub fn update_134e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 {
        let z = m.pos[2];
        m.set_pv_f(0xc, z);
        if 13 <= m.pv_i(0) as u32 {
            let k = ctx.game.rng.randi(13);
            m.set_pv_i(0, k);
        }
        m.bangles = 1 << m.pv_i(0);
        m.set_state(1, -1);
    } else {
        let base = m.pv_f(0xc);
        crate::moby::classes::props::bob(m, 0.25, 80.0 * 0.000_290_888_25, base, 4, 8);
        m.rot[2] = crate::hero::physics::add_rot(m.rot[2], 0.017_453_294);
        m.state_ticks = 0;
    }
    m.state_ticks = m.state_ticks.wrapping_add(1);
}
