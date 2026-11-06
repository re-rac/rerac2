//! The prison's props (L14 addresses).
//!
//! * **0x0b11** ([`update_0b11`], `0x3e5f58`, 1 placed; the decomp's `func_L14_003E4A40`): a hatch that tips down
//!   once hooked.
//!   * Its pvar `+0x00` holds its looped sound.
//!   * **0**: no sound, 1.
//!   * **1**: hooked (`0x3336b8` = L00 `0x32a838`) → 2.
//!   * **2**: while its pitch (`+0xf0`) is above −90°, its sound 0 looped (`0x36ada0(0, 4)`) and the pitch 0.5° down
//!     (`0x2f7ab0`, the wrapped difference). Then, every tick, the sound is stopped again (`0x36aaa0`); the original
//!     does this too, so the loop is restarted and cut on each tick it turns.
//! * **0x0ae7** ([`update_0ae7`], `0x3dfe88`, 6 placed; the decomp's `func_US2_L14_003DFE88`): a flap that swings
//!   open and sets off the moby it carries.
//!   * Its pvars:
//!     * `+0x00` the carried moby (−1 none);
//!     * `+0x04` the swing a tick;
//!     * `+0x08` the axis (≠ 0 yaw `+0xf4` down to −180°, else roll `+0xf8` up to 180°);
//!     * `+0x0c` a cuboid (−1: Ratchet within 10 instead);
//!     * `+0x10` the carried moby's record, `+0x14` a tick count, `+0x18` the pulse, `+0x1c` the start angle.
//!   * **0** (set-up):
//!     * With a carried moby: glow 0x80404040, mode | 0x10, the count 0.
//!     * Its start angle kept, then 1.
//!   * **1**: the glow pulses grey. Ratchet in the cuboid (`0x329290` = L00 `0x321858`), or with none within 10 of it
//!     (`0x2f6be0` = L00 `0x2ef018`), → 2.
//!   * **2**: the glow's red and green fade with the angle. At the end of the swing → 3. Else the angle moves by the
//!     swing; the carried moby turns with it, about its point 0.5 under it (crate::moby::attach::pivot_turn), its
//!     rows rebuilt (`0x2f7280`).
//!   * **3**: on its first tick the roll is set to 180° and the carried moby hooked (`0x333698`, `+0xbe` | 4); its
//!     glow the pulse in blue.
//!   * **Every tick**: the count goes up, and the pulse is 80 + 48·sin(2π·(count mod 150)/150) (`0x2f7028`).

use crate::moby::attach::{pivot_turn, rows_of};
use crate::moby::{Ctx, MobyWorld};
use std::f32::consts::{FRAC_PI_2, PI};

pub const HATCH: i16 = 0x0b11;
pub const FLAP: i16 = 0x0ae7;

/// `0x2f7ab0`: `a − b` wrapped into −π..π.
fn wrapped_sub(a: f32, b: f32) -> f32 {
    let d = a - b;
    if PI <= d { d - PI - PI } else if d < -PI { d + PI + PI } else { d }
}

/// Its class sound `k` looped into pvar `at` unless it plays (`0x36ada0(k, 4)`).
fn loop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: i32, at: usize) {
    let h = w.mobys[id].pv_i(at);
    if !(h != -1 && ctx.host.sound_playing(h)) {
        let h = ctx.host.play_sound(&w.mobys[id], k, 4);
        w.mobys[id].set_pv_i(at, h);
    }
}

/// The loop in pvar `at` stopped when it plays (`0x36aaa0`), −1.
fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: usize) {
    let h = w.mobys[id].pv_i(at);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(at, -1);
}

pub fn update_0b11(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            w.mobys[id].set_state(1, -1);
            w.mobys[id].set_pv_i(0, -1);
        }
        1 => if w.mobys[id].once & 4 != 0 { w.mobys[id].set_state(2, -1) },
        2 => {
            if -FRAC_PI_2 < w.mobys[id].rot[0] {
                loop_sound(w, id, ctx, 0, 0);
                let m = &mut w.mobys[id];
                m.rot[0] = wrapped_sub(m.rot[0], 0.008_726_647);
            }
            stop_sound(w, id, ctx, 0);
        }
        _ => {}
    }
}

/// A grey (or blue-tinted) glow word from its channels.
fn glow(r: i32, g: i32, b: i32) -> u32 { 0x8000_0000 | (r as u32) << 16 | (g as u32) << 8 | b as u32 }

pub fn update_0ae7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let carried = |w: &MobyWorld, id: usize| usize::try_from(w.mobys[id].pv_i(0x10)).ok().filter(|&c| c < w.mobys.len());
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            if 0 <= m.pv_i(0) {
                m.set_pv_i(0x10, m.pv_i(0));
                m.glow = 0x8040_4040;
                m.mode |= 0x10;
                m.set_pv_i(0x14, 0);
            } else {
                m.set_pv_i(0x10, -1);
            }
            let start = if m.pv_i(8) != 0 { m.rot[1] } else { m.rot[2] };
            m.set_pv_f(0x1c, start);
            m.set_state(1, -1);
        }
        1 => {
            let m = &mut w.mobys[id];
            let c = m.pv_i(0x18);
            m.glow = glow(c, c, c);
            let go = match m.pv_i(0xc) {
                -1 => crate::hero::physics::dist(ctx.hero.pos, m.pos) < 10.0,
                k => ctx.level.in_cuboid(k, ctx.hero.pos),
            };
            if go { m.set_state(2, -1); }
        }
        2 => {
            let m = &mut w.mobys[id];
            let (pulse, yaw) = (m.pv_i(0x18), m.pv_i(8) != 0);
            let t = if yaw { (pulse as f32 * (m.rot[1] + PI) * 0.318_309_87) as i32 } else { (pulse as f32 * (PI - m.rot[2]) * 0.318_309_87) as i32 };
            m.glow = glow(t, t, pulse);
            if (PI <= m.rot[2] && !yaw) || (m.rot[1] <= -PI && yaw) {
                m.set_state(3, -1);
            } else {
                let step = m.pv_f(4);
                if yaw { m.rot[1] -= step } else { m.rot[2] += step }
                if let Some(c) = carried(w, id) {
                    let o = &mut w.mobys[c];
                    pivot_turn(o, [step, 0.0, 0.0], [0.0, 0.0, -0.5]);
                    o.rows = rows_of(o.rot);
                }
            }
        }
        3 => {
            if w.mobys[id].first() {
                w.mobys[id].rot[2] = PI;
                if let Some(c) = carried(w, id) { w.mobys[c].once |= 4; }
            }
            let m = &mut w.mobys[id];
            m.glow = m.pv_i(0x18) as u32 | 0x8000_0000;
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    let n = m.pv_i(0x14).wrapping_add(1);
    m.set_pv_i(0x14, n);
    m.set_pv_i(0x18, (((n % 150) as f32 * 0.041_887_905).sin() * 48.0) as i32 + 80);
}
