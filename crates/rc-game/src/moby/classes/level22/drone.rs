//! Dobbo orbit's drones **0x0edd** (L22 `0x40dd88`, the decomp's `func_US2_L22_0040DD88`; its set-up and tail
//! `0x40eb38` / `0x40ef80`, the decomp's `func_L22_0040D638` / `func_L22_0040DA80`; 25 placed) and their shots
//! **0x0f5d** (made `0x40f300`, update `0x40f240` — the decomp's `func_L22_0040DE00` / `func_L22_0040DD40`, the
//! update shared with L23's 0x118e, whose hit `0x423b68` differs and is not ported here): small flyers that keep to
//! 7.2 above the planetoid, chase Giant Clank and shoot at him, and come back elsewhere when they die. They walk with
//! the opponent's tests (crate::moby::classes::level22::planet: the walk test from Giant Clank, the paths' nearest
//! point, the pickups' point draw `0x3a4980`).
//!
//! **pvars**: health `+0x20`, mover `+0x50` (radius 7, height −7, steps 0.4, slope π/2, options | 5, stop 0.5, the
//! turn left 89°), target `+0x1f0`, reaction `+0x140` (hurt state 5 sub 3), `+0x210` its spinner's record, `+0x220`
//! its turn, `+0x258` the chase's timer, `+0x25c`, `+0x26c` the spinner's angle, `+0x280` / `+0x284` / `+0x288` /
//! `+0x28c` words, `+0x290` the down, `+0x2a0` its goal, `+0x2b0` / `+0x2b4` the orbit's row (Giant Clank's row 0)
//! and its distance.
//!
//! **The set-up** ([`pre`], `0x40eb38`, state 0): draw distance 80, update 255, mode | 0x100; its blocks' words; the
//! walk limits (0.0067, 0.2, 0.0017, 0.21); hidden and no target; 1 (sub 4). **1** (with the opponent about): 6 (sub
//! 4), and one in two at once in play: shown, a target, at a point of the opponent's paths 7.2 up (`0x3a4980`),
//! pushed 3.7 along a random row (its sign a second draw) off another drone within 3, its goal there, 2 (sub 0),
//! sequence 1. **In the fight's phase 2** (`0x1b2a08`) with the opponent shown: within 55 of Giant Clank `+0x288` 2;
//! hidden (6) or past 55, up to 16 times (`0x1b2a0c`) with the comeback timer out (`0x1b2a10`): shown, a target, at
//! the planetoid's centre, 8 (sub 5), the timer 20. Each tick out of 6: its scale ×0.4, its damage (reaction
//! `+0x140`), the down at it and its rows stood up on it.
//!
//! **Each tick** ([`update`]): out of 6 and sub 5: within 11 of the opponent it dies (5, sub 3); facing Giant Clank
//! (45°), his walk test to it (`visible`), his distance; over 9.2 up an ice block 0x0dce within 5 of its point 2 up
//! blows up (crate::moby::blast, 1.5) and goes. **Sub 0** (sequence 1): in the intro nothing; seen within 40 → 3 (sub
//! 1, the chase's timer 480..600 when under 300); drawn within 80 → to its goal (5 a second); else still. **1**: the
//! timer out → 2; seen and faced within 30 → 4 (sub 2), still; else to him (13). **2**: facing him (or first) its
//! sequence 3 at 0.35 (`0x1b2a20`), else back to 1 at the end; unseen or past 50 → 2 (sub 0), the timer 0; past 20 to
//! him, else facing him. **3** the knock-back (crate::moby::ai::flight): dead (or hit by the opponent): the level
//! sound 17; killed by Giant Clank with his health short, a health pickup at his feet
//! (crate::moby::classes::level22::pickups::drop_at); five pieces (balls, 15..24 a second, down ×0.5); a blast; its
//! health back to full; back at the paths' point nearest a point 80 below Giant Clank and 50 (or 35 aside) off him,
//! 7.2 up, pushed off another drone; 2 (sub 0). Over: 3 (sub 1), the timer 580..600.
//! **States**: **4** at frame 1.5 its shots at Giant Clank's centre (6 damage, `0x1b2a18`); **9** to its goal 7.2
//! up (8): shots at the goal every 64 ticks after 60 (50); there → 2 (sub 0), the option 0x10 off; **7** circling
//! him (every 8 ticks the goal his row 0 × the distance off him, 7.2 up; shots every 512 ticks after 60) to its goal
//! (13); within 2 of it or 20 of him → 9 (sub 5); **8** the comeback: the distance 10..25, a start 50..70 off him
//! along a random row and a random 0.1..16.7, 7 (sub 5), the option 0x10 on.
//!
//! **The tail** (`0x40ef80`): the timers `+0x258` / `+0x25c` counted down, its ticks; its spinner (joint list 7) turned
//! 0.416 a tick about z.
//!
//! **The shots** ([`shoot`], `0x40f020`, none with the opponent dead): from its joints 2 and 3 at ⅔ a tick toward the
//! point, `damage` (6 when 0): 0x0f5d ([`update_0f5d`]) 80 ticks; each tick its move; the line it moved (flags 0)
//! meeting a moby not a drone nor the opponent — Giant Clank only under 50 — takes the damage (50.1 at most; flags
//! 0x8b0003, the kind `class << 16 | 0x305`), a small blast (`0x40e2b0`: 0.8..1.2), and it goes.
//!
//! Not ported: the shots' trail draw (`0x40f598`), the spinner record's set-up words (`0x317a30`).

use super::{boss, pickups, planet};
use crate::hero::physics::{add, cross, dist, scale, set_len, sub, unit, V3};
use crate::moby::blast::{burst, Blast};
use crate::moby::frames::{self, upright};
use crate::moby::{ai, countdown, Ctx, MobyWorld};
use crate::quat::turn_rows;

pub const CLASS: i16 = 0x0edd;
pub const SHOT: i16 = 0x0f5d;
const ICE: i16 = 0x0dce;

/// `$gp+0x215c..+0x2184` (the decomp's `D_L22_001B119C..`): the comeback timer, the opponent's reach, the comeback's
/// distances, the speeds (sub 0, state 9, the chase), the pieces' speeds and their down share.
const COMEBACK: i32 = 20;
const OPP_REACH: f32 = 11.0;
const FAR: (f32, f32) = (50.0, 70.0);
const SPEED_0: f32 = 5.0;
const SPEED_9: f32 = 8.0;
const SPEED: f32 = 13.0;
const PIECES: (f32, f32, f32) = (15.0, 24.0, 0.5);
/// `$gp+0x2c8`: the push off another drone.
const PUSH: f32 = f32::from_bits(0x406c_cccd);
const HIGH: f32 = 7.2;

fn boss_of(w: &MobyWorld) -> Option<usize> { w.level22.boss.filter(|&b| w.mobys.get(b).is_some_and(|m| m.o_class == boss::CLASS && m.state < 0xfd)) }

fn seq(w: &MobyWorld, ctx: &Ctx, id: usize) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_b) }
fn ended(w: &MobyWorld, ctx: &Ctx, id: usize) -> bool { ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.flags & 2 != 0) }

/// `0x32a7f0`: the first time since the last sub-state change.
fn first_sub(w: &mut MobyWorld, id: usize) -> bool {
    let m = &mut w.mobys[id];
    if m.once & 2 != 0 { return false; }
    m.once |= 2;
    true
}

/// `0x2e5b30(p, flags, ignore, r)` (`0x2df8d8`): the moby a sphere query meets first.
fn moby_at(ctx: &Ctx, p: V3, flags: u32, ignore: Option<usize>, r: f32) -> Option<usize> {
    let sc = ctx.scene?;
    crate::collision::mobys::coll_sphere_mobys(sc, p, r, crate::collision::QueryFlags(flags), ignore).first().copied()
}

/// Off another drone within 3 (`0x2e5b30(p, 1, 0, 3)`): pushed 3.7 along a random row, back 7.2 up.
fn push_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let p = w.mobys[id].pos;
    let Some(o) = moby_at(ctx, p, 1, None, 3.0) else { return };
    if w.mobys[o].o_class != CLASS || w.mobys[o].state == 6 { return; }
    let axis = w.mobys[id].rows[ctx.game.rng.randi(2) as usize];
    let s = if ctx.game.rng.randi(2) != 0 { PUSH } else { -PUSH };
    let p = add(p, scale(axis, 2.0 * s));
    w.mobys[id].pos = planet::surface(ctx, w.level22.radius, p, HIGH);
}

/// `0x40f140(m, to, lift, speed)`: toward `to` at `speed`/60 a tick (with `lift` kept 7.2 up), facing it
/// (crate::moby::frames::face_point), the frames' body move (crate::moby::frames::move_by).
fn go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, to: V3, lift: bool, speed: f32) {
    let s = speed * 0.016_666_668;
    let pos = w.mobys[id].pos;
    let mut v = set_len(sub(to, pos), s);
    if lift {
        let p = planet::surface(ctx, w.level22.radius, add(pos, v), HIGH);
        v = set_len(sub(p, pos), s);
    }
    frames::face_point(w, id, to);
    frames::move_by(w, id, ctx, v);
}

/// `0x40f220`: still (the body move by nothing).
fn still(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { frames::move_by(w, id, ctx, [0.0; 3]); }

/// Giant Clank's centre (`0x189f00`, a unit over his feet).
fn centre(ctx: &Ctx) -> V3 { add(ctx.hero.pos, ctx.hero.gravity.up_at(ctx.hero.pos)) }

/// Giant Clank's rows (his moby's `+0xc0..`).
fn hero_rows(w: &MobyWorld, ctx: &Ctx) -> [V3; 3] { ctx.hero_moby.and_then(|h| w.mobys.get(h)).map_or(ctx.hero.rows, |m| m.rows) }

/// `0x40eb38` (module docs).
fn pre(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let st = w.mobys[id].state;
    let r = w.level22.radius;
    if st == 0 {
        let m = &mut w.mobys[id];
        (m.draw_dist, m.update_dist) = (80, 0xff);
        m.mode |= 0x100;
        m.set_pv_b(0x150, 5);
        m.set_pv_i(0x188, 3);
        m.set_pv_f(0x17c, 0.15);
        m.set_pv_f(0x170, 0.5);
        m.set_pv_f(0x174, 0.5);
        if m.pv_s(0x24) == 0 {
            m.set_pv_s(0x24, 1);
            m.set_pv_f(0x20, 1.0);
        }
        m.set_pv_i(0x88, m.pv_i(0x88) | 5);
        for (off, v) in [(0x84, 0.5), (0x54, -7.0), (0x50, 7.0), (0x58, 0.4), (0x5c, 0.4), (0x60, 1.570_796_4), (0x80, 1.553_343_2)] {
            m.set_pv_f(off, v);
        }
        ai::senses(m, 0.006_666_667, 0.200_000_02, 0.001_745_329_5, 0.209_439_55);
        m.set_pv_i(0x288, 0);
        m.set_pv_i(0x28c, 1);
        m.has_collision = false;
        m.mode = (m.mode | 0x41) & 0xefff;
        m.set_state(1, 4);
    } else if st == 1 {
        let Some(b) = boss_of(w) else { return tail_pre(w, id, ctx) };
        w.mobys[id].set_state(6, 4);
        if ctx.game.rng.randi(2) != 0 {
            let m = &mut w.mobys[id];
            m.set_pv_s(0x178, 0);
            m.mode = (m.mode & 0xffbe) | 0x1000;
            m.has_collision = true;
            let p = pickups::pick_point(w, ctx, b, HIGH);
            w.mobys[id].pos = p;
            push_off(w, id, ctx);
            let m = &mut w.mobys[id];
            let p = m.pos;
            m.set_pv_v(0x2a0, p);
            m.set_pv_i(0x284, 0);
            m.set_state(2, 0);
            if seq(w, ctx, id) != 1 { ctx.host.set_anim(&w.mobys[id], 1, 0, 0); }
        } else {
            w.mobys[id].set_pv_i(0x284, 1);
        }
    } else if w.level22.phase == 2 {
        if let Some(b) = boss_of(w).filter(|&b| w.mobys[b].mode & 1 == 0) {
            let _ = b;
            let d = dist(ctx.hero.pos, w.mobys[id].pos);
            if st != 6 && d < 55.0 { w.mobys[id].set_pv_i(0x288, 2); }
            if st == 6 || 55.0 < d {
                let g = &mut w.level22;
                if g.drones_back < 16 && g.timer == 0 {
                    g.drones_back += 1;
                    g.timer = COMEBACK;
                    let c = ctx.hero.gravity.centre;
                    let m = &mut w.mobys[id];
                    m.set_pv_s(0x178, 0);
                    m.mode = (m.mode & 0xffbe) | 0x1000;
                    m.has_collision = true;
                    m.pos = c;
                    m.set_state(8, 5);
                    m.set_pv_i(0x288, 2);
                }
            }
        }
    }
    let _ = r;
    tail_pre(w, id, ctx);
}

/// `0x40eb38`'s end out of 6: its scale, its damage, stood up on the sphere.
fn tail_pre(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 6 { return; }
    let sc = w.class(CLASS).map_or(1.0, |c| c.scale);
    w.mobys[id].scale = sc * 0.4;
    ai::take_damage(w, id, ctx, false);
    let down = planet::down_at(ctx, w.mobys[id].pos);
    let m = &mut w.mobys[id];
    m.set_pv_v(0x290, down);
    m.rows = turn_rows(&m.rows, upright(&m.rows, scale(down, -1.0)));
}

/// 0x0edd (`0x40dd88`, module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0x2c0 { w.mobys[id].pvars.resize(0x2c0, 0); }
    pre(w, id, ctx);
    let Some(b) = boss_of(w) else { return };
    let r = w.level22.radius;
    let hero = ctx.hero.pos;
    let (mut collision, mut facing, mut visible, mut distance) = (false, false, false, 0.0);
    let (st, sub0) = (w.mobys[id].state, w.mobys[id].sub_state);
    if st != 6 && sub0 != 5 {
        let pos = w.mobys[id].pos;
        if dist(w.mobys[b].pos, pos) < OPP_REACH {
            collision = true;
            w.mobys[id].set_state(5, 3);
        }
        facing = planet::turn_to(&w.mobys[id].rows, sub(hero, pos)) <= std::f32::consts::FRAC_PI_4;
        visible = planet::clear_walk(w, ctx, b, hero, pos);
        distance = dist(hero, pos);
        if 9.2 < ctx.hero.gravity.height(pos) - r {
            let p = planet::surface(ctx, r, pos, 2.0);
            if let Some(o) = moby_at(ctx, p, 0, Some(id), 5.0).filter(|&o| w.mobys[o].o_class == ICE) {
                let at = w.mobys[o].pos;
                burst(w, ctx, at, &Blast { size: 1.5, bursts: [0x19, 0x14, 0x14], puffs: 10, streaks: 15, light: true, shake: false });
                w.delete(o);
            }
        }
    }
    match w.mobys[id].sub_state {
        0 => {
            if first_sub(w, id) && seq(w, ctx, id) != 1 { ctx.host.set_anim(&w.mobys[id], 1, 0, 20); }
            if w.level22.intro == 0 {
                if visible && distance < 40.0 {
                    w.mobys[id].set_state(3, 1);
                    if w.mobys[id].pv_i(0x258) < 300 {
                        let k = ctx.game.rng.rand_range(480, 600);
                        w.mobys[id].set_pv_i(0x258, k);
                    }
                } else if w.mobys[id].drawn != 0 && distance < 80.0 {
                    let to = w.mobys[id].pv_v(0x2a0);
                    go(w, id, ctx, to, true, SPEED_0);
                } else {
                    still(w, id, ctx);
                }
            }
        }
        1 => {
            if first_sub(w, id) && seq(w, ctx, id) != 1 { ctx.host.set_anim(&w.mobys[id], 1, 0, 20); }
            if w.mobys[id].pv_i(0x258) == 0 {
                w.mobys[id].set_state(2, 0);
            } else if visible && facing && distance < 30.0 {
                w.mobys[id].set_state(4, 2);
                still(w, id, ctx);
            } else {
                go(w, id, ctx, hero, true, SPEED);
            }
        }
        2 => {
            if facing || first_sub(w, id) {
                if seq(w, ctx, id) != 3 { ctx.host.set_anim(&w.mobys[id], 3, 0, 10); }
                let sp = w.level22.damage[3];
                ctx.host.set_anim_speed(&w.mobys[id], sp);
            } else if ended(w, ctx, id) && seq(w, ctx, id) == 3 {
                ctx.host.set_anim(&w.mobys[id], 1, 0, 10);
            }
            if !visible || 50.0 < distance {
                w.mobys[id].set_state(2, 0);
                w.mobys[id].set_pv_i(0x258, 0);
            } else if 20.0 < distance {
                go(w, id, ctx, hero, true, SPEED);
            } else {
                frames::face_point(w, id, hero);
            }
        }
        3 => {
            let f = ai::flight(w, id, ctx);
            if f & 1 != 0 || collision {
                die(w, id, ctx, b);
            } else if f & 2 != 0 {
                w.mobys[id].set_state(3, 1);
                let k = ctx.game.rng.rand_range(580, 600);
                w.mobys[id].set_pv_i(0x258, k);
            }
        }
        _ => {}
    }
    let t = w.mobys[id].state_ticks as u16;
    match w.mobys[id].state {
        4 => {
            if ai::reached(w, id, ctx, 1.5) {
                let c = centre(ctx);
                shoot(w, id, ctx, c, 0.0);
            }
        }
        9 => {
            let p = planet::surface(ctx, r, w.mobys[id].pos, HIGH);
            if 61 <= t && t & 63 == 0 {
                let to = w.mobys[id].pv_v(0x2a0);
                shoot(w, id, ctx, to, 50.0);
            }
            if dist(w.mobys[id].pos, p) < 0.1 {
                let m = &mut w.mobys[id];
                m.set_state(2, 0);
                m.set_pv_i(0x88, m.pv_i(0x88) & !0x10);
            } else {
                go(w, id, ctx, p, false, SPEED_9);
            }
        }
        7 => {
            if t & 7 == 7 {
                let row = hero_rows(w, ctx)[0];
                let p = planet::surface(ctx, r, add(hero, scale(row, w.mobys[id].pv_f(0x2b4))), HIGH);
                w.mobys[id].set_pv_v(0x2a0, p);
            }
            if 61 <= t && t & 511 == 0 {
                let to = w.mobys[id].pv_v(0x2a0);
                shoot(w, id, ctx, to, 50.0);
            }
            let m = &w.mobys[id];
            if dist(m.pos, m.pv_v(0x2a0)) < 2.0 || dist(m.pos, hero) < 20.0 {
                let m = &mut w.mobys[id];
                m.set_state(9, 5);
                m.set_pv_i(0x280, 0);
            } else {
                let to = m.pv_v(0x2a0);
                go(w, id, ctx, to, false, SPEED);
            }
        }
        8 => {
            let rows = hero_rows(w, ctx);
            let g = &mut ctx.game.rng;
            let d = g.randf(0.0, 15.0) + 10.0;
            let way = match g.randi(4) { 0 => scale(rows[1], -1.0), 1 => rows[1], 2 => scale(rows[0], -1.0), _ => rows[0] };
            let g = &mut ctx.game.rng;
            let far = g.randf(FAR.0, FAR.1);
            let (yaw, pitch) = (g.rand_angle(), g.rand_angle());
            let l = g.randf(0.1, FAR.0 * f32::from_bits(0x3eaa_aaab));
            let jitter = crate::moby::classes::shot::polar(l, yaw, pitch);
            let start = add(add(scale(way, far), hero), jitter);
            let goal = planet::surface(ctx, r, add(hero, scale(rows[0], d)), HIGH);
            let m = &mut w.mobys[id];
            m.set_pv_f(0x2b4, d);
            m.pos = start;
            m.set_pv_v(0x2a0, goal);
            m.set_state(7, 5);
            m.set_pv_i(0x28c, 0);
            m.set_pv_i(0x88, m.pv_i(0x88) | 0x10);
        }
        _ => {}
    }
    tail(w, id);
}

/// Sub 3's death (module docs).
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, b: usize) {
    ctx.host.play_level_sound_after(None, 17, 0);
    let killer = ai::link(&w.mobys[id], 0x38);
    if killer.is_some() && killer == ctx.hero_moby && ctx.hero.giant.hp < w.mobys[b].pv_s(0x324) as i32 {
        let p = ctx.hero.pos;
        pickups::drop_at(w, p, 0);
    }
    let (class, pos, rot, rows) = { let m = &w.mobys[id]; (m.o_class, m.pos, m.rot, m.rows) };
    for i in 0..5u8 {
        let mut v = ctx.game.rng.rand_vec(PIECES.0 * 0.016_666_668, PIECES.1 * 0.016_666_668);
        v[2] = v[2].abs() * PIECES.2;
        let v = crate::gravity::to_world(v, &rows);
        crate::moby::shards::spawn_ball(w, ctx, class, Some(id), pos, rot, 30.0, 15.0, i, v, 0xff);
    }
    burst(w, ctx, pos, &Blast { size: 1.5, bursts: [0x19, 0x14, 0x14], puffs: 10, streaks: 15, light: true, shake: false });
    let r = w.level22.radius;
    let rows = hero_rows(w, ctx);
    let m = &mut w.mobys[id];
    m.set_pv_s(0x178, 0);
    m.has_collision = true;
    m.set_pv_f(0x20, m.pv_s(0x24) as f32);
    let saved = planet::surface(ctx, r, ctx.hero.pos, 0.0);
    let push = scale(rows[2], -80.0);
    let half = |a: V3, b: V3| scale(add(a, b), 0.5);
    let offset = match ctx.game.rng.randi(7) {
        0 => scale(rows[0], -50.0),
        1 => scale(rows[1], 50.0),
        2 => scale(rows[1], -50.0),
        3 => half(scale(rows[1], 50.0), scale(rows[0], 50.0)),
        4 => half(scale(rows[0], 50.0), scale(rows[1], -50.0)),
        5 => half(scale(rows[1], -50.0), scale(rows[0], -50.0)),
        _ => half(scale(rows[0], -50.0), scale(rows[1], 50.0)),
    };
    let to = add(add(push, offset), saved);
    if let Some(p) = planet::nearest_point(w, ctx, b, to, true) { w.mobys[id].pos = p; }
    let p = planet::surface(ctx, r, w.mobys[id].pos, HIGH);
    w.mobys[id].pos = p;
    push_off(w, id, ctx);
    let m = &mut w.mobys[id];
    let p = m.pos;
    m.set_pv_v(0x2a0, p);
    m.set_state(2, 0);
}

/// `0x40ef80` (module docs).
fn tail(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let mut k = m.pv_i(0x258);
    countdown(&mut k);
    m.set_pv_i(0x258, k);
    let mut k = m.pv_s(0x25c) as i32;
    countdown(&mut k);
    m.set_pv_s(0x25c, k as i16);
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = (m.state_ticks as u16 + 1) as i16; }
    let a = m.pv_f(0x26c);
    m.manips.retain(|x| x.0 != 7);
    m.manips.push((7, crate::quat::axis_angle(a, [0.0, 0.0, 1.0])));
    m.set_pv_f(0x26c, crate::hero::physics::add_rot(a, f32::from_bits(0x3ed4_fa2e)));
}

/// `0x40f020(m, at, damage)` (module docs).
fn shoot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, at: V3, damage: f32) {
    if boss_of(w).is_some_and(|b| w.mobys[b].pv_f(0x20) <= 0.0) { return; }
    let d = if damage == 0.0 { w.level22.damage[1] } else { damage };
    for j in [2, 3] {
        let Some(p) = ctx.host.joint(&w.mobys[id], j) else { continue };
        let v = set_len(sub(at, p), f32::from_bits(0x3f2a_aaab));
        make_shot(w, ctx, p, v, d);
    }
}

/// `0x40f300(pos, vel, &damage)`: the shot 0x0f5d at `pos` along `vel`, its rows along it.
fn make_shot(w: &mut MobyWorld, ctx: &Ctx, pos: V3, vel: V3, damage: f32) {
    let up = ctx.hero.gravity.up_at(pos);
    let id = w.create(SHOT);
    let m = &mut w.mobys[id];
    m.pvars.resize(m.pvars.len().max(0x40), 0);
    m.set_pv_f(0x1c, damage);
    m.set_pv_i(0x10, 0x50);
    (m.update_dist, m.draw_dist, m.state) = (0xff, 0xff, 0);
    m.pos = pos;
    m.set_pv_v(0, vel);
    m.set_pv_i(0x18, 0);
    m.set_pv_i(0x14, 0);
    let r0 = unit(vel);
    let r1 = unit(cross(r0, up));
    let r2 = cross(r1, r0);
    m.rows = [r0, r1, r2];
    m.mode |= crate::moby::mode::KEEP_ROWS;
}

/// 0x0f5d (`0x40f240`, module docs).
pub fn update_0f5d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0x40 { w.mobys[id].pvars.resize(0x40, 0); }
    match w.mobys[id].state {
        0 | 1 => {
            w.mobys[id].state = 1;
            let m = &mut w.mobys[id];
            let from = m.pos;
            m.pos = add(m.pos, m.pv_v(0));
            shot_hit(w, id, ctx, from);
            let m = &mut w.mobys[id];
            let mut k = m.pv_i(0x10);
            if countdown(&mut k) { m.state = 2; }
            m.set_pv_i(0x10, k);
        }
        2 => w.delete(id),
        _ => {}
    }
}

/// `0x40f428(m, from)` (module docs).
fn shot_hit(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, from: V3) {
    let to = w.mobys[id].pos;
    let Some(h) = w.coll_line(ctx, from, to, 0, Some(id)) else { return };
    let Some(o) = h.moby else { return };
    if matches!(w.mobys[o].o_class, CLASS | boss::CLASS) { return; }
    let mut dmg = w.mobys[id].pv_f(0x1c);
    if Some(o) == ctx.hero_moby && !(dmg < 50.0) { return; }
    if 50.0 <= dmg {
        dmg = 50.1;
        w.mobys[id].set_pv_f(0x1c, dmg);
    }
    let v = unit(w.mobys[id].pv_v(0));
    let mut t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 0x8b_0003, [v[0], v[1], v[2], f32::from_bits(0x45af_df66)]);
    t.kind = (SHOT as u16 as u32) << 16 | 0x305;
    w.deliver_hit_any(o, &t, [h.point[0], h.point[1], h.point[2], 1.0]);
    let s = ctx.game.rng.randf(0.8, 1.2);
    burst(w, ctx, to, &Blast { size: s, bursts: [5, 4, 4], puffs: 4, streaks: 4, light: false, shake: false });
    w.mobys[id].state = 2;
}
