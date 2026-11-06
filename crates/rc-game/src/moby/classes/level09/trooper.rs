//! Dobbo's trooper **0x0d0f** (`0x3fb510`, 43 placed; the decomp's `func_L09_003FA080`, shared with L20 `0x420938`):
//! legs that walk the level's paths carrying a turret 0x0d0e (`0x3f85e8`, only its shadow: [`update_0d0e`]) with two
//! guns 0x0d66 on it (crate::moby::classes::level12::debris, the same code: thrown off as it breaks), on the
//! framework ([`crate::moby::ai`]): health `+0x20` (its damage byte `+0x34`, `+0x2e` = 2 a kill asked by another),
//! reaction `+0x50`, movement `+0x100` (its rail speed `+0x128`, path points `+0x1c8` / `+0x1ca`, its fall-out word
//! `+0x1d8`), target `+0x1f0` (its moby `+0x200`, found `+0x204`). It is the Level 12 trooper's twin
//! (crate::moby::classes::level12::trooper: the same framework functions, `0x3f8878` = L12 `0x414178` …) with Dobbo's
//! own set-up, waking, hand-off and a flying mode.
//!
//! Its words: the turret's aim block `+0x210` (list 3), its own break pieces `+0x290`; paths `+0x2b0` (the way in,
//! −1 none), `+0x2b4` (patrol), `+0x2b8` (the second), `+0x2bc` (after the way in; the flight's lane), `+0x2c0`
//! (where a replacement comes in); `+0x2cc` the most of its group shooting at once, `+0x2d0` / `+0x2d4` its target
//! areas (`+0x2d8` the one searched), `+0x2dc` the path it is on; timers `+0x2e0` (alert), `+0x2e2`, `+0x2e4`
//! (between shots), `+0x2e6` (between bursts), `+0x2e8` (calm), `+0x300` (a reserve's wait); `+0x2ea` alarmed,
//! `+0x2eb` shooting (counted in its group's shared word `+0x304`), `+0x2ec` the turret's heading, the guns `+0x2f0`
//! / `+0x2f4` and turret `+0x2f8` (links), the turret's turn speed `+0x2fc`; `+0x302` 1 a reserve (hidden until one
//! of its group dies), 2 taking over; `+0x303` taking over a stand; `+0x308` a second group alarmed with it;
//! `+0x30c` / `+0x30d` the shot's height picks, `+0x30e` patrol after the way in, `+0x30f` its turret's head lost;
//! `+0x310` the reaction's `+0x40`; `+0x314` the area that wakes it (−1 none), `+0x31c` show (1) / hide (−1, −2 out
//! of the area) asked (also by the waker 0x0de0, crate::moby::classes::level09::waker), `+0x31d` hidden, `+0x31e`
//! its update distance, `+0x31f` a reserve's first tick; the guns' aim blocks `+0x320` / `+0x3a0` (lists 7 / 8);
//! `+0x420` its tier in the group (−1 none) with `+0x424` its kind and `+0x42c` the most of a tier, `+0x428` flying
//! (2), `+0x4b4` / `+0x4b8` the flight's turn and lane speeds, `+0x4bc` its shooting range (30 below 10), `+0x4c0`
//! a stand at the way in.
//!
//! **Each tick** ([`update_0d0f`]): nothing while `$gp+0x20b4` is set (never on Dobbo) or out of the loaded variant
//! (`0x31dee8` = crate::level::LevelData::in_variant); its waking ([`wake`], `0x3fb198`: hidden, nothing more); no
//! collision for the tick (`0x3f88f8`); its senses ([`senses`], `0x3f9f70`); gone, or in sub 0 before them: no more.
//! Then the sub-states ([`subs`]), the states ([`states`]), its turret ([`turret_update`], `0x3fa598`), guns
//! ([`guns_update`], `0x3fa8f8`), timers ([`timers`], `0x3f9d28`), its collision back (`0x3f8938`); fallen out
//! (`+0x1d8` ≠ −1) in states 1..13: its replacement, out of the shooters, dead (15, sub 10).
//!
//! The L09 engine functions with the port's: `0x324590` = L00 `0x327e38` (crate::moby::ai::take_damage), `0x3252d8`
//! = L00 `0x328b80` (ai::flight), `0x3275b8` = L00 `0x32ad38` (ai::find_target), `0x32a218` = L00 `0x32dcf0`
//! (ai::senses), `0x32a280` / `0x32a3d0` the path walk (ai::follow_path), `0x3292a8` = L00 `0x32ceb8` (ai::move_step
//! in place), `0x31b6a0` = L00 `0x31f230` (turn_ease), `0x318408` = L00 `0x31c1e0` (ai::turn_to toward a moby),
//! `0x31b0f8` = L00 `0x31ec88` (ai::spring_to), `0x3fc6b0` = L02 `0x3340d8` (ai::move_body), `0x3270c8` = L00
//! `0x32a848` (in_area), `0x319148` = L00 `0x31ce20` (ai::ground_z), `0x2e8770` = L02 `0x2f0170` the teleport-in
//! (crate::moby::classes::teleport), `0x326a38` = L00 `0x32a2e0` the break pieces ([`part_break`]), `0x32f7c0` = L00
//! `0x332c10` (crate::particles::trail), `0x3a1a10` = L12 `0x3a7fb8` the shot (crate::moby::classes::level12::shot),
//! `0x323ca8` = L12 `0x32ad40` (crate::moby::classes::level12::debris::spawn), `0x3f55b0` = L00 `0x3f8348`
//! (crate::moby::blast::burst), `0x316fe8` = L00 `0x31adc0` (crate::moby::bolts::drop), `0x319ec8` = L00 `0x31da58`
//! (crate::shadow_volumes::probe_near), `0x35d9a8` the class sound.
//!
//! Not ported: on Dobbo, the flag `$gp−0x5fb0` cleared when the moby that last hurt it (`+0xc0`) is neither Ratchet
//! nor in one of the states `0x3280e0` names (nothing the port has reads it; `+0xc0` is still let go); the colours
//! Tabora and L20 give it (`0x3fa4d0`), the path lengths written into the points' `w` (`0x33fa50`) and the debug
//! prints.
//!
//! Its aims (crate::moby::joint_aim): the turret's on `+0x210` and the guns' on `+0x320` / `+0x3a0` (`0x3206a0` = L00
//! `0x324208`, 0.02 / 0.3, on the turret's lists 3 and 7 / 8); in [`timers`] the head aim on `+0x430` (`0x323718` =
//! L00 `0x327308`, the turret's list 4, 1.9).

use crate::hero::physics::{add, add_rot, atan2, dist, dist2, dot, len2, rot_dist, scale, set_len, sub, sub_rot, turn_ease, V3};
use crate::moby::blast::{self, Blast};
use crate::moby::classes::level12::{debris, shot};
use crate::moby::classes::props::{seq, wrapped};
use crate::moby::{ai, countdown, mode, Ctx, MobyWorld};

const H: usize = 0x20;
const REACT: usize = 0x50;
const MV: usize = 0x100;
const SPEED: usize = MV + 0x28;
const AT: usize = MV + 0xc8;
const LAST: usize = MV + 0xca;
const FELL: usize = MV + 0xd8;
const T_POINT: usize = 0x1f0;
const T_MOBY: usize = 0x200;
const FOUND: usize = 0x204;
const AIM_TURRET: usize = 0x210;
const PARTS: usize = 0x290;
const P_IN: usize = 0x2b0;
const P_PATROL: usize = 0x2b4;
const P_SECOND: usize = 0x2b8;
const P_AFTER: usize = 0x2bc;
const P_RESPAWN: usize = 0x2c0;
const MOST: usize = 0x2cc;
const AREA_A: usize = 0x2d0;
const AREA_B: usize = 0x2d4;
const AREA: usize = 0x2d8;
const PATH: usize = 0x2dc;
const T_ALERT: usize = 0x2e0;
const T_2E2: usize = 0x2e2;
const T_SHOT: usize = 0x2e4;
const T_BURST: usize = 0x2e6;
const T_CALM: usize = 0x2e8;
const ALARMED: usize = 0x2ea;
const SHOOTING: usize = 0x2eb;
const HEADING: usize = 0x2ec;
const GUNS: [usize; 2] = [0x2f0, 0x2f4];
const TURRET: usize = 0x2f8;
const TURRET_V: usize = 0x2fc;
const T300: usize = 0x300;
const FLAGS: usize = 0x302;
const STAND: usize = 0x303;
const SHARED: usize = 0x304;
const PICK_C: usize = 0x30c;
const PICK_D: usize = 0x30d;
const PATROL_AFTER: usize = 0x30e;
const HEADLESS: usize = 0x30f;
const WAKE_AREA: usize = 0x314;
/// The show / hide ask (the waker writes it).
pub const ASK: usize = 0x31c;
const HIDDEN: usize = 0x31d;
/// Its update distance kept while the waker holds it (the waker writes it).
pub const UPD: usize = 0x31e;
const FIRST: usize = 0x31f;
const AIM_GUNS: [usize; 2] = [0x320, 0x3a0];
/// The head aim's record (crate::moby::joint_aim), hung on the turret.
const HEAD_AIM: usize = 0x430;
const TIER: usize = 0x420;
const KIND: usize = 0x424;
const FLYING: usize = 0x428;
const TIER_MOST: usize = 0x42c;
const FLY_TURN: usize = 0x4b4;
const FLY_LANE: usize = 0x4b8;
const RANGE: usize = 0x4bc;
const STAND_IN: usize = 0x4c0;
const PVARS: usize = 0x500;

pub const CLASS: i16 = 0x0d0f;
pub const TURRET_CLASS: i16 = 0x0d0e;
pub const GUN_CLASS: i16 = 0x0d66;
/// `$gp+0x2090`: the turret's break thresholds (by piece, of 255 health).
const THRESHOLDS: [u8; 10] = [0xe9, 0xd4, 0xaa, 0x7f, 1, 1, 0x55, 1, 1, 1];
/// `$gp+0x1ec8`: its and its turret's scale.
const SCALE: f32 = 1.3;
/// `$gp+0x1ecc` / `+0x1ed0` (sequence 0xc) and `+0x1ed4` / `+0x1ed8` (else): the speed's gain (a second²) and top (a
/// second); `+0x1edc` / `+0x1ee0` the turn's (degrees).
const LIMITS_C: (f32, f32) = (10.0, 2.0);
const LIMITS: (f32, f32) = (16.0, 6.0);
const TURN: (f32, f32) = (1440.0, 720.0);
/// `$gp+0x1ee4`: its gravity, a second².
const GRAVITY: f32 = 9.8;
/// `$gp+0x1ee8`: the shot's speed a second.
const SHOT_SPEED: f32 = 10.0;
/// `$gp+0x1eec`: near its target (a low shot; far: the shot's climb held).
const NEAR: f32 = 3.0;
/// `$gp+0x1ef0`: its shooting range when its own `+0x4bc` is under 10.
const RANGE_DEFAULT: f32 = 30.0;
/// `$gp+0x1ef4`: one try in this many.
const ODDS: i32 = 10;
/// `$gp+0x1ef8` / `+0x1efc`: the volley's end and first shot (ticks in state 11).
const VOLLEY_END: i32 = 90;
const VOLLEY_START: i32 = 30;
/// `$gp+0x1f00`: the ticks between shots (and the gun's half of them).
const SHOT_GAP: i32 = 10;
/// `$gp+0x1f04` / `+0x1f08`: the burst timer; `+0x1f0c` the calm timer; `+0x1f10` / `+0x1f14` / `+0x1f18` the alert
/// timer's low mark and range.
const BURST: (i32, i32) = (140, 220);
const CALM: i16 = 600;
const ALERT_LOW: i16 = 300;
const ALERT: (i32, i32) = (600, 1000);
/// `$gp+0x1f1c`: within this of Ratchet it aims at his centre.
const AIM_NEAR: f32 = 5.0;
/// `$gp+0x1f20`: the second path's walk's animation speed.
const WALK_SPEED: f32 = 0.7;

fn link(w: &MobyWorld, id: usize, off: usize, class: i16) -> Option<usize> {
    ai::link(&w.mobys[id], off).filter(|&k| w.mobys.get(k).is_some_and(|m| m.o_class == class && m.state < 0xfd))
}
fn turret(w: &MobyWorld, id: usize) -> Option<usize> { link(w, id, TURRET, TURRET_CLASS) }
fn gun(w: &MobyWorld, id: usize, k: usize) -> Option<usize> { link(w, id, GUNS[k], GUN_CLASS) }
fn st(w: &mut MobyWorld, id: usize, s: u8, sub: i32) { w.mobys[id].set_state(s, sub); }
fn rr(ctx: &mut Ctx, a: i32, b: i32) -> i32 { ctx.game.rng.rand_range(a, b) }
fn seq_a(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a) }
fn t_pos(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    ai::link(&w.mobys[id], T_MOBY).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos)
}
/// `0x2ec0f8`-style wrap: an angle into −π..π.
fn wrap(a: f32) -> f32 { ((a + std::f32::consts::PI) * 0.159_154_94).fract().rem_euclid(1.0) * std::f32::consts::TAU - std::f32::consts::PI }
fn found(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_i(FOUND) == 1 }
fn tick_s(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut t = w.mobys[id].pv_s(off) as i32;
    let out = countdown(&mut t);
    w.mobys[id].set_pv_s(off, t as i16);
    out
}
fn path_pts<'a>(ctx: &'a Ctx, path: i32) -> Option<&'a Vec<[f32; 4]>> { usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| !p.is_empty()) }
fn pt(p: &[[f32; 4]], i: i32) -> V3 { let q = p[(i.max(0) as usize).min(p.len() - 1)]; [q[0], q[1], q[2]] }
/// On the way in (`+0x2dc` its first path).
fn on_way_in(w: &MobyWorld, id: usize) -> bool { let m = &w.mobys[id]; m.pv_i(PATH) != -1 && m.pv_i(PATH) == m.pv_i(P_IN) }
fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 { let q = w.mobys[id].pos; atan2(p[0] - q[0], p[1] - q[1]) }

/// `0x3f89a8(m, seq, frame, blend)`: the limits ([`LIMITS_C`] for sequence 0xc, else [`LIMITS`]; [`TURN`]), the brake
/// doubled; the reaction's sequences and speeds for the falls 0xd..0xf and the jump 0x10; the sequence on the legs —
/// flying or on its way in, 0xe for any but 0xf — and on the turret when they have another.
fn anim(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, s: u8, frame: u8, blend: i32) {
    let (a, b) = if s == 0xc { LIMITS_C } else { LIMITS };
    let m = &mut w.mobys[id];
    ai::senses(m, a * 0.000_277_777_78, b * 0.016_666_668, TURN.0 * 4.848_137_5e-6, TURN.1 * 2.908_882_5e-4);
    m.set_pv_f(MV + 0x18, m.pv_f(MV + 0x18) * 2.0);
    if (0xd..=0xf).contains(&s) {
        for (k, v) in [(0x66, 0xf), (0xc2, 7), (0xc3, 0x13), (0xc6, 1), (0x61, 0xf), (0x62, 0xf), (0xbc, 7), (0xbd, 0x13), (0xbe, 7), (0xbf, 0x13)] { m.set_pv_b(k, v); }
        for k in [0x6c, 0x70, 0x74, 0x78, 0x7c] { m.set_pv_f(k, 0.0); }
    } else if s == 0x10 {
        m.set_pv_f(0x6c, f32::from_bits(0x3ebb_bbbc));
        m.set_pv_f(0x70, f32::from_bits(0x3dee_eef0));
        m.set_pv_f(0x78, f32::from_bits(0x3c08_8889));
        m.set_pv_f(0x7c, f32::from_bits(0x3b88_8889));
        m.set_pv_f(0x74, f32::from_bits(0x3b88_8889));
        for (k, v) in [(0x61, 0x12), (0x66, 0x14), (0xbc, 5), (0xbd, 9), (0xbe, 4), (0xbf, 7), (0xc2, 10), (0xc3, 0x11), (0x62, 0x11), (0xc6, 0)] { m.set_pv_b(k, v); }
    }
    let b = blend.clamp(0, 255) as u8;
    let forced = (w.mobys[id].pv_i(FLYING) != 0 || on_way_in(w, id)) && s != 0xf;
    let own = if forced { 0xe } else { s };
    if seq(w, id, ctx) != own { ctx.host.set_anim(&w.mobys[id], own, frame, b); }
    if let Some(t) = ai::link(&w.mobys[id], TURRET).filter(|&t| t < w.mobys.len()) {
        if seq(w, t, ctx) != s { ctx.host.set_anim(&w.mobys[t], s, frame, b); }
    }
}
fn anim_r(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, s: u8) { let b = rr(ctx, 7, 0xc); anim(w, id, ctx, s, 0, b); }
fn walk_r(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = (ctx.game.rng.rand() & 1) as u8;
    let b = rr(ctx, 10, 0x14);
    anim(w, id, ctx, s, 0, b);
}
/// The walk along its path (0xb) or about (0).
fn walk_on(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { let s = if w.mobys[id].pv_i(PATH) != -1 { 0xb } else { 0 }; anim_r(w, id, ctx, s); }
fn alert_timer(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { let t = rr(ctx, ALERT.0, ALERT.1); w.mobys[id].set_pv_s(T_ALERT, t as i16); }

/// `0x3f8878`: out of its group's shooters (the shared count down, when it was in).
fn release(w: &mut MobyWorld, id: usize) {
    let m = &w.mobys[id];
    if m.group >= 0 && m.pv_b(SHOOTING) != 0 {
        let off = m.pv_i(SHARED);
        let mut v = w.shared_i(off);
        countdown(&mut v);
        w.set_shared_i(off, v);
    }
    w.mobys[id].set_pv_b(SHOOTING, 0);
}
/// `0x3f88c0`: one of its group's shooters.
fn take_slot(w: &mut MobyWorld, id: usize) {
    let m = &w.mobys[id];
    if m.group >= 0 && m.pv_b(SHOOTING) == 0 {
        let off = m.pv_i(SHARED);
        let v = w.shared_i(off) + 1;
        w.set_shared_i(off, v);
        w.mobys[id].set_pv_b(SHOOTING, 1);
    }
}

/// `0x3f90e0(m, path, last)`: onto `path` (−1: off any, its point −1) from its nearest point, to `last` (−1 its end).
fn set_path(w: &mut MobyWorld, id: usize, ctx: &Ctx, path: i32, last: i32) {
    let m = &mut w.mobys[id];
    m.set_pv_i(PATH, path);
    if path == -1 { m.set_pv_s(AT, -1); return; }
    let Some(p) = path_pts(ctx, path) else { m.set_pv_s(AT, 0); return };
    let pos = m.pos;
    let near = (0..p.len()).fold((0usize, 1e11f32), |(bi, bd), i| { let d = dist(pt(p, i as i32), pos).abs(); if bd <= d { (bi, bd) } else { (i, d) } }).0;
    m.set_pv_i(MV + 0xc4, path + 1);
    m.set_pv_s(LAST, if last == -1 { p.len() as i16 - 1 } else { last as i16 });
    m.set_pv_s(AT, near as i16);
}

/// The place moved along the path (`0x347fa0`'s twin; crate::moby::classes::level12::trooper's): onto the line from
/// point `i` toward the next in `dir`, `d` past its foot on it; past that segment, onto the next.
fn slide(w: &mut MobyWorld, id: usize, p: &[[f32; 4]], d: f32, i: i32, dir: i32) -> i32 {
    let n = p.len() as i32;
    let next = |i: i32| if i == n - 1 { if dir < 1 { n - 2 } else { 0 } } else if i == 0 { if dir < 1 { n - 1 } else { 1 } } else { i + dir };
    let mut i = i;
    let j = next(i);
    let seg = sub(pt(p, j), pt(p, i));
    let l = crate::hero::physics::len(seg);
    let mut u = set_len(seg, 1.0);
    let mut t = dot(sub(w.mobys[id].pos, pt(p, i)), u) + d;
    if l < t {
        let k = next(j);
        u = set_len(sub(pt(p, k), pt(p, j)), 1.0);
        t -= l;
        i = j;
    }
    w.mobys[id].pos = add(pt(p, i), scale(u, t));
    i
}

/// `0x3f8fb8(m, path)` (= L12 `0x414848`): along the path at its rail speed toward its last point, facing its next
/// point (at the end the target's point), eased. True at the last point (or off any path).
fn rail(w: &mut MobyWorld, id: usize, ctx: &Ctx, path: i32) -> bool {
    let Some(p) = path_pts(ctx, path).cloned() else { w.mobys[id].set_pv_s(AT, -1); return true };
    let m = &w.mobys[id];
    let (at, last) = (m.pv_s(AT) as i32, m.pv_s(LAST) as i32);
    let dir = if at < last { 1 } else { -1 };
    let speed = m.pv_f(SPEED);
    let i = slide(w, id, &p, speed, at, dir);
    let m = &mut w.mobys[id];
    m.set_pv_s(AT, i as i16);
    let done = i == last;
    let to = if i + 1 <= p.len() as i32 - 1 { pt(&p, i + 1) } else { m.pv_v(T_POINT) };
    let yaw = atan2(to[0] - m.pos[0], to[1] - m.pos[1]);
    let (a, b, c) = (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c));
    let (mut r, mut v) = (m.rot[2], m.pv_f(MV + 0xb4));
    turn_ease(yaw, a, b, c, &mut r, &mut v, 0);
    m.rot[2] = r;
    m.set_pv_f(MV + 0xb4, v);
    done
}

/// `0x3fadb8` (= L12 `0x4162c8`): its walk turned back: toward its path's end from the start (true), else the start.
fn reverse(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &mut w.mobys[id];
    if m.pv_s(LAST) == 0 {
        let n = path_pts(ctx, m.pv_i(PATH)).map_or(-1, |p| p.len() as i32 - 1);
        m.set_pv_s(LAST, n as i16);
        true
    } else {
        m.set_pv_s(LAST, 0);
        false
    }
}

/// `0x3f8f18(m, path)` (= L12 `0x4147a8`): the framework's walk along the path, or within its stop distance
/// (`+0x134`) of its last point.
fn walk_done(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, path: i32) -> bool {
    let r = ai::follow_path(w, id, ctx, path);
    r || path_pts(ctx, path).is_some_and(|p| {
        let m = &w.mobys[id];
        dist2(m.pos, pt(p, m.pv_s(LAST) as i32)) < m.pv_f(MV + 0x34)
    })
}

/// `0x3fa9f8(m, aim, from_end, low)`: a clear shot — its head lost, always; else the line from 1.76 above it (`low`:
/// 1.0; `from_end`: above its path's last point) to 1.18 above its target (flags 0) meets nothing or the target; with
/// `aim`, a 0.5 sphere where the line stopped (flags 1) meeting the world or a moby not of the kind 5.
fn can_shoot(w: &MobyWorld, id: usize, ctx: &Ctx, aim: bool, from_end: bool, low: bool) -> bool {
    let m = &w.mobys[id];
    if m.pv_b(HEADLESS) != 0 { return true; }
    let mut from = m.pos;
    if from_end && m.pv_i(PATH) != -1 && m.pv_s(LAST) != -1 {
        if let Some(p) = path_pts(ctx, m.pv_i(PATH)) { from = pt(p, m.pv_s(LAST) as i32); }
    }
    from[2] += if low { 1.0 } else { 1.76 };
    let t = ai::link(m, T_MOBY);
    let mut to = t_pos(w, id, ctx);
    to[2] += 1.18;
    let Some(c) = ctx.coll else { return true };
    let live = |k: usize| w.mobys.get(k).is_some_and(|m| m.has_collision && m.state < 0xfd);
    let line = crate::collision::coll_line_live(c, ctx.scene, from, to, crate::collision::QueryFlags(0), Some(id), &live);
    let mut clear = line.as_ref().is_none_or(|o| o.moby.is_some() && o.moby == t);
    if aim && !clear {
        let at = line.as_ref().map_or(to, |o| o.point);
        if let Some(o) = crate::collision::coll_sphere_live(c, ctx.scene, at, 0.5, crate::collision::QueryFlags(1), Some(id), &live) {
            let kind = o.moby.and_then(|k| w.mobys.get(k)).and_then(|k| w.class(k.o_class)).map(|c| c.kind);
            if kind != Some(5) { clear = true; }
        }
    }
    clear
}

/// `0x3fab98`: a shot when the timer between them is out: from the gun of the half of [`SHOT_GAP`] ticks it is in
/// (its piece 4 / 5 still on the turret), with the turret's class sound 1, at [`SHOT_SPEED`] toward the target's point
/// (beyond [`NEAR`] and not low: at most 2 a second up or down), its damage byte.
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let far = NEAR <= dist2(w.mobys[id].pos, t_pos(w, id, ctx));
    let Some(t) = turret(w, id) else { return };
    if !tick_s(w, id, T_SHOT) { return; }
    let k = ((w.mobys[id].state_ticks as u16 as i32) / SHOT_GAP & 1) as usize;
    if (w.mobys[t].bangles >> (4 + k)) & 1 == 0 {
        w.mobys[id].set_pv_s(T_SHOT, SHOT_GAP as i16);
        return;
    }
    let low = w.mobys[id].pv_b(PICK_D) != 0;
    if !can_shoot(w, id, ctx, true, false, low) { return; }
    let Some(g) = gun(w, id, k) else { return };
    w.mobys[id].set_pv_s(T_SHOT, SHOT_GAP as i16);
    let Some(p) = ctx.host.joint(&w.mobys[g], 0) else { return };
    let speed = SHOT_SPEED * 0.016_666_668;
    let mut v = set_len(sub(w.mobys[id].pv_v(T_POINT), p), speed);
    if far && !low { v[2] = v[2].clamp(-0.033_333_335, 0.033_333_335); }
    let v = set_len(v, speed);
    ctx.host.play_sound(&w.mobys[t], 1, 0);
    let dmg = w.mobys[id].pv_b(0x34) as f32;
    shot::fire(w, ctx, dmg, id, v, p);
}

/// `0x3f9ce8` (= L12 `0x415380`): every live trooper of its group and of its second group alarmed (`+0x2ea`).
fn alarm(w: &mut MobyWorld, id: usize) {
    for g in [w.mobys[id].group as i32, w.mobys[id].pv_i(0x308)] {
        let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { continue };
        for k in list {
            if let Some(m) = w.mobys.get_mut(k).filter(|m| m.state < 0x80 && m.o_class == CLASS) { m.set_pv_b(ALARMED, 1); }
        }
    }
}

/// The troopers of its moby group.
fn group(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).map_or(Vec::new(), |l| l.iter().copied().filter(|&k| w.mobys.get(k).is_some_and(|m| m.o_class == CLASS)).collect())
}
/// `0x3f8658(m, tier)`: its group's troopers not in reserve at `tier` (−1 for tier 0).
fn at_tier(w: &MobyWorld, id: usize, tier: i32) -> i32 {
    if tier == 0 { return -1; }
    group(w, id).into_iter().filter(|&k| { let m = &w.mobys[k]; m.pv_b(FLAGS) & 1 == 0 && m.pv_i(TIER) == tier }).count() as i32
}
/// `0x3f8738(m)`: the highest tier of its group's troopers neither in reserve nor taking over.
fn top_tier(w: &MobyWorld, id: usize) -> i32 {
    group(w, id).into_iter().filter(|&k| w.mobys[k].pv_b(FLAGS) == 0).map(|k| w.mobys[k].pv_i(TIER)).fold(0, i32::max)
}

/// `0x3f9a28`: its replacement — with a replacement path (`+0x2c0`), the reserve of its group nearest (across) that
/// path's first point; with a tier (`+0x420`), only one of the next tier above its group's top whose tier is not yet
/// full (`+0x42c`); with a kind (`+0x424`), only of its kind. It takes its paths and areas, the alert timer 1000, the
/// replacement path as its path, taking over (2), alarmed, its stand flag; updated again and asked to show.
fn hand_off(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let Some(start) = path_pts(ctx, m.pv_i(P_RESPAWN)).map(|p| pt(p, 0)) else { return };
    let (tier, kind) = (m.pv_i(TIER), m.pv_i(KIND));
    let mut best: Option<(usize, f32)> = None;
    for k in group(w, id) {
        let o = &w.mobys[k];
        if o.pv_b(FLAGS) & 1 == 0 { continue; }
        if tier != -1 {
            let next = top_tier(w, id) + 1;
            if next != o.pv_i(TIER) || o.pv_i(TIER_MOST) <= at_tier(w, k, next) { continue; }
        }
        if kind != 0 && kind != o.pv_i(KIND) { continue; }
        let d = dist2(o.pos, start);
        if best.is_none_or(|(_, b)| d < b) { best = Some((k, d)); }
    }
    let Some((k, _)) = best else { return };
    let src: Vec<i32> = [P_IN, P_PATROL, P_SECOND, P_AFTER, P_RESPAWN, AREA_A, AREA_B].iter().map(|&o| m.pv_i(o)).collect();
    let (respawn, stand) = (m.pv_i(P_RESPAWN), m.pv_b(STAND));
    let o = &mut w.mobys[k];
    for (off, v) in [P_IN, P_PATROL, P_SECOND, P_AFTER, P_RESPAWN, AREA_A, AREA_B].into_iter().zip(src.iter().copied()) { o.set_pv_i(off, v); }
    o.set_pv_s(T_ALERT, ALERT.1 as i16);
    o.set_pv_b(FLAGS, (o.pv_b(FLAGS) & !1) | 2);
    o.set_pv_i(PATH, respawn);
    o.set_pv_b(ALARMED, 1);
    o.set_pv_b(STAND, stand);
    o.mode &= !mode::NO_UPDATE;
    o.set_pv_b(ASK, 1);
    o.set_pv_b(HIDDEN, 1);
}

/// `0x326a38(m, b, health)` (= L00 `0x32a2e0`, as crate::moby::classes::level12::trooper's): the break pieces of `b`
/// (`+0..` a threshold of 255 health by piece, `+0x10` the health last seen).
fn part_break(w: &mut MobyWorld, ctx: &mut Ctx, id: usize, b: usize, h: usize) -> bool {
    let m = &w.mobys[id];
    let hp = m.pv_f(h);
    let most = m.pv_s(h + 4).max(1) as f32;
    let byte = |hp: f32| (hp.max(0.0) / most * 255.0) as i32 as u8;
    if m.pv_f(b + 0x10) == 0.0 && hp != 0.0 {
        let n = w.class(m.o_class).map_or(0, |c| c.pieces.len()).min(15);
        let mut bits = m.bangles;
        for k in 0..n {
            let t = m.pv_b(b + k);
            if t != 0 && t < byte(hp) { bits |= 1 << k; }
        }
        w.mobys[id].bangles = bits;
    }
    let mut broke = false;
    if hp < w.mobys[id].pv_f(b + 0x10) {
        for k in 0..15u8 {
            let m = &w.mobys[id];
            if m.bangles & (1 << k) == 0 || m.pv_b(b + k as usize) <= byte(hp) { continue; }
            let cam = ctx.host.camera_rows()[0];
            let rng = &mut ctx.game.rng;
            let a = add_rot(rng.randf_signed(90.0, 120.0) * 0.017_453_294, atan2(cam[0], cam[1]));
            let s = rng.randf(2.0, 4.0) * 0.016_666_668;
            let up = rng.randf(5.0, 8.0) * 0.016_666_668;
            let (class, pos, rot) = (m.o_class, m.pos, m.rot);
            crate::moby::shards::spawn(w, ctx, class, Some(id), pos, rot, None, 90.0, 15.0, k, [a.cos() * s, a.sin() * s, up], 0xff, 1);
            w.mobys[id].bangles &= !(1 << k);
            if !broke { ctx.host.play_level_sound(None, 7, 0); }
            broke = true;
        }
    }
    w.mobys[id].set_pv_f(b + 0x10, hp);
    broke
}

/// `0x3f8c60`: the turret and guns gone.
fn drop_parts(w: &mut MobyWorld, id: usize) {
    for off in [TURRET, GUNS[0], GUNS[1]] {
        if let Some(k) = ai::link(&w.mobys[id], off) {
            if w.mobys.get(k).is_some_and(|m| m.state < 0xfd) { w.delete(k); }
            ai::set_link(&mut w.mobys[id], off, None);
        }
    }
}

/// `0x3f8800(m, up)`: the blast `up` over it (`0x3f55b0`: size 0.5, sprites 0x19 / 0x14 / 0x14, 0x14 puffs, 0x2d
/// streaks, its flash ball, no shake).
fn blast(w: &mut MobyWorld, ctx: &mut Ctx, id: usize, up: f32) {
    let at = add(w.mobys[id].pos, [0.0, 0.0, up]);
    blast::burst(w, ctx, at, &Blast { size: 0.5, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
}

/// Its collision as its class's (`+0x98` = class `+0x10`) or none.
fn collide(w: &mut MobyWorld, k: usize, on: bool) {
    let has = on && w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision);
    w.mobys[k].has_collision = has;
}

/// Shown (drawn, mode &= !1) or hidden; with `coll` its collision too (and shown, a target again).
fn show(w: &mut MobyWorld, k: usize, on: bool, coll: bool) {
    let m = &mut w.mobys[k];
    if on { m.mode &= !1; m.drawn = 1; } else { m.mode |= 1; m.drawn = 0; }
    if coll {
        if on { m.mode |= mode::TARGET; }
        collide(w, k, on);
    }
}

/// `0x3f9170`: the set-up — its scale × [`SCALE`]; the reaction (sequences 0xe / 0x12 / 0x11 / 0x14, keys, gravities,
/// speed 0.45, `+0x90` its `+0x310`); the health block (`+0x28` 3, `+0x2a` 2, `+0x2e` 1, `+0x2f` 8, `+0x30` 1.7,
/// `+0x48` 25, `+0x4a` 16); movement (radius 0.7, height 1, steps 0.5, slope 45°, turn first 20°, gravity
/// [`GRAVITY`], options | 0x80); its limits; its turret ([`make_turret`]); a reserve hidden from the start (its first
/// tick asked); its shooting range at least 10 ([`RANGE_DEFAULT`]); onto its patrol path, out of the shooters, its
/// group's count 0; with a way in: onto it (1, sub 1; options | 4), on the ground the stagger 0xd, else the fall 2;
/// without, 5 (sub 1) with one of sequences 0 / 1; the area searched `+0x2d0`.
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
    m.scale *= SCALE;
    for (k, v) in [(0x6c, 0x3ebb_bbbcu32), (0x70, 0x3dee_eef0), (0x78, 0x3c08_8889), (0x7c, 0x3b88_8889), (0x8c, 0x3ecc_cccd), (0xf8, 0x3ee6_6666), (0x30, 0x3fd9_999a), (0x74, 0x3b88_8889), (0x80, 0x3f33_3333), (0x84, 0x3f33_3333)] { m.set_pv_f(k, f32::from_bits(v)); }
    for (k, v) in [(0x60, 0xeu8), (0xbc, 5), (0xbd, 9), (0xbf, 7), (0xc2, 10), (0xc3, 0x11), (0x61, 0x12), (0x66, 0x14), (0x67, 0xff), (0x2f, 8), (0x62, 0x11), (0x28, 3), (0xbe, 4), (0x2a, 2), (0x4d, 0), (0x2e, 1), (0x48, 25), (0x4a, 16), (0x1e8, 0xc), (0x1e9, 0xb)] { m.set_pv_b(k, v); }
    m.set_pv_f(0x90, m.pv_f(0x310));
    m.set_pv_i(0x98, 9);
    for off in [TURRET, GUNS[0], GUNS[1]] { m.set_pv_i(off, 0); }
    for (k, v) in [(0x100, 0x3f33_3333u32), (0x10c, 0x3f80_0000), (0x108, 0x3f00_0000), (0x110, 0x3f49_0fdc), (0x130, 0x3eb2_b8c4), (0x104, 0x3f00_0000)] { m.set_pv_f(k, f32::from_bits(v)); }
    m.set_pv_f(0x12c, GRAVITY * 0.000_277_777_78);
    ai::senses(m, LIMITS_C.0 * 0.000_277_777_78, LIMITS_C.1 * 0.016_666_668, TURN.0 * 4.848_137_5e-6, TURN.1 * 2.908_882_5e-4);
    m.set_pv_i(0x1dc, 0);
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x80);
    m.set_pv_f(0x1d4, 3.0);
    m.set_pv_s(0x1e4, 0);
    m.set_pv_s(0x1e2, 0);
    m.set_pv_i(0x1d0, 0);
    make_turret(w, id, ctx);
    let f = w.mobys[id].pv_b(FLAGS);
    if f & 1 != 0 {
        w.mobys[id].set_pv_b(FLAGS, (f & 0xfd) | 1);
        if w.mobys[id].pv_b(FIRST) != 0 {
            let m = &mut w.mobys[id];
            m.drawn = 0;
            m.has_collision = false;
            m.mode = (m.mode & !mode::TARGET) | 1;
            if let Some(t) = turret(w, id) {
                let tm = &mut w.mobys[t];
                tm.mode = (tm.mode | 1) & !mode::TARGET;
                tm.drawn = 0;
                tm.has_collision = false;
            }
            for k in 0..2 { if let Some(g) = gun(w, id, k) { show(w, g, false, false); } }
            let m = &mut w.mobys[id];
            m.set_pv_b(ASK, 0);
            m.set_pv_b(HIDDEN, 1);
            m.mode |= mode::NO_UPDATE;
        }
    }
    let m = &mut w.mobys[id];
    if m.pv_f(RANGE) < 10.0 { m.set_pv_f(RANGE, RANGE_DEFAULT); }
    let p = m.pv_i(P_PATROL);
    set_path(w, id, ctx, p, -1);
    release(w, id);
    let off = w.mobys[id].pv_i(SHARED);
    w.set_shared_i(off, 0);
    if w.mobys[id].pv_i(P_IN) == -1 {
        st(w, id, 5, 1);
        walk_r(w, id, ctx);
    } else {
        let m = &mut w.mobys[id];
        m.set_pv_i(PATH, m.pv_i(P_IN));
        m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 4);
        st(w, id, 1, 1);
        let m = &w.mobys[id];
        let g = ai::ground_z(ctx, 0.5, m.pos, 0);
        if m.pos[2] - m.pv_f(0x10c) - 0.1 <= g {
            let b = rr(ctx, 10, 0x14);
            anim(w, id, ctx, 0xd, 0, b);
        } else {
            let b = rr(ctx, 5, 10);
            anim(w, id, ctx, 2, 0, b);
        }
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(AREA, m.pv_i(AREA_A));
}

/// The turret 0x0d0e (its distances, colour words and glow, drawn, scaled × [`SCALE`], on its joint 0 with that
/// joint's rows; its health block, `+0x30` 0.7, `+0x2a` 2, `+0x2e` 1, `+0x2f` 4, `+0x4a` 3, `+0x4d` 1, its owner
/// `+0x70`; its break thresholds, one pair of them by its health at random) with the guns at its joints 0 and 1
/// (0x0d66, not updated, its draw distance).
fn make_turret(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let t = w.create(TURRET_CLASS);
    ai::set_link(&mut w.mobys[id], TURRET, Some(t));
    let (draw, upd, light, ambient, glow, rot) = { let m = &w.mobys[id]; (m.draw_dist, m.update_dist, m.light, m.ambient, m.glow, m.rot) };
    let at = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let frame = ctx.host.joint_matrix(&w.mobys[id], 0);
    let health: Vec<u8> = w.mobys[id].pvars[H..H + 0x30].to_vec();
    let tm = &mut w.mobys[t];
    tm.draw_dist = draw;
    tm.update_dist = upd;
    tm.light = light;
    tm.ambient = ambient;
    tm.glow = glow;
    tm.drawn = 1;
    tm.scale *= SCALE;
    tm.pos = at;
    tm.rot = rot;
    if let Some((r, _)) = frame {
        tm.rows = r.map(|row| set_len(row, 1.0));
        tm.rot = crate::moby::attach::angles_of(&tm.rows);
    }
    if tm.pvars.len() < H + 0x60 { tm.pvars.resize(H + 0x60, 0); }
    tm.pvars[H..H + 0x30].copy_from_slice(&health);
    tm.set_pv_f(0x30, f32::from_bits(0x3f33_3333));
    for (k, v) in [(0x2f, 4u8), (0x2a, 2), (0x2e, 1), (0x4a, 3), (0x4d, 1)] { tm.set_pv_b(k, v); }
    ai::set_link(tm, 0x70, Some(id));
    w.rebuild(t, &*ctx.host);
    for k in 0..2 {
        let p = ctx.host.joint(&w.mobys[t], k).unwrap_or(at);
        let off = sub(p, w.mobys[t].pos);
        let g = debris::spawn(w, ctx, t, GUN_CLASS, off, [0.0; 3], 0, 0.0, Some([0.0; 3]), Some([0.0; 3]));
        let gm = &mut w.mobys[g];
        gm.draw_dist = draw;
        gm.mode |= mode::NO_UPDATE;
        ai::set_link(&mut w.mobys[id], GUNS[k], Some(g));
    }
    let tm = &mut w.mobys[t];
    for (k, v) in THRESHOLDS.iter().enumerate() { tm.set_pv_b(0x50 + k, *v); }
    let most = w.mobys[id].pv_s(0x24).max(1) as f32;
    let (two, one) = (((2.0 / most) * 255.0) as i32 as u8, ((1.0 / most) * 255.0) as i32 as u8);
    let tm = &mut w.mobys[t];
    match ctx.game.rng.rand() % 3 {
        0 => { tm.set_pv_b(0x54, two); tm.set_pv_b(0x58, one); }
        1 => { tm.set_pv_b(0x55, two); tm.set_pv_b(0x59, one); }
        _ => { tm.set_pv_b(0x57, two); }
    }
    part_break(w, ctx, t, 0x50, 0x20);
    w.mobys[t].bangles &= 0xfcff;
}

/// `0x3fb198`: its waking. Its kept update distance taken back while unheld. With a waking area, in sub 0: Ratchet
/// out of it asks a hide (−2) unless hidden — hidden, a hide asked is let go; in it, a show (1) unless a reserve's
/// first tick or one is asked. Taking over: its tier full of others (`0x3f8658`) holds it (no ask); else a show once
/// the wait `+0x300` is out. A hide: hidden (its collision, drawing, target flag, its turret's and guns'), nothing
/// more. Hidden without a show: nothing more. A show while hidden: shown (with its class's collision), the
/// teleport-in at its place with its guns and turret (`0x2e8770`). True when nothing more runs this tick.
fn wake(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let m = &mut w.mobys[id];
    if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
    if m.pv_b(UPD) != 0 && m.update_dist == 0xff { m.update_dist = m.pv_b(UPD); }
    if m.pv_i(WAKE_AREA) != -1 && m.sub_state == 0 {
        if !ctx.level.in_area(m.pv_i(WAKE_AREA), ctx.hero.pos) {
            if m.pv_b(HIDDEN) == 0 { m.set_pv_b(ASK, 0xfe); } else if m.pv_b(ASK) as i8 == -2 { m.set_pv_b(ASK, 0); }
        } else if !(m.pv_b(FIRST) != 0 && m.pv_b(FLAGS) & 1 != 0) && m.pv_b(ASK) == 0 {
            m.set_pv_b(ASK, 1);
        }
    }
    if w.mobys[id].pv_b(FLAGS) & 2 != 0 {
        let tier = w.mobys[id].pv_i(TIER);
        if at_tier(w, id, tier) < w.mobys[id].pv_i(TIER_MOST) {
            w.mobys[id].set_pv_b(ASK, 0);
            return true;
        }
        let out = tick_s(w, id, T300);
        w.mobys[id].set_pv_b(ASK, out as u8);
    }
    let ask = w.mobys[id].pv_b(ASK) as i8;
    if ask < 0 {
        if w.mobys[id].pv_b(HIDDEN) == 0 {
            show(w, id, false, true);
            if let Some(t) = turret(w, id) { show(w, t, false, true); }
            for k in 0..2 { if let Some(g) = gun(w, id, k) { show(w, g, false, false); } }
        }
        let m = &mut w.mobys[id];
        m.set_pv_b(HIDDEN, 1);
        if m.pv_b(ASK) as i8 == -2 { m.set_pv_b(ASK, 0); }
        return true;
    }
    if ask == 0 {
        if w.mobys[id].pv_b(HIDDEN) != 0 { return true; }
    } else if w.mobys[id].pv_b(HIDDEN) != 0 {
        let m = &mut w.mobys[id];
        m.set_pv_b(HIDDEN, 0);
        m.set_pv_b(ASK, 0);
        show(w, id, true, true);
        if let Some(t) = turret(w, id) { show(w, t, true, true); }
        for k in 0..2 { if let Some(g) = gun(w, id, k) { show(w, g, true, false); } }
        let linked: Vec<usize> = [GUNS[0], GUNS[1], TURRET].iter().filter_map(|&o| ai::link(&w.mobys[id], o)).collect();
        let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
        crate::moby::classes::teleport::start(w, id, pos, rot, &linked);
    }
    let m = &mut w.mobys[id];
    if m.pv_b(ASK) as i8 == -2 { m.set_pv_b(ASK, 0); }
    false
}

/// `0x3f9f70`: the set-up in sub 0; its shadow within 24 of the camera; a hit on its turret taken as its own (the
/// turret's hit slot); the frame's damage — any: its groups alarmed; dead with its turret: no target flags, the
/// turret's pieces 8 and 9 shown and its health 0 (its health block given it), its break pieces, its replacement, the
/// blast 0.4 up, the parts gone, and (from state 4 on, half the time, walking and not on its way in) the fall 0x11
/// with its reaction's speeds, else no knock; hurt: the turret's health its own, its pieces broken, its sequence its
/// own at its speed, its head lost (piece 7 gone: the stagger 0xd sub 8 unless in the hurt's flight), pieces 8 / 9
/// shown with 4 / 5 gone, a gun whose piece is gone let go (updated). The target search in its area; the point it
/// aims at. A kill asked of it or its turret: out of the shooters, the parts gone, its replacement, its bolts, gone.
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].sub_state == 0 { setup(w, id, ctx); }
    if w.mobys[id].drawn != 0 { crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16); }
    if let Some(t) = turret(w, id) {
        if w.mobys[id].hit_slot == -1 && w.hit_message(t).is_some() {
            let slot = w.mobys[t].hit_slot;
            w.mobys[id].hit_slot = slot;
            if let Some(r) = usize::try_from(slot).ok().and_then(|s| w.hits.records.get_mut(s)) { r.target = id; }
        }
    }
    let before = w.mobys[id].state;
    let kind = ai::take_damage(w, id, ctx, false);
    if ctx.game.level == 9 && w.mobys[id].pv_s(REACT + 0x70) != -1 { w.mobys[id].set_pv_s(REACT + 0x70, -1); }
    if kind != 0 {
        alarm(w, id);
        if let Some(t) = turret(w, id) {
            if kind == 4 {
                w.mobys[id].mode &= !mode::TARGET;
                w.mobys[t].mode &= !mode::TARGET;
                w.mobys[t].bangles |= 0x300;
                w.mobys[id].set_pv_f(H, 0.0);
                let health: Vec<u8> = w.mobys[id].pvars[H..H + 0x30].to_vec();
                w.mobys[t].pvars[H..H + 0x30].copy_from_slice(&health);
                part_break(w, ctx, t, 0x50, 0x20);
                hand_off(w, id, ctx);
                blast(w, ctx, id, 0.4);
                drop_parts(w, id);
                let m = &w.mobys[id];
                let plain = before < 4 || ctx.game.rng.randi(2) != 0 || m.pv_i(FLYING) != 0 || !on_way_in(w, id);
                if plain {
                    w.mobys[id].set_pv_v(REACT + 0x80, [0.0; 3]);
                    w.mobys[id].set_pv_f(REACT + 0x8c, 0.0);
                } else {
                    if seq(w, id, ctx) != 0x11 { ctx.host.set_anim(&w.mobys[id], 0x11, 0, 5); }
                    let m = &mut w.mobys[id];
                    m.set_pv_i(REACT + 0x7c, -1);
                    m.set_pv_f(REACT + 0x60, m.pv_b(0xbf) as i8 as f32);
                    m.set_pv_f(REACT + 0x54, m.pv_b(0xbe) as i8 as f32);
                    let v = m.pv_v(REACT + 0x80);
                    m.set_pv_v(REACT + 0x80, scale(v, 0.05));
                }
            } else {
                let health: Vec<u8> = w.mobys[id].pvars[H..H + 0x30].to_vec();
                w.mobys[t].pvars[H..H + 0x30].copy_from_slice(&health);
                part_break(w, ctx, t, 0x50, 0x20);
                let s = seq(w, id, ctx);
                ctx.host.set_anim(&w.mobys[t], s, 0, 3);
                let speed = ctx.host.anim(&w.mobys[id]).map_or(1.0, |a| a.speed);
                ctx.host.set_anim_speed(&w.mobys[t], speed);
                if w.mobys[t].bangles & 0x80 == 0 {
                    w.mobys[id].set_pv_b(HEADLESS, 1);
                    if w.mobys[id].sub_state != 9 { st(w, id, 0xd, 8); }
                }
                let tm = &mut w.mobys[t];
                if tm.bangles & 0x10 == 0 { tm.bangles |= 0x100; }
                if tm.bangles & 0x20 == 0 { tm.bangles |= 0x200; }
                for k in 0..2 {
                    if (w.mobys[t].bangles >> (k + 4)) & 1 != 0 { continue; }
                    if let Some(g) = ai::link(&w.mobys[id], GUNS[k]).filter(|&g| g < w.mobys.len()) { w.mobys[g].mode &= !mode::NO_UPDATE; }
                    ai::set_link(&mut w.mobys[id], GUNS[k], None);
                }
            }
        }
    }
    let area = w.mobys[id].pv_i(AREA);
    ai::find_target(w, id, ctx, 0.0, 0.0, area, None, false);
    aim_point(w, id, ctx);
    let asked = w.mobys[id].pv_b(H + 0xe) == 2 || turret(w, id).is_some_and(|t| w.mobys[t].pv_b(H + 0xe) == 2);
    if asked {
        release(w, id);
        drop_parts(w, id);
        hand_off(w, id, ctx);
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        w.delete(id);
    }
}

/// `0x3f9df0`: the point it aims at: at Ratchet his aim point (within [`AIM_NEAR`] of him his centre), 0.4 up (0.2 in
/// his state 0xc); at another its place 0.8 up; its head lost, 4 along the turret's facing from 0..1 above itself.
fn aim_point(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let headless = w.mobys[id].pv_b(HEADLESS) != 0;
    let p = match turret(w, id).filter(|_| headless) {
        None => {
            let t = ai::link(&w.mobys[id], T_MOBY);
            if t.is_none() || t == ctx.hero_moby {
                let mut p = if dist2(w.mobys[id].pos, ctx.hero.pos) <= AIM_NEAR { add(ctx.hero.pos, [0.0, 0.0, 1.0]) } else { ctx.hero.aim_point };
                p[2] += if ctx.hero.state == 0xc { 0.2 } else { 0.4 };
                p
            } else {
                add(t_pos(w, id, ctx), [0.0, 0.0, 0.8])
            }
        }
        Some(t) => {
            let mut p = w.mobys[id].pos;
            p[2] += ctx.game.rng.randf(0.0, 1.0);
            add(p, scale(w.mobys[t].rows[0], 4.0))
        }
    };
    w.mobys[id].set_pv_v(T_POINT, p);
}

/// `0x3206a0` = L00 `0x324208`: the aim record at `b` of `id`'s pvars sprung (`k`, 0.3) and hung on `dst`'s joint list
/// `list` (crate::moby::joint_aim::aim).
fn aim(w: &mut MobyWorld, id: usize, b: usize, dst: usize, list: u8, k: f32) {
    crate::moby::joint_aim::aim(w, id, b, dst, list, k, f32::from_bits(0x3e99_999a));
}

/// `0x3fa598`: the turret with its colour at its joint 0, turned toward `+0x2ec` (its limits), playing at its speed;
/// in states 1..3 and 16 its thrusters ([`thrusters`]); seeing its target with its head, its aim at the target's
/// point; its guns' aims (in its sequences 8..10 at the target's point, gun 0 at most 5° right and gun 1 left; its
/// head lost, at random within ±45°).
fn turret_update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(t) = turret(w, id) else { return };
    let amb = w.mobys[id].ambient;
    w.mobys[t].ambient = amb;
    let at = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let (a, b, c, want) = { let m = &w.mobys[id]; (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c), m.pv_f(HEADING)) };
    let (mut r, mut v) = (w.mobys[t].rot[2], w.mobys[id].pv_f(TURRET_V));
    turn_ease(want, a, b, c, &mut r, &mut v, 0);
    w.mobys[id].set_pv_f(TURRET_V, v);
    let tm = &mut w.mobys[t];
    tm.pos = at;
    tm.rot[2] = r;
    let speed = ctx.host.anim(&w.mobys[id]).map_or(1.0, |a| a.speed);
    ctx.host.set_anim_speed(&w.mobys[t], speed);
    let s = w.mobys[id].state;
    if (1..=3).contains(&s) || s == 0x10 { thrusters(w, id, t, ctx); }
    let headless = w.mobys[id].pv_b(HEADLESS) != 0;
    let tp = w.mobys[id].pv_v(T_POINT);
    let turn_of = |p: V3, tm: &crate::moby::Moby| {
        let d = sub(tp, p);
        (-atan2(len2(d), d[2]), sub_rot(atan2(d[0], d[1]), tm.rot[2]))
    };
    if found(w, id) && !headless {
        if let Some(p) = ctx.host.joint(&w.mobys[t], 4) {
            let (pitch, turn) = turn_of(p, &w.mobys[t]);
            let pitch = sub_rot(pitch, w.mobys[t].rot[1]);
            w.mobys[id].set_pv_f(AIM_TURRET + 0x64, pitch);
            w.mobys[id].set_pv_f(AIM_TURRET + 0x68, turn);
        }
    }
    aim(w, id, AIM_TURRET, t, 3, 0.02);
    if !headless {
        let s = seq_a(w, t, ctx);
        if (8..0xb).contains(&s) {
            for k in 0..2 {
                if (w.mobys[t].bangles >> (k + 4)) & 1 == 0 { continue; }
                let Some(p) = ctx.host.joint(&w.mobys[t], k) else { continue };
                let (pitch, mut turn) = turn_of(p, &w.mobys[t]);
                if k == 0 && turn < -0.087_266_475 { turn = f32::from_bits(0xbdb2_b8c4); }
                if k == 1 && 0.087_266_475 < turn { turn = f32::from_bits(0x3db2_b8c4); }
                w.mobys[id].set_pv_f(AIM_GUNS[k] + 0x64, pitch);
                w.mobys[id].set_pv_f(AIM_GUNS[k] + 0x68, turn);
            }
        }
    } else {
        for k in 0..2 {
            let q = f32::from_bits(0x3fc9_0fdb);
            let a = ctx.game.rng.randf(-q, q) * 0.5;
            let b = ctx.game.rng.randf(-q, q) * 0.5;
            w.mobys[id].set_pv_f(AIM_GUNS[k] + 0x64, a);
            w.mobys[id].set_pv_f(AIM_GUNS[k] + 0x68, b);
        }
    }
    for k in 0..2 { aim(w, id, AIM_GUNS[k], t, 7 + k as u8, 0.02); }
}

/// `0x3f8cd0`: the turret's thrusters (its joints 5 and 6), down 1.7..3 a second with the legs' velocity: on even
/// frames blue (`0x6000ffff` to `0x80`, 20..24 ticks, 0x50 shrinking), odd ones red (`0xcf0000ff` to `0xcf`, 18..22,
/// 0x28); then a flame at each (`0xefff7f4f` to `0xff0000`, 10..15 ticks, 10..25 shrinking to 0).
fn thrusters(w: &mut MobyWorld, id: usize, t: usize, ctx: &mut Ctx) {
    let down = -(ctx.game.rng.randf(1.7, 3.0) * 0.016_666_668);
    let v = add([0.0, 0.0, down], w.mobys[id].pv_v(MV + 0xa0));
    let (Some(a), Some(b)) = (ctx.host.joint(&w.mobys[t], 5), ctx.host.joint(&w.mobys[t], 6)) else { return };
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    for p in [a, b] {
        if ctx.game.frame & 1 == 0 {
            let life = rng.rand_range(0x14, 0x18);
            crate::particles::trail(sys, rng, p, v, 0x6000_ffff, 0x80, life, 0x50, -10, true);
        } else {
            let life = rng.rand_range(0x12, 0x16);
            crate::particles::trail(sys, rng, p, v, 0xcf00_00ff, 0xcf, life, 0x28, -10, true);
        }
    }
    for p in [a, b] {
        let base = rng.rand_range(10, 0x19);
        let life = rng.rand_range(10, 0xf);
        crate::particles::trail(sys, rng, p, v, 0xefff_7f4f, 0xff_0000, life, base as i16, -(base as i16), true);
    }
}

/// `0x3fa8f8`: each gun on its turret's joint (`0x321768`: gun 1 mirrored, rows 1 and 2 turned round), shown as the
/// legs are, their colour.
fn guns_update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(t) = turret(w, id) else { return };
    let amb = w.mobys[id].ambient;
    let drawn = w.mobys[id].drawn != 0;
    for k in 0..2 {
        let Some(g) = ai::link(&w.mobys[id], GUNS[k]).filter(|&g| g < w.mobys.len()) else { continue };
        if let Some((r, p)) = ctx.host.joint_matrix(&w.mobys[t], k) {
            let mut r = r.map(|row| set_len(row, 1.0));
            if k == 1 { r[1] = scale(r[1], -1.0); r[2] = scale(r[2], -1.0); }
            ctx.host.advance(&w.mobys[g]);
            let gm = &mut w.mobys[g];
            gm.pos = p;
            gm.rows = r;
            gm.mode |= mode::NO_UPDATE | mode::NO_MATRIX;
        }
        let gm = &mut w.mobys[g];
        if drawn { gm.mode &= !1; gm.drawn = 1; } else { gm.mode |= 1; gm.drawn = 0; }
        gm.ambient = amb;
    }
}

/// `0x3f9d28`: alarmed, the alert timer at least 600..1000 when under [`ALERT_LOW`]; the calm timer held at [`CALM`]
/// while the alert one runs; the timers down; its ticks in the state; the head aim.
fn timers(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_b(ALARMED) != 0 {
        w.mobys[id].set_pv_b(ALARMED, 0);
        if w.mobys[id].pv_s(T_ALERT) < ALERT_LOW { alert_timer(w, id, ctx); }
    }
    if !tick_s(w, id, T_ALERT) { w.mobys[id].set_pv_s(T_CALM, CALM); }
    for off in [T_2E2, T_BURST, T_CALM] { tick_s(w, id, off); }
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks += 1; }
    // The head aim on the turret's list 4 (1.9): crate::moby::joint_aim::head_on.
    if let Some(t) = turret(w, id) { crate::moby::joint_aim::head_on(w, ctx, id, HEAD_AIM, t, 4, f32::from_bits(0x3ff3_3333)); }
}

/// 0x0d0f (module docs).
pub fn update_0d0f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !ctx.level.in_variant(w.mobys[id].pos, ctx.game.variant) { return; }
    if wake(w, id, ctx) { return; }
    // `0x3f88f8`: no collision for the tick.
    w.mobys[id].has_collision = false;
    if let Some(t) = turret(w, id) { w.mobys[t].has_collision = false; }
    let old_sub = w.mobys[id].sub_state;
    senses(w, id, ctx);
    if w.mobys.get(id).is_none_or(|m| m.state >= 0xfd) || old_sub == 0 { return; }
    subs(w, id, ctx);
    if states(w, id, ctx) { return; }
    turret_update(w, id, ctx);
    guns_update(w, id, ctx);
    timers(w, id, ctx);
    // `0x3f8938`: the collision back.
    collide(w, id, true);
    if let Some(t) = turret(w, id) { collide(w, t, true); }
    let s = w.mobys[id].state;
    if w.mobys[id].pv_i(FELL) != -1 && s != 0 && s < 0xe {
        hand_off(w, id, ctx);
        release(w, id);
        st(w, id, 0xf, 10);
    }
}

/// The sub-states (`+0x95`): 1 the way in, 2 / 3 on the path, 4 the second path, 5 / 6 the patrol, 7 facing its
/// target, 8 wobbling, 9 the hurt's flight. The turret's heading by each.
fn subs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let rotz = w.mobys[id].rot[2];
    let tp = t_pos(w, id, ctx);
    match w.mobys[id].sub_state {
        1 => {
            w.mobys[id].set_pv_f(HEADING, rotz);
            let stand = usize::try_from(w.mobys[id].pv_i(0x4b0)).ok().and_then(|k| w.mobys.get(k)).is_some_and(|m| m.state < 0xfd);
            if !stand && (found(w, id) || w.mobys[id].pv_s(T_ALERT) != 0) {
                let m = &mut w.mobys[id];
                m.set_pv_i(AREA, m.pv_i(AREA_B));
                if m.pv_b(FLAGS) & 1 == 0 {
                    alert_timer(w, id, ctx);
                    let s = w.mobys[id].state;
                    st(w, id, s, 2);
                } else {
                    anim_r(w, id, ctx, 0xc);
                    st(w, id, 8, 5);
                }
            }
        }
        2 | 3 => {
            if w.mobys[id].sub_state == 2 && tick_s(w, id, T300) {
                let m = &mut w.mobys[id];
                m.set_pv_i(AREA, m.pv_i(AREA_B));
                if m.pv_i(P_IN) != -1 {
                    let p = m.pv_i(P_IN);
                    set_path(w, id, ctx, p, -1);
                    if seq_a(w, id, ctx) != 2 { anim(w, id, ctx, 2, 0, 0); }
                    st(w, id, 2, 3);
                } else {
                    let p = m.pv_i(P_SECOND);
                    set_path(w, id, ctx, p, -1);
                    anim(w, id, ctx, 2, 0, 10);
                    ctx.host.set_anim_speed(&w.mobys[id], WALK_SPEED);
                    st(w, id, 4, 3);
                }
                if w.mobys[id].pv_b(FLAGS) & 2 != 0 {
                    let p = w.mobys[id].pv_i(P_RESPAWN);
                    set_path(w, id, ctx, p, -1);
                    w.mobys[id].set_pv_b(FLAGS, 0);
                }
            }
            let y = yaw_to(w, id, tp);
            w.mobys[id].set_pv_f(HEADING, y);
            if found(w, id) { alert_timer(w, id, ctx); }
        }
        4 => {
            w.mobys[id].set_pv_f(HEADING, rotz);
            if found(w, id) || w.mobys[id].pv_s(T_ALERT) != 0 {
                let p = w.mobys[id].pv_i(P_SECOND);
                set_path(w, id, ctx, p, -1);
                alert_timer(w, id, ctx);
                anim(w, id, ctx, 2, 0, 10);
                ctx.host.set_anim_speed(&w.mobys[id], WALK_SPEED);
                st(w, id, 4, 3);
            }
        }
        5 => {
            let y = yaw_to(w, id, tp);
            w.mobys[id].set_pv_f(HEADING, y);
            if w.mobys[id].pv_b(FLAGS) & 2 != 0 {
                walk_r(w, id, ctx);
                release(w, id);
                alert_timer(w, id, ctx);
                w.mobys[id].set_pv_s(T300, 1);
                let s = w.mobys[id].state;
                st(w, id, s, 2);
            }
            if !found(w, id) && w.mobys[id].pv_s(T_ALERT) == 0 {
                release(w, id);
                walk_r(w, id, ctx);
                st(w, id, 5, 6);
            }
        }
        6 => {
            w.mobys[id].set_pv_f(HEADING, rotz);
            if found(w, id) || w.mobys[id].pv_s(T_ALERT) != 0 {
                release(w, id);
                alert_timer(w, id, ctx);
                anim_r(w, id, ctx, 3);
                st(w, id, 5, 5);
            } else if w.mobys[id].pv_s(T_CALM) == 0 {
                release(w, id);
                let p = w.mobys[id].pv_i(P_SECOND);
                set_path(w, id, ctx, p, 0);
                walk_r(w, id, ctx);
                st(w, id, 4, 4);
            }
        }
        7 => {
            let y = yaw_to(w, id, tp);
            w.mobys[id].set_pv_f(HEADING, y);
        }
        8 => {
            let f = ctx.game.frame as f32 * 0.02;
            let a = wrap(f).sin();
            let m = &mut w.mobys[id];
            m.set_pv_f(HEADING, add_rot(m.pv_f(HEADING), (a + 2.0) * 0.023_271_061));
        }
        9 => {
            w.mobys[id].set_pv_f(HEADING, rotz);
            let r = ai::flight(w, id, ctx);
            let (sa, sb) = (seq_a(w, id, ctx), seq(w, id, ctx));
            let fall = sa == 0xf || sb == 0xf || on_way_in(w, id);
            if w.mobys[id].pv_f(H) <= 0.0 && fall { st(w, id, 0xf, -1); }
            if r & 1 != 0 {
                if 0.0 < w.mobys[id].pv_f(H) { hand_off(w, id, ctx); }
                release(w, id);
                st(w, id, 0xf, -1);
            } else if r & 2 != 0 {
                release(w, id);
                alert_timer(w, id, ctx);
                let flying = w.mobys[id].pv_i(FLYING);
                if fall || flying != 0 {
                    if flying != 2 {
                        let p = w.mobys[id].pv_i(P_IN);
                        set_path(w, id, ctx, p, -1);
                        st(w, id, 3, 3);
                    } else {
                        st(w, id, 0x10, 3);
                    }
                    anim_r(w, id, ctx, 0xe);
                } else {
                    walk_on(w, id, ctx);
                    if w.mobys[id].pv_b(HEADLESS) != 0 { st(w, id, 0xd, 8) } else { st(w, id, 6, 5) }
                }
            }
        }
        _ => {}
    }
}

/// The states (`+0x20`): 1 / 2 / 3 the way in (3 riding it; at its end onto the path after, the jump 0x10 and 7, or
/// flying: 16), 4 the second path's start (facing along it), 5 / 12 into the patrol, 6 the walk along its path
/// ([`path_end`]), 7 landing, 8 turning to its target, 9 choosing to shoot, 10 / 11 shooting, 13 staggering (shooting
/// wild), 14 out of the shooters, 15 dead, 16 flying ([`fly`]). True when it is gone.
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let wr = wrapped(w, id, ctx);
    match w.mobys[id].state {
        1 => { if wr { anim_r(w, id, ctx, 0xe); } }
        2 => {
            let tw = turret(w, id).is_some_and(|t| wrapped(w, t, ctx));
            if tw || wr {
                anim_r(w, id, ctx, 0xe);
                st(w, id, 3, -1);
            }
        }
        3 => {
            if wr { anim_r(w, id, ctx, 0xe); }
            let p = w.mobys[id].pv_i(PATH);
            if rail(w, id, ctx, p) {
                if w.mobys[id].pv_i(FLYING) == 0 {
                    let m = &mut w.mobys[id];
                    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !4);
                    let p = m.pv_i(P_AFTER);
                    set_path(w, id, ctx, p, -1);
                    anim_r(w, id, ctx, 0x10);
                    st(w, id, 7, 5);
                } else {
                    st(w, id, 0x10, 5);
                }
            }
        }
        4 => {
            let m = &w.mobys[id];
            if m.pv_i(PATH) != -1 {
                if let Some(p) = path_pts(ctx, m.pv_i(PATH)) {
                    let q = pt(p, m.pv_s(AT) as i32);
                    let y = yaw_to(w, id, q);
                    let pos = w.mobys[id].pos;
                    ai::move_step(w, id, ctx, y, pos);
                }
            }
            if wr {
                walk_on(w, id, ctx);
                st(w, id, 6, -1);
            }
        }
        5 => {
            if wr {
                if w.mobys[id].sub_state == 5 {
                    walk_on(w, id, ctx);
                    st(w, id, 6, -1);
                } else if w.mobys[id].pv_i(PATH) != -1 {
                    anim_r(w, id, ctx, 0xc);
                    st(w, id, 6, -1);
                }
            }
        }
        6 => path_end(w, id, ctx),
        7 => {
            if wr {
                if w.mobys[id].pv_b(PATROL_AFTER) != 0 {
                    anim_r(w, id, ctx, 4);
                    st(w, id, 4, 5);
                } else {
                    anim_r(w, id, ctx, 0xc);
                    st(w, id, 8, 5);
                }
            }
        }
        8 => {
            let tp = w.mobys[id].pv_v(T_POINT);
            let y = yaw_to(w, id, tp);
            let p = w.mobys[id].pos;
            ai::move_step(w, id, ctx, y, p);
            if rot_dist(y, w.mobys[id].rot[2]) < 0.174_532_95 {
                anim_r(w, id, ctx, 5);
                let m = &mut w.mobys[id];
                m.set_pv_b(PICK_C, 0);
                m.set_pv_b(PICK_D, 0);
                st(w, id, 9, -1);
            }
        }
        9 => choose(w, id, ctx, wr),
        10 => {
            if wr {
                if w.mobys[id].pv_b(PICK_D) != 0 { anim(w, id, ctx, 10, 0, 10) } else { anim(w, id, ctx, 0x13, 0, 0xc) }
                w.mobys[id].set_pv_s(T_SHOT, SHOT_GAP as i16);
                st(w, id, 0xb, -1);
            }
        }
        0xb => {
            let t = w.mobys[id].state_ticks as u16 as i32;
            if VOLLEY_START < t { fire(w, id, ctx); }
            let t = w.mobys[id].state_ticks as u16 as i32;
            let far = NEAR < dist2(w.mobys[id].pos, t_pos(w, id, ctx));
            if (VOLLEY_END < t && far) || ai::link(&w.mobys[id], TURRET).is_none() {
                let b = rr(ctx, BURST.0, BURST.1);
                w.mobys[id].set_pv_s(T_BURST, b as i16);
                release(w, id);
                anim_r(w, id, ctx, 0xc);
                let s = if w.mobys[id].pv_i(FLYING) != 0 { 0x10 } else if on_way_in(w, id) { 3 } else { 8 };
                st(w, id, s, 5);
            }
        }
        0xc => {
            if wr {
                walk_on(w, id, ctx);
                st(w, id, 6, 5);
            }
        }
        0xd => {
            if wr {
                let s = if ctx.game.rng.rand() & 1 != 0 { 10 } else { 0x13 };
                let b = rr(ctx, 0x19, 0x2d);
                anim(w, id, ctx, s, 0, b);
            }
            let sa = seq_a(w, id, ctx);
            if sa == 10 || sa == 0x13 { fire(w, id, ctx); }
        }
        0xe => release(w, id),
        0xf => {
            w.mobys[id].set_pv_f(H, 0.0);
            part_break(w, ctx, id, PARTS, H);
            release(w, id);
            drop_parts(w, id);
            crate::moby::bolts::drop(w, id, 0, 3, ctx);
            blast(w, ctx, id, 0.4);
            ctx.host.play_sound(&w.mobys[id], 3, 0);
            w.delete(id);
            return true;
        }
        0x10 => fly(w, id, ctx),
        _ => {}
    }
    false
}

/// State 9: within [`NEAR`] of its target a low shot (10, sequence 8) when one is clear; within its range, its
/// group not at its most shooting and its burst timer out, one tick in [`ODDS`] − 1 it tries a shot of its height
/// pick (`+0x30c`; a failed try flips the pick): clear → 10 when near, else the walk 0xb (6); else the stand 6 at each
/// sequence end. Taking over a stand, at each end sequence 6, and near (or the burst out, in range, its group not
/// full) 6.
fn choose(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, wr: bool) {
    let d = dist2(w.mobys[id].pos, t_pos(w, id, ctx));
    let in_range = d <= w.mobys[id].pv_f(RANGE);
    let near = d < NEAR;
    let m = &w.mobys[id];
    let room = w.shared_i(m.pv_i(SHARED)) < m.pv_i(MOST);
    if w.mobys[id].pv_b(STAND) == 0 {
        let (mut shot, mut tried) = (false, false);
        if !near && in_range && room && ctx.game.rng.randi(ODDS - 1) == 0 && w.mobys[id].pv_s(T_BURST) == 0 {
            tried = true;
            let c = w.mobys[id].pv_b(PICK_C) != 0;
            shot = can_shoot(w, id, ctx, false, c, false);
        }
        let c = w.mobys[id].pv_b(PICK_C) != 0;
        if c && shot && !near {
            anim_r(w, id, ctx, 0xb);
            st(w, id, 6, -1);
        } else if near || shot {
            low_shot(w, id, ctx, c);
        } else if wr {
            anim_r(w, id, ctx, 6);
        }
        if tried && !shot {
            let m = &mut w.mobys[id];
            m.set_pv_b(PICK_C, !m.pv_b(PICK_C));
        }
        return;
    }
    if wr { anim_r(w, id, ctx, 6); }
    if near || (w.mobys[id].pv_s(T_BURST) == 0 && in_range && room) {
        anim_r(w, id, ctx, 0xb);
        st(w, id, 6, -1);
    }
}

fn low_shot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, c: bool) {
    if can_shoot(w, id, ctx, false, c, true) { w.mobys[id].set_pv_b(PICK_D, 1); }
    take_slot(w, id);
    anim(w, id, ctx, 8, 0, 7);
    st(w, id, 10, -1);
}

/// `0x3faf00`: state 6, the walk along its path ([`walk_done`]; past 0x32 ticks of it stuck (`+0x180`), done). Done,
/// by its sub: 1 / 6 turned back, at a sequence end one of 0 / 1 and 5; 3 off the way in (taking over done), onto
/// the path after it, then 8 (5) or with a patrol after 4 (5); 4 onto the patrol (5, sub 1); 5 turned back (taking
/// over a stand: only from its start) and 8 (sub 5), the shot timer [`SHOT_GAP`]. Not done, taking over a stand in
/// sub 5 (or a stand at the way in, `+0x4c0`, in sub 3): its turret's sequence 10, firing in it.
fn path_end(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let path = w.mobys[id].pv_i(PATH);
    let done = path == -1 || walk_done(w, id, ctx, path) || 0x32 < w.mobys[id].pv_i(0x180);
    if done {
        match w.mobys[id].sub_state {
            1 | 6 => {
                reverse(w, id, ctx);
                if wrapped(w, id, ctx) {
                    walk_r(w, id, ctx);
                    st(w, id, 5, -1);
                }
            }
            3 => {
                let m = &mut w.mobys[id];
                m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !4);
                if m.pv_b(FLAGS) & 2 != 0 { m.set_pv_b(FLAGS, 0); }
                let p = m.pv_i(P_AFTER);
                set_path(w, id, ctx, p, -1);
                if w.mobys[id].pv_b(PATROL_AFTER) == 0 {
                    anim_r(w, id, ctx, 0xc);
                    st(w, id, 8, 5);
                } else {
                    anim_r(w, id, ctx, 4);
                    st(w, id, 4, 5);
                }
            }
            4 => {
                let p = w.mobys[id].pv_i(P_PATROL);
                set_path(w, id, ctx, p, -1);
                st(w, id, 5, 1);
            }
            5 => {
                let r = reverse(w, id, ctx);
                if w.mobys[id].pv_b(STAND) != 0 && !r { return; }
                anim_r(w, id, ctx, 0xc);
                st(w, id, 8, 5);
                w.mobys[id].set_pv_s(T_SHOT, SHOT_GAP as i16);
            }
            _ => {}
        }
        return;
    }
    let sub = w.mobys[id].sub_state;
    let stand = (w.mobys[id].pv_b(STAND) != 0 && sub == 5) || (w.mobys[id].pv_i(STAND_IN) != 0 && sub == 3);
    if !stand { return; }
    let Some(t) = turret(w, id) else { return };
    let m = &mut w.mobys[id];
    m.set_pv_b(PICK_C, 0);
    m.set_pv_b(PICK_D, 1);
    if seq_a(w, t, ctx) == 10 {
        fire(w, id, ctx);
    } else if seq(w, t, ctx) != 10 {
        ctx.host.set_anim(&w.mobys[t], 10, 0, 10);
    }
}

/// State 16, flying: its first tick flying for good (`+0x428` 2, its turn speed 0, options | 0x10). Within its range,
/// its group not full, not within [`NEAR`], one tick in [`ODDS`] − 1 with the burst out it tries a shot of its height
/// pick; near or clear (and its group not full): the low pick tried, options &= !0x10, one of the shooters, sequence
/// 8 and 10. A failed try flips the pick. Then [`chase`].
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let d = dist2(w.mobys[id].pos, t_pos(w, id, ctx));
    let in_range = d <= w.mobys[id].pv_f(RANGE);
    let near = d < NEAR;
    let m = &w.mobys[id];
    let room = w.shared_i(m.pv_i(SHARED)) < m.pv_i(MOST);
    if w.mobys[id].first() {
        let m = &mut w.mobys[id];
        m.set_pv_i(FLYING, 2);
        m.set_pv_f(FLY_TURN, 0.0);
        m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x10);
    }
    let (mut got, mut tried) = (false, false);
    if !near && in_range && room && ctx.game.rng.randi(ODDS - 1) == 0 && w.mobys[id].pv_s(T_BURST) == 0 {
        tried = true;
        let c = w.mobys[id].pv_b(PICK_C) != 0;
        got = can_shoot(w, id, ctx, false, c, false);
    }
    if (near || got) && room {
        let c = w.mobys[id].pv_b(PICK_C) != 0;
        if can_shoot(w, id, ctx, false, c, true) { w.mobys[id].set_pv_b(PICK_D, 1); }
        let m = &mut w.mobys[id];
        m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !0x10);
        take_slot(w, id);
        anim(w, id, ctx, 8, 0, 7);
        st(w, id, 10, -1);
    }
    if tried && !got {
        let m = &mut w.mobys[id];
        m.set_pv_b(PICK_C, !m.pv_b(PICK_C));
    }
    chase(w, id, ctx);
}

/// `0x3fae00`: the flight's step — facing its target (`0x318408`: the turn spring `+0x114..+0x11c` on `+0x4b4`); with
/// a lane (`+0x2bc`, 0xff none), toward the lane's span (its last point less its first) from Ratchet, sprung
/// (`+0x120..+0x128` on `+0x4b8`), and moved with its push-outs (`0x3fc6b0`, crate::moby::ai::move_body).
fn chase(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let before = w.mobys[id].pos;
    let tp = t_pos(w, id, ctx);
    let yaw = yaw_to(w, id, tp);
    let m = &mut w.mobys[id];
    let (k, d, max) = (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c));
    ai::turn_to(m, yaw, k, d, max, FLY_TURN);
    let lane = m.pv_i(P_AFTER);
    if lane == 0xff { return; }
    let Some(p) = path_pts(ctx, lane) else { return };
    let to = add(sub(pt(p, p.len() as i32 - 1), pt(p, 0)), ctx.hero.pos);
    let m = &mut w.mobys[id];
    let (k, d, max) = (m.pv_f(0x120), m.pv_f(0x124), m.pv_f(0x128));
    let mut pos = m.pos;
    let mut v = m.pv_f(FLY_LANE);
    ai::spring_to(&mut pos, to, k, d, max, &mut v);
    m.pos = pos;
    m.set_pv_f(FLY_LANE, v);
    ai::move_body(w, id, ctx, before);
}

/// 0x0d0e (`0x3f85e8`): the turret's own update, its shadow within 24 of the camera (range 0x16).
pub fn update_0d0e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].drawn != 0 { crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16); }
}

/// A trooper sent away by a director (L20 `0x40a998`'s steps on each one of a group: crate::moby::classes::level20::
/// director): its `+0x20` 0, out of its group's shooters (`0x3f8878`), its turret and guns gone (`0x3f8c60`), deleted.
pub fn dismiss(w: &mut MobyWorld, id: usize) {
    w.mobys[id].set_pv_i(H, 0);
    release(w, id);
    drop_parts(w, id);
    w.delete(id);
}
