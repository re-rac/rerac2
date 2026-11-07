//! Snivelak's mounted guns **0x10ee** ([`update_10ee`], L16 `0x4179f0`, 14 placed; no function in Ghidra, read from
//! its bytes; the decomp's `func_US2_L16_004179E8`): Ratchet steps onto one and rides it in his state 0x31
//! (crate::hero::vehicle), turning it with the stick under the script camera. L16 addresses.
//!
//! Its pvars: `+0x60` a moby (−1: the gun is deleted at its set-up), `+0x64` the moby it first turns to, `+0x70` its
//! home, `+0x78` its raised height, `+0x7c` its first yaw, `+0x80` / `+0x84` the yaw's and pitch's speeds, `+0x88` the
//! barrel that fires next, `+0x8c` the fire's rest, `+0x90` the pitch, `+0x94` its health (4), `+0x98` / `+0x9c` the
//! height's and yaw's easing speeds, `+0xa4` its turning sound.
//!
//! * **0**: put on the ground under it (`0x31ede0`: the line from 0.5 up, flags 2); a target (mode | 0x4000); its home
//!   and first yaw kept, its health 4; 1. With Ratchet dead twice or more in the fight (`$gp+0x26b4`,
//!   crate::moby::classes::level16::boss::Globals::deaths) and the halfword `0x1941dc` under 3 (nothing in the code
//!   writes it: taken as 0, as level 11's `0x194158`), the help text 0x0a0b (slot 0x62).
//! * **1** / **4** (4: just left): its sound 3 on the state's first tick. Back to 1 once Ratchet is off it (4) or 20
//!   ticks in the air. In 1, Ratchet on the ground on it: he is put at (−0.666, 0, 1.4) in its frame at its angles
//!   in state 0x31 (`0x2b9680(…, 0x31, 0)`), `0x18b650` it, the script camera swung in from the view over 60 ticks
//!   (`0x434970`, mode 3), the help slot 0x62 off (`0x18cdf0` = 0xffff, also every tick of 2), `$gp+0x2670` it; 2. Else it sinks toward 1 under its raised height (`0x3185c0`: 1/1800 a
//!   tick², at most 1/30 a tick) and, with Ratchet more than 3 away across, turns its back to him (`0x318a78`: 0.1°
//!   a tick², at most 6° a tick).
//! * **2** (rising): on its first tick its sound 3 and the pitch 0; it rises to its raised height (1/600 a tick², at
//!   most 0.1) turning toward the moby `+0x64` (0.067° a tick², at most 6°; its sound 4 looped while more than 10°
//!   off); at its height and within 5°: 3. Ratchet placed, its fire ([`fire`]), the camera led ([`hold`]).
//! * **3** (steered): the crosshair (`0x2d4770`: the marker 0x26 at the screen's centre, size 1, `0x600fff0f`). △ or
//!   Ratchet out of 0x31: off ([`get_off`], `0x4182c0`), 4, its turning sound stopped, the prompt's lease taken for
//!   30 ticks (owner 2). Else [`steer`], [`fire`] and [`hold`].
//! * **Its hits**: from the class 0x12be its health −1; from the classes 0x0f79 / 0x09b0, or at no health: gone —
//!   hidden, no collision, not a target, off if ridden, 6. Else from anything but 0x0ea7 while it is steered and
//!   Ratchet not kept from harm: its damage to Ratchet (`0x2a6750`), off and 4 at no health left, else kept from
//!   harm 60 ticks; his health shown (`0x3071c0(360)`).
//!
//! **The steer** ([`steer`], `0x418648`): the stick (`0x18bdd0`) filtered ($gp+0x2b00 / `0x1b1af4`: first held to
//! within the stick's size, then moved 0.15 a tick toward it); each axis's speed moves toward its target (the stick
//! full: 360°/s at 360°/s², else 90°/s by the filter at 90°/s² by it; 2000°/s² back down when the target is
//! smaller), the x turning the yaw and the y the pitch, kept in −30°..20° (`0x318f08`: the speed held to what can stop
//! by the limit); its sound 4 looped while it turns.
//!
//! **The hold** ([`hold`]): Ratchet at (−0.666, 0, 1.4) in its frame at its yaw (`0x418268`); the script camera's
//! targets (`0x418300`): its angles with the pitch (and 0.1 rad more down), 4.5 behind and 4 above its middle in
//! that frame, at least 1 above it.
//!
//! **The fire** ([`fire`], `0x4183e8`): once its rest `+0x8c` is out (`0x2e7e58` = L00 `0x2eeab0`) and with □ or ○
//! held (`0x138320 & 0xa0`), a blaster shot (`0x3fdb70` = L13 `0x40f0a0`, crate::moby::classes::thugs::fire, the
//! class 0x0ea7 the boss takes its full damage from) from the barrel `+0x88` (`$gp+0x2a80`: (2, ∓0.75, 2.5) in its
//! frame), at 75 a second, damage 1 (×5 in the challenge mode, `0x1a7a12`). Its way: with its own collision off
//! (`+0x98` = 0; back to its class's after), a moby within 1.5 of (2, 0, 2) in its frame but Ratchet (`0x2da788` = L00
//! `0x2e14b0`, its first) — toward that moby's place; else the camera's line from 2 to 120 along its forward
//! (`0x2d7b50` = L00 `0x2de878`, flags 0, Ratchet skipped) — toward where it meets; else toward its far end. Then the
//! rest 8 ticks (`$gp+0x2ab8`) and the other barrel.
//!
//! Not ported:
//! * Its platform record (`+0x20`, `0x31bc70`: what Ratchet standing on it is carried by).
//! * Its break (`0x418140`: five pieces, its sound 0 and the shatter `0x3e9ff0`).
//! * `0x31ede0`'s gravity-frame branch (`0x1a8ca0`).

use crate::hero::physics::{add, add_rot, atan2, dist2, ease, scale, set_len, sub, sub_rot, rot_dist, turn_ease, approach, V3};
use crate::moby::attach::rows_of;
use crate::moby::{keep_loop, stop_loop, Ctx, MobyWorld};
use crate::pad::button;
use std::f32::consts::PI;
use crate::gravity::to_world;

pub const GUN: i16 = 0x10ee;

mod pv {
    pub const LINK: usize = 0x60;
    pub const TARGET: usize = 0x64;
    pub const HOME: usize = 0x70;
    pub const HIGH: usize = 0x78;
    pub const YAW0: usize = 0x7c;
    pub const YAW_V: usize = 0x80;
    pub const PITCH_V: usize = 0x84;
    pub const BARREL: usize = 0x88;
    pub const REST: usize = 0x8c;
    pub const PITCH: usize = 0x90;
    pub const HEALTH: usize = 0x94;
    pub const Z_V: usize = 0x98;
    pub const TURN_V: usize = 0x9c;
    pub const SOUND: usize = 0xa4;
}

/// `$gp+0x2a30`: Ratchet's place in its frame.
const SEAT: V3 = [f32::from_bits(0xbf2a_7efa), 0.0, f32::from_bits(0x3fb3_3333)];
/// The help text's slot (`0x18c958 + 0xc·0x62` = `0x18cdf0`).
const HELP_SLOT: i32 = 0x62;
/// `$gp+0x2a80` / `$gp+0x2aa0`: the barrels in its frame; the point (and radius, its w) it looks about for a moby.
const BARRELS: [V3; 2] = [[2.0, -0.75, 2.5], [2.0, 0.75, 2.5]];
const AIM: V3 = [2.0, 0.0, 2.0];
const AIM_R: f32 = 1.5;
/// `$gp+0x2ab0` / `$gp+0x2ab4` / `$gp+0x2ab8`: the shot's speed (a second), its damage, the rest between shots.
const SHOT_SPEED: f32 = 75.0;
const SHOT_DAMAGE: f32 = 1.0;
const REST: i32 = 8;
/// `$gp+0x2a40` / `$gp+0x2a44`: how far under its raised height it waits, and the distance it turns its back past.
const LOW: f32 = 1.0;
const FAR: f32 = 3.0;
/// `$gp+0x2a50` / `$gp+0x2a60` / `$gp+0x2a70` / `$gp+0x2a74`: the camera's offset, its look point, its pitch, its
/// least height above the gun.
const CAM_OFF: V3 = [-4.5, 0.0, 0.0];
const CAM_LOOK: V3 = [0.0, 0.0, 4.0];
const CAM_PITCH: f32 = f32::from_bits(0xbdcc_cccd);
const CAM_ABOVE: f32 = 1.0;
/// `$gp+0x2ac0..+0x2ae8`, `0x1b1ab4..` (degrees a second, a second²): the yaw's and the pitch's (slow accel, fast
/// accel, slow speed, fast speed, brake).
const YAW_RATES: [f32; 5] = [90.0, 360.0, 90.0, 360.0, 2000.0];
const PITCH_RATES: [f32; 5] = [90.0, 360.0, 90.0, 360.0, 2000.0];
/// `$gp+0x2af0` / `0x1b1ae4`: the pitch's limits (degrees).
const PITCH_LIMITS: (f32, f32) = (20.0, -30.0);
/// Degrees a second² as radians a tick²; degrees a second as radians a tick.
const DEG_TICK2: f32 = 4.848_137_5e-6;
const DEG_TICK: f32 = 0.000_290_888_25;

pub fn update_10ee(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if w.mobys[id].pv_i(pv::LINK) == -1 { return w.delete(id); }
            let p = w.mobys[id].pos;
            if let Some(o) = w.coll_line(ctx, [p[0], p[1], p[2] + 0.5], [p[0], p[1], 0.01], 2, Some(id)) { w.mobys[id].pos = o.point; }
            let m = &mut w.mobys[id];
            m.mode |= 0x4000;
            m.set_pv_v(pv::HOME, m.pos);
            m.set_pv_f(pv::YAW0, m.rot[2]);
            m.set_pv_f(pv::HEALTH, 4.0);
            m.set_state(1, -1);
            if 2 <= w.snivelak.deaths { ctx.game.help(0x0a0b, HELP_SLOT); }
        }
        1 | 4 => wait(w, id, ctx),
        2 => {
            if w.mobys[id].first() {
                ctx.host.play_sound(&w.mobys[id], 3, 0);
                w.mobys[id].set_pv_f(pv::PITCH, 0.0);
            }
            let to = usize::try_from(w.mobys[id].pv_i(pv::TARGET)).ok().and_then(|t| w.mobys.get(t)).map(|t| t.pos);
            let m = &mut w.mobys[id];
            let angle = to.map_or(m.rot[2], |t| atan2(t[0] - m.pos[0], t[1] - m.pos[1]));
            let (high, mut z, mut zv, mut yaw, mut yv) = (m.pv_f(pv::HIGH), m.pos[2], m.pv_f(pv::Z_V), m.rot[2], m.pv_f(pv::TURN_V));
            ease(high, f32::from_bits(0x3ada_740e), f32::from_bits(0x3ada_740e), f32::from_bits(0x3dcc_ccce), &mut z, &mut zv);
            turn_ease(angle, f32::from_bits(0x3a98_825c), f32::from_bits(0x3a98_825c), f32::from_bits(0x3dd6_7752), &mut yaw, &mut yv, 0);
            (m.pos[2], m.rot[2]) = (z, yaw);
            m.set_pv_f(pv::Z_V, zv);
            m.set_pv_f(pv::TURN_V, yv);
            if sub_rot(yaw, angle).abs() < f32::from_bits(0x3e32_b8c4) { stop_loop(w, id, ctx, pv::SOUND) } else { keep_loop(w, id, ctx, pv::SOUND, 4) }
            if (z - high).abs() <= zv && rot_dist(yaw, angle) <= f32::from_bits(0x3db2_b8c4) { w.mobys[id].set_state(3, -1); }
            help_off(ctx);
            fire(w, id, ctx);
            hold(w, id, ctx);
        }
        3 => {
            crate::hud::marker(&mut ctx.game.markers, 1.0, id, 0x600f_ff0f, None, 0x26);
            if ctx.hero.items.pressed & button::TRIANGLE != 0 || ctx.hero.state != 0x31 {
                get_off(ctx);
                w.mobys[id].set_state(4, -1);
                stop_loop(w, id, ctx, pv::SOUND);
                ctx.game.prompt.owner = 2;
                ctx.game.prompt.hold = 30;
            } else {
                steer(w, id, ctx);
                fire(w, id, ctx);
                hold(w, id, ctx);
            }
        }
        _ => {}
    }
    hits(w, id, ctx);
}

/// States 1 / 4 (module docs).
fn wait(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 3, 0); }
    let state = w.mobys[id].state;
    let on = ctx.hero.ground_moby == Some(id);
    if (state == 4 && !on) || 20 <= ctx.hero.air_ticks {
        w.mobys[id].state = 1;
    } else if state == 1 && on && ctx.hero.grounded_ticks != 0 {
        let m = &w.mobys[id];
        let at = add(to_world(SEAT, &m.rows), m.pos);
        let rot = m.rot;
        ctx.hero.ride_on(id, at, rot, true);
        let (view, euler) = ctx.game.cameras.view_euler();
        ctx.game.cameras.camera_script(view, euler, crate::script_camera::MODE_SWING, 60, false, ctx.hero);
        help_off(ctx);
        w.mobys[id].set_state(2, -1);
        return;
    }
    let m = &mut w.mobys[id];
    let high = m.pv_f(pv::HIGH);
    if 0.01 < (m.pos[2] - high + LOW).abs() {
        let (mut z, mut v) = (m.pos[2], m.pv_f(pv::Z_V));
        ease(high - LOW, f32::from_bits(0x3a11_a2b4), f32::from_bits(0x3a11_a2b4), f32::from_bits(0x3d08_8889), &mut z, &mut v);
        m.pos[2] = z;
        m.set_pv_f(pv::Z_V, v);
    }
    if FAR < dist2(m.pos, ctx.hero.pos) {
        let to = add_rot(atan2(ctx.hero.pos[0] - m.pos[0], ctx.hero.pos[1] - m.pos[1]), PI);
        let (mut yaw, mut v) = (m.rot[2], m.pv_f(pv::TURN_V));
        turn_ease(to, f32::from_bits(0x3ae4_c38a), f32::from_bits(0x3ae4_c38a), f32::from_bits(0x3dd6_7752), &mut yaw, &mut v, 0);
        m.rot[2] = yaw;
        m.set_pv_f(pv::TURN_V, v);
    }
}

/// `0x418268` and `0x418300` (module docs).
fn hold(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let at = add(to_world(SEAT, &m.rows), m.pos);
    let rot = [ctx.hero.rot[0], ctx.hero.rot[1], m.rot[2]];
    ctx.hero.ride_place(at, rot);
    let mut ang = m.rot;
    ang[1] = add_rot(add_rot(ang[1], CAM_PITCH), m.pv_f(pv::PITCH));
    let look = add(to_world(CAM_LOOK, &m.rows), m.pos);
    let mut pos = add(to_world(CAM_OFF, &rows_of(ang)), look);
    if pos[2] < m.pos[2] + CAM_ABOVE { pos[2] = m.pos[2] + CAM_ABOVE; }
    ctx.game.cameras.camera_script_targets(Some(pos), Some(ang));
}

/// `0x18cdf0` = 0xffff: the help slot 0x62's count −1, so its text is never asked for again.
fn help_off(ctx: &mut Ctx) {
    if let Some(c) = ctx.game.help_counts.get_mut(HELP_SLOT as usize) { *c = -1; }
}

/// `0x4183e8` (module docs).
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut rest = w.mobys[id].pv_i(pv::REST);
    let out = crate::moby::countdown(&mut rest);
    w.mobys[id].set_pv_i(pv::REST, rest);
    if !out || ctx.hero.items.held & (button::SQUARE | button::CIRCLE) == 0 { return; }
    let m = &w.mobys[id];
    let barrel = m.pv_i(pv::BARREL);
    let muzzle = add(to_world(BARRELS[(barrel & 1) as usize], &m.rows), m.pos);
    let aim = add(to_world(AIM, &m.rows), m.pos);
    let (cam, fwd) = (ctx.host.camera_pos(), ctx.host.camera_rows()[0]);
    let near = add(scale(fwd, 2.0), cam);
    let far = add(scale(fwd, 120.0), cam);
    let had = w.mobys[id].has_collision;
    w.mobys[id].has_collision = false;
    let found = ctx.scene.and_then(|sc| {
        crate::collision::coll_sphere_mobys(sc, aim, AIM_R, crate::collision::QueryFlags(0), ctx.hero_moby).into_iter().find(|&k| k != id)
    });
    let to = match found.and_then(|k| w.mobys.get(k)) {
        Some(k) => k.pos,
        None => w.coll_line(ctx, near, far, 0, ctx.hero_moby).map_or(far, |o| o.point),
    };
    let vel = set_len(sub(to, muzzle), SHOT_SPEED * f32::from_bits(0x3c88_8889));
    w.mobys[id].has_collision = had;
    let damage = if ctx.game.challenge { SHOT_DAMAGE * 5.0 } else { SHOT_DAMAGE };
    crate::moby::classes::thugs::fire(w, ctx, id, vel, muzzle, damage);
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::REST, REST);
    m.set_pv_i(pv::BARREL, (barrel + 1) & 1);
}

/// `0x4182c0`: `0x18b650` and `$gp+0x2670` cleared, Ratchet idle (`0x2c45f8(0)`: with no body), the script camera let
/// go (`0x434ae0(3)`).
fn get_off(ctx: &mut Ctx) {
    ctx.hero.ride_off();
    ctx.game.cameras.camera_script2(3, ctx.hero);
}

/// `0x318f08(&v, gap, brake)`: a speed toward the gap held to what the brake stops within it.
fn limit(v: &mut f32, gap: f32, brake: f32) {
    if *v * gap <= 0.0 || gap == 0.0 { return; }
    if gap.abs() >= *v * *v / brake * 0.5 { return; }
    let r = (2.0 * brake * gap).abs().sqrt();
    if r < *v {
        *v = r;
    } else if *v < -r {
        *v = -r;
    }
}

/// One axis's speed toward the filtered stick `f` (module docs).
fn axis(f: f32, rates: &[f32; 5], v: &mut f32) {
    let brake = rates[4] * DEG_TICK2;
    let (acc, target) = if f == 0.0 {
        (0.0, 0.0)
    } else if 1.0 <= f.abs() {
        (rates[1] * DEG_TICK2, rates[3] * (-f * DEG_TICK))
    } else {
        (rates[0] * (f.abs() * DEG_TICK2), rates[2] * (-f * DEG_TICK))
    };
    let step = if target.abs() < v.abs() { brake } else { acc };
    approach(target, step, v);
}

/// `0x418648` (module docs).
fn steer(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = ctx.hero.stick;
    let f = &mut ctx.hero.vehicle.stick_filter;
    for k in 0..2 {
        let a = s[k].abs();
        if a < f[k] { f[k] = a } else if f[k] < -a { f[k] = -a }
        approach(s[k], 0.15, &mut f[k]);
    }
    let f = *f;
    let m = &mut w.mobys[id];
    let (mut yv, mut pvv) = (m.pv_f(pv::YAW_V), m.pv_f(pv::PITCH_V));
    axis(f[0], &YAW_RATES, &mut yv);
    axis(f[1], &PITCH_RATES, &mut pvv);
    if yv != 0.0 { m.rot[2] = add_rot(m.rot[2], yv); }
    if pvv != 0.0 {
        let brake = PITCH_RATES[4] * DEG_TICK2;
        let (high, low) = (PITCH_LIMITS.0 * 0.017_453_294, PITCH_LIMITS.1 * 0.017_453_294);
        let mut p = m.pv_f(pv::PITCH);
        limit(&mut pvv, high - p, brake);
        limit(&mut pvv, low - p, brake);
        p = add_rot(p, pvv);
        if high < p {
            p = high;
            pvv = 0.0;
        } else if p < low {
            p = low;
            pvv = 0.0;
        }
        m.set_pv_f(pv::PITCH, p);
    }
    m.set_pv_f(pv::YAW_V, yv);
    m.set_pv_f(pv::PITCH_V, pvv);
    if 0.0 < yv.abs() || 0.0 < pvv.abs() { keep_loop(w, id, ctx, pv::SOUND, 4) } else { stop_loop(w, id, ctx, pv::SOUND) }
}

/// Its hits (module docs).
fn hits(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(hit) = w.hit_message(id) else { return };
    let class = hit.tmpl.source.and_then(|s| w.mobys.get(s)).map(|s| s.o_class);
    let state = w.mobys[id].state;
    if class == Some(0x12be) {
        let mut hp = w.mobys[id].pv_f(pv::HEALTH);
        approach(0.0, 1.0, &mut hp);
        w.mobys[id].set_pv_f(pv::HEALTH, hp);
    }
    let gone = class.is_some() && (matches!(class, Some(0x0f79 | 0x09b0)) || w.mobys[id].pv_f(pv::HEALTH) == 0.0);
    if gone {
        let m = &mut w.mobys[id];
        m.mode = (m.mode | 0x41) & 0xefff;
        m.has_collision = false;
        if state == 2 || state == 3 { get_off(ctx); }
        w.mobys[id].set_state(6, -1);
    } else if class != Some(0x0ea7) && state == 3 && ctx.hero.state == 0x31 && ctx.hero.damage.invincible == 0 {
        ctx.hero.take_damage(hit.tmpl.damage as i32);
        if ctx.hero.hp == 0 {
            get_off(ctx);
            w.mobys[id].set_state(4, -1);
        } else {
            ctx.hero.damage.invincible = 60;
        }
        ctx.hero.show_health = Some(360);
    }
    w.clear_hit(id);
}
