//! The prison's gate **0x113e** (`0x431388`, one placed; the decomp's `func_L14_0042FDA8`): shut over a group of
//! hidden mobys until it is hooked and Ratchet comes near, or until one of the guards it watches wakes or dies; then
//! it opens (or, set to, bursts out sideways until it meets something and blows up).
//!
//! **Its pvars:** `+0x00..+0x0c` the four guards it watches (moby indices, −1 none), `+0x10` it bursts, `+0x14` /
//! `+0x18` two cuboids Ratchet must not be in to open it (`+0x14` also the one that closes it again), `+0x1c` the
//! group it hides.
//!
//! **0** shut: with two or more of its group alive, its group hidden (mode `|= 0x43`). Ratchet within 6 across, it
//! hooked (`0x3336b8` = L00 `0x32a838`) and he in neither cuboid: open (1, colour `0x24c102`, sequence 1, no
//! collision) and its group shown (mode `& 0xffbc`); he in the first cuboid: 5. A guard awake (its sub-state) or
//! dead: bursting (2, sound 2, the general explosion 1 over it, `0x327f70` = L00 `0x320538`,
//! crate::moby::blast::explode), or without `+0x10` open. **1** opening: at its sequence's end 3 (colour `0x224c1`,
//! sequence 2). **2** bursting: 0.3 a tick to its right; a sphere 0.6 at 0.75 over it, or one 0.68 at either side
//! (`0x2e7258` = L00 `0x2df8d8`), meeting anything: sound 2, the explosion there and deleted. **3** open: Ratchet in
//! the first cuboid, 4 (sequence 3). **4** closing: at its sequence's end 5, its collision back, sequence 0.
//!
//! The original's side spheres add offsets it never writes (the 0.8 to either side along its facing it works out
//! goes unused): the port tests them where the front one is.

use std::f32::consts::FRAC_PI_2;

use crate::hero::physics::dist2;
use crate::moby::blast::{self, Explosion};
use crate::moby::classes::props::{group_alive, set_seq, wrapped};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x113e;

const BURSTS: usize = 0x10;
const CUBOID_A: usize = 0x14;
const CUBOID_B: usize = 0x18;
const GROUP: usize = 0x1c;

fn solid_of(w: &MobyWorld, id: usize) -> bool { w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision) }

/// Its group's members, with two or more alive.
fn hidden_group(w: &MobyWorld, id: usize) -> Vec<usize> {
    let g = w.mobys[id].pv_i(GROUP);
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)) else { return Vec::new() };
    if list.is_empty() || group_alive(w, g) < 2 { return Vec::new(); }
    list.clone()
}

fn open(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].state = 1;
    set_seq(w, id, ctx, 1, 0, 0);
    w.mobys[id].has_collision = false;
}

/// The general explosion at `at` (10 embers, 3 billows, 16 flares, the flashes 4 / 2 past 9, light 15, 9 debris).
fn blow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: crate::hero::physics::V3) {
    ctx.host.play_sound(&w.mobys[id], 2, 0);
    let e = Explosion {
        flash: 4.0, flash2: 2.0, near: 9.0, size: 1.0, light: 15.0, embers: 10, billows: 3, flares: 0x10, sound: -1, shake: false,
        debris: 9, lod: -1, flags: 0x81_0001, ..Default::default()
    };
    blast::explode(w, ctx, Some(id), Some(at), &e);
}

/// 0x113e (module docs).
pub fn update_113e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        for k in hidden_group(w, id) { w.mobys[k].mode |= 0x43; }
        let m = &w.mobys[id];
        let hero = ctx.hero.pos;
        let (a, b) = (m.pv_i(CUBOID_A), m.pv_i(CUBOID_B));
        if dist2(m.pos, hero) < 6.0 && m.once & 4 != 0 && !ctx.level.in_cuboid(a, hero) && !ctx.level.in_cuboid(b, hero) {
            open(w, id, ctx);
            w.mobys[id].glow = 0x24_c102;
            for k in hidden_group(w, id) { w.mobys[k].mode &= 0xffbc; }
        } else if ctx.level.in_cuboid(a, hero) {
            w.mobys[id].state = 5;
        }
        for k in 0..4 {
            let g = w.mobys[id].pv_i(4 * k);
            let Some(o) = usize::try_from(g).ok().and_then(|g| w.mobys.get(g)) else { continue };
            if o.sub_state == 0 && o.state < 0x80 { continue; }
            if w.mobys[id].pv_i(BURSTS) != 0 {
                w.mobys[id].state = 2;
                let p = w.mobys[id].pos;
                blow(w, id, ctx, [p[0], p[1], p[2] + 1.0]);
                return;
            }
            open(w, id, ctx);
        }
    }
    if w.mobys[id].state == 1 && wrapped(w, id, ctx) {
        w.mobys[id].state = 3;
        w.mobys[id].glow = 0x2_24c1;
        set_seq(w, id, ctx, 2, 0, 0);
    }
    if w.mobys[id].state == 2 {
        let m = &mut w.mobys[id];
        let (s, c) = (m.rot[2] - FRAC_PI_2).sin_cos();
        m.pos[0] += c * 0.3;
        m.pos[1] += s * 0.3;
        let front = [m.pos[0], m.pos[1], m.pos[2] + 0.75];
        let touch = |r: f32| ctx.coll.and_then(|cl| crate::collision::coll_sphere_m(cl, ctx.scene, front, r, crate::collision::QueryFlags(0), Some(id))).is_some();
        w.changed = true;
        if touch(f32::from_bits(0x3f19_999a)) || touch(f32::from_bits(0x3f2e_147b)) {
            blow(w, id, ctx, front);
            w.delete(id);
            return;
        }
    }
    if w.mobys[id].state == 3 && ctx.level.in_cuboid(w.mobys[id].pv_i(CUBOID_A), ctx.hero.pos) {
        w.mobys[id].state = 4;
        set_seq(w, id, ctx, 3, 0, 0);
    }
    if w.mobys[id].state == 4 && wrapped(w, id, ctx) {
        w.mobys[id].state = 5;
        w.mobys[id].has_collision = solid_of(w, id);
        set_seq(w, id, ctx, 0, 0, 0);
    }
}
