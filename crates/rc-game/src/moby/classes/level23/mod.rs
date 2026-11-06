//! Level 0x17's own classes (Damosel's orbit; L23 addresses, the engine functions they call named by their L23
//! addresses with the port's). Each class's update is registered in [`update_for`]; crate::moby::classes::update_for
//! asks it after its own. The classes this level shares with Dobbo's orbit (the same code at its own places) are
//! crate::moby::classes::level22's.

/// This level's class updates.
pub fn update_for(o_class: i16) -> Option<super::UpdateFn> {
    match o_class {
        // Giant Clank's body (L23 `0x3ae5c8`).
        super::level22::giant::BODY => Some(super::level22::giant::update_027d),
        _ => None,
    }
}
