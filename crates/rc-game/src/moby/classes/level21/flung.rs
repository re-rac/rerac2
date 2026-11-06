//! The museum's flung piece, class **0x0c46** ([`update_0c46`], `0x3e2168`, 1 placed; the decomp's
//! `func_L30_003E1BA0`): struck, it is thrown whole away from Ratchet, tumbles on the floor, fades and comes back
//! where it stood.
//!
//! * **0** (set-up): update distance 0xff, mode | 0x100; its place and angles kept (`0x1b3960` / `0x1b3970`, the port's
//!   pvars `+0x80` / `+0x90`); `0x1b39a0` = (0, 0, 0.4), read by nothing here; then 1.
//! * **1**: a hit with the flags 0x10000 (`0x324388`, L00 `0x31dce8`): its way from Ratchet at 10 a second across
//!   (`0x2f5d28`, the xy length) and 10 a second up (`$gp+0x1a68` / `+0x1a6c`); its spin the quaternion of 90 × π/10800
//!   (`+0x1a70`) about (way × up) (`0x2f6000` / `0x2f5fe8`: its axis `sin(−a)`, its w `cos(a)`); ten chunks at (0, 0,
//!   1) in its frame (`0x32be80` = L00 `0x327648`, crate::moby::shards::throw_chunks: reach 0.5, 6 a second, 0.75..1.25
//!   of their size, pointed, (0, 0, 0.1) a tick, colour 0x50404040); then it is thrown in place (`0x2e2db0` = L00
//!   `0x2dbff8` mode 0, crate::moby::shards::whole_in_place: gravity 15, no sound), its collision off, 2.
//! * **2**: the throw's tick (`0x2e2fa8` = L00 `0x2dc1f0`); its life over: 3.
//! * **3**: its alpha down 2 a tick; under 2: back at its place with the kept angles' rows (`0x2f6258`), alpha 0x80,
//!   its class's collision again, 1.
//! * Every tick its hit record is let go.

use crate::hero::physics::{add, cross, set_len, set_len2, sub, V3};
use crate::moby::{shards, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c46;

const HOME: usize = 0x80;
const HOME_ROT: usize = 0x90;

/// π/10800 (`0x3998825c`) and 1/60 (`0x3c888889`).
const UNIT: f32 = 0.000_290_888_25;
const TICK: f32 = 0.016_666_668;

pub fn update_0c46(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.set_state(1, -1);
            m.mode |= 0x100;
            let (p, r) = (m.pos, m.rot);
            m.set_pv_v(HOME, p);
            m.set_pv_v(HOME_ROT, r);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x1_0000 != 0) {
                let m = &w.mobys[id];
                let mut v = set_len2(sub(m.pos, ctx.hero.pos), 10.0 * TICK);
                v[2] = 10.0 * TICK;
                let a = 90.0 * UNIT;
                let axis = set_len(cross(v, [0.0, 0.0, 1.0]), (-a).sin());
                let spin = [axis[0], axis[1], axis[2], a.cos()];
                let rows = m.rows;
                let off: V3 = [0.0, 0.0, 1.0];
                let at = add(std::array::from_fn(|k| off[0] * rows[0][k] + off[1] * rows[1][k] + off[2] * rows[2][k]), m.pos);
                shards::throw_chunks(w, ctx, 0.5, 6.0, (0.75, 1.25), id, at, 10, true, [0.0, 0.0, f32::from_bits(0x3dcc_cccd)], 0x5040_4040, None);
                shards::whole_in_place(w, id, 15.0, v, spin, 0xff);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => if shards::step_in_place(w, id, ctx) { w.mobys[id].set_state(3, -1) },
        3 => {
            if w.mobys[id].alpha < 2 {
                let has = w.class(CLASS).is_some_and(|c| c.has_collision);
                let m = &mut w.mobys[id];
                m.pos = m.pv_v(HOME);
                m.rows = crate::moby::attach::rows_of(m.pv_v(HOME_ROT));
                m.alpha = 0x80;
                m.has_collision = has;
                m.set_state(1, -1);
            } else {
                w.mobys[id].alpha -= 2;
            }
        }
        _ => {}
    }
    w.clear_hit(id);
}
