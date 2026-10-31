//! Maktar's arena (the level's chunk 1, above the resort; L02): its master 0x0b59 and the pieces of the floor and wall
//! it works. The fighters are crate::moby::classes::fighters.
//!
//! * **The master 0x0b59** (`0x3fb1b0`, [`update_master`]). Its pvars: the groups `+0x40` (the fighters and bosses,
//!   pooled above the arena), `+0x44` (the spawners), `+0x48` (the flame columns), `+0x4c` (the floor plates), `+0xc4`
//!   / `+0xc8` (the crates 0x01ff / 0x01f5) and `+0xcc` (the crate lifts); the turns to attack `+0x50..+0x5c`; the
//!   desk's cuboid `+0x70` and cylinder `+0x74` (the teleporter, once the first challenge is won), the way back
//!   `+0x78`, the arena's cylinders `+0x7c` / `+0x80` and its floor's `+0x84`, its middle `+0x88` and entry `+0x8c`
//!   (cuboids); the announcer's lines `+0x130..` and the scene triggers `+0x108` (the welcome) / `+0x10c` (the first
//!   win). **1** the desk: the prompt "Enter the Battle Arena" (owner 4) and △ → **2**: after the wipe (31 ticks)
//!   chunk 1 (crate::game::GameState::switch_chunk), Ratchet at the entry, the checkpoint, the camera behind him; the
//!   first time the welcome scene and straight into the first challenge, else the overview once he is at the middle.
//!   **4 / 5** (5 after a win: its reward, [`prize`]) the fighters pooled, the columns and plates down, Ratchet frozen
//!   at the middle, the script camera moving to the big screen → **7** the menu ([`Menu`], [`draw_menu`]: ← / → the
//!   challenges open, [`on_menu`]; ✕ starts one, △ back to the resort) or **6** the win's screen (✕ the menu at the
//!   next challenge). **8** the challenge: the waves of [`CHALLENGES`] — each its fighters sent out through the
//!   spawners (the round's spawner far from Ratchet; then its doors opened), its columns and plates up and its crates
//!   lifted; a wave is over with all sent and none left; every fourth tick the two gladiators or bladed robots nearest
//!   Ratchet get the turns; a fighter that leaves the arena dies, Ratchet leaving it dies. Lost (time out in 3 / 5,
//!   out of shots in 7 / 8, hurt in 12 / 13): its message and **10**, then the overview. **0xb** the first win's scene.
//!   The wins count in the save's flag bytes (`0x1395b8 + 0x31..`, [`win_flag`]).
//! * **The spawners 0x0a15** (`0x3d8898`, six in the rim wall, the master's group `+0x44`): each holds up to three
//!   fighters (`+0x10` / `+0x14` / `+0x18`, [`attach`]) and, opened by the master ([`open`]), slides back 2 along its
//!   second row over 19 ticks, plays its sequence 1 and lets them out ([`release`]), waits 90 ticks, plays 2 and 3 and
//!   slides home. On its first update it steps 2 forward (its closed place) and makes its frame (class 0x1172, at its
//!   joint 0 plus `(0, 9.4, 0.65)` in its rows; `0x315438`), which follows it while its sequence plays.
//! * **The floor plates 0x0a03** (`0x3d8128`, twelve, the master's group `+0x4c`): the master wants a plate up or down
//!   (`+0xbc`, [`raise`]); it turns half a turn about its first row at 180° a second (60 ticks, `$gp+0x1350`) carrying
//!   its tie (the one of its tie group `+0xb0` within 1 of it, `0x3d8448`), a hum looping (its sound 0, sound 1 as it
//!   starts up). Up (state 1) and turning it hits what stands on it ([`hurt`]); up and in view, now and then sparks.
//!
//! * **The flame columns 0x0b89** ([`update_column`]) and **the crate lifts 0x0ccc** ([`update_lift`]).
//!
//! Not ported: the bosses 0x09f8 / 0x0a44 (the challenges with a boss wave, 2, 4, 6, 9, 11, 12 and 13, are kept off the
//! menu until they are: they could not end), the crowd 0x0e96 and its cheers (`0x3fb028`, `0x42e670`), the announcer's speech (`0x401d98`, its
//! remarks `0x401e68` / `0x402090`), the HUD of a challenge (`0x4014f0`), the wipe (`0x401a20`), the win's screen
//! (`0x400e10`), the wave's banner (`0x3ffee0`), the menu's arrows (`0x3ff388`) and its vendor (□, `0x332158(5)`), the
//! music (`0x335928`), the level's sounds, the challenges' weapon lock (`0x18b6f4`) and the fixed held items of 7 / 8 /
//! 13 beyond the hand request; the plates' sparks (`0x3d85a8`: the particle burst `0x345d88` has no port).

use crate::hero::physics::{add, add_rot, atan2, dist, dist2, set_len, sub, V3};
use crate::moby::attach::{angles_of, rows_of, turn, Carrier, Kind, Member};
use crate::moby::hits::HitTemplate;
use crate::moby::{ai, mode, Ctx, MobyWorld};
use super::props::{set_seq, wrapped};

/// The spawner, its frame, the floor plate.
pub const SPAWNER: i16 = 0x0a15;
const FRAME: i16 = 0x1172;
pub const PLATE: i16 = 0x0a03;
/// The spawner's riders (links, `ai::link`).
const RIDERS: [usize; 3] = [0x10, 0x14, 0x18];
/// The spawner's frame (`+0x1c`), slide timer (`+0x22`), wait (`+0x20`).
const FRAME_LINK: usize = 0x1c;
/// `$gp+0x13b0`: the frame's place from the spawner's joint 0, in its rows.
const FRAME_AT: V3 = [0.0, 9.4, 0.65];
/// The slide's ticks (0x13) and length.
const SLIDE: i16 = 0x13;
/// The plate's turn: 180° a second (`$gp+0x1350`), so 60 ticks for the half turn (`0x1b3fdc`).
const TURN_SPEED: f32 = 180.0;
const TURN_TICKS: i32 = 60;
/// `$gp+0x135c` / `+0x1358`: the plate's reach and push.
const REACH: f32 = 1.5;
const PUSH: f32 = 2.0;

/// The fighters a spawner can let out (`0x3d8e68`).
pub const FIGHTERS: [i16; 4] = [0x09ca, 0x0b22, 0x0c3b, 0x0c3e];

fn rider(w: &MobyWorld, id: usize, k: usize) -> Option<usize> { ai::link(&w.mobys[id], RIDERS[k]) }

/// `0x3d8d20`: a spawner that can take a fighter (closed, a slot free).
pub fn ready(w: &MobyWorld, id: usize) -> bool { w.mobys[id].state == 2 && (0..3).any(|k| rider(w, id, k).is_none()) }

/// `0x3d8d60`: it holds a fighter.
pub fn loaded(w: &MobyWorld, id: usize) -> bool { (0..3).any(|k| rider(w, id, k).is_some()) }

/// `0x3d8dd0`: fighter `f` takes the first free slot and waits in it (state 2).
pub fn attach(w: &mut MobyWorld, id: usize, f: usize) {
    if let Some(k) = (0..3).find(|&k| rider(w, id, k).is_none()) { ai::set_link(&mut w.mobys[id], RIDERS[k], Some(f)); }
    w.mobys[f].state = 2;
}

/// `0x3d8e10`: the master opens it: its sequence 0 at speed 1, state 3.
pub fn open(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    ctx.host.set_anim_speed(&w.mobys[id], 1.0);
    if seq_of(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0); }
    w.mobys[id].set_state(3, -1);
}

fn seq_of(w: &MobyWorld, id: usize, ctx: &Ctx) -> u8 { super::props::seq(w, id, ctx) }

fn anim_speed(w: &MobyWorld, id: usize, ctx: &Ctx) -> f32 { ctx.host.anim(&w.mobys[id]).map_or(0.0, |a| a.speed) }

/// The frame at the spawner's joint 0 plus [`FRAME_AT`] in its rows, with its rows (`0x317e88`, `0x323c50`,
/// `0x318028`; mode |= 6).
fn place_frame(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let Some(f) = ai::link(&w.mobys[id], FRAME_LINK) else { return };
    let m = &w.mobys[id];
    let j = ctx.host.joint(m, 0).unwrap_or(m.pos);
    let r = m.rows;
    let o = [0, 1, 2].map(|k| FRAME_AT[0] * r[0][k] + FRAME_AT[1] * r[1][k] + FRAME_AT[2] * r[2][k]);
    let x = &mut w.mobys[f];
    x.pos = add(j, o);
    x.rows = r;
    x.rot = angles_of(&r);
    x.mode |= mode::KEEP_ROWS | mode::NO_UPDATE | mode::NO_MATRIX;
    w.rebuild(f, ctx.host);
}

/// 0x0a15 (`0x3d8898`).
pub fn update_spawner(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            for o in RIDERS { w.mobys[id].set_pv_i(o, 0); }
            w.mobys[id].set_state(2, -1);
            if seq_of(w, id, ctx) != 0 { ctx.host.set_anim(&w.mobys[id], 0, 0, 0); }
            ctx.host.set_anim_speed(&w.mobys[id], 0.0);
            let m = &mut w.mobys[id];
            m.pos = add(m.pos, set_len(m.rows[1], 2.0));
            let home = m.pos;
            m.set_pv_v(0, home);
            if w.class(FRAME).is_none() { return; }
            let f = w.create(FRAME);
            let (ud, rows) = (w.mobys[id].update_dist, w.mobys[id].rows);
            let x = &mut w.mobys[f];
            x.mode |= 0x41;
            x.update_dist = ud;
            x.draw_dist = 0;
            x.rows = rows;
            ai::set_link(&mut w.mobys[id], FRAME_LINK, Some(f));
            w.changed = true;
            place_frame(w, id, ctx);
            return;
        }
        s @ (1 | 3) => {
            // Back (3) or home (1): 2 along the second row over 19 ticks, then the sequence's end.
            if w.mobys[id].first() { w.mobys[id].set_pv_s(0x22, SLIDE); }
            let mut t = w.mobys[id].pv_s(0x22) as i32;
            let done = crate::moby::countdown(&mut t);
            w.mobys[id].set_pv_s(0x22, t as i16);
            if !done {
                let k = if s == 1 { 2.0 } else { -2.0 } / SLIDE as f32;
                let m = &mut w.mobys[id];
                m.pos = add(m.pos, set_len(m.rows[1], k));
            }
            if wrapped(w, id, ctx) {
                ctx.host.set_anim_speed(&w.mobys[id], 0.0);
                if s == 1 {
                    let m = &mut w.mobys[id];
                    m.pos = m.pv_v(0);
                    m.set_state(2, -1);
                } else {
                    w.mobys[id].set_state(4, -1);
                }
            }
        }
        4 => {
            w.mobys[id].set_state(5, -1);
            ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            set_seq(w, id, ctx, 1, 0, 0);
            release(w, id, ctx);
        }
        5 => {
            if wrapped(w, id, ctx) {
                w.mobys[id].set_pv_s(0x20, 0x5a);
                ctx.host.set_anim_speed(&w.mobys[id], 0.0);
                w.mobys[id].set_state(6, -1);
            }
        }
        6 => {
            let t = w.mobys[id].pv_s(0x20);
            if t != 0 {
                let mut t = t as i32;
                crate::moby::countdown(&mut t);
                w.mobys[id].set_pv_s(0x20, t as i16);
            } else {
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
                set_seq(w, id, ctx, 2, 0, 0);
                w.mobys[id].set_state(7, -1);
            }
        }
        7 => {
            if wrapped(w, id, ctx) {
                set_seq(w, id, ctx, 3, 0, 0);
                w.mobys[id].set_state(1, -1);
            }
        }
        _ => {}
    }
    if anim_speed(w, id, ctx) != 0.0 { place_frame(w, id, ctx); }
}

/// `0x3d8e68`: the riders out, below and in front of it (`(0, −9, −3.3)` in its rows; the first 2 back in x, the
/// others 1 either way in y), each facing Ratchet and started by its class.
fn release(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let r = m.rows;
    let l = [0.0, -9.0, f32::from_bits(0xc053_3333)];
    let p = add(m.pos, [0, 1, 2].map(|k| l[0] * r[0][k] + l[1] * r[1][k] + l[2] * r[2][k]));
    let at = [[p[0] - 2.0, p[1], p[2]], [p[0], p[1] + 1.0, p[2]], [p[0], p[1] - 1.0, p[2]]];
    for k in 0..3 {
        let Some(f) = rider(w, id, k) else { continue };
        let hero = ctx.hero.pos;
        let x = &mut w.mobys[f];
        x.pos = at[k];
        x.rot = [0.0, 0.0, atan2(hero[0] - x.pos[0], hero[1] - x.pos[1])];
        super::fighters::start(w, f, ctx);
    }
    for o in RIDERS { w.mobys[id].set_pv_i(o, 0); }
}

/// `0x3d8868`: the plate is wanted up.
pub fn raised(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_i(0xbc) != 0 }

/// `0x3d8878` / `0x3d8888`: want it up / down.
pub fn raise(w: &mut MobyWorld, id: usize, up: bool) { w.mobys[id].set_pv_i(0xbc, up as i32); }

/// `0x3d8448`: the plate carries the tie of its group `+0xb0` within 1 of it.
fn take_tie(w: &mut MobyWorld, id: usize) {
    let g = w.mobys[id].pv_i(0xb0);
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.tie_groups.get(g)).cloned() else { return };
    let (cp, cr) = (w.mobys[id].pos, rows_of(w.mobys[id].rot));
    let dot = |a: V3, b: V3| a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
    for i in list {
        let Some(t) = w.ties.get(i as usize) else { continue };
        let tp = [t[3][0], t[3][1], t[3][2]];
        if 1.0 <= dist2(cp, tp) { continue; }
        let d = sub(tp, cp);
        let rows = [0, 1, 2].map(|r| { let v = [t[r][0], t[r][1], t[r][2]]; [dot(v, cr[0]), dot(v, cr[1]), dot(v, cr[2])] });
        let member = Member { kind: Kind::Tie, index: i as usize, offset: [dot(d, cr[0]), dot(d, cr[1]), dot(d, cr[2])], rows };
        w.attached.insert(id, Carrier { members: vec![member], use_rows: false });
        return;
    }
}

/// 0x0a03 (`0x3d8128`).
pub fn update_plate(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].state == 0 {
        let m = &mut w.mobys[id];
        m.set_pv_b(9, 0);
        m.set_pv_b(8, 3);
        m.set_pv_i(0x10, 0);
        m.mode &= !mode::TARGET;
        m.set_state(2, -1);
        m.mode |= mode::KEEP_ROWS;
        let a = TURN_SPEED.to_radians() / 60.0;
        m.set_pv_f(0x80, a);
        take_tie(w, id);
        w.carry(id, ctx.host);
        let m = &mut w.mobys[id];
        m.set_pv_i(0xbc, 0);
        m.set_pv_s(0xc0, -1);
    }
    match w.mobys[id].state {
        1 => {
            hurt(w, id, ctx);
            if w.mobys[id].pv_i(0xbc) == 0 { turn_start(w, id, 4); }
        }
        2 => {
            if w.mobys[id].pv_i(0xbc) != 0 { turn_start(w, id, 3); }
        }
        s @ (3 | 4) => {
            if s == 3 && w.mobys[id].pv_i(0xb4) == TURN_TICKS { ctx.host.play_sound(&w.mobys[id], 1, 0); }
            let a = w.mobys[id].pv_f(0x80);
            let m = &mut w.mobys[id];
            m.rows = turn(&rows_of([a, 0.0, 0.0]), &m.rows);
            m.rot = angles_of(&m.rows);
            w.carry(id, ctx.host);
            hurt(w, id, ctx);
            let mut t = w.mobys[id].pv_i(0xb4);
            crate::moby::countdown(&mut t);
            w.mobys[id].set_pv_i(0xb4, t);
            let h = w.mobys[id].pv_s(0xc0) as i32;
            if s == 3 {
                if t == 0 { w.mobys[id].set_state(1, -1); }
                if !ctx.host.sound_playing(h) {
                    let h = ctx.host.play_sound(&w.mobys[id], 0, 4);
                    w.mobys[id].set_pv_s(0xc0, h as i16);
                }
            } else if t == 0 {
                if h != -1 { ctx.host.stop_sound(h); }
                w.mobys[id].set_pv_s(0xc0, -1);
                w.mobys[id].set_state(2, -1);
            }
        }
        _ => {}
    }
}

fn turn_start(w: &mut MobyWorld, id: usize, s: u8) {
    let m = &mut w.mobys[id];
    m.set_state(s, -1);
    m.set_pv_i(0xb4, TURN_TICKS);
}

/// `0x3d86a8`: what touches it (a sphere of 1.5) is hit for its damage (`+0x14`) and pushed 2 round it: Ratchet
/// only on the ground; a fighter only seen and with it in view, unless its movement block's `+0xc0` is clear; the
/// bosses 0x0a44 / 0x0a4b never.
fn hurt(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let pos = w.mobys[id].pos;
    let Some(sc) = ctx.scene else { return };
    let ids = crate::collision::coll_sphere_mobys(sc, pos, REACH, crate::collision::QueryFlags(0), Some(id));
    let damage = w.mobys[id].pv_b(0x14) as f32;
    let seen = w.mobys[id].drawn != 0;
    for o in ids {
        if Some(o) == ctx.hero_moby {
            if ctx.hero.grounded_ticks == 0 { continue; }
        } else {
            let x = &w.mobys[o];
            if x.mode & ai::AI != 0 {
                let mv = ai::mv(x);
                if mv != 0 && x.pv_s(mv + 0xc0) == 0 { continue; }
                if !seen || x.drawn == 0 || x.o_class == 0x0a44 || x.o_class == 0x0a4b { continue; }
            }
        }
        let d = sub(w.mobys[o].pos, pos);
        let p = set_len([-d[1], d[0], 0.0], PUSH);
        let mut t = HitTemplate::new(damage, Some(id), 0x1_0001, [p[0], p[1], 1.0, 5_627.987]);
        t.kind = 0;
        let at = w.mobys[o].pos;
        w.deliver_hit_any(o, &t, [at[0], at[1], at[2], 1.0]);
    }
}

// ---------------------------------------------------------------------------------------------------------------
// The arena master 0x0b59

/// One wave (0x14 bytes): how many of each fighter (0x09ca, 0x0b22, 0x0c3b, 0x0c3e), the floor plates and 0x0b89s
/// up, the crates lifted (0x01f5 then 0x01ff), the bosses (0x09f8, 0x0a44), the spawners it uses.
#[derive(Clone, Copy, Debug)]
pub struct Wave {
    pub fighters: [i16; 4],
    pub plates: i16,
    pub b89: i16,
    pub crates: [i16; 2],
    pub bosses: [u8; 2],
    pub spawners: u8,
    pub x13: u8,
}

/// One challenge (`$gp+0x2160[n]`): its waves, the challenge whose win opens it (−1 none), the bolts and the XP
/// (×32) its first win pays (each later win a third, at least 500 bolts and 5 XP).
#[derive(Clone, Copy, Debug)]
pub struct Challenge {
    pub needs: i32,
    pub bolts: i32,
    pub xp: i32,
    pub waves: &'static [Wave],
}

/// The fourteen challenges (L02 `0x1b1150` → `0x1b1578..0x1b16c8`; the waves at `0x2a56f8..` and `0x1b16c8..`).
pub static CHALLENGES: [Challenge; 14] = [
    // 0: 6 waves, after challenge -1
    Challenge { needs: -1, bolts: 5000, xp: 1, waves: &[
        Wave { fighters: [5, 0, 0, 0], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 1, x13: 0 },
        Wave { fighters: [0, 5, 0, 0], plates: 0, b89: 0, crates: [1, 0], bosses: [0, 0], spawners: 2, x13: 0 },
        Wave { fighters: [0, 0, 0, 15], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [8, 0, 0, 0], plates: 0, b89: 6, crates: [1, 0], bosses: [0, 0], spawners: 2, x13: 0 },
        Wave { fighters: [0, 0, 4, 0], plates: 6, b89: 6, crates: [1, 3], bosses: [0, 0], spawners: 2, x13: 0 },
        Wave { fighters: [3, 3, 0, 3], plates: 12, b89: 6, crates: [1, 3], bosses: [0, 0], spawners: 3, x13: 0 },
    ]},
    // 1: 5 waves, after challenge 0
    Challenge { needs: 0, bolts: 1500, xp: 1, waves: &[
        Wave { fighters: [4, 4, 0, 0], plates: 6, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 2, x13: 0 },
        Wave { fighters: [0, 0, 0, 16], plates: 6, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 4, 10], plates: 12, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 4, x13: 0 },
        Wave { fighters: [0, 8, 0, 10], plates: 12, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 4, x13: 0 },
        Wave { fighters: [4, 0, 4, 0], plates: 12, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
    ]},
    // 2: 1 waves, after challenge 0
    Challenge { needs: 0, bolts: 4000, xp: 1, waves: &[
        Wave { fighters: [0, 0, 0, 0], plates: 0, b89: 0, crates: [0, 6], bosses: [1, 0], spawners: 0, x13: 0 },
    ]},
    // 3: 2 waves, after challenge 0
    Challenge { needs: 0, bolts: 1500, xp: 1, waves: &[
        Wave { fighters: [5, 5, 0, 5], plates: 0, b89: 0, crates: [0, 6], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 5, 10], plates: 0, b89: 0, crates: [0, 6], bosses: [0, 0], spawners: 6, x13: 0 },
    ]},
    // 4: 1 waves, after challenge 2
    Challenge { needs: 2, bolts: 5000, xp: 1, waves: &[
        Wave { fighters: [0, 0, 0, 0], plates: 0, b89: 0, crates: [0, 6], bosses: [0, 1], spawners: 0, x13: 0 },
    ]},
    // 5: 4 waves, after challenge 3
    Challenge { needs: 3, bolts: 2000, xp: 1, waves: &[
        Wave { fighters: [5, 0, 0, 10], plates: 0, b89: 0, crates: [0, 6], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 5, 0, 10], plates: 0, b89: 0, crates: [0, 6], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [2, 3, 0, 10], plates: 0, b89: 0, crates: [0, 6], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 5, 10], plates: 0, b89: 0, crates: [0, 6], bosses: [0, 0], spawners: 6, x13: 0 },
    ]},
    // 6: 10 waves, after challenge 4
    Challenge { needs: 4, bolts: 3000, xp: 1, waves: &[
        Wave { fighters: [8, 0, 0, 0], plates: 0, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [4, 4, 0, 0], plates: 6, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 4, x13: 0 },
        Wave { fighters: [0, 4, 2, 6], plates: 6, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 4, x13: 0 },
        Wave { fighters: [4, 4, 0, 0], plates: 6, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 4, x13: 0 },
        Wave { fighters: [0, 8, 0, 6], plates: 6, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 4, x13: 0 },
        Wave { fighters: [2, 4, 2, 0], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 4, x13: 0 },
        Wave { fighters: [0, 8, 0, 0], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 2, x13: 0 },
        Wave { fighters: [0, 0, 0, 16], plates: 6, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [2, 0, 4, 10], plates: 6, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 4, x13: 0 },
        Wave { fighters: [0, 0, 0, 0], plates: 0, b89: 0, crates: [0, 3], bosses: [1, 0], spawners: 0, x13: 0 },
    ]},
    // 7: 15 waves, after challenge 1
    Challenge { needs: 1, bolts: 1500, xp: 1, waves: &[
        Wave { fighters: [6, 0, 0, 0], plates: 0, b89: 0, crates: [6, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 6, 0, 0], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 4, 0], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 0, 10], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [4, 2, 0, 0], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 4, 2, 0], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [4, 0, 2, 0], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 0, 16], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [3, 3, 2, 0], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [6, 0, 0, 8], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 6, 0, 8], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 4, 8], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [4, 2, 0, 8], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 4, 2, 8], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [4, 0, 2, 8], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
    ]},
    // 8: 4 waves, after challenge 7
    Challenge { needs: 7, bolts: 2000, xp: 1, waves: &[
        Wave { fighters: [0, 5, 0, 0], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [3, 3, 0, 3], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 6, 0], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [4, 3, 0, 6], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
    ]},
    // 9: 1 waves, after challenge 6
    Challenge { needs: 6, bolts: 6000, xp: 1, waves: &[
        Wave { fighters: [0, 0, 0, 0], plates: 12, b89: 0, crates: [0, 12], bosses: [1, 1], spawners: 0, x13: 0 },
    ]},
    // 10: 20 waves, after challenge 5
    Challenge { needs: 5, bolts: 2000, xp: 1, waves: &[
        Wave { fighters: [8, 0, 0, 0], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [4, 4, 0, 0], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 6, 10], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 8, 0, 0], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [4, 0, 4, 10], plates: 6, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [2, 2, 2, 0], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 8, 0, 0], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 0, 16], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [2, 2, 4, 10], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [6, 0, 2, 0], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 4, 4, 0], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [4, 0, 0, 12], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 6, 0, 0], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [4, 0, 4, 10], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 4, 2, 10], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [4, 4, 0, 0], plates: 0, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 8, 0, 10], plates: 6, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [6, 0, 3, 0], plates: 6, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 8, 0, 0], plates: 12, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 0, 16], plates: 12, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
    ]},
    // 11: 16 waves, after challenge 10
    Challenge { needs: 10, bolts: 7000, xp: 1, waves: &[
        Wave { fighters: [4, 0, 4, 0], plates: 6, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 0, 16], plates: 6, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 2, 4, 0], plates: 6, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 6, 0, 8], plates: 6, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [4, 4, 0, 0], plates: 6, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 0, 0], plates: 0, b89: 0, crates: [0, 6], bosses: [1, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 4, 0, 12], plates: 12, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 4, 12], plates: 12, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [8, 0, 0, 0], plates: 12, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 8, 0, 0], plates: 12, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 0, 0], plates: 0, b89: 0, crates: [0, 6], bosses: [0, 1], spawners: 6, x13: 0 },
        Wave { fighters: [4, 4, 0, 0], plates: 6, b89: 6, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 4, 12], plates: 12, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 0, 16], plates: 12, b89: 0, crates: [0, 3], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 0, 8], plates: 12, b89: 0, crates: [0, 6], bosses: [1, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 0, 8], plates: 12, b89: 0, crates: [0, 6], bosses: [0, 1], spawners: 6, x13: 0 },
    ]},
    // 12: 1 waves, after challenge 0
    Challenge { needs: 0, bolts: 6000, xp: 1, waves: &[
        Wave { fighters: [0, 0, 0, 0], plates: 0, b89: 0, crates: [0, 0], bosses: [1, 0], spawners: 0, x13: 0 },
    ]},
    // 13: 9 waves, after challenge 12
    Challenge { needs: 12, bolts: 9000, xp: 1, waves: &[
        Wave { fighters: [0, 0, 0, 16], plates: 12, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 4, 0], plates: 0, b89: 6, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [6, 0, 0, 8], plates: 6, b89: 6, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 8, 0, 0], plates: 6, b89: 6, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [4, 0, 4, 4], plates: 12, b89: 6, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 4, 4, 0], plates: 12, b89: 6, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [3, 0, 0, 12], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [2, 2, 2, 0], plates: 0, b89: 0, crates: [0, 0], bosses: [0, 0], spawners: 6, x13: 0 },
        Wave { fighters: [0, 0, 0, 16], plates: 12, b89: 0, crates: [0, 0], bosses: [1, 0], spawners: 6, x13: 0 },
    ]},
];

/// `0x3fb110(n)`: challenge `n`'s win count in the flags (`0x1395b8 + k`).
pub fn win_flag(n: i32) -> Option<usize> {
    Some(match n {
        0 => 0x31, 1 => 0x34, 2 => 0x32, 3 => 0x33, 4 => 0x35, 5 => 0x36, 6 => 0x37, 7 => 0x38, 8 => 0x91, 9 => 0x39,
        10 => 0x3a, 11 => 0x3b, 12 => 0x92, 13 => 0x93,
        _ => return None,
    })
}

/// `0x3ff768(n)`: challenge `n`'s name.
pub fn challenge_name(n: i32) -> u32 {
    [0x2dde, 0x2de4, 0x2ddf, 0x2de3, 0x2de0, 0x2de5, 0x2de2, 0x2e04, 0x30d6, 0x2de1, 0x2de6, 0x2de7, 0x30d7, 0x30d8].get(n as usize).copied().unwrap_or(0x2dde)
}

fn wins(game: &crate::game::GameState, n: i32) -> u8 { win_flag(n).and_then(|k| game.flags.get(k).copied()).unwrap_or(0) }

/// The challenges' weapons: 7 is played with item 0x1e (300 shots), 8 with 0x2a (8).
const WEAPON_7: i32 = 0x1e;
const WEAPON_8: i32 = 0x2a;

/// Whether challenge `n` is on the menu (`0x3fee58`'s test): the challenge before it won; from 12 the challenge mode;
/// 7 and 8 their weapons (`0x264a74[0x139586 / 0x139592 · 0xe0]`: here the items had, inferred).
pub fn on_menu(game: &crate::game::GameState, n: i32) -> bool {
    let Some(c) = CHALLENGES.get(n as usize) else { return false };
    // The port's, until the bosses 0x09f8 / 0x0a44 are: a challenge with a boss wave could never end.
    if c.waves.iter().any(|w| w.bosses != [0, 0]) { return false; }
    if 0 < n && wins(game, c.needs) == 0 { return false; }
    if 0xb < n && !game.challenge { return false; }
    let had = |i: i32| game.items.get(i as usize).is_some_and(|&b| b != 0);
    if n == 7 && !had(WEAPON_7) { return false; }
    if n == 8 && !had(WEAPON_8) { return false; }
    true
}

/// `0x3fe398(n)`: the bolts a win of `n` pays — its prize, a third less for each win before (×0.3), at least 500; ten
/// times that in the challenge mode.
pub fn prize(game: &crate::game::GameState, n: i32) -> i32 {
    let base = CHALLENGES.get(n as usize).map_or(0, |c| c.bolts) as f32;
    let mut f = base;
    for _ in 0..wins(game, n) { f *= 0.3; }
    let mut v = base;
    if f <= v { v = f.max(500.0); }
    if game.challenge { v *= 10.0; }
    v as i32
}

/// `0x3fe4a8(n)`: the XP a win of `n` pays (its XP, a third for each win before, at least 5; ×32).
fn xp(game: &crate::game::GameState, n: i32) -> i32 {
    let base = CHALLENGES.get(n as usize).map_or(0, |c| c.xp) as f32;
    let mut f = base;
    for _ in 0..wins(game, n) { f *= 0.33333; }
    let mut v = base;
    if f <= v { v = f.max(5.0); }
    (v as i32) << 5
}

/// What the engine draws of the challenge menu (`0x3ff808`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Menu {
    /// The selection, its prize (none for the first challenge not won yet) and wins.
    pub selected: i32,
    pub prize: Option<i32>,
    pub wins: u8,
    /// A challenge before / after it on the menu.
    pub prev: bool,
    pub next: bool,
}

/// The master's pvars (the module's docs).
mod pv {
    pub const ENEMIES: usize = 0x40;
    pub const SPAWNERS: usize = 0x44;
    pub const B89: usize = 0x48;
    pub const PLATES: usize = 0x4c;
    pub const TURNS: usize = 0x50;
    pub const ALIVE: usize = 0x60;
    pub const LIFE: usize = 0x64;
    pub const HEALTH: usize = 0x68;
    pub const SCORE: usize = 0x6c;
    pub const DESK: usize = 0x70;
    pub const DESK_WON: usize = 0x74;
    pub const BACK: usize = 0x78;
    pub const ARENA: usize = 0x7c;
    pub const KEEP: usize = 0x80;
    pub const MIDDLE: usize = 0x88;
    pub const ENTRY: usize = 0x8c;
    pub const SPAWNED: usize = 0x90;
    pub const GATES: usize = 0xa8;
    pub const CRATES_B: usize = 0xc4;
    pub const CRATES_A: usize = 0xc8;
    pub const LIFTS: usize = 0xcc;
    pub const BOSS: usize = 0xd0;
    pub const TICKS: usize = 0xd4;
    pub const GATE: usize = 0xd8;
    pub const INTRO: usize = 0x108;
    pub const FIRST_WIN: usize = 0x10c;
    pub const PRIZE: usize = 0x110;
    pub const SELECTED: usize = 0x118;
    pub const REPEAT: usize = 0x11a;
    pub const RING: usize = 0x11c;
    pub const WINDOW: usize = 0x11e;
    pub const CHALLENGE: usize = 0x120;
    pub const WAVE: usize = 0x122;
    pub const SAID: usize = 0x1d0;
}

/// `$gp+0x2198`: the turns to attack each fighter class gets at a challenge's start.
const TURNS: [i32; 4] = [3, 4, 0, 5];
/// `$gp+0x21a0` / `+0x21b0`: where the menu's camera comes to rest and its angles; `+0x21c0` / `+0x21d0` where it
/// starts; `+0x21e0` its ticks.
const MENU_EYE: V3 = [500.0, 571.5, 509.5];
const MENU_ANGLES: V3 = [0.0, 0.0, -std::f32::consts::FRAC_PI_2];
const MENU_START: V3 = [508.0, 614.0, 513.0];
const MENU_TICKS: i32 = 0xb4;
/// `$gp+0x21e4`: the score's window (ticks); `+0x21e8..+0x21f0` its weights (damage, kills, health lost).
const WINDOW: i16 = 600;
const WEIGHTS: [f32; 3] = [0.2, 1.0, -2.0];
/// The desk's prompt text (`0x32cb88(4, 0x2dd2, 1)`), its owner.
const DESK_TEXT: u32 = 0x2dd2;
const DESK_OWNER: i32 = 4;
/// The fail messages: "… kills" (time out), out of shots (7 / 8), hurt (12 / 13).
const FAIL_KILLS: (u32, u32) = (0x30ff, 0x2df7);
const FAIL_SHOTS_7: u32 = 0x3101;
const FAIL_SHOTS_8: u32 = 0x3102;
const FAIL_HURT: u32 = 0x3100;

fn group(w: &MobyWorld, id: usize, off: usize) -> Vec<usize> {
    usize::try_from(w.mobys[id].pv_i(off)).ok().filter(|&g| g != 0xff).and_then(|g| w.groups.get(g)).cloned().unwrap_or_default()
}

/// The placed moby of instance `i` (the master keeps moby indices).
fn placed(w: &MobyWorld, i: i32) -> Option<usize> { usize::try_from(i).ok().and_then(|i| w.mobys.iter().position(|m| m.instance == Some(i))) }

fn cuboid_at(ctx: &Ctx, w: &MobyWorld, id: usize, off: usize) -> Option<(V3, V3)> { ctx.level.cuboid_place(w.mobys[id].pv_i(off)) }

/// `0x401d98(m, n)`: line `n` of the announcer (`+0x130` / `+0x180`), said once a challenge (`+0x1d0`). Not ported:
/// the speech itself (`0x1a63cc` = id + 2000).
fn say(w: &mut MobyWorld, id: usize, n: i32) {
    if !(0..=0x26).contains(&n) { return; }
    w.mobys[id].set_pv_s(pv::SAID + 2 * n as usize, 1);
}

/// `0x3fc530`: the challenge and wave none, the counts cleared, state 1.
fn reset_master(w: &mut MobyWorld, id: usize) {
    let m = &mut w.mobys[id];
    m.set_pv_s(pv::WAVE, -1);
    m.set_pv_s(pv::CHALLENGE, -1);
    m.set_pv_s(pv::SELECTED, 0);
    for k in 0..6 { m.set_pv_i(pv::SPAWNED + 4 * k, 0); }
    m.set_pv_i(pv::GATE, 0);
    m.set_pv_i(pv::BOSS, 0);
    m.set_state(1, -1);
}

/// `0x3fd130`: every fighter and boss back to the pool.
fn pool_all(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    for f in group(w, id, pv::ENEMIES) {
        if w.mobys[f].state >= 0x80 { continue; }
        match w.mobys[f].o_class {
            super::fighters::C9CA => super::fighters::pool_09ca(w, f, ctx),
            super::fighters::C3B | super::fighters::C3E | super::fighters::B22 => super::fighters::pool(w, f, ctx),
            _ => {}
        }
    }
}

/// `0x3fd808` / `0x3fdab8(m, n)`: `n` of the group's 0x0b89s / floor plates up (the first ones down raised, or the
/// last ones up lowered).
fn raise_group(w: &mut MobyWorld, id: usize, off: usize, n: i32) {
    let g = group(w, id, off);
    let up = |w: &MobyWorld, p: usize| if off == pv::PLATES { raised(w, p) } else { column_on(w, p) };
    let set = |w: &mut MobyWorld, p: usize, on: bool| if off == pv::PLATES { raise(w, p, on) } else { column_set(w, p, on) };
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

/// `0x3fc5a0`: the fighters out now (not pooled) and their health, the time (health), the score's window restarted.
fn count_out(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let (mut n, mut life) = (0, 0.0);
    for f in group(w, id, pv::ENEMIES) {
        let x = &w.mobys[f];
        if x.state >= 0x80 || x.state == 1 || x.update.is_none() { continue; }
        if is_fighter_or_boss(x.o_class) {
            n += 1;
            life += x.pv_f(0x20);
        }
    }
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::ALIVE, n);
    m.set_pv_f(pv::LIFE, life);
    m.set_pv_s(pv::RING, 0);
    m.set_pv_f(pv::HEALTH, ctx.hero.hp as f32);
    m.set_pv_s(pv::WINDOW, WINDOW);
}

fn is_fighter_or_boss(c: i16) -> bool { super::fighters::FIGHTERS_AND_BOSSES.contains(&c) }

/// `0x3fc6c0`: the score — over a ring of four windows of 600 ticks, the damage done, the kills and the health lost,
/// weighted 1/1..1/4 newest first, then `0.2·damage + kills − 2·health lost` (`+0x6c`). Not ported: the crowd's gasp
/// when Ratchet is hurt (the level's sound 0x17, `0x367ff0`), the announcer's remarks (`0x401e68`).
fn score(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (mut n, mut life) = (0, 0.0);
    for f in group(w, id, pv::ENEMIES) {
        let x = &w.mobys[f];
        // The port's: a class without its update yet (the bosses) is no fighter out.
        if x.state >= 0x80 || x.state == 1 || x.update.is_none() || !is_fighter_or_boss(x.o_class) { continue; }
        life += x.pv_f(0x20);
        if 0.0 < x.pv_f(0x20) || matches!(x.o_class, 0x09f8 | 0x0a44) { n += 1; }
    }
    let hp = ctx.hero.hp as f32;
    let m = &mut w.mobys[id];
    let r = (m.pv_s(pv::RING) as usize & 3) * 0x10;
    if n < m.pv_i(pv::ALIVE) { m.set_pv_i(r, m.pv_i(r) + m.pv_i(pv::ALIVE) - n); }
    if life < m.pv_f(pv::LIFE) { m.set_pv_f(r + 4, m.pv_f(r + 4) + m.pv_f(pv::LIFE) - life); }
    if hp < m.pv_f(pv::HEALTH) { m.set_pv_f(r + 8, m.pv_f(r + 8) + m.pv_f(pv::HEALTH) - hp); }
    m.set_pv_f(pv::LIFE, life);
    m.set_pv_i(pv::ALIVE, n);
    m.set_pv_f(pv::HEALTH, hp);
    let (mut dmg, mut hurt, mut kills) = (0.0, 0.0, 0.0);
    let ring = m.pv_s(pv::RING) as i32;
    for k in (1..=4).rev() {
        let f = 1.0 / (5.0 - k as f32);
        let o = ((ring + k).rem_euclid(4) as usize) * 0x10;
        dmg += f * m.pv_f(o + 4);
        hurt += f * m.pv_f(o + 8);
        kills += f * m.pv_i(o) as f32;
    }
    m.set_pv_f(pv::SCORE, WEIGHTS[0] * dmg + WEIGHTS[1] * kills + WEIGHTS[2] * hurt);
    let mut t = m.pv_s(pv::WINDOW) as i32;
    if crate::moby::countdown(&mut t) {
        let r = ((m.pv_s(pv::RING) + 1) & 3) as i16;
        m.set_pv_s(pv::RING, r);
        let o = r as usize * 0x10;
        m.set_pv_i(o, 0);
        m.set_pv_f(o + 4, 0.0);
        m.set_pv_f(o + 8, 0.0);
        t = WINDOW as i32;
    }
    m.set_pv_s(pv::WINDOW, t as i16);
}

/// `0x3fe190`: the wave is over — every fighter it calls for sent out, its bosses out, none left.
fn wave_over(w: &MobyWorld, id: usize, wave: &Wave) -> bool {
    let m = &w.mobys[id];
    let mut done = (0..4).all(|k| m.pv_i(pv::SPAWNED + 4 * k) == wave.fighters[k] as i32);
    if wave.bosses[0] != 0 { done &= m.pv_i(pv::SPAWNED + 0x10) != 0; }
    if wave.bosses[1] != 0 { done &= m.pv_i(pv::SPAWNED + 0x14) != 0; }
    done && m.pv_i(pv::ALIVE) == 0
}

fn wave_of(w: &MobyWorld, id: usize) -> Option<&'static Wave> {
    let m = &w.mobys[id];
    CHALLENGES.get(m.pv_s(pv::CHALLENGE) as usize).and_then(|c| c.waves.get(m.pv_s(pv::WAVE) as usize))
}

fn gate(w: &MobyWorld, id: usize) -> Option<usize> {
    let k = w.mobys[id].pv_i(pv::GATE).clamp(0, 9) as usize;
    ai::link(&w.mobys[id], pv::GATES + 4 * k)
}

/// `0x3fd070`: once every spawner taken this round has let its fighters out, the round starts again.
fn gates_reset(w: &mut MobyWorld, id: usize) {
    let n = w.mobys[id].pv_i(pv::GATE).clamp(0, 9) as usize;
    for k in 0..=n {
        if let Some(g) = ai::link(&w.mobys[id], pv::GATES + 4 * k) {
            if !ready(w, g) { return; }
        }
    }
    for k in 0..=n { w.mobys[id].set_pv_i(pv::GATES + 4 * k, 0); }
    w.mobys[id].set_pv_i(pv::GATE, 0);
}

/// `0x3fcd80`: the round's spawner — the current one, or once it has let its fighters out the next (up to the wave's
/// count): one of the group closed and more than 25 from Ratchet (its joint 0), else any closed one more than 30.
fn gate_pick(w: &mut MobyWorld, id: usize, ctx: &Ctx, wave: &Wave) {
    let mut pick = gate(w, id).is_none();
    if let Some(g) = gate(w, id) {
        if !ready(w, g) && w.mobys[id].pv_i(pv::GATE) < wave.spawners as i32 - 1 {
            pick = true;
            let n = w.mobys[id].pv_i(pv::GATE) + 1;
            w.mobys[id].set_pv_i(pv::GATE, n);
        }
    }
    if !pick { return; }
    let k = w.mobys[id].pv_i(pv::GATE).clamp(0, 9) as usize;
    let gs = group(w, id, pv::SPAWNERS);
    let far = |w: &MobyWorld, g: usize, r: f32| { let p = ctx.host.joint(&w.mobys[g], 0).unwrap_or(w.mobys[g].pos); w.mobys[g].drawn != 0 && r < dist(p, ctx.hero.pos) };
    let first = gs.iter().copied().find(|&g| far(w, g, 25.0) && ready(w, g));
    let pickd = first.or_else(|| gs.iter().copied().find(|&g| ready(w, g) && 30.0 < dist(w.mobys[g].pos, ctx.hero.pos)));
    if let Some(g) = pickd { ai::set_link(&mut w.mobys[id], pv::GATES + 4 * k, Some(g)); }
}

/// `0x3fca78(m, class)` → `0x3fcc60`: a pooled fighter of `class` into the round's spawner (its life 1 until it is let
/// out), counted.
fn send(w: &mut MobyWorld, id: usize, class: i16) -> bool {
    let Some(g) = gate(w, id).filter(|&g| ready(w, g)) else { return false };
    let Some(f) = group(w, id, pv::ENEMIES).into_iter().find(|&f| { let x = &w.mobys[f]; x.state == 1 && x.o_class == class }) else { return false };
    w.mobys[f].parent = Some(id);
    attach(w, g, f);
    let k = FIGHTERS.iter().position(|&c| c == class).unwrap_or(0);
    let m = &mut w.mobys[id];
    m.set_pv_i(pv::SPAWNED + 4 * k, m.pv_i(pv::SPAWNED + 4 * k) + 1);
    w.mobys[f].set_pv_f(0x20, 1.0);
    true
}

/// `0x3fcff8`: each spawner of the round holding fighters opened.
fn open_gates(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let n = w.mobys[id].pv_i(pv::GATE).clamp(0, 9) as usize;
    for k in 0..=n {
        if let Some(g) = ai::link(&w.mobys[id], pv::GATES + 4 * k) {
            if loaded(w, g) { open(w, g, ctx); }
        }
    }
}

/// `0x3fd298` (every fourth frame): the gladiators' and bladed robots' turns given back, then the two of them nearest
/// Ratchet (out, not pooled) given one each.
fn deal_turns(w: &mut MobyWorld, id: usize, ctx: &Ctx) {
    let g = group(w, id, pv::ENEMIES);
    let turn_of = |c: i16| match c { super::fighters::C9CA => Some((0x2f6, 0x50)), super::fighters::B22 => Some((0x4ea, 0x54)), _ => None };
    for &f in &g {
        if let Some((o, k)) = turn_of(w.mobys[f].o_class) {
            if w.mobys[f].pv_s(o) == 1 {
                w.mobys[f].set_pv_s(o, 0);
                let m = &mut w.mobys[id];
                m.set_pv_i(k, m.pv_i(k) + 1);
            }
        }
    }
    for _ in 0..2 {
        let mut best: Option<(usize, f32)> = None;
        for &f in &g {
            let x = &w.mobys[f];
            let Some((o, _)) = turn_of(x.o_class) else { continue };
            if x.state >= 0x80 || x.state < 4 || x.pv_s(o) == 1 { continue; }
            let d = dist(x.pos, ctx.hero.pos);
            if best.is_none_or(|(_, b)| d < b) { best = Some((f, d)); }
        }
        let Some((f, _)) = best else { continue };
        let (o, k) = turn_of(w.mobys[f].o_class).unwrap();
        w.mobys[f].set_pv_s(o, 1);
        let m = &mut w.mobys[id];
        m.set_pv_i(k, m.pv_i(k) - 1);
    }
}

/// `0x3fdd68`: a fighter out of the arena (outside the cuboids `+0x80` and `+0x7c`) dies; Ratchet out of `+0x7c` dies.
fn keep_in(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (keep, arena) = (w.mobys[id].pv_i(pv::KEEP), w.mobys[id].pv_i(pv::ARENA));
    for f in group(w, id, pv::ENEMIES) {
        let x = &w.mobys[f];
        if x.state >= 0x80 || x.state < 3 || x.mode & mode::NO_UPDATE != 0 { continue; }
        if ctx.level.in_cylinder(keep, x.pos) || ctx.level.in_cylinder(arena, x.pos) { continue; }
        match x.o_class {
            super::fighters::C9CA => super::fighters::die_09ca_pub(w, f, ctx),
            super::fighters::B22 | super::fighters::C3B | super::fighters::C3E => {
                crate::moby::bolts::drop(w, f, 0, -1, ctx);
                super::fighters::pool(w, f, ctx);
            }
            _ => {}
        }
    }
    if !ctx.level.in_cylinder(arena, ctx.hero.pos) { ctx.hero.die(); }
}

/// The overview camera (`0x3fed10`): the script camera at `MENU_START`, moving to the menu's place over 180 ticks.
fn menu_camera(ctx: &mut Ctx) {
    ctx.game.cameras.camera_script(MENU_START, MENU_ANGLES, crate::script_camera::MODE_MOVE, MENU_TICKS, false, ctx.hero);
    ctx.game.cameras.camera_script_targets(Some(MENU_EYE), Some(MENU_ANGLES));
}

/// `0x3fe020`: back to the resort — the announcer's goodbye, state 1, chunk 0, Ratchet at the cuboid `+0x78` (before
/// the first win, facing 2.16) or the desk's `+0x74` (facing −1.59), the checkpoint, the fighters pooled, the plates
/// and 0x0b89s down.
fn leave(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let n = 0x25 + ctx.game.rng.randi(2);
    say(w, id, n);
    w.mobys[id].set_state(1, -1);
    ctx.game.switch_chunk(0);
    let m = &w.mobys[id];
    let place = if wins(ctx.game, 0) == 0 {
        ctx.level.cuboid_place(m.pv_i(pv::BACK)).map(|c| (c.0, f32::from_bits(0x400a_3d71)))
    } else {
        usize::try_from(m.pv_i(pv::DESK_WON)).ok().and_then(|i| ctx.level.cylinders.get(i)).map(|c| ([c.matrix[3][0], c.matrix[3][1], c.matrix[3][2]], f32::from_bits(0xbfcb_851f)))
    };
    if let Some((p, yaw)) = place {
        ctx.hero.teleport(p, [0.0, 0.0, yaw]);
        ctx.hero.request_state(0, true);
        let (p, r) = (ctx.hero.pos, ctx.hero.rot);
        ctx.game.save_checkpoint(p, r);
    }
    pool_all(w, id, ctx);
    raise_group(w, id, pv::B89, 0);
    raise_group(w, id, pv::PLATES, 0);
    ctx.game.arena_menu = None;
}

/// `0x3fe240(m, n)`: challenge `n` won — the first win of the first challenge (its own line), else the bolts and
/// XP (`0x3fe398`, `0x3fe4a8`); its win counted.
fn reward(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, n: i32) {
    if n == 0 && wins(ctx.game, 0) == 0 {
        say(w, id, 7);
    } else {
        let line = match n { 6 => 8, 0xb => 9, _ => 2 + ctx.game.rng.randi(5) };
        say(w, id, line);
        ctx.game.xp += xp(ctx.game, n);
        let b = prize(ctx.game, n);
        w.mobys[id].set_pv_i(pv::PRIZE, b);
        ctx.game.bolt_bank += b;
        ctx.game.show_bolts = Some(0x168);
    }
    if let Some(k) = win_flag(n) {
        if let Some(f) = ctx.game.flags.get_mut(k) { *f = f.saturating_add(1); }
    }
}

/// `0x3fe7c8`: the wave's crates lifted — for each kind (0x01f5, then 0x01ff) as many as it asks for, a broken one
/// put on a free lift (0x0ccc, one in six skipped a round; [`super::fighters::lift`]).
fn lift_crates(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, wave: &Wave) {
    for (want, from) in [(wave.crates[0], pv::CRATES_A), (wave.crates[1], pv::CRATES_B)] {
        if want == 0 { continue; }
        let crates = group(w, id, from);
        let mut free: Vec<usize> = crates.iter().copied().filter(|&c| w.mobys[c].state != 6).collect();
        let mut out = crates.len() as i32 - free.len() as i32;
        if free.is_empty() || want as i32 <= out { continue; }
        let lifts = group(w, id, pv::LIFTS);
        let mut tries = 0;
        'round: while out < want as i32 {
            for &l in &lifts {
                tries += 1;
                if 999 < tries { break 'round; }
                let Some(c) = free.last().copied() else { break 'round };
                if !lift_free(w, l) || ctx.game.rng.randi(6) != 0 { continue; }
                free.pop();
                out += 1;
                lift(w, l, c, 3.0 / 60.0, 720.0_f32.to_radians() / 60.0);
                if want as i32 <= out { break 'round; }
            }
            if lifts.is_empty() { break; }
        }
    }
}

/// `0x3fdab8`/`0x3fd808` for a wave and its crates (the start of a wave).
fn wave_start(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let Some(wave) = wave_of(w, id) else { return };
    lift_crates(w, id, ctx, wave);
    raise_group(w, id, pv::B89, wave.b89 as i32);
    raise_group(w, id, pv::PLATES, wave.plates as i32);
}

fn fail_kills(w: &MobyWorld, id: usize) -> i32 {
    let m = &w.mobys[id];
    let sent: i32 = (0..6).map(|k| m.pv_i(pv::SPAWNED + 4 * k)).sum();
    m.pv_s(pv::WAVE) as i32 * 0xf + sent - m.pv_i(pv::ALIVE)
}

/// The challenge lost (its message shown, `0x313098`): state 10.
fn lose(w: &mut MobyWorld, id: usize) { w.mobys[id].set_state(10, -1); }

/// 0x0b59 (`0x3fb1b0`), Maktar's arena master (module docs).
pub fn update_master(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            reset_master(w, id);
            ctx.game.arena_menu = None;
        }
        1 => {
            // The desk: Ratchet already in the arena (a restart there), the overview; at the desk (the sphere, or once
            // the first challenge is won the cuboid too) the prompt and △ to enter.
            let hero = ctx.hero.pos;
            let m = &w.mobys[id];
            if ctx.level.in_cylinder(m.pv_i(pv::ARENA), hero) {
                w.mobys[id].set_state(4, -1);
            } else if ctx.level.in_cuboid(m.pv_i(pv::DESK), hero) || (wins(ctx.game, 0) != 0 && ctx.level.in_cylinder(m.pv_i(pv::DESK_WON), hero)) {
                ctx.game.prompt.request(DESK_OWNER, DESK_TEXT, 1);
                if ctx.hero.items.pressed & crate::pad::button::TRIANGLE != 0 && ctx.game.prompt.owner == DESK_OWNER {
                    w.mobys[id].set_state(2, -1);
                    w.mobys[id].timer = 0;
                }
            }
        }
        2 => enter(w, id, ctx),
        s @ (4 | 5) => {
            if s == 5 && w.mobys[id].state_ticks < 0xb4 {
                raise_group(w, id, pv::B89, 0);
                raise_group(w, id, pv::PLATES, 0);
                keep_in(w, id, ctx);
            } else {
                if w.mobys[id].first() {
                    if s == 5 {
                        let n = w.mobys[id].pv_s(pv::CHALLENGE) as i32;
                        reward(w, id, ctx, n);
                    }
                    pool_all(w, id, ctx);
                    raise_group(w, id, pv::B89, 0);
                    raise_group(w, id, pv::PLATES, 0);
                    if let Some((p, _)) = cuboid_at(ctx, w, id, pv::MIDDLE) {
                        ctx.hero.teleport(p, [0.0; 3]);
                        ctx.hero.request_state(0x6f, true);
                        let (p, r) = (ctx.hero.pos, ctx.hero.rot);
                        ctx.game.save_checkpoint(p, r);
                    }
                    menu_camera(ctx);
                }
                // `0x3fed68`: the camera come to rest, the menu (4) or the results (5).
                let (eye, _) = ctx.game.cameras.view_euler();
                if crate::hero::physics::dist2(eye, MENU_EYE) < 0.0001 {
                    w.mobys[id].set_state(if s == 4 { 7 } else { 6 }, -1);
                } else if !ctx.game.cameras.script_current() {
                    menu_camera(ctx);
                }
            }
        }
        6 => {
            // After a win: ✕ the menu at the next challenge, △ back to the resort.
            if ctx.hero.state != 0x6f { ctx.hero.request_state(0x6f, true); }
            let pressed = ctx.hero.items.pressed;
            if pressed & crate::pad::button::CROSS != 0 {
                w.mobys[id].set_state(7, -1);
                let sel = w.mobys[id].pv_s(pv::SELECTED) as i32;
                if let Some(n) = (sel + 1..0xe).find(|&n| on_menu(ctx.game, n)) { w.mobys[id].set_pv_s(pv::SELECTED, n as i16); }
            } else if pressed & crate::pad::button::TRIANGLE != 0 {
                leave(w, id, ctx);
            }
        }
        7 => menu(w, id, ctx),
        8 => fight(w, id, ctx),
        10 => {
            ctx.game.arena_menu = None;
            if 0x167 < w.mobys[id].state_ticks {
                w.mobys[id].set_state(4, -1);
            } else {
                raise_group(w, id, pv::B89, 0);
                raise_group(w, id, pv::PLATES, 0);
                keep_in(w, id, ctx);
            }
        }
        0xb => {
            // The first win of the first challenge: its scene (the trigger `+0x10c` hooked after 3 seconds), then the
            // reward and the overview.
            if w.mobys[id].first() {
                raise_group(w, id, pv::B89, 0);
                raise_group(w, id, pv::PLATES, 0);
                w.mobys[id].timer = 0;
            }
            if w.mobys[id].timer == 0 {
                if 0xb3 < w.mobys[id].state_ticks {
                    if let Some(t) = placed(w, w.mobys[id].pv_i(pv::FIRST_WIN)) { w.mobys[t].once |= 4; }
                    w.mobys[id].timer = 1;
                } else {
                    keep_in(w, id, ctx);
                }
            } else if 0 < ctx.hero.hp && !(ctx.game.scenes.mode == crate::scene::SCENE) && 0xf0 < w.mobys[id].state_ticks {
                w.mobys[id].set_state(5, -1);
                w.mobys[id].once &= !1;
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    if m.state_ticks != -1 { m.state_ticks = m.state_ticks.wrapping_add(1); }
}

/// State 2: through the wipe into the arena — chunk 1, Ratchet at the entry cuboid `+0x8c`, the checkpoint, the
/// camera behind him; the first time the announcer's welcome (the trigger `+0x108` hooked) and straight into the first
/// challenge; else the overview once he has walked to the middle.
fn enter(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].first() {
        w.mobys[id].timer = 0;
        w.mobys[id].state_ticks = 0;
    }
    match w.mobys[id].timer {
        0 => {
            if w.mobys[id].state_ticks < 0x1f { return; }
            ctx.game.switch_chunk(1);
            if let Some((p, r)) = cuboid_at(ctx, w, id, pv::ENTRY) {
                ctx.hero.teleport(p, r);
                ctx.hero.request_state(0x6f, true);
                let (p, r) = (ctx.hero.pos, ctx.hero.rot);
                ctx.game.save_checkpoint(p, r);
                ctx.game.cameras.camera_script([p[0], p[1], p[2] + 2.0], r, crate::script_camera::MODE_SNAP, 0, false, ctx.hero);
            }
            w.mobys[id].timer = 2;
        }
        2 => {
            if ctx.game.flags[5] == 0 {
                ctx.game.flags[5] = 1;
                w.mobys[id].timer = 1;
                if let Some(t) = placed(w, w.mobys[id].pv_i(pv::INTRO)) { w.mobys[t].once |= 4; }
                w.mobys[id].state_ticks = 0;
            } else {
                to_middle(w, id, ctx);
            }
        }
        1 => {
            if 0x3c < w.mobys[id].state_ticks && !(ctx.game.scenes.mode == crate::scene::SCENE) { to_middle(w, id, ctx); }
        }
        3 => {
            if wins(ctx.game, 0) == 0 {
                w.mobys[id].set_pv_s(pv::CHALLENGE, 0);
                w.mobys[id].set_pv_s(pv::WAVE, 0);
                w.mobys[id].set_state(8, -1);
                ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_BLEND, ctx.hero);
                ctx.hero.request_state(0, true);
                return;
            }
            let Some((mid, _)) = cuboid_at(ctx, w, id, pv::MIDDLE) else { return };
            if dist(ctx.hero.pos, mid) < 10.0 {
                w.mobys[id].set_state(4, -1);
            } else if 0x3c < w.mobys[id].state_ticks {
                ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_BLEND, ctx.hero);
            }
        }
        _ => {}
    }
}

/// Sub 3 of the entry: Ratchet walked toward the middle (`0x2d7940(0, middle, 0)`).
fn to_middle(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    w.mobys[id].timer = 3;
    ctx.hero.request_state(0, true);
    if let Some((p, _)) = cuboid_at(ctx, w, id, pv::MIDDLE) { ctx.hero.script_walk_to(0.0, p, false); }
}

/// State 7, `0x3fee58`: the menu — ← / → the challenges open (auto-repeat every 15 ticks, the level sound 3), ✕ the
/// selected one starts, △ back to the resort. Not ported: the level's sounds 3 / 4 (`0x367f58`).
fn menu(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if ctx.hero.state != 0x6f { ctx.hero.request_state(0x6f, true); }
    let pressed = ctx.hero.items.pressed;
    let held = ctx.hero.items.held;
    let sel = w.mobys[id].pv_s(pv::SELECTED) as i32;
    let mut rep = w.mobys[id].pv_s(pv::REPEAT) as i32;
    let right = held & crate::pad::button::RIGHT != 0 && (pressed & crate::pad::button::RIGHT != 0 || rep == 0);
    let left = held & crate::pad::button::LEFT != 0 && (pressed & crate::pad::button::LEFT != 0 || rep == 0);
    if right || left {
        let next = if right { (sel + 1..0xe).find(|&n| on_menu(ctx.game, n)) } else { (0..sel).rev().find(|&n| on_menu(ctx.game, n)).or(Some(0)) };
        if let Some(n) = next.filter(|&n| n != sel) {
            w.mobys[id].set_pv_s(pv::SELECTED, n as i16);
        }
        rep = 0xf;
    } else if rep != 0 {
        rep -= 1;
    }
    w.mobys[id].set_pv_s(pv::REPEAT, rep as i16);
    let sel = w.mobys[id].pv_s(pv::SELECTED) as i32;
    ctx.game.arena_menu = Some(Menu {
        selected: sel,
        prize: (sel != 0 || wins(ctx.game, 0) != 0).then(|| prize(ctx.game, sel)),
        wins: wins(ctx.game, sel),
        prev: sel != 0,
        next: (sel + 1..0xe).any(|n| on_menu(ctx.game, n)),
    });
    if pressed & crate::pad::button::CROSS != 0 && on_menu(ctx.game, sel) {
        ctx.game.flags[0x3c] = sel as u8;
        let m = &mut w.mobys[id];
        m.set_pv_s(pv::CHALLENGE, sel as i16);
        m.set_pv_s(pv::WAVE, 0);
        m.set_state(8, -1);
        ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_BLEND, ctx.hero);
        ctx.hero.request_state(0, true);
        ctx.game.arena_menu = None;
    } else if pressed & crate::pad::button::TRIANGLE != 0 {
        ctx.game.cameras.camera_script2(crate::script_camera::RELEASE_BLEND, ctx.hero);
        leave(w, id, ctx);
    }
}

/// State 8: the challenge (module docs).
fn fight(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let ch = w.mobys[id].pv_s(pv::CHALLENGE) as i32;
    if w.mobys[id].first() {
        let m = &mut w.mobys[id];
        m.set_pv_s(pv::WAVE, 0);
        for k in 0..6 { m.set_pv_i(pv::SPAWNED + 4 * k, 0); }
        m.set_pv_i(pv::TICKS, 0);
        m.set_pv_i(pv::ALIVE, 0);
        for k in 0..10 { m.set_pv_i(pv::GATES + 4 * k, 0); }
        m.set_pv_i(pv::GATE, 0);
        count_out(w, id, ctx);
        wave_start(w, id, ctx);
        let m = &mut w.mobys[id];
        for k in 0..0x28 { m.set_pv_s(pv::SAID + 2 * k, 0); }
        for (k, &t) in TURNS.iter().enumerate() { m.set_pv_i(pv::TURNS + 4 * k, t); }
        if ch == 7 { if let Some(a) = ctx.game.ammo.get_mut(WEAPON_7 as usize) { *a = 300; } }
        if ch == 8 { if let Some(a) = ctx.game.ammo.get_mut(WEAPON_8 as usize) { *a = 8; } }
        ctx.hero.hp = ctx.game.max_health;
    }
    score(w, id, ctx);
    let wave = wave_of(w, id);
    if wave.is_some_and(|wv| wave_over(w, id, wv)) {
        let m = &mut w.mobys[id];
        m.set_pv_s(pv::WAVE, m.pv_s(pv::WAVE) + 1);
        for k in 0..6 { m.set_pv_i(pv::SPAWNED + 4 * k, 0); }
        m.set_pv_i(pv::ALIVE, 0);
        m.set_pv_i(pv::BOSS, 0);
        if CHALLENGES[ch as usize].waves.len() as i16 <= m.pv_s(pv::WAVE) {
            if ch == 0 && wins(ctx.game, 0) == 0 { m.set_state(0xb, -1); } else { m.set_state(5, -1); }
            m.set_pv_s(pv::SELECTED, ch as i16);
            return;
        }
        wave_start(w, id, ctx);
    }
    let ticks = w.mobys[id].pv_i(pv::TICKS) + 1;
    w.mobys[id].set_pv_i(pv::TICKS, ticks);
    let held = ctx.hero.items.item;
    let shots = |n: i32| ctx.game.ammo.get(n as usize).copied().unwrap_or(0);
    if (ch == 3 && 0x708 < ticks) || (ch == 5 && 0xe10 < ticks) {
        let n = fail_kills(w, id);
        ctx.game.prompt.announce_count(FAIL_KILLS.0, n, FAIL_KILLS.1, -1);
        lose(w, id);
        return;
    }
    let shot_alive = |w: &MobyWorld, cs: &[i16]| w.mobys.iter().any(|m| m.state < 0x80 && cs.contains(&m.o_class));
    if ch == 7 && held == WEAPON_7 && shots(WEAPON_7) < 1 && !shot_alive(w, &[0x0c89, 0x0bfe]) {
        ctx.game.prompt.announce(FAIL_SHOTS_7, -1);
        lose(w, id);
        return;
    }
    if ch == 8 && held == WEAPON_8 && shots(WEAPON_8) < 1 && !shot_alive(w, &[0x0c06, 0x0c07]) {
        ctx.game.prompt.announce(FAIL_SHOTS_8, -1);
        lose(w, id);
        return;
    }
    if (ch == 0xc || ch == 0xd) && ctx.hero.hp < ctx.game.max_health {
        w.mobys[id].set_pv_i(pv::BOSS, 0);
        ctx.game.prompt.announce(FAIL_HURT, -1);
        lose(w, id);
        return;
    }
    if ch == 0xc && 0x1c20 < ticks {
        w.mobys[id].set_pv_i(pv::BOSS, 0);
        w.mobys[id].set_state(5, -1);
        return;
    }
    // Sending fighters: after a second (or at once with a boss), the round's spawner, the wave's fighters into it
    // (0x0c3e, 0x0b22, 0x0c3b, 0x09ca), then its doors opened.
    if let Some(wave) = wave_of(w, id) {
        if 0x3c < w.mobys[id].state_ticks || wave.bosses[0] != 0 {
            gates_reset(w, id);
            gate_pick(w, id, ctx, wave);
            let mut sent = false;
            if gate(w, id).is_some_and(|g| ready(w, g)) {
                for (k, class) in [(3, 0x0c3e), (1, 0x0b22), (2, 0x0c3b), (0, 0x09ca)] {
                    while w.mobys[id].pv_i(pv::SPAWNED + 4 * k) < wave.fighters[k] as i32 {
                        if !send(w, id, class) { break; }
                        sent = true;
                    }
                }
            }
            if sent { open_gates(w, id, ctx); }
        }
    }
    match ch {
        7 => { ctx.hero.items.ask(ctx.game.held, WEAPON_7); }
        8 => { ctx.hero.items.ask(ctx.game.held, WEAPON_8); }
        0xd => { ctx.hero.items.ask(ctx.game.held, 10); }
        _ => {}
    }
    if ctx.game.frame & 3 == 0 { deal_turns(w, id, ctx); }
    keep_in(w, id, ctx);
}

// ---------------------------------------------------------------------------------------------------------------
// The flame columns 0x0b89 and the crate lifts 0x0ccc

pub const COLUMN: i16 = 0x0b89;
pub const LIFT: i16 = 0x0ccc;
/// `$gp+0x2810..+0x2838`: the column's rest (60), the gap before a warning (60 + 60), the warnings (2), its burn (60),
/// its height (5), its reach at the floor and at the top (1, 0.5), its glow's push and sizes, colour.
const COLUMN_REST: i16 = 60;
const COLUMN_GAP: i16 = 60;
const COLUMN_BLINK: i16 = 60;
const COLUMN_WARNINGS: i16 = 2;
const COLUMN_BURN: i16 = 60;
const COLUMN_HEIGHT: f32 = 5.0;
const COLUMN_REACH: [f32; 2] = [1.0, 0.5];
const COLUMN_GLOW: (f32, f32, f32, u32) = (0.2, 0.3, 0.2, 0x66aa_6600);

/// `0x403be8`: the column is on.
pub fn column_on(w: &MobyWorld, id: usize) -> bool { w.mobys[id].pv_s(0x30) != 0 }

/// `0x403bf8` / `0x403c08`: on / off.
pub fn column_set(w: &mut MobyWorld, id: usize, on: bool) { w.mobys[id].set_pv_s(0x30, on as i16); }

/// 0x0b89 (`0x403620`): a flame column in the floor. Off (1) it waits; on, after its rest it warns (2: its glow
/// flashes twice, 60 ticks apart) and burns (3) for 60 ticks — in view, Ratchet within 4 is hit for `+0x14` on the
/// column's line (its reach 1 at the floor, 0.5 at its top), else what is at its foot; two rising puffs a tick (the
/// exhaust puffs) — then rests again. Not ported: the level's looped sound 0x11 while it burns (`0x367ff0`).
pub fn update_column(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    m.rows = rows_of(m.rot);
    match m.state {
        0 => {
            m.set_pv_b(9, 0);
            m.set_pv_b(8, 3);
            m.set_pv_i(0x10, 0);
            m.drawn = 0;
            m.mode = (m.mode & !mode::TARGET) | 1;
            m.set_pv_s(0x30, 0);
            m.set_pv_s(0x36, -1);
            m.set_pv_s(0x32, COLUMN_REST);
            m.set_state(1, -1);
            return;
        }
        1 => {
            if m.pv_s(0x32) == 0 {
                if m.pv_s(0x30) == 0 { return; }
                m.set_pv_s(0x34, COLUMN_WARNINGS);
                m.set_pv_s(0x32, COLUMN_GAP + COLUMN_BLINK);
                m.set_state(2, -1);
                return;
            }
            if m.pv_s(0x30) == 0 { return; }
        }
        2 => {
            if m.pv_s(0x34) == 0 {
                m.set_pv_s(0x32, COLUMN_BURN);
                m.set_state(3, -1);
            } else if COLUMN_BLINK < m.pv_s(0x32) {
                w.register_draw(id);
            } else if m.pv_s(0x32) == 0 {
                m.set_pv_s(0x32, COLUMN_GAP + COLUMN_BLINK);
                let mut t = m.pv_s(0x34) as i32;
                crate::moby::countdown(&mut t);
                m.set_pv_s(0x34, t as i16);
            }
        }
        3 => {
            if m.pv_s(0x32) == 0 {
                let h = m.pv_s(0x36) as i32;
                m.set_pv_s(0x32, COLUMN_REST);
                if h != -1 { ctx.host.stop_sound(h); }
                m.set_pv_s(0x36, -1);
                m.set_state(1, -1);
                return;
            }
            let pos = m.pos;
            if !ctx.view.is_some_and(|v| v.culled(128.0, [pos[0], pos[1], pos[2], 1.0])) {
                let dmg = m.pv_b(0x14) as f32;
                if crate::hero::physics::dist(pos, ctx.hero.pos) < 4.0 {
                    let top = [pos[0], pos[1], pos[2] + COLUMN_HEIGHT];
                    let line = sub(top, pos);
                    let l2 = crate::hero::physics::dot(line, line);
                    let k = if 0.0 < l2 { (crate::hero::physics::dot(sub(ctx.hero.pos, pos), line) / l2).clamp(0.0, 1.0) } else { 0.0 };
                    let at = add(pos, crate::hero::physics::scale(line, k));
                    let r = COLUMN_REACH[0] + (COLUMN_REACH[1] - COLUMN_REACH[0]) * k;
                    ai::attack_sphere(w, id, ctx, r, dmg, 1.0, at, 0x1_0001, 0, 1, 0);
                } else {
                    ai::attack_sphere(w, id, ctx, COLUMN_REACH[0], dmg, 1.0, pos, 0x1_0001, 0, 1, 0);
                }
                column_puffs(w, id, ctx);
                w.register_draw(id);
            }
            let m = &mut w.mobys[id];
            let mut t = m.pv_s(0x32) as i32;
            crate::moby::countdown(&mut t);
            m.set_pv_s(0x32, t as i16);
            return;
        }
        _ => return,
    }
    let m = &mut w.mobys[id];
    let mut t = m.pv_s(0x32) as i32;
    crate::moby::countdown(&mut t);
    m.set_pv_s(0x32, t as i16);
}

/// `0x403a48`: the burning column's two puffs — one at its foot pulled 0.15 toward the camera rising 20..30 a second,
/// one 0.6 up (jittered 0.2) rising 30..50 a second.
fn column_puffs(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let cam = ctx.host.camera_pos();
    let pos = w.mobys[id].pos;
    let mut d = sub(cam, pos);
    d[2] = 0.0;
    let a = add(pos, set_len(d, 0.15));
    let g = &mut ctx.game.rng;
    let va = [0.0, 0.0, g.randf(20.0, 30.0) / 60.0];
    let add_a = g.randi(2) != 0;
    let mut b = [pos[0], pos[1], pos[2] + 0.6];
    for k in 0..3 { b[k] += g.randf(-0.2, 0.2); }
    let vb = [0.0, 0.0, g.randf(30.0, 50.0) / 60.0];
    let add_b = g.randi(2) != 0;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let m = &w.mobys[id];
    for (p, v, desc, additive) in [(a, va, 7, add_a), (b, vb, 6, add_b)] {
        if let Some(i) = crate::particles::exhaust(sys, &mut ctx.game.rng, m, id, p, v, desc) {
            if !additive { sys.pool.recs[i][3] = 0x44; }
        }
    }
}

/// `0x403b88` (0x0b89 warning or burning): its glow at its foot (`0x32bbe0`: a camera-facing quad of FX 0xb, pushed
/// 0.2 toward the camera; 0.2 warning, 0.3 burning).
pub fn draw_column(w: &MobyWorld, id: usize, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let m = &w.mobys[id];
    let size = match m.state { 3 => COLUMN_GLOW.1, 2 => COLUMN_GLOW.2, _ => return };
    crate::fx::sprite(out, size, COLUMN_GLOW.0, 0.0, m.pos, cam, 0xb, COLUMN_GLOW.3, true);
}

/// `0x417fb8`: a lift free (resting, no crate, not moving).
pub fn lift_free(w: &MobyWorld, id: usize) -> bool { let m = &w.mobys[id]; m.state == 1 && m.pv_i(0) == 0 && m.pv_f(4) == 0.0 }

/// `0x417ed8(rise, spin, lift, crate)`: the crate put back (`0x3a8468`) 2 under the lift, rising `rise` and turning
/// `spin` a tick with it (state 2); the crate's flags 2 (held), its `+0xb0` 1, shown.
pub fn lift(w: &mut MobyWorld, id: usize, c: usize, rise: f32, spin: f32) {
    w.mobys[id].set_state(2, -1);
    let p = w.mobys[id].pos;
    super::crate_::restore(w, c);
    let x = &mut w.mobys[c];
    x.pos = [p[0], p[1], p[2] - 2.0];
    x.mode &= !0x81;
    x.set_pv_f(0xb0, 1.0);
    x.set_pv_i(0xac, 2);
    let m = &mut w.mobys[id];
    ai::set_link(m, 0, Some(c));
    m.set_pv_f(4, rise);
    m.set_pv_f(8, spin);
}

/// 0x0ccc (`0x417bf0`): a crate lift. With its crate broken (state 6) it lets go. Resting (1) it keeps its crate's
/// height (the crate held, flags 2, while it stays); lifting (2: sequence 1) and rising (3: the speed 0 after the
/// sequence) the crate climbs `+4` and turns `+8` a tick until 0.2 over the lift (flags 8: dropped); then down (4,
/// sequence 2) and resting again (sequence 0).
pub fn update_lift(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let crate_ = ai::link(&w.mobys[id], 0).filter(|&c| w.mobys.get(c).is_some_and(|x| x.state < 0x80));
    if let Some(c) = crate_ {
        if w.mobys[c].state == 6 {
            ai::set_link(&mut w.mobys[id], 0, None);
            w.mobys[id].set_pv_f(4, 0.0);
        }
    }
    let crate_ = ai::link(&w.mobys[id], 0);
    let climb = |w: &mut MobyWorld, id: usize, c: usize| {
        let (rise, spin) = (w.mobys[id].pv_f(4), w.mobys[id].pv_f(8));
        let x = &mut w.mobys[c];
        x.pos[2] += rise;
        x.rot[2] = add_rot(x.rot[2], spin);
    };
    match w.mobys[id].state {
        0 => {
            ai::set_link(&mut w.mobys[id], 0, None);
            w.mobys[id].set_pv_f(4, 0.0);
        }
        1 => {
            let Some(c) = crate_ else { return };
            let z = w.mobys[c].pos[2];
            if z != w.mobys[id].pv_f(0xc) {
                w.mobys[id].set_pv_f(0xc, z);
                return;
            }
            w.mobys[c].set_pv_i(0xac, 2);
            return;
        }
        2 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 1, 0, 7);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
            let Some(c) = crate_ else {
                w.mobys[id].set_state(4, -1);
                return;
            };
            climb(w, id, c);
            if wrapped(w, id, ctx) { w.mobys[id].set_state(3, -1); }
            return;
        }
        3 => {
            if w.mobys[id].first() { ctx.host.set_anim_speed(&w.mobys[id], 0.0); }
            if let Some(c) = crate_ {
                climb(w, id, c);
                w.mobys[c].set_pv_i(0xac, 2);
                if w.mobys[c].pos[2] <= w.mobys[id].pos[2] + 0.2 { return; }
                w.mobys[c].set_pv_i(0xac, 8);
            }
            w.mobys[id].set_state(4, -1);
            return;
        }
        4 => {
            if w.mobys[id].first() {
                set_seq(w, id, ctx, 2, 0, 7);
                ctx.host.set_anim_speed(&w.mobys[id], 1.0);
            }
            if !wrapped(w, id, ctx) { return; }
            if let Some(c) = crate_ {
                let z = w.mobys[c].pos[2];
                w.mobys[id].set_pv_f(0xc, z);
            }
        }
        _ => return,
    }
    w.mobys[id].set_state(1, -1);
    set_seq(w, id, ctx, 0, 0, 7);
}

/// `0x3ff808` (the menu's draw, font 2, `0x90ffffff`): the challenge's name centred at (250, 150) at 1.2, its prize
/// ("%s %d %s": 0x2dda, the bolts, 0x2df4) at (90, 184), its wins ("%s %d": 0x2dd4) at (275, 184), the buttons' labels
/// at the top (0x2dd5 at 75, 0x2dd9 at 210, 0x2dd8 at 380; y 95) and the arrows' (0x2dd6 at 166 with one before, 0x2dd7
/// at 305 with one after; y 208). Not ported: the arrows' pictures and their pulse (`0x3ff388`), the other languages'
/// places of the labels (`$gp+0x22a8`, `+0x22f0`, `+0x2318`), the name's box wrapping (`0x2ee560`).
pub fn draw_menu(m: &Menu, text: &dyn Fn(u32) -> Vec<u8>, glyphs: &[rc_formats::disc::boot::Glyph; 256], out: &mut Vec<crate::gui::Quad>) {
    let colour = 0x90ff_ffffu32.to_le_bytes();
    let first = |id: u32| { let t = text(id); t.split(|&c| c == 0).next().unwrap_or(&[]).to_vec() };
    let name = first(challenge_name(m.selected));
    let w = crate::gui::text_width(glyphs, &name, 1.2);
    crate::gui::text(out, glyphs, 2, &name, 250.0 - (w >> 1) as f32, 150.0, 1.2, colour);
    if let Some(p) = m.prize {
        let t = if p < 10000 { [first(0x2dda), format!(" {p} ").into_bytes(), first(0x2df4)].concat() } else { first(0x2dda) };
        crate::gui::text(out, glyphs, 2, &t, 90.0, 184.0, 1.0, colour);
    }
    if m.wins != 0 {
        let t = [first(0x2dd4), format!(" {}", m.wins).into_bytes()].concat();
        crate::gui::text(out, glyphs, 2, &t, 275.0, 184.0, 1.0, colour);
    }
    if m.prev { crate::gui::text(out, glyphs, 2, &first(0x2dd6), 166.0, 208.0, 1.0, colour); }
    if m.next { crate::gui::text(out, glyphs, 2, &first(0x2dd7), 305.0, 208.0, 1.0, colour); }
    for (id, x) in [(0x2dd5, 75.0), (0x2dd9, 210.0), (0x2dd8, 380.0)] {
        crate::gui::text(out, glyphs, 2, &first(id), x, 95.0, 1.0, colour);
    }
}
