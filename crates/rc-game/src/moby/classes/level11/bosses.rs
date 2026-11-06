//! Joba's arena bosses (L11 addresses; the master sends them, crate::moby::classes::level11::arena's `send_boss`):
//! the flying chain **0x0d4a** ([`update_d4a`]) with its ten links **0x0d4b** ([`update_d4b`]), which fall off and
//! fight, and its bombs **0x0e20** ([`update_e20`]); the links' bolts **0x0e2d** ([`update_e2d`]); and the ring wall's
//! **0x0b3a** ([`update_b3a`]).
//!
//! Not ported: the chain's energy strip between its links (`0x43a0c0`, a draw callback), the links' and the chain's
//! blast of sparks and rings (`0x43ae80`: here its sound alone), the links' head aim (`0x43c050`, asm) and joint
//! records (`+0x220` / `+0x260`), the boss HUD's records (`0x1ffa78`), the bombs' trail and glow (`0x45a6f0`), the
//! bolts' draw (`0x45b800`) and burst (`0x45b0f8`), 0x0b3a's arcs' draw and wiggle (`0x3f51c8`, `0x3f5570`) and the
//! head record (`+0x2a0`).

use crate::hero::physics::{add, add_rot, approach, atan2, cross, dist, dist2, dot, len, scale, set_len, sub, turn_ease, V3};
use crate::moby::{ai, mode, Ctx, MobyWorld};
use super::super::props::{set_seq, wrapped};
use super::arena::{self, D4A, D4B, DF7};
use super::fighters::{bolts, lim, master, middle, tick};

const E20: i16 = 0x0e20;
const E2D: i16 = 0x0e2d;

fn moby_at(w: &MobyWorld, i: i32) -> Option<usize> { usize::try_from(i).ok().filter(|&i| w.mobys.get(i).is_some_and(|m| m.state < 0x80)) }

/// The arena's boss owner word (`0x4417a8(master, m)`: the boss HUD's moby `0x1ba780 + 0x284` follows it).
fn set_owner(w: &mut MobyWorld, id: usize, who: Option<usize>) {
    if let Some(ms) = master(w, id) { ai::set_link(&mut w.mobys[ms], arena::pv::OWNER, who); }
}

/// A basis from a way (`row 0` the way, up z): the links' and the tail's rows.
fn basis(v: V3) -> [V3; 3] {
    let r0 = set_len(v, 1.0);
    let r1 = set_len(cross(r0, [0.0, 0.0, 1.0]), 1.0);
    let r2 = set_len(cross(r1, r0), 1.0);
    [r0, r1, r2]
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0d4a

/// Its words: health `+0x20`, the target block `+0x50` (its place; its moby `+0x60`), the flash `+0x70`, the head
/// record `+0x80`, the chain's 13 points `+0x100` (their last places `+0x1d0`), the angles' speeds `+0x2a0`, the
/// paths to pick from `+0x2b0..+0x2ec` (four rows of four) and its first `+0x320`, the links' lengths `+0x2f0` (12),
/// the path `+0x324` (+ 1), found `+0x328`, its speed `+0x32c`, the links 0x0d4b `+0x330..` (10), the tail `+0x358`,
/// the damage summed `+0x360`, the link fallen `+0x364`, the bombs' group `+0x368`, area `+0x36c`, the path point
/// `+0x370`, the bomb `+0x374` and its wait `+0x376`, the links left `+0x378`, the path toggle `+0x37a`, the links'
/// states `+0x380..` (0 on, 1 falling off, 2 gone), its hum `+0x38c`.
mod d4a {
    pub const TARGET: usize = 0x50;
    pub const POINTS: usize = 0x100;
    pub const OLD: usize = 0x1d0;
    pub const SPIN: usize = 0x2a0;
    pub const CHOICES: usize = 0x2b0;
    pub const LENGTHS: usize = 0x2f0;
    pub const FIRST: usize = 0x320;
    pub const PATH: usize = 0x324;
    pub const FOUND: usize = 0x328;
    pub const SPEED: usize = 0x32c;
    pub const LINKS: usize = 0x330;
    pub const TAIL: usize = 0x358;
    pub const SUM: usize = 0x360;
    pub const FALLEN: usize = 0x364;
    pub const BOMBS: usize = 0x368;
    pub const AREA: usize = 0x36c;
    pub const POINT: usize = 0x370;
    pub const BOMB: usize = 0x374;
    pub const BOMB_WAIT: usize = 0x376;
    pub const LEFT: usize = 0x378;
    pub const TOGGLE: usize = 0x37a;
    pub const STATES: usize = 0x380;
    pub const HUM: usize = 0x38c;
}

/// `$gp+0x2230`: the links' lengths at a start (the eleventh 2).
const CHAIN_LENGTHS: [f32; 12] = [2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.5, 2.0, 2.5];

fn link(w: &MobyWorld, id: usize, i: usize) -> Option<usize> { moby_at(w, w.mobys[id].pv_i(d4a::LINKS + 4 * i)) }
fn tail(w: &MobyWorld, id: usize) -> Option<usize> { moby_at(w, w.mobys[id].pv_i(d4a::TAIL)) }
fn attached(w: &MobyWorld, id: usize, i: usize) -> bool { w.mobys[id].pv_b(d4a::STATES + i) == 0 }
fn point(w: &MobyWorld, id: usize, i: usize) -> V3 { w.mobys[id].pv_v(d4a::POINTS + 16 * i) }

fn stop_hum(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: usize) {
    let h = w.mobys[id].pv_i(at);
    if h != -1 { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(at, -1);
}

fn keep_hum(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: usize, sound: i32) {
    if !ctx.host.sound_playing(w.mobys[id].pv_i(at)) {
        let h = ctx.host.play_sound(&w.mobys[id], sound, 4);
        w.mobys[id].set_pv_i(at, h);
    }
}

/// `0x43b190`: sent — its life, shown with its collision, ten links, the links and the tail shown in its colour (the
/// links' set-up `0x43c218`), the lengths back.
pub fn start_d4a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use d4a::*;
    let coll = w.class(D4A).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.set_pv_b(0x2e, 1);
    let life = m.pv_s(0x24) as f32;
    m.set_pv_f(0x20, life);
    m.drawn = 1;
    m.mode &= !1;
    m.has_collision = coll;
    m.mode = (m.mode | mode::TARGET) & 0xffb9;
    m.drops = 0;
    m.set_pv_s(LEFT, 10);
    m.set_state(2, 0);
    set_seq(w, id, ctx, 0, 0, 0);
    w.mobys[id].set_pv_s(BOMB, -1);
    let amb = w.mobys[id].ambient;
    for i in 0..10 {
        w.mobys[id].set_pv_b(STATES + i, 0);
        if let Some(k) = link(w, id, i) {
            set_seq(w, k, ctx, 1, 0, 0);
            attach_d4b(w, k, ctx);
            w.mobys[k].ambient = amb;
        }
    }
    for (i, &l) in CHAIN_LENGTHS.iter().enumerate() { w.mobys[id].set_pv_f(LENGTHS_AT + 4 * i, l); }
    if let Some(t) = tail(w, id) {
        set_seq(w, t, ctx, 1, 0, 0);
        let tc = w.class(w.mobys[t].o_class).is_some_and(|c| c.has_collision);
        let x = &mut w.mobys[t];
        x.mode &= !(2 | 4 | 0x40 | 1);
        x.mode |= mode::TARGET;
        x.drawn = 1;
        x.has_collision = tc;
    }
    super::fighters::flash_reset(w, id);
    w.changed = true;
}

const LENGTHS_AT: usize = d4a::LENGTHS;

/// `0x43b430`: back to the pool — hidden, its hum stopped, the tail hidden.
pub fn pool_d4a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_state(1, 0);
    m.mode = (m.mode & !mode::TARGET) | 0x47;
    m.drawn = 0;
    m.has_collision = false;
    stop_hum(w, id, ctx, d4a::HUM);
    if let Some(t) = tail(w, id) {
        let x = &mut w.mobys[t];
        x.mode |= 2 | 4 | 0x40 | 1;
        x.mode &= !mode::TARGET;
        x.drawn = 0;
        x.has_collision = false;
    }
    w.changed = true;
}

/// `0x43ae80(m, p)`: a link's or the chain's blast (its sound 0x0df7's 0; the sparks and rings not ported).
fn pop(w: &MobyWorld, id: usize, ctx: &mut Ctx, _p: V3) { ctx.host.play_sound_of(&w.mobys[id], DF7, 0, 0); }

/// `0x43a758`: the next path — state 2 its first, 3 / 4 one of four from the rows (toggled); its nearest point.
fn pick_path(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use d4a::*;
    let t = w.mobys[id].pv_s(TOGGLE) ^ 1;
    w.mobys[id].set_pv_s(TOGGLE, t);
    let row = match w.mobys[id].state { 3 => Some(if t != 0 { 0 } else { 1 }), 4 => Some(if t != 0 { 2 } else { 3 }), _ => None };
    let p = match row {
        Some(r) => { let k = ctx.game.rng.randi(4) as usize; w.mobys[id].pv_i(CHOICES + 0x10 * r + 4 * k) }
        None => w.mobys[id].pv_i(FIRST),
    };
    w.mobys[id].set_pv_i(PATH, p + 1);
    let pts = usize::try_from(p).ok().and_then(|i| ctx.level.paths.get(i)).cloned().unwrap_or_default();
    let pos = w.mobys[id].pos;
    let mut best = 0;
    let mut near = pts.first().map_or(0.0, |q| dist2(pos, [q[0], q[1], q[2]]));
    for (k, q) in pts.iter().enumerate().take(pts.len().saturating_sub(1)) {
        let d = dist2(pos, [q[0], q[1], q[2]]);
        if d < near { near = d; best = k; }
    }
    w.mobys[id].set_pv_s(POINT, best as i16);
}

fn path_points(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<Vec<[f32; 4]>> {
    let p = w.mobys[id].pv_i(d4a::PATH) - 1;
    usize::try_from(p).ok().and_then(|i| ctx.level.paths.get(i)).filter(|v| !v.is_empty()).cloned()
}

/// `0x439b60(m, way, a, b, c)`: its angles eased toward those of a basis along `way` (`0x31f230`).
fn ease_to(w: &mut MobyWorld, id: usize, way: V3) {
    let target = crate::moby::attach::angles_of(&basis(way));
    let m = &mut w.mobys[id];
    for k in 0..3 {
        let (mut a, mut v) = (m.rot[k], m.pv_f(d4a::SPIN + 4 * k));
        let d = crate::hero::physics::sub_rot(target[k], a);
        let mut x = 0.0;
        turn_ease(d, 0.006_981_318, 0.006_981_318, 0.418_879_1, &mut x, &mut v, 0);
        a = add_rot(a, x);
        m.rot[k] = a;
        m.set_pv_f(d4a::SPIN + 4 * k, v);
    }
    m.rows = crate::moby::attach::rows_of(m.rot);
}

/// `0x43a910`: along its path at `lerp(15, 20, (links − 2)/8)` a second (its speed eased by 1/180), its hum on, facing
/// its target while it is within 45° of its way (else its way); the next point within 4; true at the path's end (or
/// without one; its hum stopped).
fn follow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    use d4a::*;
    let Some(pts) = path_points(w, id, ctx) else {
        stop_hum(w, id, ctx, HUM);
        return true;
    };
    keep_hum(w, id, ctx, HUM, 0);
    let k = (w.mobys[id].pv_s(POINT).max(0) as usize).min(pts.len() - 1);
    let goal = [pts[k][0], pts[k][1], pts[k][2]];
    let t = ((w.mobys[id].pv_s(LEFT) as f32 - 2.0) / 8.0).clamp(0.0, 1.0);
    let top = (15.0 + 5.0 * t) / 60.0;
    let mut s = w.mobys[id].pv_f(SPEED);
    approach(top, 1.0 / 180.0, &mut s);
    w.mobys[id].set_pv_f(SPEED, s);
    let m = &mut w.mobys[id];
    let dir = set_len(sub(goal, m.pos), s);
    m.pos = add(m.pos, dir);
    let to = set_len(sub(m.pv_v(TARGET), m.pos), 1.0);
    let way = set_len(dir, 1.0);
    ease_to(w, id, if 0.707 < dot(way, to) { to } else { way });
    if dist(w.mobys[id].pos, goal) < 4.0 {
        let v = w.mobys[id].pv_s(POINT) + 1;
        w.mobys[id].set_pv_s(POINT, v);
    }
    if (w.mobys[id].pv_s(POINT) as i32) < pts.len() as i32 - 1 { return false; }
    w.mobys[id].set_pv_i(PATH, 0);
    stop_hum(w, id, ctx, HUM);
    true
}

/// `0x439a88`: the chain laid out behind it along its first row (each point its length on).
fn lay_chain(w: &mut MobyWorld, id: usize) {
    use d4a::*;
    let m = &mut w.mobys[id];
    m.rows = crate::moby::attach::rows_of(m.rot);
    let mut p = m.pos;
    let back = scale(m.rows[0], -1.0);
    m.set_pv_v(POINTS, p);
    m.set_pv_v(OLD, p);
    for i in 0..12 {
        p = add(p, set_len(back, m.pv_f(LENGTHS + 4 * i)));
        m.set_pv_v(POINTS + 16 * (i + 1), p);
        m.set_pv_v(OLD + 16 * (i + 1), p);
    }
}

/// The chain's tick (`0x43ae08`, `0x4396e8`, `0x439810`): a falling link's lengths shrink 2 a second (to gone); the
/// points' Verlet step (gravity 1/720, the damping `lerp(0.97, 0.98, (links − 2)/8)`); four passes of the lengths
/// (the head at its place, the points kept above the middle's height past state 2).
fn chain(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    use d4a::*;
    for i in 0..10 {
        if w.mobys[id].pv_b(STATES + i) == 1 {
            let l = w.mobys[id].pv_f(LENGTHS + 4 * i) - 2.0 / 60.0;
            if l < 0.0 {
                w.mobys[id].set_pv_f(LENGTHS + 4 * i, 0.0);
                w.mobys[id].set_pv_b(STATES + i, 2);
            } else {
                w.mobys[id].set_pv_f(LENGTHS + 4 * i, l);
            }
        }
    }
    let t = ((w.mobys[id].pv_s(LEFT) as f32 - 2.0) / 8.0).clamp(0.0, 1.0);
    let damp = 0.97 + 0.01 * t;
    let m = &mut w.mobys[id];
    for i in 0..13 {
        let (p, o) = (m.pv_v(POINTS + 16 * i), m.pv_v(OLD + 16 * i));
        let n = add(add(p, scale(sub(p, o), damp)), [0.0, 0.0, -1.0 / 720.0]);
        m.set_pv_v(POINTS + 16 * i, n);
        m.set_pv_v(OLD + 16 * i, p);
    }
    let floor = middle(w, id, ctx)[2];
    let low = 3 <= w.mobys[id].state;
    let m = &mut w.mobys[id];
    for pass in 0..4 {
        m.set_pv_v(POINTS, m.pos);
        let order: Vec<usize> = if pass % 2 == 0 { (0..12).collect() } else { (0..12).rev().collect() };
        for i in order {
            let (a, b) = if pass % 2 == 0 { (i, i + 1) } else { (i + 1, i) };
            let mut pa = m.pv_v(POINTS + 16 * a);
            if low && pa[2] < floor {
                pa[2] = floor;
                let mut o = m.pv_v(OLD + 16 * a);
                o[2] = floor;
                m.set_pv_v(OLD + 16 * a, o);
            }
            let pb = m.pv_v(POINTS + 16 * b);
            let d = sub(pb, pa);
            let l = m.pv_f(LENGTHS + 4 * i);
            let k = l / (len(d) + l) - 0.5;
            let step = scale(d, k);
            m.set_pv_v(POINTS + 16 * a, sub(pa, step));
            m.set_pv_v(POINTS + 16 * b, add(pb, step));
        }
    }
    m.set_pv_v(POINTS, m.pos);
}

/// `0x439ca0` / `0x439ef8`: the links on (at the points 1..10, their rows along the chain) and the tail at point 11.
fn place_links(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    for i in 0..10 {
        if !attached(w, id, i) { continue; }
        let Some(k) = link(w, id, i) else { continue };
        let (a, b) = (point(w, id, i), point(w, id, i + 1));
        let x = &mut w.mobys[k];
        x.pos = b;
        x.rows = basis(sub(a, b));
        x.rot = crate::moby::attach::angles_of(&x.rows);
        x.mode |= mode::KEEP_ROWS;
        w.rebuild(k, &*ctx.host);
    }
    if let Some(t) = tail(w, id) {
        let (a, b) = (point(w, id, 10), point(w, id, 11));
        let x = &mut w.mobys[t];
        x.pos = b;
        x.rows = basis(sub(a, b));
        x.rot = crate::moby::attach::angles_of(&x.rows);
        x.mode |= mode::KEEP_ROWS;
        w.rebuild(t, &*ctx.host);
    }
}

/// `0x43ac18(m, i, owner)`: link `i` falls off — thrown toward the middle (`randi(10) + 15` a second across, up
/// `randi(5) + 5`), its spin by how few links are left; its owner the master's, the master's boss word it (or none);
/// the link falling.
fn fall_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, i: usize, owner: bool) {
    let Some(k) = link(w, id, i) else { return };
    let mid = middle(w, id, ctx);
    let s = (ctx.game.rng.randi(10) + 15) as f32 / 60.0;
    let mut v = scale(sub(mid, w.mobys[k].pos), s);
    for c in 0..3 { v[c] += ctx.game.rng.randf(-1.0 / 60.0, 1.0 / 60.0); }
    v[2] = (ctx.game.rng.randi(5) + 5) as f32 / 60.0;
    let t = (1.0 - (w.mobys[id].pv_s(d4a::LEFT) as f32 - 4.0) / 6.0).clamp(0.0, 1.0);
    fly_d4b(w, k, ctx, v, 20.0 / 3600.0, t);
    w.mobys[k].parent = w.mobys[id].parent;
    w.mobys[id].set_pv_i(d4a::FALLEN, k as i32);
    set_owner(w, id, if owner { Some(k) } else { None });
    w.mobys[id].set_pv_b(d4a::STATES + i, 1);
}

/// `0x438df8`: the set-up (no life: 30), the hits on it and its links (each its damage, pushing the chain; with four
/// links or more the damage summed: past 10 a link falls off and the sum comes off its life), dying (6), its links
/// left to fall when it is told to go, its target (255 out), its shadow (within 48).
fn senses_d4a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use d4a::*;
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 4);
        m.set_pv_i(HUM, -1);
        if m.pv_s(0x24) == 0 {
            m.set_pv_s(0x24, 30);
            m.set_pv_f(0x20, 30.0);
        }
        pool_d4a(w, id, ctx);
    }
    if 3 <= w.mobys[id].state && moby_at(w, w.mobys[id].pv_i(FALLEN)).is_none() {
        let mut consumed = false;
        for i in 0..10 {
            if !attached(w, id, i) { continue; }
            let Some(k) = link(w, id, i) else { continue };
            let hit = w.hit_message(k).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
            let dmg = ai::damage_of(w, id, hit.as_ref(), false);
            w.clear_hit(k);
            if dmg != 0.0 {
                let h = hit.expect("damage without a hit");
                if 3.0 < h.tmpl.damage {
                    let k = (dmg * 0.125).clamp(0.0, 1.0);
                    let o = POINTS + 0x10 + 16 * i;
                    let v = scale([h.tmpl.dir[0], h.tmpl.dir[1], h.tmpl.dir[2]], k * 0.277_777_8);
                    let p = w.mobys[id].pv_v(o);
                    w.mobys[id].set_pv_v(o, add(p, v));
                    w.mobys[id].set_state(5, -1);
                }
            }
            if 4 <= w.mobys[id].pv_s(LEFT) {
                let sum = w.mobys[id].pv_f(SUM) + dmg;
                w.mobys[id].set_pv_f(SUM, sum);
                if !consumed && 10.0 < sum {
                    let m = &mut w.mobys[id];
                    m.set_pv_f(0x20, m.pv_f(0x20) - sum);
                    m.set_pv_s(LEFT, m.pv_s(LEFT) - 1);
                    m.set_pv_f(SUM, 0.0);
                    consumed = true;
                    let p = w.mobys[k].pos;
                    pop(w, id, ctx, p);
                    fall_off(w, id, ctx, i, true);
                    w.mobys[id].set_state(5, -1);
                }
            } else {
                let m = &mut w.mobys[id];
                m.set_pv_f(0x20, m.pv_f(0x20) - dmg);
            }
        }
        let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
        let dmg = ai::damage_of(w, id, hit.as_ref(), false);
        w.clear_hit(id);
        if dmg != 0.0 {
            let h = hit.expect("damage without a hit");
            ai::flash_start(&mut w.mobys[id], 0x70);
            if 3.0 < h.tmpl.damage {
                let k = (dmg * 0.125).clamp(0.0, 1.0);
                let m = &mut w.mobys[id];
                m.pos = add(m.pos, scale([h.tmpl.dir[0], h.tmpl.dir[1], h.tmpl.dir[2]], k * 0.555_555_6));
            }
            let m = &mut w.mobys[id];
            if 4 <= m.pv_s(LEFT) { m.set_pv_f(SUM, m.pv_f(SUM) + dmg); } else { m.set_pv_f(0x20, m.pv_f(0x20) - dmg); }
        }
        let s = w.mobys[id].state;
        if !matches!(s, 6 | 1 | 7 | 0) && w.mobys[id].pv_f(0x20) <= 0.0 { w.mobys[id].set_state(6, 2); }
    } else {
        w.clear_hit(id);
        for i in 0..10 {
            if let Some(k) = link(w, id, i).filter(|_| attached(w, id, i)) { w.clear_hit(k); }
        }
        w.mobys[id].set_pv_f(SUM, 0.0);
    }
    if let Some(f) = moby_at(w, w.mobys[id].pv_i(FALLEN)) {
        if w.mobys[f].state < 2 {
            w.mobys[id].set_pv_i(FALLEN, -1);
            set_owner(w, id, Some(id));
        }
    } else if w.mobys[id].pv_i(FALLEN) != -1 && w.mobys[id].pv_i(FALLEN) != 0 {
        w.mobys[id].set_pv_i(FALLEN, -1);
    }
    if 2 <= w.mobys[id].state && w.mobys[id].pv_b(0x2e) == 2 {
        w.mobys[id].set_pv_f(0x20, 0.0);
        pool_d4a(w, id, ctx);
        let p0 = point(w, id, 0);
        pop(w, id, ctx, p0);
        for i in 0..10 {
            if !attached(w, id, i) { continue; }
            let Some(k) = link(w, id, i) else { continue };
            let v = ctx.game.rng.rand_vec(1.0 / 6.0, 1.0 / 3.0);
            crate::moby::shards::break_into(w, k, ctx, (3.0, 6.0), (3.0, 6.0), 3, v, 0xff);
            let p = point(w, id, i + 1);
            pop(w, id, ctx, p);
            pool_d4b(w, k, ctx);
        }
    }
    let a = w.mobys[id].pv_i(AREA);
    let f = ai::find_target(w, id, ctx, 255.0, 255.0, a, None, false);
    w.mobys[id].set_pv_i(FOUND, f as i32);
    if w.mobys[id].drawn != 0 && dist(w.mobys[id].pos, ctx.host.camera_pos()) < 48.0 {
        crate::shadow_volumes::probe_near(w, id, ctx, 48.0, 45);
    }
}

/// `0x43a528`: the bombs — every 15 ticks the next link on drops a bomb 0x0e20 (the first pooled of the group
/// `+0x368`) from 0.3 under it toward the target a third of the way a tick (`randvec(1/60, 1/20)`, falling 1/12).
fn drop_bomb(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use d4a::*;
    let mut i = w.mobys[id].pv_s(BOMB);
    if i == -1 { return; }
    while (0..10).contains(&i) && !attached(w, id, i as usize) { i += 1; }
    if 10 <= i {
        w.mobys[id].set_pv_s(BOMB, -1);
        return;
    }
    w.mobys[id].set_pv_s(BOMB, i);
    let mut wait = w.mobys[id].pv_s(BOMB_WAIT) as i32;
    let over = crate::moby::countdown(&mut wait);
    w.mobys[id].set_pv_s(BOMB_WAIT, wait as i16);
    if !over { return; }
    let Some(k) = link(w, id, i as usize) else { return };
    let (p, rows) = match ctx.host.joint_matrix(&w.mobys[k], 0) { Some((r, t)) => (t, r), None => (w.mobys[k].pos, w.mobys[k].rows) };
    let at = add(p, set_len(rows[2], -0.3));
    let mut v = ctx.game.rng.rand_vec(1.0 / 60.0, 0.05);
    v[2] = -1.0 / 12.0;
    let v = add(v, scale(sub(w.mobys[id].pv_v(TARGET), at), 1.0 / 3.0));
    let g = w.mobys[id].pv_i(BOMBS);
    let bomb = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).and_then(|l| l.iter().copied().find(|&b| w.mobys[b].state == 1));
    if let Some(b) = bomb {
        let target = ai::link(&w.mobys[id], 0x60);
        let dmg = w.mobys[id].pv_b(0x35) as f32;
        let ms = w.mobys[id].parent;
        throw_e20(w, b, ctx, k, ms, target, at, v, 120, 1.0 / 360.0, dmg);
    }
    w.mobys[id].set_pv_s(BOMB, i + 1);
    w.mobys[id].set_pv_s(BOMB_WAIT, 15);
}

/// 0x0d4a (`0x4387c0`): a flying chain of ten links (0x0d4b) and a tail, its points a rope (Verlet, [`chain`]). Sent,
/// it starts at its first path's start (2) and flies its paths (3 with a link fallen off, else 4: then its links drop
/// bombs, [`drop_bomb`]), its hum on, facing its target. Hit hard it flinches (5, sequence 4). With four links or more
/// its damage is summed: past 10 a link falls off ([`fall_off`]) and fights on its own (crate::moby::classes::level11::
/// bosses' [`update_d4b`]), the sum off its life. Dead (6, after 10 ticks): it bursts — four of its links fall off,
/// the rest and the tail burst — its bolts, back to the pool.
pub fn update_d4a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use d4a::*;
    senses_d4a(w, id, ctx);
    if w.mobys[id].sub_state == 1 { keep_hum(w, id, ctx, HUM, 0); }
    let next = |w: &MobyWorld| if moby_at(w, w.mobys[id].pv_i(FALLEN)).is_some() { 3 } else { 4 };
    match w.mobys[id].state {
        2 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 0, 0, 10);
                pick_path(w, id, ctx);
                if let Some(p) = path_points(w, id, ctx) {
                    let m = &mut w.mobys[id];
                    m.pos = [p[0][0], p[0][1], p[0][2]];
                    if let Some(q) = p.get(1) { m.rot[2] = atan2(q[0] - m.pos[0], q[1] - m.pos[1]); }
                    m.rows = crate::moby::attach::rows_of(m.rot);
                }
                lay_chain(w, id);
            } else if follow(w, id, ctx) {
                w.mobys[id].set_state(3, -1);
            }
        }
        3 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 0, 0, 10);
                pick_path(w, id, ctx);
            } else if follow(w, id, ctx) {
                let n = next(w);
                w.mobys[id].set_state(n, -1);
            }
        }
        4 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 0, 0, 10);
                pick_path(w, id, ctx);
            } else {
                if follow(w, id, ctx) {
                    let n = next(w);
                    w.mobys[id].set_state(n, -1);
                }
                if w.mobys[id].drawn != 0 {
                    if w.mobys[id].pv_s(BOMB) == -1 {
                        w.mobys[id].set_pv_s(BOMB, 0);
                        w.mobys[id].set_pv_s(BOMB_WAIT, 0);
                    }
                    drop_bomb(w, id, ctx);
                }
            }
        }
        5 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 4, 0, 10);
                stop_hum(w, id, ctx, HUM);
            }
            if wrapped(w, id, ctx) { set_seq(w, id, ctx, 0, 0, 7); }
            if follow(w, id, ctx) {
                let n = next(w);
                w.mobys[id].set_state(n, -1);
            }
        }
        6 => {
            if w.mobys[id].first() {
                w.mobys[id].state_ticks = 0;
                set_seq(w, id, ctx, 4, 0, 10);
            }
            if 10 < w.mobys[id].state_ticks as u16 {
                let hero = ctx.hero.pos;
                let dir = scale(sub(hero, w.mobys[id].pos), 0.5);
                let p0 = point(w, id, 0);
                pop(w, id, ctx, p0);
                let v = add(dir, ctx.game.rng.rand_vec(1.0 / 30.0, 1.0 / 15.0));
                crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 1, v, 0xff);
                let mut count = 0;
                for i in 0..10 {
                    if !attached(w, id, i) { continue; }
                    let Some(k) = link(w, id, i) else { continue };
                    if 4 <= count {
                        let v = add(dir, ctx.game.rng.rand_vec(1.0 / 6.0, 1.0 / 3.0));
                        crate::moby::shards::break_into(w, k, ctx, (3.0, 6.0), (3.0, 6.0), 3, v, 0xff);
                        let p = point(w, id, i);
                        pop(w, id, ctx, p);
                        pool_d4b(w, k, ctx);
                    } else {
                        let v = w.mobys[id].pv_s(LEFT) - 1;
                        w.mobys[id].set_pv_s(LEFT, v);
                        count += 1;
                        let p = w.mobys[k].pos;
                        pop(w, id, ctx, p);
                        fall_off(w, id, ctx, i, false);
                    }
                }
                if let Some(t) = tail(w, id) {
                    let v = add(dir, ctx.game.rng.rand_vec(1.0 / 3.0, 0.5));
                    crate::moby::shards::break_into(w, t, ctx, (3.0, 6.0), (3.0, 6.0), 7, v, 0xff);
                }
                bolts(w, id, ctx);
                pool_d4a(w, id, ctx);
            }
        }
        _ => {}
    }
    // `0x439530`: the chain, the links and the tail on; the ticks; the head (`+0x80`, list 1, 2.5).
    if 2 <= w.mobys[id].state {
        chain(w, id, ctx);
        place_links(w, id, ctx);
    }
    tick(w, id);
    crate::moby::joint_aim::head(w, ctx, id, 0x80, 1, 2.5);
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0d4b

/// Its words: health `+0x20`, the target block `+0x50`, its movement `+0x80`, react `+0x170`, the fall's velocity
/// `+0x2a0` (z `+0x2a8`) and gravity `+0x2ac`, the spin `+0x2b0`, the spot `+0x2c0`, the bolt's wait `+0x2d0`, yaw
/// offset `+0x2d4`, found `+0x2d8`, its aim's sway `+0x2dc` (`+0x2ee` its way), the sway's speed `+0x2e0`, its
/// attack's playback speed `+0x2e4`, area `+0x2e8`.
mod d4b {
    pub const VEL: usize = 0x2a0;
    pub const GRAVITY: usize = 0x2ac;
    pub const SPIN: usize = 0x2b0;
    pub const SPOT: usize = 0x2c0;
    pub const WAIT: usize = 0x2d0;
    pub const OFFSET: usize = 0x2d4;
    pub const FOUND: usize = 0x2d8;
    pub const SWAY: usize = 0x2dc;
    pub const SWAY_SPEED: usize = 0x2e0;
    pub const PLAYBACK: usize = 0x2e4;
    pub const AREA: usize = 0x2e8;
}

/// `$gp+0x2318..+0x2340`: its reach 20, its spot 6 (±2, ±45°) from the target, the bolt at frame 5, the walk's
/// playback speed 3, the spin's turns (up to 4), its height 0.8, the fall's pivot (1, 0, 0) in its rows; the sway
/// `$gp+0x2328` (15°).
const D4B_REACH: f32 = 20.0;
const D4B_SPOT: f32 = 6.0;
const D4B_FIRE: f32 = 5.0;
const D4B_PLAY: f32 = 3.0;
const D4B_TURNS: i32 = 4;
const D4B_HEIGHT: f32 = 0.8;
const D4B_SWAY: f32 = 0.261_799_4;

/// `0x43c218`: a link on the chain — its life, shown with its collision, a target, sequence 1, its yaw offset and
/// sway.
fn attach_d4b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let coll = w.class(D4B).is_some_and(|c| c.has_collision);
    let a = ctx.game.rng.randf(10.0, 30.0).to_radians();
    let neg = ctx.game.rng.randi(2) != 0;
    let sneg = ctx.game.rng.randi(2) != 0;
    let m = &mut w.mobys[id];
    m.set_pv_b(0x2e, 3);
    let life = m.pv_s(0x24) as f32;
    m.set_pv_f(0x20, life);
    m.set_pv_s(0x1a8, 0);
    m.set_pv_b(0x18a, 0);
    m.drawn = 1;
    m.mode &= !1;
    m.has_collision = coll;
    m.mode = (m.mode | mode::TARGET) & 0xffb9;
    m.drops = 0;
    m.set_pv_f(d4b::OFFSET, if neg { -a } else { a });
    m.set_pv_s(0x2ee, 0);
    m.set_pv_f(d4b::SWAY, if sneg { -D4B_SWAY } else { D4B_SWAY });
    set_seq(w, id, ctx, 1, 0, 0);
}

/// `0x43bec0(m, vel, gravity, t)`: off the chain — shown, falling (3) at `vel`, at random angles, its yaw offset
/// ±20..40°, its sway's speed `lerp(30, 90, t)`″ a tick and its attack's playback `lerp(2, 4, t)`.
fn fly_d4b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, vel: V3, gravity: f32, t: f32) {
    let r = [0; 3].map(|_| ctx.game.rng.randf(0.0, std::f32::consts::PI));
    let a = ctx.game.rng.randf(20.0, 40.0).to_radians();
    let neg = ctx.game.rng.randi(2) != 0;
    w.clear_hit(id);
    let m = &mut w.mobys[id];
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.set_state(3, 0);
    let life = m.pv_s(0x24) as f32;
    m.set_pv_f(0x20, life);
    m.set_pv_b(0x2e, 1);
    m.set_pv_v(d4b::VEL, vel);
    m.set_pv_f(d4b::GRAVITY, gravity);
    m.set_pv_s(0x2ec, 0);
    m.rot = r;
    m.rows = crate::moby::attach::rows_of(r);
    m.set_pv_f(d4b::OFFSET, if neg { -a } else { a });
    m.mode &= 0xfeff;
    m.set_pv_f(d4b::SWAY_SPEED, 30.0 + 60.0 * t);
    m.set_pv_i(d4b::PLAYBACK, (2.0 + 2.0 * t) as i32);
}

/// `0x43c368`: back to the pool — in view its pieces, its sound 1, a blast (1.0) and 0x0df7's sound 0; hidden.
pub fn pool_d4b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 { return; }
    if w.mobys[id].drawn != 0 {
        let mut v = ctx.game.rng.rand_vec(1.0 / 6.0, 1.0 / 3.0);
        v[2] = ctx.game.rng.randf(1.0 / 12.0, 1.0 / 6.0);
        crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 3, v, 0xff);
        ctx.host.play_sound(&w.mobys[id], 1, 0);
        let pos = w.mobys[id].pos;
        crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size: 1.0, bursts: [0x19, 10, 10], puffs: 5, streaks: 0, light: true, shake: true });
        ctx.host.play_sound_of(&w.mobys[id], DF7, 0, 0);
    }
    w.mobys[id].set_state(1, 0);
    ctx.host.set_anim(&w.mobys[id], 0, 0, 3);
    let m = &mut w.mobys[id];
    m.drawn = 0;
    m.has_collision = false;
    m.mode = (m.mode & !mode::TARGET) | 0x47;
    w.changed = true;
}

/// `0x43bc38`: the set-up (no life: 10), its damage past state 3, its target (255 out), its shadow (within 48).
fn senses_d4b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 12);
        m.set_pv_f(0x1a0, 1.0);
        m.set_pv_b(0x180, 6);
        m.set_pv_i(0x1b8, 2);
        m.set_pv_f(0x1ac, 0.15);
        m.set_pv_f(0x80, 1.0);
        m.set_pv_f(0x84, 0.4);
        m.set_pv_f(0x88, 0.3);
        m.set_pv_f(0x8c, 0.3);
        m.set_pv_f(0x90, std::f32::consts::FRAC_PI_2);
        m.set_pv_f(0xb0, 0.523_598_85);
        m.set_pv_f(0xb4, 2.0);
        m.set_pv_f(0xac, 1.0 / 360.0);
        m.set_pv_i(0x1a4, 0);
        lim(w, id, 0.277_777_8, 0.333_333_34, 0.001_745_329_5, 0.052_359_886);
        let m = &mut w.mobys[id];
        if m.pv_s(0x24) == 0 {
            m.set_pv_s(0x24, 10);
            m.set_pv_f(0x20, 10.0);
        }
        set_seq(w, id, ctx, 1, 0, 0);
        w.mobys[id].set_state(1, -1);
        pool_d4b(w, id, ctx);
    }
    if 4 <= w.mobys[id].state { ai::take_damage(w, id, ctx, false); }
    if 3 <= w.mobys[id].state && w.mobys[id].pv_b(0x2e) == 2 { pool_d4b(w, id, ctx); }
    let a = w.mobys[id].pv_i(d4b::AREA);
    let f = ai::find_target(w, id, ctx, 255.0, 255.0, a, None, false);
    w.mobys[id].set_pv_i(d4b::FOUND, f as i32);
    if w.mobys[id].drawn != 0 && dist(w.mobys[id].pos, ctx.host.camera_pos()) < 48.0 {
        crate::shadow_volumes::probe_near(w, id, ctx, 48.0, 45);
    }
}

/// 0x0d4b (`0x43b610`): a link of 0x0d4a's chain; on it (state 1) the chain moves it and it hits what it touches.
/// Off it: it falls (3) spinning to the middle's height (its spins whole turns over the fall, about `(1, 0, 0)` in
/// its rows), walks to a spot 6 from its target (4, at its playback speed 3, 0.8 under its place), and fires (5:
/// sequence 3, then 4 at its playback speed: at frame 5 a bolt 0x0e2d from its joint 1, [`fire_e2d`]; past 20 from
/// its target back to the walk). Knocked (6), dead: its sound 3 and back to the pool.
pub fn update_d4b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use d4b::*;
    senses_d4b(w, id, ctx);
    let t = w.mobys[id].pv_v(0x50);
    let pos = w.mobys[id].pos;
    let d = dist2(pos, t);
    let yaw = atan2(t[0] - pos[0], t[1] - pos[1]);
    let floor = middle(w, id, ctx)[2];
    match w.mobys[id].state {
        3 => {
            let h = w.mobys[id].pos[2] - D4B_HEIGHT;
            if w.mobys[id].first() {
                let m = &w.mobys[id];
                let (vz, g) = (m.pv_f(VEL + 8), m.pv_f(GRAVITY));
                let frames = if 0.0 < g { (vz + (vz * vz + 2.0 * g * (h - floor)).max(0.0).sqrt()) / g } else { 60.0 };
                let tau = std::f32::consts::TAU;
                let s = [0; 3].map(|_| (ctx.game.rng.randi(D4B_TURNS) + 1) as f32 * tau / frames.max(1.0));
                let m = &mut w.mobys[id];
                m.rot = [0.0, 0.0, yaw];
                m.set_pv_v(SPIN, s);
            }
            if floor <= h {
                let m = &mut w.mobys[id];
                m.pos = add(m.pos, m.pv_v(VEL));
                m.set_pv_f(VEL + 8, m.pv_f(VEL + 8) - m.pv_f(GRAVITY));
                let before = crate::gravity::to_world([1.0, 0.0, 0.0], &m.rows);
                let s = m.pv_v(SPIN);
                for k in 0..3 { m.rot[k] = add_rot(s[k], m.rot[k]); }
                m.rows = crate::moby::attach::rows_of(m.rot);
                let after = crate::gravity::to_world([1.0, 0.0, 0.0], &m.rows);
                m.pos = add(m.pos, sub(before, after));
            } else {
                let m = &mut w.mobys[id];
                m.rot[0] = 0.0;
                m.rot[1] = 0.0;
                m.rows = crate::moby::attach::rows_of(m.rot);
                m.set_state(4, -1);
            }
        }
        4 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 7, 0, 3);
                ctx.host.set_anim_speed(&w.mobys[id], D4B_PLAY);
                let a = atan2(pos[0] - t[0], pos[1] - t[1]) + ctx.game.rng.randf(-45f32.to_radians(), 45f32.to_radians());
                let r = D4B_SPOT + ctx.game.rng.randf(-2.0, 2.0);
                w.mobys[id].set_pv_v(SPOT, [t[0] + r * a.cos(), t[1] + r * a.sin(), t[2]]);
            }
            let spot = w.mobys[id].pv_v(SPOT);
            w.mobys[id].pos[2] -= D4B_HEIGHT;
            let off = w.mobys[id].pv_f(OFFSET);
            ai::move_toward(w, id, ctx, off, spot);
            w.mobys[id].pos[2] += D4B_HEIGHT;
            let p = w.mobys[id].pos;
            if (p[0] - spot[0]).powi(2) + (p[1] - spot[1]).powi(2) < 4.84 && w.mobys[id].drawn != 0 {
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                w.mobys[id].set_state(5, -1);
                tick(w, id);
                return;
            }
            if D4B_REACH * D4B_REACH < (spot[0] - t[0]).powi(2) + (spot[1] - t[1]).powi(2) {
                w.mobys[id].set_state(4, -1);
                tick(w, id);
                return;
            }
        }
        5 => {
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                set_seq(w, id, ctx, 3, 0, 3);
                w.mobys[id].set_pv_i(WAIT, 0);
            } else if w.mobys[id].timer == 0 && wrapped(w, id, ctx) {
                w.mobys[id].timer = 1;
                set_seq(w, id, ctx, 4, 0, 7);
                let s = w.mobys[id].pv_i(PLAYBACK) as f32;
                ctx.host.set_anim_speed(&w.mobys[id], s);
            } else if w.mobys[id].timer == 3 && wrapped(w, id, ctx) {
                w.mobys[id].set_state(4, -1);
            }
            if w.mobys[id].timer == 1 {
                let ft = ctx.host.frame_time(&w.mobys[id]);
                ctx.host.play_sound(&w.mobys[id], 2, 0);
                if w.mobys[id].pv_i(WAIT) == 0 && ft < D4B_FIRE + 1.0 {
                    w.mobys[id].set_pv_i(WAIT, D4B_FIRE as i32);
                    if let Some((r, p)) = ctx.host.joint_matrix(&w.mobys[id], 1) {
                        let dmg = w.mobys[id].pv_b(0x34) as f32;
                        fire_e2d(w, id, ctx, p, r[0], dmg);
                        ctx.host.play_sound_of(&w.mobys[id], D4B, 0, 0);
                    }
                }
                let mut k = w.mobys[id].pv_i(WAIT);
                crate::moby::countdown(&mut k);
                w.mobys[id].set_pv_i(WAIT, k);
                if D4B_REACH < d {
                    w.mobys[id].timer = 3;
                    ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                    set_seq(w, id, ctx, 5, 0, 3);
                }
            }
            w.mobys[id].pos[2] -= D4B_HEIGHT;
            ai::hold(w, id, ctx);
            w.mobys[id].pos[2] += D4B_HEIGHT;
        }
        6 => {
            if w.mobys[id].first() { ctx.host.play_sound_of(&w.mobys[id], D4B, 2, 0); }
            let f = ai::flight(w, id, ctx);
            if f & 2 != 0 { w.mobys[id].set_state(4, 1); }
            if f & 1 != 0 {
                ctx.host.play_sound(&w.mobys[id], 3, 0);
                pool_d4b(w, id, ctx);
            }
        }
        _ => {}
    }
    // `0x43be60`: the ticks, its blow on what it touches (1.2).
    tick(w, id);
    if w.mobys[id].state != 1 || w.mobys[id].drawn != 0 { super::fighters::bump(w, id, ctx, 1.2); }
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0e2d, the links' bolts

/// `$gp+0x3050..+0x305c`: the bolt's length 0.3, speed 30 a second, life 30 units (a second's flight).
const E2D_SPEED: f32 = 30.0;
const E2D_REACH: f32 = 30.0;

/// `0x45aa48(at, owner, way, damage)`: a bolt 0x0e2d at `at` along `way` at 30 a second (its rows along it), its life
/// the ticks to fly 30, its owner and damage.
fn fire_e2d(w: &mut MobyWorld, owner: usize, ctx: &mut Ctx, at: V3, way: V3, damage: f32) {
    if w.class(E2D).is_none() { return; }
    let k = w.create(E2D);
    let v = set_len(way, E2D_SPEED / 60.0);
    let n = (E2D_REACH / (E2D_SPEED / 60.0)) as i32;
    let m = &mut w.mobys[k];
    m.state = 0;
    m.alpha = 0xff;
    m.pos = at;
    let r0 = set_len(v, 1.0);
    let r1 = scale(set_len(cross(r0, [0.0, 0.0, 1.0]), 1.0), -1.0);
    m.rows = [r0, r1, cross(r1, r0)];
    m.rot = crate::moby::attach::angles_of(&m.rows);
    m.mode |= mode::KEEP_ROWS;
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.set_pv_v(0x40, v);
    m.set_pv_f(0x64, damage);
    m.set_pv_i(0x58, n);
    m.set_pv_f(0x5c, 1.0 / n.max(1) as f32);
    ai::set_link(m, 0x50, Some(owner));
    m.set_pv_f(0x60, 0.3);
    w.rebuild(k, ctx.host);
    w.changed = true;
}

/// 0x0e2d (`0x45aba8`): a link's bolt — it flies its velocity; the line it moved along meets something not of its
/// owner's class: a hit there (a sphere of 0.2, flags 0x810001, its damage) and gone; its life over, gone; fading over
/// its last 15 ticks; out of the level's box (2..1021), gone.
pub fn update_e2d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let start = w.mobys[id].pos;
    if w.mobys[id].state == 0 {
        let v = w.mobys[id].pv_v(0x40);
        let to = add(start, v);
        w.mobys[id].pos = to;
        if let Some(o) = w.coll_line(ctx, start, to, 0, Some(id)) {
            let owner = ai::link(&w.mobys[id], 0x50);
            let own = owner.and_then(|k| w.mobys.get(k)).map(|x| x.o_class);
            let same = o.moby.and_then(|k| w.mobys.get(k)).is_some_and(|x| Some(x.o_class) == own);
            if !same {
                let dmg = w.mobys[id].pv_f(0x64);
                let d = set_len(v, 1.0);
                let mut t = crate::moby::hits::HitTemplate::new(dmg, owner, 0x81_0001, [d[0], d[1], d[2], 5627.925]);
                t.kind = (own.unwrap_or(E2D) as u16 as u32) << 16 | 1 << 8 | 2;
                w.sphere_hit(ctx.scene, o.point, 0.2, 0x10, Some(id), Some(&t));
                w.delete(id);
                return;
            }
        }
    }
    let mut n = w.mobys[id].pv_i(0x58);
    let over = crate::moby::countdown(&mut n);
    w.mobys[id].set_pv_i(0x58, n);
    if over {
        w.delete(id);
        return;
    }
    if n < 16 { w.mobys[id].set_pv_f(0x60, n as f32 / 15.0); }
    let p = w.mobys[id].pos;
    if p.iter().any(|&c| !(2.0..=1021.0).contains(&c)) { w.delete(id); }
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0e20, the chain's bombs

/// `$gp+0x3000..+0x3028`: the bomb's radius 0.208, its spark ticks 12, its scale 1.3, its bounce 0.6 (four bounces,
/// then it rolls), its roll's height 0.1 over the middle and speed 5, its reach 1, its blink 30 ticks, its blast 1.0
/// (radius 1.5), its blink colour (67, 230, 132).
const E20_R: f32 = 0.208;
const E20_SPARKS: i16 = 12;
const E20_SCALE: f32 = 1.3;
const E20_BOUNCE: f32 = 0.6;
const E20_BOUNCES: i16 = 4;
const E20_HEIGHT: f32 = 0.1;
const E20_SPEED: f32 = 5.0;
const E20_NEAR: f32 = 1.0;
const E20_BLINK: i32 = 30;
const E20_BLAST: f32 = 1.5;
const E20_COLOUR: V3 = [67.0, 230.0, 132.0];

/// `0x45a550(m, owner, master, target, gravity, damage, at, vel, life)`: a bomb thrown (state 2) at `at` moving `vel`,
/// its owner (the link), its master, the moby it rolls to, its gravity and damage.
#[allow(clippy::too_many_arguments)]
fn throw_e20(w: &mut MobyWorld, id: usize, ctx: &Ctx, owner: usize, master_of: Option<usize>, target: Option<usize>, at: V3, vel: V3, life: i32, gravity: f32, damage: f32) {
    let coll = w.class(E20).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.mode &= 0xffbe & !mode::TARGET;
    m.has_collision = coll;
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.state = 2;
    m.once = 0;
    m.state_ticks = 0;
    ai::set_link(m, 0x15c, Some(owner));
    m.parent = master_of;
    m.pos = at;
    m.set_pv_v(0xc0, vel);
    m.set_pv_s(0x16c, E20_SPARKS);
    m.set_pv_i(0x150, life);
    m.set_pv_f(0x4c, gravity);
    m.set_pv_s(0x16e, 0);
    m.rot = [0.0, 0.0, atan2(vel[0], vel[1])];
    m.rows = crate::moby::attach::rows_of(m.rot);
    m.set_pv_f(0x158, damage);
    ai::set_link(m, 0x164, target);
    m.glow = 0xff84_e643;
    w.rebuild(id, &*ctx.host);
    w.changed = true;
}

fn owner_class(w: &MobyWorld, id: usize) -> Option<i16> { ai::link(&w.mobys[id], 0x15c).and_then(|k| w.mobys.get(k)).map(|x| x.o_class) }

/// 0x0e20 (`0x459ba0`): 0x0d4a's bomb. Pooled (state 1) in the chain's group, thrown ([`throw_e20`]) it flies (2)
/// under its gravity, stopped by what its move meets (not its owner's kind) and bouncing off what its sphere touches
/// (3: its sound 1, its speed 0.6 of itself; after four bounces it rolls, 5): it rolls at 0.1 over the middle toward
/// its target at 5 a second and within 1 of it blinks (4: its glow eased to its blink colour over 30 ticks), then
/// bursts (6: a blast, its sound 0, a hit of its damage in 1.5, flags 0x810001) back to the pool. Any hit, or under
/// the middle by 1, ends it at once. Its spin follows its speed.
pub fn update_e20(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_b(0x148, 15);
            m.set_pv_b(0x14a, 4);
            let r = m.pv_f(0x160);
            m.update_dist = 128;
            m.set_pv_b(0x128, 3);
            m.set_pv_f(0x4c, 1.0 / 360.0);
            m.set_pv_f(0x20, 0.3);
            m.set_pv_f(0x28, 0.75);
            m.set_pv_f(0x2c, 0.5);
            m.set_pv_f(0x30, 1.134_464_1);
            m.set_pv_f(0x50, 0.523_598_85);
            m.set_pv_f(0x54, 0.1);
            m.set_pv_i(0x58, m.pv_i(0x58) | 0x14);
            m.set_pv_i(0x24, 0);
            m.set_pv_i(0x130, 0);
            m.scale *= E20_SCALE;
            m.set_pv_i(0x170, -1);
            m.has_collision = false;
            m.mode = (m.mode & !mode::TARGET) | 0x41;
            m.set_state(1, -1);
            let a = ctx.game.rng.randf(-r, r).to_radians();
            w.mobys[id].set_pv_f(0x160, a);
            return;
        }
        s @ (2 | 3) => {
            let m = &mut w.mobys[id];
            let mut v = m.pv_v(0xc0);
            v[2] -= m.pv_f(0x4c);
            m.set_pv_v(0xc0, v);
            if s == 3 && v[2] < 0.0 { m.state = 2; }
            let mut k = m.pv_s(0x16c) as i32;
            crate::moby::countdown(&mut k);
            m.set_pv_s(0x16c, k as i16);
            let from = m.pos;
            let to = add(from, v);
            let own = owner_class(w, id);
            let line = w.coll_line(ctx, from, to, 0, Some(id));
            let pos = match line { Some(o) if o.moby.and_then(|k| w.mobys.get(k)).map(|x| x.o_class) != own || o.moby.is_none() => o.point, _ => to };
            w.mobys[id].pos = pos;
            let touch = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, E20_R, crate::collision::QueryFlags(0), Some(id)));
            if let Some(o) = touch {
                let other = o.moby.and_then(|k| w.mobys.get(k)).map(|x| x.o_class);
                let ok = (w.mobys[id].state == 2 || pos[2] - o.point[2] < 0.0) && (other.is_none() || other != own);
                if ok {
                    let n = set_len(o.normal, 1.0);
                    let m = &mut w.mobys[id];
                    m.pos = add(o.point, scale(n, E20_R));
                    let v = m.pv_v(0xc0);
                    let r = sub(v, scale(n, 2.0 * dot(v, n)));
                    m.set_pv_v(0xc0, r);
                    ctx.host.play_sound(&w.mobys[id], 1, 0);
                    let m = &mut w.mobys[id];
                    if m.state == 2 {
                        m.state = 3;
                        m.set_pv_s(0x16e, m.pv_s(0x16e) + 1);
                        let v = m.pv_v(0xc0);
                        m.set_pv_v(0xc0, scale(v, E20_BOUNCE));
                    }
                    if E20_BOUNCES <= m.pv_s(0x16e) { m.set_state(5, -1); }
                }
            }
        }
        4 => {
            if w.mobys[id].first() {
                w.mobys[id].set_pv_i(0x150, E20_BLINK);
                w.mobys[id].set_pv_f(0x154, 1.0 / E20_BLINK as f32);
            }
            let m = &mut w.mobys[id];
            let f = m.pv_i(0x150) as f32 * m.pv_f(0x154);
            let r = ((E20_COLOUR[0] - 255.0) * f + 255.0) as u32;
            let g = (E20_COLOUR[1] * f) as u32;
            let b = (E20_COLOUR[2] * f) as u32;
            m.glow = 0xff00_0000 | b << 16 | g << 8 | r;
            let mut t = m.pv_i(0x150);
            let over = crate::moby::countdown(&mut t);
            m.set_pv_i(0x150, t);
            if over { m.set_state(6, -1); }
        }
        5 => {
            let mid = middle(w, id, ctx);
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                let a = ctx.game.rng.randf(15.0, 40.0).to_radians();
                let neg = ctx.game.rng.randi(2) != 0;
                w.mobys[id].set_pv_f(0x160, if neg { -a } else { a });
                let t = E20_SPEED;
                lim(w, id, (t + t) / 3600.0, t / 60.0, 0.001_163_553, 0.104_719_77);
            } else if w.mobys[id].timer == 0 {
                if w.mobys[id].pos[2] < mid[2] + E20_HEIGHT {
                    let m = &mut w.mobys[id];
                    let (mut z, mut v) = (m.pos[2], m.pv_f(0xc8));
                    let (a, b, c) = (m.pv_f(0x40), m.pv_f(0x40), m.pv_f(0x48));
                    crate::hero::physics::ease(mid[2] + E20_HEIGHT, a, b, c, &mut z, &mut v);
                    m.pos[2] = z;
                    m.set_pv_f(0xc8, v);
                } else {
                    w.mobys[id].timer = 1;
                }
            } else if w.mobys[id].timer == 1 {
                let goal = ai::link(&w.mobys[id], 0x164).and_then(|k| w.mobys.get(k)).map(|x| x.pos);
                if let (true, Some(g)) = (w.mobys[id].drawn != 0, goal) {
                    let off = w.mobys[id].pv_f(0x160);
                    ai::move_toward(w, id, ctx, off, g);
                }
                w.mobys[id].pos[2] = mid[2] + E20_HEIGHT;
                if let (true, Some(g)) = (w.mobys[id].drawn != 0, goal) {
                    let p = w.mobys[id].pos;
                    if (p[0] - g[0]).powi(2) + (p[1] - g[1]).powi(2) < E20_NEAR * E20_NEAR { w.mobys[id].set_state(4, -1); }
                }
            }
            let h = w.mobys[id].pv_i(0x170);
            if !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 2, 4);
                w.mobys[id].set_pv_i(0x170, h);
            }
        }
        6 => {
            if w.mobys[id].first() {
                let pos = w.mobys[id].pos;
                crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size: 1.0, bursts: [10, 10, 2], puffs: 20, streaks: 60, light: true, shake: true });
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                let dmg = w.mobys[id].pv_f(0x158);
                let mut t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 0x81_0001, [0.0, 0.0, 0.0, 5627.925]);
                t.kind = (E20 as u16 as u32) << 16 | 1 << 8 | 2;
                w.sphere_hit(ctx.scene, pos, E20_BLAST, 0x10, Some(id), Some(&t));
                end_e20(w, id, ctx);
                return;
            }
        }
        _ => return,
    }
    // The spin by the speed (1°..8° a tick about its second axis), its end under the middle, a hit ends it.
    let m = &mut w.mobys[id];
    let s = (len(m.pv_v(0xc0)).clamp(0.003_333_333_6, 0.1) - 0.003_333_333_6) / 0.096_666_68;
    m.rot[1] = add_rot((s * 7.0 + 1.0) * 0.017_453_294, m.rot[1]);
    m.rows = crate::moby::attach::rows_of(m.rot);
    if master(w, id).is_some() && w.mobys[id].pos[2] < middle(w, id, ctx)[2] - 1.0 {
        end_e20(w, id, ctx);
        return;
    }
    if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x1_0000 != 0) { w.mobys[id].set_state(6, -1); }
    w.clear_hit(id);
    tick(w, id);
}

/// A bomb back in the pool (state 1, hidden, its hum stopped).
fn end_e20(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(0x170);
    if h != -1 { ctx.host.stop_sound(h); }
    let m = &mut w.mobys[id];
    m.set_pv_i(0x170, -1);
    m.has_collision = false;
    m.mode = (m.mode | 0x41) & !mode::TARGET;
    m.set_state(1, -1);
    w.changed = true;
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0b3a

/// Its words: health `+0x20`, react `+0x50`, the flash `+0x70`, the target block `+0x80`, its two arcs `+0xa0` /
/// `+0x140` (their start and end, `+0x00` / `+0x10`), their aims `+0x1f0`, starts `+0x210`, the moby held's last place
/// `+0x230`, the flip's way `+0x1e0`, its angle round the middle `+0x250` and its speed `+0x254`, found `+0x258`,
/// area `+0x25c`, the arcs' charge `+0x260`, the moby an arc holds `+0x268`, the flip's turn `+0x270`, the arcs'
/// ticks `+0x274` and states `+0x278` (0 off, 1 on, 2 charging, 3 out, 4 holding, 5 back) and repeats `+0x27c`, its
/// colour `+0x280..`, its hum `+0x290`, the taunt's sub `+0x294`, an arc hurt Ratchet `+0x296`, the flip's ticks
/// `+0x298`.
mod b3a {
    pub const TARGET: usize = 0x80;
    pub const ARCS: usize = 0xa0;
    pub const FLIP: usize = 0x1e0;
    pub const AIMS: usize = 0x1f0;
    pub const STARTS: usize = 0x210;
    pub const LAST: usize = 0x230;
    pub const ANGLE: usize = 0x250;
    pub const SPEED: usize = 0x254;
    pub const FOUND: usize = 0x258;
    pub const AREA: usize = 0x25c;
    pub const CHARGE: usize = 0x260;
    pub const HELD: usize = 0x268;
    pub const TURN: usize = 0x270;
    pub const TICKS: usize = 0x274;
    pub const STATES: usize = 0x278;
    pub const REPEATS: usize = 0x27c;
    pub const COLOUR: usize = 0x280;
    pub const HUM: usize = 0x290;
    pub const BACK: usize = 0x294;
    pub const HURT: usize = 0x296;
    pub const FLIPS: usize = 0x298;
}

/// `$gp+0x1500..+0x15c8`: its heights on the wall (468 over, 508 under), its range to attack (5..15 across), its
/// speed round (272″ a tick², 136° a second at most) 15 from the middle; a 0x0d37 sent every 90 ticks 14 out, 4.5
/// over or under it, up to 12 (the summons' end after 10 ticks); a big hit 7; the death's playback 0.2; the walk's 6;
/// the flip over 60 ticks (40 a second up or down); a flip with Ratchet within 15 of its height; the summons one time
/// in 4 (else the arcs); the arcs' aim 20 ± 5 (jitter 10), four repeats, 30 ticks of charge, 10 out, 50 held, 10 back.
mod b3k {
    pub const OVER: f32 = 468.0;
    pub const UNDER: f32 = 508.0;
    pub const NEAR: (f32, f32) = (5.0, 15.0);
    pub const ACCEL: f32 = 272.0;
    pub const TOP: f32 = 136.0;
    pub const RADIUS: f32 = 15.0;
    pub const EVERY: u16 = 90;
    pub const OUT: f32 = 14.0;
    pub const UP: f32 = 4.5;
    pub const MOST: i32 = 12;
    pub const SUMMONS: u16 = 10;
    pub const BIG: f32 = 7.0;
    pub const DEATH_PLAY: f32 = 0.2;
    pub const WALK_PLAY: f32 = 6.0;
    pub const FLIP: i32 = 60;
    pub const FLIP_SPEED: f32 = 40.0;
    pub const LEVEL: f32 = 15.0;
    pub const SUMMON_ODDS: i32 = 4;
    pub const AIM: (f32, f32, f32) = (20.0, 5.0, 10.0);
    pub const REPEATS: i16 = 4;
    pub const CHARGE: i16 = 30;
    pub const SHOT: i16 = 10;
    pub const HOLD: i16 = 50;
    pub const RETRACT: i16 = 10;
}

/// `0x3f4448`: sent — its life, shown with its collision, a target; on the wall over the middle (468) at its angle 0,
/// facing down (2, sub 2).
pub fn start_b3a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let coll = w.class(arena::B3A).is_some_and(|c| c.has_collision);
    let mid = middle(w, id, ctx);
    let m = &mut w.mobys[id];
    let life = m.pv_s(0x24) as f32;
    m.set_pv_f(0x20, life);
    m.mode &= !1;
    m.drawn = 1;
    m.has_collision = coll;
    m.set_pv_b(0x2e, 1);
    m.drops = 0;
    m.mode = (m.mode & 0xffb9) | mode::TARGET;
    m.set_pv_i(0x60, 0);
    if m.pv_s(0x70) != 0 {
        m.set_pv_s(0x70, 0);
        m.ambient = [m.pv_i(b3a::COLOUR) as u8, m.pv_i(b3a::COLOUR + 4) as u8, m.pv_i(b3a::COLOUR + 8) as u8];
        m.set_pv_s(0x72, 1);
    }
    m.set_state(2, 2);
    m.set_pv_f(b3a::ANGLE, 0.0);
    m.pos = [b3k::RADIUS + mid[0], mid[1], b3k::OVER];
    m.rot = [0.0, -std::f32::consts::FRAC_PI_2, add_rot(std::f32::consts::PI, 0.0)];
    m.rows = crate::moby::attach::rows_of(m.rot);
    w.changed = true;
}

/// `0x3f46d0`: its arcs off, its hum stopped.
fn arcs_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for i in 0..2 {
        w.mobys[id].set_pv_s(b3a::TICKS + 2 * i, 0);
        w.mobys[id].set_pv_s(b3a::STATES + 2 * i, 0);
    }
    stop_hum(w, id, ctx, b3a::HUM);
}

fn arcs_set(w: &mut MobyWorld, id: usize, state: i16) {
    for i in 0..2 {
        w.mobys[id].set_pv_s(b3a::TICKS + 2 * i, 0);
        w.mobys[id].set_pv_s(b3a::STATES + 2 * i, state);
    }
}

fn arcs(w: &MobyWorld, id: usize) -> (i16, i16) { (w.mobys[id].pv_s(b3a::STATES), w.mobys[id].pv_s(b3a::STATES + 2)) }

/// `0x3f45a8`: back to the pool — its arcs off, hidden.
pub fn pool_b3a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    arcs_off(w, id, ctx);
    ctx.host.set_anim(&w.mobys[id], 0, 0, 3);
    let m = &mut w.mobys[id];
    m.set_state(1, 0);
    m.drawn = 0;
    m.has_collision = false;
    m.mode = (m.mode | 1) & !mode::TARGET;
    w.changed = true;
}

/// `0x3f42d8`: the taunt (3), its state and sub to go back to.
fn taunt(w: &mut MobyWorld, id: usize) {
    let sub = w.mobys[id].sub_state;
    w.mobys[id].set_pv_s(b3a::BACK, sub as i16);
    w.mobys[id].set_state(3, -1);
}

/// `0x3f4248`: an attack — with its arcs idle, one time in four (while the master has fewer than 12 0x0d37s out)
/// the summons (7), else the arcs (6).
fn attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (a, b) = arcs(w, id);
    if 2 <= a || 2 <= b { return; }
    let n = master(w, id).map_or(0, |ms| w.mobys[ms].pv_s(arena::pv::D37S) as i32);
    if ctx.game.rng.randi(b3k::SUMMON_ODDS) != 0 && n < b3k::MOST { w.mobys[id].set_state(7, -1); } else { w.mobys[id].set_state(6, -1); }
}

/// `0x3f4768(m, from, &to)`: an arc's aim — its target (200 out), else Ratchet; out of `20 ± 5` from `from` a point
/// that far toward it (jittered 10); then pulled in to 15 − 0.2 (0.5 when reached) from the middle across. True
/// when the target was within reach.
fn arc_aim(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, from: V3) -> (V3, bool) {
    let a = w.mobys[id].pv_i(b3a::AREA);
    let f = ai::find_target(w, id, ctx, 200.0, 200.0, a, Some(from), false);
    w.mobys[id].set_pv_i(b3a::FOUND, f as i32);
    let mut to = if f { w.mobys[id].pv_v(b3a::TARGET) } else { ctx.hero.pos };
    let r = b3k::AIM.0 + ctx.game.rng.randf(-b3k::AIM.1, b3k::AIM.1);
    let (reached, k) = if r * r < dot(sub(to, from), sub(to, from)) {
        let mut d = sub(to, from);
        for c in 0..3 { d[c] += ctx.game.rng.randf(-b3k::AIM.2, b3k::AIM.2); }
        to = add(from, set_len(d, r));
        (false, 0.2)
    } else {
        (true, 0.5)
    };
    let mid = middle(w, id, ctx);
    let h = to[2];
    let v = set_len([to[0] - mid[0], to[1] - mid[1], 0.0], b3k::RADIUS - k);
    (add(mid, [v[0], v[1], h - mid[2]]), reached)
}

/// `0x3f48f0`: the two arcs (from its joints 1 and 0): on, from one joint to the other; charging (30 ticks, both
/// joints hitting for `+0x34` in 0.2); out (10 ticks: its end from its start to its aim, the line between them
/// hitting what it meets — not a 0x0c7d — for `+0x34`, then holding it: 50 ticks following the moby held); back (10
/// ticks; again from its aim up to four times, else off).
fn arcs_tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use b3a::*;
    let j = [ctx.host.joint(&w.mobys[id], 1), ctx.host.joint(&w.mobys[id], 0)].map(|p| p.unwrap_or(w.mobys[id].pos));
    let dmg = w.mobys[id].pv_b(0x34) as f32;
    for i in 0..2 {
        let arc = ARCS + 0xa0 * i;
        w.mobys[id].set_pv_f(CHARGE + 4 * i, 0.0);
        let ticks = w.mobys[id].pv_s(TICKS + 2 * i);
        match w.mobys[id].pv_s(STATES + 2 * i) {
            1 => {
                w.mobys[id].set_pv_v(arc, j[i]);
                w.mobys[id].set_pv_v(arc + 0x10, j[i ^ 1]);
            }
            2 => {
                w.mobys[id].set_pv_v(arc, j[i]);
                w.mobys[id].set_pv_v(arc + 0x10, j[i ^ 1]);
                for p in j { ai::attack_sphere(w, id, ctx, 0.2, dmg, 1.0, p, 0x1_0001, 0, 1, 0); }
                if b3k::CHARGE < ticks {
                    let m = &mut w.mobys[id];
                    m.set_pv_s(TICKS + 2 * i, 0);
                    m.set_pv_s(STATES + 2 * i, 3);
                    m.set_pv_s(REPEATS + 2 * i, 0);
                    m.set_pv_v(STARTS + 0x10 * i, j[i]);
                    m.set_pv_v(arc, j[i]);
                    let (to, reached) = arc_aim(w, id, ctx, j[i]);
                    w.mobys[id].set_pv_v(AIMS + 0x10 * i, to);
                    if reached { w.mobys[id].set_pv_s(REPEATS + 2 * i, b3k::REPEATS); }
                } else {
                    w.mobys[id].set_pv_f(CHARGE + 4 * i, ticks as f32 / b3k::CHARGE as f32);
                }
            }
            3 => {
                ai::attack_sphere(w, id, ctx, 0.2, dmg, 1.0, j[i], 0x1_0001, 0, 1, 0);
                if ticks <= b3k::SHOT {
                    let t = ticks as f32 / b3k::SHOT as f32;
                    let (o, a) = (w.mobys[id].pv_v(STARTS + 0x10 * i), w.mobys[id].pv_v(AIMS + 0x10 * i));
                    let end = add(o, scale(sub(a, o), t));
                    w.mobys[id].set_pv_v(arc + 0x10, end);
                    let start = w.mobys[id].pv_v(arc);
                    if let Some(hit) = w.coll_line(ctx, start, end, 0, Some(id)) {
                        if let Some(k) = hit.moby.filter(|&k| w.mobys[k].o_class != 0x0c7d) {
                            let d = scale(sub(end, start), 1.0 / 30.0);
                            let mut tm = crate::moby::hits::HitTemplate::new(dmg, Some(id), 0x21_0001, [d[0], d[1], d[2], 5627.925]);
                            tm.kind = (arena::B3A as u16 as u32) << 16 | 3 << 8 | 5;
                            w.deliver_hit_any(k, &tm, [hit.point[0], hit.point[1], hit.point[2], 1.0]);
                            let m = &mut w.mobys[id];
                            m.set_pv_v(AIMS + 0x10 * i, end);
                            m.set_pv_s(TICKS + 2 * i, 0);
                            m.set_pv_s(STATES + 2 * i, 4);
                            ai::set_link(m, HELD + 4 * i, Some(k));
                            m.set_pv_v(arc + 0x10, hit.point);
                            let kp = w.mobys[k].pos;
                            w.mobys[id].set_pv_v(LAST + 0x10 * i, kp);
                            if ctx.hero.damage.hit_by == Some(id) { w.mobys[id].set_pv_s(HURT, 1); }
                        }
                    }
                } else {
                    w.mobys[id].set_pv_s(STATES + 2 * i, 5);
                    w.mobys[id].set_pv_s(TICKS + 2 * i, 0);
                }
            }
            4 => {
                let held = ai::link(&w.mobys[id], HELD + 4 * i).filter(|&k| w.mobys[k].state < 0x80);
                match held {
                    Some(k) if ticks <= b3k::HOLD => {
                        let kp = w.mobys[k].pos;
                        let m = &mut w.mobys[id];
                        let d = sub(kp, m.pv_v(LAST + 0x10 * i));
                        let e = add(m.pv_v(arc + 0x10), d);
                        m.set_pv_v(arc + 0x10, e);
                        m.set_pv_v(LAST + 0x10 * i, kp);
                    }
                    _ => {
                        let m = &mut w.mobys[id];
                        m.set_pv_s(TICKS + 2 * i, 0);
                        ai::set_link(m, HELD + 4 * i, None);
                        m.set_pv_s(REPEATS + 2 * i, b3k::REPEATS);
                        m.set_pv_s(STATES + 2 * i, 5);
                    }
                }
            }
            5 => {
                if ticks < b3k::RETRACT {
                    let t = ticks as f32 / b3k::RETRACT as f32;
                    let (o, a) = (w.mobys[id].pv_v(STARTS + 0x10 * i), w.mobys[id].pv_v(AIMS + 0x10 * i));
                    w.mobys[id].set_pv_v(arc, add(o, scale(sub(a, o), t)));
                } else if w.mobys[id].pv_s(REPEATS + 2 * i) < b3k::REPEATS {
                    let a = w.mobys[id].pv_v(AIMS + 0x10 * i);
                    let m = &mut w.mobys[id];
                    m.set_pv_s(REPEATS + 2 * i, m.pv_s(REPEATS + 2 * i) + 1);
                    m.set_pv_s(TICKS + 2 * i, 0);
                    m.set_pv_s(STATES + 2 * i, 3);
                    m.set_pv_v(STARTS + 0x10 * i, a);
                    m.set_pv_v(arc, a);
                    let (to, reached) = arc_aim(w, id, ctx, a);
                    w.mobys[id].set_pv_v(AIMS + 0x10 * i, to);
                    if reached { w.mobys[id].set_pv_s(REPEATS + 2 * i, b3k::REPEATS); }
                } else {
                    stop_hum(w, id, ctx, HUM);
                    w.mobys[id].set_pv_s(TICKS + 2 * i, 0);
                    w.mobys[id].set_pv_s(STATES + 2 * i, 0);
                }
            }
            _ => {}
        }
    }
}

/// `0x3f3f40`: its way round the middle (its walk's sequence 1 slowing, 2 speeding: 272″ a tick², at most 136° a
/// second), on the wall 15 out; over (sub 2) at 468 facing down, under (sub 1) at 508 facing up.
fn round(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    use b3a::*;
    let sa = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
    let m = &mut w.mobys[id];
    let top = b3k::TOP * 0.000_290_888_25;
    match sa {
        1 => m.set_pv_f(SPEED, (m.pv_f(SPEED) - b3k::ACCEL * 0.000_004_848_137).clamp(-top, top)),
        2 => m.set_pv_f(SPEED, (m.pv_f(SPEED) + b3k::ACCEL * 0.000_004_848_137).clamp(-top, top)),
        _ => {}
    }
    if matches!(sa, 1 | 2) { m.set_pv_f(ANGLE, add_rot(m.pv_f(ANGLE), m.pv_f(SPEED))); }
    if matches!(m.state, 5 | 9) { return; }
    let mid = middle(w, id, ctx);
    let m = &mut w.mobys[id];
    let a = m.pv_f(ANGLE);
    m.pos[0] = b3k::RADIUS * a.cos() + mid[0];
    m.pos[1] = b3k::RADIUS * a.sin() + mid[1];
    match m.sub_state {
        2 => {
            m.pos[2] = b3k::OVER;
            m.rot = [0.0, -std::f32::consts::FRAC_PI_2, a];
            m.rows = crate::moby::attach::rows_of(m.rot);
        }
        1 => {
            m.pos[2] = b3k::UNDER;
            m.rot = [0.0, std::f32::consts::FRAC_PI_2, add_rot(std::f32::consts::PI, a)];
            m.rows = crate::moby::attach::rows_of(m.rot);
        }
        _ => {}
    }
}

/// 0x0b3a (`0x3f3178`): the ring wall's boss, sent in a wave of the ring; it waits for the gravity's cylinder mode
/// (`0x1a8ff0` / `0x1a8ff4`). It goes round the middle on the wall over or under the arena ([`round`]), turning to
/// chase Ratchet; with him 5..15 across it attacks ([`attack`]): its two arcs ([`arcs_tick`], 6) or the summons of
/// 0x0d37s (7: one 14 out, over or under it, through the master's `0x442378`); with him within 15 of its height it
/// flips to the other side (5: over 60 ticks, across the middle). A big hit (7 or more, or dead) staggers it (8, the
/// arcs again); dead it falls toward the middle (9) and bursts (a blast of 3), its bolts, back to the pool. It taunts
/// (3) when an arc hurt Ratchet.
pub fn update_b3a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use b3a::*;
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 16);
        let c = m.ambient;
        for k in 0..3 { m.set_pv_i(COLOUR + 4 * k, c[k] as i32); }
        m.set_pv_b(0x28, 3);
        m.set_pv_f(0x30, 1.5);
        m.set_pv_b(0x29, 1);
        m.mode |= mode::KEEP_ROWS;
        pool_b3a(w, id, ctx);
        w.mobys[id].set_pv_i(HUM, -1);
        return;
    }
    // `0x3f4310`: off the cylinder mode it waits (but for its fall).
    let g = w.level11.arena_gravity;
    if !(g[0] && g[1]) && w.mobys[id].state != 9 { return; }
    if w.mobys[id].drawn != 0 && dist(w.mobys[id].pos, ctx.host.camera_pos()) < 48.0 {
        crate::shadow_volumes::probe_near(w, id, ctx, 48.0, 45);
    }
    senses_b3a(w, id, ctx);
    if w.mobys[id].state == 1 {
        if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 0) { w.mobys[id].mode |= 0x46; }
        return;
    }
    let found = w.mobys[id].pv_i(FOUND) == 1;
    let t = if found { w.mobys[id].pv_v(TARGET) } else { ctx.hero.pos };
    let across = dist2(w.mobys[id].pos, t);
    let hero_z = ctx.hero.pos[2];
    let mid = middle(w, id, ctx);
    match w.mobys[id].state {
        2 => {
            w.mobys[id].set_state(4, -1);
            taunt(w, id);
        }
        3 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 8, 0, 10); }
            if wrapped(w, id, ctx) {
                let (s, b) = (w.mobys[id].prev_state, w.mobys[id].pv_s(BACK));
                w.mobys[id].set_state(s, b as i32);
            }
        }
        4 => {
            if w.mobys[id].first() {
                let here = w.mobys[id].pos;
                let ccw = (t[0] - mid[0]) * (here[1] - mid[1]) - (t[1] - mid[1]) * (here[0] - mid[0]) > 0.0;
                let under = w.mobys[id].sub_state == 1;
                let s = if ccw == under { 1 } else { 2 };
                w.mobys[id].timer = ccw as u8;
                set_seq(w, id, ctx, s, 0, 10);
                ctx.host.set_anim_speed(&w.mobys[id], b3k::WALK_PLAY);
            }
            if (hero_z - w.mobys[id].pos[2]).abs() < b3k::LEVEL {
                w.mobys[id].set_state(5, -1);
            } else if b3k::NEAR.0 < across && across < b3k::NEAR.1 {
                attack(w, id, ctx);
            }
        }
        5 => {
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                set_seq(w, id, ctx, 9, 0, 10);
                let up = if w.mobys[id].sub_state == 2 { 1.0 } else { -1.0 };
                let m = &mut w.mobys[id];
                m.set_pv_v(FLIP, [0.0, 0.0, up * b3k::FLIP_SPEED / b3k::FLIP as f32]);
                m.set_pv_f(TURN, std::f32::consts::PI / b3k::FLIP as f32);
                m.set_pv_s(FLIPS, 0);
            } else {
                match w.mobys[id].timer {
                    0 => if wrapped(w, id, ctx) {
                        w.mobys[id].timer = 1;
                        set_seq(w, id, ctx, 10, 0, 0);
                    },
                    1 => if (b3k::FLIP / 2) < w.mobys[id].state_ticks as i32 { w.mobys[id].timer = 2; },
                    2 => if b3k::FLIP - 10 < w.mobys[id].state_ticks as i32 {
                        w.mobys[id].timer = 3;
                        ctx.host.set_anim(&w.mobys[id], 9, 4, 0);
                        ctx.host.set_anim_speed(&w.mobys[id], -1.0);
                    },
                    3 => if ctx.host.frame_time(&w.mobys[id]) < 2.0 {
                        let s = if 490.0 < w.mobys[id].pos[2] { 1 } else { 2 };
                        ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                        w.mobys[id].set_state(4, s);
                    },
                    _ => {}
                }
            }
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, m.pv_v(FLIP));
            let r = crate::quat::rows(crate::quat::axis_angle(m.pv_f(TURN), m.rows[2]));
            m.rows = crate::moby::attach::turn(&m.rows, &r);
            m.rot = crate::moby::attach::angles_of(&m.rows);
            let k = m.pv_s(FLIPS) as f32 * 2.0 / b3k::FLIP as f32;
            let amp = b3k::RADIUS * (1.0 - k) * (1.0 - k);
            let a = m.pv_f(ANGLE);
            m.pos[0] = amp * a.cos() + mid[0];
            m.pos[1] = amp * a.sin() + mid[1];
            m.set_pv_s(FLIPS, m.pv_s(FLIPS) + 1);
        }
        s @ (6 | 8) => {
            let q = if s == 6 { 3 } else { 7 };
            if w.mobys[id].first() {
                set_seq(w, id, ctx, q, 0, if s == 6 { 10 } else { 3 });
                let (a, b) = arcs(w, id);
                if s == 6 || (a < 2 && b < 2) {
                    w.mobys[id].set_pv_s(HURT, 0);
                    w.mobys[id].timer = 0;
                    arcs_set(w, id, 1);
                } else {
                    w.mobys[id].timer = 3;
                }
                if s == 8 && w.mobys[id].pv_f(0x20) <= 0.0 { w.mobys[id].set_state(9, -1); }
            }
            if w.mobys[id].state == s {
                let ft = ctx.host.frame_time(&w.mobys[id]);
                let on = ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == q);
                let late = if s == 6 { 30.0 } else { 32.0 };
                match w.mobys[id].timer {
                    0 if arcs(w, id) == (1, 1) && on && late < ft => {
                        arcs_set(w, id, 2);
                        keep_hum(w, id, ctx, HUM, 2);
                        w.mobys[id].timer = 1;
                    }
                    1 if on && wrapped(w, id, ctx) => {
                        set_seq(w, id, ctx, 0, 0, 10);
                        w.mobys[id].timer = 2;
                    }
                    2 if arcs(w, id) == (0, 0) => {
                        w.mobys[id].set_state(4, -1);
                        if w.mobys[id].pv_s(HURT) != 0 { taunt(w, id); }
                    }
                    3 if on && wrapped(w, id, ctx) => w.mobys[id].set_state(4, -1),
                    _ => {}
                }
            }
        }
        7 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 4, 0, 10);
                w.mobys[id].timer = 0;
                arcs_set(w, id, 1);
            }
            match w.mobys[id].timer {
                0 => if wrapped(w, id, ctx) {
                    set_seq(w, id, ctx, 5, 0, 10);
                    w.mobys[id].timer = 1;
                    w.mobys[id].state_ticks = (b3k::EVERY - 1) as i16;
                },
                1 => {
                    let ticks = w.mobys[id].state_ticks as u16;
                    if ticks % b3k::EVERY == 0 {
                        let p = w.mobys[id].pos;
                        let v = set_len([p[0] - mid[0], p[1] - mid[1], 0.0], b3k::OUT);
                        let mut at = add(mid, v);
                        at[2] = match w.mobys[id].sub_state { 2 => p[2] + b3k::UP, 1 => p[2] - b3k::UP, _ => mid[2] };
                        if let Some(ms) = master(w, id) { summon(w, ms, ctx, at); }
                    }
                    let n = master(w, id).map_or(0, |ms| w.mobys[ms].pv_s(arena::pv::D37S) as i32);
                    if b3k::MOST <= n || b3k::SUMMONS < ticks {
                        set_seq(w, id, ctx, 6, 0, 10);
                        w.mobys[id].timer = 2;
                    }
                    if (hero_z - w.mobys[id].pos[2]).abs() < b3k::LEVEL { w.mobys[id].set_state(5, -1); }
                }
                2 => if wrapped(w, id, ctx) { w.mobys[id].set_state(4, -1); },
                _ => {}
            }
        }
        9 => {
            if w.mobys[id].first() {
                let p = w.mobys[id].pos;
                let m = &mut w.mobys[id];
                m.set_pv_v(FLIP, set_len([mid[0] - p[0], mid[1] - p[1], 0.0], 1.0 / 6.0));
                ctx.host.set_anim_speed(&w.mobys[id], b3k::DEATH_PLAY);
                arcs_off(w, id, ctx);
                w.mobys[id].mode &= !mode::TARGET;
            }
            let m = &mut w.mobys[id];
            let mut v = m.pv_v(FLIP);
            v[2] -= 1.0 / 360.0;
            m.set_pv_v(FLIP, v);
            m.pos = add(m.pos, v);
            if m.pos[2] < mid[2] + 5.0 || ctx.game.rng.randi(0x1c2) == 0 || 301 <= m.state_ticks as u16 {
                bolts(w, id, ctx);
                pool_b3a(w, id, ctx);
                let pos = w.mobys[id].pos;
                crate::moby::blast::burst(w, ctx, pos, &crate::moby::blast::Blast { size: 3.0, bursts: [25, 15, 30], puffs: 20, streaks: 30, light: false, shake: false });
                ctx.host.play_sound_of(&w.mobys[id], DF7, 0, 0);
                return;
            }
        }
        _ => {}
    }
    arcs_tick(w, id, ctx);
    round(w, id, ctx);
    // `0x3f4138`: the flash and its colour's return, the arcs' ticks, the ticks.
    let m = &mut w.mobys[id];
    ai::flash(m, 0x70);
    if m.pv_s(0x70) == 0 {
        if m.pv_i(0x28c) == 0 {
            m.ambient = [m.pv_i(COLOUR) as u8, m.pv_i(COLOUR + 4) as u8, m.pv_i(COLOUR + 8) as u8];
            m.set_pv_i(0x28c, 1);
        }
    } else {
        m.set_pv_i(0x28c, 0);
    }
    for i in 0..2 {
        let t = m.pv_s(TICKS + 2 * i);
        if t < 0x7fff { m.set_pv_s(TICKS + 2 * i, t + 1); }
    }
    tick(w, id);
}

/// `0x3f32c0`: its damage past state 2 (a hit of 7 or more, or dead, the stagger 8), told to go its pool, its target
/// (200 out).
fn senses_b3a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
    let dmg = ai::damage_of(w, id, hit.as_ref(), false);
    if let Some(h) = hit {
        if 3 <= w.mobys[id].state {
            let m = &mut w.mobys[id];
            m.set_pv_f(0x20, m.pv_f(0x20) - dmg);
            if h.tmpl.damage != 0.0 {
                ai::flash_start(m, 0x70);
                if b3k::BIG <= h.tmpl.damage || m.pv_f(0x20) <= 0.0 { m.set_state(8, -1); }
            }
        }
    }
    w.clear_hit(id);
    if 2 <= w.mobys[id].state && w.mobys[id].pv_b(0x2e) == 2 {
        w.mobys[id].set_pv_f(0x20, 0.0);
        pool_b3a(w, id, ctx);
    }
    let a = w.mobys[id].pv_i(b3a::AREA);
    let f = ai::find_target(w, id, ctx, 200.0, 200.0, a, None, false);
    w.mobys[id].set_pv_i(b3a::FOUND, f as i32);
}

/// `0x442378(master, p)`: the master's first pooled 0x0d37 of its group `+0xc0` placed at `p` (owned by the master,
/// facing out from the middle, upright on the wall) and started in the ring ([`super::fighters::start`]).
fn summon(w: &mut MobyWorld, ms: usize, ctx: &mut Ctx, p: V3) -> bool {
    let g = usize::try_from(w.mobys[ms].pv_i(arena::pv::ENEMIES)).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
    let Some(f) = g.into_iter().find(|&k| { let x = &w.mobys[k]; x.o_class == arena::D37 && x.state == 1 }) else { return false };
    let mid = arena::middle(w, ms, ctx).unwrap_or(p);
    let x = &mut w.mobys[f];
    x.parent = Some(ms);
    x.pos = p;
    x.rot = [0.0, -std::f32::consts::FRAC_PI_2, atan2(p[0] - mid[0], p[1] - mid[1])];
    x.rows = crate::moby::attach::rows_of(x.rot);
    super::fighters::start(w, f, ctx, 0, true);
    true
}
