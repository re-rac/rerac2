//! Joba's arena gladiator **0x0d98** ([`update_d98`]) and the charging brute **0x0df9** ([`update_df9`]) (L11
//! addresses; the shared pieces are crate::moby::classes::level11::fighters').

use crate::hero::physics::{add, atan2, dist2, len, rot_dist, scale, set_len, sub, V3};
use crate::moby::{ai, frames, mode, Ctx, MobyWorld};
use super::super::fighters as maktar;
use super::super::props::{seq, set_seq, wrapped};
use super::arena::{D98, DF9};
use super::fighters::{aim, bolts, bump, claim, flash_reset, give_back, go, grav, in_arena, in_middle, lim, middle, pool_body, round_plates, shadow, spin_out, spot, start_body, target_of, tick, Pivot};

// ---------------------------------------------------------------------------------------------------------------
// 0x0d98

/// Its words: movement `+0x100`, the head record `+0x230`, its frame `+0x2f0`, the head-off record `+0x2b0` (`+0x2b1`
/// linked, `+0x2b2` on), the spot `+0x330`, the spin out's spin `+0x340` (way `+0x1a0`), its target `+0x350` (from
/// `+0x220`), the shield 0x09d8 `+0x354`, its yaw offset `+0x358`, plates `+0x35c`, area `+0x360`, the shield's
/// sparks' wait `+0x364`, its swing's sound once `+0x368`, turn `+0x366` (1 the master's, 2 sparring), found `+0x36a`,
/// delay `+0x36c`, placed `+0x36e`.
mod d98 {
    pub const HEAD_OFF: usize = 0x2b0;
    pub const SPOT: usize = 0x330;
    pub const SPIN: usize = 0x340;
    pub const TARGET: usize = 0x350;
    pub const SHIELD: usize = 0x354;
    pub const OFFSET: usize = 0x358;
    pub const PLATES: usize = 0x35c;
    pub const AREA: usize = 0x360;
    pub const SPARKS: usize = 0x364;
    pub const TURN: usize = 0x366;
    pub const SWUNG: usize = 0x368;
    pub const FOUND: usize = 0x36a;
    pub const DELAY: usize = 0x36c;
    pub const RING: usize = 0x36e;
}

const SHIELD: i16 = maktar::SHIELD;

fn shield(w: &MobyWorld, id: usize) -> Option<usize> {
    ai::link(&w.mobys[id], d98::SHIELD).filter(|&s| w.mobys.get(s).is_some_and(|x| x.state < 0x80 && x.o_class == SHIELD))
}

/// `0x44acc8`: its shield made (class 0x09d8: its update and draw distances, shown, its light; no collision; `+0x3c`
/// carried).
fn make_shield(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.class(SHIELD).is_none() {
        w.mobys[id].set_pv_i(d98::SHIELD, 0);
        return;
    }
    let s = w.create(SHIELD);
    let (ud, dd, light, amb, pos) = { let m = &w.mobys[id]; (m.update_dist, m.draw_dist, m.light, m.ambient, m.pos) };
    let x = &mut w.mobys[s];
    (x.update_dist, x.draw_dist, x.drawn, x.light, x.ambient, x.pos) = (ud, dd, 1, light, amb, pos);
    x.has_collision = false;
    x.set_pv_i(0x3c, 1);
    w.rebuild(s, ctx.host);
    ai::set_link(&mut w.mobys[id], d98::SHIELD, Some(s));
    w.changed = true;
}

/// `0x44ad58(m, p)`: its shield dropped — in view thrown as a tumbling piece (`randvec(0.1, 0.167)` a tick, falling
/// 1/360; crate::moby::classes::fighters::throw_piece, `0x32b468`); the carried one deleted.
fn drop_shield(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(s) = shield(w, id) else { return };
    if w.mobys[id].drawn != 0 {
        let v = ctx.game.rng.rand_vec(0.100_000_01, 0.166_666_67);
        let x = &w.mobys[s];
        let (pos, rot, rows) = (x.pos, x.rot, x.rows);
        let d = sub([x.bsphere[0], x.bsphere[1], x.bsphere[2]], pos);
        let dot = crate::hero::physics::dot;
        let sphere = [dot(d, rows[0]), dot(d, rows[1]), dot(d, rows[2]), x.bsphere[3]];
        maktar::throw_piece(w, id, ctx, SHIELD, pos, rot, v, 1.0 / 360.0, sphere);
    }
    w.delete(s);
    w.mobys[id].set_pv_i(d98::SHIELD, 0);
}

/// Its head knocked off (`+0x2b1` once): its pieces 0x20 thrown at `randvec(1/60, 1/20)` (`0x339740`). Not ported:
/// the head joint shrunk to nothing (the record `+0x2b0` on joint 3, scale 0.0001).
fn head_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_b(d98::HEAD_OFF + 1) != 0 { return; }
    w.mobys[id].set_pv_b(d98::HEAD_OFF + 1, 1);
    w.mobys[id].set_pv_b(d98::HEAD_OFF + 2, 1);
    let v = ctx.game.rng.rand_vec(1.0 / 60.0, 0.05);
    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x20, v, 0xff);
}

/// `0x44b0c8(m, delay, placed)`: also its new shield in its colour.
pub(super) fn start_d98(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, delay: i32, placed: bool) {
    let m = &mut w.mobys[id];
    m.set_pv_s(0x88, 0);
    m.set_pv_s(d98::TURN, 0);
    m.set_pv_s(d98::RING, placed as i16);
    lim(w, id, 0.005_555_556, 0.103_333_34, 0.003_490_659, 0.418_879_1);
    w.mobys[id].set_pv_b(0x6a, 0);
    w.mobys[id].set_pv_b(0x2e, 1);
    if !placed { set_seq(w, id, ctx, 0, 0, 10); }
    start_body(w, id, placed, (4, 2), 1);
    if placed { ctx.host.set_anim(&w.mobys[id], 0, 0, 0); }
    make_shield(w, id, ctx);
    let m = &mut w.mobys[id];
    m.set_pv_s(d98::DELAY, delay as i16);
    m.set_pv_i(0x200, 0);
    flash_reset(w, id);
    let amb = w.mobys[id].ambient;
    if let Some(s) = shield(w, id) { w.mobys[s].ambient = amb; }
}

/// `0x44b2a0`: back to the pool — its turn given back, its shield dropped, its head back.
pub(super) fn pool_d98(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    pool_body(w, id, ctx);
    if w.mobys[id].pv_s(d98::TURN) == 1 { give_back(w, id); }
    w.mobys[id].set_pv_s(d98::TURN, 0);
    w.mobys[id].set_pv_i(d98::TARGET, 0);
    drop_shield(w, id, ctx);
    if w.mobys[id].pv_b(d98::HEAD_OFF + 1) != 0 {
        w.mobys[id].set_pv_b(d98::HEAD_OFF + 1, 0);
        w.mobys[id].set_pv_b(d98::HEAD_OFF + 2, 0);
    }
}

/// `0x44b398(m, no_bolts, at_once)`: dead — its turn given back, its shield dropped, its head knocked off, no
/// collision, out of the target list; its bolts unless `no_bolts`; back to the pool now or after the death fade.
pub(super) fn die_d98(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, no_bolts: bool, at_once: bool) {
    w.mobys[id].set_pv_f(0x20, 0.0);
    if w.mobys[id].pv_s(d98::TURN) == 1 { give_back(w, id); }
    w.mobys[id].set_pv_s(d98::TURN, 0);
    w.mobys[id].set_pv_i(d98::TARGET, 0);
    drop_shield(w, id, ctx);
    head_off(w, id, ctx);
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.mode &= !mode::TARGET;
    if !no_bolts { bolts(w, id, ctx); }
    if at_once { pool_d98(w, id, ctx); } else { super::super::death_fade::start(w, id, ctx, 0.0, Some(pool_d98)); }
}

/// `0x449ed0`: the set-up, its shadow, the hits (the shield takes those it can while it guards or swings: the flag,
/// sparks every 60 ticks), dying (`0x44b398(m, 1, 0)`), the ring's checks, its turn.
fn senses_d98(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use d98::*;
    if w.mobys[id].state == 0 {
        let a = ctx.game.rng.randf(10.0, 30.0).to_radians();
        let neg = ctx.game.rng.randi(2) != 0;
        let m = &mut w.mobys[id];
        m.set_pv_i(0x18, 0x100);
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 16);
        m.set_pv_f(OFFSET, if neg { -a } else { a });
        m.set_pv_f(0x80, 1.3);
        m.set_pv_f(0x84, 1.3);
        m.set_pv_b(0x69, 2);
        m.set_pv_b(0x1e9, 3);
        m.set_pv_f(0x1d4, 0.5);
        m.set_pv_f(0x104, 0.5);
        m.set_pv_f(0x108, 0.5);
        m.set_pv_b(0x60, 16);
        m.set_pv_i(0x98, 7);
        m.set_pv_f(0x8c, 0.4);
        m.set_pv_b(0x1e6, 5);
        m.set_pv_b(0x1e8, 2);
        m.set_pv_f(0x100, 1.3);
        m.set_pv_f(0x10c, 0.5);
        m.set_pv_f(0x110, std::f32::consts::FRAC_PI_2);
        m.set_pv_f(0x130, 0.523_598_85);
        m.set_pv_f(0x134, 0.01);
        m.set_pv_f(0x12c, 1.0 / 180.0);
        m.set_pv_b(0x28, 3);
        m.set_pv_b(0x29, 1);
        m.set_pv_f(0x30, 0.75);
        m.set_pv_s(SPARKS, 0);
        if ctx.game.rng.randi(2) == 0 { w.mobys[id].mode |= mode::MIRROR; }
        w.mobys[id].set_pv_i(SHIELD, 0);
        pool_d98(w, id, ctx);
    }
    let ring = w.mobys[id].pv_s(RING) != 0;
    let on = !ring && w.mobys[id].state != 18;
    shadow(w, id, ctx, on);
    if w.mobys[id].state < 3 { return; }
    w.mobys[id].set_pv_f(0xfc, 1.0);
    let hit = w.hit_message(id).filter(|h| h.tmpl.flags & 0x1_0000 != 0);
    let blocked = hit.is_some() && shield(w, id).is_some() && {
        let m = &w.mobys[id];
        matches!(m.state, 0xf | 4 | 0xe) || m.sub_state == 4
    };
    if blocked {
        w.mobys[id].set_pv_b(0x60, 15);
        ai::take_damage(w, id, ctx, true);
        w.mobys[id].set_pv_b(0x60, 16);
        if let Some(s) = shield(w, id) {
            if w.mobys[id].pv_s(SPARKS) == 0 && w.mobys[id].drawn != 0 {
                let p = w.mobys[s].pos;
                maktar::shield_sparks(ctx, p, 2);
                w.mobys[id].set_pv_s(SPARKS, 60);
            }
        }
    } else {
        ai::take_damage(w, id, ctx, false);
    }
    if w.mobys[id].pv_f(0x20) <= 0.0 && !matches!(w.mobys[id].sub_state, 7 | 8) {
        die_d98(w, id, ctx, true, false);
    }
    if ring {
        let p = w.mobys[id].pos;
        if !in_arena(w, id, ctx, p) && w.mobys[id].sub_state != 8 {
            w.mobys[id].set_pv_f(0x20, 0.0);
            w.mobys[id].set_state(18, 8);
        }
    }
    if 2 <= w.mobys[id].state && w.mobys[id].pv_b(0x2e) == 2 { die_d98(w, id, ctx, true, true); }
    turn_d98(w, id, ctx);
}

/// `0x44a820`: its turn — the master's (kept once had), else a sparring partner: the nearest free gladiator of its
/// group (both then 2, each the other's target), kept while the other still has it.
fn turn_d98(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use d98::*;
    if w.mobys[id].state < 4 {
        w.mobys[id].set_pv_i(TARGET, 0);
        w.mobys[id].set_pv_s(TURN, 0);
        return;
    }
    let has = w.mobys[id].pv_s(TURN) == 1 || {
        let got = claim(w, id);
        if got { w.mobys[id].set_pv_s(TURN, 1); }
        got
    };
    if has {
        let a = w.mobys[id].pv_i(AREA);
        let f = ai::find_target(w, id, ctx, 100.0, 100.0, a, None, false);
        let m = &mut w.mobys[id];
        m.set_pv_s(FOUND, f as i16);
        let t = m.pv_i(0x220);
        m.set_pv_i(TARGET, t);
        return;
    }
    if w.mobys[id].pv_s(TURN) == 2 {
        if let Some(o) = ai::link(&w.mobys[id], TARGET) {
            let x = &w.mobys[o];
            if x.state < 0x80 && x.sub_state > 1 && x.pv_s(TURN) == 2 && ai::link(x, TARGET) == Some(id) {
                w.mobys[id].set_pv_s(FOUND, 2);
                w.mobys[o].set_pv_s(FOUND, 2);
                return;
            }
            w.mobys[id].set_pv_i(TARGET, 0);
            w.mobys[id].set_pv_s(TURN, 0);
            return;
        }
    }
    let g = w.mobys[id].group;
    let members = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
    let mut best = None;
    let mut near = 1000.0;
    for o in members {
        let x = &w.mobys[o];
        if o == id || x.state >= 0x80 || x.state < 3 || x.o_class != D98 || x.pv_s(TURN) != 0 { continue; }
        let d = dist2(w.mobys[id].pos, x.pos);
        if d < near { near = d; best = Some(o); }
    }
    match best {
        Some(o) => {
            for (a, b) in [(id, o), (o, id)] {
                let m = &mut w.mobys[a];
                ai::set_link(m, TARGET, Some(b));
                m.set_pv_s(TURN, 2);
                m.set_pv_s(FOUND, 2);
            }
        }
        None => w.mobys[id].set_pv_s(TURN, 0),
    }
}

/// `0x44a330`: back to its guard — with a shield 4 / sub 2 (sequence 0), without 5 / sub 2 (sequence 1).
fn guard(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if shield(w, id).is_some() {
        w.mobys[id].set_state(4, 2);
        set_seq(w, id, ctx, 0, 0, 10);
    } else {
        w.mobys[id].set_state(5, 2);
        set_seq(w, id, ctx, 1, 0, 10);
    }
}

/// `0x44a3c0`: to the fight (6, sub 3, sequence 2).
fn fight(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    lim(w, id, 0.005_555_556, 0.031_666_666, 0.006_981_318, 0.837_758_2);
    w.mobys[id].set_state(6, 3);
    set_seq(w, id, ctx, 2, 0, 10);
}

/// `0x44a448`: the stand-off (10, sub 4, sequence 6).
fn standoff(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    lim(w, id, 0.005_555_556, 0.066_666_67, 0.003_490_659, 0.418_879_1);
    w.mobys[id].set_state(10, 4);
    set_seq(w, id, ctx, 6, 0, 5);
}

/// `0x44a750`: three gladiators of its group already in the fight's turn (sub 6) with the master's turn.
fn crowded(w: &MobyWorld, id: usize) -> bool {
    let g = w.mobys[id].group;
    usize::try_from(g).ok().and_then(|g| w.groups.get(g)).is_some_and(|v| 3 <= v.iter().filter(|&&o| {
        let x = &w.mobys[o];
        x.state < 0x80 && x.o_class == D98 && x.sub_state == 6 && x.pv_s(d98::TURN) == 1
    }).count())
}

/// `0x44a4d0`: within 4 of its target and facing it within 15°, it attacks: one time in four the taunt (12, sub 5,
/// sequence 7), else the swing (14 with the shield, sequence 9; 13 without, sequence 8; sub 6, from frame 8).
fn attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, t: V3, ring: bool) -> bool {
    if crowded(w, id) { return false; }
    let (_, diff, d) = aim(w, id, t, ring);
    if 4.0 <= d || 0.261_799_42 <= diff { return false; }
    if ctx.game.rng.randi(4) == 0 {
        w.mobys[id].set_state(12, 5);
        set_seq(w, id, ctx, 7, 0, 10);
    } else if shield(w, id).is_some() {
        w.mobys[id].set_state(14, 6);
        set_seq(w, id, ctx, 9, 8, 15);
    } else {
        w.mobys[id].set_state(13, 6);
        set_seq(w, id, ctx, 8, 8, 15);
    }
    true
}

/// `0x44a6a8(m, p)`: how far `p` is to its side (its second row on the way to `p`).
fn side(w: &MobyWorld, id: usize, p: V3) -> f32 {
    let m = &w.mobys[id];
    crate::hero::physics::dot(m.rows[1], set_len(sub(p, m.pos), 1.0))
}

/// `0x44aac0`: the swing's blow, frames 19..31: a sphere on the blade (joint 0 → 1) at the point nearest its target
/// (0.15..0.5 wide by how far along), the sword's own at joint 1 (0.5); its sound 5 on the first that lands.
fn swing(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, t: V3) {
    let ft = ctx.host.frame_time(&w.mobys[id]);
    if !(19.0..=31.0).contains(&ft) { return; }
    let (Some(a), Some(b)) = (ctx.host.joint(&w.mobys[id], 0), ctx.host.joint(&w.mobys[id], 1)) else { return };
    let ab = sub(a, b);
    let l2 = crate::hero::physics::dot(ab, ab);
    let k = if 0.0 < l2 { (crate::hero::physics::dot(sub(t, b), ab) / l2).clamp(0.0, 1.0) } else { 0.0 };
    let p = add(b, scale(ab, k));
    let f = (len(sub(p, b)) / len(ab).max(1e-6)).clamp(0.0, 1.0);
    let dmg = w.mobys[id].pv_b(0x34) as f32;
    let n = ai::attack_sphere(w, id, ctx, f * 0.35 + 0.15, dmg, 1.0, p, 0x1_0001, 0, 1, 0);
    if n != 0 && w.mobys[id].pv_s(d98::SWUNG) == 0 {
        ctx.host.play_sound(&w.mobys[id], 5, 0);
        w.mobys[id].set_pv_s(d98::SWUNG, 1);
    }
    ai::attack_sphere(w, id, ctx, 0.5, dmg, 1.0, b, 0x1_0001, 0, 1, 0);
}

/// The hold in place (`0x3417b0` in the ring, else `0x33fa58`).
fn hold(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, ring: bool) {
    if ring {
        let p = w.mobys[id].pos;
        frames::move_to(w, id, ctx, p);
    } else {
        ai::hold(w, id, ctx);
    }
}

/// 0x0d98 (`0x448fc8`): Maktar's gladiator 0x09ca again (a sword and a shield 0x09d8 at its joint 2) with the ring.
/// Out, it waits its delay and walks in to the middle's polygon (3), then guards (4 / 5, sub 2): it wanders between
/// spots (in the ring 15 out at ±10°, its own height ±4), now and then idling. With a turn (the master's, or sparring
/// with another gladiator) it fights (6 → 7 closing in at its yaw offset round raised plates; 10 the stand-off,
/// side-stepping (8 / 9) to face its target, stepping in to 4; 11 guarding close): within 4 and facing it the taunt
/// (12) or a swing (13 alone, 14 with its shield; [`swing`]), then the taunt if it hurt Ratchet. Hits the shield takes
/// while it guards or swings do less and spark; at 1 health its shield falls, at 0 its head; knocked (sub 7), dead
/// (in the ring one in two the spin out, 18) it fades back to the pool with its bolts.
pub fn update_d98(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use d98::*;
    if w.mobys[id].state == 1 { return; }
    let ring = w.mobys[id].pv_s(RING) != 0;
    if ring && !grav(w) && !matches!(w.mobys[id].state, 17 | 18) { return; }
    senses_d98(w, id, ctx);
    if w.mobys[id].state == 1 { return; }
    let ring = w.mobys[id].pv_s(RING) != 0;
    if w.mobys[id].pv_s(TURN) == 0 {
        w.mobys[id].set_pv_i(TARGET, 0);
        if !matches!(w.mobys[id].sub_state, 0 | 1 | 2 | 7 | 8) { guard(w, id, ctx); }
    }
    let t = target_of(w, id, ctx, TARGET);
    let (angle, _diff, d) = aim(w, id, t, ring);
    let mut flags = 0;
    match w.mobys[id].sub_state {
        2 => {
            if w.mobys[id].first_sub() {
                let mid = middle(w, id, ctx);
                set_seq(w, id, ctx, 3, 0, 10);
                w.mobys[id].timer = 0;
                spot(w, id, ctx, mid, SPOT, ring, if ring { 0.174_532_95 } else { std::f32::consts::FRAC_PI_4 }, Some(4.0));
                w.mobys[id].state_ticks = 10000;
            }
            if w.mobys[id].timer == 1 && 300 < w.mobys[id].state_ticks as u16 {
                match ctx.game.rng.randi(2) {
                    0 => set_seq(w, id, ctx, 1, 0, 10),
                    _ => set_seq(w, id, ctx, 0, 0, 10),
                }
                w.mobys[id].state_ticks = ctx.game.rng.randi(100) as i16;
            }
            if w.mobys[id].pv_s(TURN) != 0 { fight(w, id, ctx); }
            if w.mobys[id].timer == 0 {
                let sp = w.mobys[id].pv_v(SPOT);
                let p = w.mobys[id].pos;
                if (p[0] - sp[0]).powi(2) + (p[1] - sp[1]).powi(2) < 4.0 { w.mobys[id].timer = 1; }
                go(w, id, ctx, sp, ring);
            } else {
                let p = w.mobys[id].pos;
                go(w, id, ctx, p, ring);
            }
        }
        5 => {
            if wrapped(w, id, ctx) { standoff(w, id, ctx); }
        }
        3 => {
            if ring {
                go(w, id, ctx, t, true);
            } else if !round_plates(w, id, ctx, PLATES, 4.7) {
                match ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a) {
                    2 => go(w, id, ctx, t, false),
                    3 => {
                        let off = w.mobys[id].pv_f(OFFSET);
                        ai::move_toward(w, id, ctx, off, t);
                    }
                    _ => {}
                }
            }
        }
        4 => {
            if 14.0 < d {
                fight(w, id, ctx);
            } else if !attack(w, id, ctx, t, ring) {
                let (_, diff, _) = aim(w, id, t, ring);
                if d < 4.0 && diff < 0.261_799_42 {
                    w.mobys[id].set_state(11, -1);
                } else if w.mobys[id].state == 11 && 5.0 < d {
                    w.mobys[id].set_state(10, -1);
                }
            }
        }
        6 => super::fighters::face(w, id, ctx, t, ring),
        7 => {
            flags = ai::flight(w, id, ctx);
            if w.mobys[id].pv_f(0x20) <= 1.0 { drop_shield(w, id, ctx); }
            if w.mobys[id].pv_f(0x20) <= 0.0 && w.mobys[id].pv_b(HEAD_OFF + 1) == 0 {
                head_off(w, id, ctx);
                if ring && ctx.game.rng.randi(2) == 0 {
                    w.mobys[id].set_state(18, 8);
                    return;
                }
            }
            if flags & 1 != 0 || (w.mobys[id].pv_f(0x20) <= 0.0 && 60 < w.mobys[id].state_ticks as u16) {
                w.mobys[id].has_collision = false;
                bolts(w, id, ctx);
                super::super::death_fade::start(w, id, ctx, 0.0, Some(pool_d98));
            }
        }
        _ => {}
    }
    match w.mobys[id].state {
        3 => {
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                set_seq(w, id, ctx, 0, 0, 0);
                w.mobys[id].state_ticks = 0;
            }
            let m = &w.mobys[id];
            if m.timer == 0 && (m.pv_s(DELAY) as i32) < m.state_ticks as u16 as i32 {
                set_seq(w, id, ctx, 3, 0, 10);
                w.mobys[id].timer = 1;
            } else if w.mobys[id].timer == 1 {
                let p = w.mobys[id].pos;
                if in_middle(w, id, ctx, p) { w.mobys[id].set_state(4, 2); }
                let mid = middle(w, id, ctx);
                go(w, id, ctx, mid, false);
            }
        }
        7 => {
            if w.mobys[id].first() { w.mobys[id].state_ticks = 0; }
            let stuck = w.mobys[id].pv_i(0x180);
            if d < 4.0 || 450 < w.mobys[id].state_ticks as u16 || 31 <= stuck { fight(w, id, ctx); }
        }
        6 => {
            if 14.0 < d {
                lim(w, id, 0.005_555_556, 0.103_333_34, 0.003_490_659, 0.418_879_1);
                w.mobys[id].set_state(7, 3);
                set_seq(w, id, ctx, 3, 0, 10);
            } else if d < 4.0 || 31 <= w.mobys[id].pv_i(0x180) {
                standoff(w, id, ctx);
            }
        }
        10 => {
            let ft = ctx.host.frame_time(&w.mobys[id]);
            let x = side(w, id, t);
            if 0.01 < x {
                w.mobys[id].set_state(9, -1);
                set_seq(w, id, ctx, 5, 0, 10);
                super::fighters::face(w, id, ctx, t, ring);
            } else if x < -0.01 {
                w.mobys[id].set_state(8, -1);
                set_seq(w, id, ctx, 4, 0, 10);
                super::fighters::face(w, id, ctx, t, ring);
            } else if d < 4.0 {
                w.mobys[id].set_state(11, -1);
            } else if ft < 2.0 {
                lim(w, id, 0.005_555_556, 0.033_333_335, 0.003_490_659, 0.418_879_1);
                go(w, id, ctx, t, ring);
            } else if 10.0 < ft && ft < 17.0 {
                super::fighters::face(w, id, ctx, t, ring);
            } else if ft < 20.0 {
                go(w, id, ctx, t, ring);
            } else {
                super::fighters::face(w, id, ctx, t, ring);
            }
        }
        s @ (8 | 9) => {
            let x = side(w, id, t);
            let done = if s == 8 { -0.01 < x } else { x < 0.01 };
            if done {
                w.mobys[id].set_state(10, -1);
                set_seq(w, id, ctx, 6, 0, 10);
                hold(w, id, ctx, ring);
            } else if 5.0 < ctx.host.frame_time(&w.mobys[id]) {
                let keep = w.mobys[id].pv_f(0x130);
                w.mobys[id].set_pv_f(0x130, 0.017_453_294);
                super::fighters::face(w, id, ctx, t, ring);
                w.mobys[id].set_pv_f(0x130, keep);
            }
        }
        s @ (11 | 15 | 16) => {
            if s == 11 {
                if w.mobys[id].first() { set_seq(w, id, ctx, 1, 0, 10); }
                super::fighters::face(w, id, ctx, t, ring);
            }
            if flags & 2 != 0 { guard(w, id, ctx); }
        }
        13 | 14 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].set_pv_s(SWUNG, 0);
                if ctx.hero.damage.hit_by == Some(id) {
                    w.mobys[id].set_state(12, 5);
                    set_seq(w, id, ctx, 7, 0, 10);
                } else {
                    guard(w, id, ctx);
                }
            }
            swing(w, id, ctx, t);
        }
        18 => {
            if spin_out(w, id, ctx, 0x1a0, SPIN, RING, Some(13), 0x12c, Pivot::Up(0x30)) {
                w.mobys[id].has_collision = false;
                bolts(w, id, ctx);
                super::super::death_fade::start(w, id, ctx, 0.0, Some(pool_d98));
            }
        }
        _ => {}
    }
    let _ = angle;
    // `0x44a270`: the sparks' wait, its shield at its joint 2, the ticks, the head (`+0x230`, list 4, 3.5), its blow
    // on what it touches (1.5).
    let mut s = w.mobys[id].pv_s(SPARKS) as i32;
    crate::moby::countdown(&mut s);
    w.mobys[id].set_pv_s(SPARKS, s as i16);
    if let Some(s) = shield(w, id) {
        if let Some((r, t)) = ctx.host.joint_matrix(&w.mobys[id], 2) {
            ctx.host.advance(&w.mobys[s]);
            let hidden = w.mobys[id].mode & 1 != 0;
            let pm = &mut w.mobys[s];
            pm.pos = t;
            pm.rows = r.map(|row| set_len(row, 1.0));
            pm.rot = crate::moby::attach::angles_of(&pm.rows);
            pm.mode = if hidden { pm.mode | 0x41 } else { pm.mode & 0xffbe };
            pm.mode |= mode::NO_MATRIX | mode::KEEP_ROWS;
            pm.mode &= !mode::NO_UPDATE;
            w.rebuild(s, ctx.host);
        }
    }
    tick(w, id);
    crate::moby::joint_aim::head(w, ctx, id, 0x230, 4, 3.5);
    if w.mobys[id].state != 1 { bump(w, id, ctx, 1.5); }
}

// ---------------------------------------------------------------------------------------------------------------
// 0x0df9

/// Its words: movement `+0x100` (its step `+0x124`), the spot `+0x230`, its target `+0x240` (from `+0x220`), plates
/// `+0x244`, its turn spring `+0x248`, area `+0x24c`, the moby it charges at `+0x250`, found `+0x254`, turn `+0x256`,
/// delay `+0x258`; the blow's damage to Ratchet `+0x34` (the punch) / `+0x35` (the charge).
mod df9 {
    pub const SPOT: usize = 0x230;
    pub const TARGET: usize = 0x240;
    pub const PLATES: usize = 0x244;
    pub const SPRING: usize = 0x248;
    pub const AREA: usize = 0x24c;
    pub const AT: usize = 0x250;
    pub const FOUND: usize = 0x254;
    pub const TURN: usize = 0x256;
    pub const DELAY: usize = 0x258;
}

/// `0x457e90(m, delay)` (out of a spawner only).
pub(super) fn start_df9(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, delay: i32) {
    let coll = w.class(DF9).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    let life = m.pv_s(0x24) as f32;
    m.set_pv_f(0x20, life);
    m.set_pv_s(df9::TURN, 0);
    m.set_pv_s(0x88, 0);
    lim(w, id, 0.004_444_444_6, 0.133_333_34, 0.001_163_553, 0.104_719_77);
    let m = &mut w.mobys[id];
    m.set_state(3, 0);
    m.has_collision = coll;
    m.set_pv_b(0x6a, 0);
    m.set_pv_b(0x2e, 1);
    m.drawn = 1;
    m.drops = 0;
    m.mode = (m.mode | mode::TARGET) & 0xffb8;
    m.set_pv_s(df9::DELAY, delay as i16);
    m.set_pv_i(0x200, 0);
    flash_reset(w, id);
    let _ = ctx;
    w.changed = true;
}

/// `0x457f98`: back to the pool — its turn given back.
pub fn pool_df9(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_s(df9::TURN) == 1 { give_back(w, id); }
    w.mobys[id].set_pv_s(df9::TURN, 0);
    w.mobys[id].set_pv_i(df9::TARGET, 0);
    pool_body(w, id, ctx);
}

/// `0x457850`: the set-up; drawn, its damage and shadow (range 0x16 within 24); its turn (`0x457dd0`).
fn senses_df9(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_b(0x48, 15);
        m.set_pv_b(0x4a, 12);
        for o in [0x80, 0x84, 0x108] { m.set_pv_f(o, 0.5); }
        m.set_pv_s(df9::TURN, 0);
        m.set_pv_b(0x60, 13);
        m.set_pv_i(0x98, 4);
        m.set_pv_f(0x8c, 0.4);
        m.set_pv_f(0x100, 1.0);
        m.set_pv_f(0x104, 0.25);
        m.set_pv_f(0x10c, 0.5);
        m.set_pv_f(0x110, std::f32::consts::FRAC_PI_2);
        m.set_pv_f(0x130, 0.523_598_85);
        m.set_pv_f(0x134, 0.01);
        m.set_pv_f(0x12c, 1.0 / 180.0);
        m.set_pv_b(0x28, 3);
        m.set_pv_f(0x30, 0.75);
        m.set_pv_b(0x29, 1);
        pool_df9(w, id, ctx);
    }
    if w.mobys[id].drawn != 0 {
        if 3 <= w.mobys[id].state && ai::take_damage(w, id, ctx, false) == 5 {
            w.mobys[id].set_pv_i(0xe0, 0);
            w.mobys[id].set_pv_f(0xe4, 0.837_758_2);
        }
        crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    }
    if 2 <= w.mobys[id].state && w.mobys[id].pv_b(0x2e) == 2 { pool_df9(w, id, ctx); }
    super::fighters::turn(w, id, ctx, df9::TURN, df9::TARGET, df9::AREA, df9::FOUND, 0x220, false);
}

/// `0x458390`: after a move — none to fight: guard (4, sub 2); within 3 of its target, or out of view: the walk (7);
/// past 10 or one time in four: the charge (9); else the walk.
fn next_df9(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(t) = ai::link(&w.mobys[id], df9::TARGET).and_then(|t| w.mobys.get(t)).map(|t| t.pos) else {
        w.mobys[id].set_state(4, 2);
        return;
    };
    let p = w.mobys[id].pos;
    let d = dist2(p, t);
    if d < 3.0 || ctx.view.is_some_and(|v| v.culled(128.0, [p[0], p[1], p[2], 0.5])) {
        w.mobys[id].set_state(7, 3);
    } else if 10.0 < d || ctx.game.rng.randi(4) == 0 {
        w.mobys[id].set_state(9, 3);
    } else {
        w.mobys[id].set_state(7, 3);
    }
}

/// `0x4584a0(m, at, r, to Ratchet, to others, push, push others, up, up others)`: the blow's sphere `r` at `at`: Ratchet
/// (with a damage) and every other moby but a 0x0df9 it touches is hit (flags 0x10001) away from it across `push`
/// with `up` (the no-limit 5627.925).
#[allow(clippy::too_many_arguments)]
fn blow(w: &mut MobyWorld, id: usize, ctx: &Ctx, at: V3, r: f32, hero: (f32, f32, f32), other: (f32, f32, f32)) {
    let Some(sc) = ctx.scene else { return };
    let ids = crate::collision::mobys::coll_sphere_mobys(sc, at, r, crate::collision::QueryFlags(0), Some(id));
    let from = w.mobys[id].pos;
    for k in ids {
        let Some(x) = w.mobys.get(k) else { continue };
        let is_hero = Some(k) == ctx.hero_moby;
        let (dmg, push, up) = if is_hero { hero } else { other };
        if is_hero && dmg <= 0.0 { continue; }
        if !is_hero && x.o_class == DF9 { continue; }
        let p = x.pos;
        let v = set_len([p[0] - from[0], p[1] - from[1], 0.0], push);
        let t = crate::moby::hits::HitTemplate::new(dmg, Some(id), 0x1_0001, [v[0], v[1], up, 5627.925]);
        w.deliver_hit_any(k, &t, [p[0], p[1], p[2], 1.0]);
    }
}

/// The punch and charge's ranges (`$gp+0x2ddc..+0x2dfc`): the sphere 0.8 at joint 1; the swing's push 3 (others 0,
/// up 7 / 0), the charge's 5 (others 2, up 7 / 0), its hit's 5 (2; up 10 / 3).
const DF9_R: f32 = 0.8;

/// `0x457ac8(m, p)`: its walk by its sequence's own step (`0x2b6d10`, per frame, along its first row), turned to `p`
/// (with another sequence coming: the plain walk). Not ported: the footsteps' dust at frames 5 and 15.
fn stride(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, p: V3) {
    let a = ctx.host.anim(&w.mobys[id]);
    if a.is_none_or(|a| a.seq_a != a.seq_b) {
        go(w, id, ctx, p, false);
        return;
    }
    let ft = ctx.host.frame_time(&w.mobys[id]);
    let i = (ft as i32).clamp(0, STEP.len() as i32 - 2) as usize;
    let k = STEP[i] + (STEP[i + 1] - STEP[i]) * (ft - i as f32);
    let m = &mut w.mobys[id];
    m.pos = add(m.pos, scale(m.rows[0], k));
    let q = w.mobys[id].pos;
    ai::turn_in_place(w, id, ctx, atan2(p[0] - q[0], p[1] - q[1]));
}

/// `0x2b6d10`: the walk's step per frame of its sequence 3.
const STEP: [f32; 22] = [0.075, 0.075, 0.1, 0.1, 0.075, 0.05, 0.075, 0.075, 0.075, 0.075, 0.075, 0.1, 0.1, 0.1, 0.075, 0.03, 0.03, 0.03, 0.03, 0.03, 0.05, 0.0];

/// The fighter ahead to charge at (every 64th frame, on its own): the first of the master's group (but a 0x0df9)
/// within 30° of its facing and √20 across; let go once it is gone or pooled.
fn charge_at(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    use df9::AT;
    match ai::link(&w.mobys[id], AT) {
        None if ctx.game.frame & 0x3f == id as i32 & 0x3f => {
            let Some(ms) = super::fighters::master(w, id) else { return };
            let g = usize::try_from(w.mobys[ms].pv_i(super::arena::pv::ENEMIES)).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
            let m = &w.mobys[id];
            let found = g.into_iter().find(|&k| {
                let x = &w.mobys[k];
                x.o_class != DF9 && rot_dist(m.rot[2], atan2(x.pos[0] - m.pos[0], x.pos[1] - m.pos[1])) < 0.523_598_85
                    && (x.pos[0] - m.pos[0]).powi(2) + (x.pos[1] - m.pos[1]).powi(2) < 20.0
            });
            ai::set_link(&mut w.mobys[id], AT, found);
        }
        Some(k) if w.mobys[k].state >= 0x80 || w.mobys[k].state < 2 => ai::set_link(&mut w.mobys[id], AT, None),
        _ => {}
    }
}

/// 0x0df9 (`0x456970`): a brute on the framework (react `+0x50`, movement `+0x100`). Out, it waits its delay and
/// walks in to the middle's polygon (3), then wanders between spots 6..10 from the middle at a random pace (sub 2,
/// idling now and then); with a turn (the master's) it squares up (5), then walks at its target by its sequence's step
/// (7, round raised plates; [`stride`]), swinging the fighters ahead of it aside (the sphere at its joint 1 hits the
/// others for 100: [`blow`]), punches within 2.7 facing it within 20° (11: its joint 1 hits for `+0x34` at frames
/// 10..15), runs at it (6) past 15, and charges (9: Ratchet hit for `+0x35`, its turn springing to him; past 22° the
/// stop, 8; when it hurt him the hit, 10). Knocked (sub 4), dead: its bolts and the death fade into its pool.
pub fn update_df9(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use df9::*;
    if w.mobys[id].state == 1 { return; }
    senses_df9(w, id, ctx);
    if w.mobys[id].state == 1 { return; }
    if w.mobys[id].pv_s(TURN) == 0 {
        w.mobys[id].set_pv_i(TARGET, 0);
        if w.mobys[id].sub_state == 3 { w.mobys[id].set_state(4, 2); }
    }
    let ft = ctx.host.frame_time(&w.mobys[id]);
    let t = target_of(w, id, ctx, TARGET);
    let p = w.mobys[id].pos;
    let heading = atan2(t[0] - p[0], t[1] - p[1]);
    let delta = rot_dist(w.mobys[id].rot[2], heading);
    let d = dist2(p, t);
    match w.mobys[id].sub_state {
        2 => {
            if w.mobys[id].first_sub() {
                let s = ctx.game.rng.randf(0.5, 1.25);
                ctx.host.set_anim_speed(&w.mobys[id], s);
                lim(w, id, 0.002_222_222_3, 0.066_666_67, 0.001_163_553, 0.104_719_77);
                set_seq(w, id, ctx, 2, 0, 10);
                w.mobys[id].timer = 0;
                let mid = middle(w, id, ctx);
                spot(w, id, ctx, mid, SPOT, false, std::f32::consts::FRAC_PI_4, None);
                w.mobys[id].state_ticks = 10000;
            }
            if w.mobys[id].timer == 1 && 301 <= w.mobys[id].state_ticks as u16 {
                set_seq(w, id, ctx, 0, 0, 10);
                w.mobys[id].state_ticks = ctx.game.rng.randi(100) as i16;
            }
            if w.mobys[id].pv_s(TURN) != 0 {
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                w.mobys[id].set_state(5, 3);
            }
            let sp = w.mobys[id].pv_v(SPOT);
            if w.mobys[id].timer == 0 && dist2(w.mobys[id].pos, sp) < 0.5 {
                w.mobys[id].timer = 1;
            } else {
                go(w, id, ctx, sp, false);
            }
        }
        4 => {
            let f = ai::flight(w, id, ctx);
            if f & 1 != 0 {
                bolts(w, id, ctx);
                super::super::death_fade::start(w, id, ctx, 0.0, Some(pool_df9));
                return;
            } else if f & 2 != 0 {
                next_df9(w, id, ctx);
            }
        }
        _ => {}
    }
    match w.mobys[id].state {
        3 => {
            if w.mobys[id].first() {
                w.mobys[id].timer = 0;
                set_seq(w, id, ctx, 0, 0, 0);
                w.mobys[id].state_ticks = 0;
            }
            let m = &w.mobys[id];
            if m.timer == 0 && (m.pv_s(DELAY) as i32) < m.state_ticks as u16 as i32 {
                set_seq(w, id, ctx, 3, 0, 10);
                w.mobys[id].timer = 1;
            } else if w.mobys[id].timer == 1 {
                if in_middle(w, id, ctx, p) { w.mobys[id].set_state(4, 2); }
                let mid = middle(w, id, ctx);
                go(w, id, ctx, mid, false);
            }
        }
        5 => {
            if w.mobys[id].first() {
                let s = ctx.game.rng.randi(2) as u8 + 1;
                set_seq(w, id, ctx, s, 0, 10);
            }
            ai::turn_to(&mut w.mobys[id], heading, 0.001, 0.01, 0.1, SPRING);
            if wrapped(w, id, ctx) { next_df9(w, id, ctx); }
        }
        6 => {
            if w.mobys[id].first() {
                lim(w, id, 0.002_222_222_3, 0.066_666_67, 0.001_163_553, 0.104_719_77);
                set_seq(w, id, ctx, 2, 0, 10);
            }
            go(w, id, ctx, t, false);
            if 15.0 < d || d < 2.7 { next_df9(w, id, ctx); }
        }
        7 => {
            if w.mobys[id].first() {
                lim(w, id, 0.003_333_333_4, 0.100_000_01, 0.001_163_553, 0.104_719_77);
                set_seq(w, id, ctx, 3, 0, 10);
                w.mobys[id].state_ticks = 0;
            }
            charge_at(w, id, ctx);
            if !round_plates(w, id, ctx, PLATES, 4.8) {
                let to = ai::link(&w.mobys[id], AT).map_or(t, |k| w.mobys[k].pos);
                stride(w, id, ctx, to);
            }
            if let Some(j) = ctx.host.joint(&w.mobys[id], 1) { blow(w, id, ctx, j, DF9_R, (0.0, 3.0, 7.0), (100.0, 0.0, 0.0)); }
            if d < 2.7 && delta < 0.349_065_9 { w.mobys[id].set_state(11, -1); }
            if 300 < w.mobys[id].state_ticks as u16 { w.mobys[id].set_state(6, -1); }
        }
        9 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 4, 0, 10);
                w.mobys[id].set_pv_f(0x124, 0.006_944_444_5);
                lim(w, id, 0.006_666_667, 0.200_000_02, 0.000_218_166_18, 0.006_399_541_6);
            }
            charge_at(w, id, ctx);
            let to = ai::link(&w.mobys[id], AT).map_or(t, |k| w.mobys[k].pos);
            go(w, id, ctx, to, false);
            let q = w.mobys[id].pos;
            ai::turn_to(&mut w.mobys[id], atan2(to[0] - q[0], to[1] - q[1]), 0.001, 0.01, 0.1, SPRING);
            let dmg = w.mobys[id].pv_b(0x35) as f32;
            if let Some(j) = ctx.host.joint(&w.mobys[id], 1) { blow(w, id, ctx, j, DF9_R, (dmg, 5.0, 7.0), (100.0, 2.0, 0.0)); }
            if ctx.hero.damage.hit_by == Some(id) { w.mobys[id].set_state(10, -1); }
            if 0.383_972_47 < delta { w.mobys[id].set_state(8, -1); }
        }
        10 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 5, 0, 3);
                lim(w, id, 0.004_166_667, 0.033_333_335, 0.000_436_332_37, 0.013_089_972);
            }
            let m = &w.mobys[id];
            let ahead = add(m.pos, [4.0 * m.rot[2].cos(), 4.0 * m.rot[2].sin(), 0.0]);
            go(w, id, ctx, ahead, false);
            let dmg = w.mobys[id].pv_b(0x35) as f32;
            if let Some(j) = ctx.host.joint(&w.mobys[id], 1) { blow(w, id, ctx, j, DF9_R, (dmg, 5.0, 10.0), (100.0, 2.0, 3.0)); }
            if wrapped(w, id, ctx) { w.mobys[id].set_state(8, -1); }
        }
        8 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 4, 0, 10); }
            let m = &w.mobys[id];
            let ahead = add(m.pos, [4.0 * m.rot[2].cos(), 4.0 * m.rot[2].sin(), 0.0]);
            go(w, id, ctx, ahead, false);
            ai::turn_to(&mut w.mobys[id], heading, 0.001, 0.01, 0.1, SPRING);
            if let Some(j) = ctx.host.joint(&w.mobys[id], 1) { blow(w, id, ctx, j, DF9_R, (0.0, 3.0, 7.0), (100.0, 0.0, 0.0)); }
            if wrapped(w, id, ctx) { next_df9(w, id, ctx); }
        }
        11 => {
            if w.mobys[id].first() { set_seq(w, id, ctx, 6, 0, 5); }
            if seq(w, id, ctx) == 6 && (10.0..=15.0).contains(&ft) {
                let dmg = w.mobys[id].pv_b(0x34) as f32;
                ai::attack_joint(w, id, ctx, 1.0, dmg, 1.0, 1, 1, 0, 1, 0);
            }
            if wrapped(w, id, ctx) {
                if d < 2.7 && delta < 0.349_065_9 { w.mobys[id].set_state(11, -1); } else { next_df9(w, id, ctx); }
            }
        }
        12 => {
            if wrapped(w, id, ctx) { next_df9(w, id, ctx); }
        }
        _ => {}
    }
    // `0x457a68`: the ticks, its blow on what it touches (1.6).
    tick(w, id);
    bump(w, id, ctx, 1.6);
}
