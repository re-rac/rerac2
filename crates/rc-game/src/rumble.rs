//! The pad's rumble: eight wave channels (L00 `0x1b8b80`, 16 bytes each) the objects start with a pattern, mixed
//! each frame into the DualShock's two motors (R&C1's `actuator.c`; the decomp's `0x2d6998` / `0x2d6b70` /
//! `0x2d6ba8`, v1.01).
//!
//! **Start** (`0x2d6d60(pattern)`): a pattern is up to 8 channel records (a first byte 0xff ends it; the disc's are 8
//! records, the unused ones of kind 0). It starts only when there are as many free channels (kind 0) as records — so
//! a disc pattern needs all eight free, and a rumble still running makes the next start fail. Each record goes into
//! the first free channel with its amplitude less its bias, a rise of 1 when both rise and fall are 0. Returns the
//! last channel filled (the first free one after the pattern's real records), or −1. The callers keep it as their
//! "handle" and only test it against 0.
//!
//! **Stop** (`0x2d6f38`, no argument): every *held* channel (`+0x01`, the permanent ones) lets go: held 0, its
//! duration 0, so it runs out on the next mix.
//!
//! **The mix** (`0x2d6f70(out)`, from the frame's actuator update `0x2d73a0`, which the main loop calls in play, the
//! pause and the menus): for each channel in use: not held, its duration counts down and at 0 (before the count) the
//! channel is freed — it still adds this frame's value (0 for kind 0) to its motor's average; a delay counts down
//! first (nothing added); else its frame counts on and its phase is `frame % (rise + fall)`: kind 1 a square (`amp +
//! bias` for the rise, `bias` after), 2 a triangle, 3 a ramp up then 0, 4 0 then a ramp down, 5 the cosine
//! `(cos θ·amp + amp)/2 + bias` (θ from 0 to π over the fall, back over the rise; at most 255). Each motor's values
//! are averaged; a "multiply" channel's average scales the motor's (`out·mul / 255`).
//!
//! **The motors** (`0x2d73a0`, while Vibration is on, `0x1a7bba`): the small motor (0, on or off) is driven by an
//! accumulator (`$gp−0x6714`) of motor 0's level, on for a frame each time it passes 255 (a duty of level / 255);
//! the big motor (1) takes motor 1's level as its byte (`0x138309`).
//!
//! The engine plays [`Rumble::levels`] on the host's gamepad (both motors as levels: a host's motors are not on/off).

/// A channel record (`+0x00` kind, `+0x01` held, `+0x02` motor, `+0x03` multiply, `+0x04` delay, `+0x06` duration,
/// `+0x08` frame, `+0x0a` rise, `+0x0c` fall, `+0x0e` amplitude, `+0x0f` bias).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Channel {
    pub kind: u8,
    pub held: u8,
    pub motor: u8,
    pub multiply: u8,
    pub delay: i16,
    pub duration: i16,
    pub frame: i16,
    pub rise: i16,
    pub fall: i16,
    pub amplitude: u8,
    pub bias: u8,
}

/// A record as the disc has it: kind, held, motor, multiply, delay, duration, frame, rise, fall, amplitude, bias.
#[allow(clippy::too_many_arguments)]
pub const fn rec(kind: u8, held: u8, motor: u8, multiply: u8, delay: i16, duration: i16, frame: i16, rise: i16, fall: i16, amplitude: u8, bias: u8) -> Channel {
    Channel { kind, held, motor, multiply, delay, duration, frame, rise, fall, amplitude, bias }
}

/// A record of kind `kind` (held or not) on `motor` for `duration` frames: rise, fall, amplitude, bias.
#[allow(clippy::too_many_arguments)]
pub const fn ch(kind: u8, held: u8, motor: u8, duration: i16, rise: i16, fall: i16, amplitude: u8, bias: u8) -> Channel {
    rec(kind, held, motor, 0, 0, duration, 0, rise, fall, amplitude, bias)
}

/// An unused record.
pub const NONE: Channel = ch(0, 0, 0, 0, 0, 0, 0, 0);

/// A disc pattern: eight records, the unused ones of kind 0 (none on the disc ends with 0xff).
pub type Pattern = [Channel; 8];

pub const fn two(a: Channel, b: Channel) -> Pattern { [a, b, NONE, NONE, NONE, NONE, NONE, NONE] }
pub const fn one(a: Channel) -> Pattern { [a, NONE, NONE, NONE, NONE, NONE, NONE, NONE] }

/// The weapons' patterns (L00 data; the same records on every level).
pub mod pattern {
    use super::{ch, one, two, Pattern};
    /// `0x2947c0`: the Pulse Rifle's shot: both motors full for 7 frames.
    pub const PULSE_RIFLE: Pattern = two(ch(1, 0, 1, 7, 0, 0, 255, 0), ch(1, 0, 0, 7, 0, 0, 255, 0));
    /// `0x294840`: the Minirocket Tube's rocket: both at 255 (bias 140) for 5 frames.
    pub const MINIROCKET: Pattern = two(ch(1, 0, 1, 5, 0, 0, 255, 140), ch(1, 0, 0, 5, 0, 0, 255, 140));
    /// `0x2948c0`: the Blitz Gun's blast: both at 210 for 10 frames.
    pub const BLITZ: Pattern = two(ch(1, 0, 1, 10, 0, 0, 210, 0), ch(1, 0, 0, 10, 0, 0, 210, 0));
    /// `0x294940`: the Lancer's shot (class 0x5d2): the small motor's cosine 120..180, 8 frames.
    pub const LANCER: Pattern = one(ch(5, 0, 0, 8, 4, 4, 180, 120));
    /// `0x2949c0`: the Heavy Lancer's held fire: the small motor's cosine 120..240, held.
    pub const HEAVY_LANCER: Pattern = one(ch(5, 1, 0, 8, 4, 4, 240, 120));
    /// `0x295fe0`: the Mini-Nuke's throw.
    pub const MINI_NUKE: Pattern = two(ch(1, 0, 1, 7, 0, 0, 255, 140), ch(1, 0, 0, 7, 0, 0, 255, 140));
    /// `0x296060`: the RYNO II's volley.
    pub const RYNO: Pattern = two(ch(1, 0, 1, 7, 0, 0, 255, 140), ch(1, 0, 0, 7, 0, 0, 255, 140));
    /// `0x2960e0`: the upgraded Blitz Gun's blast.
    pub const BLITZ_MEGA: Pattern = two(ch(1, 0, 1, 10, 0, 0, 255, 0), ch(1, 0, 0, 10, 0, 0, 255, 0));
    /// `0x296a60`: the Vaporizer's shot.
    pub const VAPORIZER: Pattern = two(ch(1, 0, 1, 7, 0, 0, 255, 0), ch(1, 0, 0, 7, 0, 0, 255, 0));
    /// `0x29ae30`: the Gravity Bomb's throw.
    pub const GRAVITY_BOMB: Pattern = two(ch(1, 0, 1, 5, 0, 0, 255, 140), ch(1, 0, 0, 5, 0, 0, 255, 140));
    /// `0x29f1d0`: the Bouncer's throw.
    pub const BOUNCER: Pattern = two(ch(1, 0, 1, 7, 0, 0, 255, 140), ch(1, 0, 0, 7, 0, 0, 255, 140));
    /// `0x29fba0`: the Zodiac's charge: the big motor's ramp from 80 to 255 over 40 frames.
    pub const ZODIAC: Pattern = one(ch(3, 0, 1, 40, 40, 0, 255, 80));
    /// `0x29fc88`: the Megarocket Cannon's volley.
    pub const MEGAROCKET: Pattern = two(ch(1, 0, 1, 10, 0, 0, 255, 0), ch(1, 0, 0, 10, 0, 0, 255, 0));
}

/// The channels and the motors (GameState::rumble).
#[derive(Clone, Debug, Default)]
pub struct Rumble {
    /// `0x1b8b80`: the eight channels.
    pub channels: [Channel; 8],
    /// `$gp−0x6714`: the small motor's accumulator.
    pub acc: i32,
    /// `0x138308` / `0x138309`: the actuator bytes (small on, big level).
    pub small: bool,
    pub big: u8,
    /// The mix's levels of this frame (motor 0, motor 1), for the host's pad; 0 with Vibration off.
    pub out: [i32; 2],
    /// The mixes run (the engine stops the host's rumble when no mix ran since it last looked).
    pub frames: u32,
}

impl Rumble {
    /// `0x2d6d60(pattern)` (module docs): the last channel filled, or −1.
    pub fn start(&mut self, pattern: &[Channel]) -> i32 {
        let n = pattern.iter().take(8).take_while(|r| r.kind != 0xff).count();
        let free = self.channels.iter().filter(|c| c.kind == 0).count().min(n);
        if free != n { return -1; }
        let mut i = 0;
        for r in &pattern[..n] {
            i = self.channels.iter().position(|c| c.kind == 0).unwrap_or(8);
            if let Some(c) = self.channels.get_mut(i) {
                *c = Channel { amplitude: r.amplitude.wrapping_sub(r.bias), ..*r };
                if c.rise.wrapping_add(c.fall) == 0 { c.rise = c.rise.wrapping_add(1); }
            }
        }
        i as i32
    }

    /// `0x2d6d40(i)`: channel `i` in use.
    pub fn running(&self, i: i32) -> bool { usize::try_from(i).ok().and_then(|i| self.channels.get(i)).is_some_and(|c| c.kind != 0) }

    /// `0x2d6f38`: every held channel lets go (module docs).
    pub fn stop_held(&mut self) {
        for c in self.channels.iter_mut().filter(|c| c.held != 0) {
            c.held = 0;
            c.duration = 0;
        }
    }

    /// `0x2d6f70`: the channels' mix (module docs); returns the motors' levels.
    fn mix(&mut self) -> [i32; 2] {
        let mut out = [0i32; 2];
        let (mut mul, mut count, mut mul_count) = ([0i32; 2], [0i32; 2], [0i32; 2]);
        for s in self.channels.iter_mut() {
            if s.kind == 0 { continue; }
            if s.held == 0 {
                let d = s.duration;
                s.duration = d.wrapping_sub(1);
                if d <= 0 { s.kind = 0; }
            }
            if s.delay != 0 {
                s.delay = s.delay.wrapping_sub(1);
                continue;
            }
            s.frame = s.frame.wrapping_add(1);
            let (rise, fall, amp, bias) = (s.rise as i32, s.fall as i32, s.amplitude as i32, s.bias as i32);
            let phase = (s.frame as i32).checked_rem(rise + fall).unwrap_or(0);
            let div = |a: i32, b: i32| a.checked_div(b).unwrap_or(0);
            let value = match s.kind {
                1 => if phase < rise { amp + bias } else { bias },
                2 => (if phase < rise { div(amp * phase, rise) } else { div(amp * (fall + rise - phase), fall) }) + bias,
                3 => if phase < rise { div(amp * phase, rise) + bias } else { 0 },
                4 => if phase >= rise { div(amp * (fall + rise - phase), fall) + bias } else { 0 },
                5 => {
                    let pi = std::f32::consts::PI;
                    let angle = if phase < fall { phase as f32 * pi / fall as f32 } else { pi - (phase - fall) as f32 * pi / rise as f32 };
                    let a = amp as f32;
                    (((angle.cos() * a + a) * 0.5) as i32 + bias).min(255)
                }
                _ => 0,
            };
            let m = (s.motor & 1) as usize;
            if s.motor > 1 { continue; }
            if s.multiply != 0 {
                mul[m] += value;
                mul_count[m] += 1;
            } else {
                out[m] += value;
                count[m] += 1;
            }
        }
        for j in 0..2 {
            if count[j] != 0 { out[j] /= count[j]; }
            if mul_count[j] != 0 {
                mul[j] /= mul_count[j];
                out[j] = out[j] * mul[j] / 255;
            }
        }
        out
    }

    /// `0x2d73a0`: a frame's mix into the actuator bytes, while `vibration` (the option `0x1a7bba`) is on.
    pub fn tick(&mut self, vibration: bool) {
        let out = self.mix();
        self.frames = self.frames.wrapping_add(1);
        if vibration {
            self.acc += out[0];
            self.small = 0xff < self.acc;
            if self.small { self.acc -= 0xff; }
            self.big = out[1] as u8;
            self.out = out;
        } else {
            self.out = [0; 2];
        }
    }

    /// The motors' levels for the host's pad, each 0..=1 (small, big); 0 with Vibration off.
    pub fn levels(&self) -> [f32; 2] { self.out.map(|v| v.clamp(0, 255) as f32 / 255.0) }
}

/// The weapons' stop of their rumble (their ready state's tail, `0x2d6f38`): unless the trigger is held
/// (`0x138320 & 0x18b050`) with ammo (`0x2d43e0`; `ammo` false: not tested, the RYNO II's) and the use on
/// (`0x18c0d8`), a handle at the hand's `+h` (≥ 0) stops the held channels and becomes −1. Returns whether it did.
pub fn weapon_stop(w: &mut crate::moby::MobyWorld, id: usize, ctx: &mut crate::moby::Ctx, h: usize, ammo: bool) -> bool {
    let it = ctx.hero.items;
    if it.held & it.trigger != 0 && (!ammo || it.has_ammo(ctx.game)) && it.using { return false; }
    if w.mobys[id].pv_i(h) < 0 { return false; }
    ctx.game.rumble.stop_held();
    w.mobys[id].set_pv_i(h, -1);
    true
}
