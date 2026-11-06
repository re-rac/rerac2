//! Snivelak's lightning hazard **0x07fb** ([`update_07fb`], `0x3ab548`, 48 placed; the decomp's
//! `func_L16_003AA108`): level 13's power-line bolt (crate::moby::classes::level13::powerline) with a trigger and a
//! shared loop. Its tick (`0x3abec0`) and draw (`0x3ac090`) are level 13's `0x41bfc0` / `0x41c190`
//! ([`powerline::step`], [`powerline::draw_0ee7`], drawn through level 13's draw callbacks); its set-up `0x3ab830`
//! is level 13's `0x41b9e0` in its own shape ([`SHAPE_07FB`], [`powerline::build`]).
//!
//! Its pvars are 0x0ee7's ([`pv`]) with `+0x628` a moby it waits on (−1 none), `+0x62c` the group that keeps it
//! going, `+0x660` its cuboid, and `+0x664` / `+0x668` two shared words (crate::moby::MobyWorld::shared_i): the
//! group's loop sound and the moby that started it (here its index + 1; 0 none).
//!
//! **The head**, in every state but 0: in **1**, once the moby it waits on is gone (state 0xfe / 0xfd) and Ratchet is
//! in its cuboid (`0x31ae60` = L00 `0x321858`), the loop let go and 2; in **2** / **3** while its group has a member
//! alive (`0x315fd0` = L00 `0x31c8e0`, crate::moby::classes::props::group_alive). Otherwise the loop let go
//! ([`loop_off`]) and nothing more.
//!
//! **States** (`$gp+0x854` ≠ 0 skips them all; 0 here):
//! * **0**: a bolt; the countdown `+0x10` its first wait `+0xc`; 1; its radius 2.5; `+8` / `+0xa` 1; the loop −1.
//! * **1**: waiting (the head).
//! * **2**: the countdown out (`0x2e7e88` = L00 `0x2eeae0`): 3, the countdown 120.
//! * **3**: the loop, sound 0 of class 0x10fb looped (`0x358578` = L00 `0x360698`), unless it plays for its owner; the
//!   countdown out: 2 when `+6` (the pause), the countdown 0; else a new bolt each time the last is gone.
//!
//! Then while its bolt lives: its tick (unless `$gp+0x858`, 0) and, past its last tick (`$gp+0x850`, 0) and not
//! hidden (`$gp+0x85c`, 0), its draw (`0x2e0b70(0x3ac090)` = L00 `0x2e7920`).

use crate::moby::classes::level13::powerline::{self, pv, Shape};
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x07fb;

const WAIT_MOBY: usize = 0x628;
const GROUP: usize = 0x62c;
const CUBOID: usize = 0x660;
const LOOP: usize = 0x664;
const OWNER: usize = 0x668;

/// `0x3ab830`'s shape on this level (the level number `0x1a79f8` is 16 or 0xd5): 40..45 long, spread 1
/// (`$gp+0x8c8`), drifting 0.2 a tick (`0x3e4cccce`), the branches 1 a step and drifting 0.2, no yaw wobble.
pub const SHAPE_07FB: Shape = Shape {
    length: (40.0, 45.0),
    spread: 1.0,
    drift: f32::from_bits(0x3e4c_ccce),
    step: 1.0,
    branch_drift: f32::from_bits(0x3e4c_ccce),
    wobble: false,
};

/// The group's loop plays for its owner (the sound record's owner and its live byte: here the handle's).
fn loop_playing(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    let (h, owner) = (w.shared_i(m.pv_i(LOOP)), w.shared_i(m.pv_i(OWNER)));
    owner != 0 && h != -1 && ctx.host.sound_playing(h)
}

/// The loop let go: with an owner, stopped (`0x3581d0` = L00 `0x3602f0`) while it plays and −1.
fn loop_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (lp, op) = (w.mobys[id].pv_i(LOOP), w.mobys[id].pv_i(OWNER));
    if w.shared_i(op) == 0 { return; }
    let h = w.shared_i(lp);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.set_shared_i(lp, -1);
}

fn tick_count(w: &mut MobyWorld, id: usize) -> bool {
    let m = &mut w.mobys[id];
    let mut c = m.pv_s(pv::COUNT) as i32;
    let out = countdown(&mut c);
    m.set_pv_s(pv::COUNT, c as i16);
    out
}

/// 0x07fb (module docs).
pub fn update_07fb(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {}
        1 => {
            let k = w.mobys[id].pv_i(WAIT_MOBY);
            let gone = usize::try_from(k).ok().and_then(|k| w.mobys.get(k)).is_none_or(|t| t.state == 0xfe || t.state == 0xfd);
            if !(gone && ctx.level.in_cuboid(w.mobys[id].pv_i(CUBOID), ctx.hero.pos)) {
                loop_off(w, id, ctx);
                return;
            }
            loop_off(w, id, ctx);
            w.mobys[id].state = 2;
        }
        _ => {
            if crate::moby::classes::props::group_alive(w, w.mobys[id].pv_i(GROUP)) == 0 {
                loop_off(w, id, ctx);
                return;
            }
        }
    }
    match w.mobys[id].state {
        0 => {
            powerline::build(w, id, ctx, &SHAPE_07FB);
            let m = &mut w.mobys[id];
            m.set_pv_s(pv::COUNT, m.pv_s(pv::FIRST));
            m.state = 1;
            m.set_pv_f(pv::RADIUS, 2.5);
            m.set_pv_s(0xa, 1);
            m.set_pv_s(8, 1);
            let lp = m.pv_i(LOOP);
            w.set_shared_i(lp, -1);
        }
        2 => {
            if tick_count(w, id) {
                let m = &mut w.mobys[id];
                m.state = 3;
                m.set_pv_s(pv::COUNT, 0x78);
            }
        }
        3 => {
            if !loop_playing(w, id, ctx) {
                let h = ctx.host.play_sound_of(&w.mobys[id], 0x10fb, 0, 4);
                let (lp, op) = (w.mobys[id].pv_i(LOOP), w.mobys[id].pv_i(OWNER));
                w.set_shared_i(lp, h);
                w.set_shared_i(op, id as i32 + 1);
            }
            if tick_count(w, id) {
                let m = &mut w.mobys[id];
                if m.pv_s(pv::PAUSES) != 0 { m.state = 2; }
                m.set_pv_s(pv::COUNT, 0);
            } else if w.mobys[id].pv_s(pv::LIFE) == 0 {
                powerline::build(w, id, ctx, &SHAPE_07FB);
            }
        }
        _ => {}
    }
    if w.mobys[id].pv_s(pv::LIFE) == 0 { return; }
    powerline::step(w, id);
    if powerline::GONE_AT < w.mobys[id].pv_s(pv::LIFE) { w.register_draw(id); }
}
