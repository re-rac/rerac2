//! The Electrolyzer's puzzle, **0x0896** (L02 `0x3c3b80`; the same unit on levels 2, 3, 4, 9, 12, 17, 20 and 21,
//! ported from L02's copy): a panel Ratchet works with the item 0x26 in his hand (its record: class 0x870, icon
//! 0xea81) in the game's mode 7, a screen of its own. Its group holds the puzzle: the pieces 0x0873 (each turns between
//! two ways, its pvars `+0` the right way in degrees, `+0x14` it points the right way), the pulses 0x0a24 (each runs
//! its path `+4` to its piece `+0`; hidden, drawn as sparks) and the balls 0x0a55 (looping their path `+0`; hidden,
//! sparks). ○, ✕ and □ turn every free piece the other way; a pulse that reaches its piece pointing the right way
//! locks it (the piece's lit twin 0x0afc shows in its place), one that reaches it pointing the wrong way fails the
//! puzzle; with every pulse home it is solved, and the save keeps it (the uid's count).
//!
//! Its pvars: the cuboid `+0` the puzzle's screen stands in (its place, 2 higher, and angles: the camera), the prompt's
//! cuboid `+4` (−1: a box in its own frame, x 0..3, y ±1.5, z −1..3), the pulses' and balls' start speed `+8`
//! (per second), two mobys `+0x10` / `+0x14` it hooks when solved (−1: none), `+0x18` ≠ 0: hidden and without
//! collision, else dropped onto the floor under joint 1 with its arcs at `(0, 0.5, 0.15)`; `+0x1c` a level camera
//! released on the way out (−1: none). Its own: the pulses `+0x20` (20 at most), the balls `+0x70`, the pieces
//! `+0xc0`, their counts `+0x110..+0x118`, the pulses home `+0x11c`, the screen's backdrop 0x0b12 `+0x120` (index + 1
//! here), the arcs' timer `+0x124`, two glow phases `+0x128` / `+0x12c`, the wait `+0x130`, the fog kept `+0x134` /
//! `+0x138`, the held item kept `+0x13c`.
//!
//! * **0** (the level start): its group's members lit with the light set 14 (ambient 0x505010, `0x3173f8`), the
//!   pieces, pulses and balls gathered (the pulses and balls hidden, their speed `+8`/60), the light set 14 written
//!   (colour (1, 1, 1, −0.25), direction the cuboid's place); solved in the save (`0x21f984[uid]` or the uid's count):
//!   it and its two mobys hooked (`0x330490`), 5; else 1.
//! * **1** with the item had (`0x1a7b26`) and Ratchet in its prompt area: the prompt 0x130f (owner 0xc, `0x32cc40`);
//!   △ keeps the held item, asks for 0x26 (`0x18c0e8`) and walks Ratchet to joint 1 facing it (`0x3c6368`), 4. Outside a
//!   scene its arcs ([`arcs`], `0x3c6428`) every 20..40 ticks.
//! * **4** (the prompt's lease taken, owner 1): Ratchet there with his sequences settled: he stands (0x6f, sequence
//!   0x54), the script camera swings (mode 3, 300 ticks) to 3 in front of joint 0 facing it, its sound 0, 2; given up
//!   (or 240 ticks walking), 1. **2** Ratchet pushed (at most 0.025 a tick) to joint 1's frame's point (0, 0, −0.06)
//!   and turned (1° a tick) to face it, sequence 0x55; the camera there: it moves (mode 2, 300) into joint 0, its sound
//!   0, 3. **3** the camera at joint 0: it moves from 30 above the cuboid down onto it (mode 2, 480 ticks, 30 of them
//!   gone), the backdrop 0x0b12 made 15 above it, the fog's intensities 100 / 255, the view's tangent tan 0.56217
//!   (`0x1beab0`), 9. **9** Ratchet held (0x6f); the camera on the cuboid: the game's mode 7 asked for (`0x332158(7, 1,
//!   moby, 1)`, refused: back to 1); in mode 7 it and its group shown (mode 0x80 off), 6.
//! * **6** (mode 7, [`MODE`]) the screen's draw ([`draw_hud`], `0x3c58e0`); ○ ✕ □ turn the pieces ([`flip`],
//!   `0x3c5c30`); △ leaves (the mode popped); the balls run ([`ball_step`]), the pulses ([`pulse_step`]): one failed, 7;
//!   one home: bursts at its piece ([`burst`]); every one home: the balls' sound 5, 8.
//! * **7** (failed) the pulses not home spark (red at their end), the balls stand; after 60 ticks △ leaves, ○ ✕ □
//!   start again ([`reset`], `0x3c5d98`), 10. **10** the pieces turn home 3° a tick, then 6.
//! * **8** (solved) the balls race (0.4167 a tick); △ or 240 ticks: the uid counted, its mission done (`0x3173c0`), it
//!   and its two mobys hooked, the mode popped, 5. **5** done.
//!
//! Out of mode 7 in 6, 7 or 8 (after 6 ticks) it leaves as from △ (8 as solved). Each way out: the camera released
//! (kind 3), the held item asked back, Ratchet idle, the backdrop deleted, the fog back. After its update, in 6..10 its
//! group is tinted ([`tint`], `0x3c6748`: the members' ambient, flashing blue in 7; the lit pieces' glow pulsing in
//! the state's pair), and drawn last frame its own glow pulses ([`own_glow`], `0x3c61d0`).
//!
//! **The game's mode 7** (`0x3609b0` on the way in: every moby hidden, mode 0x80, the background black; `0x360a50` on
//! the way out: every moby's 0x80 cleared, the level's background back) runs only the puzzle's update (`0x3784e8`:
//! the moby `0x1ac140` the request named, the particles, the sounds, the help) and draws only the mobys, the effects
//! and the HUD on black (`0x2ea328`: the draw flags 0xf0, no sky, tfrags, ties or shrubs); [`mode_switched`] and the
//! engine's mode-7 frame.
//!
//! Not ported: the class streaming of the item (`0x309da0` / `0x309b90`: the port loads it with the level), the debug
//! cheat (L2 + R2 within 60 ticks with `0x2004a4` set: solved), the fail's flash moby 0x13a0 (`0x420a08`) and its
//! particle 0x2d (`0x3407c8`), the music switch (`0x335928(+0x140, +0x144)`, back to `+0x148`: no music requests in the
//! port), Ratchet's word `0x18a042` = 3, the level camera `+0x1c` (`0x457450`: 12 and 13 on Maktar; its D block's moby `+0x20` and `+0x26` cleared on the
//! way out of 6 by △, of 8 in the mode, and of 9 refused: a camera class the port has not got), the
//! cuboid's own z moved 2 up (`+0x38`: kept here as the puzzle's own offset), the GS TEST of the draw (`0x384178`), the
//! PAL scale and the screen offset `0x1b3e00` of the draw's texts. The view's tangent is put back on every way out (the
//! original only on △ in 6; the play camera writes its own again).

use crate::hero::physics::*;
use crate::moby::{mode, Ctx, MobyWorld};
use crate::pad::button;

pub const CLASS: i16 = 0x0896;
/// The item the puzzle needs (its record: class 0x870).
pub const ITEM: i32 = 0x26;
/// The game's mode of the puzzle's screen (`0x1a8f00`).
pub const MODE: i32 = 7;
pub const PIECE: i16 = 0x0873;
pub const PULSE: i16 = 0x0a24;
pub const BALL: i16 = 0x0a55;
/// The screen's backdrop (`0x315438(0xb12)`).
pub const BACKDROP: i16 = 0x0b12;
/// `0x130f`: the prompt.
pub const PROMPT: u32 = 0x130f;
const MAX: usize = 20;

mod pv {
    pub const CUBOID: usize = 0;
    pub const AREA: usize = 4;
    pub const SPEED: usize = 8;
    pub const HOOKS: usize = 0x10;
    pub const PLAIN: usize = 0x18;
    pub const PULSES: usize = 0x20;
    pub const BALLS: usize = 0x70;
    pub const PIECES: usize = 0xc0;
    pub const N_PULSES: usize = 0x110;
    pub const N_BALLS: usize = 0x114;
    pub const N_PIECES: usize = 0x118;
    pub const HOME: usize = 0x11c;
    pub const BACKDROP: usize = 0x120;
    pub const ARC: usize = 0x124;
    pub const PHASE: usize = 0x128;
    pub const WAIT: usize = 0x130;
    pub const FOG: usize = 0x134;
    pub const HELD: usize = 0x13c;
}

/// The pieces' pvars (0x0873).
mod piece {
    pub const ANGLE: usize = 0;
    pub const LOCKED: usize = 4;
    pub const SPARKS: usize = 8;
    pub const HOME: usize = 0x10;
    pub const RIGHT: usize = 0x14;
    pub const RIGHT0: usize = 0x18;
    /// The lit twin 0x0afc (index + 1).
    pub const TWIN: usize = 0x1c;
}

/// The pulses' pvars (0x0a24; the port keeps the indices at `+0` / `+4` for the pointers at `+8` / `+0x18`).
mod pulse {
    pub const PIECE: usize = 0;
    pub const PATH: usize = 4;
    pub const TIMER: usize = 0xc;
    pub const SPEED: usize = 0x10;
    pub const POINT: usize = 0x14;
}

/// The balls' pvars (0x0a55; the path index at `+0` for the pointer at `+0x10`).
mod ball {
    pub const PATH: usize = 0;
    pub const SPEED: usize = 4;
    pub const POINT: usize = 8;
    pub const TIMER: usize = 0xc;
}

/// The puzzle's part of the game state.
#[derive(Clone, Debug, Default)]
pub struct PuzzleState {
    /// The screen's draw this tick (`0x2eac28(0x3c58e0, moby)`).
    pub hud: Option<PuzzleHud>,
    /// The light bank's set 14 to write (`0x1c8600`, the level start).
    pub light: Option<[[f32; 4]; 4]>,
    /// The view's tangent while the puzzle holds it (`0x1beab0`, [`TAN`]).
    pub tan: Option<f32>,
    /// `0x1afe34` / `0x1afe44`: the arcs' side, flipped by every arc.
    pub arc_flip: bool,
}

/// The screen's draw: its moby and state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PuzzleHud {
    pub moby: usize,
    pub state: u8,
}

/// `sin(0x3f0fea69) / cos(0x3f0fea69)`: the view's tangent on the screen.
pub fn tan() -> f32 {
    let a = f32::from_bits(0x3f0f_ea69);
    a.sin() / a.cos()
}

/// The colour pairs of the lit pieces' glow by state (`0x3c3b80`'s tail).
const GLOW_SOLVED: (u32, u32) = (0xdd10_7010, 0xdd10_5010);
const GLOW_FAILED: (u32, u32) = (0xff00_00dd, 0xff00_0090);
const GLOW_PLAY: (u32, u32) = (0xff00_0070, 0xff00_0050);
/// `0x1afde0`: the phases' steps (× π/10800 a tick); `0x1afde8` / `0x1afdf0`: the own glow's pairs.
const PHASE_STEP: [f32; 2] = [347.0, 500.0];
const OWN_PLAY: (u32, u32) = (0x8020_0020, 0x8030_20c0);
const OWN_DONE: (u32, u32) = (0x8010_5010, 0x8030_8030);

fn idx(w: &MobyWorld, v: i32) -> Option<usize> { usize::try_from(v).ok().filter(|&i| i < w.mobys.len()) }

fn list(w: &MobyWorld, id: usize, at: usize, n: usize) -> Vec<usize> {
    let m = &w.mobys[id];
    (0..m.pv_i(n).clamp(0, MAX as i32) as usize).filter_map(|k| idx(w, m.pv_i(at + 4 * k))).collect()
}

/// The group's members (`0x1fc060[group]`), and how many are alive (`0x320a70(group, −1)`).
fn members(w: &MobyWorld, id: usize) -> (Vec<usize>, usize) {
    let Some(g) = usize::try_from(w.mobys[id].group as i32).ok().and_then(|g| w.groups.get(g)) else { return (Vec::new(), 0) };
    let live = g.iter().filter(|&&i| w.mobys.get(i).is_some_and(|m| m.state < 0x80)).count();
    (g.clone(), live)
}

fn path<'a>(ctx: &Ctx<'a>, i: i32) -> &'a [[f32; 4]] { usize::try_from(i).ok().and_then(|i| ctx.level.paths.get(i)).map_or(&[], |p| p.as_slice()) }

fn point(p: &[[f32; 4]], k: usize) -> V3 { p.get(k).map_or([0.0; 3], |q| [q[0], q[1], q[2]]) }

/// The cuboid `+0`'s place (2 higher, as the level start leaves it) and angles.
fn cuboid(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<(V3, V3)> {
    let c = usize::try_from(w.mobys[id].pv_i(pv::CUBOID)).ok().and_then(|i| ctx.level.cuboids.get(i))?;
    Some(([c.matrix[3][0], c.matrix[3][1], c.matrix[3][2] + 2.0], [c.extra[0], c.extra[1], c.extra[2]]))
}

/// `0x3168e0(m, j)`.
fn joint(w: &MobyWorld, id: usize, ctx: &Ctx, j: usize) -> V3 { ctx.host.joint(&w.mobys[id], j).unwrap_or(w.mobys[id].pos) }

fn through(rows: &[V3; 3], v: V3) -> V3 { std::array::from_fn(|k| v[0] * rows[0][k] + v[1] * rows[1][k] + v[2] * rows[2][k]) }

/// `0x3173f8(m, ambient, 0xe, 0xe, 0xe)`: lit by the light set 14 with the ambient `c`.
fn light(m: &mut crate::moby::Moby, c: u32) {
    m.light = 0x000e_0e0e;
    m.ambient = [c as u8, (c >> 8) as u8, (c >> 16) as u8];
}

/// `0x32c0b8(phase, a, b)`: the colours mixed by `sin(phase)·0.5 + 0.5`.
fn pulse_colour(phase: f32, a: u32, b: u32) -> u32 { crate::particles::tween_color((phase.sin() * 0.5 + 0.5).to_bits(), a, b) }

/// `0x3c65e8`: Ratchet within 24 (xy) and in the prompt's area (module docs).
fn in_area(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let m = &w.mobys[id];
    if 24.0 < dist2(m.pos, ctx.hero.pos) { return false; }
    let area = m.pv_i(pv::AREA);
    if area != -1 { return ctx.level.in_cuboid(area, ctx.hero.pos); }
    // `0x32a6a0`: Ratchet's offset in its frame (the inverse of its rows).
    let d = sub(ctx.hero.pos, m.pos);
    let l: V3 = std::array::from_fn(|k| { let r = m.rows[k]; let n = dot(r, r); if n == 0.0 { 0.0 } else { dot(d, r) / n } });
    (0.0..=3.0).contains(&l[0]) && (-1.5..=1.5).contains(&l[1]) && (-1.0..=3.0).contains(&l[2])
}

/// `0x3c6988(pos, life, rgba, life2, rgba2)`: a spark (twinkle) at `pos`, size 2..4, and a small one in the second
/// colour turning the other way.
fn sparks(ctx: &mut Ctx, pos: V3, life: i16, rgba: u32, life2: i16, rgba2: u32) {
    let g = &mut ctx.game.rng;
    let r = g.randf(2.0, 4.0);
    let k = g.randi(2);
    let spin = if k == 0 { -1 } else { k } as i8;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    crate::particles::twinkle(sys, g, r * 0.3, r, 0.0, pos, life, rgba, 0, spin, [0.0; 3], 0);
    if life2 != 0 && rgba2 != 0 { crate::particles::twinkle(sys, g, r * 0.04, r * 0.4, 0.0, pos, life2, rgba2, 1, -spin, [0.0; 3], 0); }
}

/// `0x3c5ef0(pos, n)`: `n` pairs of sparks thrown from `pos` (0.3 toward the camera), within 60° of the way to the
/// camera, rising; `pos` wanders by up to 0.15 a pair.
fn burst(ctx: &mut Ctx, pos: &mut V3, n: i32) {
    let cam = ctx.game.cameras.view_euler().0;
    let mut d = sub(cam, *pos);
    let o = set_len(d, 0.3);
    d[2] = 0.0;
    for _ in 0..n.max(0) {
        let g = &mut ctx.game.rng;
        let s = g.randf(0.5, 2.0);
        let k = g.randi(2);
        let spin = if k == 0 { -1 } else { k } as i8;
        // `0x325ff8(a, out, d, up)`: `d` turned about up.
        let (sa, ca) = g.randf(-1.047_197_7, 1.047_197_7).sin_cos();
        let mut v = set_len([d[0] * ca - d[1] * sa, d[0] * sa + d[1] * ca, 0.0], g.randf(0.05, 3.0) * 0.016_666_668);
        v[2] = g.randf(1.0, 5.0) * 0.016_666_668;
        let p = add(*pos, o);
        for c in pos.iter_mut() { *c += g.randf_signed(0.0, 0.15); }
        let Some(sys) = ctx.parts.as_deref_mut() else { continue };
        for (size, end, rot, sp) in [(s * 0.1, s, 0, spin), (s * 0.07, s * 0.7, 1, -spin)] {
            if let Some(i) = crate::particles::twinkle(sys, g, size, end, f32::from_bits(0x3b03_126f), p, 0x3c, 0xddff_1010, rot, sp, v, 0) {
                sys.pool.recs[i][9] = 0x74;
            }
        }
    }
}

/// `0x2f0948(fall, pos, a, b, count, 0)`: `count` staged sparks (def 0x3c additive) from `pos` moving between `a`
/// and `b`, their stages 10 / 40 / 10 ticks × 0.7..1.3, sinking `fall`/3600 a tick for their whole time, size
/// 0.1..0.2, the colours the defaults' (`$gp−0x6298..`: 0x80000080..0x80008080 to 0x80..0x8080).
pub fn spray(ctx: &mut Ctx, fall: f32, pos: V3, a: V3, b: V3, count: i32) {
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let k = fall * f32::from_bits(0x3991_a2b4);
    for _ in 0..count.max(0) {
        let f = g.randf(0.7, 1.3);
        let (s3, s2, s1) = ((10.0 * f) as i32, (40.0 * f) as i32, (10.0 * f) as i32);
        let t = g.randf(0.0, 1.0);
        let v = add(a, scale(sub(b, a), t));
        let size = g.randf(0.1, 0.2);
        let dz = -k * (s3 + s2 + s1) as f32;
        let c0 = crate::particles::tween_color(g.randf(0.0, 1.0).to_bits(), 0x8000_0080, 0x8000_8080);
        let c1 = crate::particles::tween_color(g.randf(0.0, 1.0).to_bits(), 0x80, 0x8080);
        crate::particles::staged(sys, g, pos, [v[0], v[1], v[2], size], [v[0], v[1], v[2] + dz, size], c0, c1, s3 as i16, s2 as i16, s1 as i16, 0x1003c);
    }
}

/// `0x3c6428`: the arcs: with the timer `+0x124` out, sparks sprayed (15, 12 of them) from its arc point between its
/// frame's (0, ±1, 0.3) at 2/60 and (0, ±1, −0.3) at 4/60 (the side flipping each time), the timer 20..40, its sound 4.
pub fn arcs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut t = w.mobys[id].pv_i(pv::ARC);
    let out = crate::moby::countdown(&mut t);
    w.mobys[id].set_pv_i(pv::ARC, t);
    if !out { return; }
    let m = &w.mobys[id];
    let start = if m.pv_i(pv::PLAIN) == 0 { add(through(&m.rows, [0.0, 0.5, 0.15]), m.pos) } else { m.pos };
    let y = if ctx.game.puzzle.arc_flip { -1.0 } else { 1.0 };
    let a = set_len(through(&m.rows, [0.0, y, 0.3]), 2.0 * 0.016_666_668);
    let b = set_len(through(&m.rows, [0.0, y, -0.3]), 4.0 * 0.016_666_668);
    ctx.game.puzzle.arc_flip = !ctx.game.puzzle.arc_flip;
    spray(ctx, 15.0, start, a, b, 12);
    let t = ctx.game.rng.rand_range(20, 40);
    w.mobys[id].set_pv_i(pv::ARC, t);
    ctx.host.play_sound_of(&w.mobys[id], CLASS, 4, 0);
}

/// `0x3c5770(ball, fast)`: a ball along its looping path (at its end back at the start), its speed growing
/// (`+= 0.03333/speed · 1/6000`, half that once its timer passes 480) or 0.4167 racing; the next point when within
/// 0.88 of a step (xy).
pub fn ball_step(w: &mut MobyWorld, b: usize, fast: bool, ctx: &Ctx) {
    let m = &mut w.mobys[b];
    let p = path(ctx, m.pv_i(ball::PATH));
    if p.is_empty() { return; }
    let mut k = m.pv_i(ball::POINT).max(0) as usize;
    if k == p.len() - 1 {
        k = 0;
        m.pos = point(p, 0);
    }
    let speed = if fast {
        0.416_666_7
    } else {
        let s = m.pv_f(ball::SPEED);
        s + (0.03333 / s) * if m.pv_f(ball::TIMER) < 480.0 { 0.000_166_666_66 } else { 8.333_333e-5 }
    };
    m.set_pv_f(ball::SPEED, speed);
    let to = point(p, k + 1);
    m.pos = add(m.pos, set_len(sub(to, m.pos), speed));
    if dist2(m.pos, to) <= speed * 0.88 { k += 1; }
    m.set_pv_i(ball::POINT, k as i32);
}

/// What a pulse did this tick (`0x3c5558`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PulseStep {
    Moving,
    /// At its piece pointing the wrong way (its class sound 2).
    Failed,
    /// At its piece pointing the right way: the piece locked.
    Home,
    /// Its piece locked already.
    Locked,
}

/// `0x3c5558`: a pulse along its path to its piece (the speed as [`ball_step`]'s). At the end: the piece pointing the
/// right way locks (`+4` = 1, its sparks `+8` = 20), the pulse and the piece hidden, the twin shown at the piece's
/// angles, the class sound 3; else the sound 2.
pub fn pulse_step(w: &mut MobyWorld, q: usize, ctx: &mut Ctx) -> PulseStep {
    let Some(pc) = idx(w, w.mobys[q].pv_i(pulse::PIECE)) else { return PulseStep::Locked };
    if w.mobys[pc].pv_i(piece::LOCKED) != 0 { return PulseStep::Locked; }
    let p = path(ctx, w.mobys[q].pv_i(pulse::PATH)).to_vec();
    if p.is_empty() { return PulseStep::Locked; }
    let k = w.mobys[q].pv_i(pulse::POINT).max(0) as usize;
    if k + 1 >= p.len() {
        if w.mobys[pc].pv_i(piece::RIGHT) == 0 {
            ctx.host.play_sound_of(&w.mobys[q], CLASS, 2, 0);
            return PulseStep::Failed;
        }
        let (rot, twin) = {
            let m = &mut w.mobys[pc];
            m.set_pv_i(piece::LOCKED, 1);
            m.set_pv_i(piece::SPARKS, 0x14);
            m.mode |= 0x41;
            (m.rot, m.pv_i(piece::TWIN) - 1)
        };
        w.mobys[q].mode |= 0x41;
        if let Some(t) = idx(w, twin) {
            w.mobys[t].rot = rot;
            w.rebuild(t, &*ctx.host);
            w.mobys[t].mode &= !0x41;
        }
        w.changed = true;
        ctx.host.play_sound_of(&w.mobys[q], CLASS, 3, 0);
        return PulseStep::Home;
    }
    let m = &mut w.mobys[q];
    let s = m.pv_f(pulse::SPEED);
    let speed = s + (0.03333 / s) * if m.pv_f(pulse::TIMER) < 480.0 { 0.000_166_666_66 } else { 8.333_333e-5 };
    m.set_pv_f(pulse::SPEED, speed);
    let to = point(&p, k + 1);
    m.pos = add(m.pos, set_len(sub(to, m.pos), speed));
    if dist2(m.pos, to) <= speed * 0.88 { m.set_pv_i(pulse::POINT, k as i32 + 1); }
    PulseStep::Moving
}

/// `0x3c5c30`: every free piece turned 90° the other way (toward its right way when it points the wrong way: a right
/// way of 270° counts as −90°), its flag `+0x14` flipped; the class sound 1 at the first piece.
pub fn flip(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pieces = list(w, id, pv::PIECES, pv::N_PIECES);
    for &p in &pieces {
        let m = &mut w.mobys[p];
        if m.pv_i(piece::LOCKED) != 0 { continue; }
        if m.pv_f(piece::ANGLE) == 270.0 { m.set_pv_f(piece::ANGLE, -90.0); }
        let right = m.pv_f(piece::ANGLE) * 0.017_453_294;
        let half = f32::from_bits(0x3fc9_0fdc);
        m.rot[2] = if sub_rot(m.rot[2], right).abs() < 0.001 { add_rot(m.rot[2], half) } else { sub_rot(m.rot[2], half) };
        let on = if m.pv_i(piece::RIGHT) == 0 { 1.0f32 } else { 0.0 };
        m.set_pv_f(piece::RIGHT, on);
        w.rebuild(p, &*ctx.host);
    }
    w.changed = true;
    if let Some(&p) = pieces.first() { ctx.host.play_sound_of(&w.mobys[p], CLASS, 1, 0); }
}

/// `0x3c5d98`: the puzzle set up again: the wait 60; the pieces free, their flag as placed, their twins hidden, they
/// shown; the pulses at their paths' starts at the start speed; the balls likewise at 2/60.
pub fn reset(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    w.mobys[id].set_pv_i(pv::WAIT, 0x3c);
    for p in list(w, id, pv::PIECES, pv::N_PIECES) {
        let m = &mut w.mobys[p];
        m.set_pv_i(piece::LOCKED, 0);
        m.set_pv_i(piece::RIGHT, m.pv_i(piece::RIGHT0));
        m.set_pv_i(piece::SPARKS, 0);
        m.mode &= !0x41;
        if let Some(t) = idx(w, w.mobys[p].pv_i(piece::TWIN) - 1) { w.mobys[t].mode |= 0x41; }
    }
    let speed = w.mobys[id].pv_i(pv::SPEED) as f32 * 0.016_666_668;
    for q in list(w, id, pv::PULSES, pv::N_PULSES) {
        let at = point(path(ctx, w.mobys[q].pv_i(pulse::PATH)), 0);
        let m = &mut w.mobys[q];
        m.set_pv_i(pulse::POINT, 0);
        m.set_pv_f(pulse::TIMER, -1.0);
        m.set_pv_f(pulse::SPEED, speed);
        m.pos = at;
        w.mobys[id].set_pv_i(pv::HOME, 0);
    }
    for b in list(w, id, pv::BALLS, pv::N_BALLS) {
        let at = point(path(ctx, w.mobys[b].pv_i(ball::PATH)), 0);
        let m = &mut w.mobys[b];
        m.set_pv_i(ball::POINT, 0);
        m.set_pv_f(ball::SPEED, f32::from_bits(0x3d08_8889));
        m.set_pv_f(ball::TIMER, -1.0);
        m.pos = at;
    }
    w.changed = true;
}

/// The pieces back at their home angles `+0x10`.
fn pieces_home(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    for p in list(w, id, pv::PIECES, pv::N_PIECES) {
        w.mobys[p].rot[2] = w.mobys[p].pv_f(piece::HOME);
        w.rebuild(p, &*ctx.host);
    }
    w.changed = true;
}

/// The two mobys `+0x10` / `+0x14` and itself hooked (`0x330490(m, 1)`).
fn hook(w: &mut MobyWorld, id: usize) {
    w.mobys[id].once |= 4;
    for k in 0..2 {
        if let Some(i) = idx(w, w.mobys[id].pv_i(pv::HOOKS + 4 * k)) { w.mobys[i].once |= 4; }
    }
}

/// A way out: the camera released (kind 3), the held item asked back, Ratchet idle, the backdrop deleted, then (`full`)
/// the puzzle set up again with its pieces home, and the fog and the view's tangent back.
fn leave(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, full: bool) {
    ctx.game.cameras.camera_script2(3, ctx.hero);
    ctx.hero.items.request = w.mobys[id].pv_i(pv::HELD);
    ctx.hero.request_state(0, true);
    w.mobys[id].update_dist = 0x40;
    if let Some(b) = idx(w, w.mobys[id].pv_i(pv::BACKDROP) - 1) { w.delete(b); }
    w.mobys[id].set_pv_i(pv::BACKDROP, 0);
    if full {
        reset(w, id, ctx);
        pieces_home(w, id, ctx);
    }
    let m = &w.mobys[id];
    ctx.game.fog.near_intensity = m.pv_f(pv::FOG);
    ctx.game.fog.far_intensity = m.pv_f(pv::FOG + 4);
    ctx.game.puzzle.tan = None;
}

/// `0x3c3b80`'s solved end: the uid counted, its mission done, hooked; out of the mode when in it.
fn solved(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (uid, mission) = (w.mobys[id].uid, w.mobys[id].mission);
    ctx.game.count_uid(uid);
    ctx.game.mission_done(mission);
    hook(w, id);
    leave(w, id, ctx, false);
    if ctx.game.scenes.mode == MODE { ctx.game.scenes.pop(ctx.hero.hp); }
    w.mobys[id].set_state(5, -1);
}

/// `0x3c3b80` (0x0896): see the module.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.puzzle.hud.is_some_and(|h| h.moby == id) { ctx.game.puzzle.hud = None; }
    let in_mode = ctx.game.scenes.mode == MODE;
    let pressed = ctx.hero.items.pressed;
    match w.mobys[id].state {
        0 => init(w, id, ctx),
        1 => {
            if in_area(w, id, ctx) && ctx.game.items.get(ITEM as usize).is_some_and(|&b| b != 0) {
                ctx.game.prompt.force(0xc, PROMPT, 1);
                if pressed & button::TRIANGLE != 0 {
                    w.mobys[id].set_pv_i(pv::HELD, ctx.hero.items.item);
                    ctx.hero.items.request = ITEM;
                    // `0x3c6368`: to joint 1, facing it.
                    let at = joint(w, id, ctx, 1);
                    ctx.hero.script_walk_to(add_rot(w.mobys[id].rot[2], f32::from_bits(0x4049_0fd0)), at, false);
                    w.mobys[id].set_state(4, -1);
                }
            }
            if ctx.game.scenes.mode != crate::scene::SCENE { arcs(w, id, ctx); }
        }
        2 => {
            ctx.game.prompt.force(1, 0, 1);
            if let Some((rows, at)) = ctx.host.joint_matrix(&w.mobys[id], 1) {
                let to = add(through(&rows, [0.0, 0.0, -0.06]), at);
                let mut push = sub(to, ctx.hero.pos);
                push[2] = 0.0;
                if 0.025 < len(push) { push = set_len(push, 0.025); }
                ctx.hero.push = push;
            }
            // `0x323428(yaw + π, 1°, &yaw)`.
            let want = add_rot(w.mobys[id].rot[2], std::f32::consts::PI);
            let step = f32::from_bits(0x3c8e_fa37);
            ctx.hero.rot[2] = add_rot(ctx.hero.rot[2], sub_rot(want, ctx.hero.rot[2]).clamp(-step, step));
            if ctx.hero_view.flags & 2 != 0 { ctx.hero.anim_req = Some((7.0, 0x55, 0)); }
            let yaw = w.mobys[id].rot[2];
            let j0 = joint(w, id, ctx, 0);
            let front = add([yaw.cos() * 3.0, yaw.sin() * 3.0, 0.0], j0);
            let (view, euler) = ctx.game.cameras.view_euler();
            if dist(view, front) <= 0.05 {
                ctx.game.cameras.camera_script(view, euler, crate::script_camera::MODE_MOVE, 300, false, ctx.hero);
                ctx.game.cameras.camera_script_targets(Some(j0), Some(euler));
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                w.mobys[id].update_dist = 0xff;
                w.mobys[id].set_state(3, -1);
            }
        }
        3 => {
            ctx.game.prompt.force(1, 0, 1);
            if ctx.hero_view.flags & 2 != 0 { ctx.hero.anim_req = Some((0.0, 0x55, 0)); }
            let j0 = joint(w, id, ctx, 0);
            if dist(ctx.game.cameras.view_euler().0, j0) <= 0.5 {
                if let Some((at, euler)) = cuboid(w, id, ctx) {
                    ctx.game.cameras.camera_script([at[0], at[1], at[2] + 30.0], euler, crate::script_camera::MODE_MOVE, 0x1e0, false, ctx.hero);
                    ctx.game.cameras.script.timer = 0x1c2;
                    ctx.game.cameras.camera_script_targets(Some(at), Some(euler));
                    let b = w.create(BACKDROP);
                    let bm = &mut w.mobys[b];
                    bm.alpha = 0x80;
                    bm.draw_dist = 0x80;
                    bm.mode &= !0x41;
                    bm.pos = [at[0], at[1], at[2] + 15.0];
                    bm.mode &= !0x80;
                    w.rebuild(b, &*ctx.host);
                    w.changed = true;
                    w.mobys[id].set_pv_i(pv::BACKDROP, b as i32 + 1);
                }
                let fog = ctx.game.fog;
                let m = &mut w.mobys[id];
                m.set_pv_f(pv::FOG, fog.near_intensity);
                m.set_pv_f(pv::FOG + 4, fog.far_intensity);
                ctx.game.fog.near_intensity = 100.0;
                ctx.game.fog.far_intensity = 255.0;
                ctx.game.puzzle.tan = Some(tan());
                m.set_state(9, -1);
            }
        }
        4 => {
            ctx.game.prompt.force(1, 0, 1);
            let walk = match ctx.hero.state { 0x62..=0x63 => 1, 100 => 2, _ => 0 };
            let v = ctx.hero_view;
            if walk == 2 && v.seq_a == v.seq_b {
                ctx.hero.request_state(0x6f, false);
                ctx.hero.anim_req = Some((7.0, 0x54, 0));
                let m = &w.mobys[id];
                let (yaw, rot) = (m.rot[2], m.rot);
                let front = add([yaw.cos() * 3.0, yaw.sin() * 3.0, 0.0], joint(w, id, ctx, 0));
                let (view, euler) = ctx.game.cameras.view_euler();
                ctx.game.cameras.camera_script(view, euler, crate::script_camera::MODE_SWING, 300, false, ctx.hero);
                ctx.game.cameras.camera_script_targets(Some(front), Some([rot[0], rot[1], add_rot(yaw, f32::from_bits(0x4049_0fdc))]));
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                w.mobys[id].update_dist = 0xff;
                w.mobys[id].set_state(2, -1);
            } else {
                let t = w.mobys[id].state_ticks as u16;
                w.mobys[id].state_ticks = t.wrapping_add(1) as i16;
                if walk == 0 || 0xf0 < t {
                    // `0x2d79d8`: the walk given up (idle, or 3 still moving).
                    ctx.hero.request_state(if ctx.hero.eff_len < 0.045 { 0 } else { 3 }, true);
                    w.mobys[id].set_state(1, -1);
                }
            }
        }
        6 => {
            if !in_mode {
                if 6 <= w.mobys[id].state_ticks as u16 {
                    leave(w, id, ctx, true);
                    w.mobys[id].set_state(1, -1);
                }
            } else {
                ctx.game.puzzle.hud = Some(PuzzleHud { moby: id, state: 6 });
                if pressed & 0xe0 != 0 {
                    flip(w, id, ctx);
                } else if pressed & button::TRIANGLE != 0 {
                    leave(w, id, ctx, true);
                    ctx.game.scenes.pop(ctx.hero.hp);
                    w.mobys[id].set_state(1, -1);
                }
                let balls = list(w, id, pv::BALLS, pv::N_BALLS);
                for &b in &balls {
                    ball_step(w, b, false, ctx);
                    sparks(ctx, w.mobys[b].pos, 6, 0x8880_2020, 4, 0x7f6f_6fff);
                }
                let pulses = list(w, id, pv::PULSES, pv::N_PULSES);
                for &q in &pulses {
                    match pulse_step(w, q, ctx) {
                        PulseStep::Failed => w.mobys[id].set_state(7, -1),
                        PulseStep::Moving => sparks(ctx, w.mobys[q].pos, 6, 0x8880_2020, 4, 0x7f6f_6fff),
                        PulseStep::Home => {
                            let home = w.mobys[id].pv_i(pv::HOME) + 1;
                            w.mobys[id].set_pv_i(pv::HOME, home);
                            let mut at = idx(w, w.mobys[q].pv_i(pulse::PIECE)).map_or(w.mobys[q].pos, |p| w.mobys[p].pos);
                            burst(ctx, &mut at, 0x32);
                            burst(ctx, &mut at, 0x32);
                            at[2] += 1.0;
                            sparks(ctx, at, 6, 0x88ff_1010, 4, 0x7f6f_6fff);
                        }
                        PulseStep::Locked => {
                            if let Some(p) = idx(w, w.mobys[q].pv_i(pulse::PIECE)) {
                                let n = w.mobys[p].pv_i(piece::SPARKS);
                                if n != 0 {
                                    let at = w.mobys[p].pos;
                                    sparks(ctx, [at[0], at[1], at[2] + 1.0], 6, 0x88ff_1010, 4, 0x7f6f_6fff);
                                    w.mobys[p].set_pv_i(piece::SPARKS, n - 1);
                                }
                            }
                        }
                    }
                }
                if w.mobys[id].pv_i(pv::HOME) == pulses.len() as i32 {
                    if let Some(&b) = balls.first() { ctx.host.play_sound_of(&w.mobys[b], CLASS, 5, 0); }
                    w.mobys[id].set_state(8, -1);
                }
            }
        }
        7 => {
            if !in_mode {
                if 6 <= w.mobys[id].state_ticks as u16 {
                    leave(w, id, ctx, true);
                    w.mobys[id].set_state(1, -1);
                }
            } else {
                ctx.game.puzzle.hud = Some(PuzzleHud { moby: id, state: 7 });
                let wait = w.mobys[id].pv_i(pv::WAIT);
                if wait != 0 {
                    w.mobys[id].set_pv_i(pv::WAIT, wait - 1);
                } else if pressed & button::TRIANGLE != 0 {
                    leave(w, id, ctx, true);
                    ctx.game.scenes.pop(ctx.hero.hp);
                    w.mobys[id].set_state(1, -1);
                } else if pressed & 0xe0 != 0 {
                    reset(w, id, ctx);
                    w.mobys[id].set_state(10, -1);
                }
                for q in list(w, id, pv::PULSES, pv::N_PULSES) {
                    let Some(p) = idx(w, w.mobys[q].pv_i(pulse::PIECE)) else { continue };
                    if w.mobys[p].pv_i(piece::LOCKED) == 1 { continue; }
                    let at = w.mobys[q].pos;
                    let end = path(ctx, w.mobys[q].pv_i(pulse::PATH)).len() as i32 - 1;
                    if w.mobys[q].pv_i(pulse::POINT) < end {
                        sparks(ctx, at, 6, 0x8880_2020, 4, 0x7f6f_6fff);
                    } else {
                        sparks(ctx, at, 6, 0x5026_a883, 4, 0x7f6f_6fff);
                    }
                }
                for b in list(w, id, pv::BALLS, pv::N_BALLS) { sparks(ctx, w.mobys[b].pos, 6, 0x8880_2020, 4, 0x7f6f_6fff); }
            }
        }
        8 => {
            if in_mode {
                ctx.game.puzzle.hud = Some(PuzzleHud { moby: id, state: 8 });
                for b in list(w, id, pv::BALLS, pv::N_BALLS) {
                    ball_step(w, b, true, ctx);
                    sparks(ctx, w.mobys[b].pos, 0x3c, 0x8800_ff00, 4, 0x7f6f_6fff);
                }
                if pressed & button::TRIANGLE != 0 || 0xf0 < w.mobys[id].state_ticks as u16 { solved(w, id, ctx); }
            } else if 6 <= w.mobys[id].state_ticks as u16 {
                solved(w, id, ctx);
            }
        }
        9 => {
            ctx.game.prompt.force(1, 0, 1);
            if ctx.hero.state != 0x6f { ctx.hero.request_state(0x6f, true); }
            let near = cuboid(w, id, ctx).is_some_and(|(at, _)| dist(ctx.game.cameras.view_euler().0, at) <= 1.0);
            if near {
                if !in_mode {
                    if ctx.game.scenes.request(MODE, true, id as i32, 1, ctx.hero.hp) != 0 {
                        ctx.game.cameras.camera_script2(3, ctx.hero);
                        ctx.hero.items.request = w.mobys[id].pv_i(pv::HELD);
                        ctx.hero.request_state(0, true);
                        w.mobys[id].set_state(1, -1);
                    }
                } else {
                    w.mobys[id].mode &= !0x80;
                    let (all, live) = members(w, id);
                    if 1 < live {
                        for i in all {
                            w.mobys[i].mode &= !0x80;
                            if w.mobys[i].o_class == PIECE {
                                if let Some(t) = idx(w, w.mobys[i].pv_i(piece::TWIN) - 1) { w.mobys[t].mode &= !0x80; }
                            }
                        }
                    }
                    w.mobys[id].set_state(6, -1);
                }
            }
        }
        10 => {
            ctx.game.puzzle.hud = Some(PuzzleHud { moby: id, state: 10 });
            let pieces = list(w, id, pv::PIECES, pv::N_PIECES);
            let mut home = 0;
            for &p in &pieces {
                let m = &mut w.mobys[p];
                if 0.08 <= sub_rot(m.pv_f(piece::HOME), m.rot[2]).abs() {
                    m.rot[2] = add_rot(m.rot[2], f32::from_bits(0x3d56_7751));
                    w.rebuild(p, &*ctx.host);
                } else {
                    home += 1;
                }
            }
            w.changed = true;
            if home == pieces.len() {
                pieces_home(w, id, ctx);
                w.mobys[id].set_state(6, -1);
            }
        }
        _ => {}
    }
    match w.mobys[id].state {
        8 => tint(w, id, GLOW_SOLVED),
        7 => tint(w, id, GLOW_FAILED),
        6 | 9 | 10 => tint(w, id, GLOW_PLAY),
        _ => {}
    }
    if w.mobys[id].drawn != 0 { own_glow(w, id); }
    let m = &mut w.mobys[id];
    m.state_ticks = m.state_ticks.wrapping_add(1);
}

/// The level start (state 0): see the module.
fn init(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].update_dist = 0x40;
    if w.mobys[id].pv_i(pv::PLAIN) == 0 {
        for k in 0..2 {
            let a = ctx.game.rng.rand_angle();
            w.mobys[id].set_pv_f(pv::PHASE + 4 * k, a);
        }
        // `0x2e1e10`: a line 1 above joint 1 to 1 below; on a hit it is moved onto the floor, 0.1 above.
        let p = joint(w, id, ctx, 1);
        if let Some(hit) = w.coll_line(ctx, [p[0], p[1], p[2] + 1.0], [p[0], p[1], p[2] - 1.0], 2, Some(id)) {
            w.mobys[id].pos[2] += hit.point[2] - p[2] + 0.1;
            w.rebuild(id, &*ctx.host);
        }
    } else {
        let m = &mut w.mobys[id];
        m.has_collision = false;
        m.mode = (m.mode & !mode::TARGET) | 0x41;
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::WAIT, 0x3c);
    for o in [pv::N_PULSES, pv::N_BALLS, pv::N_PIECES, pv::HOME, pv::BACKDROP] { m.set_pv_i(o, 0); }
    if let Some((at, _)) = cuboid(w, id, ctx) {
        ctx.game.puzzle.light = Some([[1.0, 1.0, 1.0, -0.25], [at[0], at[1], at[2], 0.0], [0.0; 4], [0.0; 4]]);
    }
    let speed = w.mobys[id].pv_i(pv::SPEED) as f32 * 0.016_666_668;
    let (all, live) = members(w, id);
    if 1 < live {
        for i in all {
            let class = w.mobys[i].o_class;
            if class != CLASS { light(&mut w.mobys[i], 0x50_5010); }
            let (slot, n, speed_at) = match class {
                PULSE => (pv::PULSES, pv::N_PULSES, Some(pulse::SPEED)),
                BALL => (pv::BALLS, pv::N_BALLS, Some(ball::SPEED)),
                PIECE => (pv::PIECES, pv::N_PIECES, None),
                _ => continue,
            };
            let k = w.mobys[id].pv_i(n);
            if MAX as i32 <= k { continue; }
            w.mobys[id].set_pv_i(slot + 4 * k as usize, i as i32);
            w.mobys[id].set_pv_i(n, k + 1);
            if let Some(o) = speed_at {
                let m = &mut w.mobys[i];
                m.set_pv_f(o, speed);
                m.mode |= 0x41;
            }
        }
        w.changed = true;
    }
    let m = &w.mobys[id];
    for (n, what) in [(pv::N_PULSES, "pulses 0x0a24"), (pv::N_BALLS, "balls 0x0a55"), (pv::N_PIECES, "pieces 0x0873")] {
        if m.pv_i(n) == 0 { eprintln!("electrolyzer: uid {} has no {what}", m.uid); }
    }
    let uid = m.uid;
    let g = &*ctx.game;
    let done = g.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || g.uid_count(uid) != 0;
    if done {
        hook(w, id);
        w.mobys[id].set_state(5, -1);
    } else {
        w.mobys[id].set_state(1, -1);
    }
    w.mobys[id].update_dist = 0xff;
}

/// `0x3c6748(moby, a, b)`: the phase `+0x128` on by π/30; its group's members lit with the light set 14 (in 7 their
/// ambient flashing between 0x5050dd and 0x505050, else 0x505010; a piece's twin with it); the lit classes (0x0af9..
/// 0x0afb, 0x0afd, 0x0afc, 0x0b03, a piece's twin) glowing between `a` and `b` by the phase.
pub fn tint(w: &mut MobyWorld, id: usize, (a, b): (u32, u32)) {
    let (all, live) = members(w, id);
    if live < 2 { return; }
    let phase = add_rot(w.mobys[id].pv_f(pv::PHASE), f32::from_bits(0x3dd6_7752));
    w.mobys[id].set_pv_f(pv::PHASE, phase);
    let failed = w.mobys[id].state == 7;
    for i in all {
        let class = w.mobys[i].o_class;
        let twin = if class == PIECE { idx(w, w.mobys[i].pv_i(piece::TWIN) - 1) } else { None };
        if class != CLASS {
            let c = if failed { pulse_colour(phase, 0x50_50dd, 0x50_5050) } else { 0x50_5010 };
            light(&mut w.mobys[i], c);
            if let Some(t) = twin { light(&mut w.mobys[t], c); }
        }
        if (0x0af9..=0x0afb).contains(&class) || matches!(class, 0x0afd | 0x0afc | 0x0b03) {
            w.mobys[i].glow = pulse_colour(phase, a, b);
        } else if let (PIECE, Some(t)) = (class, twin) {
            w.mobys[t].glow = pulse_colour(phase, a, b);
        }
    }
}

/// `0x3c61d0`: its phases `+0x128` / `+0x12c` on by 347 and 500 × π/10800; its glow `+0x78` between the pair by the
/// mean of their sines (5, solved) or the larger of them to the fourth.
pub fn own_glow(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    for k in 0..2 { m.set_pv_f(pv::PHASE + 4 * k, add_rot(m.pv_f(pv::PHASE + 4 * k), PHASE_STEP[k] * 0.000_290_888_25)); }
    let (sa, sb) = (m.pv_f(pv::PHASE).sin(), m.pv_f(pv::PHASE + 4).sin());
    let (f, (ca, cb)) = if m.state == 5 {
        ((sa + sb) * 0.25 + 0.5, OWN_DONE)
    } else {
        let f = (sa * 0.5 + 0.5).max(sb * 0.5 + 0.5);
        (f * f * f * f, OWN_PLAY)
    };
    m.glow = crate::particles::tween_color(f.to_bits(), ca, cb);
}

/// `0x3609b0` / `0x360a50`: into the game's mode 7 every moby is hidden (mode 0x80; the puzzle shows its own), out of
/// it every moby's 0x80 is cleared.
pub fn mode_switched(w: &mut MobyWorld, from: i32, to: i32) {
    if to == MODE && from != MODE {
        for m in w.mobys.iter_mut() { m.mode |= 0x80; }
    } else if from == MODE && to != MODE {
        for m in w.mobys.iter_mut() { m.mode &= !0x80; }
    } else {
        return;
    }
    w.changed = true;
}

/// `0x3c58e0` (the draw while in mode 7): the frame's four GUI sprites (0x70 and 0x6f in 0x60442d00, 0x71 and 0x72 in
/// 0x55f0c070) at (258, 203), the line on top (failed: 0x2df2 centred at 254; playing: 0x2df1 centred at 262) and
/// 0x2df0 centred at (254, 373), in font 3 (`0x2edbb0`: FX texture 3, `0x2ea6c8(3)`, its glyphs `0x28f150`;
/// crate::moby::classes::arena::FONT_3) in 0x88ffffff. `font`: font 3's glyphs and layer texture.
pub fn draw_hud(h: &PuzzleHud, text: &dyn Fn(u32) -> Vec<u8>, font: (&[rc_formats::disc::boot::Glyph; 256], usize), draws: &mut Vec<crate::hud::Draw>, out: &mut Vec<crate::gui::Quad>) {
    let (glyphs, slot) = font;
    for (id, colour) in [(0x70, 0x6044_2d00), (0x6f, 0x6044_2d00), (0x71, 0x55f0_c070), (0x72, 0x55f0_c070)] {
        draws.push(crate::hud::Draw::Sprite { id, pos: [258.0, 203.0], scale: [1.0, 1.0], frame: 0.0, colour });
    }
    let colour = 0x88ff_ffffu32.to_le_bytes();
    let mut line = |id: u32, x: f32, y: f32| {
        let t = text(id);
        let t = t.split(|&c| c == 0).next().unwrap_or(&[]);
        let width = crate::gui::text_width(glyphs, t, 1.0);
        crate::gui::text(out, glyphs, slot, t, x - (width >> 1) as f32, y, 1.0, colour);
    };
    match h.state {
        7 => line(0x2df2, 254.0, 23.0),
        8 | 5 => {}
        _ => line(0x2df1, 262.0, 23.0),
    }
    line(0x2df0, 254.0, 373.0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_screen_tangent_is_a_little_wider_than_play() {
        assert!((tan() - 0.629_977).abs() < 1e-5);
    }
}
