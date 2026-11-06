//! Giant Clank (`0x18c0d4` = 2): the body the orbit levels (L22 Dobbo's, L23 Damosel's) put the player in, on the
//! body switch (crate::hero::body). Its states live only in the hero code those two levels carry (L22 addresses; L23
//! has the same code at its own places): SetState `0x2c9b20` (L23 `0x2ce200`), the physics `0x2bf2b8` (jump table
//! `0x2a4360`; L23 `0x2c39b8`), the transitions `0x2d0f48` (L23 `0x2d5628`) with the hit intake `0x2b7550`, and the
//! body's tick `0x2aaa30` (L23 `0x2af130`). Level 0's code has none of them.
//!
//! **The states.** Idle 0x56 (sequences 0, its fidgets 1 / 2), walk 0x57 (3), fall 0x58 (14 high up, else 0), the
//! hurts 0x59 / 0x5a (6; 0x5a from class 0x11df's hits), the jump 0x5b (the jump group's, crate::hero::jump, with
//! its own transitions here; sequence 4), the punches 0x5d (9, 10, 11 in a chain of three), the special 0x5e (0x10)
//! and the death 0x5f (7).
//!
//! **His hits.** Everything he does is a hit (`0x2a8c28` = L00 `0x31d180`, crate::moby::ai::attack_sphere, push 1,
//! words 1 / 0): the walk's three spheres ahead (r 2.7 at 1.8 ahead, ±45°, 1.4 up; 2, flags 0x4000000), the fall's
//! under him (r 3.5, 2, 0x30000; landing again at 1 below in world z), the jump's while it comes down (r 3.7, 100,
//! 0x90000, 1.5 below), the knock-back's ahead of his flight (r 5, 100, 0xb0000), a moby under his feet (r 4 at 2.5
//! up, 100, 0x80000); the punches (`0x2e7708` = L00 `0x2e14b0`, 50 with flags 0xb0000, query 0x10, the word
//! `class << 16 | 0x107`) at his right hand (joint 3, frames 7..14), his left (4), and both (between them, frames
//! 14..21) for the third.
//!
//! **His health** is `0x18b694` (1200 at the switch, `0x2b7200`): the hits with flag 2 take their damage off it; a
//! hit with flag 4, or none left, knocks him back (0x59, the hit's push × 0.095); out of health the fall's landing
//! and the knock-back's end are the death 0x5f. Ratchet's own health stays as it is.
//!
//! **His weapons' counts** are the level's record 1 at `0x1b1890` (L22 `0x1b18a0`, L23 `0x1b1f20`; the hero's
//! `0x18b6a8` points at it): `+8` the gun's, `+0xc` the specials'. The level fills them (the pickups, L22 `0x3a42e8`);
//! the special (△) takes one.
//!
//! Not ported: the gun (○ held, `0x2aa678`: the effect slot `0x310bc8` and two shots a tick from `0x39e1f0`, one off
//! `+8` each) and the special's object (class 0x5f3, `0x3ba568` at joint 4 in 0x5e's first 22 frames and from state
//! 1 with △, `0x3ba428`; its end `0x3ba798` at a knock-back); the pickups that fill the counts (`0x3a3d50` /
//! `0x3a42e8`); the body's lean (the posture records `0x25de10 + 0x12f0..0x149c` from the walk's yaw velocity and
//! in the jump); the strafe's camera lean in the body tick; the per-hit playback rate of 0x59 (`0x25fba0`); the
//! hurt's sound bank. The level ELFs' variants of the L00 helpers (the edge brake `0x2be9e0`, the wall check
//! `0x2b8f18`) are the port's L00 ones, with their body branches.

use super::physics::*;
use super::states::Entry;
use super::Hero;
use crate::hero::anim::AnimCtl;
use crate::moby::hits::HitTemplate;
use crate::moby::{Ctx, MobyWorld};
use crate::pad::button;

/// `0x2b7c70` for body 2: the capsule's top, bottom and radius.
pub const CAPSULE: (f32, f32, f32) = (5.25, 4.45, 3.75);
/// `0x2b7200`: his health at the switch (`0x18b694`).
pub const START_HP: i32 = 0x4b0;
/// `0x5627.9248`: the hit direction's "no limit" w.
const NO_LIMIT: f32 = f32::from_bits(0x45af_df66);

/// A hit his states deal this tick, delivered after the hero's tick ([`pass`]).
#[derive(Clone, Copy, Debug)]
pub enum GiantHit {
    /// `0x2a8c28(moby, centre, flags, 0, r, damage, 1, 1, 0)` (crate::moby::ai::attack_sphere).
    Sphere { centre: V3, r: f32, damage: f32, flags: u32 },
    /// The punch: `0x2e7708(2.7, centre, 0x10, moby, t)` with `t` = 50, flags 0xb0000, along `dir`, at joint `a`
    /// (both: between `a` and `b`).
    Punch { a: usize, b: Option<usize>, dir: V3 },
}

/// Giant Clank's block.
#[derive(Clone, Debug, Default)]
pub struct GiantBlock {
    /// `0x18b694`: his health.
    pub hp: i32,
    /// `0x18a002` (s16): the ticks since the last punch began (counted to 10000 by the body tick); `0x18aa1c`: the
    /// punch in the chain (0..2).
    pub punch_gap: i16,
    pub combo: i16,
    /// The level's record for him (`0x1b18a0` on L22): `[2]` the gun's count, `[3]` the specials'.
    pub counts: [i32; 4],
    /// This tick's hits and camera shake (`0x1b9c20` on L22: the amplitude, its ticks).
    pub hits: Vec<GiantHit>,
    pub shake: Option<(f32, i32)>,
}

impl Hero {
    /// `0x2c9b20` for 0x56..0x5f (0x5b is the jump group's).
    pub(super) fn giant_entry(&mut self, anim: &mut dyn AnimCtl, _env: &Env, id: i32, play: bool) -> Entry {
        self.cam_mode = 0;
        match id {
            0x56 => {
                self.group = 0;
                Entry::Anim(15.0, 0, 0)
            }
            0x57 => {
                self.group = 1;
                self.speed = len2(self.eff);
                self.run_hold = 0;
                Entry::Anim(8.0, 3, 0x12)
            }
            0x58 => {
                self.group = 2;
                (self.fall_g, self.fall_dec) = (0.006_111_111_5, 0.001_944_444_5);
                self.vel = self.eff;
                if 2.5 < self.height { Entry::Anim(8.0, 14, 0) } else { Entry::Anim(11.0, 0, 0) }
            }
            0x59 | 0x5a => {
                self.group = 7;
                self.vel = [0.0; 3];
                self.disp = [0.0; 3];
                self.damage.invincible = 0x3c;
                if play {
                    self.set_anim(anim, -3.0, 6, 3);
                    self.damage.hurts += 1;
                }
                Entry::Done
            }
            0x5d => {
                self.group = 6;
                let g = &mut self.giant;
                g.combo = if g.punch_gap < 0xaa { (g.combo + 1) % 3 } else { 0 };
                let blend = if self.prev_state == 0x5d { 13.0 } else { 7.0 };
                self.giant.punch_gap = 0;
                Entry::Anim(blend, self.giant.combo as u8 + 9, 1)
            }
            0x5e => {
                self.group = 6;
                Entry::Anim(8.0, 0x10, 10)
            }
            _ => {
                // 0x5f.
                self.group = 0x14;
                self.momentum = self.eff;
                Entry::Anim(12.0, 7, 0)
            }
        }
    }

    /// The end of his ground physics: in the air the effective vertical speed less 1/144 (and the edge slide when
    /// `slide`), on the ground 0.015 down.
    fn giant_gravity(&mut self, slide: bool) {
        if self.air_ticks != 0 {
            self.gravity_from(self.eff_v, 0.006_944_444_5);
            if slide { self.edge_slide(); }
        } else {
            self.gravity(0.015);
        }
    }

    /// A sphere hit at `centre`.
    fn giant_hit(&mut self, centre: V3, r: f32, damage: f32, flags: u32) { self.giant.hits.push(GiantHit::Sphere { centre, r, damage, flags }); }

    /// `0x2bf2b8` for 0x56..0x5f.
    pub(super) fn phys_giant(&mut self, env: &Env, anim: &mut dyn AnimCtl) {
        let v = anim.view();
        match self.state {
            0x56 | 0x5f => {
                // `0x2c78ac`.
                self.edge_brake(env, 3.7, 0.0);
                self.stick_target(env, 1.0, 0);
                self.target_speed = 0.0;
                self.speed_step(0.0, 0.004_916_667);
                self.vel = [0.0; 3];
                self.momentum_decay(0.004_916_667);
                self.wall_check(env, false);
                self.giant_gravity(true);
            }
            0x5d => {
                // `0x2c793c`.
                self.stick_target(env, 1.0, 0);
                self.turn_to(0.02, 0.2, 0.031_997_707, 0);
                self.target_speed = 0.0;
                self.vel = [0.0; 3];
                self.momentum_decay(0.004_916_667);
                let combo = self.giant.combo;
                if !v.blending() {
                    let reach = if combo == 2 { 5.5 } else { 1.5 };
                    let dir = self.local_offset(reach, 0.0, 0.0);
                    let f = v.frame;
                    match combo {
                        0 | 1 if 7.0 < f && f < 14.0 => self.giant.hits.push(GiantHit::Punch { a: 3 + combo as usize, b: None, dir }),
                        2 if 14.0 < f && f < 21.0 => self.giant.hits.push(GiantHit::Punch { a: 3, b: Some(4), dir }),
                        _ => {}
                    }
                }
                match combo {
                    0 | 1 => {
                        if !v.blending() && 7.0 <= v.frame && v.frame < 15.7 { self.target_speed = 0.400_000_07; }
                        self.speed_step(0.05, 0.038_888_89);
                        self.set_planar_vel(99_999.0);
                    }
                    2 => {
                        if !v.blending() && 11.0 <= v.frame && v.frame < 20.0 { self.target_speed = 0.5; }
                        self.speed_step(0.041_666_668, 0.033_333_335);
                        self.set_planar_vel(99_999.0);
                    }
                    _ => {}
                }
                self.wall_check(env, true);
                self.giant_gravity(false);
            }
            0x5e => {
                // `0x2c7d48`: still; the special's object at his left hand in its first 22 frames (not ported).
                self.target_speed = 0.0;
                self.speed_step(0.0, 0.004_916_667);
                self.vel = [0.0; 3];
                self.momentum_decay(0.004_916_667);
                if !v.blending() && v.frame < 55.0 { self.anim_speed = 1.0; }
                self.giant_gravity(true);
            }
            0x58 => {
                // `0x2c7e1c`.
                self.stick_target(env, 0.216_666_68, 0);
                let under = self.local_point(0.0, 0.0, -1.0);
                self.giant_hit(under, 3.5, 2.0, 0x30000);
                if self.land_lag == 0 { self.air_control(env); }
                self.gravity(0.010_277_778);
                if self.local_z(self.vel) < -0.950_000_05 { self.vel = self.with_local_z(self.vel, -0.950_000_05); }
            }
            0x57 => self.phys_giant_walk(env),
            0x59 | 0x5a => {
                // `0x2c8344`: the flight's hit ahead of him, the facing against it, the slow-down.
                let ahead = add(add(set_len(self.vel, 3.5), scale(self.gravity.moby_rows[2], 2.0)), self.pos);
                self.giant_hit(ahead, 5.0, 100.0, 0xb0000);
                let mut f = self.flat_len(self.vel);
                if 1.0 / 12.0 < f {
                    self.stick_dir = scale(self.vel, -1.0);
                    self.turn_to(0.05, 0.2, 12.833_334, 0);
                }
                if self.air_ticks != 0 { approach(0.0, 0.019_444_445, &mut f); } else { f *= 0.9; }
                let z = self.local_z(self.vel);
                self.vel = self.with_flat_len(self.vel, f);
                let z = (z - 0.015_277_778).min(0.183_333_34);
                self.vel = self.with_local_z(self.vel, z);
            }
            _ => {}
        }
    }

    /// `0x2c7ee0`: the walk.
    fn phys_giant_walk(&mut self, env: &Env) {
        for i in 0..3 {
            let a = add_rot(-std::f32::consts::FRAC_PI_4, i as f32 * std::f32::consts::FRAC_PI_4);
            let (s, c) = a.sin_cos();
            let p = self.local_point(c * 1.8, s * 1.8, 1.4);
            self.giant_hit(p, 2.7, 2.0, 0x4000000);
        }
        self.stick_target(env, 0.216_666_68, 0);
        let mut max = 0.078_539_83;
        if self.timer < 3 {
            self.run_hold = 1;
            if 1.745_329_4 < self.yaw_residual.abs() && 0.8 < self.stick_mag { self.quick_turn = 1; }
        }
        let (a0, a2) = (stick_angle_ago(env.pad, 0), stick_angle_ago(env.pad, 2));
        if 2.617_994 < rot_dist(a0, a2) && 1.221_730_6 < self.yaw_residual.abs() && 0.8 < self.stick_mag {
            self.run_hold = 1;
            self.quick_turn = 1;
        }
        if self.yaw_residual.abs() < 0.087_266_47 { self.run_hold = 0; }
        if self.quick_turn != 0 {
            if self.yaw_residual.abs() < 0.069_813_18 { self.quick_turn = 0; } else { max = 0.107_628_65; }
        }
        if self.strafe != 0 {
            self.strafe_turn(env);
        } else {
            self.strafe_dir = self.rows[0];
            self.strafe_yaw = self.rot[2];
            self.turn_to(0.0275, 0.3, max, 0);
        }
        self.speed_step(0.010_277_778, 0.005_555_555_7);
        if self.run_hold != 0 || self.strafe != 0 {
            let d = if self.strafe != 0 { self.strafe_dir } else { self.stick_dir };
            self.vel_along(d);
        } else {
            self.set_planar_vel(99_999.0);
        }
        self.wall_check(env, true);
        self.giant_gravity(true);
    }

    /// `0x2d36e0(out, x, y, z, 0)`: an offset through his rows.
    fn local_offset(&self, x: f32, y: f32, z: f32) -> V3 { sub(self.local_point(x, y, z), self.pos) }

    /// `0x2b7550(1)` for body 2: a hit with flag 2 outside the deaths and hurts takes its damage off his health; with
    /// flag 4 or none left the knock-back (0x5a from class 0x11df, else 0x59) along the hit's push or away from its
    /// source × 0.095. Returns whether the hit was taken (the transitions end there).
    fn giant_hit_intake(&mut self, env: &Env, anim: &mut dyn AnimCtl) -> bool {
        if matches!(self.group, 0x14 | 7) || self.state == 0x31 || self.damage.invincible != 0 { return false; }
        self.damage.hit_by = None;
        let Some(h) = env.hit else { return false };
        if h.rec.tmpl.flags & 2 == 0 { return false; }
        self.damage.took = true;
        self.damage.hit_by = h.rec.tmpl.source;
        self.giant.hp = (self.giant.hp - h.rec.tmpl.damage as i32).max(0);
        let t = h.rec.tmpl;
        let dir = match (t.count & 1 != 0, h.source_pos) {
            (true, _) => [t.dir[0], t.dir[1], t.dir[2]],
            (false, Some(p)) => sub(self.pos, p),
            (false, None) => self.local_offset(0.0, -1.0, 0.0),
        };
        let push = if t.flags & 4 != 0 || self.giant.hp <= 0 {
            self.set_state(anim, env, if h.source_class == 0x11df { 0x5a } else { 0x59 }, true);
            scale(dir, 0.095)
        } else {
            [0.0; 3]
        };
        self.vel = add(self.vel, push);
        self.disp = add(self.disp, push);
        true
    }

    /// `0x2d0f48` while the body plays: the hit intake, a moby under his feet crushed (the body tick `0x2aaa30`), the
    /// state's transitions, then the punch gap counted.
    pub(super) fn giant_transitions(&mut self, env: &Env, anim: &mut dyn AnimCtl) {
        if self.ground_moby.is_some() {
            let p = add(self.pos, scale(self.gravity.moby_rows[2], 2.5));
            self.giant_hit(p, 4.0, 100.0, 0x80000);
        }
        if !self.giant_hit_intake(env, anim) {
            let next = if self.state == 1 { self.trans_ground(env, anim) } else { self.trans_giant(env, anim) };
            if let Some(n) = next { self.set_state(anim, env, n, true); }
        }
        if self.giant.punch_gap < 10000 { self.giant.punch_gap += 1; }
    }

    /// The special (△ within 8): one off the counts, 0x5e.
    fn giant_special(&mut self, env: &Env) -> Option<Option<i32>> {
        if env.pad.pressed_within(button::TRIANGLE, 8).is_none() { return None; }
        if self.giant.counts[3] == 0 { return Some(None); }
        self.giant.counts[3] -= 1;
        Some(Some(0x5e))
    }

    fn trans_giant(&mut self, env: &Env, anim: &mut dyn AnimCtl) -> Option<i32> {
        let v = anim.view();
        let pressed = |b: u32, n: i32| env.pad.pressed_within(b, n).is_some();
        match self.state {
            0x56 => {
                if self.land_lag != 0 { return None; }
                if v.flags & 2 != 0 {
                    if v.seq_b != 0 {
                        self.set_anim(anim, 12.0, 0, 0);
                    } else if 0x33 <= self.timer {
                        let r = self.rng.randf(0.0, 1.0);
                        if r < 0.1 { self.set_anim(anim, 8.0, 1, 0) } else if r < 0.5 { self.set_anim(anim, 8.0, 2, 0) }
                    }
                }
                if env.pad.held & button::L1 != 0 { return Some(1); }
                if pressed(button::CROSS, 0xf) { return Some(0x5b); }
                if pressed(button::SQUARE, 0xf) { return Some(0x5d); }
                // `0x3ba868`: the special's object not out (none is: not ported).
                if let Some(s) = self.giant_special(env) { return s; }
                if self.air_ticks != 0 && 2.0 < self.height { return Some(0x58); }
                self.stick_target(env, 1.0, 0);
                (0.22 < self.stick_mag && 0.0 < self.target_speed).then_some(0x57)
            }
            0x57 => {
                self.anim_speed = (self.eff_len * 8.0).max(0.75);
                if pressed(button::CROSS, 0xf) { return Some(0x5b); }
                if pressed(button::SQUARE, 8) { return Some(0x5d); }
                if let Some(s) = self.giant_special(env) { return s; }
                if 5 <= self.air_ticks && 1.5 < self.height && (1.5 < self.height || 0.872_664_7 < self.slope) { return Some(0x58); }
                if self.air_ticks != 0 { return None; }
                (self.stick_mag < 0.17 && 0x15 <= self.timer).then_some(0x56)
            }
            0x58 => {
                if self.jump.descending != 0 {
                    let p = [self.pos[0], self.pos[1], self.pos[2] - 1.0];
                    self.giant_hit(p, 3.5, 2.0, 0x30000);
                }
                if self.stuck != 0 && Self::cross_within(env, 10) { return Some(0x5b); }
                if self.air_ticks != 0 { return None; }
                self.disp[2] = self.disp[2].max(-0.15);
                self.vel[2] = self.vel[2].max(-0.15);
                self.giant.shake = Some((0.15, 0x1e));
                if self.giant.hp <= 0 { return Some(0x5f); }
                Some(if 0.5 < self.stick_mag && self.land_lag == 0 { 0x57 } else { 0x56 })
            }
            0x5b => self.trans_giant_jump(env, anim),
            0x5d => {
                let lim = if v.seq_b == 0xb { 25.5 } else { 15.0 };
                if !v.blending() && lim < v.frame && 0x15 <= self.timer && pressed(button::SQUARE, 0x11) { self.set_state(anim, env, 0x5d, true); }
                let v = anim.view();
                if (28.5 < v.frame && !v.blending()) || v.flags & 2 != 0 {
                    self.set_state(anim, env, 0x56, false);
                    return None;
                }
                if 19.0 < v.frame && !v.blending() && 0.75 < self.stick_mag {
                    self.set_state(anim, env, 0x57, false);
                    self.set_anim(anim, 11.0, 3, 0x12);
                }
                None
            }
            0x5e => {
                if v.flags & 2 != 0 { return Some(0x56); }
                if v.blending() { return None; }
                (32.0 < v.frame).then_some(0x56)
            }
            0x59 | 0x5a => {
                if self.grounded_ticks != 0 && 0x33 <= self.timer { return Some(0x56); }
                if 0x29 <= self.timer || v.flags & 2 != 0 {
                    if self.giant.hp <= 0 { return Some(0x5f); }
                    self.set_anim(anim, 11.0, 0xe, 0);
                }
                None
            }
            _ => {
                // 0x5f: the death at the sequence's end.
                if v.flags & 2 != 0 { self.die(); }
                None
            }
        }
    }

    /// The decider's case 0x5b: his jump (the jump group's entry and physics). Coming down, the hit under him; rising,
    /// the playback timed to the hold frame at the landing; the touchdown (the camera's shake, the landing frame + 3);
    /// descending, the apex frame timed to the landing (at most 90 ticks); a re-jump with ✕ within 4 after landing;
    /// 11 ticks down: idle at the sequence's end or after 15, else the walk at speed toward the stick.
    fn trans_giant_jump(&mut self, env: &Env, anim: &mut dyn AnimCtl) -> Option<i32> {
        if self.jump.descending != 0 {
            let p = [self.pos[0], self.pos[1], self.pos[2] - 1.5];
            self.giant_hit(p, 3.7, 100.0, 0x90000);
        }
        let j = self.jump;
        if self.timer <= j.takeoff { return None; }
        let v = anim.view();
        // `0x180`: the effective velocity's local z.
        if j.takeoff < self.timer && 0.050_000_004 < self.local_z(self.eff) {
            let eta = self.land_eta(j.g);
            self.anim_speed_for_ticks(&v, j.f_hold, eta, j.f334, -1.0);
            return None;
        }
        if j.landed != 0 {
            self.jump.landed = j.landed.saturating_add(1);
        } else if self.air_ticks == 0 && j.descending != 0 {
            self.jump.landed = 1;
            self.giant.shake = Some((0.15, 0x1e));
            if v.frame < j.f_apex { self.set_anim(anim, 6.0, v.seq_b, j.f_land as i32 + 3); }
        }
        if self.jump.landed != 0 {
            self.anim_speed = 1.0;
        } else if j.descending != 0 {
            let v = anim.view();
            if (v.frame_b as i32) < j.f_apex as i32 {
                let eta = self.land_eta(j.g).min(90.0) as i16;
                self.jump.land_eta = eta;
                self.anim_speed_for_ticks(&v, j.f_apex, eta as f32, j.f334, -1.0);
                if 1.8 < self.anim_speed { self.set_anim(anim, eta as f32 + 2.0, v.seq_b, j.f_land as i32 + 1); }
                self.anim_speed = self.anim_speed.clamp(0.2, 2.7);
            }
            if anim.view().frame.round() == j.f_apex.round() { self.anim_speed = 0.0; }
        }
        let landed = self.jump.landed;
        if landed != 0 && self.land_lag == 0 && Self::cross_within(env, 4) && self.air_ticks < 0xf {
            self.stick_target(env, 1.0, 0);
            return Some(0x5b);
        }
        if 0xb <= landed {
            self.cam_mode = 0;
            self.stick_target(env, 1.0, 0);
            if anim.view().flags & 2 != 0 || 0xf <= landed { return Some(0x56); }
            if super::jump::RUN_JUMP_SPEED < self.target_speed && 0.5 < self.stick_mag && self.land_lag == 0 && rot_dist(self.target_yaw, self.rot[2]) < PI / 2.0 {
                return Some(0x57);
            }
        }
        None
    }
}

/// After the hero's tick while Giant Clank plays: his hits of the tick and the camera's shake.
pub fn pass(world: &mut MobyWorld, ctx: &mut Ctx) {
    if let Some((amp, ticks)) = ctx.hero.giant.shake.take() { ctx.game.shakes.ask(0, amp, ticks); }
    let hits = std::mem::take(&mut ctx.hero.giant.hits);
    let Some(hm) = ctx.hero_moby.filter(|&i| i < world.mobys.len()) else { return };
    for h in hits {
        match h {
            GiantHit::Sphere { centre, r, damage, flags } => {
                crate::moby::ai::attack_sphere(world, hm, ctx, r, damage, 1.0, centre, flags, 1, 0, 0);
            }
            GiantHit::Punch { a, b, dir } => {
                let m = &world.mobys[hm];
                let pa = ctx.host.joint(m, a);
                let c = match (pa, b.and_then(|b| ctx.host.joint(m, b))) {
                    (Some(pa), Some(pb)) => scale(add(pa, pb), 0.5),
                    (Some(pa), None) if b.is_none() => pa,
                    _ => continue,
                };
                let mut t = HitTemplate::new(50.0, Some(hm), 0xb0000, [dir[0], dir[1], dir[2], NO_LIMIT]);
                t.kind = (m.o_class as u16 as u32) << 16 | 0x107;
                world.sphere_hit(ctx.scene, c, 2.7, 0x10, Some(hm), Some(&t));
            }
        }
    }
}
