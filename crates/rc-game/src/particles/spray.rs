//! Type 0x53: a spray (L06 `0x37fae8` / `0x37fc00`; the decomp's `func_L06_0037E6B0`): a camera-facing sprite (record
//! kind 0) thrown out, falling, growing and turning, that rings the water and drips once low. Notak's vents 0x0feb
//! put them out (crate::moby::classes::level06::props::update_0feb).
//!
//! **A spray** ([`spawn`], `0x37fae8(size, floor, pos, vel, life)`): its colour 0x50502010, `+0x09` 0x42 (near 2 /
//! far 4), ALPHA 0x48 (additive), its size `size`·105000, a random angle (`randi(255)`), its type's first frame;
//! `+0x20` the velocity (its w the life as a float), `+0x30` the growth 0.5, `+0x34` `size`, `+0x38` its age 0,
//! `+0x3a` its spin `randi(6) − 3`, `+0x3c` the floor.
//!
//! **Each tick** ([`update`], `0x37fc00`): its velocity's z down by 0.0005 ± 0.0001 (`$gp−0x498c` / `$gp−0x4988`), the
//! growth up 0.0075, its size `+0x34`·growth·210000, its colour 0x55503020, its angle on by its spin, the move. At or
//! under its floor, one tick in six a ring on the water there (`0x37aa30`, crate::particles::water::ring: 0.4, growing
//! 31500 a tick, the grey default); from its 50th tick, one in six a drop (`0x378a58`, crate::particles::water::drop:
//! kind 1, 20 ticks) with its velocity. Its age up; its life down (out: gone).

use super::{rec, Particles};
use crate::rng::Rng;

/// Type 0x53.
pub const SPRAY: u8 = 0x53;

/// `0x37fae8` (module docs); `None` when the pool has no room.
pub fn spawn(sys: &mut Particles, rng: &mut Rng, size: f32, floor: f32, pos: [f32; 3], vel: [f32; 3], life: i16) -> Option<usize> {
    let i = sys.create_part(SPRAY)?;
    let def = sys.def_first(SPRAY);
    let rot = rng.randi(0xff) as u8;
    let spin = rng.randi(6) as i16 - 3;
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x10, pos);
    rec::set_ff(r, 0x1c, 1.0);
    rec::set_u32(r, 4, 0x5050_2010);
    r[9] = 2 + 0x40;
    r[3] = 0x48;
    r[1] = 0;
    rec::set_ff(r, 0xc, size * 105_000.0);
    r[2] = def;
    r[8] = rot;
    rec::set_i16(r, 0xa, life);
    rec::set_v3(r, 0x20, vel);
    rec::set_ff(r, 0x34, size);
    rec::set_ff(r, 0x3c, floor);
    rec::set_i16(r, 0x38, 0);
    rec::set_ff(r, 0x2c, life as f32);
    rec::set_ff(r, 0x30, 0.5);
    rec::set_i16(r, 0x3a, spin);
    Some(i)
}

/// `0x37fc00` (module docs).
pub fn update(sys: &mut Particles, i: usize, rng: &mut Rng) {
    let g = rng.randf(-f32::from_bits(0x38d1_b717), f32::from_bits(0x38d1_b717));
    let (pos, vel, floor, age) = {
        let r = &mut sys.pool.recs[i];
        let mut v = rec::v3(r, 0x20);
        v[2] -= f32::from_bits(0x3a03_126f) + g;
        rec::set_v3(r, 0x20, v);
        let grow = rec::ff(r, 0x30) + f32::from_bits(0x3bf5_c28f);
        rec::set_ff(r, 0x30, grow);
        rec::set_ff(r, 0xc, rec::ff(r, 0x34) * grow * 210_000.0);
        rec::set_u32(r, 4, 0x5550_3020);
        r[8] = r[8].wrapping_add(r[0x3a]);
        let p = rec::v3(r, 0x10);
        let p = [p[0] + v[0], p[1] + v[1], p[2] + v[2]];
        rec::set_v3(r, 0x10, p);
        (p, v, rec::ff(r, 0x3c), rec::i16(r, 0x38))
    };
    if pos[2] <= floor && rng.randi(6) == 1 {
        super::water::ring(sys, rng, 0.4, f32::from_bits(0x46f6_1801), pos, false, None);
    }
    if rng.randi(6) == 1 && 50 <= age {
        super::water::drop(sys, rng, pos, vel, 1, 20);
    }
    let r = &mut sys.pool.recs[i];
    rec::set_i16(r, 0x38, age.wrapping_add(1));
    let mut t = rec::i16(r, 0xa) as i32;
    let done = crate::moby::countdown(&mut t);
    rec::set_i16(r, 0xa, t as i16);
    if done { sys.kill_part(i); }
}
