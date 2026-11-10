//! Class 0x100, the orbit fight's shots (L22: made `0x39e1f0`, its update `0x39e5c8`; the decomp's
//! `func_L22_0039CDC0` / `func_L22_0039D198`): Giant Clank's gun (crate::hero::giant) and the opponent fire them.
//!
//! **Made** ([`fire`], `0x39e1f0(owner, at, target, mode, player)`): update and draw distance 0xff, drawn, the owner
//! its parent with its angles and light (`0x32efc0` = L00 `0x32a748`, crate::moby::MobyWorld::adopt), its scale × 3 × 2 (`$gp+0x270`), mode |
//! 0x100 with the owner's rows; the turn speed `+0x1c` ⅓, the timer `+0x14` 150. Its target: the one given, else the
//! opponent (`$gp+0x2d4`), else none (then mode and player 0); the owner Giant Clank makes it a player's. Without a
//! mode it tests its target: the way to the target's point 5 up its up row (`+0x30`, unit) and the way on the
//! surface (both at height 0); no target when the owner is Giant Clank and the target is not drawn (`+0x31`), when
//! that way is more than acos 0.72 off the shot's first row, the target more than 50 off, or within 5 but more than
//! 10 apart in height. Giant Clank's in his jump (0x5b) without a target fly straight down his up row (`+0x30`), as
//! a plain one. The class sound 1.
//!
//! **Each tick** ([`update`]): mode &= ~0x41; the turn speed eased to 7/3 by 1/18; its rows turned toward the up
//! (`row 2 × down`·0.1, the quaternion's w −√(1 − |v|²)) and its way the first row at the turn speed. With a target
//! and a mode it homes (`0x326fb0` = L00 `0x321de8`: 0.012, 0.012, no most, near 20, about the down) on the target's
//! point 5.5 up its up row; a player's after its first 5 ticks while the target is ahead of it (near 0.1); a plain
//! one flies along `+0x30`. Then it moves; gone below 0 on an axis. The hit: a line 0 from where it was without its
//! owner, a template of 4 (`0x1b2a14`, the opponent's `+0x330`), flags 0x8b0003, the kind `class << 16 | 0x303`,
//! along its way (w 5627.925): a moby met (not its parent) or the world stops it there (`+0xbc` 2 / 1, the class
//! sound 0); none, the timer counts down and it goes at 0. Stopped, the same hit to what is within 2 of it
//! (`0x2e7708`, without the owner), and it goes.
//!
//! Not ported: its trail (`0x342750`: the draws `randf(0.1, 0.3)`, `rand(255)`, `0x321120(0, 0.1)` are made), its
//! burst `0x325a10`, Giant Clank's aim pointer (`0x18ac2c` → `+0x30`, `0x1b18b0`), L23's copy (`0x3a2828`).

use super::planet::{down_at, surface};
use crate::hero::physics::{add, cross, dist, dot, scale, set_len, sub, unit, V3};
use crate::moby::hits::HitTemplate;
use crate::moby::{countdown, Ctx, MobyWorld};
use crate::quat::turn_rows;

pub const CLASS: i16 = 0x0100;

/// The target word `+0x18`: none, a moby (index + 1), or Giant Clank's straight-down shot's 1 (here −1).
const DOWN: i32 = -1;

fn target_of(w: &MobyWorld, m: &crate::moby::Moby) -> Option<usize> { usize::try_from(m.pv_i(0x18) - 1).ok().filter(|&i| i < w.mobys.len()) }

/// `0x39e1f0` (module docs).
pub fn fire(w: &mut MobyWorld, ctx: &mut Ctx, owner: usize, at: V3, target: Option<usize>, mode: bool, player: bool) -> usize {
    let id = w.create(CLASS);
    let (opos, orows) = { let o = &w.mobys[owner]; (o.pos, o.rows) };
    let target = target.or(w.level22.boss);
    let hero = ctx.hero_moby == Some(owner);
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    w.adopt(id, Some(owner), &*ctx.host);
    let m = &mut w.mobys[id];
    m.scale *= 3.0;
    m.mode |= 0x100;
    m.set_pv_f(0x1c, f32::from_bits(0x3eaa_aaab));
    m.rows = orows;
    m.set_pv_i(0x10, owner as i32 + 1);
    m.pos = at;
    m.set_pv_i(0x14, 150);
    m.scale += m.scale;
    m.set_pv_i(0x28, mode as i32);
    m.set_pv_i(0x2c, player as i32);
    match target {
        Some(t) => m.set_pv_i(0x18, t as i32 + 1),
        None => {
            m.set_pv_i(0x18, 0);
            m.set_pv_i(0x28, 0);
            m.set_pv_i(0x2c, 0);
        }
    }
    if hero { m.set_pv_i(0x2c, 1); }
    if let Some(t) = target.filter(|_| m.pv_i(0x28) == 0) {
        let (tpos, trows, tdrawn) = { let t = &w.mobys[t]; (t.pos, t.rows, t.drawn) };
        let radius = w.level22.radius;
        let toward = add(tpos, scale(trows[2], 5.0));
        let on = surface(ctx, radius, tpos, 0.0);
        let m = &mut w.mobys[id];
        let start = surface(ctx, radius, m.pos, 0.0);
        let way = unit(sub(on, start));
        m.set_pv_v(0x30, set_len(sub(toward, m.pos), 1.0));
        let (oh, th) = (ctx.hero.gravity.height(opos), ctx.hero.gravity.height(tpos));
        let d = dist(on, start);
        let reject = d < 5.0 && 10.0 < (oh - th).abs();
        if (hero && tdrawn == 0) || dot(way, m.rows[0]) < f32::from_bits(0x3f38_51ec) || 50.0 < d || reject { m.set_pv_i(0x18, 0); }
        if hero && m.pv_i(0x18) == 0 && ctx.hero.state == 0x5b {
            m.set_pv_v(0x30, scale(orows[2], -1.0));
            m.set_pv_i(0x28, 0);
            m.set_pv_i(0x18, DOWN);
            m.set_pv_i(0x2c, 0);
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

/// 0x0100's update (`0x39e5c8`, module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let down = down_at(ctx, w.mobys[id].pos);
    let target = target_of(w, &w.mobys[id]);
    let aim = target.map(|t| add(w.mobys[t].pos, scale(w.mobys[t].rows[2], 5.5)));
    let tpos = target.map(|t| w.mobys[t].pos);
    let m = &mut w.mobys[id];
    m.mode &= 0xffbe;
    let mut turn = m.pv_f(0x1c);
    crate::hero::physics::approach(f32::from_bits(0x4015_5556), f32::from_bits(0x3d63_8e39), &mut turn);
    m.set_pv_f(0x1c, turn);
    let mut vel = set_len(m.rows[0], turn);
    let a = scale(cross(m.rows[2], unit(down)), 0.2 * 0.5);
    m.rows = turn_rows(&m.rows, [a[0], a[1], a[2], -(1.0 - dot(a, a)).sqrt()]);
    let home = |m: &mut crate::moby::Moby, aim: V3, near: f32| {
        let mut t = m.pv_f(0x20);
        let k = f32::from_bits(0x3c44_9ba6);
        m.rows[0] = crate::moby::classes::turret::home(k, k, 0.0, near, m.pos, m.rows[0], aim, &mut t, down);
        m.set_pv_f(0x20, t);
        m.rows[0] = set_len(m.rows[0], 1.0);
        set_len(m.rows[0], m.pv_f(0x1c))
    };
    match (m.pv_i(0x18), aim, tpos) {
        (0, _, _) => {}
        (_, Some(aim), _) if m.pv_i(0x28) != 0 => vel = home(m, aim, 20.0),
        (_, Some(aim), Some(tp)) if m.pv_i(0x2c) != 0 => {
            if m.pv_i(0x14) < 0x91 && dot(sub(m.pos, tp), vel) < 0.0 { vel = home(m, aim, 0.1); }
        }
        (t, _, _) if t == DOWN || (m.pv_i(0x28) == 0 && m.pv_i(0x2c) == 0) => {
            vel = scale(m.pv_v(0x30), turn);
            m.rows[0] = set_len(vel, 1.0);
        }
        _ => {}
    }
    // The trail's draws (`0x342750` itself not ported).
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
    let mut t = HitTemplate::new(w.level22.damage[0], Some(id), 0x8b_0003, [v[0], v[1], v[2], f32::from_bits(0x45af_df66)]);
    t.kind = (CLASS as u16 as u32) << 16 | 0x303;
    if w.mobys[id].timer == 0 {
        if let Some(hit) = w.coll_line(ctx, from, w.mobys[id].pos, 0, owner) {
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
            ctx.host.play_sound(&w.mobys[id], 0, 0);
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
