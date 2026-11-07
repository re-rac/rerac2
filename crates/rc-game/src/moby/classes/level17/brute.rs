//! Smolg's brute **0x0bcb** ([`update_0bcb`], `0x3e8670`, 50 placed; the decomp's v1.01 `func_L17_003E7068`, its
//! head `func_L17_003E8A58` at our `0x3ea060` (assembly only), its helpers v1.01 `003E6DE8` / `003E6F20` /
//! `003E6F58` / `003E7010` at our `0x3e83f0` / `0x3e8528` / `0x3e8560` / `0x3e8618`, and the level's own `0x32d410`):
//! a big walker that waits in a pod (crate::moby::classes::level17::hatchling's 0x0920 / 0x0929) or in the open, and
//! once Ratchet is in its trigger area comes out, walks at him through its floor's path network and swipes at him;
//! after a swipe it backs off and comes again. The members of a group take turns: one at a time (the leader, the
//! nearest to its target) runs in, the others keep their distance. On the framework (crate::moby::ai): health
//! `+0x20` (its damage byte `+0x34`), reaction `+0x60` (a hurt to sub 3), target block `+0x240` (its moby `+0x250`),
//! movement `+0x110` (radius 1, steps 1 / 1, slope 75°, gravity 1/90, its fence the path `+0x684`, option 0x10 no
//! turn).
//!
//! **Its pvars:** `+0x280` its path network (crate::path_network, 32 records at most, the count `+0x750`, the
//! records the steer is at and heads for `+0x758` / `+0x754`), `+0x680` its trigger area, `+0x684` its floor path
//! (−1 for either: deleted at the set-up), `+0x69c` the wait (s16, 200), `+0x69e` found (s16), `+0x6a4` the target's
//! moby (copied), `+0x6b8` its pod (a moby index, −1 none; a pod with index 0 is taken for none at the set-up),
//! `+0x6c0` where it came out (its `w` the facing at the set-up), `+0x6dc` a moby it waits on (−1 none), `+0x700` the
//! swipe's damage (the damage byte), `+0x704` / `+0x708` the stand-off and back-off timers, `+0x710` the walk's goal,
//! `+0x720` its group's turn (the shared words, crate::moby::MobyWorld::shared_i: `+0x744` the leader's index, `+0x748`
//! the election — 0 none, 1 under way, 2 settled — and `+0x74c` the nearest distance so far, a float), `+0x730` taking
//! turns (1), `+0x738` the camera dodges.
//!
//! **The head** ([`head`], `0x3ea060`): in state 0 the set-up: 0.85 times its class's size, in a pod 1.4 over it; the
//! reaction (sequences 14 / 13 / 8 / 7 / 9, bytes `+0xcc..+0xd5`, keys 1/3 / 1/120, gravities 4/60 / 1/1200 /
//! 0.00139, speeds 1.1 / 0.4 / 1, the flight's 40), the movement (radius 1, its fence its floor path); state 1 sub 5
//! in a pod, else 3 sub 0; the turn's words reset; the network built from its floor path (`0x32c910` = L02 `0x32f8b0`,
//! crate::path_network::build, its openings kept). Then drawn within 24 of the camera its shadow (range 0x16); a hit
//! (flags 0x10000, kept) from a 0x031a / 0x1256 / 0x1305 is let go unfelt, else, out of state 1, the frame's damage
//! (`0x32a160`, crate::moby::ai::take_damage); the search (its area, `0x32dd28`).
//!
//! **Subs** (`+0x95`): **0** waiting: found, the stand-off (15 sub 2) unless it takes turns in a group and is not the
//! leader ([`leader`]), else the walk-in (7 sub 6, sequence 1); either way its slow limits (12/3600, 5/60, 6°, 6°),
//! the wait 200, no turn. **2**: found keeps the wait at 200; at its end 5 sub 4 (sequence 3), the wait 220. **3**
//! hurt (its first tick: a playback speed not in 0..1 made 1, faster limits ×8 / ×8): under z 75 its bolts and gone;
//! the flight (`0x32aea8`, its rise at most 1/60 a tick): dead the fade (`0x3f2b58`) and its bolts (`0x31b870`, limit
//! 3); a swing at frames 13..14 on sequence 4 (see 12); on sequence 4 to frame 13 the walk to its target; the flight
//! over from frame 10, 13 sub 2; on sequence 13 from frame 10 sequence 4 (over 8) and Ratchet's invincibility ended
//! (`0x189ffc` = 0). **4**: found, alerted (11 sub 2, as 0's limits, the wait 200, no turn); else the wait as 6's.
//! **5** in a pod: once the moby it waits on is gone (or none), found and its pod ready ([`hatchling::pod_ready`])
//! the pod opens ([`hatchling::pod_open`]) and it climbs out (6); out of a 0x0929 shown with its collision, facing as
//! the pod; no pod: as 0. **6** coming out, as the leader: alerted at its sequence's end or off sequence 1; not the
//! leader: found keeps the wait; sequence 3 at its end → 14 (over 10), 14 past frame 20 → 3; on sequence 1 at its end
//! 14, on 1 facing its target (6° a tick); at the wait's end 3 sub 0 (sequence 2 over 20).
//!
//! **States** (`+0x20`): **3** idle (sequence 2 or 14 at random, again at 2's end or 14's frame 20). **4** the
//! stand-off: at its timer's end the swipe (12, sequence 4 or 5 at random). **6** climbing out (on its first tick out
//! of a 0x0929 sequence 10 backwards, else 1; on the ground 3 under it; `+0x6c0` its place): sequence 1 wrapped:
//! sequence 10 backwards and the move in place; on 10 3 ahead along its facing (on the ground 0.5 under it); 4.9
//! across from `+0x6c0`: taking turns and the leader the walk-in (7 sub 6, sequence 3), else alerted (11 sub 2).
//! **8** a walk to a point of its floor path (one more than 3 away at random, five tries: one in its sight 1 up, or
//! the wall there 2 or more away; none: 3), sequence 12, turning 2.5° a tick; at the goal, stuck 11 ticks or a wall:
//! 3. **9** sinking (sequence 7, 0.05 a tick; under z 70 its bolts and gone). **11** alerted (sequence 10 backwards at
//! 1.5, its limits ×1.5 and 3.33°): within 3 across of its target, near the camera (6) and under 25 dodges a point 12
//! past the camera toward its target kept in its path (3 in) more than 6.3 away is walked to; else the swipe (12,
//! sequence 4 or 5 from frame 2); farther the walk through the network (`0x32cda0` = L02 `0x32fd40`, crate::
//! path_network::steer, 0.5 in; `0x32fc50` = L00 `0x32cd20`, crate::moby::ai::move_step), facing its target; stuck 61
//! ticks, the stand-off (4, 25 ticks, sequence 2). **12** the swipe: facing its target 3.33° a tick; at frames 13..14,
//! on one sequence, within 110° and 4 of its target and in sight of it ([`in_sight`]) a 0.95 sphere at its joint 4 (5
//! on sequence 4) of its damage (flags 0x10001); on 4 or 5 from frame 19 the back-off (13). **13** the back-off (first
//! tick: 4 behind it kept in its path; within 2 a side step ([`side_step`], 5 across, 1 in); a wall 0.5 up stops it
//! there; sequence 10, limits 20/3600 / 5/60, its back-off timer 30, its turn given up ([`release`]), speed 1.5): the
//! walk to it; at it, stuck 10, a wall or the ground under its joint 3 below 10: within 4 the swipe, else the walk-in
//! (7 sub 6, sequence 3); the timer out within 3, the swipe; facing its target. **15** the stand-off (sequence 1,
//! facing its target; at its sequence's end alerted).
//!
//! **Each tick, last:** the ground under its joint `$gp+0x4950` (a word of the bss nothing here writes: taken as 0) 1
//! up below 20 while alerted or backing off (not hurt): 9 sub 7.
//!
//! The engine functions with the port's: `0x31c380` = L00 `0x31b8d0` (set_state), `0x31c0a0` = L00 `0x31b5f0`
//! (set_anim, crate::moby::classes::props::set_seq), `0x32d7b8` / `0x32d7e0` (the first tick of the state / sub),
//! `0x330be8` = L00 `0x32dcf0` (crate::moby::ai::senses), `0x32fb10` = L01 `0x322450` (the move to a point,
//! crate::moby::ai::move_step), `0x320318` = L00 `0x31f030` (approach_angle), `0x31e4f0` = L00 `0x31d2c8`
//! (crate::moby::ai::attack_joint), `0x31e048` = L00 `0x31ce20` (ground_z), `0x3132c8` = L00 `0x3127c8` (a joint's
//! place), `0x2df9b0` = L00 `0x2de878` (the line, crate::moby::MobyWorld::coll_line), `0x32d318` = L02 `0x3302b8`
//! (crate::path_network::keep_in), `0x312380` = L00 `0x311880` (the frame time), `0x2ef908` / `0x2ef8d8`
//! (crate::moby::countdown), `0x31c450` / `0x31c4e8` (crate::rng), `0x31ef28` = L00 `0x31dce8` (the hit),
//! `0x31ec98` (crate::shadow_volumes::probe_near), `0x312188` = L00 `0x311688` (delete), `0x3c87d8` (a 0x0929 pod
//! told about it: its `+0xa4`).
//!
//! Not ported: riding its pod (`0x324050` = L00 `0x322058`, sub 5; as the hatchling's), and the messages
//! `0x2a5e70` prints.

use crate::hero::physics::{add, add_rot, approach_angle, atan2, dist, dist2, rot_dist, set_len, sub, sub_rot, V3};
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::classes::{death_fade, level17::hatchling};
use crate::moby::{ai, bolts, Ctx, MobyWorld};
use crate::path_network;

pub const CLASS: i16 = 0x0bcb;
/// The pod it rides out (crate::moby::classes::level17::hatchling's POD_B).
const POD_B: i16 = 0x0929;

const MV: usize = 0x110;
const TGT: usize = 0x240;
const T_MOBY: usize = 0x250;
const NODES: usize = 0x280;
const NODES_MAX: usize = 0x20;
const AREA: usize = 0x680;
const PATH: usize = 0x684;
const WAIT: usize = 0x69c;
const FOUND: usize = 0x69e;
const T_COPY: usize = 0x6a4;
const POD: usize = 0x6b8;
const OUT: usize = 0x6c0;
const WAIT_FOR: usize = 0x6dc;
const DAMAGE: usize = 0x700;
const T_OFF: usize = 0x704;
const T_BACK: usize = 0x708;
const GOAL: usize = 0x710;
const TURN: usize = 0x720;
const TURNS: usize = 0x730;
const DODGES: usize = 0x738;
const SH_LEADER: usize = 0x744;
const SH_STATUS: usize = 0x748;
const SH_BEST: usize = 0x74c;
const COUNT: usize = 0x750;
const NET_GOAL: usize = 0x754;
const NET_AT: usize = 0x758;
const PVARS: usize = 0x760;

/// `$gp+0x1dfc`: under this it is gone (sub 3).
const KILL_Z: f32 = 75.0;
/// `$gp+0x1e00`: its radius.
const RADIUS: f32 = 1.0;
/// `$gp+0x1e08`: its steps.
const STEP: f32 = 1.0;
/// `$gp+0x1e18`: the swipe's reach.
const REACH: f32 = 4.0;
/// `$gp+0x1e28` / `+0x1e2c` (ints): the turn accelerations (/3600); `+0x1e30` / `+0x1e34` the walk's (/60).
const TURN_A: f32 = 12.0;
const TURN_B: f32 = 20.0;
const WALK_A: f32 = 5.0;
const WALK_B: f32 = 5.0;
/// `$gp+0x1e3c` / `+0x1e40`: the swipe's range across and its slack.
const NEAR: f32 = 3.0;
const SLACK: f32 = 1.0;
/// `$gp+0x1e48`: the back-off's length.
const BACK: f32 = 4.0;
/// `$gp+0x1e4c` (int): the stand-off's ticks; `+0x1e54` the back-off's.
const OFF_TICKS: i32 = 25;
const BACK_TICKS: i32 = 30;
/// `$gp+0x1e50`: the side step's length.
const SIDE: f32 = 5.0;
/// `$gp+0x1e5c` (int): its blend.
const BLEND: u8 = 10;
/// `$gp+0x1e64`: the camera's nearness; `+0x1e68` (int) the dodges at most.
const CAM_NEAR: f32 = 6.0;
const DODGES_MOST: i32 = 25;
/// `$gp+0x1e6c`: the swipe's last frame; `+0x1e70` / `+0x1e74` the hurt's frames.
const SWIPE_END: f32 = 19.0;
const HURT_OVER: f32 = 10.0;
const HURT_ROAR: f32 = 10.0;
/// `$gp+0x1e78`: the backing speed.
const BACK_SPEED: f32 = 1.5;
/// `$gp+0x1e80` / `+0x1e84` (ints): the swipe's joints (sequence 4: the second); `+0x1e88` its sphere.
const JOINT_A: usize = 4;
const JOINT_B: usize = 5;
const SPHERE: f32 = 0.95;
/// `$gp+0x1ef8`: the reaction's gravity (/60).
const REACT_G: f32 = 4.0;
/// `$gp+0x4950` (`0x1b3940`, bss, written by nothing found): the joint of the last tick's ground test.
const SINK_JOINT: usize = 0;
/// 6° / 3.33° / 2.5° a tick.
const TURN_6: u32 = 0x3dd6_7752;
const TURN_3: u32 = 0x3d6e_4bb0;
const TURN_2: u32 = 0x3d32_b8c4;
/// 110°: the swipe's cone.
const CONE: f32 = 1.919_862_4;

fn st(w: &mut MobyWorld, id: usize, s: u8, sub: i32) { w.mobys[id].set_state(s, sub); }
fn seq_a(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a) }
fn found(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_s(FOUND) == 1 }
fn tgt(w: &MobyWorld, id: usize) -> V3 { w.mobys[id].pv_v(TGT) }
fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 {
    let q = w.mobys[id].pos;
    atan2(p[0] - q[0], p[1] - q[1])
}
fn turn(w: &mut MobyWorld, id: usize, yaw: f32, rate: u32) { approach_angle(yaw, f32::from_bits(rate), &mut w.mobys[id].rot[2], 0); }
fn speed(w: &MobyWorld, id: usize, ctx: &mut Ctx, v: f32) { ctx.host.set_anim_speed(&w.mobys[id], v); }
fn senses(w: &mut MobyWorld, id: usize, a: f32, b: f32, c: u32) {
    ai::senses(&mut w.mobys[id], a * (1.0 / 3600.0), b * (1.0 / 60.0), f32::from_bits(c), f32::from_bits(c));
}
/// Its slow limits (subs 0 / 4 / 6, states 6 / 11).
fn slow(w: &mut MobyWorld, id: usize) { senses(w, id, TURN_A, WALK_A, TURN_6); }
fn pod_of(w: &MobyWorld, id: usize) -> Option<usize> { usize::try_from(w.mobys[id].pv_i(POD)).ok().filter(|&p| p < w.mobys.len()) }
fn pod_b(w: &MobyWorld, id: usize) -> bool { pod_of(w, id).is_some_and(|p| w.mobys[p].o_class == POD_B) }
/// Taking turns in a group (`+0x730` and its group's list).
fn in_turns(w: &MobyWorld, id: usize) -> bool {
    let m = &w.mobys[id];
    m.pv_i(TURNS) != 0 && usize::try_from(m.group as i32).ok().and_then(|g| w.groups.get(g)).is_some_and(|l| !l.is_empty())
}
/// `0x32fb10(m, p)` (L01 `0x322450`): the move to `p`; the flags.
fn go_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3) -> i32 {
    let yaw = yaw_to(w, id, p);
    ai::move_step(w, id, ctx, yaw, p)
}
/// Its floor path's points as the level holds them now.
fn path_points(w: &MobyWorld, id: usize, ctx: &Ctx) -> Vec<[f32; 4]> {
    path_network::points_now(ctx.level, &ctx.game.path_openings, w.mobys[id].pv_i(PATH)).map(|p| p.into_owned()).unwrap_or_default()
}
/// `0x32d318(r, from, to, &out, path)`: `to` kept in its floor path (none: `to`).
fn keep_in(pts: &[[f32; 4]], r: f32, from: V3, to: V3) -> V3 {
    if pts.is_empty() { return to; }
    path_network::keep_in(pts, r, from, to)
}
/// The stand-off's sequence: 4 or 5 at random (`0x31c450(2) + 4`, the game's second roll the one played).
fn swipe_seq(w: &MobyWorld, id: usize, ctx: &mut Ctx, frame: u8) {
    let a = ctx.game.rng.randi(2) as u8 + 4;
    if seq(w, id, ctx) == a { return; }
    let b = ctx.game.rng.randi(2) as u8 + 4;
    ctx.host.set_anim(&w.mobys[id], b, frame, 0x14);
}
/// The swipe (12) with its sequence.
fn swipe(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, frame: u8) {
    st(w, id, 12, -1);
    swipe_seq(w, id, ctx, frame);
}
/// The walk-in (7 sub 6, sequence 3 over 20).
fn walk_in(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    st(w, id, 7, 6);
    set_seq(w, id, ctx, 3, 0, 0x14);
}
/// Alerted (11 with `sub`), its slow limits, the wait 200, no turn.
fn alert(w: &mut MobyWorld, id: usize, sub: i32) {
    st(w, id, 11, sub);
    settle(w, id);
}
/// The slow limits, the wait 200, the movement's option 0x10.
fn settle(w: &mut MobyWorld, id: usize) {
    slow(w, id);
    let m = &mut w.mobys[id];
    m.set_pv_s(WAIT, 200);
    let o = m.pv_i(MV + 0x38);
    m.set_pv_i(MV + 0x38, o | 0x10);
}

/// `0x3e83f0(m)`: its turn in its group — true for the leader (always without a group). With no live leader it
/// stands (the election under way, its distance to its target the nearest); under way the leader settles it, a
/// nearer one takes it over.
fn leader(w: &mut MobyWorld, id: usize) -> bool {
    if w.mobys[id].group == -1 { return true; }
    let m = &w.mobys[id];
    let (p, s, best) = (m.pv_i(TURN), m.pv_i(SH_STATUS), m.pv_i(SH_BEST));
    let d = dist(m.pos, m.pv_v(TGT));
    let me = id as i32;
    let l = w.shared_i(p);
    let alive = usize::try_from(l).ok().and_then(|l| w.mobys.get(l)).is_some_and(|o| o.state != 0xfe && o.state != 0xfd);
    if l != -1 && alive {
        if w.shared_i(s) == 1 {
            if l == me {
                w.set_shared_i(s, 2);
                return true;
            }
            if d < f32::from_bits(w.shared_i(best) as u32) {
                w.set_shared_i(p, me);
                w.set_shared_i(best, d.to_bits() as i32);
                return false;
            }
        }
        return w.shared_i(s) == 2 && w.shared_i(p) == me;
    }
    w.set_shared_i(p, me);
    w.set_shared_i(s, 1);
    w.set_shared_i(best, d.to_bits() as i32);
    false
}

/// `0x3e8528(m)`: its turn given up when it has it (the election cleared).
fn release(w: &mut MobyWorld, id: usize) {
    let (p, s) = (w.mobys[id].pv_i(TURN), w.mobys[id].pv_i(SH_STATUS));
    if w.shared_i(p) == id as i32 {
        w.set_shared_i(p, -1);
        w.set_shared_i(s, 0);
    }
}

/// `0x3e8560(m)`: a point of its floor path at random, more than 3 across away (five tries; the last one else).
fn pick(w: &MobyWorld, id: usize, ctx: &mut Ctx, pts: &[[f32; 4]]) -> usize {
    let mut n = 0;
    for _ in 0..5 {
        n = ctx.game.rng.randi(pts.len() as i32).max(0) as usize;
        let p = pts.get(n).map_or([0.0; 3], |p| [p[0], p[1], p[2]]);
        if 3.0 < dist2(w.mobys[id].pos, p) { return n; }
    }
    n
}

/// `0x3e8618(m)`: its target in sight — the line to the target's point (flags 0) meets nothing, or the target's moby.
fn in_sight(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    match w.coll_line(ctx, m.pos, m.pv_v(TGT), 0, Some(id)) {
        None => true,
        Some(o) => o.moby.is_some() && o.moby == ai::link(m, T_MOBY),
    }
}

/// `0x32d410(r, h, from, to, &out, path)`: a side step from `from` across the way to `to` — `r` to the left and to
/// the right kept in the path (`h` in); between them more than `r` across, one of them by their shares; else the same
/// with points 12 away at 45°.
fn side_step(ctx: &mut Ctx, pts: &[[f32; 4]], r: f32, h: f32, from: V3, to: V3) -> V3 {
    let heading = atan2(to[0] - from[0], to[1] - from[1]);
    let pair = |a: f32, l: f32| {
        let (pa, pb) = (add_rot(heading, a), sub_rot(heading, a));
        let left = keep_in(pts, h, from, add([pa.cos() * l, pa.sin() * l, 0.0], from));
        let right = keep_in(pts, h, from, add([pb.cos() * l, pb.sin() * l, 0.0], from));
        (left, right, dist2(from, left), dist2(from, right))
    };
    let (left, right, dl, dr) = pair(f32::from_bits(0x3fc9_0fdc), r);
    if r < dl + dr {
        return if ctx.game.rng.randf(0.0, dl + dr) < dl { left } else { right };
    }
    let (left, right, dl, dr) = pair(f32::from_bits(0x3f49_0fdc), 12.0);
    if ctx.game.rng.randf(0.0, dl + dr) < dl { left } else { right }
}

/// `0x3ea060` (module docs): false when it was deleted.
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 && !setup(w, id, ctx) { return false; }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    let h = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
    let unfelt = h.is_some_and(|h| h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| matches!(s.o_class, 0x031a | 0x1256 | 0x1305)));
    if unfelt {
        w.clear_hit(id);
    } else if w.mobys[id].state != 1 {
        ai::take_damage(w, id, ctx, false);
    }
    let area = w.mobys[id].pv_i(AREA);
    let f = ai::find_target(w, id, ctx, 0.0, 0.0, area, None, false);
    let m = &mut w.mobys[id];
    m.set_pv_s(FOUND, f as i16);
    let t = m.pv_i(T_MOBY);
    m.set_pv_i(T_COPY, t);
    true
}

/// `0x3ea060`'s set-up (module docs): false when it is deleted.
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let scale = w.class(CLASS).map(|c| c.scale);
    let pod = w.mobys[id].pv_i(POD);
    let pod_z = usize::try_from(pod).ok().filter(|&p| p != 0).and_then(|p| w.mobys.get(p)).map(|p| p.pos[2]);
    let m = &mut w.mobys[id];
    if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
    if let Some(s) = scale { m.scale = s * 0.85; }
    if let Some(z) = pod_z { m.pos[2] = z + f32::from_bits(0x3fb3_3333); }
    let (p, rz) = (m.pos, m.rot[2]);
    m.set_pv_v(OUT, p);
    m.set_pv_f(MV, RADIUS);
    m.set_pv_i(0x6e8, 0);
    m.set_pv_i(0x6ec, 0);
    m.set_pv_f(OUT + 0xc, rz);
    // `0x2f0ea0`: 15 and 12.8 as bytes.
    for (k, v) in [(0x48, 15u8), (0x4a, 12), (0x70, 0xe), (0x71, 0xd), (0x76, 8), (0x29, 1), (0x75, 7), (0x77, 9), (0xd3, 0xc)] { m.set_pv_b(k, v); }
    m.set_pv_f(0x80, REACT_G * (1.0 / 60.0));
    m.set_pv_f(0x84, f32::from_bits(0x3b88_8889));
    m.set_pv_f(0x94, RADIUS + 0.1);
    for (k, v) in [(0xcc, 7u8), (0xcd, 0x14), (0xce, 4), (0xcf, 9), (0xd2, 6), (0xd5, 1)] { m.set_pv_b(k, v); }
    m.set_pv_i(0xa8, 3);
    for (k, v) in [(0x7c, 0x3eaa_aaabu32), (0x88, 0x3c08_8889), (0x8c, 0x3ab6_0b61), (0x9c, 0x3ecc_cccd)] { m.set_pv_f(k, f32::from_bits(v)); }
    let d = m.pv_b(0x34) as f32;
    m.set_pv_f(DAMAGE, d);
    m.set_pv_s(0x26, 0x3c);
    m.set_pv_f(0xb8, f32::from_bits(0x3c23_d70a));
    m.set_pv_f(0x90, RADIUS);
    let path = m.pv_i(PATH);
    m.set_pv_i(MV + 0x3c, path);
    m.set_pv_f(0xa0, 40.0);
    m.set_pv_f(MV + 0xc, STEP);
    m.set_pv_f(MV + 0x10, f32::from_bits(0x3fa7_8d37));
    m.set_pv_f(MV + 0x2c, f32::from_bits(0x3c36_0b61));
    let o = m.pv_i(MV + 0x38);
    m.set_pv_i(MV + 0x38, o | 0x10);
    m.set_pv_b(0x28, 3);
    m.set_pv_f(0x30, 1.0);
    m.set_pv_f(MV + 8, STEP);
    m.set_pv_s(0x98, 0);
    if m.pv_i(AREA) == -1 || path == -1 {
        w.delete(id);
        return false;
    }
    if (pod as u32).wrapping_add(1) < 2 { st(w, id, 3, 0); } else { st(w, id, 1, 5); }
    let m = &mut w.mobys[id];
    m.set_pv_i(TURNS, 1);
    let sh = m.pv_i(SH_LEADER);
    m.set_pv_i(TURN, sh);
    w.set_shared_i(sh, -1);
    // `0x32c910(m, path, +0x280, 32)` (L02 `0x32f8b0`).
    let mut nodes = path_network::read_nodes(&w.mobys[id].pvars, NODES, NODES_MAX);
    let n = path_network::build(ctx.level, path, &mut nodes);
    path_network::write_nodes(&mut w.mobys[id].pvars, NODES, &nodes[..n]);
    path_network::open(&mut ctx.game.path_openings, ctx.level, &nodes[..n]);
    w.mobys[id].set_pv_i(COUNT, n as i32);
    true
}

/// 0x0bcb (module docs).
pub fn update_0bcb(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !head(w, id, ctx) { return; }
    let t = tgt(w, id);
    let d = dist2(w.mobys[id].pos, t);
    let ang = yaw_to(w, id, t);
    if let Some(p) = pod_of(w, id).filter(|&p| w.mobys[p].o_class == POD_B) {
        ai::set_link(&mut w.mobys[p], 0xa4, Some(id));
    }
    if subs(w, id, ctx) { return; }
    if states(w, id, ctx, d, ang) { return; }
    let j = ctx.host.joint(&w.mobys[id], SINK_JOINT).unwrap_or(w.mobys[id].pos);
    let g = ai::ground_z(ctx, 1.0, j, 0);
    let m = &w.mobys[id];
    if g < 20.0 && m.sub_state != 3 && m.state != 9 && (m.state == 13 || m.state == 11) { st(w, id, 9, 7); }
}

/// The sub-states (module docs); true when it is gone.
fn subs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    match w.mobys[id].sub_state {
        5 => {
            let wait = w.mobys[id].pv_i(WAIT_FOR);
            let held = usize::try_from(wait).ok().filter(|&k| k != id).and_then(|k| w.mobys.get(k)).is_some_and(|o| o.state != 0xfe && o.state != 0xfd);
            if held { return false; }
            let Some(p) = pod_of(w, id) else {
                waiting(w, id, ctx);
                return false;
            };
            if !found(w, id) || !hatchling::pod_ready(w, ctx, p) { return false; }
            hatchling::pod_open(w, ctx, p);
            if w.mobys[id].state != 6 { st(w, id, 6, -1); }
            if w.mobys[p].o_class == POD_B {
                let has = w.class(CLASS).is_some_and(|c| c.has_collision);
                let rz = w.mobys[p].rot[2];
                let m = &mut w.mobys[id];
                m.mode &= 0xffbe;
                m.has_collision = has;
                m.rot[2] = rz;
            }
        }
        0 => waiting(w, id, ctx),
        2 => {
            if found(w, id) { w.mobys[id].set_pv_s(WAIT, 200); }
            if w.mobys[id].count_s(WAIT) {
                st(w, id, 5, 4);
                set_seq(w, id, ctx, 3, 0, 0x14);
                w.mobys[id].set_pv_s(WAIT, 0xdc);
            }
        }
        4 => {
            if found(w, id) { alert(w, id, 2); } else { wait_out(w, id, ctx); }
        }
        6 => {
            if leader(w, id) {
                let (a, b) = (seq_a(w, id, ctx), seq(w, id, ctx));
                if (a != 1 && b != 1) || wrapped(w, id, ctx) { alert(w, id, 2); }
            } else {
                let f = ctx.host.frame_time(&w.mobys[id]);
                if found(w, id) { w.mobys[id].set_pv_s(WAIT, 200); }
                if wrapped(w, id, ctx) && seq_a(w, id, ctx) == 3 { set_seq(w, id, ctx, 0xe, 0, BLEND); }
                if 20.0 <= f && seq_a(w, id, ctx) == 0xe { set_seq(w, id, ctx, 3, 0, BLEND); }
                if wrapped(w, id, ctx) && seq_a(w, id, ctx) == 1 { set_seq(w, id, ctx, 0xe, 0, BLEND); }
                if seq_a(w, id, ctx) == 1 {
                    let a = yaw_to(w, id, tgt(w, id));
                    turn(w, id, a, TURN_6);
                }
                wait_out(w, id, ctx);
            }
        }
        3 => return hurt(w, id, ctx),
        _ => {}
    }
    false
}

/// Sub 0 (module docs).
fn waiting(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !found(w, id) { return; }
    if !in_turns(w, id) || leader(w, id) {
        st(w, id, 15, 2);
    } else {
        st(w, id, 7, 6);
        set_seq(w, id, ctx, 1, 0, BLEND);
    }
    settle(w, id);
}

/// At the wait's end idle (3 sub 0, sequence 2 over 20).
fn wait_out(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !w.mobys[id].count_s(WAIT) { return; }
    set_seq(w, id, ctx, 2, 0, 0x14);
    st(w, id, 3, 0);
}

/// Sub 3 (module docs); true when it is gone.
fn hurt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].first_sub() {
        let s = ctx.host.anim(&w.mobys[id]).map_or(1.0, |a| a.speed);
        if s <= 0.0 || 1.0 < s { speed(w, id, ctx, 1.0); }
        ai::senses(&mut w.mobys[id], TURN_A * 8.0 * (1.0 / 3600.0), WALK_A * f32::from_bits(0x3e08_8889), f32::from_bits(TURN_6), f32::from_bits(TURN_6));
    }
    if w.mobys[id].pos[2] < KILL_Z {
        bolts::drop(w, id, 0, 3, ctx);
        w.delete(id);
        return true;
    }
    let r = ai::react(&w.mobys[id]);
    if 1.0 / 60.0 < w.mobys[id].pv_f(r + 0x88) { w.mobys[id].set_pv_f(r + 0x88, 1.0 / 60.0); }
    let fl = ai::flight(w, id, ctx);
    if fl & 1 != 0 {
        death_fade::start(w, id, ctx, 0.0, None);
        bolts::drop(w, id, 0, 3, ctx);
    }
    let f = ctx.host.frame_time(&w.mobys[id]);
    let a = seq_a(w, id, ctx);
    if a == 4 { strike(w, id, ctx, f); }
    if (a == 4 || seq(w, id, ctx) == 4) && f <= 13.0 {
        let t = tgt(w, id);
        go_to(w, id, ctx, t);
    }
    if fl & 2 != 0 && HURT_OVER <= f { st(w, id, 13, 2); }
    if seq_a(w, id, ctx) == 13 && HURT_ROAR <= f {
        set_seq(w, id, ctx, 4, 0, 8);
        ctx.hero.damage.invincible = 0;
    }
    false
}

/// The swing (module docs, 12): at frames 13..14 on one sequence, within 110° and its reach of its target and in
/// sight, the sphere at its joint.
fn strike(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, f: f32) {
    if !(13.0..=14.0).contains(&f) { return; }
    let Some(a) = ctx.host.anim(&w.mobys[id]) else { return };
    if a.seq_a != a.seq_b { return; }
    let t = tgt(w, id);
    if CONE <= rot_dist(yaw_to(w, id, t), w.mobys[id].rot[2]) || REACH <= dist(w.mobys[id].pos, t) { return; }
    if !in_sight(w, id, ctx) { return; }
    let j = if a.seq_a == 4 { JOINT_B } else { JOINT_A };
    let dmg = w.mobys[id].pv_f(DAMAGE);
    ai::attack_joint(w, id, ctx, SPHERE, dmg, 1.0, j, 0x1_0001, 0, 1, 0);
}

/// The states (module docs); true when it is gone.
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, d: f32, ang: f32) -> bool {
    match w.mobys[id].state {
        3 => {
            if w.mobys[id].first() { idle_seq(w, id, ctx); }
            let f = ctx.host.frame_time(&w.mobys[id]);
            let a = seq_a(w, id, ctx);
            if (a == 2 && wrapped(w, id, ctx)) || (a == 0xe && 20.0 <= f) { idle_seq(w, id, ctx); }
        }
        4 => {
            if w.mobys[id].count_i(T_OFF) { swipe(w, id, ctx, 0); }
        }
        6 => climb(w, id, ctx),
        8 => {
            if w.mobys[id].first() {
                let pts = path_points(w, id, ctx);
                let mut k = 0;
                while k < 5 {
                    let n = pick(w, id, ctx, &pts);
                    let m = &w.mobys[id];
                    let a = [m.pos[0], m.pos[1], m.pos[2] + 1.0];
                    let p = pts.get(n).copied().unwrap_or_default();
                    let b = [p[0], p[1], a[2]];
                    match w.coll_line(ctx, a, b, 0, Some(id)) {
                        None => {
                            w.mobys[id].set_pv_v(GOAL, b);
                            break;
                        }
                        Some(o) if 2.0 <= dist2(w.mobys[id].pos, o.point) => {
                            w.mobys[id].set_pv_v(GOAL, o.point);
                            break;
                        }
                        _ => {}
                    }
                    k += 1;
                }
                if k == 5 {
                    st(w, id, 3, -1);
                    return false;
                }
                set_seq(w, id, ctx, 0xc, 0, BLEND);
            }
            let g = w.mobys[id].pv_v(GOAL);
            let r = go_to(w, id, ctx, g);
            let a = yaw_to(w, id, g);
            turn(w, id, a, TURN_2);
            let stuck = w.mobys[id].pv_i(MV + 0x80);
            if r & 0x80 != 0 || 0xb <= stuck || r & 0x10f != 0 { st(w, id, 3, -1); }
        }
        9 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 7, 0, 0x14); }
            let m = &mut w.mobys[id];
            m.pos[2] -= f32::from_bits(0x3d4c_ccce);
            if m.pos[2] < 70.0 {
                bolts::drop(w, id, 0, 3, ctx);
                w.delete(id);
                return true;
            }
        }
        11 => alerted(w, id, ctx, d, ang),
        12 => {
            turn(w, id, ang, TURN_3);
            let f = ctx.host.frame_time(&w.mobys[id]);
            strike(w, id, ctx, f);
            if seq_a(w, id, ctx).wrapping_sub(4) < 2 && SWIPE_END <= f { st(w, id, 13, -1); }
        }
        13 => back_off(w, id, ctx, d, ang),
        15 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 1, 0, 0x14); }
            let a = yaw_to(w, id, tgt(w, id));
            turn(w, id, a, TURN_6);
            if wrapped(w, id, ctx) { st(w, id, 11, -1); }
        }
        _ => {}
    }
    false
}

/// State 3's sequence: 2 or 14 at random.
fn idle_seq(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = if ctx.game.rng.randi(2) == 0 { 2 } else { 0xe };
    set_seq(w, id, ctx, s, 0, BLEND);
}

/// State 6 (module docs).
fn climb(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        if pod_b(w, id) {
            speed(w, id, ctx, -1.0);
            set_seq(w, id, ctx, 10, 0, BLEND);
        } else {
            set_seq(w, id, ctx, 1, 0, BLEND);
        }
        slow(w, id);
        let z = ai::ground_z(ctx, 3.0, w.mobys[id].pos, 0);
        let m = &mut w.mobys[id];
        m.pos[2] = z;
        let p = m.pos;
        m.set_pv_v(OUT, p);
    }
    if wrapped(w, id, ctx) && seq_a(w, id, ctx) == 1 {
        speed(w, id, ctx, -1.0);
        set_seq(w, id, ctx, 10, 0, 0x14);
        let p = w.mobys[id].pos;
        go_to(w, id, ctx, p);
    }
    match seq_a(w, id, ctx) {
        1 => {
            let z = ai::ground_z(ctx, 3.0, w.mobys[id].pos, 0);
            w.mobys[id].pos[2] = z;
        }
        10 => {
            let rz = w.mobys[id].rot[2];
            let v = add(w.mobys[id].pos, [rz.cos() * 3.0, rz.sin() * 3.0, 0.0]);
            let z = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
            w.mobys[id].pos[2] = z;
            go_to(w, id, ctx, v);
            if dist2(w.mobys[id].pos, w.mobys[id].pv_v(OUT)) <= 4.9 { return; }
            if in_turns(w, id) {
                speed(w, id, ctx, 1.0);
                if !leader(w, id) {
                    walk_in(w, id, ctx);
                    return;
                }
            }
            st(w, id, 11, 2);
        }
        _ => {}
    }
}

/// State 11 (module docs).
fn alerted(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, d: f32, ang: f32) {
    if w.mobys[id].first() {
        set_seq(w, id, ctx, 10, 0, BLEND);
        speed(w, id, ctx, -BACK_SPEED);
        senses(w, id, TURN_A, BACK_SPEED * WALK_A, TURN_3);
    }
    if seq_a(w, id, ctx) != 10 { return; }
    if d < NEAR {
        let cam = ctx.host.camera_pos();
        let dodges = w.mobys[id].pv_i(DODGES);
        if dist2(cam, w.mobys[id].pos) < CAM_NEAR && dodges < DODGES_MOST {
            let t = tgt(w, id);
            let v = set_len(sub(t, [cam[0], cam[1], t[2]]), CAM_NEAR + CAM_NEAR);
            let v = add(cam, v);
            let pts = path_points(w, id, ctx);
            let v = keep_in(&pts, 3.0, w.mobys[id].pos, v);
            if CAM_NEAR + 0.3 < dist2(w.mobys[id].pos, v) {
                go_to(w, id, ctx, v);
                w.mobys[id].set_pv_i(DODGES, dodges + 1);
                return;
            }
        }
        w.mobys[id].set_pv_i(DODGES, 0);
        st(w, id, 12, -1);
        speed(w, id, ctx, 1.0);
        swipe_seq(w, id, ctx, 2);
        return;
    }
    let m = &w.mobys[id];
    let nodes = path_network::read_nodes(&m.pvars, NODES, (m.pv_i(COUNT).max(0) as usize).min(NODES_MAX));
    let (mut yaw, mut at, mut goal) = (0.0, m.pv_i(NET_AT), m.pv_i(NET_GOAL));
    let (p, t) = (m.pos, m.pv_v(TGT));
    path_network::steer(ctx.level, &nodes, 0.5, p, t, &mut yaw, &mut at, &mut goal);
    let m = &mut w.mobys[id];
    m.set_pv_i(NET_AT, at);
    m.set_pv_i(NET_GOAL, goal);
    ai::move_step(w, id, ctx, yaw, t);
    turn(w, id, ang, TURN_3);
    if 0x3d <= w.mobys[id].pv_i(MV + 0x80) {
        w.mobys[id].set_pv_i(T_OFF, OFF_TICKS);
        st(w, id, 4, -1);
        speed(w, id, ctx, 1.0);
        set_seq(w, id, ctx, 2, 0, 0x14);
    }
}

/// State 13 (module docs).
fn back_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, d: f32, ang: f32) {
    if w.mobys[id].first() {
        let pts = path_points(w, id, ctx);
        let p = w.mobys[id].pos;
        let a = add_rot(w.mobys[id].rot[2], f32::from_bits(0x4049_0fdc));
        let mut v = keep_in(&pts, 3.0, p, add([a.cos() * BACK, a.sin() * BACK, 0.0], p));
        if dist(p, v) < BACK * 0.5 {
            let t = tgt(w, id);
            v = side_step(ctx, &pts, SIDE, 1.0, p, t);
        }
        let (a2, b2) = ([p[0], p[1], p[2] + 0.5], [v[0], v[1], v[2] + 0.5]);
        if let Some(o) = w.coll_line(ctx, a2, b2, 2, None) { v = o.point; }
        w.mobys[id].set_pv_v(GOAL, v);
        set_seq(w, id, ctx, 10, 0, BLEND);
        senses(w, id, TURN_B, WALK_B, TURN_6);
        w.mobys[id].set_pv_i(T_BACK, BACK_TICKS);
        release(w, id);
        speed(w, id, ctx, BACK_SPEED);
    }
    let g = w.mobys[id].pv_v(GOAL);
    let r = go_to(w, id, ctx, g);
    let j = ctx.host.joint(&w.mobys[id], 3).unwrap_or(w.mobys[id].pos);
    let gz = ai::ground_z(ctx, 0.5, j, 0);
    if r & 0x80 != 0 || 9 < w.mobys[id].pv_i(MV + 0x80) || r & 0xf != 0 || gz < 10.0 {
        speed(w, id, ctx, 1.0);
        if d < NEAR + SLACK { swipe(w, id, ctx, 0); } else { walk_in(w, id, ctx); }
    }
    if w.mobys[id].count_i(T_BACK) && d < NEAR {
        st(w, id, 12, -1);
        speed(w, id, ctx, 1.0);
        swipe_seq(w, id, ctx, 0);
    }
    turn(w, id, ang, TURN_3);
}
