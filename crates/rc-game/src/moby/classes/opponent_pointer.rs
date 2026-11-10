//! The orbit fights' pointer to the opponent **0x0d55** ([`update_0d55`], L22 `0x3f9af8`, L23 `0x400cc0`; the decomp's
//! `func_L22_003F8658`; made by [`make`], L22 `0x3f9a40`, L23 `0x400c08`, `func_L22_003F85A0`): the opponents of both
//! orbits keep one at their place (crate::moby::classes::level22::boss's 0x0e0b, crate::moby::classes::level23::boss's
//! 0x0f7f).

use crate::hero::physics::{add, add_rot, V3};
use crate::moby::{ai, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0d55;

/// `0x3f9a40(owner, pos)` (L22; L23 `0x400c08`, the same code): the pointer 0x0d55 at `pos` for `owner`; its place
/// is kept by the owner (`0x3f9ae0`: its `+0x00`).
pub fn make(w: &mut MobyWorld, owner: usize, pos: V3) -> usize {
    let s = w.create(CLASS);
    let sc = w.class(CLASS).map_or(1.0, |c| c.scale);
    let m = &mut w.mobys[s];
    (m.drawn, m.state, m.pos) = (1, 0, pos);
    m.scale = sc * f32::from_bits(0x3da3_d70a);
    (m.update_dist, m.draw_dist) = (0xff, 0xff);
    m.pvars.resize(m.pvars.len().max(0x20), 0);
    m.set_pv_v(0, pos);
    ai::set_link(m, 0x10, Some(owner));
    m.set_pv_i(0x14, 0);
    s
}

/// 0x0d55, the opponent's pointer (L22 `0x3f9af8`, L23 `0x400cc0`; the decomp's `func_L22_003F8658`): gone with its
/// owner (`+0x10`); else 3 ahead of the camera and 1.2 up in its frame (`$gp+0x1b50`), turned toward the owner's place
/// (`+0x00`; its yaw and pitch), spun 120′ a tick about its x (`$gp+0x1b70`), its scale the class's × (0.08 + 0.14 ×
/// `+0x18`), `+0x0c` 3.
pub fn update_0d55(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0x20 { w.mobys[id].pvars.resize(0x20, 0); }
    let owner = ai::link(&w.mobys[id], 0x10).filter(|&o| w.mobys.get(o).is_some_and(|m| m.state < 0xfd));
    let sc = w.class(CLASS).map_or(1.0, |c| c.scale);
    let (a, b) = (f32::from_bits(0x3da3_d70a), f32::from_bits(0x3e61_47ae));
    let k = w.mobys[id].pv_f(0x18);
    w.mobys[id].scale = sc * (a + (b - a) * k);
    if owner.is_none() {
        w.delete(id);
        return;
    }
    let m = &mut w.mobys[id];
    m.set_pv_f(0xc, 3.0);
    let (cam, rows) = (ctx.host.camera_pos(), ctx.host.camera_rows());
    m.pos = add(crate::gravity::to_world([3.0, 0.0, 1.2], &rows), cam);
    let t = m.pv_v(0);
    m.rot[2] = crate::hero::physics::atan2(t[0] - m.pos[0], t[1] - m.pos[1]);
    let flat = ((t[0] - m.pos[0]).powi(2) + (t[1] - m.pos[1]).powi(2)).sqrt();
    m.rot[1] = -crate::hero::physics::atan2(flat, t[2] - m.pos[2]);
    m.rot[0] = add_rot(m.rot[0], 120.0 * 0.000_290_888_25);
}
