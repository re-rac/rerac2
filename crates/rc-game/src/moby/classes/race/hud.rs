//! The hoverbike races' screens (crate::moby::classes::race; L04 addresses, the L11 copies the same code on their
//! own data, the same values): the race's HUD ([`draw`], `0x4188b8`, L11 `0x408408`), set by the start (sub-state 2)
//! until the finish; the standings after it ([`standings`], `0x4127a8`, L11 `0x402450`), set each tick of the bike's
//! state 3. Both are the HUD's screen callback (crate::hud::ScreenCallback): they take the counters' place.
//!
//! **The race's HUD** (`0x4188b8`), in its order:
//! * ridden after a retry (sub-state 1): the box asking for a button ([`retry_box`], `0x418348`);
//! * the lap's time while `$gp+0x18dc` runs ([`lap_box`], `0x4185d8`; the bike's tick counts it down, the sound 0xd at
//!   173 — crate::moby::classes::race::bike); else, the bike turned more than 90° off the race's way (`+0x4a4`): the
//!   wrong-way box (112..142 down, 175 in from either side) and the text 0x2e2b in font 3 at (256, 118) in
//!   `0x700000e4`, `+0x76c` set (and cleared once it faces the way again);
//! * the countdown's lights while counting down (sub-states 1..4) and 60 ticks into the race ([`lights`], `0x40dd90`);
//! * the gauges' frame 0x2d / 0x2e at (10, 10); with a weapon (`+0x360` 1 shots, 2 missiles, 3 mines) its icon
//!   [`WEAPON_ICON`] (frame 0 / 1 / 2) 40 × 35 at (62, 17) in its colour ([`WEAPON_COLOURS`]) and its bar 111..202
//!   × the share left (`+0x364` of [`WEAPON_COUNTS`]) by 28..41; the turbo charges (`+0x33c`) as the frame of the
//!   sprite 0x35 at (11, 10), flashing from `0x800000ff` to `0x603f00b2` over 45 ticks after one is gained; the boost
//!   gauge 26..38 across, up from 98 by 74 × the jet boost's ticks left (`+0x278` / `+0x650`; else the turbo's
//!   `+0x338` / 240), in `0x603f00b2`;
//! * the track map's frame 0x3f / 0x40 / 0x41 at (498, 10) and its picture (Barlow 0x42 and, the short cut shut —
//!   the record's flag 4 —, 0x43 in `0x60ffde8d`; Joba 0x30);
//! * the place (its text by [`PLACE_TEXTS`], font 2 at (291, 24), from the 10th at 285, in [`PLACE_COLOURS`]) and the
//!   lap (`"%d/%d"`, the lap + 1 of 3, font 2 centred at (469, 71) in `0x60f0f0f0`);
//! * **the strip**: three lines (`0x25f0c070`), nine speed marks scrolling by the bike's speed across the race's way
//!   ([`Words::scroll`], `0x1b3210`, eased to −1.25 × it by 0.05; the marks `0x260614`, 8 apart, wrapping at 355 /
//!   419), each racer racing (or all while the bike waits, sub-states 1 / 3) seen in the race's frame (2 × along +
//!   387, 4 × across + 36) as the arrow 0x3e turned by its yaw off the race's way, its place and yaw eased in its
//!   `+0x1cc..+0x1e0` (`0x31ed78`: 5/36, 5/36, 35/6; `0x31f230`: 2π/2700, 2π/2700, 3π/2; out of the strip they reset),
//!   faded at the strip's ends; the bike's arrow at 387, its height 36 + 13 × its place across (a fifth of it, ±1);
//! * **the map**: each racer's dot (4 × 4 about its place on the map, `0x600040e4`) and the bike's (`0x8000a6e4`) at
//!   their race progress ([`map_place`], `0x43c4e8` / L11 `0x4592b0`).
//!
//! **The standings** (`0x4127a8`): a won race's banner (0x39 / 0x38 at (256, 194), 0x2e29 at (256, 32), the prize
//! `0x2e5c` or, while `$gp+0x18fc` runs, 0x1233 at (256, 65)); the panel 0x47 / 0x46 at (256, 196) growing up from 315
//! to 157 ([`Words::grow`]); its title (`"%s %s %s"`: 0x2e15, the place, 0x2e16) at (256, 106) and 0x2dd8 right to
//! 390 at its top; the list ([`Words::list`] scrolling it up from 138, 15 apart, the bike's row kept in sight) of the
//! standings chain (`+0x39c`, then each one's next: the bike's `+0x3a0`, a racer's `+0x1a4`; [`standings_chain`],
//! `0x412490`): each finished place (`"%s %s"`: its place's text, 0x2e17, font 2 centred at 175) and its time
//! (`"%s %d : %d :%d.%d"`, font 2 at 220) — the racers behind the bike that have not finished yet the dots of 0x2e46
//! (the first one's cycling every 15 ticks), the one that just did the sound 0x16; the bike's row in `0x8000c0c0`,
//! the others `0x80f0f0f0` faded in at the list's top and at the panel's bottom. Past the 10th, the bike's row below
//! the list. Ten rows at most.
//!
//! The view's tangent the screens set for their sprites (`0x1bdc70` = tan 0.5622) is the play's own here
//! (crate::gui::MENU_TAN); the race's view angle (`+0x28c`) they set back after is the bike's tick's
//! (crate::moby::classes::race::bike::boosts).
//!
//! Not ported: the debug pad's nudges and print at the HUD's top (`$gp+0x1b4c..0x1b58`, `0x2cc2f0`), the wrong-way
//! voice (the dialogue player `0x1a63cc` / `0x1a641a`, `0x351998`: the voice `+0x760` + 3000), the GS test register
//! the draws set (`0x3a0390(0x47, 0x5360b)`), the wide screen's 0.9 and its light places (`0x1a7b98`), the PAL sizes.

use crate::hero::physics::{approach, atan2, dot, ease, sub, sub_rot, turn_ease, V3};
use crate::hud::{lerp_rgba, text_of, Align, Draw, ScreenDraw, ScreenIn};
use crate::moby::MobyWorld;
use std::f32::consts::FRAC_PI_2;

/// The HUD icon of the race's weapons (frames 0..2: shots, missiles, mines).
pub const WEAPON_ICON: u16 = 0x7568;
/// L04 `0x1b0b00` (L11 `0x1b0a90`): the weapons' colours by `+0x360` − 1.
pub const WEAPON_COLOURS: [u32; 4] = [0x6000_a6e4, 0x4000_40e4, 0x60ff_de8d, 0x60e7_02a8];
/// L04 `0x1b0b28` (L11 `0x1b0ab8`): the HUD's full count of each weapon by `+0x360` (the shots' bursts, the
/// missiles, the mines; the weapons' own table, `0x1b09c0`, has the shield's ticks 600 where this has 1200).
pub const WEAPON_COUNTS: [i32; 5] = [0, 100, 3, 1, 1200];
/// L04 `$gp+0x1b20` (`0x1b0b10`): the place's colour, the first three's and the rest's.
pub const PLACE_COLOURS: [u32; 4] = [0x7040_ff00, 0x7040_d0d0, 0x7050_50e0, 0x60f0_f0f0];
/// L04 `0x2ca320` (L11 `0x2bac30`): the places' texts, 1st..20th.
pub const PLACE_TEXTS: [u32; 20] = [
    0x2db8, 0x2db9, 0x2dba, 0x2dbb, 0x2dbc, 0x2e1a, 0x2e1b, 0x2e1c, 0x2e1d, 0x2e1e, 0x2e1f, 0x2e20, 0x2e21, 0x2e22, 0x2e23, 0x2e24, 0x2e25, 0x2e26, 0x2e27, 0x2e28,
];
/// L04 `0x1b0b78` (L11 `0x1b0b08`): the speed marks' places the start sets (`0x41ace8`).
pub const MARKS: [i32; 9] = [357, 365, 373, 381, 389, 397, 405, 413, 0];

/// The screens' own words.
#[derive(Clone, Copy, Debug, Default)]
pub struct Words {
    /// L04 `$gp+0x1b30` / `$gp+0x1b34` (L11 `+0x1ac0` / `+0x1ac4`): the turbo charges shown and their flash's ticks.
    pub turbo_shown: i32,
    pub turbo_flash: i32,
    /// `0x1b3210`: the speed marks' scroll; `0x260614` (8 apart): their places.
    pub scroll: f32,
    pub marks: [f32; 9],
    /// The standings: `$gp+0x19f0` the list's scroll, `$gp+0x19f4` the panel's growth, `$gp+0x19e8` / `$gp+0x19ec`
    /// each at its end, `$gp+0x19e4` the dots' phase, `$gp+0x19f8` every racer ahead of the bike in.
    pub list: f32,
    pub grow: f32,
    pub list_done: bool,
    pub grow_done: bool,
    pub dots: i32,
    pub all_in: bool,
}

impl Words {
    /// The start's (`0x41ace8`): the scroll none, the marks at [`MARKS`].
    pub fn start(&mut self) {
        self.scroll = 0.0;
        self.marks = MARKS.map(|x| x as f32);
    }
}

fn sprite(out: &mut Vec<ScreenDraw>, id: u32, pos: [f32; 2], scale: [f32; 2], frame: f32, colour: u32) {
    out.push(ScreenDraw::Hud(Draw::Sprite { id, pos, scale, frame, colour }));
}

/// `0x31ce48(x0, y0, x1, y1, colour, 0)`: a flat rectangle.
fn rect(out: &mut Vec<ScreenDraw>, x0: i32, y0: i32, x1: i32, y1: i32, colour: u32) {
    out.push(ScreenDraw::Hud(Draw::Bar { pos: [x0 as f32, y0 as f32], size: [(x1 - x0) as f32, (y1 - y0) as f32], colours: [colour, colour] }));
}

fn text(out: &mut Vec<ScreenDraw>, font: u8, text: Vec<u8>, x: i32, y: i32, colour: u32, align: Align) {
    out.push(ScreenDraw::Text { font, text, pos: [x as f32, y as f32], scale: 1.0, colour, align });
}

/// The texts' `%s` / `%d` filled in order (`sprintf` on the formats these screens use).
pub(crate) fn format(fmt: &[u8], args: &[Arg]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut a = args.iter();
    let mut i = 0;
    while i < fmt.len() {
        if fmt[i] == b'%' && i + 1 < fmt.len() && matches!(fmt[i + 1], b's' | b'd') {
            match a.next() {
                Some(Arg::S(s)) => out.extend_from_slice(s),
                Some(Arg::D(n)) => out.extend_from_slice(n.to_string().as_bytes()),
                None => {}
            }
            i += 2;
        } else {
            out.push(fmt[i]);
            i += 1;
        }
    }
    out
}

pub(crate) enum Arg<'a> {
    S(&'a [u8]),
    D(i32),
}

/// A race time in ticks as the screens split it: hours, minutes, seconds, tenths.
fn split(t: i32) -> [i32; 4] { [t / 216_000, (t / 3600) % 60, (t / 60) % 60, (t / 6) % 10] }

/// The bike's eased angles' matrix (`0x2ef6d8` of `(0, a, 0)`, its x and z rows: crate::gui::sprite_mat).
fn turned(a: f32) -> [[f32; 2]; 2] {
    let (s, c) = a.sin_cos();
    [[c, -s], [s, c]]
}

/// `v` in the race's frame (`0x2ef340` through its rows' transpose).
fn local(rows: &[V3; 3], v: V3) -> V3 { [dot(v, rows[0]), dot(v, rows[1]), dot(v, rows[2])] }

/// `0x4188b8` (module docs).
pub fn draw(w: &mut MobyWorld, id: usize, inp: &ScreenIn, out: &mut Vec<ScreenDraw>) -> bool {
    if w.mobys.get(id).is_none_or(|m| m.pvars.len() < 0x770) { return true; }
    let level = inp.game.level;
    let sub_state = w.mobys[id].timer;
    if sub_state == 1 { retry_box(inp, out); }
    let off_way = FRAC_PI_2 < w.mobys[id].pv_f(0x4a4).abs();
    if w.race.lap_shown < 1 {
        if off_way {
            out.push(ScreenDraw::Panel { rect: [112, 142, 0xaf, crate::gui::SCREEN_W as i32 - 0xaf], alpha: 0x60, fill: 0x6044_2d00, edge: 0x55f0_c070 });
            text(out, 3, text_of(inp, 0x2e2b, usize::MAX), crate::gui::SCREEN_W as i32 / 2, 118, 0x7000_00e4, Align::Centre);
            w.mobys[id].set_pv_i(0x76c, 1);
        }
    } else {
        lap_box(w, inp, out);
        crate::moby::countdown(&mut w.race.lap_shown);
    }
    if !off_way && w.mobys[id].pv_i(0x76c) != 0 { w.mobys[id].set_pv_i(0x76c, 0); }
    if (1..=4).contains(&sub_state) || w.race.clock < 0x3c { lights(w, id, out); }
    for (s, c) in [(0x2d, 0x6044_2d00u32), (0x2e, 0x55f0_c070)] { sprite(out, s, [10.0, 10.0], [1.0, 1.0], 0.0, c); }
    let b = &w.mobys[id];
    let weapon = b.pv_i(0x360);
    if weapon != 0 {
        let colour = WEAPON_COLOURS[(weapon - 1).clamp(0, 3) as usize];
        let frame = match weapon { 2 => 1, 3 => 2, _ => 0 };
        out.push(ScreenDraw::Hud(Draw::Icon { id: WEAPON_ICON, frame, pos: [62.0, 17.0], size: [40.0, 35.0], colour }));
        let f = (b.pv_i(0x364) as f32 / WEAPON_COUNTS[weapon.clamp(0, 4) as usize] as f32).clamp(0.0, 1.0);
        rect(out, 0x6f, 28, (f * 91.0) as i32 + 0x6f, 41, colour);
    }
    let charges = b.pv_i(0x33c);
    let words = &mut w.race.hud;
    if charges < 1 {
        words.turbo_shown = 0;
    } else {
        if words.turbo_shown != charges {
            words.turbo_flash = if words.turbo_shown < charges { 0x2d } else { 0 };
            words.turbo_shown = charges;
        }
        if 0 < words.turbo_flash { words.turbo_flash -= 1; }
        let c = lerp_rgba(words.turbo_flash as f32 * 0.022_222_223, 0x603f_00b2, 0x8000_00ff);
        sprite(out, 0x35, [11.0, 10.0], [1.0, 1.0], words.turbo_shown as f32, c);
    }
    let b = &w.mobys[id];
    let jet = b.pv_i(0x278);
    let f = if jet == 0 { b.pv_i(0x338) as f32 * 0.004_166_667 } else { jet as f32 / b.pv_i(0x650) as f32 };
    let f = f.clamp(0.0, 1.0);
    rect(out, 0x1a, 0x62 - (f * 74.0) as i32, 0x26, 98, 0x603f_00b2);
    sprite(out, 0x3f, [498.0, 10.0], [1.0, 1.0], 0.0, 0x6044_2d00);
    sprite(out, 0x40, [498.0, 10.0], [1.0, 1.0], 0.0, 0x55f0_c070);
    sprite(out, 0x41, [498.0, 10.0], [1.0, 1.0], 0.0, 0x55f0_c070);
    if level == 4 {
        sprite(out, 0x42, [498.0, 10.0], [1.0, 1.0], 0.0, 0x55f0_c070);
        if super::record(level, w.race.race).flags & 4 != 0 { sprite(out, 0x43, [498.0, 10.0], [1.0, 1.0], 0.0, 0x60ff_de8d); }
    } else {
        sprite(out, 0x30, [498.0, 10.0], [1.0, 1.0], 0.0, 0x55f0_c070);
    }
    let mut place = w.mobys[id].pv_i(0x320);
    if !(1..=20).contains(&place) { place = 20; }
    let x = if 9 < place { 0x11d } else { 0x123 };
    text(out, 2, text_of(inp, PLACE_TEXTS[(place - 1) as usize], usize::MAX), x, 24, PLACE_COLOURS[(place - 1).min(3) as usize], Align::Left);
    text(out, 2, format(b"%d/%d", &[Arg::D(w.race.lap + 1), Arg::D(3)]), 0x1d5, 71, 0x60f0_f0f0, Align::Centre);
    rect(out, 0x157, 36, 0x1ab, 37, 0x25f0_c070);
    rect(out, 0x151, 28, 0x1ab, 29, 0x25f0_c070);
    rect(out, 0x160, 45, 0x1ab, 46, 0x25f0_c070);
    strip(w, id, level, out);
    true
}

/// The strip and the map (module docs).
fn strip(w: &mut MobyWorld, id: usize, level: i32, out: &mut Vec<ScreenDraw>) {
    let frame = w.race.frame;
    let rows = frame.rows;
    let b = &w.mobys[id];
    let v = crate::hero::physics::scale(b.rows[0], b.pv_f(0x30c) + b.pv_f(0x27c));
    let mut l = local(&rows, v);
    let s = dot(l, rows[0]);
    l = crate::hero::physics::add(l, crate::hero::physics::scale(rows[0], s));
    let words = &mut w.race.hud;
    approach(l[0] * -1.25, 0.050_000_004, &mut words.scroll);
    let scroll = words.scroll;
    let mut i = 0usize;
    loop {
        let x = (words.marks[i] + scroll) as i32;
        let next = i + 1;
        if 0x163 < x || 0.0 <= scroll {
            if 0x1a3 < x && 0.0 < scroll {
                words.marks[i] = if i == 8 { words.marks[0] - 8.0 } else { words.marks[next] - 8.0 };
            }
        } else if i == 0 {
            words.marks[0] = words.marks[8] + 8.0;
        } else {
            words.marks[i] = words.marks[i - 1] + 8.0;
        }
        words.marks[i] += scroll;
        let x = words.marks[i] as i32;
        let t = if x < 0x173 { (x as f32 - 355.0) * 0.066_666_67 } else if 0x194 < x { (420.0 - x as f32) * 0.066_666_67 } else { 1.0 };
        rect(out, x, 21, x + 1, 55, lerp_rgba(t.clamp(0.0, 1.0), 0x10f0_c070, 0x25f0_c070));
        i = next;
        if 9 <= i { break; }
    }
    let ll = local(&rows, sub(w.mobys[id].pos, frame.origin));
    let bike_y = ((ll[1] * 0.2).clamp(-1.0, 1.0) * 13.0) as i32 + 0x24;
    let way = atan2(rows[0][0], rows[0][1]);
    let sub_state = w.mobys[id].timer;
    let set = super::RouteSet::of(&w.mobys[id]);
    for k in 0..21 {
        let Some(r) = usize::try_from(w.mobys[id].pv_i(0x1e4 + 4 * k)).ok().filter(|&r| r < w.mobys.len()) else { continue };
        if !(w.mobys[r].state == 2 || sub_state == 1 || sub_state == 3) { continue; }
        if w.mobys[r].pvars.len() < 0x2e4 { w.mobys[r].pvars.resize(0x2e4, 0); }
        let rl = local(&rows, sub(w.mobys[r].pos, frame.origin));
        let x = (rl[0] + rl[0]) as i32 + 0x183;
        let y4 = (rl[1] * 4.0) as i32;
        let y = y4 + 0x24;
        let m = &mut w.mobys[r];
        if ((y4 + 0xd) as u32) < 0x1b && x < 0x1a5 && 0x167 < x {
            let mut t = if x < 0x173 { (x as f32 - 360.0) * 0.1 } else if 0x199 < x { (420.0 - x as f32) * 0.1 } else { 1.0 };
            if t == 1.0 {
                if y < 0x1d { t = (y as f32 - 23.0) * 0.2; } else if 0x2c <= y { t = (49.0 - y as f32) * 0.2; }
            }
            let colour = lerp_rgba(t, 0x1000_40e4, 0x6000_40e4);
            let yaw = sub_rot(way, m.rot[2]);
            ease_yaw(m, 0x1d4, 0x1e0, yaw);
            ease_int(m, 0x1cc, 0x1d8, x);
            ease_int(m, 0x1d0, 0x1dc, y);
            out.push(ScreenDraw::SpriteMat { id: 0x3e, pos: [m.pv_i(0x1cc) as f32, m.pv_i(0x1d0) as f32], frame: 0.0, m: turned(m.pv_f(0x1d4)), colour });
        } else {
            m.set_pv_i(0x1d0, -1);
            m.set_pv_f(0x1d4, -1.0);
            m.set_pv_i(0x1cc, -1);
            for o in [0x1e0, 0x1d8, 0x1dc] { m.set_pv_i(o, 0); }
        }
        let (link, point) = (m.pv_i(0x120), m.pv_i(0x124));
        let (mut sec, mut seg) = (m.pv_i(0x1c4), m.pv_i(0x1c8));
        let [mx, my] = map_place(&w.race.paths, &set, level, link, point, &mut sec, &mut seg);
        let m = &mut w.mobys[r];
        m.set_pv_i(0x1c4, sec);
        m.set_pv_i(0x1c8, seg);
        rect(out, mx - 2, my - 1, mx + 2, my + 3, 0x6000_40e4);
    }
    let m = &mut w.mobys[id];
    let yaw = sub_rot(way, m.rot[2]);
    ease_yaw(m, 0x4a4, 0x4b0, yaw);
    m.set_pv_i(0x49c, 0x183);
    ease_int(m, 0x4a0, 0x4ac, bike_y);
    out.push(ScreenDraw::SpriteMat { id: 0x3e, pos: [m.pv_i(0x49c) as f32, m.pv_i(0x4a0) as f32], frame: 0.0, m: turned(m.pv_f(0x4a4)), colour: 0x8000_a6e4 });
    let (link, point) = (m.pv_i(0x314), m.pv_i(0x318));
    let (mut sec, mut seg) = (m.pv_i(0x494), m.pv_i(0x498));
    let [mx, my] = map_place(&w.race.paths, &set, level, link, point, &mut sec, &mut seg);
    let m = &mut w.mobys[id];
    m.set_pv_i(0x494, sec);
    m.set_pv_i(0x498, seg);
    rect(out, mx, my, mx + 4, my + 4, 0x8000_a6e4);
}

/// An eased yaw (`0x31f230(yaw, 2π/2700, 2π/2700, 3π/2, &a, &v, 0)`; −1 unset: taken at once).
fn ease_yaw(m: &mut crate::moby::Moby, a: usize, v: usize, yaw: f32) {
    if m.pv_f(a) == -1.0 {
        m.set_pv_f(a, yaw);
    } else {
        let (mut x, mut s) = (m.pv_f(a), m.pv_f(v));
        turn_ease(yaw, 0.002_327_1, 0.002_327_1, 4.712_389, &mut x, &mut s, 0);
        m.set_pv_f(a, x);
        m.set_pv_f(v, s);
    }
}

/// An eased screen place kept as an int (`0x31ed78(x, 5/36, 5/36, 35/6, &f, &v)`; −1 unset: taken at once).
fn ease_int(m: &mut crate::moby::Moby, at: usize, v: usize, x: i32) {
    if m.pv_i(at) == -1 {
        m.set_pv_i(at, x);
    } else {
        let (mut f, mut s) = (m.pv_i(at) as f32, m.pv_f(v));
        ease(x as f32, 0.138_888_9, 0.138_888_9, 5.833_333_5, &mut f, &mut s);
        m.set_pv_i(at, f as i32);
        m.set_pv_f(v, s);
    }
}

/// One stretch of the track map: on the branch `link` (−1 the route) from the point `from` (the route: `from..=to`,
/// the stretch's share `(point − from) / span`; a branch: `point / its count`), drawn along `points`.
struct Stretch {
    link: i32,
    from: i32,
    to: i32,
    span: f32,
    points: &'static [[i32; 2]],
}

const fn route(from: i32, to: i32, span: f32, points: &'static [[i32; 2]]) -> Stretch { Stretch { link: -1, from, to, span, points } }
const fn branch(link: i32, points: &'static [[i32; 2]]) -> Stretch { Stretch { link, from: 0, to: 0, span: 0.0, points } }

/// L04 `0x43c4e8`'s stretches in its order (their lines `0x1b12a0`, `0x1b12d8`, `0x2ca520`, `0x1b12f0`, `0x2ca578`,
/// `0x1b1320`, `0x1b1338`; the stretch numbers 1..7).
const MAP_04: [Stretch; 7] = [
    route(0, 0x97, 151.0, &[[467, 18], [485, 18], [490, 22], [490, 43], [487, 48], [481, 48], [477, 43]]),
    route(0x97, 0xae, 23.0, &[[477, 43], [466, 43], [463, 46]]),
    route(0xae, 0x138, 138.0, &[[463, 46], [462, 48], [452, 48], [447, 45], [447, 36], [451, 31], [468, 31], [473, 36], [473, 53], [469, 59], [465, 59]]),
    route(0x138, 500, 186.0, &[[465, 59], [451, 59], [434, 40], [434, 23], [438, 18], [467, 18]]),
    branch(0, &[[477, 43], [477, 29], [473, 25], [455, 25], [451, 28], [451, 40], [455, 44], [460, 44], [463, 46]]),
    branch(2, &[[463, 46], [465, 49], [465, 59]]),
    branch(1, &[[477, 43], [466, 54]]),
];

/// L11 `0x4592b0`'s stretches in its order (their lines `0x1b1ec0..0x1b1fe0`; the stretch numbers 1..10).
const MAP_11: [Stretch; 10] = [
    route(0, 0x36, 54.0, &[[433, 26], [433, 24], [437, 17], [442, 17]]),
    route(0x36, 0x6e, 56.0, &[[442, 17], [463, 17]]),
    branch(0, &[[442, 17], [448, 22], [460, 22], [463, 17]]),
    route(0x6e, 0x76, 8.0, &[[463, 17], [469, 17]]),
    route(0x76, 0x139, 195.0, &[[469, 17], [485, 17], [490, 21], [490, 53], [486, 59], [476, 59]]),
    branch(1, &[[469, 17], [469, 26], [473, 31], [476, 31], [480, 35], [480, 47], [476, 52], [476, 59]]),
    route(0x139, 0x141, 8.0, &[[476, 59], [468, 59]]),
    route(0x141, 0x1d4, 147.0, &[[468, 59], [446, 59], [433, 45], [433, 36]]),
    branch(2, &[[468, 59], [463, 54], [451, 54], [433, 36]]),
    route(0x1d4, 500, 32.0, &[[433, 36], [433, 26]]),
];

/// `0x40b580(n, points, ways, lengths, starts, &total)` (the level's set-up `0x43c380` / L11 `0x4590b8` runs it on each
/// stretch): each segment's length, its start along the line and its unit way.
fn segments(points: &[[i32; 2]]) -> (Vec<f32>, Vec<f32>, Vec<[f32; 2]>, f32) {
    let (mut lengths, mut starts, mut ways, mut total) = (Vec::new(), Vec::new(), Vec::new(), 0.0f32);
    for p in points.windows(2) {
        let d = [(p[1][0] - p[0][0]) as f32, (p[1][1] - p[0][1]) as f32];
        let l = (d[0] * d[0] + d[1] * d[1]).sqrt();
        lengths.push(l);
        starts.push(total);
        total += l;
        ways.push([d[0] / l, d[1] / l]);
    }
    (lengths, starts, ways, total)
}

/// `0x43c4e8(&x, &y, link, point, &stretch, &segment, bike)` (L11 `0x4592b0`): the place on the track map of the
/// point `point` of the branch `link` — its stretch ([`MAP_04`] / [`MAP_11`], the first that holds it), its share of
/// it (at most 1) along the stretch's line, from the segment it was on last when the stretch is the same (stepping
/// back or on), else from the first; none found: (0, 0), the stretch −1.
pub fn map_place(paths: &super::Paths, set: &super::RouteSet, level: i32, link: i32, point: i32, stretch: &mut i32, segment: &mut i32) -> [i32; 2] {
    let table: &[Stretch] = if level == 0xb { &MAP_11 } else { &MAP_04 };
    let found = table.iter().enumerate().find(|(_, s)| {
        s.link == link && (s.link != -1 || (s.from <= point && point <= s.to))
    });
    let Some((k, s)) = found else {
        *stretch = -1;
        *segment = 0;
        return [0, 0];
    };
    let f = if s.link == -1 {
        (point - s.from) as f32 / s.span
    } else {
        let r = usize::try_from(s.link).ok().and_then(|l| set.links.get(l)).copied().unwrap_or(-1);
        point as f32 / paths.count(r) as f32
    };
    let (lengths, starts, ways, total) = segments(s.points);
    let n = lengths.len() as i32;
    let f = f.min(1.0) * total;
    let id = k as i32 + 1;
    let at = |i: i32| i.clamp(0, n - 1) as usize;
    let mut i = 0;
    let base;
    if id == *stretch {
        i = at(*segment) as i32;
        let mut c = starts[at(i)];
        if f < c {
            while f < c {
                i -= 1;
                if i < 0 { i = n - 1; }
                c -= lengths[at(i)];
            }
        } else {
            let mut more = c < f;
            while more {
                c += lengths[at(i)];
                more = c < f;
                if more {
                    i += 1;
                    if n <= i { i = 0; }
                }
            }
        }
        base = starts[at(i)];
        *segment = i;
    } else {
        let mut c = 0.0;
        if 0.0 <= f {
            c = lengths[0];
            if c <= f {
                loop {
                    i += 1;
                    if f < c { break; }
                    c += lengths[at(i)];
                    if f < c { c -= lengths[at(i)]; break; }
                }
            } else {
                c = 0.0;
            }
        }
        base = c;
        *segment = i;
        *stretch = id;
    }
    let p = s.points[at(i)];
    let way = ways[at(i)];
    [(p[0] as f32 + way[0] * (f - base)) as i32, (p[1] as f32 + way[1] * (f - base)) as i32]
}

/// `0x40dd90(bike)`: the countdown's lights. `t` the countdown's ticks gone (sub-states 1..4: 150 − `+0x394`; else
/// the race clock + 150); light `k` (0..3) fades in from none to its bright colour over `30k..30k + 30` and dims to
/// its dim one over the next 15; the last fades in over 120..150, stays to 156 and dims to 171 ([`LIGHT_BRIGHT`],
/// [`LIGHT_DIM`]). Both sides: the frame 0x3a (`0x60442d00`), 0x3b and 0x3c (`0x55f0c070`) at (10, 120) and,
/// mirrored, (498, 120); the lights 0x3d at 0.8 down at x 31 / 478 ([`LIGHT_Y`]).
fn lights(w: &MobyWorld, id: usize, out: &mut Vec<ScreenDraw>) {
    let b = &w.mobys[id];
    let t = if (1..=4).contains(&b.timer) { -b.pv_i(0x394) } else { w.race.clock } + 0x96;
    let mut c = [0u32; 5];
    for (k, c) in c.iter_mut().enumerate() {
        let k = k as i32;
        let (s, e, f, m) = if k == 4 { (0x78, 0x9c, 0xab, 0x95) } else { (k * 30, k * 30 + 30, k * 30 + 45, k * 30 + 30) };
        let (a, b) = (LIGHT_BRIGHT[k as usize], LIGHT_DIM[k as usize]);
        *c = if t < s {
            0
        } else if t < m {
            lerp_rgba((t - s) as f32 * 0.033_333_335, 0, a)
        } else if e <= t {
            if f <= t { b } else { lerp_rgba((t - e) as f32 * 0.066_666_67, a, b) }
        } else {
            a
        };
    }
    for side in 0..2 {
        let (x, sx, lx) = if side == 0 { (10.0, 1.0, 31.0) } else { (498.0, -1.0, 478.0) };
        sprite(out, 0x3a, [x, 120.0], [sx, 1.0], 0.0, 0x6044_2d00);
        sprite(out, 0x3b, [x, 120.0], [sx, 1.0], 0.0, 0x55f0_c070);
        sprite(out, 0x3c, [x, 120.0], [sx, 1.0], 0.0, 0x55f0_c070);
        for k in 0..5 { sprite(out, 0x3d, [lx, LIGHT_Y[k] as f32], [0.8, 0.8], 0.0, c[k]); }
    }
}

/// L04 `0x1b0960` / `0x1b0990` / `0x1b09a8`: the lights' heights and their bright and dim colours.
pub const LIGHT_Y: [i32; 5] = [0xa2, 0xc7, 0xed, 0x111, 0x136];
pub const LIGHT_BRIGHT: [u32; 5] = [0x8000_00ff, 0x8000_00ff, 0x8000_ffff, 0x8000_ffff, 0x8000_ff00];
pub const LIGHT_DIM: [u32; 5] = [0x6000_40e4, 0x6000_40e4, 0x6000_a6e4, 0x6000_a6e4, 0x602c_c029];

/// `0x4185d8`: the lap's time — `"%s %s %d : %d :0%d.%d"` (`" : %d.%d"` from 10 seconds) of 0x2e18 with the lap, 0x2e19
/// and the lap's ticks (`$gp+0x18e0`) as hours, minutes, seconds and tenths, font 3 centred at (256, 118) in
/// `0x80f0f0f0` on the box 112..142 down, 50 in from either side.
fn lap_box(w: &MobyWorld, inp: &ScreenIn, out: &mut Vec<ScreenDraw>) {
    let lap = format(&text_of(inp, 0x2e18, usize::MAX), &[Arg::D(w.race.lap)]);
    let label = text_of(inp, 0x2e19, usize::MAX);
    let [h, m, s, t] = split(w.race.lap_time);
    let fmt: &[u8] = if s < 10 { b"%s %s %d : %d :0%d.%d" } else { b"%s %s %d : %d : %d.%d" };
    let line = format(fmt, &[Arg::S(&lap), Arg::S(&label), Arg::D(h), Arg::D(m), Arg::D(s), Arg::D(t)]);
    let sw = crate::gui::SCREEN_W as i32;
    out.push(ScreenDraw::Panel { rect: [112, 142, 0x32, sw - 0x32], alpha: 0x60, fill: 0x6044_2d00, edge: 0x55f0_c070 });
    text(out, 3, line, sw / 2, 118, 0x80f0_f0f0, Align::Centre);
}

/// `0x418348`: ridden after a retry, the box asking for a button — the text 0xa41 laid out centred about (256, 299)
/// between 44 and 468 (`0x2eb050`, lines 16 apart, in font 2) and 0x2dc2 in font 2 centred at (256, 349), both in
/// `0x80f0f0f0`, on the box 263..371 down as wide as the wider of the two plus 6 on each side (L04 `$gp+0x1adc`).
fn retry_box(inp: &ScreenIn, out: &mut Vec<ScreenDraw>) {
    const PAD: i32 = 6;
    let wrap = text_of(inp, 0xa41, usize::MAX);
    let line = text_of(inp, 0x2dc2, usize::MAX);
    let mut layout = crate::help::Layout { top: 222, bottom: 479, x0: 0x2c, x1: 0x1d4, x: 0x100, y: 299, line: 0x10, flags: 3 | 4 };
    let (measured, wide) = match inp.glyphs_2 {
        Some(g) => (crate::help::text_box(&mut Vec::new(), &wrap, &layout, g, [0; 4]).0, crate::gui::text_width(g, &line, 1.0)),
        None => (0, 0),
    };
    let half = measured.max(wide) >> 1;
    let mid = crate::gui::SCREEN_W as i32 >> 1;
    out.push(ScreenDraw::Panel { rect: [263, 371, mid - half - PAD, mid + half + PAD], alpha: 0x60, fill: 0x6044_2d00, edge: 0x55f0_c070 });
    layout.flags &= !4;
    out.push(ScreenDraw::Wrap { text: wrap, layout, colour: 0x80f0_f0f0 });
    text(out, 2, line, mid, 349, 0x80f0_f0f0, Align::Centre);
}

/// `0x412490` (the decomp's `func_L04_00410F80`, at the finish): the standings chain — the bike and its racers
/// (`+0x1e4`, 20 slots) sorted by their finish time (the bike's the race clock; a racer not finished yet given one:
/// the clock + 3 × the route it has left, its laps to go counted, at least 6 — 10..20 at random when that is none),
/// the chain's head `+0x39c`, each one's next (the bike's `+0x3a0`, a racer's `+0x1a4`), their places (the bike's
/// `+0x320`, a racer's `+0xa2`).
pub fn standings_chain(w: &mut MobyWorld, id: usize, ctx: &mut crate::moby::Ctx) {
    let clock = w.race.clock;
    let lap = w.race.lap;
    let main = w.mobys[id].pv_i(super::route::MAIN);
    let length = w.race.paths.count(main) as f32 * w.race.paths.step(main);
    let mut list = vec![id];
    for k in 0..20 {
        let Some(r) = usize::try_from(w.mobys[id].pv_i(0x1e4 + 4 * k)).ok().filter(|&r| r < w.mobys.len()) else { continue };
        list.push(r);
        let m = &mut w.mobys[r];
        if m.pvars.len() < 0x2e4 { m.pvars.resize(0x2e4, 0); }
        if m.pv_i(0x1a0) == 0 {
            let mut d = length - m.pv_f(0x11c) + length * (lap - m.pv_i(0x160)) as f32;
            if d <= 0.0 { d = ctx.game.rng.randf(10.0, 20.0); }
            let t = ((d * 3.0) as i32).max(6);
            w.mobys[r].set_pv_i(0x1a0, t + clock);
        }
    }
    let time = |w: &MobyWorld, m: usize| match w.mobys[m].o_class {
        super::BIKE => clock,
        super::RACER => w.mobys[m].pv_i(0x1a0),
        _ => 0,
    };
    let n = list.len();
    for i in 0..n.saturating_sub(1) {
        for j in 0..n - i - 1 {
            if time(w, list[j + 1]) < time(w, list[j]) { list.swap(j, j + 1); }
        }
    }
    crate::moby::ai::set_link(&mut w.mobys[id], 0x39c, Some(list[0]));
    for (i, &m) in list.iter().enumerate() {
        let next = list.get(i + 1).copied();
        match w.mobys[m].o_class {
            super::BIKE => {
                w.mobys[id].set_pv_i(0x320, i as i32 + 1);
                crate::moby::ai::set_link(&mut w.mobys[id], 0x3a0, next);
            }
            super::RACER => {
                let r = &mut w.mobys[m];
                r.set_pv_b(0xa2, (i + 1) as u8);
                crate::moby::ai::set_link(r, 0x1a4, next);
            }
            _ => {}
        }
    }
}

/// `0x4127a8` (module docs).
pub fn standings(w: &mut MobyWorld, id: usize, inp: &ScreenIn, out: &mut Vec<ScreenDraw>) -> bool {
    if w.mobys.get(id).is_none_or(|m| m.pvars.len() < 0x770) { return true; }
    let game = inp.game;
    let finished = w.race.finished;
    let place = w.mobys[id].pv_i(0x320);
    let mid = crate::gui::SCREEN_W as i32 / 2;
    {
        let s = &mut w.race.hud;
        if finished < 10 { s.all_in = false; }
        let grow = if place < 10 { s.all_in } else { 600 < finished };
        if grow {
            if !s.list_done { s.list += 0.416_666_7; }
            if !s.grow_done { s.grow += 0.416_666_7; }
        } else {
            s.list = 0.0;
            s.grow = 0.0;
            s.list_done = false;
            s.grow_done = false;
        }
        if finished % 15 == 0 {
            s.dots += 1;
            if 3 < s.dots { s.dots = 0; }
        }
    }
    if place == 1 {
        let wins = super::bike::wins_in(game, w.race.race).min(3);
        sprite(out, 0x39, [mid as f32, 194.0], [0.92, 0.92], 0.0, 0x6044_2d00);
        sprite(out, 0x38, [mid as f32, 194.0], [0.92, 0.92], 0.0, 0x55f0_c070);
        text(out, 1, text_of(inp, 0x2e29, usize::MAX), mid, 32, 0x80f0_f0f0, Align::Centre);
        if w.race.skill_shown < 1 {
            let prize = super::record(game.level, w.race.race).prizes[(wins - 1).clamp(0, 2) as usize];
            if prize != 0 {
                let prize = if game.challenge { prize * 10 } else { prize };
                text(out, 1, crate::hud::format_int(&text_of(inp, 0x2e5c, usize::MAX), prize), mid, 65, 0x80f0_f0f0, Align::Centre);
            }
        } else {
            text(out, 1, text_of(inp, 0x1233, usize::MAX), mid, 65, 0x80f0_f0f0, Align::Centre);
            w.race.skill_shown -= 1;
        }
    }
    let mut top = 0x13b - w.race.hud.grow as i32;
    if top < 0x9d { top = 0x9d; }
    if top == 0x9d { w.race.hud.grow_done = true; }
    let frame = (1.0 - (315.0 - top as f32) * 0.006_329_114) * 0.72 + 0.08;
    sprite(out, 0x47, [mid as f32, 196.0], [0.92, 0.92], frame, 0x6044_2d00);
    sprite(out, 0x46, [mid as f32, 196.0], [0.92, 0.92], frame, 0x55f0_c070);
    let place_text = |p: i32| text_of(inp, PLACE_TEXTS[(p - 1).clamp(0, 19) as usize], usize::MAX);
    let title = format(b"%s %s %s", &[Arg::S(&text_of(inp, 0x2e15, usize::MAX)), Arg::S(&place_text(place)), Arg::S(&text_of(inp, 0x2e16, usize::MAX))]);
    text(out, 1, title, mid, 106, 0x80f0_f0f0, Align::Centre);
    text(out, 2, text_of(inp, 0x2dd8, usize::MAX), 0x186, top, 0x80f0_f0f0, Align::Right);
    let clock = w.race.clock;
    let time_text = |t: i32| {
        let [h, m, s, d] = split(t);
        let fmt: &[u8] = if s < 10 { b"%s %d : %d :0%d.%d" } else { b"%s %d : %d :%d.%d" };
        format(fmt, &[Arg::S(&text_of(inp, 0x2e19, usize::MAX)), Arg::D(h), Arg::D(m), Arg::D(s), Arg::D(d)])
    };
    let (mut rows, mut waiting, mut seen_bike) = (0, 0, false);
    let mut y = 0x8a;
    let mut cur = crate::moby::ai::link(&w.mobys[id], 0x39c);
    w.race.hud.all_in = true;
    while let Some(o) = cur.filter(|&o| o < w.mobys.len()) {
        y = rows * 15 - (w.race.hud.list as i32 - 0x8a);
        let (p, time);
        match w.mobys[o].o_class {
            super::BIKE => {
                if y < 0x8b {
                    y = 0x8a;
                    w.race.hud.list_done = true;
                }
                p = place;
                seen_bike = true;
                cur = crate::moby::ai::link(&w.mobys[id], 0x3a0);
                time = clock;
            }
            super::RACER => {
                let r = &w.mobys[o];
                p = r.pv_b(0xa2) as i32;
                cur = crate::moby::ai::link(r, 0x1a4);
                time = r.pv_i(0x1a0);
                if 0xb <= p { continue; }
                if place < p {
                    if clock + finished < time {
                        w.race.hud.all_in = false;
                        waiting += 1;
                    }
                    // `0x383ff0(0x16, 0x11, bike)` as it comes in: played by the bike's tick (crate::moby::classes::race::bike).
                }
            }
            _ => {
                p = 0;
                cur = None;
                time = 0;
            }
        }
        if y < 0x80 {
            if p < 0xb { rows += 1; }
            continue;
        }
        if top - 10 < y { continue; }
        let (full, clear) = if p == place { (0x8000_c0c0u32, 0x8000_c0c0u32) } else { (0x80f0_f0f0, 0x00f0_f0f0) };
        let mut colour = full;
        if y < 0x8a {
            colour = lerp_rgba((y as f32 - 128.0) * 0.1, clear, full);
        } else if place < p && top - y < 0x15 {
            colour = lerp_rgba(((top - y) - 10) as f32 * 0.1, clear, full);
        }
        let name = format(b"%s %s", &[Arg::S(&place_text(p)), Arg::S(&text_of(inp, 0x2e17, usize::MAX))]);
        text(out, 2, name, 0xaf, y, colour, Align::Centre);
        let right = if waiting < 1 {
            time_text(time)
        } else {
            let fmt: &[u8] = if w.race.hud.dots == 3 || 1 < waiting {
                b"            %s..."
            } else if w.race.hud.dots == 2 {
                b"            %s.."
            } else if w.race.hud.dots == 1 {
                b"            %s."
            } else {
                b"            %s"
            };
            format(fmt, &[Arg::S(&text_of(inp, 0x2e46, usize::MAX))])
        };
        rows += 1;
        text(out, 2, right, 0xdc, y, colour, Align::Left);
        if 9 < rows { break; }
    }
    if 10 < place && !w.race.hud.list_done && !seen_bike {
        let name = format(b"%s %s", &[Arg::S(&place_text(place)), Arg::S(&text_of(inp, 0x2e17, usize::MAX))]);
        text(out, 2, name, 0xaf, y + 15, 0x8000_c0c0, Align::Centre);
        text(out, 2, time_text(clock), 0xc4, y + 15, 0x8000_c0c0, Align::Left);
    }
    true
}
