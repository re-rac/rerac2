//! 0x0e4c (`0x3f76c8`, 4 placed; the decomp's `func_L07_003F6328`): a swaying turret that fires Endako's tracers
//! (crate::moby::classes::endako_boss::spawn_tracer, `0x3bf850` = L03 `0x3dae28`). It waits hidden until something
//! outside it puts it in state 2: the boss carries it in and wakes it ([`wake`], crate::moby::classes::level07::boss).
//!
//! * Every tick but in state 1 ([`hurt`], `0x3f7b58`): a hit's damage (`0x3140c0(m, 0x10000)`, `0x314120`) taken from
//!   its health (`+0x20`, toward 0, `0x314ef0`); at 0 its reward (`0x310fd0(m, 0, 3)`, crate::moby::bolts::drop), its
//!   class sound 1, state 1, the hit let go, hidden without collision, a blast at its place (`0x3e3a50`: size 1, ten
//!   puffs, twenty streaks, its flash ball) and its sequence 2 cut in (`0x311728`).
//! * **0**: its height and yaw kept (`+0x58`, `+0x60`), hidden without collision; state 1. **1**: nothing.
//! * **2**: its first tick shows it with its class's collision (and sequence 0 unless it plays); at the sequence's end
//!   sequence 1 and state 3.
//! * **3**: the sway ([`sway`], `0x3f7c90`): its phase (`+0x54`) on by `$gp+0x1f70`° a second, its yaw the kept yaw
//!   ± `$gp+0x1f74`° by the phase's sine. Every `$gp+0x1f78` frames a tracer from `$gp+0x1f80` in its frame along its
//!   first row at `$gp+0x1f90` a second, with its class sound 0 and its damage byte (`+0x34`).

use crate::hero::physics::{add, add_rot, set_len, V3};
use crate::moby::blast::{self, Blast};
use crate::moby::classes::endako_boss::spawn_tracer;
use crate::moby::classes::props::wrapped;
use crate::moby::{ai, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0e4c;

/// `$gp+0x1f70`: the sway's phase, degrees a second.
const RATE: f32 = 45.0;
/// `$gp+0x1f74`: the sway's reach either way, degrees.
const REACH: f32 = 30.0;
/// `$gp+0x1f78`: the frames between shots.
const PERIOD: i32 = 20;
/// `$gp+0x1f80`: the muzzle in its frame.
const MUZZLE: V3 = [0.0, 1.5, 0.0];
/// `$gp+0x1f90`: the tracer's speed a second.
const SPEED: f32 = 12.0;

const HIDE: u16 = 0x41;

pub fn update_0e4c(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 1 { return; }
    hurt(w, id, ctx);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let (z, yaw) = (m.pos[2], m.rot[2]);
            m.set_pv_f(0x58, z);
            m.set_pv_f(0x60, yaw);
            m.has_collision = false;
            m.mode = (m.mode | HIDE) & 0xefff;
            m.set_state(1, -1);
        }
        2 => {
            let seq_b = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_b);
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.mode = (m.mode & !HIDE) | 0x1000;
                m.has_collision = true;
                if seq_b != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 1); }
            }
            if wrapped(w, id, ctx) {
                if seq_b != 1 { ctx.host.set_anim(&w.mobys[id], 1, 0, 1); }
                w.mobys[id].set_state(3, -1);
            }
        }
        3 => {
            sway(&mut w.mobys[id], RATE * (std::f32::consts::PI / 180.0 / 60.0));
            if ctx.game.frame % PERIOD == 0 {
                let m = &w.mobys[id];
                let r = m.rows;
                let at = add(std::array::from_fn(|i| MUZZLE[0] * r[0][i] + MUZZLE[1] * r[1][i] + MUZZLE[2] * r[2][i]), m.pos);
                let vel = set_len(r[0], SPEED / 60.0);
                let damage = m.pv_b(0x34) as i32;
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                spawn_tracer(w, &*ctx.host, id, at, vel, damage);
            }
        }
        _ => {}
    }
}

/// `0x3f78c8`: woken (2) with full health, its sway's phase a quarter turn away from Ratchet's side of its kept yaw.
pub fn wake(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let hero = ctx.hero.pos;
    let m = &mut w.mobys[id];
    m.set_state(2, -1);
    m.set_pv_f(0x20, m.pv_s(0x24) as f32);
    let side = crate::hero::physics::sub_rot(crate::hero::physics::atan2(hero[0] - m.pos[0], hero[1] - m.pos[1]), m.pv_f(0x60));
    m.set_pv_f(0x54, if 0.0 < side { -std::f32::consts::FRAC_PI_2 } else { std::f32::consts::FRAC_PI_2 });
    sway(m, 0.0);
}

/// `0x3f7c90(m, rate)`: the phase on by `rate`, the yaw the kept yaw ± [`REACH`]° by its sine.
fn sway(m: &mut crate::moby::Moby, rate: f32) {
    let t = add_rot(m.pv_f(0x54), rate);
    m.set_pv_f(0x54, t);
    m.rot[2] = add_rot(REACH * (t.sin() * 0.017_453_294), m.pv_f(0x60));
}

/// `0x3f7b58`: the hit's damage taken; at no health the reward, the blast and state 1, hidden.
fn hurt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
    let d = ai::damage_of(w, id, hit.as_ref(), false);
    let mut health = w.mobys[id].pv_f(0x20);
    crate::hero::physics::approach(0.0, d, &mut health);
    w.mobys[id].set_pv_f(0x20, health);
    if 0.0 < health { return; }
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
    ctx.host.play_sound(&w.mobys[id], 1, 0);
    w.mobys[id].set_state(1, -1);
    w.clear_hit(id);
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.mode = (m.mode | HIDE) & 0xefff;
    let pos = m.pos;
    blast::burst(w, ctx, pos, &Blast { size: 1.0, bursts: [0; 3], puffs: 10, streaks: 20, light: true, shake: false });
    ctx.host.set_anim(&w.mobys[id], 2, 0, 0);
}
