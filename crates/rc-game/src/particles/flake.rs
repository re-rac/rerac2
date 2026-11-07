//! Type 0x49: the weather's snowflake (L01 spawner `0x332f80`, update `0x3330e8`; L15 `0x355b68` / `0x355cd0`, L19
//! `0x33ac28` / `0x33ad90`; the decomp's `func_L01_00331CA0` / `func_L01_00331E08`): the snow the weather
//! (crate::moby::classes::weather) makes on the levels 0xc / 0x13 / 0x200 where the rain's drops would be.
//!
//! **A flake** ([`flake`], `0x332f80(pos, kind, vel, ground)`): kind 0 size 0.12·210000 at alpha 0x20, kind 1 0.5 at
//! 0xd (each after a draw `randf(x, x)` that changes nothing: 0.25, 1), white; `+0x0a` 0x37, near 4 / far 2 (`+0x09`
//! 0x24), ALPHA 0x48, its frame the kind's of the type's list (`def[0x49][kind]`), its roll `randf(0, 360)`
//! truncated; its velocity `+0x20` and its ground `+0x2c`.
//!
//! **Each tick** ([`update`], `0x3330e8`): gone out of the 0..512 box or under its ground; it moves by its velocity
//! and the camera's move (`0x1b27a0`); into another unit square its ground looked up again (the rain's top
//! `0x31c630`, crate::particles::rain::top, at least the floor `0x1b27b0`) and gone under it; its z velocity
//! −1/7200 a tick; its ground moved with the camera's z.

use super::rain::{top, RainAux, FLOOR};
use super::{rec, Particles};
use crate::rng::Rng;

/// Type 0x49.
pub const FLAKE: u8 = 0x49;

/// `0x332f80` (module docs); `None` when the pool has no room.
pub fn flake(sys: &mut Particles, rng: &mut Rng, pos: [f32; 3], kind: i16, vel: [f32; 3], ground: f32) -> Option<usize> {
    let i = sys.create_part(FLAKE)?;
    let (size, tint) = match kind {
        0 => {
            rng.randf(0.25, 0.25);
            (0.12, 0x20u32)
        }
        1 => {
            rng.randf(1.0, 1.0);
            (0.5, 0xd)
        }
        _ => (0.0, 0),
    };
    let def = sys.def_frame(FLAKE, kind.max(0) as usize);
    let roll = rng.randf(0.0, 360.0) as i32 as u8;
    let r = &mut sys.pool.recs[i];
    rec::set_i16(r, 0xa, 0x37);
    rec::set_u32(r, 4, tint << 24 | 0xff_ffff);
    r[9] = 4 + 0x20;
    r[3] = 0x48;
    r[1] = 0;
    rec::set_ff(r, 0xc, size * 210_000.0);
    r[2] = def;
    r[8] = roll;
    rec::set_v3(r, 0x10, pos);
    rec::set_ff(r, 0x1c, 0.0);
    rec::set_v3(r, 0x20, vel);
    rec::set_ff(r, 0x2c, ground);
    Some(i)
}

/// `0x3330e8` (module docs).
pub fn update(sys: &mut Particles, i: usize, aux: &RainAux) {
    let r = &mut sys.pool.recs[i];
    let mut ground = rec::ff(r, 0x2c);
    let p = rec::pos(r);
    if p[0] < 0.0 || p[1] < 0.0 || 512.0 < p[0] || 512.0 < p[1] || p[2] < ground {
        sys.kill_part(i);
        return;
    }
    let v = rec::v3(r, 0x20);
    let n: [f32; 3] = std::array::from_fn(|k| p[k] + v[k] + aux.cam_move[k]);
    rec::set_v3(r, 0x10, n);
    if p[0] as i32 != n[0] as i32 || p[1] as i32 != n[1] as i32 {
        let h = top(aux.map, aux.live, n);
        ground = if FLOOR <= h { h } else { FLOOR };
        if n[2] < ground {
            sys.kill_part(i);
            return;
        }
    }
    let r = &mut sys.pool.recs[i];
    rec::set_ff(r, 0x28, v[2] - 0.000_138_888_89);
    rec::set_ff(r, 0x2c, ground + aux.cam_move[2]);
}
