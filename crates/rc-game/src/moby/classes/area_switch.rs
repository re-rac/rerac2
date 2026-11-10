//! The area switches: class 0x0102 ([`update`]) and, by areas, 0x0dda ([`update_0dda`]).
//!
//! Class 0x0102 (L01 `0x391e30`, Oozla; the same code on levels 3, 4, 12, 13 and 17): an area switch. Each tick
//! it tests whether Ratchet (`0x189ea0`; with `+0x7e` set the camera, L01 `0x1b94c0` = L00 `0x1b8d40`) is in its
//! area, and when that changes (or on its first tick) it switches its mobys: the members of its groups and the mobys
//! it names, each by three signed flag bytes.
//!
//! **Pvars** (`+0x68`):
//!
//! | Off | What |
//! | --- | --- |
//! | 0x00 | path (−1 none): inside its polygon (L01 `0x312d68` = L00 `0x31cf68`) |
//! | 0x04 | radius: within it of the moby (`0x2e4320` = L00 `0x2ef018`, 3-D), when above 0 |
//! | 0x08..0x20 | six cuboids (−1 none: L01 `0x3175e8` = L00 `0x321858`) |
//! | 0x20..0x50 | twelve groups (−1 none; the member lists L01 `0x1fb020` = L00 `0x1fa8a0`, `0x391dc0`) |
//! | 0x50..0x7c | eleven mobys (−1 none) |
//! | 0x7c / 0x7d / 0x7f | the flags: the update (mode 2), the draw (mode 1 and `+0x31`), the collision (`+0x98`) |
//! | 0x7e | test the camera's position instead of Ratchet's |
//!
//! A flag above 0 turns its part on inside the area and off outside, below 0 the reverse, 0 leaves it
//! (`0x391b98`). A class of kind 0x12 (class header `+0x46`) keeps its draw, and its update outside the area.
//! `+0x95` other than 0 forces the area: 1 inside, else outside. `+0xbc` keeps the last result and the state
//! `+0x20` is that + 1 (0: not switched yet).

use crate::hero::physics::dist;
use crate::moby::{Ctx, MobyWorld};

/// 0x0102.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.update_dist = 0xff;
    let inside = match m.sub_state {
        0 => {
            let p = if m.pv_b(0x7e) == 0 { ctx.hero.pos } else { ctx.host.camera_pos() };
            let r = m.pv_f(4);
            (0.0 < r && dist(m.pos, p) < r)
                || ctx.level.in_polygon(m.pv_i(0), p)
                || (0..6).any(|k| ctx.level.in_cuboid(m.pv_i(8 + 4 * k), p))
        }
        s => s == 1,
    };
    if m.state != 0 && (m.timer != 0) == inside { return; }
    m.timer = inside as u8;
    m.state = m.timer + 1;

    let m = &w.mobys[id];
    let flags = [m.pv_b(0x7c) as i8, m.pv_b(0x7d) as i8, m.pv_b(0x7f) as i8];
    let mut targets: Vec<usize> = Vec::new();
    for k in 0..12 {
        let g = m.pv_i(0x20 + 4 * k);
        if let Some(members) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)) { targets.extend(members); }
    }
    for k in 0..11 {
        if let Ok(i) = usize::try_from(m.pv_i(0x50 + 4 * k)) { targets.push(i); }
    }
    for i in targets { switch(w, i, flags, inside); }
    w.changed = true;
}

/// `0x391b98(index, pvars, inside)`: one moby's update, draw and collision by the flags.
pub fn switch(w: &mut MobyWorld, i: usize, flags: [i8; 3], inside: bool) {
    let Some(m) = w.mobys.get(i) else { return };
    let class = w.class(m.o_class);
    let kind_12 = class.is_some_and(|c| c.kind == 0x12);
    let solid = class.is_some_and(|c| c.has_collision);
    let side = if inside { 1 } else { -1 };
    let m = &mut w.mobys[i];
    m.drive = true;
    match flags[0].signum() * side {
        1 if inside || !kind_12 => m.mode &= !2,
        -1 if inside || !kind_12 => m.mode |= 2,
        _ => {}
    }
    match flags[1].signum() * side {
        1 if !kind_12 => (m.drawn, m.mode) = (1, m.mode & !1),
        -1 if !kind_12 => (m.drawn, m.mode) = (0, m.mode | 1),
        _ => {}
    }
    match flags[2].signum() * side {
        1 => m.has_collision = solid,
        -1 => m.has_collision = false,
        _ => {}
    }
}

/// 0x0dda (L06 `0x4462a8`, 16 placed; the same code — the decomp's `func_L06_00444D00` — on levels 11 `0x4526f8`,
/// 12 `0x401f90`, 13 `0x401468`, 14 `0x40c628`, 16 `0x3f03b8`, 17 `0x407468`, 18 `0x41c998`, 19 `0x3ff878` and the
/// museum `0x3f69c8`): the area switch [`update`] by areas. Its
/// update distance `+0x30` 0xff each tick. Inside: Ratchet (`0x189ea0`; with `+0x7e` set the camera, L06 `0x1b9840`,
/// L11 `0x1ba940`) in any of its four areas `+0x68`[0..0x10] (−1 none; `0x36b6d0`, L11 `0x33d5d8` = L00 `0x32a848`,
/// crate::level::LevelData::in_area); `+0x95` other than 0 forces it (1 inside, else outside). When that changes
/// from `+0xbc` (or in state 0): `+0xbc` the result, the state that + 1, and the members of its thirteen groups
/// `[0x10..0x44]` (`0x446238` over `0x1fb3a0`'s lists) and its fourteen mobys `[0x44..0x7c]` (−1 none) switched by
/// the flags `[0x7c]` / `[0x7d]` / `[0x7f]` (`0x446010`, L11 `0x452460`: [`switch`], the same code).
pub fn update_0dda(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.update_dist = 0xff;
    let inside = match m.sub_state {
        0 => {
            let p = if m.pv_b(0x7e) == 0 { ctx.hero.pos } else { ctx.host.camera_pos() };
            (0..4).map(|k| m.pv_i(4 * k)).any(|a| a != -1 && ctx.level.in_area(a, p))
        }
        s => s == 1,
    };
    if m.state != 0 && (m.timer != 0) == inside { return; }
    m.timer = inside as u8;
    m.state = m.timer + 1;
    let m = &w.mobys[id];
    let flags = [m.pv_b(0x7c) as i8, m.pv_b(0x7d) as i8, m.pv_b(0x7f) as i8];
    let mut targets: Vec<usize> = Vec::new();
    for k in 0..13 {
        let g = m.pv_i(0x10 + 4 * k);
        if g == -1 { continue; }
        if let Some(members) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)) { targets.extend(members); }
    }
    for k in 0..14 {
        if let Ok(i) = usize::try_from(m.pv_i(0x44 + 4 * k)) { targets.push(i); }
    }
    for i in targets { switch(w, i, flags, inside); }
    w.changed = true;
}
