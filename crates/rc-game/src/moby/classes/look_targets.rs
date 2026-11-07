//! The camera's look target on Tabora's hovership levels: the records `0x1b3490` (three of 0x30 bytes: the target,
//! the tick's best offer, the last tick's best), the offers into them ([`offer`], L08 `0x3d4d80`, L19 `0x3c1598`; the
//! decomp's `func_L08_003D3928`) and their director **0x0970** ([`update_0970`], L08 `0x3d4e48`, L19 `0x3c1660`,
//! museum `0x3c09d0`; the decomp's `func_L08_003D39F0`), which turns them into a camera tweak (crate::camera::
//! CamTweaks, priority 5, `0x2e31f8` = L00 `0x2d7468`).
//!
//! **A record**: `+0x00` the place, `+0x10` the moby, `+0x14` its distance from Ratchet (`0x189ea0`), `+0x18` the
//! ticks the target has been challenged (u16; 0xffff none), `+0x1c` the ticks the offer has been the best in a row,
//! `+0x20` offered this tick.
//!
//! **The offer** (`0x3d4d80(moby, place)`): nearer to Ratchet than the tick's best so far, it is the tick's best
//! (its challenge and streak 0, offered); the target's own moby keeps the target's place and distance up to date
//! (offered).
//!
//! **0x0970** (its pvar `[0]` a cuboid). **0**: the three records emptied (distance 100000, challenge 0xffff), no
//! tweak, 1. **1**, every tick:
//! * Ratchet riding (his state 0x31 with `0x18b650` set, crate::hero::vehicle): the target's place 10 ahead of the
//!   vehicle along its yaw and half its pitch reversed (`$gp+0x12c0`, `0x333c00` = L00 `0x2af000`); a tweak asked for
//!   (none to be had: no more this tick); for the vehicle 0x0905 the distance 7 (`$gp+0x12c4`), the pitch and look
//!   height left alone, for the others (the hovership 0x0eef) the distance 6, the pitch 0.1 and the look height 0
//!   (`$gp+0x12c8..+0x12d4`); the face-after-rest flag on, the way it faces the target less the camera's place;
//! * else: the last tick's best still the best grows its streak; a best that is another moby than the target
//!   challenges it — the challenge counted, and past the best's streak (at least 70, `$gp+0x12bc`) the best is the
//!   target; else a target not offered this tick is the tick's best (none: no target). With a target: the tweak (as
//!   above) at distance 7, the face-after-rest flag, the way to the target; without, the tweak let go (`0x2e3298` =
//!   L00 `0x2d7508`);
//! * the records moved on: the tick's best is the last tick's, the tick's best emptied, the target not offered.
//!
//! Its callers here: Tabora's arcer 0x1002 (crate::moby::classes::level08::arcer, its offer in its states 2..5 while
//! drawn).
//!
//! Not ported: the cuboid `[0]` (with the camera inside it, `0x1b258c` = 2: the first-person camera's veto, as
//! crate::moby::classes::level19::ride_cam's), the offers of the hovership's raritanium marker (`0x436d20`, not
//! ported).

use crate::hero::physics::{add, dist, sub, V3};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0970;
/// The vehicle with its own camera distance.
const VEHICLE_0905: i16 = 0x0905;
/// `$gp+0x12bc..+0x12d4`: the least streak a challenger needs; the look's reach; 0x0905's distance; the others'
/// distance, pitch and look height; the target's distance.
const LEAST_STREAK: u32 = 70;
const AHEAD: f32 = 10.0;
const DIST_0905: f32 = 7.0;
const DIST: f32 = 6.0;
const PITCH: f32 = f32::from_bits(0x3dcc_cccd);
const LOOK_H: f32 = 0.0;
const TARGET_DIST: f32 = 7.0;

/// One record (module docs).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Record {
    pub pos: V3,
    pub owner: Option<usize>,
    pub dist: f32,
    pub challenged: u32,
    pub streak: u32,
    pub offered: bool,
}

impl Default for Record {
    fn default() -> Self { Record { pos: [0.0; 3], owner: None, dist: 100_000.0, challenged: 0xffff, streak: 0, offered: false } }
}

/// The records `0x1b3490` and the tweak's slot (`$gp+0x12d8`).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Targets {
    pub target: Record,
    pub best: Record,
    pub last: Record,
    pub tweak: Option<usize>,
}

/// `0x3d4d80(moby, place)`: see the module.
pub fn offer(w: &mut MobyWorld, hero: V3, owner: usize, pos: V3) {
    let t = &mut w.look_targets;
    let d = dist(hero, pos);
    if d < t.best.dist {
        t.best = Record { pos, owner: Some(owner), dist: d, challenged: 0, streak: 0, offered: true };
    }
    if t.target.owner == Some(owner) {
        t.target.dist = d;
        t.target.pos = pos;
        t.target.offered = true;
    }
}

/// 0x0970 (`0x3d4e48`): see the module.
pub fn update_0970(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            w.look_targets = Targets::default();
            w.mobys[id].set_state(1, -1);
            return;
        }
        1 => {}
        _ => return,
    }
    let riding = ctx.hero.riding().and_then(|v| w.mobys.get(v)).map(|v| (v.pos, v.rot, v.o_class));
    let cam = ctx.host.camera_pos();
    if let Some((pos, rot, class)) = riding {
        let at = add(crate::moby::classes::shot::polar(AHEAD, rot[2], -rot[1] * 0.5), pos);
        w.look_targets.target.pos = at;
        if let Some(slot) = tweak(w, ctx) {
            let s = &mut ctx.game.cam_tweaks.slots[slot];
            if class == VEHICLE_0905 {
                (s.dist, s.look_h, s.pitch) = (DIST_0905, -1.0, -1.0);
            } else {
                (s.dist, s.look_h, s.pitch) = (DIST, LOOK_H, PITCH);
            }
            s.face_rest = true;
            s.dir = sub(at, cam);
        }
    } else {
        let t = &mut w.look_targets;
        if t.last.owner == t.best.owner { t.best.streak = t.last.streak + 1; }
        if t.best.owner != t.target.owner && t.best.owner.is_some() {
            let least = t.best.streak.max(LEAST_STREAK);
            t.target.challenged = t.target.challenged.wrapping_add(1) & 0xffff;
            if least <= t.target.challenged {
                t.target = t.best;
                t.target.offered = true;
            }
        } else if !t.target.offered {
            t.target = t.best;
            t.target.offered = true;
        }
        if t.target.owner.is_some() {
            let at = t.target.pos;
            if let Some(slot) = tweak(w, ctx) {
                let s = &mut ctx.game.cam_tweaks.slots[slot];
                s.dist = TARGET_DIST;
                s.face_rest = true;
                s.dir = sub(at, cam);
            }
        } else if let Some(slot) = w.look_targets.tweak.take() {
            ctx.game.cam_tweaks.release(slot);
        }
    }
    let t = &mut w.look_targets;
    t.last = t.best;
    t.best = Record::default();
    t.target.offered = false;
}

/// The tweak's slot, asked for once (priority 5).
fn tweak(w: &mut MobyWorld, ctx: &mut Ctx) -> Option<usize> {
    if w.look_targets.tweak.is_none() { w.look_targets.tweak = ctx.game.cam_tweaks.request(5); }
    w.look_targets.tweak
}
