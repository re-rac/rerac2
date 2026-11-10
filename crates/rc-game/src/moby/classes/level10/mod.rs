//! Level 0x0a's own classes (Hrugis; L10 addresses, the engine functions they call named by their L10 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.
//!
//! Ported here: 0x0cbe ([`tethered`]), 0x11c9 ([`sky`]), the carrier 0x0f85 ([`carrier`]), the cloud glints 0x11b9
//! ([`glints`]); the escort 0x0f82 (`0x417600`) is the shared crate::moby::classes::space::escort, the mission 0x1155
//! the shared crate::moby::classes::space::mission; the wingman 0x0f93 (Gorn's too, L15 `0x421538`) is registered with
//! crate::moby::classes::update_for (its numbers stay in [`ships`]).

pub mod carrier;
pub mod glints;
pub mod ships;
pub mod sky;
pub mod tethered;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        sky::RING => Some(sky::update_11c8),
        sky::LIGHT => Some(sky::update_11c9),
        glints::CLASS => Some(glints::update),
        tethered::CLASS => Some(tethered::update),
        ships::FIGHTER => Some(super::space::update_fighter),
        ships::STATION => Some(ships::update_station),
        ships::BEACON => Some(ships::update_beacon),
        carrier::CLASS => Some(carrier::update),
        // The mission (`0x422828`, the three ship levels' one system).
        super::space::mission::HRUGIS => Some(super::space::mission::update),
        _ => None,
    }
}

/// This level's space-AI ships' numbers (crate::moby::classes::space's `Spec`).
pub(crate) fn ship_spec(o_class: i16) -> Option<super::space::Spec> { if o_class == carrier::CLASS { return Some(carrier::spec()); } ships::spec(o_class) }

/// This level's draw callbacks (crate::fx's `one()`); false when `id` is not one of them.
#[allow(clippy::too_many_arguments)]
pub fn draw(w: &mut crate::moby::MobyWorld, host: &dyn crate::moby::MobyHost, game: &mut crate::game::GameState, _hero_state: i32, _view: &crate::particles::BSphereView, cam: crate::hero::physics::V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, _frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    if first { return false; }
    match w.mobys[id].o_class {
        tethered::CLASS => tethered::draw(w, host, id, &mut game.rng, cam, out),
        sky::LIGHT => sky::draw_11c9(w, id, &mut game.rng, cam, out),
        _ => return false,
    }
    true
}
