//! Damosel orbit's shots (L23 addresses): Giant Clank's gun 0x118f, the opponent's turret shot 0x118e and missile
//! 0x1230, its walkers' missile 0x1237, and the opponent's shock wave 0x11df.
//!
//! * **0x118f, Giant Clank's shot** (made [`fire`], `0x423f30(owner, at, target, mode, player)`; its update
//!   [`update_118f`], `0x424308`; the decomp's `func_L23_00422A30` / `func_L23_00422E08`; this level's own copy of Dobbo
//!   orbit's 0x0100, crate::moby::classes::level22::shot): update and draw distance 0xff, the owner its parent with
//!   its look, its scale × 3 × 2, mode | 0x100 with the owner's rows; the turn speed `+0x1c` ⅓, the timer `+0x14` 150.
//!   Its target: the one given, else the opponent (`$gp+0x2444`; its point the fight's aim word `0x1b3660`), else none
//!   (mode and player 0). Giant Clank's shots are a player's. Without a mode it tests the target: the way to its point
//!   (`+0x30`, unit) and the way on the surface; no target when the owner is Giant Clank and the target is not drawn,
//!   that way is more than acos 0.72 off its first row, the target more than 50 off, within 5 but more than 10 apart
//!   in height, or the opponent's guard up (`0x41ce48`). Giant Clank's in his jump (0x5b) without a target fly
//!   straight down his row 2. The class sound 1. **Each tick**: mode &= ~0x41; the turn speed eased to 7/3 by 1/18;
//!   its rows turned toward the up (`row 2 × down`·0.035) and its way the first row at the turn speed; with a mode it
//!   homes (`0x32b6c8`: 0.012, 0.012, near 20) on the target's point 5.5 up its row 2; a player's in its first 5 ticks
//!   while the point is ahead of it (near 0.1); else along `+0x30`. Then it moves; gone below 0 on an axis. The hit:
//!   the line it moved (flags 0x10, without its owner), a template of `0x1b3680` (the opponent's `+0x310`), flags
//!   0x8b0003, the kind `class << 16 | 0x303`: a moby met (not its parent) or the world stops it there (`+0xbc` 2 /
//!   1; the class sound 2 on the opponent, a generator or the core, else 0); none, the timer counts down and it goes
//!   at 0. Stopped, the same hit to what is within 2 of it (without the owner), and it goes.
//! * **0x118e, the turret's shot** ([`turret_shot`], `0x423a40(at, way, &damage)`, the decomp's `func_L23_00422540`):
//!   the drone shot of Dobbo orbit (its update `0x423980` the same code, crate::moby::classes::level22::drone's
//!   `update_0f5d`), 60 ticks, its rows along its way. **Its hit** ([`turret_hit`], `0x423b68`, `func_L23_00422668`):
//!   the line it moved meeting a walker 0x07e5 goes on; any other moby: Giant Clank alone takes the damage (flags
//!   0x8b0003, the kind `class << 16 | 0x305`), a small blast; the world: a small blast; then it goes.
//! * **0x1230, the opponent's missile** ([`missile`], `0x4295d8(owner, way, at, 0, damage)`, the decomp's
//!   `func_L23_004280D8`; its update [`update_1230`], `0x4291e8`, `func_L23_00427CE8`): along its way, aimed by its
//!   angles, mode | 0x308, alpha 0x80. Each tick it counts (past 240 → 1); in 0 the line it moves: met within 4 of
//!   Giant Clank's chest, or him, a blast at him (3.5 up), his hit (0x10007, its damage) and on him the level sound
//!   0x20; met elsewhere it ends; → 1. **1**: its blast (4.5), gone. It rolls 5.17° a tick about its row 1.
//! * **0x1237, the walkers' missile** ([`drone_missile`], `0x42a598`, `func_L23_00429098`; its update
//!   [`update_1237`], `0x42a1f8`, `func_L23_00428CF8`): as 0x1230, but met (the line, or a sphere 0.3) by a moby not
//!   a walker, the opponent or a generator: a 0.6 sphere hit (its damage, flags 0x90003, its way × 1.5 with z 1, the
//!   kind `class << 16 | 0x100`; on Giant Clank the level sound 0x20) and → 1; by its own kind it flies on; the world:
//!   → 1. **1**: a blast (1.0), gone.
//! * **0x11df, the shock wave** ([`wave`], `0x427748`, `func_L23_00426248`; its update [`update_11df`], `0x427ad0`,
//!   `func_L23_004265D0`): one of five at most; at the point with its owner's rows, 60 ticks; in view a camera shake
//!   (0.075, 40 ticks) within 48 of the camera. Giant Clank's hurt 0x5a is from its class (crate::hero::giant).
//!
//! Not ported: the trails, glows and draws (`0x342750`-like trail draws of 0x118f: their random draws are made; the
//! missiles' `0x2c2210` trails and their draw callbacks `0x40d7f8` / `0x40e3e8`; the wave's rings `0x428518` and its
//! fade words `0x1b3868` / `0x1b3880`), 0x118f's burst at its stop (`0x32a128`).

use super::boss;
use crate::hero::physics::{add, add_rot, cross, dist, dot, scale, set_len, sub, unit, V3};
use crate::moby::blast::{burst, Blast};
use crate::moby::hits::HitTemplate;
use crate::moby::{countdown, Ctx, MobyWorld};
use crate::quat::{axis_angle, turn_rows};

pub const SHOT: i16 = 0x118f;
pub const TURRET: i16 = 0x118e;
pub const MISSILE: i16 = 0x1230;
pub const DRONE_MISSILE: i16 = 0x1237;
pub const WAVE: i16 = 0x11df;

/// `0x5627.9248`: the hit way's "no limit" w.
const NO_LIMIT: f32 = f32::from_bits(0x45af_df66);

fn down_at(ctx: &Ctx, p: V3) -> V3 { scale(ctx.hero.gravity.up_at(p), -1.0) }

/// The target word `+0x18`: none, a moby (index + 1), or the straight-down shot's 1 (here −1).
const DOWN: i32 = -1;
fn target_of(w: &MobyWorld, m: &crate::moby::Moby) -> Option<usize> { usize::try_from(m.pv_i(0x18) - 1).ok().filter(|&i| i < w.mobys.len()) }

/// The point a shot aims at (`+0x2c`): the fight's aim word for the opponent, else the target's place.
fn aim_point(w: &MobyWorld, m: &crate::moby::Moby) -> Option<V3> {
    let t = target_of(w, m)?;
    Some(if m.pv_i(0x2c) == 2 { w.level23.aim } else { w.mobys[t].pos })
}

/// `0x423f30` (module docs).
pub fn fire(w: &mut MobyWorld, ctx: &mut Ctx, owner: usize, at: V3, target: Option<usize>, mode: bool, player: bool) -> usize {
    let id = w.create(SHOT);
    let (opos, orot, olight, orows) = { let o = &w.mobys[owner]; (o.pos, o.rot, o.light, o.rows) };
    let hero = ctx.hero_moby == Some(owner);
    let boss = w.level23.boss.filter(|&b| w.mobys.get(b).is_some_and(|m| m.state < 0xfd));
    let m = &mut w.mobys[id];
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    (m.update_dist, m.draw_dist, m.drawn, m.parent) = (0xff, 0xff, 1, Some(owner));
    (m.pos, m.rot, m.light) = (opos, orot, olight);
    m.mode |= 0x100;
    m.scale *= 3.0;
    m.set_pv_f(0x1c, f32::from_bits(0x3eaa_aaab));
    m.rows = orows;
    m.set_pv_i(0x10, owner as i32 + 1);
    m.pos = at;
    m.set_pv_i(0x14, 150);
    m.scale += m.scale;
    m.set_pv_b(0x28, mode as u8);
    m.set_pv_b(0x29, player as u8);
    match (target, boss) {
        (Some(t), _) => m.set_pv_i(0x18, t as i32 + 1),
        (None, Some(b)) => {
            m.set_pv_i(0x18, b as i32 + 1);
            m.set_pv_i(0x2c, 2);
        }
        (None, None) => {
            m.set_pv_i(0x18, 0);
            m.set_pv_b(0x29, 0);
            m.set_pv_b(0x28, 0);
        }
    }
    if hero { m.set_pv_b(0x29, 1); }
    if m.pv_b(0x28) == 0 && m.pv_i(0x18) != 0 {
        let r = w.level23.radius;
        let t = target_of(w, &w.mobys[id]).unwrap_or(owner);
        let tp = aim_point(w, &w.mobys[id]).unwrap_or(w.mobys[t].pos);
        let tdrawn = w.mobys[t].drawn;
        let on = super::paths::surface(ctx, r, tp, 0.0);
        let start = super::paths::surface(ctx, r, at, 0.0);
        let way = set_len(sub(on, start), 1.0);
        let (oh, th) = (ctx.hero.gravity.height(opos), ctx.hero.gravity.height(tp));
        let d = dist(on, start);
        let reject = d < 5.0 && 10.0 < (oh - th).abs();
        let guard = super::targets::guarded(w, super::targets::opponent(w));
        let m = &mut w.mobys[id];
        m.set_pv_v(0x30, set_len(sub(tp, m.pos), 1.0));
        if (hero && tdrawn == 0) || dot(way, m.rows[0]) < f32::from_bits(0x3f38_51ec) || 50.0 < d || reject || guard { m.set_pv_i(0x18, 0); }
        if hero && m.pv_i(0x18) == 0 && ctx.hero.state == 0x5b {
            m.set_pv_v(0x30, scale(orows[2], -1.0));
            m.set_pv_b(0x28, 0);
            m.set_pv_i(0x18, DOWN);
            m.set_pv_b(0x29, 0);
        }
    }
    let m = &mut w.mobys[id];
    m.set_pv_f(0x20, 0.0);
    ctx.host.play_sound(&w.mobys[id], 1, 0);
    let m = &mut w.mobys[id];
    m.timer = 0;
    m.mode |= 0x41;
    id
}

/// 0x118f (`0x424308`, module docs).
pub fn update_118f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let down = down_at(ctx, w.mobys[id].pos);
    let target = target_of(w, &w.mobys[id]);
    let homing = target.map(|t| add(w.mobys[t].pos, scale(w.mobys[t].rows[2], 5.5)));
    let point = aim_point(w, &w.mobys[id]);
    let m = &mut w.mobys[id];
    m.mode &= 0xffbe;
    let mut turn = m.pv_f(0x1c);
    crate::hero::physics::approach(f32::from_bits(0x4015_5556), f32::from_bits(0x3d63_8e39), &mut turn);
    m.set_pv_f(0x1c, turn);
    let mut vel = set_len(m.rows[0], turn);
    let a = scale(cross(m.rows[2], unit(down)), f32::from_bits(0x3d8f_5c29) * 0.5);
    m.rows = turn_rows(&m.rows, [a[0], a[1], a[2], -(1.0 - dot(a, a)).sqrt()]);
    let home = |m: &mut crate::moby::Moby, aim: V3, near: f32| {
        let mut t = m.pv_f(0x20);
        let k = f32::from_bits(0x3c44_9ba6);
        m.rows[0] = crate::moby::classes::turret::home(k, k, 0.0, near, m.pos, m.rows[0], aim, &mut t, down);
        m.set_pv_f(0x20, t);
        m.rows[0] = set_len(m.rows[0], 1.0);
        set_len(m.rows[0], m.pv_f(0x1c))
    };
    if m.pv_i(0x18) != 0 {
        if m.pv_b(0x28) != 0 {
            if let Some(aim) = homing { vel = home(m, aim, 20.0); }
        } else if m.pv_b(0x29) != 0 {
            if let Some(p) = point.filter(|_| m.pv_i(0x14) < 0x91) {
                if dot(sub(m.pos, p), vel) < 0.0 { vel = home(m, p, 0.1); }
            }
        } else {
            vel = scale(m.pv_v(0x30), turn);
            m.rows[0] = set_len(vel, 1.0);
        }
    }
    // The trail's draws (its draw itself not ported).
    ctx.game.rng.randf(0.1, 0.3);
    ctx.game.rng.randi(255);
    ctx.game.rng.rand_vec(0.0, 0.1);
    let m = &mut w.mobys[id];
    let from = m.pos;
    m.pos = add(m.pos, vel);
    if m.pos.iter().any(|&c| c < 0.0) {
        w.delete(id);
        return;
    }
    let owner = usize::try_from(m.pv_i(0x10) - 1).ok();
    let v = set_len(vel, 1.0);
    let mut t = HitTemplate::new(w.level23.shot_damage, Some(id), 0x8b_0003, [v[0], v[1], v[2], NO_LIMIT]);
    t.kind = (SHOT as u16 as u32) << 16 | 0x303;
    if w.mobys[id].timer == 0 {
        if let Some(hit) = w.coll_line(ctx, from, w.mobys[id].pos, 0x10, owner) {
            let parent = w.mobys[id].parent;
            match hit.moby {
                Some(o) => {
                    w.deliver_hit_any(o, &t, [hit.point[0], hit.point[1], hit.point[2], 1.0]);
                    if Some(o) != parent {
                        w.mobys[id].timer = 2;
                        w.mobys[id].pos = hit.point;
                    }
                }
                None if 0 < hit.kind => {
                    w.mobys[id].timer = 1;
                    w.mobys[id].pos = hit.point;
                }
                None => {}
            }
            let fight = hit.moby.is_some_and(|o| matches!(w.mobys[o].o_class, super::targets::F7E | boss::CLASS | super::targets::T120F));
            ctx.host.play_sound(&w.mobys[id], if fight { 2 } else { 0 }, 0);
        } else {
            let mut timer = w.mobys[id].pv_i(0x14);
            let out = countdown(&mut timer);
            w.mobys[id].set_pv_i(0x14, timer);
            if out {
                w.delete(id);
                return;
            }
        }
    }
    if w.mobys[id].timer == 0 { return; }
    let p = w.mobys[id].pos;
    w.sphere_hit(ctx.scene, p, 2.0, 0, owner, Some(&t));
    w.delete(id);
}

/// `0x423a40(at, way, &damage)` (module docs): the turret's shot 0x118e.
pub fn turret_shot(w: &mut MobyWorld, ctx: &Ctx, pos: V3, vel: V3, damage: f32) {
    let up = ctx.hero.gravity.up_at(pos);
    let id = w.create(TURRET);
    let m = &mut w.mobys[id];
    m.pvars.resize(m.pvars.len().max(0x40), 0);
    m.set_pv_f(0x1c, damage);
    m.set_pv_i(0x10, 0x3c);
    (m.update_dist, m.draw_dist, m.state) = (0xff, 0xff, 0);
    m.pos = pos;
    m.set_pv_v(0, vel);
    m.set_pv_i(0x18, 0);
    m.set_pv_i(0x14, 0);
    let r0 = unit(vel);
    let r1 = unit(cross(r0, up));
    let r2 = cross(r1, r0);
    m.rows = [r0, r1, r2];
}

/// `0x423b68(m, from)` (module docs), from the shared drone shot's update.
pub fn turret_hit(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, from: V3) {
    let to = w.mobys[id].pos;
    let Some(h) = w.coll_line(ctx, from, to, 0, Some(id)) else { return };
    let small = |w: &mut MobyWorld, ctx: &mut Ctx, at: V3| {
        let s = ctx.game.rng.randf(0.8, 1.2);
        burst(w, ctx, at, &Blast { size: s, bursts: [5, 4, 4], puffs: 4, streaks: 4, light: false, shake: false });
    };
    match h.moby {
        Some(o) => {
            if w.mobys[o].o_class == super::drones::CLASS { return; }
            let v = unit(w.mobys[id].pv_v(0));
            let mut t = HitTemplate::new(w.mobys[id].pv_f(0x1c), Some(id), 0x8b_0003, [v[0], v[1], v[2], NO_LIMIT]);
            t.kind = (TURRET as u16 as u32) << 16 | 0x305;
            if Some(o) == ctx.hero_moby { w.deliver_hit_any(o, &t, [h.point[0], h.point[1], h.point[2], 1.0]); }
            small(w, ctx, to);
        }
        None => small(w, ctx, to),
    }
    w.mobys[id].state = 2;
}

/// The missiles' make (`0x4295d8` / `0x42a598(owner, way, at, 0, damage)`): along `way` from `at`, its angles its way's.
fn make_missile(w: &mut MobyWorld, ctx: &mut Ctx, class: i16, owner: usize, vel: V3, at: V3, damage: f32) -> usize {
    let id = w.create(class);
    let roll = ctx.game.rng.randf(-std::f32::consts::PI, std::f32::consts::PI);
    let m = &mut w.mobys[id];
    m.pvars.resize(m.pvars.len().max(0x40), 0);
    (m.update_dist, m.draw_dist, m.drawn, m.timer, m.state) = (0xff, 0xff, 1, 0, 0);
    m.pos = at;
    m.set_pv_v(0, vel);
    m.set_pv_i(0x10, owner as i32 + 1);
    m.set_pv_f(0x14, damage);
    m.set_pv_f(0x20, 0.15);
    m.mode |= 0x208;
    let flat = (vel[0] * vel[0] + vel[1] * vel[1]).sqrt();
    m.rot = [roll, -crate::hero::physics::atan2(flat, vel[2]), add_rot(crate::hero::physics::atan2(vel[1], vel[0]), std::f32::consts::FRAC_PI_2)];
    m.rows = crate::moby::attach::rows_of(m.rot);
    m.alpha = 0x80;
    m.scale = 0.0001;
    m.mode |= 0x100;
    m.set_pv_i(0x24, 0);
    m.set_pv_i(0x2c, 0);
    id
}

/// `0x4295d8(owner, way, at, 0, damage)` (module docs).
pub fn missile(w: &mut MobyWorld, ctx: &mut Ctx, owner: usize, vel: V3, at: V3, damage: f32) -> usize { make_missile(w, ctx, MISSILE, owner, vel, at, damage) }

/// `0x42a598(owner, way, at, 0, damage)` (module docs).
pub fn drone_missile(w: &mut MobyWorld, ctx: &mut Ctx, owner: usize, vel: V3, at: V3, damage: f32) -> usize { make_missile(w, ctx, DRONE_MISSILE, owner, vel, at, damage) }

/// The missiles' roll: 5.17° a tick about their row 1.
fn roll(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let q = axis_angle(f32::from_bits(0x40a5_70a4) * 0.017_453_294, unit(m.rows[1]));
    m.rows = turn_rows(&m.rows, q);
}

/// The missiles' tick count: past 240 → 1.
fn count(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_i(0x24, m.pv_i(0x24) + 1);
    if 0xf0 < m.pv_i(0x24) { m.set_state(1, -1); }
}

/// 0x1230 (`0x4291e8`, module docs).
pub fn update_1230(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let n = w.mobys[id].pvars.len().max(0x40);
    w.mobys[id].pvars.resize(n, 0);
    if w.mobys[id].timer == 0 {
        count(w, id);
        match w.mobys[id].state {
            0 => {
                let m = &w.mobys[id];
                let to = add(m.pos, m.pv_v(0));
                if let Some(h) = w.coll_line(ctx, m.pos, to, 0, Some(id)) {
                    let him = h.moby.is_some() && h.moby == ctx.hero_moby;
                    if him || dist(w.level23.chest, h.point) < 4.0 {
                        let up = ctx.hero_moby.and_then(|k| w.mobys.get(k)).map_or([0.0, 0.0, 1.0], |k| k.rows[2]);
                        let b = add(ctx.hero.pos, scale(up, 3.5));
                        burst(w, ctx, b, &Blast { size: 1.5, bursts: [5, 5, 5], puffs: 0x19, streaks: 0x4b, light: false, shake: true });
                        let (pos, d) = (w.mobys[id].pos, w.mobys[id].pv_f(0x14));
                        boss::hit_hero(w, ctx, Some(id), 0x1_0007, d.trunc(), pos, None);
                        if him { ctx.host.play_level_sound_after(None, 0x20, 0); }
                    }
                    w.mobys[id].set_state(1, -1);
                    return;
                }
                w.mobys[id].pos = to;
            }
            1 => {
                let p = w.mobys[id].pos;
                burst(w, ctx, p, &Blast { size: 4.5, bursts: [0x19, 0x19, 0x19], puffs: 0x19, streaks: 0, light: false, shake: false });
                w.delete(id);
                return;
            }
            _ => {}
        }
    }
    roll(w, id);
}

/// 0x1237 (`0x42a1f8`, module docs).
pub fn update_1237(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let n = w.mobys[id].pvars.len().max(0x40);
    w.mobys[id].pvars.resize(n, 0);
    if w.mobys[id].timer == 0 {
        count(w, id);
        match w.mobys[id].state {
            0 => {
                let m = &w.mobys[id];
                let (pos, vel) = (m.pos, m.pv_v(0));
                let to = add(pos, vel);
                let line = w.coll_line(ctx, pos, to, 0, Some(id));
                let met = match &line {
                    Some(h) => Some(h.moby),
                    None => ctx.scene.and_then(|sc| crate::collision::mobys::coll_sphere_mobys(sc, pos, 0.3, crate::collision::QueryFlags(0), Some(id)).first().copied()).map(Some),
                };
                if let Some(o) = met {
                    match o {
                        Some(o) if !matches!(w.mobys[o].o_class, super::drones::CLASS | boss::CLASS | super::targets::F7E) => {
                            let v = scale(vel, 1.5);
                            let mut t = HitTemplate::new(w.mobys[id].pv_f(0x14), Some(id), 0x9_0003, [v[0], v[1], 1.0, NO_LIMIT]);
                            t.kind = (DRONE_MISSILE as u16 as u32) << 16 | 0x100;
                            if Some(o) == ctx.hero_moby { ctx.host.play_level_sound_after(None, 0x20, 0); }
                            w.sphere_hit(ctx.scene, pos, 0.6, 0, Some(id), Some(&t));
                            w.mobys[id].set_state(1, -1);
                            return;
                        }
                        Some(_) => {}
                        None => {
                            w.mobys[id].set_state(1, -1);
                            return;
                        }
                    }
                }
                w.mobys[id].pos = to;
            }
            1 => {
                let p = w.mobys[id].pos;
                burst(w, ctx, p, &Blast { size: 1.0, bursts: [25, 25, 25], puffs: 25, streaks: 0, light: false, shake: false });
                w.delete(id);
                return;
            }
            _ => {}
        }
    }
    roll(w, id);
}

/// `0x427748(1, at, owner, 1, …)` (module docs): the wave, or none with five out.
pub fn wave(w: &mut MobyWorld, ctx: &mut Ctx, at: V3, owner: usize) -> Option<usize> {
    let g = &mut w.level23;
    if 5 <= g.waves_n { return None; }
    let k = g.waves.iter().position(|&b| !b).unwrap_or(4);
    g.waves_n += 1;
    g.waves[k] = true;
    let rows = w.mobys[owner].rows;
    let id = w.create(WAVE);
    let m = &mut w.mobys[id];
    m.pvars.resize(m.pvars.len().max(0x50), 0);
    (m.update_dist, m.draw_dist, m.state) = (0xff, 0xff, 0);
    m.set_pv_i(0x3c, k as i32);
    m.pos = at;
    m.rows = rows;
    m.set_pv_f(0x14, 1.0);
    m.set_pv_s(0, 0x3c);
    m.set_pv_s(2, 0x3c);
    m.set_pv_f(8, 2.0);
    m.set_pv_f(0xc, 4.0);
    let a = ctx.game.rng.randf_signed(65.0, 80.0);
    m.set_pv_f(0x44, a);
    let seen = !ctx.view.is_some_and(|v| v.culled(128.0, [at[0], at[1], at[2], 3.0]));
    if seen {
        m.set_pv_s(4, 0x19);
        m.set_pv_f(0x10, 0.04);
        if dist(at, ctx.host.camera_pos()) < 48.0 { ctx.game.shakes.ask(0, 0.075, 0x28); }
    } else {
        m.set_pv_s(4, 0);
    }
    Some(id)
}

/// 0x11df (`0x427ad0`, module docs): its 60 ticks, then its slot freed and gone.
pub fn update_11df(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.pvars.resize(m.pvars.len().max(0x50), 0);
    if m.state == 0 { m.state = 1; }
    let mut t = m.pv_s(0) as i32;
    if !countdown(&mut t) {
        m.set_pv_s(0, t as i16);
        let mut k = m.pv_s(4) as i32;
        countdown(&mut k);
        m.set_pv_s(4, k as i16);
        return;
    }
    let k = m.pv_i(0x3c).clamp(0, 4) as usize;
    let g = &mut w.level23;
    g.waves[k] = false;
    g.waves_n -= 1;
    w.delete(id);
}
