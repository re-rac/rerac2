//! Damosel orbit's planetoid and its walks (L23 addresses; the decomp's `func_L23_00418650` … `func_L23_0041D0A0` and
//! the enemies' copies `func_L23_003C02C8` … `func_L23_003C0F88`). The opponent 0x0f7f
//! (crate::moby::classes::level23::boss) and its walkers 0x07e5 (crate::moby::classes::level23::drones) walk the
//! level's paths its pvars name: 47 slots at `+0x230` (`0x1b3628` points at them), each a level path (`0x1ff280[id]`).
//! Dobbo orbit's fight has its own copies of these (crate::moby::classes::level22::planet: 32 slots, link bits in the
//! points' w); this level's are other functions.
//!
//! * **The surface** ([`surface`], `0x419b50(p, h)`): `p` moved along its line from the sphere's centre (`0x189ed0`)
//!   to `h` above its radius (`0x1b3624`: the body 0x027d's distance from the centre at the set-up).
//! * **The links** ([`link_paths`], `0x41abb8`, at the set-up): for each slot's path, a bit for every other slot whose
//!   path has an end within 1 of its start (`0x259548[id]`, the first word) or of its end (the second); the end met
//!   is moved onto the other's (the level's path data is changed in place, here the fight's copy of it).
//! * **The skip table** (`$gp+0x2618`, its count `0x1b3840` 4, copied from `0x1b1770` at the set-up: slots 41, 42,
//!   43, 44): slots the walks may leave out ([`passable`], `0x41b1a0`). **The scan** ([`scan`], `0x41ae70`, each tick
//!   of the opponent): the next entry still in the table (`$gp+0x262c`, cyclic); both sides of its slot's line (1.5
//!   across its way at its two ends, lifted by the mover's radius and height) clear ([`clear`]) → it leaves the table.
//! * **The clear test** ([`clear`], `0x41a0c8` / the walkers' `0x3c1ea0`): no more than 70 (50) apart, and every
//!   step of 8 from `from` to `to`, each point on the surface lifted by the mover's radius and height (`+0x50 +
//!   +0x54`), free of a line 0x12 (the walkers': a line 0x10 that meets nothing but the walker itself or Giant Clank).
//! * **The nearest slot** ([`nearest_slot`], `0x41a2c0` / `0x3c20b8`): of every point of the slots (the table passing
//!   them) the one nearest the point 4 past the walker from Giant Clank → its slot taken and the point.
//! * **The walk** ([`walk`], `0x419bd0(m, h)` / the walkers' in crate::moby::classes::level23::drones): the block
//!   `mode`, its way, the slot, the point, the waypoint (module [`Walk`]). **1**: the nearest slot, the way along it
//!   that brings it nearer the goal, the waypoint the nearest point to it of that segment, put `h` up → 2. **2**: to
//!   the waypoint; within 2 → 3. **3**: within 2 of the waypoint the next point its way, past an end the next slot
//!   ([`next_slot`] mode 1); → the waypoint. **4**: → 0 (0 is 1 for the opponent).
//! * **The next slot** ([`next_slot`], `0x41a5c8(m, bits, mode, check)` / `0x3c23c8`): the slots `bits` names (the
//!   table passing them with `check`); none: the way turned back. Mode 0 a random one; 1 the one whose far end is
//!   nearest the goal; 3 the one whose far end is most toward Giant Clank; 2 (by the current slot 0x22..0x25: the one
//!   most away from him) the one whose way is nearest 0.35 off the way to him (straightened in the frame about the
//!   up). A far end with a single link to a skipped slot is refused with `check` (`0x41b1e0`). The slot taken
//!   (`0x41a588`: the last three kept) at its linked end, the way into it.
//!
//! The game's mode 2 straightens both ways about the up and the gravity frame (`0x325600`) before their dot: here the
//! ways put across the up and made unit.

use crate::hero::physics::{add, cross, dist, dot, scale, set_len, sub, unit, V3};
use crate::moby::{Ctx, MobyWorld};

/// The slots' pvars (the opponent's `+0x230`) and their count (`$gp+0x2634`, the decomp's `D_L23_001B1684`).
pub const SLOTS: usize = 0x230;
pub const N: usize = 47;
/// `0x1b1770`: the skip table's start; its count.
pub const SKIP: [i32; 4] = [41, 42, 43, 44];

/// The planetoid's paths as the fight keeps them: the slots' level paths (their points snapped by [`link_paths`]),
/// their link words and the skip table.
#[derive(Clone, Debug, Default)]
pub struct Paths {
    /// Each slot's path id and its points.
    pub ids: Vec<i32>,
    pub points: Vec<Vec<V3>>,
    /// `0x259548[id]`: the slots linked at its start (`[0]`) and its end (`[1]`).
    pub links: Vec<[u64; 2]>,
    /// `$gp+0x2618`, `0x1b3840`, `$gp+0x262c`.
    pub skip: [i32; 4],
    pub skip_n: i32,
    pub scan_at: i32,
}

/// The block of a walk in its walker's pvars.
#[derive(Clone, Copy)]
pub struct Walk {
    /// The walk's state, its way along the slot (±1), the slot, the point, the last slots (three shorts) and their
    /// index, the waypoint.
    pub mode: usize,
    pub dir: usize,
    pub slot: usize,
    pub point: usize,
    pub hist: usize,
    pub hist_i: usize,
    pub way: usize,
}

/// The opponent's (`+0x390..+0x3a0`).
pub const BOSS: Walk = Walk { mode: 0x390, dir: 0x392, slot: 0x394, point: 0x396, hist: 0x398, hist_i: 0x39e, way: 0x3a0 };
/// The walkers' (`+0x2b0..+0x2c0`).
pub const DRONE: Walk = Walk { mode: 0x2b0, dir: 0x2b2, slot: 0x2b4, point: 0x2b6, hist: 0x2b8, hist_i: 0x2be, way: 0x2c0 };

/// `0x419b50(p, h)` (module docs).
pub fn surface(ctx: &Ctx, radius: f32, p: V3, h: f32) -> V3 {
    let c = ctx.hero.gravity.centre;
    add(set_len(sub(p, c), h + radius), c)
}

fn p3(p: [f32; 4]) -> V3 { [p[0], p[1], p[2]] }

/// The paths copied from the opponent's slots at its set-up (`+0x230`).
pub fn load(w: &mut MobyWorld, ctx: &Ctx, boss: usize) {
    let ids: Vec<i32> = (0..N).map(|i| w.mobys[boss].pv_i(SLOTS + 4 * i)).collect();
    let points = ids.iter().map(|&id| usize::try_from(id).ok().and_then(|k| ctx.level.paths.get(k)).map_or(Vec::new(), |p| p.iter().map(|q| p3(*q)).collect())).collect();
    w.level23.paths = Paths { ids, points, links: vec![[0; 2]; N], skip: SKIP, skip_n: SKIP.len() as i32, scan_at: 0 };
}

/// Slot `i`'s points.
pub fn slot(w: &MobyWorld, i: usize) -> &[V3] { w.level23.paths.points.get(i).map_or(&[], |p| p.as_slice()) }

/// `0x41abb8` (module docs).
pub fn link_paths(w: &mut MobyWorld) {
    let p = &mut w.level23.paths;
    let n = p.points.len();
    for i in 0..n {
        let mut bits = [0u64; 2];
        for j in 0..n {
            if p.ids[i] == p.ids[j] || p.points[i].is_empty() || p.points[j].is_empty() { continue; }
            let (a0, a1) = (p.points[i][0], *p.points[i].last().unwrap());
            let (b0, b1) = (p.points[j][0], *p.points[j].last().unwrap());
            let d = [dist(a0, b0), dist(a0, b1), dist(a1, b0), dist(a1, b1)];
            let mut first = None;
            let mut last = None;
            for k in 0..2 {
                if d[k] < 1.0 {
                    bits[0] |= 1 << j;
                    first = Some(if k == 0 { b0 } else { b1 });
                }
            }
            for k in 2..4 {
                if d[k] < 1.0 {
                    bits[1] |= 1 << j;
                    last = Some(if k == 2 { b0 } else { b1 });
                }
            }
            // The path's ends moved in place: every slot naming the same path sees it.
            let id = p.ids[i];
            for s in 0..n {
                if p.ids[s] != id || p.points[s].is_empty() { continue; }
                if let Some(f) = first { p.points[s][0] = f; }
                if let Some(l) = last { *p.points[s].last_mut().unwrap() = l; }
            }
        }
        let id = p.ids[i];
        for s in 0..n {
            if p.ids[s] == id { p.links[s] = bits; }
        }
    }
}

/// `0x41b1a0(k)`: slot `k` is not in the skip table.
pub fn passable(w: &MobyWorld, k: usize) -> bool {
    let p = &w.level23.paths;
    !p.skip[..p.skip_n.clamp(0, 4) as usize].contains(&(k as i32))
}

/// `0x41b1e0(bits)`: a dead end — one slot linked, and that one skipped.
fn dead_end(w: &MobyWorld, bits: u64) -> bool { bits.count_ones() == 1 && !passable(w, bits.trailing_zeros() as usize) }

/// The mover's lift (`+0x50 + +0x54` of the walker's block).
pub fn lift(w: &MobyWorld, id: usize) -> f32 { w.mobys[id].pv_f(0x54) + w.mobys[id].pv_f(0x50) }

/// `0x41a0c8(m, from, to)` (`walker` false) / `0x3c1ea0` (`walker` true) (module docs).
pub fn clear(w: &MobyWorld, ctx: &Ctx, id: usize, from: V3, to: V3, walker: bool) -> bool {
    let (max, flags) = if walker { (50.0, 0x10) } else { (70.0, 0x12) };
    if max < dist(from, to) { return false; }
    let (h, r) = (lift(w, id), w.level23.radius);
    let end = surface(ctx, r, to, h);
    let step = set_len(sub(end, from), 8.0);
    let mut cursor = from;
    let mut prev = surface(ctx, r, from, h);
    loop {
        let d = dist(cursor, end);
        let last = d < 8.0;
        let next = if last { surface(ctx, r, add(cursor, set_len(step, d)), h) } else { surface(ctx, r, add(cursor, step), h) };
        if let Some(hit) = w.coll_line(ctx, prev, next, flags, None) {
            if !walker || !(hit.moby == Some(id) || (hit.moby.is_some() && hit.moby == ctx.hero_moby)) { return false; }
        }
        if last { return true; }
        prev = next;
        cursor = add(cursor, step);
    }
}

/// `0x41ae70` (module docs).
pub fn scan(w: &mut MobyWorld, ctx: &Ctx, boss: usize) {
    let p = &mut w.level23.paths;
    let n = p.skip_n.clamp(1, 4);
    let old = p.scan_at;
    loop {
        p.scan_at = (p.scan_at + 1) % n;
        if p.skip[p.scan_at as usize] != -1 { break; }
        if old == p.scan_at { return; }
    }
    let k = p.skip[p.scan_at as usize];
    let pts = slot(w, k.clamp(0, N as i32 - 1) as usize);
    let (Some(&a), Some(&c)) = (pts.first(), pts.last()) else { return };
    let up = |q: V3| ctx.hero.gravity.up_at(q);
    let (h, i) = (unit(up(a)), unit(up(c)));
    let e = set_len(cross(sub(c, a), h), -1.5);
    let f = set_len(e, -1.5);
    let lift = lift(w, boss);
    let side = |p: V3, n: V3, off: V3| add(p, add(scale(n, lift), off));
    if clear(w, ctx, boss, side(a, h, f), side(c, i, f), false) && clear(w, ctx, boss, side(a, h, e), side(c, i, e), false) {
        let p = &mut w.level23.paths;
        p.skip[p.scan_at as usize] = -1;
    }
}

/// `0x41a588(m, k)` / `0x3c2388`: slot `k` taken, the last three kept.
pub fn take_slot(w: &mut MobyWorld, id: usize, b: Walk, k: usize) {
    let m = &mut w.mobys[id];
    let old = m.pv_s(b.slot);
    let i = (m.pv_s(b.hist_i) as i32 + 1) % 3;
    m.set_pv_s(b.hist_i, i as i16);
    m.set_pv_s(b.hist + 2 * i as usize, old);
    m.set_pv_s(b.slot, k as i16);
}

/// `0x41a2c0(m, filter)` / `0x3c20b8` (module docs).
pub fn nearest_slot(w: &mut MobyWorld, ctx: &Ctx, id: usize, b: Walk, filter: bool) {
    let pos = w.mobys[id].pos;
    let at = add(pos, set_len(sub(pos, ctx.hero.pos), 4.0));
    let mut best = 30000.0;
    for i in 0..w.level23.paths.points.len() {
        if filter && !passable(w, i) { continue; }
        for k in 0..slot(w, i).len() {
            let d = dist(slot(w, i)[k], at);
            if d < best {
                best = d;
                take_slot(w, id, b, i);
                w.mobys[id].set_pv_s(b.point, k as i16);
            }
        }
    }
}

/// `0x41c6b0(out, p, filter)`: the nearest point of the slots to `p` (none within 30000: `None`).
pub fn nearest_point(w: &MobyWorld, p: V3, filter: bool) -> Option<V3> {
    let mut best = 30000.0;
    let mut out = None;
    for i in 0..w.level23.paths.points.len() {
        if filter && !passable(w, i) { continue; }
        for &q in slot(w, i) {
            let d = dist(q, p);
            if d < best {
                best = d;
                out = Some(q);
            }
        }
    }
    out
}

/// `0x41e5a0(out, 1, h)`: a random slot with none of its points within 45 of Giant Clank and passing the table, a
/// random point of it but its last, `h` up. The port's: at most 1000 draws (the game draws until one passes).
pub fn random_point(w: &MobyWorld, ctx: &mut Ctx, h: f32) -> V3 {
    let hero = ctx.hero.pos;
    for _ in 0..1000 {
        let i = ctx.game.rng.randi(N as i32) as usize;
        let pts = slot(w, i);
        if pts.iter().any(|&q| dist(hero, q) < 45.0) || !passable(w, i) { continue; }
        let k = ctx.game.rng.randi(pts.len() as i32 - 1).max(0) as usize;
        let Some(&q) = pts.get(k) else { continue };
        return surface(ctx, w.level23.radius, q, h);
    }
    surface(ctx, w.level23.radius, hero, h)
}

/// `0x41a4d0(m, k, cur)`: the end of slot `k` linked to slot `cur` — 0 when its start is, else its last point.
fn linked_end(w: &MobyWorld, k: usize, cur: i32) -> usize {
    let p = &w.level23.paths;
    if (0..64).contains(&cur) && p.links.get(k).is_some_and(|l| l[0] >> cur & 1 != 0) { return 0; }
    slot(w, k).len().saturating_sub(1)
}

/// `0x41a5c8(m, bits, mode, check)` (module docs); `goal` the walk's goal (mode 1), `flat` Giant Clank's and the
/// walker's places on the surface (mode 2).
pub fn next_slot(w: &mut MobyWorld, ctx: &mut Ctx, id: usize, b: Walk, bits: u64, mode: i32, check: bool, goal: V3, flat: (V3, V3)) {
    let ids: Vec<usize> = (0..64).filter(|&k| bits >> k & 1 != 0 && k < N && (!check || passable(w, k))).collect();
    if ids.is_empty() {
        let m = &mut w.mobys[id];
        m.set_pv_s(b.dir, -m.pv_s(b.dir));
        return;
    }
    let cur = w.mobys[id].pv_s(b.slot) as i32;
    if mode == 0 {
        let k = ids[ctx.game.rng.randi(ids.len() as i32) as usize];
        let end = linked_end(w, k, cur);
        let m = &mut w.mobys[id];
        m.set_pv_s(b.dir, if end == 0 { 1 } else { -1 });
        m.set_pv_s(b.point, end as i16);
        take_slot(w, id, b, k);
        return;
    }
    let ends: Vec<usize> = ids.iter().map(|&k| linked_end(w, k, cur)).collect();
    let far: Vec<(V3, u64)> = ids.iter().zip(&ends).map(|(&k, &e)| {
        let p = slot(w, k);
        let l = w.level23.paths.links[k];
        if e == 0 { (p.last().copied().unwrap_or([0.0; 3]), l[1]) } else { (p.first().copied().unwrap_or([0.0; 3]), l[0]) }
    }).collect();
    let n = ids.len();
    let (pos, rows) = (w.mobys[id].pos, w.mobys[id].rows);
    let hero = ctx.hero.pos;
    let mut sel = 0;
    let mut pick = |score: &dyn Fn(V3) -> f32, more: bool| {
        let mut best = if more { -30000.0 } else { 30000.0 };
        for (j, &(q, l)) in far.iter().enumerate() {
            let refused = check && dead_end(w, l);
            let s = score(q);
            if ((if more { best < s } else { s < best }) && !refused) || n == 1 {
                sel = j;
                best = s;
            }
        }
    };
    match mode {
        1 => pick(&|q| dist(q, goal), false),
        3 => pick(&|q| dot(sub(hero, pos), sub(q, pos)), true),
        _ if (0x22..=0x25).contains(&cur) => pick(&|q| dot(sub(hero, pos), sub(q, pos)), false),
        _ => {
            let up = rows[2];
            let across = |v: V3| unit(sub(v, scale(up, dot(v, up))));
            let (hero_flat, own_flat) = flat;
            pick(&|q| (dot(across(unit(sub(hero_flat, own_flat))), across(unit(sub(q, own_flat)))) + 0.35).abs(), false);
        }
    }
    let m = &mut w.mobys[id];
    m.set_pv_s(b.dir, if ends[sel] == 0 { 1 } else { -1 });
    m.set_pv_s(b.point, ends[sel] as i16);
    take_slot(w, id, b, ids[sel]);
}

/// `0x31bdc0(out, a, b, p)` (L00 `0x31d038`): the point of the segment a → b nearest `p`.
pub fn on_segment(a: V3, b: V3, p: V3) -> V3 {
    let ab = sub(b, a);
    let t = (dot(ab, sub(p, a)) / dot(ab, ab)).clamp(0.0, 1.0);
    add(a, scale(ab, if t.is_nan() { 0.0 } else { t }))
}

/// The walk's mode 1 (module docs): the nearest slot, its way and the waypoint, `h` up.
pub fn start_walk(w: &mut MobyWorld, ctx: &Ctx, id: usize, b: Walk, goal: V3, h: f32) {
    nearest_slot(w, ctx, id, b, true);
    let r = w.level23.radius;
    let goal = surface(ctx, r, goal, 0.0);
    let m = &w.mobys[id];
    let (k, i, pos) = (m.pv_s(b.slot).max(0) as usize, m.pv_s(b.point).max(0) as usize, m.pos);
    let pts = slot(w, k).to_vec();
    let Some(&node) = pts.get(i) else { return };
    let d = dist(node, goal);
    let n = pts.len();
    let (dir, way) = if i == 0 {
        if n > 1 && dist(pts[1], goal) < d { (1, on_segment(node, pts[1], pos)) } else { (-1, node) }
    } else if i == n - 1 {
        if dist(pts[n - 2], goal) < d { (-1, on_segment(node, pts[n - 2], pos)) } else { (1, node) }
    } else {
        let dir = if dist(pts[i - 1], goal) < dist(pts[i + 1], goal) { -1 } else { 1 };
        // The game's: the segment from a middle point goes to the slot's point `dir` (its header for −1).
        let other = if dir == 1 { pts[1] } else { node };
        (dir, on_segment(node, other, pos))
    };
    let way = surface(ctx, r, way, h);
    let m = &mut w.mobys[id];
    m.set_pv_s(b.dir, dir);
    m.set_pv_v(b.way, way);
    m.set_pv_s(b.mode, 2);
}

/// The walk's mode 3 step at the waypoint: the next point its way, past an end the next slot (mode 1 to `goal`),
/// the waypoint there `h` up.
pub fn next_point(w: &mut MobyWorld, ctx: &mut Ctx, id: usize, b: Walk, goal: V3, h: f32, flat: (V3, V3)) {
    let m = &w.mobys[id];
    let (k, i, dir) = (m.pv_s(b.slot).max(0) as usize, m.pv_s(b.point) as i32, m.pv_s(b.dir) as i32);
    let next = i + dir;
    let links = w.level23.paths.links.get(k).copied().unwrap_or([0; 2]);
    if next < 0 {
        next_slot(w, ctx, id, b, links[0], 1, true, goal, flat);
    } else if slot(w, k).len() as i32 <= next {
        next_slot(w, ctx, id, b, links[1], 1, true, goal, flat);
    } else {
        w.mobys[id].set_pv_s(b.point, next as i16);
    }
    let m = &w.mobys[id];
    let (k, i) = (m.pv_s(b.slot).max(0) as usize, m.pv_s(b.point).max(0) as usize);
    let Some(&q) = slot(w, k).get(i) else { return };
    let q = surface(ctx, w.level23.radius, q, h);
    w.mobys[id].set_pv_v(b.way, q);
}

/// `0x419bd0(m, h)`: the opponent's walk to its goal (`+0x400`, `goal`) `h` up (module docs); returns the point it
/// heads for this tick.
pub fn walk(w: &mut MobyWorld, ctx: &mut Ctx, id: usize, goal: V3, h: f32, flat: (V3, V3)) -> V3 {
    let b = BOSS;
    let r = w.level23.radius;
    let mut target = goal;
    if w.mobys[id].pv_s(b.mode) != 0 {
        let way = surface(ctx, r, w.mobys[id].pv_v(b.way), w.mobys[id].pv_f(0x588));
        w.mobys[id].set_pv_v(b.way, way);
    }
    if w.mobys[id].pv_s(b.mode) == 0 { w.mobys[id].set_pv_s(b.mode, 1); }
    w.mobys[id].set_pv_s(0x406, 0);
    match w.mobys[id].pv_s(b.mode) {
        1 => start_walk(w, ctx, id, b, goal, h),
        2 => {
            let way = w.mobys[id].pv_v(b.way);
            if dist(w.mobys[id].pos, way) < 2.0 { w.mobys[id].set_pv_s(b.mode, 3); } else { target = way; }
        }
        3 => {
            if dist(w.mobys[id].pos, w.mobys[id].pv_v(b.way)) < 2.0 { next_point(w, ctx, id, b, goal, h, flat); }
            target = w.mobys[id].pv_v(b.way);
        }
        4 => w.mobys[id].set_pv_s(b.mode, 0),
        _ => {}
    }
    target
}
