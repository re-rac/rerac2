//! Level 0x0a's own classes (Hrugis; L10 addresses, the engine functions they call named by their L10 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.

pub mod sky;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        sky::RING => Some(sky::update_11c8),
        _ => None,
    }
}
