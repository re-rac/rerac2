//! Dobbo's searchlight patrol **0x0d30** (`0x400398`, 8 placed; the decomp's `func_L09_003FEF00` and its helpers
//! `func_L09_003FDA38`.. `func_L09_003FECF0` at our `0x3feec8`.. `0x400180`): a hovering robot that sweeps a
//! searchlight along a path while it rides its own path in step with it; the light catching Ratchet (or a 0x00cb,
//! 0x076c or 0x0eb0) alarms its group, the light chases him and the robot fires bolts at him; shot down it breaks
//! up, tumbles and blows up. One of the group (`+0x3d6` 1) is no robot: it draws the group's sirens. On the
//! framework (crate::moby::ai): health `+0x20` (its status `+0x2e`, its damage byte `+0x34` its shots'), target
//! block `+0x140` (its point `+0x140`, moby `+0x150`, found `+0x154`), reaction `+0x160` (a hurt to state 2, sub 2;
//! its hitter `+0x1d0`). Its movement block (`+0x50`) moves nothing.
//!
//! **Its pvars:** `+0x210` / `+0x290` two aim records (joint lists 0 and 11: the first asked the light's yaw and
//! pitch, the second spun 3° a tick), `+0x310` its alarm (those of its group with the same are alarmed together),
//! `+0x314` its path and `+0x318` the light's (the robot's place along its path the share the light has come along
//! its own: [`ride`]), `+0x324` / `+0x3ec` the light's segment and distance along it, `+0x32a` the paths loop (else the
//! light turns at the ends), `+0x330` the spot (the light's place as last lit), `+0x340` the frame of its joint 1
//! (the lamp; `+0x370` its place), `+0x380` the siren's turn, `+0x388` the light's speed (signed), `+0x38c` / `+0x3b0`
//! the two paths' lengths (the closing segment left out), `+0x390` the light and `+0x3a0` its velocity, `+0x3ac` the
//! moby it chases (`+0x44c` its class), `+0x3b4` its own velocity, `+0x3c0..+0x3cc` the bob and sway, `+0x3d0` the
//! yaw it turns to (every 260 ticks ±120°, `+0x3d4`, `+0x3d8` the turn's speed), `+0x3dc` / `+0x3e0` the light's
//! yaw and pitch from it, `+0x3f0` its loop sound, `+0x3f4` the siren's place, `+0x400` the fall's velocity, `+0x410`
//! the fall's ticks (180), `+0x412` its patience on the alarm (600, counted down while its target is not found),
//! `+0x414..+0x41c` the fall's spins, `+0x420` lit, `+0x422` alarmed (the light red), `+0x424` its group's size (the
//! drawer's), `+0x426` / `+0x428` the burst's and the shot's waits, `+0x429` the gun (joint 4 or 5), `+0x42a` the
//! burst's shots, `+0x42b` firing (the spot's flash), `+0x42c` what the drawer draws (2 the light, 8 the siren),
//! `+0x430` where along its paths it starts (a share; 0: their starts), `+0x434` the spot's size, `+0x438` how far it
//! is drawn toward the camera, `+0x440` alarmed by its group, `+0x441` the light held by its fence, `+0x444` the
//! light's fence path (−1 none), `+0x448` its target area.
//!
//! **The update** (`0x400398`): nothing on another chunk's side of the level's planes
//! (crate::level::LevelData::off_chunk; Dobbo has none). The set-up ([`setup`], `0x3ffb00`) in state 0. Its status 2:
//! its bolts (`0x316fe8` = L00 `0x31adc0`) and gone. State 5 (the drawer) its draw on the second list and no more.
//! Else the frame's damage (`0x324590` = L00 `0x327e38`, crate::moby::ai::take_damage; hurt, its group alarmed:
//! [`alarm`], `0x3ffe08`) and the target search (`0x3275b8` = L00 `0x32ad38`: its area, no range); in 1..3 unlit;
//! then the state and the tail.
//! * **1** the patrol (glow 0xffaaaaaa; its loop sound 0): alarmed by its group → 3 (the alert's loop, sound 2, its
//!   patience 600). Lit ([`light`], `0x3ff550`), the light along its path ([`sweep`], `0x3ff0e8`), the robot along its
//!   own ([`ride`], `0x3ff210`), the hover ([`hover`], `0x3ff368`); lit, a sphere at the middle of the beam (0.48 of
//!   the spot's size; none there: at the spot, 0.9) meeting Ratchet or a 0x00cb / 0x076c / 0x0eb0: its group alarmed,
//!   the chase after it (3).
//! * **2** hurt: dead → sequence 0, its sound let go, 4; at the hurt's end sequence 0, its patience, 3.
//! * **3** the chase (glow 0xff00ffff): its moby there (its class still, not gone): lit and alarmed, lit; the light
//!   springs (per axis `(sin a + 1.5)·0.003`, 0.2) to the floor under it (a line 0.5 up to 3.5 down, flags 0x12; none:
//!   its place) swinging by `sin` / `cos` of a phase (the frame plus 36 × its uid, by 1/40 and 1/60), kept from its
//!   fence (`0x3408d0` = L00 `0x343678`, 1.3 × the spot's size; held: its velocity what it moved). Else the target's
//!   moby. The light's place on its path (`0x340530` = L00 `0x3432d8`, within 20 + 5), the robot along its own; in
//!   view (4) the siren at joint 8 and the hover. Not found and its patience out: 1 and its loop sound 0. Found:
//!   the target's moby chased and firing.
//! * **4** shot down: its fall's ticks, its group alarmed, its sound let go, the explosion (crate::moby::blast::explode:
//!   5 embers, 2 billows, 4 flares, flashes 4 / 2, near 9, its sound 1, light 15, shaking, 5 pieces of debris); away
//!   from Ratchet at 0.075..0.2 a tick turned ±40..70° about his down, 0.1..0.2 up; its pieces (`0x3238a8` = L00
//!   `0x327498`, crate::moby::shards::break_into: 3..6 / 3..6, mask 0x7f), its size a tenth (its first aim's scale 10),
//!   spins of up to 5° and a tilt of up to 45°; 6 ([`lamp`], `0x3ff480`).
//! * **6** the fall: by its velocity (across ×0.97 above 0.05, up ×0.97, 0.003 down a tick), spinning; out of the
//!   level's box gone; its ticks out or a 0.5 sphere meeting anything (`0x2db908` = L00 `0x2df8d8`): the explosion
//!   again with a 2.0 sphere of 1, its bolts and gone; else smoke ([`fall_smoke`]) and the lamp.
//!
//! **The tail**: its ticks, the waits ([`waits`], `0x3ffda8`); the first aim (`0x3206a0` = L00 `0x324208`, 0.15 / 0.3);
//! the gun ([`fire`], `0x3fff78`); the second aim; the thrusters ([`thrusters`], `0x400180`).
//!
//! **The draws** (the second list, `0x2e3850` / `0x2e3918` = L00 `0x2e7920`): the spot ([`draw_spot`], `0x3fec60`,
//! registered by a lit robot in view) and the drawer's ([`draw_group`], `0x3ff960`: its group's members in view
//! farthest from the camera first, each its siren ([`draw_siren`], `0x3fe8f0`) when `+0x42c` has 8).
//!
//! Not ported: the light's cone (`0x3fe5b8`, built by `0x3fe3f8` into `0x2a1ff0`, its colours `0x2a2658` whose
//! alphas the drawer's `0x3ff768` pulses; `+0x42c` 2); the start along the paths by `+0x430` (`0x33eb48`; every placed
//! one has 0); the level-9 check of its hitter (`0x3280e0`, clearing `$gp−0x5fb0`, as crate::moby::classes::level09::
//! critter's); the fall's smoke trail `0x332330` = L00 `0x335780` (its random draw is kept). The paths' lengths are
//! measured into a copy (crate::level::LevelData::walked_path), not into the level's path data.

use crate::fx::FxQuad;
use crate::hero::physics::{add, add_rot, atan2, cross, dist, euler_rows, len, len2, scale, set_len, spring, sub, sub_rot, V3};
use crate::moby::classes::props::{bob, set_seq, sway, wrapped};
use crate::moby::{ai, bolts, countdown, joint_aim, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0d30;
/// The mobys its light locks on to besides Ratchet.
const PREY: [i16; 3] = [0x00cb, 0x076c, 0x0eb0];

const H: usize = 0x20;
const STATUS: usize = 0x2e;
const DAMAGE: usize = 0x34;
const T_POINT: usize = 0x140;
const T_MOBY: usize = 0x150;
const T_FOUND: usize = 0x154;
const REACT: usize = 0x160;
const AIM_A: usize = 0x210;
const AIM_B: usize = 0x290;
const ALARM_ID: usize = 0x310;
const PATH: usize = 0x314;
const LIGHT_PATH: usize = 0x318;
const LIGHT_SEG: usize = 0x324;
const LOOPED: usize = 0x32a;
const SPOT: usize = 0x330;
const LAMP: usize = 0x340;
const LAMP_AT: usize = 0x370;
const SIREN_TURN: usize = 0x380;
const SPEED: usize = 0x388;
const LIGHT_LEN: usize = 0x38c;
const LIGHT: usize = 0x390;
const LIGHT_VEL: usize = 0x3a0;
const CHASED: usize = 0x3ac;
const PATH_LEN: usize = 0x3b0;
const VEL: usize = 0x3b4;
const BOB: usize = 0x3c0;
const BOB_LAST: usize = 0x3c4;
const SWAY_A: usize = 0x3c8;
const SWAY_B: usize = 0x3cc;
const TURN_TO: usize = 0x3d0;
const TURN_T: usize = 0x3d4;
const KIND: usize = 0x3d6;
const TURN_V: usize = 0x3d8;
const LIGHT_YAW: usize = 0x3dc;
const LIGHT_PITCH: usize = 0x3e0;
const LIGHT_T: usize = 0x3ec;
const VOICE: usize = 0x3f0;
const SIREN_AT: usize = 0x3f4;
const FALL_VEL: usize = 0x400;
const FALL_T: usize = 0x410;
const PATIENCE: usize = 0x412;
const SPIN: usize = 0x41c;
const LIT: usize = 0x420;
const RED: usize = 0x422;
const GROUP_N: usize = 0x424;
const BURST_WAIT: usize = 0x426;
const SHOT_WAIT: usize = 0x428;
const GUN: usize = 0x429;
const SHOTS: usize = 0x42a;
const FIRING: usize = 0x42b;
const DRAWS: usize = 0x42c;
const SIZE: usize = 0x434;
const PUSH: usize = 0x438;
const ALARMED: usize = 0x440;
const HELD: usize = 0x441;
const FENCE: usize = 0x444;
const AREA: usize = 0x448;
const CHASED_CLASS: usize = 0x44c;
const PVARS: usize = 0x450;

/// `$gp+0x2174..+0x2190` (`0x1b1164..`): the loop sounds idle / alarmed, the shot's sound, the shot's speed a second,
/// the burst's shots, the ticks between them, the wait after a burst, the shot's range; `$gp+0x2240`: the shot sound
/// every 7th shot.
const SOUND_IDLE: i32 = 0;
const SOUND_SHOT: i32 = 1;
const SOUND_ALARM: i32 = 2;
const SHOT_SPEED: f32 = 20.0;
const BURST: u8 = 6;
const SHOT_TICKS: u8 = 10;
const BURST_TICKS: i16 = 90;
const SHOT_RANGE: f32 = 30.0;
const SHOT_SOUND_EVERY: u8 = 7;
/// `$gp+0x2200` (`0x1b11f0`): the spot's corners; `$gp+0x21c0` (`0x1b11b0`): the siren's.
const SPOT_CORNERS: [V3; 4] = [[-1.0, 1.0, 0.0], [-1.0, -1.0, 0.0], [1.0, 1.0, 0.0], [1.0, -1.0, 0.0]];
const SIREN_CORNERS: [V3; 4] = [[0.0, -1.0, 1.0], [0.0, -1.0, -1.0], [0.0, 1.0, 1.0], [0.0, 1.0, -1.0]];
const QUAD_ST: [[f32; 2]; 4] = [[1.0, 1.0], [0.0, 1.0], [1.0, 0.0], [0.0, 0.0]];

fn link(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { ai::link(&w.mobys[id], off).filter(|&k| k < w.mobys.len()) }
fn gone(s: u8) -> bool { s == 0xfe || s == 0xfd }

/// `0x3605f0(n, 4, m)` unless its loop sound plays.
fn start_voice(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, n: i32) {
    let h = w.mobys[id].pv_i(VOICE);
    if h != -1 && ctx.host.sound_playing(h) { return; }
    let h = ctx.host.play_sound(&w.mobys[id], n, 4);
    w.mobys[id].set_pv_i(VOICE, h);
}

/// `0x3602f0`: its loop sound let go.
fn stop_voice(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(VOICE);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(VOICE, -1);
}

/// Its loop sound swapped for `n`.
fn swap_voice(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, n: i32) {
    stop_voice(w, id, ctx);
    start_voice(w, id, ctx, n);
}

/// The members of its group (`+0x21`).
fn group(w: &MobyWorld, id: usize) -> Vec<usize> { crate::moby::director::members(w, w.mobys[id].group as u8 as i32) }

/// `0x3ffe08`: each of its group with its alarm (`+0x310`, not 0xff) alarmed (`+0x440`).
fn alarm(w: &mut MobyWorld, id: usize) {
    if w.mobys[id].group as u8 == 0xff { return; }
    let a = w.mobys[id].pv_i(ALARM_ID);
    if a == 0xff { return; }
    for e in group(w, id) {
        let Some(m) = w.mobys.get_mut(e) else { break };
        if m.pvars.len() < PVARS { continue; }
        if m.pv_i(ALARM_ID) == a { m.set_pv_b(ALARMED, 1); }
    }
}

/// 0x0d30 (module docs).
pub fn update_0d30(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    if ctx.level.off_chunk(ctx.game.variant, w.mobys[id].pos[2]) { return; }
    if w.mobys[id].state == 0 {
        setup(w, id, ctx);
        if 0x80 <= w.mobys[id].state { return; }
    }
    if w.mobys[id].pv_b(STATUS) == 2 {
        w.mobys[id].set_pv_i(DRAWS, 0);
        bolts::drop(w, id, 0, 3, ctx);
        w.delete(id);
        return;
    }
    w.changed = true;
    if w.mobys[id].state == 5 {
        w.register_draw(id);
        return;
    }
    w.mobys[id].set_pv_i(DRAWS, 0);
    // `0x3ffed8`: the damage and the target.
    if ai::take_damage(w, id, ctx, false) != 0 { alarm(w, id); }
    let area = w.mobys[id].pv_i(AREA);
    ai::find_target(w, id, ctx, 0.0, 0.0, area, None, false);
    if (1..=3).contains(&w.mobys[id].state) { w.mobys[id].set_pv_s(LIT, 0); }
    match w.mobys[id].state {
        1 => patrol(w, id, ctx),
        2 => {
            if w.mobys[id].pv_f(H) <= 0.0 {
                set_seq(w, id, ctx, 0, 0, 10);
                stop_voice(w, id, ctx);
                w.mobys[id].set_state(4, -1);
            } else if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 0, 0, 10);
                let m = &mut w.mobys[id];
                m.set_pv_s(PATIENCE, 600);
                m.set_state(3, -1);
            }
        }
        3 => chase(w, id, ctx),
        4 => {
            shot_down(w, id, ctx);
            return;
        }
        6 => {
            fall(w, id, ctx);
            return;
        }
        _ => {}
    }
    if 0x80 <= w.mobys[id].state { return; }
    tail(w, id, ctx);
}

/// `0x3ffb00` (module docs): the blocks; the drawer hidden in 5 with its group's size; else at its path's start, the
/// light at its path's, the paths measured, state 1 and its loop sound; without both paths gone.
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_pv_b(STATUS, 1);
    m.set_pv_b(REACT + 0x10, 2);
    m.set_pv_i(REACT + 0x48, 2);
    for o in [0x11, 0x12, 0x16, 0x17] { m.set_pv_b(REACT + o, 1); }
    m.set_pv_f(REACT + 0x1c, 0.016_666_668);
    m.set_pv_f(REACT + 0x20, 0.016_666_668);
    m.set_pv_f(REACT + 0x24, 0.004_166_667);
    m.set_pv_f(REACT + 0x28, 0.008_333_334);
    m.set_pv_f(REACT + 0x2c, 0.004_166_667);
    m.set_pv_f(REACT + 0x30, 1.0);
    m.set_pv_f(REACT + 0x34, -0.25);
    m.set_pv_f(REACT + 0x3c, 0.4);
    for (o, v) in [(0x6c, 7), (0x6d, 12), (0x6e, 3), (0x6f, 5), (0x72, 10), (0x73, 17), (0x76, 1)] { m.set_pv_b(REACT + o, v); }
    m.set_pv_b(HELD, 0);
    if m.pv_s(KIND) == 1 {
        m.has_collision = false;
        m.update_dist = 0xff;
        m.mode = (m.mode | 0x41) & 0xefff;
        m.set_state(5, -1);
        m.set_pv_b(0x48, 17);
        m.set_pv_b(0x4a, 6);
        let n = group(w, id).len() as i16;
        w.mobys[id].set_pv_s(GROUP_N, n);
        return;
    }
    let (path, lpath) = (m.pv_i(PATH), m.pv_i(LIGHT_PATH));
    let first = |p: i32| usize::try_from(p).ok().and_then(|i| ctx.level.paths.get(i)).and_then(|v| v.first().copied());
    let (Some(a), Some(b)) = (first(path), first(lpath)) else {
        w.delete(id);
        return;
    };
    m.set_pv_b(H + 8, 3);
    m.pos = [a[0], a[1], a[2]];
    m.set_pv_v(LIGHT, [b[0], b[1], b[2]]);
    m.set_pv_f(LIGHT + 0xc, b[3]);
    // `0x3feec8`: the paths' lengths (the closing segment left out); the light and the robot at their starts.
    let total = |p: i32| { let v = ctx.level.walked_path(p); v.iter().take(v.len().saturating_sub(1)).map(|q| q[3]).sum::<f32>() };
    m.set_pv_f(LIGHT_LEN, total(lpath));
    m.set_pv_f(PATH_LEN, total(path));
    for o in [0x320, LIGHT_SEG] { m.set_pv_i(o, 0); }
    for o in [0x32c, LIGHT_T] { m.set_pv_f(o, 0.0); }
    m.set_pv_s(0x328, 1);
    m.set_pv_f(0x43c, -1.0);
    for o in [SIREN_TURN, 0x3b4, 0x3b8, 0x3bc, 0x3a0, 0x3a4, 0x3a8, 0x3cc, 0x3c8, 0x3c4, 0x3c0, 0x384, 0x3e8, 0x3e4, TURN_V] { m.set_pv_f(o, 0.0); }
    m.set_pv_i(DRAWS, 0);
    m.set_pv_f(TURN_TO, m.rot[2]);
    m.set_pv_s(TURN_T, 0);
    m.set_pv_i(CHASED, 0);
    m.set_state(1, -1);
    start_voice(w, id, ctx, SOUND_IDLE);
}

/// The 0x00cb / 0x076c / 0x0eb0 a sphere at `c` of `r` meets.
fn prey_in(w: &MobyWorld, ctx: &Ctx, c: V3, r: f32) -> Vec<usize> {
    ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, c, r, crate::collision::QueryFlags(0), None)).unwrap_or_default()
        .into_iter()
        .filter(|&k| w.mobys.get(k).is_some_and(|o| PREY.contains(&o.o_class)))
        .collect()
}

/// `0x3ff550(m, draw)`: lit while its target's point is within twice its draw distance — the lamp's frame (joint
/// 1), the light's yaw and pitch from it (from its own, for the first aim), the spot the light; with `draw` the spot
/// drawn in view (1.4 × its size) and the drawer's light (`+0x42c` 2); the last 0x00cb / 0x076c / 0x0eb0 within 0.9
/// × its size of the spot chased. Returns whether lit.
fn light(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, draw: bool) -> bool {
    let m = &w.mobys[id];
    if (m.draw_dist as f32) * 2.0 <= dist(m.pos, m.pv_v(T_POINT)) { return false; }
    if let Some((rows, p)) = ctx.host.joint_matrix(&w.mobys[id], 1) {
        let m = &mut w.mobys[id];
        for (k, r) in rows.iter().enumerate() { m.set_pv_v(LAMP + 0x10 * k, *r); }
        m.set_pv_v(LAMP_AT, p);
    }
    let m = &mut w.mobys[id];
    let v = sub(m.pv_v(LIGHT), m.pv_v(LAMP_AT));
    let yaw = sub_rot(atan2(v[0], v[1]), m.rot[2]);
    let pitch = sub_rot(-atan2(len2(v), v[2]), m.rot[1]);
    m.set_pv_f(AIM_A + 0x68, yaw);
    m.set_pv_f(LIGHT_YAW, yaw);
    m.set_pv_f(AIM_A + 0x64, pitch);
    m.set_pv_f(LIGHT_PITCH, pitch);
    let spot = m.pv_v(LIGHT);
    m.set_pv_v(SPOT, spot);
    m.set_pv_f(SPOT + 0xc, m.pv_f(LIGHT + 0xc));
    let size = m.pv_f(SIZE);
    if draw {
        let r = size * 1.4;
        if !ctx.view.is_some_and(|v| v.culled(w.mobys[id].draw_dist as f32, [spot[0], spot[1], spot[2], r])) { w.register_draw(id); }
    }
    for k in prey_in(w, ctx, spot, size * 0.9) {
        let c = w.mobys[k].o_class;
        let m = &mut w.mobys[id];
        ai::set_link(m, CHASED, Some(k));
        m.set_pv_i(CHASED_CLASS, c as i32);
    }
    if draw { let m = &mut w.mobys[id]; m.set_pv_i(DRAWS, m.pv_i(DRAWS) | 2); }
    true
}

/// `0x3ff0e8`: the light's point `+0x388` further along its path (`0x33fda8` = L00 `0x342b50`, crate::paths::advance),
/// the light springing to it (0.02, 0.2); off an open path's end, its place there kept and its speed turned round.
fn sweep(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    let pts = ctx.level.walked_path(m.pv_i(LIGHT_PATH));
    let looped = m.pv_s(LOOPED) != 0;
    let (keep_seg, keep_t) = (m.pv_i(LIGHT_SEG), m.pv_f(LIGHT_T));
    let mut seg = keep_seg.max(0) as usize;
    let mut t = keep_t;
    let (p, off) = crate::paths::advance(&pts, m.pv_f(SPEED), &mut seg, &mut t, looped);
    m.set_pv_i(LIGHT_SEG, seg as i32);
    m.set_pv_f(LIGHT_T, t);
    for k in 0..3 {
        let (mut x, mut v) = (m.pv_f(LIGHT + 4 * k), m.pv_f(LIGHT_VEL + 4 * k));
        spring(p[k], 0.02, 0.2, 0.0, &mut x, &mut v);
        m.set_pv_f(LIGHT + 4 * k, x);
        m.set_pv_f(LIGHT_VEL + 4 * k, v);
    }
    if !looped && off {
        m.set_pv_f(LIGHT_T, keep_t);
        m.set_pv_i(LIGHT_SEG, keep_seg);
        m.set_pv_f(SPEED, -m.pv_f(SPEED));
    }
}

/// `0x3ff210`: its place along its own path the share the light has come along its own (`+0x38c` against `+0x3b0`),
/// sprung to (0.02, 0.2).
fn ride(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    let lp = ctx.level.walked_path(m.pv_i(LIGHT_PATH));
    let n = (m.pv_i(LIGHT_SEG).max(0) as usize).min(lp.len());
    let came = lp[..n].iter().map(|q| q[3]).sum::<f32>() + m.pv_f(LIGHT_T);
    let pts = ctx.level.walked_path(m.pv_i(PATH));
    let (mut seg, mut t) = (0usize, 0.0f32);
    // A light path of no length (one point) gives no share (the game's divides by it).
    let share = if m.pv_f(LIGHT_LEN) != 0.0 { came / m.pv_f(LIGHT_LEN) } else { 0.0 };
    let (p, _) = crate::paths::advance(&pts, share * m.pv_f(PATH_LEN), &mut seg, &mut t, m.pv_s(LOOPED) != 0);
    for k in 0..3 {
        let (mut x, mut v) = (m.pos[k], m.pv_f(VEL + 4 * k));
        spring(p[k], 0.02, 0.2, 0.0, &mut x, &mut v);
        m.pos[k] = x;
        m.set_pv_f(VEL + 4 * k, v);
    }
}

/// `0x3ff368`: the bob (`0x3208c8` = L00 `0x324430`: 0.25, 0.06 a tick) and the sway (`0x320980` = L00 `0x3244e8`:
/// 0.15, 0.05, 0.025); patrolling, every 260 ticks a turn of ±120° to spring to (0.003, 0.2, at most 0.02).
fn hover(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    bob(m, 0.25, 0.06, -1.0, BOB, BOB_LAST);
    sway(m, 0.15, 0.05, 0.025, SWAY_A, SWAY_B);
    if m.state != 1 { return; }
    let mut t = m.pv_s(TURN_T) as i32;
    let out = countdown(&mut t);
    m.set_pv_s(TURN_T, t as i16);
    if out {
        m.set_pv_s(TURN_T, 0x104);
        let turn = if ctx.game.rng.randi(2) != 0 { -2.094_395_4 } else { 2.094_395_4 };
        let m = &mut w.mobys[id];
        m.set_pv_f(TURN_TO, add_rot(m.rot[2], turn));
    }
    let m = &mut w.mobys[id];
    let (mut a, mut v) = (m.rot[2], m.pv_f(TURN_V));
    spring(m.pv_f(TURN_TO), 0.003, 0.2, 0.02, &mut a, &mut v);
    m.rot[2] = a;
    m.set_pv_f(TURN_V, v);
}

/// Onto its target `k`: its group alarmed, chased, 3 with the alert's loop and its patience.
fn lock_on(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: usize) {
    alarm(w, id);
    let c = w.mobys[k].o_class;
    let m = &mut w.mobys[id];
    ai::set_link(m, CHASED, Some(k));
    m.set_pv_i(CHASED_CLASS, c as i32);
    m.set_state(3, -1);
    swap_voice(w, id, ctx, SOUND_ALARM);
    w.mobys[id].set_pv_s(PATIENCE, 600);
}

/// State 1 (module docs).
fn patrol(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].glow = 0xffaa_aaaa;
    start_voice(w, id, ctx, SOUND_IDLE);
    if w.mobys[id].pv_b(ALARMED) != 0 {
        w.mobys[id].set_pv_b(ALARMED, 0);
        w.mobys[id].set_state(3, -1);
        swap_voice(w, id, ctx, SOUND_ALARM);
        w.mobys[id].set_pv_s(PATIENCE, 600);
    }
    let m = &mut w.mobys[id];
    m.set_pv_s(RED, 0);
    m.set_pv_s(LIT, 1);
    let lit = light(w, id, ctx, true);
    sweep(w, id, ctx);
    ride(w, id, ctx);
    hover(w, id, ctx);
    if !lit { return; }
    let m = &w.mobys[id];
    let (spot, lamp, size) = (m.pv_v(SPOT), m.pv_v(LAMP_AT), m.pv_f(SIZE));
    let mid = add(spot, scale(sub(lamp, spot), 0.5));
    let near = |c: V3, r: f32| ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, c, r, crate::collision::QueryFlags(0), None)).unwrap_or_default();
    let mut list = near(mid, size * 0.48);
    if list.is_empty() { list = near(spot, size * 0.9); }
    for k in list {
        if k >= w.mobys.len() { continue; }
        let hero = ctx.hero_moby == Some(k);
        if !hero && !PREY.contains(&w.mobys[k].o_class) { continue; }
        lock_on(w, id, ctx, k);
        if !hero { break; }
    }
}

/// State 3 (module docs).
fn chase(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].glow = 0xff00_ffff;
    let target = link(w, id, CHASED);
    let ok = target.is_some_and(|k| { let t = &w.mobys[k]; t.o_class as i32 == w.mobys[id].pv_i(CHASED_CLASS) && !gone(t.state) });
    if let (true, Some(k)) = (ok, target) {
        let m = &mut w.mobys[id];
        m.set_pv_s(LIT, 1);
        m.set_pv_s(RED, 1);
        light(w, id, ctx, true);
        let mut a = add(w.mobys[k].pos, [0.0, 0.0, 0.5]);
        let b = sub(a, [0.0, 0.0, 4.0]);
        match w.coll_line(ctx, a, b, 0x12, None) {
            Some(hit) => a = hit.point,
            None => a[2] -= 0.5,
        }
        let m = &w.mobys[id];
        let previous = m.pv_v(LIGHT);
        let phase = ctx.game.frame as f32 + (m.uid as i32 * 36) as f32;
        let a1 = crate::hero::physics::wrap(phase * 0.025);
        let a2 = crate::hero::physics::wrap(phase * 0.016_666_668);
        let s1 = a1.sin() + 1.5;
        let s2 = a2.sin() + 1.5;
        a[0] += a2.sin();
        a[1] += a2.cos();
        let m = &mut w.mobys[id];
        for (k, kk, t) in [(0usize, s1 * 0.003, a[0]), (1, s2 * 0.003, a[1])] {
            let (mut x, mut v) = (m.pv_f(LIGHT + 4 * k), m.pv_f(LIGHT_VEL + 4 * k));
            spring(t, kk, 0.2, 0.0, &mut x, &mut v);
            m.set_pv_f(LIGHT + 4 * k, x);
            m.set_pv_f(LIGHT_VEL + 4 * k, v);
        }
        let fence = m.pv_i(FENCE);
        if fence != -1 {
            let r = m.pv_f(SIZE) * 1.3;
            let pts = crate::path_network::points_now(ctx.level, &ctx.game.path_openings, fence);
            let moved = pts.and_then(|p| ai::fence(&p, r, m.pv_v(LIGHT)));
            match moved {
                Some(q) => {
                    m.set_pv_v(LIGHT, q);
                    m.set_pv_f(LIGHT_VEL, q[0] - previous[0]);
                    m.set_pv_f(LIGHT_VEL + 4, q[1] - previous[1]);
                    m.set_pv_b(HELD, 1);
                }
                None => m.set_pv_b(HELD, 0),
            }
        }
    } else {
        let t = link(w, id, T_MOBY);
        ai::set_link(&mut w.mobys[id], CHASED, t);
        if let Some(k) = t.filter(|&k| !gone(w.mobys[k].state)) {
            let c = w.mobys[k].o_class;
            w.mobys[id].set_pv_i(CHASED_CLASS, c as i32);
        }
    }
    // `0x340530` (L00 `0x3432d8`): the light's place on its path.
    let m = &mut w.mobys[id];
    let pts = ctx.level.walked_path(m.pv_i(LIGHT_PATH));
    if let Some((_, seg, t)) = crate::paths::closest(&pts, 20.0, 5.0, 0.0, m.pv_v(LIGHT), true) {
        m.set_pv_i(LIGHT_SEG, seg as i32);
        m.set_pv_f(LIGHT_T, t);
    }
    ride(w, id, ctx);
    let m = &w.mobys[id];
    let p = m.pos;
    if !ctx.view.is_some_and(|v| v.culled(m.draw_dist as f32, [p[0], p[1], p[2], 4.0])) {
        if let Some(j) = ctx.host.joint(&w.mobys[id], 8) { w.mobys[id].set_pv_v(SIREN_AT, j); }
        let m = &mut w.mobys[id];
        m.set_pv_i(DRAWS, m.pv_i(DRAWS) | 8);
        hover(w, id, ctx);
    }
    let found = w.mobys[id].pv_i(T_FOUND);
    if found != 1 && w.mobys[id].pv_s(PATIENCE) == 0 {
        w.mobys[id].set_state(1, -1);
        swap_voice(w, id, ctx, SOUND_IDLE);
    }
    if found == 1 {
        let m = &mut w.mobys[id];
        if m.pv_i(CHASED) != 0 { m.set_pv_i(CHASED, m.pv_i(T_MOBY)); }
        m.set_pv_b(FIRING, 1);
    }
}

/// State 4 (module docs).
fn shot_down(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_s(FALL_T, 180);
    alarm(w, id);
    stop_voice(w, id, ctx);
    let e = crate::moby::blast::Explosion {
        embers: 5, billows: 2, flares: 4, flash: 4.0, flash2: 2.0, near: 9.0, size: 1.0, sound: 1, light: 15.0, shake: true, debris: 5, lod: -1, flags: 0x81_0001,
        ..Default::default()
    };
    crate::moby::blast::explode(w, ctx, Some(id), None, &e);
    let down = ctx.hero.gravity.down;
    let mut a = sub(w.mobys[id].pos, ctx.hero.pos);
    a[2] = 0.0;
    let l = ctx.game.rng.randf(0.075, 0.2);
    let a = set_len(a, l);
    let _b = cross(down, a);
    let turn = ctx.game.rng.randf_signed(0.698_131_8, 1.221_730_6);
    let v = crate::camera::rot(a, turn, down);
    let up = ctx.game.rng.randf(0.1, 0.2);
    let m = &mut w.mobys[id];
    m.set_pv_v(FALL_VEL, v);
    m.set_pv_f(FALL_VEL + 8, up);
    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x7f, [0.0; 3], 0xff);
    let m = &mut w.mobys[id];
    m.scale *= 0.1;
    m.set_pv_f(AIM_A + 0x70, 10.0);
    for o in [0x414, 0x418, SPIN] {
        let s = ctx.game.rng.randf_signed(0.0, 0.087_266_47);
        w.mobys[id].set_pv_f(o, s);
    }
    for k in 0..2 {
        let t = ctx.game.rng.randf_signed(0.0, 0.785_398_24);
        let m = &mut w.mobys[id];
        m.rot[k] = add_rot(m.rot[k], t);
    }
    w.mobys[id].set_state(6, -1);
    lamp(w, id, ctx);
}

/// State 6 (module docs).
fn fall(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let v = m.pv_v(FALL_VEL);
    m.pos = add(m.pos, v);
    for k in 0..2 {
        let x = m.pv_f(FALL_VEL + 4 * k);
        if 0.05 < x.abs() { m.set_pv_f(FALL_VEL + 4 * k, x * 0.97); }
    }
    let mut up = m.pv_f(FALL_VEL + 8);
    if 0.0 < up { up *= 0.97; }
    m.set_pv_f(FALL_VEL + 8, up - 0.003);
    m.rot[2] = add_rot(m.rot[2], m.pv_f(SPIN));
    m.set_pv_f(AIM_A + 0x70, 10.0);
    if m.pos.iter().any(|&c| !(2.0..=1021.0).contains(&c)) {
        w.delete(id);
        return;
    }
    let mut t = m.pv_s(FALL_T) as i32;
    let out = countdown(&mut t);
    m.set_pv_s(FALL_T, t as i16);
    let pos = m.pos;
    let hit = out || ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, 0.5, crate::collision::QueryFlags(0), Some(id))).is_some();
    if hit {
        let e = crate::moby::blast::Explosion {
            radius: 2.0, damage: 1.0, embers: 5, billows: 2, flares: 4, flash: 2.0, flash2: 1.0, near: 9.0, size: 1.0, sound: 1, light: 15.0, shake: true, debris: 5,
            lod: -1, flags: 0x81_0001, ..Default::default()
        };
        crate::moby::blast::explode(w, ctx, Some(id), None, &e);
        bolts::drop(w, id, 0, 3, ctx);
        w.delete(id);
        return;
    }
    fall_smoke(w, id, ctx);
    lamp(w, id, ctx);
}

/// The fall's smoke: the trail `0x332330` (L00 `0x335780`, not ported: its 10..20 drawn) and two grey glow puffs
/// (`0x332768` = L00 `0x335bb8`, crate::particles::puff: 0.025, 40000..100000, a spin of up to 5 either way, 0x60
/// grey of 0x30..0xff; ALPHA 0x44, 60 ticks).
fn fall_smoke(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let rng = &mut ctx.game.rng;
    rng.rand_range(10, 20);
    let pos = w.mobys[id].pos;
    for _ in 0..2 {
        let mut spin = rng.randi(6);
        if rng.randi(2) != 0 { spin = -spin; }
        let size = rng.randf(40_000.0, 100_000.0);
        let c = rng.rand_range(0x30, 0xff) as u32;
        let rgba = c | (c << 16) | (c << 8) | 0x6000_0000;
        let Some(sys) = ctx.parts.as_deref_mut() else { continue };
        if let Some(i) = crate::particles::puff(sys, rng, 0.025, 1.0, 1.0, size, pos, spin, [0.0; 3], rgba, 0) {
            let r = &mut sys.pool.recs[i];
            r[3] = 0x44;
            crate::particles::rec::set_i16(r, 0xa, 60);
            crate::particles::rec::set_u32(r, 0x24, 2);
            r[0x2a] = 0x60;
            r[0x2b] = r[0xa];
        }
    }
}

/// `0x3ff480`: 4 in 5 ticks, in view (4), the lamp's frame (joint 1, its rows made unit) and the drawer's light only.
fn lamp(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.rng.randi(5) == 0 { return; }
    let m = &w.mobys[id];
    let p = m.pos;
    if ctx.view.is_some_and(|v| v.culled(m.draw_dist as f32, [p[0], p[1], p[2], 4.0])) { return; }
    if let Some((rows, at)) = ctx.host.joint_matrix(&w.mobys[id], 1) {
        let m = &mut w.mobys[id];
        for (k, r) in rows.iter().enumerate() { m.set_pv_v(LAMP + 0x10 * k, set_len(*r, 1.0)); }
        m.set_pv_v(LAMP_AT, at);
    }
    w.mobys[id].set_pv_i(DRAWS, 2);
}

/// The tail (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    waits(w, id);
    joint_aim::aim(w, id, AIM_A, id, 0, 0.15, 0.3);
    fire(w, id, ctx);
    let m = &mut w.mobys[id];
    m.set_pv_f(AIM_B + 0x68, add_rot(m.pv_f(AIM_B + joint_aim::ANGLES + 8), 0.052_359_886));
    joint_aim::aim(w, id, AIM_B, id, 11, 0.15, 0.3);
    thrusters(w, id, ctx);
}

/// `0x3ffda8`: its ticks; its patience while its target is not found, the shot's and the burst's waits.
fn waits(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) < 0xffff { m.state_ticks = (m.state_ticks as u16 + 1) as i16; }
    if m.pv_i(T_FOUND) == 0 {
        let mut t = m.pv_s(PATIENCE) as i32;
        countdown(&mut t);
        m.set_pv_s(PATIENCE, t as i16);
    }
    let mut t = m.pv_b(SHOT_WAIT) as i32;
    countdown(&mut t);
    m.set_pv_b(SHOT_WAIT, t as u8);
    let mut t = m.pv_s(BURST_WAIT) as i32;
    countdown(&mut t);
    m.set_pv_s(BURST_WAIT, t as i16);
}

/// `0x3fff78`: firing with the light within 3 of its target's moby: its patience; the waits out, a bolt (crate::moby::
/// classes::gunship_bolt::fire: its damage byte, range 30, size 0.5) from its gun (joint 4 / 5 by turns) at the
/// light (held by its fence near its target's point: at that) at 20 a second, its sound 1 every 7th shot, 10 ticks to
/// the next, after 6 the burst's 90. Not firing, between shots, the burst's count runs down a shot every 10 ticks.
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let near = m.pv_b(FIRING) != 0 && link(w, id, T_MOBY).is_some_and(|k| dist(m.pv_v(LIGHT), w.mobys[k].pos) < 3.0);
    if near {
        w.mobys[id].set_pv_s(PATIENCE, 600);
        let m = &w.mobys[id];
        if m.pv_s(BURST_WAIT) != 0 || m.pv_b(SHOT_WAIT) != 0 { return; }
        let gun = m.pv_b(GUN) as usize + 4;
        let Some(from) = ctx.host.joint(&w.mobys[id], gun) else { return };
        let m = &w.mobys[id];
        let to = if m.pv_b(HELD) != 0 && dist(m.pv_v(T_POINT), m.pv_v(LIGHT)) < 3.0 { m.pv_v(T_POINT) } else { m.pv_v(LIGHT) };
        let v = set_len(sub(to, from), SHOT_SPEED * 0.016_666_668);
        let damage = m.pv_b(DAMAGE) as f32;
        super::super::gunship_bolt::fire(w, ctx, from, id, damage, SHOT_RANGE, 1.0, v, 1, 1, 0.5);
        if w.mobys[id].pv_b(SHOTS) % SHOT_SOUND_EVERY == 0 { ctx.host.play_sound(&w.mobys[id], SOUND_SHOT, 0); }
        let m = &mut w.mobys[id];
        m.set_pv_b(GUN, m.pv_b(GUN) ^ 1);
        let n = m.pv_b(SHOTS).wrapping_add(1);
        m.set_pv_b(SHOTS, n);
        m.set_pv_b(SHOT_WAIT, SHOT_TICKS);
        if BURST < n {
            m.set_pv_b(SHOTS, 0);
            m.set_pv_s(BURST_WAIT, BURST_TICKS);
        }
    } else if w.mobys[id].pv_b(SHOT_WAIT) == 0 {
        let m = &mut w.mobys[id];
        let mut t = m.pv_b(SHOTS) as i32;
        let out = countdown(&mut t);
        m.set_pv_b(SHOTS, t as u8);
        if !out { m.set_pv_b(SHOT_WAIT, SHOT_TICKS); }
    }
}

/// `0x400180`: in view (6), at its joints 12 and 13 the thrusters' trails (crate::particles::trail) 3.5..4.5 a second
/// down: two (0.5 and 0.515 up) of orange 0x77ff0060 → red (21 ticks, 300 shrinking 100) or red 0x77ff0000 → 0x00ff0060
/// (19 ticks, 200), each with a white 0x77ffffff one (12 ticks, 100); their far byte by its draw distance.
fn thrusters(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let p = m.pos;
    if ctx.view.is_some_and(|v| v.culled(m.draw_dist as f32, [p[0], p[1], p[2], 6.0])) { return; }
    let far = ((m.draw_dist >> 5) << 4) as u8;
    for j in 12..14 {
        let v = [0.0, 0.0, -(ctx.game.rng.randf(3.5, 4.5) * 0.016_666_668)];
        for k in 0..2 {
            let Some(mut b) = ctx.host.joint(&w.mobys[id], j) else { continue };
            b[2] += k as f32 * 0.015 + 0.5;
            let Some(sys) = ctx.parts.as_deref_mut() else { continue };
            let rng = &mut ctx.game.rng;
            let a = if k & 1 != 0 {
                crate::particles::trail(sys, rng, b, v, 0x77ff_0000, 0x00ff_0060, 0x13, 200, -100, true)
            } else {
                crate::particles::trail(sys, rng, b, v, 0x77ff_0060, 0x00ff_0000, 0x15, 300, -100, true)
            };
            if let Some(i) = a { sys.pool.recs[i][9] = far; }
            if let Some(i) = crate::particles::trail(sys, rng, b, v, 0x77ff_ffff, 0x00ff_0000, 0xc, 100, -100, true) { sys.pool.recs[i][9] = far; }
        }
    }
}

/// `0x3fec60` (module docs): the spot, 0.01 up and toward the camera by `+0x438` (at most 2 short of it): its size,
/// texture 0xb, red 0x7f40407f alarmed, else 0x40507f7f; firing, a flash (texture 0x20, size 1) and firing cleared.
pub fn draw_spot(w: &mut MobyWorld, id: usize, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let mut p = m.pv_v(SPOT);
    p[2] += 0.01;
    let to = sub(cam, p);
    let d = len(to);
    if 0.0 < d {
        let r = m.pv_f(PUSH).min(d - 2.0).max(0.0);
        p = add(p, set_len(to, r));
    }
    let c = if m.pv_s(RED) != 0 { 0x7f40_407f } else { 0x4050_7f7f };
    let size = m.pv_f(SIZE);
    out.push(FxQuad { fx: 0xb, additive: true, corners: SPOT_CORNERS.map(|k| add(scale(k, size), p)), st: QUAD_ST, rgba: [c; 4] });
    if m.pv_b(FIRING) != 0 {
        out.push(FxQuad { fx: 0x20, additive: true, corners: SPOT_CORNERS.map(|k| add(k, p)), st: QUAD_ST, rgba: [c; 4] });
        w.mobys[id].set_pv_b(FIRING, 0);
    }
}

/// `0x3ff960` (module docs): its group's members in view (4) by their distance from the camera, farthest first (a
/// member's uid drawn once, `0x3ff840`), each its siren.
pub fn draw_group(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, view: &crate::particles::BSphereView, cam: V3, out: &mut Vec<FxQuad>) {
    let members = group(w, id);
    let mut drawn: Vec<i16> = Vec::new();
    for _ in 0..w.mobys[id].pv_s(GROUP_N).max(0) {
        let mut best = None;
        let mut far = 0.0;
        for &e in &members {
            let Some(o) = w.mobys.get(e) else { continue };
            if drawn.contains(&o.uid) { continue; }
            let d = dist(cam, o.pos);
            if far < d {
                far = d;
                best = Some(e);
            }
        }
        let Some(k) = best else { break };
        drawn.push(w.mobys[k].uid);
        let o = &w.mobys[k];
        if o.pvars.len() < PVARS || view.culled(o.draw_dist as f32, [o.pos[0], o.pos[1], o.pos[2], 4.0]) { continue; }
        if o.pv_i(DRAWS) & 8 != 0 { draw_siren(w, host, k, cam, out); }
    }
}

/// `0x3fe8f0`: the siren at joint 8 (`0x3127c8`, as it is now; `+0x3f4` its place at the tick): 16 pairs of camera-facing squares out either way
/// along its turn (`+0x380`, 4° back a draw), 0.1 growing by 0.2 and 0.025 out growing by 0.25, from 0x5a46647f
/// fading (green and blue by 16, alpha by 8), texture 0xb.
fn draw_siren(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let at = host.joint(m, 8).unwrap_or(m.pv_v(SIREN_AT));
    let turn = m.pv_f(SIREN_TURN);
    let dir = [turn.cos(), turn.sin(), 0.0];
    let v = sub(cam, at);
    let rows = euler_rows([0.0, -atan2(len2(v), v[2]), atan2(v[0], v[1])]);
    let through = |c: V3, s: f32| -> V3 { std::array::from_fn(|k| (c[0] * rows[0][k] + c[1] * rows[1][k] + c[2] * rows[2][k]) * s + at[k]) };
    let (r, mut g, mut b, mut a) = (127i32, 100i32, 70i32, 90i32);
    let (mut size, mut step) = (0.1f32, 0.025f32);
    for _ in 0..16 {
        let col = ((a as u32) << 24) | ((b as u32) << 16) | ((g as u32) << 8) | r as u32;
        let t = scale(dir, step);
        let corners = SIREN_CORNERS.map(|c| add(through(c, size), t));
        out.push(FxQuad { fx: 0xb, additive: true, corners, st: QUAD_ST, rgba: [col; 4] });
        let back = scale(dir, step * -2.0);
        out.push(FxQuad { fx: 0xb, additive: true, corners: corners.map(|c| add(c, back)), st: QUAD_ST, rgba: [col; 4] });
        g = (g - 16).max(0);
        b = (b - 16).max(0);
        a = (a - 8).max(0);
        size += 0.2;
        step += 0.25;
    }
    let m = &mut w.mobys[id];
    m.set_pv_f(SIREN_TURN, add_rot(turn, -0.069_813_18));
}
