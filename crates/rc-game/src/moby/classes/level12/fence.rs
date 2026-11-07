//! Level 12's laser fences **0x0fae** (`0x41c710`, 4 placed; the decomp's `func_L12_0041B2A8`; L12 addresses), the
//! pens of the inspection robots 0x1169 (crate::moby::classes::level12::robot): a sheet of light across a cuboid
//! (`[0]`, the gameplay's table `0x1b2f0c`) with three beams in it, 1.5, 3.5 and 5.5 over its floor. Whatever stands in
//! a beam (the Tractor Beam's boxes 0x0fc4, crate::moby::classes::level12::tractor) cuts it short there; Ratchet in the
//! lit part is thrown back out; its two collision walls (class **0x0fdf**) stand only while Ratchet is in it and while
//! a robot of its own moves (the robot's [`walls_on`] / [`walls_off`]), so the robots stay in and Ratchet walks through.
//!
//! The cuboid's rows: `r₀` across (its thickness), `r₁` along the fence (half its width), `r₂` up (half its height);
//! its angles `+0x70` give the turn `R` (`0x2f2da0` = L00 `0x2ef6b8`, crate::moby::attach::rows_of), a point's place
//! in it `R·(p − centre)` (its transpose `0x2f3048` through `0x2f2a00`).
//!
//! Its pvars: `+0x00` the cuboid, `+0x04` the draw's phase, `+0x08` / `+0x0c` its walls, `+0x10` / `+0x20` the beams'
//! ends, `+0x30` −10 (unread), `+0x34..+0x3c` the beams' reaches along `r₁` from the centre, `+0x40` its width
//! (`2|r₁|`), `+0x44` its height (`2|r₂|`), `+0x48` its hum's handle.
//!
//! * Each tick first its walls go (`0x41d210`, [`walls_off`]: their collision `+0x98` 0).
//! * **0**: gone when its uid is marked (the save's `0x21f204` or its count in `0x19b4a8`); else the hum none, 1, the
//!   phase `randf(0, 1)`, the width and height; the beams' ends `centre ∓ r₁` at 1.5 over the floor (`centre.z − r₂.z`),
//!   each moved in by 0.1 along `r₁` while a line from it to the centre 1.5 up meets anything (`0x2e1e20`, flags 2);
//!   its walls (one when `r₂.z` is under 2): a 0x0fdf each (`0x314d78`), its owner's (`0x32ea50` = L00 `0x32a748`),
//!   the cuboid's angles and their rows, not drawn, mode `| 0x105` (hidden, rows kept, no rebuild), at
//!   `centre − r₁ − r₂` and 3 up for the second, rebuilt (`0x3177c8` = L00 `0x313dc0`).
//! * **1**: the phase on by 1/120 (wrapped at 1); the reaches all half the width; from the lowest beam up, while its
//!   line from end to end meets something (`0x2e1e20`, flags 2), its reach is where (along `r₁`), and one cut by more
//!   than 1.5 counts the uid (`0x19b4a8`: next time it is gone); the next beam 2 up. With no beam cut so the hum (class
//!   sound 1, looped) plays, else it stops. Ratchet's feet in the cuboid (`0x325270` = L00 `0x321858`), on the near
//!   side of the beam of his height (from the lowest beam's height: under 1, under 4, above): its walls up
//!   ([`walls_on`]) and, within 0.581 across (`$gp+0x2cd0`), a hit of 2 (flags 0x10001, `0x320b58` / `0x320d78` = L00
//!   `0x31d158` / `0x31d378`) toward `R·(3, 0, −3)` (`$gp+0x2cc0`), or `R·(−0.5, 0, −1)` from behind it. Else his centre
//!   (`0x189f00`, a unit over his feet) the same without the 0.581. Its draw (`0x2eac38` = L00 `0x2e7718`, the frame's
//!   first list: [`draw`], `0x41d3f0`), then its walls stretched to the beams ([`stretch`], `0x41d340`).
//!
//! Not ported: the Clank and Spiderbot checks (`0x3df858` / `0x3df898` with Clank's moby `$gp+0x18cc`, `0x405bc0` with
//! the Spiderbot's `$gp+0x2330`: in the cuboid, their height snapped to its centre's within 2, `0x3e0d10(1, −1)` /
//! `0x407578(1, −1)`); the port has neither mode. The walls' sphere radius (`+0xc`, twice their class's `+0x3c` after
//! each rebuild: the class's sphere is not in crate::moby::ClassInfo; the rebuild's is kept). The port bounds the set-up's
//! pull of the ends (the game's loops run until the line is clear).

use crate::fx::FxQuad;
use crate::hero::physics::{add, len, set_len, sub, through, V3};
use crate::moby::attach::rows_of;
use crate::moby::{Ctx, MobyHost, MobyWorld};
use crate::gravity::to_local;

pub const CLASS: i16 = 0x0fae;
/// Its collision walls.
pub const WALL: i16 = 0x0fdf;

const CUBOID: usize = 0x00;
const PHASE: usize = 0x04;
/// Its two walls (the port's: the moby + 1, 0 none; the game's their addresses).
const WALLS: usize = 0x08;
const START: usize = 0x10;
const END: usize = 0x20;
const REACH: usize = 0x34;
const WIDTH: usize = 0x40;
const HEIGHT: usize = 0x44;
const SOUND: usize = 0x48;
/// The port's, past the class's pvars: the cuboid's angles and centre for the draw (which has no level data).
const ANGLES: usize = 0x200;
const CENTRE: usize = 0x20c;

/// `$gp+0x2cd0`: how far across behind the sheet his feet are still thrown.
const ACROSS: f32 = f32::from_bits(0x3f14_bc6a);

/// The cuboid's rows 1, 2, its centre and its angles.
fn cuboid(ctx: &Ctx, i: i32) -> Option<(V3, V3, V3, V3)> {
    let c = ctx.level.cuboids.get(usize::try_from(i).ok()?)?;
    let v = |r: [f32; 4]| [r[0], r[1], r[2]];
    Some((v(c.matrix[1]), v(c.matrix[2]), v(c.matrix[3]), v(c.extra)))
}

/// Gone with its uid marked (`0x21f204[uid]` ≠ 0, or its count in `0x19b4a8`).
fn uid_done(w: &MobyWorld, id: usize, ctx: &Ctx) -> bool {
    let uid = w.mobys[id].uid;
    ctx.game.uid_dead.get(uid.max(0) as usize).is_some_and(|&d| d != 0) || ctx.game.uid_count(uid) != 0
}

/// A fence's walls (none for another class).
fn walls(w: &MobyWorld, fence: Option<usize>) -> Vec<usize> {
    let Some(m) = fence.and_then(|f| w.mobys.get(f)).filter(|m| m.o_class == CLASS) else { return Vec::new() };
    (0..2).filter_map(|i| usize::try_from(m.pv_i(WALLS + 4 * i) - 1).ok()).filter(|&k| k < w.mobys.len()).collect()
}

/// `0x41d210(fence)`: its walls' collision off.
pub fn walls_off(w: &mut MobyWorld, fence: Option<usize>) {
    for k in walls(w, fence) { w.mobys[k].has_collision = false; }
}

/// `0x41d260(fence)`: its walls stretched ([`stretch`]), then their collision their class's (`+0x24 → +0x10`).
pub fn walls_on(w: &mut MobyWorld, fence: Option<usize>, host: &dyn MobyHost) {
    let Some(f) = fence.filter(|&f| w.mobys.get(f).is_some_and(|m| m.o_class == CLASS)) else { return };
    stretch(w, f, host);
    for k in walls(w, fence) {
        let solid = w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision);
        w.mobys[k].has_collision = solid;
    }
}

/// `0x41d2c8(fence)`: its walls stretched, and whether one has its collision.
pub fn walls_lit(w: &mut MobyWorld, fence: Option<usize>, host: &dyn MobyHost) -> bool {
    let Some(f) = fence.filter(|&f| w.mobys.get(f).is_some_and(|m| m.o_class == CLASS)) else { return false };
    stretch(w, f, host);
    walls(w, fence).into_iter().any(|k| w.mobys[k].has_collision)
}

/// `0x41d340`: each wall's second row `(reach + width / 2) / 6` long (the model is 6 long), rebuilt.
fn stretch(w: &mut MobyWorld, id: usize, host: &dyn MobyHost) {
    let m = &w.mobys[id];
    let half = m.pv_f(WIDTH) * 0.5;
    let reaches = [m.pv_f(REACH), m.pv_f(REACH + 4)];
    let ids = [m.pv_i(WALLS) - 1, m.pv_i(WALLS + 4) - 1];
    for (i, k) in ids.into_iter().enumerate() {
        let Some(k) = usize::try_from(k).ok().filter(|&k| k < w.mobys.len()) else { continue };
        let r = w.mobys[k].rows[1];
        w.mobys[k].rows[1] = set_len(r, (reaches[i] + half) * f32::from_bits(0x3e2a_aaab));
        w.rebuild(k, host);
    }
}

/// 0x0fae (module docs).
pub fn update_0fae(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    walls_off(w, Some(id));
    match w.mobys[id].state {
        0 => set_up(w, id, ctx),
        1 => tick(w, id, ctx),
        _ => {}
    }
}

/// State 0 (module docs).
fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if uid_done(w, id, ctx) { return w.delete(id); }
    let phase = ctx.game.rng.randf(0.0, 1.0);
    let m = &mut w.mobys[id];
    m.set_pv_i(SOUND, -1);
    m.state = 1;
    m.set_pv_f(PHASE, phase);
    m.set_pv_f(0x30, -10.0);
    let Some((r1, r2, o, angles)) = cuboid(ctx, m.pv_i(CUBOID)) else { return };
    let (l1, l2) = (len(r1), len(r2));
    m.set_pv_f(WIDTH, l1 + l1);
    m.set_pv_f(HEIGHT, l2 + l2);
    m.set_pv_v(ANGLES, angles);
    m.set_pv_v(CENTRE, o);
    let lift = 1.5 - r2[2];
    let (mut a, mut b) = (sub(o, r1), add(o, r1));
    a[2] += lift;
    b[2] += lift;
    let mut mid = o;
    mid[2] += 1.5;
    let step = set_len(r1, 0.1);
    // The port's bound: the steps across the whole fence.
    let most = (l1 * 20.0) as usize + 2;
    for _ in 0..most {
        if w.coll_line(ctx, a, mid, 2, None).is_none() { break; }
        a = add(a, step);
    }
    for _ in 0..most {
        if w.coll_line(ctx, b, mid, 2, None).is_none() { break; }
        b = sub(b, step);
    }
    let m = &mut w.mobys[id];
    m.set_pv_v(START, a);
    m.set_pv_v(END, b);
    let rows = rows_of(angles);
    for i in 0..2 {
        if r2[2] < 2.0 && 0 < i { continue; }
        let k = w.create(WALL);
        w.adopt(k, Some(id), &*ctx.host);
        let c = &mut w.mobys[k];
        c.rot = angles;
        c.rows = rows;
        c.drawn = 0;
        c.mode |= 0x105;
        let mut p = add(sub(sub([0.0; 3], r1), r2), o);
        p[2] += i as f32 * 3.0;
        c.pos = p;
        w.rebuild(k, &*ctx.host);
        w.mobys[id].set_pv_i(WALLS + 4 * i, k as i32 + 1);
    }
}

/// State 1 (module docs).
fn tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    let mut ph = m.pv_f(PHASE) + f32::from_bits(0x3c08_8889);
    if 1.0 < ph { ph -= 1.0; }
    m.set_pv_f(PHASE, ph);
    let half = m.pv_f(WIDTH) * 0.5;
    for j in 0..3 { m.set_pv_f(REACH + 4 * j, half); }
    let ci = m.pv_i(CUBOID);
    let Some((_, _, o, angles)) = cuboid(ctx, ci) else { return };
    let r = rows_of(angles);
    let (mut a, mut b) = (m.pv_v(START), m.pv_v(END));
    let uid = m.uid;
    let mut clear = true;
    for j in 0..3 {
        let Some(hit) = w.coll_line(ctx, a, b, 2, None) else { break };
        let d = to_local(sub(hit.point, o), &r);
        let m = &mut w.mobys[id];
        if 1.5 < m.pv_f(REACH + 4 * j) - d[1] {
            ctx.game.count_uid(uid);
            clear = false;
        }
        m.set_pv_f(REACH + 4 * j, d[1]);
        a[2] += 2.0;
        b[2] += 2.0;
    }
    let h = w.mobys[id].pv_i(SOUND);
    let h = if clear {
        if h == -1 || !ctx.host.sound_playing(h) { ctx.host.play_sound(&w.mobys[id], 1, 4) } else { h }
    } else {
        if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
        -1
    };
    w.mobys[id].set_pv_i(SOUND, h);
    let m = &w.mobys[id];
    let floor = m.pv_f(START + 8);
    let reaches = [m.pv_f(REACH), m.pv_f(REACH + 4), m.pv_f(REACH + 8)];
    let reach = |z: f32| if floor + 4.0 <= z { reaches[2] } else if floor + 1.0 <= z { reaches[1] } else { reaches[0] };
    let feet = ctx.hero.pos;
    let centre = add(feet, [0.0, 0.0, 1.0]);
    if ctx.level.in_cuboid(ci, feet) {
        let d = to_local(sub(feet, o), &r);
        if d[1] < reach(feet[2]) {
            walls_on(w, Some(id), &*ctx.host);
            if d[0] <= ACROSS { throw(w, id, ctx, &r, d[0]); }
        }
    } else if ctx.level.in_cuboid(ci, centre) {
        let d = to_local(sub(centre, o), &r);
        if d[1] < reach(centre[2]) {
            walls_on(w, Some(id), &*ctx.host);
            throw(w, id, ctx, &r, d[0]);
        }
    }
    w.register_draw_first(id);
    stretch(w, id, &*ctx.host);
}

/// Ratchet thrown back: a hit of 2 (flags 0x10001) toward `R·(3, 0, −3)`, from behind the sheet `R·(−0.5, 0, −1)`.
fn throw(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, r: &[V3; 3], across: f32) {
    let v = if 0.0 <= across { [3.0, 0.0, -3.0] } else { [-0.5, 0.0, -1.0] };
    let dir = through(r, v);
    let Some(h) = ctx.hero_moby else { return };
    let t = crate::moby::hits::HitTemplate::new(2.0, Some(id), 0x1_0001, [dir[0], dir[1], dir[2], 0.0]);
    w.deliver_hit_any(h, &t, [0.0, 0.0, 0.0, 1.0]);
}

/// `$gp+0x2cd4`: the FX texture; ALPHA `$gp+0x2cd8` / `+0x2cdc` (1, 1): 0x44, blended.
const FX: usize = 6;
/// `$gp+0x2ce0` / `0x1b1cd4`: the corners' ST.
const ST: [[f32; 2]; 4] = [[0.5, 0.0], [0.5, 1.0], [0.5, 0.0], [0.5, 1.0]];
/// `$gp+0x2d0c`: the sheet's colour; `$gp+0x2d10` / `+0x2d14` the bars'.
const SHEET: u32 = 0x4050_8030;
const BAR: [u32; 2] = [0x7000_80ff, 0x0000_8000];
/// `$gp+0x2d00`: the corners' depth to start; `$gp+0x2d40` the bars' half thickness; `$gp+0x2d44` the upper bars'
/// foot.
const DEPTH: f32 = 0.25;
const THICK: f32 = 0.0;
const FOOT: f32 = 3.5;

/// `0x41d3f0` (the frame's first list): in the cuboid's turn from 0.5 under its floor (`0x2f2dc0`, the quads through it:
/// `0x2eee18` = L00 `0x2eb998` with the matrix), FX 6, ALPHA 0x44:
/// * the sheet: from the floor up in steps of 0.5 (while under the height − 0.1) a quad from the near end (`−width/2`
///   along `r₁`) to the lowest beam's reach (to 2.5 up), the middle one's (to 4.5), else the width, in 0x40508030;
/// * the bars: from the lowest beam's reach back toward the near end in steps of 0.5, a bar 0.5 wide from half the
///   height under the floor to the height, then from the middle beam's reach back to the lowest's, from 3.5 up: each
///   its faces at ±0 across (the upper ones' front twice) and its two sides, its colour the last one's lerp
///   (`0x2f36e8`, crate::hud::lerp_rgba) by the phase from 0x700080ff to 0x00008000 (the first the sheet's); the phase
///   on by `0.5 / width` a bar, less 1 once over 0.
///
/// Not ported: the corners' depth set to ±0.07 (`$gp+0x2d04`) between the two (every bar sets them again).
pub fn draw(w: &MobyWorld, id: usize, out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let (height, width) = (m.pv_f(HEIGHT), m.pv_f(WIDTH));
    let r = rows_of(m.pv_v(ANGLES));
    let mut o = m.pv_v(CENTRE);
    o[2] -= height * 0.5 + 0.5;
    let quad = |out: &mut Vec<FxQuad>, p: &[V3; 4], rgba: [u32; 4]| {
        let corners = p.map(|v| add(through(&r, v), o));
        out.push(FxQuad { fx: FX, additive: false, corners, st: ST, rgba });
    };
    let mut p: [V3; 4] = std::array::from_fn(|n| {
        let mut v = [0.0, width * 0.5, DEPTH];
        if n < 2 { v[1] = -v[1]; }
        if n & 1 != 0 { v[2] = -v[2]; }
        v[2] += 0.25;
        v
    });
    let mut rgba = [SHEET; 4];
    let (r0, r1) = (m.pv_f(REACH), m.pv_f(REACH + 4));
    let mut h = 0.0f32;
    while h <= height - f32::from_bits(0x3dcc_cccd) {
        for (j, v) in p.iter_mut().enumerate() {
            if 2 <= j { v[1] = if h <= 2.5 { r0 } else if h <= 4.5 { r1 } else { width }; }
            v[2] += 0.5;
        }
        quad(out, &p, rgba);
        h += 0.5;
    }
    let half_w = width * 0.5;
    let half_h = height * 0.5;
    let step = 0.5 / width;
    let mut phase = m.pv_f(PHASE);
    // One bar: its front (twice for the upper ones), its back, its two sides.
    let bar = |out: &mut Vec<FxQuad>, p: &mut [V3; 4], rgba: [u32; 4], fronts: usize| {
        for v in p.iter_mut() { v[0] = THICK; }
        for _ in 0..fronts { quad(out, &*p, rgba); }
        let back = p.map(|v| [-THICK, v[1], v[2]]);
        quad(out, &back, rgba);
        quad(out, &[p[0], back[0], p[2], back[2]], rgba);
        quad(out, &[p[1], back[1], p[3], back[3]], rgba);
    };
    let next_colour = |phase: &mut f32, rgba: &mut [u32; 4]| {
        let next = *phase + step;
        *rgba = [crate::hud::lerp_rgba(*phase, BAR[0], BAR[1]); 4];
        *phase = if 0.0 < next { next - 1.0 } else { next };
    };
    let mut y = r0;
    while -half_w < y {
        for (j, v) in p.iter_mut().enumerate() { v[1] = if j & 1 != 0 { y + 0.5 } else { y }; }
        (p[0][2], p[1][2]) = (-half_h, -half_h);
        (p[2][2], p[3][2]) = (height, height);
        bar(out, &mut p, rgba, 1);
        next_colour(&mut phase, &mut rgba);
        y -= 0.5;
    }
    let mut y = r1;
    while r0 < y {
        for (j, v) in p.iter_mut().enumerate() { v[1] = if j & 1 != 0 { y + 0.5 } else { y }; }
        (p[0][2], p[1][2]) = (FOOT, FOOT);
        (p[2][2], p[3][2]) = (height, height);
        bar(out, &mut p, rgba, 2);
        next_colour(&mut phase, &mut rgba);
        y -= 0.5;
    }
}
