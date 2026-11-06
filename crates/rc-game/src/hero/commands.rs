//! Clank's commands to the little robots (Endako, the Aranos prison): the ring he picks them from, the command word
//! the robots read (`0x18c304`, crate::moby::classes::endako_d) and his antenna's signal for it. L03 addresses (the
//! same engine code on every level, by its L00 places where given).
//!
//! **The ring** (GUI screen 0x23 with Clank's callbacks: init `0x2fa5a8`, update `0x2fa5f0`, draw `0x2faaf8`; L00
//! `0x2f9380` / `0x2f93c8` / `0x2f98d0`). The screen that △ opens is Ratchet's quick select (crate::quick_select)
//! while the body is Ratchet and this ring while it is Clank (`0x18c0d4` = 1): its openers (the hero's transitions
//! `0x2cd0d0`, the paused mode `0x2f4b00` / `0x2f4bd8`) register Clank's callbacks for it, and the switch to Clank
//! (`0x2b85b8`) does so for a screen already up. Its record: `+0x70` the fade, `+0x71` the name's fade, `+0x74` the
//! selected slot (−2 at the init, −1 none), `+0x78` the reset's countdown (0x1e; the top byte 1 once reset).
//!
//! Its frame ([`Commands::update`]): the input lock 2 for the next frame (`0x138180 + 0x1cc`: the hero's buttons and
//! stick off while it is up); the left stick (`+0x148` / `+0x14c`, unmirrored) normalised. Until the reset, with
//! robots near (`0x1b275c` ≠ 0) a d-pad press, the stick within 0.5 or the countdown's end (30 frames) resets it
//! (none selected, `0x10000ff`). After it: the stick past 0.9 picks the slot its angle points at (slot 0 at the top,
//! clockwise, eight even slots); with the stick centred the d-pad walks the table's neighbours (up, down, left, right
//! in turn; from none the table's first ones, `0x1a9108..0x1a9114`). A new slot restarts the name's fade and the
//! pulse (`0x31f038(…, 1)`) and plays the GUI's sound 3; else the name fades in (8 steps). △ let go: on its release
//! the slot's command (none: 0) is the command word; with Quick Select Pause on the paused mode ends (`0x2f51f0`);
//! the fade steps out, and out the screen closes (`0x2f8e00`). △ held: the fade steps in. Commands 6, 7 and 5 need a
//! robot of their kind near (0x0940, 0x0a59, 0x093c: `0x1b2760` / `0x1b2764` / `0x1b2762`), else the word is 0.
//!
//! The table (`0x282450`, 8 × 0x1c: the icon set 0x7553, its frame, the d-pad's neighbours left / right / up / down,
//! the command), from the top clockwise: 8 call off (0x0964 to its post, the others' bow, state 6), 1 follow, 5 lift
//! (0x093c), 3 attack (0x0964), 7 zap (0x0a59), 4 enter (0x0964), 6 bridge (0x0940), 2 wait.
//!
//! **The draw** ([`Commands::draw`], `0x2faaf8`): the ring's backs (sprites 8..15), its halves (7, and 0x86 glowing,
//! each drawn again mirrored) at (129, 208); the selected slot's highlight (sprite 0x10, frame slot − 1) and its bar
//! (`0x2fd380`: 33 × 7 at the slot's place, `0x1a9180`) pulsing `0x80442d00` .. `0x80ffde8d` over 50 frames
//! (`0x31f038`, the pulse counter `0x1aa1d4`); each slot's icon (frame 10 behind it in the kind's colour where the
//! command needs robots near; the bridge's icon 6 is 7 with no 0x0940 standing as one, `0x1b275e`), lit
//! (`0x80ffde8d`) when the robots it needs are near, else grey; and the lit selected slot's name (`0x1a91c0`; the
//! bridge's 0x2dca is 0x2dcb while one stands) at (128, 200) in black and at (129, 201) in `0xe0c0a0`, faded by both
//! fades (`0x2eb168`, centred).
//!
//! **The robots' counts** ([`Robots`], `0x1b275c..0x1b2764`): the main loop zeroes them each frame with Clank's body
//! and the ring not up, outside the paused mode (`0x371b48`; [`frame_start`]); the robots count themselves in
//! (crate::moby::classes::endako_d): awake within 16 across of Clank (and by kind), and in the bridge's sub-state 4.
//!
//! **The command word** (`0x18c304`): read by every robot in the moby loop of the next frame, then the body's tick
//! (`0x2a65d8`, [`tick`]) plays Clank's class sound `0x10 + command`, starts the antenna's signal (`0x18c340` = 0x39,
//! its command `0x18c342`) and clears it. The antenna (`0x2ac520`): the signal's countdown and, past 0x31 on its odd
//! frames, the sound again.
//!
//! Not ported: the antenna's moby 0x04b4 at Clank's joint 7 and its glow (`0x18b684`, `0x18c344`: the colours by the
//! command, G-HERO-004), the screen's timers `+0x6c` / `+0x7c` and the paused mode's frame count `gp−0x6240`
//! (`0x2f5238`), the record's copy at the close (`0x1b27bc`, `gp+0x37d0`), the table's other sets (`gp−0x5f58` is
//! always 0 here).

use crate::hud::{lerp_rgba, Align, Draw};
use crate::pad::{button, PadState};

/// The table's rows (`0x282450`): the icon set, its frame, the d-pad's neighbours left / right / up / down, the
/// command.
const TABLE: [(u16, u16, [i32; 4], i32); SLOTS] = [
    (ICONS, 9, [7, 1, 4, 4], 8),
    (ICONS, 2, [0, 2, 0, 2], 1),
    (ICONS, 4, [6, 6, 1, 3], 5),
    (ICONS, 0, [4, 2, 2, 4], 3),
    (ICONS, 8, [5, 3, 0, 0], 7),
    (ICONS, 1, [6, 4, 6, 4], 4),
    (ICONS, 6, [2, 2, 7, 5], 6),
    (ICONS, 3, [6, 0, 0, 6], 2),
];
/// The icon set (`+0x00` of every row).
const ICONS: u16 = 0x7553;
/// `0x1b2798`: the slots.
const SLOTS: usize = 8;
/// `0x1a9108..0x1a9114`: the slot a d-pad press picks with none selected, left, right, up, down.
const DPAD_FIRST: [i32; 4] = [6, 2, 0, 4];
/// `0x1a9180`: the highlight bars' places.
const BARS: [[f32; 2]; SLOTS] = [[114.0, 148.0], [170.0, 169.0], [194.0, 222.0], [170.0, 275.0], [114.0, 296.0], [58.0, 275.0], [34.0, 222.0], [58.0, 169.0]];
/// `0x1a91c0`: the slots' names.
const NAMES: [u32; SLOTS] = [0x312b, 0x2db0, 0x2dc7, 0x2db2, 0x2dc8, 0x2db1, 0x2dca, 0x2daf];
/// The bridge's name while one stands.
const UNBRIDGE: u32 = 0x2dcb;
/// The ring's centre.
const AT: [f32; 2] = [129.0, 208.0];
/// The signal's start (`0x18c340`) and the frames past which the antenna sounds it again.
const SIGNAL: i16 = 0x39;
const SIGNAL_SOUND: i16 = 0x31;

/// The robots' counts this frame (`0x1b275c..0x1b2764`).
#[derive(Clone, Copy, Debug, Default)]
pub struct Robots {
    /// `0x1b275c`: awake within 16 across of Clank; `0x1b275e`: 0x0940s standing as a bridge (sub-state 4).
    pub near: u16,
    pub bridges: u16,
    /// `0x1b2760` / `0x1b2762` / `0x1b2764`: the near ones of 0x0940, 0x093c and 0x0a59.
    pub near_940: u16,
    pub near_93c: u16,
    pub near_a59: u16,
}

/// The ring's record (`+0x70..+0x7b`).
#[derive(Clone, Copy, Debug)]
struct Screen {
    fade: u8,
    name_fade: u8,
    selected: i32,
    reset: i32,
}

/// What a frame of the ring asks of the game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Out {
    /// The paused mode ends (`0x2f51f0`).
    Resume,
    /// Sound 3 of the GUI's set (`0x361f28(3, 0, 0)`): a new slot.
    Sound(u32),
}

/// Clank's commands (module docs).
#[derive(Clone, Debug, Default)]
pub struct Commands {
    /// `0x18c304`: the command of this frame (0 none).
    pub command: i32,
    /// `0x18c340` / `0x18c342`: the antenna's signal countdown and its command.
    pub signal: i16,
    pub signal_command: i16,
    pub robots: Robots,
    screen: Option<Screen>,
    /// `0x1aa1d4`: the pulse's frame counter (`0x31f038` mode 0).
    pulse: i32,
}

impl Commands {
    /// `0x2f79d8(0x23, …)` with Clank's init `0x2fa5a8`: the ring opens (once).
    pub fn open(&mut self) {
        if self.screen.is_none() { self.screen = Some(Screen { fade: 0, name_fade: 0, selected: -2, reset: 0x1e }); }
    }

    pub fn is_open(&self) -> bool { self.screen.is_some() }

    /// `0x2fa5f0`: the ring's frame (`pause`: the Quick Select Pause option). The command word is set here.
    pub fn update(&mut self, pad: &mut PadState, pause: bool) -> Vec<Out> {
        let mut out = Vec::new();
        let Some(mut s) = self.screen else { return out };
        pad.lock = 2;
        let [mut x, mut y] = [pad.analog_copy[2].to_f32(), pad.analog_copy[3].to_f32()];
        let mag = (x * x + y * y).sqrt();
        if mag != 0.0 {
            x /= mag;
            y /= mag;
        }
        let angle = crate::hero::physics::atan2(x, y);
        let pressed = pad.pressed_unmirrored;
        let dpad = pressed & 0xf000;
        if s.reset >> 24 == 0 && self.robots.near != 0 && (dpad != 0 || mag < 0.5 || { s.reset -= 1; s.reset == -1 }) {
            s.selected = -1;
            s.reset = 0x0100_00ff;
        }
        let old = s.selected;
        if s.reset >> 24 == 1 {
            if 0.0 < mag {
                if 0.9 < mag {
                    use std::f32::consts::{FRAC_PI_2, PI};
                    let n = SLOTS as f32;
                    s.selected = (((angle + PI + PI + FRAC_PI_2 + PI / n) * 0.159_154_94 * n) as i32) % SLOTS as i32;
                }
            } else if dpad != 0 {
                let mut c = old;
                for (bit, dir) in [(button::UP, 2), (button::DOWN, 3), (button::LEFT, 0), (button::RIGHT, 1)] {
                    if pressed & bit == 0 { continue; }
                    c = if c == -1 { DPAD_FIRST[dir] } else { TABLE[c.rem_euclid(SLOTS as i32) as usize].2[dir] };
                }
                s.selected = c;
            }
        }
        if s.selected == old {
            if s.name_fade < 8 { s.name_fade += 1; }
        } else {
            self.pulse = 0;
            s.name_fade = 0;
            out.push(Out::Sound(3));
        }
        if pad.held_unmirrored & button::TRIANGLE == 0 {
            if pad.released_unmirrored & button::TRIANGLE != 0 {
                // None selected (−1, or −2 before the reset: the table's word before it is 0) gives no command.
                self.command = usize::try_from(s.selected).ok().and_then(|k| TABLE.get(k)).map_or(0, |r| r.3);
            }
            if pause { out.push(Out::Resume); }
            if s.fade != 0 { s.fade -= 1; }
        } else if s.fade < 8 {
            s.fade += 1;
        }
        let r = self.robots;
        if (self.command == 6 && r.near_940 == 0) || (self.command == 7 && r.near_a59 == 0) || (self.command == 5 && r.near_93c == 0) {
            self.command = 0;
        }
        self.screen = (s.fade != 0).then_some(s);
        out
    }

    /// `0x31f038(first, second, span, 0, 0)`: the pulse's colour, `second` to `first` and back over `2·span` frames.
    fn pulse(&mut self, first: u32, second: u32, span: i32) -> u32 {
        self.pulse += 1;
        let phase = self.pulse % (span * 2);
        let t = if span <= phase { 1.0 - (phase - span) as f32 / span as f32 } else { phase as f32 / span as f32 };
        lerp_rgba(t, second, first)
    }

    /// `0x2faaf8`: the ring's draw (module docs). `text` gives a text id's bytes (none: no name).
    pub fn draw(&mut self, text: &dyn Fn(u32) -> Option<Vec<u8>>, out: &mut Vec<Draw>) {
        let Some(s) = self.screen else { return };
        let r = self.robots;
        let special = r.near_940 as i32 + r.near_a59 as i32 + r.near_93c as i32;
        let near = r.near as i32;
        let fade = (s.fade as f32 / 8.0).clamp(0.0, 1.0);
        let name_fade = (s.name_fade as f32 / 8.0).clamp(0.0, 1.0);
        let sprite = |out: &mut Vec<Draw>, id: u32, sx: f32, frame: f32, colour: u32| out.push(Draw::Sprite { id, pos: AT, scale: [sx, 1.0], frame, colour });
        for id in 8..16 { sprite(out, id, 1.0, 0.0, 0x6044_2d00); }
        for sx in [1.0, -1.0] { sprite(out, 7, sx, 0.0, 0x6044_2d00); }
        for sx in [1.0, -1.0] { sprite(out, 0x86, sx, 0.0, 0x55f0_c070); }
        if 0 <= s.selected {
            let c = self.pulse(0x8044_2d00, 0x80ff_de8d, 0x19);
            let k = if s.selected - 1 < 0 { 7 } else { s.selected - 1 };
            sprite(out, 0x10, 1.0, k as f32, c);
            let b = BARS[s.selected as usize % SLOTS];
            out.push(Draw::Bar { pos: b, size: [33.0, 7.0], colours: [c, c] });
        }
        let mut chosen = false;
        for (i, row) in TABLE.iter().enumerate() {
            let a = crate::hero::physics::add_rot((i as f32 + i as f32) * std::f32::consts::PI / SLOTS as f32 - std::f32::consts::PI, std::f32::consts::FRAC_PI_2);
            let (c, sn) = (a.cos(), a.sin());
            if row.0 == 0 { continue; }
            let mut frame = row.1;
            if frame == 6 && r.bridges == 0 { frame = 7; }
            let sel = i as i32 == s.selected;
            let (mut back, mut lit, mut colour) = (false, false, 0);
            match frame {
                0 | 1 if near != 0 && special < near => (back, lit, colour) = (true, true, if frame == 0 { 0x6000_00ff } else { 0x60ff_0000 }),
                2 | 3 if near != 0 => (back, lit, colour) = (true, true, if frame == 2 { 0x6000_ff00 } else { 0x6029_a1ff }),
                4 | 6 | 7 | 8 | 9 => {
                    let n = match frame { 4 => r.near_93c, 8 => r.near_a59, 9 => r.near, _ => r.near_940 };
                    lit = n != 0;
                }
                _ => {}
            }
            if (back || lit) && sel { chosen = true; }
            if back {
                let pos = [((c * 80.0) as i32 + 0x74) as f32, ((sn * 75.0) as i32 + 0xc2) as f32];
                out.push(Draw::Icon { id: row.0, frame: 10, pos, size: [28.0, 26.0], colour });
            }
            let colour = if lit { 0x80ff_de8d } else { 0x4080_8080 };
            let pos = [((c * 80.0) as i32 + 0x72) as f32, ((sn * 75.0) as i32 + 0xc0) as f32];
            out.push(Draw::Icon { id: row.0, frame, pos, size: [30.0, 28.0], colour });
        }
        if !chosen || !(0..SLOTS as i32).contains(&s.selected) { return; }
        let mut id = NAMES[s.selected as usize];
        if id == NAMES[6] && r.bridges != 0 { id = UNBRIDGE; }
        let Some(t) = text(id) else { return };
        let t = t.split(|&b| b == 0).next().unwrap_or(&[]);
        if t.is_empty() { return; }
        let f = name_fade * fade;
        let line = String::from_utf8_lossy(t).into_owned();
        out.push(Draw::Text { text: line.clone(), pos: [128.0, 200.0], scale: 1.0, colour: lerp_rgba(f, 0, 0x8000_0000), align: Align::Centre });
        out.push(Draw::Text { text: line, pos: [129.0, 201.0], scale: 1.0, colour: lerp_rgba(f, 0x00e0_c0a0, 0x80e0_c0a0), align: Align::Centre });
    }
}

/// `0x371b48`'s part, before the moby loop: with Clank's body and the ring not up the robots' counts start from 0.
pub fn frame_start(body: u8, c: &mut Commands) {
    if body == 1 && !c.is_open() { c.robots = Robots::default(); }
}

/// The body's tick (`0x2a65d8` with Clank's body): the antenna (`0x2ac520`: the signal counted down, and past 0x31 on
/// its odd frames its sound again), then the command word's sound and signal, and the word cleared.
pub fn tick(hero: &mut super::Hero) {
    if hero.mode != 1 { return; }
    let c = &mut hero.body.commands;
    let mut t = c.signal as i32;
    let out = crate::moby::countdown(&mut t);
    c.signal = t as i16;
    if !out && SIGNAL_SOUND < c.signal && c.signal & 1 != 0 { hero.sounds.push((c.signal_command as i32 + 0x10, 0)); }
    let c = &mut hero.body.commands;
    if c.command != 0 {
        let k = c.command;
        hero.sounds.push((k + 0x10, 0));
        let c = &mut hero.body.commands;
        c.signal = SIGNAL;
        c.signal_command = k as i16;
    }
    hero.body.commands.command = 0;
}
