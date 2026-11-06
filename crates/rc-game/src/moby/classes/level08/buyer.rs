//! 0x0e3e (L08 `0x4370e0`; the same code is Grelbin's, L19 `0x42afa8`, with the same flags, scenes, texts and price):
//! the crystal buyer. Ratchet's ship is not on the pad until he has paid the buyer in crystals; after that the
//! buyer takes every crystal he brings for bolts. On the shared NPC system crate::moby::talk.
//!
//! **Each tick**, first: the actors' big heads of its class in a scene (crate::moby::talk::actor_heads, 2.3; L08
//! `0x436ff8`); in the ship mode (`0x1a8f00` = 6) hidden with its pitch `+0xf4` −π, else shown with it 0.
//!
//! By state (the crystals `0x1a7a22`, crate::game::GameState::crystals; the price [`PRICE`] crystals, `0x295edc`, the
//! same as `0x295ed8`, the table's other column read with the byte `0x1a7c97`, which is never 0):
//! * **0**: not paid yet (flag 0x1c) Ratchet's ship and its parts are hidden (L08 `0x370240`: mode | 3, no
//!   collision); paid, its prop (`+0x124`, a moby index) is deleted. Its grid cells (`0x32b020`). Paid: 6 with no
//!   crystals, else 7; met (flag 0x1b): 3 short of the price, else 4; else 1.
//! * **1**: Ratchet able to talk to it (crate::moby::talk::can_talk, 2.5): the talk scene 0x59 queued (the flag 0x1b
//!   set) and started; 2.
//! * **2**: in the scene mode it and its prop are hidden; after the scene both are shown again; 3 or 4 by the crystals.
//! * **3** (short; enough → 4): while Ratchet can talk to it the price in the text 0x12f6.
//! * **4** (enough; short → 3): while Ratchet can talk to it the price in the text 0x12e8; △ queues the sale's scene
//!   0x5a, takes the price off the crystals, sets the flag 0x1c and starts it; 5.
//! * **5**: hidden in the scene mode, its prop deleted then; after the scene: shown, its group's timers set to 1
//!   (`0x329bb0`), the flag 0x1c, the ship and its parts shown again (L08 `0x3702a8`: mode bits 1 and 2 off, their
//!   class's collision), the prop deleted, the moby `+0x120` (an index) told by its timer byte `+0xbc` = 1; 6 or 7.
//! * **6** (no crystals; some → 7): while Ratchet can talk to it the text 0x1322 with 0 when no crystal formation,
//!   crystal or 0x1002 is left on the level (`$gp+0x2f68`, `0x436fa0`), else 0x12f9.
//! * **7** (crystals; none → 6): while Ratchet can talk to it the text 0x12e9 with the crystals and their worth, 1000
//!   bolts each (10 000 in the challenge mode, `0x1a7a12`); △ pays it: the bolts, the bolt counter shown, one of the
//!   bolt classes' sounds (sound 0 of class 13..17, `0x36ebe0`), the crystals 0; 8.
//! * **8**: its ticks counted (to 0x7ff8); on every fourth of the ticks 41..89 the bolt sound again; while Ratchet can
//!   talk to it the text 0x1322 with the formations left; when he cannot, 6.
//!
//! In any state but 0 its idle and look (crate::moby::talk: looking in sequence 1, fidgeting in 2).
//!
//! Not ported: the level's NPC busy word `0x1b2728` set while its scenes play, the talk record, the debug print
//! `0x2b72f0(0x1b1f60)`, the autosave after the sale (`0x319bc8(0, −1)`). The formations left are counted when they
//! are shown (the game keeps the count `$gp+0x2f68` from the set-up, the frame 5, the payment and Ratchet's crystal
//! pickups; it only differs from a live count for the tick between a formation's break and its crystal's start).

use super::crystal;
use crate::moby::talk::{self, flag, link, set_flag, Layout, Timer};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0e3e;

/// Its idle and look: looking in sequence 1, fidgeting in 2.
const LOOK: Layout = Layout { look_seq: 1, fidget: Some(2), idle: 0x11e, seen: Timer::Word(0x114), glance: Timer::Word(0x118), pitch: talk::PITCH };

/// `0x295edc`: the price in crystals.
const PRICE: i32 = 10;
/// The flags: met (`0x1395d3`) and paid (`0x1395d4`).
const MET: usize = 0x1b;
const PAID: usize = 0x1c;
/// Its pvars: the moby told after the sale, its prop (moby indices, −1 none).
const TOLD: usize = 0x120;
const PROP: usize = 0x124;
const R: f32 = 2.5;

fn crystals(ctx: &Ctx) -> i32 { ctx.game.crystals as i16 as i32 }

/// Short of the price.
fn short(ctx: &Ctx) -> bool { crystals(ctx) < PRICE }

/// `0x436fa0`: the crystal formations, crystals and 0x1002 left (`0x329b48`: of the class, not 0xfe / 0xfd).
fn left(w: &MobyWorld) -> i32 {
    let live = |c: i16| w.mobys.iter().filter(|o| o.o_class == c && o.state != 0xfe && o.state != 0xfd).count();
    (live(crystal::CRYSTAL) + live(0x1002) + live(crystal::LEFT)) as i32
}

/// L08 `0x370240` / `0x3702a8`: Ratchet's parked ship and its parts hidden (mode | 3, no collision) or shown (mode
/// bits 1 and 2 off, their class's collision).
fn ship(w: &mut MobyWorld, ctx: &Ctx, on: bool) {
    let ids: Vec<usize> = ctx.game.travel.ship.into_iter().chain(ctx.game.ship_parts.iter().copied()).collect();
    for i in ids {
        let Some(c) = w.mobys.get(i).map(|m| m.o_class) else { continue };
        let coll = w.class(c).is_some_and(|k| k.has_collision);
        let m = &mut w.mobys[i];
        if on { m.mode &= !3; m.has_collision = coll; } else { m.mode |= 3; m.has_collision = false; }
    }
}

/// The bolts' sound: sound 0 of one of the bolt classes 13..17 (`0x36ebe0(0, 0, m, 0x328a10(13, 17))`).
fn bolt_sound(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let class = ctx.game.rng.rand_range(13, 17) as i16;
    ctx.host.play_sound_of(&w.mobys[id], class, 0, 0);
}

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    talk::actor_heads(w, id, ctx, f32::from_bits(0x4013_3333));
    if ctx.game.scenes.mode == 6 {
        talk::hide(w, id);
        w.mobys[id].rot[1] = f32::from_bits(0xc049_0fdb);
    } else {
        talk::show(w, id);
        w.mobys[id].rot[1] = 0.0;
    }
    match w.mobys[id].state {
        0 => {
            if !flag(ctx, PAID) {
                ship(w, ctx, false);
            } else if let Some(p) = link(w, id, PROP) {
                w.delete(p);
                w.mobys[id].set_pv_i(PROP, -1);
            }
            w.rebuild(id, &*ctx.host);
            let next = if flag(ctx, PAID) {
                if crystals(ctx) < 1 { 6 } else { 7 }
            } else if flag(ctx, MET) {
                if short(ctx) { 3 } else { 4 }
            } else {
                1
            };
            w.mobys[id].set_state(next, -1);
        }
        1 => {
            if talk::can_talk(w, id, ctx, R, 10) && ctx.game.scenes.queue.push(0x59) {
                set_flag(ctx, MET);
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            talk::hide_in_scene(w, id, ctx);
            let prop = link(w, id, PROP);
            if let Some(p) = prop { talk::hide_in_scene(w, p, ctx); }
            if ctx.game.scenes.in_scene() { return; }
            talk::show(w, id);
            if let Some(p) = prop { talk::show(w, p); }
            w.mobys[id].set_state(if short(ctx) { 3 } else { 4 }, -1);
        }
        3 => {
            if !short(ctx) { return w.mobys[id].set_state(4, -1); }
            talk::offer(w, id, ctx, R, 0x12f6, &[PRICE]);
        }
        4 => {
            if short(ctx) { return w.mobys[id].set_state(3, -1); }
            if !talk::offer(w, id, ctx, R, 0x12e8, &[PRICE]) { return; }
            if ctx.hero.items.pressed & crate::pad::button::TRIANGLE == 0 { return; }
            ctx.game.scenes.queue.push(0x5a);
            ctx.game.crystals = (crystals(ctx) - PRICE) as i16 as u16;
            set_flag(ctx, PAID);
            ctx.game.scenes.start_queue(ctx.hero.hp);
            w.mobys[id].set_state(5, -1);
        }
        5 => {
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.mode == crate::scene::SCENE {
                if let Some(p) = link(w, id, PROP) {
                    w.delete(p);
                    w.mobys[id].set_pv_i(PROP, -1);
                }
            }
            if ctx.game.scenes.in_scene() { return; }
            talk::show(w, id);
            let group = w.mobys[id].group;
            if group != -1 { w.poke_group(group as i32, 1); }
            set_flag(ctx, PAID);
            ship(w, ctx, true);
            if let Some(p) = link(w, id, PROP) {
                w.delete(p);
                w.mobys[id].set_pv_i(PROP, -1);
            }
            if let Some(t) = link(w, id, TOLD) { w.mobys[t].timer = 1; }
            w.mobys[id].set_state(if crystals(ctx) < 1 { 6 } else { 7 }, -1);
        }
        6 => {
            if 0 < crystals(ctx) { return w.mobys[id].set_state(7, -1); }
            if left(w) == 0 { talk::offer(w, id, ctx, R, 0x1322, &[0]); } else { talk::offer(w, id, ctx, R, 0x12f9, &[]); }
        }
        7 => {
            let c = crystals(ctx);
            if c < 1 { return w.mobys[id].set_state(6, -1); }
            let each = if ctx.game.challenge { 10 } else { 1 };
            let worth = c * each * 1000;
            if !talk::offer(w, id, ctx, R, 0x12e9, &[c, worth]) { return; }
            if ctx.hero.items.pressed & crate::pad::button::TRIANGLE == 0 { return; }
            ctx.game.bolts = (ctx.game.bolts as i32 + worth) as u32;
            bolt_sound(w, id, ctx);
            ctx.game.crystals = 0;
            ctx.game.show_bolts = Some(0xb4);
            w.mobys[id].set_state(8, -1);
        }
        8 => {
            let m = &mut w.mobys[id];
            m.state_ticks = (m.state_ticks as u16).wrapping_add(1).min(0x7ff8) as i16;
            let t = m.state_ticks as u16;
            if (0x29..=0x59).contains(&t) && t & 3 == 0 { bolt_sound(w, id, ctx); }
            let n = left(w);
            if !talk::offer(w, id, ctx, R, 0x1322, &[n]) { w.mobys[id].set_state(6, -1); }
        }
        _ => {}
    }
}

/// The update (module docs): the states, then, not deleted, the idle and the look that every state reaches.
pub fn update_0e3e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    states(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    talk::idle(w, id, ctx, &LOOK);
    talk::look(w, id, ctx, &LOOK);
}
