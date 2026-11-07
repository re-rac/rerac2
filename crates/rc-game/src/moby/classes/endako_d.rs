//! Endako (level 3)'s classes, set D: the city's breakables and its robots (L03 addresses; the engine functions they
//! call are named by their L03 addresses with the L00 ones the port knows them by where they differ).
//!
//! **The breakables.** Each waits in state 1 for a hit with the flags 0x4010000 (`0x31f0f0`, L00 `0x31dce8`) while
//! `+0x68`[0] = 0 and breaks in state 2: its pieces thrown (`0x32cd30`, L00 `0x32b580`, crate::moby::shards::
//! throw_all, from the `$gp` tables below: every one's skip piece `$gp+0x2a90` / `+0x2ad0` / `+0x2b10` is −1, so the
//! moby is deleted rather than left showing it), its class sound 0.
//! * **0x0f37** ([`update_0f37`], `0x41b360`, 50 placed): a lamp: two lasting glows 20 × its scale along its up row
//!   (crate::particles::glow, `0x340ae0` = L00 `0x33f550`); its break gives the level's bolts (`0x327b28` = L00
//!   `0x327388`, crate::moby::bolts::level_drop, 3 up), a blast of size 0 (`0x403f10`) and the general explosion's 15
//!   embers (`0x3219e0` = L00 `0x320538`) at the lamp's head, and six debris chunks out of each of its four corners
//!   (`0x328028` = L00 `0x327648`, crate::moby::shards::throw_chunks).
//! * **0x0f3e** ([`update_0f3e`], `0x41bc30`, 49 placed): a hanging lamp with a glow 8 × its scale under it; its break
//!   leaves its stub (0x0f40, `0x3285c0` = L00 `0x327898`) and a blast of size 0.75, and the lamp, hidden, pours smoke
//!   for 30 ticks ([`crate::particles::staged_box`], `0x2edd68`: 40 puffs, then 3 a tick out of a box that drifts
//!   aside) before it is deleted.
//! * **0x0f38** ([`update_0f38`], `0x41b750`, 13 placed): a box between two beams; its break gives a blast of size
//!   0.75, two debris chunks at each side and 15 puffs of smoke a side ([`crate::particles::staged_box`]).
//!
//! The breakables' count (`$gp−0x440`, `0x1aebb0`): each one with `+0x68`[0] ≠ 0 takes one off at its set-up, and
//! each break takes one off; no code the port has (nor any absolute read in the overlay) reads it, so it is not kept.
//!
//! Not ported (the breakables): 0x0f38's two beams (class 0x13ba, made by `0x42cdd0` while `+0x68`[4] = 0: from
//! (±12, −10, 20) to (±12, −25, 20) in its frame, colour 0x50285050, widths 0.5 / 1.5, kind 5, `+0x68`[8] ≠ 0 its
//! flag; its update `0x42c920` has no function in Ghidra and it draws itself); 0x0f3e keeps its glow's handle at
//! `+0x68`[4], which nothing reads.
//!
//! **The robots** 0x093c / 0x0940 / 0x0964 / 0x0a59 ([`update_robot`], `0x3c31f8`; mode 0x20, on crate::moby::ai's
//! framework, the movement block at `+0x110`): Clank's helpers. Each sleeps in a cage, its child, until Clank breaks
//! it, then follows him through the network of floor polygons of **0x0f2d** ([`update_0f2d`], `0x41b0b0`: built at
//! its set-up from its list of paths, `0x32b318`, crate::path_network::build_list; the openings between its polygons
//! no walls for the fences afterwards).
//! * **Set-up** (`0x3c4828`, state 0): mirrored at random (mode 0x8000 by `randi(2)`); on the floor under it
//!   (`0x31e228`, kept at `+0x3a4`), 0.089 up (0x0940 also 0.2 back); its child made at its place (`0x312738` +
//!   `0x32be48`: drawn, draw distance 0x30, its light and ambient; 0x0964's is 0x0972, the others' the cage **0x0d75**
//!   ([`update_cage`], `0x4096c8`) with the pieces `$gp+0xed0` (0x3fff) over it, 0x093c's 1.2 times its class's
//!   size); its movement block (radius 0.39 for 0x0940, 0.42 for 0x093c / 0x0a59, 0.15 for 0x0964; height 0.3, steps
//!   0.5, slope 1.588, gravity 1/720, the turn before walking 30°, stop 1.5); sequence 0 at a random frame; 1 (sub 0).
//!   0x0940 closes the records holding the unseen floors 0x097a of its eight groups `+0x200` (`0x41b120`); 0x0a59
//!   plays sequence 6.
//! * **Every tick** (`0x3c4828`): its shadow's slab probed within 24 of the camera (range 0x16); a hit with flag 1
//!   (`0x31f0f0`), or 10 below its start with the floor within 10 under it, and no robot's blast in the last 45 ticks
//!   (`$gp+0xec8` / `+0xecc`, crate::game::GameState::robot_blast_frame): a blast at 0.5 above it (`0x403f10`: size
//!   0.25, sprites 5 / 5 / 5, 10 puffs, 0x14 streaks, its light), 0x0964 breaks (`0x327e78`: pieces 0..3), its
//!   velocity stopped, 4 (sub 1), and it is beamed back to its child (`0x3c6c70` → `0x2ed370`, crate::moby::classes::
//!   teleport::start); the hit record let go.
//! * **1**: asleep until its child is hooked (`0x32bf38`): the help for its kind once (0x0940: 0x9cf in slot 0x26,
//!   0x093c: 0x9d1 / 0x28, 0x0a59: 0xa08 / 0x5f, while their counts are 0; 0x0964: 0x9d2 / 0x29 while never shown);
//!   4 (sub 1).
//! * **4**, beside Clank: its first tick sequence 0 (blend 15), the idles' count 2, still; Clank pressing on it
//!   (`0x18a090` / `0x18a08c`) pushes it ([`push`], `0x3c5970`) and it backs off ([`back_off`], `0x3c5848`); farther
//!   than 3 across from him with a way through the network: 3, sequence 2. At a sequence's end: from its idle 1 back
//!   to 0, else one time in its idles' count the idle 1 (the count a step toward 10, `0x31ff78`).
//! * **3**, following: toward Clank through the network ([`move_to`], `0x3c57c0`) fanned out by its place among its
//!   group's robots on the way (`0x3c6ef8`, every 16th frame; `0x3c70c0`); no way, within 1.5, or stuck over 60 ticks
//!   within 3: 4, sequence 0.
//! * **7**, sent (L14 only: `0x3cab28`, [`send_to`], by the prison's lifts and launch plates): toward its goal
//!   through the network for 60 ticks, then 4.
//! * **The sub-states' limits** (`0x3c31f8`, `limits`): the walk (`0x32f550`, accel / most a second, the turn's
//!   accel and most), `+0x134` and the stop distance `+0x144`: 1 (6 / 3, 0.2° / 6°, 1/600, 1.5) and 2 (10 / 5,
//!   0.4° / 12°, 1/180, 0) and 3 (6 / 3, 0.4° / 12°, 1/180, 0.1) on their first tick; 4 (1/600, 0.4), 5 (1/600, 0.05)
//!   and 6 (0.4° / 12°, 1/200, 0.05) every tick. Sub-state 8 (the knock-back, `0x3298a8`) nothing sets.
//!
//! **Clank's commands** (the command word `0x18c304` his ring sets, crate::hero::commands; `0x3c4828`'s end,
//! `command`): every robot awake (not 1), not entering (13) nor sent (sub-state 7), past its 30 ticks since the last
//! (`+0x364`) — in the bridge (sub-state 4) or the lift (5) only that command undoes it — takes it:
//! * **1 follow**: (not following already) farther than 3 across from Clank 3 (sub-state 1), else 4.
//! * **2 wait**: 2 (sub-state 0), its look about's countdown 60..120.
//! * **3 attack** (0x0964): the nearest enemy of its eight groups (`0x3c6348`: a target of class kind 5 within 15
//!   across and 1.5 in height it can reach): **9** after it, every 8th frame the nearest again, making way for a
//!   robot it pushes on, the enemy in a line 0.333 ahead: **10** the blow (a hit of 4 at frame 14, `0x31e6d0`); none:
//!   **11** looking for one for 300 ticks (sub-state 2).
//! * **4 enter** (0x0964): the nearest 0x0ef9 taking robots it can reach (`0x3c6560`): **12** to 1 before it (its
//!   radius 0.11 when first in line, `0x3c73d0`), **13** into it (`0x41aed8`), gone (sub-state 3).
//! * **6 bridge** (0x0940): the first 0x097a whose cuboid holds it or Clank (`0x3c6730`): **14** to its end 4.3 out
//!   (`0x3cc0f8`), **15** on (its network's record opened), **16** the floor (hooked: solid) until Clank is more than
//!   12 off, off it and out of its cuboid and none stands on it (`0x3c72f0`); then (or the command again) off
//!   (`0x3c74f0`: its record closed, at the end toward Clank, `0x3cc188`) **17**, then 6 (sub-state 4).
//! * **5 lift** (0x093c): the nearest 0x0e87 / 0x0e88 waiting within 14 (`0x3c68d8`): **18** beside it (`0x3cb750`),
//!   **19** holding it up (`0x3cb828`) until the sequence's end, then 6 (sub-state 5); the command again lets go.
//! * **7 zap** (0x0a59): the nearest arc 0x0b10 or launch plate 0x0f4d waiting within 10 (`0x3c6a90`): **20** before
//!   it (2 / 3 out, `0x38bea0` / `0x3cb838`), **21** the zap (hooked at frame 20), then 6 (sub-state 6).
//! * **8 call off**: 0x0964 beside Clank (4), the others' bow **6** (sequence 3 facing him, then 4).
//! * Each walk to its piece (14, 18, 20; 9) makes a robot of its group waiting or following in its way step aside
//!   (`0x3c7130`): **5**, 60 ticks toward the yaw given, then 4.
//! * Its helps: while awake and in reach of its piece, 0x9d0 (0x0940, slot 0x27), 0xa29 (0x093c, 0x80) and 0xa0a
//!   (0x0a59, 0x61) — again past 1080 ticks; the bridge, the lift and the zap turn them off.
//! * Its counts for the ring: awake within 16 across of Clank, and by kind; in sub-state 4.
//! * **0x16**, launched (L14 `0x3cabb0`, [`launch`], by the launch plates): along its velocity under its gravities,
//!   tumbling about its pitch; landed, 4 in the network that holds it.
//! * **The head** (`0x3c61e0` / `0x3c6018`): drawn, in 2 and 11, and in 4 with its idles past 5, it looks about; in 4
//!   else at Clank's centre; its two aim records (`+0x240` / `+0x2c0`, crate::moby::joint_aim, the walk turning them
//!   too) aimed in the tail, with Clank's big head the first's scale 1.75.
//! * **The tail** (`0x3c31f8`, `0x3c5a70`): in the air or on a moby, upright and held (`0x32e698`); its state's ticks
//!   counted, the command's 30 ticks counted down; drawn, its glow `+0x78` pulses between its sub-state's colours
//!   (`0x3c5b80`, `$gp+0xee0..`); its fence path the record it is in (`0x41b218`).
//!
//! Not ported (the robots): the list of robots the enemies may target (`0x32c138` → `0x1c63a8`, read by
//! crate::moby::ai::find_target's `0x32c4e8` and the hero's `0x2b3b00`); the colour flash `0x321690` (nothing starts
//! it); the glow's group stamp and draw hook (`0x2e8310(0x3c5e08)`); `0x3c6c70`'s search for a nearer 0x0972 of its
//! groups to beam back to (`0x2e0558`); the push on a moving floor moby (`0x3234d0`); the light `0x31bcac`. 0x0964's
//! child is the cage 0x0972 ([`update_0972`]).

use crate::hero::physics::{add, add_rot, atan2, set_len, V3};
use crate::moby::blast::{self, Blast, Explosion};
use crate::moby::{bolts, countdown, joint_aim, shards, Ctx, MobyWorld};

/// The hit flags the breakables answer (`0x31f0f0(m, 0x4010000, 0)`).
const BREAK_FLAGS: u32 = 0x0401_0000;
/// The debris chunks' colour (`0x328028`'s `0x50404040`).
const CHUNK_COLOUR: u32 = 0x5040_4040;

/// `0x3265b8(x, y, z, m, out)`: a point in the moby's frame (through its rows, unscaled) plus its place.
fn frame_point(w: &MobyWorld, id: usize, v: V3) -> V3 {
    let m = &w.mobys[id];
    add(through(&m.rows, v), m.pos)
}

/// `0x2f02d8(out, v, m + 0xc0)` (L00 `0x2ef318`): `v` through the moby's rows.
fn through(r: &[V3; 3], v: V3) -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) }

/// State 1 of every breakable: a hit with [`BREAK_FLAGS`] while `+0x68`[0] = 0 breaks it (2); the hit record let go.
fn wait_hit(w: &mut MobyWorld, id: usize) {
    if let Some(h) = w.hit_message(id) {
        if h.tmpl.flags & BREAK_FLAGS != 0 && w.mobys[id].pv_i(0) == 0 {
            w.mobys[id].set_state(2, -1);
        }
    }
    w.clear_hit(id);
}

/// `0x2edd68` on moby `id` (crate::particles::staged_box).
#[allow(clippy::too_many_arguments)]
fn smoke(w: &MobyWorld, id: usize, ctx: &mut Ctx, lo: V3, hi: V3, vlo: V3, vhi: V3, count: i32, kind: u8) {
    let m = &w.mobys[id];
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    crate::particles::staged_box(sys, &mut ctx.game.rng, &m.rows, m.pos, m.scale, lo, hi, vlo, vhi, count, kind);
}

/// 0x0f37 (`0x41b360`). **0**: the glows (`0x340ae0`): at its up row set to 20 × its scale (`0x2f00f8`), 367500 grey
/// 0x28208080 (its def's frame 0) and 325500 0x20208080 (frame 1) pulled 0.75, far 0x60, lasting; 1. **1**: the hit
/// ([`wait_hit`]). **2**: its pieces (`$gp+0x2a70`: x / y −3..3, z 1..7; spin 90..360 (`+0x2a68`), gravity 10..20
/// (`+0x2a60`); pointed 0x7f, flags 1: no bolts of their own), the level's bolts 3 up; at its head (0, 0, 20 × its
/// scale) a blast with every count 0 and size 0, and the explosion (`0x3219e0`: near 9, size 1, 15 embers, flags
/// 0x810001); out of the head's four corners (±1/30, ±1/30, 1/15 through its rows) six debris chunks each (reach 0.25,
/// 5 a second, 0.75..1.25 of their size, pointed); deleted.
pub fn update_0f37(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (pos, up) = (m.pos, set_len(m.rows[2], m.scale * 20.0));
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow_def(sys, &mut ctx.game.rng, 367_500.0, 0.0, up, id, pos, 0x60, false, 0x2820_8080, -1, 0);
                crate::particles::glow_def(sys, &mut ctx.game.rng, 325_500.0, 0.75, up, id, pos, 0x60, false, 0x2020_8080, -1, 1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => wait_hit(w, id),
        2 => {
            shards::throw_all(w, id, ctx, [-3.0, -3.0, 1.0, 0.0, 3.0, 3.0, 7.0, 0.0], (90.0, 360.0), (10.0, 20.0), 0x7f, 0, -1, 1, 0xffff);
            bolts::level_drop(w, id, 3.0, 1, ctx);
            let s = w.mobys[id].scale;
            let head = frame_point(w, id, [0.0, 0.0, s * 20.0]);
            blast::burst(w, ctx, head, &Blast::default());
            let e = Explosion { near: 9.0, size: 1.0, embers: 0xf, sound: -1, lod: -1, flags: 0x81_0001, ..Default::default() };
            blast::explode(w, ctx, Some(id), Some(head), &e);
            let head = frame_point(w, id, [0.0, 0.0, s * 20.0]);
            for i in 0..4 {
                let (x, y) = ((i / 2 * 2 - 1) as f32, (i % 2 * 2 - 1) as f32);
                let v = through(&w.mobys[id].rows, [x * 0.033_333_335, y * 0.033_333_335, 0.066_666_67]);
                shards::throw_chunks(w, ctx, 0.25, 5.0, (0.75, 1.25), id, head, 6, true, v, CHUNK_COLOUR, None);
            }
            w.delete(id);
        }
        _ => {}
    }
}

/// 0x0f3e (`0x41bc30`). Its pvars: `[0]` unbreakable, `[4]` the glow, `[8]` the smoke's timer. **0**: the glow
/// (`0x340ae0`: 4.2e6 × its scale, at (0, 0, −8) × its scale, pulled 1.5, 0x40202080, far its draw distance); 1.
/// **1**: the hit ([`wait_hit`]). **2**: its pieces (`$gp+0x2af0`: x / y −3..3, z −1..−7; spin 90..360, gravity
/// 10..20; pointed 0x3f, the level's bolts), its class sound 0 again (`0x361dc8`), the blast (`0x403f10`: size 0.75,
/// sprites 0x19 / 0x14 / 0x14, 0x14 puffs, 0x2d streaks, its light); hidden (mode 0x41), its stub 0x0f40 in its
/// place, 30 ticks; 3. **3**: smoke ([`smoke`], kind 1): 40 puffs on its first tick out of (−1, −1, −7)..(1, 1, −5)
/// at (−5, −5, −25)..(5, 5, −7) a second, then three a tick from a box and velocities eased by `1 − ticks/30` toward
/// (−3, −1, −6.5)..(−1, 1, −4.5) and (−15, −5, −10)..(−7, 5, 0) (`0x2efe58`); deleted when the ticks run out.
pub fn update_0f3e(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    const LO: V3 = [-1.0, -1.0, -7.0];
    const HI: V3 = [1.0, 1.0, -5.0];
    const LO2: V3 = [-3.0, -1.0, -6.5];
    const HI2: V3 = [-1.0, 1.0, -4.5];
    const VLO: V3 = [-5.0, -5.0, -25.0];
    const VHI: V3 = [5.0, 5.0, -7.0];
    const VLO2: V3 = [-15.0, -5.0, -10.0];
    const VHI2: V3 = [-7.0, 5.0, 0.0];
    match w.mobys[id].state {
        0 => {
            let m = &w.mobys[id];
            let (s, pos, far) = (m.scale, m.pos, m.draw_dist);
            if let Some(sys) = ctx.parts.as_deref_mut() {
                crate::particles::glow(sys, &mut ctx.game.rng, s * 4.2e6, 1.5, [0.0, 0.0, -8.0 * s], id, pos, far, false, 0x4020_2080, -1);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => wait_hit(w, id),
        2 => {
            shards::throw_all(w, id, ctx, [-3.0, -3.0, -1.0, 0.0, 3.0, 3.0, -7.0, 0.0], (90.0, 360.0), (10.0, 20.0), 0x3f, 0, -1, 0, 0xffff);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            let at = w.mobys[id].pos;
            blast::burst(w, ctx, at, &Blast { size: 0.75, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
            w.mobys[id].mode |= 0x41;
            w.mobys[id].set_pv_i(8, 0x1e);
            super::props::spawn_in_place(w, id, 0x0f40, 0, &*ctx.host);
            w.mobys[id].set_state(3, -1);
        }
        3 => {
            if w.mobys[id].first() {
                smoke(w, id, ctx, LO, HI, VLO, VHI, 0x28, 1);
            }
            let t = 1.0 - w.mobys[id].pv_i(8) as f32 * 0.033_333_335;
            let lerp = |a: V3, b: V3| -> V3 { std::array::from_fn(|k| a[k] + (b[k] - a[k]) * t) };
            let (vlo, vhi, lo, hi) = (lerp(VLO, VLO2), lerp(VHI, VHI2), lerp(LO, LO2), lerp(HI, HI2));
            smoke(w, id, ctx, lo, hi, vlo, vhi, 3, 1);
            let mut n = w.mobys[id].pv_i(8);
            let out = countdown(&mut n);
            w.mobys[id].set_pv_i(8, n);
            if out { w.delete(id); }
        }
        _ => {}
    }
}

/// 0x0f38 (`0x41b750`). **0**: its beams (not ported); 1. **1**: the hit ([`wait_hit`]). **2**: its pieces
/// (`$gp+0x2ab0`: x −4.5..4.5, y 2..4, z 2..6; spin 90..480, gravity 10..20; pointed 0x3f, the level's bolts), the
/// blast (as 0x0f3e's), its class sound 0; at (±12, −15, 20) × its scale two debris chunks each (reach 0.15, 2 a
/// second, 0.5..1 of their size, pointed, moving (0, −0.05, 0) through its rows a tick); smoke (kind 0) 15 puffs at
/// (±12, −12, 20) in its frame moving (−1.5, −5, −1.5)..(1.5, −15, 1.5) a second, the right side first; deleted.
pub fn update_0f38(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => w.mobys[id].set_state(1, -1),
        1 => wait_hit(w, id),
        2 => {
            shards::throw_all(w, id, ctx, [-4.5, 2.0, 2.0, 0.0, 4.5, 4.0, 6.0, 0.0], (90.0, 480.0), (10.0, 20.0), 0x3f, 0, -1, 0, 0xffff);
            let at = w.mobys[id].pos;
            blast::burst(w, ctx, at, &Blast { size: 0.75, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: false });
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            let s = w.mobys[id].scale;
            let v = through(&w.mobys[id].rows, [0.0, f32::from_bits(0xbd4c_ccce), 0.0]);
            for x in [12.0, -12.0] {
                let p = frame_point(w, id, [s * x, s * -15.0, s * 20.0]);
                shards::throw_chunks(w, ctx, 0.15, 2.0, (0.5, 1.0), id, p, 2, true, v, CHUNK_COLOUR, None);
            }
            for x in [12.0, -12.0] {
                let p = [x, -12.0, 20.0];
                smoke(w, id, ctx, p, p, [-1.5, -5.0, -1.5], [1.5, -15.0, 1.5], 0xf, 0);
            }
            w.delete(id);
        }
        _ => {}
    }
}

// ---------------------------------------------------------------------------------------------------------------
// The robots.

/// The robots' classes and their cage.
pub const ROBOT_93C: i16 = 0x093c;
pub const ROBOT_940: i16 = 0x0940;
pub const ROBOT_964: i16 = 0x0964;
pub const ROBOT_A59: i16 = 0x0a59;
pub const CAGE: i16 = 0x0d75;
/// The path network's class, and the robots' unseen floor (crate::moby::classes::endako_b::update_097a).
pub const NETWORK: i16 = 0x0f2d;
const FLOOR_97A: i16 = 0x097a;
/// 0x0964's child instead of a cage (crate::moby::classes::endako_c's).
const CHILD_964: i16 = 0x0972;

/// The network moby's pvars: the list of paths (`[0..0x20]` words, −1 ends it), the records (0x20 of them), their
/// count.
const NET_LIST: usize = 0;
const NET_NODES: usize = 0x80;
const NET_COUNT: usize = 0x480;
const NET_MAX: usize = 0x20;

/// The robot's pvars: its movement block (crate::moby::ai, `[0x18]`), its eight groups, its network moby (an index),
/// its child (a link), the glow's phase, its place in its group's spread, the records it and its goal are in, the
/// idles' count, the floor under its start.
const MOVE: usize = 0x110;
const GROUPS: usize = 0x200;
const NET: usize = 0x220;
const CHILD: usize = 0x354;
const PHASE: usize = 0x35c;
const SPREAD: usize = 0x360;
const AT: usize = 0x368;
const TO: usize = 0x36c;
const IDLES: usize = 0x380;
const START_Z: usize = 0x3a4;
/// State 7's goal and ticks (set by L14 `0x3cab28`, [`send_to`]); the goal is every command's (`+0x340`).
const WALK_GOAL: usize = 0x340;
const GOAL: usize = WALK_GOAL;
const WALK_TICKS: usize = 0x37c;
/// The command's piece (a link), its 30 ticks before the next command, the yaw a robot in the way steps aside to.
const TARGET: usize = 0x358;
const COOLDOWN: usize = 0x364;
const ASIDE: usize = 0x374;
/// The head's look about: its ticks, yaw, pitch, at Clank.
const LOOK_TICKS: usize = 0x384;
const LOOK_YAW: usize = 0x388;
const LOOK_PITCH: usize = 0x38c;
const LOOK_CLANK: usize = 0x390;
/// The launch (L14 `0x3cabb0`, [`launch`]): its gravities up and down, its tumble's speed, its landing height.
const LAUNCH_UP: usize = 0x394;
const LAUNCH_DOWN: usize = 0x398;
const LAUNCH_SPIN: usize = 0x39c;
const LAUNCH_LAND: usize = 0x3a0;
/// The two aim records (crate::moby::joint_aim; the movement block points at them, `+0x158` / `+0x15c`).
const AIM_A: usize = 0x240;
const AIM_B: usize = 0x2c0;
/// The pieces the commands use: 0x0ef9 (crate::moby::classes::endako_b), the arcs 0x0b10
/// (crate::moby::classes::level14::arcs), the launch plate 0x0f4d (crate::moby::classes::level14::plate).
const SLOT: i16 = 0x0ef9;
const ARC: i16 = 0x0b10;
pub const PLATE: i16 = 0x0f4d;
/// The movement block's fields: radius, walk accel / most, stop distance, options, fence path, push moby, floor moby,
/// floor height, stuck ticks, velocity, goal distance, turn limits and speed.
const RADIUS: usize = MOVE;
const WALK_ACCEL: usize = MOVE + 0x20;
const WALK_MOST: usize = MOVE + 0x28;
const STOP: usize = MOVE + 0x34;
const OPTS: usize = MOVE + 0x38;
const FENCE: usize = MOVE + 0x3c;
const STUCK: usize = MOVE + 0x80;
const PUSHER: usize = MOVE + 0x84;
const FLOOR_MOBY: usize = MOVE + 0x88;
const FLOOR_Z: usize = MOVE + 0x90;
const VEL: usize = MOVE + 0xa0;
const MOVE_FLAGS: usize = MOVE + 0x94;
const GOAL_DIST: usize = MOVE + 0xcc;
const TURN_ACCEL: usize = MOVE + 0x14;
const TURN_BRAKE: usize = MOVE + 0x18;
const TURN_MOST: usize = MOVE + 0x1c;
const TURN_SPEED: usize = MOVE + 0xb4;

/// L03 `$gp+0xeb0..`: the stop distance 1.5 and the follow distance 3; the walk (accel 6 a second², most 3 a second,
/// `$gp+0xec0` / `+0xec4`); the pushes (most 2 a second, accel 8, `+0xed4` / `+0xed8`); the blasts' least spacing 45
/// ticks (`+0xecc`); the cage's pieces drawn (`+0xed0`).
const NEAR: f32 = 1.5;
const FOLLOW: f32 = 3.0;
const WALK: (f32, f32) = (6.0, 3.0);
/// Sub-state 2's walk (accel 10 a second², most 5 a second, `$gp+0xeb8` / `+0xebc`).
const RUN: (f32, f32) = (10.0, 5.0);
const PUSH: (f32, f32) = (2.0, 8.0);
const BLAST_GAP: i32 = 0x2d;
const CAGE_PIECES: u16 = 0x3fff;
/// Degrees a second to radians a tick (`0x3998825c`).
const DEG_TICK: f32 = 0.000_290_888_25;

/// `+0x7c`: with 0x8000 one break piece drawn alone (Moby::piece), else the pieces drawn over the whole (bangles).
fn set_7c(m: &mut crate::moby::Moby, v: u16) {
    if v & 0x8000 != 0 {
        m.piece = Some((v & 0x7fff).trailing_zeros() as u8);
        m.bangles = 0;
    } else {
        m.piece = None;
        m.bangles = v;
    }
}

fn link(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { crate::moby::ai::link(&w.mobys[id], off).filter(|&i| i < w.mobys.len()) }

/// The members of group `g` (none for 0xff or past the groups).
fn members(w: &MobyWorld, g: i32) -> Vec<usize> {
    if g & 0xff == 0xff { return Vec::new(); }
    usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

fn live(w: &MobyWorld, i: usize) -> bool { w.mobys.get(i).is_some_and(|m| m.state != 0xfe && m.state != 0xfd) }

/// The class's sequence `seq` from a random frame (`0x31b5f0(m, seq, randi(frames), blend)`) unless it plays.
fn play_rand(w: &MobyWorld, id: usize, ctx: &mut Ctx, seq: u8, blend: u8) {
    if super::props::seq(w, id, ctx) == seq { return; }
    let n = ctx.host.seq_frames(&w.mobys[id], seq).unwrap_or(0) as i32;
    let f = ctx.game.rng.randi(n) as u8;
    ctx.host.set_anim(&w.mobys[id], seq, f, blend);
}

/// The network's moby (`+0x220`, a moby index), when it is one.
fn net_of(w: &MobyWorld, id: usize) -> Option<usize> {
    usize::try_from(w.mobys[id].pv_i(NET)).ok().filter(|&n| w.mobys.get(n).is_some_and(|m| m.o_class == NETWORK))
}

/// 0x0f2d's set-up (`0x41b0b0` state 0): without pvars deleted; else its network built from its list (`0x32b318`,
/// crate::path_network::build_list, 32 records at most), the count at `+0x480`, its openings kept; 1.
fn net_build(w: &mut MobyWorld, net: usize, ctx: &mut Ctx) {
    if w.mobys[net].pvars.is_empty() {
        w.delete(net);
        return;
    }
    let m = &mut w.mobys[net];
    if m.pvars.len() < NET_COUNT + 4 { m.pvars.resize(NET_COUNT + 4, 0); }
    let list: Vec<i32> = (0..NET_MAX).map(|k| m.pv_i(NET_LIST + 4 * k)).collect();
    let mut nodes = crate::path_network::read_nodes(&m.pvars, NET_NODES, NET_MAX);
    let n = crate::path_network::build_list(ctx.level, &list, &mut nodes);
    crate::path_network::write_nodes(&mut m.pvars, NET_NODES, &nodes[..n]);
    m.set_pv_i(NET_COUNT, n as i32);
    m.set_state(1, -1);
    crate::path_network::open(&mut ctx.game.path_openings, ctx.level, &nodes[..n]);
}

/// 0x0f2d (`0x41b0b0`, five placed): the robots' path network; nothing after its set-up ([`net_build`]).
pub fn update_0f2d(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 { net_build(w, id, ctx); }
}

/// The network's records.
fn net_nodes(w: &MobyWorld, net: Option<usize>) -> Vec<crate::path_network::Node> {
    let Some(m) = net.map(|n| &w.mobys[n]) else { return Vec::new() };
    let n = (m.pv_i(NET_COUNT).max(0) as usize).min(NET_MAX);
    crate::path_network::read_nodes(&m.pvars, NET_NODES, n)
}

/// A network moby by its index (the robots' `+0x220`, the prison's lifts' `+0x64`, 0x0aeb's `+0x00`), when it is one.
pub(crate) fn network(w: &MobyWorld, index: i32) -> Option<usize> {
    usize::try_from(index).ok().filter(|&n| w.mobys.get(n).is_some_and(|m| m.o_class == NETWORK))
}

/// `0x41b120(net, p, open)` (L14 `0x421f60`): the record that holds `p` closed to walking (flag 1) or opened; the
/// network built first if it is not yet.
pub(crate) fn net_mark(w: &mut MobyWorld, net: usize, ctx: &mut Ctx, p: V3, open: bool) {
    if w.mobys[net].state == 0 { net_build(w, net, ctx); }
    let mut nodes = net_nodes(w, Some(net));
    let mut k = -1;
    if !crate::path_network::locate(ctx.level, &nodes, p, &mut k) { return; }
    let n = &mut nodes[k as usize];
    if open { n.flags &= !1 } else { n.flags |= 1 }
    crate::path_network::write_nodes(&mut w.mobys[net].pvars, NET_NODES, &nodes);
}

/// L14 `0x4220d0(net, a, b, open)`: the link between the records that hold `a` and `b` (both found, not the same)
/// opened (bit 0x80 of the link byte clear) or closed (set) both ways — the first of each record's four links whose
/// low 7 bits name the other; the network built first if it is not yet. The walk ([`crate::path_network::steer`])
/// skips closed links. Its user: the prison's lifts (crate::moby::classes::level14::lift).
pub(crate) fn net_link(w: &mut MobyWorld, net: usize, ctx: &mut Ctx, a: V3, b: V3, open: bool) {
    if w.mobys[net].state == 0 { net_build(w, net, ctx); }
    let mut nodes = net_nodes(w, Some(net));
    let (mut i, mut j) = (-1, -1);
    if !crate::path_network::locate(ctx.level, &nodes, a, &mut i) || !crate::path_network::locate(ctx.level, &nodes, b, &mut j) || i == j {
        return;
    }
    for (x, y) in [(i as usize, j as u8), (j as usize, i as u8)] {
        if let Some(l) = nodes[x].links.iter_mut().find(|l| **l & 0x7f == y) {
            if open { *l &= 0x7f } else { *l |= 0x80 }
        }
    }
    crate::path_network::write_nodes(&mut w.mobys[net].pvars, NET_NODES, &nodes);
}

/// L14 `0x3cab28(r, p)`: a robot in sub-state 1, not already in 7, sent to walk to `p` (state 7, 60 ticks) when the
/// network has a way there from it ([`reachable`]). Its users: the prison's lifts keeping their prisoners on them
/// (crate::moby::classes::level14::lift, `0x3e0718`) and 0x0f4d (`0x3ccd38`).
pub(crate) fn send_to(w: &mut MobyWorld, r: usize, ctx: &mut Ctx, p: V3) {
    let m = &w.mobys[r];
    if m.sub_state != 1 || m.state == 7 { return; }
    if !reachable(w, net_of(w, r), ctx, m.pos, p) { return; }
    let m = &mut w.mobys[r];
    m.set_state(7, -1);
    m.set_pv_i(WALK_TICKS, 0x3c);
    m.set_pv_v(WALK_GOAL, p);
}

/// L14 `0x3caa18(m)`: its sub-state (`+0x95`) is 1 (whatever its class).
pub(crate) fn sub_one(w: &MobyWorld, r: usize) -> bool { w.mobys.get(r).is_some_and(|m| m.sub_state == 1) }

/// `0x41b298(net, from, to)` → `0x41b2c0`: a way from `from` to `to` through the network (crate::path_network::
/// steer, 0.1 narrowing the openings, fresh records).
fn reachable(w: &MobyWorld, net: Option<usize>, ctx: &Ctx, from: V3, to: V3) -> bool {
    let nodes = net_nodes(w, net);
    let (mut yaw, mut at, mut goal) = (0.0, -1, -1);
    crate::path_network::steer(ctx.level, &nodes, 0.1, from, to, &mut yaw, &mut at, &mut goal)
}

/// `0x3c70c0`: the yaw offset of its place in the spread: the angle 1.5 subtends at the goal's distance (at most
/// 85.7°) × 0.2 × its place (`+0x360`).
fn spread_offset(m: &crate::moby::Moby) -> f32 {
    let a = atan2(m.pv_f(GOAL_DIST), 1.5).min(1.495_996_6);
    a * 0.2 * m.pv_i(SPREAD) as f32
}

/// `0x3c57c0(m, goal, spread)`: the heading through the network toward `goal` (`0x41b1d0` → crate::path_network::
/// steer_offset with its radius `+0x110`, the records kept in `+0x368` / `+0x36c`; with `spread` its spread's
/// offset), then the framework's move (`0x32e520`, crate::moby::ai::move_step). Whether the goal is reachable.
fn move_to(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, goal: V3, spread: bool) -> bool {
    let nodes = net_nodes(w, net_of(w, id));
    let m = &mut w.mobys[id];
    let off = if spread { spread_offset(m) } else { 0.0 };
    let (mut at, mut to, mut yaw) = (m.pv_i(AT), m.pv_i(TO), 0.0);
    let ok = crate::path_network::steer_offset(ctx.level, &nodes, off, m.pv_f(RADIUS), m.pos, goal, &mut yaw, &mut at, &mut to);
    m.set_pv_i(AT, at);
    m.set_pv_i(TO, to);
    crate::moby::ai::move_step(w, id, ctx, yaw, goal);
    ok
}

/// `0x3c6ef8(m, p)`: its place in the spread toward `p`: over the live members of its group (`+0x21`) in its
/// sub-state but itself, −1 for each on its left of the line to `p` and +1 for each on its right, held to ±5.
fn spread(w: &MobyWorld, id: usize, p: V3) -> i32 {
    let m = &w.mobys[id];
    if m.group as u8 == 0xff { return 0; }
    let rows = crate::hero::physics::euler_rows([0.0, 0.0, atan2(p[0] - m.pos[0], p[1] - m.pos[1])]);
    let mut n = 0;
    for o in members(w, m.group as u8 as i32) {
        if o == id || !live(w, o) || w.mobys[o].sub_state != m.sub_state { continue; }
        let d = crate::hero::physics::sub(w.mobys[o].pos, m.pos);
        n += if 0.0 < crate::hero::physics::dot(d, rows[1]) { -1 } else { 1 };
    }
    n.clamp(-5, 5)
}

/// `0x32f4e8(yaw, m)`: its yaw eased toward `to` by its block's turn limits (`0x3206d8`).
fn turn_to(m: &mut crate::moby::Moby, to: f32) {
    let (a, b, c) = (m.pv_f(TURN_ACCEL), m.pv_f(TURN_BRAKE), m.pv_f(TURN_MOST));
    let (mut yaw, mut v) = (m.rot[2], m.pv_f(TURN_SPEED));
    crate::hero::physics::turn_ease(to, a, b, c, &mut yaw, &mut v, 0);
    m.rot[2] = yaw;
    m.set_pv_f(TURN_SPEED, v);
}

/// `0x3c5970(r, p)`: a robot in state 4 (or 2) with no floor moby pushed away from `p` across: 16/3600 added to its
/// velocity, which is held to 2/60.
fn push(w: &mut MobyWorld, r: usize, p: V3) {
    let m = &mut w.mobys[r];
    if !matches!(m.state, 2 | 4) || m.pv_i(FLOOR_MOBY) != 0 { return; }
    let mut d = crate::hero::physics::sub(m.pos, p);
    d[2] = 0.0;
    let d = set_len(d, (PUSH.1 + PUSH.1) * 0.000_277_777_78);
    let mut v = add(m.pv_v(VEL), d);
    let most = PUSH.0 * 0.016_666_668;
    let l = crate::hero::physics::len(v);
    if most < l { v = crate::hero::physics::scale(v, most / l); }
    m.set_pv_v(VEL, v);
}

/// `0x3c5848(m)`: while its velocity lasts it backs off: past 1/60 it turns to face against it, then the
/// framework's move in place with the push's limits (`0x32e328`: most 2 a second, accel 8, options 0x10), its record
/// found again (`0x41b250`), and a robot it pushes on is pushed in turn.
fn back_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let v = w.mobys[id].pv_v(VEL);
    let l = crate::hero::physics::len(v);
    if l <= 0.0 { return; }
    if 0.016_666_668 < l {
        let to = add_rot(atan2(v[0], v[1]), std::f32::consts::PI);
        turn_to(&mut w.mobys[id], to);
    }
    let m = &mut w.mobys[id];
    let saved = (m.pv_i(OPTS), m.pv_f(WALK_MOST), m.pv_f(WALK_ACCEL));
    m.set_pv_i(OPTS, 0x10);
    m.set_pv_f(WALK_MOST, PUSH.0 * 0.016_666_668);
    m.set_pv_f(WALK_ACCEL, PUSH.1 * 0.000_277_777_78);
    let (yaw, pos) = (m.rot[2], m.pos);
    crate::moby::ai::move_step(w, id, ctx, yaw, pos);
    let m = &mut w.mobys[id];
    m.set_pv_i(OPTS, saved.0);
    m.set_pv_f(WALK_MOST, saved.1);
    m.set_pv_f(WALK_ACCEL, saved.2);
    let nodes = net_nodes(w, net_of(w, id));
    let mut at = w.mobys[id].pv_i(AT);
    crate::path_network::locate(ctx.level, &nodes, w.mobys[id].pos, &mut at);
    w.mobys[id].set_pv_i(AT, at);
    if let Some(o) = link(w, id, PUSHER) {
        if matches!(w.mobys[o].o_class, ROBOT_940 | ROBOT_93C | ROBOT_964) {
            let p = w.mobys[id].pos;
            push(w, o, p);
        }
    }
}

/// The robot's set-up (`0x3c4828` in state 0); see the module.
fn robot_init(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mirror = ctx.game.rng.randi(2) != 0;
    let m = &mut w.mobys[id];
    if mirror { m.mode |= 0x8000 } else { m.mode &= 0x7fff }
    if let Some(z) = crate::moby::ground_under(ctx, m.pos) {
        m.pos[2] = z;
        m.set_pv_f(START_Z, z);
    }
    let class = m.o_class;
    let child_class = if class == ROBOT_964 { CHILD_964 } else { CAGE };
    if w.class(child_class).is_some() {
        let c = w.create(child_class);
        let (pos, rot, light, ambient) = { let m = &w.mobys[id]; (m.pos, m.rot, m.light, m.ambient) };
        let x = &mut w.mobys[c];
        x.update_dist = 0xff;
        x.draw_dist = 0xff;
        x.drawn = 1;
        x.parent = Some(id);
        x.pos = pos;
        x.rot = rot;
        x.light = light;
        x.ambient = ambient;
        w.rebuild(c, &*ctx.host);
        let base = w.class(child_class).map_or(1.0, |k| k.scale);
        let x = &mut w.mobys[c];
        x.mode &= 0xfff8;
        if child_class == CAGE { set_7c(x, CAGE_PIECES); }
        x.draw_dist = 0x30;
        if class == ROBOT_93C { x.scale = base * 1.2; }
        w.changed = true;
        crate::moby::ai::set_link(&mut w.mobys[id], CHILD, Some(c));
    }
    let m = &mut w.mobys[id];
    m.set_pv_b(0x28, 3);
    m.set_pv_f(0x20, 1.0);
    m.set_pv_s(0x24, 1);
    m.set_pv_f(STOP, NEAR);
    m.set_pv_f(MOVE + 4, f32::from_bits(0x3e99_999a));
    let r = match class {
        ROBOT_940 => 0x3ec7_ae14,
        ROBOT_93C | ROBOT_A59 => 0x3ed7_0a3d,
        _ => 0x3e19_999a,
    };
    m.set_pv_f(RADIUS, f32::from_bits(r));
    m.pos[2] += 0.089;
    if class == ROBOT_940 {
        // `0x2f00f8(−0.2, d, rows[0])`: 0.2 back along its facing.
        let d = set_len(m.rows[0], -0.2);
        m.pos = add(m.pos, d);
    }
    m.set_pv_f(MOVE + 0xc, 0.5);
    m.set_pv_f(MOVE + 0x10, f32::from_bits(0x3fcb_4bc5));
    m.set_pv_f(MOVE + 0x30, f32::from_bits(0x3f06_0a93));
    m.set_pv_f(MOVE + 0x2c, f32::from_bits(0x3bb6_0b61));
    m.set_pv_f(MOVE + 8, 0.5);
    // `+0x158` / `+0x15c`: the aim records the walk turns (crate::moby::ai::joint_turns).
    m.set_pv_i(MOVE + 0x48, AIM_A as i32);
    m.set_pv_i(MOVE + 0x4c, AIM_B as i32);
    m.set_state(1, 0);
    play_rand(w, id, ctx, 0, 0);
    if class == ROBOT_940 {
        let net = net_of(w, id);
        for k in 0..8 {
            let g = w.mobys[id].pv_i(GROUPS + 4 * k);
            for o in members(w, g) {
                if w.mobys.get(o).is_some_and(|x| x.o_class == FLOOR_97A) {
                    let p = w.mobys[o].pos;
                    if let Some(n) = net { net_mark(w, n, ctx, p, false); }
                }
            }
        }
    } else if class == ROBOT_A59 {
        super::props::set_seq(w, id, ctx, 6, 0, 0);
    }
}

/// `0x3c6c70` (the cage's place only) and `0x2ed370`: the robot beamed back to its child (cage), on the floor
/// under it, facing its way (crate::moby::classes::teleport::start).
fn respawn(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(c) = link(w, id, CHILD) else { return };
    let (mut p, rot) = (w.mobys[c].pos, w.mobys[c].rot);
    if let Some(z) = crate::moby::ground_under(ctx, p) { p[2] = z; }
    super::teleport::start(w, id, p, rot, &[]);
}

/// A help slot's step for the robots' helps (`0x18c958 + 0xc·slot`, `0x3c4828`): with the slot on, shown before, again
/// past 1080 ticks since it was last; the first time counted.
fn help_step(ctx: &mut Ctx, slot: usize, text: i32) {
    let now = ctx.game.play_time;
    let level_bit = 1u32.checked_shl(ctx.game.level.max(0) as u32).unwrap_or(0);
    let g = &mut *ctx.game;
    if g.help_counts.get(slot).is_none_or(|&c| c == -1) { return; }
    let last = g.help_last[slot];
    if g.help_counts[slot] != 0 {
        if 0x438 < now.wrapping_sub(last) as u32 || last == 0 {
            g.help(text, slot as i32);
        } else if (last as u32) < now as u32 {
            g.help_last[slot] = now;
        }
    } else {
        g.help_counts[slot] = g.help_counts[slot].wrapping_add(1);
        if (last as u32) < now as u32 { g.help_last[slot] = now; }
        g.help_levels[slot] |= level_bit | 0x8000_0000;
    }
}

/// A help slot turned off (its count −1: `0x18cb2c`, `0x18cf58`, `0x18cde4`).
fn help_off(ctx: &mut Ctx, slot: usize) {
    if let Some(c) = ctx.game.help_counts.get_mut(slot) { *c = -1; }
}

/// The robots' every-tick part before their state (`0x3c4828` past the set-up); see the module.
fn robot_pre(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 { robot_init(w, id, ctx); }
    crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16);
    let hit = w.hit_message(id).is_some_and(|h| h.tmpl.flags & 1 != 0);
    let m = &w.mobys[id];
    let z = m.pos[2];
    let fell = z < m.pv_f(START_Z) - 10.0 && z - 10.0 < m.pv_f(FLOOR_Z);
    if hit || fell {
        if BLAST_GAP < (ctx.game.robot_blast_frame - ctx.game.frame).abs() {
            ctx.game.robot_blast_frame = ctx.game.frame;
            let at = add(w.mobys[id].pos, [0.0, 0.0, 0.5]);
            blast::burst(w, ctx, at, &Blast { size: 0.25, bursts: [5, 5, 5], puffs: 10, streaks: 0x14, light: true, shake: false });
            if w.mobys[id].o_class == ROBOT_964 {
                let v = w.mobys[id].pv_v(VEL);
                shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0xf, v, 0xff);
            }
            let m = &mut w.mobys[id];
            m.set_state(4, 1);
            m.set_pv_v(VEL, [0.0; 3]);
            m.set_pv_i(MOVE + 0xd8, -1);
            respawn(w, id, ctx);
        }
        w.clear_hit(id);
    }
    let (class, state, sub, pos) = { let m = &w.mobys[id]; (m.o_class, m.state, m.sub_state, m.pos) };
    if sub == 4 { let r = &mut ctx.hero.body.commands.robots; r.bridges = r.bridges.wrapping_add(1); }
    // The helps of the robots' own commands, once one of their pieces is in reach (`0x18c958` slots 0x27, 0x80, 0x61).
    if state != 1 {
        if class == ROBOT_940 && find_floor(w, id, ctx).is_some() { help_step(ctx, 0x27, 0x9d0); }
        if class == ROBOT_93C && find_piece(w, id).is_some() { help_step(ctx, 0x80, 0xa29); }
        if class == ROBOT_A59 && find_zap(w, id).is_some() { help_step(ctx, 0x61, 0xa0a); }
        if crate::hero::physics::dist2(pos, ctx.hero.pos) < 16.0 {
            let r = &mut ctx.hero.body.commands.robots;
            // u16 counters that keep counting while the ring is up (`0x371b48` zeroes them only with it down).
            r.near = r.near.wrapping_add(1);
            match class {
                ROBOT_940 => r.near_940 = r.near_940.wrapping_add(1),
                ROBOT_93C => r.near_93c = r.near_93c.wrapping_add(1),
                ROBOT_A59 => r.near_a59 = r.near_a59.wrapping_add(1),
                _ => {}
            }
        }
    }
    command(w, id, ctx);
}

/// `0x3c4828`'s end: Clank's command word (`0x18c304`, crate::hero::commands) read — not asleep (1), entering (13),
/// sent (sub-state 7) or within its 30 ticks since the last (`+0x364`), and in the bridge's or the lift's sub-states
/// (4, 5) only their own command to undo them.
fn command(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let cmd = ctx.hero.body.commands.command;
    let (class, state, sub) = { let m = &w.mobys[id]; (m.o_class, m.state, m.sub_state) };
    if cmd == 0 || state == 1 || state == 13 || sub == 7 || w.mobys[id].pv_i(COOLDOWN) != 0 { return; }
    if (sub == 4 && cmd != 6) || (sub == 5 && cmd != 5) { return; }
    let hero = ctx.hero.pos;
    match cmd {
        // Follow.
        1 => {
            if sub == 1 { return; }
            if FOLLOW < crate::hero::physics::dist2(w.mobys[id].pos, hero) {
                w.mobys[id].set_state(3, 1);
                play_rand(w, id, ctx, 2, 10);
            } else {
                w.mobys[id].set_state(4, 1);
                play_rand(w, id, ctx, 0, 10);
            }
        }
        // Attack (0x0964).
        3 => {
            if sub == 2 || class != ROBOT_964 { return; }
            let t = find_enemy(w, id, ctx);
            set_target(w, id, t);
            if t.is_some() {
                w.mobys[id].set_state(9, 2);
                play_rand(w, id, ctx, 3, 10);
            } else {
                w.mobys[id].set_state(11, 2);
                play_rand(w, id, ctx, 0, 10);
            }
        }
        // Wait.
        2 => {
            if sub == 0 { return; }
            w.mobys[id].set_state(2, 0);
            play_rand(w, id, ctx, 0, 10);
            let n = ctx.game.rng.rand_range(60, 120);
            let m = &mut w.mobys[id];
            m.set_pv_i(LOOK_TICKS, n);
            m.set_pv_f(LOOK_YAW, 0.0);
            m.set_pv_f(LOOK_PITCH, 0.0);
        }
        // Enter (0x0964): into the nearest open 0x0ef9 it can reach.
        4 => {
            if class != ROBOT_964 { return; }
            w.mobys[id].set_pv_i(COOLDOWN, 30);
            if sub != 3 {
                let Some(t) = find_slot(w, id, ctx) else {
                    set_target(w, id, None);
                    return;
                };
                set_target(w, id, Some(t));
                let p = slot_point(&w.mobys[t]);
                if reachable(w, net_of(w, id), ctx, w.mobys[id].pos, p) {
                    w.mobys[id].set_pv_v(GOAL, p);
                    w.mobys[id].set_state(12, 3);
                    super::props::set_seq(w, id, ctx, 2, 0, 10);
                } else {
                    set_target(w, id, None);
                }
            } else if state != 13 {
                w.mobys[id].set_state(4, 1);
                super::props::set_seq(w, id, ctx, 0, 0, 10);
            }
        }
        // Bridge (0x0940): onto the unseen floor 0x097a of a cuboid Clank or it is in, or off it again.
        6 => {
            if class != ROBOT_940 { return; }
            w.mobys[id].set_pv_i(COOLDOWN, 30);
            if sub != 4 {
                let Some(t) = find_floor(w, id, ctx) else {
                    set_target(w, id, None);
                    return;
                };
                set_target(w, id, Some(t));
                w.mobys[id].set_state(14, 4);
                super::props::set_seq(w, id, ctx, 2, 0, 10);
                let (a, b) = floor_sides(&w.mobys[t]);
                let p = if reachable(w, net_of(w, id), ctx, w.mobys[id].pos, a) { a } else { b };
                w.mobys[id].set_pv_v(GOAL, p);
            } else if state == 14 {
                w.mobys[id].set_state(4, 1);
                super::props::set_seq(w, id, ctx, 0, 0, 20);
            } else if state == 16 && !held_up(w, id, ctx) {
                step_off(w, id, ctx);
            }
        }
        // Lift (0x093c): beside the nearest 0x0e87 / 0x0e88 waiting, or let it go.
        5 => {
            if class != ROBOT_93C { return; }
            if sub != 5 {
                let Some(t) = find_piece(w, id) else {
                    set_target(w, id, None);
                    return;
                };
                set_target(w, id, Some(t));
                w.mobys[id].set_state(18, 5);
                super::props::set_seq(w, id, ctx, 2, 0, 10);
                w.mobys[id].set_pv_i(COOLDOWN, 30);
                let p = piece_side(&w.mobys[t], w.mobys[id].pos);
                w.mobys[id].set_pv_v(GOAL, p);
                return;
            }
            if state != 19 { let_go(w, id, ctx); }
        }
        // Zap (0x0a59): an arc 0x0b10 or a launch plate 0x0f4d, or give it up.
        7 => {
            if class != ROBOT_A59 { return; }
            if sub != 6 {
                let Some(t) = find_zap(w, id) else {
                    set_target(w, id, None);
                    return;
                };
                set_target(w, id, Some(t));
                w.mobys[id].set_state(20, 6);
                super::props::set_seq(w, id, ctx, 2, 0, 10);
                w.mobys[id].set_pv_i(COOLDOWN, 30);
                let p = zap_point(&w.mobys[t]);
                w.mobys[id].set_pv_v(GOAL, p);
            } else if state != 21 {
                let_go(w, id, ctx);
            }
        }
        // Call off: 0x0964 back beside Clank, the others bow (6).
        8 => {
            if class == ROBOT_964 {
                if sub != 1 { w.mobys[id].set_state(4, 1); }
            } else {
                w.mobys[id].set_state(6, 1);
            }
        }
        _ => {}
    }
}

/// Commands 5 and 7 undone: its piece let go (`0x32bf18(t, 0)`), no piece, 30 ticks, beside Clank (4) in sequence 0.
fn let_go(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if let Some(t) = target(w, id) { w.mobys[t].once &= !4; }
    set_target(w, id, None);
    let m = &mut w.mobys[id];
    m.set_pv_i(COOLDOWN, 30);
    m.set_state(4, 1);
    super::props::set_seq(w, id, ctx, 0, 0, 20);
}

/// The robot's piece of its command (`+0x358`).
fn target(w: &MobyWorld, id: usize) -> Option<usize> { link(w, id, TARGET) }
fn set_target(w: &mut MobyWorld, id: usize, t: Option<usize>) { crate::moby::ai::set_link(&mut w.mobys[id], TARGET, t); }

/// The members of the robot's eight groups (`+0x200`), in order.
fn in_groups(w: &MobyWorld, id: usize) -> Vec<usize> {
    (0..8).flat_map(|k| members(w, w.mobys[id].pv_i(GROUPS + 4 * k))).collect()
}

/// The members of its own group (`+0x21`).
fn own_group(w: &MobyWorld, id: usize) -> Vec<usize> { members(w, w.mobys[id].group as u8 as i32) }

/// `0x3c6348`: the nearest enemy of its groups within 15 across it can reach — a target (mode 0x1000) of a class of
/// kind 5 (class header `+0x46`) within 1.5 of its height.
fn find_enemy(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<usize> {
    let (pos, net) = (w.mobys[id].pos, net_of(w, id));
    let mut best = (15.0, None);
    for e in in_groups(w, id) {
        let Some(m) = w.mobys.get(e).filter(|_| live(w, e)) else { continue };
        if m.mode & crate::moby::mode::TARGET == 0 || w.class(m.o_class).is_none_or(|c| c.kind != 5) { continue; }
        if 1.5 <= (pos[2] - m.pos[2]).abs() || !reachable(w, net, ctx, pos, m.pos) { continue; }
        let d = crate::hero::physics::dist2(pos, m.pos);
        if d < best.0 { best = (d, Some(e)); }
    }
    best.1
}

/// `0x3c6560`: the nearest 0x0ef9 of its groups within 15 across that takes robots ([`slot_open`]) and it can reach.
fn find_slot(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<usize> {
    let (pos, net) = (w.mobys[id].pos, net_of(w, id));
    let mut best = (15.0, None);
    for e in in_groups(w, id) {
        let Some(m) = w.mobys.get(e) else { continue };
        if m.o_class != SLOT || !slot_open(m) || !reachable(w, net, ctx, pos, m.pos) { continue; }
        let d = crate::hero::physics::dist2(pos, m.pos);
        if d < best.0 { best = (d, Some(e)); }
    }
    best.1
}

/// `0x41ae68`: a 0x0ef9 takes robots — in its state 2 with robots still to take (`[0x10]`).
fn slot_open(m: &crate::moby::Moby) -> bool { m.state == 2 && m.pv_i(0x10) != 0 }

/// `0x41ae88`: the place before a 0x0ef9 a robot walks to, 1 along its facing.
fn slot_point(m: &crate::moby::Moby) -> V3 { add(m.pos, set_len(m.rows[0], f32::from_bits(0x3f80_0000))) }

/// `0x41aed8(slot, r)`: robot `r` goes into the 0x0ef9 (`[0x1c]`, the moby it waits to see gone), one fewer to take;
/// none left, the 0x0ef9's state 3.
fn slot_enter(w: &mut MobyWorld, slot: usize, r: usize) {
    let m = &mut w.mobys[slot];
    m.set_pv_i(0x1c, r as i32);
    let n = m.pv_i(0x10) - 1;
    m.set_pv_i(0x10, n);
    if n == 0 { m.set_state(3, -1); }
}

/// `0x3c6730`: the first 0x097a of its groups whose cuboid (`[0x60]`, `0x3cc0d0`) holds it or Clank.
fn find_floor(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<usize> {
    let pos = w.mobys[id].pos;
    in_groups(w, id).into_iter().find(|&e| {
        w.mobys.get(e).is_some_and(|m| m.o_class == FLOOR_97A && (floor_holds(ctx, m, pos) || floor_holds(ctx, m, ctx.hero.pos)))
    })
}

/// `0x3cc0d0(floor, p)`: `p` in the 0x097a's cuboid `[0x60]`.
fn floor_holds(ctx: &Ctx, m: &crate::moby::Moby, p: V3) -> bool { ctx.level.in_cuboid(m.pv_i(0x60), p) }

/// `0x3cc0f8`: the 0x097a's two ends, 4.3 along its facing either way (`$gp+0x1314`).
fn floor_sides(m: &crate::moby::Moby) -> (V3, V3) {
    let d = set_len(m.rows[0], f32::from_bits(0x4089_999a));
    (add(m.pos, d), crate::hero::physics::sub(m.pos, d))
}

/// `0x3c68d8`: the nearest 0x0e87 / 0x0e88 of its groups within 14 across waiting to be raised (state 1, `0x3cb740`).
fn find_piece(w: &MobyWorld, id: usize) -> Option<usize> {
    let pos = w.mobys[id].pos;
    let mut best = (14.0, None);
    for e in in_groups(w, id) {
        let Some(m) = w.mobys.get(e) else { continue };
        if !matches!(m.o_class, 0x0e87 | 0x0e88) || m.state != 1 { continue; }
        let d = crate::hero::physics::dist2(pos, m.pos);
        if d < best.0 { best = (d, Some(e)); }
    }
    best.1
}

/// `0x3cb750(piece, from)`: the place beside the piece on `from`'s side, along its facing (its angles' rows): 1 for
/// 0x0e87 (`$gp+0x12a0`), else 2.5 (`0x1b0294`).
fn piece_side(m: &crate::moby::Moby, from: V3) -> V3 {
    let rows = crate::hero::physics::euler_rows(m.rot);
    let v = crate::gravity::to_local(crate::hero::physics::sub(from, m.pos), &m.rows);
    let mut s = if m.o_class == 0x0e87 { 1.0 } else { 2.5 };
    if v[0] < 0.0 { s = -s; }
    add(set_len(rows[0], s), m.pos)
}

/// `0x3c6a90`: the nearest arc 0x0b10, or launch plate 0x0f4d waiting (state 1, `0x3cb888`), of its groups within 10
/// across (none being deleted).
fn find_zap(w: &MobyWorld, id: usize) -> Option<usize> {
    let pos = w.mobys[id].pos;
    let mut best = (10.0, None);
    for e in in_groups(w, id) {
        let Some(m) = w.mobys.get(e).filter(|_| live(w, e)) else { continue };
        let ok = match m.o_class { ARC => true, PLATE => m.state == 1, _ => false };
        if !ok { continue; }
        let d = crate::hero::physics::dist2(pos, m.pos);
        if d < best.0 { best = (d, Some(e)); }
    }
    best.1
}

/// The place a 0x0a59 zaps from: before an arc 2 along its facing (`0x38bea0`, `$gp−0xcc`), before a plate 3
/// (`0x3cb838`, `$gp+0x12c0`).
fn zap_point(m: &crate::moby::Moby) -> V3 {
    let s = if m.o_class == ARC { 2.0 } else { 3.0 };
    add(crate::hero::physics::scale(m.rows[0], s), m.pos)
}

/// `0x3c7130(m, yaw)`: the robots of its group waiting or following (sub-state 0 / 1, awake) in its way — within 1
/// ahead, 0.7 across and 0.5 up or down in its frame — step aside toward `yaw` (5, sub-state 1, from tick 0).
fn step_aside(w: &mut MobyWorld, id: usize, yaw: f32) {
    if w.mobys[id].group as u8 == 0xff { return; }
    let (pos, rows) = (w.mobys[id].pos, w.mobys[id].rows);
    for e in own_group(w, id) {
        let Some(m) = w.mobys.get_mut(e) else { continue };
        if m.state == 1 || 2 <= m.sub_state { continue; }
        let v = crate::gravity::to_local(crate::hero::physics::sub(m.pos, pos), &rows);
        if 0.0 < v[0] && v[0] < 1.0 && v[1].abs() < 0.7 && v[2].abs() < 0.5 {
            m.set_state(5, 1);
            m.set_pv_f(ASIDE, yaw);
            m.state_ticks = 0;
        }
    }
}

/// `0x3c72f0`: someone stands on its 0x097a — Clank (`0x18a15c`) or a robot of its group (its floor moby `+0x198`).
fn held_up(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let Some(t) = target(w, id) else { return false };
    if ctx.hero.ground_moby == Some(t) { return true; }
    if w.mobys[id].group as u8 == 0xff { return false; }
    own_group(w, id).into_iter().any(|e| w.mobys.get(e).is_some_and(|m| crate::moby::ai::link(m, FLOOR_MOBY) == Some(t)))
}

/// `0x3c73d0(m, goal)`: no robot of its group after the same piece is nearer `goal` across.
fn first_in_line(w: &MobyWorld, id: usize, goal: V3) -> bool {
    let d = crate::hero::physics::dist2(w.mobys[id].pos, goal);
    let t = w.mobys[id].pv_i(TARGET);
    own_group(w, id).into_iter().all(|e| e == id || w.mobys.get(e).is_none_or(|m| m.pv_i(TARGET) != t || d <= crate::hero::physics::dist2(m.pos, goal)))
}

/// `0x3c74f0`: off the 0x097a — its record closed again (`0x41b120(net, p, 0)`); the end toward Clank
/// ([`floor_end`]) clear of the world for a sphere of its radius at its height there (`0x2e0558`, flags 0): that end
/// its goal, the floor let go, no piece, its collision back (`+0x98`), stepping off (0x11, sub-state 4) in sequence
/// 0; else the record opened again.
fn step_off(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(t) = target(w, id) else { return };
    let net = net_of(w, id);
    let tp = w.mobys[t].pos;
    if let Some(n) = net { net_mark(w, n, ctx, tp, false); }
    let p = floor_end(w, t, ctx);
    let m = &w.mobys[id];
    let r = m.pv_f(RADIUS);
    let test = [p[0], p[1], p[2] + r + m.pv_f(MOVE + 4)];
    let blocked = ctx.coll.is_some_and(|c| {
        let live = |i: usize| w.mobys.get(i).is_some_and(|m| m.state < 0x80 && m.has_collision);
        crate::collision::coll_sphere_live(c, ctx.scene, test, r, crate::collision::QueryFlags(0), Some(id), &live).is_some()
    });
    if blocked {
        if let Some(n) = net { net_mark(w, n, ctx, tp, true); }
        return;
    }
    w.mobys[id].set_pv_v(GOAL, p);
    w.mobys[t].once &= !4;
    set_target(w, id, None);
    let solid = w.class(w.mobys[id].o_class).is_some_and(|c| c.has_collision);
    let m = &mut w.mobys[id];
    m.has_collision = solid;
    m.set_state(0x11, 4);
    super::props::set_seq(w, id, ctx, 0, 0, 0x14);
}

/// `0x3cc188(floor, _, out)`: the 0x097a's end to step off at: the first from which the floor's network (`[0x64]`)
/// has a way to Clank, else the first in a record of it, else the nearer to Clank across; 0.5 up and down on the
/// ground (`0x31e228`).
fn floor_end(w: &MobyWorld, t: usize, ctx: &Ctx) -> V3 {
    let (a, b) = floor_sides(&w.mobys[t]);
    let net = network(w, w.mobys[t].pv_i(0x64));
    let hero = ctx.hero.pos;
    let locate = |p: V3| {
        let mut k = -1;
        crate::path_network::locate(ctx.level, &net_nodes(w, net), p, &mut k);
        k
    };
    let mut p = if reachable(w, net, ctx, a, hero) {
        a
    } else if reachable(w, net, ctx, b, hero) {
        b
    } else {
        let region = locate(a);
        if region != -1 {
            a
        } else if locate(b) != region {
            b
        } else if crate::hero::physics::dist2(hero, a) < crate::hero::physics::dist2(hero, b) {
            a
        } else {
            b
        }
    };
    p[2] += 0.5;
    p[2] = crate::moby::ai::ground_z(ctx, 0.5, p, 0);
    p
}

/// `0x3c61e0`: the head's look about — on its countdown's end (`+0x384`) a new one: waiting (sub-state 2), 5..60°
/// either way and −20..0° up for 30..45 ticks; else 5..20° either way, −15..5° up, and one time in four a look at
/// Clank (`+0x390`) for 180..300 ticks, else 60..120. Then the look at Clank's centre ([`look_at`]) or the turn
/// asked of both aim records (`+0x2a4` / `+0x2a8`, `+0x324` / `+0x328`).
fn look_around(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    {
        let rng = &mut ctx.game.rng;
        let m = &mut w.mobys[id];
        let mut t = m.pv_i(LOOK_TICKS);
        let out = countdown(&mut t);
        m.set_pv_i(LOOK_TICKS, t);
        if out {
            const DEG: f32 = 0.017_453_294;
            if m.sub_state == 2 {
                m.set_pv_f(LOOK_YAW, rng.randf_signed(5.0, 60.0) * DEG);
                m.set_pv_f(LOOK_PITCH, rng.randf(-20.0, 0.0) * DEG);
                m.set_pv_i(LOOK_TICKS, rng.rand_range(30, 45));
                m.set_pv_i(LOOK_CLANK, 0);
            } else {
                m.set_pv_f(LOOK_YAW, rng.randf_signed(5.0, 20.0) * DEG);
                m.set_pv_f(LOOK_PITCH, rng.randf(-15.0, 5.0) * DEG);
                let at = rng.randi(4) == 0;
                m.set_pv_i(LOOK_CLANK, at as i32);
                m.set_pv_i(LOOK_TICKS, if at { rng.rand_range(180, 300) } else { rng.rand_range(60, 120) });
            }
        }
    }
    if w.mobys[id].pv_i(LOOK_CLANK) != 0 {
        let c = add(ctx.hero.pos, [0.0, 0.0, 1.0]);
        look_at(w, id, ctx, c);
    } else {
        let m = &mut w.mobys[id];
        let (u, v) = (m.pv_f(LOOK_PITCH), m.pv_f(LOOK_YAW));
        for b in [AIM_A, AIM_B] {
            m.set_pv_f(b + joint_aim::ASK + 4, u);
            m.set_pv_f(b + joint_aim::ASK + 8, v);
        }
    }
}

/// `0x3c6018(m, p)`: the head turned to look at `p` (none for a 0x093c in its sequence 1): `p` in its frame, less its
/// eyes' height (0.1 for 0x0964, 0.8 the others: `$gp+0x1010..`), the yaw within ±90° and the pitch within ±45°,
/// each halved, asked of both aim records.
fn look_at(w: &mut MobyWorld, id: usize, ctx: &Ctx, p: V3) {
    let m = &w.mobys[id];
    if m.o_class == ROBOT_93C && ctx.host.anim(m).is_some_and(|a| a.seq_a == 1) { return; }
    let mut v = crate::gravity::to_local(crate::hero::physics::sub(p, m.pos), &m.rows);
    v[2] -= if m.o_class == ROBOT_964 { f32::from_bits(0x3dcc_cccd) } else { f32::from_bits(0x3f4c_cccd) };
    use std::f32::consts::{FRAC_PI_2, FRAC_PI_4};
    let yaw = atan2(v[0], v[1]).clamp(-FRAC_PI_2, FRAC_PI_2) * 0.5;
    let pitch = (-atan2(crate::hero::physics::len2(v), v[2])).clamp(-FRAC_PI_4, FRAC_PI_4) * 0.5;
    let m = &mut w.mobys[id];
    for b in [AIM_A, AIM_B] {
        m.set_pv_f(b + joint_aim::ASK + 4, pitch);
        m.set_pv_f(b + joint_aim::ASK + 8, yaw);
    }
}

/// `0x31ffd8(p, goal, step)`: `p` moved toward `goal` by at most `step`.
fn step_toward(p: V3, goal: V3, step: f32) -> V3 {
    let d = crate::hero::physics::sub(goal, p);
    let l = crate::hero::physics::len(d).clamp(-step, step);
    add(p, set_len(d, l))
}

/// The yaw from the robot to its piece.
fn yaw_to(w: &MobyWorld, id: usize, t: usize) -> f32 {
    let (a, b) = (w.mobys[id].pos, w.mobys[t].pos);
    atan2(b[0] - a[0], b[1] - a[1])
}

/// The glow (`0x3c5b80`), drawn: its phase steps by the sub-state's speed and the glow word goes between the
/// sub-state's two colours by `sin(phase)·0.5 + 0.5` (`$gp+0xee0..0xf2c`).
fn glow(m: &mut crate::moby::Moby) {
    if m.drawn == 0 { return; }
    let (a, b, speed) = match m.sub_state {
        1 | 7 => (0x4000_8000, 0x2000_4000, 240.0),
        2 | 8 => (0x4000_0080, 0x2000_0040, 360.0),
        3 => (0x40c0_2020, 0x2080_2020, 360.0),
        4 => (0x4080_0080, 0x2060_0060, 360.0),
        5 => (0x4000_4080, 0x2000_6080, 360.0),
        6 => (0x4080_6000, 0x2040_3000, 360.0),
        _ => (0x4000_8080, 0x2000_4040, 120.0),
    };
    let ph = add_rot(m.pv_f(PHASE), speed * DEG_TICK);
    m.set_pv_f(PHASE, ph);
    m.glow = crate::hud::lerp_rgba(ph.sin() * 0.5 + 0.5, a, b);
}

/// `0x3c31f8`'s sub-state limits: the walk's (`0x32f550`, crate::moby::ai::senses; accel and most a second², a
/// second), its accel `+0x134` and its stop distance `+0x144` — sub-states 1..3 on their first tick, 4..6 every tick.
fn limits(m: &mut crate::moby::Moby) {
    const SLOW: (f32, f32) = (f32::from_bits(0x3b64_c38a), f32::from_bits(0x3dd6_7752));
    const QUICK: (f32, f32) = (f32::from_bits(0x3be4_c38a), f32::from_bits(0x3e56_7752));
    let first = m.once & 2 == 0;
    let (walk, turn, accel, stop) = match m.sub_state {
        1 if first => (WALK, SLOW, f32::from_bits(0x3ada_740e), NEAR),
        2 if first => (RUN, QUICK, f32::from_bits(0x3bb6_0b61), 0.0),
        3 if first => (WALK, QUICK, f32::from_bits(0x3bb6_0b61), f32::from_bits(0x3dcc_cccd)),
        4 => (WALK, SLOW, f32::from_bits(0x3ada_740e), f32::from_bits(0x3ecc_cccd)),
        5 => (WALK, SLOW, f32::from_bits(0x3ada_740e), f32::from_bits(0x3d4c_cccd)),
        6 => (WALK, QUICK, f32::from_bits(0x3ba3_d70a), f32::from_bits(0x3d4c_cccd)),
        _ => return,
    };
    if matches!(m.sub_state, 1..=3) { m.once |= 2; }
    crate::moby::ai::senses(m, walk.0 * 0.000_277_777_78, walk.1 * 0.016_666_668, turn.0, turn.1);
    m.set_pv_f(MOVE + 0x24, accel);
    m.set_pv_f(STOP, stop);
}

/// 0x093c / 0x0940 / 0x0964 / 0x0a59 (`0x3c31f8`): Endako's and the prison's robots; see the module.
pub fn update_robot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    robot_pre(w, id, ctx);
    limits(&mut w.mobys[id]);
    let hero = ctx.hero.pos;
    // Clank pressing on it (`0x18a090` / `0x18a08c`); its turn of the frames spread over 4, 8 and 16.
    let pushed = ctx.hero.wall_moby == Some(id) || ctx.hero.push_moby == Some(id);
    let frame = ctx.game.frame;
    let every = |n: i32| frame & n == (id as i32) & n;
    match w.mobys[id].state {
        1 => {
            let woke = link(w, id, CHILD).is_some_and(|c| w.mobys[c].once & 4 != 0);
            if woke {
                let (text, slot) = match w.mobys[id].o_class {
                    ROBOT_940 => (0x9cf, 0x26),
                    ROBOT_93C => (0x9d1, 0x28),
                    ROBOT_A59 => (0xa08, 0x5f),
                    _ => (0x9d2, 0x29),
                };
                let k = slot as usize;
                let ask = if slot == 0x29 { (ctx.game.help_levels[k] as i32) >= 0 } else { ctx.game.help_counts[k] == 0 };
                if ask { ctx.game.help(text, slot); }
                w.mobys[id].set_state(4, 1);
            }
        }
        // Waiting (command 2): pushed by Clank and backing off, looking about while drawn.
        2 => {
            if pushed { push(w, id, hero); }
            back_off(w, id, ctx);
            if w.mobys[id].drawn != 0 { look_around(w, id, ctx); }
        }
        3 => {
            let ok = move_to(w, id, ctx, hero, true);
            let m = &w.mobys[id];
            let d = m.pv_f(GOAL_DIST);
            if !ok || d < NEAR || (0x3c < m.pv_i(STUCK) && d < FOLLOW) {
                w.mobys[id].set_state(4, -1);
                play_rand(w, id, ctx, 0, 10);
            }
            if every(0xf) {
                let n = spread(w, id, hero);
                w.mobys[id].set_pv_i(SPREAD, n);
            }
        }
        4 => {
            if w.mobys[id].first() {
                play_rand(w, id, ctx, 0, 0xf);
                let m = &mut w.mobys[id];
                m.set_pv_i(IDLES, 2);
                m.set_pv_v(VEL, [0.0; 3]);
            }
            if pushed { push(w, id, hero); }
            back_off(w, id, ctx);
            let net = net_of(w, id);
            let pos = w.mobys[id].pos;
            if FOLLOW < crate::hero::physics::dist2(pos, hero) && reachable(w, net, ctx, pos, hero) {
                w.mobys[id].set_state(3, -1);
                play_rand(w, id, ctx, 2, 10);
            } else if w.mobys[id].drawn != 0 && ctx.host.anim(&w.mobys[id]).is_none_or(|a| a.seq_a != 1) {
                // The head: at Clank's centre while its idles are few, else about.
                if w.mobys[id].pv_i(IDLES) < 5 { look_at(w, id, ctx, add(hero, [0.0, 0.0, 1.0])) } else { look_around(w, id, ctx) }
            }
            let a = ctx.host.anim(&w.mobys[id]);
            if super::props::wrapped(w, id, ctx) && a.is_some_and(|a| a.seq_a == a.seq_b) {
                if a.is_some_and(|a| a.seq_a == 1) {
                    let n = ctx.host.seq_frames(&w.mobys[id], 0).unwrap_or(0) as i32;
                    let f = ctx.game.rng.randi(n) as u8;
                    ctx.host.set_anim(&w.mobys[id], 0, f, 0xf);
                } else {
                    let n = w.mobys[id].pv_i(IDLES);
                    if ctx.game.rng.randi(n) == 0 {
                        // `0x31ff78(&n, 10, 1)`: a step toward 10.
                        let m = &mut w.mobys[id];
                        m.set_pv_i(IDLES, n + (10 - n).clamp(-1, 1));
                        super::props::set_seq(w, id, ctx, 1, 0, 0xf);
                    }
                }
            }
        }
        // Stepping aside (`0x3c7130`) toward `+0x374` for 60 ticks, fanned out, in sequence 2; then beside Clank.
        5 => {
            if w.mobys[id].first() {
                play_rand(w, id, ctx, 2, 20);
                w.mobys[id].set_pv_f(STOP, f32::from_bits(0x3dcc_cccd));
            }
            let m = &w.mobys[id];
            let h = m.pv_f(ASIDE);
            let p = add([h.cos(), h.sin(), 0.0], m.pos);
            move_to(w, id, ctx, p, true);
            if 61 <= w.mobys[id].state_ticks {
                w.mobys[id].set_pv_f(STOP, NEAR);
                w.mobys[id].set_state(4, 1);
                play_rand(w, id, ctx, 0, 10);
            }
        }
        // The bow (command 8, sequence 3; state 8's sequence 7, which nothing sets) facing Clank, then beside him.
        6 | 8 => {
            if w.mobys[id].first() {
                let s = if w.mobys[id].state == 8 { 7 } else { 3 };
                super::props::set_seq(w, id, ctx, s, 0, 20);
            }
            let p = w.mobys[id].pos;
            turn_to(&mut w.mobys[id], atan2(hero[0] - p[0], hero[1] - p[1]));
            if super::props::wrapped(w, id, ctx) { w.mobys[id].set_state(4, -1); }
        }
        7 => {
            // Sent (L14 `0x3cab28`, [`send_to`]): its first tick the stop distance 0.1; farther across than that from
            // its goal it walks there through the network (`0x3c57c0`, no spread) playing 2, else it plays 0 (blend
            // 20 each); its ticks out (`0x2eeab0`), 4.
            if w.mobys[id].first() { w.mobys[id].set_pv_f(STOP, 0.1); }
            let m = &w.mobys[id];
            let goal = m.pv_v(WALK_GOAL);
            if m.pv_f(STOP) < crate::hero::physics::dist2(m.pos, goal) {
                move_to(w, id, ctx, goal, false);
                play_rand(w, id, ctx, 2, 0x14);
            } else {
                play_rand(w, id, ctx, 0, 0x14);
            }
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(WALK_TICKS);
            let out = countdown(&mut t);
            m.set_pv_i(WALK_TICKS, t);
            if out { m.set_state(4, -1); }
        }
        // Attack (0x0964): after its enemy, every 8th frame the nearest again; it makes way for a robot it pushes on
        // (straight on a 0x097a, else to the side away from it); the enemy in a line 0.333 ahead (every 4th frame,
        // 0.2 up, `0x2df4f8` flags 0): the blow (10) in sequence 4.
        9 => {
            if every(7) {
                let t = find_enemy(w, id, ctx);
                set_target(w, id, t);
            }
            if let Some(p) = link(w, id, PUSHER) {
                let m = &w.mobys[id];
                let yaw = m.rot[2];
                let on_floor = link(w, id, FLOOR_MOBY).is_some_and(|f| w.mobys[f].o_class == FLOOR_97A);
                let h = if on_floor {
                    yaw
                } else {
                    let turn = crate::hero::physics::sub_rot(yaw_to(w, id, p), yaw);
                    add_rot(if turn < 0.0 { -std::f32::consts::FRAC_PI_2 } else { std::f32::consts::FRAC_PI_2 }, yaw)
                };
                step_aside(w, id, h);
            }
            let Some(t) = target(w, id) else {
                w.mobys[id].set_state(11, -1);
                play_rand(w, id, ctx, 0, 20);
                return robot_tail(w, id, ctx);
            };
            let goal = w.mobys[t].pos;
            move_to(w, id, ctx, goal, false);
            if every(3) {
                let m = &w.mobys[id];
                let a = add(m.pos, [0.0, 0.0, 0.2]);
                let b = add(add(set_len(m.rows[0], f32::from_bits(0x3eaa_7efa)), m.pos), [0.0, 0.0, 0.2]);
                let hit = w.coll_line(ctx, a, b, 0, Some(id)).and_then(|o| o.moby);
                if hit == Some(t) {
                    w.mobys[id].set_state(10, -1);
                    super::props::set_seq(w, id, ctx, 4, 0, 10);
                }
            }
        }
        // The blow: at frame 14 a hit of 4 (`0x31e6d0`: 0.333 round at joint 1, flags 0x10000); at its end the
        // nearest enemy again: after it (9, sequence 3), or none (11).
        10 => {
            if crate::moby::ai::reached(w, id, ctx, 14.0) {
                crate::moby::ai::attack_joint(w, id, ctx, f32::from_bits(0x3eaa_7efa), 4.0, 1.0, 1, 0x10000, 0, 1, 0);
            } else if super::props::wrapped(w, id, ctx) {
                let t = find_enemy(w, id, ctx);
                set_target(w, id, t);
                if t.is_some() {
                    w.mobys[id].set_state(9, -1);
                    play_rand(w, id, ctx, 3, 10);
                } else {
                    w.mobys[id].set_state(11, -1);
                    play_rand(w, id, ctx, 0, 10);
                }
            }
        }
        // No enemy: looking about, every 8th frame for one; one found, after it (9); none for 300 ticks, beside Clank.
        11 => {
            if every(7) {
                let t = find_enemy(w, id, ctx);
                set_target(w, id, t);
            }
            look_around(w, id, ctx);
            if target(w, id).is_some() {
                w.mobys[id].set_state(9, -1);
                play_rand(w, id, ctx, 3, 20);
            } else if 301 <= w.mobys[id].state_ticks {
                w.mobys[id].set_state(4, 1);
                play_rand(w, id, ctx, 0, 20);
            }
        }
        // To a 0x0ef9 (command 4): its radius 0.11 while first in line, else 0.15 (every 4th frame); the 0x0ef9 no
        // longer taking robots, beside Clank; there, in (13).
        12 => {
            let goal = w.mobys[id].pv_v(GOAL);
            move_to(w, id, ctx, goal, false);
            if every(3) {
                let r = if first_in_line(w, id, goal) { f32::from_bits(0x3de1_47ae) } else { f32::from_bits(0x3e19_999a) };
                w.mobys[id].set_pv_f(RADIUS, r);
            }
            let open = target(w, id).is_some_and(|t| slot_open(&w.mobys[t]));
            let m = &w.mobys[id];
            if !open {
                w.mobys[id].set_state(4, 1);
                play_rand(w, id, ctx, 0, 10);
            } else if m.pv_f(GOAL_DIST) < m.pv_f(STOP) {
                w.mobys[id].set_state(13, -1);
            }
        }
        // Into the 0x0ef9: its first tick it goes in (`0x41aed8`), the turn before walking 10°, its velocity across
        // the 0x0ef9's facing kept, no radius; at its place (within 0.1 across) it is gone.
        13 => {
            let Some(t) = target(w, id) else { return robot_tail(w, id, ctx) };
            if w.mobys[id].first() {
                slot_enter(w, t, id);
                let rows = w.mobys[t].rows;
                let m = &mut w.mobys[id];
                m.set_pv_f(MOVE + 0x30, f32::from_bits(0x3e32_b8c3));
                let mut v = crate::gravity::to_local(m.pv_v(VEL), &rows);
                v[1] = 0.0;
                m.set_pv_v(VEL, through(&rows, v));
                m.set_pv_f(RADIUS, 0.0);
            }
            let goal = w.mobys[t].pos;
            move_to(w, id, ctx, goal, false);
            if crate::hero::physics::dist2(w.mobys[id].pos, goal) < 0.1 {
                w.delete(id);
                return;
            }
        }
        // To the 0x097a's end (command 6): help 0x27 off; making way for a robot it pushes on; arrived (flag 0x80),
        // the climb on (15).
        14 => {
            let goal = w.mobys[id].pv_v(GOAL);
            move_to(w, id, ctx, goal, false);
            if w.mobys[id].first() { help_off(ctx, 0x27); }
            if link(w, id, PUSHER).is_some() {
                let h = add_rot(w.mobys[id].rot[2], std::f32::consts::PI);
                step_aside(w, id, h);
            }
            if w.mobys[id].pv_i(MOVE_FLAGS) & 0x80 != 0 { w.mobys[id].set_state(15, -1); }
        }
        // The climb on, in sequence 4 with no collision: its goal the floor's middle 3.8 across on its side; facing
        // the floor, past frame 11 it eases toward it (0.133 a tick); at the end the floor's record opened, it stands
        // as the floor (16, sequence 5) and hooks it (crate::moby::classes::endako_b::update_097a: solid).
        15 => {
            let Some(t) = target(w, id) else { return robot_tail(w, id, ctx) };
            if w.mobys[id].first() {
                super::props::set_seq(w, id, ctx, 4, 0, 10);
                let (tp, rows) = (w.mobys[t].pos, w.mobys[t].rows);
                let m = &mut w.mobys[id];
                m.has_collision = false;
                let mut v = crate::gravity::to_local(crate::hero::physics::sub(m.pv_v(GOAL), tp), &rows);
                v[1] = 0.0;
                v[0] = if v[0] < v[1] { -3.8 } else { 3.8 };
                m.set_pv_v(GOAL, add(through(&rows, v), tp));
            }
            let h = yaw_to(w, id, t);
            turn_to(&mut w.mobys[id], h);
            if 11.0 < ctx.host.frame_time(&w.mobys[id]) {
                let m = &mut w.mobys[id];
                m.pos = step_toward(m.pos, m.pv_v(GOAL), f32::from_bits(0x3e08_8889));
            }
            if super::props::wrapped(w, id, ctx) {
                let tp = w.mobys[t].pos;
                if let Some(n) = net_of(w, id) { net_mark(w, n, ctx, tp, true); }
                w.mobys[id].set_state(16, -1);
                super::props::set_seq(w, id, ctx, 5, 0, 5);
                w.mobys[t].once |= 4;
            }
        }
        // The floor: Clank more than 12 across from it, off it and out of its cuboid, and no robot on it: off (17).
        16 => {
            if w.mobys[id].first() { w.mobys[id].set_pv_i(FLOOR_MOBY, 0); }
            if let Some(t) = target(w, id) {
                let tp = w.mobys[t].pos;
                if 12.0 < crate::hero::physics::dist2(hero, tp) && !held_up(w, id, ctx) && !floor_holds(ctx, &w.mobys[t], hero) {
                    step_off(w, id, ctx);
                }
            }
        }
        // Stepping off toward its goal (at most 0.4167 a tick); at the sequence's end there, the bow (6).
        17 => {
            let m = &mut w.mobys[id];
            let goal = m.pv_v(GOAL);
            let d = crate::hero::physics::sub(goal, m.pos);
            let l = crate::hero::physics::len(d);
            let most = f32::from_bits(0x3ed5_5556);
            let d = if most < l { crate::hero::physics::scale(d, most / l) } else { d };
            m.pos = add(m.pos, d);
            if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == a.seq_b) {
                let m = &mut w.mobys[id];
                m.pos = goal;
                m.set_state(6, 1);
            }
        }
        // To beside the piece (command 5); arrived, the lift (19) holding it (hooked: crate::moby::classes::
        // endako_b::update_0e87).
        18 => {
            let goal = w.mobys[id].pv_v(GOAL);
            move_to(w, id, ctx, goal, false);
            if link(w, id, PUSHER).is_some() {
                let h = add_rot(w.mobys[id].rot[2], std::f32::consts::PI);
                step_aside(w, id, h);
            }
            if w.mobys[id].pv_i(MOVE_FLAGS) & 0x80 != 0 {
                w.mobys[id].set_state(19, -1);
                if let Some(t) = target(w, id) { w.mobys[t].once |= 4; }
            }
        }
        // The lift, in sequence 4 facing the piece: from frame 10 the piece held at its joint 3's height over it
        // (`0x3cb828`); at the end let go (it rises) and the bow (6). Help 0x80 off.
        19 => {
            if w.mobys[id].first() { super::props::set_seq(w, id, ctx, 4, 0, 10); }
            if let Some(t) = target(w, id) {
                let h = yaw_to(w, id, t);
                turn_to(&mut w.mobys[id], h);
                if ctx.host.anim(&w.mobys[id]).is_some_and(|a| a.seq_a == a.seq_b) {
                    if 10.0 <= ctx.host.frame_time(&w.mobys[id]) {
                        if let Some(j) = ctx.host.joint(&w.mobys[id], 3) {
                            let z = j[2] - w.mobys[id].pos[2];
                            w.mobys[t].set_pv_f(0xa0, z);
                        }
                    }
                    if super::props::wrapped(w, id, ctx) {
                        w.mobys[id].set_state(6, 1);
                        w.mobys[t].once &= !4;
                    }
                }
            }
            help_off(ctx, 0x80);
        }
        // To before the arc or plate (command 7); arrived, the zap (21).
        20 => {
            let goal = w.mobys[id].pv_v(GOAL);
            move_to(w, id, ctx, goal, false);
            if link(w, id, PUSHER).is_some() {
                let h = add_rot(w.mobys[id].rot[2], std::f32::consts::PI);
                step_aside(w, id, h);
            }
            if w.mobys[id].pv_i(MOVE_FLAGS) & 0x80 != 0 { w.mobys[id].set_state(21, -1); }
        }
        // The zap, in sequence 5 facing it (help 0x61 off): at frame 20 the arc or plate hooked (crate::moby::
        // classes::level14::arcs bursts, crate::moby::classes::level14::plate launches); at the end the bow (6).
        21 => {
            if w.mobys[id].first() {
                super::props::set_seq(w, id, ctx, 5, 0, 10);
                help_off(ctx, 0x61);
            }
            if let Some(t) = target(w, id) {
                let h = yaw_to(w, id, t);
                turn_to(&mut w.mobys[id], h);
                if super::props::wrapped(w, id, ctx) {
                    w.mobys[id].set_state(6, 1);
                } else if crate::moby::ai::reached(w, id, ctx, 20.0) {
                    w.mobys[t].once |= 4;
                }
            }
        }
        // Launched (L14 `0x3cabb0`, [`launch`]): along its velocity under its two gravities (`+0x394` up, `+0x398`
        // down), tumbling back 24° a tick about its pitch (eased upright once past it on the way down, `0x320220`:
        // 0.4°/tick², at most 24°) about its point 0.8 up (0.3 for 0x0964); below its landing height `+0x3a0` and on
        // the ground (`0x31e228`, 0.5 up): there, beside Clank, in the network that holds it.
        0x16 => {
            let land = {
                let m = &mut w.mobys[id];
                m.pos = add(m.pos, m.pv_v(VEL));
                let h = if m.o_class == ROBOT_964 { 0.3 } else { 0.8 };
                let point = through(&m.rows, [0.0, 0.0, h]);
                let mut v = m.pv_v(VEL);
                let spin = f32::from_bits(0x3ed6_7752);
                let mut land = false;
                if v[2] < 0.0 {
                    v[2] -= m.pv_f(LAUNCH_DOWN);
                    if 0.0 < m.rot[1] {
                        let (mut a, mut s) = (m.rot[1], m.pv_f(LAUNCH_SPIN));
                        crate::hero::physics::ease(0.0, f32::from_bits(0x3be4_c38a), f32::from_bits(0x3be4_c38a), spin, &mut a, &mut s);
                        m.rot[1] = a;
                        m.set_pv_f(LAUNCH_SPIN, s);
                    } else {
                        m.rot[1] = crate::hero::physics::sub_rot(m.rot[1], spin);
                    }
                    land = m.pos[2] < m.pv_f(LAUNCH_LAND);
                } else {
                    v[2] -= m.pv_f(LAUNCH_UP);
                    m.rot[1] = crate::hero::physics::sub_rot(m.rot[1], spin);
                }
                m.set_pv_v(VEL, v);
                m.rows = crate::hero::physics::euler_rows(m.rot);
                let other = through(&m.rows, [0.0, 0.0, h]);
                m.pos = add(m.pos, crate::hero::physics::sub(point, other));
                land
            };
            if land {
                let pos = w.mobys[id].pos;
                let floor = crate::moby::ai::ground_z(ctx, 0.5, pos, 0);
                if pos[2] <= floor {
                    w.mobys[id].pos[2] = floor;
                    w.mobys[id].set_state(4, 1);
                    let pos = w.mobys[id].pos;
                    for n in 0..w.mobys.len() {
                        if w.mobys[n].o_class == NETWORK && reachable(w, Some(n), ctx, pos, pos) {
                            let m = &mut w.mobys[id];
                            m.set_pv_i(NET, n as i32);
                            m.set_pv_i(AT, -1);
                            m.set_pv_i(TO, -1);
                        }
                    }
                }
            }
        }
        _ => {}
    }
    robot_tail(w, id, ctx);
}

/// The end of `0x3c31f8`: in the air or on a moby (not asleep, launched, climbing on or the floor), upright and held
/// (`0x32e698`); then `0x3c5a70`: its state's ticks, the command's 30 ticks (`+0x364`), the glow, with Clank's big
/// head (cheat byte 3) the first aim record's scale 1.75 while awake, both aim records (`0x325588`: crate::moby::
/// joint_aim, 0.03 / 0.3, joint lists 1 and 0) and its fence path the record it is in (`0x41b218`).
fn robot_tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if !matches!(m.state, 0x16 | 1 | 0xf | 0x10) && (m.pv_f(FLOOR_Z) < m.pos[2] - 0.05 || m.pv_i(FLOOR_MOBY) != 0) {
        m.rot[0] = 0.0;
        m.rot[1] = 0.0;
        crate::moby::ai::hold(w, id, ctx);
    }
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks += 1; }
    let mut c = m.pv_i(COOLDOWN);
    countdown(&mut c);
    m.set_pv_i(COOLDOWN, c);
    glow(m);
    if ctx.game.cheat(crate::cheats::cheat::CLANK_HEAD) && m.state != 1 { m.set_pv_f(AIM_A + 0x70, 1.75); }
    joint_aim::aim(w, id, AIM_A, id, 1, f32::from_bits(0x3cf5_c28f), f32::from_bits(0x3e99_999a));
    joint_aim::aim(w, id, AIM_B, id, 0, f32::from_bits(0x3cf5_c28f), f32::from_bits(0x3e99_999a));
    let m = &w.mobys[id];
    if m.sub_state != 7 && m.state != 1 {
        let at = m.pv_i(AT);
        let nodes = net_nodes(w, net_of(w, id));
        let path = usize::try_from(at).ok().and_then(|k| nodes.get(k)).map_or(-1, |n| n.path);
        w.mobys[id].set_pv_i(FENCE, path);
    }
}

/// L14 `0x3cabb0(r, to, peak, up, down)`: the robot launched (0x16, sub-state 7) to land at `to` over a top at the
/// height `peak` — its velocity across over the flight's time (up at the gravity `up`, down at `down`), up at the
/// speed that reaches the top; no fence path; its landing height 0.333 over `to`. Its user: the launch plate 0x0f4d
/// (crate::moby::classes::level14::plate).
pub(crate) fn launch(w: &mut MobyWorld, r: usize, to: V3, peak: f32, up: f32, down: f32) {
    let m = &mut w.mobys[r];
    m.set_state(0x16, 7);
    let rise = (peak - m.pos[2]) * 2.0;
    let t = (rise / up).sqrt() + ((peak - to[2]) * 2.0 / down).sqrt();
    let mut v = crate::hero::physics::scale(crate::hero::physics::sub(to, m.pos), 1.0 / t);
    v[2] = (rise * up).sqrt();
    m.set_pv_v(VEL, v);
    m.set_pv_f(LAUNCH_UP, up);
    m.set_pv_f(LAUNCH_DOWN, down);
    m.set_pv_i(FENCE, -1);
    m.set_pv_f(LAUNCH_LAND, to[2] + 0.333);
}

/// 0x0d75 (`0x4096c8`): a robot's cage. **0**: its pieces 10..14 drawn over it (`+0x7c` = 0x7c00, `$gp+0x25dc`); 1.
/// **1**: a hit with the flag 0x10000 breaks it: its class sound 0, the pieces 0x3cff thrown (`0x327e78` = L00
/// `0x327498`, crate::moby::shards::break_into: 1.5..3 a second across and up), 2. **2**, its first tick: piece 14
/// alone (`+0x7c` = 0xc000), its collision primitives 0..3 (the bars) off (`+0x9c` = 0xf), hooked — its robot wakes.
pub fn update_cage(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            set_7c(&mut w.mobys[id], 0x7c00);
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x10000 != 0) {
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                shards::break_into(w, id, ctx, (1.5, 3.0), (1.5, 3.0), 0x3cff, [0.0; 3], 0xff);
                w.mobys[id].set_state(2, -1);
            }
        }
        2 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                set_7c(m, 0xc000);
                m.once |= 4;
                m.coll_disable = 0xf;
            }
        }
        _ => {}
    }
}

/// 0x0972 (`0x3cb898`): 0x0964's cage, a glass dome. **0**: a placed one (below `0x1b2a20`, the made mobys' first
/// slot: no robot made it) has its collision primitives off (`+0x9c` = 0xf) and goes to 3, nothing more; one its robot
/// made has its pieces 0, 2, 4, 5 drawn over it (`+0x7c` = `$gp+0x12d0`, 0x35), 1. **1**: drawn last frame, its glow
/// (`+0x78`) pulses `$gp+0x12c8` (`0x80363636`) .. `$gp+0x12cc` (`0x80828282`) on the game's frame mod 150; a hit with
/// the flag 0x10000 breaks it: 5 debris chunks half a unit up (`0x328028`: reach 1/3, speed 3, 0.5..1 of the scale,
/// 0.05 up a tick, `0x504040c0`), its pieces 0..5 thrown (`0x327e78`, `$gp+0x12d8..`: 1.5..3 a second across and up),
/// its class sound 0, `+0x7c` 0, hooked (its robot wakes), its primitives off, no update (`+0x34` |= 2); 2.
/// Not ported: the dome's draw `0x3cbab8` (registered while drawn: a 57-point mesh of 42 quads mirroring the camera
/// within 16 across, through `0x2ec4f0`); the class's model is drawn as for any moby.
pub fn update_0972(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            if m.parent.is_none() {
                m.coll_disable = 0xf;
                m.set_state(3, -1);
            } else {
                set_7c(m, 0x35);
                m.set_state(1, -1);
            }
        }
        1 => {
            if w.mobys[id].drawn != 0 {
                let t = ((ctx.game.frame % 150) as f32 * 0.041_887_87 - std::f32::consts::PI).sin();
                w.mobys[id].glow = crate::hud::lerp_rgba((t + 1.0) * 0.5, 0x8036_3636, 0x8082_8282);
            }
            if w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x10000 != 0) {
                let at = add(w.mobys[id].pos, [0.0, 0.0, 0.5]);
                let reach = f32::from_bits(0x3eaa_7efa);
                shards::throw_chunks(w, ctx, reach, 3.0, (0.5, 1.0), id, at, 5, true, [0.0, 0.0, f32::from_bits(0x3d4c_ccce)], 0x5040_40c0, None);
                shards::break_into(w, id, ctx, (1.5, 3.0), (1.5, 3.0), 0x3f, [0.0; 3], 0xff);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
                let m = &mut w.mobys[id];
                set_7c(m, 0);
                m.once |= 4;
                m.coll_disable = 0xf;
                m.mode |= crate::moby::mode::NO_UPDATE;
                m.set_state(2, -1);
            }
        }
        _ => {}
    }
}

/// The robots' counts by kind (L14 `$gp+0x326c..+0x3274`, L03 `0x1b2750`: each robot's tick `0x3c4828` / L14
/// `0x3c7d50` counts itself in when awake, not in state 1, and within 16 across of Ratchet; the frame's start clears
/// them): here counted at the asking. Their reader: the prison's hints 0x0c30 (crate::moby::classes::level14::hints,
/// the zap robots 0x0a59).
pub fn near_hero(w: &MobyWorld, ctx: &Ctx, class: i16) -> usize {
    w.mobys.iter().filter(|m| m.o_class == class && m.state < 0x80 && m.state != 1 && crate::hero::physics::dist2(m.pos, ctx.hero.pos) < 16.0).count()
}
