//! 0x0e5e (L09 `0x40b7d8`; the decomp's `func_US2_L09_0040B7D8`): Dobbo's NPC of the talk scenes, on the shared NPC
//! system crate::moby::talk. It has no prompt: Ratchet walking into its cuboid (`+0x118`) is what starts a scene.
//!
//! By state (the flags `0x1395d5` = 0x1d, `0x1395e5` = 0x2d, `0x1395e6` = 0x2e):
//! * **0**: with planet 0xb unlocked (`0x1a7bdb`) it has no collision and its mode is `& !0x1000 | 0x41`. Scene 0x67
//!   seen: 6; the flag 0x1d or scene 0x63 seen: 3; else 1. Going to 3 with Ratchet outside its cuboid or the flag
//!   0x1d off, the arrival scene is not to play this visit ([`crate::moby::talk::Mem::arrival_done`]). Its grid
//!   cells (`0x319eb0`).
//! * **1**: Ratchet in its cuboid: the scene 0x63 queued and started; 2.
//! * **2**: hidden in the scene mode; after it: shown, 3, the flags 0x2d and 0x1d set, and the level left for 0x16
//!   (Dobbo's orbit) with the autosave (`0x2f1398(0x16, 1)`).
//! * **3**: Ratchet in its cuboid. The flag 0x2e off: in the level's first 10 frames (`0x1b2308`) with the flag 0x1d
//!   and the arrival scene not played, the scene 0x66 queued and started, cut in (crate::moby::talk::cut_in), the
//!   arrival done; 4. The flag 0x2e on: the scene 0x67 and then 0xb queued and started, cut in, the arrival done; 5.
//! * **4**: hidden in the scene mode; after it: the checkpoint at Ratchet (`0x356db8`), shown; 3.
//! * **5**: hidden in the scene mode; after it: planet 0xb unlocked with its banner (`0x31eba0` / `0x321640`), its
//!   mission done (`0x30f520`; with a mission not done yet, `0x19b2e8`), the checkpoint, shown, and it is gone.
//! * **6**: Ratchet in its cuboid in the first 10 frames with the flag 0x1d and the arrival not played: the scene 0x66,
//!   cut in, the arrival done; 7.
//! * **7**: hidden in the scene mode; after it: shown, 8, the checkpoint, and it is gone.
//!
//! In any state but 0 its idle and look (crate::moby::talk: looking in sequence 1, fidgeting in 2; its timers are
//! half-words, `+0x110` / `+0x112`, the idle one `+0x114`).
//!
//! Not ported: the level's NPC busy word `0x1b2528` while its scenes play, the talk record, the autosaves (`0x3098d0(0,
//! −1)`).

use crate::moby::talk::{self, flag, seen, set_flag, Layout, Timer};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0e5e;

const LOOK: Layout = Layout { look_seq: 1, fidget: Some(2), idle: 0x114, seen: Timer::Half(0x110), glance: Timer::Half(0x112), pitch: talk::PITCH };
/// Its cuboid.
const ZONE: usize = 0x118;
/// The flags.
const TALKED: usize = 0x1d;
const ORBIT: usize = 0x2d;
const DONE: usize = 0x2e;
/// The planet it unlocks.
const PLANET: i32 = 0xb;

fn inside(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool { ctx.level.in_cuboid(w.mobys[id].pv_i(ZONE), ctx.hero.pos) }

/// The arrival scene 0x66 may start: the level's first 10 frames, the flag 0x1d, not played yet.
fn arrival(w: &MobyWorld, ctx: &Ctx) -> bool { ctx.game.frame < 10 && flag(ctx, TALKED) && !w.talk.arrival_done }

/// The scenes' queue: `ids` queued (the first must go in), started and cut in; the arrival done.
fn play(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, ids: &[i32], next: u8) {
    if !ctx.game.scenes.queue.push(ids[0]) { return; }
    for &s in &ids[1..] { ctx.game.scenes.queue.push(s); }
    ctx.game.scenes.start_queue(ctx.hero.hp);
    talk::cut_in(ctx.game);
    w.talk.arrival_done = true;
    w.mobys[id].set_state(next, -1);
}

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let state = w.mobys[id].state;
    match state {
        0 => {
            if ctx.game.planets.get(PLANET as usize).is_some_and(|&p| p != 0) {
                let m = &mut w.mobys[id];
                m.has_collision = false;
                m.mode = (m.mode & !0x1000) | 0x41;
            }
            let next = if seen(ctx, 0x67) { 6 } else if flag(ctx, TALKED) || seen(ctx, 0x63) { 3 } else { 1 };
            w.mobys[id].set_state(next, -1);
            if next == 3 && (!inside(w, id, ctx) || !flag(ctx, TALKED)) { w.talk.arrival_done = true; }
            w.rebuild(id, &*ctx.host);
        }
        1 => {
            if inside(w, id, ctx) && ctx.game.scenes.queue.push(0x63) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 | 4 | 5 | 7 => {
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::show(w, id);
            let (pos, rot) = (ctx.hero.pos, ctx.hero.rot);
            match state {
                2 => {
                    w.mobys[id].set_state(3, -1);
                    set_flag(ctx, ORBIT);
                    set_flag(ctx, TALKED);
                    ctx.game.go_to_level(0x16, true);
                }
                4 => {
                    ctx.game.save_checkpoint(pos, rot);
                    w.mobys[id].set_state(3, -1);
                }
                5 => {
                    ctx.game.unlock_planet(PLANET);
                    crate::travel::new_planet_banner(ctx.game, PLANET);
                    ctx.game.mission_done(w.mobys[id].mission);
                    ctx.game.save_checkpoint(pos, rot);
                    w.delete(id);
                }
                _ => {
                    w.mobys[id].set_state(8, -1);
                    ctx.game.save_checkpoint(pos, rot);
                    w.delete(id);
                }
            }
        }
        3 => {
            if !inside(w, id, ctx) { return; }
            if !flag(ctx, DONE) {
                if arrival(w, ctx) { play(w, id, ctx, &[0x66], 4); }
            } else {
                play(w, id, ctx, &[0x67, 0xb], 5);
            }
        }
        6 => {
            if inside(w, id, ctx) && arrival(w, ctx) { play(w, id, ctx, &[0x66], 7); }
        }
        _ => {}
    }
}

/// The update (module docs): the states, then, not deleted, the idle and the look.
pub fn update_0e5e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    talk::actor_heads(w, id, ctx, 2.75);
    states(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    talk::idle(w, id, ctx, &LOOK);
    talk::look(w, id, ctx, &LOOK);
}
