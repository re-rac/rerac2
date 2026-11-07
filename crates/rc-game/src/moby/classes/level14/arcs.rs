//! The prison's electric arcs **0x0b10** (6 placed; `0x391f00`, the decomp's `func_L03_0038A6D8`: Endako's code too,
//! L03 `0x38bba8`, where none is placed): four arcs out of a point 1.1 over it, each a core and a glow ribbon along a
//! rope that waves, sways and jitters, pulsing in a cycle of three states; it closes the record of the robots' way
//! network under it (crate::moby::classes::endako_d's 0x0f2d) until it is hooked, when it bursts and is gone.
//!
//! **Its pvars:** `+0x00` home, `+0x10` the network moby (−1 none) and `+0x14` it closes it, `+0x18` / `+0x1c` /
//! `+0x20` the ticks of the states 2 / 3 / 1 and `+0x24` the timer, `+0x28..+0x34` the four arcs' paths (−1: a set
//! line), `+0x38` the ropes' points (16 at most), `+0x3c` flags (1 the paths, 2 the shake, 4 the swell, 8 the pull
//! toward Ratchet, 0x10 the paths moved onto it, 0x20 not drawn, 0x40 no jitter, 0x80 no wave, 0x100 no sway),
//! `+0x40` / `+0x44` the core's and glow's colours (`+0x48` / `+0x4c` the second ribbons'), `+0x50` / `+0x54` their
//! widths (`+0x58` / `+0x5c`), `+0x60..+0x7c` the same at full pulse, `+0x80` the glow's texture (16 when < 0),
//! `+0x84` / `+0x88` the core's scroll and its rate, `+0x90` the jitter, `+0x94` / `+0xa8` the wave's phases, `+0x98`
//! their rate, `+0x9c` the core's texture (14 when < 0), `+0xa0` the wave and `+0xa4` the sway (0.25 / 1), `+0xac` /
//! `+0xb0` the swell and its reach, `+0xcc` the pulse's width, `+0xd0` the shake, `+0xd4` the pull's reach, `+0xd8`
//! its looped sound, `+0xdc` its slot in the ropes' pool (× 8 at the set-up), `+0xe0` the arcs' centre (1.1 up).
//!
//! **0** the set-up (`0x392ef0`): home its place; with a network and `+0x14`, its record closed (`0x421f60`,
//! crate::moby::classes::endako_d::net_mark); the defaults above, the full-pulse colours 0x807f0000 / 0x8000007f,
//! the jitter's frame 0; with flags 0x11 its paths' points moved to centre on `+0xe0` (`0x392e10`; the port keeps
//! the move as each arc's offset); 1. **1** (`+0x20` ticks): back home on its first tick, then 2. **2** (`+0x18`): the
//! pulse rising, then 3. **3** (`+0x1c`): the pulse running out along the arcs, then 1.
//!
//! **Each tick**, unless flag 0x20: the jitter's vectors drawn again once a frame unless flag 0x40 (`0x392c40`: 16 of
//! `randf_signed(0, 1)` each axis, crate::game::GameState::arc_jitter), the ropes ([`ropes`], `0x393020`) and its
//! draw ([`draw`], `0x392440` on the frame's second list). Hooked: a blast 1 over it (size 1, 20 puffs, 45 streaks,
//! the flash), its record opened, the help 0x61 off (`0x18cde4`), its looped sound stopped, its sound 1, deleted.
//! Else its class sound 0 looped while not playing.

use crate::fx::FxQuad;
use crate::hero::physics::{add, add_rot, atan2, cross, len, scale, set_len, sub, wrap, V3};
use crate::moby::classes::endako_d;
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0b10;

const HOME: usize = 0x00;
const NET: usize = 0x10;
const NET_ON: usize = 0x14;
const TICKS_2: usize = 0x18;
const TICKS_3: usize = 0x1c;
const TICKS_1: usize = 0x20;
const TIMER: usize = 0x24;
const PATHS: usize = 0x28;
const COUNT: usize = 0x38;
const FLAGS: usize = 0x3c;
const GLOW_FX: usize = 0x80;
const SCROLL: usize = 0x84;
const SCROLL_RATE: usize = 0x88;
const JITTER: usize = 0x90;
const PHASE_A: usize = 0x94;
const RATE: usize = 0x98;
const CORE_FX: usize = 0x9c;
const WAVE: usize = 0xa0;
const SWAY: usize = 0xa4;
const PHASE_B: usize = 0xa8;
const SWELL: usize = 0xac;
const SWELL_REACH: usize = 0xb0;
const PULSE: usize = 0xcc;
const SHAKE: usize = 0xd0;
const PULL: usize = 0xd4;
const SOUND: usize = 0xd8;
const SLOT: usize = 0xdc;
const CENTRE: usize = 0xe0;
/// The port's: the ropes' eight rows of points (the original's pool `0x237a80`, 0x100 a row, at its slot) and the
/// four arcs' path offsets (the original moves the level's path points instead).
const POOL: usize = 0x100;
const MOST: usize = 16;
const OFFSETS: usize = POOL + 8 * MOST * 12;
const SIZE: usize = OFFSETS + 4 * 12;

/// The set lines of the arcs without a path (`0x393020`'s switch): up, and three down and out.
const LINES: [V3; 4] = [
    [0.0, 0.0, 1.1],
    [-0.639_534_9, -0.639_534_9, -0.639_534_9],
    [0.639_534_9, -0.639_534_9, -0.639_534_9],
    [0.0, 0.785_714_3, -0.639_534_9],
];

fn count(m: &crate::moby::Moby) -> usize { (m.pv_i(COUNT).max(0) as usize).min(MOST) }

fn point(m: &crate::moby::Moby, row: usize, col: usize) -> V3 { m.pv_v(POOL + (row * MOST + col) * 12) }

fn set_point(m: &mut crate::moby::Moby, row: usize, col: usize, p: V3) { m.set_pv_v(POOL + (row * MOST + col) * 12, p); }

/// `0x392ef0` (module docs).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.pvars.len() < SIZE { m.pvars.resize(SIZE, 0); }
    m.set_pv_f(PHASE_A, 0.0);
    if m.pv_i(CORE_FX) < 0 { m.set_pv_i(CORE_FX, 0xe); }
    if m.pv_i(GLOW_FX) < 0 { m.set_pv_i(GLOW_FX, 0x10); }
    m.set_pv_i(0xbc, 0x8040_2080_u32 as i32);
    m.set_pv_i(0xc0, 0x8000_4880_u32 as i32);
    m.set_pv_f(SWAY, 1.0);
    m.set_pv_f(WAVE, 0.25);
    m.set_pv_i(0x68, 0x807f_0000_u32 as i32);
    m.set_pv_i(0x6c, 0x8000_007f_u32 as i32);
    m.set_pv_f(PHASE_B, 0.0);
    m.set_pv_i(0x60, 0x807f_0000_u32 as i32);
    m.set_pv_i(0x64, 0x8000_007f_u32 as i32);
    let c = add(m.pos, [0.0, 0.0, 1.1]);
    m.set_pv_v(CENTRE, c);
    m.set_pv_i(SOUND, -1);
    m.set_pv_i(SLOT, m.pv_i(SLOT) << 3);
    ctx.game.arc_jitter.0 = 0;
}

/// `0x392e10`: each arc's path moved to centre its first `+0x38` points (their mean, `0x392cf0`) on `+0xe0`.
fn recentre(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let m = &mut w.mobys[id];
    let (n, c) = (count(m), m.pv_v(CENTRE));
    for r in 0..4 {
        let Some(pts) = usize::try_from(m.pv_i(PATHS + 4 * r)).ok().and_then(|p| ctx.level.paths.get(p)) else { continue };
        let mut mean = [0.0; 3];
        if n != 0 {
            for p in pts.iter().take(n) { mean = add(mean, [p[0], p[1], p[2]]); }
            mean = scale(mean, 1.0 / n as f32);
        }
        m.set_pv_v(OFFSETS + 12 * r, sub(c, mean));
    }
}

/// 0x0b10 (module docs).
pub fn update_0b10(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let p = w.mobys[id].pos;
            w.mobys[id].set_pv_v(HOME, p);
            let net = endako_d::network(w, w.mobys[id].pv_i(NET));
            if let Some(n) = net.filter(|_| w.mobys[id].pv_i(NET_ON) != 0) { endako_d::net_mark(w, n, ctx, p, false); }
            setup(w, id, ctx);
            if w.mobys[id].pv_i(FLAGS) & 0x11 == 0x11 { recentre(w, id, ctx); }
            w.mobys[id].set_state(1, -1);
        }
        s @ 1..=3 => {
            let m = &mut w.mobys[id];
            if s == 1 && m.first() { m.pos = m.pv_v(HOME); }
            let mut t = m.pv_i(TIMER);
            if countdown(&mut t) {
                let (ticks, next) = match s { 1 => (TICKS_2, 2), 2 => (TICKS_3, 3), _ => (TICKS_1, 1) };
                t = m.pv_i(ticks);
                m.set_state(next, -1);
            }
            m.set_pv_i(TIMER, t);
            w.changed = true;
        }
        _ => {}
    }
    let flags = w.mobys[id].pv_i(FLAGS);
    if flags & 0x20 == 0 {
        if flags & 0x40 == 0 && ctx.game.arc_jitter.0 < ctx.game.frame {
            let rng = &mut ctx.game.rng;
            ctx.game.arc_jitter.1 = std::array::from_fn(|_| std::array::from_fn(|_| rng.randf_signed(0.0, 1.0)));
            ctx.game.arc_jitter.0 = ctx.game.frame;
        }
        ropes(w, id, ctx);
        w.register_draw(id);
    }
    let h = w.mobys[id].pv_i(SOUND);
    if w.mobys[id].once & 4 != 0 {
        let at = add(w.mobys[id].pos, [0.0, 0.0, 1.0]);
        let b = crate::moby::blast::Blast { size: 1.0, bursts: [0, 0, 0], puffs: 0x14, streaks: 0x2d, light: true, shake: false };
        crate::moby::blast::burst(w, ctx, at, &b);
        let p = w.mobys[id].pos;
        let net = endako_d::network(w, w.mobys[id].pv_i(NET));
        if let Some(n) = net.filter(|_| w.mobys[id].pv_i(NET_ON) != 0) { endako_d::net_mark(w, n, ctx, p, true); }
        if let Some(c) = ctx.game.help_counts.get_mut(0x61) { *c = -1; }
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        w.mobys[id].set_pv_i(SOUND, -1);
        ctx.host.play_sound(&w.mobys[id], 1, 0);
        w.delete(id);
    } else if !(h != -1 && ctx.host.sound_playing(h)) {
        let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
        w.mobys[id].set_pv_i(SOUND, h);
    }
}

/// `0x393020`: the ropes' points (rows 0..3 the arcs, 4..7 their copies with the wave and sway).
fn ropes(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let hero = ctx.hero.pos;
    let jitter = ctx.game.arc_jitter.1;
    let m = &mut w.mobys[id];
    if m.pvars.len() < SIZE { m.pvars.resize(SIZE, 0); }
    let rate = m.pv_f(RATE);
    m.set_pv_f(PHASE_A, add_rot(m.pv_f(PHASE_A), rate));
    m.set_pv_f(PHASE_B, add_rot(m.pv_f(PHASE_B), rate));
    let mut s = m.pv_f(SCROLL) + m.pv_f(SCROLL_RATE);
    if 1.0 < s { s -= 1.0; }
    m.set_pv_f(SCROLL, s);
    let (n, flags, c) = (count(m), m.pv_i(FLAGS), m.pv_v(CENTRE));
    for row in 0..4 {
        let path = m.pv_i(PATHS + 4 * row);
        let pts = (flags & 1 != 0 && 0 <= path).then(|| usize::try_from(path).ok().and_then(|p| ctx.level.paths.get(p))).flatten();
        if flags & 1 != 0 && 0 <= path {
            let off = m.pv_v(OFFSETS + 12 * row);
            for col in 0..n {
                let p = pts.and_then(|p| p.get(col)).map_or([0.0; 3], |p| [p[0], p[1], p[2]]);
                set_point(m, row, col, add(p, off));
            }
            if flags & 8 != 0 {
                let d = sub(hero, c);
                let dist = len(d);
                let toward = set_len(d, 1.1);
                if dist <= m.pv_f(PULL) + 1.1 {
                    let strength = 1.0 - (dist - 1.1) / m.pv_f(PULL);
                    for col in 0..n {
                        let rel = sub(point(m, row, col), c);
                        let l = len(rel).min(1.1);
                        let a = strength * (1.0 - l * 0.909_090_9);
                        set_point(m, row, col, add(add(scale(toward, a), scale(rel, 1.0 - a)), c));
                    }
                }
            }
        } else {
            for col in 0..n {
                let p = set_len(LINES[row], col as f32 / (n as f32 - 0.5));
                set_point(m, row, col, add(p, c));
            }
        }
    }
    let t = sub(point(m, 0, 1), point(m, 0, 0));
    let seg = len(t);
    for col in 0..n {
        let j = if flags & 0x40 == 0 { scale(jitter[col], m.pv_f(JITTER) * seg) } else { [0.0; 3] };
        let (mut up, mut phase) = (0.0, 0.0);
        if flags & 0x80 == 0 {
            let a = wrap(m.pv_f(PHASE_A) + col as f32 * -1.780_236).sin();
            phase = wrap(m.pv_f(PHASE_B) + col as f32 * -1.518_436_6);
            up = seg * m.pv_f(WAVE) * a * phase.sin();
        }
        let mut side = [0.0; 3];
        if flags & 0x100 == 0 {
            let ang = add_rot(atan2(t[0], t[1]), f32::from_bits(0x3fc9_0fdc));
            let mag = seg * m.pv_f(SWAY) * phase.cos();
            side = [ang.cos() * mag, ang.sin() * mag, 0.0];
        }
        for row in 0..4 {
            let mut q = point(m, row, col);
            set_point(m, row + 4, col, q);
            if col != 0 && col + 1 != n {
                let mut q4 = q;
                if flags & 0x40 == 0 {
                    q4 = add(q4, j);
                    q = add(q, j);
                    set_point(m, row, col, q);
                }
                if flags & 0x80 == 0 { q4[2] += up; }
                if flags & 0x100 == 0 { q4 = add(q4, side); }
                set_point(m, row + 4, col, q4);
            }
        }
    }
}

/// `0x392440`: each arc's two ribbons (the core 0.9 long-drawn, the copy 0.1).
pub fn draw(w: &mut MobyWorld, id: usize, jitter: &[[f32; 3]; 16], cam: V3, out: &mut Vec<FxQuad>) {
    for k in 0..4 {
        ribbon(w, id, k, [0x40, 0x60, 0x44, 0x64], [0x50, 0x70, 0x54, 0x74], 0.9, jitter, cam, out);
        ribbon(w, id, k + 4, [0x48, 0x68, 0x4c, 0x6c], [0x58, 0x78, 0x5c, 0x7c], 0.1, jitter, cam, out);
    }
}

/// `0x392538(widths, ext, m, points, n, colours)`: a ribbon along the rope row `row`: per segment a core quad
/// (texture `+0x9c`, its S scrolled by `+0x84`) and a glow quad (`+0x80`), both additive, across the segment as the
/// camera sees it (view × way), `width × length` wide at each end — the widths and colours lerped from the rest's to
/// the full pulse's by the pulse there (state 2: rising with the state near the centre; state 3: a band `+0xcc` wide
/// running out over the state; distances × 1/1.1), × the swell near the centre with flag 4; the glow drawn `ext` of
/// the segment past both ends. Each quad takes its far end's colour at all four corners. In state 3 near the end of
/// the band (flag 2) the moby is shaken off home (along the jitter's second vector, `+0xd0` long).
#[allow(clippy::too_many_arguments)]
fn ribbon(w: &mut MobyWorld, id: usize, row: usize, colours: [usize; 4], widths: [usize; 4], ext: f32, jitter: &[[f32; 3]; 16], cam: V3, out: &mut Vec<FxQuad>) {
    let m = &mut w.mobys[id];
    if m.pvars.len() < SIZE { return; }
    let n = count(m);
    let origin = add(m.pos, [0.0, 0.0, 1.1]);
    let pulse = m.pv_f(PULSE);
    let mut phase = 0.0;
    if m.state == 3 {
        let d3 = m.pv_i(TICKS_3) as f32;
        phase = (pulse + pulse + 1.0) * (1.0 - m.pv_i(TIMER) as f32 / d3) - pulse;
        if 1.0 - pulse < phase && m.pv_i(FLAGS) & 2 != 0 {
            let home = m.pv_v(HOME);
            m.pos = add(home, set_len(jitter[1], m.pv_f(SHAKE)));
        }
    }
    let m = &w.mobys[id];
    let state = m.state;
    let near = |p: V3| len(sub(origin, p)) * 0.909_090_9;
    let (core_fx, glow_fx, scroll) = (m.pv_i(CORE_FX).max(0) as usize, m.pv_i(GLOW_FX).max(0) as usize, m.pv_f(SCROLL));
    let (ca, cb, ga, gb) = (m.pv_i(colours[0]) as u32, m.pv_i(colours[1]) as u32, m.pv_i(colours[2]) as u32, m.pv_i(colours[3]) as u32);
    let (ia, ib, oa, ob) = (m.pv_f(widths[0]), m.pv_f(widths[1]), m.pv_f(widths[2]), m.pv_f(widths[3]));
    let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;
    for i in 0..n.saturating_sub(1) {
        let (p, q) = (point(m, row, i), point(m, row, i + 1));
        let (mut t0, mut t1) = (0.0, 0.0);
        if state == 3 {
            let d = (phase - near(p)).abs();
            if d < pulse { t0 = 1.0 - d / pulse; }
            let d = (phase - near(q)).abs();
            if d < pulse { t1 = 1.0 - d / pulse; }
        } else if state == 2 {
            let d = near(p);
            if d < pulse { t0 = (1.0 - d / pulse) * (1.0 - m.pv_i(TIMER) as f32 / m.pv_i(TICKS_2) as f32); }
            let d = near(q);
            if d < pulse { t1 = 1.0 - d / pulse; }
        }
        let mut width = 1.0;
        if m.pv_i(FLAGS) & 4 != 0 {
            let r = len(sub(p, origin));
            if r < m.pv_f(SWELL_REACH) { width = m.pv_f(SWELL) * (1.0 - r / m.pv_f(SWELL_REACH)) + 1.0; }
        }
        let view = sub(p, cam);
        let way = sub(q, p);
        let l = len(way);
        // `0x2f6b50(out, way, view)` = view × way.
        let side = cross(view, way);
        let quad = |w0: f32, w1: f32| {
            let s0 = set_len(side, width * w0 * l);
            let s1 = set_len(side, width * w1 * l);
            [add(p, s0), sub(p, s0), add(q, s1), sub(q, s1)]
        };
        let core = quad(lerp(ia, ib, t0), lerp(ia, ib, t1));
        let c = crate::hud::lerp_rgba(t1, ca, cb);
        let st = [[scroll, 0.0], [scroll, 1.0], [1.0 + scroll, 0.0], [1.0 + scroll, 1.0]];
        out.push(FxQuad { fx: core_fx, additive: true, corners: core, st, rgba: [c; 4] });
        let mut glow = quad(lerp(oa, ob, t0), lerp(oa, ob, t1));
        let e = scale(way, ext);
        glow[0] = sub(glow[0], e);
        glow[1] = sub(glow[1], e);
        glow[2] = add(glow[2], e);
        glow[3] = add(glow[3], e);
        let g = crate::hud::lerp_rgba(t1, ga, gb);
        out.push(FxQuad { fx: glow_fx, additive: true, corners: glow, st: [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]], rgba: [g; 4] });
    }
}
