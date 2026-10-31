//! Particle type 0x5b, the cinder (L02 spawner `0x3468b8`, update `0x346a30` in the type table `0x1ffe80`): an ember
//! that keeps its size, its colour going from its start to its end over its life, falling a set amount a tick and
//! (with a trail) leaving type-0xf embers behind (crate::particles::ember). The big explosion's sparks are its first
//! consumer (crate::moby::classes::explosion). Its record is the ember's (`0x3348c0`) but for the fall at `+0x2c` and
//! the size kept.

use super::{dec_life, ember, rec, tween_color, Particles};
use crate::hero::physics::{add, len, scale, set_len};
use crate::rng::Rng;

/// Type 0x5b.
pub const CINDER: u8 = 0x5b;

/// `0x3468b8(size, fall, pos, vel, c0, c1, life, trail, def, alpha, w)`: a cinder (type 0x5b) at `pos` moving `vel`
/// a tick, falling `fall`/3600 more each tick, going from `c0` to `c1` over `life` ticks (none with 0); `trail`: it
/// leaves embers behind every other frame; its frame `def` of its type's list (None: the first), ALPHA `alpha` (None:
/// 0x48); a random roll; near 4 / far 0xa (byte 9 0xa4).
#[allow(clippy::too_many_arguments)]
pub fn cinder(sys: &mut Particles, rng: &mut Rng, size: f32, fall: f32, pos: [f32; 3], vel: [f32; 3], c0: u32, c1: u32, life: i32, trail: i16, def: Option<usize>, alpha: Option<u8>, w: i32) -> Option<usize> {
    if life == 0 { return None; }
    let i = sys.create_part(CINDER)?;
    let frame = def.map_or_else(|| sys.def_first(CINDER), |k| sys.def_frame(CINDER, k));
    let rot = rng.rand() as u8;
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x10, pos);
    rec::set_ff(r, 0x1c, w as f32);
    rec::set_u32(r, 4, c0);
    r[9] = 0xa4;
    r[3] = alpha.unwrap_or(0x48);
    r[1] = 0;
    rec::set_ff(r, 0xc, size);
    r[8] = rot;
    r[2] = frame;
    rec::set_i16(r, 0xa, life as i16);
    rec::set_i16(r, 0x38, life as i16);
    rec::set_ff(r, 0x3c, size);
    rec::set_u32(r, 0x30, c0);
    rec::set_u32(r, 0x34, c1);
    rec::set_i16(r, 0x3a, trail);
    rec::set_v3(r, 0x20, vel);
    rec::set_ff(r, 0x2c, fall * 0.000_277_777_78);
    Some(i)
}

/// `0x346a30`: the life down (out: killed); the colour from `c0` to `c1` by the life left; the move, then the fall;
/// with a trail on an odd frame (`frame`, `$gp+0x40d8`) an ember behind it (`0x33b3d8`: its speed's 0.15..0.25 a
/// random way added, × 0.75..0.95, 15..30 ticks, its size, colour, end colour, frame and ALPHA, no trail), else the
/// velocity × 0.96.
pub fn update(sys: &mut Particles, i: usize, rng: &mut Rng, frame: i32) {
    let r = &mut sys.pool.recs[i];
    if dec_life(r) {
        sys.kill_part(i);
        return;
    }
    let (life, l0) = (rec::i16(r, 0xa) as f32, rec::i16(r, 0x38) as f32);
    rec::set_u32(r, 4, tween_color((life / l0).to_bits(), rec::u32(r, 0x34), rec::u32(r, 0x30)));
    let p = add(rec::v3(r, 0x10), rec::v3(r, 0x20));
    rec::set_v3(r, 0x10, p);
    let mut v = rec::v3(r, 0x20);
    v[2] -= rec::ff(r, 0x2c);
    if rec::ff(r, 0x1c) != 0.0 { rec::set_ff(r, 0x1c, 2.0); }
    if rec::i16(r, 0x3a) == 0 || frame & 1 == 0 {
        rec::set_v3(r, 0x20, scale(v, 0.96));
        return;
    }
    rec::set_v3(r, 0x20, v);
    let (size, c, c1, def, alpha, w) = (rec::ff(r, 0xc), rec::u32(r, 4), rec::u32(r, 0x34), r[2], r[3], rec::ff(r, 0x1c) as i32);
    let d = [rng.randf(-1.0, 1.0), rng.randf(-1.0, 1.0), rng.randf(-1.0, 1.0)];
    let k = len(v) * rng.randf(0.15, 0.25);
    let t = add(set_len(d, k), v);
    let t = scale(t, rng.randf(0.75, 0.95));
    let life = rng.rand_range(15, 30);
    ember(sys, rng, size, p, t, c, c1, life, 0, Some(def), Some(alpha), w);
}
