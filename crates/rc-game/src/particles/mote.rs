//! Type 0x62: a mote (`0x340a78` / `0x340c18`, L14 `0x34ae20` / `0x34afc0`; the decomp's `func_002C7BB8` /
//! `func_002C7D58`): a camera-facing sprite (record kind 0) moving with its velocity, falling, turning, its size and
//! alpha stepped over its life. Level 14's prison force fields put them out (crate::moby::classes::level14::field).
//!
//! **A mote** ([`mote`], `0x340a78(size, end size, gravity, pos, vel, life, rgb, spin, alpha, end alpha, kind)`):
//! ALPHA 0x48 (additive), `+0x09` 0x44 (near 4 / far 4), its frame entry `kind` of the type's def list
//! (`*(def[0x62] + kind)`), its colour `rgb` at `alpha`, its angle `randi(spin·100) mod 255`, the size step
//! `(end size − size) / life` (`+0x30`), the alpha step `(end alpha − alpha) / life` (`+0x34`, a byte), the spin
//! (`+0x35`), the gravity (`+0x2c`), `+0x38` 60. Each tick ([`update`], `0x340c18`): the life down (out: gone), the
//! alpha up by its step, the move, the turn, the size step, the velocity's z down by the gravity.

use super::{rec, Particles};
use crate::rng::Rng;

/// Type 0x62.
pub const MOTE: u8 = 0x62;

/// `0x340a78` (module docs).
#[allow(clippy::too_many_arguments)]
pub fn mote(sys: &mut Particles, rng: &mut Rng, size: f32, end_size: f32, gravity: f32, pos: [f32; 3], vel: [f32; 3], life: i32, rgb: u32, spin: i32, alpha: i32, end_alpha: i32, kind: usize) -> Option<usize> {
    let i = sys.create_part(MOTE)?;
    let def = sys.def_frame(MOTE, kind);
    let rot = rng.rand_range(0, spin * 100) % 0xff;
    let life = life.max(1);
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x10, pos);
    rec::set_ff(r, 0x1c, 1.0);
    rec::set_u32(r, 4, (alpha as u32) << 24 | rgb);
    r[9] = 0x40 + 4;
    r[1] = 0;
    rec::set_ff(r, 0xc, size);
    r[3] = 0x48;
    r[8] = rot as u8;
    r[2] = def;
    rec::set_i16(r, 0xa, life as i16);
    rec::set_v3(r, 0x20, vel);
    rec::set_ff(r, 0x2c, gravity);
    r[0x35] = spin as u8;
    rec::set_ff(r, 0x30, (end_size - size) / life as f32);
    r[0x34] = ((end_alpha - alpha) / life) as u8;
    rec::set_u32(r, 0x38, 60);
    rec::set_u32(r, 0x3c, 0x340c18);
    Some(i)
}

/// `0x340c18` (module docs).
pub fn update(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let mut t = rec::i16(r, 0xa) as i32;
    let done = crate::moby::countdown(&mut t);
    rec::set_i16(r, 0xa, t as i16);
    if done {
        sys.kill_part(i);
        return;
    }
    let c = rec::u32(r, 4);
    let a = ((c >> 24) as i32 + r[0x34] as i32) as u32;
    rec::set_u32(r, 4, c & 0xff_ffff | a.wrapping_shl(24));
    let (p, v) = (rec::v3(r, 0x10), rec::v3(r, 0x20));
    rec::set_v3(r, 0x10, [p[0] + v[0], p[1] + v[1], p[2] + v[2]]);
    r[8] = r[8].wrapping_add(r[0x35]);
    rec::set_ff(r, 0xc, rec::ff(r, 0xc) + rec::ff(r, 0x30));
    rec::set_ff(r, 0x28, rec::ff(r, 0x28) - rec::ff(r, 0x2c));
}
