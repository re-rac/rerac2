//! Hrugis's sky pieces (L10 addresses).
//!
//! * **0x11c8** ([`update_11c8`], `0x42d9e0`, 2 placed; the decomp's `func_L10_0042BCA0`): a ring of the sky at
//!   (512, 512, 512), drawn from any distance (`+0x32` 0x1ff), four times its class's scale, with a partner 0x11cd
//!   made at its set-up. With `+0x68`[0] set it sits 6 higher, flipped (x angle π), and turns 0.28° a tick one way;
//!   else 6 lower and turns the other way. Its partner keeps its place and angles.
//! * **0x11c9** ([`update_11c9`], `0x42db60`, 1 placed; the decomp's `func_L10_0042BE20`): the sky's light at
//!   (512, 512, 512), drawn from any distance: at its set-up a point light there (`0x318638(40, 0, 0, 0, 1, at)` =
//!   L00 `0x2fdb60`: radius 40, intensity 0, colour (0, 0, 1); its slot in `$gp+0x25f8`), then each tick it turns
//!   0.5625° about z (`+0xf8`) and draws ([`draw_11c9`], `0x42dc20` on the second list): three additive glows (FX
//!   0xb, random rolls) — 8..15 in 0x800000 at alpha 0x20..0x80 0.01 toward the camera, 4..12 in 0x808080 at an
//!   alpha `n` of 1..0x80 0.1 toward it, 8..17 in 0x804040 at alpha 0..0x80 0.05 toward it — and the light's radius
//!   `n`/2 (`n` taken as a signed byte: 0x80 gives −64).

use crate::moby::{ai, Ctx, MobyWorld};

pub const RING: i16 = 0x11c8;
pub const LIGHT: i16 = 0x11c9;
/// The light's slot + 1 (the port's word; `$gp+0x25f8`).
const SLOT: usize = 0x400;
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

/// `0x42db60`.
pub fn update_11c9(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let _ = ctx;
    let m = &mut w.mobys[id];
    if m.state == 0 {
        m.set_state(1, -1);
        (m.draw_dist, m.update_dist) = (0x1ff, 0xff);
        m.pos = [512.0; 3];
        let at = m.pos;
        let slot = w.point_lights.alloc(crate::point_lights::PointLight { color: [0.0, 0.0, 1.0], intensity: 0.0, pos: at, radius: 40.0 });
        w.mobys[id].set_pv_i(SLOT, slot.map_or(0, |s| s as i32 + 1));
        w.changed = true;
    }
    let m = &mut w.mobys[id];
    m.rot[2] = crate::hero::physics::add_rot(m.rot[2], 0.009_817_478);
    w.register_draw(id);
}

/// `0x42dc20` (module docs).
pub fn draw_11c9(w: &mut MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: crate::hero::physics::V3, out: &mut Vec<crate::fx::FxQuad>) {
    let at = w.mobys[id].pos;
    let n = rng.rand_range(1, 0x80);
    let a = rng.rand_range(0x20, 0x80);
    let (size, roll) = (rng.randf(8.0, 15.0), rng.rand_angle());
    crate::fx::sprite(out, size, 0.01, roll, at, cam, 0xb, (a as u32) << 24 | 0x0080_0000, true);
    let (size, roll) = (rng.randf(4.0, 12.0), rng.rand_angle());
    crate::fx::sprite(out, size, 0.1, roll, at, cam, 0xb, (n as u32) << 24 | 0x0080_8080, true);
    let a = rng.rand_range(0, 0x80);
    let (size, roll) = (rng.randf(8.0, 17.0), rng.rand_angle());
    crate::fx::sprite(out, size, 0.05, roll, at, cam, 0xb, (a as u32) << 24 | 0x0080_4040, true);
    if let Some(s) = usize::try_from(w.mobys[id].pv_i(SLOT) - 1).ok() {
        if let Some(mut l) = w.point_lights.slots.get(s).copied().flatten() {
            l.radius = (n as u8 as i8) as f32 * 0.5;
            w.point_lights.set(s, l);
        }
    }
}
