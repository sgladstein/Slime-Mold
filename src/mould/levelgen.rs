//! **Procedural planets** for the roguelike run (thread "Generate roguelike
//! levels procedurally", 2026-11-14). The owner's story: the slime is taking
//! over planets, and each level is a new planet on the way to the core slime
//! world (15:25Z, "head canon ... it could guide our level design"). A
//! planet is a `map::Map` like the five designed ones, so the game builds it
//! with `GpuWorld::with_map` and the editor opens it.
//!
//! **The interface** (agreed with the run thread, 15:25Z): `level(seed,
//! depth)` is the map and `planet(seed, depth)` the card the shop shows
//! (name, biome, size), cheap, with no rasterising. Both are pure: the same
//! seed and depth give the same planet on every machine. That is why this
//! file uses no `sin`, `cos`, `exp`, `powf` or `hypot`: those come from the
//! platform's maths library and differ between Windows and Linux (HANDOFF,
//! decision 6). Only `+ - * /`, `sqrt` and `floor`, which IEEE fixes.
//!
//! **Recipes, not noise.** A planet is regions round a few sites (the
//! landing, the slime's nests, open fields), drawn as a warped Voronoi
//! diagram. Region borders become rock ridges, open ground, ridges with a
//! pass (a chokepoint) or a pass shut by soft rock (walls the slime and the
//! squad chew through). A random spanning tree of the regions is always
//! passable, so everything can be reached; the rest of the borders decide
//! whether there is one route or several. The biome sets the soil (the
//! ground kinds the simulation plays: sand spreads the slime, clay holds
//! it, wet slows it, barren starves it), how much is ridge, and the extras.
//! Then crystals: a home crystal under each nest, one near the landing, big
//! contested ones between, small ones scattered. A built planet is checked
//! (`check`) and rebuilt from the next attempt's stream if it fails.

use super::map::{Crystal, Items, Map, Wall, G_ROCK, G_WET, MAP_H, MAP_W};

/// Soil kinds in the G byte's high nibble (`big::SOIL_*` << 4).
pub const G_SAND: u8 = 0x10;
pub const G_CLAY: u8 = 0x20;
pub const G_BARREN: u8 = 0x30;

/// How many builds `generate` tries before it falls back to an open plan.
pub const ATTEMPTS: u32 = 24;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Biome {
    /// Barren flats: bands of ground that starve the slime between you and
    /// it. The easiest.
    Barrens,
    /// Pools of wet ground the slime crawls through, on clay banks.
    Marsh,
    /// Rock ridges and narrow passes: chokepoints.
    Canyon,
    /// Sand everywhere, clay islands under the crystals: the slime races.
    Dunes,
    /// Red clay: the slime digs in and is slow to leave.
    Claypan,
    /// Soft-rock hedges and walled nests.
    Thicket,
    /// Rock spires everywhere: cover the slime grows behind, so the squad
    /// has to move to shoot it.
    Spires,
    /// The core world: every ground, several nests.
    Core,
}

impl Biome {
    pub const ALL: [Biome; 8] = [
        Biome::Barrens,
        Biome::Marsh,
        Biome::Canyon,
        Biome::Dunes,
        Biome::Claypan,
        Biome::Thicket,
        Biome::Spires,
        Biome::Core,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Biome::Barrens => "barrens",
            Biome::Marsh => "marsh",
            Biome::Canyon => "canyon",
            Biome::Dunes => "dunes",
            Biome::Claypan => "claypan",
            Biome::Thicket => "thicket",
            Biome::Spires => "spires",
            Biome::Core => "core",
        }
    }

    pub fn from_name(s: &str) -> Option<Biome> {
        Biome::ALL.into_iter().find(|b| b.name() == s)
    }

    /// The set order (owner, 2026-11-14 15:33Z): the biomes that starve
    /// or slow the slime first, the ones that feed or shelter it later, the
    /// core world last.
    pub const ORDER: [Biome; 7] = [
        Biome::Barrens,
        Biome::Marsh,
        Biome::Canyon,
        Biome::Dunes,
        Biome::Claypan,
        Biome::Thicket,
        Biome::Spires,
    ];

    /// The biome of the planet at `depth` (from 1): the order, then the
    /// core world at `CORE_DEPTH`, then the order again.
    pub fn at(depth: u32) -> Biome {
        let d = depth.max(1);
        if d == CORE_DEPTH {
            Biome::Core
        } else {
            let k = if d > CORE_DEPTH { d - 2 } else { d - 1 };
            Biome::ORDER[k as usize % Biome::ORDER.len()]
        }
    }
}

/// The core world's depth: a run is eight planets (the run thread's
/// `run::PLANETS`, 2026-11-14 15:40Z), the core world the last.
pub const CORE_DEPTH: u32 = 8;

/// What the shop shows before a planet is built.
#[derive(Clone, Debug, PartialEq)]
pub struct Planet {
    pub seed: u64,
    pub depth: u32,
    pub name: String,
    pub biome: Biome,
    /// Map cells, as `MAP_W` x `MAP_H` (fixed until maps carry a size).
    pub size: (usize, usize),
}

/// The planet at `depth` (from 1) of the run with `seed`.
pub fn planet(seed: u64, depth: u32) -> Planet {
    let mut r = Rng::new(seed, depth, STREAM_PLANET);
    let biome = Biome::at(depth);
    let name = planet_name(&mut r);
    Planet {
        seed,
        depth,
        name,
        biome,
        size: (MAP_W, MAP_H),
    }
}

/// The map of the planet at `depth` of the run with `seed`.
pub fn level(seed: u64, depth: u32) -> Map {
    generate(&planet(seed, depth))
}

/// A `map=` value naming a planet: `planet` (depth 1), `planet:DEPTH` or
/// `planet:DEPTH:SEED`, the seed otherwise `seed` (the window's `seed=`).
/// `None` when `s` names no planet.
pub fn spec(s: &str, seed: u64) -> Option<Result<(Planet, Map), String>> {
    Some(spec_planet(s, seed)?.map(|p| {
        let m = generate(&p);
        (p, m)
    }))
}

/// `spec`'s planet card alone, without building the map.
pub fn spec_planet(s: &str, seed: u64) -> Option<Result<Planet, String>> {
    let rest = s.strip_prefix("planet")?;
    if !(rest.is_empty() || rest.starts_with(':')) {
        return None;
    }
    let read = || -> Result<Planet, String> {
        let words: Vec<&str> = rest.split(':').skip(1).collect();
        if words.len() > 2 {
            return Err(format!("{s}: planet, planet:DEPTH or planet:DEPTH:SEED"));
        }
        let depth = match words.first() {
            Some(d) => d.parse::<u32>().map_err(|e| format!("{s}: depth {d:?}: {e}"))?,
            None => 1,
        };
        if depth == 0 {
            return Err(format!("{s}: depths start at 1"));
        }
        let seed = match words.get(1) {
            Some(v) => v.parse::<u64>().map_err(|e| format!("{s}: seed {v:?}: {e}"))?,
            None => seed,
        };
        Ok(planet(seed, depth))
    };
    Some(read())
}

/// Builds `p`, checking each attempt; the first that passes is the planet.
pub fn generate(p: &Planet) -> Map {
    for attempt in 0..ATTEMPTS {
        let m = build(p, attempt);
        if check(p, &m).is_empty() {
            return m;
        }
    }
    // Every attempt failed a check (none has, over the seeds the tests
    // walk): an open plan, with no ridges, which reaches everything.
    build_with(
        p,
        ATTEMPTS,
        Some(Recipe {
            open: 1.0,
            gate: 0.0,
            outcrop: 2.0,
            ..Recipe::of(p)
        }),
    )
}

// ---------------------------------------------------------------------
// Randomness: SplitMix64, keyed by seed, depth and a stream.

const STREAM_PLANET: u64 = 1;
const STREAM_BUILD: u64 = 2;
const STREAM_NOISE: u64 = 3;

#[derive(Clone)]
pub struct Rng(u64);

fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

impl Rng {
    pub fn new(seed: u64, depth: u32, stream: u64) -> Rng {
        Rng(mix(
            mix(seed ^ 0x5EED_1E7E_15CA_FE00) ^ mix((depth as u64) << 20 | stream)
        ))
    }

    pub fn draw(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        mix(self.0)
    }

    /// Uniform in [0, 1).
    pub fn f(&mut self) -> f32 {
        (self.draw() >> 40) as f32 / (1u64 << 24) as f32
    }

    pub fn range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.f()
    }

    pub fn below(&mut self, n: usize) -> usize {
        (self.draw() % n.max(1) as u64) as usize
    }

    pub fn chance(&mut self, p: f32) -> bool {
        self.f() < p
    }
}

/// Value noise on a lattice `cell` map cells apart, in [0, 1): smoothstep
/// between hashed corners, so only multiplies and adds.
struct Noise {
    key: u64,
}

impl Noise {
    fn corner(&self, ix: i32, iy: i32) -> f32 {
        let h = mix(self.key ^ mix((ix as u32 as u64) << 32 | iy as u32 as u64));
        (h >> 40) as f32 / (1u64 << 24) as f32
    }

    fn at(&self, x: f32, y: f32, cell: f32) -> f32 {
        let (fx, fy) = (x / cell, y / cell);
        let (x0, y0) = (fx.floor(), fy.floor());
        let (tx, ty) = (fx - x0, fy - y0);
        let (sx, sy) = (tx * tx * (3.0 - 2.0 * tx), ty * ty * (3.0 - 2.0 * ty));
        let (ix, iy) = (x0 as i32, y0 as i32);
        let a = self.corner(ix, iy) + (self.corner(ix + 1, iy) - self.corner(ix, iy)) * sx;
        let b = self.corner(ix, iy + 1) + (self.corner(ix + 1, iy + 1) - self.corner(ix, iy + 1)) * sx;
        a + (b - a) * sy
    }

    /// Three octaves, in [0, 1).
    fn fbm(&self, x: f32, y: f32, cell: f32) -> f32 {
        (self.at(x, y, cell) * 4.0
            + self.at(x + 31.7, y - 12.3, cell / 2.0) * 2.0
            + self.at(x - 7.1, y + 55.9, cell / 4.0))
            / 7.0
    }
}

fn len(dx: f32, dy: f32) -> f32 {
    (dx * dx + dy * dy).sqrt()
}

fn dist(a: (f32, f32), b: (f32, f32)) -> f32 {
    len(a.0 - b.0, a.1 - b.1)
}

/// A noise field sampled every `STEP` map cells and read back bilinearly:
/// the borders' warp and the soil patches are smooth over tens of cells, so
/// sampling every cell would cost sixteen times as much for the same map.
struct Field {
    gw: usize,
    v: Vec<f32>,
}

const STEP: usize = 4;

impl Field {
    fn new(w: usize, h: usize, f: impl Fn(f32, f32) -> f32) -> Field {
        let (gw, gh) = (w / STEP + 2, h / STEP + 2);
        let mut v = Vec::with_capacity(gw * gh);
        for gy in 0..gh {
            for gx in 0..gw {
                v.push(f((gx * STEP) as f32, (gy * STEP) as f32));
            }
        }
        Field { gw, v }
    }

    fn at(&self, x: usize, y: usize) -> f32 {
        let (gx, gy, tx, ty) = (
            x / STEP,
            y / STEP,
            (x % STEP) as f32 / STEP as f32,
            (y % STEP) as f32 / STEP as f32,
        );
        let i = gy * self.gw + gx;
        let a = self.v[i] + (self.v[i + 1] - self.v[i]) * tx;
        let b = self.v[i + self.gw] + (self.v[i + self.gw + 1] - self.v[i + self.gw]) * tx;
        a + (b - a) * ty
    }
}

// ---------------------------------------------------------------------
// The recipe: a biome's numbers at a depth.

#[derive(Clone, Copy, Debug)]
pub struct Recipe {
    /// Regions (Voronoi sites), the landing's and the nests' included.
    pub regions: usize,
    /// Chance a border along the route (landing to nest) is open ground.
    pub open: f32,
    /// Chance a border across the route is a ridge: the lines a squad has
    /// to pass through.
    pub front: f32,
    /// Chance an extra ridge has a pass anyway: more than one route.
    pub loops: f32,
    /// Chance a pass is shut with soft rock.
    pub gate: f32,
    /// Ridge half-width, map cells (the noise thins and thickens it).
    pub ridge: f32,
    /// Pass radius range, map cells.
    pub pass: (f32, f32),
    /// Warp of the region borders, map cells.
    pub warp: f32,
    /// Outcrop (mesa) threshold on noise: higher is fewer, 2 none.
    pub outcrop: f32,
    /// Spire threshold on a finer noise: small rock pillars, 2 none.
    pub spires: f32,
    /// Slime nests (mould starts).
    pub nests: usize,
    /// Small scattered crystals.
    pub small: usize,
    /// Big contested crystals between the landing and the nests.
    pub big: usize,
    /// Chance a nest gets a soft-rock ring, with one gap facing the landing.
    pub ring: f32,
    /// Soft-rock hedges (curved wall lines).
    pub hedges: usize,
}

impl Recipe {
    pub fn of(p: &Planet) -> Recipe {
        let d = p.depth.max(1) as f32;
        // Nests: one, then two from depth 4, three from 8.
        let nests = 1 + (p.depth >= 4) as usize + (p.depth >= 8) as usize;
        let base = Recipe {
            regions: 11,
            open: 0.7,
            front: 0.6,
            loops: 0.4,
            gate: 0.15,
            ridge: 13.0,
            pass: (24.0, 36.0),
            warp: 60.0,
            outcrop: 0.7,
            spires: 2.0,
            nests,
            small: 12,
            big: 2,
            ring: 0.0,
            hedges: 0,
        };
        // Passes narrow with depth, to a floor a squad still walks through.
        let narrow = (1.0 - 0.03 * (d - 1.0)).max(0.7);
        let r = match p.biome {
            Biome::Barrens => Recipe { front: 0.45, ..base },
            Biome::Marsh => Recipe {
                front: 0.4,
                ridge: 10.0,
                outcrop: 0.74,
                ..base
            },
            Biome::Canyon => Recipe {
                regions: 13,
                open: 0.3,
                front: 0.9,
                loops: 0.35,
                gate: 0.2,
                ridge: 20.0,
                pass: (22.0, 32.0),
                outcrop: 0.66,
                ..base
            },
            Biome::Dunes => Recipe {
                open: 0.85,
                front: 0.35,
                ridge: 11.0,
                small: 18,
                big: 1,
                outcrop: 0.74,
                ..base
            },
            Biome::Claypan => Recipe { ..base },
            Biome::Thicket => Recipe {
                front: 0.5,
                gate: 0.6,
                ring: 1.0,
                hedges: 9,
                ..base
            },
            Biome::Spires => Recipe {
                open: 0.85,
                front: 0.3,
                ridge: 11.0,
                outcrop: 0.76,
                spires: 0.59,
                ..base
            },
            Biome::Core => Recipe {
                regions: 15,
                open: 0.6,
                front: 0.7,
                gate: 0.35,
                ring: 0.6,
                hedges: 5,
                nests: nests.max(3),
                small: 14,
                big: 3,
                ..base
            },
        };
        Recipe {
            pass: (r.pass.0 * narrow, r.pass.1 * narrow),
            ..r
        }
    }
}

// ---------------------------------------------------------------------
// Building.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Border {
    Open,
    Ridge,
    /// A ridge with a pass at `Edge::pass`.
    Pass,
    /// A pass shut with soft rock.
    Gate,
}

#[derive(Clone, Debug)]
struct Edge {
    a: usize,
    b: usize,
    kind: Border,
    /// Border cells (map cell indices).
    cells: Vec<u32>,
    pass: Option<((f32, f32), f32)>,
}

/// Attempt `attempt` at `p`, unchecked (`generate` checks each).
pub fn build(p: &Planet, attempt: u32) -> Map {
    build_with(p, attempt, None)
}

fn unit(dx: f32, dy: f32) -> (f32, f32) {
    let l = len(dx, dy).max(1e-6);
    (dx / l, dy / l)
}

fn build_with(p: &Planet, attempt: u32, recipe: Option<Recipe>) -> Map {
    let rc = recipe.unwrap_or_else(|| Recipe::of(p));
    let (w, h) = p.size;
    let (wf, hf) = (w as f32, h as f32);
    let mut r = Rng::new(p.seed ^ mix(attempt as u64 + 1), p.depth, STREAM_BUILD);
    let noise = |k: u64| Noise {
        key: mix(p.seed ^ mix(k << 8 | attempt as u64) ^ (p.depth as u64) << 40) ^ STREAM_NOISE,
    };

    // The landing: near a short edge mostly, so the slime is a long way
    // off; near a long edge's end otherwise, so it is across a diagonal.
    let inset = r.range(90.0, 130.0);
    let landing = {
        let k = r.f();
        let along = r.range(0.25, 0.75);
        let end = if r.chance(0.5) {
            r.range(0.12, 0.3)
        } else {
            r.range(0.7, 0.88)
        };
        let edge = inset.min(hf * 0.18);
        if k < 0.4 {
            (inset, hf * along)
        } else if k < 0.8 {
            (wf - inset, hf * along)
        } else if k < 0.9 {
            (wf * end, edge)
        } else {
            (wf * end, hf - edge)
        }
    };
    let far = (wf - landing.0, hf - landing.1);

    // Nests: the first across the map, the rest in the far part, apart.
    let mut nests: Vec<(f32, f32)> = Vec::new();
    let mut first = (far.0.clamp(110.0, wf - 110.0), far.1.clamp(100.0, hf - 100.0));
    for _ in 0..40 {
        let q = (far.0 + r.range(-0.15, 0.15) * wf, far.1 + r.range(-0.3, 0.3) * hf);
        let q = (q.0.clamp(110.0, wf - 110.0), q.1.clamp(100.0, hf - 100.0));
        if dist(q, landing) > 0.6 * wf {
            first = q;
            break;
        }
    }
    nests.push(first);
    for _ in 1..rc.nests {
        for _ in 0..300 {
            let q = (r.range(110.0, wf - 110.0), r.range(100.0, hf - 100.0));
            if dist(q, landing) > 0.5 * wf && nests.iter().all(|&n| dist(n, q) > 280.0) {
                nests.push(q);
                break;
            }
        }
    }
    let axis = unit(first.0 - landing.0, first.1 - landing.1);

    // Sites: the landing, the nests, then fields by dart throwing.
    let mut sites = vec![landing];
    sites.extend(&nests);
    let spacing = (wf * hf / rc.regions as f32).sqrt() * 0.75;
    for _ in 0..4000 {
        if sites.len() >= rc.regions {
            break;
        }
        let q = (r.range(40.0, wf - 40.0), r.range(40.0, hf - 40.0));
        if sites.iter().all(|&s| dist(s, q) > spacing) {
            sites.push(q);
        }
    }
    let ns = sites.len();

    // Noise fields.
    let (nx, ny, nr, nb, na, nw) = (noise(1), noise(2), noise(3), noise(4), noise(5), noise(6));
    let warp_x = Field::new(w, h, |x, y| (nx.fbm(x, y, 200.0) - 0.5) * 2.0 * rc.warp);
    let warp_y = Field::new(w, h, |x, y| (ny.fbm(x, y, 200.0) - 0.5) * 2.0 * rc.warp);
    let rough = Field::new(w, h, |x, y| nr.fbm(x, y, 90.0));
    let blobs = Field::new(w, h, |x, y| nb.fbm(x, y, 75.0));
    let fine = Field::new(w, h, |x, y| nb.fbm(x - 911.0, y + 377.0, 26.0));
    let soil_a = Field::new(w, h, |x, y| na.fbm(x, y, 130.0));
    let soil_b = Field::new(w, h, |x, y| nw.fbm(x, y, 60.0));

    // Warped Voronoi: each cell's region, the runner-up, and its distance
    // to their border.
    let n = w * h;
    let mut region = vec![0u16; n];
    let mut second = vec![0u16; n];
    let mut border = vec![0f32; n];
    for y in 0..h {
        for x in 0..w {
            let px = x as f32 + 0.5 + warp_x.at(x, y);
            let py = y as f32 + 0.5 + warp_y.at(x, y);
            let (mut d1, mut d2, mut i1, mut i2) = (f32::MAX, f32::MAX, 0usize, 0usize);
            for (k, s) in sites.iter().enumerate() {
                let d = (px - s.0) * (px - s.0) + (py - s.1) * (py - s.1);
                if d < d1 {
                    (d2, i2, d1, i1) = (d1, i1, d, k);
                } else if d < d2 {
                    (d2, i2) = (d, k);
                }
            }
            let i = y * w + x;
            region[i] = i1 as u16;
            second[i] = i2 as u16;
            border[i] = (d2 - d1) / (2.0 * dist(sites[i1], sites[i2]).max(1.0));
        }
    }

    // Borders between regions, with their cells.
    let mut edge_of = vec![usize::MAX; ns * ns];
    let mut edges: Vec<Edge> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let a = region[i] as usize;
            for j in [
                if x + 1 < w { Some(i + 1) } else { None },
                if y + 1 < h { Some(i + w) } else { None },
            ]
            .into_iter()
            .flatten()
            {
                let b = region[j] as usize;
                if a != b {
                    let (lo, hi) = (a.min(b), a.max(b));
                    if edge_of[lo * ns + hi] == usize::MAX {
                        edges.push(Edge {
                            a: lo,
                            b: hi,
                            kind: Border::Open,
                            cells: Vec::new(),
                            pass: None,
                        });
                        edge_of[lo * ns + hi] = edges.len() - 1;
                        edge_of[hi * ns + lo] = edges.len() - 1;
                    }
                    edges[edge_of[lo * ns + hi]].cells.push(i as u32);
                }
            }
        }
    }

    // A random spanning tree is always passable: borders across the route
    // get passes (the chokepoints), borders along it are mostly open. Off
    // the tree, the recipe's odds.
    let mut order: Vec<usize> = (0..edges.len()).collect();
    for i in (1..order.len()).rev() {
        order.swap(i, r.below(i + 1));
    }
    let mut parent: Vec<usize> = (0..ns).collect();
    fn root(p: &mut [usize], mut i: usize) -> usize {
        while p[i] != i {
            p[i] = p[p[i]];
            i = p[i];
        }
        i
    }
    for &e in &order {
        let (a, b) = (edges[e].a, edges[e].b);
        let (ra, rb) = (root(&mut parent, a), root(&mut parent, b));
        let tree = ra != rb;
        if tree {
            parent[ra] = rb;
        }
        let u = unit(sites[b].0 - sites[a].0, sites[b].1 - sites[a].1);
        let front = (u.0 * axis.0 + u.1 * axis.1).abs() > 0.6;
        // Tiny borders (where three regions meet) are open: a pass there
        // would be a hole in a corner.
        let tiny = edges[e].cells.len() < 50;
        let shut = !tiny && if front { r.chance(rc.front) } else { !r.chance(rc.open) };
        edges[e].kind = if !shut {
            Border::Open
        } else if tree || r.chance(rc.loops) {
            if r.chance(rc.gate) {
                Border::Gate
            } else {
                Border::Pass
            }
        } else {
            Border::Ridge
        };
    }

    // Each pass: a border cell mid-border, away from the map's edge.
    for e in edges.iter_mut() {
        if !matches!(e.kind, Border::Pass | Border::Gate) {
            continue;
        }
        let (sa, sb) = (sites[e.a], sites[e.b]);
        let t = unit(-(sb.1 - sa.1), sb.0 - sa.0);
        let mut cells: Vec<(f32, u32)> = e
            .cells
            .iter()
            .map(|&c| {
                let (x, y) = ((c as usize % w) as f32 + 0.5, (c as usize / w) as f32 + 0.5);
                (x * t.0 + y * t.1, c)
            })
            .collect();
        cells.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));
        let rad = r.range(rc.pass.0, rc.pass.1);
        let k = ((cells.len() as f32 * r.range(0.3, 0.7)) as usize).min(cells.len() - 1);
        let mut at = None;
        for step in 0..cells.len() {
            // Outward from the chosen point, alternating sides.
            let off = step.div_ceil(2);
            let i = if step % 2 == 0 { k + off } else { k.wrapping_sub(off) };
            if i >= cells.len() {
                continue;
            }
            let c = cells[i].1 as usize;
            let (x, y) = ((c % w) as f32 + 0.5, (c / w) as f32 + 0.5);
            if x > rad + 16.0 && y > rad + 16.0 && x < wf - rad - 16.0 && y < hf - rad - 16.0 {
                at = Some((x, y));
                break;
            }
        }
        match at {
            Some(c) => e.pass = Some((c, rad)),
            // A border that only runs along the map's edge: open.
            None => e.kind = Border::Open,
        }
    }

    // Rock: ridges along shut borders, outside their passes, thinning and
    // thickening along their length; mesas in the fields, off the sites. A
    // pass is a slot through the ridge: within its radius along the
    // border, and across the ridge's widest. (A disc on the border, the
    // first try, left a wall of rock wherever the ridge ran wider than the
    // disc: on the canyon, 2 of 3 seeds' first attempts had a nest and
    // half the crystals shut off.)
    let widest = rc.ridge * 1.65 + 6.0;
    let frames: Vec<((f32, f32), (f32, f32))> = edges
        .iter()
        .map(|e| {
            let (sa, sb) = (sites[e.a], sites[e.b]);
            let nrm = unit(sb.0 - sa.0, sb.1 - sa.1);
            ((-nrm.1, nrm.0), nrm)
        })
        .collect();
    let mut ground = vec![0u8; n];
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let at = (x as f32 + 0.5, y as f32 + 0.5);
            let half = rc.ridge * (0.35 + 1.3 * rough.at(x, y));
            let mut rock = false;
            if border[i] < half {
                let k = edge_of[region[i] as usize * ns + second[i] as usize];
                if k != usize::MAX {
                    let e = &edges[k];
                    let (t, nrm) = frames[k];
                    let in_slot = |(c, rad): ((f32, f32), f32)| {
                        let (dx, dy) = (at.0 - c.0, at.1 - c.1);
                        (dx * t.0 + dy * t.1).abs() < rad && (dx * nrm.0 + dy * nrm.1).abs() < widest
                    };
                    rock = match e.kind {
                        Border::Open => false,
                        Border::Ridge => true,
                        Border::Pass | Border::Gate => e.pass.is_none_or(|p| !in_slot(p)),
                    };
                }
            }
            if !rock && border[i] > 30.0 && (blobs.at(x, y) > rc.outcrop || fine.at(x, y) > rc.spires) {
                rock = !sites.iter().any(|&s| dist(s, at) < 120.0);
            }
            if rock {
                ground[i] = G_ROCK;
            }
        }
    }

    // Soil and water, by the biome, in patches that ignore the regions;
    // plain round the landing, clay under the nests where the biome holds
    // the slime there (as the designed maps do).
    let nest_clay = matches!(
        p.biome,
        Biome::Marsh | Biome::Dunes | Biome::Claypan | Biome::Thicket | Biome::Core
    );
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let at = (x as f32 + 0.5, y as f32 + 0.5);
            let (a, b) = (soil_a.at(x, y), soil_b.at(x, y));
            let dl = dist(at, landing);
            let dn = nests.iter().map(|&q| dist(at, q)).fold(f32::MAX, f32::min);
            // 0 at the landing, 1 at a nest.
            let t = dl / (dl + dn).max(1.0);
            let mut soil = 0u8;
            let mut wet = false;
            match p.biome {
                Biome::Barrens => {
                    // Bands across the middle of the route.
                    let band = if (0.25..0.75).contains(&t) { 0.12 } else { 0.0 };
                    if a + band > 0.6 {
                        soil = G_BARREN;
                    }
                }
                Biome::Marsh => {
                    if b > 0.6 {
                        wet = true;
                    } else if a > 0.55 {
                        soil = G_CLAY;
                    }
                }
                Biome::Canyon => {
                    if a > 0.57 {
                        soil = G_SAND;
                    }
                }
                Biome::Dunes => {
                    if a > 0.36 {
                        soil = G_SAND;
                    }
                }
                Biome::Claypan => {
                    if a > 0.38 {
                        soil = G_CLAY;
                    } else if b > 0.66 {
                        soil = G_BARREN;
                    }
                }
                Biome::Thicket => {
                    if a > 0.52 {
                        soil = G_CLAY;
                    }
                }
                Biome::Spires => {
                    if a > 0.56 {
                        soil = G_SAND;
                    } else if a < 0.4 {
                        soil = G_CLAY;
                    }
                }
                Biome::Core => {
                    if b > 0.66 {
                        wet = true;
                    } else if a > 0.6 {
                        soil = G_SAND;
                    } else if a < 0.38 {
                        soil = G_CLAY;
                    } else if b < 0.34 {
                        soil = G_BARREN;
                    }
                }
            }
            let ragged = (b - 0.5) * 80.0;
            if nest_clay && dn < 130.0 + ragged {
                (soil, wet) = (G_CLAY, false);
            }
            if dl < 150.0 + ragged {
                (soil, wet) = (0, false);
            }
            if wet && dn < 100.0 {
                wet = false;
            }
            if ground[i] & G_ROCK == 0 {
                ground[i] = soil | if wet { G_WET } else { 0 };
            }
        }
    }

    // Crystals.
    let mut items = Items {
        landing: Some(landing),
        ..Items::default()
    };
    let clear = |ground: &[u8], items: &Items, c: (f32, f32), rad: f32, gap: f32| -> bool {
        if c.0 < rad + 24.0 || c.1 < rad + 24.0 || c.0 > wf - rad - 24.0 || c.1 > hf - rad - 24.0 {
            return false;
        }
        if dist(c, landing) < rad + 55.0 {
            return false;
        }
        if items.crystals.iter().any(|o| dist((o.x, o.y), c) < o.r + rad + gap) {
            return false;
        }
        let m = rad + 12.0;
        let (x0, x1) = ((c.0 - m).max(0.0) as usize, ((c.0 + m) as usize).min(w - 1));
        let (y0, y1) = ((c.1 - m).max(0.0) as usize, ((c.1 + m) as usize).min(h - 1));
        for y in y0..=y1 {
            for x in x0..=x1 {
                if ground[y * w + x] & (G_ROCK | G_WET) != 0 && dist((x as f32 + 0.5, y as f32 + 0.5), c) <= m {
                    return false;
                }
            }
        }
        true
    };
    // A rich home under each nest; the slime starts on it, as on the
    // designed maps.
    for &q in &nests {
        let rad = r.range(33.0, 42.0);
        let charge = [384.0, 512.0, 640.0][r.below(3)];
        items.crystals.push(Crystal {
            x: q.0.round(),
            y: q.1.round(),
            r: rad.round(),
            charge,
        });
        items.moulds.push((q.0.round(), q.1.round()));
    }
    // One near the landing, toward the middle: a fair start.
    let inward = unit(wf / 2.0 - landing.0, hf / 2.0 - landing.1);
    for _ in 0..400 {
        let (dx, dy) = (r.range(-1.0, 1.0), r.range(-1.0, 1.0));
        let l = len(dx, dy);
        if !(0.2..=1.0).contains(&l) || (dx * inward.0 + dy * inward.1) / l < 0.3 {
            continue;
        }
        let d = r.range(85.0, 140.0);
        let c = (landing.0 + dx / l * d, landing.1 + dy / l * d);
        let rad = r.range(24.0, 28.0);
        if clear(&ground, &items, c, rad, 30.0) {
            items.crystals.push(Crystal {
                x: c.0.round(),
                y: c.1.round(),
                r: rad.round(),
                charge: [192.0, 256.0][r.below(2)],
            });
            break;
        }
    }
    // Big ones between: the middle of the way from the landing to a nest.
    let mut placed = 0;
    for _ in 0..800 {
        if placed >= rc.big {
            break;
        }
        let c = (r.range(60.0, wf - 60.0), r.range(60.0, hf - 60.0));
        let dl = dist(c, landing);
        let dn = nests.iter().map(|&q| dist(c, q)).fold(f32::MAX, f32::min);
        if !(0.35..=0.7).contains(&(dl / (dl + dn))) {
            continue;
        }
        let rad = r.range(30.0, 42.0);
        if clear(&ground, &items, c, rad, 70.0) {
            items.crystals.push(Crystal {
                x: c.0.round(),
                y: c.1.round(),
                r: rad.round(),
                charge: [384.0, 512.0][r.below(2)],
            });
            placed += 1;
        }
    }
    // Small ones scattered.
    let mut placed = 0;
    for _ in 0..3000 {
        if placed >= rc.small {
            break;
        }
        let c = (r.range(40.0, wf - 40.0), r.range(40.0, hf - 40.0));
        let rad = r.range(18.0, 24.0);
        if clear(&ground, &items, c, rad, 50.0) {
            items.crystals.push(Crystal {
                x: c.0.round(),
                y: c.1.round(),
                r: rad.round(),
                charge: [96.0, 128.0, 192.0][r.below(3)],
            });
            placed += 1;
        }
    }
    // Dunes: clay islands under the crystals, as on the Flats.
    if p.biome == Biome::Dunes {
        for c in &items.crystals {
            let m = c.r * 1.8;
            for y in ((c.y - m).max(0.0) as usize)..((c.y + m) as usize).min(h) {
                for x in ((c.x - m).max(0.0) as usize)..((c.x + m) as usize).min(w) {
                    let d = dist((x as f32 + 0.5, y as f32 + 0.5), (c.x, c.y)) / m + (soil_b.at(x, y) - 0.5) * 0.6;
                    let i = y * w + x;
                    if d < 1.0 && ground[i] & G_ROCK == 0 {
                        ground[i] = (ground[i] & 0x0F) | G_CLAY;
                    }
                }
            }
        }
    }

    // Soft rock: gates across passes, rings round nests, hedges.
    const THICK: f32 = 16.0;
    const HP: f32 = 400.0;
    let wall = |a: (f32, f32), b: (f32, f32)| Wall {
        a: (a.0.round(), a.1.round()),
        b: (b.0.round(), b.1.round()),
        thick: THICK,
        hp: HP,
    };
    for e in &edges {
        if e.kind != Border::Gate {
            continue;
        }
        let Some((c, rad)) = e.pass else { continue };
        let (sa, sb) = (sites[e.a], sites[e.b]);
        let t = unit(-(sb.1 - sa.1), sb.0 - sa.0);
        // Across the slot, its ends in the ridge's rock.
        let half = rad + 8.0;
        items.walls.push(wall(
            (c.0 - t.0 * half, c.1 - t.1 * half),
            (c.0 + t.0 * half, c.1 + t.1 * half),
        ));
    }
    // Turning a vector by fixed steps (constants, not `cos`, so every
    // machine turns it the same): 15 degrees.
    const C15: f32 = 0.965_925_8;
    const S15: f32 = 0.258_819_04;
    let turn = |v: (f32, f32), s: f32| (v.0 * C15 - v.1 * s, v.0 * s + v.1 * C15);
    let inside = |p: (f32, f32), m: f32| p.0 > m && p.1 > m && p.0 < wf - m && p.1 < hf - m;
    for (k, &q) in nests.iter().enumerate() {
        if !r.chance(rc.ring) {
            continue;
        }
        let rad = items.crystals[k].r + 55.0;
        let to = unit(landing.0 - q.0, landing.1 - q.1);
        let mut v = (1.0f32, 0.0f32);
        for _ in 0..24 {
            let u = turn(v, S15);
            let mid = unit(v.0 + u.0, v.1 + u.1);
            let (a, b) = ((q.0 + v.0 * rad, q.1 + v.1 * rad), (q.0 + u.0 * rad, q.1 + u.1 * rad));
            if mid.0 * to.0 + mid.1 * to.1 < 0.85 && inside(a, 8.0) && inside(b, 8.0) {
                items.walls.push(wall(a, b));
            }
            v = u;
        }
    }
    for _ in 0..rc.hedges {
        for _ in 0..60 {
            let mut at = (r.range(80.0, wf - 80.0), r.range(80.0, hf - 80.0));
            let mut dir = unit(r.range(-1.0, 1.0), r.range(-1.0, 1.0));
            let bend = if r.chance(0.5) { S15 } else { -S15 };
            let segs = 4 + r.below(4);
            let mut line = Vec::new();
            for _ in 0..segs {
                let b = (at.0 + dir.0 * 34.0, at.1 + dir.1 * 34.0);
                let blocked = |p: (f32, f32)| {
                    !inside(p, 24.0)
                        || dist(p, landing) < 160.0
                        || items.crystals.iter().any(|c| dist((c.x, c.y), p) < c.r + 34.0)
                        || ground[p.1 as usize * w + p.0 as usize] & G_ROCK != 0
                };
                if blocked(at) || blocked(b) {
                    break;
                }
                line.push(wall(at, b));
                at = b;
                dir = turn(dir, bend);
            }
            if line.len() >= 3 {
                items.walls.extend(line);
                break;
            }
        }
    }

    Map { ground, items }
}

// ---------------------------------------------------------------------
// The checks.

/// Map cells a walking bin.
const BIN: usize = 4;

/// What a squad can walk, in bins: no rock, no crystal, and (with `walls`)
/// no soft rock; then narrowed by one bin, so a pass a squad cannot get
/// through counts as shut.
pub fn walkable(m: &Map, size: (usize, usize), walls: bool) -> (usize, usize, Vec<bool>) {
    let (w, h) = size;
    let (bw, bh) = (w / BIN, h / BIN);
    let mut open = vec![true; bw * bh];
    for y in 0..h {
        for x in 0..w {
            if m.ground[y * w + x] & G_ROCK != 0 {
                open[(y / BIN) * bw + x / BIN] = false;
            }
        }
    }
    let shut = |open: &mut Vec<bool>, c: (f32, f32), rad: f32| {
        let (x0, x1) = (
            ((c.0 - rad) / BIN as f32).floor().max(0.0) as usize,
            (((c.0 + rad) / BIN as f32) as usize).min(bw - 1),
        );
        let (y0, y1) = (
            ((c.1 - rad) / BIN as f32).floor().max(0.0) as usize,
            (((c.1 + rad) / BIN as f32) as usize).min(bh - 1),
        );
        for by in y0..=y1 {
            for bx in x0..=x1 {
                let p = ((bx * BIN) as f32 + 2.0, (by * BIN) as f32 + 2.0);
                if dist(p, c) <= rad {
                    open[by * bw + bx] = false;
                }
            }
        }
    };
    for c in &m.items.crystals {
        shut(&mut open, (c.x, c.y), c.r);
    }
    if walls {
        for wl in &m.items.walls {
            let l = dist(wl.a, wl.b);
            let k = (l / 2.0).ceil().max(1.0) as usize;
            for i in 0..=k {
                let t = i as f32 / k as f32;
                shut(
                    &mut open,
                    (wl.a.0 + (wl.b.0 - wl.a.0) * t, wl.a.1 + (wl.b.1 - wl.a.1) * t),
                    wl.thick / 2.0 + 1.0,
                );
            }
        }
    }
    // Narrow by one bin.
    let mut out = open.clone();
    for by in 0..bh {
        for bx in 0..bw {
            if !open[by * bw + bx] {
                continue;
            }
            let edge = bx == 0 || by == 0 || bx + 1 == bw || by + 1 == bh;
            let near_shut = !edge
                && (!open[by * bw + bx - 1]
                    || !open[by * bw + bx + 1]
                    || !open[(by - 1) * bw + bx]
                    || !open[(by + 1) * bw + bx]);
            if near_shut {
                out[by * bw + bx] = false;
            }
        }
    }
    (bw, bh, out)
}

/// Walking distance in map cells from `from` to every bin (`u32::MAX` where
/// it cannot reach), 5 a side step and 7 a diagonal over `open`.
pub fn walk_from(bw: usize, bh: usize, open: &[bool], from: (f32, f32)) -> Vec<u32> {
    let mut dist = vec![u32::MAX; bw * bh];
    // Start from the nearest open bin, as a squad would step off.
    let (fx, fy) = ((from.0 / BIN as f32) as usize, (from.1 / BIN as f32) as usize);
    let mut start = None;
    'find: for r in 0..12usize {
        for by in fy.saturating_sub(r)..=(fy + r).min(bh - 1) {
            for bx in fx.saturating_sub(r)..=(fx + r).min(bw - 1) {
                if open[by * bw + bx] {
                    start = Some(by * bw + bx);
                    break 'find;
                }
            }
        }
    }
    let Some(s) = start else { return dist };
    use std::cmp::Reverse;
    let mut heap = std::collections::BinaryHeap::new();
    dist[s] = 0;
    heap.push(Reverse((0u32, s)));
    while let Some(Reverse((d, i))) = heap.pop() {
        if d > dist[i] {
            continue;
        }
        let (x, y) = ((i % bw) as i32, (i / bw) as i32);
        for (dx, dy, c) in [
            (1, 0, 5),
            (-1, 0, 5),
            (0, 1, 5),
            (0, -1, 5),
            (1, 1, 7),
            (1, -1, 7),
            (-1, 1, 7),
            (-1, -1, 7),
        ] {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= bw as i32 || ny >= bh as i32 {
                continue;
            }
            let j = ny as usize * bw + nx as usize;
            if !open[j]
                || (dx != 0
                    && dy != 0
                    && (!open[y as usize * bw + nx as usize] || !open[ny as usize * bw + x as usize]))
            {
                continue;
            }
            let nd = d + c;
            if nd < dist[j] {
                dist[j] = nd;
                heap.push(Reverse((nd, j)));
            }
        }
    }
    // Steps of 5 a bin to map cells.
    for d in dist.iter_mut() {
        if *d != u32::MAX {
            *d = *d * BIN as u32 / 5;
        }
    }
    dist
}

/// The nearest walking distance to anywhere on the rim of a disc.
fn reach(bw: usize, bh: usize, d: &[u32], c: (f32, f32), rad: f32) -> u32 {
    let m = rad + 2.0 * BIN as f32 + 2.0;
    let (x0, x1) = (
        ((c.0 - m) / BIN as f32).floor().max(0.0) as usize,
        (((c.0 + m) / BIN as f32) as usize).min(bw - 1),
    );
    let (y0, y1) = (
        ((c.1 - m) / BIN as f32).floor().max(0.0) as usize,
        (((c.1 + m) / BIN as f32) as usize).min(bh - 1),
    );
    let mut best = u32::MAX;
    for by in y0..=y1 {
        for bx in x0..=x1 {
            let p = ((bx * BIN) as f32 + 2.0, (by * BIN) as f32 + 2.0);
            if dist(p, c) <= m {
                best = best.min(d[by * bw + bx]);
            }
        }
    }
    best
}

/// What a planet measures, for the checks and the page.
#[derive(Clone, Debug, Default)]
pub struct Stats {
    pub crystals: usize,
    /// Sum of radius squared times charge: what the crystals hold, roughly.
    pub charge: f32,
    pub rock: f32,
    pub sand: f32,
    pub clay: f32,
    pub barren: f32,
    pub wet: f32,
    pub walls: usize,
    /// Walking distance from the landing to the nearest nest and to the
    /// first (the one across the map), soft rock broken (map cells), and to
    /// the nearest crystal.
    pub to_nest: u32,
    pub to_first: u32,
    pub to_crystal: u32,
    /// Crystals the squad cannot reach even through soft rock.
    pub unreachable: usize,
    /// Charge share reachable without breaking soft rock.
    pub open_share: f32,
    /// Straight-line distance landing to nearest nest.
    pub nest_line: f32,
}

pub fn stats(p: &Planet, m: &Map) -> Stats {
    let (w, h) = p.size;
    let n = (w * h) as f32;
    let count = |f: &dyn Fn(u8) -> bool| m.ground.iter().filter(|&&g| f(g)).count() as f32 / n;
    let landing = m.items.landing.unwrap_or((0.0, 0.0));
    let (bw, bh, soft) = walkable(m, p.size, false);
    let ds = walk_from(bw, bh, &soft, landing);
    let (_, _, hard) = walkable(m, p.size, true);
    let dh = walk_from(bw, bh, &hard, landing);
    let held = |c: &Crystal| c.r * c.r * c.charge;
    let total: f32 = m.items.crystals.iter().map(held).sum();
    let open: f32 = m
        .items
        .crystals
        .iter()
        .filter(|c| reach(bw, bh, &dh, (c.x, c.y), c.r) != u32::MAX)
        .map(held)
        .sum();
    Stats {
        crystals: m.items.crystals.len(),
        charge: total,
        rock: count(&|g| g & G_ROCK != 0),
        sand: count(&|g| g >> 4 == 1),
        clay: count(&|g| g >> 4 == 2),
        barren: count(&|g| g >> 4 == 3),
        wet: count(&|g| g & G_WET != 0),
        walls: m.items.walls.len(),
        to_nest: m
            .items
            .moulds
            .iter()
            .map(|&q| reach(bw, bh, &ds, q, 40.0))
            .min()
            .unwrap_or(u32::MAX),
        to_first: m
            .items
            .moulds
            .first()
            .map_or(u32::MAX, |&q| reach(bw, bh, &ds, q, 40.0)),
        to_crystal: m
            .items
            .crystals
            .iter()
            .map(|c| reach(bw, bh, &ds, (c.x, c.y), c.r))
            .min()
            .unwrap_or(u32::MAX),
        unreachable: m
            .items
            .crystals
            .iter()
            .filter(|c| reach(bw, bh, &ds, (c.x, c.y), c.r) == u32::MAX)
            .count(),
        open_share: if total > 0.0 { open / total } else { 0.0 },
        nest_line: m
            .items
            .moulds
            .iter()
            .map(|&q| dist(q, landing))
            .fold(f32::MAX, f32::min),
    }
}

/// What makes a built planet unfit, in words; empty when it is fit.
pub fn check(p: &Planet, m: &Map) -> Vec<String> {
    let mut out = Vec::new();
    let it = &m.items;
    if it.landing.is_none() || it.moulds.is_empty() || it.crystals.len() < 6 {
        out.push(format!(
            "missing items: {} crystals, {} nests",
            it.crystals.len(),
            it.moulds.len()
        ));
        return out;
    }
    let s = stats(p, m);
    let w = p.size.0 as f32;
    if s.unreachable > 0 {
        out.push(format!("{} crystals cannot be reached", s.unreachable));
    }
    if s.to_nest == u32::MAX {
        out.push("a nest cannot be reached".into());
    } else {
        if (s.to_first as f32) < 0.6 * w {
            out.push(format!("the far nest is {} cells' walk from the landing", s.to_first));
        }
        if (s.to_nest as f32) < 0.45 * w {
            out.push(format!("a nest is {} cells' walk from the landing", s.to_nest));
        }
    }
    if s.to_crystal > 200 {
        out.push(format!("nearest crystal {} cells' walk from the landing", s.to_crystal));
    }
    if s.open_share < 0.5 {
        out.push(format!(
            "only {:.0}% of the charge is open without breaking soft rock",
            s.open_share * 100.0
        ));
    }
    if s.rock > 0.3 {
        out.push(format!("{:.0}% rock", s.rock * 100.0));
    }
    if it.moulds.len() < Recipe::of(p).nests {
        out.push(format!("{} of {} nests placed", it.moulds.len(), Recipe::of(p).nests));
    }
    out
}

// ---------------------------------------------------------------------
// Names.

fn planet_name(r: &mut Rng) -> String {
    const START: [&str; 18] = [
        "ve", "or", "tha", "ka", "mu", "dre", "zo", "ya", "phe", "ul", "ne", "so", "gri", "a", "ix", "lo", "qua", "ber",
    ];
    const MID: [&str; 12] = ["sk", "r", "l", "n", "th", "v", "m", "rr", "z", "d", "ss", "k"];
    const END: [&str; 14] = [
        "a", "is", "on", "ix", "us", "e", "ar", "o", "en", "ith", "um", "or", "ae", "ek",
    ];
    let mut s = String::new();
    s += START[r.below(START.len())];
    s += MID[r.below(MID.len())];
    if r.chance(0.35) {
        s += ["a", "e", "i", "o", "u"][r.below(5)];
        s += MID[r.below(MID.len())];
    }
    s += END[r.below(END.len())];
    let mut c = s.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mould::big::BigDish;
    use crate::mould::larder_world::race_params;

    /// FNV-1a over the ground and the item text: a planet's fingerprint.
    fn print(m: &Map) -> u64 {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for b in m.ground.iter().copied().chain(m.items.text().into_bytes()) {
            h = (h ^ b as u64).wrapping_mul(0x100_0000_01b3);
        }
        h
    }

    #[test]
    fn the_same_seed_and_depth_give_the_same_planet() {
        assert_eq!(planet(7, 3), planet(7, 3));
        assert_eq!(print(&level(7, 3)), print(&level(7, 3)));
        assert_ne!(print(&level(7, 3)), print(&level(8, 3)));
        assert_ne!(print(&level(7, 3)), print(&level(7, 4)));
    }

    /// Pinned fingerprints: a planet a seed names today is the planet it
    /// names on every machine and every later build. Linux CI and the
    /// owner's Windows PC both run this, so a maths call that differs
    /// between them (`sin`, `exp`: HANDOFF decision 6) fails here.
    #[test]
    fn planets_are_pinned() {
        let got: Vec<u64> = (1..=8).map(|d| print(&level(1, d))).collect();
        assert_eq!(got, PINNED, "{got:#x?}");
    }

    const PINNED: [u64; 8] = [
        0x768679c392edeeb9,
        0x01f5bd35bd682552,
        0x72e0c58f347e1f48,
        0x8aee4093df4aa029,
        0xa1e95af23a78d5cd,
        0x85334efd43cf8aa3,
        0xde140c3db00cd880,
        0xa7d1eb358eee1e58,
    ];

    #[test]
    fn biomes_come_in_the_set_order_with_the_core_world_last() {
        let got: Vec<Biome> = (1..=9).map(|d| planet(1, d).biome).collect();
        assert_eq!(
            got,
            [
                Biome::Barrens,
                Biome::Marsh,
                Biome::Canyon,
                Biome::Dunes,
                Biome::Claypan,
                Biome::Thicket,
                Biome::Spires,
                Biome::Core,
                Biome::Barrens
            ]
        );
        for b in Biome::ALL {
            assert_eq!(Biome::from_name(b.name()), Some(b));
        }
    }

    /// Every planet over a spread of seeds and depths is built by a checked
    /// attempt (not the open fallback): every crystal and nest reachable,
    /// a crystal near the landing, the far nest a long walk off.
    #[test]
    fn every_planet_passes_its_checks() {
        let mut attempts = 0;
        for seed in 1..=6 {
            for depth in 1..=9 {
                let p = planet(seed, depth);
                let k = (0..ATTEMPTS).find(|&a| check(&p, &build(&p, a)).is_empty());
                let k = k.unwrap_or_else(|| {
                    panic!(
                        "seed {seed} depth {depth} ({}): {:?}",
                        p.biome.name(),
                        check(&p, &build(&p, 0))
                    )
                });
                attempts += k + 1;
                let m = generate(&p);
                assert_eq!(print(&m), print(&build(&p, k)));
                let s = stats(&p, &m);
                assert!(
                    s.crystals >= 12 && s.unreachable == 0,
                    "seed {seed} depth {depth}: {s:?}"
                );
            }
        }
        // Most planets pass on their first or second attempt.
        assert!(attempts <= 54 * 3, "{attempts} attempts for 54 planets");
    }

    #[test]
    fn nests_sit_on_their_crystals_and_the_landing_is_clear() {
        for seed in 1..=4 {
            for depth in [1, 4, 7, 8, 9] {
                let p = planet(seed, depth);
                let m = generate(&p);
                let it = &m.items;
                assert_eq!(it.moulds.len(), Recipe::of(&p).nests, "seed {seed} depth {depth}");
                for (k, &q) in it.moulds.iter().enumerate() {
                    let c = it.crystals[k];
                    assert_eq!((c.x, c.y), q);
                    assert!(c.r >= 33.0 && c.charge >= 384.0);
                }
                let l = it.landing.unwrap();
                let (w, _) = p.size;
                assert_eq!(m.ground[l.1 as usize * w + l.0 as usize] & (G_ROCK | G_WET), 0);
                assert!(it.crystals.iter().all(|c| dist((c.x, c.y), l) > c.r + 40.0));
                assert!(it
                    .walls
                    .iter()
                    .all(|wl| crate::mould::map_edit::segment_distance(l, wl.a, wl.b) > 60.0));
                // What the text says is the map: whole numbers throughout.
                assert_eq!(Items::parse(&it.text()).unwrap(), *it);
            }
        }
    }

    #[test]
    fn deeper_planets_have_more_nests_and_narrower_passes() {
        let a = Recipe::of(&planet(1, 1));
        let b = Recipe::of(&planet(1, 9));
        assert!(b.nests > a.nests && b.pass.0 < a.pass.0);
        assert_eq!(Recipe::of(&planet(1, CORE_DEPTH)).nests, 3, "the core world");
    }

    /// A planet builds the world as a designed map does: its crystals,
    /// its walls, the slime on its nests.
    #[test]
    fn a_planet_makes_its_world() {
        let m = level(3, 6);
        let d = BigDish::with_map(race_params(3, 2, 3000), &m);
        assert_eq!(d.piles.len(), m.items.crystals.len());
        assert_eq!(d.homes.len(), m.items.moulds.len());
        assert!(
            !m.items.walls.is_empty() && d.wall.iter().any(|&w| w),
            "the thicket's soft rock stands"
        );
        assert_eq!(d.alive.iter().filter(|&&a| a).count(), 3000);
    }

    #[test]
    fn map_specs_name_planets() {
        assert!(spec("fork", 1).is_none());
        assert!(spec("planetary", 1).is_none());
        let (p, m) = spec("planet", 5).unwrap().unwrap();
        assert_eq!((p.seed, p.depth), (5, 1));
        assert_eq!(print(&m), print(&level(5, 1)));
        let (p, _) = spec("planet:4", 5).unwrap().unwrap();
        assert_eq!((p.seed, p.depth), (5, 4));
        let (p, _) = spec("planet:4:9", 5).unwrap().unwrap();
        assert_eq!((p.seed, p.depth), (9, 4));
        assert!(spec("planet:0", 1).unwrap().is_err());
        assert!(spec("planet:x", 1).unwrap().is_err());
        assert!(spec("planet:1:2:3", 1).unwrap().is_err());
        assert_eq!(spec_planet("planet:3:9", 1).unwrap().unwrap(), planet(9, 3));
    }

    #[test]
    fn names_read_as_names() {
        let names: Vec<String> = (1..=40).map(|s| planet(s, 1).name).collect();
        assert!(
            names
                .iter()
                .all(|n| (3..=12).contains(&n.len()) && n.chars().next().unwrap().is_uppercase()),
            "{names:?}"
        );
        let distinct: std::collections::BTreeSet<&String> = names.iter().collect();
        assert!(distinct.len() >= 36, "{} distinct of 40", distinct.len());
    }
}
