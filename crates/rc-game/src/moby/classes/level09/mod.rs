//! Level 0x09's own classes (Dobbo; L09 addresses, the engine functions they call named by their L09 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.

pub mod breakables;
pub mod props;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        breakables::CRYSTAL => Some(breakables::update_1298),
        breakables::BIG_CRYSTAL => Some(breakables::update_10e7),
        props::SPINNER => Some(props::update_0c21),
        props::SWAY_A | props::SWAY_B => Some(props::update_0bf4),
        props::WHEEL => Some(props::update_0a14),
        props::FIELD => Some(props::update_0e60),
        props::MOVER => Some(props::update_094d),
        props::PAD => Some(props::update_0efc),
        props::LAUNCH_PAD => Some(props::update_0e80),
        0x0ccb => Some(super::level04::misc::update_empty),
        _ => None,
    }
}
