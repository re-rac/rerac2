//! Grelbin's props (L19 addresses).
//!
//! * **0x0629** ([`update_0629`], `0x381ac8`, 18 placed; the decomp's `func_L19_00380918`): a moby that stays while
//!   at least `+0x00` of its eight watched mobys (`+0x10..`) live as the classes they began as; fewer, and it goes.
//!   With a count of 0 it stops updating (mode 2) at once.
//! * **0x11c7** ([`update_11c7`], `0x421720`, 68 placed; the decomp's `func_L19_004203F0`): an ice formation.
//!   * It is gone if its uid is counted in the save (with a mission, `+0xb0` ≠ 0xff).
//!   * **Set-up**: it sits 0.1 into the ground under it (`0x2a89a8`), drawn, its rows kept (mode 0x100), state 1.
//!   * **1**: hits with the flags 0x230000 take their damage off its health (`+0x20`). Each hit flashes it white
//!     (`+0x50`) and cuts its sequence 0 in at the frame of the damage taken so far (at most 7). While it lives, a
//!     hit also plays one of its sounds 0..2.
//!   * At no health it shatters:
//!     * pieces 0..3 fly (`0x31dd30`: up 0.15 a tick, 3..6 / 3..6);
//!     * a 0x1266 is left in its place (alpha 0x50, mode 0x800, 0.65 of its class's scale, linked back at `+4`);
//!     * its sound 3 plays;
//!     * it is hidden without collision (2).
//!
//! Not ported for 0x11c7: its glow word (`$gp`-relative `0x1b1de0` in v1.01); its rows turned to the ground's
//! normal (`0x2b0408` with the line's normal `0x1bfec0`); its draw while drawn (`0x423270`); `0x2ab650` on the
//! piece left; the level's state table `0x21ea30` as a second gone test.

use crate::moby::{ai, shards, Ctx, MobyWorld};

pub const WATCHER: i16 = 0x0629;
pub const ICE: i16 = 0x11c7;
const ICE_LEFT: i16 = 0x1266;

pub fn update_0629(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let alive_as = |w: &MobyWorld, i: i32, c: i32| usize::try_from(i).ok().and_then(|i| w.mobys.get(i)).is_some_and(|o| o.state < 0xfd && o.o_class as i32 == c);
    match w.mobys[id].state {
        0 => {
            for k in 0..8 {
                let i = w.mobys[id].pv_i(0x10 + 4 * k);
                let c = usize::try_from(i).ok().and_then(|i| w.mobys.get(i)).filter(|o| o.state < 0xfd).map_or(-1, |o| o.o_class as i32);
                w.mobys[id].set_pv_i(0x30 + 4 * k, c);
            }
            let m = &mut w.mobys[id];
            m.state = 1;
            if m.pv_i(0) == 0 {
                m.state = 2;
                m.mode |= 2;
            }
        }
        1 => {
            let mut n = 0;
            for k in 0..8 {
                let (i, c) = (w.mobys[id].pv_i(0x10 + 4 * k), w.mobys[id].pv_i(0x30 + 4 * k));
                if i != -1 && c != -1 && alive_as(w, i, c) {
                    n += 1;
                } else {
                    w.mobys[id].set_pv_i(0x30 + 4 * k, -1);
                }
            }
            if n < w.mobys[id].pv_i(0) { w.delete(id); }
        }
        _ => {}
    }
}

pub fn update_11c7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    if m.mission != 0xff && ctx.game.uid_count(m.uid) != 0 { return w.delete(id); }
    if w.mobys[id].state == 0 {
        let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0) - 0.1;
        let m = &mut w.mobys[id];
        m.pos[2] = z;
        m.drawn = 1;
        m.mode |= 0x100;
        m.set_state(1, 0);
    }
    if w.mobys[id].state != 1 { return; }
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x23_0000 != 0);
    let dmg = ai::damage_of(w, id, hit.as_ref(), false);
    if 0.0 < dmg {
        let m = &mut w.mobys[id];
        let hp = m.pv_f(0x20) - dmg;
        m.set_pv_f(0x20, hp);
        for k in 7..10 { m.set_pv_b(0x50 + k, 0xff); }
        ai::flash_start(m, 0x50);
        let taken = (m.pv_s(0x24) as f32 - hp) as i32;
        ctx.host.set_anim(&w.mobys[id], 0, taken.clamp(0, 7) as u8, 0);
        let s = ctx.game.rng.rand_range(0, 2);
        if 0.0 < hp { ctx.host.play_sound(&w.mobys[id], s, 0); }
    }
    w.clear_hit(id);
    ai::flash(&mut w.mobys[id], 0x50);
    if 0.0 < w.mobys[id].pv_f(0x20) { return; }
    shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 15, [0.0, 0.0, 0.15], 0xff);
    if w.class(ICE_LEFT).is_some() {
        let s = w.class(ICE_LEFT).map_or(1.0, |c| c.scale) * 0.65;
        let n = w.create(ICE_LEFT);
        let pos = w.mobys[id].pos;
        let q = &mut w.mobys[n];
        (q.alpha, q.scale, q.update_dist, q.draw_dist, q.drawn, q.timer, q.pos) = (0x50, s, 0xff, 0xff, 1, 0, pos);
        q.mode |= 0x800;
        ai::set_link(q, 4, Some(id));
        w.changed = true;
    }
    ctx.host.play_sound(&w.mobys[id], 3, 0);
    let m = &mut w.mobys[id];
    m.set_state(2, -1);
    m.mode = (m.mode | 0x41) & 0xefff;
    m.has_collision = false;
}
