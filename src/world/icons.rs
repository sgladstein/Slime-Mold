//! **Line icons** for the UI: each drawn in a 24 x 24 box as strokes 2
//! units wide with round ends (the look of modern line-icon sets), or
//! filled shapes, then rasterized with `tiny-skia` to coverage at the size
//! asked. Drawn for this game; no icon set's paths are copied.
//!
//! A path is a string of commands: `M x y` move, `L x y` line, `C x1 y1 x2
//! y2 x y` curve, `Q x1 y1 x y` quadratic, `Z` close, `O cx cy r` a circle,
//! `R x y w h` a rectangle. A leading `F` fills the path instead of
//! stroking it.

use tiny_skia::{FillRule, LineCap, LineJoin, Paint, PathBuilder, Pixmap, Stroke, Transform};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    Move,
    Stop,
    Hold,
    Attack,
    Burn,
    Wall,
    Turret,
    Bait,
    Recruit,
    Upgrade,
    Rifle,
    Flamer,
    Grenade,
    Idle,
    Menu,
    Pause,
    Fast,
    Crystal,
    Soldier,
    Mould,
    Alert,
    Heart,
    Clock,
    Shield,
    Eye,
    Close,
    Check,
    Target,
    Spore,
    Group,
}

impl Icon {
    fn paths(self) -> &'static [&'static str] {
        match self {
            Icon::Move => &["M5 19 L19 5", "M10 5 L19 5 L19 14"],
            Icon::Stop => &["M6 6 L18 6 L18 18 L6 18 Z"],
            Icon::Hold => &["M12 3 L19 6 L19 11 C19 16 16 19.5 12 21 C8 19.5 5 16 5 11 L5 6 Z", "M12 8 L12 15"],
            Icon::Attack => &["O12 12 7", "M12 2 L12 7", "M12 17 L12 22", "M2 12 L7 12", "M17 12 L22 12"],
            Icon::Burn => &["M12 21 C8 21 5.5 18.5 5.5 15 C5.5 11 9 9.5 9 5 C12 7 14 9 13.5 12 C15 11.5 16 10 16 8.5 C18 10.5 18.5 13 18.5 15 C18.5 18.5 16 21 12 21 Z"],
            Icon::Wall => &["M3 6 L21 6 L21 18 L3 18 Z", "M3 12 L21 12", "M9 6 L9 12", "M15 6 L15 12", "M6 12 L6 18", "M12 12 L12 18", "M18 12 L18 18"],
            Icon::Turret => &["M7 21 L17 21", "M8.5 21 L9.5 11 L14.5 11 L15.5 21", "M12 11 L12 7", "M12 7 L20 4"],
            Icon::Bait => &["M12 3 C12 3 6 10 6 14.5 C6 18 8.7 21 12 21 C15.3 21 18 18 18 14.5 C18 10 12 3 12 3 Z"],
            Icon::Recruit => &["O9 8 3.5", "M2.5 20 C2.5 15.5 5.5 13.5 9 13.5 C12.5 13.5 15.5 15.5 15.5 20", "M19 8 L19 14", "M16 11 L22 11"],
            Icon::Upgrade => &["M6 13 L12 7 L18 13", "M6 19 L12 13 L18 19"],
            Icon::Rifle => &["M2 14 L5 10 L22 10", "M5 10 L5 13 L11 13 L12.5 17 L15 17 L14.5 13 L19 13 L19 10"],
            // A nozzle and its flame (a box and a flame read as a camera).
            Icon::Flamer => &["M2 13 L8 13", "M8 10.5 L12 11.5 L12 14.5 L8 15.5 Z", "M13 13 C15 9 17 11 18 6 C21 9 22.5 12.5 20.5 15.5 C19 18 15.5 17.5 13 13 Z"],
            Icon::Grenade => &["O12 14 6.5", "M10 7.5 L10 5 L14 5 L14 7.5", "M14 5 L17 3"],
            // A soldier and a "z" (a crescent read as night).
            Icon::Idle => &["O9 9 3.5", "M2.5 21 C2.5 16.5 5.5 14.5 9 14.5 C12.5 14.5 15.5 16.5 15.5 21", "M15.5 3 L21 3 L15.5 9 L21 9"],
            Icon::Menu => &["M4 6 L20 6", "M4 12 L20 12", "M4 18 L20 18"],
            Icon::Pause => &["M8 5 L8 19", "M16 5 L16 19"],
            Icon::Fast => &["FM4 6 L12 12 L4 18 Z", "FM12 6 L20 12 L12 18 Z"],
            Icon::Crystal => &["M12 2 L20.5 7 L20.5 17 L12 22 L3.5 17 L3.5 7 Z", "M12 2 L12 22", "M3.5 7 L20.5 17", "M20.5 7 L3.5 17"],
            Icon::Soldier => &["O12 7 4", "M4 21 C4 16 7.5 13.5 12 13.5 C16.5 13.5 20 16 20 21"],
            Icon::Mould => &["M5 15 C3 12 5 8 8 8.5 C9 5 14 4 15.5 7.5 C19 7 21 11 19 14 C21 17 17 20.5 14 18.5 C12 21 7 20.5 7 17.5 C5.5 17.5 4.5 16.5 5 15 Z", "O10.5 12.5 1", "O14.5 11.5 1"],
            Icon::Alert => &["M12 3 L22 20 L2 20 Z", "M12 9.5 L12 14", "M12 17 L12 17.2"],
            Icon::Heart => &["FM12 20.5 C12 20.5 3 15 3 9 C3 6.2 5.2 4 7.8 4 C9.6 4 11.2 5 12 6.5 C12.8 5 14.4 4 16.2 4 C18.8 4 21 6.2 21 9 C21 15 12 20.5 12 20.5 Z"],
            Icon::Clock => &["O12 12 9", "M12 7 L12 12 L15.5 14"],
            Icon::Shield => &["M12 3 L19 6 L19 11 C19 16 16 19.5 12 21 C8 19.5 5 16 5 11 L5 6 Z"],
            Icon::Eye => &["M2 12 C4.5 7 8 5 12 5 C16 5 19.5 7 22 12 C19.5 17 16 19 12 19 C8 19 4.5 17 2 12 Z", "O12 12 3"],
            Icon::Close => &["M6 6 L18 18", "M18 6 L6 18"],
            Icon::Check => &["M5 12.5 L10 17.5 L19 7"],
            Icon::Target => &["O12 12 8", "O12 12 3"],
            Icon::Spore => &["O12 12 3", "O5 6 1.6", "O19 6 1.6", "O5 18 1.6", "O19 18 1.6", "O12 3 1.2", "O12 21 1.2"],
            Icon::Group => &["O8.5 8 3", "O16 9 2.5", "M2.5 19.5 C2.5 15.5 5 13.5 8.5 13.5 C12 13.5 14.5 15.5 14.5 19.5", "M15 13.5 C18.5 13.5 21.5 15 21.5 19"],
        }
    }
}

fn parse(d: &str) -> Option<tiny_skia::Path> {
    let mut pb = PathBuilder::new();
    let toks: Vec<String> = {
        let mut v = Vec::new();
        let mut cur = String::new();
        for ch in d.chars() {
            if ch.is_ascii_alphabetic() {
                if !cur.trim().is_empty() {
                    v.push(cur.trim().to_string());
                }
                v.push(ch.to_string());
                cur.clear();
            } else if ch == ' ' || ch == ',' {
                if !cur.trim().is_empty() {
                    v.push(cur.trim().to_string());
                }
                cur.clear();
            } else {
                cur.push(ch);
            }
        }
        if !cur.trim().is_empty() {
            v.push(cur.trim().to_string());
        }
        v
    };
    let mut i = 0;
    let num = |i: &mut usize| -> f32 {
        let v = toks.get(*i).and_then(|t| t.parse::<f32>().ok()).unwrap_or(0.0);
        *i += 1;
        v
    };
    while i < toks.len() {
        let c = toks[i].clone();
        i += 1;
        match c.as_str() {
            "M" => {
                let (x, y) = (num(&mut i), num(&mut i));
                pb.move_to(x, y);
            }
            "L" => {
                let (x, y) = (num(&mut i), num(&mut i));
                pb.line_to(x, y);
            }
            "C" => {
                let v: Vec<f32> = (0..6).map(|_| num(&mut i)).collect();
                pb.cubic_to(v[0], v[1], v[2], v[3], v[4], v[5]);
            }
            "Q" => {
                let v: Vec<f32> = (0..4).map(|_| num(&mut i)).collect();
                pb.quad_to(v[0], v[1], v[2], v[3]);
            }
            "Z" => pb.close(),
            "O" => {
                let (x, y, r) = (num(&mut i), num(&mut i), num(&mut i));
                pb.push_circle(x, y, r);
            }
            "R" => {
                let v: Vec<f32> = (0..4).map(|_| num(&mut i)).collect();
                if let Some(r) = tiny_skia::Rect::from_xywh(v[0], v[1], v[2], v[3]) {
                    pb.push_rect(r);
                }
            }
            _ => {}
        }
    }
    pb.finish()
}

/// The icon's coverage, `side` x `side`, 0-255.
pub fn rasterize(icon: Icon, side: u32) -> Vec<u8> {
    let mut pm = Pixmap::new(side, side).expect("icon size");
    let s = side as f32 / 24.0;
    let t = Transform::from_scale(s, s);
    let mut paint = Paint::default();
    paint.set_color_rgba8(255, 255, 255, 255);
    paint.anti_alias = true;
    // Strokes a little heavier at small sizes, so a 16-pixel icon still
    // reads (2 units is 1.3 pixels there).
    let width = if side < 20 { 2.3 } else { 2.0 };
    let stroke = Stroke { width, line_cap: LineCap::Round, line_join: LineJoin::Round, ..Stroke::default() };
    for d in icon.paths() {
        let (fill, d) = match d.strip_prefix('F') {
            Some(rest) => (true, rest),
            None => (false, *d),
        };
        if let Some(path) = parse(d) {
            if fill {
                pm.fill_path(&path, &paint, FillRule::Winding, t, None);
            } else {
                pm.stroke_path(&path, &paint, &stroke, t, None);
            }
        }
    }
    pm.pixels().iter().map(|p| p.alpha()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_icon_draws_something() {
        use Icon::*;
        for i in [Move, Stop, Hold, Attack, Burn, Wall, Turret, Bait, Recruit, Upgrade, Rifle, Flamer, Grenade, Idle, Menu, Pause, Fast, Crystal, Soldier, Mould, Alert, Heart, Clock, Shield, Eye, Close, Check, Target, Spore, Group] {
            let c = rasterize(i, 24);
            let on = c.iter().filter(|&&a| a > 128).count();
            assert!(on > 15 && on < 24 * 24 / 2 + 150, "{i:?}: {on} pixels on");
        }
    }
}
