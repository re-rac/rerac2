//! Tabora's small props (L08 addresses).
//!
//! * **0x090e** ([`update_090e`], `0x3d1960`; the decomp's `func_L08_003D0508`): a platform something else moves.
//!   Its set-up keeps its place (`+0x90`), sets its block's flags bit 1 (`+0x8c`) and `+0x3e` = 15, points its pvar
//!   words at its platform block (`+0x50`) and health block (`+0x20`), and gives it mode 0x20. Each tick it writes
//!   the tick's move into its platform block (`0x318730`) and keeps its new place.
//! * **0x0988** ([`update_0988`], `0x3d5310`, also L09 / L19): its glow pulses (`0x335a18` = L01 `0x31cf58`, as
//!   crate::moby::classes::oozla::update_0fa0) from 0x80aaaad2 to 0x80396464, its phase on 0.0611 a tick.
//! * **0x0e5b** ([`update_0e5b`], `0x40ff80`; the decomp's v2.00 `func_US2_L08_0040FF80`): the item 0x15's pickup.
//!   It is gone once item 13 is had. Otherwise it is raised 0.75, scaled 1.5 against its class, and spins 4.5° a
//!   tick. With Ratchet in cuboid `+4` it starts scene 0x5b; when the scene is over it puts Ratchet at cuboid `+8`'s
//!   place and angles, gives item 0x15 and message 0x122b, and goes.
//!
//! Not ported: 0x0e5b's sparkle (`0x40fce0` = L01 `0x3cff58`), its hiding during the scene, the flag
//! `0x1b270a`, and the autosave (`0x319bc8` = L00 `0x30d658`).

use crate::moby::{platform, Ctx, MobyWorld};

pub const PLATFORM: i16 = 0x090e;
pub const GLOWING: i16 = 0x0988;
pub const PICKUP: i16 = 0x0e5b;

pub fn update_090e(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.state == 0 {
        let pos = m.pos;
        m.set_pv_v(0x90, pos);
        m.set_pv_i(0x8c, m.pv_i(0x8c) | 1);
        m.set_pv_s(0x3e, 15);
        m.set_pv_i(8, 0x50);
        m.set_pv_i(0, 0x20);
        m.set_state(1, -1);
        m.mode |= 0x20;
    }
    let mv = crate::hero::physics::sub(m.pos, m.pv_v(0x90));
    let rot = m.rot;
    platform::write(m, 0x50, mv, rot, rot);
    let pos = m.pos;
    m.set_pv_v(0x90, pos);
}

pub fn update_0988(w: &mut MobyWorld, id: usize, _ctx: &mut Ctx) {
    let m = &mut w.mobys[id];
    if m.pvars.len() < 0x10 { return; }
    let phase = m.pv_f(0xc);
    m.set_pv_f(0xc, crate::hero::physics::add_rot(phase, 0.061_086_536));
    m.glow = crate::hud::lerp_rgba(phase.sin() * 0.5 + 0.5, 0x80aa_aad2, 0x8039_6464);
}

pub fn update_0e5b(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    match w.mobys[id].state {
        0 => {
            if ctx.game.items.get(13).is_some_and(|&b| b != 0) { return w.delete(id); }
            let s = w.class(PICKUP).map_or(1.0, |c| c.scale) * 1.5;
            let m = &mut w.mobys[id];
            m.pos[2] += 0.75;
            m.scale = s;
            m.set_state(1, -1);
        }
        1 => {
            let m = &mut w.mobys[id];
            m.rot[2] = crate::hero::physics::add_rot(m.rot[2], 0.078_539_83);
            if ctx.level.in_cuboid(m.pv_i(4), ctx.hero.pos) && ctx.game.scenes.queue.push(0x5b) {
                ctx.game.scenes.start_queue(ctx.hero.hp);
                m.set_state(2, -1);
            }
        }
        2 => {
            if ctx.game.scenes.in_scene() { return; }
            if let Some((pos, rot)) = ctx.level.cuboid_place(w.mobys[id].pv_i(8)) { ctx.hero.teleport(pos, rot); }
            ctx.game.get_item(0x15);
            ctx.game.messages.push(crate::msgbox::Request::Text { text: 0x122b, ticks: None });
            w.delete(id);
        }
        _ => {}
    }
}
