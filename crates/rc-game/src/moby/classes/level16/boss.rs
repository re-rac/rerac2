//! Snivelak's boss **0x0f79** (L16 `0x4100e8`, 1 placed; the decomp's v1.01 `func_L16_0040ECC0`, its helpers
//! `func_L16_00410618..00413EC8` at our `0x411a40..0x4152f0`): a two-armed walker that fights Ratchet on foot and, when
//! he takes a mounted gun (crate::moby::classes::level16::turret), that gun. Its body **0x0f78** (placed beside it, the
//! empty update) is posed on its joint 2; its arms **0x07e8** (made at its set-up) on the body's joints 0 and 1. L16
//! addresses; the engine functions by their L16 places with the port's names.
//!
//! **Its pvars** ([`pv`]): health `+0x20` (most `+0x24`, 600 by default; the framework's block, crate::moby::ai), the
//! kill byte `+0x2e`, the damages `+0x34` (the blaster shots) / `+0x35` (the slam) / `+0x36` (the beam, the bombs);
//! the flash block `+0x50`; the arena's path `+0x170` (its first 8 points the walk graph, [`GRAPH`]), the body's index
//! `+0x174`, the cuboids `+0x178` (the way in), `+0x194` (the way back in), `+0x188` / `+0x18c` (Ratchet's place and
//! camera record after the fight), `+0x198` (the fight's checkpoint), `+0x19c` (the rocks' box), `+0x1b0` / `+0x1b4` /
//! `+0x1b8` (Ratchet's, its camera record's and its own place after the intro), the groups `+0x17c` / `+0x180` /
//! `+0x184` (shown for the fight), the bridge moby `+0x190`, the music `+0x1a0` (after) / `+0x1a4` / `+0x1ac`; the
//! voice tables `+0x280` / `+0x2d0` and the lines said `+0x310` (48 bytes).
//!
//! **Each tick** ([`head`], `0x411a40`): in state 0 the set-up (gone with the boss beaten, `0x1395b8 + 0x83` = 2; its
//! arms made; its floor its height; hidden unless the fight began). Out of subs 0 / 5 the frame's hit (its own, the
//! body's or an arm's, flags 0x10000) — × 0.5..1.5 by Ratchet's deaths in the fight (`$gp+0x26b4`, 0..20), × ⅓ in sub
//! 3, and from anything but the guns' shots 0x0ea7 × 0.2 (the Clank 0x05c9 × 0.01) or else a flash — comes off its
//! health. With Ratchet at a gun, below the next tenth of its most (`+0x248`) out of sub 3: state 9 (sub 3), the gun
//! its target. At no health (or the kill byte 2): 17 (sub 5). Then the voices ([`voices`]), its and its parts' hit
//! records dropped, the target (`0x3231c8`, range 128).
//!
//! **Its subs** (`+0x95`): **0** waiting: Ratchet in `+0x178` (or in `+0x194` with the fight begun): the first time
//! the intro scene 132 (the fight's start after it, [`start_fight`], `0x414170`), else at once; with it hidden and
//! Ratchet past x 610 it and its parts shown. **1** on foot: Ratchet at a gun → 7 (sub 2). **2** at a gun: off it →
//! 2 (sub 1). In the scene mode nothing more.
//!
//! **States** (`+0x20`): **2** the walk to Ratchet's nearest point on the graph (12 a second; within 32 at its steps'
//! frames 5 / 21, sound 8 and 3). **3** the stand: every 16th tick two blaster shots at him ([`blasters`]); out of
//! the graph's reach of him 2, within 12 (and 10 of its floor) 5, more than 15° off 4, after 900 ticks the beam (11)
//! at his place; his state 0x81 (on the wall) 6. **4** the turn. **5** the bombs dropped under it while it hovers
//! ([`drop_bomb`]). **6** waiting out his state 0x81. **7** the walk to a point of the gun's zone (24 a second), **8**
//! the arms' missiles at the gun ([`missile`]) for 300 ticks. **9** hurt: over 55% 10, else (at random or with a
//! throw unused) 13. **10** the walk to the gun: at 120 high the slam (12), else the beam (11). **11** the beam
//! ([`beam`]): turning to the gun, its sweep closing in over 3 seconds; the gun gone (120 ticks on) or the sweep done:
//! 3. **12** the slam at frames 40..42 (radius 4). **13** risen 20 the rock throw (14: a rock flown in, caught on the
//! body's joint 3 and thrown at the gun, [`rock`]) or the bombs lobbed (15: [`lob`]). **16** the fall to its floor.
//! **17** dying: explosions on it, the white flash; **18** falling, the end scenes 133 / 17 / 134 ([`end_fight`],
//! `0x414298`); **19** hidden.
//!
//! **The tail** ([`tail`], `0x4128a8`, `0x412a28`): hovering (`+0x26c`) its height eased toward 10 (25 in 13..15 and
//! 17) over its floor bobbing, its two jets ([`jets`]); in the fight the follow camera's ask (8 / 4) with Ratchet over
//! 118 under the follow camera, the boss bar (picture 11), a rock across the arena every 32nd tick, its shadow; its
//! flash, its parts' light, its ticks, the body's head aim (crate::moby::joint_aim, `+0x390`, list 2, × 4); its body
//! at its joint 2, the arms at the body's joints 0 / 1; its hover sound 10 looped while it hovers.
//!
//! **Its classes:** the missile **0x0f4e** ([`update_0f4e`], `0x40e910`, made by `0x40ef08`), the bomb **0x12be**
//! ([`update_12be`], `0x4202c8`, made by `0x420628`; its ring drawn by [`draw_ring`], `0x420798`), the rock **0x09b0**
//! ([`update_rock`], `0x4147d8`, its update set by its maker `0x4144d8`), the arm **0x07e8** ([`update_07e8`],
//! `0x3ab508`). The beam's quads ([`draw_beam`], `0x413988`).
//!
//! The port's: the scene-end callback (`0x3697b8` = L00 `0x371b50`, run at the scenes' end) kept in [`Globals`] and
//! run once the scenes are over; the beam's and the ring's line tests and the beam's sparks, which the original does
//! in their draws, done in the update that registers the draw (the draws get no collision here); the turret global
//! `$gp+0x2670` read as the gun Ratchet rides (crate::hero::Hero::vehicle, set and cleared where the gun sets it);
//! the sound handles start at −1.
//!
//! The fight's start and end ask for the autosave (`0x306e20(0, −1)` = L00 `0x30d658`, crate::game::GameState::autosave).
//!
//! Not ported: the camera records' resets (`0x4370f0`), the prints (`0x2a6170`), the
//! missile's class-sphere write (`0x30b158`), the glints' byte 9, the turret global's clear at the set-up.

use crate::hero::physics::{add, add_rot, approach, approach_angle, atan2, cross, dist, dot, ease, ease_speed, len, len2, rot_dist, scale, set_len, set_len2, sub, sub_rot, through, turn_ease, V3};
use crate::moby::classes::props::wrapped;
use crate::moby::{ai, blast, countdown, shards, Ctx, Moby, MobyWorld};
use crate::particles;

use super::turret;

pub const CLASS: i16 = 0x0f79;
pub const BODY: i16 = 0x0f78;
pub const ARM: i16 = 0x07e8;
pub const MISSILE: i16 = 0x0f4e;
pub const BOMB: i16 = 0x12be;
pub const ROCK: i16 = 0x09b0;
/// The guns' shots, which hurt it in full.
const GUN_SHOT: i16 = 0x0ea7;

/// pvars.
mod pv {
    pub const HP: usize = 0x20;
    pub const MAX: usize = 0x24;
    pub const KILL: usize = 0x2e;
    pub const SHOT_DMG: usize = 0x34;
    pub const POWER: usize = 0x35;
    pub const DMG: usize = 0x36;
    pub const FLASH: usize = 0x50;
    pub const PATH: usize = 0x170;
    pub const BODY_INDEX: usize = 0x174;
    pub const ENTRY: usize = 0x178;
    pub const GROUP_A: usize = 0x17c;
    pub const GROUP_B: usize = 0x180;
    pub const GROUP_C: usize = 0x184;
    pub const END_CUBOID: usize = 0x188;
    pub const BRIDGE: usize = 0x190;
    pub const EXIT: usize = 0x194;
    pub const CHECKPOINT: usize = 0x198;
    pub const ROCKS: usize = 0x19c;
    pub const MUSIC_END: usize = 0x1a0;
    pub const MUSIC: usize = 0x1a4;
    pub const STINGER: usize = 0x1ac;
    pub const BACK_CUBOID: usize = 0x1b0;
    pub const BOSS_CUBOID: usize = 0x1b8;
    pub const INTRO: usize = 0x1bc;
    pub const BODY: usize = 0x1f0;
    pub const NEAR_A: usize = 0x1f4;
    pub const NEAR_B: usize = 0x1f8;
    pub const NEAR_T: usize = 0x1fc;
    pub const DEST: usize = 0x200;
    pub const LAST: usize = 0x210;
    pub const EDGE_A: usize = 0x220;
    pub const EDGE_B: usize = 0x224;
    pub const NEXT: usize = 0x228;
    pub const FROM: usize = 0x22c;
    pub const SPEED: usize = 0x230;
    pub const TURN_V: usize = 0x234;
    pub const ARMS: usize = 0x238;
    pub const ARM: usize = 0x240;
    pub const FIRED: usize = 0x244;
    pub const NEXT_HP: usize = 0x248;
    pub const TARGET: usize = 0x24c;
    pub const AIM_T: usize = 0x250;
    pub const AIM_BLEND: usize = 0x254;
    pub const BEAM_FRAME: usize = 0x258;
    pub const AIM_WAIT: usize = 0x25c;
    pub const PHASES: usize = 0x260;
    pub const FLOOR: usize = 0x264;
    pub const VZ: usize = 0x268;
    pub const HOVER: usize = 0x26c;
    pub const RESUME: usize = 0x270;
    pub const RESUME_SUB: usize = 0x274;
    pub const LINES_A: usize = 0x280;
    pub const LINES_B: usize = 0x2d0;
    pub const SAID: usize = 0x310;
    pub const TAUNTED: usize = 0x320;
    pub const SPEECH_WAIT: usize = 0x350;
    pub const LAST_LINE: usize = 0x354;
    pub const FIRE_T: usize = 0x35c;
    pub const ROCK: usize = 0x360;
    pub const LEFT: usize = 0x364;
    pub const RIGHT: usize = 0x366;
    pub const AIM_SOUND: usize = 0x368;
    pub const LOOP_SOUND: usize = 0x36c;
    pub const HEAD: usize = 0x390;
}

/// `$gp+0x26c0` (v1.01 `0x1b1710`): the walk graph over the path's first 8 points, each point's neighbours (−1 none).
const GRAPH: [[i32; 4]; 8] = [[1, 4, -1, -1], [0, 2, -1, -1], [1, 3, 7, -1], [2, 4, 6, -1], [0, 3, 5, -1], [4, 6, -1, -1], [3, 5, 7, -1], [2, 6, -1, -1]];
/// `$gp+0x2674`: the health's phases (tenths).
const PHASES: i32 = 10;
/// `$gp+0x2694` / `+0x2698`: the walk's speeds (a second), on foot and to the gun; `+0x269c` / `+0x26a0` the zone
/// walk's (read by nothing).
const WALK: f32 = 12.0;
const RUN: f32 = 24.0;
/// `$gp+0x26a4` / `+0x26a8`: the beam's sweep (its start's distance) and its closing speed (a second).
const SWEEP: f32 = 6.0;
const SWEEP_RATE: f32 = 2.0;
/// `$gp+0x26ac` / `+0x26b0`: the follow camera's distance and pivot height asked for.
const CAM: (f32, f32) = (8.0, 4.0);
/// `$gp+0x26b8`: Ratchet this far over its floor and it hovers.
const HIGH: f32 = 24.0;
/// `$gp+0x2700`: the walk's ease (a second²).
const WALK_EASE: f32 = 120.0;
/// `$gp+0x2680`: the muzzle in the body's joint 2.
const MUZZLE: V3 = [3.0, 0.0, 2.0];
/// `$gp+0x2690`: the blaster shots' speed (a second).
const SHOT_SPEED: f32 = 30.0;
/// `$gp+0x2730..+0x2740`: the missiles' spread (degrees round and up), their bend's reach, the arms' line-test point.
const SPREAD: (f32, f32, f32) = (60.0, 40.0, 12.0);
const ARM_POINT: V3 = [0.0, 15.0, 5.0];
/// `$gp+0x2750`: the lob's start in its frame; `+0x2760` its speed (a second); `+0x2768` / `0x1b175c` its gravity.
const LOB_AT: V3 = [21.0, 8.0, 24.0];
const LOB_SPEED: f32 = 30.0;
const LOB_GRAVITY: (f32, f32) = (5.0, 10.0);
/// The beam's quads (`$gp+0x2798` FX, `+0x27a8` half-widths, `+0x27d0` colours).
const BEAM_FX: usize = 0xb;
const BEAM_W: [f32; 2] = [1.0, 0.2];
const BEAM_RGBA: [u32; 2] = [0x4000_0040, 0x8000_0080];
/// The beam's sparks: `$gp+0x27d8` a tick, `+0x27dc` the colour word, `+0x27e0` / `0x1b17d4` the end's burst speed,
/// `0x1b17d8` / `0x1b17dc` the speeds along it, `+0x27f0` / `0x1b17e4` the lives, `+0x27f8` / `0x1b17ec` the sizes.
const SPARKS: i32 = 4;
const SPARK_RGBA: u32 = 0x7f00_4080;
const SPARK_BURST: (f32, f32) = (4.0, 8.0);
const SPARK_SPEED: (f32, f32) = (8.0, 16.0);
const SPARK_LIFE: (i32, i32) = (10, 30);
const SPARK_SIZE: (f32, f32) = (0.2, 0.5);
/// The jets (`$gp+0x2850` / `0x1b1844` the fall a second; the colours and bases `0x1b1848..0x1b1868`).
const JET_FALL: (f32, f32) = (15.0, 20.0);
/// The rocks (`$gp+0x287c` / `+0x2880` / `+0x2884`): the point beside it, its height over its floor, the flight's ticks;
/// `$gp+0x28a0` the held angles.
const ROCK_REACH: f32 = 25.0;
const ROCK_HIGH: f32 = 56.0;
const ROCK_TICKS: f32 = 200.0;
const ROCK_HELD: V3 = [0.0, std::f32::consts::PI, 0.0];

const DEG: f32 = 0.017_453_294;
const TICK: f32 = 0.016_666_668;
const TICK2: f32 = 0.000_277_777_78;
const SLOW: (f32, f32) = (0.001_745_329_5, 0.104_719_77);
const SLOWER: (f32, f32) = (0.000_872_664_74, 0.104_719_77);
const FAST: (f32, f32) = (0.003_490_659, 0.209_439_55);

/// Level 16's words the boss keeps (crate::moby::MobyWorld::snivelak).
#[derive(Clone, Debug, Default)]
pub struct Globals {
    /// `$gp+0x26b4`: Ratchet's deaths in the fight (also read by the guns' help text).
    pub deaths: i32,
    /// The scene-end callback asked for (`0x1a8c80` / `0x1a8c84`): 1 the fight's start, 2 its end; its moby.
    pub scene_end: Option<(u8, usize)>,
    /// The white flash (`0x1b9940 + 0x274..0x280`, ticked by `0x2d3128`): its step up, its step down, its target, the
    /// ticks held; the screen's white itself is crate::game::GameState::fade_white (`$gp+0x34f4`).
    pub white: (f32, f32, f32, i32),
    /// The beam's ends this tick (the port's: [`draw_beam`] reads them).
    pub beam: Option<(V3, V3)>,
}

// ---- Small helpers ----

fn yaw_to(m: &Moby, p: V3) -> f32 { atan2(p[0] - m.pos[0], p[1] - m.pos[1]) }
fn dist_sq(a: V3, b: V3) -> f32 { let d = sub(a, b); dot(d, d) }
fn live(w: &MobyWorld, k: Option<usize>) -> Option<usize> { k.filter(|&k| w.mobys.get(k).is_some_and(|m| m.state < 0x80)) }
fn body(w: &MobyWorld, id: usize) -> Option<usize> { live(w, ai::link(&w.mobys[id], pv::BODY)) }
fn arms(w: &MobyWorld, id: usize) -> [Option<usize>; 2] { [0, 1].map(|k| live(w, ai::link(&w.mobys[id], pv::ARMS + 4 * k))) }
fn parts(w: &MobyWorld, id: usize) -> [Option<usize>; 3] { let a = arms(w, id); [body(w, id), a[0], a[1]] }
fn target(w: &MobyWorld, id: usize) -> Option<usize> { live(w, ai::link(&w.mobys[id], pv::TARGET)) }
fn rock_of(w: &MobyWorld, id: usize) -> Option<usize> { live(w, ai::link(&w.mobys[id], pv::ROCK)).filter(|&r| w.mobys[r].o_class == ROCK) }
fn flag(ctx: &Ctx) -> u8 { ctx.game.flags.get(0x83).copied().unwrap_or(0) }
fn set_flag(ctx: &mut Ctx, v: u8) { if let Some(b) = ctx.game.flags.get_mut(0x83) { *b = v; } }
fn in_scene_mode(ctx: &Ctx) -> bool { ctx.game.scenes.mode == crate::scene::SCENE }
fn stagger(ctx: &Ctx, id: usize, mask: i32) -> bool { ctx.game.frame & mask == id as i32 & mask }

/// `$gp+0x2670`: the gun Ratchet sits at.
fn gun(w: &MobyWorld, ctx: &Ctx) -> Option<usize> { live(w, ctx.hero.vehicle.moby).filter(|&g| w.mobys[g].o_class == turret::GUN) }

fn seqs(w: &MobyWorld, k: usize, ctx: &Ctx) -> (u8, u8, u8) { ctx.host.anim(&w.mobys[k]).map_or((0xff, 0xff, 0), |a| (a.seq_a, a.seq_b, a.frame_b)) }
fn speed(w: &MobyWorld, k: usize, ctx: &Ctx) -> f32 { ctx.host.anim(&w.mobys[k]).map_or(1.0, |a| a.speed) }
fn set_speed(w: &MobyWorld, k: Option<usize>, ctx: &mut Ctx, s: f32) { if let Some(k) = k { ctx.host.set_anim_speed(&w.mobys[k], s); } }
fn set_anim(w: &MobyWorld, k: usize, ctx: &mut Ctx, s: i32, frame: i32, blend: i32) { ctx.host.set_anim(&w.mobys[k], s as u8, frame.max(0) as u8, blend as u8); }

/// `0x414bb0(a, b, f0, f1)` (the decomp's shared `func_L07_003F90E8`): `f0` at `a` deaths or fewer, `f1` at `b` or more.
fn by_deaths(w: &MobyWorld, a: i32, b: i32, f0: f32, f1: f32) -> f32 {
    let n = w.snivelak.deaths;
    if a >= n { return f0; }
    if n >= b { return f1; }
    f0 + (f1 - f0) * ((n - a) as f32 / (b - a) as f32)
}

/// `0x3581d0` when the handle is still playing.
fn stop(ctx: &mut Ctx, h: i32) { if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); } }

/// The beam's looped sound (on the body) let go.
fn stop_aim_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    stop(ctx, w.mobys[id].pv_i(pv::AIM_SOUND));
    w.mobys[id].set_pv_i(pv::AIM_SOUND, -1);
}

/// `0x31cdd0` = L00 `0x31e048(a, b, _, speed, g)`: the vertical speed that lands a lob from `a` on `b` across at `speed`
/// a tick under the gravity `g`.
fn lob_z(a: V3, b: V3, speed: f32, g: f32) -> f32 {
    let t = len2(sub(b, a)) / speed;
    if t == 0.0 { return 0.0; }
    -((a[2] - b[2]) + g * (t * t) * 0.5) / t
}

/// `0x2e86f0`: a point through a volume's matrix (its rows and place).
fn in_volume(ctx: &Ctx, i: i32, p: V3) -> Option<V3> {
    let c = usize::try_from(i).ok().and_then(|i| ctx.level.cuboids.get(i))?;
    let m = &c.matrix;
    Some(std::array::from_fn(|k| p[0] * m[0][k] + p[1] * m[1][k] + p[2] * m[2][k] + m[3][k]))
}

/// A point of the arena's path.
fn node(ctx: &Ctx, path: i32, k: i32) -> Option<V3> {
    let p = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p))?.get(usize::try_from(k).ok()?)?;
    Some([p[0], p[1], p[2]])
}

// ---- The boss ----

/// 0x0f79 (`0x4100e8`, module docs).
pub fn update_0f79(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.snivelak.beam = None;
    white_tick(w, ctx);
    if let Some((k, m)) = w.snivelak.scene_end {
        if m == id && !ctx.game.scenes.queue.busy && !ctx.game.scenes.in_scene() {
            w.snivelak.scene_end = None;
            if k == 1 { start_fight(w, id, ctx) } else { return end_fight(w, id, ctx) }
        }
    }
    head(w, id, ctx);
    if 0xfd <= w.mobys[id].state { return; }
    match w.mobys[id].sub_state {
        0 => wait(w, id, ctx),
        1 => if let Some(g) = gun(w, ctx) {
            ai::set_link(&mut w.mobys[id], pv::TARGET, Some(g));
            w.mobys[id].set_state(7, 2);
        },
        2 => if gun(w, ctx).is_none() { w.mobys[id].set_state(2, 1) },
        _ => {}
    }
    if in_scene_mode(ctx) { return; }
    let hero = ctx.hero.pos;
    let ticks = w.mobys[id].state_ticks as u16;
    match w.mobys[id].state {
        2 => {
            if w.mobys[id].first() {
                anim(w, id, ctx, 10, 15, 3.0);
                if ctx.game.rng.randi(1) == 0 {
                    if 1600.0 < dist_sq(w.mobys[id].pos, hero) { say_one_of(w, id, ctx, 3, 6) } else { say_one_of(w, id, ctx, 7, 11) }
                }
            }
            nearest(w, id, ctx, hero);
            walk(w, id, ctx, WALK * TICK);
            let m = &w.mobys[id];
            if dist(m.pos, m.pv_v(pv::DEST)) < 32.0 {
                let f = ctx.host.frame_time(m);
                if (f - 5.0).abs() < 1.0 || (f - 21.0).abs() < 1.0 {
                    ctx.host.play_sound(&w.mobys[id], 8, 0);
                    w.mobys[id].set_state(3, -1);
                }
            }
            hover_check(w, id, ctx);
        }
        16 => {
            let m = &mut w.mobys[id];
            if m.first() { m.set_pv_f(pv::VZ, 0.0); }
            let vz = m.pv_f(pv::VZ) - 0.008_333_334;
            m.set_pv_f(pv::VZ, vz);
            m.pos[2] += vz;
            let floor = m.pv_f(pv::FLOOR);
            if m.pos[2] <= floor {
                m.pos[2] = floor;
                ctx.host.play_sound(&w.mobys[id], 9, 0);
                let m = &mut w.mobys[id];
                let (s, sub) = (m.pv_i(pv::RESUME), m.pv_i(pv::RESUME_SUB));
                m.set_state(s as u8, sub);
            }
        }
        3 => stand(w, id, ctx),
        4 => {
            let to = yaw_to(&w.mobys[id], hero);
            if w.mobys[id].first() {
                let f = if 0.0 < sub_rot(w.mobys[id].rot[2], to) { 1.0 } else { 10.0 };
                anim(w, id, ctx, 10, 30, f);
            }
            turn(w, id, to, SLOW);
            let to = yaw_to(&w.mobys[id], hero);
            let (a, b, _) = seqs(w, id, ctx);
            if rot_dist(w.mobys[id].rot[2], to) < 0.087_266_475 && a == b { w.mobys[id].set_state(3, -1); }
        }
        5 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.set_pv_f(pv::VZ, 0.0);
                m.set_pv_i(pv::HOVER, 1);
                m.set_pv_i(pv::FIRE_T, 30);
                anim(w, id, ctx, 0, 30, 0.0);
            }
            let to = yaw_to(&w.mobys[id], hero);
            turn(w, id, to, SLOWER);
            if w.mobys[id].count_i(pv::FIRE_T) {
                drop_bomb(w, id, ctx);
                ctx.host.play_level_sound_after(Some(&w.mobys[id]), 17, 0);
                let t = ctx.game.rng.rand_range(45, 75);
                w.mobys[id].set_pv_i(pv::FIRE_T, t);
            }
            let m = &w.mobys[id];
            if 16.0 < dist(m.pos, hero) || 10.0 < (hero[2] - m.pv_f(pv::FLOOR)).abs() { w.mobys[id].set_state(2, -1); }
        }
        6 => {
            if w.mobys[id].first() {
                anim(w, id, ctx, 16, 30, 0.0);
                set_speed(w, Some(id), ctx, 0.666);
            }
            if body(w, id).is_some_and(|b| wrapped(w, b, ctx)) { w.mobys[id].set_state(2, -1); }
        }
        8 => {
            if w.mobys[id].first() {
                let a = w.mobys[id].pv_i(pv::ARM);
                anim(w, id, ctx, a + 11, 20, 0.0);
                if w.mobys[id].pv_i(pv::SPEECH_WAIT) == 0 { say_one_of(w, id, ctx, 17, 23); }
                w.mobys[id].set_pv_i(pv::FIRED, 0);
                set_speed(w, Some(id), ctx, 1.0);
            }
            let to = yaw_to(&w.mobys[id], hero);
            turn(w, id, to, FAST);
            if let Some(b) = body(w, id) {
                let (sa, sb, _) = seqs(w, b, ctx);
                let f = ctx.host.frame_time(&w.mobys[b]);
                if w.mobys[id].pv_i(pv::FIRED) == 0 && sa == sb && 13.0 <= f {
                    if sa == 11 { ctx.host.play_sound(&w.mobys[id], 7, 0); }
                    missile(w, id, ctx);
                    w.mobys[id].set_pv_i(pv::FIRED, 1);
                } else if 20.0 < f {
                    let a = w.mobys[id].pv_i(pv::ARM);
                    anim(w, id, ctx, a + 11, 20, 0.0);
                    w.mobys[id].set_pv_i(pv::FIRED, 0);
                }
            }
            if 300 < ticks { w.mobys[id].set_state(7, -1); }
        }
        7 => {
            if w.mobys[id].first() {
                anim(w, id, ctx, 10, 30, 6.0);
                pick(w, id, ctx);
            }
            walk(w, id, ctx, RUN * TICK);
            let m = &w.mobys[id];
            if dist(m.pos, m.pv_v(pv::DEST)) < 1.0 { w.mobys[id].set_state(8, -1); }
            hover_check(w, id, ctx);
        }
        9 => {
            if w.mobys[id].first() { anim(w, id, ctx, 16, 30, 6.0); }
            let to = yaw_to(&w.mobys[id], hero);
            turn(w, id, to, FAST);
            if body(w, id).is_some_and(|b| wrapped(w, b, ctx)) {
                let m = &w.mobys[id];
                let next = if m.pv_s(pv::MAX) as f32 * 0.55 <= m.pv_f(pv::HP) {
                    10
                } else if ctx.game.rng.randi(2) != 0 || m.pv_s(pv::LEFT) == 0 || m.pv_s(pv::RIGHT) == 0 {
                    13
                } else {
                    10
                };
                w.mobys[id].set_state(next, -1);
            }
        }
        10 => {
            if w.mobys[id].first() {
                anim(w, id, ctx, 10, 30, 6.0);
                if let Some(t) = target(w, id) { let p = w.mobys[t].pos; nearest(w, id, ctx, p); }
            }
            walk(w, id, ctx, RUN * TICK);
            let m = &w.mobys[id];
            if dist(m.pos, m.pv_v(pv::DEST)) < 1.0 {
                let high = target(w, id).is_some_and(|t| (w.mobys[t].pos[2] - 120.0).abs() < 1.0);
                w.mobys[id].set_state(if high { 12 } else { 11 }, -1);
            }
        }
        11 => aim(w, id, ctx),
        12 => {
            if w.mobys[id].first() {
                anim(w, id, ctx, 1, 30, 0.0);
                set_speed(w, Some(id), ctx, 0.5);
            }
            let t = target(w, id);
            if let Some(t) = t { let to = yaw_to(&w.mobys[id], w.mobys[t].pos); turn(w, id, to, FAST); }
            let f = ctx.host.frame_time(&w.mobys[id]);
            if 30.0 < f {
                let mut s = speed(w, id, ctx);
                approach(1.0, 0.133_333_34, &mut s);
                set_speed(w, Some(id), ctx, s);
            }
            if 40.0 < f && f < 42.0 {
                if let Some(t) = t {
                    let (at, power) = (w.mobys[t].pos, w.mobys[id].pv_b(pv::POWER) as f32);
                    ai::attack_sphere(w, id, ctx, 4.0, power, 1.0, at, 0x10001, 0, 1, 0);
                }
            }
            if wrapped(w, id, ctx) {
                w.mobys[id].set_state(2, 1);
            } else if ctx.hero.state == 0x16 && ctx.hero.timer == 1 && ctx.hero.damage.hit_by == Some(id) {
                say_one_of(w, id, ctx, 24, 27);
            }
        }
        13 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.set_pv_f(pv::VZ, 0.0);
                m.set_pv_i(pv::HOVER, 1);
                anim(w, id, ctx, 0, 30, 0.0);
                let (l, r) = (w.mobys[id].pv_s(pv::LEFT), w.mobys[id].pv_s(pv::RIGHT));
                let rock = if l == r {
                    if ctx.game.rng.randi(2) == 0 { None } else { make_rock(w, id, ctx, true) }
                } else if l == 0 {
                    make_rock(w, id, ctx, true)
                } else {
                    None
                };
                ai::set_link(&mut w.mobys[id], pv::ROCK, rock);
            }
            let to = yaw_to(&w.mobys[id], hero);
            turn(w, id, to, FAST);
            let m = &mut w.mobys[id];
            if m.pv_f(pv::FLOOR) + 20.0 < m.pos[2] {
                if m.pv_i(pv::ROCK) != 0 {
                    m.set_state(14, -1);
                    m.set_pv_s(pv::LEFT, 1);
                } else {
                    m.set_state(15, -1);
                    m.set_pv_s(pv::RIGHT, 1);
                }
            }
        }
        15 => {
            if w.mobys[id].first() { anim(w, id, ctx, 14, 30, 0.0); }
            let to = yaw_to(&w.mobys[id], hero);
            turn(w, id, to, FAST);
            if body(w, id).is_some_and(|b| { let (a, s, _) = seqs(w, b, ctx); a == s }) && stagger(ctx, id, 15) { lob(w, id, ctx); }
            if 180 < ticks { w.mobys[id].set_state(2, 1); }
        }
        14 => throw(w, id, ctx),
        17 => dying(w, id, ctx),
        18 => {
            set_speed(w, Some(id), ctx, 0.1);
            set_speed(w, body(w, id), ctx, 0.1);
            let m = &mut w.mobys[id];
            m.set_pv_i(pv::HOVER, 0);
            let vz = m.pv_f(pv::VZ) - 0.005_555_556;
            m.set_pv_f(pv::VZ, vz);
            m.pos[2] += vz;
            if 90 < ticks && flag(ctx) != 2 && ctx.game.scenes.queue.push(133) {
                w.mobys[id].set_state(19, -1);
                let q = &mut ctx.game.scenes.queue;
                q.push(17);
                q.push(134);
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.snivelak.scene_end = Some((2, id));
            }
        }
        19 => {
            if w.mobys[id].first() {
                for k in std::iter::once(Some(id)).chain(parts(w, id)).flatten() {
                    let m = &mut w.mobys[k];
                    m.has_collision = false;
                    m.mode = (m.mode | 0x41) & 0xefff;
                }
                w.changed = true;
            }
            if in_scene_mode(ctx) { show(w, id, ctx, false); }
        }
        _ => {}
    }
    tail(w, id, ctx);
    place_parts(w, id, ctx);
    let m = &w.mobys[id];
    let h = m.pv_i(pv::LOOP_SOUND);
    if m.pv_i(pv::HOVER) != 0 {
        if !ctx.host.sound_playing(h) {
            let h = ctx.host.play_sound(&w.mobys[id], 10, 4);
            w.mobys[id].set_pv_i(pv::LOOP_SOUND, h);
        }
    } else {
        stop(ctx, h);
        w.mobys[id].set_pv_i(pv::LOOP_SOUND, -1);
    }
}

/// Sub 0 (module docs).
fn wait(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let hero = ctx.hero.pos;
    if ctx.level.in_cuboid(m.pv_i(pv::ENTRY), hero) || (ctx.level.in_cuboid(m.pv_i(pv::EXIT), hero) && flag(ctx) == 1) {
        if flag(ctx) == 0 {
            if ctx.game.scenes.queue.push(132) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                w.snivelak.scene_end = Some((1, id));
                set_flag(ctx, 1);
                w.mobys[id].set_pv_i(pv::INTRO, 1);
            }
        } else if !ctx.game.scenes.in_scene() {
            start_fight(w, id, ctx);
        }
    } else if m.mode & 1 != 0 && 610.0 <= hero[0] {
        for k in std::iter::once(Some(id)).chain(parts(w, id)).flatten() {
            let has = w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision);
            let m = &mut w.mobys[k];
            m.mode = (m.mode & 0xffbe) | 0x1000;
            m.has_collision = has;
        }
        w.changed = true;
    }
}

/// State 3 (module docs).
fn stand(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let hero = ctx.hero.pos;
    if w.mobys[id].first() {
        ctx.host.play_sound(&w.mobys[id], 9, 0);
        anim(w, id, ctx, 3, 15, 6.0);
        if ctx.game.rng.randi(1) == 0 { say_one_of(w, id, ctx, 12, 15); }
    }
    let m = &w.mobys[id];
    if m.pv_b(pv::TAUNTED) == 0 && m.pv_i(pv::SPEECH_WAIT) == 0 && ctx.game.rng.randi(2) == 0 && ctx.hero.state == 0x16 && ctx.hero.timer == 1
        && ctx.hero.damage.hit_by.and_then(|h| w.mobys.get(h)).is_some_and(|h| h.o_class == GUN_SHOT)
    {
        w.mobys[id].set_pv_b(pv::TAUNTED, 1);
        say(w, id, ctx, 16);
    }
    if stagger(ctx, id, 15) { blasters(w, id, ctx); }
    nearest(w, id, ctx, hero);
    let m = &w.mobys[id];
    if ctx.hero.state == 0x81 {
        w.mobys[id].set_state(6, -1);
    } else if 48.0 < dist(m.pos, m.pv_v(pv::DEST)) {
        w.mobys[id].set_state(2, -1);
    } else if dist(m.pos, hero) < 12.0 && (hero[2] - m.pv_f(pv::FLOOR)).abs() < 10.0 {
        w.mobys[id].set_state(5, -1);
    } else {
        let to = yaw_to(m, hero);
        if 0.261_799_42 < rot_dist(m.rot[2], to) {
            w.mobys[id].set_state(4, -1);
        } else if 900 < m.state_ticks as u16 {
            let aim = ctx.hero.aim_point;
            let m = &mut w.mobys[id];
            m.set_state(11, -1);
            ai::set_link(m, pv::TARGET, None);
            m.set_pv_v(pv::LAST, aim);
        }
    }
    hover_check(w, id, ctx);
}

/// `0x413988`'s turn and sweep (state 11, module docs).
fn aim(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        if seqs(w, id, ctx).1 != 22 { set_anim(w, id, ctx, 22, 0, 20); }
        if let Some(b) = body(w, id) { set_anim(w, b, ctx, 3, 0, 30); }
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::AIM_WAIT, 120);
        m.set_pv_f(pv::AIM_T, SWEEP);
        m.set_pv_f(pv::AIM_BLEND, 0.0);
        m.set_pv_f(pv::VZ, 0.0);
    }
    let t = target(w, id);
    let at = t.map_or(w.mobys[id].pv_v(pv::LAST), |t| w.mobys[t].pos);
    let to = yaw_to(&w.mobys[id], at);
    turn(w, id, to, FAST);
    if rot_dist(to, w.mobys[id].rot[2]) < 0.087_266_475 {
        let m = &mut w.mobys[id];
        let (mut a, mut b) = (m.pv_f(pv::AIM_T), m.pv_f(pv::AIM_BLEND));
        approach(0.0, SWEEP_RATE * TICK, &mut a);
        approach(1.0, 0.066_666_67, &mut b);
        m.set_pv_f(pv::AIM_T, a);
        m.set_pv_f(pv::AIM_BLEND, b);
        if let Some(bd) = body(w, id) {
            if !ctx.host.sound_playing(w.mobys[id].pv_i(pv::AIM_SOUND)) {
                let h = ctx.host.play_sound(&w.mobys[bd], 2, 4);
                w.mobys[id].set_pv_i(pv::AIM_SOUND, h);
            }
        }
        beam(w, id, ctx);
        w.register_draw_first(id);
    }
    let done = match t {
        Some(t) => w.mobys[t].mode & 1 != 0 && w.mobys[id].count_i(pv::AIM_WAIT),
        None => w.mobys[id].pv_f(pv::AIM_T) <= 0.0,
    };
    if done {
        w.mobys[id].set_state(3, 1);
        stop_aim_sound(w, id, ctx);
    }
}

/// State 14 (module docs): the rock caught on the body's joint 3 and thrown at the gun.
fn throw(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() { anim(w, id, ctx, 6, 30, 0.0); }
    let Some(b) = body(w, id) else { return };
    let bone = ctx.host.joint_matrix(&w.mobys[b], 3);
    let rock = rock_of(w, id);
    match seqs(w, b, ctx).1 {
        6 => if let (Some(r), Some(bone)) = (rock, bone) {
            if rock_arrived(&w.mobys[r]) {
                rock_hold(w, r, ctx, bone);
                anim(w, id, ctx, 7, 5, 0.0);
            }
        },
        7 => {
            if let (Some(r), Some(bone)) = (rock, bone) { rock_hold(w, r, ctx, bone); }
            if wrapped(w, b, ctx) { anim(w, id, ctx, 9, 5, 0.0); }
        }
        9 => {
            if let Some(r) = rock {
                if let Some(bone) = bone { rock_hold(w, r, ctx, bone); }
                if seqs(w, b, ctx).0 == 9 && 16.0 < ctx.host.frame_time(&w.mobys[b]) {
                    if let Some(t) = target(w, id) {
                        let g = 0.001_388_888_9;
                        let (from, to) = (w.mobys[r].pos, w.mobys[t].pos);
                        let d = dist(from, to) * 0.008_333_334;
                        let mut v = sub(to, from);
                        v[2] = 0.0;
                        let mut v = set_len(v, d);
                        v[2] = lob_z(from, to, d, -g);
                        rock_throw(w, r, v, g);
                    }
                    ai::set_link(&mut w.mobys[id], pv::ROCK, None);
                }
            }
            if wrapped(w, b, ctx) { w.mobys[id].set_state(2, 1); }
        }
        _ => {}
    }
}

/// State 17 (module docs).
fn dying(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        anim(w, id, ctx, 16, 30, 0.0);
        set_speed(w, Some(id), ctx, 0.5);
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::HOVER, 1);
        if m.pv_f(pv::VZ) < 0.0 { m.set_pv_f(pv::VZ, 0.0); }
        stop_aim_sound(w, id, ctx);
        stop(ctx, w.mobys[id].pv_i(pv::LOOP_SOUND));
        w.mobys[id].set_pv_i(pv::LOOP_SOUND, -1);
    }
    let to = yaw_to(&w.mobys[id], ctx.hero.pos);
    turn(w, id, to, SLOW);
    if stagger(ctx, id, 15) {
        let g = &mut ctx.game.rng;
        let off = [g.randf_signed(0.0, 6.0), g.randf_signed(0.0, 6.0), g.randf(5.0, 20.0)];
        let at = add(off, w.mobys[id].pos);
        let dir = set_len(sub(ctx.host.camera_pos(), at), 1.0);
        super::super::explosion::start(w, ctx, 0.0, at, dir);
        ctx.host.play_sound_of(&w.mobys[id], BODY, 6, 0);
    }
    if let Some(b) = body(w, id) {
        let (a, s, f) = seqs(w, b, ctx);
        if a == s && 29 < f { white_start(w, 5, 5.0, 15.0); }
    }
    if 1.0 <= ctx.game.fade_white {
        let m = &mut w.mobys[id];
        m.set_state(18, -1);
        m.drawn = 0;
        m.mode |= 1;
        m.set_pv_i(pv::HOVER, 0);
    }
}

/// `0x412af0(m, seq, blend, frame)`: its sequence (hovering 0x16, its walk 10 then 0x16 for the body too) and the
/// body's (sequences past 0x10 its 0).
fn anim(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: i32, blend: i32, f: f32) {
    let frame = f as i32;
    let mut s = s;
    let b = seqs(w, id, ctx).1 as i32;
    if w.mobys[id].pv_i(pv::HOVER) == 0 {
        if b != s { set_anim(w, id, ctx, s, frame, blend); }
    } else {
        if b != 0x16 { set_anim(w, id, ctx, 0x16, frame, blend); }
        if s == 10 { s = 0x16; }
    }
    if let Some(bd) = body(w, id) { set_anim(w, bd, ctx, if s < 0x11 { s } else { 0 }, frame, blend); }
}

/// `0x318a78` toward `to` (the turn speed `+0x234`).
fn turn(w: &mut MobyWorld, id: usize, to: f32, (k, max): (f32, f32)) {
    let m = &mut w.mobys[id];
    let (mut a, mut v) = (m.rot[2], m.pv_f(pv::TURN_V));
    turn_ease(to, k, k, max, &mut a, &mut v, 0);
    m.rot[2] = a;
    m.set_pv_f(pv::TURN_V, v);
}

/// `0x411a40` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 && !set_up(w, id, ctx) { return; }
    let sub = w.mobys[id].sub_state;
    if sub == 0 || sub == 5 { return; }
    let [b, a0, a1] = parts(w, id);
    let hit = [Some(id), b, a0, a1].into_iter().flatten().find_map(|k| w.hit_message(k).filter(|h| h.tmpl.flags & 0x10000 != 0));
    if let Some(mut h) = hit {
        h.tmpl.damage *= by_deaths(w, 0, 20, 0.5, 1.5);
        if sub == 3 { h.tmpl.damage *= 0.333; }
        match h.tmpl.source.and_then(|s| w.mobys.get(s)).map(|s| s.o_class) {
            Some(c) if c != GUN_SHOT => {
                h.tmpl.damage *= 0.2;
                if c == 0x05c9 { h.tmpl.damage *= 0.05; }
            }
            _ => ai::flash_start(&mut w.mobys[id], pv::FLASH),
        }
        let amount = ai::damage_of(w, id, Some(&h), false);
        let m = &mut w.mobys[id];
        let mut hp = m.pv_f(pv::HP);
        approach(0.0, amount, &mut hp);
        m.set_pv_f(pv::HP, hp);
        let g = gun(w, ctx);
        let m = &w.mobys[id];
        if g.is_some() && m.sub_state != 3 && hp < m.pv_f(pv::NEXT_HP) {
            let m = &mut w.mobys[id];
            let q = m.pv_s(pv::MAX) as f32 / PHASES as f32;
            let k = ((hp - 1.0) / q) as i32;
            m.set_pv_i(pv::PHASES, m.pv_i(pv::PHASES) + 1);
            m.set_pv_f(pv::NEXT_HP, k as f32 * q);
            m.set_state(9, 3);
            stop_aim_sound(w, id, ctx);
            ai::set_link(&mut w.mobys[id], pv::TARGET, g);
        } else if hp == 0.0 || m.pv_b(pv::KILL) == 2 {
            stop_aim_sound(w, id, ctx);
            w.mobys[id].set_state(17, 5);
        }
    }
    voices(w, id, ctx);
    for k in [Some(id), b, a0, a1].into_iter().flatten() { w.clear_hit(k); }
    ai::find_target(w, id, ctx, 128.0, 0.0, -1, None, false);
}

/// `0x411a40`'s set-up (module docs); false when it went.
fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let bi = usize::try_from(w.mobys[id].pv_i(pv::BODY_INDEX)).ok().filter(|&b| b < w.mobys.len());
    if flag(ctx) == 2 {
        ai::set_link(&mut w.mobys[id], pv::BODY, bi);
        show(w, id, ctx, false);
        if let Some(b) = bi { w.delete(b); }
        w.delete(id);
        return false;
    }
    let m = &mut w.mobys[id];
    if m.pv_s(pv::MAX) == 0 {
        m.set_pv_s(pv::MAX, 600);
        m.set_pv_f(pv::HP, 600.0);
    }
    m.set_pv_b(pv::KILL, 1);
    let q = m.pv_s(pv::MAX) as f32 / PHASES as f32;
    let k = ((m.pv_f(pv::HP) - 1.0) / q) as i32;
    m.set_pv_f(pv::NEXT_HP, k as f32 * q);
    for arm in 0..2 {
        if w.class(ARM).is_none() { continue; }
        let c = w.create(ARM);
        w.adopt(c, Some(id), &*ctx.host);
        w.mobys[c].mode |= 0x4100;
        ai::set_link(&mut w.mobys[id], pv::ARMS + 4 * arm, Some(c));
    }
    let m = &mut w.mobys[id];
    m.set_pv_f(pv::FLOOR, m.pos[2]);
    ai::set_link(m, pv::BODY, bi);
    m.mode |= 0x4000 | 0x400;
    m.draw_dist = 500;
    if let Some(b) = bi {
        let p = &mut w.mobys[b];
        p.mode |= 0x4000;
        p.draw_dist = 500;
    }
    let m = &mut w.mobys[id];
    for k in 0..48 { m.set_pv_b(pv::SAID + k, 0); }
    m.set_pv_i(pv::SPEECH_WAIT, 0);
    m.set_pv_i(pv::LAST_LINE, -1);
    m.set_pv_i(pv::AIM_SOUND, -1);
    m.set_pv_i(pv::LOOP_SOUND, -1);
    let p = m.pos;
    nearest(w, id, ctx, p);
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::EDGE_A, m.pv_i(pv::NEAR_A));
    m.set_pv_i(pv::EDGE_B, m.pv_i(pv::NEAR_B));
    m.set_state(1, 0);
    anim(w, id, ctx, 0, 0, 0.0);
    if flag(ctx) == 1 {
        clear_level(w, ctx);
        show(w, id, ctx, true);
    } else {
        show(w, id, ctx, false);
    }
    w.changed = true;
    true
}

/// `0x4128a8` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(pv::HOVER) != 0 { hover(w, id, ctx); }
    if w.mobys[id].sub_state != 0 {
        if 118.0 < ctx.hero.pos[2] && ctx.game.cameras.current_class() == 0 {
            ctx.game.cam_tweaks.follow_dist = Some((CAM.0, 0.003));
            ctx.game.cam_tweaks.follow_pivot = Some((CAM.1, 0.003));
        }
        let m = &w.mobys[id];
        let most = m.pv_s(pv::MAX) as f32;
        ctx.game.boss_bar = Some((m.pv_f(pv::HP) / most, 5));
        ctx.game.boss_icon = 11;
        if stagger(ctx, id, 31) { make_rock(w, id, ctx, false); }
        crate::shadow_volumes::probe(w, id, ctx);
    }
    ai::flash(&mut w.mobys[id], pv::FLASH);
    let amb = w.mobys[id].ambient;
    for k in parts(w, id).into_iter().flatten() { w.mobys[k].ambient = amb; }
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) < 0xffff { m.state_ticks = (m.state_ticks as u16).wrapping_add(1) as i16; }
    if let Some(b) = body(w, id) { crate::moby::joint_aim::head_on(w, ctx, id, pv::HEAD, b, 2, 4.0); }
}

/// `0x412a28`: the body at its joint 2 with its yaw and speed, the arms on the body's joints 0 / 1.
fn place_parts(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(b) = body(w, id) else { return };
    if let Some(p) = ctx.host.joint(&w.mobys[id], 2) { w.mobys[b].pos = p; }
    w.mobys[b].rot[2] = w.mobys[id].rot[2];
    let s = speed(w, id, ctx);
    set_speed(w, Some(b), ctx, s);
    w.rebuild(b, &*ctx.host);
    for (k, a) in arms(w, id).into_iter().enumerate() {
        let Some(a) = a else { continue };
        if let Some((rows, at)) = ctx.host.joint_matrix(&w.mobys[b], k) {
            let m = &mut w.mobys[a];
            (m.rows, m.pos) = (rows, at);
        }
        w.rebuild(a, &*ctx.host);
    }
    w.changed = true;
}

/// `0x412790`: its height eased toward its floor + 10 (25 in 13..15 and 17) and a bob over 120 ticks; its jets.
fn hover(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let mut h = if matches!(m.state, 0xd | 0xe | 0xf | 0x11) { 25.0 } else { 10.0 };
    h += ((ctx.game.frame % 120) as f32 / 120.0 * 6.283_185_5 - 3.141_592_7).sin();
    let (mut z, mut v) = (m.pos[2], m.pv_f(pv::VZ));
    ease(m.pv_f(pv::FLOOR) + h, 0.008_333_334, 0.002_777_777_8, 0.166_666_67, &mut z, &mut v);
    m.pos[2] = z;
    m.set_pv_f(pv::VZ, v);
    jets(w, id, ctx);
}

/// `0x414370`: at its joints 0 and 1, falling 15..20 a second, a red-blue puff (frames odd and even in turn) and an
/// orange one (crate::particles::trail).
fn jets(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for j in 0..2 {
        let fall = ctx.game.rng.randf(JET_FALL.0, JET_FALL.1) * TICK;
        let v = [0.0, 0.0, -fall];
        let p = ctx.host.joint(&w.mobys[id], j).unwrap_or(w.mobys[id].pos);
        let (c0, c1, life, base) = if ctx.game.frame & 1 != 0 {
            (0xcf00_00ff, 0xcf, ctx.game.rng.rand_range(0x12, 0x16), 540)
        } else {
            (0x6000_ffff, 0x80, ctx.game.rng.rand_range(0x14, 0x18), 1050)
        };
        if let Some(sys) = ctx.parts.as_deref_mut() { particles::trail(sys, &mut ctx.game.rng, p, v, c0, c1, life, base, -10, true); }
        let life = ctx.game.rng.rand_range(10, 15);
        if let Some(sys) = ctx.parts.as_deref_mut() { particles::trail(sys, &mut ctx.game.rng, p, v, 0xefff_7f4f, 0x00ff_0000, life, 180, -10, true); }
    }
}

/// `0x411fb0`: hovering with Ratchet no longer 24 over its floor, the fall (16, sub 4; its state and sub kept to go
/// back to); on foot with Ratchet over it (and not on the wall, 0x81) it hovers, its sequence 0x16 (the body's walk to
/// 0).
fn hover_check(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let z = ctx.hero.aim_point[2];
    let m = &mut w.mobys[id];
    let floor = m.pv_f(pv::FLOOR);
    if m.pv_i(pv::HOVER) != 0 {
        if z < floor + HIGH {
            m.set_pv_i(pv::HOVER, 0);
            m.set_pv_i(pv::RESUME, m.state as i32);
            m.set_pv_i(pv::RESUME_SUB, m.sub_state as i32);
            m.set_state(0x10, 4);
        }
    } else if ctx.hero.state != 0x81 && floor + HIGH < z {
        m.set_pv_f(pv::VZ, 0.0);
        m.set_pv_i(pv::HOVER, 1);
        if seqs(w, id, ctx).1 != 0x16 { set_anim(w, id, ctx, 0x16, 0, 30); }
        if let Some(b) = body(w, id) { if seqs(w, b, ctx).1 == 10 { set_anim(w, b, ctx, 0, 0, 30); } }
    }
}

// ---- The walk graph ----

/// `0x316728` (the decomp's `func_L01_00311C18`): the point of the segment `a`–`b` nearest `p` across (its z along
/// the segment) and the unclamped share along it.
fn closest(a: V3, b: V3, p: V3) -> (V3, f32) {
    let e = sub(b, a);
    let det = -(e[0] * e[0] + e[1] * e[1]);
    if det.abs() < 0.000_1 { return (a, 0.0); }
    let t = (e[0] * (p[0] - a[0]) + e[1] * (p[1] - a[1])) / -det;
    (add(scale(e, t.clamp(0.0, 1.0)), a), t)
}

/// `0x4120e8(m, p)`: the graph's nearest point to `p` (within 2001): its edge `+0x1f4` / `+0x1f8`, the share `+0x1fc`,
/// the point `+0x200` (`p` itself with none).
fn nearest(w: &mut MobyWorld, id: usize, ctx: &Ctx, p: V3) {
    let path = w.mobys[id].pv_i(pv::PATH);
    let mut best = 2001.0;
    let mut out = p;
    let m = &mut w.mobys[id];
    for (row, r) in GRAPH.iter().enumerate() {
        for &v in r.iter().take_while(|&&v| v != -1) {
            if v < row as i32 { continue; }
            let (Some(a), Some(b)) = (node(ctx, path, row as i32), node(ctx, path, v)) else { continue };
            let (q, t) = closest(a, b, p);
            let g = dist(p, q);
            if g < best {
                best = g;
                m.set_pv_i(pv::NEAR_B, v);
                m.set_pv_f(pv::NEAR_T, t);
                m.set_pv_i(pv::NEAR_A, row as i32);
                out = q;
            }
        }
    }
    m.set_pv_v(pv::DEST, out);
}

/// `0x412268`: the graph's point nearest it (within 1024) left out, a point of the gun's zone (its `+0x60` cuboid)
/// from a random start, and the graph's nearest point to it.
fn pick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let path = w.mobys[id].pv_i(pv::PATH);
    let pos = w.mobys[id].pos;
    let (mut best, mut bestf) = (-1, 1024.0);
    for i in 0..8 {
        let Some(p) = node(ctx, path, i) else { continue };
        let f = dist(pos, p);
        if f < bestf { bestf = f; best = i; }
    }
    let zone = target(w, id).map_or(-1, |t| w.mobys[t].pv_i(0x60));
    let start = ctx.game.rng.randi(8);
    let found = (0..8).map(|i| (i + start) % 8).find(|&k| node(ctx, path, k).is_some_and(|p| ctx.level.in_cuboid(zone, p)) && k != best);
    if let Some(p) = found.and_then(|k| node(ctx, path, k)) { w.mobys[id].set_pv_v(pv::DEST, p); }
    let d = w.mobys[id].pv_v(pv::DEST);
    nearest(w, id, ctx, d);
}

/// `0x4123d0`: the next point toward the nearest edge (`+0x228`, from `+0x22c`): one of its ends on the edge it is on,
/// else through the graph out from the nearest edge's ends.
fn route(m: &mut Moby) {
    let (na, nb, ea, eb) = (m.pv_i(pv::NEAR_A), m.pv_i(pv::NEAR_B), m.pv_i(pv::EDGE_A), m.pv_i(pv::EDGE_B));
    let set = |m: &mut Moby, next: i32, from: i32| { m.set_pv_i(pv::NEXT, next); m.set_pv_i(pv::FROM, from); };
    if na == ea || na == eb { return set(m, na, nb); }
    if nb == ea || nb == eb { return set(m, nb, na); }
    let bit = |k: i32| if (0..32).contains(&k) { 1u32 << k } else { 0 };
    let mut mask = bit(na) | bit(nb);
    let mut acc = 0u32;
    for _ in 0..8 {
        for (i, r) in GRAPH.iter().enumerate() {
            if mask >> i & 1 == 0 { continue; }
            for &v in r.iter().take_while(|&&v| v != -1) {
                acc |= bit(v);
                if v == ea || v == eb { return set(m, v, i as i32); }
            }
        }
        mask |= acc;
    }
}

/// `0x4124f8(m, top)`: the walk along the graph (module docs): toward the nearest point once on its edge, else the
/// next point; its speed eased toward the distance left (120 a second², at most `top`), the sequences at 3× it, the
/// turn toward its way; on foot its feet's hits (joints 0 / 1: 6 across, 50, flags 0x90000); a point reached its edge.
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, top: f32) {
    let s = w.mobys[id].pv_f(pv::SPEED) * 3.0;
    set_speed(w, Some(id), ctx, s);
    set_speed(w, body(w, id), ctx, s);
    let path = w.mobys[id].pv_i(pv::PATH);
    let m = &mut w.mobys[id];
    let (ea, eb) = (m.pv_i(pv::EDGE_A), m.pv_i(pv::EDGE_B));
    let fin = ea == m.pv_i(pv::NEAR_A) && eb == m.pv_i(pv::NEAR_B);
    let point = if fin {
        m.pv_v(pv::DEST)
    } else {
        let (prev, next) = (m.pv_i(pv::FROM), m.pv_i(pv::NEXT));
        if prev == ea || prev == eb || next != ea || next != eb { route(m); }
        node(ctx, path, m.pv_i(pv::NEXT)).unwrap_or(m.pos)
    };
    let mut d = sub(point, m.pos);
    d[2] = 0.0;
    let l = len(d);
    let mut v = m.pv_f(pv::SPEED);
    ease_speed(l, WALK_EASE * TICK2, WALK_EASE * TICK2, top, &mut v);
    m.set_pv_f(pv::SPEED, v);
    let (mut a, mut tv) = (m.rot[2], m.pv_f(pv::TURN_V));
    turn_ease(atan2(d[0], d[1]), SLOW.0, SLOW.0, SLOW.1, &mut a, &mut tv, 0);
    m.rot[2] = a;
    m.set_pv_f(pv::TURN_V, tv);
    m.pos = add(m.pos, set_len(d, v));
    if m.pv_i(pv::HOVER) == 0 {
        for j in 0..2 { ai::attack_joint(w, id, ctx, 6.0, 50.0, 1.0, j, 0x90000, 0, 1, 0); }
    }
    let m = &mut w.mobys[id];
    if !fin && l < 0.5 {
        let (cur, prev) = (m.pv_i(pv::NEXT), m.pv_i(pv::FROM));
        let (a, b) = if prev < cur { (prev, cur) } else { (cur, prev) };
        m.set_pv_i(pv::EDGE_A, a);
        m.set_pv_i(pv::EDGE_B, b);
    }
}

// ---- Its attacks ----

/// `0x412f68`: two blaster shots (crate::moby::classes::thugs::fire, 30 a second, damage `+0x34`) from the muzzle on
/// the body's joint 2: one at Ratchet's aim point, one at a point 4 short of it on its side, up to 4 off.
fn blasters(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(b) = body(w, id) else { return };
    let Some((rows, at)) = ctx.host.joint_matrix(&w.mobys[b], 2) else { return };
    let from = add(through(&rows, MUZZLE), at);
    let aim = ctx.hero.aim_point;
    let dmg = w.mobys[id].pv_b(pv::SHOT_DMG) as f32;
    let v = set_len(sub(aim, from), SHOT_SPEED * TICK);
    super::super::thugs::fire(w, ctx, id, v, from, dmg);
    let g = &mut ctx.game.rng;
    let (a, r) = (g.rand_angle(), g.randf(0.0, 4.0));
    let mut d = add(set_len(sub(aim, w.mobys[id].pos), -4.0), aim);
    d = add(d, [a.cos() * r, a.sin() * r, 0.0]);
    let v = set_len(sub(d, from), SHOT_SPEED * TICK);
    super::super::thugs::fire(w, ctx, id, v, from, dmg);
}

/// `0x412d28`: with Ratchet at a gun, a missile ([`make_missile`]) from the arm in turn's joint 0 (its fire sequence
/// 1), 20 along it, bending through a point 12 out round the gun (up to 60° round from its side, 0..40° up) onto the
/// gun 2 up; the next arm unless its line from the body to the gun is blocked.
fn missile(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(g) = gun(w, ctx) else { return };
    let k = w.mobys[id].pv_i(pv::ARM).clamp(0, 1) as usize;
    let Some(arm) = arms(w, id)[k] else { return };
    let Some((rows, origin)) = ctx.host.joint_matrix(&w.mobys[arm], 0) else { return };
    set_anim(w, arm, ctx, 1, 0, 3);
    let fwd = add(set_len(rows[0], 20.0), origin);
    let mut hero = w.mobys[g].pos;
    hero[2] += 2.0;
    let gp = w.mobys[g].pos;
    let p = w.mobys[id].pos;
    let mut a = atan2(p[0] - gp[0], p[1] - gp[1]);
    let r = &mut ctx.game.rng;
    let s = r.randf_signed(0.0, SPREAD.0);
    a = add_rot(a, s * DEG);
    let up = r.randf(0.0, SPREAD.1) * DEG;
    let bend = add(super::super::shot::polar(SPREAD.2, a, up), hero);
    make_missile(w, ctx, id, [origin, fwd, bend, hero]);
    let next = (w.mobys[id].pv_i(pv::ARM) + 1) & 1;
    w.mobys[id].set_pv_i(pv::ARM, next);
    let Some(b) = body(w, id) else { return };
    let mut c = ARM_POINT;
    if next != 0 { c[1] = -c[1]; }
    let c = add(through(&w.mobys[b].rows, c), w.mobys[b].pos);
    let mut t = w.mobys[g].pos;
    t[2] += 4.0;
    if w.coll_line(ctx, c, t, 2, Some(b)).is_some() { w.mobys[id].set_pv_i(pv::ARM, (next + 1) & 1); }
}

/// `0x413128`: a bomb lobbed (kind 0, damage `+0x36`) from its point (21, 8, 24) jittered up to 1, onto the gun with
/// up to 3 off across, 30 a second under 5..10 a second²; a blast there (size 1).
fn lob(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(t) = target(w, id) else { return };
    let a = ctx.game.rng.rand_vec(0.0, 1.0);
    let m = &w.mobys[id];
    let b = add(add(through(&m.rows, LOB_AT), m.pos), a);
    let mut c = scale(a, 3.0);
    c[2] = 0.0;
    let c = add(c, w.mobys[t].pos);
    let rate = LOB_SPEED * TICK;
    let g = ctx.game.rng.randf(LOB_GRAVITY.0, LOB_GRAVITY.1) * TICK2;
    let mut d = sub(c, b);
    d[2] = 0.0;
    let mut d = set_len(d, rate);
    d[2] = lob_z(b, c, rate, -g);
    blast::burst(w, ctx, b, &blast::Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
    let dmg = w.mobys[id].pv_b(pv::DMG) as f32;
    let owner = body(w, id);
    make_bomb(w, ctx, owner, b, d, false, g, dmg);
}

/// `0x4132f8`: a bomb dropped (kind 1: the ring) from one of its joints 0 / 1, falling 5..10 a second at first, under
/// 1/180 a tick².
fn drop_bomb(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let j = ctx.game.rng.randi(2) as usize;
    let p = ctx.host.joint(&w.mobys[id], j).unwrap_or(w.mobys[id].pos);
    let fall = ctx.game.rng.randf(5.0, 10.0) * TICK;
    let dmg = w.mobys[id].pv_b(pv::DMG) as f32;
    make_bomb(w, ctx, Some(id), p, [0.0, 0.0, -fall], true, 0.005_555_556, dmg);
}

/// `0x413988`'s line and sparks (the port does them here, [`draw_beam`] the quads): from the muzzle on the body's
/// joint 2 to the target (the gun at its raised height, or Ratchet's last place) swept out by `+0x250` away from it
/// toward itself (0.1 down), cut at what the line meets (hit with the damage `+0x36`, flags 0x90001); once a frame
/// four sparks along it (crate::particles::spark, `0x7f004080`) and two streaks each, one along it, one off its end.
fn beam(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(b) = body(w, id) else { return };
    let Some((rows, at)) = ctx.host.joint_matrix(&w.mobys[b], 2) else { return };
    let start = add(through(&rows, MUZZLE), at);
    let tp = match target(w, id) {
        Some(t) => { let m = &w.mobys[t]; [m.pos[0], m.pos[1], m.pv_f(0x78)] }
        None => w.mobys[id].pv_v(pv::LAST),
    };
    let mut end = sub(w.mobys[id].pos, tp);
    end[2] = -0.1;
    let end0 = add(set_len2(end, w.mobys[id].pv_f(pv::AIM_T)), tp);
    let mut end = end0;
    let dmg = w.mobys[id].pv_b(pv::DMG) as f32;
    if let Some(o) = w.coll_line(ctx, start, end0, 0, Some(id)) {
        end = o.point;
        if let Some(k) = o.moby {
            let t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 0x90001, [0.0; 4]);
            w.deliver_hit_any(k, &t, [end[0], end[1], end[2], 1.0]);
        }
    }
    w.snivelak.beam = Some((start, end));
    if w.mobys[id].pv_i(pv::BEAM_FRAME) == ctx.game.frame { return; }
    w.mobys[id].set_pv_i(pv::BEAM_FRAME, ctx.game.frame);
    let delta = sub(end, start);
    for _ in 0..SPARKS {
        let g = &mut ctx.game.rng;
        let life = g.rand_range(SPARK_LIFE.0, SPARK_LIFE.1);
        let spin = g.randf_signed(1.0, 2.0) as i32;
        let size = g.randf(SPARK_SIZE.0, SPARK_SIZE.1) * 210_000.0;
        let v = set_len(delta, g.randf(SPARK_SPEED.0, SPARK_SPEED.1) * TICK);
        let t = g.randf(0.0, 1.0);
        let at = add(end, scale(sub(start, end), t));
        if let Some(sys) = ctx.parts.as_deref_mut() {
            particles::spark(sys, &mut ctx.game.rng, size, 500.0, 1.0, 0.0, 1.0, at, v, life as i16, (SPARK_RGBA >> 24) as i32, SPARK_RGBA & 0xff_ffff, spin);
        }
        let fall = ctx.game.rng.randf(10.0, 20.0) * TICK2;
        if let Some(sys) = ctx.parts.as_deref_mut() { particles::streak(sys, fall, at, v, life as i16); }
        let g = &mut ctx.game.rng;
        let mut v = g.rand_vec(SPARK_BURST.0 * TICK, SPARK_BURST.1 * TICK);
        v[2] += g.randf(SPARK_SPEED.0, SPARK_SPEED.1) * TICK;
        if let Some(sys) = ctx.parts.as_deref_mut() { particles::streak(sys, f32::from_bits(0x3b84_e478), end, v, life as i16); }
    }
}

/// `0x413988`'s quads: two additive strips (FX 0xb) from the muzzle to the beam's end, 1 and 0.2 across the view, in
/// `0x40000040` / `0x80000080` faded in by `+0x254`.
pub fn draw_beam(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let Some((start, end)) = w.snivelak.beam else { return };
    let blend = w.mobys[id].pv_f(pv::AIM_BLEND);
    let side = set_len(cross(sub(cam, start), sub(end, start)), 1.0);
    for k in 0..2 {
        let c = crate::hud::lerp_rgba(blend, BEAM_RGBA[k] & 0xff00_0000, BEAM_RGBA[k]);
        let corner = |j: usize| add(scale(side, if j & 1 != 0 { BEAM_W[k] } else { -BEAM_W[k] }), if j < 2 { start } else { end });
        out.push(crate::fx::FxQuad { fx: BEAM_FX, additive: true, corners: [corner(0), corner(1), corner(2), corner(3)], st: [[0.5, 0.0], [0.5, 1.0], [0.5, 0.0], [0.5, 1.0]], rgba: [c; 4] });
    }
}

// ---- The voices ----

/// `0x413fd0(m, k)`: with the dialogue free and not the last line again, line `k` of its tables (`+0x280` / `+0x2d0`,
/// `& 0x3fff` + 2000) asked for, marked said, 900 ticks before it talks again.
fn say(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: usize) {
    if !ctx.host.speech_free() { return; }
    let m = &w.mobys[id];
    if k as i32 == m.pv_i(pv::LAST_LINE) { return; }
    let v = if k < 20 { m.pv_i(pv::LINES_A + 4 * k) } else { m.pv_i(pv::LINES_B + 4 * (k - 20)) };
    ctx.host.request_speech((v & 0x3fff) + 0x7d0);
    let m = &mut w.mobys[id];
    m.set_pv_b(pv::SAID + k, 1);
    m.set_pv_i(pv::SPEECH_WAIT, 900);
    m.set_pv_i(pv::LAST_LINE, k as i32);
}

/// `0x414080(m, a, b)`: from a random one of `a..=b` the first not said yet; all said, they are cleared and the random
/// one said.
fn say_one_of(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, a: usize, b: usize) {
    if b < a { return; }
    let n = b - a + 1;
    let mut k = ctx.game.rng.randi(n as i32) as usize;
    for _ in 0..n {
        if w.mobys[id].pv_b(pv::SAID + a + k) == 0 { return say(w, id, ctx, a + k); }
        k = (k + 1) % n;
    }
    for i in 0..n { w.mobys[id].set_pv_b(pv::SAID + a + i, 0); }
    say(w, id, ctx, a + k);
}

/// `0x413f18`: Ratchet's death's first tick a line of 28..32 and the deaths counted; a held line let go; the talk's
/// wait counted down.
fn voices(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.hero.state == 0x39 && ctx.hero.timer == 0 {
        say_one_of(w, id, ctx, 28, 32);
        w.snivelak.deaths += 1;
    }
    if ctx.host.speech_held() { ctx.host.speech_go(); }
    w.mobys[id].count_i(pv::SPEECH_WAIT);
}

// ---- The fight's start and end, the arena ----

/// `0x414170` (the intro scene's end, or Ratchet back in): after the intro Ratchet at `+0x1b0` and it at `+0x1b8`,
/// the music `+0x1a4` (back in: with the stinger `+0x1ac`); the fight begun (`0x1395b8 + 0x83` = 1), the level cleared
/// ([`clear_level`]), all shown, the bridge `+0x190` hooked, the checkpoint at `+0x198`, a line of 0..2; 2 (sub 1).
fn start_fight(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (music, stinger) = (m.pv_i(pv::MUSIC), m.pv_i(pv::STINGER));
    if m.pv_i(pv::INTRO) != 0 {
        w.mobys[id].set_pv_i(pv::INTRO, 0);
        crate::moby::talk::put_hero(ctx, w.mobys[id].pv_i(pv::BACK_CUBOID));
        if let Some((p, r)) = ctx.level.cuboid_place(w.mobys[id].pv_i(pv::BOSS_CUBOID)) {
            let m = &mut w.mobys[id];
            (m.pos, m.rot) = (p, r);
        }
        ctx.host.music_request(music, -1);
    } else {
        ctx.host.music_request(music, stinger);
    }
    set_flag(ctx, 1);
    clear_level(w, ctx);
    show(w, id, ctx, true);
    if let Some(b) = usize::try_from(w.mobys[id].pv_i(pv::BRIDGE)).ok().and_then(|b| w.mobys.get_mut(b)) { b.once |= 4; }
    if let Some((p, r)) = ctx.level.cuboid_place(w.mobys[id].pv_i(pv::CHECKPOINT)) { ctx.game.save_checkpoint(p, r); }
    say_one_of(w, id, ctx, 0, 2);
    // `0x306e20(0, −1)`: the autosave (crate::game::GameState::autosave).
    ctx.game.autosave(-1);
    w.mobys[id].set_state(2, 1);
}

/// `0x414298` (the end scenes' end): the fight won (`0x1395b8 + 0x83` = 2), the bridge back (crate::moby::classes::
/// level16::drawbridge::swing at 0), Ratchet at `+0x188` and the checkpoint there, planet 0x11 unlocked, its parts
/// gone, the music `+0x1a0`, itself gone.
fn end_fight(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    set_flag(ctx, 2);
    if let Some(b) = usize::try_from(w.mobys[id].pv_i(pv::BRIDGE)).ok().filter(|&b| b < w.mobys.len()) { super::drawbridge::swing(w, b, 0.0); }
    crate::moby::talk::put_hero(ctx, w.mobys[id].pv_i(pv::END_CUBOID));
    let (p, r) = (ctx.hero.pos, ctx.hero.rot);
    ctx.game.save_checkpoint(p, r);
    ctx.game.unlock_planet(0x11);
    for k in parts(w, id).into_iter().flatten() { w.delete(k); }
    ctx.host.music_request(w.mobys[id].pv_i(pv::MUSIC_END), -1);
    ctx.game.autosave(-1);
    w.delete(id);
}

/// `0x412bd0`: the level's other fighters out of the way — 0x0e71..0x0e73, 0x0c86 / 0x0c87, 0x113a, 0x1162, 0x07fb
/// gone; the drones, troopers and gunships (0x0f35, 0x0ea3, 0x0ea4, 0x0ebb) with a health block gone, or (its byte
/// `+0xe` 1) left giving no bolts or XP, the byte 2.
fn clear_level(w: &mut MobyWorld, _ctx: &mut Ctx) {
    for k in 0..w.mobys.len() {
        let m = &w.mobys[k];
        if 0x80 <= m.state { continue; }
        let c = m.o_class;
        if (0x0e71..=0x0e73).contains(&c) || matches!(c, 0x0c87 | 0x0c86 | 0x113a | 0x1162 | 0x07fb) { w.delete(k); }
        let m = &w.mobys[k];
        if 0x80 <= m.state || !matches!(c, 0x0f35 | 0x0ea3 | 0x0ea4 | 0x0ebb) { continue; }
        if m.mode & ai::AI == 0 || m.pv_i(0) == 0 { continue; }
        let h = ai::health(m);
        if m.pv_b(h + 0xe) == 1 {
            let m = &mut w.mobys[k];
            (m.bolts, m.xp) = (0, 0);
            m.set_pv_b(h + 0xe, 2);
        } else {
            w.delete(k);
        }
    }
}

/// `0x4133a8(m, on)`: its groups (`+0x184`, `+0x17c`, `+0x180`) and itself with its parts shown with their collision
/// (`+0x184`'s updated again); off, the groups hidden without collision (`+0x184`'s not updated) and, before the fight,
/// itself and its parts too, it and its body stopped.
fn show(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, on: bool) {
    for off in [pv::GROUP_C, pv::GROUP_A, pv::GROUP_B] {
        let g = w.mobys[id].pv_i(off);
        if g & 0xff == 0xff { continue; }
        let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { continue };
        for k in list {
            let has = w.mobys.get(k).and_then(|m| w.class(m.o_class)).is_some_and(|c| c.has_collision);
            let Some(m) = w.mobys.get_mut(k) else { break };
            if on {
                m.mode = (m.mode & 0xffbe) | 0x1000;
                if off == pv::GROUP_C { m.mode &= 0xfffd; }
                m.has_collision = has;
            } else {
                m.has_collision = false;
                m.mode = if off == pv::GROUP_C { (m.mode & 0xefff) | 0x43 } else { (m.mode | 0x41) & 0xefff };
            }
        }
    }
    let all: Vec<usize> = std::iter::once(Some(id)).chain(parts(w, id)).flatten().collect();
    if on {
        for k in all {
            let has = w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision);
            let m = &mut w.mobys[k];
            m.mode = (m.mode & 0xffbe) | 0x1000;
            m.has_collision = has;
        }
    } else if flag(ctx) == 0 {
        for k in all {
            let m = &mut w.mobys[k];
            m.has_collision = false;
            m.mode = (m.mode | 0x41) & 0xefff;
        }
        set_speed(w, Some(id), ctx, 0.0);
        set_speed(w, body(w, id), ctx, 0.0);
    }
    w.changed = true;
}

/// `0x2d30d0(k, up, down)`: the white flash asked for when none is under way — to full over `up` ticks, held `k`,
/// out over `down`.
fn white_start(w: &mut MobyWorld, k: i32, up: f32, down: f32) {
    let s = &mut w.snivelak.white;
    if s.2 == 0.0 { *s = (1.0 / up, 1.0 / down, 1.0, k); }
}

/// `0x2d3128` (the engine's tick of it; the port ticks it here, the only one asking for it on level 16).
fn white_tick(w: &mut MobyWorld, ctx: &mut Ctx) {
    let s = &mut w.snivelak.white;
    if s.0 == 0.0 { return; }
    let f = &mut ctx.game.fade_white;
    if s.2 != 0.0 { approach(s.2, s.0, f); } else { approach(s.2, s.1, f); }
    if *f == s.2 && countdown(&mut s.3) { s.2 = 0.0; }
    if s.2 <= 0.0 && *f <= 0.0 { s.0 = 0.0; }
}

// ---- The arms 0x07e8 ----

/// 0x07e8 (`0x3ab508`): its fire sequence 1 done, back to 0.
pub fn update_07e8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if seqs(w, id, ctx).1 == 1 && wrapped(w, id, ctx) { set_anim(w, id, ctx, 0, 0, 1); }
}

// ---- The missile 0x0f4e ----

mod mv {
    pub const POINTS: usize = 0x00;
    pub const T: usize = 0x40;
    pub const RATE: usize = 0x50;
    pub const SOUND: usize = 0x68;
}

/// `0x40ef08(owner, points)`: a missile 0x0f4e (scale ×6) at the first of its four points (its looped sound none), a
/// blast there (size 2), its first leg's ease started (`0x318cf8`: 1 / 1), its sequence 1.
fn make_missile(w: &mut MobyWorld, ctx: &mut Ctx, owner: usize, pts: [V3; 4]) {
    if w.class(MISSILE).is_none() { return; }
    let scale = w.class(MISSILE).map_or(1.0, |c| c.scale);
    let k = w.create(MISSILE);
    w.mobys[k].set_pv_i(mv::SOUND, -1);
    w.adopt(k, Some(owner), &*ctx.host);
    w.mobys[k].pos = pts[0];
    blast::burst(w, ctx, pts[0], &blast::Blast { size: 2.0, bursts: [0; 3], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
    let m = &mut w.mobys[k];
    for (i, p) in pts.iter().enumerate() { m.set_pv_v(mv::POINTS + 0x10 * i, *p); }
    let mut v = m.pv_f(mv::RATE);
    ease_speed(1.0, 1.0, TICK2, TICK, &mut v);
    m.set_pv_f(mv::RATE, v);
    if seqs(w, k, ctx).1 != 1 { set_anim(w, k, ctx, 1, 0, 1); }
    w.mobys[k].scale = scale * 6.0;
}

/// 0x0f4e (`0x40e910`): its looped sound 0 kept; along its three legs, each eased to 1 (the first 1 / 1 a second², the
/// others ½ / ½) once the one before is past half, its sequence 0 near the end; turning to Ratchet (12° a tick at
/// most); its exhaust ([`exhaust`]). Hit (flags 0x10000) or in the scene mode: sound 2, three pieces (balls), a blast,
/// gone. Its last leg done, or its line meeting something but the boss's body or arms, or (late in its second leg)
/// its sphere of 1.5 touching the world: state 1, the explosion (crate::moby::classes::explosion) and its hit (2
/// across, 5), sound 1, gone.
pub fn update_0f4e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if !ctx.host.sound_playing(w.mobys[id].pv_i(mv::SOUND)) {
                let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                w.mobys[id].set_pv_i(mv::SOUND, h);
            }
            let old = w.mobys[id].pos;
            let m = &w.mobys[id];
            let mut pos = m.pv_v(mv::POINTS);
            const A: [f32; 3] = [1.0, 0.5, 0.5];
            for i in 0..3 {
                let m = &mut w.mobys[id];
                if i == 0 || 0.5 < m.pv_f(mv::T + 4 * (i - 1)) {
                    let (mut t, mut v) = (m.pv_f(mv::T + 4 * i), m.pv_f(mv::RATE + 4 * i));
                    ease(1.0, A[i] * TICK2, A[i] * TICK2, A[i] * TICK, &mut t, &mut v);
                    m.set_pv_f(mv::T + 4 * i, t);
                    m.set_pv_f(mv::RATE + 4 * i, v);
                    if i == 2 && 0.95 < m.pv_f(mv::T + 4) && seqs(w, id, ctx).1 != 0 { set_anim(w, id, ctx, 0, 0, 20); }
                }
                let m = &w.mobys[id];
                let d = sub(m.pv_v(mv::POINTS + 0x10 * (i + 1)), m.pv_v(mv::POINTS + 0x10 * i));
                pos = add(pos, scale(d, m.pv_f(mv::T + 4 * i)));
            }
            let hero = ctx.hero.pos;
            let m = &mut w.mobys[id];
            m.pos = pos;
            let to = atan2(hero[0] - pos[0], hero[1] - pos[1]);
            approach_angle(to, 0.209_439_55, &mut m.rot[2], 0);
            exhaust(w, id, ctx);
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x10000 != 0) || in_scene_mode(ctx) {
                ctx.host.play_sound(&w.mobys[id], 2, 0);
                stop(ctx, w.mobys[id].pv_i(mv::SOUND));
                w.mobys[id].set_pv_i(mv::SOUND, -1);
                let (rows, rot, class) = (w.mobys[id].rows, w.mobys[id].rot, w.mobys[id].o_class);
                for i in 0..3u8 {
                    let v = match i { 0 => scale(rows[1], 0.15), 1 => scale(rows[1], -0.15), _ => scale(rows[2], 0.100_000_01) };
                    let spin = ctx.game.rng.randf_signed(90.0, 180.0);
                    let g = ctx.game.rng.randf(5.0, 15.0);
                    shards::spawn_ball(w, ctx, class, Some(id), pos, rot, spin, g, i, v, 0xff);
                }
                blast::burst(w, ctx, pos, &blast::Blast { size: 1.0, bursts: [0; 3], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
                w.delete(id);
                return;
            }
            if 1.0 <= w.mobys[id].pv_f(mv::T + 8) { w.mobys[id].set_state(1, -1); }
            if let Some(o) = w.coll_line(ctx, old, pos, 0, Some(id)) {
                if o.moby.and_then(|k| w.mobys.get(k)).is_some_and(|k| k.o_class == BODY || k.o_class == ARM) { return; }
            } else {
                if w.mobys[id].pv_f(mv::T + 4) < 0.9 { return; }
                let touch = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, 1.5, crate::collision::QueryFlags(0), Some(id)));
                if touch.is_none() { return; }
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            stop(ctx, w.mobys[id].pv_i(mv::SOUND));
            w.mobys[id].set_pv_i(mv::SOUND, -1);
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            let pos = w.mobys[id].pos;
            let dir = set_len(sub(ctx.host.camera_pos(), pos), 1.0);
            super::super::explosion::start(w, ctx, 0.0, pos, dir);
            ai::attack_sphere(w, id, ctx, 2.0, 5.0, 1.0, pos, 0x10001, 0, 1, 0);
            w.delete(id);
        }
        _ => {}
    }
}

/// `0x40f098`: its exhaust — a glint at it (crate::particles::glint: 4, `0x60006080`, 5..10 ticks) and one more up
/// to 0.25 off falling 2..5.5 a second (0.5, `0x60004090`, 40..90 ticks).
fn exhaust(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pos = w.mobys[id].pos;
    let hero = ctx.hero.pos;
    let g = &mut ctx.game.rng;
    let v = g.rand_vec(0.0, 0.0);
    let life = g.rand_range(5, 10) & 0xff;
    let rot = g.randi(255) as u8;
    if let Some(sys) = ctx.parts.as_deref_mut() { particles::glint(sys, 4.0, pos, v, 0x6000_6080, life as u16, rot, false, hero); }
    let g = &mut ctx.game.rng;
    let p = add(g.rand_vec(0.0, 0.25), pos);
    let v = [0.0, 0.0, -(g.randf(2.0, 5.5) * TICK)];
    let life = g.rand_range(40, 90) & 0xff;
    let rot = g.randi(255) as u8;
    if let Some(sys) = ctx.parts.as_deref_mut() { particles::glint(sys, 0.5, p, v, 0x6000_4090, life as u16, rot, false, hero); }
}

// ---- The bomb 0x12be ----

mod bv {
    pub const VEL: usize = 0x00;
    pub const SPIN: usize = 0x10;
    pub const KIND: usize = 0x20;
    pub const GRAVITY: usize = 0x24;
    pub const DAMAGE: usize = 0x28;
    pub const RADIUS: usize = 0x2c;
    pub const ALPHA: usize = 0x30;
    pub const SCROLL: usize = 0x34;
}

/// `0x420628(owner, pos, vel, kind, gravity, damage)`: a bomb 0x12be at `pos` (its owner's child), at random angles,
/// spinning (`+0x10`), flying `vel` under `gravity`, scale × 0.2, its light `0x800000c0`.
#[allow(clippy::too_many_arguments)]
fn make_bomb(w: &mut MobyWorld, ctx: &mut Ctx, owner: Option<usize>, pos: V3, vel: V3, ring: bool, gravity: f32, damage: f32) {
    if w.class(BOMB).is_none() { return; }
    let scale = w.class(BOMB).map_or(1.0, |c| c.scale);
    let k = w.create(BOMB);
    w.adopt(k, owner, &*ctx.host);
    let g = &mut ctx.game.rng;
    let rot = g.rand_vec(-std::f32::consts::PI, std::f32::consts::PI);
    let axis = g.rand_vec(1.0, 1.0);
    let s = g.randf_signed(180.0, 360.0) * DEG;
    let m = &mut w.mobys[k];
    m.pos = pos;
    m.rot = rot;
    m.rows = crate::moby::attach::rows_of(rot);
    m.mode |= 0x100;
    m.set_pv_q(bv::SPIN, crate::quat::axis_quat(s, axis));
    m.set_pv_v(bv::VEL, vel);
    m.set_pv_f(bv::GRAVITY, gravity);
    m.set_pv_i(bv::KIND, ring as i32);
    m.set_pv_f(bv::DAMAGE, damage);
    m.scale = scale * 0.2;
    m.set_ambient_word(0x8000_00c0);
    m.set_state(0, -1);
}

/// 0x12be (`0x4202c8`): **0** flying under its gravity, its glint trail (`0x420588`), its line from its last place
/// (its owner left out) meeting something: there, 2 (the ring) or 1. **1** a blast (size 1) and its hit (2 across,
/// its damage 6), gone. **2** the ring: on its first tick a blast (size ⅔), hidden without collision, 0.5 up, alpha
/// 1; it grows 10 a second, its scroll 1/15 a tick; at 20 across it fades out (1/15 a tick), gone when faded; else its
/// draw ([`draw_ring`]) and its hits (`0x420798`: the ring's 32 segments' lines, flags 9, its damage).
pub fn update_12be(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let old = m.pos;
            let mut v = m.pv_v(bv::VEL);
            m.pos = add(m.pos, v);
            v[2] -= m.pv_f(bv::GRAVITY);
            m.set_pv_v(bv::VEL, v);
            let (pos, parent) = (m.pos, m.parent);
            let hero = ctx.hero.pos;
            let g = &mut ctx.game.rng;
            let life = g.rand_range(3, 7) & 0xff;
            let rot = g.randi(255) as u8;
            if let Some(sys) = ctx.parts.as_deref_mut() { particles::glint(sys, 2.5, sub(pos, v), v, 0x6000_6080, life as u16, rot, false, hero); }
            if let Some(o) = w.coll_line(ctx, old, pos, 0, parent) {
                let m = &mut w.mobys[id];
                m.pos = o.point;
                let s = if m.pv_i(bv::KIND) != 0 { 2 } else { 1 };
                m.set_state(s, -1);
            }
        }
        1 => {
            let pos = w.mobys[id].pos;
            blast::burst(w, ctx, pos, &blast::Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
            ai::attack_sphere(w, id, ctx, 2.0, 6.0, 1.0, pos, 0x10001, 0, 1, 0);
            w.delete(id);
        }
        2 => {
            if w.mobys[id].first() {
                let pos = w.mobys[id].pos;
                blast::burst(w, ctx, pos, &blast::Blast { size: f32::from_bits(0x3f2a_7efa), bursts: [0x14, 0x0f, 0x0f], puffs: 0x0f, streaks: 0x23, light: true, shake: false });
                let m = &mut w.mobys[id];
                m.has_collision = false;
                m.mode = (m.mode | 0x41) & 0xefff;
                m.pos[2] += 0.5;
                m.set_pv_f(bv::ALPHA, 1.0);
                w.changed = true;
            }
            let m = &mut w.mobys[id];
            let mut s = m.pv_f(bv::SCROLL) + 0.066_666_67;
            if 1.0 < s { s -= 1.0; }
            m.set_pv_f(bv::SCROLL, s);
            let r = m.pv_f(bv::RADIUS) + 10.0 * TICK;
            m.set_pv_f(bv::RADIUS, r);
            if 20.0 <= r {
                let mut a = m.pv_f(bv::ALPHA);
                approach(0.0, 0.066_666_67, &mut a);
                m.set_pv_f(bv::ALPHA, a);
            }
            if m.pv_f(bv::ALPHA) <= 0.0 { return w.delete(id); }
            ring_hits(w, id, ctx);
            w.register_draw_first(id);
        }
        _ => {}
    }
}

/// The ring's 33 points (the last the first again) at its radius about it, `w` its half-width.
fn ring_points(m: &Moby, w: f32) -> [[f32; 4]; 33] {
    let r = m.pv_f(bv::RADIUS);
    let mut p = [[0.0; 4]; 33];
    for (i, q) in p.iter_mut().enumerate().take(32) {
        let a = ((i << 1) as f32) * 0.098_174_77 - std::f32::consts::PI;
        *q = [a.cos() * r + m.pos[0], a.sin() * r + m.pos[1], m.pos[2], w];
    }
    p[32] = p[0];
    p
}

/// `0x420798`'s hits: each segment's line (flags 9, itself left out) hits what it meets with its damage (flags 1,
/// its class's word).
fn ring_hits(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let p = ring_points(m, 0.2);
    let mut t = crate::moby::hits::HitTemplate::new(m.pv_f(bv::DAMAGE), Some(id), 1, [0.0; 4]);
    t.kind = (m.o_class as u16 as u32) << 16 | 0x100;
    for j in 0..32 {
        let (a, b) = ([p[j][0], p[j][1], p[j][2]], [p[j + 1][0], p[j + 1][1], p[j + 1][2]]);
        if let Some(o) = w.coll_line(ctx, a, b, 9, Some(id)) {
            if let Some(k) = o.moby { w.deliver_hit_any(k, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
        }
    }
}

/// `0x420798`'s ribbons (crate::fx::ribbon, FX 23 / 14, × 3): the ring 0.2 wide in `0x60006000` / `0x60000060` faded
/// by its alpha scrolling one way, then 0.05 wide (but its last point) the other.
pub fn draw_ring(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let a = m.pv_f(bv::ALPHA);
    let s = m.pv_f(bv::SCROLL);
    let c = [crate::hud::lerp_rgba(a, 0x0000_6000, 0x6000_6000), crate::hud::lerp_rgba(a, 0x0000_0060, 0x6000_0060)];
    let mut p = ring_points(m, 0.2);
    crate::fx::ribbon(out, 3.0, s, &p, cam, [23, 14], c);
    for q in p.iter_mut().take(32) { q[3] = 0.05; }
    crate::fx::ribbon(out, 3.0, -s, &p, cam, [23, 14], c);
}

// ---- The rock 0x09b0 ----

mod rv {
    pub const VEL: usize = 0x00;
    pub const GOAL: usize = 0x10;
    pub const SPIN: usize = 0x20;
    pub const LIFE: usize = 0x30;
    pub const GRAVITY: usize = 0x34;
}

/// `0x4144d8(m, aimed)`: a rock 0x09b0 (its update [`update_rock`]). Aimed: from 20 over the camera to a point 25 out
/// beside it a quarter turn round from the gun, 56 over its floor, over 200 ticks (life 240). Else across the rocks'
/// box `+0x19c` from one side to the other (600 ticks), unless its start is in view. Facing its way.
fn make_rock(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, aimed: bool) -> Option<usize> {
    w.class(ROCK)?;
    let k = w.create(ROCK);
    w.adopt(k, Some(id), &*ctx.host);
    let m = &mut w.mobys[k];
    m.update = Some(update_rock);
    m.mode &= !crate::moby::mode::NO_UPDATE;
    m.set_state(0, -1);
    if aimed {
        let Some(t) = target(w, id) else { w.delete(k); return None };
        let mut p = ctx.host.camera_pos();
        p[2] += 20.0;
        let tp = w.mobys[t].pos;
        let a = add_rot(atan2(tp[0] - p[0], tp[1] - p[1]), 1.570_796_4);
        let mut goal = add([a.cos() * ROCK_REACH, a.sin() * ROCK_REACH, 0.0], w.mobys[id].pos);
        goal[2] = w.mobys[id].pv_f(pv::FLOOR) + ROCK_HIGH;
        let m = &mut w.mobys[k];
        m.pos = p;
        m.set_pv_v(rv::GOAL, goal);
        m.set_pv_v(rv::VEL, scale(sub(goal, p), 1.0 / ROCK_TICKS));
        m.set_pv_i(rv::LIFE, (ROCK_TICKS * f32::from_bits(0x3f99_999a)) as i32);
    } else {
        let g = &mut ctx.game.rng;
        let mut a = [g.randf(-1.0, 1.0), 0.0, 0.0];
        a[2] = g.randf(-1.0, 1.0);
        let mut b = [g.randf(-1.0, 1.0), 0.0, 0.0];
        b[2] = g.randf(-1.0, 1.0);
        a[1] = if g.randi(2) != 0 { 1.0 } else { -1.0 };
        b[1] = -a[1];
        let box_ = w.mobys[id].pv_i(pv::ROCKS);
        let (Some(a), Some(b)) = (in_volume(ctx, box_, a), in_volume(ctx, box_, b)) else { w.delete(k); return None };
        if ctx.view.is_none_or(|v| !v.culled(255.0, [a[0], a[1], a[2], 3.0])) { w.delete(k); return None; }
        let m = &mut w.mobys[k];
        m.pos = a;
        m.set_pv_v(rv::GOAL, b);
        m.set_pv_v(rv::VEL, scale(sub(b, a), 0.001_666_666_7));
        m.set_pv_i(rv::LIFE, 600);
    }
    let m = &mut w.mobys[k];
    let v = m.pv_v(rv::VEL);
    m.rot[2] = atan2(v[0], v[1]);
    w.rebuild(k, &*ctx.host);
    Some(k)
}

/// `0x414a60`: past its goal (its way and the way to it apart).
fn rock_arrived(m: &Moby) -> bool { dot(m.pv_v(rv::VEL), sub(m.pv_v(rv::GOAL), m.pos)) <= 0.0 }

/// `0x414ab8(rock, bone)`: held (1) in the body's joint 3 frame turned half round.
fn rock_hold(w: &mut MobyWorld, r: usize, _ctx: &mut Ctx, (rows, at): ([V3; 3], V3)) {
    let m = &mut w.mobys[r];
    m.set_state(1, -1);
    m.mode |= 0x100;
    m.rows = crate::moby::attach::turn(&rows, &crate::moby::attach::rows_of(ROCK_HELD));
    m.pos = at;
    w.changed = true;
}

/// `0x414b28(rock, vel, g)`: thrown (2) at `vel` under `g`, tumbling about its way × up (`0x31b1c0`, 0.0582).
fn rock_throw(w: &mut MobyWorld, r: usize, vel: V3, g: f32) {
    let m = &mut w.mobys[r];
    m.set_pv_v(rv::VEL, vel);
    m.set_pv_f(rv::GRAVITY, g);
    m.set_pv_q(rv::SPIN, crate::quat::axis_quat(0.058_177_65, cross(vel, [0.0, 0.0, 1.0])));
    m.set_state(2, -1);
}

/// The rock (`0x4147d8`): **0** flying its way, gone when its life is out unseen. **2** thrown: under its gravity,
/// tumbling; hit (flags 0x10000) or after 90 ticks touching the world (a sphere of 1): the explosion facing the
/// camera, its hit (4 across, 20, pushed 2), six pieces off it, gone.
pub fn update_rock(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, m.pv_v(rv::VEL));
            if m.count_i(rv::LIFE) && m.drawn == 0 { w.delete(id); }
        }
        2 => {
            let m = &mut w.mobys[id];
            let mut v = m.pv_v(rv::VEL);
            m.pos = add(m.pos, v);
            v[2] -= m.pv_f(rv::GRAVITY);
            m.set_pv_v(rv::VEL, v);
            m.rows = crate::quat::turn_rows(&m.rows, m.pv_q(rv::SPIN));
            let mut hit = w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x10000 != 0);
            let m = &mut w.mobys[id];
            let old = m.state_ticks as u16;
            m.state_ticks = old.wrapping_add(1) as i16;
            let pos = m.pos;
            if 90 < old && ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, pos, 1.0, crate::collision::QueryFlags(0), Some(id))).is_some() { hit = true; }
            if hit {
                let dir = set_len(sub(ctx.host.camera_pos(), pos), 1.0);
                super::super::explosion::start(w, ctx, 0.0, pos, dir);
                ai::attack_sphere(w, id, ctx, 4.0, 20.0, 2.0, pos, 0x10001, 0, 1, 0);
                let rows = w.mobys[id].rows;
                let rot = crate::moby::attach::angles_of(&rows);
                w.mobys[id].rot = rot;
                let (class, vel) = (w.mobys[id].o_class, w.mobys[id].pv_v(rv::VEL));
                for i in 0..6u8 {
                    let g = &mut ctx.game.rng;
                    let v = add(g.rand_vec(0.0, 0.333_333_34), vel);
                    let spin = g.randf(180.0, 720.0);
                    let grav = g.randf(5.0, 15.0);
                    shards::spawn_shard(w, ctx, class, Some(id), pos, rot, Some(rows), spin, grav, i, v, 0xff);
                }
                w.delete(id);
            }
        }
        _ => {}
    }
}

