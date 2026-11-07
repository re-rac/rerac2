//! Smolg's path platform **0x0922** (L17 `0x3c76b8`, 1 placed; the decomp's `func_US2_L17_003C76B8`, its helpers
//! `func_L17_003C5DF0`.. `func_L17_003C7018`, here by their v2 places `0x3c7378`.. `0x3c85a0`): a platform on up to five
//! routes, each a cuboid that calls it and three of the level's paths — the way in (entry), the ride Ratchet sends it
//! on with △ (middle; back along it from its far end) and the way out (exit). Hidden until called. The family of
//! crate::moby::classes::ferry, with its own code and tunables (`$gp+0x12ec..0x1318`).
//!
//! **Its pvars:** `+0x3e` its flags (| 8 each tick), `+0x50` its platform block (crate::moby::platform; its flags
//! `+0x8c` | 1 each tick), `+0x90` its speed, `+0x94` the point it heads for, `+0x98` / `+0x9c` / `+0xa0` its yaw /
//! pitch / roll turn speeds, `+0xa4` a route asked for (−1 none), `+0xa8` its loop sound, `+0xac` it works, `+0xb0` /
//! `+0xc4` / `+0xd8` / `+0xec` the routes' cuboids and entry, middle and exit paths (−1 none), `+0x114` a word set −1
//! at the dock, `+0x118` / `+0x11c` its bob's last offset and phase, `+0x120` / `+0x124` its sway's phases, `+0x128`
//! its camera path, `+0x12c` its height at the dock. The route is the moby's `+0xbc` byte
//! ([`crate::moby::Moby::timer`]).
//!
//! * **0**: at the first route's entry start (or else its middle start) facing the path's second point; its draw
//!   distance 0, its update distance 255, its sound and route asked none, 1, hidden (mode | 0x41, & ~0x1000). Its uid
//!   marked (`0x21f014[uid]` or its count in `0x19b4a8`): deleted.
//! * **1**: working and its mission done (`0x19b2e8`, bit 7), for each route with a cuboid (or asked) the call
//!   ([`call`], `0x3c74e0`).
//! * **2**: along the route's entry ([`follow`], `0x3c7f30`; none: at once); at its end 3, `+0x114` −1 and its height
//!   kept.
//! * **3**: pitch and roll eased to 0 ([`level_out`]); with Ratchet standing on it (`0x18a15c`, `0x18a16e` 0) the
//!   prompt 0x133f (`0x329d50(10, 0x133f, 1)`, crate::prompt) and △ (`0x138324 & 0x10`) its speed 0, 4, point 1;
//!   without him the calls. Its loop sound kept playing.
//! * **4**: 5, its sound stopped. **5**: along the middle path; at its end 6.
//! * **6**: pitch and roll eased to 0; with an exit path and Ratchet more than 15 away: 7, point 1; with none and him
//!   off it (`0x18a15c`): 1. Its loop sound kept playing.
//! * **7**: along the exit path; at its end its draw distance 0, 1.
//! * **8**: as 3 with no calls; △ sends it back: its speed 0, 9, point the middle path's last but one. (Nothing in
//!   its code sets 8.)
//! * **9**: 10, its sound stopped. **10**: back along the middle path ([`follow_back`], `0x3c8270`); at its start 6.
//! * **Each tick** after: it bobs 0.01 about its dock height in 3 (else by its change) at 120°/s (`0x3265a8` = L00
//!   `0x324430`) and sways 4.5° at 70°/s and 100°/s (`0x326660` = L00 `0x3244e8`, crate::moby::classes::props: both
//!   `0x3c85a0`); its move into its platform block (`0x324660` = L00 `0x322668`) and Ratchet carried
//!   (crate::moby::carry_hero_with).
//!
//! The loop sounds: on the paths sound 0 of its class, at the dock (3, 6, 8) sound 1 (mode 4, `0x363d28` = L00
//! `0x360698`), started when its handle is not playing (`0x363938` = L00 `0x3602a8`); stopped `0x363980` = L00
//! `0x3602f0`.
//!
//! **First each tick**: its talk record's word set to whether it moves, its state in 2..6 (`0x333080(m, 0x3c7378(m))`
//! = the decomp's `func_L06_0036F728`, crate::moby::talk::set_word, as crate::moby::classes::ferry).
//!
//! Not ported: the debug lines of its first route's paths (`$gp+0x1300` 0); with Ratchet on it in 5
//! and 10 (`0x3c73a8`, crate::moby::ai::stands_on) the hero words `0x18a032` / `0x18a034` (4
//! each) and in 5 its camera path (`0x3c7400`: the path `+0x128` of 7 points rebuilt from the table `0x297070`
//! through its frame, its index into `0x18c31e`), as crate::moby::classes::level17::lifts; the class sound table's
//! test (`0x1dbf82` ≠ 0xff) before a loop sound starts; the animation word `+0x98` (the class's first, or none
//! hidden), which the engine's animation stands in for.

use crate::hero::physics::{add, approach, atan2, dist, dist2, len, len2, set_len, sub, turn_ease, V3};
use crate::moby::classes::props::{bob, sway};
use crate::moby::{stop_loop, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0922;

/// `$gp+0x1304..0x1318`: the most speed (units/s), the brake (units/s²), the pitch's, yaw's and roll's turn rates
/// (°/s²) and the roll by the yaw speed.
const SPEED: f32 = 36.0;
const BRAKE: f32 = 3.0;
const PITCH_ACCEL: f32 = 180.0;
const YAW_ACCEL: f32 = 45.0;
const ROLL_ACCEL: f32 = 45.0;
const ROLL_K: f32 = -50.0;
/// `$gp+0x12ec..0x12fc`: the bob (amplitude, °/s) and the sway (tilt, °/s, °/s).
const BOB_AMP: f32 = f32::from_bits(0x3c23_d70a);
const BOB_RATE: f32 = 120.0;
const SWAY: f32 = f32::from_bits(0x3da0_d97d);
const SWAY_A: f32 = 70.0;
const SWAY_B: f32 = 100.0;
/// 1/60, 1/3600, °/s → rad/tick, °/s² → rad/tick².
const DT: f32 = f32::from_bits(0x3c88_8889);
const DT2: f32 = f32::from_bits(0x3991_a2b4);
const DEG: f32 = f32::from_bits(0x3998_825c);
const DEG2: f32 = f32::from_bits(0x36a2_ad2f);
/// The most a turn eases by a tick (12°) and the most the pitch leans (15°).
const TILT_MAX: f32 = f32::from_bits(0x3e56_7752);
const PITCH_MAX: f32 = f32::from_bits(0x3e86_0a93);

const FLAGS: usize = 0x3e;
const BLOCK: usize = 0x50;
const SPEED_V: usize = 0x90;
const POINT: usize = 0x94;
const YAW_V: usize = 0x98;
const PITCH_V: usize = 0x9c;
const ROLL_V: usize = 0xa0;
const ASKED: usize = 0xa4;
const SOUND: usize = 0xa8;
const ENABLED: usize = 0xac;
const TRIGGER: usize = 0xb0;
const ENTRY: usize = 0xc4;
const MIDDLE: usize = 0xd8;
const EXIT: usize = 0xec;
const DOCKED: usize = 0x114;
const BOB_LAST: usize = 0x118;
const BOB_PHASE: usize = 0x11c;
const SWAY_P: usize = 0x120;
const SWAY_Q: usize = 0x124;
const DOCK_Z: usize = 0x12c;

/// The prompt's owner and text.
const OWNER: i32 = 10;
const PROMPT: u32 = 0x133f;

fn path<'a>(ctx: &Ctx<'a>, k: i32) -> Option<&'a [[f32; 4]]> {
    let level = ctx.level;
    usize::try_from(k).ok().and_then(|k| level.paths.get(k)).map(|p| p.as_slice()).filter(|p| !p.is_empty())
}

fn pt(p: &[[f32; 4]], k: usize) -> V3 { let q = p[k.min(p.len() - 1)]; [q[0], q[1], q[2]] }

/// At the path's start facing its second point.
fn place(w: &mut MobyWorld, id: usize, p: &[[f32; 4]]) {
    let m = &mut w.mobys[id];
    m.pos = pt(p, 0);
    let q = pt(p, 1);
    m.rot[2] = atan2(q[0] - m.pos[0], q[1] - m.pos[1]);
}

/// Its loop sound `index` started when its handle is not playing.
fn keep_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, index: i32) {
    if ctx.host.sound_playing(w.mobys[id].pv_i(SOUND)) { return; }
    let h = ctx.host.play_sound_of(&w.mobys[id], CLASS, index, 4);
    w.mobys[id].set_pv_i(SOUND, h);
}

/// `0x3c7f30(m, path, &sound)` / `0x3c8270` (`back`): one tick toward the path's point `+0x94` — within two ticks of
/// the most speed of it the next (the one before; true past the last / first). Its speed: once the brake's stop
/// distance reaches the distance to the path's last (first) point, toward 0 by what stops it there; else toward the
/// most by the brake (`0x31fe00` = L00 `0x31eb18`, crate::hero::physics::approach). Its pitch toward the climb (at most
/// 15°), its yaw toward the point, its roll toward −50 × its yaw speed × its speed (`0x320518` = L00 `0x31f230`,
/// crate::hero::physics::turn_ease, at most 12° a tick). Its loop sound 0 while under way, stopped at the end.
fn follow_dir(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: i32, back: bool) -> bool {
    let Some(p) = path(ctx, k) else { return true };
    let n = p.len() as i32;
    let m = &mut w.mobys[id];
    let mut i = m.pv_i(POINT);
    let target = pt(p, i.max(0) as usize);
    let mut d = sub(target, m.pos);
    let mut done = false;
    let step = SPEED * DT;
    if len(d) < step + step {
        i += if back { -1 } else { 1 };
        m.set_pv_i(POINT, i);
        done = if back { i == -1 } else { i == n };
    }
    let left = dist(m.pos, pt(p, if back { 0 } else { (n - 1) as usize }));
    let brake = BRAKE * DT2;
    let mut v = m.pv_f(SPEED_V);
    let v2 = v * v;
    if left <= v2 / (brake + brake) {
        approach(0.0, v2 / (left + left), &mut v);
    } else {
        approach(step, brake, &mut v);
    }
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
    turn_ease(yaw, YAW_ACCEL * DEG2, YAW_ACCEL * DEG2, TILT_MAX, &mut a, &mut av, 0);
    m.rot[2] = a;
    m.set_pv_f(YAW_V, av);
    let roll = av * m.pv_f(SPEED_V) * ROLL_K;
    let (mut a, mut av) = (m.rot[0], m.pv_f(ROLL_V));
    turn_ease(roll, ROLL_ACCEL * DEG2, ROLL_ACCEL * DEG2, TILT_MAX, &mut a, &mut av, 0);
    m.rot[0] = a;
    m.set_pv_f(ROLL_V, av);
    if done { stop_loop(w, id, ctx, SOUND) } else { keep_sound(w, id, ctx, 0) }
    done
}

fn follow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: i32) -> bool { follow_dir(w, id, ctx, k, false) }
fn follow_back(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: i32) -> bool { follow_dir(w, id, ctx, k, true) }

/// Pitch and roll eased back to 0 (180 / 45 °/s², 12°).
fn level_out(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let (mut a, mut v) = (m.rot[1], m.pv_f(PITCH_V));
    turn_ease(0.0, PITCH_ACCEL * DEG2, PITCH_ACCEL * DEG2, TILT_MAX, &mut a, &mut v, 0);
    m.rot[1] = a;
    m.set_pv_f(PITCH_V, v);
    let (mut a, mut v) = (m.rot[0], m.pv_f(ROLL_V));
    turn_ease(0.0, ROLL_ACCEL * DEG2, ROLL_ACCEL * DEG2, TILT_MAX, &mut a, &mut v, 0);
    m.rot[0] = a;
    m.set_pv_f(ROLL_V, v);
}

/// `0x3c74e0`: the first route whose cuboid holds Ratchet (`0x3234b8` = L00 `0x321858`), or asked, shows it (mode &
/// ~0x41 | 0x1000) at its entry start facing the path's second point (or, with none and its middle start more than 2
/// across away, there); that route, 2, its draw distance 0x80, none asked, point 1, its speed the most.
fn call(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for i in 0..5usize {
        let m = &w.mobys[id];
        let (cub, asked) = (m.pv_i(TRIGGER + 4 * i), m.pv_i(ASKED) == i as i32);
        if cub == -1 && !asked { continue; }
        if !ctx.level.in_cuboid(cub, ctx.hero.pos) && !asked { continue; }
        let m = &mut w.mobys[id];
        m.mode = (m.mode & 0xffbe) | 0x1000;
        let (ep, mp) = (m.pv_i(ENTRY + 4 * i), m.pv_i(MIDDLE + 4 * i));
        if ep != -1 {
            if let Some(p) = path(ctx, ep) { place(w, id, p); }
        } else if let (true, Some(p)) = (mp != -1, path(ctx, mp)) {
            if 2.0 < dist2(w.mobys[id].pos, pt(p, 0)) { place(w, id, p); }
        }
        let m = &mut w.mobys[id];
        m.timer = i as u8;
        m.state = 2;
        m.draw_dist = 0x80;
        m.set_pv_i(ASKED, -1);
        m.set_pv_i(POINT, 1);
        m.set_pv_f(SPEED_V, SPEED * DT);
        return;
    }
}

/// Ratchet standing on it: the prompt, and △ for it.
fn prompted(ctx: &mut Ctx, id: usize) -> Option<bool> {
    if ctx.hero.ground_moby != Some(id) || ctx.hero.air_ticks != 0 { return None; }
    ctx.game.prompt.request(OWNER, PROMPT, 1);
    Some(ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0)
}

/// 0x0922's update (module docs).
pub fn update_0922(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let moving = (2..=6).contains(&w.mobys[id].state) as i32;
    crate::moby::talk::set_word(w, id, ctx, moving);
    {
        let m = &mut w.mobys[id];
        m.set_pv_i(BLOCK + 0x3c, m.pv_i(BLOCK + 0x3c) | 1);
        m.set_pv_s(FLAGS, m.pv_s(FLAGS) | 8);
    }
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let route = w.mobys[id].timer as usize;
    match w.mobys[id].state {
        0 => {
            let (ep, mp) = (w.mobys[id].pv_i(ENTRY), w.mobys[id].pv_i(MIDDLE));
            let first = if ep != -1 { path(ctx, ep) } else if mp != -1 { path(ctx, mp) } else { None };
            if let Some(p) = first { place(w, id, p); }
            let m = &mut w.mobys[id];
            m.draw_dist = 0;
            m.update_dist = 0xff;
            m.set_pv_i(SOUND, -1);
            m.set_pv_i(ASKED, -1);
            m.state = 1;
            m.mode = (m.mode | 0x41) & 0xefff;
            let uid = m.uid;
            if ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(uid) != 0 {
                w.delete(id);
                return;
            }
        }
        1 => {
            let m = &w.mobys[id];
            if m.pv_i(ENABLED) != 0 && crate::moby::director::mission_is_done(ctx.game, m.mission as i32) {
                for i in 0..5 {
                    let m = &w.mobys[id];
                    if m.pv_i(TRIGGER + 4 * i) != -1 || m.pv_i(ASKED) == i as i32 { call(w, id, ctx); }
                }
            }
        }
        2 => {
            let ep = w.mobys[id].pv_i(ENTRY + 4 * route);
            if ep == -1 || follow(w, id, ctx, ep) {
                let m = &mut w.mobys[id];
                m.state = 3;
                m.set_pv_i(DOCKED, -1);
                m.set_pv_f(DOCK_Z, m.pos[2]);
            }
        }
        3 => {
            level_out(w, id);
            match prompted(ctx, id) {
                Some(true) => {
                    let m = &mut w.mobys[id];
                    m.set_pv_f(SPEED_V, 0.0);
                    m.state = 4;
                    m.set_pv_i(POINT, 1);
                }
                Some(false) => {}
                None => call(w, id, ctx),
            }
            keep_sound(w, id, ctx, 1);
        }
        4 => {
            w.mobys[id].state = 5;
            stop_loop(w, id, ctx, SOUND);
        }
        5 => {
            let mp = w.mobys[id].pv_i(MIDDLE + 4 * route);
            if follow(w, id, ctx, mp) { w.mobys[id].state = 6; }
        }
        6 => {
            level_out(w, id);
            let m = &mut w.mobys[id];
            let exit = m.pv_i(EXIT + 4 * route);
            if exit != -1 && 15.0 < dist(m.pos, ctx.hero.pos) {
                m.state = 7;
                m.set_pv_i(POINT, 1);
            } else if exit == -1 && ctx.hero.ground_moby != Some(id) {
                m.state = 1;
            }
            keep_sound(w, id, ctx, 1);
        }
        7 => {
            let exit = w.mobys[id].pv_i(EXIT + 4 * route);
            if follow(w, id, ctx, exit) {
                let m = &mut w.mobys[id];
                m.draw_dist = 0;
                m.state = 1;
            }
        }
        8 => {
            level_out(w, id);
            if prompted(ctx, id) == Some(true) {
                let n = path(ctx, w.mobys[id].pv_i(MIDDLE + 4 * route)).map_or(0, |p| p.len() as i32);
                let m = &mut w.mobys[id];
                m.set_pv_f(SPEED_V, 0.0);
                m.state = 9;
                m.set_pv_i(POINT, n - 2);
            }
            keep_sound(w, id, ctx, 1);
        }
        9 => {
            w.mobys[id].state = 10;
            stop_loop(w, id, ctx, SOUND);
        }
        10 => {
            let mp = w.mobys[id].pv_i(MIDDLE + 4 * route);
            if follow_back(w, id, ctx, mp) { w.mobys[id].state = 6; }
        }
        _ => {}
    }
    // `0x3c85a0`.
    let m = &mut w.mobys[id];
    let base = if m.state == 3 { m.pv_f(DOCK_Z) } else { -1.0 };
    bob(m, BOB_AMP, BOB_RATE * DEG, base, BOB_PHASE, BOB_LAST);
    sway(m, SWAY, SWAY_A * DEG, SWAY_B * DEG, SWAY_P, SWAY_Q);
    let (pos, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, BLOCK, sub(pos, old), old_rot, rot);
    let flags = m.pv_i(BLOCK + 0x3c) as u32;
    crate::moby::carry_hero_with(ctx, id, flags, old, old_rot[2], pos, rot[2]);
}
