//! The joint aims: L00 `0x324208` (L09 `0x3206a0`, L16 `0x31d6e8`; the decomp's boot `func_002AEC48`) and the head
//! aim L00 `0x327308` (L14 `0x327308`, L17 `0x329158`, L19 `0x324930`; `func_002B0990`) on top of it. An enemy keeps
//! an aim record of 0x80 bytes in its pvars; the aim springs the record's three angles toward the turn asked for and
//! hangs the record on a joint list of its owner as a joint-modifier node (`0x312480`, AttachManipulator): the
//! list's target joint turned by the angles' quaternion (`0x31d898`) and scaled. Asked for nothing, at scale 1 and
//! back within 0.005 on every axis, the node is taken off again (`0x3124f8`).
//!
//! **The record** (offsets from its start): `+0x00` the joint list, `+0x01` hung on, `+0x10` the quaternion,
//! `+0x20..+0x28` its scale (all three the same), `+0x40` the angles, `+0x50` their speeds, `+0x60` the turn asked
//! for (cleared by each aim; its z also asked by a movement block pointing at the record, crate::moby::ai::joint_turns), `+0x70` the scale asked for (back to 1 after each aim), `+0x78` its owner. Taken off,
//! `+0x00..+0x40` are cleared (so the scale reads 0).
//!
//! **The head aim** asks for the scale (the enemies' big heads, cheat byte 7 `0x1a7a4f`: the class's own scale, else
//! 1) and, with the scale not already that, aims at 0.03 / 0.3. Nothing else asks those classes' head records for a
//! turn, so without the cheat it hangs nothing.
//!
//! In the port the node is the owner's [`crate::moby::Moby::joint_nodes`] entry on the list (the engine fills in the
//! joint from the list's target, as `0x312480` does); the record's words stay in the pvars as the original keeps
//! them. The quaternion is [`crate::quat::from_rows`] of the angles' rows (what `0x31d898`'s three axis turns give).
//!
//! Users: the two aims of crate::moby::classes::level16::drone, crate::moby::classes::gunship and
//! crate::moby::classes::level20::strafer; the head aims of crate::moby::classes::level19::lurker,
//! crate::moby::classes::level14::rappeller and crate::moby::classes::level17::hatchling; the head aims of
//! crate::moby::classes::level20::biter, crate::moby::classes::breeder, crate::moby::classes::level18::stalker
//! (on its inner body, [`head_on`]) and crate::moby::classes::endako_a_enemies; the aims and head aims of
//! crate::moby::classes::critter, crate::moby::classes::beast, the burrowers of crate::moby::classes::enemies,
//! crate::moby::classes::level20::boss, crate::moby::classes::endako_boss and the troopers of
//! crate::moby::classes::trooper and crate::moby::classes::level12::trooper.

use super::{ai, Ctx, MobyWorld};
use crate::hero::physics::{euler_rows, turn_spring, V3};

/// The record's joint list.
const LIST: usize = 0x00;
/// The record's hung-on byte.
const HUNG: usize = 0x01;
/// The node's quaternion.
const QUAT: usize = 0x10;
/// The node's scale (x; y and z follow).
const NOW_SCALE: usize = 0x20;
/// The angles.
pub const ANGLES: usize = 0x40;
/// The angles' speeds.
const SPEEDS: usize = 0x50;
/// The turn asked for (x, y, z).
pub const ASK: usize = 0x60;
/// The scale asked for.
const SCALE: usize = 0x70;
/// The owner (a link, crate::moby::ai::link).
const OWNER: usize = 0x78;
/// The record's size.
const LEN: usize = 0x80;

/// `0x3124f8(owner, rec)`: the record's node taken off its owner's list, the record's first 0x40 bytes cleared.
fn take_off(w: &mut MobyWorld, id: usize, b: usize, owner: usize) {
    let list = w.mobys[id].pv_b(b + LIST);
    if let Some(o) = w.mobys.get_mut(owner) { o.joint_nodes.retain(|n| n.0 != list); }
    let m = &mut w.mobys[id];
    m.pvars[b..b + 0x40].fill(0);
}

/// `0x324208(owner, rec, list, k, d)`: the aim record at `b` of `id`'s pvars sprung toward its asked turn (each angle
/// `0x31f0c8` = crate::hero::physics::turn_spring, `k` / `d`, no limit) and hung on `owner`'s joint list `list`
/// (module docs). A record moved to another owner is taken off the one before.
pub fn aim(w: &mut MobyWorld, id: usize, b: usize, owner: usize, list: u8, k: f32, d: f32) {
    if w.mobys[id].pvars.len() < b + LEN { w.mobys[id].pvars.resize(b + LEN, 0); }
    let was = ai::link(&w.mobys[id], b + OWNER);
    if was != Some(owner) {
        if let Some(o) = was {
            if w.mobys[id].pv_b(b + HUNG) != 0 && w.mobys.get(o).is_some_and(|x| x.state < 0x7f) { take_off(w, id, b, o); }
            w.mobys[id].set_pv_b(b + HUNG, 0);
        }
        ai::set_link(&mut w.mobys[id], b + OWNER, Some(owner));
    }
    let m = &w.mobys[id];
    let ask: V3 = std::array::from_fn(|i| m.pv_f(b + ASK + 4 * i));
    if ask == [0.0; 3] && m.pv_f(b + SCALE) == 1.0 && (0..3).all(|i| m.pv_f(b + ANGLES + 4 * i).abs() < f32::from_bits(0x3ba3_d70a)) {
        if m.pv_b(b + HUNG) != 0 { take_off(w, id, b, owner); }
        return;
    }
    let m = &mut w.mobys[id];
    let mut a = [0.0; 3];
    for i in 0..3 {
        let (mut x, mut v) = (m.pv_f(b + ANGLES + 4 * i), m.pv_f(b + SPEEDS + 4 * i));
        turn_spring(ask[i], k, d, 0.0, &mut x, &mut v, 0);
        m.set_pv_f(b + ANGLES + 4 * i, x);
        m.set_pv_f(b + SPEEDS + 4 * i, v);
        a[i] = x;
    }
    if m.pv_b(b + HUNG) == 0 {
        m.set_pv_b(b + LIST, list);
        m.set_pv_b(b + HUNG, 1);
        for o in [0x1c, 0x20, 0x24, 0x28] { m.set_pv_f(b + o, 1.0); }
    }
    let q = crate::quat::from_rows(&euler_rows(a));
    for (i, c) in q.iter().enumerate() { m.set_pv_f(b + QUAT + 4 * i, *c); }
    let s = m.pv_f(b + SCALE);
    for i in 0..3 { m.set_pv_f(b + NOW_SCALE + 4 * i, s); }
    for i in 0..4 { m.set_pv_f(b + ASK + 4 * i, 0.0); }
    m.set_pv_f(b + SCALE, 1.0);
    let list = m.pv_b(b + LIST);
    let node = rc_formats::moby_anim::JointModifier { quat: q, scale: [s; 3], ..rc_formats::moby_anim::JointModifier::compose(0) };
    if let Some(o) = w.mobys.get_mut(owner) {
        o.joint_nodes.retain(|n| n.0 != list);
        o.joint_nodes.push((list, node));
    }
}

/// `0x327308(owner, rec, scale, list, 1)`: the head record at `b` of `id`'s pvars asked for `scale` with the enemies'
/// big heads on (cheat byte 7), else 1; not at it yet, aimed (0.03, 0.3) on `id`'s joint list `list`.
pub fn head(w: &mut MobyWorld, ctx: &Ctx, id: usize, b: usize, list: u8, scale: f32) { head_on(w, ctx, id, b, id, list, scale); }

/// [`head`] with the record in `id`'s pvars hung on another moby's joint list (`0x327308`'s owner not the record's
/// holder: crate::moby::classes::level18::stalker's inner body).
pub fn head_on(w: &mut MobyWorld, ctx: &Ctx, id: usize, b: usize, owner: usize, list: u8, scale: f32) {
    let s = if ctx.game.cheat(crate::cheats::cheat::ENEMY_HEADS) { scale } else { 1.0 };
    w.mobys[id].set_pv_f(b + SCALE, s);
    if s != w.mobys[id].pv_f(b + NOW_SCALE) { aim(w, id, b, owner, list, f32::from_bits(0x3cf5_c28f), f32::from_bits(0x3e99_999a)); }
}
