//! The space levels' missions (Feltzin 0x1103 L05 `0x41efc0`, Hrugis 0x1155 L10 `0x422828`, Gorn 0x119f L15
//! `0x42f288`: the same code in each level's copy, the decomp's `func_US2_L05_0041EFBC` / `func_US2_L10_00422824` /
//! `func_US2_L15_0042F288`, asm only), on the space-AI library ([`super`]: its globals, here the mission moby's
//! words: [`super::lib`]). Each level's numbers are its [`Level`].
//!
//! **The start** (state 0): updated and drawn from anywhere, the fight's sphere `+0x68`[0] into the library (`lib +0`,
//! [`super`]'s `arena`), the ready flag (`lib +0x24`), the mission none (−2), the select up, the timers in its pvars
//! (Feltzin `+0x40` 5400 the results', `+0x44` 7200 the flamers' mission, `+0x48` 150 the failure's message, `+0x54`
//! 480 the quiet start, `+0x58` 120 the end's wait; Hrugis / Gorn their own), the hit handler (`lib +0x54`:
//! [`super::hits_with`]), the death's and reset's callbacks (`lib +0x4c` / `+0x50`; not here: the flight's death
//! loads the level again, crate::ship), the "all ships put away" flag set (`0x37b188(1)`: every ship's start puts
//! it away), the boost not slowed; the missions won (`0x1395b8 + 0x61..0x64`, Hrugis `+0x66..`, Gorn `+0x6b..`:
//! [`crate::game::GameState::flags`]) and the first win still to come (their first byte 0); the no-fly volumes (`lib
//! +0x90` = 4, the pvars 1..4; not ported, as in the library).
//!
//! **Each tick** (state 1): the tick's leading group (`0x37add0`, [`super::leader`]); the help panel 230 up
//! (`0x30d710`); on the first tick the moby at `+0x28` hooked (`0x33f318`, `+0xbe` bit 2) and the frames counted from
//! 0; nothing for 5 frames, nor while a scene plays (`0x1a8f00` = 2). With the select up the flight is held (`lib
//! +0x48`) and the select runs ([`select`], `0x420238`, shared); a mission picked starts (`0x447810`: the ship camera's
//! pose kept for its restart; not ported). Else the help 0xa17 once (`0x30d7b0(0xa17, 0x6e)`), the quiet while
//! `+0x54` runs (`lib +0x2c`), the flight free, and the mission's tick (from the level's third frame on).
//!
//! **State 3**: the cut (`0x4445d8`); over, the quiet off, a wave more (`lib +0x14`), state 1.
//!
//! **The missions** (each sets itself up on its first tick: the quiet off, every ship put away (`0x37c480`,
//! [`super::put_away_all`]), the flag cleared, its groups activated (`0x37c4f0`, [`super::activate_group`]) and
//! gathered (`0x37e150` / `0x37e878`)); the wins pay through [`reward`], the failures through [`fail`]:
//! * Feltzin 0 (`0x422838`): the cruisers' groups (0x957 / 0x97c / 0xf76) round (487, 526, 531); none left, the
//!   second wave after 120 ticks behind the cut (`0x4445a0` from (592, 594, 566) at (−0.11, 0.23, −2.79) for 200
//!   ticks; the ship at (438, 487, 535) facing (593.5, 599, 566); the groups again round the first point, going for
//!   (512.5, 530, 540)); none left again, the first win's end (after 25 ticks: the planet 6 unlocked
//!   `0x33a138`, the moby at `+0x2c` hooked, the flight held) or the reward. 1 (`0x422bb0`): the flamers 0x10f5
//!   round the sphere's centre, `+0x44` (7200 ticks) to destroy them, else the failure (0x305f). 2 (`0x422d38`): the
//!   fighters 0xf73. 3 (`0x422e88`): the gate race ([`gate_race`]).
//! * Hrugis 0 (`0x425cd8`): the carriers 0xf85 (crate::moby::classes::level10::carrier), till none of their leader's
//!   group is alive (`0x37e470`); the first win unlocks the planet 12. 1 (`0x425f48`): the tethered targets 0xcbe and
//!   the wingmen 0xf93 (round the centre, their state 2 `0x41a728`: hunting the targets), won with no wingman alive,
//!   back to the select with no target left. 2 (`0x426130`): the fighters 0xf8e. 3 (`0x426260`): the gate race.
//! * Gorn 0 (`0x432488`): the big ships 0x1211 (crate::moby::classes::level15::bigship); none alive, the cut from
//!   the ship's place (32 back, or half the way to a big ship ahead within 196, 16 down) toward it for 300 ticks, then
//!   the first win (the planet 16, the moby at `+0x28` hooked) or the reward. 1 (`0x432858`): the wingmen 0xf93. 2
//!   (`0x4329f0`): the fighters 0x957 and one big ship (`0x383038`, its group's start `0x437c40` and its cloak
//!   `0x437e50`, 450 health). 3 (`0x432ba8`): the gate race.
//!
//! **The screens** (the HUD's screen callback `0x325e00(0, …)`, crate::hud::ScreenCallback): the select's while it is
//! up ([`draw_select`], `0x420420`, Hrugis `0x423cb8`, Gorn `0x430500`), the results' from the reward on
//! ([`draw_results`], `0x421f80`, Hrugis `0x4254c8`, Gorn `0x431c88`); else, after the mission's tick with no results
//! up, the mission's HUD (`0x41f478`, not ported: no callback here).
//!
//! Not ported: the mission's HUD and pause items (`0x41f478`), the autosave after a reward
//! (`0x325a28(0, −1)`), the second help 0xa18 (Hrugis / Gorn, `0x380780`), the fog's restore on the reset, the
//! sound stopped on the reward (the ship's `+0x270`), Hrugis's 75 clouds (`0x427150(75, 16)`) and its first
//! mission's two arcs (`0x39ea50`), the gate race's spark pickup (`0x42e6b0`, class 0x126c) and the gates' own
//! update and draw (class 0x114a: its colour `+8`, on `+0xc`).

use super::{activate_group, escort, leader_of, put_away_all, set_lib_i};
use crate::hero::physics::{add, atan2, cross, dist, dot, len2, scale, sub, unit, V3};
use crate::moby::{countdown, Ctx, MobyWorld};
use crate::pad::button;

pub const FELTZIN: i16 = 0x1103;
pub const HRUGIS: i16 = 0x1155;
pub const GORN: i16 = 0x119f;

/// The port's words on the mission moby (past the library's): the mission picked (Feltzin `$gp+0x2310`, −2 none),
/// the select up (`+0x2398`), its row (`+0x239c`), the mission set up (`+0x2314`), the frames since the first tick
/// (`+0x2388`), the first tick's hook done (`+0x2384`), the help shown (`+0x2390`), the results up (`+0x23a0`), the
/// failure on (`+0x2394`), the first win to come (`+0x23a4`), Gorn's end cut done (`+0x257c`); the gate race's gates
/// (their path points + 1), their mobys (+ 1), the next point, the misses, the last tick's two dots, its ticks
/// (`0x1b3550`); the timers (the original's pvars).
mod pv {
    pub const SELECTED: usize = 0x200;
    pub const MENU: usize = 0x204;
    pub const CHOICE: usize = 0x208;
    pub const SET_UP: usize = 0x20c;
    pub const FRAMES: usize = 0x210;
    pub const HOOKED: usize = 0x214;
    pub const HELPED: usize = 0x218;
    pub const RESULTS: usize = 0x21c;
    pub const FAILING: usize = 0x220;
    pub const FIRST: usize = 0x224;
    pub const CUT_DONE: usize = 0x228;
    pub const STARTED: usize = 0x22c;
    pub const GATE_A: usize = 0x230;
    pub const GATE_B: usize = 0x234;
    pub const MOBY_A: usize = 0x238;
    pub const MOBY_B: usize = 0x23c;
    pub const NEXT: usize = 0x240;
    pub const MISSES: usize = 0x244;
    pub const DOT_SHIP: usize = 0x248;
    pub const DOT_CAM: usize = 0x24c;
    pub const RACE_TICKS: usize = 0x250;
    pub const T_RESULTS: usize = 0x260;
    pub const T_MISSION: usize = 0x264;
    pub const T_FAIL: usize = 0x268;
    pub const T_QUIET: usize = 0x26c;
    pub const T_END: usize = 0x270;
}

/// A level's numbers.
struct Level {
    /// The missions won's save bytes (`0x1395b8 +`).
    won: usize,
    /// The bolts by the mission and its wins so far (0, 1, 2 or more: Feltzin `$gp+0x23a8`, Hrugis `+0x23f8`, Gorn
    /// `+0x2598`).
    rewards: [[i32; 3]; 4],
    /// The planet mission 0's first win unlocks, and the pvar of the moby it hooks.
    planet: i32,
    hook: usize,
    /// The results' wait, the failure's message time (its reset), the end's wait.
    results: i32,
    fail: i32,
    end: i32,
}

fn level(class: i16) -> Level {
    match class {
        HRUGIS => Level { won: 0x66, rewards: [[0, 2900, 900], [9000, 3000, 1000], [10000, 3250, 1100], [11000, 3650, 1200]], planet: 12, hook: 0x2c, results: 0x278d0, fail: 0xf0, end: 0x78 },
        GORN => Level { won: 0x6b, rewards: [[0, 6500, 2250], [25000, 8250, 2750], [30000, 10000, 3250], [35000, 11750, 3750]], planet: 16, hook: 0x28, results: 0x1518, fail: 0xf0, end: 0x78 },
        _ => Level { won: 0x61, rewards: [[0, 1675, 550], [5500, 1825, 600], [6500, 2150, 700], [8000, 2650, 875]], planet: 6, hook: 0x2c, results: 0x1518, fail: 0x96, end: 0x78 },
    }
}

/// The missions won (the save's bytes).
fn won(ctx: &Ctx, lv: &Level, m: usize) -> i32 { won_in(ctx.game, lv, m) }
fn won_in(game: &crate::game::GameState, lv: &Level, m: usize) -> i32 { game.flags.get(lv.won + m).copied().unwrap_or(0) as i32 }

fn get(w: &MobyWorld, id: usize, o: usize) -> i32 { w.mobys[id].pv_i(o) }
fn set(w: &mut MobyWorld, id: usize, o: usize, v: i32) { w.mobys[id].set_pv_i(o, v) }

/// The moby of the pvar's disc index, if any.
fn moby_at(w: &MobyWorld, id: usize, o: usize) -> Option<usize> {
    let i = w.mobys[id].pv_i(o);
    if i < 0 { return None; }
    w.mobys.iter().position(|m| m.index as i32 == i && m.state < 0x80)
}

/// The timer `o` counted down: true once out (`0x307008`).
fn out(w: &mut MobyWorld, id: usize, o: usize) -> bool {
    let mut t = get(w, id, o);
    let done = countdown(&mut t);
    set(w, id, o, t);
    done
}

/// The player's ship (the mission's `+0x24`; else the ship's class).
fn ship(w: &MobyWorld, id: usize) -> Option<usize> {
    moby_at(w, id, 0x24).or_else(|| w.mobys.iter().position(|m| m.o_class == crate::ship::SHIP_CLASS && m.state < 0x80))
}

/// `0x41efc0` / `0x422828` / `0x42f288`.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let lv = level(w.mobys[id].o_class);
    match w.mobys[id].state {
        0 => {
            start(w, id, ctx, &lv);
            run(w, id, ctx, &lv);
        }
        1 => run(w, id, ctx, &lv),
        3 => {
            if !super::cutting(w) {
                set_lib_i(w, super::PASSIVE, 0);
                let n = get(w, id, super::WAVES) + 1;
                set(w, id, super::WAVES, n);
                w.mobys[id].state = 1;
            }
        }
        _ => {}
    }
}

/// The start (module docs).
fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level) {
    let first = won(ctx, lv, 0) == 0;
    let m = &mut w.mobys[id];
    m.update_dist = 0xff;
    m.draw_dist = 0x1ff;
    m.state = 1;
    for o in (pv::SELECTED..pv::T_END + 4).step_by(4) { m.set_pv_i(o, 0); }
    m.set_pv_i(pv::SELECTED, -2);
    m.set_pv_i(pv::MENU, 1);
    m.set_pv_i(pv::FIRST, first as i32);
    m.set_pv_i(pv::STARTED, 1);
    m.set_pv_i(pv::T_RESULTS, lv.results);
    m.set_pv_i(pv::T_MISSION, 0x1c20);
    m.set_pv_i(pv::T_FAIL, lv.fail);
    m.set_pv_i(pv::T_QUIET, 0x1e0);
    m.set_pv_i(pv::T_END, lv.end);
    m.set_pv_i(super::CRUISE, 0);
    m.set_pv_i(super::SLOW_BOOST, 0);
    m.set_pv_i(super::WAVES, 0);
    m.set_pv_i(super::HELD, 0);
    m.set_pv_i(super::ALL_AWAY, 1);
    w.changed = true;
}

/// State 1 (module docs).
fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level) {
    let lead = super::leader(w, &mut ctx.game.rng);
    set(w, id, super::LEADER, lead);
    ctx.game.help_offset(-230, 2);
    if get(w, id, pv::HOOKED) == 0 {
        if let Some(h) = moby_at(w, id, 0x28) { w.mobys[h].once |= 4; }
        set(w, id, pv::HOOKED, 1);
        set(w, id, pv::FRAMES, 0);
    }
    let f = get(w, id, pv::FRAMES);
    set(w, id, pv::FRAMES, f + 1);
    if f < 5 || ctx.game.scenes.mode == crate::scene::SCENE { return; }
    if get(w, id, pv::MENU) != 0 {
        set(w, id, super::HELD, 1);
        let picked = select(w, id, ctx, lv);
        set(w, id, pv::SELECTED, picked);
        return;
    }
    if get(w, id, pv::HELPED) == 0 && ctx.game.help(0xa17, 0x6e) { set(w, id, pv::HELPED, 1); }
    let quiet = !out(w, id, pv::T_QUIET);
    set(w, id, super::PASSIVE, quiet as i32);
    set(w, id, super::HELD, 0);
    let m = get(w, id, pv::SELECTED);
    if m != -2 && 3 <= ctx.game.frame { tick_mission(w, id, ctx, lv, m); }
    // `0x325e00(0, 0x41f478)` once (`$gp+0x2358`) with no results up: the mission's HUD (not ported: none here).
    if get(w, id, pv::RESULTS) == 0 && matches!(ctx.game.screen_callback, Some(crate::hud::ScreenCallback::MissionSelect { .. } | crate::hud::ScreenCallback::MissionResults { .. })) {
        ctx.game.screen_callback = None;
    }
}

/// The mission `m`'s tick (module docs).
fn tick_mission(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level, m: i32) {
    match (w.mobys[id].o_class, m) {
        (_, 3) => gate_race(w, id, ctx, lv),
        (FELTZIN, 0) => feltzin_waves(w, id, ctx, lv),
        (FELTZIN, 1) => feltzin_flamers(w, id, ctx, lv),
        (FELTZIN, 2) => destroy_all(w, id, ctx, lv, &[0x0f73], 2),
        (HRUGIS, 0) => hrugis_carriers(w, id, ctx, lv),
        (HRUGIS, 1) => hrugis_defend(w, id, ctx, lv),
        (HRUGIS, 2) => hrugis_fighters(w, id, ctx, lv),
        (GORN, 0) => gorn_big_ships(w, id, ctx, lv),
        (GORN, 1) => destroy_all(w, id, ctx, lv, &[0x0f93], 1),
        (GORN, 2) => gorn_with_big_ship(w, id, ctx, lv),
        _ => {}
    }
}

/// `0x420238` (Hrugis `0x423ad0`, Gorn `0x430318`: the one code): ← / → step the row round the four (the level's
/// sound 3), △ opens the galaxy map (`0x3401a0(3, 1, 4, 0, 0)`, crate::travel::open_menu), ✕ takes the row if it is
/// open (0 always, 1 after the first win, 2 after 0 and 1, 3 after 0, 1 and 2; the sound 4, the select down) or
/// refuses it (the sound 5). Returns the mission taken, else −2.
fn select(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level) -> i32 {
    // `0x325e00(0, 0x420420)` once while it is up (`$gp+0x235c`): the select's screen ([`draw_select`]).
    if !matches!(ctx.game.screen_callback, Some(crate::hud::ScreenCallback::MissionSelect { .. })) {
        ctx.game.screen_callback = Some(crate::hud::ScreenCallback::MissionSelect { moby: id });
    }
    let pressed = ctx.hero.items.pressed;
    let mut row = get(w, id, pv::CHOICE);
    if pressed & button::LEFT != 0 {
        ctx.host.play_level_sound(None, 3, 0);
        row -= 1;
        if row < 0 { row = 3; }
    } else if pressed & button::RIGHT != 0 {
        ctx.host.play_level_sound(None, 3, 0);
        row += 1;
        if 3 < row { row = 0; }
    }
    set(w, id, pv::CHOICE, row);
    if pressed & button::TRIANGLE != 0 { crate::travel::open_menu(ctx.game); }
    if pressed & button::CROSS != 0 {
        let open = (0..row.clamp(0, 3) as usize).all(|k| won(ctx, lv, k) != 0);
        if open {
            set(w, id, pv::MENU, 0);
            ctx.host.play_level_sound(None, 4, 0);
            return row;
        }
        ctx.host.play_level_sound(None, 5, 0);
        set(w, id, pv::MENU, 1);
    }
    -2
}

/// A mission's set-up: the quiet off, every ship put away, the flag cleared, the leaders of `classes` their groups
/// activated and, with `at`, gathered `r` round it (`0x37e150` / `0x37e878`).
fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, classes: &[i16], at: Option<(V3, f32)>) {
    set_lib_i(w, super::PASSIVE, 0);
    set(w, id, pv::SET_UP, 1);
    put_away_all(w);
    super::set_all_away(w, false);
    for l in super::leaders(w) {
        if !classes.contains(&w.mobys[l].o_class) { continue; }
        activate_group(w, l, ctx);
        if let Some((p, r)) = at {
            super::scatter_group(w, l, ctx, p, r);
            super::goal_group(w, l, ctx, p, r);
        }
    }
}

/// `0x421860` (Hrugis `0x425068`, Gorn `0x42fd40`): back to the select — the mission none, the select up, the
/// quiet on, the hull and shield refilled, the waves, the cruise speed (8) and the boost reset.
fn back_to_menu(w: &mut MobyWorld, id: usize) {
    set(w, id, pv::SET_UP, 0);
    set(w, id, pv::SELECTED, -2);
    set(w, id, pv::MENU, 1);
    set(w, id, super::PASSIVE, 1);
    set(w, id, super::REFILL, 1);
    set(w, id, super::WAVES, 0);
    w.mobys[id].set_pv_f(super::CRUISE, 8.0);
    set(w, id, super::SLOW_BOOST, 0);
}

/// `0x422530(m)` (Hrugis `0x425a78`, Gorn `0x432238`): after the end's wait, once: a 30-frame fade, the bolts (the
/// level's table by the wins so far, × 10 with the challenge mode `0x1a7a12`), the win counted (`0x424060`: into the
/// save's bytes); the flight held; ✕ then goes back to the select (a fade, the timers again, its row at the furthest
/// mission open), △ opens the galaxy map.
fn reward(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level, m: usize) {
    if ctx.game.scenes.mode == crate::scene::SCENE || !out(w, id, pv::T_END) { return; }
    if get(w, id, pv::RESULTS) == 0 {
        ctx.game.blocking_fade(30);
        // `0x325e00(0, 0x421f80)` and `0x1b34f8` = m: the results' screen ([`draw_results`]).
        ctx.game.screen_callback = Some(crate::hud::ScreenCallback::MissionResults { moby: id, mission: m });
        let n = won(ctx, lv, m);
        let mut bolts = lv.rewards[m][n.min(2) as usize];
        if ctx.game.challenge { bolts *= 10; }
        set(w, id, pv::RESULTS, 1);
        if let Some(b) = ctx.game.flags.get_mut(lv.won + m) { *b = (n + 1).min(0xff) as u8; }
        ctx.game.bolts = ctx.game.bolts.wrapping_add(bolts as u32);
    }
    set(w, id, super::HELD, 1);
    if out(w, id, pv::T_RESULTS) {
        set(w, id, pv::T_RESULTS, 600);
        return;
    }
    let pressed = ctx.hero.items.pressed;
    if pressed & button::CROSS != 0 {
        set(w, id, pv::RESULTS, 0);
        set(w, id, pv::T_RESULTS, lv.results);
        ctx.game.blocking_fade(30);
        set(w, id, super::HELD, 0);
        set(w, id, pv::T_END, 0x78);
        set(w, id, pv::T_MISSION, 0x1c20);
        let row = if won(ctx, lv, 0) == 0 { get(w, id, pv::CHOICE) } else if won(ctx, lv, 1) == 0 { 1 } else if won(ctx, lv, 2) == 0 { 2 } else { 3 };
        set(w, id, pv::CHOICE, row);
        back_to_menu(w, id);
    } else if pressed & button::TRIANGLE != 0 {
        crate::travel::open_menu(ctx.game);
    }
}

/// `0x422780(m)` (Hrugis / Gorn in their gate races): while its timer runs the failure's message (`0x3273e0`:
/// 0x305f for the flamers, 0x3074 for the gate race, crate::msgbox's second box); out, the timers again, a 30-frame
/// fade, back to the select, every ship put away, the failure over.
fn fail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level, text: i32) {
    if !out(w, id, pv::T_FAIL) {
        ctx.game.messages.push(crate::msgbox::Request::Second { text, ticks: None });
        return;
    }
    set(w, id, pv::T_FAIL, lv.fail);
    set(w, id, pv::T_MISSION, 0x1c20);
    ctx.game.blocking_fade(30);
    back_to_menu(w, id);
    put_away_all(w);
    set(w, id, pv::FAILING, 0);
}

/// The mission 0s' first win (after `n` ticks of the end's wait): the planet unlocked (`0x33a138`), the level's
/// moby hooked, the flight held; the next end is the reward.
fn first_win(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level, n: i32) {
    if !out(w, id, pv::T_END) { return; }
    set(w, id, pv::T_END, n);
    ctx.game.unlock_planet(lv.planet);
    if let Some(h) = moby_at(w, id, lv.hook) { w.mobys[h].once |= 4; }
    set(w, id, pv::FIRST, 0);
    set(w, id, super::HELD, 1);
}

/// The fight over: no ship of the library active (`lib +0x10`) past the level's sixth frame.
fn cleared(w: &MobyWorld, ctx: &Ctx) -> bool { super::active_count(w) == 0 && 6 <= ctx.game.frame }

/// Feltzin's 0 (`0x422838`; module docs).
fn feltzin_waves(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level) {
    const CLASSES: [i16; 3] = [0x0957, 0x097c, 0x0f76];
    if get(w, id, pv::SET_UP) == 0 { set_up(w, id, ctx, &CLASSES, Some(([487.0, 526.0, 531.0], 64.0))); }
    if !cleared(w, ctx) { return; }
    if get(w, id, super::WAVES) <= 0 {
        if !out(w, id, pv::T_END) { return; }
        set(w, id, pv::T_END, 0x78);
        super::ask_cut(w, [592.0, 594.0, 566.0], [-0.11, 0.23, -2.79], 200);
        w.mobys[id].state = 3;
        let (a_, b, c) = ([593.5, 599.0, 566.0], [512.5, 530.0, 540.0], [438.0, 487.0, 535.0]);
        super::place_ship(w, a_, c);
        for l in super::leaders(w) {
            if !CLASSES.contains(&w.mobys[l].o_class) { continue; }
            activate_group(w, l, ctx);
            super::scatter_group(w, l, ctx, a_, 10.0);
            super::goal_group(w, l, ctx, b, 20.0);
        }
    } else if get(w, id, pv::FIRST) != 0 {
        first_win(w, id, ctx, lv, 0x19);
        if get(w, id, pv::FIRST) == 0 { set(w, id, pv::FRAMES, 0); }
    } else if out(w, id, pv::T_END) {
        reward(w, id, ctx, lv, 0);
    }
}

/// Feltzin's 1 (`0x422bb0`): the flamers, 7200 ticks.
fn feltzin_flamers(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level) {
    if get(w, id, pv::SET_UP) == 0 {
        set(w, id, pv::T_MISSION, 0x1c20);
        let c = super::centre(w, ctx);
        set_up(w, id, ctx, &[0x10f5], Some((c, 64.0)));
    }
    if cleared(w, ctx) { return reward(w, id, ctx, lv, 1); }
    if out(w, id, pv::T_MISSION) { fail(w, id, ctx, lv, 0x305f); }
}

/// Feltzin's 2 (`0x422d38`) and Gorn's 1 (`0x432858`): the class's groups round the centre, won when none is left.
fn destroy_all(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level, classes: &[i16], m: usize) {
    if get(w, id, pv::SET_UP) == 0 {
        let c = super::centre(w, ctx);
        set_up(w, id, ctx, classes, Some((c, 64.0)));
    }
    if cleared(w, ctx) { reward(w, id, ctx, lv, m); }
}

/// Hrugis's 0 (`0x425cd8`): the carriers.
fn hrugis_carriers(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level) {
    if get(w, id, pv::SET_UP) == 0 {
        set_up(w, id, ctx, &[super::super::level10::carrier::CLASS], None);
        super::sweep(w, true);
    }
    let left = leader_of(w, super::super::level10::carrier::CLASS).map_or(0, |l| super::alive(w, l));
    if left != 0 { return; }
    put_away_all(w);
    if get(w, id, pv::FIRST) != 0 { first_win(w, id, ctx, lv, 10) } else if out(w, id, pv::T_END) { reward(w, id, ctx, lv, 0) }
}

/// Hrugis's 1 (`0x425f48`): the tethered targets and the wingmen (their state 2: `0x41a728` sets it on the group).
fn hrugis_defend(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level) {
    const TARGETS: i16 = super::super::level10::tethered::CLASS;
    const WINGMEN: i16 = super::super::level10::ships::WINGMAN;
    if get(w, id, pv::SET_UP) == 0 {
        set_up(w, id, ctx, &[TARGETS], None);
        let c = super::centre(w, ctx);
        for l in super::leaders(w) {
            if w.mobys[l].o_class != WINGMEN { continue; }
            activate_group(w, l, ctx);
            super::scatter_group(w, l, ctx, c, 64.0);
            super::goal_group(w, l, ctx, c, 64.0);
            for m in super::members(w, l) { w.mobys[m].state = 2; }
        }
        super::sweep(w, true);
    }
    if leader_of(w, WINGMEN).map_or(0, |l| super::alive(w, l)) == 0 { reward(w, id, ctx, lv, 1); }
    if !leader_of(w, TARGETS).is_some_and(|l| super::group_active(w, l)) {
        ctx.game.blocking_fade(30);
        set(w, id, super::HELD, 0);
        back_to_menu(w, id);
    }
}

/// Hrugis's 2 (`0x426130`): the fighters 0xf8e, till none of their group is active.
fn hrugis_fighters(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level) {
    const FIGHTERS: i16 = super::super::level10::ships::FIGHTER;
    if get(w, id, pv::SET_UP) == 0 {
        set_up(w, id, ctx, &[FIGHTERS], None);
        super::sweep(w, true);
    }
    if !leader_of(w, FIGHTERS).is_some_and(|l| super::group_active(w, l)) { reward(w, id, ctx, lv, 2); }
}

/// Gorn's 0 (`0x432488`): the big ships, the end's cut.
fn gorn_big_ships(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level) {
    const BIG: i16 = super::super::level15::bigship::CLASS;
    if get(w, id, pv::SET_UP) == 0 {
        set_up(w, id, ctx, &[BIG], None);
        set(w, id, pv::T_END, 0x1e);
    }
    if leader_of(w, BIG).is_some_and(|l| super::alive(w, l) != 0) || ctx.game.frame < 6 { return; }
    if get(w, id, pv::CUT_DONE) == 0 {
        if !out(w, id, pv::T_END) { return; }
        set(w, id, pv::T_END, 0x3c);
        // The cut's place: from the ship (the mission's `+0x20`) along its facing — half the way to a big ship met
        // within 196 (32 back when nearer than 16), 16 down — looking at the ship (`0x338b40`).
        let s = moby_at(w, id, 0x20).or_else(|| ship(w, id));
        if let Some(s) = s {
            let (p, fwd) = (w.mobys[s].pos, w.mobys[s].rows[0]);
            let far = add(p, scale(fwd, 196.0));
            let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, p, far, crate::collision::QueryFlags(0), Some(s)));
            let mut d = 16.0;
            if let Some(h) = hit.filter(|h| h.moby.is_some_and(|m| w.mobys.get(m).is_some_and(|m| m.o_class == BIG))) { d = dist(h.point, p); }
            let d = if d < 16.0 { -32.0 } else { d * 0.5 };
            let mut at = add(p, scale(fwd, d));
            at[2] -= 16.0;
            let to = sub(p, at);
            let rot = [0.0, -atan2(len2(to), to[2]), atan2(to[0], to[1])];
            super::ask_cut(w, at, rot, 300);
        }
        w.mobys[id].state = 3;
        set(w, id, pv::CUT_DONE, 1);
    } else if get(w, id, pv::FIRST) != 0 {
        if !out(w, id, pv::T_END) { return; }
        set(w, id, pv::T_END, 10);
        ctx.game.unlock_planet(lv.planet);
        if let Some(h) = moby_at(w, id, lv.hook) { w.mobys[h].once |= 4; }
        set(w, id, pv::FRAMES, 0);
        set(w, id, super::HELD, 1);
        set(w, id, pv::FIRST, 0);
    } else if out(w, id, pv::T_END) {
        put_away_all(w);
        reward(w, id, ctx, lv, 0);
    }
}

/// Gorn's 2 (`0x4329f0`): the fighters 0x957 round the centre and one big ship.
fn gorn_with_big_ship(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level) {
    const BIG: i16 = super::super::level15::bigship::CLASS;
    if get(w, id, pv::SET_UP) == 0 {
        let c = super::centre(w, ctx);
        set_up(w, id, ctx, &[0x0957], Some((c, 64.0)));
        if let Some(b) = super::activate_one(w, BIG, ctx) {
            super::super::level15::bigship::on_activate(w, b, ctx);
            super::super::level15::bigship::cloak(w, b);
        }
    }
    if leader_of(w, BIG).map_or(0, |l| super::alive(w, l)) == 0 && 6 <= ctx.game.frame { reward(w, id, ctx, lv, 2); }
}

/// The gate class (`0x29efd0(0x114a)`).
pub const GATE: i16 = 0x114a;

/// A gate at path point `i`: its place and frame (along the path to the next point; the last one's from the one
/// before; left across it and the level's up, up from those).
fn gate_frame(path: &[[f32; 4]], i: usize) -> (V3, [V3; 3]) {
    let p = |k: usize| [path[k][0], path[k][1], path[k][2]];
    let fwd = if i + 1 == path.len() { unit(sub(p(i), p(i.saturating_sub(1)))) } else { unit(sub(p(i + 1), p(i))) };
    let right = unit(cross(fwd, [0.0, 0.0, 1.0]));
    (p(i), [fwd, right, cross(right, fwd)])
}

/// The next gate from the path's point `NEXT` on (the points whose w is not −1): its point + 1 and its moby + 1.
fn next_gate(w: &mut MobyWorld, id: usize, ctx: &Ctx, path: &[[f32; 4]]) -> (i32, i32) {
    let mut i = get(w, id, pv::NEXT);
    let mut found = (0, 0);
    while found.0 == 0 && (i as usize) < path.len() {
        if path[i as usize][3] != -1.0 {
            let (at, rows) = gate_frame(path, i as usize);
            let mut m = 0;
            if w.class(GATE).is_some() {
                let g = w.create(GATE);
                let gm = &mut w.mobys[g];
                gm.state = 0;
                gm.update_dist = 0xff;
                gm.draw_dist = 0x1ff;
                gm.drawn = 1;
                gm.mode |= crate::moby::mode::KEEP_ROWS;
                gm.pos = at;
                gm.rows = rows;
                w.rebuild(g, &*ctx.host);
                m = g as i32 + 1;
            }
            found = (i + 1, m);
        }
        i += 1;
    }
    set(w, id, pv::NEXT, i);
    found
}

fn drop_gate(w: &mut MobyWorld, m: i32) {
    if let Some(g) = usize::try_from(m - 1).ok().filter(|&g| g < w.mobys.len()) { w.delete(g); }
}

/// `0x422e88` (Hrugis `0x426260`, Gorn `0x432ba8`: the same course): the escorts' groups 0xf82 along the route (the
/// mission's `+0x20`, `lib +0x38`), the boost slowed; the gates (0x114a) two at a time at the route's points
/// ([`gate_frame`]). Past a gate's plane (the ship or the camera in front of it, its dot along the gate's facing
/// positive and changed since the last tick): within 1.25 of it a pass (the ship's sound 15, the cruise 0.5 faster)
/// else a miss (0.75 slower, counted), and the next gate. No gate left, the reward; the escort at its route's end
/// (`0x418ce8`) or 12 misses, the failure (0x3074). The first gate green while the next follows (0x40008000; the
/// last 0x4000c6ff), the next 0x40808000.
fn gate_race(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lv: &Level) {
    let path = usize::try_from(get(w, id, 0x20)).ok().and_then(|p| ctx.level.paths.get(p)).cloned().unwrap_or_default();
    if get(w, id, pv::SET_UP) == 0 {
        for o in [pv::MOBY_A, pv::MOBY_B] {
            let m = get(w, id, o);
            drop_gate(w, m);
        }
        for o in [pv::GATE_A, pv::GATE_B, pv::MOBY_A, pv::MOBY_B, pv::NEXT, pv::MISSES, pv::RACE_TICKS] { set(w, id, o, 0); }
        w.mobys[id].set_pv_f(pv::DOT_SHIP, -1.0);
        w.mobys[id].set_pv_f(pv::DOT_CAM, -1.0);
        set(w, id, pv::SET_UP, 1);
        set_lib_i(w, super::PASSIVE, 0);
        put_away_all(w);
        set(w, id, super::SLOW_BOOST, 1);
        let route = get(w, id, 0x20);
        set(w, id, super::ROUTE, route);
        super::set_all_away(w, false);
        for l in super::leaders(w) {
            if w.mobys[l].o_class == escort::CLASS { activate_group(w, l, ctx); }
        }
        let (g, m) = next_gate(w, id, ctx, &path);
        set(w, id, pv::GATE_A, g);
        set(w, id, pv::MOBY_A, m);
        let (g, m) = next_gate(w, id, ctx, &path);
        set(w, id, pv::GATE_B, g);
        set(w, id, pv::MOBY_B, m);
    }
    if get(w, id, pv::GATE_A) == 0 && get(w, id, pv::GATE_B) == 0 && get(w, id, pv::FAILING) == 0 { return reward(w, id, ctx, lv, 3); }
    if escort::done(w) || 12 <= get(w, id, pv::MISSES) {
        set(w, id, pv::FAILING, 1);
        fail(w, id, ctx, lv, 0x3074);
        put_away_all(w);
    }
    if get(w, id, pv::FAILING) != 0 { return; }
    let t = get(w, id, pv::RACE_TICKS) + 1;
    set(w, id, pv::RACE_TICKS, t);
    if t < 2 { return; }
    let Some(gi) = usize::try_from(get(w, id, pv::GATE_A) - 1).ok().filter(|&g| g < path.len()) else { return };
    let (at, rows) = gate_frame(&path, gi);
    let cam = ctx.host.camera_pos();
    let me = ship(w, id).map_or(cam, |s| w.mobys[s].pos);
    let dot_ship = dot(rows[0], unit(sub(me, at)));
    let dot_cam = dot(rows[0], unit(sub(cam, at)));
    let (old_ship, old_cam) = (w.mobys[id].pv_f(pv::DOT_SHIP), w.mobys[id].pv_f(pv::DOT_CAM));
    if (dot_ship != old_ship && 0.0 < dot_ship) || (dot_cam != old_cam && 0.0 < dot_cam) {
        if dist(cam, at) <= 1.25 || dist(me, at) <= 1.25 {
            super::add_cruise(w, 0.5);
            ctx.host.play_sound_of(&w.mobys[id], crate::ship::SHIP_CLASS, 15, 0);
        } else {
            super::add_cruise(w, -0.75);
            let n = get(w, id, pv::MISSES) + 1;
            set(w, id, pv::MISSES, n);
        }
        let m = get(w, id, pv::MOBY_A);
        drop_gate(w, m);
        let (b, mb) = (get(w, id, pv::GATE_B), get(w, id, pv::MOBY_B));
        set(w, id, pv::GATE_A, b);
        set(w, id, pv::MOBY_A, mb);
        let (g, m) = next_gate(w, id, ctx, &path);
        set(w, id, pv::GATE_B, g);
        set(w, id, pv::MOBY_B, m);
    }
    w.mobys[id].set_pv_f(pv::DOT_SHIP, dot_ship);
    w.mobys[id].set_pv_f(pv::DOT_CAM, dot_cam);
    // `0x424980(gate, 1)` / `0x424970(gate, colour)`: the gates' words.
    let last = get(w, id, pv::GATE_B) == 0;
    for (o, colour) in [(pv::MOBY_A, if last { 0x4000_c6ff_u32 } else { 0x4000_8000 }), (pv::MOBY_B, 0x4080_8000)] {
        if let Some(g) = usize::try_from(get(w, id, o) - 1).ok().filter(|&g| g < w.mobys.len()) {
            w.mobys[g].set_pv_i(0xc, 1);
            w.mobys[g].set_pv_i(8, colour as i32);
        }
    }
}

/// How a select's mission title is drawn: font 1 centred (`0x2ea570`), font 1 centred at a scale on its own line
/// (`0x2ea738`), font 2 centred (`0x2ea608`).
#[derive(Clone, Copy)]
enum Title {
    One,
    Scaled(f32, f32),
    Two,
}

/// The screens' texts by level (module docs): the missions' titles, their lines, the results' line with no bolts.
struct Screens {
    titles: [(u32, Title); 4],
    lines: [u32; 4],
    no_bolts: u32,
}

fn screens(class: i16) -> Screens {
    match class {
        HRUGIS => Screens { titles: [(0x2e78, Title::Scaled(0.9, 101.0)), (0x2e79, Title::One), (0x2e7b, Title::Two), (0x2e7c, Title::Two)], lines: [0x2e7d, 0x2e7e, 0x2e7f, 0x2e81], no_bolts: 0x11fb },
        GORN => Screens { titles: [(0x2e8a, Title::One), (0x2e8b, Title::One), (0x2e8c, Title::One), (0x2e8d, Title::One)], lines: [0x2e85, 0x2e86, 0x2e87, 0x2e88], no_bolts: 0x11ff },
        _ => Screens { titles: [(0x2e6c, Title::One), (0x2e6e, Title::Scaled(0.85, 102.0)), (0x2e6d, Title::One), (0x2e70, Title::One)], lines: [0x2e71, 0x2e73, 0x2e72, 0x2e75], no_bolts: 0x11f5 },
    }
}

const WHITE: u32 = 0x80f0_f0f0;
/// The screens' box for the lines (`0x303000(117, y, 280, 400, …)`).
const BOX: (i32, i32, i32) = (0x75, 0x118, 400);

fn text(out: &mut Vec<crate::hud::ScreenDraw>, font: u8, text: Vec<u8>, pos: [f32; 2], scale: f32, align: crate::hud::Align) {
    out.push(crate::hud::ScreenDraw::Text { font, text, pos, scale, colour: WHITE, align });
}

fn boxed(out: &mut Vec<crate::hud::ScreenDraw>, text: Vec<u8>, y: i32) {
    out.push(crate::hud::ScreenDraw::Box { font: 2, text, rect: [BOX.0, y, BOX.1, BOX.2], colour: WHITE });
}

/// The screens' panel: the GUI sprites 0x45 (`0x60442d00`) and 0x44 (`0x55f0c070`) at (256, 200).
fn panel(out: &mut Vec<crate::hud::ScreenDraw>) {
    for (id, colour) in [(0x45, 0x6044_2d00u32), (0x44, 0x55f0_c070)] {
        out.push(crate::hud::ScreenDraw::Hud(crate::hud::Draw::Sprite { id, pos: [256.0, 200.0], scale: [1.0, 1.0], frame: 0.0, colour }));
    }
}

/// The bolts of mission `m` at its `n`-th win (the level's table, × 10 in the challenge mode `0x1a7a12`).
fn bolts_of(game: &crate::game::GameState, lv: &Level, m: usize, n: i32) -> i32 {
    let b = lv.rewards[m][n.clamp(0, 2) as usize];
    if game.challenge { b * 10 } else { b }
}

/// `0x420420` (Hrugis `0x423cb8`, Gorn `0x430500`: the same code on their texts, [`screens`]): the select's screen,
/// in the counters' place; outside of play (`0x1a8f00` ≠ 0) nothing is drawn. The panel; the arrows 0x4a at (259,
/// 291) and 0x4b at (251, 291), each `0x80007fff` while → (←) is held, else pulsing over 50 frames (`0x1b1fc8`)
/// between `0x80ffde8d` and `0x60442d00`; the row's number (0x2e08, font 3 centred at (256, 46)). A row not open (1
/// before mission 0's first win, 2 before 0's and 1's, 3 before 0's, 1's and 2's): 0x2e09 (font 3 centred, 170 down).
/// An open one: its title (101 down: [`Title`]), its line in the box at (117, 131), its bolts at its next win (0x2e76,
/// in the box at (117, 242); for row 0 only when some) and 0x2dd5 (font 2, left, at (181, 294)). Then 0x2dd6 (119,
/// 294), 0x2dd7 (356, 294) and 0x2dd8 (279, 294), font 2, left.
///
/// Not ported: `$gp+0x2ef4` (cleared), the other languages' places (`0x1a7bbc`: English here), the PAL sizes.
pub fn draw_select(w: &MobyWorld, id: usize, inp: &crate::hud::ScreenIn, out: &mut Vec<crate::hud::ScreenDraw>) -> bool {
    use crate::hud::{format_int, lerp_rgba, text_of, Align, Draw, ScreenDraw};
    let Some(m) = w.mobys.get(id) else { return true };
    if inp.game.scenes.mode != crate::scene::PLAY { return true; }
    let (lv, sc) = (level(m.o_class), screens(m.o_class));
    panel(out);
    let k = w.frame.rem_euclid(50);
    let t = if k < 25 { k as f32 * 0.04 } else { 1.0 - (k - 25) as f32 * 0.04 };
    let pulse = lerp_rgba(t, 0x80ff_de8d, 0x6044_2d00);
    let held = inp.hero.items.held;
    for (sprite, x, b) in [(0x4a, 259.0, button::RIGHT), (0x4b, 251.0, button::LEFT)] {
        let colour = if held & b != 0 { 0x8000_7fff } else { pulse };
        out.push(ScreenDraw::Hud(Draw::Sprite { id: sprite, pos: [x, 291.0], scale: [1.0, 1.0], frame: 0.0, colour }));
    }
    let row = m.pv_i(pv::CHOICE);
    text(out, 3, format_int(&text_of(inp, 0x2e08, usize::MAX), row + 1), [256.0, 46.0], 1.0, Align::Centre);
    if (0..4).contains(&row) {
        let r = row as usize;
        if (0..r).any(|k| won_in(inp.game, &lv, k) == 0) {
            text(out, 3, text_of(inp, 0x2e09, usize::MAX), [256.0, 170.0], 1.0, Align::Centre);
        } else {
            let (title, how) = sc.titles[r];
            let t = text_of(inp, title, usize::MAX);
            match how {
                Title::One => text(out, 1, t, [256.0, 101.0], 1.0, Align::Centre),
                Title::Scaled(s, y) => text(out, 1, t, [256.0, y], s, Align::Centre),
                Title::Two => text(out, 2, t, [256.0, 101.0], 1.0, Align::Centre),
            }
            boxed(out, text_of(inp, sc.lines[r], usize::MAX), 131);
            let bolts = bolts_of(inp.game, &lv, r, won_in(inp.game, &lv, r));
            if r != 0 || bolts != 0 { boxed(out, format_int(&text_of(inp, 0x2e76, usize::MAX), bolts), 242); }
            text(out, 2, text_of(inp, 0x2dd5, usize::MAX), [181.0, 294.0], 1.0, Align::Left);
        }
    }
    for (t, x) in [(0x2dd6, 119.0), (0x2dd7, 356.0), (0x2dd8, 279.0)] {
        text(out, 2, text_of(inp, t, usize::MAX), [x, 294.0], 1.0, Align::Left);
    }
    true
}

/// `0x421f80` (Hrugis `0x4254c8`, Gorn `0x431c88`): the results of mission `mission`, in the counters' place. The
/// panel; 0x2e29 (font 2 centred at (256, 46)); 0x3061 in the box at (117, 131); the bolts its last win paid (0x2e5c,
/// or with none the level's no-bolts line, in the box at (117, 210)); 0x3062 at (117, 294) and 0x3063 at (274, 294),
/// font 2, left.
///
/// Not ported: the row's number it formats and does not draw (0x2e08), the other languages' places, the PAL sizes.
pub fn draw_results(w: &MobyWorld, id: usize, mission: usize, inp: &crate::hud::ScreenIn, out: &mut Vec<crate::hud::ScreenDraw>) -> bool {
    use crate::hud::{format_int, text_of, Align};
    let Some(m) = w.mobys.get(id) else { return true };
    let (lv, sc) = (level(m.o_class), screens(m.o_class));
    let mission = mission.min(3);
    panel(out);
    text(out, 2, text_of(inp, 0x2e29, usize::MAX), [256.0, 46.0], 1.0, Align::Centre);
    boxed(out, text_of(inp, 0x3061, usize::MAX), 131);
    let bolts = bolts_of(inp.game, &lv, mission, won_in(inp.game, &lv, mission) - 1);
    let line = if bolts == 0 { text_of(inp, sc.no_bolts, usize::MAX) } else { format_int(&text_of(inp, 0x2e5c, usize::MAX), bolts) };
    boxed(out, line, 210);
    text(out, 2, text_of(inp, 0x3062, usize::MAX), [117.0, 294.0], 1.0, Align::Left);
    text(out, 2, text_of(inp, 0x3063, usize::MAX), [274.0, 294.0], 1.0, Align::Left);
    true
}
