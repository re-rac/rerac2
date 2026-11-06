//! Level 0x0e's own classes (the prison on Aranos; L14 addresses, the engine functions they call named by their L14
//! addresses with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for
//! asks it after its own.

pub mod props;
pub mod steam;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        steam::CLASS => Some(steam::update_0b0d),
        steam::BOBBER => Some(steam::update_0af2),
        steam::SEQUENCER => Some(steam::update_0b19),
        props::HATCH => Some(props::update_0b11),
        props::FLAP => Some(props::update_0ae7),
        _ => None,
    }
}
