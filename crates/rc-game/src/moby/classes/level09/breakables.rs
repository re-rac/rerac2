//! Dobbo's breakables (L09 addresses).
//!
//! * **0x1298** ([`update_1298`], `0x423080`, 109 placed; the decomp's `func_L09_00421B80`):
//!   * **0**: a glow on it at (0, −0.14, 0.14) (`0x33c290` = L00 `0x33f550`: 0x50088030, 136500 across, pulled
//!     0.75, while it lives); then 1.
//!   * **1**: a hit with the flags 0x4010000 while `+0x68`[0] is 0 moves it to 2. The hit record is let go every
//!     tick.
//!   * **2**: it breaks. 25 puffs of kind 2 at the same point (`0x2e90d8`: 0.2 out, 1/12 a second, size 1.5,
//!     sinking 6, life 0.8, drifting (0, −0.05, 1/60)); then all its pieces (`0x327e00`, ranges `$gp+0x2c70` /
//!     `+0x2c68` / `+0x2c60`, every piece a shard, sound 0, flags 0: the level's bolts); then its stub 0x1299 in its
//!     place (`0x323ff0`). Then it is deleted.

use crate::moby::{shards, Ctx, MobyWorld};

pub const CRYSTAL: i16 = 0x1298;
const STUB: i16 = 0x1299;

const BREAK_FLAGS: u32 = 0x0401_0000;
const AT: [f32; 3] = [0.0, -0.14, 0.14];
/// `$gp+0x2c70` / `+0x2c68` / `+0x2c60`: the pieces' velocity ranges, spin and gravity.
const RANGE: [f32; 8] = [-1.5, -2.0, 3.0, 0.0, 1.5, -1.0, 6.0, 0.0];
const SPIN: (f32, f32) = (90.0, 360.0);
const GRAVITY: (f32, f32) = (10.0, 20.0);

pub fn update_1298(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let (pos, far) = (w.mobys[id].pos, w.mobys[id].draw_dist as i16);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow(sys, &mut ctx.game.rng, 136_500.0, 0.75, AT, id, pos, far, false, 0x5008_8030, -1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & BREAK_FLAGS != 0) && w.mobys[id].pv_i(0) == 0 {
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            let m = &w.mobys[id];
            let (rows, pos, scale) = (m.rows, m.pos, m.scale);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::staged_burst(sys, &mut ctx.game.rng, 0.2, 0.083_333_336, 1.5, 6.0, 0.8, &rows, pos, scale, AT, [0.0, -0.05, 0.016_666_668], 25, 2);
            }
            shards::throw_all(w, id, ctx, RANGE, SPIN, GRAVITY, 0xffff, 0, -1, 0, 0xffff);
            crate::moby::classes::props::spawn_in_place(w, id, STUB, 0, &*ctx.host);
            w.delete(id);
        }
        _ => {}
    }
}

pub const BIG_CRYSTAL: i16 = 0x10e7;

/// 0x10e7's `$gp+0x2b10` / `+0x2b08` / `+0x2b00`: its pieces' velocity ranges, spin and gravity (`+0x2b30`, the
/// piece left standing, is −1: none).
const BIG_RANGE: [f32; 8] = [-4.0, -4.0, 1.0, 0.0, 4.0, 4.0, 7.0, 0.0];

/// A point in the moby's frame, scaled (`0x322558` = L00 `0x326190`).
fn scaled_point(w: &MobyWorld, id: usize, v: [f32; 3]) -> [f32; 3] {
    let m = &w.mobys[id];
    let r = &m.rows;
    std::array::from_fn(|k| (v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) * m.scale + m.pos[k])
}

/// 0x10e7 (`0x41eb58`, 25 placed; the decomp's `func_L09_0041D650`): a big crystal.
///
/// * **0**: a glow at (0, 5, 18) times its scale (`0x33c290`: 0x28608020, 1470000 × its scale across, def 0, far
///   0x60, while it lives); then 1.
/// * **1**: a hit with the flags 0x4010000 while `+0x68`[0] is 0 moves it to 2. The hit record is let go every
///   tick.
/// * **2**: it breaks.
///   * Its pieces fly (mask 0x87f, every piece a shard, sound 0, flags 0: the level's bolts).
///   * At (0, 5, 18) in its scaled frame:
///     * the blast (size its scale, 12 of each sprite, 12 puffs, 12 streaks, its flash ball);
///     * the general explosion (`0x31c9a8`: 20 embers, size 0.75, near 9, flags 0x810001);
///     * ten pointed chunks moving 1/15 a tick along its second row (reach 0.25, 5 a second, 0.75..1.5,
///       0x7faaaa40);
///     * 60 puffs of kind 3 (2 × its scale out, 0.2 a second, size 1, sinking 5, life 1, drifting (0, 0.2, −1/12)).
///   * Then it is deleted.
///
/// Not ported: its beam glow (`0x429cc0` = L03 `0x42cdd0`, class 0x13ba, G-REN-019).
pub fn update_10e7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let at = [0.0, 5.0, 18.0];
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (pos, s) = (m.pos, m.scale);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow_def(sys, &mut ctx.game.rng, s * 1_470_000.0, 0.0, at.map(|x| x * s), id, pos, 0x60, false, 0x2860_8020, -1, 0);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & BREAK_FLAGS != 0) && w.mobys[id].pv_i(0) == 0 {
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            shards::throw_all(w, id, ctx, BIG_RANGE, SPIN, GRAVITY, 0xffff, 0, -1, 0, 0x87f);
            let p = scaled_point(w, id, at);
            let s = w.mobys[id].scale;
            crate::moby::blast::burst(w, ctx, p, &crate::moby::blast::Blast { size: s, bursts: [12, 12, 12], puffs: 12, streaks: 12, light: true, shake: false });
            let e = crate::moby::blast::Explosion { near: 9.0, size: 0.75, embers: 20, sound: -1, lod: -1, flags: 0x81_0001, ..Default::default() };
            crate::moby::blast::explode(w, ctx, Some(id), Some(p), &e);
            let r = w.mobys[id].rows;
            let v = std::array::from_fn(|k| 0.066_666_67 * r[1][k]);
            shards::throw_chunks(w, ctx, 0.25, 5.0, (0.75, 1.5), id, p, 10, true, v, 0x7faa_aa40, None);
            let m = &w.mobys[id];
            let (rows, pos) = (m.rows, m.pos);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::staged_burst(sys, &mut ctx.game.rng, s * 2.0, 0.2, 1.0, 5.0, 1.0, &rows, pos, s, at, [0.0, 0.2, -0.083_333_336], 60, 3);
            }
            w.delete(id);
        }
        _ => {}
    }
}
