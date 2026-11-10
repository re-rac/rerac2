//! Joba's grenade carrier **0x0e86** (`0x462db0`, 3 placed; the decomp's v1.01 `func_L11_00461308`, the same code;
//! L11 addresses) and its grenade **0x0a77** (`0x3de3b8`, made by `0x3de570`; v1.01 `func_L11_003DCEB0` /
//! `func_L11_003DD068`): a flying platform with a gun thug 0x0cf6 on it for show (made by it, never updated: mode 6,
//! crate::moby::classes::level11::gunners), bobbing and rocking, that lobs three grenades at a cuboid while Ratchet is
//! in another.
//!
//! Its pvars: `+0x00` the cuboid Ratchet sets it off in, `+0x04` the cuboid it throws into, `+0x08` shown whatever the
//! event, `+0x0c` a moby (−1 none) that holds it back while in its state 3, `+0x10` its rider, `+0x14` the bob's phase,
//! `+0x18` the bob's height.
//!
//! * **0**: the rider made (`0x33d4d8(rider, 0)` = L00 `0x32a748`, crate::moby::MobyWorld::adopt: update and draw
//!   distance 0xff, drawn; mode 6 more, sequence 0x10); 1.
//! * **1**: in an event (`$gp+0x185c`, the table `0x2b6e40`'s word `+4`) with the flag 0x10 and without `+0x08`,
//!   it and its rider hidden (mode | 0x41), no collision, and nothing else. Else shown, solid; with its moby `+0x0c`
//!   not in state 3 (or none) and Ratchet in its cuboid `+0x00`: 2.
//! * **2**: three grenades from its joints 3, 2 and 4 to points of its cuboid `+0x04` (x at random in −1..1, y
//!   spread over −1..1 from a random start; `0x301700`), their gravity 25..30 /3600; 3.
//! * **3**: Ratchet out of its cuboid `+0x00`: 1.
//! * Shown: the phase 2° a tick on; the height `sin·0.1`, the pitch and roll by `sin·0.0015` / `cos·0.0015`; the
//!   rider at its angles 1.46 along its up, its animation advanced and its matrix built; three exhaust puffs
//!   (crate::particles::exhaust, [`crate::particles::CARRIER_DESC`]..) 0.59 along its up falling 2.25..3.5, 3..5,
//!   4..8 a second.
//!
//! **The grenade** ([`update_0a77`], its pvars `+0x00` the velocity, `+0x0c` the gravity): made at its thrower
//! (`0x33d4d8`), scaled as its class (×1), mode | 0x5000, at the joint; its velocity across to the target over 45
//! ticks and up so as to land there (`0x330578`). **0**: on along its velocity, the gravity off its z; the line there
//! (flags 2) meeting anything: there, 0.8 up, 1. **1**: after 120 ticks it goes off. Hit, or gone off: the general
//! explosion (crate::moby::blast::explode: a 2.5 sphere of 1, flags 0x810001; 10 embers, 3 billows, 16 flares, the
//! flashes 4 / 2 past 9, light 15, one piece, the shake), its sound 0, gone.
//!
//! The event word `$gp+0x185c` has no writer in the port (the level's `0x3fce50` / `0x40bddc` / `0x40bf2c` set it from
//! `$gp+0x1854`), so the first event's flags hold.

use crate::hero::physics::{add, add_rot, dist, scale, set_len, sub, V3};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0e86;
pub const GRENADE: i16 = 0x0a77;
const RIDER_CLASS: i16 = 0x0cf6;

const CUBOID_ON: usize = 0x00;
const CUBOID_AT: usize = 0x04;
const SHOWN: usize = 0x08;
const HOLD: usize = 0x0c;
const RIDER: usize = 0x10;
const PHASE: usize = 0x14;
const BOB: usize = 0x18;
const PVARS: usize = 0x20;

/// `0x2b6e40`: the events' flags (the records' word `+4`, 0x40 apart).
const EVENT_FLAGS: [u32; 5] = [0x1000_0019, 1, 5, 7, 7];

fn rider(w: &MobyWorld, id: usize) -> Option<usize> { crate::moby::ai::link(&w.mobys[id], RIDER).filter(|&r| r < w.mobys.len()) }

/// 0x0e86 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut active = false;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
            let r = w.class(RIDER_CLASS).is_some().then(|| w.create(RIDER_CLASS));
            let m = &mut w.mobys[id];
            crate::moby::ai::set_link(m, RIDER, r);
            m.set_pv_f(PHASE, 0.0);
            m.set_pv_f(BOB, 0.0);
            if let Some(r) = r {
                w.adopt(r, None, &*ctx.host);
                let rm = &mut w.mobys[r];
                rm.mode |= 6;
                rm.drive = true;
                ctx.host.set_anim(&w.mobys[r], 0x10, 0, 0);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let ev = usize::try_from(w.level11.event).ok().and_then(|k| EVENT_FLAGS.get(k)).copied().unwrap_or(0);
            if ev & 0x10 != 0 && w.mobys[id].pv_i(SHOWN) == 0 {
                w.mobys[id].mode |= 0x41;
                if let Some(r) = rider(w, id) { w.mobys[r].mode |= 0x41; }
                w.mobys[id].has_collision = false;
                w.changed = true;
                return;
            }
            active = true;
            let solid = w.class(CLASS).is_some_and(|c| c.has_collision);
            w.mobys[id].mode &= 0xffbe;
            if let Some(r) = rider(w, id) { w.mobys[r].mode &= 0xffbe; }
            w.mobys[id].has_collision = solid;
            let h = w.mobys[id].pv_i(HOLD);
            let ready = h == -1 || usize::try_from(h).ok().and_then(|h| w.mobys.get(h)).is_none_or(|m| m.state != 3);
            if ready && ctx.level.in_cuboid(w.mobys[id].pv_i(CUBOID_ON), ctx.hero.pos) { w.mobys[id].set_state(2, -1); }
        }
        2 => {
            active = true;
            let mut t = ctx.game.rng.randf(0.0, 0.333);
            let at = w.mobys[id].pv_i(CUBOID_AT);
            let cub = usize::try_from(at).ok().and_then(|i| ctx.level.cuboids.get(i)).map(|c| c.matrix);
            for j in [3, 2, 4] {
                let v = ctx.host.joint(&w.mobys[id], j).unwrap_or(w.mobys[id].pos);
                let x = ctx.game.rng.randf(-1.0, 1.0);
                let y = t + t - 1.0;
                t += 0.333;
                let dir = cub.map_or([x, y, 0.0], |m| std::array::from_fn(|k| x * m[0][k] + y * m[1][k] + m[3][k]));
                let g = ctx.game.rng.randf(25.0, 30.0) * 0.000_277_777_78;
                grenade(w, id, ctx, v, dir, g);
            }
            w.mobys[id].set_state(3, -1);
        }
        3 => {
            active = true;
            if !ctx.level.in_cuboid(w.mobys[id].pv_i(CUBOID_ON), ctx.hero.pos) { w.mobys[id].set_state(1, -1); }
        }
        _ => {}
    }
    if !active { return; }
    let m = &mut w.mobys[id];
    let ph = add_rot(m.pv_f(PHASE), 0.034_906_592);
    m.set_pv_f(PHASE, ph);
    let (c, s) = (ph.cos(), ph.sin());
    m.pos[2] -= m.pv_f(BOB);
    m.set_pv_f(BOB, s * 0.1);
    m.pos[2] += s * 0.1;
    m.rot[1] += s * 0.0015;
    m.rot[0] += c * 0.0015;
    let (pos, rot, up) = (m.pos, m.rot, m.rows[2]);
    w.changed = true;
    if let Some(r) = rider(w, id) {
        let rm = &mut w.mobys[r];
        rm.rot = rot;
        rm.pos = add(pos, scale(up, 1.46));
        ctx.host.advance(&w.mobys[r]);
        w.rebuild(r, &*ctx.host);
    }
    let at = add(pos, scale(up, 0.59));
    let rng = &mut ctx.game.rng;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    for (k, (lo, hi)) in [(2.25, 3.5), (3.0, 5.0), (4.0, 8.0)].into_iter().enumerate() {
        let v = [0.0, 0.0, -(rng.randf(lo, hi) * (1.0 / 60.0))];
        crate::particles::exhaust(sys, rng, &w.mobys[id], id, at, v, crate::particles::CARRIER_DESC + k);
    }
}

/// `0x3de570(thrower, at, to, gravity)` (module docs): the grenade 0x0a77.
fn grenade(w: &mut MobyWorld, thrower: usize, ctx: &Ctx, at: V3, to: V3, g: f32) {
    if w.class(GRENADE).is_none() { return; }
    let s = w.create(GRENADE);
    w.adopt(s, Some(thrower), &*ctx.host);
    let k = w.class(GRENADE).map_or(1.0, |c| c.scale);
    let m = &mut w.mobys[s];
    m.scale = k;
    m.mode |= 0x5000;
    m.pos = at;
    let mut v = sub(to, at);
    v[2] = 0.0;
    let speed = crate::hero::physics::len(v) / 45.0;
    let mut v = set_len(v, speed);
    let ticks = if speed == 0.0 { 0.0 } else { dist(to, at) / speed };
    v[2] = if ticks == 0.0 { 0.0 } else { -((at[2] - to[2]) + -g * (ticks * ticks) * 0.5) / ticks };
    m.set_pv_v(0, v);
    m.set_pv_f(0xc, g);
    w.changed = true;
}

/// 0x0a77 (module docs).
pub fn update_0a77(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut off = false;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let next = add(m.pos, m.pv_v(0));
            m.set_pv_f(8, m.pv_f(8) - m.pv_f(0xc));
            let pos = m.pos;
            match w.coll_line(ctx, pos, next, 2, None) {
                Some(o) => {
                    let m = &mut w.mobys[id];
                    m.pos = o.point;
                    m.pos[2] += 0.8;
                    m.set_state(1, -1);
                }
                None => w.mobys[id].pos = next,
            }
            w.changed = true;
        }
        1 => {
            let m = &mut w.mobys[id];
            m.state_ticks = m.state_ticks.wrapping_add(1);
            if 0x79 <= m.state_ticks as u16 { off = true; }
        }
        _ => {}
    }
    if w.hit_message(id).is_some() || off {
        let pos = w.mobys[id].pos;
        let e = crate::moby::blast::Explosion {
            radius: 2.5, damage: 1.0, flags: 0x81_0001, flash: 4.0, flash2: 2.0, near: 9.0, size: 1.0, light: 15.0,
            embers: 10, billows: 3, flares: 16, sound: -1, shake: true, debris: 1, lod: -1, colour: 0,
        };
        crate::moby::blast::explode(w, ctx, Some(id), Some(pos), &e);
        ctx.host.play_sound(&w.mobys[id], 0, 0);
        w.delete(id);
    }
}
