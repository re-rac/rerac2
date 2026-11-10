//! Level 6's hovering sentry **0x0bc7** (L06 `0x422070`, the decomp's `func_L06_00420AE8`, 4 placed; its set-up and
//! damage `0x4217b8`, senses `0x421608`, flight to a point `0x421ad0`, flight in a frame `0x421cc8`, tail `0x421318`):
//! health `+0x20` (its damage byte `+0x34`), the flash `+0x50`, reaction `+0x60` (a hurt goes to sub 3), target block
//! `+0x190` (its moby `+0x1a0`), the search block `+0x1c0` (crate::moby::target_search; its moby `+0x200`, its
//! result `+0x204`). It moves by the line test (crate::moby::slide, G-CLS-005).
//!
//! Its words: the base point `+0x210` and the goal `+0x220` (its height `+0x228`), the velocity `+0x230`, timers
//! `+0x240` / `+0x248` / `+0x24c` / `+0x250`, its loop sound `+0x254`, what it sees `+0x258` (0 nothing, 1 Ratchet
//! far, 2 within 8 across), the turn speed `+0x25c`, the search polygons `+0x260` (before state 3) / `+0x264`, the
//! cuboid `+0x268` it hides in, its point on its path `+0x26c`, the path it shadows Ratchet along `+0x270` and its
//! way in `+0x274`, the hurt ticks `+0x278`, the bob `+0x280` / `+0x284` and sway `+0x288` / `+0x28c`, the search's
//! reach `+0x290` (back to `+0x298` when `+0x250` runs out), the most it strays across its path `+0x29c`, the cuboid
//! it fights in `+0x2a0` (−1 none), its shot's damage `+0x2a8`, the shots of a volley `+0x2ac`, the aim record
//! `+0x2b0` (its asked pitch `+0x314`), `+0x334` ≠ 0 it holds its place in the fight, its range `+0x338` (× 20),
//! `+0x33c` ≠ 0 it is gone once its mission is done.
//!
//! The L06 engine functions with the port's: `0x35b8c8` set_state, `0x35b5e8` the sequence, `0x3682a0` = L00
//! `0x327e38` (take_damage), `0x368fe8` = `0x328b80` (flight), `0x36bbc0` = `0x32ad38` (find_target), `0x362ca8` /
//! `0x362aa0` (crate::moby::target_search::search, with its polygon / without), `0x35c7b8` = `0x31c618`
//! (crate::moby::slide::slide), `0x35c2a0` = `0x31c078` (crate::moby::ai::turn_to), `0x35cb88` = `0x31c970`
//! (crate::moby::MobyWorld::poke_group), `0x365278` / `0x365330` = `0x324430` / `0x3244e8` (crate::moby::classes::props
//! bob / sway), `0x383d50` = L01 `0x3377c8` (the path's lengths: crate::level::LevelData::walked_path), `0x383e90` =
//! L00 `0x342b50` (crate::paths::advance), `0x41eeb8` = L00 `0x3e1118` (crate::moby::classes::fireball's
//! fireball 0x0b7f), `0x440a40` = L00 `0x3fabe8` (crate::moby::classes::explosion), `0x31b598` = L00 `0x2dbdb0`
//! (crate::moby::shards), `0x35adb8` = L00 `0x31adc0` (crate::moby::bolts::drop), `0x365050` = L00 `0x324208`
//! (crate::moby::joint_aim::aim), `0x33ad98` = L00 `0x335bb8` (crate::particles::puff).
//!
//! **The set-up** (state 0, [`pre`]): with its mission done and `+0x33c` it is gone. Its loop sound none, mode |=
//! 0x800; the reaction (state 7, sub 5, sequences 6 / 6 / 6, key 0.1, gravities 1/120 / 1/240 / 1/240, the speeds
//! 2.5, 0.4, its bytes 4 / 8 / 4 / 8 / 4 / 8 / 1), the health block's `+0x28` 3 and `+0xa8` 3, its shot's damage its
//! damage byte; seeing nothing, its goal its way in's first point, resting (1, sub 0, sequence 1); update and draw
//! distances 0x80; its aim record cleared. **Each tick**: what it sees ([`senses`]); its class's scale; the damage — a
//! hit pokes its group (`+0xbc` 1), under kind 4 its class sound 2, in sub 1 `+0x24c` 300..360, kinds 4 and 5 its
//! death (8); with the reaction's `+0x98 & 3` no collision; it bobs 0.3 (1.33°/tick) and sways 6° (0.3°, 0.35°).
//!
//! **Senses** ([`senses`], every 7th tick at random and on state 0): the search of its polygon (`+0x260`, from
//! state 3 `+0x264`; none: the plain search) within its reach — found: 2 within 8 across, else 1; none: nothing, its
//! point Ratchet's place. Between, the search's point follows Ratchet while he is its moby. Then the target search
//! (its range, 10 high).
//!
//! **Subs**: 0 calm: seeing, before state 3 its way in (2, sub 1), from 3 to 7 the fight ([`fight`]: in its cuboid
//! `+0x268` 6 with a fight cuboid else 5; outside 4 / 3); not seeing, from 4 to 7 the same. 1 alert: seeing near,
//! from 3 to 7 sub 2; nothing, from 3 to 7 the fight's 4 / 3. 2 near: far 1, nothing 0. 3 the hurt's flight (its
//! ticks counted): dead 8; over: sequence 1, still, sub 0.
//!
//! **States**: 2 its way in: flown point to point ([`fly_to`], 12/3600 to 12/60) to the path's end, then with no
//! fight cuboid its first point on the shadow path (3), else 4. 3 shadowing Ratchet along `+0x270` ([`shadow`]): its
//! point walks one at a time toward the point 6 behind the one by the search's point (and its base point there), its
//! goal off the path across as the search's point is (at most `+0x29c`); it heads for the search's point, weaving
//! by `sin(frame·0.02)·45°` past 8 across (12/3600 to 6/60). 4 / 6 the fight ([`fight`]): its goal in the fight cuboid
//! 6 short of its target along the cuboid's x (held inside it; seeing nothing, its way in's last point; within 13 of
//! its target 8 along the cuboid's yaw from it; with `+0x334` its place), its height the cuboid's + 1, flown in the
//! cuboid's frame ([`fly_in`]: 5 / 12 ahead, 10 / 10 back and across, 0.02 of 6 up) facing its target; the aim at
//! its target from joint 4 (0.03 / 0.3); within its range sequence 3 at half speed, from frame 4 to 4.4 a fireball
//! from joint 4 at 1/3 a tick, four of them then sequence 4 (half speed) and again; out of range sequence 0 at full
//! speed. 8 dead: the big explosion toward the camera, its class sound 3, its loop sound stopped, six balls of its
//! pieces 0..5 (`0.133..0.2` a tick, up 2.5×; spin 30, gravity 15), its bolts, gone.
//!
//! **The tail** (`0x421318`): the flash; a poke (`+0xbc`) clears and, `+0x248` under 300, makes it 480..600; `+0x24c`
//! out and the search not empty-handed, `+0x248` down; `+0x240` down; its ticks; `+0x250` out, the reach back; its
//! exhaust: two glow puffs 0.2 under it (0.1, ×1..0.9, 315000, falling 0.05 a tick, `0xdf204080`, 9 ticks) and a white
//! one (0.05, ×1..0.97, 157500, `0x8fffffff`, 23 ticks).
//!
//! Not ported: the debug markers on its paths and points (`0x383a40`, `0x379b90`: particle 0x29, no def the port
//! has); the death's explosion is aimed from its place (the original reads an unset stack word as the point).

use crate::hero::physics::{add, add_rot, atan2, cross, dist, dist2, len, scale, set_len, sub, through, wrap, V3};
use crate::moby::classes::props::{set_seq, wrapped};
use crate::moby::{ai, countdown, Ctx, Moby, MobyWorld};
use crate::gravity::to_local;

pub const CLASS: i16 = 0x0bc7;

const FLASH: usize = 0x50;
const T_MOBY: usize = 0x1a0;
const T_POINT: usize = 0x190;
const SEARCH: usize = 0x1c0;
const S_MOBY: usize = 0x200;
const S_KIND: usize = 0x204;
const BASE: usize = 0x210;
const GOAL: usize = 0x220;
const VEL: usize = 0x230;
const T240: usize = 0x240;
const T248: usize = 0x248;
const T24C: usize = 0x24c;
const T250: usize = 0x250;
const SOUND: usize = 0x254;
const SEES: usize = 0x258;
const TURN_V: usize = 0x25c;
const POLY_A: usize = 0x260;
const POLY_B: usize = 0x264;
const HIDE: usize = 0x268;
const POINT: usize = 0x26c;
const SHADOW: usize = 0x270;
const WAY_IN: usize = 0x274;
const HURT_T: usize = 0x278;
const REACH: usize = 0x290;
const REACH0: usize = 0x298;
const STRAY: usize = 0x29c;
const FIGHT: usize = 0x2a0;
const DAMAGE: usize = 0x2a8;
const SHOTS: usize = 0x2ac;
const AIM: usize = 0x2b0;
const HOLD: usize = 0x334;
const RANGE: usize = 0x338;
const GONE: usize = 0x33c;
const UP: V3 = [0.0, 0.0, 1.0];

fn st(w: &mut MobyWorld, id: usize, state: u8, sub: i32) { w.mobys[id].set_state(state, sub); }

fn p3(p: [f32; 4]) -> V3 { [p[0], p[1], p[2]] }

/// The path with its lengths (`0x383d50`: each point's w the length to the next).
fn path(ctx: &Ctx, i: i32) -> Vec<[f32; 4]> { ctx.level.walked_path(i) }

fn hero_place(ctx: &Ctx) -> V3 { ctx.hero.pos }

/// `0x383c88(p, path, 0)`: the path's point nearest `p`.
fn nearest(pts: &[[f32; 4]], p: V3) -> usize {
    let mut best = (0, 1.0e11f32);
    for (i, q) in pts.iter().enumerate() {
        let d = dist(p3(*q), p);
        if d < best.1 { best = (i, d); }
    }
    best.0
}

/// The frame along the path at point `k` (its direction from the point before, the first's to the next; across;
/// up) as three rows.
fn path_rows(pts: &[[f32; 4]], k: usize) -> [V3; 3] {
    let at = |i: usize| pts.get(i).map_or([0.0; 3], |p| p3(*p));
    let d = if 0 < k { sub(at(k), at(k - 1)) } else { sub(at(k + 1), at(k)) };
    let d = set_len(d, 1.0);
    [d, cross(d, UP), UP]
}

fn live(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool { let h = w.mobys[id].pv_i(SOUND); h != -1 && ctx.host.sound_playing(h) }

/// `0x4217b8` (module docs): true when it is gone.
fn pre(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let m = &w.mobys[id];
        let k = ctx.game.level.max(0) as usize * 0x10 + m.mission as usize;
        if ctx.game.missions.get(k).is_some_and(|b| b & 0x80 != 0) && m.pv_i(GONE) != 0 { return true; }
        let m = &mut w.mobys[id];
        m.set_pv_i(SHOTS, 0);
        m.set_pv_i(SOUND, -1);
        m.mode |= 0x800;
        for (k, v) in [(0x70, 7u8), (0x71, 5), (0x77, 6), (0xd2, 4), (0xd3, 8), (0xd6, 1), (0x28, 3), (0x72, 6), (0x76, 6), (0xcc, 4), (0xcd, 8), (0xce, 4), (0xcf, 8), (0xd4, 0)] { m.set_pv_b(k, v); }
        for (k, v) in [(0x7c, 0x3dcc_ccce), (0x88, 0x3c08_8889), (0x8c, 0x3b88_8889), (0x90, 0x4020_0000), (0x9c, 0x3ecc_cccd), (0x84, 0x3b88_8889)] { m.set_pv_f(k, f32::from_bits(v)); }
        let dmg = m.pv_b(0x34) as f32;
        m.set_pv_f(DAMAGE, dmg);
        m.set_pv_i(0xa8, 3);
        for k in [0x80, 0x94, 0x30, SEES, POINT] { m.set_pv_i(k, 0); }
        m.set_pv_v(VEL, [0.0; 3]);
        m.set_pv_f(VEL + 0xc, 0.0);
        let first = path(ctx, m.pv_i(WAY_IN)).first().map_or([0.0; 3], |p| p3(*p));
        m.set_pv_v(GOAL, first);
        st(w, id, 1, 0);
        set_seq(w, id, ctx, 1, 0, 0);
        let m = &mut w.mobys[id];
        m.draw_dist = 0x80;
        m.update_dist = 0x80;
        if m.pvars.len() < AIM + 0x80 { m.pvars.resize(AIM + 0x80, 0); }
        m.pvars[AIM..AIM + 0x80].fill(0);
    }
    let sees = senses(w, id, ctx);
    w.mobys[id].set_pv_i(SEES, sees);
    let sub = w.mobys[id].sub_state;
    let s = w.class(CLASS).map_or(1.0, |c| c.scale);
    w.mobys[id].scale = s;
    let kind = ai::take_damage(w, id, ctx, false);
    if kind != 0 {
        let g = w.mobys[id].group;
        if g as u8 != 0xff { w.poke_group(g as i32, 1); }
        if kind < 4 { ctx.host.play_sound(&w.mobys[id], 2, 0); }
        if sub == 1 {
            let t = ctx.game.rng.rand_range(300, 360);
            w.mobys[id].set_pv_i(T24C, t);
        }
        if (4..6).contains(&kind) { st(w, id, 8, -1); }
    }
    let m = &mut w.mobys[id];
    if m.pv_s(0x98) as u16 & 3 != 0 { m.has_collision = false; }
    crate::moby::classes::props::bob(m, f32::from_bits(0x3e99_999a), f32::from_bits(0x3cbe_a2f4), -1.0, 0x280, 0x284);
    crate::moby::classes::props::sway(m, f32::from_bits(0x3dd6_7751), f32::from_bits(0x3bab_92a8), f32::from_bits(0x3bc8_2b19), 0x288, 0x28c);
    false
}

/// `0x421608` (module docs): what it sees.
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> i32 {
    let fresh = w.mobys[id].state == 0 || ctx.game.rng.randi(7) == 0;
    if fresh {
        let m = &w.mobys[id];
        let poly = if m.pv_i(POLY_A) == -1 { -1 } else if m.state < 3 { m.pv_i(POLY_A) } else { m.pv_i(POLY_B) };
        let reach = m.pv_f(REACH);
        let kind = crate::moby::target_search::search(w, id, ctx, SEARCH, &[], poly, reach);
        let m = &mut w.mobys[id];
        if kind == 2 {
            m.set_pv_v(SEARCH, hero_place(ctx));
            m.set_pv_i(SEES, 0);
        } else {
            let near = dist2(m.pos, m.pv_v(SEARCH)) < 8.0;
            m.set_pv_i(SEES, if near { 2 } else { 1 });
        }
    } else if ai::link(&w.mobys[id], S_MOBY).is_some() && ai::link(&w.mobys[id], S_MOBY) == ctx.hero_moby {
        w.mobys[id].set_pv_v(SEARCH, hero_place(ctx));
    }
    let range = w.mobys[id].pv_f(RANGE) * 20.0;
    ai::find_target(w, id, ctx, range, 10.0, -1, None, false);
    w.mobys[id].pv_i(SEES)
}

/// The fight's state: in its hiding cuboid 6 (with a fight cuboid) or 5, else 4 or 3 (sub 1).
fn fight_state(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let has = m.pv_i(FIGHT) != -1;
    let s = if ctx.level.in_cuboid(m.pv_i(HIDE), m.pos) { if has { 6 } else { 5 } } else if has { 4 } else { 3 };
    st(w, id, s, 1);
}

/// `0x421ad0(m, accel, speed, yaw, blend)`: toward its goal — its loop sound kept; speeding up by `accel` (eased as
/// `1 − (1 − f)²` of what is left of `speed`) until it would need its stop distance, then slowing by it (true); the
/// velocity blended into the move by `blend`, the line test (0, 1.5, 0), facing `yaw` (crate::moby::ai::turn_to:
/// 1/30°, 0.1°, 2° a tick).
fn fly_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, accel: f32, speed: f32, yaw: f32, blend: f32) -> bool {
    let (accel, speed) = (accel * f32::from_bits(0x3991_a2b4), speed * f32::from_bits(0x3c88_8889));
    let m = &w.mobys[id];
    let d = sub(m.pv_v(GOAL), m.pos);
    let gap = len(d);
    let mut v = len(m.pv_v(VEL));
    let stop = v / accel;
    if !live(w, id, ctx) {
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(SOUND, h);
    }
    let done = gap < v * stop;
    if done {
        v -= accel;
    } else {
        let f = (speed - v) / speed;
        v += accel * (1.0 - (1.0 - f) * (1.0 - f));
    }
    let step = set_len(d, v);
    let m = &mut w.mobys[id];
    let vel = add(scale(m.pv_v(VEL), blend), scale(step, 1.0 - blend));
    m.set_pv_v(VEL, vel);
    let mut vel = vel;
    crate::moby::slide::slide(w, id, ctx, 0.0, 1.5, 0.0, &mut vel, 0);
    let m = &mut w.mobys[id];
    m.set_pv_v(VEL, vel);
    ai::turn_to(m, yaw, f32::from_bits(0x3a18_825c), f32::from_bits(0x3ae4_c38a), f32::from_bits(0x3d0e_fa37), TURN_V);
    done
}

/// One axis of [`fly_in`]: the gap `d`, the speed now `v`; the step's size and whether it slows.
fn axis(d: f32, v: f32, speed: f32, accel: f32) -> (f32, bool) {
    let (gap, mut s) = (d.abs(), v.abs());
    let slow = gap < s * (s / accel);
    if slow {
        s -= accel;
    } else {
        let f = (speed - s) / speed;
        s += accel * (1.0 - (1.0 - f) * (1.0 - f));
    }
    (if d < 0.0 { -s } else { s }, slow)
}

/// `0x421cc8(m, _, rows, inverse, yaw, blend, ahead, ahead_a, back, back_a, side, side_a, up_k, up_most)`: toward
/// its goal in the frame `r` — along x at `ahead` (`ahead_a`) forward and `back` (`back_a`) back, across at `side`
/// (`side_a`), each as [`fly_to`] (speeds a second, accelerations a second²), the velocity blended by `blend`; the
/// height's speed eased by `up_k` toward the gap (at most `up_most` a second); the line test (0, 1.5, 0); facing `yaw`.
/// True when both axes slow.
#[allow(clippy::too_many_arguments)]
fn fly_in(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, r: &[V3; 3], yaw: f32, blend: f32, ahead: [f32; 2], back: [f32; 2], side: [f32; 2], up_k: f32, up_most: f32) -> bool {
    let (sec, sec2) = (f32::from_bits(0x3c88_8889), f32::from_bits(0x3991_a2b4));
    let m = &w.mobys[id];
    let mut d = to_local(sub(m.pv_v(GOAL), m.pos), r);
    let mut v = to_local(m.pv_v(VEL), r);
    let (s, a) = if 0.0 < d[0] { (ahead[0] * sec, ahead[1] * sec2) } else { (back[0] * sec, back[1] * sec2) };
    let (x, sx) = axis(d[0], v[0], s, a);
    d[0] = x * (1.0 - blend);
    v[0] = v[0] * blend + d[0];
    let (y, sy) = axis(d[1], v[1], side[0] * sec, side[1] * sec2);
    d[1] = y * (1.0 - blend);
    v[1] = v[1] * blend + d[1];
    let vel = through(r, v);
    let dw = through(r, d);
    let most = up_most * sec;
    let dz = dw[2].clamp(-most, most);
    let m = &mut w.mobys[id];
    m.set_pv_v(VEL, vel);
    let z = m.pv_f(VEL + 8);
    m.set_pv_f(VEL + 8, z + (dz - z) * up_k);
    let mut vel = m.pv_v(VEL);
    crate::moby::slide::slide(w, id, ctx, 0.0, 1.5, 0.0, &mut vel, 0);
    let m = &mut w.mobys[id];
    m.set_pv_v(VEL, vel);
    ai::turn_to(m, yaw, f32::from_bits(0x3a18_825c), f32::from_bits(0x3ae4_c38a), f32::from_bits(0x3d0e_fa37), TURN_V);
    sx && sy
}

/// State 3 (module docs).
fn shadow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pts = path(ctx, w.mobys[id].pv_i(SHADOW));
    if pts.len() < 2 { return; }
    let n = pts.len();
    let seen = w.mobys[id].pv_v(SEARCH);
    let node = nearest(&pts, seen);
    let rows = path_rows(&pts, node);
    let off = to_local(sub(seen, p3(pts[node])), &rows);
    // The point 6 back along the path from the search's (the original starts it at Ratchet's place 2 up, which the
    // walk then overwrites).
    let mut k = node;
    while 0 < k && dist2(seen, p3(pts[k])) < 6.0 { k -= 1; }
    let back = dist2(seen, p3(pts[k])) - 6.0;
    let (mut seg, mut t) = (k, 0.0);
    let target = crate::paths::advance(&pts, back, &mut seg, &mut t, false).0;
    let pos = w.mobys[id].pos;
    let mut yaw = atan2(seen[0] - pos[0], seen[1] - pos[1]);
    if 8.0 < dist2(pos, seen) {
        let weave = wrap(ctx.game.frame as f32 * f32::from_bits(0x3ca3_d70a)).sin() * f32::from_bits(0x3f49_0fdc);
        yaw = add_rot(yaw, weave);
    }
    let want = nearest(&pts, target) as i32;
    if fly_to(w, id, ctx, 12.0, 6.0, yaw, 0.0) {
        let cur = w.mobys[id].pv_i(POINT);
        let mut desired = if want < cur { want + 1 } else { want - 1 };
        if desired < 0 { desired = 0; }
        let m = &mut w.mobys[id];
        if desired < cur {
            m.set_pv_i(POINT, cur - 1);
        } else if cur < desired {
            m.set_pv_i(POINT, cur + 1);
        }
        let now = m.pv_i(POINT);
        if desired == cur {
            m.set_pv_v(BASE, target);
            m.set_pv_i(POINT, desired);
        } else if let Some(p) = usize::try_from(now).ok().and_then(|i| pts.get(i)) {
            m.set_pv_v(BASE, p3(*p));
        }
        let now = m.pv_i(POINT).clamp(0, n as i32 - 1);
        m.set_pv_i(POINT, now);
    }
    let m = &mut w.mobys[id];
    let k = (m.pv_i(POINT).max(0) as usize).min(n - 1);
    let rows2 = path_rows(&pts, k);
    let most = m.pv_f(STRAY);
    let across = off[1].clamp(-most, most);
    let goal = add(m.pv_v(BASE), scale(rows2[1], across));
    m.set_pv_v(GOAL, goal);
}

/// States 4 and 6 (module docs).
fn fight(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let rec = w.mobys[id].pv_i(FIGHT);
    let Some(c) = usize::try_from(rec).ok().and_then(|i| ctx.level.cuboids.get(i)).cloned() else { return };
    let cpos = [c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]];
    let rows = crate::moby::attach::rows_of([c.extra[0], c.extra[1], c.extra[2]]);
    let tp = w.mobys[id].pv_v(T_POINT);
    let mut l = to_local(sub(tp, cpos), &rows);
    l[0] -= 6.0;
    let (wd, ht) = (len([c.matrix[0][0], c.matrix[0][1], c.matrix[0][2]]), len([c.matrix[1][0], c.matrix[1][1], c.matrix[1][2]]));
    l[0] = l[0].clamp(-wd, wd);
    l[1] = l[1].clamp(-ht, ht);
    l[2] = 0.0;
    let mut goal = add(cpos, through(&rows, l));
    let m = &w.mobys[id];
    if m.pv_i(SEES) == 0 {
        let pts = path(ctx, m.pv_i(WAY_IN));
        if let Some(p) = pts.last() { goal = p3(*p); }
    }
    let near = ai::link(m, T_MOBY).and_then(|k| w.mobys.get(k)).is_some_and(|t| dist2(m.pos, t.pos) < 13.0);
    if near {
        let yaw = c.extra[2];
        goal = add(m.pos, [yaw.cos() * 8.0, yaw.sin() * 8.0, 0.0]);
    }
    if m.pv_i(HOLD) != 0 { goal = m.pos; }
    goal[2] = cpos[2] + 1.0;
    w.mobys[id].set_pv_v(GOAL, goal);
    let pos = w.mobys[id].pos;
    let yaw = atan2(tp[0] - pos[0], tp[1] - pos[1]);
    fly_in(w, id, ctx, &rows, yaw, 0.0, [5.0, 12.0], [10.0, 10.0], [10.0, 10.0], f32::from_bits(0x3ca3_d70a), 6.0);
    let d = dist2(w.mobys[id].pos, tp);
    let muzzle = ctx.host.joint(&w.mobys[id], 4).unwrap_or(w.mobys[id].pos);
    let pitch = atan2(dist2(tp, muzzle), muzzle[2] - tp[2]);
    w.mobys[id].set_pv_f(AIM + 0x64, pitch);
    crate::moby::joint_aim::aim(w, id, AIM, id, 1, f32::from_bits(0x3cf5_c28f), f32::from_bits(0x3e99_999a));
    if d < w.mobys[id].pv_f(RANGE) * 20.0 {
        let a = ctx.host.anim(&w.mobys[id]);
        let (sa, sb) = a.map_or((0xff, 0xff), |a| (a.seq_a, a.seq_b));
        if sb != 4 {
            ctx.host.set_anim_speed(&w.mobys[id], 0.5);
            set_seq(w, id, ctx, 3, 0, 10);
        } else if sa == sb && wrapped(w, id, ctx) {
            ctx.host.set_anim_speed(&w.mobys[id], 0.5);
            ctx.host.set_anim(&w.mobys[id], 3, 0, 10);
        }
        let sa = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
        if sa == 3 {
            let f = ctx.host.frame_time(&w.mobys[id]);
            if (4.0..f32::from_bits(0x408c_cccd)).contains(&f) {
                let v = set_len(sub(tp, muzzle), f32::from_bits(0x3eaa_aaab));
                let dmg = w.mobys[id].pv_f(DAMAGE);
                crate::moby::classes::fireball::shot(w, id, ctx, dmg, v, muzzle);
                let m = &mut w.mobys[id];
                m.set_pv_f(SHOTS, m.pv_f(SHOTS) + 1.0);
            }
            if w.mobys[id].pv_f(SHOTS) == 4.0 {
                w.mobys[id].set_pv_f(SHOTS, 0.0);
                set_seq(w, id, ctx, 4, 0, 10);
                ctx.host.set_anim_speed(&w.mobys[id], 0.5);
            }
        }
    } else {
        ctx.host.set_anim_speed(&w.mobys[id], 1.0);
        set_seq(w, id, ctx, 0, 0, 10);
    }
}

/// State 8 (module docs).
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (pos, rot, class) = { let m = &w.mobys[id]; (m.pos, m.rot, m.o_class) };
    let dir = set_len(sub(ctx.host.camera_pos(), pos), 1.0);
    crate::moby::classes::explosion::start(w, ctx, 5.0, pos, dir);
    ctx.host.play_sound(&w.mobys[id], 3, 0);
    if live(w, id, ctx) { ctx.host.stop_sound(w.mobys[id].pv_i(SOUND)); }
    w.mobys[id].set_pv_i(SOUND, -1);
    for i in 0..6u8 {
        let mut v = ctx.game.rng.rand_vec(f32::from_bits(0x3e08_8889), f32::from_bits(0x3e4c_ccce));
        v[2] = v[2].abs() * 2.5;
        crate::moby::shards::spawn_ball(w, ctx, class, Some(id), pos, rot, 30.0, 15.0, i, v, 0xff);
    }
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
    w.delete(id);
}

/// `0x421318` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    ai::flash(m, FLASH);
    if m.timer != 0 {
        m.timer = 0;
        if m.pv_i(T248) < 300 {
            let t = ctx.game.rng.rand_range(480, 600);
            w.mobys[id].set_pv_i(T248, t);
        }
    }
    let m = &mut w.mobys[id];
    let down = |m: &mut Moby, k: usize| { let mut t = m.pv_i(k); let d = countdown(&mut t); m.set_pv_i(k, t); d };
    if down(m, T24C) && m.pv_i(S_KIND) != 2 { down(m, T248); }
    down(m, T240);
    if m.state_ticks != -1 { m.state_ticks += 1; }
    if down(m, T250) { let r = m.pv_f(REACH0); m.set_pv_f(REACH, r); }
    let mut at = m.pos;
    at[2] -= 0.2;
    let v = [0.0, 0.0, -0.05];
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let mut spin = g.randi(16);
    if g.randi(2) != 0 { spin = -spin; }
    let first = crate::particles::puff(sys, g, 0.1, 1.0, 0.9, 315_000.0, at, spin, v, 0xdf20_4080, 0);
    fn glow(r: &mut [u8], life: u8) {
        r[0x24..0x28].copy_from_slice(&2u32.to_le_bytes());
        r[0x2a] = 0xdf;
        r[0x2b] = life;
    }
    if let Some(i) = first {
        let r = &mut sys.pool.recs[i];
        r[0xa..0xc].copy_from_slice(&9i16.to_le_bytes());
        r[9] = 4 + 0x70;
        glow(&mut r[..], 9);
    }
    // The second's header is the first's again in the original (its own keeps the puff's).
    if let Some(i) = crate::particles::puff(sys, g, 0.1, 1.0, 0.9, 315_000.0, at, spin, v, 0xdf20_4080, 0) {
        let life = first.map_or(sys.pool.recs[i][0xa], |f| sys.pool.recs[f][0xa]);
        glow(&mut sys.pool.recs[i][..], life);
    }
    if let Some(i) = crate::particles::puff(sys, g, 0.05, 1.0, f32::from_bits(0x3f78_51ec), 157_500.0, at, 16, v, 0x8fff_ffff, 0) {
        let roll = g.randi(255) as u8;
        let life = first.map_or(23, |f| sys.pool.recs[f][0xa]);
        let r = &mut sys.pool.recs[i];
        r[9] = 4 + 0x70;
        r[0xa..0xc].copy_from_slice(&23i16.to_le_bytes());
        r[8] = roll;
        r[0x24..0x28].copy_from_slice(&2u32.to_le_bytes());
        r[0x2a] = 0x8f;
        r[0x2b] = life;
    }
}

/// 0x0bc7 (module docs).
pub fn update_0bc7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if pre(w, id, ctx) {
        w.delete(id);
        return;
    }
    let sees = w.mobys[id].pv_i(SEES);
    let s = w.mobys[id].state;
    let has = w.mobys[id].pv_i(FIGHT) != -1;
    match w.mobys[id].sub_state {
        0 => {
            if 0 < sees {
                if s < 3 { st(w, id, 2, 1); } else if s < 8 { fight_state(w, id, ctx); }
            } else if (4..8).contains(&s) {
                fight_state(w, id, ctx);
            }
            w.mobys[id].set_pv_i(HURT_T, 0);
        }
        1 => {
            w.mobys[id].set_pv_i(HURT_T, 0);
            if sees == 2 {
                if (3..8).contains(&s) { w.mobys[id].sub_state = 2; }
            } else if sees != 1 && (3..8).contains(&s) {
                st(w, id, if has { 4 } else { 3 }, 1);
            }
        }
        2 => {
            w.mobys[id].set_pv_i(HURT_T, 0);
            if sees == 1 { w.mobys[id].sub_state = 1; } else if sees == 0 { w.mobys[id].sub_state = 0; }
        }
        3 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(HURT_T, m.pv_i(HURT_T) + 1);
            let f = ai::flight(w, id, ctx);
            if f & 1 != 0 {
                st(w, id, 8, -1);
            } else if f & 2 != 0 {
                set_seq(w, id, ctx, 1, 0, 10);
                let m = &mut w.mobys[id];
                m.set_pv_i(HURT_T, 0);
                m.set_pv_v(VEL, [0.0; 3]);
                m.set_pv_f(VEL + 0xc, 0.0);
                m.sub_state = 0;
            }
        }
        _ => {}
    }
    match w.mobys[id].state {
        2 => {
            let (g, p) = (w.mobys[id].pv_v(GOAL), w.mobys[id].pos);
            let yaw = atan2(g[0] - p[0], g[1] - p[1]);
            if fly_to(w, id, ctx, 12.0, 12.0, yaw, 0.0) {
                let pts = path(ctx, w.mobys[id].pv_i(WAY_IN));
                let m = &mut w.mobys[id];
                let k = m.pv_i(POINT) + 1;
                m.set_pv_i(POINT, k);
                if pts.len() as i32 <= k {
                    if m.pv_i(FIGHT) == -1 {
                        m.set_pv_i(POINT, 0);
                        let first = path(ctx, m.pv_i(SHADOW)).first().map_or([0.0; 3], |p| p3(*p));
                        m.set_pv_v(GOAL, first);
                        m.set_pv_v(BASE, first);
                        st(w, id, 3, -1);
                    } else {
                        st(w, id, 4, -1);
                    }
                } else if let Some(p) = pts.get(k as usize) {
                    m.set_pv_v(GOAL, p3(*p));
                }
            }
        }
        3 => shadow(w, id, ctx),
        4 | 6 => fight(w, id, ctx),
        8 => {
            die(w, id, ctx);
            return;
        }
        _ => {}
    }
    tail(w, id, ctx);
}

