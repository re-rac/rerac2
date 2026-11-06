//! 0x0b0d (`0x3e5728`, 72 placed; the decomp's `func_L14_003E4208`): a steam vent another moby sets off by writing
//! 1.0 to its trigger (`+0x0c`).
//!
//! * **0**: the emitter's settings — the rate 2 → 30 a second over 100 puffs, puffs living 60 ticks, the spread
//!   (0.005, 0.01, 0.002), the turn 4, the sizes 0.5..1 → 1..2 (×210000), the colours 0x30505000 / 0x80508000 to
//!   0x70700050 / 0x80800070; then 1.
//! * **1**: waits for the trigger, then 2.
//! * **2**: its first tick restarts the count. Its loop sound stops. The puffs come at the rising rate:
//!   * each starts 1.1 along its first row;
//!   * each moves by the spread a tick (twice the spread, and 1.5 times the life, in the last third);
//!   * each has a colour mixed toward the second pair as the count runs out.
//!   When the count is out, 3. Set off again, it restarts.
//! * **3**: its loop sound 0 (mode 4) is kept playing, and the puffs come at the full rate with the wide spread
//!   until the trigger is set again, then 2.
//!
//! The puffs are cinders (type 0x5b, `0x33fbb8`): no fall, def 2.

use crate::hero::physics::V3;
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0b0d;

mod pv {
    pub const RATE: usize = 0x00;
    pub const RATE_END: usize = 0x04;
    pub const ACCUM: usize = 0x08;
    pub const TRIGGER: usize = 0x0c;
    pub const LIFE: usize = 0x10;
    pub const TOTAL: usize = 0x14;
    pub const RESET: usize = 0x18;
    pub const LEFT: usize = 0x1c;
    pub const SPREAD: usize = 0x20;
    pub const TURN: usize = 0x2c;
    pub const COLOUR: usize = 0x30;
    pub const SIZE_MIN: usize = 0x40;
    pub const SIZE_MAX: usize = 0x48;
    pub const SOUND: usize = 0x50;
}

pub fn update_0b0d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.set_pv_f(pv::TRIGGER, 0.0);
            m.set_pv_f(pv::RATE, 2.0);
            m.set_pv_f(pv::RATE_END, 30.0);
            m.set_pv_i(pv::LIFE, 60);
            m.set_pv_i(pv::TOTAL, 100);
            m.set_pv_i(pv::RESET, 100);
            for (k, v) in [0.005, 0.01, 0.002].into_iter().enumerate() { m.set_pv_f(pv::SPREAD + 4 * k, v); }
            m.set_pv_f(pv::TURN, 4.0);
            for (k, v) in [105_000.0, 210_000.0].into_iter().enumerate() { m.set_pv_f(pv::SIZE_MIN + 4 * k, v); }
            for (k, v) in [210_000.0, 420_000.0].into_iter().enumerate() { m.set_pv_f(pv::SIZE_MAX + 4 * k, v); }
            for (k, v) in [0x3050_5000u32, 0x7070_0050, 0x8050_8000, 0x8080_0070].into_iter().enumerate() { m.set_pv_i(pv::COLOUR + 4 * k, v as i32); }
            m.set_pv_i(pv::SOUND, -1);
            m.set_state(1, -1);
        }
        1 => if m.pv_f(pv::TRIGGER) == 1.0 {
            m.set_pv_f(pv::TRIGGER, 0.0);
            m.set_state(2, -1);
        },
        2 => {
            if m.pv_f(pv::TRIGGER) == 1.0 {
                m.set_pv_f(pv::TRIGGER, 0.0);
                m.set_state(2, -1);
            }
            if m.first() {
                m.set_pv_f(pv::ACCUM, 0.0);
                let t = m.pv_i(pv::TOTAL);
                m.set_pv_i(pv::LEFT, t);
            }
            let h = m.pv_i(pv::SOUND);
            if 0 <= h && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
            w.mobys[id].set_pv_i(pv::SOUND, -1);
            let m = &mut w.mobys[id];
            let mut left = m.pv_i(pv::LEFT);
            let out = countdown(&mut left);
            m.set_pv_i(pv::LEFT, left);
            if out { return m.set_state(3, -1); }
            let total = m.pv_i(pv::TOTAL) as f32;
            let step = (m.pv_f(pv::RATE_END) - m.pv_f(pv::RATE)) / total;
            let rate = (m.pv_f(pv::RATE) + step * ((total - left as f32) - 0.008_333_334)) / 60.0;
            puffs(w, id, ctx, rate, false);
        }
        3 => {
            let _ = m.first();
            let h = m.pv_i(pv::SOUND);
            if h < 0 || !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(pv::SOUND, h);
            }
            let m = &mut w.mobys[id];
            if m.pv_f(pv::TRIGGER) != 0.0 {
                m.set_pv_f(pv::TRIGGER, 0.0);
                return m.set_state(2, -1);
            }
            let total = m.pv_i(pv::TOTAL) as f32;
            let step = (m.pv_f(pv::RATE_END) - m.pv_f(pv::RATE)) / total;
            let rate = (m.pv_f(pv::RATE) + step * (total - 0.008_333_334)) / 60.0;
            puffs(w, id, ctx, rate, true);
        }
        _ => {}
    }
}

/// The puffs this tick owes at `rate` a tick; `full`: state 3's (the second colours, the wide spread, no turn).
fn puffs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, rate: f32, full: bool) {
    let m = &mut w.mobys[id];
    let mut acc = m.pv_f(pv::ACCUM) + rate;
    let r = m.rows;
    let through = |v: V3| -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) };
    let at = crate::hero::physics::add(m.pos, through([1.1, 0.0, 0.0]));
    let total = m.pv_i(pv::TOTAL);
    let left = m.pv_i(pv::LEFT);
    let f = left as f32 / total as f32;
    let c: [u32; 4] = std::array::from_fn(|k| m.pv_i(pv::COLOUR + 4 * k) as u32);
    let spread: [f32; 3] = std::array::from_fn(|k| m.pv_f(pv::SPREAD + 4 * k));
    let (smin, smax) = ([m.pv_f(pv::SIZE_MIN), m.pv_f(pv::SIZE_MIN + 4)], [m.pv_f(pv::SIZE_MAX), m.pv_f(pv::SIZE_MAX + 4)]);
    let (life, turn) = (m.pv_i(pv::LIFE), m.pv_f(pv::TURN));
    let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;
    let Some(sys) = ctx.parts.as_deref_mut() else { m.set_pv_f(pv::ACCUM, acc.fract()); return };
    let g = &mut ctx.game.rng;
    while 1.0 < acc {
        let late = full || left < total / 3;
        let colour = if full {
            crate::hud::lerp_rgba(g.randf(0.0, 1.0), c[1], c[3])
        } else {
            let a = crate::hud::lerp_rgba(g.randf(0.0, 1.0), c[0], c[2]);
            let b = crate::hud::lerp_rgba(g.randf(0.0, 1.0), c[1], c[3]);
            crate::hud::lerp_rgba(f, b, a)
        };
        let mut size = g.randf(lerp(smax[0], smin[0], f), lerp(smax[1], smin[1], f));
        let k = if late { 2.0 } else { 1.0 };
        let v = [g.randf(0.0, spread[0] * k), g.randf(0.0, spread[1] * k), g.randf(-spread[2], 0.0)];
        let roll = if late { size *= 1.5; 0.0 } else { turn };
        crate::particles::cinder::cinder(sys, g, size / 210_000.0, roll, at, through(v), colour, colour & 0xff_ffff, life, 0, Some(2), None, 0);
        acc -= 1.0;
    }
    w.mobys[id].set_pv_f(pv::ACCUM, acc);
}
