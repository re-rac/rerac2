//! Level 0x16's own classes (Dobbo's orbit; L22 addresses, the engine functions they call named by their L22
//! addresses with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for
//! asks it after its own.

pub mod asteroid;
pub mod debris;
pub mod giant;
pub mod director;
pub mod props;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        debris::ROCK | debris::ROCK_B | debris::ICE => Some(debris::update_rock),
        debris::DUST => Some(debris::update_0cb6),
        asteroid::CLUSTER => Some(asteroid::update),
        props::HIDDEN => Some(props::update_0a6c),
        giant::BODY => Some(giant::update_027d),
        director::CLASS => Some(director::update_133a),
        _ => None,
    }
}
