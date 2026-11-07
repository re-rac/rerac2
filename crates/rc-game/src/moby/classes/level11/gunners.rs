//! Joba's gun thug **0x0cf6** (`0x427728`, 9 placed; its senses `0x42adb8`; L11 addresses; the decomp's v2.00
//! `func_US2_L11_00427728` / `func_US2_L11_0042ADB8`) and its shot **0x0d1b** (`0x435dc0`, made by `0x436238`): the
//! thug 0x0cf5's cousin (crate::moby::classes::level11::thugs) with a gun 0x0ff9 for the club, on the same framework
//! and in the same words (health `+0x20`, flash `+0x50`, reaction `+0x60`: a hurt to state 31, sub 4, the wander block
//! `+0x110`, movement `+0x140`, the aim record `+0x2a0`, its network `+0x320..`). Its helpers are the thug's code
//! again: the network walk `0x42b988` (= `0x426fa0`, thugs::net_walk), the seat's hit `0x4276d8` (= `0x422438`), the
//! tail `0x42bc50` (= `0x427268`, thugs::tail), the alarm `0x42bcf8` (the same as `0x427310`, thugs::alarm), the
//! sparks `0x42bde0` (= `0x4273f8`, crate::moby::classes::level04::rider::sparks), the range test `0x33c970`.
//!
//! Its words where they differ from the thug's: the gun 0x0ff9 `+0x278` (on joint 2, turned half round about its up;
//! on a hover on joint 1), a cuboid to run to `+0x57c` (its place `+0x30`), the muzzle's flash ticks `+0x574`, the
//! shot's muzzle `+0x580` and way `+0x5a0`, `+0x578` / `+0x5b0` cleared (nothing reads them), `+0x5b4` a sequence
//! choice, the reaction path `+0x5b8`.
//!
//! **The set-up** (state 0, [`senses`]): update distance 0x80, the thug count up (unless `0x1a9050`); on the ground;
//! its network; its yaw offset; mirrored at random (mode 0x8000, the hooked bit); the reaction (31 sub 4, sequences
//! 6 / 7 / 8, speeds 0.2 / 0.125, gravities 1/180 / 7/720 / 1/240, the hurt share 0.18), movement (radius / height
//! 0.75, steps 0.6, slope 85°, the turn before walking 85°, stop 1.5); its damage. On a hover (`+0x26c`; 1, sub 5):
//! its place and angles, no collision, the hover's rider link `+0x27c`, the seat 0x0911 0.75 above it, the gun on
//! joint 1 (no hover: gone). On foot (4, sub 0, sequence 0): chatting (5: sequences 0xd / 0xf from frame 5 / 0xf) or
//! patrolling (11, sequence 4); the weapon 0x09ac on joint 3 (no collision) and the gun on joint 2.
//!
//! **Each tick** ([`senses`]): its shadow within 24; `+0x2e` = 2: its parts and itself gone; the weapon on joint 3, the
//! gun on joint 2; at 1 health or less the weapon drops; the frame's damage — a hit not from its own kind clears the
//! skill point flag `0x1a904c`, off its turret (the turret back to 4), playback 1, a knock-down's spin and the
//! weapon gone, the flash 0xd2, its group alarmed (not in sub 2), a kill or knock-down gives the charge to the first
//! 0x0cf6 of its group without it; the range test; off sub 4 it falls 0.313 a tick to the ground; under z 61 (after
//! the sixth frame) gone with its bolts, the charge handed on.
//!
//! **Subs** (`+0x95`) and **states** follow the original's two switches ([`update`], [`states`]); the comments
//! there. Its shots ([`fire`], `0x436238`): the bolt 0x0d1b from the gun's joint 0 (0.1 ahead, 0.05 down) toward
//! Ratchet's aim point (more than 45° off its facing: 25 ahead of it), the muzzle's twinkles (`0xaf40df20` /
//! `0x20ffffff`) while its firing sequences play.
//!
//! **The shot 0x0d1b** ([`update_0d1b`]): its pvars `+0x00` the tail, `+0x10` the head, `+0x20` its way, `+0x34` the
//! steps, `+0x38` its ticks, `+0x3c` its damage. For 25 ticks the head runs out a unit a tick along the way; then
//! for 9 the tail catches up 1.667 a tick while they are more than 1 apart; then gone. Each step the line from the
//! tail to the head (flags 0, its owner's hit: the damage along the way 1/15, flags 0x10001): on a hit, the head there,
//! the green sparks (`0x436788`: as the rider's in `0x8020ff20`, 6 / 3) once (the hooked bit), gone. Its twinkles: at
//! the head `0xaf20ff20` (15 ticks; in the second part also `0x8040df20`, 20), at the tail `0x4000af20` and a small
//! `0x20ffffff` (10).
//!
//! Not ported: the bolt's beam draws (`0x4362e8` → `0x436338` / `0x436540`); the debug print `0x26feb8`.

use crate::hero::physics::{add, add_rot, atan2, dist2, euler_rows, rot_dist, set_len, sub, sub_rot, V3};
use crate::moby::{ai, Ctx, MobyWorld};
use super::super::props::{seq, wrapped};
use super::super::leaper::seq_a;
use super::thugs as t;

pub const CLASS: i16 = 0x0cf6;
pub const SHOT: i16 = 0x0d1b;
const GUN_CLASS: i16 = 0x0ff9;

const HEALTH: usize = 0x20;
const WANDER: usize = 0x110;
const MV: usize = 0x140;
const T260: usize = 0x260;
const RANGE: usize = 0x264;
const ALARMED: usize = 0x266;
const RIDES: usize = 0x26c;
const HOVER_I: usize = 0x270;
const HOVER_LINK: usize = 0x274;
const GUN: usize = 0x278;
const SEAT: usize = 0x280;
const APATH: usize = 0x284;
const FENCE: usize = 0x288;
const DAMAGE: usize = 0x28c;
const ATTACK: usize = 0x290;
const CHARGE: usize = 0x294;
const OFFSET: usize = 0x298;
const WEAPON: usize = 0x29c;
const AIM: usize = 0x2a0;
const HEAD_TURN: usize = AIM + 0x68;
const NODES: usize = 0x320;
const NODES_MAX: usize = 16;
const N: usize = 0x520;
const START: usize = 0x524;
const NET: usize = 0x528;
const CHAT: usize = 0x540;
const PARTNER_I: usize = 0x544;
const TURRET: usize = 0x548;
const PATROL: usize = 0x54c;
const SPRING_V: usize = 0x550;
const CLIMB: usize = 0x554;
const ALARM_PATH: usize = 0x558;
const PARTNER: usize = 0x55c;
const ALARMS: usize = 0x560;
const T_PATH: usize = 0x564;
const T_POINT: usize = 0x568;
const HOME_PATH: usize = 0x56c;
const SWING_FIRST: usize = 0x570;
const FLASH_T: usize = 0x574;
const W578: usize = 0x578;
const RUN_TO: usize = 0x57c;
const MUZZLE: usize = 0x580;
const WAY: usize = 0x5a0;
const W5B0: usize = 0x5b0;
const ALT_SEQ: usize = 0x5b4;
const REACT_PATH: usize = 0x5b8;
const PVARS: usize = 0x5c0;

const DEG: f32 = 0.017_453_294;
const HALF_PI: f32 = 1.570_796_5;

/// `0x32dcf0(a, b, c, d, m)`: its limits.
fn lim(w: &mut MobyWorld, id: usize, a: f32, b: f32, c: f32, d: f32) { ai::senses(&mut w.mobys[id], a, b, c, d); }
fn calm(w: &mut MobyWorld, id: usize) { lim(w, id, 0.001_527_777_9, 0.045_833_334, 0.001_163_553, 0.104_719_77); }
fn walk(w: &mut MobyWorld, id: usize) { lim(w, id, 0.001_527_777_9, 0.045_833_334, 0.003_781_547, 0.296_706_02); }
fn run(w: &mut MobyWorld, id: usize) { lim(w, id, 0.003_888_889, 0.116_666_675, 0.001_163_553, 0.104_719_77); }
fn run_turn(w: &mut MobyWorld, id: usize) { lim(w, id, 0.003_888_889, 0.116_666_675, 0.003_393_696_2, 0.261_799_42); }
fn run_close(w: &mut MobyWorld, id: usize) { lim(w, id, 0.003_888_889, 0.116_666_675, 0.003_587_621_7, 0.279_252_74); }

/// `0x2a7470(m, s, frame, blend)` unless `s` plays.
fn anim(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, frame: u8, blend: u8) { t::anim(w, id, ctx, s, frame, blend); }
fn st(w: &mut MobyWorld, id: usize, state: u8, sub: i32) { w.mobys[id].set_state(state, sub); }
/// The move in place facing Ratchet (`0x33f8e0`).
fn face_hero(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { let h = ctx.hero.pos; t::face(w, id, ctx, h); }
/// Ratchet within 3.5 across and within 20° of its facing.
fn close_facing(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool { t::hdist(w, id, ctx) < 3.5 && t::faces_hero(w, id, ctx, 0.349_065_9) }
/// The fire 0x1c (sequence 5 blended 6, the run limits), sub 2.
fn to_shot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    anim(w, id, ctx, 5, 0, 6);
    run(w, id);
    st(w, id, 0x1c, 2);
}
/// The close fire 0x1d (sequence 5 blended 6, the close limits), sub 2.
fn to_close(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    anim(w, id, ctx, 5, 0, 6);
    run_close(w, id);
    st(w, id, 0x1d, 2);
}

/// The turret (`+0x548`), as an index.
fn turret(w: &MobyWorld, id: usize) -> Option<usize> { t::index(w, id, TURRET) }

/// The group's first member (of `class`, any when None) without the charge given it, while two of the group live.
fn hand_charge(w: &mut MobyWorld, id: usize, class: Option<i16>) {
    if !t::group_pair(w, id) { return; }
    let first = t::members(w, id).into_iter().find(|&i| class.is_none_or(|c| w.mobys[i].o_class == c) && w.mobys[i].pv_i(CHARGE) == 0);
    if let Some(i) = first { w.mobys[i].set_pv_i(CHARGE, 1); }
}

/// `0x2ac728(m, part, 0.5, 3, −1)` and `0x29f338`: the part at `off` gone in a smoke burst.
fn drop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, off: usize) { t::drop_part(w, id, ctx, off, true); }

/// The gun 0x0ff9 at its joint 2 (`0x29fc20`): its place the joint's, its rows the joint's with the first turned half
/// round about the third (`0x2acec0`) and made unit (`0x2ab028`); its animation advanced; mode | 6.
fn gun_on_joint(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, g: usize) {
    let Some((r, p)) = ctx.host.joint_matrix(&w.mobys[id], 2) else { return };
    ctx.host.advance(&w.mobys[g]);
    let r0 = crate::camera::rot(r[0], std::f32::consts::PI, r[2]);
    let m = &mut w.mobys[g];
    m.pos = p;
    m.rows = [r0, r[1], r[2]].map(|row| set_len(row, 1.0));
    m.mode |= crate::moby::mode::NO_UPDATE | crate::moby::mode::NO_MATRIX | crate::moby::mode::KEEP_ROWS;
    w.changed = true;
}

/// The gun 0x0ff9 made (`+0x278`; drawn, its owner's light) at joint `joint`; false when it cannot be.
fn make_gun(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, joint: usize) -> bool {
    let Some(g) = t::make_part(w, id, ctx, GUN_CLASS, joint, true) else { return false };
    ai::set_link(&mut w.mobys[id], GUN, Some(g));
    true
}

/// `0x42adb8` (module docs). True when it is to be deleted.
fn senses(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
        m.update_dist = 0x80;
        if w.level11.g9050 == 0 { w.level11.g338 += 1; }
        let m = &mut w.mobys[id];
        m.set_pv_i(W5B0, 0);
        m.set_pv_i(0x18, MV as i32);
        let z = ai::ground_z(ctx, 0.5, m.pos, 0);
        let m = &mut w.mobys[id];
        m.pos[2] = z;
        for o in [CLIMB, ALARMS, T_POINT] { m.set_pv_i(o, 0); }
        m.set_pv_b(0x48, 20);
        m.set_pv_b(0x4a, 14);
        if m.pv_i(NET) != 0 {
            let mut nodes = crate::path_network::read_nodes(&m.pvars, NODES, NODES_MAX);
            let n = crate::path_network::build(ctx.level, m.pv_i(START), &mut nodes);
            crate::path_network::write_nodes(&mut m.pvars, NODES, &nodes[..n]);
            m.set_pv_i(N, n as i32);
            crate::path_network::open(&mut ctx.game.path_openings, ctx.level, &nodes[..n]);
        }
        let off = w.mobys[id].pv_f(OFFSET);
        let off = if off == 0.0 { ctx.game.rng.randf_signed(15.0, 25.0) * DEG } else { off * DEG };
        w.mobys[id].set_pv_f(OFFSET, off);
        if ctx.game.rng.randi(2) == 1 {
            let m = &mut w.mobys[id];
            m.mode |= 0x8000;
            m.once |= 4;
        }
        let m = &mut w.mobys[id];
        m.set_pv_b(0x70, 31);
        m.set_pv_i(0xa8, 4);
        for (o, b) in [(0x72, 7u8), (0x71, 6), (0x76, 8), (0xcc, 5), (0xcd, 10), (0xce, 8), (0xcf, 15), (0xd2, 6), (0xd3, 11)] { m.set_pv_b(o, b); }
        for (o, v) in [(0x7c, 0.200_000_02f32), (0x80, 0.125), (0x84, 0.005_555_555_7), (0x88, 0.009_722_222_6), (0x8c, 0.004_166_666_9), (0x90, 0.75), (0x94, 0.75)] { m.set_pv_f(o, v); }
        m.set_pv_i(0x9c, 0);
        m.set_pv_f(0x10c, 0.5);
        m.set_pv_f(0xb8, 0.18);
        if m.pv_i(REACT_PATH) != -1 { m.set_pv_i(0xdc, m.pv_i(REACT_PATH)); }
        m.set_pv_i(MV + 0x48, AIM as i32);
        m.set_pv_f(MV + 0x34, 0.25);
        m.set_pv_f(MV, 0.75);
        m.set_pv_f(MV + 4, 0.75);
        m.set_pv_f(MV + 8, 0.6);
        m.set_pv_f(MV + 0xc, 0.6);
        if m.pv_i(FENCE) != -1 { m.set_pv_i(MV + 0x3c, m.pv_i(FENCE)); }
        m.set_pv_f(MV + 0x10, 1.483_530_045);
        m.set_pv_f(MV + 0x2c, 0.001_388_888_9);
        m.set_pv_f(MV + 0x30, 1.483_530_045);
        m.set_pv_f(MV + 0x34, 1.5);
        lim(w, id, 0.001_527_777_9, 0.045_833_334, 0.001_163_553, 0.104_719_773);
        let m = &mut w.mobys[id];
        m.set_pv_b(0x28, 3);
        m.set_pv_b(0x29, 1);
        m.set_pv_f(0x30, 0.75);
        m.set_pv_f(DAMAGE, m.pv_b(0x34) as f32);
        m.set_pv_b(0x2e, 1);
        if m.pv_i(RIDES) != 0 {
            st(w, id, 1, 5);
            let Some(h) = t::index(w, id, HOVER_I) else {
                let m = &mut w.mobys[id];
                m.set_pv_i(RIDES, 0);
                st(w, id, 4, 0);
                return true;
            };
            let (hp, hr) = (w.mobys[h].pos, w.mobys[h].rot);
            let m = &mut w.mobys[id];
            ai::set_link(m, HOVER_LINK, Some(h));
            m.pos = hp;
            m.rot = hr;
            m.has_collision = false;
            m.mode &= 0xafff;
            ai::set_link(&mut w.mobys[h], 0x27c, Some(id));
            let Some(s) = t::make_seat(w, id, ctx) else { return true };
            ai::set_link(&mut w.mobys[id], SEAT, Some(s));
            if !make_gun(w, id, ctx, 1) { return true; }
            w.mobys[id].set_pv_i(ATTACK, 0);
        } else {
            st(w, id, 4, 0);
            anim(w, id, ctx, 0, 0, 0);
            match w.mobys[id].pv_i(CHAT) {
                c @ 1..=3 => {
                    let p = t::index(w, id, PARTNER_I);
                    ai::set_link(&mut w.mobys[id], PARTNER, p);
                    st(w, id, 5, 0);
                    match c {
                        1 => anim(w, id, ctx, 13, 0, 0),
                        2 => anim(w, id, ctx, 15, 5, 0),
                        _ => anim(w, id, ctx, 15, 0, 0),
                    }
                }
                _ => {
                    if w.mobys[id].pv_i(PATROL) != -1 {
                        st(w, id, 11, 0);
                        anim(w, id, ctx, 4, 0, 0);
                    }
                }
            }
            if !t::arm(w, id, ctx) { return true; }
            if !make_gun(w, id, ctx, 2) { return true; }
        }
        let m = &mut w.mobys[id];
        let pos = m.pos;
        m.set_pv_v(WANDER, pos);
        m.set_pv_f(WANDER + 0x10, 1.0);
        m.set_pv_f(WANDER + 0x1c, 8.0);
        m.set_pv_f(WANDER + 0x20, 10.0);
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if w.mobys[id].pv_b(0x2e) == 2 {
        t::drop_part(w, id, ctx, WEAPON, false);
        t::drop_part(w, id, ctx, GUN, false);
        t::drop_part(w, id, ctx, SEAT, false);
        return true;
    }
    if let Some(g) = t::part(w, id, WEAPON) { super::super::enemies::place_part(w, id, g, 3, ctx); }
    if let Some(g) = t::part(w, id, GUN) { gun_on_joint(w, id, ctx, g); }
    if w.mobys[id].pv_f(HEALTH) <= 1.0 { drop(w, id, ctx, WEAPON); }
    w.mobys[id].set_pv_i(ATTACK, 0);
    let source = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0).and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).map(|s| s.o_class);
    let kind = ai::take_damage(w, id, ctx, false);
    if kind != 0 {
        if source.is_some_and(|c| !matches!(c, 0x0ed8 | 0x0047 | 0x0dd9 | SHOT | 0x0ce7 | 0x0d07 | 0)) { w.level11.g904c = 0; }
        if w.mobys[id].timer == 1 {
            if let Some(tu) = turret(w, id) { w.mobys[tu].state = 4; }
        }
        if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.speed != 1.0) { t::speed(w, id, ctx, 1.0); }
        // The attack flag was cleared above, so the original's sparks here (`0x42bde0`, 8 at joint 3) never come.
        if w.mobys[id].pv_i(ATTACK) != 0 {
            if let Some(at) = ctx.host.joint(&w.mobys[id], 3) { super::super::level04::rider::sparks(ctx, at, 8); }
        }
        if kind == 5 {
            let m = &mut w.mobys[id];
            m.set_pv_f(0xf4, 0.837_758_2);
            m.set_pv_i(0xf0, 0);
            drop(w, id, ctx, WEAPON);
        }
        let m = &mut w.mobys[id];
        m.set_pv_i(ATTACK, 0);
        m.set_pv_b(0x57, 0xd2);
        ai::flash_start(m, 0x50);
        if w.mobys[id].sub_state != 2 { t::alarm(w, id); }
        if matches!(kind, 4 | 5) { hand_charge(w, id, Some(CLASS)); }
    }
    let r = t::range_test(w, id, ctx);
    w.mobys[id].set_pv_s(RANGE, r);
    if w.mobys[id].sub_state != 4 {
        let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
        let m = &mut w.mobys[id];
        m.pos[2] -= 0.313_333_33;
        if m.pos[2] < g { m.pos[2] = g; }
    }
    if w.mobys[id].pos[2] < 61.0 && 6 <= ctx.game.frame {
        t::drop_part(w, id, ctx, GUN, false);
        t::bolts(w, id, ctx);
        hand_charge(w, id, None);
        drop(w, id, ctx, WEAPON);
        w.level11.g338 -= 1;
        return true;
    }
    false
}

/// `0x4276d8` (= `0x422438`): a hit on its seat (flags 0x10000) takes its damage off its health.
fn seat_hit(w: &mut MobyWorld, id: usize) -> bool {
    let Some(h) = t::part(w, id, SEAT).and_then(|s| w.hit_message(s)).filter(|h| h.tmpl.flags & 0x10000 != 0) else { return false };
    let m = &mut w.mobys[id];
    m.set_pv_f(HEALTH, m.pv_f(HEALTH) - h.tmpl.damage);
    true
}

/// The attack start of the calm and alert subs: the charge (sequence 10 from a random frame, blended 7..20, 0x13 sub
/// 2), the swing first (2, 0x13 sub 3) or the attack flag (2, 0x13 sub 3).
fn attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, swing_blend: u8, blend: u8, swing: bool, rand_frame: bool) {
    if w.mobys[id].pv_i(CHARGE) == 1 {
        if seq(w, id, ctx) != 10 {
            let f = if rand_frame { ctx.game.rng.randi(4) } else { 0 };
            let b = ctx.game.rng.rand_range(if rand_frame { 7 } else { 10 }, 20);
            ctx.host.set_anim(&w.mobys[id], 10, f as u8, b as u8);
        }
        st(w, id, 0x13, 2);
    } else if swing && w.mobys[id].pv_i(SWING_FIRST) != 0 {
        anim(w, id, ctx, 2, 0, swing_blend);
        st(w, id, 0x13, 3);
    } else {
        w.mobys[id].set_pv_i(ATTACK, 1);
        anim(w, id, ctx, 2, 0, blend);
        st(w, id, 0x13, 3);
    }
}

/// 0x0cf6 (module docs): the subs (`+0x95`), then [`states`] and the tail.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.variant != 0 { return; }
    if ctx.hero.state == 0x6f && w.mobys[id].sub_state != 4 { return; }
    let z0 = w.mobys[id].pos[2];
    if senses(w, id, ctx) {
        w.delete(id);
        return;
    }
    match w.mobys[id].sub_state {
        // Ratchet with item 0x41 within 6: its head turned to him (±90°); the item away, calm.
        9 => {
            if t::hdist(w, id, ctx) < 6.0 {
                let mut a = sub_rot(t::yaw_to(w, id, ctx.hero.pos), w.mobys[id].rot[2]);
                if w.mobys[id].once & 4 != 0 { a = -a; }
                w.mobys[id].set_pv_f(HEAD_TURN, a.clamp(-HALF_PI, HALF_PI));
            }
            if ctx.hero.items.item != 0x41 { w.mobys[id].sub_state = 0; }
        }
        // Calm: Ratchet near (2) — its run path (0x1a), its turret (0x19) or the attack; alarmed by its group — its
        // alarm path (6 at the first alarm, 7 after), its turret, and in range the attack.
        0 => {
            if t::first_sub(w, id) { calm(w, id); }
            if w.mobys[id].pv_s(RANGE) == 2 {
                if w.mobys[id].pv_i(APATH) != -1 {
                    anim(w, id, ctx, 3, 0, 10);
                    st(w, id, 0x1a, 2);
                }
                if w.mobys[id].pv_i(TURRET) != -1 {
                    anim(w, id, ctx, 3, 0, 10);
                    st(w, id, 0x19, 2);
                } else {
                    attack(w, id, ctx, 10, 0x28, true, true);
                }
            } else if w.mobys[id].pv_s(ALARMED) != 0 {
                if w.mobys[id].pv_i(ALARM_PATH) != -1 {
                    let n = w.mobys[id].pv_i(ALARMS);
                    if n == 1 {
                        t::anim_r(w, id, ctx, 9, 0, 5, 15);
                        st(w, id, 6, -1);
                    } else if 2 <= n {
                        t::anim_r(w, id, ctx, 3, 0, 5, 15);
                        st(w, id, 7, -1);
                        run_turn(w, id);
                    }
                }
                if w.mobys[id].pv_i(TURRET) != -1 {
                    anim(w, id, ctx, 3, 0, 10);
                    st(w, id, 0x19, 2);
                }
                if t::in_range(w, id) { attack(w, id, ctx, 0x28, 10, true, false); }
                w.mobys[id].set_pv_s(ALARMED, 0);
            }
        }
        // Alert: in range its run path from its nearest point, its turret, or the attack.
        1 => {
            if t::first_sub(w, id) {
                walk(w, id);
                w.mobys[id].set_pv_f(WANDER + 0x1c, 6.0);
            }
            if t::in_range(w, id) {
                let ap = w.mobys[id].pv_i(APATH);
                if ap != -1 {
                    anim(w, id, ctx, 3, 0, 10);
                    st(w, id, 0x1a, 2);
                    let pos = w.mobys[id].pos;
                    let mut best = (100.0, 0);
                    for (k, p) in t::path(ctx, ap).unwrap_or_default().iter().enumerate() {
                        let d = dist2(pos, *p);
                        if d < best.0 { best = (d, k as i32); }
                    }
                    t::path_start(w, id, ctx, ap, -1, best.1);
                }
                if w.mobys[id].pv_i(TURRET) != -1 {
                    anim(w, id, ctx, 3, 0, 10);
                    st(w, id, 0x19, 2);
                } else {
                    attack(w, id, ctx, 0x28, 0x28, false, true);
                }
            }
        }
        // Fighting: out of range now and then the stand (0xe sub 1, its timer 580..600); Ratchet in state 0x31, its
        // alarm path (7 sub 8).
        2 => {
            if t::first_sub(w, id) { lim(w, id, 0.003_888_889, 0.116_666_675, 0.003_781_547, 0.296_706_02); }
            if !t::in_range(w, id) && w.mobys[id].state != 0x19 && ctx.game.rng.randi(4) == 1 {
                t::speed(w, id, ctx, 1.0);
                anim(w, id, ctx, 2, 0, 10);
                st(w, id, 0xe, 1);
                w.mobys[id].set_pv_i(ALARMS, 0);
                let v = ctx.game.rng.rand_range(0x244, 0x258);
                w.mobys[id].set_pv_s(T260, v as i16);
            }
            if ctx.hero.state == 0x31 && w.mobys[id].pv_i(ALARM_PATH) != -1 {
                t::anim_r(w, id, ctx, 3, 0, 5, 15);
                st(w, id, 7, 8);
                run_turn(w, id);
            }
        }
        8 => {
            if ctx.hero.state != 0x31 {
                if w.mobys[id].pv_i(TURRET) != -1 {
                    anim(w, id, ctx, 3, 0, 10);
                    st(w, id, 0x19, 2);
                } else {
                    anim(w, id, ctx, 10, 0, 10);
                    st(w, id, 0x13, 2);
                }
            }
        }
        // Hurt (`0x33af20`, ai::flight): dead, its gun gone, its bolts and the fade; over, the charge (0x17) or the
        // close fire (0x1d) in range, its alarm path (7), else the stand.
        4 => {
            if 0.0 < w.mobys[id].pv_f(HEALTH) { face_hero(w, id, ctx); }
            let r = ai::flight(w, id, ctx);
            if r & 1 != 0 {
                t::drop_part(w, id, ctx, GUN, false);
                w.level11.g338 -= 1;
                t::bolts(w, id, ctx);
                super::super::death_fade::start(w, id, ctx, 0.0, None);
                return;
            }
            if r & 2 != 0 {
                w.mobys[id].set_pv_s(HEALTH + 6, 0);
                if t::in_range(w, id) {
                    anim(w, id, ctx, 3, 0, 10);
                    st(w, id, 0x17, 2);
                    if t::hdist(w, id, ctx) < 6.5 && t::faces_hero(w, id, ctx, 0.610_865_3) {
                        anim(w, id, ctx, 5, 1, 4);
                        run(w, id);
                        st(w, id, 0x1d, 2);
                    }
                } else if w.mobys[id].pv_i(ALARM_PATH) != -1 {
                    t::anim_r(w, id, ctx, 3, 0, 5, 15);
                    st(w, id, 7, 0);
                    run_turn(w, id);
                } else {
                    anim(w, id, ctx, 0, 0, 10);
                    st(w, id, 0xe, 1);
                    let v = ctx.game.rng.rand_range(0x244, 0x258);
                    w.mobys[id].set_pv_s(T260, v as i16);
                }
            }
        }
        3 => {
            if t::first_sub(w, id) { run(w, id); }
            if w.mobys[id].pv_i(CHARGE) == 1 {
                anim(w, id, ctx, 3, 0, 10);
                st(w, id, 0x17, 2);
                if w.mobys[id].pv_i(APATH) != -1 {
                    anim(w, id, ctx, 3, 0, 10);
                    st(w, id, 0x1a, 2);
                }
            }
            if !t::in_range(w, id) {
                anim(w, id, ctx, 0xb, 0, 0x1e);
                st(w, id, 0xe, 1);
                let v = ctx.game.rng.rand_range(0x244, 0x258);
                let m = &mut w.mobys[id];
                m.set_pv_s(T260, v as i16);
                m.set_pv_i(ALARMS, 0);
            }
        }
        5 => riding(w, id, ctx),
        // Off its hover: the weapon made on joint 3, the attack flag, upright.
        6 => {
            if t::first_sub(w, id) {
                if !t::arm(w, id, ctx) {
                    w.delete(id);
                    return;
                }
                let m = &mut w.mobys[id];
                m.set_pv_i(ATTACK, 1);
                m.rot[0] = 0.0;
                m.rot[1] = 0.0;
                m.rows = euler_rows(m.rot);
            }
        }
        7 => on_turret(w, id, ctx),
        _ => {}
    }
    states(w, id, ctx);
    if w.mobys[id].state >= 0x80 { return; }
    t::tail(w, id, ctx);
    let m = &mut w.mobys[id];
    if (m.pos[2] - z0).abs() < 0.005 { m.pos[2] = z0; }
}

/// Sub 5, on its hover (`+0x274`): the gun on joint 1, its place and angles the hover's, the seat 0.65 above it. Its
/// rider word `+0x26c` dropped (by the hover's knock-down) — or a hit on the seat — the gun drops (smoke), the seat
/// goes, the hover's rider link cleared, off it (2, or 3 after the hit with sequence 0; sub 6).
fn riding(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if let Some(g) = t::part(w, id, GUN) { super::super::enemies::place_part(w, id, g, 1, ctx); }
    let hover = ai::link(&w.mobys[id], HOVER_LINK).filter(|&h| h < w.mobys.len());
    if let Some(h) = hover {
        let (hp, hr) = (w.mobys[h].pos, w.mobys[h].rot);
        let m = &mut w.mobys[id];
        m.pos = hp;
        m.rot = hr;
        if let Some(s) = t::part(w, id, SEAT) {
            let sm = &mut w.mobys[s];
            sm.pos = add(hp, [0.0, 0.0, 0.65]);
            sm.rot = hr;
        }
        w.changed = true;
    }
    let off = w.mobys[id].pv_i(RIDES) == 0;
    if !off && !seat_hit(w, id) { return; }
    drop(w, id, ctx, GUN);
    t::drop_part(w, id, ctx, SEAT, false);
    if let Some(h) = hover { ai::set_link(&mut w.mobys[h], 0x27c, None); }
    if off {
        st(w, id, 2, 6);
    } else {
        anim(w, id, ctx, 0, 0, 0);
        st(w, id, 3, 6);
    }
}

/// Sub 7, on its turret (`+0x548`): facing Ratchet (not in 0x22), the turret's pitch toward his aim point (the
/// quaternion about y `0x301fa8` in the turret's `+0x10`); it sits 1.5 above the turret, the turret's rows from its
/// angles. Out of range (not 0x20): along the turret's path (0x22, sequence 0 on both); there with Ratchet near: the
/// firing (0x21, the turret's sequence 1).
fn on_turret(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_i(ATTACK, 0);
    let Some(tu) = turret(w, id) else { return };
    if w.mobys[id].state != 0x22 {
        face_hero(w, id, ctx);
        let (p, aim) = (w.mobys[id].pos, ctx.hero.aim_point);
        let a = atan2(dist2(p, aim), aim[2] - p[2]);
        let q = crate::quat::axis_angle(-a, [0.0, 1.0, 0.0]);
        let tm = &mut w.mobys[tu];
        if tm.pvars.len() < 0x20 { tm.pvars.resize(0x20, 0); }
        for (k, v) in q.iter().enumerate() { tm.set_pv_f(0x10 + 4 * k, *v); }
    }
    seat_on(w, id, tu);
    let s = w.mobys[id].state;
    if !t::in_range(w, id) && s != 0x20 {
        anim(w, id, ctx, 0, 0, 10);
        ctx.host.set_anim(&w.mobys[tu], 0, 0, 10);
        st(w, id, 0x22, -1);
    }
    if w.mobys[id].state == 0x22 && w.mobys[id].pv_s(RANGE) == 2 {
        anim(w, id, ctx, 0, 0, 10);
        st(w, id, 0x21, -1);
        ctx.host.set_anim(&w.mobys[tu], 1, 0, 10);
    }
}

/// It sits 1.5 above the turret; the turret's rows from its angles (`0x301a78`).
fn seat_on(w: &mut MobyWorld, id: usize, tu: usize) {
    let tp = w.mobys[tu].pos;
    let rot = w.mobys[id].rot;
    w.mobys[id].pos = [tp[0], tp[1], tp[2] + 1.5];
    w.mobys[tu].rows = euler_rows(rot);
    w.changed = true;
}

/// The gun's muzzle: its joint 0, 0.1 along its facing, 0.05 down.
fn muzzle(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<V3> {
    let g = t::part(w, id, GUN)?;
    let p = ctx.host.joint(&w.mobys[g], 0)?;
    let y = w.mobys[id].rot[2];
    Some([p[0] + y.cos() * 0.1, p[1] + y.sin() * 0.1, p[2] - 0.05])
}

/// The muzzle's twinkles while a firing sequence plays: their size by the sequence's time (÷ 5, 0.2..0.9 of it),
/// 2 ticks, `0xaf40df20` and a smaller `0x20ffffff` turning the other way; its flash ticks `+0x574` up.
fn muzzle_flash(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let a = ctx.host.frame_time(&w.mobys[id]);
    let Some(v) = muzzle(w, id, ctx) else { return };
    let rng = &mut ctx.game.rng;
    let f = rng.randf(0.2, 0.9) * (a / 5.0);
    let n = rng.randi(2);
    let n = if n == 0 { -1 } else { n } as i8;
    if let Some(sys) = ctx.parts.as_deref_mut() {
        crate::particles::twinkle(sys, rng, f * 0.3, f, 0.0, v, 2, 0xaf40_df20, 0, n, [0.0; 3], 0);
        crate::particles::twinkle(sys, rng, f * 0.04, f * 0.4, 0.0, v, 2, 0x20ff_ffff, 1, -n, [0.0; 3], 0);
    }
    let m = &mut w.mobys[id];
    m.set_pv_f(FLASH_T, m.pv_f(FLASH_T) + 1.0);
}

/// A shot: the muzzle (`+0x580`) and the way (`+0x5a0`: to Ratchet's aim point, or 25 ahead of it when he is more
/// than 45° off its facing), `+0x578` 0, the hooked bit cleared (`0x32a818(m, 0)`), the bolt 0x0d1b of its damage.
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(v) = muzzle(w, id, ctx) else { return };
    let to = if t::faces_hero(w, id, ctx, 0.785_398_24) {
        ctx.hero.aim_point
    } else {
        let (p, y) = (w.mobys[id].pos, w.mobys[id].rot[2]);
        [p[0] + y.cos() * 25.0, p[1] + y.sin() * 25.0, p[2]]
    };
    let way = sub(to, v);
    let m = &mut w.mobys[id];
    m.set_pv_v(MUZZLE, v);
    m.set_pv_v(WAY, way);
    m.set_pv_i(W578, 0);
    m.once &= !4;
    let dmg = m.pv_f(DAMAGE);
    shot(w, ctx, id, v, way, dmg);
}

/// `0x436238(owner, a, b, damage)`: the bolt 0x0d1b (update and draw distance 0xff, drawn, its owner's light): its
/// tail `+0x00` = `a`, its way `+0x20` = `b`, its damage, its steps and ticks 0; its place its pvars' first words (0).
fn shot(w: &mut MobyWorld, ctx: &Ctx, owner: usize, a: V3, b: V3, damage: f32) {
    if w.class(SHOT).is_none() { return; }
    let (light, amb) = (w.mobys[owner].light, w.mobys[owner].ambient);
    let s = w.create(SHOT);
    let m = &mut w.mobys[s];
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.state = 0;
    m.light = light;
    m.ambient = amb;
    m.pos = m.pv_v(0);
    m.set_pv_v(0, a);
    m.set_pv_v(0x20, b);
    m.set_pv_f(0x3c, damage);
    m.set_pv_f(0x38, 0.0);
    m.set_pv_f(0x34, 0.0);
    let _ = ctx;
}

/// The second switch, on the state (the original's order; the states are the decomp's decimal cases).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let state = w.mobys[id].state;
    match state {
        // Thrown (2 / 3): at its end mode | 0x5000, its collision back, the stand (0xe sub 1, its timer 480..600
        // when under 300, sequence 1); on the ground from frame 10; between frames 1.5 and 17 a step back and to its
        // left (3: on walkable ground, `0x32e668(0.5, 0.55, 0.5, 30°)`).
        2 | 3 => {
            if wrapped(w, id, ctx) {
                let solid = w.class(CLASS).is_some_and(|c| c.has_collision);
                let m = &mut w.mobys[id];
                m.mode |= 0x5000;
                m.has_collision = solid;
                st(w, id, 0xe, 1);
                if w.mobys[id].pv_s(T260) < 300 {
                    let v = ctx.game.rng.rand_range(0x1e0, 0x258);
                    w.mobys[id].set_pv_s(T260, v as i16);
                }
                anim(w, id, ctx, 1, 0, 10);
            }
            let f = ctx.host.frame_time(&w.mobys[id]);
            if 10.0 <= f {
                let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
                w.mobys[id].pos[2] = z;
            }
            if !(1.5..=17.0).contains(&f) { return; }
            let m = &w.mobys[id];
            let (back, left) = (add_rot(m.rot[2], std::f32::consts::PI), add_rot(m.rot[2], HALF_PI));
            let v0 = [back.cos() * 0.15, back.sin() * 0.15, 0.0];
            let v10 = [left.cos() * 0.05, left.sin() * 0.05, 0.0];
            let pos = m.pos;
            if state == 2 {
                w.mobys[id].pos = add(add(pos, v0), v10);
            } else {
                let mut goal = add(add(pos, v10), v0);
                let mut p = pos;
                ai::step_walk(ctx, id, 0.5, 0.55, 0.5, 0.523_598_85, &mut p, &mut goal, 0);
                w.mobys[id].pos = p;
            }
            w.changed = true;
        }
        5 => chat(w, id, ctx),
        6 => {
            w.mobys[id].set_pv_i(ATTACK, 1);
            if wrapped(w, id, ctx) {
                st(w, id, 7, -1);
                anim(w, id, ctx, 4, 0, 0x1e);
                w.mobys[id].set_pv_i(ATTACK, 0);
            }
        }
        7 => {
            if t::follow(w, id, ctx, ALARM_PATH) {
                st(w, id, 8, -1);
                anim(w, id, ctx, 9, 0, 0x14);
            }
        }
        // At its alarm path's end: two of its group alive or more, a chat with another (`0x32f018`).
        8 => {
            if wrapped(w, id, ctx) {
                calm(w, id);
                if t::group_pair(w, id) {
                    let live: Vec<usize> = t::members(w, id).into_iter().filter(|&i| t::alive(w, i)).collect();
                    let other = match live.first() { Some(&f) if f == id => live.get(1).copied(), f => f.copied() };
                    ai::set_link(&mut w.mobys[id], PARTNER, other);
                    st(w, id, 10, -1);
                    anim(w, id, ctx, 0xd, 0, 0x1e);
                    w.mobys[id].set_pv_i(CHAT, 1);
                    if other.is_some_and(|o| seq_a(w, o, ctx) == 0xd) {
                        anim(w, id, ctx, 0xf, 0, 0x1e);
                        w.mobys[id].set_pv_i(CHAT, 2);
                    }
                }
            }
        }
        10 => {
            let Some(o) = ai::link(&w.mobys[id], PARTNER).filter(|&o| o < w.mobys.len()) else { return };
            let p = w.mobys[o].pos;
            t::face(w, id, ctx, p);
            if rot_dist(w.mobys[id].rot[2], t::yaw_to(w, id, p)) < 0.087_266_475 { st(w, id, 5, -1); }
        }
        11 => {
            walk(w, id);
            if t::follow(w, id, ctx, PATROL) {
                st(w, id, 0xc, -1);
                anim(w, id, ctx, 1, 0, 0x14);
            }
        }
        12 => {
            if wrapped(w, id, ctx) {
                t::from_nearer_end(w, id, ctx, PATROL);
                st(w, id, 0xb, -1);
                anim(w, id, ctx, 4, 0, 0x14);
            }
        }
        14 => {
            if wrapped(w, id, ctx) {
                anim(w, id, ctx, 4, 0, 0x14);
                st(w, id, 0xf, -1);
                if w.mobys[id].pv_i(HOME_PATH) != -1 { st(w, id, 0x11, -1); }
            }
        }
        15 => {
            let home = w.mobys[id].pv_v(WANDER);
            t::go(w, id, ctx, home);
            if dist2(w.mobys[id].pos, home) < 2.0 {
                st(w, id, 0x10, -1);
                anim(w, id, ctx, 9, 0, 0x14);
            }
        }
        17 => {
            walk(w, id);
            if t::follow(w, id, ctx, HOME_PATH) {
                st(w, id, 0x12, -1);
                anim(w, id, ctx, 9, 0, 0x14);
            }
        }
        18 => {
            if wrapped(w, id, ctx) {
                t::from_nearer_end(w, id, ctx, HOME_PATH);
                st(w, id, 0x11, -1);
                anim(w, id, ctx, 4, 0, 0x14);
            }
        }
        // The charge: its cuboid to run to (0x18); close and facing, the fire (0x1c); close, facing him; else toward
        // him — at the network walk's end the charge handed to the first of its group without it, if nearer him and
        // alive (then this one's swing, 0x13 sub 3).
        23 => {
            if w.mobys[id].pv_i(RUN_TO) != -1 {
                st(w, id, 0x18, -1);
            } else if close_facing(w, id, ctx) {
                to_shot(w, id, ctx);
            } else if t::hdist(w, id, ctx) < 3.5 {
                face_hero(w, id, ctx);
            } else if w.mobys[id].pv_i(NET) != 0 {
                let (y, off, goal) = (w.mobys[id].rot[2], w.mobys[id].pv_f(OFFSET), ctx.hero.pos);
                if t::net_walk(w, id, ctx, y, 7.0, 0.25, off, goal) && t::group_pair(w, id) {
                    for i in t::members(w, id) {
                        if w.mobys[i].pv_i(CHARGE) != 0 { continue; }
                        if t::hdist(w, i, ctx) < t::hdist(w, id, ctx) && 0.0 < w.mobys[i].pv_f(HEALTH) {
                            w.mobys[i].set_pv_i(CHARGE, 1);
                            w.mobys[id].set_pv_i(CHARGE, 0);
                            anim(w, id, ctx, 2, 0, 10);
                            st(w, id, 0x13, 3);
                        }
                        break;
                    }
                }
            } else {
                let (off, h) = (w.mobys[id].pv_f(OFFSET), ctx.hero.pos);
                ai::move_toward(w, id, ctx, off, h);
            }
        }
        // To its cuboid (`+0x57c`, its place `+0x30`): there, the crouch (0x1b, sequence 0x12); Ratchet close, the
        // close fire (0x1d).
        24 => {
            w.mobys[id].set_pv_i(ATTACK, 0);
            lim(w, id, 0.003_888_889, 0.116_666_675, 0.003_393_696_2, 0.279_252_74);
            if let Some((v0, _)) = ctx.level.cuboid_place(w.mobys[id].pv_i(RUN_TO)) {
                t::go(w, id, ctx, v0);
                if dist2(w.mobys[id].pos, v0) < 1.5 {
                    anim(w, id, ctx, 0x12, 0, 0x10);
                    run_close(w, id);
                    st(w, id, 0x1b, 2);
                }
            }
            if t::hdist(w, id, ctx) < 3.5 { to_close(w, id, ctx); }
        }
        // To its turret (as the thug's 0x1d): the turret (unless taken) turned with it on a spring and raised toward
        // its rest less 1.2; within 1.5 across: onto it (the turret's 6), 1.5 above it, the climb 0x20 (sub 7).
        25 => {
            let Some(tu) = turret(w, id) else { return };
            if w.mobys[tu].state != 5 {
                let mut r = w.mobys[tu].rot;
                let target = w.mobys[id].rot;
                let mut v = w.mobys[id].pv_f(SPRING_V);
                ai::spring_to(&mut r, target, 0.001_163_553, 0.002_327_105_9, 0.104_719_77, &mut v);
                w.mobys[id].set_pv_f(SPRING_V, v);
                let tm = &mut w.mobys[tu];
                tm.rot = r;
                tm.rows = euler_rows(r);
                let rest = tm.pv_f(0x5c);
                tm.pos[2] += ((rest - 1.2) - tm.pos[2]) * 0.11;
            }
            let tp = w.mobys[tu].pos;
            t::go(w, id, ctx, tp);
            if dist2(w.mobys[id].pos, tp) < 1.5 && w.mobys[tu].state != 5 {
                w.mobys[tu].state = 6;
                let m = &mut w.mobys[id];
                m.pos = [tp[0], tp[1], tp[2] + 1.5];
                m.set_pv_f(CLIMB, tp[2] + 0.75);
                anim(w, id, ctx, 0, 0, 0x19);
                st(w, id, 0x20, 7);
                w.mobys[id].timer = 1;
            }
        }
        // The climb: the turret's height on a spring toward `+0x554`, it 1.5 above; there the firing (0x21; the
        // turret's sequence 1); facing Ratchet within 15° the turret's sequence 1; else in it the turret's shots.
        32 => {
            let Some(tu) = turret(w, id) else { return };
            let target = w.mobys[id].pv_f(CLIMB);
            let (mut z, mut v) = (w.mobys[tu].pos[2], w.mobys[id].pv_f(SPRING_V));
            crate::hero::physics::spring(target, 0.000_277_777_78, 0.001_666_666_7, 0.033_333_335, &mut z, &mut v);
            w.mobys[tu].pos[2] = z;
            w.mobys[id].set_pv_f(SPRING_V, v);
            w.mobys[id].pos[2] = z + 1.5;
            w.changed = true;
            if (z - target).abs() < 0.1 {
                anim(w, id, ctx, 0, 0, 5);
                st(w, id, 0x21, -1);
                anim(w, tu, ctx, 1, 0, 5);
            }
            if t::faces_hero(w, id, ctx, 0.261_799_42) && seq(w, tu, ctx) != 1 {
                ctx.host.set_anim(&w.mobys[tu], 1, 0, 5);
                anim(w, id, ctx, 0, 0, 5);
            } else if seq_a(w, tu, ctx) == 1 {
                turret_fire(w, tu, ctx);
            }
        }
        33 => {
            let Some(tu) = turret(w, id) else { return };
            turret_fire(w, tu, ctx);
        }
        // Along the turret's path (`+0x564`, its point `+0x568`): facing the point, on the turret; the point's heading
        // (not the turn to it: the original's) within 35°, the next of two.
        34 => {
            let tp = w.mobys[id].pv_i(T_PATH);
            if tp == -1 { return; }
            let Some(tu) = turret(w, id) else { return };
            let pts = t::path(ctx, tp).unwrap_or_default();
            let k = w.mobys[id].pv_i(T_POINT).max(0) as usize;
            let Some(&node) = pts.get(k) else { return };
            t::face(w, id, ctx, node);
            seat_on(w, id, tu);
            let p = w.mobys[id].pos;
            if atan2(node[0] - p[0], node[1] - p[1]) <= 0.610_865_3 {
                let m = &mut w.mobys[id];
                let n = m.pv_i(T_POINT) + 1;
                m.set_pv_i(T_POINT, if 2 <= n { 0 } else { n });
            }
        }
        // The wind-up: facing Ratchet; without the charge the strafe (0x14, sequence 0x11, the attack flag); with it
        // the charge (0x17 sub 2) or its run path (0x1a).
        19 => {
            w.mobys[id].set_pv_i(ATTACK, 0);
            face_hero(w, id, ctx);
            if w.mobys[id].pv_i(CHARGE) == 0 {
                w.mobys[id].set_pv_i(ATTACK, 1);
                anim(w, id, ctx, 0x11, 0, 10);
                st(w, id, 0x14, -1);
            } else {
                anim(w, id, ctx, 3, 0, 10);
                st(w, id, 0x17, 2);
                if w.mobys[id].pv_i(APATH) != -1 {
                    anim(w, id, ctx, 3, 0, 10);
                    st(w, id, 0x1a, 2);
                }
            }
        }
        // The strafe: turning to Ratchet; within 3.5 the fire (0x1c); a third of the time at its sequence's end the
        // wind-up again (sequence 10, a third of those 9; 0x13 sub 3).
        20 => {
            t::turn_hero(w, id, ctx);
            if t::hdist(w, id, ctx) < 3.5 {
                anim(w, id, ctx, 5, 0, 10);
                st(w, id, 0x1c, -1);
            }
            let n = ctx.game.rng.randi(3);
            if n == 1 && wrapped(w, id, ctx) {
                anim(w, id, ctx, 10, 0, 10);
                if ctx.game.rng.randi(3) == n { anim(w, id, ctx, 9, 0, 10); }
                st(w, id, 0x13, 3);
            }
        }
        21 => {
            if w.mobys[id].pv_i(NET) != 0 {
                let (y, off, goal) = (w.mobys[id].rot[2], w.mobys[id].pv_f(OFFSET), ctx.hero.pos);
                if t::net_walk(w, id, ctx, y, 7.0, 0.25, off, goal) {
                    anim(w, id, ctx, 0x11, 0, 10);
                    st(w, id, 0x14, -1);
                }
            } else {
                let (off, h) = (w.mobys[id].pv_f(OFFSET), ctx.hero.pos);
                ai::move_toward(w, id, ctx, off, h);
            }
            if t::hdist(w, id, ctx) < 8.0 {
                anim(w, id, ctx, 0x11, 0, 10);
                st(w, id, 0x14, -1);
            }
        }
        22 => {
            let d = t::hdist(w, id, ctx);
            if close_facing(w, id, ctx) {
                to_shot(w, id, ctx);
            } else if d < 3.5 {
                face_hero(w, id, ctx);
            } else if 6.0 < d {
                run(w, id);
                anim(w, id, ctx, 3, 0, 10);
                st(w, id, 0x17, -1);
            } else {
                lim(w, id, 0.000_555_555_57, 0.016_666_668, 0.001_163_553, 0.104_719_77);
                let (off, h) = (w.mobys[id].pv_f(OFFSET), ctx.hero.pos);
                ai::move_toward(w, id, ctx, off, h);
            }
        }
        // The run path: within 3.5 of Ratchet the fire (0x1c, its path gone, the walk accel 1/1200); else along it
        // (accel 1/240), at its end the charge, or (the attack flag) a third of the time the wind-up, else 0x15.
        26 => {
            w.mobys[id].set_pv_f(MV + 0x34, 1.5);
            if t::hdist(w, id, ctx) < 3.5 {
                anim(w, id, ctx, 5, 0, 2);
                st(w, id, 0x1c, -1);
                let m = &mut w.mobys[id];
                m.set_pv_i(APATH, -1);
                m.set_pv_f(MV + 0x24, 0.000_833_333_35);
            } else {
                w.mobys[id].set_pv_f(MV + 0x24, 0.004_166_667);
                if t::follow(w, id, ctx, APATH) {
                    if w.mobys[id].pv_i(CHARGE) == 1 {
                        anim(w, id, ctx, 3, 0, 10);
                        st(w, id, 0x17, 2);
                    } else {
                        w.mobys[id].set_pv_i(ATTACK, 1);
                        if ctx.game.rng.randi(3) == 1 {
                            anim(w, id, ctx, 2, 0, 10);
                            st(w, id, 0x13, 3);
                        } else {
                            anim(w, id, ctx, 3, 0, 10);
                            st(w, id, 0x15, 3);
                        }
                    }
                    w.mobys[id].set_pv_i(APATH, -1);
                }
            }
        }
        // Crouched at its cuboid: at its sequence's end the fire (0x1c); facing Ratchet; in sequence 0x12 the
        // muzzle's twinkles; Ratchet close, the close fire (0x1d).
        27 => {
            if wrapped(w, id, ctx) {
                anim(w, id, ctx, 0x13, 0, 6);
                run(w, id);
                st(w, id, 0x1c, 2);
            }
            face_hero(w, id, ctx);
            if seq_a(w, id, ctx) == 0x12 { muzzle_flash(w, id, ctx); }
            if t::hdist(w, id, ctx) < 3.5 { to_close(w, id, ctx); }
        }
        // The fire: facing Ratchet, at frames 1 / 7 / 14 a shot ([`fire`]); at its end back (0x1e, sequence 0x14 or
        // with `+0x5b4` 10; off sub 2 the strafe 0x14, sequence 0x11).
        28 => {
            face_hero(w, id, ctx);
            w.mobys[id].set_pv_i(ATTACK, 0);
            let now = ai::reached(w, id, ctx, 1.0) || ai::reached(w, id, ctx, 7.0) || ai::reached(w, id, ctx, 14.0);
            if now { fire(w, id, ctx); }
            if wrapped(w, id, ctx) {
                anim(w, id, ctx, 0x14, 0, 0x14);
                if w.mobys[id].pv_i(ALT_SEQ) != 0 { anim(w, id, ctx, 10, 0, 0x14); }
                fire_over(w, id, ctx);
            }
        }
        // The close fire: facing Ratchet; in sequence 5 the muzzle's twinkles before frame 11, a shot at 11, and between
        // frames 10 and 15 a 0.4 sphere of its damage at the gun's joint 0 (flags 1); at its end back as 28.
        29 => {
            face_hero(w, id, ctx);
            w.mobys[id].set_pv_i(ATTACK, 0);
            if seq_a(w, id, ctx) == 5 && ctx.host.frame_time(&w.mobys[id]) < 11.0 { muzzle_flash(w, id, ctx); }
            if ai::reached(w, id, ctx, 11.0) { fire(w, id, ctx); }
            let g = ctx.host.frame_time(&w.mobys[id]);
            if seq_a(w, id, ctx) == 5 && (10.0..=15.0).contains(&g) {
                let at = t::part(w, id, GUN).and_then(|gm| ctx.host.joint(&w.mobys[gm], 0));
                if let Some(v0) = at {
                    let dmg = w.mobys[id].pv_f(DAMAGE);
                    ai::attack_sphere(w, id, ctx, 0.4, dmg, 1.0, v0, 1, 0, 1, 0);
                }
            }
            if wrapped(w, id, ctx) {
                anim(w, id, ctx, 0x11, 0, 0x14);
                ctx.game.rng.randi(3);
                fire_over(w, id, ctx);
            }
        }
        // Back from the fire: the charge (0x17); its cuboid (`+0x57c`) within 1.5, the crouch (0x1b); Ratchet close,
        // the close fire.
        30 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].set_pv_i(W5B0, 0);
                anim(w, id, ctx, 3, 0, 10);
                st(w, id, 0x17, -1);
                if let Some((v0, _)) = ctx.level.cuboid_place(w.mobys[id].pv_i(RUN_TO)) {
                    t::go(w, id, ctx, v0);
                    if dist2(w.mobys[id].pos, v0) < 1.5 {
                        let b = if w.mobys[id].pv_i(ALT_SEQ) != 0 { 0x14 } else { 6 };
                        anim(w, id, ctx, 0x12, 0, b);
                        run_close(w, id);
                        st(w, id, 0x1b, 2);
                    }
                }
                if t::hdist(w, id, ctx) < 3.5 { to_close(w, id, ctx); }
            }
        }
        _ => {}
    }
}

/// The end of a fire: 0x1e; off sub 2 the strafe (0x14, sequence 0x11); the muzzle's flash ticks 0.
fn fire_over(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    st(w, id, 0x1e, -1);
    if w.mobys[id].sub_state != 2 {
        st(w, id, 0x14, -1);
        anim(w, id, ctx, 0x11, 0, 10);
    }
    w.mobys[id].set_pv_f(FLASH_T, 0.0);
}

/// The turret's gun at frames 6 / 12 / 0 fires from its joint 1, at 3 / 9 / 15 from joint 0: the bolt 0x0d1b
/// (`0x436238(turret, way, joint, 1)`) — the original passes the way 0.5 long as the tail and the joint as the way,
/// so its bolt runs from near the world's origin; kept as it is.
fn turret_fire(w: &mut MobyWorld, tu: usize, ctx: &mut Ctx) {
    let at = |w: &MobyWorld, ctx: &Ctx, ts: [f32; 3]| ts.iter().any(|&f| ai::reached(w, tu, ctx, f));
    let j = if at(&*w, &*ctx, [6.0, 12.0, 0.0]) { 1 } else if at(&*w, &*ctx, [3.0, 9.0, 15.0]) { 0 } else { return };
    let Some(v0) = ctx.host.joint(&w.mobys[tu], j) else { return };
    let v10 = set_len(sub(ctx.hero.aim_point, v0), 0.5);
    shot(w, ctx, tu, v10, v0, 1.0);
}

/// The chat (state 5): role 1 in its sequence 0xd now and then 0xc (a third of those 0xb); 0xb / 0xc over: 0xe, and
/// with a 0x0cf6 partner the roles passed (it 1 → 2); 0xe over: 0xf (role 1: 0xd); role 1 in 0xf: 0xe; role 3 with
/// its partner in 0xe: 0xe.
fn chat(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(o) = ai::link(&w.mobys[id], PARTNER).filter(|&o| o < w.mobys.len()) else { return };
    let r = w.mobys[id].pv_i(CHAT);
    if r == 1 && seq_a(w, id, ctx) == 0xd {
        let n = ctx.game.rng.randi(200);
        if n == r {
            anim(w, id, ctx, 0xc, 0, 0x1e);
            if ctx.game.rng.randi(3) == n { anim(w, id, ctx, 0xb, 0, 0x1e); }
        }
    }
    if (0xb..=0xc).contains(&seq_a(w, id, ctx)) && wrapped(w, id, ctx) {
        anim(w, id, ctx, 0xe, 0, 0x1e);
        if w.mobys[o].o_class == CLASS {
            w.mobys[o].set_pv_i(CHAT, 1);
            w.mobys[id].set_pv_i(CHAT, 2);
        }
    }
    if seq_a(w, id, ctx) == 0xe && wrapped(w, id, ctx) {
        anim(w, id, ctx, 0xf, 0, 0x1e);
        if w.mobys[id].pv_i(CHAT) == 1 { anim(w, id, ctx, 0xd, 0, 0x1e); }
    }
    if w.mobys[id].pv_i(CHAT) == 1 && seq_a(w, id, ctx) == 0xf && seq(w, id, ctx) != 0xe { ctx.host.set_anim(&w.mobys[id], 0xe, 0, 0x14); }
    if w.mobys[id].pv_i(CHAT) == 3 {
        let c = seq_a(w, o, ctx);
        if c == 0xe && seq(w, id, ctx) != c { ctx.host.set_anim(&w.mobys[id], 0xe, 0, 0x23); }
    }
}

const S_TAIL: usize = 0x00;
const S_HEAD: usize = 0x10;
const S_WAY: usize = 0x20;
const S_STEPS: usize = 0x34;
const S_TICKS: usize = 0x38;
const S_DAMAGE: usize = 0x3c;
const S_PVARS: usize = 0x40;

/// `0x436788(at, n)`: the rider's sparks (crate::moby::classes::level04::rider::sparks_in) in `0x8020ff20`.
fn green_sparks(ctx: &mut Ctx, at: V3, n: i32) { super::super::level04::rider::sparks_in(ctx, at, n, 0x8020_ff20); }

/// The line from `a` to `b` (`0x2f0678(a, b, 0, m, &hit)`, flags 0, the moby itself ignored): what it meets gets the
/// hit `t`; the point it met.
pub(super) fn line_hit(w: &mut MobyWorld, ctx: &Ctx, id: usize, a: V3, b: V3, t: &crate::moby::hits::HitTemplate) -> Option<V3> {
    let o = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, a, b, crate::collision::QueryFlags(0), Some(id)))?;
    if let Some(k) = o.moby { w.deliver_hit_any(k, t, [o.point[0], o.point[1], o.point[2], 1.0]); }
    Some(o.point)
}

/// One step of the bolt's head (`steps` + 1 along its way) and its line; true when it met something (then gone).
fn bolt_step(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, sparks: i32) -> bool {
    let m = &mut w.mobys[id];
    let steps = m.pv_f(S_STEPS) + 1.0;
    m.set_pv_f(S_STEPS, steps);
    let (tail, way, dmg) = (m.pv_v(S_TAIL), m.pv_v(S_WAY), m.pv_f(S_DAMAGE));
    let v = set_len(way, 0.066_666_67);
    let mut t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 0x1_0001, [v[0], v[1], v[2], 0.0]);
    t.kind = (SHOT as u16 as u32) << 16;
    let head = add(tail, set_len(way, steps));
    w.mobys[id].set_pv_v(S_HEAD, head);
    let Some(p) = line_hit(w, ctx, id, tail, head, &t) else { return false };
    w.mobys[id].set_pv_v(S_HEAD, p);
    if w.mobys[id].once & 4 == 0 {
        green_sparks(ctx, p, sparks);
        w.mobys[id].once |= 4;
    }
    w.delete(id);
    true
}

/// 0x0d1b (`0x435dc0`, module docs).
pub fn update_0d1b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.pvars.len() < S_PVARS { m.pvars.resize(S_PVARS, 0); }
    let ticks = m.pv_f(S_TICKS) + 1.0;
    m.set_pv_f(S_TICKS, ticks);
    if ticks < 25.0 {
        if bolt_step(w, id, ctx, 6) { return; }
        let m = &w.mobys[id];
        let (head, tail) = (m.pv_v(S_HEAD), m.pv_v(S_TAIL));
        let rng = &mut ctx.game.rng;
        let size = rng.randf(1.0, 4.0);
        let n = rng.rand_range(25, 35);
        let n = if n == 0 { -1 } else { n } as i8;
        if let Some(sys) = ctx.parts.as_deref_mut() {
            let p = [head[0], head[1], head[2] - 0.1];
            crate::particles::twinkle(sys, rng, size * 0.3, size, 0.0, p, 15, 0xaf20_ff20, 0, n, [0.0; 3], 0);
            crate::particles::twinkle(sys, rng, size * 0.3, size, 0.0, tail, 10, 0x4000_af20, 0, -n, [0.0; 3], 0);
            crate::particles::twinkle(sys, rng, size * 0.04, size * 0.4, 0.0, tail, 10, 0x20ff_ffff, 1, n, [0.0; 3], 0);
        }
    } else if ticks < 34.0 {
        let m = &w.mobys[id];
        if dist2(m.pv_v(S_TAIL), m.pv_v(S_HEAD)) <= 1.0 { return; }
        if bolt_step(w, id, ctx, 3) { return; }
        let m = &mut w.mobys[id];
        let tail = add(m.pv_v(S_TAIL), set_len(m.pv_v(S_WAY), 1.666_666_7));
        m.set_pv_v(S_TAIL, tail);
        let head = m.pv_v(S_HEAD);
        let rng = &mut ctx.game.rng;
        let size = rng.randf(1.0, 4.0);
        let n = rng.rand_range(25, 35);
        let n = if n == 0 { -1 } else { n } as i8;
        if let Some(sys) = ctx.parts.as_deref_mut() {
            let p = [head[0], head[1], head[2] - 0.1];
            crate::particles::twinkle(sys, rng, size * 0.3, size, 0.0, p, 15, 0xaf20_ff20, 0, n, [0.0; 3], 0);
            crate::particles::twinkle(sys, rng, size * 0.4, size * 0.2, 0.0, p, 20, 0x8040_df20, 1, -n, [0.0; 3], 0);
        }
    } else {
        w.delete(id);
    }
}
