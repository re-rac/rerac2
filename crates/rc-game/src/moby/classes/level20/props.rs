//! Yeedil's props (L20 addresses).
//!
//! * **0x124f** ([`update_124f`], `0x448e78`, 14 placed; the decomp's v2.00 `func_US2_L20_00448E78`): a mover that
//!   glides once to a cuboid's place when set off. Its pvars: `+0x20` its platform block (crate::moby::platform, flags
//!   | 1), `+0x60` / `+0x70` its place and angles at the start, `+0x80` the cuboid it goes to (−1: it stops updating at
//!   once, 4), `+0x84` the glide's ticks, `+0x88` its share done, `+0x8c` a moby whose uid gone for good (or whose
//!   death) sets it off, `+0x90` the hero's form switched back at its end, `+0x92` its mission's end sets it off,
//!   `+0x93` its mission done when it goes, `+0x94` a moby it pokes (`+0xbc` = 1) and then waits for, `+0x98` an area
//!   Ratchet sets it off in.
//!   * **0**: its start kept, its rows kept (mode 0x100). Already set off — the uid `+0x8c` gone for good, its mission
//!     done (with `+0x92`), or Ratchet in its area — it is put at the cuboid's place and angles and deleted; else 1.
//!   * **1**: poked (`+0xbc`), hooked, the moby `+0x8c` none or dead, its mission done (with `+0x92`), or Ratchet in
//!     its area: the moby `+0x94` poked, its mission done (with `+0x93`), 2.
//!   * **2**: its share on by 1/`+0x84` a tick (at most 1), eased (half a cosine): its third row shrunk by what is
//!     left of it, its place that share of the way to the cuboid's; at the end 3.
//!   * **3**: once the moby `+0x94` (if any) has its poke cleared, deleted.
//!   * Below 3, its move into its platform block (`0x3378b8` = L00 `0x322668`) and Ratchet carried.
//! * **0x1269** ([`update_1269`], `0x44a6c0`, 12 placed; the decomp's `func_L20_00449178`): a breakable. **0**: a
//!   glow on it (`0x354c48` = L00 `0x33f550`: 420000 across, pulled 1 toward the camera, 2 up, 0x40204080, while it
//!   lives); 1. **1**: a hit with flags 0x4010000 while `+0x68`[0] = 0 breaks it (2); its hit let go. **2**: every
//!   piece thrown (`0x340860` = L00 `0x32b580`, crate::moby::shards::throw_all: `$gp+0x3370` / `+0x3368` / `+0x3360`),
//!   the blast 2 above it (`0x41b550` = L00 `0x3f8348`: size 0.5, sprites 0x19 / 0x14 / 0x14, 0x14 puffs, 0x2d
//!   streaks, its flash ball), a 0x126a in its place (`0x33ccb0` = L00 `0x327898`), deleted.
//! * **0x119b** ([`update_119b`], `0x43fe58`, 11 placed; the decomp's `func_L19_00422420`, shared with L19's 0x1204
//!   `0x423750`): a field that fades away. Its pvars: `+0x00` a moby it watches (−1 none), `+0x04` its fade, `+0x08`
//!   its scroll, `+0x0c` its size (its scale is that over 9.77; 6.2 for a 0x1204), `+0x14` its loop sound.
//!   * **0**: alpha 0x50, mode | 0xb0c, its first row and its sphere scaled to its size, its second row by 0.8, the
//!     ambient 0x80 grey; its mission done: 3.
//!   * **1**: its loop sound 0 kept playing. The watched moby's uid counted (or the moby dead), or with none watched
//!     hooked: 2.
//!   * **2**: its sound stopped; it fades by 0.05 a tick (alpha 80 times the fade); faded out, 3.
//!   * **3**: its uid marked gone (crate::game::GameState::uid_gone), deleted.
//!
//! Not ported for 0x124f: the switch back of the hero's form at its end (`0x2c5b50(0)`, with `+0x90`; level 20's
//! own, which the port lacks). For 0x119b: the scroll of its class's texture coordinates (`0x326a20` = L00
//! `0x310838`: 0x10 a tick on s, back by 0x1000), and the point light `+0x0a` freed at its end (`0x312160` = L00
//! `0x2fdc48`; this code never takes one).

use crate::hero::physics::{add, scale, sub};
use crate::moby::blast::{self, Blast};
use crate::moby::classes::props::spawn_in_place;
use crate::moby::{shards, Ctx, MobyWorld};

/// The level's mission `mission` done (`0x19b2e8 + level·0x10`, bit 7).
fn mission_done(ctx: &Ctx, mission: u8) -> bool {
    let g = &ctx.game;
    mission != 0xff && g.missions.get(g.level.max(0) as usize * 0x10 + mission as usize).is_some_and(|b| b & 0x80 != 0)
}

fn live(w: &MobyWorld, k: i32) -> Option<usize> {
    usize::try_from(k).ok().filter(|&k| w.mobys.get(k).is_some_and(|m| m.state != 0xfe && m.state != 0xfd))
}

pub const SLIDER: i16 = 0x124f;

/// 0x124f (module docs).
pub fn update_124f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let m = &w.mobys[id];
    let to = m.pv_i(0x80);
    let place = ctx.level.cuboid_place(to);
    // Its mission's end (with `+0x92`) or Ratchet in its area `+0x98`.
    let by_mission = m.pv_b(0x92) != 0 && mission_done(ctx, m.mission);
    let area = m.pv_i(0x98);
    let in_area = area != -1 && ctx.level.in_area(area, ctx.hero.pos);
    match m.state {
        0 => {
            if to == -1 {
                let m = &mut w.mobys[id];
                m.mode |= crate::moby::mode::NO_UPDATE;
                m.set_state(4, -1);
                return;
            }
            let gone = usize::try_from(m.pv_i(0x8c)).ok().and_then(|u| ctx.game.uid_dead.get(u)).is_some_and(|&b| b != 0);
            let m = &mut w.mobys[id];
            m.set_pv_f(0x88, 0.0);
            m.set_pv_i(0x5c, m.pv_i(0x5c) | 1);
            let (p, r) = (m.pos, m.rot);
            m.set_pv_v(0x60, p);
            m.set_pv_v(0x70, r);
            m.mode |= crate::moby::mode::KEEP_ROWS;
            if gone || by_mission || in_area {
                if let Some((p, r)) = place {
                    m.pos = p;
                    m.rot = r;
                }
                w.delete(id);
                return;
            }
            m.set_state(1, -1);
        }
        1 => {
            let watched = m.pv_i(0x8c);
            let off = m.timer != 0 || m.once & 4 != 0 || watched == -1 || live(w, watched).is_none() || by_mission || in_area;
            if off {
                let poke = m.pv_i(0x94);
                let (mission, done) = (m.mission, m.pv_b(0x93) != 0);
                if let Some(p) = usize::try_from(poke).ok().filter(|&p| p < w.mobys.len()) { w.mobys[p].timer = 1; }
                if done { ctx.game.mission_done(mission); }
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            let m = &mut w.mobys[id];
            let t = (m.pv_f(0x88) + 1.0 / m.pv_f(0x84)).min(1.0);
            m.set_pv_f(0x88, t);
            let e = (1.0 - (t * std::f32::consts::PI).cos()) * 0.5;
            m.rows[2] = scale(m.rows[2], 1.0 - e);
            let start = m.pv_v(0x60);
            let end = place.map_or(start, |(p, _)| p);
            m.pos = add(start, scale(sub(end, start), e));
            if 1.0 <= t { m.set_state(3, -1); }
        }
        3 => {
            let poke = m.pv_i(0x94);
            if poke == -1 || w.mobys.get(poke as usize).is_none_or(|p| p.timer == 0) {
                w.delete(id);
                return;
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.state < 3 {
        let (pos, rot) = (m.pos, m.rot);
        crate::moby::platform::write(m, 0x20, sub(pos, old), old_rot, rot);
        let flags = m.pv_i(0x20 + 0x3c) as u32;
        crate::moby::carry_hero_with(ctx, id, flags, old, old_rot[2], pos, rot[2]);
    }
}

pub const CRYSTAL: i16 = 0x1269;
const CRYSTAL_LEFT: i16 = 0x126a;
/// `$gp+0x3370` / `+0x3368` / `+0x3360`: the pieces' velocities (a second), spins and gravities.
const CRYSTAL_H: [f32; 8] = [-3.0, -3.0, 1.0, 0.0, 3.0, 3.0, 7.0, 0.0];
const CRYSTAL_SPIN: (f32, f32) = (90.0, 360.0);
const CRYSTAL_GRAVITY: (f32, f32) = (10.0, 20.0);

/// 0x1269 (module docs).
pub fn update_1269(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (pos, far) = (m.pos, m.draw_dist);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow(sys, &mut ctx.game.rng, 420_000.0, 1.0, [0.0, 0.0, 2.0], id, pos, far, false, 0x4020_4080, -1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0) && w.mobys[id].pv_i(0) == 0 {
                w.mobys[id].set_state(2, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            shards::throw_all(w, id, ctx, CRYSTAL_H, CRYSTAL_SPIN, CRYSTAL_GRAVITY, 0xffff, 0, -1, 0, 0xffff);
            // `0x33a838(0, 0, 2, m)`: 2 up in its frame.
            let m = &w.mobys[id];
            let at = add(m.pos, m.rows[2].map(|x| x * 2.0));
            blast::burst(w, ctx, at, &Blast { size: 0.5, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
            spawn_in_place(w, id, CRYSTAL_LEFT, 0, &*ctx.host);
            w.delete(id);
        }
        _ => {}
    }
}

pub const FIELD: i16 = 0x119b;
/// L19's class on the same code (its size's base 6.2).
const FIELD_L19: i16 = 0x1204;

/// 0x119b (module docs).
pub fn update_119b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let base = if w.mobys[id].o_class == FIELD_L19 { 6.2 } else { 9.77 };
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(0x14, -1);
            m.alpha = 0x50;
            m.state = 1;
            m.mode |= 0xb0c;
            let s = m.pv_f(0xc) / base;
            m.rows[0] = scale(m.rows[0], s);
            m.rows[1] = scale(m.rows[1], 0.8);
            m.bsphere[3] *= s;
            m.set_pv_s(0xa, -1);
            m.set_pv_f(4, 1.0);
            m.ambient = [0x80; 3];
            if mission_done(ctx, m.mission) { m.set_state(3, -1); }
        }
        1 => {
            let h = w.mobys[id].pv_i(0x14);
            if h < 0 || !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(0x14, h);
            }
            let watched = w.mobys[id].pv_i(0);
            let on = if watched == -1 {
                w.mobys[id].once & 4 != 0
            } else {
                // The watched moby's uid counted (`0x19b4a8 + level·0x400`, its nibble's low three bits); dead, on.
                live(w, watched).is_none_or(|k| ctx.game.uid_count(w.mobys[k].uid) != 0)
            };
            if on {
                let m = &mut w.mobys[id];
                m.set_pv_f(4, 1.0);
                m.state = 2;
            }
        }
        2 => {
            let h = w.mobys[id].pv_i(0x14);
            if h >= 0 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
            let m = &mut w.mobys[id];
            let f = m.pv_f(4) - 0.05;
            m.set_pv_i(0x14, -1);
            m.set_pv_f(4, f);
            if f < 0.0 { m.state = 3; } else { m.alpha = (f * 80.0) as i32 as u8; }
        }
        3 => {
            let (uid, mission) = (w.mobys[id].uid, w.mobys[id].mission);
            ctx.game.uid_gone(uid, mission);
            w.delete(id);
            return;
        }
        _ => {}
    }
    // Its scroll: on by 0x10 a tick, back by 0x1000 past it (the class's texture scroll is not ported).
    let m = &mut w.mobys[id];
    let mut s = m.pv_s(8).wrapping_add(0x10);
    if 0x1000 < s { s -= 0x1000; } else if s < 0 { s += 0x1000; }
    m.set_pv_s(8, s);
}

pub const NEST: i16 = 0x11ec;
/// `$gp+0x3220` / `+0x3224` / `+0x3228`: the hop's most speed up (a second) and its most tilts (degrees a second).
const NEST_HOP: f32 = 5.0;
const NEST_TILT: (f32, f32) = (20.0, 20.0);
/// `$gp+0x323c`: the hop's gravity (a second²).
const NEST_GRAVITY: f32 = 30.0;

/// 0x11ec (`0x446718`, 11 placed; the decomp's `func_L20_004451C0`): a hopping nest of the breeding critters
/// (crate::moby::classes::level18::breeder). Its pvars: `+0x20` health (most `+0x24`, 1 when unset), `+0x50` its
/// flash, `+0x60` / `+0x64` the hop's tilts, `+0x70` / `+0x74` the cuboids that start its hopping and burst it, `+0x78`
/// its brood's group, `+0x7c` its resting height, `+0x80` the hop's speed, `+0x84` the throw's wait (45..120, read
/// by nothing here), `+0x88` its loop sound.
/// * **Each tick**: hits with flags 0x10000 take their damage off its health, each flashing it (`0x335268` /
///   `0x335328` = L00 `0x320128` / `0x3201e8`, crate::moby::ai::flash_start / flash, at `+0x50`); no health, 3. Its
///   hit let go.
/// * **0**: its resting height kept, its health 1 if unset, its sound none; 1.
/// * **1**: half the time, with a start cuboid, Ratchet in the burst cuboid bursts it (3), in the start cuboid starts
///   its hopping (2).
/// * **2**: half the time Ratchet in the burst cuboid bursts it (3, its sound stopped). On its first tick of a hop
///   (each state change starts one): level at its resting height, up to [`NEST_HOP`] a second more up and new tilts
///   (up to [`NEST_TILT`]). Its loop sound 0 kept playing. Above its rest it rises by its speed under
///   [`NEST_GRAVITY`], tilting one way going up and back coming down; below it, a new hop.
/// * **3**: the blast (`0x41b550` = L00 `0x3f8348`: size 1, sprites 0x19 / 0x14 / 0x14, 0x14 puffs, 0x2d streaks,
///   its flash ball, the shake), its pieces 0..8 (`0x33c568` = L00 `0x327498`, crate::moby::shards::break_into: 3..6 /
///   3..6), its sound 1, its bolts (`0x32fd10(m, 0, 3)`), deleted.
///
/// Not ported: the brood's throw in 3 — each hidden one of the group `+0x78` (`0x3fd4f0` = L18 `0x3f8a58`) thrown
/// (`0x3fd5f8` = L18 `0x3f8b60`) from 0.7 up in its frame at 0.1 a tick along its −y, jittered by 0.01 (`0x336918` =
/// L00 `0x3217d8`), their `+0xb8` it: the breeder's `brood_pick` and `launch` are private to
/// crate::moby::classes::level18::breeder.
pub fn update_11ec(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::hero::physics::add_rot;
    use crate::moby::{ai, bolts};
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
    let dmg = ai::damage_of(w, id, hit.as_ref(), false);
    let m = &mut w.mobys[id];
    m.set_pv_f(0x20, m.pv_f(0x20) - dmg);
    if 0.0 < dmg {
        ai::flash_start(m, 0x50);
        if m.pv_f(0x20) <= 0.0 { m.set_state(3, -1); }
    }
    w.clear_hit(id);
    ai::flash(&mut w.mobys[id], 0x50);
    let hero = ctx.hero.pos;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_f(0x7c, m.pos[2]);
            if m.pv_s(0x24) == 0 {
                m.set_pv_s(0x24, 1);
                m.set_pv_f(0x20, 1.0);
            }
            m.set_pv_i(0x84, -1);
            m.set_pv_f(0x30, 1.0);
            m.set_state(1, -1);
            m.set_pv_i(0x88, -1);
        }
        1 => {
            if ctx.game.rng.randi(2) == 0 && w.mobys[id].pv_i(0x70) != -1 {
                let m = &mut w.mobys[id];
                if ctx.level.in_cuboid(m.pv_i(0x74), hero) {
                    m.set_state(3, -1);
                } else if ctx.level.in_cuboid(m.pv_i(0x70), hero) {
                    m.set_state(2, -1);
                }
            }
        }
        2 => {
            if ctx.game.rng.randi(2) == 0 && ctx.level.in_cuboid(w.mobys[id].pv_i(0x74), hero) {
                w.mobys[id].set_state(3, -1);
                let h = w.mobys[id].pv_i(0x88);
                if h >= 0 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                w.mobys[id].set_pv_i(0x88, -1);
            }
            if w.mobys[id].first() {
                let up = ctx.game.rng.randf(0.0, NEST_HOP * 0.016_666_668);
                let tx = ctx.game.rng.randf(0.0, NEST_TILT.0 * 0.000_290_888_25);
                let ty = ctx.game.rng.randf(0.0, NEST_TILT.1 * 0.000_290_888_25);
                let m = &mut w.mobys[id];
                m.rot[0] = 0.0;
                m.rot[1] = 0.0;
                m.pos[2] = m.pv_f(0x7c);
                m.set_pv_f(0x80, m.pv_f(0x80) + up);
                m.set_pv_f(0x60, tx);
                m.set_pv_f(0x64, ty);
            }
            let h = w.mobys[id].pv_i(0x88);
            if h < 0 || !ctx.host.sound_playing(h) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(0x88, h);
            }
            let m = &mut w.mobys[id];
            if m.pv_f(0x7c) <= m.pos[2] {
                m.pos[2] += m.pv_f(0x80);
                let v = m.pv_f(0x80) - NEST_GRAVITY * 0.000_277_777_78;
                m.set_pv_f(0x80, v);
                let (tx, ty) = if 0.0 < v { (m.pv_f(0x60), m.pv_f(0x64)) } else { (-m.pv_f(0x60), -m.pv_f(0x64)) };
                m.rot[0] = add_rot(tx, m.rot[0]);
                m.rot[1] = add_rot(ty, m.rot[1]);
            } else {
                m.set_state(2, -1);
            }
        }
        3 => {
            let at = w.mobys[id].pos;
            blast::burst(w, ctx, at, &Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true });
            shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x1ff, [0.0; 3], 0xff);
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            bolts::drop(w, id, 0, 3, ctx);
            w.delete(id);
            return;
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = m.state_ticks.wrapping_add(1); }
}
