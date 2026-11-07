//! Damosel orbit's targets, parts of its opponent 0x0f7f's fight (L23 addresses; the words they read are the
//! opponent's, crate::moby::classes::level23::boss, kept in crate::moby::classes::level23::Globals):
//!
//! * **0x0f7e** ([`update_0f7e`], `0x417498` → `0x4174c8` + `0x417590`, the decomp's `func_L23_00415FC8` /
//!   `func_L23_004160B0` / `func_L23_00416090`; 10 placed) and **0x120f** ([`update_120f`], `0x428cb8` → `0x428ce8`
//!   + `0x428db0`, `func_L23_004277E8` / `func_L23_004278D8` / `func_L23_004278B0`; one placed): their set-ups are the
//!   same code. **0**: mode | 0x4100, update and draw distance 0xff, health 2000 (`+0x20` / `+0x24`), all ten of
//!   their pieces on top (`+0x7c` 0x3ff), state 1. **1**: their health the fight's word when it has one (0x0f7e
//!   `$gp+0x2388`, 0x120f `$gp+0x468c`), state 2. Each tick their hit ([`take`]), their ticks counted.
//! * **The hit** (`0x4175b0` / `0x428dd8`, `0x3231c0(m, 0x30000, 1)`, the record dropped): out of their 20-tick wait
//!   (`+0x26`) and while the opponent's guard is down (its `+0x604`; up, its hurt sound `0x41fa70`), the flash
//!   (`+0x140`, red), the blast at the hitter (`0x41da78`: from Giant Clank 3 at (1.3, 0, 4.5) in his frame, else 1.5
//!   at it; the opponent's class sound 3 unless the hitter is his shot 0x118f), the damage off their health and the
//!   wait 20 (but for his shots). The 0x0f7e keeps its health at the floor `$gp+0x238c` and hands a hit there on to the
//!   opponent (`0x41d370`: to it, its held moby `+0x610` or its slot `+0x358 + 4·+0x60c`); the 0x120f takes hits only in
//!   the opponent's states 0x0c / 0x10 (`0x41f760`) and not from Giant Clank while `$gp+0x2654` is below 0, and stops
//!   at no health. As their health falls by tenths of the fight's word their pieces come off (balls flung 2..4 a second
//!   90..120° either side of the camera's yaw, 5..8 up). Their flash counted.
//!
//! Not ported: the pieces' own update (`0x3ac280`, the shards' here); the hit's point `0x1b3660` (the target's place
//! here); the camera's yaw read at `0x1ba5d8` (the view's here).

use crate::hero::physics::{add, add_rot};
use crate::moby::hits::HitTemplate;
use crate::moby::{ai, Ctx, MobyWorld};

pub const F7E: i16 = 0x0f7e;
pub const T120F: i16 = 0x120f;
/// The opponent and its shot.
pub const OPPONENT: i16 = 0x0f7f;
const SHOT: i16 = 0x118f;

/// `$gp+0x2444`: the opponent (its set-up sets the word, crate::moby::classes::level23::boss).
pub(super) fn opponent(w: &MobyWorld) -> Option<usize> { w.level23.boss.filter(|&b| w.mobys.get(b).is_some_and(|m| m.o_class == OPPONENT && m.state < 0xfd)) }

/// `0x41ce48`: the opponent's guard up (`+0x604`).
pub(super) fn guarded(w: &MobyWorld, o: Option<usize>) -> bool { o.is_some_and(|o| w.mobys[o].pv_i(0x604) != 0) }

/// `0x41fa70`: the opponent's hurt sound (0x10) unless its wait `0x1b3694` runs; the wait `$gp+0x23a8` (60).
pub(super) fn hurt_sound(w: &mut MobyWorld, ctx: &mut Ctx, o: Option<usize>) {
    if w.level23.hurt_wait == 0 {
        if let Some(o) = o { ctx.host.play_sound(&w.mobys[o], 0x10, 0); }
    }
    w.level23.hurt_wait = 60;
}

/// `0x41da78(hitter)` (module docs).
pub(super) fn blast_at(w: &mut MobyWorld, ctx: &mut Ctx, src: usize, o: Option<usize>) {
    use crate::moby::blast::{burst, Blast};
    if ctx.hero_moby == Some(src) {
        let at = add(crate::gravity::to_world([1.3, 0.0, 4.5], &w.mobys[src].rows), ctx.hero.pos);
        burst(w, ctx, at, &Blast { size: 3.0, bursts: [10, 10, 10], puffs: 25, streaks: 30, light: false, shake: false });
    } else {
        let at = w.mobys[src].pos;
        burst(w, ctx, at, &Blast { size: 1.5, bursts: [25, 20, 20], puffs: 15, streaks: 25, light: false, shake: false });
    }
    if w.mobys[src].o_class != SHOT {
        if let Some(o) = o { ctx.host.play_sound(&w.mobys[o], 3, 0); }
    }
}

/// `0x41d370(hit)`: the hit handed on to the opponent (module docs).
pub(super) fn hand_on(w: &mut MobyWorld, o: Option<usize>, src: Option<usize>, dmg: f32) {
    let Some(o) = o else { return };
    let m = &w.mobys[o];
    let to = match m.pv_i(0x60c) {
        -1 => ai::link(m, 0x610).unwrap_or(o),
        k => match ai::link(m, 0x358 + 4 * k.clamp(0, 63) as usize) { Some(s) => s, None => return },
    };
    let Some(p) = w.mobys.get(to).map(|t| t.pos) else { return };
    let t = HitTemplate::new(dmg, src, 0x1_0000, [0.0; 4]);
    w.deliver_hit_any(to, &t, [p[0], p[1], p[2], 1.0]);
}

/// The set-up (`0x4174c8` / `0x428ce8`, module docs).
fn set_up(w: &mut MobyWorld, id: usize, health: f32) {
    let m = &mut w.mobys[id];
    if m.pvars.len() < 0x160 { m.pvars.resize(0x160, 0); }
    match m.state {
        0 => {
            m.mode |= 0x100;
            (m.draw_dist, m.update_dist) = (0xff, 0xff);
            m.set_pv_f(0x20, 2000.0);
            m.set_pv_s(0x24, 2000);
            m.bangles = 0x3ff;
            m.set_state(1, 0);
            m.mode |= 0x4000;
        }
        1 => {
            if health != 0.0 {
                m.set_pv_f(0x20, health);
                m.set_pv_s(0x24, health as i16);
            }
            m.set_state(2, 0);
        }
        _ => {}
    }
}

/// The pieces off as the health falls (module docs).
fn pieces(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, n: i32) {
    let yaw = crate::moby::attach::angles_of(&ctx.host.camera_rows())[2];
    for i in 0..n.clamp(0, 16) {
        let mask = w.mobys[id].bangles;
        if mask >> i & 1 == 0 { continue; }
        w.mobys[id].bangles = mask & !(1 << i);
        let g = &mut ctx.game.rng;
        let a = add_rot(g.randf_signed(90.0, 120.0) * 0.017_453_294, yaw);
        let s = g.randf(2.0, 4.0) * 0.016_666_668;
        let up = g.randf(5.0, 8.0) * 0.016_666_668;
        let v = [a.cos() * s, a.sin() * s, up];
        let m = &w.mobys[id];
        let (class, pos, rot) = (m.o_class, m.pos, m.rot);
        crate::moby::shards::spawn_ball(w, ctx, class, Some(id), pos, rot, 90.0, 15.0, i as u8, v, 0xff);
    }
}

/// The hit's start (both): the record dropped, the wait counted; `None` when refused.
fn hit_of(w: &mut MobyWorld, id: usize) -> Option<crate::moby::hits::HitRecord> {
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x30000 != 0);
    w.clear_hit(id);
    let m = &mut w.mobys[id];
    let mut k = m.pv_s(0x26) as i32;
    crate::moby::countdown(&mut k);
    m.set_pv_s(0x26, k as i16);
    hit
}

fn flash_hit(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_b(0x147, 0xff);
    ai::flash_start(m, 0x140);
}

fn tick(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    ai::flash(m, 0x140);
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = (m.state_ticks as u16 + 1) as i16; }
}

/// 0x0f7e (`0x417498`, module docs).
pub fn update_0f7e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (most, floor) = (w.level23.f7e_health, w.level23.f7e_floor);
    set_up(w, id, most);
    let o = opponent(w);
    let mut hit = hit_of(w, id);
    if hit.is_some() && (w.mobys[id].pv_s(0x26) != 0 || guarded(w, o)) {
        if guarded(w, o) { hurt_sound(w, ctx, o); }
        hit = None;
    }
    if let Some(h) = hit {
        flash_hit(w, id);
        if let Some(src) = h.tmpl.source.filter(|&s| s < w.mobys.len()) { blast_at(w, ctx, src, o); }
        let hp = w.mobys[id].pv_f(0x20);
        if hp <= floor {
            hand_on(w, o, h.tmpl.source, h.tmpl.damage);
            w.mobys[id].set_pv_f(0x20, floor);
        } else {
            w.mobys[id].set_pv_f(0x20, hp - h.tmpl.damage);
        }
        let m = &mut w.mobys[id];
        if m.pv_f(0x20) <= floor { m.set_pv_f(0x20, floor); }
        if h.tmpl.source.and_then(|s| w.mobys.get(s)).map_or(true, |s| s.o_class != SHOT) { w.mobys[id].set_pv_s(0x26, 0x14); }
        if w.mobys[id].bangles != 0 {
            let hp = w.mobys[id].pv_f(0x20);
            let n = if hp <= floor { 10 } else { 10 - (hp / most * 10.0) as i32 };
            pieces(w, id, ctx, n);
        }
    }
    tick(w, id);
}

/// 0x120f (`0x428cb8`, module docs).
pub fn update_120f(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let most = w.level23.t120f_health;
    set_up(w, id, most);
    let o = opponent(w);
    let mut hit = hit_of(w, id);
    if let Some(h) = hit {
        let open = o.is_some_and(|o| matches!(w.mobys[o].state, 0x10 | 0xc));
        let from_hero = h.tmpl.source.is_some() && h.tmpl.source == ctx.hero_moby;
        if !open || w.mobys[id].pv_s(0x26) != 0 || guarded(w, o) || (from_hero && w.level23.hero_guard < 0) {
            if guarded(w, o) { hurt_sound(w, ctx, o); }
            hit = None;
        }
    }
    if let Some(h) = hit {
        let left = w.mobys[id].pv_f(0x20) - h.tmpl.damage;
        w.mobys[id].set_pv_f(0x20, left);
        if 0.0 < left {
            flash_hit(w, id);
            if let Some(src) = h.tmpl.source.filter(|&s| s < w.mobys.len()) { blast_at(w, ctx, src, o); }
            if h.tmpl.source.and_then(|s| w.mobys.get(s)).map_or(true, |s| s.o_class != SHOT) { w.mobys[id].set_pv_s(0x26, 0x14); }
            if w.mobys[id].bangles != 0 {
                let n = 10 - (left / most * 10.0) as i32;
                pieces(w, id, ctx, n);
            }
        }
    }
    tick(w, id);
}
