//! Joba's arena fighters (L11 addresses; crate::moby::classes::level11::arena's master 0x0d8a sends them): pooled
//! above the arena (state 1, hidden) in the master's group `+0xc0` until it puts one in a spawner (state 2) or, in a
//! wave of the ring, places it round the middle; out of the spawner ([`start`], the spawner's release `0x418ab0`)
//! each walks in to the middle's polygon (state 3, after its delay) and fights. Maktar's (crate::moby::classes::
//! fighters) are the model: Joba's are their level's own copies, each with a mode for the ring (its word
//! "placed", set by the start's third argument): its moves are the gravity frames' (crate::moby::frames, `0x3415a0` /
//! `0x3415e8`; the cylinder mode `0x1a8ff4`: crate::gravity, read here as [`super::Globals::arena_gravity`]), its yaws in its
//! own frame (`0x338e18`, [`ring_yaw`]), it keeps within 15 of the middle ([`ring_keep`]) and in the master's
//! cylinder `+0x1c4` (`0x447be8`, else its health 0 and the spin out); until the cylinder mode is on it waits. Out of
//! the ring it is spun away from the middle ([`spin_out`]).
//!
//! The master's helpers: its middle (`0x443038`), its polygon `+0x164` (`0x443058`, [`in_middle`]), the turns to
//! attack per class (`+0xe0..+0xf4`: `0x4430b8` [`claim`], `0x4431c8` [`give_back`]), the bolts (`0x443288`,
//! [`bolts`]).
//!
//! This file: **0x0d37** ([`update_d37`]), **0x0dcf** ([`update_dcf`]), **0x0df4** ([`update_df4`]) and **0x0df7**
//! ([`update_df7`]); crate::moby::classes::level11::brutes has 0x0d98 and 0x0df9, crate::moby::classes::level11::
//! bosses the bosses.
//!
//! Not ported (each start's): the warp-in effect of a placed fighter (`0x398770`: its class 0x12b2, which draws a
//! flash and shows its owner; here the placed fighter is shown at once), the pieces' break by health (`0x33c680`),
//! the animation advance after a start's or pool's sequence (`0x325748`: the moby loop does it the same tick).

use crate::hero::physics::{add, add_rot, atan2, dist, dist2, rot_dist, scale, set_len, sub, sub_rot, V3};
use crate::moby::{ai, frames, mode, Ctx, MobyWorld};
use super::super::props::{seq, set_seq, wrapped};
use super::arena::{self, MASTER, D37, DCF, DF4, DF7};

// ---------------------------------------------------------------------------------------------------------------
// The master's helpers

/// Its master (`+0xb8`).
pub fn master(w: &MobyWorld, id: usize) -> Option<usize> { w.mobys[id].parent.filter(|&m| w.mobys.get(m).is_some_and(|x| x.o_class == MASTER)) }

/// `0x443038(master)`: the arena's middle (its own place without a master).
pub fn middle(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    master(w, id).and_then(|ms| arena::middle(w, ms, ctx)).unwrap_or(w.mobys[id].pos)
}

/// `0x443058(master, p)`: `p` inside the master's polygon (the path `+0x164`).
pub fn in_middle(w: &MobyWorld, id: usize, ctx: &Ctx, p: V3) -> bool {
    let Some(ms) = master(w, id) else { return false };
    usize::try_from(w.mobys[ms].pv_i(0x164)).ok().and_then(|i| ctx.level.paths.get(i)).is_some_and(|pts| arena::in_polygon(pts, p))
}

/// `0x447be8(master, p)`: `p` inside the master's cylinder `+0x1c4`.
pub fn in_arena(w: &MobyWorld, id: usize, ctx: &Ctx, p: V3) -> bool {
    master(w, id).is_some_and(|ms| ctx.level.in_cylinder(w.mobys[ms].pv_i(0x1c4), p))
}

/// `0x4430b8(master, class)`: a turn to attack from the master's count for the moby's class.
pub fn claim(w: &mut MobyWorld, id: usize) -> bool {
    let c = w.mobys[id].o_class;
    let (Some(ms), Some(k)) = (master(w, id), arena::FIGHTERS.iter().position(|&x| x == c)) else { return false };
    let m = &mut w.mobys[ms];
    let n = m.pv_i(arena::pv::LIMITS + 4 * k);
    if n <= 0 { return false; }
    m.set_pv_i(arena::pv::LIMITS + 4 * k, n - 1);
    true
}

/// `0x4431c8(master, class)`: the turn given back.
pub fn give_back(w: &mut MobyWorld, id: usize) {
    let c = w.mobys[id].o_class;
    let (Some(ms), Some(k)) = (master(w, id), arena::FIGHTERS.iter().position(|&x| x == c)) else { return };
    let m = &mut w.mobys[ms];
    m.set_pv_i(arena::pv::LIMITS + 4 * k, m.pv_i(arena::pv::LIMITS + 4 * k) + 1);
}

/// `0x443288(m, 0, −1)`: its bolts (Maktar's `0x3fd7e8`).
pub fn bolts(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { crate::moby::bolts::drop(w, id, 0, 3, ctx); }

/// `0x1a8ff4`: the gravity's cylinder mode (the master's state 9).
pub fn grav(w: &MobyWorld) -> bool { w.level11.arena_gravity[1] }

// ---------------------------------------------------------------------------------------------------------------
// The fighters' shared pieces

pub(super) fn lim(w: &mut MobyWorld, id: usize, a: f32, b: f32, c: f32, d: f32) { ai::senses(&mut w.mobys[id], a, b, c, d); }

/// `0x338e18(m, v, frame)`: the yaw of `v` in its own frame (signed).
pub(super) fn ring_yaw(w: &MobyWorld, id: usize, v: V3) -> f32 {
    let l = crate::gravity::to_local(v, &w.mobys[id].rows);
    atan2(l[0], l[1])
}

/// The target's place, its yaw from the moby and the difference (in the ring: none, and the yaw in its own frame),
/// the distance (`0x2ef018`).
pub(super) fn aim(w: &MobyWorld, id: usize, t: V3, ring: bool) -> (f32, f32, f32) {
    let m = &w.mobys[id];
    let d = dist(m.pos, t);
    if ring { return (0.0, ring_yaw(w, id, sub(t, m.pos)), d); }
    let a = atan2(t[0] - m.pos[0], t[1] - m.pos[1]);
    (a, rot_dist(a, m.rot[2]), d)
}

pub(super) fn target_of(w: &MobyWorld, id: usize, ctx: &Ctx, link: usize) -> V3 {
    ai::link(&w.mobys[id], link).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos)
}

/// The shadow: off the ring and the cylinder mode (and not spinning out) mode 0x400 and, drawn within 24 of the
/// camera, the probe (range 0x16); else mode 0x400 off.
pub(super) fn shadow(w: &mut MobyWorld, id: usize, ctx: &Ctx, on: bool) {
    if on && !grav(w) {
        w.mobys[id].mode |= 0x400;
        crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    } else {
        w.mobys[id].mode &= !0x400;
    }
}

/// The frame's hit with the floor's plates 0x0a03 and flame columns 0x0b89 doing nothing to it while it is not drawn
/// (0x0df7: the plates' always).
pub(super) fn floor_hits(w: &mut MobyWorld, id: usize, plates_always: bool) {
    let Ok(slot) = usize::try_from(w.mobys[id].hit_slot) else { return };
    let drawn = w.mobys[id].drawn != 0;
    let Some(r) = w.hits.records.get(slot).copied().filter(|r| r.target == id) else { return };
    let Some(c) = r.tmpl.source.and_then(|s| w.mobys.get(s)).map(|s| s.o_class) else { return };
    let zero = (c == arena::PLATE_CLASS && (plates_always || !drawn)) || (c == arena::COLUMN_CLASS && !drawn);
    if zero { w.hits.records[slot].tmpl.damage = 0.0; }
}

/// The ring's checks of the senses: out of the master's cylinder its health 0 and `(state, sub)`; unless spinning out
/// (`out`), kept within 15 of the middle across (its height kept).
pub(super) fn ring_keep(w: &mut MobyWorld, id: usize, ctx: &Ctx, health: usize, out: (u8, i32)) {
    let p = w.mobys[id].pos;
    if !in_arena(w, id, ctx, p) {
        w.mobys[id].set_pv_f(health, 0.0);
        w.mobys[id].set_state(out.0, out.1);
    }
    if w.mobys[id].state != out.0 {
        let mid = middle(w, id, ctx);
        let m = &mut w.mobys[id];
        let v = [m.pos[0] - mid[0], m.pos[1] - mid[1], 0.0];
        if 225.0 < v[0] * v[0] + v[1] * v[1] {
            let k = set_len(v, 15.0);
            m.pos = [mid[0] + k[0], mid[1] + k[1], m.pos[2]];
        }
    }
}

/// The colour flash's leftover at a start (`+0x50` set: cleared, its colour `+0x54..` back, `+0x52` 1).
pub(super) fn flash_reset(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    if m.pv_s(0x50) != 0 {
        m.set_pv_s(0x50, 0);
        m.ambient = [m.pv_b(0x54), m.pv_b(0x55), m.pv_b(0x56)];
        m.set_pv_s(0x52, 1);
    }
}

/// A start's common part: its life full, the senses, shown and a target with its collision; placed in the ring
/// (`ring`): `(state, sub)` with its rows kept, the warp-in's stand-in (shown at once); else state 3 / `sub`, shown.
pub(super) fn start_body(w: &mut MobyWorld, id: usize, ring: bool, ring_state: (u8, i32), sub: i32) {
    let coll = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    let life = m.pv_s(0x24) as f32;
    m.set_pv_f(0x20, life);
    m.drops = 0;
    m.mode = (m.mode | mode::TARGET) & 0xffb9;
    m.has_collision = coll;
    if ring {
        m.set_state(ring_state.0, ring_state.1);
        m.mode |= mode::KEEP_ROWS;
        m.mode &= !1;
        m.drawn = 1;
    } else {
        m.set_state(3, sub);
        m.drawn = 1;
        m.mode &= 0xfefe;
    }
    w.changed = true;
}

/// A pool's common part: its sequence 0 (blend 3), state 1, hidden without collision.
pub(super) fn pool_body(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.host.set_anim(&w.mobys[id], 0, 0, 3);
    let m = &mut w.mobys[id];
    m.set_state(1, 0);
    m.drawn = 0;
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 0x47;
    w.changed = true;
}

/// `0x42e300(pos, shake, a, b, c, puffs, streaks, light, s, …)`: the blast (crate::moby::blast).
pub(super) fn blast(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, size: f32, bursts: [i32; 3], puffs: i32, streaks: i32, light: bool) {
    let pos = w.mobys[id].pos;
    crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size, bursts, puffs, streaks, light, shake: false });
}

/// A spot round `p` (`0x4383b8` and its copies): in the ring 15 out at its way from `p` ±45° (±10° for 0x0d98), at
/// a height of 466..470 (0x0d98: its own ±4); else 6..10 out, `p`'s height.
pub(super) fn spot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3, out: usize, ring: bool, cone: f32, height: Option<f32>) {
    let m = &w.mobys[id];
    let a = atan2(m.pos[0] - p[0], m.pos[1] - p[1]) + ctx.game.rng.randf(-cone, cone);
    let z0 = m.pos[2];
    if ring {
        let z = match height { Some(h) => z0 + ctx.game.rng.randf(-h, h), None => ctx.game.rng.randf(0.0, 4.0) + 466.0 };
        w.mobys[id].set_pv_v(out, [p[0] + 15.0 * a.cos(), p[1] + 15.0 * a.sin(), z]);
    } else {
        let r = ctx.game.rng.randf(-2.0, 2.0) + 8.0;
        w.mobys[id].set_pv_v(out, [p[0] + r * a.cos(), p[1] + r * a.sin(), p[2]]);
    }
}

/// The move toward `p`: in the ring the frames' (`0x3415a0`), else the walk (`0x33f6c0`).
pub(super) fn go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3, ring: bool) {
    if ring {
        frames::move_to(w, id, ctx, p);
    } else {
        let q = w.mobys[id].pos;
        ai::move_step(w, id, ctx, atan2(p[0] - q[0], p[1] - q[1]), p);
    }
}

/// The turn in place toward `p` (`0x3417d0` in the ring, else `0x33fa78(m, yaw)`).
pub(super) fn face(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3, ring: bool) {
    if ring {
        frames::face_point(w, id, p);
    } else {
        let q = w.mobys[id].pos;
        ai::turn_in_place(w, id, ctx, atan2(p[0] - q[0], p[1] - q[1]));
    }
}

/// `0x438548` (and its copies): in view (`0x3024c8(128, (pos, 2))`), a raised plate of its group `group` within
/// `near` across and 85° of its facing is gone round 5° a tick (the walk's stop `+0x134` 0.0001 for the move).
pub(super) fn round_plates(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, group: usize, near: f32) -> bool {
    let p = w.mobys[id].pos;
    if ctx.view.is_some_and(|v| v.culled(128.0, [p[0], p[1], p[2], 2.0])) { return false; }
    let g = w.mobys[id].pv_i(group);
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { return false };
    for q in list {
        if w.mobys.get(q).is_none_or(|x| x.o_class != arena::PLATE_CLASS) || !super::super::arena::raised(w, q) { continue; }
        let (me, at) = (w.mobys[id].pos, w.mobys[q].pos);
        if near <= dist2(me, at) { continue; }
        let yaw = atan2(at[0] - me[0], at[1] - me[1]);
        let face = w.mobys[id].rot[2];
        if 85f32.to_radians() <= rot_dist(face, yaw) { continue; }
        let mut a = 5f32.to_radians();
        if 0.0 <= sub_rot(face, yaw) { a = -a; }
        let goal = add(crate::camera::rot(sub(me, at), a, w.mobys[id].rows[2]), at);
        let keep = w.mobys[id].pv_f(0x134);
        w.mobys[id].set_pv_f(0x134, 0.0001);
        go(w, id, ctx, goal, false);
        w.mobys[id].set_pv_f(0x134, keep);
        return true;
    }
    false
}

/// The spin out of the ring (0x0d37's state 11, 0x0dcf's 11, 0x0df4's 12, 0x0df7's 14, 0x0d98's 18): the first tick
/// its way out from the middle (across, `randf(3, 5)` a second), the ring left, a spin of `randf(240°, 720°)` a second
/// on each axis, its angles from its rows, its sequence `seq` (blend 30), no longer a target; each tick falling by its
/// gravity (`gravity`), moved by that way (`0x33fa98`), the spin slowed (0.98) and turned about `pivot` (in its
/// frame). True once it is under the middle's height + 5, one tick in 450, or after 300 ticks: its bolts and its pool
/// are the caller's.
#[allow(clippy::too_many_arguments)]
pub(super) fn spin_out(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, vel: usize, spin: usize, ring: usize, seq_out: Option<u8>, gravity: usize, pivot: Pivot) -> bool {
    let mid = middle(w, id, ctx);
    if w.mobys[id].first() {
        let p = w.mobys[id].pos;
        let s = ctx.game.rng.randf(3.0, 5.0) / 60.0;
        let v = set_len([mid[0] - p[0], mid[1] - p[1], 0.0], s);
        let sp = [0; 3].map(|_| ctx.game.rng.randf(240.0, 720.0) * 0.000_290_888_25);
        let m = &mut w.mobys[id];
        m.set_pv_v(vel, v);
        m.set_pv_s(ring, 0);
        m.set_pv_v(spin, sp);
        m.rot = crate::moby::attach::angles_of(&m.rows);
        if let Some(s) = seq_out { set_seq(w, id, ctx, s, 0, 30); }
        w.mobys[id].mode &= !mode::TARGET;
    }
    let mut v = w.mobys[id].pv_v(vel);
    v[2] -= w.mobys[id].pv_f(gravity);
    w.mobys[id].set_pv_v(vel, v);
    ai::move_by(w, id, ctx, v);
    let piv = match pivot {
        Pivot::Up(o) => [0.0, 0.0, w.mobys[id].pv_f(o)],
        Pivot::Joint(j) => {
            let m = &w.mobys[id];
            ctx.host.joint(m, j).map_or([0.0; 3], |p| crate::gravity::to_local(sub(p, m.pos), &m.rows))
        }
    };
    let m = &mut w.mobys[id];
    let sp = scale(m.pv_v(spin), 0.98);
    m.set_pv_v(spin, sp);
    crate::moby::attach::pivot_turn(m, sp, piv);
    m.rows = crate::moby::attach::rows_of(m.rot);
    m.pos[2] < mid[2] + 5.0 || ctx.game.rng.randi(450) == 0 || 300 < m.state_ticks as u16
}

/// The point a spin out turns about: up its rows by a pvar float, or a joint's place in its frame (`0x329920`'s
/// bone place, scaled).
#[derive(Clone, Copy)]
pub(super) enum Pivot { Up(usize), Joint(usize) }

/// The class's turn word, in the arena master's deal (`0x442ba0`).
fn has_turn(w: &MobyWorld, id: usize, turn: usize) -> bool { w.mobys[id].pv_s(turn) == 1 }

/// The turns (`0x438090` and its copies): before state 4 none (no target); kept once had, else asked of the master;
/// with one the target (`0x32ad38(100, 100, m, area)`) its block's link `link`, the result at `found`.
pub(super) fn turn(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, turn: usize, target: usize, area: usize, found: usize, link: usize, need_drawn: bool) {
    if w.mobys[id].state < 4 {
        w.mobys[id].set_pv_i(target, 0);
        w.mobys[id].set_pv_s(turn, 0);
        return;
    }
    if !has_turn(w, id, turn) {
        if (need_drawn && w.mobys[id].drawn == 0) || !claim(w, id) {
            w.mobys[id].set_pv_s(turn, 0);
            return;
        }
        w.mobys[id].set_pv_s(turn, 1);
    }
    let a = w.mobys[id].pv_i(area);
    let f = ai::find_target(w, id, ctx, 100.0, 100.0, a, None, false);
    let m = &mut w.mobys[id];
    m.set_pv_s(found, f as i16);
    let t = m.pv_i(link);
    m.set_pv_i(target, t);
}

/// A tail's blow on what it touches (`0x32f698(m, pos, 0x4000000, r, 1, 1, 0, 1, 0)`).
pub(super) fn bump(w: &mut MobyWorld, id: usize, ctx: &Ctx, r: f32) {
    let pos = w.mobys[id].pos;
    ai::attack_sphere(w, id, ctx, r, 1.0, 1.0, pos, 0x400_0000, 0, 1, 0);
}

pub(super) fn tick(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) < 0xffff { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

/// The class's start (the master's `0x4421c0` and the spawner's release): 0x0d37 `0x438150`, 0x0dcf `0x44dd60`,
/// 0x0df4 `0x4546b0`, 0x0df7 `0x4566d0`, 0x0d98 `0x44b0c8`, 0x0df9 `0x457e90`.
pub fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, delay: i32, placed: bool) {
    match w.mobys[id].o_class {
        D37 => start_d37(w, id, ctx, delay, placed),
        DCF => start_dcf(w, id, ctx, delay, placed),
        DF4 => start_df4(w, id, ctx, delay, placed),
        DF7 => start_df7(w, id, ctx, delay, placed),
        arena::D98 => super::brutes::start_d98(w, id, ctx, delay, placed),
        arena::DF9 => super::brutes::start_df9(w, id, ctx, delay),
        _ => {}
    }
}

/// The class's pool (the master's `0x442a18`).
pub fn pool(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].o_class {
        D37 => pool_d37(w, id, ctx),
        DCF => pool_dcf(w, id, ctx),
        DF4 => pool_df4(w, id, ctx),
        DF7 => pool_df7(w, id, ctx),
        arena::D98 => super::brutes::pool_d98(w, id, ctx),
        arena::DF9 => super::brutes::pool_df9(w, id, ctx),
        arena::D4A => super::bosses::pool_d4a(w, id, ctx),
        arena::D4B => super::bosses::pool_d4b(w, id, ctx),
        arena::B3A => super::bosses::pool_b3a(w, id, ctx),
        _ => {}
    }
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0d37

/// Its words: the movement block `+0x100`, its home spot `+0x250`, its target `+0x270` (from the target block's
/// `+0x200`), its yaw offset `+0x274`, plates' group `+0x278`, area `+0x27c`, found `+0x280`, turn `+0x282`, delay
/// `+0x284`, placed `+0x286`, the spin out's way `+0x1a0` and spin `+0x260`, the head record `+0x290`.
mod d37 {
    pub const HOME: usize = 0x250;
    pub const SPIN: usize = 0x260;
    pub const TARGET: usize = 0x270;
    pub const OFFSET: usize = 0x274;
    pub const PLATES: usize = 0x278;
    pub const AREA: usize = 0x27c;
    pub const FOUND: usize = 0x280;
    pub const TURN: usize = 0x282;
    pub const DELAY: usize = 0x284;
    pub const RING: usize = 0x286;
}

/// `0x438150(m, delay, placed)`.
fn start_d37(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, delay: i32, placed: bool) {
    let m = &mut w.mobys[id];
    m.set_pv_s(d37::TURN, 0);
    m.set_pv_s(0x88, 0);
    m.set_pv_s(d37::RING, placed as i16);
    lim(w, id, 0.004_444_444_6, 0.133_333_34, 0.006_981_318, 0.209_439_55);
    start_body(w, id, placed, (7, 3), 0);
    if placed { ctx.host.set_anim(&w.mobys[id], 1, 0, 0); }
    let m = &mut w.mobys[id];
    m.set_pv_b(0x2e, 1);
    m.set_pv_s(d37::DELAY, delay as i16);
    m.set_pv_b(0x6a, 0);
    flash_reset(w, id);
}

/// `0x4382b8`: back to the pool — its turn given back; in view a blast (0.3) and 0x0df7's sound 0.
fn pool_d37(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if has_turn(w, id, d37::TURN) { give_back(w, id); }
    w.mobys[id].set_pv_i(d37::TARGET, 0);
    w.mobys[id].set_pv_s(d37::TURN, 0);
    if w.mobys[id].drawn != 0 {
        blast(w, id, ctx, 0.3, [0, 0x1e, 5], 0, 0x32, true);
        ctx.host.play_sound_of(&w.mobys[id], DF7, 0, 0);
    }
    pool_body(w, id, ctx);
}

/// `0x437b90`: the set-up, the hits (in the ring their push turned away from the middle), its damage, its turn, the
/// ring's checks, its shadow.
fn senses_d37(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let a = ctx.game.rng.randf(15.0, 40.0).to_radians();
        let neg = ctx.game.rng.randi(2) != 0;
        let mirror = ctx.game.rng.randi(2) == 1;
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 4);
        m.set_pv_b(0x29, 1);
        m.set_pv_i(0x18, 0x100);
        m.set_pv_f(d37::OFFSET, if neg { -a } else { a });
        if mirror { m.mode |= mode::MIRROR; }
        m.set_pv_s(d37::TURN, 0);
        m.set_pv_b(0x60, 10);
        m.set_pv_i(0x98, 4);
        m.set_pv_f(0x84, 0.65);
        m.set_pv_f(0x80, 0.6);
        m.set_pv_f(0x100, 0.6);
        m.set_pv_f(0x104, 0.05);
        m.set_pv_f(0x8c, 0.4);
        m.set_pv_f(0x108, 0.4);
        m.set_pv_f(0x10c, 0.4);
        m.set_pv_f(0x134, 0.01);
        m.set_pv_f(0x110, std::f32::consts::FRAC_PI_2);
        m.set_pv_f(0x130, 0.523_598_85);
        m.set_pv_f(0x12c, 1.0 / 360.0);
        m.set_pv_b(0x28, 3);
        pool_d37(w, id, ctx);
    }
    let ring = w.mobys[id].pv_s(d37::RING) != 0;
    if 2 <= w.mobys[id].sub_state {
        floor_hits(w, id, false);
        if ring { push_out(w, id, ctx); }
        if ai::take_damage(w, id, ctx, false) == 5 {
            w.mobys[id].set_pv_i(0xe0, 0);
            w.mobys[id].set_pv_f(0xe4, 0.837_758_2);
        }
        turn(w, id, ctx, d37::TURN, d37::TARGET, d37::AREA, d37::FOUND, 0x200, false);
        if ring { ring_keep(w, id, ctx, 0x20, (11, 4)); }
    }
    let on = !ring && w.mobys[id].state != 11;
    shadow(w, id, ctx, on);
    if 2 <= w.mobys[id].state && w.mobys[id].pv_b(0x2e) == 2 { pool_d37(w, id, ctx); }
}

/// In the ring a hit's push is turned off the way to the middle (its share toward the middle across taken out, its
/// length kept).
fn push_out(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let Ok(slot) = usize::try_from(w.mobys[id].hit_slot) else { return };
    let Some(r) = w.hits.records.get(slot).copied().filter(|r| r.target == id && r.tmpl.flags & 0x1_0000 != 0) else { return };
    let mid = middle(w, id, ctx);
    let p = w.mobys[id].pos;
    let n = set_len([mid[0] - p[0], mid[1] - p[1], 0.0], 1.0);
    let v = [r.tmpl.dir[0], r.tmpl.dir[1], r.tmpl.dir[2]];
    let l = crate::hero::physics::len(v);
    let k = crate::hero::physics::dot(v, n);
    let d = set_len(sub(v, scale(n, k)), l);
    let t = &mut w.hits.records[slot].tmpl;
    t.dir = [d[0], d[1], d[2], t.dir[3]];
}

/// 0x0d37 (`0x436e40`): a small walker on the framework (react `+0x50`, movement `+0x100`) that butts. Out, it waits
/// its delay and walks in to the middle's polygon (3), then wanders between spots 6..10 from the middle (sub 2,
/// idling now and then); with a turn (the master's) it closes on its target (7: running from 14 out, walking within
/// 9; round raised plates at its yaw offset), within 3 and facing it within 1° it rears up (9) and lunges (8: its
/// joint 0 hits for `+0x34` until frame 14, then it backs off 4), the taunt (6) when it hurt Ratchet. Knocked (sub
/// 4), dead (or one in 30 in its death sequence 8): in the ring now and then the spin out, else its bolts and back
/// to the pool. It hits what it touches for 1 (1.0). In the ring it rises out of the floor first (4, until nothing
/// but a 0x0b3a is above it).
pub fn update_d37(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use d37::*;
    if w.mobys[id].state == 1 { return; }
    let ring = w.mobys[id].pv_s(RING) != 0;
    if ring && !grav(w) && !matches!(w.mobys[id].state, 10 | 11) { return; }
    senses_d37(w, id, ctx);
    if w.mobys[id].state == 1 { return; }
    let ring = w.mobys[id].pv_s(RING) != 0;
    if ring && !grav(w) { return; }
    if !has_turn(w, id, TURN) {
        w.mobys[id].set_pv_i(TARGET, 0);
        if w.mobys[id].sub_state == 3 {
            w.mobys[id].set_state(5, 2);
            let s = if ctx.game.rng.randi(2) != 0 { 1 } else { 4 };
            set_seq(w, id, ctx, s, 0, 10);
        }
    }
    let mid = middle(w, id, ctx);
    let t = target_of(w, id, ctx, TARGET);
    let (angle, diff, d) = aim(w, id, t, ring);
    if w.mobys[id].sub_state == 2 {
        if w.mobys[id].first_sub() {
            set_seq(w, id, ctx, 6, 0, 10);
            w.mobys[id].timer = 0;
            lim(w, id, 0.004_444_444_6, 0.133_333_34, 0.006_981_318, 0.209_439_55);
            spot(w, id, ctx, mid, HOME, ring, std::f32::consts::FRAC_PI_4, None);
            w.mobys[id].state_ticks = 10000;
        }
        if w.mobys[id].timer == 1 && 301 <= w.mobys[id].state_ticks as u16 {
            match ctx.game.rng.randi(2) {
                0 => set_seq(w, id, ctx, 1, 0, 10),
                _ => set_seq(w, id, ctx, 4, 0, 10),
            }
            w.mobys[id].state_ticks = ctx.game.rng.randi(100) as i16;
        }
        if has_turn(w, id, TURN) {
            w.mobys[id].set_state(6, 3);
            set_seq(w, id, ctx, 2, 0, 10);
        }
        let home = w.mobys[id].pv_v(HOME);
        if w.mobys[id].timer == 0 && dist2(w.mobys[id].pos, home) < 0.5 {
            w.mobys[id].timer = 1;
        } else {
            go(w, id, ctx, home, ring);
        }
    }
    match w.mobys[id].state {
        3 => {
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                set_seq(w, id, ctx, 1, 0, 0);
                w.mobys[id].state_ticks = 0;
            }
            let m = &w.mobys[id];
            if m.timer == 0 && (m.pv_s(DELAY) as i32) < m.state_ticks as u16 as i32 {
                set_seq(w, id, ctx, 6, 0, 10);
                w.mobys[id].timer = 1;
            } else if w.mobys[id].timer == 1 {
                let p = w.mobys[id].pos;
                if in_middle(w, id, ctx, p) { w.mobys[id].set_state(5, 2); }
                go(w, id, ctx, mid, false);
            }
        }
        4 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 6, 0, 10);
                w.mobys[id].state_ticks = 0;
            }
            // Above it (along its up by the movement block's radius and height): anything but the boss 0x0b3a.
            let m = &w.mobys[id];
            let (r, h) = (m.pv_f(0x100), m.pv_f(0x104));
            let c = add(m.pos, set_len(m.rows[2], h + r));
            let above = ctx.coll.and_then(|cl| crate::collision::coll_sphere_m(cl, ctx.scene, c, r, crate::collision::QueryFlags(5), Some(id)));
            let blocked = above.is_some_and(|o| o.moby.and_then(|k| w.mobys.get(k)).is_some_and(|x| x.o_class == arena::B3A));
            if !blocked {
                w.mobys[id].has_collision = w.class(D37).is_some_and(|c| c.has_collision);
                w.mobys[id].set_state(5, 2);
            }
            w.mobys[id].pos[2] += 0.133_333_34;
        }
        6 => {
            if wrapped(w, id, ctx) { w.mobys[id].set_state(7, -1); }
        }
        7 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 6, 0, 15); }
            let sa = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
            if sa == 5 && 14.0 < d {
                set_seq(w, id, ctx, 6, 0, 4);
                lim(w, id, 0.004_444_444_6, 0.133_333_34, 0.006_981_318, 0.209_439_55);
            } else if sa == 6 && d < 9.0 {
                set_seq(w, id, ctx, 5, 0, 4);
                lim(w, id, 0.002_222_222_3, 0.066_666_67, 0.006_981_318, 0.209_439_55);
            }
            if d < 3.0 && diff < 0.017_453_294 {
                w.mobys[id].set_state(9, -1);
                set_seq(w, id, ctx, 3, 4, 3);
            } else if ring {
                go(w, id, ctx, t, true);
            } else if 3.0 < d {
                if !round_plates(w, id, ctx, PLATES, 4.8) {
                    let off = w.mobys[id].pv_f(OFFSET);
                    ai::move_toward(w, id, ctx, off, t);
                }
            } else {
                let p = w.mobys[id].pos;
                ai::move_step(w, id, ctx, angle, p);
            }
        }
        8 => {
            if w.mobys[id].first() { lim(w, id, 0.138_888_9, 1.666_666_7, 0.003_490_659, 0.104_719_77); }
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 7) {
                let dmg = w.mobys[id].pv_b(0x34) as f32;
                if ctx.host.frame_time(&w.mobys[id]) <= 14.0 {
                    if ring { go(w, id, ctx, t, true); } else { ai::move_step(w, id, ctx, angle, t); }
                } else {
                    lim(w, id, 0.055_555_556, 0.033_333_335, 0.003_490_659, 0.104_719_77);
                    let m = &w.mobys[id];
                    let back = add(m.pos, set_len(m.rows[0], -4.0));
                    let o = ai::mv(m) + 0x38;
                    let v = w.mobys[id].pv_i(o) | 0x10;
                    w.mobys[id].set_pv_i(o, v);
                    go(w, id, ctx, back, ring);
                    let v = w.mobys[id].pv_i(o) & !0x10;
                    w.mobys[id].set_pv_i(o, v);
                }
                ai::attack_joint(w, id, ctx, 0.4, dmg, 1.0, 0, 1, 0, 1, 0);
            }
            if wrapped(w, id, ctx) {
                if ctx.hero.damage.hit_by == Some(id) {
                    set_seq(w, id, ctx, 2, 0, 10);
                    w.mobys[id].set_state(6, -1);
                } else {
                    w.mobys[id].set_state(7, -1);
                }
            }
        }
        9 => {
            if !ring {
                ai::turn_in_place(w, id, ctx, angle);
            }
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 7, 4, 3);
                w.mobys[id].set_state(8, -1);
            }
        }
        10 => {
            let f = ai::flight(w, id, ctx);
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 8) {
                if ctx.game.rng.randi(30) == 0 || f & 1 != 0 {
                    let near = ctx.host.frame_time(&w.mobys[id]) < 10.0;
                    if near && ring && ctx.game.rng.randi(2) == 0 {
                        w.mobys[id].set_state(11, -1);
                    } else {
                        bolts(w, id, ctx);
                        pool_d37(w, id, ctx);
                        return;
                    }
                }
            } else if f & 2 != 0 {
                set_seq(w, id, ctx, 2, 0, 10);
                w.mobys[id].set_state(6, 3);
            }
        }
        11 => {
            if spin_out(w, id, ctx, 0x1a0, SPIN, RING, Some(9), 0x12c, Pivot::Up(0x30)) {
                bolts(w, id, ctx);
                pool_d37(w, id, ctx);
                return;
            }
        }
        _ => {}
    }
    // `0x437fa8`: the ticks, the head (`+0x290`, list 1, 2.5), its blow on what it touches (1.0).
    tick(w, id);
    crate::moby::joint_aim::head(w, ctx, id, 0x290, 1, 2.5);
    bump(w, id, ctx, 1.0);
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0dcf

/// Its words: movement `+0x100`, the spot `+0x470`, the spin out's way `+0x1a0` and spin `+0x480`, its target
/// `+0x490` (from `+0x220`), yaw offset `+0x494`, its parts' angle `+0x49c`, the parts `+0x4a0` / `+0x4a4` (0x0aa2 at
/// joints 0 and 1), their spin `+0x4ac` (degrees a tick), plates `+0x4c0`, area `+0x4c4`, found `+0x4c8`, turn `+0x4ca`,
/// the turn about's state and sub `+0x4cc` / `+0x4ce`, delay `+0x4d0`, placed `+0x4d2`, the head record `+0x4e0`.
mod dcf {
    pub const SPOT: usize = 0x470;
    pub const SPIN: usize = 0x480;
    pub const TARGET: usize = 0x490;
    pub const OFFSET: usize = 0x494;
    pub const ANGLE: usize = 0x49c;
    pub const PARTS: usize = 0x4a0;
    pub const PART_SPIN: usize = 0x4ac;
    pub const PLATES: usize = 0x4c0;
    pub const AREA: usize = 0x4c4;
    pub const FOUND: usize = 0x4c8;
    pub const TURN: usize = 0x4ca;
    pub const BACK: usize = 0x4cc;
    pub const DELAY: usize = 0x4d0;
    pub const RING: usize = 0x4d2;
}

/// `0x44dd60(m, delay, placed)`: also its parts shown with their collision.
fn start_dcf(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, delay: i32, placed: bool) {
    let m = &mut w.mobys[id];
    m.set_pv_s(dcf::TURN, 0);
    m.set_pv_s(0x88, 0);
    m.set_pv_b(0x2e, 1);
    m.set_pv_s(dcf::RING, placed as i16);
    lim(w, id, 0.004_444_444_6, 0.133_333_34, 0.001_163_553, 0.104_719_77);
    start_body(w, id, placed, (4, 2), 0);
    if placed { ctx.host.set_anim(&w.mobys[id], 2, 0, 0); }
    w.mobys[id].set_pv_b(0x6a, 0);
    let pcoll = w.class(PART).is_some_and(|c| c.has_collision);
    for p in super::super::enemies::parts(w, id, dcf::PARTS).into_iter().flatten() {
        let x = &mut w.mobys[p];
        x.mode &= !1;
        x.drawn = 1;
        x.has_collision = pcoll;
    }
    let m = &mut w.mobys[id];
    m.set_pv_s(dcf::DELAY, delay as i16);
    m.set_pv_i(0x200, 0);
    flash_reset(w, id);
}

const PART: i16 = 0x0aa2;

/// `0x44df28`: back to the pool — its turn given back; in view a blast (0.5), 0x0df7's sound 0 and its pieces thrown;
/// its parts hidden.
fn pool_dcf(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if has_turn(w, id, dcf::TURN) { give_back(w, id); }
    w.mobys[id].set_pv_s(dcf::TURN, 0);
    w.mobys[id].set_pv_i(dcf::TARGET, 0);
    if w.mobys[id].drawn != 0 {
        blast(w, id, ctx, 0.5, [0x1e, 5, 5], 5, 0x2d, true);
        ctx.host.play_sound_of(&w.mobys[id], DF7, 0, 0);
        let v = add(ctx.game.rng.rand_vec(0.033_333_335, 0.083_333_336), w.mobys[id].pv_v(0xd0));
        crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x7f0, v, 0xff);
    }
    for p in super::super::enemies::parts(w, id, dcf::PARTS).into_iter().flatten() {
        let x = &mut w.mobys[p];
        x.mode |= 1;
        x.drawn = 0;
        x.has_collision = false;
    }
    pool_body(w, id, ctx);
}

/// `0x44d7a0`: the set-up (its parts made), its shadow, its parts' spin, the hits, its turn, the ring's checks.
fn senses_dcf(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let a = ctx.game.rng.randf(15.0, 40.0).to_radians();
        let neg = ctx.game.rng.randi(2) != 0;
        let mirror = ctx.game.rng.randi(2) == 1;
        let m = &mut w.mobys[id];
        m.set_pv_i(0x18, 0x100);
        m.set_pv_f(dcf::OFFSET, if neg { -a } else { a });
        if mirror { m.mode |= mode::MIRROR; }
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 12);
        m.set_pv_f(0x80, 0.8);
        m.set_pv_f(0x8c, 0.4);
        for o in [0xbc, 0xbd, 0xbe, 0xbf, 0xc2, 0xc3] { m.set_pv_b(o, 0xff); }
        m.set_pv_f(0x108, 0.4);
        m.set_pv_f(dcf::PART_SPIN, 8.0);
        m.set_pv_b(0x60, 10);
        m.set_pv_i(0x98, 5);
        m.set_pv_f(0x84, 0.8);
        m.set_pv_f(0x100, 0.6);
        m.set_pv_f(0x104, 0.2);
        m.set_pv_f(0x10c, 0.4);
        m.set_pv_f(0x110, std::f32::consts::FRAC_PI_2);
        m.set_pv_f(0x130, 0.523_598_85);
        m.set_pv_f(0x12c, 1.0 / 180.0);
        m.set_pv_b(0x28, 3);
        m.set_pv_f(dcf::ANGLE, 0.0);
        if w.class(PART).is_some() { super::super::enemies::make_parts_on(w, id, id, dcf::PARTS, [0, 1], ctx); }
        w.mobys[id].set_pv_s(dcf::TURN, 0);
        pool_dcf(w, id, ctx);
    }
    let ring = w.mobys[id].pv_s(dcf::RING) != 0;
    let on = !ring && w.mobys[id].state != 11;
    shadow(w, id, ctx, on);
    let m = &mut w.mobys[id];
    let a = add_rot(m.pv_f(dcf::ANGLE), m.pv_f(dcf::PART_SPIN).to_radians());
    m.set_pv_f(dcf::ANGLE, a);
    super::super::fighters::spin_parts_at(w, id, ctx, dcf::PARTS, a, true);
    if 3 <= w.mobys[id].state {
        floor_hits(w, id, false);
        if ai::take_damage(w, id, ctx, false) == 5 {
            w.mobys[id].set_pv_i(0xe0, 0);
            w.mobys[id].set_pv_f(0xe4, 0.837_758_2);
        }
        turn(w, id, ctx, dcf::TURN, dcf::TARGET, dcf::AREA, dcf::FOUND, 0x220, false);
        if ring { ring_keep(w, id, ctx, 0x20, (11, 5)); }
    }
    if 2 <= w.mobys[id].state && w.mobys[id].pv_b(0x2e) == 2 { pool_dcf(w, id, ctx); }
}

/// `0x44e250`: the turn about (6, sub 4), its state and sub kept to go back to.
fn turn_about(w: &mut MobyWorld, id: usize, back: usize, sub: i32) {
    let m = &mut w.mobys[id];
    let (s, b) = (m.state, m.sub_state);
    m.set_pv_s(back, s as i16);
    m.set_pv_s(back + 2, b as i16);
    m.set_state(6, sub);
}

/// 0x0dcf (`0x44ca00`): a bladed robot (two spinning parts 0x0aa2 at its joints 0 and 1) on the framework (react
/// `+0x50`, movement `+0x100`) — Maktar's 0x0b22 again with the ring. Out, it waits its delay and walks in to the
/// middle's polygon (3), then wanders between spots (sub 2, idling now and then; 6 turns about for one behind it);
/// with a turn it closes on its target (7: running from 15 out, walking within 7; round raised plates at its yaw
/// offset) and within 2.8 facing it within 22° slashes (8, sequence 11 from frame 4: frames 19..25 its joints 0
/// (0.35), 1 and 2 (0.15) hit for `+0x34`; its parts spin 25° a tick from frame 17), the taunt (9) if it hurt Ratchet.
/// Knocked (sub 5), dead (or one in 30 in its death sequence 15): in the ring now and then the spin out, else its
/// bolts and back to the pool. It hits what it touches for 1 (1.3).
///
/// Not ported: the slash's trail (`0x44dbb8` keeps its points, `0x44dc58` draws them), the parts' trail points
/// (`0x4515e0`).
pub fn update_dcf(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use dcf::*;
    if w.mobys[id].state == 1 { return; }
    let ring = w.mobys[id].pv_s(RING) != 0;
    if ring && !grav(w) && !matches!(w.mobys[id].state, 10 | 11) { return; }
    senses_dcf(w, id, ctx);
    if w.mobys[id].state == 1 { return; }
    let ring = w.mobys[id].pv_s(RING) != 0;
    if ring && !grav(w) { return; }
    if !has_turn(w, id, TURN) {
        w.mobys[id].set_pv_i(TARGET, 0);
        if w.mobys[id].sub_state == 4 {
            w.mobys[id].set_state(4, 2);
            let s = if ctx.game.rng.randi(2) != 0 { 2 } else { 3 };
            set_seq(w, id, ctx, s, 0, 10);
        }
    }
    let mid = middle(w, id, ctx);
    let t = target_of(w, id, ctx, TARGET);
    let (angle, diff, d) = aim(w, id, t, ring);
    if w.mobys[id].sub_state == 2 {
        if w.mobys[id].first_sub() {
            set_seq(w, id, ctx, 10, 0, 10);
            w.mobys[id].timer = 0;
            lim(w, id, 0.002_222_222_3, 0.066_666_67, 0.001_163_553, 0.104_719_77);
            spot(w, id, ctx, mid, SPOT, ring, std::f32::consts::FRAC_PI_4, None);
            w.mobys[id].state_ticks = 10000;
        }
        if w.mobys[id].timer == 1 && 300 < w.mobys[id].state_ticks as u16 {
            let s = [2, 3, 4, 6, 7][ctx.game.rng.randi(5) as usize];
            set_seq(w, id, ctx, s, 0, 10);
            w.mobys[id].state_ticks = ctx.game.rng.randi(100) as i16;
        }
        if has_turn(w, id, TURN) {
            w.mobys[id].set_state(5, 4);
            let s = if ctx.game.rng.randi(2) != 0 { 1 } else { 5 };
            set_seq(w, id, ctx, s, 0, 10);
        }
        if 2.792_527_2 < diff { turn_about(w, id, BACK, 4); }
        let spot_p = w.mobys[id].pv_v(SPOT);
        if w.mobys[id].timer == 0 && dist2(w.mobys[id].pos, spot_p) < 0.5 {
            w.mobys[id].timer = 1;
        } else {
            go(w, id, ctx, spot_p, ring);
        }
    }
    match w.mobys[id].state {
        3 => {
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                set_seq(w, id, ctx, 2, 0, 0);
                w.mobys[id].state_ticks = 0;
            }
            let m = &w.mobys[id];
            if m.timer == 0 && (m.pv_s(DELAY) as i32) < m.state_ticks as u16 as i32 {
                set_seq(w, id, ctx, 10, 0, 10);
                w.mobys[id].timer = 1;
            } else if w.mobys[id].timer == 1 {
                let p = w.mobys[id].pos;
                if in_middle(w, id, ctx, p) { w.mobys[id].set_state(4, 2); }
                go(w, id, ctx, mid, false);
            }
        }
        5 => {
            if wrapped(w, id, ctx) { w.mobys[id].set_state(7, -1); }
        }
        6 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 9, 0, 10);
                lim(w, id, 0.004_444_444_6, 0.133_333_34, 0.006_981_318, 0.837_758_2);
            }
            ai::turn_in_place(w, id, ctx, angle);
            if wrapped(w, id, ctx) {
                let (s, b) = (w.mobys[id].pv_s(BACK), w.mobys[id].pv_s(BACK + 2));
                w.mobys[id].set_state(s as u8, b as i32);
            }
        }
        7 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 10, 0, 15); }
            let sa = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
            if sa == 8 && 15.0 < d {
                set_seq(w, id, ctx, 10, 0, 4);
                lim(w, id, 0.004_444_444_6, 0.133_333_34, 0.001_163_553, 0.104_719_77);
            } else if sa == 10 && d < 7.0 {
                set_seq(w, id, ctx, 8, 0, 4);
                lim(w, id, 0.004_444_444_6, 0.033_333_335, 0.001_163_553, 0.104_719_77);
            }
            if d < 2.8 && diff < 0.383_972_47 {
                w.mobys[id].set_state(8, -1);
                set_seq(w, id, ctx, 11, 4, 3);
            } else if 2.8 < d {
                if ring {
                    go(w, id, ctx, t, true);
                } else if !round_plates(w, id, ctx, PLATES, 4.8) {
                    let off = w.mobys[id].pv_f(OFFSET);
                    ai::move_toward(w, id, ctx, off, t);
                }
            } else {
                face(w, id, ctx, t, ring);
            }
            if 2.792_527_2 < diff { turn_about(w, id, BACK, 4); }
        }
        8 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].set_pv_f(PART_SPIN, 8.0);
                if ctx.hero.damage.hit_by == Some(id) {
                    w.mobys[id].set_state(9, -1);
                    set_seq(w, id, ctx, 12, 0, 7);
                } else {
                    w.mobys[id].set_state(7, -1);
                }
            }
            let ft = ctx.host.frame_time(&w.mobys[id]);
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 11) {
                if (19.0..=25.0).contains(&ft) {
                    let dmg = w.mobys[id].pv_b(0x34) as f32;
                    ai::attack_joint(w, id, ctx, 0.35, dmg, 1.0, 0, 0x1_0001, 0, 1, 0);
                    ai::attack_joint(w, id, ctx, 0.15, dmg, 1.0, 1, 0x1_0001, 0, 1, 0);
                    ai::attack_joint(w, id, ctx, 0.15, dmg, 1.0, 2, 0x1_0001, 0, 1, 0);
                }
                if (17.0..=25.0).contains(&ft) { w.mobys[id].set_pv_f(PART_SPIN, 25.0); }
            }
            face(w, id, ctx, t, ring);
        }
        9 => {
            if wrapped(w, id, ctx) { w.mobys[id].set_state(7, -1); }
        }
        10 => {
            let f = ai::flight(w, id, ctx);
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 15) {
                if ctx.game.rng.randi(30) == 0 || f & 1 != 0 {
                    let early = ctx.host.frame_time(&w.mobys[id]) < 15.0;
                    if early && ring && ctx.game.rng.randi(2) == 0 {
                        w.mobys[id].set_state(11, -1);
                    } else {
                        bolts(w, id, ctx);
                        pool_dcf(w, id, ctx);
                        return;
                    }
                }
            } else if f & 2 != 0 {
                w.mobys[id].set_state(7, 4);
            }
        }
        11 => {
            if spin_out(w, id, ctx, 0x1a0, SPIN, RING, Some(17), 0x12c, Pivot::Joint(1)) {
                bolts(w, id, ctx);
                pool_dcf(w, id, ctx);
                return;
            }
        }
        _ => {}
    }
    // `0x44db28`: the ticks, the head (`+0x4e0`, list 7, 4), its blow on what it touches (1.3).
    tick(w, id);
    crate::moby::joint_aim::head(w, ctx, id, 0x4e0, 7, 4.0);
    bump(w, id, ctx, 1.3);
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0df4

/// Its words: movement `+0x100`, the glows' points `+0x270` / `+0x280` / `+0x290`, the spot `+0x2a0`, the spin out's
/// way `+0x1a0` and spin `+0x2b0`, its plates `+0x2c0`, target `+0x2c4` (from `+0x220`), yaw offset `+0x2c8`, area
/// `+0x2cc`, found `+0x2d0`, fighting `+0x2d2` (set at its start: it needs no turn), delay `+0x2d4`, the rocket's wait
/// `+0x2d6`, placed `+0x2d8`, the rocket fired `+0x2da`, the head record `+0x2e0`.
mod df4 {
    pub const SPOT: usize = 0x2a0;
    pub const SPIN: usize = 0x2b0;
    pub const PLATES: usize = 0x2c0;
    pub const TARGET: usize = 0x2c4;
    pub const OFFSET: usize = 0x2c8;
    pub const AREA: usize = 0x2cc;
    pub const FOUND: usize = 0x2d0;
    pub const FIGHT: usize = 0x2d2;
    pub const DELAY: usize = 0x2d4;
    pub const WAIT: usize = 0x2d6;
    pub const RING: usize = 0x2d8;
    pub const FIRED: usize = 0x2da;
}

/// `0x4546b0(m, delay, placed)`.
fn start_df4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, delay: i32, placed: bool) {
    let m = &mut w.mobys[id];
    m.set_pv_b(0x2e, 1);
    m.set_pv_s(0x88, 0);
    m.set_pv_s(df4::FIGHT, 1);
    m.set_pv_s(df4::RING, placed as i16);
    lim(w, id, 0.004_444_444_6, 0.133_333_34, 0.001_163_553, 0.104_719_77);
    set_seq(w, id, ctx, 2, 0, 0);
    start_body(w, id, placed, (4, 2), 0);
    if placed { ctx.host.set_anim(&w.mobys[id], 2, 0, 0); }
    let m = &mut w.mobys[id];
    m.set_pv_s(df4::DELAY, delay as i16);
    m.set_pv_b(0x6a, 0);
    m.set_pv_i(0x200, 0);
    flash_reset(w, id);
}

/// `0x454850`: back to the pool — in view a blast (0.5), 0x0df7's sound 0 and its pieces thrown.
fn pool_df4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_s(df4::FIGHT, 0);
    w.mobys[id].set_pv_i(df4::TARGET, 0);
    if w.mobys[id].drawn != 0 {
        blast(w, id, ctx, 0.5, [5, 5, 25], 20, 10, true);
        ctx.host.play_sound_of(&w.mobys[id], DF7, 0, 0);
        let v = add(ctx.game.rng.rand_vec(0.083_333_336, 0.116_666_675), w.mobys[id].pv_v(0xd0));
        crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0xff0, v, 0xff);
    }
    pool_body(w, id, ctx);
}

/// `0x453ea8`: the set-up, its shadow, its glows' points (joints 1, 2 and 0.25 along joint 3's first row), the hits,
/// the ring's checks, its target.
fn senses_df4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let a = ctx.game.rng.randf(15.0, 40.0).to_radians();
        let neg = ctx.game.rng.randi(2) != 0;
        let mirror = ctx.game.rng.randi(2) == 1;
        let m = &mut w.mobys[id];
        m.set_pv_i(0x18, 0x100);
        m.set_pv_f(df4::OFFSET, if neg { -a } else { a });
        if mirror { m.mode |= mode::MIRROR; }
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 12);
        m.set_pv_b(0x60, 0xb);
        m.set_pv_i(0x98, 4);
        m.set_pv_f(0x100, 0.8);
        for o in [0x80, 0x84, 0x104] { m.set_pv_f(o, 0.5); }
        for o in [0x8c, 0x108, 0x10c] { m.set_pv_f(o, 0.4); }
        m.set_pv_f(0x110, std::f32::consts::FRAC_PI_2);
        m.set_pv_f(0x130, 0.523_598_85);
        m.set_pv_f(0x12c, 1.0 / 180.0);
        m.set_pv_f(0x134, 2.8);
        lim(w, id, 0.004_444_444_6, 0.133_333_34, 0.001_163_553, 0.104_719_77);
        w.mobys[id].set_pv_b(0x28, 3);
        pool_df4(w, id, ctx);
    }
    let ring = w.mobys[id].pv_s(df4::RING) != 0;
    let on = !ring && w.mobys[id].state != 12;
    shadow(w, id, ctx, on);
    for (k, o) in [(1, 0x270), (2, 0x280)] {
        if let Some(p) = ctx.host.joint(&w.mobys[id], k) { w.mobys[id].set_pv_v(o, p); }
    }
    if let Some((r, t)) = ctx.host.joint_matrix(&w.mobys[id], 3) { w.mobys[id].set_pv_v(0x290, add(t, scale(r[0], 0.25))); }
    if 3 <= w.mobys[id].state {
        floor_hits(w, id, false);
        if ai::take_damage(w, id, ctx, false) == 5 {
            w.mobys[id].set_pv_i(0xe0, 0);
            w.mobys[id].set_pv_f(0xe4, 0.837_758_2);
        }
        if ring { ring_keep(w, id, ctx, 0x20, (12, 4)); }
        if 2 <= w.mobys[id].state && w.mobys[id].pv_b(0x2e) == 2 { pool_df4(w, id, ctx); }
        let a = w.mobys[id].pv_i(df4::AREA);
        let f = ai::find_target(w, id, ctx, 100.0, 100.0, a, None, false);
        let m = &mut w.mobys[id];
        m.set_pv_s(df4::FOUND, f as i16);
        let t = m.pv_i(0x220);
        m.set_pv_i(df4::TARGET, t);
    }
}

/// The rocket's chance and its ranges (`$gp+0x2d74..+0x2d94`): fired between 12 and 40 out, one tick in 60, 60
/// ticks apart; within 8 the flame (stopped past 10); the run from 30 out, the walk within 20; the aim to fire within
/// 5°.
const DF4_ROCKET: (f32, f32, i32, i16) = (12.0, 40.0, 60, 60);
const DF4_FLAME: (f32, f32) = (8.0, 10.0);
const DF4_PACE: (f32, f32) = (20.0, 30.0);
const DF4_AIM: f32 = 5.0;

/// 0x0df4 (`0x452bb0`): Maktar's flame robot 0x0c3b again with the ring and a rocket. Out, it waits its delay and
/// walks in to the middle's polygon (3), then wanders between spots (sub 2, idling now and then; 6 turns about); it
/// fights at once (5 → 7, its target the framework's): from 30 out running, within 20 walking; within 8 and facing
/// it within 90° the flame (sequence 12, crate::moby::classes::fighters' jet), stopped past 10; between 12 and 40
/// now and then (60 ticks apart, the line to its target clear or ending on no fighter) the rocket (9: turned to it
/// within 5°, sequence 17; past frame 17 a fireball 0x0b7f from joint 4 a third of the way to above its target a
/// tick, crate::moby::classes::fireball::shot; before it smoke), then the taunt (10, if it hurt Ratchet or one
/// in six) or the fight; else it walks to 2.8 of it round raised plates. Knocked (sub 4), dead (or one in 30 in its
/// death sequence 15): in the ring now and then the spin out, else its bolts and back to the pool. It hits what it
/// touches for 1 (1.3), and draws three glows ([`draw_df4`]).
pub fn update_df4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use df4::*;
    if w.mobys[id].state == 1 { return; }
    let ring = w.mobys[id].pv_s(RING) != 0;
    if ring && !grav(w) && !matches!(w.mobys[id].state, 11 | 12) { return; }
    senses_df4(w, id, ctx);
    if w.mobys[id].state == 1 { return; }
    let ring = w.mobys[id].pv_s(RING) != 0;
    if ring && !grav(w) { return; }
    if w.mobys[id].pv_s(FIGHT) == 0 && w.mobys[id].sub_state == 3 {
        w.mobys[id].set_state(4, 2);
        let s = if ctx.game.rng.randi(2) != 0 { 2 } else { 3 };
        set_seq(w, id, ctx, s, 0, 10);
    }
    let mid = middle(w, id, ctx);
    let tl = ai::link(&w.mobys[id], TARGET);
    let t = target_of(w, id, ctx, TARGET);
    let (heading, diff, d) = aim(w, id, t, ring);
    if w.mobys[id].sub_state == 2 {
        if w.mobys[id].first_sub() {
            set_seq(w, id, ctx, 9, 0, 10);
            w.mobys[id].timer = 0;
            lim(w, id, 0.004_444_444_6, 0.133_333_34, 0.000_581_776_5, 0.052_359_886);
            spot(w, id, ctx, mid, SPOT, ring, std::f32::consts::FRAC_PI_4, None);
            w.mobys[id].state_ticks = 10000;
        }
        if w.mobys[id].timer == 1 && 301 <= w.mobys[id].state_ticks as u16 {
            match ctx.game.rng.randi(5) {
                0 => set_seq(w, id, ctx, 2, 0, 10),
                1 => set_seq(w, id, ctx, 3, 0, 10),
                2 => set_seq(w, id, ctx, 5, 0, 10),
                3 => set_seq(w, id, ctx, 6, 0, 10),
                _ => {}
            }
            w.mobys[id].state_ticks = ctx.game.rng.randi(100) as i16;
        }
        if w.mobys[id].pv_s(FIGHT) != 0 {
            w.mobys[id].set_state(5, 3);
            let s = if ctx.game.rng.randi(2) != 0 { 1 } else { 4 };
            set_seq(w, id, ctx, s, 0, 10);
        }
        if 2.792_527_2 < diff { w.mobys[id].set_state(6, 3); }
        let sp = w.mobys[id].pv_v(SPOT);
        if w.mobys[id].timer == 0 && dist2(w.mobys[id].pos, sp) < 0.5 {
            w.mobys[id].timer = 1;
        } else {
            go(w, id, ctx, sp, ring);
        }
    }
    match w.mobys[id].state {
        3 => {
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                set_seq(w, id, ctx, 2, 0, 0);
                w.mobys[id].state_ticks = 0;
            }
            let m = &w.mobys[id];
            if m.timer == 0 && (m.pv_s(DELAY) as i32) < m.state_ticks as u16 as i32 {
                set_seq(w, id, ctx, 9, 0, 10);
                w.mobys[id].timer = 1;
            } else if w.mobys[id].timer == 1 {
                let p = w.mobys[id].pos;
                if in_middle(w, id, ctx, p) { w.mobys[id].set_state(4, 2); }
                go(w, id, ctx, mid, false);
            }
        }
        5 => {
            if wrapped(w, id, ctx) { w.mobys[id].set_state(7, -1); }
        }
        6 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 8, 0, 10);
                lim(w, id, 0.004_444_444_6, 0.133_333_34, 0.003_490_659, 0.418_879_1);
            }
            ai::turn_in_place(w, id, ctx, heading);
            if wrapped(w, id, ctx) { w.mobys[id].set_state(7, -1); }
        }
        7 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 9, 0, 15);
                w.mobys[id].set_pv_s(WAIT, 0);
            }
            if DF4_PACE.1 < d {
                lim(w, id, 0.004_444_444_6, 0.133_333_34, 0.000_581_776_5, 0.052_359_886);
            } else if d < DF4_PACE.0 {
                lim(w, id, 0.001_111_111_1, 0.033_333_335, 0.000_581_776_5, 0.052_359_886);
            }
            let sa = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
            if sa != 12 && d < DF4_FLAME.0 && diff < std::f32::consts::FRAC_PI_2 {
                set_seq(w, id, ctx, 12, 0, 10);
            } else if sa == 12 && DF4_FLAME.1 < d {
                set_seq(w, id, ctx, 9, 0, 15);
            } else if sa != 12 && DF4_ROCKET.0 < d && d < DF4_ROCKET.1 && ctx.game.rng.randi(DF4_ROCKET.2) == 0 {
                let mut wait = w.mobys[id].pv_s(WAIT) as i32;
                let over = crate::moby::countdown(&mut wait);
                w.mobys[id].set_pv_s(WAIT, wait as i16);
                if over && w.mobys[id].drawn != 0 {
                    let p = w.mobys[id].pos;
                    let clear = match w.coll_line(ctx, p, t, 1, Some(id)) {
                        None => true,
                        Some(o) => o.moby.and_then(|k| w.mobys.get(k)).is_some_and(|x| !arena::FIGHTERS.contains(&x.o_class)),
                    };
                    if clear {
                        w.mobys[id].set_state(9, -1);
                        w.mobys[id].set_pv_s(WAIT, DF4_ROCKET.3);
                    }
                }
            } else if 2.8 < d {
                if ring {
                    go(w, id, ctx, t, true);
                } else if !round_plates(w, id, ctx, PLATES, 4.8) {
                    let off = w.mobys[id].pv_f(OFFSET);
                    ai::move_toward(w, id, ctx, off, t);
                }
            } else {
                face(w, id, ctx, t, ring);
            }
            if 2.792_527_2 < diff { w.mobys[id].set_state(6, 3); }
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 12) { super::super::fighters::flame(w, id, ctx, d); }
        }
        9 => {
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                set_seq(w, id, ctx, 7, 0, 10);
                w.mobys[id].set_pv_s(FIRED, 0);
            }
            if w.mobys[id].timer == 0 && diff < DF4_AIM.to_radians() {
                w.mobys[id].timer = 1;
                set_seq(w, id, ctx, 17, 0, 10);
            } else {
                face(w, id, ctx, t, ring);
            }
            if w.mobys[id].timer == 1 {
                let ft = ctx.host.frame_time(&w.mobys[id]);
                if w.mobys[id].pv_s(FIRED) == 0 && 17.0 < ft {
                    w.mobys[id].set_pv_s(FIRED, 1);
                    let at = ctx.host.joint(&w.mobys[id], 4).unwrap_or(w.mobys[id].pos);
                    let up = tl.or(ctx.hero_moby).and_then(|k| w.mobys.get(k)).map_or([0.0, 0.0, 1.0], |x| x.rows[2]);
                    let goal = add(up, t);
                    let vel = scale(sub(goal, at), 1.0 / 3.0);
                    let dmg = w.mobys[id].pv_b(0x35) as f32;
                    super::super::fireball::shot(w, id, ctx, dmg, vel, at);
                } else if ft < 17.0 && seq(w, id, ctx) == 17 {
                    smoke(w, id, ctx);
                }
                if wrapped(w, id, ctx) {
                    if ctx.hero.damage.hit_by == Some(id) || ctx.game.rng.randi(6) == 0 {
                        w.mobys[id].set_state(10, -1);
                    } else {
                        w.mobys[id].set_state(7, -1);
                    }
                }
            }
        }
        10 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 11, 0, 7); }
            if wrapped(w, id, ctx) { w.mobys[id].set_state(7, -1); }
        }
        11 => {
            let f = ai::flight(w, id, ctx);
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 15) {
                if ctx.game.rng.randi(30) == 0 || f & 1 != 0 {
                    let early = ctx.host.frame_time(&w.mobys[id]) < 15.0;
                    if early && ring && ctx.game.rng.randi(2) == 0 {
                        w.mobys[id].set_state(12, -1);
                    } else {
                        bolts(w, id, ctx);
                        pool_df4(w, id, ctx);
                        return;
                    }
                }
            } else if f & 2 != 0 {
                w.mobys[id].set_state(7, 3);
            }
        }
        12 => {
            if spin_out(w, id, ctx, 0x1a0, SPIN, RING, Some(18), 0x12c, Pivot::Joint(1)) {
                bolts(w, id, ctx);
                pool_df4(w, id, ctx);
                return;
            }
        }
        _ => {}
    }
    // `0x454268`: the ticks, the head (`+0x2e0`, list 5, 4), its blow on what it touches (1.3); its glows' draw.
    tick(w, id);
    crate::moby::joint_aim::head(w, ctx, id, 0x2e0, 5, 4.0);
    bump(w, id, ctx, 1.3);
    w.register_draw(id);
}

/// The rocket's smoke before it fires: a grey sprite at joint 4 (`randf(1, 1.5)` → `randf(0.2, 0.4)`, life 4).
fn smoke(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(p) = ctx.host.joint(&w.mobys[id], 4) else { return };
    let g = &mut ctx.game.rng;
    let s0 = g.randf(1.0, 1.5) * 210_000.0;
    let s1 = g.randf(0.2, 0.4) * 210_000.0;
    let fall = g.randf_signed(0.0, 0.1);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    crate::particles::sprite(sys, g, s0, s1, fall, p, [0.0; 3], 4, 0x80_8080, 9, 0x4f, 0, 0);
}

/// `0x4542f8` (0x0df4 past its first sub): its three glows (`0x454430`: crate::fx::glow) — the visor at joint 1
/// (0.125, orange, 0x20 / 0x20), the chest at joint 2 (0.15, blue, 0x30 / 0x20), the mouth (0.15, blue, 0x10 / 0x30).
pub fn draw_df4(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    if m.sub_state == 0 { return; }
    crate::fx::glow(out, rng, 0.125, m.pv_v(0x270), 0x0000_36f6, 0x20, 0x20, cam);
    crate::fx::glow(out, rng, 0.15, m.pv_v(0x280), 0x0010_50b0, 0x30, 0x20, cam);
    crate::fx::glow(out, rng, 0.15, m.pv_v(0x290), 0x0010_50b0, 0x10, 0x30, cam);
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0df7

/// Its words: movement `+0x60` (its velocity `+0x100`, the vertical speed `+0x108`), the spin `+0x1b0`, its paths
/// `+0x1c0` (off the ring) / `+0x1e0` (in it), its target `+0x1c4` (from `+0x160`), the path height `+0x1c8`, the
/// yaw offset `+0x1d0`, the path point `+0x1d4`, its speed `+0x1d8` (a second, then a tick), the hum `+0x1dc`, area
/// `+0x1e4`, the way round `+0x1e8`, found `+0x1ea`, turn `+0x1ec`, delay `+0x1ee`, its colour `+0x1f0..+0x1f8`,
/// placed `+0x1fc`, the flash's end `+0x1fe`.
mod df7 {
    pub const VEL: usize = 0x100;
    pub const SPIN: usize = 0x1b0;
    pub const PATH: usize = 0x1c0;
    pub const TARGET: usize = 0x1c4;
    pub const HEIGHT: usize = 0x1c8;
    pub const OFFSET: usize = 0x1d0;
    pub const POINT: usize = 0x1d4;
    pub const SPEED: usize = 0x1d8;
    pub const HUM: usize = 0x1dc;
    pub const RING_PATH: usize = 0x1e0;
    pub const AREA: usize = 0x1e4;
    pub const WAY: usize = 0x1e8;
    pub const FOUND: usize = 0x1ea;
    pub const TURN: usize = 0x1ec;
    pub const DELAY: usize = 0x1ee;
    pub const COLOUR: usize = 0x1f0;
    pub const RING: usize = 0x1fc;
    pub const FLASHED: usize = 0x1fe;
}

/// `0x4566d0(m, delay, placed)`: its spin cleared, its yaw `randf(20°, 30°)` (`+0x1cc`).
fn start_df7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, delay: i32, placed: bool) {
    let m = &mut w.mobys[id];
    m.set_pv_s(df7::RING, placed as i16);
    m.set_pv_s(df7::TURN, 0);
    m.set_pv_b(0x2e, 1);
    set_seq(w, id, ctx, 0, 0, 10);
    start_body(w, id, placed, (7, 3), 1);
    let a = ctx.game.rng.randf(20.0, 30.0).to_radians();
    let m = &mut w.mobys[id];
    m.set_pv_v(df7::SPIN, [0.0; 3]);
    m.set_pv_s(df7::DELAY, delay as i16);
    m.set_pv_f(0x1cc, a);
    if m.pv_s(0x50) != 0 {
        m.set_pv_s(0x50, 0);
        m.ambient = [m.pv_i(df7::COLOUR) as u8, m.pv_i(df7::COLOUR + 4) as u8, m.pv_i(df7::COLOUR + 8) as u8];
        m.set_pv_s(0x52, 1);
    }
}

/// `0x456830`: back to the pool — its hum stopped, its turn given back; in view a blast (0.3) and its sound 0.
fn pool_df7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(df7::HUM);
    if h != -1 { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(df7::HUM, -1);
    if has_turn(w, id, df7::TURN) {
        w.mobys[id].set_pv_s(df7::TURN, 0);
        give_back(w, id);
    }
    if w.mobys[id].drawn != 0 {
        blast(w, id, ctx, 0.3, [10, 0x1e, 0], 10, 0x2d, true);
        ctx.host.play_sound_of(&w.mobys[id], DF7, 0, 0);
    }
    pool_body(w, id, ctx);
    w.mobys[id].set_pv_s(df7::TURN, 0);
    w.mobys[id].set_pv_i(df7::TARGET, 0);
}

/// `0x455f60`: the set-up, its shadow, its hum, the hits (knocked spinning: 0xd, sub 4; in the ring not toward the
/// middle), the ring's checks, its turn (`0x4565b8`).
fn senses_df7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use df7::*;
    if w.mobys[id].state == 0 {
        let r = w.mobys[id].pv_f(OFFSET);
        let off = ctx.game.rng.randf(-r, r).to_radians();
        let m = &mut w.mobys[id];
        m.set_pv_i(0x18, 0x60);
        m.scale *= 0.75;
        m.set_pv_i(HUM, -1);
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 12);
        m.set_pv_f(OFFSET, off);
        m.update_dist = 0x80;
        m.set_pv_i(POINT, 1);
        m.set_pv_f(SPEED, m.pv_f(SPEED) / 60.0);
        lim(w, id, 0.027_777_778, 1.666_666_7, 0.001_163_553, 0.104_719_77);
        let m = &mut w.mobys[id];
        m.set_pv_i(0x98, m.pv_i(0x98) | 4);
        m.set_pv_f(0x60, 0.75);
        m.set_pv_f(0x68, 0.75);
        m.set_pv_f(0x64, 0.0);
        m.set_pv_f(0x6c, 0.5);
        m.set_pv_f(0x70, 1.134_464_1);
        m.set_pv_f(0x90, 0.523_598_85);
        m.set_pv_f(0x8c, 1.0 / 360.0);
        let c = m.ambient;
        for k in 0..3 { m.set_pv_i(COLOUR + 4 * k, c[k] as i32); }
        m.set_pv_b(0x28, 3);
        m.set_pv_i(0x30, 0);
        m.set_pv_s(TURN, 0);
        pool_df7(w, id, ctx);
    }
    let ring = w.mobys[id].pv_s(RING) != 0;
    let on = !ring && w.mobys[id].state != 0xe;
    shadow(w, id, ctx, on);
    let h = w.mobys[id].pv_i(HUM);
    if 2 <= w.mobys[id].sub_state {
        if !ctx.host.sound_playing(h) {
            let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
            w.mobys[id].set_pv_i(HUM, h);
        }
    } else if ctx.host.sound_playing(h) {
        ctx.host.stop_sound(h);
        w.mobys[id].set_pv_i(HUM, -1);
    }
    let o = 0x98;
    let v = if ring { w.mobys[id].pv_i(o) & !0x10 } else { w.mobys[id].pv_i(o) | 0x10 };
    w.mobys[id].set_pv_i(o, v);
    floor_hits(w, id, true);
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
    let dmg = ai::damage_of(w, id, hit.as_ref(), false);
    w.clear_hit(id);
    let m = &mut w.mobys[id];
    m.set_pv_f(0x20, m.pv_f(0x20) - dmg);
    if dmg != 0.0 && !matches!(m.state, 3 | 0xe) {
        let h = hit.expect("damage without a hit");
        let mut push = add(set_len([h.tmpl.dir[0], h.tmpl.dir[1], h.tmpl.dir[2]], 0.833_333_4), ctx.game.rng.rand_vec(-0.138_888_9, 0.138_888_9));
        if !ring {
            push[2] = 0.0;
        } else {
            let mid = middle(w, id, ctx);
            let p = w.mobys[id].pos;
            let n = set_len([mid[0] - p[0], mid[1] - p[1], 0.0], 1.0);
            let k = crate::hero::physics::dot(push, n);
            if k < 0.0 {
                let l = crate::hero::physics::len(push);
                push = set_len(sub(push, scale(n, k)), l);
            }
        }
        let sp = ctx.game.rng.rand_vec(0.001_745_329_5, 0.006_981_318);
        let m = &mut w.mobys[id];
        let v = m.pv_v(VEL);
        m.set_pv_v(VEL, add(v, push));
        for k in 0..3 { m.set_pv_f(SPIN + 4 * k, add_rot(m.pv_f(SPIN + 4 * k), sp[k])); }
        ai::flash_start(m, 0x50);
        m.set_state(0xd, 4);
    }
    if ring {
        let p = w.mobys[id].pos;
        if !in_arena(w, id, ctx, p) {
            w.mobys[id].set_pv_f(0x20, 0.0);
            w.mobys[id].set_state(0xe, 4);
        }
        if w.mobys[id].state != 0xe {
            let mid = middle(w, id, ctx);
            let m = &mut w.mobys[id];
            let v = [m.pos[0] - mid[0], m.pos[1] - mid[1], 0.0];
            if 225.0 < v[0] * v[0] + v[1] * v[1] {
                let k = set_len(v, 15.0);
                m.pos = [mid[0] + k[0], mid[1] + k[1], m.pos[2]];
            }
        }
    }
    if 2 <= w.mobys[id].state && w.mobys[id].pv_b(0x2e) == 2 { pool_df7(w, id, ctx); }
    // `0x4565b8`: its turn (asked only while drawn); without one the dive and the bite go back to circling (or 5
    // without a path).
    if w.mobys[id].state < 4 {
        w.mobys[id].set_pv_s(TURN, 0);
        w.mobys[id].set_pv_i(TARGET, 0);
    } else if has_turn(w, id, TURN) || (w.mobys[id].drawn != 0 && claim(w, id)) {
        w.mobys[id].set_pv_s(TURN, 1);
        let a = w.mobys[id].pv_i(AREA);
        let f = ai::find_target(w, id, ctx, 100.0, 100.0, a, None, false);
        let m = &mut w.mobys[id];
        m.set_pv_s(FOUND, f as i16);
        let t = m.pv_i(0x160);
        m.set_pv_i(TARGET, t);
    } else {
        w.mobys[id].set_pv_s(TURN, 0);
        if matches!(w.mobys[id].state, 0xb | 8) {
            if w.mobys[id].pv_i(PATH) != -1 { w.mobys[id].set_state(7, 3); } else { w.mobys[id].set_state(5, 2); }
        }
    }
}

fn ease_z(w: &mut MobyWorld, id: usize, z: f32, a: f32, b: f32, max: f32) {
    let m = &mut w.mobys[id];
    let (mut x, mut v) = (m.pos[2], m.pv_f(0x108));
    crate::hero::physics::ease(z, a, b, max, &mut x, &mut v);
    m.pos[2] = x;
    m.set_pv_f(0x108, v);
}

/// The spin's eased speeds (`$gp+0x2db0`, `0x1b1da4`, `0x1b1da8`: 0, 0, 180° a second).
const DF7_SPIN: V3 = [0.0, 0.0, 180.0];

/// 0x0df7 (`0x454db0`): Maktar's spinning flier 0x0c3e again with the ring. Out, it waits its delay and flies in to
/// the middle's polygon (3), then circles its path (7: `+0x1c0`, or in the ring `+0x1e0` pushed out to 15 − its
/// height from the middle) at 1..3 above the middle's height, easing its speed down near the path's end; with a turn
/// it dives at its target (8: to 0.5 above the middle's height; in the ring 10 at a time) and stops (10), bites (9 →
/// 11: its four joints hit for `+0x34`, backing off within 1.8) until he is hurt by it (12) or 4 away after two
/// seconds (4, back to the dive after 25 ticks). Hit, it is knocked away spinning (0xd, sub 4: falling under its
/// gravity until it lands, its blow on what it lands on) and back to the dive, dead its bolts and the pool (in the
/// ring one in two the spin out, 0xe). Low over the floor it skims at 0.3, splashing. It hits what it touches for 5
/// (0.8). In the ring it is kept 14.3 from the middle across.
///
/// Not ported: `0x32ff00(m)` in the circling (its use is unknown here).
pub fn update_df7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use df7::*;
    if w.mobys[id].state == 1 { return; }
    let ring = w.mobys[id].pv_s(RING) != 0;
    if ring && !grav(w) && !matches!(w.mobys[id].state, 13 | 14) { return; }
    senses_df7(w, id, ctx);
    if w.mobys[id].state == 1 { return; }
    let ring = w.mobys[id].pv_s(RING) != 0;
    if ring && !grav(w) { return; }
    let t = target_of(w, id, ctx, TARGET);
    let pos = w.mobys[id].pos;
    let d = dist(pos, t);
    let hero = ctx.hero.pos;
    let mid = middle(w, id, ctx);
    match w.mobys[id].state {
        3 => {
            if w.mobys[id].first() {
                lim(w, id, 0.027_777_778, 1.666_666_7, 0.001_163_553, 0.104_719_77);
                w.mobys[id].timer = 0;
                w.mobys[id].state_ticks = 0;
            }
            let m = &w.mobys[id];
            if m.timer == 0 && (m.pv_s(DELAY) as i32) < m.state_ticks as u16 as i32 {
                w.mobys[id].timer = 1;
            } else if m.timer == 1 {
                if in_middle(w, id, ctx, pos) { w.mobys[id].set_state(7, 3); }
                go(w, id, ctx, mid, false);
            }
        }
        4 => {
            if w.mobys[id].first() { w.mobys[id].state_ticks = 0; }
            if 26 <= w.mobys[id].state_ticks as u16 {
                set_seq(w, id, ctx, 0, 0, 5);
                w.mobys[id].set_state(8, 3);
            }
        }
        7 => {
            let pi = w.mobys[id].pv_i(if ring { RING_PATH } else { PATH });
            let pts = usize::try_from(pi).ok().and_then(|i| ctx.level.paths.get(i)).cloned().unwrap_or_default();
            let n = pts.len() as i32;
            if w.mobys[id].first() {
                lim(w, id, 0.027_777_778, 3.333_333_5, 0.001_163_553, 0.104_719_77);
                let h = ctx.game.rng.randf(1.0, 3.0);
                w.mobys[id].set_pv_f(HEIGHT, h);
                let mut best = 0;
                let mut near = pts.first().map_or(0.0, |p| dist(pos, [p[0], p[1], p[2]]));
                for (k, p) in pts.iter().enumerate().take((n - 1).max(0) as usize) {
                    let e = dist(pos, [p[0], p[1], p[2]]);
                    if e < near { near = e; best = k; }
                }
                w.mobys[id].set_pv_i(POINT, best as i32);
                let way = ctx.game.rng.randi(2);
                w.mobys[id].set_pv_s(WAY, way as i16);
            } else if 0 < n {
                let k = w.mobys[id].pv_i(POINT).clamp(0, n - 1) as usize;
                let mut goal = [pts[k][0], pts[k][1], pts[k][2]];
                let h = w.mobys[id].pv_f(HEIGHT);
                if ring {
                    let v = set_len([goal[0] - mid[0], goal[1] - mid[1], 0.0], 15.0 - h);
                    goal[0] = mid[0] + v[0];
                    goal[1] = mid[1] + v[1];
                } else {
                    goal[2] = h + mid[2];
                }
                let m = &mut w.mobys[id];
                let step = set_len(sub(goal, m.pos), m.pv_f(SPEED));
                m.pos = add(m.pos, step);
                let near = if ring { 4.0 } else { 2.0 };
                let close = dist(m.pos, goal) < near;
                let mut i = m.pv_i(POINT);
                if m.pv_s(WAY) != 0 {
                    if close { i -= 1; }
                    if i < 0 { i = n - 1; }
                } else {
                    if close { i += 1; }
                    if n <= i { i = 0; }
                }
                m.set_pv_i(POINT, i);
                if n - 6 <= i && 0.100_000_01 <= m.pv_f(SPEED) { m.set_pv_f(SPEED, m.pv_f(SPEED) - 0.0025); }
                if ring {
                    frames::move_to(w, id, ctx, goal);
                } else {
                    let off = w.mobys[id].pv_f(OFFSET);
                    ai::move_toward(w, id, ctx, off, goal);
                    ease_z(w, id, h, 0.000_555_555_6, 0.000_555_555_6, 0.166_666_67);
                }
            }
            if w.mobys[id].pv_s(TURN) != 0 { w.mobys[id].set_state(8, 3); }
        }
        8 => {
            if w.mobys[id].first() { lim(w, id, 0.011_111_111, 1.333_333_4, 0.001_163_553, 0.104_719_77); }
            let close = if ring {
                let goal = if 4.0 < d { add(pos, set_len(sub(t, pos), 10.0)) } else { t };
                frames::move_to(w, id, ctx, goal);
                d < 4.0
            } else {
                let z = mid[2] + 0.5;
                ease_z(w, id, z, 0.005_555_556, 0.027_777_778, 0.166_666_67);
                let off = w.mobys[id].pv_f(OFFSET);
                ai::move_toward(w, id, ctx, off, t);
                (w.mobys[id].pos[2] - z).abs() < 0.1 && dist2(w.mobys[id].pos, hero) < 2.0
            };
            if close {
                w.mobys[id].state_ticks = 0;
                w.mobys[id].set_state(10, -1);
            }
            let (p, dmg) = (w.mobys[id].pos, w.mobys[id].pv_b(0x34) as f32);
            ai::attack_sphere(w, id, ctx, 0.3, dmg, 1.0, p, 1, 0, 1, 0);
        }
        9 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 2, 0, 3); }
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 3, 0, 5);
                w.mobys[id].set_state(11, -1);
            }
        }
        11 => {
            if w.mobys[id].first_sub() {
                lim(w, id, 0.002_222_222_3, 0.066_666_67, 0.001_163_553, 0.104_719_77);
                w.mobys[id].state_ticks = 0;
            }
            ctx.host.set_anim_speed(&w.mobys[id], 1.75);
            let dmg = w.mobys[id].pv_b(0x34) as f32;
            for j in 0..4 { ai::attack_joint(w, id, ctx, 0.5, dmg, 1.0, j, 1, 0, 1, 0); }
            let goal = if 1.8 <= dist(pos, hero) { t } else { add(pos, set_len(sub(pos, t), 2.0)) };
            if ring {
                frames::move_to(w, id, ctx, goal);
            } else {
                ease_z(w, id, mid[2] + 0.5, 0.005_555_556, 0.027_777_778, 0.166_666_67);
                go(w, id, ctx, goal, false);
            }
            if ctx.hero.damage.hit_by == Some(id) {
                w.mobys[id].set_state(12, -1);
                set_seq(w, id, ctx, 4, 0, 5);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
            if 4.0 < dist(w.mobys[id].pos, hero) && 121 <= w.mobys[id].state_ticks as u16 {
                set_seq(w, id, ctx, 5, 0, 15);
                w.mobys[id].set_state(4, -1);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
        }
        12 => {
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 0, 0, 15);
                w.mobys[id].set_state(8, -1);
            }
        }
        10 => {
            if 15 <= w.mobys[id].state_ticks as u16 {
                w.mobys[id].set_state(9, -1);
                set_seq(w, id, ctx, 2, 0, 2);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
            if d < 1.0 {
                let (p, dmg) = (w.mobys[id].pos, w.mobys[id].pv_b(0x34) as f32);
                ai::attack_sphere(w, id, ctx, 0.3, dmg, 1.0, p, 1, 0, 1, 0);
            }
        }
        13 => {
            // Knocked away: it falls by its gravity, moving its velocity, until it lands (or, in the air, one tick in
            // 30), its blow on what it lands on.
            let old = w.mobys[id].pos;
            let m = &mut w.mobys[id];
            let mut v = m.pv_v(VEL);
            v[2] -= m.pv_f(0x8c);
            m.set_pv_v(VEL, v);
            m.pos = add(m.pos, v);
            let new = m.pos;
            let landed = w.coll_line(ctx, old, new, 0, Some(id)).is_some()
                || ctx.coll.is_some_and(|c| crate::collision::coll_sphere_m(c, ctx.scene, new, 0.3, crate::collision::QueryFlags(4), Some(id)).is_some())
                || ctx.game.rng.randi(30) == 0;
            if landed {
                let dmg = w.mobys[id].pv_b(0x34) as f32;
                ai::attack_sphere(w, id, ctx, 0.5, dmg, 1.0, new, 0x1_0000, 0, 1, 0);
                if 0.0 < w.mobys[id].pv_f(0x20) {
                    w.mobys[id].set_state(8, 3);
                    set_seq(w, id, ctx, 0, 0, 5);
                } else if ring && ctx.game.rng.randi(2) == 0 {
                    w.mobys[id].set_state(14, -1);
                } else {
                    crate::moby::bolts::drop(w, id, 0, 1, ctx);
                    pool_df7(w, id, ctx);
                    return;
                }
            }
        }
        14 => {
            if spin_out(w, id, ctx, VEL, SPIN, RING, None, 0x8c, Pivot::Up(0x90)) {
                bolts(w, id, ctx);
                pool_df7(w, id, ctx);
                return;
            }
        }
        _ => {}
    }
    if ring && w.mobys[id].sub_state != 4 {
        let m = &mut w.mobys[id];
        let v = set_len([m.pos[0] - mid[0], m.pos[1] - mid[1], 0.0], 14.3);
        m.pos = [mid[0] + v[0], mid[1] + v[1], m.pos[2]];
    }
    if w.mobys[id].state != 14 {
        // The spin (its speeds eased a sixteenth of the way to their ends each tick).
        let m = &mut w.mobys[id];
        for k in 0..3 {
            let s = m.pv_f(SPIN + 4 * k);
            let s = add_rot(s, sub_rot(DF7_SPIN[k] * 0.000_290_888_25, s) * 0.0625);
            m.set_pv_f(SPIN + 4 * k, s);
            m.rot[k] = add_rot(m.rot[k], s);
        }
    }
    if !ring && w.mobys[id].pos[2] <= mid[2] + 0.4 {
        let skim = w.mobys[id].drawn == 0 || ctx.coll.is_some_and(|c| crate::collision::coll_sphere_m(c, ctx.scene, w.mobys[id].pos, 0.3, crate::collision::QueryFlags(6), None).is_some());
        if skim {
            let m = &mut w.mobys[id];
            m.set_pv_f(0x108, 1.0 / 60.0);
            m.pos[2] = mid[2] + 0.3;
            if m.drawn != 0 { splash(w, id, ctx); }
        }
    }
    let p = w.mobys[id].pos;
    ai::attack_sphere(w, id, ctx, 0.8, 5.0, 1.0, p, 0x400_0000, 0, 1, 0);
    // `0x456538`: the colour flash and its end, the ticks.
    let m = &mut w.mobys[id];
    ai::flash(m, 0x50);
    if m.pv_s(0x50) == 0 {
        if m.pv_s(FLASHED) == 0 {
            m.ambient = [m.pv_i(COLOUR) as u8, m.pv_i(COLOUR + 4) as u8, m.pv_i(COLOUR + 8) as u8];
            m.set_pv_s(FLASHED, 1);
        }
    } else {
        m.set_pv_s(FLASHED, 0);
    }
    tick(w, id);
}

/// The skim's splash: a twinkle `randf(0.3, 0.8)` (a tenth of that at its end) just under it, rising.
fn splash(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let rng = &mut ctx.game.rng;
    let pos = w.mobys[id].pos;
    let p = [pos[0] + rng.randf(-0.01, 0.01), pos[1] + rng.randf(-0.01, 0.01), pos[2] - 0.3];
    let s = rng.randf(0.3, 0.8);
    let spin = if rng.randi(2) == 0 { -1 } else { 1 };
    let mut v = rng.rand_vec(0.000_833_333_4, 0.05);
    v[2] = rng.randf(1.0, 5.0) / 60.0;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    crate::particles::twinkle(sys, &mut ctx.game.rng, s * 0.1, s, 0.002, p, 0x3c, 0x7f00_7f7f, 0, spin, v, 0);
}
