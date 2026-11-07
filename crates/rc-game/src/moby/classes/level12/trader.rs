//! Todano's trader **0x07d1** (`0x3b6518`, one placed; the decomp's `func_US2_L12_003B6518`), on the shared NPC system
//! crate::moby::talk: he gives item 7 for item 0x31. First each tick the actors' big heads of his class
//! (crate::moby::talk::actor_heads, 2.75: `0x3b6438` = L04 `0x424898`). His pvars: the head's and body's aim records
//! `+0x00` / `+0x80`, the look's point `+0x100`, its timers `+0x114` / `+0x118`, his idle timer `+0x11c` (a word) and
//! the part he holds `+0x120` (class 0x127b).
//! * **0**: item 7 had (`0x1a7b07`): gone. Else the part made (`0x314d78` = L00 `0x311320`) with his draw and update
//!   distances, light, ambient and glow, drawn, rebuilt; 2 with his scene 0x75 seen (`0x1397dc & 0x200000`), else 1.
//!   His grid cells (`0x321458`, the rebuild here).
//! * **1**: Ratchet can talk to him (crate::moby::talk::can_talk, 2.5; the talk record marked: not ported) and the
//!   scene 0x75 queued (it must go in): started; 2.
//! * **2** / **5**: in the scene mode he and the part hidden; after the scene both shown — 2: 4 with item 0x31
//!   (`0x1a7b31`), else 3; 5: the part deleted, his group's timers 1 (`0x3203b8` = L00 `0x31c970`) and, item 7 not had
//!   yet, items 0x31 and 0x69 taken, item 7 his (`0x2f7dc8` = L00 `0x2f46e0`), the message 0x1245 (`0x32a8f0` = L00
//!   `0x327448`); he is gone.
//! * **3**: item 0x31 → 4; Ratchet able to talk: the widget's own text 0x1325 for 2 ticks (`0x312a10` = L00 `0x30efb8`,
//!   crate::prompt::Prompt::announce).
//! * **4**: no item 0x31 → 3; Ratchet able to talk: the text 0x1326 in the second box (`0x334db8` = L01 `0x325830`) and △
//!   (`0x138324 & 0x10`) with the scene 0x76 queued: started; 5.
//!
//! In any state but 0: **the idle** — in sequence 0 or 1 with his idle timer out, 1200..2400 ticks again and sequence 2
//! (10 ticks' blend); else his sequence wrapped and equal to a random 0 / 1, sequence 1 when another random pick is 0,
//! else 0. **The look** (crate::moby::talk::look_with) in his looking sequences 0 and 2. The part at his joint 2 (`0x328018` =
//! L00 `0x3253f0`, crate::moby::classes::enemies::place_part).
//!
//! Not ported: the level's NPC busy word `0x1b2be8` while his scenes play, the talk record (`0x334f38`), the autosave
//! after the trade (`0x3110b0(0, −1)`).

use crate::moby::classes::props::{seq, wrapped};
use crate::moby::talk::{self, seen, Layout, Timer};
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x07d1;
const PART: i16 = 0x127b;
const GIFT: usize = 7;
const TOKEN: usize = 0x31;
const R: f32 = 2.5;
const IDLE: usize = 0x11c;
const HOLD: usize = 0x120;

fn had(ctx: &Ctx, item: usize) -> bool { ctx.game.items.get(item).is_some_and(|&b| b != 0) }

fn part(w: &MobyWorld, id: usize) -> Option<usize> { talk::link(w, id, HOLD) }

/// He and his part hidden in the scene mode; true once the scene is over (both shown).
fn wait_scene(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    if ctx.game.scenes.mode == crate::scene::SCENE {
        talk::hide(w, id);
        if let Some(p) = part(w, id) { talk::hide(w, p); }
    }
    if ctx.game.scenes.in_scene() { return false; }
    talk::show(w, id);
    true
}

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if had(ctx, GIFT) { return w.delete(id); }
            let p = w.create(PART);
            let (dd, ud, light, ambient, glow) = { let m = &w.mobys[id]; (m.draw_dist, m.update_dist, m.light, m.ambient, m.glow) };
            let pm = &mut w.mobys[p];
            (pm.draw_dist, pm.update_dist, pm.light, pm.ambient, pm.glow, pm.drawn) = (dd, ud, light, ambient, glow, 1);
            w.rebuild(p, &*ctx.host);
            w.mobys[id].set_pv_i(HOLD, p as i32);
            w.mobys[id].set_state(if seen(ctx, 0x75) { 2 } else { 1 }, -1);
            w.rebuild(id, &*ctx.host);
        }
        1 => {
            if talk::can_talk(w, id, ctx, R, 10) && ctx.game.scenes.queue.push(0x75) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            if !wait_scene(w, id, ctx) { return; }
            if let Some(p) = part(w, id) { talk::show(w, p); }
            w.mobys[id].set_state(if had(ctx, TOKEN) { 4 } else { 3 }, -1);
        }
        3 => {
            if had(ctx, TOKEN) { return w.mobys[id].set_state(4, -1); }
            if talk::can_talk(w, id, ctx, R, 10) { ctx.game.prompt.announce(0x1325, 2); }
        }
        4 => {
            if !had(ctx, TOKEN) { return w.mobys[id].set_state(3, -1); }
            if !talk::offer(w, id, ctx, R, 0x1326, &[]) { return; }
            if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 && ctx.game.scenes.queue.push(0x76) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(5, -1);
            }
        }
        5 => {
            if !wait_scene(w, id, ctx) { return; }
            if let Some(p) = part(w, id) { w.delete(p); }
            w.mobys[id].set_pv_i(HOLD, -1);
            let group = w.mobys[id].group;
            if group != -1 { w.poke_group(group as i32, 1); }
            if !had(ctx, GIFT) {
                for k in [TOKEN, 0x69] { if let Some(b) = ctx.game.items.get_mut(k) { *b = 0; } }
                ctx.game.get_item(GIFT as i32);
                ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x1245, ticks: None });
                w.delete(id);
            }
        }
        _ => {}
    }
}

/// 0x07d1's update (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    talk::actor_heads(w, id, ctx, 2.75);
    states(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    if w.mobys[id].state != 0 {
        let s = seq(w, id, ctx);
        let mut t = w.mobys[id].pv_i(IDLE);
        if s < 2 && countdown(&mut t) {
            w.mobys[id].set_pv_i(IDLE, ctx.game.rng.randf(1200.0, 2400.0) as i32);
            if s != 2 { ctx.host.set_anim(&w.mobys[id], 2, 0, 10); }
        } else {
            if s < 2 { w.mobys[id].set_pv_i(IDLE, t); }
            if wrapped(w, id, ctx) && s as i32 == ctx.game.rng.randi(2) {
                let to = (ctx.game.rng.randi(2) == 0) as u8;
                ctx.host.set_anim(&w.mobys[id], to, 0, 10);
            }
        }
    }
    let s = seq(w, id, ctx);
    let l = Layout { look_seq: if s == 2 { 2 } else { 0 }, fidget: None, idle: IDLE, seen: Timer::Word(0x114), glance: Timer::Word(0x118), pitch: talk::PITCH };
    talk::look_with(w, id, ctx, &l, true, true);
    if let Some(p) = part(w, id) { crate::moby::classes::enemies::place_part(w, id, p, 2, ctx); }
}
