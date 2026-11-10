//! Damosel's stalker **0x0f0a** (L18 `0x431738`, 38 placed; the decomp's `func_L18_00430248`): legs that carry a
//! body 0x0f0b on their joint 0 (its own update the shadow alone, L09's `0x3f85e8`: crate::moby::classes::
//! trooper::update_turret) with a gun on the body's joint 2 — a beam 0x0f0e, or with `+0x7fc` set a lobber 0x0f0d.
//! It hunts Ratchet and the breeders 0x0b69 (crate::moby::classes::breeder) of a group; some come in hidden
//! on a dropship 0x0ede (crate::moby::classes::level18::dropship) that calls [`carried`], [`release`] and [`die`]. On
//! the framework (crate::moby::ai): health `+0x20` (16 by default; its status `+0x2e`, its damage byte `+0x34`, its
//! bite's `+0x36`, the last hitter `+0x38`), reaction `+0x50` (a hurt to 13, sub 2), movement `+0x100` (radius 1.25,
//! height 0.4, steps 0.375, slope 90°, gravity 1/360), target `+0x1f0`.
//!
//! **Its pvars:** `+0x210` its path network (0x20 records, `+0x780` how many; built from the path `+0x7d4`, −1 none),
//! `+0x610..+0x750` the beam's two ribbons (`+0x610` / `+0x620` its root and tip, `+0x6b0` / `+0x6c0` the second's;
//! their wave phases `+0x650..`), `+0x750` the goal, `+0x760` where the shot goes, `+0x770` the held moby's place,
//! `+0x78c` / `+0x7d8` / `+0x7dc` the network's record now, the goal's and the heading, `+0x790` / `+0x794` its target
//! area and range, `+0x798` its fence path, `+0x79c` found, `+0x7a0` its walk's yaw offset, `+0x7a4` the yaw it
//! faces, `+0x7a8` the hunt's patience (480..600), `+0x7ac` the idle's wait (0..30), `+0x7b4` the breeders' group,
//! `+0x7b8` the nearest breeder, `+0x7bc` the moby it goes for (0 Ratchet's point), `+0x7c0` / `+0x7c4` / `+0x7c8`
//! the body, the inner body 0x0f0c and the gun, `+0x7cc` the body's turn speed, `+0x7d0` the cuboid it walks to off
//! the dropship (−1 none), `+0x7e0` the beam's reach (0..1, for its draw), `+0x7e4` the moby the beam holds,
//! `+0x7e8` its size (0.5 on the dropship, growing by 0.01 a tick to 1.5), `+0x7ec` its dropship, `+0x7f0` the beam's
//! loop sound, `+0x7f4` the most of its group at Ratchet at once (−1 any; `+0x7f6` how many are), `+0x7f8` it rides
//! a dropship (hidden at first), `+0x7fa` dormant until its group is roused, `+0x7fc` a lobber, `+0x800` its kill
//! height (−1 none), `+0x804` the beam's / reload ticks, `+0x806` the beam's mode, `+0x808` its facing this tick (1
//! turn the body, 2 turn itself), `+0x80a` the crawl's ticks, `+0x80c` the drop's wait, `+0x80e` hurt once.
//!
//! **The update** (`0x431738`): state 0 the set-up ([`setup`], `0x4317a0`), 1 nothing, else the frame's checks
//! ([`head`], `0x431de0`), the state ([`states`], `0x432348`), the move by its sequence ([`movement`], `0x433560`)
//! and the tail ([`tail`], `0x433800`). Every sequence it starts goes to its body and inner body too ([`seq_all`],
//! `0x434368`).
//!
//! **States** (`+0x20`): 1 hidden on its dropship (no update). 2 carried, shown (sequence 1). 3 dropped (from its
//! dropship): after 0..29 ticks sequence 2 and the jump to 2 ahead and 6 down (`0x348ca8` = L07 `0x324150`, 1/30 a
//! tick, gravity 1/180; its tilt eased to 0, the push-out's flags and its dropship's collision off for the step,
//! `0x348d60` = L07 `0x324208(11, 18, 27)`); landed, the run's limits, sequence 9, stop 0.2 and its cuboid walked to
//! (within 0.5 across) or 4 (sub 1). 4 idle (sequence 3 / 4, again at its end): not dormant, its target found and the
//! wait out → 5 out of 6. 5 the alert (sequence 5), then the state before. 6 the hunt (sequence 6 / 7, 480..600
//! ticks): its target found → 8 while its group has room at Ratchet, else the nearest breeder of `+0x7b4` (`0x433ca0`:
//! not gone, shown) → 7 (sub 1); the patience out → 4. 7 / 8 to the breeder / Ratchet: out of range (10 across, 30
//! for a lobber), not within 90° of the body's facing or out of the gun's sight (joint 0, line flags 0x16) → 9; within
//! 3.8 → 11; ready (a lobber 150 ticks after its shot) → 10; from 7 with room at Ratchet → 6. 9 the walk (sequence 8,
//! the walk's limits, stop 2; farther than 7 out of range sequence 9 with the run's limits, nearer than 4 back to 8):
//! in range and sight at its steps (frames 4..9, 21..26) sequence 6 / 7 and the state before; stuck (`+0x180` ≥ 2 or a
//! flag 1..8) sequence 6 / 7 and its ticks from 0. 10 the shot (sequence 10, then turned to aim (½°, the lobber 2°):
//! 0xd the beam / 0xc the lob, its frames before 5 the beam started / before 2 the lob, at its end 0xe, then 8 and the
//! state before). 11 / 12 the bite (sequence 0xf from frame 0 / 2): frames 11..17 the sphere 0.8 at joint 2 (its bite
//! byte, flags 0x10001, 4 away). 13 hurt (the framework's flight, the beam put back): dead or 180 ticks its bolts and
//! gone; over (or sequence 0x10 past 27, 0x11 past 47) with its body 8 (sub 1) then 11 within 3.8 of its target,
//! else 0xe; with its body, hit by a 0x0047 past frame 13 of 0x10, 8 then 12. 14 its body lost (1 in 7 of the deaths
//! it lives on with 8 health): the body blown off at its joint 0 (size 0.5) and all its parts gone, sequence 8; then
//! after its target point, 11 within 3.8; after 600 ticks its bolts and gone.
//!
//! **Each tick** ([`head`]): a hit from its own kind or a 0x0f9e, or while carried, does no damage; the frame's damage
//! (`0x342658` = L00 `0x327e38`, crate::moby::ai::take_damage; the state before kept from 5 and 9..13): without a body
//! and dead its bolts and gone; dead, 1 in 7 to 14 with 8 health, else its sequence again on its parts; its group
//! roused ([`rouse`], `0x434250`: each in 4 to 5 out of 6). On a 0x0f59, under 90 or its kill height, its status 2 or
//! dead outside 13: gone (bolts on the 0x0f59). The body's break pieces by its health (crate::moby::shards::part_break, `0x344b00` = L00
//! `0x32a2e0`); its piece 1 off, the inner body 0x0f0c made (sequence 0x14, mode 6, mirrored as it is); the body and
//! inner body take its ambient (`0x32e370` / `0x32e340` = L00 `0x313330` / `0x313300`). Its group's count at Ratchet
//! (`0x434318`: 8, 13, or 9..12 out of 8); the target search (`0x345e10` = L00 `0x32ad38`, its area and range, height
//! 0); from 4 on (3 once dropping) its size grows.
//!
//! **The move** ([`movement`], by its sequence `+0x42`, else `+0x43`): 1 the body faces as it does. 3..7, 10..15: a
//! facing asked to turn the body (1) within 35° of the yaw: the body turns (`0x33a350` = L00 `0x31f230`: 0.4°, 0.4°,
//! 12°) and it holds (`0x347a98` = L00 `0x32ce98`), else it turns to it in place (`0x347ab8` = L00 `0x32ceb8`); with
//! no ask it holds; the body turns to its own facing. 8 / 9 the walk: through its network (`0x345298` = L02 `0x32fd40`,
//! crate::path_network::steer, 0.4; `0x347920` = L00 `0x32cd20`) or straight (`0x347830` = L00 `0x32cc30` with its
//! offset) to the goal; the body turns as above.
//!
//! **The tail** ([`tail`]): its ticks and `+0x804`; the body on its joint 0 at its speed, the gun on the body's joint 2
//! (`0x340050` = L00 `0x3253f0`, crate::moby::classes::enemies::place_part), the inner body on the body's joint 0 with
//! its angles; from state 2 a beam's tick ([`beam`], `0x434950`); its shadow within 24 (range 0x16); with an inner body
//! the head aim (`0x341e28` = L00 `0x327308`, the record `+0x810` on the inner body's list 5, 2.5:
//! crate::moby::joint_aim::head_on).
//!
//! **The beam** ([`beam`], `+0x806`): 0 at rest (its sound let go). 1 (from 10's start, 5 ticks): the sphere 0.2 at the
//! gun's joint 0 (its damage byte, flags 0x10001), then 2 with the loop sound 1 at the target. 2 (5 ticks): the tip
//! out toward the target, the sphere, and the line root → tip (flags 0x210001, its damage byte, the tip's way / 30,
//! kind 5 | 3 << 8 | class << 16); a moby met: held (3, the tip at the hit), the world: 4. 3 (50 ticks): the tip
//! follows the held moby, then 4. 4 (5 ticks): back from the target to the gun, then 0.
//!
//! The engine functions with the port's: `0x3368c0` (crate::moby::Moby::set_state), `0x3365e0` (set_anim), `0x3458a0`
//! (first), `0x32c880` (the frame time), `0x32d808` (a joint), `0x336990` / `0x3369d0` / `0x336bb0` (the rng),
//! `0x339c38` = L00 `0x31eb18` (approach), `0x3489d0` = L00 `0x32dcf0` (crate::moby::ai::senses: the walk's
//! `0x433e00`, the run's `0x433e50`), `0x3382a0` / `0x338158` = L00 `0x31d2c8` / `0x31d180` (ai::attack_joint /
//! attack_sphere), `0x2f9110` = L00 `0x2de878` (the lines), `0x3433a0` = L00 `0x328b80` (ai::flight), `0x335db0` = L00
//! `0x31adc0` (crate::moby::bolts::drop), `0x415e70` = L00 `0x3f8348` (crate::moby::blast::burst), `0x344e08` = L02
//! `0x32f8b0` (crate::path_network::build), `0x338b78` = L00 `0x31da58` (crate::shadow_volumes::probe_near), `0x37c8d8`
//! / `0x37c590` / `0x37c5d8` (the class sounds).
//!
//! Not ported: the lob's shot (`0x4346c0`: the spit 0x0f9e, `0x438f90`, and its two sprites; its class sound 7 and
//! the reload are), the draws (`0x302130`: the beam's ribbons `0x435060` with their wave `0x435618`, the glows
//! `0x434408`, the bodiless one's `0x4364b0` and its smoke `0x4365d0`), the beam's sparks (`0x435f88`). Its state 7 does not
//! open the hatch 0x1151: the opener's callers are 0x0ef4's `0x3f9510` (state 7 there) and `0x3fbd90`.

use crate::hero::physics::{add, approach, atan2, dist2, rot_dist, scale, sub, turn_ease, V3};
use crate::moby::blast::{self, Blast};
use crate::moby::classes::enemies::place_part;
use crate::moby::classes::props::{seq, set_seq, wrapped};
use crate::moby::hits::HitTemplate;
use crate::moby::{ai, bolts, countdown, mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0f0a;
/// The body on its joint 0 (its update the shadow).
pub const BODY: i16 = 0x0f0b;
/// The inner body, made when the body's piece 1 breaks off.
pub const INNER: i16 = 0x0f0c;
/// The lobber's gun and the beam's.
pub const LOBBER_GUN: i16 = 0x0f0d;
pub const BEAM_GUN: i16 = 0x0f0e;
/// Its hits do it no harm.
const SPIT: i16 = 0x0f9e;
/// Standing on it kills it.
const KILL_FLOOR: i16 = 0x0f59;
/// The prey of state 7.
const PREY: i16 = super::super::breeder::CLASS;
/// A hitter that knocks it from its hurt into the second bite.
const KNOCKER: i16 = 0x0047;

const H: usize = 0x20;
const STATUS: usize = 0x2e;
const DAMAGE: usize = 0x34;
const BITE: usize = 0x36;
const HITTER: usize = 0x38;
const REACT: usize = 0x50;
const MV: usize = 0x100;
const T_POINT: usize = 0x1f0;
const NODES: usize = 0x210;
const NODES_MAX: usize = 0x20;
const ROOT: usize = 0x610;
const TIP: usize = 0x620;
const ROOT2: usize = 0x6b0;
const TIP2: usize = 0x6c0;
const GOAL: usize = 0x750;
const AIM: usize = 0x760;
const HELD_AT: usize = 0x770;
const N_NODES: usize = 0x780;
const NET_AT: usize = 0x78c;
const AREA: usize = 0x790;
const RANGE: usize = 0x794;
const FENCE: usize = 0x798;
const FOUND: usize = 0x79c;
const WALK_OFF: usize = 0x7a0;
const FACE: usize = 0x7a4;
const PATIENCE: usize = 0x7a8;
const WAKE: usize = 0x7ac;
const PREY_GROUP: usize = 0x7b4;
const PREY_LINK: usize = 0x7b8;
const TARGET: usize = 0x7bc;
const BODY_L: usize = 0x7c0;
const INNER_L: usize = 0x7c4;
const GUN_L: usize = 0x7c8;
const BODY_TURN: usize = 0x7cc;
const SPOT: usize = 0x7d0;
const NET: usize = 0x7d4;
const NET_GOAL: usize = 0x7d8;
const NET_YAW: usize = 0x7dc;
const REACH: usize = 0x7e0;
const HELD: usize = 0x7e4;
const SIZE: usize = 0x7e8;
const CARRIER: usize = 0x7ec;
const VOICE: usize = 0x7f0;
const MOST: usize = 0x7f4;
const AT_HERO: usize = 0x7f6;
const RIDES: usize = 0x7f8;
const DORMANT: usize = 0x7fa;
const LOBS: usize = 0x7fc;
const KILL_Z: usize = 0x800;
const RELOAD: usize = 0x804;
const BEAM: usize = 0x806;
const FACING: usize = 0x808;
const CRAWL: usize = 0x80a;
const DROP_WAIT: usize = 0x80c;
const HURT_ONCE: usize = 0x80e;
/// The head aim's record (crate::moby::joint_aim), hung on the inner body.
const HEAD_AIM: usize = 0x810;
const PVARS: usize = 0x820;

/// `$gp`-free data at `0x1b1510`: the body's break thresholds (health, over its most × 255).
const THRESHOLDS: [f32; 15] = [20.0, 20.0, 20.0, 20.0, 20.0, 5.0, 5.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
/// 0.4° / 12°: the body's turn.
const BODY_ACCEL: f32 = 0.006_981_318;
const BODY_MOST: f32 = 0.209_439_55;
/// 35°: beyond it a facing turns the stalker itself.
const BODY_REACH: f32 = 0.610_865_3;
const HALF_PI: f32 = 1.570_796_5;

fn link(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { ai::link(&w.mobys[id], off).filter(|&k| k < w.mobys.len()) }
fn lobs(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_s(LOBS) != 0 }
fn seq_a(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { ctx.host.anim(&w.mobys[id]).map_or(0xff, |a| a.seq_a) }
fn speed_of(w: &MobyWorld, id: usize, ctx: &Ctx) -> f32 { ctx.host.anim(&w.mobys[id]).map_or(1.0, |a| a.speed) }
fn ticks(w: &MobyWorld, id: usize) -> u16 { w.mobys[id].state_ticks as u16 }
fn gone(s: u8) -> bool { s == 0xfe || s == 0xfd }
fn class_coll(w: &MobyWorld, k: usize) -> bool { w.class(w.mobys[k].o_class).is_some_and(|c| c.has_collision) }

/// `+0x2c` = the class's scale × `f`.
fn resize(w: &mut MobyWorld, k: usize, f: f32) {
    let s = w.class(w.mobys[k].o_class).map_or(1.0, |c| c.scale);
    w.mobys[k].scale = s * f;
}

/// Its parts: the body, the inner body, the gun.
fn parts(w: &MobyWorld, id: usize) -> [Option<usize>; 3] { [link(w, id, BODY_L), link(w, id, INNER_L), link(w, id, GUN_L)] }

/// `0x434368(m, seq, frame, blend)`: the sequence on it, its body and inner body, each unless it plays.
fn seq_all(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, s: u8, frame: u8, blend: u8) {
    set_seq(w, id, ctx, s, frame, blend);
    for k in [link(w, id, BODY_L), link(w, id, INNER_L)].into_iter().flatten() { set_seq(w, k, ctx, s, frame, blend); }
}

fn set_speed(w: &MobyWorld, id: usize, ctx: &mut Ctx, s: f32) { ctx.host.set_anim_speed(&w.mobys[id], s); }

/// The beam's loop sound let go.
fn stop_voice(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(VOICE);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(VOICE, -1);
}

/// `0x433ea0`: gone — its sound let go, a blast at it (size 1), its class sound 6, its parts and itself deleted.
pub fn die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    stop_voice(w, id, ctx);
    let at = w.mobys[id].pos;
    blast::burst(w, ctx, at, &Blast { size: 1.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true });
    ctx.host.play_sound(&w.mobys[id], 6, 0);
    for off in [BODY_L, INNER_L, GUN_L] {
        if let Some(k) = link(w, id, off) { w.delete(k); }
        ai::set_link(&mut w.mobys[id], off, None);
    }
    w.delete(id);
}

/// Its bolts (`0x335db0(m, 0, 3)`), then gone.
fn drop_and_die(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    bolts::drop(w, id, 0, 3, ctx);
    die(w, id, ctx);
}

/// `0x434020(m, carrier)`: onto its dropship (unless already, state 2) — shown and updated with its collision, its
/// parts shown without theirs, half size, carried (2, sub 1) and no target, not dormant.
pub fn carried(w: &mut MobyWorld, id: usize, carrier: usize) {
    if w.mobys[id].state == 2 { return; }
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    let coll = class_coll(w, id);
    let m = &mut w.mobys[id];
    ai::set_link(m, CARRIER, Some(carrier));
    m.mode = (m.mode & 0xffbe) | mode::TARGET;
    m.has_collision = coll;
    m.mode &= !mode::NO_UPDATE;
    m.set_pv_f(SIZE, 0.5);
    resize(w, id, 0.5);
    for k in parts(w, id).into_iter().flatten() {
        let p = &mut w.mobys[k];
        p.mode = (p.mode & 0xffbe) | mode::TARGET;
        p.has_collision = false;
        p.mode &= !mode::NO_UPDATE;
        let f = w.mobys[id].pv_f(SIZE);
        resize(w, k, f);
    }
    let m = &mut w.mobys[id];
    m.set_state(2, 1);
    m.mode &= !mode::TARGET;
    m.set_pv_s(DORMANT, 0);
}

/// `0x4341f8(m)`: dropped (from 2: 3, sub 1), a target again, not dormant.
pub fn release(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    if m.state != 2 { return; }
    m.set_state(3, 1);
    m.mode |= mode::TARGET;
    m.set_pv_s(DORMANT, 0);
}

/// `0x433fc0` via `0x434250`: each of its group idle (4) roused — 5 out of 6 (sub 1), not dormant.
fn rouse(w: &mut MobyWorld, id: usize) {
    let g = w.mobys[id].group;
    let Some(members) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for e in members {
        let Some(m) = w.mobys.get_mut(e) else { break };
        if m.state != 4 { continue; }
        m.set_state(6, 1);
        m.set_state(5, -1);
        m.set_pv_s(DORMANT, 0);
    }
}

/// `0x434318(m)`: at Ratchet — 8, 13, or 9..12 out of 8.
fn at_hero(state: u8, prev: u8) -> bool { state == 8 || state == 13 || ((9..=12).contains(&state) && prev == 8) }

/// 0x0f0a (module docs).
pub fn update_0f0a(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < PVARS { w.mobys[id].pvars.resize(PVARS, 0); }
    match w.mobys[id].state {
        0 => setup(w, id, ctx),
        1 => {}
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

/// A part made like it (`0x32c320`): its update and draw distances, light and ambient (`+0x38`), rebuilt, no collision.
fn make_part(w: &mut MobyWorld, id: usize, ctx: &Ctx, class: i16) -> usize {
    let k = w.create(class);
    let (u, d, l, a) = { let m = &w.mobys[id]; (m.update_dist, m.draw_dist, m.light, m.ambient) };
    let p = &mut w.mobys[k];
    p.update_dist = u;
    p.draw_dist = d;
    p.light = l;
    p.ambient = a;
    w.rebuild(k, &*ctx.host);
    w.mobys[k].has_collision = false;
    k
}

/// The beam's wave: four of the six random vectors of each ribbon (`0x336bb0`), the first pair's length in
/// 0.2327..0.2909, the second's `lo..hi`.
fn waves(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, lo: f32, hi: f32) {
    for j in 0..2 {
        let o = 0xa0 * j;
        for (off, a, b) in [(0x670, 0.232_710_6, 0.290_888_25), (0x680, 0.232_710_6, 0.290_888_25), (0x690, lo, hi), (0x6a0, lo, hi)] {
            let v = ctx.game.rng.rand_vec(a, b);
            w.mobys[id].set_pv_v(off + o, v);
        }
    }
}

/// `0x4317a0` (module docs): the health, reaction and movement blocks; the body (on its joint 0, its break
/// thresholds) and the gun; the beam's wave; mirrored at random; its network; hidden on its dropship (1, half its
/// parts' collision gone, size 1) or idle (4, sub 1, size 1.5).
fn setup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    // `0x30a7a8` (L00 `0x2effb8`): 15 and 16 as bytes.
    m.set_pv_b(0x48, 15);
    m.set_pv_b(0x4a, 16);
    m.set_pv_b(REACT + 0x10, 13);
    m.set_pv_i(REACT + 0x48, 2);
    m.set_pv_f(REACT + 0x34, 1.5);
    m.set_pv_f(REACT + 0x58, 0.1);
    m.set_pv_f(REACT + 0x5c, 0.33);
    let fence = m.pv_i(FENCE);
    m.set_pv_i(REACT + 0x7c, fence);
    m.set_pv_f(REACT + 0x30, 1.5);
    m.set_pv_f(REACT + 0x3c, 0.4);
    if m.pv_s(H + 4) == 0 {
        m.set_pv_f(H, 16.0);
        m.set_pv_s(H + 4, 16);
    }
    m.set_pv_b(0x28, 3);
    m.set_pv_f(0x30, 1.0);
    m.set_pv_b(STATUS, 1);
    m.set_pv_f(MV + 4, 0.4);
    m.set_pv_f(MV, 1.25);
    m.set_pv_f(MV + 0x2c, 0.002_777_777_8);
    m.set_pv_f(MV + 0xc, 0.375);
    m.set_pv_f(MV + 0x10, 1.570_796_4);
    m.set_pv_f(MV + 8, 0.375);
    if fence != -1 { m.set_pv_i(MV + 0x3c, fence); }

    let b = make_part(w, id, ctx, BODY);
    ai::set_link(&mut w.mobys[id], BODY_L, Some(b));
    let at = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
    let rot = w.mobys[id].rot;
    let most = w.mobys[id].pv_s(H + 4) as f32;
    let bm = &mut w.mobys[b];
    bm.pos = at;
    bm.rot = rot;
    for (k, t) in THRESHOLDS.iter().enumerate() { bm.set_pv_b(0x20 + k, (t / most * 255.0) as i32 as u8); }

    let lob = lobs(w, id);
    let g = make_part(w, id, ctx, if lob { LOBBER_GUN } else { BEAM_GUN });
    ai::set_link(&mut w.mobys[id], GUN_L, Some(g));
    if !lob {
        for j in 0..2 {
            let o = 0xa0 * j;
            for (off, a, b) in [(0x650, -3.141_592_7, 3.141_592_7), (0x660, -3.141_592_7, 3.141_592_7), (0x670, 0.232_710_6, 0.290_888_25), (0x680, 0.232_710_6, 0.290_888_25), (0x690, 1.0, 1.7), (0x6a0, 1.0, 1.7)] {
                let v = ctx.game.rng.rand_vec(a, b);
                w.mobys[id].set_pv_v(off + o, v);
            }
        }
    }
    if ctx.game.rng.randi(2) == 0 {
        w.mobys[id].mode |= mode::MIRROR;
        w.mobys[b].mode |= mode::MIRROR;
    }
    let net = w.mobys[id].pv_i(NET);
    if net != -1 {
        let m = &mut w.mobys[id];
        let mut nodes = crate::path_network::read_nodes(&m.pvars, NODES, NODES_MAX);
        let n = crate::path_network::build(ctx.level, net, &mut nodes);
        crate::path_network::write_nodes(&mut m.pvars, NODES, &nodes[..n]);
        m.set_pv_i(N_NODES, n as i32);
        crate::path_network::open(&mut ctx.game.path_openings, ctx.level, &nodes[..n]);
    }
    if lob {
        let r = ctx.game.rng.randi(150) as i16;
        w.mobys[id].set_pv_s(RELOAD, r);
    }
    let m = &mut w.mobys[id];
    m.set_pv_s(AT_HERO, m.pv_s(MOST));
    let size = if m.pv_s(RIDES) == 0 {
        m.set_state(4, 1);
        1.5
    } else {
        m.set_state(1, 0);
        m.has_collision = false;
        m.mode = (m.mode & 0xefff) | 0x43;
        for k in parts(w, id).into_iter().flatten() {
            let p = &mut w.mobys[k];
            p.mode |= 0x41;
            p.mode &= !mode::TARGET;
            p.has_collision = false;
            p.mode |= mode::NO_UPDATE;
        }
        1.0
    };
    w.mobys[id].set_pv_f(SIZE, size);
    resize(w, id, size);
    for k in parts(w, id).into_iter().flatten() { resize(w, k, size); }
}

/// `0x431de0` (module docs).
fn head(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    // `0x338e08(m, 0x10000, 1)`: the record kept; its damage none from its own kind, a spit, or while carried.
    if let Some(h) = w.hit_message(id).filter(|h| h.tmpl.flags & 0x10000 != 0) {
        let own = h.tmpl.source.and_then(|s| w.mobys.get(s)).is_some_and(|s| s.o_class == CLASS || s.o_class == SPIT);
        let held = w.mobys[id].state.wrapping_sub(1) < 2;
        if own || held {
            let slot = w.mobys[id].hit_slot as usize;
            if let Some(r) = w.hits.records.get_mut(slot) { r.tmpl.damage = 0.0; }
        }
    }
    let saved = w.mobys[id].prev_state;
    if ai::take_damage(w, id, ctx, false) != 0 {
        let m = &mut w.mobys[id];
        if matches!(m.prev_state, 13 | 9 | 10 | 11 | 12 | 5) { m.prev_state = saved; }
        let hp = m.pv_f(H);
        if link(w, id, BODY_L).is_none() && hp <= 0.0 {
            drop_and_die(w, id, ctx);
            return;
        }
        if hp <= 0.0 && ctx.game.rng.randi(7) == 0 {
            let coll = class_coll(w, id);
            let m = &mut w.mobys[id];
            m.set_pv_f(H, 8.0);
            m.has_collision = coll;
            m.set_state(14, -1);
            m.set_pv_s(REACT + 0x38, 0);
            m.set_pv_b(REACT + 0x1a, 0);
        } else {
            let s = seq(w, id, ctx);
            seq_all(w, id, ctx, s, 0, 3);
        }
        rouse(w, id);
    }
    w.clear_hit(id);
    if ai::link(&w.mobys[id], MV + 0x88).and_then(|k| w.mobys.get(k)).is_some_and(|f| f.o_class == KILL_FLOOR) {
        drop_and_die(w, id, ctx);
        return;
    }
    if let Some(b) = link(w, id, BODY_L) {
        crate::moby::shards::part_break(w, ctx, b, 0x20, id, H);
        if w.mobys[b].bangles & 2 == 0 && link(w, id, INNER_L).is_none() {
            let k = make_part(w, id, ctx, INNER);
            ai::set_link(&mut w.mobys[id], INNER_L, Some(k));
            set_seq(w, k, ctx, 0x14, 0, 3);
            let (mirror, size) = (w.mobys[id].mode & mode::MIRROR, w.mobys[id].pv_f(SIZE));
            let p = &mut w.mobys[k];
            p.mode |= mode::NO_UPDATE | mode::NO_MATRIX;
            p.mode |= mirror;
            resize(w, k, size);
        }
    }
    let amb = w.mobys[id].ambient;
    for k in [link(w, id, BODY_L), link(w, id, INNER_L)].into_iter().flatten() { w.mobys[k].ambient = amb; }
    let most = w.mobys[id].pv_s(MOST);
    let n = if most != -1 {
        let g = w.mobys[id].group;
        let members = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned().unwrap_or_default();
        members.iter().filter_map(|&e| w.mobys.get(e)).filter(|e| at_hero(e.state, e.prev_state)).count() as i16
    } else {
        most
    };
    w.mobys[id].set_pv_s(AT_HERO, n);
    let m = &w.mobys[id];
    let kill_z = m.pv_f(KILL_Z);
    if m.pos[2] < 90.0 || (kill_z != -1.0 && m.pos[2] < kill_z) || m.pv_b(STATUS) == 2 || (m.pv_f(H) <= 0.0 && m.state != 13) {
        die(w, id, ctx);
        return;
    }
    let (area, range) = (m.pv_i(AREA), m.pv_f(RANGE));
    let found = ai::find_target(w, id, ctx, range, 0.0, area, None, false);
    w.mobys[id].set_pv_i(FOUND, found as i32);
    let m = &w.mobys[id];
    if (3 < m.state || (m.state == 3 && m.timer != 0)) && m.pv_f(SIZE) < 1.5 {
        let s = m.pv_f(SIZE) + 0.01;
        w.mobys[id].set_pv_f(SIZE, s);
        resize(w, id, s);
        for k in parts(w, id).into_iter().flatten() { resize(w, k, s); }
    }
}

/// `0x433a38(m, p, turn)`: face `p` this tick (1 the body, 2 itself).
fn face(w: &mut MobyWorld, id: usize, p: V3, turn: bool) {
    let m = &mut w.mobys[id];
    m.set_pv_s(FACING, if turn { 2 } else { 1 });
    m.set_pv_f(FACE, atan2(p[0] - m.pos[0], p[1] - m.pos[1]));
}

/// `0x433a88`: within 3.8 across.
fn close(w: &MobyWorld, id: usize, p: V3) -> bool { dist2(w.mobys[id].pos, p) < 3.8 }
/// `0x433ac8`: the distance across past its range (10; a lobber 30).
fn range(w: &MobyWorld, id: usize, p: V3) -> f32 { dist2(w.mobys[id].pos, p) - if lobs(w, id) { 30.0 } else { 10.0 } }
/// `0x433b20`: the turn from the body's facing to `p` past 90°.
fn off_body(w: &MobyWorld, id: usize, p: V3) -> f32 {
    let b = &w.mobys[link(w, id, BODY_L).unwrap_or(id)];
    rot_dist(b.rot[2], atan2(p[0] - b.pos[0], p[1] - b.pos[1])) - HALF_PI
}
/// `0x433b88`: nothing on the line from the gun's joint 0 to `p` (flags 0x16).
fn sight(w: &MobyWorld, id: usize, ctx: &Ctx, p: V3) -> bool {
    let g = link(w, id, GUN_L).unwrap_or(id);
    let from = ctx.host.joint(&w.mobys[g], 0).unwrap_or(w.mobys[g].pos);
    w.coll_line(ctx, from, p, 0x16, None).is_none()
}
/// `0x433bd8`: ready — a beam always, a lobber 150 ticks after its shot.
fn ready(w: &MobyWorld, id: usize) -> bool { !lobs(w, id) || 0x96 < w.mobys[id].pv_s(RELOAD) }
/// `0x433c08`: the body aims at `p` (within ½°; a lobber 2°).
fn aimed(w: &MobyWorld, id: usize, p: V3) -> bool {
    let m = &w.mobys[id];
    let b = &w.mobys[link(w, id, BODY_L).unwrap_or(id)];
    rot_dist(b.rot[2], atan2(p[0] - m.pos[0], p[1] - m.pos[1])) < if lobs(w, id) { 0.034_906_59 } else { 0.008_726_647 }
}
/// `0x433e00` / `0x433e50`: the walk's and the run's limits.
fn walk_senses(w: &mut MobyWorld, id: usize) { ai::senses(&mut w.mobys[id], 0.005_555_555_7, 0.053_333_34, 0.027_925_272, 0.837_758_2); }
fn run_senses(w: &mut MobyWorld, id: usize) { ai::senses(&mut w.mobys[id], 0.138_888_9, 0.183_333_34, 0.027_925_272, 0.837_758_2); }

/// `0x433ca0`: the nearest breeder of its group `+0x7b4` not gone and shown (kept in `+0x7b8`).
fn find_prey(w: &mut MobyWorld, id: usize) -> bool {
    let g = w.mobys[id].pv_i(PREY_GROUP);
    if g == -1 { return false; }
    let mut found = None;
    if g & 0xff != 0xff {
        let pos = w.mobys[id].pos;
        let mut best = 1000.0;
        for &e in usize::try_from(g).ok().and_then(|g| w.groups.get(g)).map_or(&[][..], |v| v.as_slice()) {
            let Some(o) = w.mobys.get(e) else { break };
            if o.o_class != PREY || gone(o.state) || o.mode & 1 != 0 { continue; }
            let d = dist2(pos, o.pos);
            if d < best {
                found = Some(e);
                best = d;
            }
        }
    }
    ai::set_link(&mut w.mobys[id], PREY_LINK, found);
    found.is_some()
}

/// Room at Ratchet for one more of its group.
fn room(w: &MobyWorld, id: usize) -> bool { let m = &w.mobys[id]; m.pv_s(MOST) == -1 || m.pv_s(AT_HERO) < m.pv_s(MOST) }

/// Into `s` with the state before kept.
fn keep_prev(w: &mut MobyWorld, id: usize, s: u8) {
    let m = &mut w.mobys[id];
    let p = m.prev_state;
    m.set_state(s, -1);
    m.prev_state = p;
}

fn back(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    let p = m.prev_state;
    m.set_state(p, -1);
}

/// Sequence 6 or 7 at random (blend 10).
fn rest_seq(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = ctx.game.rng.randi(2) as u8 + 6;
    seq_all(w, id, ctx, s, 0, 10);
}

/// `0x432348` (module docs).
fn states(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        2 => {
            if w.mobys[id].first() { seq_all(w, id, ctx, 1, 0, 10); }
        }
        3 => dropped(w, id, ctx),
        4 => {
            if w.mobys[id].first() {
                let s = ctx.game.rng.randi(2) as u8 + 3;
                seq_all(w, id, ctx, s, 0, 10);
                if w.mobys[id].prev_state != 4 {
                    let t = ctx.game.rng.rand_range(0, 30);
                    w.mobys[id].set_pv_i(WAKE, t);
                }
            } else if wrapped(w, id, ctx) {
                w.mobys[id].set_state(4, -1);
            } else {
                let m = &mut w.mobys[id];
                if m.pv_s(DORMANT) == 0 && m.pv_i(FOUND) == 1 {
                    let mut t = m.pv_i(WAKE);
                    let out = countdown(&mut t);
                    m.set_pv_i(WAKE, t);
                    if out {
                        m.set_state(6, -1);
                        m.set_state(5, -1);
                    }
                }
            }
        }
        5 => {
            if w.mobys[id].first() {
                seq_all(w, id, ctx, 5, 0, 10);
            } else if wrapped(w, id, ctx) {
                back(w, id);
            }
        }
        6 => {
            if w.mobys[id].first() {
                rest_seq(w, id, ctx);
                let t = ctx.game.rng.rand_range(480, 600);
                w.mobys[id].set_pv_i(PATIENCE, t);
            } else if w.mobys[id].pv_i(FOUND) == 1 {
                if room(w, id) {
                    w.mobys[id].set_state(8, -1);
                } else if find_prey(w, id) {
                    w.mobys[id].set_state(7, 1);
                }
            } else if w.mobys[id].pv_i(PATIENCE) < 1 {
                w.mobys[id].set_state(4, -1);
            }
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(PATIENCE);
            countdown(&mut t);
            m.set_pv_i(PATIENCE, t);
        }
        7 => {
            if w.mobys[id].first() && !find_prey(w, id) {
                w.mobys[id].set_state(6, -1);
                return;
            }
            let Some(e) = link(w, id, PREY_LINK) else {
                w.mobys[id].set_state(6, -1);
                return;
            };
            let p = w.mobys[e].pos;
            let go = |w: &mut MobyWorld, s: u8| {
                ai::set_link(&mut w.mobys[id], TARGET, Some(e));
                w.mobys[id].set_state(s, -1);
            };
            if 0.0 <= range(w, id, p) || 0.0 <= off_body(w, id, p) || !sight(w, id, ctx, p) {
                go(w, 9);
            } else if w.mobys[id].pv_i(FOUND) == 1 && room(w, id) {
                ai::set_link(&mut w.mobys[id], TARGET, None);
                w.mobys[id].set_state(6, -1);
            } else if close(w, id, p) {
                go(w, 0xb);
            } else if ready(w, id) {
                go(w, 10);
            }
        }
        8 => {
            if w.mobys[id].first() { rest_seq(w, id, ctx); }
            if w.mobys[id].pv_i(FOUND) != 1 {
                w.mobys[id].set_state(6, -1);
                return;
            }
            let p = w.mobys[id].pv_v(T_POINT);
            let s = if close(w, id, p) {
                0xb
            } else if 0.0 <= range(w, id, p) || 0.0 <= off_body(w, id, p) || !sight(w, id, ctx, p) {
                9
            } else if ready(w, id) {
                10
            } else {
                return;
            };
            ai::set_link(&mut w.mobys[id], TARGET, None);
            w.mobys[id].set_state(s, -1);
        }
        9 => walk(w, id, ctx),
        10 => shoot(w, id, ctx),
        11 | 12 => {
            if w.mobys[id].first() {
                let f = if w.mobys[id].state == 11 { 0 } else { 2 };
                seq_all(w, id, ctx, 0xf, f, 10);
                return;
            }
            let p = target_pos(w, id);
            face(w, id, p, true);
            let fr = ctx.host.frame_time(&w.mobys[id]);
            if 11.0 < fr && fr < 17.0 {
                let dmg = w.mobys[id].pv_b(BITE) as f32;
                ai::attack_joint(w, id, ctx, 0.8, dmg, 4.0, 2, 0x10001, 0, 1, 0);
            }
            if wrapped(w, id, ctx) { back(w, id); }
        }
        13 => hurt(w, id, ctx),
        14 => crawl(w, id, ctx),
        _ => {}
    }
}

/// The moby it goes for (`+0x7bc`), else Ratchet's point.
fn target_pos(w: &MobyWorld, id: usize) -> V3 {
    match link(w, id, TARGET) {
        Some(t) => w.mobys[t].pos,
        None => w.mobys[id].pv_v(T_POINT),
    }
}

/// State 3 (module docs).
fn dropped(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        w.mobys[id].timer = 0;
        let t = ctx.game.rng.randi(30) as i16;
        w.mobys[id].set_pv_s(DROP_WAIT, t);
        return;
    }
    if w.mobys[id].timer == 0 && (w.mobys[id].pv_s(DROP_WAIT) as i32) < ticks(w, id) as i32 {
        w.mobys[id].set_pv_i(MV + 0x88, 0);
        seq_all(w, id, ctx, 2, 0, 3);
        let m = &mut w.mobys[id];
        m.timer = 1;
        let mut v = add(scale(m.rows[0], 2.0), m.pos);
        v[2] -= 6.0;
        ai::jump_start(m, 0.033_333_335, 0.005_555_555_7, v);
    }
    match w.mobys[id].timer {
        1 => {
            let m = &mut w.mobys[id];
            approach(0.0, 0.052_359_88, &mut m.rot[0]);
            approach(0.0, 0.052_359_88, &mut m.rot[1]);
            m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) | 2);
            let carrier = link(w, id, CARRIER);
            if let Some(c) = carrier { w.mobys[c].has_collision = false; }
            if ai::jump_step(w, id, ctx, 11.0, 18.0, 27.0) {
                run_senses(w, id);
                seq_all(w, id, ctx, 9, 0, 10);
                w.mobys[id].set_pv_f(MV + 0x34, 0.2);
                let spot = w.mobys[id].pv_i(SPOT);
                match ctx.level.cuboid_place(spot).filter(|_| spot != -1) {
                    Some((p, _)) => {
                        w.mobys[id].timer = 2;
                        w.mobys[id].set_pv_v(GOAL, p);
                    }
                    None => w.mobys[id].set_state(4, 1),
                }
            }
            let m = &mut w.mobys[id];
            m.set_pv_i(MV + 0x38, m.pv_i(MV + 0x38) & !2);
            if let Some(c) = carrier { w.mobys[c].has_collision = class_coll(w, c); }
        }
        2 => {
            let spot = w.mobys[id].pv_i(SPOT);
            let p = ctx.level.cuboid_place(spot).map_or(w.mobys[id].pos, |c| c.0);
            if dist2(w.mobys[id].pos, p) < 0.5 { w.mobys[id].set_state(4, 1); }
        }
        _ => {}
    }
}

/// State 9 (module docs).
fn walk(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        w.mobys[id].set_pv_f(MV + 0x34, 2.0);
        match seq_a(w, id, ctx) {
            8 => {
                walk_senses(w, id);
                w.mobys[id].timer = 1;
            }
            9 => {
                run_senses(w, id);
                w.mobys[id].timer = 2;
            }
            _ => {
                walk_senses(w, id);
                w.mobys[id].timer = 1;
                seq_all(w, id, ctx, 8, 0, 10);
            }
        }
    }
    match link(w, id, TARGET) {
        Some(t) => {
            let o = &w.mobys[t];
            if !gone(o.state) && o.mode & 1 == 0 {
                let p = o.pos;
                w.mobys[id].set_pv_v(GOAL, p);
            } else {
                back(w, id);
            }
        }
        None => {
            let p = w.mobys[id].pv_v(T_POINT);
            w.mobys[id].set_pv_v(GOAL, p);
        }
    }
    let g = w.mobys[id].pv_v(GOAL);
    face(w, id, g, false);
    let fr = ctx.host.frame_time(&w.mobys[id]);
    match w.mobys[id].timer {
        2 => {
            if range(w, id, g) < 4.0 && wrapped(w, id, ctx) {
                walk_senses(w, id);
                w.mobys[id].timer = 1;
                seq_all(w, id, ctx, 8, 0, 10);
            }
        }
        1 => {
            if 7.0 < range(w, id, g) && 4.0 < fr && fr < 9.0 {
                run_senses(w, id);
                w.mobys[id].timer = 2;
                seq_all(w, id, ctx, 9, 6, 10);
            }
            if ((4.0 < fr && fr < 9.0) || (21.0 < fr && fr < 26.0)) && in_reach(w, id, ctx, g) {
                rest_seq(w, id, ctx);
                back(w, id);
            }
        }
        _ => {
            if wrapped(w, id, ctx) && 0x78 < ticks(w, id) {
                walk_senses(w, id);
                w.mobys[id].timer = 1;
                seq_all(w, id, ctx, 8, 0, 10);
            }
            if in_reach(w, id, ctx, g) {
                rest_seq(w, id, ctx);
                back(w, id);
            }
        }
    }
    let m = &w.mobys[id];
    if matches!(m.timer, 1 | 2) && (2 <= m.pv_i(MV + 0x80) || m.pv_i(MV + 0x94) & 0xf != 0) {
        w.mobys[id].timer = 0;
        rest_seq(w, id, ctx);
        w.mobys[id].state_ticks = 0;
    }
    if w.mobys[id].prev_state == 7 && w.mobys[id].pv_i(FOUND) == 1 && room(w, id) {
        ai::set_link(&mut w.mobys[id], TARGET, None);
        w.mobys[id].set_state(8, -1);
    }
}

/// In range, within 90° of the body's facing and in the gun's sight.
fn in_reach(w: &MobyWorld, id: usize, ctx: &Ctx, g: V3) -> bool { range(w, id, g) < 0.0 && off_body(w, id, g) < 0.0 && sight(w, id, ctx, g) }

/// The shot's sequence: 0xc the lob, 0xd the beam (speed 1).
fn shot_seq(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let s = if lobs(w, id) { 0xc } else { 0xd };
    seq_all(w, id, ctx, s, 0, 10);
    set_speed(w, id, ctx, 1.0);
}

/// State 10 (module docs).
fn shoot(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        seq_all(w, id, ctx, 10, 0, 10);
        w.mobys[id].timer = 0;
        set_speed(w, id, ctx, 2.0);
        if lobs(w, id) { w.mobys[id].set_pv_s(BEAM, 0); }
        return;
    }
    let v = target_pos(w, id);
    face(w, id, v, false);
    if w.mobys[id].timer == 0 {
        if close(w, id, v) {
            keep_prev(w, id, 0xb);
            return;
        }
        if wrapped(w, id, ctx) {
            if aimed(w, id, v) {
                w.mobys[id].timer = 2;
                shot_seq(w, id, ctx);
            } else {
                w.mobys[id].timer = 1;
                seq_all(w, id, ctx, 0xb, 0, 10);
                set_speed(w, id, ctx, 1.0);
            }
        }
    }
    match w.mobys[id].timer {
        1 => {
            if close(w, id, v) {
                keep_prev(w, id, 0xb);
                return;
            }
            if aimed(w, id, v) {
                w.mobys[id].timer = 2;
                shot_seq(w, id, ctx);
            }
        }
        2 => {
            let fr = ctx.host.frame_time(&w.mobys[id]);
            if lobs(w, id) {
                if fr < 2.0 && w.mobys[id].pv_s(BEAM) == 0 {
                    w.mobys[id].set_pv_v(AIM, v);
                    lob(w, id, ctx);
                    w.mobys[id].set_pv_s(BEAM, 1);
                }
            } else if fr < 5.0 && w.mobys[id].pv_s(BEAM) == 0 {
                // `0x434920`: the beam's start.
                w.mobys[id].set_pv_s(RELOAD, 0);
                w.mobys[id].set_pv_s(BEAM, 1);
            }
            if !wrapped(w, id, ctx) { return; }
            if link(w, id, TARGET).is_none() {
                if close(w, id, v) {
                    keep_prev(w, id, 0xb);
                    return;
                }
                if ready(w, id) && range(w, id, v) < 0.0 && aimed(w, id, v) {
                    w.mobys[id].timer = 2;
                    shot_seq(w, id, ctx);
                }
            }
            w.mobys[id].timer = 3;
            seq_all(w, id, ctx, 0xe, 0, 10);
            set_speed(w, id, ctx, 1.0);
        }
        3 => {
            if wrapped(w, id, ctx) {
                seq_all(w, id, ctx, 8, 0, 10);
                back(w, id);
                set_speed(w, id, ctx, 1.0);
            }
        }
        _ => {}
    }
}

/// `0x4346c0`: the lob — a lobber's reload from 0 and its class sound 7 (the spit itself is not ported: module docs).
fn lob(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !lobs(w, id) { return; }
    w.mobys[id].set_pv_s(RELOAD, 0);
    ctx.host.play_sound(&w.mobys[id], 7, 0);
}

/// State 13 (module docs).
fn hurt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        // `0x434940`: the beam put back.
        let m = &mut w.mobys[id];
        m.set_pv_s(RELOAD, 0);
        m.set_pv_s(BEAM, 0);
        m.set_pv_s(HURT_ONCE, 1);
    }
    let flags = ai::flight(w, id, ctx);
    let fr = ctx.host.frame_time(&w.mobys[id]);
    let sa = seq_a(w, id, ctx);
    if 0xb5 <= ticks(w, id) || flags & 1 != 0 {
        drop_and_die(w, id, ctx);
        return;
    }
    if (sa == 0x10 && 27.0 < fr) || (sa == 0x11 && 47.0 < fr) || flags & 2 != 0 {
        if link(w, id, BODY_L).is_some() {
            let p = w.mobys[id].pv_v(T_POINT);
            if close(w, id, p) {
                set_speed(w, id, ctx, 0.0);
                ai::set_link(&mut w.mobys[id], TARGET, None);
                w.mobys[id].set_state(8, 1);
                w.mobys[id].set_state(0xb, -1);
            } else {
                rest_seq(w, id, ctx);
                w.mobys[id].set_state(8, 1);
            }
            return;
        }
        w.mobys[id].set_state(0xe, 1);
    }
    let m = &w.mobys[id];
    let knocked = ai::link(m, HITTER).and_then(|k| w.mobys.get(k)).is_some_and(|k| k.o_class == KNOCKER);
    if m.pv_s(HURT_ONCE) != 0 && knocked && link(w, id, BODY_L).is_some() && sa == 0x10 && 13.0 < fr {
        set_speed(w, id, ctx, 0.0);
        ai::set_link(&mut w.mobys[id], TARGET, None);
        w.mobys[id].set_state(8, 1);
        w.mobys[id].set_state(0xc, -1);
    }
}

/// State 14 (module docs).
fn crawl(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        if w.mobys[id].pv_s(CRAWL) == 0 { w.mobys[id].set_pv_s(CRAWL, 600); }
        if let Some(b) = link(w, id, BODY_L) {
            let at = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
            blast::burst(w, ctx, at, &Blast { size: 0.5, bursts: [10, 0x1e, 0], puffs: 10, streaks: 0x2d, light: true, shake: true });
            w.delete(b);
            ai::set_link(&mut w.mobys[id], BODY_L, None);
        }
        for off in [INNER_L, GUN_L] {
            if let Some(k) = link(w, id, off) {
                w.delete(k);
                ai::set_link(&mut w.mobys[id], off, None);
            }
        }
        set_seq(w, id, ctx, 8, 0, 10);
        return;
    }
    let p = w.mobys[id].pv_v(T_POINT);
    w.mobys[id].set_pv_v(GOAL, p);
    if close(w, id, p) {
        w.mobys[id].set_state(0xb, -1);
        return;
    }
    let mut t = w.mobys[id].pv_s(CRAWL) as i32;
    let out = countdown(&mut t);
    w.mobys[id].set_pv_s(CRAWL, t as i16);
    if out { drop_and_die(w, id, ctx); }
}

/// The body turned toward `yaw` (`0x33a350(yaw, 0.4°, 0.4°, 12°, body yaw, +0x7cc, 0)`).
fn turn_body(w: &mut MobyWorld, id: usize, yaw: f32) {
    let Some(b) = link(w, id, BODY_L) else { return };
    let mut s = w.mobys[id].pv_f(BODY_TURN);
    let mut r = w.mobys[b].rot[2];
    turn_ease(yaw, BODY_ACCEL, BODY_ACCEL, BODY_MOST, &mut r, &mut s, 0);
    w.mobys[b].rot[2] = r;
    w.mobys[id].set_pv_f(BODY_TURN, s);
}

/// `0x433560` (module docs).
fn movement(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let mut s = seq_a(w, id, ctx);
    if s == 0xff { s = seq(w, id, ctx); }
    let (face_ask, face_yaw, yaw) = { let m = &w.mobys[id]; (m.pv_s(FACING), m.pv_f(FACE), m.rot[2]) };
    match s {
        1 => {
            if let Some(b) = link(w, id, BODY_L) { w.mobys[b].rot = w.mobys[id].rot; }
        }
        3..=7 | 10..=15 => {
            if face_ask == 1 {
                if rot_dist(face_yaw, yaw) <= BODY_REACH {
                    turn_body(w, id, face_yaw);
                    ai::hold(w, id, ctx);
                } else {
                    ai::turn_in_place(w, id, ctx, face_yaw);
                }
                return;
            }
            if face_ask == 2 { ai::turn_in_place(w, id, ctx, face_yaw); } else { ai::hold(w, id, ctx); }
            let y = w.mobys[id].rot[2];
            turn_body(w, id, y);
        }
        8 | 9 => {
            let goal = w.mobys[id].pv_v(GOAL);
            if w.mobys[id].pv_i(NET) == -1 {
                let off = w.mobys[id].pv_f(WALK_OFF);
                ai::move_toward(w, id, ctx, off, goal);
            } else {
                let m = &w.mobys[id];
                let n = (m.pv_i(N_NODES).max(0) as usize).min(NODES_MAX);
                let nodes = crate::path_network::read_nodes(&m.pvars, NODES, n);
                let (mut y, mut at, mut to) = (m.pv_f(NET_YAW), m.pv_i(NET_AT), m.pv_i(NET_GOAL));
                crate::path_network::steer(ctx.level, &nodes, 0.4, m.pos, goal, &mut y, &mut at, &mut to);
                let m = &mut w.mobys[id];
                m.set_pv_f(NET_YAW, y);
                m.set_pv_i(NET_AT, at);
                m.set_pv_i(NET_GOAL, to);
                ai::move_step(w, id, ctx, y, goal);
            }
            if link(w, id, BODY_L).is_some() {
                let yaw = w.mobys[id].rot[2];
                let t = if face_ask == 1 && rot_dist(face_yaw, yaw) < BODY_REACH { face_yaw } else { yaw };
                turn_body(w, id, t);
            }
        }
        _ => {}
    }
}

/// `0x433800` (module docs).
fn tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
    let r = m.pv_s(RELOAD);
    if r < 0x7fff { m.set_pv_s(RELOAD, r + 1); }
    if let Some(b) = link(w, id, BODY_L) {
        let at = ctx.host.joint(&w.mobys[id], 0).unwrap_or(w.mobys[id].pos);
        w.mobys[b].pos = at;
        let sp = speed_of(w, id, ctx);
        set_speed(w, b, ctx, sp);
        if let Some(g) = link(w, id, GUN_L) { place_part(w, b, g, 2, ctx); }
        if let Some(k) = link(w, id, INNER_L) {
            set_speed(w, k, ctx, sp);
            let at = ctx.host.joint(&w.mobys[b], 0).unwrap_or(w.mobys[b].pos);
            let rot = w.mobys[b].rot;
            w.mobys[k].pos = at;
            w.mobys[k].rot = rot;
            ctx.host.advance(&w.mobys[k]);
            w.rebuild(k, &*ctx.host);
        }
    }
    let st = w.mobys[id].state;
    if 2 <= st && !gone(st) && !lobs(w, id) && link(w, id, GUN_L).is_some() { beam(w, id, ctx); }
    if w.mobys[id].drawn != 0 { crate::shadow_volumes::probe_near(w, id, ctx, 24.0, 0x16); }
    w.mobys[id].set_pv_s(FACING, 0);
    // `0x341e28` = L00 `0x327308`: the head aim, on the inner body's list 5.
    if let Some(k) = link(w, id, INNER_L) { crate::moby::joint_aim::head_on(w, ctx, id, HEAD_AIM, k, 5, 2.5); }
}

/// `0x434950` (module docs).
fn beam(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(g) = link(w, id, GUN_L) else { return };
    let a = ctx.host.joint(&w.mobys[g], 0).unwrap_or(w.mobys[g].pos);
    let b = ctx.host.joint(&w.mobys[g], 1).unwrap_or(a);
    w.mobys[id].set_pv_f(REACH, 0.0);
    let n = w.mobys[id].pv_s(RELOAD);
    let dmg = w.mobys[id].pv_b(DAMAGE) as f32;
    let t = n as f32 / 5.0;
    let lerp = |p: V3, q: V3| -> V3 { std::array::from_fn(|k| p[k] + (q[k] - p[k]) * t) };
    match w.mobys[id].pv_s(BEAM) {
        0 => {
            let m = &mut w.mobys[id];
            m.set_pv_v(ROOT, a);
            m.set_pv_v(TIP, b);
            m.set_pv_v(ROOT2, b);
            m.set_pv_v(TIP2, a);
            stop_voice(w, id, ctx);
        }
        1 => {
            let m = &mut w.mobys[id];
            m.set_pv_v(ROOT, a);
            m.set_pv_v(TIP, b);
            m.set_pv_v(ROOT2, b);
            m.set_pv_v(TIP2, a);
            ai::attack_sphere(w, id, ctx, 0.2, dmg, 1.0, a, 0x10001, 0, 1, 0);
            if n < 6 {
                w.mobys[id].set_pv_f(REACH, t);
            } else {
                waves(w, id, ctx, 2.0, 5.0);
                let m = &mut w.mobys[id];
                m.set_pv_s(RELOAD, 0);
                m.set_pv_s(BEAM, 2);
                if !ctx.host.sound_playing(m.pv_i(VOICE)) {
                    let h = ctx.host.play_sound(&w.mobys[id], 1, 4);
                    w.mobys[id].set_pv_i(VOICE, h);
                }
                let p = target_pos(w, id);
                w.mobys[id].set_pv_v(AIM, p);
            }
        }
        2 => {
            w.mobys[id].set_pv_v(ROOT, a);
            w.mobys[id].set_pv_v(ROOT2, b);
            ai::attack_sphere(w, id, ctx, 0.2, dmg, 1.0, a, 0x10001, 0, 1, 0);
            if n < 6 {
                let tip = lerp(a, w.mobys[id].pv_v(AIM));
                w.mobys[id].set_pv_v(TIP, tip);
                w.mobys[id].set_pv_v(TIP2, tip);
                let d = scale(sub(tip, a), 0.033_333_335);
                let mut tm = HitTemplate::new(dmg, Some(id), 0x210001, [d[0], d[1], d[2], 0.0]);
                tm.kind = (w.mobys[id].o_class as u16 as u32) << 16 | 3 << 8 | 5;
                if let Some(o) = w.coll_line(ctx, a, tip, 0, Some(id)) {
                    w.mobys[id].set_pv_v(AIM, tip);
                    w.mobys[id].set_pv_s(RELOAD, 0);
                    match o.moby {
                        Some(k) => {
                            w.deliver_hit_any(k, &tm, [o.point[0], o.point[1], o.point[2], 1.0]);
                            let kp = w.mobys[k].pos;
                            let m = &mut w.mobys[id];
                            m.set_pv_s(BEAM, 3);
                            ai::set_link(m, HELD, Some(k));
                            m.set_pv_v(TIP, [o.point[0], o.point[1], o.point[2]]);
                            m.set_pv_v(HELD_AT, kp);
                        }
                        None => w.mobys[id].set_pv_s(BEAM, 4),
                    }
                }
            } else {
                w.mobys[id].set_pv_s(BEAM, 4);
            }
        }
        3 => {
            w.mobys[id].set_pv_v(ROOT, a);
            w.mobys[id].set_pv_v(ROOT2, b);
            match link(w, id, HELD).filter(|&k| n < 0x33 && !gone(w.mobys[k].state)) {
                Some(k) => {
                    let kp = w.mobys[k].pos;
                    let m = &mut w.mobys[id];
                    let tip = add(m.pv_v(TIP), sub(kp, m.pv_v(HELD_AT)));
                    m.set_pv_v(TIP, tip);
                    m.set_pv_v(TIP2, tip);
                    m.set_pv_v(HELD_AT, kp);
                }
                None => {
                    let m = &mut w.mobys[id];
                    m.set_pv_s(RELOAD, 0);
                    m.set_pv_s(BEAM, 4);
                    ai::set_link(m, HELD, None);
                }
            }
        }
        4 => {
            w.mobys[id].set_pv_v(ROOT, a);
            w.mobys[id].set_pv_v(ROOT2, b);
            if n < 5 {
                let aim = w.mobys[id].pv_v(AIM);
                let (tip, tip2) = (lerp(aim, a), lerp(aim, b));
                w.mobys[id].set_pv_v(TIP, tip);
                w.mobys[id].set_pv_v(TIP2, tip2);
            } else {
                w.mobys[id].set_pv_s(RELOAD, 0);
                w.mobys[id].set_pv_s(BEAM, 0);
                waves(w, id, ctx, 1.0, 1.7);
            }
        }
        _ => {}
    }
}
