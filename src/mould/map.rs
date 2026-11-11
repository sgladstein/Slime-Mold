//! **Maps** (ROADMAP WM1): a designed Larder world, read from two files
//! (agreed with the maps thread, 2026-11-11 19:40Z):
//!
//! - `<name>.png`, 1024 x 640, RGBA: G is the engine ground word's bits
//!   8-15 (`G_WET` 0x01, `G_ROCK` 0x08, the soil kind in the high nibble).
//!   R is height, parked (owner, 20:06Z), so the loader ignores it; B is 0.
//! - `<name>.txt`, one item a line, in the same 1024 x 640 cells:
//!   `crystal x y r charge` (centre, radius, charge a cell), `mould x y`
//!   (the mould's start), `landing x y` (the squad's), and
//!   `wall x0 y0 x1 y1 thick hp` (thick is the full width; soft rock is
//!   written as walls, so the engine builds it as it builds player walls).
//!   Blank lines and lines from `#` on are skipped.
//!
//! The map's cells are the fine cells of the 256 x 160 dish at scale 4, so
//! a world at scale `s` takes every `4 / s`-th cell (`Map::ground_fine`)
//! and divides positions by 4 for dish cells (`Map::dish`).

use std::path::Path;

use super::big::BigDish;

/// Map cells: the 1024 x 640 world (the dish's 256 x 160 at scale 4).
pub const MAP_W: usize = 1024;
pub const MAP_H: usize = 640;
/// Map cells a dish cell.
pub const MAP_SCALE: f32 = 4.0;

/// The G byte's bits (the ground word's bits 8-15).
pub const G_WET: u8 = 0x01;
pub const G_ROCK: u8 = 0x08;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Crystal {
    /// Centre and radius, map cells; charge a cell.
    pub x: f32,
    pub y: f32,
    pub r: f32,
    pub charge: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Wall {
    /// Ends and full width, map cells; health a cell.
    pub a: (f32, f32),
    pub b: (f32, f32),
    pub thick: f32,
    pub hp: f32,
}

/// A map's items (the `.txt`).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Items {
    pub crystals: Vec<Crystal>,
    pub mould: Option<(f32, f32)>,
    pub landing: Option<(f32, f32)>,
    pub walls: Vec<Wall>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Map {
    /// The G byte a map cell, row-major, `MAP_W` x `MAP_H`.
    pub ground: Vec<u8>,
    pub items: Items,
}

impl Items {
    pub fn parse(text: &str) -> Result<Items, String> {
        let mut it = Items::default();
        for (n, line) in text.lines().enumerate() {
            let line = line.split('#').next().unwrap_or("").trim();
            if line.is_empty() {
                continue;
            }
            let mut words = line.split_whitespace();
            let kind = words.next().unwrap_or("");
            let v: Vec<f32> = words.map(|w| w.parse::<f32>().map_err(|e| format!("line {}: {w:?}: {e}", n + 1))).collect::<Result<_, _>>()?;
            let want = |k: usize| if v.len() == k { Ok(()) } else { Err(format!("line {}: `{kind}` takes {k} numbers, got {}", n + 1, v.len())) };
            match kind {
                "crystal" => {
                    want(4)?;
                    it.crystals.push(Crystal { x: v[0], y: v[1], r: v[2], charge: v[3] });
                }
                "mould" => {
                    want(2)?;
                    it.mould = Some((v[0], v[1]));
                }
                "landing" => {
                    want(2)?;
                    it.landing = Some((v[0], v[1]));
                }
                "wall" => {
                    want(6)?;
                    it.walls.push(Wall { a: (v[0], v[1]), b: (v[2], v[3]), thick: v[4], hp: v[5] });
                }
                _ => return Err(format!("line {}: unknown item `{kind}`", n + 1)),
            }
        }
        Ok(it)
    }

    /// The items as the `.txt` holds them (`parse` reads it back).
    pub fn text(&self) -> String {
        let mut out = String::new();
        for c in &self.crystals {
            out += &format!("crystal {} {} {} {}\n", c.x, c.y, c.r, c.charge);
        }
        if let Some((x, y)) = self.mould {
            out += &format!("mould {x} {y}\n");
        }
        if let Some((x, y)) = self.landing {
            out += &format!("landing {x} {y}\n");
        }
        for w in &self.walls {
            out += &format!("wall {} {} {} {} {} {}\n", w.a.0, w.a.1, w.b.0, w.b.1, w.thick, w.hp);
        }
        out
    }
}

impl Map {
    /// `<dir>/<name>.png` and `<dir>/<name>.txt`.
    pub fn load(dir: &Path, name: &str) -> Result<Map, String> {
        let png = dir.join(format!("{name}.png"));
        let txt = dir.join(format!("{name}.txt"));
        let img = image::open(&png).map_err(|e| format!("{}: {e}", png.display()))?.to_rgba8();
        if (img.width() as usize, img.height() as usize) != (MAP_W, MAP_H) {
            return Err(format!("{}: {} x {}, want {MAP_W} x {MAP_H}", png.display(), img.width(), img.height()));
        }
        let ground = img.pixels().map(|p| p.0[1]).collect();
        let text = std::fs::read_to_string(&txt).map_err(|e| format!("{}: {e}", txt.display()))?;
        let items = Items::parse(&text).map_err(|e| format!("{}: {e}", txt.display()))?;
        Ok(Map { ground, items })
    }

    /// Writes `<dir>/<name>.png` (R and B 0, A 255) and `<dir>/<name>.txt`.
    pub fn save(&self, dir: &Path, name: &str) -> Result<(), String> {
        let img = image::RgbaImage::from_fn(MAP_W as u32, MAP_H as u32, |x, y| image::Rgba([0, self.ground[y as usize * MAP_W + x as usize], 0, 255]));
        let png = dir.join(format!("{name}.png"));
        img.save(&png).map_err(|e| format!("{}: {e}", png.display()))?;
        let txt = dir.join(format!("{name}.txt"));
        std::fs::write(&txt, self.items.text()).map_err(|e| format!("{}: {e}", txt.display()))
    }

    /// The G byte under fine cell (`fx`, `fy`) of a world at `scale`: the
    /// map cell at the fine cell's centre.
    pub fn ground_fine(&self, scale: usize, fx: usize, fy: usize) -> u8 {
        let k = MAP_SCALE / scale as f32;
        let x = (((fx as f32 + 0.5) * k) as usize).min(MAP_W - 1);
        let y = (((fy as f32 + 0.5) * k) as usize).min(MAP_H - 1);
        self.ground[y * MAP_W + x]
    }

    /// Map cells to dish cells.
    pub fn dish(p: (f32, f32)) -> (f32, f32) {
        (p.0 / MAP_SCALE, p.1 / MAP_SCALE)
    }

    /// A made world as a map, so a noise world can be kept, edited and
    /// loaded (`BigDish::with_map`): each map cell takes the rock and wet
    /// of the fine cell it falls in, the piles become crystals in their
    /// order, and the mould's start is the dish's `home`. Taken before any
    /// wall is built (walls are items, and built cells are not segments).
    pub fn of_dish(d: &BigDish) -> Map {
        assert!(d.wall.is_empty(), "a dish with no walls built");
        let s = d.p.scale;
        let on = |v: &[bool], i: usize| !v.is_empty() && v[i];
        let ground = (0..MAP_W * MAP_H)
            .map(|c| {
                let i = (c / MAP_W * s / 4) * d.w + c % MAP_W * s / 4;
                if on(&d.rock, i) {
                    G_ROCK
                } else if on(&d.wet, i) {
                    G_WET
                } else {
                    0
                }
            })
            .collect();
        let k = MAP_SCALE;
        let crystals = (0..d.piles.len()).map(|p| Crystal { x: d.piles[p].0 * k, y: d.piles[p].1 * k, r: d.pile_rs[p] * k, charge: d.pile_fill[p] }).collect();
        let items = Items { crystals, mould: Some((d.home.0 * k, d.home.1 * k)), landing: d.landing.map(|(x, y)| (x * k, y * k)), walls: Vec::new() };
        Map { ground, items }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items_parse_and_write_back() {
        let text = "# a test\ncrystal 512 600 12 384\nmould 512 590\n\nlanding 512 180  # the squad\nwall 100 100 200 120 8 400\n";
        let it = Items::parse(text).unwrap();
        assert_eq!(it.crystals, vec![Crystal { x: 512.0, y: 600.0, r: 12.0, charge: 384.0 }]);
        assert_eq!((it.mould, it.landing), (Some((512.0, 590.0)), Some((512.0, 180.0))));
        assert_eq!(it.walls, vec![Wall { a: (100.0, 100.0), b: (200.0, 120.0), thick: 8.0, hp: 400.0 }]);
        assert_eq!(Items::parse(&it.text()).unwrap(), it);
        assert!(Items::parse("crystal 1 2 3").unwrap_err().contains("takes 4"));
        assert!(Items::parse("tree 1 2").unwrap_err().contains("unknown item"));
        assert!(Items::parse("mould 1 x").is_err());
    }

    #[test]
    fn a_world_at_scale_two_takes_every_other_cell() {
        let mut ground = vec![0u8; MAP_W * MAP_H];
        ground[3 * MAP_W + 5] = G_ROCK;
        let m = Map { ground, items: Items::default() };
        // Fine cell (2, 1) at scale 2 covers map cells 4-5 x 2-3; its
        // centre is map cell (5, 3).
        assert_eq!(m.ground_fine(2, 2, 1), G_ROCK);
        assert_eq!(m.ground_fine(2, 2, 0), 0);
        assert_eq!(m.ground_fine(4, 5, 3), G_ROCK);
        // At scale 1 fine cell (1, 0) samples its centre, map cell (6, 2):
        // a lone map cell can fall between samples.
        assert_eq!(m.ground_fine(1, 1, 0), 0);
    }

    use crate::mould::big::BigParams;
    use crate::mould::larder_world::{race_params, race_world};

    /// A noise world saved as a map and loaded back is the same world, and
    /// plays the same: the loader adds nothing and loses nothing.
    #[test]
    fn a_saved_world_loads_and_plays_the_same() {
        for scale in [1, 2] {
            let mut a = race_world(7, scale, 1500);
            let m = Map::of_dish(&a);
            // Through the text, as a saved map would be.
            let m = Map { items: Items::parse(&m.items.text()).unwrap(), ..m };
            let mut b = BigDish::with_map(race_params(7, scale, 1500), &m);
            assert_eq!((a.piles.clone(), a.pile_rs.clone(), a.pile_fill.clone(), a.home), (b.piles.clone(), b.pile_rs.clone(), b.pile_fill.clone(), b.home));
            assert!(a.rock == b.rock && a.wet == b.wet && a.agar == b.agar && a.scent_agar == b.scent_agar, "scale {scale}: ground");
            assert!(a.pile_of == b.pile_of && a.pile_full == b.pile_full, "scale {scale}: crystals");
            assert!(a.rock.iter().filter(|&&r| r).count() > a.w * a.h / 100, "rock to compare");
            for _ in 0..30 {
                a.tick();
                b.tick();
            }
            assert_eq!(a.hash(), b.hash(), "scale {scale}");
        }
    }

    /// A designed map in the world: each crystal its own size and charge,
    /// the one nearest the mould's start first, the blob on the start, the
    /// map's rock, and its walls with their health.
    #[test]
    fn a_designed_map_makes_its_world() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let m = Map::load(&dir, "nest").unwrap();
        let p = race_params(1, 2, 3000);
        let d = BigDish::with_map(p.clone(), &m);
        let (start, landing) = (m.items.mould.unwrap(), m.items.landing.unwrap());
        assert_eq!(d.piles.len(), m.items.crystals.len());
        assert_eq!(d.home, Map::dish(start));
        assert_eq!(d.landing, Some(Map::dish(landing)));
        let near = |c: &Crystal| (c.x - start.0).hypot(c.y - start.1);
        let first = m.items.crystals.iter().min_by(|a, b| near(a).total_cmp(&near(b))).unwrap();
        assert_eq!(d.piles[0], Map::dish((first.x, first.y)));
        assert_eq!((d.pile_rs[0], d.pile_fill[0]), (first.r / MAP_SCALE, first.charge));
        // A crystal's cells hold its own charge; bigger crystals more cells.
        for pi in 0..d.piles.len() {
            let cells: Vec<usize> = d.pile_cells.iter().map(|&i| i as usize).filter(|&i| d.pile_of[i] as usize == pi + 1).collect();
            assert!(!cells.is_empty(), "crystal {pi} has cells");
            assert!(cells.iter().all(|&i| d.charge[i] == d.pile_fill[pi]));
        }
        // The blob sits on the start.
        let s = p.scale as f32;
        let (mx, my) = d.walkers.iter().take(3000).fold((0.0, 0.0), |a, w| (a.0 + w.0 / 3000.0, a.1 + w.1 / 3000.0));
        assert!((mx / s - d.home.0).hypot(my / s - d.home.1) < 3.0, "blob at ({}, {}), home {:?}", mx / s, my / s, d.home);
        // The Nest's 34 walls stand with 400 health.
        let walls = d.wall.iter().filter(|&&w| w).count();
        assert!(walls > 500, "{walls} wall cells");
        assert!(d.wall.iter().zip(&d.wall_health).all(|(&w, &h)| !w || h == 400.0));
        assert!(d.walkers.iter().take(3000).all(|&(x, y, _)| !d.wall[y as usize * d.w + x as usize]));
        // Maps need crystals switched on.
        let off = BigDish::with_map(BigParams { crystal_charge: 0.0, ..p.clone() }, &m);
        assert!(off.charge.is_empty());
        // The maps' rock and wet are the world's (wet only on open agar).
        let mut rocks = 0;
        for name in ["fork", "nest", "passes"] {
            let m = Map::load(&dir, name).unwrap();
            let d = BigDish::with_map(p.clone(), &m);
            let count = |v: &[bool]| v.iter().filter(|&&r| r).count();
            let want = |bit: u8| (0..d.w * d.h).filter(|&i| m.ground_fine(2, i % d.w, i / d.w) & bit != 0 && (bit == G_ROCK || d.agar[i] || d.wall[i])).count();
            assert_eq!(count(&d.rock), want(G_ROCK), "{name}: rock");
            assert_eq!(count(&d.wet), want(G_WET), "{name}: wet");
            rocks += count(&d.rock);
        }
        assert!(rocks > 0, "rock to compare");
        // The Fork's start is 12 dish cells from the rim, so its blob does
        // not fit at the usual size and widens until all its walkers do.
        let fork = BigDish::with_map(race_params(1, 2, 5000), &Map::load(&dir, "fork").unwrap());
        assert_eq!(fork.alive.iter().filter(|&&a| a).count(), 5000);
    }

    /// Every map the maps thread has written loads, with crystals and
    /// both starts.
    #[test]
    fn the_designed_maps_load() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("Reports/larder-maps-2026-11-11/maps");
        let mut n = 0;
        for e in std::fs::read_dir(&dir).unwrap() {
            let path = e.unwrap().path();
            if path.extension().is_some_and(|x| x == "txt") {
                let name = path.file_stem().unwrap().to_str().unwrap().to_string();
                let m = Map::load(&dir, &name).unwrap_or_else(|e| panic!("{e}"));
                assert!(!m.items.crystals.is_empty() && m.items.mould.is_some() && m.items.landing.is_some(), "{name}");
                n += 1;
            }
        }
        assert!(n >= 5, "{n} maps");
    }
}
