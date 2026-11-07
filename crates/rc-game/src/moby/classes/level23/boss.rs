//! 0x0f7f, Damosel orbit's opponent (L23 `0x417888`, the decomp's `func_US2_L23_00417888`; its frame `0x418cf8`,
//! asm only; its helpers `0x4195f0` … `0x41fab0`, the decomp's `func_L23_004180F0` … `func_L23_0041E5B0`; one placed):
//! a flying fortress that circles Giant Clank over the planetoid ([`paths`]), behind a shield of five generators
//! 0x0f7e and its core 0x120f (crate::moby::classes::level23::targets), firing its turret, launching its 25 walkers
//! 0x07e5 (crate::moby::classes::level23::drones) and, landed, calling a ring of lightning and its shock wave.
//!
//! **Its pvars**: health `+0x20` / most `+0x24`; mover `+0x50`; reaction `+0x140`; the slots `+0x230` (47); the
//! fight's numbers `+0x2f0..+0x33c` (Giant Clank's health step and most `+0x2f0` / `+0x2f4`, its own step `+0x2f2`
//! and most `+0x2f6` 3600, the gun's step / most `+0x2f8` / `+0x2fa`, the specials' `+0x2fc` / `+0x2fe`, the damages
//! `+0x300` turret 5, `+0x304` missiles 30, `+0x308` / `+0x30c` the walkers' missiles and beam, `+0x310` Giant
//! Clank's shots, `+0x314` the specials, the walkers' health `+0x31c`, the health shares `+0x320` generators 0.6,
//! `+0x324` its plates 0.05, `+0x328` the core 0.2 (`+0x32c` its own the rest), `+0x330` the riders' turn, the fire
//! rates `+0x334` / `+0x336` (a shot every `2^(9 − n)` ticks), the speeds `+0x338` / `+0x33a` / `+0x33c`); the
//! generators `+0x358` (5, by their joints 1..5) and their spares `+0x36c` (5); the walk `+0x390..` ([`paths::BOSS`]);
//! the flat places `+0x3e0` (its own) / `+0x3f0` (his); its goal `+0x400`; the line to him clear `+0x404`, facing
//! him `+0x405`; the down `+0x410`; its turn angles `+0x570` (the shield, joint 0x22), `+0x578` / `+0x57c` (the
//! turret, joint 0x27), `+0x580` / `+0x584` (the barrels, joints 0x28 / 0x29), `+0x380` the ring's; its height
//! `+0x588` (velocity `+0x58c`, target `+0x5d4`, reached `+0x5d8`) and speed `+0x5c8` (`+0x5cc`, `+0x5d0`, `+0x5dc`);
//! the launches `+0x590` (left) / `+0x594` (the joint) / `+0x598` (the wait) / `+0x59c` (the walker under way); the
//! ring `+0x5b0` (on) / `+0x5c4` (its hit's wait); his flat distance `+0x5b8`, distance `+0x5bc`, height `+0x5c0`;
//! the circle's side `+0x5f0`; its guard `+0x604` (up this tick); the launch wait `+0x608`; the generator it turns to
//! him `+0x60c` (−1 none left); the core `+0x610`, the cap 0x123c `+0x614`; the effects 0x1252 / 0x1251 `+0x620` /
//! `+0x624`; the shock wave `+0x408`.
//!
//! **The set-up** ([`frame`] in state 0): the fight's words cleared; the level's walkers 0x07e5 listed, Giant Clank
//! put at the body 0x027d (the radius its distance from the centre), the generators, the core, the cap and the scene
//! trigger 0x0f6e (update distance 0xff, no level, its list 0, not set off) kept; the planetoid's middle and top; it
//! at (537, 511, 533) 15 up; its health (3600) and the generators' (`3600 · 0.6 · 0.2` = 432, floor 432 / 12) and the
//! core's (720); its mover (radius 7, height −7, steps 0.4, slope π/2, options | 5, the turn left 89°) and senses;
//! state 2; the paths loaded and linked, the skip table; the pickups ([`pickups::set_up`]); height 28, mode | 0x10,
//! its plates (`+0x7c` 0xf). **Each tick**: the skip table's scan; the spares hidden but in states 4 / 0xa / 0xb; the
//! effects found; its and its parts' scale × 0.7; the intro (`$gp+0x23e4` counted; Giant Clank in a landing lag of
//! 10, in its first 30 ticks idle with his gun put away; ✕ or △ ends it: state 1, a landing lag of 20); stood up on
//! the sphere, out of state 7 put at its height; its health ([`health`]).
//!
//! **Its health** ([`health`], `0x41cef0`): the generator to turn to him (`0x41ce68`: the healthiest above the floor;
//! none: −1 for good). With none and no core: its aim point 4 up its row 2, its own hit (`0x30000`) taken outside
//! its 20-tick wait, its guard and the special's blast from Giant Clank (guarded: its hurt sound): a flash, its plates
//! off (four balls, once), the blast at the hitter, the damage, the wait; out of health the death (state 4; Giant
//! Clank idle, his gun put away). Else its health the sum of its own share, the generators' above their floor, the
//! core's (gone below 0: deleted, the wait 30) and its plates' share while it has them; its aim the generator's
//! point 8 along its joint's row 1.
//!
//! **Each tick** ([`update_0f7f`]): the frame; past 200 intro ticks nothing more. Its guard word kept (`0x1b139c`,
//! `0x1b13e0`); the shield turned ([`shield`], `0x41cbd8`: with its guard up 60′ a tick, else toward the generator
//! `+0x60c` facing him within 0.95); Giant Clank's chest and middle; the flat places and distances; the walkers' tick
//! ([`drone_tick`]); the turret's test point and the line to his chest (`+0x404`, within 80); with no more than five
//! walkers out, in state 7 every 90 ticks one relaunched when it cannot get at him; facing him (`+0x405`). **The
//! states**: **1** to the middle (speed 67, height 28) → 3 within 10; **2** (the set-up's): speed 67, height 15;
//! **3** the walk there, the ring off → 7; **5** to the top → 0x11; **6** to the middle → 0x10; **7** circling him
//! at 35 + 5..45 (speed 45, height 28): with every generator down → 6; with five walkers or fewer out and its wait
//! out → 8 (every third time at 35, else 35 + 25..55); every 480 ticks a new distance; **8** circling to it → 9
//! within 6; **9** down to 4: four walkers launched at once from its joints 30..33, 140 ticks → 0xa; **0xa** up to
//! 28: at 5.5 the launches (4) → 0xb, its spares shown in its generators' places with collision; **0xb** the launches
//! one by one ([`launch`]), then the spares away, the wait 120 and → 0xd (seen and drawn: the turret's burst) or 7;
//! **0xd** → 0xe → (40 ticks) 0xf → (40 ticks) 7; **0xc** the launches → 0x10; **0x10** landed in the middle at 15:
//! with him far (past 72.9) the ring off and, its wait out, 0xc; with the core hurt or him within 22.5 the ring
//! ([`ring`]); **0x13** down to 4 for 300 ticks → 7; **0x14** the knock-back (crate::moby::ai::flight) → 4; **4** the
//! death ([`dying`]).
//!
//! **After the states**: the spares' plates back; the pickups' tick ([`pickups::tick`]); its counters; the
//! generators at its joints 1..5 (updated, line-tested), the core and the cap at its joint 0x22 (the core line-tested
//! only when every generator is down in 0xc / 0x10; else itself); the hatch ([`hatch`]); the turret's aim and fire at
//! his chest ([`aim`], [`fire`]); the shock wave ([`shock`]); its shadow mover; its height and speed eased; landed in
//! the ring with Giant Clank's special within 27 of the middle and under 13 up, the special blown up.
//!
//! Not ported: the lid's scale and the barrels' spin (`+0x470` / `+0x4a0`, the manipulators of joints 0x26), the
//! beams 0x11ac of the launches and the ring (`0x426bf0`, draws only), the ring's lights, sounds and screen tint
//! (`0x41e290`, `0x41e3c8`), the pickup messages (`0x1b36c8`, `0x1b36cc`, `0x1b36d0`, `0x1b36d4`), the text 0x2fed at
//! the intro's end, Ratchet's voice 0x2b (`0x1b3698`), the draw `0x41ef48` and its callback `0x41c978`, the clock's
//! seed.

use super::{drones, paths, pickups, shots, special, targets};
use crate::hero::physics::{add, add_rot, cross, dist, dot, scale, set_len, sub, unit, V3};
use crate::moby::blast::{burst, Blast};
use crate::moby::frames::{self, upright};
use crate::moby::hits::HitTemplate;
use crate::moby::{ai, countdown, Ctx, MobyWorld};
use crate::quat::{axis_angle, turn_rows};

pub const CLASS: i16 = 0x0f7f;
const BODY: i16 = 0x027d;
const CAP: i16 = 0x123c;
const TRIGGER: i16 = 0x0f6e;

/// An arc minute (rad).
const MINUTE: f32 = 0.000_290_888_25;
/// `$gp+0x2430`: its and its parts' scale over the class's.
const SCALE: f32 = 0.7;
/// `$gp+0x2490`, `$gp+0x2484`: its height at the set-up and its working height.
const HIGH: f32 = 28.0;
/// `0x1b1430` / `0x1b1440..`: the circle's base distance and its band; the speeds; the heights.
const CIRCLE: f32 = 35.0;
const BAND: f32 = 3.0;
const SPEED_A: f32 = 45.0;
const SPEED_B: f32 = 67.0;
const H_TOP: f32 = 28.0;
const H_LOW: f32 = 4.0;
const H_MID: f32 = 5.5;
const H_LAND: f32 = 15.0;
/// `0x1b14b0`: the ring's reach from the middle.
const RING_REACH: f32 = 45.0;
/// `$gp+0x2608` / 2: the walkers out it allows itself to launch more.
const OUT: i32 = 5;

fn pv_grow(w: &mut MobyWorld, id: usize) { if w.mobys[id].pvars.len() < 0x630 { w.mobys[id].pvars.resize(0x630, 0); } }

/// A part it keeps (`off`), still there.
fn part(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { ai::link(&w.mobys[id], off).filter(|&p| w.mobys.get(p).is_some_and(|m| m.state < 0xfd)) }
fn generators(w: &MobyWorld, id: usize) -> [Option<usize>; 5] { std::array::from_fn(|i| part(w, id, 0x358 + 4 * i)) }
fn spares(w: &MobyWorld, id: usize) -> [Option<usize>; 5] { std::array::from_fn(|i| part(w, id, 0x36c + 4 * i)) }

/// `0x2aa548(&x, &v, target, step, accel, most)`: `x` toward `target` by a velocity that speeds up by `step` and
/// brakes by `accel` to stop there, at most `most`.
pub(super) fn glide(x: &mut f32, v: &mut f32, target: f32, step: f32, accel: f32, most: f32) {
    let d = target - *x;
    if *v == 0.0 && d == 0.0 { return; }
    let s = *v;
    if 0.0 <= s * d {
        let stop = s * s / accel * 0.5;
        if d.abs() < stop {
            let a = if stop < d.abs() + v.abs() { accel } else { accel * 1.1 };
            crate::hero::physics::approach(0.0, a, v);
        } else {
            let want = ((accel + accel) * d).sqrt().min(most);
            crate::hero::physics::approach(if d < 0.0 { -want } else { want }, step, v);
        }
        if v.abs() < d.abs() { *x += *v; } else { *x = target; }
    } else {
        *v = if 0.0 <= s { s - accel } else { s + accel };
        *x += *v;
    }
}

/// `0x41ca40(m, &v)` / `0x41cb68(m, &h)`: a new target speed / height (kept when within 0.1), its easing restarted.
fn set_speed(w: &mut MobyWorld, id: usize, v: f32) {
    let m = &mut w.mobys[id];
    if (m.pv_f(0x5d0) - v).abs() < 0.1 { return; }
    m.set_pv_f(0x5d0, v);
    m.set_pv_i(0x5cc, 0);
    m.set_pv_i(0x5dc, 0);
}
fn set_height(w: &mut MobyWorld, id: usize, h: f32) {
    let m = &mut w.mobys[id];
    if (m.pv_f(0x5d4) - h).abs() < 0.1 { return; }
    m.set_pv_f(0x5d4, h);
    m.set_pv_i(0x58c, 0);
    m.set_pv_i(0x5d8, 0);
}

/// `0x41cdf8`: its guard up (`+0x604`), the class sound 0x10 when it was down.
fn guard(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !w.level23.guarded {
        if let Some(b) = w.level23.boss { ctx.host.play_sound(&w.mobys[b], 0x10, 0); }
    }
    w.mobys[id].set_pv_i(0x604, 1);
}

/// `0x41d4f0` / `0x41d540`: its parts' and its own collision off around its move, and back.
fn parts_solid(w: &mut MobyWorld, id: usize, on: bool) {
    let mut all: Vec<usize> = generators(w, id).into_iter().flatten().collect();
    all.extend([part(w, id, 0x610), part(w, id, 0x614)].into_iter().flatten());
    all.push(id);
    for p in all { w.mobys[p].has_collision = on; }
}

/// A joint rotation (`0x29f978` + `0x283898(rec, axis, a)`) kept as its manipulator.
fn turn_joint(w: &mut MobyWorld, id: usize, joint: u8, axis: usize, a: f32) {
    let mut v = [0.0; 3];
    v[axis] = 1.0;
    let m = &mut w.mobys[id];
    m.manips.retain(|x| x.0 != joint);
    m.manips.push((joint, axis_angle(a, v)));
}

/// `0x41d418(m, a, b)`: the barrels (joints 0x28 / 0x29) spun by `a`·240′ and `b`·480′.
fn barrels(w: &mut MobyWorld, id: usize, a: f32, b: f32) {
    let (x, y) = (w.mobys[id].pv_f(0x580), w.mobys[id].pv_f(0x584));
    turn_joint(w, id, 0x28, 0, x);
    turn_joint(w, id, 0x29, 0, y);
    let m = &mut w.mobys[id];
    m.set_pv_f(0x580, add_rot(x, a * 240.0 * MINUTE));
    m.set_pv_f(0x584, add_rot(y, b * 480.0 * MINUTE));
}

/// Giant Clank idle with his gun put away (`0x18c0b4` 0, `0x317480(him, 0x18b690)`, `0x18b69c` 0, `0x2ce200(0, 1)`).
fn put_gun_away(ctx: &mut Ctx) {
    let g = &mut ctx.hero.giant;
    g.gun = None;
    g.gun_on = false;
    ctx.hero.request_state(0, true);
}

/// `0x41fab0(source, flags, damage, at, dir, save)`: Giant Clank (`0x18c0b0`) hit from `source` (`0x2a9dd8` with a way
/// `dir`, else `0x2a9f30`).
pub fn hit_hero(w: &mut MobyWorld, ctx: &Ctx, source: Option<usize>, flags: u32, damage: f32, at: V3, dir: Option<[f32; 4]>) {
    let Some(h) = ctx.hero_moby.filter(|&h| h < w.mobys.len()) else { return };
    let t = HitTemplate::new(damage, source, flags, dir.unwrap_or([0.0, 0.0, 0.0, 1.0]));
    w.deliver_hit_any(h, &t, [at[0], at[1], at[2], 1.0]);
}

/// `0x418cf8` (module docs).
fn frame(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        set_up(w, id, ctx);
    } else {
        paths::scan(w, ctx, id);
        let st = w.mobys[id].state;
        if !matches!(st, 4 | 0xa | 0xb) {
            for s in spares(w, id).into_iter().flatten() { w.mobys[s].mode |= 0x41; }
        }
        for (off, class) in [(0x620, super::effects::C1252), (0x624, super::effects::C1251)] {
            if part(w, id, off).is_none() {
                let e = w.mobys.iter().position(|m| m.o_class == class && m.state < 0xfd);
                ai::set_link(&mut w.mobys[id], off, e);
            }
        }
    }
    let sc = w.class(CLASS).map_or(1.0, |c| c.scale);
    w.mobys[id].scale = sc * SCALE;
    let mut parts: Vec<usize> = (0..10).filter_map(|i| part(w, id, 0x358 + 4 * i)).collect();
    parts.extend([part(w, id, 0x610), part(w, id, 0x614)].into_iter().flatten());
    for p in parts {
        let c = w.mobys[p].o_class;
        let sc = w.class(c).map_or(1.0, |c| c.scale);
        w.mobys[p].scale = sc * SCALE;
    }
    let g = &mut w.level23;
    if g.intro != 0 {
        g.intro += 1;
        ctx.hero.land_lag = 10;
        w.mobys[id].set_pv_s(paths::BOSS.mode, 0);
        if ctx.hero.items.pressed & 0x50 == 0 {
            if w.level23.intro < 30 { put_gun_away(ctx); }
        } else {
            w.level23.intro = 0;
            w.mobys[id].set_state(1, 0);
            ctx.hero.land_lag = 20;
            ctx.hero.items.pressed = 0;
        }
    }
    let down = paths_down(ctx, w.mobys[id].pos);
    let m = &mut w.mobys[id];
    m.set_pv_v(0x410, down);
    m.rows = turn_rows(&m.rows, upright(&m.rows, scale(down, -1.0)));
    if m.state != 7 {
        let h = m.pv_f(0x588);
        let p = paths::surface(ctx, w.level23.radius, w.mobys[id].pos, h);
        w.mobys[id].pos = p;
    }
    health(w, id, ctx);
}

/// `0x32fb58(p, out, 0)`: the down at `p`.
fn paths_down(ctx: &Ctx, p: V3) -> V3 { scale(ctx.hero.gravity.up_at(p), -1.0) }

/// The set-up (`0x418cf8` in state 0, module docs).
fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let g = &mut w.level23;
    *g = super::Globals {
        intro: 1,
        boss: Some(id),
        drone_health: m.pv_i(0x31c),
        drone_missile: m.pv_f(0x308),
        drone_beam: m.pv_f(0x30c),
        shot_damage: m.pv_f(0x310),
        special_damage: m.pv_f(0x314),
        rider_turn: m.pv_f(0x330),
        drone_missile_speed: m.pv_s(0x338) as f32,
        hero_guard_top: 180,
        relaunch: true,
        circle: CIRCLE,
        special_sound: [-1; 2],
        ..Default::default()
    };
    let m = &mut w.mobys[id];
    for off in [0x620, 0x624, 0x608, 0x5cc, 0x5d8, 0x5b0, 0x5c4, 0x594, 0x590, 0x59c, 0x408, 0x60c] { m.set_pv_i(off, 0); }
    if 8 < m.pv_s(0x334) { m.set_pv_s(0x334, 8); }
    m.set_pv_s(0x336, 2);
    m.set_pv_f(0x5c8, 10.0);
    m.set_pv_f(0x5d0, 10.0);
    m.set_pv_b(0x1b7, 1);
    m.set_pv_f(0x57c, std::f32::consts::PI / 32.0);
    m.set_pv_f(0x578, 0.0);
    let mut k = 0;
    let mut body = None;
    for i in 0..w.mobys.len() {
        if i == id || 0xfd <= w.mobys[i].state { continue; }
        match w.mobys[i].o_class {
            drones::CLASS => w.level23.drones.push(i),
            BODY => body = Some(w.mobys[i].pos),
            targets::F7E if k < 10 => {
                ai::set_link(&mut w.mobys[id], 0x358 + 4 * k, Some(i));
                k += 1;
            }
            targets::T120F => ai::set_link(&mut w.mobys[id], 0x610, Some(i)),
            CAP => ai::set_link(&mut w.mobys[id], 0x614, Some(i)),
            TRIGGER => {
                let t = &mut w.mobys[i];
                t.pvars.resize(t.pvars.len().max(0x10), 0);
                t.update_dist = 0xff;
                t.set_pv_b(0xc, 0xff);
                t.set_pv_b(8, 0);
                t.set_pv_b(0xa, 0);
                w.level23.trigger = Some(i);
            }
            _ => {}
        }
    }
    w.level23.drones.truncate(25);
    let c = ctx.hero.gravity.centre;
    if let Some(p) = body {
        ctx.hero.pos = p;
        w.level23.radius = dist(p, c);
    }
    let r = w.level23.radius;
    // The orbits' shared pickup code (crate::moby::classes::level22::pickups::update) reads this level's radius.
    w.level22.radius = r;
    w.level23.middle = paths::surface(ctx, r, [500.0, 500.0, 430.0], 0.0);
    w.level23.top = paths::surface(ctx, r, [500.0, 500.0, 552.0], 0.0);
    w.mobys[id].pos = paths::surface(ctx, r, [537.0, 511.0, 533.0], H_LAND);
    let m = &mut w.mobys[id];
    (m.update_dist, m.draw_dist) = (0xff, 0xff);
    m.mode |= 0x100;
    let most = m.pv_s(0x2f6) as f32;
    w.level23.t120f_health = most * m.pv_f(0x328);
    w.level23.f7e_health = most * m.pv_f(0x320) * 0.2;
    w.level23.f7e_floor = w.level23.f7e_health * f32::from_bits(0x3daa_aaab);
    m.set_pv_f(0x17c, 0.15);
    m.set_pv_f(0x20, most);
    m.set_pv_s(0x24, m.pv_s(0x2f6));
    m.set_pv_f(0x170, 0.5);
    m.set_pv_f(0x174, 0.5);
    m.set_pv_b(0x150, 0x14);
    m.set_pv_i(0x188, -1);
    m.set_pv_f(0x32c, 1.0 - (m.pv_f(0x324) + m.pv_f(0x320) + m.pv_f(0x328)));
    for (off, v) in [(0x84, 0.5), (0x54, -7.0), (0x50, 7.0), (0x5c, 0.4), (0x60, f32::from_bits(0x3fc9_0fdc)), (0x80, f32::from_bits(0x3fc6_d3f3)), (0x58, 0.4)] {
        m.set_pv_f(off, v);
    }
    m.set_pv_i(0x88, m.pv_i(0x88) | 5);
    ai::senses(m, f32::from_bits(0x3c9f_49f5), f32::from_bits(0x3f22_2223), f32::from_bits(0x3ae4_c38a), f32::from_bits(0x3e56_7752));
    m.set_state(2, 0);
    let b = paths::BOSS;
    for off in [b.hist, b.hist + 2, b.hist + 4] { m.set_pv_s(off, -1); }
    m.set_pv_s(b.mode, 0);
    m.set_pv_s(b.hist_i, 0);
    paths::load(w, ctx, id);
    paths::link_paths(w);
    pickups::set_up(w, id, ctx);
    let m = &mut w.mobys[id];
    m.set_pv_f(0x588, HIGH);
    m.set_pv_i(0x88, m.pv_i(0x88) | 0x10);
    m.bangles = 0xf;
}

/// `0x41ce68`: the generator to turn to him — the healthiest above the floor; −1 once none is left (for good).
fn pick_generator(w: &mut MobyWorld, id: usize) {
    if w.mobys[id].pv_i(0x60c) == -1 { return; }
    let floor = w.level23.f7e_floor;
    let mut best = (-1, 0.0);
    for (i, g) in generators(w, id).into_iter().enumerate() {
        let Some(g) = g else { continue };
        let v = w.mobys[g].pv_f(0x20);
        if floor < v && best.1 < v { best = (i as i32, v); }
    }
    w.mobys[id].set_pv_i(0x60c, best.0);
}

/// `0x41cef0` (module docs).
fn health(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    pick_generator(w, id);
    countdown(&mut w.level23.hurt_wait);
    let gi = w.mobys[id].pv_i(0x60c);
    let core = part(w, id, 0x610);
    let up = |w: &MobyWorld| { let m = &w.mobys[id]; add(m.pos, scale(m.rows[2], 4.0)) };
    if gi == -1 && core.is_none() {
        w.level23.aim = up(w);
        let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x30000 != 0);
        w.clear_hit(id);
        let mut k = w.mobys[id].pv_s(0x26) as i32;
        countdown(&mut k);
        w.mobys[id].set_pv_s(0x26, k as i16);
        let guarded = w.mobys[id].pv_i(0x604) != 0;
        let refused = hit.is_some_and(|h| w.mobys[id].pv_s(0x26) != 0 || guarded || (h.tmpl.source.is_some() && h.tmpl.source == ctx.hero_moby && w.level23.hero_guard < 0));
        if refused && guarded && w.mobys[id].pv_s(0x26) == 0 { targets::hurt_sound(w, ctx, Some(id)); }
        if let Some(h) = hit.filter(|_| !refused) {
            let m = &mut w.mobys[id];
            m.set_pv_b(0x147, 0xff);
            ai::flash_start(m, 0x140);
            if w.mobys[id].bangles != 0 {
                let yaw = crate::moby::attach::angles_of(&ctx.host.camera_rows())[2];
                for i in 0..4u8 {
                    let a = add_rot(ctx.game.rng.randf_signed(90.0, 120.0) * 0.017_453_294, yaw);
                    let s = ctx.game.rng.randf(2.0, 4.0) * 0.016_666_668;
                    let v = [a.cos() * s, a.sin() * s, ctx.game.rng.randf(5.0, 8.0) * 0.016_666_668];
                    let (pos, rot) = (w.mobys[id].pos, w.mobys[id].rot);
                    crate::moby::shards::spawn_ball(w, ctx, CLASS, Some(id), pos, rot, 90.0, 15.0, i, v, 0xff);
                }
                w.mobys[id].bangles = 0;
            }
            if let Some(src) = h.tmpl.source.filter(|&s| s < w.mobys.len()) { targets::blast_at(w, ctx, src, Some(id)); }
            let m = &mut w.mobys[id];
            m.set_pv_f(0x20, m.pv_f(0x20) - h.tmpl.damage);
            m.set_pv_s(0x26, 20);
            if m.pv_f(0x20) <= 0.0 {
                m.set_state(4, 0);
                put_gun_away(ctx);
                return;
            }
        }
        ai::flash(&mut w.mobys[id], 0x140);
        return;
    }
    let m = &w.mobys[id];
    let most = m.pv_s(0x2f6) as f32;
    let mut total = m.pv_f(0x32c) * most;
    if gi != -1 {
        if let Some((r, _)) = ctx.host.joint_matrix(&w.mobys[id], gi as usize + 1) {
            w.level23.aim = add(w.mobys[id].pos, scale(r[1], 8.0));
        }
        let floor = w.level23.f7e_floor;
        for g in generators(w, id).into_iter().flatten() {
            let v = w.mobys[g].pv_f(0x20);
            if floor < v { total += v; }
        }
    } else {
        w.level23.aim = up(w);
    }
    if let Some(c) = core {
        let h = w.mobys[c].pv_f(0x20);
        if 0.0 <= h {
            total += h;
        } else {
            w.delete(c);
            ai::set_link(&mut w.mobys[id], 0x610, None);
            w.mobys[id].set_pv_s(0x26, 30);
        }
    }
    let m = &mut w.mobys[id];
    if m.bangles != 0 { total += m.pv_f(0x324) * most; }
    m.set_pv_f(0x20, total);
}

/// `0x41cbd8` (module docs).
fn shield(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &w.mobys[id];
    let a = m.pv_f(0x570);
    if m.pv_i(0x604) != 0 {
        turn_joint(w, id, 0x22, 2, a);
        w.mobys[id].set_pv_f(0x570, add_rot(a, 60.0 * MINUTE));
        return;
    }
    let gi = m.pv_i(0x60c);
    if gi == -1 { return; }
    let (own, hero, pos, down) = (m.pv_v(0x3e0), m.pv_v(0x3f0), m.pos, m.pv_v(0x410));
    let b = paths::surface(ctx, w.level23.radius, add(set_len(sub(hero, own), 10.0), pos), 0.0);
    let to = set_len(sub(b, own), 1.0);
    let Some((r, _)) = ctx.host.joint_matrix(&w.mobys[id], gi as usize + 1) else { return };
    let c = r[1];
    if 0.95 <= dot(c, to) { return; }
    let f = if 0.0 < dot(cross(c, to), unit(down)) { 60.0 } else { -60.0 };
    turn_joint(w, id, 0x22, 2, a);
    w.mobys[id].set_pv_f(0x570, add_rot(a, -(f * MINUTE)));
}

/// `0x419660` (module docs: a launch): the next walker from the joint `+0x594` + 30, 16 down its row 2 and 2.2 out
/// from the middle of it, its state 1; the wait 30 (in 0xc / 0x10) or 20; the class sound 4.
fn launch(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let k = w.level23.next_drone.rem_euclid(25) as usize;
    let Some(&d) = w.level23.drones.get(k) else { return };
    let m = &mut w.mobys[id];
    m.set_pv_i(0x598, if matches!(m.state, 0xc | 0x10) { 30 } else { 20 });
    ai::set_link(m, 0x59c, Some(d));
    m.set_pv_i(0x590, m.pv_i(0x590) - 1);
    let j = m.pv_i(0x594) as usize + 0x1e;
    if let Some(p) = place_at_joint(w, id, ctx, j, 2.2) {
        if w.mobys[d].state != 2 {
            let rows = w.mobys[id].rows;
            let dm = &mut w.mobys[d];
            dm.rows = rows;
            dm.pos = p;
        }
    }
    w.mobys[d].set_state(1, 0);
    ctx.host.play_sound(&w.mobys[id], 4, 0);
    w.level23.next_drone = (k as i32 + 1) % 25;
}

/// A launch point: joint `j`, 16 down its row 2, `out` away from its flat place along the way to the joint's.
fn place_at_joint(w: &MobyWorld, id: usize, ctx: &Ctx, j: usize, out: f32) -> Option<V3> {
    let a = ctx.host.joint(&w.mobys[id], j)?;
    let (r, _) = ctx.host.joint_matrix(&w.mobys[id], j)?;
    let b = scale(r[2], -16.0);
    let c = paths::surface(ctx, w.level23.radius, a, 0.0);
    let d = scale(sub(c, w.mobys[id].pv_v(0x3e0)), out);
    Some(add(a, add(b, d)))
}

/// `0x4198f0` (module docs: the walkers' tick): the launch under way timed out (the joint the next); the walkers out
/// counted; with none fought, the nearest that may fight Giant Clank picked (crate::moby::classes::level23::drones);
/// the launches left made when the wait is out.
fn drone_tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ai::link(&w.mobys[id], 0x59c).is_some() {
        let m = &mut w.mobys[id];
        let mut t = m.pv_i(0x598);
        countdown(&mut t);
        m.set_pv_i(0x598, t);
        if t == 0 {
            ai::set_link(m, 0x59c, None);
            m.set_pv_i(0x594, (m.pv_i(0x594) + 1) % 4);
        }
    }
    let list = w.level23.drones.clone();
    w.level23.drones_out = list.iter().filter(|&&d| w.mobys[d].mode & 1 == 0).count() as i32;
    if w.level23.selected.is_none() {
        let mut best: Option<(usize, f32)> = None;
        for &d in &list {
            if w.mobys[d].mode & 1 != 0 { continue; }
            let Some(dd) = drones::may_fight(w, d, ctx) else { continue };
            if best.map_or(true, |(_, b)| dd < b) { best = Some((d, dd)); }
        }
        if let Some((d, _)) = best { drones::select(w, d, ctx); }
    }
    let m = &w.mobys[id];
    if m.pv_i(0x590) != 0 && m.pv_i(0x598) <= 0 { launch(w, id, ctx); }
}

/// `0x41b188` (module docs: the circle): within the band of its distance it stays; else along its slot toward him
/// (mode 3) or away (2), turning back when the side changes; moved 1/16 of the miss a tick (0.035..0.7).
fn circle(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let b = paths::BOSS;
    let r = w.level23.radius;
    let h = w.mobys[id].pv_f(0x588);
    let way = paths::surface(ctx, r, w.mobys[id].pv_v(b.way), h);
    w.mobys[id].set_pv_v(b.way, way);
    let (mid, x) = (w.level23.circle, w.mobys[id].pv_f(0x5b8));
    if mid + BAND <= x || x <= mid - BAND {
        let m = &mut w.mobys[id];
        let mut flip = false;
        let side = if mid - BAND < x {
            if 2 <= m.pv_i(0x5f0) as u32 { flip = true; }
            m.set_pv_i(0x5f0, 1);
            3
        } else {
            let s = m.pv_i(0x5f0);
            if s != 0 && s != 2 { flip = true; }
            m.set_pv_i(0x5f0, 2);
            2
        };
        if flip { m.set_pv_s(b.dir, -m.pv_s(b.dir)); }
        let dev = (x - mid).abs();
        if flip || dist(w.mobys[id].pos, way) < 2.0 {
            let m = &w.mobys[id];
            let (k, i, dir) = (m.pv_s(b.slot).max(0) as usize, m.pv_s(b.point) as i32, m.pv_s(b.dir) as i32);
            let step = i + dir;
            let links = w.level23.paths.links.get(k).copied().unwrap_or([0; 2]);
            let goal = w.mobys[id].pv_v(0x400);
            let flat = (m.pv_v(0x3f0), m.pv_v(0x3e0));
            if step < 0 {
                paths::next_slot(w, ctx, id, b, links[0], side, true, goal, flat);
            } else if paths::slot(w, k).len() as i32 <= step {
                paths::next_slot(w, ctx, id, b, links[1], side, true, goal, flat);
            } else {
                w.mobys[id].set_pv_s(b.point, step as i16);
            }
            let m = &w.mobys[id];
            let (k, i) = (m.pv_s(b.slot).max(0) as usize, m.pv_s(b.point).max(0) as usize);
            if let Some(&q) = paths::slot(w, k).get(i) {
                let q = paths::surface(ctx, r, q, h);
                w.mobys[id].set_pv_v(b.way, q);
            }
        }
        let d = (dev * 0.0625).clamp(0.7 * 0.05, 0.7);
        let way = w.mobys[id].pv_v(b.way);
        let v = set_len(sub(way, w.mobys[id].pos), d);
        let p = paths::surface(ctx, r, add(w.mobys[id].pos, v), h);
        w.mobys[id].pos = p;
    }
    w.level23.moved = true;
}

/// `0x419bd0(m, h)` + its move: the walk to its goal `+0x400` (crate::moby::classes::level23::paths::walk), at its
/// speed (`+0x5c8`/60) by the body's move with its parts' collision off.
fn walk_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3) {
    w.mobys[id].set_pv_v(0x400, goal);
    let m = &w.mobys[id];
    let (h, flat) = (m.pv_f(0x588), (m.pv_v(0x3f0), m.pv_v(0x3e0)));
    let target = paths::walk(w, ctx, id, goal, h, flat);
    let v = set_len(sub(target, w.mobys[id].pos), w.mobys[id].pv_f(0x5c8) * 0.016_666_668);
    parts_solid(w, id, false);
    frames::move_by(w, id, ctx, v);
    parts_solid(w, id, true);
    w.level23.moved = true;
}

/// `0x41d070` (module docs: the ring off): its sounds and beams gone, its hit's wait cleared.
fn ring_off(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    if m.pv_i(0x5b0) != 0 {
        m.set_pv_i(0x5c4, 0);
        m.set_pv_i(0x5b0, 0);
    }
}

/// `0x41bca0` (module docs: the ring): its radius grows by 0.1 (to 13.6) from the second half of its 110 ticks; eight
/// spokes of three points each turned about it 300′ a tick (×0.98..1.02) at 5 up; at full size, Giant Clank within 45
/// of the middle and under 10 up is hit 35 (flags 3; a blast at his row 2·3, the opponent's class sound 2 on him) every
/// 25 ticks, and the first spoke's lines hit what they meet (10, flags 0x8b0000) every 2..16 ticks.
fn ring(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let g = &mut w.level23;
    if w.mobys[id].pv_i(0x5b0) == 0 {
        g.ring_r = 0.1;
        g.ring_grow = 0.1;
        g.ring_t = 110;
        ctx.host.play_sound(&w.mobys[id], 5, 0);
    } else {
        countdown(&mut g.ring_t);
        if g.ring_t < 55 { g.ring_r = (g.ring_r + g.ring_grow).min(13.6); }
    }
    w.mobys[id].set_pv_i(0x5b0, 1);
    let r = w.level23.radius;
    let at = paths::surface(ctx, r, w.mobys[id].pos, 5.0);
    w.level23.ring_at = at;
    let k = ctx.game.rng.randf(0.98, 1.02);
    let m = &mut w.mobys[id];
    let a = add_rot(m.pv_f(0x380), 300.0 * k * MINUTE);
    m.set_pv_f(0x380, a);
    let rows = crate::moby::attach::turn(&crate::moby::attach::rows_of([0.0, 0.0, a]), &m.rows);
    let (s0, s1) = (scale(rows[0], r), scale(rows[1], r));
    let v0 = s0;
    let v4 = scale(v0, -1.0);
    let v1 = scale(add(s0, s1), 0.5);
    let v2 = s1;
    let v3 = scale(add(s1, v4), 0.5);
    let verts = [v0, v1, v2, v3, v4, scale(v1, -1.0), scale(v2, -1.0), scale(v3, -1.0)];
    let rr = w.level23.ring_r;
    for (i, v) in verts.into_iter().enumerate() {
        let v = paths::surface(ctx, r, add(v, at), 5.0);
        let step = set_len(sub(paths::surface(ctx, r, v, 5.0), at), rr);
        let mut cur = at;
        for k in 0..3 {
            let p = paths::surface(ctx, r, add(cur, step), 5.0);
            w.level23.ring_pts[i][k] = p;
            cur = add(cur, step);
        }
    }
    if w.level23.ring_r != 13.6 { return; }
    if dist(ctx.hero.pos, w.level23.middle) < RING_REACH && w.mobys[id].pv_f(0x5c0) < 10.0 && w.mobys[id].pv_i(0x5c4) == 0 {
        let up = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or([0.0, 0.0, 1.0], |h| h.rows[2]);
        let b = add(ctx.hero.pos, scale(up, 3.0));
        burst(w, ctx, b, &Blast { size: 2.5, bursts: [35, 20, 20], puffs: 15, streaks: 25, light: true, shake: true });
        let pos = w.mobys[id].pos;
        hit_hero(w, ctx, Some(id), 3, 35.0, pos, None);
        w.mobys[id].set_pv_i(0x5c4, 25);
        if let Some(h) = ctx.hero_moby { ctx.host.play_sound_of(&w.mobys[h], CLASS, 2, 0); }
    }
    let m = &mut w.mobys[id];
    let mut t = m.pv_i(0x5c4);
    countdown(&mut t);
    m.set_pv_i(0x5c4, t);
    let n = ctx.game.rng.rand_range(1, 4);
    if (w.mobys[id].state_ticks as u16 as u32) & ((1 << n) - 1) != 0 { return; }
    let mut t = HitTemplate::new(10.0, Some(id), 0x8b_0000, [0.0, 0.0, 0.0, 1.0]);
    t.kind = (CLASS as u16 as u32) << 16 | 0x303;
    for c in 1..3 {
        let (a, b) = (w.level23.ring_pts[0][c - 1], w.level23.ring_pts[0][c]);
        if let Some(hit) = w.coll_line(ctx, a, b, 0, None) {
            if let Some(o) = hit.moby { w.deliver_hit_any(o, &t, [hit.point[0], hit.point[1], hit.point[2], 1.0]); }
        }
    }
}

/// `0x41b260` (module docs: the hatch): in states 8..10 it opens over 100 ticks (0 → 1 → 2), out of them it shuts.
fn hatch(w: &mut MobyWorld, id: usize) {
    let g = &mut w.level23;
    if (8..=10).contains(&w.mobys[id].state) {
        match g.hatch {
            0 => {
                g.hatch = 1;
                g.hatch_t = 100;
            }
            2 => g.hatch_t = 100,
            _ => {
                g.hatch_t -= 1;
                if g.hatch_t == -1 { g.hatch = 2; }
            }
        }
    } else if g.hatch != 0 {
        g.hatch_t -= 1;
        if g.hatch_t == -1 { g.hatch = 0; }
    }
}

/// `0x41b538(m, at)` (module docs: the turret's aim at `at`); true when it turned to it.
fn aim(w: &mut MobyWorld, id: usize, ctx: &Ctx, at: V3) -> bool {
    let Some((r, t)) = ctx.host.joint_matrix(&w.mobys[id], 0x27) else { return false };
    w.level23.muzzle = t;
    w.level23.muzzle_way = r[0];
    if w.level23.hatch != 0 { return false; }
    let Some(axis) = ctx.host.joint(&w.mobys[id], 0x2a) else { return false };
    let dir = set_len(sub(at, axis), 1.0);
    w.level23.aim_dot = dot(dir, w.level23.muzzle_way);
    w.level23.aim_clear = w.coll_line(ctx, w.level23.test_from, w.level23.chest, 0x10, ctx.hero_moby).is_none();
    let m = &w.mobys[id];
    let speed = 80.0 * MINUTE;
    let a = m.pv_f(0x578);
    if m.pv_b(0x404) != 0 && m.pv_b(0x405) != 0 {
        let c = cross(dir, w.level23.muzzle_way);
        let v = if dot(c, m.rows[1]) < 0.0 { -speed } else { speed };
        let a = add_rot(a, v);
        turn_joint(w, id, 0x27, 1, a);
        let m = &mut w.mobys[id];
        m.set_pv_f(0x57c, v);
        m.set_pv_f(0x578, add_rot(a, v * MINUTE));
        barrels(w, id, 1.0, 1.0);
        return true;
    }
    if m.pv_b(0x404) == 0 {
        let v = -(30.0 * MINUTE);
        w.mobys[id].set_pv_f(0x57c, v);
        if 0.0 < a {
            let a = a + v;
            turn_joint(w, id, 0x27, 1, a);
            w.mobys[id].set_pv_f(0x578, add_rot(a, v * MINUTE));
            barrels(w, id, -1.0, -1.0);
        }
    }
    false
}

/// `0x41b8b8(m, at)` (module docs: the turret's fire at `at`): in 0xf a burst every 4 ticks (×1.5 damage) along it;
/// in 0xc / 0x10 with the line clear and the turret within 0.85 of it, every `2^(9 − rate)` ticks: in 0x10 a missile
/// 0x1230 (`+0x304`, a small blast and the level sound 0x1a), in 0xc a shot 0x118e (`+0x300`, the level sound 0x21).
fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: V3) {
    let m = &w.mobys[id];
    let st = m.state;
    let t = m.state_ticks as u16 as i32;
    let muzzle = w.level23.muzzle;
    let way = w.level23.muzzle_way;
    if st == 0xf {
        if t & 3 != 2 { return; }
        let v = set_len(way, m.pv_s(0x33a) as f32 * 0.016_666_668);
        let d = m.pv_f(0x300) * 1.5;
        shots::turret_shot(w, ctx, muzzle, v, d);
        ctx.host.play_level_sound_after(None, 0x21, 0);
        return;
    }
    if !matches!(st, 0xc | 0x10) || !w.level23.aim_clear || w.level23.aim_dot < f32::from_bits(0x3f59_999a) { return; }
    barrels(w, id, 2.0, 2.0);
    let m = &w.mobys[id];
    let rate = if st == 0x10 { m.pv_s(0x336) } else { m.pv_s(0x334) } as i32;
    let mask = (1i32 << (9 - rate).clamp(0, 15)) - 1;
    if t & mask != mask { return; }
    if st == 0x10 {
        let v = set_len(sub(at, muzzle), m.pv_s(0x33c) as f32 * 0.016_666_668);
        let d = m.pv_f(0x304);
        shots::missile(w, ctx, id, v, muzzle, d);
        burst(w, ctx, muzzle, &Blast { size: 0.5, bursts: [2, 3, 2], puffs: 2, streaks: 10, light: false, shake: false });
        ctx.host.play_level_sound_after(None, 0x1a, 0);
    } else {
        // The game's: the way is from the turret's way vector (not its muzzle) to the point.
        let v = set_len(sub(at, way), m.pv_s(0x33a) as f32 * 0.016_666_668);
        let d = m.pv_f(0x300);
        shots::turret_shot(w, ctx, muzzle, v, d);
        ctx.host.play_level_sound_after(None, 0x21, 0);
    }
}

/// `0x41d5e8` (module docs: the shock wave): landed in the middle (0x10, him within 14 flat, above it by under 10,
/// on the ground over 10 ticks), its count set going (420, or 180 a second time); at 180 the class sounds 10 and 0x11;
/// at 0 the wave 0x11df (crate::moby::classes::level23::shots::wave), the sounds 7 and 0xc and, him within 15 above it
/// and not kept from harm, a 70 hit pushed 20 from it; then for 240 ticks, him on the ground within 10 above it, a 70
/// hit from the wave pushed 3 every 90 ticks.
fn shock(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    countdown(&mut w.level23.guard_wait);
    let m = &w.mobys[id];
    let (flat, h, hh, d) = (m.pv_f(0x5b8), m.pv_f(0x588), m.pv_f(0x5c0), m.pv_f(0x5bc));
    let ground = ctx.hero.grounded_ticks;
    let top = w.level23.hero_guard_top;
    let g = &mut w.level23;
    if m.state == 0x10 && flat < 14.0 && h < hh && (hh - h).abs() < 10.0 && 10 < ground && g.hero_guard == 0 {
        g.guard_hit = true;
        g.hero_guard = 420;
        if g.guard_again {
            g.guard_again = false;
            g.hero_guard = top;
        }
    }
    if 0 < g.hero_guard {
        if g.hero_guard == top {
            if g.guard_hit { g.guard_wait = 0; } else { g.hero_guard += 2; }
        } else if 10 < ctx.hero.air_ticks {
            g.guard_again = true;
        }
    }
    if g.guard_again && top < g.hero_guard && 10 < ground { g.hero_guard = top; }
    if ground != 0 && ctx.hero.ground_moby.is_none() { g.guard_again = false; }
    if 10 < ground && !g.guard_hit && 0 < g.hero_guard && g.hero_guard <= top { g.hero_guard = top + 5; }
    if g.hero_guard == top {
        ctx.host.play_sound(&w.mobys[id], 10, 0);
        ctx.host.play_sound(&w.mobys[id], 0x11, 0);
    }
    let pos = w.mobys[id].pos;
    let hp = ctx.hero.pos;
    let push = move |s: f32| { let v = set_len(sub(hp, pos), s); [v[0], v[1], v[2], f32::from_bits(0x45af_df66)] };
    if w.level23.hero_guard < 0 {
        w.level23.hero_guard += 1;
        if w.level23.guard_wait == 0 && ground != 0 && d < 10.0 && h < hh {
            let src = ai::link(&w.mobys[id], 0x408).filter(|&s| s < w.mobys.len());
            let dir = push(3.0);
            hit_hero(w, ctx, src, 0x1_0007, 70.0, pos, Some(dir));
            w.level23.guard_wait = 90;
            let mid = w.level23.mid;
            burst(w, ctx, mid, &Blast { size: 3.0, bursts: [5, 0x46, 3], puffs: 15, streaks: 20, light: true, shake: true });
        }
    } else if 0 < w.level23.hero_guard {
        countdown(&mut w.level23.hero_guard);
        if w.level23.hero_guard == 0 {
            let wave = shots::wave(w, ctx, pos, id);
            ai::set_link(&mut w.mobys[id], 0x408, wave);
            w.level23.hero_guard = -240;
            ctx.host.play_sound(&w.mobys[id], 7, 0);
            ctx.host.play_sound(&w.mobys[id], 0xc, 0);
            if d < 15.0 && h < hh && ctx.hero.damage.invincible == 0 {
                let dir = push(20.0);
                hit_hero(w, ctx, Some(id), 0x1_0007, 70.0, pos, Some(dir));
                let mid = w.level23.mid;
                burst(w, ctx, mid, &Blast { size: 3.0, bursts: [5, 0x46, 3], puffs: 15, streaks: 20, light: true, shake: true });
            }
        }
    }
}

/// The shadow mover 0x0d55 kept at it (`+0x40c`; L23 `0x400c08` / `0x400ca8`, the same as Dobbo orbit's).
fn shadow(w: &mut MobyWorld, id: usize) {
    let pos = w.mobys[id].pos;
    match part(w, id, 0x40c) {
        None => {
            let s = super::super::level22::boss::make_shadow(w, id, pos);
            ai::set_link(&mut w.mobys[id], 0x40c, Some(s));
        }
        Some(s) => w.mobys[s].set_pv_v(0, pos),
    }
}

/// 0x0f7f (`0x417888`, module docs).
pub fn update_0f7f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    pv_grow(w, id);
    frame(w, id, ctx);
    if w.level23.boss.is_none() || 200 < w.level23.intro { return; }
    w.level23.moved = false;
    let was = w.mobys[id].pv_i(0x604) != 0;
    w.level23.guarded = was;
    w.level23.open = !was && w.mobys[id].pv_i(0x60c) != -1;
    shield(w, id, ctx);
    w.mobys[id].set_pv_i(0x604, 0);
    let up = ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or([0.0, 0.0, 1.0], |h| h.rows[2]);
    let hero = ctx.hero.pos;
    w.level23.chest = add(hero, scale(up, 8.0));
    w.level23.mid = add(hero, scale(up, 4.0));
    let r = w.level23.radius;
    let (hf, of) = (paths::surface(ctx, r, hero, 0.0), paths::surface(ctx, r, w.mobys[id].pos, 0.0));
    let hh = ctx.hero.gravity.height(hero) - r;
    let m = &mut w.mobys[id];
    m.set_pv_v(0x3f0, hf);
    m.set_pv_v(0x3e0, of);
    m.set_pv_f(0x5c0, hh);
    w.level23.above = m.pv_f(0x588) < hh;
    m.set_pv_f(0x5b8, dist(hf, of));
    drone_tick(w, id, ctx);
    let d = dist(w.mobys[id].pos, hero);
    w.mobys[id].set_pv_f(0x5bc, d);
    let chest = w.level23.chest;
    let mut from = chest;
    if let (Some(jo), Some((jr, jt))) = (ctx.host.joint(&w.mobys[id], 0x2a), ctx.host.joint_matrix(&w.mobys[id], 0x27)) {
        w.level23.test_from = add(scale(jr[0], 0.5), jo);
        from = add(jt, set_len(sub(chest, jt), 4.0));
    }
    let clear = d < 80.0 && w.coll_line(ctx, from, chest, 0x10, ctx.hero_moby).is_none();
    w.mobys[id].set_pv_b(0x404, clear as u8);
    if w.level23.drones_out <= OUT {
        countdown(&mut w.level23.launch_wait);
        let m = &w.mobys[id];
        if m.state == 7 && (!clear || m.drawn == 0 || 75.0 < d) && w.level23.launch_wait == 0 && m.pv_i(0x590) == 0 {
            w.level23.launch_wait = 90;
            let k = w.level23.next_drone.rem_euclid(25) as usize;
            if let Some(&dr) = w.level23.drones.get(k) { w.mobys[dr].set_state(6, 0); }
            w.level23.next_drone = (k as i32 + 1) % 25;
        }
    }
    w.mobys[id].set_pv_b(0x405, 0);
    let m = &w.mobys[id];
    if m.state != 4 && (d < 80.0 || m.state == 0x10) && (14.0 <= m.pv_f(0x5b8) || m.pv_f(0x5c0) <= m.pv_f(0x588)) {
        let f = frames::face_point(w, id, hero);
        w.mobys[id].set_pv_b(0x405, f as u8);
    }
    if !states(w, id, ctx) { return; }
    after(w, id, ctx);
}

/// The states (module docs); false: the tick ends there.
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    let (middle, top) = (w.level23.middle, w.level23.top);
    let rand = |ctx: &mut Ctx, a: f32, b: f32| ctx.game.rng.randf(a, b);
    match w.mobys[id].state {
        4 => {
            dying(w, id, ctx);
            return false;
        }
        2 => {
            if w.mobys[id].first() {
                set_speed(w, id, SPEED_B);
                set_height(w, id, H_LAND);
                turn_joint(w, id, 0x27, 1, f32::from_bits(0x3fbc_7edd));
                w.mobys[id].set_pv_f(0x578, f32::from_bits(0x3fbc_7edd));
            }
        }
        3 => {
            walk_to(w, id, ctx, middle);
            ring_off(w, id);
            w.mobys[id].set_state(7, 1);
        }
        1 => {
            guard(w, id, ctx);
            if w.mobys[id].first() {
                set_speed(w, id, SPEED_B);
                set_height(w, id, H_TOP);
            }
            if dist(w.mobys[id].pv_v(0x3e0), middle) < 10.0 { w.mobys[id].set_state(3, 0); } else { walk_to(w, id, ctx, middle); }
        }
        7 => {
            guard(w, id, ctx);
            let mut t = w.mobys[id].pv_i(0x608);
            countdown(&mut t);
            w.mobys[id].set_pv_i(0x608, t);
            if w.mobys[id].first() {
                set_speed(w, id, SPEED_A);
                set_height(w, id, H_TOP);
                w.level23.circle = CIRCLE + rand(ctx, 5.0, 45.0);
            }
            if w.mobys[id].pv_i(0x60c) == -1 {
                w.mobys[id].set_state(6, 1);
            } else if w.level23.drones_out <= OUT && w.mobys[id].pv_i(0x608) == 0 {
                w.level23.circle = if w.level23.launches % 3 == 2 { CIRCLE } else { CIRCLE + rand(ctx, 25.0, 55.0) };
                w.mobys[id].set_state(8, 1);
                w.level23.launches += 1;
            } else {
                circle(w, id, ctx);
                if (w.mobys[id].state_ticks as u16) % 480 == 0 {
                    w.level23.circle = if w.level23.drones_out < 4 { CIRCLE + 60.0 } else { CIRCLE + rand(ctx, -4.0, 80.0) };
                }
            }
        }
        8 => {
            guard(w, id, ctx);
            w.mobys[id].first();
            let x = w.mobys[id].pv_f(0x5b8);
            if w.level23.circle - 6.0 <= x && x <= w.level23.circle + 6.0 { w.mobys[id].set_state(9, 1); } else { circle(w, id, ctx); }
        }
        9 => {
            if w.mobys[id].first() {
                circle(w, id, ctx);
                set_height(w, id, H_LOW);
                w.mobys[id].set_pv_i(0x598, 140);
                ctx.host.play_sound(&w.mobys[id], 6, 0);
            } else if w.mobys[id].pv_i(0x5d8) != 0 {
                if w.mobys[id].pv_i(0x598) <= 0 {
                    w.mobys[id].set_state(10, 1);
                } else {
                    if w.mobys[id].pv_i(0x598) == 140 { burst_launch(w, id, ctx); }
                    let m = &mut w.mobys[id];
                    let mut t = m.pv_i(0x598);
                    countdown(&mut t);
                    m.set_pv_i(0x598, t);
                }
            }
        }
        10 => {
            if w.mobys[id].pv_i(0x60c) == -1 { guard(w, id, ctx); }
            if w.mobys[id].first() { set_height(w, id, H_TOP); }
            if H_MID < w.mobys[id].pv_f(0x588) {
                let m = &mut w.mobys[id];
                m.set_pv_i(0x598, 0);
                m.set_pv_i(0x590, 4);
                m.set_state(11, 1);
                let gens = generators(w, id);
                for (i, s) in spares(w, id).into_iter().enumerate() {
                    let Some(s) = s else { continue };
                    if let Some(g) = gens[i] {
                        let (rows, pos) = (w.mobys[g].rows, w.mobys[g].pos);
                        let sm = &mut w.mobys[s];
                        (sm.rows, sm.pos) = (rows, pos);
                    }
                    let sm = &mut w.mobys[s];
                    sm.mode &= 0xbff9;
                    sm.has_collision = true;
                }
            }
        }
        11 => {
            if w.mobys[id].first() { w.level23.circle = CIRCLE; }
            if w.mobys[id].pv_i(0x60c) == -1 { guard(w, id, ctx); }
            let m = &w.mobys[id];
            if m.pv_i(0x5d8) != 0 && m.pv_i(0x598) == 0 && m.pv_i(0x590) == 0 {
                for s in spares(w, id).into_iter().flatten() {
                    let sm = &mut w.mobys[s];
                    sm.has_collision = false;
                    sm.mode |= 0x41;
                }
                let m = &mut w.mobys[id];
                m.set_pv_i(0x608, 120);
                if m.drawn != 0 && m.pv_b(0x404) != 0 {
                    m.set_state(13, 2);
                    ctx.host.play_sound(&w.mobys[id], 8, 0);
                } else {
                    m.set_state(7, 1);
                }
            } else if m.pv_i(0x590) != 0 && m.pv_i(0x598) == 0 {
                launch(w, id, ctx);
            }
            circle(w, id, ctx);
        }
        13 => {
            guard(w, id, ctx);
            if w.mobys[id].state_ticks != 0 { w.mobys[id].set_state(14, 2); }
            circle(w, id, ctx);
        }
        14 => {
            guard(w, id, ctx);
            if 40 <= w.mobys[id].state_ticks as u16 { w.mobys[id].set_state(15, 2); }
            circle(w, id, ctx);
        }
        15 => {
            guard(w, id, ctx);
            w.mobys[id].first();
            if 40 <= w.mobys[id].state_ticks as u16 { w.mobys[id].set_state(7, 1); }
        }
        12 => {
            let m = &w.mobys[id];
            if m.pv_i(0x598) == 0 && m.pv_i(0x590) == 0 {
                let m = &mut w.mobys[id];
                m.set_state(0x10, 2);
                m.set_pv_i(0x608, 120);
            } else if m.pv_i(0x590) != 0 && m.pv_i(0x598) == 0 {
                launch(w, id, ctx);
            }
        }
        19 => {
            if w.mobys[id].first() {
                set_height(w, id, H_LOW);
            } else if 300 < w.mobys[id].state_ticks as u16 {
                w.mobys[id].set_state(7, 0);
            }
            if 0.0 < w.mobys[id].pv_f(0x588) { circle(w, id, ctx); }
        }
        5 => {
            if w.mobys[id].first() {
                w.mobys[id].set_pv_v(0x400, top);
                set_speed(w, id, SPEED_A);
            } else if 1.0 < dist(w.mobys[id].pv_v(0x3e0), top) {
                walk_to(w, id, ctx, top);
            } else {
                w.mobys[id].set_state(0x11, 2);
            }
        }
        6 => {
            guard(w, id, ctx);
            if w.mobys[id].first() {
                w.mobys[id].set_pv_v(0x400, middle);
                set_speed(w, id, SPEED_A);
                set_height(w, id, H_TOP);
            } else if 1.0 < dist(w.mobys[id].pv_v(0x3e0), middle) {
                walk_to(w, id, ctx, middle);
            } else {
                w.mobys[id].set_state(0x10, 2);
            }
        }
        0x10 => {
            if w.mobys[id].first() { set_height(w, id, H_LAND); }
            let x = w.mobys[id].pv_f(0x5b8);
            let relaunch = |w: &mut MobyWorld| {
                let m = &mut w.mobys[id];
                let mut t = m.pv_i(0x608);
                countdown(&mut t);
                m.set_pv_i(0x608, t);
                if w.level23.drones_out <= OUT && t == 0 {
                    let m = &mut w.mobys[id];
                    m.set_state(12, 2);
                    m.set_pv_i(0x598, 0);
                    m.set_pv_i(0x590, 4);
                }
            };
            if RING_REACH + RING_REACH * 0.62 < x {
                ring_off(w, id);
                if w.mobys[id].pv_i(0x5b0) == 0 { relaunch(w); }
            } else if !w.level23.ring {
                let core = part(w, id, 0x610).map_or(0.0, |c| w.mobys[c].pv_f(0x20));
                if core < w.level23.t120f_health || x < RING_REACH * 0.5 {
                    ring(w, id, ctx);
                    w.level23.ring = true;
                } else {
                    relaunch(w);
                }
            } else {
                ring(w, id, ctx);
            }
        }
        20 => {
            ring_off(w, id);
            if ai::flight(w, id, ctx) & 1 != 0 {
                w.mobys[id].set_state(4, 0);
                put_gun_away(ctx);
                w.level23.hero_guard = 0;
                return false;
            }
        }
        _ => {}
    }
    true
}

/// State 9's burst: four walkers from its joints 30..33 at once (state 2), 16 down each joint's row 2 and 1.8 out.
fn burst_launch(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.host.play_sound(&w.mobys[id], 0, 0);
    let mut next = w.level23.next_drone.rem_euclid(25) as usize;
    for i in 0..4 {
        let Some(&d) = w.level23.drones.get(next) else { break };
        w.mobys[d].set_state(2, 0);
        next = (next + 1) % 25;
        let (Some(a), Some((jr, _))) = (ctx.host.joint(&w.mobys[id], i + 30), ctx.host.joint_matrix(&w.mobys[id], i + 30)) else { continue };
        let r = w.level23.radius;
        let own = w.mobys[id].pv_v(0x3e0);
        let off = add(scale(jr[2], -16.0), scale(sub(paths::surface(ctx, r, a, 0.0), own), 1.8));
        let p = paths::surface(ctx, r, add(own, off), 0.0);
        let rows = w.mobys[id].rows;
        let dm = &mut w.mobys[d];
        dm.rows = rows;
        dm.pos = p;
    }
}

/// The tail after the states (module docs).
fn after(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].set_pv_i(0x5a8, 0);
    for s in spares(w, id).into_iter().flatten() {
        if w.mobys[s].bangles == 0 { w.mobys[s].bangles = 0x3ff; }
    }
    pickups::tick(w, id, ctx);
    let m = &mut w.mobys[id];
    let mut a = m.pv_i(0x348);
    countdown(&mut a);
    m.set_pv_i(0x348, a);
    let mut b = m.pv_s(0x34c) as i32;
    countdown(&mut b);
    m.set_pv_s(0x34c, b as i16);
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = (m.state_ticks as u16 + 1) as i16; }
    if m.pv_i(0x5ac) <= 0xfffe { m.set_pv_i(0x5ac, m.pv_i(0x5ac) + 1); }
    for (i, g) in generators(w, id).into_iter().enumerate() {
        let Some(g) = g else { continue };
        super::super::enemies::place_part(w, id, g, i + 1, ctx);
        let pm = &mut w.mobys[g];
        pm.mode = (pm.mode & 0xfff9) | 0x4000;
    }
    match part(w, id, 0x610) {
        Some(c) => {
            super::super::enemies::place_part(w, id, c, 0x22, ctx);
            let open = w.mobys[id].pv_i(0x60c) == -1 && matches!(w.mobys[id].state, 0x10 | 0xc);
            let pm = &mut w.mobys[c];
            pm.mode = (pm.mode & 0xfff9) | 0x800;
            if open { pm.mode |= 0x4000; } else { pm.mode &= 0xbfff; }
        }
        None => w.mobys[id].mode |= 0x4000,
    }
    if let Some(c) = part(w, id, 0x614) {
        super::super::enemies::place_part(w, id, c, 0x22, ctx);
        let pm = &mut w.mobys[c];
        pm.mode = (pm.mode & 0xbff9) | 0x100;
    }
    hatch(w, id);
    let chest = w.level23.chest;
    aim(w, id, ctx, chest);
    fire(w, id, ctx, chest);
    shock(w, id, ctx);
    shadow(w, id);
    if w.mobys[id].pv_i(0x5d8) == 0 {
        let m = &mut w.mobys[id];
        let (mut h, mut v) = (m.pv_f(0x588), m.pv_f(0x58c));
        glide(&mut h, &mut v, m.pv_f(0x5d4), 0.016_666_668, 0.016_666_668, f32::from_bits(0x3ed1_1112));
        m.set_pv_f(0x588, h);
        m.set_pv_f(0x58c, v);
        m.set_pv_f(0x74, m.pv_f(0x74) * 2.0);
        if (h - m.pv_f(0x5d4)).abs() < 0.1 {
            m.set_pv_i(0x5d8, 1);
            m.set_pv_i(0x58c, 0);
        }
    }
    let m = &mut w.mobys[id];
    if w.level23.moved {
        if m.pv_i(0x5dc) == 0 {
            let (mut s, mut v) = (m.pv_f(0x5c8), m.pv_f(0x5cc));
            glide(&mut s, &mut v, m.pv_f(0x5d0), 35.0 / 3600.0, 35.0 / 3600.0, 28.5 / 60.0);
            m.set_pv_f(0x5c8, s);
            m.set_pv_f(0x5cc, v);
            if (s - m.pv_f(0x5d0)).abs() < 0.1 {
                m.set_pv_i(0x5dc, 1);
                m.set_pv_i(0x5cc, 0);
            }
        }
    } else {
        m.set_pv_i(0x5cc, 0);
        m.set_pv_i(0x5c8, 0);
        m.set_pv_i(0x5dc, 0);
    }
    let st = w.mobys[id].state;
    if let Some(s) = ctx.hero.giant.special.filter(|_| w.level23.ring && matches!(st, 0x10 | 0xc) && RING_REACH < w.mobys[id].pv_f(0x5bc)) {
        if let Some(sm) = w.mobys.get(s) {
            let at = sm.pos;
            let h = ctx.hero.gravity.height(at) - w.level23.radius;
            if dist(at, w.level23.middle) < RING_REACH * 0.6 && h < H_LAND - 2.0 {
                burst(w, ctx, at, &Blast { size: 2.0, bursts: [15, 10, 15], puffs: 15, streaks: 15, light: true, shake: false });
                special::end(w, ctx, true, true);
            }
        }
    }
}

/// State 4, the death (module docs: `0x417888`'s state 4 and `0x41e898`).
fn dying(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    parts_solid(w, id, false);
    ctx.hero.land_lag = 10;
    let t = w.mobys[id].state_ticks as u16 as i32;
    if w.mobys[id].first() {
        // `0x41ee30`: the ring off and the effects and the cap gone.
        ring_off(w, id);
        for off in [0x620, 0x624, 0x614] {
            if let Some(p) = part(w, id, off) { w.delete(p); }
            ai::set_link(&mut w.mobys[id], off, None);
        }
        ctx.game.blocking_fade(60);
    } else if 30 < t {
        death(w, id, ctx);
        if w.level23.boss.is_none() { return; }
    } else if t == 26 {
        ctx.game.blocking_fade(1);
        let (cam, rows) = (ctx.host.camera_pos(), ctx.host.camera_rows());
        let euler = crate::moby::attach::angles_of(&rows);
        ctx.game.cameras.camera_script(cam, euler, crate::script_camera::MODE_SNAP, 0, false, ctx.hero);
    } else if 26 < t {
        ctx.game.blocking_fade(1);
        let m = &w.mobys[id];
        let r = w.level23.radius;
        let euler = crate::moby::attach::angles_of(&m.rows);
        let eye = paths::surface(ctx, r, add(scale(m.rows[0], -55.0), m.pos), 25.0);
        ctx.game.cameras.camera_script_targets(Some(eye), Some(euler));
    } else {
        ctx.game.blocking_fade(1);
    }
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = (m.state_ticks as u16 + 1) as i16; }
}

/// `0x41e898` (module docs: the blow-up): blasts from the generators every 32 ticks, every 16 past 140; past 280 the
/// big blast (the class sound 11), the spares put in the generators' places and shown, the generators gone; then ten
/// ticks of their pieces flying, the spares gone, it hidden (a blast, the class sound 12). Past 480 ticks the scene
/// trigger 0x0f6e set off (its scene flag `[0xd]`, its list 0, hooked); past 620 with no scene under way its bolts
/// and the end ([`end`]).
fn death(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let t = w.mobys[id].state_ticks as u16 as i32;
    const LONG: i32 = 420;
    if LONG + 200 < t && !ctx.game.scenes.in_scene() {
        crate::moby::bolts::drop(w, id, 0, 3, ctx);
        end(w, id, ctx);
        return;
    }
    if LONG + 60 < t {
        if let Some(tr) = w.level23.trigger.take().filter(|&k| k < w.mobys.len()) {
            let m = &mut w.mobys[tr];
            m.set_pv_b(0xd, 1);
            m.set_pv_b(8, 0);
            m.once |= 4;
        }
        return;
    }
    if w.level23.breaking {
        let n = w.level23.break_n;
        for i in 5..10 {
            let Some(c) = part(w, id, 0x358 + 4 * i) else { continue };
            if n == 0 {
                w.delete(c);
                ai::set_link(&mut w.mobys[id], 0x358 + 4 * i, None);
                w.mobys[id].mode |= 0x41;
                let p = w.mobys[id].pos;
                burst(w, ctx, p, &Blast { size: 6.5, bursts: [135, 120, 120], puffs: 115, streaks: 125, light: true, shake: true });
                ctx.host.play_sound(&w.mobys[id], 12, 0);
            } else if let Some((r, _)) = ctx.host.joint_matrix(&w.mobys[id], i - 4) {
                let k = ctx.game.rng.randf(0.0, 0.35);
                let v = add(scale(r[1], 0.7), scale(w.mobys[id].rows[2], k));
                let (cl, pos, rot) = (w.mobys[c].o_class, w.mobys[c].pos, w.mobys[c].rot);
                crate::moby::shards::spawn_ball(w, ctx, cl, Some(c), pos, rot, 140.0, 25.0, (10 - n) as u8, v, 0xff);
            }
        }
        w.level23.break_n -= 1;
        return;
    }
    if LONG * 2 / 3 < t {
        let p = w.mobys[id].pos;
        burst(w, ctx, p, &Blast { size: 4.5, bursts: [45, 30, 40], puffs: 25, streaks: 85, light: true, shake: true });
        ctx.host.play_sound(&w.mobys[id], 11, 0);
        for i in 0..5 {
            let (Some(s), Some(g)) = (part(w, id, 0x36c + 4 * i), part(w, id, 0x358 + 4 * i)) else { continue };
            let (rows, pos) = (w.mobys[g].rows, w.mobys[g].pos);
            let sm = &mut w.mobys[s];
            (sm.rows, sm.pos) = (rows, pos);
            sm.mode = ((sm.mode & 0xfff9) | 0x4000) & 0xffbe;
            w.delete(g);
            ai::set_link(&mut w.mobys[id], 0x358 + 4 * i, None);
        }
        w.level23.breaking = true;
        w.level23.break_n = 10;
        return;
    }
    let every = if LONG / 3 < t { 15 } else { 31 };
    if t & every != 0 { return; }
    let k = ctx.game.rng.randi(5) as usize;
    let s = ctx.game.rng.randf(4.0, 12.0);
    if let Some(g) = part(w, id, 0x358 + 4 * k) {
        let p = add(scale(w.mobys[g].rows[1], s), w.mobys[g].pos);
        let size = ctx.game.rng.randf(2.3, 3.7);
        burst(w, ctx, p, &Blast { size, bursts: [20, 10, 14], puffs: 15, streaks: 25, light: false, shake: k & 1 != 0 });
    }
    let n = ctx.game.rng.randi(3) + 13;
    ctx.host.play_sound(&w.mobys[id], n, 0);
}

/// `0x41eeb0`: its parts and itself gone, the fight over (`$gp+0x23e4` 1), the game's flags `0x1395b8 + 0xa1` and
/// `+ 0x88` set.
fn end(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for i in 0..5 {
        if let Some(p) = part(w, id, 0x358 + 4 * i) { w.delete(p); }
    }
    if let Some(s) = part(w, id, 0x40c) { w.delete(s); }
    w.delete(id);
    w.level23.boss = None;
    w.level23.intro = 1;
    for k in [0xa1, 0x88] {
        if let Some(f) = ctx.game.flags.get_mut(k) { *f = 1; }
    }
}
