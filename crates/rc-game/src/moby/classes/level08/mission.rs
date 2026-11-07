//! Tabora's mission controller **0x0c2a** ([`update_0c2a`], L08 `0x3f82b0`; the decomp's `func_US2_L08_003F82AC`):
//! it picks the level's chunk at its set-up — chunk 0 the canyons, chunk 1 the desert (the region past the level's
//! one variant plane, z 100 up: crate::level::LevelData::in_variant) — and runs the arrival's scenes and the wrench's
//! upgrade at the end of the level's mission. L08 addresses below.
//!
//! The level load (`0x3012a0` → L00 `0x303990`) starts every chunked level in chunk 0 (root `0x1ca080` word 0 = 0,
//! then the hide of the other chunk's objects `0x303418(0)`), as the port does; this controller's first tick then
//! switches (`0x310770(mode, 0)` = L00 `0x303530`, crate::moby::Ctx::switch_chunk, nothing when the chunk is in
//! place).
//!
//! Its pvars (words): `+0x00` the area of the last scene, `+0x04` the desert's spot (a 0x0325 by the ship), `+0x08`
//! the moby whose uid the end marks gone, `+0x0c` the cuboid of the help 0xa3a, `+0x10` the sun's point (a quad),
//! `+0x20` the other desert spot (a 0x0988) and `+0x24` the platinum bolt it waits on, `+0x2c` the moby swapped for a
//! 0x131d at the end, `+0x30` the moby deleted at the end.
//!
//! **The stage done** ([`stage_done`]): the wrench's row (`0x139568[0xa]`) at level 1 or more (4 in the challenge
//! mode `0x1a7a12`; `0x26ccf4 + 0xe0·row` = L00 `0x2624f4`, crate::upgrades::level_of).
//!
//! * **0** (set-up): update distance 0xff, its ticks 0; flag `0x1395b8 + 0x1c` clear: `0x376b00` (not ported, below).
//!   With Ratchet under the plane (z < `0x1ca0a8`, plane 0's point) and the desert's spot `+0x04` done — its state
//!   not 0, its mission done, its uid gone since the checkpoint (`0x21ef84`) or counted in the save (`0x19b4a8`) —
//!   Ratchet put there idle (`0x2cb060(pos, rot, 0, 1)`), 5. Then by the story:
//!   * the stage done: the moby `+0x30` deleted; chunk 0 under the plane, else 1; the swap ([`swap`]); 5;
//!   * scene 0x53 or 0x54 not seen (the first visit): chunk 1; 0x53 and 0x54 queued (`0x300ab8`) — when they go in,
//!     started (`0x300c00`) with the cut in (`0x380078`, crate::moby::talk::cut_in), 1 (else 0 again next tick);
//!   * scene 0x55 seen: chunk 0, 3;
//!   * else chunk 0, 1 with its ticks at 6.
//!
//!   With item 0x15 had and the spots `+0x20` / `+0x24` both there (not 0xfe / 0xfd): the bolt `+0x24` not found
//!   (`0x19b278 + 4·level + its pvar 0`) — chunk 1, Ratchet put at `+0x20` idle; the stage done — the wrench put on
//!   its row (`0x2fffa0(0xa, 4 or 1)`, crate::game::GameState::set_upgrade) and its model (`0x30b480(1)`, not
//!   ported); 5. Then the switch to the chunk, the sun's point ([`sun`]) and the flare's colours (`0x3f8cf0`, not
//!   ported).
//! * **1**: out of a scene (`0x3800c0`, crate::scene::Scenes::in_scene) and 6 ticks in: flag `0x1c` + 0x19 = 1;
//!   scene 0x55 queued — when it goes in: started, the checkpoint's chunk 0, the cut in, `0x1b2514` = 1, the fade
//!   1.2 (`0x1b2420`), the parked ship hidden ([`hide_ship`]), chunk 0; 2.
//! * **2**: out of a scene and 6 ticks in: flag 0x1a = 1, the autosave (`0x319bc8(0, −1)`,
//!   crate::game::GameState::autosave); 3.
//! * **3**: Ratchet in the area `+0x00` (`0x339420`): scene 0x56 queued — when it goes in, started; 4.
//! * **4**: in a scene (`0x1a8f00` = 2) the moby `+0x30` deleted; out of one and 6 ticks in: the wrench put on its row
//!   (as above) with its message (`0x3004b0`, crate::game::GameState::upgrade_message), the swap, the moby `+0x08`'s
//!   uid gone (`0x21e314` / `0x21ef84`, crate::game::GameState::uid_gone), the autosave; 5.
//!
//! Each tick after: its ticks + 1; with flag 0x1c and the slot 0x50's levels word (`0x18cd20`) without this level,
//! the help 0x9f9 (slot 0x50, `0x300ee0` = crate::game::GameState::help); the zone help 0xa3a (slot 0x91, kind 0, in
//! cuboid `+0x0c`: `0x3022a8`, crate::moby::classes::oozla::zone_help); on the hovership (state 0x31, `0x18b650` a
//! 0x0eef) the help 0xa40 (slot 0x97) while that slot's levels word has bit 31 clear (`0x18d074` ≥ 0).
//!
//! Not ported: the checkpoint's chunk word (`0x21f788` = the checkpoint record `0x21eb20 + 0xc68`, which the
//! checkpoint restore `0x3682a0` switches back to: the port's checkpoint keeps no chunk), the ship byte `0x152c26`
//! (`0x376b00`), `0x1b2514`, the list `0x221b10` [`hide_ship`] walks after the ship, the wrench's model
//! (`0x30b480`), the HUD call `0x300e40(round(−65·0x1b3244), 2)` on the hovership, the sun's flare (`0x3f8bf0` /
//! the draw callback `0x3f8f78` in chunk 1, the colours `0x3f8cf0`).

use crate::moby::director::{gone, link, mission_is_done, place_hero};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c2a;

mod pv {
    pub const AREA: usize = 0x00;
    pub const DESERT_SPOT: usize = 0x04;
    pub const FINISH: usize = 0x08;
    pub const HELP_ZONE: usize = 0x0c;
    pub const SUN: usize = 0x10;
    pub const ALT_SPOT: usize = 0x20;
    pub const BOLT: usize = 0x24;
    pub const SWAPPED: usize = 0x2c;
    pub const DELETED: usize = 0x30;
}

/// The class a swapped moby becomes ([`swap`]).
const SWAP_CLASS: i16 = 0x131d;
/// The hovership (crate::moby::classes::level08::hovership).
const HOVERSHIP: i16 = 0x0eef;
/// The wrench's item.
const WRENCH: i32 = 0xa;

/// [`update_0c2a`]'s stage test (module docs).
fn stage_done(ctx: &Ctx) -> bool {
    let need = if ctx.game.challenge { 4 } else { 1 };
    ctx.game.upgrade_level(WRENCH) >= need
}

/// `0x2fffa0(0xa, 4 or 1)` then `0x30b480(1)` (the model, not ported).
fn upgrade_wrench(ctx: &mut Ctx) {
    let n = if ctx.game.challenge { 4 } else { 1 };
    ctx.game.set_upgrade(&ctx.level.items, WRENCH, n);
}

/// Plane 0's point's height (`0x1ca080 + 0x28`); 0 on a level without planes (the root's zeroed word).
fn plane_z(ctx: &Ctx) -> f32 { ctx.level.variant_planes.first().map_or(0.0, |(o, _)| o[2]) }

/// A flag byte `0x1395b8 + k`.
fn flag(ctx: &Ctx, k: usize) -> bool { ctx.game.flags.get(k).is_some_and(|&b| b != 0) }
fn set_flag(ctx: &mut Ctx, k: usize) { if let Some(b) = ctx.game.flags.get_mut(k) { *b = 1; } }

/// Scene `n` seen (`0x139768`: word `n & !3`, bit `n & 31`).
fn seen(ctx: &Ctx, n: usize) -> bool { ctx.game.scenes_seen.get(n).copied().unwrap_or(false) }

/// The moby a pvar names, deleted (`0x31e688`) and the pvar −1.
fn delete_link(w: &mut MobyWorld, id: usize, off: usize) {
    if let Some(k) = link(w, w.mobys[id].pv_i(off)) {
        w.delete(k);
        w.mobys[id].set_pv_i(off, -1);
    }
}

/// `0x3f81c8` (the decomp's `func_L08_003F6D30`): the moby `+0x2c` swapped for a 0x131d (`0x31e320`) that takes its
/// draw distance `+0x32`, update distance `+0x30`, light words `+0x38`, glow `+0x78`, place and angles, drawn
/// (`+0x31` 1), at its class's scale times the old one's share of its class's; rebuilt (`0x320dc0`); the old one
/// deleted, the pvar −1.
fn swap(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(s) = link(w, w.mobys[id].pv_i(pv::SWAPPED)) else { return };
    let o = w.mobys[s].clone();
    let old_class_scale = w.class(o.o_class).map_or(1.0, |c| c.scale);
    let n = w.create(SWAP_CLASS);
    let m = &mut w.mobys[n];
    m.draw_dist = o.draw_dist;
    m.update_dist = o.update_dist;
    m.light = o.light;
    m.ambient = o.ambient;
    m.glow = o.glow;
    m.drawn = 1;
    m.scale *= o.scale / old_class_scale;
    m.rot = o.rot;
    m.pos = o.pos;
    w.rebuild(n, &*ctx.host);
    w.delete(s);
    w.mobys[id].set_pv_i(pv::SWAPPED, -1);
}

/// `0x3f8f10`: the sun's point into `+0x10` (`0x1b9e04` 0).
fn sun(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    for (k, v) in [7_000_000.0f32, 5_500_000.0, 9_000_000.0, 1.0].into_iter().enumerate() { m.set_pv_f(pv::SUN + 4 * k, v); }
}

/// `0x370240`: the parked ship (`0x152c00`) hidden (mode | 3) with no collision (`+0x98` 0).
fn hide_ship(w: &mut MobyWorld, ctx: &mut Ctx) {
    let Some(s) = ctx.game.travel.ship.filter(|&s| s < w.mobys.len()) else { return };
    w.mobys[s].mode |= 3;
    w.mobys[s].has_collision = false;
}

/// The spot `k` done (module docs, state 0).
fn spot_done(w: &MobyWorld, ctx: &Ctx, k: usize) -> bool {
    let p = &w.mobys[k];
    if p.state != 0 || mission_is_done(ctx.game, p.mission as i32) { return true; }
    if ctx.game.uid_dead.get(p.uid.max(0) as usize).is_some_and(|&d| d != 0) { return true; }
    ctx.game.uid_count(p.uid) != 0
}

/// Scenes queued and, when the first goes in, started with the cut in.
fn play(ctx: &mut Ctx, ids: &[i32]) -> bool {
    let g = &mut *ctx.game;
    if !g.scenes.queue.push(ids[0]) { return false; }
    for &n in &ids[1..] { g.scenes.queue.push(n); }
    g.scenes.start_queue(ctx.hero.hp);
    true
}

/// 0x0c2a (module docs).
pub fn update_0c2a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p = |w: &MobyWorld, o: usize| w.mobys[id].pv_i(o);
    match w.mobys[id].state {
        0 => {
            w.mobys[id].update_dist = 0xff;
            w.mobys[id].state_ticks = 0;
            let below = ctx.hero.pos[2] < plane_z(ctx);
            if let Some(k) = link(w, p(w, pv::DESERT_SPOT)).filter(|_| below) {
                if spot_done(w, ctx, k) {
                    let (pos, rot) = (w.mobys[k].pos, w.mobys[k].rot);
                    place_hero(ctx, pos, rot, 0);
                    w.mobys[id].set_state(5, -1);
                }
            }
            let mut mode;
            if stage_done(ctx) {
                delete_link(w, id, pv::DELETED);
                mode = if ctx.hero.pos[2] < plane_z(ctx) { 0 } else { 1 };
                w.mobys[id].set_state(5, -1);
                swap(w, id, ctx);
            } else if !seen(ctx, 0x53) || !seen(ctx, 0x54) {
                mode = 1;
                if play(ctx, &[0x53, 0x54]) {
                    crate::moby::talk::cut_in(ctx.game);
                    w.mobys[id].set_state(1, -1);
                }
            } else if seen(ctx, 0x55) {
                mode = 0;
                w.mobys[id].set_state(3, -1);
            } else {
                mode = 0;
                w.mobys[id].set_state(1, -1);
                w.mobys[id].state_ticks = 6;
            }
            let there = |w: &MobyWorld, o: usize| link(w, p(w, o)).filter(|&k| !gone(w, k));
            if ctx.game.items.get(0x15).is_some_and(|&b| b != 0) {
                if let (Some(alt), Some(bolt)) = (there(w, pv::ALT_SPOT), there(w, pv::BOLT)) {
                    let n = w.mobys[bolt].pv_i(0);
                    let i = (ctx.game.level.max(0) as usize * 4).wrapping_add(n as usize);
                    if !ctx.game.platinum.get(i).is_some_and(|&b| b != 0) {
                        mode = 1;
                        let (pos, rot) = (w.mobys[alt].pos, w.mobys[alt].rot);
                        place_hero(ctx, pos, rot, 0);
                    }
                    if stage_done(ctx) { upgrade_wrench(ctx); }
                    w.mobys[id].set_state(5, -1);
                }
            }
            ctx.switch_chunk(mode, 0);
            sun(w, id);
        }
        1 => {
            if !ctx.game.scenes.in_scene() && 6 <= w.mobys[id].state_ticks as u16 {
                set_flag(ctx, 0x19);
                if play(ctx, &[0x55]) {
                    crate::moby::talk::cut_in(ctx.game);
                    ctx.game.scenes.fade = 1.2;
                    hide_ship(w, ctx);
                    ctx.switch_chunk(0, 0);
                    w.mobys[id].set_state(2, -1);
                }
            }
        }
        2 => {
            if !ctx.game.scenes.in_scene() && 6 <= w.mobys[id].state_ticks as u16 {
                set_flag(ctx, 0x1a);
                // `0x319bc8(0, −1)`: the autosave.
                ctx.game.autosave(-1);
                w.mobys[id].set_state(3, -1);
            }
        }
        3 => {
            if ctx.level.in_area(p(w, pv::AREA), ctx.hero.pos) && play(ctx, &[0x56]) {
                w.mobys[id].set_state(4, -1);
            }
        }
        4 => {
            if ctx.game.scenes.mode == crate::scene::SCENE { delete_link(w, id, pv::DELETED); }
            if !ctx.game.scenes.in_scene() && 6 <= w.mobys[id].state_ticks as u16 {
                upgrade_wrench(ctx);
                ctx.game.upgrade_message(WRENCH);
                swap(w, id, ctx);
                if let Some(k) = link(w, p(w, pv::FINISH)) {
                    let (uid, mission) = (w.mobys[k].uid, w.mobys[k].mission);
                    ctx.game.uid_gone(uid, mission);
                }
                // `0x319bc8(0, −1)`: the autosave.
                ctx.game.autosave(-1);
                w.mobys[id].set_state(5, -1);
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    m.state_ticks = (m.state_ticks as u16).wrapping_add(1) as i16;
    let level = ctx.game.level.max(0) as u32;
    if flag(ctx, 0x1c) && ctx.game.help_levels.get(0x50).is_some_and(|&l| l & 1u32.checked_shl(level).unwrap_or(0) == 0) {
        ctx.game.help(0x9f9, 0x50);
    }
    crate::moby::classes::oozla::zone_help(ctx, 0xa3a, 0x91, 0, p(w, pv::HELP_ZONE), -1);
    let riding = ctx.hero.state == 0x31 && ctx.hero.vehicle.moby.and_then(|k| w.mobys.get(k)).is_some_and(|v| v.o_class == HOVERSHIP);
    if riding && ctx.game.help_levels.get(0x97).is_some_and(|&l| (l as i32) >= 0) {
        ctx.game.help(0xa40, 0x97);
    }
}
