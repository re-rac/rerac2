//! The Thugs-4-Less trooper **0x0ea3** (L13 `0x408480`, 43 placed; L16 `0x3f6f50`, 34; the decomp's v2.00
//! `func_US2_L13_00408480`, its helpers v1.01 `func_L13_00409008` / `0040A4D0` / `0040A540` / `00406E30` /
//! `0040DC20` at our L13 `0x40a570` / `0x40ba38` / `0x40baa8` / `0x408398` / `0x40f0a0`, L16 `0x3f9040` / `0x3fa508`
//! / `0x3fa578` / `0x3f6e68` / `0x3fdb70`) and its shot **0x0ea7** (L13 `0x40edb0`, L16 `0x3fd880`, L14 `0x415f20`;
//! made by `0x40f0a0`): a soldier with a blaster on the framework (crate::moby::ai) that waits at its post, playing
//! rock-paper-scissors with a partner, runs to cover or along paths when it sees Ratchet, then fires bursts at him
//! standing, crouched or kneeling. Its health `+0x20` (most `+0x24`, 1 by default; its damage byte `+0x34`),
//! movement `+0x50` (radius 0.825, height 0.2, steps 0.75, slope 90°, the turn 30°, stop 0.5), reaction `+0x140` (a
//! hurt to state 26, sub 4; radius and centre 0.5, bounce 0.15), target block `+0x1f0` (its moby `+0x200`).
//!
//! **Its pvars:** `+0x230` / `+0x234` the main searches' areas with ranges `+0x238` / `+0x23c` (found `+0x250` /
//! `+0x254`); `+0x260` / `+0x274` / `+0x2b4` three more areas (found `+0x264` / `+0x278` / `+0x2b8`); `+0x240` /
//! `+0x244` timers counted down (read by nothing here), `+0x248` the wait before it answers a sighting, `+0x24c`
//! alarmed (by its group: [`alarm`]); `+0x25c` a cuboid it runs to (−1 none), `+0x268` another (state 23), `+0x26c`
//! engaged, `+0x270` a path it takes once (`+0x27c` set when it has), `+0x280` it fires crouched; `+0x284` its
//! shots a burst less a random 0..1 (`+0x288` this burst's, `+0x28c` fired); `+0x290` a path to a stand (state 14),
//! `+0x294` hidden until the moby `+0x298` is gone, then it comes in by the teleport-in carrier along the path
//! `+0x29c`; `+0x2a0` it walks on walls ([`crawl`], module docs); `+0x2a8` it rides the walkway section `+0x2ac` (0x0d65) with its
//! gates `+0x360` / `+0x364` (0x0d95 / 0x0d97) opened at the end; `+0x2b0` a patrol path; `+0x2bc` it follows the
//! drone `+0x2d4` (0x0f35) along the path `+0x2c0`, `+0x2d8` sidestepping at `+0x2dc`° (state 11); `+0x2c4` it plays
//! rock-paper-scissors with `+0x2c8`, its throw `+0x2cc` (0 rock, 1 paper, 2 scissors); `+0x2d0` hidden at first;
//! `+0x2e0` its head aim (crate::moby::joint_aim::head, list 1, 2.5); `+0x368` it goes back home (`+0x370`, its first
//! place) when more than 4 from it; `+0x36c` the reaction's `+0x7c` (−1 none).
//!
//! **Each tick**: the head ([`head`], `0x40a570`): in state 0 the set-up (above; the senses `0x3369b0` = L00
//! `0x32dcf0`, crate::moby::ai::senses: A = 1/450, 1/15, 0.4°, 48°, B = 1/300, 0.1, 0.4°, 48°); its shadow within 24;
//! the frame's damage (`0x330d20` = L00 `0x327e38`, crate::moby::ai::take_damage) alarms its group; the five searches
//! (`0x333d98` = L00 `0x32ad38`, ai::find_target, height 0); under 89 high (with `+0x2a8` under 98) its health 0, its
//! bolts (`0x3250c0` = L00 `0x31adc0`, crate::moby::bolts::drop, 0 / 3) and the dying fade (`0x3ea3f8` = L01
//! `0x3eb1d0`, crate::moby::classes::death_fade). Then the sub-states, the states and the tail (`0x40ba38`: the
//! timers, its ticks, its head aim).
//!
//! **Sub-states** (`+0x95`): **0** calm: found by the first search once `+0x248` is out (with `+0x2a8` only while
//! Ratchet is on a magnetic walkway, `0x18a168`) — to cover ([`take_cover`]); else found by `+0x274`, once, along
//! `+0x270` (16); else alarmed and found, to the attack ([`engage`]), `+0x240` 480..600 when under 300. **8**
//! hidden: with `+0x294` its moby gone, in ([`appear`]) along `+0x29c` (28, sub 6) or to the attack; else found
//! (`+0x248` out) or `+0xbc` = 1, in and to the attack. **1** on guard: found, [`engage`] without the drone. **2**
//! fighting: found by neither (and not running to cover) — wary (9, sub 1), `+0x240` 580..600, a new burst; found by
//! `+0x260` and not engaged, to the cuboid `+0x268` (23, sub 3). **5** waiting for `+0x298` to go (or found by
//! `+0x2b4`): along `+0x29c` or to the attack. **4** hurt (`0x331a68` = L00 `0x328b80`, ai::flight): dead, its bolts
//! and the fade; over, to the attack crouched when found, else wary (sequence 1 or 4), then `+0x290`, `+0x270`,
//! `+0x294` and the drone as at the start. Dying (health out) its 40th tick fires a last shot along its joint 0's
//! second row reversed.
//!
//! **States** (`+0x20`): **5** / **8** a throw: after 56 / 71 ticks, 6 with a random sequence 0x19..0x1b (its
//! throw); **6** after 61 ticks the throws compared ([`rps`], `0x408398`): a draw again (5), lost or won (7, the
//! sequences 7 / 0x1c); **7** after 73 ticks 8. **3** its patrol path, at its end 4 (sequence 1 or 0x16, and 4 one
//! time in three); **4** at the end of its sequence the path back toward its nearer end, 3. **16** along `+0x270`,
//! then engaged (2). **2** to 1. **9** at the end of its sequence 1. **10** along `+0x2c0` (`0x336a18`), at its end
//! 13; the drone gone or Ratchet more than 100° off its facing, to the attack (or 11). **11** moved toward the target
//! turned `+0x2dc`°; after 100 ticks one time in ten to the attack. **28** along `+0x29c`; at its end or within 8 of
//! the target, to the attack. **15** to the cuboid `+0x25c` (the gates opened on its first tick with `+0x2a8`):
//! within 0.5 crouched (20; with `+0x2a8` kneeling, 25). **29** home, the same. **13** the line from joint 0 to the
//! target clear (or meeting Ratchet, a 0x0c87 or the target), to the attack; the drone gone too. **18** at the end of
//! its sequence 20. **14** along `+0x290`; at its end or within 8 of the target, 24. **19** / **21** (after 121
//! ticks) the same line: clear, or meeting Ratchet, the target, 0x0c86, 0x0c87, 0x01f4, 0x10ce, 0x0200 or 0x01ff, to
//! the attack; 19 at the end of its sequence the stand sequences. **20** / **22** / **25** facing the target
//! (`0x336870` = L01 `0x323560`, ai::face), at frame 1 (20) or 3 a shot from joint 0 at 40 a second ([`fire`]); at
//! the end of its sequence with the burst out, 24 (a new burst), else 19 / 21. **23** to the cuboid `+0x268`, then
//! engaged, crouched. **24** at the end of its sequence 21 (19 crouched); with `+0x2a8` and Ratchet more than 6
//! above, kneeling (25); with `+0x368` more than 4 from home, home (29).
//!
//! **The shot 0x0ea7** ([`fire`], [`update_0ea7`]): its look the beam (crate::fx::trails::shot_beam, its draw L13
//! `0x40f2d0` registered each tick it lives through); life 240 ticks (then a 0.6 sphere of its damage, flags
//! 0x10001); each tick the line to its next place (flags 0, its hit, flags 1, on what it meets): a moby not its
//! owner, a 0.3 sphere (its damage, flags 0x10001, along its way ×2); the world, a 0.6 sphere (×3); then gone the
//! next tick in a small blue blast (`0x413bc8`, crate::moby::blast::burst_blue: 0.25, 25 / 20 / 20 sprites, 20
//! puffs). At its firing two twinkles at it (`0x344110` = L00 `0x33a6a0`) and its class sound 0.
//!
//! Engine functions: `0x325bd0` set_state, `0x3258f0` the sequence (crate::moby::classes::props::set_seq),
//! `0x2f9190` = L00 `0x2eeab0` (crate::moby::countdown), `0x336a68` = L00 `0x32dd60` / `0x336a18` the path walk
//! (ai::follow_path), `0x336b70` = L00 `0x32de68` ([`path_set`]), `0x335760` = L01 `0x322450` (ai::move_step toward
//! a point), `0x3357b8` = L00 `0x32cc30` (ai::move_toward), `0x2e93e0` = L00 `0x2de878` (the line,
//! crate::moby::MobyWorld::coll_line), `0x32d948` = L00 `0x323bf8` (ai::reached), `0x2f7258` = L02 `0x2f0170` (the
//! teleport-in, crate::moby::classes::teleport), `0x3337d8` = L00 `0x32a7c8` (crate::moby::Moby::first), `0x2af8f0`
//! (an empty debug print).
//!
//! The heavy trooper 0x0ea4 beside it: [`update_0ea4`].
//!
//! **The wall walk** (`+0x2a0`, the heavy's `+0x2a4`; 4 troopers and 5 heavies placed on L13): along its paths
//! [`crawl`] (`0x40ada8`), to its points the move (`0x40ab60`, crate::moby::frames::move_on_walls, which turns it onto
//! the surface under it: the decomp's v1.01 `0x409b08` / `0x409c40` / `0x409ee8` / `0x40a0a0` / `0x40a2c8` are our
//! `0x40b070` the floor, `0x40b1a8` the walk, `0x40b450` the body, `0x40b608` the ground, `0x40b830` a push-out),
//! within 0.9 of a cover point crouched (the heavy 0.5); firing, after the face the turn toward Ratchet in its own
//! frame ([`crawl_face`], `0x40b9e8` → `0x40b918`).

use crate::hero::physics::{add, add_rot, atan2, dist2, len2, rot_dist, set_len, sub, V3};
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::{ai, blast, countdown, mode, Ctx, Moby, MobyWorld};

pub const CLASS: i16 = 0x0ea3;
/// The heavy trooper ([`update_0ea4`]).
pub const HEAVY: i16 = 0x0ea4;
pub const SHOT: i16 = 0x0ea7;
/// The Snivelak drone (crate::moby::classes::level16::drone; its alarm word `+0x244`).
const DRONE: i16 = 0x0f35;

const HEALTH: usize = 0x20;
const MOST: usize = 0x24;
const DAMAGE: usize = 0x34;
const MV: usize = 0x50;
const REACT: usize = 0x140;
const T_POINT: usize = 0x1f0;
const T_MOBY: usize = 0x200;
const AREA1: usize = 0x230;
const AREA2: usize = 0x234;
const RANGE1: usize = 0x238;
const RANGE2: usize = 0x23c;
const TIMER: usize = 0x240;
const TIMER2: usize = 0x244;
const WAIT: usize = 0x248;
pub(crate) const ALARM: usize = 0x24c;
const FOUND1: usize = 0x250;
const FOUND2: usize = 0x254;
const COVER: usize = 0x25c;
const AREA3: usize = 0x260;
const FOUND3: usize = 0x264;
const SPOT: usize = 0x268;
const ENGAGED: usize = 0x26c;
const ONCE_PATH: usize = 0x270;
const AREA4: usize = 0x274;
const FOUND4: usize = 0x278;
const ONCE_DONE: usize = 0x27c;
const CROUCH: usize = 0x280;
const BURST_BASE: usize = 0x284;
const BURST: usize = 0x288;
const FIRED: usize = 0x28c;
const STAND_PATH: usize = 0x290;
const HIDDEN: usize = 0x294;
const RELEASE: usize = 0x298;
const IN_PATH: usize = 0x29c;
const CRAWLS: usize = 0x2a0;
const RIDER: usize = 0x2a8;
const WALKWAY: usize = 0x2ac;
const PATROL: usize = 0x2b0;
const AREA5: usize = 0x2b4;
const FOUND5: usize = 0x2b8;
const ESCORT: usize = 0x2bc;
const ESCORT_PATH: usize = 0x2c0;
const RPS: usize = 0x2c4;
const RPS_PARTNER: usize = 0x2c8;
pub(crate) const THROW: usize = 0x2cc;
const HIDE: usize = 0x2d0;
const DRONE_ID: usize = 0x2d4;
const SIDESTEP: usize = 0x2d8;
const SIDE_ANGLE: usize = 0x2dc;
const HEAD_AIM: usize = 0x2e0;
const GATE_A: usize = 0x360;
const GATE_B: usize = 0x364;
const HOMING: usize = 0x368;
const REACT_7C: usize = 0x36c;
const HOME: usize = 0x370;
const PVARS: usize = 0x380;

/// The shot's pvars: `+0x00` its velocity, `+0x10` its owner, `+0x14` its damage, `+0x24` its ticks, `+0x28` the
/// beam's scroll.
const S_VEL: usize = 0x00;
const S_OWNER: usize = 0x10;
const S_DAMAGE: usize = 0x14;
const S_LIFE: usize = 0x24;
const S_PVARS: usize = 0x30;

/// The shot's speed a tick (`0x3f2aaaab`: 40 a second).
const SHOT_SPEED: f32 = f32::from_bits(0x3f2a_aaab);
/// The senses A (`0x3b11a2b4`, `0x3d888889`, `0x3be4c38a`, `0x3f567752`) and B (`0x3b5a740e`, `0x3dccccce`, …).
const SENSES_A: [f32; 4] = [f32::from_bits(0x3b11_a2b4), f32::from_bits(0x3d88_8889), f32::from_bits(0x3be4_c38a), f32::from_bits(0x3f56_7752)];
const SENSES_B: [f32; 4] = [f32::from_bits(0x3b5a_740e), f32::from_bits(0x3dcc_ccce), f32::from_bits(0x3be4_c38a), f32::from_bits(0x3f56_7752)];

fn st(w: &mut MobyWorld, id: usize, s: u8, sub: i32) { w.mobys[id].set_state(s, sub); }
fn senses(m: &mut Moby, s: [f32; 4]) { ai::senses(m, s[0], s[1], s[2], s[3]); }
fn found(m: &Moby) -> bool { m.pv_i(FOUND1) == 1 || m.pv_i(FOUND2) == 1 }
/// `D_001B291C + (i << 8)`: a moby by its index, None when gone (states 0xfd / 0xfe) or out of the table.
fn live(w: &MobyWorld, i: i32) -> Option<usize> {
    usize::try_from(i).ok().filter(|&k| w.mobys.get(k).is_some_and(|m| m.state != 0xfe && m.state != 0xfd))
}
fn target_pos(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    ai::link(&w.mobys[id], T_MOBY).and_then(|k| w.mobys.get(k)).map_or(ctx.hero.pos, |t| t.pos)
}
fn cuboid(ctx: &Ctx, i: i32) -> V3 {
    usize::try_from(i).ok().and_then(|i| ctx.level.cuboids.get(i)).map_or([0.0; 3], |c| [c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]])
}
fn path_points(ctx: &Ctx, path: i32) -> usize { usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).map_or(0, Vec::len) }

/// `0x336b70` = L00 `0x32de68(m, path, last, first)`: onto `path` (the movement block's `+0xc4`) at `first`, heading
/// for `last` (−1: its end).
pub fn path_set(m: &mut Moby, ctx: &Ctx, path: i32, last: i16, first: i16) {
    if m.pv_i(0x18) == 0 { return; }
    let o = ai::mv(m);
    m.set_pv_i(o + 0xc4, path + 1);
    m.set_pv_s(o + 0xca, if last == -1 { path_points(ctx, path) as i16 - 1 } else { last });
    m.set_pv_s(o + 0xc8, first);
}

/// To the attack standing (22), crouched (20) with `+0x280`.
fn engage_fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, blend: u8, sub: i32) {
    set_seq(w, id, ctx, 5, 0, blend);
    st(w, id, 0x16, sub);
    if w.mobys[id].pv_i(CROUCH) == 1 {
        set_seq(w, id, ctx, 0x13, 0, 10);
        st(w, id, 0x14, 2);
    }
}

/// The cuboid, the stand path, the walkway and the drone (sub 0 and the alarm).
fn after_engage(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, drone: bool) {
    if w.mobys[id].pv_i(COVER) != -1 {
        st(w, id, 0xf, 2);
        set_seq(w, id, ctx, 0xd, 0, 0xf);
    }
    if w.mobys[id].pv_i(STAND_PATH) != -1 {
        st(w, id, 0xe, 2);
        set_seq(w, id, ctx, 0xd, 0, 0xf);
    }
    if w.mobys[id].pv_i(RIDER) != 0 {
        let k = w.mobys[id].pv_i(WALKWAY);
        if let Some(e) = usize::try_from(k).ok().filter(|&k| k < w.mobys.len()) {
            w.mobys[e].state = 3;
            if 0x10 <= w.mobys[e].pvars.len() { ai::set_link(&mut w.mobys[e], 0xc, Some(id)); }
        }
    }
    if drone && w.mobys[id].pv_i(ESCORT) != 0 {
        st(w, id, 0xa, 7);
        set_seq(w, id, ctx, 3, 0, 0xf);
        senses(&mut w.mobys[id], SENSES_A);
    }
}

/// Sub 0's sighting: to cover (24, then the cuboid, the stand path, the walkway, the drone).
fn take_cover(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_i(ENGAGED, 1);
    st(w, id, 0x18, 2);
    set_seq(w, id, ctx, 6, 0, 10);
    after_engage(w, id, ctx, true);
}

/// Found while calm or on guard: to the attack, then the rest of [`take_cover`]'s (the drone only from calm).
fn engage(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, drone: bool) {
    w.mobys[id].set_pv_i(ENGAGED, 1);
    engage_fire(w, id, ctx, 10, 2);
    after_engage(w, id, ctx, drone);
}

/// Shown and brought in by the teleport-in carrier (`+0x34` = (mode & 0xffbe) | 0x1000, its collision back).
fn appear(w: &mut MobyWorld, id: usize) {
    let c = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.mode = (m.mode & 0xffbe) | mode::TARGET;
    m.has_collision = c;
    let (pos, rot) = (m.pos, m.rot);
    crate::moby::classes::teleport::start(w, id, pos, rot, &[]);
}

/// Let in once `+0x298` is gone: along `+0x29c` (28, sub 6), else to the attack.
fn released(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_i(HIDDEN, 0);
    if w.mobys[id].pv_i(IN_PATH) != -1 {
        st(w, id, 0x1c, 6);
        set_seq(w, id, ctx, 0xd, 0, 0xf);
    } else {
        engage_fire(w, id, ctx, 10, 2);
    }
}

/// A new burst: `+0x284` and a random `0..n`, none fired.
fn new_burst(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, n: i32) {
    let r = ctx.game.rng.randi(n);
    let m = &mut w.mobys[id];
    m.set_pv_i(FIRED, 0);
    m.set_pv_i(BURST, m.pv_i(BURST_BASE) + r);
}

/// The stand sequences: 1 or 0x16, and 4 one time in three.
fn stand_seq(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.rng.randi(2) == 1 { set_seq(w, id, ctx, 1, 0, 0x14); } else { set_seq(w, id, ctx, 0x16, 0, 10); }
    if ctx.game.rng.randi(3) == 1 { set_seq(w, id, ctx, 4, 0, 10); }
}

/// `0x40ada8(m, path)`: the wall walk along a path (crate::moby::frames::move_on_walls; onto it, [`path_set`], when
/// it is not the block's `+0xc4`). Its test of the planet word `0x1a79f8` against 0x45 (no planet's) never holds, so
/// it walks to the path's last point (`+0xca`) outright; within 0.5 across of it, its index `+0xc8` steps toward the
/// last, and once there it is at its end.
fn crawl(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, path: i32) -> bool {
    let m = &mut w.mobys[id];
    if m.pv_i(0x18) == 0 { return false; }
    let o = ai::mv(m);
    if m.pv_i(o + 0xc4) != path + 1 { path_set(m, ctx, path, -1, 0); }
    let Some(pts) = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| !p.is_empty()) else { return false };
    let last = w.mobys[id].pv_s(o + 0xca);
    let p = pts[(last.max(0) as usize).min(pts.len() - 1)];
    let goal = [p[0], p[1], p[2]];
    crate::moby::frames::move_on_walls(w, id, ctx, goal);
    let m = &mut w.mobys[id];
    if 0.5 < dist2(m.pos, goal) { return false; }
    let i = m.pv_s(o + 0xc8);
    if last == i { return true; }
    m.set_pv_s(o + 0xc8, if last < i { i - 1 } else { i + 1 });
    false
}

/// `0x40b9e8`: on walls, the wall walk's turn toward Ratchet in its own frame (crate::moby::frames::turn_walls).
fn crawl_face(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    if !crawls(m) || m.pv_i(0x18) == 0 { return; }
    let o = ai::mv(m);
    let (v, rows) = (sub(ctx.hero.pos, m.pos), m.rows);
    crate::moby::frames::turn_walls(m, o, v, &rows);
    w.changed = true;
}

/// Along a path (`0x336a68`), or with `+0x2a0` the wall walk's.
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, path: i32) -> bool {
    if crawls(&w.mobys[id]) { crawl(w, id, ctx, path) } else { ai::follow_path(w, id, ctx, path) }
}

/// It walks on walls: the trooper's `+0x2a0`, the heavy's `+0x2a4`.
fn crawls(m: &Moby) -> bool { m.pv_i(if m.o_class == HEAVY { H_CRAWLS } else { CRAWLS }) != 0 }

/// To a point (`0x335760`), or on walls the wall walk's.
fn go_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3) {
    if crawls(&w.mobys[id]) {
        crate::moby::frames::move_on_walls(w, id, ctx, goal);
    } else {
        let p = w.mobys[id].pos;
        ai::move_step(w, id, ctx, atan2(goal[0] - p[0], goal[1] - p[1]), goal);
    }
}

/// The line from joint 0 to the target: clear, or meeting one of `ok` (Ratchet and the target always).
fn line_clear(w: &MobyWorld, id: usize, ctx: &Ctx, ok: &[i16]) -> bool {
    let m = &w.mobys[id];
    let from = ctx.host.joint(m, 0).unwrap_or(m.pos);
    let to = m.pv_v(T_POINT);
    match w.coll_line(ctx, from, to, 0, Some(id)) {
        None => true,
        Some(o) => o.moby.is_some_and(|k| Some(k) == ctx.hero_moby || Some(k) == ai::link(m, T_MOBY) || w.mobys.get(k).is_some_and(|t| ok.contains(&t.o_class))),
    }
}

/// A shot from joint 0 at the target point.
fn shoot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let p = ctx.host.joint(m, 0).unwrap_or(m.pos);
    let v = set_len(sub(m.pv_v(T_POINT), p), SHOT_SPEED);
    let dmg = m.pv_b(DAMAGE) as f32;
    fire(w, ctx, id, v, p, dmg);
    let m = &mut w.mobys[id];
    m.set_pv_i(FIRED, m.pv_i(FIRED) + 1);
}

/// Facing the target (`0x336870`).
fn face_target(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let t = target_pos(w, id, ctx);
    let m = &mut w.mobys[id];
    ai::face(m, atan2(t[0] - m.pos[0], t[1] - m.pos[1]));
}

/// The throw by the sequence just set (0x19 rock, 0x1b paper, else scissors).
fn throw(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    st(w, id, 6, -1);
    let r = ctx.game.rng.randi(3) + 0x19;
    if seq(w, id, ctx) != r as u8 {
        let s = ctx.game.rng.randi(3) + 0x19;
        ctx.host.set_anim(&w.mobys[id], s as u8, 0, 3);
    }
    let t = match seq(w, id, ctx) { 0x19 => 0, 0x1b => 1, _ => 2 };
    w.mobys[id].set_pv_i(THROW, t);
}

/// 0x0ea3 (module docs).
pub fn update_0ea3(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    if head(w, id, ctx) { crate::moby::classes::death_fade::start(w, id, ctx, 0.0, None); }
    // Dead in sub 4: no states, no tail.
    if subs(w, id, ctx) { return; }
    states(w, id, ctx);
    tail(w, id, ctx);
    w.changed = true;
}

/// `0x40a570` (module docs); true when it fell out.
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let mirror = ctx.game.rng.randi(2) != 0;
        let r = ctx.game.rng.randi(2);
        let m = &mut w.mobys[id];
        let pos = m.pos;
        m.set_pv_v(HOME, pos);
        m.set_pv_f(0x30, 1.25);
        if mirror { m.mode |= mode::MIRROR; }
        m.set_pv_i(BURST, m.pv_i(BURST_BASE) + r);
        m.set_pv_b(REACT + 0x10, 0x1a);
        m.set_pv_i(REACT + 0x48, 4);
        m.set_pv_f(REACT + 0x3c, f32::from_bits(0x3e19_999a));
        m.set_pv_b(REACT + 0x15, 12);
        m.set_pv_i(FIRED, 0);
        m.set_pv_i(ENGAGED, 0);
        m.set_pv_f(REACT + 0x34, 0.5);
        m.set_pv_f(REACT + 0x30, 0.5);
        if m.pv_i(REACT_7C) != -1 { m.set_pv_i(REACT + 0x7c, m.pv_i(REACT_7C)); }
        if m.pv_s(MOST) == 0 {
            m.set_pv_s(MOST, 1);
            m.set_pv_f(HEALTH, 1.0);
        }
        m.set_pv_f(MV + 0x34, 0.5);
        m.set_pv_f(MV + 4, f32::from_bits(0x3e4c_cccd));
        m.set_pv_f(MV, f32::from_bits(0x3f53_3333));
        m.set_pv_f(MV + 0xc, 0.75);
        m.set_pv_f(MV + 8, 0.75);
        m.set_pv_f(MV + 0x10, f32::from_bits(0x3fc9_0fdc));
        m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
        if m.pv_i(CRAWLS) != 0 {
            m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 4);
            m.set_pv_f(MV + 0xc, f32::from_bits(0x3f33_3333));
            m.set_pv_f(MV + 8, f32::from_bits(0x3f33_3333));
            m.mode |= mode::KEEP_ROWS;
            m.set_pv_f(MV + 0x10, f32::from_bits(0x4149_0fdc));
            m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 1);
        }
        if m.pv_i(RIDER) != 0 {
            m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 5);
            m.set_pv_f(MV, 0.0);
            m.set_pv_f(REACT + 0x40, 97.5);
        }
        senses(m, SENSES_B);
        st(w, id, 1, 0);
        set_seq(w, id, ctx, 0x15, 0, 0);
        if w.mobys[id].pv_i(PATROL) != -1 {
            st(w, id, 3, 0);
            set_seq(w, id, ctx, 3, 0, 0);
            senses(&mut w.mobys[id], SENSES_A);
        }
        if w.mobys[id].pv_i(HIDDEN) != 0 && w.mobys[id].pv_i(RELEASE) != -1 { st(w, id, 0x1b, 5); }
        if w.mobys[id].pv_i(RPS) != 0 {
            st(w, id, 5, 0);
            set_seq(w, id, ctx, 0x18, 0, 0);
        }
        if w.mobys[id].pv_i(HIDE) != 0 {
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.mode = (m.mode | 0x41) & 0xefff;
            st(w, id, 1, 8);
        }
        let m = &mut w.mobys[id];
        // `0x2fa680` (L00 `0x2effb8`): 17 and 12 as bytes.
        m.set_pv_b(0x48, 17);
        m.set_pv_b(0x4a, 12);
        m.set_pv_b(0x29, 1);
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 22);
    if ai::take_damage(w, id, ctx, false) != 0 { alarm(w, id); }
    for (area, range, out) in [(AREA3, None, FOUND3), (AREA4, None, FOUND4), (AREA5, None, FOUND5), (AREA1, Some(RANGE1), FOUND1), (AREA2, Some(RANGE2), FOUND2)] {
        let m = &w.mobys[id];
        let (a, r) = (m.pv_i(area), range.map_or(0.0, |o| m.pv_f(o)));
        let f = ai::find_target(w, id, ctx, r, 0.0, a, None, false);
        w.mobys[id].set_pv_i(out, f as i32);
    }
    let m = &w.mobys[id];
    if m.pos[2] < 89.0 || (m.pv_i(RIDER) != 0 && m.pos[2] < 98.0) {
        w.mobys[id].set_pv_f(HEALTH, 0.0);
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        return true;
    }
    false
}

/// `0x40baa8`: its group alarmed — each 0x0ea3 and 0x0ea4 `+0x24c`, each 0x0f35 `+0x244`.
fn alarm(w: &mut MobyWorld, id: usize) {
    let g = w.mobys[id].group;
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for k in list {
        let Some(m) = w.mobys.get_mut(k) else { return };
        let off = match m.o_class { CLASS | HEAVY => ALARM, DRONE => 0x244, _ => continue };
        if off + 4 <= m.pvars.len() { m.set_pv_i(off, 1); }
    }
}

/// `0x408398`: its throw against its partner's (`+0x2c8`; a 0x0ea4's `+0x2c4`, any other class a draw): 2 a draw,
/// 1 it wins (rock over paper, paper over scissors, scissors over rock), 0 it loses.
fn rps(w: &MobyWorld, id: usize) -> i32 {
    let m = &w.mobys[id];
    let partner = usize::try_from(m.pv_i(RPS_PARTNER)).ok().and_then(|k| w.mobys.get(k));
    let (mine, other) = match partner {
        Some(e) if e.o_class == CLASS => (m.pv_i(THROW), e.pv_i(THROW)),
        Some(e) if e.o_class == HEAVY => (m.pv_i(THROW), e.pv_i(RPS)),
        _ => (m.pv_i(THROW), m.pv_i(THROW)),
    };
    match (mine, other) {
        (a, b) if a == b => 2,
        (0, 1) | (1, 2) | (2, 0) => 1,
        (0, 2) | (1, 0) | (2, 1) => 0,
        _ => 2,
    }
}

/// The sub-states (module docs); true when it died.
fn subs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    match w.mobys[id].sub_state {
        0 => {
            let m = &mut w.mobys[id];
            if m.pv_i(FOUND1) == 1 && m.count_i(WAIT) {
                if w.mobys[id].pv_i(RIDER) == 0 || ctx.hero.surf.boots == 1 { take_cover(w, id, ctx); }
            } else if m.pv_i(FOUND4) == 1 && m.pv_i(ONCE_DONE) == 0 {
                if m.pv_i(ONCE_PATH) != -1 {
                    st(w, id, 0x10, -1);
                    set_seq(w, id, ctx, 0xd, 0, 10);
                    w.mobys[id].set_pv_i(ONCE_DONE, 1);
                }
            } else if m.pv_i(ALARM) != 0 {
                if found(&w.mobys[id]) { engage(w, id, ctx, true); }
                let m = &mut w.mobys[id];
                m.set_pv_i(ALARM, 0);
                if m.pv_i(TIMER) < 300 {
                    let t = ctx.game.rng.rand_range(480, 600);
                    w.mobys[id].set_pv_i(TIMER, t);
                }
            }
        }
        8 => {
            let m = &w.mobys[id];
            if m.pv_i(HIDDEN) != 0 && m.pv_i(RELEASE) != -1 {
                if live(w, m.pv_i(RELEASE)).is_none() {
                    appear(w, id);
                    released(w, id, ctx);
                }
            } else {
                let m = &mut w.mobys[id];
                let seen = m.pv_i(FOUND1) == 1 && m.count_i(WAIT);
                if seen || w.mobys[id].timer == 1 {
                    w.mobys[id].set_pv_i(ENGAGED, 1);
                    engage_fire(w, id, ctx, 10, 2);
                    appear(w, id);
                }
            }
        }
        1 => {
            if found(&w.mobys[id]) { engage(w, id, ctx, false); }
        }
        2 => {
            let m = &w.mobys[id];
            if !found(m) && m.state != 0xf {
                st(w, id, 9, 1);
                set_seq(w, id, ctx, 0x15, 0, 0x1e);
                let t = ctx.game.rng.rand_range(0x244, 0x258);
                w.mobys[id].set_pv_i(TIMER, t);
                new_burst(w, id, ctx, 2);
            }
            let m = &w.mobys[id];
            if m.pv_i(FOUND3) == 1 && m.pv_i(ENGAGED) == 0 {
                st(w, id, 0x17, 3);
                set_seq(w, id, ctx, 0xd, 0, 0x14);
                new_burst(w, id, ctx, 2);
            }
        }
        5 => {
            let m = &w.mobys[id];
            if live(w, m.pv_i(RELEASE)).is_none() || m.pv_i(FOUND5) == 1 { released(w, id, ctx); }
        }
        4 => return hurt(w, id, ctx),
        _ => {}
    }
    false
}

/// Sub 4 (module docs); true when it died.
fn hurt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let r = ai::flight(w, id, ctx);
    if r & 1 != 0 {
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        crate::moby::classes::death_fade::start(w, id, ctx, 0.0, None);
        return true;
    }
    if r & 2 != 0 {
        if found(&w.mobys[id]) {
            set_seq(w, id, ctx, 0x13, 0, 10);
            st(w, id, 0x14, 2);
        } else {
            if ctx.game.rng.randi(2) == 1 { set_seq(w, id, ctx, 1, 0, 0x14); } else { set_seq(w, id, ctx, 4, 0, 10); }
            st(w, id, 9, 1);
            let t = ctx.game.rng.rand_range(0x244, 0x258);
            w.mobys[id].set_pv_i(TIMER, t);
        }
        if w.mobys[id].pv_i(STAND_PATH) != -1 {
            st(w, id, 0xe, 2);
            set_seq(w, id, ctx, 0xd, 0, 0xf);
        }
        let m = &w.mobys[id];
        if m.pv_i(ONCE_PATH) != -1 && m.pv_i(ENGAGED) == 0 {
            st(w, id, 0x10, -1);
            set_seq(w, id, ctx, 0xd, 0, 10);
        }
        if w.mobys[id].pv_i(HIDDEN) != 0 {
            set_seq(w, id, ctx, 0x15, 0, 10);
            st(w, id, 0x1b, 5);
        }
        let m = &w.mobys[id];
        if m.pv_i(ESCORT) != 0 && live(w, m.pv_i(DRONE_ID)).is_some() {
            st(w, id, 0xa, 7);
            set_seq(w, id, ctx, 3, 0, 0xf);
            senses(&mut w.mobys[id], SENSES_A);
        }
    }
    let m = &w.mobys[id];
    if m.pv_f(HEALTH) <= 0.0 && m.state_ticks as u16 == 0x28 {
        if let Some((rows, p)) = ctx.host.joint_matrix(m, 0) {
            let v = set_len(rows[1], f32::from_bits(0xbf55_5556));
            let dmg = m.pv_b(DAMAGE) as f32;
            fire(w, ctx, id, v, p, dmg);
        }
    }
    false
}

/// The states (module docs).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let ticks = w.mobys[id].state_ticks as u16;
    match w.mobys[id].state {
        5 => if 0x38 <= ticks { throw(w, id, ctx) },
        8 => if 0x47 <= ticks { throw(w, id, ctx) },
        6 => {
            if 0x3d <= ticks {
                match rps(w, id) {
                    2 => {
                        st(w, id, 5, -1);
                        set_seq(w, id, ctx, 0x18, 0, 3);
                    }
                    0 => {
                        st(w, id, 7, -1);
                        set_seq(w, id, ctx, 7, 2, 0x1b);
                    }
                    _ => {
                        st(w, id, 7, -1);
                        set_seq(w, id, ctx, 0x1c, 0, 10);
                    }
                }
            }
        }
        7 => {
            if 0x49 <= ticks {
                st(w, id, 8, -1);
                set_seq(w, id, ctx, 0x18, 0, 0x12);
            }
        }
        3 => {
            let path = w.mobys[id].pv_i(PATROL);
            if walk(w, id, ctx, path) {
                st(w, id, 4, -1);
                stand_seq(w, id, ctx);
            }
        }
        4 => {
            if wrapped(w, id, ctx) {
                let path = w.mobys[id].pv_i(PATROL);
                if let Some(pts) = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| !p.is_empty()) {
                    let (a, b) = (pts[0], pts[pts.len() - 1]);
                    let n = pts.len() as i16;
                    let pos = w.mobys[id].pos;
                    if dist2(pos, [b[0], b[1], b[2]]) < dist2(pos, [a[0], a[1], a[2]]) {
                        path_set(&mut w.mobys[id], ctx, path, 0, n - 1);
                    } else {
                        path_set(&mut w.mobys[id], ctx, path, n - 1, 0);
                    }
                }
                st(w, id, 3, -1);
                set_seq(w, id, ctx, 3, 0, 0x14);
            }
        }
        0x10 => {
            let path = w.mobys[id].pv_i(ONCE_PATH);
            if ai::follow_path(w, id, ctx, path) {
                w.mobys[id].set_pv_i(ENGAGED, 1);
                st(w, id, 2, -1);
                set_seq(w, id, ctx, 0x15, 0, 10);
            }
        }
        2 => st(w, id, 1, -1),
        9 => {
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 0x15, 0, 10);
                st(w, id, 1, -1);
            }
        }
        0xa => {
            let path = w.mobys[id].pv_i(ESCORT_PATH);
            if ai::follow_path(w, id, ctx, path) {
                set_seq(w, id, ctx, 0x15, 0, 10);
                st(w, id, 0xd, -1);
            }
            let m = &w.mobys[id];
            let t = m.pv_v(T_POINT);
            let off = rot_dist(m.rot[2], atan2(t[0] - m.pos[0], t[1] - m.pos[1]));
            if live(w, m.pv_i(DRONE_ID)).is_none() || f32::from_bits(0x3fdf_66f4) < off { drone_lost(w, id, ctx); }
        }
        0xb => {
            let m = &w.mobys[id];
            let (goal, off) = (m.pv_v(T_POINT), m.pv_f(SIDE_ANGLE) * f32::from_bits(0x3c8e_fa36));
            ai::move_toward(w, id, ctx, off, goal);
            if 0x65 <= ticks && ctx.game.rng.randi(10) == 1 { engage_fire(w, id, ctx, 10, 2); }
        }
        0x1c => {
            let path = w.mobys[id].pv_i(IN_PATH);
            let at = walk(w, id, ctx, path);
            let t = target_pos(w, id, ctx);
            if at || dist2(w.mobys[id].pos, t) < 8.0 { engage_fire(w, id, ctx, 10, 2); }
        }
        0xf => {
            let m = &mut w.mobys[id];
            if m.first() && m.pv_i(RIDER) != 0 {
                let (a, b) = (m.pv_i(GATE_A), m.pv_i(GATE_B));
                if a != -1 && b != -1 {
                    for k in [a, b] {
                        if let Some(g) = w.mobys.get_mut(k as usize) { g.state = 4; }
                    }
                    let m = &mut w.mobys[id];
                    m.set_pv_i(GATE_A, -1);
                    m.set_pv_i(GATE_B, -1);
                }
            }
            let goal = cuboid(ctx, w.mobys[id].pv_i(COVER));
            arrive(w, id, ctx, goal);
        }
        0x1d => {
            let goal = w.mobys[id].pv_v(HOME);
            arrive(w, id, ctx, goal);
        }
        0xd => {
            if live(w, w.mobys[id].pv_i(DRONE_ID)).is_none() {
                drone_lost(w, id, ctx);
            } else if line_clear(w, id, ctx, &[0x0c87]) {
                engage_fire(w, id, ctx, 10, 2);
            }
        }
        0x12 => {
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 0x13, 0, 10);
                st(w, id, 0x14, -1);
            }
        }
        0xe => {
            let path = w.mobys[id].pv_i(STAND_PATH);
            if w.mobys[id].pv_i(CRAWLS) != 0 {
                if crawl(w, id, ctx, path) {
                    engage_fire(w, id, ctx, 10, 2);
                    w.mobys[id].set_pv_i(STAND_PATH, -1);
                }
            } else {
                let at = ai::follow_path(w, id, ctx, path);
                let t = target_pos(w, id, ctx);
                if at || dist2(w.mobys[id].pos, t) < 8.0 {
                    st(w, id, 0x18, 2);
                    set_seq(w, id, ctx, 6, 0, 10);
                    w.mobys[id].set_pv_i(STAND_PATH, -1);
                }
            }
        }
        0x13 => {
            sight(w, id, ctx);
            if wrapped(w, id, ctx) {
                if crate::moby::classes::leaper::seq_a(w, id, ctx) == 0xe {
                    stand_seq(w, id, ctx);
                } else {
                    set_seq(w, id, ctx, 0xe, 0, 0x14);
                }
            }
        }
        0x14 => {
            face_target(w, id, ctx);
            crawl_face(w, id, ctx);
            if ai::reached(w, id, ctx, 1.0) { shoot(w, id, ctx); }
            if wrapped(w, id, ctx) {
                let m = &w.mobys[id];
                if m.pv_i(BURST) == m.pv_i(FIRED) {
                    st(w, id, 0x18, -1);
                    set_seq(w, id, ctx, 6, 0, 10);
                    new_burst(w, id, ctx, 4);
                } else {
                    aim_seq(w, id, ctx);
                    st(w, id, 0x13, -1);
                }
            }
        }
        0x15 => if 0x79 <= ticks { sight(w, id, ctx) },
        0x16 => {
            face_target(w, id, ctx);
            crawl_face(w, id, ctx);
            if ai::reached(w, id, ctx, 3.0) { shoot(w, id, ctx); }
            if wrapped(w, id, ctx) {
                let m = &w.mobys[id];
                if m.pv_i(BURST) == m.pv_i(FIRED) {
                    st(w, id, 0x18, -1);
                    set_seq(w, id, ctx, 6, 0, 10);
                    maybe_burst(w, id, ctx);
                } else {
                    aim_seq(w, id, ctx);
                    st(w, id, 0x15, -1);
                }
            }
        }
        0x19 => {
            face_target(w, id, ctx);
            crawl_face(w, id, ctx);
            if ai::reached(w, id, ctx, 3.0) { shoot(w, id, ctx); }
            let m = &w.mobys[id];
            if wrapped(w, id, ctx) && m.pv_i(BURST) == m.pv_i(FIRED) {
                st(w, id, 0x18, -1);
                set_seq(w, id, ctx, 6, 0, 0x14);
                maybe_burst(w, id, ctx);
            }
        }
        0x17 => {
            let goal = cuboid(ctx, w.mobys[id].pv_i(SPOT));
            let p = w.mobys[id].pos;
            ai::move_step(w, id, ctx, atan2(goal[0] - p[0], goal[1] - p[1]), goal);
            if dist2(w.mobys[id].pos, goal) < 0.5 {
                w.mobys[id].set_pv_i(ENGAGED, 1);
                st(w, id, 0x14, 2);
                set_seq(w, id, ctx, 0x13, 0, 0xf);
            }
        }
        0x18 => {
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 0xe, 0, 10);
                st(w, id, 0x15, 2);
                if w.mobys[id].pv_i(CROUCH) == 1 {
                    set_seq(w, id, ctx, 0xe, 0, 10);
                    st(w, id, 0x13, 2);
                }
                let m = &w.mobys[id];
                if m.pv_i(RIDER) != 0 && 6.0 < ctx.hero.pos[2] - m.pos[2] {
                    st(w, id, 0x19, -1);
                    set_seq(w, id, ctx, 0x11, 0, 0x14);
                }
                let m = &w.mobys[id];
                if m.pv_i(HOMING) != 0 && 4.0 < dist2(m.pos, m.pv_v(HOME)) {
                    st(w, id, 0x1d, -1);
                    set_seq(w, id, ctx, 0xd, 0, 10);
                }
            }
        }
        _ => {}
    }
}

/// States 15 / 29: to `goal`; within 0.5 (the wall walk's 0.9) crouched (20), with `+0x2a8` kneeling (25).
fn arrive(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3) {
    go_to(w, id, ctx, goal);
    let m = &w.mobys[id];
    let d = dist2(m.pos, goal);
    if m.pv_i(CRAWLS) != 0 {
        if d < f32::from_bits(0x3f66_6666) {
            st(w, id, 0x14, -1);
            set_seq(w, id, ctx, 0x13, 0, 0xf);
        }
    } else if d < 0.5 {
        st(w, id, 0x14, -1);
        set_seq(w, id, ctx, 0x13, 0, 0xf);
        if w.mobys[id].pv_i(RIDER) != 0 {
            st(w, id, 0x19, -1);
            set_seq(w, id, ctx, 0x11, 0, 0x14);
        }
    }
}

/// States 10 / 13 with the drone gone: to the attack, with `+0x2d8` sidestepping (11, the senses B).
fn drone_lost(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    engage_fire(w, id, ctx, 10, 2);
    if w.mobys[id].pv_i(SIDESTEP) != 0 {
        st(w, id, 0xb, 2);
        set_seq(w, id, ctx, 0xd, 0, 10);
        senses(&mut w.mobys[id], SENSES_B);
    }
}

/// States 19 / 21: the line to the target (module docs) clear, to the attack (the sequence 5 blended over 1 tick, or
/// a random 0..14 when it meets one of them).
fn sight(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let from = ctx.host.joint(m, 0).unwrap_or(m.pos);
    let to = m.pv_v(T_POINT);
    let ok: [i16; 6] = [0x0c87, 0x0c86, 0x01f4, 0x10ce, 0x0200, 0x01ff];
    match w.coll_line(ctx, from, to, 0, Some(id)) {
        None => {
            set_seq(w, id, ctx, 5, 0, 1);
            fire_tail(w, id, ctx);
        }
        Some(o) => {
            let tm = ai::link(&w.mobys[id], T_MOBY);
            let hit = o.moby.is_some_and(|k| Some(k) == ctx.hero_moby || Some(k) == tm || w.mobys.get(k).is_some_and(|t| ok.contains(&t.o_class)));
            if hit {
                if seq(w, id, ctx) != 5 {
                    let b = ctx.game.rng.randi(0xf);
                    ctx.host.set_anim(&w.mobys[id], 5, 0, b as u8);
                }
                fire_tail(w, id, ctx);
            }
        }
    }
}

/// [`sight`]'s state: 22 (its sub kept), crouched 20 (sub 2).
fn fire_tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    st(w, id, 0x16, -1);
    if w.mobys[id].pv_i(CROUCH) == 1 {
        set_seq(w, id, ctx, 0x13, 0, 10);
        st(w, id, 0x14, 2);
    }
}

/// The aim sequence 0xe, blended over a random 10..14 ticks.
fn aim_seq(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if seq(w, id, ctx) != 0xe {
        let b = ctx.game.rng.randi(5) + 10;
        ctx.host.set_anim(&w.mobys[id], 0xe, 0, b as u8);
    }
}

/// One time in two a new burst (a random 0..3 more), none fired.
fn maybe_burst(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.rng.randi(2) == 1 {
        let r = ctx.game.rng.randi(4);
        let m = &mut w.mobys[id];
        m.set_pv_i(BURST, m.pv_i(BURST_BASE) + r);
    }
    w.mobys[id].set_pv_i(FIRED, 0);
}

/// `0x40ba38`: the two timers, its ticks, its head aim.
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    for off in [TIMER, TIMER2] {
        let mut t = m.pv_i(off);
        countdown(&mut t);
        m.set_pv_i(off, t);
    }
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = (m.state_ticks as u16).wrapping_add(1) as i16; }
    crate::moby::joint_aim::head(w, ctx, id, HEAD_AIM, 1, 2.5);
}

/// `0x40f0a0(owner, vel, pos, _, damage)`: a shot 0x0ea7 at `pos` flying `vel` a tick (module docs): its update and
/// draw distances 0xff, drawn, the owner's light, mode | 0x208 | 0x100, its angles along its way (its yaw a quarter
/// turn on; a random roll drawn and lost to the rows from the angles), alpha 0x80, scale 0.0725.
pub fn fire(w: &mut MobyWorld, ctx: &mut Ctx, owner: usize, vel: V3, pos: V3, damage: f32) -> usize {
    let k = w.create(SHOT);
    let (light, amb) = (w.mobys[owner].light, w.mobys[owner].ambient);
    let m = &mut w.mobys[k];
    if m.pvars.len() < S_PVARS { m.pvars.resize(S_PVARS, 0); }
    m.update = Some(update_0ea7);
    m.mode &= !mode::NO_UPDATE;
    (m.update_dist, m.draw_dist, m.drawn, m.state) = (0xff, 0xff, 1, 0);
    (m.light, m.ambient) = (light, amb);
    m.pos = pos;
    m.set_pv_v(S_VEL, vel);
    ai::set_link(m, S_OWNER, Some(owner));
    m.set_pv_f(S_DAMAGE, damage);
    m.set_pv_f(0x20, f32::from_bits(0x3e19_999a));
    m.mode |= 0x208;
    m.rot[2] = add_rot(atan2(vel[0], vel[1]), f32::from_bits(0x3fc9_0fdc));
    m.rot[0] = -atan2(len2(vel), vel[2]);
    ctx.game.rng.rand_angle();
    let m = &mut w.mobys[k];
    m.rows = crate::hero::physics::euler_rows(m.rot);
    m.alpha = 0x80;
    m.mode |= mode::KEEP_ROWS;
    m.scale = f32::from_bits(0x3d94_7ae1);
    m.set_pv_i(S_LIFE, 0);
    let s = ctx.game.rng.randf(8.0, 10.0);
    if let Some(sys) = ctx.parts.as_deref_mut() {
        crate::particles::twinkle(sys, &mut ctx.game.rng, s * 0.3, s, 0.0, pos, 15, 0x8080_3010, 0, 10, [0.0; 3], 0);
        crate::particles::twinkle(sys, &mut ctx.game.rng, s * f32::from_bits(0x3d23_d70a), s * f32::from_bits(0x3ecc_cccd), 0.0, pos, 15, 0xffff_ffff, 1, -10, [0.0; 3], 0);
    }
    ctx.host.play_sound(&w.mobys[k], 0, 0);
    k
}

/// 0x0ea7 (`0x40edb0`, module docs).
pub fn update_0ea7(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.pvars.len() < S_PVARS { m.pvars.resize(S_PVARS, 0); }
    let life = m.pv_i(S_LIFE) + 1;
    m.set_pv_i(S_LIFE, life);
    let dmg = m.pv_f(S_DAMAGE);
    if 0xf0 < life {
        let pos = w.mobys[id].pos;
        ai::attack_sphere(w, id, ctx, f32::from_bits(0x3f19_999a), dmg, 0.0, pos, 0x1_0001, 0, 1, 0);
        st(w, id, 1, -1);
    }
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (pos, vel, owner) = (m.pos, m.pv_v(S_VEL), ai::link(m, S_OWNER));
            let next = add(pos, vel);
            let mut t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 1, [vel[0], vel[1], vel[2], 0.0]);
            t.kind = (SHOT as u16 as u32) << 16;
            let (coll, scene) = (ctx.coll, ctx.scene);
            let line = coll.and_then(|c| crate::collision::coll_line_m(c, scene, pos, next, crate::collision::QueryFlags(0), Some(id)));
            if let Some(o) = line.as_ref() {
                if let Some(k) = o.moby { w.deliver_hit_any(k, &t, [o.point[0], o.point[1], o.point[2], 1.0]); }
                let hit = match o.moby {
                    None => Some((3.0, f32::from_bits(0x3f19_999a))),
                    Some(k) if Some(k) != owner => Some((2.0, f32::from_bits(0x3e99_999a))),
                    _ => None,
                };
                if let Some((k, r)) = hit {
                    let d = crate::hero::physics::scale(vel, k);
                    let mut t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 0x1_0001, [d[0], d[1], d[2], 5_627.925]);
                    t.kind = (SHOT as u16 as u32) << 16 | 1 << 8;
                    w.sphere_hit(scene, o.point, r, 0, Some(id), Some(&t));
                    st(w, id, 1, -1);
                    return;
                }
            }
            w.mobys[id].pos = next;
        }
        1 => {
            let pos = w.mobys[id].pos;
            let b = blast::Blast { size: 0.25, bursts: [25, 20, 20], puffs: 20, streaks: 0, light: true, shake: false };
            blast::burst_blue(w, ctx, pos, &b);
            w.delete(id);
            return;
        }
        _ => {}
    }
    w.changed = true;
    w.register_draw(id);
}

/// The shot's draw (`0x40f2d0`, crate::fx::trails::shot_beam).
pub fn draw_0ea7(w: &mut MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    crate::fx::trails::shot_beam(w, id, rng, cam, out);
}

// ---- The heavy trooper 0x0ea4 ----

/// Its words where they differ from the trooper's (module docs of [`update_0ea4`]).
const H_GUN: usize = 0x25c;
const H_KIND: usize = 0x260;
const H_FACE: usize = 0x264;
const H_SPOT: usize = 0x268;
const H_FIRED: usize = 0x26c;
const H_BURST_BASE: usize = 0x270;
const H_BURST: usize = 0x274;
const H_ONCE_PATH: usize = 0x278;
const H_AREA4: usize = 0x27c;
const H_FOUND4: usize = 0x280;
const H_ONCE_DONE: usize = 0x284;
const H_ENGAGED: usize = 0x288;
const H_HURT_START: usize = 0x28c;
const H_KNEEL: usize = 0x290;
const H_STAND_PATH: usize = 0x294;
const H_HIDDEN: usize = 0x298;
const H_RELEASE: usize = 0x29c;
const H_IN_PATH: usize = 0x2a0;
const H_CRAWLS: usize = 0x2a4;
const H_RPS: usize = 0x2bc;
const H_PARTNER: usize = 0x2c0;
const H_THROW: usize = 0x2c4;
const H_HIDE: usize = 0x2c8;
const H_HEAD_AIM: usize = 0x2d0;
const H_GATE_A: usize = 0x350;
const H_GATE_B: usize = 0x354;
const H_HOMING: usize = 0x358;
const H_REACT_7C: usize = 0x35c;
const H_HOME: usize = 0x360;
const H_PVARS: usize = 0x370;
/// Its launcher, carried at its joint 0 (no update of its own).
pub const LAUNCHER: i16 = 0x0ea9;
/// The senses C (`0x3ada740e`, `0x3d2aaaab`, 0.4°, 48°): its patrol's.
const SENSES_C: [f32; 4] = [f32::from_bits(0x3ada_740e), f32::from_bits(0x3d2a_aaab), f32::from_bits(0x3be4_c38a), f32::from_bits(0x3f56_7752)];
/// 45° (`0x3f490fdc`): the most its facing may be off the target for a lob.
const LOB_ARC: f32 = f32::from_bits(0x3f49_0fdc);

/// Its launcher, when it is one.
fn gun(w: &MobyWorld, id: usize) -> Option<usize> { ai::link(&w.mobys[id], H_GUN).filter(|&k| w.mobys.get(k).is_some_and(|g| g.o_class == LAUNCHER)) }

/// Its launcher deleted.
fn drop_gun(w: &mut MobyWorld, id: usize) { if let Some(g) = gun(w, id) { w.delete(g); } }

/// To the attack: sequence 5 (0xc kneeling with `+0x290`) and 21, sub 2.
fn h_attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, sub: i32) {
    set_seq(w, id, ctx, 5, 0, 10);
    if w.mobys[id].pv_i(H_KNEEL) == 1 { set_seq(w, id, ctx, 0xc, 0, 10); }
    st(w, id, 0x15, sub);
}

/// The cuboid, the stand path and the walkway after a sighting.
fn h_after(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(H_SPOT) != -1 {
        st(w, id, 0x12, 2);
        set_seq(w, id, ctx, 3, 0, 0xf);
    }
    if w.mobys[id].pv_i(H_STAND_PATH) != -1 {
        st(w, id, 0xf, 2);
        set_seq(w, id, ctx, 3, 0, 0xf);
    }
    if w.mobys[id].pv_i(RIDER) != 0 {
        let k = w.mobys[id].pv_i(WALKWAY);
        if let Some(e) = usize::try_from(k).ok().filter(|&k| k < w.mobys.len()) {
            w.mobys[e].state = 3;
            if 0x10 <= w.mobys[e].pvars.len() { ai::set_link(&mut w.mobys[e], 0xc, Some(id)); }
        }
    }
}

/// Found (subs 0 / 1): engaged, sequence 0xf and 19, sub 2.
fn h_engage(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_i(H_ENGAGED, 1);
    set_seq(w, id, ctx, 0xf, 0, 0x19);
    st(w, id, 0x13, 2);
    h_after(w, id, ctx);
}

/// Shown with its launcher and brought in by the teleport-in carrier.
fn h_appear(w: &mut MobyWorld, id: usize) {
    if let Some(g) = gun(w, id) {
        let c = w.class(LAUNCHER).is_some_and(|c| c.has_collision);
        let m = &mut w.mobys[g];
        m.mode = (m.mode & 0xffbe) | mode::TARGET;
        m.has_collision = c;
    }
    appear(w, id);
}

/// Let in once `+0x29c` is gone: along `+0x2a0` (27, sub 7), else to the attack (21 set before its sequences when
/// `state_first`).
fn h_released(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, state_first: bool) {
    w.mobys[id].set_pv_i(H_HIDDEN, 0);
    if w.mobys[id].pv_i(H_IN_PATH) != -1 {
        st(w, id, 0x1b, 7);
        set_seq(w, id, ctx, 3, 0, 0xf);
    } else if state_first {
        st(w, id, 0x15, 2);
        set_seq(w, id, ctx, 5, 0, 10);
        if w.mobys[id].pv_i(H_KNEEL) == 1 { set_seq(w, id, ctx, 0xc, 0, 10); }
    } else {
        h_attack(w, id, ctx, 2);
    }
}

/// The stand sequences: 1 or 0, and 0xe one time in five.
fn h_stand_seq(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.rng.randi(2) == 1 { set_seq(w, id, ctx, 1, 0, 0x14); } else { set_seq(w, id, ctx, 0, 0, 0x14); }
    if ctx.game.rng.randi(5) == 1 { set_seq(w, id, ctx, 0xe, 0, 0x1e); }
}

/// After a burst: 23, sequence 6 (7 one time in two), a new burst of `+0x270` and 0..3.
fn h_reload(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    st(w, id, 0x17, -1);
    set_seq(w, id, ctx, 6, 0, 10);
    if ctx.game.rng.randi(2) == 1 { set_seq(w, id, ctx, 7, 0, 10); }
    let r = ctx.game.rng.randi(4);
    let m = &mut w.mobys[id];
    m.set_pv_i(H_FIRED, 0);
    m.set_pv_i(H_BURST, m.pv_i(H_BURST_BASE) + r);
}

/// The facing within 45° of the way to the target point.
fn h_on_target(m: &Moby) -> bool {
    let t = m.pv_v(T_POINT);
    rot_dist(m.rot[2], atan2(t[0] - m.pos[0], t[1] - m.pos[1])) < LOB_ARC
}

/// The line from `from` to the target point clear or meeting Ratchet, the target or one of 0x0c87, 0x0c86, 0x01f4,
/// 0x0200, 0x01ff.
fn h_clear(w: &MobyWorld, id: usize, ctx: &Ctx, from: V3) -> bool {
    let m = &w.mobys[id];
    match w.coll_line(ctx, from, m.pv_v(T_POINT), 0, Some(id)) {
        None => true,
        Some(o) => o.moby.is_some_and(|k| Some(k) == ctx.hero_moby || Some(k) == ai::link(m, T_MOBY) || w.mobys.get(k).is_some_and(|t| [0x0c87, 0x0c86, 0x01f4, 0x0200, 0x01ff].contains(&t.o_class))),
    }
}

/// A glob (crate::moby::classes::level17::lobber::fire, `0x40e240`) from the launcher's joint 0 at the target point,
/// `k` a tick; `test`: only along a clear line ([`h_clear`]). True when it went.
fn h_lob(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, k: f32, test: bool) -> bool {
    let from = match gun(w, id) { Some(g) => ctx.host.joint(&w.mobys[g], 0).unwrap_or(w.mobys[g].pos), None => w.mobys[id].pos };
    let m = &w.mobys[id];
    let v = set_len(sub(m.pv_v(T_POINT), from), k);
    let dmg = m.pv_b(DAMAGE) as f32;
    if test && !h_clear(w, id, ctx, from) { return false; }
    crate::moby::classes::level17::lobber::fire(w, ctx, dmg, id, v, from);
    true
}

/// `0x40bba0`: its throw (`+0x2c4`) against its partner's (`+0x2c0`; a 0x0ea3's `+0x2cc`, a 0x0ea4's `+0x2c4`, any other
/// class a draw), as [`rps`].
fn h_rps(w: &MobyWorld, id: usize) -> i32 {
    let m = &w.mobys[id];
    let partner = usize::try_from(m.pv_i(H_PARTNER)).ok().and_then(|k| w.mobys.get(k));
    let (mine, other) = match partner {
        Some(e) if e.o_class == CLASS => (m.pv_i(H_THROW), e.pv_i(THROW)),
        Some(e) if e.o_class == HEAVY => (m.pv_i(H_THROW), e.pv_i(H_THROW)),
        _ => (m.pv_i(H_THROW), m.pv_i(H_THROW)),
    };
    match (mine, other) {
        (a, b) if a == b => 2,
        (0, 1) | (1, 2) | (2, 0) => 1,
        (0, 2) | (1, 0) | (2, 1) => 0,
        _ => 2,
    }
}

/// The heavy's throw by the sequence just set (0x13 rock, 0x15 paper, else scissors).
fn h_throw(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    st(w, id, 0xa, -1);
    let r = ctx.game.rng.randi(3) + 0x13;
    if seq(w, id, ctx) != r as u8 {
        let s = ctx.game.rng.randi(3) + 0x13;
        ctx.host.set_anim(&w.mobys[id], s as u8, 0, 3);
    }
    let t = match seq(w, id, ctx) { 0x13 => 0, 0x15 => 1, _ => 2 };
    w.mobys[id].set_pv_i(H_THROW, t);
}

/// The heavy trooper **0x0ea4** (L13 `0x40bc88`, 33 placed; L16 `0x3fa758`, 13; the decomp's v2.00
/// `func_US2_L13_0040BC84` with no body, its v1.01 `func_L13_0040A720` the same code but for the two calls the v2
/// trooper renames too; its helpers v1.01 `func_L13_0040C3A0` / `0040CA00` / `0040CA70` / `0040A638` at our L13
/// `0x40d820` / `0x40de80` / `0x40def0` / `0x40bba0`): the trooper's big brother, lobbing globs
/// (crate::moby::classes::level17::lobber's 0x0ea5) from the launcher 0x0ea9 it carries at its joint 0. Health `+0x20`
/// (its kill byte `+0x2e`: 2 deletes it), movement `+0x50` (steps 0.4), reaction `+0x140` (a hurt to state 26, sub
/// 3), target `+0x1f0`, as the trooper's.
///
/// **Its words** (where not the trooper's): the launcher `+0x25c`; `+0x260` its post (1 the rock-paper-scissors
/// stance 2, 2 the lean 3); `+0x264` a moby it faces (state 6); `+0x268` a cuboid it runs to; `+0x26c` globs fired,
/// `+0x270` a burst less 0..3 (`+0x274` this one's); `+0x278` a path it takes once found by `+0x27c` (found
/// `+0x280`, done `+0x284`); `+0x288` engaged; `+0x28c` it starts hurt (1, sub 4); `+0x290` it kneels; `+0x294` a path
/// to a stand; `+0x298` hidden until `+0x29c` is gone, then in along `+0x2a0`; `+0x2a4` it walks on walls (the
/// trooper's wall walk, not ported: it stays); `+0x2a8` / `+0x2ac` the walkway it rides, `+0x350` / `+0x354` its
/// gates; `+0x2b0` its patrol, `+0x2b4` a search that lets it in (found `+0x2b8`); `+0x2bc` it plays
/// rock-paper-scissors with `+0x2c0`, its throw `+0x2c4`; `+0x2c8` hidden at first; `+0x2d0` its head aim (list 4,
/// 2.5); `+0x358` it goes home (`+0x360`) when more than 4 from it; `+0x35c` the reaction's `+0x7c`.
///
/// **Each tick**: the head (`0x40d820`): in state 0 the set-up (home, the launcher made and carried (`0x333758` = L00
/// `0x32a748`, its draw distance its own, rows kept), the senses B, idle 1 / sequence 0xd, then by its words: its post,
/// hurt, the patrol (7, the senses C), hidden (28, sub 6), rock-paper-scissors (9, sequence 0x12), hidden at first
/// (with its launcher; 1, sub 8)); its shadow within 24; its launcher at its joint 0's frame; its kill byte 2: both
/// deleted; the frame's damage alarms its group (`0x40def0`: 0x0ea3 / 0x0ea4); the four searches; under 89 (98 riding)
/// the launcher deleted, health 0, its bolts. Fallen or killed, the dying fade (not for the deleted). Then the
/// sub-states, the states and the tail (`0x40de80`: the timers, its ticks, its head aim).
///
/// **Sub-states**: **0** calm: found once `+0x248` is out (riding, only with Ratchet on a magnetic walkway), to cover
/// (23: sequence 6, 7 one time in two; then the cuboid 18, the stand path 15, the walkway); else found by `+0x27c`
/// once `+0x248` is out (counted again), along `+0x278` (16); else alarmed and found, [`h_engage`] (19). **8** hidden:
/// its moby gone, shown and in; else found or `+0xbc` = 1, shown, in and to the attack. **6** waiting for `+0x29c`. **4**
/// still (`0x335a20` = L00 `0x32ce98`, ai::hold): found, 19 (sub 5). **5** still: lost and not running to its cuboid,
/// 1 (sub 4). **1** on guard: found, [`h_engage`]; its timer out, calm. **2** fighting (the senses B once): lost and not
/// at its cuboid, wary (13, sub 1), a new burst. **3** hurt (ai::flight): dead, its launcher deleted, its bolts and the
/// fade; over, to the attack when found, else wary; then its once-path and stand path.
///
/// **States**: **9** / **12** a throw after 56 / 71 ticks (10, the sequences 0x13..0x15); **10** after 61 ticks the
/// throws compared ([`h_rps`]): a draw 9, lost or won 11 (sequences 7 / 0x16); **11** after 73 ticks 12. **7** its
/// patrol, at its end 8; **8** the path back, 7. **5** / **4** at the end of the sequence 6 / 1 (sub 0). **6** facing
/// `+0x264`. **13** to 1. **27** along `+0x2a0` (or within 8 of the target) to the attack. **14** to 16. **16** along
/// `+0x278`, engaged, 17; **17** to 1. **18** / **29** to the cuboid `+0x268` / home (the gates opened first), within
/// 0.5 to the attack (riding, 24, sequence 0x10). **15** along `+0x294` (or within 8) to the attack. **19** at the
/// sequence's end to the attack. **25** facing within 45° and the line from its joint 2 clear, to the attack; at the
/// sequence's end the stand sequences. **21** / **22** / **24** facing the target, at frame 1 within 45° a glob from
/// the launcher at 30 / 25 / 30 a second (21 and 22 along a clear line only); 21 / 24 at the sequence's end with globs
/// out [`h_reload`] (none: 25). **23** facing the target: off level 16 at the sequence's end 25 (riding 24; homing and
/// away, 29); on level 16 after 181 ticks to the attack, its sequence 4 at each end.
pub fn update_0ea4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < H_PVARS { w.mobys[id].pvars.resize(H_PVARS, 0); }
    if h_head(w, id, ctx) {
        if w.mobys.get(id).is_some_and(|m| m.state < 0xfd) { crate::moby::classes::death_fade::start(w, id, ctx, 0.0, None); }
    }
    if w.mobys.get(id).is_none_or(|m| m.state >= 0xfd) { return; }
    if h_subs(w, id, ctx) { return; }
    h_states(w, id, ctx);
    let m = &mut w.mobys[id];
    for off in [TIMER, TIMER2] {
        let mut t = m.pv_i(off);
        countdown(&mut t);
        m.set_pv_i(off, t);
    }
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = (m.state_ticks as u16).wrapping_add(1) as i16; }
    crate::moby::joint_aim::head(w, ctx, id, H_HEAD_AIM, 4, 2.5);
    w.changed = true;
}

/// `0x40d820` ([`update_0ea4`]'s docs); true when it fell out or was killed.
fn h_head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let r = ctx.game.rng.randi(4);
        let mirror = ctx.game.rng.randi(2) != 0;
        let m = &mut w.mobys[id];
        let pos = m.pos;
        m.set_pv_v(H_HOME, pos);
        m.set_pv_b(HEALTH + 0xe, 1);
        m.set_pv_f(0x30, 1.25);
        m.set_pv_i(H_BURST, m.pv_i(H_BURST_BASE) + r);
        m.set_pv_i(H_ONCE_DONE, 0);
        m.set_pv_i(H_ENGAGED, 0);
        if mirror { m.mode |= mode::MIRROR; }
        m.set_pv_b(REACT + 0x10, 0x1a);
        m.set_pv_i(REACT + 0x48, 3);
        m.set_pv_f(REACT + 0x34, 0.5);
        m.set_pv_f(REACT + 0x30, 0.5);
        m.set_pv_f(REACT + 0x3c, f32::from_bits(0x3e19_999a));
        m.set_pv_b(REACT + 0x15, 0xb);
        if m.pv_i(H_REACT_7C) != -1 { m.set_pv_i(REACT + 0x7c, m.pv_i(H_REACT_7C)); }
        if m.pv_s(MOST) == 0 {
            m.set_pv_s(MOST, 1);
            m.set_pv_f(HEALTH, 1.0);
        }
        let g = w.create(LAUNCHER);
        ai::set_link(&mut w.mobys[id], H_GUN, Some(g));
        if w.mobys[g].o_class == LAUNCHER {
            let (pos, rot, light, dd) = { let o = &w.mobys[id]; (o.pos, o.rot, o.light, o.draw_dist) };
            let x = &mut w.mobys[g];
            x.update_dist = 0xff;
            x.drawn = 1;
            x.parent = Some(id);
            (x.pos, x.rot, x.light) = (pos, rot, light);
            x.draw_dist = dd;
            x.mode |= mode::KEEP_ROWS;
            x.drive = true;
        }
        let m = &mut w.mobys[id];
        m.set_pv_f(MV + 0x34, 0.5);
        m.set_pv_f(MV + 4, f32::from_bits(0x3e4c_cccd));
        m.set_pv_f(MV, f32::from_bits(0x3f53_3333));
        m.set_pv_f(MV + 0xc, f32::from_bits(0x3ecc_cccd));
        m.set_pv_f(MV + 8, f32::from_bits(0x3ecc_cccd));
        m.set_pv_f(MV + 0x10, f32::from_bits(0x3fc9_0fdc));
        m.set_pv_f(MV + 0x30, f32::from_bits(0x3f06_0a93));
        if m.pv_i(H_CRAWLS) != 0 {
            m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 4);
            m.set_pv_f(MV + 0xc, f32::from_bits(0x3f33_3333));
            m.set_pv_f(MV + 8, f32::from_bits(0x3f33_3333));
            m.mode |= mode::KEEP_ROWS;
            m.set_pv_f(MV + 0x10, f32::from_bits(0x4149_0fdc));
            m.set_pv_f(MV + 0x34, 0.5);
            m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 1);
        }
        if m.pv_i(RIDER) != 0 {
            m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 5);
            m.set_pv_f(MV, 0.0);
            m.set_pv_f(REACT + 0x40, 97.5);
        }
        senses(m, SENSES_B);
        st(w, id, 1, 0);
        set_seq(w, id, ctx, 0xd, 0, 0);
        match w.mobys[id].pv_i(H_KIND) {
            1 => {
                st(w, id, 2, 0);
                set_seq(w, id, ctx, 0x12, 0, 0);
            }
            2 => {
                st(w, id, 3, 0);
                set_seq(w, id, ctx, 2, 0, 0);
            }
            _ => {}
        }
        if w.mobys[id].pv_i(H_HURT_START) != 0 { st(w, id, 1, 4); }
        if w.mobys[id].pv_i(PATROL) != -1 {
            st(w, id, 7, 0);
            set_seq(w, id, ctx, 2, 0, 0);
            senses(&mut w.mobys[id], SENSES_C);
        }
        if w.mobys[id].pv_i(H_HIDDEN) != 0 && w.mobys[id].pv_i(H_RELEASE) != -1 { st(w, id, 0x1c, 6); }
        if w.mobys[id].pv_i(H_RPS) != 0 {
            st(w, id, 9, 0);
            set_seq(w, id, ctx, 0x12, 0, 0);
        }
        if w.mobys[id].pv_i(H_HIDE) != 0 {
            let m = &mut w.mobys[id];
            m.has_collision = false;
            m.mode = (m.mode | 0x41) & 0xefff;
            if let Some(g) = gun(w, id) {
                let x = &mut w.mobys[g];
                x.mode = (x.mode | 0x41) & 0xefff;
                x.has_collision = false;
            }
            st(w, id, 1, 8);
        }
        let m = &mut w.mobys[id];
        m.set_pv_b(0x29, 1);
        m.set_pv_b(0x48, 17);
        m.set_pv_b(0x4a, 12);
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 22);
    if let Some(g) = gun(w, id) {
        if let Some((rows, at)) = ctx.host.joint_matrix(&w.mobys[id], 0) {
            let x = &mut w.mobys[g];
            x.rows = rows;
            x.pos = at;
        }
    }
    if w.mobys[id].pv_b(HEALTH + 0xe) == 2 {
        drop_gun(w, id);
        w.delete(id);
        return true;
    }
    if ai::take_damage(w, id, ctx, false) != 0 { h_alarm(w, id); }
    for (area, range, out) in [(H_AREA4, None, H_FOUND4), (AREA5, None, FOUND5), (AREA1, Some(RANGE1), FOUND1), (AREA2, Some(RANGE2), FOUND2)] {
        let m = &w.mobys[id];
        let (a, r) = (m.pv_i(area), range.map_or(0.0, |o| m.pv_f(o)));
        let f = ai::find_target(w, id, ctx, r, 0.0, a, None, false);
        w.mobys[id].set_pv_i(out, f as i32);
    }
    let m = &w.mobys[id];
    if m.pos[2] < 89.0 || (m.pv_i(RIDER) != 0 && m.pos[2] < 98.0) {
        drop_gun(w, id);
        w.mobys[id].set_pv_f(HEALTH, 0.0);
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        return true;
    }
    false
}

/// `0x40def0`: its group alarmed — each 0x0ea3 and 0x0ea4 `+0x24c`.
fn h_alarm(w: &mut MobyWorld, id: usize) {
    let g = w.mobys[id].group;
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for k in list {
        let Some(m) = w.mobys.get_mut(k) else { return };
        if matches!(m.o_class, CLASS | HEAVY) && ALARM + 4 <= m.pvars.len() { m.set_pv_i(ALARM, 1); }
    }
}

/// The heavy's sub-states; true when it died.
fn h_subs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    match w.mobys[id].sub_state {
        0 => {
            let m = &mut w.mobys[id];
            if m.pv_i(FOUND1) == 1 && m.count_i(WAIT) {
                if w.mobys[id].pv_i(RIDER) == 0 || ctx.hero.surf.boots == 1 {
                    w.mobys[id].set_pv_i(H_ENGAGED, 1);
                    st(w, id, 0x17, 2);
                    set_seq(w, id, ctx, 6, 0, 10);
                    if ctx.game.rng.randi(2) == 1 { set_seq(w, id, ctx, 7, 0, 10); }
                    h_after(w, id, ctx);
                }
            } else if m.pv_i(H_FOUND4) == 1 && m.pv_i(H_ONCE_DONE) == 0 && m.count_i(WAIT) {
                if m.pv_i(H_ONCE_PATH) != -1 {
                    st(w, id, 0xe, -1);
                    set_seq(w, id, ctx, 3, 0, 10);
                    w.mobys[id].set_pv_i(H_ONCE_DONE, 1);
                }
            } else if m.pv_i(ALARM) != 0 {
                if found(&w.mobys[id]) { h_engage(w, id, ctx); }
                let m = &mut w.mobys[id];
                m.set_pv_i(ALARM, 0);
                if m.pv_i(TIMER) < 300 {
                    let t = ctx.game.rng.rand_range(480, 600);
                    w.mobys[id].set_pv_i(TIMER, t);
                }
            }
        }
        8 => {
            let m = &w.mobys[id];
            if m.pv_i(H_HIDDEN) != 0 && m.pv_i(H_RELEASE) != -1 {
                if live(w, m.pv_i(H_RELEASE)).is_none() {
                    h_appear(w, id);
                    h_released(w, id, ctx, false);
                }
            } else {
                let m = &mut w.mobys[id];
                let seen = m.pv_i(FOUND1) == 1 && m.count_i(WAIT);
                if seen || w.mobys[id].timer == 1 {
                    w.mobys[id].set_pv_i(H_ENGAGED, 1);
                    h_attack(w, id, ctx, 2);
                    h_appear(w, id);
                }
            }
        }
        6 => {
            let m = &w.mobys[id];
            if live(w, m.pv_i(H_RELEASE)).is_none() || m.pv_i(FOUND5) == 1 { h_released(w, id, ctx, true); }
        }
        4 => {
            ai::hold(w, id, ctx);
            if w.mobys[id].pv_i(FOUND1) == 1 {
                st(w, id, 0x13, 5);
                set_seq(w, id, ctx, 0xf, 0, 0x28);
            }
        }
        5 => {
            ai::hold(w, id, ctx);
            let m = &w.mobys[id];
            if !found(m) && m.state != 0x12 {
                st(w, id, 1, 4);
                set_seq(w, id, ctx, 0xd, 0, 0x1e);
            }
        }
        1 => {
            if found(&w.mobys[id]) {
                h_engage(w, id, ctx);
            } else if w.mobys[id].pv_i(TIMER) == 0 {
                st(w, id, 1, 0);
            }
        }
        2 => {
            let m = &mut w.mobys[id];
            if m.once & 2 == 0 {
                m.once |= 2;
                senses(m, SENSES_B);
            }
            let m = &w.mobys[id];
            if !found(m) && m.state != 0x12 {
                st(w, id, 0xd, 1);
                set_seq(w, id, ctx, 0xd, 0, 0x1e);
                let t = ctx.game.rng.rand_range(0x244, 0x258);
                w.mobys[id].set_pv_i(TIMER, t);
                let r = ctx.game.rng.randi(4);
                let m = &mut w.mobys[id];
                m.set_pv_i(H_FIRED, 0);
                m.set_pv_i(H_BURST, m.pv_i(H_BURST_BASE) + r);
            }
        }
        3 => {
            let r = ai::flight(w, id, ctx);
            if r & 1 != 0 {
                drop_gun(w, id);
                crate::moby::bolts::drop(w, id, 0, 3, ctx);
                crate::moby::classes::death_fade::start(w, id, ctx, 0.0, None);
                return true;
            }
            if r & 2 != 0 {
                if found(&w.mobys[id]) {
                    h_attack(w, id, ctx, 2);
                } else {
                    if ctx.game.rng.randi(2) == 1 { set_seq(w, id, ctx, 1, 0, 0x14); } else { set_seq(w, id, ctx, 0, 0, 0x14); }
                    if ctx.game.rng.randi(5) == 1 { set_seq(w, id, ctx, 0xe, 0, 0x1e); }
                    st(w, id, 0xd, 1);
                    let t = ctx.game.rng.rand_range(0x244, 0x258);
                    w.mobys[id].set_pv_i(TIMER, t);
                }
                let m = &w.mobys[id];
                if m.pv_i(H_ONCE_PATH) != -1 && m.pv_i(H_ENGAGED) == 0 {
                    st(w, id, 0x10, -1);
                    set_seq(w, id, ctx, 3, 0, 10);
                }
                if w.mobys[id].pv_i(H_STAND_PATH) != -1 {
                    st(w, id, 0xf, 2);
                    set_seq(w, id, ctx, 3, 0, 0xf);
                }
            }
        }
        _ => {}
    }
    false
}

/// States 18 / 29: to `goal`, within 0.5 to the attack (riding, 24 with the sequence 0x10).
fn h_arrive(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3) {
    go_to(w, id, ctx, goal);
    if dist2(w.mobys[id].pos, goal) < 0.5 {
        st(w, id, 0x15, -1);
        set_seq(w, id, ctx, 5, 0, 0xf);
        if w.mobys[id].pv_i(H_KNEEL) == 1 { set_seq(w, id, ctx, 0xc, 0, 10); }
        if w.mobys[id].pv_i(RIDER) != 0 {
            st(w, id, 0x18, -1);
            set_seq(w, id, ctx, 0x10, 0, 0x14);
        }
    }
}

/// To the attack with the state set after the sequences (states 15, 19, 25, 27).
fn h_attack_late(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    set_seq(w, id, ctx, 5, 0, 10);
    if w.mobys[id].pv_i(H_KNEEL) == 1 { set_seq(w, id, ctx, 0xc, 0, 10); }
    st(w, id, 0x15, -1);
}

/// The heavy's states ([`update_0ea4`]'s docs).
fn h_states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let ticks = w.mobys[id].state_ticks as u16;
    match w.mobys[id].state {
        9 => if 0x38 <= ticks { h_throw(w, id, ctx) },
        0xc => if 0x47 <= ticks { h_throw(w, id, ctx) },
        0xa => {
            if 0x3d <= ticks {
                match h_rps(w, id) {
                    2 => {
                        st(w, id, 9, -1);
                        set_seq(w, id, ctx, 0x12, 0, 3);
                    }
                    0 => {
                        st(w, id, 0xb, -1);
                        set_seq(w, id, ctx, 7, 0, 10);
                    }
                    _ => {
                        st(w, id, 0xb, -1);
                        set_seq(w, id, ctx, 0x16, 0, 10);
                    }
                }
            }
        }
        0xb => {
            if 0x49 <= ticks {
                st(w, id, 0xc, -1);
                set_seq(w, id, ctx, 0x12, 0, 0x12);
            }
        }
        7 => {
            let path = w.mobys[id].pv_i(PATROL);
            if walk(w, id, ctx, path) {
                st(w, id, 8, -1);
                h_stand_seq(w, id, ctx);
            }
        }
        8 => {
            if wrapped(w, id, ctx) {
                let path = w.mobys[id].pv_i(PATROL);
                if let Some(pts) = usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| !p.is_empty()) {
                    let (a, b) = (pts[0], pts[pts.len() - 1]);
                    let n = pts.len() as i16;
                    let pos = w.mobys[id].pos;
                    if dist2(pos, [b[0], b[1], b[2]]) < dist2(pos, [a[0], a[1], a[2]]) {
                        path_set(&mut w.mobys[id], ctx, path, 0, n - 1);
                    } else {
                        path_set(&mut w.mobys[id], ctx, path, n - 1, 0);
                    }
                }
                st(w, id, 7, -1);
                set_seq(w, id, ctx, 2, 0, 0x14);
            }
        }
        5 => {
            if wrapped(w, id, ctx) {
                st(w, id, 6, 0);
                set_seq(w, id, ctx, 0xd, 0, 0x28);
            }
        }
        4 => {
            if wrapped(w, id, ctx) {
                st(w, id, 1, 0);
                set_seq(w, id, ctx, 0xd, 0, 0x14);
            }
        }
        6 => {
            let at = usize::try_from(w.mobys[id].pv_i(H_FACE)).ok().and_then(|k| w.mobys.get(k)).map(|e| e.pos);
            if let Some(e) = at {
                let m = &mut w.mobys[id];
                ai::face(m, atan2(e[0] - m.pos[0], e[1] - m.pos[1]));
            }
        }
        0xd => {
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 0xd, 0, 10);
                st(w, id, 1, -1);
            }
        }
        0x1b => {
            let path = w.mobys[id].pv_i(H_IN_PATH);
            let at = walk(w, id, ctx, path);
            let t = target_pos(w, id, ctx);
            if at || dist2(w.mobys[id].pos, t) < 8.0 { h_attack_late(w, id, ctx); }
        }
        0xe => {
            st(w, id, 0x10, -1);
            set_seq(w, id, ctx, 3, 0, 10);
        }
        0x10 => {
            let path = w.mobys[id].pv_i(H_ONCE_PATH);
            if ai::follow_path(w, id, ctx, path) {
                w.mobys[id].set_pv_i(H_ENGAGED, 1);
                st(w, id, 0x11, -1);
                set_seq(w, id, ctx, 0xd, 0, 10);
            }
        }
        0x11 => st(w, id, 1, -1),
        0x12 => {
            let m = &mut w.mobys[id];
            if m.first() && m.pv_i(RIDER) != 0 {
                let (a, b) = (m.pv_i(H_GATE_A), m.pv_i(H_GATE_B));
                if a != -1 && b != -1 {
                    for k in [a, b] {
                        if let Some(g) = w.mobys.get_mut(k as usize) { g.state = 4; }
                    }
                    let m = &mut w.mobys[id];
                    m.set_pv_i(H_GATE_A, -1);
                    m.set_pv_i(H_GATE_B, -1);
                }
            }
            let goal = cuboid(ctx, w.mobys[id].pv_i(H_SPOT));
            h_arrive(w, id, ctx, goal);
        }
        0x1d => {
            let goal = w.mobys[id].pv_v(H_HOME);
            h_arrive(w, id, ctx, goal);
        }
        0xf => {
            let path = w.mobys[id].pv_i(H_STAND_PATH);
            let done = if crawls(&w.mobys[id]) {
                crawl(w, id, ctx, path)
            } else {
                let at = ai::follow_path(w, id, ctx, path);
                at || dist2(w.mobys[id].pos, target_pos(w, id, ctx)) < 8.0
            };
            if done {
                st(w, id, 0x15, -1);
                set_seq(w, id, ctx, 5, 0, 10);
                if w.mobys[id].pv_i(H_KNEEL) == 1 { set_seq(w, id, ctx, 0xc, 0, 10); }
                w.mobys[id].set_pv_i(H_STAND_PATH, -1);
            }
        }
        0x13 => if wrapped(w, id, ctx) { h_attack_late(w, id, ctx) },
        0x19 => {
            if h_on_target(&w.mobys[id]) {
                let m = &w.mobys[id];
                let from = ctx.host.joint(m, 2).unwrap_or(m.pos);
                if h_clear(w, id, ctx, from) { h_attack_late(w, id, ctx); }
            }
            if wrapped(w, id, ctx) {
                if crate::moby::classes::leaper::seq_a(w, id, ctx) == 4 {
                    if ctx.game.rng.randi(2) == 1 { set_seq(w, id, ctx, 1, 0, 0x14); } else { set_seq(w, id, ctx, 0, 0, 0x14); }
                } else {
                    set_seq(w, id, ctx, 4, 0, 0x14);
                }
            }
        }
        0x15 => {
            face_target(w, id, ctx);
            crawl_face(w, id, ctx);
            if ai::reached(w, id, ctx, 1.0) && h_on_target(&w.mobys[id]) && h_lob(w, id, ctx, 0.5, true) {
                let m = &mut w.mobys[id];
                m.set_pv_i(H_FIRED, m.pv_i(H_FIRED) + 1);
            }
            if wrapped(w, id, ctx) {
                if w.mobys[id].pv_i(H_FIRED) <= 0 {
                    st(w, id, 0x19, -1);
                    set_seq(w, id, ctx, 4, 0, 10);
                } else {
                    h_reload(w, id, ctx);
                }
            }
        }
        0x18 => {
            face_target(w, id, ctx);
            crawl_face(w, id, ctx);
            if ai::reached(w, id, ctx, 1.0) && h_on_target(&w.mobys[id]) {
                let m = &mut w.mobys[id];
                m.set_pv_i(H_FIRED, m.pv_i(H_FIRED) + 1);
                h_lob(w, id, ctx, 0.5, false);
            }
            if wrapped(w, id, ctx) && 0 < w.mobys[id].pv_i(H_FIRED) { h_reload(w, id, ctx); }
        }
        0x16 => {
            face_target(w, id, ctx);
            crawl_face(w, id, ctx);
            if ai::reached(w, id, ctx, 1.0) && h_lob(w, id, ctx, f32::from_bits(0x3ed5_5556), true) {
                let m = &mut w.mobys[id];
                m.set_pv_i(H_FIRED, m.pv_i(H_FIRED) + 1);
            }
        }
        0x17 => {
            face_target(w, id, ctx);
            if ctx.game.level != 0x10 {
                if wrapped(w, id, ctx) {
                    st(w, id, 0x19, -1);
                    set_seq(w, id, ctx, 4, 0, 10);
                    if w.mobys[id].pv_i(RIDER) != 0 {
                        st(w, id, 0x18, -1);
                        set_seq(w, id, ctx, 0x10, 0, 0x14);
                    }
                    let m = &w.mobys[id];
                    if m.pv_i(H_HOMING) != 0 && 4.0 < dist2(m.pos, m.pv_v(H_HOME)) {
                        st(w, id, 0x1d, -1);
                        set_seq(w, id, ctx, 3, 0, 10);
                    }
                }
            } else {
                if 0xb5 <= ticks {
                    st(w, id, 0x15, -1);
                    set_seq(w, id, ctx, 5, 0, 10);
                    if w.mobys[id].pv_i(H_KNEEL) == 1 { set_seq(w, id, ctx, 0xc, 0, 10); }
                    if w.mobys[id].pv_i(RIDER) != 0 {
                        st(w, id, 0x18, -1);
                        set_seq(w, id, ctx, 0x10, 0, 0x14);
                    }
                }
                if wrapped(w, id, ctx) { set_seq(w, id, ctx, 4, 0, 5); }
            }
        }
        _ => {}
    }
}
