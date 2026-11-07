//! Level 0x0f's own classes (Gorn; L15 addresses). Each class's update is registered in [`update_for`];
//! crate::moby::classes::update_for asks it after its own. Gorn's wingmen 0x0f93 (`0x421538`, 32 placed) are
//! Hrugis's code (crate::moby::classes::level10::ships registers them); the space-AI library here is at
//! `0x3812f8..` (crate::moby::classes::space), its hit handler `0x431ad8` (crate::moby::classes::space::hits_with).
//!
//! Not ported yet (level 15's own list):
//! * **0x119f** (`0x42f288`): the mission (its phases and help texts; it sets the library's sphere, ready flag and
//!   hit handler).
//! * **0x1211** (`0x435ec0`, 4 placed): the big ships — their flight (`0x4363d0`), the cloak's fade in and out (states
//!   2..5, `+0x23` 32 ↔ 128 over 180 ticks), the fall at their death (states 6 / 7).
//! * **0x1274** (`0x4398d8`): draw callbacks only (`0x439ff8`, `0x43a020`, `0x43a2d8`).
//!
//! Ported here: 0x1216 ([`convoy`]), 0x1279 ([`ring`]).

pub mod convoy;
pub mod ring;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        convoy::CLASS => Some(convoy::update),
        ring::CLASS => Some(ring::update),
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
        _ => return None,
    })
}
