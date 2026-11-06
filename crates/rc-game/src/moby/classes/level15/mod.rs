//! Level 0x0f's own classes (Gorn; L15 addresses). Each class's update is registered in [`update_for`];
//! crate::moby::classes::update_for asks it after its own. Gorn's wingmen 0x0f93 (`0x421538`, 32 placed) are
//! Hrugis's code (crate::moby::classes::level10::ships registers them); the space-AI library here is at
//! `0x3812f8..` (crate::moby::classes::space), its hit handler `0x431ad8` (crate::moby::classes::space::hits_with).
//!
//! Not ported yet (level 15's own list):
//! * **0x119f** (`0x42f288`): the mission (its phases and help texts; it sets the library's sphere, ready flag and
//!   hit handler).
//! * **0x1279** (`0x43ae60`, 10 placed): a ring round the nearest tie group (`0x2aee70`'s circling), its wires
//!   (`0x33ccd8` / `0x33d1e8`, to the group's 0x0d33 / 0x0d34 ties) turning.
//! * **0x1211** (`0x435ec0`, 4 placed): the big ships — their flight (`0x4363d0`), the cloak's fade in and out (states
//!   2..5, `+0x23` 32 ↔ 128 over 180 ticks), the fall at their death (states 6 / 7).
//! * **0x1216** (`0x438750`, 1 placed): the convoy along its path (`0x1fe400[+0x268]`: the AI's state 0xc toward
//!   each point, a point's odd w a wait of (w >> 1) seconds), placed at the path's start by its leaders' callback
//!   (`0x438a28`, the mission's call); its radar mark (`0x35b558`).
//! * **0x1274** (`0x4398d8`): draw callbacks only (`0x439ff8`, `0x43a020`, `0x43a2d8`).
//! * **0x0f82** (`0x420eb0`): the escort along the mission's route.

/// This level's class updates (none of its own yet).
pub fn update_for(_o_class: i16) -> Option<super::UpdateFn> { None }
