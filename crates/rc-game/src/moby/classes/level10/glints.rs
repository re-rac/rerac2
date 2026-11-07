//! Hrugis's cloud glints **0x11b9** (`0x42d240`, one placed; the decomp's `func_L10_0042B500`): green glints on a tie
//! group's clouds, lit where a slowly sweeping direction from the moby crosses them.
//!
//! * **0**: without its tie group (`+0x68`[0]: out of range, −1 or empty) deleted (`0x32c1d0`). Its clock `[1]` 0,
//!   its step `[2]` 1/60, updated at any distance. Eighteen points: (0, 10, −4), (0, 30, −3.5), (3.5, 0, −2) and
//!   their turns by 120° and −120° about z, the last one's turn by 60° and that point's turns by ±120° too (points
//!   0..11), then (15, 30, 2), (15, 15, 2), (15, 0, 2), (30, 15, 2.5), (60, −15, 8), (60, −30, 10) (12..17). For each
//!   tie of the group six of them through the tie's rows (`0x309d58` = L00 `0x2ef318`) and onto its place, into the
//!   level's buffer of 1056 (`0x26ce20`, [`crate::moby::MobyWorld::glints10`]): from 0 for the tie class 0xdf6, 6 for
//!   0xdf7, else 12 (the class through `0x227340`). Then 1.
//! * **1**: the clock up a step; for each of the 1056 points in front of the camera (along its forward row, the
//!   camera's `+0x230`) the direction `(sin b cos a, sin b sin a, cos b)`, `a` = wrap(clock · π/8), `b` = wrap(clock ·
//!   0.5812) (`0x33a9b8` = L00 `0x320440`), and the angle between it and the way from the moby to the point (`π/2 −
//!   asin`): 80°..84° eased in, 84°..95° full, 95°..130° eased out (`0x42d9b8`: `x²(3 − 2x)`), else none. A lit point
//!   puts a spot (crate::particles::spot, type 0x67, `0x3559e8`) 4 toward the camera, size 2100000, colour 0x33aa00 at
//!   alpha 32 · the light.

use crate::hero::physics::{add, dot, set_len, smooth_step, sub, through, unit, wrap, V3};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x11b9;
/// The buffer's size (`0x26ce20`).
const POINTS: usize = 1056;

/// The eighteen points (module docs).
fn points() -> [V3; 18] {
    let turn = |a: u32| crate::moby::attach::rows_of([0.0, 0.0, f32::from_bits(a)]);
    let (m0, m1, m2) = (turn(0x3f86_0a93), turn(0x4006_0a93), turn(0xc006_0a93));
    let mut p = [[0.0; 3]; 18];
    p[0] = [0.0, 10.0, -4.0];
    p[1] = through(&m1, p[0]);
    p[2] = through(&m2, p[0]);
    p[3] = [0.0, 30.0, -3.5];
    p[4] = through(&m1, p[3]);
    p[5] = through(&m2, p[3]);
    p[6] = [3.5, 0.0, -2.0];
    p[7] = through(&m1, p[6]);
    p[8] = through(&m2, p[6]);
    p[9] = through(&m0, p[6]);
    p[10] = through(&m1, p[9]);
    p[11] = through(&m2, p[9]);
    p[12] = [15.0, 30.0, 2.0];
    p[13] = [15.0, 15.0, 2.0];
    p[14] = [15.0, 0.0, 2.0];
    p[15] = [30.0, 15.0, 2.5];
    p[16] = [60.0, -15.0, 8.0];
    p[17] = [60.0, -30.0, 10.0];
    p
}

/// 0x11b9 (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => set_up(w, id),
        1 => glint(w, id, ctx),
        _ => {}
    }
}

fn set_up(w: &mut MobyWorld, id: usize) {
    let Some(ties) = usize::try_from(w.mobys[id].pv_i(0)).ok().and_then(|g| w.tie_groups.get(g)).filter(|g| !g.is_empty()).cloned() else {
        w.delete(id);
        return;
    };
    let m = &mut w.mobys[id];
    m.set_pv_f(4, 0.0);
    m.set_pv_f(8, f32::from_bits(0x3c88_8889));
    m.update_dist = 0xff;
    let p = points();
    let mut out = std::mem::take(&mut w.glints10);
    out.clear();
    for t in ties {
        let t = t as usize;
        let Some(mat) = w.ties.get(t) else { break };
        let r: [V3; 3] = std::array::from_fn(|k| [mat[k][0], mat[k][1], mat[k][2]]);
        let at = [mat[3][0], mat[3][1], mat[3][2]];
        let k = match w.tie_classes.get(t).copied() { Some(0xdf6) => 0, Some(0xdf7) => 6, _ => 12 };
        for i in 0..6 {
            if out.len() < POINTS { out.push(add(at, through(&r, p[i + k]))); }
        }
    }
    // The buffer's other points stay as the level load left them (0).
    out.resize(POINTS, [0.0; 3]);
    w.glints10 = out;
    w.mobys[id].state = 1;
}

fn glint(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let t = m.pv_f(4) + m.pv_f(8);
    m.set_pv_f(4, t);
    let pos = m.pos;
    let cam = ctx.host.camera_pos();
    let fwd = ctx.host.camera_rows()[0];
    let (lower, low_end, high_start, upper) = (f32::from_bits(0x3fb2_b8c4), f32::from_bits(0x3fbb_a867), f32::from_bits(0x3fd4_3b68), f32::from_bits(0x4011_361f));
    for &p in &w.glints10 {
        if dot(fwd, sub(p, cam)) <= 0.0 { continue; }
        let a = wrap(t * f32::from_bits(0x3ec9_0fdc));
        let b = wrap(t * f32::from_bits(0x3f14_c92c));
        let dir = [b.sin() * a.cos(), b.sin() * a.sin(), b.cos()];
        let way = unit(sub(p, pos));
        let ang = std::f32::consts::FRAC_PI_2 - dot(way, dir).clamp(-1.0, 1.0).asin();
        let mut light = 0.0;
        if lower < ang {
            if ang <= low_end {
                light = smooth_step((ang - lower) / f32::from_bits(0x3d8e_fa30));
            } else if ang <= high_start {
                light = 1.0;
            } else if ang <= upper {
                light = 1.0 - smooth_step((ang - high_start) / f32::from_bits(0x3f1c_61ac));
            }
        }
        if light == 0.0 { continue; }
        let mut alpha = 32;
        if light != 1.0 { alpha = (light * alpha as f32) as i32; }
        let rgba = (alpha as u32) << 24 | 0x33_aa00;
        let at = add(p, set_len(sub(cam, p), 4.0));
        if let Some(sys) = ctx.parts.as_deref_mut() {
            crate::particles::spot::spot(sys, &mut ctx.game.rng, at, rgba, 2_100_000.0);
        }
    }
}
