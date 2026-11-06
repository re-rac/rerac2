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

/// `$gp+0x18e4` / `+0x18e8` / `+0x18ec` (L14 `0x1b08d4`): the quads' colour, their alpha and their FX texture.
const RGB: u32 = 0x0040_4090;
const ALPHA: u32 = 0x4000_0000;
const FX: usize = 0x3f;

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
/// (the first from `alpha` itself), over [`RGB`].
fn alphas(rng: &mut crate::rng::Rng, mut alpha: u32, first: f32) -> [u32; 4] {
    let mut out = [0; 4];
    for (k, o) in out.iter_mut().enumerate() {
        let j = rng.randf_signed(0.0, if k == 0 { first } else { 9.0 }) as i32;
        alpha = alpha.wrapping_sub((j as u32).wrapping_shl(24));
        *o = RGB.wrapping_add(alpha);
    }
    out
}

/// `0x3f23b0` → `0x3f2480(m, place, [`RGB`], [`ALPHA`], 0x40000000)`: the quad (0, ±2, ±4) in its frame — its
/// angles (π/2, `+0xf4`, `+0xf8`) at its place — FX 0x3f, s from the first scroll to one more along it, t across;
/// additive (ALPHA 0x48), its corners' alphas jittered (±5, then ±9 a corner). Then the same quad in the normal blend
/// (0x44) with the second scroll, its alphas from 0x40000000 jittered ±9 a corner.
pub fn draw(w: &MobyWorld, id: usize, rng: &mut crate::rng::Rng, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let rows = crate::moby::attach::rows_of([std::f32::consts::FRAC_PI_2, m.rot[1], m.rot[2]]);
    let at = |y: f32, z: f32| -> V3 { add(m.pos, add(scale(rows[1], y), scale(rows[2], z))) };
    let corners = [at(-2.0, -4.0), at(-2.0, 4.0), at(2.0, -4.0), at(2.0, 4.0)];
    let st = |s: f32| [[s, 0.0], [s, 1.0], [s + 1.0, 0.0], [s + 1.0, 1.0]];
    let rgba = alphas(rng, ALPHA, 5.0);
    out.push(crate::fx::FxQuad { fx: FX, additive: true, corners, st: st(m.pv_f(0)), rgba });
    let rgba = alphas(rng, 0x4000_0000, 9.0);
    out.push(crate::fx::FxQuad { fx: FX, additive: false, corners, st: st(m.pv_f(8)), rgba });
}
