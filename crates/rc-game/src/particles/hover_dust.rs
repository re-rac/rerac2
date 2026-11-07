//! The dust the hovercraft raise under them: staged puffs (type 2, [`super::staged`]) in a ring about the ground under
//! the craft. One function in three copies, each over its update's own `$gp` table (all three tables hold the same
//! words, [`TABLE`]): the hovership 0x0eef's `0x41fd70` (L08; L19 `0x4143d8`; the decomp's `func_L08_0041E790`), the
//! museum's 0x085c's `0x3b5d10` (L21; the decomp's `func_L08_003C67B8`) and 0x0f2f's `0x40bcf0` (L21; the decomp's
//! `func_L30_0040B6B0`, which only picks its start colour by the level, [`MUSEUM_START`]).
//!
//! **A ring** ([`ring`], `(moby, at, count, speed, rise, outer, inner, size, end)`): its colours the table's, or —
//! the surface of the last collision hit (`0x2ee1d8`: the global hit `0x1bff40`, which `at` points into) 0 — its bare
//! ones. `count` puffs, each: a random way (`randf(−π, π)`) at `randf(inner, outer)` across for x, another way and
//! distance for y, z 0, through the craft's rows (`0x2faee8`); its velocity that way at `randf(r/4, r)` with `r` =
//! `speed·(outer − |offset|)/60/outer` (the nearer the centre the faster), plus the craft's velocity (`pvars+0`) with
//! its z × 0.25, plus `randf(rise/4, rise)/60` up; at `at` + the offset, 0.2 up; staged from size `size` to `end` (the
//! end still) in stages of `randf(15, 25)`, `randf(20, 40)`, `randf(30, 60)` ticks (truncated); one in two
//! (`rand() & 1`) additive (ALPHA 0x48).
//!
//! The callers' rings ([`IDLE`] always while ridden, [`VACUUM_NEAR`] / [`VACUUM`] while the vacuum runs): the
//! hovership (crate::moby::classes::level08::hovership) and the museum's two
//! (crate::moby::classes::level21::hovercraft).

use super::{staged, Particles};
use crate::hero::physics::{add, len, set_len, V3};
use crate::rng::Rng;

/// The table each copy reads (L08 `$gp+0x2a9c..` = `0x1b1a8c..`, L21 0x085c's `$gp+0xa00..`, 0x0f2f's `$gp+0x2710..`).
pub struct Table {
    /// The start and end colours, and the bare ground's (surface 0).
    pub colours: (u32, u32),
    pub bare: (u32, u32),
    /// The stages' lengths: in, ease, out (each `randf(lo, hi)` truncated).
    pub stages: [(f32, f32); 3],
    /// The divisors of the speed's and the rise's least (`randf(x / d, x)`).
    pub spread: f32,
    pub lift: f32,
    /// The share of the craft's z velocity the puffs carry, and their height over `at`.
    pub carry_z: f32,
    pub above: f32,
}

/// The three tables' words (equal on L08, L19 and L21).
pub const TABLE: Table = Table {
    colours: (0x5020_4050, 0x007f_7f7f),
    bare: (0x5070_3030, 0x007f_7f7f),
    stages: [(15.0, 25.0), (20.0, 40.0), (30.0, 60.0)],
    spread: 4.0,
    lift: 4.0,
    carry_z: 0.25,
    above: 0.2,
};

/// 0x0f2f's start colour on the levels 0x15 / 0x1e (the museum: the only level it is on).
pub const MUSEUM_START: u32 = 0x3030_1010;

/// A ring's arguments: the count, the speed and the rise (a second), the outer and inner reach, the start and end size.
#[derive(Clone, Copy)]
pub struct Ring {
    pub count: i32,
    pub speed: f32,
    pub rise: f32,
    pub outer: f32,
    pub inner: f32,
    pub size: f32,
    pub end: f32,
}

/// Every ridden tick (L08 `$gp+0x2adc..`: 3, 8, 1.5, 1 / 0.2, 0.6 → 1).
pub const IDLE: Ring = Ring { count: 3, speed: 8.0, rise: 1.5, outer: 1.0, inner: 0.2, size: 0.6, end: 1.0 };
/// The vacuum on with a raritanium within 10 (`$gp+0x2af4..`: 3, 8, 3.2, 2 / 2, 0.5 → 1).
pub const VACUUM_NEAR: Ring = Ring { count: 3, speed: 8.0, rise: 3.2, outer: 2.0, inner: 2.0, size: 0.5, end: 1.0 };
/// The vacuum on otherwise (`$gp+0x2b0c..`: 1, 13, 2, 1.8 / 1.8, 0.3 → 0.8); 0x0f2f's only vacuum ring.
pub const VACUUM: Ring = Ring { count: 1, speed: 13.0, rise: 2.0, outer: 1.8, inner: 1.8, size: 0.3, end: 0.8 };

/// The colours for the last hit's `surface` from the start colour `start` (the table's, or [`MUSEUM_START`]).
pub fn colours(t: &Table, surface: i32, start: u32) -> (u32, u32) { if surface == 0 { t.bare } else { (start, t.colours.1) } }

/// A ring of dust (module docs): `rows` the craft's, `carry` its velocity (`pvars+0`), `at` the last hit's point.
#[allow(clippy::too_many_arguments)]
pub fn ring(sys: &mut Particles, rng: &mut Rng, t: &Table, colours: (u32, u32), rows: &[V3; 3], carry: V3, at: V3, g: &Ring) {
    const PI: f32 = std::f32::consts::PI;
    let through = |v: V3| -> V3 { std::array::from_fn(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]) };
    for _ in 0..g.count.max(0) {
        let x = rng.randf(-PI, PI).cos() * rng.randf(g.inner, g.outer);
        let y = rng.randf(-PI, PI).sin() * rng.randf(g.inner, g.outer);
        let offset = through([x, y, 0.0]);
        let r = g.speed * ((g.outer - len(offset)) * 0.016_666_668) / g.outer;
        let mut vel = set_len(offset, rng.randf(r / t.spread, r));
        vel = add(vel, [carry[0], carry[1], carry[2] * t.carry_z]);
        vel[2] += rng.randf(g.rise / t.lift, g.rise) * 0.016_666_668;
        let mut p = add(offset, at);
        p[2] += t.above;
        let a = rng.randf(t.stages[0].0, t.stages[0].1) as i32;
        let b = rng.randf(t.stages[1].0, t.stages[1].1) as i32;
        let c = rng.randf(t.stages[2].0, t.stages[2].1) as i32;
        let made = staged(sys, rng, p, [vel[0], vel[1], vel[2], g.size], [0.0, 0.0, 0.0, g.end], colours.0, colours.1, a as i16, b as i16, c as i16, -1);
        if let Some(i) = made {
            if rng.rand() & 1 != 0 { sys.pool.recs[i][3] = 0x48; }
        }
    }
}
