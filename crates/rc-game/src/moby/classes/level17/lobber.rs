//! The lobber **0x0bbf** ([`update_0bbf`], L17 `0x3e6900`, 5 placed; the same code as L12 `0x3e3838`, 5 placed: the
//! decomp's v2.00 `func_US2_L12_003E3838`, its helpers v1.01 `func_L12_003E3A88` / `003E1CB0` / `003E1EE8` /
//! `003E20F0` / `003E2370` / `003E2418` / `003E2448` at our L17 `0x3e7f28` / `0x3e6150` / `0x3e6388` / `0x3e6590` /
//! `0x3e6810` / `0x3e68b8` / `0x3e68e8`) and its glob **0x0ea5** ([`update_0ea5`], L17 `0x40d558`, the same code as
//! L12 / L13 / L16 / L30 `0x409230` / `0x40dfd8` / `0x3fcaa8` / `0x3fcab8`; fired by `0x40d7c0` = L12 `0x409498`,
//! [`fire`]): a creature that keeps to a path, turns to face Ratchet once he is in its
//! trigger area or in its sight, and lobs globs at him from its mouth (its joint 2); the members of a group take turns
//! (one slot, the shared word `+0x294`). Some come in along an entry path first. On the framework (crate::moby::ai):
//! health `+0x20`, reaction `+0x60` (its moves undone: it only stands hurt), target block `+0x130` (its moby `+0x140`),
//! movement `+0x170` (radius 1.25, height 2, steps 2 / 2, slope 180°, gravity 1/180, the turn left 30°, stop 0.3,
//! options 0x15).
//!
//! **Its pvars:** `+0x150` its trigger area (copied to `+0x154`), `+0x158` its path and `+0x164` its entry path (−1
//! none; the path −1: deleted at the set-up), `+0x168` the entry path's point it heads for (−1: onto the path's
//! first), `+0x16c` off its path (walking back to its point first), `+0x260` / `+0x262` the calm and attack timers
//! (s16), `+0x264` its point on the path, `+0x268` the next point and `+0x26c` the one it goes to (−1 none), `+0x270`
//! its target's moby (copied), `+0x274` the hurt's timer (s16), `+0x276` found (s16), `+0x278` hurt (1 hurt, 2 the
//! flight over, 3 Ratchet within 8, 5 killed), `+0x290` the times no point of its path was in sight, `+0x294` its
//! group's slot (a shared word, crate::moby::MobyWorld::shared_i: the index of the one attacking), `+0x298` in a group,
//! `+0x29c` / `+0x29e` the slot's timers (s16), `+0x2a0` set off by a hook (24 sub 4), `+0x2a4` kept dead (its uid's
//! count), `+0x2a8` its lowest height, `+0x2b0` the head aim.
//!
//! **Each tick:** kept dead and its uid gone or counted, deleted. In a scene (`0x1a8f00` 2): hidden without collision.
//! Out of sub 7 shown, targeted and with its class's collision. The head ([`head`], `0x3e7f28`): in state 0 the set-up
//! (the reaction: sequences 17 / 4 / 5 / 6, keys 1/6 / 1/15, gravities 1/144 / 1/240 / 1/240, bytes `+0xcc..+0xd6`;
//! its health block's `+0x10` 2.25; state 5 sub 0, sequence 0; at its entry path's first point, else its path's, with
//! an entry path 2 sub 7; its group's slot freed); drawn within 24 of the camera its shadow (range 0x16); hooked (and
//! `+0x2a0`) 24 sub 4; the frame's damage (`0x32a160`, crate::moby::ai::take_damage), its states kept: `+0x278` 1 (5
//! for a kill), the hurt's timer 150; at its end `+0x278` 0 (hurt: 50 more); the search (its area, `0x32dd28`);
//! within 16 across of its target and the line 0.1 up from it to the target's point (flags 0x10) clear or on Ratchet,
//! found; drawn, its glow pulsing between 0x80808080 and 0x80303030 once a second (`0x329040`, crate::hud::lerp_rgba);
//! its head aim (`0x329158` = L00 `0x327308` on `+0x2b0`, 1.8). Ratchet within 8 across while unhurt: `+0x278` 3
//! (its timer 50). Killed (5, or dead in the flight): its sound 3, a blast (`0x400940` = L00 `0x3f8348`: size 1, 25 /
//! 20 / 20, 20 puffs, 45 streaks, the light, the shake), its uid counted when kept dead, its bolts (limit 3), gone.
//! Hurt (1): the flight (`0x32aea8`) with its place and facing put back; over, `+0x278` 2.
//!
//! **Subs** (`+0x95`): **0** calm: found or hurt, its points cleared; in a group the slot taken ([`take_slot`]: free,
//! or its holder gone — 15 sub 5, the slot's timer 300), else waiting (4 sub 10, the reserve 800); alone 15 sub 5.
//! **5** watching (its first tick: walk most 12, limits 32/3600, 32/60, 0.1°, 12°, no turn): Ratchet near (3), the sub
//! 6 (its state kept); out of the group's turn (the slot's timer out), 9 with 60 ticks; else with the line from its
//! mouth (joint 2, 1 back) to the target's point (flags 2) blocked, 6; found or hurt keep the calm timer at 250, at its
//! end calm (5 sub 0, sequence 0); else facing its target 6° a tick. **6** finding a place (its first tick, the nearest
//! point of its path in sight, [`nearest`]): off its path the walk back ([`walk_back`]); with no point to go to, one
//! next to it (then the farther ones) with a clear line to the target's point, none: 5; the walk point by point the
//! shorter way round to it, facing its target; there, 5. **7** hidden until found (then 3 sub 8, shown). **9** out of
//! turn: from 23 back to 4 (sequence 3); at its timer's end the slot freed and 4 sub 10; hurt in 4, 15 sub 5 (the
//! slot's timer 150). **10** waiting its turn: the slot taken when free; hurt, 15 sub 5; calm as 5.
//!
//! **States** (`+0x20`): **3** coming in along its entry path ([`walk_entry`], 16 a second, limits 0.0089 / 0.533);
//! there 5 sub 0. **5** idle (sequence 0, the walk most 6; after 150 ticks the patrol: 21, sequence 1). **15** aiming
//! (sequence 8, its limits ÷8, the walk most 3, 40 ticks): smoke from its mouth ([`smoke`]) until the timer's end or,
//! within 4 across, its last 20 ticks; then the lob (16, sequence 7, its limits back, 20 ticks). **16** the lob (its
//! sound 3 on its first tick): until frame 6 the smoke; then a glob ([`fire`], its damage byte) from its mouth at
//! 0.533 a tick — facing more than 10° off its target along its facing, rising to 0.5 over the target's point, else
//! straight at that point; its rise at most 14/60 — 23 with 50 ticks. **21** the patrol to the next point of its path
//! (its height kept between the two points'); there 5 (200 ticks). **23** after the lob (sequence 3 under 30 ticks;
//! within 8 across under 35 ticks the next lob, 16, sequence 7; at the timer's end 15). **24** hooked (sequence 9;
//! frozen at frame 12.5).
//!
//! **The glob** ([`update_0ea5`]): along its velocity each tick; a line to its next place (flags 0, its hit — its
//! damage, flags 1 — on what it meets): the world, a 0.6 sphere of its damage pushed 3 its way (flags 0x10001); a moby
//! not its owner, the same pushed 1.5; its owner, on it goes; after 240 ticks a 0.6 sphere (`0x31e3a8`) all the same.
//! Then a blast (size 0.75, 3 / 3 / 3, 3 puffs, 3 streaks) and gone. Each tick it flies on, its draw is registered
//! ([`draw_0ea5`], `0x40d968`, the same code and data in L12 / L13 / L16: a bolt along its velocity, crate::fx::trails).
//!
//! The engine functions with the port's: `0x31c380` (set_state), `0x31c0a0` (set_anim, crate::moby::classes::props::
//! set_seq), `0x32d7b8` / `0x32d7e0` (the first tick of the state / sub), `0x330be8` = L00 `0x32dcf0` (crate::moby::
//! ai::senses), `0x32fb10` = L01 `0x322450` (the move to a point, crate::moby::ai::move_step), `0x320318` = L00
//! `0x31f030` (approach_angle), `0x31e048` (ground_z), `0x313228` = L00 `0x312728` (a joint's frame), `0x2df9b0` = L00
//! `0x2de878` (crate::moby::MobyWorld::coll_line), `0x325d28` = L00 `0x323bf8` (crate::moby::ai::reached),
//! `0x33d970` = L00 `0x33a6a0` (crate::particles::twinkle), `0x363c80` = L00 `0x3605f0` (its sound), `0x32d828` = L00
//! `0x32a838` (hooked: `once & 4`), `0x2ef908` (crate::moby::countdown), `0x31c450` / `0x31c4e8` (crate::rng),
//! `0x2e25e8` = L00 `0x2e14b0` (crate::moby::MobyWorld::sphere_hit), `0x31b870` (crate::moby::bolts::drop), `0x312188`
//! (delete).
//!
//! Not ported: the class word `+0x46` the set-up sets (5), the write `0x3e6388` makes into the level's path data (its
//! path's first point onto the ground 0.5 up when within 10 over it: the level's paths are read only here) and the
//! messages `0x2a5e70` prints. `0x3e6150`'s line to the point past its
//! path's end (the game reads the next path's header there) is taken as blocked.

use crate::hero::physics::{add, approach_angle, atan2, dist, dist2, len, rot_dist, scale, set_len, sub, V3};
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::hits::HitTemplate;
use crate::moby::{ai, blast, bolts, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0bbf;
/// Its glob.
pub const SHOT: i16 = 0x0ea5;

const TGT: usize = 0x130;
const T_MOBY: usize = 0x140;
const AREA: usize = 0x150;
const PATH: usize = 0x158;
const ENTRY: usize = 0x164;
const ENTRY_K: usize = 0x168;
const OFF_PATH: usize = 0x16c;
const MV: usize = 0x170;
const CALM: usize = 0x260;
const ATTACK: usize = 0x262;
const IDX: usize = 0x264;
const NEXT: usize = 0x268;
const DEST: usize = 0x26c;
const TARGET: usize = 0x270;
const HURT_T: usize = 0x274;
const FOUND: usize = 0x276;
const HURT: usize = 0x278;
const MISSES: usize = 0x290;
const SLOT: usize = 0x294;
const GROUPED: usize = 0x298;
const SLOT_T: usize = 0x29c;
const RESERVE: usize = 0x29e;
const HOOKABLE: usize = 0x2a0;
const KEPT: usize = 0x2a4;
const MIN_Z: usize = 0x2a8;
const HEAD_AIM: usize = 0x2b0;
const PVARS: usize = 0x330;

/// `$gp+0x1cb0`: the lob's frame; `+0x1cb4` its climb a second; `+0x1cb8` the smoke's colour.
const LOB_FRAME: f32 = 6.0;
const CLIMB: f32 = 0.5;
const SMOKE_RGBA: u32 = 0x8010_3080;
/// `$gp+0x1ce0` / `+0x1ce4` the smoke's size, `+0x1ce8` its life, `+0x1cec` (int) its spins; `+0x1cf0` the mouth's
/// point on joint 2.
const SMOKE_MIN: f32 = 2.0;
const SMOKE_MAX: f32 = 3.0;
const SMOKE_LIFE: f32 = 20.0;
const SMOKE_SPINS: i32 = 5;
const MOUTH: [f32; 4] = [-1.0, 0.0, 0.0, 1.0];
/// `$gp+0x1d08` / `+0x1d0c` its turn and walk accelerations (/3600, /60); `+0x1d10` its walk's most.
const TURN_A: f32 = 32.0;
const WALK_A: f32 = 32.0;
const WALK_MOST: f32 = 12.0;
/// `$gp+0x1d64`: the rise (a second) the flight's velocity is cleared at.
const RISE: f32 = 5.0;
/// `$gp+0x1d68`: the glob's rise at most (a second); `+0x1d6c` (s16) the out-of-turn ticks.
const LOB_RISE: f32 = 14.0;
const OUT_TICKS: i16 = 60;
/// `$gp+0x1d70`: the entry walk's speed (a second); `+0x1d74` the height over the ground it may keep.
const ENTRY_SPEED: f32 = 16.0;
const OVER: f32 = 4.0;
/// `$gp+0x1d94`: the lob's aim over the target; `+0x1dc4` its head's scale; `+0x1dc8` the sight line's rise.
const AIM_UP: f32 = 0.5;
const HEAD_SCALE: f32 = 1.8;
const SIGHT_UP: f32 = 0.1;
/// 6° / 0.1° / 12° a tick, 10°.
const TURN_6: u32 = 0x3dd6_7752;
const TURN_01: u32 = 0x3ae4_c38a;
const TURN_12: u32 = 0x3e56_7752;
const TEN: f32 = 0.174_532_95;
/// `0x40c00001`: the turn on the walks (at once).
const TURN_NOW: u32 = 0x40c0_0001;

fn st(w: &mut MobyWorld, id: usize, s: u8, sub: i32) { w.mobys[id].set_state(s, sub); }
fn seq_a(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a) }
fn hurt(w: &MobyWorld, id: usize) -> i32 { w.mobys[id].pv_i(HURT) }
fn found(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_s(FOUND) == 1 }
/// Its target's moby's place (`+0x270`; Ratchet's without one).
fn target_pos(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    ai::link(&w.mobys[id], TARGET).and_then(|k| w.mobys.get(k)).map_or(ctx.hero.pos, |t| t.pos)
}
fn yaw_to(w: &MobyWorld, id: usize, p: V3) -> f32 {
    let q = w.mobys[id].pos;
    atan2(p[0] - q[0], p[1] - q[1])
}
fn face(w: &mut MobyWorld, id: usize, ctx: &Ctx, rate: u32) {
    let yaw = yaw_to(w, id, target_pos(w, id, ctx));
    approach_angle(yaw, f32::from_bits(rate), &mut w.mobys[id].rot[2], 0);
}
fn senses(w: &mut MobyWorld, id: usize, a: f32, b: f32, c: u32, d: u32) { ai::senses(&mut w.mobys[id], a, b, f32::from_bits(c), f32::from_bits(d)); }
/// Its walking limits (sub 5, the lob).
fn walk_senses(w: &mut MobyWorld, id: usize) {
    w.mobys[id].set_pv_f(MV + 0x28, WALK_MOST);
    senses(w, id, TURN_A * (1.0 / 3600.0), WALK_A * (1.0 / 60.0), TURN_01, TURN_12);
}
fn opts(w: &mut MobyWorld, id: usize, clear: i32, set: i32) {
    let m = &mut w.mobys[id];
    let o = m.pv_i(MV + 0x38);
    m.set_pv_i(MV + 0x38, (o & !clear) | set);
}
/// A path's points (none: empty).
fn points(ctx: &Ctx, path: i32) -> Vec<[f32; 4]> {
    crate::path_network::points_now(ctx.level, &ctx.game.path_openings, path).map(|p| p.into_owned()).unwrap_or_default()
}
fn pt(pts: &[[f32; 4]], k: i32) -> V3 { usize::try_from(k).ok().and_then(|k| pts.get(k)).map_or([0.0; 3], |p| [p[0], p[1], p[2]]) }
/// The line from `a` to `b` (flags 2) clear.
fn clear(w: &MobyWorld, ctx: &Ctx, a: V3, b: V3) -> bool { w.coll_line(ctx, a, b, 2, None).is_none() }
/// Its mouth: joint 2's frame on (−1, 0, 0) (`0x313228`, `0x2f01a0`).
fn mouth(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 {
    let m = &w.mobys[id];
    let (r, t) = ctx.host.joint_matrix(m, 2).unwrap_or((m.rows, m.pos));
    let v = MOUTH;
    add(add(add(scale(r[0], v[0]), scale(r[1], v[1])), scale(r[2], v[2])), scale(t, v[3]))
}
/// The point one step from `idx` the shorter way round toward `dest` on a path of `n` points (the game's count of
/// the way round one short).
fn toward(idx: i32, dest: i32, n: i32) -> i32 {
    let mut fwd = dest - idx;
    if fwd < 0 { fwd += n - 1; }
    let mut bwd = idx - dest;
    if bwd < 0 { bwd += n - 1; }
    let mut k = if fwd < bwd { idx + 1 } else { idx - 1 };
    if k < 0 { k = n - 1; }
    if k == n { k = 0; }
    k
}

/// `0x3e6810(m)`: its group's slot taken when free (or its holder gone): 15 sub 5, the slot's timer 300.
fn take_slot(w: &mut MobyWorld, id: usize) {
    let s = w.mobys[id].pv_i(SLOT);
    let v = w.shared_i(s);
    let free = v == -1 || v == id as i32 || usize::try_from(v).ok().and_then(|v| w.mobys.get(v)).is_none_or(|o| o.state == 0xfe || o.state == 0xfd);
    if !free { return; }
    st(w, id, 15, 5);
    w.mobys[id].set_pv_s(SLOT_T, 300);
    w.set_shared_i(s, id as i32);
}

/// `0x3e68b8(m)`: its group's slot freed when it holds it.
fn free_slot(w: &mut MobyWorld, id: usize) {
    let s = w.mobys[id].pv_i(SLOT);
    if w.shared_i(s) == id as i32 { w.set_shared_i(s, -1); }
}

/// The step to point `k` of its path (`0x3e6590`'s, the patrol's and the place's): the move with no rise, its height
/// kept (in the air too high over the ground or without a floor surface, as it was; under its lowest, its lowest; with
/// `between`, at least the lower of where it was and the point's), then 0.5 a second toward the point's height when
/// more than 0.8 off it (not under the ground). Returns the move's flags.
fn step_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, pts: &[[f32; 4]], k: i32, between: bool, turn: bool) -> i32 {
    let old = w.mobys[id].pos[2];
    w.mobys[id].set_pv_f(MV + 0xa8, 0.0);
    let p = pt(pts, k);
    let yaw = yaw_to(w, id, p);
    let r = ai::move_step(w, id, ctx, yaw, p);
    if turn { face(w, id, ctx, TURN_NOW); }
    let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
    let m = &mut w.mobys[id];
    if r & 0x20 != 0 {
        if m.pv_i(MV + 0xd8) == 0 || OVER < m.pos[2] - g {
            m.pos[2] = old;
        } else if m.pos[2] < m.pv_f(MIN_Z) {
            m.pos[2] = m.pv_f(MIN_Z);
        }
        if between {
            let lo = if old < p[2] { old } else { p[2] };
            if m.pos[2] < lo { m.pos[2] = lo; }
        }
    }
    let rate = CLIMB * (1.0 / 60.0);
    if m.pos[2] < p[2] && 0.8 < p[2] - m.pos[2] {
        m.pos[2] += rate;
        if m.pos[2] < g { m.pos[2] = g; }
    }
    if p[2] < m.pos[2] && p[2] - m.pos[2] < -0.8 {
        m.pos[2] -= rate;
        if m.pos[2] < g { m.pos[2] = g; }
    }
    r
}

/// `0x3e6150(m)`: its point on its path — within 0.5 of it on it; within 5 and in sight, the walk back to it; else the
/// nearest point across (squared, within 0.5 on it), the one past the path's end in sight (taken as not: module docs)
/// or the nearest in sight, the walk back to it; none in sight, `+0x290` one more.
fn nearest(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let pts = points(ctx, w.mobys[id].pv_i(PATH));
    let pos = w.mobys[id].pos;
    let idx = w.mobys[id].pv_i(IDX);
    let d = dist2(pos, pt(&pts, idx));
    if d < 0.5 {
        w.mobys[id].set_pv_i(OFF_PATH, 0);
        return;
    }
    if d < 5.0 && clear(w, ctx, pos, pt(&pts, idx)) {
        w.mobys[id].set_pv_i(OFF_PATH, 1);
        return;
    }
    let sq = |p: V3| (pos[0] - p[0]) * (pos[0] - p[0]) + (pos[1] - p[1]) * (pos[1] - p[1]);
    let (mut best, mut bd) = (-1, 1000.0f32);
    for k in 0..pts.len() as i32 {
        let e = sq(pt(&pts, k));
        if e < bd {
            bd = e;
            best = k;
        }
    }
    if bd < 0.5 {
        let m = &mut w.mobys[id];
        m.set_pv_i(IDX, best);
        m.set_pv_i(OFF_PATH, 0);
        return;
    }
    // The line to point `n` (past the end): taken as blocked.
    let (mut best, mut bd) = (-1, 1000.0f32);
    for k in 0..pts.len() as i32 {
        if !clear(w, ctx, pos, pt(&pts, k)) { continue; }
        let e = sq(pt(&pts, k));
        if e < bd {
            bd = e;
            best = k;
        }
    }
    let m = &mut w.mobys[id];
    if best == -1 {
        m.set_pv_i(OFF_PATH, 0);
        let n = m.pv_i(MISSES);
        m.set_pv_i(MISSES, n + 1);
        return;
    }
    m.set_pv_i(IDX, best);
    m.set_pv_i(OFF_PATH, 1);
}

/// `0x3e6590(m)`: the walk back to its point (facing its target in sub 6); there, on its path.
fn walk_back(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pts = points(ctx, w.mobys[id].pv_i(PATH));
    let (k, turn) = (w.mobys[id].pv_i(IDX), w.mobys[id].sub_state == 6);
    let r = step_to(w, id, ctx, &pts, k, false, turn);
    if r & 0x80 != 0 { w.mobys[id].set_pv_i(OFF_PATH, 0); }
}

/// `0x3e6388(m, entry)`: the walk along its entry path, 16 a second straight along its segments (the last one onto
/// its path's first point); true once it is there.
fn walk_entry(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    let (patrol, entry) = (points(ctx, m.pv_i(PATH)), points(ctx, m.pv_i(ENTRY)));
    let k = m.pv_i(ENTRY_K);
    let n = entry.len() as i32;
    let goal = if 0 < k { pt(&entry, k) } else { pt(&patrol, 0) };
    let mut d = if k < 1 { sub(pt(&patrol, 0), pt(&entry, n - 1)) } else { sub(pt(&entry, k), pt(&entry, k - 1)) };
    let sp = ENTRY_SPEED * (1.0 / 60.0);
    if sp < len(d) { d = set_len(d, sp); }
    let m = &mut w.mobys[id];
    m.pos = add(m.pos, d);
    if dist(m.pos, goal) <= 0.5 {
        if k == -1 { return true; }
        // The last point: onto the path (its first point's write into the level's paths not ported).
        m.set_pv_i(ENTRY_K, if k == n - 1 { -1 } else { k + 1 });
    }
    false
}

/// The smoke at its mouth (`0x33d970`: 2..3 across, a tenth of it to start, 20 ticks, `$gp+0x1cb8`'s colour).
fn smoke(ctx: &mut Ctx, at: V3) {
    let f = ctx.game.rng.randf(SMOKE_MIN, SMOKE_MAX);
    let spin = ctx.game.rng.randi(SMOKE_SPINS);
    if let Some(sys) = ctx.parts.as_deref_mut() {
        crate::particles::twinkle(sys, &mut ctx.game.rng, f * 0.3, f, 0.0, at, SMOKE_LIFE as i16, SMOKE_RGBA, 0, spin as i8, [0.0; 3], 0);
    }
}

/// Killed: its sound 3, the blast, its uid counted (kept dead), its bolts, gone.
fn killed(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.host.play_sound(&w.mobys[id], 3, 0);
    let pos = w.mobys[id].pos;
    blast::burst(w, ctx, pos, &blast::Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true });
    if w.mobys[id].pv_i(KEPT) != 0 { ctx.game.count_uid(w.mobys[id].uid); }
    bolts::drop(w, id, 0, 3, ctx);
    w.delete(id);
}

/// 0x0bbf (module docs).
pub fn update_0bbf(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(KEPT) != 0 {
        let uid = w.mobys[id].uid;
        let gone = ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&b| b != 0);
        if gone || ctx.game.uid_count(uid) != 0 { return w.delete(id); }
    }
    if ctx.game.scenes.mode == crate::scene::SCENE {
        let m = &mut w.mobys[id];
        m.has_collision = false;
        m.mode = (m.mode & 0xefff) | 0x41;
        return;
    }
    if w.mobys[id].sub_state != 7 {
        let has = w.class(CLASS).is_some_and(|c| c.has_collision);
        let m = &mut w.mobys[id];
        m.mode = (m.mode & 0xffbe) | mode::TARGET;
        m.has_collision = has;
    }
    if !head(w, id, ctx) { return; }
    let d = dist2(target_pos(w, id, ctx), w.mobys[id].pos);
    if d < 8.0 && hurt(w, id) == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_i(HURT, 3);
        m.set_pv_s(HURT_T, 0x32);
    }
    match hurt(w, id) {
        5 => return killed(w, id, ctx),
        1 => {
            let (pos, rz) = (w.mobys[id].pos, w.mobys[id].rot[2]);
            let r = ai::flight(w, id, ctx);
            if RISE * (1.0 / 60.0) <= w.mobys[id].pv_f(MV + 0xa8) { w.mobys[id].set_pv_f(MV + 0xa8, 0.0); }
            if r & 1 != 0 { return killed(w, id, ctx); }
            if r & 2 != 0 { w.mobys[id].set_pv_i(HURT, 2); }
            let m = &mut w.mobys[id];
            m.pos = pos;
            m.rot[2] = rz;
        }
        _ => {}
    }
    subs(w, id, ctx);
    states(w, id, ctx, d);
}

/// `0x3e7f28` (module docs): false when it was deleted.
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 && !setup(w, id, ctx) { return false; }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if w.mobys[id].pv_i(HOOKABLE) != 0 && w.mobys[id].once & 4 != 0 { st(w, id, 0x18, 4); }
    let (s, sub) = (w.mobys[id].state, w.mobys[id].sub_state);
    let k = ai::take_damage(w, id, ctx, false);
    if k != 0 {
        let m = &mut w.mobys[id];
        m.set_pv_i(HURT, if k == 4 { 5 } else { 1 });
        m.state = s;
        m.sub_state = sub;
        m.first();
        m.first_sub();
        m.set_pv_s(HURT_T, 0x96);
        m.set_pv_s(0x26, 0);
    }
    if w.mobys[id].count_s(HURT_T) {
        let m = &mut w.mobys[id];
        if m.pv_i(HURT) == 1 { m.set_pv_s(HURT_T, 0x32); } else { m.set_pv_i(HURT, 0); }
    }
    let area = w.mobys[id].pv_i(AREA);
    let f = ai::find_target(w, id, ctx, 6.0, 0.0, area, None, false);
    let m = &mut w.mobys[id];
    m.set_pv_s(FOUND, f as i16);
    let t = m.pv_i(T_MOBY);
    m.set_pv_i(TARGET, t);
    if let Some(tp) = ai::link(&w.mobys[id], TARGET).and_then(|k| w.mobys.get(k)).map(|t| t.pos) {
        let m = &w.mobys[id];
        if dist2(tp, m.pos) < 16.0 {
            let p = [m.pos[0], m.pos[1], m.pos[2] + SIGHT_UP];
            let seen = match w.coll_line(ctx, p, m.pv_v(TGT), 0x10, None) {
                None => true,
                Some(o) => o.moby.is_some() && o.moby == ctx.hero_moby,
            };
            if seen { w.mobys[id].set_pv_s(FOUND, 1); }
        }
    }
    if w.mobys[id].drawn != 0 {
        let t = (ctx.game.frame % 60) as f32;
        let t = t * std::f32::consts::TAU / 60.0 - std::f32::consts::PI;
        w.mobys[id].glow = crate::hud::lerp_rgba(t.sin() * 0.5 + 0.5, 0x8080_8080, 0x8030_3030);
    }
    crate::moby::joint_aim::head(w, ctx, id, HEAD_AIM, 1, HEAD_SCALE);
    true
}

/// `0x3e7f28`'s set-up (module docs): false when it is deleted.
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let m = &mut w.mobys[id];
    if m.pvars.len() < PVARS { m.pvars.resize(PVARS, 0); }
    m.mode |= mode::TARGET;
    m.set_pv_f(0x30, 2.25);
    // `0x2f0ea0`: 20 and 24 as bytes.
    for (k, v) in [(0x48, 20u8), (0x4a, 24), (0x71, 4), (0x72, 5), (0x76, 6), (0xcd, 0x1a), (0xce, 8), (0xcf, 0xb), (0xd2, 10), (0xd3, 0x11), (0x70, 0x11), (0xcc, 0xb), (0xd5, 1)] { m.set_pv_b(k, v); }
    for (k, v) in [(0x7c, 0x3e2a_aaabu32), (0x80, 0x3d88_8889), (0x88, 0x3be3_8e39), (0x8c, 0x3b88_8889), (0x94, 0x3f00_0000), (0x9c, 0x3ecc_cccd), (0x84, 0x3b88_8889), (0x90, 0x3f00_0000)] { m.set_pv_f(k, f32::from_bits(v)); }
    m.set_pv_i(0xa8, 3);
    if m.pv_i(PATH) == -1 {
        w.delete(id);
        return false;
    }
    let m = &mut w.mobys[id];
    let o = m.pv_i(MV + 0x38);
    m.set_pv_i(MV + 0x38, o | 0x15);
    m.set_pv_b(0x28, 3);
    for (k, v) in [(0x34, 0x3e99_999au32), (0, 0x3fa0_0000), (0xc, 0x4000_0000), (0x10, 0x4049_0fdc), (0x30, 0x3f06_0a93), (0x2c, 0x3bb6_0b61), (4, 0x4000_0000), (8, 0x4000_0000)] { m.set_pv_f(MV + k, f32::from_bits(v)); }
    m.set_pv_b(0xd6, 1);
    st(w, id, 5, 0);
    set_seq(w, id, ctx, 0, 0, 0);
    let m = &mut w.mobys[id];
    let a = m.pv_i(AREA);
    m.set_pv_i(AREA + 4, a);
    let entry = m.pv_i(ENTRY);
    let from = if entry == -1 { m.pv_i(PATH) } else { entry };
    let p = pt(&points(ctx, from), 0);
    w.mobys[id].pos = p;
    if entry != -1 { st(w, id, 2, 7); }
    let s = w.mobys[id].pv_i(SLOT);
    w.set_shared_i(s, -1);
    true
}

/// The sub-states (module docs).
fn subs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].sub_state {
        0 => {
            if !(found(w, id) || hurt(w, id) == 1 || hurt(w, id) == 2) { return; }
            let m = &mut w.mobys[id];
            m.set_pv_i(DEST, -1);
            m.set_pv_i(NEXT, -1);
            if m.pv_i(GROUPED) != 0 {
                take_slot(w, id);
                if w.mobys[id].state != 15 {
                    st(w, id, 4, 10);
                    w.mobys[id].set_pv_s(RESERVE, 800);
                }
            } else {
                st(w, id, 15, 5);
            }
        }
        5 => {
            if w.mobys[id].first_sub() {
                walk_senses(w, id);
                opts(w, id, 0, 0x10);
            }
            if hurt(w, id) == 3 {
                let s = w.mobys[id].state;
                if w.mobys[id].sub_state != 15 { st(w, id, s, 6); }
                w.mobys[id].first();
            }
            let out_of_turn = w.mobys[id].pv_i(GROUPED) != 0 && w.mobys[id].count_s(SLOT_T) && hurt(w, id) != 1;
            if out_of_turn {
                // `0x3e68e8`.
                w.mobys[id].sub_state = 9;
                w.mobys[id].set_pv_s(RESERVE, OUT_TICKS);
                return;
            }
            let p = mouth(w, id, ctx);
            if w.mobys[id].sub_state != 15 {
                let t = w.mobys[id].pv_v(TGT);
                if w.coll_line(ctx, p, t, 2, Some(id)).is_some() {
                    let s = w.mobys[id].state;
                    st(w, id, s, 6);
                    w.mobys[id].first();
                }
            }
            if calm(w, id, ctx) { return; }
            face(w, id, ctx, TURN_6);
        }
        6 => place(w, id, ctx),
        7 => {
            let m = &mut w.mobys[id];
            if m.pv_s(FOUND) == 1 {
                m.set_state(3, 8);
                m.drawn = 1;
                m.mode &= 0xfffe;
            } else {
                m.drawn = 0;
                m.mode |= 1;
            }
        }
        9 => {
            w.mobys[id].first_sub();
            if w.mobys[id].state == 0x17 && hurt(w, id) != 1 {
                st(w, id, 4, -1);
                set_seq(w, id, ctx, 3, 0, 0x14);
            }
            if w.mobys[id].count_s(RESERVE) {
                free_slot(w, id);
                st(w, id, 4, 10);
                set_seq(w, id, ctx, 3, 0, 0x14);
            } else if hurt(w, id) == 1 && w.mobys[id].state == 4 {
                st(w, id, 15, 5);
                w.mobys[id].set_pv_s(SLOT_T, 0x96);
            }
        }
        10 => {
            if w.mobys[id].first_sub() { nearest(w, id, ctx); }
            if wrapped(w, id, ctx) && seq_a(w, id, ctx) != 2 && seq(w, id, ctx) != 2 { ctx.host.set_anim(&w.mobys[id], 2, 0, 0x14); }
            take_slot(w, id);
            if hurt(w, id) == 1 {
                st(w, id, 15, 5);
                w.mobys[id].set_pv_s(SLOT_T, 0x96);
                return;
            }
            calm(w, id, ctx);
        }
        _ => {}
    }
}

/// Found or hurt keep the calm timer at 250; at its end, neither, calm (5 sub 0, sequence 0): true then.
fn calm(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if found(w, id) || hurt(w, id) != 0 { w.mobys[id].set_pv_s(CALM, 0xfa); }
    if found(w, id) || hurt(w, id) != 0 || !w.mobys[id].count_s(CALM) { return false; }
    set_seq(w, id, ctx, 0, 0, 0x14);
    st(w, id, 5, 0);
    true
}

/// Sub 6 (module docs).
fn place(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first_sub() {
        w.mobys[id].set_pv_i(DEST, -1);
        nearest(w, id, ctx);
    }
    if w.mobys[id].pv_i(OFF_PATH) != 0 { return walk_back(w, id, ctx); }
    let pts = points(ctx, w.mobys[id].pv_i(PATH));
    let n = pts.len() as i32;
    let idx = w.mobys[id].pv_i(IDX);
    if w.mobys[id].pv_i(NEXT) == -1 && w.mobys[id].pv_i(DEST) == -1 {
        let t = w.mobys[id].pv_v(TGT);
        let sees = |w: &MobyWorld, k: i32| clear(w, ctx, pt(&pts, k), t);
        let mut i = idx - 1;
        let mut nx = idx + 1;
        if i < 0 { i = n - 1; }
        if nx == n { nx = 0; }
        let mut dest = -1;
        if 0 <= i && sees(w, i) { dest = i; }
        if dest == -1 && nx < n && sees(w, nx) { dest = nx; }
        if dest == -1 { dest = (0..i).rev().find(|&j| sees(w, j)).unwrap_or(-1); }
        if dest == -1 { dest = (nx + 1..n).find(|&j| sees(w, j)).unwrap_or(-1); }
        if dest == -1 {
            let s = w.mobys[id].state;
            st(w, id, s, 5);
            w.mobys[id].first();
            return;
        }
        let m = &mut w.mobys[id];
        m.set_pv_i(DEST, dest);
        m.set_pv_i(NEXT, toward(idx, dest, n));
    }
    let next = w.mobys[id].pv_i(NEXT);
    face(w, id, ctx, TURN_NOW);
    let r = step_to(w, id, ctx, &pts, next, false, false);
    if r & 0x80 == 0 { return; }
    let dest = w.mobys[id].pv_i(DEST);
    let m = &mut w.mobys[id];
    m.set_pv_i(IDX, next);
    if dest != next {
        m.set_pv_i(NEXT, toward(next, dest, n));
        return;
    }
    m.set_pv_i(NEXT, -1);
    let s = m.state;
    st(w, id, s, 5);
    w.mobys[id].first();
}

/// The states (module docs).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, d: f32) {
    match w.mobys[id].state {
        3 => {
            if w.mobys[id].first() {
                senses(w, id, f32::from_bits(0x3c11_a2b4), f32::from_bits(0x3f08_8889), TURN_6, TURN_6);
                w.mobys[id].set_pv_i(ENTRY_K, 1);
                opts(w, id, 0x10, 1);
            }
            if walk_entry(w, id, ctx) {
                st(w, id, 5, 0);
                set_seq(w, id, ctx, 0, 0, 0);
                opts(w, id, 0, 0x10);
            }
        }
        5 => {
            if w.mobys[id].first() {
                opts(w, id, 0x10, 1);
                w.mobys[id].set_pv_f(MV + 0x28, WALK_MOST * 0.5);
                set_seq(w, id, ctx, 0, 0, 0x14);
                w.mobys[id].set_pv_s(ATTACK, 0x96);
            }
            if w.mobys[id].count_s(ATTACK) {
                set_seq(w, id, ctx, 1, 0, 0x14);
                st(w, id, 0x15, -1);
            }
        }
        15 => {
            if w.mobys[id].first() {
                w.mobys[id].set_pv_s(ATTACK, 0x28);
                set_seq(w, id, ctx, 8, 0, 0x14);
                senses(w, id, TURN_A * 3.472_222_3e-5, WALK_A * 0.002_083_333_4, TURN_6, TURN_6);
                w.mobys[id].set_pv_f(MV + 0x28, WALK_MOST * 0.25);
            }
            let out = w.mobys[id].count_s(ATTACK);
            if !out && (4.0 <= d || 0x13 < w.mobys[id].pv_s(ATTACK)) {
                let p = mouth(w, id, ctx);
                return smoke(ctx, p);
            }
            walk_senses(w, id);
            set_seq(w, id, ctx, 7, 0, 0x14);
            w.mobys[id].set_pv_s(ATTACK, 0x14);
            st(w, id, 0x10, -1);
        }
        0x10 => lob(w, id, ctx),
        0x15 => {
            let pts = points(ctx, w.mobys[id].pv_i(PATH));
            if w.mobys[id].first() {
                let n = pts.len() as i32;
                let k = w.mobys[id].pv_i(IDX) + 1;
                w.mobys[id].set_pv_i(NEXT, if k == n { 0 } else { k });
                nearest(w, id, ctx);
            }
            if w.mobys[id].pv_i(OFF_PATH) != 0 { walk_back(w, id, ctx); }
            let next = w.mobys[id].pv_i(NEXT);
            let r = step_to(w, id, ctx, &pts, next, true, false);
            if r & 0x80 != 0 {
                let m = &mut w.mobys[id];
                m.set_pv_s(ATTACK, 200);
                m.set_pv_i(IDX, next);
                set_seq(w, id, ctx, 0, 0, 0x14);
                st(w, id, 5, -1);
            }
        }
        0x17 => {
            if w.mobys[id].count_s(ATTACK) { st(w, id, 15, -1); }
            if w.mobys[id].pv_s(ATTACK) < 0x1e && seq_a(w, id, ctx) != 3 && seq(w, id, ctx) != 3 { ctx.host.set_anim(&w.mobys[id], 3, 0, 0x14); }
            let t = w.mobys[id].pv_v(TGT);
            if 8.0 <= dist2(t, w.mobys[id].pos) || 0x22 < w.mobys[id].pv_s(ATTACK) { return; }
            set_seq(w, id, ctx, 7, 0, 0x14);
            st(w, id, 0x10, -1);
        }
        0x18 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 9, 0, 0x14); }
            if ai::reached(w, id, ctx, 12.5) { ctx.host.set_anim_speed(&w.mobys[id], 0.0); }
        }
        _ => {}
    }
}

/// State 16 (module docs).
fn lob(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 3, 0); }
    let p = mouth(w, id, ctx);
    let t = w.mobys[id].pv_v(TGT);
    let yaw = yaw_to(w, id, t);
    if !ai::reached(w, id, ctx, LOB_FRAME) { return smoke(ctx, p); }
    let m = &w.mobys[id];
    let mut v = if TEN < rot_dist(m.rot[2], yaw) {
        let v = set_len(m.rows[0], f32::from_bits(0x3f08_8889));
        [v[0], v[1], (t[2] - p[2] + AIM_UP) * (1.0 / 60.0)]
    } else {
        set_len(sub([t[0], t[1], t[2] + AIM_UP], p), f32::from_bits(0x3f08_8889))
    };
    let most = LOB_RISE * (1.0 / 60.0);
    if most < v[2] { v[2] = most; } else if v[2] < -100.0 { v[2] = -100.0; }
    let dmg = w.mobys[id].pv_b(0x34) as f32;
    fire(w, ctx, dmg, id, v, p);
    w.mobys[id].set_pv_s(ATTACK, 0x32);
    st(w, id, 0x17, -1);
}

/// The glob's pvars: `+0x00` its velocity, `+0x10` its owner (a link), `+0x14` its damage, `+0x20` 0.15, `+0x24` its
/// ticks.
const S_VEL: usize = 0;
const S_OWNER: usize = 0x10;
const S_DAMAGE: usize = 0x14;
const S_TICKS: usize = 0x24;

/// `0x40d7c0(damage, owner, vel, pos)` (L12 `0x408050`): a glob at `pos` moving `vel` a tick: update and draw
/// distances 0xff, drawn, the owner's colour words and facing, alpha 0x80, 1/10000 of its size, modes 0x308; two
/// twinkles at it (8..10: 0.3 of it to start, 15 ticks, `0x80103080` spinning 10; 0.04 / 0.4 of it, white, the other
/// way).
pub fn fire(w: &mut MobyWorld, ctx: &mut Ctx, damage: f32, owner: usize, vel: V3, pos: V3) -> usize {
    let k = w.create(SHOT);
    let (light, ambient, rot) = (w.mobys[owner].light, w.mobys[owner].ambient, w.mobys[owner].rot);
    let m = &mut w.mobys[k];
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.state = 0;
    m.light = light;
    m.ambient = ambient;
    m.pos = pos;
    m.set_pv_v(S_VEL, vel);
    m.set_pv_f(S_DAMAGE, damage);
    m.set_pv_f(0x20, f32::from_bits(0x3e19_999a));
    ai::set_link(m, S_OWNER, Some(owner));
    m.mode |= 0x308;
    m.rot = rot;
    m.alpha = 0x80;
    m.scale = f32::from_bits(0x38d1_b717);
    m.set_pv_i(S_TICKS, 0);
    w.rebuild(k, &*ctx.host);
    let f = ctx.game.rng.randf(8.0, 10.0);
    if let Some(sys) = ctx.parts.as_deref_mut() {
        crate::particles::twinkle(sys, &mut ctx.game.rng, f * 0.3, f, 0.0, pos, 0xf, 0x8010_3080, 0, 10, [0.0; 3], 0);
        crate::particles::twinkle(sys, &mut ctx.game.rng, f * 0.04, f * 0.4, 0.0, pos, 0xf, 0xffff_ffff, 1, -10, [0.0; 3], 0);
    }
    k
}

/// 0x0ea5 (`0x40d558`, module docs).
pub fn update_0ea5(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let t = m.pv_i(S_TICKS) + 1;
    m.set_pv_i(S_TICKS, t);
    let dmg = m.pv_f(S_DAMAGE);
    if 0xf0 < t {
        let pos = w.mobys[id].pos;
        ai::attack_sphere(w, id, ctx, f32::from_bits(0x3f19_999a), dmg, 0.0, pos, 0x1_0001, 0, 1, 0);
        st(w, id, 1, -1);
    }
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (pos, vel, owner) = (m.pos, m.pv_v(S_VEL), ai::link(m, S_OWNER));
            let next = add(pos, vel);
            let mut hit = HitTemplate::new(dmg, Some(id), 1, [vel[0], vel[1], vel[2], 0.0]);
            hit.kind = (SHOT as u16 as u32) << 16;
            if let Some(o) = w.coll_line(ctx, pos, next, 0, Some(id)) {
                if let Some(t) = o.moby { w.deliver_hit_any(t, &hit, [o.point[0], o.point[1], o.point[2], 1.0]); }
                let push = match o.moby {
                    None => Some(3.0),
                    Some(t) if Some(t) != owner => Some(1.5),
                    _ => None,
                };
                if let Some(s) = push {
                    let d = scale(vel, s);
                    let mut h = HitTemplate::new(dmg, Some(id), 0x1_0001, [d[0], d[1], 1.0, 5_627.925]);
                    h.kind = (SHOT as u16 as u32) << 16 | 1 << 8;
                    w.sphere_hit(ctx.scene, pos, f32::from_bits(0x3f19_999a), 0, Some(id), Some(&h));
                    st(w, id, 1, -1);
                    return;
                }
            }
            w.mobys[id].pos = next;
            // `0x2e89d0(0x40d968, m)`.
            w.register_draw(id);
        }
        1 => {
            let pos = w.mobys[id].pos;
            blast::burst(w, ctx, pos, &blast::Blast { size: 0.75, bursts: [3; 3], puffs: 3, streaks: 3, light: false, shake: false });
            w.delete(id);
        }
        _ => {}
    }
}

/// The glob's bolt (`0x40dad8`'s words, `$gp+0x2820..+0x28a0` and `0x1b18a4`; the same in L12 / L13 / L16): a white
/// head 10 back fading to 0x80204080 (FX 0x35, then the tail to nothing), grey flares (FX 0x38, turning 10° a call)
/// and a blue glow (FX 0x31, 0.75).
const BOLT: crate::fx::trails::Bolt = {
    use crate::fx::trails::{Bolt, Flare};
    Bolt {
        fx: [0x35, 0x38],
        head: [0xffff_ffff, 0x8020_4080],
        length: 10.0,
        width: 0.2,
        from: -0.0,
        turn: 10.0,
        flares: [
            Flare { rgba: 0x5a80_8080, width: 0.2, from: -0.0, length: 4.0 },
            Flare { rgba: 0x5a80_8080, width: 0.1, from: 0.75, length: 4.0 },
            Flare { rgba: 0x3c80_8080, width: 0.4, from: 0.75, length: 4.0 },
            Flare { rgba: 0x64ff_ffff, width: 0.2, from: 0.0, length: 0.6 },
        ],
        flare_end: 0x80_8080,
        glow: (0x31, 0.75, 0x2020_4080),
    }
};

/// `0x40d968`: the glob's draw, its bolt along its velocity (crate::fx::trails::shot_bolt: the glob's pvars `+0x00`
/// its velocity, `+0x1c` the flares' spin, `+0x28` the scroll).
pub fn draw_0ea5(w: &mut MobyWorld, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    crate::fx::trails::shot_bolt(w, id, &BOLT, rng, cam, out);
}
