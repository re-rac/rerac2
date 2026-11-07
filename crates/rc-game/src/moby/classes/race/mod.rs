//! The hoverbike races of Barlow (level 4) and Joba (level 11): one system the original runs on both levels (the same
//! functions at L04 / L11 places; L04 addresses below, the L11 ones in the census and in [`update_for`]'s list).
//!
//! * [`bike`]: Ratchet's hoverbike **0x0bbc** (L04 `0x41b020`, L11 `0x40ab70`), the race master: the mount and the
//!   dismount, the race select, the countdown, the riding, the laps, the finish, the respawn on the track.
//! * [`racers`]: the racers **0x0b2d** (L04 `0x405aa0`, L11 `0x3f0b58`; 19 placed on each level) driving their routes.
//! * [`jets`]: the boost jets **0x0474** / **0x0cef** (L04 `0x3c9760`, L11 `0x3b9738`): five flames streaming along
//!   each; a racer over one is pushed on (racers::jets, `0x403c70`).
//!
//! **The routes.** A route is a level path (`0x1fe380[i]`, L11 `0x1ff600[i]`: `count`, then the points); the race
//! code reads its points' w as its own words: point 0's the step between the points (written `dist(p0, p1)` at the
//! set-ups `0x404148` / `0x410ae0`), any other point's ±1000 or more a marker (a new random lateral offset, or none).
//! A branch of the bike's route keeps its joins in its points 1..4 (`0x410ae0`): 1 the index on the route it leaves
//! nearest its first point, 2 the index on the route it rejoins nearest its last, 3 / 4 those as distances along
//! them. The original writes these into the level's path data; the port keeps patched copies in [`Paths`].
//!
//! **Positions on a route** ([`along`], `0x367640` = boot `0x2c96f8`): the step's multiple `t` along the points
//! (clamped to the first and the last). A follower keeps `t` (`+0xa8`), the index and the fraction (`+0x114` /
//! `+0x118`) and the point (`+0xf0`).
//!
//! **The race words** ([`Globals`]): the race chosen (L04 `$gp+0x18cc`, L11 `$gp+0x185c`, written to
//! crate::moby::classes::level11::Globals::event too), the one picked in the select (`$gp+0x18c4` / `$gp+0x1854`),
//! Ratchet's lap `0x1a8fe4`, the race clock `0x1a8fe8` (ticks), and the gp words the master and the racers share.
//!
//! **The records** ([`RECORDS_04`] / [`RECORDS_11`], L04 `0x2c64c0`, L11 `0x2b6e40`, 0x40 bytes each): the title text,
//! the flags (bit 0 the pickups 0x0ab8 out, bit 1 the draft, bit 2 the short cut route `+0x6c` and the gates `+0x23c`
//! shut, 0x10000000 open from the start), the three bolt prizes and the racers' time (`+0x14`, in 1/180 s).

pub mod bike;
pub mod jets;
pub mod racers;

use crate::collision::{coll_line_m, coll_sphere_m, QueryFlags};
use crate::hero::physics::{add, dot, len, scale, set_len, sub, V3};
use crate::moby::{Ctx, Moby, MobyWorld};
use std::collections::HashMap;

pub const BIKE: i16 = 0x0bbc;
pub const RACER: i16 = 0x0b2d;
pub const JET: i16 = 0x0474;
pub const JET_B: i16 = 0x0cef;
/// The racers' riders (`0x3e5e40(m, 0x8ef, 0.25)`).
pub const RIDER: i16 = 0x08ef;

/// This system's class updates (registered once for both levels).
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        BIKE => Some(bike::update),
        RACER => Some(racers::update),
        JET | JET_B => Some(jets::update),
        _ => None,
    }
}

/// A race record (module docs).
#[derive(Clone, Copy, Debug)]
pub struct Record {
    pub text: u16,
    pub flags: u32,
    pub prizes: [i32; 3],
    pub time: i32,
}

pub const RECORDS_04: [Record; 5] = [
    Record { text: 0x2e0a, flags: 0x1000_0008, prizes: [0, 0xfa, 0xfa], time: 0x2454 },
    Record { text: 0x2e0b, flags: 1, prizes: [0x7d0, 0x2ee, 0xfa], time: 0x2454 },
    Record { text: 0x2e0c, flags: 5, prizes: [0xbb8, 0x3e8, 0x15e], time: 0x2058 },
    Record { text: 0x2e0d, flags: 7, prizes: [0xfa0, 0x4b0, 0x190], time: 0x2058 },
    Record { text: 0x2e0e, flags: 7, prizes: [0x1770, 0x7d0, 0x2ee], time: 0x1fa4 },
];
pub const RECORDS_11: [Record; 5] = [
    Record { text: 0x2e0a, flags: 0x1000_0019, prizes: [0, 0x190, 0x190], time: 0x25bc },
    Record { text: 0x2e0b, flags: 1, prizes: [0xfa0, 0x4b0, 0x190], time: 0x2508 },
    Record { text: 0x2e2e, flags: 5, prizes: [0x1388, 0x6d6, 0x258], time: 0x23dc },
    Record { text: 0x2e0d, flags: 7, prizes: [0x1770, 0x7d0, 0x2ee], time: 0x22ec },
    Record { text: 0x2e0e, flags: 7, prizes: [0x1f40, 0xabe, 0x384], time: 0x21c0 },
];

/// The record of race `k` on this level.
pub fn record(level: i32, k: i32) -> Record {
    let t = if level == 0xb { &RECORDS_11 } else { &RECORDS_04 };
    t[k.clamp(0, 4) as usize]
}

/// The race words (module docs).
#[derive(Clone, Debug, Default)]
pub struct Globals {
    /// The routes as the race code patches them (module docs).
    pub paths: Paths,
    /// The race chosen and the one picked in the select.
    pub race: i32,
    pub pick: i32,
    /// `0x1a8fe4`: Ratchet's lap; `0x1a8fe8`: the race clock.
    pub lap: i32,
    pub clock: i32,
    /// `$gp+0x18d4` (L04): the master (state 0 writes it).
    pub bike: Option<usize>,
    /// `$gp+0x18bc`: the race is entered from the level's load (`0x1a7904` = 3): the set-up mounts at once.
    pub entered: bool,
    /// `$gp+0x18e4`: a race asked for again (the restart's way back to the countdown).
    pub again: bool,
    /// `$gp+0x18f0`: the start boost may be taken (✕ not held at the countdown's end).
    pub start_boost: bool,
    /// `$gp+0x18f4`: the leader's fanfare asked for (the sound 0xa26).
    pub fanfare: bool,
    /// `$gp+0x18c8`: the select's restart (1) / quit to the countdown (2), from the pause menu; `$gp+0x18d0`: the
    /// race left from the pause menu; `$gp+0x18c0`: the finish's restart.
    pub restart: i32,
    pub quit: bool,
    pub retry: bool,
    /// `$gp+0x18b0`: on a water surface (7) this tick.
    pub on_water: bool,
    /// `$gp+0x18b8`: the ticks since the finish.
    pub finished: i32,
    /// L04 `0x1b0978`: the select open (its first tick done).
    pub select_open: bool,
    /// `$gp+0x18ec`: cleared at the countdown's start (nothing ported reads it).
    pub g18ec: i32,
    /// `$gp+0x1bac`: the select's camera flag (its z kept at 0.92 on the way out).
    pub select_cam: bool,
    /// `$gp+0x1bb0`: the master hidden by a scene.
    pub hidden: bool,
    /// `$gp+0x4224`: the countdown's step.
    pub count: i32,
    /// L04 `0x1b092c`: a crash forced on the bike's next collision (`0x414f48` mode 1).
    pub force_crash: bool,
    /// `$gp+0x1918` (`0x1b0918`): cleared by the start.
    pub g1918: i32,
    /// `0x1b0928`: the racers' route time (the last racer's `+0x1b4`).
    pub route_time: f32,
}

/// The patched routes (module docs).
#[derive(Clone, Debug, Default)]
pub struct Paths(HashMap<i32, Vec<[f32; 4]>>);

impl Paths {
    /// The route `i` (patched: point 0's w the step) — copied from the level the first time.
    pub fn ensure(&mut self, level: &crate::level::LevelData, i: i32) -> bool {
        if i < 0 { return false; }
        if self.0.contains_key(&i) { return true; }
        let Some(p) = usize::try_from(i).ok().and_then(|k| level.paths.get(k)) else { return false };
        let mut p = p.clone();
        if 2 <= p.len() { p[0][3] = crate::hero::physics::dist(xyz(p[0]), xyz(p[1])); }
        self.0.insert(i, p);
        true
    }
    pub fn get(&self, i: i32) -> Option<&[[f32; 4]]> { self.0.get(&i).map(|v| v.as_slice()) }
    pub fn get_mut(&mut self, i: i32) -> Option<&mut Vec<[f32; 4]>> { self.0.get_mut(&i) }
    /// The step of route `i` (its point 0's w).
    pub fn step(&self, i: i32) -> f32 { self.get(i).and_then(|p| p.first()).map_or(1.0, |p| p[3]) }
    pub fn count(&self, i: i32) -> i32 { self.get(i).map_or(0, |p| p.len() as i32) }
    pub fn point(&self, i: i32, k: i32) -> V3 {
        self.get(i).and_then(|p| usize::try_from(k).ok().and_then(|k| p.get(k))).map_or([0.0; 3], |q| xyz(*q))
    }
    pub fn w(&self, i: i32, k: i32) -> f32 { self.get(i).and_then(|p| usize::try_from(k).ok().and_then(|k| p.get(k))).map_or(0.0, |q| q[3]) }
}

pub fn xyz(p: [f32; 4]) -> V3 { [p[0], p[1], p[2]] }

/// `0x283cf0`: float to int (truncated).
pub fn to_i(f: f32) -> i32 { f as i32 }

/// `0x367640(t, step, route, &index, &frac, &out)` ([`along`]): the point `t` along the route.
pub fn along(paths: &Paths, route: i32, t: f32) -> (i32, f32, V3) {
    let n = paths.count(route);
    if n == 0 { return (0, 0.0, [0.0; 3]); }
    let step = paths.step(route);
    let i = to_i(t / step);
    if n - 1 <= i { return (n - 1, 0.0, paths.point(route, n - 1)); }
    if i < 0 { return (0, 0.0, paths.point(route, 0)); }
    let frac = t - i as f32 * step;
    let (a, b) = (paths.point(route, i), paths.point(route, i + 1));
    (i, frac, add(set_len(sub(b, a), frac), a))
}

/// [`along`] into a follower's words: `+0xa8` t, `+0x114` / `+0x118` the index and fraction, `+0xf0` the point.
pub fn place_along(paths: &Paths, m: &mut Moby, route: i32, t: f32, at: usize) {
    let (i, f, p) = along(paths, route, t);
    m.set_pv_i(at + 0x114 - 0xa8, i);
    m.set_pv_f(at + 0x118 - 0xa8, f);
    m.set_pv_v(at + 0xf0 - 0xa8, p);
}

fn len_sq(v: V3) -> f32 { dot(v, v) }
pub fn xy_dist(a: V3, b: V3) -> f32 { ((a[0] - b[0]) * (a[0] - b[0]) + (a[1] - b[1]) * (a[1] - b[1])).sqrt() }

/// `0x401f70(moby, route, clamp)` (the decomp's `FindNearestRoutePointIndex`): the nearest point of the route to the
/// moby (a walk in fives, then the ten about it, wrapping); `clamp`: the points the level skips past (Barlow's
/// `routes[6]` 105..123 → 124; Joba's `routes[7]` 12..21 → 22 and 59..70 → 71, `routes[0]` 183..202 → 203 and 55..61
/// → 62, `routes[2]` 4..8 → 9). `routes`: the moby's route list (`+0x54`, the bike's `+0x6e0`).
pub fn nearest_index(paths: &Paths, level: i32, m: &Moby, route: i32, clamp: bool) -> i32 {
    let n = paths.count(route);
    let mut best = 0;
    let mut near = 99999.0;
    let mut i = 0;
    while i < n {
        let d = len_sq(sub(m.pos, paths.point(route, i)));
        if d < near { near = d; best = i; }
        i += 5;
    }
    let centre = best;
    for o in -5..5 {
        let mut k = centre + o;
        if k < 0 { k += n; } else if n <= k { k -= n; }
        let d = len_sq(sub(m.pos, paths.point(route, k)));
        if d < near { best = k; near = d; }
    }
    if clamp {
        let base = if m.o_class == BIKE { 0x6e0 } else { 0x54 };
        let r = |k: usize| m.pv_i(base + 4 * k);
        if level == 4 {
            if route == r(6) && (105..124).contains(&best) { best = 124; }
        } else if level == 0xb {
            if route == r(7) {
                if (12..22).contains(&best) { best = 22; } else if (59..71).contains(&best) { best = 71; }
            } else if route == r(0) {
                if (183..203).contains(&best) { best = 203; } else if (55..62).contains(&best) { best = 62; }
            } else if route == r(2) && (4..9).contains(&best) {
                best = 9;
            }
        }
    }
    best
}

/// `0x3665e0(a, b, p)`: `p`'s distance across from the line a–b (xy; `|p − a|` across when a = b).
pub fn lateral(a: V3, b: V3, p: V3) -> f32 {
    let span = xy_dist(a, b);
    if span == 0.0 { return xy_dist(a, p); }
    let area = (p[1] - a[1]) * (b[0] - a[0]) - (p[0] - a[0]) * (b[1] - a[1]);
    (area * area / (span * span)).sqrt()
}

/// `0x33eab0` (L01 `0x311c18`): the point of the segment a–b nearest `p` across (xy; the height along with it).
fn on_segment(a: V3, b: V3, p: V3) -> V3 {
    let e = sub(b, a);
    let o = sub(a, p);
    let q = [p[0] + e[1], p[1] - e[0], 0.0];
    let r = [p[0] - q[0], p[1] - q[1]];
    let det = e[1] * r[0] - e[0] * r[1];
    if det.abs() < 0.0001 { return a; }
    let t = (r[1] * o[0] - r[0] * o[1]) / det;
    add(scale(e, t.clamp(0.0, 1.0)), a)
}

/// The bike's route words (the race state every progress reads: the bike's pvars).
pub mod route {
    /// `+0x1a8`: the route; `+0x1ac[4]` its branches; `+0x1bc[4]` the branch each branch rejoins (−1 the route);
    /// `+0x1cc[4]` the branch each leaves (−1 the route).
    pub const MAIN: usize = 0x1a8;
    pub const LINKS: usize = 0x1ac;
    pub const EXITS: usize = 0x1bc;
    pub const ENTRIES: usize = 0x1cc;
}

/// The bike's route set (its pvars, [`route`]).
#[derive(Clone, Copy, Debug)]
pub struct RouteSet {
    pub main: i32,
    pub links: [i32; 4],
    pub exits: [i32; 4],
    pub entries: [i32; 4],
}

impl RouteSet {
    pub fn of(bike: &Moby) -> RouteSet {
        let a = |o: usize| [bike.pv_i(o), bike.pv_i(o + 4), bike.pv_i(o + 8), bike.pv_i(o + 12)];
        RouteSet { main: bike.pv_i(route::MAIN), links: a(route::LINKS), exits: a(route::EXITS), entries: a(route::ENTRIES) }
    }
    fn link(&self, k: i32) -> i32 { usize::try_from(k).ok().and_then(|k| self.links.get(k)).copied().unwrap_or(-1) }
}

/// `0x410ae0`: the route's step and its branches' joins (module docs); a route under 3 points is none (−1), a branch
/// under 4 points is dropped.
pub fn patch_routes(paths: &mut Paths, level: &crate::level::LevelData, bike: &mut Moby) {
    let set = RouteSet::of(bike);
    if set.main == -1 { return; }
    if !paths.ensure(level, set.main) || paths.count(set.main) < 3 {
        bike.set_pv_i(route::MAIN, -1);
        return;
    }
    for i in 0..4 {
        let b = set.links[i];
        if b == -1 { continue; }
        if !paths.ensure(level, b) || paths.count(b) < 4 {
            bike.set_pv_i(route::LINKS + 4 * i, -1);
            continue;
        }
        let nb = paths.count(b);
        let first = paths.point(b, 0);
        let last = paths.point(b, nb - 1);
        let from = if set.entries[i] != -1 { set.link(set.entries[i]) } else { set.main };
        let to = if set.exits[i] != -1 { set.link(set.exits[i]) } else { set.main };
        paths.ensure(level, from);
        paths.ensure(level, to);
        let near = |r: i32, p: V3| -> f32 {
            let (mut index, mut best) = (0.0, 999_999.0);
            for j in 0..paths.count(r) - 1 {
                let d = len_sq(sub(paths.point(r, j), p));
                if d < best { index = j as f32; best = d; }
            }
            index
        };
        let i_from = near(from, first);
        let d_from = crate::hero::physics::dist(paths.point(from, to_i(i_from)), first) + i_from * paths.step(from);
        let i_to = near(to, last);
        let d_to = crate::hero::physics::dist(paths.point(to, to_i(i_to)), last) + i_to * paths.step(to);
        if let Some(p) = paths.get_mut(b) {
            p[1][3] = i_from;
            p[3][3] = d_from;
            p[2][3] = i_to;
            p[4][3] = d_to;
        }
    }
}

/// `0x410e00(moby, state, link, point, wrap, bike)` (the decomp's `ProjectLinkedRouteDistance`): how far along the
/// route `pos` is from its nearest point `point` of branch `link` (−1 the route): the nearer of the two segments about
/// it, the distance along that; on a branch, scaled into its stretch of the route (its joins' distances). The bike's
/// call also turns the race's frame `0x1b31a0` along the segment (not ported: nothing ported reads it).
pub fn progress(paths: &Paths, set: &RouteSet, pos: V3, link: i32, point: i32) -> f32 {
    let base = set.main;
    let nb = paths.count(base);
    if nb == 0 { return 0.0; }
    let bstep = paths.step(base);
    if link == -1 {
        let prev = (point + nb - 1) % nb;
        let next = (point + nb + 1) % nb;
        let p0 = on_segment(paths.point(base, prev), paths.point(base, point), pos);
        let p1 = on_segment(paths.point(base, point), paths.point(base, next), pos);
        return if len_sq(sub(p0, pos)) < len_sq(sub(p1, pos)) {
            prev as f32 * bstep + crate::hero::physics::dist(p0, paths.point(base, prev))
        } else {
            point as f32 * bstep + crate::hero::physics::dist(p1, paths.point(base, point))
        };
    }
    let path = set.link(link);
    let n = paths.count(path);
    let mut next = point + 1;
    let mut prev = point - 1;
    let k = usize::try_from(link).unwrap_or(0).min(3);
    if prev == -1 {
        let other = if set.entries[k] != -1 { set.link(set.entries[k]) } else { base };
        let j = paths.point(other, to_i(paths.w(path, 1)));
        let p0 = on_segment(j, paths.point(path, point), pos);
        let p1 = on_segment(paths.point(path, point), paths.point(path, next), pos);
        if len_sq(sub(p0, pos)) < len_sq(sub(p1, pos)) {
            return paths.w(path, 3) - crate::hero::physics::dist(p0, paths.point(path, point));
        }
        prev = 0;
    } else if n - 1 < next {
        let other = if set.exits[k] != -1 { set.link(set.exits[k]) } else { base };
        let j = paths.point(other, to_i(paths.w(path, 2)));
        let p0 = on_segment(paths.point(path, point), paths.point(path, prev), pos);
        let p1 = on_segment(j, paths.point(path, point), pos);
        if len_sq(sub(p1, pos)) < len_sq(sub(p0, pos)) {
            return paths.w(path, 4) + crate::hero::physics::dist(p1, paths.point(path, point));
        }
        next = point;
    }
    let step = paths.step(path);
    let p0 = on_segment(paths.point(path, prev), paths.point(path, point), pos);
    let p1 = on_segment(paths.point(path, point), paths.point(path, next), pos);
    let v = if len_sq(sub(p0, pos)) < len_sq(sub(p1, pos)) {
        prev as f32 * step + crate::hero::physics::dist(p0, paths.point(path, prev))
    } else {
        point as f32 * step + crate::hero::physics::dist(p1, paths.point(path, point))
    };
    let v = v / (n as f32 * step);
    (paths.w(path, 4) - paths.w(path, 3)) * v + paths.w(path, 1) * bstep
}

/// `0x411a28(moby, state, &link, &point, &out, wrap)` (`func_L04_00410518`): the nearest point of the route and its
/// branches (all of them), then [`progress`]. A moby that was at the route's start and is now past its half is kept
/// at point 0.
pub fn find_progress(paths: &Paths, set: &RouteSet, pos: V3, link: &mut i32, point: &mut i32) -> f32 {
    let (old_point, old_link) = (*point, *link);
    *link = -1;
    *point = 0;
    let mut best = 9999.0;
    for i in 0..paths.count(set.main) - 1 {
        let d = len_sq(sub(pos, paths.point(set.main, i)));
        if d < best { *point = i; best = d; }
    }
    for k in 0..4 {
        let r = set.links[k];
        if r == -1 { continue; }
        for j in 0..paths.count(r) - 1 {
            let d = len_sq(sub(pos, paths.point(r, j)));
            if d < best { *point = j; best = d; *link = k as i32; }
        }
    }
    let half = (paths.count(set.main) as f32 * 0.5) as i32;
    if old_link == -1 && old_point == 0 && *link == old_link && half < *point { *point = 0; }
    progress(paths, set, pos, *link, *point)
}

/// `0x411c98(moby, state, &link, &point, wrap, bike)` (`func_L04_00410788`): the nearest point found about the last
/// one (ten points of the route about it and the ends of the branches that leave or join there; on a branch, its ten
/// about it and, near its ends, the route or branch it joins), then [`progress`].
pub fn track_progress(paths: &Paths, set: &RouteSet, pos: V3, link: &mut i32, point: &mut i32, wrap: bool) -> f32 {
    let base = set.main;
    let nb = paths.count(base);
    let mut best = 9999.0;
    let old_link = *link;
    let d2 = |r: i32, k: i32| len_sq(sub(pos, paths.point(r, k)));
    if old_link == -1 {
        let mut start = *point - 5;
        if start < 0 { start += nb - 1; }
        for i in 0..4 {
            let r = set.links[i];
            if r == -1 { continue; }
            let mut end = start + 10;
            if nb <= end { end -= nb; }
            let b = to_i(paths.w(r, 1));
            if (set.entries[i] == -1 && start < end && start <= b && b < end) || (end < start && (start <= b || b < end)) {
                for j in 0..5 {
                    let d = d2(r, j);
                    if d < best { best = d; *link = i as i32; *point = j; }
                }
            }
            let b = to_i(paths.w(r, 2));
            if (set.exits[i] == -1 && start < end && start <= b && b < end) || (end < start && (start <= b || b < end)) {
                for j in 0..5 {
                    let k = paths.count(r) - 1 - j;
                    let d = d2(r, k);
                    if d < best { best = d; *link = i as i32; *point = k; }
                }
            }
        }
        for n in 0..10 {
            let mut k = start + n;
            if nb <= k { k -= nb; }
            let d = d2(base, k);
            if d < best { best = d; *link = -1; *point = k; }
        }
    } else {
        let ol = old_link.clamp(0, 3) as usize;
        let path = set.links[ol];
        let mut start = *point - 5;
        if start < 0 {
            start = 0;
            let mut n0 = to_i(paths.w(path, 1)) - 5;
            if n0 < 0 { n0 = if wrap { n0 + nb - 1 } else { 0 }; }
            for i in 0..10 {
                let mut j = n0 + i;
                if nb <= j { j -= nb; }
                let d = d2(base, j);
                if d < best { best = d; *link = -1; *point = j; }
                for l in 0..4 {
                    let o = set.links[l];
                    if o == -1 || l == ol || to_i(paths.w(o, 1)) != j { continue; }
                    for k in 0..5 {
                        let d = d2(o, k);
                        if d < best { best = d; *link = l as i32; *point = k; }
                    }
                }
            }
        } else if paths.count(path) - 4 < *point {
            let mut n0 = to_i(paths.w(path, 2)) - 5;
            let next = set.exits[ol];
            let other = if next != -1 { set.link(next) } else { base };
            let no = paths.count(other);
            if n0 < 0 { n0 = if wrap { n0 + no - 1 } else { 0 }; }
            for i in 0..10 {
                let mut j = n0 + i;
                if no <= j { j -= no; }
                let d = d2(other, j);
                if d < best { best = d; *link = next; *point = j; }
            }
        }
        for n in 0..10 {
            let k = start + n;
            if nb <= k { break; }
            let d = d2(path, k);
            if d < best { best = d; *point = k; *link = old_link; }
        }
    }
    progress(paths, set, pos, *link, *point)
}

/// `0x412358(link, point, z)` (`func_L04_00410e48`): Joba's drops off the track — on the route at points 166..186
/// under 520.2, on its branch 2 at points 1..6 under 537 or 35..47 under 534.
pub fn off_track(level: i32, link: i32, point: i32, z: f32) -> bool {
    if level != 0xb { return false; }
    if link == -1 && (166..187).contains(&point) && z < 520.2 { return true; }
    if link == 2 && (((1..7).contains(&point) && z < 537.0) || ((35..48).contains(&point) && z < 534.0)) { return true; }
    false
}

/// `0x31ce20(0.5, p, 0)` with what it found: the ground's height under `p`, its surface (`0x2e23e0`, −1 none) and
/// its unit normal (the global hit's `0x1bfb00`; up when none).
pub fn ground(ctx: &Ctx, p: V3) -> (f32, i32, V3) {
    let Some(c) = ctx.coll else { return (p[2] - 1000.0, -1, [0.0, 0.0, 1.0]) };
    match crate::collision::coll_line(c, [p[0], p[1], p[2] + 0.5], [p[0], p[1], 0.01], QueryFlags(2)) {
        Some(o) => (o.point[2], o.surface_id(), set_len(o.normal, 1.0)),
        None => (0.01, -1, [0.0, 0.0, 1.0]),
    }
}

/// `0x321688(max, v)`: `v` at most `max` long.
pub fn cap(v: V3, max: f32) -> V3 { if max < len(v) { set_len(v, max) } else { v } }

/// The moby in view within `far` (`0x3446b8(m, far)` ≠ −1: its sphere against the frame's view).
pub fn in_view(ctx: &Ctx, m: &Moby, far: f32) -> bool {
    let s = m.bsphere;
    let sphere = [s[0] / 1024.0, s[1] / 1024.0, s[2] / 1024.0, s[3] / 1024.0];
    crate::hero::physics::dist(m.pos, ctx.host.camera_pos()) < far && !ctx.view.is_some_and(|v| v.culled(far, sphere))
}

/// What [`slide`] reports besides its result.
#[derive(Clone, Copy, Debug, Default)]
pub struct Slide {
    /// The first point met (`hitpos`; w −1 none).
    pub hit: Option<V3>,
    /// The velocity reflected off a wall it crashed on (`bounce`).
    pub bounce: V3,
    /// The ground's normal it slid on last (`normalOut`, left as it was when none).
    pub normal: Option<V3>,
    /// `*flag`: 1 on a slow surface (2 / 4), 2 against a racer or the bike, 3 on a mine 0x08ba / 0x0a77.
    pub flag: i32,
    /// `*wall`: a wall met head on (its normal against the way below −0.2).
    pub wall: bool,
}

/// `0x414f48(object, pos, vel, prev, …, mode, …, friction, slope, slope_limit, height, radius)`
/// (`func_L04_00413a38`, the decomp's `ResolveVehicleMovementCollisions`), the move of a racer (`mode` 0) or the bike
/// (1) along `vel` in steps of 0.6 × the radius, the sphere (radius, `height` up, flags 4) out of what it meets:
/// * a moby: the class 0x0a1f crashes it (2); a racer bumping a racer pushes both apart (`+0x178` 40, `+0x100`
///   ±0.05 across); a racer and the bike push each other (`+0x2f0` / `+0x100` ±0.04 across, the bike's rider moby
///   flinches 3 / 4 / 5 by the side, the racer's swerve `+0x1f4` ±10° for 5, the bike's `+0x4c8` ∓15°, the racer's
///   follow steering and its speeds cut on a flat hit, `+0x64c` 40 — 58 while it follows); a mine 0x08ba / 0x0a77
///   takes a hit (flags 0x30000) and, the mine 0x0a77 or past ¼ of the speed, crashes it;
/// * the world: 1; a steep head-on wall past the slope's limit, or the surfaces 1 / 3, crash it (2, the velocity
///   reflected); the surfaces 2 / 4 slow (flag 1).
///
/// On ground (its normal's z over 0.707) the velocity is put along it and held to `friction` across; else against
/// the wall (no rise), the bike's speed cut to what is left. The way is slid along each normal. After the steps it
/// lands on the ground under it (a slope under 45°: the velocity along it, no rise from a fall) — on the surfaces 1 /
/// 3 crashed —, and a line from where it was (flags 4) that meets something crashes it there. While the bike counts
/// down (its `+0xbc` 1, 3 or 4) nothing crashes.
#[allow(clippy::too_many_arguments)]
pub fn slide(w: &mut MobyWorld, ctx: &mut Ctx, id: usize, bike_mode: bool, pos: &mut V3, vel: &mut V3, prev: &mut V3, friction: f32, slope: f32, slope_limit: f32, height: f32, radius: f32) -> (i32, Slide) {
    let mut out = Slide::default();
    if bike_mode && w.race.force_crash {
        w.race.force_crash = false;
        return (2, out);
    }
    let old = *pos;
    let mut cur = *pos;
    cur[2] += height;
    let mut ground_left = true;
    let mut result = 0;
    let mut remaining = len(*vel);
    let segment = radius * 0.6;
    let mut dir = set_len(*vel, 1.0);
    let Some(coll) = ctx.coll else { return (0, out) };
    while 0.0 < remaining {
        let step = if segment < remaining { remaining -= segment; set_len(dir, segment) } else { let s = set_len(dir, remaining); remaining = 0.0; s };
        cur = add(cur, step);
        let Some(hit) = coll_sphere_m(coll, ctx.scene, cur, radius, QueryFlags(4), Some(id)) else { continue };
        let normal = set_len(hit.normal, 1.0);
        let pushed = hit.pushed_centre.unwrap_or(cur);
        let surface = hit.surface_id();
        if let Some(other) = hit.moby.filter(|&o| o < w.mobys.len()) {
            let z = cur[2];
            cur = pushed;
            cur[2] = z;
            let across = set_len([normal[0], normal[1], 0.0], 1.0);
            if out.hit.is_none() { out.hit = Some(hit.point); }
            result = 1;
            let oc = w.mobys[other].o_class;
            if oc == 0x0a1f {
                result = 2;
            } else if !bike_mode && oc == RACER {
                w.mobys[other].set_pv_i(0x178, 40);
                w.mobys[other].set_pv_v(0x100, scale(across, -0.05));
                w.mobys[id].set_pv_i(0x178, 40);
                w.mobys[id].set_pv_v(0x100, scale(across, 0.05));
            } else if (bike_mode && oc == RACER) || (!bike_mode && oc == BIKE) {
                // The racer `a` and the bike `b` (either the mover).
                let (racer, bike) = if bike_mode { (other, id) } else { (id, other) };
                let (b_sign, a_sign) = if bike_mode { (0.04, -0.04) } else { (-0.04, 0.04) };
                if !bike_mode { ctx.host.play_sound(&w.mobys[other], 2, 0); }
                w.mobys[bike].set_pv_v(0x2f0, scale(across, b_sign));
                w.mobys[racer].set_pv_v(0x100, scale(across, a_sign));
                if !bike_mode {
                    if let Some(r) = bike::pilot(w, bike) {
                        let p = w.mobys[id].pos;
                        let a = crate::hero::physics::atan2(hit.point[0] - p[0], hit.point[1] - p[1]);
                        let a = crate::hero::physics::sub_rot(w.mobys[bike].rot[2], a);
                        let anim = if 0.523_598_85 < a && a < 2.617_994 { 3 } else if a < -0.523_598_85 && -2.617_994 < a { 4 } else { 5 };
                        let b = &mut w.mobys[bike];
                        if b.pv_i(0x36c) != 0 && b.pv_s(0x34a) == anim {
                            b.set_pv_i(0x36c, 30);
                        } else {
                            b.set_pv_i(0x36c, 30);
                            b.set_pv_s(0x34a, anim);
                            ctx.host.set_anim(&w.mobys[r], anim as u8, 3, 3);
                        }
                    }
                }
                // The racer's rows either way (the decomp's `other`).
                let push = w.mobys[racer].pv_v(0x100);
                let d = dot(w.mobys[racer].rows[1], push);
                {
                    let (sa, sb) = if d < 0.0 { (0.174_532_95, -0.261_799_43) } else { (-0.174_532_95, 0.261_799_43) };
                    w.mobys[racer].set_pv_f(0x1f4, sa);
                    w.mobys[bike].set_pv_f(0x4c8, sb);
                    w.mobys[racer].set_pv_f(0x1f8, 5.0);
                }
                let (rows0, rows1) = (w.mobys[racer].rows[0], w.mobys[racer].rows[1]);
                if 0 < w.mobys[racer].pv_i(0x1c0) {
                    let n = w.mobys[racer].pv_i(0x1bc) + 1;
                    w.mobys[racer].set_pv_i(0x1bc, n);
                    let fwd = scale(rows0, if 4 <= n { 0.02 } else { 0.005 });
                    let side = scale(rows1, if d < 0.0 { -0.035 } else { 0.035 });
                    let p = scale(add(side, fwd), n as f32 * 0.05 + 1.0);
                    w.mobys[racer].set_pv_v(0x100, p);
                    let b = scale(w.mobys[bike].pv_v(0x2f0), 1.15);
                    w.mobys[bike].set_pv_v(0x2f0, b);
                } else {
                    let k = if w.mobys[bike].pv_i(0x4d8) != 0 { 1.5 } else { 1.0 };
                    w.mobys[racer].set_pv_v(0x100, scale(push, k));
                }
                let flat = 0.707 < normal[2];
                if (w.mobys[bike].pv_s(0x274) != 0 || w.mobys[racer].pv_b(0xa5) != 0) && flat {
                    let r = &mut w.mobys[racer];
                    r.set_pv_f(0x138, r.pv_f(0x138) * 0.65);
                    r.set_pv_f(0x13c, r.pv_f(0x13c) * 0.65);
                }
                w.mobys[bike].set_pv_i(0x71c, 0);
                w.mobys[bike].set_pv_i(0x64c, 40);
                if w.mobys[racer].pv_i(0x178) <= 0 { w.mobys[racer].set_pv_i(0x178, 40); }
                if 0 < w.mobys[racer].pv_i(0x1c0) && w.mobys[racer].pv_i(0x1f0) == 1 {
                    w.mobys[bike].set_pv_i(0x64c, 58);
                    w.mobys[racer].set_pv_f(0x1e8, 1.0);
                }
                out.flag = 2;
            } else if oc == 0x08ba || oc == 0x0a77 {
                out.flag = 3;
                let mut t = crate::moby::hits::HitTemplate::new(1.0, Some(id), 0x30000, [step[0], step[1], step[2], 0.0]);
                t.kind = (w.mobys[id].o_class as u16 as u32) << 16 | 0x0100;
                w.deliver_hit_any(other, &t, [0.0; 4]);
                if 0.25 < friction || oc == 0x0a77 { result = 2; }
            }
            ground_left_step(&mut out, vel, prev, &mut dir, normal, true, bike_mode, w, id, friction, &mut ground_left);
        } else {
            cur = pushed;
            result = 1;
            if out.hit.is_none() { out.hit = Some(hit.point); }
            let d = dot(dir, normal);
            out.wall = d < -0.2;
            if (slope_limit < slope && d < -0.866) || surface == 1 || surface == 3 {
                cur = pushed;
                out.bounce = reflect(*vel, normal);
                result = 2;
                ground_left = false;
                break;
            }
            if surface == 2 || surface == 4 { out.flag = 1; }
            ground_left_step(&mut out, vel, prev, &mut dir, normal, false, bike_mode, w, id, friction, &mut ground_left);
        }
    }
    *pos = cur;
    if ground_left {
        pos[2] -= height;
        if let Some(hit) = coll_sphere_m(coll, ctx.scene, *pos, radius, QueryFlags(4), Some(id)) {
            let s = hit.surface_id();
            if s == 1 || s == 3 { result = 2; }
            *pos = hit.pushed_centre.unwrap_or(*pos);
            let n = hit.normal;
            let angle = crate::hero::physics::atan2(n[2], (n[0] * n[0] + n[1] * n[1]).sqrt());
            if angle < 0.785_398_25 {
                let was = vel[2];
                let nn = set_len(n, 1.0);
                out.normal = Some(nn);
                *vel = sub(*vel, scale(nn, dot(*vel, nn)));
                let z = vel[2];
                vel[2] = 0.0;
                *vel = cap(*vel, friction);
                if hit.moby.is_none() || z < 0.0 { vel[2] = z; }
                if was < 0.0 && 0.0 < vel[2] { vel[2] = 0.0; }
                prev[2] = vel[2];
            } else if 0.0 < vel[2] {
                vel[2] = 0.0;
            }
        }
    }
    if coll_line_m(coll, ctx.scene, old, *pos, QueryFlags(4), Some(id)).is_some() {
        result = 2;
        *pos = old;
    }
    if let Some(b) = w.race.bike {
        if matches!(w.mobys.get(b).map_or(0, |m| m.timer), 1 | 3 | 4) { result = 0; }
    }
    (result, out)
}

/// The tail of a step of [`slide`]: the velocity on the ground or against the wall, the way slid along the normal.
#[allow(clippy::too_many_arguments)]
fn ground_left_step(out: &mut Slide, vel: &mut V3, prev: &mut V3, dir: &mut V3, mut normal: V3, on_moby: bool, bike_mode: bool, w: &mut MobyWorld, id: usize, friction: f32, ground_left: &mut bool) {
    if 0.707 < normal[2] && !on_moby {
        out.normal = Some(normal);
        *vel = sub(*vel, scale(normal, dot(*vel, normal)));
        let z = vel[2];
        vel[2] = 0.0;
        *vel = cap(*vel, friction);
        vel[2] = z;
        prev[2] = vel[2];
        *ground_left = false;
    } else {
        if 0.0 < normal[2] { normal[2] = 0.0; }
        if 0.0 < vel[2] {
            vel[2] = 0.0;
            dir[2] = 0.0;
        }
        normal = set_len(normal, 1.0);
        *vel = sub(*vel, scale(normal, dot(*vel, normal)));
        if bike_mode {
            let m = &mut w.mobys[id];
            let l = len(*vel);
            if l < (m.pv_f(0x30c) + m.pv_f(0x27c)) * 0.016_666_668 {
                m.set_pv_f(0x27c, 0.0);
                if l < m.pv_f(0x30c) { m.set_pv_f(0x30c, l); }
            }
        }
    }
    *dir = sub(*dir, scale(normal, dot(normal, *dir)));
}

/// `0x283028` (boot): `v` reflected off the plane of the unit normal `n`.
fn reflect(v: V3, n: V3) -> V3 { sub(v, scale(n, 2.0 * dot(v, n))) }

/// `0x348cd8(rows, &angles, &v, n, k, d, max)` (`func_L04_00347ab8`): the moby's up row turned toward `n` by an angle
/// spring (from none to the angle between them, its speed `v` kept), the angles read back from the rows with the
/// yaw kept.
pub fn tilt(rows: &[V3; 3], rot: &mut V3, v: &mut f32, n: V3, k: f32, d: f32, max: f32) {
    let up = rows[2];
    let c = dot(n, up).clamp(-1.0, 1.0);
    let base = crate::hero::physics::atan2(c, (1.0 - c * c).max(0.0).sqrt());
    let mut a = 0.0;
    crate::hero::physics::turn_spring(base, k, d, max, &mut a, v, 0);
    let axis = crate::hero::physics::cross(up, n);
    let l = len(axis);
    let yaw = rot[2];
    if 1e-6 < l {
        let u = scale(axis, 1.0 / l);
        let (s, co) = a.sin_cos();
        let turn = |r: V3| -> V3 {
            let x = crate::hero::physics::cross(u, r);
            add(add(scale(r, co), scale(x, s)), scale(u, dot(u, r) * (1.0 - co)))
        };
        let new = [turn(rows[0]), turn(rows[1]), turn(rows[2])];
        *rot = crate::moby::attach::angles_of(&new);
    }
    rot[2] = yaw;
}
