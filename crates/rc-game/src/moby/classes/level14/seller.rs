//! The prison's seller **0x10f3** (L14 `0x427280`, 1 placed; the decomp's `func_US2_L14_00427280`), on the shared NPC
//! system crate::moby::talk: he sells item 0x31 (with 0x69) for [`PRICE`] bolts (`0x291878`, the same as `0x29187c`,
//! the table's other column read with the byte `0x1a7c97`). His pvars: the head's and body's aim records `+0x00` /
//! `+0x80`, the look's point `+0x100`, its timers `+0x114` / `+0x118`, his idle timer `+0x11c` (a word) and the part
//! he holds `+0x120` (class 0x1142; a link here, 0 none).
//!
//! First each tick the actors' big heads of class 0x11d3 (crate::moby::talk::actor_heads_of, 2.5; `0x335478` = L01
//! `0x321cc8`). By state:
//! * **0**: no part; item 7 or 0x31 had (`0x1a7b07` / `0x1a7b31`) or the scene 0x7f seen: gone. His grid cells
//!   (`0x3253d8` = L01 `0x313ae0`, the rebuild here). The scene 0x7e seen: the part ([`part`], `0x4271d8`) and 2;
//!   else 1.
//! * **1**: Ratchet can talk to him (crate::moby::talk::can_talk, 2.5; the talk record marked: not ported) and the
//!   scene 0x7e queued (it must go in): started; 2.
//! * **2**: hidden in the scene mode; after the scene shown, turned about (π, `0x2f7a68` = L00 `0x2efe60`); with the
//!   part (made again when gone) sequence 1, then 3 short of the price, else 4.
//! * **3** (the price or more → 4): while Ratchet can talk to him the bolt counter for 180 ticks (`0x3152e0` = L00
//!   `0x30d910`) and the text 0x1327 with the price in the second box (`0x339f68` = L01 `0x325830`).
//! * **4** (short → the counter, 3): while Ratchet can talk the counter and the text 0x1328; △ (`0x138324 & 0x10`) pays
//!   the price and, with the scene 0x7f queued, starts it; 5.
//! * **5**: he and the part hidden in the scene mode; after the scene he is shown, his group's timers 1 (`0x3242f0` =
//!   L00 `0x31c970`), the part deleted, item 0x31 not had: items 0x69 and 0x31 his; turned about, sequence 0; 6.
//! * In 1, 2 and 6 **the idle** (crate::moby::talk::idle_word: looking in sequence 0, no fidget).
//!
//! **The look** every tick (crate::moby::talk::look_turns, its own limits: the yaw ±50° with the head 0.5 and the
//! body 0.3 of it, the pitch −1°..30°), the part at his joint 2 (`0x32cf00` = L00 `0x3253f0`,
//! crate::moby::classes::enemies::place_part; the original's between the asks and the aims).
//!
//! Not ported: the level's NPC busy word `0x1b2268` while his scenes play, the talk record (`0x33a0e8` = L01
//! `0x3259b0`), the autosave after the sale (`0x315028(0, −1)` = L00 `0x30d658`).

use crate::moby::talk::{self, seen, Layout, Timer, Turns};
use crate::moby::{ai, Ctx, MobyWorld};

pub const CLASS: i16 = 0x10f3;
const PART: i16 = 0x1142;
/// The actor whose head the big heads take.
const ACTOR: i16 = 0x11d3;
/// `0x291878`: the price.
const PRICE: i32 = 20_000;
const R: f32 = 2.5;
const HOLD: usize = 0x120;

const LOOK: Layout = Layout { look_seq: 0, fidget: None, idle: 0x11c, seen: Timer::Word(0x114), glance: Timer::Word(0x118), pitch: talk::PITCH };
const TURNS: Turns = Turns {
    yaw: f32::from_bits(0x3f5f_66f4),
    head: 0.5,
    body: f32::from_bits(0x3e99_999a),
    pitch: (f32::from_bits(0xbc8e_fa36), f32::from_bits(0x3f06_0a93)),
};

fn had(ctx: &Ctx, item: usize) -> bool { ctx.game.items.get(item).is_some_and(|&b| b != 0) }

fn held(w: &MobyWorld, id: usize) -> Option<usize> { ai::link(&w.mobys[id], HOLD).filter(|&k| k < w.mobys.len()) }

/// `0x4271d8`: the part he holds, made when there is none or it is gone (`0x318cf0` = L00 `0x311320`) with his draw
/// and update distances, light and ambient, drawn, at his joint 2.
fn part(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> usize {
    if let Some(p) = held(w, id).filter(|&p| !matches!(w.mobys[p].state, 0xfe | 0xfd)) { return p; }
    let p = w.create(PART);
    let (dd, ud, light, ambient) = { let m = &w.mobys[id]; (m.draw_dist, m.update_dist, m.light, m.ambient) };
    let pm = &mut w.mobys[p];
    (pm.draw_dist, pm.update_dist, pm.light, pm.ambient, pm.drawn) = (dd, ud, light, ambient, 1);
    ai::set_link(&mut w.mobys[id], HOLD, Some(p));
    crate::moby::classes::enemies::place_part(w, id, p, 2, ctx);
    p
}

/// Turned about (π).
fn turn_about(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.rot[2] = crate::hero::physics::add_rot(m.rot[2], std::f32::consts::PI);
}

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let bolts = ctx.game.bolts as i32;
    match w.mobys[id].state {
        0 => {
            ai::set_link(&mut w.mobys[id], HOLD, None);
            if had(ctx, 7) || had(ctx, 0x31) || seen(ctx, 0x7f) { return w.delete(id); }
            w.rebuild(id, &*ctx.host);
            if seen(ctx, 0x7e) {
                part(w, id, ctx);
                w.mobys[id].set_state(2, -1);
            } else {
                w.mobys[id].set_state(1, -1);
            }
        }
        1 => {
            if talk::can_talk(w, id, ctx, R, 10) && ctx.game.scenes.queue.push(0x7e) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            turn_about(w, id);
            talk::show(w, id);
            part(w, id, ctx);
            if crate::moby::classes::props::seq(w, id, ctx) != 1 { ctx.host.set_anim(&w.mobys[id], 1, 0, 10); }
            w.mobys[id].set_state(if bolts < PRICE { 3 } else { 4 }, -1);
        }
        3 => {
            if PRICE <= bolts { return w.mobys[id].set_state(4, -1); }
            if talk::can_talk(w, id, ctx, R, 10) {
                ctx.game.show_bolts = Some(0xb4);
                talk::say(ctx, 0x1327, &[PRICE]);
            }
        }
        4 => {
            if bolts < PRICE {
                ctx.game.show_bolts = Some(0xb4);
                return w.mobys[id].set_state(3, -1);
            }
            if !talk::can_talk(w, id, ctx, R, 10) { return; }
            ctx.game.show_bolts = Some(0xb4);
            talk::say(ctx, 0x1328, &[PRICE]);
            if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 {
                ctx.game.bolts = (bolts - PRICE) as u32;
                if ctx.game.scenes.queue.push(0x7f) {
                    ctx.game.scenes.start_queue(ctx.hero.hp);
                    w.mobys[id].set_state(5, -1);
                }
            }
        }
        5 => {
            talk::hide_in_scene(w, id, ctx);
            if let Some(p) = held(w, id) { if ctx.game.scenes.mode == crate::scene::SCENE { talk::hide(w, p); } }
            if ctx.game.scenes.in_scene() { return; }
            talk::show(w, id);
            let group = w.mobys[id].group;
            if group != -1 { w.poke_group(group as i32, 1); }
            if let Some(p) = held(w, id) {
                w.delete(p);
                ai::set_link(&mut w.mobys[id], HOLD, None);
            }
            if !had(ctx, 0x31) {
                for k in [0x69, 0x31] { if let Some(b) = ctx.game.items.get_mut(k) { *b = 1; } }
            }
            turn_about(w, id);
            if crate::moby::classes::props::seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 10); }
            w.mobys[id].set_state(6, -1);
        }
        _ => {}
    }
}

/// 0x10f3's update (module docs).
pub fn update_10f3(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0x124 { w.mobys[id].pvars.resize(0x124, 0); }
    talk::actor_heads_of(w, id, ctx, ACTOR, 2.5);
    states(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    if matches!(w.mobys[id].state, 1 | 2 | 6) { talk::idle_word(w, id, ctx, &LOOK); }
    // The asks in any sequence: the looking sequence is the one showing.
    let l = Layout { look_seq: crate::moby::classes::props::seq(w, id, ctx), ..LOOK };
    talk::look_turns(w, id, ctx, &l, true, true, &TURNS);
    if let Some(p) = held(w, id) { crate::moby::classes::enemies::place_part(w, id, p, 2, ctx); }
}
