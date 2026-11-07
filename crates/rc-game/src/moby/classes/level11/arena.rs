//! Joba's arena (the level's chunk 1; L11 addresses): crate::moby::classes::level11's gap G-LVL-009. Maktar's arena
//! (crate::moby::classes::arena) is the model; Joba's master is its own code, which shares with Maktar's the spawner's
//! helpers (`0x418968` / `0x4189a8` / `0x4189e0` / `0x418a18` / `0x418a58` = L02 `0x3d8d20..`), the prize and XP sums
//! (`0x4441a8` / `0x4442b8`), the plates 0x0a03, the flame columns 0x0b89, the crate lifts 0x0ccc and the crates' put
//! away (`0x3b3748`), and the wipe's draw (`0x447550` = Maktar's `0x401a20`, crate::moby::classes::arena::draw_wipe).
//!
//! * **The master 0x0d8a** (`0x43f9e0`, [`update_master`]). Its pvars ([`pv`]): the groups `+0xc0` (the fighters and
//!   bosses, pooled above the arena), `+0xc4` (the spawners 0x0c7f), `+0xc8` (the flame columns), `+0xcc` (the floor
//!   plates), `+0x178` / `+0x174` (two crate groups) and `+0x17c` (their lifts); the desk's cuboids `+0x150` (and
//!   `+0x154` once the second challenge is won), the way back `+0x158` / `+0x15c`, the arena's cylinder `+0x1e4` and
//!   the pit's `+0x160`, the middle `+0x168` and the entry `+0x16c` (cuboids); the door moby `+0x1b8` (the shrinking
//!   ring of state 9, [`ring_step`]) with its polygon (paths `+0x1c8`, its rest shape `+0x1cc`), the platform tie group
//!   `+0x1c0` and the lamp moby `+0x1d0` raised with it, the scene triggers `+0x1d4` (the welcome) / `+0x1d8` / `+0x1dc`
//!   (the first wins of challenges 0 and 1); the announcer's 39 lines `+0x210..`; the music `+0x2f0..+0x304`.
//!   * **0** the set-up, **1** the desk (the prompt "Enter the Battle Arena", △ → **2**: the fade, chunk 1, Ratchet at
//!     the entry, the walk to the middle; the first time the welcome scene; then the first challenge until it is won,
//!     else the overview).
//!   * **4 / 5** (5 after a win: its reward, [`reward`]) the overview camera → **7** the menu ([`menu`]) or **6** the
//!     win's screen.
//!   * **8** a challenge's waves from the spawners ([`fight`]); **9** its waves of the ring and the platform
//!     ([`ring_fight`]); **11** lost; **12 / 13** the first wins of challenges 0 / 1 (their scenes).
//!
//!   The score ([`score`]) is the crowd's mood (0x126e, [`update_crowd`]).
//! * **The spawners 0x0c7f** (`0x4184e0`, [`update_spawner`]): Maktar's 0x0a15 code
//!   (crate::moby::classes::arena::spawner_step) with their own release ([`release`], `0x418ab0`).
//! * **The crowd 0x126e** ([`update_crowd`]).
//!
//! **The fighters** 0x0d98 / 0x0dcf / 0x0df4 / 0x0df7 / 0x0df9 / 0x0d37 and the bosses 0x0d4a / 0x0b3a are
//! crate::moby::classes::level11::fighters, ::brutes and ::bosses; the master and the spawners call into them through
//! [`start`], [`start_boss`], [`pool`] and [`die_out`].
//!
//! Not ported here (each a leftover of G-LVL-009):
//! * the menu's own draw (`0x4454e8`, asm; Maktar's crate::moby::classes::arena::draw_menu stands in with Joba's names
//!   [`challenge_name`]), the challenge's HUD (`0x444400`, asm: not published), the master's draw (`0x448020`, asm),
//!   the guide marker (`0x443c48`, `0x2d70a0(5)`), the platform's sparks and lights (`0x448a40`, `0x4486a0`,
//!   `0x4488e0`, the pvars `+0x90..+0xbc`), the ring's flashes (`+0xd0..+0xdc`), the falling drops of a wave's flag
//!   `+0x18` (`0x448bd0` → `0x461850`);
//! * the gravity's cylinder mode (`0x1a8ff0` / `0x1a8ff4` set above 480 in state 9: kept as
//!   [`super::Globals::arena_gravity`], crate::gravity has no cylinder mode) and what it resets (`0x448d88`), the
//!   launch's strength `0x1a7cd8` (80 / 5, [`super::Globals::arena_launch`]), the fall word `0x18a010` = 1 in the band
//!   460..463, Ratchet's pickup radius `0x18c2c4` (300 / 200 after a challenge; crate::moby::classes::bolt's is
//!   constant), the Zapper's word `0x18b6f4` kept off in challenges 7 / 8, the no-△ byte `0x18c0d7` in 7;
//! * the boss HUD's owner (`0x1ba780 + 0x284`) and its records (`0x1b3770`, `0x1ffa78`), the restart's path straight
//!   into the wipe (`0x1a7904` = 2), the speech's cut-off (`0x4477d8`), the debug win (L1 + L2 + R1 + R2 held with
//!   `0x200864`), the upgraded glove's end (`0x460f68` = `0x42e670`).

use crate::hero::physics::{add_rot, approach, atan2, dist, dist2, set_len, sub, V3};
use crate::moby::{ai, Ctx, MobyWorld};
use super::super::arena as maktar;

/// The arena's master, its spawners and its crowd.
pub const MASTER: i16 = 0x0d8a;
pub const SPAWNER: i16 = 0x0c7f;
pub const CROWD: i16 = 0x126e;

/// The fighters, in the order of a wave's counts (`h[0..6]`), and the bosses.
pub const D98: i16 = 0x0d98;
pub const DCF: i16 = 0x0dcf;
pub const DF4: i16 = 0x0df4;
pub const DF7: i16 = 0x0df7;
pub const DF9: i16 = 0x0df9;
pub const D37: i16 = 0x0d37;
pub const D4A: i16 = 0x0d4a;
pub const D4B: i16 = 0x0d4b;
pub const B3A: i16 = 0x0b3a;
pub const FIGHTERS: [i16; 6] = [D98, DCF, DF4, DF7, DF9, D37];

/// The master's pvars (the module's docs).
pub mod pv {
    /// The score's ring of four windows (0x10 each: kills, damage, health lost).
    pub const HISTORY: usize = 0x50;
    pub const ENEMIES: usize = 0xc0;
    pub const SPAWNERS: usize = 0xc4;
    pub const COLUMNS: usize = 0xc8;
    pub const PLATES: usize = 0xcc;
    /// The turns to attack left per fighter class (`$gp+0x24a0` at a challenge's start).
    pub const LIMITS: usize = 0xe0;
    /// The fighters sent this wave per class (`+0x100..+0x114`), the bosses `+0x118` (0x0d4a) / `+0x11c` (0x0b3a).
    pub const SENT: usize = 0x100;
    pub const SENT_D4A: usize = 0x118;
    pub const SENT_B3A: usize = 0x11c;
    /// The round's spawners (links) and the current one's index `+0x18c`.
    pub const SLOTS: usize = 0x120;
    pub const SLOT: usize = 0x18c;
    pub const ALIVE: usize = 0x140;
    pub const LIFE: usize = 0x144;
    pub const HEALTH: usize = 0x148;
    pub const SCORE: usize = 0x14c;
    pub const DESK: usize = 0x150;
    pub const DESK_WON: usize = 0x154;
    pub const BACK: usize = 0x158;
    pub const BACK_WON: usize = 0x15c;
    pub const PIT: usize = 0x160;
    pub const MIDDLE: usize = 0x168;
    pub const ENTRY: usize = 0x16c;
    pub const CRATES_B: usize = 0x174;
    pub const CRATES_A: usize = 0x178;
    pub const LIFTS: usize = 0x17c;
    /// The wave's boss (a link), its owner word `+0x184`.
    pub const BOSS: usize = 0x180;
    pub const OWNER: usize = 0x184;
    pub const FRAMES: usize = 0x188;
    /// The remarks: the wait `+0x190`, the damage and kills summed `+0x194` / `+0x198`, the sums' restart `+0x1b0`.
    pub const REMARK_WAIT: usize = 0x190;
    pub const REMARK_SUM: usize = 0x194;
    pub const REMARK_KILLS: usize = 0x198;
    pub const REMARK_RESTART: usize = 0x1b0;
    /// The way round the middle the fighters of state 9 come in at.
    pub const HEADING: usize = 0x19c;
    pub const DOOR: usize = 0x1b8;
    pub const SCALE: usize = 0x1bc;
    pub const PLATFORM: usize = 0x1c0;
    pub const RING_PATH: usize = 0x1c8;
    pub const RING_REST: usize = 0x1cc;
    pub const LAMP: usize = 0x1d0;
    pub const INTRO: usize = 0x1d4;
    pub const WIN_0: usize = 0x1d8;
    pub const WIN_1: usize = 0x1dc;
    pub const PRIZE: usize = 0x1e0;
    pub const ZONE: usize = 0x1e4;
    pub const SELECTED: usize = 0x1e8;
    pub const REPEAT: usize = 0x1ea;
    pub const RING: usize = 0x1ec;
    pub const WINDOW: usize = 0x1ee;
    pub const CHALLENGE: usize = 0x1f0;
    pub const WAVE: usize = 0x1f2;
    pub const WIPE: usize = 0x1f4;
    pub const ARROW_LEFT: usize = 0x1f6;
    pub const ARROW_RIGHT: usize = 0x1f8;
    pub const RING_MODE: usize = 0x1fa;
    pub const D37S: usize = 0x200;
    pub const LINES: usize = 0x210;
    pub const SAID: usize = 0x2a0;
    pub const MUSIC_ARENA: usize = 0x2f0;
    pub const MUSIC_RESORT: usize = 0x2f4;
    pub const STINGER_ARENA: usize = 0x2f8;
    pub const STINGER_RESORT: usize = 0x2fc;
    pub const MUSIC_BOSS: usize = 0x300;
    pub const STINGER_BOSS: usize = 0x304;
}

// ---------------------------------------------------------------------------------------------------------------
// The challenges

/// One wave (0x1a bytes): how many of each fighter ([`FIGHTERS`]), the plates and columns up, the crates lifted (the
/// groups `+0x178` then `+0x174`); its flags: 0x0d4a / 0x0b3a sent, the spawners it uses, a wave of the ring (state
/// 9), the falling drops.
#[derive(Clone, Copy, Debug)]
pub struct Wave {
    pub fighters: [i16; 6],
    pub plates: i16,
    pub columns: i16,
    pub crates: [i16; 2],
    pub d4a: bool,
    pub b3a: bool,
    pub spawners: u8,
    pub ring: bool,
    pub drops: bool,
}

/// One challenge (`$gp+0x24e8[n]`, 0x18 bytes as Maktar's: `{i32 waves, u32 → waves, i32 needs, i32 bolts, i32 xp,
/// 0}`).
#[derive(Clone, Debug)]
pub struct Challenge {
    pub needs: i32,
    pub bolts: i32,
    pub xp: i32,
    pub waves: Vec<Wave>,
}

/// `$gp+0x24e8`: the challenges' table, 13 pointers.
pub const CHALLENGE_TABLE: u32 = 0x1aef_f0 + 0x24e8;
pub const CHALLENGE_COUNT: usize = 13;

/// The challenges as the level's overlay holds them (crate::level::LevelData::joba; `read(address, length)`).
pub fn read_challenges(read: &dyn Fn(u32, usize) -> Option<Vec<u8>>) -> Option<Vec<Challenge>> {
    let word = |b: &[u8], o: usize| i32::from_le_bytes(b[o..o + 4].try_into().unwrap());
    let half = |b: &[u8], o: usize| i16::from_le_bytes(b[o..o + 2].try_into().unwrap());
    let table = read(CHALLENGE_TABLE, 4 * CHALLENGE_COUNT)?;
    (0..CHALLENGE_COUNT).map(|n| {
        let rec = read(word(&table, 4 * n) as u32, 0x18)?;
        let count = usize::try_from(word(&rec, 0)).ok().filter(|&c| c <= 0x40)?;
        let waves = read(word(&rec, 4) as u32, 0x1a * count)?;
        let waves = (0..count).map(|k| {
            let w = &waves[0x1a * k..0x1a * (k + 1)];
            Wave {
                fighters: std::array::from_fn(|i| half(w, 2 * i)),
                plates: half(w, 0xc),
                columns: half(w, 0xe),
                crates: [half(w, 0x10), half(w, 0x12)],
                d4a: w[0x14] != 0,
                b3a: w[0x15] != 0,
                spawners: w[0x16],
                ring: w[0x17] != 0,
                drops: w[0x18] != 0,
            }
        }).collect();
        Some(Challenge { needs: word(&rec, 8), bolts: word(&rec, 0xc), xp: word(&rec, 0x10), waves })
    }).collect()
}

/// `0x43f948(n)`: challenge `n`'s win count in the flags (`0x1395b8 + k`).
pub fn win_flag(n: i32) -> Option<usize> {
    Some(match n {
        0 => 61, 1 => 62, 2 => 63, 3 => 64, 4 => 65, 5 => 66, 6 => 67, 7 => 68, 9 => 69, 10 => 70, 8 => 71, 11 => 72, 12 => 75,
        _ => return None,
    })
}

/// `0x445548(n)`: challenge `n`'s name.
pub fn challenge_name(n: i32) -> u32 {
    [0x2dfc, 0x2dfd, 0x2dfe, 0x2dff, 0x2e00, 0x2e01, 0x2e02, 0x2e03, 0x2e06, 0x2e07, 0x30d9, 0x30da, 0x30db].get(n as usize).copied().unwrap_or(0x2dfc)
}

/// `0x43f8d0(n)`: the challenge's arena drop index (crate::game::GameState::arena_drop, `0x1aa170` here).
fn drop_index(n: i32) -> i32 { match n { 0 => 10, 1 => 11, 3 => 12, 4 => 13, 5 => 14, 7 => 15, 10 => 16, 8 => 17, 12 => 18, _ => -1 } }

/// The flags: the selection kept `0x1395b8 + 0x49`, the welcome seen `+0x4a`.
const FLAG_SELECTED: usize = 0x49;
const FLAG_INTRO: usize = 0x4a;

fn wins(game: &crate::game::GameState, n: i32) -> u8 { win_flag(n).and_then(|k| game.flags.get(k).copied()).unwrap_or(0) }

/// The item challenge 8 needs (`0x139586`'s row), and the hand items of 7 / 8 (`0x334f80`).
const ITEM_8: i32 = 0x1e;
const HAND_7: i32 = 10;
const HAND_8: i32 = 0x1e;

/// Challenge `n` on the menu (`0x444ce8`'s test): the challenge it needs won, from 11 the challenge mode, 8 its item
/// had (`0x264a74`'s row of the item's `+4`: here the items had, as Maktar's).
fn on_menu(cs: &[Challenge], game: &crate::game::GameState, n: i32) -> bool {
    let Some(c) = cs.get(n as usize) else { return false };
    if wins(game, c.needs) == 0 { return false; }
    if 11 <= n && !game.challenge { return false; }
    if n == 8 && !game.items.get(ITEM_8 as usize).is_some_and(|&b| b != 0) { return false; }
    true
}

fn wave_of(w: &MobyWorld, id: usize, cs: &[Challenge]) -> Option<Wave> {
    let m = &w.mobys[id];
    cs.get(m.pv_s(pv::CHALLENGE) as usize).and_then(|c| c.waves.get(m.pv_s(pv::WAVE) as usize)).copied()
}

fn waves(cs: &[Challenge], ch: i16) -> i16 { cs.get(ch as usize).map_or(0, |c| c.waves.len() as i16) }

// ---------------------------------------------------------------------------------------------------------------
// The fighters' entry points (crate::moby::classes::level11::fighters, ::brutes, ::bosses)

/// The floor's plates and flame columns (Maktar's classes, crate::moby::classes::arena).
pub const PLATE_CLASS: i16 = 0x0a03;
pub const COLUMN_CLASS: i16 = 0x0b89;

/// The fighters' starts `(m, delay, placed)` — 0x0d98 `0x44b0c8`, 0x0dcf `0x44dd60`, 0x0df4 `0x4546b0`, 0x0df7
/// `0x4566d0`, 0x0d37 `0x438150`, 0x0df9 `0x457e90` (out of a spawner only).
fn start(w: &mut MobyWorld, f: usize, ctx: &mut Ctx, delay: i32, placed: bool) { super::fighters::start(w, f, ctx, delay, placed) }

/// The bosses' starts: 0x0d4a `0x43b190`, 0x0b3a `0x3f4448`.
fn start_boss(w: &mut MobyWorld, f: usize, ctx: &mut Ctx) {
    match w.mobys[f].o_class {
        D4A => super::bosses::start_d4a(w, f, ctx),
        B3A => super::bosses::start_b3a(w, f, ctx),
        _ => {}
    }
}

/// The pools (`0x442a18`'s calls): 0x0d98 `0x44b2a0`, 0x0dcf `0x44df28`, 0x0df4 `0x454850`, 0x0df7 `0x456830`, 0x0df9
/// `0x457f98`, 0x0d37 `0x4382b8`, 0x0d4a `0x43b430`, 0x0d4b `0x43c368`, 0x0b3a `0x3f45a8`.
fn pool(w: &mut MobyWorld, f: usize, ctx: &mut Ctx) { super::fighters::pool(w, f, ctx) }

/// A fighter out of the arena (`0x4439c0`): 0x0d98 its death `0x44b398(m, 0, 0)`; 0x0dcf / 0x0df4 / 0x0df7 / 0x0d37
/// its bolts (`0x443288(m, 0, −1)` = Maktar's `0x3fd7e8`: `0x31ee88(m, 0, 3)`) and its pool; 0x0df9 its bolts and the
/// death fade into its pool (`0x4148d0` = crate::moby::classes::death_fade::start); 0x0d4b its pool when `0x43c040`
/// holds (past its state 3).
fn die_out(w: &mut MobyWorld, f: usize, ctx: &mut Ctx) {
    match w.mobys[f].o_class {
        D98 => super::brutes::die_d98(w, f, ctx, false, false),
        DCF | DF4 | DF7 | D37 => {
            super::fighters::bolts(w, f, ctx);
            pool(w, f, ctx);
        }
        DF9 => {
            super::fighters::bolts(w, f, ctx);
            crate::moby::classes::death_fade::start(w, f, ctx, 0.0, Some(super::brutes::pool_df9));
        }
        D4B => {
            if 4 <= w.mobys[f].state { pool(w, f, ctx); }
        }
        _ => {}
    }
}

/// The fighters' turn words (`0x442ba0`).
fn turn_word(c: i16) -> Option<usize> { match c { D98 => Some(0x366), DCF => Some(0x4ca), DF9 => Some(0x256), D37 => Some(0x282), _ => None } }

fn limit_slot(c: i16) -> Option<usize> { FIGHTERS.iter().position(|&x| x == c) }

// ---------------------------------------------------------------------------------------------------------------
// The master's helpers

fn group(w: &MobyWorld, id: usize, off: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].pv_i(off)).ok().filter(|&g| g != 0xff).and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// A moby the master's pvars name (the loader made them runtime indices).
fn placed(w: &MobyWorld, i: i32) -> Option<usize> { usize::try_from(i).ok().filter(|&i| w.mobys.get(i).is_some_and(|m| m.state < 0x80)) }

/// `0x443038(master)`: the arena's middle, the place of the master's cuboid `+0x168`.
pub fn middle(w: &MobyWorld, master: usize, ctx: &Ctx) -> Option<V3> { ctx.level.cuboid_place(w.mobys[master].pv_i(pv::MIDDLE)).map(|c| c.0) }

/// `$gp+0x2560`: the angles Ratchet is put at the middle with.
const MIDDLE_ANGLES: V3 = [0.0, 0.0, std::f32::consts::FRAC_PI_2];

/// `0x447750(m, n)`: line `n` of the announcer (`+0x210 + 4n`, the low 14 bits, + 2000) with the dialogue free, marked
/// said (`+0x2a0 + 2n`).
fn say(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, n: i32) -> bool {
    if !(0..0x27).contains(&n) { return false; }
    let line = (w.mobys[id].pv_i(pv::LINES + 4 * n as usize) & 0x3fff) + 2000;
    if !ctx.host.request_speech(line) { return false; }
    w.mobys[id].set_pv_s(pv::SAID + 2 * n as usize, 1);
    true
}

fn said(w: &MobyWorld, id: usize, n: i32) -> bool { w.mobys[id].pv_s(pv::SAID + 2 * n as usize) != 0 }

/// `0x441a10`: the challenge and wave none, the selection 0, the round's spawners cleared, the boss none, state 1,
/// the ring's scale 1, no guide.
fn reset(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_s(pv::CHALLENGE, -1);
    m.set_pv_s(pv::SELECTED, 0);
    m.set_pv_s(pv::WAVE, -1);
    for k in 0..6 { m.set_pv_i(pv::SLOTS + 4 * k, 0); }
    m.set_pv_i(pv::SLOT, 0);
    m.set_pv_i(pv::BOSS, 0);
    m.set_state(1, -1);
    m.set_pv_f(pv::SCALE, 1.0);
}

/// The door moby `+0x1b8` frozen and hidden (`|= 1`, `+0x31` = 0).
fn hide_door(w: &mut MobyWorld, id: usize) {
    if let Some(d) = placed(w, w.mobys[id].pv_i(pv::DOOR)) {
        w.mobys[d].mode |= 1;
        w.mobys[d].drawn = 0;
    }
}

/// The platform (the first tie of the group `+0x1c0`) and the lamp moby `+0x1d0` at height `z`; their height.
fn platform(w: &MobyWorld, id: usize) -> Option<usize> {
    usize::try_from(w.mobys[id].pv_i(pv::PLATFORM)).ok().and_then(|g| w.tie_groups.get(g)).and_then(|l| l.first()).map(|&t| (t & 0x7fff) as usize).filter(|&t| t < w.ties.len())
}

fn platform_z(w: &MobyWorld, id: usize) -> f32 { platform(w, id).map_or(PLATFORM_LOW, |t| w.ties[t][3][2]) }

fn set_platform_z(w: &mut MobyWorld, id: usize, z: f32) {
    if let Some(t) = platform(w, id) {
        w.ties[t][3][2] = z;
        if !w.moved_ties.contains(&t) { w.moved_ties.push(t); }
    }
    if let Some(l) = placed(w, w.mobys[id].pv_i(pv::LAMP)) { w.mobys[l].pos[2] = z; }
}

/// The platform's rest and top heights and its climb a tick.
const PLATFORM_LOW: f32 = 419.766;
const PLATFORM_HIGH: f32 = 421.266;
const PLATFORM_STEP: f32 = 0.008_333_334;

/// `0x4432a8` / `0x443558(m, n)`: `n` of the columns (`+0xc8`) / plates (`+0xcc`) up (the first ones down raised, or
/// the first ones up lowered).
fn raise_group(w: &mut MobyWorld, id: usize, off: usize, n: i32) {
    let g = group(w, id, off);
    let up = |w: &MobyWorld, p: usize| if off == pv::PLATES { maktar::raised(w, p) } else { maktar::column_on(w, p) };
    let set = |w: &mut MobyWorld, p: usize, on: bool| if off == pv::PLATES { maktar::raise(w, p, on) } else { maktar::column_set(w, p, on) };
    let mut count = g.iter().filter(|&&p| up(w, p)).count() as i32;
    if count < n {
        for &p in &g {
            if !up(w, p) {
                set(w, p, true);
                count += 1;
                if n <= count { return; }
            }
        }
    } else if n < count {
        for &p in &g {
            if up(w, p) {
                set(w, p, false);
                count -= 1;
                if count <= n { return; }
            }
        }
    }
}

fn floor_down(w: &mut MobyWorld, id: usize) {
    raise_group(w, id, pv::COLUMNS, 0);
    raise_group(w, id, pv::PLATES, 0);
}

fn floor_for(w: &mut MobyWorld, id: usize, wave: &Wave) {
    raise_group(w, id, pv::COLUMNS, wave.columns as i32);
    raise_group(w, id, pv::PLATES, wave.plates as i32);
}

/// `0x4444d0`: the crates of both groups (`+0x178`, `+0x174`) put away (`0x3b3748`).
fn put_crates_away(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for c in [group(w, id, pv::CRATES_A), group(w, id, pv::CRATES_B)].concat() {
        if w.mobys[c].state < 0x80 { super::super::crate_::put_away(w, c, ctx); }
    }
}

/// `$gp+0x2604` / `+0x2608`: the lifts' rise (5.3 a second) and spin (720° a second).
const LIFT_RISE: f32 = 5.3;
const LIFT_SPIN: f32 = 720.0;

/// `0x444688`: the wave's crates lifted — for each group (`+0x178` then `+0x174`) its broken ones (state 6, at most
/// 16) gathered, the others counted as out; while fewer are out than the wave asks for (at most 1000 passes over the
/// lifts `+0x17c`) the last gathered put on a free lift (one in six passed over).
fn lift_crates(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, wave: &Wave) {
    for (want, from) in [(wave.crates[0], pv::CRATES_A), (wave.crates[1], pv::CRATES_B)] {
        if want == 0 { continue; }
        let crates = group(w, id, from);
        let mut buf: Vec<usize> = Vec::new();
        let mut out = 0;
        for &c in &crates {
            if w.mobys[c].state == 6 && buf.len() < 16 { buf.push(c); } else { out += 1; }
        }
        if buf.is_empty() { continue; }
        let lifts = group(w, id, pv::LIFTS);
        let mut i = 0;
        while i < 1000 && out < want as i32 {
            for &l in &lifts {
                let Some(&c) = buf.last() else { break };
                if maktar::lift_free(w, l) && ctx.game.rng.randi(6) == 0 {
                    buf.pop();
                    maktar::lift(w, l, c, LIFT_RISE / 60.0, LIFT_SPIN.to_radians() / 60.0);
                    out += 1;
                    break;
                }
            }
            i += 1;
        }
    }
}

/// `0x442a18`: every fighter and boss back to its pool ([`pool`]).
fn pool_all(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for f in group(w, id, pv::ENEMIES) {
        if matches!(w.mobys[f].o_class, D98 | DCF | DF4 | DF7 | DF9 | D37 | D4A | D4B | B3A) { pool(w, f, ctx); }
    }
}

fn is_enemy(c: i16) -> bool { matches!(c, D98 | DCF | DF4 | DF7 | DF9 | D37 | D4A | B3A) }

/// `0x441a88`: the fighters and bosses out (not pooled) and their health, Ratchet's health.
fn count_out(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let (mut n, mut life) = (0, 0.0);
    for f in group(w, id, pv::ENEMIES) {
        let x = &w.mobys[f];
        if x.state >= 0x80 || x.state == 1 || !is_enemy(x.o_class) { continue; }
        n += 1;
        life += x.pv_f(0x20);
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::ALIVE, n);
    m.set_pv_f(pv::LIFE, life);
    m.set_pv_f(pv::HEALTH, ctx.hero.hp as f32);
}

/// `$gp+0x28cc..+0x28dc` (`0x447820`): the remarks' damage and kills to pass (55, 5), the hurt (3), the low health (a
/// quarter), the wait after one (240 + up to 240 ticks); the sums restart every 300 ticks.
const REMARK_DAMAGE: f32 = 55.0;
const REMARK_KILLS: i32 = 5;
const REMARK_HURT: f32 = 3.0;
const REMARK_LOW: f32 = 0.25;
const REMARK_WAIT: i32 = 240;
const REMARK_RESTART: i32 = 300;

/// `0x447820(m, kills, damage, hurt)`: the announcer's remarks, with the dialogue free and the last one's wait over —
/// the damage and kills summed: past 55 or 5 its lines 0x14..0x17 not said (and the sums restarted); hurt more than 3
/// its lines 0x12 / 0x13; under a quarter of his health 0x18; down 0x19 or 0x1a. One of them at random.
fn remark(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, kills: i32, damage: f32, hurt: f32) {
    if !ctx.host.speech_free() || w.mobys[id].pv_i(pv::REMARK_WAIT) != 0 { return; }
    let m = &mut w.mobys[id];
    let sum = m.pv_f(pv::REMARK_SUM) + damage;
    let k = m.pv_i(pv::REMARK_KILLS) + kills;
    m.set_pv_i(pv::REMARK_KILLS, k);
    m.set_pv_f(pv::REMARK_SUM, sum);
    let mut lines = Vec::new();
    if REMARK_DAMAGE < sum || REMARK_KILLS < k {
        lines.extend((0x14..0x18).filter(|&n| !said(w, id, n)));
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::REMARK_KILLS, 0);
        m.set_pv_f(pv::REMARK_SUM, 0.0);
    }
    let m = &mut w.mobys[id];
    let mut t = m.pv_i(pv::REMARK_RESTART);
    if crate::moby::countdown(&mut t) {
        m.set_pv_i(pv::REMARK_KILLS, 0);
        t = REMARK_RESTART;
        m.set_pv_f(pv::REMARK_SUM, 0.0);
    }
    m.set_pv_i(pv::REMARK_RESTART, t);
    if REMARK_HURT < hurt { lines.extend((0x12..0x14).filter(|&n| !said(w, id, n))); }
    if (ctx.hero.hp as f32) < ctx.game.max_health as f32 * REMARK_LOW && !said(w, id, 0x18) { lines.push(0x18); }
    if ctx.hero.hp <= 0 {
        let n = ctx.game.rng.randi(2) + 0x19;
        if !said(w, id, n) { lines.push(n); }
    }
    if lines.is_empty() { return; }
    let n = lines[ctx.game.rng.randi(lines.len() as i32) as usize];
    say(w, id, ctx, n);
    let wait = REMARK_WAIT + ctx.game.rng.randi(REMARK_WAIT);
    w.mobys[id].set_pv_i(pv::REMARK_WAIT, wait);
}

/// `$gp+0x21e4`-like window of the score: 600 ticks; its weights (damage, kills, health lost).
const WINDOW: i16 = 600;
const WEIGHTS: [f32; 3] = [0.2, 1.0, -2.0];

/// `0x441bb0`: the score (not in state 9's first phase) — the fighters and bosses out counted (those with health; the
/// bosses always) and their health summed, the 0x0d37s counted (`+0x200`); the remarks ([`remark`]) and, Ratchet
/// hurt, the crowd's gasp (the level's sound 0x17); over a ring of four windows of 600 ticks the kills, damage and
/// health lost, weighted 1/1..1/4 newest first: `0.2·damage + kills − 2·health lost` (`+0x14c`).
fn score(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 9 && w.mobys[id].timer == 0 { return; }
    let (mut n, mut life, mut d37s) = (0, 0.0, 0u16);
    for f in group(w, id, pv::ENEMIES) {
        let x = &w.mobys[f];
        if x.state >= 0x80 || x.state == 1 { continue; }
        let h = x.pv_f(0x20);
        match x.o_class {
            D98 | DCF | DF4 | DF7 | DF9 | D4B => {
                life += h;
                if 0.0 < h { n += 1; }
            }
            D37 => {
                life += h;
                d37s += 1;
                if 0.0 < h { n += 1; }
            }
            D4A | B3A => {
                n += 1;
                life += h;
            }
            _ => {}
        }
    }
    w.mobys[id].set_pv_s(pv::D37S, d37s as i16);
    let hp = ctx.hero.hp as f32;
    let m = &w.mobys[id];
    let (was_n, was_life, was_hp) = (m.pv_i(pv::ALIVE), m.pv_f(pv::LIFE), m.pv_f(pv::HEALTH));
    remark(w, id, ctx, was_n - n, was_life - life, was_hp - hp);
    if 0.0 < was_hp - hp { ctx.host.play_level_sound_after(None, 0x17, 0); }
    let m = &mut w.mobys[id];
    let r = pv::HISTORY + (m.pv_s(pv::RING) as usize & 3) * 0x10;
    if n < was_n { m.set_pv_i(r, m.pv_i(r) + was_n - n); }
    if life < was_life { m.set_pv_f(r + 4, m.pv_f(r + 4) + was_life - life); }
    if hp < was_hp { m.set_pv_f(r + 8, m.pv_f(r + 8) + was_hp - hp); }
    m.set_pv_f(pv::LIFE, life);
    m.set_pv_i(pv::ALIVE, n);
    m.set_pv_f(pv::HEALTH, hp);
    let (mut dmg, mut hurt, mut kills) = (0.0, 0.0, 0.0);
    let ring = m.pv_s(pv::RING) as i32;
    for k in (1..=4).rev() {
        let f = 1.0 / (5.0 - k as f32);
        let o = pv::HISTORY + ((ring + k).rem_euclid(4) as usize) * 0x10;
        dmg += f * m.pv_f(o + 4);
        kills += f * m.pv_i(o) as f32;
        hurt += f * m.pv_f(o + 8);
    }
    m.set_pv_f(pv::SCORE, WEIGHTS[0] * dmg + WEIGHTS[1] * kills + WEIGHTS[2] * hurt);
    let mut t = m.pv_s(pv::WINDOW) as i32;
    if crate::moby::countdown(&mut t) {
        let r = ((m.pv_s(pv::RING) + 1) & 3) as i16;
        m.set_pv_s(pv::RING, r);
        let o = pv::HISTORY + r as usize * 0x10;
        m.set_pv_i(o, 0);
        m.set_pv_f(o + 4, 0.0);
        m.set_pv_f(o + 8, 0.0);
        t = WINDOW as i32;
    }
    m.set_pv_s(pv::WINDOW, t as i16);
}

/// `0x443f60`: the wave is over — every fighter it calls for sent, its bosses sent, none left.
fn wave_over(w: &MobyWorld, id: usize, wave: &Wave) -> bool {
    let m = &w.mobys[id];
    let mut done = (0..6).all(|k| m.pv_i(pv::SENT + 4 * k) == wave.fighters[k] as i32);
    if wave.d4a { done &= m.pv_i(pv::SENT_D4A) != 0; }
    if wave.b3a { done &= m.pv_i(pv::SENT_B3A) != 0; }
    m.pv_i(pv::ALIVE) == 0 && done
}

fn slot(w: &MobyWorld, id: usize) -> Option<usize> {
    let k = w.mobys[id].pv_i(pv::SLOT).clamp(0, 7) as usize;
    ai::link(&w.mobys[id], pv::SLOTS + 4 * k)
}

/// `0x442958`: once every spawner of the round is closed and empty, the round starts again.
fn slots_reset(w: &mut MobyWorld, id: usize) {
    let n = w.mobys[id].pv_i(pv::SLOT).clamp(0, 7) as usize;
    for k in 0..=n {
        if let Some(g) = ai::link(&w.mobys[id], pv::SLOTS + 4 * k) {
            if !maktar::empty(w, g) { return; }
        }
    }
    for k in 0..=n { w.mobys[id].set_pv_i(pv::SLOTS + 4 * k, 0); }
    w.mobys[id].set_pv_i(pv::SLOT, 0);
}

/// `0x4443b0`: a spawner ahead of the camera from Ratchet (its flattened way within 35° of the camera's flattened
/// forward, `0x1bab70`) and more than 30 from him.
fn ahead(w: &MobyWorld, g: usize, ctx: &Ctx) -> bool {
    let flat = |v: V3| set_len([v[0], v[1], 0.0], 1.0);
    let a = flat(ctx.host.camera_rows()[0]);
    let c = flat(sub(w.mobys[g].pos, ctx.hero.pos));
    0.81915 < a[0] * c[0] + a[1] * c[1] && 30.0 < dist(w.mobys[g].pos, ctx.hero.pos)
}

/// `0x442668`: the round's spawner — the current one, or once it is full the next (up to the wave's count): the first
/// of the group ready and [`ahead`], else the first ready more than 30 from Ratchet.
fn slot_pick(w: &mut MobyWorld, id: usize, ctx: &Ctx, wave: &Wave) {
    let pick = match slot(w, id) {
        Some(g) => {
            let k = w.mobys[id].pv_i(pv::SLOT);
            if !maktar::ready(w, g) && k < wave.spawners as i32 - 1 {
                w.mobys[id].set_pv_i(pv::SLOT, k + 1);
                true
            } else {
                false
            }
        }
        None => true,
    };
    if !pick { return; }
    let k = w.mobys[id].pv_i(pv::SLOT).clamp(0, 7) as usize;
    let gs = group(w, id, pv::SPAWNERS);
    let found = gs.iter().copied().find(|&g| ahead(w, g, ctx) && maktar::ready(w, g));
    if let Some(g) = found {
        ai::set_link(&mut w.mobys[id], pv::SLOTS + 4 * k, Some(g));
        return;
    }
    if let Some(g) = gs.iter().copied().find(|&g| maktar::ready(w, g) && 30.0 < dist(w.mobys[g].pos, ctx.hero.pos)) {
        ai::set_link(&mut w.mobys[id], pv::SLOTS + 4 * k, Some(g));
    }
}

/// `0x4428e0`: each spawner of the round holding fighters opened (`0x418a58`).
fn open_slots(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let n = w.mobys[id].pv_i(pv::SLOT).clamp(0, 7) as usize;
    for k in 0..=n {
        if let Some(g) = ai::link(&w.mobys[id], pv::SLOTS + 4 * k) {
            if maktar::loaded(w, g) { maktar::open(w, g, ctx); }
        }
    }
}

/// `$gp+0x2594` / `+0x2598` / `+0x258c..`: state 9's fighters come in round the middle a step of 0.3142 apart, 14 out,
/// at a height of 470..480.
const PLACE_STEP: f32 = 0.3142;
const PLACE_RADIUS: f32 = 14.0;
const PLACE_Z: (f32, f32) = (470.0, 480.0);

fn count_sent(w: &mut MobyWorld, id: usize, class: i16) {
    if let Some(k) = limit_slot(class) {
        let m = &mut w.mobys[id];
        m.set_pv_i(pv::SENT + 4 * k, m.pv_i(pv::SENT + 4 * k) + 1);
    }
}

/// `0x441fe0(m, class, placed)`: a pooled fighter of `class` (its owner the master): `placed`, round the middle and
/// started at once (`0x4421c0`, counted but for 0x0df9); else into the round's spawner (`0x4424e0`: a 0x0df9 not into
/// one that holds a fighter), its life 1 until it is let out, counted.
fn send(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, class: i16, place: bool) -> bool {
    let Some(f) = group(w, id, pv::ENEMIES).into_iter().find(|&f| { let x = &w.mobys[f]; x.o_class == class && x.state == 1 }) else { return false };
    w.mobys[f].parent = Some(id);
    if place {
        let h = add_rot(w.mobys[id].pv_f(pv::HEADING), PLACE_STEP);
        w.mobys[id].set_pv_f(pv::HEADING, h);
        let mid = middle(w, id, ctx).unwrap_or_default();
        let z = ctx.game.rng.randf(PLACE_Z.0, PLACE_Z.1);
        let x = &mut w.mobys[f];
        x.pos = [PLACE_RADIUS * h.cos() + mid[0], PLACE_RADIUS * h.sin() + mid[1], z];
        x.rot = [0.0, -std::f32::consts::FRAC_PI_2, h];
        x.rows = crate::moby::attach::rows_of(x.rot);
        if class != DF9 {
            start(w, f, ctx, 0, true);
            count_sent(w, id, class);
        }
        return true;
    }
    let Some(g) = slot(w, id).filter(|&g| maktar::ready(w, g)) else { return false };
    if class == DF9 && maktar::loaded(w, g) { return false; }
    count_sent(w, id, class);
    w.mobys[f].set_pv_f(0x20, 1.0);
    maktar::attach(w, g, f);
    true
}

/// `0x4420b8(m, class)`: a pooled boss (0x0d4a or 0x0b3a) started ([`start_boss`]) and counted; the boss.
fn send_boss(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, class: i16) -> Option<usize> {
    let f = group(w, id, pv::ENEMIES).into_iter().find(|&f| { let x = &w.mobys[f]; x.o_class == class && x.state == 1 })?;
    w.mobys[f].parent = Some(id);
    start_boss(w, f, ctx);
    let o = if class == D4A { pv::SENT_D4A } else { pv::SENT_B3A };
    let m = &mut w.mobys[id];
    m.set_pv_i(o, m.pv_i(o) + 1);
    Some(f)
}

/// The wave's boss sent: its owner, the announcer's line, the bosses' music.
fn boss_out(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, class: i16, line: i32, music: bool) {
    let b = send_boss(w, id, ctx, class);
    ai::set_link(&mut w.mobys[id], pv::BOSS, b);
    ai::set_link(&mut w.mobys[id], pv::OWNER, b);
    say(w, id, ctx, line);
    if music {
        let m = &w.mobys[id];
        ctx.host.music_request(m.pv_i(pv::MUSIC_BOSS), m.pv_i(pv::STINGER_BOSS));
    }
}

/// `0x442ba0` (every fourth frame in a challenge): the fighters' turns given back to their class (`+0xe0..`), then
/// the two nearest Ratchet (out, past state 3, of 0x0d98 / 0x0dcf / 0x0df9 / 0x0d37) given one each while their
/// class has one left.
fn deal_turns(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let g = group(w, id, pv::ENEMIES);
    let give = |w: &mut MobyWorld, c: i16, d: i32| if let Some(k) = limit_slot(c) { let m = &mut w.mobys[id]; m.set_pv_i(pv::LIMITS + 4 * k, m.pv_i(pv::LIMITS + 4 * k) + d); };
    for &f in &g {
        let c = w.mobys[f].o_class;
        if let Some(o) = turn_word(c) {
            if w.mobys[f].pv_s(o) == 1 {
                w.mobys[f].set_pv_s(o, 0);
                give(w, c, 1);
            }
        }
    }
    for _ in 0..2 {
        let mut best: Option<(usize, f32)> = None;
        for &f in &g {
            let x = &w.mobys[f];
            let Some(o) = turn_word(x.o_class) else { continue };
            if x.state >= 0x80 || x.pv_s(o) == 1 || x.state < 4 { continue; }
            let d = dist2(x.pos, ctx.hero.pos);
            if best.is_none_or(|(_, b)| d < b) { best = Some((f, d)); }
        }
        let Some((f, _)) = best else { continue };
        let c = w.mobys[f].o_class;
        let Some(k) = limit_slot(c) else { continue };
        if 0 < w.mobys[id].pv_i(pv::LIMITS + 4 * k) {
            give(w, c, -1);
            w.mobys[f].set_pv_s(turn_word(c).unwrap(), 1);
        }
    }
}

/// `0x2a8af0`: inside the polygon (crossings of a ray along x).
pub(super) fn in_polygon(pts: &[[f32; 4]], p: V3) -> bool {
    let n = pts.len();
    let mut inside = false;
    for k in 0..n {
        let (a, b) = (pts[k], pts[(k + 1) % n]);
        if ((a[1] < p[1] && p[1] <= b[1]) || (b[1] < p[1] && p[1] <= a[1])) && a[0] + (p[1] - a[1]) / (b[1] - a[1]) * (b[0] - a[0]) < p[0] {
            inside = !inside;
        }
    }
    inside
}

/// `0x443808(m, x, hero)`: `x` is out — not in the arena's cylinder `+0x1e4` (a moby with no update never); a fighter
/// in the pit's cylinder `+0x160` (0x0d4b 0.8 lower); with the ring down (mode 3) under the middle's height + 3 (0.5
/// for Ratchet); with the ring shrinking or growing (2 / 4) and under the middle + 0.5, outside its polygon.
fn outside(w: &MobyWorld, id: usize, ctx: &Ctx, x: Option<usize>, pos: V3) -> bool {
    let m = &w.mobys[id];
    let hero = x.is_none() || x == ctx.hero_moby;
    let out = !ctx.level.in_cylinder(m.pv_i(pv::ZONE), pos);
    if let Some(x) = x.filter(|_| !hero) {
        if w.mobys[x].mode & crate::moby::mode::NO_UPDATE != 0 { return false; }
    }
    if out { return true; }
    if !hero {
        let mut p = pos;
        if x.is_some_and(|x| w.mobys[x].o_class == D4B) { p[2] -= 0.8; }
        if ctx.level.in_cylinder(m.pv_i(pv::PIT), p) { return true; }
    }
    let mode = m.pv_s(pv::RING_MODE);
    let mid = middle(w, id, ctx).unwrap_or_default();
    if mode == 3 && pos[2] < mid[2] + if hero { 0.5 } else { 3.0 } { return true; }
    if (mode == 2 || mode == 4) && pos[2] < mid[2] + 0.5 { return !in_polygon(&w.level11.arena_ring, pos); }
    false
}

/// `0x4439c0`: a fighter out ([`outside`], past state 2) dies ([`die_out`]); Ratchet out dies (`0x2cb710`).
fn keep_in(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for f in group(w, id, pv::ENEMIES) {
        let x = &w.mobys[f];
        if x.state >= 0x80 || x.state < 3 { continue; }
        if !matches!(x.o_class, D98 | DCF | DF4 | DF7 | DF9 | D37 | D4B) { continue; }
        if outside(w, id, ctx, Some(f), x.pos) { die_out(w, f, ctx); }
    }
    let hero = ctx.hero.pos;
    if outside(w, id, ctx, None, hero) { ctx.hero.die(); }
}

/// `$gp+0x28e4..+0x28ec`: the ring's scale out (7.5) and in (0.5), its step a tick (0.01).
const RING_OUT: f32 = 7.5;
const RING_IN: f32 = 0.5;
const RING_STEP: f32 = 0.01;

/// `0x447c10`: the ring (`+0x1fa`): **1** the door moby's rows reset, its polygon (path `+0x1c8`) its rest shape
/// (`+0x1cc`), the door shown at scale 7.5 → **2** it shrinks to 0.5 by 0.01 a tick → **3** in; **4** it grows back
/// to 7.5, then hidden → **0**; **5** at once out and hidden. A change of scale scales the polygon about the door (at
/// its height) and sets the door's first two rows to that length. The polygon is kept in
/// [`super::Globals::arena_ring`] (the original rewrites the path itself).
fn ring_step(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let Some(door) = placed(w, w.mobys[id].pv_i(pv::DOOR)) else { return };
    let mut prev = w.mobys[id].pv_f(pv::SCALE);
    let mut mode = w.mobys[id].pv_s(pv::RING_MODE);
    let mut scale = prev;
    if mode == 1 {
        w.mobys[door].rows = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
        prev = 1.0;
        w.level11.arena_ring = usize::try_from(w.mobys[id].pv_i(pv::RING_REST)).ok().and_then(|i| ctx.level.paths.get(i)).cloned().unwrap_or_default();
        let d = &mut w.mobys[door];
        d.mode &= !1;
        d.drawn = 1;
        d.draw_dist = 0xff;
        scale = RING_OUT;
        mode = 2;
    }
    match mode {
        2 => {
            approach(RING_IN, RING_STEP, &mut scale);
            if (scale - RING_IN).abs() < 0.01 { mode = 3; }
        }
        4 => {
            approach(RING_OUT, RING_STEP, &mut scale);
            if (scale - RING_OUT).abs() < 0.01 {
                w.mobys[door].mode |= 1;
                w.mobys[door].drawn = 0;
                mode = 0;
            }
        }
        5 => {
            scale = RING_OUT;
            w.mobys[door].mode |= 1;
            w.mobys[door].drawn = 0;
            mode = 0;
        }
        _ => {}
    }
    w.mobys[id].set_pv_f(pv::SCALE, scale);
    w.mobys[id].set_pv_s(pv::RING_MODE, mode);
    if prev != scale {
        let c = w.mobys[door].pos;
        for v in &mut w.level11.arena_ring {
            let k = scale / prev;
            *v = [(v[0] - c[0]) * k + c[0], (v[1] - c[1]) * k + c[1], c[2], v[3]];
        }
        let d = &mut w.mobys[door];
        d.rows[0] = set_len(d.rows[0], scale);
        d.rows[1] = set_len(d.rows[1], scale);
        d.mode |= crate::moby::mode::KEEP_ROWS;
    }
}

/// `0x444bd0`: the overview camera — the script camera at `$gp+0x2540` (angles `+0x2550`) moving over 360 ticks to
/// `+0x2520` (angles `+0x2530`).
const MENU_START: V3 = [592.0, 640.5, 440.6];
const MENU_START_ANGLES: V3 = [0.0, 0.5, 2.4];
const MENU_EYE: V3 = [550.44, 707.77, 432.02];
const MENU_ANGLES: V3 = [0.0, 0.0, std::f32::consts::FRAC_PI_2];
const MENU_TICKS: i32 = 0x168;

fn menu_camera(ctx: &mut Ctx) {
    ctx.game.cameras.camera_script(MENU_START, MENU_START_ANGLES, crate::script_camera::MODE_MOVE, MENU_TICKS, false, ctx.hero);
    ctx.game.cameras.camera_script_targets(Some(MENU_EYE), Some(MENU_ANGLES));
}

/// `0x444c28`: the camera come to rest, the menu (4) or the win's screen (5).
fn overview(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (eye, _) = ctx.game.cameras.view_euler();
    if dist2(eye, MENU_EYE) < 0.01 {
        match w.mobys[id].state {
            4 => w.mobys[id].set_state(7, -1),
            5 => w.mobys[id].set_state(6, -1),
            _ => {}
        }
        ctx.game.cameras.camera_script_targets(Some(MENU_EYE), Some(MENU_ANGLES));
    }
}

/// Ratchet put at `p` facing `r` (`0x2d1a58(p, r, state, 1)`), the bot of the Spiderbot Glove ended (its warp).
fn put_hero(w: &mut MobyWorld, ctx: &mut Ctx, p: V3, r: V3, state: i32) {
    ctx.hero.teleport(p, r);
    super::super::spiderbot::end(w, false, 0);
    ctx.hero.request_state(state, true);
}

fn checkpoint(ctx: &mut Ctx) {
    let (p, r) = (ctx.hero.pos, ctx.hero.rot);
    ctx.game.save_checkpoint(p, r);
}

/// The overview's way in (states 4 / 5 and after a first win's scene): the door hidden, the crates put away, the
/// fighters pooled, the floor down, Ratchet at the middle (state 0x6f), the checkpoint, the overview camera, the
/// ring out, the gravity off.
fn to_overview(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    hide_door(w, id);
    put_crates_away(w, id, ctx);
    pool_all(w, id, ctx);
    floor_down(w, id);
    if let Some(mid) = middle(w, id, ctx) { put_hero(w, ctx, mid, MIDDLE_ANGLES, 0x6f); }
    checkpoint(ctx);
    menu_camera(ctx);
    w.level11.arena_gravity = [false; 2];
}

/// The challenge's end: the Spiderbot Glove's bot blown up (`0x3f9048(1, −1)`; the upgraded glove's `0x460f68` not
/// ported).
fn end_bots(w: &mut MobyWorld) { super::super::spiderbot::end(w, true, -1); }

/// `0x444008(m, n)`: challenge `n` won — the first win of the first challenge: line 1 and item 0x13; of the second:
/// line 3 and item 0x33; else line 0x1e / 0x1f, the XP and the bolts (`+0x1e0`, the bolt counter shown); the tenth's
/// first win also the challenge mode's unlock (`0x1a7a60`, `0x375010(1)`, the message 0x1233, `0x1a903c`: not
/// ported); its win counted; the autosave (`0x31f9d0(0, −1)`, crate::game::GameState::autosave).
fn reward(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, n: i32) {
    if n == 0 && wins(ctx.game, 0) == 0 {
        say(w, id, ctx, 1);
        ctx.game.get_item(0x13);
    } else if n == 1 && wins(ctx.game, 1) == 0 {
        say(w, id, ctx, 3);
        ctx.game.get_item(0x33);
    } else {
        let line = ctx.game.rng.randi(2) + 0x1e;
        say(w, id, ctx, line);
        let (bolts, xp) = ctx.level.joba.get(n as usize).map_or((0, 0), |c| (c.bolts, c.xp));
        ctx.game.xp += maktar::xp_of(xp, wins(ctx.game, n));
        let b = maktar::prize_of(bolts, wins(ctx.game, n), ctx.game.challenge);
        w.mobys[id].set_pv_i(pv::PRIZE, b);
        ctx.game.bolts = (ctx.game.bolts as i32 + b).max(0) as u32;
        ctx.game.show_bolts = Some(0x168);
    }
    if let Some(k) = win_flag(n) {
        if let Some(f) = ctx.game.flags.get_mut(k) { *f = f.saturating_add(1); }
    }
    ctx.game.autosave(-1);
}

/// `0x443e18`: back to the resort — line 0x20, state 1, chunk 0 with the resort's music, Ratchet at the cuboid `+0x15c`
/// facing 2.16 once the second challenge is won, else `+0x158` facing −2.52 (the camera reset), the checkpoint, the
/// fighters pooled, the resort's music, no drop index.
fn leave(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    say(w, id, ctx, 0x20);
    w.mobys[id].set_state(1, -1);
    ctx.switch_chunk(0, w.mobys[id].pv_i(pv::MUSIC_RESORT));
    let (off, yaw) = if wins(ctx.game, 1) != 0 { (pv::BACK_WON, 2.16) } else { (pv::BACK, -2.52) };
    if let Some((p, _)) = ctx.level.cuboid_place(w.mobys[id].pv_i(off)) {
        put_hero(w, ctx, p, [0.0, 0.0, yaw], 0);
        ctx.game.cameras.reset();
    }
    checkpoint(ctx);
    pool_all(w, id, ctx);
    let m = &w.mobys[id];
    ctx.host.music_request(m.pv_i(pv::MUSIC_RESORT), m.pv_i(pv::STINGER_RESORT));
    ctx.game.arena_drop = -1;
    ctx.game.arena_menu = None;
}

/// The wipe's tick (`0x447550`): `+0x1f4` down 2 a frame (to 0), drawn by crate::moby::classes::arena::draw_wipe;
/// the height before.
fn wipe_tick(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) -> i16 {
    let before = w.mobys[id].pv_s(pv::WIPE);
    let h = (before as u16).wrapping_sub(2) as i16;
    w.mobys[id].set_pv_s(pv::WIPE, h.max(0));
    ctx.game.arena_wipe = Some(h.max(0));
    before
}

/// The first challenge from the entry: Ratchet at the middle, challenge 0 wave 0, state 8, the camera released.
fn first_challenge(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if let Some(mid) = middle(w, id, ctx) { put_hero(w, ctx, mid, MIDDLE_ANGLES, 0); }
    let m = &mut w.mobys[id];
    m.set_pv_s(pv::CHALLENGE, 0);
    m.set_pv_s(pv::WAVE, 0);
    m.set_state(8, -1);
    ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_BLEND, ctx.hero);
}

/// State 2: the fade (a twentieth a tick), then the wipe, chunk 1 with the arena's music, Ratchet at the entry cuboid
/// `+0x16c` (state 0x6f), the checkpoint, the camera 2 above him; through the wipe he walks to the middle; within 10
/// of it the first time the welcome scene (the trigger `+0x1d4` hooked, `0x1395b8 + 0x4a`); then (after the scene) the
/// first challenge until it is won, else the overview.
fn enter(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        floor_down(w, id);
        w.mobys[id].timer = 0;
        hide_door(w, id);
        set_platform_z(w, id, PLATFORM_LOW);
    }
    match w.mobys[id].timer {
        0 => {
            let t = w.mobys[id].state_ticks;
            if t < 0x1f {
                ctx.game.fade_black = (t as u16 as f32 * 0.05).min(1.0);
                return;
            }
            w.mobys[id].set_pv_s(pv::WIPE, crate::gui::SCREEN_H as i16);
            wipe_tick(w, id, ctx);
            ctx.switch_chunk(1, w.mobys[id].pv_i(pv::MUSIC_ARENA));
            if let Some((p, r)) = ctx.level.cuboid_place(w.mobys[id].pv_i(pv::ENTRY)) {
                put_hero(w, ctx, p, r, 0x6f);
                checkpoint(ctx);
                let m = &w.mobys[id];
                ctx.host.music_request(m.pv_i(pv::MUSIC_ARENA), m.pv_i(pv::STINGER_ARENA));
                ctx.game.cameras.camera_script([p[0], p[1], p[2] + 2.0], r, crate::script_camera::MODE_SNAP, 0, false, ctx.hero);
            }
            w.mobys[id].timer = 2;
        }
        2 => {
            let h = wipe_tick(w, id, ctx);
            ctx.game.fade_black = 0.0;
            if 0 < h { return; }
            w.mobys[id].timer = 3;
            ctx.hero.request_state(0, true);
            if let Some(mid) = middle(w, id, ctx) { ctx.hero.script_walk_to(0.0, mid, false); }
        }
        3 => {
            let Some(mid) = middle(w, id, ctx) else { return };
            if 10.0 <= dist(ctx.hero.pos, mid) { return; }
            if ctx.game.flags.get(FLAG_INTRO) == Some(&0) {
                ctx.game.flags[FLAG_INTRO] = 1;
                w.mobys[id].timer = 1;
                if let Some(t) = placed(w, w.mobys[id].pv_i(pv::INTRO)) { w.mobys[t].once |= 4; }
                w.mobys[id].state_ticks = 0;
            } else if wins(ctx.game, 0) != 0 {
                w.mobys[id].set_state(4, -1);
            } else {
                first_challenge(w, id, ctx);
            }
        }
        1 => {
            if 0x3d <= w.mobys[id].state_ticks && ctx.game.scenes.mode != crate::scene::SCENE {
                if wins(ctx.game, 0) == 0 { first_challenge(w, id, ctx); } else { w.mobys[id].set_state(4, -1); }
            }
        }
        _ => {}
    }
}

/// State 7, `0x444ce8`: the menu — ← / → the challenges open (a repeat of 15 ticks, the level's sound 3), ✕ the
/// selected one starts (its flag `0x1395b8 + 0x49`, the camera released, Ratchet idle, sound 4), □ the vendor, △ back
/// to the resort.
fn menu(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::pad::button;
    if ctx.game.vendor_virtual.is_some() { return; }
    if ctx.hero.state != 0x6f { ctx.hero.request_state(0x6f, true); }
    let pressed = ctx.hero.items.pressed;
    let cs = &ctx.level.joba;
    let sel = w.mobys[id].pv_s(pv::SELECTED) as i32;
    if w.mobys[id].pv_s(pv::REPEAT) == 0 {
        if pressed & button::RIGHT != 0 && sel < 12 {
            if let Some(n) = (sel + 1..13).find(|&n| on_menu(cs, ctx.game, n)) {
                let m = &mut w.mobys[id];
                m.set_pv_s(pv::SELECTED, n as i16);
                m.set_pv_s(pv::REPEAT, 15);
                m.set_pv_s(pv::ARROW_RIGHT, 15);
                ctx.host.play_level_sound(None, 3, 0);
            }
        } else if pressed & button::LEFT != 0 && 0 < sel {
            let n = if sel == 1 { 0 } else { (0..sel).rev().find(|&n| on_menu(cs, ctx.game, n)).unwrap_or(0) };
            let m = &mut w.mobys[id];
            m.set_pv_s(pv::SELECTED, n as i16);
            m.set_pv_s(pv::REPEAT, 15);
            m.set_pv_s(pv::ARROW_LEFT, 15);
            ctx.host.play_level_sound(None, 3, 0);
        }
    } else {
        let mut t = w.mobys[id].pv_s(pv::REPEAT) as i32;
        crate::moby::countdown(&mut t);
        w.mobys[id].set_pv_s(pv::REPEAT, t as i16);
    }
    let sel = w.mobys[id].pv_s(pv::SELECTED) as i32;
    if pressed & button::CROSS != 0 {
        if sel != 0 && !on_menu(cs, ctx.game, sel) { return; }
        let m = &mut w.mobys[id];
        m.set_pv_s(pv::CHALLENGE, sel as i16);
        m.set_pv_s(pv::WAVE, 0);
        ctx.game.flags[FLAG_SELECTED] = sel as u8;
        m.set_state(8, -1);
        ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_BLEND, ctx.hero);
        ctx.hero.request_state(0, true);
        ctx.host.play_level_sound(None, 4, 0);
        ctx.game.arena_menu = None;
        return;
    }
    if pressed & button::SQUARE != 0 {
        // `0x332158(5, 1, 0, 0, 0)`: the vendor (its weapons) with no vendor moby.
        ctx.game.vendor_virtual = Some(0);
        ctx.game.arena_menu = None;
        return;
    }
    if pressed & button::TRIANGLE != 0 {
        leave(w, id, ctx);
        return;
    }
    ctx.game.arena_menu = Some(maktar::Menu {
        selected: sel,
        name: challenge_name(sel),
        prize: (sel != 0 || wins(ctx.game, 0) != 0).then(|| maktar::prize_of(cs.get(sel as usize).map_or(0, |c| c.bolts), wins(ctx.game, sel), ctx.game.challenge)),
        wins: wins(ctx.game, sel),
        prev: sel != 0,
        next: (sel + 1..13).any(|n| on_menu(cs, ctx.game, n)),
        held: ctx.hero.items.held,
        frame: ctx.game.frame,
    });
}

/// State 6, `0x4473b0`: the win's screen — ✕ the menu at the next challenge open after it, △ back to the resort.
fn results(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::pad::button;
    if ctx.hero.state != 0x6f { ctx.hero.request_state(0x6f, true); }
    let pressed = ctx.hero.items.pressed;
    if pressed & button::CROSS != 0 {
        w.mobys[id].set_state(7, -1);
        let ch = w.mobys[id].pv_s(pv::CHALLENGE);
        w.mobys[id].set_pv_s(pv::SELECTED, ch);
        ctx.game.flags[FLAG_SELECTED] = ch as u8;
        if let Some(n) = (ch as i32 + 1..13).find(|&n| on_menu(&ctx.level.joba, ctx.game, n)) {
            w.mobys[id].set_pv_s(pv::SELECTED, n as i16);
            ctx.game.flags[FLAG_SELECTED] = n as u8;
        }
    } else if pressed & button::TRIANGLE != 0 {
        leave(w, id, ctx);
    } else {
        // `0x4470a8`: the first two challenges' first wins their own lines, else the prize.
        let m = &w.mobys[id];
        let sel = m.pv_s(pv::SELECTED) as i32;
        let first = match sel { 0 if wins(ctx.game, 0) < 2 => Some(0x2e2c), 1 if wins(ctx.game, 1) < 2 => Some(0x2e2d), _ => None };
        ctx.game.arena_results = Some(maktar::Results { first: first.is_some(), first_line: first.unwrap_or(0), prize: m.pv_i(pv::PRIZE) });
    }
}

/// `0x447a48`: the remarks on the wave's boss (`+0x180`): 0x0b3a — Ratchet down, line 6 or 7; under 0.25 health 8;
/// the boss gone 9; 0x0d4a — down, 0xc..0xe; under 0.25 0xf; the boss gone and none left 0x10. Each once.
fn boss_remarks(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(b) = ai::link(&w.mobys[id], pv::BOSS) else { return };
    let (life, hp) = (w.mobys[b].pv_f(0x20), ctx.hero.hp as f32);
    let mut n = -1;
    match w.mobys[b].o_class {
        D4A => {
            if hp <= 0.0 {
                let k = ctx.game.rng.randi(3) + 0xc;
                if !said(w, id, k) { n = k; }
            }
            if !said(w, id, 0xf) && hp < 0.25 { n = 0xf; }
            if !said(w, id, 0x10) && life <= 0.0 && w.mobys[id].pv_i(pv::ALIVE) == 0 { n = 0x10; }
        }
        B3A => {
            if hp <= 0.0 {
                let k = ctx.game.rng.randi(2) + 6;
                if !said(w, id, k) { n = k; }
            }
            if !said(w, id, 8) && hp < 0.25 { n = 8; }
            if !said(w, id, 9) && life <= 0.0 { n = 9; }
        }
        _ => return,
    }
    if 0 < n { say(w, id, ctx, n); }
}

/// A wave's end (states 8 and 9): the next wave, the counts cleared, the boss none.
fn next_wave(w: &mut MobyWorld, id: usize, sent: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_s(pv::WAVE, m.pv_s(pv::WAVE) + 1);
    for k in 0..sent { m.set_pv_i(pv::SENT + 4 * k, 0); }
    m.set_pv_i(pv::ALIVE, 0);
    m.set_pv_i(pv::BOSS, 0);
    m.set_pv_i(pv::OWNER, 0);
}

/// The next wave's set-up: its crates lifted, its floor up.
fn wave_start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_s(pv::CHALLENGE) >= 13 { w.mobys[id].set_pv_s(pv::CHALLENGE, 0); }
    let Some(wave) = wave_of(w, id, &ctx.level.joba) else { return };
    lift_crates(w, id, ctx, &wave);
    floor_for(w, id, &wave);
}

/// `$gp+0x24a8` / `+0x24c0` / `+0x24c8`: the kills a timed challenge (5 / 8 / 12) counts before each wave.
const KILLS_5: [i32; 6] = [0, 15, 30, 45, 60, 75];
const KILLS_8: [i32; 2] = [0, 13];
const KILLS_12: [i32; 8] = [0, 16, 31, 47, 62, 77, 92, 0];
/// `$gp+0x24a0`: the turns to attack each fighter class gets at a challenge's start.
const LIMITS: [i32; 6] = [5, 8, 0, 10, 2, 10];
/// The fail messages: "… kills" (time out), hurt (3 / 11).
const FAIL_KILLS: (u32, u32) = (0x30ff, 0x2df7);
const FAIL_HURT: u32 = 0x3100;

fn kills_so_far(w: &MobyWorld, id: usize, before: &[i32]) -> i32 {
    let m = &w.mobys[id];
    let sent: i32 = (0..6).map(|k| m.pv_i(pv::SENT + 4 * k)).sum();
    before.get(m.pv_s(pv::WAVE) as usize).copied().unwrap_or(0) + sent - m.pv_i(pv::ALIVE)
}

/// The challenge lost: its message shown, state 11, the bots ended.
fn lose(w: &mut MobyWorld, id: usize) {
    w.mobys[id].set_state(11, -1);
    end_bots(w);
}

/// The wave's sending (states 8 and 9's first phase): the round's spawner, the wave's fighters into it (0x0df7,
/// 0x0dcf, 0x0df4, 0x0d98, 0x0df9, 0x0d37), then opened.
fn send_wave(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, counts: [(usize, i16, i32); 6]) {
    let mut sent = false;
    slots_reset(w, id);
    if let Some(wave) = wave_of(w, id, &ctx.level.joba) { slot_pick(w, id, ctx, &wave); }
    if slot(w, id).is_some_and(|g| maktar::ready(w, g)) {
        for (k, class, want) in counts {
            for _ in w.mobys[id].pv_i(pv::SENT + 4 * k)..want { sent |= send(w, id, ctx, class, false); }
        }
    }
    if sent { open_slots(w, id, ctx); }
}

/// The wave's fighters placed round the middle at once (state 9's waves).
fn place_wave(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, wave: &Wave) {
    for (k, class) in [(3, DF7), (5, D37), (1, DCF), (2, DF4), (0, D98)] {
        for _ in w.mobys[id].pv_i(pv::SENT + 4 * k)..wave.fighters[k] as i32 { send(w, id, ctx, class, true); }
    }
}

/// The way round the middle to Ratchet, less a quarter turn ([`pv::HEADING`]).
fn heading(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let mid = middle(w, id, ctx).unwrap_or_default();
    let h = atan2(ctx.hero.pos[0] - mid[0], ctx.hero.pos[1] - mid[1]);
    w.mobys[id].set_pv_f(pv::HEADING, add_rot(h, -std::f32::consts::FRAC_PI_2));
}

/// The ticks of states 8 and 9 alike: the boss's remarks, the turns every fourth frame, Ratchet and the fighters kept
/// in, the drop index (`0x43f8d0`).
fn fight_tail(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, every_fourth: bool) {
    boss_remarks(w, id, ctx);
    if !every_fourth || ctx.game.frame & 3 == 0 { deal_turns(w, id, ctx); }
    keep_in(w, id, ctx);
    let n = drop_index(w.mobys[id].pv_s(pv::CHALLENGE) as i32);
    if n != -1 { ctx.game.arena_drop = n; }
}

/// State 8: a challenge's waves from the spawners (module docs).
fn fight(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let ch = w.mobys[id].pv_s(pv::CHALLENGE);
    if w.mobys[id].first() {
        let m = &mut w.mobys[id];
        m.set_pv_s(pv::WAVE, 0);
        for k in 0..8 { m.set_pv_i(pv::SENT + 4 * k, 0); }
        m.set_pv_i(pv::FRAMES, 0);
        m.set_pv_i(pv::ALIVE, 0);
        put_crates_away(w, id, ctx);
        count_out(w, id, ctx);
        if let Some(wave) = wave_of(w, id, &ctx.level.joba) {
            lift_crates(w, id, ctx, &wave);
            floor_for(w, id, &wave);
        }
        let m = &mut w.mobys[id];
        for k in 0..0x28 { m.set_pv_s(pv::SAID + 2 * k, 0); }
        for (k, &l) in LIMITS.iter().enumerate() { m.set_pv_i(pv::LIMITS + 4 * k, l); }
        ctx.hero.hp = ctx.game.max_health;
        hide_door(w, id);
        if ch == 8 { if let Some(a) = ctx.game.ammo.get_mut(ITEM_8 as usize) { *a = 300; } }
    }
    let Some(wave) = wave_of(w, id, &ctx.level.joba) else { return };
    if wave.ring {
        w.mobys[id].set_state(9, -1);
        return;
    }
    score(w, id, ctx);
    if wave_over(w, id, &wave) {
        next_wave(w, id, 7);
        if waves(&ctx.level.joba, ch) <= w.mobys[id].pv_s(pv::WAVE) {
            w.mobys[id].set_pv_s(pv::SELECTED, ch);
            if ch == 0 && wins(ctx.game, 0) == 0 {
                w.mobys[id].set_state(12, -1);
                return;
            }
            w.mobys[id].set_state(5, -1);
            end_bots(w);
            return;
        }
        wave_start(w, id, ctx);
    }
    let frames = w.mobys[id].pv_i(pv::FRAMES) + 1;
    w.mobys[id].set_pv_i(pv::FRAMES, frames);
    let ch = w.mobys[id].pv_s(pv::CHALLENGE);
    if ch == 5 && 0x1519 <= frames {
        ctx.game.prompt.announce_count(FAIL_KILLS.0, kills_so_far(w, id, &KILLS_5), FAIL_KILLS.1, -1);
        lose(w, id);
        return;
    }
    if ch == 9 && 0x2a31 <= frames {
        ctx.game.prompt.announce(FAIL_KILLS.0, -1);
        lose(w, id);
        w.mobys[id].set_pv_i(pv::BOSS, 0);
        return;
    }
    if ch == 8 && 0xe11 <= frames {
        ctx.game.prompt.announce_count(FAIL_KILLS.0, kills_so_far(w, id, &KILLS_8), FAIL_KILLS.1, -1);
        lose(w, id);
        return;
    }
    if (ch == 3 || ch == 11) && ctx.hero.hp < ctx.game.max_health {
        ctx.game.prompt.announce(FAIL_HURT, -1);
        lose(w, id);
        w.mobys[id].set_pv_i(pv::BOSS, 0);
        return;
    }
    let Some(wave) = wave_of(w, id, &ctx.level.joba) else { return };
    if 0x3d <= w.mobys[id].state_ticks || wave.d4a {
        let f = wave.fighters;
        send_wave(w, id, ctx, [(3, DF7, f[3] as i32), (1, DCF, f[1] as i32), (2, DF4, f[2] as i32), (0, D98, f[0] as i32), (4, DF9, f[4] as i32), (5, D37, f[5] as i32)]);
        if w.mobys[id].pv_i(pv::SENT_D4A) == 0 && wave.d4a {
            boss_out(w, id, ctx, D4A, 10, true);
        } else if !wave.d4a {
            // A wave without the boss asks for the arena's music unless it plays or is pending.
            let track = w.mobys[id].pv_i(pv::MUSIC_ARENA);
            if ctx.host.music_tracks().is_none_or(|(now, next)| now as i32 != track && next as i32 != track) {
                ctx.host.music_request(track, w.mobys[id].pv_i(pv::STINGER_ARENA));
            }
        }
    }
    match ch {
        7 => { ctx.hero.items.ask(ctx.game.held, HAND_7); }
        8 => { ctx.hero.items.ask(ctx.game.held, HAND_8); }
        _ => {}
    }
    fight_tail(w, id, ctx, true);
}

/// The height above which state 9 turns the gravity's cylinder mode on.
const GRAVITY_TOP: f32 = 480.0;

/// State 9: a wave of the ring (module docs). Its first phase: the platform and the lamp rise (to 421.266); Ratchet
/// on the lamp (after 300 ticks, or in state 7) launched (states 7 → 0x81); the wave's 0x0dcf / 0x0df4 / 0x0d98 sent
/// (four each) from the spawners; above 480 at rest the gravity's cylinder mode (not ported) and the ring shrinks
/// ([`ring_step`]); in, the wave's fighters placed round the middle (phase 1). Phase 1: the wave over, the next if it
/// is a ring's too (its fighters placed, its 0x0b3a sent), else Ratchet back down (0x6f, phase 2, the ring grows back).
/// Phase 2: after 180 ticks the walkway word off, Ratchet's launch down (0x81, phase 3). Phase 3: the platform down;
/// the ring out, the next wave in state 8 or the challenge's end (13 for challenge 1's first win, else 5).
fn ring_fight(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let ch = w.mobys[id].pv_s(pv::CHALLENGE);
    if w.mobys[id].first() {
        if w.mobys[id].pv_i(pv::SENT_B3A) == 0 && wave_of(w, id, &ctx.level.joba).is_some_and(|wv| wv.b3a) {
            boss_out(w, id, ctx, B3A, 4, true);
        }
        w.mobys[id].set_pv_s(pv::RING_MODE, 1);
        w.mobys[id].timer = 0;
        let n = ctx.game.rng.randi(2) + 0x1b;
        say(w, id, ctx, n);
    } else {
        match w.mobys[id].timer {
            0 => {
                ring_step(w, id, ctx);
                score(w, id, ctx);
                let z = platform_z(w, id);
                if z < PLATFORM_HIGH { set_platform_z(w, id, z + PLATFORM_STEP); }
                let lamp = placed(w, w.mobys[id].pv_i(pv::LAMP));
                if lamp.is_some() && ctx.hero.ground_moby == lamp && ctx.hero.grounded_ticks != 0 && (ctx.hero.state == 7 || 0x12d <= w.mobys[id].state_ticks) {
                    ctx.hero.vel[0] = 0.0;
                    ctx.hero.vel[1] = 0.0;
                    w.level11.arena_launch = 80.0;
                    if ctx.hero.state == 7 { ctx.hero.request_state(0x81, true); } else if ctx.hero.state != 0x81 { ctx.hero.request_state(7, true); }
                    let l = w.mobys[lamp.unwrap()].pos;
                    if dist(ctx.hero.pos, l) < 0.01 { ctx.hero.pos[1] += 0.01; }
                }
                send_wave(w, id, ctx, [(1, DCF, 4), (2, DF4, 4), (0, D98, 4), (3, DF7, 0), (4, DF9, 0), (5, D37, 0)]);
                // Above 480 and slow: Ratchet's `+0x180` (`0x189fa0`) is read here as his vertical speed (not traced).
                if !w.level11.arena_gravity[1] && GRAVITY_TOP < ctx.hero.pos[2] && ctx.hero.vel[2] < 1.0 / 30.0 {
                    w.level11.arena_gravity = [true; 2];
                }
                if w.level11.arena_gravity[1] && w.mobys[id].pv_s(pv::RING_MODE) == 3 {
                    w.mobys[id].state_ticks = 0;
                    w.mobys[id].timer = 1;
                    for k in 0..6 { w.mobys[id].set_pv_i(pv::SENT + 4 * k, 0); }
                    heading(w, id, ctx);
                    if let Some(wave) = wave_of(w, id, &ctx.level.joba) { place_wave(w, id, ctx, &wave); }
                }
            }
            1 => {
                score(w, id, ctx);
                ring_step(w, id, ctx);
                if wave_of(w, id, &ctx.level.joba).is_some_and(|wv| wave_over(w, id, &wv)) {
                    let m = &mut w.mobys[id];
                    for k in 0..8 { m.set_pv_i(pv::SENT + 4 * k, 0); }
                    m.set_pv_i(pv::ALIVE, 0);
                    let next = ctx.level.joba.get(ch as usize).and_then(|c| c.waves.get(m.pv_s(pv::WAVE) as usize + 1)).copied();
                    if let Some(wave) = next.filter(|wv| wv.ring) {
                        heading(w, id, ctx);
                        let m = &mut w.mobys[id];
                        m.set_pv_s(pv::WAVE, m.pv_s(pv::WAVE) + 1);
                        place_wave(w, id, ctx, &wave);
                        if w.mobys[id].pv_i(pv::SENT_B3A) == 0 && wave.b3a { boss_out(w, id, ctx, B3A, 4, false); }
                    } else {
                        ctx.hero.request_state(0x6f, true);
                        w.mobys[id].timer = 2;
                        w.mobys[id].set_pv_s(pv::RING_MODE, 4);
                        w.mobys[id].state_ticks = 0;
                        end_bots(w);
                    }
                }
                let frames = w.mobys[id].pv_i(pv::FRAMES) + 1;
                w.mobys[id].set_pv_i(pv::FRAMES, frames);
                if ch == 12 && 0x1771 <= frames {
                    ctx.game.prompt.announce_count(FAIL_KILLS.0, kills_so_far(w, id, &KILLS_12), FAIL_KILLS.1, -1);
                    set_platform_z(w, id, PLATFORM_LOW);
                    lose(w, id);
                    return;
                }
            }
            2 => {
                ring_step(w, id, ctx);
                if 0xb5 <= w.mobys[id].state_ticks {
                    ctx.hero.surf.boots = 0;
                    w.level11.arena_launch = 5.0;
                    ctx.hero.request_state(0x81, true);
                    w.mobys[id].timer = 3;
                }
            }
            3 => {
                w.level11.arena_gravity = [false; 2];
                ring_step(w, id, ctx);
                let z = platform_z(w, id);
                if PLATFORM_LOW < z { set_platform_z(w, id, z - PLATFORM_STEP); }
                if w.mobys[id].pv_s(pv::RING_MODE) == 0 {
                    next_wave(w, id, 7);
                    if waves(&ctx.level.joba, ch) <= w.mobys[id].pv_s(pv::WAVE) {
                        let s = if ch == 1 && wins(ctx.game, 1) == 0 { 13 } else { 5 };
                        w.mobys[id].set_state(s, -1);
                        w.mobys[id].state_ticks = 0xb4;
                        end_bots(w);
                        w.mobys[id].set_pv_s(pv::SELECTED, ch);
                        return;
                    }
                    wave_start(w, id, ctx);
                    w.mobys[id].state = 8;
                }
            }
            _ => {}
        }
    }
    // The tail: the keep-in every eighth frame and the turns every 64th, on the master's own frames (the original's
    // `(frame & 7) == (address >> 8 & 7)`: here its index), the drop index.
    let (frame, slot) = (ctx.game.frame, id as i32);
    if frame & 7 == slot & 7 { keep_in(w, id, ctx); }
    if frame & 0x3f == slot & 0x3f { deal_turns(w, id, ctx); }
    boss_remarks(w, id, ctx);
    let n = drop_index(ch as i32);
    if n != -1 { ctx.game.arena_drop = n; }
}

/// 0x0d8a (`0x43f9e0`), Joba's arena master (module docs).
pub fn update_master(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.game.arena_wipe = None;
    ctx.game.arena_results = None;
    // `0x1a9010`: set on every tick, cleared at the desk (state 1).
    ctx.game.arena_on = w.mobys[id].state != 1;
    match w.mobys[id].state {
        0 => {
            reset(w, id);
            put_crates_away(w, id, ctx);
            w.level11.arena_gravity = [false; 2];
            hide_door(w, id);
            ctx.game.arena_menu = None;
        }
        1 => {
            let hero = ctx.hero.pos;
            let m = &w.mobys[id];
            let at_desk = ctx.level.in_cuboid(m.pv_i(pv::DESK), hero) || (wins(ctx.game, 1) != 0 && ctx.level.in_cuboid(m.pv_i(pv::DESK_WON), hero));
            if ctx.level.in_cylinder(m.pv_i(pv::ZONE), hero) {
                w.mobys[id].set_state(4, -1);
            } else if at_desk {
                // `0x447058` = Maktar's `0x400dc0`: △ with the prompt up.
                ctx.game.prompt.request(4, 0x2dd2, 1);
                if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 && ctx.game.prompt.owner == 4 { w.mobys[id].set_state(2, -1); }
            }
        }
        2 => enter(w, id, ctx),
        s @ (4 | 5) => {
            if s == 5 && w.mobys[id].state_ticks < 0xb4 {
                floor_down(w, id);
                keep_in(w, id, ctx);
            } else {
                if w.mobys[id].first() {
                    if s == 5 {
                        let n = w.mobys[id].pv_s(pv::CHALLENGE) as i32;
                        reward(w, id, ctx, n);
                    }
                    to_overview(w, id, ctx);
                    w.mobys[id].set_pv_s(pv::RING_MODE, 5);
                    ring_step(w, id, ctx);
                }
                overview(w, id, ctx);
            }
        }
        11 => {
            if w.mobys[id].state_ticks < 0x168 {
                floor_down(w, id);
                keep_in(w, id, ctx);
            } else {
                w.mobys[id].set_state(4, -1);
            }
        }
        s @ (12 | 13) => {
            // The first wins of challenges 0 / 1: after three seconds their scene (`+0x1d8` / `+0x1dc` hooked); once
            // it is over (four seconds in, Ratchet alive) the reward and the overview.
            if w.mobys[id].first() {
                floor_down(w, id);
                w.mobys[id].timer = 0;
            }
            if w.mobys[id].timer == 0 {
                if w.mobys[id].state_ticks < 0xb4 {
                    keep_in(w, id, ctx);
                } else {
                    let off = if s == 12 { pv::WIN_0 } else { pv::WIN_1 };
                    if let Some(t) = placed(w, w.mobys[id].pv_i(off)) { w.mobys[t].once |= 4; }
                    w.mobys[id].state_ticks = 0;
                    w.mobys[id].timer = 1;
                }
            } else if 0 < ctx.hero.hp && ctx.game.scenes.mode != crate::scene::SCENE && 0xf1 <= w.mobys[id].state_ticks {
                let n = w.mobys[id].pv_s(pv::CHALLENGE) as i32;
                reward(w, id, ctx, n);
                w.mobys[id].state = 5;
                to_overview(w, id, ctx);
            }
        }
        6 => results(w, id, ctx),
        7 => {
            if w.mobys[id].first() {
                let mut sel = ctx.game.flags.get(FLAG_SELECTED).copied().unwrap_or(0) as i32;
                if sel != 0 && !on_menu(&ctx.level.joba, ctx.game, sel) { sel = 0; }
                w.mobys[id].set_pv_s(pv::SELECTED, sel as i16);
            }
            menu(w, id, ctx);
        }
        8 => fight(w, id, ctx),
        9 => ring_fight(w, id, ctx),
        _ => {}
    }
    // `0x4419b0`: the state's ticks, the remarks' wait down.
    let m = &mut w.mobys[id];
    if (m.state_ticks as u16) < 0xffff { m.state_ticks = m.state_ticks.wrapping_add(1); }
    let mut t = m.pv_i(pv::REMARK_WAIT);
    crate::moby::countdown(&mut t);
    m.set_pv_i(pv::REMARK_WAIT, t);
}

// ---------------------------------------------------------------------------------------------------------------
// The spawners 0x0c7f

/// 0x0c7f (`0x4184e0`, eight in the rim wall, the master's group `+0xc4`): Maktar's 0x0a15 code with its own release.
pub fn update_spawner(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) { maktar::spawner_step(w, id, ctx, release) }

/// `0x418ab0`: the riders out, below and in front of it (`(0, −9, −3)` in its rows; the first 2 back in x, the others
/// 1 either way in y), each facing Ratchet and started by its class ([`start`]; a 0x0df7 0.5 higher), their delays
/// `$gp+0x1d8c` (0) apart.
fn release(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let r = m.rows;
    let l = [0.0, -9.0, -3.0];
    let p = crate::hero::physics::add(m.pos, [0, 1, 2].map(|k| l[0] * r[0][k] + l[1] * r[1][k] + l[2] * r[2][k]));
    let at = [[p[0] - 2.0, p[1], p[2]], [p[0], p[1] + 1.0, p[2]], [p[0], p[1] - 1.0, p[2]]];
    let mut delay = 0;
    for k in 0..3 {
        let Some(f) = ai::link(&w.mobys[id], 0x10 + 4 * k) else { continue };
        let hero = ctx.hero.pos;
        let x = &mut w.mobys[f];
        x.pos = at[k];
        x.rot = [0.0, 0.0, atan2(hero[0] - x.pos[0], hero[1] - x.pos[1])];
        x.rows = crate::moby::attach::rows_of(x.rot);
        if x.o_class == DF7 {
            x.pos[2] += 0.5;
            start(w, f, ctx, (delay as f32 / 6.0) as i32, false);
        } else {
            start(w, f, ctx, delay, false);
        }
        delay += RELEASE_GAP;
    }
    for k in 0..3 { w.mobys[id].set_pv_i(0x10 + 4 * k, 0); }
}

const RELEASE_GAP: i32 = 0;

// ---------------------------------------------------------------------------------------------------------------
// The crowd 0x126e

/// 0x126e (`0x4763d8`, 60 placed): one of the crowd in the stands. Maktar's 0x0e96 again
/// (crate::moby::classes::arena::crowd_step, the same levels `$gp+0x3b00..+0x3b10`: 2 / 3 / 4, the wave's 600
/// frames and 4°) with three differences: the master's score is its `+0x14c`, the wave goes round the arena's middle
/// ([`middle`]) and it is hidden (draw distance 0) also while `0x1a8ff4` is set (the gravity's cylinder mode,
/// [`super::Globals::arena_gravity`]). It waits while its master (the moby its `+0x0` names, `0x448fb8`) is in its
/// state 0 or 1; out of view (`0x3024c8(255, its place, 5)`) its draw distance 0.
pub fn update_crowd(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let master = usize::try_from(w.mobys[id].pv_i(0)).ok().filter(|&ms| w.mobys.get(ms).is_some_and(|m| 1 < m.state));
    let Some(ms) = master else { return };
    let p = w.mobys[id].pos;
    if w.level11.arena_gravity[1] || ctx.view.is_some_and(|v| v.culled(255.0, [p[0], p[1], p[2], 5.0])) {
        w.mobys[id].draw_dist = 0;
        return;
    }
    w.mobys[id].draw_dist = 0xff;
    let score = w.mobys[ms].pv_f(pv::SCORE);
    let c = middle(w, ms, ctx).unwrap_or_default();
    maktar::crowd_step(w, id, ctx, score, [c[0], c[1]]);
}
