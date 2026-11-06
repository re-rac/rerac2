//! Particle type 0x56, the keyframed puff (L02 spawner `0x345d88`, L18 `0x35a848`, L11 `0x353250`, L19 `0x33cd90`,
//! L20 `0x354780`, L22 `0x342a18`, L23 `0x347130`; the decomp's `func_L02_003449D8`, not in L00; its update `0x33f240`
//! in L00, L02 `0x345f40`, L18 `0x35aa00`): a puff moving at a set velocity scaled each tick, turning a set step, its
//! colour and size eased along a template's keyframes ([`Keys`]). The callers' notes call its spawner "the particle
//! burst": each calls it in a small loop of its own. First consumers: Damosel's traffic (the warp's puffs and
//! 0x0d9b's exhaust, crate::moby::classes::level18::traffic).
//!
//! **The record:** `+0x10` its place, `+0x20` its velocity (a tick), `+0x30` its template (here an index into
//! [`Particles::keys`](super::Particles)), `+0x34` its keyframe, `+0x38` its turn (a byte a tick).
//!
//! Not ported: the place's w (`+0x1c`), which the spawner copies from its point's (`0x309640` adds xyz only); the
//! kind-0 draw and this update never read it.

use super::{dec_life, rec, Particles};
use crate::hero::physics::{add, scale, V3};
use crate::rng::Rng;

/// Type 0x56.
pub const KEYED: u8 = 0x56;

/// A template (`{colors, sizes, times, last_index, lifetime, scale}`, 0x18 bytes in the level's data): keyframe `k`
/// at tick `times[k]` of the life has colour `colors[k]` and size `sizes[k]` (× 1000); `last` the last keyframe it
/// steps to, `life` its ticks, `scale` the velocity's factor a tick.
#[derive(Debug)]
pub struct Keys {
    pub colors: &'static [u32],
    pub sizes: &'static [i16],
    pub times: &'static [i16],
    pub last: i16,
    pub life: i16,
    pub scale: f32,
}

impl Keys {
    // The game reads past the arrays' ends as the data after them; the templates in use never step that far.
    fn color(&self, k: i16) -> u32 { usize::try_from(k).ok().and_then(|k| self.colors.get(k)).copied().unwrap_or(0) }
    fn size(&self, k: i16) -> i32 { usize::try_from(k).ok().and_then(|k| self.sizes.get(k)).map_or(0, |&s| s as i32) }
    fn time(&self, k: i16) -> i32 { usize::try_from(k).ok().and_then(|k| self.times.get(k)).map_or(0, |&s| s as i32) }
}

/// `0x345d88(pos, vel, keys, additive, spin)`: a keyframed puff (type 0x56) as if spawned a random share `f` (0..1) of a
/// tick along `vel`: at `pos + vel·f`, moving `vel·(f·scale)` a tick, its colour and size `f` / (keyframe 1's tick −
/// 0's) of the way from keyframe 0 to 1, its life `keys.life` (one less past half a tick), turning `spin` a tick
/// from a random angle; ALPHA 0x48 with `additive`, else 0x44; byte 9 `cvt.w.s(0.04) − 0x60` = 0xa0; its type's first
/// frame.
pub fn keyed(sys: &mut Particles, rng: &mut Rng, pos: V3, vel: V3, keys: &'static Keys, additive: bool, spin: i32) -> Option<usize> {
    let i = sys.create_part(KEYED)?;
    let def = sys.def_first(KEYED);
    let slot = match sys.keys.iter().position(|k| std::ptr::eq(*k, keys)) {
        Some(s) => s,
        None => {
            sys.keys.push(keys);
            sys.keys.len() - 1
        }
    };
    let roll = rng.rand() as u8;
    let f = rng.randf(0.0, 1.0);
    let k = f / (keys.time(1) - keys.time(0)) as f32;
    let (s0, s1) = (keys.size(0), keys.size(1));
    let r = &mut sys.pool.recs[i];
    r[9] = 0xa0;
    r[3] = if additive { 0x48 } else { 0x44 };
    r[1] = 0;
    r[2] = def;
    r[8] = roll;
    rec::set_u32(r, 4, crate::hud::lerp_rgba(k, keys.color(0), keys.color(1)));
    rec::set_ff(r, 0xc, (s0 as f32 + k * (s1 - s0) as f32) * 1000.0);
    rec::set_v3(r, 0x10, add(pos, scale(vel, f)));
    rec::set_i16(r, 0xa, keys.life - (0.5 < f) as i16);
    rec::set_v3(r, 0x20, scale(vel, f * keys.scale));
    rec::set_u32(r, 0x38, spin as u32);
    rec::set_u32(r, 0x30, slot as u32);
    rec::set_i16(r, 0x34, 0);
    Some(i)
}

/// `0x33f240`: the velocity × ((scale − 1) + 1) and the move; the turn; on to the next keyframe once its tick is
/// passed (to `last` at most); the colour and size eased from the keyframe to the next by the ticks spent; the life
/// down (out: killed).
pub fn update(sys: &mut Particles, i: usize) {
    let slot = rec::u32(&sys.pool.recs[i], 0x30) as usize;
    let Some(&keys) = sys.keys.get(slot) else {
        sys.kill_part(i);
        return;
    };
    let r = &mut sys.pool.recs[i];
    let v = scale(rec::v3(r, 0x20), (keys.scale - 1.0) + 1.0);
    rec::set_v3(r, 0x20, v);
    rec::set_v3(r, 0x10, add(rec::v3(r, 0x10), v));
    r[8] = r[8].wrapping_add(r[0x38]);
    let spent = (keys.life as u16).wrapping_sub(rec::i16(r, 0xa) as u16) as i16 as i32;
    let mut k = rec::i16(r, 0x34);
    if keys.time(k + 1) < spent && k < keys.last {
        k += 1;
        rec::set_i16(r, 0x34, k);
    }
    let span = (keys.time(k + 1) - keys.time(k)) as f32;
    let t = (spent - keys.time(k)) as f32 / span;
    rec::set_u32(r, 4, crate::hud::lerp_rgba(t, keys.color(k), keys.color(k + 1)));
    let (s0, s1) = (keys.size(k), keys.size(k + 1));
    rec::set_ff(r, 0xc, (s0 as f32 + t * (s1 - s0) as f32) * 1000.0);
    if dec_life(r) { sys.kill_part(i); }
}
