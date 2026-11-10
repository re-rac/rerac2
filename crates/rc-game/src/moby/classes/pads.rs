//! The pads Ratchet stands on whose code several levels share (L09 addresses):
//!
//! * **0x0efc** ([`update_0efc`], L09 `0x41bbf0`, 1 placed; L13 `0x41c768`, 2): a pad that lights up when it is set
//!   off.
//! * **0x0e80** ([`update_0e80`], L09 `0x4116e0`, 1 placed; L18 `0x422a88`, 1; L22 `0x404080`, 1): a launch pad to
//!   another level.

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
