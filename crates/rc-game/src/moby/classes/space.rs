//! The space fights' enemies (L25 Wupash Nebula; the space-AI library `0x374b10..0x378be0` all the ship levels
//! share): the fighter 0x0957 (`0x3ca118`) and the gunship 0x097c (`0x3cb770`).
//!
//! **The AI block** (the moby's `+0x68`[`0x14`] with mode 0x20; kept here in the pvars from [`AI`], past the classes'
//! own): `+0` / `+0x10` a formation's offset and leader, `+0x60` the goal, `+0x70` a run's offset (on with `+0xfc`),
//! `+0x90` the velocity a tick, `+0xd0` the last push's face, `+0x30` / `+0x34` / `+0x38` the roll, pitch and yaw,
//! `+0x100` the state, `+0x104` in the gun's cone, `+0x108` in range, `+0x110` active, `+0x114` the attack kind,
//! `+0x118` attacking, `+0x11c` with its own target, `+0x120` the target, `+0x124` the speed, `+0x130` the goal's
//! timer, `+0x15c` the pushes off faces in a row.
//!
//! **A tick** (`0x377278`): the step (`+0x90`, not in the circling 8); kept inside the fight's sphere (`0x374f50`, not
//! in state 2: the mission object's `+0x68`[0], radius 256 round (512, 512, 512) on Wupash, less its own size), and
//! pushed off faces more than 6 ticks in a row its velocity bounces off the last one with sparks; the push
//! (`0x377a58`: toward the goal at the state's pull, then sped up below half the state's top speed and slowed above
//! it, at most pull · top); the top speed (`0x377e30`); the angles from the velocity (`0x377820`: the yaw and pitch
//! along it, the roll leaning into the turn by `0.98·roll + turned·pull'·1.41·8`, at most 81°); then the state. A
//! goal set is checked (`0x378570`, not in the states 2 and 0xc): inside the sphere, the line to it clear (else 5 out
//! from the hit), a sphere of 5 there clear (else pushed out).
//! * 1 wandering: at the goal (within 1) or its timer out, a new one (`0x3783a0`): a random point of the sphere
//!   across, ±64 up from it, 300 ticks.
//! * 3 in a formation: its leader's place plus its offset in the leader's frame.
//! * 4 closing (25 ahead of the target), 5 on its tail (5 ahead), 6 (10 ahead, 0.05 up), each plus the run's offset in
//!   the target's frame; 4 there → 5 for 300..600 ticks; 5 and 6 there or the timer out → the attack's end
//!   (`0x376950`), 1.
//! * 7 behind the target (50); there → 8, circling it (the gunship's attack); something met within 1.5 → 9; its time
//!   out, or in 9, off screen (`0x302b78`) → the attack's end.
//! * 0xd out of a no-fly volume: 8 ahead, again at each arrival. 2, 0xb, 0xc: their tuning only.
//!
//! The state's pull, push and top speed (`0x377bd0`, `$gp−0x2ab0..`): 1 and 0xd 0.02 / 0.1 / 0.4, 3 0.02 / 0.1 / 0.417,
//! 4 0.5 top, 5 0.04 / 0.2 / 0.417, 6 0.03 / 0.15 / 0.367, 7 0.167 top, 8 0.417 top; in range of the target 0.7 of
//! the pull and push.
//!
//! **The fighter** (`0x3ca118`): its start (`0x3ca4a8`, `0x376068`): active, health 1, scale 0.06, at a random
//! point of the sphere, wandering. Each tick: the AI tick; its push and hits (`0x376650`: a sphere of its size
//! against the world and the mobys puts it out of whatever it meets — the player's ship too — then the mission's
//! handler `0x40f270`: a hit with the flags 0x230000 takes its damage from the health; at 0 it bursts and is put
//! away, `0x375fe8`: hidden, no collision, not active); the range test (`0x376a60`: the target within 176 with a
//! clear line → in range; within 40.5° across and up of its facing → in the gun's cone); its own target (disc
//! `+0x1b0`: the player's ship on Wupash, so `+0x11c`) and the fly-by sound within 10 of it; the attack's start
//! (`0x3769a0` / `0x3767e8`: not in the quiet, of the tick's leading group ([`leader`]), not attacking while fewer
//! than 4 attack (`0x221820`), → 4 with a timer of 120..500 and its run's offset ±1 across and ±1 up).
//!
//! Its gun (`0x3ca5a8`): in range and in the cone, every 20 ticks a shot ([`crate::ship::spawn_enemy_shot`]).
//!
//! Its death (`0x40f270`): the blast `0x4184c8` ([`crate::moby::blast::burst`]) and a drop ([`drop`], the pickups'
//! [`update_pickup`]).
//!
//! **The other ship levels** (Feltzin 5, Hrugis 10, Gorn 15: the same library, the decomp's `func_L05_0037…` /
//! `func_002ED688`.., at L05 `0x37aad0..`, L10 `0x37c8e0..`, L15 `0x3812f8..`): their ships are the fighter's pattern
//! with their own numbers ([`Spec`], the levels' modules: crate::moby::classes::level05 / level10). Each level's
//! mission object (L05 0x1103 `0x41efc0`, L10 0x1155 `0x422828`, L15 0x119f `0x42f288`; not ported) sets the
//! library's globals at its start: the fight's sphere (its `+0x68`[0], read here from the placed mission moby, as
//! [`arena`]), the ready flag `0x221ca4` (L10; `0x37c8e0` returns it: taken as set here, as on Wupash) and the hit
//! handler `0x221cd4` that `0x37e600` (the push) calls ([`hits`]: L05 `0x421c18`, L10 `0x4252b0`, L15 `0x431ad8`).
//! The quiet and the tick's leading group are the Wupash mission's alone here (their missions keep them).
//!
//! Not ported: the exhaust trails (`0x3ca6c0` / `0x3cbc40` through `0x349c48`); the no-fly volumes' check
//! (`0x3782c8`: none on Wupash) and the formations' set-up and break-up (`0x376d20`, `0x376e58`: none form on
//! Wupash; state 3 itself is); the moby groups (`+0x21`: the leaders' groups are their classes here); the view test
//! `0x302b78` is the game's 4:3 frustum by its tangents, not its planes `0x1bd900..`.

use crate::hero::physics::{add, atan2, dist, len, len2, rot_dist, scale, set_len, sub, sub_rot, V3};
use crate::moby::{mode, Ctx, MobyWorld};

/// The fighter's class.
pub const FIGHTER: i16 = 0x0957;

/// The port's own words, past the classes' disc pvars (the gunship's reach `+0x2f8`): the health, started, a wave's
/// return asked (`0x376428`: gather round `REVIVE_AT`, go for `REVIVE_GOAL`), the engine's and the fly-by's sound
/// handles + 1, and the AI block (`+0x68`[`0x14`], 0x160 bytes).
pub(crate) const HEALTH: usize = 0x400;
pub(crate) const STARTED: usize = 0x404;
const REVIVE: usize = 0x408;
const ENGINE: usize = 0x40c;
const FLYBY: usize = 0x410;
const REVIVE_AT: usize = 0x420;
const REVIVE_GOAL: usize = 0x430;
const AI: usize = 0x440;
/// The mission object's words the flight keeps (`0x22182c` the quiet, `0x221810` the active count).
pub const PASSIVE: usize = 0x100;
pub const ACTIVE_COUNT: usize = 0x104;
/// The group that may start attacks this tick (`0x2258c0[$gp−0x2a94]`, the class).
pub const LEADER: usize = 0x108;

/// The AI block's words.
pub(crate) mod a {
    pub const GOAL: usize = 0x60;
    pub const VEL: usize = 0x90;
    pub const ROLL: usize = 0x30;
    pub const PITCH: usize = 0x34;
    pub const YAW: usize = 0x38;
    pub const STATE: usize = 0x100;
    pub const CONE: usize = 0x104;
    pub const RANGE: usize = 0x108;
    pub const ACTIVE: usize = 0x110;
    pub const ATTACKING: usize = 0x118;
    pub const TARGET: usize = 0x120;
    pub const SPEED: usize = 0x124;
    pub const TIMER: usize = 0x130;
    /// The circling's angle, side and radius (`+0x134`, `+0x140`, `+0x144`); the attack's kind (`+0x114`).
    pub const CIRCLE_ANGLE: usize = 0x134;
    pub const CIRCLE_DIR: usize = 0x140;
    pub const CIRCLE_RADIUS: usize = 0x144;
    pub const KIND: usize = 0x114;
    /// The run's offset in the target's frame (`+0x70`, on with `+0xfc`), given when the ship has its own target
    /// (`+0x11c`).
    pub const OFFSET: usize = 0x70;
    pub const OFFSET_ON: usize = 0xfc;
    pub const OWN_TARGET: usize = 0x11c;
    /// The last push's face normal (`+0xd0`) and the pushes in a row off faces (`+0x15c`).
    pub const PUSH_NORMAL: usize = 0xd0;
    pub const PUSHES: usize = 0x15c;
}

pub(crate) fn ai_i(w: &MobyWorld, id: usize, o: usize) -> i32 { w.mobys[id].pv_i(AI + o) }
pub(crate) fn ai_f(w: &MobyWorld, id: usize, o: usize) -> f32 { w.mobys[id].pv_f(AI + o) }
fn ai_v(w: &MobyWorld, id: usize, o: usize) -> V3 { w.mobys[id].pv_v(AI + o) }
pub(crate) fn set_ai_i(w: &mut MobyWorld, id: usize, o: usize, v: i32) { w.mobys[id].set_pv_i(AI + o, v) }
pub(crate) fn set_ai_f(w: &mut MobyWorld, id: usize, o: usize, v: f32) { w.mobys[id].set_pv_f(AI + o, v) }
fn set_ai_v(w: &mut MobyWorld, id: usize, o: usize, v: V3) { w.mobys[id].set_pv_v(AI + o, v) }

/// The levels' mission objects whose `+0x68`[0] is the fight's sphere (Wupash 0x0eee; Feltzin 0x1103, Hrugis
/// 0x1155, Gorn 0x119f: their `0x221cc0` / `0x221c80` / `0x221dc0`).
const MISSIONS: [i16; 4] = [0x0eee, 0x1103, 0x1155, 0x119f];

/// The fight's sphere (`0x1b2114 + 0x221800·0x80`: the mission object's `+0x68`[0]): its centre and radius.
fn arena(w: &MobyWorld, ctx: &Ctx) -> (V3, f32) {
    let i = w.mobys.iter().find(|m| MISSIONS.contains(&m.o_class)).map_or(0, |m| m.pv_i(0));
    let Some(v) = usize::try_from(i).ok().and_then(|i| ctx.level.spheres.get(i)) else { return ([512.0; 3], 256.0) };
    let m = &v.matrix;
    ([m[3][0], m[3][1], m[3][2]], len([m[0][0], m[0][1], m[0][2]]))
}

/// The target (`+0x120`, index + 1).
fn target(w: &MobyWorld, id: usize) -> Option<usize> {
    usize::try_from(ai_i(w, id, a::TARGET) - 1).ok().filter(|&t| w.mobys.get(t).is_some_and(|m| m.state < 0x80))
}

/// `0x377bd0`: the state's pull, push and top speed (a tick).
fn tuning(state: i32, in_range: bool) -> (f32, f32, f32) {
    let (mut k, mut kd, top) = match state {
        2 => (0.005, 0.025, 0.133_333_34),
        3 => (0.02, 0.1, 0.416_666_7),
        4 | 0xb => (0.02, 0.1, 0.5),
        5 => (0.04, 0.2, 0.416_666_7),
        6 => (0.03, 0.15, 0.366_666_67),
        7 => (0.02, 0.1, 0.166_666_67),
        8 => (0.02, 0.1, 0.416_666_7),
        _ => (0.02, 0.1, 0.4),
    };
    if in_range && !matches!(state, 0xb | 2 | 0xc) {
        k *= 0.7;
        kd *= 0.7;
    }
    (k, kd, top)
}

/// `0x378508`: the goal set, then checked ([`check_goal`]) but in the states 2 and 0xc.
fn set_goal(w: &mut MobyWorld, id: usize, ctx: &Ctx, goal: V3) {
    set_ai_v(w, id, a::GOAL, goal);
    if !matches!(ai_i(w, id, a::STATE), 2 | 0xc) { check_goal(w, id, ctx); }
}

/// `0x378570`: the goal kept inside the sphere; then the line to it (`0x2f1998`, flags 2: no mobys' primitives) met
/// → 5 out from the hit along its face; then a sphere of 5 there (`0x2f29f8`, flags 2) met → pushed out of it.
fn check_goal(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let (c, r) = arena(w, ctx);
    let mut goal = ai_v(w, id, a::GOAL);
    let d = sub(goal, c);
    if r < len(d) { goal = add(c, set_len(d, r)); }
    if let Some(coll) = ctx.coll {
        let flags = crate::collision::QueryFlags(2);
        if let Some(h) = crate::collision::coll_line_m(coll, ctx.scene, w.mobys[id].pos, goal, flags, Some(id)) {
            goal = add(h.point, set_len(h.normal, 5.0));
        }
        if let Some(p) = crate::collision::coll_sphere_m(coll, ctx.scene, goal, 5.0, flags, Some(id)).and_then(|h| h.pushed_centre) {
            goal = p;
        }
    }
    set_ai_v(w, id, a::GOAL, goal);
}

/// `0x3783a0`: a random goal in the sphere, ±64 up from the fighter, for 300 ticks.
fn wander_goal(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (c, r) = arena(w, ctx);
    let g = &mut ctx.game.rng;
    let x = g.rand_angle().cos() * g.randf(0.0, r);
    let y = g.rand_angle().sin() * g.randf(0.0, r);
    let z = g.randf(-64.0, 64.0) + w.mobys[id].pos[2];
    set_ai_v(w, id, a::GOAL, [c[0] + x, c[1] + y, z]);
    check_goal(w, id, ctx);
    set_ai_i(w, id, a::TIMER, 300);
}

/// `0x378498`: a goal 8 ahead along its facing (checked), for 300 ticks.
fn ahead_goal(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let goal = add(w.mobys[id].pos, scale(w.mobys[id].rows[0], 8.0));
    set_ai_v(w, id, a::GOAL, goal);
    check_goal(w, id, ctx);
    set_ai_i(w, id, a::TIMER, 300);
}

/// `0x3786e8`: at the goal (within 1; 8 in state 2), or the goal's timer out.
fn arrived(w: &mut MobyWorld, id: usize) -> bool {
    let state = ai_i(w, id, a::STATE);
    let near = if state == 2 { 8.0 } else { 1.0 };
    let at = dist(w.mobys[id].pos, ai_v(w, id, a::GOAL)) < near && state != 8;
    let mut t = ai_i(w, id, a::TIMER);
    let out = crate::moby::countdown(&mut t);
    set_ai_i(w, id, a::TIMER, t);
    at || out
}

/// `0x376950`: the attack over.
fn end_attack(w: &mut MobyWorld, id: usize) {
    set_ai_i(w, id, a::ATTACKING, 0);
    set_ai_i(w, id, a::STATE, 1);
}

/// The gunship's class (`0x3cb770`).
pub const GUNSHIP: i16 = 0x097c;

/// What sets the two apart (the fighter `0x3ca4a8` / `0x3ca5a8`, the gunship `0x3cba68` / `0x3cbb78`): the health
/// (`+0x148`), the scale (`+0x2c`), the range (`0x376a60`'s), the gun's wait, its shot's hit radius, speed and colour
/// (`0x375198`'s), and whether it may take the circling attack (kind 5).
/// The disc pvars: its own target (a moby index, −1 none: `+0x1b0` / `+0x260`), its entry in the leaders' list
/// (`+0x1b4` / `+0x264`), the gun's wait (`+0x1b8` / `+0x268`); and the fly-by's odds (`0x32e060(400)` / `(750)`).
/// The other ship levels' classes add: placed at random in the sphere at the start or left where they are (both then
/// wander), the range test's screen check (`0x376a60`'s last argument: in range only with a sphere of 0.1 there on
/// screen), the gun asking for the range or the cone alone, the aim's lead ahead of the target (`0x375198`'s second
/// argument), the flame instead of the shot (Feltzin's 0x10f5), the sounds (the engine's loop, the death's, the
/// gun's: one shot in three or two in three), and the exhaust's nozzles, puffs' speed (`(a..b)/60 · k` back),
/// descriptor (crate::particles::EXHAUST_DESCS) and whether only on screen.
#[derive(Clone, Copy)]
pub(crate) struct Spec {
    pub health: f32,
    pub scale: f32,
    pub scatter: bool,
    pub range: f32,
    pub on_screen: bool,
    pub fire_in_range: bool,
    pub lead: f32,
    pub flame: bool,
    pub wait: i32,
    pub shot: (f32, f32, u32),
    /// One shot in two (`0x336448(2) == 0`) this colour instead (Hrugis's 0xf93).
    pub shot_alt: Option<u32>,
    pub circles: bool,
    pub target: usize,
    pub leads: usize,
    pub gun: usize,
    /// The fly-by's odds (0: none).
    pub flyby: i32,
    pub engine: Option<(i16, i32)>,
    pub death: Option<(i16, i32)>,
    pub gun_sound: Option<(i16, i32, bool)>,
    pub nozzles: &'static [V3],
    pub puff: (f32, f32, f32),
    pub desc: usize,
    pub exhaust_on_screen: bool,
}

/// The Wupash fighter's numbers, the others' starting point.
pub(crate) const FIGHTER_SPEC: Spec = Spec {
    health: 1.0,
    scale: 0.06,
    scatter: true,
    range: 176.0,
    on_screen: false,
    fire_in_range: true,
    lead: 2.0,
    flame: false,
    wait: 0x14,
    shot: (1.0, 1.0, 0x6000_8000),
    shot_alt: None,
    circles: false,
    target: 0x1b0,
    leads: 0x1b4,
    gun: 0x1b8,
    flyby: 400,
    engine: Some((FIGHTER, 3)),
    death: Some((FIGHTER, 1)),
    gun_sound: Some((FIGHTER, 0, false)),
    nozzles: &[[-0.3, 0.0, 0.0]],
    puff: (30.0, 40.0, 0.4),
    desc: 0,
    exhaust_on_screen: true,
};

/// The other levels' ships ([`Spec`]).
fn level_spec(class: i16) -> Option<Spec> { super::level05::ship_spec(class).or_else(|| super::level10::ship_spec(class)) }

fn spec(class: i16) -> Spec {
    match class {
        GUNSHIP => Spec { health: 5.0, scale: 0.04, range: 32.0, wait: 0x3c, shot: (0.6, 1.3, 0x6000_0090), circles: true, target: 0x260, leads: 0x264, gun: 0x268, flyby: 0x2ee, engine: Some((GUNSHIP, 1)), death: None, gun_sound: Some((GUNSHIP, 0, false)), nozzles: &[[-0.21, 0.061, 0.045], [-0.21, -0.061, 0.045]], desc: 3, exhaust_on_screen: false, ..FIGHTER_SPEC },
        FIGHTER => FIGHTER_SPEC,
        _ => level_spec(class).unwrap_or(FIGHTER_SPEC),
    }
}

/// Its own target (`+0x1b0` / `+0x260`): the moby of that index, if any.
fn own_target(w: &MobyWorld, id: usize) -> Option<usize> {
    let idx = w.mobys[id].pv_i(spec(w.mobys[id].o_class).target);
    if idx < 0 { return None; }
    w.mobys.iter().position(|m| m.index as i32 == idx && m.state < 0x80)
}

/// The sphere `(c, r)` against the camera's view (`0x302b78`, the view's planes `0x1bd900..`): −1 outside, else in
/// (here no 0 / 1 split: the callers ask only for −1), by the game's 4:3 tangents and near plane.
pub(crate) fn in_view(ctx: &Ctx, c: V3, r: f32) -> bool {
    let (eye, rows) = (ctx.host.camera_pos(), ctx.host.camera_rows());
    let d = sub(c, eye);
    let dot = |a: V3| a[0] * d[0] + a[1] * d[1] + a[2] * d[2];
    let (z, x, y) = (dot(rows[0]), dot(rows[1]), dot(rows[2]));
    let (tx, ty): (f32, f32) = (0.63, 0.63 * 0.775);
    let (kx, ky) = ((1.0 + tx * tx).sqrt(), (1.0 + ty * ty).sqrt());
    0.031_25 < z + r && x.abs() - r * kx <= tx * z && y.abs() - r * ky <= ty * z
}

/// Its radius (`+0xc` / 1024).
fn radius(w: &MobyWorld, id: usize) -> f32 { w.mobys[id].bsphere[3] / 1024.0 }

/// The space AI's ships (the fighters and the gunships, the other levels' ships).
pub(crate) fn is_ai(class: i16) -> bool { class == FIGHTER || class == GUNSHIP || level_spec(class).is_some() }

/// `0x3ca4a8` / `0x3cba68` / `0x376068`: the start (the other levels' the same, some left where they were placed:
/// Hrugis's `0x380228`, a wander goal from there).
pub(crate) fn start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let sp = spec(w.mobys[id].o_class);
    let (c, r) = arena(w, ctx);
    {
        let g = &mut ctx.game.rng;
        let (d, yaw, pitch) = (g.randf(0.0, r), g.rand_angle(), g.rand_angle());
        let p = super::shot::polar(d, yaw, pitch);
        let m = &mut w.mobys[id];
        m.state = 1;
        m.mode = (m.mode | mode::KEEP_ROWS) & !3;
        m.update_dist = 0xff;
        m.draw_dist = 0x1ff;
        m.scale = sp.scale;
        if sp.scatter { m.pos = add(c, p); }
        m.set_pv_f(HEALTH, sp.health);
        m.set_pv_i(STARTED, 1);
    }
    for o in (0..0x160).step_by(4) { set_ai_i(w, id, o, 0); }
    set_ai_i(w, id, a::ACTIVE, 1);
    set_ai_i(w, id, a::STATE, 1);
    set_ai_i(w, id, a::KIND, 1);
    w.mobys[id].set_pv_i(sp.gun, 0);
    take_own_target(w, id);
    wander_goal(w, id, ctx);
    w.changed = true;
}

/// The class code's own target (`0x3ca118` / `0x3cb770` in the moby's state 1): with its disc target (`+0x1b0` /
/// `+0x260` not −1) `+0x11c` = 1 (its runs take an offset) and `+0x120` that moby, every tick; without, no target
/// (it wanders).
pub(crate) fn take_own_target(w: &mut MobyWorld, id: usize) {
    match own_target(w, id) {
        Some(t) => {
            set_ai_i(w, id, a::OWN_TARGET, 1);
            set_ai_i(w, id, a::TARGET, t as i32 + 1);
        }
        None => set_ai_i(w, id, a::TARGET, 0),
    }
}

/// `0x377278`: the AI tick.
pub(crate) fn think(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let state = ai_i(w, id, a::STATE);
    let in_range = ai_i(w, id, a::RANGE) != 0;
    let (k, kd, top) = tuning(state, in_range);
    // The step (not circling).
    if state != 8 { w.mobys[id].pos = add(w.mobys[id].pos, ai_v(w, id, a::VEL)); }
    // `0x374f50` (not in state 2): kept inside the sphere less its own size; pushed off faces more than 6 times in a
    // row (`+0x15c`, [`push`]), its velocity bounces off the last face (`0x301da0`) with 20 sparks.
    if state != 2 {
        let (c, r) = arena(w, ctx);
        let own = radius(w, id);
        let d = sub(w.mobys[id].pos, c);
        if r < len(d) + own { w.mobys[id].pos = add(c, set_len(d, r - own)); }
        if 6 < ai_i(w, id, a::PUSHES) {
            let n = crate::hero::physics::unit(ai_v(w, id, a::PUSH_NORMAL));
            let v = ai_v(w, id, a::VEL);
            let into = v[0] * n[0] + v[1] * n[1] + v[2] * n[2];
            if into <= 0.0 { set_ai_v(w, id, a::VEL, sub(v, scale(n, 2.0 * into))); }
            set_ai_i(w, id, a::PUSHES, 0);
            bounce_sparks(w, id, ctx);
        }
    }
    // `0x3782c8`, the no-fly volumes (`0x221890`): none on Wupash (nothing sets the count), not ported.
    let vel = ai_v(w, id, a::VEL);
    // `0x377a58`: the push.
    let pos = w.mobys[id].pos;
    let mut push = set_len(sub(ai_v(w, id, a::GOAL), pos), k);
    let speed = ai_f(w, id, a::SPEED);
    let f = (speed - top * 0.5) / top;
    let mut m = f.abs().clamp(k, kd);
    if 0.0 < f { m = -m; }
    push = add(push, set_len(vel, m));
    if k * top < len(push) { push = set_len(push, k * top); }
    let mut vel = add(vel, push);
    // `0x377e30`: the top speed.
    let mut sp = len(vel);
    if top < sp {
        vel = set_len(vel, top);
        sp = top;
    }
    set_ai_v(w, id, a::VEL, vel);
    set_ai_f(w, id, a::SPEED, sp);
    // `0x377820`: the angles.
    let old_yaw = ai_f(w, id, a::YAW);
    let yaw = atan2(vel[0], vel[1]);
    let pitch = -atan2(len2(vel), vel[2]);
    let side = if sub_rot(yaw, old_yaw) < 0.0 { -1.0 } else { 1.0 };
    let turned = rot_dist(old_yaw, yaw);
    let roll = (ai_f(w, id, a::ROLL) * 0.98 + turned * -side * kd * 1.413_716_7 * 8.0).clamp(-1.413_716_7, 1.413_716_7);
    set_ai_f(w, id, a::ROLL, roll);
    set_ai_f(w, id, a::PITCH, pitch);
    set_ai_f(w, id, a::YAW, yaw);
    w.mobys[id].rows = crate::moby::attach::rows_of([roll, pitch, yaw]);
    // The state.
    match state {
        1 => {
            if arrived(w, id) { wander_goal(w, id, ctx); }
        }
        // 3, in a formation: its leader's place (`+0x10`) plus its own offset (`+0`) in the leader's frame (its
        // facing, across it in the level's plane, and up from those). Nothing on Wupash forms one.
        3 => {
            let leader = usize::try_from(ai_i(w, id, 0x10) - 1).ok().filter(|&l| w.mobys.get(l).is_some_and(|m| m.state < 0x80));
            if let Some(l) = leader {
                let fwd = w.mobys[l].rows[0];
                let side = crate::hero::physics::unit(crate::hero::physics::cross(fwd, [0.0, 0.0, 1.0]));
                let up = crate::hero::physics::cross(side, fwd);
                let goal = add(w.mobys[l].pos, in_frame([fwd, side, up], ai_v(w, id, 0)));
                set_goal(w, id, ctx, goal);
            }
        }
        // 4 closing (25 ahead of the target), 5 on its tail (5 ahead), 6 (10 ahead and 0.05 up): plus its offset in the
        // target's frame when it has one (`+0xfc`); 4 there → 5 for 300..600 ticks; 5 and 6 there → the attack's end.
        4..=6 => match target(w, id) {
            Some(t) => {
                let rows = w.mobys[t].rows;
                let mut goal = set_len(rows[0], match state { 4 => 25.0, 5 => 5.0, _ => 10.0 });
                if state == 6 { goal = add(goal, scale(rows[2], 0.05)); }
                if ai_i(w, id, a::OFFSET_ON) != 0 { goal = add(goal, in_frame(rows, ai_v(w, id, a::OFFSET))); }
                let goal = add(w.mobys[t].pos, goal);
                set_goal(w, id, ctx, goal);
                if arrived(w, id) {
                    if state == 4 {
                        set_ai_i(w, id, a::STATE, 5);
                        let t = ctx.game.rng.rand_range(300, 600);
                        set_ai_i(w, id, a::TIMER, t);
                    } else {
                        end_attack(w, id);
                    }
                }
            }
            None => end_attack(w, id),
        },
        // 7: behind the target (50); there → 8, circling it for 900 ticks.
        7 => match target(w, id) {
            Some(t) => {
                let tm = &w.mobys[t];
                let goal = sub(tm.pos, set_len(tm.rows[0], 50.0));
                set_goal(w, id, ctx, goal);
                if arrived(w, id) {
                    set_ai_i(w, id, a::STATE, 8);
                    set_ai_i(w, id, a::TIMER, 900);
                    let g = &mut ctx.game.rng;
                    let dir = if g.randi(2) == 0 { 1.0 } else { -1.0 };
                    set_ai_f(w, id, a::CIRCLE_DIR, dir);
                    set_ai_f(w, id, a::CIRCLE_ANGLE, if dir < 0.0 { std::f32::consts::FRAC_PI_2 } else { -std::f32::consts::FRAC_PI_2 });
                    let r = g.randf(5.0, 15.0);
                    set_ai_f(w, id, a::CIRCLE_RADIUS, r);
                }
            }
            None => end_attack(w, id),
        },
        // 8: round the target in the plane across the camera's up (`0x1b91d0`'s `+0x20`: the frame (x × up, up × that,
        // up); here the target ship's up, the camera's own), 0.75° a tick, facing along the circle; its timer out →
        // the attack's end.
        8 => match target(w, id) {
            Some(t) => {
                let (tp, tr) = (w.mobys[t].pos, w.mobys[t].rows);
                let (ang, dir, r) = (ai_f(w, id, a::CIRCLE_ANGLE), ai_f(w, id, a::CIRCLE_DIR), ai_f(w, id, a::CIRCLE_RADIUS));
                let (s, co) = ang.sin_cos();
                let up = tr[2];
                let r1 = crate::hero::physics::unit(crate::hero::physics::cross([1.0, 0.0, 0.0], up));
                let r0 = crate::hero::physics::cross(up, r1);
                let off = add(scale(r0, s * r), scale(r1, -co * r));
                w.mobys[id].pos = add(tp, off);
                let fwd = crate::hero::physics::unit(scale(add(scale(r0, co), scale(r1, s)), dir));
                let left = crate::hero::physics::unit(crate::hero::physics::cross(up, fwd));
                w.mobys[id].rows = [fwd, left, crate::hero::physics::cross(fwd, left)];
                set_ai_f(w, id, a::CIRCLE_ANGLE, crate::hero::physics::add_rot(ang, dir * 0.013_089_972));
                // `0x376778`: met something within 1.5 (flags 2) → put at the contact, 9 (flying on until off screen).
                if let Some(h) = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, w.mobys[id].pos, 1.5, crate::collision::QueryFlags(2), Some(id))) {
                    w.mobys[id].pos = h.point;
                    set_ai_i(w, id, a::STATE, 9);
                }
                // Its time out, it circles on until off screen (`0x302b78`: a sphere of 5 out of the view).
                if arrived(w, id) && !in_view(ctx, w.mobys[id].pos, 5.0) { end_attack(w, id); }
            }
            None => end_attack(w, id),
        },
        9 if !in_view(ctx, w.mobys[id].pos, 5.0) => end_attack(w, id),
        // 0xd, out of a no-fly volume: there → another 8 ahead.
        0xd if arrived(w, id) => ahead_goal(w, id, ctx),
        // 2, 0xb, 0xc: their tuning only.
        _ => {}
    }
}

/// `0x301e10(out, v, rows)`: `v` in the frame `rows`.
fn in_frame(rows: [V3; 3], v: V3) -> V3 { add(add(scale(rows[0], v[0]), scale(rows[1], v[1])), scale(rows[2], v[2])) }

/// `0x374f50`'s 20 sparks: twinkles (`0x346ee8(0.2, 1.25, 0, at, 60, 0x4000ff7f, 0, 2, dir, 0)`,
/// crate::particles::twinkle), `dir` 1/6 a tick at random angles.
fn bounce_sparks(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let at = w.mobys[id].pos;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    for _ in 0..20 {
        let g = &mut ctx.game.rng;
        let v = super::shot::polar(0.166_666_67, g.rand_angle(), g.rand_angle());
        crate::particles::twinkle(sys, g, 0.2, 1.25, 0.0, at, 60, 0x4000_ff7f, 0, 2, v, 0);
    }
}

/// `0x376650(m, 0, keep, off)`, before the hits: a sphere of its size against the world and the mobys (`0x2f29f8`,
/// flags 0); met, it is put at the pushed-out place (not with `keep` when what it met is a moby), and a face (not a
/// moby's primitive) counts a push in a row (`+0x15c`) and leaves its normal (`+0xd0`); nothing met, the count goes
/// back to 0. With `off` no sphere at all (the count to 0).
pub(crate) fn push(w: &mut MobyWorld, id: usize, ctx: &Ctx, keep: bool, off: bool) {
    if off { return set_ai_i(w, id, a::PUSHES, 0); }
    let r = radius(w, id);
    let hit = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, w.mobys[id].pos, r, crate::collision::QueryFlags(0), Some(id)));
    let Some(h) = hit else { return set_ai_i(w, id, a::PUSHES, 0) };
    if !(keep && h.moby.is_some()) {
        if let Some(p) = h.pushed_centre { w.mobys[id].pos = p; }
    }
    if 0 < h.kind {
        let n = ai_i(w, id, a::PUSHES) + 1;
        set_ai_i(w, id, a::PUSHES, n);
        set_ai_v(w, id, a::PUSH_NORMAL, crate::hero::physics::unit(h.normal));
    }
}

/// The standard blast of the levels' hit handlers (`0x4184c8` / L05 `0x426718` / L10 `0x427670` / L15 `0x4296b8`:
/// L25's, Feltzin's, Hrugis's and Gorn's copies of the blasts' `0x3f8348`, class 0x116a; crate::moby::blast) at
/// `size`: no shake, the flash ball (`+0x34` = 1).
fn blast(size: f32, bursts: [i32; 3], puffs: i32, streaks: i32, light: bool) -> crate::moby::blast::Blast {
    crate::moby::blast::Blast { size, bursts, puffs, streaks, light, shake: false }
}

/// The hit handler's sound timer (L10 `0x1b34cc`: Hrugis's 0xf8e calls out at most every 10..20 ticks), kept on the
/// level's mission moby (the port's word).
const CALL_TIMER: usize = 0x400;

/// `0x376650`'s call of the level's hit handler (`0x221cd4`, set by the mission): Wupash's `0x40f270`, Feltzin's
/// `0x421c18`, Hrugis's `0x4252b0`, Gorn's `0x431ad8`. A hit with the flags 0x230000 takes its damage (`+0x2c`) from
/// the health; at 0 the blast, the drop (`0x40fbc8(m, 1)` and its copies, [`drop`]), a formation broken up (none
/// form here) and the ship put away (`0x375fe8`: hidden, no collision, not active; not with `stay`, the push's
/// second argument). Returns 1 at the death. Each level's sounds and blast:
/// * Wupash: the fighter's sound 2 each hit; the blast at 0.5.
/// * Feltzin: the ship's 0xd each hit (not 0x0b73's), and one hit in three 0x10f5's own 1 (the others' the ship's
///   0xb); the blast at 0.75, 0x10f5's (`0x39a388(0.8, …, 0x19, 0, 0, 0x1e, 0)`) and 0x111a's three
///   (`0x39a388(2, …, 0xf, 0, 0, 0x1e, 0)` at it and twice ±1.5 round it) their own.
/// * Hrugis: a ship already at 0 takes nothing, 0x0cbe only the wingmen's (0xf93) shots; the ship's 0xd (not 0x0b73's
///   and 0xf8e's), 0xf8e's own 3 when the call timer is out (again in 10..20); the blast at 0.5.
/// * Gorn: 0x1211's own 5, the others' the ship's 0xd; the blast at 0.75.
///
/// Not ported: the hit flash (`+0x68`[4], `0x33a760` / `0x33a6a0`), the drop's second chance by the level's table.
pub(crate) fn hits_with(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, stay: bool, keep: bool, off: bool) -> bool {
    push(w, id, ctx, keep, off);
    let Some(h) = w.hit_message(id) else { return false };
    w.clear_hit(id);
    if h.tmpl.flags & 0x0023_0000 == 0 { return false; }
    let class = w.mobys[id].o_class;
    let level = ctx.game.level;
    let ship = crate::ship::SHIP_CLASS;
    if level == 10 {
        if w.mobys[id].pv_f(HEALTH) <= 0.0 { return false; }
        // Not ported: the hit's source (`+0x20`) — the port's hits carry none here, so 0x0cbe takes every hit.
    }
    let hp = (w.mobys[id].pv_f(HEALTH) - h.tmpl.damage).max(0.0);
    w.mobys[id].set_pv_f(HEALTH, hp);
    match level {
        5 if class != 0x0b73 => {
            ctx.host.play_sound_of(&w.mobys[id], ship, 0xd, 0);
            if ctx.game.rng.randi(3) == 0 {
                if class == 0x10f5 { ctx.host.play_sound_of(&w.mobys[id], 0x10f5, 1, 0); } else { ctx.host.play_sound_of(&w.mobys[id], ship, 0xb, 0); }
            }
        }
        10 => {
            if class != 0x0b73 && class != 0x0f8e { ctx.host.play_sound_of(&w.mobys[id], ship, 0xd, 0); }
            let mid = w.mobys.iter().position(|m| m.o_class == 0x1155);
            let mut t = mid.map_or(0, |m| w.mobys[m].pv_i(CALL_TIMER));
            if crate::moby::countdown(&mut t) && class == 0x0f8e {
                ctx.host.play_sound_of(&w.mobys[id], 0x0f8e, 3, 0);
                t = ctx.game.rng.randf(10.0, 20.0) as i32;
            }
            if let Some(m) = mid { w.mobys[m].set_pv_i(CALL_TIMER, t); }
        }
        15 if class != 0x0b73 => {
            if class == 0x1211 { ctx.host.play_sound_of(&w.mobys[id], 0x1211, 5, 0); } else { ctx.host.play_sound_of(&w.mobys[id], ship, 0xd, 0); }
        }
        5 | 15 => {}
        // `0x36cc38(2, 0, m, 0x957)`.
        _ => { ctx.host.play_sound_of(&w.mobys[id], FIGHTER, 2, 0); }
    }
    if 0.0 < hp { return false; }
    let at = w.mobys[id].pos;
    let burst = crate::moby::blast::burst;
    match (level, class) {
        (5, 0x10f5) => burst(w, ctx, at, &blast(0.8, [0x19, 0, 0], 0x1e, 0, true)),
        (5, 0x111a) => {
            burst(w, ctx, at, &blast(2.0, [0xf, 0, 0], 0x1e, 0, false));
            for _ in 0..2 {
                let g = &mut ctx.game.rng;
                let p = [at[0] + g.randf(-1.5, 1.5), at[1] + g.randf(-1.5, 1.5), at[2] + g.randf(-1.5, 1.5)];
                burst(w, ctx, p, &blast(2.0, [0xf, 0, 0], 0x1e, 0, false));
            }
        }
        (5 | 15, _) => burst(w, ctx, at, &blast(0.75, [0x32, 0x1e, 0x1e], 0x1e, 0x28, true)),
        _ => burst(w, ctx, at, &blast(0.5, [0x32, 0x1e, 0x1e], 0x1e, 0x28, true)),
    }
    drop(w, id, ctx);
    if !stay { put_away(w, id); }
    true
}

/// `0x375fe8` (L10 `0x37dea0`): the ship put away — hidden, no collision, not active, not attacking.
pub(crate) fn put_away(w: &mut MobyWorld, id: usize) {
    set_ai_i(w, id, a::ATTACKING, 0);
    set_ai_i(w, id, a::ACTIVE, 0);
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.draw_dist = 0;
    m.drawn = 0;
    m.mode |= 3;
    w.changed = true;
}

fn hits(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> bool { hits_with(w, id, ctx, false, false, false) }

/// `0x376a60(range, m, check)`: in range (the target within the class's range, the line clear; with `check` also a
/// sphere of 0.1 at the ship on screen, `0x302b78`) and in the gun's cone (40.5° across and up; set before the
/// screen check).
pub(crate) fn range(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    set_ai_i(w, id, a::CONE, 0);
    set_ai_i(w, id, a::RANGE, 0);
    if passive(w) { return; }
    let Some(t) = target(w, id) else { return };
    let (p, q) = (w.mobys[id].pos, w.mobys[t].pos);
    let sp = spec(w.mobys[id].o_class);
    if sp.range < dist(p, q) { return; }
    // The line clear, or ending on a moby of the target's class.
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, p, q, crate::collision::QueryFlags(0), Some(id)));
    if hit.is_some_and(|h| h.moby.is_none_or(|m| w.mobys.get(m).is_none_or(|m| m.o_class != w.mobys[t].o_class))) { return; }
    let d = sub(q, p);
    let across = rot_dist(atan2(d[0], d[1]), ai_f(w, id, a::YAW));
    let up = rot_dist(atan2(len2(d), d[2]), 0.0);
    if across <= 0.706_858_3 && up <= 0.706_858_3 { set_ai_i(w, id, a::CONE, 1); }
    if sp.on_screen && !in_view(ctx, p, 0.1) { return; }
    set_ai_i(w, id, a::RANGE, 1);
}

/// `0x3ca5a8` / `0x3cbb78` (and the other levels' guns): in range and in the cone (the circling gunship: in range;
/// some levels' ships the cone alone) with the gun ready (`+0x1b8` / `+0x268`), a shot at the point the class's lead
/// ahead of the target (`0x375198`), the gun ready again after the class's wait; its sound one shot in three (or two
/// in three). Feltzin's 0x10f5 throws flame instead ([`flame`]).
pub(crate) fn fire(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let sp = spec(w.mobys[id].o_class);
    let mut wait = w.mobys[id].pv_i(sp.gun);
    crate::moby::countdown(&mut wait);
    w.mobys[id].set_pv_i(sp.gun, wait);
    let cone = ai_i(w, id, a::CONE) != 0 || (sp.circles && ai_i(w, id, a::STATE) == 8);
    if wait != 0 || (sp.fire_in_range && ai_i(w, id, a::RANGE) == 0) || !cone { return; }
    let Some(t) = target(w, id) else { return };
    if sp.flame { return flame(w, id, t, ctx); }
    if w.mobys[t].draw_dist == 0 { return; }
    let at = add(w.mobys[t].pos, set_len(w.mobys[t].rows[0], sp.lead));
    let from = w.mobys[id].pos;
    let mut shot = sp.shot;
    if let Some(c) = sp.shot_alt {
        if ctx.game.rng.randi(2) == 0 { shot.2 = c; }
    }
    if w.mobys[id].state == 1 { crate::ship::spawn_enemy_shot(w, &*ctx.host, from, at, t, shot); }
    w.mobys[id].set_pv_i(sp.gun, sp.wait);
    // `0x36cc38(0, 0, m, class)` one shot in three (`0x32e060(3) == 0`; some two in three, `≠ 0`).
    if let Some((c, i, most)) = sp.gun_sound {
        if (ctx.game.rng.randi(3) != 0) == most { ctx.host.play_sound_of(&w.mobys[id], c, i, 0); }
    }
}

/// Feltzin's 0x10f5's flame (`0x41e108`): the line 5 toward the target (`0x2f7018`) meeting the player's ship (0x0fbf)
/// shakes the camera (0.05 for 30 ticks, `0x1b9660`) and hits the ship for 1 (`0x37bef0`, crate::ship's hits); the
/// gun ready again in 4 ticks. Not ported: its three flame puffs (`0x348378` from its joint 0, 0.58 a tick toward
/// the target).
fn flame(w: &mut MobyWorld, id: usize, t: usize, ctx: &mut Ctx) {
    let from = w.mobys[id].pos;
    let to = add(from, set_len(sub(w.mobys[t].pos, from), 5.0));
    let hit = ctx.coll.and_then(|c| crate::collision::coll_line_m(c, ctx.scene, from, to, crate::collision::QueryFlags(0), Some(id)));
    if let Some(s) = hit.and_then(|h| h.moby).filter(|&s| w.mobys.get(s).is_some_and(|m| m.o_class == crate::ship::SHIP_CLASS)) {
        ctx.game.shakes.ask(0, 0.05, 30);
        let n = w.mobys[s].pv_i(crate::ship::HITS) + 1;
        w.mobys[s].set_pv_i(crate::ship::HITS, n);
    }
    let sp = spec(w.mobys[id].o_class);
    w.mobys[id].set_pv_i(sp.gun, sp.wait);
}

/// `0x3769a0` / `0x3767e8`: the attack's start (not in the quiet, of the tick's leading group, not attacking, fewer
/// than 4 (`0x221820`) attacking, not in a formation or out of a no-fly volume): a gunship takes the circling one
/// (kind 5: → 7) while none of its group's last attack was one (`0x377f20`), else → 4, for 120..500 ticks. A run
/// (→ 4) of a ship with its own target (`+0x11c`) takes an offset in the target's frame: ±1 across and ±1 up.
pub(crate) fn maybe_attack(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if passive(w) { return; }
    if target(w, id).is_none() || ai_i(w, id, a::ATTACKING) == 1 || matches!(ai_i(w, id, a::STATE), 3 | 0xd) { return; }
    let attacking = w.mobys.iter().filter(|m| is_ai(m.o_class) && m.state < 0x80 && m.pv_i(AI + a::ATTACKING) == 1).count();
    if 4 <= attacking { return; }
    let class = w.mobys[id].o_class;
    // Only the tick's leading group (`0x3769a0`).
    if w.mobys.iter().find(|m| m.o_class == 0x0eee && m.state < 0x80).is_some_and(|m| m.pv_i(LEADER) != 0 && m.pv_i(LEADER) != class as i32) { return; }
    // `0x377f20(m, 5)`: its group's active ships whose last kind was 5 — itself too, so one that just circled makes a
    // run next.
    let circling = w.mobys.iter().any(|m| m.o_class == class && m.state < 0x80 && m.pv_i(AI + a::ACTIVE) == 1 && m.pv_i(AI + a::KIND) == 5);
    let kind = if spec(class).circles && !circling { 5 } else { 1 };
    set_ai_i(w, id, a::KIND, kind);
    set_ai_i(w, id, a::ATTACKING, 1);
    set_ai_i(w, id, a::STATE, if kind == 5 { 7 } else { 4 });
    if kind != 5 && ai_i(w, id, a::OWN_TARGET) != 0 {
        let g = &mut ctx.game.rng;
        let up = if g.randi(2) != 0 { -1.0 } else { 1.0 };
        let across = if g.randi(2) != 0 { -1.0 } else { 1.0 };
        set_ai_v(w, id, a::OFFSET, [0.0, across, up]);
        set_ai_i(w, id, a::OFFSET_ON, 1);
    } else {
        set_ai_i(w, id, a::OFFSET_ON, 0);
    }
    let t = ctx.game.rng.rand_range(0x78, 500);
    set_ai_i(w, id, a::TIMER, t);
}

/// The fighters active (`0x221810`).
pub fn active_count(w: &MobyWorld) -> usize {
    w.mobys.iter().filter(|m| is_ai(m.o_class) && m.state < 0x80 && m.pv_i(STARTED) != 0 && m.pv_i(AI + a::ACTIVE) == 1).count()
}

/// `0x374e10`: the tick's leading group. The leaders' list (`0x2258c0`, `0x3750e8` at each ship's start whose disc
/// `+0x1b4` / `+0x264` is set: on Wupash one fighter and the gunships) in level order; one picked at random, then on
/// through the list to one whose group has an active ship (`0x3765b0`; or any while none is active). Its group is the
/// moby group (`+0x21`); here its class (the port keeps no groups for these). Returns the class, 0 for none.
pub fn leader(w: &MobyWorld, rng: &mut crate::rng::Rng) -> i32 {
    let list: Vec<i16> = w.mobys.iter().filter(|m| is_ai(m.o_class) && m.pv_i(spec(m.o_class).leads) != 0).map(|m| m.o_class).collect();
    let Some(&first) = list.first() else { return 0 };
    if list.len() < 2 { return first as i32; }
    let any = active_count(w) != 0;
    let has = |c: i16| w.mobys.iter().any(|m| m.o_class == c && m.state < 0x80 && m.pv_i(AI + a::ACTIVE) == 1);
    let start = rng.rand_range(0, list.len() as i32 - 1) as usize;
    (0..list.len()).map(|k| list[(start + k) % list.len()]).find(|&c| !any || has(c)).unwrap_or(first) as i32
}

/// `0x22182c`: the quiet (the mission object's word).
fn passive(w: &MobyWorld) -> bool { w.mobys.iter().find(|m| m.o_class == 0x0eee && m.state < 0x80).is_some_and(|m| m.pv_i(PASSIVE) != 0) }

/// `0x376548(1)` / `0x378250(10, at)` / `0x378878(10, goal)`: every fighter back (`0x376068`), within 10 of `at`,
/// going for a point within 10 of `goal`.
pub fn next_wave(w: &mut MobyWorld, at: V3, goal: V3) {
    for m in w.mobys.iter_mut().filter(|m| is_ai(m.o_class) && m.state < 0x80) {
        // Updated again (`0x376068` clears the mode's 3); the rest of the return is its next update's.
        m.mode &= !3;
        m.set_pv_i(REVIVE, 1);
        m.set_pv_v(REVIVE_AT, at);
        m.set_pv_v(REVIVE_GOAL, goal);
    }
}

/// `0x376068(m, 1)` (L05 `0x37c130`) on a ship put away: shown and drawn from anywhere, its health back, the AI block
/// cleared, active, wandering from where it is.
pub(crate) fn activate(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    {
        let m = &mut w.mobys[id];
        m.mode &= !3;
        m.update_dist = 0xff;
        m.draw_dist = 0x1ff;
        m.has_collision = true;
        let h = spec(m.o_class).health;
        m.set_pv_f(HEALTH, h);
    }
    for o in (0..0x160).step_by(4) { set_ai_i(w, id, o, 0); }
    set_ai_i(w, id, a::ACTIVE, 1);
    set_ai_i(w, id, a::STATE, 1);
    set_ai_i(w, id, a::KIND, 1);
    take_own_target(w, id);
    wander_goal(w, id, ctx);
    w.changed = true;
}

/// The goal again from where it is now (`0x3783a0`, L05 `0x37e480`).
pub(crate) fn wander(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { wander_goal(w, id, ctx) }

/// The return: active again (`0x376068`), shown, its health back, within 10 of the gathering point, its goal within 10
/// of the wave's.
fn revive(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (at, goal) = (w.mobys[id].pv_v(REVIVE_AT), w.mobys[id].pv_v(REVIVE_GOAL));
    let g = &mut ctx.game.rng;
    let p = super::shot::polar(g.randf(0.0, 10.0), g.rand_angle(), g.rand_angle());
    let q = super::shot::polar(g.randf(0.0, 10.0), g.rand_angle(), g.rand_angle());
    {
        let m = &mut w.mobys[id];
        m.set_pv_i(REVIVE, 0);
        m.mode &= !3;
        m.draw_dist = 0x1ff;
        m.has_collision = true;
        m.pos = add(at, p);
        let h = spec(m.o_class).health;
        m.set_pv_f(HEALTH, h);
    }
    for o in (0..0x160).step_by(4) { set_ai_i(w, id, o, 0); }
    set_ai_i(w, id, a::ACTIVE, 1);
    set_ai_i(w, id, a::STATE, 1);
    set_ai_i(w, id, a::KIND, 1);
    take_own_target(w, id);
    set_goal(w, id, ctx, add(goal, q));
    set_ai_i(w, id, a::TIMER, 300);
    w.changed = true;
}

/// `0x3ca118`.
pub fn update_fighter(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(STARTED) == 0 { return start(w, id, ctx); }
    if w.mobys[id].pv_i(REVIVE) != 0 { revive(w, id, ctx); }
    if ai_i(w, id, a::ACTIVE) != 1 { return; }
    let sp = spec(w.mobys[id].o_class);
    think(w, id, ctx);
    // The death's sound (`0x376650(m, 0, 0, 0) == 1`: the fighter's `0x36cc38(1, 0, m, 0x957)`).
    if hits(w, id, ctx) {
        if let Some((c, i)) = sp.death { ctx.host.play_sound_of(&w.mobys[id], c, i, 0); }
    }
    if ai_i(w, id, a::ACTIVE) != 1 { return stop_engine(w, id, ctx); }
    engine(w, id, ctx);
    range(w, id, ctx);
    take_own_target(w, id);
    flyby(w, id, ctx);
    fire(w, id, ctx);
    maybe_attack(w, id, ctx);
    exhaust(w, id, ctx);
    w.rebuild(id, &*ctx.host);
}

/// The engine's loop gone (the ship put away).
pub(crate) fn stop_engine(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(ENGINE) - 1;
    if 0 <= h { ctx.host.stop_sound(h); w.mobys[id].set_pv_i(ENGINE, 0); }
}

/// The engine's loop (`0x36cc38(3, 6, m, 0x957)` / `(1, 6, m, 0x97c)`, `+0x1bc` / `+0x2f0`; the class's), started
/// again whenever it is not playing.
pub(crate) fn engine(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some((c, i)) = spec(w.mobys[id].o_class).engine else { return };
    let h = w.mobys[id].pv_i(ENGINE) - 1;
    // Asked only within hearing (64: the sound layer drops the farther ones anyway).
    let near = dist(w.mobys[id].pos, ctx.host.camera_pos()) < 64.0;
    if near && (h < 0 || !ctx.host.sound_playing(h)) {
        let n = ctx.host.play_sound_of(&w.mobys[id], c, i, 4);
        w.mobys[id].set_pv_i(ENGINE, n + 1);
    }
}

/// `0x3ca6c0` (the fighter, while its sphere is on screen: `0x302b78`) / `0x3cbc40` (the gunship, its two engines),
/// and the other levels' (their [`Spec`]): not in a scene, from the nozzle (the fighter's 0.3 behind it; the
/// gunship's 0.21 behind, ±0.061 across, 0.045 up, in its frame) jittered ±0.1 (`0x333500`), three puffs
/// (crate::particles::exhaust, each from its own jitter of the nozzle) moving back 0.4·(30..40)/60 a tick in its frame.
pub(crate) fn exhaust(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.scenes.mode == crate::scene::SCENE { return; }
    let m = &w.mobys[id];
    let sp = spec(m.o_class);
    if sp.exhaust_on_screen && !in_view(ctx, m.pos, radius(w, id)) { return; }
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let (nozzles, first) = (sp.nozzles, sp.desc);
    let jitter = |g: &mut crate::rng::Rng, p: V3| [p[0] + g.randf(-0.1, 0.1), p[1] + g.randf(-0.1, 0.1), p[2] + g.randf(-0.1, 0.1)];
    for &n in nozzles {
        let base = add(m.pos, in_frame(m.rows, n));
        for k in 0..3 {
            let g = &mut ctx.game.rng;
            let at = jitter(g, base);
            let (lo, hi, k_back) = sp.puff;
            let vel = [-(g.randf(lo, hi) / 60.0) * k_back, 0.0, 0.0];
            crate::particles::exhaust(sys, g, m, id, at, vel, first + k);
        }
    }
}

/// The class code's fly-by: within 10 of its own target, its sound (`+0x1c0` / `+0x2f4`) not playing, one chance in
/// 400 (the gunship 750) a tick: the ship's sound 0xe (`0x36cc38(0xe, 0, m, 0xfbf)`).
pub(crate) fn flyby(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(t) = own_target(w, id) else { return };
    if 10.0 <= dist(w.mobys[t].pos, w.mobys[id].pos) { return; }
    let h = w.mobys[id].pv_i(FLYBY) - 1;
    if 0 <= h && ctx.host.sound_playing(h) { return; }
    let odds = spec(w.mobys[id].o_class).flyby;
    if odds == 0 || ctx.game.rng.randi(odds) != 0 { return; }
    let n = ctx.host.play_sound_of(&w.mobys[id], crate::ship::SHIP_CLASS, 0xe, 0);
    w.mobys[id].set_pv_i(FLYBY, n + 1);
}

/// The pickups the fighters drop: the shield's 0x0946 (`0x3c7408`) and the bolts' 0x10cf (`0x415790`).
pub const SHIELD_PICKUP: i16 = 0x0946;
pub const BOLT_PICKUP: i16 = 0x10cf;

/// `0x40fbc8(m, 1)`: a dead fighter's drop. Three times in four nothing; else one in 48 draws: 0..3 the shield's
/// (`0x3c7330`) while the shield is not full, 4 (`0x40fb20`) and 5..7 (the weapons 2..4 none has) the missiles' while
/// they are not full (`0x415e00`, not ported: the bolts' instead), the rest the bolts' (`0x4156b8`). Not ported: the
/// second chance of the shield's by the level's table (`0x13a148`).
fn drop(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let g = &mut ctx.game.rng;
    if g.randi(4) != 0 { return; }
    let r = g.randi(0x30);
    let ship = w.mobys.iter().position(|m| m.o_class == crate::ship::SHIP_CLASS && m.state < 0x80);
    let shield_full = ship.is_none_or(|s| w.mobys[s].pv_i(crate::ship::SHIELD_NOW) >= crate::ship::SHIELD_MAX);
    let class = if r < 4 && !shield_full { SHIELD_PICKUP } else { BOLT_PICKUP };
    if w.class(class).is_none() { return; }
    let at = w.mobys[id].pos;
    let p = w.create(class);
    let base = w.class(class).map_or(1.0, |c| c.scale);
    let yaw = ctx.game.rng.rand_angle();
    let m = &mut w.mobys[p];
    m.state = 0;
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.pos = at;
    m.rot = [0.0, 0.0, yaw];
    m.scale = base;
    w.rebuild(p, &*ctx.host);
}

/// `0x3c7408` / `0x415790`: within 60 of the ship the pickup closes on it (`0x331058`: a spring on the distance,
/// 0.0056 / 0.0017, at most 0.5 a tick) shrinking (its scale · d / 45, at least 0.3); within 0.5 it is taken — the
/// shield's adds 33 (one time in four all of it, `0x29c310` slot 6), the bolts' are counted (`0x1a7a04`). It turns
/// (0.03 a tick about z; the bolts' also 0.01 about x); taken, the ship's sound 0xc and the message (the shield's
/// 0x1283 / 0x1284 full, the bolts' 0x1282 after the first; [`pickup_sparkles`] the sparkles round it).
pub fn update_pickup(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let class = w.mobys[id].o_class;
    let base = w.class(class).map_or(1.0, |c| c.scale);
    if let Some(s) = w.mobys.iter().position(|m| m.o_class == crate::ship::SHIP_CLASS && m.state < 0x80 && m.draw_dist != 0) {
        let to = w.mobys[s].pos;
        let d = dist(to, w.mobys[id].pos);
        if d < 60.0 {
            let m = &mut w.mobys[id];
            m.scale = (base * 0.022_222_223 * d).max(base * 0.3);
            let mut v = m.pv_f(0x40);
            crate::hero::physics::spring_vel(d, 0.005_555_556, 0.001_666_667, 0.5, &mut v);
            m.set_pv_f(0x40, v);
            m.pos = add(m.pos, set_len(sub(to, m.pos), v));
        }
        if d < 0.5 {
            // `0x36cc38(0xc, 0, m, 0xfbf)`; the bolts' first one's help (`0x307d80(0xa3b, 0x92)`).
            ctx.host.play_sound_of(&w.mobys[id], crate::ship::SHIP_CLASS, 0xc, 0);
            if class == BOLT_PICKUP {
                // The first one's help; the next ones' message 0x1282 (`0x321738`, crate::msgbox).
                if ctx.game.space_bolts == 0 {
                    if !ctx.game.bolt_help_done { ctx.game.bolt_help_done = ctx.game.help(0xa3b, 0x92); }
                } else {
                    ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x1282, ticks: None });
                }
                // `0x1a7a04`: the bolts taken (the HUD's panel shows it for 300 ticks, `0x40f3e0`).
                ctx.game.space_bolts += 1;
                ctx.game.space_bolts_shown = 300;
            }
            if class == SHIELD_PICKUP {
                // One in four the shield full (message 0x1284), else 33 (0x1283).
                let full = ctx.game.rng.randi(4) == 0;
                let add = if full { crate::ship::SHIELD_MAX } else { 0x21 };
                ctx.game.messages.push(crate::msgbox::Request::Text { text: if full { 0x1284 } else { 0x1283 }, ticks: None });
                let n = w.mobys[s].pv_i(crate::ship::SHIELD_ADD) + add;
                w.mobys[s].set_pv_i(crate::ship::SHIELD_ADD, n);
            }
            return w.delete(id);
        }
    }
    let m = &mut w.mobys[id];
    m.rot[2] = crate::hero::physics::add_rot(m.rot[2], 0.03);
    if class == BOLT_PICKUP { m.rot[0] = crate::hero::physics::add_rot(m.rot[0], 0.01); }
    pickup_sparkles(w, id, ctx);
    w.rebuild(id, &*ctx.host);
}

/// The pickups' four sparkles' words (the port's, past the class's pvars): set up, the rolls, their steps, the blink
/// timers, the sizes.
const SPARKLES_SET: usize = 0x400;
const SPARKLE_ROLL: usize = 0x404;
const SPARKLE_STEP: usize = 0x414;
const SPARKLE_TIMER: usize = 0x424;
const SPARKLE_SIZE: usize = 0x434;

/// `0x3c7720` (the shield's) / `0x415a88` (the bolts'), set up by `0x3c7648`: four sparkles (crate::particles::sparkle,
/// `0x347cb0(size, at, colour, roll, def 0x35, soft, 2 ticks)`, far byte 0x61) in a line away from the camera from
/// 0.3 behind the pickup (0.25 up), 0.1 apart: each rolls by its step (−1, −2.25, 1.25, 2.5; from a random roll,
/// wrapping at 255), blinks on a timer (from 0x3f, 0x7f, 0xbf, 0xff; back to 0xff when out), its colour from the
/// first to the second by |½ − (255 − timer)/255| (the shield's 0x50ff4040 → 0x10ff4040, the bolts' 0x40405fff →
/// 0x10405fff), sizes 2.8, 3.5, 3.5, 2.8.
fn pickup_sparkles(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(SPARKLES_SET) == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_i(SPARKLES_SET, 1);
        for k in 0..4 {
            let roll = ctx.game.rng.randi(0xff) as f32;
            m.set_pv_f(SPARKLE_ROLL + 4 * k, roll);
            m.set_pv_f(SPARKLE_STEP + 4 * k, [-1.0, -2.25, 1.25, 2.5][k]);
            m.set_pv_i(SPARKLE_TIMER + 4 * k, 0x3f + 0x40 * k as i32);
            m.set_pv_f(SPARKLE_SIZE + 4 * k, [2.8, 3.5, 3.5, 2.8][k]);
        }
    }
    let (from, to) = if w.mobys[id].o_class == BOLT_PICKUP { (0x4040_5fff, 0x1040_5fff) } else { (0x50ff_4040, 0x10ff_4040) };
    let p = w.mobys[id].pos;
    let p = [p[0], p[1], p[2] + 0.25];
    let away = sub(p, ctx.host.camera_pos());
    let mut at = add(p, set_len(away, 0.3));
    let step = set_len(away, 0.1);
    let hero = ctx.hero.pos;
    for k in 0..4 {
        let m = &mut w.mobys[id];
        let mut roll = m.pv_f(SPARKLE_ROLL + 4 * k) + m.pv_f(SPARKLE_STEP + 4 * k);
        if 255.0 <= roll { roll -= 255.0 } else if roll <= 0.0 { roll += 255.0 }
        m.set_pv_f(SPARKLE_ROLL + 4 * k, roll);
        let mut t = m.pv_i(SPARKLE_TIMER + 4 * k);
        if crate::moby::countdown(&mut t) { t = 0xff; }
        m.set_pv_i(SPARKLE_TIMER + 4 * k, t);
        let f = (0.5 - (0xff - t) as f32 * 0.003_921_569).abs();
        let colour = crate::hud::lerp_rgba(f, from, to);
        let size = m.pv_f(SPARKLE_SIZE + 4 * k);
        if let Some(sys) = ctx.parts.as_deref_mut() {
            if let Some(i) = crate::particles::sparkle(sys, size, at, colour, roll as i32 as u8, 0x35, true, 2, false, hero) {
                sys.pool.recs[i][9] = 0x61;
            }
        }
        at = add(at, step);
    }
}

/// The asteroid field's spawner (0x095f, `0x3ca8b0`) and the asteroids (0x0953..0x0956, 0x0971: `0x3c91c8`).
pub const FIELD: i16 = 0x095f;
pub const ASTEROIDS: [i16; 5] = [0x0953, 0x0954, 0x0955, 0x0956, 0x0971];

/// The asteroids' tables by kind (`0x3c9db0`: the class − 0x953, 0x971 → 4): their size (`$gp+0xb18`), the class of
/// their pieces (`+0xb30`), how many (`+0xb78`), how many chips (`+0xb48`, of 0x955; 0x956's of 0x971).
const AST_SIZE: [f32; 5] = [5.0, 2.0, 0.5, 10.0, 2.0];
const AST_PIECE: [i16; 5] = [0x0954, 0x0955, -1, 0x0971, -1];
const AST_PIECES: [i32; 5] = [3, 2, -1, -1, -1];
const AST_CHIPS: [i32; 5] = [20, 10, -1, 40, -1];

fn kind(class: i16) -> usize { if class == 0x0971 { 4 } else { (class - 0x0953).clamp(0, 4) as usize } }

/// `0x3c9128` + `0x3c9b50`: an asteroid of `class` at `at` moving `vel`: its spin random (to ½°, 1°, 2° a tick), its
/// angles random, its scale its class's by its kind's size, in `state` (1 still; 2 a piece settling; 3 a chip).
fn spawn_asteroid(w: &mut MobyWorld, ctx: &mut Ctx, class: i16, at: V3, vel: V3, state: u8) {
    if w.class(class).is_none() { return; }
    let base = w.class(class).map_or(1.0, |c| c.scale);
    let g = &mut ctx.game.rng;
    let rot = [g.rand_angle(), g.rand_angle(), g.rand_angle()];
    let spin = [g.randf(0.0, 0.5) * 0.017_453_294, g.randf(0.0, 1.0) * 0.017_453_294, g.randf(0.0, 2.0) * 0.017_453_294];
    let id = w.create(class);
    let m = &mut w.mobys[id];
    m.state = state;
    m.update_dist = 0xff;
    m.draw_dist = 0x1ff;
    m.drawn = 1;
    m.pos = at;
    m.rot = rot;
    m.scale = base * AST_SIZE[kind(class)];
    m.set_pv_v(0, spin);
    m.set_pv_v(0x10, vel);
    w.rebuild(id, &*ctx.host);
}

/// `0x3ca8b0`: on its first tick, its counts of asteroids (`+0xe` of 0x956, `+0xc` of 0x953, `+8` of 0x954) at
/// random points of its cuboid (`+0`, `0x3caa48`), then gone. Not ported: the light they take from 0x12b3 / 0x12b6.
pub fn update_field(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let (cuboid, n953, n954, n956) = (m.pv_i(0), m.pv_s(0xc) as i32, m.pv_i(8), m.pv_s(0xe) as i32);
    let Some(c) = usize::try_from(cuboid).ok().and_then(|i| ctx.level.cuboids.get(i)).cloned() else { return w.delete(id) };
    for (class, n) in [(0x0956, n956), (0x0953, n953), (0x0954, n954)] {
        for _ in 0..n.max(0) {
            let g = &mut ctx.game.rng;
            let l = [g.randf(-1.0, 1.0), g.randf(-1.0, 1.0), g.randf(-1.0, 1.0)];
            let mx = &c.matrix;
            let at = std::array::from_fn(|k| mx[0][k] * l[0] + mx[1][k] * l[1] + mx[2][k] * l[2] + mx[3][k]);
            spawn_asteroid(w, ctx, class, at, [0.0; 3], 1);
        }
    }
    w.delete(id);
}

/// `0x3c91c8`: the asteroid drifts (its velocity ×0.99 a tick; a piece settles to 1 when still), spins; the ship
/// within its size breaks it (`0x3c9c40` → `0x3c9d18`: a hit on itself of 5, the ship's own damage is its collision's);
/// a hit with the flags 0x230000 (0x956 not by the lasers) or the world's box ends it (`0x3c94d8`): its pieces thrown
/// apart (24 a second along the hit, 3× for the ram; and 12 a second each way), its chips; the big ones (size 5 or
/// more) a drop (`0x3c9e78`, as the fighters'); the dust (`0x342520`), the ram's shake and the sounds.
pub fn update_asteroid(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let class = w.mobys[id].o_class;
    let k = kind(class);
    {
        let m = &mut w.mobys[id];
        let v = m.pv_v(0x10);
        m.pos = add(m.pos, v);
        let v = [v[0] * 0.99, v[1] * 0.99, v[2] * 0.99];
        m.set_pv_v(0x10, v);
        if m.state == 2 && v.iter().all(|c| c.abs() < 0.01) { m.state = 1; }
        let s = m.pv_v(0);
        m.rot = [crate::hero::physics::add_rot(m.rot[0], s[0]), crate::hero::physics::add_rot(m.rot[1], s[1]), crate::hero::physics::add_rot(m.rot[2], s[2])];
    }
    let cam = ctx.host.camera_pos();
    let mut push: Option<(V3, bool)> = None;
    if dist(w.mobys[id].pos, cam) <= AST_SIZE[k] && class != 0x0956 {
        push = Some((set_len(sub(w.mobys[id].pos, cam), 1.0), true));
    }
    if let Some(h) = w.hit_message(id) {
        w.clear_hit(id);
        let laser = h.tmpl.flags & 0x0010_0000 != 0;
        if h.tmpl.flags & 0x0023_0000 != 0 && !(laser && class == 0x0956) && push.is_none() {
            push = Some((set_len([h.tmpl.dir[0], h.tmpl.dir[1], h.tmpl.dir[2]], 1.0), false));
        }
    }
    let p = w.mobys[id].pos;
    let out = p.iter().any(|&c| !(2.0..=1021.0).contains(&c));
    if let Some((dir, ram)) = push {
        // `0x36cc38(7, 0, m, 0xfbf)` for the ram; the break's own (`0x36cc38(0, 0, m, class)`: 0x956's for the big
        // ones, 0x954 / 0x955 their own).
        if ram { ctx.host.play_sound_of(&w.mobys[id], crate::ship::SHIP_CLASS, 7, 0); }
        let own = if 5.0 <= AST_SIZE[k] { Some(0x0956) } else if matches!(class, 0x0954 | 0x0955) { Some(class) } else { None };
        if let Some(c) = own { ctx.host.play_sound_of(&w.mobys[id], c, 0, 0); }
        let along = scale(dir, 24.0 / 60.0 * if ram { 3.0 } else { 1.0 });
        let half = AST_SIZE[k] * 0.5;
        let jitter = |g: &mut crate::rng::Rng| [p[0] + g.randf(-half, half), p[1] + g.randf(-half, half), p[2] + g.randf(-half, half)];
        if 0 < AST_PIECES[k] {
            for _ in 0..AST_PIECES[k] {
                let at = jitter(&mut ctx.game.rng);
                let g = &mut ctx.game.rng;
                let v = add(along, super::shot::polar(24.0 / 120.0, g.rand_angle(), g.rand_angle()));
                spawn_asteroid(w, ctx, AST_PIECE[k], at, v, 2);
            }
        }
        if 0 < AST_CHIPS[k] {
            let chip = if class == 0x0956 { 0x0971 } else { 0x0955 };
            let speed = 24.0 / 120.0 * if class == 0x0956 { 2.0 } else { 1.0 };
            for _ in 0..AST_CHIPS[k] {
                let at = jitter(&mut ctx.game.rng);
                let v = if class == 0x0956 {
                    set_len(sub(at, p), speed)
                } else {
                    let g = &mut ctx.game.rng;
                    super::shot::polar(speed, g.rand_angle(), g.rand_angle())
                };
                spawn_asteroid(w, ctx, chip, at, add(along, v), 3);
            }
        }
        // The dust (`0x342520`, crate::particles::puff): 50 grey puffs (the ram of a chip: 40, bigger, slower) round
        // it, a twentieth of a unit a tick outward with its push, spinning ±3.
        let (n, size, speed) = if ram && class == 0x0955 { (40, AST_SIZE[k] * 0.5 * 3.0, 0.033_333_335) } else { (50, AST_SIZE[k] * 0.5, 0.05) };
        let near = dist(p, cam) < 128.0;
        if let (true, Some(sys)) = (near, ctx.parts.as_deref_mut()) {
            for _ in 0..n {
                let g = &mut ctx.game.rng;
                let at = [p[0] + g.randf(-half, half), p[1] + g.randf(-half, half), p[2] + g.randf(-half, half)];
                let v = add(super::shot::polar(speed, g.rand_angle(), g.rand_angle()), scale(along, 0.5));
                let spin = g.rand_range(-3, 3);
                crate::particles::puff(sys, g, 0.0, 1.0, 1.0, size * 210_000.0, at, spin, v, 0x7fff_ffff, 0);
            }
        }
        // `0x3c9c40`: the ram's shake (0.05 over 30 ticks for 0x955, else 0.15 over 60: `0x1b91a0`, crate::camera::Shakes).
        if ram {
            if class == 0x0955 { ctx.game.shakes.ask(0, 0.05, 0x1e) } else { ctx.game.shakes.ask(0, 0.15, 0x3c) }
        }
        if 5.0 <= AST_SIZE[k] { drop(w, id, ctx); }
        return w.delete(id);
    }
    if out { return w.delete(id); }
    w.rebuild(id, &*ctx.host);
}
