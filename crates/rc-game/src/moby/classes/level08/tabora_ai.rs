//! Tabora's enemy framework pieces (L08 addresses; level-local code the three ground enemies 0x0fff, 0x09c6 and
//! 0x1002 share), on top of crate::moby::ai (the R&C2 framework: the reaction block, the damage, the flight).
//!
//! * **The volumes** `0x32fa98(p, path, cuboid, sphere, cylinder)` (= L01 `0x317808`; [`in_volumes`]): which of
//!   them holds `p` (1 the path's polygon, 2 the cuboid, 3 the sphere, 4 the cylinder, 0 none). The Oozla flyer's copy
//!   (crate::moby::classes::flyer) leaves the cylinder out; this one tests it.
//! * **Their reach** `0x3300f8(p, path, cuboid, sphere, cylinder)` ([`volume_reach`]): the farthest across from `p`
//!   of the path's points (`0x330050`), the cuboid's 27 points at −1 / 0 / 1 on each axis (`0x32fcc0`), the sphere's six
//!   poles (`0x32fdb0`) and the cylinder's twelve rim points (`0x32ff10`), each through the volume's rows plus its
//!   place (`0x32fbe0`); 0 with none. The set-ups make their update distance of it.
//! * **The target** `0x330a10(range, near, m, block, path, cuboid, sphere, cylinder)` ([`find_target`]): Ratchet
//!   unless he is in the movement group 0x18 or the state 0x6f, or outside the volumes and farther across than
//!   `near`; then the nearest decoy (0x00cb / 0x076c / 0x0eb0 in its state 3, the list `0x1b2b30`) across within
//!   `range` (shrinking to each found) that is within `near` or in the volumes. The block: `+0x00` the place, `+0x10`
//!   the angles, `+0x20` / `+0x30` the aim (Ratchet: his aim point `0x189ef0` and `0x189f00`; a decoy: 0.6 over it,
//!   twice), `+0x40` the moby, `+0x44` the result (0 Ratchet, 1 a decoy, 2 none; also returned). No target: the
//!   block cleared.
//! * **The floor** `0x423e58(p, top, bottom, lowest, cap)` ([`floor_at`]): a line down from `top` (at most `cap`,
//!   0 none) to `bottom` (flags 2, nothing left out): its height, at least `lowest`; no floor: `lowest`, or with none
//!   `p`'s height.
//!
//! Not ported: `0x189f00` (the port takes Ratchet's place 1 up, as crate::moby::classes::level04::launcher does for
//! its copy of the search, L04 `0x343c18`); the decoy list itself (every moby of those classes is looked at).

use crate::hero::physics::{add, dist2, V3};
use crate::level::LevelData;
use crate::moby::ai::set_link;
use crate::moby::{Ctx, MobyWorld};

/// The decoys the target search takes (`0x330a10`).
const DECOYS: [i16; 3] = [0x00cb, 0x076c, 0x0eb0];

/// `0x32fa98` (module docs).
pub fn in_volumes(level: &LevelData, p: V3, path: i32, cuboid: i32, sphere: i32, cylinder: i32) -> i32 {
    if 0 <= path && level.in_polygon(path, p) { return 1; }
    if level.in_cuboid(cuboid, p) { return 2; }
    if level.in_sphere(sphere, p) { return 3; }
    if level.in_cylinder(cylinder, p) { return 4; }
    0
}

/// `0x32fbe0(p, rows, place, best, a, b, c)`: the farther across of `best` and the point `(a, b, c)` through the rows.
fn reach_point(p: V3, m: &[[f32; 4]; 4], best: f32, a: f32, b: f32, c: f32) -> f32 {
    let q: V3 = std::array::from_fn(|k| m[3][k] + m[0][k] * a + m[1][k] * b + m[2][k] * c);
    best.max(dist2(p, q))
}

/// `0x3300f8` (module docs).
pub fn volume_reach(level: &LevelData, p: V3, path: i32, cuboid: i32, sphere: i32, cylinder: i32) -> f32 {
    let mut best = 0.0f32;
    if let Some(pts) = usize::try_from(path).ok().and_then(|i| level.paths.get(i)) {
        for q in pts { best = best.max(dist2([q[0], q[1], q[2]], p)); }
    }
    if let Some(v) = usize::try_from(cuboid).ok().and_then(|i| level.cuboids.get(i)) {
        for c in [-1.0, 0.0, 1.0] {
            for b in [-1.0, 0.0, 1.0] {
                for a in [-1.0, 0.0, 1.0] { best = reach_point(p, &v.matrix, best, a, b, c); }
            }
        }
    }
    if let Some(v) = usize::try_from(sphere).ok().and_then(|i| level.spheres.get(i)) {
        for x in [-1.0, 1.0] { best = reach_point(p, &v.matrix, best, 0.0, 0.0, x); }
        for x in [-1.0, 1.0] { best = reach_point(p, &v.matrix, best, 0.0, x, 0.0); }
        for x in [-1.0, 1.0] { best = reach_point(p, &v.matrix, best, x, 0.0, 0.0); }
    }
    if let Some(v) = usize::try_from(cylinder).ok().and_then(|i| level.cylinders.get(i)) {
        for c in [-1.0, 0.0, 1.0] {
            for a in [-1.0, 1.0] { best = reach_point(p, &v.matrix, best, 0.0, a, c); }
            for b in [-1.0, 1.0] { best = reach_point(p, &v.matrix, best, b, 0.0, c); }
        }
    }
    best
}

/// The volumes a search tests (`path`, `cuboid`, `sphere`, `cylinder`).
#[derive(Clone, Copy)]
pub struct Volumes(pub i32, pub i32, pub i32, pub i32);

impl Volumes {
    pub fn holds(&self, level: &LevelData, p: V3) -> bool { in_volumes(level, p, self.0, self.1, self.2, self.3) != 0 }
}

/// `0x330a10` (module docs): the target block at `b` filled; the result.
pub fn find_target(w: &mut MobyWorld, id: usize, ctx: &Ctx, b: usize, range: f32, near: f32, vols: Volumes) -> i32 {
    let here = w.mobys[id].pos;
    let mut result = 0;
    let mut target = ctx.hero_moby;
    if ctx.hero.group == 0x18 || ctx.hero.state == 0x6f || (!vols.holds(ctx.level, ctx.hero.pos) && near < dist2(here, ctx.hero.pos)) {
        target = None;
        result = 2;
    }
    let mut range = range;
    for (k, o) in w.mobys.iter().enumerate() {
        if !DECOYS.contains(&o.o_class) || o.state != 3 { continue; }
        let d = dist2(here, o.pos);
        if d < range && (d < near || vols.holds(ctx.level, o.pos)) {
            result = 1;
            range = d;
            target = Some(k);
        }
    }
    let m = &mut w.mobys[id];
    match target {
        None => {
            set_link(m, b + 0x40, None);
            for k in 0..4 { m.set_pv_v(b + 0x10 * k, [0.0; 3]); }
        }
        Some(t) if Some(t) == ctx.hero_moby => {
            set_link(m, b + 0x40, Some(t));
            m.set_pv_v(b, ctx.hero.pos);
            m.set_pv_v(b + 0x10, ctx.hero.rot);
            m.set_pv_v(b + 0x20, ctx.hero.aim_point);
            m.set_pv_v(b + 0x30, add(ctx.hero.pos, [0.0, 0.0, 1.0]));
        }
        Some(t) => {
            let (p, r) = (w.mobys[t].pos, w.mobys[t].rot);
            let m = &mut w.mobys[id];
            set_link(m, b + 0x40, Some(t));
            m.set_pv_v(b, p);
            m.set_pv_v(b + 0x10, r);
            let up = add(p, [0.0, 0.0, 0.6]);
            m.set_pv_v(b + 0x30, up);
            m.set_pv_v(b + 0x20, up);
        }
    }
    w.mobys[id].set_pv_i(b + 0x44, result);
    result
}

/// `0x423e58` (module docs).
pub fn floor_at(ctx: &Ctx, p: V3, top: f32, bottom: f32, lowest: f32, cap: f32) -> f32 {
    let top = if cap != 0.0 { top.min(cap) } else { top };
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, [p[0], p[1], top], [p[0], p[1], bottom], crate::collision::QueryFlags(2), None));
    match hit {
        Some(o) => o.point[2].max(lowest),
        None if lowest != 0.0 => lowest,
        None => p[2],
    }
}

/// `0x329170(a, t, k, d, most, &v)` (= L01 `0x311de0`): `a` sprung toward `t` (the push the difference over 9°, at
/// most 1; the speed at most `most` and the difference); the new angle and speed.
pub fn spring_angle(a: f32, t: f32, k: f32, d: f32, most: f32, v: f32) -> (f32, f32) {
    use crate::hero::physics::{add_rot, sub_rot};
    let e = sub_rot(t, a);
    let mut v = v + (k * (e * 6.366_197).clamp(-1.0, 1.0) - d * v);
    if most != 0.0 { v = v.clamp(-most, most); }
    v = v.clamp(-e.abs(), e.abs());
    (add_rot(a, v), v)
}
