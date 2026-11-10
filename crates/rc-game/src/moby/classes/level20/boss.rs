//! Yeedil's boss **0x11e0** (`0x441ad8`, 1 placed; the decomp's v2.00 `func_US2_L20_00441AD8`): a great beast in a pool
//! that rolls at Ratchet, leaps back, eats crates and the prize 0x12d3 for its attacks, throws its biters 0x119e
//! (crate::moby::classes::level20::biter) out of the pool and spits the shooters 0x12ef
//! (crate::moby::classes::level20::shooter) at the fighters 0x12f3 (crate::moby::classes::level20::fighter) that come
//! to help. It runs only with the level's chunk 2 in place (`0x1ca4c0`, crate::game::GameState::variant).
//!
//! **Its pvars:** health `+0x20` (its most `+0x24`, its status `+0x2e`: 2 a kill asked, `+0x30` 2.5; its damage bytes
//! `+0x34` the gulp's, `+0x35` the spit's, `+0x36` the others'; `+0x38` the last hitter), movement `+0x50` (radius 4,
//! height 0.5, steps 2 / 2, slope π), target `+0x1f0` (the search's area `+0x220`). Groups: `+0x218` the crates,
//! `+0x21c` the biters, `+0x23c` the fighters, `+0x240` the shooters, `+0x244` woken at the last stage; `+0x210` the
//! roll's fence path, `+0x230` the pool's cuboid, `+0x234` the arena's six points (a path), `+0x238` the prize's moby,
//! `+0x248` the end's checkpoint cuboid; `+0x250` the head's aim record (`+0x298` its yaw, half of it the throws', `+0x2b8` the turn asked). `+0x17d0` a
//! goal, `+0x17e0` where it holds still, `+0x17f0` the camera's ease, `+0x17f4` the sub-state of the last pick,
//! `+0x17f8` the rolls left, `+0x17fc` its prey (a link), `+0x1800` / `+0x1804` its tilt (x / y), `+0x1808` the spit's
//! frame, `+0x180c` the spit's wait, `+0x1810` the crates' wait, `+0x1828` the health of the next stage, `+0x182c` the
//! stage (0..2), `+0x1830` the gulps, `+0x1834` the attack, `+0x1838` its reach, `+0x183c` the prey's class, `+0x1840`
//! the attack's count, `+0x1848` the fighters' wait, `+0x184c` the attacks not yet used (bits), `+0x1850` the pool's
//! marker, `+0x1854` its voice.
//!
//! **Each tick** ([`head`], `0x443200`): the set-up in state 0 (update and draw distances 0xff; health and most by the
//! fights lost, 1 → 0.5 from 5 to 20 ([`by_attempts`], `0x446090`); the movement block; the attacks 15; the first
//! stage's health a third down; the crates put away (`0x4443b0`); the classes 0x1231..0x1234 hidden (`0x445680`); on
//! the ground, mode 0x400, state 24 (sub 6), sequence 1; `+0x48` 15 and `+0x4a` 12; targeted; held where it is). Out
//! of sub 6: its scale; a hit (flags 0x10000) from a 0x05de, a 0x03a6 or a 0x0d0f does nothing; the damage off its
//! health; dead (or its status 2): once the scenes 0xa1 and 0xa0 are queued (its bolts unless asked), state 17 (sub
//! 6); else down to the next stage's health: hurt (15, sub 5) into the next stage (at the last, group `+0x244`
//! woken, [`wake`]); else hurt by the prize (the last hitter a 0x12d3): 15 (sub 5) with its timer set. Then the
//! target search (64, its area), and it is held where it was, its tilt cleared.
//!
//! **Sub 6** (asleep): in 24 / 25 Ratchet in its pool's cuboid makes the checkpoint ((176, 605, 385.83), yaw 1.57),
//! one more fight, 1 (sub 0); shown with its collision out of a scene, hidden in one.
//!
//! **States** (`+0x20`): **1** sequence 0x13 facing Ratchet; at its end the pick ([`pick`], `0x444158`). **2** the
//! roll (sequence 6, the chase's limits; its voice 1 looped): along its facing ([`roll`]) to its fence, then 3. **3**
//! the leap ([`leap_start`], [`leap`]): landed, with rolls left 2 (sub 1), else (the crates' wait cut short, the
//! crates brought back) the pick. **4** to its prey (the walk's limits, stop 9): gone → 1; within 9 or 301 ticks → 5.
//! **5** the grab (sequence 0xb): before frame 31 facing it (gone → 1); at frame 32 the prey eaten (crate put away,
//! the prize hidden), its class kept, playing at 0.7; at 50 at 1; from 60 the attack ([`choose`]) and 6 (sub 3).
//! **6** to Ratchet (stop half its reach): within the reach facing him to 5°, the attack. **7** the spit (sequence
//! 0x11): every 60 ticks a throw's draws (the spit 0x119d is not ported); after 600 ticks 1. **8** / **9** / **10** /
//! **11** (sequence 0x11, a count of 10..20 / 4..5 / 30..50 / 10..15): on their frame slices the throws — 9 a shooter
//! spat ([`shooter::launch`]), the others' classes not ported (their draws kept); the count out → 1. **12** the
//! biters thrown ([`biter::throw`], 60° of spin) on its frame slice until none is left or 21 are out → 13. **13**
//! to a spot 20 aside from the pool beside Ratchet (stop 1): there facing him → 14; under 3 biters out → 1. **14**
//! watching him (sequence 1): within 15 → 13; under 3 biters out → 1. **15** hurt (sequence 0x15, sunk to the
//! ground): at its end 1 (sub 0) when the prize hurt it, else 16. **16** the gulp (sequences 7 → 8 → 9): at frames
//! 7..9 of 8 and 12..30 of 9 after Ratchet, at 7..9 / 25..30 a 5.5 sphere a unit up (its gulp byte, flags 0x10008);
//! at frame 30 of 9 after two gulps the leap (3). **17** dead: the 0x0cbc gone, the fighters put away
//! ([`fighter::stow`]); hidden in a scene; at its end still. **24** wandering in its pool (to 5..15 about its centre,
//! stop 1.5): there facing Ratchet → 25. **25** a look about (sequence 0x13 or 0x14), then 24.
//!
//! **The tail** ([`tail`], `0x444ae8`): out of sub 6 the boss bar (picture 10); the fighters brought in
//! ([`fighters_in`]); the head's aim on its record `+0x250` (`0x339480` = L00 `0x324208`, list 0, 0.015 / 0.3:
//! crate::moby::joint_aim, the turn asked `+0x2b8`, the angle sprung `+0x298`); the crates' wait; held where it was
//! (its tilt, about `(0, 0, 5)` in its frame); its shadow, its ticks; the head aim (`0x33c3d8` = L00 `0x327308`, list
//! 0, 1.3). Out of state 2 its voice is let go.
//!
//! The engine functions with the port's: `0x330820` (set_state), `0x330540` (set_anim), `0x33faa8` (first),
//! `0x3267e0` (the frame time), `0x342e38` = L00 `0x32dcf0` (crate::moby::ai::senses), `0x332230` = L00 `0x31d180`
//! (ai::attack_sphere), `0x341d88` = L00 `0x32cd20` / `0x341b68` (L01 `0x322450`) / `0x341bc0` (L02 `0x331440`: the
//! options 0x10 for the tick) (ai::move_step), `0x342dd0` (L01 `0x323560`, the facing eased), `0x3596f0` = L00
//! `0x343678` (ai::fence), `0x338e28` = L00 `0x323bf8` (ai::reached), `0x340018` = L00 `0x32ad38` (ai::find_target),
//! `0x332e28` / `0x332e88` = L00 `0x31dce8` / `0x31dd48` (the hit, ai::damage_of), `0x331ed0` = L00 `0x31ce20`
//! (ai::ground_z), `0x336998` = L00 `0x321858` (in_cuboid), `0x33fb28` = L00 `0x32a848` (in_area), `0x36fbf0` = L00
//! `0x359b20` (the checkpoint), `0x309058` / `0x3091a0` = L00 `0x2f4ee8` / `0x2f5030` (the scene queue),
//! `0x32fd10` = L00 `0x31adc0` (crate::moby::bolts::drop), `0x310cf8` (L01 `0x2f20b8`, the boss bar), `0x332b98` =
//! L00 `0x31da58` (crate::shadow_volumes::probe), `0x3bb6f0` / `0x3bb760` (L02 `0x3a83f8` / `0x3a8468`,
//! crate::moby::classes::crate_::put_away / restore), `0x3766c0` / `0x3763c0` (the class sounds).
//!
//! Not ported: the scenes' end and the level's end (the callback `0x4461b0` and states 18..22: the scenes 0x1e /
//! 0x1f, the planets, the checkpoint at `+0x248`, the trip); the pool's marker 0x1316 (`0x3a0290`, `0x3a05d8`); the
//! eaten prey's flight to its mouth (`0x444cd8`, `0x444e50`); the spawns of 7 (0x119d, `0x4408b0`), 8 (L03
//! `0x3d09d0`), 10 (0x12ee, `0x44b518`) and 11 (0x12f7, `0x44c648`); the end of the gulp (`0x445720`) and Ratchet's
//! state 0x8c in it and in the roll and leap (pushed up, his `+0x1e0` / `+0x1f0`); the camera (`0x443dd8`) and its
//! shakes (`0x1bbe00`); the
//! spit's mouth particles (`0x34afd8`); the roll's touch of the classes 0x1366 / 0x1367; the crates' `+0x36`; the
//! arrivals' effects (`0x39d1b8`, `0x3007d0`); `0x1b2b68` (its height − 2 in sub 6); the moby word `+0x9c`; the
//! debug switches `$gp+0x3084..+0x30ac` (all 0). A prey is any moby here (the original asks for one of the level's).

use super::{biter, fighter, shooter};
use crate::hero::physics::{add, add_rot, approach, approach_angle, atan2, dist2, euler_rows, rot_dist, scale, sub, sub_rot, through, V3};
use crate::moby::classes::props::{set_seq, wrapped};
use crate::moby::{ai, bolts, countdown, mode, Ctx, Moby, MobyWorld};

pub const CLASS: i16 = 0x11e0;
/// The prize it eats for its attack 11.
pub const PRIZE: i16 = 0x12d3;
/// Removed when it dies.
const REMOVED: i16 = 0x0cbc;
/// Its hits do it no harm.
const HARMLESS: [i16; 3] = [0x05de, 0x03a6, 0x0d0f];
/// Hidden at its set-up.
const HIDDEN: std::ops::RangeInclusive<i16> = 0x1231..=0x1234;

mod pv {
    pub const HP: usize = 0x20;
    pub const MAX: usize = 0x24;
    pub const STATUS: usize = 0x2e;
    pub const F30: usize = 0x30;
    pub const GULP_DMG: usize = 0x34;
    pub const HITTER: usize = 0x38;
    pub const MV: usize = 0x50;
    pub const FENCE: usize = 0x210;
    pub const CRATES: usize = 0x218;
    pub const BITERS: usize = 0x21c;
    pub const AREA: usize = 0x220;
    pub const POOL: usize = 0x230;
    pub const ARENA: usize = 0x234;
    pub const PRIZE: usize = 0x238;
    pub const FIGHTERS: usize = 0x23c;
    pub const SHOOTERS: usize = 0x240;
    pub const WAKE: usize = 0x244;
    pub const AIM_YAW: usize = 0x298;
    pub const HEAD: usize = 0x2b8;
    /// The head's aim record (crate::moby::joint_aim; `HEAD` is its asked turn's z).
    pub const HEAD_AIM: usize = 0x250;
    pub const SPOT: usize = 0x17d0;
    pub const HOLD: usize = 0x17e0;
    pub const LAST: usize = 0x17f4;
    pub const ROLLS: usize = 0x17f8;
    pub const PREY: usize = 0x17fc;
    pub const TILT_X: usize = 0x1800;
    pub const TILT_Y: usize = 0x1804;
    pub const SPIT_FRAME: usize = 0x1808;
    pub const SPIT_WAIT: usize = 0x180c;
    pub const CRATE_WAIT: usize = 0x1810;
    pub const NEXT: usize = 0x1828;
    pub const STAGE: usize = 0x182c;
    pub const GULPS: usize = 0x1830;
    pub const ATTACK: usize = 0x1834;
    pub const REACH: usize = 0x1838;
    pub const PREY_CLASS: usize = 0x183c;
    pub const COUNT: usize = 0x1840;
    pub const FIGHTER_WAIT: usize = 0x1848;
    pub const UNUSED: usize = 0x184c;
    pub const VOICE: usize = 0x1854;
}
use pv::MV;

/// Level 20's globals the boss keeps.
#[derive(Clone, Copy, Debug, Default)]
pub struct Globals {
    /// `$gp+0x3060`: the fights begun (Ratchet into the pool's cuboid).
    pub attempts: f32,
}

/// °/s and °/s² as radians a tick and a tick².
const DEG_S: f32 = 0.000_290_888_25;
const DEG_S2: f32 = 4.848_137_5e-6;
const DEG5: f32 = f32::from_bits(0x3db2_b8c4);
/// `$gp+0x2ff0`: its scale over its class's (and the held point's).
const SCALE: f32 = 1.0;
/// `$gp+0x2ff4`.
const F30: f32 = 2.5;
/// `$gp+0x2ff8`: the walk to its prey stops this near.
const PREY_STOP: f32 = 9.0;
/// `$gp+0x3014`: its playback while it swallows.
const SWALLOW_SPEED: f32 = 0.7;
/// `$gp+0x3030` / `+0x3034`: the spit's wait and length (ticks).
const SPIT_EVERY: i32 = 60;
const SPIT_FOR: i32 = 600;
/// `$gp+0x3058`: the spot of 13 this far aside from the pool's centre; `+0x305c` within this of Ratchet 14 → 13.
const SPOT_OFF: f32 = 20.0;
const WATCH_NEAR: f32 = 15.0;
/// `$gp+0x3070`: its tilt's pivot in its frame.
const PIVOT: V3 = [0.0, 0.0, 5.0];
/// `$gp+0x30c0` / `0x1b20bc`: the checkpoint as the fight begins.
const CHECK_AT: V3 = [176.0, 605.0, f32::from_bits(0x43c0_ea3d)];
const CHECK_YAW: f32 = f32::from_bits(0x3fc8_f5c3);
/// `$gp+0x30d0..+0x30dc`: the chase's limits (/60, /3600, °/s², °/s).
const CHASE: (f32, f32, f32, f32) = (20.0, 10.0, 15.0, 15.0);
/// `$gp+0x30e4`: the roll's spin (°/s, by its speed); `+0x30e8` the leap's height; `+0x30ec` / `+0x30f0` its turn;
/// `+0x30f4` its bounce; `+0x30f8` its gravity (/3600).
const ROLL_SPIN: f32 = 180.0;
const LEAP_HEIGHT: f32 = 5.0;
const LEAP_TURN: (f32, f32) = (10.0, 30.0);
const BOUNCE: f32 = 0.4;
const LEAP_G: f32 = 40.0;
/// `$gp+0x30fc..+0x3108`: the walk's limits (/60, /3600, °/s, °/s²).
const WALK: (f32, f32, f32, f32) = (10.0, 30.0, 720.0, 720.0);
/// The attacks for the bits of `+0x184c`, and each stage's bits.
const ATTACKS: [(i32, i32); 4] = [(1, 7), (2, 10), (4, 9), (8, 8)];
const STAGE_BITS: [i32; 3] = [1, 6, 15];

fn seqs(w: &MobyWorld, id: usize, ctx: &Ctx) -> (u8, u8) { ctx.host.anim(&w.mobys[id]).map_or((0xff, 0xff), |a| (a.seq_a, a.seq_b)) }
fn settled(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool { let (a, b) = seqs(w, id, ctx); a == b }
fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 { let q = w.mobys[id].pos; atan2(p[0] - q[0], p[1] - q[1]) }
fn link(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { ai::link(&w.mobys[id], off).filter(|&k| k < w.mobys.len()) }
fn gone(s: u8) -> bool { 0x80 <= s || s == 6 }
fn cuboid_pos(ctx: &Ctx, i: i32) -> V3 {
    usize::try_from(i).ok().and_then(|i| ctx.level.cuboids.get(i)).map_or([0.0; 3], |c| [c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]])
}
fn speed(w: &MobyWorld, id: usize, ctx: &Ctx) -> f32 { ctx.host.anim(&w.mobys[id]).map_or(1.0, |a| a.speed) }

/// The members of group `g` (the record list `0x1fbaa0[g]`); none for −1 / 0xff.
pub(crate) fn members(w: &MobyWorld, g: i32) -> Vec<usize> {
    if g & 0xff == 0xff { return Vec::new(); }
    usize::try_from(g).ok().and_then(|g| w.groups.get(g)).map_or(Vec::new(), |l| l.iter().copied().take_while(|&k| k < w.mobys.len()).collect())
}

/// `0x4418d8(g)`: the members of `g` out of the pool (not state 6, not gone).
fn out_of_pool(w: &MobyWorld, g: i32) -> usize { members(w, g).into_iter().filter(|&k| !gone(w.mobys[k].state)).count() }

/// `0x446090(a, b, f0, f1)`: `f0` at `a` fights or fewer, `f1` at `b` or more, linear between.
fn by_attempts(w: &MobyWorld, a: i32, b: i32, f0: f32, f1: f32) -> f32 {
    let n = w.level20.attempts;
    if n <= a as f32 { return f0; }
    if b as f32 <= n { return f1; }
    f0 + (f1 - f0) * ((n - a as f32) / (b - a) as f32)
}

fn face_hero(w: &mut MobyWorld, id: usize, ctx: &Ctx) { let y = yaw_to(w, id, ctx.hero.pos); crate::moby::ai::face_at(&mut w.mobys[id], MV, y); }

/// Facing Ratchet to within 5°.
fn facing_hero(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool { rot_dist(w.mobys[id].rot[2], yaw_to(w, id, ctx.hero.pos)) < DEG5 }

/// `0x443688`: the chase's limits.
fn chase_senses(m: &mut Moby) { ai::senses(m, CHASE.1 * 0.000_277_777_8, CHASE.0 * 0.016_666_668, CHASE.2 * DEG_S2, CHASE.3 * DEG_S); }

/// `0x443c88(m, stop)`: the walk's limits and the stop distance.
fn walk_senses(m: &mut Moby, stop: f32) {
    ai::senses(m, WALK.1 * 0.000_277_777_8, WALK.0 * 0.016_666_668, WALK.3 * DEG_S2, WALK.2 * DEG_S);
    m.set_pv_f(MV + 0x34, stop);
}

/// `0x443d10(m, goal, yaw)`: a touch (its radius + 0.5, 50, flags 0x4010000) and the move: within the stop distance
/// along `yaw` (`0x341d88`), else to the goal (`0x341b68`).
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3, yaw: f32) {
    let m = &w.mobys[id];
    let (p, r, stop) = (m.pos, m.pv_f(MV), m.pv_f(MV + 0x34));
    ai::attack_sphere(w, id, ctx, r + 0.5, 50.0, 1.0, p, 0x401_0000, 0, 1, 0);
    let yaw = if dist2(p, goal) <= stop { yaw } else { yaw_to(w, id, goal) };
    ai::move_step(w, id, ctx, yaw, goal);
}

/// `0x4436f0`: a tick of the roll — facing Ratchet while he is within 90° of it; along its facing (`0x341bc0`, the
/// options 0x10 for the tick, toward the origin; Ratchet's collision off for it), its velocity kept to its facing;
/// its tilt spun by its speed; spheres of 3 a unit ahead (its gulp byte, flags 0x10008) and of 5.5 five up (50,
/// flags 0x10000); kept inside its fence path — true once it meets it.
fn roll(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let yh = yaw_to(w, id, ctx.hero.pos);
    if rot_dist(w.mobys[id].rot[2], yh) < std::f32::consts::FRAC_PI_2 { crate::moby::ai::face_at(&mut w.mobys[id], MV, yh); }
    let hm = ctx.hero_moby.filter(|&h| h < w.mobys.len());
    let hc = hm.map(|h| w.mobys[h].has_collision);
    if let Some(h) = hm { w.mobys[h].has_collision = false; }
    let yaw = w.mobys[id].rot[2];
    ai::move_with(w, id, ctx, yaw, [0.0; 3], 0x10, -1.0, -1.0);
    // The original gives him his class's collision back; the port gives back what he had.
    if let (Some(h), Some(c)) = (hm, hc) { w.mobys[h].has_collision = c; }
    let m = &mut w.mobys[id];
    let rows = m.rows;
    let v = m.pv_v(MV + 0xa0);
    let local = [crate::hero::physics::dot(v, rows[0]), 0.0, crate::hero::physics::dot(v, rows[2])];
    m.set_pv_v(MV + 0xa0, through(&rows, local));
    let f = ((m.pv_f(MV + 0xb0) + m.pv_f(MV + 0xb0)) / m.pv_f(MV + 0x28)).max(0.5);
    let t = add_rot(m.pv_f(pv::TILT_Y), ROLL_SPIN * f * DEG_S);
    m.set_pv_f(pv::TILT_Y, t);
    let (p, ahead, dmg) = (m.pos, add(rows[0], m.pos), m.pv_b(pv::GULP_DMG) as f32);
    ai::attack_sphere(w, id, ctx, 3.0, dmg, 2.0, ahead, 0x1_0008, 0, 1, 0);
    ai::attack_sphere(w, id, ctx, 5.5, 50.0, 2.0, [p[0], p[1], p[2] + 5.0], 0x1_0000, 0, 1, 0);
    let fence = usize::try_from(w.mobys[id].pv_i(pv::FENCE)).ok().and_then(|i| ctx.level.paths.get(i));
    let r = w.mobys[id].pv_f(MV);
    match fence.and_then(|pts| ai::fence(pts, r, w.mobys[id].pos)) {
        Some(q) => {
            w.mobys[id].pos = q;
            true
        }
        None => false,
    }
}

/// `0x4439e8`: the leap's start — no walk, its turn; gravity 40/3600; the ground's height; still, then up at
/// `√(2·g·5)`.
fn leap_start(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    ai::senses(m, 0.0, 0.0, LEAP_TURN.0 * DEG_S2, LEAP_TURN.1 * DEG_S);
    let g = LEAP_G * 0.000_277_777_8;
    m.set_pv_f(MV + 0x2c, g);
    let z = ai::ground_z(ctx, 0.5, m.pos, 0);
    let m = &mut w.mobys[id];
    m.set_pv_f(MV + 0x90, z);
    m.set_pv_v(MV + 0xa0, [0.0; 3]);
    m.set_pv_f(MV + 0xac, 0.0);
    m.set_pv_f(MV + 0xa8, ((g + g) * LEAP_HEIGHT).sqrt());
}

/// `0x443a98`: a tick of the leap — its playback eased to 1 (1/30); from the roll its tilt eased upright (90°/s); up
/// and down under its gravity, facing Ratchet; at the ground its class sound 0 and a bounce of 0.4 — true when that
/// is under 1/60.
fn leap(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let mut s = speed(w, id, ctx);
    approach(1.0, f32::from_bits(0x3d08_8889), &mut s);
    ctx.host.set_anim_speed(&w.mobys[id], s);
    let m = &mut w.mobys[id];
    if m.prev_state == 2 {
        let mut t = m.pv_f(pv::TILT_Y);
        approach_angle(0.0, ROLL_SPIN * 0.000_145_444_12, &mut t, 1);
        m.set_pv_f(pv::TILT_Y, t);
    }
    let vz = m.pv_f(MV + 0xa8);
    m.pos[2] += vz;
    m.set_pv_f(MV + 0xa8, vz - m.pv_f(MV + 0x2c));
    face_hero(w, id, ctx);
    let m = &mut w.mobys[id];
    let floor = m.pv_f(MV + 0x90);
    if m.pos[2] <= floor {
        m.pos[2] = floor;
        ctx.host.play_sound(&w.mobys[id], 0, 0);
        let m = &mut w.mobys[id];
        let v = m.pv_f(MV + 0xa8) * -BOUNCE;
        m.set_pv_f(MV + 0xa8, v);
        if v < 0.016_666_668 {
            m.set_pv_f(MV + 0xa8, 0.0);
            return true;
        }
    }
    false
}

/// `0x444a50(m, prey)`: the prey in its area and still there (a crate 0x1ff / 0x200 not put away; the prize out and
/// not taken).
fn prey_ok(w: &MobyWorld, id: usize, ctx: &Ctx, prey: usize) -> bool {
    let p = &w.mobys[prey];
    if !ctx.level.in_area(w.mobys[id].pv_i(pv::AREA), p.pos) { return false; }
    match p.o_class {
        0x1ff | 0x200 => !gone(p.state),
        PRIZE => p.state != 1 && p.pv_i(0xac) == 0,
        _ => false,
    }
}

/// `0x4448f8`: its prey — the prize when out and not taken, else the last crate (`+0x218`) not put away within
/// 200 (the original keeps the first nearest by its own compare).
fn find_prey(w: &MobyWorld, id: usize) -> Option<usize> {
    let m = &w.mobys[id];
    if let Some(p) = usize::try_from(m.pv_i(pv::PRIZE)).ok().filter(|&p| p < w.mobys.len()) {
        let o = &w.mobys[p];
        if o.state != 1 && o.pv_i(0xac) == 0 { return Some(p); }
    }
    let mut best = None;
    let mut near = 200.0;
    for k in members(w, m.pv_i(pv::CRATES)) {
        if gone(w.mobys[k].state) { continue; }
        let d = dist2(w.mobys[k].pos, m.pos);
        if d < near {
            near = d;
            best = Some(k);
        }
    }
    best
}

/// `0x444158`: the pick — after a pick from sub 1 its prey (4, sub 2; none: 12, sub 4); else 2..3 rolls, the
/// crates' wait 180..360 and 3 (sub 1). The sub-state kept.
fn pick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(pv::LAST) == 1 {
        let p = find_prey(w, id);
        let m = &mut w.mobys[id];
        ai::set_link(m, pv::PREY, p);
        if p.is_some() { m.set_state(4, 2); } else { m.set_state(12, 4); }
    } else {
        let n = ctx.game.rng.randi(2) + 2;
        let t = ctx.game.rng.rand_range(0xb4, 0x168);
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::ROLLS, n);
        m.set_pv_i(pv::CRATE_WAIT, t);
        m.set_state(3, 1);
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::LAST, m.sub_state as i32);
}

/// `0x444220`: the attack after a meal — reach 50; the prize eaten: 11; else one of the stage's attacks not yet
/// used (all of them used: any of the stage's), from a random one of the four on (7 / 10 / 9 / 8; 10 with reach 30
/// only when forced).
fn choose(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_pv_f(pv::REACH, 50.0);
    if m.pv_i(pv::PREY_CLASS) == PRIZE as i32 {
        m.set_pv_i(pv::ATTACK, 11);
        return;
    }
    let mut bits = STAGE_BITS.get(m.pv_i(pv::STAGE) as usize).copied().unwrap_or(0);
    let left = m.pv_i(pv::UNUSED) & bits;
    if left != 0 { bits = left; }
    let r = ctx.game.rng.randi(4);
    let m = &mut w.mobys[id];
    for i in 0..4 {
        let bit = 1 << ((r + i) % 4);
        if bit & bits == 0 { continue; }
        m.set_pv_i(pv::UNUSED, m.pv_i(pv::UNUSED) & !bit);
        if let Some(&(_, a)) = ATTACKS.iter().find(|(b, _)| *b == bit) { m.set_pv_i(pv::ATTACK, a); }
        break;
    }
}

/// `0x444f58(m, out)`: a spot 20 from the pool's centre, a quarter turn from the boss's way to it, on Ratchet's side.
fn spot(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    let c = cuboid_pos(ctx, w.mobys[id].pv_i(pv::POOL));
    let yh = yaw_to(w, id, ctx.hero.pos);
    let mut h = yaw_to(w, id, c);
    h = if 0.0 < sub_rot(yh, h) { sub_rot(h, std::f32::consts::FRAC_PI_2) } else { add_rot(h, std::f32::consts::FRAC_PI_2) };
    [h.cos() * SPOT_OFF + c[0], h.sin() * SPOT_OFF + c[1], c[2]]
}

/// `0x4450a8(m, &pos, &vel, &f)`: a throw's place `(5, r, 5.5)` and velocity `(5..10, r, 4..6)` a second (r in −1..1)
/// in its frame turned by half of `+0x298`; `f` × r.
fn throw_origin(w: &MobyWorld, id: usize, ctx: &mut Ctx, f: f32) -> (V3, V3, f32) {
    let r = ctx.game.rng.randf(-1.0, 1.0);
    let m = &w.mobys[id];
    let turn = euler_rows([0.0, 0.0, m.pv_f(pv::AIM_YAW) * 0.5]);
    let basis = turn.map(|row| through(&m.rows, row));
    let pos = add(through(&basis, [5.0, r, 5.5]), m.pos);
    let x = ctx.game.rng.randf(5.0, 10.0);
    let z = ctx.game.rng.randf(4.0, 6.0);
    let vel = scale(through(&basis, [x, r, z]), 0.016_666_668);
    (pos, vel, f * r)
}

/// `0x445ff0(m, p)`: the head's turn toward `p`, within ±60° (asked of its aim, crate::moby::joint_aim).
fn head_at(w: &mut MobyWorld, id: usize, p: V3) {
    let t = sub_rot(yaw_to(w, id, p), w.mobys[id].rot[2]).clamp(-1.047_197_6, 1.047_197_6);
    w.mobys[id].set_pv_f(pv::HEAD, t);
}

/// `0x39fc58(g, −1)`: group `g`'s members in state 1 shown with their collision, 2.
fn wake(w: &mut MobyWorld, g: i32) {
    for k in members(w, g) {
        if w.mobys[k].state != 1 { continue; }
        let c = w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision);
        let m = &mut w.mobys[k];
        m.mode = (m.mode & 0xffbe) | mode::TARGET;
        m.has_collision = c;
        m.set_state(2, -1);
    }
    w.changed = true;
}

/// `0x4444c8`: a crate or the prize back at one of the arena's points the stage allows (0 the first and fourth, 1 the
/// others, 2 any) farther than 3 from Ratchet and 6 from the boss, clear of the world (0.5 at 0.6 over the ground),
/// the nearest the camera's yaw; the prize from stage 1 one time in two.
fn bring_back(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> Option<usize> {
    let m = &w.mobys[id];
    let stage = m.pv_i(pv::STAGE);
    let pts = usize::try_from(m.pv_i(pv::ARENA)).ok().and_then(|i| ctx.level.paths.get(i))?.clone();
    let rows = ctx.host.camera_rows();
    let (cam, cam_yaw) = (ctx.host.camera_pos(), rows[0][1].atan2(rows[0][0]));
    let mut limit = std::f32::consts::PI;
    let mut best = None;
    for i in 0..6.min(pts.len()) {
        let edge = i == 0 || i == 3;
        if (stage == 0 && !edge) || (stage == 1 && edge) { continue; }
        let p = [pts[i][0], pts[i][1], pts[i][2]];
        if dist2(p, ctx.hero.pos) <= 3.0 || dist2(p, w.mobys[id].pos) <= 6.0 { continue; }
        let a = rot_dist(cam_yaw, atan2(p[0] - cam[0], p[1] - cam[1]));
        if limit <= a { continue; }
        let q = [p[0], p[1], ai::ground_z(ctx, 0.5, p, 0) + f32::from_bits(0x3f19_999a)];
        let blocked = ctx.coll.is_some_and(|c| crate::collision::coll_sphere_m(c, ctx.scene, q, 0.5, crate::collision::QueryFlags(0), None).is_some());
        if !blocked {
            limit = a;
            best = Some(p);
        }
    }
    let p = best?;
    let at = [p[0], p[1], ai::ground_z(ctx, 0.5, p, 0)];
    if stage != 0 && ctx.game.rng.randi(2) != 0 {
        // `0x444870`: the prize back (`0x44af30`: 6 seconds, the boss its owner, not taken, shown, half a unit up, 2).
        let k = usize::try_from(w.mobys[id].pv_i(pv::PRIZE)).ok().filter(|&k| k < w.mobys.len())?;
        if w.mobys[k].state != 1 { return None; }
        let c = w.class(PRIZE).is_some_and(|c| c.has_collision);
        let m = &mut w.mobys[k];
        if m.pvars.len() < 0xb0 { m.pvars.resize(0xb0, 0); }
        m.set_pv_i(0xa8, 360);
        ai::set_link(m, 0xa4, Some(id));
        m.set_pv_i(0xac, 0);
        m.mode = (m.mode & 0xffbe) | mode::TARGET;
        m.has_collision = c;
        m.pos = [at[0], at[1], at[2] + 0.5];
        m.set_pv_v(0x50, [0.0; 3]);
        m.set_pv_f(0x5c, 0.0);
        m.set_state(2, -1);
        w.changed = true;
        return Some(k);
    }
    // `0x444728`: the group's first crate, when put away, back there (`+0xac` | 8).
    let k = *members(w, w.mobys[id].pv_i(pv::CRATES)).first()?;
    if !gone(w.mobys[k].state) { return None; }
    w.mobys[k].pos = at;
    crate::moby::classes::crate_::restore(w, k);
    let m = &mut w.mobys[k];
    if 0xb0 <= m.pvars.len() { m.set_pv_i(0xac, m.pv_i(0xac) | 8); }
    Some(k)
}

/// `0x443200` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let k = by_attempts(w, 5, 20, 1.0, 0.5);
        let z0 = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::VOICE, -1);
        m.update_dist = 0xff;
        m.draw_dist = 0xff;
        m.set_pv_f(pv::HP, m.pv_f(pv::HP) * k);
        let most = (m.pv_s(pv::MAX) as f32 * k) as i32;
        m.set_pv_s(pv::MAX, most as i16);
        m.set_pv_f(MV + 4, 0.5);
        m.set_pv_f(MV, 4.0);
        m.set_pv_f(MV + 0x90, z0);
        m.set_pv_f(MV + 8, 2.0);
        m.set_pv_f(MV + 0xc, 2.0);
        m.set_pv_i(pv::UNUSED, 15);
        m.set_pv_f(MV + 0x10, f32::from_bits(0x4049_0fdb));
        let third = (2.0 - m.pv_i(pv::STAGE) as f32) * (most as f32 * f32::from_bits(0x3eaa_aa3b));
        m.set_pv_f(pv::NEXT, third);
        for k in members(w, w.mobys[id].pv_i(pv::CRATES)) { crate::moby::classes::crate_::put_away(w, k, ctx); }
        for m in w.mobys.iter_mut().filter(|m| HIDDEN.contains(&m.o_class) && m.state != 0xff) {
            m.drawn = 0;
            m.mode |= 1;
        }
        let m = &mut w.mobys[id];
        m.pos[2] += 5.0;
        let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
        let m = &mut w.mobys[id];
        m.pos[2] = z;
        m.mode |= 0x400;
        m.set_state(24, 6);
        if crate::moby::classes::props::seq(w, id, ctx) != 1 { ctx.host.set_anim(&w.mobys[id], 1, 0, 0); }
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 12);
        m.mode |= mode::TARGET;
        let p = m.pos;
        m.set_pv_v(pv::HOLD, p);
    }
    if w.mobys[id].sub_state == 6 { return; }
    let s = w.class(CLASS).map_or(1.0, |c| c.scale);
    let m = &mut w.mobys[id];
    m.scale = s * SCALE;
    m.set_pv_f(pv::F30, F30);
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
    if hit.as_ref().and_then(|h| h.tmpl.source).and_then(|s| w.mobys.get(s)).is_some_and(|s| HARMLESS.contains(&s.o_class)) { w.clear_hit(id); }
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
    let h = ai::damage_of(w, id, hit.as_ref(), false);
    let m = &mut w.mobys[id];
    let mut hp = m.pv_f(pv::HP);
    approach(0.0, h, &mut hp);
    m.set_pv_f(pv::HP, hp);
    w.clear_hit(id);
    let m = &mut w.mobys[id];
    if hp <= 0.0 || m.pv_b(pv::STATUS) == 2 {
        m.set_pv_f(pv::HP, 0.0);
        if ctx.game.scenes.queue.push(0xa1) {
            if w.mobys[id].pv_b(pv::STATUS) != 2 { bolts::drop(w, id, 0, 3, ctx); }
            ctx.game.scenes.queue.push(0xa0);
            ctx.game.scenes.start_queue(ctx.hero.hp);
            w.mobys[id].set_state(17, 6);
        }
    } else if hp <= m.pv_f(pv::NEXT) {
        m.set_state(15, 5);
        let stage = m.pv_i(pv::STAGE) + 1;
        m.set_pv_i(pv::STAGE, stage);
        m.set_pv_f(pv::TILT_X, 0.0);
        m.set_pv_f(pv::TILT_Y, 0.0);
        let most = m.pv_s(pv::MAX) as f32;
        m.set_pv_f(pv::NEXT, (2.0 - stage as f32) * (most * f32::from_bits(0x3eaa_aa3b)));
        if stage == 2 { let g = m.pv_i(pv::WAKE); wake(w, g); }
    } else if h != 0.0 && link(w, id, pv::HITTER).is_some_and(|k| w.mobys[k].o_class == PRIZE) {
        let m = &mut w.mobys[id];
        m.set_state(15, 5);
        m.set_pv_f(pv::TILT_X, 0.0);
        m.set_pv_f(pv::TILT_Y, 0.0);
        m.timer = 1;
    }
    w.mobys[id].set_pv_b(pv::STATUS, 1);
    let area = w.mobys[id].pv_i(pv::AREA);
    ai::find_target(w, id, ctx, 64.0, 0.0, area, None, false);
    let m = &mut w.mobys[id];
    m.pos = m.pv_v(pv::HOLD);
    if m.rot[0] != 0.0 || m.rot[1] != 0.0 {
        m.rot[0] = 0.0;
        m.rot[1] = 0.0;
        m.rows = euler_rows(m.rot);
    }
}

/// `0x443f38`: from stage 1 (not in sub 5), none of the fighters out and their wait over: the wait again (0 → 60
/// seconds by the fights, + 1200 ticks), and 4 fighters (6 at the last stage) brought in at points along the arena's
/// sides (on the ground, a 0.9 sphere a unit up clear of the world), 30 tries at most.
fn fighters_in(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    if m.pv_i(pv::STAGE) < 1 || m.sub_state == 5 { return; }
    let g = m.pv_i(pv::FIGHTERS);
    if members(w, g).into_iter().any(|k| w.mobys[k].state != 6) { return; }
    let mut t = w.mobys[id].pv_i(pv::FIGHTER_WAIT);
    let over = countdown(&mut t);
    w.mobys[id].set_pv_i(pv::FIGHTER_WAIT, t);
    if !over { return; }
    let f = by_attempts(w, 5, 20, 0.0, 60.0);
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::FIGHTER_WAIT, (((f * 60.0) as i32) as f32) as i32 + 1200);
    let want = if m.pv_i(pv::STAGE) == 2 { 6 } else { 4 };
    let Some(pts) = usize::try_from(m.pv_i(pv::ARENA)).ok().and_then(|i| ctx.level.paths.get(i)).cloned() else { return };
    if pts.len() < 6 { return; }
    let (mut made, mut tries) = (0, 0);
    loop {
        let i = ctx.game.rng.randi(6) as usize;
        let t = ctx.game.rng.randf(f32::from_bits(0x3e4c_cccd), f32::from_bits(0x3f4c_cccd));
        let (a, b) = (pts[i], pts[(i + 1) % 6]);
        let mut p: V3 = std::array::from_fn(|k| a[k] + (b[k] - a[k]) * t);
        p[2] = ai::ground_z(ctx, 0.5, p, 0) + 1.0;
        let blocked = ctx.coll.is_some_and(|c| crate::collision::coll_sphere_m(c, ctx.scene, p, f32::from_bits(0x3f66_6666), crate::collision::QueryFlags(0), None).is_some());
        if !blocked {
            p[2] -= 1.0;
            made += 1;
            if fighter::bring_in(w, id, ctx, g, p).is_none() { return; }
        }
        tries += 1;
        if 29 < tries || want <= made { return; }
    }
}

/// `0x444ae8` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].sub_state != 6 {
        let m = &w.mobys[id];
        ctx.game.boss_bar = Some((m.pv_f(pv::HP) / m.pv_s(pv::MAX) as f32, 5));
        ctx.game.boss_icon = 10;
        fighters_in(w, id, ctx);
    }
    // `0x339480` = L00 `0x324208`: the head's aim (list 0, 0.015 / 0.3; its turn `+0x2b8`).
    crate::moby::joint_aim::aim(w, id, pv::HEAD_AIM, id, 0, f32::from_bits(0x3c75_c28f), f32::from_bits(0x3e99_999a));
    if 0 < w.mobys[id].pv_i(pv::CRATE_WAIT) {
        let mut t = w.mobys[id].pv_i(pv::CRATE_WAIT);
        let over = countdown(&mut t);
        w.mobys[id].set_pv_i(pv::CRATE_WAIT, t);
        if over {
            bring_back(w, id, ctx);
            w.mobys[id].set_pv_i(pv::CRATE_WAIT, -1);
        }
    }
    let m = &mut w.mobys[id];
    let p = m.pos;
    m.set_pv_v(pv::HOLD, p);
    let (tx, ty) = (m.pv_f(pv::TILT_X), m.pv_f(pv::TILT_Y));
    if tx != 0.0 || ty != 0.0 {
        m.rot[0] = tx;
        m.rot[1] = ty;
        let before = through(&m.rows, PIVOT);
        m.rows = euler_rows(m.rot);
        let after = through(&m.rows, PIVOT);
        m.pos = sub(m.pos, sub(after, before));
    }
    crate::shadow_volumes::probe(w, id, ctx);
    let m = &mut w.mobys[id];
    m.shadow_dir = 2;
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = m.state_ticks.wrapping_add(1); }
    // `0x33c3d8` = L00 `0x327308`: the head aim (list 0, `$gp+0x3124` = 1.3).
    crate::moby::joint_aim::head(w, ctx, id, pv::HEAD_AIM, 0, f32::from_bits(0x3fa6_6666));
    w.changed = true;
}

/// Its voice let go.
fn hush(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(pv::VOICE);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(pv::VOICE, -1);
}

/// 0x11e0 (module docs).
pub fn update_11e0(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.variant != 2 { return; }
    head(w, id, ctx);
    let in_scene = ctx.game.scenes.mode == crate::scene::SCENE;
    if w.mobys[id].sub_state == 6 && matches!(w.mobys[id].state, 24 | 25) {
        if ctx.level.in_cuboid(w.mobys[id].pv_i(pv::POOL), ctx.hero.pos) {
            ctx.game.save_checkpoint(CHECK_AT, [0.0, 0.0, CHECK_YAW]);
            w.level20.attempts += 1.0;
            w.mobys[id].set_state(1, 0);
        }
        let c = w.class(CLASS).is_some_and(|c| c.has_collision);
        let m = &mut w.mobys[id];
        if in_scene {
            m.has_collision = false;
            m.mode = (m.mode | 0x41) & 0xefff;
        } else {
            m.mode = (m.mode & 0xffbe) | mode::TARGET;
            m.has_collision = c;
        }
    }
    let hero = ctx.hero.pos;
    let ticks = w.mobys[id].state_ticks as u16;
    let frame = ctx.game.frame as u32;
    let slice = |mask: u32| (frame & mask) == (id as u32 & mask);
    match w.mobys[id].state {
        1 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0x13, 0, 0x14); }
            face_hero(w, id, ctx);
            if wrapped(w, id, ctx) { pick(w, id, ctx); }
        }
        2 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 6, 0, 0x14);
                chase_senses(&mut w.mobys[id]);
            }
            let h = w.mobys[id].pv_i(pv::VOICE);
            if h == -1 || !ctx.host.sound_playing(h) {
                let v = ctx.host.play_sound(&w.mobys[id], 1, 4);
                w.mobys[id].set_pv_i(pv::VOICE, v);
            }
            if roll(w, id, ctx) {
                w.mobys[id].set_state(3, -1);
                hush(w, id, ctx);
            }
        }
        3 => {
            if w.mobys[id].first() {
                leap_start(w, id, ctx);
                set_seq(w, id, ctx, 6, 0, 0x14);
            }
            let m = &mut w.mobys[id];
            let mut t = m.pv_f(pv::TILT_Y);
            approach_angle(0.0, f32::from_bits(0x3dd6_7752), &mut t, 0);
            m.set_pv_f(pv::TILT_Y, t);
            if leap(w, id, ctx) {
                let out = ctx.hero.state != 0x8c && {
                    let mut n = w.mobys[id].pv_i(pv::ROLLS);
                    let o = countdown(&mut n);
                    w.mobys[id].set_pv_i(pv::ROLLS, n);
                    o
                };
                if out {
                    if 0 < w.mobys[id].pv_i(pv::CRATE_WAIT) {
                        w.mobys[id].set_pv_i(pv::CRATE_WAIT, -1);
                        bring_back(w, id, ctx);
                    }
                    pick(w, id, ctx);
                } else {
                    w.mobys[id].set_state(2, 1);
                }
            }
        }
        4 => {
            if w.mobys[id].first() {
                walk_senses(&mut w.mobys[id], PREY_STOP);
                set_seq(w, id, ctx, 2, 0, 0x14);
            }
            match link(w, id, pv::PREY) {
                Some(p) => {
                    let at = w.mobys[p].pos;
                    let y = yaw_to(w, id, at);
                    walk(w, id, ctx, at, y);
                    if !prey_ok(w, id, ctx, p) {
                        w.mobys[id].set_state(1, -1);
                    } else if w.mobys[id].pv_f(MV + 0xcc) <= PREY_STOP || 0x12d <= ticks {
                        w.mobys[id].set_state(5, -1);
                    }
                }
                None => w.mobys[id].set_state(1, -1),
            }
        }
        5 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0xb, 0, 0xf); }
            let f = ctx.host.frame_time(&w.mobys[id]);
            let prey = link(w, id, pv::PREY);
            if f < 31.0 {
                match prey {
                    Some(p) => {
                        let y = yaw_to(w, id, w.mobys[p].pos);
                        crate::moby::ai::face_at(&mut w.mobys[id], MV, y);
                        if !prey_ok(w, id, ctx, p) { w.mobys[id].set_state(1, -1); }
                    }
                    None => w.mobys[id].set_state(1, -1),
                }
            } else if ai::reached(w, id, ctx, 32.0) {
                if let Some(p) = prey {
                    // `0x444cd8`: its copy's flight to the held point (`(0, 0, 7)` in its frame) is not ported.
                    let class = w.mobys[p].o_class;
                    let m = &mut w.mobys[id];
                    ai::set_link(m, pv::PREY, None);
                    m.set_pv_i(pv::PREY_CLASS, class as i32);
                    // `0x444478`: a crate put away, the prize hidden (`0x44afc0`).
                    if (500..541).contains(&class) {
                        crate::moby::classes::crate_::put_away(w, p, ctx);
                    } else if class == PRIZE {
                        let o = &mut w.mobys[p];
                        o.set_state(1, -1);
                        o.has_collision = false;
                        o.mode = (o.mode & 0xefff) | 0x41;
                    }
                    ctx.host.set_anim_speed(&w.mobys[id], SWALLOW_SPEED);
                }
            } else if ai::reached(w, id, ctx, 50.0) {
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
            if 60.0 <= ctx.host.frame_time(&w.mobys[id]) {
                choose(w, id, ctx);
                w.mobys[id].set_state(6, 3);
            }
        }
        6 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 2, 0, 0x14);
                let r = w.mobys[id].pv_f(pv::REACH) * 0.5;
                walk_senses(&mut w.mobys[id], r);
            }
            let y = yaw_to(w, id, hero);
            walk(w, id, ctx, hero, y);
            if w.mobys[id].pv_f(MV + 0xcc) < w.mobys[id].pv_f(pv::REACH) && facing_hero(w, id, ctx) {
                let a = w.mobys[id].pv_i(pv::ATTACK);
                w.mobys[id].set_state(a as u8, -1);
            }
        }
        7 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 0x11, 0, 0x14);
                w.mobys[id].set_pv_i(pv::SPIT_WAIT, 0);
            }
            face_hero(w, id, ctx);
            w.mobys[id].set_pv_i(pv::SPIT_FRAME, ctx.game.frame as i32);
            // The mouth's two particles (`0x3217d8`'s jitter of 1.5, `0x34afd8`) are not ported; the jitter's and the
            // size's draws are kept (the particles' own are not).
            for _ in 0..2 {
                for _ in 0..3 { let _ = ctx.game.rng.randf(-1.5, 1.5); }
                let _ = ctx.game.rng.randf(2.0, 3.0);
            }
            if settled(w, id, ctx) {
                let mut t = w.mobys[id].pv_i(pv::SPIT_WAIT);
                let over = countdown(&mut t);
                w.mobys[id].set_pv_i(pv::SPIT_WAIT, t);
                if over {
                    w.mobys[id].set_pv_i(pv::SPIT_WAIT, SPIT_EVERY);
                    // `0x4408b0`: the spit 0x119d is not ported; the throw's draws are kept.
                    let _ = throw_origin(w, id, ctx, 0.0);
                    let _ = ctx.game.rng.randi(2);
                }
            }
            if SPIT_FOR < ticks as i32 { w.mobys[id].set_state(1, -1); }
        }
        8..=11 => {
            let s = w.mobys[id].state;
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 0x11, 0, 0x14);
                let (a, b) = match s { 8 => (10, 20), 9 => (4, 5), 10 => (0x1e, 0x32), _ => (10, 15) };
                let n = ctx.game.rng.rand_range(a, b);
                w.mobys[id].set_pv_i(pv::COUNT, n);
            }
            if s == 11 { head_at(w, id, hero); }
            let on = settled(w, id, ctx) && match s { 8 | 9 => slice(0x1f), 10 => slice(7), _ => ticks % 60 == 0x1e };
            if on {
                let mut more = true;
                match s {
                    9 => {
                        // `0x445560`: a shooter spat from `+0x240`.
                        let (p, v, _) = throw_origin(w, id, ctx, f32::from_bits(0x3f49_0fdc));
                        let g = w.mobys[id].pv_i(pv::SHOOTERS);
                        more = shooter::launch(w, id, ctx, g, p, v).is_some();
                    }
                    8 => {
                        // `0x445338`: L03's `0x3d09d0` is not ported; its draws are kept.
                        let _ = throw_origin(w, id, ctx, f32::from_bits(0x3f49_0fdc));
                        if ctx.game.rng.randi(2) == 0 { let _ = ctx.game.rng.randf(0.4, 0.6); }
                    }
                    10 => {
                        // `0x4452a0`: the 0x12ee is not ported; its draws are kept.
                        let _ = throw_origin(w, id, ctx, f32::from_bits(0x3f86_0a92));
                        let _ = ctx.game.rng.rand_range(0x12c, 0x258);
                    }
                    _ => {
                        // `0x4455d0`: the 0x12f7 is not ported; its draws are kept.
                        let _ = throw_origin(w, id, ctx, f32::from_bits(0x3f49_0fdc));
                        let _ = ctx.game.rng.rand_angle();
                        let _ = ctx.game.rng.randf(f32::from_bits(0x3d56_7750), f32::from_bits(0x3dd6_7750));
                    }
                }
                if more {
                    let mut n = w.mobys[id].pv_i(pv::COUNT);
                    countdown(&mut n);
                    w.mobys[id].set_pv_i(pv::COUNT, n);
                } else {
                    w.mobys[id].set_pv_i(pv::COUNT, 0);
                }
            }
            if w.mobys[id].pv_i(pv::COUNT) == 0 { w.mobys[id].set_state(1, -1); }
        }
        12 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0x11, 0, 0x14); }
            if slice(7) {
                // `0x445238`: a biter thrown with 60° of spin.
                let (p, v, spin) = throw_origin(w, id, ctx, f32::from_bits(0x3f86_0a93));
                let g = w.mobys[id].pv_i(pv::BITERS);
                let thrown = biter::throw(w, ctx, g, p, v, spin);
                if thrown.is_none() || 0x15 <= out_of_pool(w, g) { w.mobys[id].set_state(13, -1); }
            }
        }
        13 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 2, 0, 0x14);
                let s = spot(w, id, ctx);
                w.mobys[id].set_pv_v(pv::SPOT, s);
                walk_senses(&mut w.mobys[id], 1.0);
            }
            let s = w.mobys[id].pv_v(pv::SPOT);
            let y = yaw_to(w, id, hero);
            walk(w, id, ctx, s, y);
            if w.mobys[id].pv_f(MV + 0xcc) < 1.0 && facing_hero(w, id, ctx) {
                w.mobys[id].set_state(14, -1);
            } else if out_of_pool(w, w.mobys[id].pv_i(pv::BITERS)) < 3 {
                w.mobys[id].set_state(1, -1);
            }
        }
        14 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 1, 0, 0x14); }
            head_at(w, id, hero);
            if dist2(w.mobys[id].pos, hero) < WATCH_NEAR { w.mobys[id].set_state(13, -1); }
            if out_of_pool(w, w.mobys[id].pv_i(pv::BITERS)) < 3 { w.mobys[id].set_state(1, -1); }
        }
        15 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 0x15, 0, 0xa); }
            let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
            let m = &mut w.mobys[id];
            approach(z, f32::from_bits(0x3e2a_aaab), &mut m.pos[2]);
            if wrapped(w, id, ctx) {
                let m = &mut w.mobys[id];
                if m.timer != 0 { m.set_state(1, 0); } else { m.set_state(0x10, -1); }
                m.timer = 0;
            }
        }
        16 => gulp(w, id, ctx),
        17 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 0x15, 0, 0xa);
                let biters = w.mobys[id].pv_i(pv::BITERS);
                for k in 0..w.mobys.len() {
                    let o = &w.mobys[k];
                    if o.state == 0xfe || o.state == 0xfd { continue; }
                    if o.o_class == REMOVED {
                        w.delete(k);
                    } else if o.o_class == 0x0d0f && o.group as i32 == biters && o.update.is_some_and(|f| f as usize == fighter::update_12f3 as crate::moby::UpdateFn as usize) {
                        // Only the fighters (class 0x0d0f since their set-up) are put away here; the original
                        // takes any 0x0d0f of the group.
                        bolts::drop(w, k, 0, 3, ctx);
                        fighter::stow(w, k);
                    }
                }
            }
            if in_scene {
                let m = &mut w.mobys[id];
                m.has_collision = false;
                m.mode = (m.mode | 0x41) & 0xefff;
            }
            if wrapped(w, id, ctx) { ctx.host.set_anim_speed(&w.mobys[id], 0.0); }
        }
        24 => {
            if w.mobys[id].first() {
                let r = ctx.game.rng.randf(5.0, 15.0);
                let a = ctx.game.rng.rand_angle();
                let c = cuboid_pos(ctx, w.mobys[id].pv_i(pv::POOL));
                w.mobys[id].set_pv_v(pv::SPOT, [a.cos() * r + c[0], a.sin() * r + c[1], c[2]]);
                set_seq(w, id, ctx, 2, 0, 0x14);
                walk_senses(&mut w.mobys[id], 1.5);
            }
            let s = w.mobys[id].pv_v(pv::SPOT);
            let y = yaw_to(w, id, hero);
            walk(w, id, ctx, s, y);
            if w.mobys[id].pv_f(MV + 0xcc) <= 1.5 && facing_hero(w, id, ctx) { w.mobys[id].set_state(25, -1); }
        }
        25 => {
            if w.mobys[id].first() {
                let s = if ctx.game.rng.randi(3) != 0 { 0x13 } else { 0x14 };
                set_seq(w, id, ctx, s, 0, 0x14);
            }
            if wrapped(w, id, ctx) { w.mobys[id].set_state(24, -1); }
        }
        _ => {}
    }
    tail(w, id, ctx);
    if w.mobys[id].state != 2 { hush(w, id, ctx); }
}

/// State 16, the gulp (module docs).
fn gulp(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let hero = ctx.hero.pos;
    if w.mobys[id].first() {
        set_seq(w, id, ctx, 7, 0, 0x14);
        walk_senses(&mut w.mobys[id], 1.0);
        w.mobys[id].set_pv_i(pv::GULPS, 0);
    }
    let (a, _) = seqs(w, id, ctx);
    if wrapped(w, id, ctx) {
        match a {
            7 => set_seq(w, id, ctx, 8, 0, 1),
            8 => set_seq(w, id, ctx, 9, 0, 1),
            9 => { let m = &mut w.mobys[id]; m.set_pv_i(pv::GULPS, m.pv_i(pv::GULPS) + 1); }
            _ => {}
        }
    }
    let f = ctx.host.frame_time(&w.mobys[id]);
    let (a, b) = seqs(w, id, ctx);
    if a == b && ((a == 8 && f < 9.0) || (a == 9 && 12.0 < f && f < 30.0)) {
        let y = yaw_to(w, id, hero);
        walk(w, id, ctx, hero, y);
        let (a, b) = seqs(w, id, ctx);
        if a == b && ((a == 8 && 7.0 < f && f < 9.0) || (b == 9 && 25.0 < f && f < 30.0)) {
            let m = &w.mobys[id];
            let (p, dmg) = (m.pos, m.pv_b(pv::GULP_DMG) as f32);
            ai::attack_sphere(w, id, ctx, 5.5, dmg, 2.0, [p[0], p[1], p[2] + 1.0], 0x1_0008, 0, 1, 0);
        }
    }
    let (a, _) = seqs(w, id, ctx);
    if a == 9 && ai::reached(w, id, ctx, 30.0) && (2 <= w.mobys[id].pv_i(pv::GULPS) || ctx.hero.state == 0x8c) {
        // `0x445720` (Ratchet swallowed and spat out) is not ported.
        w.mobys[id].set_state(3, -1);
    }
}

/// `0x445df0(boss, who)`: a target for `who` among the boss's groups — a fighter (class 0x0d0f since its set-up)
/// picks from the biters `+0x21c` and the shooters `+0x240`, a biter or a shooter from the fighters `+0x23c`: the
/// last one shown within 1025 across of the boss (the original never narrows the range).
pub fn pick_for(w: &MobyWorld, boss: usize, who: usize) -> Option<usize> {
    let b = w.mobys.get(boss)?;
    let lists: &[usize] = match w.mobys[who].o_class {
        0x0d0f => &[pv::BITERS, pv::SHOOTERS],
        biter::CLASS | shooter::CLASS => &[pv::FIGHTERS],
        _ => &[],
    };
    let mut best = None;
    for &l in lists {
        for k in members(w, b.pv_i(l)) {
            let e = &w.mobys[k];
            if e.mode & 1 == 0 && dist2(e.pos, b.pos) < 1025.0 { best = Some(k); }
        }
    }
    best
}
