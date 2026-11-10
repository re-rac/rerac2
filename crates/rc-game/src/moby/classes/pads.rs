//! The pads Ratchet stands on whose code several levels share (L09 addresses):
//!
//! * **0x0efc** ([`update_0efc`], L09 `0x41bbf0`, 1 placed; L13 `0x41c768`, 2): a pad that lights up when it is set
//!   off.
//! * **0x0e80** ([`update_0e80`], L09 `0x4116e0`, 1 placed; L18 `0x422a88`, 1; L22 `0x404080`, 1): a launch pad to
//!   another level.
//! * **0x0ed0** ([`update_0ed0`], L18 `0x42c4f8`, 1 placed; L19 `0x40efd0`, 5; L20 `0x432580`, 10): a step pad that
//!   sets off what it is tied to.

use crate::moby::{Ctx, MobyWorld};

pub const PAD: i16 = 0x0efc;

/// 0x0efc (`0x41bbf0`, 1 placed; L13 `0x41c768`; the decomp's `func_L09_0041A700`): a pad that lights up when it is
/// set off. The test runs top to bottom:
///
/// 1. Ratchet standing on it sets it off.
/// 2. Otherwise, with its mission (`+0xb0`) done and no enable word (`+0x68`[4]), it is set off.
/// 3. Otherwise, with no enable word, it waits.
/// 4. Otherwise, it is set off once its uid's save count is set.
///
/// * **0**: its pulse's phase is random; it sinks 0.35; then 1.
/// * **1**: its glow word (`+0x78`) pulses (6° a tick; grey 32..128 by 4·sin − 3). Set off, it turns green (0x80208020), plays
///   its class sound 0, counts its uid and goes to 2.
///
/// Not ported: the second way it is set off — with its enable word, the level's special moby
/// (`$gp+0x1844` / `0x1b1648`) within 0.5 of it — and the level's spent table `0x21e9f0`.
pub fn update_0efc(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let a = ctx.game.rng.rand_angle();
            let m = &mut w.mobys[id];
            m.set_pv_f(0, a);
            m.pos[2] -= 0.35;
            m.set_state(1, -1);
        }
        1 => {
            let m = &mut w.mobys[id];
            let a = crate::hero::physics::add_rot(m.pv_f(0), 0.104_719_77);
            m.set_pv_f(0, a);
            let shade = (((a.sin() * 4.0 - 3.0) * 128.0) as i32).clamp(32, 128) as u32;
            m.glow = 0x8000_0000 | shade << 16 | shade << 8 | shade;
            let on = ctx.hero.ground_moby == Some(id);
            let enabled = m.pv_i(4) != 0;
            let level = ctx.game.level.max(0) as usize;
            let done = ctx.game.missions.get(level * 0x10 + m.mission as usize).is_some_and(|b| b & 0x80 != 0);
            let go = on || (done && !enabled) || (enabled && ctx.game.uid_count(m.uid) != 0);
            if !go { return; }
            m.set_state(2, -1);
            m.timer = 1;
            m.glow = 0x8020_8020;
            let uid = m.uid;
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            ctx.game.count_uid(uid);
        }
        _ => {}
    }
}

pub const LAUNCH_PAD: i16 = 0x0e80;

/// 0x0e80's `$gp+0x2760..+0x27a0`: the twinkles' lives, ring, rise, sink, sizes, colours, spins and depth.
const PAD_LIFE: (i32, i32) = (90, 160);
const PAD_RING: (f32, f32) = (0.3, 1.25);
const PAD_RISE: (f32, f32) = (0.3, 1.5);
const PAD_LIFT: (f32, f32) = (0.1, 2.0);
const PAD_SIZE: (f32, f32) = (1.0, 0.2);
const PAD_END: (f32, f32) = (1.0, 3.0);
const PAD_RGBA: (u32, u32) = (0x80f0_f000, 0x8000_f0f0);
const PAD_SPIN: (i32, i32) = (2, 5);
const PAD_DEPTH: f32 = 0.25;
/// The prompt's owner (`0x327a28(4, …)`).
const PAD_PROMPT: i32 = 4;

/// 0x0e80 (`0x4116e0`, 3 placed; L18 `0x422a88`, L22 `0x404080`; the decomp's v2.00 `func_US2_L09_004116E0`): a
/// launch pad to level `+0x68`[0].
///
/// * **Every tick**: a twinkle rises from a random point 0.3..1.25 across it, 0.25 below it (`0x2c2210`).
/// * **0**: its loop sound handle cleared; then 1.
/// * **1**: its loop sound 0 is kept playing. Ratchet standing on it gets the prompt (owner 4: on Dobbo 0x1318, on
///   level 18 0x133f). With △ pressed while that prompt shows, it goes to 2 (to 4 on other levels).
/// * **2**: scene 100 (level 18: 0x92) is started; then 3.
/// * **3**: when the scene is over, 4.
/// * **4**: shown. Dobbo's flags 0x1d / 0x2d (level 18's 0x87 / 0x9f) are set, and the trip to its level asked for
///   with the save (`0x2eb1b8`).
///
/// Not ported: the second prompt 0x1306 (for Ratchet's help state 2), and the flag `0x1b25a8` held during the
/// scene.
pub fn update_0e80(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::add;
    let pos = w.mobys[id].pos;
    if let Some(sys) = ctx.parts.as_deref_mut() {
        let g = &mut ctx.game.rng;
        let r = g.randf(PAD_RING.0, PAD_RING.1);
        let (a, b) = (g.rand_angle(), g.rand_angle());
        let mut p = add([a.cos() * r, b.sin() * r, 0.0], pos);
        p[2] -= PAD_DEPTH;
        let vz = g.randf(PAD_RISE.0, PAD_RISE.1) / 60.0;
        let life = g.rand_range(PAD_LIFE.0, PAD_LIFE.1) as i16;
        let size = g.randf(PAD_SIZE.0, PAD_SIZE.1);
        let end = g.randf(PAD_END.0, PAD_END.1);
        let t = g.randf(0.0, 1.0);
        let rgba = crate::hud::lerp_rgba(t, PAD_RGBA.0, PAD_RGBA.1);
        let n = g.rand_range(PAD_SPIN.0, PAD_SPIN.1);
        let spin = if g.rand() & 1 != 0 { -n } else { n } as i8;
        let lift = -(g.randf(PAD_LIFT.0, PAD_LIFT.1) / 3600.0);
        crate::particles::twinkle(sys, g, size, end, lift, p, life, rgba, 2, spin, [0.0, 0.0, vz], 0);
    }
    let level = ctx.game.level;
    match w.mobys[id].state {
        0 => {
            w.mobys[id].set_state(1, -1);
            w.mobys[id].set_pv_i(8, -1);
        }
        1 => {
            let h = w.mobys[id].pv_i(8);
            if h < 0 || !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(8, h);
            }
            if ctx.hero.ground_moby != Some(id) { return; }
            let text = match level { 9 => 0x1318, 18 => 0x133f, _ => 0 };
            let lease = ctx.game.prompt.request(PAD_PROMPT, text, 1);
            if lease == 0 || ctx.hero.items.pressed & crate::pad::button::TRIANGLE == 0 { return; }
            w.mobys[id].set_state(if level == 9 || level == 18 { 2 } else { 4 }, -1);
        }
        2 => {
            let scene = if level == 9 { 100 } else { 0x92 };
            if ctx.game.scenes.queue.push(scene) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(3, -1);
            }
        }
        3 => if !ctx.game.scenes.in_scene() { w.mobys[id].set_state(4, -1) },
        4 => {
            let m = &mut w.mobys[id];
            m.drawn = 1;
            m.mode &= !1;
            let to = m.pv_i(0);
            let flags: &[usize] = match level { 9 => &[0x1d, 0x2d], 18 => &[0x87, 0x9f], _ => &[] };
            for &k in flags { if let Some(f) = ctx.game.flags.get_mut(k) { *f = 1; } }
            ctx.game.go_to_level(to, true);
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) < 0xffff { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

/// The step pad 0x0ed0 ([`update_0ed0`], L18 `0x42c4f8`, 1 placed; L19 `0x40efd0`, 5; L20 `0x432580`, 10; the decomp's
/// `func_US2_L18_0042C4F8`, the same code on the three levels): a pad that pulses grey until Ratchet stands on it (or
/// its uid or mission says it was set off before), then sets off what it is tied to and glows orange.
///
/// **Its pvars:** `+0x00` a moby to hook (−1 none), `+0x04` the pulse's phase, `+0x08` its sound played, `+0x09` the
/// body back to Ratchet when set off (`0x2d0050`), `+0x0a` the timer byte for its group (0xff none), `+0x0b` its group
/// hooked, `+0x0c` the group (−1 none), `+0x10` its uid not counted, `+0x11` its uid's record not read, `+0x12` its
/// mission done and the checkpoint where Ratchet stands, `+0x13` set off once its mission is done.
///
/// **Each tick** (L18 addresses) the phase turns 6° (`0x30a650` = L00 `0x2efe60`, wrapped).
/// * **0**: touched — Ratchet on it (`+0x33c` / `+0x340`: crate::hero::Hero::ground_moby, grounded) within half its
///   size (its scale over its class's) across of its joint 0 (`0x32d808` = L00 `0x3127c8`; `0x309820` = L00
///   `0x2ef050`, the xy distance): its uid counted (crate::game::GameState::count_uid) unless `+0x10`, its mission done
///   (`0x32e2e8` = L00 `0x3132a8`) with `+0x12`. Set off when touched, when (with a mission and not `+0x11`) its uid is
///   gone for good (crate::game::GameState::uid_dead) or counted, or with `+0x13` its mission done: state 1 and its
///   timer byte 1; the moby `+0x00` hooked (`0x3458f0` = L00 `0x32a818`); with `+0x09` the body back
///   (crate::hero::Hero::body_back); its group hooked (`0x337928` = L02 `0x320b50`) with `+0x0b` and given its timer
///   byte (`0x3378d8` = L00 `0x31c970`) unless 0xff; touched with `+0x12`, the checkpoint at Ratchet (`0x375e08` =
///   L00 `0x359b20`). Its glow (`+0x78`) a grey pulse: `(sin(phase)·4 − 3)·128` (`0x309c50` / `0x30a7a8` = L00
///   `0x2ef460` / `0x2effb8`) held to 32..128.
/// * **Else**: on its first tick its sound (class 0x08a1's sound 0, `0x37c980` = L00 `0x360698`); its glow
///   0x80208010.
///
/// The checkpoint is at Ratchet's own moby (`+0xc18`): with the Hypnomatic's body playing (crate::hero::hypno) where
/// he stands, not the creature.
pub const STEP_PAD: i16 = 0x0ed0;
/// The class whose sound 0 it plays when set off.
const SOUND_CLASS: i16 = 0x08a1;

/// 0x0ed0 ([`STEP_PAD`]).
pub fn update_0ed0(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.is_empty() { return; }
    let phase = crate::hero::physics::add_rot(w.mobys[id].pv_f(4), 0.104_719_77);
    w.mobys[id].set_pv_f(4, phase);
    let flag = |w: &MobyWorld, k: usize| w.mobys[id].pvars.get(k).copied().unwrap_or(0);
    if w.mobys[id].state != 0 {
        if flag(w, 8) == 0 {
            ctx.host.play_sound_of(&w.mobys[id], SOUND_CLASS, 0, 0);
            w.mobys[id].pvars[8] = 1;
        }
        w.mobys[id].glow = 0x8020_8010;
        return;
    }
    let m = &w.mobys[id];
    let p = ctx.host.joint(m, 0).unwrap_or(m.pos);
    let hp = ctx.hero.pos;
    let d = ((hp[0] - p[0]).powi(2) + (hp[1] - p[1]).powi(2)).sqrt();
    w.mobys[id].pvars[8] = 0;
    let m = &w.mobys[id];
    let class_scale = w.class(m.o_class).map_or(1.0, |c| c.scale);
    let (uid, mission) = (m.uid, m.mission);
    let touched = ctx.hero.ground_moby == Some(id) && ctx.hero.grounded_ticks != 0 && d < m.scale * 0.5 / class_scale;
    if touched {
        if flag(w, 0x10) == 0 { ctx.game.count_uid(uid); }
        if flag(w, 0x12) != 0 { ctx.game.mission_done(mission); }
    }
    let before = mission != 0xff && flag(w, 0x11) == 0
        && (ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&v| v != 0) || ctx.game.uid_count(uid) != 0);
    let done = flag(w, 0x13) != 0
        && ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + mission as usize).is_some_and(|&b| b & 0x80 != 0);
    if before || done || touched {
        let m = &mut w.mobys[id];
        m.timer = 1;
        m.state = 1;
        let hook = m.pv_i(0);
        if let Some(t) = usize::try_from(hook).ok().and_then(|k| w.mobys.get_mut(k)) { t.once |= 4; }
        if flag(w, 9) != 0 { ctx.hero.body_back(ctx.game.scenes.mode == crate::scene::SCENE); }
        let group = w.mobys[id].pv_i(0xc);
        if group != -1 {
            if flag(w, 0xb) != 0 { crate::moby::classes::maktar::hook_group(w, group, true); }
            let t = flag(w, 0xa);
            if t != 0xff { w.poke_group(group, t); }
        }
        if flag(w, 0x12) != 0 && touched {
            let h = &ctx.hero;
            let (p, r) = if h.mode == 4 { (h.hypno.ratchet_pos, h.hypno.ratchet_rot) } else { (h.pos, h.rot) };
            ctx.game.save_checkpoint(p, r);
        }
    }
    let v = (((phase.sin() * 4.0 - 3.0) * 128.0) as i32).clamp(32, 128) as u32;
    w.mobys[id].glow = 0x8000_0000 | v << 16 | v << 8 | v;
}
