//! Level 0x10's own classes (Snivelak; L16 addresses, the engine functions they call named by their L16 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.

pub mod props;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        props::SINKER => Some(props::update_0e73),
        props::POT => Some(props::update_11b5),
        props::BLINKER => Some(props::update_10f0),
        _ => None,
    }
}
