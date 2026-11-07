//! The gunship **0x0cbc** (L09 `0x3ebf90`, 3 placed; L20 `0x412cb8`, 9 placed, the same code and tunables; the
//! decomp's `func_L09_003EAB20`, its helpers `func_L09_003E99E0`.. `func_L09_003EAF60` at our `0x3eae50`..
//! `0x3ec3d0`): a hovering craft on five jets that flies in along a path, patrols another back and forth, and once
//! Ratchet is in its area follows that path toward him, firing bursts of bolts (0x05de,
//! crate::moby::classes::level09::bolt) from its joint 2. It is L16's drone (crate::moby::classes::level16::drone)
//! with the paths: the same framework (crate::moby::ai), jets and joint aims. Health block `+0x20` (`+0x34` the
//! bolts' damage, `+0x35` the jets'), movement `+0x50` (radius 3.5, the turn left 30°; its path `+0xc4`, the point
//! `+0x118` and the last `+0x11a`), target block `+0x140` (found `+0x154`), reaction `+0x180` (a hurt to state 5, sub 2).
//!
//! **Its pvars:** `+0x230` / `+0x2b0` its joint aims (lists 0 and 1: its body's turn toward `+0x298`, its gun's tilt
//! toward `+0x314`; crate::moby::joint_aim), `+0x330` / `+0x340` its place and angles without the bob and sway,
//! `+0x350` its patrol path, `+0x354` the area searched, `+0x358` / `+0x35c` the sway's phases, `+0x360` / `+0x364`
//! the bob's phase and offset, `+0x368` the wait between bolts (u8), `+0x369` the bolts of the burst, `+0x36a` the
//! wait between bursts (u16), `+0x36c` its path in (−1 none), `+0x374` / `+0x378` the areas calm and alerted,
//! `+0x37c` (cleared after a hurt), `+0x380` in from its path, `+0x384` the bolts' range (under 1: 30).
//!
//! **Each tick** ([`update_0cbc`]): nothing while `$gp+0x1c0c` (`0x1b0bfc`, 0) is set or out of the loaded variant
//! (`0x31dee8`). The head (`0x3ebe88`): in sub 0 the set-up ([`setup`], `0x3ebc08`); the shadow within 32 (range 0x1e);
//! the frame's damage (`0x324590` = L00 `0x327e38`, crate::moby::ai::take_damage): killed, state 6; the search
//! (`0x3275b8` = L00 `0x32ad38`) in the area `+0x354`. Back at `+0x330`.
//!
//! **Sub-states** (`+0x95`): **1** waiting: found or hooked (`0x3270b8` = L00 `0x32a838`), the alerted area and the
//! way in (2, sub 2). **3** patrolling: found, the senses B and the attack (4, sub 4). **4** attacking: lost and not
//! mid-burst, sequence 0, the senses A and the patrol (3, sub 3). The senses (`0x32a218` = L00 `0x32dcf0`): A
//! (5, 3, 180, 120), B (5, 6, 180, 180), at the set-up (10, 8, 90, 120) — in 1/3600, 1/60, arc-seconds and arc-minutes
//! (`0x1b0ba0..0x1b0bcc`).
//!
//! **States** (`+0x20`): **2** the way in: along `+0x36c` ([`follow`]); at its end (or with none) the senses A, onto the
//! patrol path at its point nearest ([`start`]), in from its path and the patrol (3, sub 3). **3** the patrol: along
//! `+0x350`; at its end the other way. **4** the attack: toward its target along the patrol path ([`chase`]). **5**
//! hurt: the framework's flight (`0x3252d8` = L00 `0x328b80`); over, without health the death, else the attack (in
//! from its path) or (sequence 0) the way in, the alerted area. **6** dead ([`die`], `0x3ec3d0`): its class sound 2,
//! its bolts (`0x316fe8`, limit 3), the blast (`0x3f55b0` = L00 `0x3f8348`: size 1, 25 / 20 / 20 sprites, 20 puffs, 45
//! streaks, the flash ball, the shake) and gone.
//!
//! **The flight** ([`fly`], `0x3eb2d0`): toward a goal at most its top speed (`+0x78`) away: out of the attack turned
//! to it (`0x31f230`) and slowed by `1 − e²` with e its turn left over `+0x80`; attacking, side-on to Ratchet; its
//! speed eased (`0x31f4b0`) by the distance it is given (along a path: to its last point), on along the goal. Along a
//! path ([`follow`], `0x3eb4c8`) the goal is 10 ticks of its top speed on from its point toward the last
//! (crate::paths::step, L01 `0x337888`); toward its target ([`chase`], `0x3eb578`) the patrol path's point nearest the
//! target (crate::paths::closest, L00 `0x341fa8`), from over 2 away by the step either way that comes nearer it.
//!
//! **The tail**: the hover ([`hover`], `0x3eae50`: its place kept, the bob 0.2 at 0.5° a tick about it, the sway 5° at
//! 0.42° / 0.52° a tick, its angles back on, its pitch then 0); the aims and bolts ([`aims`], `0x3eb7d0`); the jets
//! ([`jets`], `0x3eaf10`); its ticks and the two waits (`0x3ebe40`).
//!
//! The port takes the level's load (`0x1b230c` / `0x1b2310`) as light: the jets always smoke when it is drawn.
//!
//! Not ported: in the game mode 9 (`$gp−0x75f8`) its kill clearing `$gp−0x5fb0` by the record `+0x1f0` (as
//! crate::moby::classes::trooper's); the path lengths written into the points' `w` at the set-up (L09
//! `0x33fa00`, read by nothing here); the debug prints of sub and state 0.

use crate::hero::physics::{add, add_rot, atan2, dist, dist2, ease_speed, len, len2, rot_dist, scale, set_len, sub, sub_rot, turn_ease, V3};
use crate::moby::classes::props::{bob, sway};
use crate::moby::{ai, bolts, joint_aim, Ctx, Moby, MobyWorld};

pub const CLASS: i16 = 0x0cbc;

const HEALTH: usize = 0x20;
const DAMAGE: usize = 0x34;
const JET_DAMAGE: usize = 0x35;
const MV: usize = 0x50;
const VEL: usize = MV + 0xa0;
const TURN_V: usize = MV + 0xb4;
const PATH_AT: usize = MV + 0xc4;
const CUR: usize = MV + 0xc8;
const LAST: usize = MV + 0xca;
const T_POINT: usize = 0x140;
const FOUND: usize = 0x154;
const AIM_TURN: usize = 0x230;
const AIM_TILT: usize = 0x2b0;
const BASE: usize = 0x330;
const BASE_ROT: usize = 0x340;
const PATROL: usize = 0x350;
const AREA: usize = 0x354;
const SWAY_A: usize = 0x358;
const SWAY_B: usize = 0x35c;
const BOB: usize = 0x360;
const BOB_LAST: usize = 0x364;
const WAIT: usize = 0x368;
const SHOTS: usize = 0x369;
const PAUSE: usize = 0x36a;
const PATH_IN: usize = 0x36c;
const AREA_CALM: usize = 0x374;
const AREA_ALERT: usize = 0x378;
const W37C: usize = 0x37c;
const IN: usize = 0x380;
const RANGE: usize = 0x384;
const PVARS: usize = 0x388;

/// The senses (`0x1b0ba0..`): at the set-up, A, B.
const SENSES_SETUP: [f32; 4] = [10.0, 8.0, 90.0, 120.0];
const SENSES_A: [f32; 4] = [5.0, 3.0, 180.0, 120.0];
const SENSES_B: [f32; 4] = [5.0, 6.0, 180.0, 180.0];
/// `0x1b0b84`: the bolts' speed a second.
const SHOT_SPEED: f32 = 20.0;
/// `0x1b0b88` / `0x1b0b8c`: the bolts of a burst (one more) and the ticks between them.
const BURST: u8 = 6;
const BETWEEN: u8 = 20;
/// `0x1b0b90` / `0x1b0b94` / `0x1b0b98`: the ticks between bursts, within 10 of its target 15.
const PAUSE_FAR: i16 = 180;
const PAUSE_NEAR: i16 = 15;
const NEAR: f32 = 10.0;
/// `0x1b0b9c`: the ticks the aims take to come to full strength in the attack.
const BLEND_TICKS: f32 = 30.0;
/// `0x1b0bd4`: the bolts' range when `+0x384` has none.
const RANGE_DEFAULT: f32 = 30.0;
/// `0x1b0bdc` / `0x1b0be0` / `0x1b0be4`: the lead's random share and the aim's jitter (all 0).
const LEAD: [f32; 2] = [0.0, 0.0];
const JITTER: f32 = 0.0;
/// `0x1b0be8` / `0x1b0bec`: the aims' springs.
const AIM_K: f32 = f32::from_bits(0x3c23_d70a);
const AIM_D: f32 = f32::from_bits(0x3ca3_d70a);
/// `0x1b0bf0` / `0x1b0bf4` / `0x1b0bf8`: the jets' reach across Ratchet, the drop under them and their sphere.
const JET_REACH: f32 = 2.0;
const JET_DROP: f32 = f32::from_bits(0x3f33_3333);
const JET_R: f32 = f32::from_bits(0x3f59_999a);

fn st(w: &mut MobyWorld, id: usize, s: u8, sub: i32) { w.mobys[id].set_state(s, sub); }

/// `0x32dcf0` with the tunables in their units.
fn senses(m: &mut Moby, t: [f32; 4]) {
    ai::senses(m, t[0] * f32::from_bits(0x3991_a2b4), t[1] * f32::from_bits(0x3c88_8889), t[2] * f32::from_bits(0x36a2_ad2f), t[3] * f32::from_bits(0x3998_825c));
}

fn path<'a>(ctx: &'a Ctx, i: i32) -> Option<&'a Vec<[f32; 4]>> { usize::try_from(i).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| !p.is_empty()) }

/// `0x31b5f0(m, s, 0, 0)` when sequence `s` is not the one it shows.
fn seq_to(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8) {
    if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_b != s) { ctx.host.set_anim(&w.mobys[id], s, 0, 0); }
}

/// 0x0cbc (`0x3ebf90`, module docs).
pub fn update_0cbc(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    if !ctx.level.in_variant(w.mobys[id].pos, ctx.game.variant) { return; }
    head(w, id, ctx);
    let m = &mut w.mobys[id];
    m.pos = m.pv_v(BASE);
    let found = m.pv_i(FOUND) == 1;
    match m.sub_state {
        1 => {
            if found || m.once & 4 != 0 {
                m.set_pv_i(AREA, m.pv_i(AREA_ALERT));
                st(w, id, 2, 2);
            }
        }
        3 => {
            if found {
                senses(m, SENSES_B);
                st(w, id, 4, 4);
            }
        }
        4 => {
            if !found && m.pv_b(SHOTS) == 0 {
                seq_to(w, id, ctx, 0);
                senses(&mut w.mobys[id], SENSES_A);
                st(w, id, 3, 3);
            }
        }
        _ => {}
    }
    match w.mobys[id].state {
        2 => {
            let p = w.mobys[id].pv_i(PATH_IN);
            if p == -1 || follow(w, id, ctx, p) {
                senses(&mut w.mobys[id], SENSES_A);
                let patrol = w.mobys[id].pv_i(PATROL);
                start(w, id, ctx, patrol, -1);
                w.mobys[id].set_pv_i(IN, 1);
                st(w, id, 3, 3);
            }
        }
        3 => {
            let p = w.mobys[id].pv_i(PATROL);
            if p == -1 || follow(w, id, ctx, p) {
                let m = &mut w.mobys[id];
                if m.pv_s(LAST) != 0 {
                    m.set_pv_s(LAST, 0);
                } else {
                    let last = path(ctx, p).map_or(-1, |q| q.len() as i16 - 1);
                    w.mobys[id].set_pv_s(LAST, last);
                }
            }
        }
        4 => {
            let p = w.mobys[id].pv_i(PATROL);
            if p != -1 { chase(w, id, ctx, p); }
        }
        5 => {
            if ai::flight(w, id, ctx) & 2 != 0 {
                if w.mobys[id].pv_f(HEALTH) <= 0.0 {
                    st(w, id, 6, -1);
                } else {
                    w.mobys[id].set_pv_i(W37C, 0);
                    if w.mobys[id].pv_i(IN) != 0 {
                        let m = &mut w.mobys[id];
                        m.set_pv_b(SHOTS, 0);
                        m.set_pv_i(AREA, m.pv_i(AREA_ALERT));
                        st(w, id, 4, 4);
                    } else {
                        seq_to(w, id, ctx, 0);
                        let m = &mut w.mobys[id];
                        m.set_pv_i(AREA, m.pv_i(AREA_ALERT));
                        st(w, id, 2, 2);
                    }
                }
            }
        }
        6 => return die(w, id, ctx),
        _ => {}
    }
    hover(&mut w.mobys[id]);
    w.mobys[id].rot[0] = 0.0;
    aims(w, id, ctx);
    jets(w, id, ctx);
    // `0x3ebe40`: its ticks, the two waits.
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    let mut t = m.pv_s(PAUSE) as i32;
    crate::moby::countdown(&mut t);
    m.set_pv_s(PAUSE, t as i16);
    let b = m.pv_b(WAIT);
    m.set_pv_b(WAIT, b.saturating_sub(1));
    w.changed = true;
}

/// `0x3ebe88` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].sub_state == 0 { setup(w, id, ctx); }
    crate::shadow_volumes::probe_near(w, id, ctx, 32.0, 0x1e);
    if 4 <= ai::take_damage(w, id, ctx, false) { st(w, id, 6, -1); }
    let area = w.mobys[id].pv_i(AREA);
    ai::find_target(w, id, ctx, 0.0, 0.0, area, None, false);
}

/// `0x3ebc08`: its place and angles kept; the framework's set-up (the movement block: radius 3.5, slope 180°, no
/// gravity, the turn left 30°; the reaction block: a hurt to state 5); the senses; the calm area; on its path in at
/// that path's first point (onto it at its nearest, [`start`]), else in already; state 1 (sub 1).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_pv_v(BASE, m.pos);
    m.set_pv_v(BASE_ROT, m.rot);
    m.set_pv_b(0x48, 25);
    m.set_pv_b(0x4a, 19);
    for (o, v) in [(0x1ec, 4u8), (0x1ee, 4), (0x191, 2), (0x192, 2), (0x196, 2), (0x197, 2), (0x1ed, 8), (0x1ef, 8), (0x190, 5), (0x1f2, 4), (0x1f3, 8), (0x1f6, 1), (0x28, 3), (0x2f, 2)] { m.set_pv_b(o, v); }
    m.set_pv_i(0x1c8, 5);
    m.set_pv_f(0x1ac, f32::from_bits(0x3ab6_0b61));
    m.set_pv_f(0x1a4, f32::from_bits(0x3b23_d70a));
    m.set_pv_f(0x1a8, f32::from_bits(0x3b23_d70a));
    m.set_pv_f(0x1b0, 1.5);
    m.set_pv_f(0x1bc, f32::from_bits(0x3ecc_cccd));
    m.set_pv_f(0x30, 1.0);
    for o in [0x19c, 0x1a0, 0x1b4, 0x54, 0x58, 0x5c, 0x7c] { m.set_pv_i(o, 0); }
    m.set_pv_s(0x1b8, 0);
    m.set_pv_f(0x50, 3.5);
    m.set_pv_f(0x60, f32::from_bits(0x4049_0fdc));
    m.set_pv_f(0x80, f32::from_bits(0x3f06_0a93));
    senses(m, SENSES_SETUP);
    m.set_pv_i(AREA, m.pv_i(AREA_CALM));
    let entry = m.pv_i(PATH_IN);
    match path(ctx, entry).map(|p| p[0]) {
        Some(p) => {
            w.mobys[id].pos = [p[0], p[1], p[2]];
            start(w, id, ctx, entry, -1);
            w.mobys[id].set_pv_i(IN, 0);
        }
        None => w.mobys[id].set_pv_i(IN, 1),
    }
    st(w, id, 1, 1);
}

/// `0x3eb740(m, path, last)`: onto `path` at its point nearest (L06 `0x382850`, crate::paths::nearest) heading for
/// `last` (−1: its end; `0x32de68`); no path, at point 0.
fn start(w: &mut MobyWorld, id: usize, ctx: &Ctx, p: i32, last: i16) {
    let m = &mut w.mobys[id];
    let Some(points) = path(ctx, p) else {
        m.set_pv_s(CUR, 0);
        return;
    };
    let near = crate::paths::nearest(points, m.pos, 0.0);
    m.set_pv_i(PATH_AT, p + 1);
    m.set_pv_s(LAST, if last == -1 { points.len() as i16 - 1 } else { last });
    m.set_pv_s(CUR, near as i16);
}

/// `0x3eb4c8(m, path)`: along the path toward its last point (module docs); true at it.
fn follow(w: &mut MobyWorld, id: usize, ctx: &Ctx, p: i32) -> bool {
    let Some(points) = path(ctx, p) else { return true };
    let m = &mut w.mobys[id];
    let (cur, last) = (m.pv_s(CUR), m.pv_s(LAST));
    let dir = if cur < last { 1 } else { -1 };
    let (goal, seg) = crate::paths::step(points, m.pos, cur.max(0) as usize, dir, m.pv_f(MV + 0x28) * 10.0);
    m.set_pv_s(CUR, seg as i16);
    let end = points[(last.max(0) as usize).min(points.len() - 1)];
    let f = dist(m.pos, [end[0], end[1], end[2]]);
    fly(m, ctx, goal, f)
}

/// `0x3eb578(m, path)`: along the path toward the target's nearest point on it (module docs).
fn chase(w: &mut MobyWorld, id: usize, ctx: &Ctx, p: i32) {
    let Some(points) = path(ctx, p) else { return };
    let m = &mut w.mobys[id];
    let Some((near, _, _)) = crate::paths::closest(points, 100_000.0, 1.0, 0.0, m.pv_v(T_POINT), false) else { return };
    let goal = if 2.0 < dist(near, m.pos) {
        let (cur, step) = (m.pv_s(CUR).max(0) as usize, m.pv_f(MV + 0x28) * 10.0);
        let (a, sa) = crate::paths::step(points, m.pos, cur, 1, step);
        let (b, sb) = crate::paths::step(points, m.pos, cur, -1, step);
        if dist(near, a) < dist(near, b) {
            m.set_pv_s(CUR, sa as i16);
            a
        } else {
            m.set_pv_s(CUR, sb as i16);
            b
        }
    } else {
        near
    };
    let f = dist(m.pos, near);
    fly(m, ctx, goal, f);
}

/// `0x3eb2d0(m, goal, d)` (module docs): true when its point is the last.
fn fly(m: &mut Moby, ctx: &Ctx, goal: V3, d: f32) -> bool {
    if m.pv_s(CUR) == -1 { return true; }
    let mut v = sub(goal, m.pos);
    let most = m.pv_f(MV + 0x28);
    let l = len(v);
    if most <= l { v = scale(v, most / l); }
    let yaw = atan2(v[0], v[1]);
    let (ta, tb, tm) = (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c));
    let (mut r, mut tv) = (m.rot[2], m.pv_f(TURN_V));
    let mut k = 1.0;
    if m.state < 4 {
        turn_ease(yaw, ta, tb, tm, &mut r, &mut tv, 0);
        let e = (rot_dist(yaw, r) / m.pv_f(MV + 0x30)).clamp(0.0, 1.0);
        let x = 1.0 - e;
        k = 1.0 - (1.0 - x) * (1.0 - x);
    } else {
        let to = atan2(ctx.hero.pos[0] - m.pos[0], ctx.hero.pos[1] - m.pos[1]);
        turn_ease(add_rot(to, f32::from_bits(0x3fc9_0fdb)), ta, tb, tm, &mut r, &mut tv, 0);
    }
    m.rot[2] = r;
    m.set_pv_f(TURN_V, tv);
    let mut speed = len(m.pv_v(VEL));
    ease_speed(d, m.pv_f(MV + 0x20), m.pv_f(MV + 0x24), most, &mut speed);
    let vel = set_len(v, speed * k);
    m.set_pv_v(VEL, vel);
    m.pos = add(m.pos, vel);
    m.pv_s(CUR) == m.pv_s(LAST)
}

/// `0x3eae50` (module docs).
fn hover(m: &mut Moby) {
    m.set_pv_v(BASE, m.pos);
    let base = m.pos[2];
    bob(m, f32::from_bits(0x3e4c_cccd), f32::from_bits(0x3c0e_fa37), base, BOB, BOB_LAST);
    sway(m, f32::from_bits(0x3db2_b8c4), f32::from_bits(0x3bee_4bb0), f32::from_bits(0x3c13_be49), SWAY_A, SWAY_B);
    let r = m.pv_v(BASE_ROT);
    m.rot[0] = add_rot(r[0], m.rot[0]);
    m.rot[1] = add_rot(r[1], m.rot[1]);
}

/// `0x2abb18`: an angle wrapped into [−π, π) by its turns.
fn wrap(x: f32) -> f32 {
    use std::f32::consts::PI;
    let t = (x + PI) * 0.159_154_94;
    (t - t.trunc()) * 6.283_185_5 - PI
}

/// `0x3eb7d0`: in states 3..5 the aims at the target's point (led by Ratchet's move over the bolt's time, × a random
/// share: none here, and jittered by 0): its body's turn from its joint 0 and its gun's tilt from its joint 1 (at most
/// 20°); attacking, the bursts (sequence 1; a bolt from its joint 2 along its first row at [`SHOT_SPEED`], its damage
/// `+0x34`, its range, push 1, kind 1 / 1, size 1; every [`BETWEEN`] ticks, after [`BURST`] + 1 a pause, sequence 0)
/// and the aims' strength over its first [`BLEND_TICKS`] ticks; in states 0..2 the aims swaying (15° at 1 rad a second,
/// 20° at 6/7) at 0.1 of their strength. Then the joint aims (`0x3206a0`).
fn aims(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = w.mobys[id].state;
    let mut blend = 1.0;
    if (3..6).contains(&s) {
        if let Some(j0) = ctx.host.joint(&w.mobys[id], 0) {
            let o = w.mobys[id].pv_v(T_POINT);
            let mut n = (dist(j0, o) - 1.0) / (SHOT_SPEED * f32::from_bits(0x3c88_8889));
            n *= ctx.game.rng.randf(LEAD[0], LEAD[1]);
            let mut t = scale(ctx.hero.disp, n);
            for c in t.iter_mut() { *c += ctx.game.rng.randf(-JITTER, JITTER); }
            t[2] = 0.0;
            let t = add(t, o);
            let d = sub(t, j0);
            let turn = sub_rot(atan2(d[0], d[1]), w.mobys[id].rot[2]);
            w.mobys[id].set_pv_f(AIM_TURN + joint_aim::ASK + 8, turn);
            if let Some(j1) = ctx.host.joint(&w.mobys[id], 1) {
                let d = sub(t, j1);
                const MOST: f32 = f32::from_bits(0x3eb2_b8c4);
                let tilt = sub_rot(-atan2(len2(d), d[2]), w.mobys[id].rot[1]).clamp(-MOST, MOST);
                w.mobys[id].set_pv_f(AIM_TILT + joint_aim::ASK + 4, tilt);
            }
        }
    }
    if s == 4 {
        let m = &w.mobys[id];
        if m.pv_b(WAIT) == 0 && m.pv_s(PAUSE) == 0 {
            if ctx.host.anim(m).is_some_and(|a| a.seq_a != 1 && a.seq_b != 1) { ctx.host.set_anim(m, 1, 0, 0); }
            let r = m.pv_f(RANGE);
            let range = if r < 1.0 { RANGE_DEFAULT } else { r };
            if let (Some(p), Some((rows, _))) = (ctx.host.joint(m, 2), ctx.host.joint_matrix(m, 2)) {
                let vel = set_len(rows[0], SHOT_SPEED * f32::from_bits(0x3c88_8889));
                let damage = m.pv_b(DAMAGE) as f32;
                super::bolt::fire(w, ctx, p, id, damage, range, 1.0, vel, 1, 1, 1.0);
            }
            let m = &mut w.mobys[id];
            let n = m.pv_b(SHOTS).wrapping_add(1);
            m.set_pv_b(SHOTS, n);
            m.set_pv_b(WAIT, BETWEEN);
            if BURST < n {
                m.set_pv_b(SHOTS, 0);
                let near = dist2(m.pos, m.pv_v(T_POINT)) < NEAR;
                m.set_pv_s(PAUSE, if near { PAUSE_NEAR } else { PAUSE_FAR });
                seq_to(w, id, ctx, 0);
            }
        }
        blend = (w.mobys[id].state_ticks as u16 as f32 / BLEND_TICKS).clamp(0.0, 1.0);
    } else if s < 3 {
        seq_to(w, id, ctx, 0);
        blend = f32::from_bits(0x3dcc_cccd);
        let f = ctx.game.frame as f32;
        let m = &mut w.mobys[id];
        m.set_pv_f(AIM_TILT + joint_aim::ASK + 4, wrap(f * f32::from_bits(0x3c88_8889)).sin() * f32::from_bits(0x3e86_0a93));
        m.set_pv_f(AIM_TURN + joint_aim::ASK + 8, wrap(f * f32::from_bits(0x3c6a_0ea1)).sin() * f32::from_bits(0x3eb2_b8c4));
    }
    joint_aim::aim(w, id, AIM_TURN, id, 0, AIM_K * blend, AIM_D);
    joint_aim::aim(w, id, AIM_TILT, id, 1, AIM_K * blend, AIM_D);
}

/// `0x3eaf10`: at each of its joints 5..7 within [`JET_REACH`] of Ratchet across a hit under it (a [`JET_R`] sphere
/// [`JET_DROP`] down, its damage `+0x35`, flags 0x10001, push 2); drawn (else 1 in 2) the exhaust down at 4.5..5.5 a
/// second (on odd frames red `0xcf0000ff` → 0xcf, 18..22 ticks, 180; else blue `0x6000ffff` → 0x80, 20..24, 350; −10)
/// and orange (`0xefff7f4f` → 0xff0000, 10..15 ticks, 20..90 shrinking as much); drawn, the same back along its facing
/// from its joints 3 and 4 (bases 50 / 100, the orange 10..25).
fn jets(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let odd = ctx.game.frame & 1 != 0;
    for j in 5..8 {
        let Some(p) = ctx.host.joint(&w.mobys[id], j) else { continue };
        if dist2(p, ctx.hero.pos) < JET_REACH {
            let at = [p[0], p[1], p[2] - JET_DROP];
            let dmg = w.mobys[id].pv_b(JET_DAMAGE) as f32;
            ai::attack_sphere(w, id, ctx, JET_R, dmg, 2.0, at, 0x1_0001, 0, 1, 0);
        }
        if w.mobys[id].drawn != 0 || ctx.game.rng.randi(2) != 0 {
            let v = [0.0, 0.0, -(ctx.game.rng.randf(4.5, 5.5) * 0.016_666_668)];
            smoke(ctx, p, v, odd, 180, 350, (20, 90));
        }
    }
    if w.mobys[id].drawn != 0 {
        for j in 3..5 {
            let s = ctx.game.rng.randf(4.5, 5.5) * 0.016_666_668;
            let v = set_len(w.mobys[id].rows[0], -s);
            let Some(p) = ctx.host.joint(&w.mobys[id], j) else { continue };
            smoke(ctx, p, v, odd, 50, 100, (10, 25));
        }
    }
}

/// One jet's exhaust (crate::particles::trail, `0x332c10`): red or blue by the frame, then orange.
fn smoke(ctx: &mut Ctx, p: V3, v: V3, odd: bool, red: i16, blue: i16, orange: (i32, i32)) {
    let rng = &mut ctx.game.rng;
    let (c0, c1, life, base) = if odd { (0xcf00_00ff, 0xcf, rng.rand_range(18, 22), red) } else { (0x6000_ffff, 0x80, rng.rand_range(20, 24), blue) };
    if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::trail(sys, rng, p, v, c0, c1, life, base, -10, true); }
    let n = rng.rand_range(orange.0, orange.1);
    let life = rng.rand_range(10, 15);
    if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::trail(sys, rng, p, v, 0xefff_7f4f, 0x00ff_0000, life, n as i16, -n as i16, true); }
}

/// `0x3ec3d0` (`func_L09_003EAF60`, L20 `0x4130f8`; module docs): its class sound 2, its bolts, the blast, deleted.
/// Also L20's bridge 0x11e8 crushing a gunship (crate::moby::classes::level20::misc).
pub fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.host.play_sound(&w.mobys[id], 2, 0);
    bolts::drop(w, id, 0, 3, ctx);
    let pos = w.mobys[id].pos;
    let b = crate::moby::blast::Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true };
    crate::moby::blast::burst(w, ctx, pos, &b);
    w.delete(id);
}
