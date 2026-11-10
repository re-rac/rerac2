//! The breakables whose code several levels share. Each waits in state 1 for a hit with the flags 0x4010000 (L13
//! `0x327f60`, L00 `0x31dce8`) and breaks in state 2; the hit record is let go every tick of state 1.
//!
//! * **0x1293** ([`update_1293`], L04 `0x457ef8`, 12 placed; L20 `0x44aaf0`, 9; the decomp's `func_L04_00456748`):
//!   the level's bolts and thirty debris chunks.
//! * **0x0852** / **0x0884** ([`update_0852`], L12 `0x3bd440`, 4 and 4 placed; L09 `0x3b78c0`, 0x0852 only, 5;
//!   `func_L09_003B6550`): animated from a random frame; 0x0852 breaks into the pieces of 0x12a1 with a blast and
//!   chunks, 0x0884 never does.
//! * **0x130b** ([`update_130b`], L13 `0x38db60`, 37 placed; the museum `0x3855e0`, 3; `func_L13_0038C718`): every
//!   piece, the level's bolts.
//! * **0x130f** ([`update_130f`], L13 `0x38de68`, 13 placed; the museum `0x385be8`, 8; `func_L13_0038CA20`): every
//!   piece, the bolts and a blast.
//!
//! Level 13's own 0x130a and 0x130d (crate::moby::classes::level13::breakables) take [`hit`], [`throw`] and
//! [`blast_at`] from here.
//!
//! The engine functions (L13; L12 / L04 where noted): `0x3345e0` = L00 `0x32b580` ([`shards::throw_all`]),
//! `0x330570` = L00 `0x327388` ([`bolts::level_drop`]), `0x3fa180` (L12 `0x3fb0a0`) = L00 `0x3f8348`
//! ([`blast::burst`]), L12 `0x32aaf0` / L04 `0x349c68` = L00 `0x327648` ([`shards::throw_chunks`]), L12 `0x32b088` =
//! L00 `0x327898` (crate::moby::classes::props::spawn_in_place), L12 `0x31ef20` = L00 `0x31b518` the sequence at a
//! frame.

use crate::hero::physics::{add, set_len, V3};
use crate::moby::blast::{self, Blast};
use crate::moby::{bolts, shards, Ctx, MobyWorld};

/// The hit flags the breakables answer (`0x327f60(m, 0x4010000, 0)`).
pub const BREAK_FLAGS: u32 = 0x0401_0000;

/// State 1: a hit with [`BREAK_FLAGS`] while `+0x68`[0] = 0 breaks it; the record let go.
pub fn hit(w: &mut MobyWorld, id: usize) -> bool {
    let h = w.hit_message(id).is_some_and(|h| h.tmpl.flags & BREAK_FLAGS != 0) && w.mobys[id].pv_i(0) == 0;
    w.clear_hit(id);
    h
}

/// The blast the breakables give (`0x3fa180(size, 20, at, 0, 0x19, 0x14, 0x14, 0x14, 0x2d, 1, 1, 1, 0, 0)`): sprites
/// 0x19 / 0x14 / 0x14, 0x14 puffs, 0x2d streaks, its flash ball, no shake.
pub fn blast_at(w: &mut MobyWorld, ctx: &mut Ctx, at: V3, size: f32) {
    blast::burst(w, ctx, at, &Blast { size, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
}

/// `$gp−0x430` / `−0x438` / `−0x440` (L13): 0x130b's pieces' velocity ranges (x / y −3..3, z 3..7), spin and gravity.
const H_130B: [f32; 8] = [-3.0, -3.0, 3.0, 0.0, 3.0, 3.0, 7.0, 0.0];
/// `$gp−0x3f0`: 0x130f's (as 0x130b's).
const H_130F: [f32; 8] = [-3.0, -3.0, 3.0, 0.0, 3.0, 3.0, 7.0, 0.0];
/// Every one's spin (90..360) and gravity (10..20).
const SPIN: (f32, f32) = (90.0, 360.0);
const GRAVITY: (f32, f32) = (10.0, 20.0);

/// The break of 0x130b / 0x130a / 0x130f: every piece pointed (`0x3345e0(m, h, spin, gravity, 0xffff, 0, −1, 0,
/// 0xffff, 0xff, 0)`: its class sound 0, the level's bolts once).
pub fn throw(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, h: [f32; 8]) {
    shards::throw_all(w, id, ctx, h, SPIN, GRAVITY, 0xffff, 0, -1, 0, 0xffff);
}

/// 0x130b (L13 `0x38db60`). **0**: 1 (`0x325bd0`). **1**: the hit ([`hit`]) → 2. **2**: its pieces ([`throw`],
/// `$gp−0x430`); deleted.
pub fn update_130b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => if hit(w, id) { w.mobys[id].set_state(2, -1) },
        2 => {
            throw(w, id, ctx, H_130B);
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x130f (L13 `0x38de68`). **0**: 1. **1**: the hit ([`hit`]) → 2. **2**: its pieces ([`throw`], `$gp−0x3f0`), the
/// blast of size 0.3 ([`blast_at`]) at its place; deleted.
pub fn update_130f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => if hit(w, id) { w.mobys[id].set_state(2, -1) },
        2 => {
            throw(w, id, ctx, H_130F);
            let pos = w.mobys[id].pos;
            blast_at(w, ctx, pos, 0.3);
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x1293 (L04 `0x457ef8`): **0** → 1. **1**: the hit ([`hit`], L04 `0x33f8d0`) → 2. **2**: the level's bolts
/// (`0x3499a8` = L00 `0x327388`, crate::moby::bolts::level_drop, height 0), its sound 0, thirty debris chunks ([`chunks_1293`], `0x458010`), deleted.
pub fn update_1293(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => if hit(w, id) { w.mobys[id].set_state(2, -1) },
        2 => {
            bolts::level_drop(w, id, 0.0, 1, ctx);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            chunks_1293(w, id, ctx, 0.4, f32::from_bits(0x408c_cccd), 1.0, f32::from_bits(0x3daa_aaab), 0x1e);
            w.delete(id);
        }
        _ => {}
    }
}

/// `0x458010(lo, hi, r, speed, m, n)`: `n` debris chunks (`0x349c68` = L00 `0x327648`, crate::moby::shards::
/// throw_chunks: pointed, 1..3.5 of the class's size, colour `0x80402000`), each `r` out a random way and `lo..hi`
/// up from its place, moving `speed`·0.75..1.25 a tick out along the way with 0..1 up.
#[allow(clippy::too_many_arguments)]
fn chunks_1293(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lo: f32, hi: f32, r: f32, speed: f32, n: i32) {
    for _ in 0..n {
        let rng = &mut ctx.game.rng;
        let a = rng.randf(-std::f32::consts::PI, std::f32::consts::PI);
        let mut d = [a.cos() * r, a.sin() * r, rng.randf(0.0, 1.0)];
        let vel = set_len(d, rng.randf(speed * 0.75, speed * 1.25));
        d[2] = rng.randf(lo, hi);
        let at = add(w.mobys[id].pos, d);
        shards::throw_chunks(w, ctx, 0.0, 0.0, (1.0, 3.5), id, at, 1, true, vel, 0x8040_2000, None);
    }
}

/// `$gp+0xa90` / `+0xa98` / `+0xaa0`: 0x12a1's pieces' gravity (10..20), spin (90..360) and velocity ranges (x / y
/// 4..10 of either sign, z 8..12).
const CAN_GRAVITY: (f32, f32) = (10.0, 20.0);
const CAN_SPIN: (f32, f32) = (90.0, 360.0);
const CAN_H: [f32; 8] = [4.0, 4.0, 8.0, 0.0, 10.0, 10.0, 12.0, 0.0];

/// 0x0852 / 0x0884 (L12 `0x3bd440`). **0**: its sequence 0 from a random frame of it (`0x31ef20`); 1. **1**: 0x0852
/// only: a hit with [`BREAK_FLAGS`] (no `+0x68` test): its class sound 0, 2; the record let go. **2**: a 0x12a1 in
/// its place (`0x32b088`) thrown into all its pieces (flags 0x16: x / y of either sign, the level's bolts and nine
/// times them again; no sound); the blast at its place (`0x3fb0a0`: size 1, sprites 0x19 / 0x14 / 0x14, 0x14 puffs,
/// 0x2d streaks, its flash ball, the shake); 0x14 chunks (`0x32aaf0`: reach 6, 3 a second, 1.5..7 of their size, balls,
/// (0, 0, 0.2) a tick, colour 0x80804040); the 0x12a1 and it deleted.
pub fn update_0852(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let n = ctx.host.seq_frames(&w.mobys[id], 0).unwrap_or(0) as i32;
            let f = ctx.game.rng.randi(n) as u8;
            ctx.host.set_anim(&w.mobys[id], 0, f, 0);
            w.mobys[id].set_state(1, -1);
        }
        1 => if w.mobys[id].o_class == 0x0852 {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & BREAK_FLAGS != 0) {
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        },
        2 => {
            let c = super::props::spawn_in_place(w, id, 0x12a1, 0, &*ctx.host);
            shards::throw_all(w, c, ctx, CAN_H, CAN_SPIN, CAN_GRAVITY, 0xffff, 0, -1, 0x16, 0xffff);
            let pos = w.mobys[id].pos;
            blast::burst(w, ctx, pos, &Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true });
            shards::throw_chunks(w, ctx, 6.0, 3.0, (1.5, 7.0), id, pos, 0x14, false, [0.0, 0.0, f32::from_bits(0x3e4c_ccce)], 0x8080_4040, None);
            w.delete(c);
            w.delete(id);
        }
        _ => {}
    }
}
