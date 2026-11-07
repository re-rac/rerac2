//! Tabora's breather **0x09c6** (L08 `0x3db3c0`, 244 placed; the decomp's `func_US2_L08_003DB3BC`): a walker that
//! lies under the sand, bursts out, patrols and hunts, and breathes fire at Ratchet. On the framework
//! (crate::moby::ai): health `+0x20`, reaction `+0x50` (a hurt goes to state 9), movement `+0x120` (its fence path
//! `+0x15c`), and Tabora's target search (crate::moby::classes::level08::tabora_ai) into the block `+0x210` (`+0x230`
//! the aim, `+0x250` the moby, `+0x254` the result). No ported class shares its code.
//!
//! **Its pvars:** `+0x3e0` home, `+0x3f0` the goal, `+0x400` / `+0x404` / `+0x468` the mobys its wake-ups watch
//! (`+0x404` marked gone with it marked: it goes at once; `+0x468` also a moby it keeps 3.6 from), `+0x408` / `+0x40a`
//! its update distance and its wander radius (s16), `+0x40c` how it starts (0 under the sand, 1 / 3 standing, 2
//! launched along `+0xd0`; 3 goes straight for Ratchet when it first sees him), `+0x40d` what wakes it (0 Ratchet in
//! its first volumes, 1 the moby `+0x400` past its state 2, 2 a poke, 3 / 4 the moby `+0x468` / `+0x400` gone, 5 the
//! moby `+0x404` past its state 2 or gone; a poke with bit 1 always), `+0x40e` the search's near range (s16: 12
//! alarmed, 3 after), `+0x410` / `+0x414` / `+0x418` its first volumes (path, sphere, cuboid) and `+0x41c` /
//! `+0x420` / `+0x424` the hunt's, `+0x428` its path (`+0x42c` a fence path instead), `+0x430` the knock's ticks,
//! `+0x434` the floor under it, `+0x440` its timer, `+0x448` its alarm, `+0x44c` its wait after an attack, `+0x460`
//! / `+0x464` the tilt's speeds, `+0x46c` / `+0x470` / `+0x474` the weave (degrees, its angle, its ticks),
//! `+0x480` the stuck ticks of the run in, `+0x484` / `+0x488` the groups it alarms, `+0x48c` the chase after a hurt.
//!
//! **States** (`+0x20`): 0 the set-up ([`setup`], `0x3d97a0`). 1 under the sand until woken ([`wake`]), then 3 a
//! wait of 0..180 and 2 bursting out (sequence 4, sub 0: then 6 sub 1) or sinking (sub 1: sequence 11, then 1), sand
//! puffs about it early in its sequence (crate::moby::classes::level08::tabora_ai::puffs). 4 launched: it flies on
//! its velocity, bouncing off what it meets (0.8 of it, a 0x0945 / 0x09c6 passed through) until it comes down on the
//! floor (sequence 15) → 5, its landing (to its end, then 6 sub 1). 6 calm: idles (0 / 1) and walks to new goals
//! ([`walk`]: 120″ / 360″ / 90°, 8, top 2, stop 1); home and down when nothing is about (sub 2); seen (or alarmed),
//! 1 in 9 the hunt (7, sequence 2 / 3); a type-3 starter at once the run in (8). 7 hunting: goals near Ratchet
//! (faster turns while waiting); losing him, 1 in 4 a breath anyway, or calm; within 12 (or chasing) the run in (8).
//! 8 the attack: the run in to him (`walk` 360″ / 180″ / 180°, 16, top 4, stop 0.5; stuck 60 ticks without him close
//! ahead: back off), within 4.2 facing within 90° the wind-up (sequence 6), the breath (7: facing him; past frame 24
//! [`breathe`] each tick), the end (8), then 7 with a 90..180 wait. 9 the knock (the framework's flight, 360 ticks at
//! most): dead → 11; outside its fence → 10 (walks home until back in), else 7. 11 dying: the explosion, its bolts
//! (flag 0x1000 for one launched), its sparkles, its uid marked; deleted.
//!
//! **Each tick** ([`head`], `0x3da730`; [`tail`], `0x3da660`): its shadow's probe within 24 of the camera; the floor
//! (`0x3d9748`: a line from home 3 up to 20 down, flags 2); on the floor's surface 1 or 11 within 0.12 of it, its
//! health 0 and state 11 at once; its size eased to 1.7 times its class's (a twentieth a tick); the frame's damage
//! (`0x336b18` = L00 `0x327e38`, crate::moby::ai::take_damage): hurt, 240 ticks of chase and its groups alarmed
//! ([`alarm`], `0x3d9e80`; a kind-5 hurt clears the reaction's `+0x90`); below state 9 (but 4) its pitch and roll
//! eased to the ground's normal under it (`0x32c8d8` = L00 `0x31f230`, 0.1°/tick², at most 6° a tick); running in,
//! its target's place and aim follow Ratchet. At the end the alarm (a poke: 300..360 and the near range 12 once under
//! 200; counted down while nothing is targeted, at its end the near range 3), its ticks, the chase (running in), the
//! timer and the wait. Out of 30..993 on any axis it is deleted.
//!
//! **The walk** ([`walk`], `0x3da490(turn accel, _, most, accel, _, top, stop)`): the movement limits
//! (`0x33bf38` = L00 `0x32dcf0`, crate::moby::ai::senses), the stop distance (`+0x154`), the weave flipped every
//! 30..90 ticks (none while chasing, winding up or within 5 of the goal), the move toward the goal
//! (`0x33ae78` = L00 `0x32cc30`, crate::moby::ai::move_toward); its sequence's speed 30 × its speed and turn (0..1.5;
//! blocked, the alarm counted down and only the turn's).
//!
//! The engine functions with the port's: `0x33b0e0` = L00 `0x32ce98` (crate::moby::ai::hold), `0x33b100` = L00
//! `0x32ceb8` (the move in place to a yaw, crate::moby::ai::move_step), `0x33ae20` (L01 `0x322450`: the move to a
//! point), `0x33c030` = L00 `0x32df58` (the fence test, [`fence_ok`]), `0x337860` = L00 `0x328b80`
//! (crate::moby::ai::flight), `0x32a760` = L00 `0x31d2c8` (crate::moby::ai::attack_joint), `0x32a808` (L01
//! `0x3132c8`, crate::moby::ai::attack_line), `0x32b058` = L00 `0x31da58` (crate::shadow_volumes::probe_near),
//! `0x2ee1d8` = L00 `0x2e23e0` (the last line's surface), `0x2fbb18` = L00 `0x2eff48` (rot_dist), `0x2faef8` = L00
//! `0x2ef2a8` (the bounce), `0x329db8` / `0x329ea0` = L00 `0x31c9d8` / `0x31cac0` (its group's members), the rest as
//! crate::moby::classes::level08::burrower's.
//!
//! The update starts with the level's chunk gate `0x1ca080` (as 0x0fff's, crate::level::LevelData::off_chunk).
//!
//! Not ported: the head aims (`0x332d48` = L00 `0x324208` on the joint blocks `+0x260..`: the turns `+0x2c0..` toward
//! its target while it runs in), the ground marks of its death (`0x3d92c0`, the copy of 0x0fff's unported marks), the
//! path length (`0x351498`), the arena statistics wake-up (`0x2b72f0`), and the goal's pull into the path table's
//! word before path 0 when it has no path (`0x351560(goal, $0x1fe7bc)`).

use super::tabora_ai::{self, find_target, Volumes};
use crate::hero::physics::{add, add_rot, atan2, dist2, dot, len, rot_dist, scale, set_len, sub, turn_ease, V3};
use crate::moby::blast::{self, Explosion};
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::{ai, bolts, countdown, Ctx, MobyWorld};
use std::f32::consts::{FRAC_PI_2, PI};

pub const CLASS: i16 = 0x09c6;

const HEALTH: usize = 0x20;
const REACT: usize = 0x50;
const MV: usize = 0x120;
const T_BLOCK: usize = 0x210;
const T_AIM: usize = 0x230;
const T_MOBY: usize = 0x250;
const T_RESULT: usize = 0x254;
const HOME: usize = 0x3e0;
const GOAL: usize = 0x3f0;
const UD: usize = 0x408;
const START: usize = 0x40c;
const WAKE: usize = 0x40d;
const NEAR: usize = 0x40e;
const PATH: usize = 0x428;
const KNOCK: usize = 0x430;
const FLOOR: usize = 0x434;
const TIMER: usize = 0x440;
const ALARM: usize = 0x448;
const WAIT: usize = 0x44c;
const MOUNT: usize = 0x468;
const STUCK: usize = 0x480;
const CHASE: usize = 0x48c;
const LAUNCH: usize = 0xd0;

fn i_down(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let m = &mut w.mobys[id];
    let mut t = m.pv_i(off);
    let r = countdown(&mut t);
    m.set_pv_i(off, t);
    r
}

/// `if seq ≠ rand & 1 { sequence rand & 1 }`: the game draws twice.
fn idle_seq(w: &MobyWorld, id: usize, ctx: &mut Ctx, add_: u8, blend: u8) {
    let r = (ctx.game.rng.rand() & 1) as u8 + add_;
    if seq(w, id, ctx) != r {
        let s = (ctx.game.rng.rand() & 1) as u8 | add_;
        ctx.host.set_anim(&w.mobys[id], s, 0, blend);
    }
}

/// `seq` with a blend of 3..6 drawn only when it does not play.
fn seq_rand_blend(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8) {
    if seq(w, id, ctx) != s {
        let b = ctx.game.rng.rand_range(3, 6) as u8;
        ctx.host.set_anim(&w.mobys[id], s, 0, b);
    }
}

/// The hunt's search (`+0x41c` / `+0x424` / `+0x420`): its draw distance across, the near range `+0x40e`.
fn search(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> i32 {
    let m = &w.mobys[id];
    let (range, near) = (m.draw_dist as f32, m.pv_s(NEAR) as f32);
    let v = Volumes(m.pv_i(0x41c), m.pv_i(0x424), m.pv_i(0x420), -1);
    find_target(w, id, ctx, T_BLOCK, range, near, v)
}

/// `0x3d9748`: the floor under it (`+0x434`: a line from home 3 up to 20 down, flags 2; 0 none) and its surface.
fn floor(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> i32 {
    let m = &w.mobys[id];
    let (p, z) = (m.pos, m.pv_f(HOME + 8));
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, [p[0], p[1], z + 3.0], [p[0], p[1], z - 20.0], crate::collision::QueryFlags(2), None));
    w.mobys[id].set_pv_f(FLOOR, hit.as_ref().map_or(0.0, |o| o.point[2]));
    hit.map_or(-1, |o| o.surface_id())
}

/// `0x3d9e80`: its group, `+0x484` and (the game reads `+0x484` again for it) `+0x488` poked; itself too.
fn alarm(w: &mut MobyWorld, id: usize) {
    let m = &w.mobys[id];
    let (g, a, b) = (m.group as i32, m.pv_i(0x484), m.pv_i(0x488));
    if g != -1 { w.poke_group(g, 1); }
    if a != -1 { w.poke_group(a, 1); }
    if b != -1 { w.poke_group(a, 1); }
    w.mobys[id].timer = 1;
}

/// `0x33c030` = L00 `0x32df58`: inside its fence path's polygon clear of its walls; else marked out of it (`+0x9c`
/// of the movement block |= 1) — no fence path: false, unmarked.
fn fence_ok(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    let path = m.pv_i(MV + 0x3c);
    if path == -1 { return false; }
    let Some(pts) = crate::path_network::points_now(ctx.level, &ctx.game.path_openings, path) else { return false };
    if ctx.level.in_polygon(path, m.pos) && ai::fence(&pts, m.pv_f(MV), m.pos).is_none() { return true; }
    let m = &mut w.mobys[id];
    m.set_pv_i(MV + 0x9c, m.pv_i(MV + 0x9c) | 1);
    false
}

/// `0x3da730` (module docs); false when it went to state 11 at once.
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    let surface = floor(w, id, ctx);
    let m = &mut w.mobys[id];
    if m.pos[2] < m.pv_f(FLOOR) + 0.12 && (surface == 1 || surface == 11) {
        m.set_pv_f(HEALTH, 0.0);
        m.state = 11;
        return false;
    }
    let s = w.class(CLASS).map_or(1.0, |c| c.scale);
    let m = &mut w.mobys[id];
    m.scale += (s * 1.7 - m.scale) * 0.05;
    let hit = ai::take_damage(w, id, ctx, false);
    if hit != 0 {
        let m = &mut w.mobys[id];
        if hit == 5 { m.set_pv_i(REACT + 0x90, 0); }
        m.set_pv_i(CHASE, 0xf0);
        alarm(w, id);
    }
    let state = w.mobys[id].state;
    if state < 9 && state != 4 {
        let m = &w.mobys[id];
        let p = m.pos;
        let normal = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, [p[0], p[1], p[2] + 0.5], [p[0], p[1], 0.01], crate::collision::QueryFlags(2), None)).map_or([0.0; 3], |o| set_len(o.normal, 1.0));
        let rows = crate::hero::physics::euler_rows([0.0, 0.0, m.rot[2]]);
        let l = crate::gravity::to_local(normal, &rows);
        let m = &mut w.mobys[id];
        let (mut a, mut v) = (m.rot[1], m.pv_f(0x460));
        turn_ease(atan2(l[2], l[0]), 0.001_745_329_4, 0.001_745_329_4, f32::from_bits(0x3dd6_7752), &mut a, &mut v, 0);
        m.rot[1] = a;
        m.set_pv_f(0x460, v);
        let (mut a, mut v) = (m.rot[0], m.pv_f(0x464));
        turn_ease(-atan2(l[2], l[1]), 0.001_745_329_4, 0.001_745_329_4, f32::from_bits(0x3dd6_7752), &mut a, &mut v, 0);
        m.rot[0] = a;
        m.set_pv_f(0x464, v);
    }
    let m = &mut w.mobys[id];
    if m.state == 8 && m.sub_state == 0 && ai::link(m, T_MOBY) == ctx.hero_moby && ctx.hero_moby.is_some() {
        m.set_pv_v(T_BLOCK, ctx.hero.pos);
        m.set_pv_v(T_AIM, ctx.hero.aim_point);
    }
    true
}

/// `0x3da660` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].timer != 0 {
        w.mobys[id].timer = 0;
        if w.mobys[id].pv_i(ALARM) < 200 {
            let t = ctx.game.rng.rand_range(300, 0x168);
            let m = &mut w.mobys[id];
            m.set_pv_i(ALARM, t);
            m.set_pv_s(NEAR, 12);
        }
    }
    if w.mobys[id].pv_i(T_RESULT) == 2 && i_down(w, id, ALARM) { w.mobys[id].set_pv_s(NEAR, 3); }
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) <= 0xfffd { m.state_ticks = m.state_ticks.wrapping_add(1); }
    if m.state == 8 && m.sub_state == 0 { i_down(w, id, CHASE); }
    i_down(w, id, TIMER);
    i_down(w, id, WAIT);
}

/// `0x3d8ab0`: the goal kept in its path (`+0x428`): outside, half the time (or while nothing is targeted, or
/// waiting) home or (half of those, with one) 3 about the moby `+0x404`, else 3 about Ratchet.
fn keep_goal(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let path = m.pv_i(PATH);
    if path == -1 { return; }
    if tabora_ai::in_volumes(ctx.level, m.pv_v(GOAL), path, -1, -1, -1) != 0 { return; }
    let away = ctx.game.rng.rand() & 1 != 0 || m.pv_i(T_RESULT) == 2 || m.pv_i(WAIT) != 0;
    let other = m.pv_i(0x404);
    let goal = if away {
        if other == -1 || ctx.game.rng.rand() & 1 != 0 {
            m.pv_v(HOME)
        } else {
            let x = ctx.game.rng.randf(-PI, PI).cos() * 3.0;
            let y = ctx.game.rng.randf(-PI, PI).sin() * 3.0;
            let at = w.mobys.get(other as usize).map_or([0.0; 3], |o| o.pos);
            add([x, y, 0.0], at)
        }
    } else {
        let x = ctx.game.rng.randf(-PI, PI).cos() * 3.0;
        let y = ctx.game.rng.randf(-PI, PI).sin() * 3.0;
        add([x, y, 0.0], ctx.hero.pos)
    };
    w.mobys[id].set_pv_v(GOAL, goal);
}

/// `0x3da100`: a new goal: waiting within 8 of Ratchet, 8 away from him; else (something targeted, 1 in 2) 3 about
/// him, or 8 about itself; pushed 4/d from each of its group within 3, 5 off the moby `+0x468` when it or the goal is
/// within 3.6 of it; at its own height; kept in its path ([`keep_goal`]).
fn new_goal(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let pos = m.pos;
    let mut goal = if m.pv_i(WAIT) != 0 && dist2(pos, ctx.hero.pos) < 8.0 {
        let d = sub(pos, ctx.hero.pos);
        add(pos, set_len([d[0], d[1], 0.0], 8.0))
    } else if m.pv_i(T_RESULT) != 2 && ctx.game.rng.randi(2) == 0 {
        let x = ctx.game.rng.randf(-PI, PI).cos() * 3.0;
        let y = ctx.game.rng.randf(-PI, PI).sin() * 3.0;
        add([x, y, 0.0], ctx.hero.pos)
    } else {
        let x = ctx.game.rng.randf(-PI, PI).cos() * 8.0;
        let y = ctx.game.rng.randf(-PI, PI).sin() * 8.0;
        add([x, y, 0.0], pos)
    };
    let group = w.mobys[id].group as i32;
    if let Some(members) = usize::try_from(group).ok().and_then(|g| w.groups.get(g)) {
        for &o in members {
            let Some(other) = w.mobys.get(o).filter(|m| m.state < 0x80 && o != id) else { continue };
            let d = dist2(pos, other.pos);
            if d < 3.0 { goal = add(goal, set_len(sub(pos, other.pos), 4.0 / d)); }
        }
    }
    let mount = w.mobys[id].pv_i(MOUNT);
    if let Some(mt) = usize::try_from(mount).ok().and_then(|i| w.mobys.get(i)).filter(|m| m.state != 0xfe && m.state != 0xfd) {
        if dist2(pos, mt.pos) < 3.6 || dist2(goal, mt.pos) < 3.6 {
            let d = sub(pos, mt.pos);
            goal = add(set_len([d[0], d[1], 0.0], 5.0), pos);
        }
    }
    goal[2] = pos[2];
    w.mobys[id].set_pv_v(GOAL, goal);
    keep_goal(w, id, ctx);
}

/// `0x3da490` (module docs); the move's flags (0x80 at the goal).
#[allow(clippy::too_many_arguments)]
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, turn_a: f32, turn_most: f32, accel: f32, top: f32, stop: f32) -> i32 {
    let m = &mut w.mobys[id];
    ai::senses(m, accel * 0.000_277_777_8, top * 0.016_666_668, turn_a * 4.848_137_5e-6, turn_most * 0.000_290_888_25);
    m.set_pv_f(MV + 0x34, stop);
    if i_down(w, id, 0x474) {
        let k = ctx.game.rng.randf(30.0, 90.0) as i32;
        let m = &mut w.mobys[id];
        let deg = -m.pv_f(0x46c);
        m.set_pv_f(0x46c, deg);
        m.set_pv_f(0x470, deg * 0.017_453_294);
        m.set_pv_i(0x474, k);
    }
    let m = &mut w.mobys[id];
    let keep = m.pv_i(CHASE) == 0 && !(m.state == 8 && m.sub_state != 0) && 5.0 <= dist2(m.pos, m.pv_v(GOAL));
    if !keep { m.set_pv_f(0x470, 0.0); }
    let (offset, goal) = (m.pv_f(0x470), m.pv_v(GOAL));
    let flags = ai::move_toward(w, id, ctx, offset, goal);
    let m = &w.mobys[id];
    let turn = m.pv_f(MV + 0xb4).abs();
    let mut speed = len(m.pv_v(MV + 0xa0)) + turn;
    if flags & 0xf != 0 {
        i_down(w, id, ALARM);
        speed = turn + 0.010_000_001;
    }
    let s = speed * 29.999_998;
    let s = if 1.5 < s { 1.5 } else if s < 0.0 { 0.0 } else { s };
    ctx.host.set_anim_speed(&w.mobys[id], s);
    flags
}

/// The move in place to a yaw (`0x33b100` = L00 `0x32ceb8`).
fn turn_in_place(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, yaw: f32) {
    let pos = w.mobys[id].pos;
    ai::move_step(w, id, ctx, yaw, pos);
}

/// The volumes' reach (+2) as an update distance.
fn reach(ctx: &Ctx, p: V3, a: i32, b: i32, c: i32) -> f32 { tabora_ai::volume_reach(ctx.level, p, a, b, c, -1) }

/// `0x3d97a0` (module docs): gone with the moby `+0x404` (both marked); home, the floor, its update distance from its
/// first volumes (the wander radius as it was, the hunt's for the first), the reaction and movement blocks, its start.
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_pv_b(0x48, 15);
    m.set_pv_b(0x4a, 9);
    m.set_pv_b(0x29, 1);
    let other = m.pv_i(0x404);
    if let Some(o) = usize::try_from(other).ok().and_then(|i| w.mobys.get(i)).filter(|o| o.mission != 0xff) {
        let ou = o.uid;
        let done = ctx.game.uid_dead.get(ou.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(ou) != 0;
        if done && ctx.game.uid_count(w.mobys[id].uid) != 0 {
            w.delete(id);
            return;
        }
    }
    let m = &mut w.mobys[id];
    let pos = m.pos;
    m.set_pv_v(HOME, pos);
    floor(w, id, ctx);
    let m = &mut w.mobys[id];
    let r = reach(ctx, pos, m.pv_i(0x410), m.pv_i(0x418), m.pv_i(0x414));
    if 0.0 < r { m.update_dist = (r + 2.0) as i32 as u8; }
    let ud = m.update_dist as i16;
    m.set_pv_s(UD, ud);
    m.set_pv_s(UD + 2, ud);
    let r = reach(ctx, pos, m.pv_i(0x41c), m.pv_i(0x424), m.pv_i(0x420));
    if 0.0 < r { m.set_pv_s(UD, (r + 2.0) as i32 as i16); }
    // The reaction block (`+0x50`): a hurt goes to state 9.
    m.set_pv_s(0x88, 0);
    m.set_pv_b(0x6a, 0);
    m.set_pv_i(0x98, -1);
    m.set_pv_i(0x10, REACT as i32);
    m.set_pv_b(0x60, 9);
    m.set_pv_b(0x61, 12);
    m.set_pv_b(0x62, 12);
    m.set_pv_b(0x66, 13);
    m.set_pv_b(0x67, 10);
    m.set_pv_f(0x6c, 0.25);
    m.set_pv_f(0x70, 0.116_666_675);
    m.set_pv_f(0x78, 0.008_333_334);
    m.set_pv_f(0x74, 0.004_166_667);
    m.set_pv_f(0x7c, 0.004_166_667);
    m.set_pv_f(0x80, 0.8);
    m.set_pv_f(0x84, 0.8);
    m.set_pv_f(0x8c, 0.4);
    m.set_pv_b(0xbc, 2);
    m.set_pv_b(0xbe, 2);
    m.set_pv_b(0xbd, 8);
    m.set_pv_b(0xbf, 8);
    m.set_pv_b(0xc2, 4);
    m.set_pv_b(0xc3, 14);
    m.set_pv_f(0xa8, 0.1);
    m.set_pv_f(0xac, 0.3);
    m.set_pv_i(0xf8, 0);
    m.set_pv_b(0x28, 1);
    m.set_pv_f(0x30, 0.2);
    m.set_pv_b(0xc6, 0);
    m.set_pv_b(0xc4, 0);
    m.set_pv_i(KNOCK, 0);
    // The movement block (`+0x120`): its fence path, the turn left before it walks (5°), radius 0.7, slope 45°.
    let fence = if m.pv_i(0x42c) != -1 { m.pv_i(0x42c) } else { m.pv_i(PATH) };
    m.set_pv_i(MV + 0x3c, fence);
    m.set_pv_f(MV + 0x30, 0.087_266_475);
    m.set_pv_f(MV, 0.7);
    m.set_pv_f(MV + 0x10, 0.785_398_24);
    m.set_pv_v(GOAL, pos);
    m.set_pv_i(0x450, 0);
    match m.pv_b(START) {
        0 => {
            m.pos[2] = m.pv_f(FLOOR);
            let p = m.pos;
            m.set_pv_v(HOME, p);
            m.set_pv_v(MV + 0xa0, [0.0; 3]);
            set_seq(w, id, ctx, 11, 0, 2);
            let m = &mut w.mobys[id];
            m.set_state(1, -1);
            m.drawn = 0;
            m.has_collision = false;
            m.mode = (m.mode & 0xefff) | 1;
        }
        1 | 3 => {
            m.pos[2] = m.pv_f(FLOOR);
            let p = m.pos;
            m.set_pv_v(HOME, p);
            m.set_pv_v(MV + 0xa0, [0.0; 3]);
            idle_seq(w, id, ctx, 0, 2);
            let m = &mut w.mobys[id];
            m.set_state(6, -1);
            m.drawn = 1;
            m.mode = (m.mode | 0x1000) & !1;
        }
        2 => {
            m.set_pv_v(MV + 0xa0, [0.0; 3]);
            m.pos = add(m.pos, m.pv_v(LAUNCH));
            let p = m.pos;
            m.set_pv_v(HOME, p);
            set_seq(w, id, ctx, 14, 0, 2);
            let s = w.class(CLASS).map_or(1.0, |c| c.scale);
            let m = &mut w.mobys[id];
            m.scale = s * 0.5;
            m.drawn = 1;
            m.mode = (m.mode | 0x1000) & !1;
            m.set_state(4, -1);
        }
        _ => {}
    }
    w.changed = true;
}

/// The moby `i` (−1 none: true) past its state 2, or gone.
fn past(w: &MobyWorld, i: i32) -> bool { i == -1 || w.mobys.get(i as usize).is_some_and(|m| 3 <= m.state) }
fn gone(w: &MobyWorld, i: i32) -> bool { i == -1 || w.mobys.get(i as usize).is_none_or(|m| matches!(m.state, 0xfe | 0xfd)) }

/// State 1: what wakes it (`+0x40d`, module docs); woken, its fence path, its collision, its update distance.
fn wake(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let mut ready = match m.pv_b(WAKE) {
        0 => {
            if ctx.game.rng.randi(7) == 0 {
                let m = &w.mobys[id];
                let range = m.pv_s(UD) as f32;
                let v = Volumes(m.pv_i(0x410), m.pv_i(0x418), m.pv_i(0x414), -1);
                find_target(w, id, ctx, T_BLOCK, range, 0.0, v) != 2
            } else {
                false
            }
        }
        1 => {
            let i = m.pv_i(0x400);
            if i != -1 {
                past(w, i)
            } else {
                w.mobys[id].set_pv_b(WAKE, 0);
                false
            }
        }
        2 => m.timer != 0,
        3 => gone(w, m.pv_i(MOUNT)),
        4 => gone(w, m.pv_i(0x400)),
        5 => {
            let i = m.pv_i(0x404);
            past(w, i) || gone(w, i)
        }
        _ => false,
    };
    if w.mobys[id].timer & 2 != 0 { ready = true; }
    if !ready { return; }
    let blob = w.class(CLASS).is_some_and(|c| c.has_collision);
    let t = ctx.game.rng.rand_range(0, 0xb4);
    let m = &mut w.mobys[id];
    m.set_pv_i(MV + 0x3c, m.pv_i(PATH));
    m.has_collision = blob;
    m.update_dist = m.pv_s(UD) as u8;
    m.set_pv_v(MV + 0xa0, [0.0; 3]);
    m.set_pv_i(TIMER, t);
    m.set_state(3, -1);
    w.changed = true;
}

/// 0x09c6 (module docs).
pub fn update_09c6(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.level.off_chunk(ctx.game.variant, w.mobys[id].pos[2]) { return; }
    if !head(w, id, ctx) { return; }
    match w.mobys[id].state {
        0 => {
            setup(w, id, ctx);
            if 0x80 <= w.mobys[id].state { return; }
        }
        1 => wake(w, id, ctx),
        3 => {
            if i_down(w, id, TIMER) {
                set_seq(w, id, ctx, 4, 0, 10);
                let m = &mut w.mobys[id];
                m.drawn = 1;
                m.mode &= !1;
                m.set_state(2, 0);
            }
        }
        2 => emerge(w, id, ctx),
        4 => launched(w, id, ctx),
        5 => {
            ai::hold(w, id, ctx);
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 9, 0, 2);
                w.mobys[id].set_state(6, 1);
            }
        }
        6 => calm(w, id, ctx),
        7 => hunt(w, id, ctx),
        8 => attack(w, id, ctx),
        9 => knock(w, id, ctx),
        10 => {
            w.mobys[id].set_pv_f(MV + 0x34, 0.1);
            let (pos, home) = (w.mobys[id].pos, w.mobys[id].pv_v(HOME));
            ai::move_step(w, id, ctx, atan2(home[0] - pos[0], home[1] - pos[1]), home);
            if w.mobys[id].pv_i(MV + 0x94) & 0x100 == 0 { w.mobys[id].set_state(7, 0); }
        }
        11 => {
            die(w, id, ctx);
            return;
        }
        _ => {}
    }
    tail(w, id, ctx);
    let p = w.mobys[id].pos;
    if p.iter().any(|&c| !(30.0..=993.0).contains(&c)) { w.delete(id); }
}

/// State 2: bursting out (sub 0) or sinking (sub 1), at its sequence's end.
fn emerge(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.host.frame_time(&w.mobys[id]) < 12.0 {
        let m = &w.mobys[id];
        let at = [m.pos[0], m.pos[1], m.pv_f(FLOOR)];
        tabora_ai::puffs(w, id, ctx, at, 3, 1.0, 1.5, 0.5, 0.0, 0.1, 0.8);
    }
    if !wrapped(w, id, ctx) { return; }
    match w.mobys[id].sub_state {
        0 => {
            let t = ctx.game.rng.rand_range(0xb4, 0x168);
            let m = &mut w.mobys[id];
            m.mode = (m.mode & !1) | 0x1000;
            m.drawn = 1;
            m.set_pv_i(TIMER, t);
            new_goal(w, id, ctx);
            set_seq(w, id, ctx, 9, 0, 10);
            w.mobys[id].set_state(6, 1);
        }
        1 => {
            w.mobys[id].mode &= !0x1000;
            set_seq(w, id, ctx, 11, 0, 10);
            let m = &mut w.mobys[id];
            m.set_pv_i(TIMER, 0);
            m.drawn = 0;
            m.mode |= 1;
            m.set_state(1, -1);
        }
        _ => {}
    }
}

/// `0x2ef2a8(out, v, n)`: `v` reflected off the face of normal `n` (unit).
fn bounce(v: V3, n: V3) -> V3 { sub(v, scale(n, 2.0 * dot(v, n))) }

/// State 4: launched (module docs).
fn launched(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let old = m.pos;
    m.pos = add(m.pos, m.pv_v(LAUNCH));
    m.set_pv_f(LAUNCH + 8, m.pv_f(LAUNCH + 8) - 0.002_722_222_4);
    m.pos[2] += 1.5;
    let to = m.pos;
    let passed = |w: &MobyWorld, k: Option<usize>| k.is_some_and(|i| matches!(w.mobys[i].o_class, 0x0945 | CLASS));
    if let Some(o) = w.coll_line(ctx, old, to, 0, Some(id)) {
        if !passed(w, o.moby) {
            let d = set_len(sub(old, o.point), 1.5);
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, d);
        }
    }
    let at = w.mobys[id].pos;
    let hit = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, at, 1.5, crate::collision::QueryFlags(0), Some(id)));
    if let Some(o) = hit {
        let landed = o.moby.is_none() && {
            let m = &w.mobys[id];
            m.pos[2] <= m.pv_f(FLOOR) + 1.5 + 0.1 && m.pv_f(LAUNCH + 8) < 0.0
        };
        if landed {
            let blob = w.class(CLASS).is_some_and(|c| c.has_collision);
            let t = ctx.game.rng.rand_range(0xb4, 0x168);
            let m = &mut w.mobys[id];
            m.mode = (m.mode | 0x1000) & !1;
            m.has_collision = blob;
            m.drawn = 1;
            m.set_pv_i(TIMER, t);
            new_goal(w, id, ctx);
            set_seq(w, id, ctx, 15, 0, 5);
            w.mobys[id].set_state(5, -1);
        } else if !passed(w, o.moby) {
            let n = set_len(o.normal, 1.0);
            let m = &mut w.mobys[id];
            if let Some(c) = o.pushed_centre { m.pos = c; }
            let v = scale(bounce(m.pv_v(LAUNCH), n), 0.8);
            m.set_pv_v(LAUNCH, v);
        }
    }
    let m = &mut w.mobys[id];
    m.pos[2] -= 1.5;
    m.set_pv_f(LAUNCH + 8, m.pv_f(LAUNCH + 8) - 0.002_722_222_4);
    if m.pos[2] < m.pv_f(FLOOR) { m.pos[2] = m.pv_f(FLOOR); }
    w.changed = true;
}

/// The calm walk's arguments.
fn calm_walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> i32 { walk(w, id, ctx, 120.0, 90.0, 8.0, 2.0, 1.0) }

/// State 6: calm (module docs).
fn calm(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].sub_state {
        0 => {
            ai::hold(w, id, ctx);
            if wrapped(w, id, ctx) {
                if w.mobys[id].pv_i(TIMER) == 0 {
                    let t = ctx.game.rng.rand_range(0xb4, 0x168);
                    w.mobys[id].set_pv_i(TIMER, t);
                    new_goal(w, id, ctx);
                    set_seq(w, id, ctx, 9, 0, 10);
                    w.mobys[id].set_state(6, 1);
                } else {
                    idle_seq(w, id, ctx, 0, 10);
                }
            }
        }
        1 => {
            if calm_walk(w, id, ctx) & 0x80 != 0 || w.mobys[id].pv_i(TIMER) == 0 {
                let t = ctx.game.rng.rand_range(0x1e, 0x5a);
                w.mobys[id].set_pv_i(TIMER, t);
                idle_seq(w, id, ctx, 0, 10);
                w.mobys[id].set_state(6, 0);
            }
        }
        2 => {
            let home = w.mobys[id].pv_v(HOME);
            w.mobys[id].set_pv_v(GOAL, home);
            if calm_walk(w, id, ctx) & 0x80 != 0 {
                set_seq(w, id, ctx, 5, 0, 10);
                w.mobys[id].set_state(2, 1);
            }
        }
        _ => {}
    }
    if ctx.game.rng.randi(7) != 0 { return; }
    if w.mobys[id].pv_i(ALARM) != 0 || search(w, id, ctx) != 2 {
        let path = w.mobys[id].pv_i(PATH);
        if path != -1 {
            let p = super::super::flyer::into_polygon(ctx, path, w.mobys[id].pv_v(T_BLOCK));
            w.mobys[id].set_pv_v(T_BLOCK, p);
        }
        if w.mobys[id].pv_b(START) == 3 {
            let m = &mut w.mobys[id];
            m.set_pv_i(CHASE, 600);
            m.set_pv_i(MV + 0x3c, m.pv_i(PATH));
            m.set_pv_b(START, 1);
            let a = m.pv_v(T_AIM);
            m.set_pv_v(GOAL, a);
            seq_rand_blend(w, id, ctx, 9);
            let m = &mut w.mobys[id];
            m.set_state(8, 0);
            m.set_pv_i(STUCK, 0);
            return;
        }
        if ctx.game.rng.randi(9) == 0 {
            let t = ctx.game.rng.rand_range(300, 0x168) * 2;
            let m = &mut w.mobys[id];
            m.set_pv_i(MV + 0x3c, m.pv_i(PATH));
            m.set_pv_i(ALARM, t);
            idle_seq(w, id, ctx, 2, 10);
            w.mobys[id].set_state(7, 0);
        }
    } else {
        let m = &w.mobys[id];
        if m.state == 6 && m.pv_b(START) == 0 && m.sub_state != 2 {
            set_seq(w, id, ctx, 9, 0, 10);
            w.mobys[id].set_state(6, 2);
        }
    }
}

/// State 7: hunting (module docs).
fn hunt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].sub_state {
        0 => {
            ai::hold(w, id, ctx);
            let a = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
            if wrapped(w, id, ctx) && a.wrapping_sub(2) < 2 { w.mobys[id].set_pv_i(TIMER, 0); }
            if w.mobys[id].pv_i(TIMER) == 0 {
                let t = ctx.game.rng.rand_range(0xb4, 0x168);
                w.mobys[id].set_pv_i(TIMER, t);
                set_seq(w, id, ctx, 9, 0, 10);
                w.mobys[id].set_state(7, 1);
            }
        }
        1 => {
            let moved = if w.mobys[id].pv_i(WAIT) != 0 {
                walk(w, id, ctx, 720.0, 720.0, 12.0, 4.0, 1.0)
            } else {
                walk(w, id, ctx, 180.0, 180.0, 12.0, 4.0, 1.0)
            };
            if moved & 0x80 != 0 { w.mobys[id].set_pv_i(TIMER, 0); }
            if w.mobys[id].pv_i(TIMER) == 0 {
                new_goal(w, id, ctx);
                let t = ctx.game.rng.rand_range(0x14, 0x3c);
                w.mobys[id].set_pv_i(TIMER, t);
                idle_seq(w, id, ctx, 0, 10);
                w.mobys[id].set_state(7, 0);
            }
        }
        _ => {}
    }
    if ctx.game.rng.randi(7) != 0 { return; }
    if w.mobys[id].pv_i(CHASE) == 0 {
        let result = search(w, id, ctx);
        if result == 2 || ctx.hero.state == 0x31 {
            w.mobys[id].set_pv_i(T_RESULT, 2);
            if ctx.game.rng.randi(4) == 0 {
                seq_rand_blend(w, id, ctx, 6);
                w.mobys[id].set_state(8, 1);
            }
            if w.mobys[id].pv_i(ALARM) == 0 {
                idle_seq(w, id, ctx, 0, 10);
                w.mobys[id].set_state(6, 0);
            }
            return;
        }
    }
    let m = &w.mobys[id];
    if m.pv_i(WAIT) == 0 && (dist2(m.pos, m.pv_v(T_AIM)) < 12.0 || m.pv_i(CHASE) != 0) {
        let a = m.pv_v(T_AIM);
        w.mobys[id].set_pv_v(GOAL, a);
        seq_rand_blend(w, id, ctx, 9);
        let m = &mut w.mobys[id];
        m.set_state(8, 0);
        m.set_pv_i(STUCK, 0);
    }
}

/// Ahead within 90°: the facing's difference to the yaw toward the target's aim.
fn ahead(w: &MobyWorld, id: usize) -> bool {
    let m = &w.mobys[id];
    let a = m.pv_v(T_AIM);
    rot_dist(m.rot[2], atan2(a[0] - m.pos[0], a[1] - m.pos[1])) < FRAC_PI_2
}

/// The run in's walk.
fn run_walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> i32 { walk(w, id, ctx, 360.0, 180.0, 16.0, 4.0, 0.5) }

/// State 8: the attack (module docs).
fn attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let aim = w.mobys[id].pv_v(T_AIM);
    w.mobys[id].set_pv_v(GOAL, aim);
    match w.mobys[id].sub_state {
        0 => {
            let mut blocked = false;
            if run_walk(w, id, ctx) != 0 {
                let m = &mut w.mobys[id];
                m.set_pv_i(STUCK, m.pv_i(STUCK) + 1);
            }
            if 60 < w.mobys[id].pv_i(STUCK) {
                if dist2(w.mobys[id].pos, aim) < 6.0 && ahead(w, id) {
                    seq_rand_blend(w, id, ctx, 6);
                    w.mobys[id].set_state(8, 1);
                } else {
                    w.mobys[id].set_pv_i(STUCK, 0);
                    blocked = true;
                }
            }
            if dist2(w.mobys[id].pos, aim) < 4.2 && ahead(w, id) {
                seq_rand_blend(w, id, ctx, 6);
                w.mobys[id].set_state(8, 1);
            }
            if w.mobys[id].pv_i(CHASE) == 0 && (blocked || search(w, id, ctx) == 2) {
                if w.mobys[id].pv_i(ALARM) == 0 {
                    w.mobys[id].set_pv_i(TIMER, 0);
                    idle_seq(w, id, ctx, 0, 10);
                    w.mobys[id].set_state(6, 0);
                } else {
                    run_walk(w, id, ctx);
                    let t = ctx.game.rng.rand_range(0x5a, 0xb4);
                    let m = &mut w.mobys[id];
                    m.set_pv_i(TIMER, 0);
                    m.set_pv_i(WAIT, t);
                    new_goal(w, id, ctx);
                    w.mobys[id].set_state(7, 0);
                }
                return;
            }
            if 0 < w.mobys[id].pv_i(CHASE) {
                let m = &mut w.mobys[id];
                m.set_pv_v(T_BLOCK, ctx.hero.pos);
                let mut a = ctx.hero.pos;
                a[2] += 0.4;
                m.set_pv_v(T_AIM, a);
            }
        }
        1 => {
            ai::hold(w, id, ctx);
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 6) && wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 7, 0, 5);
                w.mobys[id].set_state(8, 2);
            }
        }
        2 => {
            let m = &w.mobys[id];
            let at = ai::link(m, T_MOBY).and_then(|t| w.mobys.get(t)).map_or(m.pv_v(T_BLOCK), |t| t.pos);
            let yaw = atan2(at[0] - m.pos[0], at[1] - m.pos[1]);
            turn_in_place(w, id, ctx, yaw);
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 7) && 24.0 < ctx.host.frame_time(&w.mobys[id]) {
                breathe(w, id, ctx);
                if wrapped(w, id, ctx) {
                    set_seq(w, id, ctx, 8, 0, 10);
                    w.mobys[id].set_state(8, 3);
                }
            }
        }
        3 => {
            ai::hold(w, id, ctx);
            if wrapped(w, id, ctx) {
                let t = ctx.game.rng.rand_range(0x5a, 0xb4);
                let m = &mut w.mobys[id];
                for off in [CHASE, TIMER, ALARM, STUCK] { m.set_pv_i(off, 0); }
                m.set_pv_i(WAIT, t);
                m.set_state(7, 0);
                new_goal(w, id, ctx);
            }
        }
        _ => {}
    }
}

/// `0x3dabf0`: the breath: a sphere of 0.5 at joint 1 (its damage `+0x34`, flags 1), three lines 3 ahead of it along
/// its facing (to its target's height, ±1.2; spread 0.1 and −0.2 across its second row, ±0.2 up; a marker at the
/// first's end: frame 1, `0x00808080`, 0.5, 30 ticks), and two pairs of
/// staged puffs (smoke 0x40101080 → 0x10101080, sizes 0.6 → 1.5; flame 0x801010ff, 0.05 → 0.07), half additive.
fn breathe(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let damage = w.mobys[id].pv_b(0x34) as f32;
    ai::attack_joint(w, id, ctx, 0.5, damage, 1.0, 1, 1, 0, 1, 0);
    let Some(origin) = ctx.host.joint(&w.mobys[id], 1) else { return };
    let m = &w.mobys[id];
    let rows = m.rows;
    let fwd = [rows[0][0], rows[0][1], 0.0];
    let mut start = origin;
    let mut end = set_len(fwd, 3.0);
    if ai::link(m, T_MOBY).is_some() {
        let z = if matches!(ctx.hero.group, 0x11 | 0x12) { m.pv_f(T_BLOCK + 8) } else { m.pv_f(0x248) };
        end[2] = (z - start[2]).clamp(-1.2, 1.2);
    }
    end = add(end, start);
    // `0x347620(end, m, 1, 0x00808080, 0.5, 0x7f, 30, 0xff)`: a marker at the line's end (crate::particles::marker).
    let look = crate::particles::marker::Look { rgba: 0x0080_8080, size: 0.5, near: 0x7f, life: 30, alpha: 0xff };
    if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::marker::marker(sys, [end[0], end[1], end[2], 0.0], Some(id), 1, &look); }
    ai::attack_line(w, id, ctx, start, end, 1, damage, 1.0, 0);
    for k in [0.1, -0.2] {
        let step = set_len(rows[1], k);
        start = add(start, step);
        end = add(end, step);
        end[2] += ctx.game.rng.randf(-0.2, 0.2);
        ai::attack_line(w, id, ctx, start, end, 1, damage, 1.0, 0);
    }
    let heading = w.mobys[id].rot[2];
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let puff = |g: &mut crate::rng::Rng, sys: &mut crate::particles::Particles, jit: f32, spread: f32, speed: f32, rise: (f32, f32), ease: (f32, f32), size: (f32, f32), c: (u32, u32)| {
        let mut p = origin;
        for c in p.iter_mut() { *c += g.randf(-jit, jit); }
        let a = if spread != 0.0 { g.randf(-spread, spread) * 0.017_453_294 } else { 0.0 };
        let h = add_rot(heading, a);
        let mut v = [h.cos() * speed, h.sin() * speed, 0.0];
        v[2] = g.randf(0.0, 0.0);
        let b = g.randf(-90.0, 90.0) * 0.017_453_294;
        let hb = add_rot(heading, b);
        let mut acc = [hb.cos() * 0.008_333_334, hb.sin() * 0.008_333_334, 0.0];
        acc[2] = g.randf(rise.0, rise.1);
        let i = g.randf(2.0, 4.0) as i32 as i16;
        let e = g.randf(ease.0, ease.1) as i32 as i16;
        let o = g.randf(200.0, 240.000_02) as i32 as i16;
        if let Some(r) = crate::particles::staged(sys, g, p, [v[0], v[1], v[2], size.0], [acc[0], acc[1], acc[2], size.1], c.0, c.1, i, e, o, -1) {
            if g.rand() & 1 != 0 { sys.pool.recs[r][3] = 0x48; }
        }
    };
    for _ in 0..2 { puff(g, sys, 0.065, 0.0, 0.283_333_36, (0.015, 0.018_333_334), (12.0, 14.400_001), (0.6, 1.5), (0x4010_1080, 0x1010_1080)); }
    for _ in 0..2 { puff(g, sys, 0.2, 10.0, 0.2, (0.011_666_667, 0.025), (20.0, 24.0), (0.05, 0.07), (0x8010_10ff, 0x8010_10ff)); }
}

/// State 9: the knock (module docs).
fn knock(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_pv_i(KNOCK, m.pv_i(KNOCK) + 1);
    let res = ai::flight(w, id, ctx);
    if res & 1 != 0 {
        w.mobys[id].set_state(11, -1);
        return;
    }
    if res & 2 == 0 && w.mobys[id].pv_i(KNOCK) <= 360 { return; }
    let m = &mut w.mobys[id];
    m.set_pv_i(KNOCK, 0);
    m.set_pv_b(REACT + 0x1a, 2);
    m.set_pv_v(MV + 0xa0, [0.0; 3]);
    m.set_pv_i(TIMER, 0);
    if m.pv_f(HEALTH) <= 0.0 {
        m.set_state(11, -1);
    } else if !fence_ok(w, id, ctx) {
        set_seq(w, id, ctx, 9, 0, 10);
        w.mobys[id].set_state(10, -1);
    } else {
        if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 10); }
        w.mobys[id].set_state(7, 0);
    }
}

/// State 11: its death — the ground marks (`0x3d92c0`, not ported), the sparkles (`0x3d9410`, the same code as
/// 0x0fff's: crate::moby::classes::level08::tabora_ai::sparkles), the explosion (`0x32e558`: flashes 2 and 1 past 9, 4 embers, the colour shift 1, flags 0x810001), its bolts (flag
/// 0x1000 for one launched), its uid marked (the game's code past the drop's own marking); deleted.
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    tabora_ai::sparkles(w, id, ctx, [0.0; 3]);
    let e = Explosion { flash: 2.0, flash2: 1.0, near: 9.0, size: 1.0, embers: 4, sound: -1, lod: -1, colour: 1, flags: 0x81_0001, ..Default::default() };
    blast::explode(w, ctx, Some(id), None, &e);
    let flags = if w.mobys[id].pv_b(START) == 2 { 0x1000 } else { 0 };
    bolts::drop(w, id, flags, 3, ctx);
    let (uid, mission) = (w.mobys[id].uid, w.mobys[id].mission);
    ctx.game.uid_gone(uid, mission);
    ctx.game.bump_uid(uid);
    w.delete(id);
}
