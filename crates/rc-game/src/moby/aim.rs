//! The weapons' aim search `0x3257a0` (through `0x325f90`): the moby of the target list (`0x1fac60`) a shot from
//! `from` along `dir` would pick. A candidate is drawn, run by the enemy framework (mode 0x20) with health left and of
//! class type 5 (or a crate, which the searches here allow), not of the excluded classes (`0x1aa300`: 0xc87, 0xecc).
//! Its aim point (the position raised by its health block's `+0x10`) is within `range` (and 0.1 away); its yaw and
//! pitch off `dir` are within limits that narrow with the distance (from `near_yaw` / `near_pitch` at Ratchet to
//! `yaw` / `pitch` at the range, `0x31bc68`'s ease 1 − (1 − t)³); the score `3·yaw off + 0.2·pitch off·distance +
//! 0.1·distance` (a crate's × 0.8·range) is the lowest, and the line to it (flags 0x12) is clear. Through the wrapper
//! `0x325f90` ([`search_lock`]) the Lock On mod's lock (`0x18b270`, crate::moby::lock_on) wins outright.
//!
//! The hero's target search `0x2ae900` with class type 5 ([`hero_search`]: the Lock On mod's, the Meteor Pad's) is here
//! too.
//!
//! Not ported: the walls-and-ceilings branch (`$gp − 0x6000`, the gravity boots' planes) and the see-through classes a
//! blocked line may still pass (`0x381350`, the list at `0x2318d0`).

use super::{ai, mode, Ctx, MobyWorld};
use crate::hero::physics::{atan2, len, len2, rot_dist, sub, V3};

/// `0x1aa300`: the classes the search skips.
const EXCLUDED: [i16; 2] = [0xc87, 0xecc];

/// The limits of a search (`0x3257a0`'s floats 1..5).
#[derive(Clone, Copy, Debug)]
pub struct Cone {
    pub yaw: f32,
    pub pitch: f32,
    pub range: f32,
    pub near_pitch: f32,
    pub near_yaw: f32,
}

/// `0x31fa20(m)`: moby `m`'s health and aim height (its health block's `+0x00` / `+0x10`) when the enemy framework
/// runs it.
pub fn health(w: &MobyWorld, m: usize) -> Option<(f32, f32)> {
    let x = w.mobys.get(m).filter(|x| x.mode & ai::AI != 0)?;
    let h = ai::health(x);
    Some((x.pv_f(h), x.pv_f(h + 0x10)))
}

/// `0x3204e8`: a crate (o_class 500..540).
pub fn is_crate(w: &MobyWorld, m: usize) -> bool { w.mobys.get(m).is_some_and(|x| (500..=540).contains(&x.o_class)) }

/// The aim point of moby `m`: its position raised along its up row (`+0xe0`) by its aim height.
pub fn aim_point(w: &MobyWorld, m: usize) -> V3 {
    let x = &w.mobys[m];
    let up = health(w, m).map_or(0.0, |(_, a)| a);
    std::array::from_fn(|k| x.pos[k] + x.rows[2][k] * up)
}

/// `0x3257a0(cone, from, dir, ..)`: the moby picked, if any.
pub fn search(w: &MobyWorld, ctx: &Ctx, cone: Cone, from: V3, dir: V3, crates: bool) -> Option<usize> { scan(w, ctx, cone, from, dir, crates, &[], &EXCLUDED, None) }

/// `0x325f90(cone, from, dir, ..)`: the Lock On mod's lock while it is alive with health left
/// (crate::moby::lock_on::live), else [`search`].
pub fn search_lock(w: &MobyWorld, ctx: &Ctx, cone: Cone, from: V3, dir: V3, crates: bool) -> Option<usize> {
    super::lock_on::live(w, ctx).or_else(|| search(w, ctx, cone, from, dir, crates))
}

/// `0x2ae900(range, yaw, yaw cone, pitch cone, .., class type 5, line)` (crate::hero::Hero::target_search's score,
/// third person): of the target list, the moby of class type 5 with health left, within `range` and both cones
/// (≤ 0: none) of `yaw` from Ratchet, with the lowest `d + d·(yaw off + pitch off)` (a crate's or a type 0x16's
/// + 0.8·range); with `line`, one a line from Ratchet's centre (`0x189f00`, a unit over his feet) to its place plus
/// its up row meets the world (flags 2) is left out.
pub fn hero_search(w: &MobyWorld, ctx: &Ctx, range: f32, yaw: f32, yaw_cone: f32, pitch_cone: f32, line: bool) -> Option<usize> {
    use crate::hero::physics::{add, dist, dist2};
    let h = &*ctx.hero;
    let eye = if h.state == 0x14 { [h.pos[0], h.pos[1], h.ground_z] } else { h.pos };
    let mut best = 1e8f32;
    let mut out = None;
    for t in w.targets(ctx.host.camera_pos()) {
        if t.kind != 5 || health(w, t.id).is_none_or(|(hp, _)| hp == 0.0) { continue; }
        let d = dist(eye, t.pos);
        let a_yaw = rot_dist(atan2(t.pos[0] - eye[0], t.pos[1] - eye[1]), yaw);
        let a_pitch = rot_dist(atan2(dist2(eye, t.pos), t.pos[2] - eye[2]), 0.0);
        let skip = range < d || (0.0 < yaw_cone && yaw_cone < a_yaw) || (0.0 < pitch_cone && pitch_cone < a_pitch);
        let mut score = d + (a_yaw + a_pitch) * d;
        if t.crate_like || t.kind == 0x16 { score += range * 0.8; }
        if skip || best <= score { continue; }
        if line {
            let centre = add(h.pos, [0.0, 0.0, 1.0]);
            let blocked = ctx.coll.is_some_and(|c| crate::collision::coll_line_m(c, ctx.scene, centre, add(t.pos, t.up), crate::collision::QueryFlags(2), None).is_some());
            if blocked { continue; }
        }
        best = score;
        out = Some(t.id);
    }
    out
}

/// `0x3257a0` with a list (`&count, list, max`) and no excluded classes (its `param_13` null): every candidate that
/// beat the best so far, in the target list's order (at most `max`), the best then swapped to the front.
pub fn search_list(w: &MobyWorld, ctx: &Ctx, cone: Cone, from: V3, dir: V3, crates: bool, max: usize) -> Vec<usize> {
    search_list_except(w, ctx, cone, from, dir, crates, max, &[], &[])
}

/// [`search_list`] leaving out the mobys `mobys` (`param_11`, a list ending at 0) and the classes `classes`
/// (`param_13`, ending at −1): the Plasma Coil's ball's arcs (crate::moby::classes::plasma_coil).
#[allow(clippy::too_many_arguments)]
pub fn search_list_except(w: &MobyWorld, ctx: &Ctx, cone: Cone, from: V3, dir: V3, crates: bool, max: usize, mobys: &[usize], classes: &[i16]) -> Vec<usize> {
    let mut list = Vec::new();
    if let Some(b) = scan(w, ctx, cone, from, dir, crates, mobys, classes, Some((&mut list, max))) {
        if let Some(i) = list.iter().position(|&t| t == b) { list.swap(0, i); } else if let Some(f) = list.first_mut() { *f = b; }
    }
    list
}

#[allow(clippy::too_many_arguments)]
fn scan(w: &MobyWorld, ctx: &Ctx, cone: Cone, from: V3, dir: V3, crates: bool, except: &[usize], excluded: &[i16], mut list: Option<(&mut Vec<usize>, usize)>) -> Option<usize> {
    let mut best: Option<(f32, usize)> = None;
    let dir_yaw = atan2(dir[0], dir[1]);
    let dir_pitch = atan2(len2(dir), dir[2]);
    for t in w.targets(ctx.host.camera_pos()) {
        let id = t.id;
        let m = &w.mobys[id];
        if m.drawn == 0 || m.mode & mode::TARGET == 0 { continue; }
        let crate_ = is_crate(w, id);
        if crate_ && !crates { continue; }
        let Some((hp, _)) = health(w, id) else { continue };
        if !(0.0 < hp || crate_) { continue; }
        if !(t.kind == 5 || crate_) { continue; }
        if except.contains(&id) || excluded.contains(&m.o_class) { continue; }
        let p = aim_point(w, id);
        let d = sub(p, from);
        let dist = len(d);
        if !(dist <= cone.range && 0.1 <= dist) { continue; }
        let k = dist / cone.range;
        let ease = 1.0 - (1.0 - k) * (1.0 - k) * (1.0 - k);
        // A near limit of 0 keeps the far one all the way (the eased limit only with a near limit above 0).
        let limit = |near: f32, far: f32| if 0.0 < near { near + (far - near) * ease } else { far };
        // Limits of π both ways (`0x40490fdc`, not below the compare's π): no cone, the offs taken as 0 (the
        // Chopper's blade's search all round).
        let all_round = std::f32::consts::PI < cone.yaw && std::f32::consts::PI < cone.pitch;
        let (yaw_off, pitch_off) = if all_round {
            (0.0, 0.0)
        } else {
            let yaw_off = rot_dist(atan2(d[0], d[1]), dir_yaw);
            if limit(cone.near_yaw, cone.yaw) <= yaw_off { continue; }
            let pitch_off = rot_dist(atan2(len2(d), d[2]), dir_pitch);
            if limit(cone.near_pitch, cone.pitch) <= pitch_off { continue; }
            (yaw_off, pitch_off)
        };
        let mut score = yaw_off * 3.0 + pitch_off * dist * 0.2 + dist * 0.1;
        if crate_ { score *= 0.8 * cone.range; }
        if best.is_some_and(|(b, _)| b <= score) { continue; }
        // `0x2de878(from, point, 0x12, m)`: a wall or another moby between.
        let blocked = ctx.coll.is_some_and(|c| crate::collision::coll_line_m(c, ctx.scene, from, p, crate::collision::QueryFlags(0x12), Some(id)).is_some());
        if blocked { continue; }
        best = Some((score, id));
        if let Some((l, max)) = list.as_mut() {
            if l.len() < *max { l.push(id); }
        }
    }
    best.map(|(_, id)| id)
}
