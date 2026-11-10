//! Snivelak's gunship **0x0ebb** (L16 `0x404d28`, 7 placed; the decomp's v2.00 `func_US2_L16_00404D28`, its head
//! v1.01 `func_L16_00404A60` at our `0x405e88`) and its shot **0x0ebd** (`0x4065d0`, made by `0x4068b0`): a hovering
//! craft with a spinning rotor (joint list 7) and a second model riding it (0x0ecb), that waits until Ratchet is in
//! its area, flies in along a path, then holds or patrols between two paths, turning and pitching to him and
//! firing from its joints 2 and 3. Two of them (level 0x10, uids 0x27 and 0x114) only rise away into the sky and
//! hurt Ratchet when he stands on their deck. On the framework (crate::moby::ai): health `+0x20` (most `+0x24`, 5 by
//! default; `+0x26` a wait before hits count, its status byte `+0x2e`: 2 deletes it; its shot's damage byte `+0x34`),
//! movement `+0x50` (radius 0, height 1, steps 0.4 / 0.4, slope 90°, the turn left 89°, stop 0.5, options | 0x17),
//! reaction `+0x140` (its colour flash at its start: a hurt to state 8 sub 3, radius and centre 0.5, bounce 0.15).
//!
//! **Its pvars:** `+0x1e8` a damage scale for hits from its own kind (0 none), `+0x210` the rotor's node (`+0x211`
//! linked, its rotation `+0x220`), `+0x250` / `+0x254` the first search's area and range, `+0x258` a timer counted
//! down (480..600 after an alarm, 580..600 when it calms; read by nothing here), `+0x25c` / `+0x2b8` the sequence
//! timers, `+0x25e` the alarm's wait (0..30), `+0x260` / `+0x2b4` the two searches' results (the second's area
//! `+0x2b0`, no range), `+0x264` alarmed, `+0x26c` the rotor's angle (23.8° a tick), `+0x270` the pitch speed,
//! `+0x274` the rider 0x0ecb (made on its first tick), `+0x278` / `+0x284` the bob's last offset and phase, `+0x288`
//! straight to the rise, `+0x28c` / `+0x290` the rise's height and speed, `+0x294` the path in (−1 none; `+0x2a4` the
//! point it is at), `+0x298` / `+0x29c` the two patrol paths (−1 none), `+0x2a0` its speed (a second), `+0x2a8` keep
//! its sequence on the path in, `+0x2ac` a riser (the uids above).
//!
//! **Each tick** ([`head`], `0x405e88`): in state 0 the set-up — a riser's height 300 up, the reaction, health and
//! movement blocks, the senses (`0x325d70` = L00 `0x32dcf0`: 2·speed/3600, speed/60, 0.1°, 12°), state 1 (sub 0),
//! sequence 0. Its shadow within 24 (range 0x16, `0x3172a0` = L00 `0x31da58`); the bob (0.75 at 100°/s, `0x31d910` =
//! L00 `0x324430`); the flash (`0x319b00` = L00 `0x3201e8`); a hit of flag 0x10000 (`0x317530` = L00 `0x31dce8`) once
//! `+0x26`'s wait is out, scaled by `+0x1e8` when its own kind threw it, does its damage (`0x317590` = L00
//! `0x31dd48`, crate::moby::ai::damage_of); a third of its most at once, or none left: sequence 4 and the hurt (9, sub
//! 3). Status 2: the rider and its bolts (`0x314538` = L00 `0x31adc0`) go, and it. The two searches (`0x3231c8` = L00
//! `0x32ad38`, height 0).
//!
//! **Sub-states** (`+0x95`): **0** calm (its wait drawn on the first tick): found, sequence 3 and the aim (5, sub 2);
//! a riser or `+0x288` the rise (4) at sequence 0; a path in, the path (3; sequence 0 unless `+0x2a8`, a riser
//! always). Alarmed, after the wait: on guard (2, sub 1). **1** on guard: found by either, the aim. **2** attacking:
//! found by neither and not rising or on the path in, calm (1, sub 0) at sequence 0. **3** hurt (its sequence 4
//! plays out): no health — its bolts, five of its own pieces as balls (`0x2d5088` = L00 `0x2dbdb0` mode 1,
//! crate::moby::shards::spawn_ball: 0.13..0.2 a random way, up at 2.5×, spin 30, gravity 15), its class sound 0, two
//! green blasts (sizes 2 and 1: 15 / 10 / 10 sprites, 10 puffs, 15 streaks, the flash ball; crate::moby::blast::
//! burst_green), the camera shaken (0.2 for 60 ticks, the roll 0.1 for 90), the rider gone and a wreck of it thrown
//! (mode 3, crate::moby::shards::spawn_whole_ball: ±0.5..4 / ±0..4 across, 14..18 up a second, spin 180, gravity 25,
//! Ratchet's light, `0x319798` = L00 `0x31fe80`), gone. At its sequence's end: found, the aim (or the patrol), else
//! calm; a riser the rise.
//!
//! **States** (`+0x20`): **3** the path in (`0x406268`: to each point by a fifth of the way a tick, `0x324e78` = L07
//! `0x322f38`, crate::moby::ai::move_by; within 0.5 the next — across, a riser in 3D), facing (`0x325d08` = L01
//! `0x323560`, crate::moby::ai::face) and pitched to Ratchet (`0x318a78` = L00 `0x31f230`: 0.2°, 0.2°, 12°; not a
//! riser); at its end the patrol (6) or the aim (5) at sequence 3, a riser the rise (260 up). **4** the rise (z eased
//! to its height, `0x318ee8` = L00 `0x31ed78`: 0.0125, 1/360, 1/15); within 0.3 (not a riser) the aim or patrol.
//! **5** the aim: facing and pitched to Ratchet. **6** / **7** the patrol along `+0x298` / `+0x29c`
//! (`0x325dd8` = L09 `0x32a280`, crate::moby::ai::follow_path), at each end the other. In 3, 5, 6 and 7 at frame 1.5
//! of its sequence with Ratchet within 90° of its facing, a shot from each of its joints 2 and 3 at his aim point
//! (50 a second, its damage byte; in 5..7 only with nothing on the line, `0x2d7b50` = L00 `0x2de878`, flags 0, or a
//! moby that is Ratchet or not of its kind). In 6 / 7 its sequences: 3 for 240 ticks, 0 for 60.
//!
//! **The tail** (`0x406378`): the timer, its ticks, the rider on it (`0x406500`: made once, `0x322bd8` = L00
//! `0x32a748`, at its draw distance, sequence 1; then at its place and angles, shown and hidden with it), the rotor
//! turned (`0x30bc48` = L00 `0x312480`, crate::moby::Moby::manips); a riser's deck (3.5 up, 1.3 back) hurts Ratchet
//! standing on it (within 0.1 high and 9.1 across: 10, flags 1, `0x316bc0` = L00 `0x31d378`).
//!
//! **The shot 0x0ebd** ([`fire`], [`update_0ebd`]): life 240 ticks (then a 0.6 sphere of its damage, flags 0x10003);
//! each tick the line to its next place (flags 0, its hit on what it meets: flags 3) or a 0.3 sphere (flags 0); the
//! world, or a moby not its owner: a 0.6 sphere (its damage, flags 0x10003, along its way ×3 / ×1.5) and it ends —
//! the next tick a small green blast (0.25: 25 / 20 / 20 sprites, 20 puffs, the flash ball), its class sound 0, gone.
//! Its rows roll 40° a tick about the side row (`0x31b208` = L00 `0x321c00`). At its firing two twinkles at it
//! (`0x3324d0` = L00 `0x33a6a0`, crate::particles::twinkle: 0.3·s → s, 0x80103080; 0.04·s → 0.4·s, white; s 8..10).
//!
//! Its draw ([`draw_0ebd`], `0x406ad0`, registered at the end of each tick it lives through): its bolt
//! (crate::fx::trails::bolt, [`BOLT`]) twice, the shot's own model being at a scale of 0.0001.
//!
//! Not ported: the sound the rotor makes is the class's.

use crate::hero::physics::{add, add_rot, atan2, dist, dist2, ease, euler_rows, len2, rot_dist, scale, set_len, sub, turn_ease, V3};
use crate::moby::classes::props::{bob, seq, set_seq, wrapped};
use crate::moby::{ai, blast, bolts, mode, shards, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0ebb;
pub const SHOT: i16 = 0x0ebd;
/// The rider (its own model; no update of its own).
pub const RIDER: i16 = 0x0ecb;

const HEALTH: usize = 0x20;
const MOST: usize = 0x24;
const WAIT: usize = 0x26;
const STATUS: usize = 0x2e;
const DAMAGE: usize = 0x34;
const MV: usize = 0x50;
const REACT: usize = 0x140;
const KIND_SCALE: usize = 0x1e8;
const NODE_LINKED: usize = 0x211;
const AREA1: usize = 0x250;
const RANGE1: usize = 0x254;
const TIMER: usize = 0x258;
const T_SEQ0: usize = 0x25c;
const T_ALARM: usize = 0x25e;
const FOUND1: usize = 0x260;
const ALARM: usize = 0x264;
const ROTOR: usize = 0x26c;
const PITCH_V: usize = 0x270;
const RIDER_LINK: usize = 0x274;
const BOB_LAST: usize = 0x278;
const BOB_PHASE: usize = 0x284;
const RISE_NOW: usize = 0x288;
const RISE_TO: usize = 0x28c;
const RISE_V: usize = 0x290;
const PATH_IN: usize = 0x294;
const PATROL_A: usize = 0x298;
const PATROL_B: usize = 0x29c;
const SPEED: usize = 0x2a0;
const PATH_AT: usize = 0x2a4;
const KEEP_SEQ: usize = 0x2a8;
const RISER: usize = 0x2ac;
const AREA2: usize = 0x2b0;
const FOUND2: usize = 0x2b4;
const T_SEQ3: usize = 0x2b8;
const PVARS: usize = 0x2c0;

/// 1°/s as radians a tick.
const DEG_S: f32 = 0.000_290_888_25;
/// The rotor's turn a tick (`0x3ed4fa0b`).
const ROTOR_STEP: f32 = f32::from_bits(0x3ed4_fa0b);
/// The shot's speed a tick (`0x3f555556`: 50 a second).
const SHOT_SPEED: f32 = f32::from_bits(0x3f55_5556);
/// 90°.
const QUARTER: f32 = f32::from_bits(0x3fc9_0fdc);
/// The pitch's limits (0.2°, 0.2°, 12°).
const PITCH_A: f32 = f32::from_bits(0x3b64_c38a);
const PITCH_MAX: f32 = f32::from_bits(0x3e56_7752);

fn st(w: &mut MobyWorld, id: usize, s: u8, sub: i32) { w.mobys[id].set_state(s, sub); }
/// `if seq != 0 { set_anim(0, 0, 10) }`.
fn seq0(w: &MobyWorld, id: usize, ctx: &mut Ctx) { if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 10); } }
fn seq3(w: &MobyWorld, id: usize, ctx: &mut Ctx) { set_seq(w, id, ctx, 3, 0, 10); }
fn found1(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_i(FOUND1) == 1 }
fn riser(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_i(RISER) != 0 }
/// Ratchet's moby's place (`0x18c0b0`'s `+0x10`).
fn hero_at(w: &MobyWorld, ctx: &Ctx) -> V3 { ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or(ctx.hero.pos, |h| h.pos) }
fn yaw_to_hero(w: &MobyWorld, id: usize, ctx: &Ctx) -> f32 {
    let (h, p) = (ctx.hero.pos, w.mobys[id].pos);
    atan2(h[0] - p[0], h[1] - p[1])
}

/// 0x0ebb (module docs).
pub fn update_0ebb(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    if head(w, id, ctx) {
        w.delete(id);
        return;
    }
    if subs(w, id, ctx) { return; }
    states(w, id, ctx);
    if matches!(w.mobys[id].state, 6 | 7) {
        let sa = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
        let m = &mut w.mobys[id];
        if sa == 3 {
            let t = m.pv_i(T_SEQ3) + 1;
            m.set_pv_i(T_SEQ3, t);
            if 240 < t {
                seq0(w, id, ctx);
                w.mobys[id].set_pv_i(T_SEQ3, 0);
            }
        } else if sa == 0 {
            let t = m.pv_s(T_SEQ0) + 1;
            m.set_pv_s(T_SEQ0, t);
            if 60 < t {
                seq3(w, id, ctx);
                w.mobys[id].set_pv_s(T_SEQ0, 0);
            }
        }
    }
    tail(w, id, ctx);
    w.changed = true;
}

/// `0x405e88` (module docs): true when it is to be deleted.
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 { setup(w, id, ctx); }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    bob(&mut w.mobys[id], 0.75, 100.0 * DEG_S, -1.0, BOB_PHASE, BOB_LAST);
    ai::flash(&mut w.mobys[id], REACT);
    let mut hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
    if !w.mobys[id].count_s(WAIT) { hit = None; }
    if let Some(h) = hit.as_mut() {
        let k = w.mobys[id].pv_f(KIND_SCALE);
        if k != 0.0 && h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == CLASS) { h.tmpl.damage *= k; }
    }
    w.clear_hit(id);
    let wrench = hit.and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == crate::hero::items::WRENCH_CLASS);
    let dmg = ai::damage_of(w, id, hit.as_ref(), wrench);
    let m = &mut w.mobys[id];
    let h = m.pv_f(HEALTH) - dmg;
    m.set_pv_f(HEALTH, h);
    if m.pv_s(MOST) as f32 * 0.33 <= dmg || h <= 0.0 {
        set_seq(w, id, ctx, 4, 0, 3);
        st(w, id, 9, 3);
    }
    if w.mobys[id].pv_b(STATUS) == 2 {
        if let Some(r) = ai::link(&w.mobys[id], RIDER_LINK) { w.delete(r); }
        bolts::drop(w, id, 0, 3, ctx);
        return true;
    }
    let m = &w.mobys[id];
    let (a1, r1, a2) = (m.pv_i(AREA1), m.pv_f(RANGE1), m.pv_i(AREA2));
    let f1 = ai::find_target(w, id, ctx, r1, 0.0, a1, None, false);
    w.mobys[id].set_pv_i(FOUND1, f1 as i32);
    let f2 = ai::find_target(w, id, ctx, 0.0, 0.0, a2, None, false);
    w.mobys[id].set_pv_i(FOUND2, f2 as i32);
    false
}

/// `0x405e88`'s set-up (module docs).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if ctx.game.level == 0x10 && (m.uid == 0x27 || m.uid == 0x114) { m.set_pv_i(RISER, 1); }
    if m.pv_i(RISER) != 0 { m.set_pv_f(RISE_TO, m.pos[2] + 300.0); }
    m.set_pv_b(REACT + 0x10, 8);
    m.set_pv_i(REACT + 0x48, 3);
    m.set_pv_f(REACT + 0x3c, f32::from_bits(0x3e19_999a));
    m.set_pv_i(PATH_AT, 0);
    m.set_pv_f(REACT + 0x30, 0.5);
    m.set_pv_f(REACT + 0x34, 0.5);
    if m.pv_s(MOST) == 0 {
        m.set_pv_s(MOST, 5);
        m.set_pv_f(HEALTH, 5.0);
    }
    let s = m.pv_f(SPEED);
    m.set_pv_f(MV, 0.0);
    m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 0x17);
    m.set_pv_b(STATUS, 1);
    m.set_pv_f(MV + 0x34, 0.5);
    m.set_pv_f(MV + 4, 1.0);
    m.set_pv_f(MV + 0xc, f32::from_bits(0x3ecc_cccd));
    m.set_pv_f(MV + 0x10, QUARTER);
    m.set_pv_f(MV + 0x30, f32::from_bits(0x3fc6_d3f3));
    m.set_pv_f(MV + 8, f32::from_bits(0x3ecc_cccd));
    ai::senses(m, (s + s) * 0.000_277_777_78, s * 0.016_666_668, f32::from_bits(0x3ae4_c38a), PITCH_MAX);
    st(w, id, 1, 0);
    seq0(w, id, ctx);
    let m = &mut w.mobys[id];
    // `0x2e93a8` (L00 `0x2effb8`): 17 and 32 as bytes.
    m.set_pv_b(0x48, 17);
    m.set_pv_b(0x4a, 32);
}

/// The sub-states (module docs); true when it was destroyed.
fn subs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    match w.mobys[id].sub_state {
        0 => {
            if w.mobys[id].first_sub() {
                let t = ctx.game.rng.rand_range(0, 30);
                w.mobys[id].set_pv_s(T_ALARM, t as i16);
            }
            if found1(w, id) {
                seq3(w, id, ctx);
                st(w, id, 5, 2);
                if w.mobys[id].pv_i(RISE_NOW) != 0 || riser(w, id) {
                    seq0(w, id, ctx);
                    st(w, id, 4, 2);
                }
                if w.mobys[id].pv_i(PATH_IN) != -1 {
                    st(w, id, 3, 2);
                    if w.mobys[id].pv_i(KEEP_SEQ) == 0 || riser(w, id) { seq0(w, id, ctx); }
                }
            } else if w.mobys[id].pv_i(ALARM) != 0 && w.mobys[id].count_s(T_ALARM) {
                st(w, id, 2, 1);
                w.mobys[id].set_pv_i(ALARM, 0);
                if w.mobys[id].pv_i(TIMER) < 300 {
                    let t = ctx.game.rng.rand_range(480, 600);
                    w.mobys[id].set_pv_i(TIMER, t);
                }
            }
        }
        1 => {
            if found1(w, id) || w.mobys[id].pv_i(FOUND2) == 1 { st(w, id, 5, 2); }
        }
        2 => {
            let m = &w.mobys[id];
            if !found1(w, id) && m.pv_i(FOUND2) != 1 && m.state != 4 && m.state != 3 {
                st(w, id, 1, 0);
                seq0(w, id, ctx);
                let t = ctx.game.rng.rand_range(580, 600);
                w.mobys[id].set_pv_i(TIMER, t);
            }
        }
        3 => {
            if w.mobys[id].pv_f(HEALTH) <= 0.0 {
                die(w, id, ctx);
                return true;
            }
            if !wrapped(w, id, ctx) { return false; }
            let m = &mut w.mobys[id];
            m.set_pv_i(T_SEQ3, 0);
            m.set_pv_s(T_SEQ0, 0);
            if found1(w, id) {
                seq3(w, id, ctx);
                st(w, id, 5, 2);
                if w.mobys[id].pv_i(PATROL_A) != -1 { st(w, id, 6, -1); }
            } else {
                st(w, id, 2, 1);
                seq0(w, id, ctx);
                let t = ctx.game.rng.rand_range(580, 600);
                w.mobys[id].set_pv_i(TIMER, t);
            }
            if riser(w, id) {
                st(w, id, 4, 2);
                seq0(w, id, ctx);
            }
        }
        _ => {}
    }
    false
}

/// Sub 3 with no health left (module docs).
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    bolts::drop(w, id, 0, 3, ctx);
    let (pos, rot, o_class) = (w.mobys[id].pos, w.mobys[id].rot, w.mobys[id].o_class);
    for k in 0..5u8 {
        let mut v = ctx.game.rng.rand_vec(f32::from_bits(0x3e08_8889), f32::from_bits(0x3e4c_ccce));
        v[2] = v[2].abs() * 2.5;
        shards::spawn_ball(w, ctx, o_class, Some(id), pos, rot, 30.0, 15.0, k, v, 0xff);
    }
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    for size in [2.0, 1.0] {
        let b = blast::Blast { size, bursts: [15, 10, 10], puffs: 10, streaks: 15, light: true, shake: false };
        blast::burst_green(w, ctx, pos, &b);
    }
    // `0x1b9940`: the up-and-down shake and the roll (`+0x160..` / `+0x180..`).
    ctx.game.shakes.ask(0, f32::from_bits(0x3e4c_cccd), 60);
    let roll = &mut ctx.game.shakes.0[2];
    (roll.amp, roll.timer) = (f32::from_bits(0x3dcc_cccd), 90);
    if let Some(r) = ai::link(&w.mobys[id], RIDER_LINK) { w.delete(r); }
    let rng = &mut ctx.game.rng;
    let x = rng.randf_signed(0.5, 4.0) * 0.016_666_668;
    let y = rng.randf_signed(0.0, 4.0) * 0.016_666_668;
    let z = rng.randf(7.0, 9.0) * 0.016_666_668;
    let v = [x, y, z.abs() * 2.0];
    if let Some(k) = shards::spawn_whole_ball(w, ctx, RIDER, Some(id), pos, rot, 180.0, 25.0, v, 0xff) {
        if let Some((l, a)) = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map(|h| (h.light, h.ambient)) { (w.mobys[k].light, w.mobys[k].ambient) = (l, a); }
    }
    w.delete(id);
}

/// Pitched toward Ratchet's moby (`0x318a78` = L00 `0x31f230`).
fn pitch(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let t = hero_at(w, ctx);
    let m = &mut w.mobys[id];
    let p = atan2(dist2(m.pos, t), t[2] - m.pos[2]);
    let (mut r, mut v) = (m.rot[1], m.pv_f(PITCH_V));
    turn_ease(-p, PITCH_A, PITCH_A, PITCH_MAX, &mut r, &mut v, 0);
    m.rot[1] = r;
    m.set_pv_f(PITCH_V, v);
}

/// At frame 1.5 with Ratchet within 90° of its facing: a shot from each of its joints 2 and 3 (`sight`: only with
/// nothing on the line, or Ratchet's moby or a moby not of its kind on it).
fn volley(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, sight: bool) {
    if !ai::reached(w, id, ctx, 1.5) { return; }
    let yaw = yaw_to_hero(w, id, ctx);
    if QUARTER <= rot_dist(w.mobys[id].rot[2], yaw) { return; }
    let aim = ctx.hero.aim_point;
    let dmg = w.mobys[id].pv_b(DAMAGE) as f32;
    for j in [2, 3] {
        let Some(p) = ctx.host.joint(&w.mobys[id], j) else { continue };
        let v = set_len(sub(aim, p), SHOT_SPEED);
        if sight {
            if let Some(o) = w.coll_line(ctx, p, aim, 0, Some(id)) {
                let ok = o.moby.is_some_and(|k| Some(k) == ctx.hero_moby || w.mobys.get(k).is_some_and(|m| m.o_class != CLASS));
                if !ok { continue; }
            }
        }
        fire(w, ctx, dmg, id, v, p);
    }
}

/// `0x406268` (module docs): true at the path's end.
fn path_in(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    let Some(pts) = usize::try_from(m.pv_i(PATH_IN)).ok().and_then(|p| ctx.level.paths.get(p)) else { return true };
    let i = m.pv_i(PATH_AT);
    let Some(q) = usize::try_from(i).ok().and_then(|i| pts.get(i)) else { return true };
    let p = [q[0], q[1], q[2]];
    let d = set_len(sub(p, m.pos), f32::from_bits(0x3e4c_ccce));
    ai::move_by(w, id, ctx, d);
    let m = &mut w.mobys[id];
    let d = if m.pv_i(RISER) == 0 { dist2(m.pos, p) } else { dist(m.pos, p) };
    if d < 0.5 { m.set_pv_i(PATH_AT, i + 1); }
    false
}

/// The states (module docs).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        3 => {
            if !riser(w, id) {
                let yaw = yaw_to_hero(w, id, ctx);
                ai::face(&mut w.mobys[id], yaw);
            }
            if path_in(w, id, ctx) {
                if w.mobys[id].pv_i(PATROL_A) != -1 { st(w, id, 6, -1); } else { st(w, id, 5, -1); }
                seq3(w, id, ctx);
                if w.mobys[id].pv_i(RISE_NOW) != 0 || riser(w, id) {
                    seq0(w, id, ctx);
                    st(w, id, 4, 2);
                    let m = &mut w.mobys[id];
                    m.set_pv_f(RISE_TO, m.pos[2] + 260.0);
                }
            }
            if !riser(w, id) { pitch(w, id, ctx); }
            volley(w, id, ctx, false);
        }
        6 | 7 => {
            let s = w.mobys[id].state;
            let yaw = yaw_to_hero(w, id, ctx);
            ai::face(&mut w.mobys[id], yaw);
            let (on, other, next) = if s == 6 { (PATROL_A, PATROL_B, 7) } else { (PATROL_B, PATROL_A, 6) };
            let path = w.mobys[id].pv_i(on);
            if path != -1 && ai::follow_path(w, id, ctx, path) && w.mobys[id].pv_i(other) != -1 { st(w, id, next, -1); }
            pitch(w, id, ctx);
            volley(w, id, ctx, true);
        }
        4 => {
            let m = &mut w.mobys[id];
            let (to, mut z, mut v) = (m.pv_f(RISE_TO), m.pos[2], m.pv_f(RISE_V));
            ease(to, 0.0125, f32::from_bits(0x3b36_0b61), f32::from_bits(0x3d88_8889), &mut z, &mut v);
            m.pos[2] = z;
            m.set_pv_f(RISE_V, v);
            if to - z < 0.3 && m.pv_i(RISER) == 0 {
                seq3(w, id, ctx);
                st(w, id, 5, -1);
                if w.mobys[id].pv_i(PATROL_A) != -1 { st(w, id, 6, -1); }
            }
        }
        5 => {
            let yaw = yaw_to_hero(w, id, ctx);
            ai::face(&mut w.mobys[id], yaw);
            pitch(w, id, ctx);
            volley(w, id, ctx, true);
        }
        _ => {}
    }
}

/// `0x406378` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.count_i(TIMER);
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    rider(w, id, ctx);
    let m = &mut w.mobys[id];
    m.set_pv_b(NODE_LINKED, 1);
    let a = m.pv_f(ROTOR);
    let q = crate::quat::from_rows(&euler_rows([0.0, 0.0, a]));
    m.manips.retain(|x| x.0 != 7);
    m.manips.push((7, q));
    m.set_pv_f(ROTOR, add_rot(a, ROTOR_STEP));
    if m.pv_i(RISER) == 0 { return; }
    let mut t = crate::moby::hits::HitTemplate::new(10.0, Some(id), 1, [m.rows[2][0], m.rows[2][1], m.rows[2][2], 0.0]);
    t.kind = (CLASS as u16 as u32) << 16;
    let mut p = m.pos;
    p[2] += 3.5;
    let p = add(p, scale(m.rows[0], f32::from_bits(0xbfa6_6666)));
    let h = ctx.hero.pos;
    if (h[2] - p[2]).abs() < 0.1 && dist2(p, h) < f32::from_bits(0x4111_999a) {
        if let Some(k) = ctx.hero_moby { w.deliver_hit_any(k, &t, [0.0, 0.0, 0.0, 1.0]); }
    }
}

/// `0x406500`: the rider made (`0x322bd8` = L00 `0x32a748`, crate::moby::MobyWorld::adopt: update and draw distance 0xff, drawn, its place, angles
/// and light, mode & 0xfff8; then its draw distance and sequence 1), else kept on it, shown and hidden with it.
fn rider(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (pos, rot, hidden, dd) = { let m = &w.mobys[id]; (m.pos, m.rot, m.mode & 1 != 0, m.draw_dist) };
    match ai::link(&w.mobys[id], RIDER_LINK).filter(|&k| k < w.mobys.len()) {
        None => {
            let k = w.create(RIDER);
            ai::set_link(&mut w.mobys[id], RIDER_LINK, Some(k));
            w.adopt(k, Some(id), &*ctx.host);
            let r = &mut w.mobys[k];
            r.draw_dist = dd;
            r.drive = true;
            ctx.host.set_anim(&w.mobys[k], 1, 0, 0);
            w.rebuild(k, &*ctx.host);
        }
        Some(k) => {
            let r = &mut w.mobys[k];
            (r.pos, r.rot) = (pos, rot);
            if hidden {
                r.mode |= 1;
                r.drawn = 0;
            } else {
                r.mode &= !1;
                r.drawn = 1;
            }
            w.rebuild(k, &*ctx.host);
        }
    }
}

// The shot 0x0ebd.

const S_VEL: usize = 0x00;
const S_OWNER: usize = 0x10;
const S_DAMAGE: usize = 0x14;
const S_LIFE: usize = 0x24;
const S_PVARS: usize = 0x30;

/// `0x4068b0(damage, owner, vel, pos)` (module docs): update and draw distances 0xff, drawn, state 0, the owner's
/// light; its yaw along `vel` + 90°, its x angle the climb; mode | 0x308; scale `$gp+0x24b0` (0.0001); the two
/// twinkles.
pub fn fire(w: &mut MobyWorld, ctx: &mut Ctx, damage: f32, owner: usize, vel: V3, pos: V3) -> usize {
    let k = w.create(SHOT);
    let (light, amb) = (w.mobys[owner].light, w.mobys[owner].ambient);
    let m = &mut w.mobys[k];
    if m.pvars.len() < S_PVARS { m.pvars.resize(S_PVARS, 0); }
    m.update = Some(update_0ebd);
    m.mode &= !mode::NO_UPDATE;
    (m.update_dist, m.draw_dist, m.drawn, m.state) = (0xff, 0xff, 1, 0);
    (m.light, m.ambient) = (light, amb);
    m.pos = pos;
    m.set_pv_v(S_VEL, vel);
    ai::set_link(m, S_OWNER, Some(owner));
    m.set_pv_f(S_DAMAGE, damage);
    m.set_pv_f(0x20, f32::from_bits(0x3e19_999a));
    m.mode |= 0x208;
    m.rot[2] = add_rot(atan2(vel[0], vel[1]), QUARTER);
    m.rot[0] = -atan2(len2(vel), vel[2]);
    // `0x315278`: the angle its rows turn by before they are built again from its angles.
    ctx.game.rng.rand_angle();
    let m = &mut w.mobys[k];
    m.rows = euler_rows(m.rot);
    m.alpha = 0x80;
    m.mode |= mode::KEEP_ROWS;
    m.scale = f32::from_bits(0x38d1_b717);
    m.set_pv_i(S_LIFE, 0);
    let s = ctx.game.rng.randf(8.0, 10.0);
    if let Some(sys) = ctx.parts.as_deref_mut() {
        crate::particles::twinkle(sys, &mut ctx.game.rng, s * 0.3, s, 0.0, pos, 15, 0x8010_3080, 0, 10, [0.0; 3], 0);
        crate::particles::twinkle(sys, &mut ctx.game.rng, s * 0.04, s * 0.4, 0.0, pos, 15, 0xffff_ffff, 1, -10, [0.0; 3], 0);
    }
    k
}

/// 0x0ebd (`0x4065d0`, module docs).
pub fn update_0ebd(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let life = m.pv_i(S_LIFE) + 1;
    m.set_pv_i(S_LIFE, life);
    let dmg = m.pv_f(S_DAMAGE);
    if 0xf0 < life {
        let pos = w.mobys[id].pos;
        ai::attack_sphere(w, id, ctx, f32::from_bits(0x3f19_999a), dmg, 0.0, pos, 0x1_0003, 0, 1, 0);
        st(w, id, 1, -1);
    }
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (pos, vel, owner) = (m.pos, m.pv_v(S_VEL), ai::link(m, S_OWNER));
            let next = add(pos, vel);
            let mut t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 3, [vel[0], vel[1], vel[2], 0.0]);
            t.kind = (SHOT as u16 as u32) << 16;
            let (coll, scene) = (ctx.coll, ctx.scene);
            let line = coll.and_then(|c| crate::collision::coll_line_m(c, scene, pos, next, crate::collision::QueryFlags(0), Some(id)));
            if let Some(o) = line.as_ref() {
                if let Some(k) = o.moby { w.deliver_hit_any(k, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
            }
            let met = match line {
                Some(o) => Some(o.moby),
                None => coll.and_then(|c| crate::collision::coll_sphere_m(c, scene, pos, f32::from_bits(0x3e99_999a), crate::collision::QueryFlags(0), Some(id))).map(|o| o.moby),
            };
            let push = match met {
                Some(None) => Some(3.0),
                Some(Some(k)) if Some(k) != owner => Some(1.5),
                _ => None,
            };
            if let Some(k) = push {
                let d = scale(vel, k);
                let mut t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 0x1_0003, [d[0], d[1], 1.0, 5_627.925]);
                t.kind = (SHOT as u16 as u32) << 16 | 1 << 8;
                w.sphere_hit(scene, pos, f32::from_bits(0x3f19_999a), 0, Some(id), Some(&t));
                st(w, id, 1, -1);
                return;
            }
            w.mobys[id].pos = next;
        }
        1 => {
            let pos = w.mobys[id].pos;
            let b = blast::Blast { size: 0.25, bursts: [25, 20, 20], puffs: 20, streaks: 0, light: true, shake: false };
            blast::burst_green(w, ctx, pos, &b);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.delete(id);
            return;
        }
        _ => {}
    }
    // `$gp+0x240c`: 40° a tick about the side row.
    let m = &mut w.mobys[id];
    let a = 40.0 * 0.017_453_294;
    let n = m.rows[1];
    m.rows[2] = crate::camera::rot(m.rows[2], a, n);
    m.rows[0] = crate::camera::rot(m.rows[0], a, n);
    w.changed = true;
    w.register_draw(id);
}

/// The shot's bolt (`0x406c40`'s words, `$gp+0x2418..+0x24ac`, `0x1b14a4`): a green head (FX 8: `0xff14ff14` →
/// `0x80208040`, 6 back, 0.2 across), green flares (FX 0xd, turning 10° a call) and glows (FX 5, 0.75, `0x20408020`).
pub const BOLT: crate::fx::trails::Bolt = {
    use crate::fx::trails::{Bolt, Flare};
    Bolt {
        fx: [8, 0xd],
        head: [0xff14_ff14, 0x8020_8040],
        length: 6.0,
        width: 0.2,
        from: -0.0,
        turn: 10.0,
        flares: [
            Flare { rgba: 0x5a14_8014, width: 0.2, from: -0.0, length: 4.0 },
            Flare { rgba: 0x5a14_8014, width: 0.1, from: 0.75, length: 4.0 },
            Flare { rgba: 0x3c14_8014, width: 0.4, from: 0.75, length: 4.0 },
            Flare { rgba: 0x640a_ff0a, width: 0.2, from: 0.0, length: 0.6 },
        ],
        flare_end: 0x80_8080,
        glow: (5, 0.75, 0x2040_8020),
    }
};

/// `0x406ad0`: the shot's bolt (crate::fx::trails::shot_bolt).
pub fn draw_0ebd(w: &mut MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    crate::fx::trails::shot_bolt(w, id, &BOLT, rng, cam, out);
}
