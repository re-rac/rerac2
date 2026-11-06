//! 0x1305 (`0x4220e0`, 4 placed; the decomp's `func_L17_00420A50`, its wall `func_L17_00420758` at our `0x421de8`):
//! a barrier held up by up to five mobys. While any of them stands, its wall hurts what touches it; once they are all
//! gone (or changed), it lets its target moby go.
//!
//! **Its words:** `+0x00` / `+0x04` the wall's width and height, `+0x08` its flags, `+0x0c` the fade's time (ticks),
//! `+0x10` the target (a moby index, −1 none) and `+0x14` its class, `+0x16` the state the target is given when the
//! wall drops and `+0x17` the target's state that ends the fade (−1 none), `+0x18` five links (8 bytes each: `+0` a
//! moby index, −1 none; `+4` its class, taken at the set-up; `+6` the state it counts in, −1 any), `+0x40` the fade's
//! timer, `+0x44` the target's alpha at the start, `+0x48` / `+0x4c` the wall's reach across and up, `+0x50` its
//! damage.
//!
//! **The flags:** 1 the target kept at the end, 2 its alpha not faded, 4 / 8 the fade not ended by the target's state
//! / by the timer, 0x10 / 0x40 it takes the target's place / gives the target its own, 0x20 / 0x80 the same for the
//! angles, 0x100 no wall.
//!
//! **Each tick** its links count: a link's moby of its class, alive, in its state (or any) — none counted once its
//! mission (`+0xb0`) has its bit 0x80 (`0x19b2e8 + level·0x10`).
//! * **0**: the target's class and alpha, the places and angles by the flags; the links' classes; its scale the wall's
//!   larger side times its class's scale; 1.
//! * **1**: the wall ([`wall`]); none counted: the target into its state `+0x16`, the fade's timer `+0x0c`, 2.
//! * **2**: the wall; the target's alpha fades with the timer; the target in its state `+0x17`, or the timer over: 3.
//! * **3**: the target goes (unless flag 1), and so does it.
//!
//! Not ported: the debug prints (`0x2a5e70`).

use crate::hero::physics::{add, dot, euler_rows, sub, V3};
use crate::moby::{countdown, hits::HitTemplate, Ctx, MobyWorld};

pub const CLASS: i16 = 0x1305;

const LINKS: usize = 0x18;

fn target(w: &MobyWorld, id: usize) -> Option<usize> { usize::try_from(w.mobys[id].pv_i(0x10)).ok().filter(|&k| k < w.mobys.len()) }

/// The links still standing (module docs).
fn standing(w: &MobyWorld, id: usize) -> usize {
    let m = &w.mobys[id];
    (0..5).filter(|&j| {
        let l = LINKS + j * 8;
        let Some(o) = usize::try_from(m.pv_i(l)).ok().and_then(|k| w.mobys.get(k)) else { return false };
        if o.o_class != m.pv_s(l + 4) || o.state == 0xfe || o.state == 0xfd { return false; }
        let s = m.pv_b(l + 6) as i8;
        s == -1 || o.state != s as u8
    }).count()
}

/// `0x421de8`: the wall — a sphere of half its larger side about its middle (`(0, 0, r)` through its angles) finds
/// the mobys near (flags 0x10); each alive one whose nearest point of the wall's plane (its local x and z kept, held
/// to the half sides once beyond them by more than the reaches) has it within the reach across of it takes a hit of
/// its damage (flags 0x10001, from it, at that point).
fn wall(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    if m.pv_i(8) & 0x100 != 0 { return; }
    let Some(sc) = ctx.scene else { return };
    let rows = euler_rows(m.rot);
    let through = |v: V3| -> V3 { std::array::from_fn(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]) };
    let (wd, ht, rx, rz, damage) = (m.pv_f(0), m.pv_f(4), m.pv_f(0x48), m.pv_f(0x4c), m.pv_f(0x50));
    let r = wd.max(ht) * 0.5;
    let pos = m.pos;
    let centre = add(through([0.0, 0.0, r]), pos);
    let near = crate::collision::coll_sphere_mobys(sc, centre, r, crate::collision::QueryFlags(0x10), Some(id));
    for c in near {
        let Some(o) = w.mobys.get(c) else { continue };
        if o.state == 0xfe || o.state == 0xfd { continue; }
        let d = sub(o.pos, pos);
        let mut l = [dot(d, rows[0]), 0.0, dot(d, rows[2])];
        if l[0] < -(wd * 0.5 + rx) { l[0] = -wd * 0.5; } else if wd * 0.5 + rx < l[0] { l[0] = wd * 0.5; }
        if l[2] < -(ht * 0.5 + rz) { l[2] = -ht * 0.5; } else if ht * 0.5 + rz < l[2] { l[2] = ht * 0.5; }
        let p = add(through(l), pos);
        let touch = crate::collision::coll_sphere_mobys(sc, p, rx, crate::collision::QueryFlags(0x10), Some(id));
        if touch.contains(&c) {
            let t = HitTemplate::new(damage, Some(id), 0x10001, [0.0; 4]);
            w.deliver_hit_any(c, &t, [p[0], p[1], p[2], 1.0]);
        }
    }
}

/// 0x1305 (module docs).
pub fn update_1305(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let t = target(w, id);
    let m = &w.mobys[id];
    let done = m.mission != 0xff
        && ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + m.mission as usize).is_some_and(|b| b & 0x80 != 0);
    let count = if done || m.state == 0 { 0 } else { standing(w, id) };
    let flags = w.mobys[id].pv_i(8);
    match w.mobys[id].state {
        0 => {
            if let Some(t) = t {
                let (class, alpha) = (w.mobys[t].o_class, w.mobys[t].alpha);
                w.mobys[id].set_pv_s(0x14, class);
                w.mobys[id].set_pv_f(0x44, alpha as f32);
                if flags & 0x10 != 0 { w.mobys[id].pos = w.mobys[t].pos; } else if flags & 0x40 != 0 { w.mobys[t].pos = w.mobys[id].pos; }
                if flags & 0x20 != 0 { w.mobys[id].rot = w.mobys[t].rot; } else if flags & 0x80 != 0 { w.mobys[t].rot = w.mobys[id].rot; }
            }
            for j in 0..5 {
                let l = LINKS + j * 8;
                if let Some(c) = usize::try_from(w.mobys[id].pv_i(l)).ok().and_then(|k| w.mobys.get(k)).map(|o| o.o_class) {
                    w.mobys[id].set_pv_s(l + 4, c);
                }
            }
            let s = w.class(CLASS).map_or(1.0, |c| c.scale);
            let m = &mut w.mobys[id];
            m.scale = m.pv_f(0).max(m.pv_f(4)) * s;
            m.state = 1;
        }
        1 => {
            if count == 0 {
                let s = w.mobys[id].pv_b(0x16) as i8;
                if let Some(t) = t.filter(|_| s != -1) { w.mobys[t].state = s as u8; }
                let m = &mut w.mobys[id];
                m.set_pv_i(0x40, m.pv_i(0xc));
                m.state = 2;
            }
            wall(w, id, ctx);
        }
        2 => {
            if let Some(t) = t.filter(|_| flags & 2 == 0) {
                let m = &w.mobys[id];
                let a = (m.pv_f(0x44) * m.pv_i(0x40) as f32 / m.pv_i(0xc) as f32) as i32;
                w.mobys[t].alpha = a as u8;
            }
            if flags & 4 == 0 {
                let s = w.mobys[id].pv_b(0x17) as i8;
                if t.is_some_and(|t| s != -1 && w.mobys[t].state == s as u8) { w.mobys[id].state = 3; }
            }
            if flags & 8 == 0 {
                let mut n = w.mobys[id].pv_i(0x40);
                let over = countdown(&mut n);
                w.mobys[id].set_pv_i(0x40, n);
                if over { w.mobys[id].state = 3; }
            }
            wall(w, id, ctx);
        }
        3 => {
            if let Some(t) = t.filter(|_| flags & 1 == 0) { w.delete(t); }
            w.delete(id);
        }
        _ => {}
    }
}
