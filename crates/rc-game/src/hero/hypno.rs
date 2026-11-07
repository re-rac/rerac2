//! The Hypnomatic's body (`0x18c0d4` = 4): a creature the Hypnomatic took over (the biter 0x0ec0 and the gunner 0x0ee1,
//! crate::moby::classes::level18::capturable) played in Ratchet's machine on Damosel, Grelbin and Yeedil. Its states
//! live only in the hero code those levels carry (L18 addresses; L19 has the same code at its own places, L20 its own
//! copy): SetState `0x2e02e8`, the physics `0x2d7758`, the transitions `0x2e6b50` with the hit intake `0x2d02b8`, the
//! body's tick `0x2c41f0` (L19 `0x2a9d60`), the gun `0x2c4000` (L19 `0x2a9b70`, L20 `0x2b91f0`), the switch `0x2cfe70`
//! (L19 `0x2b58b0`) and the way back `0x2d0050` (L19 `0x2b5a90`).
//!
//! **The states.** Idle 0x8d (the creature's sequence 13; at the switch its sequence 15, the take-over, then 13 once
//! it has run out), walk 0x8e (14), fall 0x8f (9), the hurt 0x90 (7, from frame 3 at twice the speed), the death 0x91
//! (the creature blown up, then the way back), the mud 0x94 (9); its jump 0x92 is the jump group's (crate::hero::jump:
//! the creature's sequence 6) with its own transitions here, its bite 0x93 the melee group's entry with the biter's
//! bite here.
//!
//! **Its time.** The switch gives the body the creature's time (`+0x3be`, in its `+0x3bc` while it plays, kept here):
//! the body's tick counts it down (not while a capture runs, in a landing lag or in 0x6f) and at 0 the body has no
//! health and dies (0x91); a hit takes 180 off it (the hit intake: no other damage — the body's health is 3 and only
//! the time's end takes it). Below the level's death height (more than 2 above the ground) or the creature's own
//! (`+0x3ec`) the body dies too.
//!
//! **Its gun** (`0x2c4000`, a creature other than the biter): □ held (no capture running, no landing lag, not in
//! 0x6f) — in use, the sequence 11 (blend 5, from frame 1) at three times the speed; let go, at the sequence's end the
//! walk's 14 (blend 8) or the idle's 13 (blend 9). While the sequence 11 plays the gunner fires from its two guns in
//! turn every 6 ticks in use, the second 3 after (from the 4th tick), 0x0ed4 from one on 5 of each 6 (the shots:
//! crate::moby::classes::level18::capturable::shoot).
//!
//! **Its bite** (0x93: □ within 9 ticks, a creature other than the gunner): the biter's wait 90 (`0x429bc0`) and its
//! sequence 10 (blend 5, from frame 1); still (1/97 up, 1/129 down, its own yaw), the creature's attack
//! (`0x429c28` while taken over: a new □ press holds the wait at 90; the bite's line between its joints 2 and 3;
//! the wait out ends it, the body back to idle — here a tick later, the attack being the world's).
//!
//! **The way in** ([`Hero::hypno_switch`], `0x2cfe70(4, 0x8d, it)`): Ratchet's moby where he stands (hidden), the
//! player at the creature's place and angles, Ratchet's health kept and the body's 3, the idle 0x8d and the
//! creature's sequence 15. **The way back** (`0x2d0050(keep)`, [`Hero::hypno_leave`] from crate::hero::Hero::body_back):
//! the player at Ratchet's moby again, his health back, the time's meter off (unless `keep`: a capture from the body),
//! his motion stopped (`0x2b2cd8`), in the air for the probe (`0x18a16e` = 1).
//!
//! The world side of the body (its shots, its bite, its blast, the fade) is [`pass`], after the hero's tick.
//!
//! Not ported: the creature's gun aim (`0x4295f0` → the record `0x2571a0` the Hypnomatic's update aims, `0x3db8f8`),
//! the look stance from the idle (L1: state 1), the item triggers and the water checks of the transitions' prologue
//! (`0x2c96d0`, `0x2e42f8`, `0x2c8cb0`, `0x2c8c00`, `0x2cb218`, `0x2cb2d8`), the hurt's playback-rate table
//! (`0x2672a0`, as for Ratchet's hurts), the flags `0x18c0d5` / `0x18c2e1` / `0x18c2e2`, the `0x18a1c4` / `0x18a1c0`
//! words of the way back, the help texts of the body on L19 / L20 (`0x18c958 + 0x504`, `0x18c958 + 0x7900`,
//! `0x13965e`).

use super::physics::*;
use super::states::Entry;
use super::Hero;
use crate::hero::anim::AnimCtl;
use crate::moby::{Ctx, MobyWorld};
use crate::pad::button;

/// The creatures (crate::moby::classes::level18::capturable).
pub const BITER: i16 = 0x0ec0;
pub const GUNNER: i16 = 0x0ee1;
const GUNNER_B: i16 = 0x0ed4;
/// The body's health at the switch (`0x18c2ec` = 3).
const HP: i32 = 3;
/// What a hit takes off its time (`0x2d02b8`).
const HIT_TIME: i16 = 0xb4;

/// A request of the body's states for the world ([`pass`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HypnoReq {
    /// `0x429918(body, side)`: a shot from its gun `side`.
    Shot(usize),
    /// `0x429bc0(body)`: the biter's bite begins (its wait 90).
    BiteStart,
    /// `0x429c28(body)`: the bite's tick (the attack while taken over); its end reported in [`HypnoBlock::bite_done`].
    Bite,
    /// `0x33b658(moby, …)`: the creature blown up (0x91's entry).
    Blast(usize),
    /// `0x3024a0(n)` = L00 `0x2e7c90`: the blocking fade.
    Fade(i32),
}

/// The body's block.
#[derive(Clone, Debug, Default)]
pub struct HypnoBlock {
    /// L18 `0x1afb60` (`$gp+0xb70`; L19 `0x1b0040`, L20 `0x1aff00`): a capture's swirl runs — no creature takes a hit,
    /// the body's physics and its time wait, set by the creature hooked and cleared at the switch or its release.
    pub capturing: bool,
    /// The body's class.
    pub class: i16,
    /// The creature's `+0x3bc` / `+0x3be`: its time left and its whole, kept here while the body plays.
    pub time: i16,
    pub time_full: i16,
    /// The creature's `+0x3ec`: the height under which the body dies.
    pub kill_z: f32,
    /// Ratchet's moby's place and angles while the body plays (`0x18c0b0 + 0x10` / `+ 0xf0`): where the way back
    /// puts him.
    pub ratchet_pos: V3,
    pub ratchet_rot: V3,
    /// `0x327fe8(0, 0x427fa0)`: the time's meter is the HUD's callback (cleared by `0x327fe8(0, 0)`).
    pub meter: bool,
    /// The bite's end as the last [`pass`] found it.
    pub bite_done: bool,
    /// The requests of this tick.
    pub reqs: Vec<HypnoReq>,
}

impl Hero {
    /// `0x2cfe70(4, 0x8d, moby)` (module docs): the player becomes the creature `moby` at `pos` / `rot`, of class
    /// `class`, with its time `time` and its death height `kill_z`.
    pub fn hypno_switch(&mut self, moby: usize, pos: V3, rot: V3, class: i16, time: i16, kill_z: f32) {
        self.hypno.ratchet_pos = self.pos;
        self.hypno.ratchet_rot = self.rot;
        self.switch_body(4, 0x8d, moby);
        self.pos = pos;
        self.rot = rot;
        self.rows = euler_rows(rot);
        // `0x18c2f4` keeps Ratchet's health; the body's is 3.
        self.body.ratchet_hp = self.hp as i16;
        self.hp = HP;
        let h = &mut self.hypno;
        h.class = class;
        h.time = time;
        h.time_full = time;
        h.kill_z = kill_z;
        h.bite_done = false;
        // `0x2d2508(15, 0, 5)` after the state: the take-over.
        self.anim_req = Some((5.0, 15, 0));
    }

    /// `0x2d0050(keep)`'s part for the body (module docs), before the way back every body takes.
    pub fn hypno_leave(&mut self, keep: bool) {
        self.pos = self.hypno.ratchet_pos;
        self.rot = self.hypno.ratchet_rot;
        self.rows = euler_rows(self.rot);
        self.hp = self.body.ratchet_hp as i32;
        if !keep { self.hypno.meter = false; }
        self.stop_motion();
        self.air_ticks = 1;
        self.hypno.capturing = false;
    }

    /// `0x2b2cd8`: the motion stopped — the push, the velocity, the displacement, the effective velocity and its
    /// vertical part, the momentum, the tumble's velocities, the yaw velocity, the forward speed and |eff|.
    fn stop_motion(&mut self) {
        (self.push, self.vel, self.disp, self.eff, self.eff_v, self.momentum) = ([0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3], [0.0; 3]);
        self.rot_vel = [0.0; 2];
        self.yaw_vel = 0.0;
        self.fwd_speed = 0.0;
        self.eff_len = 0.0;
    }

    /// The body's physics and time wait while a capture runs or Ratchet is held (0x6f).
    fn hypno_held(&self) -> bool { self.hypno.capturing || self.state == 0x6f }

    /// `0x2e02e8` for the body's states.
    pub(super) fn hypno_entry(&mut self, anim: &mut dyn AnimCtl, env: &Env, id: i32, play: bool) -> Entry {
        self.cam_mode = 0;
        let mud = self.surf.mud && self.f16c == 0;
        match id {
            0x8d => {
                self.group = 0;
                if mud {
                    self.set_state(anim, env, 0x94, true);
                    return Entry::Return(false);
                }
                Entry::Anim(9.0, 13, 0)
            }
            0x8e => {
                if mud {
                    self.set_state(anim, env, 0x94, true);
                    return Entry::Return(false);
                }
                self.group = 1;
                self.speed = len2(self.eff);
                self.run_hold = 0;
                self.quick_turn = 0;
                if self.prev_group == 0 { self.speed = len2(self.momentum); }
                Entry::Anim(8.0, 14, 0)
            }
            0x8f => {
                self.group = 2;
                self.fall_dec = 0.001_944_444_5;
                self.vel = self.eff;
                Entry::Anim(8.0, 9, 0)
            }
            0x94 => {
                self.group = 2;
                self.vel = self.eff;
                Entry::Anim(12.0, 9, 0)
            }
            0x90 => {
                self.group = 7;
                self.scale_motion(0.5);
                self.vel = self.eff;
                // `0x33c498(vel, 1/12)`: at most 1/12 long.
                if 1.0 / 12.0 < len(self.vel) { self.vel = set_len(self.vel, 1.0 / 12.0); }
                self.damage.invincible = 50;
                if play {
                    self.set_anim(anim, -3.0, 7, 3);
                    self.anim_speed = 2.0;
                    self.damage.hurts += 1;
                }
                Entry::Done
            }
            _ => {
                // 0x91: the creature blown up once, the landing lag 45.
                self.group = 0x14;
                self.stop_motion();
                if self.land_lag == 0 && self.prev_state != 0x91 {
                    self.land_lag = 45;
                    if let Some(b) = self.body.moby { self.hypno.reqs.push(HypnoReq::Blast(b)); }
                }
                Entry::Done
            }
        }
    }

    /// The bite's entry with the body (the melee group's 0x93, then the biter's `0x429bc0` and its sequence 10).
    pub(super) fn hypno_bite_entry(&mut self, anim: &mut dyn AnimCtl) {
        if self.hypno.class != BITER { return; }
        self.hypno.reqs.push(HypnoReq::BiteStart);
        self.hypno.bite_done = false;
        self.set_anim(anim, 5.0, 10, 1);
    }

    /// The end of the body's ground physics: in the air the effective vertical speed less 1/144 and the edge slide,
    /// on the ground 0.015 down.
    fn hypno_gravity(&mut self) {
        if self.air_ticks != 0 {
            self.gravity_from(self.eff_v, 0.006_944_444_5);
            self.edge_slide();
        } else {
            self.gravity(0.015);
        }
    }

    /// `0x2d7758` for the body's states.
    pub(super) fn phys_hypno(&mut self, env: &Env, anim: &mut dyn AnimCtl) {
        match self.state {
            0x8d => {
                self.edge_brake(env, 3.7, 0.0);
                if self.hypno_held() { return; }
                self.hypno_gun(anim);
                self.stick_target(env, 1.0, 0);
                self.target_speed = 0.0;
                self.speed_step(0.0, 0.003_5);
                self.vel = [0.0; 3];
                self.wall_check(env, false);
                self.hypno_gravity();
            }
            0x91 => {
                if self.land_lag < 10 && !self.hypno_held() {
                    self.land_lag = 40;
                    self.group = 0;
                    self.cam_mode = 0;
                    self.body_back(false);
                    self.hypno.reqs.push(HypnoReq::Fade(0x10));
                }
            }
            0x93 => {
                if self.hypno_held() { return; }
                if std::mem::take(&mut self.hypno.bite_done) {
                    self.set_state(anim, env, 0x8d, true);
                    return;
                }
                self.target_speed = 0.0;
                self.speed_step(0.010_277_778, 0.007_777_778);
                self.set_planar_vel(99_999.0);
                if self.air_ticks != 0 {
                    self.vel[2] = self.disp[2] - 0.006_944_444_5;
                    self.edge_slide();
                } else {
                    self.vel[2] -= 0.015;
                }
                self.edge_brake(env, 3.7, 0.0);
                self.hypno.reqs.push(HypnoReq::Bite);
            }
            0x94 => {
                if self.hypno_held() { return; }
                self.target_yaw = self.ground_yaw;
                self.target_speed = 0.091_666_67;
                self.air_accel(0.006_944_444_5);
                self.gravity_from(self.eff_v, 0.005);
            }
            0x8f => {
                if self.hypno_held() { return; }
                self.hypno_gun(anim);
                self.stick_target(env, 0.095_833_34, 0);
                if self.land_lag == 0 { self.air_control(env); }
                self.gravity(0.003_333_333_4);
                if self.vel[2] < -1.0 / 6.0 { self.vel[2] = -1.0 / 6.0; }
            }
            0x8e => {
                if self.hypno_held() { return; }
                self.stick_target(env, 0.095_833_34, 0);
                self.hypno_gun(anim);
                if self.strafe != 0 {
                    self.strafe_turn(env);
                } else {
                    self.strafe_yaw = self.rot[2];
                    self.strafe_dir = self.rows[0];
                    let (k, max) = if self.quick_turn != 0 { (0.037, 0.136_717_48) } else { (0.033, 0.107_628_65) };
                    self.turn_to(k, 0.15, max, 0);
                }
                self.speed_step(0.005, 0.005_555_556);
                if self.eff_len_xy < 0.011_666_667 {
                    self.run_hold = 1;
                    if 1.745_329_4 < self.yaw_residual.abs() && 0.8 < self.stick_mag { self.quick_turn = 1; }
                }
                let (a0, a2) = (stick_angle_ago(env.pad, 0), stick_angle_ago(env.pad, 2));
                if 2.617_994 < rot_dist(a0, a2) && 1.221_730_6 < self.yaw_residual.abs() && 0.8 < self.stick_mag {
                    self.quick_turn = 1;
                    self.run_hold = 1;
                }
                if self.yaw_residual.abs() < 0.087_266_475 { self.run_hold = 0; }
                if self.quick_turn != 0 {
                    if self.yaw_residual.abs() < 0.069_813_18 { self.quick_turn = 0; }
                    if self.quick_turn != 0 {
                        let v = (0.095_833_34 - self.yaw_residual.abs() * 0.021_220_66).max(0.0);
                        if v < self.target_speed { self.target_speed = v; }
                    }
                }
                let d = if self.strafe != 0 { self.strafe_dir } else { self.stick_dir };
                self.vel_along(d);
                self.wall_check(env, true);
                self.hypno_gravity();
            }
            _ => {
                // 0x90.
                if self.hypno_held() { return; }
                let k = if self.air_ticks != 0 { 0.99 } else { 0.92 };
                let l = len2(self.vel) * k;
                self.vel = set_len2(self.vel, l);
                self.vel[2] = 0.0;
                if self.air_ticks != 0 { self.gravity_from(self.eff, 0.006_111_111_5) } else { self.gravity(0.004) }
            }
        }
    }

    /// `0x2c4000`: the gun (module docs).
    fn hypno_gun(&mut self, anim: &mut dyn AnimCtl) {
        let class = self.hypno.class;
        let v = anim.view();
        if class != BITER {
            if self.items.held & button::SQUARE != 0 && !self.hypno.capturing && self.land_lag == 0 && self.state != 0x6f {
                self.items.using = true;
                self.items.keep = true;
                if v.seq_b != 11 {
                    self.set_anim(anim, 5.0, 11, 1);
                    self.anim_speed = 3.0;
                }
            } else if v.flags & 2 != 0 {
                self.items.using = false;
                self.items.keep = false;
                if self.state == 0x8e { self.set_anim(anim, 8.0, 14, 0) } else { self.set_anim(anim, 9.0, 13, 0) }
            }
        }
        let v = anim.view();
        if v.seq_a == 11 || (v.seq_a == 0xff && v.seq_b == 11) {
            self.anim_speed = 3.0;
            let n = self.items.use_ticks;
            match class {
                GUNNER => if 3 < n {
                    if n % 6 == 0 { self.hypno.reqs.push(HypnoReq::Shot(0)); }
                    if (n + 3) % 6 == 0 { self.hypno.reqs.push(HypnoReq::Shot(1)); }
                },
                GUNNER_B => if n % 6 != 0 { self.hypno.reqs.push(HypnoReq::Shot(0)); },
                _ => {}
            }
        }
    }

    /// `0x2c41f0`'s part for the body, before its transitions: the time counted down (module docs); then the ticks in
    /// use (`0x2c3330`, as the item pass counts them for Ratchet).
    pub fn hypno_tick(&mut self, env: &Env, anim: &mut dyn AnimCtl) {
        if self.mode != 4 { return; }
        if !self.hypno_held() && self.land_lag == 0 {
            let mut t = self.hypno.time as i32;
            let out = crate::moby::countdown(&mut t);
            self.hypno.time = t as i16;
            if out {
                self.hp = 0;
                self.set_state(anim, env, 0x91, true);
            }
        }
        self.items.use_ticks = if self.items.using { self.items.use_ticks + 1 } else { 0 };
    }

    /// `0x2d02b8(1)` for the body: a hit on its moby outside the deaths and hurts, while not invincible and no capture
    /// runs: 180 off its time, the hurt 0x90 and the knock-back away from the source (1/12 out, 0.05 up; along the
    /// template its full length to 1/12, its up part to 0.05). Returns whether the hit was taken.
    fn hypno_hit_intake(&mut self, env: &Env, anim: &mut dyn AnimCtl) -> bool {
        if matches!(self.group, 0x14 | 7) || self.state == 0x31 || self.damage.invincible != 0 { return false; }
        self.damage.hit_by = None;
        let Some(h) = env.hit else { return false };
        if self.hypno_held() { return false; }
        self.damage.took = true;
        self.damage.hit_by = h.rec.tmpl.source;
        self.hypno.time = (self.hypno.time - HIT_TIME).max(0);
        let t = h.rec.tmpl;
        let mut dir = match (t.count & 1 != 0, h.source_pos) {
            (true, _) => [t.dir[0], t.dir[1], t.dir[2]],
            (false, Some(p)) => sub(self.pos, p),
            // `0x2d36e0(out, −1, 0, 0)`: behind him.
            (false, None) => scale(self.rows[0], -1.0),
        };
        let along = t.count & 1 != 0 && (t.dir[3] - 5627.925).abs() < 1e-3;
        self.set_state(anim, env, 0x90, true);
        // `0x2b2d60(dir, along, 1/12, 0.05)`.
        let (out, up) = (0.083_333_336, 0.050_000_004);
        if along {
            dir = set_len(dir, out);
        } else {
            dir[2] = 0.0;
            dir = set_len2(dir, out);
        }
        dir[2] = up;
        self.vel = add(self.vel, dir);
        self.disp = add(self.disp, dir);
        true
    }

    /// `0x2e6b50` while the body plays: the hit intake; below the level's death height or the creature's, the death;
    /// then the state's transitions.
    pub(super) fn hypno_transitions(&mut self, env: &Env, anim: &mut dyn AnimCtl) {
        if self.hypno_hit_intake(env, anim) { return; }
        if let Some(z) = env.death_z {
            if self.pos[2] < z && 2.0 < self.height && self.state != 0x74 {
                self.set_state(anim, env, 0x91, true);
                return;
            }
        }
        if self.pos[2] < self.hypno.kill_z { self.set_state(anim, env, 0x91, true); }
        let next = self.trans_hypno(env, anim);
        if let Some(n) = next { self.set_state(anim, env, n, true); }
    }

    /// The body's per-state transitions.
    fn trans_hypno(&mut self, env: &Env, anim: &mut dyn AnimCtl) -> Option<i32> {
        let bite = |h: &Hero, env: &Env| h.hypno.class != GUNNER && env.pad.pressed_within(button::SQUARE, 9).is_some();
        match self.state {
            0x8d => {
                let v = anim.view();
                if v.flags & 2 != 0 && v.seq_b != 13 { self.set_anim(anim, 12.0, 13, 0); }
                if self.surf.mud { return Some(0x94); }
                if self.air_ticks != 0 { return Some(0x8f); }
                if Self::cross_within(env, 8) { return Some(0x92); }
                if bite(self, env) { return Some(0x93); }
                self.stick_target(env, 1.0, 0);
                let turned = self.strafe_turned(env, 0.174_532_95);
                (0.2 < self.stick_mag || turned).then_some(0x8e)
            }
            0x93 => self.surf.mud.then_some(0x94),
            0x92 => self.trans_hypno_jump(env, anim),
            0x8f => {
                if self.air_ticks != 0 { return None; }
                if self.disp[2] < -0.15 { self.disp[2] = -0.15; }
                if self.vel[2] < -0.15 { self.vel[2] = -0.15; }
                Some(if self.hp <= 0 {
                    0x91
                } else if 0.5 < self.stick_mag && self.land_lag == 0 {
                    0x8e
                } else {
                    0x8d
                })
            }
            0x94 => {
                if 0x12d <= self.timer { self.die(); }
                if self.surf.mud { return None; }
                Some(if self.air_ticks != 0 { 0x8f } else { 0x8d })
            }
            0x8e => {
                if self.surf.mud { return Some(0x94); }
                if Self::cross_within(env, 8) { return Some(0x92); }
                if bite(self, env) { return Some(0x93); }
                if 5 <= self.air_ticks && (0.8 < self.height || 0.872_664_7 < self.slope) { return Some(0x8f); }
                (self.air_ticks == 0 && self.stick_mag < 0.17 && self.strafe == 0).then_some(0x8d)
            }
            0x90 => {
                if 0x20 <= self.timer && self.air_ticks != 0 && 1.0 < self.height { return Some(0x8f); }
                if anim.view().flags & 2 != 0 || 0x24 <= self.timer {
                    if self.hp <= 0 { return Some(0x91); }
                    self.set_state(anim, env, 0x8d, false);
                }
                None
            }
            _ => None,
        }
    }

    /// `0x2e6b50` case 0x92: the body's jump — the gun; the jump's take-off and the landing's playback (as the jump
    /// group's own, with the creature's jump sequence 6 and the factor 0.5); landed, ✕ within 4 jumps again; two ticks
    /// down the walk (at more than the run jump's speed, more than half the stick, not on mud, no landing lag, the
    /// stick within 90° of the facing) or, after 13, the idle; high up and falling from the 41st tick, the fall.
    fn trans_hypno_jump(&mut self, env: &Env, anim: &mut dyn AnimCtl) -> Option<i32> {
        self.hypno_gun(anim);
        let j = self.jump;
        if self.timer <= j.takeoff { return None; }
        let v = anim.view();
        if self.timer == j.takeoff + 1 {
            if v.seq_b == 6 {
                let eta = self.land_eta(j.g);
                self.anim_speed_for_ticks(&v, j.f_hold, eta, 0.5, -1.0);
            }
            return None;
        }
        let mut pose = true;
        if j.landed != 0 {
            self.jump.landed = j.landed.saturating_add(1);
        } else if self.air_ticks == 0 && j.descending != 0 {
            self.jump.landed = 1;
            if v.seq_b != 6 {
                pose = false;
            } else if v.frame < j.f_apex {
                self.set_anim(anim, 6.0, v.seq_b, j.f_land as i32 + 3);
            }
        }
        if pose && anim.view().seq_b == 6 {
            let v = anim.view();
            if self.jump.landed != 0 {
                self.anim_speed = 1.0;
            } else if j.descending != 0 {
                if (v.frame_b as i32) < j.f_land as i32 {
                    let eta = self.land_eta(j.g).min(60.0) as i16;
                    self.jump.land_eta = eta;
                    self.anim_speed_for_ticks(&v, j.f_apex, eta as f32, 0.5, -1.0);
                    if 1.8 < self.anim_speed { self.set_anim(anim, (eta + 2) as f32, v.seq_b, j.f_land as i32 + 1); }
                    self.anim_speed = self.anim_speed.clamp(0.2, 2.7);
                }
                if v.frame as i32 == j.f_apex as i32 { self.anim_speed = 0.0; }
            }
        }
        let landed = self.jump.landed;
        if landed != 0 && self.land_lag == 0 && Self::cross_within(env, 4) && self.air_ticks < 4 {
            self.stick_target(env, 1.0, 0);
            return Some(0x92);
        }
        if 2 <= landed {
            self.cam_mode = 0;
            self.stick_target(env, 1.0, 0);
            if super::jump::RUN_JUMP_SPEED < self.target_speed && 0.5 < self.stick_mag && !self.surf.mud && self.land_lag == 0
                && rot_dist(self.target_yaw, self.rot[2]) < std::f32::consts::FRAC_PI_2
            {
                return Some(0x8e);
            }
            if 0xd <= landed { return Some(0x8d); }
        }
        (2.0 < self.height && j.descending != 0 && 0x29 <= self.timer).then_some(0x8f)
    }
}

/// The body's world side after the hero's tick (and after the way back, from the item pass): its requests (module
/// docs); while it plays, its time into the creature's `+0x3bc` and the creature's death height read.
pub fn pass(world: &mut MobyWorld, ctx: &mut Ctx) {
    use crate::moby::classes::level18::capturable;
    let body = ctx.hero.body.moby.filter(|_| ctx.hero.mode == 4);
    for r in std::mem::take(&mut ctx.hero.hypno.reqs) {
        match r {
            HypnoReq::Fade(n) => ctx.game.blocking_fade(n),
            HypnoReq::Blast(id) => if world.mobys.get(id).is_some_and(|m| m.state < 0xfe) { capturable::release_blast(world, id, ctx) },
            HypnoReq::Shot(side) => if let Some(id) = body { capturable::shoot(world, id, ctx, side) },
            HypnoReq::BiteStart => if let Some(id) = body { capturable::bite_start(world, id) },
            HypnoReq::Bite => if let Some(id) = body { ctx.hero.hypno.bite_done = capturable::controlled_attack(world, id, ctx) },
        }
    }
    if let Some(id) = body.filter(|&id| world.mobys.get(id).is_some_and(|m| m.pvars.len() >= capturable::PVARS)) {
        let m = &mut world.mobys[id];
        m.set_pv_s(capturable::TIME, ctx.hero.hypno.time);
        ctx.hero.hypno.kill_z = m.pv_f(capturable::KILL_Z);
    }
}
