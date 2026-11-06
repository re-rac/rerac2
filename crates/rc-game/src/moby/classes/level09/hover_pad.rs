//! The hover pad **0x0e84** (L09 `0x411ae8`, L12 `0x408a28`; the decomp's `func_L09_00410608`; 4 placed on levels 9 and
//! 12): a platform (its block `+0x20`, crate::moby::platform) that, with Ratchet standing on it, glides to the place of
//! its cuboid `+0x60` and, once he is off, back home, bobbing and swaying on its thrusters' puffs all the while.
//!
//! **Its pvars:** `+0x60` the cuboid (−1: the game prints that it needs one and the pad stays), `+0x64` its glide's
//! share of the way a second (a tick from state 0 on), `+0x6c` its loop sound, `+0x70` / `+0x74` the bob's last offset
//! and phase, `+0x78` / `+0x7c` the sway's phases, `+0x80` home.
//!
//! * **0**: home its place, `+0x64` ÷ 60, its scale × 1.75, 1.
//! * **1** (loop sound 1): Ratchet on it on the ground (`0x18a15c` it, `0x18a160` not 0) → 2.
//! * **2** (loop sound 0): each tick its share of the way to the cuboid's place; within 0.88 → 3.
//! * **3** (loop sound 1): Ratchet off it → 4.
//! * **4** (loop sound 0 while it moves): unless a moby is within 1.5 of it (the sphere test `0x2db908` = L00
//!   `0x2df8d8`, flags 0x10), its share of the way home; within 0.88 → 1.
//!
//! Each state's first tick stops its loop sound. **Each tick** after: the bob (`0x3208c8` = L00 `0x324430`: 0.1 at
//! 120°/s, by the change), the sway (`0x320980` = L00 `0x3244e8`: 4.5° at 70°/s and 100°/s; the tunables `$gp+0x27e8..
//! 0x27f8`, L12 `$gp+0x24b8..`, the same values), the thrusters' puffs ([`puffs`], `0x4120a8`) and the platform block
//! (`0x31ead8` = L00 `0x322668`).

use crate::hero::physics::{add, dist, scale, sub, V3};
use crate::moby::classes::props::{bob, sway};
use crate::moby::{platform, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0e84;
const DEG_S: f32 = 0.000_290_888_25;

/// The loop sound `n` kept playing (`0x35d9a8(n, 4, m)` unless its handle still plays).
fn keep_sound(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, n: i32) {
    let h = w.mobys[id].pv_i(0x6c);
    if 0 <= h && ctx.host.sound_playing(h) { return; }
    let h = ctx.host.play_sound(&w.mobys[id], n, 4);
    w.mobys[id].set_pv_i(0x6c, h);
}

/// The first tick of a state: the loop sound stopped.
fn first(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !w.mobys[id].first() { return; }
    let h = w.mobys[id].pv_i(0x6c);
    if 0 <= h && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(0x6c, -1);
}

/// Its share of the way to `to`; whether it is then within 0.88.
fn glide(w: &mut MobyWorld, id: usize, to: V3) -> bool {
    let m = &mut w.mobys[id];
    let k = m.pv_f(0x64);
    m.pos = add(m.pos, scale(sub(to, m.pos), k));
    dist(m.pos, to) <= 0.88
}

/// `0x4120a8`: two orange puffs (0.1 jitter, 420000, turning 0..15 either way) and a white one (0.05, 210000, 16) 0.25
/// under it, falling 0.5; the first two live 10 ticks, the white one 5.
fn puffs(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    use crate::particles::{puff, rec};
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let g = &mut ctx.game.rng;
    let vel = [0.0, 0.0, -0.5];
    let mut spin = g.randi(0x10);
    if g.randi(2) != 0 { spin = -spin; }
    let p = w.mobys[id].pos;
    let pos = [p[0], p[1], p[2] - 0.25];
    let set_first = |sys: &mut crate::particles::Particles, a: usize| {
        rec::set_i16(&mut sys.pool.recs[a], 0xa, 10);
        sys.pool.recs[a][9] = 0x70 + 4;
    };
    let tail = |sys: &mut crate::particles::Particles, i: usize, life: u8| {
        let r = &mut sys.pool.recs[i];
        rec::set_u32(r, 0x24, 2);
        r[0x2a] = 0xdf;
        r[0x2b] = life;
    };
    let a = puff(sys, g, 0.1, 1.0, 0.9, 420_000.0, pos, spin, vel, 0xdf20_4080, 0);
    let life = |sys: &crate::particles::Particles| a.map_or(0, |a| sys.pool.recs[a][0xa]);
    if let Some(a) = a {
        set_first(sys, a);
        let l = life(sys);
        tail(sys, a, l);
    }
    if let Some(b) = puff(sys, g, 0.1, 1.0, 0.9, 420_000.0, pos, spin, vel, 0xdf20_4080, 0) {
        // The game sets the first puff's life here again (not the second's).
        if let Some(a) = a { set_first(sys, a); }
        let l = life(sys);
        tail(sys, b, l);
    }
    if let Some(c) = puff(sys, g, 0.05, 1.0, f32::from_bits(0x3f78_51ec), 210_000.0, pos, 0x10, vel, 0xdfff_ffff, 0) {
        rec::set_i16(&mut sys.pool.recs[c], 0xa, 5);
        sys.pool.recs[c][9] = 0x70 + 4;
        sys.pool.recs[c][8] = g.randi(0xff) as u8;
        let l = life(sys);
        tail(sys, c, l);
    }
}

/// 0x0e84 (module docs).
pub fn update_0e84(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let (old_pos, old_rot) = (w.mobys[id].pos, w.mobys[id].rot);
    let on = ctx.hero.ground_moby == Some(id);
    let cuboid = w.mobys[id].pv_i(0x60);
    match w.mobys[id].state {
        0 => {
            let m = &mut w.mobys[id];
            let p = m.pos;
            m.set_pv_v(0x80, p);
            m.set_pv_f(0x64, m.pv_f(0x64) * 0.016_666_668);
            m.scale *= 1.75;
            m.set_pv_i(0x6c, -1);
            m.set_state(1, -1);
        }
        1 => {
            first(w, id, ctx);
            keep_sound(w, id, ctx, 1);
            if cuboid != -1 && on && ctx.hero.grounded_ticks != 0 { w.mobys[id].set_state(2, -1); }
        }
        2 => {
            first(w, id, ctx);
            keep_sound(w, id, ctx, 0);
            if let Some((to, _)) = ctx.level.cuboid_place(cuboid) {
                if glide(w, id, to) { w.mobys[id].set_state(3, -1); }
            }
        }
        3 => {
            first(w, id, ctx);
            keep_sound(w, id, ctx, 1);
            if !on { w.mobys[id].set_state(4, -1); }
        }
        4 => {
            first(w, id, ctx);
            let p = w.mobys[id].pos;
            let blocked = ctx.coll.and_then(|c| crate::collision::coll_sphere_m(c, ctx.scene, p, 1.5, crate::collision::QueryFlags(0x10), Some(id))).is_some_and(|h| h.moby.is_some());
            if !blocked {
                keep_sound(w, id, ctx, 0);
                let home = w.mobys[id].pv_v(0x80);
                if glide(w, id, home) { w.mobys[id].set_state(1, -1); }
            }
        }
        _ => {}
    }
    let m = &mut w.mobys[id];
    bob(m, 0.1, 120.0 * DEG_S, -1.0, 0x74, 0x70);
    sway(m, f32::from_bits(0x3da0_d97d), 70.0 * DEG_S, 100.0 * DEG_S, 0x78, 0x7c);
    puffs(w, id, ctx);
    let m = &mut w.mobys[id];
    let (pos, rot) = (m.pos, m.rot);
    platform::write(m, 0x20, sub(pos, old_pos), old_rot, rot);
}
