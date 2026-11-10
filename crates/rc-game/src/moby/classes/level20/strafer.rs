//! Yeedil's strafing gunship **0x12f8** (`0x39f730`, 2 placed; the decomp's `func_L20_0039E368`, its head and tail
//! `func_L20_0039E5F0` / `0039E770` at our `0x39f9b8` / `0x39fb38`): a gunship (its set-up makes it class 0x0cbc,
//! crate::moby::classes::gunship's model) that slides to and fro along a cuboid's x axis, turning round at
//! each end, its head and gun aimed at its target, firing bolts (0x05de, crate::moby::classes::gunship_bolt) in
//! bursts. Health `+0x20` (30 by default), the bolts' damage `+0x34`, target block `+0x50` (`+0x60` the moby), the
//! search's area `+0x80`, the cuboid `+0x84`, its joint aims `+0x90` / `+0x110` (lists 0 and 1:
//! crate::moby::joint_aim), `+0x190` the way it slides (±1), `+0x194` / `+0x198` its place along the cuboid (−1..1) and
//! its speed, `+0x19c` its turn speed.
//!
//! **Each tick** ([`update_12f8`]): the head (`0x39f9b8`): in state 0 the set-up (health 30 when none; class 0x0cbc,
//! `0x331cd8`; draw distance 0xff, mode not 4; its scale the class's × 1.5 (`$gp−0x338`); sliding +1; hidden and not a
//! target, mode `& 0xefff | 0x41`; state 1); the search (`0x340018` = L00 `0x32ad38`, its area); a hit with flags
//! 0x10000 (`0x332e28`) takes its damage (`0x332e88`, crate::moby::ai::damage_of): none left, state 5; 5 or more at
//! once, state 4; its hit let go.
//!
//! **States** (`+0x20`): **1** waiting: nothing more. **2** idle (on its first tick sequence 0, blend 10); drawn for
//! more than 180 ticks, 3. **3** firing (sequence 1, blend 15): on its slice of 16 frames (the original's address
//! `>> 8`; the port's slot index) a bolt from its joint 2 along that joint's first row at 20 a second (`$gp−0x334`),
//! its damage, range 40 (`$gp−0x330`), push 1, kind 1 / 1, size 1; not drawn, or after 180 ticks, 2. **4** flinching
//! (sequence 2, blend 10): at its end 2. **5** dead: the big explosion facing the camera (crate::moby::classes::explosion,
//! `0x421d88` = L00 `0x3fabe8`), its bolts (`0x32fd10`, limit 3) and gone.
//!
//! **The tail** (`0x39fb38`): its ticks (up to 65000); the head turned to its target (`+0xf8`) and the gun tilted
//! from 2 above it (`+0x174`), each aimed (`0x339480` = L00 `0x324208`: 0.03, 0.3); the slide (`0x39fdc0`): facing the
//! cuboid's yaw (`+0x78`; sliding −1, the other way) within 1°, else turned to it (`0x334370` = L00 `0x31f230`: 0.05° a
//! tick², 3° a tick); faced, its place eased toward the end it slides to (`0x333eb8` = L00 `0x31ed78`: 0.1 / 3600, 0.3
//! / 60, `$gp−0x328` / `−0x32c`), at the end the other way; at `x · row 0 + row 3` of the cuboid.
//!
//! Not ported: in a scene (`0x1a8f00` = 2) its deletion once the loaded part (`0x1bdc70`) is 5 or more (as
//! crate::moby::classes::level20::fighter's); nothing the port has takes it out of state 1.

use crate::hero::physics::{add, add_rot, atan2, dist2, ease, rot_dist, scale, set_len, sub, sub_rot, turn_ease};
use crate::moby::{ai, bolts, joint_aim, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x12f8;
/// What its set-up makes it.
const AS: i16 = 0x0cbc;

const HEALTH: usize = 0x20;
const MOST: usize = 0x24;
const DAMAGE: usize = 0x34;
const T_POINT: usize = 0x50;
const T_MOBY: usize = 0x60;
const AREA: usize = 0x80;
const CUBOID: usize = 0x84;
const AIM_TURN: usize = 0x90;
const AIM_TILT: usize = 0x110;
const WAY: usize = 0x190;
const ALONG: usize = 0x194;
const ALONG_V: usize = 0x198;
const TURN_V: usize = 0x19c;
const PVARS: usize = 0x1a0;

/// `$gp−0x338..−0x328`: its scale's factor, the bolts' speed a second and range, the slide's most (/60) and accel
/// (/3600).
const SCALE: f32 = 1.5;
const SHOT_SPEED: f32 = 20.0;
const RANGE: f32 = 40.0;
const SLIDE_MOST: f32 = f32::from_bits(0x3e99_999a);
const SLIDE_ACCEL: f32 = f32::from_bits(0x3dcc_cccd);

/// 0x12f8 (`0x39f730`, module docs).
pub fn update_12f8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    head(w, id, ctx);
    match w.mobys[id].state {
        1 => return,
        2 => {
            if w.mobys[id].first() && !shows(w, id, ctx, 0) { ctx.host.set_anim(&w.mobys[id], 0, 0, 0xa); }
            let m = &mut w.mobys[id];
            if m.drawn != 0 && 0xb5 <= m.state_ticks as u16 { m.set_state(3, -1); }
        }
        3 => {
            if w.mobys[id].first() && !shows(w, id, ctx, 1) { ctx.host.set_anim(&w.mobys[id], 1, 0, 0xf); }
            if ctx.game.frame & 0xf == id as i32 & 0xf {
                let m = &w.mobys[id];
                if let (Some(p), Some((rows, _))) = (ctx.host.joint(m, 2), ctx.host.joint_matrix(m, 2)) {
                    let vel = set_len(rows[0], SHOT_SPEED * f32::from_bits(0x3c88_8889));
                    let damage = m.pv_b(DAMAGE) as f32;
                    crate::moby::classes::gunship_bolt::fire(w, ctx, p, id, damage, RANGE, 1.0, vel, 1, 1, 1.0);
                }
            }
            let m = &mut w.mobys[id];
            if m.drawn == 0 || 0xb5 <= m.state_ticks as u16 { m.set_state(2, -1); }
        }
        4 => {
            if w.mobys[id].first() && !shows(w, id, ctx, 2) { ctx.host.set_anim(&w.mobys[id], 2, 0, 0xa); }
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.flags & 2 != 0) { w.mobys[id].set_state(2, -1); }
        }
        5 => {
            let pos = w.mobys[id].pos;
            let dir = set_len(sub(ctx.host.camera_pos(), pos), 1.0);
            crate::moby::classes::explosion::start(w, ctx, 0.0, pos, dir);
            bolts::drop(w, id, 0, 3, ctx);
            w.delete(id);
            return;
        }
        _ => {}
    }
    tail(w, id, ctx);
    w.changed = true;
}

/// `+0x43`: the sequence it shows is `s`.
fn shows(w: &MobyWorld, id: usize, ctx: &Ctx, s: u8) -> bool { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_b) == s }

/// `0x39f9b8` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        if m.pv_f(HEALTH) == 0.0 {
            m.set_pv_s(MOST, 0x1e);
            m.set_pv_f(HEALTH, 30.0);
        }
        // `0x331cd8(m, 0x0cbc)`: its class, slot, scale, collision and glow made the gunship's.
        if let Some(c) = w.class(AS).cloned() {
            let m = &mut w.mobys[id];
            m.o_class = AS;
            m.slot = c.slot;
            m.scale = c.scale;
            m.has_collision = c.has_collision;
            if c.glow != 0 {
                m.mode |= mode::GLOW;
                m.glow = c.glow;
            }
        }
        let s = w.class(AS).map_or(w.mobys[id].scale, |c| c.scale) * SCALE;
        let m = &mut w.mobys[id];
        m.draw_dist = 0xff;
        m.mode &= !4;
        m.scale = s;
        m.set_pv_i(WAY, 1);
        m.mode = (m.mode & 0xefff) | 0x41;
        m.set_state(1, -1);
    }
    let area = w.mobys[id].pv_i(AREA);
    ai::find_target(w, id, ctx, 0.0, 0.0, area, None, false);
    if ai::link(&w.mobys[id], T_MOBY).is_none() { ai::set_link(&mut w.mobys[id], T_MOBY, ctx.hero_moby); }
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
    let dmg = ai::damage_of(w, id, hit.as_ref(), false);
    let m = &mut w.mobys[id];
    let hp = m.pv_f(HEALTH) - dmg;
    m.set_pv_f(HEALTH, hp);
    if hp <= 0.0 { m.set_state(5, -1); }
    if 5.0 <= dmg { m.set_state(4, -1); }
    w.clear_hit(id);
}

/// `0x39fb38` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) < 65000 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    let t = m.pv_v(T_POINT);
    let turn = sub_rot(atan2(t[0] - m.pos[0], t[1] - m.pos[1]), m.rot[2]);
    m.set_pv_f(AIM_TURN + joint_aim::ASK + 8, turn);
    let (k, d) = (f32::from_bits(0x3cf5_c28f), f32::from_bits(0x3e99_999a));
    joint_aim::aim(w, id, AIM_TURN, id, 0, k, d);
    let m = &mut w.mobys[id];
    let p = add(m.pos, [0.0, 0.0, 2.0]);
    let tilt = -atan2(dist2(p, t), t[2] - p[2]);
    m.set_pv_f(AIM_TILT + joint_aim::ASK + 4, tilt);
    joint_aim::aim(w, id, AIM_TILT, id, 1, k, d);
    slide(w, id, ctx);
}

/// `0x39fdc0` (module docs).
fn slide(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    let c = m.pv_i(CUBOID);
    let Some(v) = usize::try_from(c).ok().and_then(|i| ctx.level.cuboids.get(i)) else { return };
    let accel = SLIDE_ACCEL * f32::from_bits(0x3991_a2b4);
    let most = SLIDE_MOST * f32::from_bits(0x3c88_8889);
    let way = m.pv_i(WAY);
    let mut yaw = v.extra[2];
    if way < 0 { yaw = add_rot(yaw, std::f32::consts::PI); }
    if f32::from_bits(0x3c8e_fa36) < rot_dist(m.rot[2], yaw) {
        let (mut r, mut tv) = (m.rot[2], m.pv_f(TURN_V));
        turn_ease(yaw, f32::from_bits(0x3a64_c38a), f32::from_bits(0x3a64_c38a), f32::from_bits(0x3d56_7752), &mut r, &mut tv, 0);
        m.rot[2] = r;
        m.set_pv_f(TURN_V, tv);
    } else {
        let (mut x, mut xv) = (m.pv_f(ALONG), m.pv_f(ALONG_V));
        ease(way as f32, accel, accel, most, &mut x, &mut xv);
        m.set_pv_f(ALONG, x);
        m.set_pv_f(ALONG_V, xv);
        if 0.0 < x * way as f32 && way.abs() as f32 <= x.abs() { m.set_pv_i(WAY, -way); }
    }
    let x = m.pv_f(ALONG);
    let r0 = [v.matrix[0][0], v.matrix[0][1], v.matrix[0][2]];
    m.pos = add(scale(r0, x), [v.matrix[3][0], v.matrix[3][1], v.matrix[3][2]]);
}
