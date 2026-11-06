//! Level 0x05's own classes (Feltzin System; L05 addresses). Each class's update is registered in [`update_for`];
//! crate::moby::classes::update_for asks it after its own. The space fight's ships run on the space-AI library
//! (crate::moby::classes::space, the decomp's `func_L05_0037…`, here at `0x37aad0..`): [`ships`].
//!
//! Not ported yet (level 5's own list):
//! * **0x1103** (`0x41efc0`): the mission (the waves, the cuts, the end; it sets the library's sphere, ready flag
//!   and hit handler `0x421c18`, crate::moby::classes::space::hits_with).
//! * **0x0f82** (`0x418668`, all three ship levels): the escort along the mission's route (the mission's path table).
//! * **0x0fbf** (`0x4192e8`): the player's ship's own update (crate::ship flies the ship).

pub mod ships;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        ships::FIGHTER | ships::CRUISER | ships::FLAMER => Some(super::space::update_fighter),
        ships::CARRIER => Some(ships::update_carrier),
        _ => None,
    }
}

/// This level's space-AI ships' numbers (crate::moby::classes::space's `Spec`).
pub(crate) fn ship_spec(o_class: i16) -> Option<super::space::Spec> { ships::spec(o_class) }
