//! The strand curtains: an energy barrier **0x08a0** (L06 `0x401b40`, its draw `0x401f00`; L13 `0x3c6108` /
//! `0x3c64c8`; L17 `0x3c3ab8` / `0x3c3e78`; the decomp's `func_L06_004005F8` / `func_L06_004009B8`; 1 placed on
//! Notak, 1 on Siberius, 12 on Grelbin) drawn from the jittering strands of a field **0x0431** (L06 `0x3e7268`, L13
//! `0x3ae168`, L17 `0x3a68a8`; the decomp's `func_L06_003E5D30`, asm only; one a level). R&C1's Orxon has the same
//! pair (its classes 1067 / 1073). A gate (crate::moby::classes::level06::gate, 0x031a and its kin) switches a
//! barrier off (state 2) and deletes it.
//!
//! **The field 0x0431** keeps 4 layers × 10 strands × 20 points (positions `+0x00` + layer·0x2580 + strand·0x10 +
//! point·0x140, their drifts at `+0x9600` the same way, the strands' timers at `+0x12c00` + layer·0x78 + strand·4, a
//! scroll phase at `+0x12de0`).
//! * **0**: the lifetime 45 and the fade point 22 into the globals `0x1b31b0` / `0x1b31b4`; the timers staggered
//!   (`strand·45/10` + 0 / 11 / 23 / 34 by layer); 1.
//! * **1**: the phase += 2/60 (`$gp+0x6d8`, wrapped past 1). Every strand's timer counts down (`0x32e0a0` = L00
//!   `0x2eeab0`, crate::moby::countdown): running, its points 1..18 drift; out, the strand is laid straight along y
//!   from −1.15 to 1.15 (20 points, `$gp+0x6d4` 2.3 long), each point drawn a `rand_vec(0, 0.4/60)` (`0x35bc80` =
//!   L00 `0x31bbc0`, `$gp+0x6d0`), its inner points' drifts the mean of three neighbouring draws, the timer 45.
//!
//! **A barrier 0x08a0** (pvars: `+0x00` the width, `+0x04` the rows, `+0x08` their spacing, `+0x0c` the field (−1:
//! a debug print and deleted), `+0x10..` a strand per row, `+0x30` its hum, `+0x34` the fade-out timer):
//! * **0**: the strand list 0..9 (`0x1affb0`) shuffled (10 passes of swapping forward by `randi(9)`, the draw made
//!   anew each step) into its rows; its yaw a quarter turn on (`0x32f438`); update distance 0xff; mode | 0x41 (not
//!   drawn: the draw is the barrier); the hum −1; the fade-out timer `0.45·60` (`$gp+0xee0`); 1.
//! * **1**: within 80 of the camera (`0x1b9840`) and its sphere (its middle, `rows·spacing/2` up; radius the half
//!   diagonal) in view (`0x32f658` = L00 `0x2f0080`): its draw on the second list (`0x327080` = L00 `0x2e7920`). Its
//!   hum (class sound 0, flags 4; `0x3a1c98`) kept playing.
//! * **2** (switched off): the fade-out timer down; the hum stopped; the draw as in 1.
//!
//! Not ported: both classes' deletion on level 10 while Clank is the hero (`0x18c0d4` = 1): neither is placed there.
//!
//! **The draw** ([`draw`]): per layer, per segment 0..18 (s stepping 0.25 a segment, wrapped past 1), per row (the
//! rows 1..n spacing above the barrier; the barrier's rows scaled by width/2): a quad from the segment's two points of
//! the row's strand, ±0.05 along local z (`$gp+0xe60`), ST (s, 0) (s, 1) (s + 0.25, 0) (s + 0.25, 1) plus the phase
//! (`$gp+0xe38`); FX 0xe, additive (`$gp+0xe10` 0, 2, 0, 1). The colour by the strand's age (`0x2f0000`,
//! crate::hud::lerp_rgba, (t − 22)/23 above 22, else 1 − t/22, from 0x7fc04070 to 0x00ff0000), its alpha · fade² (1
//! within 48 of the camera, to 0 at 80); switched off, that alpha · timer / 27.

use crate::fx::FxQuad;
use crate::hero::physics::{dist, V3};
use crate::moby::attach::rows_of;
use crate::moby::{countdown, Ctx, MobyWorld};

pub const BARRIER: i16 = 0x08a0;
pub const FIELD: i16 = 0x0431;

const FIELD_LEN: usize = 0x12de4;
const VEL: usize = 0x9600;
const TIMERS: usize = 0x12c00;
const PHASE: usize = 0x12de0;
const LIFE: i32 = 45;
const MID: i32 = 22;
const LENGTH: f32 = 2.3;
const DRIFT: f32 = 0.4;
const SCROLL: f32 = 2.0;
const SEG_S: f32 = 0.25;
const OFF: f32 = 0.45;
const COLOURS: (u32, u32) = (0x7fc0_4070, 0x00ff_0000);
const DT: f32 = 1.0 / 60.0;
const HUM: usize = 0x30;
const FADE_OUT: usize = 0x34;

fn timer_at(layer: usize, strand: usize) -> usize { TIMERS + layer * 0x78 + strand * 4 }
fn point_at(base: usize, layer: usize, strand: usize, pt: usize) -> usize { base + layer * 0x2580 + strand * 0x10 + pt * 0x140 }

/// 0x0431 (L06 `0x3e7268`): the field (module doc).
pub fn update_field(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.pvars.len() < FIELD_LEN { m.pvars.resize(FIELD_LEN, 0); }
    match m.state {
        0 => {
            for (layer, base) in [0, 11, 23, 34].into_iter().enumerate() {
                for s in 0..10 { m.set_pv_i(timer_at(layer, s), s as i32 * LIFE / 10 + base); }
            }
            m.state = 1;
        }
        1 => {
            let mut ph = m.pv_f(PHASE) + SCROLL * DT;
            if 1.0 < ph { ph -= 1.0; }
            m.set_pv_f(PHASE, ph);
            for layer in 0..4 {
                for s in 0..10 {
                    let mut t = m.pv_i(timer_at(layer, s));
                    let out = countdown(&mut t);
                    m.set_pv_i(timer_at(layer, s), t);
                    if !out {
                        for pt in 1..19 {
                            let (a, v) = (m.pv_v(point_at(0, layer, s, pt)), m.pv_v(point_at(VEL, layer, s, pt)));
                            m.set_pv_v(point_at(0, layer, s, pt), std::array::from_fn(|k| a[k] + v[k]));
                        }
                        continue;
                    }
                    let mut draws = [[0.0f32; 3]; 20];
                    for (pt, d) in draws.iter_mut().enumerate() {
                        let y = pt as f32 * 0.05 * LENGTH - LENGTH * 0.5;
                        m.set_pv_v(point_at(0, layer, s, pt), [0.0, y, 0.0]);
                        m.set_pv_f(point_at(0, layer, s, pt) + 0xc, 1.0);
                        *d = ctx.game.rng.rand_vec(0.0, DRIFT * DT);
                    }
                    for pt in 1..19 {
                        let v = std::array::from_fn(|k| (draws[pt - 1][k] + draws[pt][k] + draws[pt + 1][k]) * f32::from_bits(0x3eaa_7efa));
                        m.set_pv_v(point_at(VEL, layer, s, pt), v);
                    }
                    m.set_pv_i(timer_at(layer, s), LIFE);
                }
            }
        }
        _ => {}
    }
}

fn field_of(w: &MobyWorld, id: usize) -> Option<usize> {
    usize::try_from(w.mobys[id].pv_i(0xc)).ok().filter(|&f| f < w.mobys.len())
}

/// States 1 and 2: the draw registered when near and in view.
fn queue(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let h = m.pv_i(4) as f32 * m.pv_f(8) * 0.5;
    let r = (h * h + m.pv_f(0) * 0.5 * m.pv_f(0) * 0.5).sqrt();
    let sphere = [m.pos[0], m.pos[1], m.pos[2] + h, r];
    if dist(m.pos, ctx.host.camera_pos()) < 80.0 && !ctx.view.is_some_and(|v| v.culled(80.0, sphere)) { w.register_draw(id); }
}

/// 0x08a0 (L06 `0x401b40`): a barrier (module doc).
pub fn update_barrier(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(0xc) == -1 {
        w.delete(id);
        return;
    }
    match w.mobys[id].state {
        0 => {
            let mut order: [u8; 10] = std::array::from_fn(|i| i as u8);
            for _ in 0..10 {
                let mut j = 0;
                while j < ctx.game.rng.randi(9) as usize {
                    order.swap(j, j + 1);
                    j += 1;
                }
            }
            let m = &mut w.mobys[id];
            let n = m.pv_i(4).clamp(0, 0x20) as usize;
            if m.pvars.len() < 0x38 { m.pvars.resize(0x38, 0); }
            for (k, &v) in order.iter().take(n).enumerate() { m.pvars[0x10 + k] = v; }
            m.rot[2] = crate::hero::physics::add_rot(m.rot[2], std::f32::consts::FRAC_PI_2);
            m.update_dist = 0xff;
            m.state = 1;
            m.set_pv_i(HUM, -1);
            m.set_pv_i(FADE_OUT, (OFF * 60.0) as i32);
            m.mode |= 0x41;
        }
        1 => {
            queue(w, id, ctx);
            let h = w.mobys[id].pv_i(HUM);
            if !(h != -1 && ctx.host.sound_playing(h)) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(HUM, h);
            }
        }
        2 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(FADE_OUT);
            countdown(&mut t);
            m.set_pv_i(FADE_OUT, t);
            let h = m.pv_i(HUM);
            if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
            w.mobys[id].set_pv_i(HUM, -1);
            queue(w, id, ctx);
        }
        _ => {}
    }
}

/// L06 `0x401f00`: the barrier's quads (module doc).
pub fn draw(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let Some(f) = field_of(w, id).map(|f| &w.mobys[f]).filter(|f| f.pvars.len() >= FIELD_LEN) else { return };
    let d = dist(m.pos, cam);
    let fade = if 48.0 < d { (80.0 - d) * 0.031_25 } else { 1.0 };
    let (width, rows, sp) = (m.pv_f(0), m.pv_i(4).clamp(0, 0x20) as usize, m.pv_f(8));
    let r = rows_of(m.rot).map(|row| row.map(|x| x * width * 0.5));
    let phase = f.pv_f(PHASE);
    for layer in 0..4 {
        let mut seg_s = 0.0f32;
        for seg in 0..19 {
            seg_s += SEG_S;
            if 1.0 < seg_s { seg_s -= 1.0; }
            let (s0, s1) = (seg_s + phase, SEG_S + seg_s + phase);
            for row in 0..rows {
                let strand = m.pv_b(0x10 + row) as usize % 10;
                let t = f.pv_i(timer_at(layer, strand));
                let k = if MID < t { (t - MID) as f32 / (LIFE - MID) as f32 } else { 1.0 - t as f32 / MID as f32 };
                let c = crate::hud::lerp_rgba(k, COLOURS.0, COLOURS.1);
                let mut a = (fade * fade * (c >> 24) as f32) as i32;
                if m.state == 2 { a = (a as f32 * m.pv_i(FADE_OUT) as f32 / (OFF * 60.0)) as i32; }
                let rgba = (c & 0xff_ffff).wrapping_add((a as u32) << 24);
                let base = [m.pos[0], m.pos[1], m.pos[2] + (row + 1) as f32 * sp];
                let corner = |pt: usize, dz: f32| -> V3 {
                    let l = f.pv_v(point_at(0, layer, strand, pt));
                    let v = [l[0], l[1], l[2] + dz];
                    std::array::from_fn(|i| v[0] * r[0][i] + v[1] * r[1][i] + v[2] * r[2][i] + base[i])
                };
                out.push(FxQuad {
                    fx: 0xe,
                    additive: true,
                    corners: [corner(seg, -0.05), corner(seg, 0.05), corner(seg + 1, -0.05), corner(seg + 1, 0.05)],
                    st: [[s0, 0.0], [s0, 1.0], [s1, 0.0], [s1, 1.0]],
                    rgba: [rgba; 4],
                });
            }
        }
    }
}
