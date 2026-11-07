//! Hrugis's tethered targets 0x0cbe (L10 `0x3f7418`, 16 placed; the decomp's `func_L10_003F5EF8`): each rides a
//! joint of another moby (`+0x260` its index, `+0x26c` the joint) 50 out along the joint's second row, held by a
//! beam back to the joint, turning about the beam; a target of the space-AI library ([`crate::moby::classes::space`]:
//! pushed and hit, `0x37e600(m, 0, 0, 0)`; Hrugis's hit handler `0x4252b0` takes only the wingmen's shots,
//! not told apart here).
//!
//! * **Its start** (state 0, once the library's ready flag is set: `0x37c8e0`, taken as set): `0x3f7630` — health 2
//!   (`+0x148`, `+0x180`), active (`0x37df20(m, 1)`, [`space::start`]), state 1, drawn from anywhere, the rows kept,
//!   its class's scale ×4.1, out of the target list (`+0x34` &= ~0x1000), `+0x14c` 0; with its leader word
//!   (`+0x268`) into the leaders' list (`0x37ceb8(m, 0x3f76e0, 0, 0)`: an empty callback); its turn a random angle
//!   (`+0x27c`).
//! * **Each tick** (state 1): `+0x14c` 0; the turn +0.667° (`0x30a8a0(turn, 0.01164)`); the joint's frame
//!   (`0x32d220`): its place there plus 50 along the joint's second row; its first row back along that row, the second
//!   across it in the level's plane (`× (0, 0, 1)`, unit), the third their cross; that frame times the turn about the
//!   first row (`0x283738(rows, q, rows)`, the quaternion `0x283958` of the turn's half-angle sine and cosine about
//!   the first row's world components); the push and the hits.
//! * **Its draw** (every tick, `0x3f76e8` on the second list, [`draw`]): two scrolling beam layers from it to the joint
//!   (FX 0x17 or, one time in two, 0x30 then 0x2f; 2 wide; alpha 0x20..0x80 on white; s 0..5 less the layer's scroll
//!   `+0x270` / `+0x274`, wrapped past −8, which runs on by 0.1..0.25 and 0.08..0.1 a frame), an additive glow strip
//!   (FX 0xe, 0x40ff0020, 4 wide at it, 3 at the joint) and two glows at the joint (`0x3089d0`, FX 0x31: 3.5..4.5 in
//!   0x40800000 and 2.5..3.5 in 0x7f7f7f at alpha 0x40..0x80, 2.5 toward the camera, random rolls).

use crate::hero::physics::{add, cross, scale, set_len, sub, unit, V3};
use crate::moby::classes::space;
use crate::moby::{mode, Ctx, MobyWorld};

pub const CLASS: i16 = 0x0cbe;

const RIDE: usize = 0x260;
const JOINT: usize = 0x26c;
const SCROLL: [usize; 2] = [0x270, 0x274];
const TURN: usize = 0x27c;

/// The moby of game index `idx` (the moby table's `idx << 8`).
fn by_index(w: &MobyWorld, idx: i32) -> Option<usize> {
    if idx < 0 { return None; }
    w.mobys.iter().position(|m| m.index as i32 == idx && m.state < 0x80)
}

/// The joint's place (`0x32d220`): the ridden moby's joint, else its own place and rows.
fn joint(w: &MobyWorld, id: usize, ctx: &Ctx) -> Option<([V3; 3], V3)> {
    let m = &w.mobys[id];
    let r = by_index(w, m.pv_i(RIDE))?;
    let j = m.pv_i(JOINT).max(0) as usize;
    Some(ctx.host.joint_matrix(&w.mobys[r], j).unwrap_or((w.mobys[r].rows, w.mobys[r].pos)))
}

/// The rows turned by `ang` about the axis `n` (`0x283958` of the quaternion (n·sin ½, cos ½), then `0x283738(rows,
/// q, rows)`: each new row the quaternion's row in the old rows' frame).
fn turned(rows: [V3; 3], n: V3, ang: f32) -> [V3; 3] {
    let (s, c) = (ang * 0.5).sin_cos();
    let (x, y, z, w) = (n[0] * s, n[1] * s, n[2] * s, c);
    let q = [
        [1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y + w * z), 2.0 * (x * z - w * y)],
        [2.0 * (x * y - w * z), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z + w * x)],
        [2.0 * (x * z + w * y), 2.0 * (y * z - w * x), 1.0 - 2.0 * (x * x + y * y)],
    ];
    q.map(|r| add(add(scale(rows[0], r[0]), scale(rows[1], r[1])), scale(rows[2], r[2])))
}

/// `0x3f7418`.
pub fn update(w: &mut MobyWorld, id: usize, ctx: &mut Ctx) {
    if w.mobys[id].pv_i(space::STARTED) == 0 {
        // `0x3f7630`.
        space::start(w, id, ctx);
        let s = w.class(CLASS).map_or(1.0, |c| c.scale) * 4.1;
        let t = ctx.game.rng.rand_angle();
        let m = &mut w.mobys[id];
        m.mode = (m.mode | mode::KEEP_ROWS) & !mode::TARGET;
        m.scale = s;
        m.set_pv_f(TURN, t);
    } else {
        let ang = crate::hero::physics::add_rot(w.mobys[id].pv_f(TURN), 0.011_635_531);
        w.mobys[id].set_pv_f(TURN, ang);
        if let Some((jr, jp)) = joint(w, id, ctx) {
            let at = add(jp, set_len(jr[1], 50.0));
            let f = set_len(jr[1], -1.0);
            let l = unit(cross(f, [0.0, 0.0, 1.0]));
            let u = cross(l, f);
            let m = &mut w.mobys[id];
            m.rows = turned([f, l, u], f, ang);
            m.pos = at;
        }
        space::hits_with(w, id, ctx, false, false, false);
        w.rebuild(id, &*ctx.host);
    }
    w.register_draw(id);
}

/// A beam quad's corners from `a` to `b`, `side` across.
fn corners(a: V3, b: V3, side: V3) -> [V3; 4] { [add(a, side), sub(a, side), add(b, side), sub(b, side)] }

/// `0x3f76e8` (module docs).
pub fn draw(w: &mut MobyWorld, host: &dyn crate::moby::MobyHost, id: usize, rng: &mut crate::rng::Rng, cam: V3, out: &mut Vec<crate::fx::FxQuad>) {
    let Some(r) = by_index(w, w.mobys[id].pv_i(RIDE)) else { return };
    let j = w.mobys[id].pv_i(JOINT).max(0) as usize;
    let end = host.joint_matrix(&w.mobys[r], j).map_or(w.mobys[r].pos, |(_, p)| p);
    let pos = w.mobys[id].pos;
    let side = set_len(cross(sub(pos, cam), sub(end, pos)), 2.0);
    for (k, (fx, step)) in [(0x30, (0.1, 0.25)), (0x2f, (0.08, 0.1))].into_iter().enumerate() {
        let fx = if rng.rand() & 1 != 0 { 0x17 } else { fx };
        let rgba = (rng.rand_range(0x20, 0x80) as u32) << 24 | 0x00ff_ffff;
        let scroll = w.mobys[id].pv_f(SCROLL[k]);
        let u = |s: f32| { let s = s - scroll; if s <= -8.0 { s + 8.0 } else { s } };
        let st = [[u(0.0), 0.0], [u(0.0), 1.0], [u(5.0), 0.0], [u(5.0), 1.0]];
        let mut next = scroll + rng.randf(step.0, step.1);
        if 8.0 <= next { next -= 8.0; }
        w.mobys[id].set_pv_f(SCROLL[k], next);
        out.push(crate::fx::FxQuad { fx, additive: false, corners: corners(pos, end, side), st, rgba: [rgba; 4] });
    }
    let (near, far) = (set_len(side, 4.0), set_len(side, 3.0));
    let st = [[0.0, 0.0], [0.0, 1.0], [1.0, 0.0], [1.0, 1.0]];
    out.push(crate::fx::FxQuad { fx: 0xe, additive: true, corners: [add(pos, near), sub(pos, near), add(end, far), sub(end, far)], st, rgba: [0x40ff_0020; 4] });
    let size = rng.randf(3.5, 4.5);
    let roll = rng.rand_angle();
    crate::fx::sprite(out, size, 2.5, roll, end, cam, 0x31, 0x4080_0000, true);
    let rgba = (rng.rand_range(0x40, 0x80) as u32) << 24 | 0x007f_7f7f;
    let size = rng.randf(2.5, 3.5);
    let roll = rng.rand_angle();
    crate::fx::sprite(out, size, 2.5, roll, end, cam, 0x31, rgba, true);
}
