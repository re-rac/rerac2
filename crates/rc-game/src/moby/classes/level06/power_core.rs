//! The core **0x0fc2** (L06 `0x458a00`, the decomp's `func_US2_L06_00458A00`; 1 placed): the machine that blows up
//! once its four guard groups are gone and it is hit, throwing its eight rays (crate::moby::classes::ray) and its
//! pieces about. The level's two words: `0x1b375c` the pieces' glow (crate::moby::classes::level06::Globals::core_glow)
//! and `0x1b3758` the core gone (`core_gone`).
//!
//! **pvars**: `+0x60..+0x6c` the four groups, `+0x80..+0x9c` the eight rays, `+0xa0..+0x11c` their ways out (unit),
//! `+0x120..+0x128` the rays' turn (Euler angles), `+0x130` the timer.
//!
//! **States**:
//! * **0**: the rays ([`rays`], L06 `0x458ce0`): each of eight a way out of the yaw / pitch pair in degrees of the
//!   table `0x1b1840` (each wrapped, `0x2efef0`; `0x325100(1, yaw, pitch)`), the ray made from (0, 0, 0) to (0, 1, 0)
//!   (colour 0x3c3c3c78, 25 squares, its length 0 until the turn sets its ends); the glow 10, not gone; 1.
//! * **1**: a hit with the flags 0x4010000 while none of the groups `+0x64`, `+0x68`, `+0x6c`, `+0x60` has a member
//!   alive (`0x35caf8`, crate::moby::classes::props::group_alive): 3. Its hit record cleared (`+0xa8` = −1).
//! * **3** (its first tick: its pieces ([`pieces`], `0x458f38`), its scale × 0.9, its ambient (255, 127, 127), its mode
//!   0x200, the timer 180, the shakes — the roll (`0x1b9880`) 0.02 for 180 ticks, the up and down (`0x1b9860`) 0.4 for
//!   120 — and its sound 0): the rays turned ([`turn`], `0x458dc8`), the glow up by one to 127; the timer out: 4.
//! * **4** (first tick): its sound 1; the skill point 0x11 (`0x1a7a79`, crate::game::GameState::award_skill_point);
//!   the blast at its place (`0x462190`: size 7, shaking, bursts 25 / 20 / 20, 20 puffs, 45 streaks, the flash;
//!   crate::moby::blast); twenty times the level's bolts (`0x367558`, crate::moby::bolts::level_drop); gone set; the
//!   shakes 0.05 for 120 (up and down) and 0.05 for 180 (the roll); it is gone.
//!
//! **The turn** ([`turn`]): the three angles move by (37, 41, 70) / 60 degrees a tick (`$gp+0x2890`, `0x1b1884`,
//! `0x1b1888`; `0x2efe60`); each ray's way turned by their rows (`0x2ef6b8`, `0x2ef318`) runs from 3 out (size 1) to
//! 10 out (size 2) of the core (`0x46c2e8`).
//!
//! **The pieces** ([`pieces`]): its pieces 0..14 (`0x2dbdb0` mode 2: spin 0, gravity 10, no velocity, no sound;
//! crate::moby::shards::spawn_shard), each drifting out at 0.002 a tick along its pivot (`+0x20`) under their own
//! update ([`drift`], `0x458fe0`): moved, the ambient (1.5·g, 1.25·g, g) of the glow `g`; once the core is gone, their
//! velocity 0.1167 out and 0.1833 up, the pieces' tick (crate::moby::shards::update, L06 `0x3df1e0`), the scale × 0.75
//! and the ambient (16, 16, 16).

use crate::hero::physics::{add, add_rot, scale, set_len, wrap_turn, V3};
use crate::moby::classes::{props::group_alive, ray};
use crate::moby::{countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0fc2;

const GROUPS: [usize; 4] = [0x64, 0x68, 0x6c, 0x60];
const RAYS: usize = 0x80;
const WAYS: usize = 0xa0;
const TURN: usize = 0x120;
const TIMER: usize = 0x130;

/// `0x1b1840`: the eight rays' yaw and pitch (degrees).
const TABLE: [[f32; 2]; 8] = [[-300.0, -80.0], [-200.0, -60.0], [-100.0, -40.0], [0.0, -20.0], [200.0, 0.0], [300.0, 20.0], [400.0, 40.0], [500.0, 60.0]];
/// `$gp+0x2890` / `0x1b1884` / `0x1b1888`: the turn's degrees a second.
const SPIN: [f32; 3] = [37.0, 41.0, 70.0];

/// 0x0fc2.
pub fn update_0fc2(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pvars.len() < 0x134 { w.mobys[id].pvars.resize(0x134, 0); }
    match w.mobys[id].state {
        0 => {
            rays(w, id, ctx);
            w.level06.core_glow = 10;
            w.level06.core_gone = 0;
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let hit = w.hit_message(id).is_some_and(|h| h.tmpl.flags & 0x0401_0000 != 0);
            if hit && GROUPS.iter().all(|&g| group_alive(w, w.mobys[id].pv_i(g)) == 0) { w.mobys[id].set_state(3, -1); }
            w.clear_hit(id);
        }
        3 => {
            if w.mobys[id].first() {
                pieces(w, id, ctx);
                let m = &mut w.mobys[id];
                m.scale *= f32::from_bits(0x3f66_6666);
                m.ambient = [255, 127, 127];
                m.mode |= 0x200;
                m.set_pv_i(TIMER, 180);
                let s = &mut ctx.game.shakes.0;
                (s[2].amp, s[2].timer) = (f32::from_bits(0x3ca3_d70a), 180);
                (s[0].amp, s[0].timer, s[0].max) = (f32::from_bits(0x3ecc_cccd), 120, 0);
                ctx.host.play_sound(&w.mobys[id], 0, 0);
            }
            turn(w, id);
            if w.level06.core_glow < 127 { w.level06.core_glow += 1; }
            let mut t = w.mobys[id].pv_i(TIMER);
            let out = countdown(&mut t);
            w.mobys[id].set_pv_i(TIMER, t);
            if out { w.mobys[id].set_state(4, -1); }
        }
        4 => {
            if !w.mobys[id].first() { return; }
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            ctx.game.award_skill_point(0x11);
            let at = w.mobys[id].pos;
            let b = crate::moby::blast::Blast { size: 7.0, bursts: [0x19, 0x14, 0x14], puffs: 0x14, streaks: 0x2d, light: true, shake: true };
            crate::moby::blast::burst(w, ctx, at, &b);
            crate::moby::bolts::level_drop(w, id, 0.0, 20, ctx);
            w.level06.core_gone = 1;
            let s = &mut ctx.game.shakes.0;
            let k = f32::from_bits(0x3d4c_cccd);
            (s[2].amp, s[2].timer) = (k, 180);
            (s[0].amp, s[0].timer, s[0].max) = (k, 120, 0);
            w.delete(id);
        }
        _ => {}
    }
}

/// L06 `0x458ce0` (module docs).
fn rays(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let deg = f32::from_bits(0x3c8e_fa36);
    for (k, [yaw, pitch]) in TABLE.iter().enumerate() {
        let (yaw, pitch) = (wrap_turn(yaw * deg), wrap_turn(pitch * deg));
        let way = crate::moby::classes::shot::polar(1.0, yaw, pitch);
        w.mobys[id].set_pv_v(WAYS + 0x10 * k, way);
        let r = ray::spawn(w, ctx.game.level, [0.0; 4], [0.0, 1.0, 0.0, 0.0], 0x3c3c_3c78, Some(id), 0x19, false);
        w.mobys[id].set_pv_i(RAYS + 4 * k, r.map_or(0, |r| r as i32));
    }
}

/// L06 `0x458dc8` (module docs).
fn turn(w: &mut MobyWorld, id: usize) {
    let step = f32::from_bits(0x3998_825c);
    let m = &mut w.mobys[id];
    let mut a = m.pv_v(TURN);
    for k in 0..3 { a[k] = add_rot(a[k], SPIN[k] * step); }
    m.set_pv_v(TURN, a);
    let rows = crate::moby::attach::rows_of(a);
    let at = m.pos;
    for k in 0..8 {
        let m = &w.mobys[id];
        let d = m.pv_v(WAYS + 0x10 * k);
        let v: V3 = add(add(scale(rows[0], d[0]), scale(rows[1], d[1])), scale(rows[2], d[2]));
        let near = scale(v, 3.0);
        let far = scale(near, f32::from_bits(0x4055_5556));
        let (a, b) = (add(near, at), add(far, at));
        if let Some(r) = m.pv_link(RAYS + 4 * k) {
            ray::ends(w, r, [a[0], a[1], a[2], 1.0], [b[0], b[1], b[2], 2.0]);
        }
    }
}

/// L06 `0x458f38` (module docs).
fn pieces(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (o_class, pos, rot) = { let m = &w.mobys[id]; (m.o_class, m.pos, m.rot) };
    for k in 0..15u8 {
        let Some(p) = crate::moby::shards::spawn_shard(w, ctx, o_class, Some(id), pos, rot, None, 0.0, 10.0, k, [0.0; 3], 0xff) else { continue };
        let m = &mut w.mobys[p];
        let v = set_len(m.pv_v(0x20), f32::from_bits(0x3b03_126f));
        m.set_pv_v(0, v);
        m.update = Some(drift);
    }
}

/// L06 `0x458fe0` (module docs).
fn drift(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let g = w.level06.core_glow as f32;
    let gone = w.level06.core_gone != 0;
    let m = &mut w.mobys[id];
    m.pos = add(m.pos, m.pv_v(0));
    m.ambient = [(g * 1.5) as i32 as u8, (g * 1.25) as i32 as u8, g as i32 as u8];
    if gone {
        let mut v = set_len(m.pv_v(0), f32::from_bits(0x3dee_eef0));
        v[2] += f32::from_bits(0x3e3b_bbbc);
        m.set_pv_v(0, v);
        m.update = Some(crate::moby::shards::update);
        m.scale *= 0.75;
        m.ambient = [16, 16, 16];
    }
}
