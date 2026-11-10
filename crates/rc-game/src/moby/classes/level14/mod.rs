//! Level 0x0e's own classes (the prison on Aranos; L14 addresses, the engine functions they call named by their L14
//! addresses with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for
//! asks it after its own.
//!
//! Shared with other levels: the sinking pit 0x135f and its corners 0x1130 (crate::moby::classes::sinking_pit, also
//! L19's), the kill cuboids 0x040f (crate::moby::classes::kill_zone), the target 0x0c87
//! (crate::moby::classes::hit_targets), the fireball 0x0b7f (crate::moby::classes::fireball).

pub mod arcs;
pub mod blocker;
pub mod director;
pub mod energy;
pub mod fence;
pub mod field;
pub mod gate;
pub mod guard;
pub mod hints;
pub mod lift;
pub mod lift_guard;
pub mod movers;
pub mod plate;
pub mod props;
pub mod rappeller;
pub mod roamer;
pub mod screen;
pub mod seller;
pub mod sheet;
pub mod steam;
pub mod switch;

/// This level's draw callbacks (crate::fx): true when the moby's class is one of them.
#[allow(clippy::too_many_arguments)]
pub fn draw(w: &mut crate::moby::MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, _hero_state: i32, _view: &crate::particles::BSphereView, cam: crate::hero::physics::V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        // The dark (`0x428830`) is a first-list callback.
        guard::CLASS => if first { guard::draw(w, id) },
        fence::CLASS => fence::draw(w, id, cam, out),
        field::CLASS => field::draw(w, id, cam, host.camera_rows(), frame),
        sheet::CLASS => sheet::draw(w, id, &mut game.rng, out),
        screen::CLASS => screen::draw(w, id, &mut game.rng, out),
        energy::CLASS => energy::draw(w, id, &mut game.rng, cam, out),
        blocker::CLASS | blocker::CLASS_B => blocker::draw(w, id, out),
        arcs::CLASS => {
            let j = game.arc_jitter.1;
            arcs::draw(w, id, &j, cam, out);
        }
        _ => return false,
    }
    true
}

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        hints::CLASS => Some(hints::update_0c30),
        steam::CLASS => Some(steam::update_0b0d),
        steam::BOBBER => Some(steam::update_0af2),
        steam::SEQUENCER => Some(steam::update_0b19),
        fence::CLASS => Some(fence::update_0f80),
        field::CLASS => Some(field::update_0afe),
        energy::CLASS => Some(energy::update_1164),
        sheet::CLASS => Some(sheet::update_0bff),
        screen::CLASS => Some(screen::update_12b9),
        props::HATCH => Some(props::update_0b11),
        props::FLAP => Some(props::update_0ae7),
        props::BREAKABLE => Some(props::update_1108),
        props::PAD => Some(props::update_0d60),
        rappeller::CLASS => Some(rappeller::update_09b4),
        lift::CLASS | lift::CLASS_B => Some(lift::update_0ae8),
        movers::BRIDGE => Some(movers::update_0ae4),
        blocker::CLASS | blocker::CLASS_B => Some(blocker::update_0aeb),
        lift_guard::CLASS => Some(lift_guard::update_112c),
        // Also Endako's code (L03 `0x38bba8`, none placed there).
        arcs::CLASS => Some(arcs::update_0b10),
        movers::RISER => Some(movers::update_0ae5),
        switch::CLASS => Some(switch::update_113b),
        gate::CLASS => Some(gate::update_113e),
        director::CLASS => Some(director::update_10ff),
        plate::CLASS => Some(plate::update_0f4d),
        super::critter::PRISON => Some(super::critter::update),
        roamer::CLASS => Some(roamer::update_0f91),
        guard::CLASS => Some(guard::update_1100),
        seller::CLASS => Some(seller::update_10f3),
        _ => None,
    }
}
