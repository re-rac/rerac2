//! Smolg's floating things (L17 addresses).
//!
//! * **0x08af** ([`update_08af`], `0x3c5610`, 18 placed; the decomp's `func_L17_003C4090`): a buoy. Its glow word
//!   flashes white 15 ticks in 60 by its uid (else grey); stopped (`+0x40` 1) nothing more. It rises and sinks on its
//!   own ([`rise_fall`]), bobs 5 about that height at 45°/s (`0x3265a8` = L00 `0x324430`; `$gp+0x120c` / `+0x1208`
//!   8 s), sways 0.07 (`$gp+0x1210`; 5 s and 6 s, `$gp+0x1214` / `+0x1218`, `0x326660` = L00 `0x3244e8`) and spins
//!   `+0x38` a tick (0.25..0.75°, either way, set at the start). Within 80 of Ratchet its loop sound 0 (mode 4,
//!   `+0x3c`) plays, else it stops. A hit with the flags 0x4010000 blows it up: a blast (size 10, sprites 25 / 20 / 20,
//!   20 puffs, 45 streaks, its flash ball, the shake), its pieces 0..6 (`0x329528` = L00 `0x327498`, 3..6 / 3..6), the
//!   level's bolts (`0x3291d8` = L00 `0x327388`, 9), and it goes; without one the hit record is let go.
//!   Its words: `+0x00` / `+0x04` the bob's phase and offset, `+0x08` / `+0x0c` the sway's phases, then the motion's
//!   (`+0x10` the rise, `+0x14` its state, `+0x18` the timer, `+0x1c` the low height, `+0x20` / `+0x24` the rise's
//!   range, `+0x28` / `+0x2c` the wait's (seconds), `+0x30` / `+0x34` the rise's and the fall's time).
//! * **0x0937** ([`update_0937`], `0x3c91f8`, 25 placed; the decomp's `func_L17_003C7C70`): a platform that tips
//!   over when Ratchet stands on it. Every tick: a platform (`+0x8c` |= 1, `+0x3e` |= 8, mode 0x20); it bobs 0.1
//!   (`$gp+0x1378`) by its change (2 s, `$gp+0x1374`; `+0xa0` / `+0xa4`) and sways 0.03 (`$gp+0x137c`; 5 s, 6 s;
//!   `+0xa8` / `+0xac`).
//!   * **0**: its home `+0x90` is where it is; 1.
//!   * **1**: Ratchet on it ([`hero_on`], `0x327488`): 2, the shake timer `+0xb0` 2 s (`$gp+0x1388`).
//!   * **2** the shake: its sound 0 on its first tick and at 60 left; its pitch and roll wobble (sin / cos of the time
//!     left × 0.75 / 0.89, wrapped, as degrees), eased in from the sway while the fall timer `+0xb4` (from the last
//!     fall) is over 2 s less 30 ticks; at the end the fall timer 1 s (`$gp+0x138c`) and 3.
//!   * **3** the fall: it tips over its side 90° about the point 3 (`$gp+0x1394`) from its home along its yaw + 90°,
//!     dropping (8 by the end, `$gp+0x1398`); no platform (its flags and mode 0x20 off); at the end the rise timer
//!     `+0xb8` 1 s (`$gp+0x1390`) and 4.
//!   * **4** the rise: back up the other way round; at the end home, its angles as they were, 1.
//!
//!   Below 3 its move into its platform block `+0x50` (crate::moby::platform, `0x324660` = L00 `0x322668`).

use crate::hero::physics::{add_rot, dist2, sub};
use crate::moby::classes::props::{bob, sway};
use crate::moby::{blast, bolts, countdown, platform, shards, Ctx, MobyWorld};

pub const BUOY: i16 = 0x08af;
pub const TIPPER: i16 = 0x0937;

/// °/s as radians a tick.
const DEG_S: f32 = 0.000_290_888_25;
const HALF_PI: f32 = std::f32::consts::FRAC_PI_2;
/// `0x2f04e8` (L00 `0x2efef0`): `a` wrapped to [−π, π) by its fraction of a turn.
fn wrap(a: f32) -> f32 {
    let f = a * 0.159_154_94 + 0.5;
    ((f - f.floor()) - 0.5) * std::f32::consts::TAU
}
/// `0x3220a0` (L00 `0x320440`): `a` wrapped into −π..π (modf, the fraction keeping the sign).
fn wrap_f(a: f32) -> f32 { ((a + std::f32::consts::PI) * 0.159_154_94).fract() * 6.283_185_5 - std::f32::consts::PI }

fn tick_i(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let mut t = w.mobys[id].pv_i(off);
    let out = countdown(&mut t);
    w.mobys[id].set_pv_i(off, t);
    out
}

/// `0x327488` (L00 `0x325398`): Ratchet stands on moby `id` — hanging or climbing (group 3 or state 0x1b) from its
/// ledge (`0x18a3a8`), else on the ground (`0x18a16e` 0) on it (`0x18a15c`).
pub fn hero_on(ctx: &Ctx, id: usize) -> bool {
    let h = &*ctx.hero;
    if h.group == 3 || h.state == 0x1b { return h.ledge.moby == Some(id); }
    h.air_ticks == 0 && h.ground_moby == Some(id)
}

/// The offsets of a rise-and-fall motion's words.
pub struct Motion {
    pub rise: usize,
    pub state: usize,
    pub timer: usize,
    pub base: usize,
    pub min_wait: usize,
    pub max_wait: usize,
    pub rise_t: usize,
    pub fall_t: usize,
}

const BUOY_MOTION: Motion = Motion { rise: 0x10, state: 0x14, timer: 0x18, base: 0x1c, min_wait: 0x28, max_wait: 0x2c, rise_t: 0x30, fall_t: 0x34 };

/// A wait of the motion's range (`min..max` seconds, as ticks).
fn wait(m: &crate::moby::Moby, ctx: &mut Ctx, k: &Motion) -> i32 {
    let (lo, hi) = ((m.pv_f(k.min_wait) * 60.0) as i32, (m.pv_f(k.max_wait) * 60.0) as i32);
    ctx.game.rng.rand_range(lo, hi)
}

/// The motion's start, after its low height and its rise are set: one in two up at once (3), else low (1); a first
/// wait somewhere in one.
pub fn rise_start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: &Motion) {
    if ctx.game.rng.randi(2) != 0 {
        let m = &mut w.mobys[id];
        m.pos[2] += m.pv_f(k.rise);
        m.set_pv_i(k.state, 3);
    } else {
        w.mobys[id].set_pv_i(k.state, 1);
    }
    let t = wait(&w.mobys[id], ctx, k);
    let t = ctx.game.rng.rand_range(0, t);
    w.mobys[id].set_pv_i(k.timer, t);
}

/// The motion's states 1..4 (the buoys `0x3c5610` and the carriers `0x3c5b20`): **1** low until its wait is over,
/// then the rise (its time); **2** rising (half a cosine) to the top and a wait; **3** at the top until the wait is
/// over, then the fall (its time); **4** falling the same way, then a wait.
pub fn rise_fall(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: &Motion) {
    let s = w.mobys[id].pv_i(k.state);
    let m = &w.mobys[id];
    let (base, rise) = (m.pv_f(k.base), m.pv_f(k.rise));
    let wave = |t: f32| base + rise * (t * f32::from_bits(0x4049_0fdc)).cos() * 0.5 + rise * 0.5;
    match s {
        1 => {
            if tick_i(w, id, k.timer) {
                let m = &mut w.mobys[id];
                m.set_pv_i(k.timer, (m.pv_f(k.rise_t) * 60.0) as i32);
                m.set_pv_i(k.state, 2);
            }
            w.mobys[id].pos[2] = base;
        }
        2 => {
            if tick_i(w, id, k.timer) {
                w.mobys[id].pos[2] = base + rise;
                w.mobys[id].set_pv_i(k.state, 3);
                let t = wait(&w.mobys[id], ctx, k);
                w.mobys[id].set_pv_i(k.timer, t);
            } else {
                let m = &mut w.mobys[id];
                let r = m.pv_i(k.timer) as f32 / (m.pv_f(k.rise_t) * 60.0);
                m.pos[2] = wave(r);
            }
        }
        3 => {
            if tick_i(w, id, k.timer) {
                let m = &mut w.mobys[id];
                m.set_pv_i(k.timer, (m.pv_f(k.fall_t) * 60.0) as i32);
                m.set_pv_i(k.state, 4);
            }
            w.mobys[id].pos[2] = base + rise;
        }
        4 => {
            if tick_i(w, id, k.timer) {
                w.mobys[id].pos[2] = base;
                w.mobys[id].set_pv_i(k.state, 1);
                let t = wait(&w.mobys[id], ctx, k);
                w.mobys[id].set_pv_i(k.timer, t);
            } else {
                let m = &mut w.mobys[id];
                let r = m.pv_i(k.timer) as f32 / (m.pv_f(k.fall_t) * 60.0);
                m.pos[2] = wave(1.0 - r);
            }
        }
        _ => {}
    }
}

/// The white flash 15 ticks in 60 by the uid (else grey), on the glow word.
pub fn flash_glow(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let frame = ctx.game.frame;
    let m = &mut w.mobys[id];
    m.glow = if (frame + m.uid as i32).rem_euclid(60) < 15 { 0x80ff_ffff } else { 0x8040_4040 };
}

/// 0x08af (module docs).
pub fn update_08af(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    flash_glow(w, id, ctx);
    if w.mobys[id].pv_i(0x40) == 1 { return; }
    if w.mobys[id].pv_i(0x14) == 0 {
        let m = &mut w.mobys[id];
        let z = m.pos[2];
        m.set_pv_f(0x1c, z);
        let (lo, hi) = (m.pv_f(0x20), m.pv_f(0x24));
        let r = ctx.game.rng.randf(lo, hi);
        w.mobys[id].set_pv_f(0x10, r);
        let s = ctx.game.rng.randf(f32::from_bits(0x3b8e_fa37), f32::from_bits(0x3c56_7752));
        let s = if ctx.game.rng.randi(2) != 0 { -s } else { s };
        let m = &mut w.mobys[id];
        m.set_pv_f(0x38, s);
        m.set_pv_i(0x3c, -1);
        rise_start(w, id, ctx, &BUOY_MOTION);
    } else {
        rise_fall(w, id, ctx, &BUOY_MOTION);
    }
    let m = &mut w.mobys[id];
    let z = m.pos[2];
    bob(m, 5.0, 360.0 / 8.0 * DEG_S, z, 0, 4);
    sway(m, f32::from_bits(0x3d8f_5c29), 360.0 / 5.0 * DEG_S, 360.0 / 6.0 * DEG_S, 8, 0xc);
    m.rot[2] = add_rot(m.rot[2], m.pv_f(0x38));
    let h = m.pv_i(0x3c);
    if dist2(m.pos, ctx.hero.pos) < 80.0 {
        if h < 0 || !ctx.host.sound_playing(h) {
            let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
            w.mobys[id].set_pv_i(0x3c, h);
        }
    } else {
        if 0 <= h && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(0x3c, -1);
    }
    if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) {
        let pos = w.mobys[id].pos;
        blast::burst(w, ctx, pos, &blast::Blast { size: 10.0, bursts: [25, 20, 20], puffs: 20, streaks: 45, light: true, shake: true });
        shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x7f, [0.0; 3], 0xff);
        bolts::level_drop(w, id, 0.0, 9, ctx);
        w.delete(id);
    } else {
        w.clear_hit(id);
    }
}

/// The tipping point and drop of 0x0937's fall and rise (`f` the reach along its pitch, `drop` its drop's share).
fn tip(w: &mut MobyWorld, id: usize, f: f32, drop: f32) {
    let m = &mut w.mobys[id];
    let o = m.pv_v(0x90);
    let a = add_rot(HALF_PI, m.rot[2]);
    let s = m.rot[0].sin();
    m.pos = [a.cos() * f * s + o[0], a.sin() * f * s + o[1], o[2] - f * m.rot[0].cos() - (drop * HALF_PI).cos() * 8.0];
    m.set_pv_i(0x8c, m.pv_i(0x8c) & !1);
    m.set_pv_s(0x3e, m.pv_s(0x3e) & !8);
    m.mode &= !0x20;
}

/// 0x0937 (module docs).
pub fn update_0937(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    const SHAKE: f32 = 2.0;
    const FALL: f32 = 1.0;
    const RISE: f32 = 1.0;
    const REACH: f32 = 3.0;
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let m = &mut w.mobys[id];
    m.set_pv_i(0x8c, m.pv_i(0x8c) | 1);
    m.set_pv_s(0x3e, m.pv_s(0x3e) | 8);
    m.mode |= 0x20;
    bob(m, 0.1, 360.0 / 2.0 * DEG_S, -1.0, 0xa0, 0xa4);
    sway(m, 0.03, 360.0 / 5.0 * DEG_S, 360.0 / 6.0 * DEG_S, 0xa8, 0xac);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_v(0x90, old);
            m.state = 1;
        }
        1 => if hero_on(ctx, id) {
            let m = &mut w.mobys[id];
            m.set_state(2, -1);
            m.set_pv_i(0xb0, (SHAKE * 60.0) as i32);
        },
        2 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            if w.mobys[id].pv_i(0xb0) == 60 { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            if tick_i(w, id, 0xb0) {
                let m = &mut w.mobys[id];
                m.set_pv_i(0xb4, (FALL * 60.0) as i32);
                m.state = 3;
            } else {
                let m = &mut w.mobys[id];
                let b0 = m.pv_i(0xb0) as f32;
                let deg = f32::from_bits(0x3c8e_fa36);
                let r0 = wrap_f(b0 * 0.75).sin() * deg;
                let r1 = wrap_f(b0 * f32::from_bits(0x3f63_d70a)).cos() * deg;
                let span = SHAKE * 60.0 - 30.0;
                if span < m.pv_i(0xb4) as f32 {
                    let t = b0 / span;
                    m.rot[0] = m.rot[0] * t + r0 * (1.0 - t);
                    m.rot[1] = m.rot[1] * t + r1 * (1.0 - t);
                } else {
                    m.rot[1] = r1;
                    m.rot[0] = r0;
                }
            }
        }
        3 => {
            let m = &mut w.mobys[id];
            let t = m.pv_i(0xb4) as f32 / (FALL * 60.0);
            m.rot[1] = 0.0;
            m.rot[0] = (1.0 - t) * HALF_PI;
            tip(w, id, (1.0 - t) * REACH, t);
            if tick_i(w, id, 0xb4) {
                let m = &mut w.mobys[id];
                m.state = 4;
                m.set_pv_i(0xb8, (RISE * 60.0) as i32);
            }
        }
        4 => {
            let m = &mut w.mobys[id];
            let t = m.pv_i(0xb8) as f32 / (RISE * 60.0);
            let rot = m.rot;
            let c = 1.0 - t;
            m.rot[0] = wrap((rot[0] * c + c * f32::from_bits(0x4096_cbe4)) + HALF_PI);
            m.rot[1] = rot[1] * c;
            tip(w, id, (1.0 - c) * REACH, c);
            if tick_i(w, id, 0xb8) {
                let m = &mut w.mobys[id];
                m.state = 1;
                m.rot = rot;
                m.pos = m.pv_v(0x90);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.state < 3 {
        let (mv, rot) = (sub(m.pos, old), m.rot);
        platform::write(m, 0x50, mv, old_rot, rot);
    }
}
