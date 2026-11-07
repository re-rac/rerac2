//! Level 0x0a's own classes (Hrugis; L10 addresses, the engine functions they call named by their L10 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.
//!
//! Not ported yet (the space fight's, [`ships`] has the rest):
//! * **0x1155** (`0x422828`): the mission (its phases, the help texts; it sets the space-AI library's sphere, ready
//!   flag and hit handler `0x4252b0`, crate::moby::classes::space::hits_with).
//! * **0x0f85** (`0x417c88`, 6 placed): the carrier — two parts (`0x419280`), a sway, its own flight round the
//!   fight (`0x418b68`), its guns (`0x418d88` / `0x418fd8`) and the wingmen it docks and sends along a spline
//!   (state 2, `0x336748`).
//!
//! Ported here: 0x0cbe ([`tethered`]), 0x11c9 ([`sky`]); the escort 0x0f82 (`0x417600`) is the shared
//! crate::moby::classes::space::escort.

pub mod ships;
pub mod sky;
pub mod tethered;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        sky::RING => Some(sky::update_11c8),
        sky::LIGHT => Some(sky::update_11c9),
        tethered::CLASS => Some(tethered::update),
        // 0xf93 is Gorn's too (the same code, L15 `0x421538`).
        ships::FIGHTER | ships::WINGMAN => Some(super::space::update_fighter),
        ships::STATION => Some(ships::update_station),
        ships::BEACON => Some(ships::update_beacon),
        _ => None,
    }
}

/// This level's space-AI ships' numbers (crate::moby::classes::space's `Spec`).
pub(crate) fn ship_spec(o_class: i16) -> Option<super::space::Spec> { ships::spec(o_class) }

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
