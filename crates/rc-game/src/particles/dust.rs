//! Particle type 0x42, the dust puff (L02 spawner `0x342ed8`, update `0x343040` at `+0x108` of the type table
//! `0x1ffe80`; L00 `0x33c390`): a faint grey puff that drifts slowly across, grows each tick and fades one alpha
//! step every other tick. Its first consumer is Maktar's footstep puffs (crate::moby::classes::maktar::update_0a57).

use super::{rec, Particles};
use crate::rng::Rng;

/// Type 0x42.
pub const DUST: u8 = 0x42;

/// `0x342ed8(size, growth, pos, rgba)`: a dust puff at `pos` of `size` (× 210000) growing `growth` a tick, coloured
/// `rgba` (None: 0x808080 at alpha `20 + randi(6)`); its frame the first or second of its type's list (`randi(256)`'s
/// low bit), a random roll (`0x1163b0`'s low byte), drifting `randf(0.0015, 0.0025)` a tick in a random direction
/// across; byte 9 0x21, ALPHA 0x48; its alpha steps down first after 4 ticks.
pub fn dust(sys: &mut Particles, rng: &mut Rng, size: f32, growth: f32, pos: [f32; 3], rgba: Option<u32>) -> Option<usize> {
    let i = sys.create_part(DUST)?;
    let rgba = rgba.unwrap_or_else(|| ((rng.randi(6) + 0x14) as u32) << 24 | 0x80_8080);
    let frame = if rng.randi(0x100) & 1 == 0 { sys.def_frame(DUST, 1) } else { sys.def_first(DUST) };
    let roll = rng.rand() as u8;
    let a = rng.rand_angle();
    let speed = rng.randf(f32::from_bits(0x3ac4_9ba6), f32::from_bits(0x3b23_d70a));
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x10, pos);
    rec::set_u32(r, 4, rgba);
    r[9] = 0x21;
    r[3] = 0x48;
    r[1] = 1;
    r[2] = frame;
    r[8] = roll;
    rec::set_i16(r, 0xa, 4);
    rec::set_ff(r, 0xc, size * 210000.0);
    rec::set_ff(r, 0x20, a.cos() * speed);
    rec::set_ff(r, 0x30, growth);
    rec::set_u32(r, 0x28, 0);
    rec::set_ff(r, 0x24, a.sin() * speed);
    Some(i)
}

/// `0x343040`: it moves by its drift, turns a step, grows; its counter `+0xa` runs out every other tick (4 the first
/// time), each time its alpha down by one; killed at alpha 0.
pub fn update(sys: &mut Particles, i: usize) {
    let r = &mut sys.pool.recs[i];
    let p = rec::v3(r, 0x10);
    let v = rec::v3(r, 0x20);
    rec::set_v3(r, 0x10, [p[0] + v[0], p[1] + v[1], p[2] + v[2]]);
    let n = rec::i16(r, 0xa);
    r[8] = r[8].wrapping_add(1);
    rec::set_ff(r, 0xc, rec::ff(r, 0xc) + rec::ff(r, 0x30));
    rec::set_i16(r, 0xa, n.wrapping_sub(1));
    if n == 1 {
        rec::set_i16(r, 0xa, 2);
        rec::set_u32(r, 4, rec::u32(r, 4).wrapping_sub(0x0100_0000));
    }
    if rec::u32(r, 4) & 0xff00_0000 == 0 { sys.kill_part(i); }
}
