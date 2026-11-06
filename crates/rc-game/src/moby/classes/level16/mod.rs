//! Level 0x10's own classes (Snivelak; L16 addresses, the engine functions they call named by their L16 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.

pub mod bolts;
pub mod breakables;
pub mod copter;
pub mod drawbridge;
pub mod drone;
pub mod gate;
pub mod laser;
pub mod movers;
pub mod props;
pub mod rocks;
pub mod turret;

/// This level's draw callbacks (crate::fx): true when the moby's class is one of them.
#[allow(clippy::too_many_arguments)]
pub fn draw(w: &mut crate::moby::MobyWorld, _host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, _hero_state: i32, _view: &crate::particles::BSphereView, cam: crate::hero::physics::V3, id: usize, _first: bool, out: &mut Vec<crate::fx::FxQuad>, _frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        copter::SHOT => copter::draw_0ebd(w, id, &mut game.rng, cam, out),
        drone::SHOT => drone::draw_0f7a(w, id, &mut game.rng, cam, out),
        laser::CLASS => laser::draw(w, id, cam, out),
        gate::CLASS => gate::draw(w, id, &mut game.rng, out),
        _ => return false,
    }
    true
}

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        props::SINKER => Some(props::update_0e73),
        laser::CLASS => Some(laser::update_1162),
        gate::CLASS => Some(gate::update),
        props::POT => Some(props::update_11b5),
        props::BLINKER => Some(props::update_10f0),
        breakables::LAMP => Some(breakables::update_10f1),
        bolts::CLASS => Some(bolts::update_07fb),
        movers::MOVER_A | movers::MOVER_B | movers::MOVER_C => Some(movers::update_10fd),
        drawbridge::BRIDGE => Some(drawbridge::update_125d),
        movers::FLIPPER => Some(movers::update_114c),
        rocks::ROCK => Some(rocks::update_0a80),
        rocks::FACE => Some(rocks::update_0e85),
        copter::CLASS => Some(copter::update_0ebb),
        copter::SHOT => Some(copter::update_0ebd),
        drone::CLASS => Some(drone::update_0f35),
        drone::SHOT => Some(drone::update_0f7a),
        turret::GUN => Some(turret::update_10ee),
        _ => None,
    }
}
