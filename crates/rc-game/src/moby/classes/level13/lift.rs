//! Boldan's lift **0x116d** (`0x424588`, 9 placed; the decomp's `func_L13_004230B0`): a platform between its placed
//! height (the top) and `+0x68` below it that Ratchet rides by standing on it. Its pvars: `+0x20` its platform block
//! (crate::moby::platform; flag 1 set at the start), `+0x60` / `+0x74` and `+0x78` / `+0x7c` two words it keeps (no
//! reader here), `+0x64` its speed, `+0x68` its travel, `+0x80` / `+0x90` its bottom and top places (`+0x88` / `+0x98`
//! their heights), `+0xb0` its loop sound, `+0xb4` it asks for a camera tweak, `+0xb8` / `+0xbc` the tweak's slot and
//! held.
//!
//! Nothing at all while Ratchet is in his state 0x82. **0**: updated at any distance, the flag 1, `+0x7c` = `+0x78`,
//! `+0x60` = `+0x74`, its top its place, its bottom `+0x68` under it, the sound none, the speed 0; 1.
//! * **1**: its sound stopped. Ratchet on it (`0x32e568` = L00 `0x325398`, crate::moby::classes::level17::floaters::
//!   hero_on): with `+0xb4` a camera tweak of priority 5 (`0x2e1f68` = L00 `0x2d7468`: distance 1, pivot height 4.5),
//!   then 2 at the top, else 3. Else at the top with Ratchet more than half its travel below it: 2; at the bottom with
//!   him more than half of it above and not in his state 0x82: 3.
//! * **2** (down): at the bottom its tweak let go (`0x2e2008` = L00 `0x2d7508`), its sound stopped, Ratchet off it: 1.
//!   Else with nothing in a 2.8 sphere about it (`0x2ea440` = L00 `0x2df8d8`, flags 1, itself left out) — or Ratchet
//!   at or above its height, or on it, or the sphere's last moby (`0x1bfc98`) a trooper 0x0ea3 — its loop sound (sound
//!   0, looped) and its height eased to the bottom (`0x328ff0` = L00 `0x31ed78`: 0.0019 / 0.0017 a tick², at most 0.1);
//!   else its sound stopped (it waits).
//! * **3** (up): its height eased to the top; there its tweak let go, its sound stopped and, Ratchet off it, `+0x60` =
//!   `+0x74`, `+0x78` one less, 1; else its loop sound.
//! * **4**: its sound stopped.
//! * **Each tick** after: its glow `0x7f000000 | c·0x10101`, `c` a triangle wave 0..255 of the frame (`0x1b23c8`) over
//!   two seconds; its move into its platform block (`0x32c5d0` = L00 `0x322668`) and Ratchet carried
//!   (crate::moby::carry_hero_with).

use crate::hero::physics::{ease, sub};
use crate::moby::classes::level17::floaters::hero_on;
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x116d;

const BLOCK: usize = 0x20;
const SPEED: usize = 0x64;
const TRAVEL: usize = 0x68;
const LOW: usize = 0x80;
const TOP: usize = 0x90;
const SOUND: usize = 0xb0;
const WANTS_TWEAK: usize = 0xb4;
const TWEAK: usize = 0xb8;
const TWEAK_HELD: usize = 0xbc;
const TROOPER: i16 = 0x0ea3;

fn stop_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(SOUND);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(SOUND, -1);
}

fn keep_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(SOUND);
    if h < 0 || !ctx.host.sound_playing(h) {
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(SOUND, h);
    }
}

fn ask_tweak(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(WANTS_TWEAK) == 0 { return; }
    let Some(s) = ctx.game.cam_tweaks.request(5) else { return };
    let t = &mut ctx.game.cam_tweaks.slots[s];
    (t.dist, t.pivot_h) = (1.0, 4.5);
    w.mobys[id].set_pv_i(TWEAK, s as i32);
    w.mobys[id].set_pv_i(TWEAK_HELD, 1);
}

fn let_go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(TWEAK_HELD) == 0 { return; }
    ctx.game.cam_tweaks.release(w.mobys[id].pv_i(TWEAK) as usize);
    w.mobys[id].set_pv_i(TWEAK_HELD, 0);
}

fn ease_to(w: &mut MobyWorld, id: usize, z: f32) {
    let m = &mut w.mobys[id];
    let (mut y, mut v) = (m.pos[2], m.pv_f(SPEED));
    ease(z, f32::from_bits(0x3afe_dcbb), f32::from_bits(0x3ada_740e), f32::from_bits(0x3dcc_ccce), &mut y, &mut v);
    m.pos[2] = y;
    m.set_pv_f(SPEED, v);
}

/// The way down is clear (module docs, state 2).
fn clear_below(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    let hit = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, m.pos, f32::from_bits(0x4033_3333), crate::collision::QueryFlags(1), Some(id)));
    let Some(hit) = hit else { return true };
    m.pos[2] <= ctx.hero.pos[2] || ctx.hero.ground_moby == Some(id) || hit.moby.is_some_and(|k| w.mobys.get(k).is_some_and(|t| t.o_class == TROOPER))
}

/// 0x116d (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.hero.state == 0x82 { return; }
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let (low, top, travel) = (w.mobys[id].pv_f(LOW + 8), w.mobys[id].pv_f(TOP + 8), w.mobys[id].pv_f(TRAVEL));
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.set_pv_i(0x7c, m.pv_i(0x78));
            m.set_pv_i(BLOCK + 0x3c, m.pv_i(BLOCK + 0x3c) | 1);
            let p = m.pos;
            m.set_pv_v(TOP, p);
            m.set_pv_v(LOW, [p[0], p[1], p[2] - m.pv_f(TRAVEL)]);
            m.set_pv_i(0x60, m.pv_i(0x74));
            m.state = 1;
            m.set_pv_i(SOUND, -1);
            m.set_pv_i(SPEED, 0);
        }
        1 => {
            stop_sound(w, id, ctx);
            if hero_on(ctx, id) {
                ask_tweak(w, id, ctx);
                let to = if w.mobys[id].pos[2] == top { 2 } else { 3 };
                w.mobys[id].set_state(to, -1);
            }
            let z = w.mobys[id].pos[2];
            let dz = z - ctx.hero.pos[2];
            if travel * 0.5 < dz && z == top {
                w.mobys[id].set_state(2, -1);
            } else if dz < -travel * 0.5 && z == low && ctx.hero.state != 0x82 {
                w.mobys[id].set_state(3, -1);
            }
        }
        2 => {
            if w.mobys[id].pos[2] <= low {
                let_go(w, id, ctx);
                stop_sound(w, id, ctx);
                if !hero_on(ctx, id) { w.mobys[id].state = 1; }
            } else if clear_below(w, id, ctx) {
                keep_sound(w, id, ctx);
                ease_to(w, id, low);
            } else {
                stop_sound(w, id, ctx);
            }
        }
        3 => {
            ease_to(w, id, top);
            if top <= w.mobys[id].pos[2] {
                let_go(w, id, ctx);
                stop_sound(w, id, ctx);
                if !hero_on(ctx, id) {
                    let m = &mut w.mobys[id];
                    m.set_pv_i(0x60, m.pv_i(0x74));
                    m.set_pv_i(0x78, m.pv_i(0x78) - 1);
                    m.state = 1;
                }
            } else {
                keep_sound(w, id, ctx);
            }
        }
        4 => stop_sound(w, id, ctx),
        _ => {}
    }
    let f = ctx.game.frame as f32 * f32::from_bits(0x3c88_8889);
    let tick = f as i32;
    let mut f = f - tick as f32;
    if tick & 1 == 0 { f = 1.0 - f; }
    let c = (f * 255.0) as i32 as u32;
    let m = &mut w.mobys[id];
    m.glow = 0x7f00_0000 | c << 16 | c << 8 | c;
    let (pos, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, BLOCK, sub(pos, old), old_rot, rot);
    let flags = m.pv_i(BLOCK + 0x3c) as u32;
    crate::moby::carry_hero_with(ctx, id, flags, old, old_rot[2], pos, rot[2]);
}
