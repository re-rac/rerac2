//! Dobbo's mission director 0x0d9d (`func_L09_00406FC8`, L09 `0x408458`, one placed): seven states over the level's
//! areas that switch two area switches (0x0102 / 0x0de0, their `+0x95`: crate::moby::classes::area_switch and
//! crate::moby::classes::level09::waker) and the mobys and groups it names on and off (crate::moby::director::
//! set_parts / set_group_parts), with the music of its two stretches.
//!
//! The pvars (words; −1 none): `+0x00` / `+0x04` / `+0x0c` / `+0x10` areas (the first two with the switches `+0x20` /
//! `+0x24` they hold outside while Ratchet is not in them: [`area`]), `+0x2c` a platinum bolt, `+0x30..+0x3c` four
//! groups, `+0x40` a moby whose `+0x68`[0] becomes `+0x48`, `+0x4c` / `+0x50` / `+0x54` / `+0x58` / `+0x5c` / `+0x60` /
//! `+0x64` / `+0x68` mobys, `+0x6c` a cuboid Ratchet is put at, `+0x70` / `+0x78` music tracks with the stinger
//! `+0x74`, `+0x7c` a nanotech pickup. Its update distance is 0xff every tick. Its helpers:
//! * [`area`] (`0x408088`): Ratchet in the area (`0x3270c8` = L00 `0x32a848`); the switch's `+0x95` 0 then, else 2
//!   (0 with no area).
//! * [`force`] (`0x408030`): a switch (class 0x0102 or 0x0de0, not gone) given its `+0x95`.
//! * [`done`] (`0x408150`): not with Ratchet in state 0x31; the platinum bolt `+0x2c` found (`0x19b278 + 4·level +`
//!   its `+0x68`[0]), the nanotech `+0x7c` had (`0x1a7a30 +` its `+0x68`[0]) and the moby `+0x50`'s uid spent this
//!   visit (`0x21e004 + uid`) or counted (`0x19b4a8`); then `+0x64` and `+0x68` deleted and `+0x5c` let go
//!   (`0x408120`: its timer byte 1, the pvar −1). True when so.
//! * [`lift`] (`0x408308`): with `+0x64` alive and its `+0x68`[0x70] a cuboid, the moby `+0x4c` on and solid while the
//!   uid of `+0x50` is spent, else off.
//!
//! The states (`0x2a6620`'s jump table):
//! * **0**: on the level's first frame (`0x1b2308`): `+0x58` deleted with Ratchet in area `+0x0c`; with the flag
//!   `0x1395b8[0x2d]` Ratchet put at the moby `+0x60` (its place and angles, state 0) and the flag cleared; with
//!   `[0x1e]` the moby `+0x40`'s `+0x68`[0] set; `+0x4c` on (and `+0x5c` let go) with `+0x50`'s uid spent, else off;
//!   [`done`]; `[0xa3]` set once. From the sixth frame: the groups and mobys off, then by area: in `+0x00` → 1, in
//!   `+0x04` → 2 (its mobys on), in `+0x10` → 4; and [`lift`].
//! * **1**: its first tick the music `+0x78`; in `+0x04`: its mobys on, the group `+0x3c` on once `+0x58` is gone; 2.
//! * **2**: its first tick the music `+0x70`; the group `+0x3c` on once `+0x58` is gone; in `+0x00` → 1; on the
//!   ground in `+0x0c`: the group `+0x30` on, the flag `[0x1e]` and the moby `+0x40`'s word, [`lift`], 3 (and on into
//!   3 the same tick).
//! * **3**: `+0x5c` on once `+0x54` is past state 2; `+0x68` / `+0x64` on unless [`done`] while scene 0x5f has been
//!   seen; out of `+0x0c` → 2 (switch `+0x24` released); Ratchet in state 0x31 → [`lift`], the group `+0x3c` off, 4.
//! * **4**: in `+0x10`: the group `+0x30` off, [`done`], 5.
//! * **5**: [`lift`]; in `+0x0c`: `+0x5c` let go, the switch `+0x24` released, Ratchet put at the cuboid `+0x6c`
//!   (state 0), [`done`], 6.
//! * **6**: [`done`]; in `+0x00` → 1; out of `+0x0c` → 2; Ratchet in state 0x31 → 4 as from 3.
//!
//! Not ported: [`lift`]'s cuboid height (the cuboid `+0x64`'s `+0x68`[0x70] set to z 325.25 / 325: the level's
//! cuboids are read-only here), the ship byte `0x152c26` = 1 before Ratchet is put (`0x365810`). The autosave
//! `0x3098d0(0, −1)` goes with `[0xa3]` (crate::game::GameState::autosave).

use crate::moby::director::{gone, link, music_to, place_hero, place_hero_at, set_group_parts, set_parts};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0d9d;

/// The game's flag bytes (`0x1395b8 +`) it reads and sets.
const PUT: usize = 0x2d;
const SET_40: usize = 0x1e;
const ONCE: usize = 0xa3;

fn pv(w: &MobyWorld, id: usize, o: usize) -> i32 { w.mobys[id].pv_i(o) }

/// A moby's state (`+0x20`; 0 with none there).
fn state_of(w: &MobyWorld, i: i32) -> u8 { link(w, i).map_or(0, |i| w.mobys[i].state) }

/// `0x408030(i, v)`.
pub fn force(w: &mut MobyWorld, i: i32, v: u8) {
    let Some(i) = link(w, i).filter(|&i| !gone(w, i)) else { return };
    if matches!(w.mobys[i].o_class, 0x0102 | 0x0de0) { w.mobys[i].sub_state = v; }
}

/// `0x408088(a, sw)`.
pub fn area(w: &mut MobyWorld, ctx: &Ctx, a: i32, sw: i32) -> bool {
    let inside = a != -1 && ctx.level.in_area(a, ctx.hero.pos);
    force(w, sw, if a != -1 && !inside { 2 } else { 0 });
    inside
}

/// The uid of moby `i` spent this visit or counted in the save.
fn spent(w: &MobyWorld, ctx: &Ctx, i: i32) -> bool {
    let Some(i) = link(w, i) else { return false };
    let uid = w.mobys[i].uid;
    ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(uid) != 0
}

/// `0x408120`: the moby `+0x5c` let go (its timer byte 1).
fn let_go(w: &mut MobyWorld, id: usize) {
    let i = pv(w, id, 0x5c);
    if i == -1 { return; }
    if let Some(i) = link(w, i) { w.mobys[i].timer = 1; }
    w.mobys[id].set_pv_i(0x5c, -1);
}

/// `0x408150` (module docs).
pub fn done(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    if ctx.hero.state == 0x31 { return false; }
    let g = &*ctx.game;
    if let Some(b) = link(w, pv(w, id, 0x2c)).filter(|&b| !gone(w, b)) {
        let k = (4 * g.level.max(0)).wrapping_add(w.mobys[b].pv_i(0));
        if usize::try_from(k).ok().and_then(|k| g.platinum.get(k)).is_none_or(|&f| f == 0) { return false; }
    }
    if let Some(n) = link(w, pv(w, id, 0x7c)).filter(|&n| !gone(w, n)) {
        let k = w.mobys[n].pv_i(0);
        if usize::try_from(k).ok().and_then(|k| g.nanotech.get(k)).is_none_or(|&f| !f) { return false; }
    }
    let u = pv(w, id, 0x50);
    if u != -1 && !spent(w, ctx, u) { return false; }
    for o in [0x64, 0x68] {
        let i = pv(w, id, o);
        if i == -1 { continue; }
        if let Some(i) = link(w, i) { w.delete(i); }
        w.mobys[id].set_pv_i(o, -1);
    }
    let_go(w, id);
    true
}

/// `0x408308` (module docs).
pub fn lift(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let Some(i) = link(w, pv(w, id, 0x64)).filter(|&i| !gone(w, i)) else { return };
    if w.mobys[i].pv_i(0x70) == -1 { return; }
    let on = pv(w, id, 0x50) != -1 && spent(w, ctx, pv(w, id, 0x50));
    let f = on as i32;
    set_parts(w, pv(w, id, 0x4c), f, f, f);
}

/// The moby `+0x40` given the word `+0x48`.
fn set_40(w: &mut MobyWorld, id: usize) {
    let (a, v) = (pv(w, id, 0x40), pv(w, id, 0x48));
    if a == -1 || v == -1 { return; }
    if let Some(a) = link(w, a) { w.mobys[a].set_pv_i(0, v); }
}

/// The four mobys `+0x50..+0x5c` switched (`+0x5c` keeps its collision).
fn four(w: &mut MobyWorld, id: usize, on: i32) {
    set_parts(w, pv(w, id, 0x50), on, on, on);
    set_parts(w, pv(w, id, 0x54), on, on, -1);
    set_parts(w, pv(w, id, 0x58), on, on, on);
}

/// `+0x68` / `+0x64` on unless [`done`], with scene 0x5f seen (`0x1397c4` < 0).
fn extras(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    if !done(w, id, ctx) && ctx.game.scenes_seen.get(0x5f).copied().unwrap_or(false) {
        set_parts(w, pv(w, id, 0x68), 1, 1, 1);
        set_parts(w, pv(w, id, 0x64), 1, -1, -1);
    }
}

/// Into state 1 from areas `+0x00` (switch `+0x24` held).
fn to_1(w: &mut MobyWorld, id: usize, with_4c: bool) {
    force(w, pv(w, id, 0x24), 2);
    four(w, id, 0);
    set_parts(w, pv(w, id, 0x5c), 0, 0, -1);
    if with_4c { set_parts(w, pv(w, id, 0x4c), 0, 0, 0); }
    w.mobys[id].set_state(1, -1);
}

/// The group `+0x3c` on once `+0x58` is gone (254 / 253), and `+0x58` forgotten.
fn release_58(w: &mut MobyWorld, id: usize, also_none: bool) {
    let i = pv(w, id, 0x58);
    let s = state_of(w, i);
    if (also_none && i == -1) || (i != -1 && (s == 0xfe || s == 0xfd)) {
        set_group_parts(w, pv(w, id, 0x3c), 1, 1, 1);
        w.mobys[id].set_pv_i(0x58, -1);
    }
}

/// Ratchet in state 0x31 (from 3 and 6): [`lift`], the switch `+0x24` held, the group `+0x3c` off, 4.
fn to_4(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    lift(w, id, ctx);
    force(w, pv(w, id, 0x24), 2);
    set_group_parts(w, pv(w, id, 0x3c), 0, 0, 0);
    w.mobys[id].set_state(4, -1);
}

/// State 3's body (also run from 2 the tick it moves to 3).
fn state_3(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let i = pv(w, id, 0x54);
    if i != -1 && 3 <= state_of(w, i) { set_parts(w, pv(w, id, 0x5c), 1, 1, -1); }
    extras(w, id, ctx);
    if !area(w, ctx, pv(w, id, 0x0c), -1) {
        force(w, pv(w, id, 0x24), 0);
        w.mobys[id].set_state(2, -1);
    } else if ctx.hero.state == 0x31 {
        to_4(w, id, ctx);
    }
}

/// `0x408458` (module docs).
pub fn update_0d9d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].update_dist = 0xff;
    match w.mobys[id].state {
        0 => {
            if w.frame == 0 {
                let i = pv(w, id, 0x58);
                if i != -1 {
                    let s = state_of(w, i);
                    if s != 0xfe && s != 0xfd && area(w, ctx, pv(w, id, 0x0c), -1) {
                        if let Some(i) = link(w, i) { w.delete(i); }
                        w.mobys[id].set_pv_i(0x58, -1);
                    }
                }
                let p = pv(w, id, 0x60);
                if ctx.game.flags.get(PUT).is_some_and(|&f| f != 0) && p != -1 {
                    if let Some(p) = link(w, p) {
                        let (pos, rot) = (w.mobys[p].pos, w.mobys[p].rot);
                        place_hero(ctx, pos, rot, 0);
                    }
                    if let Some(f) = ctx.game.flags.get_mut(PUT) { *f = 0; }
                }
                if ctx.game.flags.get(SET_40).is_some_and(|&f| f != 0) { set_40(w, id); }
                let u = pv(w, id, 0x50);
                if u != -1 && spent(w, ctx, u) {
                    set_parts(w, pv(w, id, 0x4c), 1, 1, 1);
                    let_go(w, id);
                } else {
                    set_parts(w, pv(w, id, 0x4c), 0, 0, 0);
                }
                done(w, id, ctx);
                if let Some(f) = ctx.game.flags.get_mut(ONCE).filter(|f| **f == 0) {
                    *f = 1;
                    // `0x3098d0(0, −1)`: the autosave (crate::game::GameState::autosave).
                    ctx.game.autosave(-1);
                }
            }
            if w.frame < 6 { return; }
            set_group_parts(w, pv(w, id, 0x30), 0, 0, -1);
            set_group_parts(w, pv(w, id, 0x34), 0, 0, -1);
            set_group_parts(w, pv(w, id, 0x38), 0, -1, -1);
            set_group_parts(w, pv(w, id, 0x3c), 0, 0, 0);
            four(w, id, 0);
            set_parts(w, pv(w, id, 0x5c), 0, 0, -1);
            set_parts(w, pv(w, id, 0x68), 0, 0, 0);
            set_parts(w, pv(w, id, 0x64), 0, 0, -1);
            if area(w, ctx, pv(w, id, 0x00), pv(w, id, 0x20)) {
                to_1(w, id, true);
            } else if area(w, ctx, pv(w, id, 0x04), pv(w, id, 0x24)) {
                force(w, pv(w, id, 0x20), 2);
                four(w, id, 1);
                let i = pv(w, id, 0x54);
                let on = (i != -1 && 3 <= state_of(w, i)) as i32;
                set_parts(w, pv(w, id, 0x5c), on, on, -1);
                extras(w, id, ctx);
                w.mobys[id].set_state(2, -1);
            } else if ctx.level.in_area(pv(w, id, 0x10), ctx.hero.pos) {
                force(w, pv(w, id, 0x20), 2);
                four(w, id, 1);
                set_parts(w, pv(w, id, 0x5c), 1, 1, -1);
                extras(w, id, ctx);
                w.mobys[id].set_state(4, -1);
            }
            lift(w, id, ctx);
        }
        1 => {
            if w.mobys[id].first() { music_to(ctx, pv(w, id, 0x78), pv(w, id, 0x74)); }
            if !area(w, ctx, pv(w, id, 0x04), pv(w, id, 0x24)) { return; }
            force(w, pv(w, id, 0x20), 2);
            four(w, id, 1);
            set_parts(w, pv(w, id, 0x5c), 0, 0, -1);
            release_58(w, id, true);
            w.mobys[id].set_state(2, -1);
        }
        2 => {
            if w.mobys[id].first() { music_to(ctx, pv(w, id, 0x70), pv(w, id, 0x74)); }
            release_58(w, id, false);
            if area(w, ctx, pv(w, id, 0x00), pv(w, id, 0x20)) {
                to_1(w, id, true);
            } else if ctx.hero.grounded_ticks != 0 && area(w, ctx, pv(w, id, 0x0c), -1) {
                set_group_parts(w, pv(w, id, 0x30), 1, 1, -1);
                if let Some(f) = ctx.game.flags.get_mut(SET_40) { *f = 1; }
                set_40(w, id);
                lift(w, id, ctx);
                w.mobys[id].set_state(3, -1);
                state_3(w, id, ctx);
            }
        }
        3 => state_3(w, id, ctx),
        4 => {
            if area(w, ctx, pv(w, id, 0x10), -1) {
                set_group_parts(w, pv(w, id, 0x30), 0, 0, -1);
                done(w, id, ctx);
                w.mobys[id].set_state(5, -1);
            }
        }
        5 => {
            lift(w, id, ctx);
            if !area(w, ctx, pv(w, id, 0x0c), -1) { return; }
            let_go(w, id);
            force(w, pv(w, id, 0x24), 0);
            let c = pv(w, id, 0x6c);
            if c != -1 { place_hero_at(ctx, c, 0); }
            done(w, id, ctx);
            w.mobys[id].set_state(6, -1);
        }
        6 => {
            done(w, id, ctx);
            if area(w, ctx, pv(w, id, 0x00), pv(w, id, 0x20)) {
                four(w, id, 0);
                set_parts(w, pv(w, id, 0x5c), 0, 0, -1);
                force(w, pv(w, id, 0x24), 2);
                w.mobys[id].set_state(1, -1);
            } else if !area(w, ctx, pv(w, id, 0x0c), -1) {
                force(w, pv(w, id, 0x24), 0);
                w.mobys[id].set_state(2, -1);
            } else if ctx.hero.state == 0x31 {
                to_4(w, id, ctx);
            }
        }
        _ => {}
    }
}
