//! The enemy framework's movement in the gravity frames (crate::gravity): L02 `0x3347e8` → `0x334830` (the decomp's
//! `func_L02_00333438` / `func_L02_00333480`; the same code on levels 2, 11, 12, 13, 14, 15, 22, 23 and 26, L26
//! `0x32d8d0` / `0x32d918`). It stands in for crate::moby::ai's movement ([`ai::move_step`]) while the sphere is on
//! (`0x1a8ff0`): the same movement block (the pointer word `+0x18`), the same flags, worked in the moby's own frame (its
//! rows `+0xc0`, kept by mode 0x100) instead of along world z, heights measured from the sphere's centre
//! ([`crate::gravity::Gravity::height`]) and up away from it ([`crate::gravity::Gravity::up_at`]).
//!
//! **The move** ([`move_to`], `0x334830(m, goal, goal − place)`): none without the block (8); a second call in a frame
//! returns its flags. The frame's transform is the rows' transpose as they come in (`0x2efc00`); then:
//! * **the upright** (`0x32ade0` → `0x32ae08`): the rows turned so row 2 heads for the up at its place — the quaternion
//!   ½·(up × row 2), w −√(1 − its length²) (the game's arithmetic, not a true half angle) — and the velocity `+0xa0`
//!   turned with them;
//! * the flags moved to `+0x98`; after a frame or more without a move, the floor (`0x3346b0`) and the turn, the stuck
//!   ticks and the velocity cleared (as [`ai::move_step`]);
//! * **the walk** (`0x333e30`): unless option 0x10, the turn (`0x334500`, [`face_point`]'s) in its own frame: the goal's
//!   yaw there (`0x32af40`) eased by the turn speed `+0xb4` (`+0x14` accel, `+0x18` brake, most π) and the rows
//!   turned by it about their z; with the yaw left within `+0x30`, the walk along row 0 at
//!   `min(√(2·accel·(d − stop)), most)` (`0x32a7d0`; d the goal's distance, `+0xcc`); the velocity's change has its
//!   local z dropped and is held to `+0x20` speeding up (else `+0x24`); `+0xb0` its speed across in the frame. Then
//!   (unless option 4) the vertical: above the floor's height, the fall toward the centre (`+0x2c`, `0x32ab80`; with
//!   no ground ticks flag 0x20); on it, the local z eased up to the floor (1/90, 1/90, most 1);
//! * **the body** (`0x3340d8`): the move by the velocity and up to eight push-outs of the sphere `+0x00` at `+0x04 +
//!   +0x00` along row 2 (`0x3345c8`: flags of option 2; the pushing moby `+0x84`, the touch `+0x60`); pushed, the
//!   velocity is the move made, held across to the speed it had, its local z dropped when it is on or under the
//!   floor (lifted to it along row 2); the stuck ticks `+0x80` count while it makes under half its top speed;
//!   `+0x70` the push-out;
//! * **the ground** (`0x3342a0`): the floor again; sunk more than the step up under it (flag 2), a drop ahead (unless
//!   option 1, in the first 5 air ticks: more than the step down above it; flag 1), too steep a floor (the slope
//!   `0x32a708` against the up over `+0x10`; flag 4): back where it was. Within 0.005 of the floor it is lifted onto it
//!   and the ground ticks `+0xc0` count; more than the step down above it the air ticks `+0xc2`. At the goal (`+0x34`)
//!   flag 0x80; option 0x200 the fall kill (`0x32db90`: a 255 hit when its height is half a unit under the reaction
//!   block's `+0x40`, while its z is over the floor's height).
//!
//! The local frame of the walk's and the body's conversions is the rows as they came in; the way back to the world
//! is through the rows as turned (the game's mix, kept).
//!
//! **The face** ([`face_point`], `0x3339e0` → `0x333a18` with the frames on): the turn toward a point alone; whether the
//! yaw left is within `+0x30`.
//!
//! The branch points (`0x1a8ff0` set): crate::moby::classes::critter's move (`0x3cfe08`, L26 `0x3bdf40`) and face
//! (`0x3339e0`), crate::moby::classes::keeper's face; L26's 0x0c8e calls the move outright
//! (crate::moby::classes::level26::spinner).
//!
//! Not ported: the profile counters (`0x333cb0`), the floor moby's carry (`0x3335b0`, flag 0x40; as in
//! [`ai::move_step`]), the cylinder mode (`0x1a8ff4`).

use super::{ai, Ctx, Moby, MobyWorld};
use crate::gravity::{to_local, Gravity};
use crate::hero::physics::*;
use crate::moby::attach::{rows_of, turn};
use crate::quat::{rotate, turn_rows, Q};

/// `0x2b04e0(m, v, t)` (L26 `0x3261a0`): the yaw of `v` in the frame whose transpose is `t` (`t` the rows, here).
fn yaw_in(v: V3, rows: &[V3; 3]) -> f32 {
    let l = to_local(v, rows);
    atan2(l[0], l[1])
}

/// `v` into the frame `inv` and back through `rows` with its local z set to `z` (`0x2afeb8`, L26 `0x325a70`).
fn with_z(v: V3, inv: &[V3; 3], rows: &[V3; 3], z: f32) -> V3 {
    let mut l = to_local(v, inv);
    l[2] = z;
    crate::gravity::to_world(l, rows)
}

/// `0x32ae08(rows, up, out)`: the quaternion that turns row 2 toward `up` (module docs).
fn upright(rows: &[V3; 3], up: V3) -> Q {
    let a = scale(cross(set_len(up, 1.0), rows[2]), 0.5);
    let w = -(1.0 - dot(a, a)).sqrt();
    [a[0], a[1], a[2], w]
}

/// `0x334500(m, block, v, t)` (L00 `0x3c13b0`, L26 `0x32d5e8`): the turn toward the local yaw of `v` (in the frame
/// `inv`): the turn speed `+0xb4` eased toward it (`0x31f230`, most π) and the rows turned by it about their z.
/// Returns |yaw| − |speed|.
fn turn_by(m: &mut Moby, o: usize, v: V3, inv: &[V3; 3]) -> f32 {
    let f = yaw_in(v, inv);
    let (mut a, mut s) = (0.0, m.pv_f(o + 0xb4));
    turn_ease(f, m.pv_f(o + 0x14), m.pv_f(o + 0x18), std::f32::consts::PI, &mut a, &mut s, 0);
    m.set_pv_f(o + 0xb4, s);
    m.rows = turn(&rows_of([0.0, 0.0, s]), &m.rows);
    f.abs() - s.abs()
}

/// `0x333a18(m, v)` with the frames on (`0x3339e0(m, p)`: `v` = `p` − its place): [`turn_by`] in its own frame;
/// whether the yaw left is within `+0x30` (none without the block).
pub fn face_point(w: &mut MobyWorld, id: usize, p: V3) -> bool {
    let m = &mut w.mobys[id];
    if m.pv_i(0x18) == 0 { return false; }
    let o = ai::mv(m);
    let rows = m.rows;
    let h = turn_by(m, o, sub(p, m.pos), &rows);
    let within = h <= m.pv_f(o + 0x30);
    w.changed = true;
    within
}

/// `0x3346b0(m, block)`: the floor — a line along the up from the step up above the feet to the step down below:
/// its height `+0x90` (from the sphere's centre), its moby `+0x88` and kind `+0x8c`, its normal `+0x50`; none: all
/// cleared. (The surface `+0xd8` is left.)
fn floor(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let g = &ctx.hero.gravity;
    let m = &w.mobys[id];
    let o = ai::mv(m);
    let up = g.up_at(m.pos);
    let a = add(set_len(up, m.pv_f(o + 8) + 0.1), m.pos);
    let b = add(set_len(up, -m.pv_f(o + 0xc) - 0.1), m.pos);
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, a, b, crate::collision::QueryFlags(0), Some(id)));
    let m = &mut w.mobys[id];
    match hit {
        None => {
            m.set_pv_f(o + 0x90, 0.0);
            m.set_pv_i(o + 0x88, 0);
            m.set_pv_i(o + 0x8c, 0);
            m.set_pv_v(o + 0x50, [0.0; 3]);
        }
        Some(h) => {
            m.set_pv_f(o + 0x90, g.height(h.point));
            ai::set_link(m, o + 0x88, h.moby);
            m.set_pv_i(o + 0x8c, h.kind);
            m.set_pv_v(o + 0x50, set_len(h.normal, 1.0));
        }
    }
}

/// `0x32a708(m, n, 0)` with the frames on: the slope of the normal `n` against the up at its place,
/// `atan2(√(1 − n·up), n·up)` (the game's: not √(1 − (n·up)²)).
fn slope(g: &Gravity, p: V3, n: V3) -> f32 {
    let d = dot(set_len(n, 1.0), g.up_at(p));
    atan2(d, (1.0 - d).sqrt())
}

/// `0x333e30` (module docs): the turn, the walk, the vertical.
fn walk(w: &mut MobyWorld, id: usize, ctx: &Ctx, goal: V3, v: V3, inv: &[V3; 3]) {
    let g = &ctx.hero.gravity;
    let m = &mut w.mobys[id];
    let o = ai::mv(m);
    let opts = m.pv_i(o + 0x38) as u32;
    if opts & 0x10 == 0 { turn_by(m, o, v, inv); }
    let heading = if opts & 0x10 == 0 { yaw_in(v, inv).abs() } else { 0.0 };
    let mut want = [0.0f32; 3];
    if heading <= m.pv_f(o + 0x30) {
        let d = dist(m.pos, goal);
        m.set_pv_f(o + 0xcc, d);
        let stop = m.pv_f(o + 0x34);
        if stop < d {
            let s = ((d - stop) * 2.0 * m.pv_f(o + 0x24)).sqrt().min(m.pv_f(o + 0x28));
            want = scale(m.rows[0], s);
        }
    }
    let vel = m.pv_v(o + 0xa0);
    let mut dv = with_z(sub(want, vel), inv, &m.rows, 0.0);
    let lim = if 0.0 < dot(want, vel) { m.pv_f(o + 0x20) } else { m.pv_f(o + 0x24) };
    if lim < len(dv) { dv = set_len(dv, lim); }
    let mut vel = add(vel, dv);
    m.set_pv_f(o + 0xb0, len2(to_local(vel, inv)));
    if opts & 4 == 0 {
        let h = g.height(m.pos);
        let fl = m.pv_f(o + 0x90);
        if fl < h {
            vel = add(vel, set_len(g.down_at(m.pos), m.pv_f(o + 0x2c)));
            if m.pv_s(o + 0xc0) == 0 { let f = m.pv_i(o + 0x94); m.set_pv_i(o + 0x94, f | 0x20); }
        } else {
            let mut vz = to_local(vel, inv)[2];
            if fl - h < vz { vz = fl - h } else if vz < 0.0 { vz = 0.0 }
            ease_speed(fl - h, 1.0 / 90.0, 1.0 / 90.0, 1.0, &mut vz);
            vel = with_z(vel, inv, &m.rows, vz);
        }
    }
    m.set_pv_v(o + 0xa0, vel);
}

/// `0x3345c8(m, block)`: the sphere `+0x00` at `+0x04 + +0x00` up its row 2 pushed out of the world and the mobys
/// (flags: option 2); the pushing moby `+0x84`, the touch `+0x60`.
fn push_out(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    let o = ai::mv(m);
    let r = m.pv_f(o);
    let up = scale(m.rows[2], m.pv_f(o + 4) + r);
    let c = add(m.pos, up);
    let flags = m.pv_i(o + 0x38) as u32 & 2;
    let live = |k: usize| w.mobys.get(k).is_some_and(|m| m.has_collision && m.state < 0xfd);
    let Some(hit) = ctx.coll.and_then(|cl| crate::collision::coll_sphere_live(cl, ctx.scene, c, r, crate::collision::QueryFlags(flags), Some(id), &live)) else { return false };
    let m = &mut w.mobys[id];
    ai::set_link(m, o + 0x84, hit.moby);
    m.set_pv_v(o + 0x60, hit.point);
    m.pos = sub(hit.pushed_centre.unwrap_or(c), up);
    true
}

/// `0x3340d8` (module docs): the move and its push-outs.
fn body(w: &mut MobyWorld, id: usize, ctx: &Ctx, from: V3, inv: &[V3; 3]) {
    let m = &mut w.mobys[id];
    let o = ai::mv(m);
    m.pos = add(m.pos, m.pv_v(o + 0xa0));
    let start = m.pos;
    ai::set_link(m, o + 0x84, None);
    m.set_pv_v(o + 0x60, [0.0; 3]);
    let mut any = false;
    for _ in 0..8 {
        if !push_out(w, id, ctx) { break; }
        any = true;
    }
    let g = &ctx.hero.gravity;
    let m = &mut w.mobys[id];
    let mut stuck = m.pv_i(o + 0x80);
    if any {
        let mut v = sub(m.pos, from);
        let s = len2(to_local(v, inv));
        if m.pv_f(o + 0xb0) < s {
            let mut l = to_local(v, inv);
            l = set_len2(l, m.pv_f(o + 0xb0));
            v = crate::gravity::to_world(l, &m.rows);
        } else {
            m.set_pv_f(o + 0xb0, s);
        }
        let h = g.height(m.pos);
        let fl = m.pv_f(o + 0x90);
        if h <= fl {
            m.pos = add(m.pos, scale(m.rows[2], fl - h));
            v = with_z(v, inv, &m.rows, 0.0);
        }
        m.set_pv_v(o + 0xa0, v);
        if m.pv_f(o + 0xb0) < m.pv_f(o + 0x28) * 0.5 { stuck += 1 } else { crate::moby::countdown(&mut stuck); }
    } else {
        crate::moby::countdown(&mut stuck);
    }
    m.set_pv_i(o + 0x80, stuck);
    m.set_pv_v(o + 0x70, sub(m.pos, start));
}

/// `0x3342a0` (module docs): the ground and the flags.
fn ground(w: &mut MobyWorld, id: usize, ctx: &Ctx, goal: V3, from: V3) {
    floor(w, id, ctx);
    let g = &ctx.hero.gravity;
    let m = &mut w.mobys[id];
    let o = ai::mv(m);
    let opts = m.pv_i(o + 0x38) as u32;
    let mut h = g.height(m.pos);
    let fl = m.pv_f(o + 0x90);
    let mut flags = m.pv_i(o + 0x94);
    let mut back = false;
    if h < fl - m.pv_f(o + 8) {
        back = true;
        flags |= 2;
    }
    if opts & 1 == 0 && m.pv_s(o + 0xc2) < 5 && fl + m.pv_f(o + 0xc) < h {
        back = true;
        flags |= 1;
    }
    if m.pv_s(o + 0xc0) != 0 && m.pv_f(o + 0x10) < slope(g, m.pos, m.pv_v(o + 0x50)) {
        back = true;
        flags |= 4;
    }
    m.set_pv_i(o + 0x94, flags);
    if back {
        m.pos = from;
        floor(w, id, ctx);
        h = g.height(w.mobys[id].pos);
    }
    let m = &mut w.mobys[id];
    let fl = m.pv_f(o + 0x90);
    if h <= fl + 0.005 {
        m.pos = add(m.pos, scale(m.rows[2], fl - h));
        m.set_pv_s(o + 0xc2, 0);
        m.set_pv_s(o + 0xc0, m.pv_s(o + 0xc0).wrapping_add(1));
    } else if fl + m.pv_f(o + 0xc) <= h {
        m.set_pv_s(o + 0xc0, 0);
        m.set_pv_s(o + 0xc2, m.pv_s(o + 0xc2).wrapping_add(1));
    }
    let d = dist(m.pos, goal);
    m.set_pv_f(o + 0xcc, d);
    if d <= m.pv_f(o + 0x34) { let f = m.pv_i(o + 0x94); m.set_pv_i(o + 0x94, f | 0x80); }
    if opts & 0x200 != 0 && m.pv_f(o + 0x90) < m.pos[2] {
        if let Some(r) = ai::react_of(m) {
            if g.height(m.pos) < m.pv_f(r + 0x40) - 0.5 {
                let p = m.pos;
                let t = super::hits::HitTemplate::new(255.0, None, 0x10000, [0.0; 4]);
                w.deliver_hit_any(id, &t, [p[0], p[1], p[2], 1.0]);
            }
        }
    }
}

/// `0x3347e8(m, goal)` → `0x334830(m, goal, goal − place)`: the frame's move in the gravity frames (module docs);
/// returns the flags (`+0x94`; 8 without the block).
pub fn move_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3) -> i32 {
    let frame = ctx.game.frame;
    let m = &mut w.mobys[id];
    if m.pv_i(0x18) == 0 { return 8; }
    let o = ai::mv(m);
    if frame == m.pv_i(o + 0xbc) { return m.pv_i(o + 0x94); }
    let v = sub(goal, m.pos);
    let inv = m.rows;
    let q = upright(&m.rows, ctx.hero.gravity.up_at(m.pos));
    m.rows = turn_rows(&m.rows, q);
    let vel = rotate(q, m.pv_v(o + 0xa0));
    m.set_pv_v(o + 0xa0, vel);
    m.set_pv_i(o + 0x98, m.pv_i(o + 0x94));
    m.set_pv_i(o + 0x94, 0);
    let from = m.pos;
    let last = m.pv_i(o + 0xbc);
    if 1 < frame - last || last == 0 {
        floor(w, id, ctx);
        let m = &mut w.mobys[id];
        m.set_pv_i(o + 0x80, 0);
        m.set_pv_b(o + 0xea, 0xff);
        m.set_pv_i(o + 0x9c, m.pv_i(o + 0x9c) & !2);
        m.set_pv_f(o + 0xb4, 0.0);
        m.set_pv_v(o + 0xa0, [0.0; 3]);
    }
    w.mobys[id].set_pv_i(o + 0xbc, frame);
    walk(w, id, ctx, goal, v, &inv);
    body(w, id, ctx, from, &inv);
    ground(w, id, ctx, goal, from);
    w.changed = true;
    w.mobys[id].pv_i(o + 0x94)
}
