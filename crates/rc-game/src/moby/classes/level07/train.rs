//! Level 7's convoy: the lead ship **0x11d9** and its cars **0x11da..0x11dd** (one update, `0x40f548`, the decomp's
//! `func_L07_0040E198`; two of each placed). The mobys are hidden carriers: what shows is the tie group each carries
//! (`+0xcc`, crate::moby::attach). One set flies a looped path in the sky chunk (1) with the boss fight on it (the
//! boss 0x0e5f, crate::moby::classes::level07::boss, and its dropships, the script [`script`]); the other, without a
//! path, is the parked ship of chunk 0 whose ramp takes Ratchet up there.
//!
//! * **0**: drawn off, hidden (mode 1), the hum's handle `+0x1ac` −1; without a path (`+0x90` −1) the lead to 3, a car
//!   to 4. With one: alpha 0xff, the script's steps cleared (`0x411000`), the convoy set up ([`set_up`], `0x40fd30`),
//!   2. (`0x410ca8`, the three point clouds' centres `0x2a00c0` for the areas' outlines, is left out with them.)
//! * **2**: the sky chunk in place (`0x1c9700`, crate::game::GameState::variant ≠ 0): 1, its class sound 2 looped
//!   (`+0x16cc`) and its lift's rise 6 (`+0x16c8`). (The crash's start on `$gp+0x26f0`, which nothing sets, is left
//!   out.)
//! * **1**: the convoy's move ([`run`], `0x40ffb8`), then the script ([`script`], `0x411038`).
//! * **3**: the parked ship (the lead alone): its first tick its class sound 1; its parts ([`parts`], `0x410480`).
//!   Unless planet 8 is unlocked (`0x1a7bd8`) or its mission done (bit 7): its ramp (`+0x110`) eased toward 0.5
//!   ([`EASE`]); with Ratchet standing on it (`0x18a15c`) toward −1 instead (class sound 1 while it is still past 0.5);
//!   down at −1 the fade out (`0x2cfc50(10, 1)`) and, black, the sky chunk (`0x2fa460(1, 0)`) and Ratchet at
//!   [`UP_AT`] facing π. Done: its parts again, mode |= 2.
//! * **4**: nothing. **5**: the crash ([`crash`]; [`crash_start`] sets it up on the fourth car).
//!
//! pvars: `+0x08` its platform block's offset (0x20; flags `+0x5c` 2, the angles it turned from `+0x40`), `+0x90` the
//! path, `+0x94..+0xa0` moby groups (the enemies aboard), `+0xa8..+0xb4` four mobys it carries (their place and angles
//! in its frame `+0x120` / `+0x160`), `+0xb8` its dropship, `+0xbc` its area, `+0xc4` the boss, `+0xc8` the cuboid it
//! takes its ties at, `+0xcc` the tie group it carries, `+0xd0` the tie group of the wreck, `+0xd4` the crash's line,
//! `+0xd8` the camera, `+0xe0..+0xf0` the cars' places on the path (points), `+0xf4..+0x104` the cars, `+0x108` /
//! `+0x10c` its two parts 0x11e1 / 0x11e2 and their angles `+0x110` / `+0x114` (speed `+0x118`), `+0x11c` the speed
//! (points a tick) toward `+0x1a0` by `+0x1a4`, `+0x1a8` the camera's turn speed, `+0x1ac` the hum, `+0x15b0` eight
//! spinner records, `+0x16b0..+0x16bc` the crash's, `+0x16c0` its lift 0x0d3b, `+0x16c4` / `+0x16c8` the lift's rise
//! and its end, `+0x16cc` the lift's sound.
//!
//! The engine functions: `0x311ae0` set_state, `0x356570` / `0x356618` / `0x356270` the class sound, another class's
//! and the stop (L00 `0x3605f0` / `0x360698` / `0x3602f0`), `0x339f38` = crate::paths::pose, `0x314ef0` approach,
//! `0x315608` = crate::hero::physics::turn_ease, `0x315150` / `0x315888` = ease / ease_speed, `0x314cb0` the lerp,
//! `0x309fd0` rebuild, `0x318730` = crate::moby::platform::write, `0x318638` / `0x3184b0` = platform::to_local /
//! to_world, `0x31a7d0` / `0x31ace0` = crate::moby::MobyWorld::attach / carry, `0x307580` create, `0x320ba8` the
//! owner's place (L00 `0x32a748`), `0x312bb8` = crate::moby::classes::props::group_alive, `0x320c78` / `0x312c98`
//! hooked (one / a group), `0x31c1a8` a point into a moby's frame, `0x2b1f88` = crate::hero::Hero::die, `0x2b7e90` =
//! crate::hero::Hero::teleport, `0x2fa460` = crate::moby::Ctx::switch_chunk, `0x34faa0` the checkpoint, `0x427730`
//! the placed view back on, `0x309508` the mission done, `0x424fb0` / `0x424ee8` / `0x424f40` the script camera and
//! its targets.
//!
//! Not ported: the spinners (`+0x15b0`: the carried ties of the tag `0x227dff`, turned 180° a second by their records
//! `+0x40`, which crate::moby::attach does not read), the hum's place at the camera (`0x1886f0`), the cars' areas
//! following them (`0x410e40`: area `+0xbc`'s outline path and sphere from the point clouds `0x2a00c0`; the port's
//! areas are the level's), the flags `0x1b1998` (the follow camera's), `0x18c0e2` and `0x1b1fa8`, Ratchet's state
//! 0x6f on the crash's teleport, and the crash's wait for its line to start (`0x1a641a` = 3, `0x1a6414` < 6000; then
//! `0x325018`). The cars' time slices (`car >> 8 & 7` of its address) are their index's low bits.

use crate::hero::physics::{add, add_rot, approach, atan2, dist, dist2, dot, ease, ease_speed, sub, turn_ease, V3};
use crate::moby::attach::{angles_of, rows_of, turn};
use crate::moby::classes::props::group_alive;
use crate::moby::{ai, platform, Ctx, Moby, MobyWorld};

/// The lead ship; its cars are `LEAD + 1 ..= LEAD + 4`.
pub const LEAD: i16 = 0x11d9;
pub const CAR_LAST: i16 = 0x11dd;
/// Its two parts and its lift.
const PART: [i16; 2] = [0x11e1, 0x11e2];
const LIFT_CLASS: i16 = 0x0d3b;

/// The convoy's globals.
#[derive(Clone, Debug, Default)]
pub struct Globals {
    /// `0x1b3120`: the script's sixteen steps (0 waiting, 1 under way, 2 done).
    pub steps: [u8; 16],
    /// `$gp−0x1564` (`0x1ada8c`): the car Ratchet rides this tick (the snow and the boss read it).
    pub riding: Option<usize>,
    /// `$gp−0x430`: the snow 0x12ec (crate::moby::classes::level07::snow).
    pub snow: Option<usize>,
    /// `0x1b24d0`: the snow's drift a tick (its flakes add it).
    pub drift: V3,
}

mod pv {
    /// The platform block's offset word (crate::moby::platform::block).
    pub const PTR: usize = 0x08;
    pub const BLOCK: usize = 0x20;
    pub const BLOCK_FLAGS: usize = 0x5c;
    pub const TURNED_FROM: usize = 0x40;
    pub const PATH: usize = 0x90;
    pub const GROUPS: usize = 0x94;
    pub const CARRIED: usize = 0xa8;
    pub const DROPSHIP: usize = 0xb8;
    pub const BOSS: usize = 0xc4;
    pub const CUBOID: usize = 0xc8;
    pub const TIES: usize = 0xcc;
    pub const WRECK: usize = 0xd0;
    pub const LINE: usize = 0xd4;
    pub const CAMERA: usize = 0xd8;
    pub const DIST: usize = 0xe0;
    pub const CARS: usize = 0xf4;
    pub const PARTS: usize = 0x108;
    pub const PART_ANGLE: usize = 0x110;
    pub const PART_SPEED: usize = 0x118;
    pub const SPEED: usize = 0x11c;
    pub const LOCAL: usize = 0x120;
    pub const LOCAL_ROT: usize = 0x160;
    pub const TOP: usize = 0x1a0;
    pub const ACCEL: usize = 0x1a4;
    pub const CAM_TURN: usize = 0x1a8;
    pub const HUM: usize = 0x1ac;
    pub const SINK: usize = 0x16b0;
    pub const TIP_V: usize = 0x16b4;
    pub const GLIDE: usize = 0x16b8;
    pub const SHOOK: usize = 0x16bc;
    pub const LIFT: usize = 0x16c0;
    pub const RISE: usize = 0x16c4;
    pub const RISE_END: usize = 0x16c8;
    pub const LIFT_SOUND: usize = 0x16cc;
}

/// The boss's tier word (`+0x420`, what `0x3f8f80` returns).
const BOSS_TIER: usize = 0x420;

/// `$gp+0x26b0` / `+0x26b4`: the convoy's top speed (a second) and its gain (a second²), over the path's first span.
const TOP: f32 = 60.0;
const ACCEL: f32 = 30.0;
/// `$gp+0x26b8`: the points either side the heading is taken over (`0x410378`).
const HEADING: f32 = 10.0;
/// `$gp+0x2720`: each car's gap behind the one before, in points.
const GAPS: [f32; 5] = [0.0, 32.5, 50.0, 32.5, 40.0];
/// `$gp+0x2734`: the cars' height over the path.
const RIDE: f32 = 9.0;
/// `$gp+0x2740`: the two parts' places in the ship's frame.
const PART_AT: [V3; 2] = [[18.25, 0.0, -2.5], [-16.0, 0.0, -2.5]];
/// `$gp+0x2760`: the lift's place (the lead's, the third car's).
const LIFT_AT: [V3; 2] = [[2.0, 0.0, -3.0], [0.0, 2.0, -3.0]];
/// `0x315608(t, 0.1°, 0.1°, 6°, …)`: the parts' ease (a tick², a tick).
const EASE: (f32, f32) = (0.001_745_329_5, 0.104_719_77);
/// The parked ship's ramp's teleport (191.58, 501.15, 457.69), facing π.
const UP_AT: V3 = [f32::from_bits(0x433f_947b), f32::from_bits(0x43fa_9333), f32::from_bits(0x43e4_d852)];
/// `$gp+0x26c0` / `0x1b16c0`: the second car's dropship's start and target in the car's frame.
const DROP_FROM: V3 = [40.0, 0.0, 21.0];
const DROP_TO: V3 = [-6.0, 0.0, 17.0];
/// `$gp+0x26e0` / `+0x26ec`: Ratchet's place after the fight, and his yaw.
const DOWN_AT: V3 = [f32::from_bits(0x43bf_5d71), f32::from_bits(0x4430_e28f), f32::from_bits(0x4347_bae1)];
const DOWN_YAW: f32 = f32::from_bits(0x4037_ae14);
/// `$gp+0x2784` / `+0x2788`: the camera's turn toward the ridden car (degrees a second², a second).
const CAM_TURN: (f32, f32) = (360.0, 720.0);
/// `$gp+0x2790` / `+0x27a0` / `+0x27b0` / `+0x27c0`: the crashing ship's start and angles, Ratchet's place, the
/// camera's eye.
const CRASH_AT: V3 = [460.0, 707.0, 210.0];
const CRASH_ROT: V3 = [0.0, 0.0, 0.0];
const CRASH_HERO: V3 = [460.0, 707.0, f32::from_bits(0x4357_cccd)];
const CRASH_EYE: V3 = [470.0, 707.0, 222.5];
/// `$gp+0x26f4` / `+0x26f8`: the glide's top speed (a second) and brake (a second²); `+0x26fc` the camera's sink
/// (a second²); `+0x2700` the yaw it glides from; `+0x2708` the camera's roll (degrees); `+0x270c` its fall (a
/// second); `+0x2710` the point it tips about, in its frame.
const GLIDE_MAX: f32 = 60.0;
const GLIDE_BRAKE: f32 = 10.0;
const SINK: f32 = 30.0;
const GLIDE_YAW: f32 = f32::from_bits(0xbf92_1f9f);
const CAM_ROLL: f32 = 10.0;
const CAM_FALL: f32 = -1.0;
const PIVOT: V3 = [0.0, -6.0, -10.0];

const HIDDEN: u16 = 1;
const PI: f32 = std::f32::consts::PI;
const DEG: f32 = 0.017_453_294;

fn is_car(c: i16) -> bool { (LEAD..=CAR_LAST).contains(&c) }

/// `0x40f548` (module docs).
pub fn update_11d9(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !is_car(w.mobys[id].o_class) { return; }
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            m.drawn = 0;
            m.mode |= HIDDEN;
            m.set_pv_i(pv::HUM, -1);
            // The port's: the block's offset word, when the level's pvars leave it unset.
            if m.pv_i(pv::PTR) == 0 { m.set_pv_i(pv::PTR, pv::BLOCK as i32); }
            if m.pv_i(pv::PATH) == -1 {
                let s = if m.o_class == LEAD { 3 } else { 4 };
                m.set_state(s, -1);
            } else {
                m.alpha = 0xff;
                w.convoy.steps = [0; 16];
                set_up(w, id, ctx);
                w.mobys[id].set_state(2, -1);
            }
        }
        1 => {
            run(w, id, ctx);
            script(w, id, ctx);
        }
        2 => {
            if ctx.game.variant != 0 {
                w.mobys[id].set_state(1, -1);
                let h = ctx.host.play_sound(&w.mobys[id], 2, 4);
                let m = &mut w.mobys[id];
                m.set_pv_i(pv::LIFT_SOUND, h);
                m.set_pv_f(pv::RISE_END, 6.0);
            }
        }
        3 => parked(w, id, ctx),
        5 => crash(w, id, ctx),
        _ => {}
    }
}

/// State 3 (module docs).
fn parked(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 1, 0); }
    parts(w, id, ctx);
    let mission = ctx.game.level.max(0) as usize * 0x10 + w.mobys[id].mission as usize;
    let done = ctx.game.planets.get(8).is_some_and(|&p| p != 0) || ctx.game.missions.get(mission).is_some_and(|&b| b & 0x80 != 0);
    if done {
        parts(w, id, ctx);
        w.mobys[id].mode |= crate::moby::mode::NO_UPDATE;
        return;
    }
    let m = &mut w.mobys[id];
    let (mut a, mut v) = (m.pv_f(pv::PART_ANGLE), m.pv_f(pv::PART_SPEED));
    if ctx.hero.ground_moby != Some(id) {
        turn_ease(0.5, EASE.0, EASE.0, EASE.1, &mut a, &mut v, 0);
        m.set_pv_f(pv::PART_ANGLE, a);
        m.set_pv_f(pv::PART_SPEED, v);
        return;
    }
    if 0.5 <= a { ctx.host.play_sound(&w.mobys[id], 1, 0); }
    turn_ease(-1.0, EASE.0, EASE.0, EASE.1, &mut a, &mut v, 0);
    let m = &mut w.mobys[id];
    m.set_pv_f(pv::PART_ANGLE, a);
    m.set_pv_f(pv::PART_SPEED, v);
    if -1.0 < a { return; }
    ctx.game.scenes.fade_out(10.0, 1);
    if 1.0 <= ctx.game.scenes.fade {
        ctx.switch_chunk(1, 0);
        ctx.hero.teleport(UP_AT, [0.0, 0.0, PI]);
    }
}

/// A car in the convoy (`+0xf4 + 4k`).
fn car(w: &MobyWorld, lead: usize, k: usize) -> Option<usize> { ai::link(&w.mobys[lead], pv::CARS + 4 * k).filter(|&c| c < w.mobys.len()) }

/// A moby a pvar word names (−1: none).
fn named(w: &MobyWorld, id: usize, off: usize) -> Option<usize> { usize::try_from(w.mobys[id].pv_i(off)).ok().filter(|&i| i < w.mobys.len()) }

/// `0x31c1a8(m, &out, p − m.pos, 0)`: a point into moby `m`'s frame (its rows' transpose).
fn local(m: &Moby, p: V3) -> V3 {
    let d = sub(p, m.pos);
    [dot(d, m.rows[0]), dot(d, m.rows[1]), dot(d, m.rows[2])]
}

/// `0x2e53d8(v, rows)`: `v` through the rows.
fn through(v: V3, r: &[V3; 3]) -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) }

/// The car's time slice: `0x1b1d88 & 7` against the low bits of its address (here its index).
fn slice(ctx: &Ctx, car: usize) -> bool { ctx.game.frame & 7 == (car & 7) as i32 }

/// `0x320ba8(m, owner)` (L00 `0x32a748`): updated and drawn from any distance, drawn, the owner, its place, angles and
/// light word.
fn adopt(w: &mut MobyWorld, id: usize, owner: usize) {
    let (pos, rot, light) = { let o = &w.mobys[owner]; (o.pos, o.rot, o.light) };
    let m = &mut w.mobys[id];
    m.update_dist = 0xff;
    m.draw_dist = 0xff;
    m.drawn = 1;
    m.parent = Some(owner);
    (m.pos, m.rot, m.light) = (pos, rot, light);
}

/// `0x40fd30`: the convoy set up from the lead — its speed's target and gain over the path's first span, the lead its
/// own first car (place 0), its ties taken ([`take_ties`]), hidden; then for each car class 0x11da..0x11dd in its moby
/// group (`+0x21`): hidden, not updated, a platform (mode |= 0x23, its block at 0x20 with flags 2), scaled as the lead
/// is, its place the last car's plus its gap ([`GAPS`]), its carried mobys taken into its frame (`0x410a68`) and its
/// ties taken. (`0x410948` walks the car's moby groups for nothing.)
fn set_up(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let lead_share = {
        let m = &w.mobys[id];
        m.scale / w.class(m.o_class).map_or(1.0, |c| c.scale)
    };
    // `0x2e50d8(path + 0x10, path + 0x20)`: the path's first span.
    let span = usize::try_from(w.mobys[id].pv_i(pv::PATH)).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| 1 < p.len()).map_or(1.0, |p| {
        dist([p[0][0], p[0][1], p[0][2]], [p[1][0], p[1][1], p[1][2]])
    });
    {
        let m = &mut w.mobys[id];
        m.set_pv_f(pv::SPEED, 0.0);
        m.set_pv_f(pv::ACCEL, 1.0 / span * 0.000_277_777_8 * ACCEL);
        m.set_pv_f(pv::TOP, 1.0 / span * 0.016_666_668 * TOP);
        ai::set_link(m, pv::CARS, Some(id));
        m.set_pv_i(pv::BLOCK_FLAGS, m.pv_i(pv::BLOCK_FLAGS) | 2);
        m.set_pv_f(pv::DIST, 0.0);
        let rot = m.rot;
        m.set_pv_v(pv::TURNED_FROM, rot);
    }
    take_ties(w, id, ctx);
    let m = &mut w.mobys[id];
    m.drawn = 0;
    m.mode |= HIDDEN;
    let Ok(g) = usize::try_from(m.group) else { return };
    let members = w.groups.get(g).cloned().unwrap_or_default();
    for k in 1..5usize {
        for &e in &members {
            if e >= w.mobys.len() || w.mobys[e].o_class != LEAD + k as i16 { continue; }
            let base = w.class(w.mobys[e].o_class).map_or(1.0, |c| c.scale);
            let m = &mut w.mobys[e];
            m.drawn = 0;
            m.mode |= HIDDEN | crate::moby::mode::NO_UPDATE | 0x20;
            m.scale = base * lead_share;
            m.set_pv_i(pv::PTR, pv::BLOCK as i32);
            m.set_pv_i(pv::BLOCK_FLAGS, 2);
            let rot = m.rot;
            m.set_pv_v(pv::TURNED_FROM, rot);
            let lead = &mut w.mobys[id];
            let d = lead.pv_f(pv::DIST + 4 * (k - 1)) + GAPS[k];
            lead.set_pv_f(pv::DIST + 4 * k, d);
            ai::set_link(lead, pv::CARS + 4 * k, Some(e));
            carried_into_frame(w, e);
            take_ties(w, e, ctx);
        }
    }
}

/// `0x410a68`: the four carried mobys' places and angles into the car's frame (`+0x120` / `+0x160`).
fn carried_into_frame(w: &mut MobyWorld, car: usize) {
    for k in 0..4 {
        let Some(a) = named(w, car, pv::CARRIED + 4 * k) else { continue };
        let (pos, rot) = (w.mobys[a].pos, w.mobys[a].rot);
        let Some((l, la)) = platform::to_local(w, car, pos, rot) else { continue };
        let m = &mut w.mobys[car];
        m.set_pv_v(pv::LOCAL + 0x10 * k, l);
        m.set_pv_v(pv::LOCAL_ROT + 0x10 * k, la);
    }
}

/// `0x411938`: the car takes up to 0x40 members of its tie group `+0xcc` (`0x31a7d0(car, block, 0x40, −1, ties, −1)`)
/// as it stands at its cuboid `+0xc8` (`0x1b234c + 0x80·i`: `+0x30` its place, `+0x70` its angles), its place and
/// angles put back after. The spinners' records (its members of the tag `0x227dff`, `+0x40` → `+0x15b0`) are not
/// ported.
fn take_ties(w: &mut MobyWorld, car: usize, ctx: &Ctx) {
    let (pos, rot) = (w.mobys[car].pos, w.mobys[car].rot);
    if let Some((p, r)) = ctx.level.cuboid_place(w.mobys[car].pv_i(pv::CUBOID)) {
        let m = &mut w.mobys[car];
        (m.pos, m.rot) = (p, r);
    }
    w.mobys[car].rows = rows_of(w.mobys[car].rot);
    let ties = w.mobys[car].pv_i(pv::TIES);
    w.attach(car, 0x40, -1, ties, -1);
    let m = &mut w.mobys[car];
    (m.pos, m.rot) = (pos, rot);
    m.rows = rows_of(rot);
}

/// `0x40ffb8`: the convoy's tick. The riding car cleared; the speed toward its target; the hum kept playing (class
/// sound 2, mode 0xc). Each car: placed on the looped path at its place (`0x339f38`) [`RIDE`] up, its pitch and yaw
/// from the points [`HEADING`] either side (`0x410378`: from the one ahead to the one behind), rebuilt, its place on by
/// the speed (wrapped at the path's end), its platform block written and Ratchet carried; ridden (`0x18a1c0` it or
/// one of its parts) it is the riding car and the snow's origin follows it (`0x37a3f0`). Then its carried mobys
/// ([`carry_mobys`]), its ties carried (`0x411a68`), the lead's and the third car's parts ([`parts`]); and Ratchet,
/// off any moby, dies within 1 of the ground or more than 3 under the riding car.
fn run(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.convoy.riding = None;
    let level = ctx.level;
    let Some(points) = usize::try_from(w.mobys[id].pv_i(pv::PATH)).ok().and_then(|p| level.paths.get(p)).filter(|p| 1 < p.len()) else { return };
    let n = points.len() as f32;
    {
        let m = &mut w.mobys[id];
        let mut v = m.pv_f(pv::SPEED);
        approach(m.pv_f(pv::TOP), m.pv_f(pv::ACCEL), &mut v);
        m.set_pv_f(pv::SPEED, v);
    }
    let h = w.mobys[id].pv_i(pv::HUM);
    if !ctx.host.sound_playing(h) {
        let h = ctx.host.play_sound(&w.mobys[id], 2, 0xc);
        w.mobys[id].set_pv_i(pv::HUM, h);
    }
    for k in 0..5 {
        let Some(c) = car(w, id, k) else { continue };
        let t = w.mobys[id].pv_f(pv::DIST + 4 * k);
        let (old, old_rot) = (w.mobys[c].pos, w.mobys[c].rot);
        let (p, pitch, yaw, roll) = crate::paths::pose(points, t, true);
        let mut rot = [roll, pitch, yaw];
        // `0x410378(t, 10, path, &rot)`.
        let back = if t - HEADING < 0.0 { t - HEADING + n } else { t - HEADING };
        let ahead = if n < t + HEADING { t + HEADING - n } else { t + HEADING };
        let a = crate::paths::pose(points, back, true).0;
        let b = crate::paths::pose(points, ahead, true).0;
        rot[2] = atan2(a[0] - b[0], a[1] - b[1]);
        rot[1] = -atan2(dist2(b, a), a[2] - b[2]);
        let pos = [p[0], p[1], p[2] + RIDE];
        {
            let m = &mut w.mobys[c];
            (m.pos, m.rot) = (pos, rot);
        }
        w.rebuild(c, &*ctx.host);
        let speed = w.mobys[id].pv_f(pv::SPEED);
        let mut d = t + speed;
        if n <= d { d -= n; }
        w.mobys[id].set_pv_f(pv::DIST + 4 * k, d);
        platform::write(&mut w.mobys[c], pv::BLOCK, sub(pos, old), old_rot, rot);
        crate::moby::carry_hero_with(ctx, c, 2, old, old_rot[2], pos, rot[2]);
        let plat = ctx.hero.plat.map(|p| p.moby);
        let kids = [ai::link(&w.mobys[c], pv::PARTS), ai::link(&w.mobys[c], pv::PARTS + 4)];
        if plat.is_some() && (plat == Some(c) || kids.contains(&plat)) {
            w.convoy.riding = Some(c);
            snow_follow(w, ctx, d);
        }
        carry_mobys(w, c, ctx);
        w.mobys[c].rows = rows_of(w.mobys[c].rot);
        w.carry(c, &*ctx.host);
        let class = w.mobys[c].o_class;
        if class == LEAD || class == LEAD + 2 { parts(w, c, ctx); }
        let riding = w.convoy.riding;
        let under = riding.is_some_and(|r| local(&w.mobys[r], ctx.hero.pos)[2] < -3.0);
        if ctx.hero.ground_moby.is_none() && (under || ctx.hero.height < 1.0) { ctx.hero.die(); }
    }
}

/// `0x37a3f0(t)`: the snow's origin and axes from its own path (`+0x14`) at `(t + 45) mod 1000` (`+0x50`, the rows
/// `+0x20..+0x50`).
fn snow_follow(w: &mut MobyWorld, ctx: &Ctx, t: f32) {
    let Some(s) = w.convoy.snow.filter(|&s| s < w.mobys.len()) else { return };
    let Some(points) = usize::try_from(w.mobys[s].pv_i(0x14)).ok().and_then(|p| ctx.level.paths.get(p)).filter(|p| 1 < p.len()) else { return };
    let x = t + 45.0;
    let (p, pitch, yaw, roll) = crate::paths::pose(points, x - 1000.0 * (x / 1000.0).trunc(), true);
    let rows = rows_of([roll, pitch, yaw]);
    let m = &mut w.mobys[s];
    for (k, r) in rows.iter().enumerate() { m.set_pv_v(0x20 + 0x10 * k, *r); }
    m.set_pv_v(0x50, p);
}

/// `0x410b10`: the car's four carried mobys put back by its frame (`0x3184b0`) and rebuilt. A Swingshot node 0x2f6
/// among them: Ratchet swinging (state 0xd) to it within 8 makes the car his platform (`0x18a1c0`); else, in the car's
/// time slice and Ratchet not swinging, it is a target unless on the car he rides.
fn carry_mobys(w: &mut MobyWorld, c: usize, ctx: &mut Ctx) {
    for k in 0..4 {
        let Some(a) = named(w, c, pv::CARRIED + 4 * k) else { continue };
        let (l, la) = (w.mobys[c].pv_v(pv::LOCAL + 0x10 * k), w.mobys[c].pv_v(pv::LOCAL_ROT + 0x10 * k));
        if let Some(to) = platform::to_world(w, a, ctx.hero_moby, c, l, la) {
            (w.mobys[a].pos, w.mobys[a].rot) = (to.pos, to.angles);
            if let Some((l, la)) = to.local {
                w.mobys[c].set_pv_v(pv::LOCAL + 0x10 * k, l);
                w.mobys[c].set_pv_v(pv::LOCAL_ROT + 0x10 * k, la);
            }
        }
        w.rebuild(a, &*ctx.host);
        if w.mobys[a].o_class != 0x02f6 { continue; }
        let plat = ctx.hero.plat.map(|p| p.moby);
        if ctx.hero.state == 0xd && ctx.hero.swing.node == Some(a) && plat != Some(c) {
            if dist(ctx.hero.pos, w.mobys[a].pos) < 8.0 { ctx.hero.plat = Some(crate::hero::physics::Plat { moby: c, step: [0.0; 3], flags: 2 }); }
        } else if slice(ctx, c) && ctx.hero.state != 0xd {
            let m = &mut w.mobys[a];
            if w.convoy.riding == Some(c) { m.mode &= !crate::moby::mode::TARGET } else { m.mode |= crate::moby::mode::TARGET }
        }
    }
}

/// `0x410480`: the ship's two parts (made the first time: 0x11e1 / 0x11e2 at it, mode |= 0x120, platforms with flags
/// 2, angles ∓1.2) at [`PART_AT`] in its frame, turned about their y by `+0x110` / `+0x114`, rebuilt, their blocks
/// written; and its lift 0x0d3b (made the first time: 1.1 its class's scale, mode 0x102 without 0x20) while shown: its
/// rise (`+0x16c4`) toward its end (`+0x16c8`) by 1/30 a tick, at the end the lift's sound stopped and the lift hidden;
/// at [`LIFT_AT`] (the lead's, the third car's) up by its rise, turned 90° about z.
fn parts(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (pos, rows) = (w.mobys[id].pos, w.mobys[id].rows);
    let at = |v: V3| add(through(v, &rows), pos);
    for i in 0..2 {
        match ai::link(&w.mobys[id], pv::PARTS + 4 * i).filter(|&p| p < w.mobys.len()) {
            None => {
                let p = w.create(PART[i]);
                adopt(w, p, id);
                let m = &mut w.mobys[p];
                m.mode |= 0x120;
                m.set_pv_i(pv::PTR, pv::BLOCK as i32);
                m.set_pv_i(pv::BLOCK_FLAGS, m.pv_i(pv::BLOCK_FLAGS) | 2);
                m.rows = rows_of(m.rot);
                let me = &mut w.mobys[id];
                ai::set_link(me, pv::PARTS + 4 * i, Some(p));
                me.set_pv_f(pv::PART_ANGLE + 4 * i, if i == 0 { -1.2 } else { 1.2 });
            }
            Some(p) => {
                let (old, old_rot) = (w.mobys[p].pos, w.mobys[p].rot);
                let angle = w.mobys[id].pv_f(pv::PART_ANGLE + 4 * i);
                let new = at(PART_AT[i]);
                {
                    let m = &mut w.mobys[p];
                    m.pos = new;
                    m.rows = turn(&rows_of([0.0, angle, 0.0]), &rows);
                }
                w.rebuild(p, &*ctx.host);
                let m = &mut w.mobys[p];
                m.rot = angles_of(&m.rows);
                let rot = m.rot;
                platform::write(m, pv::BLOCK, sub(new, old), old_rot, rot);
                crate::moby::carry_hero_with(ctx, p, 2, old, old_rot[2], new, rot[2]);
            }
        }
    }
    match ai::link(&w.mobys[id], pv::LIFT).filter(|&l| l < w.mobys.len()) {
        None => {
            let l = w.create(LIFT_CLASS);
            adopt(w, l, id);
            let base = w.class(LIFT_CLASS).map_or(1.0, |c| c.scale);
            let m = &mut w.mobys[l];
            m.scale = base * 1.1;
            m.mode = (m.mode & !0x20) | 0x102;
            ai::set_link(&mut w.mobys[id], pv::LIFT, Some(l));
        }
        Some(l) if w.mobys[l].mode & HIDDEN == 0 => {
            let end = w.mobys[id].pv_f(pv::RISE_END);
            if end != 0.0 {
                let mut r = w.mobys[id].pv_f(pv::RISE);
                approach(end, f32::from_bits(0x3d08_8889), &mut r);
                w.mobys[id].set_pv_f(pv::RISE, r);
                if end <= r {
                    let h = w.mobys[id].pv_i(pv::LIFT_SOUND);
                    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
                    w.mobys[id].set_pv_i(pv::LIFT_SOUND, -1);
                    let m = &mut w.mobys[l];
                    m.drawn = 0;
                    m.mode |= HIDDEN;
                }
            }
            let mut off = LIFT_AT[(w.mobys[id].o_class != LEAD) as usize];
            off[2] += w.mobys[id].pv_f(pv::RISE);
            let new = at(off);
            let m = &mut w.mobys[l];
            m.pos = new;
            m.rows = turn(&rows_of([0.0, 0.0, PI * 0.5]), &rows);
            w.rebuild(l, &*ctx.host);
        }
        Some(_) => {}
    }
}

/// `0x312c98(group, 1)`: a moby group's members hooked (`+0xbe` bit 2).
fn hook_group(w: &mut MobyWorld, g: i32) {
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)).cloned() else { return };
    for i in list { if let Some(m) = w.mobys.get_mut(i) { m.once |= 4; } }
}

/// `0x3eff38(dropship, car, &from, &to)`: a dropship sent off from a car it rides (`+0x110`): shown with its class's
/// collision, state 2, alpha and draw distance 0xff, at `from` in the car's frame, its target `to` (`+0xd0`, in the
/// car's frame as its flight reads it), its place before the bob `+0xf0`; out of the target list.
fn launch(w: &mut MobyWorld, ds: usize, car: usize, from: V3, to: V3) {
    let has = w.class(w.mobys[ds].o_class).is_some_and(|c| c.has_collision);
    let (cp, cr) = (w.mobys[car].pos, w.mobys[car].rows);
    let m = &mut w.mobys[ds];
    m.mode = (m.mode & 0xffbe) | 0x1000;
    m.has_collision = has;
    m.set_state(2, -1);
    m.alpha = 0xff;
    m.draw_dist = 0xff;
    ai::set_link(m, 0x110, Some(car));
    m.pos = add(through(from, &cr), cp);
    m.mode &= 0xefff;
    let p = m.pos;
    m.set_pv_v(0xf0, p);
    m.set_pv_v(0xd0, to);
}

/// The ease of a part's angle toward `t` (`0x315608(t, 0.1°, 0.1°, 6°, &a, &v, 0)`); the angle after.
fn ease_part(m: &mut Moby, slot: usize, t: f32) -> f32 {
    let (mut a, mut v) = (m.pv_f(pv::PART_ANGLE + 4 * slot), m.pv_f(pv::PART_SPEED));
    turn_ease(t, EASE.0, EASE.0, EASE.1, &mut a, &mut v, 0);
    m.set_pv_f(pv::PART_ANGLE + 4 * slot, a);
    m.set_pv_f(pv::PART_SPEED, v);
    a
}

/// Ratchet past `x` (< 0: behind) along car `c`'s first row, in its time slice.
fn behind(w: &MobyWorld, ctx: &Ctx, c: usize, x: f32) -> bool { slice(ctx, c) && local(&w.mobys[c], ctx.hero.pos)[0] < x }

/// `0x411038`: the fight's script on the convoy, its steps `0x1b3120` (each runs until 2). The cars: 0 the lead, 1..4.
///
/// * 0: the lead's enemies (`+0x94`) gone, or Ratchet 14 behind it: its class sound 1 (of 0x11d9, `0x356618`); then
///   its second part (`+0x114`) eased shut (to 0).
/// * 1: the second car's enemies (`+0x94`) below 3: its dropship hooked.
/// * 2: Ratchet on the second car (`0x18a1c0`): its enemies hooked, its dropship sent off from it ([`launch`]).
/// * 3: the second car's other groups (`+0x98`, `+0x9c`) gone, or Ratchet 12 behind it: its dropship away (state 5,
///   `0x3f0148`) and the boss set up on it at tier 0 (crate::moby::classes::level07::boss::set_up).
/// * 4: the boss's tier at 1: the rear three cars drop back (their places down by the speed / 10 a tick) until the
///   third is within 32.5 of the second; then step 5 and the second car's sound 1. Before, once step 3 is done, the
///   camera turned toward the ridden car ([`turn_camera`]).
/// * 5: the third car's first part (`+0x110`) eased open (to 0).
/// * 6: Ratchet standing on the third car (`0x18a15c`): its enemies hooked.
/// * 7: the third car's `+0xa0` group gone: its lift's sound (class sound 2 of 0x11d9, looped) and rise to 6.
/// * 8: as 0 on the third car (its `+0x98` / `+0x9c` groups gone, or 14 behind): then its second part eased shut.
/// * 9: Ratchet standing on the fourth car more than 5.8 over it: the boss set up on it at tier 1.
/// * 10: the boss's tier at 2, 121 ticks into its state, Ratchet neither dying (0x14) nor hurt (7): the fade out
///   (`0x2cfc50(10, 5)`); black with one tick held: held 60 more, chunk 0 (`0x2fa460(0, 0)`), Ratchet at
///   [`DOWN_AT`] (the checkpoint there), the camera `+0xd8` back on, nobody riding, the fourth car's crash
///   ([`crash_start`] with the wreck group `+0xd0`), the mission done and the lead back to 2. Before, once step 9 is
///   done, the camera turned toward the ridden car.
fn script(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let cars: [Option<usize>; 5] = std::array::from_fn(|k| car(w, id, k));
    let boss = named(w, id, pv::BOSS);
    let level = ctx.level;
    for i in 0..16 {
        let step = w.convoy.steps[i];
        if step == 2 { continue; }
        let set = |w: &mut MobyWorld, v: u8| w.convoy.steps[i] = v;
        match i {
            0 | 8 => {
                let Some(c) = cars[if i == 0 { 0 } else { 2 }] else { continue };
                if step == 0 {
                    let m = &w.mobys[c];
                    let clear = if i == 0 { group_alive(w, m.pv_i(pv::GROUPS)) == 0 } else { group_alive(w, m.pv_i(0x98)) == 0 && group_alive(w, m.pv_i(0x9c)) == 0 };
                    if clear || behind(w, ctx, c, -14.0) {
                        ctx.host.play_sound_of(&w.mobys[c], LEAD, 1, 0);
                        set(w, 1);
                    }
                } else if ease_part(&mut w.mobys[c], 1, 0.0) <= 0.0 {
                    set(w, 2);
                }
            }
            1 => {
                let Some(c) = cars[1] else { continue };
                if step == 0 && group_alive(w, w.mobys[c].pv_i(pv::GROUPS)) < 3 {
                    if let Some(d) = named(w, c, pv::DROPSHIP) { w.mobys[d].once |= 4; }
                    set(w, 2);
                }
            }
            2 => {
                let Some(c) = cars[1] else { continue };
                if step == 0 && ctx.hero.plat.map(|p| p.moby) == Some(c) {
                    hook_group(w, w.mobys[c].pv_i(pv::GROUPS));
                    if let Some(d) = named(w, c, pv::DROPSHIP) { launch(w, d, c, DROP_FROM, DROP_TO); }
                    set(w, 2);
                }
            }
            3 => {
                let Some(c) = cars[1] else { continue };
                if step != 0 { continue; }
                let m = &w.mobys[c];
                let clear = group_alive(w, m.pv_i(0x98)) == 0 && group_alive(w, m.pv_i(0x9c)) == 0;
                if clear || behind(w, ctx, c, -12.0) {
                    if let Some(d) = named(w, c, pv::DROPSHIP) { w.mobys[d].set_state(5, -1); }
                    if let Some(b) = boss { super::boss::set_up(w, ctx, b, Some(c), 0); }
                    set(w, 2);
                }
            }
            4 => {
                if step == 0 {
                    if boss.is_some_and(|b| w.mobys[b].pv_i(BOSS_TIER) == 1) {
                        set(w, 1);
                    } else if w.convoy.steps[3] == 2 {
                        turn_camera(w, id, ctx);
                    }
                    continue;
                }
                let n = usize::try_from(w.mobys[id].pv_i(pv::PATH)).ok().and_then(|p| level.paths.get(p)).map_or(0.0, |p| p.len() as f32);
                let m = &mut w.mobys[id];
                let back = m.pv_f(pv::SPEED) * 0.1;
                for k in 2..5 {
                    let mut d = m.pv_f(pv::DIST + 4 * k) - back;
                    if d < 0.0 { d += n; }
                    m.set_pv_f(pv::DIST + 4 * k, d);
                }
                let mut gap = m.pv_f(pv::DIST + 8) - m.pv_f(pv::DIST + 4);
                if gap < 0.0 { gap += n; }
                if gap <= 32.5 {
                    set(w, 2);
                    w.convoy.steps[5] = 1;
                    if let Some(c) = cars[1] { ctx.host.play_sound_of(&w.mobys[c], LEAD, 1, 0); }
                }
            }
            5 => {
                let Some(c) = cars[2] else { continue };
                if step != 0 && 0.0 <= ease_part(&mut w.mobys[c], 0, 0.0) { set(w, 2); }
            }
            6 => {
                let Some(c) = cars[2] else { continue };
                if step == 0 && ctx.hero.ground_moby == Some(c) {
                    hook_group(w, w.mobys[c].pv_i(pv::GROUPS));
                    set(w, 2);
                }
            }
            7 => {
                let Some(c) = cars[2] else { continue };
                if step == 0 && group_alive(w, w.mobys[c].pv_i(0xa0)) == 0 {
                    let h = ctx.host.play_sound_of(&w.mobys[c], LEAD, 2, 4);
                    let m = &mut w.mobys[c];
                    m.set_pv_i(pv::LIFT_SOUND, h);
                    m.set_pv_f(pv::RISE_END, 6.0);
                    set(w, 2);
                }
            }
            9 => {
                let Some(c) = cars[3] else { continue };
                if step == 0 && ctx.hero.ground_moby == Some(c) && 5.8 < local(&w.mobys[c], ctx.hero.ground_point)[2] {
                    if let Some(b) = boss { super::boss::set_up(w, ctx, b, Some(c), 1); }
                    set(w, 2);
                }
            }
            10 => {
                if step == 0 {
                    let ready = boss.is_some_and(|b| w.mobys[b].pv_i(BOSS_TIER) == 2 && 121 <= w.mobys[b].state_ticks as u16);
                    if ready && ctx.hero.state != 0x14 && ctx.hero.state != 7 {
                        set(w, 1);
                    } else if w.convoy.steps[9] == 2 {
                        turn_camera(w, id, ctx);
                    }
                    continue;
                }
                ctx.game.scenes.fade_out(10.0, 5);
                if 1.0 <= ctx.game.scenes.fade && ctx.game.scenes.fade_hold == 1 {
                    ctx.game.scenes.fade_hold = 60;
                    set(w, 2);
                    ctx.switch_chunk(0, 0);
                    let rot = [0.0, 0.0, DOWN_YAW];
                    ctx.hero.teleport(DOWN_AT, rot);
                    let (p, r) = (ctx.hero.pos, ctx.hero.rot);
                    ctx.game.save_checkpoint(p, r);
                    ctx.game.cameras.reset_placed(w.mobys[id].pv_i(pv::CAMERA));
                    w.convoy.riding = None;
                    let wreck = w.mobys[id].pv_i(pv::WRECK);
                    if let Some(c) = cars[3] { crash_start(w, c, wreck, ctx); }
                    let mission = w.mobys[id].mission;
                    ctx.game.mission_done(mission);
                    w.mobys[id].set_state(2, -1);
                }
            }
            _ => {}
        }
    }
}

/// `0x410818`: with the follow camera current (`0x1b91d0 + 0x86` = 0) and a car ridden, the camera turned toward the
/// ridden car's back (its yaw + π; with step 9 not yet started, its front) at [`CAM_TURN`] on the speed `+0x1a8`
/// (crate::camera::CamTweaks::turn_toward, no script bits).
fn turn_camera(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.game.cameras.current_class() != crate::cameras::CLASS_FOLLOW { return; }
    let Some(r) = w.convoy.riding else { return };
    let to = add_rot(w.mobys[r].rot[2], PI);
    let behind = if w.convoy.steps[9] == 0 { PI } else { 0.0 };
    let mut v = w.mobys[id].pv_f(pv::CAM_TURN);
    ctx.game.cam_tweaks.turn_toward(to, behind, CAM_TURN.0 * 4.848_137_5e-6, CAM_TURN.1 * 0.000_290_888_25, &mut v, 0);
    w.mobys[id].set_pv_f(pv::CAM_TURN, v);
}

/// `0x411ab0(car, wreck)`: the crash set up on `car`: state 5, at [`CRASH_AT`] with [`CRASH_ROT`], updated again
/// (mode &= !2), rebuilt; its line asked for (`0x1a63cc` = `(+0xd4 & 0x3fff) + 3000`); its glide's speed `+0x11c`
/// started toward 400 (`0x315888(400 − x, 5, 10/3600, 1)`), the glide 1, the wreck group kept (`+0xd0`); Ratchet at
/// [`CRASH_HERO`]; the script camera snapped in at [`CRASH_EYE`] facing him (1 over his feet); its ties carried; the
/// wreck's ties hidden (`0x1b28d4` byte `+0x1e` bit 4) and the occlusion off (`0x1b1e10` = 0).
fn crash_start(w: &mut MobyWorld, c: usize, wreck: i32, ctx: &mut Ctx) {
    {
        let m = &mut w.mobys[c];
        m.set_state(5, -1);
        (m.pos, m.rot) = (CRASH_AT, CRASH_ROT);
        m.mode &= !crate::moby::mode::NO_UPDATE;
    }
    w.rebuild(c, &*ctx.host);
    let line = (w.mobys[c].pv_i(pv::LINE) & 0x3fff) + 3000;
    ctx.host.request_speech(line);
    let m = &mut w.mobys[c];
    let mut v = m.pv_f(pv::SPEED);
    ease_speed(400.0 - CRASH_AT[0], 5.0, GLIDE_BRAKE * 0.000_277_777_8, GLIDE_MAX * 0.016_666_668, &mut v);
    m.set_pv_f(pv::SPEED, v);
    m.set_pv_i(pv::SHOOK, 0);
    m.set_pv_f(pv::SINK, 0.0);
    m.set_pv_i(pv::WRECK, wreck);
    m.set_pv_f(pv::GLIDE, 1.0);
    ctx.hero.teleport(CRASH_HERO, CRASH_ROT);
    let t = add(ctx.hero.pos, [0.0, 0.0, 1.0]);
    let e = CRASH_EYE;
    let euler = [0.0, atan2(dist2(t, e), e[2] - t[2]), atan2(ctx.hero.pos[0] - e[0], ctx.hero.pos[1] - e[1])];
    ctx.game.cameras.camera_script(e, euler, crate::script_camera::MODE_SNAP, 0, false, ctx.hero);
    w.mobys[c].rows = rows_of(w.mobys[c].rot);
    w.carry(c, &*ctx.host);
    tie_group_hidden(w, wreck, true);
    ctx.game.occlusion = 0;
}

/// A tie group's ties hidden or shown (their info byte `+0x1e` bit 4, `0x1b28d4 + 0x20·i`).
fn tie_group_hidden(w: &mut MobyWorld, g: i32, on: bool) {
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.tie_groups.get(g)).cloned() else { return };
    for t in list {
        if on { w.hidden_ties.insert(t as usize); } else { w.hidden_ties.remove(&(t as usize)); }
    }
}

/// State 5, the crash (on the fourth car). While it glides (`+0x16b8` > 0): x eased to 400 ([`GLIDE_BRAKE`] / 3600,
/// at most [`GLIDE_MAX`] / 60 on `+0x11c`), the glide `(x − 400) / 70`, its yaw and roll from [`GLIDE_YAW`] / 45° to 0
/// by it, its height 213 − 3 × the glide. Glided and the screen clear: it tips over about [`PIVOT`] (its roll's speed
/// `+0x16b4` on by `(roll + 1) · 1/240°` a tick; past 90° held there, bounced back by −1/3, or once slow below 1/30°
/// the fade out (`0x2cfc50(15, 1)`); the first time the camera's roll shake (0.02, 90 ticks)). Black after: the end
/// ([`crash_end`]). Each tick then: the camera's target sinks ([`SINK`] / 3600² toward 1 on `+0x16b0`, with the
/// glide's speed, never up) and falls [`CAM_FALL`] / 60, and looks at it (6 over it, rolled [`CAM_ROLL`]°); its ties
/// carried and its block written. While the glide is done and the screen black the line's start is not waited for
/// (module docs): the hold is let go (`0x1b92b0` = 0).
fn crash(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let fade = ctx.game.scenes.fade;
    if 1.0 <= w.mobys[id].pv_f(pv::GLIDE) && 1.0 <= fade {
        ctx.game.scenes.fade_hold = 0;
    }
    let (p0, r0) = (w.mobys[id].pos, w.mobys[id].rot);
    let glide = w.mobys[id].pv_f(pv::GLIDE);
    if 0.0 < glide {
        let m = &mut w.mobys[id];
        let (mut x, mut v) = (m.pos[0], m.pv_f(pv::SPEED));
        ease(400.0, 0.0, GLIDE_BRAKE * 0.000_277_777_8, GLIDE_MAX * 0.016_666_668, &mut x, &mut v);
        m.pos[0] = x;
        m.set_pv_f(pv::SPEED, v);
        let g = (m.pos[0] - 400.0) * 0.014_285_714;
        m.set_pv_f(pv::GLIDE, g);
        m.rot[2] = GLIDE_YAW + (0.0 - GLIDE_YAW) * g;
        m.rot[0] = 0.785_397_5 + (0.0 - 0.785_397_5) * g;
        m.pos[2] = 213.0 - g * 3.0;
    } else if fade == 0.0 {
        let m = &mut w.mobys[id];
        let before = through(PIVOT, &m.rows);
        let mut tv = m.pv_f(pv::TIP_V) + m.rot[0] * 7.272_206e-5 + 7.272_206e-5;
        m.rot[0] += tv;
        if 1.570_796_4 < m.rot[0] {
            m.rot[0] = 1.570_796_4;
            if tv.abs() < 0.000_581_776_5 { ctx.game.scenes.fade_out(15.0, 1); } else { tv *= -0.333; }
            if m.pv_i(pv::SHOOK) == 0 {
                let s = &mut ctx.game.shakes.0;
                s[2].amp = 0.02;
                s[2].timer = 90;
                s[0].max = 0;
                m.set_pv_i(pv::SHOOK, 1);
            }
        }
        m.set_pv_f(pv::TIP_V, tv);
        m.rows = rows_of(m.rot);
        let after = through(PIVOT, &m.rows);
        m.pos = sub(m.pos, sub(after, before));
    } else if 1.0 <= fade {
        crash_end(w, id, ctx);
        return;
    }
    let m = &mut w.mobys[id];
    let mut s = m.pv_f(pv::SINK);
    approach(1.0, SINK * 0.000_277_777_8, &mut s);
    m.set_pv_f(pv::SINK, s);
    let k = (m.pv_f(pv::SPEED) + s).min(0.0);
    let cam = ctx.host.camera_pos();
    let target = [cam[0] + k, cam[1], cam[2] + CAM_FALL * 0.016_666_668];
    let x = add(m.pos, [0.0, 0.0, 6.0]);
    let euler = [CAM_ROLL * DEG, atan2(dist2(x, cam), cam[2] - x[2]), atan2(m.pos[0] - cam[0], m.pos[1] - cam[1])];
    ctx.game.cameras.camera_script_targets(Some(target), Some(euler));
    m.rows = rows_of(m.rot);
    let (pos, rot) = (m.pos, m.rot);
    w.carry(id, &*ctx.host);
    platform::write(&mut w.mobys[id], pv::BLOCK, sub(pos, p0), r0, rot);
}

/// `0x411ce0`: the crash's end: Ratchet at [`DOWN_AT`] facing [`DOWN_YAW`], the ship not updated and without
/// collision; the wreck's ties shown, the ship's own hidden and sunk 100, the occlusion back (`0x1b1e10` = 2).
fn crash_end(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.hero.teleport(DOWN_AT, [0.0, 0.0, DOWN_YAW]);
    let m = &mut w.mobys[id];
    m.has_collision = false;
    m.mode |= crate::moby::mode::NO_UPDATE;
    let (wreck, ties) = (m.pv_i(pv::WRECK), m.pv_i(pv::TIES));
    tie_group_hidden(w, wreck, false);
    tie_group_hidden(w, ties, true);
    if let Some(list) = usize::try_from(ties).ok().and_then(|g| w.tie_groups.get(g)).cloned() {
        for t in list {
            let t = t as usize;
            if let Some(mat) = w.ties.get_mut(t) { mat[3][2] -= 100.0; }
            if !w.moved_ties.contains(&t) { w.moved_ties.push(t); }
        }
    }
    ctx.game.occlusion = 2;
}
