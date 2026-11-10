//! Level 0x13's own classes (level 19; L19 addresses, the engine functions they call named by their L19 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.
//!
//! Shared with other levels: the hovership 0x0eef and its raritanium 0x099a (crate::moby::classes::hovership, L08),
//! the glider 0x0905 and its pad 0x0988 (crate::moby::classes::glider / glider_pad), the breeding critter 0x0b69
//! (crate::moby::classes::breeder) and the step pad 0x0ed0 (crate::moby::classes::pads; L18, L20), the floating
//! platform 0x0f4b (L20's 0x1257) and the movers 0x0edb / 0x1146 (L18's 0x10ed) (crate::moby::classes::floe), the
//! fading field 0x1204 (crate::moby::classes::fading_field, L20's 0x119b). Grelbin's riser 0x1271: [`riser`].

pub mod den;
pub mod director;
pub mod fly_by;
pub mod frozen;
pub mod lurker;
pub mod props;
pub mod ride_cam;
pub mod riser;
pub mod switches;

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

/// The level's globals its classes share.
#[derive(Clone, Debug, Default)]
pub struct Globals {
    /// `$gp+0x2cd0`: the lurkers attacking (at most lurker::MOST_ATTACKERS; emptied by each one's set-up).
    pub attackers: i32,
    /// `$gp+0x2f6c` / `+0x2f70`: the level's census (director::census).
    pub census: crate::moby::director::Census,
}

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        props::WATCHER => Some(props::update_0629),
        fly_by::CLASS => Some(fly_by::update_0c35),
        props::ICE => Some(props::update_11c7),
        props::TELEPORTER => Some(props::update_0edc),
        props::DROPPER => Some(props::update_11ff),
        props::TURN_LIFT => Some(props::update_114d),
        props::REVEALED => Some(props::update_1270),
        props::LASER => Some(props::update_11f0),
        props::SCENE_HIDDEN => Some(props::update_1331),
        lurker::CLASS => Some(lurker::update_10d0),
        den::CLASS => Some(den::update_1177),
        frozen::CLASS => Some(frozen::update_0f57),
        riser::CLASS => Some(riser::update_1271),
        switches::CLEARER => Some(switches::update_1205),
        switches::SPINNER => Some(switches::update_11f4),
        switches::PLANET_SCENE => Some(switches::update_1267),
        director::TALKER => Some(director::update_1254),
        director::TRADER => Some(director::update_1313),
        ride_cam::CLASS => Some(ride_cam::update_1258),
        // The liquid lid 0x12b4 (L19 `0x422c70`) is crate::water::lid, the sinking pit 0x135f (L19 `0x383478`) and its
        // corners crate::moby::classes::sinking_pit.
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx's `one`): true when the class has one here.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        frozen::CLASS if first => {
            frozen::draw(w, host, id, &mut game.rng, cam, out);
            true
        }
        props::LASER if first => {
            props::draw_11f0(w, id, cam, out);
            true
        }
        _ => false,
    }
}
