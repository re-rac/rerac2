//! The colour-cycling light **0x0cfd** ([`update_0cfd`], L04 `0x433998`, 4 placed; the same code — the decomp's
//! `func_L04_00432258` — at L06 `0x43b1c8`, 32; L07 `0x3e3698`, 28; L12 `0x3face8`, 7; L13 `0x3f9dc8`, 29).

use crate::hero::physics::{set_len, sub};
use crate::moby::{Ctx, MobyWorld};

/// `0x433c70(a, alpha)`: a hue: `a`/2π·6 through red → yellow → green → cyan → blue → magenta (each sixth the next
/// channel up or the last down), `alpha` in the top byte.
fn hue(a: f32, alpha: u32) -> u32 {
    let f = a * 0.159_154_94 * 6.0;
    let k = f as i32;
    let t = f - k as f32;
    let up = (t * 255.0) as u32;
    let down = ((1.0 - t) * 255.0) as u32;
    let c = match k { 0 => up << 8 | 0xff, 1 => down | 0xff00, 2 => up << 16 | 0xff00, 3 => down << 8 | 0xff_0000, 4 => up | 0xff_0000, 5 => down << 16 | 0xff, _ => 0 };
    alpha << 24 | c
}

/// 0x0cfd (`0x433998`, 4 placed; the same code at L06 `0x43b1c8`, L07 `0x3e3698`, L12 `0x3face8`, L13 `0x3f9dc8`):
/// a light whose colour cycles. With pvars: **0** update distance 0xff, its phase 0 and step 2°; 1. **1**: the phase
/// on (wrapped at 2π); its glow word (`+0x78`) the hue of 30° + 30°·sin(phase) ([`hue`], alpha 0x7f); facing the
/// camera (its first row's dot with the way to it ≥ 0), a glow (`0x362ba0` = L00 `0x33f550`, crate::particles::glow:
/// 105000, pulled 1, two ticks, far 0xc0) in that colour at alpha 127·the dot.
pub fn update_0cfd(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.is_empty() { return; }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.set_pv_f(4, f32::from_bits(0x3d0e_fa36));
            m.set_pv_f(0, 0.0);
            m.state = 1;
        }
        1 => {
            let m = &mut w.mobys[id];
            let mut ph = m.pv_f(0) + m.pv_f(4);
            if std::f32::consts::TAU <= ph { ph -= std::f32::consts::TAU; }
            m.set_pv_f(0, ph);
            m.glow = hue(ph.sin() * 0.523_598_8 + 0.523_598_8, 0x7f);
            let d = set_len(sub(ctx.host.camera_pos(), m.pos), 1.0);
            let r = m.rows[0];
            let f = r[0] * d[0] + r[1] * d[1] + r[2] * d[2];
            if 0.0 <= f {
                let (pos, c) = (m.pos, m.glow & 0xff_ffff | ((f * 127.0) as i32 as u32) << 24);
                if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::glow(sys, &mut ctx.game.rng, 105_000.0, 1.0, [0.0; 3], id, pos, 0xc0, false, c, 2); }
            }
        }
        _ => {}
    }
}
