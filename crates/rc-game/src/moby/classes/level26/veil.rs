//! Level 26's veil **0x0c91** (`0x3e0b90`, 1 placed; the decomp's `func_L26_003DF680`, its draw `func_L26_003DF7E8`
//! = `0x3e0cf8`): 64 quads in the world, drawn twice with two scrolling, pulsing layers of light while the gravity
//! frames are on. Its pvars: `+0x00` / `+0x04` the layers' pulses, `+0x08` / `+0x0c` their scrolls.
//!
//! * **0**: alpha 0xff; 1; pulse 0 at 0, pulse 1 at 0.5.
//! * **1**: with the gravity frames on (`0x1a8ff0`, crate::gravity): each layer's pulse on by `$gp+0x1620` (0.5) / 60;
//!   past 1: less 1, and its scroll the other layer's plus `randf(0.333, 0.666)` (`0x31bbf0`), less 1 past 1; its draw
//!   ([`draw`]) on the first list (`0x2e7db0` = L00 `0x2e7718`).
//!
//! The quads are the overlay's tables, read at the level's start ([`Mesh::read`]): 64 faces (`0x29b3d0`, 0x18 bytes
//! each, the four corners' point indices in shorts 0, 3, 6, 9) over the points `0x29abd0` (16 bytes each).

use crate::fx::FxQuad;
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c91;

/// `0x29abd0` / `0x29b3d0` (L26).
const POINTS_AT: u32 = 0x0029_abd0;
const FACES_AT: u32 = 0x0029_b3d0;
const FACES: usize = 64;
/// `$gp+0x1620` (`0x1b0610`): the pulses' rate a second.
const RATE: f32 = 0.5;
/// `$gp+0x1638`: the FX texture; `$gp+0x1640`: the colours by row; `$gp+0x1648`: the corners' STs.
const FX: usize = 14;
const COLOURS: [u32; 2] = [0x3000_4060, 0x0000_00ff];
const ST: [[f32; 2]; 4] = [[0.0, 0.5], [1.0, 0.5], [0.0, 1.0], [1.0, 1.0]];

/// The veil's quads (the engine reads them at the level's start: crate::moby::MobyWorld::veil26).
#[derive(Clone, Debug, PartialEq)]
pub struct Mesh {
    pub faces: Vec<[[f32; 3]; 4]>,
}

impl Mesh {
    /// The tables at the level's own addresses.
    pub fn read(read: &dyn Fn(u32, usize) -> Option<Vec<u8>>) -> Option<Mesh> {
        let f = read(FACES_AT, FACES * 0x18)?;
        let half = |i: usize| u16::from_le_bytes([f[2 * i], f[2 * i + 1]]) as usize;
        let idx: Vec<[usize; 4]> = (0..FACES).map(|k| std::array::from_fn(|j| half(k * 12 + j * 3))).collect();
        let n = idx.iter().flatten().max()? + 1;
        let p = read(POINTS_AT, n * 16)?;
        let fl = |i: usize| f32::from_le_bytes(p[4 * i..4 * i + 4].try_into().unwrap());
        Some(Mesh { faces: idx.iter().map(|q| q.map(|i| [fl(4 * i), fl(4 * i + 1), fl(4 * i + 2)])).collect() })
    }
}

/// `0x3e0b90` (module docs).
pub fn update_0c91(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    match m.state {
        0 => {
            m.alpha = 0xff;
            m.set_state(1, -1);
            m.set_pv_f(0, 0.0);
            m.set_pv_f(4, 0.5);
        }
        1 => {
            if !ctx.hero.gravity.on { return; }
            for i in 0..2 {
                let v = m.pv_f(4 * i) + RATE * 0.016_666_668;
                m.set_pv_f(4 * i, v);
                if 1.0 < v {
                    m.set_pv_f(4 * i, v - 1.0);
                    let s = m.pv_f(8 + 4 * ((i + 1) & 1)) + ctx.game.rng.randf(0.333, 0.666);
                    m.set_pv_f(8 + 4 * i, if 1.0 < s { s - 1.0 } else { s });
                }
            }
            w.register_draw_first(id);
        }
        _ => {}
    }
}

/// `0x3e0cf8` (a first-list callback): each face drawn twice, layer by layer — FX 14 additive (ALPHA 0x48, CLAMP
/// REPEAT), each corner's s its ST's plus the layer's scroll, its colour the row's (`⌊t + 0.2⌋`: 0x30004060 on the
/// near row, 0x000000ff on the far one) with its alpha × `sin(pulse·π)` (`0x2f0000` from its colour without alpha).
pub fn draw(w: &MobyWorld, id: usize, out: &mut Vec<FxQuad>) {
    let Some(mesh) = w.veil26.as_deref() else { return };
    let m = &w.mobys[id];
    let layers = [0, 1].map(|i| {
        let blend = (m.pv_f(4 * i) * std::f32::consts::PI).sin();
        let rgba = ST.map(|[_, t]| {
            let c = COLOURS[((t + 0.2) as i32).clamp(0, 1) as usize];
            crate::hud::lerp_rgba(blend, c & 0xff_ffff, c)
        });
        (m.pv_f(8 + 4 * i), rgba)
    });
    for q in &mesh.faces {
        for (scroll, rgba) in layers {
            out.push(FxQuad { fx: FX, additive: true, corners: *q, st: ST.map(|[s, t]| [s + scroll, t]), rgba });
        }
    }
}
