//! **The slime viewer's tweaks** (2026-11-20, the thread "Build a slime
//! viewer to tune looks"). The owner, 23:17Z: "I want to review some of the
//! changes in the slime looks. Can we have a thread build a slime viewer so
//! I can play around with the features/looks that we have build and modify
//! them myself?"
//!
//! Two pieces, both without a window so they are tested here:
//!
//! - **The look's numbers** are the view shader's own top-level constants
//!   (`view.wgsl`: `const SHEET_VEIL: f32 = 0.4;`, colours as
//!   `vec3<f32>(...)`). [`shader_consts`] reads every one whose value is a
//!   plain number, with the comment above it as its help line, and
//!   [`apply`] writes new values into a copy of the source, which the view
//!   compiles again (`View::reshade`). So every look number the Look lane
//!   adds is tunable the day it lands, and the shader keeps its constants
//!   (a uniform for each would cost the view's every pixel a read).
//! - **The tweaks file** ([`Tweaks`], `slime-tweaks.txt` at the repo's
//!   root, gitignored so `play.cmd` still pulls): what the viewer saved,
//!   one line a value. The game's window reads it when it opens and on
//!   leaving the viewer (the look, each slime's rules, engine keys and body
//!   look), so what the
//!   owner tuned is what he plays, and a thread can read the file he sends
//!   (`play-logs/`, which `play.cmd` pushes) to make it the default.

use std::path::PathBuf;

use crate::mould::tiers::Tier;

/// A custom slime's name: 1 to 24 lower-case letters, digits and `-`,
/// starting with a letter (one word in the file's lines).
pub fn ok_name(n: &str) -> bool {
    (1..=24).contains(&n.len()) && n.starts_with(|c: char| c.is_ascii_lowercase()) && n.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// The tweaks file's name, at the repo's root (`build::root`).
pub const FILE: &str = "slime-tweaks.txt";

/// Where the tweaks file is: `LARDER_TWEAKS=PATH`, else [`FILE`] at the
/// repo's root.
pub fn path() -> PathBuf {
    match std::env::var_os("LARDER_TWEAKS").filter(|v| !v.is_empty()) {
        Some(p) => PathBuf::from(p),
        None => crate::build::root().join(FILE),
    }
}

/// One of the view shader's constants with a plain-number value.
#[derive(Clone, Debug, PartialEq)]
pub struct ShaderConst {
    pub name: String,
    /// Its value in the shader as built (1 to 4 numbers).
    pub value: Vec<f32>,
    /// A colour (three numbers, none negative, one over 1.5: the shader's
    /// colours are 0-255).
    pub colour: bool,
    /// The comment above it, joined into one line ("" with none).
    pub about: String,
}

/// Every top-level `const NAME: f32 = N;` and `const NAME: vecK<f32> =
/// vecK<f32>(A, B, ..);` (with or without the type) whose value is plain
/// numbers, in the order of the source. Integers, flags, arrays and
/// worked values (`5000.0 / (512.0 * 320.0)`) are left out.
pub fn shader_consts(src: &str) -> Vec<ShaderConst> {
    let mut out = Vec::new();
    let mut about: Vec<&str> = Vec::new();
    for line in src.lines() {
        if let Some(c) = line.strip_prefix("//") {
            about.push(c.trim());
            continue;
        }
        if let Some((name, value)) = parse_const(line) {
            let colour = value.len() == 3 && value.iter().all(|&v| v >= 0.0) && value.iter().any(|&v| v > 1.5);
            out.push(ShaderConst { name, value, colour, about: about.join(" ") });
        }
        about.clear();
    }
    out
}

/// `const NAME[: TYPE] = VALUE;` with a plain-number VALUE: its name and
/// numbers.
fn parse_const(line: &str) -> Option<(String, Vec<f32>)> {
    let rest = line.strip_prefix("const ")?;
    let (lhs, rhs) = rest.split_once('=')?;
    let name = lhs.split(':').next()?.trim();
    if name.is_empty() || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return None;
    }
    let ty = lhs.split_once(':').map(|(_, t)| t.trim());
    let rhs = rhs.trim().strip_suffix(';')?.trim();
    let nums = |s: &str| -> Option<Vec<f32>> { s.split(',').map(|x| x.trim().parse::<f32>().ok().filter(|v| v.is_finite())).collect() };
    for n in 2..=4 {
        let head = format!("vec{n}<f32>(");
        if let Some(inner) = rhs.strip_prefix(head.as_str()).and_then(|r| r.strip_suffix(')')) {
            if ty.is_some_and(|t| t != format!("vec{n}<f32>")) {
                return None;
            }
            let v = nums(inner)?;
            return (v.len() == n).then(|| (name.to_string(), v));
        }
    }
    // A scalar: an f32 literal (`0.4`, `1.0e9`), not a u32 (`3u`) or bool.
    if ty.is_some_and(|t| t != "f32") || !rhs.contains(['.', 'e']) {
        return None;
    }
    let v = rhs.parse::<f32>().ok().filter(|v| v.is_finite())?;
    Some((name.to_string(), vec![v]))
}

/// A WGSL float literal for `v` (always with a point or an exponent).
fn literal(v: f32) -> String {
    let v = if v.is_finite() { v } else { 0.0 };
    let s = format!("{v:?}");
    if s.contains(['.', 'e', 'E']) { s } else { format!("{s}.0") }
}

/// `src` with each named constant's value replaced (names it lacks, or
/// with the wrong count of numbers, are skipped). Lines keep their place,
/// so the shader's error lines still match the file.
pub fn apply(src: &str, values: &[(String, Vec<f32>)]) -> String {
    if values.is_empty() {
        return src.to_string();
    }
    let mut out = String::with_capacity(src.len() + 64);
    for line in src.split_inclusive('\n') {
        let body = line.trim_end_matches(['\n', '\r']);
        let new = parse_const(body).and_then(|(name, old)| {
            let (_, v) = values.iter().find(|(n, v)| *n == name && v.len() == old.len())?;
            let value = match v.len() {
                1 => format!("const {name}: f32 = {};", literal(v[0])),
                n => format!("const {name}: vec{n}<f32> = vec{n}<f32>({});", v.iter().map(|&x| literal(x)).collect::<Vec<_>>().join(", ")),
            };
            Some(value)
        });
        match new {
            Some(n) => {
                out.push_str(&n);
                out.push_str(&line[body.len()..]);
            }
            None => out.push_str(line),
        }
    }
    out
}

/// What the viewer saved: look numbers, the game's rules, each slime's
/// engine keys, and each slime's body look, in the order set.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Tweaks {
    /// `look NAME = A [B C D]`: a view shader constant.
    pub look: Vec<(String, Vec<f32>)>,
    /// `rule TIER NAME = V`: a game rule (`Rules::set`) on that slime's
    /// planets only. (At first `rule NAME = V`, on every round in the
    /// window: the review of #607 found a rule tuned on one planet then
    /// overrode the values other planets set by depth, `star_worth` 50 on
    /// planet 2 making planet 7's ore 60, not 100.)
    pub rules: Vec<(String, String, f32)>,
    /// `slime TIER KEY = V`: an engine key on that slime only (`gpu::set`).
    pub slime: Vec<(String, String, f32)>,
    /// `body TIER = LOOK`: that slime's body look (`SlimeLook::parse`).
    pub body: Vec<(String, String)>,
    /// `custom NAME = BASE`: a slime of the owner's own (11-21, his card
    /// "Make new slimes"), the ladder's slime `BASE` with whatever
    /// `slime`, `rule` and `body` lines name it. Its NAME stands where a
    /// tier's does in those lines.
    pub custom: Vec<(String, String)>,
    /// `planet N = SLIME`: the slime a run's planet N lands on, a custom
    /// or another rung's, in place of the ladder's.
    pub planets: Vec<(u32, String)>,
}

impl Tweaks {
    /// Read from `text`; lines it does not know are skipped and named.
    pub fn parse(text: &str) -> (Tweaks, Vec<String>) {
        let mut t = Tweaks::default();
        let mut bad = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let ok = (|| -> Option<()> {
                let (lhs, rhs) = line.split_once('=')?;
                let words: Vec<&str> = lhs.split_whitespace().collect();
                let rhs = rhs.trim();
                match words.as_slice() {
                    ["look", name] => {
                        let v: Vec<f32> = rhs.split_whitespace().map(|x| x.parse::<f32>().ok().filter(|v| v.is_finite())).collect::<Option<_>>()?;
                        (1..=4).contains(&v.len()).then(|| t.set_look(name, &v))
                    }
                    ["rule", tier, name] => {
                        t.set_rule(tier, name, rhs.parse().ok().filter(|v: &f32| v.is_finite())?);
                        Some(())
                    }
                    ["slime", tier, key] => {
                        t.set_slime(tier, key, rhs.parse().ok().filter(|v: &f32| v.is_finite())?);
                        Some(())
                    }
                    ["body", tier] => {
                        t.set_body(tier, rhs);
                        Some(())
                    }
                    ["custom", name] => {
                        let base = Tier::from_name(rhs)?;
                        (ok_name(name) && Tier::from_name(name).is_none()).then_some(())?;
                        t.custom.retain(|(n, _)| n != name);
                        t.custom.push((name.to_string(), base.name().to_string()));
                        Some(())
                    }
                    ["planet", n] => {
                        let n: u32 = n.parse().ok().filter(|n| (1..=crate::mould::run::PLANETS).contains(n))?;
                        t.set_planet(n, Some(rhs));
                        Some(())
                    }
                    _ => None,
                }
            })();
            if ok.is_none() {
                bad.push(line.to_string());
            }
        }
        (t, bad)
    }

    /// The file's text.
    pub fn text(&self) -> String {
        let mut s = String::from(
            "# Your slime viewer tweaks (viewer.cmd). The game reads this file when it\n\
             # opens, so what you tuned is what you play. Delete it, or press Reset all\n\
             # and then Save in the viewer, to play the game's own looks and rules again.\n\
             # look NAME = value(s): a number of the slime's picture (view.wgsl)\n\
             # rule SLIME NAME = value: a game rule on that slime's planets only\n\
             # slime SLIME KEY = value: an engine key on that slime only\n\
             # body SLIME = LOOK: that slime's body look (plain, threads, sheet, cords,\n\
             # swell, crust, scars)\n\
             # custom NAME = SLIME: a slime of your own, starting from that one\n\
             # planet N = SLIME: the slime a run's planet N lands on\n",
        );
        for (n, base) in &self.custom {
            s += &format!("custom {n} = {base}\n");
        }
        for (n, who) in &self.planets {
            s += &format!("planet {n} = {who}\n");
        }
        // Rust's shortest form that reads back to the same f32. (At first
        // rounded to 4 places, which saved the Sporer's `spore_rate`,
        // 0.00001, as 0 or 0.0001: no spores, or ten times as many.)
        let num = |v: f32| format!("{v}");
        for (n, v) in &self.look {
            s += &format!("look {n} = {}\n", v.iter().map(|&x| num(x)).collect::<Vec<_>>().join(" "));
        }
        for (tier, n, v) in &self.rules {
            s += &format!("rule {tier} {n} = {}\n", num(*v));
        }
        for (tier, k, v) in &self.slime {
            s += &format!("slime {tier} {k} = {}\n", num(*v));
        }
        for (tier, l) in &self.body {
            s += &format!("body {tier} = {l}\n");
        }
        s
    }

    /// Nothing set.
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    /// How many values are set.
    pub fn count(&self) -> usize {
        self.look.len() + self.rules.len() + self.slime.len() + self.body.len() + self.custom.len() + self.planets.len()
    }

    /// The ladder's slime a slime starts from: a custom's base, else the
    /// name itself if it is a rung's.
    pub fn base_of(&self, slime: &str) -> Option<Tier> {
        match self.custom.iter().find(|(n, _)| n == slime) {
            Some((_, b)) => Tier::from_name(b),
            None => Tier::from_name(slime),
        }
    }

    /// The slime run planet `depth` lands on, if set.
    pub fn planet(&self, depth: u32) -> Option<&str> {
        self.planets.iter().find(|(n, _)| *n == depth).map(|(_, s)| s.as_str())
    }

    /// Sets (or with None clears) the slime planet `depth` lands on.
    pub fn set_planet(&mut self, depth: u32, slime: Option<&str>) {
        self.planets.retain(|(n, _)| *n != depth);
        if let Some(s) = slime {
            self.planets.push((depth, s.to_string()));
            self.planets.sort_by_key(|p| p.0);
        }
    }

    /// A new slime of the owner's own from `from` (a rung's or a custom),
    /// with everything `from` has set, named `from-2`, `from-3`, ...; its
    /// name.
    pub fn new_custom(&mut self, from: &str) -> Option<String> {
        let base = self.base_of(from)?;
        let stem = from.trim_end_matches(|c: char| c.is_ascii_digit() || c == '-');
        let stem = if stem.is_empty() { base.name() } else { stem };
        let name = (2..).map(|k| format!("{stem}-{k}")).find(|n| !self.custom.iter().any(|(c, _)| c == n))?;
        self.custom.push((name.clone(), base.name().to_string()));
        self.copy_slime(from, &name);
        Some(name)
    }

    /// Everything `from` sets (`slime`, `rule`, `body`), set for `to` too.
    fn copy_slime(&mut self, from: &str, to: &str) {
        let slime: Vec<_> = self.slime.iter().filter(|(t, _, _)| t == from).map(|(_, k, v)| (k.clone(), *v)).collect();
        for (k, v) in slime {
            self.set_slime(to, &k, v);
        }
        let rules: Vec<_> = self.rules.iter().filter(|(t, _, _)| t == from).map(|(_, k, v)| (k.clone(), *v)).collect();
        for (k, v) in rules {
            self.set_rule(to, &k, v);
        }
        if let Some(l) = self.body_of(from).map(str::to_string) {
            self.set_body(to, &l);
        }
    }

    /// A custom renamed, everywhere it is named; false if `to` is taken or
    /// not a name (lower-case letters, digits and `-`).
    pub fn rename(&mut self, from: &str, to: &str) -> bool {
        if from == to {
            return true;
        }
        if !ok_name(to) || Tier::from_name(to).is_some() || self.custom.iter().any(|(n, _)| n == to) || !self.custom.iter().any(|(n, _)| n == from) {
            return false;
        }
        for (n, _) in self.custom.iter_mut().filter(|(n, _)| n == from) {
            *n = to.to_string();
        }
        for (t, _, _) in self.slime.iter_mut().chain(self.rules.iter_mut()).filter(|(t, _, _)| t == from) {
            *t = to.to_string();
        }
        for (t, _) in self.body.iter_mut().filter(|(t, _)| t == from) {
            *t = to.to_string();
        }
        for (_, s) in self.planets.iter_mut().filter(|(_, s)| s == from) {
            *s = to.to_string();
        }
        true
    }

    /// A custom deleted, with everything it set and the planets it held.
    pub fn delete_custom(&mut self, name: &str) {
        if !self.custom.iter().any(|(n, _)| n == name) {
            return;
        }
        self.custom.retain(|(n, _)| n != name);
        self.slime.retain(|(t, _, _)| t != name);
        self.rules.retain(|(t, _, _)| t != name);
        self.body.retain(|(t, _)| t != name);
        self.planets.retain(|(_, s)| s != name);
    }

    /// Everything but the owner's own slimes and the planets' picks
    /// cleared (the viewer's Reset all: a named slime is not lost to it).
    pub fn reset_but_own(&mut self) {
        let own = |t: &String, c: &[(String, String)]| c.iter().any(|(n, _)| n == t);
        let custom = self.custom.clone();
        self.look.clear();
        self.slime.retain(|(t, _, _)| own(t, &custom));
        self.rules.retain(|(t, _, _)| own(t, &custom));
        self.body.retain(|(t, _)| own(t, &custom));
    }

    pub fn set_look(&mut self, name: &str, v: &[f32]) {
        match self.look.iter_mut().find(|(n, _)| n == name) {
            Some(e) => e.1 = v.to_vec(),
            None => self.look.push((name.to_string(), v.to_vec())),
        }
    }

    pub fn set_rule(&mut self, tier: &str, name: &str, v: f32) {
        match self.rules.iter_mut().find(|(t, n, _)| t == tier && n == name) {
            Some(e) => e.2 = v,
            None => self.rules.push((tier.to_string(), name.to_string(), v)),
        }
    }

    pub fn set_slime(&mut self, tier: &str, key: &str, v: f32) {
        match self.slime.iter_mut().find(|(t, k, _)| t == tier && k == key) {
            Some(e) => e.2 = v,
            None => self.slime.push((tier.to_string(), key.to_string(), v)),
        }
    }

    pub fn set_body(&mut self, tier: &str, look: &str) {
        match self.body.iter_mut().find(|(t, _)| t == tier) {
            Some(e) => e.1 = look.to_string(),
            None => self.body.push((tier.to_string(), look.to_string())),
        }
    }

    /// The look value set for `name`, if any.
    pub fn look_of(&self, name: &str) -> Option<&[f32]> {
        self.look.iter().find(|(n, _)| n == name).map(|(_, v)| v.as_slice())
    }

    pub fn rule_of(&self, tier: &str, name: &str) -> Option<f32> {
        self.rules.iter().find(|(t, n, _)| t == tier && n == name).map(|e| e.2)
    }

    pub fn slime_of(&self, tier: &str, key: &str) -> Option<f32> {
        self.slime.iter().find(|(t, k, _)| t == tier && k == key).map(|e| e.2)
    }

    pub fn body_of(&self, tier: &str) -> Option<&str> {
        self.body.iter().find(|(t, _)| t == tier).map(|(_, l)| l.as_str())
    }

    /// Read from the tweaks file ([`path`]); empty with none. Lines it
    /// cannot read are printed and skipped.
    pub fn load() -> Tweaks {
        let p = path();
        let Ok(text) = std::fs::read_to_string(&p) else { return Tweaks::default() };
        let (t, bad) = Tweaks::parse(&text);
        for b in bad {
            eprintln!("larder-world: {}: skipped a line it cannot read: {b}", p.display());
        }
        t
    }

    /// Written to the tweaks file, or the file deleted when empty.
    pub fn save(&self) -> std::io::Result<PathBuf> {
        let p = path();
        if self.is_empty() {
            match std::fs::remove_file(&p) {
                Err(e) if e.kind() != std::io::ErrorKind::NotFound => return Err(e),
                _ => {}
            }
        } else {
            std::fs::write(&p, self.text())?;
        }
        Ok(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC: &str = "// The sheet's veil.\n// Second line.\nconst SHEET_VEIL: f32 = 0.4;\nconst AGAR: u32 = 0u;\nconst HUE: vec3<f32> = vec3<f32>(240.0, 165.9, 63.2);\nconst SCENT_RGB = vec3<f32>(70.0, 170.0, 255.0);\n\nconst DENS: f32 = 5000.0 / (512.0 * 320.0);\nconst ON: bool = false;\nconst LIGHT: vec3<f32> = vec3<f32>(-0.42, -0.58, 0.70);\nconst BIG: f32 = 1.0e9;\nfn f() -> f32 {\n    const LOCAL: f32 = 2.0;\n    return LOCAL;\n}\n";

    #[test]
    fn the_shaders_plain_numbers_are_read_with_their_comments() {
        let c = shader_consts(SRC);
        let names: Vec<&str> = c.iter().map(|c| c.name.as_str()).collect();
        assert_eq!(names, ["SHEET_VEIL", "HUE", "SCENT_RGB", "LIGHT", "BIG"]);
        assert_eq!(c[0].about, "The sheet's veil. Second line.");
        assert_eq!(c[0].value, [0.4]);
        assert!(c[1].colour && c[2].colour && !c[3].colour && !c[0].colour);
        assert_eq!(c[1].about, "", "a comment ends at the line after it");
    }

    #[test]
    fn applied_values_replace_only_their_lines() {
        let out = apply(SRC, &[("SHEET_VEIL".into(), vec![0.75]), ("HUE".into(), vec![1.0, 2.0, 3.5]), ("SCENT_RGB".into(), vec![1.0]), ("NONE".into(), vec![1.0])]);
        assert_eq!(out.lines().count(), SRC.lines().count());
        assert!(out.contains("const SHEET_VEIL: f32 = 0.75;\n"));
        assert!(out.contains("const HUE: vec3<f32> = vec3<f32>(1.0, 2.0, 3.5);\n"));
        assert!(out.contains("const SCENT_RGB = vec3<f32>(70.0, 170.0, 255.0);"), "the wrong count of numbers is skipped");
        let back = shader_consts(&out);
        assert_eq!(back[0].value, [0.75]);
        assert_eq!(back[1].value, [1.0, 2.0, 3.5]);
        assert_eq!(apply(SRC, &[]), SRC);
    }

    #[test]
    fn the_real_shader_reads_and_writes_back_the_same() {
        let src = crate::world::view::SHADER;
        let c = shader_consts(src);
        assert!(c.len() > 80, "the view's look numbers: {}", c.len());
        for name in ["MOULD_HUE", "SHEET_VEIL", "THREAD_VEIL", "CORD_VEIL", "SWELL_GAIN", "LUMP_GLOSS", "SET_AMBER", "DROP_GOLD"] {
            assert!(c.iter().any(|k| k.name == name), "{name} is tunable");
        }
        // Every value written back as it was parses to the same numbers.
        let same: Vec<(String, Vec<f32>)> = c.iter().map(|k| (k.name.clone(), k.value.clone())).collect();
        assert_eq!(shader_consts(&apply(src, &same)), c);
    }

    #[test]
    fn own_slimes_copy_rename_delete_and_survive_reset_all() {
        let mut t = Tweaks::default();
        t.set_slime("drifter", "keep", 0.9);
        t.set_rule("drifter", "fruit_hurt", 0.05);
        t.set_body("drifter", "threads");
        t.set_look("SHEET_VEIL", &[0.5]);
        let a = t.new_custom("drifter").unwrap();
        assert_eq!(a, "drifter-2");
        assert_eq!(t.slime_of(&a, "keep"), Some(0.9), "it starts as its base is set");
        assert_eq!(t.body_of(&a), Some("threads"));
        assert_eq!(t.new_custom(&a).as_deref(), Some("drifter-3"));
        assert_eq!(t.base_of("drifter-3"), Some(Tier::Drifter));
        t.set_planet(4, Some(&a));
        assert!(t.rename(&a, "fog"));
        assert!(!t.rename("fog", "trunk"), "a rung's name is taken");
        assert!(!t.rename("fog", "Has Space"));
        assert_eq!(t.planet(4), Some("fog"));
        assert_eq!(t.rule_of("fog", "fruit_hurt"), Some(0.05));
        let (back, bad) = Tweaks::parse(&t.text());
        assert!(bad.is_empty(), "{bad:?}");
        assert_eq!(back, t);
        t.reset_but_own();
        assert_eq!(t.slime_of("drifter", "keep"), None);
        assert!(t.look.is_empty());
        assert_eq!(t.slime_of("fog", "keep"), Some(0.9), "Reset all keeps your own slimes");
        assert_eq!(t.planet(4), Some("fog"));
        t.delete_custom("fog");
        assert_eq!(t.planet(4), None);
        assert_eq!(t.slime_of("fog", "keep"), None);
        assert_eq!(Tweaks::parse("custom trunk = drifter\ncustom x = nothing\nplanet 13 = x\n").1.len(), 3);
    }

    #[test]
    fn the_file_round_trips_and_skips_what_it_cannot_read() {
        let mut t = Tweaks::default();
        t.set_look("HUE", &[1.0, 2.5, 3.0]);
        t.set_look("SHEET_VEIL", &[0.4]);
        t.set_look("SHEET_VEIL", &[0.5]);
        t.set_rule("drifter", "fruit_hurt", 0.05);
        t.set_slime("sporer", "spore_rate", 0.00001);
        t.set_slime("drifter", "keep", 0.9);
        t.set_body("sprawler", "sheet");
        assert_eq!(t.count(), 6);
        let (back, bad) = Tweaks::parse(&(t.text() + "nonsense line\nlook X = a\n"));
        assert_eq!(back, t);
        assert_eq!(bad, ["nonsense line", "look X = a"]);
        assert_eq!(back.look_of("SHEET_VEIL"), Some(&[0.5][..]));
        assert_eq!(back.slime_of("drifter", "keep"), Some(0.9));
        assert_eq!(back.slime_of("sporer", "spore_rate"), Some(0.00001), "small numbers keep their figures");
        assert_eq!(back.rule_of("drifter", "fruit_hurt"), Some(0.05));
        assert_eq!(back.rule_of("trunk", "fruit_hurt"), None, "a rule is the slime's own");
        assert_eq!(back.body_of("sprawler"), Some("sheet"));
    }
}
