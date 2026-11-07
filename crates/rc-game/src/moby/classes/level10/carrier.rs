//! Hrugis's carrier 0x0f85 (L10 `0x417c88`, 6 placed; the decomp's `func_L10_004166E8`): a big ship turning in
//! place, its two gun parts firing at the player, which a hit to 0 does not kill: its parts blow off and a station
//! 0x0f8d (crate::moby::classes::level10::ships) flies to it along a spline and mends it — unless shot down on the
//! way. Its first mission (crate::moby::classes::space::mission) is won with every carrier down at once.
//!
//! Each tick first its push and hits (`0x37e600(m, 1, 1, 1)`: no push, left where it is, the death's return) and the
//! range test (`0x37ea10(m, 1, 32)`).
//! * **State 0** (once the library's ready flag is set, taken as set): `0x4185e0` — health 30 (the AI block's
//!   `+0x148`, `+0x180`), active (`0x37df20(m, 1)`, [`space::start`]), state 1, drawn and updated from anywhere, the
//!   sway's phase random (`+0x2bc`) about its height (`+0x2b8`), no parts nor station, its class's scale × 6.4, the
//!   guns' wait 540 (`+0x2e4`), the turn and sway still (`+0x2cc` / `+0x2d0`), the glow word `+0x7c` 0xff. With its
//!   leader word (`+0x2b4`) it leads with the callbacks `0x4189e0` ([`on_activate`]: its group back in state 1) and
//!   `0x418a68` ([`on_put_away`]: the parts gone).
//! * **State 1**: its two parts made when missing (`0x419280`, [`make_part`]); the station's flight time
//!   (`0x1b0ee0`: 30 + 2.5 per active ship of the library, at most 60 s); its own target (`+0x2b0`, the player's
//!   ship); the turn (`0x418b68`, [`spin_up`]: the yaw step up 0.0000388 a tick to 0.0233, the sway's to 0.0116) and
//!   the sway (4 up and down); on screen (`0x283db8`, 64) and 24 or more from the target the guns (`0x418c60`,
//!   [`guns`]); its smoke below 25 health (`0x418d88`, 2..3) and 15 (`0x418fd8`, 1.8..2.8). At its death: the blasts
//!   at the two parts (`0x427670`, the hit handler's: crate::moby::blast at 1), the parts (and their followers)
//!   gone, the mending 0, state 2 and a station sent (`0x418698`, [`launch`]).
//! * **State 2**: the turn slowing (0.0000582 a tick to 0); with a station: it gone (shot down, `+0x110` 0) the next
//!   one in 600 ticks; else its way along the spline (`+0x2d4` by `+0x2d8` up to π, then the mending `+0x2e0` by
//!   `1 / (60 · the flight time)` a tick), facing its way; mended (1) → state 1, the station put away (`0x37dea0`),
//!   its health back. Smoke while the mending is below 0.5 / 0.75.
//!
//! The station's spline (`0x418698`): from the nearest of its four points (`+0x2a0..+0x2ac`, the level's spheres) to
//! the carrier, the station facing the fight's centre; the control points (`0x31aa28`'s two middles) each an end
//! pushed `160 · (1 − (1 − d / 512)²)` toward the centre, then mirrored across the segment (about its nearest point,
//! `0x311c18`, crate::moby::classes::race's `on_segment`); the carrier's end 8 toward the station.
//!
//! Not ported: the parts' joints (`0x29f978`: their manipulators at the joints 0 and 1, the parts' own update
//! `0x4194d0` is taken as run each tick here — side 0 turned a quarter about the carrier's facing, side 1 as it is,
//! the followers 0x134f (scale 16) with them), the shards (`0x29f9f0`) at the death, the mending's two arcs
//! (`0x39ea50`) while the station sits, the beams' pool cleared by [`on_activate`] (`0x265ba0`; the port's shots are
//! mobys) and the flags `0x1b0ee4` / `0x1b0ee8` nothing reads.

use crate::hero::physics::{add, cross, dist, len, scale, set_len, sub, unit, V3};
use crate::moby::classes::space::{self, a};
use crate::moby::{countdown, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0f85;
/// Its parts and their followers (`0x29efd0`).
pub const PART: i16 = 0x0f86;
pub const FOLLOWER: i16 = 0x134f;
/// The stations it sends (crate::moby::classes::level10::ships::STATION).
const STATION: i16 = 0x0f8d;

/// The pvars (the module's docs; the parts and the station kept as index + 1, their followers past the original's).
mod pv {
    pub const P0: usize = 0x260;
    pub const P1: usize = 0x270;
    pub const C0: usize = 0x280;
    pub const C1: usize = 0x290;
    pub const POINTS: usize = 0x2a0;
    pub const BASE_Z: usize = 0x2b8;
    pub const PHASE: usize = 0x2bc;
    pub const PART_A: usize = 0x2c0;
    pub const PART_B: usize = 0x2c4;
    pub const CHILD: usize = 0x2c8;
    pub const TURN: usize = 0x2cc;
    pub const SWAY: usize = 0x2d0;
    pub const T: usize = 0x2d4;
    pub const STEP: usize = 0x2d8;
    pub const WAIT: usize = 0x2dc;
    pub const MEND: usize = 0x2e0;
    pub const GUN: usize = 0x2e4;
    pub const FOLLOW_A: usize = 0x700;
    pub const FOLLOW_B: usize = 0x704;
    pub const FLIGHT: usize = 0x708;
}

/// Its numbers on the library (crate::moby::classes::space's `Spec`): health 30, left where it is, its target
/// `+0x2b0`, its leader word `+0x2b4`; the range 32 on screen.
pub(crate) fn spec() -> space::Spec {
    space::Spec { health: 30.0, scale: 1.0, scatter: false, range: 32.0, on_screen: true, target: 0x2b0, leads: 0x2b4, gun: pv::GUN, flyby: 0, engine: None, death: None, gun_sound: None, nozzles: &[], ..space::FIGHTER_SPEC }
}

fn idx(w: &MobyWorld, id: usize, o: usize) -> Option<usize> { usize::try_from(w.mobys[id].pv_i(o) - 1).ok().filter(|&i| i < w.mobys.len() && w.mobys[i].state < 0x80) }

/// `0x4189e0` (`func_L10_00417440`): the group's members back in state 1.
pub(crate) fn on_activate(w: &mut MobyWorld, id: usize) {
    for m in space::members(w, id) { w.mobys[m].state = 1; }
}

/// `0x418a68` (`func_L10_004174C8`): each member's parts (and their followers) gone.
pub(crate) fn on_put_away(w: &mut MobyWorld, id: usize) {
    for m in space::members(w, id) {
        if w.mobys[m].o_class == CLASS { drop_parts(w, m); }
    }
}

fn drop_parts(w: &mut MobyWorld, id: usize) {
    for o in [pv::FOLLOW_A, pv::FOLLOW_B, pv::PART_A, pv::PART_B] {
        if let Some(p) = idx(w, id, o) { w.delete(p); }
        w.mobys[id].set_pv_i(o, 0);
    }
}

/// `0x419280(carrier, side)`: a part 0xf86 (updated and drawn from anywhere, the rows kept, its class's scale × 0.8)
/// and its follower 0x134f (scale 16).
fn make_part(w: &mut MobyWorld, id: usize, ctx: &Ctx, side: bool) -> (i32, i32) {
    if w.class(PART).is_none() { return (0, 0); }
    let p = w.create(PART);
    let s = w.class(PART).map_or(1.0, |c| c.scale) * 0.8;
    {
        let m = &mut w.mobys[p];
        m.state = 0;
        m.update_dist = 0xff;
        m.draw_dist = 0x1ff;
        m.drawn = 1;
        m.mode |= mode::KEEP_ROWS;
        m.scale = s;
        m.set_pv_i(0x14, side as i32);
    }
    let mut f = 0;
    if w.class(FOLLOWER).is_some() {
        let q = w.create(FOLLOWER);
        let m = &mut w.mobys[q];
        m.state = 0;
        m.update_dist = 0xff;
        m.draw_dist = 0x1ff;
        m.drawn = 1;
        m.mode |= mode::KEEP_ROWS;
        m.scale = 16.0;
        f = q as i32 + 1;
    }
    place_part(w, id, p, if f == 0 { None } else { Some(f as usize - 1) }, side, ctx);
    (p as i32 + 1, f)
}

/// `0x4194d0`: the part at the carrier's place, side 1 with its rows, side 0 turned a quarter about its facing; the
/// follower with the part.
fn place_part(w: &mut MobyWorld, id: usize, p: usize, follower: Option<usize>, side: bool, ctx: &Ctx) {
    let (at, r) = (w.mobys[id].pos, w.mobys[id].rows);
    let rows = if side { r } else { [r[0], r[2], scale(r[1], -1.0)] };
    for m in std::iter::once(p).chain(follower) {
        w.mobys[m].pos = at;
        w.mobys[m].rows = rows;
        w.rebuild(m, &*ctx.host);
    }
}

/// `0x418b68`: the turn's and sway's steps up in state 1, down in state 2.
fn spin_up(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let (t, s) = (m.pv_f(pv::TURN), m.pv_f(pv::SWAY));
    match m.state {
        1 => {
            m.set_pv_f(pv::TURN, (t + 3.878_51e-5).min(0.023_271_061));
            m.set_pv_f(pv::SWAY, (s + 3.878_51e-5).min(0.011_635_531));
        }
        2 => {
            m.set_pv_f(pv::TURN, (t - 5.817_765e-5).max(0.0));
            m.set_pv_f(pv::SWAY, (s - 5.817_765e-5).max(0.0));
        }
        _ => {}
    }
}

/// The yaw by its step, the sway's phase by its own, the height 4 about its base by the sway.
fn turn(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.rot[2] = crate::hero::physics::add_rot(m.rot[2], m.pv_f(pv::TURN));
    let p = crate::hero::physics::add_rot(m.pv_f(pv::PHASE), m.pv_f(pv::SWAY));
    m.set_pv_f(pv::PHASE, p);
    m.pos[2] = p.sin() * 4.0 + m.pv_f(pv::BASE_Z);
}

/// `0x418c60`: with a target and the wait out, the guns' sound (0xf86's 0) and a shot from each part's joint 2
/// (`0x37cff8(m, 0x6000ff00, 1, 0.5, 0, 2, 0.9, 1.3, 0x1e, 2, part)`: 2 ahead of the target), the next in 50..120.
fn guns(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(t) = usize::try_from(space::ai_i(w, id, a::TARGET) - 1).ok().filter(|&t| t < w.mobys.len()) else { return };
    if w.mobys[id].pv_i(pv::GUN) != 0 { return; }
    ctx.host.play_sound_of(&w.mobys[id], PART, 0, 0);
    if w.mobys[t].draw_dist != 0 {
        let at = add(w.mobys[t].pos, set_len(w.mobys[t].rows[0], 2.0));
        for o in [pv::PART_A, pv::PART_B] {
            let Some(p) = idx(w, id, o) else { continue };
            let from = ctx.host.joint_matrix(&w.mobys[p], 2).map_or(w.mobys[p].pos, |(_, at)| at);
            crate::ship::spawn_enemy_shot(w, &*ctx.host, from, at, t, (0.5, 1.3, 0x6000_ff00));
        }
    }
    let n = ctx.game.rng.rand_range(0x32, 0x78);
    w.mobys[id].set_pv_i(pv::GUN, n);
}

/// `0x418d88(m, 0, r)` / `0x418fd8(m, 0, r)`: a smoke puff `r` round it (`0x2bdce8`, crate::particles::puff: size
/// `r` · 210000, rising away from the camera's side, 40..70 ticks): the dark one (0x101010, kind 0x44) drifting
/// `r`/30..`r`/15 a second at 40..50° up, or the grey one (0x307f7f7f, kind 0x48) `r`/120..`r`/60 at up to 80° up.
fn smoke(w: &MobyWorld, id: usize, ctx: &mut Ctx, r: f32, grey: bool) {
    let p = w.mobys[id].pos;
    let cam = ctx.host.camera_pos();
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let heading = crate::hero::physics::atan2(cam[0] - p[0], cam[1] - p[1]);
    let at = [p[0] + g.randf_signed(0.0, r), p[1] + g.randf_signed(0.0, r), p[2] + g.randf_signed(0.0, r)];
    let (speed, tilt) = if grey {
        (g.randf(r / 120.0, r / 60.0), g.randf_signed(0.0, 1.396_263_6))
    } else {
        (g.randf(r / 30.0, r / 15.0), g.randf_signed(0.698_131_8, 0.872_664_7))
    };
    let yaw = crate::hero::physics::add_rot(heading, g.randf_signed(0.0, std::f32::consts::FRAC_PI_2));
    let vel = super::super::shot::polar(speed, yaw, tilt);
    let colour = if grey { 0x307f_7f7f } else { 0x0010_1010 };
    let Some(i) = crate::particles::puff(sys, g, 0.0, 1.0, 1.01, r * 210_000.0, at, -1, vel, colour, 0) else { return };
    let life = g.rand_range(40, 70);
    let frame = (g.randi(4) + 0x60) as u8;
    let roll = g.rand_range(1, 10) as f32;
    let rec = &mut sys.pool.recs[i];
    rec[3] = if grey { 0x48 } else { 0x44 };
    rec[0xa..0xc].copy_from_slice(&(life as i16).to_le_bytes());
    rec[9] = frame;
    rec[0x3c..0x40].copy_from_slice(&roll.to_le_bytes());
    rec[0x24..0x28].copy_from_slice(&3i32.to_le_bytes());
    rec[0x2a] = 0x7f;
    rec[0x2b] = life as u8;
}

/// `0x418698` (module docs): a station sent from the nearest of its four points.
fn launch(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if idx(w, id, pv::CHILD).is_some() { return; }
    let Some(child) = space::activate_one(w, STATION, ctx) else { return };
    w.mobys[id].set_pv_i(pv::CHILD, child as i32 + 1);
    let me = w.mobys[id].pos;
    let point = |k: usize| usize::try_from(w.mobys[id].pv_i(pv::POINTS + 4 * k)).ok().and_then(|i| ctx.level.spheres.get(i)).map(|s| [s.matrix[3][0], s.matrix[3][1], s.matrix[3][2]]);
    let Some(best) = (0..4).filter_map(point).min_by(|p, q| dist(*p, me).total_cmp(&dist(*q, me))) else { return };
    let centre = space::centre(w, ctx);
    let fwd = unit(sub(centre, best));
    let left = unit(cross([0.0, 0.0, 1.0], fwd));
    {
        let c = &mut w.mobys[child];
        c.pos = best;
        c.rows = [fwd, left, cross(fwd, left)];
        c.mode |= mode::KEEP_ROWS;
    }
    let (p0, mut p1) = (best, me);
    let d = dist(p0, p1);
    let s = (1.0 - (1.0 - d * 0.001_953_13) * (1.0 - d * 0.001_953_13)) * 160.0;
    // Each control point the far end pushed along the way to the centre, then mirrored across the segment.
    let mirror = |c: V3| { let n = super::super::race::on_segment(p0, p1, c); sub(n, sub(c, n)) };
    let c0 = mirror(add(set_len(sub(centre, p0), s), p0));
    let c1 = mirror(add(set_len(sub(centre, p1), s), p1));
    p1 = add(p1, set_len(sub(p0, p1), 8.0));
    let m = &mut w.mobys[id];
    m.set_pv_v(pv::P0, p0);
    m.set_pv_v(pv::P1, p1);
    m.set_pv_v(pv::C0, c0);
    m.set_pv_v(pv::C1, c1);
    m.set_pv_f(pv::T, 0.0);
    m.set_pv_f(pv::STEP, 0.011_635_531);
}

/// `0x31aa28(out, b, c, a, d, t)`: the curve from `b` (t 0) to `c` (t 1) by `a` and `d`.
fn curve(b: V3, c: V3, a_: V3, d: V3, t: f32) -> V3 {
    if t == 0.0 { return b; }
    if t == 1.0 { return c; }
    let (t2, t3) = (t * t, t * t * t);
    std::array::from_fn(|k| {
        let p = (d[k] - c[k]) - (a_[k] - b[k]);
        let q = (a_[k] - b[k]) - p;
        let r = c[k] - a_[k];
        p * t3 + q * t2 + r * t + b[k]
    })
}

/// `0x417c88`.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let started = w.mobys[id].pv_i(space::STARTED) != 0;
    let died = started && space::hits_with(w, id, ctx, true, true, true);
    if started { space::range(w, id, ctx); }
    match w.mobys[id].state {
        0 => {
            w.mobys[id].update_dist = 0xff;
            // `0x4185e0`.
            space::start(w, id, ctx);
            let phase = ctx.game.rng.rand_angle();
            let s = w.class(CLASS).map_or(1.0, |c| c.scale) * 6.4;
            let m = &mut w.mobys[id];
            m.mode &= !mode::KEEP_ROWS;
            m.state = 1;
            m.set_pv_f(pv::PHASE, phase);
            m.set_pv_f(pv::BASE_Z, m.pos[2]);
            for o in [pv::PART_A, pv::PART_B, pv::CHILD, pv::FOLLOW_A, pv::FOLLOW_B, pv::TURN, pv::SWAY, pv::MEND, pv::WAIT] { m.set_pv_i(o, 0); }
            m.scale = s;
            m.set_pv_i(pv::GUN, 0x21c);
            m.set_pv_f(pv::FLIGHT, 30.0);
            m.glow = 0xff;
            return;
        }
        1 => {
            for (o, f, side) in [(pv::PART_A, pv::FOLLOW_A, true), (pv::PART_B, pv::FOLLOW_B, false)] {
                if idx(w, id, o).is_none() {
                    let (p, q) = make_part(w, id, ctx, side);
                    w.mobys[id].set_pv_i(o, p);
                    w.mobys[id].set_pv_i(f, q);
                }
            }
            let secs = (space::active_count(w) as f32 * 2.5 + 30.0).min(60.0);
            w.mobys[id].set_pv_f(pv::FLIGHT, secs);
            space::take_own_target(w, id);
            spin_up(w, id);
            turn(w, id);
            let target = usize::try_from(space::ai_i(w, id, a::TARGET) - 1).ok().filter(|&t| t < w.mobys.len());
            let far = target.map_or(0.0, |t| dist(w.mobys[t].pos, w.mobys[id].pos));
            let me = w.mobys[id].pos;
            let r = w.mobys[id].bsphere[3] / 1024.0;
            if space::in_view(ctx, me, r) && dist(me, ctx.host.camera_pos()) < 64.0 + r && 24.0 <= far { guns(w, id, ctx); }
            let mut g = w.mobys[id].pv_i(pv::GUN);
            countdown(&mut g);
            w.mobys[id].set_pv_i(pv::GUN, g);
            let hp = w.mobys[id].pv_f(space::HEALTH);
            if hp < 25.0 { let r = ctx.game.rng.randf(2.0, 3.0); smoke(w, id, ctx, r, false); }
            if hp < 15.0 { let r = ctx.game.rng.randf(1.8, 2.8); smoke(w, id, ctx, r, true); }
            if died {
                let blast = crate::moby::blast::Blast { size: 1.0, bursts: [0x32, 0x1e, 0x1e], puffs: 0x1e, streaks: 0x28, light: true, shake: false };
                for (o, light) in [(pv::PART_A, true), (pv::PART_B, false)] {
                    if let Some(p) = idx(w, id, o) {
                        let at = w.mobys[p].pos;
                        crate::moby::blast::burst(w, ctx, at, &crate::moby::blast::Blast { light, ..blast });
                    }
                }
                drop_parts(w, id);
                w.mobys[id].set_pv_f(pv::MEND, 0.0);
                w.mobys[id].state = 2;
                launch(w, id, ctx);
            }
        }
        2 => {
            spin_up(w, id);
            turn(w, id);
            mend(w, id, ctx);
            let m = w.mobys[id].pv_f(pv::MEND);
            if m <= 0.5 { let r = ctx.game.rng.randf(1.9, 2.25) - m; smoke(w, id, ctx, r, true); }
            if m <= 0.75 { let r = ctx.game.rng.randf(2.0, 3.0) - m; smoke(w, id, ctx, r, false); }
        }
        _ => {}
    }
    spin_up(w, id);
    for (o, f, side) in [(pv::PART_A, pv::FOLLOW_A, true), (pv::PART_B, pv::FOLLOW_B, false)] {
        if let Some(p) = idx(w, id, o) {
            let q = idx(w, id, f);
            place_part(w, id, p, q, side, ctx);
        }
    }
    w.rebuild(id, &*ctx.host);
}

/// State 2's station: on its way, or the next one sent once the wait is out.
fn mend(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(c) = idx(w, id, pv::CHILD) else {
        let mut t = w.mobys[id].pv_i(pv::WAIT);
        if countdown(&mut t) { launch(w, id, ctx); }
        w.mobys[id].set_pv_i(pv::WAIT, t);
        return;
    };
    if space::ai_i(w, c, a::ACTIVE) == 0 {
        w.mobys[id].set_pv_i(pv::WAIT, 600);
        w.mobys[id].set_pv_i(pv::CHILD, 0);
        return;
    }
    let m = &mut w.mobys[id];
    let mut t = m.pv_f(pv::T) + m.pv_f(pv::STEP);
    if std::f32::consts::PI < t {
        t = std::f32::consts::PI;
        let secs = m.pv_f(pv::FLIGHT).max(1.0);
        let v = m.pv_f(pv::MEND) + 1.0 / (secs * 60.0);
        m.set_pv_f(pv::MEND, v);
    }
    m.set_pv_f(pv::T, t);
    let (p0, p1, c0, c1) = (m.pv_v(pv::P0), m.pv_v(pv::P1), m.pv_v(pv::C0), m.pv_v(pv::C1));
    let old = w.mobys[c].pos;
    let at = curve(p0, p1, c0, c1, ((t - std::f32::consts::PI).cos() + 1.0) * 0.5);
    let d = sub(at, old);
    {
        let s = &mut w.mobys[c];
        s.pos = at;
        if 0.0001 <= len(d) {
            let fwd = unit(d);
            let left = unit(cross([0.0, 0.0, 1.0], fwd));
            s.rows = [fwd, left, cross(fwd, left)];
        }
    }
    w.rebuild(c, &*ctx.host);
    if 1.0 <= w.mobys[id].pv_f(pv::MEND) {
        space::put_away(w, c);
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::WAIT, 0);
        m.state = 1;
        m.set_pv_f(pv::MEND, 1.0);
        m.set_pv_i(pv::CHILD, 0);
        m.set_pv_f(space::HEALTH, 30.0);
    }
}
