//! The rain's particles (L01 addresses; the type table `0x1fee40[type]` that `0x327af8` fills), spawned by the
//! weather (crate::moby::classes::weather).
//!
//! | Type | Spawner | Update | |
//! |---|---|---|---|
//! | 0 | [`drop`] `0x3280b8(ground, pos, kind, vel)` | `0x3282b8` ([`update_drop`]) | a drop: a ribbon falling with gravity, moved with the camera, gone under the height map |
//! | 1 | [`splash`] `0x3284b8(pos, alpha)` | `0x328590` ([`update_splash`]) | a splash on the ground: a sprite through its frames, fading over 27 ticks |
//!
//! The drops stop on the level's height map (`0x31c630`, [`top`]) and on the floor word `0x1b27b0` (never written on
//! Oozla: 0). The drop's update is the same on every level but 0xf, which skips the 0..1024 box.

use super::{rec, Particles};
use crate::hero::physics::{add, scale};
use crate::rng::Rng;
use rc_formats::height_map::HeightMap;

pub const DROP: u8 = 0;
pub const SPLASH: u8 = 1;
/// `0x1b27b0`: the floor the drops never fall below (0: never written on the weather's levels).
const FLOOR: f32 = 0.0;
/// The splash's life in ticks.
const SPLASH_LIFE: i16 = 0x1b;

/// What the rain's updates read of the world each tick.
#[derive(Clone, Copy, Default)]
pub struct RainAux<'a> {
    /// The level's height map (`0x1b2420`).
    pub map: Option<&'a HeightMap>,
    /// The map is the loaded variant's: `0x1a94c0` (the weather writes the variant `0x1c9a40` there each tick) is
    /// the variant.
    pub live: bool,
    /// `0x1b27a0`: the camera's move this tick.
    pub cam_move: [f32; 3],
    /// `0x1a79f8`: the level.
    pub level: i32,
}

/// `0x31c630(p)`: the height the rain stops at over `p` — 0 without a map, 1023 outside it, over an empty cell or
/// when the map is not the loaded variant's.
pub fn top(map: Option<&HeightMap>, live: bool, p: [f32; 3]) -> f32 {
    let Some(m) = map else { return 0.0 };
    if !live { return 1023.0; }
    m.height(p).unwrap_or(1023.0)
}

/// `0x3280b8(ground, pos, kind, vel)`: a drop (type 0) from `pos` to `pos + vel·len` moving `vel` a tick, ending at
/// `ground`. Kind 0: width 0.015, alpha 0x14, len `randf(0.01, 1)`; 1: 0.5, 0xd, `randf(1, 5)`; 2: 1, 0xd,
/// `randf(20, 30)` with kind 1's def. Colours `(alpha/4)·0x606060` and `alpha·0x907070`, the ribbon (`+1` = 3),
/// ALPHA 0x44.
pub fn drop(sys: &mut Particles, rng: &mut Rng, ground: f32, pos: [f32; 3], kind: i16, vel: [f32; 3]) -> Option<usize> {
    let i = sys.create_part(DROP)?;
    let (width, alpha, len, def) = match kind {
        0 => (0.015, 0x14, rng.randf(0.01, 1.0), 0),
        1 => (0.5, 0xd, rng.randf(1.0, 5.0), 1),
        2 => (1.0, 0xd, rng.randf(20.0, 30.0), 1),
        _ => (0.0, 0, 0.0, kind as usize),
    };
    let def = sys.def_frame(DROP, def);
    let r = &mut sys.pool.recs[i];
    rec::set_i16(r, 0xa, kind);
    rec::set_u32(r, 4, ((alpha >> 2) as u32) << 24 | 0x60_6060);
    rec::set_u32(r, 0xc, (alpha as u32) << 24 | 0x90_7070);
    r[9] = 4 + 0x60;
    r[1] = 3;
    r[3] = 0x44;
    r[2] = def;
    rec::set_v3(r, 0x10, pos);
    rec::set_v3(r, 0x20, add(pos, scale(vel, len)));
    rec::set_ff(r, 0x1c, width);
    rec::set_ff(r, 0x2c, 1.0);
    rec::set_v3(r, 0x30, vel);
    rec::set_ff(r, 0x3c, ground);
    Some(i)
}

/// `0x3282b8`: gone once its back end (`+0x18`) is under its ground (`+0x3c`) or (but on level 0xf) it leaves the
/// 0..1024 box; both ends move by its velocity and the camera's move; on crossing into another unit square the front
/// end's ground is looked up again (not for kind 2) and the drop goes when its front end is under it; the velocity's z
/// gains gravity (−0.0027222224 a tick) and the ground moves with the camera's z.
pub fn update_drop(sys: &mut Particles, i: usize, aux: &RainAux) {
    let r = &mut sys.pool.recs[i];
    let mut ground = rec::ff(r, 0x3c);
    let p = rec::pos(r);
    let gone = if aux.level == 0xf { p[2] < ground } else { !(ground <= p[2] && (0.0..=1024.0).contains(&p[0]) && (0.0..=1024.0).contains(&p[1])) };
    if gone {
        sys.kill_part(i);
        return;
    }
    let (ox, oy) = (rec::ff(r, 0x20), rec::ff(r, 0x24));
    let m = add(rec::v3(r, 0x30), aux.cam_move);
    let front = add(rec::v3(r, 0x20), m);
    rec::set_v3(r, 0x20, front);
    rec::set_v3(r, 0x10, add(p, m));
    if rec::i16(r, 0xa) != 2 && (ox as i32 != front[0] as i32 || oy as i32 != front[1] as i32) {
        let h = top(aux.map, aux.live, front);
        ground = if FLOOR <= h { h } else { FLOOR };
        if front[2] < ground {
            sys.kill_part(i);
            return;
        }
    }
    let r = &mut sys.pool.recs[i];
    rec::set_ff(r, 0x38, rec::ff(r, 0x38) - 0.002_722_222_4);
    rec::set_ff(r, 0x3c, ground + aux.cam_move[2]);
}

/// `0x3284b8(pos, alpha)`: a splash (type 1) 0.2 above `pos`, size 63000, angle 0xa0, colour `alpha·0x907070` (its
/// start alpha kept at `+0x20`), ALPHA 0x44, for 27 ticks.
pub fn splash(sys: &mut Particles, pos: [f32; 3], alpha: i32) -> Option<usize> {
    let i = sys.create_part(SPLASH)?;
    let def = sys.def_frame(SPLASH, 0);
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x10, [pos[0], pos[1], pos[2] + 0.2]);
    rec::set_u32(r, 4, (alpha as u32) << 24 | 0x90_7070);
    r[9] = 2 + 0x40;
    r[3] = 0x44;
    r[1] = 0;
    rec::set_u32(r, 0x20, alpha as u32);
    r[2] = def;
    rec::set_u32(r, 0xc, 0x4776_1801);
    r[8] = 0xa0;
    rec::set_i16(r, 0xa, SPLASH_LIFE);
    Some(i)
}

/// `0x328590`: the alpha is the start alpha · life / 27; the life counts down (`0x2e3e18`) and at 0 it goes, else its
/// frame is the `(27 − life) / 3`th of type 1's.
pub fn update_splash(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let life = rec::i16(r, 0xa) as i32;
    let a = (life * rec::u32(r, 0x20) as i32) / SPLASH_LIFE as i32;
    rec::set_u32(r, 4, rec::u32(r, 4) & 0xff_ffff | (a as u32) << 24);
    let mut t = life;
    let done = crate::moby::countdown(&mut t);
    rec::set_i16(r, 0xa, t as i16);
    if done {
        sys.kill_part(i);
        return;
    }
    let def = sys.def_frame(SPLASH, ((SPLASH_LIFE as i32 - t) / 3) as usize);
    sys.pool.recs[i][2] = def;
}
