//! Type 0x28: vapour (`0x338478` / `0x338648`, L17 `0x33b730` / `0x33b900`; the decomp's `func_002C02F0`): a puff
//! (record kind 0, ALPHA 0x44) flying with its velocity, growing, splitting once, rising and slowing; on the world it
//! bounces off, and from then shrinks away. Level 17's vents 0x02be put them out
//! (crate::moby::classes::level17::vent).
//!
//! Its record: `+0x0a` the split's countdown, `+0x0c` the size, `+0x10` / `+0x20` place and velocity, `+0x30` its
//! turn (a fraction of a turn; `+0x08` × 256), `+0x34` its age (never stepped: 0), `+0x36` its bounces, `+0x38` its
//! flags (8 bounced, 0x10 no collision, 0x40 no push in its hit, 1 turning the other way, 0x80 split off), `+0x39`
//! its scale, `+0x3c` its owner.
//!
//! **A puff** ([`vapour`], `0x338478(pos, vel, flags, owner, scale, shade)`): colour (0.7, 0.7, 0.7, 0.08)
//! (`0x31f6c8`), `+0x09` `(shade >> 5) << 4` + 4, its turn a random angle (`0x31bb00`, as a fraction), its frame one
//! of the def list's first eight (`rand & 7`); with flags below 5 its size `randf(656.25, 937.5)·scale` and the split's
//! countdown `18 − rand mod 15`, else `937.5·scale` and 18.
//!
//! **Each tick** ([`update`], `0x338648`):
//! * the split's countdown out (`0x2eeae0`): flags | 0x88; `k = size / (scale·28125)`; size × 0.8; a puff split off
//!   at its place ± k (each axis) with its velocity × `randf(0.5, 0.7)`, x and y each ± 0.2 of themselves and z less
//!   up to 0.2 of itself, flags `(flags & 0x10) | 0x88`, the split's countdown 2 and its size; then its own velocity
//!   and place the same way.
//! * its frame the def list's `(countdown + bounces + frame − first) & 7`-th;
//! * not bounced yet: the line from its place (back by its velocity from age 4) to its place plus its velocity,
//!   unless flag 0x10 (`0x2de878` flags 0, its owner skipped, with the hit of 1, flags 0x10001, along its velocity,
//!   0 with flag 0x40); bounced: the same line with flags 4 and no hit while its alpha is above 0, else gone.
//! * the line hits: its velocity mirrored in the face (`0x2ef2a8`) × 0.02, its place the hit point 0.055 out along the
//!   face, its size doubled below 200000 the first time, flags | 8; else its place on by its velocity.
//! * not bounced: its size up by `(8437.5 − 937.5)·scale / 18`; bounced: its bounces + 1 and its size down by
//!   `(8437.5 − 937.5)·scale·0.0694445` (out: gone).
//! * its alpha `256·a` with `a = 0.08 + age·0.000246913` held to 0..0.08 (bounced: / (bounces / 7 + 1)); its z
//!   speed + 0.008; from age 26 or bounced its x and y speeds × 0.95; its turn on by −0.01 (+0.01 with flag 1; bounced:
//!   / bounces, and its z speed + 0.008 more), wrapped into 0..1 once a tick.
//!
//! Not ported: the hit the line gives the moby it meets (the particle pass has no moby scene: only the level's
//! collision stops the puffs here).

use super::{rec, Particles};
use crate::rng::Rng;

/// Type 0x28.
pub const VAPOUR: u8 = 0x28;

/// `0x31f6c8(r, g, b, a)`: a colour word from four 0..1 floats (each × 255, truncated).
fn pack(r: f32, g: f32, b: f32, a: f32) -> u32 {
    let k = |x: f32| (x * 255.0) as i32 as u32;
    k(r) & 0xff | (k(g) & 0xff) << 8 | (k(b) & 0xff) << 16 | k(a) << 24
}

/// `0x338478` (module docs).
pub fn vapour(sys: &mut Particles, rng: &mut Rng, pos: [f32; 3], vel: [f32; 3], flags: u8, owner: i32, scale: u8, shade: i32) -> Option<usize> {
    let i = sys.create_part(VAPOUR)?;
    let turn = rng.rand_angle();
    let frame = sys.def_frame(VAPOUR, (rng.rand() & 7) as usize);
    let (size, life) = if flags < 5 {
        (rng.randf(scale as f32 * 656.25, scale as f32 * 937.5), (0x12 - rng.rand() as i32 % 15) as i16)
    } else {
        (scale as f32 * 937.5, 0x12)
    };
    let r = &mut sys.pool.recs[i];
    rec::set_v3(r, 0x10, pos);
    rec::set_v3(r, 0x20, vel);
    rec::set_u32(r, 4, pack(0.7, 0.7, 0.7, 0.08));
    r[1] = 0;
    r[3] = 0x44;
    r[9] = (((shade >> 5) << 4) + 4) as u8;
    rec::set_ff(r, 0x30, turn);
    r[8] = (turn * 256.0) as i32 as u8;
    r[2] = frame;
    r[0x39] = scale;
    r[0x38] = flags;
    rec::set_i16(r, 0x34, 0);
    rec::set_i16(r, 0x36, 0);
    rec::set_ff(r, 0xc, size);
    rec::set_i16(r, 0xa, life);
    rec::set_u32(r, 0x3c, owner as u32);
    Some(i)
}

fn add(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
fn sub(a: [f32; 3], b: [f32; 3]) -> [f32; 3] { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn scale(a: [f32; 3], k: f32) -> [f32; 3] { [a[0] * k, a[1] * k, a[2] * k] }

/// The split's jitter of a velocity: × `randf(0.5, 0.7)`, x and y ± 0.2 of themselves, z less up to 0.2 of itself.
fn jitter_vel(rng: &mut Rng, v: [f32; 3]) -> [f32; 3] {
    let mut v = scale(v, rng.randf(0.5, 0.7));
    v[0] += rng.randf(-v[0] * 0.2, v[0] * 0.2);
    v[1] += rng.randf(-v[1] * 0.2, v[1] * 0.2);
    v[2] += rng.randf(-v[2] * 0.2, 0.0);
    v
}

/// `0x338648` (module docs). `coll`: the level's collision.
pub fn update(sys: &mut Particles, i: usize, rng: &mut Rng, coll: Option<&rc_formats::collision::Collision>) {
    let r = &sys.pool.recs[i];
    let mut life = rec::i16(r, 0xa) as i32;
    if life != 0 && crate::moby::countdown(&mut life) {
        let r = &mut sys.pool.recs[i];
        rec::set_i16(r, 0xa, life as i16);
        r[0x38] |= 0x88;
        let sc = r[0x39];
        let k = rec::ff(r, 0xc) / (sc as f32 * 28125.0);
        rec::set_ff(r, 0xc, rec::ff(r, 0xc) * 0.8);
        let (pos, vel, flags, owner, size) = (rec::v3(r, 0x10), rec::v3(r, 0x20), r[0x38], rec::u32(r, 0x3c) as i32, rec::ff(r, 0xc));
        let m = rng.randf(0.5, 0.7);
        let p = [pos[0] + rng.randf(-k, k), pos[1] + rng.randf(-k, k), pos[2] + rng.randf(-k, k)];
        let mut v = scale(vel, m);
        v[0] += rng.randf(-vel[0] * 0.2, vel[0] * 0.2);
        v[1] += rng.randf(-vel[1] * 0.2, vel[1] * 0.2);
        v[2] += rng.randf(-vel[2] * 0.2, 0.0);
        if let Some(c) = vapour(sys, rng, p, v, flags & 0x10 | 0x88, owner, sc, 0x80) {
            let rc = &mut sys.pool.recs[c];
            rec::set_i16(rc, 0xa, 2);
            rec::set_ff(rc, 0xc, size);
        }
        let v = jitter_vel(rng, vel);
        let p = [pos[0] + rng.randf(-k, k), pos[1] + rng.randf(-k, k), pos[2] + rng.randf(-k, k)];
        let r = &mut sys.pool.recs[i];
        rec::set_v3(r, 0x20, v);
        rec::set_v3(r, 0x10, p);
    } else {
        rec::set_i16(&mut sys.pool.recs[i], 0xa, life as i16);
    }
    let first = sys.def_frame(VAPOUR, 0);
    let r = &sys.pool.recs[i];
    let n = (rec::i16(r, 0xa) as i32 + rec::i16(r, 0x36) as i32 + (r[2] as i32 - first as i32)) & 7;
    let frame = sys.def_frame(VAPOUR, n as usize);
    let r = &mut sys.pool.recs[i];
    r[2] = frame;
    let (pos, vel) = (rec::v3(r, 0x10), rec::v3(r, 0x20));
    let next = add(pos, vel);
    let prev = if rec::i16(r, 0x34) < 4 { pos } else { sub(pos, vel) };
    let flags = r[0x38];
    let line = |f: u32| coll.and_then(|c| crate::collision::coll_line(c, prev, next, crate::collision::QueryFlags(f)));
    let hit = if flags & 8 == 0 {
        if flags & 0x10 == 0 { line(0) } else { None }
    } else {
        if rec::u32(r, 4) & 0xff00_0000 == 0 {
            sys.kill_part(i);
            return;
        }
        if flags & 0x10 == 0 { line(4) } else { None }
    };
    match hit {
        Some(o) => {
            let n = o.normal;
            let nn = n[0] * n[0] + n[1] * n[1] + n[2] * n[2];
            let mut v = vel;
            if 0.0 < nn {
                let d = (v[0] * n[0] + v[1] * n[1] + v[2] * n[2]) / nn;
                v = sub(v, scale(n, d + d));
            }
            rec::set_v3(r, 0x20, scale(v, 0.02));
            let l = nn.sqrt();
            let out = if 0.0 < l { scale(n, 0.055 / l) } else { [0.0; 3] };
            rec::set_v3(r, 0x10, add(o.point, out));
            let size = rec::ff(r, 0xc);
            if size < 200_000.0 && r[0x38] & 8 == 0 { rec::set_ff(r, 0xc, size + size); }
            r[0x38] |= 8;
        }
        None => rec::set_v3(r, 0x10, next),
    }
    let sc = r[0x39] as f32;
    let grow = sc * 8437.5 - sc * 937.5;
    let bounced = r[0x38] & 8 != 0;
    if !bounced {
        rec::set_ff(r, 0xc, rec::ff(r, 0xc) + grow * 0.055_555_556);
    } else {
        rec::set_i16(r, 0x36, rec::i16(r, 0x36).wrapping_add(1));
        let s = rec::ff(r, 0xc) - grow * 0.069_444_45;
        rec::set_ff(r, 0xc, s);
        if s <= 0.0 {
            sys.kill_part(i);
            return;
        }
    }
    let r = &mut sys.pool.recs[i];
    let mut a = (rec::i16(r, 0x34) as f32 * 0.000_246_913_58 + 0.08).clamp(0.0, 0.08);
    if bounced { a /= rec::i16(r, 0x36) as f32 * 0.142_857_15 + 1.0; }
    rec::set_u32(r, 4, rec::u32(r, 4) & 0xff_ffff | ((a * 256.0) as i32 as u32) << 24);
    let mut v = rec::v3(r, 0x20);
    v[2] += 0.008;
    if 0x1a <= rec::i16(r, 0x34) || bounced {
        v[0] *= 0.95;
        v[1] *= 0.95;
    }
    let sign = if r[0x38] & 1 != 0 { 1.0 } else { -1.0 };
    let mut turn = rec::ff(r, 0x30);
    if !bounced {
        turn += sign * -0.01;
    } else {
        v[2] += 0.008;
        turn += sign * -0.01 / rec::i16(r, 0x36) as f32;
    }
    rec::set_v3(r, 0x20, v);
    if 1.0 < turn { turn -= 1.0; }
    if turn < 0.0 { turn += 1.0; }
    rec::set_ff(r, 0x30, turn);
    r[8] = (turn * 256.0) as i32 as u8;
}
