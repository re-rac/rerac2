//! Level 0x0f's own classes (Gorn; L15 addresses). Each class's update is registered in [`update_for`];
//! crate::moby::classes::update_for asks it after its own. Gorn's wingmen 0x0f93 (`0x421538`, 32 placed) are
//! Hrugis's code (crate::moby::classes::level10::ships registers them); the space-AI library here is at
//! `0x3812f8..` (crate::moby::classes::space), its hit handler `0x431ad8` (crate::moby::classes::space::hits_with).
//!
//! Ported here: 0x1216 ([`convoy`]), 0x1279 ([`ring`]), the big ships 0x1211 ([`bigship`]), the storm 0x1274 and its
//! bolts 0x11aa ([`storm`]); the mission 0x119f is the shared crate::moby::classes::space::mission.

pub mod bigship;
pub mod convoy;
pub mod ring;
pub mod storm;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        convoy::CLASS => Some(convoy::update),
        ring::CLASS => Some(ring::update),
        bigship::CLASS => Some(bigship::update),
        storm::CLASS => Some(storm::update),
        storm::BOLT => Some(storm::update_bolt),
        // The mission (`0x42f288`, the three ship levels' one system).
        super::space::mission::GORN => Some(super::space::mission::update),
        _ => None,
    }
}

/// This level's space-AI ships' numbers (crate::moby::classes::space's `Spec`).
pub(crate) fn ship_spec(o_class: i16) -> Option<super::space::Spec> {
    use super::space::{Spec, FIGHTER_SPEC};
    let base = Spec { target: 0x260, leads: 0x264, gun: 0x268, flyby: 0, engine: None, death: None, gun_sound: None, nozzles: &[], ..FIGHTER_SPEC };
    Some(match o_class {
        // 0x1216: left where it is (`0x37c130` does not move it), health 100.
        convoy::CLASS => Spec { health: 100.0, scale: 0.2, scatter: false, ..base },
        bigship::CLASS => bigship::spec(),
        _ => return None,
    })
}

/// This level's draw callbacks (crate::fx's `one()`); false when `id` is not one of them.
#[allow(clippy::too_many_arguments)]
pub fn draw(w: &mut crate::moby::MobyWorld, _host: &dyn crate::moby::MobyHost, _game: &mut crate::game::GameState, _hero_state: i32, _view: &crate::particles::BSphereView, cam: crate::hero::physics::V3, id: usize, first: bool, out: &mut Vec<crate::fx::FxQuad>, _frame: &mut Vec<crate::fx::FrameQuad>) -> bool {
    if first || w.mobys[id].o_class != storm::CLASS { return false; }
    storm::draw(w, id, cam, out);
    true
}
