//! Level 0x12's own classes (Damosel; L18 addresses, the engine functions they call named by their L18 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own. The chain is global, so a class whose code levels 19 and 20 share is registered here once. The
//! Hypnomatic's creatures 0x0ec0 / 0x0ee1 and their shot 0x1263: crate::moby::classes::capturable.

pub mod breakables;
pub mod breeder;
pub mod director;
pub mod dropship;
pub mod hatch;
pub mod hints;
pub mod launcher;
pub mod pad;
pub mod prey;
pub mod props;
pub mod seller;
pub mod spinner;
pub mod stalker;
pub mod traffic;

/// This level's draw callbacks (crate::fx's, the second list).
#[allow(clippy::too_many_arguments)]
pub fn draw(w: &mut crate::moby::MobyWorld, host: &dyn crate::moby::MobyHost, _game: &mut crate::game::GameState, _hero_state: i32, _view: &crate::particles::BSphereView, cam: crate::hero::physics::V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, _frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    match w.mobys[id].o_class {
        prey::CLASS if !first => prey::draw(w, host, id, cam, out),
        _ => return false,
    }
    true
}

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        seller::CLASS => Some(seller::update_1292),
        breeder::CLASS => Some(breeder::update_0b69),
        breakables::PANEL_A | breakables::PANEL_B | breakables::PANEL_C => Some(breakables::update_122c),
        breakables::GLOW_PANEL => Some(breakables::update_1184),
        breakables::BIT_PANEL => Some(breakables::update_1186),
        breakables::BOLT_PANEL => Some(breakables::update_12a5),
        hatch::CLASS => Some(hatch::update_1151),
        prey::CLASS => Some(prey::update_0ef4),
        stalker::CLASS => Some(stalker::update_0f0a),
        // The body 0x0f0b (`0x436980`, L09's `0x3f85e8`): crate::moby::classes::trooper::update_turret, registered there.
        dropship::CLASS => Some(dropship::update_0ede),
        // Level 20's 0x1212 runs the same code (`0x447010`).
        props::ARCS => Some(props::update_1212),
        props::PANEL => Some(props::update_12a6),
        props::SLIDER_A | props::SLIDER_B | props::SLIDER_C | props::SLIDER_D => Some(props::update_11af),
        props::NEST => Some(props::update_0f06),
        props::SHELL => Some(props::update_12bf),
        props::ORB => Some(props::update_124a),
        props::SWING => Some(props::update_0f54),
        director::CLASS => Some(director::update_0f01),
        // Also levels 19 and 20's (L19 `0x40efd0`, L20 `0x432580`, the same code).
        pad::CLASS => Some(pad::update_0ed0),
        launcher::CLASS => Some(launcher::update_1158),
        hints::CLASS => Some(hints::update_0c34),
        spinner::CLASS => Some(spinner::update_10f8),
        _ => None,
    }
}
