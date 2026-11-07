//! Type 0x67: a spot (L04 `0x364f80` / `0x365030`, L06 `0x3824b0`, L09 `0x33e328`, L10 `0x3559e8`, L12 `0x3467c0`,
//! L13 `0x34b058`, L14 `0x34b990`, L16 `0x339418`, L17 `0x344980`; the decomp's `func_L04_00363CA0` and its update
//! `func_L00_003402B8`): a camera-facing sprite (record kind 0) shown for two ticks — the glows the tie waves 0x1137
//! put on their ties each tick (crate::moby::classes::level04::tie_wave) and Hrugis's cloud glints 0x11b9
//! (crate::moby::classes::level10::glints).
//!
//! **A spot** ([`spot`], `0x364f80(pos, rgba, size)`): ALPHA 0x48 (additive), `+0x09` `(int)2.0 − 0x10` (near 2 / far
//! 15), its type's first frame, its colour `rgba`, a random angle (`0x33cca0(0xff)`), the life 2. **Each tick**
//! ([`update`], `0x365030`): the life down (`0x30f718`); out: gone.

use super::{rec, Particles};
use crate::rng::Rng;

/// Type 0x67.
pub const SPOT: u8 = 0x67;

/// `0x364f80` (module docs); `None` when the pool has no room.
pub fn spot(sys: &mut Particles, rng: &mut Rng, pos: [f32; 3], rgba: u32, size: f32) -> Option<usize> {
    let i = sys.create_part(SPOT)?;
    let def = sys.def_first(SPOT);
    let rot = rng.randi(0xff) as u8;
    let r = &mut sys.pool.recs[i];
    rec::set_u32(r, 4, rgba);
    r[9] = 2u8.wrapping_sub(0x10);
    r[3] = 0x48;
    r[1] = 0;
    r[2] = def;
    rec::set_ff(r, 0xc, size);
    r[8] = rot;
    rec::set_i16(r, 0xa, 2);
    rec::set_v3(r, 0x10, pos);
    Some(i)
}

/// `0x365030` (module docs).
pub fn update(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let mut t = rec::i16(r, 0xa) as i32;
    let done = crate::moby::countdown(&mut t);
    rec::set_i16(r, 0xa, t as i16);
    if done { sys.kill_part(i); }
}
