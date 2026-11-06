//! 0x0e5f (`0x3f83e8`, 1 placed; the decomp's v2.00 `func_US2_L07_003F83E8`): Siberius's boss, a hovering gunship
//! with two parts (0x0d41, its body's copy at its place; 0x0d45, the charge glow at its joint 0, [`update_0d45`]).
//! Hidden (1, sub 2) until Ratchet enters cuboid `+0x170`; then (the first time, the intro scene 0x4f, 2) it takes
//! its tier-2 fight over the arena cuboid `+0x174`: it circles Ratchet at 12, picks an attack, winds up, fires, and
//! between its health's sixths calls in a dropship (crate::moby::classes::level07::dropship::set_off). Beaten, the
//! scenes 0x50 / 0x51 / 8 / 0x52, planet 8 unlocked and the trip there (`0x3187f8`, `0x2eb1b8`).
//!
//! Earlier, tiers 0 / 1 ride a platform ([`set_up`] with one, sub 0: its offset `+0x3e0` beside it, retreating
//! (10, 11) when hit down a sixth) — the train class 0x11d9..0x11dd sets those up.
//!
//! **Each tick** ([`pre`], `0x3f8f90`): set-up in state 0 (gone once planet 8 is unlocked; its parts made; health 100,
//! or after a loss cut by up to half over the attempts 5..20, [`by_attempts`]; hidden, 1 sub 2). A hit's damage
//! (`0x10000`, while not hurt in tiers below 3) flashes it (`+0x160`) and comes off its health down to its floor
//! `+0x430` (below tier 2) or 0; reaching the floor: the next floor a sixth lower, the next tier, its drops `+0x458`
//! (1..4 by the attempts 3..7), state 10 (sub 0) or 12. Then the target: Ratchet within 16 (sub 0) or the occupant
//! of area `+0x188`.
//!
//! **States**: 3 fly in (to 4 after 120 ticks); 4 hover (60 ticks) and [`pick`] an attack; 5 wind up — the glow at
//! `end − glow` ticks, the aim taken 5 before the end, sound 8 45 before it for the laser 7 — and at its end the
//! attack; 6..9 the attack's sequence, fired at its frame 1 ([`fire`]), then 4 when its count `+0x438` is out, else
//! 5; 10 / 11 the riding tiers' retreat; 12 hurt (a dropship at tiers 4 / 5; at no health the arena's mobys within
//! 50 hit for 100, then 13); 13 the end scenes; 14 the trip.
//!
//! Not ported: the joints' aims (`0x31a318`, `0x31dba0`: blocks `+0x260` / `+0x2e0` / `+0x360`, the banks `+0x2c0` /
//! `+0x2c4`), the weapons' damage tally (`0x1fe3b8`), the global `$gp+0x3cc8` set to 40° on sub 0, `0x18c2c4` = 40
//! while dying, and the class word `+0x46` = 5.

use crate::hero::physics::{add, add_rot, atan2, dist2, dot, len, set_len, sub, sub_rot, V3};
use crate::moby::classes::level04::launcher::carry;
use crate::moby::classes::props::wrapped;
use crate::moby::{ai, countdown, Ctx, Moby, MobyWorld};

use super::{bomb, dropship, laser, turret};

pub const CLASS: i16 = 0x0e5f;
/// Its body's copy and its charge glow.
pub const BODY: i16 = 0x0d41;
pub const GLOW: i16 = 0x0d45;

/// pvars.
mod pv {
    pub const HP: usize = 0x20;
    pub const MAX: usize = 0x24;
    pub const KILL: usize = 0x2e;
    pub const DMG_LASER: usize = 0x34;
    pub const DMG_BEAM: usize = 0x35;
    pub const FLASH: usize = 0x160;
    pub const TRIGGER: usize = 0x170;
    pub const ARENA: usize = 0x174;
    pub const SHIP_A: usize = 0x178;
    pub const SHIP_B: usize = 0x17c;
    pub const TURRETS: usize = 0x180;
    pub const BOMBS: usize = 0x184;
    pub const AREA: usize = 0x188;
    pub const MUSIC: usize = 0x18c;
    pub const GOAL: usize = 0x3e0;
    pub const VEL: usize = 0x3f0;
    pub const AIM: usize = 0x400;
    pub const BODY: usize = 0x410;
    pub const GLOW: usize = 0x414;
    pub const PLATFORM: usize = 0x418;
    pub const TIER: usize = 0x420;
    pub const YAW_V: usize = 0x424;
    pub const ROLL_V: usize = 0x428;
    pub const HOVER: usize = 0x42c;
    pub const FLOOR: usize = 0x430;
    pub const BACK: usize = 0x434;
    pub const COUNT: usize = 0x438;
    pub const ATTACK: usize = 0x43c;
    pub const MOVE: usize = 0x440;
    pub const END: usize = 0x444;
    pub const CHARGE: usize = 0x448;
    pub const BOB: usize = 0x454;
    pub const DROPS: usize = 0x458;
}

/// `$gp+0x1ff0` / `+0x2000`: the riding tiers' offsets beside the platform.
const TIER_OFF: [V3; 2] = [[-20.0, 0.0, 6.0], [20.0, 0.0, 8.0]];
/// `$gp+0x2014`: the aim is taken this many ticks before the wind-up's end.
const AIM_AHEAD: i32 = 5;
/// `$gp+0x2030`: where it starts, in the camera's frame.
const START: V3 = [0.0, 3.0, 0.0];
/// `$gp+0x2040..+0x204c`: the steering (a second², /3600) and top speed (a second), riding and free.
const STEER: [(f32, f32); 2] = [(80.0, 80.0), (30.0, 30.0)];
/// `$gp+0x2050`: the roll toward its sideways speed; `+0x2054` / `+0x2058` its ease (degrees a second²) and top rate.
const BANK: f32 = -5.0;
const ROLL: (f32, f32) = (360.0, 720.0);
/// `$gp+0x2060`: the riding offset's lean toward Ratchet; `+0x2064` / `+0x2068` the free fight's band (xy); `+0x206c`
/// its circle; `+0x2074` its height over the arena; `+0x2078` / `+0x207c` the move timer (ticks, by the attempts);
/// `+0x2080` / `+0x2084` the bob (height, degrees a second).
const LEAN: f32 = -10.0;
const BAND: (f32, f32) = (6.0, 18.0);
const CIRCLE: f32 = 12.0;
const HEIGHT: f32 = 1.5;
const MOVE_T: (i32, i32) = (120, 300);
const BOB: (f32, f32) = (0.2, 300.0);
/// `$gp+0x2088` / `+0x208c`: the follow camera's distance and pivot height asked for (`0x422648`, `0x422690`);
/// `+0x2090` the tweak's yaw rate.
const CAM: (f32, f32, i32) = (8.0, 3.0, 5);
/// `$gp+0x2098..+0x20b0`: each attack's wind-up end and glow, ticks (6, 7, 9).
const WIND: [(i32, i32, i32); 3] = [(6, 120, 45), (7, 120, 30), (9, 90, 45)];

const HIDE: u16 = 0x41;
const SIXTH: f32 = 0.166_944_92;

/// Level 7's globals the boss keeps.
#[derive(Clone, Copy, Debug, Default)]
pub struct Globals {
    /// `$gp+0x2018`: the fights lost (counted on the set-up after one).
    pub attempts: i32,
    /// `$gp+0x201c`: a fight began since the last set-up.
    pub active: bool,
    /// `0x13962a`: the intro scene seen (the port keeps it for the session).
    pub intro_seen: bool,
    /// The camera tweak of the free fight (`+0x450`), +1.
    pub tweak: usize,
}

/// `0x3fa488(a, b, f0, f1)`: `f0` at `a` attempts or fewer, `f1` at `b` or more, linear between.
fn by_attempts(w: &MobyWorld, a: i32, b: i32, f0: f32, f1: f32) -> f32 {
    let n = w.level07.attempts;
    if n <= a { return f0; }
    if b <= n { return f1; }
    f0 + (f1 - f0) * ((n - a) as f32 / (b - a) as f32)
}

fn cuboid_pos(ctx: &Ctx, i: i32) -> V3 { ctx.level.cuboid_place(i).map_or([0.0; 3], |c| c.0) }

fn through(v: V3, r: &[V3; 3]) -> V3 { std::array::from_fn(|i| v[0] * r[0][i] + v[1] * r[1][i] + v[2] * r[2][i]) }
fn into(v: V3, r: &[V3; 3]) -> V3 { [dot(v, r[0]), dot(v, r[1]), dot(v, r[2])] }

fn seqs(w: &MobyWorld, id: usize, ctx: &Ctx) -> (u8, u8) { ctx.host.anim(&w.mobys[id]).map_or((0xff, 0xff), |a| (a.seq_a, a.seq_b)) }

/// `0x3f8c98`: it and its parts hidden without collision.
fn hide_all(w: &mut MobyWorld, id: usize) {
    let parts = [ai::link(&w.mobys[id], pv::BODY), ai::link(&w.mobys[id], pv::GLOW)];
    for k in std::iter::once(Some(id)).chain(parts) {
        let Some(m) = k.and_then(|k| w.mobys.get_mut(k)) else { continue };
        m.mode = (m.mode | HIDE) & 0xefff;
        m.has_collision = false;
    }
}

/// `0x3f8d08(m, platform, tier)`: shown with its class's collision (and its parts); riding `platform` at its tier's
/// offset (state 3, sub 0) or free (3, sub 1); the tally cleared on the first; its health set to its tier's sixths
/// and its floor a sixth lower; its angles' x / y 0; placed [`START`] in the camera's frame; its velocity stopped; a
/// tier ≥ 0 kept with its drops (4..1 by the attempts 3..7).
pub fn set_up(w: &mut MobyWorld, ctx: &mut Ctx, id: usize, platform: Option<usize>, tier: i32) {
    let parts = [ai::link(&w.mobys[id], pv::BODY), ai::link(&w.mobys[id], pv::GLOW)];
    for k in std::iter::once(Some(id)).chain(parts) {
        let Some(k) = k.filter(|&k| k < w.mobys.len()) else { continue };
        let has = w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision);
        let m = &mut w.mobys[k];
        m.mode = (m.mode & !HIDE & 0xff7f) | 0x1000;
        m.has_collision = has;
    }
    let m = &mut w.mobys[id];
    match platform {
        Some(p) => {
            m.set_pv_v(pv::GOAL, TIER_OFF[tier.clamp(0, 1) as usize]);
            ai::set_link(m, pv::PLATFORM, Some(p));
            m.set_state(3, 0);
        }
        None => {
            ai::set_link(m, pv::PLATFORM, None);
            m.set_state(3, 1);
        }
    }
    let most = m.pv_s(pv::MAX) as f32;
    m.set_pv_b(pv::KILL, 1);
    m.set_pv_f(pv::FLOOR, (most - most * ((tier + 1) as f32 * SIXTH)).max(0.0));
    m.set_pv_f(pv::HP, most - most * (tier as f32 * SIXTH));
    m.rot[0] = 0.0;
    m.rot[1] = 0.0;
    m.pos = add(through(START, &ctx.host.camera_rows()), ctx.host.camera_pos());
    m.set_pv_v(pv::VEL, [0.0; 3]);
    if 0 <= tier {
        let drops = by_attempts(w, 3, 7, 4.0, 1.0) as i32;
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::TIER, tier);
        m.set_pv_i(pv::DROPS, drops);
    }
    w.changed = true;
}

pub fn update_0e5f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !pre(w, id, ctx) { return; }
    let sub_s = w.mobys[id].sub_state;
    let state = w.mobys[id].state;
    match sub_s {
        0 | 1 => if state != 3 && state != 11 { camera(w, id, ctx) },
        2 => if 0 < w.level07.tweak {
            ctx.game.cam_tweaks.release(w.level07.tweak - 1);
            w.level07.tweak = 0;
        },
        _ => {}
    }
    let ticks = w.mobys[id].state_ticks as i32;
    match state {
        1 => {
            if ctx.level.in_cuboid(w.mobys[id].pv_i(pv::TRIGGER), ctx.hero.pos) {
                w.level07.active = true;
                let mut scene = false;
                if !w.level07.intro_seen && ctx.game.scenes.queue.push(0x4f) {
                    ctx.game.scenes.start_queue(ctx.hero.hp);
                    w.mobys[id].set_state(2, -1);
                    scene = true;
                }
                if !scene { begin(w, id, ctx); }
                ctx.host.music_request(w.mobys[id].pv_i(pv::MUSIC), -1);
            }
        }
        2 => if !ctx.game.scenes.in_scene() {
            w.level07.intro_seen = true;
            if let Some(c) = usize::try_from(w.mobys[id].pv_i(pv::ARENA)).ok().and_then(|i| ctx.level.cuboids.get(i)) {
                let centre = [c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]];
                let at = add(set_len([c.matrix[1][0], c.matrix[1][1], c.matrix[1][2]], 8.0), centre);
                ctx.hero.teleport(at, [0.0, 0.0, atan2(centre[0] - at[0], centre[1] - at[1])]);
            }
            begin(w, id, ctx);
        },
        3 => {
            follow(w, id, ctx, -1.0);
            if 121 <= ticks { w.mobys[id].set_state(4, -1); }
        }
        4 => {
            if w.mobys[id].first() {
                hover_seq(w, id, ctx);
                w.mobys[id].set_pv_i(pv::HOVER, 60);
            }
            follow(w, id, ctx, -1.0);
            if w.mobys[id].pv_i(pv::HOVER) == 0 {
                let a = pick(w, id, ctx);
                let m = &mut w.mobys[id];
                m.set_pv_i(pv::ATTACK, a);
                m.set_state(5, -1);
            }
        }
        5 => {
            if w.mobys[id].first() { hover_seq(w, id, ctx); }
            follow(w, id, ctx, -1.0);
            let m = &w.mobys[id];
            let (end, charge) = (m.pv_i(pv::END), m.pv_i(pv::CHARGE));
            if ticks == end - charge {
                if let Some(g) = ai::link(m, pv::GLOW).filter(|&g| g < w.mobys.len()) { glow_start(&mut w.mobys[g], charge, charge + 10); }
            } else if ticks == end - AIM_AHEAD {
                take_aim(w, id);
            } else if end < ticks {
                let a = w.mobys[id].pv_i(pv::ATTACK);
                w.mobys[id].set_state(a as u8, -1);
            }
            if w.mobys[id].pv_i(pv::ATTACK) == 7 && ticks == end - 45 { ctx.host.play_sound(&w.mobys[id], 8, 0); }
        }
        6..=9 => {
            if w.mobys[id].first() {
                let s = if w.mobys[id].pv_i(pv::TIER) == 0 { 3 } else { 9 };
                if seqs(w, id, ctx).1 != s { ctx.host.set_anim(&w.mobys[id], s, 0, 10); }
            }
            follow(w, id, ctx, -1.0);
            let (a, b) = seqs(w, id, ctx);
            if a == b {
                if ai::reached(w, id, ctx, 1.0) {
                    fire(w, id, ctx, state);
                } else if wrapped(w, id, ctx) {
                    let mut n = w.mobys[id].pv_i(pv::COUNT);
                    let out = countdown(&mut n);
                    let m = &mut w.mobys[id];
                    m.set_pv_i(pv::COUNT, n);
                    m.set_state(if out { 4 } else { 5 }, -1);
                }
            }
        }
        10 => {
            if w.mobys[id].first() && seqs(w, id, ctx).1 != 7 { ctx.host.set_anim(&w.mobys[id], 7, 0, 20); }
            follow(w, id, ctx, -1.0);
            if wrapped(w, id, ctx) { w.mobys[id].set_state(11, -1); }
        }
        11 => {
            if w.mobys[id].first() {
                if seqs(w, id, ctx).1 != 4 { ctx.host.set_anim(&w.mobys[id], 4, 0, 20); }
                w.mobys[id].set_pv_f(pv::BACK, 1.083_333_4);
            }
            let back = w.mobys[id].pv_f(pv::BACK);
            follow(w, id, ctx, back);
            let m = &mut w.mobys[id];
            let mut b = m.pv_f(pv::BACK);
            crate::hero::physics::approach(0.0, 0.005_555_556, &mut b);
            m.set_pv_f(pv::BACK, b);
            let mut z = m.pv_f(pv::GOAL + 8);
            crate::hero::physics::approach(18.0, 0.033_333_335, &mut z);
            m.set_pv_f(pv::GOAL + 8, z);
            if m.drawn == 0 {
                m.set_state(1, 2);
                hide_all(w, id);
            }
        }
        12 => {
            if w.mobys[id].first() {
                if seqs(w, id, ctx).1 != 7 { ctx.host.set_anim(&w.mobys[id], 7, 0, 20); }
                let m = &mut w.mobys[id];
                m.set_pv_f(pv::GOAL + 8, m.pv_f(pv::GOAL + 8) + 1.0);
                m.set_pv_i(pv::MOVE, 600);
                if m.pv_f(pv::HP) == 0.0 { crate::moby::bolts::drop(w, id, 10, 3, ctx); }
            }
            follow(w, id, ctx, -1.0);
            if w.mobys[id].pv_f(pv::HP) <= 0.0 { clear_arena(w, id, ctx); }
            if wrapped(w, id, ctx) {
                if w.mobys[id].pv_f(pv::HP) <= 0.0 {
                    w.mobys[id].set_state(13, 2);
                } else {
                    let ship = match w.mobys[id].pv_i(pv::TIER) { 4 => Some(pv::SHIP_A), 5 => Some(pv::SHIP_B), _ => None };
                    if let Some(k) = ship.and_then(|o| usize::try_from(w.mobys[id].pv_i(o)).ok()).filter(|&k| k < w.mobys.len()) {
                        dropship::set_off(w, k, ctx);
                    }
                    let m = &mut w.mobys[id];
                    m.set_state(4, -1);
                    m.set_pv_i(pv::MOVE, 0);
                }
            }
        }
        13 => {
            let q = &mut ctx.game.scenes.queue;
            if q.push(0x50) {
                q.push(0x51);
                q.push(8);
                q.push(0x52);
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(14, -1);
            }
        }
        14 => {
            if w.mobys[id].mode & 1 == 0 && ctx.game.scenes.mode == crate::scene::SCENE { hide_all(w, id); }
            clear_arena(w, id, ctx);
            if !ctx.game.scenes.in_scene() && ctx.game.scenes.mode != 1 && ctx.game.scenes.mode != crate::scene::SCENE && ctx.hero.state != 20 {
                ctx.game.unlock_planet(8);
                ctx.game.go_to_level(8, true);
            }
        }
        _ => {}
    }
    tail(w, id, ctx);
}

/// States 1 / 2: the free fight at tier 2, its goal over the arena 1.5 up.
fn begin(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    set_up(w, ctx, id, None, 2);
    let mut at = cuboid_pos(ctx, w.mobys[id].pv_i(pv::ARENA));
    at[2] += HEIGHT;
    w.mobys[id].set_pv_v(pv::GOAL, at);
}

/// `0x313428(m, arena, 0x10000, 50, 100, 0, …)`: everything within 50 of the arena hit for 100.
fn clear_arena(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let at = cuboid_pos(ctx, w.mobys[id].pv_i(pv::ARENA));
    ai::attack_sphere(w, id, ctx, 50.0, 100.0, 0.0, at, 0x10000, 0, 1, 0);
}

/// `0x3fa3c8`: the hover sequence (2, or 8 from tier 1), blended over 10.
fn hover_seq(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = if w.mobys[id].pv_i(pv::TIER) == 0 { 2 } else { 8 };
    if seqs(w, id, ctx).1 != s { ctx.host.set_anim(&w.mobys[id], s, 0, 10); }
}

/// `0x3f8f90` (module docs). False when it went.
fn pre(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        if ctx.game.planets.get(8).is_some_and(|&p| p != 0) {
            w.mobys[id].set_state(1, 2);
            w.delete(id);
            return false;
        }
        let m = &mut w.mobys[id];
        let r = ai::react(m);
        m.set_pv_b(r + 0x48, 30);
        m.set_pv_b(r + 0x4a, 11);
        m.set_pv_b(pv::KILL, 1);
        m.draw_dist = 0xff;
        for (class, at, bits) in [(BODY, pv::BODY, 0x400), (GLOW, pv::GLOW, 0x100)] {
            if w.class(class).is_none() { continue; }
            let c = w.create(class);
            let p = &mut w.mobys[c];
            p.parent = Some(id);
            p.mode |= bits;
            p.update_dist = 0xff;
            ai::set_link(&mut w.mobys[id], at, Some(c));
        }
        if w.mobys[id].pv_s(pv::MAX) == 0 {
            w.mobys[id].set_pv_s(pv::MAX, 100);
            w.mobys[id].set_pv_f(pv::HP, 100.0);
        } else {
            let k = 1.0 - by_attempts(w, 5, 20, 0.0, 0.5);
            let m = &mut w.mobys[id];
            m.set_pv_s(pv::MAX, (k * m.pv_s(pv::MAX) as f32) as i16);
            m.set_pv_f(pv::HP, m.pv_f(pv::HP) * k);
        }
        w.mobys[id].mode |= 0x5000;
        if w.level07.active { w.level07.attempts += 1; }
        w.level07.active = false;
        w.mobys[id].set_state(1, 2);
        hide_all(w, id);
        w.changed = true;
    }
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
    let dmg = ai::damage_of(w, id, hit.as_ref(), false);
    let m = &w.mobys[id];
    let tier = m.pv_i(pv::TIER);
    if (3 <= tier || !(10..15).contains(&m.state)) && (dmg != 0.0 || m.pv_b(pv::KILL) == 2) {
        let m = &mut w.mobys[id];
        ai::flash_start(m, pv::FLASH);
        let floor = if tier < 2 { m.pv_f(pv::FLOOR) } else { 0.0 };
        let mut hp = m.pv_f(pv::HP);
        crate::hero::physics::approach(floor, dmg, &mut hp);
        if m.pv_b(pv::KILL) == 2 { hp = if tier <= 1 { m.pv_f(pv::FLOOR) } else { 0.0 }; }
        m.set_pv_f(pv::HP, hp);
        if hp <= m.pv_f(pv::FLOOR) {
            let most = m.pv_s(pv::MAX) as f32;
            let mut f = most - most * ((tier + 2) as f32 * SIXTH);
            m.set_pv_i(pv::TIER, tier + 1);
            let drops = by_attempts(w, 3, 7, 1.0, 4.0) as i32;
            let m = &mut w.mobys[id];
            m.set_pv_i(pv::DROPS, drops);
            if m.pv_f(pv::HP) - 1.0 < f { f = m.pv_f(pv::HP) - 1.0; }
            m.set_pv_f(pv::FLOOR, f.max(0.0));
            m.set_state(if m.sub_state == 0 { 10 } else { 12 }, -1);
        }
    }
    w.clear_hit(id);
    w.mobys[id].set_pv_b(pv::KILL, 1);
    if w.mobys[id].sub_state == 0 {
        let hero = ctx.hero.pos;
        ai::find_target(w, id, ctx, 16.0, 4.0, -1, Some(hero), false);
    } else {
        let area = w.mobys[id].pv_i(pv::AREA);
        ai::find_target(w, id, ctx, 0.0, 0.0, area, None, false);
    }
    true
}

/// `0x3f9b20`: the follow camera asked for [`CAM`]'s distance and pivot; free, a tweak slot (priority 5) holding them.
fn camera(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.game.cam_tweaks.follow_dist = Some((CAM.0, 0.003));
    ctx.game.cam_tweaks.follow_pivot = Some((CAM.1, 0.003));
    if w.mobys[id].sub_state == 0 { return; }
    if w.level07.tweak == 0 { w.level07.tweak = ctx.game.cam_tweaks.request(5).map_or(0, |s| s + 1); }
    if let Some(s) = w.level07.tweak.checked_sub(1).and_then(|s| ctx.game.cam_tweaks.slots.get_mut(s)) {
        s.dist = CAM.0;
        s.pivot_h = CAM.1;
        s.rate_deg = CAM.2;
    }
}

/// `0x3f9ca8` (module docs): the attack after this wind-up and its wind-up's end, glow and count; tier 0 the laser
/// 7, tier 1 the laser or the beam 6 at random, then by `randi(tier + 1)`: 7, 6, the drop 8 while drops are left
/// (one used) else 6 or 7, or the bomb 9.
fn pick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> i32 {
    let tier = w.mobys[id].pv_i(pv::TIER);
    let g = &mut ctx.game.rng;
    let a = match tier {
        0 => 7,
        1 => if g.randi(0xff) & 1 != 0 { 6 } else { 7 },
        _ => match g.randi(tier + 1) {
            1 => 6,
            2 => {
                let m = &mut w.mobys[id];
                if m.pv_i(pv::DROPS) != 0 {
                    m.set_pv_i(pv::DROPS, m.pv_i(pv::DROPS) - 1);
                    8
                } else if ctx.game.rng.randi(2) != 0 { 7 } else { 6 }
            }
            3 => 9,
            _ => 7,
        },
    };
    let g = &mut ctx.game.rng;
    let m = &mut w.mobys[id];
    if a == 8 {
        m.set_pv_i(pv::COUNT, 1);
    } else if let Some(&(_, end, charge)) = WIND.iter().find(|x| x.0 == a) {
        let n = match a { 6 => g.rand_range(2, 3), 7 => g.rand_range(3, 5), _ => g.rand_range(1, 3) };
        m.set_pv_i(pv::END, end);
        m.set_pv_i(pv::CHARGE, charge);
        m.set_pv_i(pv::COUNT, n);
    }
    a
}

/// `0x3f9bc8`: the aim taken at the target (riding: in the platform's frame).
fn take_aim(w: &mut MobyWorld, id: usize) {
    let m = &w.mobys[id];
    let t = m.pv_v(ai::target(m));
    let aim = match (m.sub_state, ai::link(m, pv::PLATFORM).and_then(|p| w.mobys.get(p))) {
        (0, Some(p)) => into(sub(t, p.pos), &p.rows),
        _ => t,
    };
    w.mobys[id].set_pv_v(pv::AIM, aim);
}

/// `0x3f9c38`: the aim in the world.
fn aim(w: &MobyWorld, id: usize) -> V3 {
    let m = &w.mobys[id];
    let a = m.pv_v(pv::AIM);
    match (m.sub_state, ai::link(m, pv::PLATFORM).and_then(|p| w.mobys.get(p))) {
        (0, Some(p)) => add(through(a, &p.rows), p.pos),
        _ => a,
    }
}

/// The attack at its sequence's frame 1: 7 the laser (`0x3f9e50`: sound 1, from joint 0 to 2 past the aim or what
/// the line meets first, its damage byte `+0x34`); 6 the beam (`0x3f9f50`: sound 3, joint 0 to the aim, `+0x35`); 9
/// a waiting bomb launched (`0x3f9fd8`: sound 5, hovering 0.5 over the arena); 8 every waiting turret of `+0x180`
/// woken and carried in from joint 0 (`0x3fa070`: lifted 2 along its facing, landing from 5 up, over 90 ticks).
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, attack: u8) {
    let from = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let platform = ai::link(&w.mobys[id], pv::PLATFORM);
    match attack {
        7 => {
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            let to = aim(w, id);
            let d = sub(to, from);
            let mut end = add(set_len(d, len(d) + 2.0), from);
            if let Some(o) = w.coll_line(ctx, from, end, 2, Some(id)) { end = o.point; }
            let dmg = w.mobys[id].pv_b(pv::DMG_LASER) as f32;
            laser::spawn(w, ctx, id, platform, from, end, dmg);
        }
        6 => {
            ctx.host.play_sound(&w.mobys[id], 3, 0);
            let to = aim(w, id);
            let dmg = w.mobys[id].pv_b(pv::DMG_BEAM) as f32;
            laser::spawn(w, ctx, id, platform, from, to, dmg);
        }
        9 => {
            let Some(b) = bomb::waiting(w, w.mobys[id].pv_i(pv::BOMBS)) else { return };
            ctx.host.play_sound(&w.mobys[id], 5, 0);
            let h = cuboid_pos(ctx, w.mobys[id].pv_i(pv::ARENA))[2] + 0.5;
            bomb::launch(w, ctx, b, id, from, h);
        }
        8 => {
            let up = set_len(w.mobys[id].rows[0], 2.0);
            let group = w.mobys[id].pv_i(pv::TURRETS);
            while let Some(t) = random_waiting(w, ctx, group) {
                turret::wake(w, t, ctx);
                let (to, rot) = (w.mobys[t].pos, w.mobys[t].rot);
                carry(w, ctx, t, from, up, to, [0.0, 0.0, 5.0], rot, 0x5a, None, None);
            }
        }
        _ => {}
    }
}

/// `0x3f7988(group)`: one of the group's waiting (state 1) at random — `randi(n)` counted down from the first, so
/// the first is twice as likely.
fn random_waiting(w: &MobyWorld, ctx: &mut Ctx, group: i32) -> Option<usize> {
    let g = w.groups.get(usize::try_from(group).ok()?)?;
    let waiting: Vec<usize> = g.iter().map(|&i| i as usize).filter(|&i| w.mobys.get(i).is_some_and(|m| m.state == 1)).collect();
    if waiting.is_empty() { return None; }
    let k = ctx.game.rng.randi(waiting.len() as i32);
    waiting.get((k - 1).max(0) as usize).copied()
}

/// `0x3f97d0`: its goal. Riding: beside the platform toward Ratchet (`+0x3e0` x toward his local x ∓ 10 by 1/6,
/// within ±20; y toward his by 1/6, within ±5). Free: on a fresh move (its move timer out — 120..300 by the attempts
/// 5..15, plus 0..60 — or Ratchet outside 6..18 of it, xy, once within 2 of the goal) a point 12 from Ratchet,
/// 10..40° round from the camera's yaw on its side; its height over the arena bobbing.
fn place(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let target = m.pv_v(ai::target(m));
    if m.sub_state == 0 {
        let Some(p) = ai::link(m, pv::PLATFORM).and_then(|p| w.mobys.get(p)) else { return };
        let v = into(sub(target, p.pos), &p.rows);
        let lean = if m.pv_i(pv::TIER) != 0 { -LEAN } else { LEAN };
        let mut g = m.pv_v(pv::GOAL);
        crate::hero::physics::approach(v[0] + lean, 1.0 / 6.0, &mut g[0]);
        g[0] = g[0].clamp(TIER_OFF[0][0], TIER_OFF[1][0]);
        crate::hero::physics::approach(v[1], 1.0 / 6.0, &mut g[1]);
        g[1] = g[1].clamp(-5.0, 5.0);
        w.mobys[id].set_pv_v(pv::GOAL, g);
        return;
    }
    let mv = m.pv_i(pv::MOVE);
    if mv < 0 {
        w.mobys[id].set_pv_i(pv::MOVE, -mv);
    } else if dist2(m.pos, m.pv_v(pv::GOAL)) < 2.0 {
        let d = dist2(target, m.pos);
        let mut change = d < BAND.0 || BAND.1 < d;
        let mut t = mv;
        if countdown(&mut t) {
            change = true;
            t = by_attempts(w, 5, 15, MOVE_T.0 as f32, MOVE_T.1 as f32) as i32 + ctx.game.rng.rand_range(0, 60);
        }
        w.mobys[id].set_pv_i(pv::MOVE, t);
        if change {
            let mut step = ctx.game.rng.randf(10.0, 40.0) * 0.017_453_294;
            let cam = ctx.host.camera_pos();
            let r = ctx.host.camera_rows();
            let cam_yaw = atan2(r[0][0], r[0][1]);
            let p = w.mobys[id].pos;
            if 0.0 < sub_rot(atan2(p[0] - cam[0], p[1] - cam[1]), cam_yaw) * step { step = -step; }
            let a = add_rot(cam_yaw, step);
            w.mobys[id].set_pv_v(pv::GOAL, add([a.cos() * CIRCLE, a.sin() * CIRCLE, 0.0], target));
        }
    }
    let m = &mut w.mobys[id];
    let ph = add_rot(m.pv_f(pv::BOB), BOB.1 * 0.000_290_888_25);
    m.set_pv_f(pv::BOB, ph);
    let z = cuboid_pos(ctx, m.pv_i(pv::ARENA))[2] + HEIGHT + BOB.0 * ph.sin();
    m.set_pv_f(pv::GOAL + 8, z);
}

/// `0x3f9458(m, limit)`: [`place`], then the flight to the goal (riding: in the platform's frame, carried by its move,
/// at most `limit` of it with one, facing away from it; free: turning to Ratchet): the velocity steered toward the
/// stopping speed (at most its top, or `limit`) by at most the steering; the roll eased toward its sideways speed.
fn follow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, limit: f32) {
    place(w, id, ctx);
    let hero = ctx.hero.pos;
    let m = &w.mobys[id];
    let riding = m.sub_state == 0;
    let target = if riding {
        let Some(p) = ai::link(m, pv::PLATFORM).and_then(|p| w.mobys.get(p)) else { return };
        let t = add(through(m.pv_v(pv::GOAL), &p.rows), p.pos);
        let carried = crate::moby::platform::block(p).map_or([0.0; 3], |b| p.pv_v(b + 0x10));
        let carried = if 0.0 <= limit && limit < len(carried) { set_len(carried, limit) } else { carried };
        let yaw = add_rot(p.rot[2], std::f32::consts::PI);
        let m = &mut w.mobys[id];
        m.pos = add(m.pos, carried);
        m.rot[2] = yaw;
        t
    } else {
        let t = m.pv_v(pv::GOAL);
        let m = &mut w.mobys[id];
        let (mut yaw, mut v) = (m.rot[2], m.pv_f(pv::YAW_V));
        crate::hero::physics::turn_ease(atan2(hero[0] - m.pos[0], hero[1] - m.pos[1]), 0.003_490_659, 0.003_490_659, 0.209_439_55, &mut yaw, &mut v, 0);
        m.rot[2] = yaw;
        m.set_pv_f(pv::YAW_V, v);
        t
    };
    let (rate, top) = STEER[!riding as usize];
    let (rate, mut top) = (rate / 3600.0, top / 60.0);
    let m = &mut w.mobys[id];
    let d = sub(target, m.pos);
    let l = len(d);
    if 0.0 <= limit && limit < top { top = limit; }
    let amount = (l * rate * 0.5).sqrt().min(top);
    let step = if amount < l { set_len(d, amount) } else { d };
    let mut vel = m.pv_v(pv::VEL);
    let mut a = sub(step, vel);
    if rate < len(a) { a = set_len(a, rate); }
    vel = add(vel, a);
    m.set_pv_v(pv::VEL, vel);
    m.pos = add(m.pos, vel);
    let bank = (dot(m.rows[1], vel) * BANK).clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
    let k = 0.000_004_848_137_5;
    let (mut roll, mut rv) = (m.rot[0], m.pv_f(pv::ROLL_V));
    crate::hero::physics::ease(bank, ROLL.0 * k, ROLL.0 * k, ROLL.1 * 0.000_290_888_25, &mut roll, &mut rv);
    m.rot[0] = roll;
    m.set_pv_f(pv::ROLL_V, rv);
}

/// `0x3fa130`: its parts kept on it (but sub 2: the body's copy at its place and angles, the glow at its joint 0's
/// frame), its flash, the boss bar (picture 6) below sub 2, its ticks, the hover timer.
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if id >= w.mobys.len() || 0xfd <= w.mobys[id].state { return; }
    if w.mobys[id].sub_state != 2 {
        let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
        if let Some(b) = ai::link(&w.mobys[id], pv::BODY).filter(|&b| b < w.mobys.len()) {
            let p = &mut w.mobys[b];
            (p.pos, p.rot) = (pos, rot);
            p.rows = crate::hero::physics::euler_rows(rot);
        }
        if let Some(g) = ai::link(&w.mobys[id], pv::GLOW).filter(|&g| g < w.mobys.len()) {
            if let Some((rows, at)) = ctx.host.joint_matrix(&w.mobys[id], 0) {
                let p = &mut w.mobys[g];
                (p.rows, p.pos) = (rows, at);
            }
        }
        w.changed = true;
    }
    let m = &mut w.mobys[id];
    ai::flash(m, pv::FLASH);
    if m.sub_state < 2 {
        let most = m.pv_s(pv::MAX) as f32;
        if 0.0 < most {
            ctx.game.boss_bar = Some(((m.pv_f(pv::HP) / most).clamp(0.0, 1.0), 5));
            ctx.game.boss_icon = 6;
        }
    }
    m.state_ticks = m.state_ticks.wrapping_add(1);
    let mut t = m.pv_i(pv::HOVER);
    countdown(&mut t);
    m.set_pv_i(pv::HOVER, t);
}

// ---- The charge glow 0x0d45 ----

/// `0x3f0520(glow, ticks, life)`: its size from 0, to 1 over `ticks`, gone after `life` (state 1).
fn glow_start(g: &mut Moby, ticks: i32, life: i32) {
    g.set_pv_f(0, 0.0);
    g.set_pv_f(4, 1.0 / ticks.max(1) as f32);
    g.set_pv_i(8, life);
    g.set_state(1, -1);
}

/// 0x0d45 (`0x3f04a8`; L00 `0x3fa998`): lit (1), its size up to 1 by its step, its draw registered ([`draw_0d45`]),
/// back to 0 when its life is out.
pub fn update_0d45(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    if w.mobys[id].state != 1 { return; }
    let m = &mut w.mobys[id];
    let mut s = m.pv_f(0);
    crate::hero::physics::approach(1.0, m.pv_f(4), &mut s);
    m.set_pv_f(0, s);
    w.register_draw(id);
    let m = &mut w.mobys[id];
    let mut t = m.pv_i(8);
    let out = countdown(&mut t);
    m.set_pv_i(8, t);
    if out { m.set_state(0, -1); }
}

/// `$gp+0x1c50..+0x1c68`: the glows' colour pairs; `+0x1c70..+0x1c7c` their sizes; `+0x1c80` the point in its frame.
const GLOW_RGBA: [(u32, u32); 4] = [(0x5010_3080, 0x6010_3080), (0x5000_8080, 0x6000_0080), (0x0800_4080, 0x1800_6080), (0x1800_6080, 0x2000_4080)];
const GLOW_SIZE: [f32; 4] = [1.0, 0.6, 1.4, 0.9];
const GLOW_FX: [usize; 4] = [0x31, 0x31, 0xb, 0xb];
const GLOW_AT: V3 = [0.2, -0.5, 0.0];

/// `0x3f0590`: four glows at its point, each its pair's colours mixed at random, its size × the glow's, rolled at
/// random.
pub fn draw_0d45(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let at = add(through(GLOW_AT, &m.rows), m.pos);
    for k in 0..4 {
        let t = rng.randf(0.0, 1.0);
        let c = crate::hud::lerp_rgba(t, GLOW_RGBA[k].0, GLOW_RGBA[k].1);
        let roll = rng.rand_angle();
        crate::fx::sprite(out, GLOW_SIZE[k] * m.pv_f(0), 0.0, roll, at, cam, GLOW_FX[k], c, true);
    }
}
