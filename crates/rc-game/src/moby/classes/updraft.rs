//! The updrafts (L11 addresses):
//!
//! * **0x08e2** ([`update_08e2`], L11 `0x3d0750`, 1 placed; L17 `0x3c6a38`, 6; the decomp's `func_US2_L11_003D0750`,
//!   and its twin of the same code `func_US2_L13_003C6DC0`, L13 `0x3c6dc8` and L14 `0x3c5db8`): the updraft pad.
//!   Level 17's own pad of this kind is 0x120d (crate::moby::classes::level17::misc::update_120d).
//! * **0x123a** ([`update_123a`], L11 `0x476340`, 1 placed; L13 `0x427cf8`, 1; `func_L11_00474888`): the updraft's
//!   drain.

use crate::moby::{Ctx, MobyWorld};

/// 0x123a (L11 `0x476340`): Ratchet floating on an updraft (state 0x82) in its cuboid (`+0x00`) loses his lift
/// ten times as fast: nine more frames off it (`0x300ee0(0x18a3d0)`, a sixtieth each, not below 0).
pub fn update_123a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !ctx.level.in_cuboid(w.mobys[id].pv_i(0), ctx.hero.pos) || ctx.hero.state != 0x82 { return; }
    let lift = &mut ctx.hero.gear.lift;
    for _ in 0..9 { *lift = if 1.0 / 60.0 < *lift { *lift - 1.0 / 60.0 } else { 0.0 }; }
}

/// 0x08e2 (L11 `0x3d0750`): the updraft pad. Its pvars: `+0x00` a moby (−1 none), `+0x04` the glow's phase,
/// `+0x08` a cuboid (−1 none), `+0x0c` the lift it gives (0: Ratchet's own). **0**: mode | 0x10 (the glow list), **1**.
/// **1**: Ratchet standing on it with item 8: the prompt 0x1349 (owner 0xc, `0x339e80`, crate::prompt), the updraft's
/// 10 ticks (`0x18a012`), so a jump floats (state 0x82), with its lift (`0x18a3d8`) when it has one. Ratchet floating
/// (0x82) in its cuboid: the fall (`0x2d9208(6, 1)`). Drawn, its glow word pulses 6° a tick between 0x20 and 0x80 of
/// red (green with item 8).
///
/// Not ported: its moby `+0x00` let go on the first tick (`0x48bd00()`, no twin found; −1 on the disc).
pub fn update_08e2(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let has = ctx.game.items.get(8).is_some_and(|&b| b != 0);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.mode |= 0x10;
            m.set_state(1, -1);
        }
        1 => {
            if w.mobys[id].first() && w.mobys[id].pv_i(0) != -1 { w.mobys[id].set_pv_i(0, -1); }
            if ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0 {
                if has {
                    ctx.game.prompt.force(0xc, 0x1349, 1);
                    ctx.hero.gear.updraft = 10;
                    let lift = w.mobys[id].pv_f(0xc);
                    if lift != 0.0 { ctx.hero.gear.lift_store = lift; }
                }
            } else if ctx.hero.state == 0x82 && 0 <= w.mobys[id].pv_i(8) && ctx.level.in_cuboid(w.mobys[id].pv_i(8), ctx.hero.pos) {
                ctx.hero.request_state(6, true);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.drawn == 0 { return; }
    let a = crate::hero::physics::add_rot(m.pv_f(4), f32::from_bits(0x3dd6_7752));
    m.set_pv_f(4, a);
    let v = ((a.sin() * 128.0) as i32).clamp(0x20, 0x80) as u32;
    m.glow = if has { v << 8 } else { v } | 0x8000_0000;
}
