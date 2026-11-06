//! Level 0x12's own classes (Damosel; L18 addresses, the engine functions they call named by their L18 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own. The chain is global, so a class whose code levels 19 and 20 share is registered here once.

pub mod breeder;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        breeder::CLASS => Some(breeder::update_0b69),
        _ => None,
    }
}
