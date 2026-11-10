//! Dobbo's hovering flyer **0x0d57** (`0x404c28`, 12 placed; the decomp's `func_L09_00403798`, its helpers
//! `func_L09_00402D50`.. `func_L09_00403700` at our `0x4041e0`.. `0x404b90`): it hovers over the ground on its
//! own (wandering about home, or first flying in along a path), and once Ratchet is in its area (or its group is
//! alarmed) it charges him, its four joints hitting as it flies into him, backs off and charges again. On the
//! framework (crate::moby::ai): health `+0x20` (its damage byte `+0x34`), reaction `+0x50` (a hurt to state 9, sub
//! 1; radius 0.5, centre 0.7, bounce 0.4), movement `+0x100` (radius 1, height 1.1, steps 2.8, no turn, the fence
//! path `+0x13c`), the wander block `+0x1f0` (home its place at the set-up, radius 1, 5 out, 10 from Ratchet), target
//! block `+0x220` (`+0x230` the moby, `+0x234` found).
//!
//! **Its pvars:** `+0x240` / `+0x244` the target areas calm / alerted (the one searched `+0x28c`), `+0x24c` the
//! range, `+0x250` the path in (−1 none), `+0x254` / `+0x258` the fence paths alerted / at the set-up (−1: the
//! first), `+0x25c` the path's time in seconds, `+0x260` the weave in degrees (−1: 25..100), `+0x264` the lowest it
//! hovers at, `+0x26a` charging (set on one of its group's when one is killed), `+0x26c` / `+0x26e` timers counted
//! down (s16; `+0x26c` the alert, 360), `+0x270` alarmed by its group ([`alarm`]), `+0x274` the charge's looped
//! sound, `+0x278` / `+0x27c` the bob's phase and offset, `+0x280` / `+0x284` the sway's phases, `+0x288` the
//! ground under it (its place's height while it moves), `+0x290` its place along the path, `+0x294` the hover
//! height.
//!
//! **Each tick** (`0x404c28`): the head (`0x404700`: the set-up `0x4041e0` in state 0; the shadow within 48 of the
//! camera, range 0x2d, `0x319ec8` = L00 `0x31da58`, crate::shadow_volumes::probe_near; the frame's damage `0x324590`
//! = L00 `0x327e38`, crate::moby::ai::take_damage: hurt, its group alarmed and, killed with two of its group alive,
//! the first of its group's 0x0d57s not charging charges; the target `0x3275b8` = L00 `0x32ad38` in the range
//! `+0x24c`, no height, the area `+0x28c`); out of the hurt state its height back to the ground's `+0x288`
//! (`0x404808`). Then the sub-state, the state, the charge's sound stopped outside state 7, the timers ([`timers`],
//! `0x404668`), the hover ([`hover`], `0x404828`) and the hits ([`hits`], `0x404b90`).
//!
//! **Sub-states** (`+0x95`): **0** waiting at the path's start: found or alerted, its group alarmed, sequence 2,
//! the alerted area, no gravity, along the path (4, sub 1). **2** calm: found or alerted, its group alarmed, the
//! alert 360, sequence 2, the senses, the alerted fence path and area, 3 (sub 3). **3** alert: neither found nor
//! alerted, the senses, sequence 0 and calm (2, sub 2).
//!
//! **States** (`+0x20`): **2** the wander (L09 `0x31fdd0` = L04 `0x345c78`, crate::moby::classes::leaper::wander, on
//! `+0x1f0`). **3** at its sequence's end 5. **4** along the path: `+0x290` up by the path's points over `+0x25c`
//! seconds a tick; at its end (the point count) the senses, gravity back and 5 (sub 3); the pose there (L09
//! `0x341208` = L00 `0x343fb8`, crate::paths::pose). **5** the wander alerted (sequence 0 after each round);
//! charging, the senses, sequence 2 and 6. **6** at the end of sequence 2, sequence 3 and 7. **7** the charge: its
//! looped sound 4 (started on its first tick), the move toward its target's point (L09 `0x329040` = L00 `0x32cc30`,
//! crate::moby::ai::move_toward); once it hit Ratchet (`0x18c2bc`) or is within 1.2 of him across, sequence 4 and 8.
//! **8** at the end of sequence 4, sequence 0 and 5. **9** hurt: the framework's flight (L09 `0x3252d8` = L00
//! `0x328b80`); dead, its pieces (`0x3238a8` = L00 `0x327498`, crate::moby::shards::break_into: all six, 3..6 / 3..6,
//! 0.1 away from Ratchet across its own up), sound 0, its bolts (`0x316fe8` = L00 `0x31adc0`, limit 3) and deleted;
//! over, the alert 360, sound 3 and 5 (sub 3).
//!
//! **The hover** ([`hover`]): outside states 4 and 9 its height `+0x294` falls 1/30 a tick and is eased up (by 0.09
//! of the way) to 0.7 over the ground `+0x288` (unless it stands on Ratchet, on a 0x0d57 or on a moby whose class's
//! `+0x46` is under 11) and to 0.7 over `+0x264`; charging (6..8) by 0.06 to 0.5 over its target. Its group's members
//! within 3 push it away (each by `d·(1 − d/3)`); the bob (L09 `0x3208c8` = L00 `0x324430`,
//! crate::moby::classes::props::bob: 0.1 at 5° a tick about the hover height) and the sway (L09 `0x320980` = L00
//! `0x3244e8`, crate::moby::classes::props::sway: 8°, 1.5° / 1.5° a tick).
//!
//! The L09 engine functions with the port's: `0x317af8` = L00 `0x31b8d0` (crate::moby::Moby::set_state), `0x317818`
//! = L00 `0x31b5f0` (set_anim, crate::moby::classes::props::set_seq), `0x327048` = L00 `0x32a7c8` (the first tick of
//! the state), `0x32a218` = L00 `0x32dcf0` (crate::moby::ai::senses), `0x319148` = L00 `0x31ce20`
//! (crate::moby::ai::ground_z), `0x3195f0` = L00 `0x31d2c8` (crate::moby::ai::attack_joint), `0x318b08` = L00
//! `0x31c8e0` (crate::moby::classes::props::group_alive), `0x2ead98` (the countdown, crate::moby::countdown),
//! `0x35d9a8` / `0x35d6a8` = L00 `0x3605f0` / `0x3602f0` (the sounds), `0x30d900` = L00 `0x311688` (delete),
//! `0x317cc0` = L00 `0x31ba98` (crate::rng).
//!
//! Not ported: the flag `$gp−0x5fb0` cleared when the moby that last hurt it (`+0xc0`) is neither Ratchet nor one of
//! the states `0x3280e0` names (as crate::moby::classes::trooper); the floor moby's class `+0x46` (the
//! port has no class header byte for it: any other moby counts as 11 or more); the debug print at its death.

use crate::hero::physics::{dist2, dot, len, scale, set_len, sub, V3};
use crate::moby::classes::leaper;
use crate::moby::classes::props::{bob, group_alive, set_seq, sway, wrapped};
use crate::moby::{ai, bolts, countdown, Ctx, Moby, MobyWorld};

pub const CLASS: i16 = 0x0d57;

const HEALTH: usize = 0x20;
const REACT: usize = 0x50;
const MV: usize = 0x100;
const WANDER: usize = 0x1f0;
const T_POINT: usize = 0x220;
const T_MOBY: usize = 0x230;
const T_FOUND: usize = 0x234;
const AREA_CALM: usize = 0x240;
const AREA_ALERT: usize = 0x244;
const RANGE: usize = 0x24c;
const PATH: usize = 0x250;
const FENCE_ALERT: usize = 0x254;
const FENCE_START: usize = 0x258;
const PATH_TIME: usize = 0x25c;
const WEAVE: usize = 0x260;
const LOWEST: usize = 0x264;
const CHARGE: usize = 0x26a;
const ALERT: usize = 0x26c;
const TIMER2: usize = 0x26e;
const ALARM: usize = 0x270;
const SOUND: usize = 0x274;
const BOB_PHASE: usize = 0x278;
const BOB_LAST: usize = 0x27c;
const SWAY_A: usize = 0x280;
const SWAY_B: usize = 0x284;
const GROUND: usize = 0x288;
const AREA: usize = 0x28c;
const PATH_T: usize = 0x290;
const HEIGHT: usize = 0x294;

/// `0x32a218(1/150, 1/15, 48°, 48°)`.
const SENSES: [u32; 4] = [0x3bda_740e, 0x3d88_8889, 0x3f56_7752, 0x3f56_7752];
/// `$gp+0x237c`: the hover over the ground.
const LIFT: f32 = f32::from_bits(0x3f33_3333);
/// `$gp+0x2378`: the hover over its target while charging.
const OVER_TARGET: f32 = 0.5;
/// `$gp+0x2380`: the alert's ticks.
const ALERT_TICKS: i16 = 360;
/// `$gp+0x2388` / `$gp+0x238c`: the hover's eases.
const EASE: f32 = f32::from_bits(0x3db8_51ec);
const EASE_TARGET: f32 = f32::from_bits(0x3d75_c28f);
/// `$gp+0x2390`: how near its group's members push it.
const SPREAD: f32 = 3.0;
/// `$gp+0x2394`: the charge's looped sound.
const CHARGE_SOUND: i32 = 4;
const DEG: f32 = 0.017_453_294;

fn senses(m: &mut Moby) {
    let s = SENSES.map(f32::from_bits);
    ai::senses(m, s[0], s[1], s[2], s[3]);
}

fn found(m: &Moby) -> bool { m.pv_i(T_FOUND) == 1 }

/// Its group's members (none for group −1).
fn members(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// Its target moby's place (Ratchet's feet for him).
fn target_place(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<V3> {
    let k = ai::link(&w.mobys[id], T_MOBY)?;
    if Some(k) == ctx.hero_moby { return Some(ctx.hero.pos); }
    w.mobys.get(k).map(|t| t.pos)
}

fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_s(SOUND) as i32;
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_s(SOUND, -1);
}

/// 0x0d57 (module docs).
pub fn update_0d57(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if head(w, id, ctx) { return; }
    let m = &mut w.mobys[id];
    if m.state != 9 { m.pos[2] = m.pv_f(GROUND); }
    subs(w, id, ctx);
    if states(w, id, ctx) { return; }
    if w.mobys[id].state != 7 { stop_sound(w, id, ctx); }
    timers(&mut w.mobys[id]);
    hover(w, id, ctx);
    hits(w, id, ctx);
    w.changed = true;
}

/// `0x404700`: the head (module docs); true when it was deleted (never here).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 { setup(w, id, ctx); }
    crate::shadow_volumes::probe_near(w, id, ctx, 48.0, 0x2d);
    let kind = ai::take_damage(w, id, ctx, false);
    // `$gp−0x5fb0` (not ported): the hitter `+0xc0` let go.
    if w.mobys[id].pv_s(REACT + 0x70) != -1 { w.mobys[id].set_pv_s(REACT + 0x70, -1); }
    if kind != 0 {
        alarm(w, id);
        let g = w.mobys[id].group;
        if (kind == 4 || kind == 5) && g != -1 && 2 <= group_alive(w, g as i32) {
            for k in members(w, id) {
                let Some(o) = w.mobys.get_mut(k) else { break };
                if o.o_class == CLASS && o.pv_s(CHARGE) == 0 {
                    o.set_pv_s(CHARGE, 1);
                    break;
                }
            }
        }
    }
    let m = &w.mobys[id];
    let (range, area) = (m.pv_f(RANGE), m.pv_i(AREA));
    ai::find_target(w, id, ctx, range, 0.0, area, None, false);
    false
}

/// `0x4041e0`: the set-up.
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
    let weave = if w.mobys[id].pv_f(WEAVE) == -1.0 { ctx.game.rng.randf(25.0, 100.0) } else { w.mobys[id].pv_f(WEAVE) };
    let m = &mut w.mobys[id];
    m.set_pv_f(GROUND, g);
    m.set_pv_f(HEIGHT, (g + LIFT).max(m.pv_f(LOWEST)));
    m.set_pv_s(SOUND, -1);
    m.set_pv_f(WEAVE, weave * DEG);
    // `0x2ec258`: 15 and 2.4 as bytes.
    m.set_pv_b(HEALTH + 0x28, 15);
    m.set_pv_b(HEALTH + 0x2a, 2);
    m.set_pv_b(REACT + 0x10, 9);
    m.set_pv_b(REACT + 0x11, 0);
    m.set_pv_b(REACT + 0x12, 0);
    m.set_pv_b(REACT + 0x16, 0);
    m.set_pv_b(REACT + 0x17, 0);
    m.set_pv_f(REACT + 0x1c, f32::from_bits(0x3dcc_ccce));
    m.set_pv_f(REACT + 0x20, f32::from_bits(0x3d88_8889));
    m.set_pv_f(REACT + 0x28, f32::from_bits(0x3c08_8889));
    m.set_pv_f(REACT + 0x2c, f32::from_bits(0x3b88_8889));
    m.set_pv_f(REACT + 0x30, 0.5);
    m.set_pv_f(REACT + 0x34, LIFT);
    m.set_pv_f(REACT + 0x3c, f32::from_bits(0x3ecc_cccd));
    m.set_pv_b(REACT + 0x73, 0xff);
    m.set_pv_b(REACT + 0x76, 1);
    m.set_pv_f(MV, 1.0);
    m.set_pv_f(MV + 4, f32::from_bits(0x3f8c_cccd));
    m.set_pv_f(MV + 0xc, f32::from_bits(0x4033_3333));
    m.set_pv_f(MV + 0x10, std::f32::consts::PI);
    m.set_pv_f(MV + 0x30, f32::from_bits(0x41c9_0fdc));
    m.set_pv_f(MV + 0x2c, f32::from_bits(0x3a4b_e3c9));
    m.set_pv_i(REACT + 0x48, 1);
    m.set_pv_f(REACT + 0x24, f32::from_bits(0x3b88_8889));
    for o in [0x6c, 0x6d, 0x6e, 0x6f, 0x72] { m.set_pv_b(REACT + o, 0xff); }
    m.set_pv_f(MV + 8, f32::from_bits(0x4033_3333));
    let fence = if m.pv_i(FENCE_START) == -1 { m.pv_i(FENCE_ALERT) } else { m.pv_i(FENCE_START) };
    m.set_pv_i(MV + 0x3c, fence);
    m.set_pv_s(MV + 0xe4, 0xf0);
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x10);
    // The original converts the degrees twice.
    m.set_pv_f(MV + 0xdc, m.pv_f(WEAVE) * DEG);
    m.set_pv_s(MV + 0xe2, 0x78);
    senses(m);
    let p = m.pos;
    m.set_pv_v(WANDER, p);
    m.set_pv_f(WANDER + 0x10, 1.0);
    m.set_pv_f(WANDER + 0x1c, 5.0);
    m.set_pv_f(WANDER + 0x20, 10.0);
    m.set_pv_b(HEALTH + 8, 3);
    m.set_pv_i(AREA, m.pv_i(AREA_CALM));
    m.set_pv_i(HEALTH + 0x10, 0);
    let start = usize::try_from(m.pv_i(PATH)).ok().and_then(|p| ctx.level.paths.get(p)).and_then(|p| p.first().copied());
    match start {
        Some(q) if m.pv_i(PATH) != -1 => {
            m.pos = [q[0], q[1], q[2]];
            m.set_state(1, 0);
        }
        _ => m.set_state(2, 2),
    }
    set_seq(w, id, ctx, 0, 0, 1);
}

/// The sub-state (module docs).
fn subs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (alerted, sub) = (found(m) || m.pv_s(ALERT) != 0, m.sub_state);
    match sub {
        0 if alerted => {
            alarm(w, id);
            set_seq(w, id, ctx, 2, 0, 5);
            let m = &mut w.mobys[id];
            m.set_pv_f(PATH_T, 0.0);
            m.set_pv_i(AREA, m.pv_i(AREA_ALERT));
            m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 4);
            m.set_state(4, 1);
        }
        2 if alerted => {
            alarm(w, id);
            w.mobys[id].set_pv_s(ALERT, ALERT_TICKS);
            set_seq(w, id, ctx, 2, 0, 5);
            let m = &mut w.mobys[id];
            senses(m);
            m.set_pv_i(MV + 0x3c, m.pv_i(FENCE_ALERT));
            m.set_pv_i(AREA, m.pv_i(AREA_ALERT));
            m.set_state(3, 3);
        }
        3 if !alerted => {
            senses(&mut w.mobys[id]);
            set_seq(w, id, ctx, 0, 0, 0x14);
            w.mobys[id].set_state(2, 2);
        }
        _ => {}
    }
}

/// The state (module docs): true when it was deleted.
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    match w.mobys[id].state {
        2 => { leaper::wander(w, id, ctx, WANDER); }
        3 => {
            if wrapped(w, id, ctx) { w.mobys[id].set_state(5, -1); }
        }
        4 => {
            if wrapped(w, id, ctx) { set_seq(w, id, ctx, 0, 0, 0x14); }
            let Some(points) = usize::try_from(w.mobys[id].pv_i(PATH)).ok().and_then(|p| ctx.level.paths.get(p)) else { return false };
            if points.is_empty() { return false; }
            let n = points.len() as f32;
            let m = &mut w.mobys[id];
            let t = m.pv_f(PATH_T) + n / (m.pv_f(PATH_TIME) * 60.0);
            m.set_pv_f(PATH_T, t);
            if n <= t {
                m.set_pv_f(PATH_T, n);
                senses(m);
                m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !4);
                m.set_state(5, 3);
            }
            let t = m.pv_f(PATH_T);
            let (p, pitch, yaw, _) = crate::paths::pose(points, t, false);
            m.pos = p;
            m.rot[1] = pitch;
            m.rot[2] = yaw;
        }
        5 => {
            if wrapped(w, id, ctx) { set_seq(w, id, ctx, 0, 0, 0x14); }
            leaper::wander(w, id, ctx, WANDER);
            if w.mobys[id].pv_s(CHARGE) != 0 {
                senses(&mut w.mobys[id]);
                set_seq(w, id, ctx, 2, 0, 5);
                w.mobys[id].set_state(6, -1);
            }
        }
        6 => {
            if wrapped(w, id, ctx) && ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 2) {
                set_seq(w, id, ctx, 3, 0, 5);
                w.mobys[id].set_state(7, -1);
            }
        }
        7 => {
            if w.mobys[id].first() {
                let h = w.mobys[id].pv_s(SOUND) as i32;
                if !(h != -1 && ctx.host.sound_playing(h)) {
                    let h = ctx.host.play_sound(&w.mobys[id], CHARGE_SOUND, 4);
                    w.mobys[id].set_pv_s(SOUND, h as i16);
                }
            }
            let goal = w.mobys[id].pv_v(T_POINT);
            ai::move_toward(w, id, ctx, 0.0, goal);
            if ctx.hero.damage.hit_by == Some(id) || dist2(w.mobys[id].pos, ctx.hero.pos) < 1.2 {
                set_seq(w, id, ctx, 4, 0, 5);
                w.mobys[id].set_state(8, -1);
            }
        }
        8 => {
            if wrapped(w, id, ctx) && ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == 4) {
                set_seq(w, id, ctx, 0, 0, 0x14);
                w.mobys[id].set_state(5, -1);
            }
        }
        9 => {
            let f = ai::flight(w, id, ctx);
            if f & 1 != 0 {
                stop_sound(w, id, ctx);
                let m = &w.mobys[id];
                // `0x3226f0(m, v, v, 0, 0)`: across its own up.
                let v = sub(m.pos, ctx.hero.pos);
                let up = m.rows[2];
                let v = sub(v, scale(up, dot(v, up)));
                let v = if len(v) == 0.0 { v } else { set_len(v, f32::from_bits(0x3dcc_ccce)) };
                crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x3f, v, 0xff);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                bolts::drop(w, id, 0, 3, ctx);
                w.delete(id);
                return true;
            }
            if f & 2 != 0 {
                let m = &mut w.mobys[id];
                m.set_pv_s(ALERT, ALERT_TICKS);
                m.set_state(5, 3);
                ctx.host.play_sound(&w.mobys[id], 3, 0);
            }
        }
        _ => {}
    }
    false
}

/// `0x404510`: its group's living 0x0d57s alarmed (`+0x270`).
fn alarm(w: &mut MobyWorld, id: usize) {
    for k in members(w, id) {
        let Some(o) = w.mobys.get_mut(k) else { break };
        if o.state < 0x80 && o.o_class == CLASS { o.set_pv_s(ALARM, 1); }
    }
}

/// `0x404668`: alarmed, or found with the alert under half, the alert 360 again; the two timers; its state ticks.
fn timers(m: &mut Moby) {
    if m.pv_s(ALARM) != 0 || (m.pv_i(T_FOUND) == 1 && m.pv_s(ALERT) < ALERT_TICKS / 2) {
        m.set_pv_s(ALARM, 0);
        m.set_pv_s(ALERT, ALERT_TICKS);
    }
    for o in [ALERT, TIMER2] {
        let mut t = m.pv_s(o) as i32;
        countdown(&mut t);
        m.set_pv_s(o, t as i16);
    }
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

/// `0x404828`: the hover (module docs).
fn hover(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let target_z = target_place(w, id, ctx).map(|p| p[2]);
    let m = &w.mobys[id];
    let floor = ai::link(m, MV + 0x88);
    // On nothing, or on a moby that is not Ratchet nor one of them (its class's `+0x46`: not ported).
    let lift = match floor {
        None => true,
        Some(k) => Some(k) != ctx.hero_moby && w.mobys.get(k).is_some_and(|o| o.o_class != CLASS),
    };
    let m = &mut w.mobys[id];
    let z = m.pos[2];
    m.set_pv_f(GROUND, z);
    if matches!(m.state, 9 | 4) {
        m.set_pv_f(HEIGHT, z);
        return;
    }
    let ease_up = |m: &mut Moby, to: f32, k: f32| {
        let h = m.pv_f(HEIGHT);
        if h < to { m.set_pv_f(HEIGHT, h + (to - h) * k); }
    };
    m.set_pv_f(HEIGHT, m.pv_f(HEIGHT) - f32::from_bits(0x3d08_8889));
    let (ground, lowest) = (m.pv_f(GROUND) + LIFT, m.pv_f(LOWEST) + LIFT);
    if lift { ease_up(m, ground, EASE); }
    ease_up(m, lowest, EASE);
    if (6..=8).contains(&m.state) {
        if let Some(tz) = target_z { ease_up(m, tz + OVER_TARGET, EASE_TARGET); }
    }
    m.pos[2] = m.pv_f(HEIGHT);
    // Pushed away by its group's members within 3.
    let mut push = [0.0f32; 3];
    let p = w.mobys[id].pos;
    for k in members(w, id) {
        let Some(o) = w.mobys.get(k) else { break };
        let c = sub(p, o.pos);
        let d = len(c).clamp(0.0, SPREAD);
        let s = 1.0 - d / SPREAD;
        if d * s != 0.0 { push = crate::hero::physics::add(push, set_len(c, d * s)); }
    }
    let m = &mut w.mobys[id];
    m.pos = crate::hero::physics::add(m.pos, push);
    let base = m.pos[2];
    bob(m, f32::from_bits(0x3dcc_cccd), f32::from_bits(0x3db2_b8c4), base, BOB_PHASE, BOB_LAST);
    sway(m, f32::from_bits(0x3e0e_fa36), f32::from_bits(0x3cd6_7752), f32::from_bits(0x3cd6_7752), SWAY_A, SWAY_B);
}

/// `0x404b90`: charging, a 0.17 sphere at each of joints 0..3 (its damage byte, flags 0x4000001).
fn hits(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    if w.mobys[id].state != 7 { return; }
    let damage = w.mobys[id].pv_b(HEALTH + 0x14) as f32;
    for j in 0..4 {
        ai::attack_joint(w, id, ctx, f32::from_bits(0x3e2e_147b), damage, 1.0, j, 0x400_0001, 0, 1, 0);
    }
}
