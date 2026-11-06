//! Grelbin's den **0x1177** (`0x420190`, 32 placed; the decomp's v1.01 `func_L19_0041EE60`): it holds the lurkers
//! 0x10d0 of its group (crate::moby::classes::level19::lurker) until the moby it watches is gone and Ratchet is
//! near, then breaks open and lets them out one at a time. On the framework (crate::moby::ai): its target block
//! `+0x50` (`+0x64` found).
//!
//! **Its pvars:** `+0x120` its group (−1: deleted), `+0x128` its search's range (and height), `+0x138` the moby it
//! watches (−1 none: as gone), `+0x13c` the wait between releases.
//!
//! **Each tick**: nothing outside the loaded variant (`0x31f3f0` = L00 `0x321a78`); deleted (`0x30f008` = L00
//! `0x311688`) without a group or once its uid is done (a mission one: `0x21ee44[uid]` or its save count).
//!
//! **States** (`+0x20`): **0** the set-up: on the ground (`0x31a908` = L00 `0x31ce20`) and 0.1..0.3 into it, its
//! pieces `0x1f` drawn on top, its sequence stopped; every lurker of its group held (`+0x26c`), straight to the run
//! once out (`+0x274`), its first search's range `+0x128` + 10; then 1. **1** closed: the search (`0x327e70` = L00
//! `0x32ad38`, its range and height `+0x128`); Ratchet found with the watched moby gone, it breaks (its pieces off,
//! `0x324a08` = L00 `0x327498`, crate::moby::shards::break_into: mask 0x1f, 3..6 across and up, 0.15 up, no sound),
//! its sound 0 (`0x35e400` = L00 `0x3605f0`) and 3. **2** open: the search (range `+0x128` + 10); found, with the pool
//! of attackers open (`$gp+0x2cd0` < `$gp+0x2cd4`) and fewer than three of its lurkers out and alive, after its wait
//! (`0x2ec0d0` = L00 `0x2eeab0`) 3. **3** the release: the first held lurker of its group put at it (its home too,
//! on the ground under 1 up), no longer held, its mode bit 0x80 cleared; then the wait 200 and 2, or with none left
//! 4. **4** empty.

use crate::moby::classes::level19::lurker::{self, HELD, HOME, MOST_ATTACKERS, RANGE1, STRAIGHT};
use crate::moby::shards;
use crate::moby::{ai, countdown, Ctx, MobyWorld};

pub const CLASS: i16 = 0x1177;

const FOUND: usize = 0x64;
const GROUP: usize = 0x120;
const RANGE: usize = 0x128;
const WATCH: usize = 0x138;
const WAIT: usize = 0x13c;

/// Its group's lurkers.
fn lurkers(w: &MobyWorld, id: usize) -> Vec<usize> {
    let g = w.mobys[id].pv_i(GROUP);
    let Some(list) = usize::try_from(g).ok().and_then(|g| w.groups.get(g)) else { return Vec::new() };
    list.iter().copied().take_while(|&k| k < w.mobys.len()).filter(|&k| w.mobys[k].o_class == lurker::CLASS).collect()
}

/// 0x1177 (module docs).
pub fn update_1177(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if !ctx.level.in_variant(w.mobys[id].pos, ctx.game.variant) { return; }
    let m = &w.mobys[id];
    let uid = m.uid;
    let done = m.mission != 0xff && (ctx.game.uid_dead.get(uid.max(0) as usize).copied().unwrap_or(0) != 0 || ctx.game.uid_count(uid) != 0);
    if m.pv_i(GROUP) == -1 || done {
        w.delete(id);
        return;
    }
    match w.mobys[id].state {
        0 => {
            let p = w.mobys[id].pos;
            let z = ai::ground_z(ctx, 0.5, p, 0);
            let sink = ctx.game.rng.randf(0.1, 0.3);
            let m = &mut w.mobys[id];
            m.bangles = 0x1f;
            m.pos[2] = z - sink;
            ctx.host.set_anim_speed(&w.mobys[id], 0.0);
            let range = w.mobys[id].pv_f(RANGE) + 10.0;
            for k in lurkers(w, id) {
                let l = &mut w.mobys[k];
                l.set_pv_i(STRAIGHT, 1);
                l.set_pv_i(HELD, 1);
                l.set_pv_f(RANGE1, range);
            }
            w.mobys[id].set_state(1, -1);
        }
        1 => {
            let r = w.mobys[id].pv_f(RANGE);
            ai::find_target(w, id, ctx, r, r, -1, None, false);
            let k = w.mobys[id].pv_i(WATCH);
            let gone = usize::try_from(k).ok().and_then(|k| w.mobys.get(k)).is_none_or(|e| e.state == 0xfe || e.state == 0xfd);
            if w.mobys[id].pv_i(FOUND) != 1 || !gone { return; }
            w.mobys[id].bangles = 0;
            shards::break_into(w, id, ctx, (3.0, 6.0), (3.0, 6.0), 0x1f, [0.0, 0.0, f32::from_bits(0x3e19_999a)], 0xff);
            ctx.host.play_sound(&w.mobys[id], 0, 0);
            w.mobys[id].set_state(3, -1);
        }
        2 => {
            let r = w.mobys[id].pv_f(RANGE);
            ai::find_target(w, id, ctx, r + 10.0, r, -1, None, false);
            if w.mobys[id].pv_i(FOUND) != 1 { return; }
            let out = lurkers(w, id).into_iter().filter(|&k| { let l = &w.mobys[k]; l.pv_i(HELD) == 0 && l.state != 0xfe && l.state != 0xfd }).count();
            if MOST_ATTACKERS <= w.level19.attackers || 2 < out { return; }
            let m = &mut w.mobys[id];
            let mut t = m.pv_i(WAIT);
            let ready = countdown(&mut t);
            m.set_pv_i(WAIT, t);
            if ready { m.set_state(3, -1); }
        }
        3 => {
            let at = w.mobys[id].pos;
            let next = lurkers(w, id).into_iter().find(|&k| w.mobys[k].pv_i(HELD) == 1);
            match next {
                Some(k) => {
                    let l = &mut w.mobys[k];
                    l.pos = at;
                    l.set_pv_v(HOME, at);
                    let z = ai::ground_z(ctx, 1.0, at, 0);
                    let l = &mut w.mobys[k];
                    l.pos[2] = z;
                    l.set_pv_i(HELD, 0);
                    l.mode &= 0xff7f;
                    let m = &mut w.mobys[id];
                    m.set_pv_i(WAIT, 200);
                    m.set_state(2, -1);
                }
                None => w.mobys[id].set_state(4, -1),
            }
        }
        _ => {}
    }
    w.changed = true;
}
