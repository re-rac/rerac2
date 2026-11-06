//! The bounce pads: a jump from their top is Ratchet's bounce 0x81 (crate::hero::jump's `BOUNCE_PADS`, his
//! `bounce` = `$gp − 0x7318` / `$gp − 0x7314`: the bounce's height and gravity, which the pads set).
//!
//! * **0x0f05** ([`update_0f05`], L16 `0x40d2f8`, 23 placed; L17 `0x417c30`, 2; the decomp's v1.01
//!   `func_L16_0040BED0`, its helpers `func_L16_0040C7E8` / `0040C5F0` at our L16 `0x40dc10` / `0x40da18`): a pad that
//!   flings Ratchet across to a point. Its pvars: `+0x00` the cuboid it throws toward (−1: the moby `+0x04`'s place;
//!   both −1, deleted), `+0x0c` the fling's slowing a tick, `+0x10` its velocity, `+0x20` its target, `+0x30` its
//!   looped sound, `+0x38` / `+0x40` the camera tweak it holds on level 17.
//!   * **0** facing away from its target, update distance 0xff, mode | 0x2000, its sound none; on level 16 the bounce's
//!     gravity 25; then 1.
//!   * **1** Ratchet on it: the bounce's height `(target z − his z) · 2.5`, at least 37. Bouncing (0x81) up off it,
//!     its sound 1 and the fling (2): across toward the target at `√(2·r·d)` (`d` the distance across, `r` =
//!     gravity · d / 3600 · ½ (on level 16 0.3) / height, kept as the slowing), his own speed across 0. Else, one
//!     tick in 16, with the moby `+0x04` hidden, 3.
//!   * **2** while he bounces: the fling slowed by `r` (stopped within 0.5 of the target or past it), his speed across 0
//!     and his push (`0x189f20`) the fling, facing along it, 10 ticks unhurt; on level 17 a camera tweak (priority 5:
//!     distance 4, pivot 1, pitch 0, the stick off, facing (−2, −3, −5) once). Landed, the tweak let go, 1.
//!   * **3** with the moby `+0x04` hidden it fades out (alpha −4 a tick) without collision; shown again it fades in and
//!     at full (0x80) its collision comes back, 1.
//!   * Drawn and not in 3, a streak a tick off its top ([`streak`]). On level 16 with Ratchet within 3 the help 0xa0c
//!     (slot 99, again after 1081 ticks); flinging, the slot off. Shown and solid its looped class sound 0, else
//!     stopped.
//! * **0x09b9** ([`update_09b9`], L22 `0x3cacc8`, 1 placed; L26 `0x3bc9f0`, 11; the decomp's `func_L22_003C9880`,
//!   its helpers `func_L22_003C9B60` / `003C9D48` at our L22 `0x3cafa8` / `0x3cb190`): the planet pads. Its pvars:
//!   `+0x00` the bounce's height, `+0x04` its glow's phase, `+0x10` / `+0x20` / `+0x30` four rings' radii, phases and
//!   rates.
//!   * **0** the rings: phases 0, ¼, ½, ¾, rates 1.5..2 a second, radii 3..4; draw distance 0xff; then 1.
//!   * **1** within 3 of Ratchet the bounce's height its `+0x00`; his bounce's first tick, its class sound 0; the help
//!     0x9c8 (slot 31) while he stands on it, the slot off once he bounces. Upright against his gravity (its up row
//!     · his down `0x18a0e0` under −0.6), the rings ([`rings`]).
//!   * Drawn, its glow pulses (240° a second, `0x80505050` → `0x80c0c0c0` by sin⁸).
//!
//! Engine functions: `0x315048` / `0x320e30` set_state, `0x3584d0` / `0x363f80` the class sound, `0x3581d0` its stop,
//! `0x2ee660` / `0x2fac00` = L00 `0x2f5270` the help (crate::game::GameState::help), `0x2d0680` / `0x2d0720` = L00
//! `0x2d7468` / `0x2d7508` the camera tweak (crate::camera::CamTweaks), `0x337770` = L00 `0x33f940` the streak
//! (crate::particles::streak), `0x33ec90` = L00 `0x33b5e0` the glint (crate::particles::glint).
//!
//! Not ported: 0x09b9's rings' shimmer (`0x3cb190` on the frame's first list: four heat-haze sprites `0x2ee190`
//! sampling the frame behind them).

use crate::hero::physics::{add, add_rot, atan2, dist, dist2, dot, len2, set_len2, sub, V3};
use crate::moby::{Ctx, MobyWorld};

pub const PAD: i16 = 0x0f05;
pub const PLANET_PAD: i16 = 0x09b9;

const P_CUBOID: usize = 0x00;
const P_MOBY: usize = 0x04;
const P_RATE: usize = 0x0c;
const P_VEL: usize = 0x10;
const P_TARGET: usize = 0x20;
const P_SOUND: usize = 0x30;
const P_ACTIVE: usize = 0x38;
const P_TWEAK: usize = 0x40;
const P_PVARS: usize = 0x50;

/// L16 `$gp+0x2568` (L17 `+0x2ac8`): the bounce's gravity on level 16.
const GRAVITY_16: f32 = 25.0;
/// The help slot of 0x0f05 and its text.
const HELP_SLOT: usize = 99;
const HELP_TEXT: i32 = 0xa0c;

fn moby_pos(w: &MobyWorld, id: usize) -> V3 { usize::try_from(w.mobys[id].pv_i(P_MOBY)).ok().and_then(|k| w.mobys.get(k)).map_or([0.0; 3], |t| t.pos) }
fn cuboid_pos(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    usize::try_from(w.mobys[id].pv_i(P_CUBOID)).ok().and_then(|c| ctx.level.cuboids.get(c)).map_or([0.0; 3], |c| [c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]])
}

/// `0x40dc10`: the pad's target, the moby `+0x04`'s place, or with none the cuboid `+0x00`'s.
fn target(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 { if w.mobys[id].pv_i(P_MOBY) != -1 { moby_pos(w, id) } else { cuboid_pos(w, id, ctx) } }

/// The moby `+0x04` hidden (mode bit 1).
fn linked_hidden(w: &MobyWorld, id: usize) -> bool {
    usize::try_from(w.mobys[id].pv_i(P_MOBY)).ok().and_then(|k| w.mobys.get(k)).is_some_and(|t| t.mode & 1 != 0)
}

/// A help slot's step (both pads): with the slot on and shown before, again after 1081 ticks; first time, counted.
fn help_step(ctx: &mut Ctx, slot: usize, text: i32) {
    let now = ctx.game.play_time;
    let level_bit = 1u32.checked_shl(ctx.game.level.max(0) as u32).unwrap_or(0);
    let g = &mut *ctx.game;
    let last = g.help_last[slot];
    if g.help_counts[slot] != 0 {
        if 0x438 < now.wrapping_sub(last) as u32 || last == 0 {
            g.help(text, slot as i32);
        } else if (last as u32) < now as u32 {
            g.help_last[slot] = now;
        }
    } else {
        g.help_counts[slot] = g.help_counts[slot].wrapping_add(1);
        if (last as u32) < now as u32 { g.help_last[slot] = now; }
        g.help_levels[slot] |= level_bit | 0x8000_0000;
    }
}

/// 0x0f05 (module docs).
pub fn update_0f05(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.scenes.mode == crate::scene::SCENE { return; }
    if w.mobys[id].pvars.len() < P_PVARS { w.mobys[id].pvars.resize(P_PVARS, 0); }
    let level = ctx.game.level;
    match w.mobys[id].state {
        0 => {
            if level == 0x10 { ctx.hero.bounce[1] = GRAVITY_16; }
            let m = &w.mobys[id];
            let to = if m.pv_i(P_CUBOID) != -1 {
                cuboid_pos(w, id, ctx)
            } else {
                if m.pv_i(P_MOBY) == -1 {
                    w.delete(id);
                    return;
                }
                moby_pos(w, id)
            };
            let m = &mut w.mobys[id];
            m.rot[2] = atan2(m.pos[0] - to[0], m.pos[1] - to[1]);
            m.set_pv_i(P_SOUND, -1);
            m.update_dist = 0xff;
            m.mode |= 0x2000;
            m.set_state(1, -1);
        }
        1 => {
            if ctx.hero.ground_moby == Some(id) {
                let to = target(w, id, ctx);
                w.mobys[id].set_pv_v(P_TARGET, to);
                let h = (to[2] - ctx.hero.pos[2]) * 2.5;
                ctx.hero.bounce[0] = if h < 37.0 { 37.0 } else { h };
            }
            if ctx.hero.state == 0x81 {
                if ctx.hero.ground_moby == Some(id) && 0.0 < ctx.hero.vel[2] {
                    ctx.host.play_sound(&w.mobys[id], 1, 0);
                    let k = if level == 0x10 { f32::from_bits(0x3e99_999a) } else { 0.5 };
                    let to = w.mobys[id].pv_v(P_TARGET);
                    let d = dist2(ctx.hero.pos, to);
                    let [height, gravity] = ctx.hero.bounce;
                    let r = gravity * (d * f32::from_bits(0x3991_a2b4)) * k / height;
                    let speed = ((r + r) * d).sqrt();
                    let mut diff = sub(to, ctx.hero.pos);
                    diff[2] = 0.0;
                    let m = &mut w.mobys[id];
                    m.set_pv_v(P_VEL, set_len2(diff, speed));
                    ctx.hero.vel = set_len2(ctx.hero.vel, 0.0);
                    m.set_pv_f(P_RATE, r);
                    m.set_state(2, -1);
                }
            } else if ctx.game.frame & 0xf == id as i32 & 0xf && linked_hidden(w, id) {
                w.mobys[id].set_state(3, -1);
            }
        }
        2 => {
            if ctx.hero.state == 0x81 {
                let m = &w.mobys[id];
                let (vel, to) = (m.pv_v(P_VEL), m.pv_v(P_TARGET));
                let mut v = len2(vel);
                if dist2(ctx.hero.pos, to) < 0.5 {
                    v = 0.0;
                } else if v != 0.0 {
                    let mut d = sub(to, ctx.hero.pos);
                    d[2] = 0.0;
                    if dot(d, vel) < 0.0 { v = 0.0; }
                }
                v -= m.pv_f(P_RATE);
                let vel = set_len2(vel, v);
                w.mobys[id].set_pv_v(P_VEL, vel);
                let h = &mut *ctx.hero;
                h.vel = set_len2(h.vel, 0.0);
                h.push = vel;
                if v != 0.0 { h.target_yaw = atan2(vel[0], vel[1]); }
                h.damage.invincible = 10;
                if level == 0x11 && w.mobys[id].pv_i(P_ACTIVE) == 0 {
                    if let Some(i) = ctx.game.cam_tweaks.request(5) {
                        w.mobys[id].set_pv_i(P_TWEAK, i as i32 + 1);
                        w.mobys[id].set_pv_i(P_ACTIVE, 1);
                        let t = &mut ctx.game.cam_tweaks.slots[i];
                        t.dir = [-2.0, -3.0, -5.0];
                        t.dist = 4.0;
                        t.pivot_h = 1.0;
                        t.pitch = 0.0;
                        t.face_once = true;
                        t.no_stick = true;
                    }
                }
            } else {
                if w.mobys[id].pv_i(P_ACTIVE) != 0 {
                    let s = w.mobys[id].pv_i(P_TWEAK);
                    if 0 < s {
                        ctx.game.cam_tweaks.slots[s as usize - 1].no_stick = false;
                        ctx.game.cam_tweaks.release(s as usize - 1);
                    }
                    w.mobys[id].set_pv_i(P_ACTIVE, 0);
                }
                w.mobys[id].set_state(1, -1);
            }
        }
        3 => {
            let hidden = linked_hidden(w, id);
            let solid = w.class(PAD).is_some_and(|c| c.has_collision);
            let m = &mut w.mobys[id];
            if hidden && m.alpha != 0 {
                m.has_collision = false;
                m.alpha = if 3 < m.alpha { m.alpha - 4 } else { 0 };
            } else if !hidden {
                if m.alpha < 0x7c {
                    m.alpha += 4;
                } else {
                    m.alpha = 0x80;
                    m.set_state(1, -1);
                    m.has_collision = solid;
                }
            }
        }
        _ => {}
    }
    if w.mobys[id].drawn != 0 && w.mobys[id].state != 3 { streak(w, id, ctx); }
    if level == 0x10 {
        let d = sub(ctx.hero.pos, w.mobys[id].pos);
        if dot(d, d) < 9.0 { help_step(ctx, HELP_SLOT, HELP_TEXT); }
    }
    if w.mobys[id].state == 2 { ctx.game.help_counts[HELP_SLOT] = -1; }
    let m = &w.mobys[id];
    let h = m.pv_i(P_SOUND);
    if m.mode & 1 == 0 && m.has_collision {
        if h != -1 && ctx.host.sound_playing(h) { return; }
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(P_SOUND, h);
        return;
    }
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(P_SOUND, -1);
}

/// `0x40da18`: a streak from a random point of its top's triangle (`(−0.67, 0, 0)`, `(1.2, 1.3, 0)`, `(1.2, −1.3, 0)` in
/// its frame), rising 15..30 a second², 30..60 ticks, its colours `0x7080ff40` / `0x7000ff00`.
fn streak(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let rng = &mut ctx.game.rng;
    let fall = -(rng.randf(15.0, 30.0) * f32::from_bits(0x3991_a2b4));
    let life = rng.rand_range(0x1e, 0x3c);
    let mut k = [0.0f32; 3];
    for x in k.iter_mut() { *x = rng.randf(0.0, 1.0); }
    let inv = 1.0 / (k[0] + k[1] + k[2]);
    for x in k.iter_mut() { *x *= inv; }
    let tri: [V3; 3] = [[f32::from_bits(0xbf2b_851f), 0.0, 0.0], [f32::from_bits(0x3f99_999a), f32::from_bits(0x3fa6_6666), 0.0], [f32::from_bits(0x3f99_999a), f32::from_bits(0xbfa6_6666), 0.0]];
    let l: V3 = std::array::from_fn(|c| tri[0][c] * k[0] + tri[1][c] * k[1] + tri[2][c] * k[2]);
    let m = &w.mobys[id];
    let r = m.rows;
    let p: V3 = std::array::from_fn(|c| l[0] * r[0][c] + l[1] * r[1][c] + l[2] * r[2][c] + m.pos[c]);
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    if let Some(i) = crate::particles::streak(sys, fall, p, [0.0; 3], life as i16) {
        crate::particles::rec::set_u32(&mut sys.pool.recs[i], 4, 0x7080_ff40);
        crate::particles::rec::set_u32(&mut sys.pool.recs[i], 0xc, 0x7000_ff00);
    }
}

/// 0x09b9's pvars.
const Q_HEIGHT: usize = 0x00;
const Q_PHASE: usize = 0x04;
const Q_RADII: usize = 0x10;
const Q_PHASES: usize = 0x20;
const Q_RATES: usize = 0x30;
const Q_PVARS: usize = 0x40;
/// L22 `$gp+0xec8..` (L26 `$gp+0xbf8..`, the same values): the rings' rates (a second) and radii.
const RING_RATE: (f32, f32) = (1.5, 2.0);
const RING_RADIUS: (f32, f32) = (3.0, 4.0);

/// A ring's new rate and radius.
fn ring(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: usize) {
    let rng = &mut ctx.game.rng;
    let rate = rng.randf(RING_RATE.0, RING_RATE.1) * f32::from_bits(0x3c88_8889);
    let radius = rng.randf(RING_RADIUS.0, RING_RADIUS.1);
    let m = &mut w.mobys[id];
    m.set_pv_f(Q_RATES + 4 * k, rate);
    m.set_pv_f(Q_RADII + 4 * k, radius);
}

/// 0x09b9 (module docs).
pub fn update_09b9(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < Q_PVARS { w.mobys[id].pvars.resize(Q_PVARS, 0); }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_state(1, -1);
            m.draw_dist = 0xff;
            for k in 0..4 {
                w.mobys[id].set_pv_f(Q_PHASES + 4 * k, k as f32 * 0.25);
                ring(w, id, ctx, k);
            }
        }
        1 => {
            if dist(ctx.hero.pos, w.mobys[id].pos) < 3.0 { ctx.hero.bounce[0] = w.mobys[id].pv_f(Q_HEIGHT); }
            if ctx.hero.state == 0x81 && ctx.hero.timer == 1 { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            let slot = 31;
            if ctx.game.help_counts[slot] as u16 <= 0xfffe {
                if ctx.hero.state == 0x81 {
                    ctx.game.help_counts[slot] = -1;
                } else if ctx.hero.ground_moby == Some(id) {
                    help_step(ctx, slot, 0x9c8);
                }
            }
            if dot(w.mobys[id].rows[2], ctx.hero.gravity.down) < f32::from_bits(0xbf19_999a) { rings(w, id, ctx); }
        }
        _ => {}
    }
    if w.mobys[id].drawn != 0 {
        let m = &mut w.mobys[id];
        let p = add_rot(m.pv_f(Q_PHASE), 240.0 * f32::from_bits(0x3998_825c));
        m.set_pv_f(Q_PHASE, p);
        let f = p.sin();
        let f = f * f;
        let f = f * f;
        m.glow = crate::hud::lerp_rgba(f * f, 0x8050_5050, 0x80c0_c0c0);
    }
}

/// `0x3cafa8`: the rings stepped (a ring through starts again at 0 with a new rate and radius); one tick in two a
/// glint off its top, 0.1..0.75 out a random way, rising 1..4 a second, size 0.1..0.3, 30..90 ticks, `0x70002080` or
/// `0x7000ffff`.
fn rings(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for k in 0..4 {
        let m = &mut w.mobys[id];
        let t = m.pv_f(Q_PHASES + 4 * k) + m.pv_f(Q_RATES + 4 * k);
        m.set_pv_f(Q_PHASES + 4 * k, t);
        if 1.0 <= t {
            m.set_pv_f(Q_PHASES + 4 * k, 0.0);
            ring(w, id, ctx, k);
        }
    }
    let rng = &mut ctx.game.rng;
    if rng.randi(2) == 0 { return; }
    let up = rng.randf(1.0, 4.0) * f32::from_bits(0x3c88_8889);
    let r = w.mobys[id].rows;
    let turn = |l: V3| -> V3 { std::array::from_fn(|c| l[0] * r[0][c] + l[1] * r[1][c] + l[2] * r[2][c]) };
    let vel = turn([0.0, 0.0, up]);
    let a = rng.rand_angle();
    let s = rng.randf(f32::from_bits(0x3dcc_cccd), 0.75);
    let at = add(turn([a.cos() * s, a.sin() * s, 0.0]), w.mobys[id].pos);
    let life = rng.rand_range(0x1e, 0x5a);
    let size = rng.randf(f32::from_bits(0x3dcc_cccd), f32::from_bits(0x3e99_999a));
    let rgba = if rng.randi(2) != 0 { 0x7000_2080 } else { 0x7000_ffff };
    let rot = rng.randi(0xff);
    let hero = ctx.hero.pos;
    if let Some(sys) = ctx.parts.as_deref_mut() {
        crate::particles::glint(sys, size, at, vel, rgba, (life & 0xff) as u16, (rot & 0xff) as u8, false, hero);
    }
}
