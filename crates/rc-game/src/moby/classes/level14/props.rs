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
pub const BREAKABLE: i16 = 0x1108;
pub const PAD: i16 = 0x0d60;

/// 0x0d60 (`0x40b080`, 2 placed; the decomp's `func_L14_00409B20`): a pad that, stood on, sets off its moby. **0**:
/// Ratchet on it (`0x32cea8` = L00 `0x325398`, crate::moby::classes::level17::floaters::hero_on): the moby `+0x00`
/// (−1 none) hooked (`0x333698` = L00 `0x32a818`), 1. **1**: on its first tick its class sound 0 (`0x36ada0`).
/// **Every tick** its phase `+0x04` turns 6.17° (`0x2f7a68`, wrapped); its glow (`+0x78`) eased 0.15 of the way
/// (`0x2f7c08` = boot `0x2846e8`, crate::gui::lerp_colour) to the colour between 0x808c8c8c and 0x80282828 (in 1
/// 0x80468cdc and 0x801e78aa) by `sin(phase)·0.5 + 0.5` (`0x2f7028`).
pub fn update_0d60(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (mut a, mut b) = (0x808c_8c8cu32, 0x8028_2828u32);
    match w.mobys[id].state {
        0 => if crate::moby::classes::level17::floaters::hero_on(ctx, id) {
            let k = w.mobys[id].pv_i(0);
            if let Some(t) = usize::try_from(k).ok().and_then(|k| w.mobys.get_mut(k)) { t.once |= 4; }
            w.mobys[id].set_state(1, -1);
        },
        1 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            (a, b) = (0x8046_8cdc, 0x801e_78aa);
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    let f = crate::hero::physics::add_rot(m.pv_f(4), 0.107_628_65);
    m.set_pv_f(4, f);
    let to = u32::from_le_bytes(crate::gui::lerp_colour(f.sin() * 0.5 + 0.5, a, b));
    m.glow = u32::from_le_bytes(crate::gui::lerp_colour(0.15, m.glow, to));
}

/// 0x1108 (`0x428a38`, 21 placed; the decomp's `func_L14_00427458`): a breakable. **0**: 1 (`0x323250`). **1**: a hit
/// with the flags 0x4010000 (`0x325680`) while its pvar `+0x00` is 0 breaks it: its pieces thrown (`0x334460` = L00
/// `0x32b580`, crate::moby::shards::throw_all: from the hit's way 8 long − (3, 3, 0) to 18 long + (3, 3, 0) a second (`0x2f6d00` = L00 `0x2ef138`), not
/// through its rows (flag 8); spin 90..360 and gravity 10..20 (`$gp+0x27e8` / `+0x27e0`), every piece pointed, no
/// skip (`$gp+0x2810` −1), the bolts' sound 0), its class sound 0 (`0x36ada0(0, 0)`), 2; the hit let go (`+0xa8`
/// −1) either way. **2**: deleted (`0x319058`).
pub fn update_1108(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::{add, set_len, sub};
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x0401_0000 != 0);
            if let Some(h) = hit {
                if w.mobys[id].pv_i(0) == 0 {
                    let dir = [h.tmpl.dir[0], h.tmpl.dir[1], h.tmpl.dir[2]];
                    let off = [3.0, 3.0, 0.0];
                    let lo = sub(set_len(dir, 8.0), off);
                    let hi = add(set_len(dir, 18.0), off);
                    crate::moby::shards::throw_all(w, id, ctx, [lo[0], lo[1], lo[2], 0.0, hi[0], hi[1], hi[2], 0.0], (90.0, 360.0), (10.0, 20.0), 0xffff, 0, -1, 8, 0xffff);
                    ctx.host.play_sound(&w.mobys[id], 0, 0);
                    w.mobys[id].set_state(2, -1);
                }
            }
            w.mobys[id].hit_slot = -1;
        }
        2 => w.delete(id),
        _ => {}
    }
}

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
