//! The Jamming Array's bladed walker, class **0x0c8e** ([`update_0c8e`], `0x3dcd28`, 14 placed; the decomp's
//! `func_L26_003DB818`; its head `0x3dd328`, tail `0x3dd558`): an enemy on the framework (crate::moby::ai) that walks
//! the gravity sphere (crate::moby::frames) with two spinning parts 0x0aa2 at its joints 0 and 2 (`+0x270` /
//! `+0x274`, the 0x0b0b's: crate::moby::classes::enemies::make_parts). It does nothing while the sphere is off
//! (`0x1a8ff0`).
//!
//! Its pvars: the health `+0x20`, the reaction block `+0x50` (its velocity `+0xd0`), movement `+0x100`, target
//! `+0x1f0`, its area `+0x210`, the timers `+0x220` / `+0x222`, seen `+0x224`, alarmed `+0x226`, its frame's transpose
//! `+0x230` and down `+0x260` (as its head found them), the parts' spin `+0x27c`.
//!
//! **The head** ([`head`], `0x3dd328`). The set-up (state 0): the reaction (`+0x98` 3, `+0x80` 1, `+0x84` 0.8, `+0x60`
//! 7, `+0x8c` 0.15, the fall height `+0x90` 1.5 under its height from the sphere's centre), movement (radius 1, height
//! 0.5, the weave ±15° (`$gp+0x1598`, its side random) over 30..90 ticks), `+0x2e` 1, mode | 0x100 (its rows are its
//! own), health 3 unless placed with some, its parts, then state 1 (sub 0, sequence 0). Each tick: its shadow within 24
//! of the camera (range 0x16); the frame's damage (a hurt alarms its group's 0x0c8e, `0x3dd638`); its target (Ratchet
//! within 16, in its area) into `+0x224`; its frame and down kept; with `+0x2e` 2 its parts go and so does it.
//!
//! **Subs** (`+0x95`): 0 calm — seeing him it heads for him (state 5; 4 first from state 1), alarmed it watches (3,
//! sub 1; the idle timer `rand_range(480, 600)` when under 300). 1 watching — seeing him, the chase; the timer out at a
//! sequence's end, calm (2, sub 0). 2 fighting — him gone (not mid-strike), watching (timer `rand_range(580, 600)`).
//! 3 the knock (the framework's flight): over or no health, its death — its parts go, its bolts (3), its sound 13, a
//! blast 1 up its row 2 (size 0.8; 25 / 20 / 20, 20 puffs, 45 streaks), its pieces (3..6 across and up, mask 0x3f8)
//! flung at 1/12 its knock's velocity, deleted; its flight's end (or the reaction's phase past 1), the fight (6, sub
//! 2).
//!
//! **States**: 2 idle (sequence 2). 3 watching Ratchet (crate::moby::frames::face_point), sequences 5 / 6 / 10 / 9 at
//! random (`$gp+0x1588`), a new one at each end. 4 alerted (4) → 5. 5 the chase (8): the frames' move to its target;
//! within 2.5 facing it within 10° ([`yaw_to`] in the frame of `+0x230`) → 6. 6 the strike (9 from frame 8): at its
//! frames 18..24 (unblended) a hit sphere at each part (radius 1/3, damage `+0x34`, flags 0x4000001) → 5 at its end.
//!
//! **The tail** (`0x3dd558`): the timers, the parts' spin (`+0x27c`, 12° a tick fighting, else 2/3°) with them placed
//! at their joints, the state's ticks.
//!
//! Not ported: the parts' sparks while fighting (`0x3e02b8`, one in six a tick, table `0x1a8f20`), the draw registered
//! once a frame for its group (`0x3e0798`, `+0x278`).

use crate::gravity::to_local;
use crate::hero::physics::{add, add_rot, atan2, set_len, sub, V3};
use crate::moby::classes::enemies::{delete_parts, make_parts, parts, spin_parts};
use crate::moby::classes::props::{seq, wrapped};
use crate::moby::{ai, frames, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0c8e;
/// The target block's point.
const TARGET: usize = 0x1f0;
/// Its frame's transpose (`+0x230`, three rows) and down (`+0x260`).
const FRAME: usize = 0x230;
const DOWN: usize = 0x260;
/// The parts.
const PARTS: usize = 0x270;
/// `$gp+0x1580` (`0x1b0570`): the strike's reach.
const REACH: f32 = 2.5;
/// `0x1b0578`: the watching sequences.
const LOOKS: [u8; 4] = [5, 6, 10, 9];

/// `0x325890(m, p, t)` (the decomp's `func_L26_00324598`): the yaw of `p` in the frame whose transpose is `t`
/// (`0x325828`), `t` here the rows `+0x230` held (their transpose is the rows themselves).
pub fn yaw_to(m: &crate::moby::Moby, p: V3, rows: &[V3; 3]) -> f32 {
    let l = to_local(sub(p, m.pos), rows);
    atan2(l[0], l[1])
}

/// The rows as `+0x230` holds them (`0x2efc00(+0x230, rows)`: stored transposed, read back here as rows).
fn held_rows(m: &crate::moby::Moby) -> [V3; 3] {
    let t = [m.pv_v(FRAME), m.pv_v(FRAME + 0x10), m.pv_v(FRAME + 0x20)];
    std::array::from_fn(|r| [t[0][r], t[1][r], t[2][r]])
}

fn group(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// `0x3dd638`: its group's living 0x0c8e alarmed (`+0x226`).
fn alarm(w: &mut MobyWorld, id: usize) {
    for m in group(w, id) {
        let x = &mut w.mobys[m];
        if x.state < 0x80 && x.o_class == CLASS { x.set_pv_s(0x226, 1); }
    }
}

/// `0x3dd328` (module docs). Returns whether it is to go.
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool {
    if w.mobys[id].state == 0 {
        let h = ctx.hero.gravity.height(w.mobys[id].pos);
        let side = ctx.game.rng.randi(2) != 0;
        let m = &mut w.mobys[id];
        m.set_pv_i(0x98, 3);
        m.set_pv_f(0x80, 1.0);
        m.set_pv_f(0x84, f32::from_bits(0x3f4c_cccd));
        m.set_pv_b(0x60, 7);
        m.set_pv_f(0x8c, f32::from_bits(0x3e19_999a));
        m.set_pv_f(0x90, h - 1.5);
        m.set_pv_f(0x100, 1.0);
        m.set_pv_f(0x104, 0.5);
        // `$gp+0x1598` (15) degrees, `$gp+0x15a0` / `0x1b0594` (30 / 90) ticks.
        let weave = 15.0 * f32::from_bits(0x3c8e_fa36);
        m.set_pv_f(0x1dc, if side { -weave } else { weave });
        m.set_pv_s(0x1e2, 30);
        m.set_pv_s(0x1e4, 90);
        m.set_pv_b(0x2e, 1);
        m.mode |= crate::moby::mode::KEEP_ROWS;
        if m.pv_f(0x20) == 0.0 {
            m.set_pv_s(0x24, 3);
            m.set_pv_f(0x20, 3.0);
        }
        make_parts(w, id, id, PARTS, ctx);
        w.mobys[id].set_state(1, 0);
        if seq(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0); }
    }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    if ai::take_damage(w, id, ctx, false) != 0 { alarm(w, id); }
    let area = w.mobys[id].pv_i(0x210);
    let seen = ai::find_target(w, id, ctx, 16.0, 0.0, area, None, false);
    let down = ctx.hero.gravity.down_at(w.mobys[id].pos);
    let m = &mut w.mobys[id];
    m.set_pv_s(0x224, seen as i16);
    let r = m.rows;
    for k in 0..3 { m.set_pv_v(FRAME + 0x10 * k, [r[0][k], r[1][k], r[2][k]]); }
    m.set_pv_v(DOWN, if ctx.hero.gravity.on { down } else { [0.0, 0.0, -1.0] });
    if m.pv_b(0x2e) == 2 {
        delete_parts(w, id, PARTS);
        return true;
    }
    false
}

/// Sub 3's end: its death (module docs).
fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    delete_parts(w, id, PARTS);
    crate::moby::bolts::drop(w, id, 0, 3, ctx);
    let m = &w.mobys[id];
    let at = add(m.rows[2], m.pos);
    ctx.host.play_sound(&w.mobys[id], 0xd, 0);
    let b = crate::moby::blast::Blast { size: f32::from_bits(0x3f4c_cccd), bursts: [25, 20, 20], puffs: 20, streaks: 45, light: false, shake: false };
    crate::moby::blast::burst(w, ctx, at, &b);
    let m = &mut w.mobys[id];
    m.rot = crate::moby::attach::angles_of(&m.rows);
    let v = set_len(m.pv_v(0xd0), f32::from_bits(0x3daa_aaab));
    crate::moby::shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x3f8, v, 0xff);
    w.delete(id);
}

/// Its sequence `s` (blend `b`, from frame `f`) on entering a state, unless it plays.
fn enter_seq(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, s: u8, f: u8, b: u8) {
    if w.mobys[id].first() && seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, f, b); }
}

/// 0x0c8e (module docs).
pub fn update_0c8e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !ctx.hero.gravity.on { return; }
    if head(w, id, ctx) {
        w.delete(id);
        return;
    }
    let m = &mut w.mobys[id];
    match m.sub_state {
        0 => {
            if m.pv_s(0x224) == 1 {
                let s = if m.state == 1 { 4 } else { 5 };
                m.set_state(s, 2);
            } else if m.pv_s(0x226) != 0 {
                m.set_state(3, 1);
                m.set_pv_s(0x226, 0);
                if m.pv_s(0x220) < 300 { m.set_pv_s(0x220, ctx.game.rng.rand_range(480, 600) as i16); }
            }
        }
        1 => {
            if m.pv_s(0x224) == 1 {
                m.set_state(5, 2);
            } else if m.pv_s(0x220) == 0 && wrapped(w, id, ctx) {
                w.mobys[id].set_state(2, 0);
            }
        }
        2 => {
            if m.pv_s(0x224) != 1 && m.state != 6 {
                m.set_state(3, 1);
                m.set_pv_s(0x220, ctx.game.rng.rand_range(580, 600) as i16);
            }
        }
        3 => {
            let flags = ai::flight(w, id, ctx);
            if flags & 1 != 0 || w.mobys[id].pv_f(0x20) <= 0.0 {
                die(w, id, ctx);
                return;
            }
            if flags & 2 != 0 || 1 < w.mobys[id].pv_b(0x6a) {
                let t = ctx.game.rng.rand_range(580, 600);
                let m = &mut w.mobys[id];
                m.set_state(6, 2);
                m.set_pv_s(0x220, t as i16);
            }
        }
        _ => {}
    }
    match w.mobys[id].state {
        2 => enter_seq(w, id, ctx, 2, 0, 20),
        3 => {
            if w.mobys[id].first() {
                let s = LOOKS[ctx.game.rng.randi(4) as usize];
                if seq(w, id, ctx) != s {
                    let s = LOOKS[ctx.game.rng.randi(4) as usize];
                    ctx.host.set_anim(&w.mobys[id], s, 0, 20);
                }
            }
            let p = ctx.hero.pos;
            frames::face_point(w, id, p);
            if wrapped(w, id, ctx) {
                let mut next = LOOKS[ctx.game.rng.randi(4) as usize] as i32;
                if next == seq(w, id, ctx) as i32 { next = (next + ctx.game.rng.randi(3)) % 4; }
                if seq(w, id, ctx) as i32 != next { ctx.host.set_anim(&w.mobys[id], next as u8, 0, 20); }
            }
        }
        4 => {
            enter_seq(w, id, ctx, 4, 0, 10);
            if wrapped(w, id, ctx) { w.mobys[id].set_state(5, -1); }
        }
        5 => {
            enter_seq(w, id, ctx, 8, 0, 20);
            let goal = w.mobys[id].pv_v(TARGET);
            frames::move_to(w, id, ctx, goal);
            let m = &w.mobys[id];
            if m.pv_f(0x1cc) < REACH && yaw_to(m, goal, &held_rows(m)).abs() < f32::from_bits(0x3e32_b8c4) {
                w.mobys[id].set_state(6, -1);
            }
        }
        6 => {
            enter_seq(w, id, ctx, 9, 8, 20);
            if wrapped(w, id, ctx) { w.mobys[id].set_state(5, -1); }
            let t = ctx.host.frame_time(&w.mobys[id]);
            let even = ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == a.seq_b);
            if even && (18.0..=24.0).contains(&t) {
                let dmg = w.mobys[id].pv_b(0x34) as f32;
                for p in parts(w, id, PARTS).into_iter().flatten() {
                    let at = w.mobys[p].pos;
                    ai::attack_sphere(w, id, ctx, f32::from_bits(0x3eaa_7efa), dmg, 1.0, at, 0x400_0001, 0, 1, 0);
                }
            }
        }
        _ => {}
    }
    tail(w, id, ctx);
}

/// `0x3dd558` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    for o in [0x220, 0x222] {
        let mut t = m.pv_s(o) as i32;
        crate::moby::countdown(&mut t);
        m.set_pv_s(o, t as i16);
    }
    let step = if m.sub_state == 2 { f32::from_bits(0x3e56_7752) } else { f32::from_bits(0x3c3e_a2f4) };
    let a = add_rot(m.pv_f(0x27c), step);
    m.set_pv_f(0x27c, a);
    spin_parts(w, id, id, PARTS, ctx, a);
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    w.changed = true;
}
