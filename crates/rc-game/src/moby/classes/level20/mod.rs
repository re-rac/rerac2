//! Level 0x14's own classes (Yeedil; L20 addresses, the engine functions they call named by their L20 addresses with
//! the L00 ones the port knows them by). Each class's update is registered in [`update_for`];
//! crate::moby::classes::update_for asks it after its own.

pub mod biter;
pub mod boss;
pub mod fighter;
pub mod props;
pub mod shooter;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        biter::CLASS => Some(biter::update_119e),
        boss::CLASS => Some(boss::update_11e0),
        fighter::CLASS => Some(fighter::update_12f3),
        shooter::CLASS => Some(shooter::update_12ef),
        props::SLIDER => Some(props::update_124f),
        props::CRYSTAL => Some(props::update_1269),
        props::FIELD => Some(props::update_119b),
        props::NEST => Some(props::update_11ec),
        _ => None,
    }
}
