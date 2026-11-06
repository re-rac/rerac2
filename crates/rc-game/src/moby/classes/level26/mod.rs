//! Level 0x1a's own classes (the Jamming Array; L26 addresses, the engine functions they call named by their L26
//! addresses with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for
//! asks it after its own.

pub mod node;
pub mod props;
pub mod spinner;

/// This level's draw callbacks (crate::fx): true when the moby's class is one of them.
#[allow(clippy::too_many_arguments)]
pub fn draw(w: &mut crate::moby::MobyWorld, _host: &dyn crate::moby::MobyHost, _game: &mut crate::game::GameState, _hero_state: i32, _view: &crate::particles::BSphereView, _cam: crate::hero::physics::V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, _frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        props::MOTE if first => props::draw_0c00(w, id, out),
        _ => return false,
    }
    true
}

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        props::FLOATER => Some(props::update_0af4),
        props::MOTE => Some(props::update_0c00),
        node::CLASS => Some(node::update_0a79),
        spinner::CLASS => Some(spinner::update_0c8e),
        _ => None,
    }
}
