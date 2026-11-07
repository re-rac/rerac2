//! `0x31c618(up, r, ground, m, &vel, flags)`: a moby's move along its velocity against the world and the mobys (the
//! Synthenoids' drones, crate::moby::classes::synthenoid). The moby steps by `vel`; a step longer than `r` is tested
//! first by a line from its old place to `r` past the new one, both `up` high (`0x2de878`, flags `(flags & 0x10) << 1`,
//! the moby left out), stopping `r` short of what it meets; then a sphere of `r` at `up` pushes it out of what it
//! overlaps (`0x2df8d8`, those flags | 4), six times at most; then a line down from `up` above it to `ground + 0.05`
//! below (flags | 2) finds the ground: on a world face it is grounded (2; 6 on a face steeper than 0.5 rad), and when
//! it sank below that point it rises 0.3 of the way back. `vel` becomes the move made. The result: 1 when the line or
//! the sphere met something, | the ground's flags.
//!
//! `0x37edb8(record)` ([`sweep`]): a thrown moby's swept sphere (the Bouncer's bombs, the Gravity Bomb, the Mini-Turret).

use crate::collision::QueryFlags;
use crate::hero::physics::{add, atan2, dot, len, len2, scale, set_len, sub, unit, V3};
use crate::moby::{Ctx, MobyWorld};

/// `0x31c618` (module docs).
#[allow(clippy::too_many_arguments)]
pub fn slide(w: &mut MobyWorld, id: usize, ctx: &Ctx, up: f32, r: f32, ground: f32, vel: &mut V3, flags: u32) -> u32 {
    let f = (flags & 0x10) << 1;
    let old = w.mobys[id].pos;
    let mut pos = add(old, *vel);
    let mut out = 0;
    let line = |a: V3, b: V3, fl: u32| ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, a, b, QueryFlags(fl), Some(id)));
    if r < len(*vel) {
        let ahead = set_len(*vel, r);
        let a = [old[0], old[1], old[2] + up];
        let mut b = add(ahead, pos);
        b[2] += up;
        if let Some(o) = line(a, b, f) {
            pos = sub(o.point, ahead);
            pos[2] -= up;
            out = 1;
        }
    }
    for _ in 0..6 {
        let c = [pos[0], pos[1], pos[2] + up];
        let Some(o) = ctx.coll.and_then(|m| crate::collision::coll_sphere_m(m, ctx.scene, c, r, QueryFlags(f | 4), Some(id))) else { break };
        pos = o.pushed_centre.unwrap_or(c);
        pos[2] -= up;
        out = 1;
    }
    let low = pos[2] - ground;
    let mut flags_out = out;
    if let Some(o) = line([pos[0], pos[1], pos[2] + up], [pos[0], pos[1], low - 0.05], f | 2).filter(|o| 0 < o.kind) {
        flags_out |= 2;
        if 0.5 < atan2(o.normal[2], len2(o.normal)) { flags_out = out | 6; }
        if low < o.point[2] { pos[2] += (o.point[2] - low) * 0.3; }
    }
    *vel = sub(pos, old);
    w.mobys[id].pos = pos;
    flags_out
}

/// L04 `0x33cb08(m, &vel, r, flags, up, ground)` (L04 `0x33dd28` = L06 `0x35ca60` = L17 `0x31d450`; the decomp's
/// `func_L04_0033CB08`): [`slide`] with its radius in 1/1024ths.
#[allow(clippy::too_many_arguments)]
pub fn slide_r(w: &mut MobyWorld, id: usize, ctx: &Ctx, vel: &mut V3, r: i32, flags: u32, up: f32, ground: f32) -> u32 {
    slide(w, id, ctx, up, r as f32 * f32::from_bits(0x3a80_0000), ground, vel, flags)
}

/// The record `0x37edb8` takes: the move `from` → `to`, the sphere's radius (lifted `lift` along the moby's up row when
/// given), the query flags, the classes whose hit counts as none, the least up of a floor (a normal under it along the
/// up at the moby bounces), the bounce's share of the normal speed, and whether a moby of class type 5 bounces too.
pub struct SweepRec<'a> {
    pub from: V3,
    pub to: V3,
    pub radius: f32,
    pub lift: Option<f32>,
    pub flags: u32,
    pub pass: &'a [i16],
    pub floor: f32,
    pub bounce: f32,
    pub creatures: bool,
}

/// What `0x37edb8` gives back: `res` 0 none, 1 bounced (`vel` reflected), 2 stopped; the place (the sphere's pushed-out
/// centre, else the line's point; a stop: the hit's point), that centre as the query left it (`0x1bf1a0`, a stop
/// too), the unit normal, the moby met, the surface and the hit's own point.
pub struct Swept {
    pub res: i32,
    pub at: V3,
    pub centre: V3,
    pub normal: V3,
    pub moby: Option<usize>,
    pub surface: i32,
    pub point: V3,
}

/// `0x37edb8(record)`: the sphere at `to` (`0x2df8d8`, moby `id` left out), else the line `from` → `to` (`0x2de878`);
/// a hit on a passed class is none. The unit normal less than `floor` along the up at the moby (`0x37ecc0` at its
/// place), or a moby of class type 5 with `creatures`, bounces `vel` (`0x2ef2a8` reflected, then the normal part
/// `bounce`); else it stops at the hit's point.
pub fn sweep(w: &MobyWorld, ctx: &Ctx, id: usize, rec: &SweepRec, vel: &mut V3) -> Swept {
    let none = Swept { res: 0, at: rec.to, centre: rec.to, normal: [0.0; 3], moby: None, surface: -1, point: rec.to };
    let Some(c) = ctx.coll else { return none };
    let up = crate::gravity::up(ctx.hero, w.mobys[id].pos);
    let centre = rec.lift.map_or(rec.to, |l| add(rec.to, set_len(w.mobys[id].rows[2], l)));
    let hit = crate::collision::coll_sphere_m(c, ctx.scene, centre, rec.radius, QueryFlags(rec.flags), Some(id)).map(|h| (h.pushed_centre.unwrap_or(h.point), h))
        .or_else(|| w.coll_line(ctx, rec.from, rec.to, rec.flags, Some(id)).map(|h| (h.point, h)));
    let Some((at, h)) = hit else { return none };
    if h.moby.is_some_and(|t| w.mobys.get(t).is_some_and(|x| rec.pass.contains(&x.o_class))) { return none; }
    let surface = h.surface_id();
    let n = unit(h.normal);
    let creature = rec.creatures && h.moby.and_then(|t| w.mobys.get(t)).and_then(|x| w.class(x.o_class)).is_some_and(|c| c.kind == 5);
    if dot(up, n) < rec.floor || creature {
        let r = sub(*vel, scale(n, 2.0 * dot(*vel, n)));
        let along = scale(n, dot(r, n));
        *vel = add(sub(r, along), scale(along, rec.bounce));
        return Swept { res: 1, at, centre: at, normal: n, moby: h.moby, surface, point: h.point };
    }
    Swept { res: 2, at: h.point, centre: at, normal: n, moby: h.moby, surface, point: h.point }
}
