//! The mobys Ratchet rides (his state 0x31 with `0x18b650` set): the vehicles of Tabora and Grelbin (the hovership
//! 0x0eef, crate::moby::classes::level08::hovership; the glider 0x0905 of Tabora, Dobbo and Grelbin,
//! crate::moby::classes::level08::glider, with Ratchet seen) and the mounted guns of level 16 (0x10ee,
//! crate::moby::classes::level16::turret). Endako's crane (crate::moby::classes::endako_crane) uses the same state
//! and the same word `0x18b650` for the crane it steers from its panel (kept there as its own `steered`).
//!
//! **The hero's side** is small: the state 0x31 (SetState L08 `0x2cb8f0` case 0x31, as L00's: group 9, `0x18c0dd`
//! the hero's probe ahead off, the idle sequence held; crate::hero::script) holds him while the moby moves him; the
//! moby writes his place and angles every tick (`0x189ea0` / `0x189eb0`) and asks for his sequence. A vehicle also
//! sets the view lock `0x18c0d5` (his model and items hidden: the vehicle's model has its pilot) every tick it is
//! steered. Leaving is the moby's too: it puts him back in state 0 (`0x2c4560(0, 1)`, or L16's `0x2c45f8(0)`).
//!
//! **The block** (`0x18b650..0x18b668`): `0x18b650` the moby ridden ([`VehicleBlock::moby`]), `0x18b654` set 0xffff
//! on the hovership's way off, `0x18b65c` / `0x18b660` the vehicle's health and its most (the HUD's bar `0x18b668`,
//! 0..200, [`VehicleBlock::bar`]). The classes that look at what he rides (Tabora's raritanium 0x099a, Grelbin's
//! camera 0x1258) read [`Hero::riding`].
//!
//! The port's: his own velocity is cleared each time a moby places him ([`Hero::ride_place`]), as the state 0x31's
//! physics adds none (crate::hero::script) and the move would carry a velocity he came in with.
//!
//! Not ported: the hero moby's `+0x9c` (−1 while on, 0 at the way off: nothing in the port reads it), the hovership's
//! `0x189f28` (1/6 at the way off) and `0x33e830()`.

use super::physics::{euler_rows, V3};
use super::Hero;

/// The vehicle block (module docs).
#[derive(Clone, Copy, Debug)]
pub struct VehicleBlock {
    /// `0x18b650`: the moby he rides (None: none).
    pub moby: Option<usize>,
    /// `0x18b654` (u16): 0xffff once off the hovership.
    pub word_654: u16,
    /// `0x18b65c` / `0x18b660`: the vehicle's health and its most.
    pub health: f32,
    pub max_health: f32,
    /// `0x18b668`: the HUD's vehicle bar (0..200; 200 while not riding).
    pub bar: i32,
    /// `0x1382ac`: □'s pressure (0..1) as the hero tick last read the pad (the hovership's brake).
    pub square: f32,
    /// `0x138290..0x13829c`: the d-pad's pressures (0..1: →, ←, ↑, ↓) as the hero tick last read the pad (the glider's
    /// steering, crate::moby::classes::level08::glider).
    pub dpad: [f32; 4],
    /// L16 `$gp+0x2b00` / `0x1b1af4`: the mounted gun's filtered stick (one pair for all the guns).
    pub stick_filter: [f32; 2],
}

impl Default for VehicleBlock {
    fn default() -> Self { VehicleBlock { moby: None, word_654: 0, health: 0.0, max_health: 1.0, bar: 200, square: 0.0, dpad: [0.0; 4], stick_filter: [0.0; 2] } }
}

impl Hero {
    /// What he rides: state 0x31 with `0x18b650` set.
    pub fn riding(&self) -> Option<usize> { if self.state == 0x31 { self.vehicle.moby } else { None } }

    /// A moby takes him (the hovership's `0x2d31f8(0x31, play)`, the gun's `0x2b9680(pos, rot, 0x31, 0)`): state 0x31
    /// asked for (`play`: its idle sequence started), `0x18b650` = `moby`, placed at `pos` facing `rot`.
    pub fn ride_on(&mut self, moby: usize, pos: V3, rot: V3, play: bool) {
        self.request_state(0x31, play);
        self.vehicle.moby = Some(moby);
        self.ride_place(pos, rot);
    }

    /// The moby's write of his place and angles (`0x189ea0` / `0x189eb0`) while he rides (module docs).
    pub fn ride_place(&mut self, pos: V3, rot: V3) {
        self.pos = pos;
        self.rot = rot;
        self.rows = euler_rows(rot);
        self.vel = [0.0; 3];
        self.momentum = [0.0; 3];
    }

    /// The moby lets him go: `0x18b650` cleared, the view lock off, idle (`0x2c4560(0, 1)`).
    pub fn ride_off(&mut self) {
        self.vehicle.moby = None;
        self.view_lock = 0;
        if self.mode == 0 { self.request_state(0, true); }
    }
}
