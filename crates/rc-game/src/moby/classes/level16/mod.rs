//! Level 0x10's own classes (Snivelak; L16 addresses, the engine functions they call named by their L16 addresses
//! with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for asks it
//! after its own.

pub mod bolts;
pub mod breakables;
pub mod drawbridge;
pub mod movers;
pub mod props;
pub mod rocks;

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        props::SINKER => Some(props::update_0e73),
        props::POT => Some(props::update_11b5),
        props::BLINKER => Some(props::update_10f0),
        breakables::LAMP => Some(breakables::update_10f1),
        bolts::CLASS => Some(bolts::update_07fb),
        movers::MOVER_A | movers::MOVER_B | movers::MOVER_C => Some(movers::update_10fd),
        drawbridge::BRIDGE => Some(drawbridge::update_125d),
        movers::FLIPPER => Some(movers::update_114c),
        rocks::ROCK => Some(rocks::update_0a80),
        rocks::FACE => Some(rocks::update_0e85),
        _ => None,
    }
}
