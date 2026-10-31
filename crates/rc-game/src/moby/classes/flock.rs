//! The flocks (L02 `0x38ce78`, on levels 2, 3, 6, 7, 9, 13, 16, 18 and 20): a moby group of flying creatures that
//! weave along a looped path, bob, bank into their turns, glow, trail streaks from their wings and fall to a shot.
//! Classes 0x09b0, 0x09b7, 0x0b13, 0x0b15, 0x0d2c, 0x0d5c, 0x127d, 0x128b, 0x1290, 0x1340, 0x134b; one of the group
//! leads (its update moves them all; the others are frozen, mode |= 2).
//!
//! **The leader** (state 0, [`set_up`] `0x38d150`) waits for its group (`+0x21`), then spaces it out along its path
//! `+0`. In state 1 it moves them ([`tick`] `0x38dde8`) and draws their trails ([`draw`] `0x38ee50`); with the camera
//! in its cuboid `+0x0c` it hides the group (state 2) until the camera leaves.
//!
//! **A member's pvars**: `+0x10` its place on the path (in points), `+0x7c` / `+0x80` / `+0x84` the weave about it
//! (amplitude, phase, phase step), `+0x68` / `+0x6c` the bob (phase, step), `+0x78` its scale over its class's,
//! `+0x30..+0x5c` its glows' points and `+0xa0..` its trails' (both × `+0x78`), `+0x90` / `+0x94` the glows' reaches,
//! `+0x98` the trails', `+0x1c` the trails' phase, `+0x18` its looped sound.
//!
//! A shot member explodes (`0x324930`, crate::moby::blast::explode). Not ported: the kill count `0x1a9048` a shot
//! member adds to; on level 0xd the leader's hiding above z 89 (that level is not ported).

use crate::hero::physics::{add_rot, approach, atan2, dist, len2, sub, sub_rot, V3};
use crate::hud::lerp_rgba;
use crate::moby::{Ctx, MobyWorld};
use crate::particles;

/// The flocks' classes.
pub const CLASSES: [i16; 11] = [0x09b0, 0x09b7, 0x0b13, 0x0b15, 0x0d2c, 0x0d5c, 0x127d, 0x128b, 0x1290, 0x1340, 0x134b];
/// `$gp−0xe0` (`0x1aef10`): the bank's turn, degrees a second.
const BANK_SPEED: f32 = 30.0;
/// π / 180 / 60: degrees a second to radians a tick.
const DEG_TICK: f32 = 0.000_290_888_25;

const fn v(x: u32, y: u32, z: u32) -> V3 { [f32::from_bits(x), f32::from_bits(y), f32::from_bits(z)] }

/// The glows' points (`+0x30`, `+0x40`, `+0x50`) and the trails' (`+0xa0..`) of each class, before the scale
/// (`0x38d150`'s constants and `0x1aee40..0x1aef0c`). 0x127d's and 0x1290's trails are their glows' points (scaled
/// twice, as the game does).
fn points(class: i16) -> (&'static [V3], &'static [V3]) {
    const P9B0: [V3; 3] = [v(0xc006_6666, 0x3f40_0000, 0x3f5c_28f6), v(0xc006_6666, 0xbf40_0000, 0x3f5c_28f6), v(0xc040_0000, 0, 0xbf2b_851f)];
    const T9B0: [V3; 3] = [v(0xbfe1_47ae, 0x3f39_5810, 0x3f5e_b852), v(0xbfe1_47ae, 0xbf39_5810, 0x3f5e_b852), v(0xc02c_cccd, 0, 0xbf22_4dd3)];
    const P9B7: [V3; 2] = [v(0xc06d_f3b6, 0, 0x3f46_e979), v(0xc06d_f3b6, 0, 0xbf66_24dd)];
    const T9B7: [V3; 2] = [v(0xc059_999a, 0, 0x3f4d_4fdf), v(0xc059_999a, 0, 0xbf66_24dd)];
    const PB15: [V3; 2] = [v(0xc01b_3333, 0x4057_50b1, 0x3f6e_147b), v(0xc01b_3333, 0xc040_51ec, 0x3f6e_147b)];
    const P134B: [V3; 2] = [v(0xbe96_0419, 0x3e63_53f8, 0xbe18_9375), v(0xbe96_0419, 0xbe63_53f8, 0xbe18_9375)];
    const PB13: [V3; 2] = [v(0xc06b_645a, 0x3f4e_d917, 0x3dcc_cccd), v(0xc06b_645a, 0xbf4e_d917, 0x3dcc_cccd)];
    const P127D: [V3; 2] = [v(0xbe28_f5c3, 0x3f33_3333, 0xbe24_dd2f), v(0xbe28_f5c3, 0xbf33_3333, 0xbe24_dd2f)];
    const P128B: [V3; 2] = [v(0xbb44_9ba6, 0x3f57_ced9, 0xbe9b_22d1), v(0xbb44_9ba6, 0xbf57_ced9, 0xbe9b_22d1)];
    const P1290: [V3; 3] = [v(0xbf52_b021, 0x3f32_b021, 0xbe60_4189), v(0xbf52_b021, 0xbf32_b021, 0xbe60_4189), v(0xbfb0_20c5, 0, 0x3e54_fdf4)];
    const P1340: [V3; 2] = [v(0xbe5c_28f6, 0x4008_72b0, 0xbea3_53f8), v(0xbe5c_28f6, 0xc008_72b0, 0xbea3_53f8)];
    match class {
        0x09b0 => (&P9B0, &T9B0),
        0x09b7 => (&P9B7, &T9B7),
        0x0b15 => (&PB15, &PB15),
        0x134b => (&P134B, &P134B),
        0x0b13 | 0x0d5c => (&PB13, &[]),
        0x127d => (&P127D, &P127D),
        0x128b => (&P128B, &P128B),
        0x1290 => (&P1290, &P1290),
        0x1340 => (&P1340, &P1340),
        _ => (&[], &[]),
    }
}

/// The glows of `0x38e238`: (near size, near colour, far size, far colour, points).
fn glows(class: i16) -> Option<(f32, u32, f32, u32, usize)> {
    Some(match class {
        0x09b0 => (756_000.0, 0x4080_4040, 777_000.0, 0x2080_4040, 3),
        0x09b7 | 0x0b15 => (756_000.0, 0x4056_96ff, 777_000.0, 0x2056_96ff, 2),
        0x0b13 | 0x0d5c => (756_000.0, 0x4080_4040, 777_000.0, 0x2080_4040, 2),
        0x127d => (294_000.0, 0x4020_2080, 231_000.0, 0x2056_96ff, 2),
        0x1290 => (210_000.0, 0x4080_4020, 210_000.0, 0x2090_5010, 3),
        0x134b => (340_200.0, 0x4080_4040, 194_250.0, 0x2080_4040, 2),
        _ => return None,
    })
}

/// The trails of `0x38ee50`, in its order: (class, width, length, first point, points, c0, c1).
const TRAILS: [(i16, f32, f32, usize, usize, u32, u32); 9] = [
    (0x09b0, 0.8, 5.0, 0, 3, 0x6080_4040, 0x0020_4090),
    (0x09b7, 1.0, 7.0, 0, 2, 0x6056_96ff, 0x0000_00c0),
    (0x0b15, 1.0, 7.0, 0, 2, 0x6056_96ff, 0x0000_00c0),
    (0x134b, 0.3, 2.1, 0, 2, 0x6080_4040, 0x0020_4090),
    (0x127d, 0.4, 4.0, 0, 2, 0x6056_96ff, 0x0000_00c0),
    (0x128b, 0.4, 3.0, 0, 2, 0x6056_96ff, 0x0000_00c0),
    (0x1290, 0.4, 3.0, 0, 2, 0x6080_4020, 0x00c0_0000),
    (0x1290, 0.6, 4.5, 2, 1, 0x6080_4020, 0x00c0_0000),
    (0x1340, 1.0, 7.0, 0, 2, 0x6056_96ff, 0x0000_00c0),
];

fn group(w: &MobyWorld, id: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].group).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// The flocks' update (`0x38ce78`).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].mode & 2 != 0 { return; }
    let cam = ctx.host.camera_pos();
    match w.mobys[id].state {
        0 => set_up(w, id, ctx),
        1 => {
            let c = w.mobys[id].pv_i(0xc);
            if c != -1 && ctx.level.in_cuboid(c, cam) {
                for m in group(w, id) {
                    w.mobys[m].drawn = 0;
                    w.mobys[m].mode |= 1;
                }
                w.mobys[id].set_state(2, -1);
            }
            tick(w, id, ctx);
            w.register_draw(id);
        }
        2 => {
            if !ctx.level.in_cuboid(w.mobys[id].pv_i(0xc), cam) {
                for m in group(w, id) {
                    w.mobys[m].drawn = 1;
                    w.mobys[m].mode &= !1;
                }
                w.mobys[id].set_state(1, -1);
            }
        }
        _ => {}
    }
}

/// `0x38d150(m, group)`: with its group there, update distance 0xff, the step along the path a tick (`+0x14` = speed
/// `+8` / the first segment's length / 60) and whether it closes (`+0x70`: its ends within 5); the group shuffled (but
/// its last), and each member k of n placed `(n−1−k)/n` of the way round, weaving ± `0.4·points/n` at
/// `randf(0.75, 1.25)` of a quarter of the step, its bob `randf_signed(+4·0.5, +4)` high at `randf_signed(60, 120)`°/s,
/// its wing beat in `rand_range(30, 90)` ticks, its points scaled, state 1, draw distance 0xbe (not on level 3).
fn set_up(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(g) = usize::try_from(w.mobys[id].group).ok().filter(|&g| w.groups.get(g).is_some_and(|l| !l.is_empty())) else { return };
    let Some(path) = usize::try_from(w.mobys[id].pv_i(0)).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| 2 <= p.len()) else { return };
    let p3 = |k: usize| [path[k][0], path[k][1], path[k][2]];
    let n_points = path.len() as f32;
    let seg = dist(p3(0), p3(1));
    let closed = dist(p3(0), p3(path.len() - 1)) < 5.0;
    let leader = &mut w.mobys[id];
    leader.update_dist = 0xff;
    leader.set_pv_s(0x70, closed as i16);
    let step = leader.pv_f(8) * (1.0 / seg) * 0.016_666_668;
    leader.set_pv_f(0x14, step);
    let rng = &mut ctx.game.rng;
    let list = &mut w.groups[g];
    for k in 1..list.len().saturating_sub(1) {
        let j = rng.randi(k as i32) as usize;
        list.swap(j, k);
    }
    let list = list.clone();
    let n = list.len();
    let spread = (n_points * 0.4) / n as f32;
    let weave = (step * 0.25) / spread;
    let level = ctx.game.level;
    for (pos, &m) in list.iter().enumerate() {
        let k = (n - 1 - pos) as i32;
        let class_scale = w.class(w.mobys[m].o_class).map_or(1.0, |c| c.scale);
        let rng = &mut ctx.game.rng;
        let x = &mut w.mobys[m];
        if m != id { x.mode |= 2; }
        x.set_pv_f(0x7c, spread);
        x.set_pv_f(0x10, (k as f32 / n as f32) * n_points);
        x.set_pv_f(0x84, weave * rng.randf(0.75, 1.25));
        x.set_pv_f(0x80, rng.rand_angle());
        x.set_pv_s(0x72, 1);
        x.set_pv_v(0x20, [0.0; 3]);
        x.set_pv_f(0x28, x.pv_f(4) * rng.randf_signed(0.5, 1.0));
        x.set_pv_s(0x76, rng.rand_range(30, 90) as i16);
        x.set_pv_f(0x68, rng.rand_angle());
        x.set_pv_f(0x6c, rng.randf_signed(60.0, 120.0) * DEG_TICK);
        x.set_pv_i(0x18, -1);
        x.set_pv_f(0x1c, 0.0);
        x.set_pv_i(0x88, 0);
        let s = if class_scale != 0.0 { x.scale / class_scale } else { 1.0 };
        x.set_pv_f(0x78, s);
        let (glow_pts, trail_pts) = points(x.o_class);
        for (j, p) in glow_pts.iter().enumerate() { x.set_pv_v(0x30 + 0x10 * j, p.map(|c| c * s)); }
        for (j, p) in trail_pts.iter().enumerate() { x.set_pv_v(0xa0 + 0x10 * j, p.map(|c| c * s * if matches!(x.o_class, 0x127d | 0x1290) { s } else { 1.0 })); }
        x.state = 1;
        if level != 3 { x.draw_dist = 0xbe; }
    }
}

/// `0x34ad68(t, path, closed, &pos, &rot)`: the place `t` (in points) along the path: the point between its point
/// `⌊t⌋` and the next, an open path held at its last segment; facing along the way there, eased into the next segment
/// (yaw from its xy, pitch down by its climb).
pub(crate) fn follow(path: &[[f32; 4]], closed: bool, t: f32, pos: &mut V3, rot: &mut V3) {
    let n = path.len() as i32;
    let mut i = t as i32;
    let end = !closed && n - 2 <= i;
    if end { i = n - 2; }
    let mut f = t - i as f32;
    if end && 1.0 < f { f = 1.0; }
    let (mut i0, mut i1, mut i2) = (i, i + 1, i + 2);
    if n <= i2 {
        i0 %= n;
        i1 %= n;
        i2 %= n;
    }
    let p = |k: i32| { let q = path[k.rem_euclid(n) as usize]; [q[0], q[1], q[2]] };
    let (a, b) = (p(i0), p(i1));
    *pos = std::array::from_fn(|k| a[k] + (b[k] - a[k]) * f);
    let d = if end {
        sub(b, a)
    } else {
        let (u, v) = (sub(b, a), sub(p(i2), b));
        std::array::from_fn(|k| u[k] + (v[k] - u[k]) * f)
    };
    rot[1] = -atan2(len2(d), d[2]);
    rot[2] = atan2(d[0], d[1]);
}

/// `0x38dde8`: each member at its place plus the weave (`+0x10 + +0x7c·sin +0x80`, wrapped round the path), bobbing
/// by `sin +0x68`, its place on by the step and its weave on; a shot (flags 0x4010000) brings a drawn one down —
/// hidden, no collision, its pieces thrown (−7..7 across, 3..13 up, sound 2) and 0x1290 / 0x134b's sound 0 or 0x0b15
/// / 0x0b13's 1. A drawn one banks into its turn (−50 × the yaw's change, at most 45°, 30°/s), glows
/// ([`member_glows`]) and 0x09b0 blinks.
fn tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(path) = usize::try_from(w.mobys[id].pv_i(0)).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| 2 <= p.len()).cloned() else { return };
    let closed = w.mobys[id].pv_s(0x70) != 0;
    let n = path.len() as f32;
    let step = w.mobys[id].pv_f(0x14);
    for m in group(w, id) {
        let x = &mut w.mobys[m];
        let yaw = x.rot[2];
        let mut t = x.pv_f(0x10) + x.pv_f(0x7c) * x.pv_f(0x80).sin();
        if n <= t { t -= n } else if t < 0.0 { t += n }
        let (mut pos, mut rot) = (x.pos, x.rot);
        follow(&path, closed, t, &mut pos, &mut rot);
        x.pos = pos;
        x.rot = rot;
        let b = add_rot(x.pv_f(0x68), x.pv_f(0x6c));
        x.set_pv_f(0x68, b);
        x.pos[2] += b.sin();
        let mut p = x.pv_f(0x10) + step;
        if n <= p { p -= n; }
        x.set_pv_f(0x10, p);
        x.set_pv_f(0x80, add_rot(x.pv_f(0x80), x.pv_f(0x84)));
        let shot = w.hit_message(m).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0);
        if shot && w.mobys[m].drawn != 0 {
            let x = &mut w.mobys[m];
            x.drawn = 0;
            x.has_collision = false;
            x.mode |= 1;
            crate::moby::shards::throw_all(w, m, ctx, [-7.0, -7.0, 3.0, 0.0, 7.0, 7.0, 13.0, 0.0], (90.0, 360.0), (10.0, 20.0), 0x3f, 2, -1, 0, 0xffff);
            match w.mobys[m].o_class {
                0x1290 | 0x134b => { ctx.host.play_sound(&w.mobys[m], 0, 0); }
                0x0b15 | 0x0b13 => { ctx.host.play_sound(&w.mobys[m], 1, 0); }
                _ => {}
            }
            // `0x324930`: the explosion (crate::moby::blast::explode).
            let e = crate::moby::blast::Explosion { flash: 5.0, flash2: 2.5, size: 3.0, light: 15.0, embers: 0x23, billows: 8, flares: 0x10, sound: -1, debris: 10, lod: -1, flags: 0x81_0001, ..Default::default() };
            crate::moby::blast::explode(w, ctx, Some(m), None, &e);
        }
        w.clear_hit(m);
        w.mobys[m].hit_slot = -1;
        let x = &w.mobys[m];
        if x.drawn != 0 && x.mode & 1 == 0 {
            let bank = (-sub_rot(x.rot[2], yaw) * 50.0).clamp(-std::f32::consts::FRAC_PI_4, std::f32::consts::FRAC_PI_4);
            let x = &mut w.mobys[m];
            approach(bank, BANK_SPEED * DEG_TICK, &mut x.rot[0]);
            if x.pv_s(0x72) != 0 { member_glows(w, m, ctx); }
            let x = &mut w.mobys[m];
            if x.o_class == 0x09b0 { blink(x, 0x32, if x.pv_i(0x8c) == 0 { 0x5014_1414 } else { 0x5078_7878 }); }
        }
        w.rebuild(m, ctx.host);
    }
    w.changed = true;
}

/// The blink of 0x09b0 (`0x38edc8`) and 0x0d2c (in `0x38e238`): every `period` ticks (`+0x74`) the light `+0x8c`
/// flips; the colour `+0x78` eases 0.15 of the way to `to`.
fn blink(x: &mut crate::moby::Moby, period: i16, to: u32) {
    let mut t = x.pv_s(0x74) as i32;
    if crate::moby::countdown(&mut t) {
        t = period as i32;
        x.set_pv_i(0x8c, (x.pv_i(0x8c) == 0) as i32);
    }
    x.set_pv_s(0x74, t as i16);
    x.glow = lerp_rgba(0.15, x.glow, to);
}

/// `0x38e238`: 0x09b0 / 0x09b7 / 0x0d2c / 0x0d5c keep their looped sound 0 (`+0x18`); 0x0d2c blinks toward
/// 0x50000046 every 35 ticks; within `+0x90` of the camera its near glows (frame 0, pulled 0.5) on its points, within
/// `+0x94` its far ones (frame 1, pulled 0.6), each for 2 ticks.
fn member_glows(w: &mut MobyWorld, m: usize, ctx: &mut Ctx) {
    let class = w.mobys[m].o_class;
    if matches!(class, 0x09b0 | 0x09b7 | 0x0d2c | 0x0d5c) && ctx.game.level != 0xd {
        let h = w.mobys[m].pv_i(0x18);
        if !ctx.host.sound_playing(h) {
            let h = ctx.host.play_sound(&w.mobys[m], 0, 4);
            w.mobys[m].set_pv_i(0x18, h);
        }
    }
    if class == 0x0d2c { blink(&mut w.mobys[m], 0x23, 0x5000_0046); }
    let Some((near_size, near, far_size, far, n)) = glows(class) else { return };
    let x = &w.mobys[m];
    let d = dist(ctx.host.camera_pos(), x.pos);
    let (s, pos, reach) = (x.pv_f(0x78), x.pos, [x.pv_i(0x90), x.pv_i(0x94)]);
    let offsets: Vec<V3> = (0..n).map(|j| x.pv_v(0x30 + 0x10 * j)).collect();
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    if d < reach[0] as f32 {
        for &o in &offsets { particles::glow_def(sys, rng, s * near_size, 0.5, o, m, pos, reach[0] as i16, false, near, 2, 0); }
    }
    if d < reach[1] as f32 {
        for &o in &offsets { particles::glow_def(sys, rng, s * far_size, 0.6, o, m, pos, reach[1] as i16, false, far, 2, 1); }
    }
}

/// `0x38ee50` (the leader's draw callback): class by class (the game's eight draws), each member of its group that is
/// drawn, shown and in state 1, in view (its sphere 1.1 wide within `+0x98`), runs its trails' phase down 0.1
/// (wrapping at −8; the game skips the step on some classes when `0x2f2440` says so, read here as always) and draws its trails (crate::fx::strip, effect texture 0x17) at its points `+0xa0..`.
pub fn draw(w: &mut MobyWorld, id: usize, view: &crate::particles::BSphereView, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let members = group(w, id);
    for (class, a, b, first, n, c0, c1) in TRAILS {
        for &m in &members {
            let x = &mut w.mobys[m];
            if x.o_class != class || x.drawn == 0 || x.mode & 1 != 0 || x.state != 1 { continue; }
            let s = x.bsphere.map(|c| c / 1024.0);
            if view.culled(x.pv_i(0x98) as f32, [s[0], s[1], s[2], s[3] * 1.1]) { continue; }
            // The 0x1290's second trail reads the phase its first just stepped.
            if first == 0 {
                let mut ph = x.pv_f(0x1c) - 0.1;
                if ph <= -8.0 { ph += 8.0; }
                x.set_pv_f(0x1c, ph);
            }
            let k = x.pv_f(0x78);
            let pts: Vec<V3> = (first..first + n).map(|j| x.pv_v(0xa0 + 0x10 * j)).collect();
            crate::fx::strip(out, a * k, b * k, x.pv_f(0x1c), &x.rows, x.pos, cam, &pts, c0, c1, 0x17);
        }
    }
}
