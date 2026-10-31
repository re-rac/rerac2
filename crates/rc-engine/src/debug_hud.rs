//! The inspect overlay (dev): with `RC_DEBUG_HUD=1` (`show` / `mobys` / `sound`: shown from the start on that page), F3
//! shows or hides a column of live data over the HUD, F4 cycles its pages, F8 makes a repro line.
//!
//! * **Player**: the level, tick and scene mode; Ratchet's position, facing, state, speed and ground; the camera; the
//!   moby the camera looks at; the scene playing.
//! * **Mobys**: a label over every moby within [`LABEL_RANGE`] of Ratchet (id, class, state) and, for the one the camera
//!   looks at, its position, mode, update / draw distances, piece, health and first pvar words.
//! * **Sound**: every busy sound slot (owner, sound, loop, volume, slot state, sound instance).
//! * **F8**: `RC_LEVEL=… RC_SCENES=0 RC_HERO_AT=x,y,z,yaw RC_GIVE_ITEMS=…` (the items had, the hand item last so it is
//!   asked into the hand): printed, copied to the clipboard where a copier exists, and shown for five seconds.
//!
//! The pages only read the game; crate::play draws them at the end of the HUD in the HUD's font. Without the switch
//! nothing is added.

use bevy::prelude::*;

/// The overlay's state.
#[derive(Resource, Default)]
pub struct DebugHud {
    pub on: bool,
    pub page: Page,
    /// The last repro line and the game tick it stops showing at.
    pub note: Option<(String, u64)>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Page {
    #[default]
    Player,
    Mobys,
    Sound,
}

impl Page {
    fn next(self) -> Page {
        match self {
            Page::Player => Page::Mobys,
            Page::Mobys => Page::Sound,
            Page::Sound => Page::Player,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Page::Player => "PLAYER",
            Page::Mobys => "MOBYS",
            Page::Sound => "SOUND",
        }
    }
}

/// Mobys this close to Ratchet get a label on the mobys page.
pub const LABEL_RANGE: f32 = 15.0;
/// The game picture the HUD draws in.
const W: f32 = 512.0;
const H: f32 = 416.0;
/// The view's horizontal tangent and the NTSC ratio of its vertical one (crate::game_camera).
const TAN: f32 = crate::game_camera::TAN_HALF_FOV_X;
const TAN_Y_RATIO: f32 = 0.775;

pub struct DebugHudPlugin;

impl Plugin for DebugHudPlugin {
    fn build(&self, app: &mut App) {
        match std::env::var("RC_DEBUG_HUD").as_deref().map(str::trim) {
            Ok("1") => { app.init_resource::<DebugHud>().add_systems(Update, keys); }
            // Shown from the start on a page (headless captures, which cannot press F3 / F4).
            Ok(v @ ("show" | "mobys" | "sound")) => {
                let page = match v { "mobys" => Page::Mobys, "sound" => Page::Sound, _ => Page::Player };
                app.insert_resource(DebugHud { on: true, page, ..default() }).add_systems(Update, keys);
            }
            _ => {}
        }
    }
}

fn keys(keys: Res<ButtonInput<KeyCode>>, mut d: ResMut<DebugHud>, play: Option<Res<crate::play::Play>>) {
    if keys.just_pressed(KeyCode::F3) { d.on = !d.on; }
    if keys.just_pressed(KeyCode::F4) { d.page = d.page.next(); }
    if keys.just_pressed(KeyCode::F8) {
        let Some(p) = play else { return };
        let line = repro_line(&p);
        println!("repro: {line}");
        let copied = copy_to_clipboard(&line);
        d.note = Some((if copied { format!("COPIED: {line}") } else { line }, p.ticks + 300));
    }
}

/// The repro line for this moment (module doc).
pub fn repro_line(p: &crate::play::Play) -> String {
    let h = &p.hero;
    let hand = h.items.item;
    let mut items: Vec<usize> = (1..p.game.items.len()).filter(|&i| p.game.items[i] != 0 && i as i32 != hand).collect();
    if 0 < hand && p.game.items.get(hand as usize).is_some_and(|&b| b != 0) { items.push(hand as usize); }
    let items: Vec<String> = items.iter().map(|i| i.to_string()).collect();
    let mut s = format!("RC_LEVEL={} RC_SCENES=0 RC_HERO_AT={:.2},{:.2},{:.2},{:.3}", crate::level_load::level_index(), h.pos[0], h.pos[1], h.pos[2], h.rot[2]);
    if !items.is_empty() { s.push_str(&format!(" RC_GIVE_ITEMS={}", items.join(","))); }
    s
}

/// The system's clipboard copier (`pbcopy`, `clip`, `wl-copy` or `xclip`); false when none ran.
fn copy_to_clipboard(s: &str) -> bool {
    use std::io::Write;
    let tools: &[(&str, &[&str])] = if cfg!(target_os = "macos") {
        &[("pbcopy", &[])]
    } else if cfg!(windows) {
        &[("clip", &[])]
    } else {
        &[("wl-copy", &[]), ("xclip", &["-selection", "clipboard"])]
    };
    for (cmd, args) in tools {
        let Ok(mut child) = std::process::Command::new(cmd).args(*args).stdin(std::process::Stdio::piped()).spawn() else { continue };
        let ok = child.stdin.take().is_some_and(|mut i| i.write_all(s.as_bytes()).is_ok());
        if ok && child.wait().is_ok_and(|st| st.success()) { return true; }
    }
    false
}

fn world(p: &crate::play::Play) -> Option<&rc_game::moby::MobyWorld> { p.mobys.as_ref().map(|m| &m.world) }
fn hero_moby(p: &crate::play::Play) -> Option<usize> { p.mobys.as_ref().and_then(|m| m.hero_moby) }

/// The moby nearest the camera's forward line: within 40 and 10° of it, the smallest angle first (Ratchet's own moby
/// left out). (id, distance).
fn looked_at(p: &crate::play::Play) -> Option<(usize, f32)> {
    let (eye, fwd) = (p.camera.pos, p.camera.rows[0]);
    let hero = hero_moby(p);
    let mut best: Option<(usize, f32, f32)> = None;
    for (id, m) in world(p)?.mobys.iter().enumerate() {
        if m.state >= 0x80 || Some(id) == hero { continue; }
        let d = [m.pos[0] - eye[0], m.pos[1] - eye[1], m.pos[2] - eye[2]];
        let dist = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        if !(0.5..=40.0).contains(&dist) { continue; }
        let cos = (d[0] * fwd[0] + d[1] * fwd[1] + d[2] * fwd[2]) / dist;
        if cos < 10f32.to_radians().cos() { continue; }
        if best.is_none_or(|b| cos > b.2) { best = Some((id, dist, cos)); }
    }
    best.map(|(id, d, _)| (id, d))
}

fn moby_text(p: &crate::play::Play, id: usize) -> String {
    world(p).and_then(|w| w.mobys.get(id)).map_or_else(|| format!("{id}"), |m| format!("{id} CLASS {:#X} UID {} STATE {:#X}", m.o_class, m.uid, m.state))
}

/// The player page.
pub fn lines(p: &crate::play::Play) -> Vec<String> {
    let h = &p.hero;
    let yaw = h.rot[2];
    let d = h.disp;
    let moved = [(d[0] * d[0] + d[1] * d[1]).sqrt() * 60.0, (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt() * 60.0];
    let (cp, f) = (p.camera.pos, p.camera.rows[0]);
    let (cam_yaw, cam_pitch) = (f[1].atan2(f[0]), f[2].clamp(-1.0, 1.0).asin());
    let mut out = vec![
        format!("LEVEL {}  TICK {}  SCENE MODE {}", crate::level_load::level_index(), p.ticks, p.game.scenes.mode),
        format!("POS {:.2} {:.2} {:.2}", h.pos[0], h.pos[1], h.pos[2]),
        format!("YAW {:.3} RAD  {:.1} DEG", yaw, yaw.to_degrees()),
        format!("STATE {:#X}  GROUP {:#X}  BODY {}  AIR {}", h.state, h.group, h.mode, h.air_ticks),
        format!("VEL {:.3} {:.3} {:.3}", h.vel[0], h.vel[1], h.vel[2]),
        format!("SPEED {:.3}/TICK {:.2} U/S  TARGET {:.2} U/S", h.speed, h.speed * 60.0, h.target_speed * 60.0),
        format!("MOVED {:.2} U/S  3D {:.2} U/S", moved[0], moved[1]),
        format!("GROUND {}", h.ground_moby.map_or_else(|| "WORLD".to_string(), |id| moby_text(p, id))),
        format!("CAM {:.2} {:.2} {:.2}  YAW {:.3}  PITCH {:.3}", cp[0], cp[1], cp[2], cam_yaw, cam_pitch),
        format!("LOOK {}", looked_at(p).map_or_else(|| "-".to_string(), |(id, d)| format!("{} DIST {d:.1}", moby_text(p, id)))),
    ];
    if let Some(s) = p.game.scenes.player.as_ref() { out.push(format!("SCENE {}  TICK {}", s.id, s.tick)); }
    out
}

/// The mobys page's column: the count labelled and the looked-at moby's details.
pub fn moby_lines(p: &crate::play::Play, labelled: usize) -> Vec<String> {
    let mut out = vec![format!("{labelled} MOBYS WITHIN {LABEL_RANGE:.0}  (ID CLASS STATE)")];
    let (Some(w), Some((id, d))) = (world(p), looked_at(p)) else {
        out.push("LOOK -".into());
        return out;
    };
    let m = &w.mobys[id];
    out.push(format!("LOOK {} DIST {d:.1}", moby_text(p, id)));
    out.push(format!("POS {:.2} {:.2} {:.2}  ROT Z {:.3}  SCALE {:.3}", m.pos[0], m.pos[1], m.pos[2], m.rot[2], m.scale));
    out.push(format!("MODE {:#06X}  SUB {}  COLL {}  UPDATE {:#X}  DRAW {:#X}  DRAWN {}", m.mode, m.sub_state, m.has_collision, m.update_dist, m.draw_dist, m.drawn));
    out.push(format!("GROUP {}  PIECE {}  ALPHA {:#X}  PORTED {}", m.group, m.piece.map_or_else(|| "-".to_string(), |k| k.to_string()), m.alpha, m.update.is_some()));
    if let Some((hp, _)) = rc_game::moby::aim::health(w, id) { out.push(format!("HEALTH {hp:.2}")); }
    for (k, row) in m.pvars.chunks(4).take(16).collect::<Vec<_>>().chunks(4).enumerate() {
        let words: Vec<String> = row.iter().map(|c| format!("{:08X}", u32::from_le_bytes([c[0], c.get(1).copied().unwrap_or(0), c.get(2).copied().unwrap_or(0), c.get(3).copied().unwrap_or(0)]))).collect();
        out.push(format!("+{:02X} {}", k * 16, words.join(" ")));
    }
    out
}

/// A point on the game picture (512 × 416) from the camera, `None` behind it.
fn project(eye: [f32; 3], rows: [[f32; 3]; 3], at: [f32; 3]) -> Option<[f32; 2]> {
    let d = [at[0] - eye[0], at[1] - eye[1], at[2] - eye[2]];
    let dot = |r: [f32; 3]| r[0] * d[0] + r[1] * d[1] + r[2] * d[2];
    let f = dot(rows[0]);
    if f < 0.5 { return None; }
    let x = W * 0.5 - dot(rows[1]) / f * (W * 0.5 / TAN);
    let y = H * 0.5 - dot(rows[2]) / f * (H * 0.5 / (TAN * TAN_Y_RATIO));
    ((0.0..W).contains(&x) && (0.0..H).contains(&y)).then_some([x, y])
}

/// The mobys page's labels: (screen x, y in the 512 × 416 picture, text) over every moby within [`LABEL_RANGE`] of
/// Ratchet in front of the camera, 1 above its position.
pub fn moby_labels(p: &crate::play::Play) -> Vec<(f32, f32, String)> {
    let pos = p.hero.pos;
    let hero = hero_moby(p);
    let mut out = Vec::new();
    let Some(w) = world(p) else { return out };
    for (id, m) in w.mobys.iter().enumerate() {
        if m.state >= 0x80 || Some(id) == hero { continue; }
        let d = ((m.pos[0] - pos[0]).powi(2) + (m.pos[1] - pos[1]).powi(2) + (m.pos[2] - pos[2]).powi(2)).sqrt();
        if d > LABEL_RANGE { continue; }
        let Some(c) = project(p.camera.pos, p.camera.rows, [m.pos[0], m.pos[1], m.pos[2] + 1.0]) else { continue };
        out.push((c[0], c[1], format!("{id} C{:X} S{:X}", m.o_class, m.state)));
    }
    // Labels that would overlap one placed before (about 5 pixels a character, rows of 10) move down a row until free.
    out.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.total_cmp(&b.0)));
    let mut placed: Vec<(f32, f32, f32)> = Vec::new();
    for l in &mut out {
        let w = 5.0 * l.2.len() as f32;
        while placed.iter().any(|&(x0, x1, y)| (l.1 - y).abs() < 10.0 && l.0 < x1 && x0 < l.0 + w) { l.1 += 10.0; }
        placed.push((l.0, l.0 + w, l.1));
    }
    out
}

/// The sound page: every busy slot.
pub fn sound_lines(a: &rc_game::audio::AudioSystem, p: &crate::play::Play) -> Vec<String> {
    use rc_game::audio::voices::state;
    let hero = hero_moby(p);
    let mut out = Vec::new();
    for (k, s) in a.slots.slots.iter().enumerate() {
        if s.state == state::FREE { continue; }
        let owner = match s.owner {
            Some(o) if Some(o.id as usize) == hero => "RATCHET".to_string(),
            Some(o) => world(p).and_then(|w| w.mobys.get(o.id as usize)).map_or_else(|| format!("MOBY {}", o.id), |m| format!("{} C{:X}", o.id, m.o_class)),
            None => "-".to_string(),
        };
        let inst = if s.instance() >= 0 { format!("  INST {}", s.instance()) } else { String::new() };
        let lp = if s.looped() { " LOOP" } else { "" };
        out.push(format!("{k:2}: {owner}  SND {}{lp}  VOL {:#X}  ST {}{inst}", s.class_index, s.volume(), s.state));
    }
    let n = out.len();
    if out.is_empty() { out.push("NO SOUND SLOT BUSY".into()); }
    out.insert(0, format!("{n} SOUND SLOTS BUSY"));
    out
}

/// The overlay's text for this frame (the column, the labels, the repro note) as HUD quads in `glyphs`' font
/// (`slot`), each line with a one-pixel shadow.
pub fn quads(d: &DebugHud, p: &crate::play::Play, audio: Option<&rc_game::audio::AudioSystem>, glyphs: &[rc_formats::disc::boot::Glyph; 256], slot: usize) -> Vec<rc_game::gui::Quad> {
    const SCALE: f32 = 0.5;
    let mut out = Vec::new();
    let mut text = |x: f32, y: f32, l: &str, rgba: [u8; 4]| {
        let t = l.as_bytes();
        rc_game::gui::text(&mut out, glyphs, slot, t, x + 1.0, y + 1.0, SCALE, [0, 0, 0, 0x80]);
        rc_game::gui::text(&mut out, glyphs, slot, t, x, y, SCALE, rgba);
    };
    let mut col = vec![format!("DEBUG {}  (F3 HIDE  F4 PAGE  F8 REPRO)", d.page.name())];
    match d.page {
        Page::Player => col.extend(lines(p)),
        Page::Mobys => {
            let labels = moby_labels(p);
            for (x, y, l) in &labels { text(*x - 12.0, *y, l, [0xff, 0xff, 0x60, 0x80]); }
            col.extend(moby_lines(p, labels.len()));
        }
        Page::Sound => match audio {
            Some(a) => col.extend(sound_lines(a, p)),
            None => col.push("NO AUDIO (RC_AUDIO=0)".into()),
        },
    }
    for (i, l) in col.iter().enumerate() { text(12.0, 48.0 + 10.0 * i as f32, l, if i == 0 { [0xff, 0x80, 0x80, 0x80] } else { [0xff, 0xff, 0xff, 0x80] }); }
    if let Some((n, _)) = d.note.as_ref().filter(|(_, u)| p.ticks < *u) { text(12.0, H - 24.0, n, [0x60, 0xff, 0x60, 0x80]); }
    out
}
