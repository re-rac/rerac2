//! Dobbo's keyed point light **0x0292** ([`update_0292`], L09 `0x39cd78`; the decomp's `func_L09_0039BA18`): an
//! unseen moby that owns one of the point lights (crate::point_lights, `0x2f9dd8` = L00 `0x2fdb60` takes the slot,
//! `0x2f9ec0` = L00 `0x2fdc48` frees it) and runs it through a list of keys.
//!
//! **Its pvars**: `+0x00` the offset (`+0x0c` the flags' word while it runs, as a float: the sphere's least scale),
//! `+0x10` its first place, `+0x20` the moby it rides (−1 none; kept at `+0x34`), `+0x24` the flags, `+0x28` its life
//! (u16, 0xffff forever), `+0x2a` the intensity (hundredths), `+0x2c` the last key, `+0x2d` the key now, `+0x2e` the
//! slot (−1 none), `+0x2f` the wait before it starts, `+0x30` the ticks into the key, `+0x38..` the keys: five bytes,
//! the colour (r, g, b, hundredths), the radius and the key's length in ticks.
//!
//! **The flags**: 0x4000 the keys loop; 0x100000 the offset random — 0x200000 within the box `±offset`, else the
//! offset scaled by `randf(least, 1)`; 0x400000 the offset turned by the ridden moby's rows; 0x800000 no offset (at
//! the origin); 0x1000000 dark while the ridden moby is hidden.
//!
//! **Each tick**: its life out (counted, not 0xffff): the light freed and the moby gone. **0**: updated at any
//! distance, its first place kept, the wait `+0x2f`; 1. **1**: the wait out, 2. **2**: the colour and the radius
//! the key's eased linearly into the next's by the ticks into the key (past the last key the first's with the loop,
//! else the key before); at the origin (the ridden moby's place, else its first place) plus the offset; in view within
//! 255 of the radius (`0x2f0080`) the slot taken or rewritten, else freed (and while the ridden moby is hidden with
//! 0x1000000). The key's length out: the next key, and past the last the first again (the loop) or 3. **3**: the
//! light freed and the moby gone.

use crate::hero::physics::{add, V3};
use crate::moby::{countdown, Ctx, MobyWorld};
use crate::point_lights::PointLight;

pub const CLASS: i16 = 0x0292;

const LIFE: usize = 0x28;
const INTENSITY: usize = 0x2a;
const LAST_KEY: usize = 0x2c;
const KEY: usize = 0x2d;
const SLOT: usize = 0x2e;
const WAIT: usize = 0x2f;
const TICKS: usize = 0x30;
const KEYS: usize = 0x38;

const LOOP: i32 = 0x4000;
const RANDOM: i32 = 0x10_0000;
const BOX: i32 = 0x20_0000;
const TURNED: i32 = 0x40_0000;
const AT_ORIGIN: i32 = 0x80_0000;
const DARK_HIDDEN: i32 = 0x100_0000;

fn free(w: &mut MobyWorld, id: usize) {
    let s = w.mobys[id].pv_b(SLOT) as i8;
    if s != -1 {
        w.point_lights.free(s as u8 as usize);
        w.mobys[id].set_pv_b(SLOT, 0xff);
    }
}

/// 0x0292 (`0x39cd78`): see the module.
pub fn update_0292(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < KEYS + 5 { return; }
    let life = w.mobys[id].pv_s(LIFE) as u16;
    if life != 0xffff {
        let mut t = life as i32;
        let out = countdown(&mut t);
        w.mobys[id].set_pv_s(LIFE, t as i16);
        if out {
            free(w, id);
            return w.delete(id);
        }
    }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            let pos = m.pos;
            m.set_pv_v(0x10, pos);
            let flags = m.pv_i(0x24);
            m.set_pv_i(0xc, flags);
            m.set_pv_b(SLOT, 0xff);
            m.set_pv_b(KEY, 0);
            let wait = m.pv_b(WAIT) as i32;
            m.set_pv_i(TICKS, wait);
            m.set_state(1, -1);
        }
        1 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(TICKS);
            let out = countdown(&mut t);
            m.set_pv_i(TICKS, t);
            if out { m.set_state(2, -1); }
        }
        2 => run(w, id, ctx),
        3 => {
            if w.mobys[id].pv_b(SLOT) as i8 != -1 {
                free(w, id);
                w.delete(id);
            }
        }
        _ => {}
    }
}

/// State 2 (module docs).
fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let flags = m.pv_i(0x24);
    let k = KEYS + m.pv_b(KEY) as usize * 5;
    let key = |o: usize| m.pv_b(o) as f32;
    let (r, g, b, radius, length) = (key(k) * 0.01, key(k + 1) * 0.01, key(k + 2) * 0.01, key(k + 3), m.pv_b(k + 4) as i32);
    let t = m.pv_i(TICKS) as f32 / length as f32;
    let next = if (m.pv_b(LAST_KEY) as usize) < m.pv_b(KEY) as usize + 1 { if flags & LOOP != 0 { KEYS } else { k - 5 } } else { k + 5 };
    let (nr, ng, nb, nradius) = (key(next) * 0.01, key(next + 1) * 0.01, key(next + 2) * 0.01, key(next + 3));
    let rgb = [r + (nr - r) * t, g + (ng - g) * t, b + (nb - b) * t];
    let radius = radius + (nradius - radius) * t;
    let rider = usize::try_from(m.pv_i(0x20)).ok().and_then(|i| w.mobys.get(i));
    let p = m.pos;
    let seen = !ctx.view.is_some_and(|v| v.culled(255.0, [p[0], p[1], p[2], radius]));
    let dark = flags & DARK_HIDDEN != 0 && rider.is_some_and(|r| r.mode & 1 != 0);
    if seen && !dark {
        let origin = rider.map_or(m.pv_v(0x10), |r| r.pos);
        let pos = if flags & AT_ORIGIN != 0 {
            origin
        } else {
            let off = m.pv_v(0);
            let mut d: V3 = if flags & RANDOM != 0 {
                let g = &mut ctx.game.rng;
                if flags & BOX != 0 {
                    [g.randf(-off[0], off[0]), g.randf(-off[1], off[1]), g.randf(-off[2], off[2])]
                } else {
                    let s = g.randf(m.pv_f(0xc), 1.0);
                    [off[0] * s, off[1] * s, off[2] * s]
                }
            } else {
                off
            };
            if flags & TURNED != 0 {
                if let Some(r) = rider { d = std::array::from_fn(|j| d[0] * r.rows[0][j] + d[1] * r.rows[1][j] + d[2] * r.rows[2][j]); }
            }
            add(origin, d)
        };
        let light = PointLight { color: rgb, intensity: m.pv_b(INTENSITY) as f32 * 0.01, pos, radius };
        let slot = m.pv_b(SLOT) as i8;
        w.mobys[id].pos = pos;
        if slot == -1 {
            let s = w.point_lights.alloc(light).map_or(0xff, |s| s as u8);
            w.mobys[id].set_pv_b(SLOT, s);
        } else {
            w.point_lights.set(slot as u8 as usize, light);
        }
    } else {
        free(w, id);
    }
    let m = &mut w.mobys[id];
    let ticks = m.pv_i(TICKS) + 1;
    m.set_pv_i(TICKS, ticks);
    if length <= ticks {
        m.set_pv_i(TICKS, 0);
        let key = m.pv_b(KEY).wrapping_add(1);
        m.set_pv_b(KEY, key);
        if m.pv_b(LAST_KEY) < key {
            if flags & LOOP != 0 { m.set_pv_b(KEY, 0); } else { m.set_state(3, -1); }
        }
    }
}
