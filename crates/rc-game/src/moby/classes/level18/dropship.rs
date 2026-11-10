//! Damosel's dropship **0x0ede** (L18 `0x4300a8`, 7 placed; the decomp's `func_L18_0042EBB8`): it carries up to six
//! stalkers 0x0f0a (crate::moby::classes::level18::stalker) on its sides, comes in along a path once Ratchet is in
//! its cuboid, hovers to drop them a group at a time, and leaves along its own path.
//!
//! **Its pvars:** health `+0x20` (its most `+0x24`, its status `+0x2e`: 2 a kill asked by another), its break pieces
//! `+0x50` (`+0x60` the health last seen), `+0x70` the path point it eases to, `+0x80` its velocity, `+0x90..+0xa8` its
//! six stalkers (moby indices, −1 none), `+0xa8` / `+0xac` its two ways in (−1 none), `+0xb0..+0xbc` each stalker's
//! group (s16), `+0xbc` the path point (s16), `+0xbe` the group it drops next, `+0xc0` its way out, `+0xc4` the cuboid
//! that wakes it (−1 none), `+0xc8` / `+0xcc` the bob's phase and last offset, `+0xd0` / `+0xd4` the sway's phases,
//! `+0xd8` its turn speed, `+0xdc` its loop sound.
//!
//! **Each tick** ([`update_0ede`]): from state 2 its loop sound 0 kept playing, before let go. The hit (flags
//! 0x10000, `0x338e08` / `0x338e68` = L00 `0x31dce8` / `0x31dd48`, crate::moby::ai::damage_of, the record let go):
//! from 2 off its health, its break pieces by it (crate::moby::shards::part_break); dead (or its
//! status 2) a blast at it (size 1), its stalkers gone ([`stalker::die`]), its sound let go, its pieces thrown
//! (crate::moby::shards::break_into, all of 63, 3..6 / 3..6, its velocity plus a random 0.1..0.25) and it is gone.
//! Its ticks after the state.
//!
//! **States** (`+0x20`): 0 the set-up: sequence 1 held still, no collision, hidden and a target, 1, its status 1,
//! its sound let go; none of its stalkers left: gone. 1 waiting: Ratchet in its cuboid → shown with its collision, 3
//! (`0x430c58`). 3 the way in (its group's path, from the point nearest it, `0x35e438`): along it ([`path_step`],
//! points passed within 5, the last 0.3), at its end 2 out of 3 (a second way in left) or 4. 2 the drop: at its start
//! its stalkers carried ([`stalker::carried`]), speed 1; past frame 8 the group's stalkers dropped
//! ([`stalker::release`]) and let go, the next group, still; after 31 ticks speed 1; past frame 59 still and the
//! state before. 4 the way out (from its nearest point, within 2), at its end gone.
//!
//! **The flight** ([`fly`], `0x4307f8(m, 1, 1)`, the only call): it bobs 1 (0.5 in 2) at 90°/s (`0x33f1b0` = L00
//! `0x324430`) and sways 15° at 87°/s and 211°/s (`0x33f268` = L00 `0x3244e8`), each axis eased to its point
//! (`0x339e98` = L00 `0x31ed78`, [`ease`]: 40/3600 a tick², 20/60 a tick at most); farther than 1 across it turns to it
//! (`0x33a350` = L00 `0x31f230`: 360″, 360″, 180°/s). Its stalkers ride at their places in its frame (`0x2a9ba0`,
//! `0x309b08` = L00 `0x2ef318`), tilted with it (the left three by its pitch and roll swapped, the right three
//! negated) and turned by their yaws (`$gp+0x2418..`: ±65°, ±90°, ±115°).
//!
//! The tunables (`$gp+0x23e0..0x2410`) are written in as their values. Not ported: its thrusters (`0x430ca8` at
//! `$gp+0x2430` and `0x1b1430`: the particle bursts `0x35a848` = L02 `0x345d88` by its speed) and its draw (`0x302130`
//! with `0x430f98`: two glow sprites `0x430fd0` at the same places).

use super::stalker;
use crate::hero::physics::{add, add_rot, approach, atan2, dist, dist2, scale, turn_ease, V3};
use crate::moby::blast::{self, Blast};
use crate::moby::classes::props::{bob, sway};
use crate::moby::{ai, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0ede;

const H: usize = 0x20;
const STATUS: usize = 0x2e;
const PIECES: usize = 0x50;
const POINT_AT: usize = 0x70;
const VEL: usize = 0x80;
const RIDERS: usize = 0x90;
const PATHS: usize = 0xa8;
const GROUPS: usize = 0xb0;
const POINT: usize = 0xbc;
const GROUP: usize = 0xbe;
const ESCAPE: usize = 0xc0;
const TRIGGER: usize = 0xc4;
const BOB: usize = 0xc8;
const BOB_LAST: usize = 0xcc;
const SWAY_A: usize = 0xd0;
const SWAY_B: usize = 0xd4;
const TURN: usize = 0xd8;
const VOICE: usize = 0xdc;
const PVARS: usize = 0xe0;

/// °/s as radians a tick.
const DEG_S: f32 = 0.000_290_888_25;
/// `$gp+0x23e0` / `+0x23f4`: the bob (in 2), `+0x23e4` its rate.
const BOB_AMP: [f32; 2] = [1.0, 0.5];
const BOB_RATE: f32 = 90.0;
/// `$gp+0x23e8` (= `+0x23f8`): the sway, `+0x23ec` / `+0x23f0` its rates.
const SWAY: f32 = 0.261_799_42;
const SWAY_RATES: [f32; 2] = [87.0, 211.0];
/// `$gp+0x23fc` / `+0x2400` (″ a tick²) and `+0x2404` (°/s): the turn.
const TURN_ACCEL: f32 = 360.0 * 4.848_137_5e-6;
const TURN_MOST: f32 = 180.0 * DEG_S;
/// `$gp+0x2408` / `+0x240c` (/ 3600) and `+0x2410` (/ 60): the ease.
const EASE_K: f32 = 40.0 * 0.000_277_777_78;
const EASE_MOST: f32 = 20.0 * 0.016_666_668;
/// `0x2a9ba0`: the stalkers' places in its frame.
const SEATS: [V3; 6] = [[-1.1, 3.0, -2.7], [-1.85, 3.0, -2.7], [-2.6, 3.0, -2.7], [-1.1, -3.0, -2.7], [-1.85, -3.0, -2.7], [-2.6, -3.0, -2.7]];
/// `$gp+0x2418..`: their yaws.
const SEAT_YAWS: [f32; 6] = [1.134_464_1, 1.570_796_5, 2.007_129, -1.134_464_1, -1.570_796_5, -2.007_129];

fn rider(w: &MobyWorld, id: usize, i: usize) -> Option<usize> {
    usize::try_from(w.mobys[id].pv_i(RIDERS + 4 * i)).ok().filter(|&k| k < w.mobys.len())
}

/// Its loop sound let go.
fn stop_voice(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(VOICE);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(VOICE, -1);
}

/// `0x31ed78(t, accel, brake, max, &x, &v)`: `x` toward `t` with a velocity built by `accel` toward
/// `min(√(2·brake·|d|), max)`, braked (by `brake`, 1.1 × once it would stop short) so as to land on it; turned back by
/// `brake` while it goes the wrong way. Returns the velocity (the last difference on landing).
pub fn ease(t: f32, accel: f32, brake: f32, max: f32, x: &mut f32, v: &mut f32) -> f32 {
    let d = t - *x;
    if *v == 0.0 && d == 0.0 { return 0.0; }
    if *v * d < 0.0 {
        *v += if 0.0 <= *v { -brake } else { brake };
        *x += *v;
        return *v;
    }
    let stop = *v * *v / brake * 0.5;
    if d.abs() < stop {
        let step = if stop < d.abs() + v.abs() { brake } else { brake * 1.1 };
        approach(0.0, step, v);
    } else {
        let want = ((brake + brake) * d).abs().sqrt().min(max);
        approach(if d < 0.0 { -want } else { want }, accel, v);
    }
    if d.abs() <= v.abs() {
        *x = t;
        return d;
    }
    *x += *v;
    *v
}

/// `0x35e438(0, p, path)`: the path's point nearest `p` (as crate::moby::classes::breeder's).
fn nearest_point(ctx: &Ctx, p: V3, path: i32) -> i16 {
    let Some(pts) = usize::try_from(path).ok().and_then(|i| ctx.level.paths.get(i)).filter(|v| !v.is_empty()) else { return -1 };
    let mut best = 1e11;
    let mut at = 0;
    for (i, q) in pts.iter().enumerate() {
        let d = dist([q[0], q[1], q[2]], p).abs();
        if d < best {
            best = d;
            at = i;
        }
    }
    at as i16
}

/// `0x430b78(r, m, &to, path, &point, 0)`: the point it eases to its path point; passed within `r`, the next; false
/// past the last.
fn path_step(w: &mut MobyWorld, id: usize, ctx: &Ctx, r: f32, path: i32) -> bool {
    let Some(pts) = usize::try_from(path).ok().and_then(|i| ctx.level.paths.get(i)).filter(|v| !v.is_empty()) else { return false };
    let m = &mut w.mobys[id];
    let mut i = m.pv_s(POINT);
    let q = pts[(i.max(0) as usize).min(pts.len() - 1)];
    m.set_pv_v(POINT_AT, [q[0], q[1], q[2]]);
    m.set_pv_f(POINT_AT + 0xc, q[3]);
    if dist(m.pos, [q[0], q[1], q[2]]) < r { i += 1; }
    m.set_pv_s(POINT, i);
    (i as i32) < pts.len() as i32
}

/// `0x4307f8(m, 1, 1)` (module docs).
fn fly(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let amp = BOB_AMP[(m.state == 2) as usize];
    bob(m, amp, BOB_RATE * DEG_S, -1.0, BOB, BOB_LAST);
    sway(m, SWAY, SWAY_RATES[0] * DEG_S, SWAY_RATES[1] * DEG_S, SWAY_A, SWAY_B);
    let to = m.pv_v(POINT_AT);
    for k in 0..3 {
        let mut v = m.pv_f(VEL + 4 * k);
        ease(to[k], EASE_K, EASE_K, EASE_MOST, &mut m.pos[k], &mut v);
        m.set_pv_f(VEL + 4 * k, v);
    }
    if 1.0 < dist2(m.pos, to) {
        let yaw = atan2(to[0] - m.pos[0], to[1] - m.pos[1]);
        let mut s = m.pv_f(TURN);
        turn_ease(yaw, TURN_ACCEL, TURN_ACCEL, TURN_MOST, &mut m.rot[2], &mut s, 0);
        m.set_pv_f(TURN, s);
    }
    let (pos, rows, rot) = (m.pos, m.rows, m.rot);
    for i in 0..6 {
        let Some(k) = rider(w, id, i) else { continue };
        let o = SEATS[i];
        let e = &mut w.mobys[k];
        e.pos = add(add(add(scale(rows[0], o[0]), scale(rows[1], o[1])), scale(rows[2], o[2])), pos);
        if i < 3 {
            e.rot[0] = rot[1];
            e.rot[1] = -rot[0];
        } else {
            e.rot[0] = -rot[1];
            e.rot[1] = rot[0];
        }
        e.rot[2] = add_rot(rot[2], SEAT_YAWS[i]);
    }
    w.changed = true;
}

/// 0x0ede (module docs).
pub fn update_0ede(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    if w.mobys[id].state < 2 {
        if ctx.host.sound_playing(w.mobys[id].pv_i(VOICE)) { stop_voice(w, id, ctx); }
    } else if !ctx.host.sound_playing(w.mobys[id].pv_i(VOICE)) {
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(VOICE, h);
    }
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
    let dmg = ai::damage_of(w, id, hit.as_ref(), false);
    w.clear_hit(id);
    if 2 <= w.mobys[id].state {
        let hp = w.mobys[id].pv_f(H) - dmg;
        w.mobys[id].set_pv_f(H, hp);
        crate::moby::shards::part_break(w, ctx, id, PIECES, id, H);
        if w.mobys[id].pv_f(H) <= 0.0 || w.mobys[id].pv_b(STATUS) == 2 {
            let at = w.mobys[id].pos;
            blast::burst(w, ctx, at, &Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true });
            for i in 0..6 {
                if let Some(k) = rider(w, id, i) { stalker::die(w, k, ctx); }
            }
            if ctx.host.sound_playing(w.mobys[id].pv_i(VOICE)) { stop_voice(w, id, ctx); }
            let v = add(ctx.game.rng.rand_vec(0.1, 0.25), w.mobys[id].pv_v(VEL));
            crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 63, v, 0xff);
            w.delete(id);
            return;
        }
    }
    match w.mobys[id].state {
        0 => {
            ctx.host.set_anim(&w.mobys[id], 1, 0, 0);
            ctx.host.set_anim_speed(&w.mobys[id], 0.0);
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.mode = (m.mode & 0xefff) | 0x41;
            m.set_state(1, -1);
            m.set_pv_b(STATUS, 1);
            stop_voice(w, id, ctx);
            w.mobys[id].mode |= mode::TARGET;
            let left = (0..6).any(|i| rider(w, id, i).is_some_and(|k| { let s = w.mobys[k].state; s != 0xfe && s != 0xfd }));
            if !left {
                w.delete(id);
                return;
            }
        }
        1 => {
            let t = w.mobys[id].pv_i(TRIGGER);
            if t != -1 && ctx.level.in_cuboid(t, ctx.hero.pos) {
                // `0x430c58`.
                let coll = w.class(CLASS).is_some_and(|c| c.has_collision);
                let m = &mut w.mobys[id];
                if m.state == 1 {
                    m.mode = (m.mode & 0xffbe) | mode::TARGET;
                    m.has_collision = coll;
                    m.set_state(3, -1);
                }
            }
        }
        2 => {
            let fr = ctx.host.frame_time(&w.mobys[id]);
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                for i in 0..6 {
                    if let Some(k) = rider(w, id, i) { stalker::carried(w, k, id); }
                }
            } else {
                match w.mobys[id].timer {
                    0 => {
                        fly(w, id);
                        if 8.0 < fr {
                            w.mobys[id].timer = 1;
                            let g = w.mobys[id].pv_s(GROUP);
                            for i in 0..6 {
                                if w.mobys[id].pv_s(GROUPS + 2 * i) != g { continue; }
                                let Some(k) = rider(w, id, i) else { continue };
                                stalker::release(w, k);
                                w.mobys[id].set_pv_i(RIDERS + 4 * i, -1);
                            }
                            w.mobys[id].set_pv_s(GROUP, g + 1);
                            ctx.host.set_anim_speed(&w.mobys[id], 0.0);
                        }
                    }
                    1 => {
                        fly(w, id);
                        if 0x1f <= w.mobys[id].state_ticks as u16 {
                            w.mobys[id].timer = 2;
                            ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                        }
                    }
                    2 => {
                        fly(w, id);
                        if 59.0 < fr {
                            ctx.host.set_anim_speed(&w.mobys[id], 0.0);
                            let m = &mut w.mobys[id];
                            let p = m.prev_state;
                            m.set_state(p, -1);
                        }
                    }
                    _ => {}
                }
            }
        }
        3 => {
            let g = w.mobys[id].pv_s(GROUP).clamp(0, 1) as usize;
            let path = w.mobys[id].pv_i(PATHS + 4 * g);
            if w.mobys[id].first() {
                let p = nearest_point(ctx, w.mobys[id].pos, path);
                w.mobys[id].set_pv_s(POINT, p);
            } else {
                let last = usize::try_from(path).ok().and_then(|i| ctx.level.paths.get(i)).map_or(0, |p| p.len() as i32) - 1;
                let r = if w.mobys[id].pv_s(POINT) as i32 == last { 0.3 } else { 5.0 };
                if !path_step(w, id, ctx, r, path) {
                    let m = &mut w.mobys[id];
                    let next = if m.pv_s(GROUP) < 1 && m.pv_i(PATHS + 4) != -1 { 3 } else { 4 };
                    m.set_state(next, -1);
                    m.set_state(2, -1);
                }
                fly(w, id);
            }
        }
        4 => {
            let path = w.mobys[id].pv_i(ESCAPE);
            if w.mobys[id].first() {
                let p = nearest_point(ctx, w.mobys[id].pos, path);
                w.mobys[id].set_pv_s(POINT, p);
            } else {
                if !path_step(w, id, ctx, 2.0, path) {
                    if ctx.host.sound_playing(w.mobys[id].pv_i(VOICE)) { stop_voice(w, id, ctx); }
                    w.delete(id);
                    return;
                }
                fly(w, id);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    w.changed = true;
}
