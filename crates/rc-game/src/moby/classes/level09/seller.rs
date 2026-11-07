//! Dobbo's seller of the coordinates to planet 0xa, **0x0e10** ([`update_0e10`], L09 `0x409908`, one placed; the
//! decomp's `func_US2_L09_00409904`), on the shared NPC system crate::moby::talk. The price [`PRICE`] bolts
//! (`0x28e360`, the same as the challenge mode's `0x28e364`). Its pvars: `+0x00` the cuboid it talks from, `+0x08`
//! the cuboid Ratchet is put at after its scenes (−1 none).
//!
//! By state:
//! * **0**: planet 0xa unlocked (`0x1a7bda`): 9. Its scene 0x60 seen: 6 short of the price, else 7 (sub 1); else its
//!   scene 0x5f seen: updated at any distance, 4 (sub 1) once it has been hooked before (`0x1395d7`), else 3; else
//!   1. Its grid cells (`0x319eb0`).
//! * **1**: Ratchet in `+0x00`: the scene 0x5f queued and started; 2.
//! * **2**: hidden in the scene mode; after it Ratchet put at `+0x08`, shown, 3.
//! * **3**: updated at any distance; hooked (`+0xbe & 4`): `0x1395d7` set, 4 (sub 1).
//! * **4**: updated at any distance; Ratchet in `+0x00`: the scene 0x60 queued and started at once
//!   (`0x36ee10` = L00 `0x371b60`, crate::moby::talk::cut_in); 5.
//! * **5**: updated within 0x40; hidden in the scene mode; after it Ratchet put at `+0x08`, shown, `0x1395d8` set, 6
//!   or 7 by the bolts.
//! * **6** (more than the price → 7): while Ratchet can talk to it (within 2.5) the price in the text 0x1323.
//! * **7** (short → 6): while Ratchet can talk to it the price in the text 0x1324; △ with the planet's scene 0xa
//!   going in: the price paid, the scenes 0x61 and 0x62 after it, started; 8.
//! * **8**: hidden in the scene mode; after it: shown, planet 0xa unlocked with its banner (`0x31eba0` / `0x321640` =
//!   L00 `0x322730` / `0x325270`), 9; with a mission (`+0xb0`) the mission done (unless it is) and the checkpoint at
//!   Ratchet (`0x356db8` = L00 `0x359b20`); else the autosave.
//! * **9**: nothing.
//!
//! Not ported: the level's NPC busy word `0x1b25a8` while its scenes play, the save's log of its state
//! (`0x357330` = L06 `0x39a2f0` on the list `0x21dde0`), the autosave (`0x3098d0` = L00 `0x30d658(0, −1)`), the
//! debug prints (`0x2a82f0`).

use crate::moby::talk;
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0e10;

/// `0x28e360`: the price.
const PRICE: i32 = 10_000;
const PLANET: i32 = 0xa;
const ZONE: usize = 0;
const PUT: usize = 8;
const R: f32 = 2.5;
/// `0x1395d7` / `0x1395d8`: hooked once; its second scene seen.
const HOOKED: usize = 0x1f;
const SOLD_SCENE: usize = 0x20;

fn by_bolts(ctx: &Ctx) -> u8 { if (ctx.game.bolts as i32) < PRICE { 6 } else { 7 } }

/// After a scene: Ratchet at `+0x08`, shown.
fn after_scene(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let put = w.mobys[id].pv_i(PUT);
    if put != -1 { talk::put_hero(ctx, put); }
    talk::show(w, id);
}

/// 0x0e10 (`0x409908`): see the module.
pub fn update_0e10(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let bolts = ctx.game.bolts as i32;
    match w.mobys[id].state {
        0 => {
            if ctx.game.planets.get(PLANET as usize).is_some_and(|&p| p != 0) { return w.mobys[id].set_state(9, -1); }
            if talk::seen(ctx, 0x60) {
                let s = by_bolts(ctx);
                w.mobys[id].set_state(s, 1);
            } else if talk::seen(ctx, 0x5f) {
                w.mobys[id].update_dist = 0xff;
                if talk::flag(ctx, HOOKED) { w.mobys[id].set_state(4, 1); } else { w.mobys[id].set_state(3, 0); }
            } else {
                w.mobys[id].set_state(1, 0);
            }
            w.rebuild(id, &*ctx.host);
        }
        1 => {
            if ctx.level.in_cuboid(w.mobys[id].pv_i(ZONE), ctx.hero.pos) && ctx.game.scenes.queue.push(0x5f) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            after_scene(w, id, ctx);
            w.mobys[id].set_state(3, -1);
        }
        3 => {
            w.mobys[id].update_dist = 0xff;
            if w.mobys[id].once & 4 != 0 {
                talk::set_flag(ctx, HOOKED);
                w.mobys[id].set_state(4, 1);
            }
        }
        4 => {
            w.mobys[id].update_dist = 0xff;
            if ctx.level.in_cuboid(w.mobys[id].pv_i(ZONE), ctx.hero.pos) && ctx.game.scenes.queue.push(0x60) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                talk::cut_in(ctx.game);
                w.mobys[id].set_state(5, -1);
            }
        }
        5 => {
            w.mobys[id].update_dist = 0x40;
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            after_scene(w, id, ctx);
            talk::set_flag(ctx, SOLD_SCENE);
            let s = by_bolts(ctx);
            w.mobys[id].set_state(s, -1);
        }
        6 => {
            if PRICE < bolts { return w.mobys[id].set_state(7, -1); }
            talk::offer(w, id, ctx, R, 0x1323, &[PRICE]);
        }
        7 => {
            if bolts < PRICE { return w.mobys[id].set_state(6, -1); }
            if !talk::offer(w, id, ctx, R, 0x1324, &[PRICE]) { return; }
            if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 && ctx.game.scenes.queue.push(PLANET as _) {
                ctx.game.bolts = (bolts - PRICE) as u32;
                ctx.game.scenes.queue.push(0x61);
                ctx.game.scenes.queue.push(0x62);
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.mobys[id].set_state(8, -1);
            }
        }
        8 => {
            talk::hide_in_scene(w, id, ctx);
            if ctx.game.scenes.in_scene() { return; }
            talk::show(w, id);
            ctx.game.unlock_planet(PLANET);
            crate::travel::new_planet_banner(ctx.game, PLANET);
            w.mobys[id].set_state(9, -1);
            let mission = w.mobys[id].mission;
            if mission != 0xff {
                let done = ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + mission as usize).is_some_and(|&b| b & 0x80 != 0);
                if !done { ctx.game.mission_done(mission); }
                let (p, r) = (ctx.hero.pos, ctx.hero.rot);
                ctx.game.save_checkpoint(p, r);
            }
        }
        _ => {}
    }
}
