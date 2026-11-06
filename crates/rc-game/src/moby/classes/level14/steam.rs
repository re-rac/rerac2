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

pub const BOBBER: i16 = 0x0af2;

/// 0x0af2 (`0x3e32e8`, 5 placed; the decomp's `func_L14_003E1DD0`): a spinner that bobs. **0**: its height kept
/// (`+4`), its phase 0 (`+8`); 1. **1**: its yaw on by `+0` a tick, its phase on by `+0xc`, its height the kept one
/// plus `+0x10` × the phase's sine.
pub fn update_0af2(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    use crate::hero::physics::add_rot;
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            let z = m.pos[2];
            m.set_pv_f(4, z);
            m.set_pv_f(8, 0.0);
            m.set_state(1, -1);
        }
        1 => {
            m.rot[2] = add_rot(m.rot[2], m.pv_f(0));
            let ph = add_rot(m.pv_f(8), m.pv_f(0xc));
            m.set_pv_f(8, ph);
            m.pos[2] = m.pv_f(4) + m.pv_f(0x10) * ph.sin();
        }
        _ => {}
    }
}

pub const SEQUENCER: i16 = 0x0b19;

/// 0x0b19's `$gp+0x1580..+0x158c`: the light's two scales and the two ramps' shapes.
const RAMP_SCALE: (f32, f32) = (0.05, 0.4);
const RAMP_SHAPE: (f32, f32) = (2.0, 8.0);

fn linked(w: &MobyWorld, m: usize, off: usize) -> Option<usize> {
    usize::try_from(w.mobys[m].pv_i(off)).ok().filter(|&i| w.mobys.get(i).is_some_and(|o| o.state < 0xfd))
}

fn rgb(m: &crate::moby::Moby, at: usize) -> u32 {
    (m.pv_s(at + 4) as u32) << 16 | (m.pv_s(at + 2) as u32) << 8 | m.pv_s(at) as u32
}

/// 0x0b19 (`0x3e60b8`, 25 placed; the decomp's `func_L14_003E4BA0`): the vents' sequencer. A ring of eight
/// thresholds (`+0x00..+0x0e`, s16, falling) is walked by its clock (`+0x14`, counting down from the period `+0x34`
/// and synced across its group). The index (`+0x16`) moves past every threshold the clock is under. At an even index
/// it is in state 1, at an odd one in state 2.
///
/// * **0**: it hands its colours (`+0x40..+0x4a`) to the other 0x0b19s of its group.
/// * **1**: on the first tick its light (`+0x24`) is lit with the colours. Each tick the light's two levels
///   (`+0x40`, `+0x44`) ramp over the stretch to the next threshold.
/// * **2**: on the first tick its two vents (`+0x1c`, `+0x20`, 0x0b0d) are set off. Their counts come from this
///   stretch (`+0x14`, `+0x18`), their colours from its own, and their trigger is 1.0. Its light goes out.
pub fn update_0b19(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let _ = ctx;
    let group = w.mobys[id].group;
    let members: Vec<usize> = usize::try_from(group).ok().and_then(|g| w.groups.get(g)).map_or(Vec::new(), |g| {
        g.iter().map(|&i| i as usize).filter(|&i| w.mobys.get(i).is_some_and(|o| o.o_class == SEQUENCER)).collect()
    });
    match w.mobys[id].state {
        0 => {
            let colours: Vec<i16> = (0..6).map(|k| w.mobys[id].pv_s(0x40 + 2 * k)).collect();
            for &o in &members { for k in 0..6 { w.mobys[o].set_pv_s(0x40 + 2 * k, colours[k]); } }
        }
        1 => {
            if w.mobys[id].first() {
                if let Some(l) = linked(w, id, 0x24) {
                    let (a, b, period) = (rgb(&w.mobys[id], 0x40), rgb(&w.mobys[id], 0x46), w.mobys[id].pv_i(0x18));
                    let q = &mut w.mobys[l];
                    q.set_pv_i(0x48, a as i32);
                    q.set_pv_i(0x4c, b as i32);
                    q.set_pv_i(0x3c, 1);
                    q.set_pv_i(0x34, period);
                }
            }
            ramp(w, id);
        }
        2 => if w.mobys[id].first() {
            let m = &w.mobys[id];
            let k = m.pv_s(0x16) as usize & 7;
            if k % 2 == 1 {
                let (a, b) = (m.pv_s(2 * k) as i32, m.pv_s(2 * ((k + 1) % 8)) as i32);
                let cur = m.pv_s(0x14) as i32;
                let c0 = 0x8000_0000 | rgb(m, 0x40);
                let c1 = 0x8000_0000 | rgb(m, 0x46);
                for off in [0x1c, 0x20] {
                    let Some(v) = linked(w, id, off) else { continue };
                    let q = &mut w.mobys[v];
                    q.set_pv_i(0x14, cur - a);
                    q.set_pv_i(0x18, a - b);
                    for (at, c) in [(0x30, c0), (0x38, c0), (0x34, c1), (0x3c, c1)] { q.set_pv_i(at, c as i32); }
                    q.set_pv_f(0xc, 1.0);
                }
            }
            if let Some(l) = linked(w, id, 0x24) { w.mobys[l].set_pv_i(0x3c, 0); }
        },
        _ => {}
    }
    let period = w.mobys[id].pv_i(0x34);
    if period != 0 {
        let mut t = w.mobys[id].pv_s(0x14) as i32;
        if crate::moby::countdown(&mut t) {
            t = period;
            for &o in &members { w.mobys[o].set_pv_s(0x16, 0); }
        }
        w.mobys[id].set_pv_s(0x14, t as i16);
        for &o in &members { w.mobys[o].set_pv_s(0x14, t as i16); }
    }
    let m = &mut w.mobys[id];
    let mut k = m.pv_s(0x16) as usize & 7;
    for _ in 0..8 {
        if m.pv_s(0x14) >= m.pv_s(2 * k) { break; }
        k = (k + 1) % 8;
    }
    m.set_pv_s(0x16, k as i16);
    let want = if k % 2 == 0 { 1 } else { 2 };
    if m.state != want { m.set_state(want, -1); }
}

/// `0x3e63b8`: the light's two levels by where the clock is between this threshold and the one before (the period
/// at index 0), each `(shape − |shape − 1 − (2·shape − 2)·f|)·scale`.
fn ramp(w: &mut MobyWorld, id: usize) {
    let Some(l) = linked(w, id, 0x24) else { return };
    let m = &w.mobys[id];
    let k = m.pv_s(0x16) as usize & 7;
    let cur = m.pv_s(0x14) as f32;
    let before = if k == 0 {
        if 0 < m.pv_i(0x34) { m.pv_i(0x34) } else { m.pv_s(0x14) as i32 }
    } else {
        m.pv_s(2 * (k - 1)) as i32
    };
    let at = m.pv_s(2 * k) as f32;
    let d = before as f32 - at;
    if d == 0.0 { return; }
    let f = (cur - at) / d;
    let level = |s: f32| s - ((s - 1.0) - (2.0 * s - 2.0) * f).abs();
    let (a, b) = (level(RAMP_SHAPE.0) * RAMP_SCALE.0, level(RAMP_SHAPE.1) * RAMP_SCALE.1);
    let q = &mut w.mobys[l];
    q.set_pv_f(0x40, a);
    q.set_pv_f(0x44, b);
}
