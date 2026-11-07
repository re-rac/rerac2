//! The ferry 0x0330 (`func_US2_L06_003E5460`: L06 `0x3e5460`, L09 `0x3a0038`, L11 `0x3b7340`, L13 `0x3ac360`, L18
//! `0x3c00f8`; 9 placed): a boat on up to five routes, each three of the level's paths — the way in to its dock
//! (approach), the ride Ratchet sends it on with △ (depart) and the way back (back) — and a cuboid that calls it.
//! One update for every level; its tunables (`$gp+0x4d0..0x500` on L06, `+0x508` L09, `+0x5a8` L11, `+0x590` L13,
//! `+0x458` L18) are the same on all five.
//!
//! **Its pvars:** `+0x20` its platform block (crate::moby::platform), `+0x60` its speed, `+0x64` the point it heads
//! for, `+0x68` / `+0x6c` / `+0x70` its yaw / pitch / roll turn speeds on a path, `+0x74` a route forced (−1 none),
//! `+0x78` its loop sound, `+0x7c` it works, `+0x80` / `+0x94` / `+0xa8` / `+0xbc` the routes' cuboids and approach,
//! depart and back paths (−1 none), `+0xe4` a word set −1 at the dock, `+0xe8` / `+0xec` its bob's last offset and
//! phase, `+0xf0` / `+0xf4` its sway's phases, `+0xf8` its camera path, `+0xfc` its turn speed at the dock, `+0x100`
//! which help texts it shows, `+0x104` a wait (120 at the far end; nothing reads it), `+0x108` its height to bob about.
//! The route is the moby's `+0xbc` byte ([`crate::moby::Moby::timer`]).
//!
//! * On Joba (level 11) with `+0x100` 0 and item 8 had (`0x1a7b08`) it deletes itself.
//! * **0**: its bob height its own; at the first route's approach start (or else its depart start) facing the
//!   path's second point — placed straight there when shown, else through the fuse effect (`0x32bed8` = the decomp's
//!   `func_L02_002EF388`, class 0x09bb: not ported, placed straight there too) — the height that point's. Its draw
//!   distance 0x80, its update distance 255, no forced route, no collision, 1. Its uid marked (`0x21ecc4[uid]` or its
//!   count in `0x19b4a8`): deleted.
//! * **1**: working and its mission done (`0x19b2e8`), [`call`]: a route whose cuboid holds Ratchet (`0x3622d0` = L00
//!   `0x321858`) or forced.
//! * **2**: along the route's approach ([`follow`]; none: at once); at its end 3 and `+0xe4` −1.
//! * **3**: at the dock: it turns toward its depart path's second point (`0x35c2a0` = L00 `0x31c078`: 0.0006 / 0.0006,
//!   at most 1.17°), its pitch and roll eased to 0 (`0x35f460` = L00 `0x31f230`); with Ratchet standing on it
//!   (`0x18a15c`, `0x18a16e` 0) its help text ([`prompt_text`], `0x3e5228`, owner 8) and △ with the prompt its own
//!   (`0x1a8fb4` = 8): its speed 0, 5, point 1. Another route called ([`call`]) takes it there instead.
//! * **4**: 5. **5**: along the depart path; at its end 6, the wait 120.
//! * **6**: pitch and roll eased to 0; with a back path and Ratchet more than 15 away: 7, point 1; with none and him
//!   more than 5 away across: 1.
//! * **7**: along the back path; at its end no collision, 1.
//! * **Each tick** after: unless in 5 or 7 it bobs 0.2 about its height at 120°/s (`0x365278` = L00 `0x324430`), it
//!   sways 4.5° at 70°/s and 100°/s (`0x365330` = L00 `0x3244e8`, crate::moby::classes::props), its move into its
//!   platform block (`0x363680` = L00 `0x322668`) and Ratchet carried (crate::moby::carry_hero_with).
//!
//! **First each tick** (past level 11's deletion): its talk record's word set to whether it moves, its state in 2..6
//! (`0x370b60(m, 0x3e5200(m))` = the decomp's `func_L06_0036F728`, crate::moby::talk::set_word).
//!
//! Not ported: the debug lines of its first route (`$gp+0x4e4` 0); on the ride with Ratchet on it
//! (`0x3e5408`, the ledge or the ground) the hero words `0x18a032` / `0x18a034` (4 / 8: his air states refused) and its camera path (`0x3e64d8`: the path `+0xf8` of 7
//! points rebuilt from the table `0x296f80` through its frame, its index into `0x18c31e`), as
//! crate::moby::classes::level17::lifts; the fuse effect 0x09bb.

use crate::hero::physics::{add, atan2, dist, dist2, ease, len, len2, set_len, sub, turn_ease, V3};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0330;

const SPEED: f32 = 18.0;
const ACCEL: f32 = 8.0;
const YAW_ACCEL: f32 = 90.0;
const YAW_MAX: f32 = 40.0;
const PITCH_ACCEL: f32 = 45.0;
const ROLL_ACCEL: f32 = 45.0;
const ROLL_K: f32 = -50.0;
const BOB_AMP: f32 = f32::from_bits(0x3e4c_cccd);
const BOB_RATE: f32 = 120.0;
const SWAY: f32 = f32::from_bits(0x3da0_d97d);
const SWAY_A: f32 = 70.0;
const SWAY_B: f32 = 100.0;
/// 1/60, 1/3600, °/s → rad/tick, °/s² → rad/tick².
const DT: f32 = f32::from_bits(0x3c88_8889);
const DT2: f32 = f32::from_bits(0x3991_a2b4);
const DEG: f32 = f32::from_bits(0x3998_825c);
const DEG2: f32 = f32::from_bits(0x36a2_ad2f);
/// The most a path's pitch eases by (12°) and the most it leans (15°).
const TILT_MAX: f32 = f32::from_bits(0x3e56_7752);
const PITCH_MAX: f32 = f32::from_bits(0x3e86_0a93);

const BLOCK: usize = 0x20;
const SPEED_V: usize = 0x60;
const POINT: usize = 0x64;
const YAW_V: usize = 0x68;
const PITCH_V: usize = 0x6c;
const ROLL_V: usize = 0x70;
const FORCED: usize = 0x74;
const SOUND: usize = 0x78;
const ENABLED: usize = 0x7c;
const TRIGGER: usize = 0x80;
const APPROACH: usize = 0x94;
const DEPART: usize = 0xa8;
const BACK: usize = 0xbc;
const BOB_LAST: usize = 0xe8;
const BOB_PHASE: usize = 0xec;
const SWAY_P: usize = 0xf0;
const SWAY_Q: usize = 0xf4;
const DOCK_V: usize = 0xfc;
const VARIANT: usize = 0x100;
const WAIT: usize = 0x104;
const BASE: usize = 0x108;

/// The prompt's owner (`0x1a8fb4`).
const OWNER: i32 = 8;

fn path<'a>(ctx: &Ctx<'a>, k: i32) -> Option<&'a [[f32; 4]]> {
    let level = ctx.level;
    usize::try_from(k).ok().and_then(|k| level.paths.get(k)).map(|p| p.as_slice()).filter(|p| !p.is_empty())
}

fn pt(p: &[[f32; 4]], k: usize) -> V3 { let q = p[k.min(p.len() - 1)]; [q[0], q[1], q[2]] }

/// The yaw from its place toward the path's second point (`0x2ef510`).
fn yaw_to(at: V3, p: &[[f32; 4]]) -> f32 { let q = pt(p, 1); atan2(q[0] - at[0], q[1] - at[1]) }

/// At the path's start facing its second point (the fuse effect's place, module docs), its bob height that point's.
fn place(w: &mut MobyWorld, id: usize, p: &[[f32; 4]]) {
    let m = &mut w.mobys[id];
    let yaw = yaw_to(m.pos, p);
    m.pos = pt(p, 0);
    m.rot = [0.0, 0.0, yaw];
    m.set_pv_f(BASE, p[0][2]);
}

/// `0x3e6118(m, path, &sound)` (the decomp's `func_L06_003E4BE0`): one tick toward the path's point `+0x64` — within
/// two ticks of the most speed of it the next (true at the end). The speed eases toward the distance left to the
/// last point (`0x35efa8` = L00 `0x31ed78`: 8/3600 a tick, at most 18/60); its pitch toward the climb (at most 15°,
/// 45 °/s², 12° a tick), its yaw toward the point (90 °/s², at most 40 °/s), its roll toward −50 × its yaw speed × its
/// speed (45 °/s², 12°). Its bob height the point's and its bob reset. Its loop sound (sound 0 of its class, looped)
/// while under way, let go at the end.
fn follow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: i32) -> bool {
    let Some(p) = path(ctx, k) else { return true };
    let n = p.len() as i32;
    let m = &mut w.mobys[id];
    let mut i = m.pv_i(POINT);
    let target = pt(p, i.max(0) as usize);
    m.set_pv_f(BASE, target[2]);
    m.set_pv_f(BOB_PHASE, 0.0);
    m.set_pv_f(BOB_LAST, 0.0);
    let mut d = sub(target, m.pos);
    let mut done = false;
    let step = SPEED * DT;
    if len(d) < step + step {
        i += 1;
        m.set_pv_i(POINT, i);
        done = i == n;
    }
    let left = dist(m.pos, pt(p, (n - 1) as usize));
    let (mut h, mut v) = (0.0, m.pv_f(SPEED_V));
    ease(left, ACCEL * DT2, ACCEL * DT2, step, &mut h, &mut v);
    m.set_pv_f(SPEED_V, v);
    d = set_len(d, v);
    m.pos = add(m.pos, d);
    let pitch = (-atan2(len2(d), d[2])).clamp(-PITCH_MAX, PITCH_MAX);
    let (mut a, mut av) = (m.rot[1], m.pv_f(PITCH_V));
    turn_ease(pitch, PITCH_ACCEL * DEG2, PITCH_ACCEL * DEG2, TILT_MAX, &mut a, &mut av, 0);
    m.rot[1] = a;
    m.set_pv_f(PITCH_V, av);
    let yaw = atan2(target[0] - m.pos[0], target[1] - m.pos[1]);
    let (mut a, mut av) = (m.rot[2], m.pv_f(YAW_V));
    turn_ease(yaw, YAW_ACCEL * DEG2, YAW_ACCEL * DEG2, YAW_MAX * DEG, &mut a, &mut av, 0);
    m.rot[2] = a;
    m.set_pv_f(YAW_V, av);
    let roll = av * m.pv_f(SPEED_V) * ROLL_K;
    let (mut a, mut av) = (m.rot[0], m.pv_f(ROLL_V));
    turn_ease(roll, ROLL_ACCEL * DEG2, ROLL_ACCEL * DEG2, TILT_MAX, &mut a, &mut av, 0);
    m.rot[0] = a;
    m.set_pv_f(ROLL_V, av);
    let h = m.pv_i(SOUND);
    if !done {
        if !ctx.host.sound_playing(h) {
            let s = ctx.host.play_sound_of(&w.mobys[id], CLASS, 0, 4);
            w.mobys[id].set_pv_i(SOUND, s);
        }
    } else {
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(SOUND, -1);
    }
    done
}

/// Pitch and roll eased back to 0 (90 / 45 °/s², 12°).
fn level_out(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let (mut a, mut v) = (m.rot[1], m.pv_f(PITCH_V));
    turn_ease(0.0, YAW_ACCEL * DEG2, YAW_ACCEL * DEG2, TILT_MAX, &mut a, &mut v, 0);
    m.rot[1] = a;
    m.set_pv_f(PITCH_V, v);
    let (mut a, mut v) = (m.rot[0], m.pv_f(ROLL_V));
    turn_ease(0.0, ROLL_ACCEL * DEG2, ROLL_ACCEL * DEG2, TILT_MAX, &mut a, &mut v, 0);
    m.rot[0] = a;
    m.set_pv_f(ROLL_V, v);
}

/// The routes' calls (other than `skip`): the first whose cuboid holds Ratchet, or forced, gets its collision and
/// goes to its approach start (or, with none and its depart start more than 4 away, there; then it turns toward the
/// depart path: 0.0012 / 0.0012, at most 6°); that route, 2, its draw distance 0x80, its speed the most, point 1, no
/// forced route.
fn call(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, skip: Option<u8>) {
    for i in 0..5usize {
        let m = &w.mobys[id];
        let (cub, forced) = (m.pv_i(TRIGGER + 4 * i), m.pv_i(FORCED) == i as i32);
        if cub == -1 && !forced { continue; }
        if skip == Some(i as u8) { continue; }
        if !ctx.level.in_cuboid(cub, ctx.hero.pos) && !forced { continue; }
        let coll = w.class(CLASS).is_some_and(|c| c.has_collision);
        w.mobys[id].has_collision = coll;
        let (ap, dp) = (w.mobys[id].pv_i(APPROACH + 4 * i), w.mobys[id].pv_i(DEPART + 4 * i));
        if let (true, Some(p)) = (ap != -1, path(ctx, ap)) {
            place(w, id, p);
        } else if let (true, Some(p)) = (ap == -1 && dp != -1, path(ctx, dp)) {
            if 4.0 < dist(w.mobys[id].pos, pt(p, 0)) { place(w, id, p); }
            let m = &mut w.mobys[id];
            let yaw = yaw_to(m.pos, p);
            crate::moby::ai::turn_to(m, yaw, f32::from_bits(0x3a98_825c), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752), DOCK_V);
        }
        let m = &mut w.mobys[id];
        m.timer = i as u8;
        m.state = 2;
        m.draw_dist = 0x80;
        m.set_pv_f(SPEED_V, SPEED * DT);
        m.set_pv_i(POINT, 1);
        m.set_pv_i(FORCED, -1);
        break;
    }
}

/// `0x3e5228(m, route)`: the help text for Ratchet on it at the dock, by the level, the route and `+0x100`
/// (`0x329d50(8, text, 1)`, crate::prompt); none for some.
pub fn prompt_text(level: i32, variant: bool, route: u8) -> Option<u32> {
    Some(match route {
        0 => match level {
            13 => if variant { 0x12e5 } else { 0x1336 },
            18 => if variant { 0x12e5 } else { 0x1339 },
            _ => 0x12e5,
        },
        1 => match level {
            9 => 0x131c,
            18 => if variant { 0x134f } else { 0x1339 },
            12 => 0x12ee,
            13 => if variant { 0x1337 } else { 0x12e5 },
            11 => if variant { 0x1314 } else { 0x1315 },
            _ => 0x130c,
        },
        2 => match level {
            11 => 0x12e5,
            18 => if variant { 0x1339 } else { 0x12e5 },
            _ => return None,
        },
        3 => match level {
            18 => if variant { 0x12e5 } else { 0x1339 },
            _ => return None,
        },
        4 => return (level == 18 && variant).then_some(0x1339),
        _ => 0x12e5,
    })
}

/// 0x0330's update (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.level == 11 && w.mobys[id].pv_i(VARIANT) == 0 && ctx.game.items.get(8).is_some_and(|&b| b != 0) {
        w.delete(id);
        return;
    }
    let moving = (2..=6).contains(&w.mobys[id].state) as i32;
    crate::moby::talk::set_word(w, id, ctx, moving);
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let route = w.mobys[id].timer as usize;
    match w.mobys[id].state {
        0 => {
            let z = w.mobys[id].pos[2];
            w.mobys[id].set_pv_f(BASE, z);
            let (ap, dp) = (w.mobys[id].pv_i(APPROACH), w.mobys[id].pv_i(DEPART));
            let first = if ap != -1 { path(ctx, ap) } else if dp != -1 { path(ctx, dp) } else { None };
            if let Some(p) = first { place(w, id, p); }
            let m = &mut w.mobys[id];
            m.draw_dist = 0x80;
            m.update_dist = 255;
            m.set_pv_i(FORCED, -1);
            m.state = 1;
            m.has_collision = false;
            let uid = m.uid;
            if ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(uid) != 0 {
                w.delete(id);
                return;
            }
        }
        1 => {
            let m = &w.mobys[id];
            if m.pv_i(ENABLED) != 0 && crate::moby::director::mission_is_done(ctx.game, m.mission as i32) {
                call(w, id, ctx, None);
            }
        }
        2 => {
            let ap = w.mobys[id].pv_i(APPROACH + 4 * route);
            if ap == -1 || follow(w, id, ctx, ap) {
                w.mobys[id].state = 3;
                w.mobys[id].set_pv_i(0xe4, -1);
            }
        }
        3 => {
            if let Some(p) = path(ctx, w.mobys[id].pv_i(DEPART + 4 * route)) {
                let m = &mut w.mobys[id];
                let yaw = yaw_to(m.pos, p);
                crate::moby::ai::turn_to(m, yaw, f32::from_bits(0x3a18_825c), f32::from_bits(0x3a18_825c), f32::from_bits(0x3ca6_ce95), DOCK_V);
            }
            level_out(w, id);
            if ctx.hero.ground_moby == Some(id) && ctx.hero.air_ticks == 0 {
                let variant = w.mobys[id].pv_i(VARIANT) != 0;
                if let Some(t) = prompt_text(ctx.game.level, variant, route as u8) { ctx.game.prompt.request(OWNER, t, 1); }
                if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 && ctx.game.prompt.owner == OWNER {
                    let m = &mut w.mobys[id];
                    m.set_pv_f(SPEED_V, 0.0);
                    m.state = 5;
                    m.set_pv_i(POINT, 1);
                }
            }
            call(w, id, ctx, Some(route as u8));
        }
        4 => w.mobys[id].state = 5,
        5 => {
            let dp = w.mobys[id].pv_i(DEPART + 4 * route);
            if follow(w, id, ctx, dp) {
                w.mobys[id].state = 6;
                w.mobys[id].set_pv_i(WAIT, 120);
            }
        }
        6 => {
            level_out(w, id);
            let m = &mut w.mobys[id];
            let back = m.pv_i(BACK + 4 * route);
            if back != -1 && 15.0 < dist(m.pos, ctx.hero.pos) {
                m.state = 7;
                m.set_pv_i(POINT, 1);
            } else if back == -1 && 5.0 < dist2(m.pos, ctx.hero.pos) {
                m.state = 1;
            }
        }
        7 => {
            let back = w.mobys[id].pv_i(BACK + 4 * route);
            if follow(w, id, ctx, back) {
                w.mobys[id].has_collision = false;
                w.mobys[id].state = 1;
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.state != 5 && m.state != 7 {
        let base = m.pv_f(BASE);
        super::props::bob(m, BOB_AMP, BOB_RATE * DEG, base, BOB_PHASE, BOB_LAST);
    }
    super::props::sway(m, SWAY, SWAY_A * DEG, SWAY_B * DEG, SWAY_P, SWAY_Q);
    let (pos, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, BLOCK, sub(pos, old), old_rot, rot);
    let flags = m.pv_i(BLOCK + 0x3c) as u32;
    crate::moby::carry_hero_with(ctx, id, flags, old, old_rot[2], pos, rot[2]);
}
