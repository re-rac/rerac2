//! Dobbo's waker **0x0de0** (`0x409760`, 3 placed; the decomp's `func_L09_004082D0`): while Ratchet (or, with its
//! `+0xbe` set, the camera `0x1b9980`) is in one of its four areas, it hands its bytes `+0xbc` / `+0xbd` / `+0xbf`
//! (switched off: their negations) to the mobys of up to 29 moby groups and 14 single mobys, the troopers 0x0d0f
//! (crate::moby::classes::level09::trooper) among them.
//!
//! Its pvars: `[0..4]` the areas (−1 none), `[4..0x21]` the moby groups, `+0x84..+0xbc` the mobys (−1 none), `+0xbc`
//! the update ask, `+0xbd` the draw ask, `+0xbf` the collision ask, `+0xbe` the camera instead of Ratchet. Its moby
//! byte `+0xbc` (on, kept at pvar `+0xc0` here) and its state (`+0x20`: 1 off, 2 on).
//!
//! **Each tick** past the fifth frame (`0x1b2308`): updated from any distance (`+0x30` 0xff). In sub 0 it is on with
//! the point in one of its areas (`0x3270c8` = crate::level::LevelData::in_area); sub 1 forces it on, any other off.
//! A change (or the first tick, state 0) hands the bytes to every moby ([`hand`], `0x4095c8`): a trooper gets the
//! update ask as its show / hide ask (`+0x31c`), its update distance kept (`+0x31e`) and, asked, held at 0xff; the
//! energy nodes 0x0d09 and their towers 0x0cea only the draw and collision asks; another waker its sub-state; any other
//! all three ([`apply`], `0x4094e8`).

use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0de0;
/// The port's: its moby byte `+0xbc` (on).
const ON: usize = 0xc0;

/// `0x409760` (module docs).
pub fn update_0de0(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.frame < 5 { return; }
    let m = &mut w.mobys[id];
    m.update_dist = 0xff;
    let at = if m.pv_b(0xbe) != 0 { ctx.host.camera_pos() } else { ctx.hero.pos };
    let on = match m.sub_state {
        0 => (0..4).any(|k| { let a = m.pv_i(4 * k); a != -1 && ctx.level.in_area(a, at) }),
        s => s == 1,
    };
    let was = m.pv_b(ON) != 0;
    if on == was && m.state != 0 { return; }
    m.set_pv_b(ON, on as u8);
    m.state = on as u8 + 1;
    let f = |v: u8| if on { v as i8 } else { (v as i8).wrapping_neg() };
    let (a, b, c) = (f(m.pv_b(0xbc)), f(m.pv_b(0xbd)), f(m.pv_b(0xbf)));
    let groups: Vec<i32> = (0..29).map(|k| m.pv_i(0x10 + 4 * k)).collect();
    let singles: Vec<i32> = (0..14).map(|k| m.pv_i(0x84 + 4 * k)).collect();
    for g in groups {
        if g == -1 { continue; }
        let list = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
        for e in list { hand(w, id, e, a, b, c); }
    }
    for e in singles {
        if let Ok(e) = usize::try_from(e) { hand(w, id, e, a, b, c); }
    }
}

/// `0x4095c8(waker, e, a, b, c)` (module docs).
fn hand(w: &mut MobyWorld, id: usize, e: usize, a: i8, b: i8, c: i8) {
    let Some(m) = w.mobys.get(e).filter(|m| m.state < 0xfd) else { return };
    match m.o_class {
        super::trooper::CLASS => {
            let m = &mut w.mobys[e];
            m.set_pv_b(super::trooper::ASK, a as u8);
            if m.update_dist != 0xff { let d = m.update_dist; m.set_pv_b(super::trooper::UPD, d); }
            if a != 0 { m.update_dist = 0xff; }
        }
        0x0d09 | 0x0cea => apply(w, e, 0, b, c),
        CLASS => { let s = w.mobys[id].sub_state; w.mobys[e].sub_state = s; }
        _ => apply(w, e, a, b, c),
    }
}

/// `0x4094e8(e, update, draw, collision)`: each ask set (≠ 0) taken — updated (> 0) or not (mode 2), drawn or hidden
/// (mode 1), its class's collision or none; the first two not for a class of the kind 0x12.
fn apply(w: &mut MobyWorld, e: usize, a: i8, b: i8, c: i8) {
    let (kind, has) = w.class(w.mobys[e].o_class).map_or((None, None), |k| (Some(k.kind), Some(k.has_collision)));
    let m = &mut w.mobys[e];
    if a != 0 && kind != Some(0x12) {
        if a < 1 { m.mode |= crate::moby::mode::NO_UPDATE } else { m.mode &= !crate::moby::mode::NO_UPDATE }
    }
    if b != 0 && kind != Some(0x12) {
        if b < 1 { m.drawn = 0; m.mode |= 1; } else { m.drawn = 1; m.mode &= !1; }
    }
    if c != 0 {
        if let Some(has) = has { m.has_collision = 0 < c && has; }
    }
}
