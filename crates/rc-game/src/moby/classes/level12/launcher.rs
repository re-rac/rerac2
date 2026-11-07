//! Level 12's launcher system: the carrier 0x0fa5 (`0x41a510`, 1 placed) lands on its first path, lets out the
//! critters 0x0bb8 (`0x3e0d78`, 10 placed) one every 45 ticks and brings out its guide 0x0fec (`0x41b2a8`, 1 placed;
//! also level 13's `0x41eea0`); the guide walks its path, talking at its stops, and each stop sends the critters on to
//! their next waypoint; past the sixth they come back aboard, and once they all rest the carrier flies off and comes
//! again when the guide is back home. The critters fly on the enemy framework ([`crate::moby::ai`]); the walkers
//! 0x0fb4 ([`super::walker`]) take them at their pick-up waypoint.
//!
//! The L12 engine functions with the L00 ones the port has: `0x32c370` = L00 `0x328b80` (crate::moby::ai::flight),
//! `0x32b628` = `0x327e38` (take_damage), `0x331d30` = `0x32dcf0` (senses), `0x330c28` = `0x32cd20` (move_step; `0x330dc0`
//! = `0x32ceb8` the same in place), `0x330a60` = L02 `0x3327f0` (crate::moby::ai::move_with), `0x32df40` / `0x32e3d0` =
//! L02 `0x32f8b0` / `0x32fd40` (crate::path_network::build / steer), `0x323c00` = `0x3201e8` (flash), `0x327a50` =
//! `0x324208` (crate::moby::joint_aim::aim), `0x321470` = `0x31da58` (crate::shadow_volumes::probe_near), `0x324ac0` =
//! `0x3210a8` (crate::moby::blast::smoke_burst), `0x31e7c8` = `0x31adc0` (crate::moby::bolts::drop), `0x325270` =
//! `0x321858` (crate::level::LevelData::in_cuboid), `0x320968` = `0x31cf68` (in_polygon), `0x320820` = `0x31ce20`
//! (crate::moby::ai::ground_z), `0x322790` = `0x31ed78` (crate::hero::physics::ease), `0x349658` = `0x343fb8`
//! (crate::paths::pose), `0x33ad98` = `0x335bb8` (crate::particles::puff), `0x2f8958` = `0x2f5270` (the help),
//! `0x2efc98` = L02 `0x2f0170` (crate::moby::classes::teleport::start), `0x3ca410` = L02 `0x3cb600` (teleport::burst),
//! `0x2f1658` = `0x2edf78` (crate::fx::sprite), `0x316220` = `0x3127c8` (the joint's place), `0x323f50` = `0x320538`
//! (crate::moby::blast::explode), `0x3177c8` = `0x313dc0` (the rebuild).
//!
//! **The critter 0x0bb8.** Pvars: health block `+0x20` (its limit `+0x24`), the flash `+0x50`, reaction `+0x60`, its
//! network `+0x110` (32 records, `+0x790` of them, from path `+0x510`), the carrier `+0x514`, its waypoints `+0x518`
//! (six cuboids), the four look-at cuboids `+0x530`, its pick-up waypoint `+0x540` (0 none), the walkers' group
//! `+0x544`, the moby it looks at `+0x548` (the guide), the carrier's door cuboid `+0x54c`, movement `+0x550`, two aim
//! records `+0x640` / `+0x6c0`, the launch's landing `+0x740` and the way back in `+0x750`, its home `+0x760`, its place
//! in its first waypoint's frame `+0x770`, the look point `+0x780`, the waypoint `+0x794` and the one asked `+0x798`,
//! the network's places `+0x79c` / `+0x7a0`, the network's margin `+0x7a4`, the waits `+0x7a8` / `+0x7b0` / `+0x7bc`,
//! its speed `+0x7ac`, the swerve `+0x7b4`, the walker that takes it `+0x7b8`.
//! * **The set-up** ([`setup`] `0x3e1890`, state 0): its pieces mask `1 << randi(6)`; the reaction (state 9, sub 8,
//!   sequences 8 / 9 / 10, keys 0.1 / 1/15, gravities 1/240 / 1/120 / 1/240, the speeds 0.5 / 0.5, 0.4, its bytes 3 /
//!   6 / 3 / 6 / 13 / 26), the health block's `+0x28` 3, `+0xa8` 3; update and draw distances 200; movement (radius
//!   0.3, height 0.23, steps 1, slope π, gravity 1/180, the turn before moving 15°, stop 0.25, no ledge test, no fence,
//!   its aim records); the limits 1/300 / 0.1 / 0.2°/tick / 12°; its home; its place in waypoint 0's frame; the network
//!   built (its openings kept); hidden without collision, resting (1, sub 0, sequence 0); the taken count 0.
//! * **Each tick**: drawn within 24 of the camera its shadow (range 0x16); the damage; outside 1 and 8 its draw
//!   ([`draw`] `0x3e1f40`: at joint 2 two glows of 0.25, FX 0x2e `0x8080ffff` and FX 0xc `0x8020f020`). Sub 3, the
//!   hurt's flight: dead — the kills `$gp+0x19b0` counted (the help 0xa2d / 0xa2e / 0xa2f / 0xa30 in slots 0x84..0x87
//!   at 50 / 100 / 150 / 200), the taken count `$gp+0x19b4` up, a smoke burst, its bolts, its health back, resting
//!   hidden; over: 4 (sub 1).
//! * **States**: 1 resting, hidden without collision. 2 out of the carrier ([`launch`] `0x3e1c30`): 0.1 a tick to its
//!   landing; there 3 (sequence 4); the taken count 0 meanwhile. 3 to its waypoint (the waypoint's cuboid frame applied
//!   to its place, through the network with its margin, [`fly`]): at its pick-up waypoint +1 at 0.05, given to a walker
//!   in whose area it is ([`give`] `0x3e2f00`: 7 sub 2, sequence 5); stuck over 60 a swerve of 90°..135° for 60 ticks;
//!   at it 4 (sequence 1), its wait 80..180 at the pick-up waypoint, else 10..160; below z 50 resting; stuck over 200
//!   resting (a smoke burst, its bolts, its health back); its margin 0.4 within 4 of the goal; asked on, its wait at
//!   most 20. 4 at a waypoint: asked on and the wait out — past waypoint 5 back to the carrier (the carrier in state 5
//!   gives the landing and the way in, `0x41b1d0`; the first back with nothing taken earns skill point 0x0a), state 5
//!   (sequence 4); else on to the asked one (3, sequence 4, margin 0.5..0.7). Meanwhile it looks about (each 60..120
//!   ticks: at waypoints 1..4 half the time their look-at cuboid, else the guide shown 0.5 up, else a random way up to
//!   0.5 up), turning to it in place, both aim records pitched half of it. 5 back to the landing as 3 (no pick-up) to
//!   the door cuboid: 6 (sequence 3). 6 in at 0.1 a tick: aboard resting (sequence 4, a new pieces mask). 7 being
//!   given: the sequence's end 3 (sub 1, sequence 7). 8 taken by a walker: from its fifth tick resting (a smoke burst),
//!   pulled in as 6.
//! * **The tail** (`0x3e1b88`): the flash, both aims sprung (0.03 / 0.3) on joint lists 1 and 0, its ticks counted.
//!
//! **The carrier 0x0fa5.** Pvars: its three paths `+0x50` / `+0x54` / `+0x58`, the critters' group `+0x5c`, the guide
//! `+0x60`, its place and angles on the path `+0x70` / `+0x80`, the first leg's length `+0x90`, three wobble phases
//! `+0x94..`, the progress and its speed `+0xa0` / `+0xa4`, the wait `+0xa8`, its sound `+0xac` and the sound's wait
//! `+0xb0` (the index its slot plays, the slot's `+0x7e`, kept at `+0xb4`). With no paths, group or guide (path 1 not
//! 0) it is a breakable (7: a hit with 0x10000 not from 0x0b69 / 0x0f9e blows it up — flashes 4 and 2 past 9, light
//! 15, 10 embers, 3 billows, 9 flares, the shake, one debris piece — its bolts, gone). Else: 1 waits for the guide at
//! rest; 2 down path 0 (eased 1/72 / 1/720 to 1/3 a tick, the exhaust, its pitch −0.333 of the path's, its loop sound
//! 0): at the end 3 (sequence 1, wait 60, sound 1). 3: the wait out, a critter let out at its door (`(3.25, −2, 0)`
//! in its frame) toward the ground below `(3.25, −3.5, 0)` + 2, then 45 more; none left: the guide out ([`pad`]
//! `0x41ba60`), 4 (sequence 0, sound 2). 4 along path 1 (1/720 / 1/720 to 1/3; sound 2 while its wait runs): 5. 5:
//! the guide at its path's end (and sequence 0 shown) sequence 1, sound 1; every critter resting: 6 (sequence 0, the
//! loop sound 2); with sequence 1, sound 1 kept after its wait. 6 up path 2 (2/3600 to 1/6, loop sound 0): 1. Its
//! first tick in each state stops its sound. Out of 7 it wobbles on its path: x by 1° (54°/s), y by 1° (87°/s), z by
//! 0.1 (76°/s). **The exhaust** (`0x41a188`): at `(−4, −1, 1)` and `(−4, 1.5, 1)` in its frame three puffs each
//! (0.1, ×1..0.9, 210000, behind it at 0.05 / 0.05 / 1/60 a tick and 1/30 up; `0xdf202020` / `0xdf208080` /
//! `0xdf808080`, 10 ticks).
//!
//! **The guide 0x0fec.** Pvars: its path `+0x110`, the critters' group `+0x114`, its next line `+0x11c`, movement
//! `+0x120`, its home and yaw `+0x210` / `+0x21c`, its stop `+0x220` and the point it walks to `+0x224`, the yaw it
//! faces `+0x228`, the waits `+0x22c` / `+0x25c`, its ten lines `+0x230` (+ 2000), the last talk sequence `+0x258`.
//! On level 13 only the scenes show it. The set-up (`0x41b720`): update and draw distances 200, its home, hidden
//! without collision (1), sequence 0, movement (radius 0.4, height and steps 0.5, slope π, gravity 1/180, turn 15°,
//! stop 0.25, no fence), the limits 0.0089 / 32/60 / 0.2°/tick / 12°. Each tick its scale 1.333 of its class's and the
//! shadow within 24. **States**: 1 hidden. 2 (out by the teleport beam, [`pad`]): the sequence's end and a line
//! free — the line said ([`say`] `0x41bb38`), 8 and a talk sequence 1..4 (not the last). 8 facing `+0x228` till the
//! line waits for its go: let go, 5. 5: the line over — the critters asked on (`0x41bb98`), 6 (sequence 0), the next
//! stop (`0x41b9e0`: the next point with w 0, else the last); else a new talk sequence at each end. 6 along its path
//! (`0x41b8c8`): at the stop — the last 3 (sequence 9), else 7 (sequence 0, 120 ticks, turned about). 7 facing it, the
//! wait out and a line free: 8 (a talk sequence, the next line). 3: the end, its stop and line reset, 10. 10: hidden
//! without collision for 25 ticks with the beam's burst, then home (1).

use crate::hero::physics::{add, add_rot, atan2, dist, ease, len, len2, set_len, sub, through, V3};
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::{ai, mode, Ctx, MobyWorld};
use crate::path_network;

pub const CRITTER: i16 = 0x0bb8;
pub const CARRIER: i16 = 0x0fa5;
pub const GUIDE: i16 = 0x0fec;
const WALKER: i16 = 0x0fb4;

// The critter's pvars.
const C_HEALTH: usize = 0x20;
const C_FLASH: usize = 0x50;
const C_NODES: usize = 0x110;
const C_START: usize = 0x510;
const C_CARRIER: usize = 0x514;
const C_POINTS: usize = 0x518;
const C_LOOKS: usize = 0x530;
const C_PICK: usize = 0x540;
const C_WALKERS: usize = 0x544;
const C_TARGET: usize = 0x548;
const C_DOOR: usize = 0x54c;
const C_MV: usize = 0x550;
const C_AIM0: usize = 0x640;
const C_AIM1: usize = 0x6c0;
const C_LAND: usize = 0x740;
const C_IN: usize = 0x750;
const C_HOME: usize = 0x760;
const C_LOCAL: usize = 0x770;
const C_LOOK: usize = 0x780;
const C_COUNT: usize = 0x790;
const C_AT: usize = 0x794;
const C_ASKED: usize = 0x798;
const C_NODE_AT: usize = 0x79c;
const C_NODE_GOAL: usize = 0x7a0;
const C_MARGIN: usize = 0x7a4;
const C_WAIT: usize = 0x7a8;
const C_SPEED: usize = 0x7ac;
const C_SWERVE_T: usize = 0x7b0;
const C_SWERVE: usize = 0x7b4;
const C_TAKER: usize = 0x7b8;
const C_LOOK_T: usize = 0x7bc;
const NODES_MAX: usize = 32;

// The carrier's pvars.
const K_PATHS: usize = 0x50;
const K_GROUP: usize = 0x5c;
const K_GUIDE: usize = 0x60;
const K_POS: usize = 0x70;
const K_ROT: usize = 0x80;
const K_LEN: usize = 0x90;
const K_PHASE: usize = 0x94;
const K_T: usize = 0xa0;
const K_V: usize = 0xa4;
const K_WAIT: usize = 0xa8;
const K_SOUND: usize = 0xac;
const K_SOUND_T: usize = 0xb0;
/// The port's: the index the carrier's sound slot plays (the slot's `+0x7e`).
const K_KIND: usize = 0xb4;

// The guide's pvars.
const G_PATH: usize = 0x110;
const G_GROUP: usize = 0x114;
const G_LINE: usize = 0x11c;
const G_MV: usize = 0x120;
const G_HOME: usize = 0x210;
const G_YAW: usize = 0x21c;
const G_STOP: usize = 0x220;
const G_POINT: usize = 0x224;
const G_FACE: usize = 0x228;
const G_WAIT: usize = 0x22c;
const G_LINES: usize = 0x230;
const G_TALK: usize = 0x258;
const G_GONE: usize = 0x25c;

/// `$gp+0x2bc0` / `+0x2bd0`: the carrier's door and the ground before it, in its frame.
const DOOR: V3 = [3.25, -2.0, 0.0];
const STEP: V3 = [3.25, -3.5, 0.0];
/// `$gp+0x2c10` / `+0x2c20`: the exhausts.
const EXHAUSTS: [V3; 2] = [[-4.0, -1.0, 1.0], [-4.0, 1.5, 1.0]];
/// `$gp+0x2be4..+0x2c00`: the wobble (z 0.1 at 76°/s; y 1° at 87°/s; x 1° at 54°/s), path 2's ease (2 and 10).
const DEG: f32 = f32::from_bits(0x3c8e_fa36);
const PER_TICK: f32 = f32::from_bits(0x3998_825c);

fn st(w: &mut MobyWorld, id: usize, state: u8, sub: i32) { w.mobys[id].set_state(state, sub); }

/// `+0x98` 0 and mode `| 0x41 & !0x1000`: hidden, no collision, no target.
fn rest(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.mode = (m.mode | 0x41) & !mode::TARGET;
}

/// Mode `& 0xffbe | 0x1000` and its class's collision: shown, a target.
fn show(w: &mut MobyWorld, id: usize) {
    let coll = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.mode = (m.mode & 0xffbe) | mode::TARGET;
    m.has_collision = coll;
}

fn moby_at(w: &MobyWorld, i: i32) -> Option<usize> { usize::try_from(i).ok().filter(|&k| k < w.mobys.len()) }

fn members(w: &MobyWorld, g: i32) -> Vec<usize> { usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default() }

fn rows4(m: &[[f32; 4]]) -> [V3; 3] { std::array::from_fn(|k| [m[k][0], m[k][1], m[k][2]]) }

/// Cuboid `i`'s place (`+0x30`).
fn cuboid_pos(ctx: &Ctx, i: i32) -> V3 { ctx.level.cuboid_place(i).map_or([0.0; 3], |p| p.0) }

/// The path's points as the level has them now (its network openings applied).
fn path(ctx: &Ctx, i: i32) -> Vec<[f32; 4]> { path_network::points_now(ctx.level, &ctx.game.path_openings, i).map(|p| p.into_owned()).unwrap_or_default() }

// ---- the critter ----

/// `0x3e1890` (module docs): the set-up on state 0, then the shadow, the damage and the draw.
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let r = ctx.game.rng.randi(6);
        let m = &mut w.mobys[id];
        m.bangles = 1 << r;
        for (k, v) in [(0x77, 10u8), (0xd3, 0x1a), (0x28, 3), (0xcc, 3), (0xce, 3), (0x72, 8), (0x76, 9), (0xcf, 6), (0xd2, 0xd), (0x70, 9), (0x71, 8), (0xcd, 6)] { m.set_pv_b(k, v); }
        for (k, v) in [(0x7c, 0x3dcc_ccce), (0x80, 0x3d88_8889), (0x84, 0x3b88_8889), (0x88, 0x3c08_8889), (0x8c, 0x3b88_8889), (0x90, 0x3f00_0000), (0x94, 0x3f00_0000), (0x9c, 0x3ecc_cccd)] { m.set_pv_f(k, f32::from_bits(v)); }
        m.set_pv_i(0xa8, 3);
        m.update_dist = 200;
        m.draw_dist = 200;
        let o = C_MV;
        m.set_pv_f(o + 0x34, 0.25);
        // `$gp+0x19b8`: 0.23.
        m.set_pv_f(o + 4, f32::from_bits(0x3e6b_851f));
        m.set_pv_f(o, f32::from_bits(0x3e99_999a));
        m.set_pv_f(o + 0xc, 1.0);
        m.set_pv_f(o + 0x30, f32::from_bits(0x3e86_0a93));
        m.set_pv_f(o + 0x2c, f32::from_bits(0x3bb6_0b61));
        m.set_pv_i(o + 0x3c, -1);
        m.set_pv_f(o + 8, 1.0);
        m.set_pv_f(o + 0x10, f32::from_bits(0x4049_0fd0));
        m.set_pv_i(o + 0x38, m.pv_i(o + 0x38) | 1);
        let [a, b, c, d] = [0x3b5a_740e, 0x3dcc_ccce, 0x3b64_c38a, 0x3e56_7752].map(f32::from_bits);
        ai::senses(m, a, b, c, d);
        let p = m.pos;
        m.set_pv_v(C_HOME, p);
        let first = m.pv_i(C_POINTS);
        if let Some(c) = usize::try_from(first).ok().and_then(|i| ctx.level.cuboids.get(i)) {
            let d = sub(p, [c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]]);
            let local = through(&rows4(&c.inverse), d);
            w.mobys[id].set_pv_v(C_LOCAL, local);
        }
        let m = &mut w.mobys[id];
        m.set_pv_i(o + 0x4c, C_AIM0 as i32);
        m.set_pv_i(o + 0x48, C_AIM1 as i32);
        let start = m.pv_i(C_START);
        let mut nodes = path_network::read_nodes(&m.pvars, C_NODES, NODES_MAX);
        let n = path_network::build(ctx.level, start, &mut nodes);
        path_network::write_nodes(&mut w.mobys[id].pvars, C_NODES, &nodes[..n]);
        path_network::open(&mut ctx.game.path_openings, ctx.level, &nodes[..n]);
        w.mobys[id].set_pv_i(C_COUNT, n as i32);
        rest(w, id);
        st(w, id, 1, 0);
        if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0); }
        w.level12.taken = 0;
    }
    if w.mobys[id].drawn != 0 { crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16); }
    ai::take_damage(w, id, ctx, false);
    let s = w.mobys[id].state;
    if s != 8 && s != 1 { w.register_draw(id); }
}

/// Its health back (`+0x20` = the limit `+0x24`).
fn heal(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let full = m.pv_s(0x24) as f32;
    m.set_pv_f(C_HEALTH, full);
}

/// A smoke burst at it and its bolts.
fn poof(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let at = w.mobys[id].pos;
    crate::moby::blast::smoke_burst(w, ctx, 1.0, 0.0, Some(id), at, -1);
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
}

/// `0x32e3d0(0, margin, pos, to, &yaw, nodes, n, &at, &goal)`: the heading through its network.
fn heading(w: &mut MobyWorld, id: usize, ctx: &Ctx, to: V3) -> f32 {
    let m = &w.mobys[id];
    let n = (m.pv_i(C_COUNT).max(0) as usize).min(NODES_MAX);
    let nodes = path_network::read_nodes(&m.pvars, C_NODES, n);
    let (mut yaw, mut at, mut goal) = (0.0, m.pv_i(C_NODE_AT), m.pv_i(C_NODE_GOAL));
    path_network::steer(ctx.level, &nodes, m.pv_f(C_MARGIN), m.pos, to, &mut yaw, &mut at, &mut goal);
    let m = &mut w.mobys[id];
    m.set_pv_i(C_NODE_AT, at);
    m.set_pv_i(C_NODE_GOAL, goal);
    yaw
}

/// The swerve: while `+0x7b0` runs the heading turned by `+0x7b4`; then, stuck over 60 ticks, a new one of 90°..135°
/// for 60.
fn swerve(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, yaw: f32) -> f32 {
    let m = &mut w.mobys[id];
    let mut t = m.pv_i(C_SWERVE_T);
    let done = crate::moby::countdown(&mut t);
    m.set_pv_i(C_SWERVE_T, t);
    let yaw = if done { yaw } else { add_rot(yaw, m.pv_f(C_SWERVE)) };
    if m.pv_i(C_SWERVE_T) == 0 && 60 < m.pv_i(C_MV + 0x80) {
        m.set_pv_i(C_SWERVE_T, 60);
        let a = ctx.game.rng.randf(90.0, 135.0) * DEG;
        w.mobys[id].set_pv_f(C_SWERVE, a);
    }
    yaw
}

/// The step toward `to`, at most 0.1 long; true when it reaches it.
fn step_to(w: &MobyWorld, id: usize, to: V3) -> (V3, bool) {
    let v = sub(to, w.mobys[id].pos);
    if 0.1 < len(v) { (set_len(v, f32::from_bits(0x3dcc_ccce)), false) } else { (v, true) }
}

/// 0x0bb8 (module docs).
pub fn update_0bb8(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    setup(w, id, ctx);
    if w.mobys[id].sub_state == 3 {
        let hit = ai::flight(w, id, ctx);
        if hit & 1 != 0 {
            w.level12.kills += 1;
            match w.level12.kills {
                50 => { ctx.game.help(0xa2d, 0x84); }
                100 => { ctx.game.help(0xa2e, 0x85); }
                150 => { ctx.game.help(0xa2f, 0x86); }
                200 => { ctx.game.help(0xa30, 0x87); }
                _ => {}
            }
            st(w, id, 1, 0);
            set_seq(w, id, ctx, 4, 0, 0x14);
            w.level12.taken += 1;
            poof(w, id, ctx);
            w.mobys[id].set_pv_s(0x98, 0);
            heal(w, id);
            rest(w, id);
        } else if hit & 2 != 0 {
            st(w, id, 4, 1);
        }
    }
    states(w, id, ctx);
    let m = &mut w.mobys[id];
    ai::flash(m, C_FLASH);
    let k = f32::from_bits(0x3cf5_c28f);
    let d = f32::from_bits(0x3e99_999a);
    crate::moby::joint_aim::aim(w, id, C_AIM1, id, 1, k, d);
    crate::moby::joint_aim::aim(w, id, C_AIM0, id, 0, k, d);
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks += 1; }
}

fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        2 => {
            w.level12.taken = 0;
            w.mobys[id].set_pv_i(C_TAKER, 0);
            let to = w.mobys[id].pv_v(C_LAND);
            let (v, there) = step_to(w, id, to);
            if there {
                st(w, id, 3, -1);
                set_seq(w, id, ctx, 4, 0, 0x14);
            }
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, v);
        }
        3 => fly(w, id, ctx),
        5 => {
            let to = w.mobys[id].pv_v(C_LAND);
            let yaw = heading(w, id, ctx, to);
            let yaw = {
                let m = &mut w.mobys[id];
                let mut t = m.pv_i(C_SWERVE_T);
                let done = crate::moby::countdown(&mut t);
                m.set_pv_i(C_SWERVE_T, t);
                if done { yaw } else { add_rot(yaw, m.pv_f(C_SWERVE)) }
            };
            let speed = w.mobys[id].pv_f(C_SPEED);
            ai::move_with(w, id, ctx, yaw, to, 1, speed, -1.0);
            if w.mobys[id].pos[2] < 50.0 { st(w, id, 1, 0); }
            let m = &w.mobys[id];
            if m.pv_i(C_SWERVE_T) == 0 && 60 < m.pv_i(C_MV + 0x80) {
                w.mobys[id].set_pv_i(C_SWERVE_T, 60);
                let a = ctx.game.rng.randf(90.0, 135.0) * DEG;
                w.mobys[id].set_pv_f(C_SWERVE, a);
            }
            let m = &w.mobys[id];
            if ctx.level.in_cuboid(m.pv_i(C_DOOR), m.pos) {
                st(w, id, 6, -1);
                set_seq(w, id, ctx, 3, 0, 0x14);
            }
        }
        4 => look(w, id, ctx),
        8 | 6 => {
            if w.mobys[id].state == 8 && 5 <= w.mobys[id].state_ticks as u16 {
                rest(w, id);
                let at = w.mobys[id].pos;
                crate::moby::blast::smoke_burst(w, ctx, 1.0, 0.0, Some(id), at, -1);
                st(w, id, 1, 0);
            }
            let to = w.mobys[id].pv_v(C_IN);
            let (v, there) = step_to(w, id, to);
            if there {
                st(w, id, 1, 0);
                set_seq(w, id, ctx, 4, 0, 0x14);
                let r = ctx.game.rng.randi(6);
                w.mobys[id].bangles = 1 << r;
                rest(w, id);
            }
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, v);
        }
        7 => {
            if wrapped(w, id, ctx) {
                st(w, id, 3, 1);
                set_seq(w, id, ctx, 7, 0, 10);
            }
        }
        1 => rest(w, id),
        _ => {}
    }
}

/// State 3 (module docs): to its waypoint.
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (k, local) = { let m = &w.mobys[id]; (m.pv_i(C_AT), m.pv_v(C_LOCAL)) };
    let c = w.mobys[id].pv_i(C_POINTS + 4 * k.clamp(0, 5) as usize);
    let to = usize::try_from(c).ok().and_then(|i| ctx.level.cuboids.get(i)).map_or(w.mobys[id].pos, |c| add(through(&rows4(&c.matrix), local), [c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]]));
    let yaw = heading(w, id, ctx, to);
    let pick = w.mobys[id].pv_i(C_PICK);
    let speed = if pick != 0 && pick == k - 1 {
        if ai::link(&w.mobys[id], C_TAKER).is_none() {
            let g = w.mobys[id].pv_i(C_WALKERS);
            let taker = give(w, ctx, g, id);
            ai::set_link(&mut w.mobys[id], C_TAKER, taker);
            if taker.is_some() {
                st(w, id, 7, 2);
                set_seq(w, id, ctx, 5, 0, 10);
            }
        }
        f32::from_bits(0x3d4c_ccce)
    } else {
        w.mobys[id].pv_f(C_SPEED)
    };
    let yaw = swerve(w, id, ctx, yaw);
    if ai::move_with(w, id, ctx, yaw, to, 1, speed, -1.0) & 0x80 != 0 {
        st(w, id, 4, -1);
        set_seq(w, id, ctx, 1, 0, 0x14);
        let (pick, k) = (w.mobys[id].pv_i(C_PICK), w.mobys[id].pv_i(C_AT));
        let t = if pick != 0 && pick == k { ctx.game.rng.rand_range(80, 180) } else { ctx.game.rng.rand_range(10, 160) };
        w.mobys[id].set_pv_i(C_WAIT, t);
    }
    if w.mobys[id].pos[2] < 50.0 { st(w, id, 1, 0); }
    if 200 < w.mobys[id].pv_i(C_MV + 0x80) {
        st(w, id, 1, 0);
        set_seq(w, id, ctx, 4, 0, 0x14);
        poof(w, id, ctx);
        heal(w, id);
        rest(w, id);
    } else {
        let m = &mut w.mobys[id];
        if 0.5 <= m.pv_f(C_MARGIN) && m.pv_f(C_MV + 0xcc) < 4.0 { m.set_pv_f(C_MARGIN, f32::from_bits(0x3ecc_cccd)); }
    }
    let m = &mut w.mobys[id];
    if m.pv_i(C_AT) != m.pv_i(C_ASKED) && 20 < m.pv_i(C_WAIT) { m.set_pv_i(C_WAIT, 20); }
}

/// State 4 (module docs): at a waypoint, looking about till asked on.
fn look(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (at, asked) = (w.mobys[id].pv_i(C_AT), w.mobys[id].pv_i(C_ASKED));
    if at != asked {
        let m = &mut w.mobys[id];
        let mut t = m.pv_i(C_WAIT);
        let done = crate::moby::countdown(&mut t);
        m.set_pv_i(C_WAIT, t);
        if done {
            if 6 <= asked {
                if w.level12.taken == 0 { ctx.game.award_skill_point(0x0a); }
                let carrier = moby_at(w, w.mobys[id].pv_i(C_CARRIER));
                if let Some((land, way_in)) = carrier.and_then(|c| carrier_door(w, c, ctx)) {
                    let m = &mut w.mobys[id];
                    m.set_pv_v(C_LAND, land);
                    m.set_pv_v(C_IN, way_in);
                }
                st(w, id, 5, -1);
                set_seq(w, id, ctx, 4, 0, 0x14);
            } else {
                w.mobys[id].set_pv_i(C_AT, asked);
                st(w, id, 3, -1);
                set_seq(w, id, ctx, 4, 0, 0x14);
                let r = ctx.game.rng.randf(0.5, f32::from_bits(0x3f33_3333));
                w.mobys[id].set_pv_f(C_MARGIN, r);
            }
            return;
        }
    }
    let early = (w.mobys[id].state_ticks as u16) < 2;
    let pick_new = early || {
        let m = &mut w.mobys[id];
        let mut t = m.pv_i(C_LOOK_T);
        let done = crate::moby::countdown(&mut t);
        m.set_pv_i(C_LOOK_T, t);
        done
    };
    if pick_new {
        let k = w.mobys[id].pv_i(C_AT);
        let cub = (1..=4).contains(&k) && (early || ctx.game.rng.randi(2) != 0);
        if cub {
            let c = w.mobys[id].pv_i(C_LOOKS + 4 * (k - 1) as usize);
            let p = cuboid_pos(ctx, c);
            w.mobys[id].set_pv_v(C_LOOK, p);
        } else {
            let t = moby_at(w, w.mobys[id].pv_i(C_TARGET));
            match t.filter(|&t| w.mobys[t].mode & 1 == 0) {
                Some(t) => {
                    let p = w.mobys[t].pos;
                    w.mobys[id].set_pv_v(C_LOOK, [p[0], p[1], p[2] + 0.5]);
                }
                None => {
                    let a = ctx.game.rng.rand_angle();
                    let z = ctx.game.rng.randf(0.0, 0.5);
                    let p = w.mobys[id].pos;
                    w.mobys[id].set_pv_v(C_LOOK, add([a.cos(), a.sin(), z], p));
                }
            }
        }
        let t = ctx.game.rng.rand_range(60, 120);
        w.mobys[id].set_pv_i(C_LOOK_T, t);
    }
    let (p, q) = (w.mobys[id].pv_v(C_LOOK), w.mobys[id].pos);
    let v = sub(p, q);
    let yaw = atan2(v[0], v[1]);
    let pitch = -atan2(len2(v), v[2]);
    ai::move_step(w, id, ctx, yaw, q);
    let m = &mut w.mobys[id];
    m.set_pv_f(C_AIM0 + 0x64, pitch * 0.5);
    m.set_pv_f(C_AIM1 + 0x64, pitch * 0.5);
}

/// `0x3e2f00(group, critter)`: the first walker 0x0fb4 of the group whose area path (`+0x138`) holds the critter takes
/// it (its `+0x374`): one raised (9) is shown with its collision, sequence 5 (blend 10), 10; any other live one 11
/// (sub 4), sequence 7 (blend 20).
fn give(w: &mut MobyWorld, ctx: &mut Ctx, g: i32, critter: usize) -> Option<usize> {
    let at = w.mobys[critter].pos;
    for k in members(w, g) {
        let Some(m) = w.mobys.get(k) else { continue };
        if m.o_class != WALKER || !ctx.level.in_polygon(m.pv_i(0x138), at) { continue; }
        if m.state == 9 {
            show(w, k);
            set_seq(w, k, ctx, 5, 0, 10);
            st(w, k, 10, -1);
        } else if m.state != 0xfe && m.state != 0xfd {
            st(w, k, 0xb, 4);
            set_seq(w, k, ctx, 7, 0, 0x14);
        }
        ai::set_link(&mut w.mobys[k], 0x374, Some(critter));
        return Some(k);
    }
    None
}

/// `0x3e1c30(group, at, to)`: the first resting critter of the group let out at `at` toward `to` — shown, a target,
/// its angles to it, sequence 2 (blend 2), state 2 (the byte alone), its speed `randf(5.5, 6.5)/60`, waypoints 0,
/// rebuilt. Whether one was.
fn launch(w: &mut MobyWorld, ctx: &mut Ctx, g: i32, at: V3, to: V3) -> bool {
    for k in members(w, g) {
        if w.mobys.get(k).is_none_or(|m| m.o_class != CRITTER || m.state != 1) { continue; }
        show(w, k);
        let m = &mut w.mobys[k];
        m.pos = at;
        m.set_pv_v(C_LAND, to);
        m.rot = [0.0, 0.0, atan2(to[0] - at[0], to[1] - at[1])];
        ctx.host.set_anim(&w.mobys[k], 2, 0, 2);
        let s = ctx.game.rng.randf(5.5, 6.5) * f32::from_bits(0x3c88_8889);
        let m = &mut w.mobys[k];
        m.state = 2;
        m.set_pv_f(C_SPEED, s);
        m.set_pv_i(C_ASKED, 0);
        m.set_pv_i(C_AT, 0);
        w.rebuild(k, &*ctx.host);
        return true;
    }
    false
}

/// `0x3e1da0(group)`: every critter of the group resting.
fn all_rest(w: &MobyWorld, g: i32) -> bool { members(w, g).into_iter().filter_map(|k| w.mobys.get(k)).all(|m| m.o_class != CRITTER || m.state == 1) }

/// `0x41bb98(group)`: the critters of the group asked on to their next waypoint.
fn ask_on(w: &mut MobyWorld, g: i32) {
    for k in members(w, g) {
        if let Some(m) = w.mobys.get_mut(k).filter(|m| m.o_class == CRITTER) { m.set_pv_i(C_ASKED, m.pv_i(C_ASKED) + 1); }
    }
}

/// `0x41b1d0(carrier, &land, &in)`: with the carrier in state 5, the ground below its step (+ 2) and its door.
fn carrier_door(w: &MobyWorld, c: usize, ctx: &Ctx) -> Option<(V3, V3)> {
    let m = &w.mobys[c];
    if m.o_class != CARRIER || m.state != 5 { return None; }
    let way_in = add(through(&m.rows, DOOR), m.pos);
    let mut land = add(through(&m.rows, STEP), m.pos);
    land[2] += 2.0;
    land[2] = ai::ground_z(ctx, 0.5, land, 0);
    Some((land, way_in))
}

/// `0x3e1f40`: the critter's two glows at joint 2.
pub fn draw(w: &MobyWorld, id: usize, host: &dyn crate::moby::MobyHost, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let Some(p) = host.joint(&w.mobys[id], 2) else { return };
    crate::fx::sprite(out, 0.25, 0.0, 0.0, p, cam, 0x2e, 0x8080_ffff, true);
    crate::fx::sprite(out, 0.25, 0.0, 0.0, p, cam, 0xc, 0x8020_f020, true);
}

// ---- the carrier ----

/// Its sound let go (`+0xac` −1).
fn hush(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(K_SOUND);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(K_SOUND, -1);
}

fn live(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool { let h = w.mobys[id].pv_i(K_SOUND); h != -1 && ctx.host.sound_playing(h) }

fn sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, index: i32, mode: i32) {
    let h = ctx.host.play_sound(&w.mobys[id], index, mode);
    let m = &mut w.mobys[id];
    m.set_pv_i(K_SOUND, h);
    m.set_pv_i(K_KIND, index);
}

/// Sound `index` unless it already plays.
fn keep_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, index: i32) {
    if !live(w, id, ctx) || w.mobys[id].pv_i(K_KIND) != index { sound(w, id, ctx, index, 0); }
}

/// The carrier's ride along path `k` eased by `accel` / `brake` to `max`: its pose, the exhaust, its pitch −0.333 of
/// the path's. True at the end.
fn ride(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: usize, accel: f32, brake: f32, max: f32) -> bool {
    let pts = path(ctx, w.mobys[id].pv_i(K_PATHS + 4 * k));
    if pts.len() < 2 { return true; }
    let end = (pts.len() - 1) as f32;
    let m = &mut w.mobys[id];
    let (mut t, mut v) = (m.pv_f(K_T), m.pv_f(K_V));
    ease(end, accel, brake, max, &mut t, &mut v);
    m.set_pv_f(K_T, t);
    m.set_pv_f(K_V, v);
    let (p, pitch, yaw, _) = crate::paths::pose(&pts, t, false);
    m.set_pv_v(K_POS, p);
    m.set_pv_f(K_ROT + 4, pitch);
    m.set_pv_f(K_ROT + 8, yaw);
    exhaust(w, id, ctx);
    let m = &mut w.mobys[id];
    m.set_pv_f(K_ROT + 4, m.pv_f(K_ROT + 4) * f32::from_bits(0xbeaa_7efa));
    end <= m.pv_f(K_T)
}

/// `0x41a188` (module docs).
fn exhaust(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let back = add_rot(m.rot[2], f32::from_bits(0x4049_0fdc));
    let (c, s) = (back.cos(), back.sin());
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    for off in EXHAUSTS {
        let at = add(through(&m.rows, off), m.pos);
        for (k, (rgba, b3)) in [(0xdf20_2020u32, 0x44u8), (0xdf20_8080, 0x48), (0xdf80_8080, 0x44)].into_iter().enumerate() {
            let f = if k == 2 { f32::from_bits(0x3c88_8889) } else { f32::from_bits(0x3d4c_ccce) };
            let v = [c * f, s * f, f32::from_bits(0x3d08_8889)];
            let spin = g.randi(0x10);
            if let Some(i) = crate::particles::puff(sys, g, f32::from_bits(0x3dcc_cccd), 1.0, f32::from_bits(0x3f66_6666), 210_000.0, at, spin, v, rgba, 0) {
                let r = &mut sys.pool.recs[i];
                r[0xa..0xc].copy_from_slice(&10i16.to_le_bytes());
                r[9] = 4 + 0x70;
                r[0x24..0x28].copy_from_slice(&2u32.to_le_bytes());
                r[0x2a] = 0xdf;
                r[0x2b] = r[0xa];
                r[3] = b3;
            }
        }
    }
}

/// 0x0fa5 (module docs).
pub fn update_0fa5(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let first = |w: &mut MobyWorld| w.mobys[id].first();
    let group = w.mobys[id].pv_i(K_GROUP);
    let guide = moby_at(w, w.mobys[id].pv_i(K_GUIDE));
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_i(K_SOUND, -1);
            let p: [i32; 3] = std::array::from_fn(|k| m.pv_i(K_PATHS + 4 * k));
            if p[0] == -1 && p[2] == -1 && p[1] != 0 && group == -1 && m.pv_i(K_GUIDE) == -1 {
                st(w, id, 7, -1);
                return;
            }
            let pts = path(ctx, p[0]);
            let l = if 2 <= pts.len() { dist([pts[0][0], pts[0][1], pts[0][2]], [pts[1][0], pts[1][1], pts[1][2]]) } else { 0.0 };
            let m = &mut w.mobys[id];
            m.set_pv_f(K_LEN, l);
            m.set_state(3, -1);
            m.update_dist = 0xff;
            m.draw_dist = 0xff;
            let (pos, rot) = (m.pos, m.rot);
            m.set_pv_v(K_POS, pos);
            m.set_pv_v(K_ROT, rot);
            for k in 0..3 {
                let a = ctx.game.rng.rand_angle();
                w.mobys[id].set_pv_f(K_PHASE + 4 * k, a);
            }
            st(w, id, 1, -1);
        }
        1 => {
            if first(w) { hush(w, id, ctx); }
            if guide.is_some_and(|g| w.mobys[g].state == 1) { st(w, id, 2, -1); }
        }
        2 => {
            if first(w) { hush(w, id, ctx); }
            let end = ride(w, id, ctx, 0, f32::from_bits(0x3c63_8e39), f32::from_bits(0x3ab6_0b61), f32::from_bits(0x3eaa_aaab));
            if !live(w, id, ctx) { sound(w, id, ctx, 0, 4); }
            if end {
                st(w, id, 3, -1);
                w.mobys[id].set_pv_i(K_WAIT, 60);
                set_seq(w, id, ctx, 1, 0, 0x1e);
                hush(w, id, ctx);
                sound(w, id, ctx, 1, 0);
            }
        }
        3 => {
            if first(w) { hush(w, id, ctx); }
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(K_WAIT);
            let done = crate::moby::countdown(&mut t);
            m.set_pv_i(K_WAIT, t);
            if done {
                let m = &w.mobys[id];
                let at = add(through(&m.rows, DOOR), m.pos);
                let mut to = add(through(&m.rows, STEP), m.pos);
                to[2] += 2.0;
                to[2] = ai::ground_z(ctx, 0.5, to, 0);
                if !launch(w, ctx, group, at, to) {
                    if let Some(g) = guide { pad(w, g, ctx); }
                    st(w, id, 4, -1);
                    set_seq(w, id, ctx, 0, 0, 0x1e);
                    hush(w, id, ctx);
                    sound(w, id, ctx, 2, 0);
                    let m = &mut w.mobys[id];
                    m.set_pv_i(K_SOUND_T, 15);
                    m.set_pv_f(K_T, 0.0);
                    m.set_pv_f(K_V, 0.0);
                } else {
                    w.mobys[id].set_pv_i(K_WAIT, 45);
                }
            } else if 46 <= w.mobys[id].pv_i(K_WAIT) {
                keep_sound(w, id, ctx, 1);
            }
        }
        4 => {
            if first(w) { hush(w, id, ctx); }
            let k = f32::from_bits(0x3ab6_0b61);
            let end = ride(w, id, ctx, 1, k, k, f32::from_bits(0x3eaa_aaab));
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(K_SOUND_T);
            let done = crate::moby::countdown(&mut t);
            m.set_pv_i(K_SOUND_T, t);
            if !done && !live(w, id, ctx) { sound(w, id, ctx, 2, 0); }
            if end {
                st(w, id, 5, -1);
                w.mobys[id].set_pv_f(K_T, 0.0);
            }
        }
        5 => {
            if first(w) { hush(w, id, ctx); }
            let shown = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
            if seq(w, id, ctx) == 0 && guide.is_some_and(|g| at_end(w, g, ctx)) {
                set_seq(w, id, ctx, 1, 0, 0x1e);
                w.mobys[id].set_pv_i(K_SOUND_T, 15);
                hush(w, id, ctx);
                sound(w, id, ctx, 1, 0);
            } else if all_rest(w, group) {
                st(w, id, 6, -1);
                set_seq(w, id, ctx, 0, 0, 0x1e);
                hush(w, id, ctx);
                if !live(w, id, ctx) { sound(w, id, ctx, 2, 4); }
            } else if shown == 1 {
                let m = &mut w.mobys[id];
                let mut t = m.pv_i(K_SOUND_T);
                let done = crate::moby::countdown(&mut t);
                m.set_pv_i(K_SOUND_T, t);
                if !done { keep_sound(w, id, ctx, 1); }
            }
        }
        6 => {
            if first(w) { hush(w, id, ctx); }
            // `$gp+0x2bfc` 2 / 3600 and `$gp+0x2c00` 10 / 60.
            let k = 2.0 * f32::from_bits(0x3991_a2b4);
            let end = ride(w, id, ctx, 2, k, k, 10.0 * f32::from_bits(0x3c88_8889));
            if !live(w, id, ctx) { sound(w, id, ctx, 0, 4); }
            if end {
                st(w, id, 1, -1);
                w.mobys[id].set_pv_f(K_T, 0.0);
            }
        }
        7 => {
            if first(w) { hush(w, id, ctx); }
            let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0);
            if let Some(h) = hit {
                let spared = h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == 0x0b69 || s.o_class == 0x0f9e);
                if !spared {
                    let e = crate::moby::blast::Explosion { flash: 4.0, flash2: 2.0, near: 9.0, size: 0.0, light: 15.0, embers: 10, billows: 3, flares: 9, sound: -1, shake: true, debris: 1, lod: -1, flags: 0x81_0001, ..Default::default() };
                    crate::moby::blast::explode(w, ctx, Some(id), None, &e);
                    crate::moby::bolts::drop(w, id, 0, 3, ctx);
                    w.delete(id);
                    return;
                }
            }
        }
        _ => {}
    }
    if w.mobys[id].state == 7 { return; }
    let m = &mut w.mobys[id];
    m.pos = m.pv_v(K_POS);
    let base = m.pv_v(K_ROT);
    let mut rot = base;
    let ph = |m: &mut crate::moby::Moby, k: usize, rate: f32| {
        let p = add_rot(m.pv_f(K_PHASE + 4 * k), rate * PER_TICK);
        m.set_pv_f(K_PHASE + 4 * k, p);
        p.sin()
    };
    rot[0] = add_rot(1.0 * (ph(m, 0, 54.0) * DEG), base[0]);
    rot[1] = add_rot(1.0 * (ph(m, 1, 87.0) * DEG), base[1]);
    m.rot = rot;
    let z = ph(m, 2, 76.0);
    m.pos[2] += 0.1 * z;
}

// ---- the guide ----

/// `0x41bb00(guide)`: at its path's last point.
fn at_end(w: &MobyWorld, g: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[g];
    let n = path(ctx, m.pv_i(G_PATH)).len() as i32;
    m.pv_i(G_STOP) == n - 1
}

/// `0x41ba60(guide)`: out at its home (its yaw), its walk from point 0, shown with its collision, in by the teleport
/// beam, 2 with sequence 8 (blend 1).
fn pad(w: &mut MobyWorld, g: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[g];
    m.pos = m.pv_v(G_HOME);
    m.rot[2] = m.pv_f(G_YAW);
    m.set_pv_i(G_POINT, 0);
    show(w, g);
    let (p, r) = (w.mobys[g].pos, w.mobys[g].rot);
    crate::moby::classes::teleport::start(w, g, p, r, &[]);
    st(w, g, 2, -1);
    set_seq(w, g, ctx, 8, 0, 1);
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

/// 0x0fec (module docs; levels 12 and 13).
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
            let (yaw, p) = (w.mobys[id].pv_f(G_FACE), w.mobys[id].pos);
            ai::move_step(w, id, ctx, yaw, p);
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
            let (yaw, p) = (w.mobys[id].pv_f(G_FACE), w.mobys[id].pos);
            ai::move_step(w, id, ctx, yaw, p);
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
