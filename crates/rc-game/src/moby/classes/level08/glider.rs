//! Ratchet's glider **0x0905** ([`update_0905`], L08 `0x3cf3e8`, L09 `0x3bfb08`, L19 `0x3bded8`: the same code and
//! data; the decomp's `func_US2_L08_003CF3E8`, its functional reconstruction `func_L08_003CDF98`). It waits on its
//! launch pad 0x0988 (crate::moby::classes::level08::props), takes Ratchet off it in his state 0x31
//! (crate::hero::vehicle; he is seen, in his sequences 0x80..0x88) and glides him down to a landing cuboid. L08
//! addresses below; its tuning is `$gp+0xf90..+0x1100` (`0x1aff80..`, the same values on all three levels).
//!
//! **Its pvars**: `+0x00` / `+0x10` the pose (place; roll, pitch, yaw) Ratchet and the glider are put at, `+0x20` the
//! velocity, `+0x40` / `+0x44` / `+0x48` the roll's, the pitch's and the turn's speeds, `+0x50` a timer, `+0x54` the
//! speed, `+0x58` the height it set off at, `+0x5c` the turn (the stick across, sprung), `+0x60` the pitch input (the
//! stick's), `+0x64` the blend into the glide, `+0x68` the pitch's step of the climb (7), `+0x6c` its pad (a moby
//! index), `+0x70..+0x7c` four landing cuboids (0xff none), `+0x80` the landing point, `+0x90` / `+0x94` the distance
//! to it and the speed toward it, `+0x98` a timer (never set), `+0x9c` the take-off's bits, `+0xa0` / `+0xa4` two
//! sound cuboids (0 / 0xff none) and `+0xb4` their bits, `+0xa8` the camera's count, `+0xac` its loop sound, `+0xb0`
//! the sounds' rest, `+0xb8` / `+0xbc` the music at the take-off (stinger, track; −1 none), `+0xc0` / `+0xc4` at the
//! landing, `+0xc8` a cuboid (−1 none) for the camera's pitch. Its sounds are the pad class's (0x0988) table.
//!
//! **Each tick** its ticks in the state counted (to 0x7ffd), its pose kept (for the crash's line).
//! * **0**: the pose its pad's (`+0x6c` > 0), the glider there, the pose 1 up; its sequence 0 (blend 10); update
//!   distance 0xff, hidden, draw distance 0x3ff; 1.
//! * **1** (waiting): without the Glider (item 0x15, `0x1a7b15`) nothing more this tick. Ratchet on its pad (`0x18a15c`
//!   that 0x0988; with no pad, within 2 of the pose and not in 0x31): its sequence 0, 2.
//! * **2** (on the pad): the pose Ratchet's and the glider at him. Off the pad: its sequence 0, the pose its pad's 1
//!   up, 1. The prompt 0x12f0 (owner 7) asked; with it, still 2, ✕ pressed and Ratchet on the pad on the ground
//!   (`0x18a160`, no air ticks): Ratchet to 0x31 (`0x2d31f8(0x31, 0)`) in his sequence 0x87 (blend 3), the velocity
//!   along the pad's forward row at 8/s and up at 9/s, the timer 30, the yaw the pad's, `0x18b650` the glider; 3.
//! * **3** (the jump): its first tick the music `+0xbc` / `+0xb8` and the rest 30, else at the rest's end its sound 6
//!   and the rest 210. Until Ratchet's frame passes 10 nothing moves; then the bit 4, the pitch 37.5°/s toward 70°,
//!   the pose on by the velocity, gravity 9.8/s², falling faster than 5/s the bit 1, his sequence's end the bit 2 and
//!   his sequence 0x81 (blend 15). At the timer's end with the three bits: shown (its sequence 1, blend 5; its sound
//!   0), the timer 120, Ratchet in 0x31 again in 0x81, the pitch 70°, the speed 0.06, still, the camera's count 0 and
//!   its camera ([`camera`]), the height it set off at, the blend 0, the smoothed look's spring 0.04 / 0.2 (the
//!   camera's kept, crate::camera::CamTweaks::look_spring), `0x18b650` the glider; 5. Ratchet put at the pose.
//! * **4** (never set): Ratchet more than 3 away across: its sequence 0, 1.
//! * **5** (the dive): its loop sound (1, looped) kept, its camera; the velocity the speed along the pitch and yaw.
//!   From 75 ticks left it is shown (sequence 1, blend 10), the blend `(75 − timer)/75`, the steering ([`steer`]) by it
//!   and the velocity blended to the glide's. The pose on; a crash ([`crash`]) ends the tick's glide, else the speed
//!   up by 20/s² to 20/s, the pitch down by 46°/s once 30 ticks are gone (not under 0), at the timer's end its
//!   sequence 2 (blend 10), Ratchet's 0x82 (blend 15), the turn speeds 0, the pitch's −46°/s, 6. Its sequence over:
//!   2 (blend 10).
//! * **6** (gliding): its camera, its loop sound, the steering. Ratchet's sequence by the turn (past ∓0.4: 0x85 /
//!   0x86, blend 15, with its sound 2), else by the pitch's speed (up past 80/s·k: 0x83, blend 25; down: 0x81, blend
//!   25, with its sound 3 unless resting, the rest 45; else 0x82, blend 20), each unless playing. The speed: 20 below
//!   where it set off, eased (10%) to 20/s unless `+0x98`; else to `20/s + (1 − f)(4/s − 20/s)` (f the drop past 4
//!   over 16, 0..1; eased only when lower or `+0x98` is 0), at most 20/s. The velocity the glide's (blended in by
//!   0.03 a tick while the blend is under 1), the pose on, the camera's shift `0x1b9d88` half way to its rise. Above 4
//!   under where it set off: its sequence 2 (blend 3), Ratchet's 0x84 (blend 15), the pitch input 0, the timer 15,
//!   the pitch's step to −40° over 15 ticks (none when not above it), 7. Ratchet put at the pose, the crash test;
//!   Ratchet out of 0x31: 1.
//! * **7** (the climb): its camera; at the timer's end the timer 150, 8. Gravity 5/s², the pose on, the speed the
//!   velocity's, the pitch on by its step, the steering, the crash test, Ratchet put at the pose.
//! * **8** (stalled): its camera; pitched down and under 4 below where it set off: Ratchet's 0x82 (blend 15), its
//!   sequence 2 (blend 10), the blend 0, 6; else the timer counted. Gravity 5/s², the pose on, the speed the
//!   velocity's, the steering, Ratchet put at the pose, the crash test.
//! * **9** (crashed): its camera; its first tick the loop sound stopped, its sound 4, the rest 15. The yaw 480°/s,
//!   gravity 9.8/s², the pose on, Ratchet put at the pose, the crash test; rising, under 12 high, slower than 1/s or 240
//!   ticks on: an explosion (flashes 4 and 2 past 9, 10 embers, 3 billows, 16 flares, the light 15, the shake, one
//!   debris), `0x18b650` cleared, Ratchet dead (`0x2c4940` = L00 `0x2b31f0`).
//! * **10** (landing): the loop sound stopped. The pitch and the roll 90°/s toward level; its sequence 3 over: its
//!   sequence 0 (blend 10), hidden. Short of the landing point (by the least of the distance and `+0x94`): the
//!   velocity × 0.85 plus 0.15 of that toward it, the pose on, Ratchet put at the pose. There: the pose the point, the
//!   speed 0; at Ratchet's sequence's end `0x18b650` cleared, the look's spring back, the timer 0, Ratchet idle
//!   (`0x2d31f8(0, 1)`) and still (`0x2c4368` = L00 `0x2b2cd8`), its sequence 0, the pose its pad's, the music `+0xc4`
//!   / `+0xc0`, 1.
//! * **After** (but the waiting 1's early return): the pose held inside 10..1013. In 5..9: drawn within 80 of the
//!   camera its shadow (range 0x4b); its trails (a glow puff at each of the joints 0..6 and 4 behind them along the
//!   velocity, `0x3441c0(j, 0.5, 1.01, randf(24000, 35000), …, 2, velocity/4, 0x0e7f7f7f, 0)`, j 0.07 from 7, a random
//!   roll); in a landing cuboid above it (its height under Ratchet's): its sequence 3 (blend 3), Ratchet's 0x88
//!   (blend 15), the landing point the cuboid's, the distance Ratchet's to it, the speed toward it the velocity's (at
//!   least 1/15), Ratchet's animation speed `33 / (distance / speed)`, 10. Then each sound cuboid entered the first
//!   time (not in 9): its sound 5. The timers `+0x98` and `+0xb0` counted.
//!
//! **The steering** ([`steer`], `0x3ceb58(moby, k)`): the stick across (the pad's, `0x138288`, with the d-pad's
//! pressures) × k, held to ±k (L3 held: × 1.25, the spring's most 0.08, else 0.2). In 6 and 8 the pitch: the input
//! the stick's down (back to it at 0.15 a tick when it falls); past 0.2 the pitch's speed toward the way to 80° × the
//! input (at most 80°/s) by the stick's length × 250″, the pitch on by it; else the pitch sprung level (`0x31f0c8(0,
//! 0.011, 0.3, 0.0145)`). The turn sprung (`0x31ebd8(x, 0.03, 1, most)`) toward the input, the roll sprung (0.05, 1,
//! 1) toward the turn, the yaw 80°/s × the turn.
//!
//! **The crash test** ([`crash`], `0x3cefc8`): a sphere of 0.318 at 0.101 along its forward row and 0.156 along its up
//! from its place, then the lines from there to its joints 1 and 4 (flags 0x10), then the line from the pose it had
//! to the pose (Ratchet's moby left out). A hit: an explosion at the point (3 embers, billows, flares and debris,
//! size 0.8, colour shift 7, no shake), its sound 4 unless resting (the rest 15), the pose out of what it met (a
//! line's: back from its start by its length past the hit). Off level 9 on a surface of kind 1: the look's spring
//! back, the timer 0, Ratchet idle, its sequence 0, the pose its pad's, 1 — and Ratchet's health 1 (`0x18c2ec`).
//! Else the velocity off the face (`0x2ef2a8`) × 0.6 less 29.4/s² of gravity; not yet 9: falling (no rise), Ratchet's
//! 0x80 (blend 15), 9. The glider on by the velocity.
//!
//! **Its camera** ([`camera`], `0x3ce8d8`): the follow camera's look height 0.25 (rate 0.1), distance 4 (rate 1),
//! with Ratchet in `+0xc8` the pitch 30°, the pivot 1 (0.003), the look from the smoothed target, the springs
//! `(0.015 + 0.985 t, 0.2 + 0.8 t)` and `(0.015 + 0.045 t, 0.2)` (t its count over 90, counted to 90), running toward
//! it does not pull it in (crate::camera::FollowAsks). The look target 0x0970 (crate::moby::classes::look_targets) and
//! Grelbin's ride camera (crate::moby::classes::level19::ride_cam) give it the distance 7; Dobbo's leader 0x0e1a runs
//! ahead of it (crate::moby::classes::level09::leader).
//!
//! Not ported: `+0x30` (the joint 8's offset in its frame: nothing reads it), the current camera's `+0x250` (2 at
//! each camera call), the hero moby's `+0x9c` (crate::hero::vehicle), the debug prints, the pitch's other branch
//! (`$gp+0xfdc`, 0 on the disc) and the stick's y with it. Port: the crash's queries leave the glider out, and the
//! look's kept spring (L08 `$gp+0x4420..`) is its pvars `+0xd0`.

use crate::collision::{coll_line_live, coll_sphere_live, CollOutput, QueryFlags};
use crate::hero::physics::{add, add_rot, approach, approach_angle, dist, dist2, dot, len, scale, set_len, spring, sub, sub_rot, turn_spring, unit, V3};
use crate::moby::blast::{self, Explosion};
use crate::moby::classes::props::{set_seq, wrapped};
use crate::moby::classes::shot::polar;
use crate::moby::{countdown, Ctx, MobyWorld};
use crate::pad::button;

pub const CLASS: i16 = 0x0905;
/// Its launch pad, whose sound table it plays.
const PAD: i16 = 0x0988;
/// The Glider (`0x1a7b00 + 0x15`).
const GLIDER: usize = 0x15;

/// The pvars (module docs).
mod pv {
    pub const POS: usize = 0x00;
    pub const ROLL: usize = 0x10;
    pub const PITCH: usize = 0x14;
    pub const YAW: usize = 0x18;
    pub const VEL: usize = 0x20;
    pub const ROLL_V: usize = 0x40;
    pub const PITCH_V: usize = 0x44;
    pub const TURN_V: usize = 0x48;
    pub const TIMER: usize = 0x50;
    pub const SPEED: usize = 0x54;
    pub const START_Z: usize = 0x58;
    pub const TURN: usize = 0x5c;
    pub const PITCH_IN: usize = 0x60;
    pub const BLEND: usize = 0x64;
    pub const CLIMB: usize = 0x68;
    pub const PAD: usize = 0x6c;
    pub const LANDINGS: usize = 0x70;
    pub const HOME: usize = 0x80;
    pub const HOME_D: usize = 0x90;
    pub const HOME_V: usize = 0x94;
    pub const T98: usize = 0x98;
    pub const BITS: usize = 0x9c;
    pub const SOUNDS: usize = 0xa0;
    pub const CAM_T: usize = 0xa8;
    pub const LOOP: usize = 0xac;
    pub const REST: usize = 0xb0;
    pub const SOUND_BITS: usize = 0xb4;
    pub const MUSIC_ON: (usize, usize) = (0xbc, 0xb8);
    pub const MUSIC_OFF: (usize, usize) = (0xc4, 0xc0);
    pub const CAM_CUBOID: usize = 0xc8;
    /// Port: the look's kept spring (L08 `$gp+0x4420` / `0x1b3414`).
    pub const KEPT: usize = 0xd0;
    pub const LEN: usize = 0xd8;
}

const DEG: f32 = 0.017_453_294;
/// Degrees a second as radians a tick.
const DEG_TICK: f32 = 0.000_290_888_25;
const PER_TICK: f32 = 0.016_666_668;
const PER_TICK2: f32 = 0.000_277_777_78;
/// `$gp+0x1100`: the moves on (1 on the disc).
const MOVES: bool = true;

/// `$gp+0xf90..` (L08 `0x1aff80..`).
const LIFT: f32 = 1.0;
const NEAR: f32 = 2.0;
const AWAY: f32 = 3.0;
const TAKE_OFF: i32 = 120;
const BLEND_IN: i32 = 75;
const LEVEL_OUT: i32 = 30;
const DIVE_PITCH: f32 = 70.0;
const TOP_SPEED: f32 = 20.0;
const SPEED_UP: f32 = 20.0;
const PITCH_UP: f32 = 46.0;
const CLIMB_AT: f32 = -4.0;
const SLOW: f32 = 4.0;
const DEEP: f32 = -20.0;
const PITCH_RATE: f32 = 80.0;
const ROLL_K: f32 = 1.0;
const SHIFT_K: f32 = 1.0;
const CLIMB_TICKS: i32 = 15;
const CLIMB_FALL: f32 = 5.0;
const CLIMB_PITCH: f32 = -40.0;
const STALL_TICKS: i32 = 150;
const STALL_PITCH: f32 = 0.0;
const SPIN: f32 = 480.0;
const CRASH_FALL: f32 = 9.8;
const BLEND_STEP: f32 = 0.03;
const LOOK_SPRING: [f32; 2] = [0.04, 0.2];
const JUMP_SPEED: f32 = 8.0;
const JUMP_RISE: f32 = 9.0;
const JUMP_FALLING: f32 = 5.0;
const JUMP_TICKS: i32 = 30;
const JUMP_FALL: f32 = 9.8;
const LAND_SLOW: f32 = 0.85;
const LAND_PULL: f32 = 0.15;
const SPIN_TICKS: u16 = 240;
const JUMP_PITCH_RATE: f32 = 37.5;
const LEVEL_RATE: f32 = 90.0;
const ANIM_LAND: f32 = 33.0;
/// The prompt (owner 7).
const PROMPT: (i32, u32) = (7, 0x12f0);

fn sound(w: &MobyWorld, id: usize, ctx: &mut Ctx, n: i32) -> i32 { ctx.host.play_sound_of(&w.mobys[id], PAD, n, 0) }

/// Its pad (`+0x6c`, the moby table's).
fn pad(w: &MobyWorld, id: usize) -> Option<usize> { usize::try_from(w.mobys[id].pv_i(pv::PAD)).ok().filter(|&p| p < w.mobys.len()) }

/// `0x3cef88`: the glider at the pose.
fn to_pose(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    (m.pos, m.rot) = (m.pv_v(pv::POS), m.pv_v(pv::ROLL));
}

/// The pose its pad's place (`+0x6c` > 0), the glider there (the waiting state's way back).
fn back_to_pad(w: &mut MobyWorld, id: usize) {
    if w.mobys[id].pv_i(pv::PAD) <= 0 { return; }
    let Some(p) = pad(w, id) else { return };
    let at = w.mobys[p].pos;
    w.mobys[id].set_pv_v(pv::POS, at);
    to_pose(w, id);
}

/// `0x3cea98`: Ratchet put at the pose, the glider at him.
fn place(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let (p, r) = (m.pv_v(pv::POS), m.pv_v(pv::ROLL));
    ctx.hero.ride_place(p, r);
    (m.pos, m.rot) = (p, r);
}

/// The pose on by the velocity.
fn advance(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let p = add(m.pv_v(pv::POS), m.pv_v(pv::VEL));
    m.set_pv_v(pv::POS, p);
}

/// Shown (`+0x31` 1, mode bit 1 off) with its sequence 1, when hidden.
fn show(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, blend: u8) -> bool {
    let m = &mut w.mobys[id];
    if m.mode & 1 == 0 { return false; }
    m.mode &= !1;
    m.drawn = 1;
    set_seq(w, id, ctx, 1, 0, blend);
    true
}

/// Its loop sound (1, looped) going.
fn keep_loop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.host.sound_playing(w.mobys[id].pv_i(pv::LOOP)) { return; }
    let h = ctx.host.play_sound_of(&w.mobys[id], PAD, 1, 4);
    w.mobys[id].set_pv_i(pv::LOOP, h);
}

/// Its loop sound stopped.
fn stop_loop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(pv::LOOP);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(pv::LOOP, -1);
}

/// The smoothed look's spring back (`0x1b2044` / `0x1b2048` from the kept).
fn spring_back(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    ctx.game.cam_tweaks.look_spring = [m.pv_f(pv::KEPT), m.pv_f(pv::KEPT + 4)];
}

/// Ratchet's sequence `s` asked for (`0x2de288(s, 0, blend)` = L00 `0x2d2508`).
fn hero_seq(ctx: &mut Ctx, s: u8, blend: f32) { ctx.hero.anim_req = Some((blend, s, 0)); }

/// Ratchet's moby plays `s` (its `+0x42` or `+0x43`).
fn hero_plays(ctx: &Ctx, s: u8) -> bool { ctx.hero_view.seq_a == s || ctx.hero_view.seq_b == s }

/// The glide's velocity: the speed along the pitch and yaw (`0x333c00(speed, yaw, −pitch)` = L00 `0x2af000`).
fn glide(w: &MobyWorld, id: usize) -> V3 {
    let m = &w.mobys[id];
    polar(m.pv_f(pv::SPEED), m.pv_f(pv::YAW), -m.pv_f(pv::PITCH))
}

/// 0x0905 (`0x3cf3e8`): see the module.
pub fn update_0905(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < pv::LEN { w.mobys[id].pvars.resize(pv::LEN, 0); }
    let saved = w.mobys[id].pv_v(pv::POS);
    {
        let m = &mut w.mobys[id];
        if (m.state_ticks as u16) < 0x7ffd { m.state_ticks = m.state_ticks.wrapping_add(1); }
    }
    match w.mobys[id].state {
        0 => {
            if let Some(p) = pad(w, id).filter(|_| 0 < w.mobys[id].pv_i(pv::PAD)) {
                let (at, rot) = (w.mobys[p].pos, w.mobys[p].rot);
                w.mobys[id].set_pv_v(pv::POS, at);
                w.mobys[id].set_pv_v(pv::ROLL, rot);
                to_pose(w, id);
            }
            let m = &mut w.mobys[id];
            let z = m.pv_f(pv::POS + 8);
            m.set_pv_f(pv::POS + 8, z + LIFT);
            set_seq(w, id, ctx, 0, 0, 10);
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.drawn = 0;
            m.draw_dist = 0x3ff;
            m.mode |= 1;
            m.set_state(1, -1);
            m.set_pv_i(pv::T98, 0);
            m.set_pv_i(pv::REST, 0);
        }
        1 => {
            if ctx.game.items.get(GLIDER).is_none_or(|&b| b == 0) { return; }
            let on = if w.mobys[id].pv_i(pv::PAD) < 0 {
                dist(w.mobys[id].pv_v(pv::POS), ctx.hero.pos) < NEAR && ctx.hero.state != 0x31
            } else {
                on_pad(w, id, ctx)
            };
            if on {
                set_seq(w, id, ctx, 0, 0, 10);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => on_pad_tick(w, id, ctx),
        3 => jump(w, id, ctx),
        4 => {
            if AWAY < dist2(w.mobys[id].pos, ctx.hero.pos) {
                set_seq(w, id, ctx, 0, 0, 10);
                w.mobys[id].set_state(1, -1);
            }
        }
        5 => dive(w, id, ctx, saved),
        6 => gliding(w, id, ctx, saved),
        7 => {
            camera(w, id, ctx);
            let mut t = w.mobys[id].pv_i(pv::TIMER);
            if countdown(&mut t) {
                t = STALL_TICKS;
                w.mobys[id].set_state(8, -1);
            }
            let m = &mut w.mobys[id];
            m.set_pv_i(pv::TIMER, t);
            let vz = m.pv_f(pv::VEL + 8);
            m.set_pv_f(pv::VEL + 8, vz + -CLIMB_FALL * PER_TICK2);
            advance(w, id);
            let m = &mut w.mobys[id];
            m.set_pv_f(pv::SPEED, len(m.pv_v(pv::VEL)));
            if MOVES { m.set_pv_f(pv::PITCH, add_rot(m.pv_f(pv::PITCH), m.pv_f(pv::CLIMB))); }
            steer(w, id, ctx, 1.0);
            crash(w, id, ctx, saved);
            place(w, id, ctx);
        }
        8 => {
            camera(w, id, ctx);
            let m = &mut w.mobys[id];
            if STALL_PITCH * DEG < m.pv_f(pv::PITCH) && m.pv_f(pv::POS + 8) - m.pv_f(pv::START_Z) < CLIMB_AT {
                hero_seq(ctx, 0x82, 15.0);
                set_seq(w, id, ctx, 2, 0, 10);
                let m = &mut w.mobys[id];
                m.set_pv_f(pv::BLEND, 0.0);
                m.set_state(6, -1);
            } else {
                let mut t = m.pv_i(pv::TIMER);
                countdown(&mut t);
                m.set_pv_i(pv::TIMER, t);
            }
            let m = &mut w.mobys[id];
            let vz = m.pv_f(pv::VEL + 8);
            m.set_pv_f(pv::VEL + 8, vz + -CLIMB_FALL * PER_TICK2);
            if MOVES { advance(w, id); }
            let m = &mut w.mobys[id];
            m.set_pv_f(pv::SPEED, len(m.pv_v(pv::VEL)));
            steer(w, id, ctx, 1.0);
            place(w, id, ctx);
            crash(w, id, ctx, saved);
        }
        9 => crashed(w, id, ctx, saved),
        10 => landing(w, id, ctx),
        _ => {}
    }
    after(w, id, ctx);
}

/// Ratchet on its pad (`0x18a15c` its pad, a 0x0988).
fn on_pad(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let Some(g) = ctx.hero.ground_moby else { return false };
    w.mobys.get(g).is_some_and(|m| m.o_class == PAD) && pad(w, id) == Some(g)
}

/// 2 (module docs).
fn on_pad_tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    // `0x3ceb18`: the pose Ratchet's, the glider at him.
    let (hp, hr) = (ctx.hero.pos, ctx.hero.rot);
    let m = &mut w.mobys[id];
    m.set_pv_v(pv::POS, hp);
    m.set_pv_v(pv::ROLL, hr);
    (m.pos, m.rot) = (hp, hr);
    if !ctx.hero.ground_moby.and_then(|g| w.mobys.get(g)).is_some_and(|g| g.o_class == PAD) {
        set_seq(w, id, ctx, 0, 0, 10);
        if let Some(p) = pad(w, id) {
            let at = w.mobys[p].pos;
            w.mobys[id].set_pv_v(pv::POS, at);
        }
        let m = &mut w.mobys[id];
        let z = m.pv_f(pv::POS + 8);
        m.set_pv_f(pv::POS + 8, z + LIFT);
        m.set_state(1, -1);
    }
    let ready = ctx.game.prompt.request(PROMPT.0, PROMPT.1, 1) != 0;
    if w.mobys[id].state != 2 || ctx.hero.items.pressed & button::CROSS == 0 || !ready { return; }
    let Some(p) = pad(w, id) else { return };
    if ctx.hero.ground_moby != Some(p) || ctx.hero.grounded_ticks == 0 || ctx.hero.air_ticks != 0 { return; }
    ctx.hero.request_state(0x31, false);
    hero_seq(ctx, 0x87, 3.0);
    let (fwd, yaw) = (w.mobys[p].rows[0], w.mobys[p].rot[2]);
    let m = &mut w.mobys[id];
    let mut v = set_len(fwd, JUMP_SPEED * PER_TICK);
    v[2] = JUMP_RISE * PER_TICK;
    m.set_pv_v(pv::VEL, v);
    m.set_pv_i(pv::TIMER, JUMP_TICKS);
    m.set_pv_f(pv::YAW, yaw);
    ctx.hero.vehicle.moby = Some(id);
    m.set_pv_i(pv::BITS, 0);
    m.set_state(3, -1);
}

/// 3 (module docs).
fn jump(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        let m = &w.mobys[id];
        let (track, stinger) = (m.pv_i(pv::MUSIC_ON.0), m.pv_i(pv::MUSIC_ON.1));
        if track != -1 { ctx.host.music_request(track, stinger); }
        w.mobys[id].set_pv_i(pv::REST, 30);
    } else {
        let mut t = w.mobys[id].pv_i(pv::REST);
        if countdown(&mut t) {
            sound(w, id, ctx, 6);
            t = 210;
        }
        w.mobys[id].set_pv_i(pv::REST, t);
    }
    let mut bits = w.mobys[id].pv_i(pv::BITS);
    if bits == 0 && !(10.0 < ctx.hero.anim_frame) {
        w.mobys[id].set_pv_i(pv::SOUND_BITS, 0);
        return;
    }
    bits |= 4;
    let m = &mut w.mobys[id];
    let mut pitch = m.pv_f(pv::PITCH);
    approach_angle(DIVE_PITCH * DEG, JUMP_PITCH_RATE * DEG_TICK, &mut pitch, 0);
    m.set_pv_f(pv::PITCH, pitch);
    if MOVES { advance(w, id); }
    let m = &mut w.mobys[id];
    let vz = m.pv_f(pv::VEL + 8) - JUMP_FALL * PER_TICK2;
    m.set_pv_f(pv::VEL + 8, vz);
    if vz < -(JUMP_FALLING * PER_TICK) { bits |= 1; }
    if ctx.hero_view.flags & 2 != 0 {
        bits |= 2;
        hero_seq(ctx, 0x81, 15.0);
    }
    m.set_pv_i(pv::BITS, bits);
    let mut t = m.pv_i(pv::TIMER);
    let out = countdown(&mut t);
    m.set_pv_i(pv::TIMER, t);
    if out && bits == 7 {
        if show(w, id, ctx, 5) { sound(w, id, ctx, 0); }
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::TIMER, TAKE_OFF);
        ctx.hero.request_state(0x31, false);
        hero_seq(ctx, 0x81, 15.0);
        if MOVES { m.set_pv_f(pv::PITCH, DIVE_PITCH * DEG); }
        m.set_pv_f(pv::SPEED, f32::from_bits(0x3d75_c290));
        m.set_pv_v(pv::VEL, [0.0; 3]);
        m.set_pv_i(pv::CAM_T, 0);
        let z = m.pv_f(pv::POS + 8);
        m.set_pv_f(pv::START_Z, z);
        camera(w, id, ctx);
        let m = &mut w.mobys[id];
        m.set_pv_f(pv::BLEND, 0.0);
        let kept = ctx.game.cam_tweaks.look_spring;
        m.set_pv_f(pv::KEPT, kept[0]);
        m.set_pv_f(pv::KEPT + 4, kept[1]);
        ctx.game.cam_tweaks.look_spring = LOOK_SPRING;
        ctx.hero.vehicle.moby = Some(id);
        m.set_state(5, -1);
    }
    place(w, id, ctx);
    w.mobys[id].set_pv_i(pv::SOUND_BITS, 0);
}

/// 5 (module docs).
fn dive(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, saved: V3) {
    keep_loop(w, id, ctx);
    camera(w, id, ctx);
    let m = &mut w.mobys[id];
    let (pitch, yaw, speed) = (m.pv_f(pv::PITCH), m.pv_f(pv::YAW), m.pv_f(pv::SPEED));
    let t = speed * pitch.cos();
    m.set_pv_v(pv::VEL, [t * yaw.cos(), t * yaw.sin(), -pitch.sin() * speed]);
    if m.pv_i(pv::TIMER) <= BLEND_IN {
        show(w, id, ctx, 10);
        let m = &mut w.mobys[id];
        let mut b = (BLEND_IN - m.pv_i(pv::TIMER)) as f32 / BLEND_IN as f32;
        if 1.0 < b { b = 1.0; } else if b < 0.0 { b = 0.0; }
        m.set_pv_f(pv::BLEND, b);
        steer(w, id, ctx, b);
        let g = glide(w, id);
        let m = &mut w.mobys[id];
        let v = add(scale(m.pv_v(pv::VEL), 1.0 - b), scale(g, b));
        m.set_pv_v(pv::VEL, v);
    }
    if MOVES { advance(w, id); }
    if !crash(w, id, ctx, saved) {
        let m = &mut w.mobys[id];
        let mut s = m.pv_f(pv::SPEED) + SPEED_UP * PER_TICK2;
        if TOP_SPEED * PER_TICK < s { s = TOP_SPEED * PER_TICK; }
        m.set_pv_f(pv::SPEED, s);
        if m.pv_i(pv::TIMER) < TAKE_OFF - LEVEL_OUT && MOVES { m.set_pv_f(pv::PITCH, sub_rot(m.pv_f(pv::PITCH), PITCH_UP * DEG_TICK)); }
        if m.pv_f(pv::PITCH) < 0.0 { m.set_pv_f(pv::PITCH, 0.0); }
        let mut t = m.pv_i(pv::TIMER);
        let out = countdown(&mut t);
        m.set_pv_i(pv::TIMER, t);
        if out {
            set_seq(w, id, ctx, 2, 0, 10);
            hero_seq(ctx, 0x82, 15.0);
            let m = &mut w.mobys[id];
            m.set_pv_f(pv::TURN_V, 0.0);
            m.set_pv_f(pv::ROLL_V, 0.0);
            m.set_pv_f(pv::PITCH_V, -(PITCH_UP * DEG_TICK));
            m.set_state(6, -1);
        }
    }
    if wrapped(w, id, ctx) { set_seq(w, id, ctx, 2, 0, 10); }
    place(w, id, ctx);
}

/// 6 (module docs).
fn gliding(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, saved: V3) {
    camera(w, id, ctx);
    keep_loop(w, id, ctx);
    steer(w, id, ctx, 1.0);
    let turn = w.mobys[id].pv_f(pv::TURN);
    let edge = f32::from_bits(0x3ecc_cccd);
    if turn < -edge || edge < turn {
        let s = if turn < -edge { 0x85 } else { 0x86 };
        if !hero_plays(ctx, s) {
            hero_seq(ctx, s, 15.0);
            sound(w, id, ctx, 2);
        }
    } else {
        let r = w.mobys[id].pv_f(pv::PITCH_V);
        let most = PITCH_RATE * f32::from_bits(0x382e_4bd8);
        if most < r {
            if !hero_plays(ctx, 0x83) { hero_seq(ctx, 0x83, 25.0); }
        } else if r < -most {
            if !hero_plays(ctx, 0x81) {
                hero_seq(ctx, 0x81, 25.0);
                if w.mobys[id].pv_i(pv::REST) == 0 {
                    sound(w, id, ctx, 3);
                    w.mobys[id].set_pv_i(pv::REST, 0x2d);
                }
            }
        } else if !hero_plays(ctx, 0x82) {
            hero_seq(ctx, 0x82, 20.0);
        }
    }
    let m = &mut w.mobys[id];
    let (z, z0) = (m.pv_f(pv::POS + 8), m.pv_f(pv::START_Z));
    let mut s = m.pv_f(pv::SPEED);
    let top = TOP_SPEED * PER_TICK;
    let ease = 0.1;
    if z - z0 <= DEEP {
        if m.pv_i(pv::T98) == 0 { s += (top - s) * ease; }
    } else {
        let mut f = ((z0 - z) + CLIMB_AT) / (CLIMB_AT - DEEP);
        if 1.0 < f { f = 1.0; } else if f < 0.0 { f = 0.0; }
        let t = top + (1.0 - f) * (SLOW * PER_TICK - top);
        if m.pv_i(pv::T98) == 0 || t < s { s += (t - s) * ease; }
        if top < s { s = top; }
    }
    m.set_pv_f(pv::SPEED, s);
    let b = m.pv_f(pv::BLEND);
    let g = glide(w, id);
    let m = &mut w.mobys[id];
    if b < 1.0 {
        let mut b = b + BLEND_STEP;
        if 1.0 < b { b = 1.0; }
        m.set_pv_f(pv::BLEND, b);
        let v = add(scale(m.pv_v(pv::VEL), 1.0 - b), scale(g, b));
        m.set_pv_v(pv::VEL, v);
    } else {
        m.set_pv_v(pv::VEL, g);
    }
    if MOVES { advance(w, id); }
    // The camera's shift `0x1b9d88` (L00 `0x1b8fc8`) half way to its rise.
    let vz = w.mobys[id].pv_f(pv::VEL + 8);
    let sh = &mut ctx.game.cam_tweaks.shift[2];
    *sh += (vz * SHIFT_K - *sh) * 0.5;
    let m = &mut w.mobys[id];
    if CLIMB_AT < m.pv_f(pv::POS + 8) - m.pv_f(pv::START_Z) {
        set_seq(w, id, ctx, 2, 0, 3);
        hero_seq(ctx, 0x84, 15.0);
        let m = &mut w.mobys[id];
        let (t, f) = (m.pv_f(pv::PITCH), CLIMB_PITCH * DEG);
        m.set_pv_f(pv::PITCH_IN, 0.0);
        m.set_pv_i(pv::TIMER, CLIMB_TICKS);
        m.set_pv_f(pv::CLIMB, if f < t { sub_rot(f, t) / CLIMB_TICKS as f32 } else { 0.0 });
        m.set_state(7, -1);
    }
    place(w, id, ctx);
    crash(w, id, ctx, saved);
    if ctx.hero.state != 0x31 { w.mobys[id].set_state(1, -1); }
}

/// 9 (module docs).
fn crashed(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, saved: V3) {
    camera(w, id, ctx);
    if w.mobys[id].first() {
        stop_loop(w, id, ctx);
        sound(w, id, ctx, 4);
        w.mobys[id].set_pv_i(pv::REST, 15);
    }
    let m = &mut w.mobys[id];
    if MOVES { m.set_pv_f(pv::YAW, add_rot(m.pv_f(pv::YAW), SPIN * DEG_TICK)); }
    let vz = m.pv_f(pv::VEL + 8);
    m.set_pv_f(pv::VEL + 8, vz + -CRASH_FALL * PER_TICK2);
    if MOVES { advance(w, id); }
    place(w, id, ctx);
    crash(w, id, ctx, saved);
    let m = &w.mobys[id];
    let done = 0.0 < m.pv_f(pv::VEL + 8) || m.pv_f(pv::POS + 8) < 12.0 || len(m.pv_v(pv::VEL)) < PER_TICK || SPIN_TICKS < m.state_ticks as u16;
    if !done { return; }
    let e = Explosion { flash: 4.0, flash2: 2.0, near: 9.0, size: 1.0, light: 15.0, embers: 10, billows: 3, flares: 0x10, sound: -1, shake: true, debris: 1, lod: -1, flags: 0x81_0001, ..Default::default() };
    blast::explode(w, ctx, Some(id), None, &e);
    ctx.hero.vehicle.moby = None;
    ctx.hero.die();
}

/// 10 (module docs).
fn landing(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    stop_loop(w, id, ctx);
    let m = &mut w.mobys[id];
    let home = m.pv_v(pv::HOME);
    let d = dist(m.pv_v(pv::POS), home);
    let lim = d.min(m.pv_f(pv::HOME_V));
    if MOVES {
        let (mut p, mut r) = (m.pv_f(pv::PITCH), m.pv_f(pv::ROLL));
        approach_angle(0.0, LEVEL_RATE * DEG_TICK, &mut p, 0);
        approach_angle(0.0, LEVEL_RATE * DEG_TICK, &mut r, 0);
        m.set_pv_f(pv::PITCH, p);
        m.set_pv_f(pv::ROLL, r);
    }
    let seq_a = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
    if wrapped(w, id, ctx) && seq_a == 3 {
        set_seq(w, id, ctx, 0, 0, 10);
        let m = &mut w.mobys[id];
        m.drawn = 0;
        m.mode |= 1;
    }
    let m = &mut w.mobys[id];
    if lim < d {
        let to = set_len(sub(home, m.pv_v(pv::POS)), lim * LAND_PULL);
        let v = add(scale(m.pv_v(pv::VEL), LAND_SLOW), to);
        m.set_pv_v(pv::VEL, v);
        if MOVES { advance(w, id); }
        place(w, id, ctx);
        return;
    }
    m.set_pv_v(pv::POS, home);
    m.set_pv_f(pv::HOME_V, 0.0);
    if ctx.hero_view.flags & 2 == 0 { return; }
    ctx.hero.vehicle.moby = None;
    spring_back(w, id, ctx);
    w.mobys[id].set_pv_i(pv::TIMER, 0);
    ctx.hero.request_state(0, true);
    ctx.hero.stop_motion();
    set_seq(w, id, ctx, 0, 0, 10);
    back_to_pad(w, id);
    let m = &w.mobys[id];
    let (track, stinger) = (m.pv_i(pv::MUSIC_OFF.0), m.pv_i(pv::MUSIC_OFF.1));
    if track != -1 { ctx.host.music_request(track, stinger); }
    w.mobys[id].set_state(1, -1);
}

/// After the states (module docs).
fn after(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let mut p = m.pv_v(pv::POS);
    for c in &mut p {
        if *c < 10.0 { *c = 10.0; }
        if 1013.0 < *c { *c = 1013.0; }
    }
    m.set_pv_v(pv::POS, p);
    if (5..10).contains(&m.state) {
        crate::shadow_volumes::probe_near(w, id, ctx, 80.0, 0x4b);
        trails(w, id, ctx);
        landings(w, id, ctx);
    }
    for k in 0..2 {
        let m = &w.mobys[id];
        let c = m.pv_i(pv::SOUNDS + 4 * k);
        if c == 0 || c == 0xff || (m.pv_i(pv::SOUND_BITS) >> k) & 1 != 0 || !ctx.level.in_cuboid(c, m.pv_v(pv::POS)) { continue; }
        if m.state == 9 { continue; }
        sound(w, id, ctx, 5);
        let m = &mut w.mobys[id];
        let b = m.pv_i(pv::SOUND_BITS) | 1 << k;
        m.set_pv_i(pv::SOUND_BITS, b);
    }
    let m = &mut w.mobys[id];
    for off in [pv::T98, pv::REST] {
        let mut t = m.pv_i(off);
        countdown(&mut t);
        m.set_pv_i(off, t);
    }
}

/// The trails (module docs).
fn trails(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let vel = m.pv_v(pv::VEL);
    let drift = scale(vel, 0.25);
    let jitter = if 7 <= m.state { f32::from_bits(0x3d8f_5c29) } else { 0.0 };
    let joints: Vec<V3> = (0..7).map(|j| ctx.host.joint(m, j).unwrap_or(m.pos)).collect();
    for base in joints {
        let mut f = 0.0f32;
        loop {
            let at = add(scale(vel, -f), base);
            let size = ctx.game.rng.randf(24_000.0, 35_000.0);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                let g = &mut ctx.game.rng;
                if let Some(i) = crate::particles::puff(sys, g, jitter, 0.5, f32::from_bits(0x3f81_47ae), size, at, 2, drift, 0x0e7f_7f7f, 0) {
                    sys.pool.recs[i][8] = g.rand() as u8;
                }
            }
            f += 0.2;
            if !(f < 1.0) { break; }
        }
    }
}

/// The landing cuboids (module docs).
fn landings(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for k in 0..4 {
        let m = &w.mobys[id];
        let c = m.pv_i(pv::LANDINGS + 4 * k);
        if c == 0xff || !ctx.level.in_cuboid(c, m.pv_v(pv::POS)) { continue; }
        let Some(cub) = usize::try_from(c).ok().and_then(|c| ctx.level.cuboids.get(c)) else { continue };
        let home = [cub.matrix[3][0], cub.matrix[3][1], cub.matrix[3][2]];
        if !(home[2] < ctx.hero.pos[2]) { continue; }
        set_seq(w, id, ctx, 3, 0, 3);
        hero_seq(ctx, 0x88, 15.0);
        let m = &mut w.mobys[id];
        m.set_pv_v(pv::HOME, home);
        let d = dist(ctx.hero.pos, home);
        m.set_pv_f(pv::HOME_D, d);
        let l = len(m.pv_v(pv::VEL));
        let v = if f32::from_bits(0x3d88_8889) < l { l } else { f32::from_bits(0x3d88_8889) };
        m.set_pv_f(pv::HOME_V, v);
        ctx.hero.anim_speed = ANIM_LAND / (d / v);
        m.set_state(10, -1);
    }
}

/// `0x3ce8d8(&count, moby)`: its camera (module docs).
fn camera(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let n = m.pv_i(pv::CAM_T);
    let c = m.pv_i(pv::CAM_CUBOID);
    let t = &mut ctx.game.cam_tweaks;
    t.follow.look_h = Some((0.25, 0.1, false));
    t.follow_dist = Some((4.0, 1.0));
    if 0 <= c && ctx.level.in_cuboid(c, ctx.hero.pos) { t.follow.pitch = Some(f32::from_bits(0x3f06_0a93)); }
    t.follow_pivot = Some((1.0, f32::from_bits(0x3b44_9ba6)));
    t.follow.look_from_s = true;
    let mut k = n as f32 / 90.0;
    if 1.0 < k { k = 1.0; }
    t.follow.h_spring = Some((k * 0.985 + 0.015, k * 0.8 + 0.2));
    t.follow.v_spring = Some((k * 0.045 + 0.015, 0.2));
    t.follow.lock_toward = true;
    if n < 90 { m.set_pv_i(pv::CAM_T, n + 1); }
}

/// `0x3ceb58(moby, k)`: the steering (module docs).
fn steer(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: f32) {
    let state = w.mobys[id].state;
    let dp = ctx.hero.vehicle.dpad;
    let mut x = ((ctx.hero.pad_stick[0] - dp[1]) + dp[0]) * k;
    if k < x { x = k; } else if x < -k { x = -k; }
    let mut most = 0.2;
    if ctx.hero.items.held & button::L3 != 0 {
        x = x / 80.0 * 100.0;
        most = f32::from_bits(0x3da3_d70a);
    }
    let m = &mut w.mobys[id];
    if state != 7 && 6 <= state {
        let sy = ctx.hero.stick[1];
        let mut input = m.pv_f(pv::PITCH_IN);
        if sy.abs() < input.abs() { approach(-sy, 0.15, &mut input); } else { input = -sy; }
        m.set_pv_f(pv::PITCH_IN, input);
        let want = input * DEG * PITCH_RATE;
        let (mut p, mut v) = (m.pv_f(pv::PITCH), m.pv_f(pv::PITCH_V));
        if 0.2 < input.abs() {
            let top = PITCH_RATE * DEG_TICK;
            let mut d = sub_rot(want, p);
            if top < d { d = top; } else if d < -top { d = -top; }
            approach(d, 250.0 * (ctx.hero.stick_mag * 4.848_137_5e-6), &mut v);
            p = add_rot(p, v);
        } else {
            turn_spring(0.0, f32::from_bits(0x3c34_3958), f32::from_bits(0x3e99_999a), f32::from_bits(0x3c6e_4bb0), &mut p, &mut v, 0);
        }
        m.set_pv_f(pv::PITCH, p);
        m.set_pv_f(pv::PITCH_V, v);
    }
    let (mut turn, mut tv) = (m.pv_f(pv::TURN), m.pv_f(pv::TURN_V));
    spring(x, 0.03, 1.0, most, &mut turn, &mut tv);
    m.set_pv_f(pv::TURN, turn);
    m.set_pv_f(pv::TURN_V, tv);
    if MOVES {
        let (mut r, mut rv) = (m.pv_f(pv::ROLL), m.pv_f(pv::ROLL_V));
        turn_spring(turn * ROLL_K, 0.05, 1.0, 1.0, &mut r, &mut rv, 0);
        m.set_pv_f(pv::ROLL, r);
        m.set_pv_f(pv::ROLL_V, rv);
        m.set_pv_f(pv::YAW, sub_rot(m.pv_f(pv::YAW), turn * DEG_TICK * PITCH_RATE));
    }
}

/// `0x2ef2a8`: `v` off the face of normal `n` (mirrored when it goes into it).
fn mirror(v: V3, n: V3) -> V3 {
    let u = unit(n);
    let d = dot(v, u);
    if d < 0.0 { sub(v, scale(u, 2.0 * d)) } else { v }
}

/// `0x3cefc8(moby, &pose before)`: the crash test (module docs); true on a hit.
fn crash(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, saved: V3) -> bool {
    let Some(coll) = ctx.coll else { return false };
    let hero = ctx.hero_moby;
    let live = |i: usize| i != id;
    let m = &w.mobys[id];
    let probe = add(add(scale(m.rows[0], f32::from_bits(0x3dce_d917)), m.pos), scale(m.rows[2], f32::from_bits(0x3e1f_be77)));
    let hit: Option<(CollOutput, V3, V3)> = match coll_sphere_live(coll, ctx.scene, probe, f32::from_bits(0x3ea2_d0e5), QueryFlags(0), hero, &live) {
        Some(o) => {
            let pushed = o.pushed_centre.unwrap_or(probe);
            Some((o, pushed, probe))
        }
        None => {
            let ends = [(probe, ctx.host.joint(m, 1).unwrap_or(m.pos)), (probe, ctx.host.joint(m, 4).unwrap_or(m.pos)), (saved, m.pv_v(pv::POS))];
            ends.into_iter().find_map(|(a, b)| {
                let o = coll_line_live(coll, ctx.scene, a, b, QueryFlags(0x10), hero, &live)?;
                let back = set_len(sub(a, b), dist(a, b) - dist(a, o.point));
                Some((o, add(back, a), a))
            })
        }
    };
    let Some((o, pushed, start)) = hit else { return false };
    let e = Explosion { near: 9.0, size: 0.8, embers: 3, billows: 3, flares: 3, sound: -1, debris: 3, lod: -1, colour: 7, flags: 0x81_0001, ..Default::default() };
    blast::explode(w, ctx, Some(id), Some(o.point), &e);
    if w.mobys[id].pv_i(pv::REST) == 0 {
        sound(w, id, ctx, 4);
        w.mobys[id].set_pv_i(pv::REST, 15);
    }
    let m = &mut w.mobys[id];
    let pushed = add(pushed, sub(m.pv_v(pv::POS), start));
    if ctx.game.level != 9 && o.surface_id() == 1 {
        spring_back(w, id, ctx);
        w.mobys[id].set_pv_i(pv::TIMER, 0);
        ctx.hero.request_state(0, true);
        set_seq(w, id, ctx, 0, 0, 10);
        back_to_pad(w, id);
        w.mobys[id].set_state(1, -1);
        ctx.hero.hp = 1;
        return true;
    }
    m.set_pv_v(pv::POS, pushed);
    let mut v = scale(mirror(m.pv_v(pv::VEL), o.normal), 0.6);
    let fall = 29.4 * PER_TICK2;
    v[2] -= fall;
    if m.state != 9 {
        if 0.0 < v[2] { v[2] = -fall; }
        m.set_pv_v(pv::VEL, v);
        hero_seq(ctx, 0x80, 15.0);
        m.set_state(9, -1);
    } else {
        m.set_pv_v(pv::VEL, v);
    }
    m.pos = add(m.pos, v);
    true
}
