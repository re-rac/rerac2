//! The prison's small walker **0x0f91** (L14 `0x424560`, 2 placed; the decomp's `func_L14_00422FA0`, its helpers
//! `func_L14_00423000`.. `func_L14_00423E50`): a creature on the enemy framework (crate::moby::ai) that idles,
//! watches a moby or a cuboid, walks a path, runs to Ratchet and walks home through its path network. Its layout is
//! the breeders' prey's (crate::moby::classes::level18::prey). Health `+0x20` (16), reaction `+0x60` (a hurt to state
//! 8, sub 4), movement `+0x110` (radius 0.4, height 0.5, steps 0.5 / 0.5, slope π, gravity 1/180, stop 0.25; its fence
//! `+0x738` when set; its two aim records `+0x680` / `+0x600`), no target block; its update and draw distances 200.
//!
//! **Its pvars:** `+0x50` a colour flash block, `+0x200` its path network (0x20 records from the path `+0x73c`,
//! `+0x750` how many; `+0x75c` / `+0x760` its record and the goal's), `+0x600` / `+0x680` the aim records on joint
//! lists 0 / 1, `+0x700` the goal, `+0x710` home (its place at the set-up), `+0x720` / `+0x724` the walk's and the
//! turn's rates for state 3, `+0x728` / `+0x72c` for states 5 and 6 (the limits `0x336f28` = L00 `0x32dcf0`,
//! crate::moby::ai::senses, of `(2a/3600, a/60, 2b·4.848e-6, b·2.909e-4)`), `+0x730` the path of state 3, `+0x734`
//! the path of state 5, `+0x740` the walk's yaw offset (degrees at the set-up), `+0x744` the yaw it faces, `+0x748` the
//! moby it watches in state 2, `+0x74c` how near Ratchet it runs, `+0x764` the cuboid it watches in state 4 (−1
//! none), `+0x768` the network's heading, `+0x770` its path point, `+0x774` the state it starts in, `+0x776` facing
//! this tick, `+0x77a` the network this tick, `+0x77c` along the path of state 5 backward (at random).
//!
//! **The update** (`0x424560`): state 0 the set-up ([`setup`], `0x4245c0`), else the frame's damage ([`head`],
//! `0x424800`), the state ([`states`], `0x424918`), the move by its sequence ([`movement`], `0x4250f0`) and the tail
//! ([`tail`], `0x425198`).
//!
//! **The set-up**: the blocks; the walk offset to radians; its network (`0x332608` = L02 `0x32f8b0`,
//! crate::path_network::build); `+0x77c` at random; home; the state `+0x774` (sub 1) when above 0 (3 only with a path
//! `+0x730`), else 1.
//!
//! **States** (`+0x20`; "sequence n + r" picks r of 0..k at random, `0x322f70` = L00 `0x31b5f0` with blend 10 when the
//! sequence showing is not the one a first draw names; "at its end" is the sequence wrapping, `+0x60 & 2`):
//! * **1** idle: sequence 1 + r on its first tick.
//! * **2** watching `+0x748`: sequence 1 and `+0x96` 0..89 on its first tick; then facing the moby ([`face`],
//!   `0x425380`); past 180 ticks `+0x96` again 0..89 and, 4 in 5, sequence 5.
//! * **3** the path `+0x730`: its limits from `+0x720` / `+0x724`, sequence 6 + r (of 4), its point 0; after that a
//!   step along it (`0x425290` = L18 `0x3f8ff0`, crate::moby::classes::breeder::path_step_at, within 2):
//!   not past its last point, state 2. (What the code does: the state lasts two ticks.)
//! * **4** watching the cuboid `+0x764`: sequence 4 or 3 at random (`+0xbc`); facing the cuboid's place; at its end
//!   5 (sub 2).
//! * **5** the walk along `+0x734`: sequence 10 + r; at its end with the path its limits from `+0x728` / `+0x72c`,
//!   sequence 14 + r, from the path's point nearest it (`0x34cf20` = L18 `0x35e438`); without, the stop at once. On it
//!   steps within 1 (backward with `+0x77c`); every 180 ticks 1 in 2 the stop (sequence 12 + r), at whose end 5 again
//!   when another of its group lives that is no 0x0f91 ([`others`], `0x425410`), else 1.
//! * **6** the run to Ratchet: sequence 10 + r; at its end its limits from `+0x728` / `+0x72c`, sequence 14 + r, the
//!   goal Ratchet's place; within `+0x74c` across of him while drawn the stop (12 + r), at whose end 6 again.
//! * **7** home: sequence 14 + r; through its network toward home (`0x332f48` = L02 `0x32fd40`,
//!   crate::path_network::steer, 0.4), the goal home; within 0.2 across, 1 (sub 1).
//! * **8** hurt: the framework's flight (`0x330a38` = L00 `0x328b80`); its end (or 1 in 30 in sequence 0x12) its bolts
//!   (`0x322740` = L00 `0x31adc0`, limit 3) and deleted; the flight over, the state before (sub 2).
//!
//! **Each tick** ([`head`]): a hit (flags 0x10000, its record kept, `0x325680`) from a breeder 0x0b69 does 1 while it
//! is drawn, else none; the frame's damage (`0x32fcf0` = L00 `0x327e38`, crate::moby::ai::take_damage); no health left,
//! a smoke burst (`0x328ae0` = L00 `0x3210a8`, 1, 13), its bolts and deleted; under 90 its bolts and deleted.
//!
//! **The move** ([`movement`], by its sequence `+0x42`): 1..5 and 10..13 facing `+0x744` with `+0x776` (`0x335fb8` =
//! L00 `0x32ceb8`), else held (`0x335f98` = L00 `0x32ce98`); 6..9, 14 and 15 with `+0x77a` along the network's
//! heading (`0x335e20` = L00 `0x32cd20`), else toward the goal with its offset (`0x335d30` = L00 `0x32cc30`).
//!
//! **The tail** ([`tail`]): the flash `+0x50` (`0x327c20` = L00 `0x3201e8`); its aims `+0x680` / `+0x600`
//! (`0x32bd18` = L00 `0x324208`, 0.03 / 0.3, lists 1 / 0: crate::moby::joint_aim); its ticks; `+0x776`, `+0x778` and
//! `+0x77a` cleared; its shadow within 24 of the camera (`0x3253f0` = L00 `0x31da58`, range 0x16).
//!
//! **The face** ([`face`], `0x425380`): `+0x776` set; from joint 2 (`0x31a198` = L00 `0x3127c8`) to the point, the
//! yaw into `+0x744` and half the pitch down into both aim records' asked turns (`+0x64`).
//!
//! What sets it into states 3..7 besides its start is not in its own code.

use crate::hero::physics::{atan2, dist2, len2, sub, V3};
use crate::moby::classes::breeder::{nearest_point, path_step_at};
use crate::moby::classes::props::{seq, wrapped};
use crate::moby::{ai, bolts, joint_aim, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0f91;
/// A hit from it does 1 while this one is drawn, else none.
const BREEDER: i16 = 0x0b69;

const H: usize = 0x20;
const FLASH: usize = 0x50;
const REACT: usize = 0x60;
const MV: usize = 0x110;
const NODES: usize = 0x200;
const NODES_MAX: usize = 0x20;
const AIM_B: usize = 0x600;
const AIM_A: usize = 0x680;
const GOAL: usize = 0x700;
const HOME: usize = 0x710;
const RATES3: usize = 0x720;
const RATES: usize = 0x728;
const PATH3: usize = 0x730;
const PATH5: usize = 0x734;
const FENCE: usize = 0x738;
const NET: usize = 0x73c;
const OFFSET: usize = 0x740;
const FACE: usize = 0x744;
const WATCH: usize = 0x748;
const KEEP: usize = 0x74c;
const N_NODES: usize = 0x750;
const NET_AT: usize = 0x75c;
const NET_GOAL: usize = 0x760;
const CUBOID: usize = 0x764;
const NET_YAW: usize = 0x768;
const POINT: usize = 0x770;
const START: usize = 0x774;
const FACING: usize = 0x776;
const CLEARED: usize = 0x778;
const NET_ON: usize = 0x77a;
const BACKWARD: usize = 0x77c;
const PVARS: usize = 0x780;

/// `gp + 0x2664` / `0x2668` / `0x266c`: the watch's period, the stop's period and its odds.
const WATCH_TICKS: i32 = 180;
const STOP_TICKS: u16 = 180;
const STOP_ODDS: i32 = 2;
/// `gp + 0x2660`: under it, gone.
const FLOOR: f32 = 90.0;

/// `0x322f70(m, base + r, 0, 10)` unless the sequence showing is `base + r'` (two draws of r of 0..n, as the game's).
fn rand_seq(w: &MobyWorld, id: usize, ctx: &mut Ctx, base: u8, n: i32) {
    let a = ctx.game.rng.randi(n) as u8 + base;
    if seq(w, id, ctx) != a {
        let b = ctx.game.rng.randi(n) as u8 + base;
        ctx.host.set_anim(&w.mobys[id], b, 0, 10);
    }
}

/// `0x322f70(m, s, 0, 10)` unless it shows `s`.
fn set_seq(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: u8) {
    if seq(w, id, ctx) != s { ctx.host.set_anim(&w.mobys[id], s, 0, 10); }
}

/// The limits from the rates at `o` (module docs).
fn rate_senses(w: &mut MobyWorld, id: usize, o: usize) {
    let m = &mut w.mobys[id];
    let (a, b) = (m.pv_f(o), m.pv_f(o + 4));
    ai::senses(
        m,
        (a + a) * f32::from_bits(0x3991_a2b4),
        a * f32::from_bits(0x3c88_8889),
        (b + b) * f32::from_bits(0x36a2_ad2f),
        b * f32::from_bits(0x3998_825c),
    );
}

/// 0x0f91 (module docs).
pub fn update_0f91(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    if w.mobys[id].state == 0 {
        setup(w, id, ctx);
    } else {
        // The game runs the four on whatever came of the one before; a deleted moby is left alone here.
        head(w, id, ctx);
        if 0x80 <= w.mobys[id].state { return; }
        states(w, id, ctx);
        if 0x80 <= w.mobys[id].state { return; }
        movement(w, id, ctx);
        if 0x80 <= w.mobys[id].state { return; }
        tail(w, id, ctx);
    }
    w.changed = true;
}

/// `0x4245c0` (module docs).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    // The reaction block.
    for (o, v) in [(0x70, 8u8), (0x71, 0x11), (0x72, 0x11), (0x76, 0x12), (0x77, 0x13), (0xcc, 3), (0xcd, 6), (0xce, 3), (0xcf, 6), (0xd2, 0xd), (0xd3, 0x1a)] {
        m.set_pv_b(o, v);
    }
    m.set_pv_i(REACT + 0x48, 4);
    for (o, v) in [(0x7c, 0x3dcc_ccce_u32), (0x80, 0x3d88_8889), (0x84, 0x3b88_8889), (0x88, 0x3c08_8889), (0x8c, 0x3b88_8889), (0x90, 0x3f00_0000), (0x94, 0x3f00_0000), (0x9c, 0x3ecc_cccd)] {
        m.set_pv_f(o, f32::from_bits(v));
    }
    // The health block.
    m.set_pv_f(H, 16.0);
    m.set_pv_s(H + 4, 0x10);
    m.set_pv_b(H + 8, 3);
    m.update_dist = 0xc8;
    m.draw_dist = 0xc8;
    // The movement block.
    m.set_pv_f(MV + 0x34, 0.25);
    m.set_pv_f(MV, f32::from_bits(0x3ecc_cccd));
    m.set_pv_f(MV + 4, 0.5);
    m.set_pv_f(MV + 8, 0.5);
    m.set_pv_f(MV + 0xc, 0.5);
    m.set_pv_f(MV + 0x30, f32::from_bits(0x3e86_0a93));
    m.set_pv_f(MV + 0x2c, f32::from_bits(0x3bb6_0b61));
    m.set_pv_i(MV + 0x3c, -1);
    m.set_pv_f(MV + 0x10, f32::from_bits(0x4049_0fd0));
    ai::senses(m, f32::from_bits(0x3b5a_740e), f32::from_bits(0x3dcc_ccce), f32::from_bits(0x3b64_c38a), f32::from_bits(0x3e56_7752));
    let off = m.pv_f(OFFSET) * f32::from_bits(0x3c8e_fa36);
    m.set_pv_f(OFFSET, off);
    m.set_pv_i(MV + 0x48, AIM_A as i32);
    m.set_pv_i(MV + 0x4c, AIM_B as i32);
    if m.pv_i(FENCE) != -1 { m.set_pv_i(MV + 0x3c, m.pv_i(FENCE)); }
    let net = m.pv_i(NET);
    let mut nodes = crate::path_network::read_nodes(&m.pvars, NODES, NODES_MAX);
    let n = crate::path_network::build(ctx.level, net, &mut nodes);
    crate::path_network::write_nodes(&mut m.pvars, NODES, &nodes[..n]);
    m.set_pv_i(N_NODES, n as i32);
    crate::path_network::open(&mut ctx.game.path_openings, ctx.level, &nodes[..n]);
    let back = ctx.game.rng.randi(2) as i16;
    let m = &mut w.mobys[id];
    m.set_pv_s(BACKWARD, back);
    let pos = m.pos;
    m.set_pv_v(HOME, pos);
    let s = m.pv_s(START);
    if 0 < s && (s != 3 || m.pv_i(PATH3) != -1) { m.set_state(s as u8, 1) } else { m.set_state(1, 1) }
}

/// `0x424800` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    // `0x325680(m, 0x10000, 1)`: the record kept.
    if let Some(h) = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0) {
        if h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == BREEDER) {
            let d = if w.mobys[id].drawn != 0 { 1.0 } else { 0.0 };
            let slot = w.mobys[id].hit_slot as usize;
            if let Some(r) = w.hits.records.get_mut(slot) { r.tmpl.damage = d; }
        }
    }
    ai::take_damage(w, id, ctx, false);
    if w.mobys[id].pv_f(H) <= 0.0 {
        let at = w.mobys[id].pos;
        crate::moby::blast::smoke_burst(w, ctx, 1.0, 13.0, Some(id), at, -1);
        bolts::drop(w, id, 0, 3, ctx);
        w.delete(id);
    } else if w.mobys[id].pos[2] < FLOOR {
        bolts::drop(w, id, 0, 3, ctx);
        w.delete(id);
    }
}

/// `0x425380(m, p)` (module docs).
fn face(w: &mut MobyWorld, id: usize, ctx: &Ctx, p: V3) {
    w.mobys[id].set_pv_s(FACING, 1);
    let j = ctx.host.joint(&w.mobys[id], 2).unwrap_or(w.mobys[id].pos);
    let v = sub(p, j);
    let m = &mut w.mobys[id];
    m.set_pv_f(FACE, atan2(v[0], v[1]));
    let pitch = -atan2(len2(v), v[2]) * 0.5;
    m.set_pv_f(AIM_A + joint_aim::ASK + 4, pitch);
    m.set_pv_f(AIM_B + joint_aim::ASK + 4, pitch);
}

/// `0x425410(m)`: another of its group alive that is no 0x0f91.
fn others(w: &MobyWorld, id: usize) -> bool {
    let Some(g) = usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)) else { return false };
    g.iter().any(|&k| w.mobys.get(k as usize).is_some_and(|o| o.state < 0x80 && o.o_class != CLASS))
}

/// `0x424918` (module docs).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        1 => {
            if w.mobys[id].first() { rand_seq(w, id, ctx, 1, 2); }
        }
        2 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 1);
                let t = ctx.game.rng.randi(WATCH_TICKS >> 1);
                let m = &mut w.mobys[id];
                m.timer = 0;
                m.state_ticks = t as i16;
                return;
            }
            let k = w.mobys[id].pv_i(WATCH);
            if let Some(p) = usize::try_from(k).ok().and_then(|k| w.mobys.get(k)).map(|o| o.pos) { face(w, id, ctx, p); }
            if w.mobys[id].timer == 0 && (w.mobys[id].state_ticks as u16 as i32) > WATCH_TICKS {
                let t = ctx.game.rng.randi(WATCH_TICKS >> 1);
                w.mobys[id].state_ticks = t as i16;
                if ctx.game.rng.randi(5) != 0 { set_seq(w, id, ctx, 5); }
            }
            if w.mobys[id].timer == 1 && wrapped(w, id, ctx) { w.mobys[id].set_state(2, -1); }
        }
        3 => {
            if w.mobys[id].first() {
                rate_senses(w, id, RATES3);
                rand_seq(w, id, ctx, 6, 4);
                w.mobys[id].set_pv_s(POINT, 0);
            } else {
                let path = w.mobys[id].pv_i(PATH3);
                if path_step_at(w, id, ctx, 2.0, path, GOAL, POINT, false) { w.mobys[id].set_state(2, -1); }
            }
        }
        4 => {
            if w.mobys[id].first() {
                let r = ctx.game.rng.randi(2) as u8;
                w.mobys[id].timer = r;
                match r {
                    0 => set_seq(w, id, ctx, 4),
                    _ => set_seq(w, id, ctx, 3),
                }
            }
            let c = w.mobys[id].pv_i(CUBOID);
            if c != -1 {
                if let Some((p, _)) = ctx.level.cuboid_place(c) { face(w, id, ctx, p); }
            }
            if wrapped(w, id, ctx) { w.mobys[id].set_state(5, 2); }
        }
        5 => walk(w, id, ctx),
        6 => run(w, id, ctx),
        7 => {
            if w.mobys[id].first() { rand_seq(w, id, ctx, 14, 2); }
            let m = &w.mobys[id];
            let n = (m.pv_i(N_NODES).max(0) as usize).min(NODES_MAX);
            let nodes = crate::path_network::read_nodes(&m.pvars, NODES, n);
            let (home, pos) = (m.pv_v(HOME), m.pos);
            let (mut y, mut at, mut to) = (m.pv_f(NET_YAW), m.pv_i(NET_AT), m.pv_i(NET_GOAL));
            crate::path_network::steer(ctx.level, &nodes, 0.4, pos, home, &mut y, &mut at, &mut to);
            let m = &mut w.mobys[id];
            m.set_pv_f(NET_YAW, y);
            m.set_pv_i(NET_AT, at);
            m.set_pv_i(NET_GOAL, to);
            m.set_pv_v(GOAL, home);
            m.set_pv_s(NET_ON, 1);
            if dist2(pos, home) < 0.2 { m.set_state(1, 1); }
        }
        8 => hurt(w, id, ctx),
        _ => {}
    }
}

/// State 5 (module docs).
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        w.mobys[id].timer = 0;
        rand_seq(w, id, ctx, 10, 2);
        return;
    }
    match w.mobys[id].timer {
        0 if wrapped(w, id, ctx) => {
            let path = w.mobys[id].pv_i(PATH5);
            if path != -1 {
                w.mobys[id].timer = 1;
                rate_senses(w, id, RATES);
                rand_seq(w, id, ctx, 14, 2);
                let k = nearest_point(ctx, w.mobys[id].pos, path);
                w.mobys[id].set_pv_s(POINT, k);
            } else {
                w.mobys[id].timer = 2;
            }
        }
        1 => {
            let (path, back) = (w.mobys[id].pv_i(PATH5), w.mobys[id].pv_s(BACKWARD) != 0);
            path_step_at(w, id, ctx, 1.0, path, GOAL, POINT, back);
            if (w.mobys[id].state_ticks as u16) % STOP_TICKS == 0 && ctx.game.rng.randi(STOP_ODDS) == 0 {
                w.mobys[id].timer = 2;
                rand_seq(w, id, ctx, 12, 2);
            }
        }
        2 if wrapped(w, id, ctx) => {
            let s = if others(w, id) { 5 } else { 1 };
            w.mobys[id].set_state(s, -1);
        }
        _ => {}
    }
}

/// State 6 (module docs).
fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        w.mobys[id].timer = 0;
        rand_seq(w, id, ctx, 10, 2);
        return;
    }
    match w.mobys[id].timer {
        0 if wrapped(w, id, ctx) => {
            w.mobys[id].timer = 1;
            rate_senses(w, id, RATES);
            rand_seq(w, id, ctx, 14, 2);
        }
        1 => {
            let hero = ctx.hero.pos;
            let m = &mut w.mobys[id];
            m.set_pv_v(GOAL, hero);
            if dist2(m.pos, hero) < m.pv_f(KEEP) && m.drawn != 0 {
                w.mobys[id].timer = 2;
                rand_seq(w, id, ctx, 12, 2);
            }
        }
        2 if wrapped(w, id, ctx) => w.mobys[id].set_state(6, -1),
        _ => {}
    }
}

/// State 8 (module docs).
fn hurt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let hit = ai::flight(w, id, ctx);
    let s = ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a);
    if (s == 0x12 && ctx.game.rng.randi(30) == 0) || hit & 1 != 0 {
        bolts::drop(w, id, 0, 3, ctx);
        w.delete(id);
    } else if hit & 2 != 0 {
        let m = &mut w.mobys[id];
        let p = m.prev_state;
        m.set_state(p, 2);
    }
}

/// `0x4250f0` (module docs).
fn movement(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = ctx.host.anim(&w.mobys[id]).map_or(0, |a| a.seq_a);
    let m = &w.mobys[id];
    match s {
        1..=5 | 10..=13 => {
            if m.pv_s(FACING) != 0 {
                let yaw = m.pv_f(FACE);
                ai::turn_in_place(w, id, ctx, yaw);
            } else {
                ai::hold(w, id, ctx);
            }
        }
        6..=9 | 14 | 15 => {
            let goal = m.pv_v(GOAL);
            if m.pv_s(NET_ON) != 0 {
                let y = m.pv_f(NET_YAW);
                ai::move_step(w, id, ctx, y, goal);
            } else {
                let off = m.pv_f(OFFSET);
                ai::move_toward(w, id, ctx, off, goal);
            }
        }
        _ => {}
    }
}

/// `0x425198` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ai::flash(&mut w.mobys[id], FLASH);
    joint_aim::aim(w, id, AIM_A, id, 1, f32::from_bits(0x3cf5_c28f), f32::from_bits(0x3e99_999a));
    joint_aim::aim(w, id, AIM_B, id, 0, f32::from_bits(0x3cf5_c28f), f32::from_bits(0x3e99_999a));
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = (m.state_ticks as u16 + 1) as i16; }
    m.set_pv_s(NET_ON, 0);
    m.set_pv_s(FACING, 0);
    m.set_pv_s(CLEARED, 0);
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
}
