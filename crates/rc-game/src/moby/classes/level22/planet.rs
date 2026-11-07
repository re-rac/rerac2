//! The orbit fight's planetoid helpers (L22 addresses; the same code at L23's places, the decomp's
//! `func_L22_003A2160` … `func_L22_003A4EE8`): the sphere the fight is on is the gravity frames' (crate::gravity, its
//! centre `0x189ed0`); its radius is the opponent's distance from that centre at its set-up (`0x1b2a24`,
//! crate::moby::classes::level22::boss). The opponent walks the level's paths its pvars name: 32 of them at `+0x2a0`
//! (the slots 0..31; each a level path, `0x1ffec0[id]`: its points, their w the slot's link bits at its two ends) and
//! one more at `+0x340` (the pickups'). The small enemies 0x0edd (crate::moby::classes::level22::drone) use the same
//! tests on the opponent's slots.
//!
//! * **The surface** ([`surface`], `0x3a3db8(p, h)`): `p` moved along its line from the centre to `h` above it.
//! * **The walk test** ([`clear_walk`], `0x3a3bb0(m, from, to)`): no more than 50 apart, and every step of 8 from
//!   `from` to `to` (each point put on the surface lifted by the mover's radius and height, `+0x110 + +0x114`: 3.25
//!   for the opponent, whose block it reads for the enemies too) free of the world (a line 0x12).
//! * **The links** ([`link_paths`], `0x3a4798`, at the set-up): each slot's ends get a bit for every other slot with an
//!   end within 1 of it (the start's bits in the first point's w, the end's in the last's).
//! * **The skip table** (`$gp+0x370`, its count `0x1b2a54` = 5, copied from `0x1af580` at the set-up: slots 10, 27,
//!   28, 31, 15): slots the walks leave out ([`passable`], `0x3a5d28`; [`skipped`], `0x3a6098`). **The scan**
//!   ([`scan`], `0x3a5d68`, each tick of the fight): the next entry still in the table (`$gp+0x384`, cyclic); both
//!   sides of its slot's line (1.5 across its way at its two ends, lifted as the walk test) free → it leaves the
//!   table (−1).
//! * **The nearest point** ([`nearest_point`], `0x3a3fc0(m, out, p, filter)`): of every point of the slots (those
//!   the table passes with `filter`) the nearest to `p` (none nearer than 30000: `out` unchanged).
//! * **The path walk** ([`follow`], `0x3a3590`; its block `+0x350` the walk's state, `+0x352` its way, `+0x354` the
//!   slot, `+0x356` the point, `+0x360` the waypoint, `+0x3ac` its goal — Ratchet's place or the point `+0x3b0` —,
//!   `+0x3d8` the walk test to Ratchet, `+0x400` its own place on the surface, `+0x43c` the turn left): the goal
//!   flattened; the walk test to it (Ratchet's from `+0x3d8`). Clear, a walk in state 0 stays there; not, state 1.
//!   Unless the walk is over (2: the waypoint clear; 4: the goal clear; the goal clear in 0; 3 never; Ratchet's goal
//!   clear) it steers round what is ahead (`+0x378`, at the point `+0x390` until within 10 or stuck 5 ticks,
//!   `+0x190`; [`avoid`], `0x3a3308`, else, and the walk's ticks `+0x384` counted). **1**: the nearest point of the
//!   slots the table passes 4 behind it from Ratchet ([`nearest_path`], `0x3a3e58`), then the way along that slot
//!   that brings it nearer the goal and the waypoint the nearest point to it of the segment that way (the game's:
//!   from a middle point the segment is to the slot's point `+0x352`, its header for −1, not to the next), → 2.
//!   **2**: to the waypoint; within 2 → 3. **3**: within 20 of the goal flattened with the goal clear → 4; else
//!   within 2 of the waypoint the next point its way, past an end the next slot ([`next_path`], `0x3a4280` mode 1).
//!   **4**: → 0. Then the move toward the target (crate::moby::frames::move_to) and the turn left to it.
//! * **The next slot** ([`next_path`], `0x3a4280(bits, m, mode, filter)`): the slots linked at the end (`bits`, the
//!   table passing them with `filter`); none: the way turned back. Mode 0 a random one; 1 the one whose far end is
//!   nearest the goal; 3 the one whose far end is most toward Ratchet; 2 the one whose way is nearest 0.35 off the way
//!   to Ratchet (a dead end's far end — one link, to a slot in the table — refused with `filter`, `0x3a7558`). The
//!   slot taken (`0x3a4240`: the last three kept, `+0x358..+0x35c` and `+0x35e`) at its linked end, the way into it.
//! * **The point by Ratchet** ([`beside_hero`], `0x3a6318`): the nearest point (the table passing) to the middle of
//!   its own place's way from him and his down, each 4 radii, from his place → `+0x3b0`, the goal.
//!
//! The game's mode 2 straightens both ways in a frame about its up and the down (`0x32b870`) before their dot: here
//! the ways put across the up and made unit.

use crate::hero::physics::{add, cross, dist, dot, len, scale, set_len, sub, unit, V3};
use crate::moby::{Ctx, MobyWorld};

/// `0x3a3db8(p, h)`: `p` moved along its line from the sphere's centre to `h` above the surface (radius + `h` from
/// the centre).
pub fn surface(ctx: &Ctx, radius: f32, p: V3, h: f32) -> V3 {
    let c = ctx.hero.gravity.centre;
    add(set_len(sub(p, c), h + radius), c)
}

/// `0x32b440(p, out, 0)` (L00 `0x326448`): the down at `p` (world −z with the frames off).
pub fn down_at(ctx: &Ctx, p: V3) -> V3 { scale(ctx.hero.gravity.up_at(p), -1.0) }

/// `0x32b9d0(m, v, 0)`: the turn from the moby's facing to `v` (its yaw in the moby's frame, made positive).
pub fn turn_to(rows: &[V3; 3], v: V3) -> f32 {
    let l = crate::gravity::to_local(v, rows);
    crate::hero::physics::atan2(l[0], l[1]).abs()
}

/// The slots' pvars (the opponent's `+0x2a0`).
pub const SLOTS: usize = 0x2a0;
/// The skip table's start (`0x1af580`) and count.
pub const SKIP: [i32; 5] = [10, 27, 28, 31, 15];

/// A slot's points (its level path).
pub fn slot<'a>(w: &MobyWorld, ctx: &Ctx<'a>, boss: usize, i: usize) -> &'a [[f32; 4]] {
    let id = w.mobys[boss].pv_i(SLOTS + 4 * i);
    let level: &'a crate::level::LevelData = ctx.level;
    usize::try_from(id).ok().and_then(|k| level.paths.get(k)).map_or(&[], |p| p.as_slice())
}

fn p3(p: [f32; 4]) -> V3 { [p[0], p[1], p[2]] }

/// The mover's lift (`+0x110 + +0x114` of the opponent's block).
fn lift(w: &MobyWorld, boss: usize) -> f32 { w.mobys[boss].pv_f(0x114) + w.mobys[boss].pv_f(0x110) }

/// `0x3a3bb0(m, from, to)` (module docs); `m` the opponent, whose mover is read.
pub fn clear_walk(w: &MobyWorld, ctx: &Ctx, boss: usize, from: V3, to: V3) -> bool {
    if 50.0 < dist(from, to) { return false; }
    let (h, r) = (lift(w, boss), w.level22.radius);
    let end = surface(ctx, r, to, h);
    let step = set_len(sub(end, from), 8.0);
    let mut cursor = from;
    let mut prev = surface(ctx, r, from, h);
    loop {
        let d = dist(cursor, end);
        let last = d < 8.0;
        let next = if last { surface(ctx, r, add(cursor, set_len(step, d)), h) } else { surface(ctx, r, add(cursor, step), h) };
        if w.coll_line(ctx, prev, next, 0x12, None).is_some() { return false; }
        if last { return true; }
        prev = next;
        cursor = add(cursor, step);
    }
}

/// `0x3a5d28(k)`: slot `k` is not in the skip table.
pub fn passable(w: &MobyWorld, k: usize) -> bool {
    let g = &w.level22;
    !g.skip[..g.skip_n.clamp(0, 5) as usize].contains(&(k as i32))
}

/// `0x3a6098(_, k)`: slot `k` is in the skip table.
pub fn skipped(w: &MobyWorld, k: i32) -> bool {
    let g = &w.level22;
    g.skip[..g.skip_n.clamp(0, 5) as usize].contains(&k)
}

/// `0x3a4798` (module docs).
pub fn link_paths(w: &mut MobyWorld, ctx: &Ctx, boss: usize) {
    let ends: Vec<Option<(V3, V3)>> = (0..32).map(|i| {
        let p = slot(w, ctx, boss, i);
        Some((p3(*p.first()?), p3(*p.last()?)))
    }).collect();
    let ids: Vec<i32> = (0..32).map(|i| w.mobys[boss].pv_i(SLOTS + 4 * i)).collect();
    let mut links = [[0u32; 2]; 32];
    for i in 0..32 {
        let Some((a0, a1)) = ends[i] else { continue };
        for j in 0..32 {
            // The same record (a slot naming the same path) is left out.
            if ids[i] == ids[j] { continue; }
            let Some((b0, b1)) = ends[j] else { continue };
            if dist(a0, b0) < 1.0 || dist(a0, b1) < 1.0 { links[i][0] |= 1 << j; }
            if dist(a1, b0) < 1.0 || dist(a1, b1) < 1.0 { links[i][1] |= 1 << j; }
        }
    }
    w.level22.links = links;
}

/// `0x3a5d68` (module docs).
pub fn scan(w: &mut MobyWorld, ctx: &Ctx, boss: usize) {
    let n = w.level22.skip_n.clamp(1, 5);
    let old = w.level22.scan_at;
    loop {
        let g = &mut w.level22;
        g.scan_at = (g.scan_at + 1) % n;
        if g.skip[g.scan_at as usize] != -1 { break; }
        if old == g.scan_at { return; }
    }
    let k = w.level22.skip[w.level22.scan_at as usize];
    let p = slot(w, ctx, boss, k.clamp(0, 31) as usize);
    let (Some(&f), Some(&l)) = (p.first(), p.last()) else { return };
    let (a, c) = (p3(f), p3(l));
    let up = |q: V3| ctx.hero.gravity.up_at(q);
    let (h, i) = (unit(up(a)), unit(up(c)));
    let e = set_len(cross(sub(c, a), h), -1.5);
    let f2 = set_len(e, -1.5);
    let lift = lift(w, boss);
    let side = |p: V3, n: V3, off: V3| add(p, add(scale(n, lift), off));
    let ok1 = clear_walk(w, ctx, boss, side(a, h, f2), side(c, i, f2));
    if ok1 && clear_walk(w, ctx, boss, side(a, h, e), side(c, i, e)) {
        let g = &mut w.level22;
        g.skip[g.scan_at as usize] = -1;
    }
}

/// `0x3a3fc0(m, out, p, filter)` (module docs): the nearest point, none within 30000.
pub fn nearest_point(w: &MobyWorld, ctx: &Ctx, boss: usize, p: V3, filter: bool) -> Option<V3> {
    let mut best = 30000.0;
    let mut out = None;
    for i in 0..32 {
        if filter && !passable(w, i) { continue; }
        for q in slot(w, ctx, boss, i) {
            let d = dist(p3(*q), p);
            if d < best {
                best = d;
                out = Some(p3(*q));
            }
        }
    }
    out
}

/// `0x3a4240(m, k)`: slot `k` taken, the last three kept.
pub fn take_slot(w: &mut MobyWorld, boss: usize, k: usize) {
    let m = &mut w.mobys[boss];
    let old = m.pv_s(0x354);
    let i = (m.pv_s(0x35e) as i32 + 1) % 3;
    m.set_pv_s(0x35e, i as i16);
    m.set_pv_s(0x358 + 2 * i as usize, old);
    m.set_pv_s(0x354, k as i16);
}

/// `0x3a3e58(m, filter)` (module docs).
pub fn nearest_path(w: &mut MobyWorld, ctx: &Ctx, boss: usize, filter: bool) {
    let pos = w.mobys[boss].pos;
    let at = add(pos, set_len(sub(pos, ctx.hero.pos), 4.0));
    let mut best = 30000.0;
    for i in 0..32 {
        if filter && !passable(w, i) { continue; }
        let pts = slot(w, ctx, boss, i);
        for (k, q) in pts.iter().enumerate() {
            let d = dist(p3(*q), at);
            if d < best {
                best = d;
                take_slot(w, boss, i);
                w.mobys[boss].set_pv_s(0x356, k as i16);
            }
        }
    }
}

/// `0x3a4188(m, k, cur)`: the end of slot `k` linked to slot `cur` — 0 when its start is, else its last point.
fn linked_end(w: &MobyWorld, ctx: &Ctx, boss: usize, k: usize, cur: i32) -> usize {
    if (0..32).contains(&cur) && w.level22.links[k][0] >> cur & 1 != 0 { return 0; }
    slot(w, ctx, boss, k).len().saturating_sub(1)
}

/// `0x3a7558(bits)`: a dead end — the end links to one slot alone, and that one is in the skip table.
fn dead_end(w: &MobyWorld, bits: u32) -> bool { bits.count_ones() == 1 && !passable(w, bits.trailing_zeros() as usize) }

/// The opponent's goal (`+0x3ac`): Ratchet's place, or its point `+0x3b0`.
pub fn goal(w: &MobyWorld, ctx: &Ctx, boss: usize) -> V3 {
    let m = &w.mobys[boss];
    if m.pv_i(0x3ac) == 0 { ctx.hero.pos } else { m.pv_v(0x3b0) }
}

/// `0x3a4280(bits, m, mode, filter)` (module docs).
pub fn next_path(w: &mut MobyWorld, ctx: &mut Ctx, boss: usize, bits: u32, mode: i32, filter: bool) {
    let ids: Vec<usize> = (0..32).filter(|&k| bits >> k & 1 != 0 && (!filter || passable(w, k))).take(4).collect();
    if ids.is_empty() {
        let m = &mut w.mobys[boss];
        m.set_pv_s(0x352, -m.pv_s(0x352));
        return;
    }
    let cur = w.mobys[boss].pv_s(0x354) as i32;
    if mode == 0 {
        let id = ids[ctx.game.rng.randi(ids.len() as i32) as usize];
        let end = linked_end(w, ctx, boss, id, cur);
        let m = &mut w.mobys[boss];
        m.set_pv_s(0x352, if end == 0 { 1 } else { -1 });
        m.set_pv_s(0x356, end as i16);
        take_slot(w, boss, id);
        return;
    }
    let ends: Vec<usize> = ids.iter().map(|&k| linked_end(w, ctx, boss, k, cur)).collect();
    // The far end of each (and its link bits: the start's when the far end is the start).
    let far: Vec<(V3, u32)> = ids.iter().zip(&ends).map(|(&k, &e)| {
        let p = slot(w, ctx, boss, k);
        if e == 0 { (p.last().map_or([0.0; 3], |q| p3(*q)), w.level22.links[k][1]) } else { (p.first().map_or([0.0; 3], |q| p3(*q)), w.level22.links[k][0]) }
    }).collect();
    let n = ids.len();
    let (pos, rows) = (w.mobys[boss].pos, w.mobys[boss].rows);
    let goal = goal(w, ctx, boss);
    let mut sel = 0;
    match mode {
        1 => {
            let mut best = 30000.0;
            for (j, &(q, b)) in far.iter().enumerate() {
                let rejected = filter && dead_end(w, b);
                let s = dist(q, goal);
                if (s < best && !rejected) || n == 1 {
                    sel = j;
                    best = s;
                }
            }
        }
        3 => {
            let mut best = -30000.0;
            for (j, &(q, b)) in far.iter().enumerate() {
                let rejected = filter && dead_end(w, b);
                let s = dot(sub(ctx.hero.pos, pos), sub(q, pos));
                if (best < s && !rejected) || n == 1 {
                    sel = j;
                    best = s;
                }
            }
        }
        _ => {
            let m = &w.mobys[boss];
            let (hero_flat, own_flat) = (m.pv_v(0x3f0), m.pv_v(0x400));
            let up = rows[2];
            let across = |v: V3| unit(sub(v, scale(up, dot(v, up))));
            let mut best = 30000.0;
            for (j, &(q, b)) in far.iter().enumerate() {
                let blocked = filter && dead_end(w, b);
                let (e, f) = (across(unit(sub(hero_flat, own_flat))), across(unit(sub(q, own_flat))));
                let s = (dot(e, f) + 0.35).abs();
                if (s < best && !blocked) || n == 1 {
                    sel = j;
                    best = s;
                }
            }
        }
    }
    let m = &mut w.mobys[boss];
    m.set_pv_s(0x352, if ends[sel] == 0 { 1 } else { -1 });
    m.set_pv_s(0x356, ends[sel] as i16);
    take_slot(w, boss, ids[sel]);
}

/// `0x322510(out, a, b, p)` (L00 `0x31d038`): the point of the segment a → b nearest `p`.
fn on_segment(a: V3, b: V3, p: V3) -> V3 {
    let ab = sub(b, a);
    let t = (dot(ab, sub(p, a)) / dot(ab, ab)).clamp(0.0, 1.0);
    add(a, scale(ab, if t.is_nan() { 0.0 } else { t }))
}

/// `0x3a3308(m)` (module docs: the steer round what is ahead): a line from its lift up its row 2 to (4, −1.5..1.5,
/// lift) in its frame; a hit puts the point `+0x390` 30 from it across the face's normal (on the surface) and
/// `+0x378` on.
fn avoid(w: &mut MobyWorld, ctx: &mut Ctx, boss: usize) -> bool {
    let h = lift(w, boss);
    let r = ctx.game.rng.randi(8) as f32 * f32::from_bits(0x3edb_6db7) - 1.5;
    let m = &w.mobys[boss];
    let (pos, rows) = (m.pos, m.rows);
    let start = add(scale(rows[2], h), pos);
    let end = add(crate::gravity::to_world([4.0, r, h], &rows), pos);
    let Some(hit) = w.coll_line(ctx, start, end, 0x12, None) else { return false };
    let axis = unit(hit.normal);
    let d = sub(start, hit.point);
    let off = set_len(sub(d, scale(axis, dot(d, axis))), -30.0);
    let p = surface(ctx, w.level22.radius, add(pos, off), 0.0);
    let m = &mut w.mobys[boss];
    m.set_pv_v(0x390, p);
    m.set_pv_s(0x378, 1);
    true
}

/// `0x3a3590(m)` (module docs).
pub fn follow(w: &mut MobyWorld, ctx: &mut Ctx, boss: usize) {
    let radius = w.level22.radius;
    let goal_hero = w.mobys[boss].pv_i(0x3ac) == 0;
    let goal = goal(w, ctx, boss);
    let mut target = goal;
    let flat = surface(ctx, radius, goal, 0.0);
    let pos = w.mobys[boss].pos;
    let hit = if goal_hero { w.mobys[boss].pv_i(0x3d8) != 0 } else { clear_walk(w, ctx, boss, pos, goal) };
    if !hit && w.mobys[boss].pv_s(0x350) == 0 { w.mobys[boss].set_pv_s(0x350, 1); }
    let st = w.mobys[boss].pv_s(0x350);
    let done = match st {
        2 => clear_walk(w, ctx, boss, pos, w.mobys[boss].pv_v(0x360)),
        4 => hit,
        _ if goal_hero && w.mobys[boss].pv_i(0x3d8) != 0 => true,
        0 => hit,
        3 => true,
        _ => false,
    };
    if done {
        w.mobys[boss].set_pv_s(0x378, 0);
    } else if w.mobys[boss].pv_s(0x378) != 0 && w.mobys[boss].pv_i(0x190) < 5 {
        let a = w.mobys[boss].pv_v(0x390);
        if dist(pos, a) < 10.0 { w.mobys[boss].set_pv_s(0x378, 0); }
        target = a;
    } else {
        if avoid(w, ctx, boss) {
            target = w.mobys[boss].pv_v(0x390);
            w.mobys[boss].set_pv_v(0x3c0, pos);
        }
        let m = &mut w.mobys[boss];
        m.set_pv_s(0x384, (m.pv_s(0x384) as u16).wrapping_add(1) as i16);
    }
    match w.mobys[boss].pv_s(0x350) {
        1 => {
            nearest_path(w, ctx, boss, true);
            let (k, pt) = (w.mobys[boss].pv_s(0x354).clamp(0, 31) as usize, w.mobys[boss].pv_s(0x356) as i32);
            let pts = slot(w, ctx, boss, k);
            // `rec + n` of the game: point n − 1, the header (taken as the origin) for −1.
            let at = |n: i32| if n < 0 { [0.0; 3] } else { pts.get(n as usize).map_or([0.0; 3], |q| p3(*q)) };
            let here = at(pt);
            w.mobys[boss].set_pv_s(0x350, 2);
            let d = dist(here, goal);
            let count = pts.len() as i32;
            let (dir, wp): (i16, V3) = if pt == 0 {
                if dist(at(1), goal) < d { (1, on_segment(here, at(pt + 1), pos)) } else { (-1, here) }
            } else if pt == count - 1 {
                if dist(at(count - 2), goal) < d { (-1, on_segment(here, at(pt - 1), pos)) } else { (1, here) }
            } else {
                let dir = if dist(at(pt - 1), goal) < dist(at(pt + 1), goal) { -1 } else { 1 };
                (dir, on_segment(here, at(dir as i32), pos))
            };
            // The target stays as it was (the goal or the steer's point) in this state.
            let m = &mut w.mobys[boss];
            m.set_pv_s(0x352, dir);
            m.set_pv_v(0x360, surface(ctx, radius, wp, 0.0));
        }
        2 => {
            let wp = w.mobys[boss].pv_v(0x360);
            if dist(pos, wp) < 2.0 {
                w.mobys[boss].set_pv_s(0x350, 3);
                return finish(w, ctx, boss, target);
            }
            target = wp;
        }
        3 => {
            let m = &w.mobys[boss];
            if dist(m.pv_v(0x400), flat) < 20.0 && (m.pv_i(0x418) != 0 || hit) {
                w.mobys[boss].set_pv_s(0x350, 4);
            } else {
                let wp = w.mobys[boss].pv_v(0x360);
                if dist(pos, wp) < 2.0 {
                    let m = &w.mobys[boss];
                    let k = m.pv_s(0x354).clamp(0, 31) as usize;
                    let next = m.pv_s(0x356) as i32 + m.pv_s(0x352) as i32;
                    let count = slot(w, ctx, boss, k).len() as i32;
                    if next < 0 {
                        let bits = w.level22.links[k][0];
                        next_path(w, ctx, boss, bits, 1, false);
                    } else if count <= next {
                        let bits = w.level22.links[k][1];
                        next_path(w, ctx, boss, bits, 1, false);
                    } else {
                        w.mobys[boss].set_pv_s(0x356, next as i16);
                    }
                    let m = &w.mobys[boss];
                    let (k, pt) = (m.pv_s(0x354).clamp(0, 31) as usize, m.pv_s(0x356).max(0) as usize);
                    let q = slot(w, ctx, boss, k).get(pt).map_or(wp, |q| p3(*q));
                    w.mobys[boss].set_pv_v(0x360, surface(ctx, radius, q, 0.0));
                }
                target = w.mobys[boss].pv_v(0x360);
            }
        }
        4 => w.mobys[boss].set_pv_s(0x350, 0),
        _ => {}
    }
    finish(w, ctx, boss, target)
}

/// The walk's end: the move toward the target and the turn left (`+0x43c`).
fn finish(w: &mut MobyWorld, ctx: &mut Ctx, boss: usize, target: V3) {
    crate::moby::frames::move_to(w, boss, ctx, target);
    let m = &mut w.mobys[boss];
    let a = turn_to(&m.rows, sub(target, m.pos));
    m.set_pv_f(0x43c, a);
}

/// `0x3a6318(m)` (module docs).
pub fn beside_hero(w: &mut MobyWorld, ctx: &Ctx, boss: usize) {
    let r = w.level22.radius;
    let m = &w.mobys[boss];
    let hero_flat = m.pv_v(0x3f0);
    let b0 = set_len(sub(m.pos, hero_flat), r * 4.0);
    let hero_up = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or([0.0, 0.0, 1.0], |h| h.rows[2]);
    let b1 = scale(hero_up, -r * 4.0);
    let b2 = add(scale(add(b0, b1), 0.5), hero_flat);
    let p = nearest_point(w, ctx, boss, b2, true).unwrap_or(m.pv_v(0x3b0));
    let m = &mut w.mobys[boss];
    m.set_pv_v(0x3b0, surface(ctx, r, p, 0.0));
    m.set_pv_i(0x3ac, 1);
}

/// The length of `v` (`0x2f4930`).
pub fn length(v: V3) -> f32 { len(v) }
