//! Level 14's energy field **0x1164** (`0x4319f0`, 1 placed; the decomp's `func_L14_00430410`, its draw
//! `func_L14_00430B70` = `0x432150`): a glowing ball of camera-facing quads with rings and rays about it, humming,
//! that shrinks away once hooked. Its model is not drawn. Its pvars: `+0x00` the core's turn, `+0x04` / `+0x08` /
//! `+0x0c` the core's FX, size and colour, `+0x10` the rings' ticks, `+0x14` their colour's split, `+0x18` their size,
//! `+0x1c` their FX, `+0x20..` their rates, `+0x30..` / `+0x40..` their start and end colours, `+0x50..` their timers,
//! `+0x60..` their turns, `+0x80` its size now, `+0x84` its size, `+0x88` its pulse, `+0x90` the pulse's ticks,
//! `+0x94` / `+0x98` the rays' start and end colours, `+0xa0` / `+0xa4` their end and start shortening, `+0xa8` their
//! ticks, `+0xac` their colour's split, `+0xb0` their FX, `+0xb4` their length, `+0xb8..+0xc0` the halo's FX, size and
//! colour, `+0xc4` its loop sound.
//!
//! * **0**: those values set (the instance gives the size, pulse and pulse ticks), the 32 rays' turns `randf(−π, π)`
//!   and their phases `k / 32` (`0x260270` / `0x2602f0`, the level's), not drawn (`+0x31` 0), mode | 1, no sound; 1.
//! * **1**: its class sound 0 kept looping (`0x36ada0(0, 4)`); its draw ([`draw`]) registered (`0x2ef2a0` = L00
//!   `0x2e7920`); hooked (`0x3336b8` = L00 `0x32a838`): within the level's first 50 frames (`0x1b2048`) its size 0;
//!   2.
//! * **2**: the sound stopped (`0x36aaa0`); on its first tick no collision (`+0x98` 0); while its size is above 0.5
//!   it is drawn and shrinks by 0.05 a tick.
//! * **Every tick**: its size now `size + pulse·sin(wrap(frame·2π / pulse ticks))` (`0x431e78`: `2π·fract(x / 2π)`).

use crate::fx::FxQuad;
use crate::hero::physics::{add, cross, scale, set_len, sub, unit, V3};
use crate::moby::{countdown, Ctx, MobyWorld};
use std::f32::consts::{FRAC_PI_2, PI, TAU};

pub const CLASS: i16 = 0x1164;

const SIZE_NOW: usize = 0x80;
const SIZE: usize = 0x84;
const PULSE: usize = 0x88;
const PULSE_TICKS: usize = 0x90;
const SOUND: usize = 0xc4;

/// The rays' turns and phases (`0x260270` / `0x2602f0`: the level's, one set for the class).
#[derive(Clone, Debug, Default)]
pub struct Globals {
    pub turn: [f32; 32],
    pub phase: [f32; 32],
}

/// `0x4319f0` (module docs).
pub fn update_1164(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let words: &[(usize, u32)] = &[
                (0x00, 0), (0x04, 69), (0x08, 1.2f32.to_bits()), (0x0c, 0x5000_0080),
                (0x20, 2.0f32.to_bits()), (0x24, 4.2f32.to_bits()), (0x28, 3.1f32.to_bits()), (0x2c, 3.7f32.to_bits()),
                (0x30, 0x8000_0080), (0x34, 0x8000_0080), (0x38, 0x8020_0060), (0x3c, 0x8080_8080),
                (0x40, 0x8020_0060), (0x44, 0x8030_0050), (0x48, 0x8020_2880), (0x4c, 0x8060_6020),
                (0x50, 0), (0x54, 50), (0x10, 300),
                (0x60, (-0.125f32).to_bits()), (0x64, 0.133f32.to_bits()), (0x68, 0.241f32.to_bits()), (0x6c, (-0.221f32).to_bits()),
                (0x18, 1.2f32.to_bits()), (0x70, 0.6f32.to_bits()), (0x74, 0.5f32.to_bits()), (0x78, 0.4f32.to_bits()), (0x7c, 0.7f32.to_bits()),
                (0x1c, 54), (0x94, 0x2828_0080), (0x98, 0x0a80_0000), (0xa0, 0.9f32.to_bits()), (0xa4, 0.1f32.to_bits()),
                (0xa8, 20), (0xac, 0.8f32.to_bits()), (0x14, 0.8f32.to_bits()), (0xb0, 70), (0xb4, 2.0f32.to_bits()), (0x9c, 2.0f32.to_bits()),
                (0xb8, 71), (0xbc, 1.0f32.to_bits()), (0xc0, 0x8080_0000), (SOUND, u32::MAX),
            ];
            for &(off, v) in words { m.set_pv_i(off, v as i32); }
            for k in 0..32 {
                w.energy14.turn[k] = ctx.game.rng.randf(-PI, PI);
                w.energy14.phase[k] = k as f32 * 0.031_25;
            }
            let m = &mut w.mobys[id];
            m.drawn = 0;
            m.mode |= 1;
            m.set_state(1, -1);
        }
        1 => {
            let h = w.mobys[id].pv_i(SOUND);
            if !(h != -1 && ctx.host.sound_playing(h)) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(SOUND, h);
            }
            w.register_draw(id);
            if w.mobys[id].once & 4 != 0 {
                if ctx.game.frame < 50 { w.mobys[id].set_pv_f(SIZE, 0.0); }
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            let h = w.mobys[id].pv_i(SOUND);
            if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
            let m = &mut w.mobys[id];
            m.set_pv_i(SOUND, -1);
            if m.first() { m.has_collision = false; }
            if 0.5 < m.pv_f(SIZE) {
                m.set_pv_f(SIZE, m.pv_f(SIZE) - 0.05);
                w.register_draw(id);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    let x = ctx.game.frame as f32 * TAU / m.pv_i(PULSE_TICKS) as f32;
    let p = x * 0.159_154_94;
    m.set_pv_f(SIZE_NOW, m.pv_f(SIZE) + m.pv_f(PULSE) * ((p - p.trunc()) * TAU).sin());
}

/// `0x431eb0(at, fx, rgba, 1, size, push, turn)`: a square facing the camera (`0x1b9480`) — its rows the way to the
/// camera `f`, `r = f × down`, `u = r × f`, at `at` pushed `push` toward the camera, the corners (0, ∓1, ±1) of
/// `0x1b1960` × size — its STs turned by `turn` about (0.5, 0.5); FX `fx` additive (`0x2f4aa8(fx, 1, 0x80)`), under
/// TEST 0x53001 (z GEQUAL, no z write).
#[allow(clippy::too_many_arguments)]
fn quad(out: &mut Vec<FxQuad>, at: V3, cam: V3, fx: i32, rgba: u32, size: f32, push: f32, turn: f32) {
    let f = unit(sub(cam, at));
    let r = unit(cross(f, crate::particles::GRAVITY));
    let u = cross(r, f);
    let c = add(at, scale(f, push));
    let corner = |y: f32, z: f32| add(c, add(scale(r, y * size), scale(u, z * size)));
    let (co, si) = (turn.cos(), turn.sin());
    let st = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]].map(|[s, t]: [f32; 2]| {
        let (x, y) = (s - 0.5, t - 0.5);
        [x * co - y * si + 0.5, x * si + y * co + 0.5]
    });
    out.push(FxQuad { fx: fx.max(0) as usize, additive: true, corners: [corner(-1.0, 1.0), corner(-1.0, -1.0), corner(1.0, 1.0), corner(1.0, -1.0)], st, rgba: [rgba; 4] });
}

/// `0x432150` (a second-list callback; module docs): the halo ([`quad`]: FX `+0xb8`, `+0xc0`, `+0xbc` × its size now,
/// pushed half its size now); the two rings — each's timer out (`0x2f6660`): the timer `⌊+0x10 / rate⌋` again;
/// `t = 1 − timer / ⌊+0x10 / rate⌋`; past the split `+0x14` its end colour at alpha `⌊(1 − t)/(1 − split)·80⌋ + 48`,
/// else its start colour lerped to its end (`0x2f7c08`) by `t / split` at alpha `⌊t / split·80⌋ + 48`; FX `+0x1c`,
/// `+0x18` × its size now, turned `wrap(t·turn·2π) − π`; the core turned 0.007 more (`0x2fba30`), FX `+0x04` in
/// `+0x0c`, `+0x08` × its size now; then the rays ([`rays`]).
pub fn draw(w: &mut MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &mut w.mobys[id];
    let (at, now) = (m.pos, m.pv_f(SIZE_NOW));
    quad(out, at, cam, m.pv_i(0xb8), m.pv_i(0xc0) as u32, m.pv_f(0xbc) * now, now * 0.5, 0.0);
    let split = m.pv_f(0x14);
    for i in 0..2 {
        let ticks = (m.pv_i(0x10) as f32 / m.pv_f(0x20 + 4 * i)) as i32;
        let mut timer = m.pv_i(0x50 + 4 * i);
        if countdown(&mut timer) { timer = ticks; }
        m.set_pv_i(0x50 + 4 * i, timer);
        let t = 1.0 - timer as f32 / ticks as f32;
        let (start, end) = (m.pv_i(0x30 + 4 * i) as u32, m.pv_i(0x40 + 4 * i) as u32);
        let colour = if split < t {
            (end & 0xff_ffff).wrapping_add((((1.0 - t) / (1.0 - split) * 80.0) as i32 + 48).wrapping_shl(24) as u32)
        } else {
            let c = crate::hud::lerp_rgba(t / split, start, end);
            (c & 0xff_ffff).wrapping_add((((t / split * 80.0) as i32) + 48).wrapping_shl(24) as u32)
        };
        let p = t * (m.pv_f(0x60 + 4 * i) * TAU) * 0.159_154_94;
        let turn = (p - p.trunc()) * TAU;
        quad(out, at, cam, m.pv_i(0x1c), colour, now * m.pv_f(0x18), now * 0.5, turn - PI);
    }
    m.set_pv_f(0, crate::hero::physics::add_rot(m.pv_f(0), 0.007));
    quad(out, at, cam, m.pv_i(4), m.pv_i(0xc) as u32, m.pv_f(8) * now, now * 0.5, m.pv_f(0));
    rays(w, id, rng, cam, out);
}

/// `0x4323e8`: each of the 32 rays' phases on by `1 / +0xa8` (past 1: 0, its turn `randf(−π/2, π/2)`); then each ray
/// a quad of FX `+0xb0` (additive) out of its place `+ r` (`r` the way to the camera, half its size now long), along
/// `d = r × r'` (`r'` = `r` turned its turn about up, `0x329638`) `+0xb4·(1 − phase)` long: corners 0 / 1 at `−d`,
/// 2 / 3 at `+d`, each pushed along `r'` — the even ones its size now × (1 − k), the odd ones its size now
/// (k = `+0xa4 + phase·(+0xa0 − +0xa4)`, at most 1); its colour past the split `+0xac` the end colour `+0x98`, else
/// `+0x94` lerped to it by phase / split, the odd corners in `+0x0c` (the packet keeps them for the next ray, whose
/// four are set again).
fn rays(w: &mut MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let g = &mut w.energy14;
    for k in 0..32 {
        g.phase[k] += 1.0 / m.pv_i(0xa8) as f32;
        if 1.0 < g.phase[k] {
            g.phase[k] = 0.0;
            g.turn[k] = rng.randf(-FRAC_PI_2, FRAC_PI_2);
        }
    }
    let now = m.pv_f(SIZE_NOW);
    let r = set_len(sub(cam, m.pos), now * 0.5);
    let up = [0.0, 0.0, 1.0];
    let (split, c0, c1, tip) = (m.pv_f(0xac), m.pv_i(0x94) as u32, m.pv_i(0x98) as u32, m.pv_i(0xc) as u32);
    for k in 0..32 {
        let ph = g.phase[k];
        let colour = if split < ph { c1 } else { crate::hud::lerp_rgba(ph / split, c0, c1) };
        let turned = crate::camera::rot(r, g.turn[k], up);
        let d = set_len(cross(r, turned), m.pv_f(0xb4) * (1.0 - ph));
        let short = (m.pv_f(0xa4) + ph * (m.pv_f(0xa0) - m.pv_f(0xa4))).min(1.0);
        let mut rgba = [colour; 4];
        let corners: [V3; 4] = std::array::from_fn(|j| {
            let p = if 2 <= j { add(m.pos, d) } else { sub(m.pos, d) };
            let reach = if j & 1 == 0 { now * (1.0 - short) } else { now };
            add(add(p, set_len(turned, reach)), r)
        });
        rgba[1] = tip;
        rgba[3] = tip;
        out.push(FxQuad { fx: m.pv_i(0xb0).max(0) as usize, additive: true, corners, st: [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]], rgba });
    }
}
