//! Level 14's scrolling sheet **0x0bff** (`0x3f2298`, 5 placed; the decomp's `func_L14_003F0D50`): two quads of FX
//! 0x3f in its frame whose texture runs along them. Its pvars: `+0x00` / `+0x08` the two scrolls, `+0x04` / `+0x0c`
//! their steps, `+0x10` on.
//!
//! * **0**: on; the view distance 2048 (`0x2ed778`, crate::game::GameState::set_view_far); 1.
//! * **Every tick**: each scroll on by its step, back by 8 at 8; hooked (`0x3336b8`): off. On: its draw ([`draw`],
//!   `0x3f23b0`) registered. New steps: `randf(0.003, 0.005)` and `randf(0.001, 0.0025)`.

use crate::hero::physics::{add, scale, V3};
use crate::moby::MobyWorld;

pub const CLASS: i16 = 0x0bff;

/// A sheet's look: the quads' colour, their first alpha, FX texture, extents across (y) and along (z) in its frame,
/// and the pvars of their two scrolls.
pub struct Look {
    pub rgb: u32,
    pub alpha: u32,
    pub fx: usize,
    pub y: (f32, f32),
    pub z: (f32, f32),
    pub scrolls: (usize, usize),
}

/// L14's (`$gp+0x18e4` / `+0x18e8` / `+0x18ec`, `0x1b08d4`; the extents `0x3f23e0`'s).
pub const L14: Look = Look { rgb: 0x0040_4090, alpha: 0x4000_0000, fx: 0x3f, y: (-2.0, 2.0), z: (-4.0, 4.0), scrolls: (0, 8) };
/// L16's copy (`0x419620` → `0x4196e0`, its set-up `0x419650`; `$gp+0x2bd0..0x2be8`): the gate 0x10fb
/// (crate::moby::classes::level16::gate), its scrolls `+0x10` / `+0x18`.
pub const L16: Look = Look { rgb: 0x0030_1010, alpha: 0x4000_0000, fx: 0x18, y: (-0.0, 22.0), z: (-14.0, 14.0), scrolls: (0x10, 0x18) };

/// 0x0bff (module docs).
pub fn update_0bff(w: &mut MobyWorld, id: usize, ctx: &mut crate::moby::Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 {
        m.set_pv_i(0x10, 1);
        ctx.game.set_view_far(2048.0);
        m.state = 1;
    }
    let a = m.pv_f(0) + m.pv_f(4);
    m.set_pv_f(0, if 8.0 <= a { a - 8.0 } else { a });
    let b = m.pv_f(8) + m.pv_f(0xc);
    m.set_pv_f(8, if 8.0 <= b { b - 8.0 } else { b });
    if m.once & 4 != 0 { m.set_pv_i(0x10, 0); }
    if m.pv_i(0x10) != 0 { w.register_draw(id); }
    let g = &mut ctx.game.rng;
    let (s, t) = (g.randf(f32::from_bits(0x3b44_9ba6), f32::from_bits(0x3ba3_d70a)), g.randf(f32::from_bits(0x3a83_126f), f32::from_bits(0x3b23_d70a)));
    let m = &mut w.mobys[id];
    m.set_pv_f(4, s);
    m.set_pv_f(0xc, t);
}

/// `0x3f2480`'s corners' alphas: from `alpha`, each corner `ftoi(randf_signed(0, spread))` lower than the one before
/// (the first from `alpha` itself), over `rgb`.
fn alphas(rng: &mut crate::rng::Rng, rgb: u32, mut alpha: u32, first: f32) -> [u32; 4] {
    let mut out = [0; 4];
    for (k, o) in out.iter_mut().enumerate() {
        let j = rng.randf_signed(0.0, if k == 0 { first } else { 9.0 }) as i32;
        alpha = alpha.wrapping_sub((j as u32).wrapping_shl(24));
        *o = rgb.wrapping_add(alpha);
    }
    out
}

/// `0x3f23b0` → `0x3f2480(m, place, rgb, alpha, 0x40000000)`: the quad ([`L14`]: (0, ±2, ±4)) in its frame — its
/// angles (π/2, `+0xf4`, `+0xf8`) at its place — FX 0x3f, s from the first scroll to one more along it, t across;
/// additive (ALPHA 0x48), its corners' alphas jittered (±5, then ±9 a corner). Then the same quad in the normal blend
/// (0x44) with the second scroll, its alphas from 0x40000000 jittered ±9 a corner.
pub fn draw(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, out: &mut Vec<crate::fx::FxQuad>) { draw_as(w, id, &L14, rng, out) }

/// [`draw`] with a level's [`Look`].
pub fn draw_as(w: &MobyWorld, id: usize, look: &Look, rng: &mut crate::rng::Rng, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let rows = crate::moby::attach::rows_of([std::f32::consts::FRAC_PI_2, m.rot[1], m.rot[2]]);
    let at = |y: f32, z: f32| -> V3 { add(m.pos, add(scale(rows[1], y), scale(rows[2], z))) };
    let ((y0, y1), (z0, z1)) = (look.y, look.z);
    let corners = [at(y0, z0), at(y0, z1), at(y1, z0), at(y1, z1)];
    let st = |s: f32| [[s, 0.0], [s, 1.0], [s + 1.0, 0.0], [s + 1.0, 1.0]];
    let rgba = alphas(rng, look.rgb, look.alpha, 5.0);
    out.push(crate::fx::FxQuad { fx: look.fx, additive: true, corners, st: st(m.pv_f(look.scrolls.0)), rgba });
    let rgba = alphas(rng, look.rgb, 0x4000_0000, 9.0);
    out.push(crate::fx::FxQuad { fx: look.fx, additive: false, corners, st: st(m.pv_f(look.scrolls.1)), rgba });
}
