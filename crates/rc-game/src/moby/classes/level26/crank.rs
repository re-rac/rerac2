//! The Jamming Array's bolt crank, class **0x0118** ([`update_0118`], `0x39a1b8`, 6 placed; the decomp's
//! `func_L16_00395250`, shared with L16 `0x396680`, where none is placed): Ratchet fits the wrench to it with □ and
//! turns it by walking round it; its amount (`+0x34`) raises its node 0x0a79 (crate::moby::classes::level26::node,
//! which reads it), and let go before the end it winds back.
//!
//! Its pvars: `+0x04` its node (−1 none), `+0x08` its radius (the amount grows by the turn over 2π of it), `+0x10` its
//! place at the set-up, `+0x20` its angles then, `+0x30` the turn's speed, `+0x34` the amount, `+0x38` this tick's turn
//! toward Ratchet, `+0x3c` the wait before he may take it again, `+0x40` the amount's speed winding back, `+0x44` /
//! `+0x48` its height and its speed at the end, `+0x4c` / `+0x50` the camera's blend and its speed, `+0x58` the times
//! he let go. The camera's two poses (`0x1b2510` / `0x1b2520` quaternions, `0x1b2530` / `0x1b2540` places in its
//! first frame; level globals, one crank at a time) the port keeps in its pvars `+0x60..+0xa0`.
//!
//! * **0**: with the sphere's frames on (`0x1a8ff0`, crate::gravity) only: its class's scale; its rows kept (mode |
//!   0x100) and turned up to the up there (`0x325b90` = L00 `0x326448`; twice `0x325fc0` = L00 `0x326878`, [`align`]);
//!   its place and angles kept (`0x3201b8` = L00 `0x320000`); its node gone: the amount 1 and 4, else 0 and 1.
//! * **1**: 0.3 down its up row (`$gp+0x2f0`). Once the wait is out (`0x2eed80` = L00 `0x2eeab0`) and Ratchet's feet
//!   are within 1.75, unless his wrench is thrown (`0x18b05a`) or a body plays (`0x18c0d4`): the help text 0x9c7
//!   (slot 0x1e, `0x2f5550` = L00 `0x2f5270`) once, with the panel idle; then with the wrench in his hand (`0x18b068`
//!   10, `0x18b040`), □ within 7 ticks (`0x32fdd0` = L00 `0x330240`), less than 0.2 above the ground (`0x18a13c`),
//!   nothing of kind 5 within 5 ([`clear`], `0x39b060`) and the amount short of 1: facing it (his forward and the way
//!   to him flat in its frame, `0x325828` = L00 `0x326248`, their dot under 0.2) he is put in 0x37 (`0x2c4d60` = L00
//!   `0x2c4560`) and the camera comes in ([`camera_in`], `0x39ac98`); 2 either way. Every tick the amount winds back
//!   (1/360 a tick² more; at 0 a third back up while over 1/600 a tick, else still).
//! * **2**: □ again after 20 ticks of his state (`0x189fd8`), the amount at 1 or Ratchet out of 0x37 let it go:
//!   Ratchet idle (from 0x37), the camera let go (`0x41b7d0` = L00 `0x4289f0(1)`), the wait 20, the count of lets-go
//!   on (the help text again at 3 and 6); at 1 (on level 0x1a the help slot off) the amount 1 and 4, else 1. Else 0.3
//!   down its up row (`$gp+0x2f4`), his sequences ([`hero_anim`], `0x39a7c0`), his walk round it ([`walk`],
//!   `0x39a920`), its turn after him ([`turn`], `0x39ab68`) and the camera ([`camera_blend`], `0x39ae38`).
//! * **4**: from 0.3 down (its first tick) it falls (1/360 a tick²) to 0.7 down (`$gp+0x2f8`), bouncing a third back
//!   while faster than 1/600, then not updated (mode | 2).
//!
//! Not ported: the stick's read (`0x2b42f8` = L00 `0x2b3af8`) takes the hero's camera as the hero tick does
//! (crate::camera::Cameras::hero_view), the pad history the last hero tick's (crate::hero::items::ItemBlock::square_ago).

use crate::hero::physics::{add, add_rot, approach, atan2, cross, dist, dot, ease, len, scale, set_len, set_len2, sub, sub_rot, unit, V3};
use crate::gravity::{to_local, to_world};
use crate::moby::attach::{angles_of, rows_of};
use crate::moby::director::{gone, link};
use crate::moby::{countdown, Ctx, MobyWorld};
use crate::quat;

pub const CLASS: i16 = 0x0118;

const NODE: usize = 0x04;
const RADIUS: usize = 0x08;
const BASE: usize = 0x10;
const ANGLES: usize = 0x20;
const SPIN: usize = 0x30;
/// The amount the node reads.
pub const AMOUNT: usize = 0x34;
const TURN: usize = 0x38;
const WAIT: usize = 0x3c;
const BACK: usize = 0x40;
const HEIGHT: usize = 0x44;
const FALL: usize = 0x48;
const BLEND: usize = 0x4c;
const BLEND_VEL: usize = 0x50;
const LET_GO: usize = 0x58;
/// The port's copies of `0x1b2510` / `0x1b2520` / `0x1b2530` / `0x1b2540`.
const CAM_Q: usize = 0x60;
const AIM_Q: usize = 0x70;
const CAM_AT: usize = 0x80;
const AIM_AT: usize = 0x90;

/// `$gp+0x2f0` / `+0x2f4` / `+0x2f8`: its height up its row waiting, turned and at rest; `+0x2fc` the reach Ratchet is
/// held at; `+0x300` the camera's place in the frame toward the node.
const WAITING: f32 = -0.3;
const TURNED: f32 = -0.3;
const REST: f32 = -0.7;
const REACH: f32 = 1.1;
const CAM_OFFSET: V3 = [-9.0, 0.0, 4.0];
/// 1/360, 1/600, −0.333.
const PULL: u32 = 0x3b36_0b61;
const STILL: u32 = 0x3ada_740f;
const BOUNCE: u32 = 0xbeaa_7efa;

fn up_at(m: &crate::moby::Moby, h: f32) -> V3 { add(set_len(m.rows[2], h), m.pv_v(BASE)) }

/// `0x326878(rows, v)` = L00 `0x3268c8(rows, −v)`: its up row turned toward `v` by the quaternion
/// `(½·r₂ × unit(−v), −√(1 − |…|²))` (`0x2efc20` + `0x2efa00`).
fn align(rows: &[V3; 3], v: V3) -> [V3; 3] {
    let q = unit(scale(v, -1.0));
    // `0x2ef228(out, q, r₂)` = r₂ × q.
    let a = scale(cross(rows[2], q), 0.5);
    let x = len(a);
    quat::turn_rows(rows, [a[0], a[1], a[2], -(1.0 - x * x).sqrt()])
}

/// 0x0118 (module docs).
pub fn update_0118(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => set_up(w, id, ctx),
        1 => waiting(w, id, ctx),
        2 => turning(w, id, ctx),
        4 => resting(w, id),
        _ => {}
    }
}

fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !ctx.hero.gravity.on { return; }
    let s = w.class(CLASS).map_or(1.0, |c| c.scale);
    let m = &mut w.mobys[id];
    m.scale = s;
    let up = ctx.hero.gravity.up_at(m.pos);
    m.mode |= crate::moby::mode::KEEP_ROWS;
    m.rows = align(&align(&m.rows, up), up);
    m.set_pv_v(BASE, m.pos);
    let a = angles_of(&m.rows);
    m.set_pv_v(ANGLES, a);
    m.set_pv_i(LET_GO, 0);
    let node = m.pv_i(NODE);
    if usize::try_from(node).map_or(true, |k| gone(w, k)) {
        w.mobys[id].set_pv_f(AMOUNT, 1.0);
        w.mobys[id].set_state(4, -1);
    } else {
        w.mobys[id].set_pv_f(AMOUNT, 0.0);
        w.mobys[id].set_state(1, -1);
    }
}

/// □ pressed within the last `n` ticks.
fn square_within(ctx: &Ctx, n: i32) -> bool { ctx.hero.items.square_ago.is_some_and(|t| t < n) }

/// The help text 0x9c7 in slot 0x1e, the panel idle.
fn help(ctx: &mut Ctx) {
    let g = &mut *ctx.game;
    if g.help_counts.get(0x1e).is_some_and(|&c| c != -1) && g.help_state == 0 && g.help_req.is_none() {
        g.help(0x9c7, 0x1e);
        g.help_counts[0x1e] = -1;
    }
}

fn waiting(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.pos = up_at(m, WAITING);
    let mut t = m.pv_i(WAIT);
    let out = countdown(&mut t);
    m.set_pv_i(WAIT, t);
    if out && dist(m.pos, ctx.hero.pos) < 1.75 && ctx.hero.items.thrown == 0 && ctx.hero.mode == 0 {
        help(ctx);
        let it = &ctx.hero.items;
        let ready = it.item == 10 && it.hand.is_some() && square_within(ctx, 7) && ctx.hero.height < f32::from_bits(0x3e4c_cccd);
        if ready && clear(w, id, ctx) && w.mobys[id].pv_f(AMOUNT) < 1.0 {
            let m = &w.mobys[id];
            let mut fwd = to_local(ctx.hero.rows[0], &m.rows);
            let mut way = to_local(sub(ctx.hero.pos, m.pos), &m.rows);
            fwd[2] = 0.0;
            way[2] = 0.0;
            if dot(set_len(way, 1.0), set_len(fwd, 1.0)) < f32::from_bits(0x3e4c_cccd) {
                ctx.hero.request_state(0x37, true);
                camera_in(w, id, ctx);
            }
            w.mobys[id].set_state(2, -1);
        }
    }
    // The amount winds back.
    let m = &mut w.mobys[id];
    let mut a = m.pv_f(AMOUNT);
    if a == 0.0 { return; }
    let v = m.pv_f(BACK) - f32::from_bits(PULL);
    m.set_pv_f(BACK, v);
    a += v;
    m.set_pv_f(AMOUNT, a);
    if 0.0 < a { return; }
    m.set_pv_f(AMOUNT, 0.0);
    if f32::from_bits(STILL) < v.abs() {
        let b = v * f32::from_bits(BOUNCE);
        m.set_pv_f(AMOUNT, b);
        m.set_pv_f(BACK, b);
    } else {
        m.set_pv_f(BACK, 0.0);
    }
}

fn turning(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let amount = w.mobys[id].pv_f(AMOUNT);
    let again = square_within(ctx, 7) && 21 <= ctx.hero.timer;
    if !again && amount < 1.0 && ctx.hero.state == 0x37 {
        let m = &mut w.mobys[id];
        m.pos = up_at(m, TURNED);
        hero_anim(w, id, ctx);
        walk(w, id, ctx);
        turn(w, id, ctx);
        camera_blend(w, id, ctx);
        return;
    }
    if ctx.hero.state == 0x37 { ctx.hero.request_state(0, true); }
    ctx.game.cameras.camera_script2(1, ctx.hero);
    let m = &mut w.mobys[id];
    m.set_pv_i(WAIT, 20);
    let n = m.pv_i(LET_GO) + 1;
    m.set_pv_i(LET_GO, n);
    if n == 3 || n == 6 { ctx.game.help(0x9c7, 0x1e); }
    if 1.0 <= w.mobys[id].pv_f(AMOUNT) {
        if ctx.game.level == 0x1a { if let Some(c) = ctx.game.help_counts.get_mut(0x1e) { *c = -1; } }
        w.mobys[id].set_pv_f(AMOUNT, 1.0);
        w.mobys[id].set_state(4, -1);
    } else {
        w.mobys[id].set_state(1, -1);
    }
}

fn resting(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    if m.first() { m.set_pv_f(HEIGHT, WAITING); }
    let mut v = m.pv_f(FALL) - f32::from_bits(PULL);
    let mut h = m.pv_f(HEIGHT) + v;
    if h <= REST {
        h = REST;
        if f32::from_bits(STILL) < v.abs() { v *= f32::from_bits(BOUNCE) } else { m.mode |= crate::moby::mode::NO_UPDATE }
    }
    m.set_pv_f(FALL, v);
    m.set_pv_f(HEIGHT, h);
    m.pos = up_at(m, h);
}

/// `0x39b060`: no moby of a class of kind 5 (class header `+0x46`) within 5 of it (`0x2e1bd0` = L00 `0x2e14b0`).
fn clear(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let Some(sc) = ctx.scene else { return true };
    let near = crate::collision::coll_sphere_mobys(sc, w.mobys[id].pos, 5.0, crate::collision::QueryFlags(0), Some(id));
    !near.iter().any(|&k| w.mobys.get(k).and_then(|m| w.class(m.o_class)).is_some_and(|c| c.kind == 5))
}

/// `0x39a7c0`: Ratchet's sequences (`0x2d2d08` = L00 `0x2d2508`): the fit 0x44 over → the hold 0x3f (blend 7); 0x3f
/// with the crank turning → 0x40; 0x40 faster than 0.05 → the push 0x41, stopped → 0x3f; 0x41 under 0.05 → 0x40
/// (blend 10).
fn hero_anim(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let spin = w.mobys[id].pv_f(SPIN);
    let fast = f32::from_bits(0x3d4c_ccce);
    let v = ctx.hero_view;
    let to = match v.seq_b {
        0x44 if v.flags & 2 != 0 => Some((7.0, 0x3f)),
        0x3f if spin != 0.0 => Some((10.0, 0x40)),
        0x40 if fast < spin => Some((10.0, 0x41)),
        0x40 if spin == 0.0 => Some((10.0, 0x3f)),
        0x41 if spin < fast => Some((10.0, 0x40)),
        _ => None,
    };
    if let Some((blend, seq)) = to { ctx.hero.anim_req = Some((blend, seq, 0)); }
}

/// `0x39a920`: Ratchet walked round it. The stick read (`0x2b42f8(0, 1)` = L00 `0x2b3af8`); his stick's way kept, then
/// the way round it at his place (`0x2ef228` = L00 `0x2eef88`: its up row × the way to him), unit. His
/// goal: his place in its frame at the reach across (`0x2ef428` = L00 `0x2ef188`), out of its frame. With the stick
/// over half (`0x18c2d8`) and over half along the way round, the turn's speed eased (`0x31ecd0` = L00 `0x31eb18`, 1/720 a
/// tick) to 1/12 of that share, else to 0; turning, the goal on by the way round × the speed and the amount by the
/// speed over 2π × its radius (at most 1). His `0x189f10..0x189fb0` cleared (`0x2eeec0`), his velocity the way to the
/// goal, at most 0.1 (`0x321840` = L00 `0x321688`).
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let cam_rows = ctx.game.cameras.hero_view().map_or_else(|| ctx.host.camera_rows(), |v| v.1);
    let h = &mut *ctx.hero;
    h.stick_target_in(&cam_rows, cam_rows[0][1].atan2(cam_rows[0][0]), false, 1.0, 0);
    let was = h.stick_dir;
    let m = &mut w.mobys[id];
    let d = sub(h.pos, m.pos);
    // `0x2ef228(out, d, r₂)` = r₂ × d.
    let round = set_len(cross(m.rows[2], d), 1.0);
    h.stick_dir = round;
    let mut goal = to_world(set_len2(to_local(d, &m.rows), REACH), &m.rows);
    let share = dot(was, round);
    let mut spin = m.pv_f(SPIN);
    let step = f32::from_bits(0x3ab6_0b61);
    if 0.5 < h.stick_mag && 0.5 < share { approach(share * f32::from_bits(0x3daa_aaab), step, &mut spin) } else { approach(0.0, step, &mut spin) };
    m.set_pv_f(SPIN, spin);
    if spin != 0.0 {
        goal = add(goal, scale(round, spin));
        let a = (m.pv_f(AMOUNT) + spin / (m.pv_f(RADIUS) * f32::from_bits(0x40c9_0fdb))).min(1.0);
        m.set_pv_f(AMOUNT, a);
    }
    let goal = add(goal, m.pos);
    (h.vel, h.disp, h.eff, h.eff_v, h.eff_h, h.carry, h.momentum) = ([0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3]);
    (h.carry_yaw, h.eff_len, h.eff_len_xy, h.fwd_speed, h.slope_ratio, h.gravity.push_step) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let v = sub(goal, h.pos);
    let most = f32::from_bits(0x3dcc_ccce);
    h.vel = if most < len(v) { set_len(v, most) } else { v };
}

/// `0x39ab68`: it turns after Ratchet: his forward in its frame (`0x325828`), the turn the negated yaw of it
/// (`0x2ef7b0` = L00 `0x2ef510`), at most 12° (`+0x38`); past 0.1° its rows turned about its up row by it
/// (`0x2eff00` / `0x2efce0` = L00 `0x2efc20` / `0x2efa00`).
fn turn(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    let f = to_local(ctx.hero.rows[0], &m.rows);
    let most = f32::from_bits(0x3e56_7752);
    let a = (-atan2(f[0], f[1])).clamp(-most, most);
    m.set_pv_f(TURN, a);
    if a.abs() <= f32::from_bits(0x3ae4_c38a) { return; }
    let (s, c) = (a * 0.5).sin_cos();
    let v = set_len(m.rows[2], s);
    m.rows = quat::turn_rows(&m.rows, [v[0], v[1], v[2], c]);
}

/// `0x39ac98`: with a node, the script camera brought in where the view is (`0x41b660` = L00 `0x428880`, mode 1); the
/// blend 0; its first pose the view's (the quaternion of its angles, `0x31da50` = L00 `0x31d898`; its place in its first
/// frame), the second 9 back and 4 up (`$gp+0x300`) in the frame toward 3 up the node's up row from it (its up row the
/// third, the second their cross), looking along it.
fn camera_in(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(node) = link(w, w.mobys[id].pv_i(NODE)) else { return };
    let (view, euler) = ctx.game.cameras.view_euler();
    ctx.game.cameras.camera_script(view, euler, crate::script_camera::MODE_SNAP, 0, false, ctx.hero);
    let (np, nr) = (w.mobys[node].pos, w.mobys[node].rows);
    let m = &mut w.mobys[id];
    m.set_pv_f(BLEND, 0.0);
    m.set_pv_f(BLEND_VEL, 0.0);
    let first = rows_of(m.pv_v(ANGLES));
    m.set_pv_q(CAM_Q, quat::from_rows(&rows_of(euler)));
    m.set_pv_v(CAM_AT, to_local(sub(view, m.pv_v(BASE)), &first));
    let at = add(set_len(nr[2], 3.0), np);
    let a0 = set_len(sub(at, m.pos), 1.0);
    let a2 = first[2];
    // `0x2ef228(out, a0, a2)` = a2 × a0.
    let axes = [a0, cross(a2, a0), a2];
    m.set_pv_v(AIM_AT, to_local(to_world(CAM_OFFSET, &axes), &first));
    m.set_pv_q(AIM_Q, quat::from_rows(&rows_of(angles_of(&axes))));
}

/// `0x39ae38`: with a node, the blend eased to 1 (`0x31ef30` = L00 `0x31ed78`: 1.85e-4 a tick², at most 0.011); the
/// camera's place between the two (their yaws turned the short way, `0x31eb38` = L00 `0x31e980`, their lengths and
/// heights lerped; at 1 the second), out of its first frame; its angles the two quaternions' blend (`0x2efdb8` = L00
/// `0x2efad8`); both set as the script camera's targets (`0x41b598` / `0x41b5f0` = L00 `0x4287b8` / `0x428810`).
fn camera_blend(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.pv_i(NODE) == -1 { return; }
    let (mut t, mut v) = (m.pv_f(BLEND), m.pv_f(BLEND_VEL));
    if t < 1.0 {
        let a = f32::from_bits(0x3941_fc90);
        ease(1.0, a, a, f32::from_bits(0x3c35_dcc7), &mut t, &mut v);
        m.set_pv_f(BLEND, t);
        m.set_pv_f(BLEND_VEL, v);
    }
    let first = rows_of(m.pv_v(ANGLES));
    let (b, c) = (m.pv_v(CAM_AT), m.pv_v(AIM_AT));
    let p = if t < 1.0 {
        let (a0, a1) = (atan2(b[0], b[1]), atan2(c[0], c[1]));
        let (l0, l1) = (len(b), len(c));
        let yaw = add_rot(a0, sub_rot(a1, a0) * t);
        let l = (l1 - l0) * t + l0;
        [yaw.cos() * l, yaw.sin() * l, (c[2] - b[2]) * t + b[2]]
    } else {
        c
    };
    let p = add(to_world(p, &first), m.pv_v(BASE));
    let q = quat::nlerp(m.pv_q(CAM_Q), m.pv_q(AIM_Q), t);
    ctx.game.cameras.camera_script_targets(Some(p), Some(angles_of(&quat::rows(q))));
}
