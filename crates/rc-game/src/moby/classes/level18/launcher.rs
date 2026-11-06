//! Damosel's brood launcher **0x1158** (L18 `0x43d748`, 8 placed; the decomp's `func_L18_0043C258`): a hopping pod
//! that, once Ratchet is near, bursts open and keeps throwing the breeding critters hidden in its group
//! (crate::moby::classes::level18::breeder) at him. Health on its block at `+0x20` (25).
//!
//! **Its pvars:** `+0x50` its colour flash, `+0x60` / `+0x64` its tilts a tick, `+0x70` the cuboid that starts its
//! hopping, `+0x74` the cuboid that opens it, `+0x78` its brood's group, `+0x7c` its resting height, `+0x80` its speed
//! up, `+0x84` its throw's timer (−1 shut).
//!
//! **Each tick:** a hit with flags 0x10000 (`0x338e08` = L00 `0x31dce8`) takes its damage (`0x338e68` = L00
//! `0x31dd48`, crate::moby::ai::damage_of) off its health; any, its flash starts (`0x33b248` = L00 `0x320128`); none
//! left: a blast at it (`0x415e70` = L00 `0x3f8348`: size 1, 0x19 / 0x14 / 0x14 sprites, 0x14 puffs, 0x2d streaks, the
//! flash, the shake), its pieces 1, 3, 4, 5 (mask 0x3a) thrown (`0x341fb8` = L00 `0x327498`,
//! crate::moby::shards::break_into: 3..6 / 3..6), its class sound 1, deleted. Its hit let go, its flash's tick
//! (`0x33b308` = L00 `0x3201e8`).
//! * **0**: its resting height kept; health 1 when it has none; shut; `+0x30` 1; 1.
//! * **1**: Ratchet in `+0x70` (`0x33c668` = L00 `0x321858`): 2.
//! * **2**: each hop starts level at its resting height with up to 5 a second more up and new tilts (up to 20° a
//!   second each), its class sound 0; above its rest it rises by its speed under 30 a second², tilting one way going
//!   up and back coming down; below it, a new hop. Its throw: shut and Ratchet in `+0x74`, it opens — the blast, its
//!   class sound 1 and its pieces 0 and 2 (mask 5) thrown 5 a second along (−1, 0, 0.5) through its rows, jittered by
//!   0.6 a second; open with the timer out, the last hidden critter of its group (`0x3f8a58`) thrown
//!   (`0x3f8b60`, crate::moby::classes::level18::breeder::launch) from 0.7 up its rows at 6 a second along (−1, 0,
//!   0.5) through them, jittered by 0.6 a second (`0x33c5e8` = L00 `0x3217d8`), itself its parent (`+0xb8`), and the
//!   timer 45..120 (none hidden: the timer stays out); else the timer down.
//! * **Every tick** its state's ticks.
//!
//! The opening's jitter is added to a vector the original leaves unset (its stack's); here from 0.

use crate::hero::physics::{add, add_rot, set_len, V3};
use crate::moby::blast::{self, Blast};
use crate::moby::{ai, countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x1158;

const H: usize = 0x20;
const FLASH: usize = 0x50;
const TILT_X: usize = 0x60;
const TILT_Y: usize = 0x64;
const START: usize = 0x70;
const OPEN: usize = 0x74;
const GROUP: usize = 0x78;
const REST: usize = 0x7c;
const UP: usize = 0x80;
const TIMER: usize = 0x84;

/// `$gp+0x2a60..+0x2a84`: its hop (a second), its tilts (° a second), the jitter, the throw's and the opening's speeds
/// (a second), its gravity (a second²), the throw's timer range.
const HOP: f32 = 5.0;
const TILT: (f32, f32) = (20.0, 20.0);
const JITTER: f32 = 0.6;
const THROW: f32 = 6.0;
const BURST: f32 = 5.0;
const GRAVITY: f32 = 30.0;
const THROW_WAIT: (i32, i32) = (45, 120);
/// `$gp+0x2a90` / `+0x2aa0`: where it throws from and the way, in its frame.
const THROW_AT: V3 = [0.0, 0.0, 0.7];
const THROW_WAY: V3 = [-1.0, 0.0, 0.5];

const BLAST: Blast = Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true };

/// `0x309b08` (L00 `0x2ef318`): `v` through the rows.
fn through(rows: &[V3; 3], v: V3) -> V3 { std::array::from_fn(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]) }

pub fn update_1158(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::moby::classes::level08::tabora_ai::jitter;
    use crate::moby::classes::level18::breeder;
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
    let dmg = ai::damage_of(w, id, hit.as_ref(), false);
    let m = &mut w.mobys[id];
    m.set_pv_f(H, m.pv_f(H) - dmg);
    if 0.0 < dmg {
        ai::flash_start(m, FLASH);
        if m.pv_f(H) <= 0.0 {
            let at = m.pos;
            blast::burst(w, ctx, at, &BLAST);
            crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x3a, [0.0; 3], 0xff);
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            w.delete(id);
            return;
        }
    }
    let m = &mut w.mobys[id];
    m.hit_slot = -1;
    ai::flash(m, FLASH);
    let hero = ctx.hero.pos;
    match m.state {
        0 => {
            m.set_pv_f(REST, m.pos[2]);
            if m.pv_s(H + 4) == 0 {
                m.set_pv_s(H + 4, 1);
                m.set_pv_f(H, 1.0);
            }
            m.set_pv_i(TIMER, -1);
            m.set_pv_f(0x30, 1.0);
            m.set_state(1, -1);
        }
        1 => {
            let c = m.pv_i(START);
            if c != -1 && ctx.level.in_cuboid(c, hero) { m.set_state(2, -1); }
        }
        2 => {
            if w.mobys[id].first() {
                let up = ctx.game.rng.randf(0.0, HOP * 0.016_666_668);
                let m = &mut w.mobys[id];
                m.rot[0] = 0.0;
                m.rot[1] = 0.0;
                m.pos[2] = m.pv_f(REST);
                m.set_pv_f(UP, m.pv_f(UP) + up);
                let tx = ctx.game.rng.randf(0.0, TILT.0 * 0.000_290_888_25);
                let ty = ctx.game.rng.randf(0.0, TILT.1 * 0.000_290_888_25);
                let m = &mut w.mobys[id];
                m.set_pv_f(TILT_X, tx);
                m.set_pv_f(TILT_Y, ty);
                m.pos[2] = m.pv_f(REST);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
            }
            let m = &mut w.mobys[id];
            if m.pv_f(REST) <= m.pos[2] {
                m.pos[2] += m.pv_f(UP);
                m.set_pv_f(UP, m.pv_f(UP) - GRAVITY * 0.000_277_777_8);
                let (tx, ty) = if 0.0 < m.pv_f(UP) { (m.pv_f(TILT_X), m.pv_f(TILT_Y)) } else { (-m.pv_f(TILT_X), -m.pv_f(TILT_Y)) };
                m.rot[0] = add_rot(tx, m.rot[0]);
                m.rot[1] = add_rot(ty, m.rot[1]);
            } else {
                m.set_state(2, -1);
            }
            let t = w.mobys[id].pv_i(TIMER);
            if t == 0 {
                let (k, _) = breeder::brood_pick(w, w.mobys[id].pv_i(GROUP));
                if let Some(k) = k {
                    let rows = w.mobys[id].rows;
                    let mut v = through(&rows, set_len(THROW_WAY, THROW * 0.016_666_668));
                    jitter(&mut ctx.game.rng, &mut v, JITTER * 0.016_666_668);
                    let at = add(through(&rows, THROW_AT), w.mobys[id].pos);
                    breeder::launch(w, k, at, v);
                    w.mobys[k].parent = Some(id);
                    let t = ctx.game.rng.rand_range(THROW_WAIT.0, THROW_WAIT.1);
                    w.mobys[id].set_pv_i(TIMER, t);
                }
            } else if t == -1 {
                let c = w.mobys[id].pv_i(OPEN);
                if c != -1 && ctx.level.in_cuboid(c, hero) {
                    w.mobys[id].set_pv_i(TIMER, 0);
                    let at = w.mobys[id].pos;
                    blast::burst(w, ctx, at, &BLAST);
                    ctx.host.play_sound(&w.mobys[id], 1, 0);
                    let rows = w.mobys[id].rows;
                    let v = through(&rows, set_len(THROW_WAY, BURST * 0.016_666_668));
                    let mut p = [0.0; 3];
                    jitter(&mut ctx.game.rng, &mut p, JITTER * 0.016_666_668);
                    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 5, add(v, p), 0xff);
                }
            } else {
                let mut t = t;
                countdown(&mut t);
                w.mobys[id].set_pv_i(TIMER, t);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.state_ticks as u16 <= 0xfffe { m.state_ticks = m.state_ticks.wrapping_add(1); }
}
