//! Dobbo's critter **0x0d90** (L09 `0x406aa0`, 18 placed; the decomp's `func_US2_L09_00406AA0`; its senses `0x4062e8`
//! with the set-up `0x4060e0`, tail `0x406458`, glow `0x406788` and the group's draw `0x406920`), a small creature on
//! the framework ([`crate::moby::ai`]) built like Maktar's (crate::moby::classes::critter), its words 0x20 further on:
//! health / reaction `+0x50`, movement `+0x100`, target `+0x1f0` (its moby `+0x200`), its area `+0x230` and fence path
//! `+0x234`, its retreat's reach `+0x238`.
//!
//! **Its words**: home `+0x300`, the stroll's goal `+0x310`, timers `+0x330` / `+0x332`, seeing `+0x334`, alarmed
//! `+0x336`, the chase's spread `+0x33c`, the bite's heading `+0x340`, its size `+0x344` (`randf(1, 1.333)`, its scale
//! that times its class's), the glow's phase `+0x348` and pulse `+0x34c`, the frame its group's glow was asked `+0x350`,
//! the sight's delay `+0x35c` (a word); its joint 0 kept for the draw `+0x240`, the aim record `+0x280` and the head
//! record `+0x360` (crate::moby::joint_aim).
//!
//! **The set-up**: home its place; the reaction (`+0x48` 15, `+0x4a` 4, `+0x60` 0xb, `+0x28` 3, `+0x2a` 2, `+0x30`
//! 1/3, `+0x80` / `+0x84` 0.2, `+0x8c` 0.4, `+0x98` 3); movement (radius 0.3·size, height 0.1·size, steps 0.1, slope
//! 10.2°, turn 30°, stop 0.05, options `|= 0x80`, the weave `randf_signed(18°, 22°)` over 120..180 ticks, walk
//! sequences 7 / 6, its fence its path `+0x234`, the walk's speed); idle (1, sub 0, sequence 0); its collision word `+0x9c` 12
//! and a target (mode 0x1000).
//!
//! **Each tick** (its senses): drawn and within 24 of the camera, its shadow (range 0x16); the frame's damage (a hurt
//! alarms its group's 0x0d90, `+0x336`; a kill → [`die`]); outside the knock (subs 3 / 4) its target (Ratchet within 16,
//! in its area) into `+0x334`. (Its update first returns on `$gp+0x23e8`, a word nothing on the level writes: 0.)
//!
//! **Subs**: 0 calm — seeing him after `+0x35c` ticks, the chase (9, sub 2) or one in four the stare (8); alarmed it
//! turns (6 / 7, sub 1; `+0x330` 480..600 when under 300). 1 hunting — seeing him the chase; `+0x330` out, calm. 2 wary
//! (`+0x35c` 0..30) — not seeing him after it, the back-off (3, sub 1, `+0x330` 580..600). 3 the knock: dead → [`die`];
//! over, the chase or the back-off.
//!
//! **States**: 1 idle (drawn only: sequence 0, or 1 one time in four, for 60..120 ticks) → 2. 2 / 5 a stroll to a point
//! 2..5 from home (kept in its fence; running / walking) for 31..181 ticks → 1 / 3. 3 alert (sequence 4 / 5): within 8 of
//! Ratchet → 4, else at the sequence's end 6 / 7 / 5. 4 retreat home (the walk) until 8..14 + 0.5 from him → 6 / 7. 6 / 7
//! turning to the target (sequences 2..3 / 0xc) → 3. 8 the stare (2..3, turning faster to its target moby) → 9. 9 the
//! chase (the walk, sequence 6, its group spread 5° a member every four ticks, the head's look) → within 1.25, 10. 10
//! the bite (sequence 8 from frame 2): facing the heading it took, at frame 13 a hit from joint 1 (0.75, its damage
//! byte, flags 0x4000001) → 9.
//!
//! The L09 engine functions with the port's: `0x327070` / `0x327048` the sub's / state's first tick, `0x32a218` senses,
//! `0x324590` take_damage, `0x3252d8` flight, `0x3275b8` find_target, `0x328fe8` / `0x329040` the moves (move_step /
//! move_toward), `0x3292a8` = L00 `0x32ceb8` the turn in place (crate::moby::ai::turn_in_place), `0x32a0d8` = L02
//! `0x333980` the turn (crate::moby::ai::face_at), `0x32a138` face_point, `0x326ec0` keep_in_path, `0x326d30` spread, `0x320048` reached, `0x3195f0` attack_joint, `0x316fe8` the
//! bolts, `0x31d518` = L00 `0x3210a8` (smoke_burst), `0x3238a8` = L00 `0x327498` (break_into), `0x3206a0` /
//! `0x323718` the aims, `0x323128` the glow sprite, `0x30d900` delete.
//!
//! Not ported: the senses' level-9 check of the moby `+0xc0` (`0x3280e0`, clearing `$gp−0x5fb0`).

use crate::hero::physics::{add_rot, atan2, dist, dist2, sub, V3};
use crate::hud::lerp_rgba;
use crate::moby::classes::critter::{face_point, keep_in_path, spread};
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::{ai, countdown, joint_aim, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0d90;
const MV: usize = 0x100;
const TARGET: usize = 0x1f0;
const T_MOBY: usize = 0x200;
const AREA: usize = 0x230;
const FENCE: usize = 0x234;
const REACH: usize = 0x238;
const JOINT: usize = 0x240;
const AIM: usize = 0x280;
const HOME: usize = 0x300;
const GOAL: usize = 0x310;
const T330: usize = 0x330;
const T332: usize = 0x332;
const SEEING: usize = 0x334;
const ALARMED: usize = 0x336;
const SPREAD: usize = 0x33c;
const BITE_YAW: usize = 0x340;
const SIZE: usize = 0x344;
const PHASE: usize = 0x348;
const PULSE: usize = 0x34c;
const DRAWN_AT: usize = 0x350;
const DELAY: usize = 0x35c;
const HEAD: usize = 0x360;
const PVARS: usize = 0x3e0;
/// `$gp+0x23b0` / `0x23b4` (`0x1b13a0`): the run's and the walk's speeds a second.
const RUN_SPEED: f32 = 1.0;
const WALK_SPEED: f32 = 5.0;
/// `$gp+0x23b8` / `0x23bc`: Ratchet's distance that makes it retreat, and its retreat's reach (+ 0.5).
const NEAR: f32 = 8.0;
const FAR: f32 = 14.0;
/// `0x1b13b0`: its glow word's pulse; `0x1b13c0`: the eye's place in joint 0's frame (× its size); `0x1b13d0`: the
/// eye's colours.
const GLOW: [u32; 2] = [0x8000_0080, 0x0000_0020];
const EYE: V3 = [0.25, 0.0, 0.1];
const EYE_RGBA: [u32; 2] = [0x3000_0080, 0x0000_0020];

fn group(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// `0x406020` / `0x406080`: the run (sequence 7, limit 3) or the walk (sequence 6, limit 1.75), with `blend`.
fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, blend: u8) {
    let m = &mut w.mobys[id];
    m.set_pv_f(MV + 0xd4, 3.0);
    m.set_pv_f(MV + 0xd0, RUN_SPEED * 0.016_666_668);
    set_seq(w, id, ctx, 7, 0, blend);
}
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, blend: u8) {
    let m = &mut w.mobys[id];
    m.set_pv_f(MV + 0xd4, 1.75);
    m.set_pv_f(MV + 0xd0, WALK_SPEED * 0.016_666_668);
    set_seq(w, id, ctx, 6, 0, blend);
}

/// `0x406420(m, goal, toward)`: straight to `goal`, or (`toward`) offset by the spread.
fn go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3, toward: bool) -> i32 {
    if toward {
        let off = w.mobys[id].pv_f(SPREAD);
        ai::move_toward(w, id, ctx, off, goal)
    } else {
        let p = w.mobys[id].pos;
        ai::move_step(w, id, ctx, atan2(goal[0] - p[0], goal[1] - p[1]), goal)
    }
}

/// `0x4060e0` (module docs).
fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let class_scale = w.class(CLASS).map_or(1.0, |c| c.scale);
    let rng = &mut ctx.game.rng;
    let s = rng.randf(1.0, f32::from_bits(0x3faa_9fbe));
    let weave = rng.randf_signed(18.0, 22.0);
    let phase = rng.rand_angle();
    let m = &mut w.mobys[id];
    if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
    let p = m.pos;
    m.set_pv_v(HOME, p);
    m.set_pv_f(SIZE, s);
    m.scale = class_scale * s;
    m.set_pv_b(0x48, 15);
    m.set_pv_b(0x4a, 4);
    m.set_pv_f(0x84, 0.2);
    m.set_pv_b(0x28, 3);
    m.set_pv_f(MV + 0x34, 0.05);
    m.set_pv_f(MV + 4, s * 0.1);
    m.set_pv_f(MV + 0x10, f32::from_bits(0x3e32_b8c4));
    m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
    m.set_pv_i(0x98, 3);
    m.set_pv_f(0x80, 0.2);
    m.set_pv_b(0x60, 0xb);
    m.set_pv_f(0x30, f32::from_bits(0x3eaa_7efa));
    m.set_pv_b(0x2a, 2);
    m.set_pv_f(MV, s * 0.3);
    m.set_pv_f(MV + 0xc, 0.1);
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x80);
    m.set_pv_f(MV + 8, 0.1);
    m.set_pv_f(0x8c, 0.4);
    m.set_pv_b(MV + 0xe8, 7);
    m.set_pv_b(MV + 0xe9, 6);
    m.set_pv_s(MV + 0xe2, 0x78);
    let fence = m.pv_i(FENCE);
    m.set_pv_i(MV + 0x3c, fence);
    m.set_pv_s(MV + 0xe4, 0xb4);
    m.set_pv_f(MV + 0xd4, 1.75);
    m.set_pv_f(MV + 0xd0, WALK_SPEED * 0.016_666_668);
    m.set_pv_f(MV + 0xdc, weave * 0.017_453_294);
    m.set_pv_f(PHASE, phase);
    m.set_state(1, 0);
    set_seq(w, id, ctx, 0, 0, 0);
    let m = &mut w.mobys[id];
    m.coll_disable = 0xc;
    m.mode |= mode::TARGET;
}

/// `0x4064f0`: its group's living 0x0d90 alarmed (`+0x336`).
fn alarm(w: &mut MobyWorld, id: usize) {
    for k in group(w, id) {
        let x = &mut w.mobys[k];
        if x.state < 0x80 && x.o_class == CLASS { x.set_pv_s(ALARMED, 1); }
    }
}

/// `0x406570`: dead — its bolts (3), its class sound 1, a small smoke burst (1/3) 0.75 up its frame, its pieces flung
/// away from Ratchet (3..6 across and up, at 0.1) and it goes.
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
    ctx.host.play_sound(&w.mobys[id], 1, 0);
    let m = &w.mobys[id];
    let at: V3 = std::array::from_fn(|k| m.pos[k] + 0.75 * m.rows[2][k]);
    crate::moby::blast::smoke_burst(w, ctx, f32::from_bits(0x3eaa_7efa), 0.0, Some(id), at, -1);
    let mut away = sub(w.mobys[id].pos, ctx.hero.pos);
    away[2] = 0.0;
    let v = crate::hero::physics::scale(away, f32::from_bits(0x3dcc_ccce));
    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x1f, v, 0xff);
    w.delete(id);
}

/// `0x4062e8` (module docs); true when it is gone.
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 { set_up(w, id, ctx); }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    let k = ai::take_damage(w, id, ctx, false);
    if k != 0 {
        alarm(w, id);
        if 4 <= k {
            die(w, id, ctx);
            return true;
        }
    }
    if !matches!(w.mobys[id].sub_state, 3 | 4) {
        let area = w.mobys[id].pv_i(AREA);
        let s = ai::find_target(w, id, ctx, 16.0, 0.0, area, None, false);
        w.mobys[id].set_pv_s(SEEING, s as i16);
    }
    false
}

fn back_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let t = ctx.game.rng.rand_range(0x244, 600);
    let m = &mut w.mobys[id];
    m.set_state(3, 1);
    m.set_pv_s(T330, t as i16);
}

/// 0x0d90 (module docs).
pub fn update_0d90(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if senses(w, id, ctx) { return; }
    let walk_speed = WALK_SPEED * 0.016_666_668;
    match w.mobys[id].sub_state {
        0 => {
            if w.mobys[id].first_sub() {
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3a91_a2b4), f32::from_bits(0x3d08_8889), f32::from_bits(0x3be4_c38a), f32::from_bits(0x3ed6_7752));
                let m = &mut w.mobys[id];
                m.set_pv_f(MV + 0x24, f32::from_bits(0x3bda_740e));
                m.set_pv_f(MV + 0x34, 0.5);
            }
            if w.mobys[id].pv_s(SEEING) == 1 && w.mobys[id].count_i(DELAY) {
                let s = if ctx.game.rng.randi(4) == 0 { 9 } else { 8 };
                w.mobys[id].set_state(s, 2);
            } else if w.mobys[id].pv_s(ALARMED) != 0 {
                let s = if ctx.game.rng.randi(1) == 0 { 7 } else { 6 };
                let m = &mut w.mobys[id];
                m.set_state(s, 1);
                m.set_pv_s(ALARMED, 0);
                if m.pv_s(T330) < 300 {
                    let t = ctx.game.rng.rand_range(0x1e0, 600);
                    w.mobys[id].set_pv_s(T330, t as i16);
                }
            }
        }
        1 => {
            if w.mobys[id].first_sub() {
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3b11_a2b4), f32::from_bits(0x3daa_aaab), f32::from_bits(0x3be4_c38a), f32::from_bits(0x3ed6_7752));
                let m = &mut w.mobys[id];
                m.set_pv_f(MV + 0x34, 0.05);
                m.set_pv_f(MV + 0xd0, walk_speed);
            }
            let m = &mut w.mobys[id];
            if m.pv_s(SEEING) == 1 { m.set_state(9, 2) } else if m.pv_s(T330) == 0 { m.set_state(1, 0) }
        }
        2 => {
            if w.mobys[id].first_sub() {
                ai::senses(&mut w.mobys[id], f32::from_bits(0x3b5a_740e), f32::from_bits(0x3daa_aaab), f32::from_bits(0x3be4_c38a), f32::from_bits(0x3ed6_7752));
                let t = ctx.game.rng.rand_range(0, 0x1e);
                let m = &mut w.mobys[id];
                m.set_pv_f(MV + 0x24, f32::from_bits(0x3bda_740e));
                m.set_pv_f(MV + 0x34, 0.05);
                m.set_pv_f(MV + 0xd0, walk_speed);
                m.set_pv_i(DELAY, t);
            }
            if w.mobys[id].pv_s(SEEING) != 1 && w.mobys[id].count_i(DELAY) { back_off(w, id, ctx); }
        }
        3 => {
            let r = ai::flight(w, id, ctx);
            if r & 1 != 0 {
                die(w, id, ctx);
                return;
            }
            if r & 2 != 0 {
                if w.mobys[id].pv_s(SEEING) == 1 { w.mobys[id].set_state(9, 2) } else { back_off(w, id, ctx) }
            }
        }
        _ => {}
    }
    states(w, id, ctx);
    tail(w, id, ctx);
    if w.mobys[id].drawn != 0 { glow(w, id, ctx); }
}

/// The stroll's start (states 2 / 5): the run (2) or the walk (5), blend 20, a point 2..5 from home a random way, kept
/// in its fence.
fn stroll_start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, running: bool) {
    if running { run(w, id, ctx, 0x14) } else { walk(w, id, ctx, 0x14) }
    let rng = &mut ctx.game.rng;
    let a = rng.rand_angle();
    let r = rng.randf(2.0, 5.0);
    let m = &w.mobys[id];
    let home = m.pv_v(HOME);
    let mut p = [a.cos() * r + home[0], a.sin() * r + home[1], home[2]];
    let fence = m.pv_i(FENCE);
    if fence != -1 { p = keep_in_path(ctx, 1.5, home, p, fence); }
    w.mobys[id].set_pv_v(GOAL, p);
}

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let next = match w.mobys[id].state {
        1 => {
            if w.mobys[id].drawn == 0 { return; }
            if w.mobys[id].first() {
                let r = ctx.game.rng.randi(4);
                if (seq(w, id, ctx) != 0) != (r == 0) { ctx.host.set_anim(&w.mobys[id], (r == 0) as u8, 0, 0x14); }
                let t = ctx.game.rng.rand_range(0x3c, 0x78);
                w.mobys[id].set_pv_s(T332, t as i16);
            }
            if seq(w, id, ctx) == 1 {
                if wrapped(w, id, ctx) { ctx.host.set_anim(&w.mobys[id], 0, 0, 0x14); }
                return;
            }
            if w.mobys[id].pv_s(T332) != 0 { return; }
            2
        }
        s @ (2 | 5) => {
            if w.mobys[id].first() { stroll_start(w, id, ctx, s == 2); }
            let g = w.mobys[id].pv_v(GOAL);
            go(w, id, ctx, g, false);
            let m = &w.mobys[id];
            let t = m.state_ticks as u16;
            if t < 0x1f || (0.5 < m.pv_f(MV + 0xcc) && t < 0xb5) { return; }
            if s == 2 { 1 } else { 3 }
        }
        3 => {
            if w.mobys[id].first() {
                let r = ctx.game.rng.randi(2) as u8 + 4;
                if seq(w, id, ctx) != r {
                    let r = ctx.game.rng.randi(2) as u8 + 4;
                    ctx.host.set_anim(&w.mobys[id], r, 0, 10);
                }
            }
            if dist(w.mobys[id].pos, ctx.hero.pos) < NEAR { 4 } else {
                if !wrapped(w, id, ctx) { return; }
                match ctx.game.rng.randi(3) { 0 => 6, 1 => 7, 2 => 5, _ => return }
            }
        }
        4 => {
            if w.mobys[id].first() {
                walk(w, id, ctx, 10);
                let r = ctx.game.rng.randf(NEAR, FAR) + 0.5;
                w.mobys[id].set_pv_f(REACH, r);
            }
            let home = w.mobys[id].pv_v(HOME);
            let flags = go(w, id, ctx, home, false);
            if flags & 0x80 == 0 && dist2(w.mobys[id].pos, ctx.hero.pos) <= w.mobys[id].pv_f(REACH) { return; }
            if ctx.game.rng.randi(2) == 0 { 7 } else { 6 }
        }
        s @ (6 | 7) => {
            if w.mobys[id].first() {
                if s == 6 {
                    let r = ctx.game.rng.randi(2) as u8 + 2;
                    if seq(w, id, ctx) != r {
                        let r = ctx.game.rng.randi(2) as u8 + 2;
                        ctx.host.set_anim(&w.mobys[id], r, 0, 10);
                    }
                } else {
                    set_seq(w, id, ctx, 0xc, 0, 10);
                }
            }
            let t = w.mobys[id].pv_v(TARGET);
            face_point(w, id, ctx, t);
            if !wrapped(w, id, ctx) { return; }
            3
        }
        8 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.set_pv_f(MV + 0x14, f32::from_bits(0x3c91_5c40));
                m.set_pv_f(MV + 0x1c, f32::from_bits(0x3f88_467c));
                let r = ctx.game.rng.randi(2) as u8 + 2;
                if seq(w, id, ctx) != r {
                    let r = ctx.game.rng.randi(2) as u8 + 2;
                    ctx.host.set_anim(&w.mobys[id], r, 0, 10);
                }
            }
            let m = &w.mobys[id];
            let t = ai::link(m, T_MOBY).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos);
            let yaw = atan2(t[0] - m.pos[0], t[1] - m.pos[1]);
            ai::turn_in_place(w, id, ctx, yaw);
            if !wrapped(w, id, ctx) { return; }
            let m = &mut w.mobys[id];
            m.set_pv_f(MV + 0x14, f32::from_bits(0x3be4_c38a));
            m.set_pv_f(MV + 0x1c, f32::from_bits(0x3ed6_7752));
            9
        }
        9 => {
            if w.mobys[id].first() { walk(w, id, ctx, 5); }
            if w.mobys[id].state_ticks & 3 == 0 {
                let m = &w.mobys[id];
                let t = ai::link(m, T_MOBY).and_then(|t| w.mobys.get(t)).map_or(ctx.hero.pos, |t| t.pos);
                let n = spread(w, id, t);
                w.mobys[id].set_pv_f(SPREAD, n as f32 * 0.087_266_475);
            }
            let t = w.mobys[id].pv_v(TARGET);
            go(w, id, ctx, t, true);
            crate::moby::classes::critter::aim_into(w, id, AIM + 0x60);
            if 1.25 < w.mobys[id].pv_f(MV + 0xcc) { return; }
            10
        }
        10 => {
            if w.mobys[id].first() {
                let m = &w.mobys[id];
                let t = m.pv_v(TARGET);
                let yaw = atan2(t[0] - m.pos[0], t[1] - m.pos[1]);
                w.mobys[id].set_pv_f(BITE_YAW, yaw);
                if seq(w, id, ctx) != 8 { ctx.host.set_anim(&w.mobys[id], 8, 2, 10); }
            }
            let yaw = w.mobys[id].pv_f(BITE_YAW);
            crate::moby::ai::face_at(&mut w.mobys[id], MV, yaw);
            if ai::reached(w, id, ctx, 13.0) {
                let d = w.mobys[id].pv_b(0x34) as f32;
                ai::attack_joint(w, id, ctx, 0.75, d, 1.0, 1, 0x400_0001, 0, 1, 0);
            }
            if !wrapped(w, id, ctx) { return; }
            9
        }
        _ => return,
    };
    w.mobys[id].set_state(next, -1);
}

/// `0x406458`: its timers `+0x330` / `+0x332` down, its state's ticks on; the aim record `+0x280` sprung (list 0, 0.03 /
/// 0.3) and the head record `+0x360` asked (list 0, 1.5).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    for off in [T330, T332] {
        let mut t = m.pv_s(off) as i32;
        countdown(&mut t);
        m.set_pv_s(off, t as i16);
    }
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    joint_aim::aim(w, id, AIM, id, 0, f32::from_bits(0x3cf5_c28f), f32::from_bits(0x3e99_999a));
    joint_aim::head(w, ctx, id, HEAD, 0, 1.5);
}

/// `0x406788`: its glow word's pulse (12° a tick); unless its stamp is this frame's, every drawn, living 0x0d90 of its
/// group keeps its joint 0 (`+0x240`) and is stamped, and the group's eyes are asked once ([`draw`]).
fn glow(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let p = add_rot(m.pv_f(PHASE), f32::from_bits(0x3e56_7752));
    m.set_pv_f(PHASE, p);
    let k = p.sin() * 0.5 + 0.5;
    m.set_pv_f(PULSE, k);
    m.glow = lerp_rgba(k, GLOW[0], GLOW[1]);
    let frame = ctx.game.frame;
    if m.pv_i(DRAWN_AT) == frame { return; }
    for o in group(w, id) {
        let x = &w.mobys[o];
        if x.drawn == 0 || x.o_class != CLASS || x.state >= 0xfd { continue; }
        let Some((rows, at)) = ctx.host.joint_matrix(x, 0) else { continue };
        let x = &mut w.mobys[o];
        for (k, r) in rows.iter().enumerate() { x.set_pv_v(JOINT + 0x10 * k, *r); }
        x.set_pv_v(JOINT + 0x30, at);
        x.set_pv_i(DRAWN_AT, frame);
    }
    w.register_draw(id);
}

/// `0x406920` (the group's draw callback): each drawn, living 0x0d90's eye, a glow (FX 0xb, additive) `0.2·size`
/// across, 0.08 toward the camera, at `(0.25, 0, 0.1)·size` in its joint 0's frame, its colour the pulse's way from
/// `0x30000080` to `0x00000020`.
pub fn draw(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    for o in group(w, id) {
        let x = &w.mobys[o];
        if x.drawn == 0 || x.o_class != CLASS || x.state >= 0xfd { continue; }
        let s = x.pv_f(SIZE);
        let rows = [x.pv_v(JOINT), x.pv_v(JOINT + 0x10), x.pv_v(JOINT + 0x20)];
        let l = EYE.map(|v| v * s);
        let p: V3 = std::array::from_fn(|k| l[0] * rows[0][k] + l[1] * rows[1][k] + l[2] * rows[2][k] + x.pv_f(JOINT + 0x30 + 4 * k));
        let rgba = lerp_rgba(x.pv_f(PULSE), EYE_RGBA[0], EYE_RGBA[1]);
        crate::fx::sprite(out, s * 0.2, f32::from_bits(0x3da3_d70a), 0.0, p, cam, 0xb, rgba, true);
    }
}
