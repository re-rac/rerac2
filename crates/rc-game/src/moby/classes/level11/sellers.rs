//! Joba's two talking NPCs, on the shared NPC system crate::moby::talk: they follow Oozla's Tractor Beam seller
//! 0x0ace (crate::moby::classes::oozla::update_0ace). First each tick, the actors' big heads of their class
//! (crate::moby::talk::actor_heads, 2.75: L11 `0x463288` and `0x3f9970`, the same function twice).
//!
//! **0x0e95** ([`update_0e95`], `0x463368`; the decomp's `func_US2_L11_00463368`): the seller of item 8, its price
//! item 8's record's (`0x279c20 + 0xe0·0x139570`, crate::game::GameState::record). By state:
//! * **0**: item 8 had (`0x1a7b08`): gone. Its talk scene 0x6b seen (the scene bit `0x1397d0 & 0x800`): 2, else 1.
//!   Its grid cells (`0x32ff70`).
//! * **1**: Ratchet in its cuboid (`+0x120`): the scene 0x6b queued (it must go in) and started; 2.
//! * **2**: hidden in the scene mode; after it: Ratchet put at the cuboid's place (crate::moby::talk::put_hero),
//!   shown, 3 short of the price, else 4. (Started in 2 with the scene seen, it puts Ratchet there at once.)
//! * **3** (more than the price → 4): while Ratchet can talk to it (crate::moby::talk::offer, 2.5) the bolt counter
//!   shown and the price in the text 0x130e.
//! * **4** (short → the counter shown, 3): the same with the text 0x130d; △ queues the sale's scene 0x6c and starts
//!   it; 5.
//! * **5**: hidden in the scene mode; after it: shown, its group's timers set to 1 (`0x32ed60`); item 8 not had yet:
//!   the price paid, the counter shown, item 8 his (`0x306b28`); Ratchet put at the cuboid `+0x124` (−1 none), the
//!   message 0x1230 (`0x3396f0`), and it is gone.
//!
//! In any state but 0 its idle and look (crate::moby::talk: looking in sequence 0, fidgeting in 1).
//!
//! **0x0b7c** ([`update_0b7c`], `0x3f9a50`; `func_US2_L11_003F9A50`): the giver of the Charge Boots (item 0x36). By
//! state:
//! * **0**: the boots had (`0x1a7b36`): gone. Its grid cells; its second scene 0x70 seen (`0x1397d8 & 0x10000`): 3,
//!   else 1.
//! * **1**: Ratchet in its cuboid (`+0x120`): the scene 0x70 queued and started; 2.
//! * **2**: hidden in the scene mode; after it: shown; 3.
//! * **3**: with the flag 0x57 (`0x13960f`) and Ratchet in the cuboid: the scene 0x71 queued and started, cut in
//!   (crate::moby::talk::cut_in); 4.
//! * **4**: hidden in the scene mode; after it: shown; the boots not had yet: his (`0x306b28`), asked for
//!   (`0x334f80` = L00 `0x3227d8`), the message 0x1232; with its mission (`+0xb0`, not 0xff) the mission done
//!   (`0x325620`) and the checkpoint at Ratchet (`0x36e3e0`). It is gone.
//!
//! In any state but 0 its idle and look (looking in sequence 1, no fidget: the idle timer only restarts).
//!
//! Not ported: the level's NPC busy word `0x1b3368` while their scenes play, the talk record (`0x343b10`), the
//! autosaves (`0x31f9d0(0, −1)`).

use crate::moby::talk::{self, flag, seen, Layout, Timer};
use crate::moby::{Ctx, MobyWorld};

pub const SELLER: i16 = 0x0e95;
pub const BOOTS: i16 = 0x0b7c;

const ITEM: i32 = 8;
const BOOTS_ITEM: i32 = 0x36;
const ZONE: usize = 0x120;
const AFTER: usize = 0x124;
const R: f32 = 2.5;
/// The flag byte 0x57 (`0x13960f`).
const BOOTS_FLAG: usize = 0x57;

const SELLER_LOOK: Layout = Layout { look_seq: 0, fidget: Some(1), idle: 0x11e, seen: Timer::Word(0x114), glance: Timer::Word(0x118), pitch: talk::PITCH };
const BOOTS_LOOK: Layout = Layout { look_seq: 1, fidget: None, idle: 0x11e, seen: Timer::Word(0x114), glance: Timer::Word(0x118), pitch: talk::PITCH };

fn had(ctx: &Ctx, item: i32) -> bool { ctx.game.items.get(item as usize).is_some_and(|&b| b != 0) }

/// Ratchet in the cuboid `+0x120` and the scene `scene` queued (it must go in) and started.
fn ask_scene(w: &MobyWorld, id: usize, ctx: &mut Ctx, scene: i32) -> bool {
    if !ctx.level.in_cuboid(w.mobys[id].pv_i(ZONE), ctx.hero.pos) || !ctx.game.scenes.queue.push(scene) { return false; }
    ctx.game.scenes.start_queue(ctx.hero.hp);
    true
}

fn states_0e95(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let price = ctx.game.record(&ctx.level.items, ITEM).map_or(0, |r| r.price);
    let bolts = ctx.game.bolts as i32;
    let had = had(ctx, ITEM);
    match w.mobys[id].state {
        0 => {
            if had { return w.delete(id); }
            w.mobys[id].set_state(if seen(ctx, 0x6b) { 2 } else { 1 }, -1);
            w.rebuild(id, &*ctx.host);
        }
        1 => {
            if ask_scene(w, id, ctx, 0x6b) { w.mobys[id].set_state(2, -1); }
        }
        2 => {
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::put_hero(ctx, w.mobys[id].pv_i(ZONE));
            talk::show(w, id);
            w.mobys[id].set_state(if bolts < price { 3 } else { 4 }, -1);
        }
        3 => {
            if price < bolts { return w.mobys[id].set_state(4, -1); }
            if talk::offer(w, id, ctx, R, 0x130e, &[price]) { ctx.game.show_bolts = Some(0xb4); }
        }
        4 => {
            if bolts < price {
                ctx.game.show_bolts = Some(0xb4);
                return w.mobys[id].set_state(3, -1);
            }
            if !talk::offer(w, id, ctx, R, 0x130d, &[price]) { return; }
            ctx.game.show_bolts = Some(0xb4);
            if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 && ctx.game.scenes.queue.push(0x6c) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(5, -1);
            }
        }
        5 => {
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::show(w, id);
            let group = w.mobys[id].group as i32;
            w.poke_group(group, 1);
            if !had {
                ctx.game.bolts = (bolts - price) as u32;
                ctx.game.show_bolts = Some(0xb4);
                ctx.game.get_item(ITEM);
            }
            let after = w.mobys[id].pv_i(AFTER);
            if after != -1 { talk::put_hero(ctx, after); }
            ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x1230, ticks: None });
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x0e95's update (module docs).
pub fn update_0e95(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    talk::actor_heads(w, id, ctx, 2.75);
    states_0e95(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    talk::idle(w, id, ctx, &SELLER_LOOK);
    talk::look(w, id, ctx, &SELLER_LOOK);
}

fn states_0b7c(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if had(ctx, BOOTS_ITEM) { return w.delete(id); }
            w.rebuild(id, &*ctx.host);
            w.mobys[id].set_state(if seen(ctx, 0x70) { 3 } else { 1 }, -1);
        }
        1 => {
            if ask_scene(w, id, ctx, 0x70) { w.mobys[id].set_state(2, -1); }
        }
        3 => {
            if flag(ctx, BOOTS_FLAG) && ask_scene(w, id, ctx, 0x71) {
                talk::cut_in(ctx.game);
                w.mobys[id].set_state(4, -1);
            }
        }
        2 | 4 => {
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::show(w, id);
            if w.mobys[id].state == 2 { return w.mobys[id].set_state(3, -1); }
            if !had(ctx, BOOTS_ITEM) {
                ctx.game.get_item(BOOTS_ITEM);
                let held = ctx.game.held;
                ctx.hero.items.ask(held, BOOTS_ITEM);
                ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x1232, ticks: None });
                let mission = w.mobys[id].mission;
                if mission != 0xff {
                    ctx.game.mission_done(mission);
                    let (pos, rot) = (ctx.hero.pos, ctx.hero.rot);
                    ctx.game.save_checkpoint(pos, rot);
                }
            }
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x0b7c's update (module docs).
pub fn update_0b7c(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    talk::actor_heads(w, id, ctx, 2.75);
    states_0b7c(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    talk::idle(w, id, ctx, &BOOTS_LOOK);
    talk::look(w, id, ctx, &BOOTS_LOOK);
}
