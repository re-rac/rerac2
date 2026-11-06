//! Level 0x11's own classes (level 17; L17 addresses, the engine functions they call named by their L17 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.

pub mod carrier;
pub mod brute;
pub mod floaters;
pub mod gate;
pub mod hatchling;
pub mod hazard;
pub mod lobber;
pub mod lifts;
pub mod misc;
pub mod mortar;
pub mod props;
pub mod seller;

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        seller::CLASS => Some(seller::update_11a6),
        hazard::CHAIN | hazard::CHAIN_B => Some(hazard::update_0878),
        props::CONVEYOR => Some(props::update_0fbd),
        props::CRATE => Some(props::update_0fd7),
        props::LAMP => Some(props::update_10ec),
        hatchling::HATCHLING => Some(hatchling::update_0b88),
        hatchling::POD | hatchling::POD_OPEN => Some(hatchling::update_0920),
        floaters::BUOY => Some(floaters::update_08af),
        floaters::TIPPER => Some(floaters::update_0937),
        gate::CLASS => Some(gate::update_1305),
        // Also level 8's (L08 `0x3f7ab0`).
        mortar::CLASS => Some(mortar::update_0c1a),
        brute::CLASS => Some(brute::update_0bcb),
        // Also level 12's (L12 `0x3e3838`) and its glob 0x0ea5.
        lobber::CLASS => Some(lobber::update_0bbf),
        lobber::SHOT => Some(lobber::update_0ea5),
        carrier::CARRIER => Some(carrier::update_08b1),
        carrier::PLATFORM => Some(carrier::update_093b),
        lifts::DOCK => Some(lifts::update_0935),
        lifts::LIFT_A | lifts::LIFT_B => Some(lifts::update_08ac),
        lifts::ELEVATOR => Some(lifts::update_08a7),
        misc::MARKER_A | misc::MARKER_B | misc::MARKER_C => Some(misc::update_0877),
        misc::UPDRAFT => Some(misc::update_120d),
        misc::BARRIER => Some(misc::update_1256),
        misc::SWITCH => Some(misc::update_1326),
        misc::MUSIC => Some(misc::update_1272),
        misc::SCENE_FX => Some(misc::update_0f27),
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx's `one`): true when the class has one here.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        carrier::CARRIER => { carrier::draw_08b1(w, id, cam, out); true }
        _ => false,
    }
}
