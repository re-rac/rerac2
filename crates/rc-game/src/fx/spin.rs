//! Quads spun about an axis (the decomp's `func_L01_002E1510`, no L00 place: L02 `0x2f07e8`, L13 `0x2f78d0`, L16
//! `0x2e6040`, on 13 levels): `0x2e1510(packets, n, turn, steps, frame)` turns the last two corners of each of the `n`
//! quad packets by `turn` once, then `steps` times turns one pair of corners by the turn twice over (the first pair on
//! even steps, the second on odd ones: the pair turned last leads) and draws every packet (`0x280b90(packet, frame, 1)`:
//! its corners through `frame`, additive). With a turn of −π/k, the steps lay a ring of `steps` segments of a
//! (2π/k)-wide arc each, starting a turn in.
//!
//! The packets are made by the decomp's `func_L02_002EF968` (L02 `0x2f0750`, L13 `0x2f7838`; [`Packet::new`]).
//!
//! Its callers: Maktar's lock 0x0c81 (L02 `0x413460`, crate::moby::classes::maktar::draw_0c81) and the barrier 0x112e
//! (L13 `0x4210b8` / L16 `0x41aa00`, crate::moby::classes::barrier::draw). Not ported (their own draws): the
//! telepad's beam (`0x4126b0` and its places on six more levels), Oozla's beam (`0x399f40`, L08 / L09 / L12 / L17),
//! the Maktar pads (`0x3e36b8`; L12 `0x3d0890`), Endako's swirl (L03 `0x3cd248`), level 7's nest (`0x3e63b0`) and L20
//! `0x39d290`.

use crate::fx::FxQuad;
use crate::hero::physics::V3;

/// One quad packet (0x90 bytes): its FX texture, colour, ST and corners (GS strip order) in the frame it is drawn by.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Packet {
    pub fx: usize,
    pub rgba: u32,
    pub st: [[f32; 2]; 4],
    pub v: [V3; 4],
}

impl Packet {
    /// `func_L02_002EF968(packet, fx, rgba, st, corners, 1)` (all its callers pass 1: additive).
    pub fn new(fx: usize, rgba: u32, st: [[f32; 2]; 4], v: [V3; 4]) -> Self { Packet { fx, rgba, st, v } }
}

/// `v` times the rows `r` (`0x283098`).
fn times(v: V3, r: &[V3; 3]) -> V3 { std::array::from_fn(|k| v[0] * r[0][k] + v[1] * r[1][k] + v[2] * r[2][k]) }

/// `0x2e1510` (module docs): `turn` the rows of the turn (`0x2833f0` of its angles), `rows` / `pos` the frame.
pub fn spin(out: &mut Vec<FxQuad>, packets: &mut [Packet], turn: &[V3; 3], steps: usize, rows: &[V3; 3], pos: V3) {
    for p in packets.iter_mut() {
        p.v[2] = times(p.v[2], turn);
        p.v[3] = times(p.v[3], turn);
    }
    // `0x283738(twice, turn, turn)`.
    let twice = turn.map(|row| times(row, turn));
    for i in 0..steps {
        let k = if i & 1 != 0 { 2 } else { 0 };
        for p in packets.iter_mut() {
            p.v[k] = times(p.v[k], &twice);
            p.v[k + 1] = times(p.v[k + 1], &twice);
            let corners = p.v.map(|c| { let w = times(c, rows); [w[0] + pos[0], w[1] + pos[1], w[2] + pos[2]] });
            out.push(FxQuad { fx: p.fx, additive: true, corners, st: p.st, rgba: [p.rgba; 4] });
        }
    }
}
