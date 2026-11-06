//! Level 12's towers **0x12b1** (`0x429a10`, 4 placed; the decomp's `func_US2_L12_00429A10`; L12 addresses), shot
//! down one after another, each in its own way by how many went before it (`$gp+0x4d44`, cleared by every tower's set-up):
//! the first topples and breaks in three, the second blows apart, the third lifts off and falls back to break in three,
//! the fourth flies off and drops later onto the moby `+0x74`, crushing what is under it. The last one gives the skill
//! point 9 (`0x1a7a71`, crate::skill_points).
//!
//! Its pvars: `+0x00` its health block (`+0x20` the health, 100; `+0x24` its most), `+0x50` the colour flash, `+0x64`
//! the topple's speed, `+0x68` a timer, `+0x6c` its climb, `+0x70` 50 over its place, `+0x74` the moby it drops onto
//! (an index), `+0x78` that moby's height, `+0x7c` the ground under it (`0x320820` = L00 `0x31ce20`), `+0x80` its
//! rumble's handle.
//!
//! * **0**: the health 100, the heights, the rumble none, the count 0; 1.
//! * **1**: a hit with the flags 0x230000 (`0x321700` / `0x321760` = L00 `0x31dce8` / `0x31dd48`) takes its damage
//!   off the health and flashes it (0xff8080, `0x323b40` = L00 `0x320128`); under 0: with no other tower (nor any of
//!   their thrown pieces) alive (state under 2), the skill point (`0x366038` = L00 `0x360750`: the level's sound 1, the
//!   message 0x1233, the reward flag `0x1a903c`); then by the count 2, 7, 3, 4 (more: 7) and the count on. Its hit
//!   record cleared.
//! * **2**, the topple: its first tick no collision, a blast (`0x3fb0a0` = L00 `0x3f8348`, crate::moby::blast::burst:
//!   2.5, 20 each), class sound 0; its pitch (`+0xf0`) on by `+0x64`, which grows 0.0125° a tick; past 45°, eight
//!   spheres of 1.2 along its length (every 2.034 up its rows, `0x328db0` = L01 `0x31b498`) hit for 12 (flags 0x30008,
//!   `0x320b80` = L00 `0x31d180`, crate::moby::ai::attack_sphere); past 82°, the camera shakes (`0x1b9da0`: up and
//!   down 0.2 for 60, the roll 0.2 for 90; crate::camera::Shakes) and 8.
//! * **3**, the hop: its first tick 180 ticks, the shakes (the roll 0.1); while they run its rumble (class sound 3,
//!   looped), smoke under it ([`smoke`]), the dust ([`dust`]) under 140 of them, and it climbs (0.0002 a tick² more);
//!   then the rumble stops and it falls (0.0027 a tick²) to the ground: 8.
//! * **4**, the flight: its first tick 260 ticks, the shakes, updated from afar; the smoke, the rumble, the dust under
//!   220, the climb; 50 up and out of sight: the rumble stops, 5.
//! * **5**: the moby `+0x74` there (not deleted), drawn and within 45 of Ratchet across: shown, a target (mode `&
//!   ~0x41 | 0x1000`), its collision its class's, 60 over that moby upside down (`+0xf0` π), falling 0.25 a tick, that
//!   moby hooked (`0x32eb20` = L00 `0x32a818`); 6. Else hidden, no collision, not a target.
//! * **6**, the drop: down by its fall, the rumble; within 16.27 of that moby's height with nothing thrown yet, class
//!   sound 1, a blast of 3 and its piece 0 thrown (`0x32f888` = L00 `0x32b580`, crate::moby::shards::throw_all, the
//!   pieces' update [`update_piece`]); within 11.2 with piece 1 left, a blast of 2 and piece 1 (drawn as piece 2 only
//!   after, `+0x7c` 0x8004); within 6.44 with piece 2 left: the rumble stops, a blast of 1, piece 2, the level's bolts
//!   25 times (`0x32a830` = L00 `0x327388`), gone. Else a sphere of 4 at 12.27 under it hits for 1000 (flags 0x10000);
//!   then the smoke above it.
//! * **7**: class sound 4, a blast of 4, every piece thrown (as 6's), the bolts, gone.
//! * **8**: its three parts 0x12e2, 0x12e1, 0x12e0 made 0, 6.44 and 11 up its rows (`$gp+0x3220`), its angles and
//!   scale, broken at once (`0x32a940` = L00 `0x327498`, crate::moby::shards::break_into: 6..12 across and up, plus
//!   8/60 up; their pieces `$gp+0x3230` 0xf / 3 / 3) with a blast of 1 (no shake) and gone; the bolts, class sound 1,
//!   gone.
//! * Each tick it stays, its flash (`0x323c00` = L00 `0x3201e8`).
//!
//! **A thrown piece** ([`update_piece`], `0x42acd0`, the pieces' update in place of the engine's): 20..50 ticks
//! (`+0x34`); then, or once the piece's own step says it is over (`0x2df798` = L00 `0x2dc1f0`,
//! crate::moby::shards::step), a blast of 1 at its centre, class 0x12b1's sound 2, and by the piece 0..2 a part
//! 0x12e0 / 0x12e1 / 0x12e2 there, its scale the tower's ratio, broken at once (as 8's pieces but with every piece of
//! it thrown, `$gp+0x3270` 3.5..4 across, 5..7.5 up) and gone; the piece gone.
//!
//! Not ported: the count kept as such (the port's: the placed towers already down, the same while all four start
//! together; a respawn that brought back some but not others would count differently); a tower drawn as pieces 1 and 2
//! (`+0x7c` 0x8006 after its first throw: the port's moby draws one piece, so it is drawn whole until the next); the
//! part's rows (`0x2f22f8(rows, tower's rows, 0x18)`: its first row and half its second, the rest the created moby's);
//! the skill point's reward flag (`0x1a903c`, crate::skill_points).

use crate::hero::physics::{add, dist2, scale, V3};
use crate::moby::attach::rows_of;
use crate::moby::blast::{burst, Blast};
use crate::moby::{ai, mode, shards, Ctx, MobyWorld};

pub const CLASS: i16 = 0x12b1;
/// Its parts (`0x12e2 − i`).
const PARTS: [i16; 3] = [0x12e2, 0x12e1, 0x12e0];

const HEALTH: usize = 0x20;
const FLASH: usize = 0x50;
const TOPPLE: usize = 0x64;
const TIMER: usize = 0x68;
const CLIMB: usize = 0x6c;
const TOP: usize = 0x70;
const ONTO: usize = 0x74;
const ONTO_Z: usize = 0x78;
const GROUND: usize = 0x7c;
const RUMBLE: usize = 0x80;
/// The port's, past the class's pvars: its `+0x7c` piece word (0x8000 | the pieces it is drawn as).
const PIECES: usize = 0x100;

/// The skill point it gives.
const SKILL_POINT: usize = 9;
const HIT_FLAGS: u32 = 0x23_0000;
/// The climb's step and the fall's (a tick²).
const LIFT: f32 = f32::from_bits(0x3969_0453);
const DROP: f32 = f32::from_bits(0x3b32_6750);

/// `$gp+0x3220` / `+0x3230`: the parts' places up its rows and their pieces.
const PART_UP: [f32; 3] = [0.0, f32::from_bits(0x40ce_147b), 11.0];
const PART_MASK: [u32; 3] = [0xf, 3, 3];
/// `$gp+0x3250` / `+0x3270`: the thrown pieces' speeds (x, y, z from, x, y, z to; a second), the tower's and a
/// part's; `$gp+0x3248` / `+0x3240` their spin and gravity; `$gp+0x3290` the piece skipped (none).
const THROW: [f32; 8] = [5.0, 5.0, 10.0, 0.0, 8.0, 8.0, 15.0, 0.0];
const THROW_PART: [f32; 8] = [3.5, 3.5, 5.0, 0.0, 4.0, 4.0, 7.5, 0.0];
const SPIN: (f32, f32) = (90.0, 360.0);
const GRAVITY: (f32, f32) = (10.0, 20.0);

/// `0x3fb0a0(at, shake, 25, 20, 20, 20, 45, 1, s, 20, 1, 1, 0, 0)`.
fn blast(w: &mut MobyWorld, ctx: &mut Ctx, at: V3, size: f32, shake: bool) {
    burst(w, ctx, at, &Blast { size, bursts: [25, 20, 20], puffs: 20, streaks: 45, light: true, shake });
}

/// The camera's shakes (`0x1b9da0`): up and down 0.2 for 60 (its length cleared), the roll `roll` for 90.
fn shake(ctx: &mut Ctx, roll: f32) {
    let s = &mut ctx.game.shakes.0;
    (s[2].amp, s[2].timer) = (roll, 90);
    (s[0].amp, s[0].timer, s[0].max) = (0.2, 60, 0);
}

/// Its rumble (class sound 3, looped) kept playing.
fn rumble(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(RUMBLE);
    if h == -1 || !ctx.host.sound_playing(h) {
        let h = ctx.host.play_sound(&w.mobys[id], 3, 4);
        w.mobys[id].set_pv_i(RUMBLE, h);
    }
}

/// Its rumble stopped.
fn hush(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    let h = w.mobys[id].pv_i(RUMBLE);
    if h != -1 && ctx.host.sound_playing(h) { ctx.host.stop_sound(h); }
    w.mobys[id].set_pv_i(RUMBLE, -1);
}

/// `0x32f888(m, h, spin, gravity, 0xff, −1, −1, 0xf, mask, 0xff, 0x42acd0)`: its pieces in `mask` thrown, their
/// update [`update_piece`].
fn throw(w: &mut MobyWorld, id: usize, ctx: &mut Ctx, mask: u32) {
    let before = w.created;
    shards::throw_all(w, id, ctx, THROW, SPIN, GRAVITY, 0xff, -1, -1, 0xf, mask);
    for m in w.mobys.iter_mut().filter(|m| before < m.serial && m.o_class == CLASS) { m.update = Some(update_piece); }
}

/// `0x31f620(v, r)` (L02 `0x31fce0`): a random point across within `r` of the origin.
fn random_within(ctx: &mut Ctx, r: f32) -> V3 {
    let rng = &mut ctx.game.rng;
    let a = rng.rand_angle();
    let u = (((rng.rand() >> 16) & 0xfff) as f32) * 0.000_244_140_62;
    let l = (1.0 - u * u) * r;
    [a.cos() * l, a.sin() * l, 0.0]
}

/// A puff's record after `0x33ad98` (L00 `0x335bb8`): 10 ticks, its byte 9 0x74 (`0x2f36a0(4)` + 0x70), its mode 2,
/// alpha 0xdf, its life byte `life`'s.
fn shape(rec: &mut [u8], life: u8) {
    rec[9] = 4 + 0x70;
    rec[0x24..0x28].copy_from_slice(&2u32.to_le_bytes());
    rec[0x2a] = 0xdf;
    rec[0x2b] = life;
}

/// `0x42a808`: three dust puffs at its place on the ground (`0x33ad98(0.1, 1, 0.9, 420000, …, randi(16), …)`), moving
/// 0.5, 0.5 and 1/3 across a random way and 1/12 up: 0xdf202020 (ALPHA 0x44), 0xdf208080 (0x48), 0xdf808080 (0x44);
/// the second and third only shaped when the first was made.
fn dust(w: &MobyWorld, id: usize, ctx: &mut Ctx) {
    let m = &w.mobys[id];
    let at = [m.pos[0], m.pos[1], m.pv_f(GROUND)];
    let mut first = None;
    for (k, (r, rgba, alpha)) in [(0.5, 0xdf20_2020u32, 0x44u8), (0.5, 0xdf20_8080, 0x48), (f32::from_bits(0x3eaa_aaab), 0xdf80_8080, 0x44)].into_iter().enumerate() {
        let mut v = random_within(ctx, r);
        v[2] = f32::from_bits(0x3daa_aaab);
        let spin = ctx.game.rng.randi(0x10);
        let Some(sys) = ctx.parts.as_deref_mut() else { return };
        let i = crate::particles::puff(sys, &mut ctx.game.rng, f32::from_bits(0x3dcc_cccd), 1.0, f32::from_bits(0x3f66_6666), 420_000.0, at, spin, v, rgba, 0);
        if k == 0 { first = i; }
        if let (Some(i), Some(_)) = (i, first) {
            let rec = &mut sys.pool.recs[i];
            rec[0xa..0xc].copy_from_slice(&10i16.to_le_bytes());
            let life = rec[0xa];
        shape(rec, life);
            rec[3] = alpha;
        }
    }
}

/// `0x42aa78(m, up)`: smoke 0.25 under its place moving 0.5 a tick up (or down), turning `randi(16)` either way: two
/// puffs 0xdf204080 (`0x33ad98(0.1, 1, 0.9, 420000, …)`), then a white one (`(0.05, 1, 0.97, 210000, …, 16, …,
/// 0xdfffffff)`, 5 ticks, a random roll); the second's and third's shapes go to the first's header (the game's).
fn smoke(w: &MobyWorld, id: usize, ctx: &mut Ctx, up: bool) {
    let v = [0.0, 0.0, if up { 0.5 } else { -0.5 }];
    let r = ctx.game.rng.randi(0x10);
    let spin = if ctx.game.rng.randi(2) != 0 { -r } else { r };
    let mut at = w.mobys[id].pos;
    at[2] -= 0.25;
    let Some(sys) = ctx.parts.as_deref_mut() else { return };
    let rng = &mut ctx.game.rng;
    let (jitter, shrink) = (f32::from_bits(0x3dcc_cccd), f32::from_bits(0x3f66_6666));
    let first = crate::particles::puff(sys, rng, jitter, 1.0, shrink, 420_000.0, at, spin, v, 0xdf20_4080, 0);
    if let Some(i) = first {
        let rec = &mut sys.pool.recs[i];
        rec[0xa..0xc].copy_from_slice(&10i16.to_le_bytes());
        let life = rec[0xa];
        shape(rec, life);
    }
    let second = crate::particles::puff(sys, rng, jitter, 1.0, shrink, 420_000.0, at, spin, v, 0xdf20_4080, 0);
    if let (Some(i), Some(f)) = (second, first) {
        sys.pool.recs[f][0xa..0xc].copy_from_slice(&10i16.to_le_bytes());
        sys.pool.recs[f][9] = 4 + 0x70;
        let life = sys.pool.recs[f][0xa];
        let rec = &mut sys.pool.recs[i];
        rec[0x24..0x28].copy_from_slice(&2u32.to_le_bytes());
        rec[0x2a] = 0xdf;
        rec[0x2b] = life;
    }
    let third = crate::particles::puff(sys, rng, f32::from_bits(0x3d4c_cccd), 1.0, f32::from_bits(0x3f78_51ec), 210_000.0, at, 0x10, v, 0xdfff_ffff, 0);
    if let Some(i) = third {
        let roll = rng.randi(0xff) as u8;
        let life = first.map_or(5, |f| sys.pool.recs[f][0xa]);
        let rec = &mut sys.pool.recs[i];
        rec[0xa..0xc].copy_from_slice(&5i16.to_le_bytes());
        rec[9] = 4 + 0x70;
        rec[8] = roll;
        rec[0x2a] = 0xdf;
        rec[0x24..0x28].copy_from_slice(&2u32.to_le_bytes());
        rec[0x2b] = life;
    }
}

/// The towers already down (the port's count, module docs).
fn count(w: &MobyWorld, id: usize) -> i32 {
    w.mobys.iter().enumerate().filter(|(k, m)| *k != id && m.o_class == CLASS && m.instance.is_some() && 2 <= m.state).count() as i32
}

/// 0x12b1 (module docs).
pub fn update_12b1(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let ground = ai::ground_z(ctx, 0.5, w.mobys[id].pos, 0);
            let m = &mut w.mobys[id];
            m.set_pv_s(HEALTH + 4, 100);
            m.set_pv_f(HEALTH, 100.0);
            let top = m.pos[2] + 50.0;
            m.set_pv_f(TOP, top);
            m.set_pv_f(GROUND, ground);
            m.set_pv_i(RUMBLE, -1);
            m.set_state(1, -1);
        }
        1 => {
            let hit = w.hit_message(id).filter(|h| h.tmpl.flags & HIT_FLAGS != 0);
            let dmg = ai::damage_of(w, id, hit.as_ref(), false);
            let m = &mut w.mobys[id];
            if 0.0 < dmg {
                m.set_pv_b(FLASH + 7, 0xff);
                m.set_pv_b(FLASH + 9, 0x80);
                m.set_pv_b(FLASH + 8, 0x80);
                ai::flash_start(m, FLASH);
                m.set_pv_f(HEALTH, m.pv_f(HEALTH) - dmg);
            }
            if w.mobys[id].pv_f(HEALTH) < 0.0 {
                let alive = w.mobys.iter().enumerate().any(|(k, m)| k != id && m.o_class == CLASS && m.state < 2);
                if !alive { ctx.game.award_skill_point(SKILL_POINT); }
                let s = match count(w, id) { 0 => 2, 1 => 7, 2 => 3, 3 => 4, _ => 7 };
                w.mobys[id].set_state(s, -1);
            }
            w.clear_hit(id);
        }
        2 => {
            if w.mobys[id].first() {
                w.mobys[id].has_collision = false;
                let p = w.mobys[id].pos;
                burst(w, ctx, p, &Blast { size: 2.5, bursts: [20, 20, 20], puffs: 20, streaks: 20, light: true, shake: true });
                ctx.host.play_sound(&w.mobys[id], 0, 0);
            }
            let m = &mut w.mobys[id];
            m.rot[0] += m.pv_f(TOPPLE);
            m.set_pv_f(TOPPLE, m.pv_f(TOPPLE) + f32::from_bits(0x3964_c38a));
            if f32::from_bits(0x3f49_0fdc) < m.rot[0] {
                for i in 0..8 {
                    let m = &w.mobys[id];
                    let c = add(scale(m.rows[2], i as f32 * f32::from_bits(0x4002_28f6)), m.pos);
                    ai::attack_sphere(w, id, ctx, f32::from_bits(0x3f99_999a), 12.0, 1.0, c, 0x3_0008, 0, 1, 0);
                }
            }
            if f32::from_bits(0x3fb7_3095) < w.mobys[id].rot[0] {
                shake(ctx, 0.2);
                w.mobys[id].set_state(8, -1);
            }
        }
        3 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.set_pv_i(TIMER, 0xb4);
                m.set_pv_f(CLIMB, 0.0);
                shake(ctx, 0.1);
            }
            if w.mobys[id].pv_i(TIMER) != 0 {
                rumble(w, id, ctx);
                smoke(w, id, ctx, false);
                let mut t = w.mobys[id].pv_i(TIMER);
                let out = crate::moby::countdown(&mut t);
                w.mobys[id].set_pv_i(TIMER, t);
                if !out && t < 0x8c { dust(w, id, ctx); }
                let m = &mut w.mobys[id];
                m.pos[2] += m.pv_f(CLIMB);
                m.set_pv_f(CLIMB, m.pv_f(CLIMB) + LIFT);
            } else {
                hush(w, id, ctx);
                let m = &mut w.mobys[id];
                m.pos[2] += m.pv_f(CLIMB);
                m.set_pv_f(CLIMB, m.pv_f(CLIMB) - DROP);
                if m.pos[2] < m.pv_f(GROUND) { m.set_state(8, -1); }
            }
        }
        4 => {
            if w.mobys[id].first() {
                let m = &mut w.mobys[id];
                m.set_pv_i(TIMER, 0x104);
                m.set_pv_f(CLIMB, 0.0);
                m.update_dist = 0xff;
                shake(ctx, 0.1);
            }
            smoke(w, id, ctx, false);
            rumble(w, id, ctx);
            let mut t = w.mobys[id].pv_i(TIMER);
            let out = crate::moby::countdown(&mut t);
            w.mobys[id].set_pv_i(TIMER, t);
            if !out && t < 0xdc { dust(w, id, ctx); }
            let m = &mut w.mobys[id];
            m.pos[2] += m.pv_f(CLIMB);
            m.set_pv_f(CLIMB, m.pv_f(CLIMB) + LIFT);
            if m.pv_f(TOP) < m.pos[2] && m.drawn == 0 {
                hush(w, id, ctx);
                w.mobys[id].set_state(5, -1);
            }
        }
        5 => {
            let p = usize::try_from(w.mobys[id].pv_i(ONTO)).ok().filter(|&p| w.mobys.get(p).is_some_and(|m| m.state != 0xfe && m.state != 0xfd));
            let near = p.filter(|&p| dist2(w.mobys[p].pos, ctx.hero.pos) < 45.0 && w.mobys[p].drawn != 0);
            match near {
                Some(p) => {
                    let solid = w.class(CLASS).is_some_and(|c| c.has_collision);
                    let at = w.mobys[p].pos;
                    let m = &mut w.mobys[id];
                    m.mode = (m.mode & 0xffbe) | mode::TARGET;
                    m.has_collision = solid;
                    m.set_pv_f(ONTO_Z, at[2]);
                    m.pos = at;
                    m.pos[2] += 60.0;
                    m.rot[0] = std::f32::consts::PI;
                    m.set_pv_f(CLIMB, -0.25);
                    w.mobys[p].once |= 4;
                    w.mobys[id].set_state(6, -1);
                }
                None => {
                    let m = &mut w.mobys[id];
                    m.has_collision = false;
                    m.mode = (m.mode | 0x41) & !mode::TARGET;
                }
            }
        }
        6 => {
            let m = &mut w.mobys[id];
            let base = [m.pos[0], m.pos[1], m.pv_f(ONTO_Z)];
            m.pos[2] += m.pv_f(CLIMB);
            rumble(w, id, ctx);
            let (z, left) = (w.mobys[id].pos[2], w.mobys[id].pv_i(PIECES));
            if z < base[2] + f32::from_bits(0x40ce_147b) && left & 4 != 0 {
                hush(w, id, ctx);
                blast(w, ctx, base, 1.0, true);
                throw(w, id, ctx, 4);
                crate::moby::bolts::level_drop(w, id, 0.0, 0x19, ctx);
                return w.delete(id);
            } else if z < base[2] + f32::from_bits(0x4133_3333) && left & 2 != 0 {
                blast(w, ctx, base, 2.0, true);
                throw(w, id, ctx, 2);
                w.mobys[id].set_pv_i(PIECES, 0x8004);
                w.mobys[id].piece = Some(2);
            } else if z < base[2] + f32::from_bits(0x4182_28f6) && left == 0 {
                ctx.host.play_sound(&w.mobys[id], 1, 0);
                blast(w, ctx, base, 3.0, true);
                throw(w, id, ctx, 1);
                w.mobys[id].set_pv_i(PIECES, 0x8006);
            } else {
                let m = &w.mobys[id];
                let c = add(m.pos, [0.0, 0.0, f32::from_bits(0xc144_51ec)]);
                ai::attack_sphere(w, id, ctx, 4.0, 1000.0, 1.0, c, 0x1_0000, 0, 1, 0);
            }
            smoke(w, id, ctx, true);
        }
        7 => {
            if w.mobys[id].first() { ctx.host.play_sound(&w.mobys[id], 4, 0); }
            let p = w.mobys[id].pos;
            blast(w, ctx, p, 4.0, true);
            throw(w, id, ctx, 0xff);
            crate::moby::bolts::level_drop(w, id, 0.0, 0x19, ctx);
            return w.delete(id);
        }
        8 => {
            for i in 0..3 {
                let k = w.create(PARTS[i]);
                let ratio = { let m = &w.mobys[id]; m.scale / w.class(CLASS).map_or(1.0, |c| c.scale) };
                let part_scale = w.class(PARTS[i]).map_or(1.0, |c| c.scale);
                let (pos, rot, up) = { let m = &w.mobys[id]; (m.pos, m.rot, m.rows[2]) };
                let c = &mut w.mobys[k];
                c.pos = add(scale(up, PART_UP[i]), pos);
                c.rot = rot;
                c.rows = rows_of(rot);
                c.scale = part_scale * ratio;
                shards::break_into(w, k, ctx, (6.0, 12.0), (6.0, 12.0), PART_MASK[i], [0.0, 0.0, f32::from_bits(0x3e08_8889)], 0xff);
                let at = w.mobys[k].pos;
                blast(w, ctx, at, 1.0, false);
                w.delete(k);
            }
            crate::moby::bolts::level_drop(w, id, 0.0, 0x19, ctx);
            ctx.host.play_sound(&w.mobys[id], 1, 0);
            return w.delete(id);
        }
        _ => {}
    }
    ai::flash(&mut w.mobys[id], FLASH);
}

/// `0x42acd0`: a thrown piece's update (module docs).
pub fn update_piece(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            let life = ctx.game.rng.rand_range(20, 50);
            w.mobys[id].set_pv_s(0x34, life as i16);
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let life = w.mobys[id].pv_s(0x34);
            if 0 <= life && !shards::step(w, id, ctx) {
                w.mobys[id].set_pv_s(0x34, life - 1);
                return;
            }
            let (at, piece, ratio) = {
                let m = &w.mobys[id];
                let p = m.pv_v(0x20);
                let at = add(add(add(scale(m.rows[0], p[0]), scale(m.rows[1], p[1])), scale(m.rows[2], p[2])), m.pos);
                (at, m.piece, m.scale / w.class(CLASS).map_or(1.0, |c| c.scale))
            };
            blast(w, ctx, at, 1.0, false);
            ctx.host.play_sound_of(&w.mobys[id], CLASS, 2, 0);
            let part = match piece { Some(0) => Some(0x12e0), Some(1) => Some(0x12e1), Some(2) => Some(0x12e2), _ => None };
            if let Some(part) = part {
                let k = w.create(part);
                let part_scale = w.class(part).map_or(1.0, |c| c.scale);
                let rows = w.mobys[id].rows;
                let c = &mut w.mobys[k];
                c.pos = at;
                c.scale = part_scale * ratio;
                c.rows[0] = rows[0];
                (c.rows[1][0], c.rows[1][1]) = (rows[1][0], rows[1][1]);
                shards::throw_all(w, k, ctx, THROW_PART, SPIN, GRAVITY, 0xff, -1, -1, 0xf, 0xff);
                w.delete(k);
            }
            w.delete(id);
        }
        _ => {}
    }
}
