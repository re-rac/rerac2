//! Level 0x1a's own classes (the Jamming Array; L26 addresses, the engine functions they call named by their L26
//! addresses with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for
//! asks it after its own.

pub mod props;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        props::FLOATER => Some(props::update_0af4),
        _ => None,
    }
}
