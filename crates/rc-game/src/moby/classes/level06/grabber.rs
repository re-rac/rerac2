//! Level 6's water creature that grabs Ratchet, **0x0fe6** (L06 `0x45bbc8`, the decomp's `func_L06_0045A5B0`, 5
//! placed, each alone in its group): it swims a polygon of the liquid grid's water, wades and bites in the shallows,
//! and in deep water takes Ratchet swimming near it — he dies in its jaws. It takes no damage (its health block's
//! function, L00 `0x410d18`, is empty).
//!
//! Its pvars: `+0x00` its health block (`+0x20`; `+0x29` set 1), `+0x48` / `+0x4a` two bytes the bite sets (8, 9;
//! nothing here reads them), its colour flash `+0x50` (crate::moby::ai::flash), the knock block `+0x60`
//! (crate::moby::knock), the velocity `+0xc0` (a tick), its place at the set-up `+0xd0`, home `+0xe0` (cuboid
//! `+0x104`'s place), the grab's start `+0xf0`, the polygon it swims in `+0x100` and the one it hunts in `+0x134` (the
//! level's paths), the cuboid `+0x104` it rests in, the liquid grid moby `+0x108` (crate::water::grid: its height the
//! water's, its `+0x14` frozen), the yaw and pitch speeds `+0x110` / `+0x114`, the ground under it `+0x120`, a timer
//! `+0x124`, the height it keeps `+0x128` (half way up the water), the heading `+0x12c`, the step its way search turns
//! by `+0x130` (±20°), the hold of Ratchet `+0x13c` (his place its own from 1), the big-head record `+0x140`. Its timer
//! byte `+0xbc` is its depth: 0 deep (more than 0.68 of water), 1 wading, 2 shallow (under 0.41; [`depth`],
//! `0x45b1c0`).
//!
//! **Each tick**: its head 2.2 with the enemies' big heads (cheat 7; L06 `0x367468`, [`big_head`]). In state 0 the
//! set-up ([`set_up`], `0x45b320`): with no polygons, grid or cuboid it is gone; its depth, home and place, its state
//! by its depth ([`settle`], `0x45b250`: deep 1, wading 7, shallow 0xf, sequences 0 / 3 / 9), the timer `rand % 120`,
//! the step ±20°. Else: the target search (`0x362aa0` = crate::moby::target_search::search without areas or polygon,
//! 30 across), its flash, the timer down (`s3`: out). Deep (states under 7, or the swim polygon 0) Ratchet is its
//! target within 30 across, `s2` him in the hunt polygon; wading the search's target, `s2` it in the polygon. Its depth
//! again: a change goes to the table's state and sequence (blend 6) at the state's end (deep → wading 7 / 3, → shallow
//! 0x10 / 8; wading → deep 1 / 0, → shallow 0x10 / 8; shallow → either 0x11 / 10). Its mouth (joint 0) in a sphere of
//! 0.8 touching water (surface 0): one tick in 45 (its uid's) at even odds a ripple on the grid at its place
//! (crate::water::grid::Grid::disturb, added) of the half water depth at most its mouth's depth (down while it sinks,
//! twice in states 0xc / 0xe) over twice its distance from the touch (at least 1); one tick in `20 + rand & 7` a ring
//! on the water there (crate::particles::water::ring: 1.5..2.4, 5250, ±0.15 across). Holding Ratchet he is at its
//! place. The grid frozen: sequence 0xd, 0x14.
//!
//! **States**:
//! * **1** idle deep: wrapped, sequence 0. Ratchet in the polygon and no other of its group in 4..6 ([`others_in`],
//!   `0x45bb18`): sequence 1, 4. Ratchet within 1.5 of its mouth ([`near`], `0x45baa8`): the grab (sequence 2 blend
//!   2, its sound 1, 5); else the timer out: sequence 1, the timer 300..500, a random heading, 2. Slowed 0.95, its
//!   height kept ([`keep_height`], `0x45b6e8`: the water's 0.3 below it, falling 9.8 a second²; eased to `+0x128`;
//!   the slide, `0x35ca60` = L04 `0x33cb08`, halving the velocity on a touch).
//! * **2** swimming: Ratchet in the polygon → 4 as in 1; else turned and pitched level toward the heading, in its
//!   cuboid with the timer out 1 (the timer 60..180). Its way ([`find_way`], `0x45b4d8`: 5.2 ahead inside the swim
//!   polygon, clear of the world by a line and a sphere of 1.5, else the step round until one is, else home); its
//!   speed ([`speed`], `0x45b7c8`: faster within 12° of its heading, slower past 30°, 0.05..0.117 a tick); facing its
//!   heading with the timer out, 1; the grab as in 1.
//! * **4** after Ratchet: turned and pitched to him; within 3 across slowed 0.9, and faced, with him swimming (group
//!   0x11 / 0x12), the grab: sequence 2, its sound, no collision, the hold 0, the start its place, 5. Its height his
//!   less 0.25. Him out of the polygon or `0x18a16c` 0: sequence 1, 2.
//! * **5** the grab: its mouth turned to him, its pitch × 0.7, the hold up by 0.028 and its place walked
//!   (crate::moby::ai::step_walk: 0.75, 0.75, 40, 2π, flags 3) the hold's share of the way from the start to where its
//!   mouth is on him. At 1 he dies: his health 0, his state 0x39, the view lock; 6.
//! * **6** swimming off with him (as 2's way and speed, slower turns).
//! * **7** wading idle: Ratchet (or a decoy) in the polygon: sequence 4, 9; the timer out: sequence 4, the timer
//!   300..500, a random heading, 8. Slowed 0.95, falling, the slide.
//! * **8** wading: as 2 with its own rates (to 7, sequence 3).
//! * **9** / **10** at the target: turned to it, its speed; faced within 1.9 the bite's start (sequence 5 blend 2,
//!   10); the target gone, 8; the bite ([`bite`], `0x45b8f0`: in 10 on sequence 5 between frames 17 and 20 a hit of 1
//!   from its mouth (a sphere of 1, flags 0x21) along its velocity — Ratchet hit first, it backs off, sequence 4 from
//!   frame 4 blend 5, 0xb); wrapped, sequence 4, the timer 300..500, 8.
//! * **0xb** wrapped: 7. **0xc** the knock's fall (crate::moby::knock::fall) landed / **0xd** wrapped: by its depth.
//!   **0xe** the fall touching something and wrapped: the general explosion (crate::moby::blast::explode: flashes 2 / 1
//!   past 4, three embers, billows and five flares, its debris, a light of 7) and it is gone. **0xf** / **0x10**
//!   stranded (slowed, falling, the slide), 0x10 by its depth once wrapped. **0x11** slowed, by its depth once wrapped.
//! * **0x14** frozen until the grid thaws: 0x15. **0x15** 360 ticks of its height kept, then sequence 0, 1.
//!
//! Out of 2..1021 on any axis it is gone. Nothing in the level enters states 0xc..0xe (the knock is never started).
//!
//! Not ported: the set-up's messages and state 0's (`0x2e3ff0`, prints); the polygons' lengths (`0x45b130`, each
//! point's w: nothing here reads them); Ratchet's moby's `+0x9c` = −1 at the grab's end; the explosion in 0xe is given
//! its velocity as the place, as the original passes it.

use crate::collision::{coll_sphere_m, CollOutput, QueryFlags};
use crate::hero::physics::{add, add_rot, atan2, dist2, ease, len2, rot_dist, scale, sub, V3};
use crate::moby::classes::level08::tabora_ai::spring_angle;
use crate::moby::classes::props::{set_seq, wrapped};
use crate::moby::{ai, countdown, Ctx, Moby, MobyWorld};

pub const CLASS: i16 = 0x0fe6;

const FLASH: usize = 0x50;
const KNOCK: usize = 0x60;
const VEL: usize = 0xc0;
const SPAWN: usize = 0xd0;
const HOME: usize = 0xe0;
const START: usize = 0xf0;
const SWIM: usize = 0x100;
const CUBOID: usize = 0x104;
const GRID: usize = 0x108;
const YAW_V: usize = 0x110;
const PITCH_V: usize = 0x114;
const GROUND: usize = 0x120;
const TIMER: usize = 0x124;
const HEIGHT: usize = 0x128;
const HEADING: usize = 0x12c;
const STEP: usize = 0x130;
const HUNT: usize = 0x134;
const HOLD: usize = 0x13c;
const HEAD: usize = 0x140;
/// The port's: the target search's block (the original's on the stack).
const SEARCH: usize = 0x180;
const LEN: usize = SEARCH + 0x48;
/// The grid moby's frozen byte (crate::water::grid).
const FROZEN: usize = 0x14;

/// The falls' gravity (9.8 / 60 a tick).
const FALL: u32 = 0x3e27_40db;
/// `$gp+0x2930` / `+0x2940` (`0x1b1920` / `0x1b1930`): the state and sequence of a change of depth, `old · 3 + new`.
const NEXT_STATE: [u8; 9] = [0xff, 7, 0x10, 1, 0xff, 0x10, 0x11, 0x11, 0xff];
const NEXT_SEQ: [u8; 9] = [0xff, 3, 8, 0, 0xff, 8, 10, 10, 0xff];

fn f(bits: u32) -> f32 { f32::from_bits(bits) }

/// The liquid grid moby `+0x108`.
fn grid(w: &MobyWorld, id: usize) -> Option<usize> { usize::try_from(w.mobys[id].pv_i(GRID)).ok().filter(|&k| k < w.mobys.len()) }

/// `*+0x11c`: the water's height (the grid moby's).
fn water_z(w: &MobyWorld, id: usize) -> f32 { grid(w, id).map_or(0.0, |k| w.mobys[k].pos[2]) }

/// `0x2df8d8(r, c, flags, m)`: the sphere test.
fn sphere(ctx: &Ctx, c: V3, r: f32, flags: u32, id: usize) -> Option<CollOutput> {
    ctx.coll.and_then(|cl| coll_sphere_m(cl, ctx.scene, c, r, QueryFlags(flags), Some(id)))
}

fn mouth(w: &MobyWorld, id: usize, ctx: &Ctx) -> V3 { ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos) }

/// `0x31b5f0(m, seq, 0, blend)` unless it plays, and the state.
fn go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, seq: u8, blend: u8, state: u8) {
    set_seq(w, id, ctx, seq, 0, blend);
    w.mobys[id].state = state;
}

fn rand_ticks(ctx: &mut Ctx, n: i32, base: i32) -> i32 { ctx.game.rng.rand() as i32 % n + base }

/// L06 `0x367468(2.2, m, 0, +0x140)` (module docs): the big head's node on joint list 0 (`0x3524c8` = L00
/// `0x312480`), taken off (`0x352540` = L00 `0x3124f8`) without the cheat.
fn big_head(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let on = ctx.game.cheat(crate::cheats::cheat::ENEMY_HEADS);
    let s = f(0x400c_cccd);
    let m = &mut w.mobys[id];
    if on {
        if m.pv_b(HEAD + 1) == 0 {
            m.set_pv_b(HEAD + 1, 1);
            m.joint_scales.retain(|n| n.0 != 0);
            m.joint_scales.push((0, s));
        }
        for o in [0x20, 0x24, 0x28] { m.set_pv_f(HEAD + o, s); }
    } else if m.pv_b(HEAD + 1) != 0 {
        m.joint_scales.retain(|n| n.0 != 0);
        m.pvars[HEAD..HEAD + 0x40].fill(0);
    }
}

/// `0x45b1c0`: the ground under it (`0x31ce98`), the height to keep half way up the water, its depth.
fn depth(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> u8 {
    let g = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0x20);
    let water = water_z(w, id);
    let m = &mut w.mobys[id];
    m.set_pv_f(GROUND, g);
    let t = water - g;
    m.set_pv_f(HEIGHT, g + t * 0.5);
    if t < f(0x3ed1_eb85) { 2 } else if f(0x3f2e_147b) < t { 0 } else { 1 }
}

/// `0x45b250`: its state by its depth.
fn settle(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].timer {
        0 => go(w, id, ctx, 0, 10, 1),
        1 => go(w, id, ctx, 3, 10, 7),
        2 => go(w, id, ctx, 9, 10, 0xf),
        _ => {}
    }
}

/// `0x45b320` (module docs): false when it is gone.
fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let m = &w.mobys[id];
    if m.pv_i(SWIM) == -1 || m.pv_i(HUNT) == -1 || m.pv_i(GRID) == -1 { return false; }
    let d = depth(w, id, ctx);
    w.mobys[id].timer = d;
    let Some((home, _)) = ctx.level.cuboid_place(w.mobys[id].pv_i(CUBOID)) else { return false };
    let m = &mut w.mobys[id];
    m.set_pv_v(HOME, home);
    let p = m.pos;
    m.set_pv_v(SPAWN, p);
    settle(w, id, ctx);
    let t = rand_ticks(ctx, 0x78, 0);
    let step = if ctx.game.rng.rand() & 1 == 0 { f(0xbeb2_b8c4) } else { f(0x3eb2_b8c4) };
    let m = &mut w.mobys[id];
    m.set_pv_i(TIMER, t);
    m.set_pv_f(STEP, step);
    m.set_pv_f(HOLD, 0.0);
    m.set_pv_b(0x29, 1);
    true
}

/// `0x45b6e8(m, pvars, accel, brake, most)` (module docs).
fn keep_height(w: &mut MobyWorld, id: usize, ctx: &Ctx, accel: u32, brake: u32, most: u32) {
    let water = water_z(w, id);
    let m = &w.mobys[id];
    let mut v = m.pv_v(VEL);
    if water - f(0x3e99_999a) < m.pos[2] { v[2] -= f(FALL); }
    let mut z = m.pos[2];
    ease(m.pv_f(HEIGHT), f(accel), f(brake), f(most), &mut z, &mut v[2]);
    slide(w, id, ctx, v, true);
}

/// `0x35ca60(1, 0, m, &vel, 0x400, 0x10)` (= L04 `0x33cb08`): its move; `halve`: the velocity halved on a touch.
fn slide(w: &mut MobyWorld, id: usize, ctx: &Ctx, mut v: V3, halve: bool) {
    let r = crate::moby::slide::slide_r(w, id, ctx, &mut v, 0x400, 0x10, 1.0, 0.0);
    if halve && r != 0 { v = scale(v, 0.5); }
    w.mobys[id].set_pv_v(VEL, v);
}

/// Slowed (`0x2eeee0(0.95)`).
fn slow(m: &mut Moby, k: u32) { m.set_pv_v(VEL, scale(m.pv_v(VEL), f(k))); }

/// Falling and the slide (the wading states).
fn fall_slide(w: &mut MobyWorld, id: usize, ctx: &Ctx, halve: bool) {
    let mut v = w.mobys[id].pv_v(VEL);
    v[2] -= f(FALL);
    slide(w, id, ctx, v, halve);
}

/// `0x45b7c8(yaw, near, up, far, down, most, least, m, pvars)` (module docs): true when it faces `yaw`.
#[allow(clippy::too_many_arguments)]
fn speed(m: &mut Moby, yaw: f32, near: u32, up: u32, far: u32, down: u32, most: u32, least: u32) -> bool {
    let mut v = m.pv_v(VEL);
    let mut s = len2(v);
    let off = rot_dist(m.rot[2], yaw);
    let mut faced = false;
    if off < f(near) {
        s += f(up);
        faced = true;
    } else if f(far) < off {
        s -= f(down);
    }
    let s = if f(most) < s { f(most) } else if s < f(least) { f(least) } else { s };
    v[0] = m.rot[2].cos() * s;
    v[1] = m.rot[2].sin() * s;
    m.set_pv_v(VEL, v);
    faced
}

/// `0x35c2a0` (= L00 `0x31c078`) on its yaw and `0x35c138` (= L01 `0x311de0`) on its pitch toward level.
fn turn(m: &mut Moby, yaw: f32, k: u32, d: u32, most: u32) {
    ai::turn_to(m, yaw, f(k), f(d), f(most), YAW_V);
    pitch(m, 0.0, k, d, most);
}

fn pitch(m: &mut Moby, t: f32, k: u32, d: u32, most: u32) {
    let (a, v) = spring_angle(m.rot[1], t, f(k), f(d), f(most), m.pv_f(PITCH_V));
    m.rot[1] = a;
    m.set_pv_f(PITCH_V, v);
}

/// `0x45b4d8` (module docs): its heading.
fn find_way(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let (pos, yaw, step, poly, home) = (m.pos, m.rot[2], m.pv_f(STEP), m.pv_i(SWIM), m.pv_v(HOME));
    let mut heading = None;
    if m.pv_i(TIMER) != 0 {
        let reach = f(0x40a6_6666);
        let open = |a: f32| {
            let p = add([a.cos() * reach, a.sin() * reach, 0.0], pos);
            ctx.level.in_polygon(poly, p) && w.coll_line(ctx, pos, p, 0x24, Some(id)).is_none() && sphere(ctx, p, 1.5, 0x24, id).is_none()
        };
        if open(yaw) { return; }
        let (mut a, mut total) = (yaw, 0.0);
        loop {
            a = add_rot(a, step);
            if open(a) {
                heading = Some(a);
                break;
            }
            total += step.abs();
            if f(0x40c9_0fdc) <= total { break; }
        }
    }
    let h = heading.unwrap_or_else(|| atan2(home[0] - pos[0], home[1] - pos[1]));
    w.mobys[id].set_pv_f(HEADING, h);
}

/// `0x45bb18(m, lo, hi)`: another live one of its group in a state from `lo` to `hi`.
fn others_in(w: &MobyWorld, id: usize, lo: u8, hi: u8) -> bool {
    let Some(g) = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)) else { return false };
    g.iter().any(|&k| k != id && w.mobys.get(k).is_some_and(|o| o.state < 0x80 && (lo..=hi).contains(&o.state)))
}

/// `0x45baa8(m, who, r)`: a sphere of `r` at its mouth touches `who` (flags 0x21).
fn near(w: &MobyWorld, id: usize, ctx: &Ctx, who: Option<usize>, r: f32) -> bool {
    let c = mouth(w, id, ctx);
    sphere(ctx, c, r, 0x21, id).is_some_and(|h| h.moby.is_some() && h.moby == who)
}

/// The grab's start (sequence 2 blend 2, its sound 1, 5).
fn grab(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    go(w, id, ctx, 2, 2, 5);
    ctx.host.play_sound(&w.mobys[id], 1, 0);
}

/// `0x45b8f0(17, 20, 1, 1, 1, m, pvars, 10, 5, 4, 4, 0xb, target)` (module docs).
fn bite(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, target: Option<usize>) {
    let m = &w.mobys[id];
    let now = ctx.host.frame_time(m);
    let seq_a = ctx.host.anim(m).map_or(0xff, |a| a.seq_a);
    if m.state != 10 || seq_a != 5 || now <= 17.0 || 20.0 <= now { return; }
    let v = m.pv_v(VEL);
    let t = crate::moby::hits::HitTemplate::new(1.0, Some(id), 1, [v[0], v[1], 1.0, m.pv_f(VEL + 0xc)]);
    let c = mouth(w, id, ctx);
    let n = w.sphere_hit(ctx.scene, c, 1.0, 0x21, Some(id), Some(&t));
    if n == 0 || w.hits.last_target != target { return; }
    let m = &mut w.mobys[id];
    let mut v = scale(m.pv_v(VEL), -1.0);
    v[2] = f(0xbd4c_cccd);
    m.set_pv_v(VEL, v);
    set_seq(w, id, ctx, 4, 4, 5);
    w.mobys[id].state = 0xb;
}

/// 0x0fe6 (module docs).
pub fn update_0fe6(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < LEN { w.mobys[id].pvars.resize(LEN, 0); }
    big_head(w, id, ctx);
    if w.mobys[id].state == 0 {
        if !set_up(w, id, ctx) { return w.delete(id); }
        return bounds(w, id);
    }
    crate::moby::target_search::search(w, id, ctx, SEARCH, &[], -1, 30.0);
    // `0x410d18(m, pvars, +0x20)`: empty.
    let m = &mut w.mobys[id];
    ai::flash(m, FLASH);
    let mut t = m.pv_i(TIMER);
    let s3 = countdown(&mut t);
    m.set_pv_i(TIMER, t);
    let hero = ctx.hero.pos;
    let (mut at, mut target) = (m.pv_v(SEARCH), ai::link(m, SEARCH + 0x40));
    let s2 = if m.pv_i(SWIM) == 0 || m.state < 7 {
        if dist2(m.pos, hero) < 30.0 {
            (at, target) = (hero, ctx.hero_moby);
        } else {
            (at, target) = ([0.0; 3], None);
        }
        ctx.level.in_polygon(m.pv_i(HUNT), hero)
    } else {
        ctx.level.in_polygon(m.pv_i(HUNT), at)
    };
    let is_hero = target.is_some() && target == ctx.hero_moby;

    let old = w.mobys[id].timer;
    let new = depth(w, id, ctx);
    w.mobys[id].timer = new;
    let k = new as usize + old as usize * 3;
    let (next, next_seq) = if old != new { (NEXT_STATE.get(k).copied().unwrap_or(0xff), NEXT_SEQ.get(k).copied().unwrap_or(0xff)) } else { (0xff, 0xff) };

    water_fx(w, id, ctx);
    if 1.0 <= w.mobys[id].pv_f(HOLD) { ctx.hero.pos = w.mobys[id].pos; }
    if grid(w, id).is_some_and(|g| w.mobys[g].pv_b(FROZEN) != 0) { go(w, id, ctx, 0xd, 10, 0x14); }

    // The depth's change at the state's end.
    let table = |w: &mut MobyWorld, ctx: &mut Ctx| if next != 0xff { go(w, id, ctx, next_seq, 6, next) };
    // Light rates of 2 / 6 / 8 (all in bits).
    const R2: (u32, u32, u32) = (0x39fe_2e99, 0x3aa5_37e4, 0x3cee_4bb0);
    const R6: (u32, u32, u32) = (0x39e4_c38a, 0x3a64_c38a, 0x3d56_7752);
    const R8: (u32, u32, u32) = (0x3964_c38a, 0x39e4_c38a, 0x3c56_7752);
    const H1: (u32, u32, u32) = (0x3b88_8889, 0x3c63_8e39, 0x3e08_8889);
    match w.mobys[id].state {
        0x14 => {
            if let Some(g) = grid(w, id) {
                if w.mobys[g].pv_b(FROZEN) == 0 { w.mobys[id].state = 0x15; }
            }
        }
        0x15 => {
            if (w.mobys[id].state_ticks as u16) < 0x168 {
                keep_height(w, id, ctx, H1.0, H1.1, H1.2);
                w.mobys[id].state_ticks += 1;
            } else {
                set_seq(w, id, ctx, 0, 0, 10);
                w.mobys[id].state_ticks = 0;
                w.mobys[id].state = 1;
            }
        }
        1 => {
            if wrapped(w, id, ctx) { set_seq(w, id, ctx, 0, 0, 10); }
            if s2 && is_hero && !others_in(w, id, 4, 6) { go(w, id, ctx, 1, 10, 4); }
            if is_hero && near(w, id, ctx, target, 1.5) {
                grab(w, id, ctx);
            } else if s3 {
                go(w, id, ctx, 1, 10, 2);
                let t = rand_ticks(ctx, 200, 300);
                let a = ctx.game.rng.rand_angle();
                let m = &mut w.mobys[id];
                m.set_pv_i(TIMER, t);
                m.set_pv_f(HEADING, a);
            }
            slow(&mut w.mobys[id], 0x3f73_3333);
            keep_height(w, id, ctx, H1.0, H1.1, H1.2);
            table(w, ctx);
        }
        2 => {
            if wrapped(w, id, ctx) { set_seq(w, id, ctx, 1, 0, 10); }
            if s2 && is_hero {
                if !others_in(w, id, 4, 6) { go(w, id, ctx, 1, 10, 4); }
            } else {
                let m = &mut w.mobys[id];
                let h = m.pv_f(HEADING);
                turn(m, h, R2.0, R2.1, R2.2);
                if s3 && ctx.level.in_cuboid(m.pv_i(CUBOID), m.pos) {
                    go(w, id, ctx, 0, 10, 1);
                    let t = rand_ticks(ctx, 0x78, 0x3c);
                    w.mobys[id].set_pv_i(TIMER, t);
                }
            }
            find_way(w, id, ctx);
            let m = &mut w.mobys[id];
            let h = m.pv_f(HEADING);
            if speed(m, h, 0x3e56_7751, 0x3ada_740e, 0x3f06_0a93, 0x3a91_a2b4, 0x3dee_eef0, 0x3d4c_ccce) && s3 {
                go(w, id, ctx, 0, 10, 1);
                let t = rand_ticks(ctx, 0x78, 0x3c);
                w.mobys[id].set_pv_i(TIMER, t);
            }
            if is_hero && near(w, id, ctx, target, 1.5) { grab(w, id, ctx); }
            keep_height(w, id, ctx, H1.0, H1.1, H1.2);
            table(w, ctx);
        }
        4 => {
            let m = &mut w.mobys[id];
            let pos = m.pos;
            let yaw = atan2(at[0] - pos[0], at[1] - pos[1]);
            ai::turn_to(m, yaw, f(0x3b64_c38a), 0.0, f(0x3e56_7752), YAW_V);
            let p = atan2(dist2(pos, at), at[2] - pos[2]);
            pitch(m, -p, 0x3b64_c38a, 0, 0x3e56_7752);
            let close = dist2(pos, hero) < 3.0;
            if close { slow(m, 0x3f66_6666); }
            let faced = speed(m, yaw, 0x3e56_7751, 0x3bb6_0b61, 0x3f06_0a93, 0x3a5a_740e, 0x3eaa_aaab, 0x3daa_aaab);
            if faced && close && (ctx.hero.group == 0x11 || ctx.hero.group == 0x12) {
                grab(w, id, ctx);
                let m = &mut w.mobys[id];
                m.has_collision = false;
                m.set_pv_f(HOLD, 0.0);
                let p = m.pos;
                m.set_pv_v(START, p);
            }
            w.mobys[id].set_pv_f(HEIGHT, at[2] - 0.25);
            keep_height(w, id, ctx, 0x3c2c_f136, 0, 0x3f22_2223);
            if !s2 || ctx.hero.f16c == 0 { go(w, id, ctx, 1, 10, 2); }
            table(w, ctx);
        }
        5 => {
            let c = mouth(w, id, ctx);
            let m = &mut w.mobys[id];
            let off = sub(c, m.pos);
            let yaw = atan2(at[0] - c[0], at[1] - c[1]);
            ai::turn_to(m, yaw, f(0x3b64_c38a), 0.0, f(0x3e56_7752), YAW_V);
            m.rot[1] *= f(0x3f33_3333);
            let hold = m.pv_f(HOLD) + f(0x3ce5_6042);
            m.set_pv_f(HOLD, hold);
            let before = m.pos;
            let start = m.pv_v(START);
            let mut to = add(scale(sub(sub(at, off), start), hold.min(1.0)), start);
            let mut p = before;
            ai::step_walk(ctx, id, 0.75, 0.75, 40.0, f(0x40c9_0fdc), &mut p, &mut to, 3);
            let m = &mut w.mobys[id];
            m.pos = p;
            m.set_pv_v(VEL, sub(p, before));
            if 1.0 <= hold {
                ctx.hero.hp = 0;
                ctx.hero.request_state(0x39, true);
                ctx.hero.view_lock = 1;
                m.state = 6;
            } else {
                m.set_pv_f(HEADING, yaw);
            }
        }
        6 => {
            find_way(w, id, ctx);
            let m = &mut w.mobys[id];
            let h = m.pv_f(HEADING);
            speed(m, h, 0x3e56_7751, 0x3ada_740e, 0x3f06_0a93, 0x3a91_a2b4, 0x3dee_eef0, 0x3d4c_ccce);
            keep_height(w, id, ctx, 0x3b5a_740e, 0x3c63_8e39, 0x3dee_eef0);
            let m = &mut w.mobys[id];
            turn(m, h, R6.0, R6.1, R6.2);
            if wrapped(w, id, ctx) { set_seq(w, id, ctx, 1, 0, 10); }
            table(w, ctx);
        }
        7 => {
            if s2 {
                go(w, id, ctx, 4, 10, 9);
            } else if s3 {
                go(w, id, ctx, 4, 10, 8);
                let t = rand_ticks(ctx, 200, 300);
                let a = ctx.game.rng.rand_angle();
                let m = &mut w.mobys[id];
                m.set_pv_i(TIMER, t);
                m.set_pv_f(HEADING, a);
            }
            slow(&mut w.mobys[id], 0x3f73_3333);
            fall_slide(w, id, ctx, true);
            table(w, ctx);
        }
        8 => {
            if s2 {
                go(w, id, ctx, 4, 10, 9);
            } else {
                let m = &mut w.mobys[id];
                let h = m.pv_f(HEADING);
                turn(m, h, R8.0, R8.1, R8.2);
                if s3 && ctx.level.in_cuboid(m.pv_i(CUBOID), m.pos) {
                    go(w, id, ctx, 3, 10, 7);
                    let t = rand_ticks(ctx, 0x78, 0x3c);
                    w.mobys[id].set_pv_i(TIMER, t);
                }
            }
            find_way(w, id, ctx);
            let m = &mut w.mobys[id];
            let h = m.pv_f(HEADING);
            if speed(m, h, 0x3eb2_b8c4, 0x3a5a_740e, 0x3f86_0a93, 0x3b23_d70a, 0x3c88_8889, 0x3c88_8889) && s3 {
                go(w, id, ctx, 3, 10, 7);
                let t = rand_ticks(ctx, 0x78, 0x3c);
                w.mobys[id].set_pv_i(TIMER, t);
            }
            fall_slide(w, id, ctx, true);
            table(w, ctx);
        }
        9 | 10 => {
            let m = &mut w.mobys[id];
            let pos = m.pos;
            let yaw = atan2(at[0] - pos[0], at[1] - pos[1]);
            ai::turn_to(m, yaw, f(R8.0), f(R8.1), f(R8.2), YAW_V);
            let faced = speed(m, yaw, 0x3f06_0a93, 0x3a91_a2b4, 0x3fc9_0fdc, 0x3b23_d70a, 0x3ccc_ccce, 0x3c88_8889);
            if faced && dist2(pos, at) < f(0x3ff3_3333) && m.state != 10 { go(w, id, ctx, 5, 2, 10); }
            fall_slide(w, id, ctx, true);
            let m = &mut w.mobys[id];
            m.set_pv_b(0x48, 8);
            m.set_pv_b(0x4a, 9);
            if !s2 { go(w, id, ctx, 4, 10, 8); }
            bite(w, id, ctx, target);
            if wrapped(w, id, ctx) {
                go(w, id, ctx, 4, 10, 8);
                let t = rand_ticks(ctx, 200, 300);
                w.mobys[id].set_pv_i(TIMER, t);
            }
            table(w, ctx);
        }
        0xb => {
            if wrapped(w, id, ctx) { w.mobys[id].state = 7; }
        }
        0xc => {
            if crate::moby::knock::fall(w, id, ctx, KNOCK) & 1 != 0 { settle(w, id, ctx); }
        }
        0xd => {
            if wrapped(w, id, ctx) { settle(w, id, ctx); }
        }
        0xe => {
            let fl = crate::moby::knock::fall(w, id, ctx, KNOCK);
            if fl & 2 != 0 && wrapped(w, id, ctx) {
                // `0x360fb0(0, 0, 2, 1, 4, 1, 7, m, +0xc0, 0, 3, 3, 5, −1, 0, 1, −1, 0, 0x810001)` (= L00 `0x320538`).
                let e = crate::moby::blast::Explosion {
                    radius: 0.0, damage: 0.0, flags: 0x81_0001, flash: 2.0, flash2: 1.0, near: 4.0, size: 1.0, light: 7.0,
                    embers: 3, billows: 3, flares: 5, sound: -1, shake: false, debris: 1, lod: -1, colour: 0,
                };
                let at = w.mobys[id].pv_v(VEL);
                crate::moby::blast::explode(w, ctx, Some(id), Some(at), &e);
                return w.delete(id);
            }
        }
        0xf => {
            slow(&mut w.mobys[id], 0x3f73_3333);
            fall_slide(w, id, ctx, false);
            table(w, ctx);
        }
        0x10 => {
            slow(&mut w.mobys[id], 0x3f73_3333);
            fall_slide(w, id, ctx, false);
            if wrapped(w, id, ctx) { settle(w, id, ctx); }
        }
        0x11 => {
            slow(&mut w.mobys[id], 0x3f73_3333);
            if wrapped(w, id, ctx) { settle(w, id, ctx); }
        }
        _ => {}
    }
    bounds(w, id);
}

/// The water's touch at its mouth (module docs): the ripple and the rings.
fn water_fx(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let c = mouth(w, id, ctx);
    let Some(h) = sphere(ctx, c, f(0x3f4c_cccd), 1, id).filter(|h| h.surface_id() == 0) else { return };
    let water = water_z(w, id);
    let m = &w.mobys[id];
    let half = (water - m.pv_f(GROUND)) * 0.5;
    let d = (m.pos[2] - c[2]).abs();
    let mut amp = if half < d { half } else { d };
    if m.pv_f(VEL + 8) < 0.0 { amp = -amp; }
    if m.state == 0xe || m.state == 0xc { amp += amp; }
    let far = dist2(m.pos, h.point);
    let r = if 0.5 < far { far + far } else { 1.0 };
    let (x, y, uid) = (m.pos[0], m.pos[1], m.uid as i32);
    if ctx.game.frame % 45 == uid && ctx.game.rng.randi(2) == 0 {
        let n = w.liquid.patches.len();
        w.liquid.disturb(x, y, r, amp, 0..n, true);
    }
    let k = (ctx.game.rng.rand() & 7) as i32 + 0x14;
    if ctx.game.frame % k == 0 {
        let rng = &mut ctx.game.rng;
        let size = rng.randf(1.5, f(0x4019_999a));
        let j = f(0x3e19_999a);
        let mut p = c;
        p[0] += rng.randf(-j, j);
        p[1] += rng.randf(-j, j);
        p[2] = w.liquid.surface(h.point).unwrap_or(h.point[2]);
        if let Some(sys) = ctx.parts.as_deref_mut() { crate::particles::water::ring(sys, rng, size, 5250.0, p, false, None); }
    }
}

/// Out of 2..1021 on any axis it is gone (`0x3516d0`).
fn bounds(w: &mut MobyWorld, id: usize) {
    if w.mobys.get(id).is_some_and(|m| m.pos.iter().any(|&x| x < 2.0 || 1021.0 < x)) { w.delete(id); }
}
