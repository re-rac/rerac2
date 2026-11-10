//! Level 0x09's own classes (Dobbo; L09 addresses, the engine functions they call named by their L09 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.
//!
//! Shared with other levels: the gunship 0x0cbc (crate::moby::classes::gunship) and its bolt 0x05de
//! (crate::moby::classes::gunship_bolt), both also L20's; the pads 0x0efc and 0x0e80 (crate::moby::classes::pads); the texture
//! scroller 0x0d06 (crate::moby::classes::tex_scroll, also L16's).

pub mod breakables;
pub mod critter;
pub mod director;
pub mod flyer;
pub mod hover_pad;
pub mod lamp;
pub mod leader;
pub mod mission;
pub mod patrol;
pub mod props;
pub mod seller;
pub mod talker;
pub mod towers;
pub mod waker;

/// This level's draw callbacks (crate::fx): true when the moby's class is one of them.
#[allow(clippy::too_many_arguments)]
pub fn draw(w: &mut crate::moby::MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, _hero_state: i32, view: &crate::particles::BSphereView, cam: crate::hero::physics::V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, _frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        towers::MANAGER if !first => crate::fx::beams::draw_dobbo(w, id, game, view, cam, out),
        critter::CLASS => critter::draw(w, id, cam, out),
        // The drawer's group draw (`0x3ff960`) and a robot's spot (`0x3fec60`), both on the second list.
        patrol::CLASS if !first && w.mobys[id].state == 5 => patrol::draw_group(w, host, id, view, cam, out),
        patrol::CLASS if !first => patrol::draw_spot(w, id, cam, out),
        _ => return false,
    }
    true
}

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        talker::CLASS => Some(talker::update_0e5e),
        critter::CLASS => Some(critter::update_0d90),
        patrol::CLASS => Some(patrol::update_0d30),
        // Also level 12's (L12 `0x408a28`, the same code and data).
        hover_pad::CLASS => Some(hover_pad::update_0e84),
        breakables::CRYSTAL => Some(breakables::update_1298),
        breakables::BIG_CRYSTAL => Some(breakables::update_10e7),
        props::SPINNER => Some(props::update_0c21),
        props::SWAY_A | props::SWAY_B => Some(props::update_0bf4),
        props::WHEEL => Some(props::update_0a14),
        props::FIELD => Some(props::update_0e60),
        props::MOVER => Some(props::update_094d),
        waker::CLASS => Some(waker::update_0de0),
        flyer::CLASS => Some(flyer::update_0d57),
        director::CLASS => Some(director::update_0c2b),
        mission::CLASS => Some(mission::update_0d9d),
        towers::MANAGER => Some(towers::update_0d12),
        towers::TOWER => Some(towers::update_0cea),
        towers::NODE => Some(towers::update_0d09),
        lamp::CLASS => Some(lamp::update_0292),
        leader::CLASS => Some(leader::update_0e1a),
        seller::CLASS => Some(seller::update_0e10),
        _ => None,
    }
}
