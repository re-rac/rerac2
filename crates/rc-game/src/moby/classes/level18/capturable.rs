//! The creatures the Hypnomatic can take over: the biter **0x0ec0** and the gunner **0x0ee1** (L18 `0x42a3f8`, 14 + 2
//! placed; L19 `0x40ced0`, 13 + 51; L20 `0x438588`, 20 gunners; the decomp's `func_US2_L18_0042A3F8`, the same code on
//! the three levels, which also serves a third kind 0x0ed4 no level places), and the gunners' shot **0x1263** (L18
//! `0x4416d0`, L19 `0x424148`, L20 `0x44a268`; `func_L18_004401E0`). Their helpers are named by their L18 places.
//!
//! On the framework (crate::moby::ai): health `+0x20` (its most `+0x24`, status `+0x2e`, damage byte `+0x34`),
//! reaction `+0x50` (a hurt to state 10, sub 9), movement `+0x100` (its path point `+0x1c8` / last `+0x1ca`, its
//! floor's surface `+0x1d8`), the wander block `+0x1f0` (crate::moby::classes::leaper::wander's), target `+0x220`
//! (its moby `+0x230`, found `+0x234`), the aim record `+0x240` (list 4) and the head aim `+0x2c0` (list 1).
//!
//! **Its pvars:** `+0x360` / `+0x364` its target areas idle / alerted (`+0x37c` the one searched), `+0x368` the path
//! it jumps down onto (−1 none), `+0x36c` / `+0x370` / `+0x374` / `+0x378` its paths (the first; the patrol after
//! the alert; the chase's; a reserve's way in), `+0x380` / `+0x384` its fences idle / alerted (the movement's `+0x3c`;
//! `+0x380` also the polygon it jumps again inside), `+0x388` / `+0x3a8` / `+0x3ac` words its group shares (the held
//! count; the jump's cooldown and the number jumping in, at most 3: crate::moby::MobyWorld::shared_i), `+0x38c` the
//! path it walks (−1 none), (s16) `+0x390` its patience, `+0x392` its wait, `+0x394` a count, `+0x396` its attack's
//! cooldown; `+0x398` the alert's wait, `+0x399` 1 a reserve (2 called in), `+0x39a` alerted, `+0x39b` its kind,
//! `+0x39c` hidden at the start, `+0x39d` the burst's shots left, `+0x39f` holding one of `+0x388`, `+0x3a0` how near
//! its target ends a biter's path, `+0x3a4` an area its target is safe in, `+0x3b0` one of the jumpers, `+0x3b1` its
//! lives (0x80 on: endless), `+0x3b2` / `+0x3b3` its target one of its kind before / now, `+0x3b4` the turning lift
//! 0x114d it comes back on (crate::moby::classes::level19::props::update_114d; −1 none), `+0x3c0..+0x3dc` its walk
//! and run limits, `+0x3e0` its update distance, `+0x3e8` asleep, `+0x3e9` gone once its mission is done, `+0x3ea` a
//! reserve that comes on its lift.
//!
//! **The update** (`0x42a3f8`): nothing out of the loaded variant (`0x33c8e0` = L00 `0x321a78`); the frame's checks
//! ([`head`], `0x4290d0`); then by its sub-state (the behaviour) and its state (the move); the tail ([`tail`],
//! `0x429538`).
//!
//! **Sub-states:** 1 asleep (sequence 0xd still). 2 watching: not a reserve, its target found (or its patience on)
//! and not one of its kind out of view ([`kin_in_view`]): its group's biters alerted ([`alert`]), the alerted area
//! and fence, the wait `+0x398`; with a jump path and its target not in `+0x3a4`, when the group's cooldown is out
//! and fewer than 3 jump in, the cooldown 30..90, one of them, state 1 sub 4; else onto `+0x370`, state 1 sub 3. 3
//! the alert: its wait out, sequence 3, state 3 and sub 5 (with `+0x370`) or 6 onto `+0x374` with its patience
//! 600..1000. 4 the jump: its wait out, shown, sequence 6, a jump (5/60 across, gravity 9.8/3600) to the point of
//! `+0x368` nearest Ratchet ([`nearest_on_path`]), state 5 sub 10. 5 the patrol: found, its patience 600..1000. 6 the
//! chase: not found with its patience out (or its target no longer its kind): sequence 4 / 5, state 2 sub 2; else, not
//! jumping, as 2's jump. 9 hurt: the framework's flight (`0x3433a0` = L00 `0x328b80`); dead its bolts and state 12;
//! over, its patience, its group alerted, sequence 4 / 5, state 4 sub 6. 11 on its lift: the lift down (its timer
//! byte 0), shown at full alpha and size, sub 12 (no lift: the teleport-in, crate::moby::classes::teleport, sub 14).
//! 12 riding up at the lift's place; the lift's timer byte 2: sequence 2, state 4 sub 13. 13 walking off; the lift's
//! timer byte 3: back in play ([`show`]), state 2 sub 2. 14 teleporting in: at full size, back in play; asleep (sub 1,
//! sequence 0xd) or state 2 sub 2.
//!
//! **States:** 1 turning in place to its target (`0x347ab8` = L00 `0x32ceb8`). 2 still (`0x347a98` = L00
//! `0x32ce98`); its wait out, the wait 240, sequence 2, 4. 3 turning to Ratchet (`0x33a350` = L00 `0x31f230`) until
//! its sequence ends: sequence 2, 4. 4 the walk ([`walk`], `0x429fd8`). 5 the jump's flight (`0x348d60` = L07
//! `0x324208(7, 32, 37)`): landed inside `+0x380` (`0x33c880` → L00 `0x31cf68`) the jump again (state 0 sub 4); else
//! sequence 2 and onto `+0x370` (state 4 sub 5) or `+0x374` (sub 6). 6 the attack ([`attack`], `0x429c28`). 12 its
//! death's blast (crate::moby::blast::explode: 10 embers, 3 billows, 16 flares, flashes 4 / 2, light 15, shaking,
//! its class sound 3 / 10), then as 11: out of the jumpers, its uid counted and gone (crate::game::GameState::uid_gone);
//! with lives left and its mission not done, back on its lift ([`respawn`]); else deleted.
//!
//! The engine functions with the port's: `0x3368c0` (set_state), `0x3365e0` / `0x428418` (the sequence,
//! crate::moby::classes::props::set_seq), `0x3369d0` / `0x336a28` / `0x1163b0` (the rng), `0x342658` = L00 `0x327e38`
//! (crate::moby::ai::take_damage), `0x345e10` = L00 `0x32ad38` (ai::find_target), `0x3489d0` = L00 `0x32dcf0`
//! (ai::senses), `0x3477d8` = L01 `0x322450` (ai::move_step toward a goal), `0x347830` = L00 `0x32cc30`
//! (ai::move_toward), `0x33e7c8` = L01 `0x319858` (crate::moby::classes::leaper::wander), `0x348b88` / `0x35e438` (onto
//! a path from its nearest point, [`set_path`]), `0x428580` = L09 `0x3f8f18` (the walk along it, ai::follow_path),
//! `0x348ca8` = L07 `0x324150` (ai::jump_start), `0x335db0` = L00 `0x31adc0` (crate::moby::bolts::drop), `0x338b78`
//! = L00 `0x31da58` (crate::shadow_volumes::probe_near), `0x33ef88` / `0x341e28` = L00 `0x324208` / `0x327308`
//! (crate::moby::joint_aim), `0x338158` = L00 `0x31d180` (ai::attack_sphere), `0x338348` = L01 `0x3132c8`
//! (ai::attack_line), `0x33b658` = L00 `0x320538` (crate::moby::blast::explode), `0x2fbd48` = L00 `0x2e14b0`
//! (crate::moby::MobyWorld::sphere_hit), `0x307068` = L02 `0x2f0170` (crate::moby::classes::teleport::start).
//!
//! **The capture** (the Hypnomatic's beam hooks a gunner of endless lives, crate::moby::classes::hypnomatic; the code
//! serves every kind). The head ([`head`]), not taken over and not already in sub-state 7, a hooked creature
//! (`0x345910` = L00 `0x32a838`): update distance 0xff, the time's meter off (`0x327fe8(0, 0)`), `+0x39f` let go, the
//! script camera switched in where the view is (`0x456fa8` = L00 `0x428880`, a snap; `0x456e60` / `0x456eb8` its
//! targets the same), Ratchet out of the look states 1 / 0x1d (idle) and held 20 ticks (`0x18a000`), the swirl's
//! speed `+0x3b8` = (its distance across to the view − 3) / 180, unhooked (`0x3458f0`), the swirl's ticks `+0x396` =
//! 180, sequence 16, a capture runs (`$gp+0xb70`, crate::hero::hypno::HypnoBlock::capturing), the prompt's lease 6
//! (`0x342248(6, 0, 20)`), state 0 sub 7.
//! * **Sub 7, the swirl** ([`swirl`]): the lease; its class sound 6 (of 0x0ee1's table) on the sub-state's first tick
//!   and the loop 9 (mode 4) kept playing (`+0x3f0`); its flash (`0x33b308`); turning to Ratchet; Ratchet invincible
//!   5 ticks and held 10. Its ticks out: the time's meter on (`0x327fe8(0, 0x427fa0)`), its time `+0x3bc` = `+0x3be`;
//!   from the body, the way back first (`0x2d0050(1)`); its flash off and grey (`0x428288`), taken over (timer byte
//!   1), no fence, its wait 30, the switch (`0x2cfe70(4, 0x8d, it)`: crate::hero::Hero::hypno_switch, Ratchet facing
//!   its yaw), no capture running, state 7 sub 8, the loop stopped. The camera: t = 1 − ticks/180; the point 3 from
//!   it across toward Ratchet and 2 up; the view's place drawn to it, `+0x3b8 · ticks` away from it; looking at the
//!   creature (yaw, pitch) rolled 15° · sin(2πt). In sub 8 the script camera lets go (`0x457118(1)`, the pose kept:
//!   the follow camera takes it from there, on the body).
//! * **Sub 8, taken over**: the lease; out of the moby loop while the body plays (crate::hero::hypno); the player no
//!   longer on it (another body or Ratchet) and its wait out: with Ratchet the meter off, no capture running, its
//!   idle fence and area, no path, not taken over, its walk limits, shown, its blast (2 across, its damage byte
//!   `+0x35`, 5 embers, 2 billows, 9 flares, flashes 6 / 4, size 1.1, its death sound, shaking, lod −2, flags
//!   0x850001: [`release_blast`]), state 11 (gone, or back on its lift).
//!
//! While taken over the body's states drive it (crate::hero::hypno): its gun ([`shoot`]), its bite
//! ([`controlled_attack`]: `0x429c28` with timer byte 1 — a new □ press (`0x138324`) holds the wait at 90, its own
//! countdown of the wait, the kill height, the bite; the wait out ends it).
//!
//! Not ported: the capture's stats (`0x18c360 + 0x294..`), the body's gun aim record `0x2571a0` (cleared at the
//! switch), `0x1b2564`, the help texts on L19 / L20 at the switch (`0x18c958 + 0x504`, `0x13965e`); the debug prints
//! (`0x2c27f0`); the paths' lengths into their points' `w` (`0x35ea60`, read by nothing here); a taken-over 0x0ed4's
//! attack (no level places one).

use crate::hero::physics::{add, atan2, dist, dist2, rot_dist, set_len, sub, turn_ease, V3};
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::{ai, bolts, countdown, mode, Ctx, MobyWorld};

/// The biter (no shot; its bite between joints 2 and 3).
pub const BITER: i16 = 0x0ec0;
/// The gunner (two guns on joints 2 and 3).
pub const GUNNER: i16 = 0x0ee1;
/// The third kind the code serves (one gun), placed nowhere.
const GUNNER_B: i16 = 0x0ed4;
/// The gunners' shot.
pub const SHOT: i16 = 0x1263;
/// The turning lift its reserves come back on.
const LIFT: i16 = super::super::level19::props::TURN_LIFT;

const H: usize = 0x20;
const STATUS: usize = 0x2e;
const DAMAGE: usize = 0x34;
const REACT: usize = 0x50;
const MV: usize = 0x100;
const AT: usize = 0x1c8;
const LAST: usize = 0x1ca;
const FLOOR: usize = 0x1d8;
const HOME: usize = 0x1f0;
const T: usize = 0x220;
const T_MOBY: usize = 0x230;
const FOUND: usize = 0x234;
const AIM: usize = 0x240;
const HEAD_AIM: usize = 0x2c0;
const AREA_IDLE: usize = 0x360;
const AREA_ALERT: usize = 0x364;
const JUMP_PATH: usize = 0x368;
const PATH_A: usize = 0x36c;
const PATH_B: usize = 0x370;
const PATH_C: usize = 0x374;
const PATH_D: usize = 0x378;
const AREA: usize = 0x37c;
const FENCE_A: usize = 0x380;
const FENCE_B: usize = 0x384;
const HELD_SLOTS: usize = 0x388;
const PATH: usize = 0x38c;
const PATIENCE: usize = 0x390;
const WAIT: usize = 0x392;
const COUNT: usize = 0x394;
const COOL: usize = 0x396;
const ALERT_WAIT: usize = 0x398;
const RESERVE: usize = 0x399;
const ALERTED: usize = 0x39a;
const KIND: usize = 0x39b;
const HIDDEN: usize = 0x39c;
const BURSTS: usize = 0x39d;
const HELD: usize = 0x39f;
const NEAR: usize = 0x3a0;
const SAFE_AREA: usize = 0x3a4;
const COOLDOWN: usize = 0x3a8;
const JUMPERS: usize = 0x3ac;
const JUMPING: usize = 0x3b0;
const LIVES: usize = 0x3b1;
const WAS_KIN: usize = 0x3b2;
const KIN: usize = 0x3b3;
const LIFT_L: usize = 0x3b4;
const SWIRL_SPEED: usize = 0x3b8;
/// `+0x3bc` / `+0x3be`: its time left while taken over and its whole (crate::hero::hypno).
pub const TIME: usize = 0x3bc;
const TIME_FULL: usize = 0x3be;
const LIMITS: usize = 0x3c0;
const UPDATE_DIST: usize = 0x3e0;
/// `+0x3ec`: the height under which its body dies.
pub const KILL_Z: usize = 0x3ec;
const ASLEEP: usize = 0x3e8;
const MISSION_KILL: usize = 0x3e9;
const LIFT_RESERVE: usize = 0x3ea;
const SOUND: usize = 0x3f0;
pub const PVARS: usize = 0x400;

/// `$gp+0x21c8..+0x21dc` (`0x1b11b8`): the walk limits (`+0x3c4`, `+0x3c0`), the run's (`+0x3cc`, `+0x3c8`) and the
/// turns (`+0x3d4` / `+0x3dc`, `+0x3d0` / `+0x3d8`) when a creature has none.
const DEFAULT_LIMITS: [f32; 6] = [2.0, 2.0, 16.0, 2.0, 1440.0, 720.0];
/// `$gp+0x21e0` / `+0x21e8`: the gravity (a second²) and the jump's speed (a second).
const GRAVITY: f32 = 9.8;
const JUMP_SPEED: f32 = 5.0;
/// `$gp+0x21f0..+0x2218`: ticks — the biter's attack, its cooldown after, the gunners' burst (shots), the gunners'
/// cooldown range, the patience's floor and range, the still wait, the wander's wait, the gunners' wait after.
const BITE_TICKS: i16 = 90;
const BITE_COOL: i16 = 30;
const BURST: u8 = 6;
const GUN_COOL: (i32, i32) = (140, 220);
const PATIENCE_FLOOR: i16 = 300;
const PATIENCE_RANGE: (i32, i32) = (600, 1000);
const STILL_WAIT: i16 = 240;
const WANDER_WAIT: i16 = 180;
const GUN_WAIT: i16 = 60;
/// `$gp+0x221c` / `+0x2220`: the biter's reach and how near it stops.
const BITE_REACH: f32 = 2.0;
const BITE_STOP: f32 = 1.1;
/// `$gp+0x2224..+0x222c`: the jump's cooldown range and the most of a group jumping in at once.
const JUMP_COOL: (i32, i32) = (30, 90);
const MOST_JUMPERS: i32 = 3;
/// `$gp+0x2230..+0x223c`: the gunners' reach and their shots' speed (a second), 0x0ed4's then 0x0ee1's.
const GUN_RANGE: [f32; 2] = [30.0, 30.0];
const SHOT_SPEED: [f32; 2] = [10.0, 20.0];
/// `$gp+0x2270`: the hits' push.
const PUSH: f32 = 2.0;
/// `$gp+0x2240..+0x224c`: the capture's swirl — its ticks, the camera's end across from the creature and up, its
/// roll (degrees).
const SWIRL_TICKS: i16 = 180;
const SWIRL_NEAR: f32 = 3.0;
const SWIRL_UP: f32 = 2.0;
const SWIRL_ROLL: f32 = 15.0;
/// The prompt's lease the capture holds (`0x342248(6, 0, 20)`).
const LEASE: i32 = 6;
/// The time a taken-over biter's □ press keeps its bite going (`$gp+0x21f0`).
const BITE_HOLD: i16 = BITE_TICKS;

fn seq_a(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a) }
fn gone(s: u8) -> bool { s == 0xfe || s == 0xfd }
fn class_coll(w: &MobyWorld, k: usize) -> bool { w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision) }
fn class_scale(w: &MobyWorld, k: usize) -> f32 { w.class(w.mobys[k].o_class).map_or(1.0, |c| c.scale) }
fn rnd(ctx: &mut Ctx, r: (i32, i32)) -> i16 { ctx.game.rng.rand_range(r.0, r.1) as i16 }
fn shared(w: &MobyWorld, id: usize, off: usize) -> i32 { w.shared_i(w.mobys[id].pv_i(off)) }
fn set_shared(w: &mut MobyWorld, id: usize, off: usize, v: i32) { let o = w.mobys[id].pv_i(off); w.set_shared_i(o, v); }
fn tick_s(w: &mut MobyWorld, id: usize, off: usize) {
    let mut t = w.mobys[id].pv_s(off) as i32;
    countdown(&mut t);
    w.mobys[id].set_pv_s(off, t as i16);
}
fn tick_b(w: &mut MobyWorld, id: usize, off: usize) {
    let mut t = w.mobys[id].pv_b(off) as i32;
    countdown(&mut t);
    w.mobys[id].set_pv_b(off, t as u8);
}
fn tick_shared(w: &mut MobyWorld, id: usize, off: usize) {
    let mut t = shared(w, id, off);
    countdown(&mut t);
    set_shared(w, id, off, t);
}
fn mission_done(ctx: &Ctx, mission: u8) -> bool {
    ctx.game.missions.get(ctx.game.level.max(0) as usize * 0x10 + mission as usize).is_some_and(|&b| b & 0x80 != 0)
}
fn grouped(w: &MobyWorld, id: usize) -> bool { w.mobys[id].group != -1 }
/// Its target's place (the target block's moby, else its point).
fn target_pos(w: &MobyWorld, id: usize) -> V3 {
    let m = &w.mobys[id];
    ai::link(m, T_MOBY).and_then(|k| w.mobys.get(k)).map_or(m.pv_v(T), |t| t.pos)
}
fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 { let q = w.mobys[id].pos; atan2(p[0] - q[0], p[1] - q[1]) }
/// `0x4283f0`: its class sound for its death (the biter 3, the gunner 10, else none).
fn death_sound(o_class: i16) -> i32 { match o_class { BITER => 3, GUNNER => 10, _ => -1 } }
/// Its live turning lift (`+0x3b4`, a 0x114d not gone).
fn lift(w: &MobyWorld, id: usize) -> Option<usize> {
    usize::try_from(w.mobys[id].pv_i(LIFT_L)).ok().filter(|&k| w.mobys.get(k).is_some_and(|l| l.o_class == LIFT && !gone(l.state)))
}

/// `0x428418(m, seq, frame, blend)`: its sequence (not while taken over).
fn anim_to(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, blend: u8) {
    if w.mobys[id].timer != 1 { set_seq(w, id, ctx, s, 0, blend); }
}
fn anim_rand(w: &MobyWorld, id: usize, ctx: &mut Ctx, base: u8, blend: u8) {
    let s = (ctx.game.rng.rand() & 1) as u8 | base;
    anim_to(w, id, ctx, s, blend);
}

/// `0x3489d0(m, a/3600, b/60, c·π/648000, d·π/10800)` (ai::senses) with its limits at `+off`, `+off − 4`, `+off + 0x10`,
/// `+off + 0xc`.
fn limits(w: &mut MobyWorld, id: usize, off: usize) {
    let m = &mut w.mobys[id];
    let (a, b, c, d) = (m.pv_f(off + 4), m.pv_f(off), m.pv_f(off + 0x14), m.pv_f(off + 0x10));
    ai::senses(m, a * f32::from_bits(0x3991_a2b4), b * f32::from_bits(0x3c88_8889), c * f32::from_bits(0x36a2_ad2f), d * f32::from_bits(0x3998_825c));
}

/// `0x428468`: holding one of its group's `+0x388`, its walk limits back and the count down; not holding.
fn release_held(w: &mut MobyWorld, id: usize) {
    if grouped(w, id) && w.mobys[id].pv_b(HELD) != 0 {
        limits(w, id, LIMITS);
        tick_shared(w, id, HELD_SLOTS);
    }
    w.mobys[id].set_pv_b(HELD, 0);
}
/// `0x428548`: one of its group's jumpers.
fn take_slot(w: &mut MobyWorld, id: usize) {
    if !grouped(w, id) || w.mobys[id].pv_b(JUMPING) != 0 { return; }
    let n = shared(w, id, JUMPERS) + 1;
    set_shared(w, id, JUMPERS, n);
    w.mobys[id].set_pv_b(JUMPING, 1);
}
/// `0x428500`: out of its group's jumpers.
fn free_slot(w: &mut MobyWorld, id: usize) {
    if grouped(w, id) && w.mobys[id].pv_b(JUMPING) != 0 { tick_shared(w, id, JUMPERS); }
    w.mobys[id].set_pv_b(JUMPING, 0);
}

fn path_pts<'a>(ctx: &'a Ctx, path: i32) -> Option<&'a Vec<[f32; 4]>> { usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| !p.is_empty()) }
fn pt(p: &[[f32; 4]], i: i32) -> V3 { let q = p[(i.max(0) as usize).min(p.len() - 1)]; [q[0], q[1], q[2]] }

/// `0x428620(m, path, last)`: the path it walks (`+0x38c`); onto it from its nearest point (`0x35e438`), toward
/// `last` (−1 its end; `0x348b88`). Returns the point it starts from (−1 none).
fn set_path(w: &mut MobyWorld, id: usize, ctx: &Ctx, path: i32, last: i32) -> i32 {
    let m = &mut w.mobys[id];
    m.set_pv_i(PATH, path);
    if path == -1 {
        m.set_pv_s(AT, -1);
        return -1;
    }
    let Some(p) = path_pts(ctx, path) else {
        m.set_pv_s(AT, 0);
        return -1;
    };
    let pos = m.pos;
    let near = (0..p.len()).fold((0usize, 1e11f32), |(bi, bd), i| { let d = dist(pt(p, i as i32), pos); if bd <= d { (bi, bd) } else { (i, d) } }).0;
    m.set_pv_i(MV + 0xc4, path + 1);
    m.set_pv_s(LAST, if last == -1 { p.len() as i16 - 1 } else { last as i16 });
    m.set_pv_s(AT, near as i16);
    near as i32
}

/// `0x4286b0`: at its path's end turned back — toward its end from the start (true), else the start.
fn turn_back(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    if w.mobys[id].pv_s(LAST) != 0 {
        w.mobys[id].set_pv_s(LAST, 0);
        return false;
    }
    let n = path_pts(ctx, w.mobys[id].pv_i(PATH)).map_or(0, |p| p.len() as i16 - 1);
    w.mobys[id].set_pv_s(LAST, n);
    true
}

/// `0x428580` (= L09 `0x3f8f18`): the walk along the path, or within its stop distance of the last point.
fn walk_done(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, path: i32) -> bool {
    let r = ai::follow_path(w, id, ctx, path);
    r || path_pts(ctx, path).is_some_and(|p| {
        let m = &w.mobys[id];
        dist2(m.pos, pt(p, m.pv_s(LAST) as i32)) < m.pv_f(MV + 0x34)
    })
}

/// `0x35e8b8(&out, from, to, path, 0)` → `0x35e908`: the point of the path nearest `to` (each segment's nearest point,
/// `0x35e548`), a tie within 0.1 to the one nearer `from`.
fn nearest_on_path(ctx: &Ctx, path: i32, from: V3, to: V3) -> Option<V3> {
    let p = path_pts(ctx, path)?;
    let mut best = (1e11f32, from);
    for i in 0..p.len().saturating_sub(1) {
        let (a, b) = (pt(p, i as i32), pt(p, i as i32 + 1));
        let seg = sub(b, a);
        let l = crate::hero::physics::dot(seg, seg);
        let t = if l == 0.0 { 0.0 } else { -crate::hero::physics::dot(sub(a, to), seg) / l };
        let c = if t < 0.0 { a } else if 1.0 < t { b } else { add(crate::hero::physics::scale(seg, t), a) };
        let d = dist(c, to);
        if d < best.0 || ((d - best.0).abs() < 0.1 && dist(c, from) < dist(best.1, from)) { best = (d, c); }
    }
    Some(best.1)
}

/// `0x429038(m, group)`: every biter of the group not gone alerted (`+0x39a`).
fn alert(w: &mut MobyWorld, id: usize) {
    let group = w.mobys[id].group;
    let Some(members) = usize::try_from(group).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for k in members {
        let Some(e) = w.mobys.get_mut(k) else { continue };
        if e.o_class == BITER && !gone(e.state) { e.set_pv_b(ALERTED, 1); }
    }
}

/// `0x4282f0(t)`: `t` one of its kind, not gone.
fn is_kin(w: &MobyWorld, t: Option<usize>) -> bool {
    t.and_then(|k| w.mobys.get(k)).is_some_and(|e| !gone(e.state) && matches!(e.o_class, BITER | GUNNER_B | GUNNER))
}

/// `0x428338(m, t)`: `t` within its reach across (`$gp+0x2258..`: 16, 0x0ed4 and 0x0ee1 30) and its view's half
/// angle (170°, 175°, 178°) of its facing.
fn kin_in_view(w: &MobyWorld, id: usize, t: usize) -> bool {
    let m = &w.mobys[id];
    let (r, a) = match m.o_class {
        GUNNER_B => (30.0, f32::from_bits(0x4043_7a16)),
        GUNNER => (30.0, f32::from_bits(0x4046_d3f3)),
        _ => (16.0, f32::from_bits(0x403d_e450)),
    };
    let tp = w.mobys[t].pos;
    dist2(m.pos, tp) < r && rot_dist(m.rot[2], yaw_to(w, id, tp)) < a
}

/// Back in play (sub-states 13 / 14): its update distance (`+0x3e0`), its class's collision, a target, its class's
/// size, the movement's radius 1 (0x0ed4) / 1.1 (0x0ee1).
fn show(w: &mut MobyWorld, id: usize) {
    let (coll, s) = (class_coll(w, id), class_scale(w, id));
    let m = &mut w.mobys[id];
    m.update_dist = m.pv_b(UPDATE_DIST);
    m.has_collision = coll;
    m.mode |= mode::TARGET;
    m.scale = s;
    match m.o_class {
        GUNNER_B => m.set_pv_f(MV, 1.0),
        GUNNER => m.set_pv_f(MV, 1.1),
        _ => {}
    }
}

/// `0x4286f8`: back on its lift (`+0x3b4`; none: nothing): at the lift's place and angles, alive with its most health,
/// sequence 7 slowed to 0.1, the idle fence, onto its first path, no collision nor drops, not a target, a life used
/// (unless endless), radius 0.7; the lift live, hidden and its rider (speed 1, state 0 sub 11); else the teleport-in
/// there (sub 14).
fn respawn(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(k) = usize::try_from(w.mobys[id].pv_i(LIFT_L)).ok().filter(|&k| k < w.mobys.len()) else { return };
    let (pos, rot) = (w.mobys[k].pos, w.mobys[k].rot);
    let m = &mut w.mobys[id];
    m.pos = pos;
    m.rot = rot;
    m.set_pv_b(STATUS, 1);
    m.set_pv_b(REACT + 0x1a, 0);
    m.set_pv_s(REACT + 0x38, 0);
    m.set_pv_f(H, m.pv_s(H + 4) as f32);
    m.set_pv_i(REACT + 0x44, 0);
    m.hit_slot = -1;
    anim_to(w, id, ctx, 7, 1);
    ctx.host.set_anim_speed(&w.mobys[id], 0.1);
    let m = &mut w.mobys[id];
    m.set_pv_i(MV + 0x3c, m.pv_i(FENCE_A));
    let first = m.pv_i(PATH_A);
    set_path(w, id, ctx, first, -1);
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.drops = 0;
    m.mode &= !mode::TARGET;
    if m.pv_b(LIVES) & 0x80 == 0 { tick_b(w, id, LIVES); }
    w.mobys[id].set_pv_f(MV, 0.7);
    if let Some(l) = lift(w, id) {
        let m = &mut w.mobys[id];
        m.drawn = 0;
        m.mode |= 1;
        ai::set_link(&mut w.mobys[l], 0, Some(id));
        ctx.host.set_anim_speed(&w.mobys[id], 1.0);
        w.mobys[id].set_state(0, 11);
    } else {
        crate::moby::classes::teleport::start(w, id, pos, rot, &[]);
        w.mobys[id].set_state(0, 14);
    }
}

/// `0x4288e0`: its replacement — the reserve (`+0x399` & 1) of its group nearest across to its way-in path's first
/// point (`+0x378`), else to the point of its jump path nearest Ratchet: shown, given its paths, areas and wait,
/// patience 1000, called in (2), alerted; to the jump (state 0 sub 4), its lift (a lift reserve), or the alert (sub
/// 3).
fn replace(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (way_in, jump) = (m.pv_i(PATH_D), m.pv_i(JUMP_PATH));
    let mut found = false;
    let at = if way_in == -1 {
        if jump == -1 {
            // The original prints an error and reads its vector uninitialised; its own place here.
            m.pos
        } else {
            found = true;
            nearest_on_path(ctx, jump, m.pos, ctx.hero.pos).unwrap_or(m.pos)
        }
    } else {
        path_pts(ctx, way_in).map_or(m.pos, |p| pt(p, 0))
    };
    let Some(members) = usize::try_from(m.group).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    let mut best: Option<(usize, f32)> = None;
    for k in members {
        let Some(e) = w.mobys.get(k) else { break };
        let d = dist2(e.pos, at);
        if e.pvars.len() > RESERVE && e.pv_b(RESERVE) & 1 != 0 && best.is_none_or(|(_, bd)| d < bd) { best = Some((k, d)); }
    }
    let Some((b, _)) = best else { return };
    let src = w.mobys[id].pvars[PATH_A..PATH_D + 4].to_vec();
    let (a_idle, a_alert, wait) = (w.mobys[id].pv_i(AREA_IDLE), w.mobys[id].pv_i(AREA_ALERT), w.mobys[id].pv_b(ALERT_WAIT));
    if w.mobys[b].pvars.len() < PVARS { w.mobys[b].pvars.resize(PVARS, 0); }
    let e = &mut w.mobys[b];
    e.drawn = 1;
    e.mode &= !1;
    e.pvars[PATH_A..PATH_D + 4].copy_from_slice(&src);
    e.set_pv_i(AREA_IDLE, a_idle);
    e.set_pv_i(AREA_ALERT, a_alert);
    e.set_pv_s(PATIENCE, PATIENCE_RANGE.1 as i16);
    e.set_pv_i(PATH, way_in);
    e.set_pv_b(ALERTED, 1);
    e.set_pv_b(RESERVE, 2);
    e.set_pv_s(WAIT, wait as i16);
    let on_lift = e.pv_b(LIFT_RESERVE) != 0;
    if found {
        e.set_state(0, 4);
    } else if on_lift && lift(w, b).is_some() {
        respawn(w, b, ctx);
    } else {
        w.mobys[b].set_state(0, 3);
    }
}

/// `0x428b78` (module docs): gone for good (its mission done and its uid gone) — true; else its blocks set up, its
/// limits, its home, its first path, a random idle sequence and asleep (state 0 sub 1, still), hidden (sub 2) or
/// watching (state 2 sub 2).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let m = &w.mobys[id];
    if m.mission != 0xff && mission_done(ctx, m.mission) && ctx.game.uid_dead.get(m.uid.max(0) as usize).is_some_and(|&v| v != 0) { return true; }
    let m = &mut w.mobys[id];
    let ud = m.update_dist as i32;
    for (k, v) in [(0x60, 10u8), (0x61, 7), (0x62, 8), (0x67, 0xff), (0xbe, 5), (0xbf, 11), (STATUS, 1), (0xbd, 6), (0x66, 8), (0x65, 9), (0xbc, 3), (0xc2, 5), (0xc3, 11), (0x28, 1), (0x2f, 0xff), (0x2a, 4), (0x4a, 7), (0x4d, 0)] { m.set_pv_b(k, v); }
    m.set_pv_i(REACT + 0x48, 9);
    for (k, v) in [(0x80, 0.4), (0x84, 0.4), (0x8c, 0.4), (0x30, 0.75), (MV, 0.7), (MV + 4, 0.5), (MV + 8, 0.5), (MV + 0xc, 0.5)] { m.set_pv_f(k, v); }
    m.set_pv_f(0x74, f32::from_bits(0x3b88_8889));
    m.set_pv_f(0x7c, f32::from_bits(0x3b88_8889));
    m.set_pv_f(0x6c, f32::from_bits(0x3e3b_bbbc));
    m.set_pv_f(0x70, f32::from_bits(0x3daa_aaab));
    m.set_pv_f(0x78, f32::from_bits(0x3c08_8889));
    m.set_pv_i(UPDATE_DIST, ud);
    m.set_pv_f(MV + 0x10, f32::from_bits(0x3f9c_61ab));
    let r = ctx.game.rng.randf(20.0, 25.0);
    let m = &mut w.mobys[id];
    m.set_pv_f(MV + 0x34, 0.5);
    m.set_pv_b(MV + 0xe8, 0);
    m.set_pv_b(MV + 0xe9, 0);
    m.set_pv_f(MV + 0x30, r * f32::from_bits(0x3c8e_fa36));
    m.set_pv_f(MV + 0x2c, GRAVITY * f32::from_bits(0x3991_a2b4));
    match m.o_class {
        GUNNER_B => m.set_pv_f(MV, 1.0),
        GUNNER => m.set_pv_f(MV, 1.1),
        _ => {}
    }
    if [0x3c4, 0x3c0, 0x3cc, 0x3c8].iter().all(|&k| m.pv_f(k) == 0.0) {
        let d = DEFAULT_LIMITS;
        for (k, v) in [(0x3c4, d[0]), (0x3c0, d[1]), (0x3cc, d[2]), (0x3c8, d[3]), (0x3dc, d[4]), (0x3d8, d[5]), (0x3d4, d[4]), (0x3d0, d[5])] { m.set_pv_f(k, v); }
    }
    limits(w, id, LIMITS);
    let r = ctx.game.rng.randf(20.0, 40.0);
    let m = &mut w.mobys[id];
    m.set_pv_s(MV + 0xe2, 180);
    m.set_pv_s(MV + 0xe4, 120);
    m.set_pv_f(MV + 0xdc, r * f32::from_bits(0x3c8e_fa36));
    m.set_pv_v(HOME, m.pos);
    m.set_pv_f(HOME + 0x10, m.pv_f(MV));
    m.set_pv_f(HOME + 0x20, 5.0);
    m.set_pv_f(HOME + 0x1c, 5.0);
    m.set_pv_i(AREA, m.pv_i(AREA_IDLE));
    let wait = ctx.game.rng.rand_range(0, 45) as i16;
    let m = &mut w.mobys[id];
    m.set_pv_s(WAIT, wait);
    m.set_pv_i(MV + 0x3c, m.pv_i(FENCE_A));
    let first = m.pv_i(PATH_A);
    set_path(w, id, ctx, first, -1);
    let m = &mut w.mobys[id];
    if m.pv_b(RESERVE) != 0 { m.set_pv_b(RESERVE, 1); }
    release_held(w, id);
    free_slot(w, id);
    let m = &mut w.mobys[id];
    match m.o_class {
        BITER => {
            m.set_pv_b(KIND, 0);
            m.set_pv_i(0x1c, 0);
        }
        GUNNER_B | GUNNER => m.set_pv_b(KIND, 2),
        _ => {}
    }
    let s = (ctx.game.rng.rand() & 1) as u8;
    anim_to(w, id, ctx, s, 1);
    let m = &mut w.mobys[id];
    if m.pv_b(ASLEEP) != 0 {
        m.set_state(0, 1);
        ctx.host.set_anim_speed(&w.mobys[id], 0.0);
    } else if m.pv_b(HIDDEN) != 0 {
        m.drawn = 0;
        m.mode |= 1;
        m.set_state(0, 2);
    } else {
        m.set_state(2, 2);
    }
    false
}

/// `0x4290d0` (module docs): the set-up (sub 0; gone for good, deleted); its shadow within 24 (range 22); the aim
/// record `+0x240` (its asked scale 0, list 4, 1 / 1; a gunner with endless lives only with the big heads, at 0.5/3);
/// then, unless taken over or Ratchet in 0x6f, the frame's damage (any: `+0x39f` let go, its group alerted; dead, its
/// bolts, its replacement ([`replace`]) and state 12 sub 10) and its target (`+0x37c`, range 0); its hit let go; on a
/// floor of surface 0, its mission's kill, or a kill asked by another (status 2: its replacement, state 11 sub 10)
/// it goes.
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].sub_state == 0 {
        w.mobys[id].set_pv_i(SOUND, -1);
        if setup(w, id, ctx) {
            w.delete(id);
            return;
        }
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 22);
    let m = &w.mobys[id];
    if m.pv_b(LIVES) & 0x80 == 0 || m.o_class != GUNNER {
        w.mobys[id].set_pv_f(AIM + 0x70, 0.0);
        crate::moby::joint_aim::aim(w, id, AIM, id, 4, 1.0, 1.0);
    } else if ctx.game.cheat(crate::cheats::cheat::ENEMY_HEADS) {
        w.mobys[id].set_pv_f(AIM + 0x70, 0.5 / 3.0);
        crate::moby::joint_aim::aim(w, id, AIM, id, 4, 1.0, 1.0);
    }
    let m = &w.mobys[id];
    if m.sub_state != 8 && m.timer != 1 && m.sub_state != 7 && !ctx.hero.hypno.capturing && ctx.hero.state != 0x6f {
        let hit = ai::take_damage(w, id, ctx, false);
        if hit != 0 {
            release_held(w, id);
            alert(w, id);
            if hit == 4 {
                bolts::drop(w, id, 0, 3, ctx);
                replace(w, id, ctx);
                w.mobys[id].set_state(12, 10);
            }
        }
        let m = &mut w.mobys[id];
        m.set_pv_b(WAS_KIN, m.pv_b(KIN));
        let area = m.pv_i(AREA);
        ai::find_target(w, id, ctx, 0.0, 0.0, area, None, false);
        let t = ai::link(&w.mobys[id], T_MOBY).and_then(|k| w.mobys.get(k)).map(|t| t.o_class);
        w.mobys[id].set_pv_b(KIN, matches!(t, Some(BITER | GUNNER_B | GUNNER)) as u8);
        let m = &w.mobys[id];
        if m.timer == 0 && m.sub_state != 7 && m.once & 4 != 0 { hooked(w, id, ctx); }
    }
    w.mobys[id].hit_slot = -1;
    let m = &w.mobys[id];
    if m.pv_i(FLOOR) == 0 { w.mobys[id].set_state(12, 10); }
    let m = &w.mobys[id];
    if m.pv_b(MISSION_KILL) != 0 && mission_done(ctx, m.mission) { w.mobys[id].set_state(12, 10); }
    if w.mobys[id].pv_b(STATUS) == 2 {
        release_held(w, id);
        alert(w, id);
        replace(w, id, ctx);
        w.mobys[id].set_state(11, 10);
    }
}

/// The head's branch on a hooked creature (module docs): the capture begins.
fn hooked(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].update_dist = 0xff;
    ctx.hero.hypno.meter = false;
    release_held(w, id);
    let (cp, ce) = ctx.game.cameras.view_euler();
    ctx.game.cameras.camera_script(cp, ce, crate::script_camera::MODE_SNAP, 0, false, ctx.hero);
    ctx.game.cameras.camera_script_targets(Some(cp), Some(ce));
    if matches!(ctx.hero.state, 1 | 0x1d) { ctx.hero.request_state(0, true); }
    ctx.hero.land_lag = 20;
    let m = &mut w.mobys[id];
    m.set_pv_f(SWIRL_SPEED, (dist2(m.pos, cp) - SWIRL_NEAR) / SWIRL_TICKS as f32);
    m.once &= !4;
    m.set_pv_s(COOL, SWIRL_TICKS);
    anim_to(w, id, ctx, 16, 10);
    ctx.hero.hypno.capturing = true;
    ctx.game.prompt.request(LEASE, 0, 20);
    w.mobys[id].set_state(0, 7);
}

/// `0x33b560` = L00 `0x320440`: an angle wrapped to [−π, π).
fn wrap(a: f32) -> f32 { use std::f32::consts::PI; ((a + PI) * 0.159_154_94).rem_euclid(1.0) * std::f32::consts::TAU - PI }

/// `0x32a7f0`: the first time since the last sub-state change.
fn first_sub(m: &mut crate::moby::Moby) -> bool {
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

/// Sub-state 7, the swirl (module docs).
fn swirl(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.game.prompt.request(LEASE, 0, 20);
    if first_sub(&mut w.mobys[id]) { ctx.host.play_sound_of(&w.mobys[id], GUNNER, 6, 0); }
    let h = w.mobys[id].pv_i(SOUND);
    if h == -1 || !ctx.host.sound_playing(h) {
        let h = ctx.host.play_sound_of(&w.mobys[id], GUNNER, 9, 4);
        w.mobys[id].set_pv_i(SOUND, h);
    }
    ai::flash(&mut w.mobys[id], REACT);
    let yaw = yaw_to(w, id, ctx.hero.pos);
    {
        let m = &mut w.mobys[id];
        let (a, b, c) = (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c));
        let (mut r, mut v) = (m.rot[2], m.pv_f(MV + 0xb4));
        turn_ease(yaw, a, b, c, &mut r, &mut v, 0);
        m.rot[2] = r;
        m.set_pv_f(MV + 0xb4, v);
    }
    let (view, _) = ctx.game.cameras.view_euler();
    ctx.hero.damage.invincible = 5;
    ctx.hero.land_lag = 10;
    let mut cool = w.mobys[id].pv_s(COOL) as i32;
    let out = countdown(&mut cool);
    w.mobys[id].set_pv_s(COOL, cool as i16);
    if out {
        ctx.hero.hypno.meter = true;
        let m = &mut w.mobys[id];
        m.set_pv_s(TIME, m.pv_s(TIME_FULL));
        let yaw = m.rot[2];
        if ctx.hero.mode == 4 { ctx.hero.body_back_keep(true, ctx.game.scenes.mode == crate::scene::SCENE); }
        // `0x428288`: the flash off, grey.
        let m = &mut w.mobys[id];
        m.set_pv_s(REACT, 0);
        m.set_pv_s(REACT + 2, 1);
        ai::flash(m, REACT);
        for k in 4..7 { m.set_pv_b(REACT + k, 0x80); }
        m.ambient = [0x80; 3];
        m.timer = 1;
        m.set_pv_i(MV + 0x3c, -1);
        m.set_pv_s(WAIT, 30);
        let (pos, rot, class, time, kill_z) = (m.pos, m.rot, m.o_class, m.pv_s(TIME), m.pv_f(KILL_Z));
        ctx.hero.hypno_switch(id, pos, rot, class, time, kill_z);
        ctx.hero.rot[2] = yaw;
        w.mobys[id].rot[2] = yaw;
        ctx.hero.hypno.capturing = false;
        w.mobys[id].set_state(7, 8);
        let h = w.mobys[id].pv_i(SOUND);
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(SOUND, -1);
    }
    let m = &w.mobys[id];
    let t = (1.0 - m.pv_s(COOL) as f32 / SWIRL_TICKS as f32).clamp(0.0, 1.0);
    let mut b = sub(ctx.hero.pos, m.pos);
    b[2] = 0.0;
    let mut b = add(set_len(b, SWIRL_NEAR), m.pos);
    b[2] += SWIRL_UP;
    let a = add(set_len(sub(view, b), m.pv_f(SWIRL_SPEED) * m.pv_s(COOL) as f32), b);
    let p = m.pos;
    let ang = [
        SWIRL_ROLL * (wrap(t * f32::from_bits(0x40c9_0fdb)).sin() * 0.017_453_294),
        -atan2(dist2(a, p), p[2] - a[2]),
        atan2(p[0] - a[0], p[1] - a[1]),
    ];
    ctx.game.cameras.camera_script_targets(Some(a), Some(ang));
    if w.mobys[id].sub_state == 8 { ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_KEEP, ctx.hero); }
}

/// Sub-state 8, taken over (module docs): let go once the player is no longer on it and its wait is out.
fn taken(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.game.prompt.request(LEASE, 0, 20);
    let on = ctx.hero.mode == 4 && ctx.hero.body.moby == Some(id);
    if on || w.mobys[id].pv_s(WAIT) != 0 { return; }
    if ctx.hero.mode != 4 { ctx.hero.hypno.meter = false; }
    ctx.hero.hypno.capturing = false;
    let m = &mut w.mobys[id];
    m.set_pv_i(MV + 0x3c, m.pv_i(FENCE_B));
    m.set_pv_i(AREA, m.pv_i(AREA_IDLE));
    m.set_pv_i(PATH, -1);
    m.timer = 0;
    limits(w, id, LIMITS);
    w.mobys[id].mode &= !0x41;
    release_blast(w, id, ctx);
    w.mobys[id].set_state(11, -1);
}

/// The blast of a creature let go (sub 8) or of its body's death (crate::hero::hypno's 0x91): `0x33b658(m, …, 2, its
/// damage byte, 5, 2, 9, 6, 4, 9, 1.1, its death sound, 0, 1, 1, −2, 0, 0x850001)`.
pub(crate) fn release_blast(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let e = crate::moby::blast::Explosion {
        radius: 2.0, damage: m.pv_b(0x35) as f32, embers: 5, billows: 2, flares: 9, flash: 6.0, flash2: 4.0, near: 9.0, size: 1.1,
        sound: death_sound(m.o_class), shake: true, debris: 1, lod: -2, flags: 0x85_0001, ..Default::default()
    };
    crate::moby::blast::explode(w, ctx, Some(id), None, &e);
}

/// `0x429bc0`: a taken-over biter's bite begins (its wait 90).
pub(crate) fn bite_start(w: &mut MobyWorld, id: usize) {
    if w.mobys[id].o_class == BITER { w.mobys[id].set_pv_s(WAIT, BITE_HOLD); }
}

/// `0x429c28` while taken over (the bite 0x93 of crate::hero::hypno; module docs): true when it is over.
pub(crate) fn controlled_attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    // Only the biter bites from the body (0x0ee1 has no bite; 0x0ed4 is placed nowhere).
    if w.mobys[id].o_class != BITER { return true; }
    if ctx.hero.items.pressed & crate::pad::button::SQUARE != 0 { w.mobys[id].set_pv_s(WAIT, BITE_HOLD); }
    tick_s(w, id, WAIT);
    if w.mobys[id].pos[2] < w.mobys[id].pv_f(REACT + 0x40) {
        bolts::drop(w, id, 0, 3, ctx);
        w.mobys[id].set_state(12, -1);
    }
    bite(w, id, ctx);
    w.mobys[id].pv_s(WAIT) == 0
}

/// `0x429538`: alerted, its patience at least 600..1000 (under 300 drawn again); its wait, cooldown and count; its
/// group's jump cooldown down; its state's ticks; the head aim `+0x2c0` (list 1, 3).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_b(ALERTED) != 0 {
        w.mobys[id].set_pv_b(ALERTED, 0);
        if w.mobys[id].pv_s(PATIENCE) < PATIENCE_FLOOR {
            let p = rnd(ctx, PATIENCE_RANGE);
            w.mobys[id].set_pv_s(PATIENCE, p);
        }
    }
    tick_s(w, id, WAIT);
    tick_s(w, id, COOL);
    let m = &mut w.mobys[id];
    m.set_pv_s(COUNT, m.pv_s(COUNT).wrapping_add(1));
    tick_shared(w, id, COOLDOWN);
    let m = &mut w.mobys[id];
    if m.state_ticks as u16 <= 0xfffe { m.state_ticks = m.state_ticks.wrapping_add(1); }
    crate::moby::joint_aim::head(w, ctx, id, HEAD_AIM, 1, 3.0);
}

/// `0x429628`: in reach to attack (not with Ratchet held, `0x18a000`, or in 0x6f): the biter within 2 across with
/// its cooldown out — its run limits, the wait 90; a gunner within its range — then sequence 10, state 6.
fn in_reach(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if ctx.hero.land_lag != 0 || ctx.hero.state == 0x6f { return false; }
    let m = &w.mobys[id];
    let d = dist2(m.pos, m.pv_v(T));
    let cool = m.pv_s(COOL) == 0;
    let reach = match m.o_class {
        BITER => BITE_REACH,
        GUNNER_B => GUN_RANGE[0],
        GUNNER => GUN_RANGE[1],
        _ => return false,
    };
    if !(d < reach && cool) { return false; }
    if m.o_class == BITER {
        limits(w, id, LIMITS + 8);
        w.mobys[id].set_pv_s(WAIT, BITE_TICKS);
    }
    anim_to(w, id, ctx, 10, 10);
    w.mobys[id].set_state(6, -1);
    true
}

/// `0x429828`: the bite — in sequence 0xa, the line from its joint 2 to 3 (`0x338348` = L01 `0x3132c8`, flags
/// 0x4000001, its damage, pushed 2).
fn bite(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    // Taken over, its sequence is the body's (the hero's animation).
    let s = if ctx.hero.body.moby == Some(id) { ctx.hero_view.seq_a } else { seq_a(w, id, ctx) };
    if s != 0xa { return; }
    let m = &w.mobys[id];
    let (Some(a), Some(b)) = (ctx.host.joint(m, 2), ctx.host.joint(m, 3)) else { return };
    let flags = if m.timer == 1 { 0x0401_0000 } else { 0x0400_0001 };
    let damage = m.pv_b(DAMAGE) as f32;
    ai::attack_line(w, id, ctx, a, b, flags, damage, PUSH, 0);
}

/// `0x429918(m, side)`: a shot from its gun (joint `side + 2`): something already there takes a 0.35 sphere's hit
/// (flags 0x10000, kinds 1 / 3) — then a small blast there (5 embers, 2 billows, 9 flares, size 1.1, its death sound)
/// and a sphere of 1 (flags 0x10) of its damage (flags 0x810001, up 2); else a shot ([`fire`]) along its forward row
/// at its speed (0x0ee1 20, else 10 a second), dropping toward its target's height over the distance, its range 30.
pub(crate) fn shoot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, side: usize) {
    let m = &w.mobys[id];
    let (range, speed) = if m.o_class == GUNNER { (GUN_RANGE[1], SHOT_SPEED[1]) } else { (GUN_RANGE[0], SHOT_SPEED[0]) };
    let Some(pos) = ctx.host.joint(m, side + 2) else { return };
    let mask = if m.timer == 0 { 0x10000 } else { 1 };
    let damage = m.pv_b(DAMAGE) as f32;
    if ai::attack_sphere(w, id, ctx, 0.35, damage, PUSH, pos, mask, 1, 3, 0) != 0 {
        let m = &w.mobys[id];
        let mut t = crate::moby::hits::HitTemplate::new(damage, Some(id), 0x81_0001, [0.0, 0.0, PUSH, 5_627.925]);
        t.kind = (m.o_class as u16 as u32) << 16 | 3 << 8 | 2;
        let e = crate::moby::blast::Explosion { embers: 5, billows: 2, flares: 9, near: 9.0, size: 1.1, sound: death_sound(m.o_class), debris: 1, lod: -2, flags: 0x81_0001, ..Default::default() };
        crate::moby::blast::explode(w, ctx, Some(id), Some(pos), &e);
        w.sphere_hit(ctx.scene, pos, 1.0, 16, Some(id), Some(&t));
        return;
    }
    let m = &w.mobys[id];
    let speed = speed * 0.016_666_668;
    let mut vel = set_len(m.rows[0], speed);
    if m.timer == 0 {
        let to = m.pv_v(T);
        let t = dist2(pos, to) / speed;
        vel[2] = (to[2] - pos[2]) / t;
    }
    fire(w, ctx, pos, id, vel, 1, 3, damage, range, PUSH);
}

/// `0x429c28` (state 6): the biter closes (within 1.1 still, else toward its target), under its kill height (`+0x90`)
/// its bolts and state 12, bites ([`bite`]); its wait out, `+0x39f` let go, its cooldown 30, sequence 2, state 4 sub
/// 6. A gunner turns in place to its target; sequence 10 at its end 11 at 3× speed with a burst of 6; in 11 the
/// gunner shoots from either gun in turn every 6 ticks (the second 3 after), 0x0ed4 from one on 5 of each 6, each a
/// shot of the burst; none left, 12; 12 at its end its cooldown 140..220, its wait 60, sequence 4 / 5, state 2 sub 6.
fn attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let class = w.mobys[id].o_class;
    match class {
        BITER => {
            if w.mobys[id].timer != 1 {
                let m = &w.mobys[id];
                let to = m.pv_v(T);
                if dist2(m.pos, to) < BITE_STOP {
                    ai::hold(w, id, ctx);
                } else {
                    let yaw = yaw_to(w, id, to);
                    ai::move_step(w, id, ctx, yaw, to);
                }
            }
            if w.mobys[id].pos[2] < w.mobys[id].pv_f(REACT + 0x40) {
                bolts::drop(w, id, 0, 3, ctx);
                w.mobys[id].set_state(12, -1);
            }
            bite(w, id, ctx);
            if w.mobys[id].pv_s(WAIT) == 0 && w.mobys[id].timer != 1 {
                release_held(w, id);
                w.mobys[id].set_pv_s(COOL, BITE_COOL);
                anim_to(w, id, ctx, 2, 10);
                w.mobys[id].set_state(4, 6);
            }
        }
        GUNNER_B | GUNNER => {
            if w.mobys[id].timer != 1 {
                let yaw = yaw_to(w, id, target_pos(w, id));
                let pos = w.mobys[id].pos;
                ai::move_step(w, id, ctx, yaw, pos);
            }
            match seq_a(w, id, ctx) {
                10 => if wrapped(w, id, ctx) {
                    anim_to(w, id, ctx, 11, 10);
                    ctx.host.set_anim_speed(&w.mobys[id], 3.0);
                    let m = &mut w.mobys[id];
                    m.set_pv_s(COUNT, 0);
                    m.set_pv_b(BURSTS, BURST);
                },
                11 => {
                    let n = w.mobys[id].pv_s(COUNT);
                    if class == GUNNER {
                        if n % 6 == 0 {
                            shoot(w, id, ctx, 0);
                            tick_b(w, id, BURSTS);
                        }
                        if (n + 3) % 6 == 0 { shoot(w, id, ctx, 1); }
                    } else if n % 6 != 0 {
                        shoot(w, id, ctx, 0);
                        tick_b(w, id, BURSTS);
                    }
                    if w.mobys[id].timer != 1 && w.mobys[id].pv_b(BURSTS) == 0 { anim_to(w, id, ctx, 12, 10); }
                }
                12 => if wrapped(w, id, ctx) {
                    let c = rnd(ctx, GUN_COOL);
                    let m = &mut w.mobys[id];
                    m.set_pv_s(COOL, c);
                    m.set_pv_s(WAIT, GUN_WAIT);
                    anim_rand(w, id, ctx, 4, 10);
                    w.mobys[id].set_state(2, 6);
                },
                _ => {}
            }
        }
        _ => {}
    }
}

/// `0x429fd8` (state 4): its lift's collision off for the move. Walking off its lift (sub 13): toward 5 ahead of the
/// lift at its own height, turning freely (any slope, half a step down, 10× its walk). On a path (`+0x38c`), along it
/// ([`walk_done`]); at its end: the patrol (5) onto `+0x374` (a called-in reserve a reserve no more) with the wait 90,
/// sequence 2 and the chase; the chase (6) turned back (then the same) and a gunner's attack ([`in_reach`]); the
/// watch (2) turned back, at its sequence's end a random idle sequence and state 2. A biter patrolling or chasing
/// within `+0x3a0` of its target leaves its path for the chase. Off a path: the chase toward its target
/// (`0x347830`) and the attack; else the wander about its home, its wait out the wait 180, a random idle sequence,
/// state 2; under its kill height its bolts and state 12 sub 10.
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let lift_k = lift(w, id);
    if let Some(l) = lift_k { w.mobys[l].has_collision = false; }
    let sub_state = w.mobys[id].sub_state;
    if sub_state == 13 {
        if let Some(l) = lift_k {
            let (lp, lr) = (w.mobys[l].pos, w.mobys[l].rows[0]);
            let mut a = add(set_len(lr, 5.0), lp);
            a[2] = w.mobys[id].pos[2];
            let m = &mut w.mobys[id];
            m.set_pv_f(MV + 0x28, m.pv_f(MV + 0x28) * 10.0);
            m.set_pv_f(MV + 0x20, m.pv_f(MV + 0x20) * 10.0);
            let (slope, down) = (m.pv_f(MV + 0x10), m.pv_f(MV + 0xc));
            m.set_pv_f(MV + 0x10, std::f32::consts::PI);
            m.set_pv_f(MV + 0xc, 0.5);
            let yaw = yaw_to(w, id, a);
            ai::move_step(w, id, ctx, yaw, a);
            let m = &mut w.mobys[id];
            m.set_pv_f(MV + 0x28, m.pv_f(MV + 0x28) * 0.1);
            m.set_pv_f(MV + 0x20, m.pv_f(MV + 0x20) * 0.1);
            m.set_pv_f(MV + 0x10, slope);
            m.set_pv_f(MV + 0xc, down);
        }
    } else if w.mobys[id].pv_i(PATH) != -1 {
        let path = w.mobys[id].pv_i(PATH);
        if walk_done(w, id, ctx, path) {
            match sub_state {
                5 => {
                    if w.mobys[id].pv_b(RESERVE) & 2 != 0 { w.mobys[id].set_pv_b(RESERVE, 0); }
                    let p = w.mobys[id].pv_i(PATH_C);
                    set_path(w, id, ctx, p, -1);
                    w.mobys[id].set_pv_s(WAIT, BITE_TICKS);
                    anim_to(w, id, ctx, 2, 10);
                    w.mobys[id].set_state(4, 6);
                }
                6 => {
                    if turn_back(w, id, ctx) {
                        w.mobys[id].set_pv_s(WAIT, BITE_TICKS);
                        anim_to(w, id, ctx, 2, 10);
                        w.mobys[id].set_state(4, 6);
                    }
                    if w.mobys[id].o_class != BITER { in_reach(w, id, ctx); }
                }
                2 => {
                    turn_back(w, id, ctx);
                    if wrapped(w, id, ctx) {
                        anim_rand(w, id, ctx, 0, 10);
                        w.mobys[id].set_state(2, -1);
                    }
                }
                _ => {}
            }
        }
        let m = &w.mobys[id];
        if m.o_class == BITER && m.sub_state.wrapping_sub(5) < 2 && dist2(m.pos, target_pos(w, id)) < m.pv_f(NEAR) {
            w.mobys[id].set_pv_i(PATH, -1);
            w.mobys[id].set_state(4, 6);
        }
    } else {
        if sub_state == 6 {
            let to = w.mobys[id].pv_v(T);
            if w.mobys[id].state == 6 {
                let yaw = yaw_to(w, id, to);
                ai::move_step(w, id, ctx, yaw, to);
            } else {
                ai::move_toward(w, id, ctx, 0.0, to);
                in_reach(w, id, ctx);
            }
        } else {
            super::super::leaper::wander(w, id, ctx, HOME);
            if w.mobys[id].pv_s(WAIT) == 0 {
                w.mobys[id].set_pv_s(WAIT, WANDER_WAIT);
                anim_rand(w, id, ctx, 0, 10);
                w.mobys[id].set_state(2, -1);
            }
        }
        if w.mobys[id].pos[2] < w.mobys[id].pv_f(REACT + 0x40) {
            bolts::drop(w, id, 0, 3, ctx);
            w.mobys[id].set_state(12, 10);
        }
    }
    if let Some(l) = lift_k {
        let c = class_coll(w, l);
        w.mobys[l].has_collision = c;
    }
}

/// The jump in (sub-states 2 and 6): the group's cooldown out and fewer than 3 in, the cooldown 30..90, one of them.
fn jump_in(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if shared(w, id, COOLDOWN) != 0 || MOST_JUMPERS <= shared(w, id, JUMPERS) { return false; }
    let c = rnd(ctx, JUMP_COOL) as i32;
    set_shared(w, id, COOLDOWN, c);
    take_slot(w, id);
    true
}
/// Its target outside its safe area (`+0x3a4`, −1 none: never safe).
fn unsafe_target(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    let a = m.pv_i(SAFE_AREA);
    a == -1 || !ctx.level.in_area(a, m.pv_v(T))
}

/// The sub-states (module docs).
fn subs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].sub_state {
        1 => if seq_a(w, id, ctx) == 0xd { ctx.host.set_anim_speed(&w.mobys[id], 0.0); },
        2 => {
            let m = &w.mobys[id];
            let t = ai::link(m, T_MOBY);
            let seen = m.pv_b(RESERVE) & 1 == 0 && (m.pv_i(FOUND) == 1 || m.pv_s(PATIENCE) != 0);
            if !(seen && (!is_kin(w, t) || t.is_some_and(|t| kin_in_view(w, id, t)))) { return; }
            alert(w, id);
            let m = &mut w.mobys[id];
            m.set_pv_i(AREA, m.pv_i(AREA_ALERT));
            m.set_pv_i(MV + 0x3c, m.pv_i(FENCE_B));
            m.set_pv_s(WAIT, m.pv_b(ALERT_WAIT) as i16);
            let jump = m.pv_i(JUMP_PATH) != -1;
            if jump && unsafe_target(w, id, ctx) {
                if jump_in(w, id, ctx) { w.mobys[id].set_state(1, 4); }
            } else {
                let p = w.mobys[id].pv_i(PATH_B);
                set_path(w, id, ctx, p, -1);
                w.mobys[id].set_state(1, 3);
            }
        }
        3 => if w.mobys[id].pv_s(WAIT) == 0 {
            anim_to(w, id, ctx, 3, 5);
            if w.mobys[id].pv_i(PATH_B) != -1 {
                w.mobys[id].set_state(3, 5);
            } else {
                let p = w.mobys[id].pv_i(PATH_C);
                set_path(w, id, ctx, p, -1);
                let pa = rnd(ctx, PATIENCE_RANGE);
                w.mobys[id].set_pv_s(PATIENCE, pa);
                w.mobys[id].set_state(3, 6);
            }
        },
        4 => if w.mobys[id].pv_s(WAIT) == 0 {
            let m = &mut w.mobys[id];
            m.drawn = 1;
            m.mode &= !1;
            let (path, pos) = (m.pv_i(JUMP_PATH), m.pos);
            let to = nearest_on_path(ctx, path, pos, ctx.hero.pos).unwrap_or(pos);
            anim_to(w, id, ctx, 6, 5);
            ai::jump_start(&mut w.mobys[id], JUMP_SPEED * 0.016_666_668, GRAVITY * 0.000_277_777_8, to);
            w.mobys[id].set_state(5, 10);
        },
        5 => {
            let m = &w.mobys[id];
            if m.pv_i(FOUND) == 1 && m.pv_s(PATIENCE) == 0 {
                let p = rnd(ctx, PATIENCE_RANGE);
                w.mobys[id].set_pv_s(PATIENCE, p);
            }
        }
        6 => {
            let m = &w.mobys[id];
            let t = ai::link(m, T_MOBY).and_then(|k| w.mobys.get(k));
            let lost = (m.pv_i(FOUND) != 1 && m.pv_s(PATIENCE) == 0)
                || (m.pv_b(WAS_KIN) != 0 && m.pv_b(KIN) == 0)
                || (m.pv_b(KIN) != 0 && t.is_some_and(|t| t.timer == 0));
            if lost {
                w.mobys[id].set_pv_s(PATIENCE, 0);
                anim_rand(w, id, ctx, 4, 10);
                w.mobys[id].set_state(2, 2);
            } else if m.pv_b(JUMPING) == 0 && m.pv_i(JUMP_PATH) != -1 && m.pv_i(SAFE_AREA) != -1 && unsafe_target(w, id, ctx) && jump_in(w, id, ctx) {
                w.mobys[id].set_state(0, 4);
            }
        }
        9 => {
            let r = ai::flight(w, id, ctx);
            if r & 1 != 0 {
                bolts::drop(w, id, 0, 3, ctx);
                w.mobys[id].set_state(12, -1);
            } else if r & 2 != 0 {
                let p = rnd(ctx, PATIENCE_RANGE);
                w.mobys[id].set_pv_s(PATIENCE, p);
                alert(w, id);
                anim_rand(w, id, ctx, 4, 10);
                w.mobys[id].set_state(4, 6);
            }
        }
        11 => {
            w.mobys[id].set_pv_f(MV, 0.7);
            if let Some(l) = lift(w, id) {
                if w.mobys[l].timer == 0 {
                    let s = class_scale(w, id);
                    let m = &mut w.mobys[id];
                    m.alpha = 0x80;
                    m.scale = s;
                    m.drawn = 1;
                    m.mode &= !1;
                    m.set_state(0, 12);
                }
            } else {
                let m = &mut w.mobys[id];
                m.drawn = 1;
                m.mode &= !1;
                let (pos, rot) = (m.pos, m.rot);
                crate::moby::classes::teleport::start(w, id, pos, rot, &[]);
                w.mobys[id].set_state(0, 14);
            }
        }
        12 => {
            let Some(l) = usize::try_from(w.mobys[id].pv_i(LIFT_L)).ok().filter(|&k| k < w.mobys.len()) else { return };
            w.mobys[id].pos = w.mobys[l].pos;
            if w.mobys[l].timer == 2 {
                if seq(w, id, ctx) != 2 { ctx.host.set_anim(&w.mobys[id], 2, 0, 10); }
                w.mobys[id].set_state(4, 13);
            }
        }
        13 => {
            let Some(l) = usize::try_from(w.mobys[id].pv_i(LIFT_L)).ok().filter(|&k| k < w.mobys.len()) else { return };
            if w.mobys[l].timer == 3 {
                show(w, id);
                w.mobys[id].set_pv_s(WAIT, 0);
                w.mobys[id].set_state(2, 2);
            }
        }
        7 => swirl(w, id, ctx),
        8 => taken(w, id, ctx),
        14 => {
            w.mobys[id].alpha = 0x80;
            if class_scale(w, id) <= w.mobys[id].scale {
                show(w, id);
                if w.mobys[id].pv_b(ASLEEP) != 0 {
                    if seq(w, id, ctx) != 13 { ctx.host.set_anim(&w.mobys[id], 13, 0, 10); }
                    w.mobys[id].set_state(0, 1);
                } else {
                    w.mobys[id].set_pv_s(WAIT, 0);
                    w.mobys[id].set_state(2, 2);
                }
            }
        }
        _ => {}
    }
}

/// The states (module docs); false when it was deleted.
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    match w.mobys[id].state {
        1 => {
            let yaw = yaw_to(w, id, w.mobys[id].pv_v(T));
            let pos = w.mobys[id].pos;
            ai::move_step(w, id, ctx, yaw, pos);
        }
        2 => {
            ai::hold(w, id, ctx);
            if w.mobys[id].pv_s(WAIT) == 0 {
                w.mobys[id].set_pv_s(WAIT, STILL_WAIT);
                anim_to(w, id, ctx, 2, 10);
                w.mobys[id].set_state(4, -1);
            }
        }
        3 => {
            let yaw = yaw_to(w, id, ctx.hero.pos);
            let m = &mut w.mobys[id];
            let (a, b, c) = (m.pv_f(MV + 0x14), m.pv_f(MV + 0x18), m.pv_f(MV + 0x1c));
            let (mut r, mut v) = (m.rot[2], m.pv_f(MV + 0xb4));
            turn_ease(yaw, a, b, c, &mut r, &mut v, 0);
            m.rot[2] = r;
            m.set_pv_f(MV + 0xb4, v);
            if wrapped(w, id, ctx) {
                anim_to(w, id, ctx, 2, 10);
                w.mobys[id].set_state(4, -1);
            }
        }
        4 => walk(w, id, ctx),
        5 => if ai::jump_step(w, id, ctx, 7.0, 32.0, 37.0) {
            let m = &w.mobys[id];
            if ctx.level.in_polygon(m.pv_i(FENCE_A), m.pos) {
                w.mobys[id].set_pv_s(WAIT, 0);
                w.mobys[id].set_state(0, 4);
            } else {
                anim_to(w, id, ctx, 2, 10);
                let (b, c) = (w.mobys[id].pv_i(PATH_B), w.mobys[id].pv_i(PATH_C));
                if b != -1 {
                    set_path(w, id, ctx, b, -1);
                    w.mobys[id].set_state(4, 5);
                } else {
                    if c != -1 { set_path(w, id, ctx, c, -1); } else { w.mobys[id].set_pv_i(PATH, -1); }
                    w.mobys[id].set_state(4, 6);
                }
            }
        },
        6 => attack(w, id, ctx),
        s @ (11 | 12) => {
            if s == 12 {
                let sound = death_sound(w.mobys[id].o_class);
                let e = crate::moby::blast::Explosion { embers: 10, billows: 3, flares: 16, flash: 4.0, flash2: 2.0, near: 9.0, size: 1.0, sound, light: 15.0, shake: true, debris: 1, lod: -1, flags: 0x81_0001, ..Default::default() };
                crate::moby::blast::explode(w, ctx, Some(id), None, &e);
            }
            free_slot(w, id);
            let (uid, mission) = (w.mobys[id].uid, w.mobys[id].mission);
            ctx.game.count_uid(uid);
            ctx.game.uid_gone(uid, mission);
            let lives = w.mobys[id].pv_b(LIVES);
            if lives != 0 && lives != 0xfe && (mission == 0xff || !mission_done(ctx, mission)) {
                respawn(w, id, ctx);
            } else {
                w.delete(id);
                return false;
            }
        }
        _ => {}
    }
    true
}

/// 0x0ec0 / 0x0ee1 (`0x42a3f8`, module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    if !ctx.level.in_variant(w.mobys[id].pos, ctx.game.variant) { return; }
    head(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    subs(w, id, ctx);
    if 0x80 <= w.mobys[id].state { return; }
    if !states(w, id, ctx) { return; }
    tail(w, id, ctx);
    w.changed = true;
}

/// The shot's pvars: `+0x00` its velocity, `+0x10` its owner (a link), `+0x1c` its range, `+0x20` its damage,
/// `+0x24` its push, `+0x28` / `+0x2c` its hit's kinds, `+0x30` its trail, `+0x34` its life.
const VEL: usize = 0x00;
const OWNER: usize = 0x10;
const RANGE: usize = 0x1c;
const S_DAMAGE: usize = 0x20;
const S_PUSH: usize = 0x24;
const KIND_A: usize = 0x28;
const KIND_B: usize = 0x2c;
const TRAIL: usize = 0x30;
const LIFE: usize = 0x34;

/// `0x4414e8(pos, owner, vel, a, b, damage, range, push)`: a shot 0x1263 at `pos` moving `vel` a tick (rows kept, mode
/// 0x100; update and draw distances 0xff), its yaw and pitch along it, its rows from it (the second its cross with
/// the vector at `0x18a0e0`, here up, made −1 long; the third their cross), its life its range over its speed
/// (truncated); a gunner's class sound 5; hidden until its first update.
#[allow(clippy::too_many_arguments)]
pub fn fire(w: &mut MobyWorld, ctx: &mut Ctx, pos: V3, owner: usize, vel: V3, a: i32, b: i32, damage: f32, range: f32, push: f32) -> usize {
    use crate::hero::physics::{cross, len};
    let k = w.create(SHOT);
    let m = &mut w.mobys[k];
    m.mode = mode::KEEP_ROWS;
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.state = 0;
    m.set_pv_v(VEL, vel);
    m.pos = pos;
    m.rot[2] = atan2(vel[0], vel[1]);
    m.rot[1] = -atan2(len([vel[0], vel[1], 0.0]), vel[2]);
    m.set_pv_i(TRAIL, -1);
    m.set_pv_f(RANGE, range);
    let r1 = set_len(cross(vel, [0.0, 0.0, 1.0]), -1.0);
    m.rows = [vel, r1, cross(r1, vel)];
    let l = len(vel);
    m.set_pv_i(LIFE, if l == 0.0 { 0 } else { (range / l) as i32 });
    m.set_pv_f(S_PUSH, push);
    m.set_pv_f(S_DAMAGE, damage);
    m.set_pv_i(KIND_A, a);
    m.set_pv_i(KIND_B, b);
    ai::set_link(m, OWNER, Some(owner));
    if w.mobys[owner].o_class == GUNNER { ctx.host.play_sound(&w.mobys[owner], 5, 0); }
    let m = &mut w.mobys[k];
    m.drawn = 0;
    m.mode |= 1;
    w.changed = true;
    k
}

/// 0x1263 (`0x4416d0`): on to its next place; its hit (its damage from it, flags 0x10001, along its way its push
/// long, its kinds and class in the kind word); the line there (flags 0, its owner skipped) delivering it, else a 0.2
/// sphere there (`0x2fa170` = L00 `0x2df8d8`), else its life: none of them, on it goes. A moby met (not its owner): a
/// 0.5 sphere of the hit (`0x2fbd48`) and a small blast (2 billows, 10 flares, size 0.666, its class sound 0), gone;
/// the world (or its life out): a smaller one (8 flares, size 0.5), gone.
///
/// Not ported: its sparks (`0x354f20` = L00 `0x339848`, crate::particles::spark, and `0x350d20` = L00 `0x335780`, from
/// the table `0x2bd9c0..`, every other frame when the frame is slow, `$gp+0x335c`) and their random draws; its trail
/// (`0x356130`, as crate::moby::classes::level12::shot's).
pub fn update_1263(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (pos, vel) = (m.pos, m.pv_v(VEL));
    let next = add(pos, vel);
    let owner = ai::link(m, OWNER);
    let d = set_len(vel, m.pv_f(S_PUSH));
    let mut t = crate::moby::hits::HitTemplate::new(m.pv_f(S_DAMAGE), Some(id), 0x1_0001, [d[0], d[1], d[2], 5_627.925]);
    t.kind = (SHOT as u16 as u32) << 16 | (m.pv_i(KIND_B) as u32 & 0xff) << 8 | (m.pv_i(KIND_A) as u32 & 0xff);
    let (coll, scene) = (ctx.coll, ctx.scene);
    let line = coll.and_then(|c| crate::collision::coll_line_m(c, scene, pos, next, crate::collision::QueryFlags(0), owner));
    if let Some(o) = line.as_ref() {
        if let Some(k) = o.moby { w.deliver_hit_any(k, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
    }
    let met = match line {
        Some(o) => Some(o.moby),
        None => coll.and_then(|c| crate::collision::coll_sphere_m(c, scene, next, 0.2, crate::collision::QueryFlags(0), owner)).map(|o| o.moby),
    };
    let met = match met {
        Some(mo) => Some(mo),
        None => {
            let mut l = w.mobys[id].pv_i(LIFE);
            let out = countdown(&mut l);
            w.mobys[id].set_pv_i(LIFE, l);
            out.then_some(None)
        }
    };
    if let Some(mo) = met {
        let (size, flares) = match mo {
            Some(k) if Some(k) == owner => {
                w.mobys[id].pos = next;
                return;
            }
            Some(_) => {
                w.sphere_hit(scene, pos, 0.5, 0, owner, Some(&t));
                (f32::from_bits(0x3f2a_7efa), 10)
            }
            None => (0.5, 8),
        };
        let e = crate::moby::blast::Explosion { near: 9.0, size, billows: 2, flares, sound: 0, debris: 1, lod: -1, flags: 0x81_0001, ..Default::default() };
        crate::moby::blast::explode(w, ctx, Some(id), None, &e);
        w.delete(id);
        return;
    }
    w.mobys[id].pos = next;
    w.changed = true;
}
