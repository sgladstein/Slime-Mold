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

/// How near a river's water may run to a nest, in map cells from the nest:
/// outside its bowl (up to 192 out) with room for the road round it.
pub const RIVER_OFF: f32 = 220.0;

/// How near a river's fords and bridge may sit to a nest, where the river
/// has room (map cells from the nest).
pub const CROSSING_OFF: f32 = 320.0;

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
    pub const ALL: [Biome; 8] = [
        Biome::Meadow,
        Biome::Barrens,
        Biome::Fen,
        Biome::Valley,
        Biome::Jungle,
        Biome::Highlands,
        Biome::Caves,
        Biome::Core,
    ];

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

    /// The ground's moisture and how much it varies (`Items::moisture`, the
    /// map's `moisture BASE SPREAD` line; `plants::moisture`), agreed with
    /// the plants thread on 11-14: damp fens and jungle burn slowly and go
    /// out, the dry barrens and highlands carry fire. A map without the
    /// line plays at the engine's 0.15 and 0.3.
    pub fn moisture(self) -> (f32, f32) {
        match self {
            Biome::Meadow => (0.25, 0.3),
            Biome::Barrens => (0.0, 0.2),
            Biome::Fen => (0.5, 0.3),
            Biome::Valley => (0.3, 0.3),
            Biome::Jungle => (0.4, 0.3),
            Biome::Highlands => (0.1, 0.2),
            Biome::Caves => (0.35, 0.2),
            Biome::Core => (0.2, 0.35),
        }
    }

    /// The set order (owner, 2026-11-14 15:33Z "Set order"; 17:02Z, open
    /// and readable early, mazes and caves later, "That makes sense"), the
    /// core world last. Round 2's biomes (barrens, marsh, canyon, dunes,
    /// claypan, thicket, spires) are dials of these now: their shapes are
    /// kept, as `Openness`, `Water` and `Soil` values.
    pub const ORDER: [Biome; 7] = [
        Biome::Meadow,
        Biome::Barrens,
        Biome::Fen,
        Biome::Valley,
        Biome::Jungle,
        Biome::Highlands,
        Biome::Caves,
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
    /// Map cells (`size_at`): 60% of the designed maps' area on planet 1,
    /// six times it on the core world.
    pub size: (usize, usize),
}

/// Each planet's map area as a share of the designed maps' (1024 x 640):
/// the owner's "first planet about 60% of today's map, today's size by
/// planet 3 or 4, up to 6 the area for the core world" (2026-11-14 15:33Z).
/// Past the core world, the core world's. `GROW_CAP` caps it.
pub const AREA: [f32; 8] = [0.6, 0.8, 1.0, 1.4, 2.0, 2.8, 4.0, 6.0];

/// The biggest share `size_at` gives. The technical lead (2026-11-14
/// 16:42Z): "Before the default changes, book one 1070 timing ... at 1x,
/// 2x, 4x and 6x area". Timed 19:11Z (`raw/sizes-1070/`): the GPU step
/// 0.63, 0.97, 1.50 and 2.11 ms and frame p95 at most 5.2 ms at every
/// size, but the window's ground refresh (`refresh_ground`, once a game
/// second: a charge read-back and every cell's code on the CPU) peaks at
/// 4.6, 8.4, 23.6 and 32.9 ms, a dropped frame or two every second from
/// 4x. So 2x (planets 5-8) until that refresh touched only the cells that
/// change. Then the refresh gathers only crystal and wall cells (`Gather`,
/// #302), walking maps are built a slice a step (#306: the rush hand's step
/// max 1.72 ms at 6x on the 1070) and the gas press reads back only its
/// words (#307), so the cap is the owner's 6x. `planet:D:S:AREA` asks for
/// any share up to `AREA_MAX`.
pub const GROW_CAP: f32 = 6.0;

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
    Planet {
        seed,
        depth,
        name,
        biome,
        size: size_at(depth),
    }
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
            return Err(format!(
                "{s}: planet, planet:DEPTH, planet:DEPTH:SEED or planet:DEPTH:SEED:AREA"
            ));
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
                    return Err(format!(
                        "{s}: area {area} (a share of today's map) runs from 0.25 to {AREA_MAX}"
                    ));
                }
                Ok(Planet {
                    size: size_of(area),
                    ..p
                })
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
    fallback(p)
}

/// The planet when every attempt fails a check (none has, over 1,600
/// planets): an open plan that reaches everything, the biome's water and
/// soil kept, tried up to `FALLBACKS` times for one that passes (the
/// nests' places are still dice: 2 of 32 first tries put the far nest
/// too far), else the last. At first it was one try that kept the
/// biome's openness, fronts, pillars, hedges and rings: 6 of 96 forced
/// fallbacks failed their own checks (planet 8 seed 11 with 46% of its
/// charge open) and a cave's was 54% rock (technical review, 2026-11-15).
pub fn fallback(p: &Planet) -> Map {
    let rc = Recipe::of(p);
    let open = Recipe {
        openness: Openness::Open,
        open: 1.0,
        front: 0.0,
        fronts: 0,
        gate: 0.0,
        outcrop: 2.0,
        pillars: 0,
        ring: 0.0,
        hedges: 0,
        ..rc
    };
    let mut m = build_with(p, ATTEMPTS, Some(open));
    for k in 1..FALLBACKS {
        if check(p, &m).is_empty() {
            break;
        }
        m = build_with(p, ATTEMPTS + k, Some(open));
    }
    m
}

/// Tries of the open plan before the fallback keeps one that fails.
const FALLBACKS: u32 = 4;

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
type Odds = (
    &'static [(Openness, u32)],
    &'static [(Water, u32)],
    &'static [(Soil, u32)],
    &'static [(Pattern, u32)],
);

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
    /// Open arenas (the core world): field regions with every border to
    /// their neighbours open and no outcrops, fronts or hedges in them.
    pub arenas: usize,
    /// A moat of deep water round each nest's rock rim, the rock a thinner
    /// lip inside it, the door a bridge across the moat. Off on every
    /// biome: made for the fens (the planet reviews could name only 4 of 8
    /// biomes at zoom 12, and every bowl's rim was the same grey rock), but
    /// in game the slime covers most of the moat by 30 s and it then reads
    /// as little more than a thinner rim (2026-11-15). Up to the owner, as
    /// a blind A/B (`Reports/levelgen-fen-moats-2026-11-15/`).
    pub moat: bool,
}

/// Open arenas on the core world (`Recipe::arenas`), on the 1024 x 640
/// map and more on a bigger one at the same density: three, so the
/// busiest world still has a few grounds to fight on in the open.
const ARENAS: usize = 3;
/// The least distance between two arenas' sites, map cells.
const ARENA_APART: f32 = 190.0;

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
            Biome::Meadow => (
                &[(O::Open, 3), (O::Outcrops, 2), (O::Pillars, 1)],
                &[(W::Pools, 2), (W::Dry, 1), (W::Streams, 1)],
                &[(S::Plain, 2), (S::Sand, 1), (S::Clay, 1)],
                &[
                    (P::Scattered, 3),
                    (P::Fields, 3),
                    (P::Richer, 2),
                    (P::Veins, 1),
                    (P::Jackpot, 1),
                ],
            ),
            Biome::Barrens => (
                &[(O::Open, 2), (O::Outcrops, 2), (O::Pillars, 1)],
                &[(W::Dry, 1)],
                &[(S::Bands, 1), (S::Oases, 1)],
                &[(P::Scattered, 2), (P::Veins, 2), (P::Richer, 2), (P::Jackpot, 1)],
            ),
            // Lakes 3 to 1: at 2 to 2, 6 of 8 fens in the planet reviews
            // rolled only pools, which a squad walks through, and played
            // like the barrens (2026-11-15).
            Biome::Fen => (
                &[(O::Open, 2), (O::Outcrops, 2)],
                &[(W::Lakes, 3), (W::Pools, 1)],
                &[(S::Clay, 2), (S::Plain, 1)],
                &[(P::Veins, 3), (P::Fields, 2), (P::Scattered, 2), (P::Richer, 1)],
            ),
            Biome::Valley => (
                &[(O::Ridges, 2), (O::Outcrops, 1)],
                &[(W::Rivers, 1)],
                &[(S::Dunes, 2), (S::Sand, 1)],
                &[(P::Fields, 3), (P::Veins, 2), (P::Richer, 2), (P::Scattered, 1)],
            ),
            Biome::Jungle => (
                &[(O::Outcrops, 2), (O::Open, 1)],
                &[(W::Streams, 2), (W::Pools, 1)],
                &[(S::Clay, 2), (S::Plain, 1)],
                &[(P::Scattered, 3), (P::Fields, 2), (P::Jackpot, 1), (P::Richer, 1)],
            ),
            Biome::Highlands => (
                &[(O::Rooms, 3), (O::Ridges, 1)],
                &[(W::Dry, 2), (W::Pools, 1)],
                &[(S::Cracks, 2), (S::Clay, 1)],
                &[(P::Fields, 3), (P::Jackpot, 2), (P::Richer, 2), (P::Scattered, 1)],
            ),
            Biome::Caves => (
                &[(O::Caves, 1)],
                &[(W::Pools, 2), (W::Dry, 1)],
                &[(S::Plain, 1), (S::Clay, 1), (S::Sand, 1)],
                &[(P::Veins, 3), (P::Jackpot, 2), (P::Scattered, 2), (P::Fields, 1)],
            ),
            Biome::Core => (
                &[(O::Outcrops, 1), (O::Pillars, 1), (O::Ridges, 1), (O::Rooms, 1)],
                &[
                    (W::Pools, 1),
                    (W::Lakes, 1),
                    (W::Rivers, 1),
                    (W::Streams, 1),
                    (W::Dry, 1),
                ],
                &[(S::Mixed, 1)],
                &[
                    (P::Scattered, 1),
                    (P::Fields, 1),
                    (P::Veins, 1),
                    (P::Jackpot, 1),
                    (P::Richer, 1),
                ],
            ),
        };
        let openness = pick(&mut r, open);
        let water = pick(&mut r, water);
        let soil = pick(&mut r, soil);
        // Oases soil keeps its crystals in the oases.
        let pattern = if soil == S::Oases {
            P::Oases
        } else {
            pick(&mut r, pattern)
        };
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
            arenas: 0,
            rivers: 0,
            moat: false,
        };
        // Openness: the rock.
        let rc = match openness {
            O::Open => Recipe {
                open: 0.9,
                front: 0.25,
                ridge: 11.0,
                outcrop: 2.0,
                ..base
            },
            O::Outcrops => Recipe {
                front: 0.45,
                outcrop: 0.72,
                ..base
            },
            O::Pillars => Recipe {
                open: 0.85,
                front: 0.3,
                ridge: 11.0,
                outcrop: 0.78,
                pillars: 210,
                ..base
            },
            // Two long ridges across the route (three past the first round),
            // whole and wide, between fewer, bigger regions; no mesas, which
            // read as debris (graphics lead, 2026-11-14: round 1's canyon
            // rock was "short stubs").
            O::Ridges => Recipe {
                regions: 9,
                open: 0.55,
                front: 0.0,
                loops: 0.35,
                gate: 0.25,
                ridge: 18.0,
                ridge_vary: 0.5,
                fronts: 2 + (p.depth > CORE_DEPTH) as usize,
                pass: (22.0, 32.0),
                outcrop: 2.0,
                ..base
            },
            // Walls whole and wide, few loops, small corners shut: rooms.
            O::Rooms => Recipe {
                regions: 12,
                open: 0.08,
                front: 0.95,
                loops: 0.2,
                gate: 0.08,
                ridge: 21.0,
                ridge_vary: 0.4,
                pass: (20.0, 28.0),
                outcrop: 2.0,
                ..base
            },
            // Every border shut; the spanning tree and the loops are tunnels.
            O::Caves => Recipe {
                regions: 13,
                open: 0.0,
                front: 1.0,
                loops: 0.3,
                gate: 0.2,
                outcrop: 2.0,
                arenas: 1,
                ..base
            },
        };
        let rc = match water {
            W::Rivers => Recipe {
                rivers: 1 + r.chance(0.4) as usize,
                ..rc
            },
            W::Streams => Recipe {
                rivers: 2 + r.below(3),
                ..rc
            },
            _ => rc,
        };
        // The biome's own extras.
        let rc = match p.biome {
            Biome::Jungle => Recipe {
                gate: 0.6,
                ring: 1.0,
                hedges: 9,
                ..rc
            },
            Biome::Meadow => Recipe {
                hedges: r.below(3),
                ..rc
            },
            // Fewer gates in rooms, where every pass is a tree's: a gate
            // there shuts all the rooms behind it (rooms with gates at
            // 0.35 left under half the charge open on 8 of 50 core worlds).
            // A few big open arenas, and a hedge round each nest unless the
            // rooms wall it (graphics lead, 11-14: the core world was too
            // busy to read the way to each nest at zoom 1).
            Biome::Core => Recipe {
                regions: rc.regions + 4,
                gate: if openness == O::Rooms { 0.1 } else { 0.35 },
                ring: if openness == O::Rooms { 0.0 } else { 1.0 },
                hedges: 5,
                nests: nests.max(3),
                small: 14,
                big: 3,
                arenas: ARENAS,
                ..rc
            },
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
        // Streams keep their count on a bigger map and rivers stop at two:
        // grown with the route's length, a 6x core world had 5-10 streams
        // webbed over the whole map and up to four parallel rivers, the
        // same obstacle four times (looks and play reviews, 2026-11-15).
        let rivers = if water == W::Streams {
            rc.rivers
        } else {
            more(rc.rivers, area.sqrt()).min(2)
        };
        Recipe {
            pass: (rc.pass.0 * narrow, rc.pass.1 * narrow),
            regions: more(rc.regions, area),
            small: more(rc.small, area),
            big: more(rc.big, area.sqrt()),
            hedges: more(rc.hedges, area),
            arenas: more(rc.arenas, area),
            rivers,
            ..rc
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
    /// On the spanning tree (always passable).
    tree: bool,
    /// Border cells (map cell indices).
    cells: Vec<u32>,
    pass: Option<((f32, f32), f32)>,
    /// A cave tunnel's way, every 8 map cells. Empty off the caves.
    way: Vec<WayPoint>,
}

/// A point on a cave tunnel's way: the point, the way's direction there and
/// the tunnel's half-width.
type WayPoint = ((f32, f32), (f32, f32), f32);

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
        walls.push(Wall {
            a: (s[0].0.round(), s[0].1.round()),
            b: (s[1].0.round(), s[1].1.round()),
            thick,
            hp: HP,
        });
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
            off = if i == 0 || i == k {
                0.0
            } else {
                off * 0.5 + r.range(-wobble, wobble)
            };
            (a.0 + (b.0 - a.0) * t + nrm.0 * off, a.1 + (b.1 - a.1) * t + nrm.1 * off)
        })
        .collect()
}

/// The cheapest way over `ground` from `from` to `to`, in steps of 8 map
/// cells, smoothed: rock costs 6 times plain ground and deep water 3 times,
/// with `noise` (0-1) making it wander; never where `shut` holds (`from`
/// and `to` excepted). `None` if `shut` walls `to` off.
///
/// A nest's road follows the ground this way, not a straight line: cut
/// straight, it ran as a wide boulevard right across the caves and through
/// the highlands' ridges (2026-11-15).
fn route(
    w: usize,
    h: usize,
    ground: &[u8],
    from: (f32, f32),
    to: (f32, f32),
    noise: &Field,
    shut: impl Fn((f32, f32)) -> bool,
) -> Option<Vec<(f32, f32)>> {
    const RB: usize = 8;
    let (bw, bh) = (w / RB, h / RB);
    let bin =
        |p: (f32, f32)| ((p.1.max(0.0) as usize / RB).min(bh - 1)) * bw + (p.0.max(0.0) as usize / RB).min(bw - 1);
    let (s, t) = (bin(from), bin(to));
    // A bin's cost, by its centre and four corners; 0 for shut.
    let cost: Vec<f32> = (0..bw * bh)
        .map(|i| {
            let (x, y) = ((i % bw) * RB, (i / bw) * RB);
            let c = ((x + RB / 2) as f32, (y + RB / 2) as f32);
            if i != s && i != t && shut(c) {
                return 0.0;
            }
            let mut c = 0.0;
            for (dx, dy) in [(RB / 2, RB / 2), (0, 0), (RB - 1, 0), (0, RB - 1), (RB - 1, RB - 1)] {
                let g = ground[(y + dy) * w + x + dx];
                c += if g & G_ROCK != 0 {
                    6.0
                } else if g == DEEP {
                    3.0
                } else {
                    1.0
                };
            }
            c / 5.0 * (0.7 + 0.6 * noise.at(x + RB / 2, y + RB / 2))
        })
        .collect();
    use std::cmp::Reverse;
    let mut best = vec![u32::MAX; bw * bh];
    let mut prev = vec![usize::MAX; bw * bh];
    let mut heap = std::collections::BinaryHeap::new();
    best[s] = 0;
    heap.push(Reverse((0u32, s)));
    while let Some(Reverse((d, i))) = heap.pop() {
        if i == t {
            break;
        }
        if d > best[i] {
            continue;
        }
        let (x, y) = ((i % bw) as i32, (i / bw) as i32);
        for (dx, dy, step) in [
            (1, 0, 10.0),
            (-1, 0, 10.0),
            (0, 1, 10.0),
            (0, -1, 10.0),
            (1, 1, 14.14),
            (1, -1, 14.14),
            (-1, 1, 14.14),
            (-1, -1, 14.14),
        ] {
            let (nx, ny) = (x + dx, y + dy);
            if nx < 0 || ny < 0 || nx >= bw as i32 || ny >= bh as i32 {
                continue;
            }
            let j = ny as usize * bw + nx as usize;
            if cost[j] == 0.0 {
                continue;
            }
            let nd = d + ((cost[i].max(1.0) + cost[j]) * 0.5 * step) as u32;
            if nd < best[j] {
                best[j] = nd;
                prev[j] = i;
                heap.push(Reverse((nd, j)));
            }
        }
    }
    if best[t] == u32::MAX {
        return None;
    }
    let mut bins = vec![t];
    while *bins.last().unwrap() != s {
        bins.push(prev[*bins.last().unwrap()]);
    }
    bins.reverse();
    // A point every 4 bins (32 map cells), the ends exact, then cut corners
    // twice (Chaikin), so the 8 directions of the steps don't show.
    let mut pts: Vec<(f32, f32)> = vec![from];
    pts.extend(
        bins.iter()
            .skip(4)
            .step_by(4)
            .filter(|&&i| i != t)
            .map(|&i| (((i % bw) * RB + RB / 2) as f32, ((i / bw) * RB + RB / 2) as f32)),
    );
    pts.push(to);
    for _ in 0..2 {
        let mut out = vec![pts[0]];
        for sg in pts.windows(2) {
            out.push((sg[0].0 * 0.75 + sg[1].0 * 0.25, sg[0].1 * 0.75 + sg[1].1 * 0.25));
            out.push((sg[0].0 * 0.25 + sg[1].0 * 0.75, sg[0].1 * 0.25 + sg[1].1 * 0.75));
        }
        out.push(*pts.last().unwrap());
        pts = out;
    }
    Some(pts)
}

/// Turning a vector by fixed steps (constants, not `cos`, so every machine
/// turns it the same): 15 degrees.
const C15: f32 = 0.965_925_8;
const S15: f32 = 0.258_819_04;

fn turn15(v: (f32, f32)) -> (f32, f32) {
    (v.0 * C15 - v.1 * S15, v.0 * S15 + v.1 * C15)
}

/// `turn15` one way (`side` 1) or the other (-1).
fn turn15_to(v: (f32, f32), side: f32) -> (f32, f32) {
    (v.0 * C15 - side * v.1 * S15, side * v.0 * S15 + v.1 * C15)
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
    let (xa, xb) = (
        (x0 - reach).max(0.0) as usize,
        ((x1 + reach).max(0.0) as usize + 1).min(w),
    );
    let (ya, yb) = (
        (y0 - reach).max(0.0) as usize,
        ((y1 + reach).max(0.0) as usize + 1).min(h),
    );
    if xa >= xb || ya >= yb {
        return;
    }
    // Each segment marks only the cells within `reach` of its own box, and
    // a cell keeps its least distance: the same distances and calls, in
    // the same order, as testing every segment at every cell of the line's
    // box. That was 3-5 s for a planet's streams at 6x, where a stream
    // wanders right across the map in about a hundred pieces.
    let bw = xb - xa;
    let mut near = vec![f32::MAX; bw * (yb - ya)];
    for s in pts.windows(2) {
        let (sx0, sx1) = (s[0].0.min(s[1].0) - reach, s[0].0.max(s[1].0) + reach);
        let (sy0, sy1) = (s[0].1.min(s[1].1) - reach, s[0].1.max(s[1].1) + reach);
        let (ca, cb) = ((sx0.max(0.0) as usize).max(xa), ((sx1.max(0.0) as usize) + 2).min(xb));
        let (ra, rb) = ((sy0.max(0.0) as usize).max(ya), ((sy1.max(0.0) as usize) + 2).min(yb));
        for y in ra..rb {
            for x in ca..cb {
                let d = seg_dist((x as f32 + 0.5, y as f32 + 0.5), s[0], s[1]);
                let k = (y - ya) * bw + (x - xa);
                near[k] = near[k].min(d);
            }
        }
    }
    for y in ya..yb {
        for x in xa..xb {
            let d = near[(y - ya) * bw + (x - xa)];
            if d < reach {
                f(y * w + x, (x as f32 + 0.5, y as f32 + 0.5), d);
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

/// The dunes' clay islands under the crystals, as on the Flats: big and
/// ragged, since exact circles read as decals or holes (graphics lead,
/// 2026-11-14). Not on rock, deep water or a bridge: their kind is the
/// whole soil nibble, and clay over deep water (0x41 to 0x21) made wet clay
/// a soldier can wade, an unplanned ford `cut_fords` never saw (bug check,
/// 2026-11-15: 26 of 32 dune planets over seeds 1-40).
fn clay_islands(ground: &mut [u8], w: usize, h: usize, crystals: &[Crystal], soil_b: &Field, fine: &Field) {
    for c in crystals {
        let m = c.r * 2.4;
        for y in ((c.y - m * 1.4).max(0.0) as usize)..((c.y + m * 1.4) as usize).min(h) {
            for x in ((c.x - m * 1.4).max(0.0) as usize)..((c.x + m * 1.4) as usize).min(w) {
                let d = dist((x as f32 + 0.5, y as f32 + 0.5), (c.x, c.y)) / m
                    + (soil_b.at(x, y) - 0.5) * 0.6
                    + (fine.at(x, y) - 0.5) * 1.2;
                let i = y * w + x;
                if d < 1.0 && ground[i] & G_ROCK == 0 && ground[i] != G_DEEP && ground[i] != G_BRIDGE {
                    ground[i] = (ground[i] & 0x0F) | G_CLAY;
                }
            }
        }
    }
}

/// How a planet opens: the crystals by the landing, by biome. Every planet
/// opened the same way before, one crystal 85-140 map cells off toward the
/// middle (play review, 2026-11-15: "the first crystal is 12-26 cells from
/// the landing on 64 of 64 planets"; Age of Empires II gives each map a
/// start built for it). The meadow keeps that start.
struct Opening {
    /// Crystals placed by the landing.
    count: usize,
    /// How far off, map cells, first toward the middle (or the slime).
    near: (f32, f32),
    /// The farthest, any way round, when `near` has no room.
    far: f32,
    /// Radius, map cells.
    size: (f32, f32),
    charges: &'static [f32],
    /// Toward the nearest nest, not the map's middle.
    toward_slime: bool,
}

impl Opening {
    fn of(b: Biome) -> Opening {
        let base = Opening {
            count: 1,
            near: (85.0, 140.0),
            far: 220.0,
            size: (24.0, 28.0),
            charges: &[192.0, 256.0],
            toward_slime: false,
        };
        match b {
            // A gentle start, as on every planet before.
            Biome::Meadow => base,
            // Lean: one small crystal a walk off.
            Biome::Barrens => Opening {
                near: (140.0, 185.0),
                far: 220.0,
                size: (18.0, 22.0),
                charges: &[128.0, 192.0],
                ..base
            },
            // Two middling crystals, a squad's choice of bank.
            Biome::Fen => Opening {
                count: 2,
                near: (100.0, 160.0),
                size: (20.0, 24.0),
                charges: &[192.0],
                ..base
            },
            // One big crystal across the valley floor.
            Biome::Valley => Opening {
                near: (120.0, 180.0),
                far: 240.0,
                size: (28.0, 32.0),
                charges: &[256.0, 384.0],
                ..base
            },
            // Three small ones close by in the undergrowth.
            Biome::Jungle => Opening {
                count: 3,
                near: (60.0, 110.0),
                far: 160.0,
                size: (15.0, 18.0),
                charges: &[128.0],
                ..base
            },
            // One rich crystal, a climb away.
            Biome::Highlands => Opening {
                near: (150.0, 195.0),
                far: 230.0,
                size: (28.0, 31.0),
                charges: &[384.0],
                ..base
            },
            // Two small ones in the landing's own cavern.
            Biome::Caves => Opening {
                count: 2,
                near: (50.0, 95.0),
                far: 140.0,
                size: (18.0, 22.0),
                charges: &[192.0],
                ..base
            },
            // A rich crystal toward the slime: fought over from the start.
            Biome::Core => Opening {
                near: (180.0, 230.0),
                far: 260.0,
                size: (34.0, 38.0),
                charges: &[512.0],
                toward_slime: true,
                ..base
            },
        }
    }
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
    // Off the edges by a share of the map, at least 150 map cells, so the
    // slime's start (a blob of radius 130-190 map cells at the default
    // walkers) isn't cut by the frame: at 110 and 100 map cells, 6 of 16
    // first nests sat on an edge (looks review, 2026-11-15). Apart by 0.3
    // of the map's width (470 cells at least, below), not a fixed 280: at
    // 6x the nearest pair was 0.14-0.19 of the width on 5 of 8 core
    // worlds, and seed 1's three nests shared 7% of the map (play review).
    // The spacing gives way a step at a time where the far part has no
    // room (on a map below today's area, down to 400 cells, below which
    // the bowls would meet).
    let (mx, my) = ((0.16 * wf).clamp(150.0, 240.0), (0.2 * hf).clamp(150.0, 240.0));
    let mut nests: Vec<(f32, f32)> = Vec::new();
    let mut first = (far.0.clamp(mx, wf - mx), far.1.clamp(my, hf - my));
    for _ in 0..40 {
        let q = (far.0 + r.range(-0.15, 0.15) * wf, far.1 + r.range(-0.3, 0.3) * hf);
        let q = (q.0.clamp(mx, wf - mx), q.1.clamp(my, hf - my));
        if dist(q, landing) > 0.6 * wf {
            first = q;
            break;
        }
    }
    nests.push(first);
    // At least 470 map cells: two bowls and the road round one fit
    // between (`Bowl`; at 364, 0.3 of planet 4's width, a nest by an edge
    // had no side for its road on 8 of 64 planets).
    let apart = (0.3 * wf).max(470.0);
    for _ in 1..rc.nests {
        let mut best: Option<((f32, f32), f32)> = None;
        for t in 0..400 {
            let q = (r.range(mx, wf - mx), r.range(my, hf - my));
            if dist(q, landing) <= 0.5 * wf {
                continue;
            }
            let near = nests.iter().map(|&n| dist(n, q)).fold(f32::INFINITY, f32::min);
            if near > apart * (1.0 - 0.1 * (t / 50) as f32) {
                best = Some((q, near));
                break;
            }
            if best.is_none_or(|b| near > b.1) {
                best = Some((q, near));
            }
        }
        // Below today's area, not where two bowls would run into each
        // other (each reaches up to 192 cells out): a map that small, such
        // as a cave asked for at 0.6 of today's area, gets fewer nests. At
        // 175 apart there, the two bowls made one cavern with no way round
        // (every attempt failed `check`). Not on the bigger maps the run
        // plays: dropping a nest under 400 apart there failed 7 of 20 river
        // valleys' first tries, which pass `check` with the nests closer.
        if let Some((q, _)) = best.filter(|b| b.1 >= 400.0 || wf * hf >= (MAP_W * MAP_H) as f32) {
            nests.push(q);
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
                            tree: false,
                            cells: Vec::new(),
                            pass: None,
                            way: Vec::new(),
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

    // Arenas (`Recipe::arenas`): field regions away from the nests and the
    // landing, far apart, each cleared of outcrops, fronts, pillars and
    // hedges, with every border to a neighbour open (none to a nest's
    // region, whose ridges and ring stay), so the arena opens on all
    // sides. Opening borders only joins more ground, so every check that
    // held still holds. Clearing the neighbours too was tried first: with
    // three arenas on a 15-region world nearly every pillar went
    // (seed 3, 11-14), and the world was open everywhere, not in places.
    let mut arena = vec![false; ns];
    if rc.arenas > 0 {
        let mut cand: Vec<usize> = (1 + nests.len()..ns)
            .filter(|&k| nests.iter().all(|&q| dist(sites[k], q) > 260.0) && dist(sites[k], landing) > 160.0)
            .collect();
        for i in (1..cand.len()).rev() {
            cand.swap(i, r.below(i + 1));
        }
        let apart = ARENA_APART;
        let mut hubs: Vec<usize> = Vec::new();
        for k in cand {
            if hubs.len() < rc.arenas && hubs.iter().all(|&h| dist(sites[h], sites[k]) > apart) {
                hubs.push(k);
            }
        }
        for &hub in &hubs {
            arena[hub] = true;
            for e in edges.iter_mut() {
                let other = if e.a == hub {
                    e.b
                } else if e.b == hub {
                    e.a
                } else {
                    continue;
                };
                if (1..=nests.len()).contains(&other) {
                    continue;
                }
                e.kind = Border::Open;
            }
        }
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
        let room: Vec<f32> = (-300..300)
            .map(|i| i as f32 * 8.0)
            .filter(|&u| {
                let q = at_line(s, u);
                q.0 > rad + 40.0 && q.1 > rad + 40.0 && q.0 < wf - rad - 40.0 && q.1 < hf - rad - 40.0
            })
            .collect();
        if room.is_empty() {
            continue;
        }
        let mut passes = vec![(room[r.below(room.len())], rad)];
        if r.chance(0.5) {
            let rad2 = r.range(rc.pass.0, rc.pass.1);
            let far: Vec<f32> = room
                .iter()
                .copied()
                .filter(|&u| (u - passes[0].0).abs() > 200.0)
                .collect();
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
        // Each cavern lobed: its radius wanders with the angle by three
        // harmonics (2, 3 and 5 lobes) at their own phases, by up to about
        // a third. Round caverns on even tubes read as a node graph (looks
        // review, 2026-11-15: rooms as discs at 0.6 roughness); the rough
        // noise alone wobbled the rim but kept the disc. The landing's and
        // the nests' caverns only bulge, so they keep their room.
        // Arenas (`Recipe::arenas`) are galleries nearly as wide as the
        // regions' spacing, a room to fight in on a route of tunnels (play
        // review: "the caves are a slog at 4x", 2026-11-15).
        for (k, &c) in sites.iter().enumerate() {
            let rad = if k == 0 {
                130.0
            } else if k <= nests.len() {
                115.0
            } else if arena[k] {
                spacing * r.range(0.85, 0.95)
            } else {
                spacing * r.range(0.32, 0.5)
            };
            let lobes =
                [(2.0, 0.16), (3.0, 0.12), (5.0, 0.07)].map(|(n, a)| (n, a, r.range(0.0, std::f32::consts::TAU)));
            let home = k <= nests.len();
            // The rim's radius by angle, in 256 steps round.
            let shape: Vec<f32> = (0..256)
                .map(|i| {
                    let ang = i as f32 / 256.0 * std::f32::consts::TAU;
                    let lobe: f32 = lobes.iter().map(|&(n, a, ph)| a * (n * ang + ph).cos()).sum();
                    rad * (1.0 + if home { lobe.max(0.0) } else { lobe })
                })
                .collect();
            let (lo, hi) = shape
                .iter()
                .fold((f32::MAX, 0.0f32), |(lo, hi), &s| (lo.min(s), hi.max(s)));
            // The rough noise moves the rim by up to 0.3 of its radius.
            let m = hi * 1.4;
            for y in ((c.1 - m).max(0.0) as usize)..((c.1 + m).max(0.0) as usize).min(h) {
                for x in ((c.0 - m).max(0.0) as usize)..((c.0 + m).max(0.0) as usize).min(w) {
                    let (dx, dy) = (x as f32 + 0.5 - c.0, y as f32 + 0.5 - c.1);
                    let d = len(dx, dy);
                    if d >= hi * 1.4 {
                        continue;
                    }
                    let edge = if d < lo * 0.7 {
                        lo
                    } else {
                        shape[((dy.atan2(dx) / std::f32::consts::TAU + 1.0) * 256.0) as usize % 256]
                    };
                    if d / edge + (rough.at(x, y) - 0.5) * 0.6 < 1.0 {
                        cave_open[y * w + x] = true;
                    }
                }
            }
        }
        // Each tunnel's width follows a smooth noise along its way, from
        // 0.55 to 1.65 of its own, never under 8.5 map cells either side
        // (8 to about 25 dish cells on the caves' planet): pinches a squad
        // files through and galleries it spreads out in. Every tunnel was
        // 12-18 wide all along before (20 cave planets,
        // `Reports/levelgen-caves-2026-11-15/raw/widths.py`). A floor of 6
        // (0.4 to 1.65) cut crystals off on 17 of 20 first tries and 5 fell
        // back to the open plan: `walkable` narrows by a bin, so a pinch
        // under about 8 dish cells is shut to a squad.
        let girth = Field::new(w, h, |x, y| {
            ((nb.fbm(x + 517.0, y - 2203.0, 55.0) - 0.5) * 3.0 + 0.5).clamp(0.0, 1.0)
        });
        let wide =
            |tw: f32, x: usize, y: usize| (tw * (0.55 + 1.1 * girth.at(x, y)) * (0.85 + 0.3 * rough.at(x, y))).max(8.5);
        for e in edges.iter_mut() {
            if e.kind == Border::Ridge {
                continue;
            }
            let (a, b) = (sites[e.a], sites[e.b]);
            let pieces = ((dist(a, b) / 60.0).floor() as usize).max(2);
            let pts = wobbly(&mut r, a, b, pieces, 25.0);
            let tw = r.range(rc.pass.0, rc.pass.1) * 0.6;
            along_line(w, h, &pts, tw * 1.65 * 1.15, |i, _, d| {
                let (x, y) = (i % w, i / w);
                if d < wide(tw, x, y) {
                    cave_open[i] = true;
                }
            });
            let mid = pts[pts.len() / 2];
            let (mx, my) = (
                (mid.0.max(0.0) as usize).min(w - 1),
                (mid.1.max(0.0) as usize).min(h - 1),
            );
            e.pass = Some((mid, wide(tw, mx, my)));
            for s in pts.windows(2) {
                let (d, dir) = (dist(s[0], s[1]), unit(s[1].0 - s[0].0, s[1].1 - s[0].1));
                let mut t = 0.0;
                while t < d {
                    let q = (s[0].0 + dir.0 * t, s[0].1 + dir.1 * t);
                    let (x, y) = ((q.0.max(0.0) as usize).min(w - 1), (q.1.max(0.0) as usize).min(h - 1));
                    e.way.push((q, dir, wide(tw, x, y)));
                    t += 8.0;
                }
            }
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
                rock = fronts.iter().any(|f| {
                    (s - f.s).abs() < f.half * wide && !f.passes.iter().any(|&(pu, rad)| (u - pu).abs() < rad)
                }) && !nests.iter().chain([&landing]).any(|&q| dist(q, at) < 110.0)
                    && !arena[region[i] as usize];
            }
            if !rock && border[i] > 30.0 && blobs.at(x, y) > rc.outcrop && !arena[region[i] as usize] {
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
            let (Some((c, rad)), Border::Pass | Border::Gate | Border::Open) = (e.pass, e.kind) else {
                continue;
            };
            let (t, nrm) = frames[k];
            let m = rad.max(disc) + 6.0;
            for y in ((c.1 - m).max(0.0) as usize)..((c.1 + m).max(0.0) as usize).min(h) {
                for x in ((c.0 - m).max(0.0) as usize)..((c.0 + m).max(0.0) as usize).min(w) {
                    let (dx, dy) = (x as f32 + 0.5 - c.0, y as f32 + 0.5 - c.1);
                    if ((dx * t.0 + dy * t.1).abs() < rad && (dx * nrm.0 + dy * nrm.1).abs() < widest + 4.0)
                        || dx * dx + dy * dy < disc * disc
                    {
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
            if dl < 200.0
                || dn < 160.0
                || ground[i] & G_ROCK != 0
                || oases.iter().any(|&(o, _)| dist(o, c) < 170.0)
                || !(0.2..=0.85).contains(&(dl / (dl + dn)))
            {
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
        // Each nest's surroundings in the rivers' coordinate (`along` plus
        // the warp, as the river is drawn below): points every 6 cells
        // within `RIVER_OFF` + 100 of it, with their distance to the nest.
        let ring: Vec<Vec<(f32, f32)>> = nests
            .iter()
            .map(|&q| {
                let reach = RIVER_OFF + 100.0;
                let mut pts = Vec::new();
                let mut y = (q.1 - reach).max(0.0);
                while y < (q.1 + reach).min(hf - 1.0) {
                    let mut x = (q.0 - reach).max(0.0);
                    while x < (q.0 + reach).min(wf - 1.0) {
                        let dd = dist((x, y), q);
                        if dd < reach {
                            pts.push((along((x, y)) + warp_x.at(x as usize, y as usize) * 1.6, dd));
                        }
                        x += 6.0;
                    }
                    y += 6.0;
                }
                pts
            })
            .collect();
        // How near a river at `s` with half-width `half` runs to any nest:
        // its widest reach is `half * 1.1` (the rough banks).
        let off_nests = |s: f32, half: f32| {
            ring.iter()
                .flatten()
                .filter(|&&(sa, _)| (sa - s).abs() < half * 1.1)
                .map(|&(_, dd)| dd)
                .fold(f32::MAX, f32::min)
        };
        let mut placed: Vec<f32> = Vec::new();
        for j in 0..k {
            let want = span * (0.5 + (j as f32 - (k as f32 - 1.0) / 2.0) * 0.18 + r.range(-0.03, 0.03));
            // At least 40 map cells across (10-14 of the 256 x 160 grid the
            // graphics lead counts in, 4 map cells each): at 34 the river
            // read as a thin line at zoom 1 (graphics lead, 2026-11-14 18:56Z).
            let half = r.range(22.0, 28.0);
            // Off the nests (the three reviews' map item, 2026-11-15): a
            // river through a nest's bowl is cleared within 100 cells of
            // the nest, so the nest itself became the crossing, and on 19
            // of 40 river-valley nests river water ran within 220 cells
            // (valley seed 2's near nest sat by the bridge: the slime held
            // it from the start). So the river moves along the route, the
            // least it must, to keep `RIVER_OFF` from every nest and 130
            // from the other river; its fords and bridge, on it, keep off
            // too. The places tried run from a quarter of the way to the
            // first nest (and 230 cells, past the landing's dry ground) to
            // nine tenths. Where none clears them, a second river is left
            // out: with the second nest partway along the route there is
            // seldom room for two (at first the clearest place was taken,
            // and on 4 of 20 valleys that ran the second river between the
            // two nests, through both bowls). A first river takes the
            // clearest place.
            let options: Vec<f32> = (0..=80)
                .map(|i| (span * (0.25 + 0.65 * i as f32 / 80.0)).max(230.0))
                .filter(|&c| placed.iter().all(|&p| (p - c).abs() > 130.0))
                .collect();
            let s = if off_nests(want, half) >= RIVER_OFF {
                want
            } else if let Some(c) = options
                .iter()
                .copied()
                .filter(|&c| off_nests(c, half) >= RIVER_OFF)
                .min_by(|a, b| (a - want).abs().total_cmp(&(b - want).abs()))
            {
                c
            } else if placed.is_empty() {
                options
                    .iter()
                    .copied()
                    .max_by(|a, b| off_nests(*a, half).total_cmp(&off_nests(*b, half)))
                    .unwrap_or(want)
            } else {
                continue;
            };
            placed.push(s);
            let room: Vec<f32> = (-300..300)
                .map(|i| i as f32 * 8.0)
                .filter(|&u| {
                    let q = at_line(s, u);
                    q.0 > 60.0 && q.1 > 60.0 && q.0 < wf - 60.0 && q.1 < hf - 60.0
                })
                .collect();
            // The fords and the bridge keep `CROSSING_OFF` from the nests
            // where the river has room: with the river off the bowls, a ford
            // could still sit at the bowl's door (1 of 20 valleys' nests
            // had one within 230 cells). Where the river is (through the
            // warp, as it is drawn) at a place along it.
            let on_river = |u: f32| {
                let q = at_line(s, u);
                let (x, y) = (q.0.clamp(0.0, wf - 1.0) as usize, q.1.clamp(0.0, hf - 1.0) as usize);
                at_line(s - warp_x.at(x, y) * 1.6, u)
            };
            let off: Vec<f32> = room
                .iter()
                .copied()
                .filter(|&u| nests.iter().all(|&q| dist(on_river(u), q) >= CROSSING_OFF))
                .collect();
            let room = if off.is_empty() { room } else { off };
            let mut fords = Vec::new();
            if !room.is_empty() {
                fords.push((room[r.below(room.len())], r.range(26.0, 36.0)));
                if r.chance(0.6) {
                    let far: Vec<f32> = room
                        .iter()
                        .copied()
                        .filter(|&u| (u - fords[0].0).abs() > 220.0)
                        .collect();
                    if !far.is_empty() {
                        fords.push((far[r.below(far.len())], r.range(26.0, 36.0)));
                    }
                }
            }
            let far: Vec<f32> = room
                .iter()
                .copied()
                .filter(|&u| fords.iter().all(|f| (u - f.0).abs() > 180.0))
                .collect();
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
    let nest_clay = matches!(rc.soil, Soil::Clay | Soil::Cracks | Soil::Dunes | Soil::Mixed)
        || matches!(rc.water, Water::Pools | Water::Lakes);
    // The cracks (`Soil::Cracks`): the borders of a jittered grid's Voronoi
    // cells, `CRACK` map cells across (a mud crack's polygons), broken up
    // by noise.
    const CRACK: f32 = 110.0;
    let (cgw, cgh) = ((wf / CRACK) as usize + 3, (hf / CRACK) as usize + 3);
    let crack_pts: Vec<(f32, f32)> = if rc.soil == Soil::Cracks {
        (0..cgw * cgh)
            .map(|k| {
                (
                    ((k % cgw) as f32 - 1.0 + r.range(0.15, 0.85)) * CRACK,
                    ((k / cgw) as f32 - 1.0 + r.range(0.15, 0.85)) * CRACK,
                )
            })
            .collect()
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
                        // The bridge's deck is straight and runs 6 cells past
                        // the widest the banks reach. At first it was the
                        // river's own ragged outline (looks review,
                        // 2026-11-15).
                        if (sa - rv.s).abs() < rv.half * 1.1 + 6.0
                            && rv.bridge.is_some_and(|(bu, bh)| (u - bu).abs() < bh)
                        {
                            soil = G_BRIDGE;
                        } else if off < 0.0 {
                            // A ford: shallow right across, the deep part
                            // narrowing to it over 9 cells either side, so
                            // its edge runs smooth. At first the deep part
                            // stopped square at the ford's edge, a box.
                            let taper = rv
                                .fords
                                .iter()
                                .map(|&(fu, fr)| (fr + 9.0 - (u - fu).abs()).max(0.0))
                                .fold(0.0, f32::max);
                            water = water.max(if off < -8.0 - 2.5 * taper { 2 } else { 1 });
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
            // No water by the nests or the landing, and no bridge either:
            // a bridge cell has no water of its own, so clearing only the
            // water left planks on dry land there (bug check, 2026-11-15).
            if dn < 100.0 || dl < 170.0 {
                water = 0;
                if soil == G_BRIDGE {
                    soil = 0;
                }
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

    // Front doors and back roads. The play review (2026-11-15) found 4 of
    // 112 nests with both a direct way at most 10 dish cells wide (a door
    // to hold, wall or turret) and a way round at least 16 wide (an open
    // approach): the open planets had no cheap choke, and on the river
    // valleys, highlands and caves every way in was narrow. So each nest
    // sits in a bowl inside a rock rim. A door 7-9 dish cells wide faces
    // the landing; a breach 28-35 wide opens 105-150 degrees round to one
    // side; and a road 20-23 wide runs from the landing to in front of the
    // door, then round the outside of the rim to the breach. The road
    // follows the ground (`route`), cuts rock where it must and is made
    // shallow across deep water; it is kept clear of crystals, pillars,
    // rings and hedges (`lanes`). The door is the direct way, the road the
    // way round, and `check` holds every nest to both (`doors`).
    struct Bowl {
        q: (f32, f32),
        inner: f32,
        thick: f32,
        /// The most the rim (or moat) reaches.
        reach: f32,
        front: (f32, f32),
        back: (f32, f32),
        door: f32,
        breach: f32,
    }
    let mut bowls: Vec<Bowl> = Vec::new();
    let mut lanes: Vec<(Vec<(f32, f32)>, f32)> = Vec::new();
    let clear_cell = |ground: &mut [u8], i: usize| {
        if ground[i] & G_ROCK != 0 {
            ground[i] = 0;
        } else if ground[i] == DEEP {
            ground[i] = G_WET;
        }
    };
    // Each bowl's rim: room inside for the slime's start and a fight, and
    // small enough that a bowl and the road round it fit across planet 1
    // (496 map cells high) and beside a second nest 0.3 of the width away.
    // Shots stop at rock, so the rim needn't enclose the gun's whole
    // range of the nest (`doors` counts only ground in sight of it).
    let sizes: Vec<(f32, f32, f32)> = (0..nests.len())
        .map(|_| {
            let thick = r.range(26.0, 36.0);
            let inner = r.range(104.0, 136.0);
            // The most the rim reaches, with its wobble.
            let reach = inner * 1.15 + thick * if rc.moat { 1.9 } else { 1.4 };
            (thick, inner, reach)
        })
        .collect();
    for (k, &q) in nests.iter().enumerate() {
        let (thick, inner, outer) = sizes[k];
        let front = unit(landing.0 - q.0, landing.1 - q.1);
        let hw = r.range(40.0, 46.0);
        let ra = outer + hw + 8.0;
        // The road's arc, from in front of the door round to the breach
        // `turns` steps of 15, on one side. From the door, not from 60
        // degrees round: the road's leg from the landing to 60 degrees,
        // then a short arc, was as quick a way to the breach as to the door
        // on the caves and big planets (12 of 64 failed `check`), and in
        // the caves the door's lane could end in rock.
        let arc = |turns: usize, side: f32| -> Vec<(f32, f32)> {
            let mut v = front;
            let mut pts = vec![(q.0 + v.0 * ra, q.1 + v.1 * ra)];
            for _ in 1..=turns {
                v = turn15_to(v, side);
                pts.push((q.0 + v.0 * ra, q.1 + v.1 * ra));
            }
            pts
        };
        // How far the arc keeps inside the map's walkable ground and off
        // the other nests' bowls (negative where it doesn't fit).
        // The other bowls, each with the road's room round it.
        let others: Vec<((f32, f32), f32)> = nests
            .iter()
            .zip(&sizes)
            .enumerate()
            .filter(|&(j, _)| j != k)
            .map(|(_, (&o, sz))| (o, sz.2 + hw + 12.0))
            .collect();
        // From the landing to the arc's start, round any bowl in the way: a
        // waypoint beside it on the side the straight line passes. (Straight,
        // the road to the far nest ran through the near nest's bowl on 9 of
        // 64 planets.)
        let legs = |to: (f32, f32)| -> Vec<(f32, f32)> {
            let mut way = vec![landing, to];
            for _ in 0..4 {
                let hit = way.windows(2).enumerate().find_map(|(i, sg)| {
                    others
                        .iter()
                        .find(|&&(o, room)| seg_dist(o, sg[0], sg[1]) < room)
                        .map(|&(o, room)| (i, o, room))
                });
                let Some((i, o, room)) = hit else { break };
                let (a, b) = (way[i], way[i + 1]);
                let (dx, dy) = (b.0 - a.0, b.1 - a.1);
                let t = (((o.0 - a.0) * dx + (o.1 - a.1) * dy) / (dx * dx + dy * dy).max(1e-6)).clamp(0.0, 1.0);
                let near = (a.0 + dx * t, a.1 + dy * t);
                let n = if dist(near, o) > 1.0 {
                    unit(near.0 - o.0, near.1 - o.1)
                } else {
                    unit(-dy, dx)
                };
                way.insert(i + 1, (o.0 + n.0 * (room + 24.0), o.1 + n.1 * (room + 24.0)));
            }
            way
        };
        // The road's lead, from the landing to in front of the door: over
        // the ground (`route`), off the map's edges and the other bowls;
        // straight legs round the bowls, wobbled, if that finds none.
        let door_front = (q.0 + front.0 * ra, q.1 + front.1 * ra);
        let shut = |c: (f32, f32)| {
            (dist(c, landing) >= 160.0 && c.0.min(c.1).min(wf - c.0).min(hf - c.1) < hw + RIM_MAP as f32)
                || dist(c, q) < outer + hw * 0.5
                || others.iter().any(|&(o, room)| dist(o, c) < room)
        };
        let lead = route(w, h, &ground, landing, door_front, &rough, shut).unwrap_or_else(|| {
            let mut road = vec![landing];
            for sg in legs(door_front).windows(2) {
                let pieces = ((dist(sg[0], sg[1]) / 120.0) as usize).max(2);
                road.extend(&wobbly(&mut r, sg[0], sg[1], pieces, 18.0)[1..]);
            }
            road
        });
        // How far the arc keeps inside the map's walkable ground and off
        // the other bowls (negative where it doesn't fit).
        let room = |pts: &[(f32, f32)]| {
            pts.iter()
                .map(|&a| {
                    let edge = a.0.min(a.1).min(wf - a.0).min(hf - a.1) - hw - RIM_MAP as f32;
                    let other = others
                        .iter()
                        .map(|&(o, room)| dist(o, a) - room + 4.0)
                        .fold(f32::MAX, f32::min);
                    edge.min(other)
                })
                .fold(f32::MAX, f32::min)
        };
        // The breach as far round as the road fits, 150 degrees down to
        // 105, on a side that fits, either if both do. (At 90 the way round
        // on a big map was within reach of the walk to the door.)
        let mut pick = None;
        for turns in (7..=10).rev() {
            let fits: Vec<f32> = [1.0f32, -1.0]
                .into_iter()
                .filter(|&sd| room(&arc(turns, sd)) >= 0.0)
                .collect();
            if !fits.is_empty() {
                pick = Some((turns, fits[r.below(fits.len())]));
                break;
            }
        }
        let (turns, side) = pick.unwrap_or_else(|| {
            (
                7,
                if room(&arc(7, 1.0)) >= room(&arc(7, -1.0)) {
                    1.0
                } else {
                    -1.0
                },
            )
        });
        let pts = arc(turns, side);
        let mut back = front;
        for _ in 0..turns {
            back = turn15_to(back, side);
        }
        let mut road = lead;
        road.extend(&pts[1..]);
        road.push((q.0 + back.0 * inner * 0.55, q.1 + back.1 * inner * 0.55));
        lanes.push((road, hw));
        bowls.push(Bowl {
            q,
            inner,
            thick,
            reach: outer,
            front,
            back,
            door: r.range(14.0, 18.0),
            breach: r.range(56.0, 70.0),
        });
    }
    // The roads first, then the rims over them (a road can't cut another
    // nest's rim), then the bowls, doors and breaches.
    for (pts, hw) in &lanes {
        along_line(w, h, pts, hw * 1.2, |i, _, d| {
            if d < hw * (0.92 + 0.16 * fine.at(i % w, i / w)) {
                clear_cell(&mut ground, i);
            }
        });
    }
    let cross = |v: (f32, f32), u: (f32, f32)| (v.0 * u.1 - v.1 * u.0).abs();
    let dot = |v: (f32, f32), u: (f32, f32)| v.0 * u.0 + v.1 * u.1;
    // A rim's inner edge, its rock's outer edge and its moat's outer edge
    // at a cell. Lobed and knobbly, not a ring: the noises sit near 0.5
    // (most within 0.25-0.75), so at first, at 0.16 and 0.5, the rim read
    // as a drawn circle in game (2026-11-15). A moat (`Recipe::moat`) lies
    // outside a thinner rock lip, wide where the lip is thin. Deep water
    // alone, as the whole rim, held nothing: the slime's start covered the
    // moat by 1 s (its start fills every cell of its disc but rock), the
    // moat never showed, and shots fly over deep water (fens, 2026-11-15).
    let rim_at = |b: &Bowl, x: usize, y: usize| {
        let inner = b.inner * (1.0 + 0.6 * (rough.at(x, y) - 0.5));
        let f = fine.at(x, y);
        if rc.moat {
            let lip = inner + b.thick * (0.3 + 1.0 * f);
            (inner, lip, lip + b.thick * (0.6 + 1.0 * (1.0 - f)))
        } else {
            let lip = inner + b.thick * (0.3 + 1.4 * f);
            (inner, lip, lip)
        }
    };
    // A moat's door is a bridge: a straight deck along the door from 6
    // cells inside the water's nearest reach to 6 past its farthest, as
    // the rivers' decks (planet reviews: a deck that followed the bank's
    // outline read as a mistake).
    let decks: Vec<(f32, f32)> = bowls
        .iter()
        .map(|b| {
            if !rc.moat {
                return (0.0, -1.0);
            }
            let side = (-b.front.1, b.front.0);
            let (mut lo, mut hi) = (f32::MAX, f32::MIN);
            for c in [-b.door, 0.0, b.door] {
                for t in (b.inner as usize / 2)..(b.reach as usize + 8) {
                    let p = (
                        b.q.0 + b.front.0 * t as f32 + side.0 * c,
                        b.q.1 + b.front.1 * t as f32 + side.1 * c,
                    );
                    if p.0 < 0.0 || p.1 < 0.0 || p.0 >= wf || p.1 >= hf {
                        continue;
                    }
                    let (_, lip, outer) = rim_at(b, p.0 as usize, p.1 as usize);
                    let rho = dist(p, b.q);
                    if rho >= lip && rho < outer {
                        lo = lo.min(t as f32);
                        hi = hi.max(t as f32);
                    }
                }
            }
            (lo - 6.0, hi + 6.0)
        })
        .collect();
    for pass in 0..2 {
        for (b, deck) in bowls.iter().zip(&decks) {
            let m = b.inner * 1.3 + b.thick * 2.0 + 60.0;
            for y in ((b.q.1 - m).max(0.0) as usize)..((b.q.1 + m).max(0.0) as usize).min(h) {
                for x in ((b.q.0 - m).max(0.0) as usize)..((b.q.0 + m).max(0.0) as usize).min(w) {
                    let v = (x as f32 + 0.5 - b.q.0, y as f32 + 0.5 - b.q.1);
                    let rho = len(v.0, v.1);
                    let (inner, lip, outer) = rim_at(b, x, y);
                    let door = dot(v, b.front) > 0.0 && cross(v, b.front) < b.door;
                    let breach = dot(v, b.back) > 0.0 && cross(v, b.back) < b.breach * (0.9 + 0.2 * fine.at(x, y));
                    let i = y * w + x;
                    if pass == 0 {
                        if rho >= inner && rho < outer && !door && !breach {
                            ground[i] = if rho < lip { G_ROCK } else { DEEP };
                        }
                    } else if rc.moat && door && (deck.0..=deck.1).contains(&dot(v, b.front)) {
                        ground[i] = G_BRIDGE;
                    } else if rho < inner || ((door || breach) && rho > inner * 0.5 && rho < outer + 40.0) {
                        clear_cell(&mut ground, i);
                    }
                }
            }
        }
    }
    // Each door's lane: straight on toward the landing, as narrow as the
    // door, until it meets open ground (at once, on open planets; in the
    // caves, the nearest tunnel or cavern).
    for b in &bowls {
        let outer = b.reach;
        let mut end = outer + 30.0;
        while end < outer + 600.0 {
            let p = (b.q.0 + b.front.0 * end, b.q.1 + b.front.1 * end);
            if p.0 < 0.0 || p.1 < 0.0 || p.0 >= wf || p.1 >= hf || ground[p.1 as usize * w + p.0 as usize] & G_ROCK == 0
            {
                break;
            }
            end += 8.0;
        }
        let door = vec![
            (b.q.0 + b.front.0 * b.inner * 0.6, b.q.1 + b.front.1 * b.inner * 0.6),
            (b.q.0 + b.front.0 * (end + 16.0), b.q.1 + b.front.1 * (end + 16.0)),
        ];
        along_line(w, h, &door, b.door, |i, _, _| clear_cell(&mut ground, i));
        lanes.push((door, b.door));
        lanes.push((
            vec![
                (b.q.0 + b.back.0 * b.inner * 0.5, b.q.1 + b.back.1 * b.inner * 0.5),
                (b.q.0 + b.back.0 * (outer + 50.0), b.q.1 + b.back.1 * (outer + 50.0)),
            ],
            b.breach,
        ));
    }
    // In the caves, a cavern the rims cut off is filled in: a rim across
    // the tunnels round a nest left caverns behind it with no way in, and
    // their crystals failed `check` (3 of 8 cave planets, 2026-11-15). A
    // cave with no way in is rock. Deep pools count as a way through here,
    // as they did before the bowls.
    if rc.openness == Openness::Caves {
        let mut seen = vec![false; w * h];
        let start = (landing.1 as usize).min(h - 1) * w + (landing.0 as usize).min(w - 1);
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(i) = stack.pop() {
            let (x, y) = (i % w, i / w);
            for (nx, ny) in [(x.wrapping_sub(1), y), (x + 1, y), (x, y.wrapping_sub(1)), (x, y + 1)] {
                if nx < w && ny < h && !seen[ny * w + nx] && ground[ny * w + nx] & G_ROCK == 0 {
                    seen[ny * w + nx] = true;
                    stack.push(ny * w + nx);
                }
            }
        }
        for (g, seen) in ground.iter_mut().zip(&seen) {
            if !seen && *g & G_ROCK == 0 {
                *g = G_ROCK;
            }
        }
    }
    let in_lane = |c: (f32, f32), gap: f32| {
        lanes
            .iter()
            .any(|(pts, half)| pts.windows(2).any(|s| seg_dist(c, s[0], s[1]) < half + gap))
    };

    // Crystals.
    let mut items = Items {
        landing: Some(landing),
        moisture: Some(p.biome.moisture()),
        flora: Some(p.biome as u8),
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
    // The opening, by biome (`Opening`): one near the landing, toward the
    // middle, on the meadow, as on every planet before. Off the roads,
    // which leave the landing toward the middle too: on one there, it
    // narrowed the way round to 10 (highlands seed 3, 2026-11-15). Then any
    // way round the landing and a little farther (a landing by the edge,
    // its roads fanning out inward, had no room within 140), and last as
    // before, on a road if need be.
    let op = Opening::of(p.biome);
    let inward = if op.toward_slime {
        let q = nests
            .iter()
            .copied()
            .min_by(|&a, &b| dist(a, landing).total_cmp(&dist(b, landing)))
            .unwrap_or((wf / 2.0, hf / 2.0));
        unit(q.0 - landing.0, q.1 - landing.1)
    } else {
        unit(wf / 2.0 - landing.0, hf / 2.0 - landing.1)
    };
    for _ in 0..op.count {
        for tries in 0..1200 {
            let phase = tries / 400;
            let (dx, dy) = (r.range(-1.0, 1.0), r.range(-1.0, 1.0));
            let l = len(dx, dy);
            if !(0.2..=1.0).contains(&l) || (phase != 1 && (dx * inward.0 + dy * inward.1) / l < 0.3) {
                continue;
            }
            let d = r.range(op.near.0, if phase == 1 { op.far } else { op.near.1 });
            let c = (landing.0 + dx / l * d, landing.1 + dy / l * d);
            let rad = r.range(op.size.0, op.size.1);
            // In the landing's own region, or in plain sight of it: across a
            // ridge, the nearest crystal was a long walk round (rocky rooms,
            // 2026-11-14). In sight, as the region alone left a landing in a
            // small room no room off its roads (highlands seed 4, 2026-11-15).
            let inside = c.0 > 0.0 && c.1 > 0.0 && c.0 < wf && c.1 < hf;
            let seen = || {
                let n = (d / 2.0) as usize;
                (0..=n).all(|j| {
                    let t = j as f32 / n as f32;
                    let g = ground[(landing.1 + (c.1 - landing.1) * t) as usize * w
                        + (landing.0 + (c.0 - landing.0) * t) as usize];
                    g & G_ROCK == 0 && g != DEEP
                })
            };
            let home = inside && (region[c.1 as usize * w + c.0 as usize] == 0 || seen());
            if home && (phase == 2 || !in_lane(c, rad + 8.0)) && clear(&ground, &items, c, rad, 30.0) {
                items.crystals.push(Crystal {
                    x: c.0.round(),
                    y: c.1.round(),
                    r: rad.round(),
                    charge: op.charges[r.below(op.charges.len())],
                });
                break;
            }
        }
    }
    // The rest by the pattern. `put` places one if its ground is clear.
    let put = |items: &mut Items, ground: &[u8], c: (f32, f32), rad: f32, charge: f32, gap: f32| -> bool {
        // Off the roads, doors and breaches.
        let ok = !in_lane(c, rad + 8.0) && clear(ground, items, c, rad, gap);
        if ok {
            items.crystals.push(Crystal {
                x: c.0.round(),
                y: c.1.round(),
                r: rad.round(),
                charge,
            });
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
    let midway = |r: &mut Rng,
                  items: &mut Items,
                  ground: &[u8],
                  count: usize,
                  size: (f32, f32),
                  charges: &[f32],
                  band: (f32, f32),
                  gap: f32|
     -> Vec<(f32, f32)> {
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
            midway(
                &mut r,
                &mut items,
                &ground,
                rc.big,
                (30.0, 42.0),
                &[384.0, 512.0],
                (0.35, 0.7),
                70.0,
            );
            scatter(
                &mut r,
                &mut items,
                &ground,
                rc.small,
                (18.0, 24.0),
                &[96.0, 128.0, 192.0],
            );
        }
        Pattern::Fields => {
            // A big crystal ringed by small ones, a few of them; a field
            // held is worth a lot.
            let k = rc.big + 1;
            let centres = midway(
                &mut r,
                &mut items,
                &ground,
                k,
                (32.0, 42.0),
                &[384.0, 512.0],
                (0.25, 0.8),
                150.0,
            );
            for c in centres {
                let big = items.crystals.last().map_or(36.0, |b| b.r);
                let m = 3 + r.below(3);
                let mut v = unit(r.range(-1.0, 1.0), r.range(-1.0, 1.0));
                for _ in 0..m {
                    let d = big + r.range(38.0, 58.0);
                    let q = (c.0 + v.0 * d, c.1 + v.1 * d);
                    put(
                        &mut items,
                        &ground,
                        q,
                        r.range(14.0, 20.0),
                        [96.0, 128.0][r.below(2)],
                        16.0,
                    );
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
            midway(
                &mut r,
                &mut items,
                &ground,
                1,
                (30.0, 40.0),
                &[384.0, 512.0],
                (0.4, 0.65),
                70.0,
            );
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
                        if put(
                            &mut items,
                            &ground,
                            c,
                            r.range(12.0, 18.0),
                            [128.0, 192.0][r.below(2)],
                            14.0,
                        ) {
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
            let got = midway(
                &mut r,
                &mut items,
                &ground,
                1,
                (56.0, 70.0),
                &[512.0, 640.0],
                (0.4, 0.62),
                90.0,
            );
            if got.is_empty() {
                midway(
                    &mut r,
                    &mut items,
                    &ground,
                    1,
                    (36.0, 46.0),
                    &[512.0, 640.0],
                    (0.3, 0.7),
                    70.0,
                );
            }
            scatter(
                &mut r,
                &mut items,
                &ground,
                rc.small * 2 / 3,
                (14.0, 20.0),
                &[96.0, 128.0],
            );
        }
        Pattern::Oases => {
            // Clusters at the oases: one middling crystal at the middle and
            // one to three small ones round it.
            for &(c, rad) in &oases {
                put(
                    &mut items,
                    &ground,
                    c,
                    r.range(24.0, 32.0),
                    [256.0, 384.0][r.below(2)],
                    20.0,
                );
                let m = 1 + r.below(3);
                let mut placed = 0;
                for _ in 0..m * 30 {
                    if placed >= m {
                        break;
                    }
                    let v = unit(r.range(-1.0, 1.0), r.range(-1.0, 1.0));
                    let d = r.range(48.0, rad * 0.8 + 10.0);
                    if put(
                        &mut items,
                        &ground,
                        (c.0 + v.0 * d, c.1 + v.1 * d),
                        r.range(14.0, 20.0),
                        [96.0, 128.0][r.below(2)],
                        14.0,
                    ) {
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
        let in_arena = arena[region[(c.1 as usize).min(h - 1) * w + (c.0 as usize).min(w - 1)] as usize];
        if in_arena
            || in_lane(c, 20.0)
            || dist(c, landing) < 150.0
            || nests.iter().any(|&q| dist(q, c) < 120.0)
            || items.crystals.iter().any(|k| dist((k.x, k.y), c) < k.r + 36.0)
            || pillars.iter().any(|&(q, _)| dist(q, c) < 46.0)
        {
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
        clay_islands(&mut ground, w, h, &items.crystals, &soil_b, &fine);
    }

    // Soft rock: gates across passes, rings round nests, hedges. Each is a
    // chain of short pieces that wobbles and changes thickness, so it reads
    // as grown: one straight bar, the first try, read as UI, and perfect
    // arcs as selection rings (graphics lead, 2026-11-14).
    let inside = |p: (f32, f32), m: f32| p.0 > m && p.1 > m && p.0 < wf - m && p.1 < hf - m;
    let dry = |ground: &[u8], p: (f32, f32)| {
        ground[(p.1.max(0.0) as usize).min(h - 1) * w + (p.0.max(0.0) as usize).min(w - 1)] & G_WET == 0
    };
    for e in &edges {
        if e.kind != Border::Gate {
            continue;
        }
        let Some((c, rad)) = e.pass else { continue };
        let (sa, sb) = (sites[e.a], sites[e.b]);
        let t = unit(-(sb.1 - sa.1), sb.0 - sa.0);
        // A cave gate stands on dry ground. At the tunnel's middle, the
        // first place tried, a pool could fill the tunnel: 40 of 163 cave
        // gates had soft rock in water, 138 of 652 pieces (20 caves,
        // 2026-11-15; the map fixes' report, *Not fixed here*). So where
        // the middle is wet the gate moves along its tunnel, nearest the
        // middle first, to the first place where the ground across the
        // tunnel and 12 cells either side of it is dry and both ends are
        // rock (in a tunnel, not a cavern).
        let wet_across = |c: (f32, f32), t: (f32, f32), along: (f32, f32), half: f32| {
            (-2..=2).any(|k| {
                let o = k as f32 * 6.0;
                let mut v = -half - 2.0;
                let mut wet = false;
                while v <= half + 2.0 && !wet {
                    wet = !dry(&ground, (c.0 + t.0 * v + along.0 * o, c.1 + t.1 * v + along.1 * o));
                    v += 1.0;
                }
                wet
            })
        };
        let rock_at = |p: (f32, f32)| inside(p, 1.0) && ground[p.1 as usize * w + p.0 as usize] & G_ROCK != 0;
        let (c, t, rad) = if e.way.is_empty() || !wet_across(c, t, (t.1, -t.0), rad + 8.0) {
            (c, t, rad)
        } else {
            let mid = e.way.len() / 2;
            let mut order: Vec<usize> = (0..e.way.len()).collect();
            order.sort_by_key(|&i| i.abs_diff(mid));
            order
                .into_iter()
                .map(|i| e.way[i])
                .find(|&(q, dir, wd)| {
                    let n = (-dir.1, dir.0);
                    let half = wd + 8.0;
                    rock_at((q.0 + n.0 * half, q.1 + n.1 * half))
                        && rock_at((q.0 - n.0 * half, q.1 - n.1 * half))
                        && !wet_across(q, n, dir, half)
                })
                .map_or((c, t, rad), |(q, dir, wd)| (q, (-dir.1, dir.0), wd))
        };
        // Where no dry place in a tunnel has rock at both ends (on cave seed
        // 2, three of ten gates' tunnels were wet all along), the gate
        // stands on a sill: the water under it, 16-30 cells either side (a
        // ragged edge, by the fine noise), is let out (the clay under a pool
        // stays), so the soft rock sits on a bank. At 14 cells either side
        // the bank was a thin dark line beside the gate at zoom 6.
        if !e.way.is_empty() && wet_across(c, t, (t.1, -t.0), rad + 8.0) {
            let along = (t.1, -t.0);
            let half = rad + 8.0;
            let m = half + 30.0;
            for y in ((c.1 - m).max(0.0) as usize)..((c.1 + m).max(0.0) as usize).min(h) {
                for x in ((c.0 - m).max(0.0) as usize)..((c.0 + m).max(0.0) as usize).min(w) {
                    let (dx, dy) = (x as f32 + 0.5 - c.0, y as f32 + 0.5 - c.1);
                    if (dx * t.0 + dy * t.1).abs() <= half + 3.0
                        && (dx * along.0 + dy * along.1).abs() <= 16.0 + 14.0 * fine.at(x, y)
                        && ground[y * w + x] & G_ROCK == 0
                    {
                        let i = y * w + x;
                        ground[i] = if ground[i] == G_DEEP { 0 } else { ground[i] & !G_WET };
                    }
                }
            }
        }
        // Across the slot, its ends in the ridge's rock.
        let half = rad + 8.0;
        let pts = wobbly(
            &mut r,
            (c.0 - t.0 * half, c.1 - t.1 * half),
            (c.0 + t.0 * half, c.1 + t.1 * half),
            4,
            5.0,
        );
        // No gate across a road or a nest's door or breach.
        if !pts.iter().any(|&p| in_lane(p, 10.0)) {
            hedge(&mut r, &mut items.walls, &pts);
        }
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
        if !pts.iter().any(|&p| in_lane(p, 10.0)) {
            hedge(&mut r, &mut items.walls, &pts);
        }
    }
    for (k, &q) in nests.iter().enumerate() {
        if !r.chance(rc.ring) {
            continue;
        }
        let rad = items.crystals[k].r + 55.0;
        let to = unit(landing.0 - q.0, landing.1 - q.1);
        // Two or three arcs, each at its own radius with a gap between, not
        // one round ring: rings read as bullseyes (looks review,
        // 2026-11-15). The radius still wanders along each arc, smoothly.
        let arcs = 2 + r.below(2);
        let start = r.below(24);
        let lift = [0.0f32, 0.16, -0.1];
        let mut radii = [0.0f32; 24];
        let mut off = 0.0;
        for (i, v) in radii.iter_mut().enumerate() {
            off = off * 0.6 + r.range(-0.07, 0.07);
            let arc = ((i + 24 - start) % 24) * arcs / 24;
            *v = rad * (1.0 + off + lift[arc]);
        }
        let mut v = (1.0f32, 0.0f32);
        for i in 0..24 {
            let u = turn15(v);
            let mid = unit(v.0 + u.0, v.1 + u.1);
            // Each arc's last piece is its gap. No hedge on water (the looks
            // review found hedges laid across lakes), nor across the
            // breach: there the ring shut the way round (jungle, 2026-11-15).
            // The ring's gap toward the landing is the door's.
            let j = (i + 24 - start) % 24;
            let (arc, next) = (j * arcs / 24, ((j + 1) % 24) * arcs / 24);
            let (ra, rb) = (radii[i], radii[(i + 1) % 24]);
            let (a, b) = ((q.0 + v.0 * ra, q.1 + v.1 * ra), (q.0 + u.0 * rb, q.1 + u.1 * rb));
            let half = ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0);
            if next == arc
                && mid.0 * to.0 + mid.1 * to.1 < 0.85
                && inside(a, 8.0)
                && inside(b, 8.0)
                && dry(&ground, a)
                && dry(&ground, half)
                && dry(&ground, b)
                && ![a, half, b].iter().any(|&p| in_lane(p, 4.0))
            {
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
                        || edges
                            .iter()
                            .any(|e| e.pass.is_some_and(|(c, rad)| dist(p, c) < rad + 30.0))
                        || items.crystals.iter().any(|c| dist((c.x, c.y), p) < c.r + 34.0)
                        || ground[p.1 as usize * w + p.0 as usize] & G_ROCK != 0
                        || !dry(&ground, p)
                        || in_lane(p, 16.0)
                        || arena[region[p.1 as usize * w + p.0 as usize] as usize]
                };
                if blocked(at) || blocked(b) || !dry(&ground, ((at.0 + b.0) / 2.0, (at.1 + b.1) / 2.0)) {
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
    Map {
        w,
        h,
        ground,
        plants,
        items,
    }
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
    let targets: Vec<((f32, f32), f32)> = items
        .crystals
        .iter()
        .map(|c| ((c.x, c.y), c.r))
        .chain(items.moulds.iter().map(|&q| (q, 8.0)))
        .collect();
    let mut skip = vec![false; targets.len()];
    for _ in 0..12 {
        let m = Map {
            w,
            h,
            ground: ground.to_vec(),
            plants: Vec::new(),
            items: items.clone(),
        };
        let (bw, bh, open) = walkable(&m, (w, h), false);
        let d = walk_from(bw, bh, &open, landing);
        let Some(t) =
            (0..targets.len()).find(|&t| !skip[t] && reach(bw, bh, &d, targets[t].0, targets[t].1) == u32::MAX)
        else {
            return;
        };
        // The same walk with deep water as ground.
        let shallow = Map {
            ground: m.ground.iter().map(|&g| if g == G_DEEP { G_WET } else { g }).collect(),
            ..m
        };
        let (_, _, open2) = walkable(&shallow, (w, h), false);
        let d2 = walk_from(bw, bh, &open2, landing);
        let (c, rad) = targets[t];
        // The bin by the target nearest the landing, as `reach` reads it.
        let mm = rad + 2.0 * BIN as f32 + 2.0;
        let mut at = None;
        for by in ((c.1 - mm) / BIN as f32).floor().max(0.0) as usize..=(((c.1 + mm) / BIN as f32) as usize).min(bh - 1)
        {
            for bx in
                ((c.0 - mm) / BIN as f32).floor().max(0.0) as usize..=(((c.0 + mm) / BIN as f32) as usize).min(bw - 1)
            {
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
///
/// Before 11-14 22:00Z every biome but the jungle was 25-60% bare by the
/// canopy alone, so with plants drawn almost every planet was green edge
/// to edge, Barrens, Cave and Highlands too (graphics lead on the plant
/// thumbnails: plants should tell biomes apart). Now `clumps` sows on the
/// fine noise (clumps about 20 map cells across, not 140-cell patches) and
/// `soils` keeps the growth to the soils named.
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
    /// Sparse clumps: bare below `bare` on the fine noise instead.
    clumps: bool,
    /// The soil kinds (bit `1 << kind`) anything but reeds grows on.
    soils: u8,
}

impl Sow {
    fn of(b: Biome, rc: &Recipe) -> Sow {
        use crate::mould::plants::{MEADOW, SCRUB};
        let base = Sow {
            bare: 0.3,
            main: MEADOW,
            jungle: 0.78,
            shore: 10.0,
            sand: Some(SCRUB),
            thin: 0.35,
            ramp: 0.3,
            lush: false,
            clumps: false,
            soils: 0xff,
        };
        match b {
            // Grass nearly everywhere, a few copses.
            Biome::Meadow => Sow {
                bare: 0.25,
                jungle: 0.74,
                ..base
            },
            // Scrub in sparse clumps; the oases green.
            Biome::Barrens => Sow {
                bare: BARRENS_BARE,
                main: SCRUB,
                jungle: 1.1,
                shore: 0.0,
                thin: 0.25,
                ramp: 0.1,
                lush: rc.soil == Soil::Oases,
                clumps: true,
                ..base
            },
            // Reed beds round every pool and lake.
            Biome::Fen => Sow { shore: 18.0, ..base },
            // Reeds along the banks, scrub on the dunes.
            Biome::Valley => Sow {
                shore: 14.0,
                jungle: 0.8,
                thin: 0.3,
                ..base
            },
            // Canopy over most of it, meadow in the clearings.
            Biome::Jungle => Sow {
                bare: 0.15,
                jungle: 0.3,
                shore: 8.0,
                thin: 0.4,
                ramp: 0.2,
                ..base
            },
            // Grass in patches on the clay only.
            Biome::Highlands => Sow {
                bare: HIGHLANDS_BARE,
                jungle: 1.1,
                shore: 6.0,
                thin: 0.25,
                soils: 1 << 2,
                ..base
            },
            // Reeds at the pools and a clump or two in the caverns.
            Biome::Caves => Sow {
                bare: CAVES_BARE,
                jungle: 1.1,
                shore: 6.0,
                sand: None,
                thin: 0.2,
                ramp: 0.1,
                clumps: true,
                ..base
            },
            Biome::Core => Sow { jungle: 0.62, ..base },
        }
    }
}

/// The fine noise's level above which the Barrens grow a clump of scrub,
/// and the Cave a clump of grass: about 12% and 3% of their ground (the
/// graphics lead's 10-15% and "a little", 11-14). The Highlands' level
/// on the canopy, on clay only.
const BARRENS_BARE: f32 = 0.62;
/// How far a plant patch's edge fades to nothing (`sow`): in the canopy
/// noise (about 6 map cells, 3 of the game's fine cells), in the fine noise
/// (`Sow::clumps`, steeper), and in map cells at the reeds' edge and past a
/// crystal's or the landing's clearing.
const FADE: f32 = 0.06;
const FADE_CLUMPS: f32 = 0.15;
const REED_FADE: f32 = 4.0;
const CLEAR_FADE: f32 = 6.0;
const CAVES_BARE: f32 = 0.7;
const HIGHLANDS_BARE: f32 = 0.6;

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
fn sow(
    sw: &Sow,
    w: usize,
    h: usize,
    ground: &[u8],
    items: &Items,
    landing: (f32, f32),
    canopy: &Field,
    fine: &Field,
) -> Vec<u8> {
    use crate::mould::plants::{pack, JUNGLE, REEDS};
    let wet = |i: usize| ground[i] & G_WET != 0;
    // Map cells to the water's edge, either side (one chamfer from the
    // cells whose neighbour is the other kind); none on a dry planet.
    let any_wet = sw.shore > 0.0 && ground.iter().any(|&g| g & G_WET != 0);
    let to_edge = if any_wet {
        chamfer(w, h, |i| {
            let (x, y) = (i % w, i / w);
            (x > 0 && wet(i - 1) != wet(i))
                || (x + 1 < w && wet(i + 1) != wet(i))
                || (y > 0 && wet(i - w) != wet(i))
                || (y + 1 < h && wet(i + w) != wet(i))
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
            let soil = g >> 4;
            let oasis = sw.lush && soil == 0;
            let clumps = sw.clumps && !oasis;
            let n = if clumps {
                fine.at(x, y)
            } else {
                canopy.at(x, y) + (fine.at(x, y) - 0.5) * 0.14
            };
            // Every edge fades to nothing over a few cells (`FADE`): a
            // patch, a bed's edge where reeds or jungle start, a crystal's
            // clearing. The view draws a patch's edge where its density
            // crosses 0.2 between cells, so a ramp puts it between them and
            // the edge runs smooth; a step from 0.4 to nothing put it on
            // the cells' own staircase (terrain look thread at zoom 24,
            // 11-14 22:15Z: Fen and Jungle beds in 1-cell steps every ~7).
            // Where two kinds meet, both fade, leaving a seam a cell wide.
            let fade = if clumps { FADE_CLUMPS } else { FADE };
            // Deep water and bridges grow nothing (`plants::SOIL_CAP`).
            if g == G_DEEP || g == G_BRIDGE {
                continue;
            }
            // How far past the reeds' reach, map cells (none: no reeds).
            let mut past_reeds = f32::MAX;
            if any_wet {
                // Reeds: thickest at the water's edge, thinning out to
                // `shore` on land and in the water both.
                // Into the water only a few cells, so open water still
                // reads (a full fringe hid the rivers' shallows).
                let d = to_edge[i] as f32 / 3.0;
                let reach = sw.shore * (0.7 + 0.6 * fine.at(x, y)) * if wet(i) { 0.35 } else { 1.0 };
                if d <= reach {
                    // (A fringe into the water is a few cells: it fades over
                    // its outer half, or it broke into beads.)
                    out[i] = pack(
                        REEDS,
                        (1.0 - 0.6 * d / reach.max(1.0)) * ramp(reach - d, 0.0, REED_FADE.min(0.5 * reach)),
                    );
                    continue;
                }
                past_reeds = d - reach;
            }
            if wet(i) || soil == 3 || sw.soils & (1 << soil.min(7)) == 0 {
                continue;
            }
            let (bare, jungle) = if oasis { (0.12, 0.6) } else { (sw.bare, sw.jungle) };
            if n < bare {
                continue;
            }
            let edge = ramp(n, bare, fade) * ramp(past_reeds, 0.0, REED_FADE);
            out[i] = if soil == 1 {
                match sw.sand {
                    Some(k) => pack(k, (sw.thin + (1.0 - sw.thin) * ramp(n, bare, sw.ramp)) * edge),
                    None => 0,
                }
            } else if n >= jungle {
                pack(
                    JUNGLE,
                    (0.5 + 0.5 * ramp(n, jungle, 0.15)) * ramp(n, jungle, fade) * ramp(past_reeds, 0.0, REED_FADE),
                )
            } else {
                let kind = if oasis { crate::mould::plants::MEADOW } else { sw.main };
                pack(
                    kind,
                    (sw.thin + (1.0 - sw.thin) * ramp(n, bare, sw.ramp)) * edge * ramp(jungle - n, 0.0, fade),
                )
            };
        }
    }
    // A ragged clear patch where the squad lands, and nothing on a crystal
    // (the game grows nothing there either), each fading in over `CLEAR_FADE`
    // map cells past it. Each over its own box: over every cell and every
    // crystal this pass was most of a planet's build.
    let mut clear = |c: (f32, f32), reach: f32, keep: &dyn Fn(usize, usize, (f32, f32)) -> f32| {
        let (x0, y0) = ((c.0 - reach).max(0.0) as usize, (c.1 - reach).max(0.0) as usize);
        let (x1, y1) = (((c.0 + reach) as usize + 1).min(w), ((c.1 + reach) as usize + 1).min(h));
        for y in y0..y1 {
            for x in x0..x1 {
                let k = keep(x, y, (x as f32 + 0.5, y as f32 + 0.5));
                let b = &mut out[y * w + x];
                if k < 1.0 && *b != 0 {
                    let (kind, d) = crate::mould::plants::unpack(*b);
                    *b = pack(kind, d * k);
                }
            }
        }
    };
    clear(landing, 50.0 + CLEAR_FADE, &|x, y, at| {
        ramp(dist(at, landing), 30.0 + (fine.at(x, y) - 0.5) * 40.0, CLEAR_FADE)
    });
    for c in &items.crystals {
        let q = (c.x, c.y);
        clear(q, c.r + 2.0 + CLEAR_FADE, &|_, _, at| {
            ramp(dist(at, q), c.r + 2.0, CLEAR_FADE)
        });
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
    // The dish has no agar within 3 dish cells (12 map cells) of its edge
    // (`big::rect`), so the game walks none of it: those bins are shut
    // after the narrowing, which models a squad's width against rock, not
    // against the rim. Treating the edge as open, at first, passed a
    // planet whose landing reached 3 of 20 crystals in the game, round a
    // ridge's end through the rim (planet 5 seed 39, technical review,
    // 2026-11-15).
    let rim = RIM_MAP.div_ceil(BIN);
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
    for by in 0..bh {
        for bx in 0..bw {
            if bx < rim || by < rim || bx + rim >= bw || by + rim >= bh {
                out[by * bw + bx] = false;
            }
        }
    }
    (bw, bh, out)
}

/// The dish's rim, map cells: no agar within 3 dish cells of the edge
/// (`BigDish::with_map`).
const RIM_MAP: usize = 12;

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
        // Less the door's straight way in: a nest's bowl opens a door
        // facing the landing, so the walk to it runs nearly straight.
        if (s.to_first as f32) < 0.6 * w - 60.0 {
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
    // Below today's area a map may be too small for two bowls 400 apart
    // (`build_with`'s nests): one nest will do there. The run's planets
    // with more than one nest are all bigger.
    let want = if m.w * m.h >= MAP_W * MAP_H { rc.nests } else { 1 };
    if it.moulds.len() < want {
        out.push(format!("{} of {} nests placed", it.moulds.len(), rc.nests));
    }
    out.extend(door_problems(p, m));
    out
}

/// The widest the direct way into a nest may be, and the least the way
/// round must be, in dish cells (play review, 2026-11-15; its
/// `localchoke.py`).
pub const DOOR_MAX: u32 = 10;
pub const ROAD_MIN: u32 = 16;

/// The longest a direct way may be, from the shortest walk `l0` (map
/// cells): 15% longer, as the play review measured, but at most 250 cells
/// (62 dish cells) longer. On the 6x core world 15% is 330 cells, so any
/// way round a nest's bowl short of its far side counted as direct.
fn direct_limit(l0: u32) -> u32 {
    l0.saturating_add(((l0 as f32 * 0.15) as u32).min(250))
}

/// Walks over the walking bins whose clearance is at least a level, from
/// the landing to each nest's gun range, made as they are asked for.
struct Ways {
    bw: usize,
    bh: usize,
    open: Vec<bool>,
    /// Bins to the nearest shut bin, by a 3-4 chamfer (thirds of a bin).
    clear: Vec<u16>,
    landing: (f32, f32),
    /// Each nest's bins: within the gun's range (24 dish cells) of its
    /// home crystal's rim, as the play review measured, and in sight of
    /// the nest.
    goals: Vec<Vec<usize>>,
    walks: Vec<(u16, Vec<u32>)>,
}

impl Ways {
    fn new(p: &Planet, m: &Map) -> Ways {
        let (bw, bh, open) = walkable(m, p.size, true);
        let clear = chamfer(bw, bh, |i| !open[i]);
        // No rock between: shots stop at rock (`Rules::aim_min`'s sight
        // check), so a squad outside a nest's rim can't shoot it.
        let (w, h) = p.size;
        let sight = |a: (f32, f32), b: (f32, f32)| {
            let k = (dist(a, b) / 2.0).ceil().max(1.0) as usize;
            (0..=k).all(|i| {
                let t = i as f32 / k as f32;
                let (x, y) = ((a.0 + (b.0 - a.0) * t) as usize, (a.1 + (b.1 - a.1) * t) as usize);
                x >= w || y >= h || m.ground[y * w + x] & G_ROCK == 0
            })
        };
        let goals = m
            .items
            .moulds
            .iter()
            .map(|&q| {
                let home = m
                    .items
                    .crystals
                    .iter()
                    .find(|c| dist((c.x, c.y), q) < 2.0)
                    .map_or(40.0, |c| c.r);
                let rad = home + 96.0;
                let (x0, x1) = (
                    ((q.0 - rad) / BIN as f32).max(0.0) as usize,
                    (((q.0 + rad) / BIN as f32) as usize).min(bw - 1),
                );
                let (y0, y1) = (
                    ((q.1 - rad) / BIN as f32).max(0.0) as usize,
                    (((q.1 + rad) / BIN as f32) as usize).min(bh - 1),
                );
                let mut out = Vec::new();
                for by in y0..=y1 {
                    for bx in x0..=x1 {
                        let c = ((bx * BIN) as f32 + 2.0, (by * BIN) as f32 + 2.0);
                        if open[by * bw + bx] && dist(c, q) <= rad && sight(c, q) {
                            out.push(by * bw + bx);
                        }
                    }
                }
                out
            })
            .collect();
        Ways {
            bw,
            bh,
            open,
            clear,
            landing: m.items.landing.unwrap_or((0.0, 0.0)),
            goals,
            walks: Vec::new(),
        }
    }

    /// The shortest walk to nest `k` over bins at least `level` bins clear
    /// (`u32::MAX` if there is none).
    fn walk(&mut self, level: u16, k: usize) -> u32 {
        if !self.walks.iter().any(|w| w.0 == level) {
            let open: Vec<bool> = self
                .open
                .iter()
                .zip(&self.clear)
                .map(|(&o, &c)| o && c >= 3 * level)
                .collect();
            self.walks
                .push((level, walk_from(self.bw, self.bh, &open, self.landing)));
        }
        let d = &self.walks.iter().find(|w| w.0 == level).unwrap().1;
        self.goals[k].iter().map(|&i| d[i]).min().unwrap_or(u32::MAX)
    }

    /// Dish cells across, for a clearance level: the narrowing in
    /// `walkable` took a bin off each side.
    fn width(level: u16) -> u32 {
        2 * level as u32 + 2
    }
}

/// Each nest's direct way and way round, in dish cells: the widest route
/// from the landing to within the gun's range of the nest, in its sight,
/// at most 15% (and 250 cells) longer than the shortest (`direct_limit`), and the widest route of any
/// length; soft rock kept. (0, 0) where none reaches it. As the play
/// review's `localchoke.py` measured them, on the checks' walking bins,
/// but counting only ground in sight of the nest: shots stop at rock.
pub fn doors(p: &Planet, m: &Map) -> Vec<(u32, u32)> {
    let mut ways = Ways::new(p, m);
    let top = ways.clear.iter().copied().filter(|&c| c != u16::MAX).max().unwrap_or(0) / 3;
    (0..m.items.moulds.len())
        .map(|k| {
            let l0 = ways.walk(1, k);
            if l0 == u32::MAX {
                return (0, 0);
            }
            // The highest level at which `ok` holds (it holds at 1, and
            // fails above some level).
            let best = |ways: &mut Ways, ok: &dyn Fn(u32) -> bool| {
                let (mut lo, mut hi) = (1u16, top.max(1));
                while lo < hi {
                    let mid = (lo + hi).div_ceil(2);
                    if ok(ways.walk(mid, k)) {
                        lo = mid;
                    } else {
                        hi = mid - 1;
                    }
                }
                Ways::width(lo)
            };
            let limit = direct_limit(l0);
            let direct = best(&mut ways, &|d| d <= limit);
            let round = best(&mut ways, &|d| d != u32::MAX);
            (direct, round)
        })
        .collect()
}

/// What `check` asks of each nest's ways in: no direct way wider than
/// `DOOR_MAX`, and a way round at least `ROAD_MIN` wide. Three walks, not
/// `doors`'s searches.
fn door_problems(p: &Planet, m: &Map) -> Vec<String> {
    let mut ways = Ways::new(p, m);
    // The least level wider than the door may be, and the least the road
    // must keep.
    let wide = (1u16..).find(|&l| Ways::width(l) > DOOR_MAX).unwrap();
    let road = (1u16..).find(|&l| Ways::width(l) >= ROAD_MIN).unwrap();
    let mut out = Vec::new();
    for k in 0..m.items.moulds.len() {
        let l0 = ways.walk(1, k);
        if l0 == u32::MAX {
            out.push(format!("nest {k} cannot be reached past soft rock"));
            continue;
        }
        if ways.walk(wide, k) <= direct_limit(l0) {
            out.push(format!("nest {k}: the direct way is wider than {DOOR_MAX}"));
        }
        if ways.walk(road, k) == u32::MAX {
            out.push(format!("nest {k}: no way round {ROAD_MIN} wide"));
        }
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

    /// FNV-1a over the ground, the plants and the item text: a planet's
    /// fingerprint.
    fn print(m: &Map) -> u64 {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for b in m
            .ground
            .iter()
            .copied()
            .chain(m.plants.iter().copied())
            .chain(m.items.text().into_bytes())
        {
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
    // up to 2x (`GROW_CAP`); then each biome's moisture line and plants
    // that tell the biomes apart, their edges faded (11-14 22:30Z), and
    // the core world's arenas and nest hedges (23:30Z), and planets 6-8 at
    // their full 2.8x, 4x and 6x (`GROW_CAP` 6; planet 8 is the big pin
    // below). Then planet 4 (river valley): no dune clay over deep water
    // or bridges, no bridge planks by the nests (bug check, 11-15). Then
    // every planet again for the reviews' map fixes (11-15): nests spaced
    // by the map's width and kept off its edges, streams and rivers
    // capped, nest rings in arcs, no hedges on water, straight bridges and
    // tapered fords. Round 2's depths 3-8 had kept their pins through the
    // tiled region scan, which finds the same nearest sites as the full
    // scan. The item text's `flora` line (11-15, plants thread: each
    // biome's plant forms in the view) changed every print, not the ground
    // or plants. Then every planet again for the front doors and back
    // roads (11-15): a rock bowl round each nest, roads over the ground,
    // the landing's crystal and the rings off the roads, cut-off caverns
    // filled. Then the fens (planet 3) alone for lakes 3 to 1 (11-15).
    // Then the caves (planet 7) alone for lobed caverns, tunnels of
    // varying width and arenas (11-15).
    // Then planets 2-8 for each biome's opening crystals (`Opening`,
    // 11-15); the meadow keeps its start and its pin. Then the river
    // valley (planet 4) alone for rivers off the nests (11-15).
    const PINNED: [u64; 8] = [
        0x710b41cc2b682699,
        0x689ef2ee1bd03e2b,
        0x88abf15af468b6c7,
        0x3a3ec87750ebb333,
        0x1c111e19292277c5,
        0x6e94308f248ff649,
        0x800af9fb4a2f4910,
        0x6f215522f0271821,
    ];

    /// The dunes' clay islands leave rock, deep water and bridges as they
    /// are (bug check, 2026-11-15: clay over deep water made it wadeable).
    #[test]
    fn clay_islands_keep_deep_water_and_bridges() {
        let (w, h) = (64, 64);
        let flat = Field::new(w, h, |_, _| 0.5);
        let mut ground = vec![0u8; w * h];
        let at = |x: usize, y: usize| y * w + x;
        ground[at(30, 32)] = G_DEEP;
        ground[at(31, 32)] = G_BRIDGE;
        ground[at(33, 32)] = G_ROCK;
        ground[at(34, 32)] = G_WET;
        clay_islands(
            &mut ground,
            w,
            h,
            &[Crystal {
                x: 32.0,
                y: 32.0,
                r: 4.0,
                charge: 0.0,
            }],
            &flat,
            &flat,
        );
        assert_eq!(ground[at(30, 32)], G_DEEP);
        assert_eq!(ground[at(31, 32)], G_BRIDGE);
        assert_eq!(ground[at(33, 32)], G_ROCK);
        assert_eq!(
            ground[at(34, 32)],
            G_WET | G_CLAY,
            "shallow water takes the clay under it"
        );
        assert_eq!(ground[at(32, 30)], G_CLAY);
        assert_eq!(ground[at(2, 2)], 0, "outside the island");
    }

    /// The dials seed 1 never rolls, pinned too: pillars (planet 1 seed
    /// 17), oases (2:7), lakes (3:3), ridges across a river with its fords
    /// and bridges (4:2), a cave with pools (7:2), and a core world of
    /// pillars and streams at the full 6x (8:3). Until 2026-11-15 the big
    /// pin here was planet 8 seed 1 again, the same as `PINNED[7]`
    /// (technical review, finding 7).
    #[test]
    fn every_dial_is_pinned() {
        let got: Vec<u64> = MORE_PINNED.iter().map(|&(d, s, _)| print(&level(s, d))).collect();
        let want: Vec<u64> = MORE_PINNED.iter().map(|p| p.2).collect();
        assert_eq!(got, want, "{got:#x?}");
    }

    // Re-pinned for the item text's `flora` line too (11-15, plants
    // thread), then for the front doors and back roads. The fen (3:3)
    // rolled lakes before lakes went to 3 to 1, and kept its pin. The cave
    // (7:2) re-pinned for lobed caverns and tunnels of varying width.
    // All but the meadow's (1:17) re-pinned for each biome's opening.
    // The valley (4:2) re-pinned for rivers off the nests.
    // The cave (7:2), which rolled pools, re-pinned for gates off pools.
    const MORE_PINNED: [(u32, u64, u64); 6] = [
        (1, 17, 0x3fcdee47fe91e8b2),
        (2, 7, 0x9cf43bc6b64daab9),
        (3, 3, 0x162f0835a692b0e6),
        (4, 2, 0x6ec7406552e2d2ff),
        (7, 2, 0x1bbca9c8f72e8eaa),
        (8, 3, 0xd8db02e2b122e92e),
    ];

    /// Each biome opens its own way (`Opening`): three small crystals close
    /// by in the jungle, a rich one a climb away in the highlands, a rich
    /// one toward the slime on the core world. Every planet opened with one
    /// crystal 85-140 map cells off before (play review, 2026-11-15).
    #[test]
    fn each_biome_opens_its_own_way() {
        for s in 1..=3 {
            let near = |d: u32, f: &dyn Fn(&Crystal, f32) -> bool| {
                let m = level(s, d);
                let land = m.items.landing.unwrap();
                m.items.crystals.iter().filter(|c| f(c, dist((c.x, c.y), land))).count()
            };
            let small = near(5, &|c, d| d <= 165.0 && c.r <= 18.0 && c.charge == 128.0);
            assert!(small >= 3, "jungle {s}: {small} small crystals by the landing");
            let rich = near(6, &|c, d| d <= 235.0 && c.charge == 384.0);
            assert!(rich >= 1, "highlands {s}: no rich crystal by the landing");
            let core = near(8, &|c, d| d <= 265.0 && c.r >= 34.0 && c.charge == 512.0);
            assert!(core >= 1, "core {s}: no rich crystal by the landing");
        }
    }

    /// Each cell's distance to the nearest cell where `open` is false,
    /// exactly (Felzenszwalb and Huttenlocher's squared distance transform,
    /// rows then columns), in cells.
    fn clearance(open: &[bool], w: usize, h: usize) -> Vec<f32> {
        const FAR: f32 = 1e12;
        fn pass(f: &[f32]) -> Vec<f32> {
            let n = f.len();
            let (mut v, mut z, mut k) = (vec![0usize; n], vec![0f32; n + 1], 0usize);
            z[0] = -FAR;
            z[1] = FAR;
            for q in 1..n {
                loop {
                    let p = v[k];
                    let s = ((f[q] + (q * q) as f32) - (f[p] + (p * p) as f32)) / (2.0 * (q as f32 - p as f32));
                    if s <= z[k] && k > 0 {
                        k -= 1;
                        continue;
                    }
                    if s <= z[k] {
                        v[0] = q;
                        z[1] = FAR;
                        break;
                    }
                    k += 1;
                    v[k] = q;
                    z[k] = s;
                    z[k + 1] = FAR;
                    break;
                }
            }
            k = 0;
            (0..n)
                .map(|q| {
                    while z[k + 1] < q as f32 {
                        k += 1;
                    }
                    let d = q as f32 - v[k] as f32;
                    d * d + f[v[k]]
                })
                .collect()
        }
        let mut d: Vec<f32> = open.iter().map(|&o| if o { FAR } else { 0.0 }).collect();
        for y in 0..h {
            let row = pass(&d[y * w..(y + 1) * w]);
            d[y * w..(y + 1) * w].copy_from_slice(&row);
        }
        for x in 0..w {
            let col: Vec<f32> = (0..h).map(|y| d[y * w + x]).collect();
            for (y, v) in pass(&col).into_iter().enumerate() {
                d[y * w + x] = v;
            }
        }
        d.into_iter().map(f32::sqrt).collect()
    }

    /// Cave tunnels pinch and widen along their way, and each cave planet
    /// has a room to fight in away from the landing and the nests. On 20
    /// caves before (main at 2026-11-15 08:00Z, `widths.py` in
    /// `Reports/levelgen-caves-2026-11-15/raw/`), a tunnel was 12-13 dish
    /// cells wide at the 10th percentile and 17-18 at the 90th, the same
    /// all along, and the widest room away from home had 76-109 map cells
    /// of clearance; after, 9 and 21-25, and 120-235.
    #[test]
    fn cave_tunnels_vary_and_open_into_rooms() {
        for s in 1..=3 {
            let p = planet(s, 7);
            let m = level(s, 7);
            let (w, h) = p.size;
            let open: Vec<bool> = m.ground.iter().map(|&g| g & G_ROCK == 0).collect();
            let d = clearance(&open, w, h);
            // A tunnel's middle: a ridge of the clearance (no neighbour
            // farther from rock), under 30 dish cells wide. Its clearance
            // in map cells is its width in dish cells.
            let mut widths = Vec::new();
            for y in 1..h - 1 {
                for x in 1..w - 1 {
                    let c = d[y * w + x];
                    if c > 1.0
                        && c < 30.0
                        && [(0, 1), (1, 0), (1, 1), (1, -1)]
                            .iter()
                            .all(|&(dx, dy): &(isize, isize)| {
                                let (a, b) = (
                                    ((y as isize + dy) as usize) * w + (x as isize + dx) as usize,
                                    ((y as isize - dy) as usize) * w + (x as isize - dx) as usize,
                                );
                                c >= d[a] && c >= d[b]
                            })
                    {
                        widths.push(c);
                    }
                }
            }
            widths.sort_by(f32::total_cmp);
            let at = |q: f32| widths[(q * (widths.len() - 1) as f32) as usize];
            assert!(
                at(0.1) <= 10.5 && at(0.9) >= 19.5,
                "cave {s}: tunnel widths {:.1} to {:.1} (10th to 90th percentile)",
                at(0.1),
                at(0.9)
            );
            let land = m.items.landing.unwrap();
            let homes: Vec<(f32, f32)> = std::iter::once(land).chain(m.items.moulds.iter().copied()).collect();
            let room = (0..w * h)
                .filter(|&i| homes.iter().all(|&q| dist(q, ((i % w) as f32, (i / w) as f32)) > 260.0))
                .map(|i| d[i])
                .fold(0.0, f32::max);
            assert!(
                room >= 115.0,
                "cave {s}: the widest room away from home has {room:.0} map cells of clearance"
            );
        }
    }

    /// Every nest has a front door and a back road (play review,
    /// 2026-11-15: 4 of 112 had both): on seeds 1-6 of every planet, the
    /// direct way in is at most `DOOR_MAX` dish cells wide and the way
    /// round at least `ROAD_MIN`. And the door is a door: the direct way
    /// is no narrower than 6, so a squad still files through it.
    #[test]
    fn every_nest_has_a_front_door_and_a_back_road() {
        let mut bad = Vec::new();
        for d in 1..=8 {
            for s in 1..=6 {
                let p = planet(s, d);
                for (k, (direct, round)) in doors(&p, &level(s, d)).into_iter().enumerate() {
                    if !(6..=DOOR_MAX).contains(&direct) || round < ROAD_MIN {
                        bad.push(format!("planet {d} seed {s} nest {k}: direct {direct}, round {round}"));
                    }
                }
            }
        }
        assert!(bad.is_empty(), "{}", bad.join("\n"));
    }
    /// Cave gates stand on dry ground (2026-11-15): on cave seeds 2, 3, 5
    /// and 6, which rolled pools, no soft-rock piece crosses water. Before,
    /// 40 of 163 cave gates over 20 caves had a piece in water (5 on seed 2).
    #[test]
    fn cave_gates_keep_out_of_pools() {
        for s in [2, 3, 5, 6] {
            let m = level(s, 7);
            for wl in &m.items.walls {
                let n = (dist(wl.a, wl.b) as usize).max(1);
                for k in 0..=n {
                    let f = k as f32 / n as f32;
                    let (x, y) = (
                        (wl.a.0 + (wl.b.0 - wl.a.0) * f) as usize,
                        (wl.a.1 + (wl.b.1 - wl.a.1) * f) as usize,
                    );
                    assert!(
                        x >= m.w || y >= m.h || m.ground[y * m.w + x] & G_WET == 0,
                        "cave seed {s}: soft rock {:?}-{:?} in water at ({x}, {y})",
                        wl.a,
                        wl.b
                    );
                }
            }
        }
    }
    /// Rivers keep off the nests (2026-11-15): on river valley seeds 1-6,
    /// no river water (wet, deep or a bridge deck) within `RIVER_OFF` of a
    /// nest, less the 6-cell step its places are tried at. Before, valley
    /// seed 1's second nest had river water 100 cells off (the water
    /// cleared round it), and 19 of 40 valley nests had it within 220.
    #[test]
    fn rivers_keep_off_the_nests() {
        for s in 1..=6 {
            let m = level(s, 4);
            for (k, &q) in m.items.moulds.iter().enumerate() {
                let reach = RIVER_OFF - 8.0;
                let (x0, x1) = ((q.0 - reach).max(0.0) as usize, ((q.0 + reach) as usize).min(m.w));
                let (y0, y1) = ((q.1 - reach).max(0.0) as usize, ((q.1 + reach) as usize).min(m.h));
                let mut nearest = f32::MAX;
                for y in y0..y1 {
                    for x in x0..x1 {
                        let g = m.ground[y * m.w + x];
                        if g & G_ROCK == 0 && (g & G_WET != 0 || g == G_BRIDGE) {
                            nearest = nearest.min(dist((x as f32 + 0.5, y as f32 + 0.5), q));
                        }
                    }
                }
                assert!(
                    nearest >= reach,
                    "valley seed {s} nest {k}: river water {nearest:.0} cells off"
                );
            }
        }
    }
    /// With `Recipe::moat` on, a fen's nests sit in moats (2026-11-15): on
    /// seeds 1-6, at least a tenth of the ground 100-260 map cells from
    /// each nest is deep water (13-30% with the moats, 0% on 5 of 6
    /// without), and the door is a bridge (a deck within 260 cells of the
    /// nest). Off, as on every biome now, no bridge by a meadow's nest.
    #[test]
    fn moats_ring_the_nests_with_a_bridge() {
        let near = |m: &Map, q: (f32, f32), lo: f32, hi: f32, g: u8| {
            let (mut n, mut hit) = (0, 0);
            for y in 0..m.h {
                for x in 0..m.w {
                    let d = dist((x as f32 + 0.5, y as f32 + 0.5), q);
                    if d >= lo && d < hi {
                        n += 1;
                        hit += (m.ground[y * m.w + x] == g) as usize;
                    }
                }
            }
            (hit, n)
        };
        for s in 1..=6 {
            let p = planet(s, 3);
            let m = build_with(
                &p,
                0,
                Some(Recipe {
                    moat: true,
                    ..Recipe::of(&p)
                }),
            );
            for (k, &q) in m.items.moulds.iter().enumerate() {
                let (deep, n) = near(&m, q, 100.0, 260.0, G_DEEP);
                assert!(deep * 10 >= n, "fen seed {s} nest {k}: {deep} of {n} cells deep");
                assert!(
                    near(&m, q, 0.0, 260.0, G_BRIDGE).0 > 0,
                    "fen seed {s} nest {k}: no bridge"
                );
            }
            let m = level(s, 1);
            for &q in &m.items.moulds {
                assert_eq!(
                    near(&m, q, 0.0, 260.0, G_BRIDGE).0,
                    0,
                    "meadow seed {s}: a bridge by the nest"
                );
            }
        }
    }
    /// Nests keep off the map's edges by at least 150 map cells, and two
    /// nests stand at least a fifth of the map's width apart (0.3 wanted;
    /// the spacing gives way only where the far part has no room), over
    /// seeds 1-6 of every planet (reviews, 2026-11-15).
    #[test]
    fn nests_keep_off_the_edges_and_apart() {
        for d in 1..=8 {
            for s in 1..=6 {
                let m = level(s, d);
                let (wf, hf) = (m.w as f32, m.h as f32);
                let ns = &m.items.moulds;
                for &(x, y) in ns {
                    assert!(
                        x.min(y).min(wf - x).min(hf - y) >= 150.0 - 1e-3,
                        "planet {d} seed {s}: a nest at ({x}, {y})"
                    );
                }
                for (i, a) in ns.iter().enumerate() {
                    for b in &ns[i + 1..] {
                        assert!(
                            dist(*a, *b) >= 0.2 * wf,
                            "planet {d} seed {s}: nests {:.0} apart on a map {wf} wide",
                            dist(*a, *b)
                        );
                    }
                }
            }
        }
    }

    /// The fallback, built for every planet of seeds 1-4, passes the checks
    /// the attempts must (technical review, 2026-11-15: 6 of 96 failed
    /// when it kept the biome's fronts, pillars and hedges).
    #[test]
    fn every_fallback_passes_its_checks() {
        let mut bad = Vec::new();
        for d in 1..=8 {
            for s in 1..=4 {
                let p = planet(s, d);
                let fails = check(&p, &fallback(&p));
                if !fails.is_empty() {
                    bad.push(format!("planet {d} seed {s}: {fails:?}"));
                }
            }
        }
        assert!(
            bad.is_empty(),
            "{} of 32 fallbacks fail:\n{}",
            bad.len(),
            bad.join("\n")
        );
    }

    /// The checks walk none of the dish's rim, where the game has no agar
    /// (technical review, 2026-11-15): a map whose only way from the
    /// landing to the crystals is round a ridge's end along the rim fails,
    /// and passes again with the ridge stopping short of the rim's width.
    #[test]
    fn the_checks_walk_inside_the_dish_rim() {
        let (w, h) = (512, 320);
        let ridge = |gap: usize| {
            let mut ground = vec![0u8; w * h];
            for y in gap..h {
                for x in 250..262 {
                    ground[y * w + x] = G_ROCK;
                }
            }
            let crystals = (0..6)
                .map(|k| Crystal {
                    x: 400.0 + 12.0 * k as f32,
                    y: 200.0,
                    r: 6.0,
                    charge: 100.0,
                })
                .collect();
            let items = Items {
                crystals,
                moulds: vec![(450.0, 100.0)],
                landing: Some((100.0, 160.0)),
                walls: Vec::new(),
                moisture: None,
                flora: None,
            };
            Map {
                w,
                h,
                ground,
                plants: Vec::new(),
                items,
            }
        };
        let open = |m: &Map| {
            let (bw, bh, ok) = walkable(m, (w, h), false);
            let d = walk_from(bw, bh, &ok, (100.0, 160.0));
            d[(280 / BIN) * bw + 400 / BIN] != u32::MAX
        };
        assert!(
            !open(&ridge(RIM_MAP)),
            "a ridge from the far edge to the rim's inner side shuts the way"
        );
        assert!(open(&ridge(RIM_MAP + 3 * BIN)), "a way round its end inside the rim");
    }

    /// Each planet's plants: the share of its map cells with any, and the
    /// mean density over all of them (plants are drawn as crowns sized by
    /// density, PR #304), seeds 1-3.
    fn plant_cover(d: u32) -> (f32, f32) {
        let (mut n, mut any, mut sum) = (0usize, 0usize, 0.0f32);
        for seed in 1..=3 {
            let m = level(seed, d);
            for &b in &m.plants {
                let (_, dens) = crate::mould::plants::unpack(b);
                n += 1;
                any += (b != 0) as usize;
                sum += if b != 0 { dens } else { 0.0 };
            }
            n += m.ground.len() - m.plants.len();
        }
        (any as f32 / n as f32, sum / n as f32)
    }

    /// Plants tell the biomes apart (graphics lead on the plant thumbnails,
    /// 11-14 21:56Z: "almost every planet is green from edge to edge"):
    /// bare biomes bare, lush ones lush, from about 5% to 80% cover.
    #[test]
    fn plants_tell_the_biomes_apart() {
        let cover: Vec<(Biome, f32, f32)> = (1..=8)
            .map(|d| {
                let (c, m) = plant_cover(d);
                (Biome::at(d), c, m)
            })
            .collect();
        for &(b, c, m) in &cover {
            eprintln!("{:>12}: {:.1}% covered, mean density {:.3}", b.name(), 100.0 * c, m);
        }
        let of = |b: Biome| cover.iter().find(|x| x.0 == b).expect("every biome").1;
        let barrens = of(Biome::Barrens);
        assert!((0.06..0.2).contains(&barrens), "barrens: sparse scrub, {barrens}");
        assert!(of(Biome::Caves) < 0.08, "cave: next to none, {}", of(Biome::Caves));
        assert!(
            of(Biome::Highlands) < 0.4,
            "highlands: patches, {}",
            of(Biome::Highlands)
        );
        assert!(
            of(Biome::Meadow) > 0.6 && of(Biome::Jungle) > 0.6,
            "meadow and jungle lush: {cover:?}"
        );
        // On the highlands nothing but reeds grows off clay.
        for seed in 1..=3 {
            let m = level(seed, 6);
            assert_eq!(planet(seed, 6).biome, Biome::Highlands);
            for (i, &b) in m.plants.iter().enumerate() {
                let (kind, _) = crate::mould::plants::unpack(b);
                assert!(
                    b == 0 || kind == crate::mould::plants::REEDS || m.ground[i] >> 4 == 2,
                    "seed {seed} cell {i}: {b:#x} on {:#x}",
                    m.ground[i]
                );
            }
        }
    }

    #[test]
    fn biomes_come_in_the_set_order_with_the_core_world_last() {
        let got: Vec<Biome> = (1..=9).map(|d| planet(1, d).biome).collect();
        assert_eq!(
            got,
            [
                Biome::Meadow,
                Biome::Barrens,
                Biome::Fen,
                Biome::Valley,
                Biome::Jungle,
                Biome::Highlands,
                Biome::Caves,
                Biome::Core,
                Biome::Meadow
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
                // A jackpot or oases planet has few crystals by design, and
                // planet 1 is 60% of today's area: `check`'s six at least.
                assert!(
                    s.crystals >= 6 && s.unreachable == 0,
                    "seed {seed} depth {depth}: {s:?}"
                );
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
                let p = Planet {
                    size: size_of(area),
                    ..planet(3, depth)
                };
                let k = (0..ATTEMPTS).find(|&a| check(&p, &build(&p, a)).is_empty());
                assert!(k.is_some(), "depth {depth} area {area}: {:?}", check(&p, &build(&p, 0)));
                let s = stats(&p, &generate(&p));
                assert!(
                    s.unreachable == 0 && s.crystals >= 6,
                    "depth {depth} area {area}: {s:?}"
                );
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
        assert!(spec("planet:1:2:3:4", 1).unwrap().is_err());
        assert!(spec("planet:1:2:9", 1).unwrap().is_err());
        assert_eq!(spec_planet("planet:3:9:2", 1).unwrap().unwrap().size, size_of(2.0));
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
