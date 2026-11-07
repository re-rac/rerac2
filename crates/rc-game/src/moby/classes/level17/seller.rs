//! 0x11a6 (L17 `0x41f898`; the decomp's `func_US2_L17_0041F898`): Smolg's seller of the coordinates to planet 0x12
//! (Damosel), on the shared NPC system crate::moby::talk. The price [`PRICE`] bolts (`0x28c604`, the same as
//! `0x28c600`, the table's other column read with the byte `0x1a7c97`, never 0).
//!
//! By state:
//! * **0**: planet 0x12 unlocked (`0x1a7be2`): gone. Its talk scene 0x89 seen: 3 short of the price, else 4; else 1.
//!   Its grid cells (`0x31ec80`).
//! * **1**: Ratchet in its cuboid (`+0x120`): the scene 0x89 queued and started; 2.
//! * **2**: hidden in the scene mode; after it: Ratchet put at the cuboid's place (crate::moby::talk::put_hero),
//!   shown, 3 or 4 by the bolts.
//! * **3** (more than the price → 4): while Ratchet can talk to it (crate::moby::talk::can_talk, 4) the bolt counter
//!   shown and the price in the text 0x1329.
//! * **4** (short → the counter shown, 3): while Ratchet can talk to it the counter and the price in the text 0x132a;
//!   △ queues the sale's scene 0x8a (it must go in), then 0x12 and 0x8b, and starts them; 5.
//! * **5**: hidden in the scene mode; after it: shown; with the planet not unlocked yet the price paid, the counter
//!   shown, planet 0x12 unlocked with its banner (`0x324728` / `0x327360`), the moby `+0x124` (an index) told by its
//!   timer byte `+0xbc` = 1, and it is gone.
//!
//! First each tick, the actors' big heads of its class (crate::moby::talk::actor_heads, 2.75; `0x41f7b8`). In any
//! state but 0 its idle and look (crate::moby::talk: looking in sequence 0, no fidget: the idle timer only restarts).
//!
//! The talk record (crate::moby::talk::mark) is marked after the scene 0x89 starts and with the texts 0x1329 / 0x132a;
//! the level's NPC busy word `0x1b2968` (crate::moby::talk::busy) is set while its scenes play (2, 5); the autosave
//! after the sale (`0x30e158(0, −1)`, crate::game::GameState::autosave).

use crate::moby::talk::{self, link, seen, Layout, Timer};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x11a6;

const LOOK: Layout = Layout { look_seq: 0, fidget: None, idle: 0x11e, seen: Timer::Word(0x114), glance: Timer::Word(0x118), pitch: talk::PITCH };
/// `0x28c604`: the price.
const PRICE: i32 = 40_000;
const PLANET: i32 = 0x12;
const ZONE: usize = 0x120;
const TOLD: usize = 0x124;
const R: f32 = 4.0;

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let bolts = ctx.game.bolts as i32;
    let had = ctx.game.planets.get(PLANET as usize).is_some_and(|&p| p != 0);
    match w.mobys[id].state {
        0 => {
            if had { return w.delete(id); }
            let next = if seen(ctx, 0x89) { if bolts < PRICE { 3 } else { 4 } } else { 1 };
            w.mobys[id].set_state(next, -1);
            w.rebuild(id, &*ctx.host);
        }
        1 => {
            if ctx.level.in_cuboid(w.mobys[id].pv_i(ZONE), ctx.hero.pos) && ctx.game.scenes.queue.push(0x89) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                talk::mark(w, id, ctx);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            talk::busy(ctx, true);
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::busy(ctx, false);
            talk::put_hero(ctx, w.mobys[id].pv_i(ZONE));
            talk::show(w, id);
            w.mobys[id].set_state(if bolts < PRICE { 3 } else { 4 }, -1);
        }
        3 => {
            if PRICE < bolts { return w.mobys[id].set_state(4, -1); }
            if talk::offer(w, id, ctx, R, 0x1329, &[PRICE]) {
                ctx.game.show_bolts = Some(0xb4);
                talk::mark(w, id, ctx);
            }
        }
        4 => {
            if bolts < PRICE {
                ctx.game.show_bolts = Some(0xb4);
                return w.mobys[id].set_state(3, -1);
            }
            if !talk::offer(w, id, ctx, R, 0x132a, &[PRICE]) { return; }
            ctx.game.show_bolts = Some(0xb4);
            talk::mark(w, id, ctx);
            if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 && ctx.game.scenes.queue.push(0x8a) {
                ctx.game.scenes.queue.push(0x12);
                ctx.game.scenes.queue.push(0x8b);
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(5, -1);
            }
        }
        5 => {
            talk::busy(ctx, true);
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::busy(ctx, false);
            talk::show(w, id);
            if had { return; }
            ctx.game.bolts = (bolts - PRICE) as u32;
            ctx.game.show_bolts = Some(0xb4);
            ctx.game.unlock_planet(PLANET);
            crate::travel::new_planet_banner(ctx.game, PLANET);
            if let Some(t) = link(w, id, TOLD) { w.mobys[t].timer = 1; }
            ctx.game.autosave(-1);
            w.delete(id);
        }
        _ => {}
    }
}

/// The update (module docs).
pub fn update_11a6(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    talk::actor_heads(w, id, ctx, 2.75);
    states(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    talk::idle(w, id, ctx, &LOOK);
    talk::look(w, id, ctx, &LOOK);
}
