//! Class 0x5f3, Giant Clank's special (L22: made `0x3ba428`, held `0x3ba568`, given up `0x3ba798`, out `0x3ba868`,
//! its update `0x3ba890`; the decomp's `func_L22_003B8FE8` / `func_L22_003B9128` / `func_L22_003B9358` /
//! `func_L22_003B9428` / `func_L22_003B9450`): a ball of charge that grows while he holds it and then rolls off along
//! the planetoid, knocking away what it meets. Each owner has one word for it: Giant Clank's `$gp+0x9c8`
//! (crate::hero::giant::GiantBlock::special), the opponent's `$gp+0x9cc`; their sounds' handles `$gp+0x9c0` / `+0x9c4`.
//!
//! * **Held** ([`hold`], `0x3ba568(owner, at, 1)`): with none out, one made ([`make`]) and the level sound 0x14
//!   started; with the gravity frames on it aims at the opponent (`+0x44`) when the way to its point 4 up its up row
//!   (`+0x50`, unit, from Giant Clank) is within 0.75 (a dot) of his facing and the opponent within 55, `+0x42` then
//!   set, and the homing's turn `+0x48` 0; `+0x40` his. Then it is set at `at` and flagged held (`+0x2e`).
//! * **Made** ([`make`], `0x3ba428`): update and draw distance 0xff, drawn, its owner the parent (`+0xb8`), state 0
//!   at the point; the charge `+0x10` 0; with the frames on mode | 0x100 and the owner's rows.
//! * **0, charging**: held (this tick, `+0x2e`) while Giant Clank is still the player's body: the charge counts.
//!   Else it is let go: the level sound 0x16, his 0x14 swapped for 0x15; it sets off 2 along its first row (frames
//!   off: ⅔ along his yaw) — state 1, the roll's ticks `+0x14` 0.
//! * **1, rolling**: its radius `t/3 + 1.3` (t the roll's ticks, counted), its row 2 turned toward `row 2 × up`
//!   by 0.4 (the opponent's 0.8; the quaternion's w −√(1 − |v|²)) and its way 2 along its first row (the
//!   opponent's 3.5 along `+0x50`); within 4 of Giant Clank its aim ends. Aimed and set off 6 ticks or more, with the
//!   target ahead of it: it homes (`0x326fb0`, 0.0043, 0.0043) on the target's point 6.5 up its up row. Gone past 50
//!   ticks, once Giant Clank is no longer the body, or out of the level's box (2..1021 on each axis); else it moves
//!   by its way and knocks away what is within half its radius (a sphere query 0x10 without itself,
//!   `0x3233d0` = L00 `0x31def8`: 48 a hit — `0x1b2a1c` — flags 0x8b0000, out 1, up 1; the opponent's flags
//!   0x8b0007, without the opponent). With the frames on it stands up on the sphere each tick.
//! * **Given up** ([`end`], `0x3ba798`): his sound stopped; one still charging (state 0) deleted; the word cleared.
//!
//! Not ported: its look (the screen's fill `0x1a8ab0..0x1a8ac8`, its point light `+0x2c` `0x302fd0`, the charge's
//! sparks `0x33ad38`, the roll's particles and trail `0x3585b8` / `0x3364e0` / `0x34b2e8`, the blast `0x3f8348` at a
//! hit; their random draws with them); the opponent's special (its AI, crate::moby::classes::level22::boss, is not
//! ported: only Giant Clank's is held); L23's copy (its update differs: L23's Giant Clank makes none).

use super::planet::{down_at, surface};
use crate::hero::physics::{add, cross, dist, dot, scale, set_len, sub, unit, V3};
use crate::moby::frames::upright;
use crate::moby::{Ctx, MobyWorld};
use crate::quat::turn_rows;

pub const CLASS: i16 = 0x05f3;

/// `0x1b2a1c`: the knock's damage (the opponent's pvar `+0x338`, kept by its update: crate::moby::classes::level22::boss).
fn damage(w: &MobyWorld) -> f32 { w.level22.damage[2] }

/// `0x3ba428(owner, at)`: the ball (module docs); its word is set by the caller.
pub fn make(w: &mut MobyWorld, ctx: &Ctx, owner: usize, at: V3) -> usize {
    let id = w.create(CLASS);
    let rows = w.mobys[owner].rows;
    let m = &mut w.mobys[id];
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.parent = Some(owner);
    m.state = 0;
    m.pos = at;
    m.set_pv_i(0x10, 0);
    m.set_pv_s(0x2c, -1);
    if ctx.hero.gravity.on {
        m.mode |= 0x100;
        m.rows = rows;
    }
    id
}

/// `0x3ba568(Giant Clank, at, 1)` (module docs).
pub fn hold(w: &mut MobyWorld, ctx: &mut Ctx, owner: usize, at: V3) {
    if ctx.hero.giant.special.is_none() {
        let id = make(w, ctx, owner, at);
        ctx.hero.giant.special = Some(id);
        w.level22.special_sound[0] = ctx.host.play_level_sound_after(None, 0x14, 0);
        if ctx.hero.gravity.on {
            let (opos, orows) = (w.mobys[owner].pos, w.mobys[owner].rows);
            match w.level22.boss {
                None => w.mobys[id].set_pv_i(0x48, 0),
                Some(b) => {
                    let (bp, brows) = (w.mobys[b].pos, w.mobys[b].rows);
                    let aim = add(bp, scale(brows[2], 4.0));
                    let to = set_len(sub(aim, opos), 1.0);
                    let m = &mut w.mobys[id];
                    m.set_pv_v(0x50, to);
                    let facing = set_len(orows[0], 1.0);
                    let d = dist(bp, ctx.hero.pos);
                    if dot(to, facing) < 0.75 || 55.0 < d {
                        m.set_pv_i(0x44, 0);
                    } else {
                        m.set_pv_i(0x44, b as i32 + 1);
                        m.set_pv_s(0x42, 1);
                    }
                    m.set_pv_i(0x48, 0);
                }
            }
        }
        w.mobys[id].set_pv_s(0x40, 1);
    }
    let Some(id) = ctx.hero.giant.special else { return };
    let m = &mut w.mobys[id];
    m.pos = at;
    m.set_pv_s(0x2e, 1);
}

/// `0x3ba798(Giant Clank)` (module docs).
pub fn end(w: &mut MobyWorld, ctx: &mut Ctx) {
    let s = &mut w.level22.special_sound[0];
    if *s != -1 {
        ctx.host.stop_sound(*s);
        *s = -1;
    }
    let Some(id) = ctx.hero.giant.special else { return };
    if !w.mobys.get(id).is_some_and(|m| m.o_class == CLASS) {
        ctx.hero.giant.special = None;
        return;
    }
    if w.mobys[id].state != 0 { return; }
    w.delete(id);
    ctx.hero.giant.special = None;
}

/// The end of its roll: its owner's sound stopped, deleted, the word cleared.
fn finish(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, hero: bool) {
    let s = &mut w.level22.special_sound[!hero as usize];
    if *s != -1 {
        ctx.host.stop_sound(*s);
        *s = -1;
    }
    w.delete(id);
    if hero { ctx.hero.giant.special = None; } else { w.level22.boss_special = None; }
}

/// 0x05f3's update (`0x3ba890`, module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let hero = w.mobys[id].parent == ctx.hero_moby;
    let mine = w.mobys[id].pv_s(0x40) != 0;
    let giant = ctx.hero.mode == 2;
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            if m.pv_s(0x2e) != 0 && (giant || !mine) {
                m.set_pv_i(0x10, m.pv_i(0x10) + 1);
            } else {
                m.set_pv_i(0x10, 0);
                ctx.host.play_level_sound_after(None, 0x16, 0);
                let k = !mine as usize;
                let s = w.level22.special_sound[k];
                if s != -1 {
                    ctx.host.stop_sound(s);
                    w.level22.special_sound[k] = ctx.host.play_level_sound_after(None, 0x15, 0);
                }
                let m = &mut w.mobys[id];
                let way = if !ctx.hero.gravity.on {
                    let y = ctx.hero.rot[2];
                    [y.cos() * f32::from_bits(0x3f2a_aaab), y.sin() * f32::from_bits(0x3f2a_aaab), 0.0]
                } else {
                    scale(m.rows[0], 2.0)
                };
                m.set_pv_v(0, way);
                m.state = 1;
                m.set_pv_i(0x14, 0);
            }
        }
        1 => {
            let t = w.mobys[id].pv_i(0x14);
            let mut radius = t as f32 * f32::from_bits(0x3eaa_aaab) + 1.3;
            w.mobys[id].set_pv_i(0x14, t + 1);
            radius *= 0.5;
            let up = ctx.hero.gravity.up_at(w.mobys[id].pos);
            let m = &mut w.mobys[id];
            let mut way = set_len(m.rows[0], if mine { 2.0 } else { 3.5 });
            let a = scale(cross(m.rows[2], unit(up)), if mine { 0.4 } else { 0.8 });
            m.rows = turn_rows(&m.rows, [a[0], a[1], a[2], -(1.0 - dot(a, a)).sqrt()]);
            if mine {
                if dist(ctx.hero.pos, m.pos) < 4.0 {
                    m.set_pv_i(0x44, 0);
                    m.set_pv_s(0x42, 0);
                }
            } else {
                way = scale(m.pv_v(0x50), 1.0);
                m.rows[0] = set_len(way, 1.0);
            }
            let target = usize::try_from(m.pv_i(0x44) - 1).ok().filter(|&i| i < w.mobys.len());
            if let Some(tg) = target.filter(|_| mine && w.mobys[id].pv_s(0x42) != 0 && 6 <= t + 1) {
                let (tp, trows) = (w.mobys[tg].pos, w.mobys[tg].rows);
                let m = &mut w.mobys[id];
                if dot(sub(m.pos, tp), way) < 0.0 {
                    let aim = add(tp, scale(trows[2], 6.5));
                    let mut turn = m.pv_f(0x48);
                    let k = f32::from_bits(0x3b8c_e704);
                    m.rows[0] = crate::moby::classes::turret::home(k, k, 0.0, 0.0, m.pos, m.rows[0], aim, &mut turn, up);
                    m.set_pv_f(0x48, turn);
                    m.rows[0] = set_len(m.rows[0], 1.0);
                    way = scale(m.rows[0], 2.0);
                }
            }
            w.mobys[id].set_pv_v(0, way);
            if 50 < t + 1 || !giant {
                finish(w, id, ctx, hero);
                return;
            }
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, way);
            if !mine && ctx.hero.gravity.height(m.pos) - w.level22.radius < 1.3 {
                let p = surface(ctx, w.level22.radius, w.mobys[id].pos, 1.3);
                w.mobys[id].pos = p;
            }
            let p = w.mobys[id].pos;
            if p.iter().any(|&c| c < 2.0 || 1021.0 < c) {
                finish(w, id, ctx, hero);
                return;
            }
            let ignore = if mine { Some(id) } else { w.mobys[id].parent };
            let list = ctx.scene.map_or(Vec::new(), |sc| crate::collision::mobys::coll_sphere_mobys(sc, p, radius, crate::collision::QueryFlags(0x10), ignore));
            let flags = if mine { 0x8b_0000 } else { 0x8b_0007 };
            w.knock_hits(damage(w), 1.0, 1.0, id, p, &list, None, flags, 2, 1);
        }
        _ => {}
    }
    if ctx.hero.gravity.on {
        let m = &mut w.mobys[id];
        let down = down_at(ctx, m.pos);
        m.rows = turn_rows(&m.rows, upright(&m.rows, scale(down, -1.0)));
    }
    w.mobys[id].set_pv_s(0x2e, 0);
}
