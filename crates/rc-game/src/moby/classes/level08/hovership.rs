//! Tabora's and Grelbin's hovership **0x0eef** ([`update_0eef`], L08 `0x4201e0`, L19 `0x414848`: the same code and
//! data; the decomp's `func_US2_L08_004201E0`) and the raritanium it vacuums up **0x099a** ([`update_099a`], L08
//! `0x3d55e8`, L19 `0x3c1cc8`; the decomp's `func_L08_003D4190`). Ratchet rides it in his state 0x31
//! (crate::hero::vehicle). L08 addresses below.
//!
//! **The hovership** (its pvars: `+0x00` the velocity, `+0x20` the roll's speed, `+0x24` the pitch's, `+0x28` the
//! yaw, `+0x30` its home, `+0x48` / `+0x4c` the bob's phase and height, `+0x50` / `+0x54` the sway's phases, `+0x58`
//! its height before the bob, `+0x5c` the throttle, `+0x60` its health, `+0x68` / `+0x6c` the nearest raritanium's
//! distance and moby (written by 0x099a), `+0x78` the engine's power (0..1), `+0x7c` a timer, `+0x8b` the hit's
//! rest, `+0x90` its flash, `+0xa0` the last moby that hit it, `+0x170` / `+0x180` the three rotors' angles and
//! speeds, `+0x17c` the height it is lost under, `+0x194` / `+0x19c` the music asked for on the way in and `+0x198`
//! / `+0x1a0` on the way out, `+0x1a4` its engine / vacuum sound, `+0x1a8` / `+0x1ac` the vacuum's sound state and
//! rest, `+0x1b4` its idle sound):
//! * **Each tick**: in a scene (`0x1a8f00` = 2) or with Ratchet in 0x61 it is hidden and does nothing more; else
//!   shown. The hit's rest counted down (out: the last hitter forgotten). Past its set-up its height is put back
//!   under the bob and a hit (not from the classes 0x0eef / 0x0942 / 0x0fff, and not the last hitter while the rest
//!   lasts) flashes it grey (0xd2), takes its damage off its health (and `0x18b65c`), 5 at none; the hitter kept, the
//!   rest 30 (`$gp+0x2b24`). Its scale is 0.85 of its class's. The HUD's bar `0x18b668`: its health of its most
//!   × 200 while Ratchet rides it, else 200.
//! * **0** (set-up): sequence 0 (blend 10) unless playing; the three rotors' nodes on the joint lists 2..4
//!   (`0x31f480`); its home here, still; its health and `0x18b65c` / `0x18b660` its most (L08 8 `$gp+0x2994`, L19 200
//!   `$gp+0x2998`, else 8); the power 0; 1.
//! * **1** (parked): with Ratchet free (not in the states 0x1c / 0x16 / 0x39 / 0x6f / 0x62 / 0x31, not in the
//!   groups 2..5, 7, 10, 0x10 or past 0xc, with health, in play, on the ground, not on the classes 0x013e / 0x046f)
//!   and his moby's sphere within 2 of its (`0x3203d0`): the prompt 0x12ea (owner 7); △ on it — the HUD's callback
//!   (`0x31a090(0, 0x41edd0)`), the music (`+0x194`, `+0x19c`), Ratchet in 0x31 (`0x2d31f8(0x31, 0)`) in his
//!   sequence 0x1b (`$gp+0x28d8`, blend 15) at its place and angles, `0x18b650` it, its idle sound stopped, the view
//!   lock on; 3. Its idle sound 9 (looped) kept going; then as 3, unsteered.
//! * **3** (ridden) and 1: no collision for its own tests; the stick, ✕ / ○, L2 / R2 and □ ([`controls`],
//!   `0x41fc10`) steer it while ridden ([`fly`]); □ held turns the engine's power over to the vacuum (its
//!   sequences 1 → 2 in, 3 out; the power −0.02 a tick, else +0.01; [`vacuum`] one tick in nine or on □ pressed);
//!   unridden the power falls. △ again (not the tick it was taken): off ([`get_off`], `0x420068`); 1.
//! * **5** (lost: no health, a surface of kind 1 or 0xb under it, or under `+0x17c`): off, an explosion at its joint
//!   0 (flashes 4 and 2 past 9, 10 embers, 3 billows, 16 flares, the light 15, the shake, one debris), back home; 0.
//! * **After**: its height kept, the bob (70°/s, 0.2 by the power), the sway (3°, 18°/s and 28°/s by the power,
//!   crate::moby::classes::props::sway), its angles from the yaw, the roll's and pitch's speeds. Ridden, Ratchet put
//!   0.5 behind its middle (its forward row's z at least 0) and 1.5 up, at its angles; its collision back; within 64
//!   of the camera its shadow probed (range 60). Its sounds: the throttle on — sound 0 looped; off — that one stopped
//!   unless the vacuum's.
//!
//! **The flight** ([`fly`]): the hover height 1.5 by the power (0.4 at none while ridden, 0 parked); the yaw turned
//! 240°/s by the stick × the steer (`min(speed, 40/s)/(40/s) · 0.2 + 0.8 − 0.3·(1 − power)`); the roll's speed eased
//! (0.05) toward −40° by the stick (by the power and `0.2·throttle + 0.8`) less its roll; the throttle (×0.5
//! backwards) pushes 45/s² × `(1 − speed/80)²` along its forward row (its rise at most the gravity's quarter, none
//! past 45° nose down), the strafe 10/s² along its side; the rotors (660°/s for the first, the others by the speed,
//! throttle and stick). The ground under its next place (the line from 0.5 up; a surface of kind 1 or 0xb loses it
//! close to it): its pitch's speed eased (0.05) toward the ground's slope ahead plus the stick's pitch (20°, by the
//! power); within 0.2 of the hover height the spring (0.1) holds it up, □'s pressure brakes it (5%), it drags (1%,
//! 0.8% across). Gravity 22/s². A sphere of 3 above its joint 0 (the world, then the mobys — on L19 Ratchet's left
//! out) puts it out of what it meets and slides its velocity along it (1% lost; on the ground by the power).
//!
//! **Its hits**: each tick a sphere of 1 at its joint 0 (0.3 down) hits the mobys (damage 1, L19 20, flags 0x10000,
//! along its velocity); past 10 a second also one of 0.7 at its joint 1.
//!
//! **The raritanium 0x099a** (its pvars: `+0` its life in seconds, `+4` the timer, `+8` the pull's speed): gone when
//! its uid is done (with a mission). **0**: hidden, update distance 0xff, the timer its life, sequence 0, on the ground
//! under it (from 200 up); 1. **1**: while Ratchet rides a hovership it tells it of itself (`+0x1b0`, the nearest at
//! `+0x68` / `+0x6c`, by the xy distance); within 10 with □ held and the hovership in its vacuum sequence 2 it drifts
//! toward him (a tenth of the way, at most 2 a second) and its timer runs: out — the hovership's `+0x8e` 180, the
//! timer 120, at him, shown (mode 0x800), the pull 0.1, sequence 1 (blend 30) at twice its pull's speed; 2. **2**: at
//! him, the pull eased (0.029) toward 1; when its timer is out (or its scale is none) it shrinks by 1% of its class's
//! scale, and at that or less — Ratchet not in the groups 0x14 / 0x19 or the states 0x39 / 0x6f — it is taken:
//! raritanium + 1 (`0x1a7a04`), its uid counted, gone.
//!
//! Not ported:
//! * The HUD's callback `0x41edd0` (the vehicle's health bar, the vacuum's texts 0x2dee / 0x2ded / 0x2def, the
//!   nearest raritanium's distance 0x3076 and its mark): the port has no draw hook for a class's screen text. Its
//!   reset of `+0x68` / `+0x6c` (1 000 000, none) after the moby loop is done here at the end of the hovership's own
//!   update while it is ridden.
//! * The nearest raritanium's marker moby (`+0x74`, `0x436c68` / `0x436d08`) and the HUD state behind it (`+0x88`,
//!   `+0x8a`, `+0x8c`, `$gp+0x2a38`, `$gp+0x29a4`).
//! * The dust under it (`0x41fd70`), the vacuum's camera shake (`0x1b9b20`), its engine trail (`0x33a4f0` /
//!   `0x33a550`), the sound `+0x18c` (`0x329d28`), the raritanium's glitter (`0x3d5350`, the crystals' glitter with
//!   its own colours: crate::moby::classes::level08::crystal::glitter).
//! * The global hit's lowest point (`0x1bff68`) it lowers to its height.
//! * The world sphere `0x2edbd0` (a list of its own at `$gp+0x3610`): the port's world sphere test.
//! * The get-off's `0x33e830()`, `0x189f28` and the hero moby's `+0x9c` (crate::hero::vehicle).

use crate::collision::{coll_line_m, coll_sphere_live, coll_sphere_m, QueryFlags};
use crate::hero::physics::{add, add_rot, cross, dist, dist2, dot, len, scale, set_len, sub, sub_rot, unit, V3};
use crate::moby::attach::angles_of;
use crate::moby::blast::{self, Explosion};
use crate::moby::hits::HitTemplate;
use crate::moby::{ai, Ctx, MobyWorld};
use crate::pad::button;
use rc_formats::moby_anim::JointModifier;

pub const HOVERSHIP: i16 = 0x0eef;
pub const RARITANIUM: i16 = 0x099a;

/// The hovership's pvars (module docs).
mod pv {
    pub const VEL: usize = 0x00;
    pub const ROLL: usize = 0x20;
    pub const PITCH: usize = 0x24;
    pub const YAW: usize = 0x28;
    pub const HOME: usize = 0x30;
    pub const BOB_PHASE: usize = 0x48;
    pub const BOB: usize = 0x4c;
    pub const SWAY_A: usize = 0x50;
    pub const SWAY_B: usize = 0x54;
    pub const LAST_Z: usize = 0x58;
    pub const THROTTLE: usize = 0x5c;
    pub const HEALTH: usize = 0x60;
    pub const NEAR_D: usize = 0x68;
    pub const NEAR: usize = 0x6c;
    pub const POWER: usize = 0x78;
    pub const T7C: usize = 0x7c;
    pub const COOL: usize = 0x8b;
    pub const HUD_8E: usize = 0x8e;
    pub const FLASH: usize = 0x90;
    pub const LAST_HIT: usize = 0xa0;
    pub const ANGLES: usize = 0x170;
    pub const KILL_Z: usize = 0x17c;
    pub const RATES: usize = 0x180;
    pub const MUSIC_ON: (usize, usize) = (0x194, 0x19c);
    pub const MUSIC_OFF: (usize, usize) = (0x198, 0x1a0);
    pub const SND: usize = 0x1a4;
    pub const VAC: usize = 0x1a8;
    pub const VAC_T: usize = 0x1ac;
    pub const PICKUP: usize = 0x1b0;
    pub const IDLE: usize = 0x1b4;
}

const DEG: f32 = 0.017_453_294;
/// Degrees a second as radians a tick.
const DEG_TICK: f32 = 0.000_290_888_25;
const PER_TICK: f32 = 0.016_666_668;
const PER_TICK2: f32 = 0.000_277_777_78;

// `$gp+0x2890..` (L08 `0x1b1880..`).
const ROTOR_SPIN: f32 = 660.0;
const ROTOR_STRENGTH: f32 = 1800.0;
const GET_ON: f32 = 2.0;
const BOB_AMP: f32 = 0.2;
const BOB_RATE: f32 = 70.0;
const SWAY: (f32, f32, f32) = (3.0, 18.0, 28.0);
const YAW_RATE: f32 = 240.0;
const DRAG: f32 = 0.99;
const DRAG_ACROSS: f32 = 0.992;
const PUSH: f32 = 45.0;
const TOP_SPEED: f32 = 80.0;
const HOVER: f32 = 1.5;
const SPRING: f32 = 0.1;
const SPHERE: f32 = 3.0;
/// Ratchet's sequence aboard (`$gp+0x28d8`) and the word it is tested against (`$gp+0x28dc`, 0 on the disc).
const RIDE_SEQ: i32 = 0x1b;
const RIDE_SEQ_WORD: i32 = 0;
const GRAVITY: f32 = 22.0;
const WALL_SLOW: f32 = 0.99;
const GROUND_SLOW: f32 = 0.99;
const BANK_EASE: f32 = 0.05;
const BANK: f32 = -40.0;
const BACK: f32 = 0.5;
const TURN_PUSH: f32 = 0.0;
const BANK_MIX: f32 = 0.2;
const PITCH_EASE: f32 = 0.05;
const PITCH_IN: f32 = 20.0;
const STEER_MIX: f32 = 0.2;
const STEER_RATE: f32 = 40.0;
const BRAKE: f32 = 0.05;
const HIT_R: (f32, f32) = (1.0, 0.7);
/// `$gp+0x291c`: the vacuum's reach.
const VACUUM_REACH: f32 = 35.0;
const SCALE: f32 = 0.85;
/// `$gp+0x2978..+0x2980`: its hits' damage (else, L08, L19).
const DAMAGE: (f32, f32, f32) = (1.0, 1.0, 20.0);
const HIT_DOWN: f32 = -0.3;
const HIT_FAST: f32 = 10.0;
/// `$gp+0x2990..+0x2998`: its most health (else, L08, L19).
const HEALTH: (f32, f32, f32) = (8.0, 8.0, 200.0);
const POWER_DOWN: f32 = 0.02;
const POWER_UP: f32 = 0.01;
/// The hover height at no power: parked, ridden.
const HOVER_LOW: (f32, f32) = (0.0, 0.4);
const STEER_LOW: f32 = -0.3;
const PITCH_MIX: f32 = 0.5;
const STRAFE: f32 = 10.0;
/// `$gp+0x2b24`: the hit's rest.
const HIT_REST: u8 = 30;
/// The prompt (owner 7).
const PROMPT: (i32, u32) = (7, 0x12ea);

/// The raritanium's (`$gp+0x12dc..`): its reach, the pull's ease and start, the shrink, the drift (a second), the
/// timer at him.
const NEAR: f32 = 10.0;
const PULL_EASE: f32 = 0.029;
const PULL: f32 = 0.1;
const SHRINK: f32 = 0.01;
const DRIFT: f32 = 2.0;
const HELD: i32 = 0x78;

/// `0x41fc10`: (turn, throttle, pitch, brake, strafe) — the stick's x and y reversed, ✕ − ○, □'s pressure, L2 − R2
/// (all but the pitch and the brake at most 1 each way).
fn controls(ctx: &Ctx) -> (f32, f32, f32, f32, f32) {
    let h = ctx.hero.items.held;
    let b = |m: u32| -> f32 { if h & m != 0 { 1.0 } else { 0.0 } };
    let turn = (-ctx.hero.stick[0]).clamp(-1.0, 1.0);
    let throttle = (b(button::CROSS) - b(button::CIRCLE)).clamp(-1.0, 1.0);
    let strafe = (b(button::L2) - b(button::R2)).clamp(-1.0, 1.0);
    (turn, throttle, -ctx.hero.stick[1], ctx.hero.vehicle.square, strafe)
}

/// `0x3203d0(a, b)`: the gap between two mobys' spheres (negative when they overlap).
fn sphere_gap(w: &MobyWorld, a: usize, b: Option<usize>, ctx: &Ctx) -> f32 {
    let s = w.mobys[a].bsphere;
    let t = b.and_then(|b| w.mobys.get(b)).map(|m| m.bsphere).filter(|t| t[3] != 0.0).unwrap_or({
        let p = ctx.hero.pos;
        [p[0] * 1024.0, p[1] * 1024.0, p[2] * 1024.0, 0.0]
    });
    (dist([s[0], s[1], s[2]], [t[0], t[1], t[2]]) - s[3] - t[3]) / 1024.0
}

fn by_level(ctx: &Ctx, v: (f32, f32, f32)) -> f32 {
    match ctx.game.level {
        8 => v.1,
        0x13 => v.2,
        _ => v.0,
    }
}

/// Keeps the looped sound `slot` going: `index` again when it no longer plays.
fn keep_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, slot: usize, index: i32) {
    if !ctx.host.sound_playing(w.mobys[id].pv_i(slot)) {
        let h = ctx.host.play_sound(&w.mobys[id], index, 4);
        w.mobys[id].set_pv_i(slot, h);
    }
}

fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, slot: usize) {
    let h = w.mobys[id].pv_i(slot);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(slot, -1);
}

pub fn update_0eef(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.scenes.mode == crate::scene::SCENE || ctx.hero.state == 0x61 {
        let m = &mut w.mobys[id];
        m.drawn = 0;
        m.mode |= 1;
        return;
    }
    {
        let m = &mut w.mobys[id];
        m.drawn = 1;
        m.mode &= !1;
        let mut c = m.pv_b(pv::COOL) as i32;
        let out = crate::moby::countdown(&mut c);
        m.set_pv_b(pv::COOL, c as u8);
        if out { m.set_pv_i(pv::LAST_HIT, 0); }
    }
    if w.mobys[id].state == 0 {
        let z = w.mobys[id].pos[2];
        w.mobys[id].set_pv_f(pv::LAST_Z, z);
    } else {
        let z = w.mobys[id].pv_f(pv::LAST_Z);
        w.mobys[id].pos[2] = z;
        if let Some(hit) = w.hit_message(id) {
            let src = hit.tmpl.source;
            let class = src.and_then(|s| w.mobys.get(s)).map(|s| s.o_class);
            let m = &mut w.mobys[id];
            let last = ai::link(m, pv::LAST_HIT);
            if let Some(c) = class {
                if c != HOVERSHIP && c != 0x0942 && c != 0x0fff && (src != last || m.pv_b(pv::COOL) == 0) {
                    for k in 7..10 { m.set_pv_b(pv::FLASH + k, 0xd2); }
                    ai::flash_start(m, pv::FLASH);
                    let hp = m.pv_f(pv::HEALTH) - hit.tmpl.damage;
                    ctx.hero.vehicle.health = hp;
                    m.set_pv_f(pv::HEALTH, hp);
                    if hp <= 0.0 { m.set_state(5, -1); }
                    ai::set_link(m, pv::LAST_HIT, src);
                    m.set_pv_b(pv::COOL, HIT_REST);
                }
            }
        }
        w.clear_hit(id);
    }
    ai::flash(&mut w.mobys[id], pv::FLASH);
    let class_scale = w.class(HOVERSHIP).map_or(1.0, |c| c.scale);
    w.mobys[id].scale = class_scale * SCALE;
    let ridden = ctx.hero.vehicle.moby == Some(id);
    let v = &mut ctx.hero.vehicle;
    v.bar = if ridden && w.mobys[id].state != 0 { (v.health / v.max_health * 200.0) as i32 } else { 200 };

    let mut fresh = false;
    match w.mobys[id].state {
        0 => {
            let seq_b = ctx.host.anim(&w.mobys[id]).map_or(0, |a| a.seq_b);
            if seq_b != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 10); }
            let m = &mut w.mobys[id];
            if m.joint_nodes.is_empty() { m.joint_nodes = (2u8..5).map(|l| (l, JointModifier::compose(0))).collect(); }
            m.set_pv_v(pv::HOME, m.pos);
            m.set_pv_v(pv::VEL, [0.0; 3]);
            let most = by_level(ctx, HEALTH);
            ctx.hero.vehicle.health = most;
            ctx.hero.vehicle.max_health = most;
            m.set_pv_f(pv::HEALTH, most);
            m.set_state(1, -1);
            m.set_pv_i(pv::SND, -1);
            m.set_pv_f(pv::POWER, 0.0);
            m.set_pv_i(pv::VAC_T, 0);
            m.set_pv_i(pv::PICKUP, 0);
        }
        1 | 3 => {
            if w.mobys[id].state == 1 {
                fresh = try_get_on(w, id, ctx);
                keep_sound(w, id, ctx, pv::IDLE, 9);
            }
            steer(w, id, ctx);
            fly(w, id, ctx);
            if w.mobys[id].state == 3 { vacuum_sounds(w, id, ctx); }
            hits(w, id, ctx);
            if !fresh && ctx.hero.vehicle.moby == Some(id) && ctx.hero.items.pressed & button::TRIANGLE != 0 {
                get_off(w, id, ctx);
                w.mobys[id].set_state(1, -1);
                keep_sound(w, id, ctx, pv::IDLE, 9);
            }
        }
        5 => {
            get_off(w, id, ctx);
            let m = &w.mobys[id];
            let at = ctx.host.joint(m, 0).unwrap_or(m.pos);
            let e = Explosion { flash: 4.0, flash2: 2.0, near: 9.0, size: 1.0, light: 15.0, embers: 10, billows: 3, flares: 16, sound: -1, shake: true, debris: 1, lod: -1, flags: 0x81_0001, ..Default::default() };
            blast::explode(w, ctx, Some(id), Some(at), &e);
            let m = &mut w.mobys[id];
            m.pos = m.pv_v(pv::HOME);
            m.set_state(0, -1);
        }
        _ => {}
    }
    after(w, id, ctx);
}

/// State 1's way in (module docs); true the tick it is taken.
fn try_get_on(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let h = &*ctx.hero;
    let busy_state = matches!(h.state, 0x1c | 0x16 | 0x39 | 0x6f | 0x62 | 0x31);
    let busy_group = matches!(h.group, 2 | 3 | 4 | 5 | 7 | 10 | 0x10) || 0xc < h.group;
    if busy_state || busy_group || h.hp < 1 || ctx.game.scenes.mode != crate::scene::PLAY || h.grounded_ticks == 0 { return false; }
    if let Some(c) = h.ground_moby.and_then(|g| w.mobys.get(g)) {
        if c.o_class == 0x013e || c.o_class == 0x046f { return false; }
    }
    if GET_ON <= sphere_gap(w, id, ctx.hero_moby, ctx) { return false; }
    let ready = ctx.game.prompt.request(PROMPT.0, PROMPT.1, 1) != 0;
    if ctx.hero.items.pressed & button::TRIANGLE == 0 || !ready { return false; }
    let m = &w.mobys[id];
    ctx.host.music_request(m.pv_i(pv::MUSIC_ON.0), m.pv_i(pv::MUSIC_ON.1));
    let (pos, rot) = (m.pos, m.rot);
    ctx.hero.ride_on(id, pos, rot, false);
    ctx.hero.anim_req = Some((15.0, RIDE_SEQ as u8, 0));
    w.mobys[id].set_state(3, -1);
    stop_sound(w, id, ctx, pv::IDLE);
    ctx.hero.view_lock = 1;
    true
}

/// The moby's sequence while ridden or parked (module docs): `held` □ held (ridden).
fn vacuum_seq(w: &MobyWorld, id: usize, ctx: &mut Ctx, held: bool, blend: u8) {
    let Some(a) = ctx.host.anim(&w.mobys[id]) else { return };
    let (sa, sb, wrapped) = (a.seq_a, a.seq_b, a.flags & 2 != 0);
    let m = &w.mobys[id];
    if held {
        if (1..=3).contains(&sa) || (1..=3).contains(&sb) {
            if !wrapped { return; }
            if sa == 1 {
                if sb != 2 { ctx.host.set_anim(m, 2, 0, blend); }
            } else if (sb == 3 || sb == 0) && sb != 1 {
                ctx.host.set_anim(m, 1, 0, blend);
            }
        } else if sb != 1 {
            ctx.host.set_anim(m, 1, 0, blend);
        }
    } else if wrapped {
        if (1..=2).contains(&sa) || sb == 1 || sb == 2 {
            if sb != 3 { ctx.host.set_anim(m, 3, 0, blend); }
        } else if sb != 0 {
            ctx.host.set_anim(m, 0, 0, blend);
        }
    }
}

/// The power and the sequences, ridden or not (module docs).
fn steer(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].has_collision = false;
    let mut t = w.mobys[id].pv_s(pv::T7C) as i32;
    crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_s(pv::T7C, t as i16);
    let mut power = w.mobys[id].pv_f(pv::POWER);
    if w.mobys[id].state == 3 {
        ctx.hero.view_lock = 1;
        if ctx.hero.items.held & button::SQUARE == 0 {
            vacuum_seq(w, id, ctx, false, 10);
            power += POWER_UP;
        } else {
            vacuum_seq(w, id, ctx, true, 10);
            power -= POWER_DOWN;
            if ctx.game.rng.randi(9) == 0 || ctx.hero.items.pressed & button::SQUARE != 0 { vacuum(w, id); }
        }
        if RIDE_SEQ_WORD != RIDE_SEQ && RIDE_SEQ != -1 { ctx.hero.anim_req = Some((15.0, RIDE_SEQ as u8, 0)); }
    } else {
        power -= POWER_DOWN;
        vacuum_seq(w, id, ctx, false, 2);
    }
    w.mobys[id].set_pv_f(pv::POWER, power.clamp(0.0, 1.0));
}

/// `0x41fad8`: the vacuum wakes the classes 0x0942 / 0x0962 / 0x0fff / 0x1002 within 35 across (their timer byte's
/// bit 1; their update and draw distances raised to 5 past the distance).
fn vacuum(w: &mut MobyWorld, id: usize) {
    let at = w.mobys[id].pos;
    for m in w.mobys.iter_mut() {
        if 0x80 <= m.state || !matches!(m.o_class, 0x0942 | 0x0962 | 0x0fff | 0x1002) { continue; }
        let d = dist2(at, m.pos);
        if VACUUM_REACH <= d { continue; }
        m.timer |= 2;
        if (m.update_dist as f32) < d { m.update_dist = (d + 5.0).min(255.0) as u8; }
        if (m.draw_dist as f32) < d { m.draw_dist = (d + 5.0) as i16; }
    }
}

/// The flight (module docs).
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let ridden = w.mobys[id].state == 3;
    let (mut turn, mut throttle, mut pitch_in, mut brake, mut strafe) = if ridden { controls(ctx) } else { (0.0, 0.0, 0.0, 0.0, 0.0) };
    let m = &w.mobys[id];
    let power = m.pv_f(pv::POWER);
    let rows = m.rows;
    turn *= power;
    throttle *= power;
    brake += 1.0 - power;
    strafe *= power;
    let hover = HOVER * power + if ridden { HOVER_LOW.1 } else { HOVER_LOW.0 } * (1.0 - power);
    let mut vel = m.pv_v(pv::VEL);
    let speed = len(vel) * power;
    let top = TOP_SPEED * PER_TICK;
    let rate = STEER_RATE * PER_TICK;
    pitch_in *= DEG * PITCH_IN;
    let steer = speed.min(rate) / rate * STEER_MIX + (1.0 - STEER_MIX) + STEER_LOW * (1.0 - power);
    let yaw = add_rot(m.pv_f(pv::YAW), YAW_RATE * turn * steer * DEG_TICK);
    let roll = m.pv_f(pv::ROLL) + (BANK * turn * DEG * power * (BANK_MIX * throttle + (1.0 - BANK_MIX)) - m.rot[0]) * BANK_EASE;
    if throttle < 0.0 { throttle *= BACK; }
    let sign = if throttle < 0.0 { -1.0 } else { 1.0 };
    let damping = ((top - speed) / top).powi(2);
    let acc = (throttle + TURN_PUSH * turn.abs() * sign) * PER_TICK2 * PUSH * damping;
    // The rotors.
    let mut rates = [power * DEG_TICK * ROTOR_SPIN, 0.0, 0.0];
    for (k, r) in rates.iter_mut().enumerate().skip(1) {
        let mut x = speed / top + throttle;
        x = if k & 1 == 0 { x - turn } else { x + turn };
        x *= ROTOR_STRENGTH * DEG_TICK * 0.5;
        *r = if k & 1 != 0 { -x } else { x };
    }
    let m = &mut w.mobys[id];
    m.set_pv_f(pv::THROTTLE, throttle);
    m.set_pv_f(pv::YAW, yaw);
    m.set_pv_f(pv::ROLL, roll);
    for k in 0..3 {
        m.set_pv_f(pv::RATES + 4 * k, rates[k]);
        let a = add_rot(m.pv_f(pv::ANGLES + 4 * k), rates[k]);
        m.set_pv_f(pv::ANGLES + 4 * k, a);
        let axis = if k == 0 { [0.0, 0.0, 1.0] } else { [1.0, 0.0, 0.0] };
        if let Some((_, n)) = m.joint_nodes.get_mut(k) { n.quat = crate::quat::axis_angle(a, axis); }
    }
    let pos = m.pos;
    let mut next = add(pos, vel);
    // The ground under the next place (`0x32a160(0.5, next, 0)`).
    let ground = ctx.coll.and_then(|c| coll_line_m(c, ctx.scene, [next[0], next[1], next[2] + 0.5], [next[0], next[1], 0.01], QueryFlags(2), Some(id)));
    let height = ground.map_or(0.0, |g| g.point[2]);
    let special = ground.is_some_and(|g| matches!(g.surface_id(), 1 | 0xb));
    let low = height + hover;
    if pos[2] < low + 0.3 && special { w.mobys[id].set_state(5, -1); }
    let mut airborne = true;
    vel[2] -= GRAVITY * PER_TICK2;
    let m = &mut w.mobys[id];
    if height != 0.0 {
        let n = unit(ground.map_or([0.0, 0.0, 1.0], |g| g.normal));
        let b1 = cross(rows[0], n);
        let b0 = cross(b1, n);
        let slope = angles_of(&[b0, b1, n])[1];
        let pitch = m.pv_f(pv::PITCH);
        let to = add_rot(-slope, pitch_in * 0.5 * PITCH_MIX * (power + 1.0));
        m.set_pv_f(pv::PITCH, add_rot(pitch, sub_rot(to, pitch) * PITCH_EASE));
        if next[2] < low + 0.2 {
            airborne = false;
            if next[2] + 0.1 < height || special {
                next[2] = height;
                vel = scale(vel, GROUND_SLOW);
            }
            vel[2] += ((low - next[2]) * SPRING - vel[2]) * SPRING;
            if 0.0 < brake {
                let k = 1.0 - BRAKE * brake;
                vel[0] *= k;
                vel[1] *= k;
            }
            vel = scale(vel, DRAG);
            let mut l = [dot(vel, rows[0]), dot(vel, rows[1]), dot(vel, rows[2])];
            l[1] *= DRAG_ACROSS;
            vel = add(add(scale(rows[0], l[0]), scale(rows[1], l[1])), scale(rows[2], l[2]));
        }
        let mut thrust = set_len(rows[0], acc);
        if m.pv_f(pv::PITCH) < -0.785_398_24 {
            if 0.0 < thrust[2] { thrust[2] = 0.0; }
        } else {
            let most = GRAVITY * 6.944_444_6e-5;
            if most < thrust[2] { thrust[2] = most; }
        }
        thrust = set_len(thrust, acc.abs());
        let side = set_len(rows[1], STRAFE * strafe * PER_TICK2 * power);
        vel = add(vel, add(thrust, side));
    }
    m.pos = next;
    m.set_pv_v(pv::VEL, vel);
    // The sphere above joint 0.
    let mut c = ctx.host.joint(&w.mobys[id], 0).unwrap_or(next);
    c[2] += SPHERE - HOVER_LOW.0;
    let back = sub(next, c);
    let hit = ctx.coll.and_then(|coll| {
        coll_sphere_m(coll, None, c, SPHERE, QueryFlags(0), Some(id)).or_else(|| {
            if ctx.game.level == 0x13 {
                let hero = ctx.hero_moby;
                coll_sphere_live(coll, ctx.scene, c, SPHERE, QueryFlags(0), Some(id), &|i| Some(i) != hero)
            } else {
                coll_sphere_m(coll, ctx.scene, c, SPHERE, QueryFlags(2), Some(id))
            }
        })
    });
    let solid = w.class(HOVERSHIP).is_some_and(|k| k.has_collision);
    w.mobys[id].has_collision = solid;
    let Some(hit) = hit else { return };
    if matches!(hit.surface_id(), 1 | 0xb) { w.mobys[id].set_state(5, -1); }
    if hit.moby.is_some() && hit.moby == ctx.hero_moby { return; }
    let mut give = 1.0;
    let mut pushed = hit.pushed_centre.unwrap_or(c);
    if airborne {
        for k in 0..2 {
            let mut d = pushed[k] - c[k];
            if d < 0.050_000_004 { d *= power; }
            pushed[k] = d + c[k];
        }
    } else {
        give = power;
    }
    let m = &mut w.mobys[id];
    m.pos = add(pushed, back);
    let n = hit.normal;
    let mut vel = m.pv_v(pv::VEL);
    let s = unit(cross(cross(n, vel), n));
    vel = scale(set_len(s, dot(vel, s) * give), WALL_SLOW);
    m.set_pv_v(pv::VEL, vel);
    if m.pos[2] < m.pv_f(pv::KILL_Z) { m.set_state(5, -1); }
}

/// State 3's vacuum sounds (module docs): □ held with the power under 0.98 — sound 1 once (unless resting), then
/// sound 2 looped in the engine's slot, the idle stopped; else back: that stopped, sound 4 once (unless resting), or
/// the idle sound 9 kept going.
fn vacuum_sounds(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    if m.pv_f(pv::POWER) < 0.98 && ctx.hero.items.held & button::SQUARE != 0 {
        match m.pv_i(pv::VAC) {
            0 => {
                if m.pv_i(pv::VAC_T) == 0 { ctx.host.play_sound(m, 1, 0); }
                let m = &mut w.mobys[id];
                m.set_pv_i(pv::VAC, 2);
                m.set_pv_i(pv::VAC_T, 15);
            }
            2 => {
                if !ctx.host.sound_playing(m.pv_i(pv::SND)) {
                    let h = ctx.host.play_sound(m, 2, 4);
                    w.mobys[id].set_pv_i(pv::SND, h);
                    stop_sound(w, id, ctx, pv::IDLE);
                }
            }
            _ => {}
        }
        stop_sound(w, id, ctx, pv::IDLE);
    } else if m.pv_i(pv::VAC) == 2 {
        stop_sound(w, id, ctx, pv::SND);
        let m = &w.mobys[id];
        if m.pv_i(pv::VAC_T) == 0 { ctx.host.play_sound(m, 4, 0); }
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::VAC, 0);
        m.set_pv_i(pv::VAC_T, 15);
    } else {
        keep_sound(w, id, ctx, pv::IDLE, 9);
    }
}

/// Its hits on the mobys (module docs).
fn hits(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let vel = m.pv_v(pv::VEL);
    let t = HitTemplate::new(by_level(ctx, DAMAGE), Some(id), 0x1_0000, [vel[0], vel[1], vel[2], 0.0]);
    let mut c = ctx.host.joint(m, 0).unwrap_or(m.pos);
    c[2] += HIT_DOWN;
    w.sphere_hit(ctx.scene, c, HIT_R.0, 0, Some(id), Some(&t));
    if HIT_FAST * PER_TICK < len(vel) {
        let m = &w.mobys[id];
        let c = ctx.host.joint(m, 1).unwrap_or(m.pos);
        w.sphere_hit(ctx.scene, c, HIT_R.1, 0, Some(id), Some(&t));
    }
}

/// `0x420068`: off the hovership — the music (`+0x198`, `+0x1a0`), the HUD's callback off, Ratchet idle with the view
/// lock off and `0x18b650` cleared, his landing lag 10, `0x18b654` 0xffff; put 2.5 along the camera's forward across
/// from it, 0.7 up — where the line from him to there meets something, there (and where a sphere of 0.4 there
/// meets something, there).
fn get_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    ctx.host.music_request(m.pv_i(pv::MUSIC_OFF.0), m.pv_i(pv::MUSIC_OFF.1));
    ctx.hero.ride_off();
    ctx.hero.land_lag = 10;
    ctx.hero.vehicle.word_654 = 0xffff;
    let f = ctx.host.camera_rows()[0];
    let mut to = set_len([f[0], f[1], 0.0], 2.5);
    to[2] = 0.7;
    to = add(to, m.pos);
    if let Some(coll) = ctx.coll {
        let hero = ctx.hero_moby;
        let line = coll_sphere_line(coll, ctx, ctx.hero.pos, to, hero, id);
        if let Some(p) = line {
            to = p;
            if let Some(o) = coll_sphere_live(coll, ctx.scene, to, 0.4, QueryFlags(2), hero, &|i| i != id) { to = o.point; }
        }
    }
    let rot = ctx.hero.rot;
    ctx.hero.ride_place(to, rot);
}

/// The line `0x2ea670(a, b, 2, hero, 0)` with the hovership's collision off: its hit point.
fn coll_sphere_line(coll: &rc_formats::collision::Collision, ctx: &Ctx, a: V3, b: V3, hero: Option<usize>, me: usize) -> Option<V3> {
    crate::collision::coll_line_live(coll, ctx.scene, a, b, QueryFlags(2), hero, &|i| i != me).map(|o| o.point)
}

/// After the states (module docs).
fn after(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let power = m.pv_f(pv::POWER);
    m.set_pv_f(pv::LAST_Z, m.pos[2]);
    let phase = add_rot(m.pv_f(pv::BOB_PHASE), BOB_RATE * power * DEG_TICK);
    m.set_pv_f(pv::BOB_PHASE, phase);
    let bob = phase.sin() * BOB_AMP * power;
    m.set_pv_f(pv::BOB, bob);
    m.pos[2] += bob * power;
    let k = power * DEG_TICK;
    super::super::props::sway(m, SWAY.0 * DEG, SWAY.1 * k, SWAY.2 * k, pv::SWAY_A, pv::SWAY_B);
    m.rot[2] = m.pv_f(pv::YAW);
    m.rot[0] = add_rot(m.rot[0], m.pv_f(pv::ROLL));
    m.rot[1] = add_rot(m.rot[1], m.pv_f(pv::PITCH));
    if m.state == 3 {
        let mut off = set_len(m.rows[0], -0.5);
        if off[2] < 0.0 { off[2] = 0.0; }
        off[2] += 1.5;
        let at = add(off, m.pos);
        let (z, rot) = (m.pos[2], m.rot);
        ctx.hero.ride_place(at, rot);
        ctx.hero.ground_point = [at[0], at[1], z];
        // The HUD callback's reset after the moby loop (module docs).
        m.set_pv_f(pv::NEAR_D, 1_000_000.0);
        m.set_pv_i(pv::NEAR, 0);
    }
    let solid = w.class(HOVERSHIP).is_some_and(|k| k.has_collision);
    w.mobys[id].has_collision = solid;
    if w.mobys[id].drawn != 0 && dist(w.mobys[id].pos, ctx.host.camera_pos()) < 64.0 {
        crate::shadow_volumes::probe_near(w, id, ctx, 64.0, 0x3c);
    }
    if w.mobys[id].pv_f(pv::THROTTLE) == 0.0 {
        if w.mobys[id].pv_i(pv::VAC) != 2 { stop_sound(w, id, ctx, pv::SND); }
    } else if ctx.host.sound_playing(w.mobys[id].pv_i(pv::SND)) {
        stop_sound(w, id, ctx, pv::IDLE);
    } else {
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(pv::SND, h);
    }
    let m = &mut w.mobys[id];
    let mut t = m.pv_i(pv::VAC_T);
    crate::moby::countdown(&mut t);
    m.set_pv_i(pv::VAC_T, t);
}

/// 0x099a (module docs; its pvars `+0` life, `+4` timer, `+8` pull).
pub fn update_099a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    if m.mission != 0xff {
        let dead = ctx.game.uid_dead.get(m.uid.max(0) as usize).is_some_and(|&d| d != 0);
        if dead || ctx.game.uid_count(m.uid) != 0 { return w.delete(id); }
    }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.drawn = 0;
            m.mode |= 1;
            m.set_pv_i(4, m.pv_i(0) * 60);
            let seq_b = ctx.host.anim(&w.mobys[id]).map_or(0, |a| a.seq_b);
            if seq_b != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 2); }
            w.mobys[id].set_state(1, -1);
            let z = ai::ground_z(ctx, 200.0, w.mobys[id].pos, 0);
            if z != 0.0 { w.mobys[id].pos[2] = z; }
        }
        1 => {
            let Some(v) = ctx.hero.vehicle.moby.filter(|&v| w.mobys.get(v).is_some_and(|o| o.o_class == HOVERSHIP && o.state < 0xfd)) else { return };
            if ctx.hero.state != 0x31 { return; }
            let pos = w.mobys[id].pos;
            let d = dist2(pos, ctx.hero.pos);
            let o = &mut w.mobys[v];
            ai::set_link(o, pv::PICKUP, Some(id));
            if d < o.pv_f(pv::NEAR_D) {
                o.set_pv_f(pv::NEAR_D, d);
                ai::set_link(o, pv::NEAR, Some(id));
            }
            if NEAR <= d || ctx.hero.items.held & button::SQUARE == 0 { return; }
            let Some(a) = ctx.host.anim(&w.mobys[v]) else { return };
            if a.seq_a != 2 && a.seq_b != 2 { return; }
            let mut step = scale(sub(ctx.hero.pos, pos), 0.1);
            step[2] = 0.0;
            if DRIFT * PER_TICK < len(step) { step = set_len(step, DRIFT * PER_TICK); }
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, step);
            let mut t = m.pv_i(4);
            let out = crate::moby::countdown(&mut t);
            m.set_pv_i(4, t);
            if !out { return; }
            w.mobys[v].set_pv_s(pv::HUD_8E, 180);
            let m = &mut w.mobys[id];
            m.set_pv_i(4, HELD);
            m.pos = ctx.hero.pos;
            m.drawn = 1;
            m.mode = (m.mode & !1) | 0x800;
            m.set_pv_f(8, PULL);
            let seq_b = ctx.host.anim(&w.mobys[id]).map_or(0, |a| a.seq_b);
            if seq_b != 1 { ctx.host.set_anim(&w.mobys[id], 1, 0, 30); }
            w.mobys[id].set_state(2, -1);
            ctx.host.set_anim_speed(&w.mobys[id], PULL + PULL);
        }
        2 => {
            let m = &mut w.mobys[id];
            let pull = m.pv_f(8);
            let pull = pull + (1.0 - pull) * PULL_EASE;
            m.set_pv_f(8, pull);
            m.pos = ctx.hero.pos;
            let mut t = m.pv_i(4);
            let out = crate::moby::countdown(&mut t);
            m.set_pv_i(4, t);
            ctx.host.set_anim_speed(&w.mobys[id], pull + pull);
            let m = &w.mobys[id];
            if out || m.scale <= 0.0 {
                let cost = SHRINK * w.class(RARITANIUM).map_or(1.0, |c| c.scale);
                let h = &*ctx.hero;
                if m.scale <= cost && h.group != 0x14 && h.group != 0x19 && h.state != 0x39 && h.state != 0x6f {
                    ctx.game.raritanium = ctx.game.raritanium.wrapping_add(1);
                    ctx.game.count_uid(m.uid);
                    return w.delete(id);
                }
                w.mobys[id].scale -= cost;
            }
        }
        _ => {}
    }
}

