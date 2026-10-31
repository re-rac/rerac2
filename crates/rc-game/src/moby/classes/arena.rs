//! Maktar's arena (the level's chunk 1, under the resort; L02): the pieces of the floor and wall the arena master
//! 0x0b59 works.
//!
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
//! Not ported: the plates' sparks (`0x3d85a8`: one in 60 ticks, the particle burst `0x345d88` of `$gp+0x1380` 0.75
//! toward the camera; the burst has no port).

use crate::hero::physics::{add, atan2, dist2, scale, set_len, sub, V3};
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
            m.pos = add(m.pos, scale(m.rows[1], 2.0));
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
                m.pos = add(m.pos, scale(m.rows[1], k));
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
