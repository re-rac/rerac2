//! Joba's hover **0x0d07** (`0x430848`, 10 placed; its senses `0x433090`; L11 addresses; the decomp's v2.00
//! `func_US2_L11_00430848` / `func_US2_L11_00433090`): the walking mount the gun thug 0x0cf6 (and the thug 0x0cf5)
//! rides, on the framework (crate::moby::ai): health `+0x20`, flash `+0x50`, reaction `+0x60` (a hurt to state 26,
//! sub 3), the wander block `+0x110`, movement `+0x140` (radius 1, height 0.25, steps 0.6, slope 85°). With a rider
//! (its link `+0x27c`, written by the rider: crate::moby::classes::level11::gunners, thugs) it fires the rider's
//! gun's beam; without, it charges and butts. Its sequences go with the rider's ([`anim2`], `0x4307c0`: with a rider
//! both are set, the rider's its own number).
//!
//! Its words: the beam's muzzle `+0x230`, head `+0x240` and way `+0x250`; the range test's paths `+0x260` / `+0x264`
//! and ranges `+0x268` / `+0x26c` (`0x33c970`, its result `+0x274`, who `+0x278`); the timers `+0x270` / `+0x272`
//! (s16), alarmed `+0x276`; the ticks `+0x280` it has chased, its turn speed `+0x284`, the flash's size `+0x288`, the
//! wander's pause `+0x28c`, how far it wanders `+0x290` (under 0.2 it stands), a path to jump down `+0x294`, the
//! beam's steps `+0x298`, its patrol path `+0x29c` (and its direction `+0x2c0`), the beams fired `+0x2a0`, a cuboid to
//! go back to `+0x2a4`, the butt `+0x2a8`, an area that sends it down `+0x2ac`, its light `+0x2b0` (crate::
//! point_lights), the butt's and the body's damage `+0x2b4` / `+0x2b8` (its damage bytes `+0x34` / `+0x36`), a moby
//! it waits on `+0x2bc` (until its state 2), its hiding path `+0x2c4` (while Ratchet is in state 0x31), `+0x2c8` (the
//! rider's: 0 when it mounts; set nowhere here).
//!
//! **The set-up** ([`senses`], state 0): the thug count up (unless `0x1a9050`), on the ground; the reaction (26 / 11
//! / 11 / 13, speeds 0.1 / 1/15, gravities 1/240 / 1/120 / 1/240, sub 3, its bounce 0.4, no reactions' sequences but
//! `+0xd7`); the limits (1/450, 1/15, 0.067°, 6°); the wander home its place (radius 1.25, its range `+0x290`, 10 from
//! Ratchet); its pause 120..700; standing (1, sequences 4 / 1), its playback 0.5..1.25; waiting (2) with `+0x2bc`.
//!
//! **Each tick**: its shadow within 24; off sub 3 it falls 1.147 a tick to the ground; the frame's damage (skill point
//! flag `0x1a904c` cleared by others' hits; its beams 0, its light freed; a knock-down's spin; the rider's sequence 8
//! (a hit) or 9 (a knock-down, and its rider word `+0x26c` cleared: it gets off), the group's hovers alarmed
//! (`0x4335f0`), calm: to its patrol (12) or the stand (11, its timer 580..600)); the range test. Then the subs and
//! the states ([`update`]); the tail (`0x433590`: the flash, the timers down, its ticks); under z 74 gone with its
//! bolts.
//!
//! **The beam** (state 0x16, its rider's gun's joint 0, 0.1 ahead and 0.05 down): at frame 3 its way to Ratchet's aim
//! point (25 ahead of it past 45°), a red light (radius 5) at it; frames 3..9 the head runs a unit a tick (the line,
//! the hit: the damage byte `+0x35`, flags 1; on a hit the sparks `0x433b88` (as the rider's in `0x80ca00cb`, 6)
//! once), frames 9..15 the tail follows 1.667 a tick while they are more than 1.667 apart; twinkles at both ends and
//! the light at the head; after 15 the light freed. Before frame 3 the muzzle's twinkles (`0x804020ff` /
//! `0x7f6f6fff`, growing by `+0x288`).
//!
//! Its light word `+0x2b0` is 0 in the level's data and the set-up leaves it: a hit before its first beam frees the
//! light slot 0, as the original does.
//!
//! Not ported: the beam's draw (`0x433670` → `0x433728` / `0x433938`, registered on the first list each tick it
//! runs).

use crate::hero::physics::{add, approach, atan2, dist2, scale, set_len, sub, V3};
use crate::moby::{ai, countdown, Ctx, MobyWorld};
use crate::point_lights::PointLight;
use super::super::props::{seq, wrapped};
use super::super::leaper::seq_a;
use super::thugs as t;

pub const CLASS: i16 = 0x0d07;

const WANDER: usize = 0x110;
const MV: usize = 0x140;
const B_MUZZLE: usize = 0x230;
const B_HEAD: usize = 0x240;
const B_WAY: usize = 0x250;
const RANGE_AT: usize = 0x260;
const T270: usize = 0x270;
const T272: usize = 0x272;
const RANGE: usize = 0x274;
const ALARMED: usize = 0x276;
const WHO: usize = 0x278;
const RIDER: usize = 0x27c;
const CHASED: usize = 0x280;
const TURN_V: usize = 0x284;
const FLASH_F: usize = 0x288;
const PAUSE: usize = 0x28c;
const ROAM: usize = 0x290;
const DROP_PATH: usize = 0x294;
const STEPS: usize = 0x298;
const PATROL: usize = 0x29c;
const BEAMS: usize = 0x2a0;
const HOME_CUBOID: usize = 0x2a4;
const BUTT: usize = 0x2a8;
const AREA: usize = 0x2ac;
const LIGHT: usize = 0x2b0;
const BUTT_DMG: usize = 0x2b4;
const BODY_DMG: usize = 0x2b8;
const WAIT_ON: usize = 0x2bc;
const PATROL_DIR: usize = 0x2c0;
const HIDE_PATH: usize = 0x2c4;
const MOUNTED: usize = 0x2c8;
const PVARS: usize = 0x2d0;

/// `0x2b6c60` (v2): the walk's step along its facing by the sequence's frame (`0x433e68`).
const STRIDE: [f32; 22] = [
    0.075, 0.075, 0.1, 0.1, 0.075, 0.05, 0.075, 0.075, 0.075, 0.075, 0.075, 0.1, 0.1, 0.1, 0.075, 0.03, 0.03, 0.03, 0.03,
    0.03, 0.05, 0.0,
];

fn lim(w: &mut MobyWorld, id: usize, a: f32, b: f32, c: f32, d: f32) { ai::senses(&mut w.mobys[id], a, b, c, d); }
fn slow(w: &mut MobyWorld, id: usize) { lim(w, id, 0.003_333_333_4, 0.100_000_01, 0.001_163_553, 0.104_719_77); }
fn st(w: &mut MobyWorld, id: usize, state: u8, sub: i32) { w.mobys[id].set_state(state, sub); }
fn rider(w: &MobyWorld, id: usize) -> Option<usize> { ai::link(&w.mobys[id], RIDER).filter(|&r| r < w.mobys.len()) }
fn in_range(w: &MobyWorld, id: usize) -> bool { (1..=2).contains(&w.mobys[id].pv_s(RANGE)) }
fn in_area(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let a = w.mobys[id].pv_i(AREA);
    a != -1 && ctx.level.in_polygon(a, ctx.hero.pos)
}

/// `0x4307c0(m, s, rider's s, frame, blend)`: with a rider both sequences set, else its own unless it plays.
fn anim2(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, rs: u8, frame: u8, blend: u8) {
    if let Some(r) = rider(w, id) {
        ctx.host.set_anim(&w.mobys[id], s, frame, blend);
        ctx.host.set_anim(&w.mobys[r], rs, frame, blend);
    } else if seq(w, id, ctx) != s {
        ctx.host.set_anim(&w.mobys[id], s, frame, blend);
    }
}

/// The jump down its path (0x11 sub 2): the fast limits, the walk accel 1/180.
fn to_drop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    anim2(w, id, ctx, 6, 5, 0, 10);
    lim(w, id, 0.016_666_668, 0.5, 0.001_163_553, 0.104_719_77);
    w.mobys[id].set_pv_f(MV + 0x24, 0.005_555_555_7);
    st(w, id, 0x11, 2);
}

/// The light at `p` 1 up (`$gp`-bank record: red 1, radius 5).
fn light_at(w: &mut MobyWorld, id: usize, p: V3) {
    let s = w.mobys[id].pv_i(LIGHT);
    if s < 0 { return; }
    w.point_lights.set(s as usize, PointLight { color: [1.0, 0.0, 0.0], intensity: 0.0, pos: [p[0], p[1], p[2] + 1.0], radius: 5.0 });
}
fn free_light(w: &mut MobyWorld, id: usize) {
    let s = w.mobys[id].pv_i(LIGHT);
    if s >= 0 {
        w.point_lights.free(s as usize);
        w.mobys[id].set_pv_i(LIGHT, -1);
    }
}

/// `0x433590`: the flash, the timers down, its ticks.
fn tail(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    ai::flash(m, 0x50);
    for off in [T270, T272] {
        let mut v = m.pv_s(off) as i32;
        countdown(&mut v);
        m.set_pv_s(off, v as i16);
    }
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

/// `0x4335f0`: its group's live hovers alarmed (`+0x276`).
fn alarm(w: &mut MobyWorld, id: usize) {
    for i in t::members(w, id) {
        let m = &mut w.mobys[i];
        if m.state < 0x80 && m.o_class == CLASS { m.set_pv_s(ALARMED, 1); }
    }
}

/// `0x433090` (module docs).
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
        if w.level11.g9050 == 0 { w.level11.g338 += 1; }
        let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
        let m = &mut w.mobys[id];
        m.set_pv_i(0x18, MV as i32);
        m.pos[2] = z;
        m.set_pv_i(BEAMS, 0);
        m.set_pv_b(0x48, 20);
        m.set_pv_b(0x4a, 12);
        for (o, b) in [(0x70, 26u8), (0x71, 11), (0x72, 11), (0x76, 13), (0xcc, 0xff), (0xcd, 0xff), (0xce, 0xff), (0xcf, 0xff), (0xd2, 0xff), (0xd3, 0xff), (0xd7, 1)] { m.set_pv_b(o, b); }
        for (o, v) in [(0x7c, 0.100_000_01f32), (0x80, 0.066_666_67), (0x84, 0.004_166_666_9), (0x88, 0.008_333_334), (0x8c, 0.004_166_666_9), (0x90, 0.5), (0x94, 0.5), (0x9c, 0.4)] { m.set_pv_f(o, v); }
        m.set_pv_i(0xa8, 3);
        for (o, v) in [(MV, 1.0f32), (MV + 4, 0.25), (MV + 8, 0.6), (MV + 0xc, 0.6), (MV + 0x10, 1.483_530_04), (MV + 0x30, 1.483_530_04), (MV + 0x2c, 0.001_388_888_9)] { m.set_pv_f(o, v); }
        lim(w, id, 0.002_222_222_3, 0.066_666_67, 0.001_163_553, 0.104_719_77);
        let m = &mut w.mobys[id];
        let pos = m.pos;
        m.set_pv_v(WANDER, pos);
        m.set_pv_f(WANDER + 0x10, 1.25);
        m.set_pv_f(WANDER + 0x1c, m.pv_f(ROAM));
        m.set_pv_f(WANDER + 0x20, 10.0);
        let p = ctx.game.rng.rand_range(120, 700);
        let m = &mut w.mobys[id];
        m.set_pv_i(PAUSE, p);
        m.set_pv_b(0x28, 3);
        m.set_pv_b(0x29, 1);
        m.set_pv_f(BUTT_DMG, m.pv_b(0x34) as f32);
        m.set_pv_f(BODY_DMG, m.pv_b(0x36) as f32);
        m.set_pv_f(0x30, 0.75);
        st(w, id, 1, 0);
        anim2(w, id, ctx, 4, 1, 0, 0);
        let s = ctx.game.rng.randf(0.5, 1.25);
        t::speed(w, id, ctx, s);
        if w.mobys[id].pv_i(WAIT_ON) != -1 { st(w, id, 2, -1); }
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if w.mobys[id].sub_state != 3 {
        let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
        let m = &mut w.mobys[id];
        m.pos[2] -= 1.146_666_8;
        if m.pos[2] < g { m.pos[2] = g; }
    }
    let source = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0).and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).map(|s| s.o_class);
    let kind = ai::take_damage(w, id, ctx, false);
    if kind != 0 {
        if source.is_some_and(|c| !matches!(c, 0x0ed8 | 0x0047 | 0x0dd9 | 0x0d1b | 0x0ce7 | CLASS | 0)) { w.level11.g904c = 0; }
        w.mobys[id].set_pv_i(BEAMS, 0);
        free_light(w, id);
        if kind == 5 {
            let m = &mut w.mobys[id];
            m.set_pv_i(0xf0, 0);
            m.set_pv_f(0xf4, 0.837_758_2);
        }
        if let Some(r) = rider(w, id) {
            match kind {
                2 | 3 => ctx.host.set_anim(&w.mobys[r], 8, 0, 3),
                4 | 5 => {
                    ctx.host.set_anim(&w.mobys[r], 9, 0, 3);
                    w.mobys[r].set_pv_i(0x26c, 0);
                }
                _ => {}
            }
        }
        alarm(w, id);
        if w.mobys[id].sub_state == 0 {
            if w.mobys[id].pv_i(PATROL) != -1 {
                st(w, id, 12, 1);
                anim2(w, id, ctx, 4, 3, 0, 12);
            } else {
                st(w, id, 11, 1);
                let v = ctx.game.rng.rand_range(580, 600);
                w.mobys[id].set_pv_s(T270, v as i16);
                anim2(w, id, ctx, 6, 5, 0, 10);
            }
        }
    }
    let r = t::range_at(w, id, ctx, WHO, RANGE_AT);
    w.mobys[id].set_pv_s(RANGE, r);
}

/// 0x0d07 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.variant != 0 || ctx.hero.state == 0x6f {
        let m = &mut w.mobys[id];
        m.timer = 1;
        m.has_collision = false;
        m.mode = (m.mode | 0x41) & 0xefff;
        w.changed = true;
        return;
    }
    if w.mobys[id].timer != 0 {
        let solid = w.class(CLASS).is_some_and(|c| c.has_collision);
        let m = &mut w.mobys[id];
        m.mode = (m.mode & 0xffbe) | 0x1000;
        m.has_collision = solid;
        m.timer = 0;
        w.changed = true;
    }
    senses(w, id, ctx);
    match w.mobys[id].sub_state {
        // Calm: Ratchet near (not in 0x6f, not mounting) — down its path (0x11), or the chase (0xe sub 2); alarmed —
        // the stand (0xb sub 1; with a patrol 0xb sub 12), its timer 480..600 under 300; Ratchet in its area, down.
        0 => {
            if w.mobys[id].first_sub() {
                lim(w, id, 0.002_222_222_3, 0.066_666_67, 0.001_163_553, 0.104_719_77);
                if w.mobys[id].pv_f(ROAM) < 0.2 { anim2(w, id, ctx, 0, 0, 0, 0xc); }
                if w.mobys[id].pv_i(PATROL) != -1 {
                    anim2(w, id, ctx, 5, 4, 0, 0xc);
                    st(w, id, 5, -1);
                }
            }
            let m = &w.mobys[id];
            if m.pv_s(RANGE) == 2 && ctx.hero.state != 0x6f && m.pv_i(MOUNTED) == 0 {
                if m.pv_i(DROP_PATH) != -1 {
                    to_drop(w, id, ctx);
                } else if rider(w, id).is_some() {
                    st(w, id, 0xe, 2);
                } else {
                    anim2(w, id, ctx, 2, 2, 0, 10);
                    st(w, id, 0xe, 2);
                }
            } else if m.pv_s(ALARMED) != 0 {
                st(w, id, 0xb, 1);
                w.mobys[id].set_pv_s(ALARMED, 0);
                if w.mobys[id].pv_s(T270) < 300 {
                    let v = ctx.game.rng.rand_range(0x1e0, 0x258);
                    w.mobys[id].set_pv_s(T270, v as i16);
                }
                if w.mobys[id].pv_i(PATROL) != -1 {
                    st(w, id, 0xb, 0xc);
                    anim2(w, id, ctx, 4, 3, 0, 0xc);
                }
            } else if in_area(w, id, ctx) {
                to_drop(w, id, ctx);
            }
        }
        // Alert: in range the chase or down its path; its timer out, standing again (1 sub 0).
        1 => {
            if w.mobys[id].first_sub() { slow(w, id); }
            if in_range(w, id) {
                if rider(w, id).is_some() {
                    st(w, id, 0xe, 2);
                } else if w.mobys[id].pv_i(DROP_PATH) != -1 {
                    to_drop(w, id, ctx);
                } else {
                    anim2(w, id, ctx, 2, 2, 0, 10);
                    st(w, id, 0xe, 2);
                }
            } else if w.mobys[id].pv_s(T270) == 0 {
                st(w, id, 1, 0);
                anim2(w, id, ctx, 5, 4, 0, 0xc);
            }
        }
        // Fighting (Ratchet not in its area): out of range — the butt's end (0x14), its patrol (5 sub 1) or the
        // stand (0xb sub 1, back to its cuboid, its timer 1200..1300); Ratchet in state 0x31, its hiding path (0xd sub
        // 4); its moby to wait on not in state 2 (and not near, ridden): waiting (2 sub 0).
        2 => {
            if w.mobys[id].first_sub() { slow(w, id); }
            if !in_area(w, id, ctx) {
                if !in_range(w, id) {
                    let s = w.mobys[id].state;
                    if matches!(s, 0x11 | 0x14 | 0x10) {
                    } else if s == 0x13 {
                        anim2(w, id, ctx, 8, 0, 1, 3);
                        st(w, id, 0x14, -1);
                    } else if w.mobys[id].pv_i(PATROL) != -1 && w.mobys[id].pv_i(DROP_PATH) == -1 {
                        anim2(w, id, ctx, 5, 4, 0, 0xc);
                        st(w, id, 5, 1);
                    } else {
                        w.mobys[id].set_pv_i(BEAMS, 0);
                        anim2(w, id, ctx, 3, 1, 0, 0xc);
                        st(w, id, 0xb, 1);
                        if let Some((p, _)) = ctx.level.cuboid_place(w.mobys[id].pv_i(HOME_CUBOID)) {
                            let m = &mut w.mobys[id];
                            m.set_pv_v(WANDER, p);
                            if m.pv_f(WANDER + 0x1c) == 0.0 { m.set_pv_f(WANDER + 0x1c, 4.0); }
                        }
                        let v = ctx.game.rng.rand_range(0x4b0, 0x514);
                        w.mobys[id].set_pv_s(T270, v as i16);
                    }
                }
                if ctx.hero.state == 0x31 && w.mobys[id].pv_i(HIDE_PATH) != -1 {
                    t::anim_r(w, id, ctx, 6, 0, 5, 15);
                    st(w, id, 0xd, 4);
                    lim(w, id, 0.003_333_333_4, 0.100_000_01, 0.003_393_696_2, 0.261_799_42);
                }
                let wo = t::index(w, id, WAIT_ON);
                if wo.is_some_and(|o| w.mobys[o].state != 2) && w.mobys[id].pv_s(RANGE) != 2 && rider(w, id).is_some() {
                    st(w, id, 2, 0);
                    anim2(w, id, ctx, 4, 1, 0, 0xf);
                    let s = ctx.game.rng.randf(0.5, 1.25);
                    t::speed(w, id, ctx, s);
                }
            }
        }
        4 => {
            if ctx.hero.state != 0x31 {
                t::anim(w, id, ctx, 6, 0, 10);
                st(w, id, 0xf, 2);
            }
        }
        // Hurt (`0x33af20`, ai::flight): dead, the count down, its bolts and the fade; over, the chase (0xf; waiting
        // on a moby, the turn 0x18) in range, else the stand (its timer 580..600; with a patrol 0xb sub 12).
        3 => {
            let r = ai::flight(w, id, ctx);
            if r & 1 != 0 {
                w.level11.g338 -= 1;
                t::bolts(w, id, ctx);
                super::super::death_fade::start(w, id, ctx, 0.0, None);
                return;
            }
            if r & 2 != 0 {
                if in_range(w, id) {
                    anim2(w, id, ctx, 6, 5, 0, 10);
                    st(w, id, 0xf, 2);
                    if w.mobys[id].pv_i(WAIT_ON) != -1 {
                        st(w, id, 0x18, -1);
                        anim2(w, id, ctx, 0xf, 4, 0, 10);
                    }
                } else {
                    st(w, id, 0xb, 1);
                    anim2(w, id, ctx, 1, 1, 0, 10);
                    let v = ctx.game.rng.rand_range(0x244, 0x258);
                    w.mobys[id].set_pv_s(T270, v as i16);
                    if w.mobys[id].pv_i(PATROL) != -1 {
                        st(w, id, 0xb, 0xc);
                        anim2(w, id, ctx, 4, 3, 0, 0xc);
                    }
                }
            }
        }
        _ => {}
    }
    states(w, id, ctx);
    if w.mobys[id].state >= 0x80 { return; }
    tail(w, id);
    if w.mobys[id].pos[2] < 74.0 {
        t::bolts(w, id, ctx);
        w.level11.g338 -= 1;
        w.delete(id);
    }
}

/// The wander (`0x336100` = L00 `0x323958`, ai::wander(0.35, 0.25)) with its pause: `+0x28c` down; over 20 its speed
/// `+0x124` set to (or eased toward) `hi`, else eased to 0; in its walk sequence `walk` at the pause's end the stand
/// (`stand`, a half the time `stand2`), else at its stand's end a new pause 120..700 and the walk.
#[allow(clippy::too_many_arguments)]
fn roam(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, walk: u8, stand: (u8, u8), stand2: (u8, u8), walking: (u8, u8), stands: std::ops::RangeInclusive<u8>) {
    let s = seq(w, id, ctx);
    if s == walk {
        if w.mobys[id].pv_i(PAUSE) == 0 {
            anim2(w, id, ctx, stand.0, stand.1, 0, 0xc);
            if ctx.game.rng.randi(2) == 1 { anim2(w, id, ctx, stand2.0, stand2.1, 0, 0xc); }
        }
        ai::wander(w, id, ctx, 0.35, 0.25, WANDER);
    } else if stands.contains(&s) && wrapped(w, id, ctx) {
        let p = ctx.game.rng.rand_range(0x78, 0x2bc);
        w.mobys[id].set_pv_i(PAUSE, p);
        anim2(w, id, ctx, walking.0, walking.1, 0, 0xc);
    }
}

/// A body hit (states 0x13 / 0x14): a 1.3 sphere at its joint 1 of the body's damage, flags 0x10001, pushed along its
/// facing and up (×2.5).
fn body_hit(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let Some(p) = ctx.host.joint(&w.mobys[id], 1) else { return };
    let y = w.mobys[id].rot[2];
    let d = scale([y.cos(), y.sin(), 2.0], 2.5);
    let mut tm = crate::moby::hits::HitTemplate::new(w.mobys[id].pv_f(BODY_DMG), Some(id), 0x1_0001, [d[0], d[1], d[2], 5_627.925]);
    tm.kind = (CLASS as u16 as u32) << 16 | 1 << 8;
    w.sphere_hit(ctx.scene, p, 1.3, 0, Some(id), Some(&tm));
}

/// The gun's muzzle of its rider (the rider's `+0x278`, joint 0): 0.1 along its facing, 0.05 down.
fn muzzle(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<V3> {
    let r = rider(w, id)?;
    let g = ai::link(&w.mobys[r], 0x278).filter(|&g| g < w.mobys.len())?;
    let p = ctx.host.joint(&w.mobys[g], 0)?;
    let y = w.mobys[id].rot[2];
    Some([p[0] + y.cos() * 0.1, p[1] + y.sin() * 0.1, p[2] - 0.05])
}

/// The muzzle's twinkles of size `0.2·k` (0.2..0.9 of it), 2 ticks, `0x804020ff` and `0x7f6f6fff`.
fn muzzle_twinkles(ctx: &mut Ctx, v: V3, k: f32) {
    let rng = &mut ctx.game.rng;
    let f = rng.randf(0.2, 0.9) * 0.2 * k;
    let n = rng.randi(2);
    let n = if n == 0 { -1 } else { n } as i8;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    crate::particles::twinkle(sys, rng, f * 0.3, f, 0.0, v, 2, 0x8040_20ff, 0, n, [0.0; 3], 0);
    crate::particles::twinkle(sys, rng, f * 0.04, f * 0.4, 0.0, v, 2, 0x7f6f_6fff, 1, -n, [0.0; 3], 0);
}

/// `0x3327f0(yaw, −1, −1, m, goal, 0x10)` (L02 `0x331440`): the move with its options 0x10 for the tick.
fn strafe_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3) {
    let yaw = t::yaw_to(w, id, goal);
    let o = w.mobys[id].pv_i(MV + 0x38);
    w.mobys[id].set_pv_i(MV + 0x38, 0x10);
    ai::move_step(w, id, ctx, yaw, goal);
    w.mobys[id].set_pv_i(MV + 0x38, o);
}

/// The chase's end: with a rider the beam's wind-up (0x12, sequences 9 / 6), else the chase (0xf, 6 / 5); the walk
/// accel 1/1200.
fn chase_or_aim(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_f(MV + 0x24, 0.000_833_333_35);
    if rider(w, id).is_some() {
        lim(w, id, 0.002_222_222_3, 0.066_666_67, 0.001_163_553, 0.104_719_77);
        anim2(w, id, ctx, 9, 6, 0, 10);
        st(w, id, 0x12, -1);
    } else {
        slow(w, id);
        anim2(w, id, ctx, 6, 5, 0, 10);
        st(w, id, 0xf, -1);
    }
}

/// `0x433e68(m, target)`: the walk toward `target` — in a blend toward it (`0x33f6c0`); else along its facing by the
/// frame's stride ([`STRIDE`]), turned toward it in place (`0x33fa78`), and at frames 5 / 15 ten dust puffs at its
/// joint 3 / 2 half a unit ahead (`0x3499f0`: 0x406080, 60 ticks, 0.3..1 × 210000).
fn walk_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, target: V3) {
    if seq_a(w, id, ctx) != seq(w, id, ctx) {
        t::go(w, id, ctx, target);
        return;
    }
    let frame = ctx.host.frame_time(&w.mobys[id]);
    let k = frame as i32;
    let at = |i: i32| usize::try_from(i).ok().and_then(|i| STRIDE.get(i)).copied().unwrap_or(0.0);
    let step = at(k) + (at(k + 1) - at(k)) * (frame - k as f32);
    let m = &mut w.mobys[id];
    m.pos = add(scale(m.rows[0], step), m.pos);
    w.changed = true;
    let (y, pos) = (t::yaw_to(w, id, target), w.mobys[id].pos);
    ai::move_step(w, id, ctx, y, pos);
    if k != 5 && k != 15 { return; }
    let Some(j) = ctx.host.joint(&w.mobys[id], if k == 5 { 3 } else { 2 }) else { return };
    let mut origin = add(j, scale(w.mobys[id].rows[0], 0.5));
    origin[2] -= 0.2;
    let rng = &mut ctx.game.rng;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    for _ in 0..10 {
        let vel = rng.rand_vec(0.005, 0.05);
        let kind = rng.randf_signed(1.0, 3.0) as i32;
        let size = rng.randf(0.3, 1.0) * 210_000.0;
        let mut p = origin;
        for c in &mut p { *c += rng.randf_signed(0.0, 0.3); }
        if let Some(i) = crate::particles::puff(sys, rng, 0.0, 1.0, 1.01, size, p, kind, vel, 0x0040_6080, 0) {
            let r = &mut sys.pool.recs[i];
            crate::particles::rec::set_i16(r, 0xa, 60);
            r[3] = 0x44;
            r[9] = 0x44;
            crate::particles::rec::set_ff(r, 0x3c, 5.0);
            crate::particles::rec::set_u32(r, 0x24, 3);
            r[0x2a] = 0x20;
            r[0x2b] = r[0xa];
        }
    }
}

/// One step of the beam's head (`+0x298` + 1 along its way from the muzzle), its line and its hit (the damage byte
/// `+0x35`, flags 1); on a hit the head there and, once, the sparks.
fn beam_step(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let hp = w.mobys[id].pv_b(0x35) as f32;
    let m = &mut w.mobys[id];
    let steps = m.pv_f(STEPS) + 1.0;
    m.set_pv_f(STEPS, steps);
    let (mz, way) = (m.pv_v(B_MUZZLE), m.pv_v(B_WAY));
    let v = set_len(way, steps);
    let mut tm = crate::moby::hits::HitTemplate::new(hp, Some(id), 1, [v[0], v[1], v[2], 0.0]);
    tm.kind = (CLASS as u16 as u32) << 16;
    let head = add(mz, v);
    w.mobys[id].set_pv_v(B_HEAD, head);
    if let Some(p) = super::gunners::line_hit(w, ctx, id, mz, head, &tm) {
        w.mobys[id].set_pv_v(B_HEAD, p);
        if w.mobys[id].once & 4 == 0 {
            super::super::level04::rider::sparks_in(ctx, p, 6, 0x80ca_00cb);
            w.mobys[id].once |= 4;
        }
    }
}

/// The beam's head twinkles: `0xff000080` (25 ticks), `0x7f6f6fff` (5), `0xff4080ff` (20), 0.1 under the head.
fn head_twinkles(ctx: &mut Ctx, head: V3, muzzle_too: Option<V3>) {
    let rng = &mut ctx.game.rng;
    let f = rng.randf(1.0, 6.0);
    let n = rng.rand_range(0x19, 0x23);
    let n = if n == 0 { -1 } else { n } as i8;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let v = [head[0], head[1], head[2] - 0.1];
    crate::particles::twinkle(sys, rng, f * 0.3, f, 0.0, v, 0x19, 0xff00_0080, 0, n, [0.0; 3], 0);
    crate::particles::twinkle(sys, rng, f * 0.04, f * 0.4, 0.0, v, 5, 0x7f6f_6fff, 1, -n, [0.0; 3], 0);
    crate::particles::twinkle(sys, rng, f * 0.4, f * 0.2, 0.0, v, 0x14, 0xff40_80ff, 1, -n, [0.0; 3], 0);
    if let Some(m) = muzzle_too {
        crate::particles::twinkle(sys, rng, f * 0.3, f, 0.0, m, 10, 0x80ca_00cb, 0, -n, [0.0; 3], 0);
        crate::particles::twinkle(sys, rng, f * 0.04, f * 0.4, 0.0, m, 10, 0x7f6f_6fff, 1, n, [0.0; 3], 0);
    }
}

/// The second switch, on the state.
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        // Standing / wandering (with a range of 0.2 or more): the walk 5 with its pauses, the stands 0 / 1.
        1 => {
            if 0.2 < w.mobys[id].pv_f(ROAM) {
                pause_speed(w, id, 0.066_666_67, 0.001_111_111_1, true);
                roam(w, id, ctx, 5, (1, 1), (0, 0), (5, 4), 0..=1);
            } else if let Some(r) = rider(w, id) {
                if seq(w, r, ctx) != 0 { anim2(w, id, ctx, 0, 0, 0, 0xc); }
            }
        }
        // Waiting on `+0x2bc`: its state 2 with a rider, the rider's sequence 0xb (3).
        2 => {
            if let (Some(o), Some(r)) = (t::index(w, id, WAIT_ON), rider(w, id)) {
                if w.mobys[o].state == 2 {
                    st(w, id, 3, -1);
                    ctx.host.set_anim(&w.mobys[r], 0xb, 0, 0xf);
                }
            }
        }
        3 => {
            if rider(w, id).is_none_or(|r| wrapped(w, r, ctx)) { st(w, id, 0xe, 2); }
        }
        // Along its patrol path (5 forward, 6 back): at its end the turn (7, its direction kept), the path again.
        5 | 6 => {
            let fwd = w.mobys[id].state == 5;
            let p = w.mobys[id].pv_i(PATROL);
            if p != -1 && t::follow(w, id, ctx, PATROL) {
                st(w, id, 7, -1);
                anim2(w, id, ctx, 4, 3, 0, 10);
                if fwd { t::path_start(w, id, ctx, p, 0, 1) } else { t::path_start(w, id, ctx, p, 1, 0) }
                w.mobys[id].set_pv_i(PATROL_DIR, fwd as i32);
            }
        }
        7 => {
            if wrapped(w, id, ctx) {
                anim2(w, id, ctx, 0xe, 0xa, 0, 7);
                st(w, id, 0x19, -1);
            }
        }
        13 => {
            let p = w.mobys[id].pv_i(HIDE_PATH);
            ai::follow_path(w, id, ctx, p);
        }
        // The stand (alarmed): the wander with its pauses in the walk 6, the stands 3 / 4.
        11 => {
            pause_speed(w, id, 0.066_666_67, 0.001_944_444_5, false);
            roam(w, id, ctx, 6, (3, 2), (4, 3), (6, 5), 3..=4);
        }
        12 => {
            if wrapped(w, id, ctx) {
                anim2(w, id, ctx, 5, 4, 0, 0xc);
                st(w, id, 5, -1);
            }
        }
        // Turned to Ratchet: ridden, the turn (0x18); with `+0x2a8` the butt (0x13); else the chase (0xf).
        14 => {
            let y = t::yaw_to(w, id, ctx.hero.pos);
            ai::turn_to(&mut w.mobys[id], y, 0.001, 0.01, 0.1, TURN_V);
            if rider(w, id).is_some() {
                st(w, id, 0x18, -1);
                anim2(w, id, ctx, 0xf, 4, 0, 10);
            } else if w.mobys[id].pv_i(BUTT) != 0 {
                st(w, id, 0x13, -1);
                anim2(w, id, ctx, 7, 5, 0, 10);
            } else {
                st(w, id, 0xf, -1);
                anim2(w, id, ctx, 6, 5, 0, 10);
            }
        }
        // The chase ([`walk_to`] Ratchet): ridden, within 20 and 10° the beam's wind-up (0x12); else within 2.7 and
        // 20° the butt (0x15, sequence 0xc), and past 2.7 in sequence 5 / 0xf the walk 6 again.
        15 => {
            let h = ctx.hero.pos;
            walk_to(w, id, ctx, h);
            let m = &mut w.mobys[id];
            m.set_pv_i(CHASED, m.pv_i(CHASED) + 1);
            let d = t::hdist(w, id, ctx);
            if rider(w, id).is_some() {
                if d < 20.0 && t::faces_hero(w, id, ctx, 0.174_532_95) {
                    anim2(w, id, ctx, 9, 6, 0, 10);
                    st(w, id, 0x12, -1);
                }
            } else {
                if d < 2.7 && t::faces_hero(w, id, ctx, 0.349_065_9) {
                    anim2(w, id, ctx, 0xc, 0, 0, 5);
                    st(w, id, 0x15, -1);
                }
                if 2.7 < d && matches!(seq_a(w, id, ctx), 5 | 0xf) {
                    anim2(w, id, ctx, 6, 5, 0, 4);
                    slow(w, id);
                }
            }
        }
        // The butt's run: toward Ratchet fast (walk accel 1/144), turning to him; within 2.1 its end (0x14, sequence
        // 8, playback 1); its body hit all the while.
        19 => {
            w.mobys[id].set_pv_f(MV + 0x24, 0.006_944_444_5);
            lim(w, id, 0.006_666_667, 0.200_000_02, 0.001_163_553, 0.104_719_77);
            let h = ctx.hero.pos;
            t::go(w, id, ctx, h);
            let y = t::yaw_to(w, id, h);
            ai::turn_to(&mut w.mobys[id], y, 0.001, 0.01, 0.1, TURN_V);
            let m = &mut w.mobys[id];
            m.set_pv_i(CHASED, m.pv_i(CHASED) + 1);
            if t::hdist(w, id, ctx) < 2.1 {
                anim2(w, id, ctx, 8, 0, 1, 3);
                t::speed(w, id, ctx, 1.0);
                st(w, id, 0x14, -1);
            }
            body_hit(w, id, ctx);
        }
        // The butt's end: toward Ratchet; at its sequence's end the back-off (0x10, sequence 2), within 3 of him the
        // retreat (0x1b, sequence 0x10); its body hit.
        20 => {
            lim(w, id, 0.004_166_667, 0.033_333_335, 0.001_163_553, 0.104_719_77);
            let h = ctx.hero.pos;
            t::go(w, id, ctx, h);
            let m = &mut w.mobys[id];
            m.set_pv_i(CHASED, m.pv_i(CHASED) + 1);
            if wrapped(w, id, ctx) {
                anim2(w, id, ctx, 2, 2, 0, 10);
                st(w, id, 0x10, -1);
                if t::hdist(w, id, ctx) < 3.0 {
                    lim(w, id, 0.002_222_222_3, 0.066_666_67, 0.001_163_553, 0.104_719_77);
                    t::anim(w, id, ctx, 0x10, 0, 10);
                    st(w, id, 0x1b, -1);
                }
            }
            body_hit(w, id, ctx);
        }
        // The retreat: backing 5.5 along its facing (`0x3327f0`, options 0x10); after 120 ticks the chase's end.
        27 => {
            lim(w, id, 0.001_111_111_1, 0.033_333_335, 0.001_163_553, 0.104_719_77);
            let m = &w.mobys[id];
            let v = add(scale(m.rows[0], -5.5), m.pos);
            strafe_to(w, id, ctx, v);
            if 0x79 <= w.mobys[id].state_ticks as u16 { chase_or_aim(w, id, ctx); }
        }
        16 => {
            if wrapped(w, id, ctx) { chase_or_aim(w, id, ctx); }
        }
        // Down its path (`+0x294`): at its end the turn (0x18), the path and the area gone; no rider, the chase.
        17 => {
            slow(w, id);
            if t::follow(w, id, ctx, DROP_PATH) {
                st(w, id, 0x18, -1);
                anim2(w, id, ctx, 0xf, 4, 0, 10);
                slow(w, id);
                let m = &mut w.mobys[id];
                m.set_pv_f(MV + 0x24, 0.000_833_333_35);
                m.set_pv_i(DROP_PATH, -1);
                m.set_pv_i(AREA, -1);
                if rider(w, id).is_none() { st(w, id, 0xe, 2); }
            }
        }
        // The beam's wind-up (the rider's sequence 9): the muzzle's twinkles growing with the frame; at its end the
        // beam (0x16, sequences 0xa / 7).
        18 => {
            if rider(w, id).is_some() && seq_a(w, id, ctx) == 9 {
                let g = ctx.host.frame_time(&w.mobys[id]);
                if let Some(v) = muzzle(w, id, ctx) { muzzle_twinkles(ctx, v, g); }
                w.mobys[id].set_pv_f(FLASH_F, g);
            }
            if wrapped(w, id, ctx) {
                anim2(w, id, ctx, 0xa, 7, 0, 10);
                st(w, id, 0x16, -1);
            }
        }
        // The butt: frames 10..15 of its sequence 0xc a 1.0 sphere at joint 1 (its butt's damage, flags 1); at its
        // end again when Ratchet is still close and ahead, else the chase.
        21 => {
            let f = ctx.host.frame_time(&w.mobys[id]);
            if seq_a(w, id, ctx) == 0xc && (10.0..=15.0).contains(&f) {
                let dmg = w.mobys[id].pv_f(BUTT_DMG);
                ai::attack_joint(w, id, ctx, 1.0, dmg, 1.0, 1, 1, 0, 1, 0);
            }
            if wrapped(w, id, ctx) {
                if t::hdist(w, id, ctx) < 2.7 && t::faces_hero(w, id, ctx, 0.349_065_9) {
                    anim2(w, id, ctx, 0xc, 0, 0, 10);
                    st(w, id, 0x15, -1);
                } else {
                    st(w, id, 0xf, -1);
                    anim2(w, id, ctx, 6, 5, 0, 10);
                    slow(w, id);
                }
            }
        }
        22 => beam(w, id, ctx),
        // After three beams away from Ratchet: at its end the turn (0x18) within 25 ridden, else the chase; within 6
        // the wind-up.
        23 => {
            if wrapped(w, id, ctx) {
                if t::hdist(w, id, ctx) < 25.0 && rider(w, id).is_some() {
                    st(w, id, 0x18, -1);
                    anim2(w, id, ctx, 5, 4, 0, 10);
                } else {
                    st(w, id, 0xf, -1);
                    anim2(w, id, ctx, 6, 5, 0, 10);
                }
            } else if t::hdist(w, id, ctx) < 6.0 {
                anim2(w, id, ctx, 9, 6, 0, 10);
                st(w, id, 0x12, -1);
            }
        }
        // Turning to Ratchet: within 5° the wind-up.
        24 => {
            if t::faces_hero(w, id, ctx, 0.087_266_48) {
                anim2(w, id, ctx, 9, 6, 0, 10);
                st(w, id, 0x12, -1);
            } else {
                let h = ctx.hero.pos;
                t::face(w, id, ctx, h);
            }
        }
        // The turn at its patrol's end: at its end along it again (5 / 6 by `+0x2c0`); in sequence 0xe the fast turn
        // along it.
        25 => {
            if wrapped(w, id, ctx) {
                anim2(w, id, ctx, 5, 4, 0, 0);
                let s = if w.mobys[id].pv_i(PATROL_DIR) == 0 { 5 } else { 6 };
                st(w, id, s, -1);
            } else if seq(w, id, ctx) == 0xe {
                lim(w, id, 0.133_333_34, 0.066_666_67, 0.009_696_275, 0.698_131_8);
                t::follow(w, id, ctx, PATROL);
            }
        }
        _ => {}
    }
}

/// The wander's pause (`+0x28c`) down and its speed `+0x124`: over 20 left `hi` (state 1: set; 11: eased), else
/// eased toward 0 by `step`.
fn pause_speed(w: &mut MobyWorld, id: usize, hi: f32, step: f32, set: bool) {
    let m = &mut w.mobys[id];
    let mut p = m.pv_i(PAUSE);
    countdown(&mut p);
    m.set_pv_i(PAUSE, p);
    let mut v = m.pv_f(WANDER + 0x14);
    if 0x15 <= p {
        if set { v = hi } else { approach(hi, step, &mut v); }
    } else {
        approach(0.0, step, &mut v);
    }
    m.set_pv_f(WANDER + 0x14, v);
}

/// State 0x16, the beam (module docs).
fn beam(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if rider(w, id).is_none() {
        st(w, id, 0xf, -1);
        anim2(w, id, ctx, 6, 5, 0, 10);
    }
    if wrapped(w, id, ctx) {
        let (d, r) = (t::hdist(w, id, ctx), rider(w, id).is_some());
        let n = w.mobys[id].pv_i(BEAMS);
        if d < 25.0 && r && n < 3 {
            if rot_far(w, id, ctx) {
                st(w, id, 0x18, -1);
                anim2(w, id, ctx, 0xf, 4, 0, 10);
            } else {
                anim2(w, id, ctx, 9, 6, 0, 10);
                st(w, id, 0x12, -1);
            }
        } else if 10.0 < d && r && 3 <= n && w.mobys[id].pv_i(WAIT_ON) == -1 {
            anim2(w, id, ctx, 3, 2, 0, 10);
            st(w, id, 0x17, -1);
            w.mobys[id].set_pv_i(BEAMS, 0);
        } else {
            st(w, id, 0x18, -1);
            anim2(w, id, ctx, 0xf, 4, 0, 10);
        }
        if r {
            let m = &mut w.mobys[id];
            m.set_pv_i(BEAMS, m.pv_i(BEAMS) + 1);
        } else {
            st(w, id, 0xf, -1);
            anim2(w, id, ctx, 6, 5, 0, 10);
            return;
        }
    } else if ai::reached(w, id, ctx, 3.0) && rider(w, id).is_some() {
        if let Some(v0) = muzzle(w, id, ctx) {
            let to = if t::faces_hero(w, id, ctx, 0.785_398_24) {
                ctx.hero.aim_point
            } else {
                let (p, y) = (w.mobys[id].pos, w.mobys[id].rot[2]);
                [p[0] + y.cos() * 25.0, p[1] + y.sin() * 25.0, p[2]]
            };
            let m = &mut w.mobys[id];
            m.set_pv_v(B_MUZZLE, v0);
            m.set_pv_v(B_WAY, sub(to, v0));
            m.set_pv_f(STEPS, 0.0);
            m.once &= !4;
            let pos = m.pos;
            let s = w.point_lights.alloc(PointLight { color: [1.0, 0.0, 0.0], intensity: 0.0, pos, radius: 5.0 }).map_or(-1, |s| s as i32);
            w.mobys[id].set_pv_i(LIGHT, s);
            light_at(w, id, v0);
        }
    }
    let f = ctx.host.frame_time(&w.mobys[id]);
    let firing = seq_a(w, id, ctx) == 0xa;
    if (f < 3.0 || !firing) && rider(w, id).is_some() {
        let k = w.mobys[id].pv_f(FLASH_F);
        if let Some(v) = muzzle(w, id, ctx) { muzzle_twinkles(ctx, v, k); }
        let m = &mut w.mobys[id];
        m.set_pv_f(FLASH_F, k + 1.0);
    }
    if (3.0..9.0).contains(&f) && firing {
        beam_step(w, id, ctx);
        let (head, mz) = (w.mobys[id].pv_v(B_HEAD), w.mobys[id].pv_v(B_MUZZLE));
        head_twinkles(ctx, head, Some(mz));
        light_at(w, id, head);
    }
    if (9.0..15.0).contains(&f) && firing {
        let m = &w.mobys[id];
        if 1.666_666_7 < dist2(m.pv_v(B_MUZZLE), m.pv_v(B_HEAD)) {
            beam_step(w, id, ctx);
            let m = &mut w.mobys[id];
            let mz = add(m.pv_v(B_MUZZLE), set_len(m.pv_v(B_WAY), 1.666_666_7));
            m.set_pv_v(B_MUZZLE, mz);
            let head = m.pv_v(B_HEAD);
            head_twinkles(ctx, head, None);
            light_at(w, id, head);
        } else {
            free_light(w, id);
        }
    } else if 15.0 < f && firing {
        free_light(w, id);
    }
}

/// Ratchet more than 5° off its facing (the beam's end).
fn rot_far(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let y = atan2(ctx.hero.pos[0] - w.mobys[id].pos[0], ctx.hero.pos[1] - w.mobys[id].pos[1]);
    0.087_266_475 < crate::hero::physics::rot_dist(w.mobys[id].rot[2], y)
}
