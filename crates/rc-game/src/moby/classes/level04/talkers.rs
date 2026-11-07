//! Barlow's three talking NPCs, on the shared NPC system crate::moby::talk (G-UI-010). First each tick, the actors'
//! big heads of their class (crate::moby::talk::actor_heads, 2.75: `0x424898`, `0x436210`, `0x43afb0`, one function
//! three times).
//!
//! **0x0c38** ([`update_0c38`], `0x424978`; the decomp's `func_US2_L04_00424978`): the NPC who gives planet 5's
//! coordinates and item 0x30. By state:
//! * **0**: planet 5 unlocked (`0x1a7bd5`): gone. Its grid cells (`0x33f628`); its first scene 0x37 seen (`0x13979c
//!   & 0x800000`): 3, else 1.
//! * **1**: Ratchet can talk to it (crate::moby::talk::can_talk, 4): the scene 0x37 queued and started; 2.
//! * **2**: hidden in the scene mode; after it: shown; 3.
//! * **3**: with the flag 0x4d (`0x139605`) and Ratchet able to talk to it (2.5): the scene 0x39 queued (it must go
//!   in), then 5 and 0x3a, started and cut in (crate::moby::talk::cut_in); 4.
//! * **4**: hidden in the scene mode; after it: shown, planet 5 unlocked (`0x3446f8`), item 0x30 his (`0x3153d0`),
//!   the planet's banner (`0x3475b0`); with its mission (`+0xb0`, not 0xff) the mission done and the checkpoint at
//!   Ratchet (`0x3345f8` / `0x37d520`). It is gone.
//!
//! In any state but 0 its idle (looking in sequence 0, no fidget; its idle timer a word, `+0x11c`) and look, whose
//! pitch it works out and never stores (crate::moby::talk::look_with).
//!
//! **0x0d3e** ([`update_0d3e`], `0x4362f0`; `func_US2_L04_004362F0`): the seller of item 0x27 by the cuboid `+0x120`,
//! its price item 0x27's record's (`0x27b4f0 + 0xe0·0x13958f`). Its links (moby indices, −1 none): `+0x124` a moby
//! the sale brings back, `+0x128` one told by its timer byte, `+0x12c` one that must be gone before the talk; `+0x11c`
//! its prop 0xd99. By state:
//! * **0** (from the level's second frame, `0x1b2008`; before it nothing runs at all): item 0x27 had (`0x1a7b27`): the
//!   moby `+0x124` brought back (below), the moby `+0x12c` gone, and it is gone. Else its grid cells, its prop 0xd99
//!   made (`0x332670`: its update and draw distances, drawn, rebuilt), 1; its first scene 0x3b seen (`0x1397a0 &
//!   0x8000000`): its sequence 1 (10 ticks' blend), 4 and the moby `+0x12c` gone.
//! * **1**: Ratchet can talk to it (4) with the moby `+0x12c` gone (state 0xfe / 0xfd): the flag 0x11 set
//!   (`0x1395c9`), its animation speed 1, the scene 0x3b queued and started; 2.
//! * **2**: hidden in the scene mode; after it: Ratchet put at (338, 366.81, 63) facing its yaw + π, shown, its
//!   sequence 1; 3 short of the price, else 4.
//! * **3** (more than the price → 4): while Ratchet can talk to it (2.5) the bolt counter and the price in the text
//!   0x12f2.
//! * **4** (short → the counter shown, 3): while Ratchet can talk to it in its cuboid the counter and the price in the
//!   text 0x12f1; △ queues the sale's scene 0x3c and starts it; 5.
//! * **5**: hidden in the scene mode; after it: Ratchet put at the same place facing its yaw − π/2, shown, its group's
//!   timers set to 1; item 0x27 not had yet: the price paid, the counter shown, item 0x27 his and asked for
//!   (`0x3153d0` / `0x3447a0`), the message 0x122a (`0x349a68`); the moby `+0x128` told (timer byte 1), the moby
//!   `+0x124` brought back, the help 0x9dc in slot 0x33 (`0x315f60`), and it is gone.
//!
//! **Brought back** (the moby `+0x124`): its state byte 2, its pvar `+0x9c` 0, its uid's records cleared (`0x21ded4`
//! / `0x21eb44` + uid, the save's count `0x19b4a8`).
//!
//! Every tick (past the second frame, not gone): from state 2 its pitch angle (`+0xf0`) 0 and its look
//! (crate::moby::talk: looking in sequence 1; no idle); before, its animation stopped (speed 0) and its pitch −5°, the
//! aims alone. Its prop at its joint 2 (`0x347730`, crate::moby::classes::enemies::place_part).
//!
//! **0x0da8** ([`update_0da8`], `0x43b090`; `func_US2_L04_0043B090`): the NPC of the scene 0x3d. By state:
//! * **0**: the moby `+0x120` without collision (`+0x98` 0) and hidden; its grid cells; the scene 0x3d seen
//!   (`0x1397a4 & 0x20000000`): 4, else 1.
//! * **1**: Ratchet can talk to it (6.5): the scene 0x3d queued and started; 2.
//! * **2**: hidden in the scene mode; after it: shown, Ratchet put 4 along its facing row (`+0xc0`) with his own
//!   angles; 3.
//! * **3**: the flag 0xa4 (`0x13965c`) not set yet: set; 4.
//! * **4**: nothing.
//!
//! In any state but 0 its idle (looking in sequence 0, fidgeting in 1; the idle timer a word, `+0x11c`) and look
//! (the pitch within ±10°).
//!
//! The talk record (`0x3531a8`, crate::moby::talk::mark) is marked once Ratchet can talk before each first scene
//! (0x0c38's 1 and 3, 0x0d3e's 1, 0x0da8's 1) and with 0x0d3e's texts 0x12f2 / 0x12f1; the level's NPC busy word
//! `0x1b2228` (crate::moby::talk::busy) is set while their scenes play; the autosaves (`0x32df60(0, −1)`,
//! crate::game::GameState::autosave): 0x0c38's at its end without a mission (with one, the checkpoint), 0x0d3e's
//! after the sale.
//!
//! Not ported: 0x0c38's `$gp+0x18e4` = 1 after its first scene (`0x40b7f0`: read only by the
//! hoverbike 0x0bbc, G-LVL-010); 0x0d3e's prop's `+0x38` taken from Ratchet's moby (`0x341a70`); 0x0da8's message
//! of state 3: the game's mode 4 asked for (`0x34f440(4, 1, 10, 0, 0)`, the in-game dialog kind 10) with the text
//! 0x31c3 and the button 0x312c (`0x312878` / `0x3127d8(0x40, 0)`), a dialog the port does not have.

use crate::hero::physics::{add, add_rot, scale, V3};
use crate::moby::talk::{self, can_talk, flag, link, say, seen, set_flag, Layout, Timer};
use crate::moby::{Ctx, MobyWorld};

pub const PLANET_GIVER: i16 = 0x0c38;
pub const SELLER: i16 = 0x0d3e;
pub const TALKER: i16 = 0x0da8;

fn had(ctx: &Ctx, item: i32) -> bool { ctx.game.items.get(item as usize).is_some_and(|&b| b != 0) }

/// The scenes `ids` queued (the first must go in) and started.
fn play(ctx: &mut Ctx, ids: &[i32]) -> bool {
    if !ctx.game.scenes.queue.push(ids[0]) { return false; }
    for &s in &ids[1..] { ctx.game.scenes.queue.push(s); }
    ctx.game.scenes.start_queue(ctx.hero.hp);
    true
}

/// `0x2dfe20(pos, rot, 0, 1)`: Ratchet put there, idle.
fn place_hero(ctx: &mut Ctx, pos: V3, rot: V3) {
    ctx.hero.teleport(pos, rot);
    ctx.hero.request_state(0, true);
}

/// Sequence `s` unless it is in it already (10 ticks' blend).
fn to_seq(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8) {
    if crate::moby::classes::props::seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, 10); }
}

// 0x0c38.

const GIVER_LOOK: Layout = Layout { look_seq: 0, fidget: None, idle: 0x11c, seen: Timer::Word(0x114), glance: Timer::Word(0x118), pitch: talk::PITCH };
const GIVER_PLANET: i32 = 5;
const GIVER_ITEM: i32 = 0x30;
/// The flag byte 0x4d (`0x139605`).
const GIVER_FLAG: usize = 0x4d;

fn states_0c38(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if ctx.game.planets.get(GIVER_PLANET as usize).is_some_and(|&p| p != 0) { return w.delete(id); }
            w.rebuild(id, &*ctx.host);
            w.mobys[id].set_state(if seen(ctx, 0x37) { 3 } else { 1 }, -1);
        }
        1 => {
            if !can_talk(w, id, ctx, 4.0, 10) { return; }
            talk::mark(w, id, ctx);
            if play(ctx, &[0x37]) { w.mobys[id].set_state(2, -1); }
        }
        2 => {
            talk::busy(ctx, true);
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::busy(ctx, false);
            talk::show(w, id);
            w.mobys[id].set_state(3, -1);
        }
        3 => {
            if !flag(ctx, GIVER_FLAG) || !can_talk(w, id, ctx, 2.5, 10) { return; }
            talk::mark(w, id, ctx);
            if play(ctx, &[0x39, 5, 0x3a]) {
                talk::cut_in(ctx.game);
                w.mobys[id].set_state(4, -1);
            }
        }
        4 => {
            talk::busy(ctx, true);
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::busy(ctx, false);
            talk::show(w, id);
            ctx.game.unlock_planet(GIVER_PLANET);
            ctx.game.get_item(GIVER_ITEM);
            crate::travel::new_planet_banner(ctx.game, GIVER_PLANET);
            let mission = w.mobys[id].mission;
            if mission != 0xff {
                ctx.game.mission_done(mission);
                let (pos, rot) = (ctx.hero.pos, ctx.hero.rot);
                ctx.game.save_checkpoint(pos, rot);
            } else {
                ctx.game.autosave(-1);
            }
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x0c38's update (module docs).
pub fn update_0c38(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    talk::actor_heads(w, id, ctx, 2.75);
    states_0c38(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    talk::idle_word(w, id, ctx, &GIVER_LOOK);
    talk::look_with(w, id, ctx, &GIVER_LOOK, false, true);
}

// 0x0d3e.

const SELLER_LOOK: Layout = Layout { look_seq: 1, fidget: None, idle: 0x11e, seen: Timer::Word(0x114), glance: Timer::Word(0x118), pitch: talk::PITCH };
const SELLER_ITEM: i32 = 0x27;
const SELLER_ZONE: usize = 0x120;
const PROP: usize = 0x11c;
const PROP_CLASS: i16 = 0x0d99;
const BACK: usize = 0x124;
const TOLD: usize = 0x128;
const FIRST_GONE: usize = 0x12c;
/// The flag byte 0x11 (`0x1395c9`).
const SELLER_FLAG: usize = 0x11;
/// Where its scenes leave Ratchet.
const SPOT: V3 = [338.0, f32::from_bits(0x43b7_67ae), 63.0];

/// The moby `+0x124` brought back (module docs).
fn bring_back(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(o) = link(w, id, BACK) else { return };
    let m = &mut w.mobys[o];
    m.state = 2;
    m.set_pv_i(0x9c, 0);
    let uid = m.uid;
    if let Ok(u) = usize::try_from(uid) {
        if let Some(b) = ctx.game.uid_pending.get_mut(u) { *b = 0; }
        if let Some(b) = ctx.game.uid_dead.get_mut(u) { *b = 0; }
    }
    ctx.game.clear_uid(uid);
}

fn delete_link(w: &mut MobyWorld, id: usize, off: usize) {
    if let Some(o) = link(w, id, off) { w.delete(o); }
}

/// Ratchet at [`SPOT`] facing its yaw turned by `turn`.
fn place_at_spot(w: &MobyWorld, id: usize, ctx: &mut Ctx, turn: f32) {
    let mut rot = w.mobys[id].rot;
    rot[2] = add_rot(rot[2], turn);
    place_hero(ctx, SPOT, rot);
}

fn states_0d3e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let price = ctx.game.record(&ctx.level.items, SELLER_ITEM).map_or(0, |r| r.price);
    let bolts = ctx.game.bolts as i32;
    let had = had(ctx, SELLER_ITEM);
    match w.mobys[id].state {
        0 => {
            if had {
                bring_back(w, id, ctx);
                delete_link(w, id, FIRST_GONE);
                return w.delete(id);
            }
            if seen(ctx, 0x3b) {
                to_seq(w, id, ctx, 1);
                w.mobys[id].set_state(2, -1);
            }
            w.rebuild(id, &*ctx.host);
            if w.class(PROP_CLASS).is_some() {
                let c = w.create(PROP_CLASS);
                let (ud, dd) = (w.mobys[id].update_dist, w.mobys[id].draw_dist);
                let m = &mut w.mobys[c];
                (m.update_dist, m.draw_dist, m.drawn) = (ud, dd, 1);
                w.rebuild(c, &*ctx.host);
                w.mobys[id].set_pv_i(PROP, c as i32);
            } else {
                w.mobys[id].set_pv_i(PROP, 0);
            }
            w.mobys[id].set_state(1, -1);
            if seen(ctx, 0x3b) {
                to_seq(w, id, ctx, 1);
                w.mobys[id].set_state(4, -1);
                delete_link(w, id, FIRST_GONE);
            }
        }
        1 => {
            let first_gone = link(w, id, FIRST_GONE).is_none_or(|o| matches!(w.mobys[o].state, 0xfe | 0xfd));
            if !can_talk(w, id, ctx, 4.0, 10) || !first_gone { return; }
            talk::mark(w, id, ctx);
            set_flag(ctx, SELLER_FLAG);
            ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            ctx.game.scenes.queue.push(0x3b);
            ctx.game.scenes.start_queue(ctx.hero.hp);
            w.mobys[id].set_state(2, -1);
        }
        2 => {
            talk::busy(ctx, true);
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::busy(ctx, false);
            place_at_spot(w, id, ctx, f32::from_bits(0x4049_0fdc));
            talk::show(w, id);
            to_seq(w, id, ctx, 1);
            w.mobys[id].set_state(if bolts < price { 3 } else { 4 }, -1);
        }
        3 => {
            if price < bolts { return w.mobys[id].set_state(4, -1); }
            if talk::offer(w, id, ctx, 2.5, 0x12f2, &[price]) {
                ctx.game.show_bolts = Some(0xb4);
                talk::mark(w, id, ctx);
            }
        }
        4 => {
            if bolts < price {
                ctx.game.show_bolts = Some(0xb4);
                return w.mobys[id].set_state(3, -1);
            }
            if !can_talk(w, id, ctx, 2.5, 10) || !ctx.level.in_cuboid(w.mobys[id].pv_i(SELLER_ZONE), ctx.hero.pos) { return; }
            ctx.game.show_bolts = Some(0xb4);
            say(ctx, 0x12f1, &[price]);
            talk::mark(w, id, ctx);
            if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 {
                ctx.game.scenes.queue.push(0x3c);
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(5, -1);
            }
        }
        5 => {
            talk::busy(ctx, true);
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::busy(ctx, false);
            place_at_spot(w, id, ctx, f32::from_bits(0xbfc9_0fdc));
            talk::show(w, id);
            let group = w.mobys[id].group as i32;
            w.poke_group(group, 1);
            if !had {
                ctx.game.bolts = (bolts - price) as u32;
                ctx.game.show_bolts = Some(0xb4);
                ctx.game.get_item(SELLER_ITEM);
                let held = ctx.game.held;
                ctx.hero.items.ask(held, SELLER_ITEM);
                ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x122a, ticks: None });
            }
            if let Some(t) = link(w, id, TOLD) { w.mobys[t].timer = 1; }
            bring_back(w, id, ctx);
            ctx.game.help(0x9dc, 0x33);
            ctx.game.autosave(-1);
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x0d3e's update (module docs).
pub fn update_0d3e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    talk::actor_heads(w, id, ctx, 2.75);
    if w.mobys[id].state == 0 && w.frame < 2 { return; }
    states_0d3e(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    let talked = 2 <= w.mobys[id].state;
    if talked {
        w.mobys[id].rot[0] = 0.0;
    } else {
        ctx.host.set_anim_speed(&w.mobys[id], 0.0);
        w.mobys[id].rot[0] = f32::from_bits(0xbdb2_b8c4);
    }
    talk::look_with(w, id, ctx, &SELLER_LOOK, true, talked);
    if let Some(p) = w.mobys[id].pv_link(PROP).filter(|&p| p != id && w.mobys.get(p).is_some_and(|m| m.state < 0x80)) {
        crate::moby::classes::enemies::place_part(w, id, p, 2, ctx);
    }
}

// 0x0da8.

const TALKER_LOOK: Layout = Layout { look_seq: 0, fidget: Some(1), idle: 0x11c, seen: Timer::Word(0x114), glance: Timer::Word(0x118), pitch: 0.174_532_95 };
const TALKER_LINK: usize = 0x120;
/// The flag byte 0xa4 (`0x13965c`).
const TALKER_FLAG: usize = 0xa4;

fn states_0da8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if let Some(o) = link(w, id, TALKER_LINK) {
                w.mobys[o].has_collision = false;
                talk::hide(w, o);
            }
            w.rebuild(id, &*ctx.host);
            w.mobys[id].set_state(if seen(ctx, 0x3d) { 4 } else { 1 }, -1);
        }
        1 => {
            if can_talk(w, id, ctx, 6.5, 10) {
                talk::mark(w, id, ctx);
                ctx.game.scenes.queue.push(0x3d);
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            talk::busy(ctx, true);
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::busy(ctx, false);
            talk::show(w, id);
            let m = &w.mobys[id];
            let pos = add(scale(m.rows[0], 4.0), m.pos);
            let rot = ctx.hero.rot;
            place_hero(ctx, pos, rot);
            w.mobys[id].set_state(3, -1);
        }
        3 => {
            if !flag(ctx, TALKER_FLAG) {
                set_flag(ctx, TALKER_FLAG);
                w.mobys[id].set_state(4, -1);
            }
        }
        _ => {}
    }
}

/// 0x0da8's update (module docs).
pub fn update_0da8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    talk::actor_heads(w, id, ctx, 2.75);
    states_0da8(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    talk::idle_word(w, id, ctx, &TALKER_LOOK);
    talk::look(w, id, ctx, &TALKER_LOOK);
}
