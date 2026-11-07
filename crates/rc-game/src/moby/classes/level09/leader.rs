//! Dobbo's leader **0x0e1a** ([`update_0e1a`], L09 `0x40a7f8`, one placed; the decomp's `func_L09_00409308`): a
//! creature that idles about until Ratchet rides the vehicle 0x0905 (his state 0x31 with `0x18b650` it), then runs
//! along its path ahead of him, a point light with it, and is gone at the path's end.
//!
//! **Its pvars**: `+0x250` the point it looks at, `+0x260` its path, `+0x264` its speed (a second, along the path's
//! first segment), `+0x268` how far along (points), `+0x274` its light's slot, `+0x278` the light's radius, `+0x27c..`
//! its colour, `+0x288` its intensity, `+0x290` the ticks it keeps looking at Ratchet, `+0x294` the wait for a new
//! look point, `+0x298` the wait for an idle fidget, `+0x29c` the step along the path a tick.
//!
//! **0**: the path's start, its step its speed over the first segment's length (a tick); the light taken
//! (`0x2f9dd8` = L00 `0x2fdb60`); 2. **2** (waiting): Ratchet on a 0x0905 that is not going: updated and drawn at any
//! distance, the start again, its sequence 4 (blend 10), 1; then as 3. **3** (and 2): in its sequence 0 with the wait
//! out, the wait `randf(1200, 2400)` ticks and the sequence 1 or 2 (by a coin, blend 10); else at its sequence's end
//! the sequence 0; its yaw 1% of the way a tick toward Ratchet. **1** (running): in the sequence 4 with the wait
//! out, the wait anew and the sequence 2; else at its sequence's end the sequence 4; its place and angles the path's
//! pose at its count (`0x341250` = L00 `0x343fb8`, looped); its step 1% up within 10 of Ratchet, 1% down past 30, else
//! eased 5% toward its speed's; the count on by the step; the light at its place; within two points of the path's end
//! it is done.
//!
//! **After the state** in the sequence 0: within 8 of Ratchet across (`0x2ef050`) with him in front (within 90°), the look on him for 120
//! ticks while he moves (else counted down), else (the look over) the look point his centre; the wait for a new
//! look point out, a new wait `randf(180, 300)` and the look point 6 out at a random turn of ±90° and a pitch of up
//! to 30°. Done: the light freed and gone; else its sparkles (`0x40a6b8`, [`sparkles`]).
//!
//! Not ported: the head's turn toward the look point (the update works it out and nothing reads it here).

use crate::hero::physics::{add, add_rot, dist, len, sub_rot};
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::{Ctx, MobyWorld};
use crate::point_lights::PointLight;

pub const CLASS: i16 = 0x0e1a;
/// The vehicle Ratchet rides when it runs.
const VEHICLE: i16 = 0x0905;

const LOOK: usize = 0x250;
const PATH: usize = 0x260;
const SPEED: usize = 0x264;
const ALONG: usize = 0x268;
const LIGHT: usize = 0x274;
const RADIUS: usize = 0x278;
const RGB: usize = 0x27c;
const INTENSITY: usize = 0x288;
const STARE: usize = 0x290;
const LOOK_WAIT: usize = 0x294;
const FIDGET: usize = 0x298;
const STEP: usize = 0x29c;

fn points<'a>(w: &MobyWorld, id: usize, ctx: &'a Ctx) -> &'a [[f32; 4]] {
    usize::try_from(w.mobys[id].pv_i(PATH)).ok().and_then(|p| ctx.level.paths.get(p)).map_or(&[], |p| p.as_slice())
}

/// The step a tick for its speed over the path's first segment.
fn base_step(w: &MobyWorld, id: usize, ctx: &Ctx) -> f32 {
    let p = points(w, id, ctx);
    let d = if 2 <= p.len() { dist([p[0][0], p[0][1], p[0][2]], [p[1][0], p[1][1], p[1][2]]) } else { 0.0 };
    1.0 / d * 0.016_666_668 * w.mobys[id].pv_f(SPEED)
}

fn light(w: &MobyWorld, id: usize) -> PointLight {
    let m = &w.mobys[id];
    PointLight { color: m.pv_v(RGB), intensity: m.pv_f(INTENSITY), pos: m.pos, radius: m.pv_f(RADIUS) }
}

/// 0x0e1a (`0x40a7f8`): see the module.
pub fn update_0e1a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0x2a0 { w.mobys[id].pvars.resize(0x2a0, 0); }
    let state = w.mobys[id].state;
    match state {
        0 => {
            w.mobys[id].set_pv_f(ALONG, 0.0);
            let step = base_step(w, id, ctx);
            w.mobys[id].set_pv_f(STEP, step);
            let s = w.point_lights.alloc(light(w, id)).map_or(-1, |s| s as i32);
            w.mobys[id].set_pv_i(LIGHT, s);
            w.mobys[id].set_state(2, -1);
        }
        2 | 3 => {
            if state == 2 {
                let going = ctx.hero.riding().and_then(|v| w.mobys.get(v)).is_some_and(|v| v.o_class == VEHICLE && !matches!(v.state, 0xfe | 0xfd));
                if going {
                    let m = &mut w.mobys[id];
                    m.update_dist = 0xff;
                    m.draw_dist = 0x3ff;
                    m.set_pv_f(ALONG, 0.0);
                    set_seq(w, id, ctx, 4, 0, 10);
                    w.mobys[id].set_state(1, -1);
                }
            }
            if seq(w, id, ctx) == 0 && w.mobys[id].count_i(FIDGET) {
                let t = ctx.game.rng.randf(1200.0, 2400.0) as i32;
                w.mobys[id].set_pv_i(FIDGET, t);
                let s = (ctx.game.rng.rand() & 1) as u8 + 1;
                set_seq(w, id, ctx, s, 0, 10);
            } else if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 0, 0, 10);
            }
            let m = &mut w.mobys[id];
            let to = (ctx.hero.pos[1] - m.pos[1]).atan2(ctx.hero.pos[0] - m.pos[0]);
            m.rot[2] = add_rot(m.rot[2], sub_rot(to, m.rot[2]) * 0.01);
        }
        1 => {
            if seq(w, id, ctx) == 4 && w.mobys[id].count_i(FIDGET) {
                let t = ctx.game.rng.randf(1200.0, 2400.0) as i32;
                w.mobys[id].set_pv_i(FIDGET, t);
                set_seq(w, id, ctx, 2, 0, 10);
            } else if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 4, 0, 10);
            }
            let along = w.mobys[id].pv_f(ALONG);
            let p = points(w, id, ctx);
            let n = p.len();
            if 2 <= n {
                let (pos, pitch, yaw, roll) = crate::paths::pose(p, along, true);
                let m = &mut w.mobys[id];
                m.pos = pos;
                m.rot = [roll, pitch, yaw];
            }
            let d = dist(w.mobys[id].pos, ctx.hero.pos);
            let mut step = w.mobys[id].pv_f(STEP);
            if d < 10.0 {
                step *= 1.01;
            } else if 30.0 < d {
                step *= 0.99;
            } else {
                step += (base_step(w, id, ctx) - step) * 0.05;
            }
            let m = &mut w.mobys[id];
            m.set_pv_f(STEP, step);
            m.set_pv_f(ALONG, along + step);
            let slot = m.pv_i(LIGHT);
            if slot != -1 { let l = light(w, id); w.point_lights.set(slot as usize, l); }
            let m = &mut w.mobys[id];
            if n as i32 <= m.pv_f(ALONG) as i32 + 2 { m.timer = 1; }
        }
        _ => {}
    }
    if seq(w, id, ctx) == 0 {
        let m = &w.mobys[id];
        let centre = add(ctx.hero.pos, [0.0, 0.0, 1.0]);
        let to = (centre[1] - m.pos[1]).atan2(centre[0] - m.pos[0]);
        let across = (m.pos[0] - ctx.hero.pos[0]).hypot(m.pos[1] - ctx.hero.pos[1]);
        if across < 8.0 && sub_rot(m.rot[2], to).abs() < std::f32::consts::FRAC_PI_2 {
            if 0.01 < len(ctx.hero.vel) { w.mobys[id].set_pv_i(STARE, 120); } else { w.mobys[id].count_i(STARE); }
        } else if w.mobys[id].pv_i(STARE) != 0 {
            let m = &mut w.mobys[id];
            m.set_pv_i(STARE, 0);
            m.set_pv_v(LOOK, centre);
        }
        if w.mobys[id].count_i(LOOK_WAIT) {
            let g = &mut ctx.game.rng;
            let wait = g.randf(180.0, 300.0) as i32;
            let yaw = add_rot(w.mobys[id].rot[2], g.randf(-90.0, 90.0) * 0.017_453_294);
            let pitch = g.randf(0.0, 30.0) * 0.017_453_294;
            let m = &mut w.mobys[id];
            m.set_pv_i(LOOK_WAIT, wait);
            let at = add(crate::moby::classes::shot::polar(6.0, yaw, pitch), m.pos);
            m.set_pv_v(LOOK, at);
        }
    }
    if w.mobys[id].timer != 0 {
        let slot = w.mobys[id].pv_i(LIGHT);
        if slot != -1 { w.point_lights.free(slot as usize); }
        w.delete(id);
    } else {
        sparkles(w, id, ctx);
    }
}

/// `0x40a6b8` (the decomp's `func_L09_004091C8`): two pairs of falling sparks a tick at its joint 2 (each pair 0.015
/// higher than the last), red-violet (life 17, size 150 shrinking 100) or red (19, 220) then white (10, 80 shrinking
/// 80), all at one fall `randf(3.5, 4.5)` a second (`0x32f7c0` = L00 `0x332c10`, crate::particles::trail).
fn sparkles(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let fall = -(ctx.game.rng.randf(3.5, 4.5) * 0.016_666_668);
    let vel = [0.0, 0.0, fall];
    let Some(at) = ctx.host.joint(&w.mobys[id], 2) else { return };
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    for i in 0..2 {
        let pos = [at[0], at[1], at[2] + i as f32 * 0.015];
        if i & 1 != 0 {
            crate::particles::trail(sys, rng, pos, vel, 0x77ff_0000, 0x00ff_0060, 17, 150, -100, true);
        } else {
            crate::particles::trail(sys, rng, pos, vel, 0x77ff_0060, 0x00ff_0000, 19, 220, -100, true);
        }
        crate::particles::trail(sys, rng, pos, vel, 0x77ff_ffff, 0x00ff_0000, 10, 80, -80, true);
    }
}
