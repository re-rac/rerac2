//! Ratchet's hoverbike **0x0bbc** (L04 `0x41b020`, L11 `0x40ab70`; the decomp's `func_US2_L04_0041B01C`), the race
//! master (crate::moby::classes::race, L04 addresses). Ratchet rides it in his state 0x31 (crate::hero::vehicle):
//! hidden (the view lock), his collision off, placed at it every tick; its pilot moby `+0x1dc` (his model on the
//! bike) is shown in its place.
//!
//! **Its pvars** (0x770; the decomp's `RaceState`): `+0x20` / `+0x24` the sphere's radius and height (0.25, 0.1 once
//! mounted), `+0x110[32]` the respawn cuboids, `+0x190` / `+0x194` the finish's and the half way's cuboids, `+0x198`
//! the start's cuboid, `+0x19c` the cuboid Ratchet is left at, `+0x1a0` Barlow's short cut cuboid, `+0x1a4` Joba's
//! gate moby, `+0x1a8..` the route set (crate::moby::classes::race::route), `+0x1dc` the pilot, `+0x1e0` / `+0x238`
//! mobys hidden while it is ridable, `+0x1e4[21]` the racers, `+0x23c[4]` the short cut's gates, `+0x254` the jets'
//! group, `+0x25c` the turbo pickups' group (0x0a99), `+0x260` the weapon pickups' group (0x0ab8), `+0x26c` the height
//! it is lost under, `+0x270` the respawn cuboid, `+0x274` / `+0x276` in the air and its ticks, `+0x278` the jet
//! boost's ticks, `+0x27c` the boost, `+0x280` its last move, `+0x288` the speed's rate, `+0x28c` the view's angle,
//! `+0x290` / `+0x2a0` its velocity now and before, `+0x2b0` the ground's normal, `+0x2c0` the eased stick,
//! `+0x2d0` / `+0x2e0` its home's place and angles, `+0x2f0` a push, `+0x300` / `+0x304` / `+0x308` the roll's,
//! pitch's and yaw's speeds, `+0x30c` its speed, `+0x310` how far along the race it is (`+0x314` / `+0x318` the
//! branch and point; `+0x3b8` the last), `+0x31c` past half way, `+0x320` Ratchet's place, `+0x338` / `+0x33c` the
//! turbo's ticks and charges, `+0x340` / `+0x344` the races lost and won, `+0x348` / `+0x349` a bounce (`+0x34c` its
//! ticks, `+0x354` its lift), `+0x34a` / `+0x36c` the pilot's flinch and its rest, `+0x358` its fall, `+0x370[6]`
//! random phases, `+0x38c` the bob's phase, `+0x394` the countdown, `+0x398` the results' wait, `+0x3a4` the ticks
//! running backwards, `+0x3a8` the rank's rest after a branch, `+0x3ac` / `+0x3b0` the nearest racer, `+0x480` /
//! `+0x484` / `+0x488` / `+0x48c` / `+0x75c` music, `+0x4c4` the racer following it, `+0x4c8` the pilot's swerve,
//! `+0x63c` the follower's pull, `+0x64c` / `+0x71c` a push's ticks and its once, `+0x650` the boost shown,
//! `+0x654` / `+0x6d4` / `+0x6a0` / `+0x6d8` the auto-pilot's index, fraction, point and place along `+0x6d0`,
//! `+0x6e0[11]` the racers' routes (copied from the first), `+0x70c` the walls in a row, `+0x750` the ticks stuck.
//! The port's: `+0x6d0` holds the route's index.
//!
//! **States** ([`update`]): **0** set-up; **1** parked (bobbing at its home; within 5.5 of Ratchet, △: mounted, the
//! start's cuboid); **2** racing, by its sub-state `+0xbc`: 0 the race select ([`select`]), 1 ridden after a retry
//! (any button: 3), 2 the start ([`start`]), 3 the racers on the grid, 4 the countdown (the sound 0xd at 2, 1.5, 1
//! and 0.5 s, 0xe at the end; ✕ not held then: the start boost may be taken), 5 the race ([`ride`], [`finish`]; lost
//! under `+0x26c` or off Joba's track: 6), 6 the respawn (100 ticks after a crash: at the respawn cuboid `+0x270`);
//! **3** finished (the auto-pilot, the results' wait `+0x398`, △); **4** off (back to the select after a race, else
//! Ratchet left at `+0x19c`, the bike home, 1).
//!
//! A scene hides it (not on Joba) until it ends.
//!
//! **The screens**: the select's ([`draw_select`]), the race's HUD from the start and the standings after the finish
//! (crate::moby::classes::race::hud). The view's angle `+0x28c` is the view's (`0x1bdc70`) while racing ([`boosts`])
//! and in the select, the play's again once Ratchet is off.
//!
//! Not ported (left, with their addresses):
//! * The bike's glow callback (`0x40e2e0` → `0x40e658` / `0x40e340`); the results' message box (`0x1a7cd0` /
//!   `0x1a7cd4`: △ goes on at once here).
//! * The pause menu's race items (`0x40c170` restart, `0x40c190` quit, `0x40c1a8` the countdown again: the state 0
//!   puts them in the pause list's slots `0x1aa6c0` / `0x1aa6c8` / `0x1aa6cc`; they set `0x1a8ee4` 1, `0x1a8ed0` 2 and
//!   the words `$gp+0x18c8` / `$gp+0x18d0` read here, crate::moby::classes::race::Globals::restart / quit): the pause
//!   list has no level rows.
//! * The weapons' projectiles ([`weapons`]), the draft (`0x41a2f8`: the lock on the racer `+0x734` ahead, its ring
//!   `0x273318` and missile), the post-finish route choice (`0x4143d0`), the trails (`0x417528`, `0x416390`), the
//!   sprays and sparks (`0x40d260`, `0x40d3e0`, `0x40d5d0`, `0x40d878`), the crash's debris (`0x408538`) and camera
//!   shake (`0x1b96e0..`), the afterimage (`0x322a98` on `+0x4e0`), the pilot's Clank (0x12df / 0x50a) and head aim
//!   (`+0x3c0`), the autosave after a race (`0x30d658`; `0x1a8ed0[5]`, `0x132af8(93)`), the fades (`0x308410`), the
//!   help texts on the set-up (`0x3153d0(0x36)`, `0x3446f8(5)`), the race camera (not looked into: the play's
//!   camera follows the hidden Ratchet here).
//! * The gp word `$gp+0x3018` the mount waits for (≥ 4): taken as met.

use super::{find_progress, nearest_index, off_track, slide, track_progress, RouteSet};
use crate::hero::physics::{add, add_rot, approach, atan2, dist, dot, len, scale, set_len, spring, sub, sub_rot, turn_spring};
use crate::moby::{ai, countdown, Ctx, Moby, MobyWorld};
use crate::pad::button;
use std::f32::consts::PI;

fn idx(m: &Moby, off: usize) -> Option<usize> { usize::try_from(m.pv_i(off)).ok() }

/// The pilot moby (`+0x1dc`).
pub fn pilot(w: &MobyWorld, bike: usize) -> Option<usize> { idx(&w.mobys[bike], 0x1dc).filter(|&p| p < w.mobys.len()) }

/// The racers (`+0x1e4[21]`).
fn racers(w: &MobyWorld, id: usize) -> Vec<usize> {
    (0..21).filter_map(|k| idx(&w.mobys[id], 0x1e4 + 4 * k)).filter(|&r| r < w.mobys.len()).collect()
}

/// The race's wins (`0x40b6c0`: the flag byte `0x1395b8 + 0x4d + k`, Joba's `+0x57`) and losses (`0x40b758`: `+0x52`,
/// Joba's `+0x5c`) of race `k`.
fn wins(ctx: &Ctx, k: i32) -> i32 { wins_in(ctx.game, k) }
pub(super) fn wins_in(game: &crate::game::GameState, k: i32) -> i32 { flag(game, if game.level == 0xb { 0x57 } else { 0x4d }, k) }
fn losses(ctx: &Ctx, k: i32) -> i32 { flag(ctx.game, if ctx.game.level == 0xb { 0x5c } else { 0x52 }, k) }
fn flag(game: &crate::game::GameState, base: usize, k: i32) -> i32 {
    if !matches!(game.level, 4 | 0xb) { return 0; }
    usize::try_from(k).ok().and_then(|k| game.flags.get(base + k)).map_or(0, |&b| b as i32)
}
fn bump_flag(ctx: &mut Ctx, base4: usize, base11: usize, k: i32) {
    let base = if ctx.game.level == 0xb { base11 } else { base4 };
    if !matches!(ctx.game.level, 4 | 0xb) { return; }
    if let Some(b) = usize::try_from(k).ok().and_then(|k| ctx.game.flags.get_mut(base + k)) { *b = b.wrapping_add(1); }
}

/// The race chosen (`$gp+0x18cc`; Joba's `$gp+0x185c`, also crate::moby::classes::level11::Globals::event).
fn set_race(w: &mut MobyWorld, ctx: &Ctx, k: i32) {
    w.race.race = k;
    if ctx.game.level == 0xb { w.level11.event = k; }
}

fn in_scene(ctx: &Ctx) -> bool { ctx.game.scenes.mode == crate::scene::SCENE }

fn class_solid(w: &MobyWorld, m: usize) -> bool { w.class(w.mobys[m].o_class).is_some_and(|c| c.has_collision) }

pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0x770 { w.mobys[id].pvars.resize(0x770, 0); }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    let scene = in_scene(ctx);
    if scene && !w.race.hidden && ctx.game.level != 0xb {
        let m = &mut w.mobys[id];
        m.has_collision = false;
        m.mode = (m.mode & 0xefff) | 0x41;
        w.race.hidden = true;
        return;
    }
    if !scene && w.race.hidden {
        let solid = class_solid(w, id);
        let m = &mut w.mobys[id];
        m.mode = (m.mode & 0xffbe) | 0x1000;
        m.has_collision = solid;
        w.race.hidden = false;
    }
    match w.mobys[id].state {
        0 => set_up(w, id, ctx),
        1 => parked(w, id, ctx),
        2 => racing(w, id, ctx),
        3 => finished(w, id, ctx),
        4 => off(w, id, ctx),
        _ => {}
    }
    // The race HUD's lap box counts `$gp+0x18dc` down as it draws (crate::moby::classes::race::hud::draw); its sound
    // 0xd at 173 (`0x383ff0(0xd, 0, bike)`) is played here.
    if ctx.game.screen_callback == Some(crate::hud::ScreenCallback::RaceHud { moby: id }) && w.race.lap_shown == 0xad && !w.race.lap_sound {
        ctx.host.play_sound(&w.mobys[id], 0xd, 0);
        w.race.lap_sound = true;
    }
}

/// State 0: the sound handles none; `$gp+0x18d4` it; updated and drawn within 64; the route set's steps and joins
/// ([`super::patch_routes`]); its home; the racers' follow words cleared; its races won and lost counted from the
/// flag bytes; six random phases; 1.
fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.race.bike = Some(id);
    let level = ctx.game.level;
    {
        let m = &mut w.mobys[id];
        for o in [0x334, 0x748, 0x74c, 0x714, 0x3bc, 0x328, 0x330] { m.set_pv_i(o, -1); }
        m.update_dist = 0x40;
        m.draw_dist = 0x40;
        m.state = 1;
    }
    let paths = &mut w.race.paths;
    super::patch_routes(paths, ctx.level, &mut w.mobys[id]);
    w.mobys[id].mode |= 0x400;
    w.race.entered = false;
    let m = &mut w.mobys[id];
    let (p, r) = (m.pos, m.rot);
    m.set_pv_v(0x2d0, p);
    m.set_pv_v(0x2e0, r);
    m.set_pv_v(0x290, [0.0; 3]);
    m.set_pv_v(0x2a0, [0.0; 3]);
    m.set_pv_i(0x494, -1);
    for (k, f) in [0.01f32, 0.02, 0.03, 0.04].into_iter().enumerate() { m.set_pv_f(0x660 + 4 * k, f); }
    m.set_pv_i(0x314, -1);
    m.set_pv_i(0x318, 0);
    m.set_pv_i(0x498, 0);
    m.set_pv_i(0x640, 0);
    let (a, b) = if level == 4 { (0x48, 0x4d) } else { (0x41, 0x42) };
    m.set_pv_i(0x648, a);
    m.set_pv_i(0x644, b);
    m.set_pv_i(0x768, 0);
    m.set_pv_i(0x764, 0x28);
    for r in racers(w, id) {
        let rm = &mut w.mobys[r];
        if rm.pvars.len() < 0x2e4 { rm.pvars.resize(0x2e4, 0); }
        rm.set_pv_i(0x1c8, 0);
        rm.set_pv_i(0x120, -1);
        rm.set_pv_i(0x124, 0);
        rm.set_pv_i(0x1c4, -1);
    }
    let (mut won, mut lost) = (0, 0);
    for k in 0..5 {
        won += wins(ctx, k);
        lost += losses(ctx, k);
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(0x344, won);
    m.set_pv_i(0x340, lost);
    for k in 0..6 {
        let f = ctx.game.rng.randf(0.0, 1.0);
        w.mobys[id].set_pv_f(0x370 + 4 * k, f);
    }
}

/// `0x414348(m, s, step, size)` (`func_L04_00412e38`): the bob's phase on by `step`, its roll and pitch by its cosine
/// and sine × `size`.
fn bob(m: &mut Moby, step: f32, size: f32) {
    let ph = add_rot(m.pv_f(0x38c), step);
    m.set_pv_f(0x38c, ph);
    m.rot[0] += ph.cos() * size;
    m.rot[1] += ph.sin() * size;
}

/// State 1 (module docs; `0x41b020` case 1).
fn parked(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut ready = w.race.entered;
    let blocker = idx(&w.mobys[id], 0x238).filter(|&b| b < w.mobys.len());
    if w.mobys[id].pv_i(0x238) == -1 || blocker.is_none_or(|b| 0x80 <= w.mobys[b].state) {
        ready = true;
        let m = &mut w.mobys[id];
        bob(m, 0.02, 0.000_290_888_2);
        m.pos[2] = m.pv_f(0x2d8) + m.pv_f(0x38c).sin() * 0.04;
        for o in [0x238, 0x1e0] {
            if let Some(b) = idx(&w.mobys[id], o).filter(|&b| b < w.mobys.len()) {
                w.mobys[b].has_collision = false;
                w.mobys[b].mode |= 0x43;
            }
        }
    }
    let d = dist(ctx.hero.pos, w.mobys[id].pos);
    if !w.race.entered {
        let h = &*ctx.hero;
        if !ready || 5.5 <= d || h.group == 0x1f || matches!(h.state, 0x1d | 0x1c | 1) || in_scene(ctx) { return; }
    }
    ctx.game.prompt.request(4, 0x12f3, 1);
    if !w.race.entered && !w.race.again && ctx.hero.items.pressed & button::TRIANGLE == 0 { return; }
    mount(w, id, ctx);
}

/// △ at the parked bike: Ratchet on it at the start's cuboid (state 0x31), the bike there too, its sphere 0.25 / 0.1,
/// the counts of the race cleared, hidden (its pilot shown instead later), 2 (the select, or the countdown after a
/// restart).
fn mount(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let start = w.mobys[id].pv_i(0x198);
    if let Some((p, r)) = ctx.level.cuboid_place(start) {
        w.mobys[id].pos = p;
        w.mobys[id].rot = r;
    }
    let (p, r) = (w.mobys[id].pos, w.mobys[id].rot);
    ctx.hero.ride_on(id, p, r, true);
    let music = w.mobys[id].pv_i(0x488);
    ctx.host.music_request(music, -1);
    let m = &mut w.mobys[id];
    m.set_pv_i(0x58, m.pv_i(0x58) | 1);
    m.set_pv_f(0x24, 0.1);
    m.set_pv_f(0x20, 0.25);
    m.set_pv_f(0x2c, 2.0);
    m.set_pv_f(0x30, 1.0);
    m.set_pv_f(0x44, 0.008_333_334);
    m.set_pv_f(0x48, 0.5);
    m.set_pv_f(0x4c, 0.000_833_333_4);
    m.set_pv_i(0x340, 0);
    m.set_pv_f(0x28, 2.0);
    m.set_pv_i(0x54, 0);
    m.set_pv_f(0x40, 0.008_333_334);
    m.set_pv_i(0x270, 0);
    m.set_pv_i(0x31c, 0);
    m.mode |= 0x41;
    w.race.lap = 0;
    w.race.clock = 0;
    if let Some(p) = pilot(w, id) { w.mobys[p].mode |= 0x41; }
    let m = &mut w.mobys[id];
    m.state = 2;
    m.timer = if w.race.again { 2 } else { 0 };
    // `0x32e338(0, 0x40c228)` unless asked for again: the select's screen ([`draw_select`]).
    if !w.race.again { ctx.game.screen_callback = Some(crate::hud::ScreenCallback::RaceSelect { moby: id }); }
    w.race.entered = false;
    m.set_pv_i(0x394, 0x96);
    m.update_dist = 0xff;
    if ctx.game.prompt.owner == 4 { ctx.game.prompt.owner = 0; }
    ctx.hero.view_lock = 1;
    ctx.host.music_request(music, -1);
}

/// Ratchet kept on it (the view lock, his collision off: `0x18c0d5`, `+0x98`).
fn hold_hero(ctx: &mut Ctx, id: usize) {
    ctx.hero.view_lock = 1;
    ctx.hero.vehicle.moby = Some(id);
}

/// State 2 (module docs; `0x41b020` case 2).
fn racing(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.race.restart != 0 {
        let hidden = w.race.restart != 2;
        retry(w, id, ctx, hidden);
        w.race.restart = 0;
        return;
    }
    if w.race.quit {
        w.mobys[id].mode &= 0xffbe;
        stop(w, id, ctx);
        w.mobys[id].set_pv_i(0x320, -1);
        w.mobys[id].state = 4;
        w.race.quit = false;
        return;
    }
    let mut sub = w.mobys[id].timer;
    match sub {
        0 => {
            w.race.select_cam = false;
            // The select's screen sets the view's angle to the bike's (`0x40c228`: tan(`+0x28c`° / 2)).
            ctx.game.cam_tweaks.blend.tan = Some((w.mobys[id].pv_f(0x28c) * 0.008_726_647).tan());
            select(w, id, ctx);
            // `0x32e338(0, 0x40c228)` each tick of the select.
            ctx.game.screen_callback = Some(crate::hud::ScreenCallback::RaceSelect { moby: id });
            return;
        }
        1 => {
            ride(w, id, ctx);
            if ctx.hero.items.pressed & 0xff != 0 {
                ctx.host.play_level_sound(None, 4, 0);
                w.mobys[id].timer = 3;
            }
            return;
        }
        2 => {
            if let Some(p) = pilot(w, id) {
                let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
                let pm = &mut w.mobys[p];
                pm.mode &= 0xffbe;
                pm.pos = pos;
                pm.rot = rot;
                pm.update_dist = 0;
                ctx.host.set_anim(&w.mobys[p], 0, 0, 0);
                ctx.host.advance(&w.mobys[p]);
                w.rebuild(p, &*ctx.host);
            }
            w.mobys[id].mode &= 0xffbe;
            // `0x32e338(0, 0x4188b8)`: the race's HUD (crate::moby::classes::race::hud::draw).
            ctx.game.screen_callback = Some(crate::hud::ScreenCallback::RaceHud { moby: id });
            w.race.count = 0;
            w.race.start_boost = false;
            w.race.on_water = false;
            start(w, id, ctx);
            hold_hero(ctx, id);
            if w.race.again {
                w.mobys[id].timer = 1;
                w.mobys[id].state_ticks = 0x78;
                w.race.again = false;
                return;
            }
            w.mobys[id].timer = 3;
            sub = 3;
        }
        _ => {}
    }
    if sub == 3 {
        w.race.g18ec = 0;
        for r in racers(w, id) {
            w.mobys[r].set_pv_b(0xa0, 1);
            w.mobys[r].state = 2;
        }
        w.mobys[id].timer = 4;
        sub = 4;
    }
    match sub {
        4 => {
            ride(w, id, ctx);
            let c = w.mobys[id].pv_i(0x394);
            let k = w.race.count;
            if (c < 0x78 && k == 0) || (c < 0x5a && k == 1) || (c < 0x3c && k == 2) || (c < 0x1e && k == 3) {
                ctx.host.play_sound(&w.mobys[id], 0xd, 0);
                w.race.count += 1;
            }
            w.mobys[id].count_i(0x394);
            if w.mobys[id].pv_i(0x394) == 0 {
                ctx.host.play_sound(&w.mobys[id], 0xe, 0);
                w.mobys[id].timer = 5;
                w.race.start_boost = ctx.hero.items.held & button::CROSS == 0;
            }
        }
        5 => {
            ride(w, id, ctx);
            w.race.fanfare = false;
            if finish(w, id, ctx) { return; }
            let m = &w.mobys[id];
            if m.pos[2] < m.pv_f(0x26c) || off_track(ctx.game.level, m.pv_i(0x314), m.pv_i(0x318), m.pos[2]) {
                stop_sounds(w, id, ctx);
                w.mobys[id].timer = 6;
                w.mobys[id].state_ticks = 1;
            }
            if !w.race.retry { return; }
            retry(w, id, ctx, false);
            w.mobys[id].set_pv_i(0x340, 0);
            w.race.retry = false;
        }
        6 => respawn(w, id, ctx),
        _ => {}
    }
}

/// Sub-state 6: the follower let go, the draft off, the speeds none, the sounds stopped, the pilot hidden at it; at
/// the end of its ticks back on the track at the respawn cuboid `+0x270` (else the start's), pushed out of what is
/// there, reset ([`reset`], its counts kept), Ratchet and the bike there, how far along it found again, 5.
fn respawn(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if let Some(f) = ai::link(&w.mobys[id], 0x4c4).filter(|&f| f < w.mobys.len()) { w.mobys[f].set_pv_i(0x1c0, 0); }
    {
        let m = &mut w.mobys[id];
        m.set_pv_i(0x4c4, 0);
        m.set_pv_i(0x724, 0);
        m.set_pv_i(0x720, 0);
        m.set_pv_f(0x30c, 0.0);
        m.set_pv_f(0x27c, 0.0);
    }
    stop_sounds(w, id, ctx);
    if let Some(p) = pilot(w, id) {
        let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
        let scale = w.class(w.mobys[p].o_class).map_or(1.0, |c| c.scale) * 0.25;
        let pm = &mut w.mobys[p];
        pm.mode |= 0x41;
        pm.drawn = 1;
        pm.draw_dist = 0x7f;
        pm.scale = scale;
        pm.pos = pos;
        pm.rot = rot;
        pm.update_dist = 0;
        ctx.host.advance(&w.mobys[p]);
        w.rebuild(p, &*ctx.host);
        w.mobys[p].mode |= 6;
    }
    let m = &mut w.mobys[id];
    m.state_ticks -= 1;
    if m.state_ticks != 0 { return; }
    let k = m.pv_i(0x270).clamp(0, 31) as usize;
    let mut cub = m.pv_i(0x110 + 4 * k);
    if cub == -1 { cub = m.pv_i(0x198); }
    let Some((mut place, rot)) = ctx.level.cuboid_place(cub) else { return };
    let r = w.mobys[id].pv_f(0x20);
    if let Some(h) = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, place, r, crate::collision::QueryFlags(4), Some(id))) {
        place = h.pushed_centre.unwrap_or(place);
    }
    reset(w, id, ctx, true);
    ctx.hero.ride_on(id, place, rot, true);
    hold_hero(ctx, id);
    let m = &mut w.mobys[id];
    m.set_pv_i(0x4a0, -1);
    m.set_pv_i(0x49c, -1);
    m.set_pv_f(0x4a4, -1.0);
    m.set_pv_i(0x4b0, 0);
    m.set_pv_i(0x4a8, 0);
    m.set_pv_i(0x4ac, 0);
    m.pos = place;
    m.rot = rot;
    let set = RouteSet::of(m);
    let (mut link, mut point) = (m.pv_i(0x314), m.pv_i(0x318));
    let r = &mut w.race;
    let p = find_progress(&r.paths, &set, place, &mut link, &mut point, Some(&mut r.frame));
    let m = &mut w.mobys[id];
    m.set_pv_i(0x314, link);
    m.set_pv_i(0x318, point);
    m.set_pv_f(0x310, p);
    m.mode &= 0xffbe;
    if let Some(pl) = pilot(w, id) {
        let pm = &mut w.mobys[pl];
        pm.pos = place;
        pm.rot = rot;
        ctx.host.set_anim(&w.mobys[pl], 0, 0, 0);
        ctx.host.advance(&w.mobys[pl]);
        w.rebuild(pl, &*ctx.host);
        w.mobys[pl].mode &= 0xffbe;
    }
    let m = &mut w.mobys[id];
    m.timer = 5;
    m.update_dist = 0xff;
    m.mode |= 0x400;
}

/// State 3 (module docs; `0x41b020` case 3): ridden by the auto-pilot; lost: back on its route at the nearest point;
/// the results' wait, then △ (the results' box is not ported: △ goes on), stopped, 4.
fn finished(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ride(w, id, ctx);
    w.race.finished += 1;
    // The standings' sound as a racer behind the bike comes in (`0x4127a8`: `0x383ff0(0x16, 0x11, bike)`, the racer's
    // finish time the clock + the ticks since the bike's).
    let (place, due) = (w.mobys[id].pv_i(0x320), w.race.clock + w.race.finished);
    if racers(w, id).into_iter().any(|r| { let m = &w.mobys[r]; let k = m.pv_b(0xa2) as i32; k < 0xb && place < k && m.pv_i(0x1a0) == due }) {
        ctx.host.play_sound(&w.mobys[id], 0x16, 0x11);
    }
    let level = ctx.game.level;
    let m = &w.mobys[id];
    if m.pos[2] < m.pv_f(0x26c) || off_track(level, m.pv_i(0x314), m.pv_i(0x318), m.pos[2]) {
        let route = m.pv_i(0x6d0);
        w.race.paths.ensure(ctx.level, route);
        let k = nearest_index(&w.race.paths, level, &w.mobys[id], route, true);
        let r = &mut w.race;
        let paths = &r.paths;
        let m = &mut w.mobys[id];
        m.pos = paths.point(route, k);
        let t = k as f32 * paths.step(route) + if r.on_water { 15.0 } else { 5.0 };
        autopilot_at(paths, m, route, t);
        let set = RouteSet::of(m);
        let (mut link, mut point) = (m.pv_i(0x314), m.pv_i(0x318));
        let p = find_progress(paths, &set, m.pos, &mut link, &mut point, Some(&mut r.frame));
        let m = &mut w.mobys[id];
        m.set_pv_i(0x314, link);
        m.set_pv_i(0x318, point);
        m.set_pv_f(0x310, p);
    }
    // `0x32e338(0, 0x4127a8)` each tick: the standings (crate::moby::classes::race::hud::standings).
    ctx.game.screen_callback = Some(crate::hud::ScreenCallback::RaceStandings { moby: id });
    w.mobys[id].count_i(0x398);
    if 0 < w.mobys[id].pv_i(0x398) || ctx.hero.items.pressed & button::TRIANGLE == 0 { return; }
    stop(w, id, ctx);
    w.mobys[id].state = 4;
    let music = w.mobys[id].pv_i(0x488);
    ctx.host.music_request(music, -1);
}

/// The auto-pilot's place `t` along route `r` (`+0x6d8`, `+0x654`, `+0x6d4`, `+0x6a0`).
fn autopilot_at(paths: &super::Paths, m: &mut Moby, r: i32, t: f32) {
    let (i, f, p) = super::along(paths, r, t);
    m.set_pv_f(0x6d8, t);
    m.set_pv_i(0x654, i);
    m.set_pv_f(0x6d4, f);
    m.set_pv_v(0x6a0, p);
}

/// State 4 (module docs; `0x41b020` case 4): after a race back to the select (the place taken counted up while the
/// next race is won); else (the select left) Ratchet put off at `+0x19c` in state 0, his view and collision back, the
/// bike home and parked, the racers' riders gone.
fn off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].mode &= 0xffbe;
    let home = w.mobys[id].pv_i(0x19c);
    if home == -1 {
        retry(w, id, ctx, true);
        w.race.retry = false;
        return;
    }
    let race = w.race.race;
    if w.mobys[id].pv_i(0x320) != -1 && (race != 0 || wins(ctx, 0) != 1) {
        if wins(ctx, race) == 1 { set_race(w, ctx, (race + 1).min(4)); }
        retry(w, id, ctx, true);
        return;
    }
    // `0x32e338(0, 0)` after the results' box (`0x41c1ec`).
    ctx.game.screen_callback = None;
    let music = if ctx.game.level == 0xb { w.mobys[id].pv_i(0x484) } else { w.mobys[id].pv_i(0x480) };
    let Some((p, mut r)) = ctx.level.cuboid_place(home) else { return };
    if w.race.select_cam { r[2] = 0.92; }
    // The view's angle back to the play's (`0x41c340`: tan 0.5622).
    ctx.game.cam_tweaks.blend.tan = Some((crate::camera::PLAY_FOV * 0.5).tan());
    if w.mobys[id].pv_i(0x320) == 1 && race == 0 { set_race(w, ctx, 1); }
    ctx.hero.ride_off();
    ctx.hero.view_lock = 0;
    ctx.hero.teleport(p, r);
    ctx.hero.request_state(0, true);
    let class_scale = w.class(super::BIKE).map_or(1.0, |c| c.scale);
    let m = &mut w.mobys[id];
    m.pos = m.pv_v(0x2d0);
    m.rot = m.pv_v(0x2e0);
    m.state = 1;
    m.scale = class_scale;
    ctx.host.music_request(music, -1);
    for rc in racers(w, id) {
        let rm = &mut w.mobys[rc];
        rm.set_pv_i(0x2c8, 0);
        rm.set_pv_b(0x251, 0);
        if let Some(rider) = ai::link(rm, 0x8c).filter(|&x| x < w.mobys.len()) { w.delete(rider); }
        ai::set_link(&mut w.mobys[rc], 0x8c, None);
    }
}

/// `0x40be48(m, hidden)` (`func_US2_L04_0040BE48`): the race again — the sounds stopped, reset, set up ([`start`]),
/// Ratchet and the bike at the start's cuboid, the countdown 150, the laps and the clock none, the effects' mobys
/// gone (0x0abd / 0x0abe / 0x0b3d..0x0b40 / 0x027f / 0x0070 / 0x007a), the racers on the grid; `hidden`: to the select
/// with everything hidden, else to the start.
fn retry(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, hidden: bool) {
    let start_cub = w.mobys[id].pv_i(0x198);
    let (point, rot) = ctx.level.cuboid_place(start_cub).unwrap_or((w.mobys[id].pos, w.mobys[id].rot));
    stop_sounds(w, id, ctx);
    reset(w, id, ctx, false);
    start(w, id, ctx);
    ctx.hero.ride_on(id, point, rot, true);
    let m = &mut w.mobys[id];
    m.state = 2;
    m.pos = point;
    m.rot = rot;
    m.mode = (m.mode & 0xffbe) | 0x400;
    m.set_pv_i(0x394, 150);
    m.update_dist = 0xff;
    m.set_pv_i(0x270, 0);
    m.set_pv_i(0x31c, 0);
    w.race.lap = 0;
    w.race.clock = 0;
    w.race.g1918 = 0;
    for i in w.first_created.min(w.mobys.len())..w.mobys.len() {
        let c = w.mobys[i].o_class;
        if w.mobys[i].state < 0x80 && ((0x0abd..=0x0abe).contains(&c) || (0x0b3d..=0x0b40).contains(&c) || matches!(c, 0x027f | 0x0070 | 0x007a)) { w.delete(i); }
    }
    for r in racers(w, id) {
        w.mobys[r].set_pv_b(0xa0, 1);
        w.mobys[r].state = 2;
    }
    hold_hero(ctx, id);
    if hidden {
        w.mobys[id].timer = 0;
        // `0x32e338(0, 0)`.
        ctx.game.screen_callback = None;
        w.mobys[id].mode |= 0x41;
        if let Some(p) = pilot(w, id) { w.mobys[p].mode |= 0x41; }
        for r in racers(w, id) {
            w.mobys[r].mode |= 0x41;
            if let Some(c) = ai::link(&w.mobys[r], 0x8c).filter(|&c| c < w.mobys.len()) { w.mobys[c].mode |= 0x41; }
        }
    } else {
        w.mobys[id].timer = 2;
    }
}

/// `0x40c120(m, s)` (`func_L04_0040ac30`): the sounds stopped, its and its pilot's animations frozen.
fn stop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    stop_sounds(w, id, ctx);
    ctx.host.set_anim_speed(&w.mobys[id], 0.0);
    if let Some(p) = pilot(w, id) { ctx.host.set_anim_speed(&w.mobys[p], 0.0); }
}

/// `0x40f758(m, s)` (`func_L04_0040e248`): its looped sounds stopped (`+0x748`, `+0x74c`, `+0x714`, `+0x3bc`, `+0x328`,
/// `+0x330`, `+0x334`).
fn stop_sounds(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    for o in [0x748, 0x74c, 0x714, 0x3bc, 0x328, 0x330, 0x334] {
        let h = m.pv_i(o);
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        m.set_pv_i(o, -1);
    }
    m.set_pv_i(0x32c, 0);
    m.set_pv_i(0x324, 0);
}

/// `0x40b8a8(m, keep)` (`func_L04_0040a3b8`): the ride's words cleared, the bike at Ratchet, the view angle 90°, the
/// pilot in its sequence 0, the sounds stopped; `keep` false: how far along the race it is too; the crash's mobys
/// (0x0b3d..0x0b40, 0x027f, 0x0070, 0x007a) gone.
fn reset(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, keep: bool) {
    stop_sounds(w, id, ctx);
    let hero = ctx.hero.pos;
    let m = &mut w.mobys[id];
    m.set_pv_v(0x2b0, [0.0, 0.0, 1.0]);
    for o in [0x290, 0x2a0, 0x2c0, 0x6c0, 0x2f0] { m.set_pv_v(o, [0.0; 3]); }
    m.pos = hero;
    for o in [0x76c, 0x718, 0x34c, 0x354, 0x358, 0x278, 0x27c, 0x300, 0x304, 0x308, 0x30c, 0x280, 0x288, 0x750, 0x738, 0x724, 0x324, 0x32c, 0x33c, 0x338, 0x38c, 0x390, 0x4c4, 0x3b0, 0x3ac, 0x3b4, 0x360, 0x364, 0x368, 0x4d8, 0x4dc, 0x36c, 0x64c, 0x70c, 0x388] {
        m.set_pv_i(o, 0);
    }
    m.set_pv_s(0x274, 0);
    m.set_pv_b(0x349, 0);
    m.set_pv_f(0x28c, 90.0);
    m.set_pv_i(0x3a8, 10);
    m.set_pv_s(0x34a, -1);
    if !keep {
        m.set_pv_i(0x3a4, 0);
        m.set_pv_i(0x314, -1);
        m.set_pv_f(0x3b8, 0.0);
        m.set_pv_f(0x310, 0.0);
        m.set_pv_i(0x318, 0);
    }
    if let Some(p) = pilot(w, id) { ctx.host.set_anim(&w.mobys[p], 0, 0, 0); }
    for i in w.first_created.min(w.mobys.len())..w.mobys.len() {
        let c = w.mobys[i].o_class;
        if w.mobys[i].state < 0x80 && ((0x0b3d..=0x0b40).contains(&c) || matches!(c, 0x027f | 0x0070 | 0x007a)) { w.delete(i); }
    }
}

/// `0x41ace8(m)` (`func_L04_004195e0`): reset, the weapon pickups (0x0ab8, group `+0x260`) shown with the record's
/// flag 1, the turbo pickups (0x0a99, `+0x25c`) ready, the short cut's gates (`+0x23c`) shut with its flag 4, the
/// racers put on the grid ([`participants`]).
fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    reset(w, id, ctx, false);
    let rec = super::record(ctx.game.level, w.race.race);
    {
        let m = &mut w.mobys[id];
        m.set_pv_i(0x494, -1);
        m.set_pv_i(0x498, 0);
        m.set_pv_f(0x4a4, -1.0);
        m.set_pv_i(0x49c, -1);
        m.set_pv_i(0x4a0, -1);
        m.set_pv_i(0x4b0, 0);
        m.set_pv_i(0x4a8, 0);
        m.set_pv_i(0x4ac, 0);
        // The race's frame (crate::moby::classes::race::Frame): at the bike, its rows (the across one negated); the
        // HUD's speed marks.
        let rows = m.rows;
        w.race.frame = super::Frame { origin: m.pos, rows: [rows[0], scale(rows[1], -1.0), rows[2]] };
        w.race.hud.start();
    }
    let group = |w: &MobyWorld, o: usize| -> Vec<usize> {
        usize::try_from(w.mobys[id].pv_i(o)).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
    };
    for p in group(w, 0x260) {
        if w.mobys[p].o_class != 0x0ab8 { continue; }
        let solid = class_solid(w, p);
        let pm = &mut w.mobys[p];
        if rec.flags & 1 != 0 {
            pm.set_pv_i(0, 0);
            pm.mode &= 0xffbe;
            pm.has_collision = solid;
            pm.mode &= 0xfffd;
        } else {
            pm.mode |= 0x41;
            pm.has_collision = false;
            pm.mode |= 2;
        }
    }
    for p in group(w, 0x25c) {
        if w.mobys[p].o_class != 0x0a99 { continue; }
        let pm = &mut w.mobys[p];
        pm.set_pv_i(0, 0);
        pm.mode &= 0xffbe;
    }
    for k in 0..4 {
        let Some(g) = idx(&w.mobys[id], 0x23c + 4 * k).filter(|&g| g < w.mobys.len()) else { continue };
        let solid = class_solid(w, g);
        let gm = &mut w.mobys[g];
        if rec.flags & 4 != 0 {
            gm.has_collision = false;
            gm.mode |= 0x41;
        } else {
            gm.mode &= 0xffbe;
            gm.has_collision = solid;
        }
    }
    participants(w, id, ctx);
}

/// `0x40bb18(m)` (`func_L04_0040a628`, the decomp's `InitRaceParticipants`): the first racer's routes copied to the
/// bike (`+0x6e0`); each racer told the count and the bike, a skill 1..10 (at most four of each), its race time the
/// record's (the first race's longer by 120 per win) / 180, the route's length its, put on the grid
/// (crate::moby::classes::race::racers::start), 1 (the countdown makes it 2).
fn participants(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let list = racers(w, id);
    let count = list.len() as u8;
    let mut used = [0; 10];
    let rec = super::record(ctx.game.level, w.race.race);
    let route = w.mobys[id].pv_i(super::route::MAIN);
    w.race.paths.ensure(ctx.level, route);
    let duration = w.race.paths.step(route) * w.race.paths.count(route) as f32;
    for (n, &r) in list.iter().enumerate() {
        if n == 0 {
            for k in 0..6 {
                let v = w.mobys[r].pv_i(0x54 + 4 * k);
                w.mobys[id].set_pv_i(0x6e0 + 4 * k, v);
            }
            for k in 0..4 {
                let v = w.mobys[r].pv_i(0x6c + 4 * k);
                w.mobys[id].set_pv_i(0x6f8 + 4 * k, v);
            }
            let v = w.mobys[r].pv_i(0x7c);
            w.mobys[id].set_pv_i(0x708, v);
        }
        let mut j = ctx.game.rng.rand_range(0, 9);
        while 4 <= used[j as usize] { j = ctx.game.rng.rand_range(0, 9); }
        used[j as usize] += 1;
        let mut t = rec.time;
        if w.race.race == 0 { t += wins(ctx, 0) * 120; }
        let rm = &mut w.mobys[r];
        if rm.pvars.len() < 0x2e4 { rm.pvars.resize(0x2e4, 0); }
        rm.set_pv_b(0xa3, count);
        ai::set_link(rm, 0x164, Some(id));
        rm.set_pv_i(0x1a8, j + 1);
        rm.set_pv_f(0x2d0, 0.0);
        rm.set_pv_f(0x1b0, t as f32 * 0.005_555_556);
        rm.set_pv_f(0x1b4, duration);
        rm.set_pv_f(0x1d4, -1.0);
        rm.set_pv_i(0x1b8, 0);
        rm.set_pv_i(0x1c4, -1);
        rm.set_pv_i(0x1c8, 0);
        rm.set_pv_i(0x1cc, -1);
        rm.set_pv_i(0x1d0, -1);
        rm.set_pv_i(0x1e0, 0);
        rm.set_pv_i(0x1d8, 0);
        rm.set_pv_i(0x1dc, 0);
        super::racers::start(w, r, ctx);
        w.mobys[r].state = 1;
    }
    w.race.route_time = duration;
}

/// `0x40cf60(m, s)` (`func_US2_L04_0040CF60`): the race select. The first tick: reset, the pick the race last chosen.
/// Ratchet held in 0x31, everything hidden, the sounds stopped. ←/→ change the pick (the level sound 3, wrapping in
/// 0..4); Start / ✕ choose it when it is open (the record's 0x10000000, or the race before it won: the sound 4, the
/// sub-state 2) — else the sound 5; △ leaves (the sound 4, stopped, `+0x320` −1, state 4).
fn select(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let level = ctx.game.level;
    let mut accept = true;
    if !w.race.select_open {
        accept = false;
        reset(w, id, ctx, false);
        w.race.pick = w.race.race;
        w.race.select_open = true;
    }
    if ctx.hero.state != 0x31 { ctx.hero.request_state(0x31, true); }
    w.mobys[id].mode |= 0x41;
    if let Some(p) = pilot(w, id) { w.mobys[p].mode |= 0x41; }
    for r in racers(w, id) {
        w.mobys[r].mode |= 0x41;
        if let Some(c) = ai::link(&w.mobys[r], 0x8c).filter(|&c| c < w.mobys.len()) { w.mobys[c].mode |= 0x41; }
    }
    stop_sounds(w, id, ctx);
    hold_hero(ctx, id);
    let pressed = ctx.hero.items.pressed;
    if pressed & button::LEFT != 0 {
        ctx.host.play_level_sound(None, 3, 0);
        w.race.pick -= 1;
        if w.race.pick < 0 { w.race.pick = 4; }
    } else if pressed & button::RIGHT != 0 {
        ctx.host.play_level_sound(None, 3, 0);
        w.race.pick += 1;
        if 5 <= w.race.pick { w.race.pick = 0; }
    }
    let mut done = false;
    if accept && pressed & (button::START | button::CROSS) != 0 {
        let k = w.race.pick;
        if super::record(level, k).flags & 0x1000_0000 != 0 || (0 < k && 0 < wins(ctx, k - 1)) {
            ctx.host.play_level_sound(None, 4, 0);
            done = true;
            set_race(w, ctx, k);
            w.mobys[id].timer = 2;
        } else {
            ctx.host.play_level_sound(None, 5, 0);
        }
    } else if pressed & button::TRIANGLE != 0 {
        ctx.host.play_level_sound(None, 4, 0);
        done = true;
        stop(w, id, ctx);
        w.mobys[id].set_pv_i(0x320, -1);
        w.mobys[id].state = 4;
    }
    if done { w.race.select_open = false; }
}

/// `0x41a160(m, s)` (`func_L04_00418a58`): a tick ridden — the start boost (✕ held in the race's first 12 ticks:
/// 240 ticks of boost, the sound 0xf), Ratchet held, its scale a quarter of its class's, the move ([`drive`]), the
/// boosts ([`boosts`]), the pilot ([`pilot_pose`]); finished: the respawn cuboid and the racers' progress.
fn ride(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.race.start_boost && w.race.clock < 12 && ctx.hero.items.held & button::CROSS != 0 {
        let m = &mut w.mobys[id];
        m.set_pv_i(0x650, 240);
        m.set_pv_i(0x278, 240);
        let h = m.pv_i(0x714);
        if h == -1 || !ctx.host.sound_playing(h) {
            let h = ctx.host.play_sound(&w.mobys[id], 0xf, 4);
            w.mobys[id].set_pv_i(0x714, h);
        }
        w.race.start_boost = false;
    }
    hold_hero(ctx, id);
    let class_scale = w.class(super::BIKE).map_or(1.0, |c| c.scale);
    w.mobys[id].scale = class_scale * 0.25;
    drive(w, id, ctx);
    pilot_pose(w, id, ctx);
    weapons(w, id, ctx);
    // `0x417528`, the trails: not ported.
    engine_sounds(w, id, ctx);
    if w.mobys[id].state == 3 {
        respawn_pick(w, id, ctx);
        let set = RouteSet::of(&w.mobys[id]);
        for r in racers(w, id) {
            let rm = &w.mobys[r];
            let (mut link, mut point) = (rm.pv_i(0x120), rm.pv_i(0x124));
            let wrap = rm.pv_i(0x160) != 0;
            let p = track_progress(&w.race.paths, &set, rm.pos, &mut link, &mut point, wrap, None);
            let rm = &mut w.mobys[r];
            rm.set_pv_i(0x120, link);
            rm.set_pv_i(0x124, point);
            rm.set_pv_f(0x11c, p);
        }
    }
}

/// L04 `0x1b09c0` (L11 its copy): the weapons' counts by the choice 1..4 (the shots' bursts, the missiles, the mines,
/// the shield's ticks).
const CHOICES: [i32; 5] = [0, 100, 3, 1, 600];

/// `0x410460(m)` (`func_L04_0040EF50`): the race's weapons.
/// * The ram: the mobys within 0.7 of Ratchet's head (`0x189f00`, taken here as 1 over his place) 3 ticks of its
///   velocity on (`0x2e14b0`'s gather, flags 0, his moby spared), the crates and the class 0x85 among them, take a hit
///   (`0x31d4c8`: 1, flags 0x10000, its way 7 × the velocity, from his place).
/// * The boosts ([`boosts`]).
/// * The weapon pickups 0x0ab8 (group `+0x260`, with the record's flag 1) within 1 and past their rest (their `+0`):
///   one with no weapon of its own (its `+4` −1) deals a new one at random of 1..4 (not the one held), any other the
///   one held again; 4 is the shield (`+0x4d8` for [`CHOICES`]`[4]` ticks: no crash, `+0x4dc`), else `+0x360` the
///   weapon and `+0x364` its count, `+0x368` none; the pickup's sound (not finished), its rest 300. The pickup's
///   spray (`0x40d5d0`) is not ported.
/// * The shield's ticks; its end readies the cooldown (`+0x768`, `+0x764` 40).
/// * Not finished, the weapon held fires: shots (1) held L1 / L2 every 3 ticks, missiles (2) and mines (3) pressed
///   every 30 ticks, each its count down (out: none). The shots play the sound 7, the missiles 8.
///
/// Not ported: the projectiles — shots `0x3f9320` (class 0x0abc, with their trail and glow particles), missiles
/// `0x3fba68` (0x0abd, aimed by `0x410190` at the racers and mines ahead) and mines `0x3fd240` (0x0abe, the sound 9):
/// none of the three classes is in Barlow's or Joba's class table (the census), so what they leave on the original
/// is to be seen in PCSX2; the gun's joint (`0x2a0af8`, alternating), the missile's voice `0x2d6cc8(47)`.
fn weapons(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let vel = w.mobys[id].pv_v(0x290);
    let head = add(ctx.hero.pos, [0.0, 0.0, 1.0]);
    let probe = add(scale(vel, 3.0), head);
    let near = ctx.scene.map(|sc| crate::collision::mobys::coll_sphere_mobys(sc, probe, 0.7, crate::collision::QueryFlags(0), ctx.hero_moby)).unwrap_or_default();
    let dir = scale(vel, 7.0);
    for k in near {
        if !(crate::moby::aim::is_crate(w, k) || w.mobys.get(k).is_some_and(|m| m.o_class == 0x85)) { continue; }
        let t = crate::moby::hits::HitTemplate::new(1.0, ctx.hero_moby, 0x10000, [dir[0], dir[1], dir[2], 0.0]);
        let p = ctx.hero.pos;
        w.deliver_hit_any(k, &t, [p[0], p[1], p[2], 1.0]);
    }
    boosts(w, id, ctx);
    let finished = w.mobys[id].state == 3;
    let group = usize::try_from(w.mobys[id].pv_i(0x260)).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
    if super::record(ctx.game.level, w.race.race).flags & 1 != 0 && w.mobys[id].pv_i(0x260) != -1 {
        for p in group {
            if w.mobys[p].o_class != 0x0ab8 || w.mobys[p].pvars.len() < 8 { continue; }
            if w.mobys[p].pv_i(0) != 0 || 1.0 <= len(sub(w.mobys[p].pos, w.mobys[id].pos)) { continue; }
            let held = w.mobys[id].pv_i(0x360);
            let mut choice = held;
            if w.mobys[p].pv_i(4) == -1 {
                loop {
                    choice = ctx.game.rng.rand_range(1, 4);
                    if held != choice { break; }
                }
            }
            let m = &mut w.mobys[id];
            if choice == 4 {
                m.set_pv_i(0x4d8, 1);
                m.set_pv_i(0x4dc, CHOICES[4]);
            } else {
                m.set_pv_i(0x360, choice);
                m.set_pv_i(0x364, CHOICES[choice.clamp(0, 4) as usize]);
                m.set_pv_i(0x368, 0);
            }
            if !finished { ctx.host.play_sound_of(&w.mobys[id], 0x0ab8, 0, 0); }
            w.mobys[p].set_pv_i(0, 300);
        }
    }
    let m = &mut w.mobys[id];
    if m.pv_i(0x4d8) != 0 {
        m.set_pv_i(0x768, 0);
        if m.count_i(0x4dc) {
            m.set_pv_i(0x4d8, 0);
            m.set_pv_i(0x768, 1);
        }
    }
    if m.pv_i(0x768) != 0 && m.count_i(0x764) {
        m.set_pv_i(0x768, 0);
        m.set_pv_i(0x764, 40);
    }
    if finished { return; }
    let (held, pressed) = (ctx.hero.items.held, ctx.hero.items.pressed);
    let fire = button::L1 | button::L2;
    let (rest, sound, input) = match m.pv_i(0x360) {
        1 => (3, Some(7), held),
        2 => (30, Some(8), pressed),
        3 => (30, None, pressed),
        _ => return,
    };
    if !m.count_i(0x368) || input & fire == 0 { return; }
    m.set_pv_i(0x368, rest);
    if let Some(k) = sound { ctx.host.play_sound(&w.mobys[id], k, 0); }
    let m = &mut w.mobys[id];
    if m.count_i(0x364) { m.set_pv_i(0x360, 0); }
}

/// `0x40f4a0(m, s)` (`func_L04_0040DF90`): the engine's two loops — `+0x330` the sound 0 (finished 20) and `+0x334` 1
/// (21), started again whenever not playing (mode 5); by `e` = the speed + the boost − 0.05 (at least 0), the first's
/// volume `1024·(1 − min(3e, 1))` and bend `2816·min(3e, 1)`, the second's volume `1024·(0.2 + 0.94·e/0.45)` (past
/// 0.85 of 0.45: `0.2 + 0.8·(1.85 − e/0.45)`) and bend `−2816·(1 − e/0.45)`. A third sound `+0x328` stops when the
/// ticks `+0x324` of its `+0x32c` run out.
fn engine_sounds(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let finished = w.mobys[id].state == 3;
    let m = &mut w.mobys[id];
    if m.pv_i(0x32c) != 0 && m.count_i(0x324) {
        let h = m.pv_i(0x328);
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        m.set_pv_i(0x32c, 0);
        m.set_pv_i(0x328, -1);
        m.set_pv_i(0x324, 0);
    }
    let e = (m.pv_f(0x30c) + m.pv_f(0x27c) - 0.050_000_004).max(0.0);
    for (o, k) in [(0x330, if finished { 20 } else { 0 }), (0x334, if finished { 21 } else { 1 })] {
        let h = w.mobys[id].pv_i(o);
        if h == -1 || !ctx.host.sound_playing(h) {
            let h = ctx.host.play_sound(&w.mobys[id], k, 5);
            w.mobys[id].set_pv_i(o, h);
        }
    }
    let m = &w.mobys[id];
    let (a, b) = (m.pv_i(0x330), m.pv_i(0x334));
    if a != -1 && ctx.host.sound_playing(a) {
        let t = (e * 3.0).min(1.0);
        ctx.host.set_sound_volume(a, (1024.0 * (1.0 - t)) as i32);
        ctx.host.set_sound_bend(a, (t * 2816.0) as i32);
    }
    if b != -1 && ctx.host.sound_playing(b) {
        let o = e / 0.45;
        let t = if o < 0.85 { o * 0.941_176_4 + 0.2 } else { (1.0 - (o - 0.85)) * 0.8 + 0.2 };
        ctx.host.set_sound_volume(b, (1024.0 * t) as i32);
        ctx.host.set_sound_bend(b, ((1.0 - o) * -2816.0) as i32);
    }
}

/// `0x412420(m)` (`func_L04_00410f10`): the respawn cuboid `+0x270` the first of `+0x110[32]` it is in.
fn respawn_pick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    for i in 0..32 {
        if ctx.level.in_cuboid(m.pv_i(0x110 + 4 * i), m.pos) {
            m.set_pv_i(0x270, i as i32);
            return;
        }
    }
}

/// `0x40f970(m)` (`func_L04_0040e460`): the boosts. The jet boost (`+0x278` ticks): eased up to 1/6 (1/180 a tick);
/// else R1 / R2 (or finished) with a turbo charge (`+0x33c`) and none running: 240 ticks of it (the charge spent; the
/// sound 0xf, 0x18 finished; the follower let go when slow); while its ticks run the boost eases to 0.1 (1/720), else
/// to none. The turbo pickups 0x0a99 (group `+0x25c`) within 1: a charge (at most 3; the pickup's rest 300). The jets
/// of its group (`+0x254`, glowing in the colour of the pulse: 0x0474 1.25 wide, 0x0cef 3.75, 2.8 × 1.5) under
/// Ratchet and past their rest (`+0xbc`) add their ticks (the jet's pvar `+0`, the sound 0xc; their rest 60).
fn boosts(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let finished = w.mobys[id].state == 3;
    let period = 50.0;
    let wave = (((w.race.clock % 50) as f32 / period) * 2.0 * PI - PI).sin();
    let red = (wave * 48.0) as i32 + 104;
    let green = ((wave * 64.0) as i32 + 208).min(255);
    let blue = (wave * 24.0) as i32 + 60;
    let colour = (128u32 << 24) | ((blue as u32 & 0xff) << 16) | ((green as u32 & 0xff) << 8) | (red as u32 & 0xff);
    let follower = ai::link(&w.mobys[id], 0x4c4).filter(|&f| f < w.mobys.len());
    {
        let m = &mut w.mobys[id];
        let (mut b, mut fov) = (m.pv_f(0x27c), m.pv_f(0x28c));
        if m.pv_i(0x278) != 0 {
            approach(0.166_666_69, 0.005_555_555_7, &mut b);
            approach(97.0, 0.566_666_7, &mut fov);
            m.count_i(0x278);
        } else {
            if (finished || ctx.hero.items.held & 10 != 0) && m.pv_i(0x338) < 30 && 0 < m.pv_i(0x33c) {
                m.set_pv_i(0x338, 240);
                m.set_pv_i(0x33c, m.pv_i(0x33c) - 1);
                let h = m.pv_i(0x714);
                if h == -1 || !ctx.host.sound_playing(h) {
                    let h = ctx.host.play_sound(&w.mobys[id], if finished { 0x18 } else { 0xf }, if finished { 5 } else { 4 });
                    w.mobys[id].set_pv_i(0x714, h);
                }
                if let Some(f) = follower {
                    if w.mobys[id].pv_f(0x27c) <= 0.016_666_668 { w.mobys[f].set_pv_i(0x1c0, 0); }
                }
            }
            let m = &mut w.mobys[id];
            let active = if m.count_i(0x338) { m.set_pv_i(0x640, 0); 0.0 } else { 1.0 };
            approach(active * 0.100_000_01, 0.001_388_888_9, &mut b);
            b = b.max(0.0);
            if 0.5 < active { approach(97.0, 0.566_666_7, &mut fov); } else { approach(80.0, 0.141_666_68, &mut fov); }
        }
        let m = &mut w.mobys[id];
        m.set_pv_f(0x27c, b);
        m.set_pv_f(0x28c, fov);
        // The view's angle (`0x1bdc70` = tan(`+0x28c`° / 2), `0x305e48`) while not finished.
        if !finished { ctx.game.cam_tweaks.blend.tan = Some((fov * 0.008_726_647).tan()); }
    }
    let group = |o: usize| -> Vec<usize> { usize::try_from(w.mobys[id].pv_i(o)).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default() };
    let (pickups, jets) = (group(0x25c), group(0x254));
    let pos = w.mobys[id].pos;
    for p in pickups {
        if w.mobys[p].o_class != 0x0a99 { continue; }
        let d = sub(w.mobys[p].pos, pos);
        if w.mobys[p].pv_i(0) == 0 && w.mobys[id].pv_i(0x33c) < 3 && dot(d, d) < 1.0 {
            let n = w.mobys[id].pv_i(0x33c) + 1;
            w.mobys[id].set_pv_i(0x33c, n);
            if !finished { ctx.host.play_sound_of(&w.mobys[id], 0x0a99, 0, 0); }
            w.mobys[p].set_pv_i(0, 300);
        }
    }
    let hero = ctx.hero.pos;
    for j in jets {
        let half = match w.mobys[j].o_class { super::JET => 1.25, super::JET_B => 3.75, _ => continue };
        let jm = &mut w.mobys[j];
        jm.mode |= 16;
        jm.glow = colour;
        let mut t = jm.timer as i32;
        let out = countdown(&mut t);
        jm.timer = t as u8;
        if !out { continue; }
        let d = sub(jm.pos, hero);
        if 20.0 < dot(d, d) { continue; }
        let local = [dot(d, jm.rows[0]), dot(d, jm.rows[1]), dot(d, jm.rows[2])];
        if !(local[0].abs() < 2.8) || !(local[1].abs() < half) || !(local[2].abs() < 1.5) { continue; }
        let ticks = jm.pv_i(0);
        jm.timer = 60;
        let m = &mut w.mobys[id];
        m.set_pv_i(0x278, m.pv_i(0x278) + ticks);
        let h = m.pv_i(0x714);
        if h == -1 || !ctx.host.sound_playing(h) {
            let h = ctx.host.play_sound(&w.mobys[id], if finished { 0x18 } else { 0xf }, if finished { 5 } else { 4 });
            w.mobys[id].set_pv_i(0x714, h);
        }
        if !finished { ctx.host.play_sound(&w.mobys[id], 0xc, 0); }
        w.mobys[id].set_pv_i(0x650, ticks);
        if let Some(f) = follower {
            if w.mobys[id].pv_f(0x27c) <= 0.016_666_668 { w.mobys[f].set_pv_i(0x1c0, 0); }
        }
    }
}

/// `0x414168(height, base, factor, speed, margin)` (`func_L04_00412c58`): its fall — a bounce's lift (`+0x34c` ≥ 8),
/// else over `base + margin` down by `speed`/3600 × how far over (more on Barlow's last jumps, by the boost: 3.5..10.5
/// from the second lap, else 1.5..4.5), under it up by `factor`·`speed`/3600 × how far under.
fn fall(m: &mut Moby, lap: i32, level: i32, height: f32, base: f32, factor: f32, speed: f32, margin: f32) {
    let prev = m.pv_f(0x2a8);
    let z = if 8 <= m.pv_i(0x34c) {
        prev + speed * 0.000_277_777_8
    } else if margin + base < height {
        let extra = if level == 4 && m.pv_s(0x274) != 0 && m.pv_i(0x314) == -1 && 476 <= m.pv_i(0x318) {
            let t = (m.pv_f(0x27c) * 9.999_999).clamp(0.0, 1.0);
            if 2 <= lap { t * 7.0 + 3.5 } else { t * 3.0 + 1.5 }
        } else {
            0.0
        };
        let t = ((height - base + margin) / (base + margin)).clamp(0.0, 1.0);
        prev - (speed + extra) * t * 0.000_277_777_8
    } else {
        let t = ((base - height + margin) / base).clamp(0.0, 1.0) * factor;
        prev + speed * t * 0.000_277_777_8
    };
    m.set_pv_f(0x298, z);
}

/// The camera's yaw (`0x1b96d8`): its forward row's.
fn cam_yaw(ctx: &Ctx) -> f32 { let f = ctx.host.camera_rows()[0]; atan2(f[0], f[1]) }

/// `0x415ba8(m, s)`: the move. Its ground (`0x31ce20`) and height; in the race (`+0xbc` 5) its yaw's aim: finished
/// toward the auto-pilot's point (the ticks stuck turning it more), else the stick eased by 0.12 (`0x2a9db0`) through
/// the camera's yaw. On plain ground over 0.19 its speed eases to 0.19 (no throttle, the jet boost's ticks gone); a
/// bounce (`+0x348`) eases it to 0.05 without the turbo `+0x4d8`. Water (surface 7) `$gp+0x18b0`. The throttle: ✕
/// (always finished) springs the speed toward 1/3 (0.022, 0.5, 1/36; half on water), else it eases to 0.05. The
/// turn: its rate 0.0582 slow down to 0.0218 fast (0.0166 with the turbo running), × 2.75 / 1.25 on water, × 4 / its
/// follower's pull when followed, by the stick's tilt; the yaw sprung (`factor`·0.016, 0.3). The velocity: along its
/// yaw at the speed and boost, sliding out by the yaw's speed (−1.75 ×); on water eased in by a fortieth (its push
/// taken off once). A push (`+0x64c`) adds `+0x2f0` × its ticks × 0.025. Its fall ([`fall`]: 0.2, 0.23, 12, 0.1); in
/// the air over 0.44; a landing after 13 ticks falling on a surface other than plain ground bounces it (as the
/// racers'). Its pitch: the bounce's lift, else tilted to the ground (0.03, 0.3) or nose down in the air; the bob
/// (0.15, 0.004). The move ([`slide`], the bike's mode: its sphere `+0x20` / `+0x24`, its speed + boost, its last
/// move, 1/6): a crash (not with the turbo, not finished) — the sounds stopped, a blast (0.666; 0x19, 0x14, 0x14;
/// 0x14 puffs, 0x2d streaks; shaking), the sound 3, the sub-state 6 for 100 ticks, hidden; a wall: the pilot flinches
/// 3 / 4 / 5 by the side (its rest 30), a mine cuts the speed to a quarter. Its last move `+0x280` (under 0.01 the
/// ticks stuck count). Ratchet at it.
fn drive(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let level = ctx.game.level;
    let lap = w.race.lap;
    let start_pos = w.mobys[id].pos;
    let (gz, surface, gn) = super::ground(ctx, start_pos);
    let height = start_pos[2] - gz;
    let finished = w.mobys[id].state == 3;
    if finished { autopilot(w, id, ctx); }
    let racing = w.mobys[id].timer == 5;
    let mut target = ctx.hero.target_yaw;
    let fv17;
    {
        let stick = ctx.hero.stick;
        let held = ctx.hero.items.held;
        let m = &mut w.mobys[id];
        if racing {
            if finished {
                let to = m.pv_v(0x6a0);
                target = atan2(to[0] - m.pos[0], to[1] - m.pos[1]);
                if 10 < m.pv_i(0x750) { target = add_rot(target, m.pv_i(0x750) as f32 * 0.01); }
            } else {
                let (mut sx, mut sy) = (m.pv_f(0x2c0), m.pv_f(0x2c4));
                approach(-stick[0], 0.12, &mut sx);
                approach(stick[1], 0.12, &mut sy);
                m.set_pv_f(0x2c0, sx);
                m.set_pv_f(0x2c4, sy);
                target = add_rot(cam_yaw(ctx), atan2(-sy, sx));
            }
            let mut throttle = true;
            if m.pv_b(0x348) == 0 || m.pv_i(0x4d8) != 0 {
                if surface == 0 && m.pv_s(0x274) == 0 {
                    let mut s = m.pv_f(0x30c);
                    if 0.191_666_68 < s {
                        throttle = false;
                        approach(0.191_666_68, 0.003_194_444_6, &mut s);
                        m.set_pv_f(0x30c, s);
                    }
                    m.set_pv_i(0x278, 0);
                }
            } else {
                throttle = false;
                let mut s = m.pv_f(0x30c);
                approach(0.050_000_004, 0.003_055_555_7, &mut s);
                m.set_pv_f(0x30c, s);
            }
            w.race.on_water = surface == 7;
            let water = w.race.on_water;
            let m = &mut w.mobys[id];
            if throttle {
                let mut s = m.pv_f(0x30c);
                if held & button::CROSS == 0 && !finished {
                    approach(0.050_000_004, 0.002_777_777_8, &mut s);
                } else {
                    let mut v = m.pv_f(0x288);
                    spring(0.333_333_34, if water { 0.011 } else { 0.022 }, 0.5, if water { 0.013_888_889 } else { 0.027_777_778 }, &mut s, &mut v);
                    m.set_pv_f(0x288, v);
                }
                m.set_pv_f(0x30c, s);
            }
            fv17 = m.pv_f(0x30c) + m.pv_f(0x27c);
        } else {
            fv17 = 0.0;
        }
    }
    let water = w.race.on_water;
    let stick_mag = ctx.hero.stick_mag;
    let m = &mut w.mobys[id];
    let speed = m.pv_f(0x30c);
    let t = (speed * 3.0).min(1.0);
    let fast = if m.pv_i(0x338) != 0 { 0.016_581_3 } else { 0.021_816_6 };
    let mut limit = 0.058_177_64 + (fast - 0.058_177_64) * t;
    let mut factor = ((1.0 - t) + 1.0).min(1.9);
    if water {
        factor = 2.75;
        limit *= 1.25;
    }
    if m.pv_i(0x4c4) != 0 {
        factor = 4.0;
        limit *= m.pv_f(0x63c);
    }
    if racing {
        if !finished { limit *= stick_mag; }
        limit = limit.max(2.908_882_6e-5);
        let (mut yaw, mut v) = (m.rot[2], m.pv_f(0x308));
        turn_spring(target, factor * 0.016, 0.3, limit, &mut yaw, &mut v, 0);
        m.rot[2] = yaw;
        m.set_pv_f(0x308, v);
    }
    let yaw = m.rot[2];
    let mut vel = m.pv_v(0x290);
    if !water {
        vel = super::cap(crate::moby::classes::shot::polar(fv17, yaw, 0.0), fv17);
        let side = set_len(m.rows[1], m.pv_f(0x30c) * m.pv_f(0x308) * -1.75);
        vel = add(vel, side);
    } else {
        if m.pv_i(0x64c) != 0 && m.pv_i(0x71c) != 0 {
            vel = sub(vel, scale(m.pv_v(0x2f0), m.pv_i(0x64c) as f32 * 0.025));
            m.set_pv_i(0x71c, 0);
        }
        let d = scale(crate::moby::classes::shot::polar(fv17, yaw, 0.0), 0.025);
        vel = super::cap(add(vel, d), fv17);
    }
    if !m.count_i(0x64c) {
        vel = add(vel, scale(m.pv_v(0x2f0), m.pv_i(0x64c) as f32 * 0.025));
        m.set_pv_i(0x71c, 1);
    }
    m.set_pv_v(0x290, vel);
    fall(m, lap, level, height, 0.2, 0.23, 12.0, 0.1);
    if 0.44 < height {
        m.set_pv_s(0x274, 1);
        m.set_pv_s(0x276, m.pv_s(0x276).wrapping_add(1));
    } else {
        if m.pv_s(0x274) != 0 && 13 <= m.pv_s(0x276) && m.pv_f(0x358) < 0.0 && surface != 0 && m.pv_b(0x349) == 0 && surface != 5 {
            let sink = m.pv_f(0x358).min(-0.05);
            m.set_pv_f(0x358, sink);
            let t = (1.0 - (sink + 0.35) * 2.857_143).clamp(0.0, 1.0);
            m.set_pv_i(0x34c, (t * 5.0) as i32 + 15);
            m.set_pv_b(0x349, 1);
            m.set_pv_f(0x354, t.max(0.5) * 0.06);
            m.set_pv_f(0x2a8, sink * -0.3);
        }
        m.set_pv_s(0x274, 0);
        m.set_pv_s(0x276, 0);
    }
    let roll = m.rot[0];
    if m.pv_b(0x349) == 0 {
        let n = if m.pv_s(0x274) == 0 { gn } else { crate::moby::classes::shot::polar(1.0, add_rot(m.rot[2], PI), 0.994_840_6) };
        m.set_pv_v(0x2b0, n);
        let (rows, mut rot, mut v) = (m.rows, m.rot, m.pv_f(0x304));
        super::tilt(&rows, &mut rot, &mut v, n, 0.03, 0.3, 0.0);
        m.rot = rot;
        m.set_pv_f(0x304, v);
    } else {
        m.rot[1] += m.pv_f(0x354);
        m.count_i(0x34c);
        if m.pv_i(0x34c) == 0 { m.set_pv_b(0x349, 0); }
    }
    m.rot[0] = roll;
    bob(m, 0.15, 0.004);
    let v = m.pv_v(0x290);
    m.set_pv_v(0x2a0, v);
    m.set_pv_f(0x358, v[2]);
    let (fr, sl, hg, rd) = (m.pv_f(0x30c) + m.pv_f(0x27c), m.pv_f(0x280), m.pv_f(0x24), m.pv_f(0x20));
    let (mut pos, mut vel, mut prev) = (m.pos, m.pv_v(0x290), m.pv_v(0x2a0));
    let (result, s) = slide(w, ctx, id, true, &mut pos, &mut vel, &mut prev, fr, sl, 0.166_666_67, hg, rd);
    let m = &mut w.mobys[id];
    m.pos = pos;
    m.set_pv_v(0x2a0, prev);
    if let Some(n) = s.normal { m.set_pv_v(0x2b0, n); }
    if result == 0 {
        m.set_pv_i(0x750, 0);
    } else {
        if water { vel = scale(vel, 0.85); }
        if s.wall { m.set_pv_i(0x70c, m.pv_i(0x70c) + 1); } else { m.set_pv_i(0x70c, 0); }
        m.set_pv_v(0x290, vel);
        if result == 2 {
            if m.pv_i(0x4d8) == 0 && !finished {
                stop_sounds(w, id, ctx);
                let at = w.mobys[id].pos;
                crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { size: 0.666, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true });
                ctx.host.play_sound(&w.mobys[id], 3, 0);
                let m = &mut w.mobys[id];
                m.timer = 6;
                m.mode = (m.mode & 0xfbff) | 0x41;
                m.state_ticks = 100;
            }
        } else if result == 1 {
            if let Some(p) = pilot(w, id) {
                let m = &w.mobys[id];
                let h = s.hit.unwrap_or(m.pos);
                let a = sub_rot(m.rot[2], atan2(h[0] - m.pos[0], h[1] - m.pos[1]));
                let anim = if 0.523_598_85 < a && a < 2.617_994 { 3 } else if a < -0.523_598_85 && -2.617_994 < a { 4 } else { 5 };
                let m = &mut w.mobys[id];
                if m.pv_i(0x36c) == 0 || m.pv_s(0x34a) != anim {
                    m.set_pv_i(0x36c, 0x1e);
                    m.set_pv_s(0x34a, anim);
                    ctx.host.set_anim(&w.mobys[p], anim as u8, 3, 3);
                } else {
                    m.set_pv_i(0x36c, 0x1e);
                }
            }
            let m = &mut w.mobys[id];
            if s.flag == 3 && m.pv_i(0x4d8) == 0 { m.set_pv_f(0x30c, m.pv_f(0x30c) * 0.25); }
        }
    }
    let m = &mut w.mobys[id];
    if result == 0 { m.set_pv_v(0x290, vel); }
    let mv = len([m.pos[0] - start_pos[0], m.pos[1] - start_pos[1], 0.0]);
    m.set_pv_f(0x280, mv);
    if mv < 0.01 { m.set_pv_i(0x750, m.pv_i(0x750) + 1); }
    let (p, r) = (m.pos, m.rot);
    ctx.hero.ride_place(p, r);
}

/// `0x414ac8(m, s)` (`func_L04_004135b8`): the auto-pilot after the finish — how far along the race it is, its place
/// along its route `+0x6d0` 5 (20 on water) on past the point when it is that near (from the nearest point when the
/// point is more than 60° off its way), the markers it passed setting a lateral offset; the route's branches
/// (`0x4143d0`) are not ported.
fn autopilot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let level = ctx.game.level;
    let set = RouteSet::of(&w.mobys[id]);
    let wrap = w.race.lap != 0;
    let water = w.race.on_water;
    let m = &w.mobys[id];
    let (mut link, mut point) = (m.pv_i(0x314), m.pv_i(0x318));
    let r = &mut w.race;
    let p = track_progress(&r.paths, &set, m.pos, &mut link, &mut point, wrap, Some(&mut r.frame));
    let route = m.pv_i(0x6d0);
    w.race.paths.ensure(ctx.level, route);
    let paths = &w.race.paths;
    let m = &mut w.mobys[id];
    m.set_pv_i(0x314, link);
    m.set_pv_i(0x318, point);
    m.set_pv_f(0x310, p);
    let old = m.pv_i(0x654);
    let near = if water { 20.0 } else { 5.0 };
    let total = paths.step(route) * paths.count(route) as f32;
    let mut t = m.pv_f(0x6d8);
    let d = super::xy_dist(m.pv_v(0x6a0), m.pos);
    if d < near {
        t += near - d;
        if total < t { t -= total; }
    }
    autopilot_at(paths, m, route, t);
    let to = m.pv_v(0x6a0);
    if 1.047_197_7 < crate::hero::physics::rot_dist(m.rot[2], atan2(to[0] - m.pos[0], to[1] - m.pos[1])) {
        let t = nearest_index(paths, level, m, route, false) as f32 * paths.step(route) + near;
        autopilot_at(paths, m, route, t);
    }
    for i in old..=m.pv_i(0x654) {
        let f = if i == 0 { paths.step(route) } else { paths.w(route, i) };
        if 1000.0 <= f {
            let o = (m.pv_f(0x658) + ctx.game.rng.randf(-0.25, 0.25)).clamp(-1.5, 1.5);
            m.set_pv_f(0x658, o);
            let t = ctx.game.rng.randf(3.0, 6.0);
            m.set_pv_i(0x65c, (t * 60.0) as i32);
        } else if f <= -1000.0 {
            m.set_pv_f(0x658, 0.0);
            m.set_pv_i(0x65c, -1000);
        }
    }
    if 0 < m.pv_i(0x65c) {
        let t = m.pv_i(0x65c) - 1;
        m.set_pv_i(0x65c, t);
        if t == -1 {
            let o = (m.pv_f(0x658) + ctx.game.rng.randf(-0.25, 0.25)).clamp(-1.5, 1.5);
            m.set_pv_f(0x658, o);
            let t = ctx.game.rng.randf(3.0, 6.0);
            m.set_pv_i(0x65c, (t * 60.0) as i32);
        }
    }
}

/// `0x417cc0(m, s)` (`func_L04_004167b0`, the pilot): the bike's lean sprung as the racers' (`+0x300`; a swerve
/// `+0x4c8` × 5 until reached; none slow unless finished); the pilot at the bike (a quarter of its class's scale,
/// alpha 127, its sequences: from the flinches 2..5 back to 0, 1 in the air, landing 2 after a long fall else 0;
/// its sequences 0 / 1 at the speed × 6, × 3 more past 1/3).
fn pilot_pose(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].timer == 0 { return; }
    let finished = w.mobys[id].state == 3;
    let stick_mag = ctx.hero.stick_mag;
    let m = &mut w.mobys[id];
    let a = (m.pv_f(0x30c) + m.pv_f(0x27c)).max(0.083_333_336);
    let mut b = m.pv_f(0x308) * 13.750_985;
    let c = (a * 3.0).min(1.0);
    let mut d = m.pv_f(0x308) * 22.918_308;
    b *= b.abs();
    d = 1.0 - (1.0 - d) * (1.0 - d);
    let (k, target) = if m.pv_f(0x4c8) != 0.0 {
        if (m.rot[0] - m.pv_f(0x4c8)).abs() < 0.003 { m.set_pv_f(0x4c8, 0.0); }
        (5.0, m.pv_f(0x4c8))
    } else {
        let mut t = ((1.0 - c) * b + c * d).clamp(-1.0, 1.0) * -0.523_598_85;
        if !finished && stick_mag < 0.2 { t = 0.0; }
        (1.0, t)
    };
    let (mut r, mut v) = (m.rot[0], m.pv_f(0x300));
    turn_spring(target, k * 0.035, 0.45, k * 0.027_634_384, &mut r, &mut v, 0);
    m.rot[0] = r;
    m.set_pv_f(0x300, v);
    let Some(p) = pilot(w, id) else { return };
    let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let class_scale = w.class(w.mobys[p].o_class).map_or(1.0, |c| c.scale);
    {
        let pm = &mut w.mobys[p];
        pm.scale = class_scale * 0.25;
        pm.alpha = 127;
        pm.drawn = 1;
        pm.pos = pos;
        pm.rot = rot;
        pm.state = 0;
        pm.update_dist = 0;
    }
    ctx.host.advance(&w.mobys[p]);
    w.rebuild(p, &*ctx.host);
    w.mobys[p].mode |= 6;
    w.mobys[p].drive = true;
    let anim = ctx.host.anim(&w.mobys[p]);
    let seq = anim.map_or(0, |a| a.seq_b);
    if anim.is_some_and(|a| a.flags & 2 != 0) && (2..6).contains(&seq) { ctx.host.set_anim(&w.mobys[p], 0, 0, 3); }
    let (rest, air, long) = { let m = &w.mobys[id]; (m.pv_i(0x36c), m.pv_s(0x274) != 0, 16 <= m.pv_s(0x276)) };
    if rest == 0 {
        if air && (seq == 0 || seq == 2) { ctx.host.set_anim(&w.mobys[p], 1, 0, 12); }
        if !air && seq == 1 { ctx.host.set_anim(&w.mobys[p], if long { 2 } else { 0 }, 0, if long { 3 } else { 6 }); }
    } else {
        w.mobys[id].count_i(0x36c);
    }
    if seq < 2 {
        let s = w.mobys[id].pv_f(0x30c);
        let mut speed = s * 3.0 / 0.5;
        if 0.333_333_34 < s { speed *= 3.0; }
        ctx.host.set_anim_speed(&w.mobys[p], speed);
    }
}

/// `0x413818(m, s)` (`func_L04_00412308`): the race's tick. The respawn cuboid; how far along it is (the ticks
/// running backwards counted, the rank's rest 10 after a branch change, else the ranks [`ranks`]); the clock. Through
/// the half way's cuboid it is armed (not while running backwards); through the finish's armed: a lap (under 2: the
/// sound 0xd) — else the finish: the auto-pilot's route the racers' first (`+0x6e0`) from its nearest point, state 3,
/// the results' wait 60, the sounds 0x17 / 0x14 / 0x15, the ranks (and the losses or wins counted, the music
/// `+0x48c` / `+0x75c`, the prize by the wins of the race, × 10 with the bolt multiplier), the skill point of the time
/// (L04 2 under 7800 ticks, L11 3 under 8820); true. Else its place: 1 + the racers ahead (by lap, then progress,
/// past 700 the route's time taken off when not armed), the nearest racer (within 10 it may start following it:
/// one in `(4 − race)·3.75 + 2 + losses·0.3`, for `(race + 1)·120 + 360..720` ticks).
fn finish(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    respawn_pick(w, id, ctx);
    let level = ctx.game.level;
    let set = RouteSet::of(&w.mobys[id]);
    let lap = w.race.lap;
    {
        let m = &w.mobys[id];
        let (old_link, old_point) = (m.pv_i(0x314), m.pv_i(0x318));
        let prev = m.pv_f(0x310);
        let (mut link, mut point) = (old_link, old_point);
        let r = &mut w.race;
        let p = track_progress(&r.paths, &set, m.pos, &mut link, &mut point, lap != 0, Some(&mut r.frame));
        let m = &mut w.mobys[id];
        m.set_pv_f(0x3b8, prev);
        m.set_pv_f(0x310, p);
        m.set_pv_i(0x314, link);
        m.set_pv_i(0x318, point);
        if p < prev || (451 <= point && old_point < 3) { m.set_pv_i(0x3a4, m.pv_i(0x3a4) + 1); } else { m.set_pv_i(0x3a4, 0); }
        if old_link != link { m.set_pv_i(0x3a8, 10); }
        if 0 < m.pv_i(0x3a8) { m.count_i(0x3a8); } else { ranks(w, id, ctx); }
    }
    w.race.g1918 += 1;
    w.race.clock += 1;
    let m = &mut w.mobys[id];
    if ctx.level.in_cuboid(m.pv_i(0x194), m.pos) { m.set_pv_i(0x31c, if 0 < m.pv_i(0x3a4) { 0 } else { 1 }); }
    if ctx.level.in_cuboid(m.pv_i(0x190), m.pos) && m.pv_i(0x31c) != 0 {
        if w.race.lap < 2 {
            w.race.lap += 1;
            // The HUD's lap box for 180 ticks with this lap's time (`$gp+0x18dc` / `$gp+0x18e0`).
            w.race.lap_shown = 180;
            w.race.lap_sound = false;
            ctx.host.play_sound(&w.mobys[id], 13, 0);
            w.race.lap_time = w.race.g1918;
            w.race.g1918 = 0;
            w.mobys[id].set_pv_i(0x31c, 0);
        } else {
            let route = w.mobys[id].pv_i(0x6e0);
            w.race.paths.ensure(ctx.level, route);
            let k = nearest_index(&w.race.paths, level, &w.mobys[id], route, false);
            let paths = &w.race.paths;
            let m = &mut w.mobys[id];
            m.set_pv_i(0x6d0, route);
            let t = k as f32 * paths.step(route) + if w.race.on_water { 20.0 } else { 5.0 };
            autopilot_at(paths, m, route, t);
            m.set_pv_i(0x65c, -1000);
            m.set_pv_i(0x710, 240);
            m.set_pv_i(0x658, 0);
            m.state = 3;
            m.set_pv_i(0x398, 60);
            w.race.finished = 0;
            ctx.host.play_sound(&w.mobys[id], 23, 0);
            for (o, k) in [(0x330, 20), (0x334, 21)] {
                let h = w.mobys[id].pv_i(o);
                if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                let h = ctx.host.play_sound(&w.mobys[id], k, 4);
                w.mobys[id].set_pv_i(o, h);
            }
            ranks(w, id, ctx);
            // `0x412490`: the standings chain and the places by the finish times; then `0x40f758`, which stops the two
            // loops just started.
            super::hud::standings_chain(w, id, ctx);
            stop_sounds(w, id, ctx);
            let race = w.race.race;
            if w.mobys[id].pv_i(0x320) != 1 {
                let m = &mut w.mobys[id];
                m.set_pv_i(0x340, m.pv_i(0x340) + 1);
                bump_flag(ctx, 0x52, 0x5c, race);
                ctx.host.music_request(w.mobys[id].pv_i(0x48c), -1);
            } else {
                ctx.host.music_request(w.mobys[id].pv_i(0x75c), -1);
                let m = &mut w.mobys[id];
                m.set_pv_i(0x344, m.pv_i(0x344) + 1);
                bump_flag(ctx, 0x4d, 0x57, race);
                let n = wins(ctx, race).min(3);
                if 0 < n {
                    let prize = super::record(level, race).prizes[(n - 1) as usize];
                    let multiplier = ctx.game.challenge;
                    ctx.game.bolts = ctx.game.bolts.wrapping_add((prize * if multiplier { 10 } else { 1 }) as u32);
                }
            }
            // The best time (`0x1a7368` / `0x1a737c` by the race, saved with the game).
            if w.mobys[id].pv_i(0x320) != -1 {
                let k = race.clamp(0, 4) as usize + if level == 4 { 0 } else { 5 };
                let best = &mut ctx.game.race_times[k];
                if w.race.clock < *best || *best == 0 { *best = w.race.clock; }
            }
            let limit = if level == 4 { 7800 } else { 8820 };
            let ticks = w.race.clock;
            if 0 < ticks && ticks / 6 <= limit / 6 {
                let k = if level == 4 { 2 } else { 3 };
                // The standings' text 0x1233 for 300 ticks (`$gp+0x18fc`).
                if let Some(b) = ctx.game.skill_points.get_mut(k) {
                    if *b == 0 {
                        *b = 1;
                        w.race.skill_shown = 300;
                    }
                }
            }
            return true;
        }
    }
    let m = &w.mobys[id];
    let mut time = m.pv_f(0x310);
    let armed = m.pv_i(0x31c) != 0;
    if 700.0 < time && !armed { time -= w.race.route_time; }
    let mut place = 1;
    let mut nearest = 999.0;
    let mut near_racer = None;
    for r in racers(w, id) {
        let rm = &w.mobys[r];
        if !(2..4).contains(&rm.state) { continue; }
        let mut other = rm.pv_f(0x11c);
        if 700.0 < other && rm.pv_i(0x15c) == 0 { other -= w.race.route_time; }
        let ahead = if rm.pv_i(0x1a0) != 0 { true } else {
            let ol = rm.pv_i(0x160);
            !(ol <= lap && (!(time <= other) || ol != lap))
        };
        if ahead { place += 1; }
        w.mobys[r].set_pv_i(0x168, 0);
        let d = sub(w.mobys[id].pos, w.mobys[r].pos);
        let d = dot(d, d);
        if d < nearest {
            near_racer = Some(r);
            nearest = d;
        }
    }
    w.mobys[id].set_pv_i(0x320, place);
    ai::set_link(&mut w.mobys[id], 0x3b0, near_racer);
    if let Some(r) = near_racer {
        w.mobys[r].set_pv_i(0x168, 1);
        if nearest < 10.0 {
            let race = w.race.race;
            let n = ((4 - race) as f32 * 3.75) as i32 + 2 + (losses(ctx, race) as f32 * 0.3) as i32;
            let (bm, rm) = (&w.mobys[id], &w.mobys[r]);
            let ok = 61 <= w.race.clock && rm.pv_i(0x1f0) < 4 && rm.pv_i(0x1c0) <= 0 && bm.pv_i(0x4c4) == 0 && rm.state == 2 && bm.pv_i(0x314) == rm.pv_i(0x120)
                && (lap <= rm.pv_i(0x160) || bm.pv_f(0x27c) <= 0.0);
            if ok && ctx.game.rng.randi(n - 1) == 0 {
                let ticks = (race + 1) * 120 + ctx.game.rng.rand_range(120, 360) + 360;
                let rm = &mut w.mobys[r];
                rm.set_pv_i(0x1bc, 0);
                rm.set_pv_i(0x1c0, ticks);
                rm.set_pv_f(0x1e4, 1.0);
                rm.set_pv_f(0x1e8, 1.0);
                rm.set_pv_f(0x1ec, 0.0);
                rm.set_pv_i(0x1f0, 0);
                w.mobys[id].set_pv_f(0x63c, 1.0);
                ai::set_link(&mut w.mobys[id], 0x4c4, Some(r));
            } else if w.mobys[r].pv_i(0x1c0) <= 0 {
                w.mobys[r].set_pv_i(0x1f0, 4);
            }
        }
    }
    let m = &mut w.mobys[id];
    if m.pv_i(0x3ac) != m.pv_i(0x3b0) {
        let v = m.pv_i(0x3b0);
        m.set_pv_i(0x3ac, v);
        m.set_pv_i(0x3b4, 0);
    }
    false
}

/// `0x413640(m)` (`func_L04_00412130`, the decomp's `UpdateRacerRanks`): every racer's progress tracked; those racing
/// ranked by it (a bubble sort: one behind with no more laps swaps), their rank `+0xa2` and the count `+0xa3`.
fn ranks(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let set = RouteSet::of(&w.mobys[id]);
    let mut list = Vec::new();
    for r in racers(w, id) {
        let rm = &w.mobys[r];
        let (mut link, mut point) = (rm.pv_i(0x120), rm.pv_i(0x124));
        let wrap = rm.pv_i(0x160) != 0;
        let p = track_progress(&w.race.paths, &set, rm.pos, &mut link, &mut point, wrap, None);
        let rm = &mut w.mobys[r];
        rm.set_pv_i(0x120, link);
        rm.set_pv_i(0x124, point);
        rm.set_pv_f(0x11c, p);
        if rm.state == 2 { list.push(r); }
    }
    let n = list.len();
    for i in 0..n.saturating_sub(1) {
        for j in 0..n - i - 1 {
            let (a, b) = (&w.mobys[list[j]], &w.mobys[list[j + 1]]);
            if a.pv_f(0x11c) < b.pv_f(0x11c) && a.pv_i(0x160) <= b.pv_i(0x160) { list.swap(j, j + 1); }
        }
    }
    for (l, &r) in list.iter().enumerate() {
        w.mobys[r].set_pv_b(0xa2, (l + 1) as u8);
        w.mobys[r].set_pv_b(0xa3, n as u8);
    }
}

/// `0x40c228` (L11 its copy, the same code): the race select's screen, in the counters' place, for the race picked
/// (`$gp+0x18c4`). The panel 0x45 (`0x60442d00`) and 0x44 (`0x55f0c070`) at (256, 200), scaled (1, 0.92); the arrows
/// 0x4a at (259, 283) and 0x4b at (251, 283), each `0x80f0f0f0` while → (←) is held, else pulsing over 50 frames
/// between `0x80ffde8d` and `0x60442d00`; the race's number (0x2e08, font 3 centred at (256, 60)); 0x2dd6 (119, 286),
/// 0x2dd7 (356, 286), 0x2dd5 (181, 286), 0x2dd8 (279, 286) in font 2, left; the race's title (its record's text,
/// font 1 centred at (256, 110)). With `k` 5 (Joba 10) and the prize its record's by its wins so far (at most the
/// third, × 10 in the challenge mode): a race not open (no 0x10000000 and the one before it not won) — 0x2e09 (font
/// 3 centred, 150 down) and `"%s %d %s"` of 0x2e13, the prize and 0x2df4 (210 + k down); an open one — its rules
/// in font 1 centred: 0x2e0f / 0x2e10 (flag 4; Joba 0x2e2f / 0x2e30) at 150 − k, 0x2e11 / 0x2e12 (flag 1) at 170 −
/// k, on Joba 0x2e32 / 0x2e31 (flag 0x10) at 190 − k, 0x2e5b (flag 2) at 210 − k (190 − k off Joba); the prize
/// (`"%s %d %s"`, or `"%s ?"` with none: 0x1b0900) at 210 + k; its wins (0x2e14) at 230 + k.
///
/// The best time of an open race (its words crate::game::GameState::race_times): see the code.
///
/// Not ported: the GUI's view tangent it sets for its sprites (`0x1bdc70`: tan 0.5622, the GUI's own here; then the
/// bike's `+0x28c`, which its tick sets: [`boosts`]), the wide screen's 0.9 (`0x1a7b98`), the PAL sizes.
pub fn draw_select(w: &MobyWorld, id: usize, inp: &crate::hud::ScreenIn, out: &mut Vec<crate::hud::ScreenDraw>) -> bool {
    use crate::hud::{format_int, lerp_rgba, text_of, Align, Draw, ScreenDraw};
    let _ = id;
    let game = inp.game;
    let white = 0x80f0_f0f0;
    let text = |out: &mut Vec<ScreenDraw>, font: u8, text: Vec<u8>, pos: [f32; 2], align: Align| out.push(ScreenDraw::Text { font, text, pos, scale: 1.0, colour: white, align });
    for (sprite, colour) in [(0x45, 0x6044_2d00u32), (0x44, 0x55f0_c070)] {
        out.push(ScreenDraw::Hud(Draw::Sprite { id: sprite, pos: [256.0, 200.0], scale: [1.0, 0.92], frame: 0.0, colour }));
    }
    let k = w.frame.rem_euclid(50);
    let t = if k < 25 { k as f32 * 0.04 } else { 1.0 - (k - 25) as f32 * 0.04 };
    let pulse = lerp_rgba(t, 0x80ff_de8d, 0x6044_2d00);
    let held = inp.hero.items.held;
    for (sprite, x, b) in [(0x4a, 259.0, button::RIGHT), (0x4b, 251.0, button::LEFT)] {
        let colour = if held & b != 0 { white } else { pulse };
        out.push(ScreenDraw::Hud(Draw::Sprite { id: sprite, pos: [x, 283.0], scale: [1.0, 1.0], frame: 0.0, colour }));
    }
    let pick = w.race.pick;
    text(out, 3, format_int(&text_of(inp, 0x2e08, usize::MAX), pick + 1), [256.0, 60.0], Align::Centre);
    for (t, x) in [(0x2dd6, 119.0), (0x2dd7, 356.0), (0x2dd5, 181.0), (0x2dd8, 279.0)] {
        text(out, 2, text_of(inp, t, usize::MAX), [x, 286.0], Align::Left);
    }
    let rec = super::record(game.level, pick);
    text(out, 1, text_of(inp, rec.text as u32, usize::MAX), [256.0, 110.0], Align::Centre);
    let joba = game.level == 0xb;
    let k = if joba { 10.0 } else { 5.0 };
    let won = wins_in(game, pick);
    let prize = rec.prizes[won.clamp(0, 2) as usize];
    let prize = if game.challenge { prize * 10 } else { prize };
    let prize_text = |none: bool| -> Vec<u8> {
        let mut t = text_of(inp, 0x2e13, usize::MAX);
        if none {
            t.extend_from_slice(b" ?");
        } else {
            t.push(b' ');
            t.extend_from_slice(prize.to_string().as_bytes());
            t.push(b' ');
            t.extend_from_slice(&text_of(inp, 0x2df4, usize::MAX));
        }
        t
    };
    if rec.flags & 0x1000_0000 == 0 && (pick < 1 || wins_in(game, pick - 1) < 1) {
        text(out, 3, text_of(inp, 0x2e09, usize::MAX), [256.0, 150.0], Align::Centre);
        text(out, 1, prize_text(false), [256.0, 210.0 + k], Align::Centre);
        return true;
    }
    let rule = if rec.flags & 4 == 0 { if joba { 0x2e2f } else { 0x2e0f } } else if joba { 0x2e30 } else { 0x2e10 };
    text(out, 1, text_of(inp, rule, usize::MAX), [256.0, 150.0 - k], Align::Centre);
    text(out, 1, text_of(inp, if rec.flags & 1 != 0 { 0x2e12 } else { 0x2e11 }, usize::MAX), [256.0, 170.0 - k], Align::Centre);
    if joba {
        text(out, 1, text_of(inp, if rec.flags & 0x10 != 0 { 0x2e31 } else { 0x2e32 }, usize::MAX), [256.0, 190.0 - k], Align::Centre);
        if rec.flags & 2 != 0 { text(out, 1, text_of(inp, 0x2e5b, usize::MAX), [256.0, 210.0 - k], Align::Centre); }
    } else if rec.flags & 2 != 0 {
        text(out, 1, text_of(inp, 0x2e5b, usize::MAX), [256.0, 190.0 - k], Align::Centre);
    }
    text(out, 1, prize_text(rec.prizes[won.clamp(0, 2) as usize] == 0), [256.0, 210.0 + k], Align::Centre);
    // The best time (`0x1a7368` / `0x1a737c` by the race): `"%s %d :%d.%d"` (`":0%d"` under 10 seconds) of 0x305e,
    // the minutes, seconds and tenths, font 2 centred 50 right of the middle; the wins' text 110 left of it then.
    let best = game.race_times[pick.clamp(0, 4) as usize + if joba { 5 } else { 0 }];
    let mut left = 0.0;
    if best != 0 {
        left = 110.0;
        let (m, sec, tenth) = ((best / 3600) % 60, (best / 60) % 60, (best / 6) % 10);
        let fmt: &[u8] = if sec < 10 { b"%s %d :0%d.%d" } else { b"%s %d :%d.%d" };
        use super::hud::Arg;
        let line = super::hud::format(fmt, &[Arg::S(&text_of(inp, 0x305e, usize::MAX)), Arg::D(m), Arg::D(sec), Arg::D(tenth)]);
        out.push(ScreenDraw::Text { font: 2, text: line, pos: [306.0, 230.0 + k], scale: 1.0, colour: white, align: Align::Centre });
    }
    text(out, 1, format_int(&text_of(inp, 0x2e14, usize::MAX), won), [256.0 - left, 230.0 + k], Align::Centre);
    true
}
