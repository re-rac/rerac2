//! Level 14's electric strand **0x0f80** (`0x423798`, 18 placed; the decomp's `func_L14_004221D8`): the vents'
//! sequencer's light (crate::moby::classes::level14::steam, `0x3e66c8`: it sets the strand's colours `+0x48` /
//! `+0x4c`, its path `+0x34` and turns it on, `+0x3c` = 1; its ramp sets the core's widths `+0x40` / `+0x44`; it turns
//! it off again). L11's fence 0x0ee0 (crate::moby::classes::level11::beams) is the same strand.
//!
//! Its pvars: `+0x00` / `+0x10` the ends of its line (its place, and 6..177 along y from it), `+0x20` the line's
//! length, `+0x2c` the points drawn, `+0x34` its path, `+0x38` its row of the strand buffer (`0x259470`, 0x280 a
//! row: the strand `+0x000` and the glow strand `+0x140`, 20 points each; [`Globals::strands`]), `+0x3c` on.
//!
//! Each tick:
//! * its line's second end 6..177 out along y (`+0x20 += (6 − it)·0.07 + 12`), the points 20 (`$gp+0x25c8`);
//! * `0x423948` (L11 `0x46d6c0`): once for the level (`$gp+0x25cc`), its row's strand laid straight along its line
//!   in steps of 15 / 20 and the phases 0;
//! * on: `0x423a58` (the crackle): the phases `$gp+0x25e8` / `+0x25f0` a random angle each, `+0x2604` on by 3°; the
//!   strand at the path's points, each but the first moved by `±(0..0.3)` along z, x and y; the glow strand at the
//!   same points plus `0.8·sin(+0x2604 + 12°·k)·sin(+0x25f0 + 57°·k) ± (0..0.1)` up and `0.5·cos(+0x25f0 + 57°·k)`
//!   across the strand; past the second point, at every second one and the last, the line along the path from two
//!   points back meets what it can (`0x2e61f8(a, b, 0x11, m, hit)`: 0 damage, flags 1, its class's kind), and when
//!   that is Ratchet his updraft's lift is spent (`0x18a3d0` / `0x18a3d8` = 0, crate::hero::extras); then the
//!   strand's last four points at the path's and its draw ([`draw`], `0x423e80`);
//! * the first tick: off, the points 20, its colours 0x7f7f7f / 0x803040 and the core's widths 0.05 / 0.4
//!   (`$gp+0x2630..0x263c`), `+0x30` −1, **1**.
//!
//! Not ported: the point light its first tick takes (`0x3053a0(0, 0, place, 0)`, never set again).

use crate::hero::physics::{add, add_rot, atan2, set_len, sub, wrap, V3};
use crate::moby::classes::level11::beams::Strands;
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0f80;

const POINTS: usize = 20;

/// The level's strand state (`$gp+0x25cc..`).
#[derive(Clone, Debug)]
pub struct Globals {
    /// `+0x25cc` the straight lay still to do, `+0x25e8` / `+0x25f0` / `+0x2604` the phases, `+0x2628` the core's
    /// texture scroll.
    pub lay: bool,
    pub phase_a: f32,
    pub phase_b: f32,
    pub phase_c: f32,
    pub scroll: f32,
    /// The strand buffer's rows (`0x259470 + 0x280·row`), by row.
    pub strands: std::collections::HashMap<i32, Strands>,
}

impl Default for Globals {
    fn default() -> Self { Globals { lay: true, phase_a: 0.0, phase_b: 0.0, phase_c: 0.0, scroll: 0.0, strands: Default::default() } }
}

/// 0x0f80 (module docs).
pub fn update_0f80(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let a = m.pos;
    m.set_pv_v(0, a);
    let l = m.pv_f(0x20);
    let l = l + (6.0 - l) * 0.07 + 12.0;
    m.set_pv_f(0x20, l);
    let b = add(a, [0.0, l, 0.0]);
    m.set_pv_v(0x10, b);
    let (row, on, path) = (m.pv_i(0x38), m.pv_i(0x3c) != 0, m.pv_i(0x34));
    let g = &mut w.fence14;
    let s = g.strands.entry(row).or_insert_with(|| Strands { core: vec![[0.0; 3]; POINTS], glow: vec![[0.0; 3]; POINTS] });
    // `0x423948`.
    if g.lay {
        g.lay = false;
        let step = set_len(sub(b, a), 15.0 / POINTS as f32);
        s.core[0] = a;
        for k in 1..POINTS { s.core[k] = add(s.core[k - 1], step); }
        g.phase_a = 0.0;
        g.phase_b = 0.0;
        g.phase_c = 0.0;
    }
    if on {
        let path = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).cloned().unwrap_or_default();
        let point = |k: usize| -> V3 { path.get(k).map_or([0.0; 3], |p| [p[0], p[1], p[2]]) };
        // `0x423a58`.
        let rng = &mut ctx.game.rng;
        g.phase_a = rng.rand_angle();
        g.phase_b = rng.rand_angle();
        g.phase_c = add_rot(g.phase_c, f32::from_bits(0x3d56_7770));
        s.core[0] = point(0);
        s.glow[0] = point(0);
        let mut lines = Vec::new();
        for k in 1..POINTS {
            let (j1, j2, j3) = (rng.randf_signed(0.0, 0.3), rng.randf_signed(0.0, 0.3), rng.randf_signed(0.0, 0.3));
            let mut p = point(k);
            p[2] += j1;
            p[0] += j2;
            p[1] += j3;
            s.core[k] = p;
            let d = sub(s.core[k], s.core[k - 1]);
            let u4 = wrap(g.phase_b + k as f32 * f32::from_bits(0x3f7e_adb0));
            let u5 = wrap(g.phase_c + k as f32 * f32::from_bits(0x3e56_7751));
            let up = 0.8 * u5.sin() * u4.sin() + rng.randf_signed(0.0, 0.1);
            let yaw = add_rot(atan2(d[0], d[1]), f32::from_bits(0x3fc9_0fdc));
            let across = 0.5 * u4.cos();
            s.glow[k] = add(add(p, [0.0, 0.0, up]), [yaw.cos() * across, yaw.sin() * across, 0.0]);
            if 1 < k && (k == POINTS - 1 || k & 1 == 0) { lines.push((point(k - 2), point(k))); }
        }
        for k in POINTS - 4..POINTS { s.core[k] = point(k); }
        w.mobys[id].set_pv_i(0x2c, POINTS as i32);
        let mut t = crate::moby::hits::HitTemplate::new(0.0, Some(id), 1, [0.0; 4]);
        t.kind = (w.mobys[id].o_class as u16 as u32) << 16;
        for (p0, p1) in lines {
            let Some(o) = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, p0, p1, crate::collision::QueryFlags(0x11), Some(id))) else { continue };
            if let Some(k) = o.moby { w.deliver_hit_any(k, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
            if o.moby.is_some() && o.moby == ctx.hero_moby {
                ctx.hero.gear.lift = 0.0;
                ctx.hero.gear.lift_store = 0.0;
            }
        }
        w.register_draw(id);
    }
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_i(0x3c, 0);
        m.state = 1;
        m.set_pv_i(0x30, -1);
        m.set_pv_i(0x2c, POINTS as i32);
        m.set_pv_i(0x48, 0x7f_7f7f);
        m.set_pv_i(0x4c, 0x80_3040);
        m.set_pv_f(0x40, 0.05);
        m.set_pv_f(0x44, 0.4);
    }
}

/// `0x423f60`'s textures and the strands' ends (`$gp+0x2658` / `+0x265c`, `+0x2630` / `+0x2634`); their colours are
/// the strand's own (`+0x48` / `+0x4c`).
const LOOK: crate::fx::trails::StrandLook = crate::fx::trails::StrandLook { fx: [0xe, 0x10], rgb: [0x7f_7f7f, 0x80_3040], ends: [0x7f_7f7f, 0x80_3040] };

/// `0x423e80`: the scroll back 0.3 a frame (`$gp+0x262c`, held above −8), then its row's strand
/// (`0x423f60(+0x40, +0x44, 0.9, strand, 0x80, 0x40, points − 2)`) and glow strand (`(0.1, 0.3, 0.1, glow strand,
/// 0x40, 0x40, points)`), coloured `+0x48` / `+0x4c` (crate::fx::trails::strand).
pub fn draw(w: &mut MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let g = &mut w.fence14;
    let t = g.scroll - 0.3;
    g.scroll = if t <= -8.0 { t + 8.0 } else { t };
    let m = &w.mobys[id];
    let n = m.pv_i(0x2c);
    let look = crate::fx::trails::StrandLook { rgb: [m.pv_i(0x48) as u32, m.pv_i(0x4c) as u32], ..LOOK };
    let Some(s) = g.strands.get(&m.pv_i(0x38)) else { return };
    crate::fx::trails::strand(out, &s.core, n - 2, g.scroll, m.pv_f(0x40), m.pv_f(0x44), 0.9, [0x80, 0x40], &look, cam);
    crate::fx::trails::strand(out, &s.glow, n, g.scroll, 0.1, 0.3, 0.1, [0x40, 0x40], &look, cam);
}
