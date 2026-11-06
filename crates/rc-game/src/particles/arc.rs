//! Type **0x45**, the arcs' sparks: the spawner L08 `0x34b3b8` (also L09 `0x339728`, L22 `0x3401c0`, L23 `0x3448d8`;
//! the decomp's `func_L08_0034A078`) and its update L08 `0x34b4e0` (= L00 `0x33cb10`, the decomp's `func_002C4500`; the
//! type table's entry 0x45, `0x340940` in L08's `0x3404a8`). The beams (crate::fx::beams) throw them.
//!
//! **Made** (`0x34b3b8(pos, vel, alpha, owner)`): at `pos` (its byte 9 0x74, byte 3 0x48: additive), size
//! `randf(8000, 50000)`, a random angle (`randi(256)`), its first frame the type's own, a life of `rand_range(10, 30)`;
//! `+0x20` the velocity (four lanes), `+0x30` one over the life, `+0x34` the alpha (s16), `+0x36` how it follows its
//! owner (0: it does not), `+0x38` the colour (0x7f7f7f), `+0x3c` the owner; its colour word `alpha << 24 | 0x7f7f7f`.
//!
//! **Each tick** (`0x34b4e0`): out of [2, 1021] on any axis (before and after the move) it is killed; it moves by its
//! velocity; its angle turns by `ftoi(randf_signed(0, 64))` (past 255: less 255); its life down (out: killed); its
//! alpha `life · (1 / life₀) · alpha`.
//!
//! Not ported: the follow modes (`+0x36` 1..4: riding the owner's pvars `+0xc0` / `+0x1e0` / `+0xd0` or 0.2 over its
//! place); only the beams' slots with their follow word 1 ask for them, and no class claims such a slot.

use super::{dec_life, rec, Particles};
use crate::rng::Rng;

/// The type.
pub const ARC: u8 = 0x45;

/// The spawner's random draws, in its order (size, angle, life), for a record made later than they are drawn
/// ([`put`]: the beams' sparks drawn during the frame's draw, crate::fx::beams).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rolls {
    pub size: f32,
    pub rot: u8,
    pub life: i16,
}

/// The spawner's draws.
pub fn rolls(rng: &mut Rng) -> Rolls {
    let size = rng.randf(8000.0, 50000.0);
    let rot = rng.randi(256) as u8;
    let life = rng.rand_range(10, 30) as i16;
    Rolls { size, rot, life }
}

/// The record written as `0x34b3b8` writes it, with its draws `r`.
pub fn put(sys: &mut Particles, pos: [f32; 3], vel: [f32; 4], alpha: i32, r: Rolls) -> Option<usize> {
    let i = sys.create_part(ARC)?;
    let def = sys.def_first(ARC);
    let p = &mut sys.pool.recs[i];
    rec::set_v3(p, 0x10, pos);
    rec::set_ff(p, 0x1c, 0.0);
    p[9] = 4 + 0x70;
    p[1] = 0;
    p[3] = 0x48;
    rec::set_ff(p, 0xc, r.size);
    p[8] = r.rot;
    p[2] = def;
    rec::set_i16(p, 0xa, r.life);
    rec::set_u32(p, 0x3c, 0);
    rec::set_ff(p, 0x30, 1.0 / r.life as f32);
    rec::set_i16(p, 0x34, alpha as i16);
    rec::set_u32(p, 0x38, 0x7f_7f7f);
    rec::set_i16(p, 0x36, 0);
    rec::set_v4(p, 0x20, vel);
    rec::set_u32(p, 4, (alpha as u32) << 24 | 0x7f_7f7f);
    Some(i)
}

/// `0x34b3b8(pos, vel, alpha, owner)` (module docs): nothing drawn when the pool is full.
pub fn spawn(sys: &mut Particles, rng: &mut Rng, pos: [f32; 3], vel: [f32; 4], alpha: i32) -> Option<usize> {
    if sys.pool.hint >= 0x800 { return None; }
    let r = rolls(rng);
    put(sys, pos, vel, alpha, r)
}

/// A made record's life (`+0xa`), size (`+0xc`), colour (`+0x38`) and one over its life (`+0x30`) set again, as the
/// beams do to some of theirs; `def`: its first frame another type's.
pub fn tweak(sys: &mut Particles, i: usize, life: Option<i16>, size: f32, colour: u32, def: Option<u8>) {
    let def = def.map(|d| sys.def_first(d));
    let p = &mut sys.pool.recs[i];
    if let Some(l) = life { rec::set_i16(p, 0xa, l); }
    rec::set_ff(p, 0xc, size);
    rec::set_ff(p, 0x30, 1.0 / rec::i16(p, 0xa) as f32);
    rec::set_u32(p, 0x38, colour);
    if let Some(d) = def { p[2] = d; }
}

fn outside(p: [f32; 3]) -> bool { p.iter().any(|&c| !(2.0..=1021.0).contains(&c)) }

/// `0x34b4e0` (module docs).
pub fn update(sys: &mut Particles, rng: &mut Rng, i: usize) {
    if outside(rec::v3(&sys.pool.recs[i], 0x10)) {
        sys.kill_part(i);
        return;
    }
    let r = &mut sys.pool.recs[i];
    let (p, v) = (rec::v3(r, 0x10), rec::v3(r, 0x20));
    rec::set_v3(r, 0x10, [p[0] + v[0], p[1] + v[1], p[2] + v[2]]);
    if outside(rec::v3(r, 0x10)) {
        sys.kill_part(i);
        return;
    }
    let mut angle = r[8] as i32 + rng.randf_signed(0.0, 64.0) as i32;
    if 255 < angle { angle -= 255; }
    let r = &mut sys.pool.recs[i];
    r[8] = angle as u8;
    if dec_life(r) {
        sys.kill_part(i);
        return;
    }
    let a = rec::i16(r, 0xa) as f32 * rec::ff(r, 0x30) * rec::i16(r, 0x34) as f32;
    rec::set_u32(r, 4, ((a as i32) as u32) << 24 | rec::u32(r, 0x38));
}
