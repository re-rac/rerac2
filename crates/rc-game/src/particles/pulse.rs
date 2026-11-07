//! Type 0x54: a pulse (L17 `0x342180` / `0x342250`; the decomp's `func_L17_00340DD0` / `func_L17_00340EA0`): a
//! camera-facing sprite (record kind 0) at a moby's place whose alpha swells and fades once over its life. Smolg's
//! glows 0x09ad put them out (crate::moby::classes::level17::misc).
//!
//! **A pulse** ([`spawn`], `0x342180(size, moby, rgba, life)`): at the moby's place, `+0x20` the moby, `+0x24` /
//! `+0x26` the life left and the whole life, `+0x2c` the alpha (`rgba >> 24`), its colour `rgba`, a random angle
//! (`0x1163b0`), `+0x09` 0x74 (near 4 / far 7), ALPHA 0x48 (additive), its size, its type's first frame.
//! **Each tick** ([`update`], `0x342250`): the life down (out: gone); its alpha `|alpha · sin(2π · left / life)|`.

use super::{rec, Particles};
use crate::rng::Rng;

/// Type 0x54.
pub const PULSE: u8 = 0x54;

/// `0x342180` (module docs); `None` when the pool has no room.
pub fn spawn(sys: &mut Particles, rng: &mut Rng, size: f32, owner: usize, pos: [f32; 3], rgba: u32, life: i16) -> Option<usize> {
    let i = sys.create_part(PULSE)?;
    let def = sys.def_first(PULSE);
    let rot = rng.randi(256) as u8;
    let r = &mut sys.pool.recs[i];
    rec::set_u32(r, 0x20, owner as u32);
    rec::set_i16(r, 0x24, life);
    rec::set_u32(r, 0x2c, (rgba as i32 >> 24) as u32);
    rec::set_i16(r, 0x26, life);
    rec::set_v3(r, 0x10, pos);
    rec::set_ff(r, 0x1c, 1.0);
    rec::set_u32(r, 4, rgba);
    r[8] = rot;
    r[9] = 4 + 0x70;
    r[3] = 0x48;
    rec::set_ff(r, 0xc, size);
    r[1] = 0;
    r[2] = def;
    Some(i)
}

/// `0x342250` (module docs).
pub fn update(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let mut t = rec::i16(r, 0x24) as i32;
    let done = crate::moby::countdown(&mut t);
    rec::set_i16(r, 0x24, t as i16);
    if done {
        sys.kill_part(i);
        return;
    }
    let s = (t as f32 / rec::i16(r, 0x26) as f32 * 6.283_186).sin();
    let a = (rec::u32(r, 0x2c) as i32 as f32 * s).abs() as i32;
    let c = rec::u32(r, 4);
    rec::set_u32(r, 4, (c & 0xff_ffff).wrapping_add((a as u32) << 24));
}
