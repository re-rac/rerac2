//! The Jamming Array's node, class **0x0a79** ([`update_0a79`], `0x3c5210`, 6 placed; the decomp's v2.00
//! `func_US2_L16_003C404C`, shared with L16 `0x3c4050`, where none is placed): it rises out along its up row as
//! another moby's amount grows, and once fully out a hit blows it up; the last of its group done, its mission is.
//! It works only while the gravity sphere is on (`0x1a8ff0`, crate::gravity).
//!
//! Its pvars: `+0x00` the moby whose amount (its pvar `+0x34`, `0x39b050`) it follows, `+0x10` its place at the set-up,
//! `+0x20` / `+0x24` the pulse's phase and rate, `+0x28` / `+0x2c` a count and the timer, `+0x40` flags, `+0x44` its
//! looped sound (−1 none), `+0x48` the amount it had.
//!
//! * **0** (set-up): its mission done (`0x19b2e8 + level·0x10 + +0xb0`, bit 7) → deleted. Update distance 0xff, draw
//!   distance 0x113, mode | 0x4000; its place kept; the amount 0, no sound, the rate `$gp+0xf08` (360) × π/10800;
//!   then 1.
//! * **1** (with the sphere on): the amount `a` read.
//!   * `a` or the last amount not 0: its sparks ([`sparks`], `0x3c5d08` = L14 `0x4288b8`); its place `+0x10` + its up
//!     row at `4 · a · its scale / its class's`; with `a` below the last, a hit sphere (`0x31d338`: radius 2.1, damage
//!     3, push 1, flags 0x10000, 0, 1, 0).
//!   * `a` changing (not the last, not 0 or 1): its class sound 2 looped (mode 4) unless its own plays; else that one
//!     stopped (`0x35fd60`).
//!   * `a` reaching 1: the help text 0x9ec in slot 0x43 (`0x2f5550`, crate::game::GameState::help).
//!   * Fully out (`a` ≥ 1): a hit with the flags 0x10000 (`0x31dea0`): its class sound 0, the record let go, 2,
//!     `0x18cc7c` = 0xffff. Otherwise the record is let go.
//! * **2** (blowing up): its ticks `+0x96` counted; past 60, two big explosions (`0x3eeb48` = L02 `0x425300`,
//!   crate::moby::classes::explosion::start, no ground) 2 down its up row and 5 up from there, each facing the camera
//!   (`0x1b8b00`); a hit sphere there (radius 6, damage 3, push 1, flags 0x10000); its class sound 1; the shakes
//!   (`0x1b8b20` 0.2 for 60 ticks, the roll `0x1b8b40` 0.1 for 90); its angles from its rows (`0x3201b8` = L00
//!   `0x320000`); every piece (`0x326c50` = crate::moby::shards::break_into: 2..4 out, 10..20 up, no velocity, no
//!   sound); the timer 180, hidden and untargeted (mode | 0x41, & ~0x1000). With others of its group left
//!   (`0x31ca98` = crate::moby::classes::props::group_alive, less itself), "text 0x2dd0 n" (`"%s %d"`) for 180 ticks
//!   (`0x30f1c0`, crate::prompt::Prompt::announce_count) and 4; else text 0x2dd1 and 5. Before then its class sound 3
//!   on each 20th tick (at 10).
//! * **4**: the timer out (`0x2eed80`): deleted.
//! * **5**: the timer out: its mission done (`0x3134b0`, when it has one and Ratchet has health `0x18c2ec`), deleted.
//! * **Drawn** (`+0x31`): the glow `+0x78` pulses (`0x326a60` = L01 `0x31ce88`: the colours lerped by
//!   `sin(phase)·0.5 + 0.5`) between 0x80c0c0c0 and 0x80808080, the phase on by the rate; in state 2 between
//!   0x803030c0 and 0x80202020 at `$gp+0xf0c` (720) × π/10800.
//!
//! Not ported: the group's draw (`0x3c5910` = L16 `0x3c4750`, the decomp's `func_L16_003C32F8`: once a frame, the
//! frame number in `0x1b2814`, each member in state 1 drawn turning inside a 30-sphere through its own packets; a
//! custom render path); `0x18cc7c` (not mapped). The count's text is drawn "text n " (the port's count carries a third
//! text; none here).

use crate::hero::physics::{add, scale, set_len, sub, unit, V3};
use crate::hud::lerp_rgba;
use crate::moby::attach::angles_of;
use crate::moby::classes::props::group_alive;
use crate::moby::{ai, countdown, shards, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0a79;

const LINK: usize = 0x00;
const ORIGIN: usize = 0x10;
const PHASE: usize = 0x20;
const RATE: usize = 0x24;
const COUNT: usize = 0x28;
const TIMER: usize = 0x2c;
const FLAGS: usize = 0x40;
const SOUND: usize = 0x44;
const AMOUNT: usize = 0x48;

/// π/10800: the rates' unit (`0x3998825c`).
const UNIT: f32 = 0.000_290_888_25;
/// `$gp+0xf00` / `+0xf08` / `+0xf0c`: the rise at the full amount, the pulse's rates.
const RISE: f32 = 4.0;
const RATE_1: f32 = 360.0;
const RATE_2: f32 = 720.0;
/// `$gp+0xf10..+0xf1c`: the pulse's colours (states 1 and 2).
const COLOURS: [u32; 4] = [0x80c0_c0c0, 0x8080_8080, 0x8030_30c0, 0x8020_2020];

fn mission_done(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let k = ctx.game.level.max(0) as usize * 0x10 + w.mobys[id].mission as usize;
    ctx.game.missions.get(k).is_some_and(|b| b & 0x80 != 0)
}

pub fn update_0a79(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if mission_done(w, id, ctx) {
                w.delete(id);
                return;
            }
            let m = &mut w.mobys[id];
            m.update_dist = 0xff;
            m.draw_dist = 0x113;
            m.mode |= 0x4000;
            let p = m.pos;
            m.set_pv_v(ORIGIN, p);
            m.set_pv_f(AMOUNT, 0.0);
            m.set_pv_i(SOUND, -1);
            m.set_pv_f(RATE, RATE_1 * UNIT);
            m.set_state(1, -1);
        }
        1 => if ctx.hero.gravity.on { rising(w, id, ctx) },
        2 => blow_up(w, id, ctx),
        4 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(TIMER);
            let done = countdown(&mut t);
            m.set_pv_i(TIMER, t);
            if done {
                w.delete(id);
                return;
            }
        }
        5 => {
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(TIMER);
            let done = countdown(&mut t);
            m.set_pv_i(TIMER, t);
            if done {
                let mission = m.mission;
                if mission != 0xff && ctx.hero.hp != 0 { ctx.game.mission_done(mission); }
                w.delete(id);
                return;
            }
        }
        _ => {}
    }
    pulse(w, id);
}

/// State 1 (module docs).
fn rising(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let link = usize::try_from(w.mobys[id].pv_i(LINK)).ok();
    let a = link.and_then(|k| w.mobys.get(k)).map_or(0.0, |o| o.pv_f(0x34));
    let last = w.mobys[id].pv_f(AMOUNT);
    if a != 0.0 || last != 0.0 {
        sparks(w, id, ctx);
        let class_scale = w.class(CLASS).map_or(1.0, |c| c.scale);
        let m = &mut w.mobys[id];
        let up = set_len(m.rows[2], RISE * a * (m.scale / class_scale));
        m.pos = add(m.pv_v(ORIGIN), up);
        if a < last {
            let pos = m.pos;
            ai::attack_sphere(w, id, ctx, f32::from_bits(0x4006_6666), 3.0, 1.0, pos, 0x1_0000, 0, 1, 0);
        }
    }
    let h = w.mobys[id].pv_i(SOUND);
    if a != last && a != 1.0 && a != 0.0 {
        if !ctx.host.sound_playing(h) {
            let s = ctx.host.play_sound(&w.mobys[id], 2, 4);
            w.mobys[id].set_pv_i(SOUND, s);
        }
    } else {
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(SOUND, -1);
    }
    if a == 1.0 && last != 1.0 { ctx.game.help(0x9ec, 0x43); }
    w.mobys[id].set_pv_f(AMOUNT, a);
    if 1.0 <= a {
        if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x1_0000 != 0) {
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.clear_hit(id);
            w.mobys[id].set_state(2, -1);
        }
    } else {
        w.clear_hit(id);
    }
}

/// State 2 (module docs).
fn blow_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let old = m.state_ticks as u16;
    let ticks = old.wrapping_add(1);
    m.state_ticks = ticks as i16;
    let (pos, up) = (m.pos, m.rows[2]);
    if old < 60 {
        if ticks % 20 == 10 { ctx.host.play_sound(&w.mobys[id], 3, 0); }
        return;
    }
    let mut at = add(set_len(up, -2.0), pos);
    let step = set_len(up, 5.0);
    ai::attack_sphere(w, id, ctx, 6.0, 3.0, 1.0, at, 0x1_0000, 0, 1, 0);
    ctx.host.play_sound(&w.mobys[id], 1, 0);
    for _ in 0..2 {
        let dir = unit(sub(ctx.host.camera_pos(), at));
        crate::moby::classes::explosion::start(w, ctx, 0.0, at, dir);
        at = add(at, step);
    }
    let s = &mut ctx.game.shakes;
    s.0[2].amp = 0.1;
    s.0[2].timer = 90;
    s.ask(0, 0.2, 60);
    let m = &mut w.mobys[id];
    m.rot = angles_of(&m.rows);
    shards::break_into(w, id, ctx, (2.0, 4.0), (10.0, 20.0), 0xfff, [0.0; 3], 0xff);
    let m = &mut w.mobys[id];
    m.set_pv_i(COUNT, 0);
    m.set_pv_i(TIMER, 180);
    m.set_pv_i(FLAGS, 0);
    m.mode = (m.mode | 0x41) & 0xefff;
    let left = group_alive(w, w.mobys[id].group as i32) as i32 - 1;
    if left != 0 {
        w.mobys[id].set_state(4, -1);
        ctx.game.prompt.announce_count(0x2dd0, left, 0, 180);
    } else {
        ctx.game.prompt.announce(0x2dd1, 180);
        w.mobys[id].set_state(5, -1);
    }
}

/// The glow's pulse while drawn (module docs).
fn pulse(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    if m.drawn == 0 { return; }
    let (rate, a, b) = if m.state == 2 { (RATE_2, COLOURS[2], COLOURS[3]) } else { (RATE_1, COLOURS[0], COLOURS[1]) };
    let p = crate::hero::physics::add_rot(m.pv_f(PHASE), rate * UNIT);
    m.set_pv_f(PHASE, p);
    m.glow = lerp_rgba(p.sin() * 0.5 + 0.5, a, b);
}

/// `$gp+0x1000` / `+0x1008` / `+0x1010` / `+0x1020` / `+0x1030`: the sparks' sizes, colours, life ranges, the share of
/// the big kind, and the two ends of the line they run along (in its frame).
const SPARK_SIZE: [f32; 2] = [0.5, 2.0];
const SPARK_RGBA: [u32; 2] = [0x6000_6080, 0x6000_4090];
const SPARK_LIFE: [(i32, i32); 2] = [(0x2d, 0x3c), (0x46, 0x5a)];
const SPARK_BIG: f32 = 0.2;
const SPARK_ENDS: [V3; 2] = [[0.0, 0.0, -4.0], [0.0, 0.0, -1.5]];

/// `0x3c5d08` (L14 `0x4288b8`, the decomp's `func_L14_004272D8`): a glint (`0x33b050` = L00 `0x33b5e0`,
/// crate::particles::glint) at one end of its line (`0x31bb58(2)`), running to the other over its life (one time in
/// five, `0x31bbf0(0, 1)` < 0.2, the big kind), its roll `0x31bb58(0xff)`.
fn sparks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (pos, rows, hero) = (m.pos, m.rows, ctx.hero.pos);
    let rng = &mut ctx.game.rng;
    let side = rng.randi(2) as usize;
    let kind = usize::from(rng.randf(0.0, 1.0) < SPARK_BIG);
    let n = rng.rand_range(SPARK_LIFE[kind].0, SPARK_LIFE[kind].1);
    let through = |v: V3| -> V3 { std::array::from_fn(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]) };
    let at = add(through(SPARK_ENDS[side]), pos);
    let step = scale(through(sub(SPARK_ENDS[(side + 1) & 1], SPARK_ENDS[side])), 1.0 / n as f32);
    let rot = rng.randi(0xff) as u8;
    if let Some(sys) = ctx.parts.as_deref_mut() {
        crate::particles::glint(sys, SPARK_SIZE[kind], at, step, SPARK_RGBA[kind], (n & 0xff) as u16, rot, false, hero);
    }
}
