//! Level 6's wrench crank **0x0b57** (L06 `0x41b200`, the decomp's `func_L06_00419CA8`, 3 placed): Ratchet fits the
//! wrench to it with □ (his state 0x37, as on the Jamming Array's crank, crate::moby::classes::level26::crank) and
//! walks round it under a swinging script camera; turned far enough it is done for good (its uid counted) and with a
//! mission its mission is done and the checkpoint set.
//!
//! Its pvars: `+0x00` the amount (0..1), `+0x14` the ticks since it was let go (31 before it can be taken again),
//! `+0x18` the turn so far, `+0x1c` Ratchet's speed round it, `+0x20` the camera's cuboid, `+0x24` his fall speed,
//! `+0x28` / `+0x2c` the swing camera's distance curve's slopes (`0x479b70`: the script camera's D+0x124 / D+0x128),
//! `+0x30` the turns it takes, `+0x34` the checkpoint's cuboid (−1 none), `+0x38` ≠ 0 it may be taken again after
//! it is done, `+0x3c` its height at rest. Without pvars it spins 6° a tick.
//!
//! * Ratchet out of 0x37 while it is in 3: 1, the script camera let go (`0x479d50(4)` = L00 `0x4289f0`).
//! * **0**: its rest height its place's; 1.
//! * **1**: done before (its uid gone for good `0x21ecc4`, crate::game::GameState::uid_dead, or counted): 0.5 down,
//!   the amount 1, 5. Ratchet within 1.75 across, his wrench not thrown (`0x18b05a`), no body playing (`0x18c0d4`),
//!   facing it within 90°, his feet 0.3 above its top (within 0.2), the wrench in his hand (`0x18b068` 10), □ within
//!   7 ticks (`0x370fd8` = L00 `0x330240`), let go 31 ticks or more ago, at rest and short of the end: the count 0,
//!   the pull 0 and, with no moby of the target list (`0x1fb760` = L00 `0x1fac60`) within 3.25, Ratchet in 0x37
//!   (`0x303de0` = L00 `0x2c4560`), the script camera swung in from the view over 180 ticks (`0x479be0` = L00
//!   `0x428880`, mode 3) to the cuboid `+0x20`'s place and angles (`0x479ab8` / `0x479b10`) with the curve
//!   `+0x28` / `+0x2c`, his fall 0, the turn the amount's; 3. Else short of the end: counted, the amount back toward 0
//!   by 1/60 a tick (`0x35ed48` = L00 `0x31eb18`), turning back with it (`+0x30 · 6°` a tick), rising 1/120 a tick to
//!   rest; at the end with `+0x38`: the mission ([`finish`]) and sinking 1/120 a tick to 0.5 below rest.
//! * **3** turning ([`turn`]): □ after 60 ticks of his state (`0x189fd8`) or the amount at 1: Ratchet idle; with
//!   `+0x38` back to 1; else at the end the uid counted (at most 7), its class sound 0, 4 and the mission; the camera
//!   let go (`0x479d50(3)`).
//! * **4**: sinking 1/120 a tick to 0.5 below rest, then the amount 1, 5. **5**: the amount 1.
//!
//! The amount is noted in the level's state log as it is finished (`0x39b620(+0x00, 4, m, 2, 0x21dc50)`,
//! crate::game::GameState::log_state: at the amount 1 in 3 and before the mission and checkpoint, [`finish`]), so a
//! death after the checkpoint gives the respawned crank its amount back (crate::save::replay_log).

use crate::hero::physics::{add, add_rot, approach, approach_angle, atan2, dist, dist2, dot, euler_rows, len2, rot_dist, scale, spring, sub, sub_rot};
use crate::moby::{Ctx, MobyWorld};

pub const CLASS: i16 = 0x0b57;

const AMOUNT: usize = 0x00;
const COUNT: usize = 0x14;
const TURNED: usize = 0x18;
const SPIN: usize = 0x1c;
const CAMERA: usize = 0x20;
const FALL: usize = 0x24;
const CURVE: usize = 0x28;
const TURNS: usize = 0x30;
const CHECKPOINT: usize = 0x34;
const AGAIN: usize = 0x38;
const REST: usize = 0x3c;

fn f(bits: u32) -> f32 { f32::from_bits(bits) }

/// 1/120 a tick.
const SINK: u32 = 0x3c08_8889;
const TAU: u32 = 0x40c9_0fdb;

/// □ within the last `n` ticks (`0x330240(0x80, n, 0)`).
fn square_within(ctx: &Ctx, n: i32) -> bool { ctx.hero.items.square_ago.is_some_and(|t| t < n) }

/// With a checkpoint cuboid and its mission not done: the mission done and the checkpoint there (`0x3132a8`,
/// `0x39b0a8` = L00 `0x359b20`).
fn finish(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let c = m.pv_i(CHECKPOINT);
    if c == -1 || crate::moby::director::mission_is_done(ctx.game, m.mission as i32) { return; }
    log_amount(w, id, ctx);
    let m = &w.mobys[id];
    let Some((pos, rot)) = ctx.level.cuboid_place(c) else { return };
    crate::moby::director::finish(ctx, m.mission as i32, pos, rot);
}

/// `0x39b620(m + 0x68 → +0x00, 4, m, 2, log)`: the amount into the level's state log.
fn log_amount(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let v = m.pv_f(AMOUNT).to_le_bytes();
    ctx.game.log_state(m, 2, AMOUNT as i32, &v);
}

/// 0x0b57 (module docs).
pub fn update_0b57(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.is_empty() {
        let m = &mut w.mobys[id];
        m.rot[2] = add_rot(m.rot[2], f(0x3dd6_7752));
        return;
    }
    if ctx.hero.state != 0x37 && w.mobys[id].state == 3 {
        w.mobys[id].state = 1;
        ctx.game.cameras.camera_script2(4, ctx.hero);
        return;
    }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let z = m.pos[2];
            m.set_pv_f(REST, z);
            m.state = 1;
        }
        1 => waiting(w, id, ctx),
        3 => turn(w, id, ctx),
        4 => {
            let m = &mut w.mobys[id];
            m.pos[2] -= f(SINK);
            let bottom = m.pv_f(REST) - 0.5;
            if m.pos[2] <= bottom {
                m.pos[2] = bottom;
                m.state = 5;
                m.set_pv_f(AMOUNT, 1.0);
            }
        }
        5 => w.mobys[id].set_pv_f(AMOUNT, 1.0),
        _ => {}
    }
}

fn waiting(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let uid = m.uid;
    let dead = ctx.game.uid_dead.get(uid.max(0) as usize).copied().unwrap_or(0) != 0;
    if dead || ctx.game.uid_count(uid) != 0 {
        let m = &mut w.mobys[id];
        m.state = 5;
        m.pos[2] -= 0.5;
        m.set_pv_f(AMOUNT, 1.0);
        return;
    }
    let h = &*ctx.hero;
    let it = &h.items;
    let face = atan2(m.pos[0] - h.pos[0], m.pos[1] - h.pos[1]);
    let take = dist2(m.pos, h.pos) < 1.75
        && it.thrown == 0
        && h.mode == 0
        && rot_dist(h.rot[2], face) < f(0x3fc9_0fdc)
        && (h.pos[2] - f(0x3e99_999a) - m.pos[2]).abs() < f(0x3e4c_cccd)
        && it.item == 10
        && it.hand.is_some()
        && square_within(ctx, 7)
        && 31 <= m.pv_i(COUNT) as u32
        && m.pos[2] == m.pv_f(REST)
        && m.pv_f(AMOUNT) < 1.0;
    if take {
        w.mobys[id].set_pv_i(COUNT, 0);
        w.level06.crank_pull = 0.0;
        let here = w.mobys[id].pos;
        let clear = !w.targets(ctx.host.camera_pos()).iter().any(|t| dist(here, t.pos) < 3.25);
        if clear {
            ctx.hero.request_state(0x37, true);
            let (view, euler) = ctx.game.cameras.view_euler();
            ctx.game.cameras.camera_script(view, euler, crate::script_camera::MODE_SWING, 0xb4, false, ctx.hero);
            let m = &w.mobys[id];
            if let Some((pos, rot)) = ctx.level.cuboid_place(m.pv_i(CAMERA)) {
                ctx.game.cameras.camera_script_targets(Some(pos), Some(rot));
            }
            ctx.game.cameras.script.curve = [m.pv_f(CURVE), m.pv_f(CURVE + 4)];
            let m = &mut w.mobys[id];
            m.state = 3;
            m.set_pv_f(FALL, 0.0);
            let t = m.pv_f(AMOUNT) * f(TAU) * m.pv_f(TURNS);
            m.set_pv_f(TURNED, t);
        }
        return;
    }
    let m = &mut w.mobys[id];
    if m.pv_f(AMOUNT) < 1.0 {
        m.set_pv_i(COUNT, m.pv_i(COUNT) + 1);
        let mut a = m.pv_f(AMOUNT);
        approach(0.0, f(0x3c88_8889), &mut a);
        m.set_pv_f(AMOUNT, a);
        if 0.0 < a { m.rot[2] = sub_rot(m.rot[2], m.pv_f(TURNS) * f(0x3dd6_7751)); }
        m.pos[2] += f(SINK);
        let rest = m.pv_f(REST);
        if rest <= m.pos[2] { m.pos[2] = rest; }
    } else if m.pv_i(AGAIN) != 0 {
        finish(w, id, ctx);
        let m = &mut w.mobys[id];
        m.pos[2] -= f(SINK);
        let bottom = m.pv_f(REST) - 0.5;
        if m.pos[2] <= bottom { m.pos[2] = bottom; }
    }
}

/// State 3 (module docs): Ratchet walked round it by the stick, held 1.05 from it, and it turns after him.
fn turn(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let at = w.mobys[id].pos;
    let angle = atan2(ctx.hero.pos[0] - at[0], ctx.hero.pos[1] - at[1]);
    if (square_within(ctx, 7) && 61 <= ctx.hero.timer) || w.mobys[id].pv_f(AMOUNT) == 1.0 {
        ctx.hero.request_state(0, true);
        let m = &mut w.mobys[id];
        if m.pv_i(AGAIN) != 0 {
            m.state = 1;
        } else if m.pv_f(AMOUNT) == 1.0 {
            log_amount(w, id, ctx);
            let m = &mut w.mobys[id];
            m.state = 4;
            let uid = m.uid;
            ctx.game.count_uid(uid);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            finish(w, id, ctx);
        }
        ctx.game.cameras.camera_script2(3, ctx.hero);
        return;
    }
    // The stick in the view's turn (`0x2ef6d8` of the view's yaw `0x1b9858`, `0x2ef340`).
    let h = &mut *ctx.hero;
    let mut input = [-h.stick[1], -h.stick[0], 0.0];
    let l = len2(input);
    if 1.0 < l { input = scale(input, 1.0 / l); }
    let yaw = ctx.game.cameras.view_euler().1[2];
    let input = crate::gravity::to_world(input, &euler_rows([0.0, 0.0, yaw]));
    let facing = [h.rot[2].cos(), h.rot[2].sin(), 0.0];
    let m = &mut w.mobys[id];
    let step = f(0x3ac8_3fb8);
    let mut spin = m.pv_f(SPIN);
    spin += (dot(facing, input) - spin).clamp(-step, step);
    if f(0x3d7c_9631) < spin { spin = f(0x3d7c_9631); }
    if spin < 0.0 { spin = 0.0; }
    if spin.abs() < f(0x39c8_3fb8) { spin = 0.0; }
    m.set_pv_f(SPIN, spin);
    let motion = scale(facing, spin);
    // His sequences (`0x311d88` = L00 `0x2d2508`): the fit 0x44 over → the hold 0x3f (blend 7); else, no blend
    // running: 0x40 stopped → 0x3f, 0x3f moving → 0x40, 0x40 faster than 0.037 → the push 0x41, 0x41 under 0.0247 →
    // 0x40 (blend 14, 16 the last).
    let v = ctx.hero_view;
    if v.flags & 2 != 0 && v.seq_b == 0x44 {
        h.anim_req = Some((7.0, 0x3f, 0));
    } else if v.seq_b != 0x44 && !v.blending() {
        let to = match v.seq_a {
            0x40 if spin.abs() == 0.0 => Some((14.0, 0x3f)),
            0x3f => (0.0 < spin).then_some((14.0, 0x40)),
            0x40 if f(0x3d17_8d51) < spin => Some((14.0, 0x41)),
            0x41 if spin < f(0x3cca_11c1) => Some((16.0, 0x40)),
            _ => None,
        };
        if let Some((blend, seq)) = to { h.anim_req = Some((blend, seq, 0)); }
    }
    let z = h.pos[2];
    h.pos = add(h.pos, motion);
    let round = atan2(h.pos[0] - at[0], h.pos[1] - at[1]);
    let reach = f(0x3f86_6666);
    let mut d = sub(add(at, [round.cos() * reach, round.sin() * reach, 0.0]), h.pos);
    d[2] = 0.0;
    let mut gap = -len2(d);
    let pull = &mut w.level06.crank_pull;
    spring(0.0, f(0x3c75_c28f), f(0x3e99_999a), f(0x3dee_eef0), &mut gap, pull);
    h.pos = add(h.pos, scale(d, *pull));
    h.pos[2] = z;
    let side = if ctx.game.cheat(crate::cheats::cheat::LEFT_HANDED) { -f(0x3fc9_0fdb) } else { f(0x3fc9_0fdb) };
    h.target_yaw = add_rot(side, round);
    approach_angle(h.target_yaw, f(0x3dd6_7752), &mut h.rot[2], 0);
    // His fall (1/367 a tick², `0x35d038` = L00 `0x31ce20` from 2 above).
    let m = &mut w.mobys[id];
    let fall = m.pv_f(FALL) + f(0x3b32_6750);
    m.set_pv_f(FALL, fall);
    h.pos[2] -= fall - 2.0;
    let p = h.pos;
    let g = crate::moby::ai::ground_z(ctx, 0.5, p, 0);
    let h = &mut *ctx.hero;
    h.pos[2] -= 2.0;
    if h.pos[2] < g {
        h.pos[2] = g;
        w.mobys[id].set_pv_f(FALL, 0.0);
    }
    let m = &mut w.mobys[id];
    if v.seq_b != 0x44 {
        let most = f(0x3f86_0a93);
        let t = add_rot(f(0x3e86_0a93), round);
        approach_angle(t, f(0x3d56_7752), &mut m.rot[2], 0);
        let off = sub_rot(m.rot[2], t);
        if most < off {
            m.rot[2] = sub_rot(m.rot[2], most);
        } else if off < -most {
            m.rot[2] = add_rot(m.rot[2], most);
        }
    }
    let turned = m.pv_f(TURNED) + rot_dist(round, angle);
    m.set_pv_f(TURNED, turned);
    let a = (turned / (m.pv_f(TURNS) * f(TAU))).clamp(0.0, 1.0);
    m.set_pv_f(AMOUNT, a);
    // `0x32e1e0(0x189f10, 0xa0)`: his velocities cleared.
    (h.vel, h.disp, h.eff, h.eff_v, h.eff_h, h.carry, h.momentum) = ([0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3]);
    (h.carry_yaw, h.eff_len, h.eff_len_xy, h.fwd_speed, h.slope_ratio, h.gravity.push_step) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
}
