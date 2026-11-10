//! The guide **0x0fec** ([`update_0fec`], L12 `0x41b2a8`, 1 placed; L13 `0x41eea0`, 1; the decomp's
//! `func_L12_00419E40`, the same code on the two levels, its helpers `0x41b720` / `0x41b8c8` / `0x41b9e0` /
//! `0x41bb38` / `0x41bb98` too): the guide of level 12's launcher system (crate::moby::classes::level12::launcher),
//! brought out by its carrier 0x0fa5; it walks its path, talking at its stops, and each stop sends the critters 0x0bb8
//! on to their next waypoint. On level 13 it only shows out of the scenes. L12 addresses (the engine functions with
//! the L00 ones as in crate::moby::classes::level12::launcher).
//!
//! Pvars: its path `+0x110`, the critters' group `+0x114`, its next line `+0x11c`, movement `+0x120`, its home and yaw
//! `+0x210` / `+0x21c`, its stop `+0x220` and the point it walks to `+0x224`, the yaw it faces `+0x228`, the waits
//! `+0x22c` / `+0x25c`, its ten lines `+0x230` (+ 2000), the last talk sequence `+0x258`. The set-up (`0x41b720`):
//! update and draw distances 200, its home, hidden without collision (1), sequence 0, movement (radius 0.4, height and
//! steps 0.5, slope π, gravity 1/180, turn 15°, stop 0.25, no fence), the limits 0.0089 / 32/60 / 0.2°/tick / 12°. Each
//! tick its scale 1.333 of its class's and the shadow within 24. **States**: 1 hidden. 2 (out by the teleport beam: the
//! carrier's call `0x41ba60`, crate::moby::classes::level12::launcher): the sequence's end and a line free — the line
//! said ([`say`] `0x41bb38`), 8 and a talk sequence 1..4 (not the last). 8 facing `+0x228` (`0x330dc0` = L00
//! `0x32ceb8`, crate::moby::ai::turn_in_place) till the line waits for its go: let go, 5. 5: the line over — the
//! critters asked on (`0x41bb98`), 6 (sequence 0), the next stop (`0x41b9e0`: the next point with w 0, else the last);
//! else a new talk sequence at each end. 6 along its path (`0x41b8c8`): at the stop — the last 3 (sequence 9), else 7
//! (sequence 0, 120 ticks, turned about). 7 facing it, the wait out and a line free: 8 (a talk sequence, the next
//! line). 3: the end, its stop and line reset, 10. 10: hidden without collision for 25 ticks with the beam's burst,
//! then home (1).

use crate::hero::physics::{add_rot, atan2};
use crate::moby::classes::level12::launcher;
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::{ai, mode, Ctx, MobyWorld};
use crate::path_network;

pub const CLASS: i16 = 0x0fec;

const G_PATH: usize = 0x110;
const G_GROUP: usize = 0x114;
const G_LINE: usize = 0x11c;
const G_MV: usize = 0x120;
pub(crate) const G_HOME: usize = 0x210;
pub(crate) const G_YAW: usize = 0x21c;
const G_STOP: usize = 0x220;
pub(crate) const G_POINT: usize = 0x224;
const G_FACE: usize = 0x228;
const G_WAIT: usize = 0x22c;
const G_LINES: usize = 0x230;
const G_TALK: usize = 0x258;
const G_GONE: usize = 0x25c;

fn st(w: &mut MobyWorld, id: usize, state: u8, sub: i32) { w.mobys[id].set_state(state, sub); }

/// `+0x98` 0 and mode `| 0x41 & !0x1000`: hidden, no collision, no target.
pub(crate) fn rest(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.mode = (m.mode | 0x41) & !mode::TARGET;
}

/// Mode `& 0xffbe | 0x1000` and its class's collision: shown, a target.
pub(crate) fn show(w: &mut MobyWorld, id: usize) {
    let coll = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.mode = (m.mode & 0xffbe) | mode::TARGET;
    m.has_collision = coll;
}

/// The path's points as the level has them now (its network openings applied).
pub(crate) fn path(ctx: &Ctx, i: i32) -> Vec<[f32; 4]> { path_network::points_now(ctx.level, &ctx.game.path_openings, i).map(|p| p.into_owned()).unwrap_or_default() }

/// `0x41bb98(group)`: the critters of the group asked on to their next waypoint.
fn ask_on(w: &mut MobyWorld, g: i32) {
    for k in usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default() {
        if let Some(m) = w.mobys.get_mut(k).filter(|m| m.o_class == launcher::CRITTER) { m.set_pv_i(launcher::C_ASKED, m.pv_i(launcher::C_ASKED) + 1); }
    }
}

/// `0x41bb00(guide)` (the carrier's call; the guide's update has the same test inline): at its path's last point.
pub(crate) fn at_end(w: &MobyWorld, g: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[g];
    let n = path(ctx, m.pv_i(G_PATH)).len() as i32;
    m.pv_i(G_STOP) == n - 1
}

/// `0x41bb38(guide, k)`: with the dialogue free, its line `k` (of ten: `+0x230 + 4k`, + 2000) asked for.
fn say(w: &MobyWorld, g: usize, ctx: &mut Ctx, k: i32) -> bool {
    if !ctx.host.speech_free() || !(0..10).contains(&k) { return false; }
    ctx.host.request_speech(w.mobys[g].pv_i(G_LINES + 4 * k as usize) + 2000);
    true
}

/// A talk sequence 1..4 (`randi(4)`, not the last: then the next), kept at `+0x258`.
fn talk(w: &mut MobyWorld, g: usize, ctx: &mut Ctx) {
    let mut r = ctx.game.rng.randi(4);
    if r == w.mobys[g].pv_i(G_TALK) { r = if r != 3 { r + 1 } else { 0 }; }
    set_seq(w, g, ctx, (r + 1) as u8, 0, 0x14);
    w.mobys[g].set_pv_i(G_TALK, r);
}

/// `0x41b8c8(guide)`: the walk to its point `+0x224` (heading to it); arrived (any of the flags 0x8f), at its stop:
/// true; else on to the next point.
fn walk(w: &mut MobyWorld, g: usize, ctx: &mut Ctx) -> bool {
    let pts = path(ctx, w.mobys[g].pv_i(G_PATH));
    let k = w.mobys[g].pv_i(G_POINT);
    let Some(p) = usize::try_from(k).ok().and_then(|k| pts.get(k)).map(|p| [p[0], p[1], p[2]]) else { return false };
    let q = w.mobys[g].pos;
    if ai::move_step(w, g, ctx, atan2(p[0] - q[0], p[1] - q[1]), p) & 0x8f == 0 { return false; }
    let m = &mut w.mobys[g];
    if m.pv_i(G_POINT) == m.pv_i(G_STOP) { return true; }
    m.set_pv_i(G_POINT, k + 1);
    false
}

/// `0x41b9e0(guide, k)`: the next point after `k` whose w is 0, else the last.
fn next_stop(ctx: &Ctx, path_i: i32, k: i32) -> i32 {
    let pts = path(ctx, path_i);
    let n = pts.len() as i32;
    ((k + 1)..n).find(|&i| pts[i as usize][3] == 0.0).unwrap_or(n - 1)
}

/// `0x41b720` (module docs).
fn guide_setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.update_dist = 200;
        m.draw_dist = 200;
        let (p, yaw) = (m.pos, m.rot[2]);
        m.set_pv_v(G_HOME, p);
        m.set_pv_f(G_YAW, yaw);
        rest(w, id);
        st(w, id, 1, 0);
        if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0); }
        let m = &mut w.mobys[id];
        let o = G_MV;
        m.set_pv_f(o + 0x34, 0.25);
        m.set_pv_f(o, f32::from_bits(0x3ecc_cccd));
        m.set_pv_f(o + 0xc, 0.5);
        m.set_pv_f(o + 0x30, f32::from_bits(0x3e86_0a93));
        m.set_pv_f(o + 0x2c, f32::from_bits(0x3bb6_0b61));
        m.set_pv_i(o + 0x3c, -1);
        m.set_pv_f(o + 0x10, f32::from_bits(0x4049_0fd0));
        m.set_pv_f(o + 4, 0.5);
        m.set_pv_f(o + 8, 0.5);
        // `$gp+0x2c34`: 32 (a second).
        let b = 32.0 * f32::from_bits(0x3c88_8889);
        ai::senses(m, f32::from_bits(0x3c11_a2b4), b, f32::from_bits(0x3b64_c38a), f32::from_bits(0x3e56_7752));
        m.set_pv_i(G_TALK, 0);
        m.set_pv_i(G_LINE, 0);
    }
    // `$gp+0x2c30`: 1.333.
    let s = w.class(w.mobys[id].o_class).map_or(1.0, |c| c.scale) * f32::from_bits(0x3faa_9fbe);
    w.mobys[id].scale = s;
    if w.mobys[id].drawn != 0 { crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16); }
}

/// 0x0fec (module docs).
pub fn update_0fec(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.level == 0xd {
        // Only the scenes show it there (`0x1a8f00` = 2: hidden).
        if ctx.game.scenes.mode != crate::scene::SCENE { show(w, id); } else { rest(w, id); }
        return;
    }
    guide_setup(w, id, ctx);
    if w.mobys[id].sub_state == 1 {
        let r = ai::flight(w, id, ctx);
        if r & 1 != 0 {
            w.delete(id);
            return;
        }
        if r & 2 != 0 { st(w, id, 4, 0); }
    }
    let group = w.mobys[id].pv_i(G_GROUP);
    match w.mobys[id].state {
        2 => {
            let line = w.mobys[id].pv_i(G_LINE);
            if wrapped(w, id, ctx) && say(w, id, ctx, line) {
                st(w, id, 8, -1);
                talk(w, id, ctx);
                w.mobys[id].set_pv_i(G_LINE, line + 1);
            }
        }
        8 => {
            let yaw = w.mobys[id].pv_f(G_FACE);
            ai::turn_in_place(w, id, ctx, yaw);
            if ctx.host.speech_held() {
                ctx.host.speech_go();
                st(w, id, 5, -1);
            }
        }
        5 => {
            if ctx.host.speech_free() {
                ask_on(w, group);
                st(w, id, 6, -1);
                set_seq(w, id, ctx, 0, 0, 0x14);
                let (pi, k) = (w.mobys[id].pv_i(G_PATH), w.mobys[id].pv_i(G_POINT));
                let s = next_stop(ctx, pi, k);
                w.mobys[id].set_pv_i(G_STOP, s);
            } else if wrapped(w, id, ctx) {
                talk(w, id, ctx);
            }
        }
        6 => {
            if walk(w, id, ctx) {
                if at_end(w, id, ctx) {
                    st(w, id, 3, -1);
                    set_seq(w, id, ctx, 9, 0, 0x14);
                } else {
                    st(w, id, 7, -1);
                    set_seq(w, id, ctx, 0, 0, 0x14);
                    let m = &mut w.mobys[id];
                    m.set_pv_i(G_WAIT, 120);
                    let f = add_rot(m.rot[2], f32::from_bits(0x4049_0fd0));
                    m.set_pv_f(G_FACE, f);
                }
            }
        }
        7 => {
            let yaw = w.mobys[id].pv_f(G_FACE);
            ai::turn_in_place(w, id, ctx, yaw);
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(G_WAIT);
            let done = crate::moby::countdown(&mut t);
            m.set_pv_i(G_WAIT, t);
            let line = m.pv_i(G_LINE);
            if done && say(w, id, ctx, line) {
                st(w, id, 8, -1);
                talk(w, id, ctx);
                w.mobys[id].set_pv_i(G_LINE, line + 1);
            }
        }
        3 => {
            if wrapped(w, id, ctx) {
                let m = &mut w.mobys[id];
                m.set_pv_i(G_STOP, 0);
                m.set_pv_i(G_POINT, 0);
                m.set_pv_i(G_LINE, 0);
                st(w, id, 10, -1);
            }
        }
        10 => {
            if w.mobys[id].first() {
                w.mobys[id].set_pv_i(G_GONE, 25);
                rest(w, id);
            }
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(G_GONE);
            let done = crate::moby::countdown(&mut t);
            m.set_pv_i(G_GONE, t);
            if done {
                st(w, id, 1, -1);
                let m = &mut w.mobys[id];
                m.pos = m.pv_v(G_HOME);
            }
            crate::moby::classes::teleport::burst(w, id, ctx);
        }
        1 => rest(w, id),
        _ => {}
    }
    let m = &mut w.mobys[id];
    ai::flash(m, 0x50);
    if m.state_ticks != -1 { m.state_ticks += 1; }
}
