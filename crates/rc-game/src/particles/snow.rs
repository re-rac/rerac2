//! The snowflakes, particle type 0x68: their tick ([`update`], L00 `0x341620`, L07 `0x337bc0`: the engine's
//! `func_002C8610`, in every level). Their maker the port has: level 7's snow (crate::moby::classes::level07::snow's
//! `flake`, L07 `0x337a30`).
//!
//! **Each tick**: more than 7 under the camera (`0x1b9188`), or a kind-1 flake's life out: its top one down (gone at
//! 0) and its alpha that; else its alpha up by one to the top. It rolls 2 a tick (back on records whose address has
//! bit 0x80: here the index's bit 1), moves by its velocity and the drift (`0x1b24d0`, kept by level 7's convoy:
//! crate::moby::classes::level07::train::Globals); a kind-0 flake sways `sin(2π · (frame mod period) / period) / 60`
//! along the camera's left row (`0x1b93c0`).

use crate::hero::physics::{add, scale, V3};
use crate::moby::MobyWorld;
use crate::particles::{rec, Particles};

/// The flakes' particle type.
pub const FLAKE: u8 = 0x68;

/// `0x337bc0` (module docs): `cam` the camera (`0x1b9180`), `mobys` for its rows, the frame and the drift.
pub fn update(sys: &mut Particles, i: usize, mobys: &MobyWorld, cam: V3) {
    let r = &mut sys.pool.recs[i];
    let kind1 = rec::u32(r, 0x34) != 0;
    let mut out = rec::ff(r, 0x18) - cam[2] < -7.0;
    if !out && kind1 {
        let mut t = rec::i16(r, 0xa) as i32;
        out = crate::moby::countdown(&mut t);
        rec::set_i16(r, 0xa, t as i16);
    }
    let c = rec::u32(r, 4);
    if out {
        let top = rec::u32(r, 0x30) as i32 - 1;
        rec::set_u32(r, 0x30, top as u32);
        if top < 1 {
            sys.kill_part(i);
            return;
        }
        rec::set_u32(r, 4, (c & 0xff_ffff).wrapping_add((top as u32) << 24));
    } else if ((c as i32) >> 24) < rec::u32(r, 0x30) as i32 {
        rec::set_u32(r, 4, c.wrapping_add(0x100_0000));
    }
    r[8] = if (i >> 1) & 1 == 0 { r[8].wrapping_add(2) } else { r[8].wrapping_sub(2) };
    let mut p = add(add(rec::v3(r, 0x10), rec::v3(r, 0x20)), mobys.convoy.drift);
    if !kind1 {
        let period = rec::i16(r, 0xa) as i32;
        if period != 0 {
            let f = (mobys.frame & (period - 1)) as f32 / period as f32;
            let s = (f * 6.283_185_5).sin();
            p = add(p, scale(mobys.cam_rows[1], s * 0.016_666_668));
        }
    }
    rec::set_v3(r, 0x10, p);
}
