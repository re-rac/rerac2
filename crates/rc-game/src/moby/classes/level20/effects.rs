//! Yeedil's scene effects and its rising group (L20 addresses; the engine functions by their L20 places with the L00
//! ones the port knows them by).
//!
//! * **0x0f2a** ([`update_0f2a`], `0x39be60`, one placed; the decomp's `func_L20_0039AA98`): the effects of the level's
//!   scene 7 (the scene player `0x1bdc40`: `+0x30` the record, `+0x34` the tick, `+0x18c` / `+0x194` the actors 0 and
//!   2). **0**: updated from any distance; 1 (a byte). **1**: in a scene (`0x1a8f00` = 2) of the record 7: at its tick
//!   157 a blast at the actor 2 (`0x41b550` = L00 `0x3f8348`, crate::moby::blast::burst: size 1.5, the shake, 40 /
//!   20 / 30 sprites, 40 puffs, 50 streaks, the light); at its ticks 171..319 a spark (`0x34b690` = L00 `0x336148`,
//!   crate::particles::lava::spark) one tick in two (`0x3308f0(2)`) at the actor 0's joint 0 (`0x327768` = L00
//!   `0x3127c8`) 0.03 up, flying 0.0283..0.05 a tick (`0x330988` = L00 `0x31ba38`) at a random heading (`0x330a50` =
//!   L00 `0x31bb00`) and a pitch of 30°..150° (`0x339838` = L00 `0x325100`), its size 1470..7350 (the random draws in
//!   that order, the coin's before the size).
//! * **0x0548** ([`update_0548`], `0x3c1450`, one placed; the decomp's `func_US2_L20_003C1450`): a group hidden 20
//!   under its place that rises once a moby reaches a state, with a fly-by camera. Its pvars: `+0x00` the group, `+0x04`
//!   the rise's countdown, `+0x08` its inverse length, `+0x0c` the moby watched, `+0x10` the fly-by camera, the bytes
//!   `+0x14` the state watched for, `+0x15` hooked sets it off too, `+0x16` the hero's form switched back at the end,
//!   `+0x17` its mission done and a checkpoint. The states are written as bytes.
//!   * **0**: without a moby, a group or a camera it is deleted (`0x3265e8` = L00 `0x311688`). Its uid gone
//!     (`0x21f3c4`, crate::game::GameState::uid_dead) or counted (`0x19b4a8`): 3. Else 1 and the group sunk
//!     ([`sink`], `0x3c16d8`: each member's height kept, mode | 0x41, 20 down).
//!   * **1**: the moby in the state watched, or with `+0x15` hooked (`0x33fb18` = L00 `0x32a838`): its uid counted;
//!     with `+0x17` its mission done (`0x328248` = L00 `0x3132a8`); 2, the countdown 120, its inverse 1/120
//!     (`0x304118` = L00 `0x2effa8`); the camera armed (`0x4619f0` = L00 `0x42a4e0`, crate::cameras::Cameras::arm).
//!   * **2**: the countdown (`0x302c30` = L00 `0x2eeab0`, crate::moby::countdown); the group risen by it ([`rise`],
//!     `0x3c1758`: shown, at its kept height less 20 plus 20 times the cubic `0x330bb8(−1, 0, 1, 0, 1 − t/120)` = L00
//!     `0x31bc68`, crate::moby::classes::nanotech::cubic). At its end, Ratchet not in his state 0x6f: with `+0x17` the
//!     checkpoint at his place and angles (`0x36fbf0` = L00 `0x359b20`); 3.
//!   * The kept heights: the original's buffer `0x23e670` (one for the level); the port's in its own pvars from
//!     `+0x100`.
//!
//! Not ported: 0x0548's switch back of the hero's form (`0x2c5b50(0)` with `+0x16`; level 20's own, as
//! crate::moby::classes::level20::misc's 0x1209).

use crate::moby::{countdown, Ctx, MobyWorld};

pub const SCENE_FX: i16 = 0x0f2a;

/// 0x0f2a (module docs).
pub fn update_0f2a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.state = 1;
        }
        1 => {
            if ctx.game.scenes.mode != crate::scene::SCENE { return; }
            let Some(p) = ctx.game.scenes.player.as_ref() else { return };
            if rc_formats::disc::scene::record_of(p.id) != 7 { return; }
            let n = w.mobys.len();
            let actor = |k: usize| p.actors.get(k).map(|a| a.0).filter(|&a| a < n);
            let (tick, first, second) = (p.tick, actor(0), actor(2));
            if tick == 157 {
                if let Some(s) = second {
                    let at = w.mobys[s].pos;
                    let b = crate::moby::blast::Blast { size: 1.5, bursts: [40, 20, 30], puffs: 40, streaks: 50, light: true, shake: true };
                    crate::moby::blast::burst(w, ctx, at, &b);
                }
            }
            if !(171..320).contains(&tick) { return; }
            let Some(f) = first else { return };
            let Some(mut at) = ctx.host.joint(&w.mobys[f], 0) else { return };
            at[2] += f32::from_bits(0x3cf5_c28f);
            let g = &mut ctx.game.rng;
            let speed = g.randf(f32::from_bits(0x3ce8_1b50), f32::from_bits(0x3d4c_ccce));
            let heading = g.rand_angle();
            let tilt = g.randf(f32::from_bits(0x3f06_0a93), f32::from_bits(0x4027_8d37));
            // `0x339838(out, r, yaw, pitch)`.
            let vel = [heading.cos() * speed * tilt.cos(), heading.sin() * speed * tilt.cos(), tilt.sin() * speed];
            if g.randi(2) != 0 {
                let size = g.randf(f32::from_bits(0x44b7_c000), f32::from_bits(0x45e5_b000));
                if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::lava::spark(sys, g, size, at, vel, false); }
            }
        }
        _ => {}
    }
}

pub const RISING: i16 = 0x0548;

const GROUP: usize = 0x00;
const TIMER: usize = 0x04;
const INV: usize = 0x08;
const WATCHED: usize = 0x0c;
const CAMERA: usize = 0x10;
/// The port's kept heights (module docs).
const KEPT: usize = 0x100;
const RISE_TICKS: i32 = 120;

fn members(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].pv_i(GROUP)).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// `0x3c16d8`: the group sunk 20, its heights kept.
fn sink(w: &mut MobyWorld, id: usize) {
    for (k, e) in members(w, id).into_iter().enumerate() {
        let Some(z) = w.mobys.get(e).map(|o| o.pos[2]) else { continue };
        w.mobys[id].set_pv_f(KEPT + 4 * k, z);
        let o = &mut w.mobys[e];
        o.mode |= 0x41;
        o.pos[2] = z - 20.0;
    }
}

/// `0x3c1758`: the group shown and risen by the countdown.
fn rise(w: &mut MobyWorld, id: usize) {
    let v = w.mobys[id].pv_i(TIMER) as f32 * w.mobys[id].pv_f(INV);
    let s = crate::moby::classes::nanotech::cubic(-1.0, 0.0, 1.0, 0.0, 1.0 - v);
    for (k, e) in members(w, id).into_iter().enumerate() {
        let z = w.mobys[id].pv_f(KEPT + 4 * k);
        let Some(o) = w.mobys.get_mut(e) else { continue };
        let d = z - 20.0;
        o.mode &= 0xffbe;
        o.pos[2] = d + (z - d) * s;
    }
}

fn uid_done(ctx: &Ctx, uid: i16) -> bool {
    ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(uid) != 0
}

/// 0x0548 (module docs).
pub fn update_0548(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            if m.pv_i(WATCHED) < 0 || m.pv_i(GROUP) < 0 || m.pv_i(CAMERA) < 0 {
                w.delete(id);
                return;
            }
            if uid_done(ctx, m.uid) {
                w.mobys[id].state = 3;
                return;
            }
            w.mobys[id].state = 1;
            sink(w, id);
        }
        1 => {
            let m = &w.mobys[id];
            let watched = usize::try_from(m.pv_i(WATCHED)).ok().and_then(|k| w.mobys.get(k)).map(|o| o.state);
            let set_off = watched == Some(m.pv_b(0x14)) || (m.pv_b(0x15) != 0 && m.once & 4 != 0);
            if !set_off { return; }
            let (uid, mission, done, cam) = (m.uid, m.mission, m.pv_b(0x17) != 0, m.pv_i(CAMERA));
            ctx.game.count_uid(uid);
            if done { ctx.game.mission_done(mission); }
            let m = &mut w.mobys[id];
            m.state = 2;
            m.set_pv_i(TIMER, RISE_TICKS);
            m.set_pv_f(INV, 1.0 / RISE_TICKS as f32);
            ctx.game.cameras.arm(cam);
        }
        2 => {
            let mut t = w.mobys[id].pv_i(TIMER);
            countdown(&mut t);
            w.mobys[id].set_pv_i(TIMER, t);
            rise(w, id);
            if t != 0 || ctx.hero.state == 0x6f { return; }
            if w.mobys[id].pv_b(0x17) != 0 {
                let (p, r) = (ctx.hero.pos, ctx.hero.rot);
                ctx.game.save_checkpoint(p, r);
            }
            w.mobys[id].state = 3;
        }
        _ => {}
    }
}
