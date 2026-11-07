//! Type 0xd: the break's dust puff (L00 spawner `0x334528`, update `0x3346a8`; the decomp's `func_002BC768` /
//! `func_002BC8E8`, every level's): a grey puff that swells, rises and turns, fading in over 10 ticks and out over 30.
//! The crates' break (crate::moby::classes::crate_) makes six ([`burst`]).
//!
//! **A puff** ([`puff`], `0x334528(pos, spin, rgba, spread, low, high, rise, size)`): at `pos` moved
//! `randf_signed(0, spread)` on each axis, colour `rgba`, size `size`, angle 0, near 4 / far 4 (`+0x09` 0x44), ALPHA
//! 0x48, its type's first frame, its life 10 in phase 0 (`+0x24`), turning `spin` a tick (`−spin` one time in two,
//! `randi(2)`), growing ×`randf(low, high)` a tick (`+0x2c`), rising `rise` a tick (`+0x30`), its colour `+0x34`.
//!
//! **Each tick** ([`update`], `0x3346a8`): it turns, rises and grows; phase 0: its life down — out, phase 1 for 30
//! ticks at alpha 0x7f; else alpha `32 + (10 − life)·9.6`; phase 1: its life down — out, gone; else alpha
//! `life·4.2333333`.

use super::{rec, Particles};
use crate::rng::Rng;

/// Type 0xd.
pub const BREAK_DUST: u8 = 0xd;

/// `0x334528` (module docs); `None` when the pool has no room.
#[allow(clippy::too_many_arguments)]
pub fn puff(sys: &mut Particles, rng: &mut Rng, pos: [f32; 3], spin: i32, rgba: u32, spread: f32, low: f32, high: f32, rise: f32, size: f32) -> Option<usize> {
    let i = sys.create_part(BREAK_DUST)?;
    let def = sys.def_first(BREAK_DUST);
    let p = [rng.randf_signed(0.0, spread), rng.randf_signed(0.0, spread), rng.randf_signed(0.0, spread)];
    let spin = if rng.randi(2) != 0 { spin } else { -spin };
    let grow = rng.randf(low, high);
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x10, [pos[0] + p[0], pos[1] + p[1], pos[2] + p[2]]);
    rec::set_u32(r, 4, rgba);
    r[9] = 4 + 0x40;
    r[3] = 0x48;
    rec::set_ff(r, 0xc, size);
    r[1] = 0;
    r[8] = 0;
    r[2] = def;
    rec::set_i16(r, 0xa, 10);
    rec::set_u32(r, 0x24, 0);
    rec::set_u32(r, 0x28, spin as u32);
    rec::set_u32(r, 0x34, rgba & 0xff_ffff);
    rec::set_ff(r, 0x2c, grow);
    rec::set_ff(r, 0x30, rise);
    Some(i)
}

/// The break's dust (L00 `0x39daa0` and `0x3928f8`, the same six calls): 0.3 above `at`, one puff growing
/// ×1.01..1.075 and five ×1.01..1.07 a tick, `0x40808080`, spread 0.3, rising 0.05, size 50000, turning 1.
pub fn burst(sys: &mut Particles, rng: &mut Rng, at: [f32; 3]) {
    let p = [at[0], at[1], at[2] + 0.3];
    puff(sys, rng, p, 1, 0x4080_8080, 0.3, f32::from_bits(0x3f81_47ae), f32::from_bits(0x3f89_999a), 0.05, 50_000.0);
    for _ in 0..5 { puff(sys, rng, p, 1, 0x4080_8080, 0.3, f32::from_bits(0x3f81_47ae), f32::from_bits(0x3f88_f5c3), 0.05, 50_000.0); }
}

/// `0x3346a8` (module docs).
pub fn update(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    r[8] = r[8].wrapping_add(r[0x28]);
    rec::set_ff(r, 0x18, rec::ff(r, 0x18) + rec::ff(r, 0x30));
    rec::set_ff(r, 0xc, rec::ff(r, 0xc) * rec::ff(r, 0x2c));
    let rgb = rec::u32(r, 0x34);
    let mut t = rec::i16(r, 0xa) as i32;
    let out = crate::moby::countdown(&mut t);
    rec::set_i16(r, 0xa, t as i16);
    if rec::u32(r, 0x24) == 0 {
        if out {
            rec::set_u32(r, 0x24, 1);
            rec::set_i16(r, 0xa, 30);
            rec::set_u32(r, 4, rgb | 0x7f00_0000);
        } else {
            let v = ((10 - t) as f32 * 9.6) as i32;
            rec::set_u32(r, 4, ((v + 32) as u32) << 24 | rgb);
        }
    } else if out {
        sys.kill_part(i);
    } else {
        let v = (t as f32 * f32::from_bits(0x4087_7778)) as i32;
        rec::set_u32(r, 4, (v as u32) << 24 | rgb);
    }
}
