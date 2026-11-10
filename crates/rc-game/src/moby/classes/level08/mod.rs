//! Level 0x08's own classes (Tabora; L08 addresses, the engine functions they call named by their L08 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.
//!
//! Shared with other levels: Ratchet's glider 0x0905 (crate::moby::classes::glider; L09, L19) and its launch pad
//! 0x0988 (crate::moby::classes::glider_pad), the crystals' counter 0x09f3 (crate::moby::classes::gem_counter; L19's
//! moonstones), the hovership 0x0eef and its raritanium 0x099a (crate::moby::classes::hovership; L19).

pub mod arcer;
pub mod breakables;
pub mod buyer;
pub mod breather;
pub mod burrower;
pub mod crystal;
pub mod geyser;
pub mod lift;
pub mod mission;
pub mod props;
pub mod seesaw;
pub mod tabora_ai;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        // Level 19's table runs the same code (L19 `0x42afa8`), but there 0x0e3e is only a scene actor (no update).
        buyer::CLASS => Some(buyer::update_0e3e),
        breakables::CLASS_12AA | breakables::CLASS_129C | breakables::CLASS_129E => Some(breakables::update_12aa),
        breakables::CLASS_12D0 => Some(breakables::update_12d0),
        breakables::CLASS_129F => Some(breakables::update_129f),
        burrower::CLASS => Some(burrower::update_0fff),
        arcer::CLASS => Some(arcer::update_1002),
        arcer::COVER => Some(arcer::update_0987),
        arcer::MANAGER => Some(arcer::update_1006),
        breather::CLASS => Some(breather::update_09c6),
        props::PLATFORM => Some(props::update_090e),
        props::PICKUP => Some(props::update_0e5b),
        // Also levels 9 (0x0d49, L09 `0x401350`) and 19 (0x1176, L19 `0x41fcb8`): the same code.
        lift::CLASS | 0x0d49 | 0x1176 => Some(lift::update_0aaa),
        mission::CLASS => Some(mission::update_0c2a),
        seesaw::CLASS => Some(seesaw::update_090d),
        crystal::CRYSTAL => Some(crystal::update_096a),
        crystal::LEFT => Some(crystal::update_08c4),
        crystal::CARRIER => Some(crystal::update_0e78),
        c if geyser::CLASSES.contains(&c) => Some(geyser::update),
        // The liquid lid 0x11fe is crate::water::lid.
        _ => None,
    }
}

/// This level's draw callbacks (crate::fx): true when the moby's class is one of them (the beam manager 0x1006's,
/// crate::fx::beams; the liquid lid is crate::water::lid).
#[allow(clippy::too_many_arguments)]
pub fn draw(w: &mut crate::moby::MobyWorld, _host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, _hero_state: i32, _view: &crate::particles::BSphereView, cam: crate::hero::physics::V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, _frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        arcer::MANAGER if !first => crate::fx::beams::draw_tabora(w, id, game, cam, out),
        _ => return false,
    }
    true
}
