//! **Maps** (ROADMAP WM1): a designed Larder world, read from two files
//! (agreed with the maps thread, 2026-11-11 19:40Z):
//!
//! - `<name>.png`, 1024 x 640 (or any size in whole dish cells, 4 map
//!   cells each: a generated planet grows with depth), RGBA: G is the engine ground word's bits
//!   8-15 (`G_WET` 0x01, `G_ROCK` 0x08, the soil kind in the high nibble).
//!   R is height, parked (owner, 20:06Z), so the loader ignores it; B is
//!   the start plants (`Map::plants`, plants and levelgen threads,
//!   2026-11-14): the plant kind in the top 2 bits (`plants::MEADOW`,
//!   `JUNGLE`, `REEDS`, `SCRUB`), the density 1-63 in the low 6, 0 no
//!   plant (`plants::pack`); B 0 everywhere is a map with no plants.
//! - `<name>.txt`, one item a line, in the same 1024 x 640 cells:
//!   `crystal x y r charge` (centre, radius, charge a cell), `mould x y`
//!   (a start of the mould's; a map may give several, and the walkers are
//!   shared among them), `landing x y` (the squad's), and
//!   `wall x0 y0 x1 y1 thick hp` (thick is the full width; soft rock is
//!   written as walls, so the engine builds it as it builds player walls).
//!   Blank lines and lines from `#` on are skipped.
//!
//! The map's cells are the fine cells of the dish at scale 4 (the 256 x 160
//! dish for a 1024 x 640 map; a map `w` x `h` makes a `w / 4` x `h / 4`
//! dish, `Map::dish_size`), so a world at scale `s` takes every `4 / s`-th
//! cell (`Map::ground_fine`) and divides positions by 4 for dish cells
//! (`Map::dish`).

use std::path::Path;

use super::big::BigDish;

/// Map cells: the 1024 x 640 world (the dish's 256 x 160 at scale 4), the
/// designed maps' size and a new map's.
pub const MAP_W: usize = 1024;
pub const MAP_H: usize = 640;
/// The biggest map the loader takes, each way: a guard against a stray
/// picture, well past the run's core world (about 2,500 x 1,570).
pub const MAP_MAX: usize = 4096;
/// Map cells a dish cell.
pub const MAP_SCALE: f32 = 4.0;

/// The G byte's bits (the ground word's bits 8-15).
pub const G_WET: u8 = 0x01;
pub const G_ROCK: u8 = 0x08;
/// Deep water's whole G byte: soil kind 4 (`big::SOIL_DEEP`) with the wet
/// bit, so a build from before deep water plays it as wet (new ground
/// kinds, 2026-11-14, agreed with the procedural levels thread).
pub const G_DEEP: u8 = 0x41;
/// A bridge's G byte: soil kind 5 (`big::SOIL_BRIDGE`), dry.
pub const G_BRIDGE: u8 = 0x50;

/// Whether a soldier can stand on a map cell's ground: not rock, not deep
/// water. Walls (soft rock) are items and count as ground here, since a
/// squad can shoot or blast through them.
pub fn standable(g: u8) -> bool {
    g & G_ROCK == 0 && g >> 4 != super::big::SOIL_DEEP
}

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
    pub moulds: Vec<(f32, f32)>,
    pub landing: Option<(f32, f32)>,
    pub walls: Vec<Wall>,
    /// `moisture BASE SPREAD` (plants and levelgen threads, 11-14): the
    /// map's ground moisture and how much it varies, for fire
    /// (`plants::moisture`); None, the engine's 0.15 and 0.3.
    pub moisture: Option<(f32, f32)>,
    /// The map's flora: how its plants are drawn up close (the map's
    /// `flora N` line, the level generator's biome, `levelgen::Biome` in
    /// order: 0 meadow, 1 barrens, 2 fen, 3 valley, 4 jungle, 5
    /// highlands, 6 caves, 7 core; view.wgsl's `plant_layer`). Looks
    /// only: the plants' kinds and amounts are the simulation's either
    /// way. None draws flora 0.
    pub flora: Option<u8>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Map {
    /// Map cells across and down: multiples of 4 (map cells a dish cell).
    pub w: usize,
    pub h: usize,
    /// The G byte a map cell, row-major, `w` x `h`.
    pub ground: Vec<u8>,
    /// The B byte a map cell, the start plants (`plants::pack`),
    /// or empty: a map with B all 0 has no plants.
    pub plants: Vec<u8>,
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
                    it.moulds.push((v[0], v[1]));
                }
                "landing" => {
                    want(2)?;
                    it.landing = Some((v[0], v[1]));
                }
                "moisture" => {
                    want(2)?;
                    it.moisture = Some((v[0], v[1]));
                }
                "flora" => {
                    want(1)?;
                    it.flora = Some(v[0].clamp(0.0, 255.0) as u8);
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
        for (x, y) in &self.moulds {
            out += &format!("mould {x} {y}\n");
        }
        if let Some((x, y)) = self.landing {
            out += &format!("landing {x} {y}\n");
        }
        for w in &self.walls {
            out += &format!("wall {} {} {} {} {} {}\n", w.a.0, w.a.1, w.b.0, w.b.1, w.thick, w.hp);
        }
        if let Some((b, s)) = self.moisture {
            out += &format!("moisture {b} {s}\n");
        }
        if let Some(f) = self.flora {
            out += &format!("flora {f}\n");
        }
        out
    }
}

impl Map {
    /// A blank map, plain ground and no items, `w` x `h` map cells.
    pub fn blank(w: usize, h: usize) -> Map {
        Map { w, h, ground: vec![0; w * h], plants: Vec::new(), items: Items::default() }
    }

    /// The dish this map makes, in dish cells (`BigParams::dish`).
    pub fn dish_size(&self) -> (usize, usize) {
        (self.w / 4, self.h / 4)
    }

    /// `<dir>/<name>.png` and `<dir>/<name>.txt`.
    pub fn load(dir: &Path, name: &str) -> Result<Map, String> {
        let png = dir.join(format!("{name}.png"));
        let txt = dir.join(format!("{name}.txt"));
        let img = image::open(&png).map_err(|e| format!("{}: {e}", png.display()))?.to_rgba8();
        let (w, h) = (img.width() as usize, img.height() as usize);
        if w % 4 != 0 || h % 4 != 0 || !(64..=MAP_MAX).contains(&w) || !(64..=MAP_MAX).contains(&h) {
            return Err(format!("{}: {w} x {h}, want multiples of 4 from 64 to {MAP_MAX} (the designed maps are {MAP_W} x {MAP_H})", png.display()));
        }
        let ground = img.pixels().map(|p| p.0[1]).collect();
        let mut plants: Vec<u8> = img.pixels().map(|p| p.0[2]).collect();
        if plants.iter().all(|&b| b == 0) {
            plants = Vec::new();
        }
        let text = std::fs::read_to_string(&txt).map_err(|e| format!("{}: {e}", txt.display()))?;
        let items = Items::parse(&text).map_err(|e| format!("{}: {e}", txt.display()))?;
        Ok(Map { w, h, ground, plants, items })
    }

    /// Writes `<dir>/<name>.png` (R 0, B the plants, A 255) and `<dir>/<name>.txt`.
    pub fn save(&self, dir: &Path, name: &str) -> Result<(), String> {
        let img = image::RgbaImage::from_fn(self.w as u32, self.h as u32, |x, y| {
            let i = y as usize * self.w + x as usize;
            image::Rgba([0, self.ground[i], if self.plants.is_empty() { 0 } else { self.plants[i] }, 255])
        });
        let png = dir.join(format!("{name}.png"));
        img.save(&png).map_err(|e| format!("{}: {e}", png.display()))?;
        let txt = dir.join(format!("{name}.txt"));
        std::fs::write(&txt, self.items.text()).map_err(|e| format!("{}: {e}", txt.display()))
    }

    /// The G byte under fine cell (`fx`, `fy`) of a world at `scale`: the
    /// map cell at the fine cell's centre.
    pub fn ground_fine(&self, scale: usize, fx: usize, fy: usize) -> u8 {
        let k = MAP_SCALE / scale as f32;
        let x = (((fx as f32 + 0.5) * k) as usize).min(self.w - 1);
        let y = (((fy as f32 + 0.5) * k) as usize).min(self.h - 1);
        self.ground[y * self.w + x]
    }

    /// What the squad can't walk to from its landing (owner, 2026-11-14:
    /// every map must stay beatable by any strategy, so water must never
    /// shut the squad off a crystal or a nest): each crystal with no
    /// standable cell within its radius + `NEAR` that connects to the
    /// landing by standable cells (4-neighbour, so a ford one diagonal
    /// wide doesn't count), and each mould start with none within `NEAR`.
    /// Empty with no landing. `("crystal", i)` or `("slime start", i)`.
    pub fn cut_off(&self) -> Vec<(&'static str, usize)> {
        const NEAR: f32 = 8.0;
        let Some(l) = self.items.landing else { return Vec::new() };
        let (mw, mh) = (self.w, self.h);
        let ok = |i: usize| standable(self.ground[i]);
        let mut seen = vec![false; mw * mh];
        let start = (l.1 as usize).min(mh - 1) * mw + (l.0 as usize).min(mw - 1);
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(i) = stack.pop() {
            let (x, y) = (i % mw, i / mw);
            let n = [
                (x > 0).then(|| i - 1),
                (x + 1 < mw).then(|| i + 1),
                (y > 0).then(|| i - mw),
                (y + 1 < mh).then(|| i + mw),
            ];
            for j in n.into_iter().flatten() {
                if !seen[j] && ok(j) {
                    seen[j] = true;
                    stack.push(j);
                }
            }
        }
        let near = |c: (f32, f32), r: f32| {
            let (x0, x1) = (
                (c.0 - r).floor().max(0.0) as usize,
                ((c.0 + r).ceil() as usize).min(mw - 1),
            );
            let (y0, y1) = (
                (c.1 - r).floor().max(0.0) as usize,
                ((c.1 + r).ceil() as usize).min(mh - 1),
            );
            (y0..=y1).any(|y| {
                (x0..=x1).any(|x| seen[y * mw + x] && (x as f32 + 0.5 - c.0).hypot(y as f32 + 0.5 - c.1) <= r)
            })
        };
        let mut out = Vec::new();
        for (i, c) in self.items.crystals.iter().enumerate() {
            if !near((c.x, c.y), c.r + NEAR) {
                out.push(("crystal", i));
            }
        }
        for (i, &m) in self.items.moulds.iter().enumerate() {
            if !near(m, NEAR) {
                out.push(("slime start", i));
            }
        }
        out
    }

    /// The B byte (`plants::unpack`) under fine cell (`fx`, `fy`), sampled
    /// as `ground_fine` samples the ground; 0 on a map with no plants.
    pub fn plant_fine(&self, scale: usize, fx: usize, fy: usize) -> u8 {
        if self.plants.is_empty() {
            return 0;
        }
        let k = MAP_SCALE / scale as f32;
        let x = (((fx as f32 + 0.5) * k) as usize).min(self.w - 1);
        let y = (((fy as f32 + 0.5) * k) as usize).min(self.h - 1);
        self.plants[y * self.w + x]
    }

    /// Map cells to dish cells.
    pub fn dish(p: (f32, f32)) -> (f32, f32) {
        (p.0 / MAP_SCALE, p.1 / MAP_SCALE)
    }

    /// A made world as a map, so a noise world can be kept, edited and
    /// loaded (`BigDish::with_map`): each map cell takes the rock and wet
    /// of the fine cell it falls in, the piles become crystals in their
    /// order, and the mould's starts are the dish's `homes`. Taken before any
    /// wall is built (walls are items, and built cells are not segments).
    pub fn of_dish(d: &BigDish) -> Map {
        assert!(d.wall.is_empty(), "a dish with no walls built");
        let s = d.p.scale;
        let (w, h) = (d.w * 4 / s, d.h * 4 / s);
        let on = |v: &[bool], i: usize| !v.is_empty() && v[i];
        let ground = (0..w * h)
            .map(|c| {
                let i = (c / w * s / 4) * d.w + c % w * s / 4;
                let soil = if d.soil.is_empty() { 0 } else { d.soil[i] << 4 };
                soil | if on(&d.rock, i) {
                    G_ROCK
                } else if on(&d.wet, i) {
                    G_WET
                } else {
                    0
                }
            })
            .collect();
        let plants = if d.plant.is_empty() {
            Vec::new()
        } else {
            (0..w * h)
                .map(|c| {
                    let i = (c / w * s / 4) * d.w + c % w * s / 4;
                    crate::mould::plants::pack(d.plant_kind[i], d.plant[i])
                })
                .collect()
        };
        let k = MAP_SCALE;
        let crystals = (0..d.piles.len()).map(|p| Crystal { x: d.piles[p].0 * k, y: d.piles[p].1 * k, r: d.pile_rs[p] * k, charge: d.pile_fill[p] }).collect();
        let items = Items { crystals, moulds: d.homes.iter().map(|&(x, y)| (x * k, y * k)).collect(), landing: d.landing.map(|(x, y)| (x * k, y * k)), walls: Vec::new(), moisture: d.moisture_of, flora: d.flora_of };
        Map { w, h, ground, plants, items }
    }
}

/// A test map: plain ground split top to bottom by a deep river (map x
/// 480-543), with a shallow ford across it at y 280-359 if `ford` and a
/// bridge at y 440-471 if `bridge`; the landing west of it at (300, 500),
/// a crystal and the mould's start east of it.
pub fn river(ford: bool, bridge: bool) -> Map {
    let mut ground = vec![0u8; MAP_W * MAP_H];
    for y in 0..MAP_H {
        for x in 480..544 {
            ground[y * MAP_W + x] = if ford && (280..360).contains(&y) {
                G_WET
            } else if bridge && (440..472).contains(&y) {
                G_BRIDGE
            } else {
                G_DEEP
            };
        }
    }
    let items = Items {
        crystals: vec![
            Crystal {
                x: 800.0,
                y: 320.0,
                r: 30.0,
                charge: 384.0,
            },
            Crystal {
                x: 300.0,
                y: 200.0,
                r: 20.0,
                charge: 384.0,
            },
        ],
        moulds: vec![(860.0, 320.0)],
        landing: Some((300.0, 500.0)),
        walls: Vec::new(),
        moisture: None,
        flora: None,
    };
    Map { w: MAP_W, h: MAP_H, ground, items, plants: Vec::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deep water cuts the squad off what lies across it until a ford or a
    /// bridge joins the banks (`Map::cut_off`, the editor's warning).
    #[test]
    fn deep_water_cuts_off_until_a_ford_or_bridge() {
        assert_eq!(river(false, false).cut_off(), vec![("crystal", 0), ("slime start", 0)]);
        assert!(river(true, false).cut_off().is_empty());
        assert!(river(false, true).cut_off().is_empty());
        // Deep water loads wet (older builds' reading of the byte), with
        // its soil kind, and is no soldier's ground; the bridge is dry.
        let d = BigDish::with_map(crate::mould::larder_world::race_params(1, 2, 500), &river(false, true));
        let at = |mx: usize, my: usize| (my / 2) * d.w + mx / 2;
        assert!(d.wet[at(500, 100)] && d.deep(at(500, 100)) && d.agar[at(500, 100)]);
        assert!(!d.wet[at(500, 450)] && d.soil[at(500, 450)] == crate::mould::big::SOIL_BRIDGE);
        assert!(!standable(G_DEEP) && standable(G_BRIDGE) && standable(G_WET) && !standable(G_ROCK));
    }

    #[test]
    fn items_parse_and_write_back() {
        let text = "# a test\ncrystal 512 600 12 384\nmould 512 590\n\nlanding 512 180  # the squad\nwall 100 100 200 120 8 400\n";
        let it = Items::parse(text).unwrap();
        assert_eq!(it.crystals, vec![Crystal { x: 512.0, y: 600.0, r: 12.0, charge: 384.0 }]);
        assert_eq!((it.moulds.clone(), it.landing), (vec![(512.0, 590.0)], Some((512.0, 180.0))));
        assert_eq!(it.walls, vec![Wall { a: (100.0, 100.0), b: (200.0, 120.0), thick: 8.0, hp: 400.0 }]);
        assert_eq!(Items::parse(&it.text()).unwrap(), it);
        assert!(Items::parse("crystal 1 2 3").unwrap_err().contains("takes 4"));
        assert!(Items::parse("tree 1 2").unwrap_err().contains("unknown item"));
        assert!(Items::parse("mould 1 x").is_err());
    }

    #[test]
    fn a_world_at_scale_two_takes_every_other_cell() {
        let mut m = Map::blank(MAP_W, MAP_H);
        m.ground[3 * MAP_W + 5] = G_ROCK;
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
            assert_eq!((a.piles.clone(), a.pile_rs.clone(), a.pile_fill.clone(), a.homes.clone()), (b.piles.clone(), b.pile_rs.clone(), b.pile_fill.clone(), b.homes.clone()));
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
        let (start, landing) = (m.items.moulds[0], m.items.landing.unwrap());
        assert_eq!(d.piles.len(), m.items.crystals.len());
        assert_eq!(d.homes, vec![Map::dish(start)]);
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
        assert!((mx / s - d.homes[0].0).hypot(my / s - d.homes[0].1) < 3.0, "blob at ({}, {}), home {:?}", mx / s, my / s, d.homes);
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
            let soil = |k: u8| d.soil.iter().filter(|&&v| v == k).count();
            let want_soil = |k: u8| (0..d.w * d.h).filter(|&i| m.ground_fine(2, i % d.w, i / d.w) >> 4 == k).count();
            for k in 1..=3 {
                assert_eq!(soil(k), want_soil(k), "{name}: soil {k}");
            }
            rocks += count(&d.rock);
        }
        assert!(rocks > 0, "rock to compare");
        // The Fork's start is 12 dish cells from the rim, so its blob does
        // not fit at the usual size and widens until all its walkers do.
        let fork = BigDish::with_map(race_params(1, 2, 5000), &Map::load(&dir, "fork").unwrap());
        assert_eq!(fork.alive.iter().filter(|&&a| a).count(), 5000);
        // The Passes has two starts: half the walkers each, each blob on its own.
        let m = Map::load(&dir, "passes").unwrap();
        let d = BigDish::with_map(race_params(1, 2, 5000), &m);
        assert_eq!(d.homes.len(), 2);
        for &(hx, hy) in &d.homes {
            let near = d.walkers.iter().zip(&d.alive).filter(|&(w, &a)| a && (w.0 / s - hx).hypot(w.1 / s - hy) < 40.0).count();
            assert_eq!(near, 2500, "start ({hx}, {hy})");
        }
    }

    /// A map carries its own size: planets smaller (planet 1, 60% of the
    /// designed maps' area) and bigger (the core world, 6x) save, load and
    /// make a world of their own size with the map's rock and crystals.
    #[test]
    fn a_map_of_another_size_makes_its_world() {
        use crate::mould::levelgen::{size_of, spec};
        let dir = std::env::temp_dir().join(format!("map-size-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        for (depth, area) in [(1, 0.6), (8, 6.0)] {
            let (p, m) = spec(&format!("planet:{depth}:2:{area}"), 1).unwrap().unwrap();
            assert_eq!((m.w, m.h), size_of(area));
            assert_eq!(p.size, size_of(area));
            assert_ne!((m.w, m.h), (MAP_W, MAP_H));
            m.save(&dir, "p").unwrap();
            let back = Map::load(&dir, "p").unwrap();
            assert_eq!((back.w, back.h), (m.w, m.h));
            assert!(back.ground == m.ground && back.items.text() == m.items.text(), "depth {depth} round trip");
            let d = BigDish::with_map(race_params(1, 2, 3000), &back);
            assert_eq!((d.w, d.h), (m.w / 2, m.h / 2), "depth {depth}");
            let rock = d.rock.iter().filter(|&&r| r).count();
            let want = (0..d.w * d.h).filter(|&i| m.ground_fine(2, i % d.w, i / d.w) & G_ROCK != 0).count();
            assert_eq!(rock, want, "depth {depth}: rock");
            assert_eq!(d.piles.len(), m.items.crystals.len());
            assert!(d.piles.iter().all(|&(x, y)| x < (d.w / 2) as f32 && y < (d.h / 2) as f32));
        }
        std::fs::remove_dir_all(&dir).ok();
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
                assert!(!m.items.crystals.is_empty() && !m.items.moulds.is_empty() && m.items.landing.is_some(), "{name}");
                n += 1;
            }
        }
        assert!(n >= 5, "{n} maps");
    }
}
