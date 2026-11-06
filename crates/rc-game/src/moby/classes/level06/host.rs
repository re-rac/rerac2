//! 0x0ccf (L06 `0x4318a0`; the decomp's `func_US2_L06_004318A0`): Notak's seller of the coordinates to planet 7, on
//! the shared NPC system crate::moby::talk (G-UI-010). The price [`PRICE`] bolts (`0x295950`, the same as `0x295954`,
//! the table's other column read with the byte `0x1a7c97`).
//!
//! By state:
//! * **0**: planet 7 unlocked (`0x1a7bd7`): gone. Its grid cells (`0x35dc70`); its pvar `+0x124` set: 6; its talk
//!   scene 0x4a seen (`0x1397b0 & 0x400`): 2 (which puts Ratchet at its place at once); else 1.
//! * **1**: Ratchet in its cuboid (`+0x128`): the scene 0x4a queued and started; 2.
//! * **2**: hidden in the scene mode; after it: shown, Ratchet put at (456.08, 251.24, 68.92) facing −120°
//!   (`0x2f96e8` = L00 `0x2b9e68`), 3 short of the price, else 4.
//! * **3** (more than the price → 4): while Ratchet can talk to it (crate::moby::talk::offer, 2.5) the bolt counter
//!   shown and the price in the text 0x1320.
//! * **4** (short → the counter shown, 3): the same with the text 0x1321; △ queues the sale's scenes 0x4b, 7 and 0x4c
//!   and starts them; 5.
//! * **5**: hidden in the scene mode; after it: shown, its group's timers set to 1 (`0x35cb88`), the flag 0x15 set
//!   (`0x1395cd`); planet 7 not unlocked yet: unlocked with its banner (`0x363748` / `0x3654f0`), the price paid, the
//!   counter shown; the moby `+0x120` (an index, −1 none) told by its timer byte, and it is gone.
//! * **6**: hidden in the scene mode, else shown; gone once planet 7 is unlocked.
//!
//! In any state but 0 its idle: in its looking sequence 2 with its idle timer (`+0x11e`, a half-word) out, the timer
//! 1200..2400 again and its fidget 3 started (a 10..20 ticks' blend, `0x35b9d8`); else, its sequence having wrapped,
//! the looking one again (the same blend) unless in it, and its animation speed 0.9..1.1 (`0x35ba30`). Then its look
//! (crate::moby::talk::look: in sequence 2).
//!
//! Not ported: the level's NPC busy word `0x1b2568` while its scenes play, the talk record (`0x370ab0`), the autosave
//! (`0x34cc58(0, −1)`); the debug print `0x2e3ff0` of state 5 does nothing.

use crate::moby::classes::props::{seq, wrapped};
use crate::moby::talk::{self, link, seen, set_flag, Layout, Timer};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0ccf;

const LOOK: Layout = Layout { look_seq: 2, fidget: Some(3), idle: 0x11e, seen: Timer::Word(0x114), glance: Timer::Word(0x118), pitch: talk::PITCH };
/// `0x295950`: the price.
const PRICE: i32 = 6000;
const PLANET: i32 = 7;
const TOLD: usize = 0x120;
const STAY: usize = 0x124;
const ZONE: usize = 0x128;
const R: f32 = 2.5;
/// The flag byte 0x15 (`0x1395cd`).
const SOLD: usize = 0x15;

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let bolts = ctx.game.bolts as i32;
    let had = ctx.game.planets.get(PLANET as usize).is_some_and(|&p| p != 0);
    match w.mobys[id].state {
        0 => {
            if had { return w.delete(id); }
            w.rebuild(id, &*ctx.host);
            let next = if w.mobys[id].pv_i(STAY) != 0 { 6 } else if seen(ctx, 0x4a) { 2 } else { 1 };
            w.mobys[id].set_state(next, -1);
        }
        1 => {
            if ctx.level.in_cuboid(w.mobys[id].pv_i(ZONE), ctx.hero.pos) {
                ctx.game.scenes.queue.push(0x4a);
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::show(w, id);
            let pos = [f32::from_bits(0x43e4_0a3d), f32::from_bits(0x437b_3d71), f32::from_bits(0x4289_d70a)];
            ctx.hero.teleport(pos, [0.0, 0.0, f32::from_bits(0xc006_0a93)]);
            ctx.hero.request_state(0, true);
            w.mobys[id].set_state(if bolts < PRICE { 3 } else { 4 }, -1);
        }
        3 => {
            if PRICE < bolts { return w.mobys[id].set_state(4, -1); }
            if talk::offer(w, id, ctx, R, 0x1320, &[PRICE]) { ctx.game.show_bolts = Some(0xb4); }
        }
        4 => {
            if bolts < PRICE {
                ctx.game.show_bolts = Some(0xb4);
                return w.mobys[id].set_state(3, -1);
            }
            if !talk::offer(w, id, ctx, R, 0x1321, &[PRICE]) { return; }
            ctx.game.show_bolts = Some(0xb4);
            if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 {
                for s in [0x4b, 7, 0x4c] { ctx.game.scenes.queue.push(s); }
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
            set_flag(ctx, SOLD);
            if !had {
                ctx.game.unlock_planet(PLANET);
                crate::travel::new_planet_banner(ctx.game, PLANET);
                ctx.game.bolts = (bolts - PRICE) as u32;
                ctx.game.show_bolts = Some(0xb4);
            }
            if let Some(t) = link(w, id, TOLD) { w.mobys[t].timer = 1; }
            w.delete(id);
        }
        6 => {
            if ctx.game.scenes.mode == crate::scene::SCENE { talk::hide(w, id) } else { talk::show(w, id) }
            if had { w.delete(id); }
        }
        _ => {}
    }
}

/// The idle (module docs). Not in state 0.
fn idle(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 { return; }
    let s = seq(w, id, ctx);
    let run_out = |w: &mut MobyWorld| {
        let mut v = w.mobys[id].pv_s(LOOK.idle) as i32;
        let done = crate::moby::countdown(&mut v);
        w.mobys[id].set_pv_s(LOOK.idle, v as i16);
        done
    };
    if s == LOOK.look_seq && run_out(w) {
        let t = ctx.game.rng.randf(1200.0, 2400.0) as i32;
        w.mobys[id].set_pv_s(LOOK.idle, t as i16);
        if s != 3 {
            let blend = ctx.game.rng.rand_range(10, 20) as u8;
            ctx.host.set_anim(&w.mobys[id], 3, 0, blend);
        }
    } else if wrapped(w, id, ctx) {
        if s != LOOK.look_seq {
            let blend = ctx.game.rng.rand_range(10, 20) as u8;
            ctx.host.set_anim(&w.mobys[id], LOOK.look_seq, 0, blend);
        }
        let speed = ctx.game.rng.randf(0.9, 1.1);
        ctx.host.set_anim_speed(&w.mobys[id], speed);
    }
}

/// The update (module docs).
pub fn update_0ccf(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    states(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    idle(w, id, ctx);
    talk::look(w, id, ctx, &LOOK);
}
