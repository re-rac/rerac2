//! 0x1292 (L18 `0x441c08`; the decomp's `func_US2_L18_00441C08`): Damosel's seller of item 0x37, on the shared NPC
//! system crate::moby::talk. It sells only once the three flag bytes `0x13963c`, `0x13963e`, `0x139641` (0x84, 0x86,
//! 0x89) are set; the price is the item's record's (`0x26bcf0 + 0xe0·0x13959f`: its row's `+0x70`).
//!
//! By state:
//! * **0**: the item had (`0x1a7b37`): gone. Its talk scene 0x90 seen: 3; else 1. Its grid cells (`0x338b60`).
//! * **1**: Ratchet in its cuboid (`+0x120`): the scene 0x90 queued and started; 2.
//! * **2**: hidden in the scene mode; after it: Ratchet put at the cuboid's place (crate::moby::talk::put_hero),
//!   shown; 3.
//! * **3**: the three flags set: 4 short of the price, else 5. Else, while Ratchet can talk to it
//!   (crate::moby::talk::can_talk, 1), the text 0x132b with how many of the three are left.
//! * **4** (more than the price → 5): while Ratchet can talk to it the bolt counter and the price in the text 0x132c.
//! * **5** (short → the counter shown, 4): while Ratchet can talk to it the counter and the price in the text 0x132d;
//!   △ queues the sale's scene 0x91 and starts it; 6, its mission done (`0x32e2e8`).
//! * **6**: hidden in the scene mode; from its 21st tick, after the scene: shown; with the item not had yet the price
//!   paid, the counter shown, the moby `+0x124` (an index) told by its timer byte `+0xbc` = 1, the item had
//!   (`0x30eed0`) and asked for in the hand (`0x33d640`); 7, hidden.
//! * **7**: from its 42nd tick: the message 0x1231 (`0x341f68`), the help text 0xa11 in slot 0x68, the checkpoint at
//!   Ratchet (`0x375e08`), and it is gone.
//!
//! First each tick, the actors' big heads of its class (crate::moby::talk::actor_heads, 2.75; `0x441b28`). In any
//! state but 0 its idle and look (crate::moby::talk: looking in sequence 0, fidgeting in 1).
//!
//! Not ported: the level's NPC busy word `0x1b24e8` while its scenes play, the talk record, the autosave
//! (`0x327c10(0, −1)`).

use crate::moby::talk::{self, flag, link, seen, Layout, Timer};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x1292;

const LOOK: Layout = Layout { look_seq: 0, fidget: Some(1), idle: 0x11e, seen: Timer::Word(0x114), glance: Timer::Word(0x118), pitch: talk::PITCH };
const ITEM: i32 = 0x37;
const NEEDS: [usize; 3] = [0x84, 0x86, 0x89];
const ZONE: usize = 0x120;
const TOLD: usize = 0x124;
const R: f32 = 1.0;

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let bolts = ctx.game.bolts as i32;
    let price = ctx.level.items.get(ITEM as usize).map_or(0, |r| r.price);
    let had = ctx.game.items.get(ITEM as usize).is_some_and(|&b| b != 0);
    match w.mobys[id].state {
        0 => {
            if had { return w.delete(id); }
            w.mobys[id].set_state(if seen(ctx, 0x90) { 3 } else { 1 }, -1);
            w.rebuild(id, &*ctx.host);
        }
        1 => {
            if ctx.level.in_cuboid(w.mobys[id].pv_i(ZONE), ctx.hero.pos) && ctx.game.scenes.queue.push(0x90) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::put_hero(ctx, w.mobys[id].pv_i(ZONE));
            talk::show(w, id);
            w.mobys[id].set_state(3, -1);
        }
        3 => {
            let set = NEEDS.iter().filter(|&&k| flag(ctx, k)).count() as i32;
            if set == 3 { return w.mobys[id].set_state(if bolts < price { 4 } else { 5 }, -1); }
            talk::offer(w, id, ctx, R, 0x132b, &[3 - set]);
        }
        4 => {
            if price < bolts { return w.mobys[id].set_state(5, -1); }
            if talk::offer(w, id, ctx, R, 0x132c, &[price]) { ctx.game.show_bolts = Some(0xb4); }
        }
        5 => {
            if bolts < price {
                ctx.game.show_bolts = Some(0xb4);
                return w.mobys[id].set_state(4, -1);
            }
            if !talk::offer(w, id, ctx, R, 0x132d, &[price]) { return; }
            ctx.game.show_bolts = Some(0xb4);
            if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 && ctx.game.scenes.queue.push(0x91) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(6, -1);
                ctx.game.mission_done(w.mobys[id].mission);
            }
        }
        6 => {
            talk::hide_in_scene(w, id, ctx);
            let t = w.mobys[id].state_ticks as u16;
            w.mobys[id].state_ticks = t.wrapping_add(1) as i16;
            if t < 0x14 || ctx.game.scenes.in_scene() { return; }
            talk::show(w, id);
            if !had {
                ctx.game.bolts = (bolts - price) as u32;
                ctx.game.show_bolts = Some(0xb4);
                if let Some(k) = link(w, id, TOLD) { w.mobys[k].timer = 1; }
                ctx.game.get_item(ITEM);
                let held = ctx.game.held;
                ctx.hero.items.ask(held, ITEM);
            }
            w.mobys[id].set_state(7, -1);
            talk::hide(w, id);
        }
        7 => {
            let t = w.mobys[id].state_ticks as u16;
            w.mobys[id].state_ticks = t.wrapping_add(1) as i16;
            if t < 0x29 { return; }
            ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x1231, ticks: None });
            ctx.game.help(0xa11, 0x68);
            let (pos, rot) = (ctx.hero.pos, ctx.hero.rot);
            ctx.game.save_checkpoint(pos, rot);
            w.delete(id);
        }
        _ => {}
    }
}

/// The update (module docs).
pub fn update_1292(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    talk::actor_heads(w, id, ctx, 2.75);
    states(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    talk::idle(w, id, ctx, &LOOK);
    talk::look(w, id, ctx, &LOOK);
}
