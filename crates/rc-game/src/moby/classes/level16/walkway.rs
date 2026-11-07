//! Snivelak's walkway cuboid **0x0c32** (`0x3d8190`, one placed; the decomp's `func_L16_003D6D00`): a cuboid
//! (`+0x68`[0]) where Ratchet walks round its centre on the magnetic walkway.
//!
//! **0**: updated at any distance; 1. **Each tick**, with Ratchet's feet in the cuboid (`0x31ae60` = L00 `0x321858`)
//! and him not grinding (group 0xf): the walkways' down `0x18a110` the way to the cuboid's centre, its xy made unit
//! and its z dropped (`0x2e8538` = L00 `0x2ef188`), its word `0x18a11c` 2, and `0x18a038` 5: frame mode 1 for 5 more
//! ticks (crate::gravity, crate::hero::frame). On the ground (`0x18a160`) and not in the lava hop 0x38, a line (flags
//! 0, his moby left out: `0x2d7b50` = L00 `0x2de878`) from his centre (`0x189f00`: 0.7 up his rows, Clank 0.4) to his
//! feet plus that down that meets the world (no moby) on anything but the walkway (`0x2db6b8` = L00 `0x2e23e0`: its
//! surface ≠ 2) takes a third of his health limit (`0x1a7a08` · 0.333, `0x3183b8` = L00 `0x31eb70`: his health
//! `0x18c2ec` toward 0 by it) and, with health left, sends him to the lava hop 0x38 (SetState(0x38, 1)).

use crate::collision::{coll_line_m, QueryFlags};
use crate::hero::physics::{add, sub, V3};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c32;

/// 0x0c32 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 {
        m.state = 1;
        m.update_dist = 0xff;
    }
    let c = m.pv_i(0);
    let h = &mut *ctx.hero;
    if !ctx.level.in_cuboid(c, h.pos) || h.group == 0xf { return; }
    let Some(v) = ctx.level.cuboids.get(c as usize) else { return };
    let centre: V3 = [v.matrix[3][0], v.matrix[3][1], v.matrix[3][2]];
    let d = crate::hero::physics::set_len2(sub(centre, h.pos), 1.0);
    h.gravity.walkway_normal = [d[0], d[1], 0.0];
    h.gravity.walkway_k = 2.0;
    h.gravity.walkway_hold = 5;
    if h.grounded_ticks == 0 || h.state == 0x38 { return; }
    let head = h.up_point(if h.mode == 1 { 0.4 } else { 0.7 });
    let feet = add(h.pos, h.gravity.walkway_normal);
    let Some(coll) = ctx.coll else { return };
    let Some(o) = coll_line_m(coll, ctx.scene, head, feet, QueryFlags(0), ctx.hero_moby) else { return };
    if o.surface_id() == 2 || o.moby.is_some() { return; }
    let dmg = (ctx.game.max_health as f32 * f32::from_bits(0x3eaa_7efa)) as i32;
    h.hp -= (h.hp).clamp(-dmg, dmg);
    if h.hp != 0 { h.request_state(0x38, true); }
}
