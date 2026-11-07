//! Level 0x08's own classes (Tabora; L08 addresses, the engine functions they call named by their L08 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.

pub mod arcer;
pub mod breakables;
pub mod buyer;
pub mod breather;
pub mod burrower;
pub mod counter;
pub mod crystal;
pub mod glider;
pub mod hovership;
pub mod lift;
pub mod mission;
pub mod props;
pub mod seesaw;
pub mod tabora_ai;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        // Also level 19's (L19 `0x42afa8`, the same code and data).
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
        props::GLOWING => Some(props::update_0988),
        props::PICKUP => Some(props::update_0e5b),
        // Also levels 9 (0x0d49, L09 `0x401350`) and 19 (0x1176, L19 `0x41fcb8`): the same code.
        lift::CLASS | 0x0d49 | 0x1176 => Some(lift::update_0aaa),
        mission::CLASS => Some(mission::update_0c2a),
        // Also level 19's (L19 `0x414848` / `0x3c1cc8`, the same code and data).
        hovership::HOVERSHIP => Some(hovership::update_0eef),
        hovership::RARITANIUM => Some(hovership::update_099a),
        seesaw::CLASS => Some(seesaw::update_090d),
        // Also levels 9 and 19's (L09 `0x3bfb08`, L19 `0x3bded8`; the counter L19 `0x3c5b50`): the same code and data.
        glider::CLASS => Some(glider::update_0905),
        counter::CLASS => Some(counter::update_09f3),
        crystal::CRYSTAL => Some(crystal::update_096a),
        crystal::LEFT => Some(crystal::update_08c4),
        crystal::CARRIER => Some(crystal::update_0e78),
        // The chips: the same code as Level 12's (`0x3d49b8` = L12 `0x400850`).
        crystal::CHIP | crystal::CHIP_B => Some(super::level12::debris::update_0d66),
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
