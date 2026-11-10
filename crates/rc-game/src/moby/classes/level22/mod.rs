//! Level 0x16's own classes (Dobbo's orbit; L22 addresses, the engine functions they call named by their L22
//! addresses with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for
//! asks it after its own. Shared with Damosel orbit (level 23): the debris (crate::moby::classes::orbit_debris: 0x0aa9's
//! update is L23's 0x1213's; 0x0dce and 0x0cb6 there too), the asteroid clusters 0x0d3d (crate::moby::classes::asteroid_cluster) and
//! their rocks 0x0a80 (crate::moby::classes::thrown_rock), the pickups 0x0e3a / 0x0e3b / 0x0e3d
//! (crate::moby::classes::orbit_pickups), and the classes crate::moby::classes::update_for lists for both orbits.

pub mod boss;
pub mod director;
pub mod drone;
pub mod pickups;
pub mod planet;
pub mod props;
pub mod shot;
pub mod special;

/// The fight's globals its classes and Giant Clank share (crate::moby::MobyWorld::level22).
#[derive(Clone, Debug)]
pub struct Globals {
    /// `$gp+0x2d4`: the opponent 0x0e0b ([`boss`]), set at its set-up.
    pub boss: Option<usize>,
    /// `0x1b2a28`: its most health; `0x1b2a24`: the planetoid's radius ([`planet`]).
    pub most: i32,
    pub radius: f32,
    /// `0x1b1890`: the opponent's record (Giant Clank's is crate::hero::giant::GiantBlock::counts, [`pickups`]).
    pub boss_counts: [i32; 4],
    pub pickups: pickups::Pickups,
    /// `$gp+0x9cc`: the opponent's special ([`special`]); `$gp+0x9c0` / `+0x9c4`: Giant Clank's and its sounds.
    pub boss_special: Option<usize>,
    pub special_sound: [i32; 2],
    /// `0x1b2a14..0x1b2a20`: the damages the opponent keeps from its pvars `+0x330..` ([`shot`], [`special`]).
    pub damage: [f32; 4],
    /// `$gp+0x2a0`: the scene trigger 0x0f6e the opponent waits for; `$gp+0x2a4`: the wait over ([`boss`]).
    pub trigger: Option<usize>,
    pub started: bool,
    /// `$gp+0x2c4`: the intro's ticks (0 out of it).
    pub intro: i32,
    /// `0x1b2a30`: the hit-landed timer (its double punch's, [`boss`]); `0x1b2a10`: a timer counted down;
    /// `0x1b2a08`: the fight's phase; `$gp+0x354`: the phase changed this tick; `0x1b2a2c`: the dropped 0x0f8f's height.
    pub immune: i32,
    pub timer: i32,
    pub phase: i32,
    pub flag: bool,
    pub hat: f32,
    /// `$gp+0x370` (its count `0x1b2a54`): the skip table; `$gp+0x384`: the scan's entry; the slots' link bits at
    /// their starts and ends (the paths' end points' w) ([`planet`]).
    pub skip: [i32; 5],
    pub skip_n: i32,
    pub scan_at: i32,
    pub links: [[u32; 2]; 32],
    /// `0x1b2a0c`: the drones come back so far in phase 2 (at most 16, [`drone`]; the timer between them `0x1b2a10`).
    pub drones_back: i32,
}

impl Default for Globals {
    fn default() -> Self {
        Globals {
            boss: None, most: 0, radius: 0.0, boss_counts: [0; 4], pickups: Default::default(), boss_special: None, special_sound: [-1; 2], damage: [0.0; 4],
            trigger: None, started: false, intro: 0, immune: 0, timer: 0, phase: 0, flag: false, hat: 0.0, skip: planet::SKIP, skip_n: 5, scan_at: 0, links: [[0; 2]; 32],
            drones_back: 0,
        }
    }
}

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        props::WATCHER => Some(props::update_12db),
        props::HIDDEN => Some(props::update_0a6c),
        director::CLASS => Some(director::update_133a),
        boss::CLASS => Some(boss::update_0e0b),
        drone::CLASS => Some(drone::update),
        shot::CLASS => Some(shot::update),
        special::CLASS => Some(special::update),
        _ => None,
    }
}
