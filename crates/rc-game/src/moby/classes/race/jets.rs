//! The boost jets **0x0474** / **0x0cef** of the race tracks (L04 `0x3c9760`, L11 `0x3b9738`; the decomp's
//! `func_L04_003C8380`, `UpdateChildMobyStream`): five flame mobys (0x0f10 for 0x0cef, else 0x0f0f) streaming along
//! the jet's forward row. A racer over a jet takes its boost (crate::moby::classes::race::racers, `0x403c70`: the
//! jet's pvar short `+0` the boost's ticks).
//!
//! Its pvars: `+0x04[5]` the flames, `+0x18[5]` their distances along the forward row.
//! * **0**: the five made (`0x311320`; `0x32a748(child, 0)`: update distance 0xff, draw distance 0xff, drawn, mode
//!   & 0xfff8), turned as the jet, spaced from the first place (0x0cef: −2, 0.85 apart; else −1.5, 0.75 apart); 1.
//! * **Each tick**: each flame on by 1/12 (from the end, 1.4 / 1.5, back to the start), placed that far along the
//!   forward row and 0.15 up; its scale its class's × (the least, 0.14 / 0.114, up to 0.24 / 0.21 at the start) by
//!   how far it has to go; faded in over the first quarter and out over the last (alpha × 128); its colour
//!   (`0x313300`) from yellow (252 + 2g, 224g, 6g with g = (1 − t)⁴) to red.

use crate::hero::physics::{add, scale};
use crate::moby::{Ctx, MobyWorld};

const OFFSETS: usize = 0x18;
const CHILDREN: usize = 0x04;

pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let b_class = w.mobys[id].o_class == super::JET_B;
    let (first, spacing, start, end, min_scale, max_scale, child) =
        if b_class { (-2.0, 0.85, -2.0, 1.4, 0.14, 0.24, 0x0f10) } else { (-1.5, 0.75, -1.5, 1.5, 0.114, 0.21, 0x0f0f) };
    let length = end - first;
    let range = max_scale - min_scale;
    if w.mobys[id].state == 0 {
        let mut at = first;
        let rot = w.mobys[id].rot;
        for i in 0..5 {
            let c = w.create(child);
            {
                let cm = &mut w.mobys[c];
                cm.update_dist = 0xff;
                cm.draw_dist = 0xff;
                cm.drawn = 1;
                cm.mode &= 0xfff8;
                cm.rot = rot;
            }
            crate::moby::ai::set_link(&mut w.mobys[id], CHILDREN + 4 * i, Some(c));
            w.mobys[id].set_pv_f(OFFSETS + 4 * i, at);
            at += spacing;
        }
        w.mobys[id].state = 1;
    }
    let (pos, fwd, up) = { let m = &w.mobys[id]; (m.pos, m.rows[0], m.rows[2]) };
    for i in 0..5 {
        let mut off = w.mobys[id].pv_f(OFFSETS + 4 * i) + 0.083_333_336;
        if end < off { off = start; }
        w.mobys[id].set_pv_f(OFFSETS + 4 * i, off);
        let Some(c) = crate::moby::ai::link(&w.mobys[id], CHILDREN + 4 * i).filter(|&c| c < w.mobys.len()) else { continue };
        let t = (off - start) / length;
        let class_scale = w.class(child).map_or(1.0, |k| k.scale);
        let alpha = if t < 0.25 { t * 4.0 } else if 0.75 < t { 1.0 - (t - 0.75) * 4.0 } else { 1.0 };
        let mut g = 1.0 - t;
        g *= g;
        g *= g;
        let cm = &mut w.mobys[c];
        cm.pos = add(add(pos, scale(fwd, off)), scale(up, 0.15));
        cm.scale = class_scale * (min_scale + (1.0 - t) * range);
        cm.alpha = (alpha * 128.0) as i32 as u8;
        cm.ambient = [(252 + (g * 2.0) as i32) as u8, (g * 224.0) as i32 as u8, (g * 6.0) as i32 as u8];
        w.rebuild(c, &*ctx.host);
    }
}
