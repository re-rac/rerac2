//! Platforms: what a moby riding another reads of its move (L00 `0x322028..0x3226d0`). A platform keeps a block in
//! its pvars (the lifts, the floaters and the parked ship's parts at `+0x20`) and, for the riders to find it, has mode
//! 0x20 with pvars word 8 pointing at it ([`block`]). Each tick it writes there what it just did ([`write`]); the
//! riders turn their places through it.
//!
//! **The block**: `+0x00` the tick's turn (Euler angles of `D = R_new·R_oldᵀ`), `+0x10` the tick's move, `+0x20` the
//! angles it turned from (flag 2 only), `+0x3c` flags: 2 the turn through the angles it turned from and its rows (the
//! exact one; without it `D` turns the offset from its place, which is exact for a turn about one axis), 4 a rider's
//! place in its frame follows its move, 0x40 its frame its angles (`+0xf0`) rather than its rows (`+0xc0`).
//!
//! Not ported: the users still waiting on it — the crates and the nanotech clusters on movers, Ratchet's own read of
//! the block (`0x2b4a08`, `0x2ac5f0`, `0x2ad000`, `0x2d44e0`; crate::moby::carry_hero stands in) and the parked
//! ship's parts' block (`0x361050`).

use super::attach::{angles_of, rows_of, turn};
use super::{Moby, MobyWorld};
use crate::hero::physics::{add, len, set_len, sub, V3};

/// The block's flags word.
const FLAGS: usize = 0x3c;

/// `0x322028(m)`: moby `m`'s platform block (its offset in the pvars), when it has one (mode 0x20).
pub fn block(m: &Moby) -> Option<usize> { (m.mode & 0x20 != 0).then(|| super::ai::block(m, 8)) }

fn t(m: &[V3; 3]) -> [V3; 3] { std::array::from_fn(|i| std::array::from_fn(|j| m[j][i])) }

/// `0x2ef318`: `v` through the rows (`v.x·r0 + v.y·r1 + v.z·r2`).
fn through(v: V3, r: &[V3; 3]) -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) }

/// `0x322668(block, move, old, new)`: the platform's tick into its block at `b`: the move, the turn from the angles
/// `old` to `new` (`0x2efa50(Rᵀ(old), R(new))` → `0x320000`) and, with flag 2, `old`.
pub fn write(m: &mut Moby, b: usize, mv: V3, old: V3, new: V3) {
    let d = turn(&rows_of(new), &t(&rows_of(old)));
    m.set_pv_v(b, angles_of(&d));
    if m.pv_i(b + FLAGS) & 2 != 0 { m.set_pv_v(b + 0x20, old); }
    m.set_pv_v(b + 0x10, mv);
}

/// The turn the riders' offsets take: `D`, or with flag 2 `R(old)ᵀ` then the platform's rows.
fn frame(p: &Moby, b: usize) -> [V3; 3] {
    if p.pv_i(b + FLAGS) & 2 == 0 { rows_of(p.pv_v(b)) } else { turn(&t(&rows_of(p.pv_v(b + 0x20))), &p.rows) }
}

/// `0x3222a8(_, p, point, out)`: how far `point` riding platform `p` moved this tick (None: `p` is no platform; the
/// game's zero vector).
pub fn carry(w: &MobyWorld, p: usize, point: V3) -> Option<V3> {
    let pm = w.mobys.get(p)?;
    let b = block(pm)?;
    let e = through(sub(add(point, pm.pv_v(b + 0x10)), pm.pos), &frame(pm, b));
    Some(sub(add(e, pm.pos), point))
}

/// `0x3221c8(_, p, v, out)`: a direction riding platform `p` turned by this tick's turn.
pub fn turn_dir(w: &MobyWorld, p: usize, v: V3) -> Option<V3> {
    let pm = w.mobys.get(p)?;
    let b = block(pm)?;
    Some(through(v, &frame(pm, b)))
}

/// `0x322058(_, p, pos, angles, &pos', &angles')`: a place and angles riding platform `p` after this tick's move and
/// turn (the angles by `D` alone).
pub fn ride(w: &MobyWorld, p: usize, pos: V3, angles: V3) -> Option<(V3, V3)> {
    let pm = w.mobys.get(p)?;
    let b = block(pm)?;
    let e = through(sub(add(pos, pm.pv_v(b + 0x10)), pm.pos), &frame(pm, b));
    Some((add(e, pm.pos), angles_of(&turn(&rows_of(angles), &rows_of(pm.pv_v(b))))))
}

/// The platform's frame: its angles' rows (flag 0x40) or its rows.
fn rows(pm: &Moby, b: usize) -> [V3; 3] { if pm.pv_i(b + FLAGS) & 0x40 != 0 { rows_of(pm.rot) } else { pm.rows } }

/// `0x322570(rider, p, pos, angles, &local, &local_angles)`: a place and angles in the world into platform `p`'s
/// frame.
pub fn to_local(w: &MobyWorld, p: usize, pos: V3, angles: V3) -> Option<(V3, V3)> {
    let pm = w.mobys.get(p)?;
    let b = block(pm)?;
    let inv = t(&rows(pm, b));
    Some((through(sub(pos, pm.pos), &inv), angles_of(&turn(&rows_of(angles), &inv))))
}

/// What [`to_world`] gives: the place and angles, and with flag 4 the rider's new place and angles in the frame.
pub struct World {
    pub pos: V3,
    pub angles: V3,
    pub local: Option<(V3, V3)>,
}

/// `0x3223e8(rider, p, local, local_angles, &pos, &angles)`: a place and angles in platform `p`'s frame into the
/// world. With flag 4 the place takes the move and the frame's place is taken again from there; else a rider other
/// than Ratchet whose class slot is below the platform's (updated before it this tick) takes the move, at most 1.
pub fn to_world(w: &MobyWorld, rider: usize, hero: Option<usize>, p: usize, local: V3, local_angles: V3) -> Option<World> {
    let pm = w.mobys.get(p)?;
    let b = block(pm)?;
    let r = rows(pm, b);
    let mut pos = add(through(local, &r), pm.pos);
    let angles = angles_of(&turn(&rows_of(local_angles), &r));
    let mv = pm.pv_v(b + 0x10);
    if pm.pv_i(b + FLAGS) & 4 != 0 {
        pos = add(pos, mv);
        return Some(World { pos, angles, local: to_local(w, p, pos, angles) });
    }
    if Some(rider) != hero && w.mobys.get(rider).is_some_and(|m| m.slot < pm.slot) {
        // `0x321688(1, v)`: at most 1 long.
        pos = add(pos, if 1.0 < len(mv) { set_len(mv, 1.0) } else { mv });
    }
    Some(World { pos, angles, local: None })
}
