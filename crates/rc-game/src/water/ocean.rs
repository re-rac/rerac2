//! The ocean about the camera (R&C1's oceans: 1111's `0x30a908`, Pokitaru's): one water moby per level that keeps
//! the water's level and draws, each frame, a far band and two scrolled surface layers following the camera. Three
//! copies of the code, one [`Kind`] each:
//!
//! | Kind | Class | Update / draw | Levels |
//! |---|---|---|---|
//! | [`Kind::Swamp`] | 0x0a17 | L01 `0x3bcdd8` / `0x3bc3c0` | Oozla |
//! | [`Kind::Sea`] | 0x12b5 | L09 `0x3819a0` / `0x380fe8` (L20 `0x39c9d0`) | Dobbo, Yeedil |
//! | [`Kind::Sheet`] | 0x093e | L06 `0x404680` / `0x403ac0` (L08 `0x3d25e0`, L11 `0x3d19f8`, L13 `0x3c7fd0`, L16 `0x3b8598`, L18 `0x3db930`) | Notak, Tabora, Joba, Boldan, Snivelak, Damosel |
//!
//! **The update.**
//! * The sea only while the level's variant (`0x1c92c0`'s per-level copy: L09 `0x1c9f00`) is its `+0x30`, the sheet
//!   its `+0x38`; the sheet not on level 13 with Ratchet below 88 out of state 0x74, nor on level 11 with
//!   `0x1a9010` set; the sheet takes its level (`+0x14`) every tick.
//! * State 0: state 1, its update distance 0xff, the layers' scroll 0 (`game.water_scroll`), the camera noted
//!   (`game.water_cam`), the level `+0x14` (`game.water_base`); the swamp also sets its colours (`+0x00` the band
//!   near `0x80103030`, `+0x04` the band far and the wall's foot `0x800c1006`, `+0x0c` / `+0x10` the surface near /
//!   at two tiles `0x50106060` / `0x00106060`) and its reach `+0x28` 224. The others keep their placed ones.
//! * Ratchet in a dry cuboid ends the tick there: the swamp's `+0x30` / `+0x34`, the sea's four `+0x40..+0x4c`, the
//!   sheet's outer `+0x30` and inner `+0x34`. Else the level is `+0x14`, or with him in one of the swamp's eight
//!   cuboids `+0x40..+0x5c` (the sea's four `+0x50..+0x5c`) that cuboid's centre height + 0.15.
//! * The camera's move since the last tick turns each layer's scroll by `move · 2·scale / (5·tile)` (scales 2, 3;
//!   the tile `+0x18`, the sheet's held to 32 in Ratchet's state 0x1c; the whole turns dropped); while the camera is
//!   above `+0x20` the height (`game.water_z`) waves about the level by `sin(frame mod 360 · 0.017444445 − 3.14)` ×
//!   0.1 (the sea's `+0x34`, the sheet's `+0x3c`), the scroll drifts by the layer's speed ((0.04, 0.04), (0, −0.04)) /
//!   60, both kept within ±1, and the draw ([`draw`]) is registered: on the second list for the swamp, for the sea with
//!   `+0x38` set and for the sheet with `+0x00` negative; else on the first.
//!
//! The scales and speeds are the same words in every level's overlay (L01 `$gp+0xdc0` / `$gp+0xdd0`, L09 `0x1aebf0` /
//! `0x1aec00`, L06 `0x1b0010` / `0x1b0020`, …).
//!
//! Not ported: the hero's no-dive word `0x18a01e` (0 in a dry cuboid, else 2), the swamp's camera test of its dry
//! cuboids in state 0x1c, `0x1a8a48` (read as 0), the sheet's camera rows on level 13 and on level 16 with
//! Ratchet's `+0x348` = 1 (`0x1b3290`: rebuilt about the up axis), the sea's unflattened rows on level 13 (dead: the
//! sea is on 9 and 20 only).

use crate::fx::FxQuad;
use crate::hero::physics::{add, len, scale, set_len, sub, V3};
use crate::hud::lerp_rgba;
use crate::moby::{Ctx, MobyWorld};

pub const SWAMP: i16 = 0x0a17;
pub const SEA: i16 = 0x12b5;
pub const SHEET: i16 = 0x093e;
pub const CLASSES: [i16; 3] = [SWAMP, SEA, SHEET];

/// The layers' texture scales and drift speeds (s, t a second).
const SCALE: [f32; 2] = [2.0, 3.0];
const SPEED: [[f32; 2]; 2] = [[0.04, 0.04], [0.0, -0.04]];

/// The copy of the code a class runs (module docs).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Swamp,
    Sea,
    Sheet,
}

impl Kind {
    pub fn of(o_class: i16) -> Option<Kind> {
        match o_class {
            SWAMP => Some(Kind::Swamp),
            SEA => Some(Kind::Sea),
            SHEET => Some(Kind::Sheet),
            _ => None,
        }
    }

    /// The dry cuboids' and the height cuboids' pvars.
    fn cuboids(self) -> (std::ops::Range<usize>, std::ops::Range<usize>) {
        match self {
            Kind::Swamp => (0..2, 0..8),
            Kind::Sea => (0..4, 0..4),
            Kind::Sheet => (0..2, 0..0),
        }
    }

    fn dry_at(self) -> usize { match self { Kind::Swamp | Kind::Sheet => 0x30, Kind::Sea => 0x40 } }
    fn height_at(self) -> usize { match self { Kind::Swamp => 0x40, Kind::Sea | Kind::Sheet => 0x50 } }
}

/// The tile (`+0x18`): the sheet's held to 32 with Ratchet in state 0x1c (`0x1a8a48` not ported).
fn tile(kind: Kind, m: &crate::moby::Moby, hero_state: i32) -> f32 {
    let t = m.pv_f(0x18);
    if kind == Kind::Sheet && hero_state == 0x1c { t.min(32.0) } else { t }
}

/// The ocean's update (module docs).
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(kind) = Kind::of(w.mobys[id].o_class) else { return };
    let cam = ctx.host.camera_pos();
    let hero = ctx.hero.pos;
    let m = &mut w.mobys[id];
    match kind {
        Kind::Sea => if ctx.game.variant != m.pv_i(0x30) { return },
        Kind::Sheet => {
            if ctx.game.variant != m.pv_i(0x38) { return; }
            if ctx.game.level == 13 && hero[2] < 88.0 && ctx.hero.state != 0x74 { return; }
            if ctx.game.level == 11 && ctx.game.arena_on { return; }
            ctx.game.water_base = m.pv_f(0x14);
        }
        Kind::Swamp => {}
    }
    if m.state == 0 {
        m.update_dist = 0xff;
        m.state = 1;
        if kind == Kind::Swamp {
            for (o, c) in [(0x00, 0x8010_3030u32), (0x04, 0x800c_1006), (0x0c, 0x5010_6060), (0x10, 0x0010_6060)] { m.set_pv_i(o, c as i32); }
            m.set_pv_f(0x28, 224.0);
        }
        ctx.game.water_scroll = [[0.0; 2]; 2];
        ctx.game.water_cam = cam;
        ctx.game.water_base = m.pv_f(0x14);
        return;
    }
    let (dry, heights) = kind.cuboids();
    if dry.map(|k| m.pv_i(kind.dry_at() + 4 * k)).any(|c| ctx.level.in_cuboid(c, hero)) { return; }
    ctx.game.water_base = heights
        .map(|k| m.pv_i(kind.height_at() + 4 * k))
        .find(|&c| ctx.level.in_cuboid(c, hero))
        .and_then(|c| ctx.level.cuboid_place(c))
        .map_or(m.pv_f(0x14), |(centre, _)| centre[2] + 0.15);
    let d = sub(cam, ctx.game.water_cam);
    ctx.game.water_cam = cam;
    if cam[2] <= m.pv_f(0x20) { return; }
    let wrap = |x: f32| if 1.0 < x { x - 1.0 } else if x < -1.0 { x + 1.0 } else { x };
    let t = tile(kind, m, ctx.hero.state);
    for (k, sc) in ctx.game.water_scroll.iter_mut().enumerate() {
        let f = SCALE[k] * 2.0 / (t * 5.0);
        let step = |v: f32| if v != 1.0 { v - v.trunc() } else { v };
        sc[0] = wrap(sc[0] + step(d[0] * f));
        sc[1] = wrap(sc[1] + step(d[1] * f));
    }
    let amplitude = match kind { Kind::Swamp => 0.1, Kind::Sea => m.pv_f(0x34), Kind::Sheet => m.pv_f(0x3c) };
    // The game's own 3.14 (not π).
    #[allow(clippy::approx_constant)]
    let phase = (ctx.game.frame % 360) as f32 * 0.017_444_445 - 3.14;
    ctx.game.water_z = ctx.game.water_base + phase.sin() * amplitude;
    for (k, sc) in ctx.game.water_scroll.iter_mut().enumerate() {
        sc[0] = wrap(sc[0] + SPEED[k][0] / 60.0);
        sc[1] = wrap(sc[1] + SPEED[k][1] / 60.0);
    }
    let second = match kind { Kind::Swamp => true, Kind::Sea => m.pv_i(0x38) != 0, Kind::Sheet => m.pv_i(0x00) < 0 };
    if second { w.register_draw(id) } else { w.register_draw_first(id) }
}

/// The ocean's draw (`0x3bc3c0` / `0x380fe8` / `0x403ac0`) about the camera at the water's height (`game.water_z`).
/// F and L are the camera's forward and left rows flattened and normalised.
/// * The far band (FX 0x16, ST 0, ALPHA 0x44: the colours' own alpha): a strip from `cam − 75F ∓ 87L` to
///   `cam + reach·F ∓ 750L` (`+0x28`; the sea's ∓ 748L; the sheet's on level 16 from `cam − 150F ∓ 175L` to ∓ 1500L)
///   in five steps, its colour `+0x00` → `+0x04` along it.
/// * The swamp's wall (the same FX, ALPHA 0x44), with the camera more than `+0x24` above the level (`+0x14`; not in
///   state 0x1c): from the band's far edge up to `camera z − +0x24`, `+0x04` at its foot and `+0x08` at its top. The
///   sea and the sheet have none.
/// * The surface, two layers (FX `+0x1c + layer + 0x3f`, ALPHA 0x48: added): 4 × 4 tiles about the camera
///   (`cam + (x, y) + 1`), ST `x / (2·tile) · scale + scroll`, each corner's colour `+0x0c` → `+0x10` by its distance
///   to the camera over two tiles. The sea lifts layer k by 0.05·(k + 1); the sheet both by
///   `max(10, camera z − height) · tile · 0.00002`, at least 0.125.
///
/// The water's own fog (`0x1bdbe8` / `0x1bdbec` set to 255 / 249.9 for its draw: next to none) is the effects' (none).
/// Not ported: its draws' Z test without Z write (the effects test Z the same way; the sea with `+0x38` and the sheet
/// with `+0x00` negative draw the band with the alpha test `0x5310b`, Z written where its alpha is 0x10 or more), the
/// hero state 0x1c's normal fog.
pub fn draw(w: &MobyWorld, id: usize, game: &crate::game::GameState, hero_state: i32, cam: V3, rows: [V3; 3], out: &mut Vec<FxQuad>) {
    let m = &w.mobys[id];
    let Some(kind) = Kind::of(m.o_class) else { return };
    let c = |o: usize| m.pv_i(o) as u32;
    let z = game.water_z;
    let flat = |v: V3| set_len([v[0], v[1], 0.0], 1.0);
    let (f, l) = (flat(rows[0]), flat(rows[1]));
    let at = |a: f32, b: f32| -> V3 { let p = add(add(scale(f, a), scale(l, b)), cam); [p[0], p[1], z] };
    let ((back, near), far) = match kind {
        Kind::Sheet if game.level == 16 => ((-150.0, 175.0), 1500.0),
        Kind::Sea => ((-75.0, 87.0), 748.0),
        _ => ((-75.0, 87.0), 750.0),
    };
    let reach = m.pv_f(0x28);
    let (n0, n1, f0, f1) = (at(back, -near), at(back, near), at(reach, -far), at(reach, far));
    let lerp = |a: V3, b: V3, t: f32| add(a, scale(sub(b, a), t));
    let st0 = [[0.0; 2]; 4];
    let band = |i: usize| (i as f32 * 0.2, lerp_rgba(i as f32 * 0.2, c(0x00), c(0x04)));
    for i in 0..5 {
        let ((t0, c0), (t1, c1)) = (band(i), band(i + 1));
        out.push(FxQuad { fx: 0x16, additive: false, corners: [lerp(n0, f0, t0), lerp(n1, f1, t0), lerp(n0, f0, t1), lerp(n1, f1, t1)], st: st0, rgba: [c0, c0, c1, c1] });
    }
    if kind == Kind::Swamp {
        let top = cam[2] - m.pv_f(0x24);
        if m.pv_f(0x14) < top {
            let up = |p: V3| [p[0], p[1], top];
            out.push(FxQuad { fx: 0x16, additive: false, corners: [f0, up(f0), f1, up(f1)], st: st0, rgba: [c(0x04), c(0x08), c(0x04), c(0x08)] });
        }
    }
    let tile = tile(kind, m, hero_state);
    if tile <= 0.0 { return; }
    let colour = |p: V3| lerp_rgba((len(sub(p, cam)) / (tile * 2.0)).min(1.0), c(0x0c), c(0x10));
    for (layer, sc) in game.water_scroll.iter().enumerate() {
        let lift = match kind {
            Kind::Swamp => 0.0,
            Kind::Sea => (layer + 1) as f32 * 0.05,
            Kind::Sheet => ((cam[2] - z).max(10.0) * tile * 0.000_02).max(0.125),
        };
        let fx = (m.pv_i(0x1c) + layer as i32 + 0x3f).max(0) as usize;
        let st = |x: f32, y: f32| [x / (tile * 2.0) * SCALE[layer] + sc[0], y / (tile * 2.0) * SCALE[layer] + sc[1]];
        for row in 0..4 {
            let y = (row as f32 - 2.0) * tile;
            for col in 0..4 {
                let x = (col as f32 - 2.0) * tile;
                let p = |dx: f32, dy: f32| [cam[0] + x + dx + 1.0, cam[1] + y + dy + 1.0, z + lift];
                let corners = [p(0.0, 0.0), p(0.0, tile), p(tile, 0.0), p(tile, tile)];
                out.push(FxQuad {
                    fx,
                    additive: true,
                    corners,
                    st: [st(x, y), st(x, y + tile), st(x + tile, y), st(x + tile, y + tile)],
                    rgba: corners.map(colour),
                });
            }
        }
    }
}
