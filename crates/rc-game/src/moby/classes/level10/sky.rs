//! Hrugis's sky pieces (L10 addresses).
//!
//! * **0x11c8** ([`update_11c8`], `0x42d9e0`, 2 placed; the decomp's `func_L10_0042BCA0`): a ring of the sky at
//!   (512, 512, 512), drawn from any distance (`+0x32` 0x1ff), four times its class's scale, with a partner 0x11cd
//!   made at its set-up. With `+0x68`[0] set it sits 6 higher, flipped (x angle π), and turns 0.28° a tick one way;
//!   else 6 lower and turns the other way. Its partner keeps its place and angles.

use crate::moby::{ai, Ctx, MobyWorld};

pub const RING: i16 = 0x11c8;
const PARTNER: i16 = 0x11cd;

pub fn update_11c8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let _ = ctx;
    if w.mobys[id].state == 0 {
        let s = w.class(RING).map_or(1.0, |c| c.scale) * 4.0;
        let p = w.class(PARTNER).is_some().then(|| w.create(PARTNER));
        let ps = w.class(PARTNER).map_or(1.0, |c| c.scale) * 4.0;
        if let Some(p) = p {
            let q = &mut w.mobys[p];
            (q.update_dist, q.draw_dist, q.scale) = (0xff, 0x1ff, ps);
        }
        let m = &mut w.mobys[id];
        m.set_state(1, -1);
        (m.draw_dist, m.update_dist, m.scale) = (0x1ff, 0xff, s);
        m.pos = [512.0, 512.0, 512.0];
        ai::set_link(m, 4, p);
        if m.pv_i(0) != 0 {
            m.pos[2] += 6.0;
            m.rot[0] = std::f32::consts::PI;
        } else {
            m.pos[2] -= 6.0;
        }
        w.changed = true;
    }
    let m = &mut w.mobys[id];
    let step = 0.004_908_739;
    m.rot[2] = if m.pv_i(0) != 0 { crate::hero::physics::add_rot(m.rot[2], step) } else { crate::hero::physics::sub_rot(m.rot[2], step) };
    let (pos, rot) = (m.pos, m.rot);
    if let Some(p) = ai::link(m, 4).filter(|&p| p < w.mobys.len()) {
        let q = &mut w.mobys[p];
        (q.pos, q.rot[0], q.rot[2]) = (pos, rot[0], rot[2]);
    }
}
