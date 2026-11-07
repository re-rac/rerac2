//! Gorn's big ships 0x1211 (L15 `0x435ec0`, 4 placed; the decomp's `func_L15_00434100`): they fly their paths on the
//! space-AI library (crate::moby::classes::space), send fighters 0x957 from their sides, fire their turrets at the
//! player, cloak and uncloak (the second mission's), shed pieces as they break and fall at their death.
//!
//! * **State 0** (once the library's ready flag is set, taken as set): `0x4371a8` — health 150 (`+0x148`, `+0x180`),
//!   active (`0x382980(m, 1)`, [`space::start`]), state 1, drawn and updated from anywhere, scale 0.675, the rows
//!   kept, its pieces 0..5 shown (`+0x7c` 63), the fighters' wait 300 (`+0x444`) from side 0 (`+0x440`), the AI
//!   block's `+0x150` 0xffff; its path `+0x448`, its place kept (`+0x430`). With its leader word (`+0x264`) it leads
//!   with the callback `0x437c40` ([`on_activate`]).
//! * **The flight** (`0x4363d0(m, cloaked)`, [`fly`]), every state but 6 / 7: its target the player's ship
//!   (`+0x260`); the wait out, three fighters from the next side (the first put-away 0x957s of their leader's group,
//!   `0x383038`: at (0 / 3.375 / 6.75, ±2.25, 4.8) in its frame, flying along its left (or right) row, their goal 10
//!   on, 500 ticks), again in 300; its path's next point at each arrival (`0x3850a0`, round to the first), the AI state
//!   2 and its tick (`0x383d58`, [`space::think`]); on screen within 100 with a clear line to its target, the turrets
//!   ([`turrets`], `0x438058`); not cloaked, the push and hits (`0x383130(m, 1, 1, 1)`): at its death state 6, its
//!   sound 1, its engine's loop stopped, its angles kept (`+0x400`). Below 100 health (400 in state 4, uncloaked): the
//!   alarm (its sound 4, once), and at each stage (`health × 0.08`, `× 0.02` in state 4) below 7 the blasts at its
//!   place and at the stage's joint (`$gp+0x2770`: 2, 0x13, 1, 3, 0x11, 0x14, 0), a piece off ([`piece_off`],
//!   `0x437e98`), its pieces shown `(1 << (n − 1)) − 1`; its damaged engine's loop (`0x436ae8`: its sound 3).
//! * **States 2..5**, the cloak (`0x437e50`, [`cloak`]: 450 health, alpha 32, no collision, the mode's bit 8): 2
//!   cloaked till `+0x274` (480) is out; 3 fading in over 180 (alpha 32 → 127), then 4: seen and hit for 600 ticks
//!   (alpha 128, collision, bit 8 off); 5 fading out over 180, then 2 for 480.
//! * **State 6**, falling: one big blast (`0x424990` = the blasts' 0x116a at 3, with the shake) ±3 round it, small
//!   ones one tick in 12 while above 350; down by `+0x27c` (+0.000556 a tick), its second angle turning 0.083° a
//!   tick; out of
//!   the world's box put away (`0x382900`).
//! * **State 7**, a piece off: moving `+0x410` (its z falling 0.00306 a tick), its second angle turning 1.5° a tick;
//!   out of the box
//!   gone.
//! The turrets' wait (`+0x450`) is counted every tick.
//!
//! Not ported: the turrets' and the hatch's joint turns (`0x4375f8`, the manipulators `+0x460..+0x660` from the joints
//! 12..19), the exhaust (`0x438188`: four nozzles' puffs, descriptors `$gp+0x27e0..` not in crate::particles), the
//! damaged engine's smoke and arcs (`0x436ae8`: `0x436d50` at its joints `0x1b1780`, `0x3a24d8`), the stage's
//! piece by the class's byte table (`+0x2e`'s: taken as the stage itself here), `0x3804b8(m, 1)`.

use crate::hero::physics::{add, cross, dist, scale, sub, unit, V3};
use crate::moby::attach::{angles_of, rows_of};
use crate::moby::classes::space::{self, a};
use crate::moby::{countdown, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x1211;
/// The fighters it sends.
const FIGHTER: i16 = 0x0957;

/// The pvars (the module's docs); the original's past `+0x400` kept past the library's words here.
mod pv {
    pub const TARGET: usize = 0x260;
    pub const LEADS: usize = 0x264;
    pub const WHOLE: usize = 0x268;
    pub const STAGE: usize = 0x26c;
    pub const CLOAK: usize = 0x270;
    pub const TIMER: usize = 0x274;
    pub const BLASTED: usize = 0x278;
    pub const FALL: usize = 0x27c;
    pub const ALARM: usize = 0x700;
    pub const SIDE: usize = 0x704;
    pub const SPAWN: usize = 0x708;
    pub const PATH: usize = 0x70c;
    pub const POINT: usize = 0x710;
    pub const GUN: usize = 0x714;
    pub const TURRET: usize = 0x718;
    pub const SOUND: usize = 0x71c;
    pub const HOME: usize = 0x720;
    pub const ANGLES: usize = 0x730;
    pub const VEL: usize = 0x740;
}

/// Its numbers on the library: health 150, left where it is, its target `+0x260` and leader word `+0x264`.
pub(crate) fn spec() -> space::Spec {
    space::Spec { health: 150.0, scale: 0.675, scatter: false, target: pv::TARGET, leads: pv::LEADS, gun: pv::GUN, flyby: 0, engine: None, death: None, gun_sound: None, nozzles: &[], ..space::FIGHTER_SPEC }
}

/// The fighters' places in its frame (`0x2b6cb0`: side 0, then side 1).
const SPAWNS: [V3; 6] = [[0.0, 2.25, 4.8], [3.375, 2.25, 4.8], [6.75, 2.25, 4.8], [0.0, -2.25, 4.8], [3.375, -2.25, 4.8], [6.75, -2.25, 4.8]];
/// The stages' joints (`$gp+0x2770`, by 7 − the stage).
const STAGE_JOINTS: [usize; 8] = [0, 2, 0x13, 1, 3, 0x11, 0x14, 0];

fn path(w: &MobyWorld, id: usize, ctx: &Ctx) -> Vec<[f32; 4]> {
    usize::try_from(w.mobys[id].pv_i(pv::PATH)).ok().and_then(|p| ctx.level.paths.get(p)).cloned().unwrap_or_default()
}

fn in_frame(rows: [V3; 3], v: V3) -> V3 { add(add(scale(rows[0], v[0]), scale(rows[1], v[1])), scale(rows[2], v[2])) }

fn out_of_box(p: V3) -> bool { p.iter().any(|&c| !(2.0..=1021.0).contains(&c)) }

/// `0x437c40` (`func_L15_00435E80`): each member of the group back at its place and on its path — its pieces shown,
/// its angles cleared, state 1, at the path's first point going for the second (the velocity a sixtieth of the way a
/// tick, 2000000 ticks), the turrets ready, a random first one; the leader's own alpha 0x7f and the bit 8 off (the
/// original writes those on the leader each time).
pub(crate) fn on_activate(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for m in space::members(w, id) {
        if w.mobys[m].o_class != CLASS { continue; }
        let pts = path(w, m, ctx);
        let turret = ctx.game.rng.randi(2);
        {
            let s = &mut w.mobys[m];
            s.bangles = 0x3f;
            s.pos = s.pv_v(pv::HOME);
            s.rot = [0.0; 3];
            s.rows = rows_of([0.0; 3]);
            s.set_pv_f(pv::FALL, 0.0);
            s.set_pv_v(pv::ANGLES, [0.0; 3]);
            s.state = 1;
            s.set_pv_i(pv::POINT, 1);
            s.set_pv_i(pv::GUN, 0);
            s.set_pv_i(pv::TURRET, turret);
        }
        if let (Some(p0), Some(p1)) = (pts.first(), pts.get(1)) {
            w.mobys[m].pos = [p0[0], p0[1], p0[2]];
            space::set_ai_i(w, m, a::STATE, 2);
            space::set_goal(w, m, ctx, [p1[0], p1[1], p1[2]]);
            w.mobys[m].set_pv_i(pv::POINT, 2);
            let v = scale(sub(space::ai_v(w, m, a::GOAL), w.mobys[m].pos), 1.0 / 60.0);
            space::set_ai_v(w, m, a::VEL, v);
            space::set_ai_i(w, m, a::TIMER, 2_000_000);
        }
    }
    let l = &mut w.mobys[id];
    l.alpha = 0x7f;
    l.mode &= !8;
}

/// `0x437e50` (`func_L15_00436090`): cloaked — 450 health, state 2 for 480 ticks, alpha 32, no collision, bit 8.
pub(crate) fn cloak(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::CLOAK, 1);
    m.set_pv_i(pv::TIMER, 0x1e0);
    m.alpha = 0x20;
    m.state = 2;
    m.mode |= 8;
    m.set_pv_f(space::HEALTH, 450.0);
    m.has_collision = false;
}

/// `0x435ec0`.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            w.mobys[id].update_dist = 0xff;
            // `0x4371a8` (its path and place read before the AI block's start clears them).
            let (route, home) = (w.mobys[id].pv_i(0x448), w.mobys[id].pos);
            w.mobys[id].set_pv_i(pv::PATH, route);
            w.mobys[id].set_pv_v(pv::HOME, home);
            space::start(w, id, ctx);
            let m = &mut w.mobys[id];
            m.state = 1;
            m.mode |= mode::KEEP_ROWS;
            m.bangles = 0x3f;
            for o in [pv::WHOLE, pv::STAGE, pv::CLOAK, pv::BLASTED, pv::FALL, pv::ALARM, pv::SIDE, pv::GUN, pv::TURRET] { m.set_pv_i(o, 0); }
            m.set_pv_i(pv::WHOLE, 1);
            m.set_pv_i(pv::SOUND, 0);
            m.set_pv_i(pv::SPAWN, 300);
            m.set_pv_i(pv::POINT, 0);
            space::set_ai_i(w, id, 0x150, 0xffff);
        }
        1 => fly(w, id, ctx, false),
        2 => {
            fly(w, id, ctx, true);
            if timer_out(w, id) {
                w.mobys[id].state = 3;
                w.mobys[id].set_pv_i(pv::TIMER, 180);
            }
        }
        3 => {
            fly(w, id, ctx, true);
            let t = w.mobys[id].pv_i(pv::TIMER) as f32 / 180.0;
            w.mobys[id].alpha = ((1.0 - t) * 95.0) as u8 + 32;
            if timer_out(w, id) {
                let m = &mut w.mobys[id];
                m.has_collision = true;
                m.state = 4;
                m.set_pv_i(pv::TIMER, 600);
                m.alpha = 128;
                m.mode &= !8;
            }
        }
        4 => {
            fly(w, id, ctx, false);
            if timer_out(w, id) {
                let m = &mut w.mobys[id];
                m.has_collision = false;
                m.state = 5;
                m.set_pv_i(pv::TIMER, 180);
                m.mode |= 8;
            }
        }
        5 => {
            fly(w, id, ctx, true);
            let t = w.mobys[id].pv_i(pv::TIMER) as f32 / 180.0;
            w.mobys[id].alpha = (t * 95.0) as u8 + 32;
            if timer_out(w, id) {
                let m = &mut w.mobys[id];
                m.has_collision = true;
                m.state = 2;
                m.set_pv_i(pv::TIMER, 480);
                m.alpha = 32;
            }
        }
        6 => fall(w, id, ctx),
        7 => {
            let v = w.mobys[id].pv_v(pv::VEL);
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, v);
            m.set_pv_v(pv::VEL, [v[0], v[1], v[2] - 0.003_055_555_7]);
            let mut ang = m.pv_v(pv::ANGLES);
            ang[1] = crate::hero::physics::add_rot(ang[1], 0.026_179_943);
            m.set_pv_v(pv::ANGLES, ang);
            m.rows = rows_of(ang);
            if out_of_box(m.pos) { return w.delete(id); }
        }
        _ => {}
    }
    let mut g = w.mobys[id].pv_i(pv::GUN);
    countdown(&mut g);
    w.mobys[id].set_pv_i(pv::GUN, g);
    w.rebuild(id, &*ctx.host);
}

fn timer_out(w: &mut MobyWorld, id: usize) -> bool {
    let mut t = w.mobys[id].pv_i(pv::TIMER);
    let out = countdown(&mut t);
    w.mobys[id].set_pv_i(pv::TIMER, t);
    out
}

/// `0x4363d0(m, cloaked)` (module docs).
fn fly(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, cloaked: bool) {
    space::take_own_target(w, id);
    // The fighters.
    let mut t = w.mobys[id].pv_i(pv::SPAWN);
    let out = countdown(&mut t);
    w.mobys[id].set_pv_i(pv::SPAWN, t);
    if out {
        w.mobys[id].set_pv_i(pv::SPAWN, 300);
        let side = w.mobys[id].pv_i(pv::SIDE);
        let (rows, at) = (w.mobys[id].rows, w.mobys[id].pos);
        for k in 0..3 {
            let p = add(in_frame(rows, SPAWNS[side as usize * 3 + k]), at);
            let Some(f) = space::activate_one(w, FIGHTER, ctx) else { continue };
            let fwd = scale(rows[1], if side != 0 { -1.0 } else { 1.0 });
            let left = unit(cross(fwd, [0.0, 0.0, 1.0]));
            {
                let m = &mut w.mobys[f];
                m.pos = p;
                m.rows = [fwd, left, cross(left, fwd)];
            }
            space::set_ai_v(w, f, a::VEL, fwd);
            space::set_ai_v(w, f, a::GOAL, add(p, scale(fwd, 10.0)));
            space::set_ai_i(w, f, a::TIMER, 500);
        }
        w.mobys[id].set_pv_i(pv::SIDE, side ^ 1);
    }
    // Its path.
    let pts = path(w, id, ctx);
    if space::arrived(w, id) && !pts.is_empty() {
        let next = w.mobys[id].pv_i(pv::POINT).clamp(0, pts.len() as i32 - 1);
        let p = pts[next as usize];
        space::set_goal(w, id, ctx, [p[0], p[1], p[2]]);
        space::set_ai_i(w, id, a::TIMER, 2_000_000);
        let n = next + 1;
        w.mobys[id].set_pv_i(pv::POINT, if pts.len() as i32 <= n { 0 } else { n });
    }
    space::set_ai_i(w, id, a::STATE, 2);
    space::think(w, id, ctx);
    let me = w.mobys[id].pos;
    let r = w.mobys[id].bsphere[3] / 1024.0;
    let cam = ctx.host.camera_pos();
    if let Some(tg) = usize::try_from(space::ai_i(w, id, a::TARGET) - 1).ok().filter(|&t| t < w.mobys.len()) {
        let seen = space::in_view(ctx, me, r) && dist(me, cam) < 100.0 + r;
        let to = w.mobys[tg].pos;
        let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, me, to, crate::collision::QueryFlags(0), Some(id)));
        let clear = hit.is_some_and(|h| h.moby.is_some_and(|m| w.mobys.get(m).is_some_and(|m| m.o_class == w.mobys[tg].o_class)));
        if seen && clear { turrets(w, id, ctx); }
    }
    if !cloaked && space::hits_with(w, id, ctx, true, true, true) {
        w.mobys[id].state = 6;
        ctx.host.play_sound_of(&w.mobys[id], CLASS, 1, 0);
        let h = w.mobys[id].pv_i(pv::SOUND) - 1;
        if 0 <= h { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(pv::SOUND, 0);
        let ang = angles_of(&w.mobys[id].rows);
        w.mobys[id].set_pv_v(pv::ANGLES, ang);
    }
    // The damage's stages.
    let shown = w.mobys[id].state == 4;
    let hp = w.mobys[id].pv_f(space::HEALTH);
    if hp <= if shown { 400.0 } else { 100.0 } {
        w.mobys[id].set_pv_i(pv::WHOLE, 0);
        if w.mobys[id].pv_i(pv::ALARM) == 0 {
            ctx.host.play_sound_of(&w.mobys[id], CLASS, 4, 0);
            w.mobys[id].set_pv_i(pv::ALARM, 1);
        }
        let n = (hp * if shown { 0.02 } else { 0.08 }) as i32;
        if n < 7 && n != w.mobys[id].pv_i(pv::STAGE) {
            let b = crate::moby::blast::Blast { size: 2.5, bursts: [0x32, 0x1e, 0x1e], puffs: 0x1e, streaks: 0x28, light: true, shake: false };
            crate::moby::blast::burst(w, ctx, me, &b);
            let j = STAGE_JOINTS[(7 - n).clamp(0, 7) as usize];
            let at = ctx.host.joint_matrix(&w.mobys[id], j).map_or(me, |(_, p)| p);
            crate::moby::blast::burst(w, ctx, at, &b);
            if 0 < n { piece_off(w, id, ctx, n - 1); }
        }
        engine_loop(w, id, ctx);
        w.mobys[id].set_pv_i(pv::STAGE, n);
        w.mobys[id].bangles = if 0 < n { ((1u32 << (n - 1)) - 1) as u16 } else { 0 };
    }
}

/// `0x436ae8`'s loop: the damaged engine's sound 3, started again whenever it is not playing.
fn engine_loop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(pv::SOUND) - 1;
    if h < 0 || !ctx.host.sound_playing(h) {
        let n = ctx.host.play_sound_of(&w.mobys[id], CLASS, 3, 4);
        w.mobys[id].set_pv_i(pv::SOUND, n + 1);
    }
}

/// `0x438058`: with a target and the wait out, its sound 2 and a shot from each of the turret's two joints
/// (`0x381a60(m, 0x600000cf, 1, 1, 0, 2, 1.5, 1.5, 0x1e, 4 + 2k / 5 + 2k)`: 2 ahead of the target), the next in
/// 20..60, the next turret of three.
fn turrets(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(pv::GUN) != 0 { return; }
    let Some(t) = usize::try_from(space::ai_i(w, id, a::TARGET) - 1).ok().filter(|&t| t < w.mobys.len()) else { return };
    ctx.host.play_sound_of(&w.mobys[id], CLASS, 2, 0);
    let k = w.mobys[id].pv_i(pv::TURRET).clamp(0, 2) as usize;
    if w.mobys[t].draw_dist != 0 {
        let at = add(w.mobys[t].pos, crate::hero::physics::set_len(w.mobys[t].rows[0], 2.0));
        for j in [4 + 2 * k, 5 + 2 * k] {
            let from = ctx.host.joint_matrix(&w.mobys[id], j).map_or(w.mobys[id].pos, |(_, p)| p);
            crate::ship::spawn_enemy_shot(w, &*ctx.host, from, at, t, (1.0, 1.5, 0x6000_00cf));
        }
    }
    let n = ctx.game.rng.rand_range(0x14, 0x3c);
    w.mobys[id].set_pv_i(pv::GUN, n);
    w.mobys[id].set_pv_i(pv::TURRET, if 3 <= k + 1 { 0 } else { k as i32 + 1 });
}

/// `0x437e98(m, k)`: a piece off — a 0x1211 drawn as piece `k` alone (`+0x7c` 0x8000 | 1 << k), at its centre (the
/// class's piece, in its frame at its scale), drifting away at 0.267 of its offset a tick, its frame and angles the
/// ship's, state 7.
fn piece_off(w: &mut MobyWorld, id: usize, ctx: &Ctx, k: i32) {
    let Some(centre) = w.class(CLASS).and_then(|c| c.pieces.get(k as usize).cloned().flatten()).map(|(c, _)| c) else { return };
    let (pos, rows, sc) = (w.mobys[id].pos, w.mobys[id].rows, w.mobys[id].scale);
    let at = add(in_frame(rows, scale(centre, sc)), pos);
    let p = w.create(CLASS);
    let m = &mut w.mobys[p];
    m.update_dist = 0xff;
    m.draw_dist = 0x1ff;
    m.drawn = 1;
    m.scale = sc;
    m.mode |= mode::KEEP_ROWS;
    m.rows = rows;
    m.pos = pos;
    m.set_pv_v(pv::VEL, scale(sub(at, pos), 0.266_666_68));
    m.set_pv_v(pv::ANGLES, angles_of(&rows));
    m.state = 7;
    m.piece = Some(k as u8);
    m.has_collision = false;
    w.rebuild(p, &*ctx.host);
}

/// State 6 (module docs).
fn fall(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p = w.mobys[id].pos;
    let g = &mut ctx.game.rng;
    let at = [p[0] + g.randf(-3.0, 3.0), p[1] + g.randf(-3.0, 3.0), p[2] + g.randf(-3.0, 3.0)];
    if w.mobys[id].pv_i(pv::BLASTED) == 0 {
        let b = crate::moby::blast::Blast { size: 3.0, bursts: [0x18, 0x18, 0x18], puffs: 0x18, streaks: 0x18, light: true, shake: true };
        crate::moby::blast::burst(w, ctx, at, &b);
        w.mobys[id].set_pv_i(pv::BLASTED, 1);
    }
    if ctx.game.rng.randi(12) == 0 && 350.0 < p[2] {
        let b = crate::moby::blast::Blast { size: 1.5, bursts: [4, 4, 4], puffs: 4, streaks: 4, light: false, shake: false };
        crate::moby::blast::burst(w, ctx, at, &b);
    }
    let m = &mut w.mobys[id];
    let f = m.pv_f(pv::FALL);
    m.pos[2] -= f;
    m.set_pv_f(pv::FALL, f + 0.000_555_555_6);
    let mut ang = m.pv_v(pv::ANGLES);
    ang[1] = crate::hero::physics::add_rot(ang[1], 0.001_454_441_3);
    m.set_pv_v(pv::ANGLES, ang);
    m.rows = rows_of(ang);
    engine_loop(w, id, ctx);
    if out_of_box(w.mobys[id].pos) { space::put_away(w, id); }
}
