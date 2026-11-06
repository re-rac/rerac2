//! Level 12's inspection robots **0x1169** (`0x4257d8`, 2 placed; the decomp's `func_L12_00424370`; L12 addresses):
//! Todano's copy of Maktar's critter 0x09bc (crate::moby::classes::maktar_pads), that Ratchet drags with the Tractor
//! Beam onto the security pads 0x0a61 (the same code here, `0x3d0180`: the robot in their `[0]`) to open their doors.
//! It strolls along its path (`+0x298`) instead of a path network, kept in its pen by the laser fences 0x0fae
//! (`+0x2c4` / `+0x2c8`, crate::moby::classes::level12::fence): their walls stand while it moves.
//!
//! On crate::moby::ai's framework (pvars: its health block `+0x20`, movement block `+0x50` (`[0x18]`), the drag block
//! at `+0x140` (crate::moby::classes::tractor::drag_at: the hold distance `+0x174`, the range `+0x178`; `+0x150` its
//! place when taken), its path `+0x298`, the point it heads for `+0x2b0`, its home point `+0x2b4`, the idle's length
//! `+0x2b8`, the glow's phase `+0x2bc`, the chirp's timer `+0x2c0`, its fences `+0x2c4` / `+0x2c8` (moby indices, −1
//! none), its place `+0x2d0`, the drag's fence path `+0x360`):
//! * Every tick: gone when its uid is marked (the save's `0x21f204` or its count in `0x19b4a8`). Its fences' walls up
//!   (`0x41d260`, crate::moby::classes::level12::fence::walls_on), whether they were up before kept (`0x41d2c8`); its
//!   scale its class's (`$gp+0x3060` 1); drawn within 24 of the camera, its shadow's slab probed (range 0x16,
//!   `0x321470`); in sub-state 0, held by the Tractor Beam (`0x18a3e0`) it is dragged (5, sub 1; `+0x150` its place),
//!   else a target (mode 0x1000) while Ratchet is within `+0x178` across.
//! * **0**: the movement block's set-up (`+0x50..+0x88`, as the critter's), the point 0, at its path's first point,
//!   1, updated within 0x80, a target.
//! * **1**: idles: on its first tick a sequence of the two idles (`randi(2)`; set, `randi(2)` again, when it is not the
//!   one it plays) and its length (30 when more than 24 across from its home point, else 180); past it, home (4) when
//!   short of its home point and more than 20 from it, else a stroll (2).
//! * **2**, the stroll: on its first tick sequence 4 (blend 0x14), the next point ([`next_point`], toward home), the
//!   walk's limits (`0x331d30` = L00 `0x32dcf0`, crate::moby::ai::senses: 0.0011, 4/60 (`$gp+0x3068`), 0.00087,
//!   0.105), the idle 900 when more than 24 from home, else 300; then it walks to the point (`0x330a08`, the framework's
//!   move toward it); there (the movement's flag 0x80) or after 300 ticks, 1.
//! * **3**, the run (let go of the beam): sequence 5, a next point (any), the limits 0.0039, 7/60 (`$gp+0x306c`),
//!   0.0035, 0.418; there, a next point toward home; after 420 ticks, 1.
//! * **4**, home: sequence 5, the point one step toward home (`0x322588` = L00 `0x31eb70`), the limits 0.0022, 7/60,
//!   0.0035, 0.209; each tick the point on by one while the line from 0.5 over it to 0.5 over the next is clear
//!   (`0x2e1e20`, flags 2), short of home; it walks to the point; within 8 of home, 1.
//! * **5**, dragged: its sequence by how it turns (along its facing the drag's velocity under −1/30: 10, over 1/30:
//!   9, else 7), the chirp at once; it turns to face Ratchet (`0x331bf0` = L01 `0x323560`, crate::moby::ai::face), its
//!   fences' walls up, it is dragged (`0x329790(0.7, 0.5, 0.1, m, +0x140, +0x360)`) and chirps; hooked (`+0xbe` bit 2,
//!   by its pad) 6; let go, the run (3, sub 0) from the first of its path's points it sees ([`first_seen`]).
//! * **6**: sequence 0xa, it faces Ratchet and chirps.
//! * The chirp (`0x4268b0`): class sound 1 every `rand_range(30, 180)` ticks (`$gp+0x3080`, `0x1b2074`; `+0x2c0`).
//! * Every tick after: its fences' walls down unless they were up before it ([`fence::walls_off`]); every 16 frames
//!   the home point the farthest point past it on its path's floor (within 2 of the first's height) within 6 of
//!   Ratchet across (`0x4267b0`); drawn, its glow `+0x78` pulses (`sin` of the phase stepping 400°/s, from 0x80c0c0c0
//!   to 0x80606060, `0x32a698`); its ticks on; `+0x2d0` its place.
//!
//! Not ported: its fall (7: its velocity `+0x2a0`, gravity `$gp+0x3064` 15/3600, slowed in the cuboid `+0x29c`,
//! landing on a floor within 18° of flat, `0x2e2e80`) and its leaving (8: a teleport carrier 0x09bb over it, after 60
//! ticks its uid marked gone and it deleted): no code enters either (as the critter's 4 and 7); the big-head cheat
//! (`0x32a7b0`, `$gp+0x3088` 2.5); its aim joint (`+0x9c` → `+0x190`, the framework's joint aims are not ported).

use super::fence;
use crate::hero::physics::{add_rot, atan2, dist2, dot, V3};
use crate::moby::classes::tractor;
use crate::moby::{ai, countdown, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x1169;

/// The drag block and the movement block.
const DRAG: usize = 0x140;
const MOVE: usize = 0x50;
const PATH: usize = 0x298;
const AT: usize = 0x2b0;
const HOME: usize = 0x2b4;
const IDLE: usize = 0x2b8;
const GLOW: usize = 0x2bc;
const CHIRP: usize = 0x2c0;
const FENCES: usize = 0x2c4;
const PLACE: usize = 0x2d0;
const FENCE_PATH: usize = 0x360;

/// `$gp+0x3068` / `+0x306c`: the walk's and the run's most, a second.
const WALK: f32 = 4.0;
const RUN: f32 = 7.0;
/// `$gp+0x308c`: how far from home a point toward home may be and still lead away.
const FAR: f32 = 16.0;

/// Gone with its uid marked (`0x21f204[uid]` ≠ 0, or its count in `0x19b4a8`).
fn uid_done(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let uid = w.mobys[id].uid;
    ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(uid) != 0
}

/// Its path's points.
fn points(w: &MobyWorld, id: usize, ctx: &Ctx) -> Vec<V3> {
    let p = w.mobys[id].pv_i(PATH);
    usize::try_from(p).ok().and_then(|p| ctx.level.paths.get(p)).map_or(Vec::new(), |v| v.iter().map(|q| [q[0], q[1], q[2]]).collect())
}

/// Point `k` of its path (none past it: the origin, as a read past the game's list would be junk).
fn point(pts: &[V3], k: i32) -> V3 { usize::try_from(k).ok().and_then(|k| pts.get(k)).copied().unwrap_or([0.0; 3]) }

/// Its fence `k` (a moby index, −1 none).
fn fence_of(w: &MobyWorld, id: usize, k: usize) -> Option<usize> {
    usize::try_from(w.mobys[id].pv_i(FENCES + 4 * k)).ok().filter(|&f| f < w.mobys.len())
}

/// `0x31eff8(m, s, 0, blend)` unless `s` plays.
fn play(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8, blend: u8) {
    if crate::moby::classes::props::seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, blend); }
}

/// `0x4268b0`: class sound 1 every `rand_range(30, 180)` ticks.
fn chirp(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut t = w.mobys[id].pv_i(CHIRP);
    let due = countdown(&mut t);
    w.mobys[id].set_pv_i(CHIRP, t);
    if due {
        let n = ctx.game.rng.rand_range(30, 180);
        w.mobys[id].set_pv_i(CHIRP, n);
        ctx.host.play_sound(&w.mobys[id], 1, 0);
    }
}

/// Its yaw eased toward Ratchet (`0x331bf0(m, atan2(Ratchet − it))`).
fn face_hero(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    let yaw = atan2(ctx.hero.pos[0] - m.pos[0], ctx.hero.pos[1] - m.pos[1]);
    ai::face(m, yaw);
}

/// `0x330a08(m, p)`: the framework's move toward `p` (its heading the one to it).
fn walk_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3) -> i32 {
    let q = w.mobys[id].pos;
    ai::move_step(w, id, ctx, atan2(p[0] - q[0], p[1] - q[1]), p)
}

/// A clear line between two points 0.5 up (`0x2e1e20(a, b, 2, m)`).
fn clear(w: &MobyWorld, id: usize, ctx: &Ctx, a: V3, b: V3) -> bool {
    w.coll_line(ctx, [a[0], a[1], a[2] + 0.5], [b[0], b[1], b[2] + 0.5], 2, Some(id)).is_none()
}

/// `0x4264d8`: the first point of its path but the one it heads for with a clear line from it, else that one.
fn first_seen(w: &MobyWorld, id: usize, ctx: &Ctx) -> i32 {
    let pts = points(w, id, ctx);
    let (pos, at) = (w.mobys[id].pos, w.mobys[id].pv_i(AT));
    (0..pts.len() as i32).find(|&i| i != at && clear(w, id, ctx, pos, pts[i as usize])).unwrap_or(at)
}

/// `0x4265e0(m, homeward)`: up to 30 tries of a random point of its path (`randi(count)`) but the one it heads for,
/// not more than 2 over it, with a clear line from it; homeward, not one more than 16 across from home (`$gp+0x308c`)
/// farther along the path from home than the one it heads for. None: the one it heads for.
fn next_point(w: &MobyWorld, id: usize, ctx: &mut Ctx, homeward: bool) -> i32 {
    let pts = points(w, id, ctx);
    let m = &w.mobys[id];
    let (pos, at, home) = (m.pos, m.pv_i(AT), m.pv_i(HOME));
    let target = point(&pts, home);
    for _ in 0..30 {
        let pick = ctx.game.rng.randi(pts.len() as i32);
        if pick == at { continue; }
        let to = point(&pts, pick);
        let raised = [to[0], to[1], to[2] + 0.5];
        if homeward && FAR < dist2(raised, target) && (at - home).abs() < (pick - home).abs() { continue; }
        if pos[2] + 0.5 + 2.0 < raised[2] { continue; }
        if clear(w, id, ctx, pos, to) { return pick; }
    }
    at
}

/// `0x4267b0`: the home point the last point past it on the path's floor (within 2 of its first point's height)
/// within 6 of Ratchet across.
fn find_home(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let pts = points(w, id, ctx);
    let Some(&first) = pts.first() else { return };
    let home = w.mobys[id].pv_i(HOME);
    for i in (home + 1).max(0) as usize..pts.len() {
        if (pts[i][2] - first[2]).abs() < 2.0 && dist2(ctx.hero.pos, pts[i]) < 6.0 { w.mobys[id].set_pv_i(HOME, i as i32); }
    }
}

/// 0x1169 (module docs).
pub fn update_1169(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if uid_done(w, id, ctx) { return w.delete(id); }
    let fences = [fence_of(w, id, 0), fence_of(w, id, 1)];
    let mut was = fences.map(|f| fence::walls_lit(w, f, &*ctx.host));
    for f in fences { fence::walls_on(w, f, &*ctx.host); }
    let class_scale = w.class(CLASS).map_or(1.0, |c| c.scale);
    w.mobys[id].scale = class_scale;
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if w.mobys[id].sub_state == 0 {
        if tractor::held(w) == Some(id) {
            let m = &mut w.mobys[id];
            m.set_state(5, 1);
            let p = m.pos;
            m.set_pv_v(DRAG + 0x10, p);
        } else {
            let m = &mut w.mobys[id];
            if dist2(m.pos, ctx.hero.pos) < m.pv_f(DRAG + 0x38) { m.mode |= mode::TARGET } else { m.mode &= !mode::TARGET }
        }
    }
    match w.mobys[id].state {
        0 => {
            let start = points(w, id, ctx).first().copied();
            let m = &mut w.mobys[id];
            m.set_pv_i(MOVE + 0x38, m.pv_i(MOVE + 0x38) | 1); // `+0x88` |= 1
            for (o, v) in [(0x20, 1.0), (0x30, 1.5), (0x84, 0.05), (0x54, 0.2), (0x50, f32::from_bits(0x3f53_3333)), (0x5c, 0.4)] { m.set_pv_f(o, v); }
            m.set_pv_f(0x60, f32::from_bits(0x3fc9_0fdc));
            m.set_pv_f(0x80, f32::from_bits(0x3f06_0a93));
            m.set_pv_s(0x24, 1);
            m.set_pv_f(0x58, 0.4);
            m.set_pv_i(AT, 0);
            // The port's: no slide sound playing yet (the drag's handle).
            m.set_pv_i(DRAG + 0x40, -1);
            if let Some(p) = start { m.pos = p; }
            m.state = 1;
            m.update_dist = 0x80;
            m.mode |= mode::TARGET;
        }
        1 => {
            if w.mobys[id].first() {
                let r = ctx.game.rng.randi(2) as u8;
                if crate::moby::classes::props::seq(w, id, ctx) != r {
                    let s = ctx.game.rng.randi(2) as u8;
                    ctx.host.set_anim(&w.mobys[id], s, 0, 10);
                }
                let home = point(&points(w, id, ctx), w.mobys[id].pv_i(HOME));
                let n = if 24.0 < dist2(w.mobys[id].pos, home) { 30 } else { 180 };
                w.mobys[id].set_pv_i(IDLE, n);
            }
            let m = &w.mobys[id];
            if m.pv_i(IDLE) < m.state_ticks as u16 as i32 {
                let home = point(&points(w, id, ctx), m.pv_i(HOME));
                let s = if m.pv_i(AT) < m.pv_i(HOME) && 20.0 < dist2(m.pos, home) { 4 } else { 2 };
                w.mobys[id].set_state(s, -1);
            }
        }
        4 => {
            if w.mobys[id].first() {
                play(w, id, ctx, 5, 0x14);
                let m = &mut w.mobys[id];
                let (at, home) = (m.pv_i(AT), m.pv_i(HOME));
                m.set_pv_i(AT, at + (home - at).clamp(-1, 1));
                ai::senses(m, f32::from_bits(0x3b11_a2b4), RUN * 0.016_666_668, f32::from_bits(0x3b64_c38a), f32::from_bits(0x3e56_7752));
            }
            let pts = points(w, id, ctx);
            let (at, home) = (w.mobys[id].pv_i(AT), w.mobys[id].pv_i(HOME));
            if at < home && clear(w, id, ctx, w.mobys[id].pos, point(&pts, at + 1)) { w.mobys[id].set_pv_i(AT, at + 1); }
            let to = point(&pts, w.mobys[id].pv_i(AT));
            walk_to(w, id, ctx, to);
            if dist2(w.mobys[id].pos, point(&pts, home)) < 8.0 { w.mobys[id].set_state(1, -1); }
        }
        2 => {
            if w.mobys[id].first() {
                play(w, id, ctx, 4, 0x14);
                let next = next_point(w, id, ctx, true);
                let home = point(&points(w, id, ctx), w.mobys[id].pv_i(HOME));
                let m = &mut w.mobys[id];
                m.set_pv_i(AT, next);
                ai::senses(m, f32::from_bits(0x3a91_a2b4), WALK * 0.016_666_668, f32::from_bits(0x3a64_c38a), f32::from_bits(0x3dd6_7752));
                let n = if 24.0 < dist2(m.pos, home) { 900 } else { 300 };
                m.set_pv_i(IDLE, n);
            }
            let to = point(&points(w, id, ctx), w.mobys[id].pv_i(AT));
            let flags = walk_to(w, id, ctx, to);
            if flags & 0x80 != 0 || 300 <= w.mobys[id].state_ticks as u16 { w.mobys[id].set_state(1, -1); }
        }
        3 => {
            if w.mobys[id].first() {
                play(w, id, ctx, 5, 0x14);
                let next = next_point(w, id, ctx, false);
                let m = &mut w.mobys[id];
                m.set_pv_i(AT, next);
                ai::senses(m, f32::from_bits(0x3b7e_dcbb), RUN * 0.016_666_668, f32::from_bits(0x3b64_c38a), f32::from_bits(0x3ed6_7752));
            }
            let to = point(&points(w, id, ctx), w.mobys[id].pv_i(AT));
            if walk_to(w, id, ctx, to) & 0x80 != 0 {
                let next = next_point(w, id, ctx, true);
                w.mobys[id].set_pv_i(AT, next);
            }
            if 0x1a4 <= w.mobys[id].state_ticks as u16 { w.mobys[id].set_state(1, -1); }
        }
        5 => {
            if w.mobys[id].first() {
                play(w, id, ctx, 7, 10);
                w.mobys[id].set_pv_i(CHIRP, 0);
            }
            let m = &w.mobys[id];
            let d = dot(m.rows[0], m.pv_v(DRAG));
            let s = if d < -1.0 / 30.0 { 10 } else if 1.0 / 30.0 < d { 9 } else { 7 };
            play(w, id, ctx, s, 10);
            face_hero(w, id, ctx);
            for (k, f) in fences.into_iter().enumerate() { was[k] |= fence::walls_lit(w, f, &*ctx.host); }
            for f in fences { fence::walls_on(w, f, &*ctx.host); }
            let path = w.mobys[id].pv_i(FENCE_PATH);
            tractor::drag_at(w, id, ctx, 0.5, path, DRAG);
            chirp(w, id, ctx);
            if w.mobys[id].once & 4 != 0 {
                w.mobys[id].set_state(6, -1);
            } else if tractor::held(w) != Some(id) {
                w.mobys[id].set_state(3, 0);
                let k = first_seen(w, id, ctx);
                w.mobys[id].set_pv_i(AT, k);
            }
        }
        6 => {
            if w.mobys[id].first() { play(w, id, ctx, 0xa, 10); }
            face_hero(w, id, ctx);
            chirp(w, id, ctx);
        }
        _ => {}
    }
    for (k, f) in fences.into_iter().enumerate() {
        if f.is_some() && !was[k] { fence::walls_off(w, f); }
    }
    if ctx.game.frame & 0xf == 0 { find_home(w, id, ctx); }
    let m = &mut w.mobys[id];
    if m.drawn != 0 {
        let a = add_rot(m.pv_f(GLOW), 400.0 * f32::from_bits(0x3998_825c));
        m.set_pv_f(GLOW, a);
        m.glow = crate::hud::lerp_rgba(a.sin() * 0.5 + 0.5, 0x80c0_c0c0, 0x8060_6060);
    }
    m.state_ticks = m.state_ticks.wrapping_add(1);
    let p = m.pos;
    m.set_pv_v(PLACE, p);
}
