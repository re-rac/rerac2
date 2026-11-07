//! Grelbin's fly-by and fall-height watcher **0x0c35** (`0x3ea670`, one placed; the decomp's `func_L19_003E9400`).
//! Its pvars: `+0x00..+0x0c` the fall height by the level's variant (`0x1c9f40`), `+0x10` a cuboid, `+0x14` the
//! camera cutaway it drives (a 0x02e1, crate::moby::classes::oozla::update_02e1), `+0x18` / `+0x1c` the mobys its
//! flight runs between, `+0x20` its mission, `+0x24` armed, `+0x28` / `+0x2c` the flight's ticks and those left,
//! `+0x30` / `+0x34` the heights over its ends, `+0x38` a cuboid.
//! * **Every tick**: the fall height (`$gp+0x35b8`, read by Ratchet's update `0x2cace8`: crate::game::GameState::
//!   death_z): Ratchet in the cuboid `+0x38` (above 0), its height (the cuboid's `+0x38`, its place's z); else the
//!   variant's word, when above 0.
//! * **0**: updated at any distance; armed while its mission (`0x19b2e8 + level·0x10 + +0x20`, bit 7) is not done; 1.
//! * **1**: the help text 0xa12 (slot 0x69, `0x2f2908` = L00 `0x2f5270`) without item 0x37 (`0x1a7b37`), with
//!   Ratchet in `+0x10` and more than 3600 ticks since the slot's last showing (`0x18ce48`). Its mission done while
//!   armed: 2.
//! * **2**: on its first tick unarmed — with no cutaway or ends, 1; else the cutaway poked (its timer byte 1) and the
//!   flight's ticks its (`+0x2c`). Each tick the cutaway's camera (its pvars `+0x00` place, `+0x18` yaw) along the
//!   way from `+0x18`'s place (`+0x30` up) to `+0x1c`'s (`+0x34` up), at `0.45 + 0.45·sin((1 − left/ticks)·π − π/2)`
//!   of it, facing along it; the ticks left counted down (`0x2ec0d0` = L00 `0x2eeab0`), and out, the autosave
//!   (`0x30af90` = L00 `0x30d658(0, −1)`, not ported) and 1.

use crate::hero::physics::{add, atan2, scale, sub};
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c35;

mod pv {
    pub const CUBOID: usize = 0x10;
    pub const CUTAWAY: usize = 0x14;
    pub const FROM: usize = 0x18;
    pub const TO: usize = 0x1c;
    pub const MISSION: usize = 0x20;
    pub const ARMED: usize = 0x24;
    pub const TICKS: usize = 0x28;
    pub const LEFT: usize = 0x2c;
    pub const FROM_UP: usize = 0x30;
    pub const TO_UP: usize = 0x34;
    pub const FALL_IN: usize = 0x38;
}

/// The help text, its slot and the item that keeps it away.
const HELP: (i32, usize) = (0xa12, 0x69);
const ITEM: usize = 0x37;

fn idx(w: &MobyWorld, v: i32) -> Option<usize> { usize::try_from(v).ok().filter(|&i| i < w.mobys.len()) }

/// 0x0c35 (module docs).
pub fn update_0c35(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let fall_in = m.pv_i(pv::FALL_IN);
    if 0 < fall_in && ctx.level.in_cuboid(fall_in, ctx.hero.pos) {
        if let Some((p, _)) = ctx.level.cuboid_place(fall_in) { ctx.game.death_z = Some(p[2]); }
    } else {
        let z = m.pv_f(ctx.game.variant.max(0) as usize * 4);
        if 0.0 < z { ctx.game.death_z = Some(z); }
    }
    let level = ctx.game.level.max(0) as usize;
    let done = |ctx: &Ctx, row: i32| ctx.game.missions.get((row + level as i32 * 16).max(0) as usize).is_some_and(|&b| b & 0x80 != 0);
    match w.mobys[id].state {
        0 => {
            let armed = !done(ctx, w.mobys[id].pv_i(pv::MISSION));
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.set_pv_i(pv::ARMED, armed as i32);
            m.set_state(1, -1);
        }
        1 => {
            let m = &w.mobys[id];
            let had = ctx.game.items.get(ITEM).is_some_and(|&b| b != 0);
            if !had && ctx.level.in_cuboid(m.pv_i(pv::CUBOID), ctx.hero.pos) {
                let last = ctx.game.help_last.get(HELP.1).copied().unwrap_or(0);
                if 3600 < ctx.game.play_time.wrapping_sub(last) as u32 { ctx.game.help(HELP.0, HELP.1 as i32); }
            }
            if done(ctx, w.mobys[id].pv_i(pv::MISSION)) && w.mobys[id].pv_i(pv::ARMED) != 0 { w.mobys[id].set_state(2, -1); }
        }
        2 => fly(w, id),
        _ => {}
    }
}

/// State 2 (module docs).
fn fly(w: &mut MobyWorld, id: usize) {
    let m = &w.mobys[id];
    let (cut, from, to) = (idx(w, m.pv_i(pv::CUTAWAY)), idx(w, m.pv_i(pv::FROM)), idx(w, m.pv_i(pv::TO)));
    if w.mobys[id].first() {
        w.mobys[id].set_pv_i(pv::ARMED, 0);
        let (Some(c), Some(_), Some(_)) = (cut, from, to) else { return w.mobys[id].set_state(1, -1) };
        w.mobys[c].timer = 1;
        let n = w.mobys[c].pv_i(0x2c);
        w.mobys[id].set_pv_i(pv::TICKS, n);
        w.mobys[id].set_pv_i(pv::LEFT, n);
    }
    let (Some(c), Some(a), Some(b)) = (cut, from, to) else { return };
    let m = &w.mobys[id];
    let start = add(w.mobys[a].pos, [0.0, 0.0, m.pv_f(pv::FROM_UP)]);
    let end = add(w.mobys[b].pos, [0.0, 0.0, m.pv_f(pv::TO_UP)]);
    let yaw = atan2(end[0] - start[0], end[1] - start[1]);
    let ratio = m.pv_i(pv::LEFT) as f32 / m.pv_i(pv::TICKS).max(1) as f32;
    let t = ((1.0 - ratio) * std::f32::consts::PI - std::f32::consts::FRAC_PI_2).sin();
    let at = add(start, scale(sub(end, start), t * 0.45 + 0.45));
    let o = &mut w.mobys[c];
    o.set_pv_v(0, at);
    o.set_pv_f(0x18, yaw);
    let mut left = w.mobys[id].pv_i(pv::LEFT);
    let out = countdown(&mut left);
    w.mobys[id].set_pv_i(pv::LEFT, left);
    if out { w.mobys[id].set_state(1, -1); }
}
