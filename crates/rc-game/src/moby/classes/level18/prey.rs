//! The breeders' prey **0x0ef4** (L18 `0x3f90e0`, 11 placed; the decomp's `func_L18_003F7C70`): a small walker the
//! breeders 0x0b69 (crate::moby::classes::breeder) hunt. Alarmed by one, it flees along its path, runs to
//! Ratchet for cover or into a hatch 0x1151 (crate::moby::classes::level18::hatch), and goes back into its pool when it
//! escapes or dies. On the framework (crate::moby::ai): health `+0x20` (16 by default; its status `+0x2e`), reaction
//! `+0x60` (a hurt to state 9, sub 4; `+0x90` 0.8, `+0x94` 0.9, `+0x9c` 0.4), movement `+0x110` (radius 0.8, height
//! 0.5, steps 0.5 / 2, slope 90°, gravity 1/180, stop 0.25; its fence the path `+0x718`; its two aim records `+0x680`
//! / `+0x600` the turns' [`crate::moby::ai::joint_turns`]), no target block.
//!
//! **Its pvars:** `+0x50` a colour flash block (the tail's), `+0x200` its path network (0x20 records, `+0x730` how
//! many, from the path `+0x71c`, −1 none; `+0x73c` / `+0x740` its record and the goal's, `+0x748` the heading),
//! `+0x600` / `+0x680` the aim records on joint lists 0 / 1, `+0x700` the goal, `+0x714` the path it flees along,
//! `+0x720` its walk's yaw offset (±5..20°), `+0x724` a yaw to face (with `+0x766`: set by nothing here), `+0x72c`
//! how near Ratchet it keeps (5..7), `+0x744` a cuboid it walks home to (−1 none), `+0x74c` the hatches' group,
//! `+0x750` the hatch it makes for, `+0x754` a polygon (path) it and Ratchet must both be in for it to run to him
//! (−1 any), `+0x760` its path point, `+0x764` how it starts (1 in its pool, 2 out), `+0x768` alarmed (a breeder
//! after it sets it each tick), `+0x76a` the network's goal this tick, `+0x76c` along its path backward, `+0x76e`
//! the times it went back into its pool, `+0x770` the head aim's record.
//!
//! **The update** (`0x3f90e0`): state 0 the set-up ([`setup`], `0x3f9148`), 1 nothing (in its pool), else the frame's
//! damage ([`head`], `0x3f9368`), the state ([`states`], `0x3f9510`), the move by its sequence ([`movement`],
//! `0x3f9f08`) and the tail ([`tail`], `0x3fa078`).
//!
//! **The set-up**: the blocks; its limits (`0x3489d0` = L00 `0x32dcf0`, crate::moby::ai::senses); its network
//! (`0x344e08` = L02 `0x32f8b0`, crate::path_network::build); the offset; `+0x76c` and `+0x72c` at random; one of its
//! six pieces drawn on top (`+0x7c`, [`crate::moby::Moby::bangles`]); `+0x764` 1: into its pool ([`hide`], `0x3fa418`);
//! 2: state 2 (sub 1); else it stays in 0 and sets up again.
//!
//! **States** (`+0x20`; "sequence n + r" picks r of 0 / 1 at random, `0x3365e0` with blend 10 when the sequence
//! showing is not the one a first draw names):
//! * **1** in its pool (hidden, untargeted, no collision, still): sequence 0 + r on its first tick.
//! * **2** idle: its slow limits, sequence 0 + r; alarmed → 5 (sub 2) out of 4.
//! * **3** stuck: sequence 4 + r and a wait of 120..299 as a byte (`+0xbc`); past it the state before.
//! * **4** startled: sequence 2 + r; at its end the state before.
//! * **5** the flight: without a path (`+0x714`) or with Ratchet in 0x6f straight to the pause; else its fast limits,
//!   sequence 6 + r, from its path's point nearest it (`0x35e438`) along it ([`follow`], `0x3fa318`, points passed
//!   within 4 across); Ratchet in 0x6f, or 1 in 4 every 180 ticks, the pause (`+0xbc` 1, sequence 4 + r), at whose end
//!   5 again out of 4. No longer alarmed: to its hatch (7) if it has one, home (8) with a cuboid, else 2. Alarmed, with
//!   it and Ratchet in `+0x754` (or no polygon) and the run to Ratchet free ([`take_slot`]) → 6.
//! * **6** to Ratchet: sequence 2 + r; at its end its fast limits and sequence 6 + r, the goal Ratchet's place; out of
//!   `+0x754` (it or he) the run freed ([`free_slot`]) and 5; within `+0x72c` across sequence 4 + r, at whose end 6
//!   again. No longer alarmed: the run freed, then as 5's.
//! * **7** into a hatch: sequence 6 + r and the nearest member of `+0x74c` (within 1000 across, kept in `+0x750`):
//!   the goal its place inside it ([`crate::moby::classes::level18::hatch::entry`]) less its row 1; within 0.5
//!   across, back into its pool (gone after the fifth time); a closed hatch within 2 opened
//!   ([`crate::moby::classes::level18::hatch::open`], no dust).
//! * **8** home: sequence 6 + r, the goal its cuboid's place; within 0.5 across → 2.
//! * **9** hurt: the framework's flight (`0x3433a0` = L00 `0x328b80`); dead (or 1 in 30 in sequence 9): the run freed
//!   when the state before was 6, its bolts (`0x335db0` = L00 `0x31adc0`), the explosion (crate::moby::blast::explode:
//!   20 embers, 10 flares, near 9, light 3, shaking, 5 pieces of debris, flags 0x810001), back into its pool (gone
//!   after the fourth time); the flight over: the state before (sub 2).
//!
//! **Each tick** ([`head`]): a hit (flags 0x10000, its record kept, `0x338e08`) from a breeder does 4 while it is drawn,
//! else none, and from a stalker 0x0f0a or its spit 0x0f9e none; the frame's damage (`0x342658` = L00 `0x327e38`,
//! crate::moby::ai::take_damage; into 9 from 9, 4 or 3 the state before kept); under 90 its bolts and gone; a sphere
//! of its radius + 0.2 above it pushing what it touches (`0x338158` = L00 `0x31d180`, 10, flags 0x4000000); its draw
//! ([`draw`], `0x3fa488`, from state 2) registered on the second list (`0x302130` = L00 `0x2e7920`).
//!
//! **The move** ([`movement`], by its sequence `+0x42`): 0..5 facing `+0x724` (`0x347ab8` = L00 `0x32ceb8`) with
//! `+0x766`, else held (`0x347a98` = L00 `0x32ce98`). 6 / 7 the walk: with a network and a goal for it this tick
//! through it (`0x345298` = L02 `0x32fd40`, crate::path_network::steer, 0.4; `0x347920` = L00 `0x32cd20`), else
//! straight (`0x347830` = L00 `0x32cc30` with its offset); outside 3, 4 and 9, stuck 6 ticks (`+0x190`) or a drop or
//! sinking (`+0x1a4` & 3, then jittered 0.01, `0x33c5e8` = L00 `0x3217d8`) → 3.
//!
//! **The tail** ([`tail`]): the flash `+0x50` (`0x33b308` = L00 `0x3201e8`); its aims `+0x680` / `+0x600` (`0x33ef88` =
//! L00 `0x324208`, 0.03 / 0.3, lists 1 / 0) and its head (`0x341e28` = L00 `0x327308`, `+0x770`, list 2, 2.5:
//! crate::moby::joint_aim); its ticks; `+0x766`, `+0x768` and `+0x76a` cleared; its shadow within 24 of the camera
//! (`0x338b78` = L00 `0x31da58`, range 0x16).
//!
//! **The run to Ratchet** is one at a time: the moby that owns it (`+0xb8`, the director 0x0f01's streamed groups:
//! crate::moby::classes::level18::director) holds the count in its pvars' `+0xa8` (1), which `0x3fbd50` takes and
//! `0x3fbd78` gives back (the shared L18 / L19 / L20 code).
//!
//! Not ported: what brings it out of its pool — the director's streamed spawns (`0x3fbd90`, which also opens a hatch
//! with its dust), so the six placed with `+0x764` 1 stay there. A state 7 without a hatch in its group keeps the goal
//! it had (the game's reads a null moby). The L19 / L20 copy of the states (`0x3ddf48` / `0x3fdb88`) has no caller.

use crate::hero::physics::{add, scale, sub, V3};
use crate::moby::blast::{self, Explosion};
use crate::moby::classes::level08::tabora_ai::jitter;
use crate::moby::classes::props::{seq, wrapped};
use crate::moby::{ai, bolts, joint_aim, Ctx, MobyWorld};

use super::hatch;
use crate::moby::classes::breeder;

pub const CLASS: i16 = 0x0ef4;
/// A hit from it does 4 while this one is drawn, else none.
const BREEDER: i16 = breeder::CLASS;
/// Hits from these do none.
const STALKER: i16 = super::stalker::CLASS;
const SPIT: i16 = 0x0f9e;

const H: usize = 0x20;
const FLASH: usize = 0x50;
const REACT: usize = 0x60;
const MV: usize = 0x110;
const NODES: usize = 0x200;
const NODES_MAX: usize = 0x20;
const AIM_B: usize = 0x600;
const AIM_A: usize = 0x680;
const GOAL: usize = 0x700;
const PATH: usize = 0x714;
const FENCE: usize = 0x718;
const NET: usize = 0x71c;
const OFFSET: usize = 0x720;
const FACE: usize = 0x724;
const KEEP: usize = 0x72c;
const N_NODES: usize = 0x730;
const NET_AT: usize = 0x73c;
const NET_GOAL: usize = 0x740;
const HOME: usize = 0x744;
const NET_YAW: usize = 0x748;
const HATCHES: usize = 0x74c;
const HATCH: usize = 0x750;
const AREA: usize = 0x754;
const POINT: usize = 0x760;
const START: usize = 0x764;
const FACING: usize = 0x766;
const ALARM: usize = 0x768;
const NET_ON: usize = 0x76a;
const BACKWARD: usize = 0x76c;
const POOLED: usize = 0x76e;
const HEAD_AIM: usize = 0x770;
const PVARS: usize = 0x7f0;

/// The run to Ratchet's count in its owner's pvars (`0x3fbd50` / `0x3fbd78`).
const SLOT: usize = 0xa8;
/// The head's scale with the enemies' big heads.
const HEAD_SCALE: f32 = 2.5;
const DEG: f32 = 0.017_453_294;

/// `0x309878` (L02 `0x2f21c0`): the distance across, squared.
fn across_sq(a: V3, b: V3) -> f32 {
    let d = sub(a, b);
    d[0] * d[0] + d[1] * d[1]
}

/// `0x3365e0(m, base + r, 0, 10)` unless the sequence showing is `base + r'` (two draws, as the game's).
fn rand_seq(w: &MobyWorld, id: usize, ctx: &mut Ctx, base: u8) {
    let a = ctx.game.rng.randi(2) as u8 + base;
    if seq(w, id, ctx) != a {
        let b = ctx.game.rng.randi(2) as u8 + base;
        ctx.host.set_anim(&w.mobys[id], b, 0, 10);
    }
}

fn slow_senses(w: &mut MobyWorld, id: usize) { ai::senses(&mut w.mobys[id], 0.002_222_222_3, 0.066_666_67, 0.006_981_318, 0.209_439_55); }
fn fast_senses(w: &mut MobyWorld, id: usize) { ai::senses(&mut w.mobys[id], 0.004_166_667, 0.125, 0.006_981_318, 0.209_439_55); }

/// `0x3fbd50(owner)`: the run to Ratchet taken from its owner's count, when there is one left.
fn take_slot(w: &mut MobyWorld, id: usize) -> bool {
    let Some(o) = w.mobys[id].parent.filter(|&o| o < w.mobys.len()) else { return false };
    let n = w.mobys[o].pv_s(SLOT);
    if n <= 0 { return false; }
    w.mobys[o].set_pv_s(SLOT, n - 1);
    true
}

/// `0x3fbd78(owner)`: the run given back.
fn free_slot(w: &mut MobyWorld, id: usize) {
    let Some(o) = w.mobys[id].parent.filter(|&o| o < w.mobys.len()) else { return };
    let n = w.mobys[o].pv_s(SLOT);
    w.mobys[o].set_pv_s(SLOT, n.wrapping_add(1));
}

/// `0x3fa418`: into its pool — sequence 0, still, the reaction's options cleared, no collision, hidden and untargeted
/// (mode | 0x41, & ~0x1000), state 1 (sub 1).
fn hide(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.host.set_anim(&w.mobys[id], 0, 0, 3);
    ctx.host.set_anim_speed(&w.mobys[id], 0.0);
    let m = &mut w.mobys[id];
    m.set_pv_s(REACT + 0x38, 0);
    m.has_collision = false;
    m.mode = (m.mode | 0x41) & 0xefff;
    m.set_state(1, 1);
    w.changed = true;
}

/// Back into its pool, or gone once it has been back `most` times.
fn pool_or_die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, most: i16) {
    let n = w.mobys[id].pv_s(POOLED);
    if most <= n {
        w.delete(id);
    } else {
        w.mobys[id].set_pv_s(POOLED, n + 1);
        hide(w, id, ctx);
    }
}

/// No longer alarmed: to its hatch, home, or idle.
fn calm(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let s = if m.pv_i(HATCH) != 0 { 7 } else if m.pv_i(HOME) != -1 { 8 } else { 2 };
    m.set_state(s, -1);
}

/// 0x0ef4 (module docs).
pub fn update_0ef4(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    match w.mobys[id].state {
        0 => setup(w, id, ctx),
        1 => return,
        _ => {
            // The game runs the four on whatever came of the one before; a deleted moby is left alone here.
            head(w, id, ctx);
            if 0x80 <= w.mobys[id].state { return; }
            states(w, id, ctx);
            if 0x80 <= w.mobys[id].state { return; }
            movement(w, id, ctx);
            if 0x80 <= w.mobys[id].state { return; }
            tail(w, id, ctx);
        }
    }
    w.changed = true;
}

/// `0x3f9148` (module docs).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.set_pv_b(REACT + 0x10, 9);
    m.set_pv_i(REACT + 0x48, 4);
    m.set_pv_f(REACT + 0x30, 0.8);
    m.set_pv_f(REACT + 0x34, 0.9);
    m.set_pv_f(REACT + 0x3c, 0.4);
    if m.pv_s(H + 4) == 0 {
        m.set_pv_s(H + 4, 16);
        m.set_pv_f(H, 16.0);
    }
    m.set_pv_b(H + 8, 3);
    m.set_pv_f(MV + 0x34, 0.25);
    m.set_pv_f(MV, 0.8);
    m.set_pv_f(MV + 0x2c, 0.005_555_556_9);
    m.set_pv_f(MV + 4, 0.5);
    m.set_pv_f(MV + 8, 0.5);
    m.set_pv_f(MV + 0xc, 2.0);
    m.set_pv_f(MV + 0x10, 1.570_796_5);
    ai::senses(m, 0.003_333_333_4, 0.100_000_01, 0.003_490_659, 0.209_439_55);
    m.set_pv_i(MV + 0x48, AIM_A as i32);
    m.set_pv_i(MV + 0x4c, AIM_B as i32);
    let rate = ctx.game.rng.randf(5.0, 20.0);
    let off = if ctx.game.rng.randi(2) != 0 { rate * -DEG } else { rate * DEG };
    let m = &mut w.mobys[id];
    m.set_pv_f(OFFSET, off);
    m.set_pv_i(MV + 0x3c, m.pv_i(FENCE));
    let net = m.pv_i(NET);
    if net != -1 {
        let mut nodes = crate::path_network::read_nodes(&m.pvars, NODES, NODES_MAX);
        let n = crate::path_network::build(ctx.level, net, &mut nodes);
        crate::path_network::write_nodes(&mut m.pvars, NODES, &nodes[..n]);
        m.set_pv_i(N_NODES, n as i32);
        crate::path_network::open(&mut ctx.game.path_openings, ctx.level, &nodes[..n]);
    }
    let back = ctx.game.rng.randi(2) as i16;
    w.mobys[id].set_pv_s(BACKWARD, back);
    let keep = ctx.game.rng.randf(5.0, 7.0);
    w.mobys[id].set_pv_f(KEEP, keep);
    let piece = ctx.game.rng.randi(6);
    let m = &mut w.mobys[id];
    m.bangles |= 1 << piece;
    match m.pv_s(START) {
        1 => hide(w, id, ctx),
        2 => w.mobys[id].set_state(2, 1),
        _ => {}
    }
}

/// `0x3f9368` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    // `0x338e08(m, 0x10000, 1)`: the record kept.
    if let Some(h) = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0) {
        let from = h.tmpl.source.and_then(|s| w.mobys.get(s)).map(|s| s.o_class);
        let damage = match from {
            Some(BREEDER) => Some(if w.mobys[id].drawn != 0 { 4.0 } else { 0.0 }),
            Some(STALKER | SPIT) => Some(0.0),
            _ => None,
        };
        if let Some(d) = damage {
            let slot = w.mobys[id].hit_slot as usize;
            if let Some(r) = w.hits.records.get_mut(slot) { r.tmpl.damage = d; }
        }
    }
    let old = w.mobys[id].prev_state;
    ai::take_damage(w, id, ctx, false);
    let m = &mut w.mobys[id];
    if m.state == 9 && matches!(m.prev_state, 9 | 4 | 3) { m.prev_state = old; }
    if m.pos[2] < 90.0 {
        bolts::drop(w, id, 0, 3, ctx);
        w.delete(id);
        return;
    }
    let m = &w.mobys[id];
    let c = add(m.pos, [0.0, 0.0, m.pv_f(MV + 4) + m.pv_f(MV)]);
    let r = m.pv_f(MV) + 0.2;
    ai::attack_sphere(w, id, ctx, r, 10.0, 1.0, c, 0x400_0000, 0, 1, 0);
    if 2 <= w.mobys[id].state { w.register_draw(id); }
}

/// `0x3f9510` (module docs).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        1 => {
            if w.mobys[id].first() { rand_seq(w, id, ctx, 0); }
        }
        2 => {
            if w.mobys[id].first() {
                slow_senses(w, id);
                rand_seq(w, id, ctx, 0);
            } else if w.mobys[id].pv_s(ALARM) != 0 {
                let m = &mut w.mobys[id];
                m.set_state(5, 2);
                m.set_state(4, -1);
            }
        }
        3 => {
            if w.mobys[id].first() {
                rand_seq(w, id, ctx, 4);
                let t = ctx.game.rng.randi(180) + 120;
                w.mobys[id].timer = t as u8;
            } else {
                let m = &mut w.mobys[id];
                if (m.state_ticks as u16) > m.timer as u16 {
                    let p = m.prev_state;
                    m.set_state(p, -1);
                }
            }
        }
        4 => {
            if w.mobys[id].first() {
                rand_seq(w, id, ctx, 2);
            } else if wrapped(w, id, ctx) {
                let m = &mut w.mobys[id];
                let p = m.prev_state;
                m.set_state(p, -1);
            }
        }
        5 => flee(w, id, ctx),
        6 => to_hero(w, id, ctx),
        7 => to_hatch(w, id, ctx),
        8 => {
            if w.mobys[id].first() {
                rand_seq(w, id, ctx, 6);
                let home = w.mobys[id].pv_i(HOME);
                if let Some((p, _)) = ctx.level.cuboid_place(home) { w.mobys[id].set_pv_v(GOAL, p); }
            }
            let m = &mut w.mobys[id];
            m.set_pv_s(NET_ON, 1);
            if across_sq(m.pos, m.pv_v(GOAL)) < 0.25 { m.set_state(2, -1); }
        }
        9 => hurt(w, id, ctx),
        _ => {}
    }
}

/// The pause of state 5: `+0xbc` 1, sequence 4 + r.
fn pause(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].timer = 1;
    rand_seq(w, id, ctx, 4);
}

/// State 5 (module docs).
fn flee(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        if w.mobys[id].pv_i(PATH) == -1 || ctx.hero.state == 0x6f {
            pause(w, id, ctx);
        } else {
            w.mobys[id].timer = 0;
            fast_senses(w, id);
            rand_seq(w, id, ctx, 6);
            let m = &w.mobys[id];
            let k = breeder::nearest_point(ctx, m.pos, m.pv_i(PATH));
            w.mobys[id].set_pv_s(POINT, k);
        }
    } else if w.mobys[id].timer == 0 {
        follow(w, id, ctx, 4.0);
        let ticks = w.mobys[id].state_ticks as u16;
        if ctx.hero.state == 0x6f || (ticks % 180 == 0 && ctx.game.rng.randi(4) == 0) { pause(w, id, ctx); }
    } else if w.mobys[id].timer == 1 && wrapped(w, id, ctx) {
        let m = &mut w.mobys[id];
        m.set_state(5, -1);
        m.set_state(4, -1);
    }
    let m = &w.mobys[id];
    if m.pv_s(ALARM) == 0 {
        calm(w, id);
        return;
    }
    let area = m.pv_i(AREA);
    if area != -1 {
        if ctx.level.in_polygon(area, m.pos) && ctx.level.in_polygon(area, ctx.hero.pos) && take_slot(w, id) { w.mobys[id].set_state(6, -1); }
    } else if take_slot(w, id) {
        w.mobys[id].set_state(6, -1);
    }
}

/// State 6 (module docs).
fn to_hero(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        w.mobys[id].timer = 0;
        rand_seq(w, id, ctx, 2);
    } else if w.mobys[id].timer == 0 && wrapped(w, id, ctx) {
        w.mobys[id].timer = 1;
        fast_senses(w, id);
        rand_seq(w, id, ctx, 6);
    } else if w.mobys[id].timer == 1 {
        let hero = ctx.hero.pos;
        let m = &mut w.mobys[id];
        m.set_pv_v(GOAL, hero);
        m.set_pv_s(NET_ON, 1);
        let (area, keep, pos) = (m.pv_i(AREA), m.pv_f(KEEP), m.pos);
        if area != -1 && (!ctx.level.in_polygon(area, pos) || !ctx.level.in_polygon(area, hero)) {
            free_slot(w, id);
            w.mobys[id].set_state(5, -1);
        } else if across_sq(pos, hero) < keep * keep {
            w.mobys[id].timer = 2;
            rand_seq(w, id, ctx, 4);
        }
    } else if w.mobys[id].timer == 2 && wrapped(w, id, ctx) {
        w.mobys[id].set_state(6, -1);
    }
    if w.mobys[id].pv_s(ALARM) == 0 {
        free_slot(w, id);
        calm(w, id);
    }
}

/// State 7 (module docs).
fn to_hatch(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        rand_seq(w, id, ctx, 6);
        let pos = w.mobys[id].pos;
        let mut best = 1000.0;
        for e in crate::moby::director::members(w, w.mobys[id].pv_i(HATCHES)) {
            let Some(o) = w.mobys.get(e) else { break };
            let d = across_sq(pos, o.pos);
            if d < best {
                ai::set_link(&mut w.mobys[id], HATCH, Some(e));
                best = d;
            }
        }
        if let Some(k) = ai::link(&w.mobys[id], HATCH).filter(|&k| k < w.mobys.len()) {
            let goal = add(hatch::entry(w, k), scale(w.mobys[k].rows[1], -1.0));
            w.mobys[id].set_pv_v(GOAL, goal);
        }
    }
    w.mobys[id].set_pv_s(NET_ON, 1);
    let m = &w.mobys[id];
    if across_sq(m.pos, m.pv_v(GOAL)) < 0.25 {
        pool_or_die(w, id, ctx, 5);
        return;
    }
    let Some(k) = ai::link(m, HATCH).filter(|&k| k < w.mobys.len()) else { return };
    if w.mobys[k].state != 2 && across_sq(m.pos, hatch::entry(w, k)) < 4.0 { hatch::open(w, k, ctx, false); }
}

/// State 9 (module docs).
fn hurt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let hit = ai::flight(w, id, ctx);
    let s = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
    if (s == 9 && ctx.game.rng.randi(30) == 0) || hit & 1 != 0 {
        if w.mobys[id].prev_state == 6 { free_slot(w, id); }
        bolts::drop(w, id, 0, 3, ctx);
        let e = Explosion { embers: 20, flares: 10, near: 9.0, size: 1.0, sound: -1, light: 3.0, shake: true, debris: 5, lod: -1, flags: 0x81_0001, ..Default::default() };
        blast::explode(w, ctx, Some(id), None, &e);
        pool_or_die(w, id, ctx, 4);
    } else if hit & 2 != 0 {
        let m = &mut w.mobys[id];
        let p = m.prev_state;
        m.set_state(p, 2);
    }
}

/// `0x3fa318(m, &goal, path, r, &point, backward)`: the goal its path point; passed within `r` across, the next (or
/// back); past the last (first) false, the point wrapped.
fn follow(w: &mut MobyWorld, id: usize, ctx: &Ctx, r: f32) -> bool {
    let m = &mut w.mobys[id];
    let Some(pts) = usize::try_from(m.pv_i(PATH)).ok().and_then(|i| ctx.level.paths.get(i)).filter(|v| !v.is_empty()) else { return false };
    let n = pts.len() as i16;
    let mut i = m.pv_s(POINT);
    let q = pts[(i.max(0) as usize).min(pts.len() - 1)];
    m.set_pv_v(GOAL, [q[0], q[1], q[2]]);
    m.set_pv_f(GOAL + 0xc, q[3]);
    let passed = across_sq(m.pos, [q[0], q[1], q[2]]) < r * r;
    let ok = if m.pv_s(BACKWARD) != 0 {
        if passed { i -= 1; }
        if i < 0 {
            i = n - 1;
            false
        } else {
            true
        }
    } else {
        if passed { i += 1; }
        if n <= i {
            i = 0;
            false
        } else {
            true
        }
    };
    m.set_pv_s(POINT, i);
    ok
}

/// `0x3f9f08` (module docs).
fn movement(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
    if s < 6 {
        let m = &w.mobys[id];
        if m.pv_s(FACING) != 0 {
            let yaw = m.pv_f(FACE);
            ai::turn_in_place(w, id, ctx, yaw);
        } else {
            ai::hold(w, id, ctx);
        }
        return;
    }
    if 8 <= s { return; }
    let m = &w.mobys[id];
    let goal = m.pv_v(GOAL);
    if m.pv_i(NET) != -1 && m.pv_s(NET_ON) != 0 {
        let n = (m.pv_i(N_NODES).max(0) as usize).min(NODES_MAX);
        let nodes = crate::path_network::read_nodes(&m.pvars, NODES, n);
        let (mut y, mut at, mut to) = (m.pv_f(NET_YAW), m.pv_i(NET_AT), m.pv_i(NET_GOAL));
        crate::path_network::steer(ctx.level, &nodes, 0.4, m.pos, goal, &mut y, &mut at, &mut to);
        let m = &mut w.mobys[id];
        m.set_pv_f(NET_YAW, y);
        m.set_pv_i(NET_AT, at);
        m.set_pv_i(NET_GOAL, to);
        ai::move_step(w, id, ctx, y, goal);
    } else {
        let off = m.pv_f(OFFSET);
        ai::move_toward(w, id, ctx, off, goal);
    }
    let m = &w.mobys[id];
    if matches!(m.state, 9 | 4 | 3) { return; }
    if 6 <= m.pv_i(MV + 0x80) { w.mobys[id].set_state(3, -1); }
    if w.mobys[id].pv_i(MV + 0x94) & 3 != 0 {
        let mut p = w.mobys[id].pos;
        jitter(&mut ctx.game.rng, &mut p, 0.01);
        let m = &mut w.mobys[id];
        m.pos = p;
        m.set_state(3, -1);
    }
}

/// `0x3fa078` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ai::flash(&mut w.mobys[id], FLASH);
    joint_aim::aim(w, id, AIM_A, id, 1, 0.03, 0.3);
    joint_aim::aim(w, id, AIM_B, id, 0, 0.03, 0.3);
    joint_aim::head(w, ctx, id, HEAD_AIM, 2, HEAD_SCALE);
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = (m.state_ticks as u16 + 1) as i16; }
    m.set_pv_s(NET_ON, 0);
    m.set_pv_s(FACING, 0);
    m.set_pv_s(ALARM, 0);
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
}

/// `0x3fa488` (the second list): two glows at joint 5 (`0x3127c8`), 0.25 across (`0x2edf78`, crate::fx::sprite):
/// texture 0x2e in 0x80ffffff and 0xc in 0x80f0f020.
pub fn draw(w: &MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let Some(p) = host.joint(&w.mobys[id], 5) else { return };
    crate::fx::sprite(out, 0.25, 0.0, 0.0, p, cam, 0x2e, 0x80ff_ffff, true);
    crate::fx::sprite(out, 0.25, 0.0, 0.0, p, cam, 0xc, 0x80f0_f020, true);
}
