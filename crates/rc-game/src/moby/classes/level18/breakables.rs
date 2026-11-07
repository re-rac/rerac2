//! Damosel's breakables (L18 addresses). All four break on a hit with the flags 0x4010000 while `+0x68`[0] is 0,
//! and let the hit record go every tick in their state 1 (`+0xa8` = 0xff). Their first pieces fly off along the hit
//! (or their side), 6..8 a second (`0x336a28`, the rng), ±3 across either way (flags 9: no bolts, in the world).
//!
//! * **0x122c / 0x122a / 0x122d** ([`update_122c`], `0x440ff8`, 99 / 77 / 28 placed, one code; the decomp's
//!   `func_L18_0043FB08`): on the hit its pieces 0 and 2 fly (mask `$gp+0x2c98` = 5) back along its second row
//!   (`+0xd0`), the ranges `$gp+0x2c68` / `+0x2c60`, every piece a shard (`+0x2c94`), none left (`+0x2c90` = −1);
//!   then its class sound 0 (`0x37c980` = L00 `0x360698`) and it goes (`0x32c688`). No state 2.
//! * **0x1184** ([`update_1184`], `0x43dc78`, 35 placed; `func_L18_0043C778`): **0** four glows 19.7 up its frame
//!   at ±2.6 along its rows (`$gp+0x2af0`, times its scale; `0x35ad10` = L00 `0x33f550`: 0x28808080, 1050000 × its
//!   scale across, pulled 0.15, def 0, far 0x60, while it lives); then 1. **1** on the hit (its direction's z set
//!   to 0.75 in the record) pieces 7 on fly (mask −0x80), → 2. **2** the rest (mask 0xfa7f, the ranges `$gp+0x2ac0`
//!   / `+0x2ab8` / `+0x2ab0`, sound 0, flags 6: x and y of either sign, the level's bolts), the blast 20 up its
//!   scaled frame (`0x340e40` = L00 `0x326190`; `0x415e70` = L00 `0x32c700`: size 2 × its scale, 5 of each sprite,
//!   5 puffs, 5 streaks, its flash ball), gone.
//! * **0x1186** ([`update_1186`], `0x43dfe8`, 17 placed; `func_L18_0043CAF8`): **0** → 1. **1** on the hit (z 0.75)
//!   piece 1 flies (mask 2), → 2. **2** the rest (mask 0x3d, the ranges `$gp+0x2b40` / `+0x2b38` / `+0x2b30`, sound
//!   0, flags 6), then 0..4 bits (`0x336990`) thrown from (0.35, 0, 1.8) in its frame (`0x340de8` = L01 `0x31a278`):
//!   8..12 a second ahead, `(2i − n + 1)/15` a tick across, 0..2 a second up, through its rows (`0x309b08`), each
//!   0.75..1.25 in size and life; gone.
//! * **0x12a5** ([`update_12a5`], `0x442540`, 8 placed; `func_L18_00441050`): **0** → 1. **1** on the hit (z 1)
//!   piece 0 flies (mask 1), → 2. **2** the rest (mask 0xfe, the ranges `$gp+0x2ca0` / `+0x2cc8` / `+0x2cc0`, sound
//!   0): the first time (bit `+0x68`[4] of the flag byte `0x139658` clear, or that index 8 or more) with the nine
//!   extra bolts (flags 0x10) and the bit set, else the level's bolts once (flags 0); the blast at it (size 10 × its
//!   scale, as 0x1184's); gone.
//!
//! Not ported: 0x1186's bits, mobys of class 0x11b7 (`0x43f760` = L06 `0x465358`, a class the port lacks, as at
//! level 06), with their random draws (the count, each one's speeds, size and life, and the spawner's own). The
//! blasts' screen flash (`0x32c700`'s
//! thirteenth argument, crate::moby::blast's `+0x38`). 0x1184's set-up also takes two frame points per glow
//! (`0x340e40`, `0x340ef8` = L17 `0x326f38`) that nothing reads.

use crate::hero::physics::{scale, V3};
use crate::moby::blast::{self, Blast};
use crate::moby::{shards, Ctx, MobyWorld};

pub const PANEL_A: i16 = 0x122a;
pub const PANEL_B: i16 = 0x122c;
pub const PANEL_C: i16 = 0x122d;
pub const GLOW_PANEL: i16 = 0x1184;
pub const BIT_PANEL: i16 = 0x1186;
pub const BOLT_PANEL: i16 = 0x12a5;

const BREAK_FLAGS: u32 = 0x0401_0000;
const SPIN: (f32, f32) = (90.0, 360.0);
const GRAVITY: (f32, f32) = (10.0, 20.0);
/// `$gp+0x2af0`: 0x1184's glows (times its scale).
const GLOWS: [V3; 4] = [[2.6, 0.0, 19.7], [-2.6, 0.0, 19.7], [0.0, 2.6, 19.7], [0.0, -2.6, 19.7]];
/// `$gp+0x2ac0`: 0x1184's last pieces' velocity ranges.
const GLOW_RANGE: [f32; 8] = [1.0, 1.0, 3.0, 0.0, 3.0, 3.0, 6.0, 0.0];
/// `$gp+0x2b40`: 0x1186's.
const BIT_RANGE: [f32; 8] = [1.0, 1.0, 3.0, 0.0, 3.0, 3.0, 9.0, 0.0];
/// `$gp+0x2ca0`: 0x12a5's.
const BOLT_RANGE: [f32; 8] = [-2.0, 2.0, 2.0, 0.0, 2.0, 4.0, 4.0, 0.0];
/// `0x139658` = `0x1395b8 + 0xa0` (crate::game::GameState::flags): 0x12a5's broken ones.
const BROKEN: usize = 0xa0;

/// The hit that breaks it (`0x338e08(m, 0x4010000, 0)`), while `+0x68`[0] is 0.
fn breaking_hit(w: &MobyWorld, id: usize) -> Option<crate::moby::hits::HitRecord> {
    w.hit_message(id).filter(|h| h.tmpl.flags & BREAK_FLAGS != 0 && w.mobys[id].pv_i(0) == 0)
}

/// The first pieces' range: `v ± (3, 3, 0)`.
fn spread(v: V3) -> [f32; 8] { [v[0] - 3.0, v[1] - 3.0, v[2], 0.0, v[0] + 3.0, v[1] + 3.0, v[2], 0.0] }

/// The hit's direction with its z set to `z` (in the record too), 6..8 along.
fn along_hit(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, h: &crate::moby::hits::HitRecord, z: f32) -> V3 {
    if let Some(r) = usize::try_from(w.mobys[id].hit_slot).ok().and_then(|s| w.hits.records.get_mut(s)) { r.tmpl.dir[2] = z; }
    let d = [h.tmpl.dir[0], h.tmpl.dir[1], z];
    scale(d, ctx.game.rng.randf(6.0, 8.0))
}

/// 0x122c / 0x122a / 0x122d (module docs).
pub fn update_122c(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if breaking_hit(w, id).is_some() {
                let v = scale(w.mobys[id].rows[1], -ctx.game.rng.randf(6.0, 8.0));
                shards::throw_all(w, id, ctx, spread(v), SPIN, GRAVITY, 0xffff, -1, -1, 9, 5);
                let c = w.mobys[id].o_class;
                ctx.host.play_sound_of(&w.mobys[id], c, 0, 0);
                w.delete(id);
            } else {
                w.clear_hit(id);
            }
        }
        _ => {}
    }
}

/// 0x1184 (module docs).
pub fn update_1184(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let (pos, s) = (w.mobys[id].pos, w.mobys[id].scale);
            for q in GLOWS {
                if let Some(sys) = ctx.parts.as_deref_mut() {
                    crate::particles::glow_def(sys, &mut ctx.game.rng, s * 1_050_000.0, 0.15, scale(q, s), id, pos, 0x60, false, 0x2880_8080, -1, 0);
                }
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if let Some(h) = breaking_hit(w, id) {
                let v = along_hit(w, id, ctx, &h, 0.75);
                shards::throw_all(w, id, ctx, spread(v), SPIN, GRAVITY, 0xffff, -1, -1, 9, 0xffff_ff80);
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            shards::throw_all(w, id, ctx, GLOW_RANGE, SPIN, GRAVITY, 0xffff, 0, -1, 6, 0xfa7f);
            let at = crate::moby::ai::frame_point(w, id, [0.0, 0.0, 20.0]);
            let s = w.mobys[id].scale;
            blast::burst(w, ctx, at, &Blast { size: s * 2.0, bursts: [5; 3], puffs: 5, streaks: 5, light: true, shake: false });
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x1186 (module docs).
pub fn update_1186(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if let Some(h) = breaking_hit(w, id) {
                let v = along_hit(w, id, ctx, &h, 0.75);
                shards::throw_all(w, id, ctx, spread(v), SPIN, GRAVITY, 0xffff, -1, -1, 9, 2);
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            shards::throw_all(w, id, ctx, BIT_RANGE, SPIN, GRAVITY, 0xffff, 0, -1, 6, 0x3d);
            // The 0x11b7 bits (module docs: not ported).
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x12a5 (module docs).
pub fn update_12a5(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if let Some(h) = breaking_hit(w, id) {
                let v = along_hit(w, id, ctx, &h, 1.0);
                shards::throw_all(w, id, ctx, spread(v), SPIN, GRAVITY, 0xffff, -1, -1, 9, 1);
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            let k = w.mobys[id].pv_i(4) as u32;
            let byte = ctx.game.flags.get(BROKEN).copied().unwrap_or(0);
            if 8 <= k || (byte >> k) & 1 == 0 {
                shards::throw_all(w, id, ctx, BOLT_RANGE, SPIN, GRAVITY, 0xffff, 0, -1, 0x10, 0xfe);
                if k < 8 {
                    if let Some(b) = ctx.game.flags.get_mut(BROKEN) { *b |= 1 << k; }
                }
            } else {
                shards::throw_all(w, id, ctx, BOLT_RANGE, SPIN, GRAVITY, 0xffff, 0, -1, 0, 0xfe);
            }
            let (pos, s) = (w.mobys[id].pos, w.mobys[id].scale);
            blast::burst(w, ctx, pos, &Blast { size: s * 10.0, bursts: [5; 3], puffs: 5, streaks: 5, light: true, shake: false });
            w.delete(id);
        }
        _ => {}
    }
}
