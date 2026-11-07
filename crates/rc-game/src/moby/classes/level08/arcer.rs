//! Tabora's arcer **0x1002** (L08 `0x427d98`, 35 placed; the decomp's `func_L08_00426730`), the cover it bursts out
//! of **0x0987** (`0x3d5258`, made by `0x3d51d8`) and the beam manager **0x1006** (`0x42a750`, crate::fx::beams). The
//! arcer lies under the sand until woken, rises, roams about its home, closes on Ratchet and sweeps him with an arc
//! (a beam slot of crate::fx::beams), and dying leaves a crystal (0x08c4, crate::moby::classes::level08::crystal).
//! On the framework (crate::moby::ai): health `+0x20`, reaction `+0x50` (a hurt goes to state 5).
//!
//! **Its pvars:** `+0x120` the target block (`+0x140` its aim, `+0x164` the result: 2 none), `+0x170` home,
//! `+0x180` the goal, `+0x190` the velocity, `+0x198` the rise's speed, `+0x1a0` the knock's push, the s16 timers
//! `+0x1b0` (the wait), `+0x1b4`, `+0x1b6` (after an attack), `+0x1b8` (Ratchet near), `+0x1ba` (the knock's ticks),
//! `+0x1bc` (ticks unseen), `+0x1be` (ticks since it rose, to 120), `+0x1c0` / `+0x220` the turn's and the pitch's
//! spring speeds, `+0x1c4` the floor, `+0x1c8` its height, `+0x1cc` the aim's lean to Ratchet, `+0x1d0` / `+0x1d4`
//! its first volumes (path, sphere), `+0x1d8` / `+0x1dc` / `+0x1e0` the roam's (path, sphere, cuboid), `+0x1e4` the
//! path its goals keep to, `+0x1e8` / `+0x1ec` the mobys it guards / waits for, `+0x1f0` / `+0x1f4` the sweep's way
//! and angle, `+0x1f8` / `+0x1fc` / `+0x200` the group's shared turn, count and stamp (0x4cb2f), `+0x208` its heading,
//! `+0x20c` how it wakes (0 / 1 Ratchet in view and in its first volumes, one tick in seven; 2 a poke; 3 the moby
//! `+0x1ec` gone; 5 the moby `+0x1e8` past its state 2; a poke with bit 1 always), `+0x20d` its turn in the group,
//! `+0x20e` / `+0x20f` its update distances awake and under (also the roam's radius), `+0x218` / `+0x219` / `+0x21a`
//! the attack's step, sweeps and pause, `+0x21c` the arc's power (0..1), `+0x224` / `+0x228` the groups a hit pokes,
//! `+0x22c` its cover.
//!
//! **Each tick** (outside the level's loaded region nothing; [`head`], `0x427590`, first): its floor ([`floor`]), its
//! class's scale, a hit from 0x0a66 / 0x0a82 / 0x0cbb / 0x0cf3 halved (`0x427518`), the frame's damage (crate::moby::
//! ai::take_damage): pushed away from Ratchet (a sixth) on 1, else its turn passed on ([`pass_turn`]); then its groups
//! poked (`+0x21`, `+0x224` and, when `+0x228` is set, `+0x224` again) and its own byte `+0xbc` set. In state 0, a uid
//! counted in the save deletes it; else [`setup`]. Farther across from the camera than its update distance, nothing
//! more.
//! * **1** under the sand ([`wake`]): woken and in view (`0x331400`), its awake distance, shown, the camera's roll
//!   0.25 for 38 ticks, its cover told to break; 2.
//! * **2** rising to its height (`0x2ab3b0`, crate::hero::physics::ease); sand puffs ([`puffs`]) while low; its
//!   sequence wrapped: its collision, targetable, a new goal ([`new_goal`]); 4 (sub 0).
//! * **3** sinking home to under its floor; there, its collision gone, 1 with its update distance under.
//! * **4** awake, one tick in seven looking for Ratchet in its roam's volumes (60 near); within 8 of him `+0x1b8` 120.
//!   Sub 0 roams to its goals, then (after 60 ticks) home (sub 4, then 3); sub 1 waits; with Ratchet seen and not
//!   unseen, sub 2 hunts: its goal 15 from him ([`goal_at`]), fast while he is far; on its turn, its wait over and a
//!   beam slot claimed (light bits 1), sub 3 attacks — turned to him within 40°, the arc's power up 0.01 a tick, three
//!   sweeps of ±45° (1.17° a tick; [`aim`]), the power down, its turn passed and the slot given back; Ratchet lost or
//!   unseen 240 ticks, back to roaming. Its facing is its heading.
//! * **5** the knock (crate::moby::ai::flight): dead, 6; landed or 115 ticks, hunting (sub 2) or 6 with no health;
//!   puffs while low.
//! * **6 / 7** dying: the crystal 0x08c4 left at it (on its floor), 7 the explosion (crate::moby::blast::explode: 20
//!   embers, 6 billows, 32 flares, flashes 8 and 4 past 9, size 2, light 30, the shake, 2 debris), its turn given up
//!   ([`leave_turn`]), its bolts (crate::moby::bolts::drop), its uid's marks cleared (the crystal carries them);
//!   deleted.
//! * At the end ([`tail`], `0x426bd0`): its ticks, unseen ticks, the wait down, `+0x1be` up from state 3, its shadow's
//!   slab from state 2. Out of 30..993 on any axis it gives up its turn and is deleted.
//!
//! **0x0987** ([`update_0987`]): made at the arcer's place with its distances (`0x3d51d8`); in 1 it waits for its byte
//! `+0xbc`, then breaks into its pieces (crate::moby::shards::break_into: 0x7f, 0.15 up) and is gone.
//!
//! The engine functions with the port's: `0x330a10` (tabora_ai::find_target), `0x3300f8` / `0x32fa98`
//! (tabora_ai::volume_reach / in_volumes), `0x329170` (tabora_ai::spring_angle), `0x3297f0` = L00 `0x31c618`
//! (crate::moby::slide::slide), `0x32a160` = L00 `0x31ce20` (crate::moby::ai::ground_z), `0x2ea670` the line test,
//! `0x2fada0` = L00 `0x2ef1d0` (at most so long), `0x3d4d80` the camera's look target (crate::moby::classes::look_targets::offer).
//!
//! Not ported: the path's length `0x351498` the set-up asks; the vehicle
//! heights (`0x1bff58` against Ratchet's vehicle `0x18a650`) in the floor and the aim; the shadow's two bytes
//! `+0x6f` / `+0xbd`; the debug print of a wake mode 1 / 4 (stepped to 2 / 5).

use super::tabora_ai::{find_target, in_volumes, spring_angle, volume_reach, Volumes};
use crate::fx::beams::{self, at_most, Kind};
use crate::hero::physics::{add, add_rot, atan2, cross, dist2, len, rot_dist, scale, set_len, sub, V3};
use crate::moby::classes::props::{seq, wrapped};
use crate::moby::{ai, countdown, Ctx, MobyWorld};
use std::f32::consts::{FRAC_PI_4, PI};

pub const CLASS: i16 = 0x1002;
pub const COVER: i16 = 0x0987;
pub const MANAGER: i16 = 0x1006;

const HEALTH: usize = 0x20;
const T_BLOCK: usize = 0x120;
const T_AIM: usize = 0x140;
const T_RESULT: usize = 0x164;
const HOME: usize = 0x170;
const GOAL: usize = 0x180;
const VEL: usize = 0x190;
const RISE_V: usize = 0x198;
const PUSH: usize = 0x1a0;
const WAIT: usize = 0x1b0;
const LOST: usize = 0x1b4;
const AFTER: usize = 0x1b6;
const NEAR: usize = 0x1b8;
const KNOCK: usize = 0x1ba;
const UNSEEN: usize = 0x1bc;
const RISEN: usize = 0x1be;
const TURN_V: usize = 0x1c0;
const FLOOR: usize = 0x1c4;
const HEIGHT: usize = 0x1c8;
const LEAN: usize = 0x1cc;
const ANCHOR: usize = 0x1e8;
const WAITS_FOR: usize = 0x1ec;
const SWEEP_WAY: usize = 0x1f0;
const SWEEP: usize = 0x1f4;
const GROUP_TURN: usize = 0x1f8;
const GROUP_COUNT: usize = 0x1fc;
const GROUP_STAMP: usize = 0x200;
const HEADING: usize = 0x208;
const WAKE: usize = 0x20c;
const MY_TURN: usize = 0x20d;
const AWAKE_DIST: usize = 0x20e;
const UNDER_DIST: usize = 0x20f;
const STEP: usize = 0x218;
const SWEEPS: usize = 0x219;
const PAUSE: usize = 0x21a;
const POWER: usize = 0x21c;
const PITCH_V: usize = 0x220;
const POKE: usize = 0x224;
const POKE_B: usize = 0x228;
const COVER_AT: usize = 0x22c;
/// `$gp+0x2ce4`: the beam's height over the ground.
const OVER: f32 = 0.6;
/// The stamp the group's shared words carry once set up.
const STAMP: i32 = 0x4cb2f;

/// `0x42a750`.
pub fn update_1006(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { beams::step(w, id, ctx, Kind::Tabora); }

fn first_vols(w: &MobyWorld, id: usize) -> Volumes { let m = &w.mobys[id]; Volumes(m.pv_i(0x1d0), -1, m.pv_i(0x1d4), -1) }
fn roam_vols(w: &MobyWorld, id: usize) -> Volumes { let m = &w.mobys[id]; Volumes(m.pv_i(0x1d8), m.pv_i(0x1e0), m.pv_i(0x1dc), -1) }

fn down(w: &mut MobyWorld, id: usize, off: usize) -> bool {
    let m = &mut w.mobys[id];
    let mut t = m.pv_s(off) as i32;
    let r = countdown(&mut t);
    m.set_pv_s(off, t as i16);
    r
}

fn anim(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, blend: u8) {
    if seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, blend); }
}

/// `0x331400(m, far)`: its sphere in view within `far`.
fn in_view(w: &MobyWorld, id: usize, ctx: &Ctx, far: f32) -> bool {
    let b = w.mobys[id].bsphere.map(|x| x / 1024.0);
    !ctx.view.is_some_and(|v| v.culled(far, b))
}

/// `0x426990(p, top, bottom, low)`: a line down at `p` from `top` to `bottom` (flags 2): its height, at least `low`
/// (none: `low`), and whether it met water (surface 0).
fn probe(ctx: &Ctx, p: V3, top: f32, bottom: f32, low: f32) -> (f32, bool) {
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, [p[0], p[1], top], [p[0], p[1], bottom], crate::collision::QueryFlags(2), None));
    match hit {
        Some(h) => (low.max(h.point[2]), h.surface_id() == 0),
        None => (low, false),
    }
}

/// `0x426a00`: the floor under it (from home 10 up to 3 down; 2 more on water) and its height (6.5 over it, at least
/// Ratchet's).
fn floor(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let home = m.pv_f(HOME + 8);
    let (mut f, water) = probe(ctx, m.pos, home + 10.0, home - 3.0, m.pv_f(0x90));
    if water { f += 2.0; }
    let m = &mut w.mobys[id];
    m.set_pv_f(FLOOR, f);
    m.set_pv_f(HEIGHT, (f + 6.5).max(ctx.hero.pos[2]));
}

/// The group's shared words (pvar pointers into section 21).
fn shared(w: &MobyWorld, id: usize, off: usize) -> i32 { w.shared_i(w.mobys[id].pv_i(off)) }
fn set_shared(w: &mut MobyWorld, id: usize, off: usize, v: i32) { let p = w.mobys[id].pv_i(off); w.set_shared_i(p, v); }

/// `0x426538`: its turn over — the group's turn on past it, kept under the count.
fn pass_turn(w: &mut MobyWorld, id: usize) {
    if w.mobys[id].group == -1 { return; }
    let mine = w.mobys[id].pv_b(MY_TURN) as i32;
    if mine == shared(w, id, GROUP_TURN) { set_shared(w, id, GROUP_TURN, mine + 1); }
    let n = shared(w, id, GROUP_COUNT);
    let t = if 0 < n { shared(w, id, GROUP_TURN) % n } else { 0 };
    set_shared(w, id, GROUP_TURN, t);
}

/// `0x4265a8`: it leaves the group's turns: the count down one, its turn passed. (The game lowers its own turn byte
/// once per member of its group, meaning the others'; it is deleted right after, so the port leaves that out.)
fn leave_turn(w: &mut MobyWorld, id: usize) {
    if w.mobys[id].group == -1 { return; }
    let n = shared(w, id, GROUP_COUNT);
    set_shared(w, id, GROUP_COUNT, n - 1);
    pass_turn(w, id);
}

/// `0x427590` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    floor(w, id, ctx);
    w.mobys[id].scale = w.class(CLASS).map_or(1.0, |c| c.scale);
    // `0x427518`: a hit from these classes counts half.
    if let Ok(s) = usize::try_from(w.mobys[id].hit_slot) {
        let src = w.hits.records.get(s).filter(|r| r.target == id).and_then(|r| r.tmpl.source).and_then(|k| w.mobys.get(k)).map(|m| m.o_class);
        if matches!(src, Some(0x0a66 | 0x0a82 | 0x0cbb | 0x0cf3)) { w.hits.records[s].tmpl.damage *= 0.5; }
    }
    let r = ai::take_damage(w, id, ctx, false);
    if r == 0 { return; }
    if r == 1 {
        let m = &mut w.mobys[id];
        let v = sub(m.pos, ctx.hero.pos);
        m.set_pv_v(PUSH, set_len(v, 0.166_666_67));
    } else {
        if r == 4 {
            let m = &mut w.mobys[id];
            m.set_pv_b(0xc6, 0);
            m.set_pv_i(0xcc, -1);
        }
        pass_turn(w, id);
    }
    // `0x426ab0`.
    let m = &w.mobys[id];
    let (g, a, b) = (m.group as i32, m.pv_i(POKE), m.pv_i(POKE_B));
    if g != -1 { w.poke_group(g, 1); }
    if a != -1 { w.poke_group(a, 1); }
    if b != -1 { w.poke_group(a, 1); }
    w.mobys[id].timer = 1;
}

/// `0x3d51d8`: its cover, at its place with its distances.
fn cover(w: &mut MobyWorld, id: usize, ctx: &Ctx) -> Option<usize> {
    w.class(COVER)?;
    let n = w.create(COVER);
    let (ud, dd, pos) = (w.mobys[id].update_dist, w.mobys[id].draw_dist, w.mobys[id].pos);
    let q = &mut w.mobys[n];
    (q.update_dist, q.draw_dist, q.drawn, q.state, q.pos) = (ud, dd, 1, 0, pos);
    ai::set_link(q, 0, Some(id));
    w.rebuild(n, &*ctx.host);
    w.changed = true;
    Some(n)
}

/// `0x427110` (module docs: its home, distances, reaction block, floor, cover and the group's turns).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_pv_b(0x48, 40);
    m.set_pv_b(0x4a, 64);
    m.set_pv_b(0x29, 1);
    let p = m.pos;
    m.set_pv_v(HOME, p);
    let h = m.rot[2];
    m.set_pv_f(HEADING, h);
    floor(w, id, ctx);
    if w.mobys[id].group != -1 && shared(w, id, GROUP_STAMP) != STAMP {
        set_shared(w, id, GROUP_COUNT, 0);
        set_shared(w, id, GROUP_TURN, 0);
        set_shared(w, id, GROUP_STAMP, STAMP);
    }
    let m = &mut w.mobys[id];
    let ud = m.update_dist;
    m.set_pv_b(AWAKE_DIST, ud);
    let v = first_vols(w, id);
    let f = volume_reach(ctx.level, p, v.0, v.1, v.2, v.3);
    if 0.0 < f { w.mobys[id].update_dist = (f + 2.0) as i32 as u8; }
    let m = &mut w.mobys[id];
    let ud = m.update_dist;
    m.set_pv_b(AWAKE_DIST, ud);
    m.set_pv_b(UNDER_DIST, ud);
    let v = roam_vols(w, id);
    let f = volume_reach(ctx.level, p, v.0, v.1, v.2, v.3);
    let m = &mut w.mobys[id];
    if 0.0 < f { m.set_pv_b(AWAKE_DIST, (f + 2.0) as i32 as u8); }
    for (o, b) in [(0x60, 5u8), (0x61, 8), (0x62, 8), (0x66, 10), (0x67, 11), (0xbc, 7), (0xbd, 24), (0xbe, 6), (0xbf, 13), (0xc2, 7), (0xc3, 14), (0xc6, 1), (0xc4, 0), (0xc5, 0), (0xc7, 1), (0x2e, 3), (0x28, 1)] { m.set_pv_b(o, b); }
    m.set_pv_i(0x98, -1);
    for (o, x) in [(0x6c, 0.383_333_36f32), (0x70, 0.233_333_35), (0x74, 0.004_166_667), (0x78, 0.008_333_334), (0x7c, 0.002_777_778), (0x80, 2.75), (0x84, 0.0), (0x8c, 0.4), (0x30, 0.0)] { m.set_pv_f(o, x); }
    let path = m.pv_i(0x1e4);
    m.set_pv_i(0xcc, path);
    m.set_pv_f(SWEEP_WAY, 1.0);
    m.set_pv_v(VEL, [0.0; 3]);
    m.set_pv_v(GOAL, p);
    m.set_pv_v(PUSH, [0.0; 3]);
    if m.pv_f(FLOOR) == 0.0 {
        let low = m.pv_f(0x90);
        let (h, _) = probe(ctx, p, p[2] + 20.0, low, low);
        let m = &mut w.mobys[id];
        m.set_pv_f(FLOOR, h);
        let home = m.pv_f(HOME + 8);
        m.set_pv_f(HEIGHT, if h <= home { h } else { home });
    }
    let m = &mut w.mobys[id];
    if 0.0 < m.pv_f(FLOOR) { m.pos[2] = m.pv_f(FLOOR) - 0.5; }
    let c = cover(w, id, ctx);
    let m = &mut w.mobys[id];
    ai::set_link(m, COVER_AT, c);
    m.set_pv_s(RISEN, 0);
    let wake = m.pv_b(WAKE);
    if wake == 1 || wake == 4 { m.set_pv_b(WAKE, wake + 1); }
    let p = m.pos;
    m.set_pv_v(HOME, p);
    m.mode &= !crate::moby::mode::TARGET;
    m.set_state(1, -1);
    if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 2); }
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.drawn = 0;
    m.mode |= 1;
    let dd = m.draw_dist as i32;
    let a = m.pv_b(AWAKE_DIST) as i32;
    m.set_pv_b(AWAKE_DIST, a.max(dd) as u8);
    if w.mobys[id].group != -1 {
        let n = shared(w, id, GROUP_COUNT);
        let m = &mut w.mobys[id];
        if (m.pv_b(MY_TURN) as i32) < n { m.set_pv_b(MY_TURN, n as u8); }
        set_shared(w, id, GROUP_COUNT, n + 1);
    }
    w.changed = true;
}

/// A moby of the level's table by its index (−1 none).
fn moby_at(w: &MobyWorld, i: i32) -> Option<&crate::moby::Moby> { usize::try_from(i).ok().and_then(|i| w.mobys.get(i)) }
/// `0x426b38`: none, or gone.
fn gone(w: &MobyWorld, i: i32) -> bool { i == -1 || moby_at(w, i).is_none_or(|m| matches!(m.state, 0xfe | 0xfd)) }
/// `0x426b80`: none, past its state 2, or gone.
fn past(w: &MobyWorld, i: i32) -> bool { i == -1 || moby_at(w, i).is_none_or(|m| 3 <= m.state || matches!(m.state, 0xfe | 0xfd)) }

/// State 1 (module docs).
fn wake(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let range = w.mobys[id].draw_dist as f32;
    let mut ready = false;
    match w.mobys[id].pv_b(WAKE) {
        0 | 1 => {
            if ctx.game.rng.randi(7) == 0 {
                let v = first_vols(w, id);
                find_target(w, id, ctx, T_BLOCK, range, 0.0, v);
                if w.mobys[id].pv_i(T_RESULT) != 2 { ready = in_view(w, id, ctx, range); }
            }
        }
        2 => {
            if w.mobys[id].timer != 0 {
                w.mobys[id].timer = 0;
                ready = true;
            }
        }
        3 => ready = gone(w, w.mobys[id].pv_i(WAITS_FOR)),
        5 => ready = past(w, w.mobys[id].pv_i(ANCHOR)),
        _ => {}
    }
    if w.mobys[id].timer & 2 != 0 {
        w.mobys[id].timer = 0;
        ready = true;
    }
    if !ready || !in_view(w, id, ctx, range) { return; }
    let m = &mut w.mobys[id];
    m.update_dist = m.pv_b(AWAKE_DIST);
    m.set_pv_v(VEL, [0.0; 3]);
    anim(w, id, ctx, 1, 10);
    let m = &mut w.mobys[id];
    m.drawn = 1;
    m.mode &= !1;
    m.set_state(2, -1);
    let r = &mut ctx.game.shakes.0[2];
    (r.amp, r.timer, r.value) = (0.25, 38, 0.0);
    if let Some(c) = ai::link(&w.mobys[id], COVER_AT) {
        if let Some(c) = w.mobys.get_mut(c) { c.timer = 1; }
        ai::set_link(&mut w.mobys[id], COVER_AT, None);
    }
    w.changed = true;
}

/// `0x426660(m, at, n, speed, rise, far, near, size, end)`: `n` sand puffs about `at` (as tabora_ai::puffs, 0.75 lower,
/// 0x70204050 → 0x7f7f7f, 15..25 in, 20..40 easing, 50..70 out); in the knock (state 5) they take 0.8 of its flight's
/// velocity (`+0xd0`) and 0.4 of that as their ease.
#[allow(clippy::too_many_arguments)]
pub fn puffs(w: &MobyWorld, id: usize, ctx: &mut Ctx, at: V3, n: i32, speed: f32, rise: f32, far: f32, near: f32, size: f32, end: f32) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let m = &w.mobys[id];
    let (rows, knock, flight) = (m.rows, m.state == 5, m.pv_v(0xd0));
    let g = &mut ctx.game.rng;
    for _ in 0..n {
        let x = g.randf(-PI, PI).cos() * g.randf(near, far);
        let y = g.randf(-PI, PI).sin() * g.randf(near, far);
        let off = crate::gravity::to_world([x, y, 0.0], &rows);
        let range = speed * ((far - len(off)) * 0.016_666_668) / far;
        let mut v = set_len(off, g.randf(range * 0.25, range));
        v[2] += g.randf(rise * 0.25, rise) * 0.016_666_668;
        let mut ease = [0.0; 3];
        if knock {
            let motion = scale(flight, 0.8);
            v = add(v, motion);
            ease = scale(motion, 0.4);
        }
        let mut p = add(off, at);
        p[2] -= 0.75;
        let a = g.randf(15.0, 25.0) as i32 as i16;
        let b = g.randf(20.0, 40.0) as i32 as i16;
        let c = g.randf(50.0, 70.0) as i32 as i16;
        let made = crate::particles::staged(sys, g, p, [v[0], v[1], v[2], size], [ease[0], ease[1], ease[2], end], 0x7020_4050, 0x7f_7f7f, a, b, c, -1);
        if let Some(i) = made {
            if g.rand() & 1 != 0 { sys.pool.recs[i][3] = 0x48; }
        }
    }
}

fn low_puffs(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let h = m.pv_f(FLOOR);
    if m.pos[2] < h + 1.5 {
        let at = [m.pos[0], m.pos[1], h];
        puffs(w, id, ctx, at, 8, 10.0, 4.0, 4.0, 0.0, 0.25, 3.5);
    }
}

/// `0x426c90`: a goal outside the roam's volumes taken back: half the time (always when nothing is targeted or after an
/// attack) home, or 3 about the moby it guards; else Ratchet. 6.5 up.
fn keep_goal(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let v = roam_vols(w, id);
    if in_volumes(ctx.level, m.pv_v(GOAL), v.0, v.1, v.2, v.3) != 0 { return; }
    let g = &mut ctx.game.rng;
    let mut goal = if g.rand() & 1 != 0 || m.pv_i(T_RESULT) == 2 || m.pv_s(AFTER) != 0 {
        let anchor = m.pv_i(ANCHOR);
        if anchor == -1 || g.rand() & 1 != 0 {
            m.pv_v(HOME)
        } else {
            let x = g.randf(-PI, PI).cos() * 3.0;
            let y = g.randf(-PI, PI).sin() * 3.0;
            add([x, y, 0.0], moby_at(w, anchor).map_or([0.0; 3], |a| a.pos))
        }
    } else {
        ctx.hero.pos
    };
    goal[2] += 6.5;
    w.mobys[id].set_pv_v(GOAL, goal);
}

/// `0x426e08`: a goal a random way within its roam's radius of home, 6.5 up; after an attack 3 more away from
/// Ratchet; kept ([`keep_goal`]).
fn new_goal(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let r = m.pv_b(UNDER_DIST) as f32;
    let g = &mut ctx.game.rng;
    let x = g.randf(-PI, PI).cos() * r;
    let y = g.randf(-PI, PI).sin() * r;
    let mut goal = add([x, y, 0.0], m.pv_v(HOME));
    goal[2] += 6.5;
    if m.pv_s(AFTER) != 0 {
        let mut d = sub(goal, ctx.hero.pos);
        d[2] = 0.0;
        goal = add(goal, set_len(d, 3.0));
    }
    w.mobys[id].set_pv_v(GOAL, goal);
    keep_goal(w, id, ctx);
}

/// `0x351560(out, p, path)`: `p` inside the path's polygon as it is, else the nearest point of its edges.
fn into_path(ctx: &Ctx, p: V3, path: i32) -> V3 {
    if ctx.level.in_polygon(path, p) { return p; }
    let Some(pts) = crate::path_network::points_now(ctx.level, &ctx.game.path_openings, path) else { return p };
    let n = pts.len();
    let mut best = (f32::MAX, p);
    for i in 0..n {
        let a = [pts[i][0], pts[i][1], pts[i][2]];
        let b = [pts[(i + 1) % n][0], pts[(i + 1) % n][1], pts[(i + 1) % n][2]];
        let ab = sub(b, a);
        let l2 = ab[0] * ab[0] + ab[1] * ab[1] + ab[2] * ab[2];
        let t = if 0.0 < l2 { (((p[0] - a[0]) * ab[0] + (p[1] - a[1]) * ab[1] + (p[2] - a[2]) * ab[2]) / l2).clamp(0.0, 1.0) } else { 0.0 };
        let near = add(a, scale(ab, t));
        let d = len(sub(p, near));
        if i == 0 || d < best.0 { best = (d, near); }
    }
    best.1
}

/// `0x426f38`: its goal 15 across from its target on its own side — with a guarded moby within 9 across of the
/// target, on the guarded moby's side, turned 1 aside — kept to its path, at its height.
fn goal_at(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let t = m.pv_v(T_BLOCK);
    let anchor = m.pv_i(ANCHOR);
    let mut goal = match moby_at(w, anchor).filter(|_| anchor != -1) {
        None => {
            let mut v = set_len(sub(m.pos, t), 15.0);
            v[2] = 0.0;
            add(v, t)
        }
        Some(a) => {
            let mut v0 = sub(a.pos, t);
            v0[2] = 0.0;
            if len(v0) < 9.0 {
                let mut v1 = sub(m.pos, t);
                v1[2] = 0.0;
                let v1 = set_len(v1, 15.0);
                let v3 = set_len(cross(v0, cross(v0, v1)), 1.0);
                add(add(v1, t), v3)
            } else {
                add(set_len(v0, 15.0), t)
            }
        }
    };
    let path = m.pv_i(0x1e4);
    if path != -1 { goal = into_path(ctx, goal, path); }
    goal[2] = m.pv_f(HEIGHT);
    w.mobys[id].set_pv_v(GOAL, goal);
}

/// `0x427650(m, accel, top, turn, blend, pitch)`: the speed eased toward `top` a second (braking within its stop of
/// the goal: true), the velocity toward the goal plus the push, the move against the world 2 up (a reach of the
/// risen ticks / 20, at most 6), the heading and pitch sprung toward `turn` / `pitch`, the push cut to 0.95; the move
/// made at most 0.133 long.
fn steer(w: &mut MobyWorld, id: usize, ctx: &Ctx, accel: f32, top: f32, turn: f32, blend: f32, pitch: f32) -> bool {
    let m = &w.mobys[id];
    let old = m.pos;
    let top = top * 0.016_666_668;
    let accel = accel * 0.000_277_777_78;
    let dir = sub(m.pv_v(GOAL), m.pos);
    let d = len(dir);
    let mut v = len(m.pv_v(VEL));
    let braking = d < v * (v / accel);
    if braking { v -= accel } else {
        let k = 1.0 - (top - v) / top;
        v += accel * (1.0 - k * k);
    }
    let dir = set_len(dir, v);
    let push = m.pv_v(PUSH);
    let vel = add(add(scale(m.pv_v(VEL), blend), scale(dir, 1.0 - blend)), push);
    let reach = ((m.pv_s(RISEN) as f32 * 6.0) / 120.0).min(6.0);
    let mut vel = vel;
    w.mobys[id].pos[2] += 2.0;
    crate::moby::slide::slide(w, id, ctx, 0.0, reach, 0.0, &mut vel, 0);
    let m = &mut w.mobys[id];
    m.pos[2] -= 2.0;
    m.set_pv_v(VEL, sub(vel, push));
    let (h, hv) = spring_angle(m.pv_f(HEADING), turn, 0.000_581_776_5, 0.001_745_329_5, 0.034_906_59, m.pv_f(TURN_V));
    m.set_pv_f(HEADING, h);
    m.set_pv_f(TURN_V, hv);
    let (a, av) = spring_angle(m.rot[1], pitch, 0.000_484_813_73, 0.001_745_329_5, 0.034_906_59, m.pv_f(PITCH_V));
    m.rot[1] = a;
    m.set_pv_f(PITCH_V, av);
    m.set_pv_v(PUSH, scale(push, 0.95));
    m.pos = add(old, at_most(sub(m.pos, old), 0.133_333_34));
    w.changed = true;
    braking
}

fn to_target(w: &MobyWorld, id: usize) -> f32 {
    let m = &w.mobys[id];
    let t = m.pv_v(T_BLOCK);
    atan2(t[0] - m.pos[0], t[1] - m.pos[1])
}

/// The hunt's look: when Ratchet was seen and is not unseen, the hunt (sub 2) at full distance.
fn start_hunt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    if m.pv_i(T_RESULT) == 2 || m.pv_s(UNSEEN) != 0 { return; }
    if seq(w, id, ctx) != 6 { let b = ctx.game.rng.rand_range(15, 30) as u8; ctx.host.set_anim(&w.mobys[id], 6, 0, b); }
    let m = &mut w.mobys[id];
    m.update_dist = 255;
    m.set_state(4, 2);
}

fn set_sub_keep_ticks(w: &mut MobyWorld, id: usize, sub: i32) {
    let t = w.mobys[id].state_ticks;
    w.mobys[id].set_state(4, sub);
    w.mobys[id].state_ticks = t;
}

/// State 4 (module docs).
fn roam(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.rng.randi(7) == 0 {
        let range = w.mobys[id].draw_dist as f32;
        let v = roam_vols(w, id);
        find_target(w, id, ctx, T_BLOCK, range, 60.0, v);
    }
    if dist2(w.mobys[id].pos, w.mobys[id].pv_v(T_BLOCK)) < 8.0 { w.mobys[id].set_pv_s(NEAR, 120); }
    match w.mobys[id].sub_state {
        1 => {
            if w.mobys[id].pv_s(WAIT) == 0 {
                let t = ctx.game.rng.rand_range(240, 480) as i16;
                w.mobys[id].set_pv_s(WAIT, t);
                if seq(w, id, ctx) != 6 { let b = ctx.game.rng.rand_range(15, 30) as u8; ctx.host.set_anim(&w.mobys[id], 6, 0, b); }
                set_sub_keep_ticks(w, id, 0);
            }
            start_hunt(w, id, ctx);
            return;
        }
        0 => {
            let yaw = to_target(w, id);
            if steer(w, id, ctx, 5.5, 5.5, yaw, 0.0, 0.0) { w.mobys[id].set_pv_s(WAIT, 0); }
            if w.mobys[id].pv_s(WAIT) == 0 {
                if (w.mobys[id].state_ticks as u16) < 60 {
                    new_goal(w, id, ctx);
                    let t = ctx.game.rng.rand_range(90, 180) as i16;
                    w.mobys[id].set_pv_s(WAIT, t);
                    let s = (ctx.game.rng.rand() & 1) as u8 + 2;
                    if seq(w, id, ctx) != s { let s = (ctx.game.rng.rand() & 1) as u8 | 2; ctx.host.set_anim(&w.mobys[id], s, 0, 30); }
                    set_sub_keep_ticks(w, id, 1);
                } else {
                    let m = &mut w.mobys[id];
                    m.set_pv_s(WAIT, 32767);
                    let home = m.pv_v(HOME);
                    m.set_pv_v(GOAL, home);
                    let z = m.pos[2];
                    m.set_pv_f(GOAL + 8, z);
                    m.set_state(4, 4);
                }
            }
            start_hunt(w, id, ctx);
        }
        4 => {
            let yaw = to_target(w, id);
            if steer(w, id, ctx, 5.5, 5.5, yaw, 0.0, 0.0) {
                if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 180); }
                w.mobys[id].set_state(3, -1);
            }
            start_hunt(w, id, ctx);
        }
        2 => hunt(w, id, ctx),
        3 => attack(w, id, ctx),
        _ => return,
    }
    let m = &mut w.mobys[id];
    m.rot[2] = m.pv_f(HEADING);
}

/// Sub 2 (module docs).
fn hunt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    goal_at(w, id, ctx);
    let m = &mut w.mobys[id];
    let hh = m.pv_f(HEIGHT);
    m.set_pv_f(GOAL + 8, hh);
    let yaw = to_target(w, id);
    if down(w, id, NEAR) { steer(w, id, ctx, 5.5, 5.5, yaw, 0.0, 0.0); } else { steer(w, id, ctx, 32.0, 12.0, yaw, 0.0, 0.0); }
    if wrapped(w, id, ctx) {
        if len(w.mobys[id].pv_v(VEL)) < 0.016_666_668 {
            let s = (ctx.game.rng.rand() & 1) as u8 + 4;
            if seq(w, id, ctx) != s { let s = (ctx.game.rng.rand() & 1) as u8 | 4; ctx.host.set_anim(&w.mobys[id], s, 0, 10); }
        } else {
            anim(w, id, ctx, 6, 10);
        }
    }
    let resume = if w.mobys[id].pv_i(T_RESULT) == 2 {
        down(w, id, LOST)
    } else if 240 <= w.mobys[id].pv_s(UNSEEN) {
        true
    } else {
        w.mobys[id].set_pv_s(LOST, 240);
        let m = &w.mobys[id];
        let not_mine = m.group != -1 && shared(w, id, GROUP_TURN) != m.pv_b(MY_TURN) as i32;
        if !not_mine && down(w, id, AFTER) && w.beams.claim(Kind::Tabora, id, None, 1, 0) && w.mobys[id].pv_s(UNSEEN) == 0 {
            let m = &mut w.mobys[id];
            m.set_pv_f(SWEEP, -FRAC_PI_4);
            m.set_pv_f(SWEEP_WAY, 1.0);
            m.set_pv_i(0x204, 0);
            m.set_pv_i(POWER, 0);
            m.set_pv_b(SWEEPS, 0);
            if seq(w, id, ctx) != 6 { let b = ctx.game.rng.rand_range(15, 30) as u8; ctx.host.set_anim(&w.mobys[id], 6, 0, b); }
            let m = &mut w.mobys[id];
            m.set_state(4, 3);
            m.set_pv_b(STEP, 0);
            m.update_dist = 255;
        }
        false
    };
    if resume {
        let s = (ctx.game.rng.rand() & 2) as u8 + 2;
        if seq(w, id, ctx) != s { let s = (ctx.game.rng.rand() & 2) as u8 + 2; ctx.host.set_anim(&w.mobys[id], s, 0, 10); }
        let m = &mut w.mobys[id];
        m.update_dist = m.pv_b(AWAKE_DIST);
        m.set_state(4, 0);
    }
}

/// Sub 3 (module docs).
fn attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    goal_at(w, id, ctx);
    let angle = to_target(w, id);
    let m = &w.mobys[id];
    let at = m.pv_v(T_AIM);
    let pitch = -atan2(dist2(m.pos, at), at[2] - m.pos[2]);
    if down(w, id, NEAR) { steer(w, id, ctx, 5.5, 5.5, angle, 0.0, pitch); } else { steer(w, id, ctx, 32.0, 12.0, angle, 0.0, pitch); }
    let mut aim_now = false;
    match w.mobys[id].pv_b(STEP) {
        0 => {
            if rot_dist(angle, w.mobys[id].pv_f(HEADING)) < 0.698_131_8 {
                let m = &mut w.mobys[id];
                m.set_pv_f(SWEEP, -FRAC_PI_4);
                m.set_pv_f(SWEEP_WAY, 1.0);
                if seq(w, id, ctx) != 7 { let b = ctx.game.rng.rand_range(8, 15) as u8; ctx.host.set_anim(&w.mobys[id], 7, 0, b); }
                w.mobys[id].set_pv_b(STEP, 1);
            }
        }
        1 => {
            let m = &mut w.mobys[id];
            let p = m.pv_f(POWER) + 0.01;
            m.set_pv_f(POWER, p);
            if 1.0 <= p {
                m.set_pv_f(POWER, 1.0);
                m.set_pv_b(STEP, 2);
                m.set_pv_b(SWEEPS, 0);
            }
            aim_now = true;
        }
        2 => {
            let m = &mut w.mobys[id];
            let s = add_rot(m.pv_f(SWEEP), m.pv_f(SWEEP_WAY) * 0.020_362_178);
            m.set_pv_f(SWEEP, s);
            let way = m.pv_f(SWEEP_WAY);
            if (0.0 < way && FRAC_PI_4 <= s) || (way <= 0.0 && s <= -FRAC_PI_4) {
                m.set_pv_s(PAUSE, 0);
                m.set_pv_f(SWEEP_WAY, -way);
                let n = m.pv_b(SWEEPS).wrapping_add(1);
                m.set_pv_b(SWEEPS, n);
            }
            let next = if 3 <= m.pv_b(SWEEPS) && m.pv_s(NEAR) == 0 { 4 } else { 3 };
            m.set_pv_b(STEP, next);
            aim_now = true;
        }
        3 => {
            if down(w, id, PAUSE) { w.mobys[id].set_pv_b(STEP, 2); }
            aim_now = true;
        }
        4 => {
            let m = &mut w.mobys[id];
            let p = m.pv_f(POWER) - 0.01;
            m.set_pv_f(POWER, p);
            if p <= 0.01 {
                m.set_pv_b(SWEEPS, 0);
                m.set_pv_b(STEP, 5);
            }
            aim_now = true;
        }
        5 => {
            pass_turn(w, id);
            beams::Beams::release(w, Kind::Tabora, id);
            let m = &mut w.mobys[id];
            m.set_pv_i(0x204, 0);
            m.set_pv_s(AFTER, 90);
            if seq(w, id, ctx) != 6 { let b = ctx.game.rng.rand_range(8, 15) as u8; ctx.host.set_anim(&w.mobys[id], 6, 0, b); }
            w.mobys[id].set_state(4, 0);
        }
        _ => {}
    }
    if aim_now { aim(w, id, ctx); }
    let m = &w.mobys[id];
    if m.pv_i(T_RESULT) == 2 || 240 <= m.pv_s(UNSEEN) {
        pass_turn(w, id);
        beams::Beams::release(w, Kind::Tabora, id);
        w.mobys[id].set_pv_s(AFTER, 0);
        if seq(w, id, ctx) != 6 { let b = ctx.game.rng.rand_range(8, 15) as u8; ctx.host.set_anim(&w.mobys[id], 6, 0, b); }
        let m = &mut w.mobys[id];
        m.update_dist = m.pv_b(AWAKE_DIST);
        m.set_state(4, 0);
    } else {
        w.mobys[id].set_pv_s(LOST, 240);
    }
}

/// `0x427928`: the arc — from joint 4 (0.7 out along its row 0) toward a point leaning from 8 ahead along the sweep
/// (on the ground, 0.6 up) to Ratchet's (`+0x1cc`, 0.05 a tick toward him while he is near), at his aim's height,
/// `power · 20` long; reaching him, a hit along the line through him (its damage `+0x34`, flags 1, 2 long on the ground
/// 0.6 up; again 0.5 lower when it meets nothing); the beam reported (crate::fx::beams).
fn aim(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let speed = m.pv_f(POWER) * 20.0;
    let mut origin = m.pv_v(T_BLOCK);
    origin[2] = ai::ground_z(ctx, 6.0, origin, 0) + OVER;
    let a = add_rot(m.pv_f(HEADING), m.pv_f(SWEEP));
    let mut desired = add([a.cos() * 8.0, a.sin() * 8.0, 0.0], m.pos);
    desired[2] = ai::ground_z(ctx, 0.5, desired, 0) + OVER;
    let near = m.pv_s(NEAR) != 0;
    let m = &mut w.mobys[id];
    let lean = (m.pv_f(LEAN) + if near { 0.05 } else { -0.05 }).clamp(0.0, 1.0);
    m.set_pv_f(LEAN, lean);
    let mut target = add(scale(desired, 1.0 - lean), scale(origin, lean));
    let Some((rows, jp)) = ctx.host.joint_matrix(&w.mobys[id], 4) else { return };
    let mp = add(jp, set_len(rows[0], 0.7));
    let m = &w.mobys[id];
    let aim_z = m.pv_f(T_AIM + 8);
    target[2] = aim_z;
    let mut flat = sub(target, mp);
    flat[2] = 0.0;
    let flat = set_len(flat, speed);
    let mut desired = add(flat, mp);
    desired[2] = aim_z;
    let desired = add(set_len(sub(desired, mp), speed), mp);
    let distance = dist2(m.pv_v(T_BLOCK), mp);
    if distance < speed {
        let rz = m.rot[2];
        let mut t = crate::moby::hits::HitTemplate::new(m.pv_b(0x34) as f32, Some(id), 1, [rz.cos(), rz.sin(), 1.0, 5_627.925]);
        t.kind = (m.o_class as u16 as u32) << 16 | 1 << 8;
        let mut start = add(set_len(flat, distance - 1.0), mp);
        let mut end = add(set_len(flat, distance + 1.0), mp);
        let z = ai::ground_z(ctx, 0.5, m.pv_v(T_AIM), 0) + OVER;
        start[2] = z;
        end[2] = z;
        if !line_hit(w, id, ctx, start, end, &t) {
            start[2] -= 0.5;
            end[2] -= 0.5;
            line_hit(w, id, ctx, start, end, &t);
        }
    }
    let r = rows;
    w.beams.report_tabora(id, [r[0], r[1], r[2], mp], desired);
}

/// `0x2de878(a, b, 0, m, hit)`: the line test (the moby itself left out), its hit delivered to the moby it meets.
fn line_hit(w: &mut MobyWorld, id: usize, ctx: &Ctx, a: V3, b: V3, t: &crate::moby::hits::HitTemplate) -> bool {
    let Some(o) = w.coll_line(ctx, a, b, 0, Some(id)) else { return false };
    if let Some(k) = o.moby { w.deliver_hit_any(k, t, [o.point[0], o.point[1], o.point[2], 1.0]); }
    true
}

/// State 5 (module docs).
fn knock(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.update_dist = 255;
    m.draw_dist = 255;
    beams::Beams::release(w, Kind::Tabora, id);
    let m = &mut w.mobys[id];
    m.set_pv_s(KNOCK, m.pv_s(KNOCK).wrapping_add(1));
    let flags = ai::flight(w, id, ctx);
    let m = &mut w.mobys[id];
    let h = m.rot[2];
    m.set_pv_f(HEADING, h);
    if flags & 1 != 0 {
        m.set_state(6, -1);
    } else if flags & 2 != 0 || 115 <= m.pv_s(KNOCK) {
        if 0.0 < m.pv_f(HEALTH) {
            m.set_pv_v(PUSH, [0.0; 3]);
            m.set_pv_b(0x6a, 2);
            anim(w, id, ctx, 4, 40);
            let m = &mut w.mobys[id];
            m.set_pv_s(KNOCK, 0);
            m.set_pv_v(VEL, [0.0; 3]);
            m.update_dist = m.pv_b(AWAKE_DIST);
            m.set_state(4, 2);
            pass_turn(w, id);
        } else {
            m.set_state(6, -1);
        }
    }
    let m = &w.mobys[id];
    if flags & 4 != 0 || m.pos[2] < m.pv_f(FLOOR) + 1.5 {
        let m = &mut w.mobys[id];
        m.set_pv_s(KNOCK, m.pv_s(KNOCK).wrapping_add(1));
        let at = [m.pos[0], m.pos[1], m.pv_f(FLOOR)];
        puffs(w, id, ctx, at, 10, 14.0, 6.0, 5.5, 0.0, 0.5, 4.0);
    }
}

/// States 6 / 7 (module docs); false when its crystal's class is not loaded (it stays).
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    beams::Beams::release(w, Kind::Tabora, id);
    let left = super::crystal::LEFT;
    if w.class(left).is_none() { return false; }
    let n = w.create(left);
    let (pos, floor) = (w.mobys[id].pos, w.mobys[id].pv_f(FLOOR));
    let lit = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map(|h| (h.light, h.ambient));
    let q = &mut w.mobys[n];
    (q.alpha, q.update_dist, q.draw_dist, q.drawn, q.state, q.timer, q.pos) = (80, 255, 255, 1, 0, 0, pos);
    q.mode |= 0x800;
    if floor != 0.0 { q.pos[2] = floor; }
    ai::set_link(q, 4, Some(id));
    if let Some((light, ambient)) = lit { (q.light, q.ambient) = (light, ambient); }
    w.rebuild(n, &*ctx.host);
    if w.mobys[id].state == 7 {
        let e = crate::moby::blast::Explosion { embers: 20, billows: 6, flares: 32, flash: 8.0, flash2: 4.0, near: 9.0, size: 2.0, sound: -1, light: 30.0, shake: true, debris: 2, lod: -1, colour: 0, flags: 0x81_0001, ..Default::default() };
        crate::moby::blast::explode(w, ctx, Some(id), None, &e);
    }
    leave_turn(w, id);
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
    let uid = w.mobys[id].uid;
    let u = uid.max(0) as usize;
    if let Some(p) = ctx.game.uid_pending.get_mut(u) { *p = 0; }
    if let Some(d) = ctx.game.uid_dead.get_mut(u) { *d = 0; }
    ctx.game.clear_uid(uid);
    w.delete(id);
    w.changed = true;
    true
}

/// `0x426bd0` (module docs).
fn tail(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) < 0x7ffd { m.state_ticks = m.state_ticks.wrapping_add(1); }
    if m.drawn == 0 { m.set_pv_s(UNSEEN, m.pv_s(UNSEEN).wrapping_add(1)); } else { m.set_pv_s(UNSEEN, 0); }
    down(w, id, WAIT);
    let m = &mut w.mobys[id];
    if 3 <= m.state && m.pv_s(RISEN) < 120 { m.set_pv_s(RISEN, m.pv_s(RISEN) + 1); }
    if 2 <= m.state { m.shadow_slab = [m.pv_f(FLOOR) - m.pos[2], 2.0]; }
}

/// `0x427d98` (module docs).
pub fn update_1002(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !ctx.level.in_variant(w.mobys[id].pos, ctx.game.variant) { return; }
    if w.mobys[id].pvars.len() < 0x230 { w.mobys[id].pvars.resize(0x230, 0); }
    if w.mobys[id].state == 0 {
        if ctx.game.uid_count(w.mobys[id].uid) != 0 {
            w.delete(id);
            return;
        }
        setup(w, id, ctx);
    }
    let m = &w.mobys[id];
    if (m.update_dist as f32) < dist2(m.pos, ctx.host.camera_pos()) { return; }
    head(w, id, ctx);
    match w.mobys[id].state {
        1 => wake(w, id, ctx),
        2 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 0, 0); }
            let m = &mut w.mobys[id];
            let (mut z, mut v) = (m.pos[2], m.pv_f(RISE_V));
            crate::hero::physics::ease(m.pv_f(HEIGHT), 0.008_333_334, 0.004_166_667, 0.533_333_36, &mut z, &mut v);
            m.pos[2] = z;
            m.set_pv_f(RISE_V, v);
            low_puffs(w, id, ctx);
            if wrapped(w, id, ctx) {
                let m = &mut w.mobys[id];
                m.has_collision = true;
                m.mode |= crate::moby::mode::TARGET;
                new_goal(w, id, ctx);
                let s = (ctx.game.rng.rand() & 1) as u8 + 4;
                if seq(w, id, ctx) != s { let s = (ctx.game.rng.rand() & 1) as u8 | 4; ctx.host.set_anim(&w.mobys[id], s, 0, 10); }
                w.mobys[id].set_state(4, 0);
            }
            w.changed = true;
        }
        3 => {
            let m = &mut w.mobys[id];
            let (mut z, mut v) = (m.pos[2], m.pv_f(RISE_V));
            crate::hero::physics::ease(m.pv_f(FLOOR) - 1.5, 0.008_333_334, 0.004_166_667, 0.533_333_36, &mut z, &mut v);
            m.pos[2] = z;
            m.set_pv_f(RISE_V, v);
            let mut d = scale(sub(m.pv_v(HOME), m.pos), 0.025);
            d[2] = 0.0;
            m.pos = add(m.pos, d);
            low_puffs(w, id, ctx);
            let m = &w.mobys[id];
            let h = m.pv_f(FLOOR);
            let landed = ctx.host.anim(m).is_none_or(|a| a.seq_a == 0);
            if m.pos[2] < h - 1.4 && landed {
                let at = m.pos;
                puffs(w, id, ctx, at, 8, 10.0, 4.0, 2.0, 0.0, 0.25, 3.5);
                let m = &mut w.mobys[id];
                m.has_collision = false;
                m.mode &= !crate::moby::mode::TARGET;
                m.pos[2] = h;
                if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 2); }
                let m = &mut w.mobys[id];
                m.set_state(1, -1);
                m.update_dist = m.pv_b(UNDER_DIST);
            }
            w.changed = true;
        }
        4 => roam(w, id, ctx),
        5 => knock(w, id, ctx),
        6 | 7 => {
            if die(w, id, ctx) { return; }
        }
        _ => {}
    }
    tail(w, id);
    // The camera's look target (`0x3d4d80`, crate::moby::classes::look_targets): the moby it guards while its group
    // counts any, else itself.
    let m = &w.mobys[id];
    if (2..6).contains(&m.state) && m.drawn != 0 {
        let anchor = m.pv_i(ANCHOR);
        let guarded = if anchor != -1 && shared(w, id, GROUP_COUNT) != 0 { moby_at(w, anchor).filter(|a| !matches!(a.state, 0xfe | 0xfd)).map(|a| a.pos) } else { None };
        let at = guarded.unwrap_or(m.pos);
        crate::moby::classes::look_targets::offer(w, ctx.hero.pos, id, at);
    }
    let p = w.mobys[id].pos;
    if p.iter().any(|&c| !(30.0..=993.0).contains(&c)) {
        leave_turn(w, id);
        w.delete(id);
    }
}

/// `0x3d5258` (module docs).
pub fn update_0987(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].state = 1,
        1 => { if w.mobys[id].timer != 0 { w.mobys[id].state = 2; } }
        2 => {
            crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x7f, [0.0, 0.0, 0.15], 0xff);
            w.delete(id);
        }
        _ => {}
    }
}
