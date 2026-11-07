//! The gravity frames (G-LVL-003): R&C2's gravity spheres — places where "down" is toward (or away from) a point
//! instead of world −z, so Ratchet walks round a small world. One global flag (`0x1a8ff0` = `$gp − 0x6000`) turns
//! them on; Ratchet's frame mode (`0x18c0e3`) and his gravity direction (`0x18a0e0`) follow from it each tick, and
//! about 110 readers across the engine test it (Ratchet's physics, his camera, the shadows' probes, the AI, the
//! aims, the shards, the particles).
//!
//! **The data.** The block at `0x1a8ff0`: `+0` on, `+4` the cylinder mode (gravity across a vertical axis through
//! `0x1a9000`, heights measured as 100 − the distance from it), `+0x10` the cylinder's axis point. The sphere's
//! centre is in Ratchet's block at `0x189ed0` (its w `0x189edc` > 0: down toward the centre, else away from it);
//! the heights (`0x326820`, L02 `0x32ad88`) are distances from a second copy, `0x1b1a90` (L02 `0x1b3210`).
//!
//! **The writers.** The level load (`0x3012a0`) clears the flag and sets it from the level settings (their word
//! after the death height, with the centre `0x1b1a90` after it); Maktar's upper area (class 0x0b25, L02 `0x3f6820`,
//! crate::moby::classes::maktar::update_0b25) sets both centres to (500, 200, 500) and turns the flag on when
//! Ratchet rises past z 420 within 150 of it, off when he drops back below 420 (or rises past it farther away). Nothing of Maktar's world lies there (no collision
//! or tfrag of the level or its chunks within 150 of the centre; the arena, the only world above z 420, is 400 away), so
//! in its play the frames never turn on; the dev pin `RC_GRAVITY_AT=x,y,z` (crate `rc-engine` play.rs) tries them.
//!
//! **Ratchet's frame** (`0x2b37e8` → `0x2d37a0`): mode 2 while the sphere is on (not the cylinder). In a frame
//! mode his Euler angles (`0x189eb0`) carry a pitch and roll, the rows `0x189e20` (and their transpose `0x189e60`)
//! are his local frame, and the helpers `0x2d3860` … `0x2d4248` (crate::hero::physics, crate::hero::frame) do
//! their world-z work along his local up instead; his ground probe looks along the last tick's down, and the
//! post-move (`0x2c3330` → `0x2b9488` → `0x2b92c8` / `0x2b9008`) springs his up toward −down.
//!
//! **Frame mode 1** (the magnetic walkways): `0x2d37a0` gives mode 1 on a walkway's surface with the Gravity Boots
//! (`0x18a168`, crate::hero::surface), in the states 0x3b / 0x6d / 0x6e (0x6e not on mud), while `0x18a038`
//! ([`Gravity::walkway_hold`]) counts, or standing (state 0, under 4 air ticks) on the magnetic flag `0x18b2b7` with
//! the boots; mud (`0x18b2ba`) makes it 0. In mode 1 the down (`0x2b4a08`'s end) is the walkways' normal `0x18a110`
//! while its word `0x18a11c` counts down to 0 a tick ([`Gravity::walkway_normal`] / [`Gravity::walkway_k`]), else
//! the ground's normal turned over; the gravity pull and the flat part of a vector go along the ground's normal
//! (`0x2d3860`, `0x2d3d00`), his up springs to the ground's normal (`0x2b92c8`; softly, 0.001, in the air while
//! `0x18a038` counts) about a point 0.8 up him in the air, and the ground probe runs along his moby's up row
//! (`0x2b48b0`): crate::hero::frame.
//!
//! Ported: the flag, the sphere's centres, [`Gravity::height`] (`0x326820`), [`Gravity::up_at`] (`0x326448` /
//! L02 `0x3864f0` without the magnetic walkway), Ratchet's modes 1 and 2 and their down, the hero helpers' frame branches, his
//! camera's up (`0x2d9760`) and its frame branches (`0x421dc8`, `0x4227b8`, `0x4239d0`), the shadow probes'
//! frame branch (`0x31da58`, `0x2a97d8`).
//!
//! The level settings' sphere ([`Gravity::from_settings`]) is set at Ratchet's spawns (the Jamming Array has one).
//!
//! **The cylinder mode** (`0x1a8ff4`, [`Gravity::cylinder`]): set with the flag by Joba's arena master (L11 0x0d8a,
//! crate::moby::classes::level11::arena) in the ring's waves above 480. The up is toward the vertical axis through
//! [`AXIS`] (`0x1a9000`, boot data no code writes), flat ([`cylinder_up`], `0x3263d0`); the heights are 100 less the
//! distance from that point; Ratchet's frame mode is 1 (on mud 0), his down away from the axis, and the walkway's
//! state 0x6e is not forced on him (`0x2c6288`, `0x2cbe4c`).
//!
//! **The frames' up** ([`up`], `0x37ecc0`): the readers that also follow the walkways take Ratchet's moby's up row
//! there (the boots' flag `0x18a168`, the cylinder off), else [`Gravity::up_at`].
//!
//! **The trackers** ([`Trackers`], `0x2319e0`: 0x80 slots of 0x20 bytes): a shot or a missile keeps the up it last saw
//! ([`track`] `0x37f538` takes one, [`untrack`] `0x37f5f8` gives it back) and each tick its velocity is turned with the
//! up's turn since ([`turn`], `0x37f618`), by a weight per gravity (the sphere's, the cylinder's, the walkway's).
//!
//! **The moving frame** (`$gp−0x14e4`, L00 `0x1adb0c`; a level copy each): the platform the effects ride with —
//! [`ride`] (`0x380e08`, a place), [`ride_dir`] (`0x380ec0`, a direction) and [`ride_carry`] (`0x380e58`, a place plus
//! a point's move) through crate::moby::platform. Every level clears it at its start (`0x37f858`); only Level 7's
//! convoy sets it (the car Ratchet rides, crate::moby::classes::level07::train::Globals::riding).
//!
//! Not ported: the camera's walkway branch (`0x4227b8`, G-CAM-003), Ratchet's lean in the frames (`0x2b7820`), the
//! other readers (listed in docs/plan/gaps.md G-LVL-003).

use crate::hero::physics::{add, cross, dist, dot, len, scale, set_len, sub, unit, V3};
use crate::hero::Hero;
use crate::moby::{platform, MobyWorld};

/// `0x1a9000`: the cylinder mode's axis point (551, 674, 499.8).
pub const AXIS: V3 = [551.0, 674.0, f32::from_bits(0x43f9_e666)];

/// The gravity frames' state. Lives in Ratchet's block (crate::hero::Hero::gravity): the readers outside the hero
/// (the class updates, the camera, the shadows) reach it through him.
#[derive(Clone, Copy, Debug)]
pub struct Gravity {
    /// `0x1a8ff0`: the frames on.
    pub on: bool,
    /// `0x1a8ff4`: the cylinder mode (with `on`): the up toward the axis through [`AXIS`].
    pub cylinder: bool,
    /// `0x189ed0`: the sphere's centre; `0x189edc` > 0 (`inward`): down is toward it.
    pub centre: V3,
    pub inward: bool,
    /// `0x1b1a90` (L02 `0x1b3210`): the centre the heights are measured from.
    pub height_centre: V3,
    /// `0x18c0e3`: Ratchet's frame mode this tick (0 world, 1 the walkways, 2 the sphere; crate::hero::frame).
    pub mode: u8,
    /// `0x18a0e0`: Ratchet's gravity direction (unit; (0, 0, −1) in mode 0), set at the end of his ground probe.
    pub down: V3,
    /// `0x18be50`: Ratchet's position at the start of the tick (the post-move holds him there when he stands still in
    /// a frame).
    pub tick_start: V3,
    /// Ratchet's moby's rows (its angles `+0xf0` as the last tick's write-back left them): the helpers `0x2d3b38`,
    /// `0x2d3c40`, `0x2d3df8` turn through them instead of `0x189e20`.
    pub moby_rows: [V3; 3],
    /// `0x189fa4`: the length of this tick's push step (`0x189f70`; 0 without a push).
    pub push_step: f32,
    /// The port's: Ratchet still carries a frame's pitch and roll. Out of the frames `0x2b9488`'s mode-0 branch springs
    /// them back to 0; the port runs it only while this is set (crate::hero::frame), so the world path of a level
    /// without frames is unchanged.
    pub tilted: bool,
    /// `0x18a110` / `0x18a11c`: the magnetic walkways' down and its word beside it (the quad's w), as Boldan's cuboids
    /// 0x0f55 (crate::moby::classes::level13::walkway: against a cuboid's first row, the word 10) and Snivelak's
    /// 0x0c32 (crate::moby::classes::level16::walkway: toward a cuboid's centre, the word 2) set them. Frame mode 1's
    /// ground probe takes the down from it while the word is above 0, counting it down a tick.
    pub walkway_normal: V3,
    pub walkway_k: f32,
    /// `0x18a038` (s16): frame mode 1 forced while it counts (the post-move's timers count it down; Snivelak's 0x0c32
    /// sets 5).
    pub walkway_hold: i16,
}

impl Default for Gravity {
    fn default() -> Self {
        Gravity {
            on: false,
            cylinder: false,
            centre: [0.0; 3],
            inward: false,
            height_centre: [0.0; 3],
            mode: 0,
            down: [0.0, 0.0, -1.0],
            tick_start: [0.0; 3],
            moby_rows: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            push_step: 0.0,
            tilted: false,
            walkway_normal: [0.0; 3],
            walkway_k: 0.0,
            walkway_hold: 0,
        }
    }
}

impl Gravity {
    /// The level's own sphere: the level load (`0x3012a0`) turns the flag on when the settings' word after the death
    /// height (`+0x2c`) is set, with the centre `+0x30` (w 1) into `0x1b1a90`. Ratchet's set-up (`0x2a4fa8`, L26)
    /// then copies that centre and its w into his block (`0x189ed0`), so down is toward it.
    pub fn from_settings(settings: &[u8]) -> Gravity {
        let f = |o: usize| settings.get(o..o + 4).map_or(0.0, |b| f32::from_le_bytes(b.try_into().unwrap()));
        let mut g = Gravity::default();
        if settings.get(0x2c..0x30).is_some_and(|b| b != [0; 4]) {
            let c = [f(0x30), f(0x34), f(0x38)];
            (g.on, g.height_centre, g.centre, g.inward) = (true, c, c, true);
        }
        g
    }

    /// `0x2d37a0`'s sphere part: mode 2 while the sphere is on, the cylinder off (the walkways' and the cylinder's
    /// mode 1: crate::hero::Hero::frame_mode).
    pub fn frame_mode(&self) -> u8 { if self.on && !self.cylinder { 2 } else { 0 } }

    /// `0x326820(p)` (L02 `0x32ad88`): the height in the frames — the distance from the centre `0x1b1a90`; in the
    /// cylinder mode 100 less the distance from [`AXIS`].
    pub fn height(&self, p: V3) -> f32 {
        if self.cylinder { 100.0 - dist(p, AXIS) } else { dist(p, self.height_centre) }
    }

    /// `0x2b4a08`'s end in mode 2: the down at `p`, toward the centre (`0x189edc` > 0) or away from it.
    pub fn down_at(&self, p: V3) -> V3 {
        let d = unit(sub(self.centre, p));
        if self.inward { d } else { scale(d, -1.0) }
    }

    /// `0x326448(p, out, 1)` / L02 `0x3864f0`: the up at `p` — (0, 0, 1) with the frames off, toward the axis in the
    /// cylinder mode ([`cylinder_up`]), else away from the centre (toward it when `0x189edc` ≤ 0). The walkways'
    /// up (Ratchet's up row) is [`up`]'s.
    pub fn up_at(&self, p: V3) -> V3 {
        if !self.on { return [0.0, 0.0, 1.0]; }
        if self.cylinder { return cylinder_up(p); }
        scale(self.down_at(p), -1.0)
    }
}

/// `0x3263d0(p, out, 1)`: the cylinder mode's up at `p` — toward the axis through [`AXIS`], flat (`0x3263d0(p, out,
/// 0)`, the down, is its negation).
pub fn cylinder_up(p: V3) -> V3 {
    let d = sub(AXIS, p);
    unit([d[0], d[1], 0.0])
}

/// What the frames' readers outside the hero take of him: his gravity block and the boots' flag `0x18a168`
/// (crate::hero::surface's `boots`, the walkways), copied (crate::particles::PartAux carries one).
#[derive(Clone, Copy, Debug, Default)]
pub struct Frames {
    pub gravity: Gravity,
    pub walkway: bool,
}

impl Frames {
    pub fn of(hero: &Hero) -> Frames { Frames { gravity: hero.gravity, walkway: hero.surf.boots != 0 } }

    /// `0x37ecc0(p, out)`: the frames' up at `p` with the walkways — Ratchet's moby's up row (`+0xe0`) on the boots'
    /// flag with the cylinder off; the cylinder's up; the sphere's; else (0, 0, 1).
    pub fn up(&self, p: V3) -> V3 {
        let g = &self.gravity;
        if self.walkway && !g.cylinder { return g.moby_rows[2]; }
        if g.cylinder { return cylinder_up(p); }
        if g.on { return scale(g.down_at(p), -1.0); }
        [0.0, 0.0, 1.0]
    }
}

/// `0x37ecc0(p, out)`: [`Frames::up`] of Ratchet's.
pub fn up(hero: &Hero, p: V3) -> V3 { Frames::of(hero).up(p) }

/// The trackers' slot count (`0x2319e0`, 0x80 of 0x20 bytes).
pub const TRACKERS: usize = 0x80;

/// A tracker: `+0x00` the up it last saw, `+0x10` / `+0x14` / `+0x18` its weights in the sphere, the cylinder and on
/// the walkways, `+0x1c` its owner (a moby; none: free).
#[derive(Clone, Copy, Debug, Default)]
pub struct Tracker {
    pub up: V3,
    pub weights: [f32; 3],
    pub owner: Option<usize>,
}

/// `0x2319e0`: the trackers (crate::moby::MobyWorld::trackers).
#[derive(Clone, Debug)]
pub struct Trackers(pub Vec<Tracker>);

impl Default for Trackers {
    fn default() -> Self { Trackers(vec![Tracker::default(); TRACKERS]) }
}

impl Tracker {
    /// `0x37f538`'s slot: `owner`'s, with the three weights and the up at `at` — Ratchet's moby's up row on the boots'
    /// flag `0x18a168`, else [`Gravity::up_at`].
    pub fn new(hero: &Hero, owner: usize, at: V3, weights: [f32; 3]) -> Tracker {
        let up = if hero.surf.boots != 0 { hero.gravity.moby_rows[2] } else { hero.gravity.up_at(at) };
        Tracker { up, weights, owner: Some(owner) }
    }

    /// `0x37f618` on this slot with its owner at `origin` ([`turn`]).
    pub fn turn(&mut self, hero: &Hero, dir: V3, origin: V3) -> V3 {
        let g = &hero.gravity;
        let walkway = hero.surf.boots != 0;
        if !g.on && !g.cylinder && !walkway { return dir; }
        let mut weight = 1.0;
        if walkway { weight = if g.cylinder { self.weights[1] } else { self.weights[2] }; } else if g.cylinder { weight = self.weights[1]; }
        if g.on { weight = self.weights[0]; }
        let old = self.up;
        let length = len(dir);
        let new = if walkway && !g.cylinder { g.moby_rows[2] } else { g.up_at(origin) };
        self.up = new;
        let d = set_len(dir, 1.0);
        // `0x2eef88(out, old, new)`: new × old.
        let axis = cross(new, old);
        let a = -dot(old, new) * (weight * 0.5);
        let (s, c) = a.sin_cos();
        let q = [axis[0] * s, axis[1] * s, axis[2] * s, c];
        let r = crate::quat::mul(crate::quat::mul(q, [d[0], d[1], d[2], 0.0]), [-q[0], -q[1], -q[2], c]);
        set_len([r[0], r[1], r[2]], length)
    }
}

/// `0x37f538(owner, at, sphere, cylinder, walkway)`: the first free tracker for `owner` ([`Tracker::new`]; `at` none:
/// the owner's place). −1 when all are taken.
pub fn track(w: &mut MobyWorld, hero: &Hero, owner: usize, at: Option<V3>, weights: [f32; 3]) -> i32 {
    let at = at.unwrap_or_else(|| w.mobys.get(owner).map_or([0.0; 3], |m| m.pos));
    let Some(i) = w.trackers.0.iter().position(|t| t.owner.is_none()) else { return -1 };
    w.trackers.0[i] = Tracker::new(hero, owner, at, weights);
    i as i32
}

/// `0x37f5f8(i)`: tracker `i` given back (none below 0).
pub fn untrack(w: &mut MobyWorld, i: i32) {
    if let Some(t) = usize::try_from(i).ok().and_then(|i| w.trackers.0.get_mut(i)) { t.owner = None; }
}

/// `0x37f618(i, dir, at)`: `dir` turned with the up's turn at `at` (none: the owner's place) since tracker `i` last
/// looked, by its weight — the sphere's with the frames on, else the cylinder's, else the walkway's (the boots' flag
/// `0x18a168`); nothing with none of them, `i` below 0 or no owner. The new up is Ratchet's moby's up row on the
/// walkways (the cylinder off), else [`Gravity::up_at`]. The turn is the game's: the quaternion of `new × old` by
/// `sin(a)` and `cos(a)` with `a = −(old · new) · weight / 2`, as `q · dir · q̄`, back to `dir`'s length.
pub fn turn(w: &mut MobyWorld, hero: &Hero, i: i32, dir: V3, at: Option<V3>) -> V3 {
    let g = &hero.gravity;
    if i < 0 || (!g.on && !g.cylinder && hero.surf.boots == 0) { return dir; }
    let Some(t) = w.trackers.0.get(i as usize).copied() else { return dir };
    let origin = match at {
        Some(p) => p,
        None => match t.owner.and_then(|o| w.mobys.get(o)) {
            Some(m) => m.pos,
            None => return dir,
        },
    };
    w.trackers.0[i as usize].turn(hero, dir, origin)
}

/// `$gp−0x14e4` (L00 `0x1adb0c`): the level's moving frame (module docs).
pub fn moving_frame(w: &MobyWorld) -> Option<usize> { w.convoy.riding }

/// `0x380e08(out, p)`: `p` riding the level's moving frame this tick (`0x322058`'s place, crate::moby::platform::ride;
/// its angles unread), else `p`.
pub fn ride(w: &MobyWorld, p: V3) -> V3 {
    moving_frame(w).and_then(|f| platform::ride(w, f, p, [0.0; 3])).map_or(p, |(q, _)| q)
}

/// `0x380ec0(v, v)`: `v` turned by the moving frame's turn this tick (`0x3221c8`, crate::moby::platform::turn_dir),
/// else `v`.
pub fn ride_dir(w: &MobyWorld, v: V3) -> V3 { moving_frame(w).and_then(|f| platform::turn_dir(w, f, v)).unwrap_or(v) }

/// `0x380e58(out, base, p)`: `base` plus how far `p` moved riding the moving frame this tick (`0x3222a8`,
/// crate::moby::platform::carry), else `base`.
pub fn ride_carry(w: &MobyWorld, base: V3, p: V3) -> V3 {
    moving_frame(w).and_then(|f| platform::carry(w, f, p)).map_or(base, |m| add(base, m))
}

/// A vector into the frame of `rows` (row i = image of axis i): `0x2ef340(out, v, transpose)`.
pub fn to_local(v: V3, rows: &[V3; 3]) -> V3 { [dot(v, rows[0]), dot(v, rows[1]), dot(v, rows[2])] }

/// A local vector back into the world: `0x2ef340(out, l, rows)`.
pub fn to_world(l: V3, rows: &[V3; 3]) -> V3 { std::array::from_fn(|k| l[0] * rows[0][k] + l[1] * rows[1][k] + l[2] * rows[2][k]) }

/// `v` turned by `angle` about the unit `axis` (right-handed).
pub fn rotate(v: V3, axis: V3, angle: f32) -> V3 {
    let (s, c) = angle.sin_cos();
    let k = axis;
    let kv = [k[1] * v[2] - k[2] * v[1], k[2] * v[0] - k[0] * v[2], k[0] * v[1] - k[1] * v[0]];
    let d = dot(k, v) * (1.0 - c);
    std::array::from_fn(|i| v[i] * c + kv[i] * s + k[i] * d)
}

/// Whether `axis` can be made unit.
pub fn has_len(v: V3) -> bool { 0.0 < len(v) }

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: V3, b: V3) -> bool { (0..3).all(|i| (a[i] - b[i]).abs() < 1e-5) }

    #[test]
    fn up_is_world_z_off_and_radial_on() {
        let mut g = Gravity::default();
        assert_eq!(g.up_at([3.0, 4.0, 5.0]), [0.0, 0.0, 1.0]);
        assert_eq!(g.frame_mode(), 0);
        g.on = true;
        g.inward = true;
        g.centre = [500.0, 200.0, 500.0];
        g.height_centre = g.centre;
        assert!(close(g.up_at([500.0, 200.0, 510.0]), [0.0, 0.0, 1.0]));
        assert!(close(g.down_at([510.0, 200.0, 500.0]), [-1.0, 0.0, 0.0]));
        assert!((g.height([500.0, 210.0, 500.0]) - 10.0).abs() < 1e-5);
        assert_eq!(g.frame_mode(), 2);
    }

    #[test]
    fn frames_round_trip_and_rotate() {
        let rows = crate::hero::physics::euler_rows([0.3, -0.2, 1.1]);
        let v = [0.4, -1.5, 2.0];
        assert!(close(to_world(to_local(v, &rows), &rows), v));
        assert!(close(rotate([0.0, 0.0, 1.0], [0.0, 1.0, 0.0], std::f32::consts::FRAC_PI_2), [1.0, 0.0, 0.0]));
    }
}
