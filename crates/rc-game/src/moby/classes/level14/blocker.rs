//! The prison's blockers **0x0aeb** (3 placed) and **0x1124** (2), one update (`0x3e0d10`, the decomp's
//! `func_L14_003DF7F8`): each closes a record of the robots' way network (crate::moby::classes::endako_d's 0x0f2d)
//! until it is hooked, then rolls away and is gone.
//!
//! **Its pvars:** `+0x00` the network moby (−1 none), `+0x04` a moby whose count its marker shows (−1: no marker),
//! `+0x08` it rolls the other way, `+0x0c` its speed a tick, `+0x10` the way rolled.
//!
//! **0**: 1; with a network, the record under the point one first row from its place closed (`0x3e0f28` →
//! crate::moby::classes::endako_d::net_mark, `0x421f60`). **1**: hooked (`+0xbe` bit 2, `0x3336b8`) — that record
//! opened, 2, its speed −2 a second (`$gp+0x1348`; +2 with `+0x08`), the way 0. **2**: its first tick its class sound 0
//! (`0x36ada0`); each tick it moves its speed along up × its first row (`0x2f6b50`, set to that length `0x2f6d00`), the way on by the speed, the speed
//! 4 a second² on (`$gp+0x134c`; off with `+0x08`) and its x angle turned by −0.75 × the speed (`$gp+0x1350`); past 4
//! either way, deleted (`0x319058`).
//!
//! **The marker** (drawn, with `+0x04` ≠ −1: [`draw`], `0x3e0f78` on the frame's first list, `0x2ef098`): one quad,
//! FX 2 (ALPHA row 0), colour 0x80202080, in its frame (its rows × its scale over its class's): at x −0.15, ±0.5 along
//! y and z, 0.25 up; its S from −0.005 plus the shown count over 16 (one more at the second pair of corners), its T
//! 0.52 / 0.625. The count: that moby's `+0x10` word while it is in state 2 (`0x421de8`), else 0.
//!
//! Not ported: the mirrored-controls option (`0x1a7a4c`) turns the marker's y over; the port has no such option.

use crate::fx::FxQuad;
use crate::hero::physics::{add, add_rot, cross, scale, set_len, V3};
use crate::moby::classes::endako_d;
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0aeb;
pub const CLASS_B: i16 = 0x1124;

const NET: usize = 0x00;
const SHOWN: usize = 0x04;
const BACK: usize = 0x08;
const SPEED: usize = 0x0c;
const ROLLED: usize = 0x10;

/// `$gp+0x1348..0x1350`: the speed, its change and the spin.
const START: f32 = -2.0;
const ACCEL: f32 = 4.0;
const SPIN: f32 = -0.75;

/// `0x3e0f28(m, open)`: the record one first row from its place opened or closed.
fn mark(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, open: bool) {
    let Some(net) = endako_d::network(w, w.mobys[id].pv_i(NET)) else { return };
    let m = &w.mobys[id];
    let p = add(m.rows[0], m.pos);
    endako_d::net_mark(w, net, ctx, p, open);
}

/// 0x0aeb / 0x1124 (module docs).
pub fn update_0aeb(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            w.mobys[id].set_state(1, -1);
            if w.mobys[id].pv_i(NET) != -1 { mark(w, id, ctx, false); }
        }
        1 => {
            if w.mobys[id].once & 4 != 0 {
                if w.mobys[id].pv_i(NET) != -1 { mark(w, id, ctx, true); }
                let m = &mut w.mobys[id];
                m.set_state(2, -1);
                let s = START * 0.016_666_668;
                m.set_pv_f(SPEED, if m.pv_i(BACK) != 0 { -s } else { s });
                m.set_pv_f(ROLLED, 0.0);
            }
        }
        2 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            let m = &mut w.mobys[id];
            let v = m.pv_f(SPEED);
            let d = set_len(cross([0.0, 0.0, 1.0], m.rows[0]), v);
            m.pos = add(m.pos, d);
            m.set_pv_f(ROLLED, m.pv_f(ROLLED) + v);
            let a = ACCEL * f32::from_bits(0x3991_a2b4);
            let s = if m.pv_i(BACK) != 0 { v - a } else { v + a };
            m.set_pv_f(SPEED, s);
            m.rot[0] = add_rot(m.rot[0], s * SPIN);
            w.changed = true;
            if 4.0 < w.mobys[id].pv_f(ROLLED).abs() {
                w.delete(id);
                return;
            }
        }
        _ => {}
    }
    let m = &w.mobys[id];
    if m.drawn != 0 && m.pv_i(SHOWN) != -1 { w.register_draw_first(id); }
}

/// `0x3e0f78` (module docs).
pub fn draw(w: &MobyWorld, id: usize, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let n = usize::try_from(m.pv_i(SHOWN)).ok().and_then(|k| w.mobys.get(k)).map_or(0, |o| if o.state == 2 { o.pv_i(0x10) } else { 0 });
    let ratio = w.class(m.o_class).map_or(1.0, |k| if k.scale != 0.0 { m.scale / k.scale } else { 1.0 });
    let rows = m.rows.map(|r| scale(r, ratio));
    let world = |v: V3| add(m.pos, add(add(scale(rows[0], v[0]), scale(rows[1], v[1])), scale(rows[2], v[2])));
    let half = 0.5;
    let corners: [V3; 4] = std::array::from_fn(|k| {
        let y = if k < 2 { half } else { -half };
        let z = if k & 1 == 0 { half } else { -half };
        world([-0.15, y, z + 0.25])
    });
    let st: [[f32; 2]; 4] = std::array::from_fn(|k| {
        let f = if k < 2 { n } else { n + 1 } as f32;
        [-0.005 + f / 16.0, if k & 1 == 0 { 0.52 } else { 0.625 }]
    });
    out.push(FxQuad { fx: 2, additive: false, corners, st, rgba: [0x8020_2080; 4] });
}
