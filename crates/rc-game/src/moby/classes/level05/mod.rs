//! Level 0x05's own classes (Feltzin System; L05 addresses). Each class's update is registered in [`update_for`];
//! crate::moby::classes::update_for asks it after its own. The space fight's ships run on the space-AI library
//! (crate::moby::classes::space, the decomp's `func_L05_0037…`, here at `0x37aad0..`): [`ships`].
//!
//! The mission 0x1103 (`0x41efc0`) is the shared crate::moby::classes::space::mission, the escort 0x0f82 (`0x418668`)
//! the shared crate::moby::classes::space::escort.
//!
//! Not ported yet (level 5's own list):
//! * **0x0fbf** (`0x4192e8`): the player's ship's own update (crate::ship flies the ship).

pub mod ships;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        ships::FIGHTER | ships::CRUISER | ships::FLAMER => Some(super::space::update_fighter),
        ships::CARRIER => Some(ships::update_carrier),
        // The mission (`0x41efc0`, the three ship levels' one system).
        super::space::mission::FELTZIN => Some(super::space::mission::update),
        _ => None,
    }
}

/// This level's space-AI ships' numbers (crate::moby::classes::space's `Spec`).
pub(crate) fn ship_spec(o_class: i16) -> Option<super::space::Spec> { ships::spec(o_class) }
