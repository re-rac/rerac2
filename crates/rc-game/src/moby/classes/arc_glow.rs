//! The arc glow **0x1212** ([`update_1212`], L18 `0x43fc50`, 12 placed; L20 `0x447010`, 11; the decomp's
//! `func_L18_0043E760`, the same code on the two levels): a bobbing glow with two arcs. L18 addresses. Its pvars:
//! `+0x140` its place at the start, `+0x150` / `+0x154` its bob's phase and last offset, `+0x158` / `+0x15c` its pulse
//! and its facing (for its draws).
//! * **0**: the bob's phase random (`0x336af0` = L00 `0x31bb00`), its start kept, its arcs set up (`0x4402d8`), 1.
//! * **Drawn**: the pulse's phase `(frame mod 60)/60·2π − π`; its glow (`+0x78`) the pulse (`sin·½ + ½`,
//!   `0x341d10`) of 0x80202080 / 0x20202040 and of 0x80400000 / 0x20000000 blended (`0x338d30` = L00 `0x31dc10`,
//!   crate::hud::lerp_rgba) by the square of how square its first row faces the camera (`0x1b9740`).
//! * **Each tick** it bobs 0.1 about its start's height at 300° a second (`0x33f1b0` = L00 `0x324430`, `$gp+0x2be8` /
//!   `+0x2bec`).
//!
//! Not ported: its arcs (`0x4402d8` the set-up, `0x440430` each drawn tick: two ribbons from random points of the
//! table `0x2bd9b0`, their waves) and its three draws (`0x301f28(0x43ff80)`, `(0x43fe28)`, `(0x43fea0)`), which are a
//! render path of their own.

use crate::hero::physics::{dot, set_len, sub};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x1212;
/// `$gp+0x2bd8..+0x2be4`: the pulses' colours (the first pair, the second pair).
const ARC_GLOW: [u32; 4] = [0x8020_2080, 0x2020_2040, 0x8040_0000, 0x2000_0000];
/// `$gp+0x2be8` / `+0x2bec`: the bob's height and speed (degrees a second).
const ARC_BOB: (f32, f32) = (0.1, 300.0);

/// 0x1212 (module docs).
pub fn update_1212(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let ph = ctx.game.rng.rand_angle();
        let m = &mut w.mobys[id];
        m.set_pv_f(0x150, ph);
        let p = m.pos;
        m.set_pv_v(0x140, p);
        m.set_state(1, -1);
    }
    if w.mobys[id].drawn != 0 {
        let phase = (ctx.game.frame % 60) as f32 * std::f32::consts::TAU / 60.0 - std::f32::consts::PI;
        let s = phase.sin() * 0.5 + 0.5;
        let a = crate::hud::lerp_rgba(s, ARC_GLOW[0], ARC_GLOW[1]);
        let b = crate::hud::lerp_rgba(s, ARC_GLOW[2], ARC_GLOW[3]);
        let cam = ctx.host.camera_pos();
        let m = &mut w.mobys[id];
        m.set_pv_f(0x158, phase.cos() * 0.5 + 0.5);
        let f = dot(set_len(sub(cam, m.pos), 1.0), m.rows[0]).abs();
        let f = f * f;
        m.set_pv_f(0x15c, f);
        m.glow = crate::hud::lerp_rgba(f, b, a);
    }
    let m = &mut w.mobys[id];
    let base = m.pv_f(0x148);
    crate::moby::classes::props::bob(m, ARC_BOB.0, ARC_BOB.1 * 0.000_290_888_24, base, 0x150, 0x154);
}
