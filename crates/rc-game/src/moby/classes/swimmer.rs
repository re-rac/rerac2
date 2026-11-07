//! **0x0947**, the water creature that swallows Ratchet (L06 `0x404c60`, L09 `0x3c19e0`, L12 `0x3c54f8`; the decomp's
//! `func_L06_00403710`, one function on Notak, Dobbo and Todano; 10 placed). Its pvars: `+0x48` / `+0x4a` two bytes
//! (15 and 12 when it lies in wait; nothing here reads them), `+0x50` the path (the level's paths), `+0x54` lying in
//! wait (else it swims the path), `+0x58` the path point it swims to, `+0x5c` that point's timeout, `+0x60` the turn
//! speed, `+0x64` its home height, `+0x68` the rise speed, `+0x6c` the wait, `+0x70` the swim height, `+0x78` a cuboid
//! Ratchet is safe in, `+0x7c` / `+0x80` the waits outside / inside the path's polygon.
//!
//! **States**:
//! * **0**: lying in wait: the bytes 15 / 12, 2, its update distance 255, hidden (`+0x34` bit 0), its home height its
//!   place's, the wait 30. Else with no path (−1) it is gone; else a random point of the path (`0x31b9a0`), the
//!   timeout 600, its sequence 1 (blend 10) unless it plays, its height the path's first point's less 1 (also
//!   `+0x70`), 1.
//! * **1**: swimming the path: its yaw eased toward the point (`0x31f230`: 0.000873, 0.000873, at most 0.0524), 0.05
//!   along its forward row a tick; within 3 of the point or the timeout out (`0x2eeab0`), the point moves on by a
//!   random count of the path's points (`(point + rand(n)) mod n`) and the timeout is 600 again.
//! * **2**: lying in wait: the wait counts down. Ratchet swimming on the surface (his group 0x12) with his water level
//!   (`0x18a150`) within 0.8 of its home height and out of the cuboid `+0x78`: once the wait is out, it takes him —
//!   pitched −89° and turned to the view's yaw (`0x1b9858`, the camera's; the port's last view's forward) plus 90°, its
//!   rows (`0x2ef6b8`) and place under him (his place less its up row), the rise speed 8/60, shown, its sound 1, its
//!   sequence 2 (blend 5) unless it plays, 3; the splash ([`splash`]) at his place on his water level, and Ratchet
//!   placed where he is in his state 0x6f (`0x2b9e68(pos, rot, 0x6f, 0)`, crate::moby::director::place_hero) with the
//!   view lock `0x18c0d5` set (he is hidden). Otherwise, Ratchet not in his state 0x12, the wait is `+0x80` inside the
//!   path's polygon (`0x31cf68`), else `+0x7c`.
//! * **3**: rising with him: its first tick its sound 1 again; every tick his velocity, his move, his effective
//!   velocity and its vertical part zeroed (`0x189f10` / `0x189f30` / `0x189f40` / `0x189f50`) and the view lock set.
//!   Its height rises by the speed, which falls by 1/360 a tick (Dobbo's 1/180); under 1/30 it tips over toward +89°
//!   (`0x31f230`: 0.00349, 0.00349, at most 0.419) about the point its up row reaches (its place moved by the up row's
//!   change). Falling below Ratchet's water level: the splash there each tick; below its home height: Ratchet dies
//!   (`0x2b31f0`, crate::hero::Hero::die). Its sequence wrapped while not 1: 1 (blend 10).
//!
//! **The splash** (L06 `0x4052f8`, the decomp's `func_L06_00403DA8`): 16 drops (crate::particles::water::drop) out of
//! the point, each a random way across at up to 0.05 and 0.05..0.108 up, of kind `rand(2)` for 90..120 ticks; then 16
//! sprays (crate::particles::water::spray) within 1 of the point across at the water plane's height (`0x1b2fc8`,
//! crate::water::plane: off but on Todano) + 0.05, sized 0.7..1, spinning 2 (the first) or −2, each living 30..60
//! ticks.
//!
//! Not ported: Ratchet's word `0x18a010` it sets to 3 while it holds him; the sprays' height pointer (the water plane's
//! height, which they settle on within 0.2: here they keep their own).

use crate::hero::physics::{add, add_rot, atan2, dist, set_len, sub, turn_ease, V3};
use crate::moby::{countdown, Ctx, MobyWorld};
use super::props::{set_seq, wrapped};

pub const CLASS: i16 = 0x0947;

const PATH: usize = 0x50;
const WAITS: usize = 0x54;
const POINT: usize = 0x58;
const TIMEOUT: usize = 0x5c;
const TURN: usize = 0x60;
const HOME: usize = 0x64;
const SPEED: usize = 0x68;
const WAIT: usize = 0x6c;
const SWIM_Z: usize = 0x70;
const SAFE: usize = 0x78;
const WAIT_OUT: usize = 0x7c;
const WAIT_IN: usize = 0x80;

/// The level's `$gp` word every copy reads (L06 `0x1b0040`, L09 `0x1b0050`, L12 `0x1aff90`): 1.
const REACH: f32 = 1.0;

/// 0x0947.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let path = w.mobys[id].pv_i(PATH);
    let pts = usize::try_from(path).ok().and_then(|i| ctx.level.paths.get(i)).cloned().unwrap_or_default();
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            if m.pv_i(WAITS) != 0 {
                m.set_pv_b(0x48, 15);
                m.set_pv_b(0x4a, 12);
                m.set_state(2, -1);
                m.update_dist = 255;
                m.mode |= 1;
                m.set_pv_f(HOME, m.pos[2]);
                m.set_pv_i(WAIT, 30);
                return;
            }
            if path == -1 { return w.delete(id); }
            let Some(first) = pts.first().copied() else { return };
            let k = ctx.game.rng.randi(pts.len() as i32);
            let m = &mut w.mobys[id];
            m.set_pv_i(POINT, k);
            m.set_pv_i(TIMEOUT, 600);
            set_seq(w, id, ctx, 1, 0, 10);
            let m = &mut w.mobys[id];
            m.pos[2] = first[2] - REACH;
            m.set_pv_f(SWIM_Z, m.pos[2]);
            m.set_state(1, -1);
        }
        1 => {
            if pts.is_empty() { return; }
            let n = pts.len() as i32;
            let m = &mut w.mobys[id];
            let p = pts[m.pv_i(POINT).rem_euclid(n) as usize];
            let to = [p[0], p[1], p[2]];
            let (mut yaw, mut v) = (m.rot[2], m.pv_f(TURN));
            let a = f32::from_bits(0x3a64_c38a);
            turn_ease(atan2(to[0] - m.pos[0], to[1] - m.pos[1]), a, a, f32::from_bits(0x3d56_7752), &mut yaw, &mut v, 0);
            m.rot[2] = yaw;
            m.set_pv_f(TURN, v);
            m.pos = add(m.pos, set_len(m.rows[0], f32::from_bits(0x3d4c_ccce)));
            let near = dist(m.pos, to) < 3.0;
            let mut t = m.pv_i(TIMEOUT);
            let out = !near && countdown(&mut t);
            m.set_pv_i(TIMEOUT, t);
            if near || out {
                let step = ctx.game.rng.randi(n);
                let m = &mut w.mobys[id];
                m.set_pv_i(POINT, (m.pv_i(POINT) + step) % n);
                m.set_pv_i(TIMEOUT, 600);
            }
        }
        2 => {
            let m = &mut w.mobys[id];
            let mut wait = m.pv_i(WAIT);
            countdown(&mut wait);
            m.set_pv_i(WAIT, wait);
            let hero = ctx.hero.pos;
            let water = ctx.hero.surf.water_z;
            if (water - m.pv_f(HOME)).abs() < f32::from_bits(0x3f4c_cccd) && ctx.hero.group == 0x12 && !ctx.level.in_cuboid(m.pv_i(SAFE), hero) {
                if wait == 0 { take(w, id, ctx); }
            } else if ctx.hero.state != 0x12 {
                let inside = ctx.level.in_polygon(path, hero);
                let m = &mut w.mobys[id];
                m.set_pv_i(WAIT, m.pv_i(if inside { WAIT_IN } else { WAIT_OUT }));
            }
        }
        3 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 1, 0); }
            let h = &mut *ctx.hero;
            (h.vel, h.disp, h.eff, h.eff_v) = ([0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3]);
            h.view_lock = 1;
            let m = &mut w.mobys[id];
            let mut speed = m.pv_f(SPEED);
            m.pos[2] += speed;
            speed -= if ctx.game.level != 9 { f32::from_bits(0x3b36_0b61) } else { f32::from_bits(0x3bb6_0b61) };
            m.set_pv_f(SPEED, speed);
            if speed < f32::from_bits(0x3d08_8889) {
                let before = set_len(m.rows[2], REACH);
                let (mut pitch, mut v) = (m.rot[1], m.pv_f(TURN));
                let a = f32::from_bits(0x3b64_c38a);
                turn_ease(f32::from_bits(0x3fc6_d3f3), a, a, f32::from_bits(0x3ed6_7752), &mut pitch, &mut v, 0);
                m.rot[1] = pitch;
                m.set_pv_f(TURN, v);
                m.rows = crate::moby::attach::rows_of(m.rot);
                let after = set_len(m.rows[2], REACH);
                m.pos = sub(m.pos, sub(after, before));
            }
            let water = ctx.hero.surf.water_z;
            if speed < 0.0 && m.pos[2] < water {
                let at = [ctx.hero.pos[0], ctx.hero.pos[1], water];
                splash(ctx, at, w.liquid.plane.at[2]);
            }
            let m = &w.mobys[id];
            if speed < 0.0 && m.pos[2] < m.pv_f(HOME) { ctx.hero.die(); }
            if wrapped(w, id, ctx) { set_seq(w, id, ctx, 1, 0, 10); }
        }
        _ => {}
    }
}

/// State 2's take (module docs).
fn take(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let r = ctx.game.cameras.last_view.1[0];
    let view_yaw = atan2(r[0], r[1]);
    let hero = ctx.hero.pos;
    let m = &mut w.mobys[id];
    m.rot[1] = f32::from_bits(0xbfc6_d3f3);
    m.rot[2] = add_rot(view_yaw, f32::from_bits(0x3fc9_0fdc));
    m.rows = crate::moby::attach::rows_of(m.rot);
    m.pos = sub(hero, set_len(m.rows[2], REACH));
    m.set_pv_f(SPEED, f32::from_bits(0x3e08_8889));
    m.mode &= !1;
    ctx.host.play_sound(&w.mobys[id], 1, 0);
    set_seq(w, id, ctx, 2, 10, 5);
    w.mobys[id].set_state(3, -1);
    let water = ctx.hero.surf.water_z;
    splash(ctx, [hero[0], hero[1], water], w.liquid.plane.at[2]);
    let rot = ctx.hero.rot;
    crate::moby::director::place_hero(ctx, hero, rot, 0x6f);
    ctx.hero.view_lock = 1;
}

/// L06 `0x4052f8` (module docs).
/// `plane_z`: the water plane's height (`0x1b2fc8`).
fn splash(ctx: &mut Ctx, at: V3, plane_z: f32) {
    let rng = &mut ctx.game.rng;
    for _ in 0..16 {
        let angle = rng.rand_angle();
        let r = rng.randf(0.0, f32::from_bits(0x3d4c_ccce));
        let up = rng.randf(f32::from_bits(0x3d4c_ccce), f32::from_bits(0x3ddd_dddf));
        let vel = [angle.cos() * r, angle.sin() * r, up];
        let life = rng.rand_range(90, 120);
        let kind = rng.randi(2);
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::water::drop(sys, rng, at, vel, kind, life); }
    }
    for i in 0..16 {
        let mut p = add(rng.rand_vec(1.0, 1.0), at);
        p[2] = plane_z + f32::from_bits(0x3d4c_cccd);
        let size = rng.randf(f32::from_bits(0x3f33_3333), 1.0);
        let spin = if i == 0 { 2.0 } else { -2.0 };
        let Some(sys) = ctx.parts.as_deref_mut() else { continue };
        if let Some(k) = crate::particles::water::spray(sys, rng, size, spin, p, [0.0; 3], false) {
            let life = rng.randf(30.0, 60.0) as i32 as i16;
            crate::particles::rec::set_i16(&mut sys.pool.recs[k], 0xa, life);
        }
    }
}
