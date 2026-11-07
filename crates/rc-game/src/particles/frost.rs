//! The Thermanator's frost (type 0x60, L00 `0x340608` / `0x340748`; crate::moby::classes::thermanator): the cold
//! shot's trail, eight a tick about it.
//!
//! **A grain** (`0x340608(pos, vel, life)`): far 2 (`+0x09` 0x42), ALPHA 0x44, frame `*def[0x60]`, size
//! `randf(0.1, 0.4)`·105000 (`gp−0x48c4` / `−0x48c0`), a random roll, `life` ticks, the colour `0x40808080`, at `pos`
//! moving `vel` (the shot's) a tick; `+0x38` twice its size (unread), its roll's step `rand_range(0, 3)` (`+0x34`),
//! its fade from half its life (`+0x30`), its fall `randf(9, 11)` a second² (`gp−0x48bc`, + 2; × 1/3600). Each tick
//! (`0x340748`): its velocity × 0.9 (`gp−0x48b8`) and its fall taken off; it moves; under half its life its alpha
//! `life / half · 64` (truncated), grey `0x808080`; its roll on a step; its life out: gone.
//!
//! Not ported: the gravity frames' bends (`0x380e08`).

use super::{rec, Particles};
use crate::rng::Rng;

/// Type 0x60.
pub const FROST: u8 = 0x60;
/// `gp−0x48c4..−0x48b8` (L00; L04's `gp−0x4984..`: the same values): the size's range, the fall's least, the drag.
const SIZE: [u32; 2] = [0x3dcc_cccd, 0x3ecc_cccd];
const FALL: f32 = 9.0;
const DRAG: u32 = 0x3f66_6666;

/// `0x340608` (module docs).
pub fn grain(sys: &mut Particles, rng: &mut Rng, pos: [f32; 3], vel: [f32; 3], life: i32) -> Option<usize> {
    let i = sys.create_part(FROST)?;
    let def = sys.def_first(FROST);
    let size = rng.randf(f32::from_bits(SIZE[0]), f32::from_bits(SIZE[1])) * 105_000.0;
    let roll = rng.randi(0xff) as u8;
    let step = rng.rand_range(0, 3);
    let fall = rng.randf(FALL, FALL + 2.0) * f32::from_bits(0x3991_a2b4);
    let r = &mut sys.pool.recs[i];
    r[9] = 2 + 0x40;
    r[3] = 0x44;
    r[1] = 0;
    r[2] = def;
    rec::set_ff(r, 0xc, size);
    r[8] = roll;
    rec::set_i16(r, 0xa, life as i16);
    rec::set_u32(r, 4, 0x4080_8080);
    rec::set_v3(r, 0x10, pos);
    rec::set_v3(r, 0x20, vel);
    rec::set_u32(r, 0x2c, 0);
    rec::set_ff(r, 0x38, size + size);
    rec::set_u32(r, 0x34, step as u32);
    rec::set_u32(r, 0x30, (life / 2) as u32);
    rec::set_ff(r, 0x3c, fall);
    Some(i)
}

/// `0x340748` (module docs).
pub fn update(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let mut v = rec::v3(r, 0x20).map(|x| x * f32::from_bits(DRAG));
    v[2] -= rec::ff(r, 0x3c);
    rec::set_v3(r, 0x20, v);
    let p = rec::v3(r, 0x10);
    rec::set_v3(r, 0x10, [p[0] + v[0], p[1] + v[1], p[2] + v[2]]);
    let life = rec::i16(r, 0xa) as i32;
    let half = rec::u32(r, 0x30) as i32;
    let colour = if life < half { ((life as f32 / half as f32 * 64.0) as i32 as u32) << 24 | 0x80_8080 } else { 0x4080_8080 };
    rec::set_u32(r, 4, colour);
    r[8] = r[8].wrapping_add(r[0x34]);
    let mut n = life;
    let out = crate::moby::countdown(&mut n);
    rec::set_i16(r, 0xa, n as i16);
    if out { sys.kill_part(i); }
}
