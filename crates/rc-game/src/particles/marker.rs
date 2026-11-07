//! Type 0x29: a marker, a sprite held a few ticks (or until killed) — spawner L03 `0x33a558` (L06 `0x379b90`, L08
//! `0x347620`, L09 `0x3359a8`, L11 `0x34ce50`, L13 `0x342850`, L14 `0x343188`, L15 `0x351710`, L17 `0x33c0b0`, L18
//! `0x354398`; the decomp's `func_L03_00339128`), update L00 `0x338e00` (L03 `0x33a698`, L15 `0x351850`; the decomp's
//! `func_002C0C70`, every level's). Its callers: the path markers ([`path`], Gorn's convoy
//! crate::moby::classes::level15::convoy).
//!
//! **A marker** ([`marker`], `(pos, moby, frame, rgba, size, near, life, alpha)`): at `pos` (its w too), its colour
//! `rgba`, size `size`·210000, its frame `def[0x29][frame]`, angle 0xa0, its life `life` (−1: kept until killed), the
//! moby `+0x20`; `alpha` 0xff: near 4 / far `(near >> 5)·16` (`+0x09`), ALPHA 0x48 for an odd `near` (else 0x44);
//! else `+0x09` `near` and ALPHA `alpha`.
//!
//! **Each tick** ([`update`], `0x338e00`): a life of −1 stays; else the life down (`0x2eeae0`), out: gone.
//!
//! **The path markers** ([`path`], L06 `0x383a40` / L15 `0x35b558`, the decomp's `func_L06_00382608`, `(moby, path,
//! rgba, flags, size, step)`): a path of −1 only prints a message; else every `step`-th point of the level's path
//! (`0x1fe480[path]`) gets a marker of size `size`, near 0x7f, alpha 0xff, life 2 (−1 with `flags` bit 0x40): `flags
//! & 7` 1 frame 0, 2 frame 1; 4 the point's index in digits (frames 2..11, 0.5·size up, each the next along a vector
//! of 1.2·size, `0x1b9990`): not ported, nothing calls it outside the debug paths; other modes nothing.

use super::{rec, Particles};

/// Type 0x29.
pub const MARKER: u8 = 0x29;

/// A marker's look (module docs): the colour, the size, the near byte and the alpha mode, the life.
#[derive(Clone, Copy)]
pub struct Look {
    pub rgba: u32,
    pub size: f32,
    pub near: u8,
    pub life: i16,
    pub alpha: u8,
}

/// `0x33a558` (module docs); `None` when the pool has no room.
pub fn marker(sys: &mut Particles, pos: [f32; 4], moby: Option<usize>, frame: usize, look: &Look) -> Option<usize> {
    let i = sys.create_part(MARKER)?;
    let def = sys.def_frame(MARKER, frame);
    let r = &mut sys.pool.recs[i];
    rec::set_v4(r, 0x10, pos);
    rec::set_u32(r, 4, look.rgba);
    if look.alpha == 0xff {
        r[9] = 4 + ((look.near >> 5) << 4);
        r[3] = if look.near & 1 != 0 { 0x48 } else { 0x44 };
    } else {
        r[9] = look.near;
        r[3] = look.alpha;
    }
    r[1] = 0;
    rec::set_ff(r, 0xc, look.size * 210_000.0);
    r[2] = def;
    rec::set_i16(r, 0xa, look.life);
    r[8] = 0xa0;
    rec::set_u32(r, 0x20, moby.map_or(0, |m| m as u32 + 1));
    Some(i)
}

/// `0x338e00` (module docs).
pub fn update(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let mut t = rec::i16(r, 0xa) as i32;
    if t == -1 { return; }
    let done = crate::moby::countdown(&mut t);
    rec::set_i16(r, 0xa, t as i16);
    if done { sys.kill_part(i); }
}

/// `0x383a40` (module docs): the markers along `points` (the level's path, `None` for −1).
pub fn path(sys: &mut Particles, moby: usize, points: Option<&[[f32; 4]]>, rgba: u32, flags: u32, size: f32, step: usize) {
    let Some(points) = points else { return };
    let look = Look { rgba, size, near: 0x7f, life: if flags & 0x40 != 0 { -1 } else { 2 }, alpha: 0xff };
    let frame = match flags & 7 {
        1 => 0,
        2 => 1,
        _ => return,
    };
    for p in points.iter().step_by(step.max(1)) {
        marker(sys, *p, Some(moby), frame, &look);
    }
}
