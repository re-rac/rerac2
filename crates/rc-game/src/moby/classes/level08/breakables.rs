//! Level 8's breakables (L08 addresses; the engine functions named by their L08 addresses with the L00 ones the port
//! knows them by).
//!
//! * **0x12aa** ([`update_12aa`], `0x431128`, 25 placed), **0x129c** (`0x430aa8`, 22 placed) and **0x129e**
//!   (`0x430cc8`, 18 placed): wait for a hit (or a linked breakable's break), break into their pieces with the level's
//!   bolts, then linger hidden for 120 ticks ([`linger`]); 0x129e also explodes.
//! * **0x12d0** ([`update_12d0`], `0x431348`, 8 placed): turns; hits flash it and wear its health down; it breaks into
//!   its pieces, the last of its group with a heap of bolts.
//! * **0x129f** ([`update_129f`], `0x430fa0`, 6 placed): a blast, every piece, five times the level's bolts and a
//!   0x12a0 left in its place.
//!
//! **The count** (`0x1b3aa0`): the breakables lingering (each adds one on its first tick of state 3 and takes it off
//! when it goes; 0x129e's set-up clears it). The full break needs it under 10, 0x12d0's health only goes down under 4.
//! The port counts the 0x12aa / 0x129c / 0x129e in state 3. The full break also needs the moby drawn (`+0x31`) and the
//! frame load `$gp+0x351c` under 0.9, which the port's always is.
//!
//! The engine functions: `0x32b2e8` = L00 `0x31dce8` (the hit with flags), `0x335e30` = L00 `0x327498`
//! ([`shards::break_into`]), `0x335ae0` = L00 `0x327388` ([`bolts::level_drop`]), `0x36eb38` = L00 `0x3605f0` the
//! class sound, `0x339370` = L00 `0x32a7c8` (crate::moby::Moby::first), `0x2fa6b0` = L00 `0x2eeab0` (the countdown),
//! `0x31e688` = L00 `0x311688` (the delete), `0x334b28` = L00 `0x326190` (a point in its scaled frame,
//! [`scaled_point`]), `0x32e558` = L00 `0x320538` ([`blast::explode`]), `0x32b348` = L00 `0x31dd48`
//! (crate::moby::ai::damage_of), `0x32e148` / `0x32e208` = L00 `0x320128` / `0x3201e8` (crate::moby::ai::flash_start
//! / flash), `0x320300` = L00 `0x313300` (the ambient), `0x329ab8` = L00 `0x31c8e0`
//! (crate::moby::classes::props::group_alive), `0x331800` = L00 `0x322af0` ([`bolts::spawn`]), `0x407748` = L00
//! `0x3f8348` ([`blast::burst`]), `0x339a08` = L00 `0x32b580` ([`shards::throw_all`]), `0x336578` = L00 `0x327898`
//! (crate::moby::classes::props::spawn_in_place).

use crate::collision::mobys::GRID_NONE;
use crate::gravity::to_world;
use crate::hero::physics::{add, add_rot, scale, V3};
use crate::moby::blast::{self, Blast, Explosion};
use crate::moby::classes::props::{group_alive, spawn_in_place};
use crate::moby::{ai, bolts, countdown, shards, Ctx, MobyWorld};

pub const CLASS_12AA: i16 = 0x12aa;
pub const CLASS_129C: i16 = 0x129c;
pub const CLASS_129E: i16 = 0x129e;
pub const CLASS_12D0: i16 = 0x12d0;
pub const CLASS_129F: i16 = 0x129f;

/// The hit flags the breakables answer (`0x32b2e8(m, 0x4010000, 0)`).
const BREAK_FLAGS: u32 = 0x0401_0000;
/// `0x1a8f20`: the pieces' added velocity (zero).
const NO_VEL: V3 = [0.0; 3];

/// `0x1b3aa0`: the breakables lingering (module docs).
fn lingering(w: &MobyWorld) -> usize {
    w.mobys.iter().filter(|m| m.state == 3 && matches!(m.o_class, CLASS_12AA | CLASS_129C | CLASS_129E)).count()
}

/// `0x334b28(m, out, x, y, z)`: `v` through its rows, times its scale, plus its place.
fn scaled_point(w: &MobyWorld, id: usize, v: V3) -> V3 {
    let m = &w.mobys[id];
    add(scale(to_world(v, &m.rows), m.scale), m.pos)
}

/// State 1 of 0x12aa / 0x129c / 0x129e: a hit with [`BREAK_FLAGS`] while `+0x68`[0] = 0 breaks it (the record kept
/// then); else the record let go, and the moby `[1]` (when above 0) gone, freed or in its own state 3 breaks it too.
fn wait_hit(w: &mut MobyWorld, id: usize) {
    if w.hit_message(id).is_some_and(|h| h.tmpl.flags & BREAK_FLAGS != 0) && w.mobys[id].pv_i(0) == 0 {
        w.mobys[id].set_state(2, -1);
        return;
    }
    w.clear_hit(id);
    let n = w.mobys[id].pv_i(4);
    if 0 < n && w.mobys.get(n as usize).is_some_and(|e| matches!(e.state, 0xfe | 0xfd | 3)) {
        w.mobys[id].set_state(2, -1);
    }
}

/// State 3: its first tick hides it without collision (`+0x31` 0, mode | 1, `+0x98` 0) and its timer `+0x68`[2] 120;
/// that out, it is deleted.
fn linger(w: &mut MobyWorld, id: usize) {
    if w.mobys[id].first() {
        let m = &mut w.mobys[id];
        m.drawn = 0;
        m.mode |= 1;
        m.has_collision = false;
        m.set_pv_i(8, 0x78);
        let old = m.grid_rect;
        m.grid_rect = GRID_NONE;
        w.grid.update(id as u16, old, GRID_NONE);
        w.changed = true;
    }
    let mut t = w.mobys[id].pv_i(8);
    let out = countdown(&mut t);
    w.mobys[id].set_pv_i(8, t);
    if out { w.delete(id); }
}

/// The lesser break's mask of each (the full one 0xf).
fn lesser(o_class: i16) -> u32 { if o_class == CLASS_129C { 9 } else { 1 } }

/// 0x12aa / 0x129c / 0x129e. **0**: 1 (0x129e: the count cleared). **1**: [`wait_hit`]. **2**: drawn with fewer
/// than 10 lingering, the full break: (0x129e: the explosion at (0, 0, 3) in its scaled frame, `0x32e558`: flashes 1
/// and 0.5 past 9, size 1.3 × its scale, light 15, 10 embers, 3 billows, 16 flares, one debris) its pieces 0..3; else
/// the lesser (piece 0; 0x129c pieces 0 and 3); both 3..6 across and up, no sound. Then the level's bolts, its class
/// sound 0; 3. **3**: [`linger`].
pub fn update_12aa(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let o_class = w.mobys[id].o_class;
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => wait_hit(w, id),
        2 => {
            let full = w.mobys[id].drawn != 0 && lingering(w) < 10;
            if full && o_class == CLASS_129E {
                let at = scaled_point(w, id, [0.0, 0.0, 3.0]);
                let e = Explosion {
                    flash: 1.0, flash2: 0.5, near: 9.0, size: w.mobys[id].scale * 1.3, light: 15.0, embers: 10, billows: 3,
                    flares: 0x10, sound: -1, debris: 1, lod: -1, ..Default::default()
                };
                blast::explode(w, ctx, Some(id), Some(at), &e);
            }
            let mask = if full { 0xf } else { lesser(o_class) };
            shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), mask, NO_VEL, 0xff);
            bolts::level_drop(w, id, 0.0, 1, ctx);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.mobys[id].set_state(3, -1);
        }
        3 => linger(w, id),
        _ => {}
    }
}

/// 0x12d0's health and flash blocks (`+0x20`, `+0x50`).
const HEALTH: usize = 0x20;
const FLASH: usize = 0x50;
/// `0x2b3660[8]` × 35 − 1: the bolts the last of a group gives (Tabora's 10 a share).
const GROUP_BOLTS: i32 = 10 * 35 - 1;

/// 0x12d0 (`0x431348`). **0**: its health 5 (`+0x20`, `+0x24` 5); 1. **1**: its yaw on by `+0x68`[0x64] × π/10800; a
/// hit with the flags 0x230000 (none: the damage's timers still run): its damage (`0x32b348`) above 0 flashes it to
/// (255, 128, 128) (`0x32e148`) and, with fewer than 4 lingering, comes off its health; under 0 its class sound 0
/// and 2. The flash's tick (`0x32e208`); the record let go. **2**: its colour back (`0x320300`, the flash's
/// `+4..+6`); the last of its group alive gives [`GROUP_BOLTS`] in 1..3 bolts (`0x331800`, flags 3); its pieces
/// (`0x335e30`: 3..6 across and up, along (0.1167, 0.0667, 0) through its rows, mask 0xff drawn, else 0xaa, no
/// sound); deleted.
pub fn update_12d0(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_s(HEALTH + 4, 5);
            m.set_pv_f(HEALTH, 5.0);
            m.set_state(1, -1);
        }
        1 => {
            let m = &mut w.mobys[id];
            m.rot[2] = add_rot(m.rot[2], m.pv_f(0x64) * f32::from_bits(0x3998_825c));
            let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x23_0000 != 0);
            let wrench = hit.and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == crate::hero::items::WRENCH_CLASS);
            let dmg = ai::damage_of(w, id, hit.as_ref(), wrench);
            if 0.0 < dmg {
                let m = &mut w.mobys[id];
                m.set_pv_b(FLASH + 7, 0xff);
                m.set_pv_b(FLASH + 9, 0x80);
                m.set_pv_b(FLASH + 8, 0x80);
                ai::flash_start(m, FLASH);
                if lingering(w) < 4 {
                    let m = &mut w.mobys[id];
                    m.set_pv_f(HEALTH, m.pv_f(HEALTH) - dmg);
                }
            }
            if w.mobys[id].pv_f(HEALTH) < 0.0 {
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                w.mobys[id].set_state(2, -1);
            }
            ai::flash(&mut w.mobys[id], FLASH);
            w.clear_hit(id);
        }
        2 => {
            let m = &mut w.mobys[id];
            m.ambient = [m.pv_b(FLASH + 4), m.pv_b(FLASH + 5), m.pv_b(FLASH + 6)];
            if group_alive(w, w.mobys[id].group as i32) == 1 {
                let limit = ctx.game.rng.rand_range(1, 3);
                bolts::spawn(w, id, 0.0, GROUP_BOLTS, GROUP_BOLTS, 3, limit, ctx);
            }
            let v = to_world([f32::from_bits(0x3dee_eef0), f32::from_bits(0x3d88_8889), 0.0], &w.mobys[id].rows);
            let mask = if w.mobys[id].drawn != 0 { 0xff } else { 0xaa };
            shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), mask, v, 0xff);
            w.delete(id);
        }
        _ => {}
    }
}

/// `$gp+0x2ea0` / `+0x2ea8` / `+0x2eb0`: 0x129f's pieces' gravity (10..20), spin (90..360) and velocity ranges (x / y
/// 3..6 of either sign, z 5..9).
const POST_GRAVITY: (f32, f32) = (10.0, 20.0);
const POST_SPIN: (f32, f32) = (90.0, 360.0);
const POST_H: [f32; 8] = [3.0, 3.0, 5.0, 0.0, 6.0, 6.0, 9.0, 0.0];
/// What it leaves in its place.
const STUB: i16 = 0x12a0;

/// 0x129f (`0x430fa0`). **0**: 1. **1**: a hit with [`BREAK_FLAGS`] while `+0x68`[0] = 0: 2; the record let go every
/// tick. **2**: 7 above it the blast (`0x407748`: size 0.75, sprites 0x19 / 0x14 / 0x14, 0x14 puffs, 0x2d streaks,
/// its flash ball, the shake); every piece (flags 7: x / y of either sign, no bolts; its class sound 0, none
/// skipped); five times the level's bolts 4 up; a 0x12a0 in its place; deleted.
pub fn update_129f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & BREAK_FLAGS != 0) && w.mobys[id].pv_i(0) == 0 {
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            let at = add(w.mobys[id].pos, [0.0, 0.0, 7.0]);
            blast::burst(w, ctx, at, &Blast { size: 0.75, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true });
            shards::throw_all(w, id, ctx, POST_H, POST_SPIN, POST_GRAVITY, 0xffff, 0, -1, 7, 0xffff);
            bolts::level_drop(w, id, 4.0, 5, ctx);
            spawn_in_place(w, id, STUB, 0, &*ctx.host);
            w.delete(id);
        }
        _ => {}
    }
}
