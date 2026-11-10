//! The orbits' floating debris (L22 / L23 addresses; 0x0dce and 0x0cb6 are Dobbo orbit's only). Any hit breaks it.
//!
//! * **0x0aa9** (`0x3d5878`, 127 placed on level 22) and **0x1213** (L23 `0x4291b0`, 196 placed): [`update_rock`]
//!   (the decomp's `func_L22_003D4428` → `0x3d5770`). Updated from any distance. Hit by anything, it:
//!   * plays one of three sounds of another class at random (0x1213: class 0xf7f's 0xb..0xd; else 0xe0b's 0xc..0xe);
//!   * makes a smoke burst at it (`0x326580` = L00 `0x3210a8`: size 2, the light 13);
//!   * plays its class sound 0;
//!   * breaks into pieces 0 and 1 (`0x31dd30`: moving 0.233 a tick along the hit's way, 3..6 / 3..6, no sound);
//!   * goes.
//! * **0x0dce** (`0x3fb748`, 17 placed): the same with class 0xe0b's sounds and 0.25 along the hit's way.
//! * **0x0cb6** (`0x3e9cb0`, 12 placed): [`update_0cb6`]: the sound and the smoke burst only, then it goes.

use crate::hero::physics::set_len;
use crate::moby::{blast, shards, Ctx, MobyWorld};

pub const ROCK: i16 = 0x0aa9;
pub const ROCK_B: i16 = 0x1213;
pub const ICE: i16 = 0x0dce;
pub const DUST: i16 = 0x0cb6;

/// The hit's way, or none without a hit.
fn hit_way(w: &MobyWorld, id: usize) -> Option<[f32; 3]> {
    w.hit_message(id).map(|h| [h.tmpl.dir[0], h.tmpl.dir[1], h.tmpl.dir[2]])
}

fn sound_and_smoke(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (class, base) = if w.mobys[id].o_class == ROCK_B { (0x0f7f, 0xb) } else { (0x0e0b, 0xc) };
    let k = base + ctx.game.rng.randi(3);
    ctx.host.play_sound_of(&w.mobys[id], class, k, 0);
    let pos = w.mobys[id].pos;
    blast::smoke_burst(w, ctx, 2.0, 13.0, Some(id), pos, -1);
}

pub fn update_rock(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].update_dist = 0xff;
    let Some(way) = hit_way(w, id) else { return };
    sound_and_smoke(w, id, ctx);
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    let speed = if w.mobys[id].o_class == ICE { 0.25 } else { 0.233_333_35 };
    shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 3, set_len(way, speed), 0xff);
    w.delete(id);
}

pub fn update_0cb6(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].update_dist = 0xff;
    if hit_way(w, id).is_none() { return; }
    sound_and_smoke(w, id, ctx);
    w.delete(id);
}
