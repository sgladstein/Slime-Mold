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
    // The gun cards (`mould::rounds`, the loadout panel, 2026-11-14): one
    // glyph a card, drawn for this game after the look of a spell deck.
    Blast,
    Gas,
    Germ,
    Bomb,
    Wide,
    Scatter,
    Quiet,
    Delay,
    Thick,
    Pierce,
    Split,
    // The gun kinds (`mould::guns`, 2026-11-14 16:46Z): the rifle is
    // `Rifle` above.
    Carbine,
    LongGun,
    Scattergun,
    // The bullets (`mould::bullets`, owner 16:46Z, and the roguelike
    // shop's "Guns and bullets should have icons"): a plain bullet, and
    // the three sets that lean its rolls.
    Bullet,
    Slug,
    Needle,
    Buckshot,
    // The carrier cards (`rounds::Card::Trigger`, `Card::Expire`,
    // 2026-11-16): a bullet with another inside it.
    Carry,
    CarryStop,
    // The shove cards (`rounds::Card::Heave`, `Corral`, `Pull`, 2026-11-16).
    Heave,
    Corral,
    Pull,
    // Multicast (`rounds::Card::Multicast`, 2026-11-16): two bullets side
    // by side from one shot.
    Multicast,
    // Seek (`rounds::Card::Seek`, 2026-11-16): a bullet's path that runs
    // straight, then bends onto a blob.
    Seek,
    // The Mind bullets (`rounds::Card::Blind` and on, 2026-11-16): how
    // the slime senses.
    Blind,
    Sour,
    Spin,
    // The readers (`rounds::Card::Shatter`, `Detonate`, 2026-11-16): what
    // they read, broken.
    Shatter,
    Detonate,
    // The route family (`rounds::Card::Flare`, 2026-11-17): a flare
    // burning on its stick.
    Flare,
    // The element family (`rounds::Card::Torch`, 2026-11-17): a flame
    // rising out of the grass, which is what it lights (the burn order's
    // icon is a flame alone).
    Torch,
    // The element family's Tinder (2026-11-17): dry brush laid along a
    // bullet's line, the bullet at its head.
    Tinder,
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
            // A burst with a hot core.
            Icon::Blast => &["M12 2 L14 8.5 L20.5 6 L16.5 11.5 L22 15 L15 15.5 L14.5 22 L11 16.5 L5.5 20.5 L7.5 14 L2 11.5 L8.5 9.5 L6.5 3.5 Z", "FO12 12.5 2.2"],
            // A cloud and the wisps it trails.
            Icon::Gas => &["M6.5 15 C3.5 15 3 10.5 6.5 10 C6.5 6 11.5 5 13 8 C14.5 5.5 19.5 6.5 19 10.5 C22 11 21.5 15 18.5 15 Z", "M5 18.5 L13 18.5", "M9 21.5 L19 21.5"],
            // A cell with its spikes.
            // A core and the mould it throws: dots flying straight out.
            Icon::Bomb => &["FO12 12 3.5", "O12 12 6", "M12 1.5 L12 4", "M12 20 L12 22.5", "M1.5 12 L4 12", "M20 12 L22.5 12", "FO5 5 1.3", "FO19 5 1.3", "FO5 19 1.3", "FO19 19 1.3"],
            Icon::Germ => &["O12 12 5", "FO10.5 11 1.2", "FO13.5 13.5 1", "M12 2.5 L12 7", "M12 17 L12 21.5", "M2.5 12 L7 12", "M17 12 L21.5 12", "M5.3 5.3 L8.5 8.5", "M15.5 15.5 L18.7 18.7", "M18.7 5.3 L15.5 8.5", "M8.5 15.5 L5.3 18.7"],
            // Two arrows pushing out from a centre.
            Icon::Wide => &["FO12 12 2.4", "M2.5 12 L8 12", "M5 9 L2.5 12 L5 15", "M21.5 12 L16 12", "M19 9 L21.5 12 L19 15", "M12 2.5 L12 6", "M12 18 L12 21.5"],
            Icon::Scatter => &["FO12 12 2.3", "FO5 5.5 2", "FO19 6.5 2", "FO5.5 18.5 2", "FO18.5 18 2", "FO12 3 1.3", "FO21 12.5 1.3", "FO3 12 1.3"],
            // A speaker, struck through: no alarm.
            Icon::Quiet => &["M3.5 9 L7.5 9 L12.5 4.5 L12.5 19.5 L7.5 15 L3.5 15 Z", "M16 9 L21 15", "M21 9 L16 15"],
            Icon::Delay => &["M6 3 L18 3", "M6 21 L18 21", "M7.5 3 C7.5 8.5 12 9.5 12 12 C12 14.5 7.5 15.5 7.5 21", "M16.5 3 C16.5 8.5 12 9.5 12 12 C12 14.5 16.5 15.5 16.5 21", "FM9.5 19.5 L12 16.5 L14.5 19.5 Z"],
            // Feet on a plate: it waits for the mould to crowd it.
            Icon::Thick => &["M2.5 19 L21.5 19", "M5.5 19 L5.5 15.5 L18.5 15.5 L18.5 19", "M12 3 L12 11", "M8.5 7.5 L12 11 L15.5 7.5", "FO5 5 1.4", "FO19 5 1.4"],
            // A shot through a wall and on.
            Icon::Pierce => &["M2 12 L21.5 12", "M17 7.5 L21.5 12 L17 16.5", "M9 4 L9 9", "M9 15 L9 20"],
            // A shot that forks.
            Icon::Split => &["M2 12 L10 12", "M10 12 L20.5 5", "M10 12 L21.5 12", "M10 12 L20.5 19", "M17 4 L20.5 5 L19 8.5", "M19 16 L20.5 19 L17 20"],
            // A short gun and two lines off its muzzle: fast and short.
            Icon::Carbine => &["M3 13 L5 9.5 L16 9.5", "M5 9.5 L5 12.5 L9.5 12.5 L10.5 16.5 L12.5 16.5 L12 12.5 L16 12.5 L16 9.5", "M18.5 9.5 L22 9.5", "M18.5 12.5 L21 12.5"],
            // A long barrel under a scope: slow and far.
            Icon::LongGun => &["M1.5 15.5 L3.5 12 L22.5 12", "M3.5 12 L3.5 15 L9 15 L10 18.5 L12 18.5 L11.5 15 L15 15 L15 12", "M7 7.5 L13.5 7.5 L13.5 9.5 L7 9.5 Z", "M10 9.5 L10 12"],
            // A wide muzzle and the fan of shot leaving it.
            // A round-nosed bullet on its case.
            Icon::Bullet => &["M4 9 L13 9 C17.5 9 20.5 10.5 21 12 C20.5 13.5 17.5 15 13 15 L4 15 Z", "M8.5 9 L8.5 15"],
            // A short, fat, flat-nosed slug: heavy.
            Icon::Slug => &["M5 6.5 L14 6.5 C17.5 6.5 19.5 8 19.5 10 L19.5 14 C19.5 16 17.5 17.5 14 17.5 L5 17.5 Z", "M9.5 6.5 L9.5 17.5"],
            // A long dart with fins: light, fast and far.
            Icon::Needle => &["M5 12 L22 12", "M17 10.5 L22 12 L17 13.5", "M2 9 L6 12 L2 15"],
            // A spray of pellets.
            Icon::Buckshot => &["FO6.5 12 2", "FO12 7 2", "FO12.5 17 2", "FO18 10.5 2", "FO18.5 15.5 1.6"],
            // A bullet with a payload inside, letting it out of its nose.
            Icon::Carry => &["M2 8 L10 8 C13.5 8 15.5 10 16 12 C15.5 14 13.5 16 10 16 L2 16 Z", "FO8 12 2", "M18.5 9 L21.5 6", "M19 12 L22.5 12", "M18.5 15 L21.5 18"],
            // The same, stopped at a wall: it lets it out where it stops.
            Icon::CarryStop => &["M2 8 L10 8 C13.5 8 15.5 10 16 12 C15.5 14 13.5 16 10 16 L2 16 Z", "FO8 12 2", "M20 4 L20 20", "M22.5 4 L22.5 20"],
            // A shove: a push line and two heads, the second further on.
            Icon::Heave => &["M2 12 L14 12", "M10 7 L15 12 L10 17", "M16 7 L21 12 L16 17"],
            // Slime driven in from both sides onto a target.
            Icon::Corral => &["O12 12 3.5", "FO12 12 1.2", "M1.5 12 L6.5 12", "M4.5 9.5 L7 12 L4.5 14.5", "M22.5 12 L17.5 12", "M19.5 9.5 L17 12 L19.5 14.5", "M12 1.5 L12 6.5", "M12 22.5 L12 17.5"],
            // A hook drawing slime back toward the gun.
            Icon::Pull => &["M21 12 L5 12", "M9 7.5 L4.5 12 L9 16.5", "M21 7 C17 7 15.5 9 15.5 12 C15.5 15 17 17 21 17"],
            // Two bullets fanning out of one muzzle.
            Icon::Seek => &["M2 18 L9 18", "M9 18 C13 18 15 15 16 11", "M13.5 12 L16 11 L17.2 13.6", "M15 5 C17 3 21 4 21 7 C21 10 18 10 17 9 C15 8 14 7 15 5 Z"],
            // An eye struck through: it senses nothing.
            Icon::Blind => &["M2 12 C5 7.5 8.5 5.5 12 5.5 C15.5 5.5 19 7.5 22 12 C19 16.5 15.5 18.5 12 18.5 C8.5 18.5 5 16.5 2 12 Z", "O12 12 2.8", "M4 20.5 L20 3.5"],
            // A U-turn: it turns from its own trail.
            Icon::Sour => &["M17 21 L17 10 C17 6.5 14.8 4 12 4 C9.2 4 7 6.5 7 10 L7 15", "M3.8 11.8 L7 15 L10.2 11.8"],
            // A circling arrow round a knot: it turns in tight rings.
            Icon::Spin => &["M12 4 C16.4 4 20 7.6 20 12 C20 16.4 16.4 20 12 20 C7.6 20 4 16.4 4 12 C4 9.6 5 7.5 6.7 6", "M6.6 2.6 L6.7 6 L10 6.7", "FO12 12 2.2"],
            // A sleeper's crescent moon split by a crack.
            Icon::Shatter => &["M15 3 C9.8 3.6 6 7.9 6 13 C6 18 10 22 15 22 C17.6 22 20 20.8 21.4 18.6 C16 18.8 11.8 14.8 11.8 9.6 C11.8 7.2 13 4.7 15 3 Z", "M4 3.5 L9 9 L6.5 12 L11.5 17 L9.5 21"],
            // The germ's two spots inside a blast.
            Icon::Detonate => &["M12 2 L14 8.5 L20.5 6 L16.5 11.5 L22 15 L15 15.5 L14.5 22 L11 16.5 L5.5 20.5 L7.5 14 L2 11.5 L8.5 9.5 L6.5 3.5 Z", "FO10.7 11.2 1.3", "FO13.6 13.8 1.1"],
            Icon::Flare => &["M12 3 C14.5 6 16 8.2 16 10.6 C16 13 14.2 14.6 12 14.6 C9.8 14.6 8 13 8 10.6 C8 8.6 9.2 7 10.4 5.8 C10.6 7.4 11.2 8.4 12.2 8.8 C12.4 6.8 12.2 4.8 12 3 Z", "M12 14.6 L12 21", "M4 7 L6 8", "M20 7 L18 8", "M3.5 13 L5.7 12.6", "M20.5 13 L18.3 12.6"],
            Icon::Torch => &["M12 2.5 C15 6 16.5 8.6 16.5 11 C16.5 13.6 14.5 15.5 12 15.5 C9.5 15.5 7.5 13.6 7.5 11 C7.5 9 8.8 7.3 10 6.1 C10.3 7.7 11 8.7 12 9.1 C12.3 6.9 12.2 4.7 12 2.5 Z", "M3 21 L21 21", "M4.5 21 C4.8 18.4 5.8 16.4 7.4 15", "M8 21 C8 19.2 8.6 17.9 9.6 17", "M19.5 21 C19.2 18.4 18.2 16.4 16.6 15", "M16 21 C16 19.2 15.4 17.9 14.4 17"],
            Icon::Tinder => &["M2 20 L19 20", "M3.5 20 L5.5 14", "M6.5 20 L7 15", "M9 20 L11 13.5", "M12 20 L12.5 15.5", "M14.5 20 L16.5 14", "M3 20 L1.8 16.5", "M8 20 L7.2 16.8", "M13.5 20 L13 17", "FO20.5 8 2", "M18.5 10 L16.5 12"],
            Icon::Multicast => &["M2 12 L6 12", "M6 12 L13 7.5", "M6 12 L13 16.5", "M13 5 L19 5 C21 5 22 6 22 7.5 C22 9 21 10 19 10 L13 10 Z", "M13 14 L19 14 C21 14 22 15 22 16.5 C22 18 21 19 19 19 L13 19 Z"],
            Icon::Scattergun => &["M2 14 L4 10.5 L13 10.5 L14.5 9.5 L14.5 15 L13 14 L11 14", "M4 10.5 L4 14 L8 14 L9 17.5 L11 17.5 L11 14", "FO18.5 7.5 1.3", "FO21 12.2 1.3", "FO18.5 17 1.3"],
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
        for i in [Move, Stop, Hold, Attack, Burn, Wall, Turret, Bait, Recruit, Upgrade, Rifle, Flamer, Grenade, Idle, Menu, Pause, Fast, Crystal, Soldier, Mould, Alert, Heart, Clock, Shield, Eye, Close, Check, Target, Spore, Group, Blast, Gas, Germ, Bomb, Wide, Scatter, Quiet, Delay, Thick, Pierce, Split, Carbine, LongGun, Scattergun, Bullet, Slug, Needle, Buckshot, Carry, CarryStop, Heave, Corral, Pull, Multicast, Seek, Flare, Torch, Tinder] {
            let c = rasterize(i, 24);
            let on = c.iter().filter(|&&a| a > 128).count();
            assert!(on > 15 && on < 24 * 24 / 2 + 150, "{i:?}: {on} pixels on");
        }
    }
}
