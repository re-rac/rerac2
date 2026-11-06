//! Damosel's flying traffic **0x11cf** (28 placed), **0x0d9b** (7), **0x11cc** (13) and **0x0d5c** (12), one update
//! (L18 `0x3fcb78`; the decomp's `func_L18_003FB708`): a car that flies a path or straight ahead (or rides along with a leader, like the cars of a
//! train), carries Ratchet (its platform block `+0x20`, crate::moby::platform), and at the end of its run drops and
//! bursts, or warps out (stretched along its first row) and comes back at its start. 0x0d9b also rams: what its nose
//! touches is knocked away, and Ratchet is knocked down and dies 30 ticks later.
//!
//! **Its pvars:** `+0x20` its platform block, `+0x60` / `+0x70` its place and angles at the start, `+0x80` its
//! velocity (a tick), `+0x90` the warp's point (for its draws), `+0xa0` its place in its leader's frame, `+0xb0` its
//! path (−1 none: straight ahead), `+0xb4` a cuboid Ratchet starts it in (−1: it starts at once), `+0xb8` the cuboid
//! that ends its run (−1 none), `+0xbc` it comes back after a warp, `+0xc0` / `+0xc4` the warp's size and colour (for
//! its draws), `+0xc8` its leader (−1 none), `+0xcc` the warp-in's size (as an integer), `+0xd0` its speed (a second),
//! `+0xd4` an area that, with the camera in it, puts it away (−1 none), `+0xd8` / `+0xdc` its share of the path and
//! its rate (a tick), `+0xe0` the countdown to Ratchet's death, `+0xe4` its loop sound. In state 0 the classes other than
//! 0x0d9b and 0x11cf clear `+0x08`.
//!
//! **Each tick**: with the camera (`0x1b9740`) in its area `+0xd4` it is put away ([`put_away`], `0x3fddc8`: state
//! 2). The death's countdown on: under 2, Ratchet dies (`0x2d0748` = L00 `0x2b31f0`).
//! * **0**: its start kept. With a path: its rate 1/(the first segment's length) of its speed, put at the path's start.
//!   With a leader: its place in the leader's frame (the leader's rows transposed). With a start cuboid hidden (mode
//!   1) and 4; else 1.
//! * **4**: hidden; Ratchet in the start cuboid (`0x33c668` = L00 `0x321858`): with a leader 1; else its first row
//!   shrunk to 0.0323 long, its rows kept (mode 0x100) and 7 (the warp-in).
//! * **1**: on its first tick its velocity its first row at its speed. With a path: its pose there (`0x360058` = L00
//!   `0x343fb8`, crate::paths::pose) and its share on. Else with a leader: the leader's mode, alpha, collision, rows
//!   and angles, its place the leader's plus its own in that frame; or alone it moves by its velocity, keeps its loop
//!   sound 0 of class 0x0d5c playing (`0x37c980` = L00 `0x360698`) and, in the end cuboid, stops it and drops (5).
//!   * 0x0d9b: the sphere of 2 at (5.5, 0, −0.2) in its frame (`0x2fbd48` = L00 `0x2e14b0`): Ratchet in it is put
//!     to his state 0 (`0x2e02e8`) and hit for the most of 100 and 30 times his health limit, flags 0x10001, pushed
//!     (−20 along its first row, 80 up; `0x3385d0` = L00 `0x31d4c8`, crate::moby::MobyWorld::deliver_hit_any), his
//!     death's countdown 30 and a blast at him (`0x415e70` = L00 `0x3f8348`: size 1, 0x19 / 0x14 / 0x14 sprites,
//!     0x14 puffs, 0x2d streaks, the flash, the shake); the others hit for 50, the same flags, pushed 0.1 along its
//!     first row and 0.4 up. Drawn, one frame in 16 ten sparks (`0x3516e8` = L00 `0x336148`,
//!     crate::particles::lava::spark) from (5.8, 0, −0.8) in its frame: 60000..120000 across, 2..6 a second along its
//!     second row either way, −5..1 a second up, falling 1/120 a tick.
//! * **5**: it falls (100 a second²) and turns (90° a second about its y) by its velocity; under 85, a blast (size 4,
//!   5 / 0 / 5 sprites, 0x14 puffs, the flash) and it is put away for good (3).
//! * **6** (the warp-out; the run's end of other classes): on its first tick rows kept and mode 8, the warp's point 16
//!   ahead and 0.5 up, its velocity 30 a second along its first row, its class sound 1 (0x0d5c's); then for 30 ticks
//!   its first row stretched by 1 + 0.6/60 a tick (the warp's size 0 → 8, colour 0x00ff2020 → 0x7fffffff), its
//!   velocity 1 a second; then for 12 ticks shrunk by 1 − 15/60 (13.5 with Progressive Scan, `0x1a7b98`; the size back to
//!   0, the colour on to 0x7f306000), its alpha out; at the end back at its start (7) with `+0xbc`, else put away (3).
//! * **7** (the warp-in): on its first tick shown, rows kept, mode 8, alpha 0, the warp's point 0.5 up its unit rows
//!   times `+0xcc` + 1, its velocity (`+0xcc` + 1)·90 a second (75 with the option) along its first row, its class
//!   sound 1; then 12 ticks of its first row over the shrink's (alpha back in) and 30 of it over the stretch's (its
//!   velocity its speed), then its rows rebuilt and 1. Its loop sound kept playing.
//! * **2** (put away): the camera out of its area, shown again with its collision and the state it had.
//! * Every tick its move into its platform block (`0x33d4d0` = L00 `0x322668`) and Ratchet carried.
//!
//! The engine functions with the port's: `0x3368c0` = L00 `0x31b8d0` (set_state), `0x3458a0` = L00 `0x32a7c8`
//! (first), `0x345920` = L00 `0x32a848` (crate::level::LevelData::in_area), `0x3092b0` = L00 `0x2eeab0`
//! (crate::moby::countdown), `0x309928` = L00 `0x2ef138` (set_len), `0x3096b0` = L00 `0x2eeee0` (scale),
//! `0x309b08` = L00 `0x2ef318` (through the rows), `0x30a1a0` = L00 `0x2ef9b0` (the transpose), `0x30a650` = L00
//! `0x2efe60` (add_rot), `0x30a7f0` = L00 `0x2f0000` (crate::hud::lerp_rgba), `0x37c5d8` = L00 `0x3602f0`.
//!
//! **Its particles** (`0x35a848` = L02 `0x345d88`, crate::particles::keyed): on the warps' first ticks, drawn, 1..5
//! puffs at the warp's point ([`warp_puffs`], `0x3fe1e0`); 0x0d9b's exhaust every tick of its run ([`exhaust`],
//! `0x3fe0b8`).
//!
//! Not ported: the warp's draws (6 and 7, drawn: `0x302130(0x3feaa0)`, `0x302130(0x3fdea8)`, `0x301f28(0x3feba0)`;
//! strips and a glow of the warp's size and colour, not the particles). A leader's collision is shared as its flag
//! (the game copies its pointer `+0x98`).

use crate::hero::physics::{add, add_rot, dist, scale, set_len, sub, V3};
use crate::moby::blast::{self, Blast};
use crate::moby::hits::HitTemplate;
use crate::moby::{Ctx, MobyWorld};
use crate::moby::classes::level08::tabora_ai::jitter;
use crate::particles::keyed::{keyed, Keys};

pub const CAR: i16 = 0x11cf;
pub const RAM: i16 = 0x0d9b;
/// The class whose sounds it plays.
const SOUNDS: i16 = 0x0d5c;

const START: usize = 0x60;
const START_ROT: usize = 0x70;
const VEL: usize = 0x80;
const WARP_AT: usize = 0x90;
const OFFSET: usize = 0xa0;
const PATH: usize = 0xb0;
const START_CUBOID: usize = 0xb4;
const END_CUBOID: usize = 0xb8;
const COMES_BACK: usize = 0xbc;
const WARP_SIZE: usize = 0xc0;
const WARP_COLOUR: usize = 0xc4;
const LEADER: usize = 0xc8;
const WARP_IN: usize = 0xcc;
const SPEED: usize = 0xd0;
const AWAY_AREA: usize = 0xd4;
const T: usize = 0xd8;
const RATE: usize = 0xdc;
const DEATH: usize = 0xe0;
const LOOP: usize = 0xe4;

/// `$gp+0x15a0` / `+0x15a4`: the drop's gravity (a second²) and turn (degrees a second).
const FALL: (f32, f32) = (100.0, 90.0);
/// `$gp+0x15b0` / `+0x15c0`: the ram's sphere (in its frame) and radius; `+0x15c4` / `+0x15c8` the pushes of the
/// others and of Ratchet; `+0x15cc` his death's countdown.
const RAM_AT: V3 = [5.5, 0.0, -0.2];
const RAM_R: f32 = 2.0;
const PUSH: f32 = 0.1;
const PUSH_HERO: f32 = 20.0;
const DEATH_TICKS: i32 = 30;
/// `$gp+0x15e0`: the sparks' place in its frame; `+0x15f0` how many; `+0x15f4` / `+0x15f8` their size;
/// `+0x15fc` / `+0x1600` their speed (a second).
const SPARK_AT: V3 = [5.8, 0.0, -0.8];
const SPARKS: i32 = 10;
const SPARK_SIZE: (f32, f32) = (60000.0, 120000.0);
const SPARK_SPEED: (f32, f32) = (2.0, 6.0);
/// `$gp+0x1604` / `+0x1608`: the warp-out's speeds (a second); `+0x1610` the warp-in's (by the option `0x1a7b98`).
const WARP_OUT_SPEED: (f32, f32) = (30.0, 1.0);
const WARP_IN_SPEED: [f32; 2] = [90.0, 75.0];
/// `$gp+0x1618` / `+0x1628`: the stretch's and the shrink's rates (a second, by the option); `+0x1620` / `+0x1630`
/// their ticks.
const STRETCH: [f32; 2] = [0.6, 0.6];
const SHRINK: [f32; 2] = [15.0, 13.5];
const STRETCH_TICKS: f32 = 30.0;
const SHRINK_TICKS: f32 = 12.0;
/// `$gp+0x1640` / `+0x1650`: the warp's points (out, in) in its frame.
const WARP_OUT_AT: V3 = [16.0, 0.0, 0.5];
const WARP_IN_AT: V3 = [0.0, 0.0, 0.5];
/// `$gp+0x1668`, `0x1b065c`, `0x1b0660`: the warp's sizes; `$gp+0x1678`, `0x1b066c`, `0x1b0670` its colours.
const WARP_SIZES: [f32; 3] = [0.0, 8.0, 0.0];
const WARP_COLOURS: [u32; 3] = [0x00ff_2020, 0x7fff_ffff, 0x7f30_6000];
/// `0x3fce2c`: the first row's length at the warp-in's start.
const WARP_IN_LEN: f32 = 0.032_34;
/// `0x3fdcfc`: the drop's floor.
const FLOOR: f32 = 85.0;
const NO_LIMIT: f32 = 5_627.925;
/// `$gp+0x15d0`: the exhaust's place in its frame.
const EXHAUST_AT: V3 = [8.0, 0.0, 1.5];
/// `$gp+0x16b8` (`0x1b06a8`): the exhaust's template: dark blue → grey → dark, 300 → 400 → 600, over 15 ticks.
static EXHAUST_KEYS: Keys = Keys { colors: &[0x0000_2030, 0x7f60_6060, 0x0023_2323, 0], sizes: &[300, 400, 600, 0], times: &[0, 5, 15, 0], last: 3, life: 15, scale: 1.0 };
/// `$gp+0x16f0` (`0x1b06e0`): the warp's puffs' template: white → orange → yellow → white, 0 → 3000 → 1000 → 0, over
/// 60 ticks.
static PUFF_KEYS: Keys = Keys { colors: &[0x00ff_ffff, 0x4060_3000, 0x7fff_ff00, 0x00ff_ffff], sizes: &[0, 3000, 1000, 0], times: &[0, 15, 45, 60], last: 4, life: 60, scale: 1.0 };
const PER_TICK: f32 = 0.016_666_668;

/// `0x2ef318`: `v` through the rows.
fn through(v: V3, r: &[V3; 3]) -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) }

fn idx(w: &MobyWorld, k: i32) -> Option<usize> { usize::try_from(k).ok().filter(|&k| k < w.mobys.len()) }

/// `0x3fddc8(m, gone)`: put away — 3 (`gone`) or 2, without collision, hidden (mode | 0x41), not a target.
fn put_away(w: &mut MobyWorld, id: usize, gone: bool) {
    let m = &mut w.mobys[id];
    m.set_state(if gone { 3 } else { 2 }, -1);
    m.has_collision = false;
    m.mode = (m.mode | 0x41) & 0xefff;
}

/// Its loop sound 0 (0x0d5c's) kept playing.
fn keep_loop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(LOOP);
    if h == -1 || !ctx.host.sound_playing(h) {
        let h = ctx.host.play_sound_of(&w.mobys[id], SOUNDS, 0, 4);
        w.mobys[id].set_pv_i(LOOP, h);
    }
}

/// The options' first word `0x1a7b98` (Progressive Scan, crate::menus::special; the port leaves it 0) picks the second
/// of a pair.
fn opt(_ctx: &Ctx) -> usize { 0 }

/// 0x11cf / 0x0d9b (module docs).
pub fn update_11cf(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let area = w.mobys[id].pv_i(AWAY_AREA);
    let cam = ctx.host.camera_pos();
    if area != -1 && ctx.level.in_area(area, cam) { put_away(w, id, false); }
    {
        let m = &mut w.mobys[id];
        let mut d = m.pv_i(DEATH);
        if 0 < d {
            crate::moby::countdown(&mut d);
            m.set_pv_i(DEATH, d);
            if d < 2 { ctx.hero.die(); }
        }
    }
    let class = w.mobys[id].o_class;
    match w.mobys[id].state {
        0 => {
            let leader = idx(w, w.mobys[id].pv_i(LEADER));
            let lead = leader.map(|l| (w.mobys[l].pos, w.mobys[l].rows));
            let path = w.mobys[id].pv_i(PATH);
            let pts = usize::try_from(path).ok().and_then(|i| ctx.level.paths.get(i)).filter(|p| 1 < p.len());
            let m = &mut w.mobys[id];
            let (p, r) = (m.pos, m.rot);
            m.set_pv_v(START, p);
            m.set_pv_v(START_ROT, r);
            if let Some(pts) = pts {
                m.set_pv_f(T, 0.0);
                let a = [pts[0][0], pts[0][1], pts[0][2]];
                let b = [pts[1][0], pts[1][1], pts[1][2]];
                m.set_pv_f(RATE, 1.0 / dist(a, b) * PER_TICK * m.pv_f(SPEED));
                m.pos = a;
                m.set_pv_i(DEATH, -1);
            }
            if class != RAM && class != CAR {
                m.set_pv_i(0x08, 0);
            }
            if let Some((lp, lr)) = lead {
                // `0x2ef9b0` then `0x2ef318`: the offset through the leader's rows transposed.
                let d = sub(m.pos, lp);
                m.set_pv_v(OFFSET, std::array::from_fn(|i| crate::hero::physics::dot(lr[i], d)));
            }
            if m.pv_i(START_CUBOID) != -1 {
                m.mode |= 1;
                m.set_state(4, -1);
            } else {
                m.set_state(1, -1);
            }
        }
        4 => {
            let m = &mut w.mobys[id];
            if m.first() { m.mode |= 1; }
            if ctx.level.in_cuboid(m.pv_i(START_CUBOID), ctx.hero.pos) {
                if m.pv_i(LEADER) == -1 {
                    m.mode |= 0x100;
                    m.rows[0] = set_len(m.rows[0], WARP_IN_LEN);
                    m.set_state(7, -1);
                } else {
                    m.set_state(1, -1);
                }
            }
        }
        1 => {
            run(w, id, ctx);
            if class == RAM { ram(w, id, ctx); }
        }
        5 => {
            let m = &mut w.mobys[id];
            m.first();
            let mut v = m.pv_v(VEL);
            v[2] -= FALL.0 * 0.000_277_777_78;
            m.set_pv_v(VEL, v);
            m.rot[1] = add_rot(m.rot[1], FALL.1 * 0.000_290_888_25);
            m.pos = add(m.pos, v);
        }
        6 => warp_out(w, id, ctx),
        7 => warp_in(w, id, ctx),
        2 => {
            let area = w.mobys[id].pv_i(AWAY_AREA);
            if area != -1 && !ctx.level.in_area(area, cam) {
                let solid = w.class(class).is_some_and(|c| c.has_collision);
                let m = &mut w.mobys[id];
                m.mode = m.mode & 0xffbe | 0x1000;
                m.has_collision = solid;
                m.state = m.prev_state;
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    let (pos, rot) = (m.pos, m.rot);
    crate::moby::platform::write(m, 0x20, sub(pos, old), old_rot, rot);
    let flags = m.pv_i(0x20 + 0x3c) as u32;
    crate::moby::carry_hero_with(ctx, id, flags, old, old_rot[2], pos, rot[2]);
    if pos[2] < FLOOR && w.mobys[id].state == 5 {
        blast::burst(w, ctx, pos, &Blast { size: 4.0, bursts: [5, 0, 5], puffs: 0x14, streaks: 0, light: true, shake: false });
        put_away(w, id, true);
    }
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) <= 0xfffe { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

/// State 1's run (module docs).
fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let class = w.mobys[id].o_class;
    {
        let m = &mut w.mobys[id];
        if m.first() {
            let v = set_len(m.rows[0], m.pv_f(SPEED) * PER_TICK);
            m.set_pv_v(VEL, v);
        }
    }
    let path = w.mobys[id].pv_i(PATH);
    if path != -1 {
        if let Some(pts) = usize::try_from(path).ok().and_then(|i| ctx.level.paths.get(i)).filter(|p| 1 < p.len()) {
            let m = &mut w.mobys[id];
            let (p, pitch, yaw, roll) = crate::paths::pose(pts, m.pv_f(T), false);
            m.pos = p;
            m.rot = [roll, pitch, yaw];
            m.set_pv_f(T, m.pv_f(T) + m.pv_f(RATE));
        }
        return;
    }
    if let Some(l) = idx(w, w.mobys[id].pv_i(LEADER)) {
        let lm = &w.mobys[l];
        let (mode, alpha, solid, rows, rot, lp) = (lm.mode, lm.alpha, lm.has_collision, lm.rows, lm.rot, lm.pos);
        let m = &mut w.mobys[id];
        m.mode = mode;
        m.alpha = alpha;
        m.has_collision = solid;
        m.rows = rows;
        m.rot = rot;
        m.pos = add(through(m.pv_v(OFFSET), &rows), lp);
    } else {
        let m = &mut w.mobys[id];
        m.pos = add(m.pos, m.pv_v(VEL));
    }
    keep_loop(w, id, ctx);
    let m = &w.mobys[id];
    let end = m.pv_i(END_CUBOID);
    if end != -1 && m.pv_i(LEADER) == -1 && ctx.level.in_cuboid(end, m.pos) {
        let h = m.pv_i(LOOP);
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        let m = &mut w.mobys[id];
        m.set_pv_i(LOOP, -1);
        m.set_state(if class == RAM || class == CAR { 5 } else { 6 }, -1);
    }
}

/// 0x0d9b's ram and sparks (module docs).
fn ram(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (rows, pos) = (m.rows, m.pos);
    let at = add(through(RAM_AT, &rows), pos);
    let near = ctx.scene.map(|sc| crate::collision::coll_sphere_mobys(sc, at, RAM_R, crate::collision::QueryFlags(0), Some(id))).unwrap_or_default();
    for t in near {
        if Some(t) == ctx.hero_moby {
            let d = scale(rows[0], -PUSH_HERO);
            ctx.hero.request_state(0, true);
            let damage = (ctx.game.max_health as f32 * 30.0).max(100.0);
            let hp = ctx.hero.pos;
            let tmpl = HitTemplate::new(damage, Some(id), 0x10001, [d[0], d[1], PUSH_HERO * 4.0, NO_LIMIT]);
            w.deliver_hit_any(t, &tmpl, [hp[0], hp[1], hp[2], 1.0]);
            w.mobys[id].set_pv_i(DEATH, DEATH_TICKS);
            blast::burst(w, ctx, hp, &Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true });
        } else {
            let d = scale(rows[0], PUSH);
            let tp = w.mobys[t].pos;
            let tmpl = HitTemplate::new(50.0, Some(id), 0x10001, [d[0], d[1], PUSH * 4.0, NO_LIMIT]);
            w.deliver_hit_any(t, &tmpl, [tp[0], tp[1], tp[2], 1.0]);
        }
    }
    exhaust(w, id, ctx);
    if w.mobys[id].drawn == 0 || (ctx.game.frame & 0xf) as usize != id & 0xf { return; }
    for _ in 0..SPARKS {
        let m = &w.mobys[id];
        let p = add(through(SPARK_AT, &m.rows), m.pos);
        let r1 = m.rows[1];
        let f = ctx.game.rng.randf(SPARK_SPEED.0, SPARK_SPEED.1) * PER_TICK;
        let f = if ctx.game.rng.randi(2) != 0 { -f } else { f };
        let mut v = set_len(r1, f);
        v[2] = ctx.game.rng.randf(-0.083_333_336, PER_TICK);
        let size = ctx.game.rng.randf(SPARK_SIZE.0, SPARK_SIZE.1);
        if let Some(sys) = ctx.parts.as_deref_mut() {
            if let Some(i) = crate::particles::lava::spark(sys, &mut ctx.game.rng, size, p, v, false) {
                crate::particles::rec::set_ff(&mut sys.pool.recs[i], 0x2c, -0.008_333_334);
            }
        }
    }
}

/// `0x3fe0b8`: 0x0d9b's exhaust: 1..2 puffs at [`EXHAUST_AT`] in its frame jittered 0.5, the one velocity (first
/// straight up) set to 8..12 a second and jittered 1/15 for each, additive or not at random, turning −2..2.
fn exhaust(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let (rows, pos) = (w.mobys[id].rows, w.mobys[id].pos);
    let g = &mut ctx.game.rng;
    let mut v: V3 = [0.0, 0.0, 1.0];
    for _ in 0..g.randi(2) + 1 {
        let mut p = add(through(EXHAUST_AT, &rows), pos);
        jitter(g, &mut p, 0.5);
        v = set_len(v, g.randf(8.0, 12.0) * PER_TICK);
        jitter(g, &mut v, 0.066_666_67);
        let additive = g.randi(2) != 0;
        let spin = g.rand_range(-2, 2);
        if let Some(sys) = ctx.parts.as_deref_mut() { keyed(sys, g, p, v, &EXHAUST_KEYS, additive, spin); }
    }
}

/// `0x3fe1e0`: the warp's puffs: 1..5 at the warp's point jittered 0.5, the one velocity (first none) jittered
/// 1/120 for each, additive, turning −5..5.
fn warp_puffs(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let at = w.mobys[id].pv_v(WARP_AT);
    let g = &mut ctx.game.rng;
    let mut v: V3 = [0.0; 3];
    for _ in 0..g.randi(5) + 1 {
        let mut p = at;
        jitter(g, &mut p, 0.5);
        jitter(g, &mut v, 0.008_333_334);
        let spin = g.rand_range(-5, 5);
        if let Some(sys) = ctx.parts.as_deref_mut() { keyed(sys, g, p, v, &PUFF_KEYS, true, spin); }
    }
}

/// The warp's size and colour at share `t` from step `a` to `b` of [`WARP_SIZES`] / [`WARP_COLOURS`].
fn warp_look(w: &mut MobyWorld, id: usize, t: f32, a: usize, b: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_f(WARP_SIZE, WARP_SIZES[a] + (WARP_SIZES[b] - WARP_SIZES[a]) * t);
    m.set_pv_i(WARP_COLOUR, crate::hud::lerp_rgba(t, WARP_COLOURS[a], WARP_COLOURS[b]) as i32);
}

/// State 6 (module docs).
fn warp_out(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let k = opt(ctx);
    let ticks = w.mobys[id].state_ticks as u16 as f32;
    if w.mobys[id].first() {
        let m = &mut w.mobys[id];
        m.timer = 0;
        m.state_ticks = 0;
        m.mode |= 0x100;
        m.set_pv_f(WARP_SIZE, WARP_SIZES[0]);
        let at = add(m.pos, through(WARP_OUT_AT, &m.rows));
        m.set_pv_v(WARP_AT, at);
        m.set_pv_v(VEL, set_len(m.rows[0], WARP_OUT_SPEED.0 * PER_TICK));
        m.mode |= 8;
        ctx.host.play_sound_of(&w.mobys[id], SOUNDS, 1, 0);
        if w.mobys[id].drawn != 0 { warp_puffs(w, id, ctx); }
    } else if w.mobys[id].timer == 0 {
        let m = &mut w.mobys[id];
        m.rows[0] = scale(m.rows[0], STRETCH[k] * PER_TICK + 1.0);
        warp_look(w, id, ticks / STRETCH_TICKS, 0, 1);
        if STRETCH_TICKS < ticks {
            let m = &mut w.mobys[id];
            m.timer = 1;
            m.set_pv_v(VEL, set_len(m.rows[0], WARP_OUT_SPEED.1 * PER_TICK));
            m.state_ticks = 0;
        }
    } else if w.mobys[id].timer == 1 {
        let m = &mut w.mobys[id];
        m.rows[0] = scale(m.rows[0], 1.0 - SHRINK[k] * PER_TICK);
        warp_look(w, id, ticks / SHRINK_TICKS, 1, 2);
        let m = &mut w.mobys[id];
        m.alpha = ((1.0 - ticks / SHRINK_TICKS) * 128.0) as i32 as u8;
        if SHRINK_TICKS < ticks || m.rows[2][2] < 0.0 {
            if m.pv_i(COMES_BACK) != 0 {
                m.set_pv_f(T, 0.0);
                m.pos = m.pv_v(START);
                m.rot = m.pv_v(START_ROT);
                m.set_state(7, -1);
            } else {
                m.mode &= !0x100;
                put_away(w, id, true);
            }
        }
    }
    let m = &mut w.mobys[id];
    m.pos = add(m.pos, m.pv_v(VEL));
}

/// State 7 (module docs).
fn warp_in(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let k = opt(ctx);
    let ticks = w.mobys[id].state_ticks as u16 as f32;
    if w.mobys[id].first() {
        let m = &mut w.mobys[id];
        m.timer = 1;
        m.state_ticks = 0;
        m.mode = (m.mode & !1) | 0x100;
        m.set_pv_f(WARP_SIZE, WARP_SIZES[0]);
        m.alpha = 0;
        let unit: [V3; 3] = std::array::from_fn(|i| set_len(m.rows[i], 1.0));
        let s = m.pv_i(WARP_IN) as f32 + 1.0;
        let at = add(scale(through(WARP_IN_AT, &unit), s), m.pos);
        m.set_pv_v(WARP_AT, at);
        m.set_pv_v(VEL, set_len(m.rows[0], s * (PER_TICK * WARP_IN_SPEED[k])));
        m.mode |= 8;
        if w.mobys[id].drawn != 0 { warp_puffs(w, id, ctx); }
        ctx.host.play_sound_of(&w.mobys[id], SOUNDS, 1, 0);
    } else if w.mobys[id].timer == 1 {
        let m = &mut w.mobys[id];
        m.rows[0] = scale(m.rows[0], 1.0 / (1.0 - SHRINK[k] * PER_TICK));
        warp_look(w, id, ticks / SHRINK_TICKS, 2, 1);
        let m = &mut w.mobys[id];
        m.alpha = (ticks * 128.0 / SHRINK_TICKS) as i32 as u8;
        if SHRINK_TICKS < ticks || m.rows[2][2] < 0.0 {
            m.alpha = 0x80;
            m.timer = 0;
            m.mode &= !8;
            m.set_pv_v(VEL, set_len(m.rows[0], m.pv_f(SPEED) * PER_TICK));
            m.state_ticks = 0;
        }
    } else if w.mobys[id].timer == 0 {
        let m = &mut w.mobys[id];
        m.rows[0] = scale(m.rows[0], 1.0 / (STRETCH[k] * PER_TICK + 1.0));
        warp_look(w, id, ticks / STRETCH_TICKS, 1, 0);
        if STRETCH_TICKS < ticks {
            let m = &mut w.mobys[id];
            m.mode &= !0x100;
            m.set_state(1, -1);
        }
    }
    keep_loop(w, id, ctx);
    let m = &mut w.mobys[id];
    m.pos = add(m.pos, m.pv_v(VEL));
}
