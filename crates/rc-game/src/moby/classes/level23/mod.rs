//! Level 0x17's own classes (Damosel's orbit; L23 addresses, the engine functions they call named by their L23
//! addresses with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for
//! asks it after its own. The classes this level shares with Dobbo's orbit (the same code at its own places): Giant
//! Clank's body 0x027d (crate::moby::classes::giant_body), the opponent's pointer 0x0d55
//! (crate::moby::classes::opponent_pointer), the drone shot's update the 0x118e runs (crate::moby::classes::orbit_shot);
//! the pickups 0x0e3a / 0x0e3b / 0x0e3d are crate::moby::classes::level22's.
//!
//! The fight: the opponent 0x0f7f ([`boss`]) hovers over the planetoid ([`paths`]) circling Giant Clank, its shield
//! of five generators 0x0f7e and its core 0x120f ([`targets`]) taking his hits for it, its turret firing 0x118e and
//! its missiles 0x1230 ([`shots`]), launching its 25 walkers 0x07e5 ([`drones`]) and at the end calling down a ring
//! of lightning; the pickups lie about ([`pickups`]); Giant Clank's own shots 0x118f and special 0x1190 are this
//! level's ([`shots`], [`special`]).

pub mod boss;
pub mod drones;
pub mod effects;
pub mod paths;
pub mod pickups;
pub mod shots;
pub mod special;
pub mod targets;

use crate::hero::physics::V3;

/// The fight's words its classes and Giant Clank share (crate::moby::MobyWorld::level23; L23 v2 addresses).
#[derive(Clone, Debug, Default)]
pub struct Globals {
    /// `$gp+0x2444`: the opponent 0x0f7f, set at its set-up.
    pub boss: Option<usize>,
    /// `$gp+0x23e4`: the intro's ticks (0 out of it; past 200 the opponent's update stops there).
    pub intro: i32,
    /// `0x1b3624`: the planetoid's radius; `0x1b36e0` / `0x1b36f0`: the points of its surface under (500, 500, 430)
    /// and (500, 500, 552) (the arena's middle and its top).
    pub radius: f32,
    pub middle: V3,
    pub top: V3,
    /// `0x1b3660`: the point the shots aim at on the opponent (its shield's generator, or 4 up its row 2).
    pub aim: V3,
    /// `0x1b3640` / `0x1b3650`: Giant Clank's chest (8 up his row 2) and his middle (4 up).
    pub chest: V3,
    pub mid: V3,
    /// The level's paths as the fight keeps them ([`paths`]).
    pub paths: paths::Paths,
    /// `0x2594e0`: the walkers 0x07e5 (25, placed); `$gp+0x260c` the next to launch; `0x1b362c` how many are out;
    /// `0x1b3630` the one Giant Clank is fighting.
    pub drones: Vec<usize>,
    pub next_drone: i32,
    pub drones_out: i32,
    pub selected: Option<usize>,
    /// `$gp+0x2610`: the walkers' first launch still to do (set at the opponent's set-up; ten come out, then cleared).
    pub relaunch: bool,
    /// The opponent's pvars the others read (kept at its set-up): `0x1b3670` the walkers' health (`+0x31c`),
    /// `0x1b3674` their missiles' damage (`+0x308`), `0x1b3678` their beam's (`+0x30c`), `0x1b3680` Giant Clank's
    /// shots' (`+0x310`), `0x1b3684` the specials' (`+0x314`), `0x1b3688` the riders' turn (`+0x330`), `0x1b368c` the
    /// walkers' missile speed (`+0x338`).
    pub drone_health: i32,
    pub drone_missile: f32,
    pub drone_beam: f32,
    pub shot_damage: f32,
    pub special_damage: f32,
    pub rider_turn: f32,
    pub drone_missile_speed: f32,
    /// `$gp+0x2388` / `$gp+0x238c`: a generator's health and its floor; `$gp+0x468c`: the core's health ([`targets`]).
    pub f7e_health: f32,
    pub f7e_floor: f32,
    pub t120f_health: f32,
    /// `$gp+0x2654` / `$gp+0x2658` (180): the special's count and its top ([`boss`]'s `0x41d5e8`, [`targets`],
    /// [`effects`]); `$gp+0x2664` / `$gp+0x2668`: its flags; `0x1b369c` its blast's wait.
    pub hero_guard: i32,
    pub hero_guard_top: i32,
    pub guard_hit: bool,
    pub guard_again: bool,
    pub guard_wait: i32,
    /// `0x1b3694`: the opponent's hurt sound's wait.
    pub hurt_wait: i32,
    /// `0x1b139c`: its guard was up this tick; `0x1b13e0`: its guard down with a generator to face.
    pub guarded: bool,
    pub open: bool,
    /// `0x1b13c0`: it moved this tick (its speed eased, else stopped).
    pub moved: bool,
    /// `0x1b13dc`: the wait between walkers launched in state 7; `0x1b36d8`: the launches' count.
    pub launch_wait: i32,
    pub launches: i32,
    /// `0x1b1494`: the distance it keeps from Giant Clank (`0x41ce60`).
    pub circle: f32,
    /// `0x1b3690`: Giant Clank above its height.
    pub above: bool,
    /// `$gp+0x23f8` / `0x1b36a4`: the turret's hatch (0 shut, 1 opening, 2 open) and its count.
    pub hatch: i32,
    pub hatch_t: i32,
    /// `0x1b36a8`: the turret's way's dot with the way to Giant Clank; `0x1b3704`: the line from it to him clear;
    /// `0x1b37d0` / `0x1b37f0`: its muzzle and its way; `0x1b37e0` / `0x1b3800`: the points of the line test.
    pub aim_dot: f32,
    pub aim_clear: bool,
    pub muzzle: V3,
    pub muzzle_way: V3,
    pub test_from: V3,
    /// `0x1b3700`: the ring of lightning under way; `0x1b370c` its radius, `$gp+0x2550` its growth, `0x1b3708` its
    /// ticks; `0x1b3720` its middle; `0x259360` its 8 × 3 points.
    pub ring: bool,
    pub ring_r: f32,
    pub ring_grow: f32,
    pub ring_t: i32,
    pub ring_at: V3,
    pub ring_pts: [[V3; 3]; 8],
    /// `$gp+0x23e8`: the death's pieces flying; `0x1b36a0` their count.
    pub breaking: bool,
    pub break_n: i32,
    /// `$gp+0x2390`: the scene trigger 0x0f6e it sets off at its end.
    pub trigger: Option<usize>,
    /// The pickups ([`pickups`]).
    pub pickups: pickups::Pickups,
    /// `$gp+0x2914` / `$gp+0x2918`: Giant Clank's special 0x1190 and the opponent's ([`special`]); `$gp+0x290c` /
    /// `$gp+0x2910` their sounds.
    pub special: [Option<usize>; 2],
    pub special_sound: [i32; 2],
    /// `0x1b3898` / `$gp+0x29bc`: the shock waves' five slots and how many are out ([`shots::wave`]).
    pub waves: [bool; 5],
    pub waves_n: i32,
}

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        // Giant Clank's body (L23 `0x3ae5c8`).
        boss::CLASS => Some(boss::update_0f7f),
        targets::F7E => Some(targets::update_0f7e),
        targets::T120F => Some(targets::update_120f),
        effects::C1251 => Some(effects::update_1251),
        effects::C1252 => Some(effects::update_1252),
        drones::CLASS => Some(drones::update_07e5),
        // The turret's shot (L23 `0x423980`: the drone shot's update, its hit L23's own).
        shots::SHOT => Some(shots::update_118f),
        shots::MISSILE => Some(shots::update_1230),
        shots::DRONE_MISSILE => Some(shots::update_1237),
        special::CLASS => Some(special::update_1190),
        shots::WAVE => Some(shots::update_11df),
        _ => None,
    }
}
