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
//! * **0x0cbe** (`0x3f7418`, 16 placed): the targets riding a joint of another moby (`+0x260`, joint `+0x26c`), 50
//!   along its second row, turning (0.67° a tick), with a draw callback of its own (`0x3f76e8`).
//! * **0x0f82** (`0x417600`): the escort along the mission's route.

pub mod ships;
pub mod sky;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        sky::RING => Some(sky::update_11c8),
        // 0xf93 is Gorn's too (the same code, L15 `0x421538`).
        ships::FIGHTER | ships::WINGMAN => Some(super::space::update_fighter),
        ships::STATION => Some(ships::update_station),
        ships::BEACON => Some(ships::update_beacon),
        _ => None,
    }
}

/// This level's space-AI ships' numbers (crate::moby::classes::space's `Spec`).
pub(crate) fn ship_spec(o_class: i16) -> Option<super::space::Spec> { ships::spec(o_class) }
