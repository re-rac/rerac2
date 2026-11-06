//! Level 0x0e's own classes (the prison on Aranos; L14 addresses, the engine functions they call named by their L14
//! addresses with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for
//! asks it after its own.

pub mod gate;
pub mod lift;
pub mod movers;
pub mod props;
pub mod rappeller;
pub mod steam;
pub mod switch;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        steam::CLASS => Some(steam::update_0b0d),
        steam::BOBBER => Some(steam::update_0af2),
        steam::SEQUENCER => Some(steam::update_0b19),
        props::HATCH => Some(props::update_0b11),
        props::FLAP => Some(props::update_0ae7),
        rappeller::CLASS => Some(rappeller::update_09b4),
        lift::CLASS | lift::CLASS_B => Some(lift::update_0ae8),
        movers::BRIDGE => Some(movers::update_0ae4),
        movers::RISER => Some(movers::update_0ae5),
        switch::CLASS => Some(switch::update_113b),
        gate::CLASS => Some(gate::update_113e),
        super::critter::PRISON => Some(super::critter::update),
        _ => None,
    }
}
