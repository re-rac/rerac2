//! Level 0x11's own classes (level 17; L17 addresses, the engine functions they call named by their L17 addresses with
//! the L00 ones the port knows them by where they differ). Each class's update is registered in [`update_for`] and
//! its draw callback, if any, in [`draw`]; crate::moby::classes::update_for and crate::fx ask these after their own.

pub mod floaters;
pub mod gate;
pub mod hatchling;
pub mod hazard;
pub mod props;

use crate::hero::physics::V3;
use crate::moby::MobyWorld;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        hazard::CHAIN | hazard::CHAIN_B => Some(hazard::update_0878),
        props::CONVEYOR => Some(props::update_0fbd),
        props::CRATE => Some(props::update_0fd7),
        props::LAMP => Some(props::update_10ec),
        hatchling::HATCHLING => Some(hatchling::update_0b88),
        hatchling::POD | hatchling::POD_OPEN => Some(hatchling::update_0920),
        floaters::BUOY => Some(floaters::update_08af),
        floaters::TIPPER => Some(floaters::update_0937),
        gate::CLASS => Some(gate::update_1305),
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx's `one`): true when the class has one here.
#[allow(clippy::too_many_arguments, unused_variables)]
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, hero_state: i32, view: &crate::particles::BSphereView, cam: V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        _ => false,
    }
}
