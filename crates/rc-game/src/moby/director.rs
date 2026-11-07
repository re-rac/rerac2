//! The pieces the level directors share: the objects that run a level's flow (its missions, scenes, checkpoints,
//! chunk switches, music and help). The game has no director routine of its own; each level's director carries the
//! same steps inline, and the port keeps them here once.
//!
//! * **A mission's test** ([`mission_is_done`]): bit 7 of `0x19b2e8 + level·0x10 + mission`.
//! * **A step done** ([`finish`]): its mission done (`0x3132a8`) and the checkpoint at a place (`0x359b20(pos, rot)`).
//! * **Ratchet put** ([`place_hero`], `0x2b9e68(pos, rot, state, 1)`): placed, and with a state other than −1 sent to
//!   it (`0x2c4560(state, 2)`).
//! * **The music** ([`music_to`]): the track asked for (`0x335928(track, stinger)`) unless it is the one playing
//!   (`0x1a63f0`, a half-word against the whole word).
//! * **A moby shown and solid or hidden and passed through** ([`solid`]): mode bits 0x41 off and 0x1000 on with its
//!   class's collision (`+0x98` = class `+0x10`), or the other way with none.
//! * **A moby's or a group's update, draw and collision switched** ([`set_parts`], [`set_group_parts`]; L08, L09,
//!   L19): each flag on, off or left (−1).
//! * **A group's members** ([`members`]): the list `0x1fa8a0[group]`, nothing for a group whose low byte is 0xff or
//!   past the level's count.
//! * **The census** ([`Census`], L09 `0x3e46f8`, L19 `0x42bbf8`): 40 mobys a frame of the whole table (`0x1b1e1c` ..
//!   its end), counting the ones the caller names; a full pass that counted none is the level's skill point
//!   (crate::skill_points: every site's four steps, [`crate::game::GameState::award_skill_point`]).
//! * **The camera tweak a director holds** ([`hold_tweak`], [`let_tweak_go`]): one of priority 5 (`0x2d7468(5)`)
//!   kept in a pvar while Ratchet is in a region, its fields set each tick, let go (`0x2d7508`) when he leaves. The
//!   pvar keeps the slot + 1 (0 none) where the game keeps the record's address.

use super::{Ctx, Moby, MobyWorld};
use crate::hero::physics::V3;

/// The level's mission `mission` done (`0x19b2e8 + level·0x10`, bit 7).
pub fn mission_is_done(g: &crate::game::GameState, mission: i32) -> bool {
    let i = (g.level.max(0) as usize * 0x10).wrapping_add(mission as usize);
    g.missions.get(i).is_some_and(|&b| b & 0x80 != 0)
}

/// `0x3132a8(mission)` then `0x359b20(pos, rot)`.
pub fn finish(ctx: &mut Ctx, mission: i32, pos: V3, rot: V3) {
    ctx.game.mission_done(mission as u8);
    ctx.game.save_checkpoint(pos, rot);
}

/// `0x2b9e68(pos, rot, state, 1)`: Ratchet placed, then `state` asked for unless it is −1.
pub fn place_hero(ctx: &mut Ctx, pos: V3, rot: V3, state: i32) {
    ctx.hero.teleport(pos, rot);
    if state != -1 { ctx.hero.request_state(state, true); }
}

/// [`place_hero`] at cuboid `c`'s place and angles (`+0x30` / `+0x70`).
pub fn place_hero_at(ctx: &mut Ctx, c: i32, state: i32) {
    if let Some((p, r)) = ctx.level.cuboid_place(c) { place_hero(ctx, p, r, state); }
}

/// `if 0x1a63f0 != track { 0x335928(track, stinger) }` (the low byte the port keeps against the whole word, as
/// crate::moby::classes::arena). Without the audio the track is always asked for.
pub fn music_to(ctx: &mut Ctx, track: i32, stinger: i32) {
    if ctx.host.music_tracks().is_some_and(|(cur, _)| cur as i32 == track) { return; }
    ctx.host.music_request(track, stinger);
}

/// Moby `id` shown and solid (`on`: mode &= ~0x41, |= 0x1000, `+0x98` = its class's `+0x10`) or hidden and passed
/// through (mode |= 0x41, &= ~0x1000, `+0x98` = 0).
pub fn solid(w: &mut MobyWorld, id: usize, on: bool) {
    let Some(class) = w.mobys.get(id).map(|m| m.o_class) else { return };
    let coll = on && w.class(class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.mode = if on { (m.mode & !0x41) | 0x1000 } else { (m.mode | 0x41) & !0x1000 };
    m.has_collision = coll;
}

/// The members of group `g` (the list `0x1fa8a0[g]`); none for a low byte of 0xff or a group past the level's.
pub fn members(w: &MobyWorld, g: i32) -> Vec<usize> {
    if g & 0xff == 0xff { return Vec::new(); }
    usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// Deleted (state 0xfe or 0xfd), or no moby there.
pub fn gone(w: &MobyWorld, i: usize) -> bool { w.mobys.get(i).is_none_or(|m| m.state == 0xfe || m.state == 0xfd) }

/// The moby index a pvar names (−1 or past the table: none).
pub fn link(w: &MobyWorld, v: i32) -> Option<usize> { usize::try_from(v).ok().filter(|&i| i < w.mobys.len()) }

/// `0x329cf8(i, update, draw, solid)` → `0x329c58` (the decomp's `func_L08_00328B80` / `func_L08_00328AE0`: L09
/// `0x318ce0` / `0x318c40`, L19 `0x31a4a0` / `0x31a400`): moby `i` (−1 or gone: nothing) with each flag that is not
/// −1 applied: its update on (mode &= ~2) or off (|= 2), drawn (`+0x31` = 1, mode &= ~1) or not (0, mode |= 1),
/// solid (`+0x98` = its class's `+0x10`) or not.
pub fn set_parts(w: &mut MobyWorld, i: i32, update: i32, draw: i32, solid: i32) {
    let Some(i) = link(w, i).filter(|&i| !gone(w, i)) else { return };
    let coll = w.class(w.mobys[i].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[i];
    if update != -1 { if update != 0 { m.mode &= !2 } else { m.mode |= 2 } }
    if draw != -1 {
        if draw != 0 { m.drawn = 1; m.mode &= !1 } else { m.drawn = 0; m.mode |= 1 }
    }
    if solid != -1 { m.has_collision = solid != 0 && coll; }
    w.changed = true;
}

/// `0x329d28(group, update, draw, solid)` (the decomp's `func_L08_00328BB0`: L09 `0x318d10`, L19 `0x31a4d0`):
/// [`set_parts`] on every member of the group.
pub fn set_group_parts(w: &mut MobyWorld, g: i32, update: i32, draw: i32, solid: i32) {
    for i in members(w, g) { set_parts(w, i as i32, update, draw, solid); }
}

/// What the census makes of one moby.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tally {
    /// Not one of the kind it counts.
    Pass,
    /// One of them.
    Count,
    /// Stepped over without the table's end test (L09's busy and state-5 troopers: the end comes a step later).
    Skip,
}

/// The census's cursor and its count so far (L09 its pvars `+0x58` / `+0x5c`, L19 `$gp+0x2f6c` / `+0x2f70`).
#[derive(Clone, Copy, Debug, Default)]
pub struct Census {
    pub cursor: usize,
    pub count: i32,
}

impl Census {
    /// 40 steps over the table (module docs): `tally` for each moby there. True when a pass ended with none counted;
    /// the cursor and count are then reset with `reset_on_empty` (L09), else left where they are (L19). A pass that
    /// counted some is reset and goes on.
    pub fn step(&mut self, w: &MobyWorld, reset_on_empty: bool, tally: impl Fn(&Moby) -> Tally) -> bool {
        for _ in 0..40 {
            let t = w.mobys.get(self.cursor).map_or(Tally::Pass, &tally);
            self.cursor += 1;
            match t {
                Tally::Skip => continue,
                Tally::Count => self.count += 1,
                Tally::Pass => {}
            }
            if self.cursor < w.mobys.len() { continue; }
            let empty = self.count == 0;
            if empty && !reset_on_empty { return true; }
            self.cursor = 0;
            self.count = 0;
            if empty { return true; }
        }
        false
    }
}

/// A camera tweak of priority 5 held in pvar `pv` (slot + 1) with `fields` set on it: the distance, the pivot's
/// height and (L18) the look height; `None` leaves a field alone.
pub fn hold_tweak(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, pv: usize, fields: [Option<f32>; 3]) {
    let mut s = w.mobys[id].pv_i(pv);
    if s == 0 {
        // `0x2d7468(5)`: none free leaves the pvar at 0 (the game then writes through a null record).
        let Some(i) = ctx.game.cam_tweaks.request(5) else { return };
        s = i as i32 + 1;
        w.mobys[id].set_pv_i(pv, s);
    }
    let t = &mut ctx.game.cam_tweaks.slots[s as usize - 1];
    if let Some(v) = fields[0] { t.dist = v; }
    if let Some(v) = fields[1] { t.pivot_h = v; }
    if let Some(v) = fields[2] { t.look_h = v; }
}

/// The tweak in pvar `pv` let go (`0x2d7508`) and the pvar cleared.
pub fn let_tweak_go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, pv: usize) {
    let s = w.mobys[id].pv_i(pv);
    if 0 < s { ctx.game.cam_tweaks.release(s as usize - 1); }
    w.mobys[id].set_pv_i(pv, 0);
}
