//! The Insomniac Museum's two hovercraft, the hovership's earlier siblings (crate::moby::classes::level08::hovership,
//! whose shared pieces they are built on): **0x085c** ([`update_085c`], the decomp's `func_US2_L08_003C7F00`: L21
//! `0x3b6008`, also in the overlays of L08 `0x3c7f00` and L19 `0x3b7610`, the class placed on the museum only) and
//! **0x0f2f** ([`update_0f2f`], `0x40c018`, its flight `0x40c8b0`; the decomp's `func_US2_L30_0040C018` /
//! `func_L30_0040C270`). Ratchet rides them in his state 0x31 (crate::hero::vehicle). L21 addresses below.
//!
//! **Their pvars** (as the hovership's: `+0x00` the velocity, `+0x20` / `+0x24` the roll's and pitch's speeds,
//! `+0x28` the yaw, `+0x30` home, `+0x48..+0x54` the bob and sway, `+0x58` its height before the bob, `+0x5c` the
//! throttle, `+0x60` its health, `+0x64` the HUD's texture set); then 0x085c: `+0x68` / `+0x6c` a nearest
//! raritanium's distance and moby (reset each tick), `+0x74` a sound, `+0x78` the engine's power, `+0x7c` / `+0x7e`
//! the guns' rest and side, `+0x80` / `+0x84` the guns, `+0x88` / `+0x8a` a HUD state, `+0x8b` the hit's rest,
//! `+0x90` the flash, `+0xa0` the last hitter, `+0xa4` / `+0xa8` the HUD's eased health and bar, `+0xac` the jump's
//! timer; 0x0f2f: `+0x68` the power, `+0x6e` the hit's rest, `+0x80` the flash, `+0x90` the last hitter, `+0x94` /
//! `+0x98` the HUD's, `+0x9c` the jump's timer.
//!
//! * **Each tick** (crate::moby::classes::level08::hovership::prologue): the hit's rest counted down; past state 0 its
//!   height put back under the bob and a hit (not from its own class, 0x0942 or 0x0fff) taken; its scale (1.4 / 1 of
//!   its class's). Ridden, the HUD's bar `0x18b668` its health (`0x18b65c`) of 30 × 200.
//! * **0**: home here, still, its health and `0x18b65c` / `0x18b660` 30, the power and the jump 0, the HUD's words
//!   (`+0xa8` / `+0x98` 0xff, `+0xa4` / `+0x94` 0); 1.
//! * **1** (parked): Ratchet free (hovership::hero_ready, and not in 0x62) and within 2: the prompt 0x12ea; △ on it
//!   — on (hovership::board: his sequence 0x1b), 3 (0x0f2f: its collision off). 0x085c flies only while he is free
//!   (unsteered, the power falling), and on from the tick he gets on; 0x0f2f coasts (unsteered) while it moves faster
//!   than 0.25/s across.
//! * **3** (ridden): L1 held turns the power over to the vacuum (−0.02 a tick, else +0.01; hovership::vacuum one tick
//!   in nine or on L1 pressed); his sequence 0x1b asked for; the flight ([`fly`]) steered; the power held to 0..1 (0x085c
//!   before the flight, 0x0f2f after). 0x085c: its collision off; R1 held with its guns' rest out — the side's gun's
//!   timer byte 1, the side swapped, the rest 20; its jump's timer counted down; △ (not the tick it was taken): 4.
//!   0x0f2f: △: 4; Ratchet's aim point (`0x189ef0`) his centre (`0x189f00`).
//! * **4** (off). 0x085c: its collision back, Ratchet idle (`0x2c4560(0, 1)`, `0x18b650` cleared) 1 up, `0x18b654`
//!   0xffff, `+0x74` / `+0x88` / `+0x8a` 0; 1. 0x0f2f: the first tick, Ratchet's stick (`0x18bdd0`) (0, −1) and his
//!   flip 0xb (`0x2c4560(0xb, 1)`), `0x18b650` cleared and `0x18b654` 0xffff; its collision off and the flight
//!   unsteered; past 30 ticks in the state its collision back, 1.
//! * **5** (lost: no health, a surface of kind 1 / 0xb under it): its collision back, Ratchet idle, the explosion
//!   (hovership::wreck), `0x18b654` 0xffff (0x085c: `+0x74` / `+0x88` / `+0x8a` 0); home, 0.
//! * **After** (hovership::bob_sway): the bob (70°/s, 0.2), the sway (3°, 18°/s, 28°/s), by the power, its angles.
//!   Ridden, Ratchet at its joint 0 (0.1 down for 0x085c, 0.1 up for 0x0f2f) at its angles. Within 64 of the camera
//!   its shadow probed (range 60). 0x085c: its collision back; the nearest raritanium's words reset (1 000 000,
//!   none).
//!
//! **The HUD** ([`draw_hud`], 0x085c `0x3b4e98`, 0x0f2f `0x40b478`: the same code on their own words, all equal):
//! registered each ridden tick on the second draw list (`0x2ee5d8` = L00 `0x2e7920`; crate::hud::list_screen).
//!
//! **The flight** ([`fly`]; 0x085c's inline, 0x0f2f's `0x40c8b0`; the controls `0x3b5a80` / `0x40bb20`: the stick
//! reversed for the turn and the pitch, `1 − (1 − p)²` of ✕'s pressure less ○'s for the throttle and of L2's less
//! R2's for the strafe, □'s pressure the brake on the levels 8, 21 and 30 (0x0f2f also 16), else the jump): as the
//! hovership's (hovership::drive, with the stick not by the power) — the yaw 240°/s (0x0f2f 120°/s, its steer's mix
//! −0.5), the hover height 2.2 (0x0f2f 1.5) by the power, 0.4 (0x0f2f 0) at none; the playback speed (`+0x48`) eased
//! (0.02) toward twice the throttle; the ground under its next place (hovership::ground_below): a surface of kind 1 /
//! 0xb close loses it; the jump's first 30 ticks of 180 lift it 50/s², else gravity 14/s². Over the hover height its
//! pitch eases (0.01) toward the stick's (20°), and its push is had unless on level 8; at it the jump starts (its
//! timer 0, the jump's input past 0.2), it holds (hovership::settle: the spring 0.1, the brake 5%, the drag 1%, 0.8%
//! across — 0x0f2f 5%) and its pitch eases (0.05) toward the slope ahead plus the stick's. The push: 45/s² ×
//! `(1 − speed/80)²` along its forward row, the strafe 10/s² along its side. A sphere of 1.8 (0x0f2f 2) above its joint
//! 0 (the world, then the mobys) puts it out of what it meets (across by the power), its velocity slid along it (by
//! the power, 1% lost). Its hits (hovership::hits_on): a sphere of 1 at joint 0 (0.3 down), damage 1 (0x0f2f 10),
//! along the velocity before the sphere; past 10/s a second of 0.7 at joint 3 (0x0f2f 1.5 at joint 1).
//!
//! The port's pad buttons are digital (pressure 0 or 1), so the controls read the held bits and □'s pressure.
//!
//! Not ported:
//! * The health bar of the HUD ([`draw_hud`]): its two FX textures (`$gp+0x920` / `0x924` + the set `+0x64`, 128 ×
//!   128 at (368, 144 up from the bottom), `0x70808080`) drawn through their palette (`$gp+0x9a8`), whose entries
//!   from 2 up to the bar's cut (`+0xa8` / `+0x98`) are the set's own (copied by state 0 to `0x243b20` / `0x25cd30`
//!   once a level) and the rest `0x80000000`: the HUD's layer holds the FX textures decoded, not their indices, so
//!   the bar is not drawn (its words are kept).
//! * The dust under them while ridden (`0x3b5d10`, the hovership's `0x41fd70`; 0x0f2f's `0x40bcf0`) and 0x085c's
//!   vacuum camera shake (`0x1b9c60`).
//! * 0x085c's guns (`0x3b5c48`: two 0x09a1 mobys, `0x3c1468`, kept at its joints 7 / 8; their update `0x3c14f8`
//!   fires when its timer byte is set): no level carries the class 0x09a1, so they are created classless and do
//!   nothing; the port creates none.
//! * 0x085c's level-8 block (the nearest raritanium's marker state `+0x8a` / `+0x88` / `+0x8c` and its sound `+0x74`):
//!   the class is placed on the museum only.
//! * The hero moby's `+0x9c`, `0x189f28` (1/6 at 0x085c's way off).

use super::super::level08::hovership::{self as hs, Bob, Drive, Hull, Settle, Strike, DEG, PER_TICK, PER_TICK2};
use crate::collision::{coll_sphere_m, QueryFlags};
use crate::hero::physics::{add, add_rot, len, set_len, sub, sub_rot};
use crate::moby::{Ctx, MobyWorld};
use crate::pad::button;

pub const EARLY: i16 = 0x085c;
pub const MUSEUM: i16 = 0x0f2f;

/// The pvar places the two share with the hovership.
mod pv {
    pub const VEL: usize = 0x00;
    pub const ROLL: usize = 0x20;
    pub const PITCH: usize = 0x24;
    pub const YAW: usize = 0x28;
    pub const HOME: usize = 0x30;
    pub const THROTTLE: usize = 0x5c;
    pub const HEALTH: usize = 0x60;
}

/// 0x085c's own pvars.
mod e {
    pub const NEAR_D: usize = 0x68;
    pub const NEAR: usize = 0x6c;
    pub const SOUND: usize = 0x74;
    pub const GUN_REST: usize = 0x7c;
    pub const GUN_SIDE: usize = 0x7e;
    pub const HUD_88: usize = 0x88;
    pub const HUD_8A: usize = 0x8a;
    pub const HUD_EASE: usize = 0xa4;
    pub const HUD_BAR: usize = 0xa8;
}

/// 0x0f2f's own pvars.
mod f {
    pub const HUD_EASE: usize = 0x94;
    pub const HUD_BAR: usize = 0x98;
}

/// A craft's table (its update's `$gp` words: 0x085c `$gp+0x860..`, 0x0f2f `$gp+0x25b0..`, the same layout).
struct Craft {
    hull: Hull,
    bob: Bob,
    drive: Drive,
    settle: Settle,
    strike: Strike,
    power: usize,
    jump: usize,
    damage: f32,
    most: f32,
    get_on: f32,
    hover: f32,
    hover_low: f32,
    sphere: f32,
    gravity: f32,
    rise: f32,
    /// The jump's timer, and its first ticks that lift.
    jump_time: i32,
    rise_ticks: i32,
    pitch_in: f32,
    pitch_mix: f32,
    /// The pitch's ease in the air and on the ground.
    ease: (f32, f32),
    strafe: f32,
    wall_slow: f32,
    /// The power's fall and rise a tick.
    power_step: (f32, f32),
    /// The playback speed by the throttle.
    anim: f32,
    /// Ratchet's seat: the joint and the height over it.
    seat: (usize, f32),
    /// The levels where □ brakes (else it jumps).
    brake_levels: &'static [i32],
}

/// Ratchet's sequence aboard (`$gp+0x8a4` / `+0x25f4`; the word it is tested against, 0, never matches it).
const RIDE_SEQ: u8 = 0x1b;
/// 0x085c's guns' rest (`$gp+0x99c`).
const GUN_REST: i16 = 20;
/// 0x0f2f's coasting speed across (/s).
const COAST: f32 = 0.25;

const C085C: Craft = Craft {
    hull: Hull { own: EARLY, flash: 0x90, cool: 0x8b, last: 0xa0, health: pv::HEALTH, rest: 30, scale: 1.4 },
    bob: Bob { amp: 0.2, rate: 70.0, sway: (3.0, 18.0, 28.0) },
    drive: Drive { yaw_rate: 240.0, steer_rate: 40.0, steer_mix: 0.2, steer_low: -0.3, bank: -40.0, bank_mix: 0.2, bank_ease: 0.05, back: 0.5, turn_push: 0.0, push: 45.0, top: 80.0 },
    settle: Settle { ground_slow: 0.99, spring: 0.1, brake: 0.05, drag: 0.99, across: 0.992 },
    strike: Strike { radius: (1.0, 0.7), down: -0.3, fast: 10.0, joint: 3 },
    power: 0x78,
    jump: 0xac,
    damage: 1.0,
    most: 30.0,
    get_on: 2.0,
    hover: 2.2,
    hover_low: 0.4,
    sphere: 1.8,
    gravity: 14.0,
    rise: 50.0,
    jump_time: 180,
    rise_ticks: 30,
    pitch_in: 20.0,
    pitch_mix: 0.5,
    ease: (0.01, 0.05),
    strafe: 10.0,
    wall_slow: 0.99,
    power_step: (0.02, 0.01),
    anim: 2.0,
    seat: (0, -0.1),
    brake_levels: &[8, 0x1e, 0x15],
};

const C0F2F: Craft = Craft {
    hull: Hull { own: MUSEUM, flash: 0x80, cool: 0x6e, last: 0x90, health: pv::HEALTH, rest: 30, scale: 1.0 },
    bob: Bob { amp: 0.2, rate: 70.0, sway: (3.0, 18.0, 28.0) },
    drive: Drive { yaw_rate: 120.0, steer_rate: 40.0, steer_mix: -0.5, steer_low: -0.3, bank: -40.0, bank_mix: 0.2, bank_ease: 0.05, back: 0.5, turn_push: 0.0, push: 45.0, top: 80.0 },
    settle: Settle { ground_slow: 0.99, spring: 0.1, brake: 0.05, drag: 0.99, across: 0.95 },
    strike: Strike { radius: (1.0, 1.5), down: -0.3, fast: 10.0, joint: 1 },
    power: 0x68,
    jump: 0x9c,
    damage: 10.0,
    most: 30.0,
    get_on: 2.0,
    hover: 1.5,
    hover_low: 0.0,
    sphere: 2.0,
    gravity: 14.0,
    rise: 50.0,
    jump_time: 180,
    rise_ticks: 30,
    pitch_in: 20.0,
    pitch_mix: 0.5,
    ease: (0.01, 0.05),
    strafe: 10.0,
    wall_slow: 0.99,
    power_step: (0.02, 0.01),
    anim: 2.0,
    seat: (0, 0.1),
    brake_levels: &[8, 0x10, 0x15, 0x1e],
};

/// `0x3b5a80` / `0x40bb20` (module docs): (turn, throttle, pitch, brake, strafe, jump).
fn controls(ctx: &Ctx, brake_levels: &[i32]) -> (f32, f32, f32, f32, f32, f32) {
    let h = ctx.hero.items.held;
    // `0x3220d0`: `1 − (1 − p)²` of a button's pressure (here 0 or 1).
    let b = |m: u32| -> f32 { if h & m != 0 { 1.0 } else { 0.0 } };
    let turn = (-ctx.hero.stick[0]).clamp(-1.0, 1.0);
    let throttle = (b(button::CROSS) - b(button::CIRCLE)).clamp(-1.0, 1.0);
    let strafe = (b(button::L2) - b(button::R2)).clamp(-1.0, 1.0);
    let square = ctx.hero.vehicle.square;
    let (brake, jump) = if brake_levels.contains(&ctx.game.level) { (square, 0.0) } else { (0.0, square) };
    (turn, throttle, -ctx.hero.stick[1], brake, strafe, jump)
}

fn class_solid(w: &MobyWorld, class: i16) -> bool { w.class(class).is_some_and(|k| k.has_collision) }

/// State 0 (module docs; the HUD's words by the caller).
fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, c: &Craft) {
    let m = &mut w.mobys[id];
    m.set_pv_v(pv::HOME, m.pos);
    m.set_pv_v(pv::VEL, [0.0; 3]);
    m.set_pv_f(pv::HEALTH, c.most);
    ctx.hero.vehicle.health = c.most;
    ctx.hero.vehicle.max_health = c.most;
    m.set_state(1, -1);
    m.set_pv_f(c.power, 0.0);
    m.set_pv_i(c.jump, 0);
}

/// State 1's way on (module docs): None while Ratchet is not free, else whether he got on this tick.
fn approach(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, c: &Craft) -> Option<bool> {
    if ctx.hero.state == 0x62 || !hs::hero_ready(w, ctx) { return None; }
    let fresh = hs::offer(w, id, ctx, c.get_on);
    if fresh { hs::board(w, id, ctx, RIDE_SEQ); }
    Some(fresh)
}

/// State 3's power and vacuum (module docs): true while ridden (the flight then steered).
fn power_and_vacuum(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, c: &Craft) -> bool {
    let mut power = w.mobys[id].pv_f(c.power);
    let ridden = w.mobys[id].state == 3;
    if !ridden {
        power -= c.power_step.0;
    } else {
        if ctx.hero.items.held & button::L1 == 0 {
            power += c.power_step.1;
        } else {
            power -= c.power_step.0;
            if ctx.game.rng.randi(9) == 0 || ctx.hero.items.pressed & button::L1 != 0 { hs::vacuum(w, id); }
        }
        ctx.hero.anim_req = Some((15.0, RIDE_SEQ, 0));
    }
    w.mobys[id].set_pv_f(c.power, power);
    ridden
}

fn clamp_power(w: &mut MobyWorld, id: usize, c: &Craft) {
    let m = &mut w.mobys[id];
    let p = m.pv_f(c.power).clamp(0.0, 1.0);
    m.set_pv_f(c.power, p);
}

/// The flight (module docs): `input` steered.
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, c: &Craft, input: bool) {
    let (turn, mut throttle, mut pitch_in, mut brake, mut strafe, mut jump) = if input { controls(ctx, c.brake_levels) } else { (0.0, 0.0, 0.0, 0.0, 0.0, 0.0) };
    let m = &mut w.mobys[id];
    let power = m.pv_f(c.power);
    let rows = m.rows;
    throttle *= power;
    brake += 1.0 - power;
    strafe *= power;
    jump *= power;
    m.set_pv_f(pv::THROTTLE, throttle);
    let hover = c.hover * power + c.hover_low * (1.0 - power);
    pitch_in *= DEG * c.pitch_in;
    let mut vel = m.pv_v(pv::VEL);
    let speed = len(vel) * power;
    let (yaw, roll, throttle, acc) = hs::drive(m, &c.drive, pv::ROLL, pv::YAW, power, speed, turn, throttle);
    m.set_pv_f(pv::YAW, yaw);
    m.set_pv_f(pv::ROLL, roll);
    // `+0x48`: the playback speed.
    let s = ctx.host.anim(&w.mobys[id]).map_or(0.0, |a| a.speed);
    ctx.host.set_anim_speed(&w.mobys[id], s + (throttle * c.anim - s) * 0.02);
    let pos = w.mobys[id].pos;
    let mut next = add(pos, vel);
    let (height, special, normal) = hs::ground_below(ctx, next, id);
    let low = height + hover;
    if pos[2] < low + 0.3 && special { w.mobys[id].set_state(5, -1); }
    let m = &mut w.mobys[id];
    let timer = m.pv_i(c.jump);
    vel[2] += if c.jump_time - c.rise_ticks < timer { c.rise } else { -c.gravity } * PER_TICK2;
    if height != 0.0 {
        let mut push = true;
        let pitch = m.pv_f(pv::PITCH);
        if low < next[2] {
            m.set_pv_f(pv::PITCH, add_rot(pitch, sub_rot(pitch_in, pitch) * c.ease.0));
            push = ctx.game.level != 8;
        } else {
            if timer == 0 && 0.2 < jump { m.set_pv_i(c.jump, c.jump_time); }
            (vel, next[2]) = hs::settle(&c.settle, vel, next[2], low, height, special, brake, &rows);
            let to = add_rot(-hs::slope(&rows, normal), pitch_in * 0.5 * c.pitch_mix * (power + 1.0));
            m.set_pv_f(pv::PITCH, add_rot(pitch, sub_rot(to, pitch) * c.ease.1));
        }
        if push {
            let thrust = set_len(rows[0], acc);
            let side = set_len(rows[1], c.strafe * strafe * PER_TICK2 * power);
            vel = add(vel, add(thrust, side));
        }
    }
    // The hits' template (`0x3238b8`) is made here, along this velocity.
    let along = vel;
    m.pos = next;
    m.set_pv_v(pv::VEL, vel);
    // The sphere above joint 0.
    let mut ctr = ctx.host.joint(&w.mobys[id], 0).unwrap_or(next);
    ctr[2] += c.sphere - c.hover_low;
    let back = sub(next, ctr);
    let hit = ctx.coll.and_then(|coll| coll_sphere_m(coll, None, ctr, c.sphere, QueryFlags(0), Some(id)).or_else(|| coll_sphere_m(coll, ctx.scene, ctr, c.sphere, QueryFlags(2), Some(id))));
    if let Some(hit) = hit {
        if matches!(hit.surface_id(), 1 | 0xb) { w.mobys[id].set_state(5, -1); }
        if !(hit.moby.is_some() && hit.moby == ctx.hero_moby) {
            let mut pushed = hit.pushed_centre.unwrap_or(ctr);
            for k in 0..2 { pushed[k] = (pushed[k] - ctr[k]) * power + ctr[k]; }
            let m = &mut w.mobys[id];
            m.pos = add(pushed, back);
            let vel = hs::slide(m.pv_v(pv::VEL), hit.normal, power, c.wall_slow);
            m.set_pv_v(pv::VEL, vel);
        }
    }
    hs::hits_on(w, id, ctx, &c.strike, c.damage, along);
}

/// The bar while ridden, both crafts (module docs).
fn bar(id: usize, ctx: &mut Ctx, c: &Craft) {
    let v = &mut ctx.hero.vehicle;
    if v.moby == Some(id) { v.bar = (v.health / c.most * 200.0) as i32; }
}

/// After the states (module docs): the bob and sway; ridden, Ratchet in his seat; the shadow.
fn after(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, c: &Craft) {
    let m = &mut w.mobys[id];
    let power = m.pv_f(c.power);
    hs::bob_sway(m, &c.bob, power);
    if m.state == 3 {
        let m = &w.mobys[id];
        let mut at = ctx.host.joint(m, c.seat.0).unwrap_or(m.pos);
        at[2] += c.seat.1;
        let rot = m.rot;
        ctx.hero.ride_place(at, rot);
        // `0x2ee5d8(0x3b4e98 / 0x40b478, moby)`: its HUD on the second draw list ([`draw_hud`]).
        w.register_draw(id);
    }
}

fn shadow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    crate::shadow_volumes::probe_near(w, id, ctx, 64.0, 0x3c);
}

/// 0x085c (module docs).
pub fn update_085c(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let c = &C085C;
    hs::prologue(w, id, ctx, &c.hull);
    bar(id, ctx, c);
    match w.mobys[id].state {
        0 => {
            start(w, id, ctx, c);
            let m = &mut w.mobys[id];
            m.set_pv_i(e::HUD_BAR, 0xff);
            m.set_pv_f(e::HUD_EASE, 0.0);
            m.set_pv_i(e::SOUND, 0);
        }
        s @ (1 | 3) => {
            let fresh = if s == 1 { approach(w, id, ctx, c) } else { Some(false) };
            if let Some(fresh) = fresh {
                ride_085c(w, id, ctx, c, fresh);
            }
        }
        4 => {
            w.mobys[id].has_collision = class_solid(w, EARLY);
            ctx.hero.ride_off();
            ctx.hero.pos[2] += 1.0;
            ctx.hero.vehicle.word_654 = 0xffff;
            off_085c(&mut w.mobys[id]);
            w.mobys[id].set_state(1, -1);
        }
        5 => {
            w.mobys[id].has_collision = class_solid(w, EARLY);
            ctx.hero.ride_off();
            hs::wreck(w, id, ctx);
            ctx.hero.vehicle.word_654 = 0xffff;
            off_085c(&mut w.mobys[id]);
        }
        _ => {}
    }
    after(w, id, ctx, c);
    w.mobys[id].has_collision = class_solid(w, EARLY);
    shadow(w, id, ctx);
    let m = &mut w.mobys[id];
    m.set_pv_i(e::NEAR, 0);
    m.set_pv_f(e::NEAR_D, 1_000_000.0);
}

/// 0x085c's ways off: its sound (none on the museum) and the HUD state cleared.
fn off_085c(m: &mut crate::moby::Moby) {
    m.set_pv_i(e::SOUND, 0);
    m.set_pv_s(e::HUD_88, 0);
    m.set_pv_b(e::HUD_8A, 0);
}

/// 0x085c's states 1 (Ratchet free) and 3 (module docs).
fn ride_085c(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, c: &Craft, fresh: bool) {
    let m = &mut w.mobys[id];
    m.has_collision = false;
    let mut t = m.pv_s(e::GUN_REST) as i32;
    crate::moby::countdown(&mut t);
    m.set_pv_s(e::GUN_REST, t as i16);
    let mut j = m.pv_i(c.jump);
    crate::moby::countdown(&mut j);
    m.set_pv_i(c.jump, j);
    if m.state == 3 && ctx.hero.items.held & button::R1 != 0 && m.pv_s(e::GUN_REST) == 0 {
        // The side's gun's timer byte (none: module docs).
        let side = m.pv_s(e::GUN_SIDE) ^ 1;
        m.set_pv_s(e::GUN_SIDE, side);
        m.set_pv_s(e::GUN_REST, GUN_REST);
    }
    let input = power_and_vacuum(w, id, ctx, c);
    clamp_power(w, id, c);
    fly(w, id, ctx, c, input);
    if !fresh && ctx.hero.vehicle.moby == Some(id) && ctx.hero.items.pressed & button::TRIANGLE != 0 {
        w.mobys[id].set_state(4, -1);
    }
}

/// 0x0f2f (module docs).
pub fn update_0f2f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let c = &C0F2F;
    hs::prologue(w, id, ctx, &c.hull);
    bar(id, ctx, c);
    match w.mobys[id].state {
        0 => {
            start(w, id, ctx, c);
            let m = &mut w.mobys[id];
            m.set_pv_i(f::HUD_BAR, 0xff);
            m.set_pv_f(f::HUD_EASE, 0.0);
        }
        1 => {
            let v = w.mobys[id].pv_v(pv::VEL);
            if COAST * PER_TICK < (v[0] * v[0] + v[1] * v[1]).sqrt() { fly(w, id, ctx, c, false); }
            if approach(w, id, ctx, c) == Some(true) { w.mobys[id].has_collision = false; }
        }
        3 => {
            let input = power_and_vacuum(w, id, ctx, c);
            fly(w, id, ctx, c, input);
            clamp_power(w, id, c);
            if ctx.hero.vehicle.moby == Some(id) {
                if ctx.hero.items.pressed & button::TRIANGLE != 0 { w.mobys[id].set_state(4, -1); }
                // His centre `0x189f00`: a unit over his feet.
                ctx.hero.aim_point = add(ctx.hero.pos, [0.0, 0.0, 1.0]);
            }
        }
        4 => {
            if w.mobys[id].first() {
                ctx.hero.stick = [0.0, -1.0];
                ctx.hero.request_state(0xb, true);
                ctx.hero.vehicle.moby = None;
                ctx.hero.vehicle.word_654 = 0xffff;
            }
            w.mobys[id].has_collision = false;
            fly(w, id, ctx, c, false);
            let solid = class_solid(w, MUSEUM);
            let m = &mut w.mobys[id];
            let t = m.state_ticks;
            m.state_ticks = t.wrapping_add(1);
            if 0x1e < t as u16 {
                m.set_state(1, -1);
                m.has_collision = solid;
            }
        }
        5 => {
            w.mobys[id].has_collision = class_solid(w, MUSEUM);
            ctx.hero.ride_off();
            hs::wreck(w, id, ctx);
            ctx.hero.vehicle.word_654 = 0xffff;
        }
        _ => {}
    }
    after(w, id, ctx, c);
    shadow(w, id, ctx);
}

/// The HUD's words (0x085c `$gp+0x900..`, 0x0f2f `$gp+0x2650..`: the same values) and pvars.
mod hud {
    /// The dials' colour, their FX textures (+ the set) and the needles'.
    pub const DIAL: u32 = 0xff80_8080;
    pub const DIAL_FX: [i32; 2] = [0x3f, 0x3f];
    pub const NEEDLE_FX: i32 = 0x40;
    pub const NEEDLE: u32 = 0x6080_8080;
    /// The speed's sweep and start (degrees), the throttle's.
    pub const SPEED_DIAL: (f32, f32) = (270.0, 45.0);
    pub const THROTTLE_DIAL: (f32, f32) = (-270.0, -42.0);
    /// The dials' left edge; the speed the speed's dial is full at (a second).
    pub const LEFT: f32 = 16.0;
    pub const TOP_SPEED: f32 = 80.0;
    /// The texture set (`+0x64`).
    pub const SET: usize = 0x64;
}

/// `0x3b4e98` (0x085c) / `0x40b478` (0x0f2f): the ridden craft's HUD (module docs; crate::hud::list_screen).
/// * Its two dials: FX `0x3f` + the set (`+0x64`) twice, 64 × 64 (its 64 × 64 texels) at (16, 80 up from the bottom)
///   and 64 to the right of it, in `0xff808080`.
/// * Its needles: FX `0x40` + the set, 32 × 32 (its 32 × 32 texels) turned about (0.5, 0.35) of it at the dials'
///   centres (48 and 112 across, 48 up), in `0x60808080` — the speed's at `45° + 270°·|v| / 80` (|v| the velocity's
///   length a second, `0x2eefa0`), the throttle's at `−42° − 270°·|throttle|` (each `0x2efe60`).
/// * Its health bar's words: the shown health (`+0xa4` / `+0x94`) eased a fifth of the way to the health
///   (`0x18b65c`), a step at most 1; the bar's cut (`+0xa8` / `+0x98`) `2 + 251·shown / most` held to 2..0xfd (the
///   bar itself: module docs).
/// * Below a tenth of its most health every other 90 frames (`$gp+0x36d8`) the text 0x31d5 (font 2, centred at
///   (256, 370), `0x80000080`).
///
/// Not ported: 0x085c's level-8 block (the raritanium's texts and marker, the hovership's; the class is on the
/// museum only), the GS state words (`0x37ce68`).
pub fn draw_hud(w: &mut MobyWorld, id: usize, inp: &crate::hud::ScreenIn, out: &mut Vec<crate::hud::ScreenDraw>) {
    use crate::hud::{fx_rect, fx_turned, text_of, Align, ScreenDraw};
    let (ease_at, cut_at) = if w.mobys[id].o_class == EARLY { (e::HUD_EASE, e::HUD_BAR) } else { (f::HUD_EASE, f::HUD_BAR) };
    let m = &mut w.mobys[id];
    if m.pvars.len() < cut_at + 4 { return; }
    let set = m.pv_i(hud::SET);
    let sh = crate::gui::SCREEN_H as i32;
    let dials_y = (sh - 0x50) as f32;
    let x = hud::LEFT as i32;
    for (k, fx) in hud::DIAL_FX.into_iter().enumerate() {
        fx_rect(out, (fx + set) as u16, [(x + 0x40 * k as i32) as f32, dials_y, 64.0, 64.0], [0.0, 0.0, 64.0, 64.0], hud::DIAL);
    }
    let needle = (hud::NEEDLE_FX + set) as u16;
    let centre_y = (sh - 0x30) as f32;
    let speed = len(m.pv_v(pv::VEL)) * DEG;
    let a = add_rot(hud::SPEED_DIAL.0 * speed / (hud::TOP_SPEED * PER_TICK), hud::SPEED_DIAL.1 * DEG);
    fx_turned(out, needle, [hud::LEFT + 32.0, centre_y], [32.0, 32.0], a, [32.0, 32.0], [0.5, 0.35], hud::NEEDLE);
    let throttle = m.pv_f(pv::THROTTLE).abs() * DEG;
    let a = add_rot(hud::THROTTLE_DIAL.1 * DEG, hud::THROTTLE_DIAL.0 * throttle);
    fx_turned(out, needle, [hud::LEFT + 64.0 + 32.0, centre_y], [32.0, 32.0], a, [32.0, 32.0], [0.5, 0.35], hud::NEEDLE);
    let v = &inp.hero.vehicle;
    let ease = m.pv_f(ease_at);
    let mut d = (v.health - ease) * 0.2;
    if 1.0 < d.abs() { d /= d.abs(); }
    m.set_pv_f(ease_at, ease + d);
    let cut = (((ease + d) * 251.0 / v.max_health) as i32 + 2).min(0xfd).max(2);
    m.set_pv_i(cut_at, cut);
    if v.health < v.max_health * 0.1 && (w.frame / 90) & 1 != 0 {
        out.push(ScreenDraw::Text { font: 2, text: text_of(inp, 0x31d5, 100), pos: [256.0, 370.0], scale: 1.0, colour: 0x8000_0080, align: Align::Centre });
    }
}
