//! Level 0x08's own classes (Tabora; L08 addresses, the engine functions they call named by their L08 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.

pub mod breakables;
pub mod lift;
pub mod props;
pub mod seesaw;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        breakables::CLASS_12AA | breakables::CLASS_129C | breakables::CLASS_129E => Some(breakables::update_12aa),
        breakables::CLASS_12D0 => Some(breakables::update_12d0),
        breakables::CLASS_129F => Some(breakables::update_129f),
        props::PLATFORM => Some(props::update_090e),
        props::GLOWING => Some(props::update_0988),
        props::PICKUP => Some(props::update_0e5b),
        lift::CLASS => Some(lift::update_0aaa),
        seesaw::CLASS => Some(seesaw::update_090d),
        _ => None,
    }
}
