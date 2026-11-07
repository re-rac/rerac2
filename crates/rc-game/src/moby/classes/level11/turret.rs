//! Joba's mounted turret **0x0c20** (`0x4134b0`, 4 placed; L11 addresses; the decomp's v1.01 `func_L11_00411D40`,
//! the same code): Ratchet climbs onto it and rides it in his state 0x31 (crate::hero::vehicle: the turret places him
//! each tick), turning it with the stick under the script camera and firing with ○; the thugs 0x0cf5 / 0x0cf6 man it
//! too (crate::moby::classes::level11::thugs, gunners: they write its state — 6 while one is on it, 4 when he
//! leaves —, its sequence, playback speed, angles, height and its aim record `+0x70`).
//!
//! Its pvars: `+0x10` a quaternion the gun thug writes (nothing here reads it), `+0x40` / `+0x48` / `+0x4c` cleared,
//! `+0x42` the wait before Ratchet may mount (s16), `+0x44` the pitch, `+0x52` the wait before the camera follows
//! (s16), `+0x54` the barrel that fired last (s16), `+0x5c` its rest height, `+0x60` / `+0x64` the yaw's and pitch's
//! speeds, `+0x68` its light (crate::point_lights; −1 none), `+0x6c` the light's ticks, `+0x70` its aim record
//! (crate::moby::joint_aim, list 2; the asked pitch `+0xb4`), `+0xf0` its turning sound. Its timer byte `+0xbc` is
//! the mount's command: 1 Ratchet getting on, 2 just got off.
//!
//! * **Every tick** (not in state 6, nor in a variant): mode bit 4 off; the seat its joint 3, 0.4 up. From state 4:
//!   with the command 1, the mount's wait out, Ratchet not in the groups 0x14 / 7 nor the state 0x39 and the prompt's
//!   lease free (`0x1a8fb4`), he is put on the seat at its angles, 0.4 back along his facing, his velocity cleared:
//!   △ gets him off (the script camera released, kind 0; idle `0x2d9208(0, 1)`; the command 2; 4; playback 1,
//!   sequence 0); else, first time, the camera's wait 2, the script camera in at him (`0x489db8`, mode 0), the
//!   sequence 4, then 5 at playback 2. With the command 2 his leaving (more than 1.2 from the seat or off it) clears
//!   it and the mount waits 60 ticks. Else, Ratchet on it within 1.2 of the seat (the same tests): the command 1, his
//!   state 0x31 (`0x2d9208(0x31, 1)`).
//! * **0**: 4, its rest height, pitch and yaw 0, no light.
//! * **4**: Ratchet within 4 across: it sinks toward 1 under its rest (×0.11 a tick) and, while he is not on it,
//!   turns its back to him (its speed: (the speed + the turn ×0.01) ×0.9); a light (radius 0.4, red 4) at
//!   (0, −0.3, −0.1) in its frame from the seat, its sound 2 when it comes, blinking (out on its ticks' bit 2 between
//!   5 and 40, but on at 25.. with bits 1 and 3). Farther: back up to its rest by 1/60 a tick; the light gone (sound 0)
//!   once he is off it.
//! * **5** (ridden): up to its rest; the stick (its y as the Camera page's "up/down reversed" leaves it; length at
//!   most 1) turns it — the yaw speed by −x·0.05° a tick (0 with no x), the pitch by −y·0.018°, both ×0.97 a tick, the
//!   pitch within ±30° —, its sound 3 looped while it turns; the aim record's pitch. ○ held: its press the barrel's
//!   sequence (3 after barrel 0, else 2); in sequence 2 / 3 at frame 1 (±0.25) a shot from joint 0 / 1 along its yaw
//!   and pitch (`0x451d60`: the turret's shot 0x0dd9, 6 damage, crate::moby::classes::level11::thugs::shot), its
//!   sound 1, the camera's shakes (0.03 for 10 and 7 ticks), and at the sequence's end the other barrel's; ○ let go,
//!   at the sequence's end 4. After the camera's wait: its matrix built (mode | 4), the script camera snapping
//!   (`0x48a068`) to joint 2 + 0.5 along the aim, 0.2 up, at (0, −pitch, yaw); the crosshair (`0x2ed298`: FX 0xd,
//!   `0xff0f0fff`, size 1).
//!
//! Not ported: the shot's muzzle sparks (`0x34a770` = L04 `0x359dc8`: the particle type 0x1b, which the port has
//! not), the sound table's owner word (`0x188660 + 0x6c`); the Camera page's pitch option (`0x1a7bac`) is not in the
//! moby context, so the stick's y is taken as the default (reversed).

use crate::hero::physics::{add, add_rot, atan2, dist, dist2, sub_rot, V3};
use crate::moby::{countdown, Ctx, MobyWorld};
use crate::pad::button;
use crate::point_lights::PointLight;
use super::super::props::wrapped;
use super::super::leaper::seq_a;

pub const CLASS: i16 = 0x0c20;

const MOUNT_WAIT: usize = 0x42;
const PITCH: usize = 0x44;
const CAM_WAIT: usize = 0x52;
const BARREL: usize = 0x54;
const REST: usize = 0x5c;
const YAW_V: usize = 0x60;
const PITCH_V: usize = 0x64;
const LIGHT: usize = 0x68;
const LIGHT_T: usize = 0x6c;
const AIM: usize = 0x70;
const SOUND: usize = 0xf0;
const PVARS: usize = 0x100;

/// `$gp+0x1c90` (`0x1b0c80`): the light's place in its frame from the seat; `$gp+0x1c80` / `+0x1c84`: the camera's
/// reach along the aim and its lift.
const LIGHT_AT: V3 = [0.0, -0.3, -0.1];
const CAM_REACH: f32 = 0.5;
const CAM_LIFT: f32 = 0.2;

/// `0x300e80` on an s16 word: down to 0; true when it is (or just got) 0.
fn wait_out(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let m = &mut w.mobys[id];
    let mut v = m.pv_s(off) as i32;
    let r = countdown(&mut v);
    m.set_pv_s(off, v as i16);
    r
}

/// A point in the frame `r` (`0x3016d8`).
fn to_world(l: V3, r: &[V3; 3]) -> V3 { std::array::from_fn(|k| l[0] * r[0][k] + l[1] * r[1][k] + l[2] * r[2][k]) }

/// `0x337298(&out, len, yaw, pitch)`.
fn along(len: f32, yaw: f32, pitch: f32) -> V3 { [yaw.cos() * len * pitch.cos(), yaw.sin() * len * pitch.cos(), pitch.sin() * len] }

/// Ratchet free to mount (not in the groups 0x14 / 7 nor the state 0x39, the prompt's lease free).
fn hero_free(ctx: &Ctx) -> bool { ctx.hero.group != 0x14 && ctx.hero.group != 7 && ctx.hero.state != 0x39 && ctx.game.prompt.owner == 0 }

/// 0x0c20 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.variant != 0 || w.mobys[id].state == 6 { return; }
    {
        let m = &mut w.mobys[id];
        if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
        m.mode &= !4;
    }
    let mut seat = ctx.host.joint(&w.mobys[id], 3).unwrap_or(w.mobys[id].pos);
    seat[2] += 0.4;
    let on = ctx.hero.ground_moby == Some(id);
    if w.mobys[id].state >= 4 {
        let cmd = w.mobys[id].timer;
        if wait_out(w, id, MOUNT_WAIT) && cmd == 1 && hero_free(ctx) {
            let rot = w.mobys[id].rot;
            let y = rot[2];
            let at = add(seat, [y.cos() * -0.4, y.sin() * -0.4, 0.0]);
            ctx.hero.ride_place(at, rot);
            if ctx.hero.items.pressed & button::TRIANGLE != 0 {
                ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_CUT_BEHIND, ctx.hero);
                ctx.hero.request_state(0, true);
                let m = &mut w.mobys[id];
                m.timer = 2;
                m.state = 4;
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                ctx.host.set_anim(&w.mobys[id], 0, 0, 1);
            } else {
                if w.mobys[id].state != 5 {
                    w.mobys[id].set_pv_s(CAM_WAIT, 2);
                    let (p, r) = (ctx.hero.pos, ctx.hero.rot);
                    ctx.game.cameras.camera_script(p, r, crate::script_camera::MODE_SPRING, 0, false, ctx.hero);
                    super::thugs::anim(w, id, ctx, 4, 0, 5);
                }
                w.mobys[id].state = 5;
                ctx.host.set_anim_speed(&w.mobys[id], 2.0);
            }
        } else if cmd == 2 {
            if 1.2 < dist(seat, ctx.hero.pos) || !on {
                let m = &mut w.mobys[id];
                m.timer = 0;
                m.set_pv_s(MOUNT_WAIT, 60);
            }
        } else if hero_free(ctx) && on && dist(seat, ctx.hero.pos) < 1.2 && w.mobys[id].pv_s(MOUNT_WAIT) == 0 {
            w.mobys[id].timer = 1;
            ctx.hero.request_state(0x31, true);
        }
    }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.state = 4;
            m.set_pv_f(REST, m.pos[2]);
            m.set_pv_f(PITCH, 0.0);
            m.rot[2] = 0.0;
            m.set_pv_i(LIGHT, -1);
            m.set_pv_s(MOUNT_WAIT, 0);
            m.set_pv_i(0x48, 0);
            m.set_pv_i(0x4c, 0);
            m.set_pv_s(0x40, 0);
        }
        4 => waiting(w, id, ctx, seat, on),
        5 => ridden(w, id, ctx),
        _ => {}
    }
}

/// State 4 (module docs).
fn waiting(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, seat: V3, on: bool) {
    if dist2(w.mobys[id].pos, ctx.hero.pos) < 4.0 {
        let m = &mut w.mobys[id];
        let rest = m.pv_f(REST);
        m.pos[2] += ((rest - 1.0) - m.pos[2]) * 0.11;
        if !on {
            let to = atan2(ctx.hero.pos[0] - m.pos[0], ctx.hero.pos[1] - m.pos[1]);
            let turn = add_rot(sub_rot(to, m.rot[2]), std::f32::consts::PI) * 0.01;
            let v = add_rot(m.pv_f(YAW_V), turn) * 0.9;
            m.set_pv_f(YAW_V, v);
            m.rot[2] = add_rot(m.rot[2], v);
        } else {
            m.set_pv_f(YAW_V, 0.0);
        }
        let p = add(to_world(LIGHT_AT, &m.rows), seat);
        let slot = m.pv_i(LIGHT);
        if slot == -1 {
            m.set_pv_i(LIGHT_T, 0);
            let s = w.point_lights.alloc(PointLight { color: [4.0, 4.0, 4.0], intensity: 0.0, pos: p, radius: 0.4 }).map_or(-1, |s| s as i32);
            w.mobys[id].set_pv_i(LIGHT, s);
            ctx.host.play_sound(&w.mobys[id], 2, 0);
        } else {
            let n = m.pv_i(LIGHT_T) + 1;
            m.set_pv_i(LIGHT_T, n);
            if let Some(Some(l)) = w.point_lights.slots.get_mut(slot as usize) {
                l.pos = p;
                if 40 < n || n & 4 == 0 || (25 <= n && n & 0xa == 0xa) {
                    l.color[0] = 4.0;
                    l.radius = 0.4;
                } else {
                    l.radius = 0.0;
                }
            }
        }
    } else {
        let m = &mut w.mobys[id];
        let rest = m.pv_f(REST);
        if m.pos[2] < rest { m.pos[2] += 0.016_666_668 } else { m.pos[2] = rest }
        let slot = m.pv_i(LIGHT);
        if !on && slot != -1 {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.point_lights.free(slot as usize);
            w.mobys[id].set_pv_i(LIGHT, -1);
        }
    }
    w.changed = true;
}

/// A shot from the barrel joint `j` (module docs): `0x451d60`, its sound 1, the shakes; the barrel kept.
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, j: usize) {
    let (yaw, pitch) = (w.mobys[id].rot[2], w.mobys[id].pv_f(PITCH));
    let dir = along(1.0, yaw, pitch);
    let Some(at) = ctx.host.joint(&w.mobys[id], j) else { return };
    super::thugs::shot(w, ctx, 6.0, id, dir, at);
    ctx.host.play_sound(&w.mobys[id], 1, 0);
    ctx.game.shakes.ask(0, 0.03, 10);
    ctx.game.shakes.ask(1, 0.03, 7);
    w.mobys[id].set_pv_s(BARREL, j as i16);
}

/// State 5 (module docs).
fn ridden(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let rest = m.pv_f(REST);
    if m.pos[2] < rest { m.pos[2] += 0.016_666_668 } else { m.pos[2] = rest }
    let (mut x, mut y) = (ctx.hero.stick[0], ctx.hero.stick[1]);
    let l = (x * x + y * y).sqrt();
    if 1.0 < l {
        x /= l;
        y /= l;
    }
    let mut yv = m.pv_f(YAW_V) + -x * 0.000_872_664_7;
    if x == 0.0 { yv = 0.0; }
    let snd = m.pv_i(SOUND);
    if yv != 0.0 {
        if !ctx.host.sound_playing(snd) {
            let h = ctx.host.play_sound(&w.mobys[id], 3, 4);
            w.mobys[id].set_pv_i(SOUND, h);
        }
    } else {
        if snd != -1 && ctx.host.sound_playing(snd) { ctx.host.stop_sound(snd); }
        w.mobys[id].set_pv_i(SOUND, -1);
    }
    let m = &mut w.mobys[id];
    let mut pv = m.pv_f(PITCH_V) + -y * 0.000_315_128_94;
    if y == 0.0 { pv = 0.0; }
    m.rot[2] = add_rot(m.rot[2], yv);
    let mut p = add_rot(m.pv_f(PITCH), pv);
    if p < -0.523_598_85 {
        p = -0.523_598_85;
        pv = 0.0;
    } else if 0.523_598_85 < p {
        p = 0.523_598_85;
        pv = 0.0;
    }
    m.set_pv_f(PITCH, p);
    m.set_pv_f(YAW_V, yv * 0.97);
    m.set_pv_f(PITCH_V, pv * 0.97);
    m.set_pv_f(AIM + 0x44, -p);
    crate::moby::joint_aim::aim(w, id, AIM, id, 2, 0.03, 0.3);
    let (held, pressed) = (ctx.hero.items.held, ctx.hero.items.pressed);
    if held & button::CIRCLE != 0 {
        let s = seq_a(w, id, ctx);
        if pressed & button::CIRCLE != 0 {
            let b = if w.mobys[id].pv_s(BARREL) == 0 { 3 } else { 2 };
            ctx.host.set_anim(&w.mobys[id], b, 0, 1);
        } else if s == 2 || s == 3 {
            if (ctx.host.frame_time(&w.mobys[id]) - 1.0).abs() <= 0.25 {
                fire(w, id, ctx, if s == 2 { 0 } else { 1 });
            } else if wrapped(w, id, ctx) {
                ctx.host.set_anim(&w.mobys[id], if s == 2 { 3 } else { 2 }, 0, 1);
            }
        }
    } else if super::super::props::seq(w, id, ctx) != 4 && wrapped(w, id, ctx) {
        ctx.host.set_anim(&w.mobys[id], 4, 0, 1);
    }
    if wait_out(w, id, CAM_WAIT) {
        w.rebuild(id, &*ctx.host);
        w.mobys[id].mode |= 4;
        if ctx.game.cameras.script_current() { ctx.game.cameras.script.mode = crate::script_camera::MODE_SNAP; }
        let (yaw, pitch) = (w.mobys[id].rot[2], w.mobys[id].pv_f(PITCH));
        let base = ctx.host.joint(&w.mobys[id], 2).unwrap_or(w.mobys[id].pos);
        let mut v = along(CAM_REACH, yaw, pitch);
        v[2] += CAM_LIFT;
        ctx.game.cameras.camera_script_targets(Some(add(base, v)), Some([0.0, -pitch, yaw]));
    }
    crate::hud::marker(&mut ctx.game.markers, 1.0, id, 0xff0f_0fff, None, 0xd);
    w.changed = true;
}
