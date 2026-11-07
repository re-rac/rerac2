//! The museum's Hacker exhibit, class **0x1312** ([`update_1312`], `0x418008`, 1 placed; the decomp's
//! `func_US2_L30_00418008`): the hacking panel's machine (crate::moby::classes::hacker_panel, L03 `0x3e5c90`) in the
//! museum's own copy. Standing on it with the Hacker, △ opens the game's **mode 7**, the Hacker's puzzle
//! (crate::hacker: the museum's overlay holds the same unit, its board 0x0b28 updated by `0x3d2f48` = L03
//! `0x3e37c0`), after the panel's walk, held pose and camera swing.
//!
//! Its pvars: `+0x00` / `+0x04` two mobys it hooks (−1 none), `+0x08` a camera to release (−1 none), `+0x0c..` the
//! list (14 at most, −1 ends it), `+0x44` the seed (a short), `+0x48` the clock's start, `+0x4c` a new depth's bonus
//! (the panel's layout so far); its own `+0x50` the board (index + 1 here), `+0x54` the editor's menu, `+0x58`
//! completed, `+0x5c` the held item kept, `+0x64` the fade. The placed one: no hooks, no camera, 14 steps of kind 3,
//! seed 500, 180 ticks, 30 a depth.
//!
//! Where it differs from the panel:
//! * **0**: done in the save (`0x21f0c4[uid]`, the museum's copy of the panel's `0x21eec4`, or the uid's count):
//!   hooked with its two mobys (`0x32d208` = L00 `0x32a818`), 8; else 1.
//! * **1**: the prompt 0x1bc9 (owner 0xc); △ keeps the held item (`0x18b068`), asks for 0x33, 2.
//! * **2**: the walk given up or 301 ticks: Ratchet idle after the 301 (`0x2cfb30` = L00 `0x2c8e30`), 1 — no way out
//!   (the item asked for stays asked).
//! * **3**: no sound; Ratchet pushed 0.0125 a tick at most (the panel 0.05); not held: 1 without the way out.
//! * **4**: no sound and no held test: from tick 21 the white, at 31 the field of view back and mode 7 asked for
//!   (`0x32e360(7, 1, moby, 0, 0)` = L00 `0x32c5f0`), 5.
//! * **5**: out of the mode for 6 ticks: 1, no way out.
//! * **6**: out of the mode: the way out (`0x418cf0`: the camera released, kind 3, `0x42c028`; the held item asked
//!   back; the board deleted); completed `+0x58`: 7, its uid counted (with a mission); else Ratchet idle, 1. The
//!   board's "solved" (`0x3d5fb0` = L03 `0x3e6828`) writes `+0x60`, the panel's word, not `+0x58`: per the code a
//!   solved exhibit is never marked completed and stays open.
//! * **7**: hooked with its two mobys, Ratchet idle, 8. **8**: done.
//!
//! Not ported: the editor (state 9: L2 + R2 in state 1 on levels 0x15 / 0x1e, or L1 + L2 + R1 + R2 in state 8; its
//! menu moby 0x118a, `0x413b08` / `0x413ba0`, over the seed, the clock and the list), as the emitter 0x1317's
//! (crate::moby::classes::level21::emitter); the hidden board made at the level start (`0x418bf0` in state 0,
//! placed 1 above it, hidden, never deleted: nothing reads it), the free moby slots it waits for (`0x1b2ac0` ≥ 16), the
//! class streaming of the item (`0x30c478` / `0x30c268`), the camera `+0x08`'s release (`0x42ecc8` = L00 `0x42b9f0`;
//! none placed), Ratchet's word `0x18a042` = 3, as the panel's (crate::moby::classes::hacker_panel).

use crate::hero::physics::*;
use crate::moby::classes::hacker_panel::{frame_point, held, hook, idle, idx, list, walk, AIM, EYE, FOV, FOV_ZOOM, STAND};
use crate::moby::{mode, Ctx, MobyWorld};
use crate::pad::button;

pub const CLASS: i16 = 0x1312;
/// `0x1bc9`: the prompt.
pub const PROMPT: u32 = 0x1bc9;

const SEED: usize = 0x44;
const START: usize = 0x48;
const BONUS: usize = 0x4c;
const BOARD: usize = 0x50;
const COMPLETED: usize = 0x58;
const HELD: usize = 0x5c;
const FADE: usize = 0x64;

/// `0x32e360(7, 1, moby, 0, 0)`: the mode asked for; whether it was taken.
fn ask_mode(id: usize, ctx: &mut Ctx) -> bool { ctx.game.scenes.request(crate::hacker::MODE, true, id as i32, 0, ctx.hero.hp) == 0 }

/// `0x418cf0`: the way out (module docs), with the port's records of the mode (the board's view, HUD and draw).
fn clean_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.game.cameras.camera_script2(3, ctx.hero);
    let h = &mut ctx.game.hacker;
    (h.view, h.hud, h.board) = (None, None, None);
    h.quads.clear();
    ctx.game.puzzle.tan = None;
    ctx.hero.items.request = w.mobys[id].pv_i(HELD);
    if let Some(b) = idx(w, w.mobys[id].pv_i(BOARD) - 1) {
        if w.mobys[b].state < 0x80 && w.mobys[b].o_class == crate::hacker::BOARD { w.delete(b); }
        w.changed = true;
    }
    w.mobys[id].set_pv_i(BOARD, 0);
}

/// `+0x96` up by one; its value before.
fn tick(w: &mut MobyWorld, id: usize) -> u16 {
    let t = w.mobys[id].state_ticks as u16;
    w.mobys[id].state_ticks = t.wrapping_add(1) as i16;
    t
}

/// 0x1312 (`0x418008`, module docs).
pub fn update_1312(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let first = w.mobys[id].first();
    let in_mode = ctx.game.scenes.mode == crate::hacker::MODE;
    let face = add_rot(w.mobys[id].rot[2], f32::from_bits(0x4049_0fdb));
    match w.mobys[id].state {
        0 => {
            let uid = w.mobys[id].uid;
            let g = &*ctx.game;
            let done = g.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || g.uid_count(uid) != 0;
            if done {
                w.mobys[id].set_state(8, -1);
                hook(w, id, true);
            } else {
                w.mobys[id].set_state(1, -1);
            }
        }
        1 => {
            let had = ctx.game.items.get(crate::moby::classes::hacker_panel::ITEM as usize).is_some_and(|&b| b != 0);
            if ctx.hero.ground_moby == Some(id) && had {
                ctx.game.prompt.force(0xc, PROMPT, 1);
                if ctx.hero.items.pressed & button::TRIANGLE != 0 {
                    w.mobys[id].set_pv_i(HELD, ctx.hero.items.item);
                    ctx.hero.items.request = crate::moby::classes::hacker_panel::ITEM;
                    w.mobys[id].set_state(2, -1);
                }
            }
        }
        2 => {
            if first {
                let m = &w.mobys[id];
                let at = frame_point(&m.rows, m.pos, STAND);
                ctx.hero.script_walk_to(face, at, false);
            }
            let s = walk(ctx);
            if s == 2 {
                w.mobys[id].set_state(3, -1);
                return;
            }
            if 0x12d <= tick(w, id) {
                idle(ctx);
            } else if s != 0 {
                return;
            }
            w.mobys[id].set_state(1, -1);
        }
        3 => {
            if first {
                ctx.hero.request_state(0x6f, false);
                ctx.hero.anim_req = Some((20.0, 0x54, 0));
                let (view, euler) = ctx.game.cameras.view_euler();
                ctx.game.cameras.camera_script(view, euler, crate::script_camera::MODE_SWING, 0x78, false, ctx.hero);
                w.mobys[id].set_pv_f(FADE, 0.0);
            }
            let m = &w.mobys[id];
            let to = frame_point(&m.rows, m.pos, STAND);
            let mut push = sub(to, ctx.hero.pos);
            push[2] = 0.0;
            let most = f32::from_bits(0x3c4c_ccce);
            if most < len(push) { push = set_len(push, most); }
            ctx.hero.push = push;
            // `0x325528(&yaw, 0, yaw + π, 1°)`.
            let step = f32::from_bits(0x3c8e_fa37);
            ctx.hero.rot[2] = add_rot(ctx.hero.rot[2], sub_rot(face, ctx.hero.rot[2]).clamp(-step, step));
            if ctx.hero_view.flags & 2 != 0 && ctx.hero_view.seq_b == 0x54 { ctx.hero.anim_req = Some((7.0, 0x55, 0)); }
            if !held(ctx) { w.mobys[id].set_state(1, -1); }
            let (rows, pos) = (ctx.hero.rows, ctx.hero.pos);
            let (eye, aim) = (frame_point(&rows, pos, EYE), frame_point(&rows, pos, AIM));
            let d = sub(aim, eye);
            let yaw = atan2(d[0], d[1]);
            let pitch = -atan2(dist2(eye, aim), d[2]);
            ctx.game.cameras.camera_script_targets(Some(eye), Some([0.0, pitch, yaw]));
            if 0x3c < tick(w, id) { w.mobys[id].set_state(4, -1); }
        }
        4 => {
            if first { ctx.game.hacker.zoom = Some((0x1e, 0x1e, f32::from_bits(FOV), f32::from_bits(FOV_ZOOM))); }
            let t = tick(w, id);
            if 0x15 <= t {
                let mut f = w.mobys[id].pv_f(FADE);
                approach(1.0, f32::from_bits(0x3dee_eef0), &mut f);
                w.mobys[id].set_pv_f(FADE, f);
                ctx.game.hacker.white = f;
                if 0x1f <= t + 1 {
                    ctx.game.hacker.zoom = None;
                    ctx.game.puzzle.tan = Some(crate::moby::classes::electrolyzer::tan());
                    if ask_mode(id, ctx) { w.mobys[id].set_state(5, -1); }
                }
            }
        }
        5 => {
            if !in_mode {
                if 6 <= tick(w, id) { w.mobys[id].set_state(1, -1); }
                return;
            }
            ctx.game.hacker.view = Some(([0.0; 3], crate::hacker::VIEW_ROWS));
            // `0x418bf0`: the board, its turn kept (`+0x34` |= 0x100); shown (`+0x34` &= ~0x81, `+0x31` = 1).
            let b = w.create(crate::hacker::BOARD);
            {
                let bm = &mut w.mobys[b];
                bm.mode = (bm.mode | mode::KEEP_ROWS) & !0x81;
                bm.drawn = 1;
                bm.rot = [0.0, f32::from_bits(0xbfc9_0fdb), 0.0];
                bm.rows = rc_formats::moby_light::rotation_rows(bm.rot).map(|r| [f32::from_bits(r[0]), f32::from_bits(r[1]), f32::from_bits(r[2])]);
                bm.pos = set_len(crate::hacker::VIEW_ROWS[0], crate::hacker::BOARD_DIST);
            }
            w.rebuild(b, &*ctx.host);
            w.changed = true;
            let m = &w.mobys[id];
            let kinds = list(w, id);
            let board = crate::hacker::Board::new(b, id, &kinds, m.pv_s(SEED), m.pv_i(START) as i16, m.pv_i(BONUS) as i16);
            w.mobys[id].set_pv_i(BOARD, b as i32 + 1);
            ctx.game.hacker.board = Some(board);
            w.mobys[id].set_state(6, -1);
        }
        6 => {
            if in_mode {
                // `0x418c60`: the board 1.2 in front of the camera, its update.
                let (eye, rows) = ([0.0; 3], crate::hacker::VIEW_ROWS);
                ctx.game.hacker.view = Some((eye, rows));
                if let Some(b) = ctx.game.hacker.board.as_ref().map(|b| b.moby).filter(|&b| w.mobys.get(b).is_some_and(|m| m.state < 0x80)) {
                    w.mobys[b].pos = add(set_len(rows[0], crate::hacker::BOARD_DIST), eye);
                    crate::hacker::tick(w, ctx);
                }
                return;
            }
            clean_up(w, id, ctx);
            if w.mobys[id].pv_i(COMPLETED) != 0 {
                w.mobys[id].set_state(7, -1);
                let m = &w.mobys[id];
                let (uid, mission) = (m.uid, m.mission);
                if mission != 0xff { ctx.game.count_uid(uid); }
                return;
            }
            idle(ctx);
            w.mobys[id].set_state(1, -1);
        }
        7 => {
            hook(w, id, true);
            idle(ctx);
            w.mobys[id].set_state(8, -1);
        }
        _ => {}
    }
    // The field of view's ease (`0x2de7b0`'s, the camera's step after the mobys), as the panel's.
    if let Some(tan) = ctx.game.hacker.zoom_step() { ctx.game.puzzle.tan = Some(tan); }
}
