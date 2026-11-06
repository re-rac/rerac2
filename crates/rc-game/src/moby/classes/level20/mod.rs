//! Level 0x14's own classes (Yeedil; L20 addresses, the engine functions they call named by their L20 addresses with
//! the L00 ones the port knows them by). Each class's update is registered in [`update_for`];
//! crate::moby::classes::update_for asks it after its own.

pub mod biter;
pub mod boss;
pub mod fighter;
pub mod misc;
pub mod props;
pub mod shooter;
pub mod strafer;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        biter::CLASS => Some(biter::update_119e),
        boss::CLASS => Some(boss::update_11e0),
        fighter::CLASS => Some(fighter::update_12f3),
        shooter::CLASS => Some(shooter::update_12ef),
        strafer::CLASS => Some(strafer::update_12f8),
        props::SLIDER => Some(props::update_124f),
        props::CRYSTAL => Some(props::update_1269),
        props::FIELD => Some(props::update_119b),
        props::NEST => Some(props::update_11ec),
        // `0x446c78`: `jr ra`.
        0x11ed => Some(super::level04::misc::update_empty),
        misc::SLAB => Some(misc::update_11ef),
        misc::LIFT => Some(misc::update_1246),
        misc::CRATE => Some(misc::update_131a),
        misc::TRIGGER => Some(misc::update_04f7),
        misc::BOX => Some(misc::update_0f88),
        misc::PAD => Some(misc::update_1209),
        misc::BRIDGE => Some(misc::update_11e8),
        // `0x449298`: L19's floating platform `0x4168b8`, the same code.
        0x1257 => Some(super::level19::floe::update_0f4b),
        _ => None,
    }
}
