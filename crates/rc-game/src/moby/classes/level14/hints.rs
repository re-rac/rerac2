//! The prison's hints and its wrench upgrade, **0x0c30** (`0x3f64e0`, one placed; the decomp's
//! `func_US2_L14_003F64DC`). Its pvars: the cuboids `+0x00` (the upgrade's), `+0x04`, `+0x08`, `+0x0c`, `+0x14`;
//! `+0x10` a mission; `+0x18` its ticks in its state; `+0x20` the moby it shows (−1 none; the port keeps it, the
//! original its pointer `+0x24`); `+0x28` that moby turns.
//!
//! "Had" below is the wrench at its level 2 or more (5 in the challenge mode, `0x2fcda8` = L00 `0x2f51a0`; the
//! weapons' table `0x2686f0` by its row `0x139568 + 10`: crate::game::GameState::upgrade_level).
//! * **0**: updated at any distance, `+0x96` 0. Not had: the moby turns (when there is one). Had: the moby hidden
//!   (mode | 1, `+0x31` 0), it does not. 1.
//! * **1**: (first tick: its ticks 0.) Ratchet in `+0x00` and not had: 2. The moby turns about its point (0, −0.1, 0)
//!   by 0.05 rad a tick about x (`0x334b60`, crate::moby::attach::pivot_turn).
//! * **2**: (first tick: its ticks 0.) Out of a scene (`0x1a8f00` ≠ 2) and 6 ticks on: the wrench to its level 2 (5)
//!   (`0x2fbfd8` = L00 `0x2f43d0`, crate::game::GameState::set_upgrade) and its model (`0x3068e0(2)`, not ported, as
//!   L08's `0x30b480`), the moby hidden and still; 1.
//! * Then every tick (the asks crate::game::GameState::help_again / help_after_first, `0x2fce78` = L00 `0x2f5270`):
//!   - in `+0x04` without the flag `0x139630` bit 1 (`0x1395b8 + 0x78`): 0xa2c (slot 0x83) past 1080 ticks;
//!   - in `+0x08`: 0xa09 (0x60) after its first, and the slot 0x61 off (`0x18cde4`, its count 0xffff);
//!   - in `+0x0c` with a zap robot 0x0a59 about Ratchet (`$gp+0x3274`, crate::moby::classes::endako_d::near_hero):
//!     0xa0a (0x61) after its first;
//!   - in `+0x14` with the mission `+0x10` not done and the slot 0x5e shown under twice: 0xa07 (0x5e) after its
//!     first;
//!   - its ticks on by one.

use crate::moby::{attach, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c30;

mod pv {
    pub const UPGRADE_IN: usize = 0x00;
    pub const FLAG_IN: usize = 0x04;
    pub const SEEN_IN: usize = 0x08;
    pub const ZAP_IN: usize = 0x0c;
    pub const MISSION: usize = 0x10;
    pub const HINT_IN: usize = 0x14;
    pub const TICKS: usize = 0x18;
    pub const MOBY: usize = 0x20;
    pub const TURNS: usize = 0x28;
}

const WRENCH: i32 = crate::hero::items::WRENCH;
/// `0x1395b8 + 0x78`, bit 1.
const FLAG: usize = 0x78;

fn moby(w: &MobyWorld, id: usize) -> Option<usize> { usize::try_from(w.mobys[id].pv_i(pv::MOBY)).ok().filter(|&k| k < w.mobys.len()) }

/// Hidden: mode | 1, `+0x31` 0.
fn hide(w: &mut MobyWorld, k: Option<usize>) {
    if let Some(m) = k.and_then(|k| w.mobys.get_mut(k)) {
        m.mode |= 1;
        m.drawn = 0;
    }
}

/// 0x0c30 (module docs).
pub fn update_0c30(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let limit = if ctx.game.challenge { 5 } else { 2 };
    let had = |ctx: &Ctx| limit <= ctx.game.upgrade_level(WRENCH) as i32;
    let k = moby(w, id);
    let inside = |ctx: &Ctx, w: &MobyWorld, off: usize| ctx.level.in_cuboid(w.mobys[id].pv_i(off), ctx.hero.pos);
    match w.mobys[id].state {
        0 => {
            w.mobys[id].state_ticks = 0;
            w.mobys[id].update_dist = 0xff;
            if had(ctx) { hide(w, k); }
            let turns = !had(ctx) && k.is_some();
            w.mobys[id].set_pv_i(pv::TURNS, turns as i32);
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if w.mobys[id].first() { w.mobys[id].set_pv_i(pv::TICKS, 0); }
            if inside(ctx, w, pv::UPGRADE_IN) && !had(ctx) { w.mobys[id].set_state(2, -1); }
            if w.mobys[id].pv_i(pv::TURNS) != 0 {
                if let Some(m) = k.and_then(|k| w.mobys.get_mut(k)) { attach::pivot_turn(m, [0.05, 0.0, 0.0], [0.0, -0.1, 0.0]); }
            }
        }
        2 => {
            if w.mobys[id].first() { w.mobys[id].set_pv_i(pv::TICKS, 0); }
            if ctx.game.scenes.mode != crate::scene::SCENE && 6 <= w.mobys[id].pv_i(pv::TICKS) {
                ctx.game.set_upgrade(&ctx.level.items, WRENCH, limit);
                hide(w, k);
                w.mobys[id].set_pv_i(pv::TURNS, 0);
                w.mobys[id].set_state(1, -1);
            }
        }
        _ => {}
    }
    if inside(ctx, w, pv::FLAG_IN) && ctx.game.flags.get(FLAG).is_some_and(|&b| b & 2 == 0) {
        ctx.game.help_again(0xa2c, 0x83, 1080);
    }
    if inside(ctx, w, pv::SEEN_IN) {
        ctx.game.help_after_first(0xa09, 0x60);
        if let Some(c) = ctx.game.help_counts.get_mut(0x61) { *c = -1; }
    }
    if inside(ctx, w, pv::ZAP_IN) && super::super::endako_d::near_hero(w, ctx, super::super::endako_d::ROBOT_A59) != 0 {
        ctx.game.help_after_first(0xa0a, 0x61);
    }
    let level = ctx.game.level.max(0);
    let row = (w.mobys[id].pv_i(pv::MISSION) + level * 16).max(0) as usize;
    let done = ctx.game.missions.get(row).is_some_and(|&b| b & 0x80 != 0);
    if inside(ctx, w, pv::HINT_IN) && !done && (ctx.game.help_counts.get(0x5e).copied().unwrap_or(0) as u16) < 2 {
        ctx.game.help_after_first(0xa07, 0x5e);
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::TICKS, m.pv_i(pv::TICKS) + 1);
}
