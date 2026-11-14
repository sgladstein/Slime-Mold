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

use super::map::{standable, Crystal, Items, Map, Wall, G_BRIDGE, G_DEEP, G_ROCK, G_WET, MAP_H, MAP_W};

/// Soil kinds in the G byte's high nibble (`big::SOIL_*` << 4).
pub const G_SAND: u8 = 0x10;
pub const G_CLAY: u8 = 0x20;
pub const G_BARREN: u8 = 0x30;

/// Deep water, the ground thread's (soil kind 4 with the wet bit,
/// `map::G_DEEP`): soldiers cannot stand in it, shots fly over, the slime
/// crawls through. The rivers' fords and bridges and the lakes' rims are
/// shallow or dry, and the walk checks (`walkable`, `map::standable`) see
/// it, so a planet that water cuts off is rebuilt.
const DEEP: u8 = G_DEEP;

/// How many builds `generate` tries before it falls back to an open plan.
pub const ATTEMPTS: u32 = 24;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Biome {
    /// Open ground, ponds, meadows: the first planet, readable from the
    /// landing. Every build has room.
    Meadow,
    /// Ground that starves the slime: barren bands across the route, or
    /// barren everywhere but its oases.
    Barrens,
    /// Pools and lakes on clay banks: wet ground slows the slime.
    Fen,
    /// One or two rivers right across the route, with fords; sand bars,
    /// terraces of rock.
    Valley,
    /// Streams, soft-rock hedges and rings, clay: the slime's cover.
    Jungle,
    /// Rocky rooms joined by passes, cracked clay floors: room by room.
    Highlands,
    /// Mostly rock: caverns joined by tunnels.
    Caves,
    /// The core world: any of it, several nests.
    Core,
}

impl Biome {
    pub const ALL: [Biome; 8] = [Biome::Meadow, Biome::Barrens, Biome::Fen, Biome::Valley, Biome::Jungle, Biome::Highlands, Biome::Caves, Biome::Core];

    /// As the shop's card says it ("a river valley planet").
    pub fn name(self) -> &'static str {
        match self {
            Biome::Meadow => "meadow",
            Biome::Barrens => "barrens",
            Biome::Fen => "fen",
            Biome::Valley => "river valley",
            Biome::Jungle => "jungle",
            Biome::Highlands => "highlands",
            Biome::Caves => "cave",
            Biome::Core => "core",
        }
    }

    pub fn from_name(s: &str) -> Option<Biome> {
        Biome::ALL.into_iter().find(|b| b.name() == s)
    }

    /// The set order (owner, 2026-11-14 15:33Z "Set order"; 17:02Z, open
    /// and readable early, mazes and caves later, "That makes sense"), the
    /// core world last. Round 2's biomes (barrens, marsh, canyon, dunes,
    /// claypan, thicket, spires) are dials of these now: their shapes are
    /// kept, as `Openness`, `Water` and `Soil` values.
    pub const ORDER: [Biome; 7] = [Biome::Meadow, Biome::Barrens, Biome::Fen, Biome::Valley, Biome::Jungle, Biome::Highlands, Biome::Caves];

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
    /// Map cells (`size_at`): 60% of the designed maps' area on planet 1,
    /// six times it on the core world.
    pub size: (usize, usize),
}

/// Each planet's map area as a share of the designed maps' (1024 x 640):
/// the owner's "first planet about 60% of today's map, today's size by
/// planet 3 or 4, up to 6 the area for the core world" (2026-11-14 15:33Z).
/// Past the core world, the core world's. `GROW_CAP` holds it at twice
/// today's for now.
pub const AREA: [f32; 8] = [0.6, 0.8, 1.0, 1.4, 2.0, 2.8, 4.0, 6.0];

/// The biggest share `size_at` gives. The technical lead (2026-11-14
/// 16:42Z): "Before the default changes, book one 1070 timing ... at 1x,
/// 2x, 4x and 6x area". Timed 19:11Z (`raw/sizes-1070/`): the GPU step
/// 0.63, 0.97, 1.50 and 2.11 ms and frame p95 at most 5.2 ms at every
/// size, but the window's ground refresh (`refresh_ground`, once a game
/// second: a charge read-back and every cell's code on the CPU) peaks at
/// 4.6, 8.4, 23.6 and 32.9 ms, a dropped frame or two every second from
/// 4x. So 2x (planets 5-8) until that refresh touches only the cells that
/// change. `planet:D:S:AREA` asks for any share up to `AREA_MAX`.
pub const GROW_CAP: f32 = 2.0;

/// The biggest share a `planet:D:S:AREA` spec may ask for.
pub const AREA_MAX: f32 = 6.0;

/// The map size at `depth`, map cells: `AREA`'s share of 1024 x 640 (at
/// most `GROW_CAP`), the same shape.
pub fn size_at(depth: u32) -> (usize, usize) {
    size_of(AREA[(depth.clamp(1, AREA.len() as u32) - 1) as usize].min(GROW_CAP))
}

/// A map `area` times 1024 x 640's area, the same shape, each side a
/// multiple of 16 map cells (4 dish cells).
pub fn size_of(area: f32) -> (usize, usize) {
    let k = area.sqrt();
    let side = |v: usize| ((v as f32 * k / 16.0 + 0.5).floor() as usize).max(4) * 16;
    (side(MAP_W), side(MAP_H))
}

/// The planet at `depth` (from 1) of the run with `seed`.
pub fn planet(seed: u64, depth: u32) -> Planet {
    let mut r = Rng::new(seed, depth, STREAM_PLANET);
    let biome = Biome::at(depth);
    let name = planet_name(&mut r);
    Planet { seed, depth, name, biome, size: size_at(depth) }
}

/// The map of the planet at `depth` of the run with `seed`.
pub fn level(seed: u64, depth: u32) -> Map {
    generate(&planet(seed, depth))
}

/// A `map=` value naming a planet: `planet` (depth 1), `planet:DEPTH`,
/// `planet:DEPTH:SEED` or `planet:DEPTH:SEED:AREA`, the seed otherwise
/// `seed` (the window's `seed=`) and the area (a share of today's map, to
/// `AREA_MAX`) otherwise the depth's. `None` when `s` names no planet.
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
        if words.len() > 3 {
            return Err(format!("{s}: planet, planet:DEPTH, planet:DEPTH:SEED or planet:DEPTH:SEED:AREA"));
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
        let p = planet(seed, depth);
        match words.get(2) {
            Some(v) => {
                let area = v.parse::<f32>().map_err(|e| format!("{s}: area {v:?}: {e}"))?;
                if !(0.25..=AREA_MAX).contains(&area) {
                    return Err(format!("{s}: area {area} (a share of today's map) runs from 0.25 to {AREA_MAX}"));
                }
                Ok(Planet { size: size_of(area), ..p })
            }
            None => Ok(p),
        }
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
    build_with(p, ATTEMPTS, Some(Recipe { open: 1.0, gate: 0.0, outcrop: 2.0, ..Recipe::of(p) }))
}

// ---------------------------------------------------------------------
// Randomness: SplitMix64, keyed by seed, depth and a stream.

const STREAM_PLANET: u64 = 1;
const STREAM_BUILD: u64 = 2;
const STREAM_NOISE: u64 = 3;
/// The planet's dials (`Recipe::of`): rolled once a planet, not an attempt,
/// so a failed attempt redraws the layout, not the kind of planet.
const STREAM_DIALS: u64 = 4;

#[derive(Clone)]
pub struct Rng(u64);

fn mix(mut z: u64) -> u64 {
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

impl Rng {
    pub fn new(seed: u64, depth: u32, stream: u64) -> Rng {
        Rng(mix(mix(seed ^ 0x5EED_1E7E_15CA_FE00) ^ mix((depth as u64) << 20 | stream)))
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
        (self.at(x, y, cell) * 4.0 + self.at(x + 31.7, y - 12.3, cell / 2.0) * 2.0 + self.at(x - 7.1, y + 55.9, cell / 4.0)) / 7.0
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

    /// The least and greatest value over cells `x0..x1` x `y0..y1`: the
    /// samples `at` reads there bound it.
    fn range(&self, x0: usize, y0: usize, x1: usize, y1: usize) -> (f32, f32) {
        let (mut lo, mut hi) = (f32::MAX, f32::MIN);
        for gy in y0 / STEP..=(y1 - 1) / STEP + 1 {
            for gx in x0 / STEP..=(x1 - 1) / STEP + 1 {
                let v = self.v[gy * self.gw + gx];
                (lo, hi) = (lo.min(v), hi.max(v));
            }
        }
        (lo, hi)
    }

    fn at(&self, x: usize, y: usize) -> f32 {
        let (gx, gy, tx, ty) = (x / STEP, y / STEP, (x % STEP) as f32 / STEP as f32, (y % STEP) as f32 / STEP as f32);
        let i = gy * self.gw + gx;
        let a = self.v[i] + (self.v[i + 1] - self.v[i]) * tx;
        let b = self.v[i + self.gw] + (self.v[i + self.gw + 1] - self.v[i + self.gw]) * tx;
        a + (b - a) * ty
    }
}

// ---------------------------------------------------------------------
// The dials and the recipe: a biome is a named range on each dial, and a
// planet rolls its value on each (the owner, 2026-11-14 17:02Z: "Lets go
// with your rec", the plan's dials plus named biomes;
// Reports/levelgen-2026-11-14/biomes-plan.md).

/// How much rock divides the ground.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Openness {
    /// Few ridges, no mesas.
    Open,
    /// Ridges at the region borders, mesas in the fields.
    Outcrops,
    /// Many small rock pillars (round 2's Spires).
    Pillars,
    /// Long ridges right across the route with a pass or two (round 2's
    /// Canyon).
    Ridges,
    /// Nearly every border a ridge: rooms joined by passes.
    Rooms,
    /// Rock everywhere but caverns round the sites and tunnels between.
    Caves,
}

/// Water on the ground (wet: the slime's step halves).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Water {
    Dry,
    /// Big pools that run together (round 2's Marsh).
    Pools,
    /// Lakes: broad, with a deep middle (deep water once the ground
    /// thread's kind 4 lands; wet until then).
    Lakes,
    /// One or two rivers right across the route, deep in the middle with
    /// shallow fords.
    Rivers,
    /// Narrow streams that wander, shallow all through.
    Streams,
}

/// The soil the slime plays on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Soil {
    Plain,
    /// Sand in broad patches.
    Sand,
    /// Sand nearly everywhere, clay islands under the crystals (round 2's
    /// Dunes).
    Dunes,
    /// Barren bands right across the route (round 2's Barrens).
    Bands,
    /// Barren everywhere but the oases, where the crystals are.
    Oases,
    /// Clay in broad patches.
    Clay,
    /// Clay nearly everywhere, cracked into polygons (round 2's Claypan).
    Cracks,
    /// A bit of everything (the core world).
    Mixed,
}

/// How the crystals lie (the owner, 17:02Z: the pattern "rolls its own,
/// but some biomes are more likely to get certain patterns"). Every pattern
/// keeps a home crystal under each nest and one near the landing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pattern {
    /// Big ones midway, small ones anywhere (rounds 1 and 2).
    Scattered,
    /// A few fields: a big crystal ringed by small ones.
    Fields,
    /// Lines of small crystals across the route, like seams.
    Veins,
    /// One giant crystal midway, a few small ones.
    Jackpot,
    /// Clusters at the oases, nothing between.
    Oases,
    /// Small near the landing, bigger and richer toward the slime.
    Richer,
}

impl Pattern {
    pub fn name(self) -> &'static str {
        match self {
            Pattern::Scattered => "scattered",
            Pattern::Fields => "fields",
            Pattern::Veins => "veins",
            Pattern::Jackpot => "jackpot",
            Pattern::Oases => "oases",
            Pattern::Richer => "richer farther",
        }
    }
}

/// A biome's odds on each dial: openness, water, soil, crystal pattern.
type Odds = (&'static [(Openness, u32)], &'static [(Water, u32)], &'static [(Soil, u32)], &'static [(Pattern, u32)]);

/// One of `odds` by its weight.
fn pick<T: Copy>(r: &mut Rng, odds: &[(T, u32)]) -> T {
    let total: u32 = odds.iter().map(|o| o.1).sum();
    let mut k = (r.draw() % total.max(1) as u64) as u32;
    for &(v, w) in odds {
        if k < w {
            return v;
        }
        k -= w;
    }
    odds[0].0
}

#[derive(Clone, Copy, Debug)]
pub struct Recipe {
    pub openness: Openness,
    pub water: Water,
    pub soil: Soil,
    pub pattern: Pattern,
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
    /// How much the noise varies a ridge's width: its half-width runs from
    /// `ridge * (1 - vary / 2)` to `ridge * (1 + vary / 2)`. Low keeps a
    /// ridge whole; high breaks it into stubs.
    pub ridge_vary: f32,
    /// Long ridges right across the map, across the route, each with one or
    /// two passes (`Openness::Ridges`): 0 for none.
    pub fronts: usize,
    /// Pass radius range, map cells.
    pub pass: (f32, f32),
    /// Warp of the region borders, map cells.
    pub warp: f32,
    /// Outcrop (mesa) threshold on noise: higher is fewer, 2 none.
    pub outcrop: f32,
    /// Small rock pillars, for a 1024 x 640 map (scaled by area).
    pub pillars: usize,
    /// Slime nests (mould starts).
    pub nests: usize,
    /// Small crystals (the pattern places them).
    pub small: usize,
    /// Big crystals (the pattern places them).
    pub big: usize,
    /// Chance a nest gets a soft-rock ring, with one gap facing the landing.
    pub ring: f32,
    /// Soft-rock hedges (curved wall lines).
    pub hedges: usize,
    /// Rivers or streams, for `Water::Rivers` and `Water::Streams`.
    pub rivers: usize,
}

impl Recipe {
    pub fn of(p: &Planet) -> Recipe {
        let mut r = Rng::new(p.seed, p.depth, STREAM_DIALS);
        let d = p.depth.max(1) as f32;
        // Nests: one, then two from depth 4, three from 8.
        let nests = 1 + (p.depth >= 4) as usize + (p.depth >= 8) as usize;
        use Openness as O;
        use Pattern as P;
        use Soil as S;
        use Water as W;
        // Each biome's odds on each dial.
        let (open, water, soil, pattern): Odds = match p.biome {
            Biome::Meadow => (&[(O::Open, 3), (O::Outcrops, 2), (O::Pillars, 1)], &[(W::Pools, 2), (W::Dry, 1), (W::Streams, 1)], &[(S::Plain, 2), (S::Sand, 1), (S::Clay, 1)], &[(P::Scattered, 3), (P::Fields, 3), (P::Richer, 2), (P::Veins, 1), (P::Jackpot, 1)]),
            Biome::Barrens => (&[(O::Open, 2), (O::Outcrops, 2), (O::Pillars, 1)], &[(W::Dry, 1)], &[(S::Bands, 1), (S::Oases, 1)], &[(P::Scattered, 2), (P::Veins, 2), (P::Richer, 2), (P::Jackpot, 1)]),
            Biome::Fen => (&[(O::Open, 2), (O::Outcrops, 2)], &[(W::Lakes, 2), (W::Pools, 2)], &[(S::Clay, 2), (S::Plain, 1)], &[(P::Veins, 3), (P::Fields, 2), (P::Scattered, 2), (P::Richer, 1)]),
            Biome::Valley => (&[(O::Ridges, 2), (O::Outcrops, 1)], &[(W::Rivers, 1)], &[(S::Dunes, 2), (S::Sand, 1)], &[(P::Fields, 3), (P::Veins, 2), (P::Richer, 2), (P::Scattered, 1)]),
            Biome::Jungle => (&[(O::Outcrops, 2), (O::Open, 1)], &[(W::Streams, 2), (W::Pools, 1)], &[(S::Clay, 2), (S::Plain, 1)], &[(P::Scattered, 3), (P::Fields, 2), (P::Jackpot, 1), (P::Richer, 1)]),
            Biome::Highlands => (&[(O::Rooms, 3), (O::Ridges, 1)], &[(W::Dry, 2), (W::Pools, 1)], &[(S::Cracks, 2), (S::Clay, 1)], &[(P::Fields, 3), (P::Jackpot, 2), (P::Richer, 2), (P::Scattered, 1)]),
            Biome::Caves => (&[(O::Caves, 1)], &[(W::Pools, 2), (W::Dry, 1)], &[(S::Plain, 1), (S::Clay, 1), (S::Sand, 1)], &[(P::Veins, 3), (P::Jackpot, 2), (P::Scattered, 2), (P::Fields, 1)]),
            Biome::Core => (&[(O::Outcrops, 1), (O::Pillars, 1), (O::Ridges, 1), (O::Rooms, 1)], &[(W::Pools, 1), (W::Lakes, 1), (W::Rivers, 1), (W::Streams, 1), (W::Dry, 1)], &[(S::Mixed, 1)], &[(P::Scattered, 1), (P::Fields, 1), (P::Veins, 1), (P::Jackpot, 1), (P::Richer, 1)]),
        };
        let openness = pick(&mut r, open);
        let water = pick(&mut r, water);
        let soil = pick(&mut r, soil);
        // Oases soil keeps its crystals in the oases.
        let pattern = if soil == S::Oases { P::Oases } else { pick(&mut r, pattern) };
        let base = Recipe {
            openness,
            water,
            soil,
            pattern,
            regions: 11,
            open: 0.7,
            front: 0.6,
            loops: 0.4,
            gate: 0.15,
            ridge: 13.0,
            ridge_vary: 1.3,
            fronts: 0,
            pass: (24.0, 36.0),
            warp: 60.0,
            outcrop: 0.7,
            pillars: 0,
            nests,
            small: 12,
            big: 2,
            ring: 0.0,
            hedges: 0,
            rivers: 0,
        };
        // Openness: the rock.
        let rc = match openness {
            O::Open => Recipe { open: 0.9, front: 0.25, ridge: 11.0, outcrop: 2.0, ..base },
            O::Outcrops => Recipe { front: 0.45, outcrop: 0.72, ..base },
            O::Pillars => Recipe { open: 0.85, front: 0.3, ridge: 11.0, outcrop: 0.78, pillars: 210, ..base },
            // Two long ridges across the route (three past the first round),
            // whole and wide, between fewer, bigger regions; no mesas, which
            // read as debris (graphics lead, 2026-11-14: round 1's canyon
            // rock was "short stubs").
            O::Ridges => Recipe { regions: 9, open: 0.55, front: 0.0, loops: 0.35, gate: 0.25, ridge: 18.0, ridge_vary: 0.5, fronts: 2 + (p.depth > CORE_DEPTH) as usize, pass: (22.0, 32.0), outcrop: 2.0, ..base },
            // Walls whole and wide, few loops, small corners shut: rooms.
            O::Rooms => Recipe { regions: 12, open: 0.08, front: 0.95, loops: 0.2, gate: 0.08, ridge: 21.0, ridge_vary: 0.4, pass: (20.0, 28.0), outcrop: 2.0, ..base },
            // Every border shut; the spanning tree and the loops are tunnels.
            O::Caves => Recipe { regions: 13, open: 0.0, front: 1.0, loops: 0.3, gate: 0.2, outcrop: 2.0, ..base },
        };
        let rc = match water {
            W::Rivers => Recipe { rivers: 1 + r.chance(0.4) as usize, ..rc },
            W::Streams => Recipe { rivers: 2 + r.below(3), ..rc },
            _ => rc,
        };
        // The biome's own extras.
        let rc = match p.biome {
            Biome::Jungle => Recipe { gate: 0.6, ring: 1.0, hedges: 9, ..rc },
            Biome::Meadow => Recipe { hedges: r.below(3), ..rc },
            // Fewer gates in rooms, where every pass is a tree's: a gate
            // there shuts all the rooms behind it (rooms with gates at
            // 0.35 left under half the charge open on 8 of 50 core worlds).
            Biome::Core => Recipe { regions: rc.regions + 4, gate: if openness == O::Rooms { 0.1 } else { 0.35 }, ring: if openness == O::Rooms { 0.0 } else { 0.6 }, hedges: 5, nests: nests.max(3), small: 14, big: 3, ..rc },
            _ => rc,
        };
        // Passes narrow with depth, to a floor a squad still walks through.
        let narrow = (1.0 - 0.03 * (d - 1.0)).max(0.7);
        // More of everything on a bigger map, at the same density: regions
        // (so the same spacing), crystals, hedges, pillars (`pillars` is
        // per 1024 x 640 already). Big crystals grow with the route's
        // length, not the area.
        let area = (p.size.0 * p.size.1) as f32 / (MAP_W * MAP_H) as f32;
        let more = |n: usize, k: f32| (n as f32 * k + 0.5).floor() as usize;
        Recipe { pass: (rc.pass.0 * narrow, rc.pass.1 * narrow), regions: more(rc.regions, area), small: more(rc.small, area), big: more(rc.big, area.sqrt()), hedges: more(rc.hedges, area), rivers: more(rc.rivers, area.sqrt()), ..rc }
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
    /// On the spanning tree (always passable).
    tree: bool,
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

/// `v` turned by the angle whose tangent is `k` (a square root, no `cos`).
fn bend(v: (f32, f32), k: f32) -> (f32, f32) {
    unit(v.0 - k * v.1, v.1 + k * v.0)
}

/// Soft rock's health a cell, as the designed maps' walls.
const HP: f32 = 400.0;

/// Soft rock along `pts`, each piece its own thickness.
fn hedge(r: &mut Rng, walls: &mut Vec<Wall>, pts: &[(f32, f32)]) {
    for s in pts.windows(2) {
        let thick = r.range(11.0, 21.0).round();
        walls.push(Wall { a: (s[0].0.round(), s[0].1.round()), b: (s[1].0.round(), s[1].1.round()), thick, hp: HP });
    }
}

/// `a` to `b` in `k` pieces, the inner points pushed sideways by a smooth
/// wander of up to about `wobble` map cells.
fn wobbly(r: &mut Rng, a: (f32, f32), b: (f32, f32), k: usize, wobble: f32) -> Vec<(f32, f32)> {
    let nrm = unit(-(b.1 - a.1), b.0 - a.0);
    let mut off = 0.0;
    (0..=k)
        .map(|i| {
            let t = i as f32 / k as f32;
            off = if i == 0 || i == k { 0.0 } else { off * 0.5 + r.range(-wobble, wobble) };
            (a.0 + (b.0 - a.0) * t + nrm.0 * off, a.1 + (b.1 - a.1) * t + nrm.1 * off)
        })
        .collect()
}

/// Turning a vector by fixed steps (constants, not `cos`, so every machine
/// turns it the same): 15 degrees.
const C15: f32 = 0.965_925_8;
const S15: f32 = 0.258_819_04;

fn turn15(v: (f32, f32)) -> (f32, f32) {
    (v.0 * C15 - v.1 * S15, v.0 * S15 + v.1 * C15)
}

/// Distance from `q` to the segment `a`-`b`.
fn seg_dist(q: (f32, f32), a: (f32, f32), b: (f32, f32)) -> f32 {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let l2 = (dx * dx + dy * dy).max(1e-6);
    let t = (((q.0 - a.0) * dx + (q.1 - a.1) * dy) / l2).clamp(0.0, 1.0);
    dist(q, (a.0 + dx * t, a.1 + dy * t))
}

/// Calls `f(i, at, d)` for each cell within `reach` of the polyline `pts`,
/// with `d` its distance to the line.
fn along_line(w: usize, h: usize, pts: &[(f32, f32)], reach: f32, mut f: impl FnMut(usize, (f32, f32), f32)) {
    if pts.len() < 2 {
        return;
    }
    let (mut x0, mut y0, mut x1, mut y1) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for q in pts {
        (x0, y0, x1, y1) = (x0.min(q.0), y0.min(q.1), x1.max(q.0), y1.max(q.1));
    }
    let (xa, xb) = ((x0 - reach).max(0.0) as usize, ((x1 + reach).max(0.0) as usize + 1).min(w));
    let (ya, yb) = ((y0 - reach).max(0.0) as usize, ((y1 + reach).max(0.0) as usize + 1).min(h));
    for y in ya..yb {
        for x in xa..xb {
            let at = (x as f32 + 0.5, y as f32 + 0.5);
            let d = pts.windows(2).map(|s| seg_dist(at, s[0], s[1])).fold(f32::MAX, f32::min);
            if d < reach {
                f(y * w + x, at, d);
            }
        }
    }
}

/// A line that wanders from `from` along `dir` (bending by up to `k` a
/// piece) until it leaves the map: pieces `piece` map cells long.
fn wander(r: &mut Rng, w: f32, h: f32, from: (f32, f32), dir: (f32, f32), piece: f32, k: f32) -> Vec<(f32, f32)> {
    let (mut at, mut dir, mut pts) = (from, dir, vec![from]);
    let mut bendk = 0.0;
    for _ in 0..400 {
        at = (at.0 + dir.0 * piece, at.1 + dir.1 * piece);
        pts.push(at);
        if at.0 < -piece || at.1 < -piece || at.0 > w + piece || at.1 > h + piece {
            break;
        }
        bendk = bendk * 0.6 + r.range(-k, k);
        dir = bend(dir, bendk);
    }
    pts
}

fn build_with(p: &Planet, attempt: u32, recipe: Option<Recipe>) -> Map {
    let rc = recipe.unwrap_or_else(|| Recipe::of(p));
    let (w, h) = p.size;
    let (wf, hf) = (w as f32, h as f32);
    let mut r = Rng::new(p.seed ^ mix(attempt as u64 + 1), p.depth, STREAM_BUILD);
    let noise = |k: u64| Noise { key: mix(p.seed ^ mix(k << 8 | attempt as u64) ^ (p.depth as u64) << 40) ^ STREAM_NOISE };

    // The landing: near a short edge mostly, so the slime is a long way
    // off; near a long edge's end otherwise, so it is across a diagonal.
    let inset = r.range(90.0, 130.0);
    let landing = {
        let k = r.f();
        let along = r.range(0.25, 0.75);
        let end = if r.chance(0.5) { r.range(0.12, 0.3) } else { r.range(0.7, 0.88) };
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
    // The Marsh's pools: one broad noise, so they are big and run together.
    let pools = Field::new(w, h, |x, y| nw.fbm(x + 1733.0, y - 419.0, 210.0));

    // Warped Voronoi: each cell's region, the runner-up, and its distance
    // to their border.
    let n = w * h;
    let mut region = vec![0u16; n];
    let mut second = vec![0u16; n];
    let mut border = vec![0f32; n];
    // Tile by tile, only the sites that can be one of a cell's nearest two:
    // the warped points of a tile lie in a box (the warp is bilinear, so its
    // samples bound it; padded a cell for rounding), and a site whose
    // nearest approach to the box is farther than the second-nearest
    // site's farthest cannot be either. The candidates keep their order, so
    // the result is the full scan's to the bit, ties included; on the core
    // world (90 sites, 3.9 million cells) the full scan was most of a
    // second.
    const TILE: usize = 16;
    let mut cand: Vec<usize> = Vec::with_capacity(ns);
    let mut far: Vec<f32> = Vec::with_capacity(ns);
    for ty in (0..h).step_by(TILE) {
        for tx in (0..w).step_by(TILE) {
            let (x1, y1) = ((tx + TILE).min(w), (ty + TILE).min(h));
            let (wx0, wx1) = warp_x.range(tx, ty, x1, y1);
            let (wy0, wy1) = warp_y.range(tx, ty, x1, y1);
            let bx = (tx as f32 + 0.5 + wx0 - 1.0, (x1 - 1) as f32 + 0.5 + wx1 + 1.0);
            let by = (ty as f32 + 0.5 + wy0 - 1.0, (y1 - 1) as f32 + 0.5 + wy1 + 1.0);
            far.clear();
            far.extend(sites.iter().map(|s| {
                let dx = (s.0 - bx.0).abs().max((s.0 - bx.1).abs());
                let dy = (s.1 - by.0).abs().max((s.1 - by.1).abs());
                dx * dx + dy * dy
            }));
            let (mut f1, mut f2) = (f32::MAX, f32::MAX);
            for &f in &far {
                if f < f1 {
                    (f2, f1) = (f1, f);
                } else if f < f2 {
                    f2 = f;
                }
            }
            cand.clear();
            cand.extend((0..ns).filter(|&k| {
                let s = sites[k];
                let dx = (bx.0 - s.0).max(0.0).max(s.0 - bx.1);
                let dy = (by.0 - s.1).max(0.0).max(s.1 - by.1);
                dx * dx + dy * dy <= f2
            }));
            for y in ty..y1 {
                for x in tx..x1 {
                    let px = x as f32 + 0.5 + warp_x.at(x, y);
                    let py = y as f32 + 0.5 + warp_y.at(x, y);
                    let (mut d1, mut d2, mut i1, mut i2) = (f32::MAX, f32::MAX, 0usize, 0usize);
                    for &k in &cand {
                        let s = sites[k];
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
        }
    }

    // Borders between regions, with their cells.
    let mut edge_of = vec![usize::MAX; ns * ns];
    let mut edges: Vec<Edge> = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let a = region[i] as usize;
            for j in [if x + 1 < w { Some(i + 1) } else { None }, if y + 1 < h { Some(i + w) } else { None }].into_iter().flatten() {
                let b = region[j] as usize;
                if a != b {
                    let (lo, hi) = (a.min(b), a.max(b));
                    if edge_of[lo * ns + hi] == usize::MAX {
                        edges.push(Edge { a: lo, b: hi, kind: Border::Open, tree: false, cells: Vec::new(), pass: None });
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
        edges[e].tree = tree;
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

    // Each pass: a border cell mid-border, away from the map's edge. An
    // open border on the spanning tree gets one too, cut below like a
    // pass's: a short open border (a corner where three regions meet) was
    // shut by its neighbours' ridges, which walled the landing in on rocky
    // rooms (2026-11-14).
    for e in edges.iter_mut() {
        if !matches!(e.kind, Border::Pass | Border::Gate) && !(e.tree && e.kind == Border::Open) {
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
    let widest = rc.ridge * (1.0 + rc.ridge_vary / 2.0) + 6.0;

    // Fronts (the Canyon's): ridges right across the map, across the route,
    // at even steps between the landing and the far nest, each with one or
    // two passes. A front is a band in the route's coordinate (warped, so
    // it meanders); a pass is a slot across it in the other coordinate.
    let perp = (-axis.1, axis.0);
    let along = |q: (f32, f32)| (q.0 - landing.0) * axis.0 + (q.1 - landing.1) * axis.1;
    let across = |q: (f32, f32)| (q.0 - landing.0) * perp.0 + (q.1 - landing.1) * perp.1;
    let at_line = |s: f32, u: f32| (landing.0 + axis.0 * s + perp.0 * u, landing.1 + axis.1 * s + perp.1 * u);
    let span = along(first);
    struct Front {
        s: f32,
        half: f32,
        passes: Vec<(f32, f32)>,
        gate: Option<usize>,
    }
    let mut fronts: Vec<Front> = Vec::new();
    for k in 0..rc.fronts {
        let s = span * ((k + 1) as f32 / (rc.fronts + 1) as f32 + r.range(-0.04, 0.04));
        let half = rc.ridge * r.range(1.6, 1.9);
        // Where on the line a pass can go: inside the map, clear of its rim.
        let rad = r.range(rc.pass.0, rc.pass.1);
        let room: Vec<f32> = (-300..300).map(|i| i as f32 * 8.0).filter(|&u| {
            let q = at_line(s, u);
            q.0 > rad + 40.0 && q.1 > rad + 40.0 && q.0 < wf - rad - 40.0 && q.1 < hf - rad - 40.0
        }).collect();
        if room.is_empty() {
            continue;
        }
        let mut passes = vec![(room[r.below(room.len())], rad)];
        if r.chance(0.5) {
            let rad2 = r.range(rc.pass.0, rc.pass.1);
            let far: Vec<f32> = room.iter().copied().filter(|&u| (u - passes[0].0).abs() > 200.0).collect();
            if !far.is_empty() {
                passes.push((far[r.below(far.len())], rad2));
            }
        }
        let gate = (passes.len() == 2 && r.chance(rc.gate)).then_some(1);
        fronts.push(Front { s, half, passes, gate });
    }
    let frames: Vec<((f32, f32), (f32, f32))> = edges
        .iter()
        .map(|e| {
            let (sa, sb) = (sites[e.a], sites[e.b]);
            let nrm = unit(sb.0 - sa.0, sb.1 - sa.1);
            ((-nrm.1, nrm.0), nrm)
        })
        .collect();
    let mut ground = vec![0u8; n];
    // Caves: rock everywhere but a cavern round each site and a tunnel
    // along each border that is passable (the spanning tree, the loops and
    // the tiny corners); a gated border's tunnel is shut with soft rock at
    // its middle (`e.pass`, set here).
    let caves = rc.openness == Openness::Caves;
    let mut cave_open = vec![!caves; if caves { n } else { 0 }];
    if caves {
        let spacing = (wf * hf / rc.regions as f32).sqrt() * 0.75;
        for (k, &c) in sites.iter().enumerate() {
            let rad = if k == 0 { 130.0 } else if k <= nests.len() { 115.0 } else { spacing * r.range(0.32, 0.5) };
            let m = rad * 1.4;
            for y in ((c.1 - m).max(0.0) as usize)..((c.1 + m).max(0.0) as usize).min(h) {
                for x in ((c.0 - m).max(0.0) as usize)..((c.0 + m).max(0.0) as usize).min(w) {
                    if dist((x as f32 + 0.5, y as f32 + 0.5), c) / rad + (rough.at(x, y) - 0.5) * 0.6 < 1.0 {
                        cave_open[y * w + x] = true;
                    }
                }
            }
        }
        for e in edges.iter_mut() {
            if e.kind == Border::Ridge {
                continue;
            }
            let (a, b) = (sites[e.a], sites[e.b]);
            let pieces = ((dist(a, b) / 60.0).floor() as usize).max(2);
            let pts = wobbly(&mut r, a, b, pieces, 25.0);
            let tw = r.range(rc.pass.0, rc.pass.1) * 0.6;
            along_line(w, h, &pts, tw * 1.15, |i, _, d| {
                let (x, y) = (i % w, i / w);
                if d < tw * (0.85 + 0.3 * rough.at(x, y)) {
                    cave_open[i] = true;
                }
            });
            let mid = pts.len() / 2;
            e.pass = Some((pts[mid], tw));
        }
    }
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let at = (x as f32 + 0.5, y as f32 + 0.5);
            if caves {
                if !cave_open[i] {
                    ground[i] = G_ROCK;
                }
                continue;
            }
            let half = rc.ridge * (1.0 - rc.ridge_vary / 2.0 + rc.ridge_vary * rough.at(x, y));
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
            if !rock && !fronts.is_empty() {
                let (s, u) = (along(at) + warp_x.at(x, y), across(at));
                let wide = 0.85 + 0.3 * rough.at(x, y);
                rock = fronts.iter().any(|f| (s - f.s).abs() < f.half * wide && !f.passes.iter().any(|&(pu, rad)| (u - pu).abs() < rad))
                    && !nests.iter().chain([&landing]).any(|&q| dist(q, at) < 110.0);
            }
            if !rock && border[i] > 30.0 && blobs.at(x, y) > rc.outcrop {
                rock = !sites.iter().any(|&s| dist(s, at) < 120.0);
            }
            if rock {
                ground[i] = G_ROCK;
            }
        }
    }
    // Each pass's slot clear of every ridge's rock, not only its own
    // border's, and a disc round it as wide as the widest ridge: near a
    // corner where three regions meet a neighbouring ridge reached into
    // the slot, and where the warp bends the border the slot (square to
    // the sites' line) ran along the ridge, not through it (rocky rooms,
    // 2026-11-14: 14 of 100 highlands and core worlds took 4 to 15
    // attempts).
    if !caves {
        let disc = widest + 6.0;
        for (k, e) in edges.iter().enumerate() {
            let (Some((c, rad)), Border::Pass | Border::Gate | Border::Open) = (e.pass, e.kind) else { continue };
            let (t, nrm) = frames[k];
            let m = rad.max(disc) + 6.0;
            for y in ((c.1 - m).max(0.0) as usize)..((c.1 + m).max(0.0) as usize).min(h) {
                for x in ((c.0 - m).max(0.0) as usize)..((c.0 + m).max(0.0) as usize).min(w) {
                    let (dx, dy) = (x as f32 + 0.5 - c.0, y as f32 + 0.5 - c.1);
                    if ((dx * t.0 + dy * t.1).abs() < rad && (dx * nrm.0 + dy * nrm.1).abs() < widest + 4.0) || dx * dx + dy * dy < disc * disc {
                        ground[y * w + x] = 0;
                    }
                }
            }
        }
    }
    // Oases (`Soil::Oases`): good ground in barren, where the crystals go
    // (`Pattern::Oases`). Chosen before the soil, which leaves them plain.
    let mut oases: Vec<((f32, f32), f32)> = Vec::new();
    if rc.soil == Soil::Oases || rc.pattern == Pattern::Oases {
        let area = wf * hf / (MAP_W * MAP_H) as f32;
        let want = ((4.0 * area + 0.5).floor() as usize).max(3) + r.below(3);
        for _ in 0..want * 60 {
            if oases.len() >= want {
                break;
            }
            let c = (r.range(90.0, wf - 90.0), r.range(90.0, hf - 90.0));
            let dl = dist(c, landing);
            let dn = nests.iter().map(|&q| dist(c, q)).fold(f32::MAX, f32::min);
            let i = c.1 as usize * w + c.0 as usize;
            if dl < 200.0 || dn < 160.0 || ground[i] & G_ROCK != 0 || oases.iter().any(|&(o, _)| dist(o, c) < 170.0) || !(0.2..=0.85).contains(&(dl / (dl + dn))) {
                continue;
            }
            oases.push((c, r.range(70.0, 100.0)));
        }
    }

    // Rivers (`Water::Rivers`): bands right across the route, meandering
    // more than the fronts, deep in the middle and shallow at the banks,
    // with one or two fords where the deep part stops. Streams
    // (`Water::Streams`): narrow wandering lines, shallow all through.
    struct River {
        s: f32,
        half: f32,
        fords: Vec<(f32, f32)>,
        /// A bridge across the deep part (`G_BRIDGE`, the ground thread's
        /// kind 5): where along the river and its half-width, a narrow
        /// crossing to hold or wall off, away from the fords.
        bridge: Option<(f32, f32)>,
    }
    let mut rivers: Vec<River> = Vec::new();
    let mut stream = vec![false; if rc.water == Water::Streams { n } else { 0 }];
    if rc.water == Water::Rivers {
        let k = rc.rivers;
        for j in 0..k {
            let s = span * (0.5 + (j as f32 - (k as f32 - 1.0) / 2.0) * 0.18 + r.range(-0.03, 0.03));
            // At least 40 map cells across (10-14 of the 256 x 160 grid the
            // graphics lead counts in, 4 map cells each): at 34 the river
            // read as a thin line at zoom 1 (graphics lead, 2026-11-14 18:56Z).
            let half = r.range(22.0, 28.0);
            let room: Vec<f32> = (-300..300).map(|i| i as f32 * 8.0).filter(|&u| {
                let q = at_line(s, u);
                q.0 > 60.0 && q.1 > 60.0 && q.0 < wf - 60.0 && q.1 < hf - 60.0
            }).collect();
            let mut fords = Vec::new();
            if !room.is_empty() {
                fords.push((room[r.below(room.len())], r.range(26.0, 36.0)));
                if r.chance(0.6) {
                    let far: Vec<f32> = room.iter().copied().filter(|&u| (u - fords[0].0).abs() > 220.0).collect();
                    if !far.is_empty() {
                        fords.push((far[r.below(far.len())], r.range(26.0, 36.0)));
                    }
                }
            }
            let far: Vec<f32> = room.iter().copied().filter(|&u| fords.iter().all(|f| (u - f.0).abs() > 180.0)).collect();
            let bridge = (r.chance(0.5) && !far.is_empty()).then(|| (far[r.below(far.len())], r.range(6.0, 8.0)));
            rivers.push(River { s, half, fords, bridge });
        }
    }
    if rc.water == Water::Streams {
        for _ in 0..rc.rivers {
            let from = (r.range(0.2, 0.8) * wf, r.range(0.2, 0.8) * hf);
            let dir = bend(perp, r.range(-0.6, 0.6));
            // At least 16 map cells across (4 of the graphics lead's grid
            // cells); 8-12 was too thin to read (graphics lead, 18:56Z).
            let half = r.range(9.0, 12.0);
            for d in [dir, (-dir.0, -dir.1)] {
                let pts = wander(&mut r, wf, hf, from, d, 24.0, 0.25);
                along_line(w, h, &pts, half * 1.3, |i, _, dd| {
                    if dd < half * (0.9 + 0.2 * fine.at(i % w, i / w)) {
                        stream[i] = true;
                    }
                });
            }
        }
    }

    // Soil and water, by the dials, in patches that ignore the regions;
    // plain round the landing, clay under the nests where the soil or the
    // water holds the slime there (as the designed maps do).
    let nest_clay = matches!(rc.soil, Soil::Clay | Soil::Cracks | Soil::Dunes | Soil::Mixed) || matches!(rc.water, Water::Pools | Water::Lakes);
    // The cracks (`Soil::Cracks`): the borders of a jittered grid's Voronoi
    // cells, `CRACK` map cells across (a mud crack's polygons), broken up
    // by noise.
    const CRACK: f32 = 110.0;
    let (cgw, cgh) = ((wf / CRACK) as usize + 3, (hf / CRACK) as usize + 3);
    let crack_pts: Vec<(f32, f32)> = if rc.soil == Soil::Cracks {
        (0..cgw * cgh).map(|k| (((k % cgw) as f32 - 1.0 + r.range(0.15, 0.85)) * CRACK, ((k / cgw) as f32 - 1.0 + r.range(0.15, 0.85)) * CRACK)).collect()
    } else {
        Vec::new()
    };
    let crack_at = |q: (f32, f32)| -> f32 {
        let (gx, gy) = ((q.0 / CRACK) as usize + 1, (q.1 / CRACK) as usize + 1);
        let (mut d1, mut d2, mut p1, mut p2) = (f32::MAX, f32::MAX, q, q);
        for j in gy - 1..=gy + 1 {
            for i in gx - 1..=gx + 1 {
                let c = crack_pts[j * cgw + i];
                let d = (q.0 - c.0) * (q.0 - c.0) + (q.1 - c.1) * (q.1 - c.1);
                if d < d1 {
                    (d2, p2, d1, p1) = (d1, p1, d, c);
                } else if d < d2 {
                    (d2, p2) = (d, c);
                }
            }
        }
        (d2 - d1) / (2.0 * dist(p1, p2).max(1.0))
    };
    // Lakes: a broader noise than the pools, with a deep middle.
    let lakes = Field::new(w, h, |x, y| nw.fbm(x - 2903.0, y + 811.0, 300.0));
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let at = (x as f32 + 0.5, y as f32 + 0.5);
            let (a, b) = (soil_a.at(x, y), soil_b.at(x, y));
            let dl = dist(at, landing);
            let dn = nests.iter().map(|&q| dist(at, q)).fold(f32::MAX, f32::min);
            let ragged = (b - 0.5) * 80.0 + (fine.at(x, y) - 0.5) * 110.0;
            let mut soil = 0u8;
            match rc.soil {
                Soil::Plain => {}
                Soil::Sand => {
                    if a > 0.57 {
                        soil = G_SAND;
                    }
                }
                Soil::Dunes => {
                    if a > 0.36 {
                        soil = G_SAND;
                    }
                }
                Soil::Bands => {
                    // Three wide bands right across the route, meandering
                    // and ragged. (Noise patches, the first try, read as
                    // plain ground at zoom 1: graphics lead, 2026-11-14.)
                    let s = (along(at) + warp_x.at(x, y)) / span.max(1.0);
                    let half = 0.06 * (0.8 + 0.4 * a) + (fine.at(x, y) - 0.5) * 0.08;
                    if [0.28, 0.5, 0.72].iter().any(|&c| (s - c).abs() < half) {
                        soil = G_BARREN;
                    }
                }
                Soil::Oases => {
                    // Barren but the oases (ragged discs), with clay round
                    // their rims.
                    let o = oases.iter().map(|&(c, rad)| dist(at, c) - rad).fold(f32::MAX, f32::min) + ragged * 0.3;
                    soil = if o < 0.0 {
                        0
                    } else if o < 18.0 {
                        G_CLAY
                    } else {
                        G_BARREN
                    };
                }
                Soil::Clay => {
                    if a > 0.52 {
                        soil = G_CLAY;
                    }
                }
                Soil::Cracks => {
                    // Clay nearly everywhere, cracked into polygons of plain
                    // ground here and there.
                    if a > 0.24 && !(b > 0.42 && crack_at(at) < 2.5 + 4.0 * fine.at(x, y)) {
                        soil = G_CLAY;
                    }
                }
                Soil::Mixed => {
                    if a > 0.6 {
                        soil = G_SAND;
                    } else if a < 0.38 {
                        soil = G_CLAY;
                    } else if b < 0.34 {
                        soil = G_BARREN;
                    }
                }
            }
            // Water: 0 none, 1 shallow, 2 deep.
            let mut water = 0u8;
            match rc.water {
                Water::Dry => {}
                Water::Pools => {
                    // Big pools that run together, on clay banks.
                    let pool = pools.at(x, y) + (fine.at(x, y) - 0.5) * 0.12;
                    if pool > 0.56 {
                        water = 1;
                    } else if pool > 0.5 {
                        soil = G_CLAY;
                    }
                }
                Water::Lakes => {
                    let lake = lakes.at(x, y) + (fine.at(x, y) - 0.5) * 0.1;
                    if lake > 0.63 {
                        water = 2;
                    } else if lake > 0.57 {
                        water = 1;
                    } else if lake > 0.53 {
                        soil = G_CLAY;
                    }
                }
                Water::Rivers => {
                    let (sa, u) = (along(at) + warp_x.at(x, y) * 1.6, across(at));
                    for rv in &rivers {
                        let off = (sa - rv.s).abs() - rv.half * (0.9 + 0.2 * rough.at(x, y));
                        if off < 0.0 && rv.bridge.is_some_and(|(bu, bh)| (u - bu).abs() < bh) {
                            soil = G_BRIDGE;
                        } else if off < 0.0 {
                            let ford = rv.fords.iter().any(|&(fu, fr)| (u - fu).abs() < fr);
                            water = water.max(if off < -8.0 && !ford { 2 } else { 1 });
                        } else if off < 12.0 && soil == 0 {
                            // Sand on the banks.
                            soil = G_SAND;
                        }
                    }
                }
                Water::Streams => {
                    if stream[i] {
                        water = 1;
                    }
                }
            }
            if nest_clay && dn < 130.0 + ragged {
                (soil, water) = (G_CLAY, 0);
            }
            if dl < 150.0 + ragged {
                (soil, water) = (0, 0);
            }
            if water > 0 && (dn < 100.0 || dl < 170.0) {
                water = 0;
            }
            if ground[i] & G_ROCK == 0 {
                ground[i] = match water {
                    0 => soil,
                    1 => G_WET,
                    _ => DEEP,
                };
            }
        }
    }

    // Crystals.
    let mut items = Items { landing: Some(landing), ..Items::default() };
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
        items.crystals.push(Crystal { x: q.0.round(), y: q.1.round(), r: rad.round(), charge });
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
        // In the landing's own region: across a ridge, the nearest crystal
        // was a long walk round (rocky rooms, 2026-11-14).
        let home = c.0 > 0.0 && c.1 > 0.0 && c.0 < wf && c.1 < hf && region[c.1 as usize * w + c.0 as usize] == 0;
        if home && clear(&ground, &items, c, rad, 30.0) {
            items.crystals.push(Crystal { x: c.0.round(), y: c.1.round(), r: rad.round(), charge: [192.0, 256.0][r.below(2)] });
            break;
        }
    }
    // The rest by the pattern. `put` places one if its ground is clear.
    let put = |items: &mut Items, ground: &[u8], c: (f32, f32), rad: f32, charge: f32, gap: f32| -> bool {
        let ok = clear(ground, items, c, rad, gap);
        if ok {
            items.crystals.push(Crystal { x: c.0.round(), y: c.1.round(), r: rad.round(), charge });
        }
        ok
    };
    // Where a point lies between the landing (0) and the nearest nest (1).
    let between = |c: (f32, f32)| {
        let dl = dist(c, landing);
        let dn = nests.iter().map(|&q| dist(c, q)).fold(f32::MAX, f32::min);
        dl / (dl + dn).max(1.0)
    };
    let scatter = |r: &mut Rng, items: &mut Items, ground: &[u8], count: usize, size: (f32, f32), charges: &[f32]| {
        let mut placed = 0;
        for _ in 0..count * 250 {
            if placed >= count {
                break;
            }
            let c = (r.range(40.0, wf - 40.0), r.range(40.0, hf - 40.0));
            let rad = r.range(size.0, size.1);
            if put(items, ground, c, rad, charges[r.below(charges.len())], 50.0) {
                placed += 1;
            }
        }
    };
    let midway = |r: &mut Rng, items: &mut Items, ground: &[u8], count: usize, size: (f32, f32), charges: &[f32], band: (f32, f32), gap: f32| -> Vec<(f32, f32)> {
        let mut at = Vec::new();
        for _ in 0..count * 400 {
            if at.len() >= count {
                break;
            }
            let c = (r.range(60.0, wf - 60.0), r.range(60.0, hf - 60.0));
            if !(band.0..=band.1).contains(&between(c)) {
                continue;
            }
            let rad = r.range(size.0, size.1);
            if put(items, ground, c, rad, charges[r.below(charges.len())], gap) {
                at.push(c);
            }
        }
        at
    };
    match rc.pattern {
        Pattern::Scattered => {
            // Big ones between, the middle of the way from the landing to a
            // nest; small ones anywhere.
            midway(&mut r, &mut items, &ground, rc.big, (30.0, 42.0), &[384.0, 512.0], (0.35, 0.7), 70.0);
            scatter(&mut r, &mut items, &ground, rc.small, (18.0, 24.0), &[96.0, 128.0, 192.0]);
        }
        Pattern::Fields => {
            // A big crystal ringed by small ones, a few of them; a field
            // held is worth a lot.
            let k = rc.big + 1;
            let centres = midway(&mut r, &mut items, &ground, k, (32.0, 42.0), &[384.0, 512.0], (0.25, 0.8), 150.0);
            for c in centres {
                let big = items.crystals.last().map_or(36.0, |b| b.r);
                let m = 3 + r.below(3);
                let mut v = unit(r.range(-1.0, 1.0), r.range(-1.0, 1.0));
                for _ in 0..m {
                    let d = big + r.range(38.0, 58.0);
                    let q = (c.0 + v.0 * d, c.1 + v.1 * d);
                    put(&mut items, &ground, q, r.range(14.0, 20.0), [96.0, 128.0][r.below(2)], 16.0);
                    for _ in 0..24 / m {
                        v = turn15(v);
                    }
                }
            }
            scatter(&mut r, &mut items, &ground, rc.small / 3, (16.0, 22.0), &[96.0, 128.0]);
        }
        Pattern::Veins => {
            // Lines of small crystals across the route, like seams, each
            // bending a little; one big one midway.
            midway(&mut r, &mut items, &ground, 1, (30.0, 40.0), &[384.0, 512.0], (0.4, 0.65), 70.0);
            let veins = 1 + r.below(2) + (rc.small + 6) / 12;
            for _ in 0..veins {
                // A middle point between the landing and the nests, then
                // both ways along the seam from it.
                let mut mid = None;
                for _ in 0..200 {
                    let c = (r.range(60.0, wf - 60.0), r.range(60.0, hf - 60.0));
                    if (0.25..=0.8).contains(&between(c)) && clear(&ground, &items, c, 18.0, 14.0) {
                        mid = Some(c);
                        break;
                    }
                }
                let Some(mid) = mid else { continue };
                let d0 = bend(perp, r.range(-0.4, 0.4));
                let len = 4 + r.below(4);
                let mut placed = 0;
                for d in [d0, (-d0.0, -d0.1)] {
                    let (mut c, mut dir) = (mid, d);
                    if d != d0 {
                        c = (c.0 + dir.0 * 60.0, c.1 + dir.1 * 60.0);
                    }
                    for _ in 0..len * 2 {
                        if placed >= len || c.0 < 30.0 || c.1 < 30.0 || c.0 > wf - 30.0 || c.1 > hf - 30.0 {
                            break;
                        }
                        if put(&mut items, &ground, c, r.range(12.0, 18.0), [128.0, 192.0][r.below(2)], 14.0) {
                            placed += 1;
                        }
                        let step = r.range(52.0, 68.0);
                        c = (c.0 + dir.0 * step, c.1 + dir.1 * step);
                        dir = bend(dir, r.range(-0.2, 0.2));
                    }
                }
            }
        }
        Pattern::Jackpot => {
            // One giant midway, a few small ones; little else.
            let got = midway(&mut r, &mut items, &ground, 1, (56.0, 70.0), &[512.0, 640.0], (0.4, 0.62), 90.0);
            if got.is_empty() {
                midway(&mut r, &mut items, &ground, 1, (36.0, 46.0), &[512.0, 640.0], (0.3, 0.7), 70.0);
            }
            scatter(&mut r, &mut items, &ground, rc.small * 2 / 3, (14.0, 20.0), &[96.0, 128.0]);
        }
        Pattern::Oases => {
            // Clusters at the oases: one middling crystal at the middle and
            // one to three small ones round it.
            for &(c, rad) in &oases {
                put(&mut items, &ground, c, r.range(24.0, 32.0), [256.0, 384.0][r.below(2)], 20.0);
                let m = 1 + r.below(3);
                let mut placed = 0;
                for _ in 0..m * 30 {
                    if placed >= m {
                        break;
                    }
                    let v = unit(r.range(-1.0, 1.0), r.range(-1.0, 1.0));
                    let d = r.range(48.0, rad * 0.8 + 10.0);
                    if put(&mut items, &ground, (c.0 + v.0 * d, c.1 + v.1 * d), r.range(14.0, 20.0), [96.0, 128.0][r.below(2)], 14.0) {
                        placed += 1;
                    }
                }
            }
        }
        Pattern::Richer => {
            // Small near the landing, bigger and richer toward the slime,
            // which pulls the squad out.
            let count = rc.small + rc.big;
            let mut placed = 0;
            for _ in 0..count * 250 {
                if placed >= count {
                    break;
                }
                let c = (r.range(40.0, wf - 40.0), r.range(40.0, hf - 40.0));
                let t = between(c).clamp(0.0, 1.0);
                let rad = 12.0 + 28.0 * t + r.range(-3.0, 3.0);
                let charge = [96.0, 128.0, 192.0, 256.0, 384.0, 512.0][((t * 6.0) as usize).min(5)];
                if put(&mut items, &ground, c, rad, charge, 40.0) {
                    placed += 1;
                }
            }
        }
    }
    // Pillars (`Openness::Pillars`): many small ones, apart, each its own ragged
    // disc, off the landing, the nests and the crystals. After the
    // crystals, which would otherwise find no room between them.
    let mut pillars: Vec<((f32, f32), f32)> = Vec::new();
    let want = rc.pillars * n / (MAP_W * MAP_H);
    for _ in 0..want * 30 {
        if pillars.len() >= want {
            break;
        }
        let c = (r.range(30.0, wf - 30.0), r.range(30.0, hf - 30.0));
        if dist(c, landing) < 150.0 || nests.iter().any(|&q| dist(q, c) < 120.0) || items.crystals.iter().any(|k| dist((k.x, k.y), c) < k.r + 36.0) || pillars.iter().any(|&(q, _)| dist(q, c) < 46.0) {
            continue;
        }
        pillars.push((c, r.range(7.0, 15.0)));
    }
    for &(c, rad) in &pillars {
        let m = rad * 1.3;
        for y in ((c.1 - m).max(0.0) as usize)..((c.1 + m) as usize).min(h) {
            for x in ((c.0 - m).max(0.0) as usize)..((c.0 + m) as usize).min(w) {
                if dist((x as f32 + 0.5, y as f32 + 0.5), c) < rad * (0.75 + 0.5 * fine.at(x, y)) {
                    ground[y * w + x] = G_ROCK;
                }
            }
        }
    }

    // Dunes: clay islands under the crystals, as on the Flats.
    if rc.soil == Soil::Dunes {
        for c in &items.crystals {
            // Big and ragged: exact circles read as decals or holes
            // (graphics lead, 2026-11-14).
            let m = c.r * 2.4;
            for y in ((c.y - m * 1.4).max(0.0) as usize)..((c.y + m * 1.4) as usize).min(h) {
                for x in ((c.x - m * 1.4).max(0.0) as usize)..((c.x + m * 1.4) as usize).min(w) {
                    let d = dist((x as f32 + 0.5, y as f32 + 0.5), (c.x, c.y)) / m + (soil_b.at(x, y) - 0.5) * 0.6 + (fine.at(x, y) - 0.5) * 1.2;
                    let i = y * w + x;
                    if d < 1.0 && ground[i] & G_ROCK == 0 {
                        ground[i] = (ground[i] & 0x0F) | G_CLAY;
                    }
                }
            }
        }
    }

    // Soft rock: gates across passes, rings round nests, hedges. Each is a
    // chain of short pieces that wobbles and changes thickness, so it reads
    // as grown: one straight bar, the first try, read as UI, and perfect
    // arcs as selection rings (graphics lead, 2026-11-14).
    let inside = |p: (f32, f32), m: f32| p.0 > m && p.1 > m && p.0 < wf - m && p.1 < hf - m;
    for e in &edges {
        if e.kind != Border::Gate {
            continue;
        }
        let Some((c, rad)) = e.pass else { continue };
        let (sa, sb) = (sites[e.a], sites[e.b]);
        let t = unit(-(sb.1 - sa.1), sb.0 - sa.0);
        // Across the slot, its ends in the ridge's rock.
        let half = rad + 8.0;
        let pts = wobbly(&mut r, (c.0 - t.0 * half, c.1 - t.1 * half), (c.0 + t.0 * half, c.1 + t.1 * half), 4, 5.0);
        hedge(&mut r, &mut items.walls, &pts);
    }
    for f in &fronts {
        let Some(g) = f.gate else { continue };
        let (u, rad) = f.passes[g];
        // The front's middle line at the pass, through the warp.
        let mut s = f.s;
        for _ in 0..3 {
            let q = at_line(s, u);
            let (x, y) = ((q.0.max(0.0) as usize).min(w - 1), (q.1.max(0.0) as usize).min(h - 1));
            s = f.s - warp_x.at(x, y);
        }
        let half = rad + 8.0;
        let pts = wobbly(&mut r, at_line(s, u - half), at_line(s, u + half), 4, 5.0);
        hedge(&mut r, &mut items.walls, &pts);
    }
    for (k, &q) in nests.iter().enumerate() {
        if !r.chance(rc.ring) {
            continue;
        }
        let rad = items.crystals[k].r + 55.0;
        let to = unit(landing.0 - q.0, landing.1 - q.1);
        // The radius wanders round the ring, smoothly.
        let mut radii = [0.0f32; 24];
        let mut off = 0.0;
        for v in radii.iter_mut() {
            off = off * 0.6 + r.range(-0.07, 0.07);
            *v = rad * (1.0 + off);
        }
        let mut v = (1.0f32, 0.0f32);
        for i in 0..24 {
            let u = turn15(v);
            let mid = unit(v.0 + u.0, v.1 + u.1);
            let (ra, rb) = (radii[i], radii[(i + 1) % 24]);
            let (a, b) = ((q.0 + v.0 * ra, q.1 + v.1 * ra), (q.0 + u.0 * rb, q.1 + u.1 * rb));
            if mid.0 * to.0 + mid.1 * to.1 < 0.85 && inside(a, 8.0) && inside(b, 8.0) {
                hedge(&mut r, &mut items.walls, &[a, b]);
            }
            v = u;
        }
    }
    for _ in 0..rc.hedges {
        for _ in 0..60 {
            let mut at = (r.range(80.0, wf - 80.0), r.range(80.0, hf - 80.0));
            let mut dir = unit(r.range(-1.0, 1.0), r.range(-1.0, 1.0));
            // A steady curve (7.5 degrees a piece) with a wander on it.
            let curve = if r.chance(0.5) { 0.13 } else { -0.13 };
            let pieces = 8 + r.below(8);
            let mut pts = vec![at];
            for _ in 0..pieces {
                let b = (at.0 + dir.0 * 17.0, at.1 + dir.1 * 17.0);
                // Off the passes, which a hedge would plug (gates do that,
                // at their own odds).
                let blocked = |p: (f32, f32)| {
                    !inside(p, 24.0)
                        || dist(p, landing) < 160.0
                        || edges.iter().any(|e| e.pass.is_some_and(|(c, rad)| dist(p, c) < rad + 30.0))
                        || items.crystals.iter().any(|c| dist((c.x, c.y), p) < c.r + 34.0)
                        || ground[p.1 as usize * w + p.0 as usize] & G_ROCK != 0
                };
                if blocked(at) || blocked(b) {
                    break;
                }
                pts.push(b);
                at = b;
                dir = bend(dir, curve + r.range(-0.2, 0.2));
            }
            if pts.len() >= 7 {
                hedge(&mut r, &mut items.walls, &pts);
                break;
            }
        }
    }

    cut_fords(w, h, &mut ground, &items, landing);
    let canopy = Field::new(w, h, |x, y| noise(7).fbm(x + 4111.0, y - 2377.0, 140.0));
    let plants = sow(&Sow::of(p.biome, &rc), w, h, &ground, &items, landing, &canopy, &fine);
    Map { w, h, ground, plants, items }
}

/// Fords where deep water cuts the squad off. For each crystal or nest the
/// landing can't walk to but could if deep water were shallow, the
/// shortest way there is made shallow (`G_WET`) `FORD` map cells either
/// side, as a pass is cut through a ridge. What rock shuts off is left to
/// `check`. Before this, deep lakes and rivers through rocky rooms shut 29
/// of 400 planets on the first attempt, one core world for 23 attempts.
fn cut_fords(w: usize, h: usize, ground: &mut [u8], items: &Items, landing: (f32, f32)) {
    const FORD: f32 = 16.0;
    if !ground.contains(&G_DEEP) {
        return;
    }
    let targets: Vec<((f32, f32), f32)> = items.crystals.iter().map(|c| ((c.x, c.y), c.r)).chain(items.moulds.iter().map(|&q| (q, 8.0))).collect();
    let mut skip = vec![false; targets.len()];
    for _ in 0..12 {
        let m = Map { w, h, ground: ground.to_vec(), plants: Vec::new(), items: items.clone() };
        let (bw, bh, open) = walkable(&m, (w, h), false);
        let d = walk_from(bw, bh, &open, landing);
        let Some(t) = (0..targets.len()).find(|&t| !skip[t] && reach(bw, bh, &d, targets[t].0, targets[t].1) == u32::MAX) else { return };
        // The same walk with deep water as ground.
        let shallow = Map { ground: m.ground.iter().map(|&g| if g == G_DEEP { G_WET } else { g }).collect(), ..m };
        let (_, _, open2) = walkable(&shallow, (w, h), false);
        let d2 = walk_from(bw, bh, &open2, landing);
        let (c, rad) = targets[t];
        // The bin by the target nearest the landing, as `reach` reads it.
        let mm = rad + 2.0 * BIN as f32 + 2.0;
        let mut at = None;
        for by in ((c.1 - mm) / BIN as f32).floor().max(0.0) as usize..=(((c.1 + mm) / BIN as f32) as usize).min(bh - 1) {
            for bx in ((c.0 - mm) / BIN as f32).floor().max(0.0) as usize..=(((c.0 + mm) / BIN as f32) as usize).min(bw - 1) {
                let p = ((bx * BIN) as f32 + 2.0, (by * BIN) as f32 + 2.0);
                let i = by * bw + bx;
                if dist(p, c) <= mm && d2[i] != u32::MAX && at.is_none_or(|j: usize| d2[i] < d2[j]) {
                    at = Some(i);
                }
            }
        }
        let Some(mut i) = at else {
            skip[t] = true;
            continue;
        };
        // Down the walk to the landing, making the deep water shallow.
        loop {
            let p = ((i % bw * BIN) as f32 + 2.0, (i / bw * BIN) as f32 + 2.0);
            let (x0, x1) = ((p.0 - FORD).max(0.0) as usize, ((p.0 + FORD) as usize).min(w - 1));
            let (y0, y1) = ((p.1 - FORD).max(0.0) as usize, ((p.1 + FORD) as usize).min(h - 1));
            for y in y0..=y1 {
                for x in x0..=x1 {
                    if ground[y * w + x] == G_DEEP && dist((x as f32 + 0.5, y as f32 + 0.5), p) <= FORD {
                        ground[y * w + x] = G_WET;
                    }
                }
            }
            if d2[i] == 0 {
                break;
            }
            let (x, y) = ((i % bw) as i32, (i / bw) as i32);
            let next = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)]
                .iter()
                .map(|&(dx, dy)| (x + dx, y + dy))
                .filter(|&(nx, ny)| nx >= 0 && ny >= 0 && nx < bw as i32 && ny < bh as i32)
                .map(|(nx, ny)| ny as usize * bw + nx as usize)
                .min_by_key(|&j| d2[j]);
            match next {
                Some(j) if d2[j] < d2[i] => i = j,
                _ => break,
            }
        }
        // A target the shallow walk still can't reach is the check's.
        skip[t] = true;
    }
}

/// How a biome's plants start (the plants thread's field, `plants::pack`;
/// the B byte the map carries, 2026-11-14). On one canopy noise `n` (0-1,
/// patches about 140 map cells across, ragged by the fine noise): bare
/// below `bare`, `main` above it, jungle above `jungle` (over 1: never);
/// reeds within `shore` map cells of the water's edge on both sides, so
/// pools and rivers are fringed and their middles open; `sand` the kind on
/// sand (`None` bare). Density runs from `thin` at `bare` to full `ramp`
/// above it, so a patch thickens toward its middle (the graphics lead
/// asked for density gradients, not flat fills, 18:20Z). The soil's own
/// capacity caps it in the game: barren and deep water grow nothing.
#[derive(Clone, Copy, Debug)]
struct Sow {
    bare: f32,
    main: u8,
    jungle: f32,
    shore: f32,
    sand: Option<u8>,
    thin: f32,
    ramp: f32,
    /// Plain ground thick with growth (an oasis in the barrens).
    lush: bool,
}

impl Sow {
    fn of(b: Biome, rc: &Recipe) -> Sow {
        use crate::mould::plants::{MEADOW, SCRUB};
        let base = Sow { bare: 0.3, main: MEADOW, jungle: 0.78, shore: 10.0, sand: Some(SCRUB), thin: 0.35, ramp: 0.3, lush: false };
        match b {
            // Grass nearly everywhere, a few copses.
            Biome::Meadow => Sow { bare: 0.25, jungle: 0.74, ..base },
            // Thin scrub on what isn't barren; the oases green.
            Biome::Barrens => Sow { bare: 0.42, main: SCRUB, jungle: 1.1, shore: 0.0, thin: 0.25, ramp: 0.4, lush: rc.soil == Soil::Oases, ..base },
            // Reed beds round every pool and lake.
            Biome::Fen => Sow { shore: 18.0, ..base },
            // Reeds along the banks, scrub on the dunes.
            Biome::Valley => Sow { shore: 14.0, jungle: 0.8, thin: 0.3, ..base },
            // Canopy over most of it, meadow in the clearings.
            Biome::Jungle => Sow { bare: 0.15, jungle: 0.3, shore: 8.0, thin: 0.4, ramp: 0.2, ..base },
            // Sparse grass in the rooms.
            Biome::Highlands => Sow { bare: 0.6, jungle: 1.1, shore: 6.0, thin: 0.25, ..base },
            // A little growth in the caverns, reeds at the pools.
            Biome::Caves => Sow { bare: 0.6, jungle: 1.1, shore: 6.0, sand: None, thin: 0.2, ..base },
            Biome::Core => Sow { jungle: 0.62, ..base },
        }
    }
}

/// Map cells to the nearest cell where `to` holds, by a 3-4 chamfer (in
/// thirds of a cell; `u16::MAX` where there is none).
fn chamfer(w: usize, h: usize, to: impl Fn(usize) -> bool) -> Vec<u16> {
    let mut d: Vec<u16> = (0..w * h).map(|i| if to(i) { 0 } else { u16::MAX }).collect();
    let near = |d: &[u16], i: usize, c: u16| d[i].saturating_add(c);
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let mut v = d[i];
            if x > 0 {
                v = v.min(near(&d, i - 1, 3));
            }
            if y > 0 {
                v = v.min(near(&d, i - w, 3));
                if x > 0 {
                    v = v.min(near(&d, i - w - 1, 4));
                }
                if x + 1 < w {
                    v = v.min(near(&d, i - w + 1, 4));
                }
            }
            d[i] = v;
        }
    }
    for y in (0..h).rev() {
        for x in (0..w).rev() {
            let i = y * w + x;
            let mut v = d[i];
            if x + 1 < w {
                v = v.min(near(&d, i + 1, 3));
            }
            if y + 1 < h {
                v = v.min(near(&d, i + w, 3));
                if x + 1 < w {
                    v = v.min(near(&d, i + w + 1, 4));
                }
                if x > 0 {
                    v = v.min(near(&d, i + w - 1, 4));
                }
            }
            d[i] = v;
        }
    }
    d
}

/// The plants a map starts with, by `sow` (see `Sow`), as B bytes.
#[allow(clippy::too_many_arguments)]
fn sow(sw: &Sow, w: usize, h: usize, ground: &[u8], items: &Items, landing: (f32, f32), canopy: &Field, fine: &Field) -> Vec<u8> {
    use crate::mould::plants::{pack, JUNGLE, REEDS};
    let wet = |i: usize| ground[i] & G_WET != 0;
    // Map cells to the water's edge, either side (one chamfer from the
    // cells whose neighbour is the other kind); none on a dry planet.
    let any_wet = sw.shore > 0.0 && ground.iter().any(|&g| g & G_WET != 0);
    let to_edge = if any_wet {
        chamfer(w, h, |i| {
            let (x, y) = (i % w, i / w);
            (x > 0 && wet(i - 1) != wet(i)) || (x + 1 < w && wet(i + 1) != wet(i)) || (y > 0 && wet(i - w) != wet(i)) || (y + 1 < h && wet(i + w) != wet(i))
        })
    } else {
        Vec::new()
    };
    let ramp = |v: f32, from: f32, over: f32| ((v - from) / over).clamp(0.0, 1.0);
    let mut out = vec![0u8; w * h];
    for y in 0..h {
        for x in 0..w {
            let i = y * w + x;
            let g = ground[i];
            if g & G_ROCK != 0 {
                continue;
            }
            let n = canopy.at(x, y) + (fine.at(x, y) - 0.5) * 0.14;
            let soil = g >> 4;
            // Deep water and bridges grow nothing (`plants::SOIL_CAP`).
            if g == G_DEEP || g == G_BRIDGE {
                continue;
            }
            if any_wet {
                // Reeds: thickest at the water's edge, thinning out to
                // `shore` on land and in the water both.
                // Into the water only a few cells, so open water still
                // reads (a full fringe hid the rivers' shallows).
                let d = to_edge[i] as f32 / 3.0;
                let reach = sw.shore * (0.7 + 0.6 * fine.at(x, y)) * if wet(i) { 0.35 } else { 1.0 };
                if d <= reach {
                    out[i] = pack(REEDS, 1.0 - 0.6 * d / reach.max(1.0));
                    continue;
                }
            }
            if wet(i) || soil == 3 {
                continue;
            }
            let (bare, jungle) = if sw.lush && soil == 0 { (0.12, 0.6) } else { (sw.bare, sw.jungle) };
            if n < bare {
                continue;
            }
            out[i] = if soil == 1 {
                match sw.sand {
                    Some(k) => pack(k, sw.thin + (1.0 - sw.thin) * ramp(n, bare, sw.ramp)),
                    None => 0,
                }
            } else if n >= jungle {
                pack(JUNGLE, 0.5 + 0.5 * ramp(n, jungle, 0.15))
            } else {
                pack(if sw.lush && soil == 0 { crate::mould::plants::MEADOW } else { sw.main }, sw.thin + (1.0 - sw.thin) * ramp(n, bare, sw.ramp))
            };
        }
    }
    // A ragged clear patch where the squad lands, and nothing on a crystal
    // (the game grows nothing there either). Each over its own box: over
    // every cell and every crystal this pass was most of a planet's build.
    let mut clear = |c: (f32, f32), reach: f32, inside: &dyn Fn(usize, usize, (f32, f32)) -> bool| {
        let (x0, y0) = ((c.0 - reach).max(0.0) as usize, (c.1 - reach).max(0.0) as usize);
        let (x1, y1) = (((c.0 + reach) as usize + 1).min(w), ((c.1 + reach) as usize + 1).min(h));
        for y in y0..y1 {
            for x in x0..x1 {
                if inside(x, y, (x as f32 + 0.5, y as f32 + 0.5)) {
                    out[y * w + x] = 0;
                }
            }
        }
    };
    clear(landing, 50.0, &|x, y, at| dist(at, landing) < 30.0 + (fine.at(x, y) - 0.5) * 40.0);
    for c in &items.crystals {
        let q = (c.x, c.y);
        clear(q, c.r + 2.0, &|_, _, at| dist(at, q) < c.r + 2.0);
    }
    out
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
            if !standable(m.ground[y * w + x]) {
                open[(y / BIN) * bw + x / BIN] = false;
            }
        }
    }
    let shut = |open: &mut Vec<bool>, c: (f32, f32), rad: f32| {
        let (x0, x1) = (((c.0 - rad) / BIN as f32).floor().max(0.0) as usize, (((c.0 + rad) / BIN as f32) as usize).min(bw - 1));
        let (y0, y1) = (((c.1 - rad) / BIN as f32).floor().max(0.0) as usize, (((c.1 + rad) / BIN as f32) as usize).min(bh - 1));
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
                shut(&mut open, (wl.a.0 + (wl.b.0 - wl.a.0) * t, wl.a.1 + (wl.b.1 - wl.a.1) * t), wl.thick / 2.0 + 1.0);
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
            let near_shut = !edge && (!open[by * bw + bx - 1] || !open[by * bw + bx + 1] || !open[(by - 1) * bw + bx] || !open[(by + 1) * bw + bx]);
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
        for (dx, dy, c) in [(1, 0, 5), (-1, 0, 5), (0, 1, 5), (0, -1, 5), (1, 1, 7), (1, -1, 7), (-1, 1, 7), (-1, -1, 7)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= bw as i32 || ny >= bh as i32 {
                continue;
            }
            let j = ny as usize * bw + nx as usize;
            if !open[j] || (dx != 0 && dy != 0 && (!open[y as usize * bw + nx as usize] || !open[ny as usize * bw + x as usize])) {
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
    let (x0, x1) = (((c.0 - m) / BIN as f32).floor().max(0.0) as usize, (((c.0 + m) / BIN as f32) as usize).min(bw - 1));
    let (y0, y1) = (((c.1 - m) / BIN as f32).floor().max(0.0) as usize, (((c.1 + m) / BIN as f32) as usize).min(bh - 1));
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
    let open: f32 = m.items.crystals.iter().filter(|c| reach(bw, bh, &dh, (c.x, c.y), c.r) != u32::MAX).map(held).sum();
    Stats {
        crystals: m.items.crystals.len(),
        charge: total,
        rock: count(&|g| g & G_ROCK != 0),
        sand: count(&|g| g >> 4 == 1),
        clay: count(&|g| g >> 4 == 2),
        barren: count(&|g| g >> 4 == 3),
        wet: count(&|g| g & G_WET != 0),
        walls: m.items.walls.len(),
        to_nest: m.items.moulds.iter().map(|&q| reach(bw, bh, &ds, q, 40.0)).min().unwrap_or(u32::MAX),
        to_first: m.items.moulds.first().map_or(u32::MAX, |&q| reach(bw, bh, &ds, q, 40.0)),
        to_crystal: m.items.crystals.iter().map(|c| reach(bw, bh, &ds, (c.x, c.y), c.r)).min().unwrap_or(u32::MAX),
        unreachable: m.items.crystals.iter().filter(|c| reach(bw, bh, &ds, (c.x, c.y), c.r) == u32::MAX).count(),
        open_share: if total > 0.0 { open / total } else { 0.0 },
        nest_line: m.items.moulds.iter().map(|&q| dist(q, landing)).fold(f32::MAX, f32::min),
    }
}

/// What makes a built planet unfit, in words; empty when it is fit.
pub fn check(p: &Planet, m: &Map) -> Vec<String> {
    let mut out = Vec::new();
    let it = &m.items;
    if it.landing.is_none() || it.moulds.is_empty() || it.crystals.len() < 6 {
        out.push(format!("missing items: {} crystals, {} nests", it.crystals.len(), it.moulds.len()));
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
        out.push(format!("only {:.0}% of the charge is open without breaking soft rock", s.open_share * 100.0));
    }
    // Rock's share, at most: caves are mostly rock, rooms and ridges
    // more than open ground.
    let rc = Recipe::of(p);
    let most = match rc.openness {
        Openness::Caves => 0.72,
        Openness::Rooms | Openness::Ridges => 0.4,
        _ => 0.3,
    };
    if s.rock > most {
        out.push(format!("{:.0}% rock", s.rock * 100.0));
    }
    if it.moulds.len() < rc.nests {
        out.push(format!("{} of {} nests placed", it.moulds.len(), rc.nests));
    }
    out
}

// ---------------------------------------------------------------------
// Names.

fn planet_name(r: &mut Rng) -> String {
    const START: [&str; 18] = ["ve", "or", "tha", "ka", "mu", "dre", "zo", "ya", "phe", "ul", "ne", "so", "gri", "a", "ix", "lo", "qua", "ber"];
    const MID: [&str; 12] = ["sk", "r", "l", "n", "th", "v", "m", "rr", "z", "d", "ss", "k"];
    const END: [&str; 14] = ["a", "is", "on", "ix", "us", "e", "ar", "o", "en", "ith", "um", "or", "ae", "ek"];
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

    /// FNV-1a over the ground, the plants and the item text: a planet's
    /// fingerprint.
    fn print(m: &Map) -> u64 {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for b in m.ground.iter().copied().chain(m.plants.iter().copied()).chain(m.items.text().into_bytes()) {
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

    // Round 3 (dials and the new biome order, 2026-11-14), then plants in
    // the fingerprint, deep water, bridges and fords, and planets 4-8 at
    // up to 2x (`GROW_CAP`). Round 2's depths 3-8 had kept their pins
    // through the tiled region scan, which finds the same nearest sites as
    // the full scan.
    const PINNED: [u64; 8] = [0xcd9eb867fac3322f, 0x994ec126bc931eba, 0xbc5df2aaa15113e2, 0xdfaa92ef45d1a174, 0xb9644a3f74b4e46f, 0x152db0909725a507, 0xb8f53c4c69603249, 0xfce5fb42e242613e];

    /// The core world at the owner's 6x area, pinned too: the bigger maps
    /// run more of the generator's loops, on every machine the same.
    #[test]
    fn a_big_planet_is_pinned() {
        let (_, m) = spec("planet:8:1:6", 1).unwrap().unwrap();
        assert_eq!(print(&m), PINNED_BIG, "{:#x}", print(&m));
    }

    const PINNED_BIG: u64 = 0x034bef6b33578452;

    #[test]
    fn biomes_come_in_the_set_order_with_the_core_world_last() {
        let got: Vec<Biome> = (1..=9).map(|d| planet(1, d).biome).collect();
        assert_eq!(got, [Biome::Meadow, Biome::Barrens, Biome::Fen, Biome::Valley, Biome::Jungle, Biome::Highlands, Biome::Caves, Biome::Core, Biome::Meadow]);
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
                let k = k.unwrap_or_else(|| panic!("seed {seed} depth {depth} ({}): {:?}", p.biome.name(), check(&p, &build(&p, 0))));
                attempts += k + 1;
                let m = generate(&p);
                assert_eq!(print(&m), print(&build(&p, k)));
                let s = stats(&p, &m);
                // A jackpot or oases planet has few crystals by design, and
                // planet 1 is 60% of today's area: `check`'s six at least.
                assert!(s.crystals >= 6 && s.unreachable == 0, "seed {seed} depth {depth}: {s:?}");
            }
        }
        // Most planets pass on their first or second attempt.
        assert!(attempts <= 54 * 3, "{attempts} attempts for 54 planets");
    }

    /// Planets of every biome pass their checks at the owner's sizes, the
    /// core world's 6x area included, not only at today's.
    #[test]
    fn bigger_planets_pass_their_checks() {
        for depth in 1..=8 {
            for area in [0.6, 2.0, 6.0] {
                let p = Planet { size: size_of(area), ..planet(3, depth) };
                let k = (0..ATTEMPTS).find(|&a| check(&p, &build(&p, a)).is_empty());
                assert!(k.is_some(), "depth {depth} area {area}: {:?}", check(&p, &build(&p, 0)));
                let s = stats(&p, &generate(&p));
                assert!(s.unreachable == 0 && s.crystals >= 6, "depth {depth} area {area}: {s:?}");
            }
        }
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
                assert!(it.walls.iter().all(|wl| crate::mould::map_edit::segment_distance(l, wl.a, wl.b) > 60.0));
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
        assert!(!m.items.walls.is_empty() && d.wall.iter().any(|&w| w), "the thicket's soft rock stands");
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
        assert!(spec("planet:1:2:3:4", 1).unwrap().is_err());
        assert!(spec("planet:1:2:9", 1).unwrap().is_err());
        assert_eq!(spec_planet("planet:3:9:2", 1).unwrap().unwrap().size, size_of(2.0));
        assert_eq!(spec_planet("planet:3:9", 1).unwrap().unwrap(), planet(9, 3));
    }

    #[test]
    fn names_read_as_names() {
        let names: Vec<String> = (1..=40).map(|s| planet(s, 1).name).collect();
        assert!(names.iter().all(|n| (3..=12).contains(&n.len()) && n.chars().next().unwrap().is_uppercase()), "{names:?}");
        let distinct: std::collections::BTreeSet<&String> = names.iter().collect();
        assert!(distinct.len() >= 36, "{} distinct of 40", distinct.len());
    }
}
