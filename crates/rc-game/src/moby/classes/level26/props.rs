//! The Jamming Array's props (L26 addresses).
//!
//! * **0x0af4** ([`update_0af4`], `0x40de48`, 10 placed; the decomp's `func_L26_0040C8A8`): a floating platform
//!   (crate::moby::floater, its block at `+0x60`). Its set-up (`0x32bd98`) puts the pivot `+0x120` × 1.7 along its y.
//!   With a pivot it has no y or z response to Ratchet and no y wobble (`+0xd0`, `+0xd4`, `+0x100` = 0). Then each
//!   tick is a step (`0x32c0b0`).
//! * **0x0c00** ([`update_0c00`], `0x3d5aa0`, 7 placed; the decomp's `func_L26_003D4590`): a mote that circles the
//!   level's gravity sphere, spinning.
//!   * It is shown only while the sphere is on (`0x1a8ff0`, crate::gravity). Off, after its set-up, it is hidden and
//!     its mode gets bit 1.
//!   * **0** (set-up):
//!     * Update and draw distances 0xff / 0x200, mode | 0x110.
//!     * Its circle: `+0x50` the way from the heights' centre (`0x1b1810`, crate::gravity::Gravity::height_centre) to
//!       it, of length r. `+0x70` is `+0x50` × (row 0 at length r) and `+0x60` is `+0x70` × `+0x50`, each at length r.
//!     * Its rates: the spin `+0x188` 60..180 (`0x31bc50`, crate::rng randf_signed) and the circling `+0x180`
//!       10..30 (`0x31bbf0`), each × π/10800; its angle `+0x184` random (`0x31bcb8`). Then 1.
//!   * **1**:
//!     * The angle moves on by the circling rate (`0x2f0140`, wrapped). Its place is the centre plus (cos, sin, 0)
//!       of it through the circle's rows.
//!     * Its rows are turned so row 2 lies along the circle's tangent (`0x326010`), then spun by the spin rate
//!       about z. On each 256th frame (`0x1b16c8`) they are made orthonormal again (`0x3260e0`).
//!     * Its timer `+0x18c` counting down (`0x2eed80`) flips `+0xbc` and with it its glow (0x80202020 / 0x80c0c0c0)
//!       and the next wait (45 / 15).
//!
//! Not ported for 0x0c00:
//! * its trail: its history of places and angles, and the draw registered once a frame for its group (`0x3d5e40`,
//!   the draw `0x3d5ff8`);
//! * the six-point table `0x2967a0` its first set-up rotates, which only that draw reads.
//!
//! The level settings that turn the sphere on at the load (`0x3014a8`) are not ported (crate::gravity), so the mote
//! stays hidden.

use crate::hero::physics::{add, cross, scale, set_len, sub, unit, V3};
use crate::moby::attach::{rows_of, turn};
use crate::moby::{countdown, floater, Ctx, MobyWorld};
use std::f32::consts::{FRAC_PI_2, PI};

pub const FLOATER: i16 = 0x0af4;
pub const MOTE: i16 = 0x0c00;
const BLOCK: usize = 0x60;

pub fn update_0af4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            floater::init(w, id, ctx, BLOCK, &floater::Params::default());
            let m = &mut w.mobys[id];
            let n = m.pv_i(0x120);
            if n != 0 {
                m.set_pv_v(BLOCK + 0x10, [0.0, n as f32 * 1.7, 0.0]);
                m.set_pv_f(0xd0, 0.0);
                m.set_pv_f(0x100, 0.0);
                m.set_pv_f(0xd4, 0.0);
            }
            m.set_state(1, -1);
        }
        1 => floater::step(w, id, ctx, BLOCK),
        _ => {}
    }
}

/// `0x2f0140`: `a + b` wrapped into −π..π.
fn wrapped_add(a: f32, b: f32) -> f32 {
    let d = a + b;
    if PI <= d { d - PI - PI } else if d < -PI { d + PI + PI } else { d }
}

/// `v` through the rows `r` (`0x2ef5b8`).
fn through(v: V3, r: &[V3; 3]) -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) }

/// `0x326010(rows, v, 0)`: the rows turned so that row 2 lies along `v`. The quaternion's axis is
/// ½·(unit(v) × row 2), its w −√(1 − |axis|²): the game's arithmetic, not a true half angle.
fn rows_along(rows: &[V3; 3], v: V3) -> [V3; 3] {
    let a = scale(cross(unit(v), rows[2]), 0.5);
    let w = -(1.0 - crate::hero::physics::dot(a, a)).sqrt();
    crate::quat::turn_rows(rows, [a[0], a[1], a[2], w])
}

/// `0x3260e0(rows, 0)`: row 0 unit, row 1 = row 2 × row 0 unit, row 2 = row 0 × row 1.
fn orthonormal(r: &mut [V3; 3]) {
    r[0] = unit(r[0]);
    r[1] = unit(cross(r[2], r[0]));
    r[2] = cross(r[0], r[1]);
}

pub fn update_0c00(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let g = &ctx.hero.gravity;
    let m = &mut w.mobys[id];
    if !g.on && m.state != 0 {
        m.drawn = 0;
        m.mode |= 1;
        return;
    }
    m.drawn = 1;
    m.mode &= !1;
    let centre = g.height_centre;
    match m.state {
        0 => {
            m.draw_dist = 0x200;
            m.update_dist = 0xff;
            m.mode |= 0x110;
            let p50 = sub(m.pos, centre);
            let r = crate::hero::physics::len(p50);
            let p60 = set_len(m.rows[0], r);
            let p70 = set_len(cross(p50, p60), r);
            let p60 = set_len(cross(p70, p50), r);
            m.set_pv_v(0x50, p50);
            m.set_pv_v(0x60, p60);
            m.set_pv_v(0x70, p70);
            let spin = ctx.game.rng.randf_signed(60.0, 180.0) * 0.000_290_888_25;
            m.set_pv_f(0x188, spin);
            let circling = ctx.game.rng.randf(10.0, 30.0) * 0.000_290_888_25;
            m.set_pv_f(0x180, circling);
            m.set_pv_f(0x184, ctx.game.rng.rand_angle());
            m.set_state(1, -1);
        }
        1 => {
            let basis = [m.pv_v(0x50), m.pv_v(0x60), m.pv_v(0x70)];
            let a = wrapped_add(m.pv_f(0x184), m.pv_f(0x180));
            m.set_pv_f(0x184, a);
            m.pos = add(through([a.cos(), a.sin(), 0.0], &basis), centre);
            let t = wrapped_add(a, FRAC_PI_2);
            let tangent = through([t.cos(), t.sin(), 0.0], &basis);
            m.rows = rows_along(&m.rows, tangent);
            m.rows = turn(&rows_of([0.0, 0.0, m.pv_f(0x188)]), &m.rows);
            if ctx.game.frame & 0xff == 0 { orthonormal(&mut m.rows); }
            let mut left = m.pv_i(0x18c);
            let done = countdown(&mut left);
            m.set_pv_i(0x18c, left);
            if done {
                m.timer = (m.timer + 1) & 1;
                m.glow = [0x8020_2020, 0x80c0_c0c0][m.timer as usize];
                m.set_pv_i(0x18c, [45, 15][m.timer as usize]);
            }
        }
        _ => {}
    }
}
