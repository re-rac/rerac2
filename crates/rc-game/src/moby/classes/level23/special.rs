//! Class 0x1190, Damosel orbit's special (L23: made `0x424880`, held `0x4249c0`, given up `0x424a88`, its update
//! `0x424bb8`; the decomp's `func_L23_00423380` / `func_L23_004234C0` / `func_L23_00423588` / `func_L23_004236B8`;
//! this level's own copy of Dobbo orbit's 0x05f3, crate::moby::classes::level22::special): Giant Clank's ball of
//! charge that grows while he holds it and then rolls off along the planetoid, knocking away what it meets. His word
//! for it is `$gp+0x2914` (the orbits' shared "one out" test `0x424b70` reads it: crate::hero::giant::GiantBlock::special),
//! another owner's `$gp+0x2918`; their sounds' handles `$gp+0x290c` / `+0x2910`.
//!
//! * **Held** ([`hold`], `0x4249c0(owner, at, hero)`): with none out, one made ([`make`]), the level sound 0x14
//!   started, its kind `+0x40` (his) and words `+0x44` / `+0x48` 0. Then it is set at `at` and flagged held (`+0x2e`).
//! * **Made** ([`make`], `0x424880`): update and draw distance 0xff, drawn, its owner the parent (`+0xb8`), state 0
//!   at the point; the charge `+0x10` 0; with the frames on mode | 0x100 and the owner's rows.
//! * **0, charging**: held this tick while Giant Clank is still the body (or another's): the charge counts. Else it
//!   is let go: the level sound 0x16, the held sound swapped for 0x15; it sets off 2 along its first row (frames off:
//!   ⅔ along his yaw) — state 1, its ticks `+0x14` 0.
//! * **1, rolling**: its radius `t/3 + 1.3`; its row 2 turned toward `row 2 × up` by 0.4 (another's 0.8; the
//!   quaternion's w −√(1 − |v|²)), its way 2 along its first row (another's 3.5; with `+0x42` its way `+0x50`). Gone
//!   at 51 ticks, once Giant Clank is no longer the body, or out of the level's box (2..1021); else it moves by its
//!   way and knocks away what is within its radius (a sphere query 0x10 without itself: `0x1b3684`, the opponent's
//!   `+0x314`, flags 0x8b0000, out 1, up 1; another's without its owner, 0x8b0007), a blast where it hits. With the
//!   frames on it stands up on the sphere each tick.
//! * **Given up** ([`end`], `0x424a88(owner, all)`): its sound stopped; one charging (state 0), or any with `all`,
//!   deleted and the word cleared (Giant Clank's knock-back gives up a charging one; the opponent's ring blows up any).
//!
//! Not ported: its look (the screen's fill `0x1a8ab0..0x1a8ac8`, its point light `+0x2c`, the charge's sparks, the
//! roll's particles and trails, their random draws with them).

use crate::hero::physics::{add, cross, dot, scale, set_len, unit, V3};
use crate::moby::frames::upright;
use crate::moby::{Ctx, MobyWorld};
use crate::quat::turn_rows;

pub const CLASS: i16 = 0x1190;

fn word(w: &MobyWorld, ctx: &Ctx, hero: bool) -> Option<usize> { if hero { ctx.hero.giant.special } else { w.level23.special[1] } }
fn set_word(w: &mut MobyWorld, ctx: &mut Ctx, hero: bool, v: Option<usize>) { if hero { ctx.hero.giant.special = v } else { w.level23.special[1] = v } }

fn stop_sound(w: &mut MobyWorld, ctx: &mut Ctx, hero: bool) {
    let s = &mut w.level23.special_sound[!hero as usize];
    if *s != -1 {
        ctx.host.stop_sound(*s);
        *s = -1;
    }
}

/// `0x424880(owner, at)` (module docs); its word is set here.
pub fn make(w: &mut MobyWorld, ctx: &mut Ctx, owner: usize, at: V3) -> usize {
    let hero = ctx.hero_moby == Some(owner);
    let id = w.create(CLASS);
    let rows = w.mobys[owner].rows;
    let m = &mut w.mobys[id];
    m.pvars.resize(crate::moby::PVAR_BYTES, 0);
    (m.update_dist, m.draw_dist, m.drawn, m.parent, m.state) = (0xff, 0xff, 1, Some(owner), 0);
    m.pos = at;
    m.set_pv_i(0x10, 0);
    m.set_pv_s(0x2c, -1);
    if ctx.hero.gravity.on {
        m.mode |= 0x100;
        m.rows = rows;
    }
    set_word(w, ctx, hero, Some(id));
    id
}

/// `0x4249c0(owner, at, hero)` (module docs).
pub fn hold(w: &mut MobyWorld, ctx: &mut Ctx, owner: usize, at: V3, hero: bool) {
    if word(w, ctx, hero).is_none() {
        let id = make(w, ctx, owner, at);
        w.level23.special_sound[!hero as usize] = ctx.host.play_level_sound_after(None, 0x14, 0);
        let m = &mut w.mobys[id];
        m.set_pv_s(0x40, hero as i16);
        m.set_pv_i(0x48, 0);
        m.set_pv_i(0x44, 0);
    }
    let Some(id) = word(w, ctx, hero) else { return };
    let m = &mut w.mobys[id];
    m.pos = at;
    m.set_pv_s(0x2e, 1);
}

/// `0x424a88(owner, all)` (module docs); `hero` the owner is Giant Clank.
pub fn end(w: &mut MobyWorld, ctx: &mut Ctx, hero: bool, all: bool) {
    stop_sound(w, ctx, hero);
    let Some(id) = word(w, ctx, hero) else { return };
    if !w.mobys.get(id).is_some_and(|m| m.o_class == CLASS) {
        set_word(w, ctx, hero, None);
        return;
    }
    if !all && w.mobys[id].state != 0 { return; }
    w.delete(id);
    set_word(w, ctx, hero, None);
}

fn finish(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, hero: bool) {
    stop_sound(w, ctx, hero);
    w.delete(id);
    set_word(w, ctx, hero, None);
}

/// 0x1190's update (`0x424bb8`, module docs).
pub fn update_1190(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].pvars.resize(crate::moby::PVAR_BYTES, 0);
    let hero = w.mobys[id].parent.is_some() && w.mobys[id].parent == ctx.hero_moby;
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
                let s = w.level23.special_sound[k];
                if s != -1 {
                    ctx.host.stop_sound(s);
                    w.level23.special_sound[k] = ctx.host.play_level_sound_after(None, 0x15, 0);
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
            let radius = t as f32 * f32::from_bits(0x3eaa_aaab) + 1.3;
            w.mobys[id].set_pv_i(0x14, t + 1);
            let up = ctx.hero.gravity.up_at(w.mobys[id].pos);
            let m = &mut w.mobys[id];
            let mut way = set_len(m.rows[0], if mine { 2.0 } else { 3.5 });
            let a = scale(cross(m.rows[2], unit(up)), if mine { 0.4 } else { 0.8 });
            m.rows = turn_rows(&m.rows, [a[0], a[1], a[2], -(1.0 - dot(a, a)).sqrt()]);
            if m.pv_s(0x42) != 0 {
                way = scale(m.pv_v(0x50), if mine { f32::from_bits(0x3f2a_aaab) } else { 1.0 });
                m.rows[0] = set_len(way, 1.0);
            }
            m.set_pv_v(0, way);
            if 51 <= t + 1 || !giant {
                finish(w, id, ctx, hero);
                return;
            }
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, way);
            let p = m.pos;
            if p.iter().any(|&c| c < 2.0 || 1021.0 < c) {
                finish(w, id, ctx, hero);
                return;
            }
            let ignore = if mine { Some(id) } else { w.mobys[id].parent };
            let list = ctx.scene.map_or(Vec::new(), |sc| crate::collision::mobys::coll_sphere_mobys(sc, p, radius, crate::collision::QueryFlags(0x10), ignore));
            let flags = if mine { 0x8b_0000 } else { 0x8b_0007 };
            w.knock_hits(w.level23.special_damage, 1.0, 1.0, id, p, &list, None, flags, 2, 1);
            if !list.is_empty() {
                use crate::moby::blast::{burst, Blast};
                burst(w, ctx, p, &Blast { size: 1.0, bursts: [25, 20, 20], puffs: 20, streaks: 45, light: true, shake: false });
            }
        }
        _ => {}
    }
    if ctx.hero.gravity.on {
        let m = &mut w.mobys[id];
        let down = scale(ctx.hero.gravity.up_at(m.pos), -1.0);
        m.rows = turn_rows(&m.rows, upright(&m.rows, scale(down, -1.0)));
    }
    w.mobys[id].set_pv_s(0x2e, 0);
}
