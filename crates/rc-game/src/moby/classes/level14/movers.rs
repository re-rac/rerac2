//! The prison's two moving platforms (L14 addresses; the decomp's `func_L14_003DDD38` / `func_L14_003DE4E8`, their
//! helpers `func_L14_003DE2D8`.. `func_L14_003DE908` at our `0x3df7b0`.. `0x3dfde0`).
//!
//! * **0x0ae4** ([`update_0ae4`], `0x3df210`, 2 placed): a bridge that swings a quarter turn between two ends about
//!   a pivot, sent to the end Ratchet is not at once the prison's guards on it are beaten.
//! * **0x0ae5** ([`update_0ae5`], `0x3df9c0`, one): a lift between its height and another that takes Ratchet up or
//!   down as he steps on it (or crosses its middle height), and Clank up.
//!
//! Both keep a platform block at `+0x20` (flag 1 set at the set-up) and write their tick's move into it (`0x32a0a0`
//! = L00 `0x322668`, crate::moby::platform::write); the port carries Ratchet with it (crate::moby::carry_hero, as
//! crate::moby::classes::level12::lifts).
//!
//! The L14 engine functions with the port's: `0x323250` = L00 `0x31b8d0` (crate::moby::Moby::set_state), `0x333648`
//! = L00 `0x32a7c8` (the first tick of the state), `0x333698` = L00 `0x32a818` (hooked), `0x32cea8` = L00 `0x325398`
//! (crate::moby::classes::level17::floaters::hero_on), `0x329290` = L00 `0x321858` (the cuboid test), `0x329638` =
//! L00 `0x321c00` (crate::camera::rot), `0x326270` = L00 `0x31e8d8` (the lerp), `0x3267b0` = L00 `0x31ed78`
//! (crate::hero::physics::ease), `0x2e61f8` = L00 `0x2de878` (the line), `0x2f6660` (an int countdown,
//! crate::moby::countdown), `0x36ada0` / `0x36aaa0` / `0x36aa58` = L00 `0x3605f0` / `0x3602f0` / `0x3602a8` (the
//! class sound, its stop, it plays).

use std::f32::consts::FRAC_PI_2;

use crate::hero::physics::{add, cross, dist, len, set_len, sub, V3};
use crate::moby::classes::level17::floaters::hero_on;
use crate::moby::{ai, countdown, mode, Ctx, MobyWorld};

pub const BRIDGE: i16 = 0x0ae4;
pub const RISER: i16 = 0x0ae5;

const BLOCK: usize = 0x20;
const BLOCK_FLAGS: usize = BLOCK + 0x3c;

fn stop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, o: usize) {
    let h = w.mobys[id].pv_i(o);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(o, -1);
}

fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, o: usize) {
    let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
    w.mobys[id].set_pv_i(o, h);
}

// ---------------------------------------------------------------------------------------------------
// 0x0ae4: the bridge.

const B_POINT: usize = 0x60;
const B_END: usize = 0x70;
const B_CUBOID: usize = 0x74;
const B_SOUND: usize = 0x78;
const B_STEP: usize = 0x7c;
const B_ANGLE: usize = 0x80;
const B_LEN_A: usize = 0x84;
const B_LEN_B: usize = 0x88;
const B_WAIT: usize = 0x8c;
const B_B: usize = 0x90;
const B_A: usize = 0xa0;
const B_PIVOT: usize = 0xb0;
const B_AXIS: usize = 0xc0;
const B_IDS: usize = 0xd0;
const B_LINKS: usize = 0xe0;
const B_DIST: usize = 0xf0;

/// 0x0ae4 (`0x3df210`). Its pvars: `+0x60` the far end's point, `+0x70` it starts there, `+0x74` a cuboid that wakes
/// it (−1 none), `+0x78` its looped sound, `+0x7c` its swing a tick (0.5°), `+0x80` the swing's angle (0 at end A,
/// 90° at B), `+0x84` / `+0x88` the reach to A and to B from the pivot, `+0x8c` a wait counted down, `+0x90` / `+0xa0`
/// the ends B and A, `+0xb0` the pivot (A at B's height), `+0xc0` the axis (B − pivot × A − pivot), `+0xd0..+0xdc`
/// the four guards on it (moby indices, 0 none), `+0xe0..+0xec` the same as links (cleared once one is dead,
/// `0x3df7e8`), `+0xf0..+0x10c` their reaches from the pivot at either end (read by nothing here).
///
/// **0** the set-up: its rows kept, at A (state 3) or with `+0x70` at B (4); the pivot, the reaches, the axis and
/// the guards'; with guards and a cuboid, 1. **1**: Ratchet in the cuboid, 2. **2**: 3, and on as 3. **3** / **4** at
/// rest (A / B): on its first tick the guards that wait to be hooked (`+0x2e0`) hooked and a wait of 100; once
/// Ratchet is on it, or within 30 of it and nearer the other end, with no guard left and the wait out, 6 / 5.
/// **5** / **6** the swing to A / B, its sound 0 looped from the first tick ([`swing`], `0x3df838`); there, the sound
/// stopped and (Ratchet off it) at rest.
pub fn update_0ae4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let mut state = w.mobys[id].state;
    match state {
        0 => {
            bridge_setup(w, id);
            let m = &w.mobys[id];
            if guards(m) != 0 && 0 <= m.pv_i(B_CUBOID) { w.mobys[id].set_state(1, -1); }
        }
        1 => {
            w.mobys[id].first();
            if ctx.level.in_cuboid(w.mobys[id].pv_i(B_CUBOID), ctx.hero.pos) { w.mobys[id].set_state(2, -1); }
        }
        _ => {}
    }
    if state == 2 {
        w.mobys[id].set_state(3, -1);
        state = 3;
    }
    match state {
        3 | 4 => {
            if w.mobys[id].first() {
                for k in 0..4 {
                    let Some(g) = ai::link(&w.mobys[id], B_LINKS + 4 * k) else { continue };
                    if w.mobys.get(g).is_some_and(|o| o.pv_i(0x2e0) != 0) { w.mobys[g].once |= 4; }
                }
                w.mobys[id].set_pv_i(B_WAIT, 100);
            }
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(B_WAIT);
            countdown(&mut t);
            m.set_pv_i(B_WAIT, t);
            let go = hero_on(ctx, id) || {
                let (h, p) = (ctx.hero.pos, m.pos);
                let (a, b) = (m.pv_v(B_A), m.pv_v(B_B));
                dist(h, p) < 30.0 && if dist(p, a) < dist(p, b) { dist(h, b) <= dist(h, a) } else { dist(h, a) < dist(h, b) }
            };
            let m = &mut w.mobys[id];
            if go && guards(m) == 0 && m.pv_i(B_WAIT) <= 0 { m.set_state(if state == 4 { 5 } else { 6 }, -1); }
        }
        5 => {
            if w.mobys[id].first() { start(w, id, ctx, B_SOUND); }
            if 0.0 < w.mobys[id].pv_f(B_ANGLE) {
                let s = -w.mobys[id].pv_f(B_STEP);
                swing(&mut w.mobys[id], s);
            } else {
                if 0 <= w.mobys[id].pv_i(B_SOUND) { stop(w, id, ctx, B_SOUND); }
                if !hero_on(ctx, id) { w.mobys[id].set_state(3, -1); }
            }
        }
        6 => {
            if w.mobys[id].first() { start(w, id, ctx, B_SOUND); }
            if FRAC_PI_2 <= w.mobys[id].pv_f(B_ANGLE) {
                if 0 <= w.mobys[id].pv_i(B_SOUND) { stop(w, id, ctx, B_SOUND); }
                if !hero_on(ctx, id) { w.mobys[id].set_state(4, -1); }
            } else {
                let s = w.mobys[id].pv_f(B_STEP);
                swing(&mut w.mobys[id], s);
            }
        }
        _ => {}
    }
    // `0x3df7e8`: the guards' links let go once dead.
    for k in 0..4 {
        let Some(g) = ai::link(&w.mobys[id], B_LINKS + 4 * k) else { continue };
        if w.mobys.get(g).is_none_or(|o| 0x80 <= o.state || o.pv_f(0x40) <= 0.0) { ai::set_link(&mut w.mobys[id], B_LINKS + 4 * k, None); }
    }
    finish(w, id, ctx, old, old_rot);
}

/// `0x3df7b0`: its guards left.
fn guards(m: &crate::moby::Moby) -> usize { (0..4).filter(|k| ai::link(m, B_LINKS + 4 * k).is_some()).count() }

fn bridge_setup(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.mode |= mode::KEEP_ROWS;
    m.set_pv_f(B_STEP, f32::from_bits(0x3c0e_fa36));
    m.set_pv_i(B_SOUND, -1);
    m.set_pv_i(BLOCK_FLAGS, m.pv_i(BLOCK_FLAGS) | 1);
    let (p, far) = (m.pos, m.pv_v(B_POINT));
    if m.pv_i(B_END) == 0 {
        m.set_pv_v(B_A, p);
        m.set_pv_v(B_B, far);
        m.set_pv_f(B_ANGLE, 0.0);
        m.set_state(3, -1);
    } else {
        m.set_pv_v(B_B, p);
        m.set_pv_v(B_A, far);
        m.set_pv_f(B_ANGLE, FRAC_PI_2);
        m.set_state(4, -1);
    }
    let (a, b) = (m.pv_v(B_A), m.pv_v(B_B));
    let pivot = [a[0], a[1], b[2]];
    m.set_pv_v(B_PIVOT, pivot);
    let (d1, d2) = (sub(a, pivot), sub(b, pivot));
    m.set_pv_f(B_LEN_A, len(d1));
    m.set_pv_f(B_LEN_B, len(d2));
    m.set_pv_v(B_AXIS, cross(d2, d1));
    let at_b = m.state == 4;
    for k in 0..4 {
        let idx = w.mobys[id].pv_i(B_IDS + 4 * k);
        let g = usize::try_from(idx).ok().filter(|&g| 0 < g && g < w.mobys.len());
        ai::set_link(&mut w.mobys[id], B_LINKS + 4 * k, g);
        let Some(g) = g else { continue };
        let f = len(sub(w.mobys[g].pos, pivot));
        let m = &mut w.mobys[id];
        let (la, lb) = (m.pv_f(B_LEN_A), m.pv_f(B_LEN_B));
        let (near, far) = if at_b { (f, la - lb + f) } else { (lb - la + f, f) };
        m.set_pv_f(B_DIST + 4 * k, near);
        m.set_pv_f(B_DIST + 0x10 + 4 * k, far);
    }
}

/// `0x3df838(m, step)`: its place about the pivot turned by `step` about the axis, the angle on, the reach the lerp of
/// A's and B's by it (0..90°); its rows: the second the axis, the third toward the pivot, the first their cross.
fn swing(m: &mut crate::moby::Moby, step: f32) {
    let pivot = m.pv_v(B_PIVOT);
    let axis = m.pv_v(B_AXIS);
    let v = crate::camera::rot(sub(m.pos, pivot), step, axis);
    let a = m.pv_f(B_ANGLE) + step;
    m.set_pv_f(B_ANGLE, a);
    let (la, lb) = (m.pv_f(B_LEN_A), m.pv_f(B_LEN_B));
    let v = set_len(v, la + (lb - la) * (a * f32::from_bits(0x3f22_f983)));
    m.pos = add(pivot, v);
    let r1 = set_len(axis, 1.0);
    let r2 = set_len(v, -1.0);
    m.rows = [cross(r2, r1), r1, r2];
}

/// Each tick's end: the platform block takes the move (its angles kept as they are), Ratchet carried.
fn finish(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, old: V3, old_rot: V3) {
    let m = &mut w.mobys[id];
    let (pos, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, BLOCK, sub(pos, old), old_rot, rot);
    crate::moby::carry_hero(ctx, id, old, old_rot[2], pos, rot[2]);
    w.changed = true;
}

// ---------------------------------------------------------------------------------------------------
// 0x0ae5: the lift that rides on its own.

const R_START_UP: usize = 0x60;
const R_TOP: usize = 0x64;
const R_BOTTOM: usize = 0x68;
const R_UP_NEXT: usize = 0x6c;
const R_TRIPS: usize = 0x70;
const R_SOUND: usize = 0x74;
const R_SPEED: usize = 0x78;
const R_ACCEL: usize = 0x7c;
const R_MIDDLE: usize = 0x80;
const R_CLANK: usize = 0x84;
const R_CUBOID: usize = 0x88;

/// 0x0ae5 (`0x3df9c0`). Its pvars: `+0x60` it goes up first, `+0x64` the other height (the top), `+0x68` its own
/// (the bottom), `+0x6c` up next, `+0x70` its trips, `+0x74` its looped sound, `+0x78` its speed, `+0x7c` its
/// acceleration (1/860), `+0x80` the heights' middle, `+0x84` it takes Clank, `+0x88` a cuboid it waits for Ratchet
/// to leave before it goes down.
///
/// **0** the set-up: its bottom its height, the middle, up next with `+0x60`; 1. **1** waiting: Clank (`0x18c0d4` =
/// 1) with `+0x84` sends it up (2) when up is next, its sound 0 looped; Ratchet on it, or across the middle height
/// from it after a first trip, sends it the way that is next (2 up, 3 down), its sound started unless it plays.
/// **2** up: on its first tick its speed 0 and a trip more; under 0.03 under the top it is there — the sound stopped
/// and, with Ratchet off it and not over it within 3 ([`over`], `0x3dfde0`), down next, at the top, 1; else eased up
/// (both rates `+0x7c`, at most 0.1). **3** down: the same to the bottom (up next), eased only with Ratchet on it or
/// out of the cuboid `+0x88`.
pub fn update_0ae5(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let st = w.mobys[id].state;
    match st {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(BLOCK_FLAGS, m.pv_i(BLOCK_FLAGS) | 1);
            let z = m.pos[2];
            m.set_pv_i(R_SOUND, -1);
            m.set_pv_f(R_ACCEL, f32::from_bits(0x3a98_ead7));
            m.set_pv_f(R_BOTTOM, z);
            m.set_pv_i(R_TRIPS, 0);
            m.set_pv_f(R_MIDDLE, (z + m.pv_f(R_TOP)) * 0.5);
            m.set_pv_i(R_UP_NEXT, (m.pv_i(R_START_UP) != 0) as i32);
            m.state = 1;
        }
        1 => {
            let m = &w.mobys[id];
            if ctx.hero.mode == 1 && m.pv_i(R_CLANK) != 0 {
                if m.pv_i(R_UP_NEXT) == 1 {
                    w.mobys[id].set_state(2, -1);
                    start(w, id, ctx, R_SOUND);
                }
            } else {
                let mid = m.pv_f(R_MIDDLE);
                let go = hero_on(ctx, id) || ((m.pos[2] - mid) * (ctx.hero.pos[2] - mid) < 0.0 && m.pv_i(R_TRIPS) != 0);
                if go {
                    let s = if m.pv_i(R_UP_NEXT) == 0 { 3 } else { 2 };
                    w.mobys[id].set_state(s, -1);
                    let h = w.mobys[id].pv_i(R_SOUND);
                    if !(h != -1 && ctx.host.sound_playing(h)) { start(w, id, ctx, R_SOUND); }
                }
            }
        }
        s @ (2 | 3) => {
            let up = s == 2;
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.set_pv_f(R_SPEED, 0.0);
                m.set_pv_i(R_TRIPS, m.pv_i(R_TRIPS) + 1);
            }
            let m = &w.mobys[id];
            let (t, z) = (m.pv_f(if up { R_TOP } else { R_BOTTOM }), m.pos[2]);
            let left = if up { t - z } else { z - t };
            if 0.03 <= left {
                let ease = up || hero_on(ctx, id) || !ctx.level.in_cuboid(m.pv_i(R_CUBOID), ctx.hero.pos);
                if ease {
                    let m = &mut w.mobys[id];
                    let (a, mut z, mut v) = (m.pv_f(R_ACCEL), m.pos[2], m.pv_f(R_SPEED));
                    crate::hero::physics::ease(t, a, a, f32::from_bits(0x3dcc_ccce), &mut z, &mut v);
                    m.pos[2] = z;
                    m.set_pv_f(R_SPEED, v);
                }
            } else {
                stop(w, id, ctx, R_SOUND);
                if !over(id, ctx) {
                    let m = &mut w.mobys[id];
                    m.set_pv_i(R_UP_NEXT, (!up) as i32);
                    m.pos[2] = t;
                    m.set_state(1, -1);
                }
            }
        }
        _ => {}
    }
    finish(w, id, ctx, old, old_rot);
}

/// `0x3dfde0`: Ratchet on it, or the line from him 3 down (flags 0x10) meeting it first.
pub(crate) fn over(id: usize, ctx: &Ctx) -> bool {
    if hero_on(ctx, id) { return true; }
    let a = ctx.hero.pos;
    let b = [a[0], a[1], a[2] - 3.0];
    ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, a, b, crate::collision::QueryFlags(0x10), None)).is_some_and(|h| h.moby == Some(id))
}
