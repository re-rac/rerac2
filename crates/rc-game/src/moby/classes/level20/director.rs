//! Level 0x14's director (Yeedil; L20 addresses).

use crate::moby::director::{gone, link, members, mission_is_done, place_hero, solid};
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c36;

/// Its pvars (words): the missions `+0x00` (done at the switch), `+0x04` (the chunk's goal) and `+0x08` (the save's);
/// the fade `+0x0c` (a float, 1 black); the music asked `+0x10`; the fade's ticks `+0x14`; a tick count `+0x18`; the
/// state after the fade `+0x1c`; the chunk to switch to `+0x20`; the cuboids to put Ratchet at `+0x24 + 4·k` (`k` in
/// `+0x3c`); the cuboids of the save `+0x28`, the goal `+0x2c` and the music `+0x60`; the mobys it waits on `+0x48` /
/// `+0x4c`, shows `+0x58` / `+0x5c`, the two gunships `+0x64` / `+0x68`, the one whose timer byte holds it `+0x6c`, the
/// one it raises `+0x70` (its height `+0x74`, the rise's length `+0x78`, risen `+0x7c`); the trooper groups `+0x50` /
/// `+0x54`; the music `+0x80`, the switch's track `+0x84`; the save asked `+0x8c`.
mod pv {
    pub const MISSION_SWITCH: usize = 0x00;
    pub const MISSION_GOAL: usize = 0x04;
    pub const MISSION_SAVE: usize = 0x08;
    pub const FADE: usize = 0x0c;
    pub const MUSIC_ON: usize = 0x10;
    pub const FADE_T: usize = 0x14;
    pub const TICKS: usize = 0x18;
    pub const NEXT: usize = 0x1c;
    pub const CHUNK: usize = 0x20;
    pub const PLACES: usize = 0x24;
    pub const AT_SAVE: usize = 0x28;
    pub const AT_GOAL: usize = 0x2c;
    pub const PLACE_K: usize = 0x3c;
    pub const WAIT_B: usize = 0x48;
    pub const WAIT_A: usize = 0x4c;
    pub const GROUPS: [usize; 2] = [0x50, 0x54];
    pub const SHOW: [usize; 2] = [0x5c, 0x58];
    pub const AT_MUSIC: usize = 0x60;
    pub const GUNSHIPS: [usize; 2] = [0x64, 0x68];
    pub const HOLDER: usize = 0x6c;
    pub const RISER: usize = 0x70;
    pub const RISER_Z: usize = 0x74;
    pub const RISE_LEN: usize = 0x78;
    pub const RISEN: usize = 0x7c;
    pub const MUSIC: usize = 0x80;
    pub const TRACK: usize = 0x84;
    pub const SAVED: usize = 0x8c;
}

/// The gunship class whose two it waits on.
const GUNSHIP: i16 = 0x0cbc;
/// How far the riser goes (its height + 20 until the rise).
const RISE: f32 = 20.0;
/// The fades' length (ticks) and step (`0x3c03126f`).
const FADE_TICKS: i32 = 125;
const FADE_STEP: f32 = f32::from_bits(0x3c03_126f);

/// 0x0c36 (`0x40a2b0`, the decomp's `func_L20_00408D98`): Yeedil's flow between its chunks.
/// * Each tick before chunk 2 (`0x1ca4c0` < 2): `0x1b2b68` = 85 (not ported: crate::moby::classes::level20::boss).
/// * **0**: the chunk now (`+0x20`), `+0x10` / `+0x14` 0, `+0x8c` the save's mission done. With the goal's mission
///   not done the riser `+0x70` goes 20 up (its height kept in `+0x74`, `+0x7c` 0), else `+0x7c` 1. Then 1.
/// * **1**: the save's mission done and Ratchet in cuboid `+0x28`: once, the autosave (`0x321b70` = L00 `0x30d658(0,
///   −1)`, crate::game::GameState::autosave). In chunk 2 in cuboid `+0x60`, once: the music `+0x80`. In cuboid `+0x2c` with the goal not
///   done: the goal done, the moby `+0x4c` hooked (≥ 0), the troopers of the groups `+0x50` / `+0x54` sent away
///   ([`dismiss_group`]), the switch to chunk 2 at place 3, 5. Without the goal: with the holder `+0x6c`'s timer byte
///   not 1, the riser not risen and both gunships `+0x64` / `+0x68` gone (or not gunships): 7.
/// * **2** / **3**: a fade out (2) or in (3) over 125 ticks into `+0x0c`, drawn by the callback ([`draw`]); then the
///   state `+0x1c`.
/// * **4**: the chunk switch (`0x318718` = L00 `0x304200`; crate::moby::Ctx::switch_chunk) to `+0x20` with track
///   `+0x84`, the mission `+0x00` done, Ratchet put at cuboid `+0x24[k]` in state 0x6f then his body's idle
///   (`0x2dc618(0)`: Ratchet 0, Clank 0x3f, body 4 0x8d), the screen black; `+0x1c` 1, then 6.
/// * **5**: out of a scene with none asked for (`0x1a8f08` −2, `0x1a8f00` ≠ 2): black; once the moby `+0x4c` is gone
///   (state past 0x80), 4.
/// * **6**: black (out of a scene; in one the fade 0); out of a scene, once the moby `+0x48` is gone: the mobys
///   `+0x5c` / `+0x58` shown and solid (mode bits 2 and 0x41 off, 0x1000 on, their class's collision), the fade 0,
///   then 3.
/// * **7**: the holder's timer byte 1 and the rise's length its pvar `+0x2c`; the riser back down to its height over
///   it; then 1.
pub fn update_0c36(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p = |w: &MobyWorld, o: usize| w.mobys[id].pv_i(o);
    match w.mobys[id].state {
        0 => {
            let save_done = mission_is_done(ctx.game, p(w, pv::MISSION_SAVE));
            let goal_done = mission_is_done(ctx.game, p(w, pv::MISSION_GOAL));
            let m = &mut w.mobys[id];
            m.set_pv_i(pv::CHUNK, ctx.game.variant);
            m.set_pv_i(pv::FADE_T, 0);
            m.set_pv_i(pv::MUSIC_ON, 0);
            m.set_pv_i(pv::SAVED, save_done as i32);
            if goal_done {
                m.set_pv_i(pv::RISEN, 1);
            } else {
                m.set_pv_i(pv::RISEN, 0);
                if let Some(r) = link(w, p(w, pv::RISER)) {
                    let z = w.mobys[r].pos[2];
                    w.mobys[id].set_pv_f(pv::RISER_Z, z);
                    raise(w, ctx, r, z + RISE);
                }
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let hero = ctx.hero.pos;
            let at = |ctx: &Ctx, c: i32| ctx.level.in_cuboid(c, hero);
            if p(w, pv::SAVED) == 0 && mission_is_done(ctx.game, p(w, pv::MISSION_SAVE)) && at(ctx, p(w, pv::AT_SAVE)) {
                // `0x321b70(0, −1)`: the autosave.
                ctx.game.autosave(-1);
                w.mobys[id].set_pv_i(pv::SAVED, 1);
            }
            if ctx.game.variant == 2 && p(w, pv::MUSIC_ON) == 0 && at(ctx, p(w, pv::AT_MUSIC)) {
                w.mobys[id].set_pv_i(pv::MUSIC_ON, 1);
                ctx.host.music_request(p(w, pv::MUSIC), -1);
            }
            if at(ctx, p(w, pv::AT_GOAL)) {
                let goal = p(w, pv::MISSION_GOAL);
                if mission_is_done(ctx.game, goal) { return; }
                ctx.game.mission_done(goal as u8);
                if let Some(k) = link(w, p(w, pv::WAIT_A)) { w.mobys[k].once |= 4; }
                for g in pv::GROUPS { dismiss_group(w, p(w, g)); }
                let m = &mut w.mobys[id];
                m.set_pv_i(pv::CHUNK, 2);
                m.set_pv_i(pv::PLACE_K, 3);
                m.set_state(5, -1);
            }
            if mission_is_done(ctx.game, p(w, pv::MISSION_GOAL)) { return; }
            if link(w, p(w, pv::HOLDER)).is_some_and(|k| w.mobys[k].timer == 1) { return; }
            if p(w, pv::RISEN) != 0 { return; }
            for g in pv::GUNSHIPS {
                if let Some(k) = link(w, p(w, g)) {
                    if w.mobys[k].o_class == GUNSHIP && !gone(w, k) { return; }
                }
            }
            w.mobys[id].set_state(7, -1);
        }
        2 | 3 => {
            let m = &mut w.mobys[id];
            if m.first() { m.set_pv_i(pv::FADE_T, FADE_TICKS); }
            let mut t = m.pv_i(pv::FADE_T);
            let out = countdown(&mut t);
            m.set_pv_i(pv::FADE_T, t);
            if out {
                let next = m.pv_i(pv::NEXT);
                m.set_state(next as u8, -1);
            } else {
                let f = t as f32 * FADE_STEP;
                m.set_pv_f(pv::FADE, if m.state == 2 { 1.0 - f } else { f });
            }
            w.register_draw(id);
        }
        4 => {
            let k = p(w, pv::PLACE_K);
            let cuboid = p(w, pv::PLACES.wrapping_add((k as usize).wrapping_mul(4)));
            ctx.switch_chunk(p(w, pv::CHUNK), p(w, pv::TRACK));
            ctx.game.mission_done(p(w, pv::MISSION_SWITCH) as u8);
            if let Some((pos, rot)) = ctx.level.cuboid_place(cuboid) { place_hero(ctx, pos, rot, 0x6f); }
            body_idle(ctx);
            ctx.game.scenes.fade = 1.0;
            let m = &mut w.mobys[id];
            m.set_pv_i(pv::NEXT, 1);
            m.set_pv_f(pv::FADE, 1.0);
            w.register_draw(id);
            w.mobys[id].set_state(6, -1);
        }
        5 => {
            if ctx.game.scenes.pending == crate::scene::NONE && ctx.game.scenes.mode != crate::scene::SCENE {
                w.mobys[id].set_pv_f(pv::FADE, 1.0);
                w.register_draw(id);
                if link(w, p(w, pv::WAIT_A)).is_some_and(|k| 0x80 < w.mobys[k].state) { w.mobys[id].set_state(4, -1); }
            }
            tick(w, id);
        }
        6 => {
            let in_scene = ctx.game.scenes.mode == crate::scene::SCENE;
            if in_scene {
                ctx.game.scenes.fade = 0.0;
            } else {
                ctx.game.scenes.fade = 1.0;
                w.mobys[id].set_pv_f(pv::FADE, 1.0);
                w.register_draw(id);
                if link(w, p(w, pv::WAIT_B)).is_some_and(|k| 0x80 < w.mobys[k].state) {
                    for o in pv::SHOW {
                        if let Some(k) = link(w, p(w, o)) {
                            w.mobys[k].mode &= !2;
                            solid(w, k, true);
                        }
                    }
                    ctx.game.scenes.fade = 0.0;
                    w.mobys[id].set_state(3, -1);
                }
            }
            tick(w, id);
        }
        7 => {
            if w.mobys[id].first() {
                if let Some(h) = link(w, p(w, pv::HOLDER)) {
                    w.mobys[h].timer = 1;
                    let len = w.mobys[h].pv_i(0x2c);
                    let m = &mut w.mobys[id];
                    m.set_pv_i(pv::RISEN, 1);
                    m.set_pv_i(pv::RISE_LEN, len);
                    m.set_pv_i(pv::TICKS, len);
                }
            }
            let (t, len, z) = (p(w, pv::TICKS), p(w, pv::RISE_LEN), w.mobys[id].pv_f(pv::RISER_Z));
            // The game divides by the length as it is; a length of 0 is left alone here.
            if len != 0 {
                if let Some(r) = link(w, p(w, pv::RISER)) { raise(w, ctx, r, z + t as f32 * RISE / len as f32); }
            }
            let mut t = t;
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(pv::TICKS, t);
            if out { w.mobys[id].set_state(1, -1); }
        }
        _ => {}
    }
}

/// `+0x18` up by one.
fn tick(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::TICKS, m.pv_i(pv::TICKS).wrapping_add(1));
}

/// The riser `r` at height `z` (its `+0x18`).
fn raise(w: &mut MobyWorld, ctx: &mut Ctx, r: usize, z: f32) {
    let m = &mut w.mobys[r];
    m.pos[2] = z;
    m.drive = true;
    w.rebuild(r, &*ctx.host);
}

/// `0x40a998(group)`: every trooper 0x0d0f of the group still there sent away
/// (crate::moby::classes::level09::trooper::dismiss).
fn dismiss_group(w: &mut MobyWorld, g: i32) {
    for k in members(w, g) {
        if w.mobys.get(k).is_some_and(|m| m.o_class == super::super::level09::trooper::CLASS) && !gone(w, k) {
            super::super::level09::trooper::dismiss(w, k);
        }
    }
}

/// `0x2dc618(0)`: the playing body's idle asked for (`0x2d79b0(state, 1)`): Ratchet 0, Clank 0x3f, body 4 0x8d.
fn body_idle(ctx: &mut Ctx) {
    let s = match ctx.hero.mode { 0 => 0, 1 => 0x3f, 4 => 0x8d, _ => return };
    ctx.hero.request_state(s, true);
}

/// `0x40aae8`, its draw callback (the frame's second list): the screen's fade (`0x1b2960` = L00 `0x1b1860`) its
/// `+0x0c` · 1.2, at most 1.
pub fn draw(w: &MobyWorld, game: &mut crate::game::GameState, id: usize) {
    let v = w.mobys[id].pv_f(pv::FADE) * f32::from_bits(0x3f99_999a);
    game.scenes.fade = if 1.0 < v { 1.0 } else { v };
}
