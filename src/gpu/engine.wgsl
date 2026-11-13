// One species' own settings (`BigParams::species_sets`, `Params::sets`):
// sensor angle and reach, turn and step, scent reach, deposit, keep,
// blocked share, grazing weight, crowd limit and explorer stage.
struct Set {
    sa: f32, so: f32, ra: f32, st: f32,
    sso: f32, deposit: f32, keep: f32, bdep: f32,
    graze_w: f32, crowd: u32, explore: u32, _p: u32,
};
struct Params {
    w: u32, h: u32, n: u32, crowd: u32,
    s_turn: u32, s_blocked: u32, s_claim: u32, s_jump: u32,
    sa: f32, ra: f32, so: f32, sso: f32,
    st: f32, deposit: f32, keep: f32, scent_keep: f32,
    respawn: f32, pull: f32, bx: f32, by: f32,
    br: f32, knee: f32, gamma: f32, s_pull: u32,
    wipe: u32, colour: u32, field: u32, curve: u32,
    tube: f32, fed: f32, cap: u32, s_wander: u32,
    // Jenson's looks (`BigParams`, 2026-11-11): settings set by the trail a
    // walker stands in (`sense_scale`, each `*_mod` as amp and exponent),
    // the sensing floor and wander.
    ss: f32, om_a: f32, om_e: f32, am_a: f32,
    am_e: f32, tm_a: f32, tm_e: f32, sm_a: f32,
    sm_e: f32, sfloor: f32, wander: f32, catch_at: f32,
    // Contagion: off while `sick_steps` is 0. `infect` makes every walker
    // within `ir` of (`ix`, `iy`) sick at the tick's start.
    sick_steps: u32, immune: u32, infect: u32, _p0: u32,
    sick_dep: f32, sick_keep: f32, sick_speed: f32, sick_trail: f32,
    sick_reach: f32, sick_turn: f32, ix: f32, iy: f32,
    ir: f32, _p1: f32, _p2: f32, _p3: f32,
    // The outside rules (`BigParams` of PR #55, Reports/outside-rules-2026-11-11.md),
    // all off at 0: species fields (`nsp`, 0 for none, else 2-4), cyclic
    // species, explorers (`explore` steps), blocked walkers' deposit share
    // and kept heading, the chance turn's power, size spread, area sensors.
    nsp: u32, cycle: u32, explore: u32, s_size: u32,
    avoid: f32, snext: f32, sprev: f32, edep: f32,
    espread: f32, bshare: f32, bkeep: u32, breset: f32,
    chance: f32, size_sp: f32, sensor_r: u32, s_chance: u32,
    // PR #56: a walker joins a species whose trail at its cell outweighs
    // its own `convert` times (0 off); grazing (`graze_w` 0 off). PR #58:
    // species 0 lays `boost` times the deposit.
    s_keep: u32, convert: f32, graze_w: f32, graze_eat: f32,
    graze_regrow: f32, boost: f32, upkeep: f32, birth_at: f32,
    // Growth (Larder World W0, `BigParams::upkeep`, 0 off): births cost
    // `birth_cost`; a walker at zero energy dies with chance `starve` a
    // step; a child takes slot k ^ (a keyed draw & `bmask`). `wither` and
    // `s_wither` (the row's spare words, borrowed by the slime-enemy
    // thread 2026-11-13): a walker that has heard of no food for its lag
    // dies with chance `wither` at each check (`life`; clock in `wclock`).
    birth_cost: f32, starve: f32, s_starve: u32, s_birth: u32,
    s_bclaim: u32, bmask: u32, wither: f32, s_wither: u32,
    // The Larder World (W1b, 2026-11-11: `BigParams`' crystals, terrain and
    // walls), off while `world` is 0. Its bits: CRYSTALS, WETS (wet ground,
    // or rubble a broken wall leaves), WALLS (a wall has been built),
    // TERRAIN (rock or wet ground to draw). `full` is `crystal_charge`;
    // `npc` the crystal cells `pcell` lists; `hp` the walls' full health.
    // `grid` is the mould grid's square, fine cells (`count`; 0 for none).
    world: u32, npiles: u32, npc: u32, grid: u32,
    full: f32, eat: f32, flow: f32, regrow: f32,
    wet_step: f32, bite: f32, wall_scent: f32, hp: f32,
    // The squad (Larder World W2a, `Engine::set_squad`): this frame's units
    // and shots in `SQ`, the trail that looks like slime, and the alarm a
    // hit lays with the alarm field on (`squad::SHOT_ALARM`).
    nunits: u32, nshots: u32, slime: f32, shot_alarm: f32,
    // The look's hits (`Engine::set_hits`, the world look's stand-in fight).
    // The player's own mould (`own_mould`, world bit GARDEN): its tend
    // entries this frame and where they start in `SQ.path` (`squad::TEND`).
    // Then the shots' sink (`squad::Frame::sink`, squared; 0 off): a hit's
    // blast sinks up to floor(`shot_sink` x its strike's r2) along its line.
    nhits: u32, ntend: u32, tend_at: u32, shot_sink: f32,
    // The species block (2026-11-11, the creature thread's species on the
    // GPU): whether each species uses its own `sets` entry (`per`), the
    // walker kinds (`mix_share`: 2 with the mix on, a walker's kind in bit
    // 8 of its `spec` word), conversion's square (`species_convert_r`), and
    // the second kind's multipliers (`mix_*`).
    // Two spare words of this row are a cut-off part's panic (11-13, the
    // slime-enemy thread, `BigParams::panic`): `panic_r2` the charge's
    // squared radius in fine cells, `panic` the haste (0 off).
    per: u32, kinds: u32, conv_r: u32, panic_r2: u32,
    mreach: f32, mangle: f32, mturn: f32, mstep: f32,
    mdep: f32, panic: f32, panic_creep: f32, panic_dash: u32,
    sets: array<Set, 4>,
    // The life cycle (2026-11-11, `BigParams::spore_rate` and below: the
    // tiers' Sporer, Sleeper and Swarmer, Reports/mould-tiers-2026-11-11.md).
    // `cyc` is on once any of its rules is or a strike has landed (the
    // CPU's `life` states, allocated then and kept); `alarm_on` with the
    // alarm field (`alarm_keep` > 0). A walker's state and timer are in its
    // `spec` word (`LIFE_*` below); the alarm is the germ fields' second
    // block, its relays and the cells' dormant walkers `gdep`'s second and
    // third, and spores' and sleepers' deposits `bdep`'s third and fourth.
    // `sleep_death` (PR #85) is a dormant walker's chance a tick of dying.
    // `wclock` (the row's spare word, borrowed by the slime-enemy thread
    // 2026-11-13) is withering's: its lag in steps (low 16 bits), the steps
    // between checks (next 8) and whether the news spread ends in its
    // second block (bit 24; `Engine::wither_clock`).
    cyc: u32, alarm_on: u32, s_fruit: u32, s_sleep: u32,
    s_land: u32, s_spore_h: u32, spore_steps: u32, spore_rest: u32,
    wake_steps: u32, sleep_rest: u32, s_sleep_die: u32, wclock: u32,
    spore_rate: f32, spore_scent: f32, spore_alarm: f32, spore_burst: f32,
    spore_st: f32, spore_reach: f32, spore_dep: f32, spore_land: f32,
    sleep_rate: f32, sleep_dep: f32, wake_scent: f32, wake_alarm: f32,
    alarm_keep: f32, alarm_w: f32, relay: f32, sleep_death: f32,
    // A strike (`Engine::strike`, the CPU's `BigDish::strike`), in fine
    // cells: its centre, kill radius, alarm and alarm radius; with a
    // toolkit burst (`Engine::burst`) also its crater's radius and fine
    // steps (the CPU's `BigDish::scorch_disc`; 0 lays none).
    kx: f32, ky: f32, kr: f32, kalarm: f32,
    kar: f32, kcr: f32, kcs: f32, _l5: f32,
    // Soil (WM3, 2026-11-12; `BigParams::sand_keep` and on), read with
    // `world(SOIL)`: the trail on sand and on clay is x these after the
    // spread, and on barren ground upkeep is x `barren_up`. The row's last
    // word is one body's `eloss` (2026-11-13, `BigParams::flow_loss`; the
    // row had a spare word, so Params keeps its size): in `egain` what a
    // cell takes from a richer neighbour arrives x 1 - `eloss`.
    sand_keep: f32, clay_keep: f32, barren_up: f32, eloss: f32,
    // One body (2026-11-12, `BigParams::energy_flow`, `hunger_scent`,
    // `hunger_at`; Reports/one-body-2026-11-12.md), read with growth on:
    // energy evens out between neighbouring cells (`esum`, `egain`), and a
    // hungry walker weighs the scent 1 + `hscent` x its hunger. A walker
    // with `fed_full` energy or more steps and breeds under the crowd limit
    // + `fed_crowd` (`BigParams::fed_crowd`, the lone row's last; `crowd_for`).
    eflow: f32, hscent: f32, hat: f32, fed_full: f32,
    // The crater (2026-11-12, `BigParams::scorch_keep`, `scorch_full`;
    // `squad::Frame::crater`): while a cell's countdown runs, its trail is
    // x 1 - (1 - `scorch_keep`) x min(1, steps left / `scorch_full`) after
    // the spread and the soil, and the countdown drops by one, once
    // `scorched` (flag bit 0; the CPU's `tick` once `scorch` is made). A
    // shot's hit sets the countdown on its struck agar cells to at least
    // `crater`, or with the cone (bit 1) fewer steps toward the rim.
    scorch_keep: f32, scorch_full: f32, crater: f32, scorched: u32,
    // Lone step (2026-11-12, `BigParams::lone_step`): a walker with n other
    // walkers in the square of radius `lone_r` round its cell, as the step
    // began, steps x clamp(n / `lone`, `lfloor`, 1). 0 off.
    lone: f32, lfloor: f32, lone_r: u32, fed_crowd: u32,
};
struct State { t: u32, top: f32, _a: u32, _b: u32 };
// A pile's crystal: its run of `pcell` (`start`, `count`), and from the
// last `pilesum` its cells still crystal, its charge and the share left
// (the CPU's `crystal_left`; 1 for a pile with no crystal left to hold).
// `full` is the pile's charge a full cell (`BigDish::pile_fill`: a map
// gives each crystal its own, else every pile's is `crystal_charge`).
// `centre`: the pile's centre as a fine cell, x | y << 16 (`BigDish::
// pile_centre`), where a panicking walker runs.
struct Pile { start: u32, count: u32, cells: u32, centre: u32, sum: f32, left: f32, full: f32, _b: f32 };
// A wall being built (`Engine::build_wall`): the segment and its half
// width in fine cells, the health it starts with, the box of cells the
// `build` pass covers, whether it lets scent through, and how far a
// walker inside looks for a free cell (`evict`).
struct WallBuild { ax: f32, ay: f32, bx: f32, by: f32, r: f32, hp: f32, x0: u32, y0: u32, bw: u32, bh: u32, scent: u32, reach: u32 };

@group(0) @binding(0) var<uniform> P: Params;
@group(0) @binding(1) var<storage, read_write> S: State;
@group(0) @binding(2) var<storage, read_write> walkers: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read_write> intent: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> icell: array<u32>;
@group(0) @binding(5) var<storage, read_write> occ: array<atomic<u32>>;
@group(0) @binding(6) var<storage, read_write> claim: array<atomic<u32>>;
@group(0) @binding(7) var<storage, read_write> dep: array<atomic<u32>>;
// Writable for the Larder World: a wall leaves the agar, rubble comes back.
@group(0) @binding(8) var<storage, read_write> agar: array<u32>;
@group(0) @binding(9) var<storage, read> lay: array<f32>;
@group(0) @binding(10) var<storage, read_write> trail_in: array<f32>;
@group(0) @binding(11) var<storage, read_write> trail_out: array<f32>;
@group(0) @binding(12) var<storage, read_write> scent_in: array<f32>;
@group(0) @binding(13) var<storage, read_write> scent_out: array<f32>;
@group(0) @binding(14) var<storage, read_write> hist: array<atomic<u32>>;
@group(0) @binding(15) var<storage, read_write> frame: array<u32>;
@group(0) @binding(16) var<storage, read> hud: array<u32>;
// Which pile a cell's food belongs to (index + 1, 0 for none), and whether
// the tube has reached each pile (the CPU's `BigDish::reached`).
@group(0) @binding(17) var<storage, read> pile: array<u32>;
@group(0) @binding(18) var<storage, read_write> reached: array<atomic<u32>>;
// Contagion (the CPU's `BigDish::sick` and `germ`): each walker's counter
// (0 healthy, over `immune` sick, 1..=`immune` immune), sick walkers' trail
// a cell (`sdep`, as `dep` counts the rest), sick walkers a cell for the
// germ (`gdep`), and the germ field.
@group(0) @binding(19) var<storage, read_write> sick: array<u32>;
@group(0) @binding(20) var<storage, read_write> sdep: array<atomic<u32>>;
@group(0) @binding(21) var<storage, read_write> gdep: array<atomic<u32>>;
@group(0) @binding(22) var<storage, read_write> germ_in: array<f32>;
@group(0) @binding(23) var<storage, read_write> germ_out: array<f32>;
// The outside rules. `dep` and `sdep` hold a block of cells a species
// (species s at s x cells); explorers count apart in `edep` (healthy, then
// sick, a block each per species), and with `bshare` blocked walkers in
// `bdep` (ordinary, then explorers). Each species' trail field ping-pongs
// as the trail does (`sp_in`, `sp_out`), and each walker's age counts up
// a step while explorers are on (the CPU's `BigDish::age`).
@group(0) @binding(24) var<storage, read_write> edep: array<atomic<u32>>;
@group(0) @binding(25) var<storage, read_write> bdep: array<atomic<u32>>;
@group(0) @binding(26) var<storage, read_write> sp_in: array<f32>;
@group(0) @binding(27) var<storage, read_write> sp_out: array<f32>;
@group(0) @binding(28) var<storage, read_write> age: array<u32>;
// PR #56: the grass a cell (1 on agar at the start; eaten and regrown in
// place by the trail pass), walkers a cell after the move for the eating,
// and each walker's species (k % n at the start; `convert` changes it).
@group(0) @binding(29) var<storage, read_write> grass: array<f32>;
@group(0) @binding(30) var<storage, read_write> gcount: array<atomic<u32>>;
@group(0) @binding(31) var<storage, read_write> spec: array<u32>;
// Growth: each slot's energy, below 0 for an empty slot (the CPU's `alive`
// false). Read only while growth is on; one a slot.
@group(0) @binding(32) var<storage, read_write> energy: array<f32>;
// What each walker on a cell took from its crystal this tick (the CPU's
// `took`: a cell's n living walkers take equal shares), written by the
// Larder World's `eat` pass for crystal cells (0 elsewhere) and added to
// energy in `life`. A cell, not a slot: `life` reads it where the walker
// stands, and an empty slot never runs `life`.
@group(0) @binding(33) var<storage, read_write> share: array<f32>;
// The Larder World (W1b). `ground` a cell: its crystal's pile in the low
// byte (index + 1, 0 none: the CPU's `pile_of`), and the bits below.
// `charge` a crystal cell's charge, `health` a wall cell's; `bites` the
// blocked walkers that hit a wall cell this tick; `pcell` lists
// the crystal cells pile by pile, `piles` each pile's run and sums, and
// `wstat` counts bites (0) and broken cells (1). `wb` is the wall a
// `build` pass is building.
@group(0) @binding(34) var<storage, read_write> ground: array<u32>;
@group(0) @binding(35) var<storage, read_write> charge: array<f32>;
@group(0) @binding(36) var<storage, read_write> health: array<f32>;
@group(0) @binding(37) var<storage, read_write> bites: array<atomic<u32>>;
@group(0) @binding(38) var<storage, read> pcell: array<u32>;
@group(0) @binding(39) var<storage, read_write> piles: array<Pile>;
@group(0) @binding(40) var<storage, read_write> wstat: array<atomic<u32>>;
@group(0) @binding(41) var<storage, read> wb: WallBuild;
// The squad (W2a; `src/mould/squad.rs` holds the CPU twins, the order of a
// frame and the layout, `squad::pack`): units in and out, shots in and out,
// and the cells each shot's line crosses. A unit's cell and its squared
// radii are whole numbers, so both sides test the same integers. `kills`
// counts the walkers each shot killed.
struct SqUnit { x: u32, y: u32, touch2: u32, sight2: u32, take2: u32, take: f32, smell2: u32, smell: f32 };
struct SqUnitOut { touching: u32, aim: u32, took: f32, dense: u32 };
// A shot's `r2` is its strike's, `ar2` its alarm's (`squad::SHOT_ALARM_R`).
struct SqShot { start: u32, count: u32, r2: u32, ar2: u32 };
struct SqShotOut { hit: u32, _a: u32, _b: u32, _c: u32 };
struct Squad { units: array<SqUnit, 96>, uout: array<SqUnitOut, 96>, shots: array<SqShot, 256>, sout: array<SqShotOut, 256>, path: array<u32> };
@group(0) @binding(42) var<storage, read_write> SQ: Squad;
@group(0) @binding(43) var<storage, read_write> kills: array<atomic<u32>>;
// The look (2026-11-11, src/world/look in the view): shots that landed this
// frame, `nhits` discs (x, y, r, _) in fine cells, which cut trail as the
// right mouse's wipe does; and a u32 a cell the view shades the world from:
// mould thickness (the trail's brightness), walkers on the cell, the wound
// (the thickness a hit took away, fading), and the scent, a byte each.
@group(0) @binding(44) var<storage, read> hits: array<vec4<f32>>;
@group(0) @binding(45) var<storage, read_write> look: array<u32>;
// One body's sums (`energy_flow`): a cell's ordinary walkers' energy in
// fixed point (x `ESCALE`) in the first block of cells, their count in the
// second. Fixed point, so the sum does not depend on the order the walkers
// add in; `life` clears each cell after `egain` has read it.
@group(0) @binding(46) var<storage, read_write> body: array<atomic<u32>>;
// The damage memory (the squad toolkit's layout, 2026-11-12): the crater
// countdown a cell in fine steps, whole numbers in f32 (the CPU's u16
// `BigDish::scorch`), in the first block; the tiers thread's scar in the
// second, unused so far. Held as the f32's bits in atomics (`crater_at`,
// `crater_set`), so `cut` can take the larger of two overlapping cones'
// steps with `atomicMax` (2026-11-12, the technical lead): a non-negative
// float's bits sort as the float does. Before, `cut` wrote
// `max(crater[i], steps)` from a workgroup a shot, a race of the kind that
// lost harvest's adds on the GTX 1070 (`Reports/dead-ends.md`): two cones
// meeting on a cell could keep the smaller.
@group(0) @binding(47) var<storage, read_write> crater: array<atomic<u32>>;
// Food's news (`Settings::wither`, the CPU's `BigDish::news` and `heard`,
// 2026-11-13): the step each slot last heard of food, `P.n` words; then
// three blocks of cells: the latest news the walkers on each cell carry
// (`ngather`, cleared by `nhear`), and the spread's two turns (`nspread`,
// `nab`, `nba`), each written whole by the pass that writes it.
@group(0) @binding(48) var<storage, read_write> news: array<atomic<u32>>;

fn crater_at(i: u32) -> f32 { return bitcast<f32>(atomicLoad(&crater[i])); }
fn crater_set(i: u32, v: f32) { atomicStore(&crater[i], bitcast<u32>(v)); }
const ESCALE: f32 = 65536.0;

const NONE: u32 = 0xFFFFFFFFu;

// The outside rules' switches, read through these: the pipelines built with
// OUTSIDE false (every outside rule off, which `Engine::write_params` picks
// when none is on) compile the rules out. With them in, every pass paid for
// their branches even when off: on the software GPU, 24% more GPU time a
// frame on key 1 and 18% on key 4 (3 of 3 rounds each, 2026-11-11).
override OUTSIDE: bool = true;
fn nsp() -> u32 { if (OUTSIDE) { return P.nsp; } return 0u; }
fn explore() -> u32 { if (OUTSIDE) { return P.explore; } return 0u; }
fn bshare() -> f32 { if (OUTSIDE) { return P.bshare; } return 0.0; }
fn bkeep() -> u32 { if (OUTSIDE) { return P.bkeep; } return 0u; }
fn chance() -> f32 { if (OUTSIDE) { return P.chance; } return 0.0; }
fn size_sp() -> f32 { if (OUTSIDE) { return P.size_sp; } return 0.0; }
fn sensor_r() -> u32 { if (OUTSIDE) { return P.sensor_r; } return 0u; }
fn graze_w() -> f32 { if (OUTSIDE) { return P.graze_w; } return 0.0; }
fn growth() -> bool { if (OUTSIDE) { return P.upkeep > 0.0; } return false; }
fn per() -> bool { if (OUTSIDE) { return P.per != 0u; } return false; }
fn mixes() -> bool { if (OUTSIDE) { return P.kinds > 1u; } return false; }
fn conv_r() -> u32 { if (OUTSIDE) { return P.conv_r; } return 0u; }
// A species' own settings where it has them (`per`), else the global ones.
// With `per` the globals `explore`, `bshare` and `graze_w` are switches
// only (on if any species has one).
fn crowd_of(sp: u32) -> u32 { if (per()) { return P.sets[sp].crowd; } return P.crowd; }
// Walker k's crowd limit where it steps or breeds: its species' (`sp`), or
// that + `fed_crowd` while it holds `fed_full` energy or more (the CPU's
// `fed_crowd`; a species with no limit keeps none). Claims still follow the
// species' own limit (`walk`, `born`), as on the CPU.
fn crowd_for(k: u32, sp: u32) -> u32 {
    let c = crowd_of(sp);
    if (c > 0u && P.fed_crowd > 0u && growth() && energy[k] >= P.fed_full) { return c + P.fed_crowd; }
    return c;
}
fn deposit_of(sp: u32) -> f32 { if (per()) { return P.sets[sp].deposit; } return P.deposit; }
fn keep_of(sp: u32) -> f32 { if (per()) { return P.sets[sp].keep; } return P.keep; }
fn bshare_of(sp: u32) -> f32 { if (per()) { return P.sets[sp].bdep; } return bshare(); }
fn graze_of(sp: u32) -> f32 { if (per()) { return P.sets[sp].graze_w; } return graze_w(); }
fn explore_of(sp: u32) -> u32 { if (per()) { return P.sets[sp].explore; } return explore(); }
fn cycles() -> bool { if (OUTSIDE) { return P.cyc != 0u; } return false; }
// The alarm field is on with `alarm_keep` over 0, with or without the
// states, as the CPU makes it with the dish: an alarm-only dish (the
// Swarmer) senses and spreads the alarm a shot lays. (It once also needed
// the states, which only a strike or spores or dormancy switch on, so a
// shot's alarm would have gone nowhere on such a dish.)
fn alarms() -> bool { if (OUTSIDE) { return P.alarm_on != 0u; } return false; }

// The life cycle's word in `spec`: bits 9-10 the state (the CPU's
// `LIFE_WALKER` 0, `LIFE_SPORE` 1, `LIFE_DORMANT` 2, `LIFE_DEAD` 3), bits
// 16-31 the timer (`life_t`), and two marks for this tick: `FRUIT`, set by
// `sense` for a walker that fruits and kept by `fate` on the new spore
// until `fly` (which it skips: it flies from the next tick), and `DOZE`,
// set by `sense` for one that falls dormant, cleared by `fate`.
const L_WALKER: u32 = 0u;
const L_SPORE: u32 = 1u;
const L_DORMANT: u32 = 2u;
const L_DEAD: u32 = 3u;
const FRUIT: u32 = 0x800u;
const DOZE: u32 = 0x1000u;
fn state(k: u32) -> u32 { if (!cycles()) { return L_WALKER; } return (spec[k] >> 9u) & 3u; }
// An ordinary walker (the CPU's `active`): living, and not a spore,
// dormant or dead.
fn ordinary(k: u32) -> bool { return lives(k) && state(k) == L_WALKER; }
// The alarm on cell i (0 with no alarm field).
fn alarm_at(i: u32) -> f32 {
    if (!alarms() || i == NONE) { return 0.0; }
    return germ_in[P.w * P.h + i];
}
// Whether slot k holds a living walker (always, with growth off).
fn lives(k: u32) -> bool { return !growth() || energy[k] >= 0.0; }
// The walkers eat the grass with grazing or growth on (the CPU makes the
// grass field for either), but not growth on crystals with grazing off:
// the CPU makes no grass then, and here it would regrow from 0 and feed
// the walkers (the world twin test: grass 306 after one tick).
fn grazes() -> bool { return graze_w() != 0.0 || (growth() && !world(CRYSTALS)); }
const TAU: f32 = 6.2831855;

// The Larder World's switches (`P.world`) and a cell's `ground` bits.
const CRYSTALS: u32 = 1u;
const WETS: u32 = 2u;
const WALLS: u32 = 4u;
const TERRAIN: u32 = 8u;
const SOIL: u32 = 16u;
// The gardeners' world (`own_mould`, 2026-11-12): the player's own mould is
// species `OWN_SP`, and the squad's passes see the enemy alone: a shot stops
// at the enemy's trail (the trail less the player's), strikes only the
// enemy's walkers and clears only its trail, and a unit's reads count the
// enemy's walkers (`occ` less the player's, counted into `gcount`, which
// only a grazing world uses: `GpuWorld::garden` refuses one). Eating runs
// on every cell, so charge the gardeners spread on open ground is eaten as
// the CPU's eat does (on the CPU any cell's charge is food).
const GARDEN: u32 = 32u;
const OWN_SP: u32 = 3u;
const PILE_BITS: u32 = 0xFFu;
const WET: u32 = 0x100u;
const WALL: u32 = 0x200u;
const SCENT_WALL: u32 = 0x400u;
const ROCK: u32 = 0x800u;
fn world(b: u32) -> bool { return (P.world & b) != 0u; }
fn garden() -> bool { return world(GARDEN); }
// The enemy's trail and walkers on cell i (all of them outside a garden).
fn enemy_trail(i: u32) -> f32 {
    if (garden()) { return trail_in[i] - sp_in[OWN_SP * P.w * P.h + i]; }
    return trail_in[i];
}
fn enemy_occ(i: u32) -> u32 {
    if (garden()) { return atomicLoad(&occ[i]) - atomicLoad(&gcount[i]); }
    return atomicLoad(&occ[i]);
}
// A cell's soil kind (the ground word's bits 12-15; big.rs `SOIL_SAND`
// and on), 0 (plain) with the dish's soil off.
const SOIL_SAND: u32 = 1u;
const SOIL_CLAY: u32 = 2u;
const SOIL_BARREN: u32 = 3u;
fn soil(i: u32) -> u32 {
    if (!world(SOIL)) { return 0u; }
    return (ground[i] >> 12u) & 15u;
}
// What the trail on cell i keeps of its spread value (the CPU's `tick`
// multiplies after the spread, and the trail after the species' sum).
fn soil_keep(i: u32) -> f32 {
    let k = soil(i);
    if (k == SOIL_SAND) { return P.sand_keep; }
    if (k == SOIL_CLAY) { return P.clay_keep; }
    return 1.0;
}
// Whether scent spreads through cell j: agar, or a wall that lets it
// through (`wall_scent` > 0; the CPU's `scent_agar` on the fine grid).
fn open(j: u32) -> bool {
    if (agar[j] != 0u) { return true; }
    return world(WALLS) && (ground[j] & SCENT_WALL) != 0u;
}

fn lowbias32(v: u32) -> u32 {
    var x = v;
    x ^= x >> 16u; x *= 0x7FEB352Du; x ^= x >> 15u; x *= 0x846CA68Bu;
    return x ^ (x >> 16u);
}
// The CPU's `keyed`, with the seed and use folded into `s` on the CPU.
fn keyed(s: u32, t: u32, walker: u32) -> u32 {
    return lowbias32(walker ^ lowbias32(lowbias32(s) ^ t));
}
fn unit(x: u32) -> f32 { return f32(x >> 8u) / 16777216.0; }

fn cell(x: f32, y: f32) -> u32 {
    if (x < 0.0 || y < 0.0) { return NONE; }
    let cx = u32(x); let cy = u32(y);
    if (cx >= P.w || cy >= P.h) { return NONE; }
    return cy * P.w + cx;
}
fn scent_at(x: f32, y: f32) -> f32 {
    let i = cell(x, y);
    if (i == NONE) { return 0.0; }
    return scent_in[i];
}
// Species fields in use: one block of cells each (one, the plain one, when
// species are off).
fn nsp1() -> u32 { return max(nsp(), 1u); }
// The count blocks: one a species and kind (kind 1's after kind 0's).
fn nblk() -> u32 { return nsp1() * select(1u, 2u, mixes()); }
fn blk(sp: u32, kd: u32) -> u32 { return kd * nsp1() + sp; }

// The trail a walker of species `sp` reads at cell j: the trail, or its
// own field weighed against the others' (the CPU's `trail_seen`, its
// expressions in its order).
fn seen(j: u32, sp: u32) -> f32 {
    if (nsp() == 0u) { return trail_in[j]; }
    let c = P.w * P.h;
    let own = sp_in[sp * c + j];
    if (P.cycle != 0u && nsp() >= 3u) {
        let nx = sp_in[((sp + 1u) % nsp()) * c + j];
        let pv = sp_in[((sp + nsp() - 1u) % nsp()) * c + j];
        return own + P.snext * nx + P.sprev * pv - P.avoid * (trail_in[j] - own - nx - pv);
    }
    return (1.0 + P.avoid) * own - P.avoid * trail_in[j];
}

// One cell, or with `sensor_r` the sum over the agar cells of the square
// around it, row by row as the CPU sums it (the centre decides the wall).
fn trail_at(i: u32, sp: u32) -> f32 {
    let r = sensor_r();
    if (r == 0u) { return seen(i, sp); }
    let x = i % P.w; let y = i / P.w;
    var sum = 0.0;
    for (var yy = y - min(y, r); yy < min(y + r + 1u, P.h); yy++) {
        for (var xx = x - min(x, r); xx < min(x + r + 1u, P.w); xx++) {
            let j = yy * P.w + xx;
            if (agar[j] != 0u) { sum += seen(j, sp); }
        }
    }
    return sum;
}

fn sense_at(x: f32, y: f32, a: f32, so: f32, sso: f32, sp: u32, gw: f32, sw: f32) -> f32 {
    let c = cos(a); let s = sin(a);
    let i = cell(x + c * so, y + s * so);
    if (i == NONE || agar[i] == 0u) { return -1.0e6; }
    // `sw` is 1, or more for a hungry walker (`hunger_scent`); x 1 is exact.
    var v = trail_at(i, sp) + sw * scent_at(x + c * sso, y + s * sso);
    // Grazing: fresh grass smells good (the CPU's `graze_weight`, the
    // walker's species' own with `per`).
    if (gw != 0.0) { v += gw * grass[i]; }
    // The alarm (`alarm_weight`): over 0 the mould swarms to a wound,
    // under 0 it flees one.
    if (alarms()) { v += P.alarm_w * alarm_at(i); }
    return v;
}

// Walker k's species: its own (`spec`'s low byte, which conversion
// changes), or the one plain field.
fn species_of(k: u32) -> u32 {
    if (nsp() == 0u) { return 0u; }
    return spec[k] & 0xFFu;
}

// Walker k's kind (`mix_share`): 1 for the second kind, fixed for its life
// and passed to its children.
fn kind_of(k: u32) -> u32 {
    if (!mixes()) { return 0u; }
    return (spec[k] >> 8u) & 1u;
}

// Whether walker k eats grass: everyone, but with species' own settings
// only a species that grazes, unless growth feeds on the grass (no
// crystals), as the CPU's `eats`.
fn eats(k: u32) -> bool {
    if (!per()) { return true; }
    return (growth() && !world(CRYSTALS)) || P.sets[species_of(k)].graze_w != 0.0;
}

// Whether walker k is still an explorer: younger than `explore` steps, or
// with `espread` its own share of them, from a draw on k alone (the CPU's
// `explores`).
fn explores(k: u32) -> bool {
    var n = f32(explore_of(species_of(k)));
    if (P.espread > 0.0) {
        let u = unit(keyed(P.s_size, 0u, k | 0x80000000u));
        n *= 1.0 + P.espread * (2.0 * u - 1.0);
    }
    return f32(age[k]) < n;
}

// v^e as the CPU's `powf` gives it at v = 0 (0, or 1 when e is 0); WGSL's
// `pow` leaves 0 undefined.
fn pow0(v: f32, e: f32) -> f32 {
    if (v <= 0.0) { return select(0.0, 1.0, e == 0.0); }
    return pow(v, e);
}

// A cut-off part panics (`BigParams::panic`, the CPU's `panic_haste` and
// `panic_target`): a walker whose news is older than the lag runs for the
// nearest unit of the squad within `panic_r2` (ties to the first), else
// for the nearest crystal with more than `PANIC_LEFT` of its charge left;
// whole cells, so both engines pick the same. For `wclock`'s lean (bits
// 25-31) after the lag runs out it steers there at `panic_creep` of its
// pace (the CPU clamps it above 0), then `panic` times as fast; a charge
// at a unit bolts only for `panic_dash` steps (0 always), then creeps again. (x, y, haste); (-1, -1, 0) when calm.
const PANIC_LEFT: f32 = 0.02;
fn panic_to(k: u32, here: u32) -> vec3<f32> {
    if (P.panic <= 0.0) { return vec3<f32>(-1.0, -1.0, 0.0); }
    let heard = atomicLoad(&news[k]);
    // The age of its news (S.t + 1 is never before it).
    let age = S.t + 1u - min(heard, S.t + 1u);
    let lag = P.wclock & 0xFFFFu;
    let lean = (P.wclock >> 25u) & 0x7Fu;
    var haste = 0.0;
    if (age > lag + lean) { haste = P.panic; } else if (age > lag) { haste = P.panic_creep; }
    if (haste <= 0.0) { return vec3<f32>(-1.0, -1.0, 0.0); }
    let cx = i32(here % P.w);
    let cy = i32(here / P.w);
    var best = -1;
    var bx = 0;
    var by = 0;
    for (var u = 0u; u < P.nunits; u++) {
        let dx = i32(SQ.units[u].x) - cx;
        let dy = i32(SQ.units[u].y) - cy;
        let d2 = dx * dx + dy * dy;
        if (u32(d2) <= P.panic_r2 && (best < 0 || d2 < best)) { best = d2; bx = i32(SQ.units[u].x); by = i32(SQ.units[u].y); }
    }
    // The charge gives up (`BigParams::panic_dash`): past the lean and the
    // dash, a walker charging a unit creeps on.
    if (best >= 0 && P.panic_dash > 0u && age > lag + lean + P.panic_dash) { haste = P.panic_creep; }
    if (best < 0 && world(CRYSTALS)) {
        for (var p = 0u; p < P.npiles; p++) {
            if (piles[p].left <= PANIC_LEFT) { continue; }
            let c = piles[p].centre;
            let px = i32(c & 0xFFFFu);
            let py = i32(c >> 16u);
            let d2 = (px - cx) * (px - cx) + (py - cy) * (py - cy);
            if (best < 0 || d2 < best) { best = d2; bx = px; by = py; }
        }
    }
    if (best < 0) { return vec3<f32>(-1.0, -1.0, 0.0); }
    return vec3<f32>(f32(bx) + 0.5, f32(by) + 0.5, haste);
}

// A panicking walker's sensor: how far heading `a` points toward the
// target `g`; a wall is still a wall.
fn panic_at(x: f32, y: f32, a: f32, so: f32, g: vec3<f32>) -> f32 {
    let c = cos(a); let s = sin(a);
    let i = cell(x + c * so, y + s * so);
    if (i == NONE || agar[i] == 0u) { return -1.0e6; }
    return c * (g.x - x) + s * (g.y - y);
}

@compute @workgroup_size(256)
fn sense(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n || !lives(k)) { return; }
    let t = S.t;
    let wk = walkers[k];
    var h = wk.z;
    let here = cell(wk.x, wk.y);
    // Infect reaches every living walker, as the CPU's `infect` before the
    // tick, whatever its life state.
    if (P.sick_steps > 0u && P.infect != 0u) {
        let dx = wk.x - P.ix; let dy = wk.y - P.iy;
        if (dx * dx + dy * dy <= P.ir * P.ir) { sick[k] = P.sick_steps + P.immune; }
    }
    // The life cycle, before anything else the walker does, as the CPU's
    // move loop: spores, the dormant and the dead do not sense or move
    // (`fate` and `fly` take them); an ordinary walker with its timer at 0
    // may fruit (scent over `spore_scent`, plus `spore_burst` over
    // `spore_alarm` of alarm) or else fall dormant (under `wake_scent` and
    // `wake_alarm`), from the fields as the pass began, and then neither
    // senses nor moves this tick.
    if (cycles()) {
        let w = spec[k];
        if (((w >> 9u) & 3u) != L_WALKER) { icell[k] = NONE; return; }
        if ((w >> 16u) == 0u) {
            let al = alarm_at(here);
            let sc = scent_at(wk.x, wk.y);
            var ch = 0.0;
            if (sc > P.spore_scent) { ch = P.spore_rate; }
            if (P.spore_burst > 0.0 && al > P.spore_alarm) { ch += P.spore_burst; }
            if (ch > 0.0 && unit(keyed(P.s_fruit, t, k)) < ch) {
                spec[k] = w | FRUIT;
                icell[k] = NONE;
                return;
            }
            if (P.sleep_rate > 0.0 && sc < P.wake_scent && al < P.wake_alarm && unit(keyed(P.s_sleep, t, k)) < P.sleep_rate) {
                spec[k] = w | DOZE;
                icell[k] = NONE;
                return;
            }
        }
    }
    // The CPU's `move_walkers` order: the walker's size, trail-set settings
    // from the trail where it stands, then catching and the sick walker's
    // changes.
    let sp = species_of(k);
    var so = P.so; var sa = P.sa; var ra = P.ra; var st = P.st; var sso = P.sso;
    if (per()) {
        let t = P.sets[sp];
        so = t.so; sa = t.sa; ra = t.ra; st = t.st; sso = t.sso;
    }
    // The second kind (`mix_*`), before the size as on the CPU. Its scent
    // reach stays: `mix_reach` does not make a walker smell farther.
    if (kind_of(k) == 1u) { so *= P.mreach; sa *= P.mangle; ra *= P.mturn; st *= P.mstep; }
    if (size_sp() > 0.0) {
        let sc = exp2(size_sp() * (2.0 * unit(keyed(P.s_size, 0u, k)) - 1.0));
        so *= sc; st *= sc;
    }
    var ex = false;
    if (explore() > 0u) { ex = explores(k); }
    // Wet ground and rubble: a shorter step (`wet_step`), where the CPU
    // takes it, after the size and before the trail-set settings.
    if (world(WETS) && (ground[here] & WET) != 0u) { st *= P.wet_step; }
    if (P.ss > 0.0) {
        let v = min(trail_in[here] * P.ss, 1.0);
        so = so + P.om_a * pow0(v, P.om_e);
        sa = sa + P.am_a * pow0(v, P.am_e);
        ra = ra + P.tm_a * pow0(v, P.tm_e);
        st = max(st + P.sm_a * pow0(v, P.sm_e), 0.0);
    }
    if (P.sick_steps > 0u) {
        // (Infect was applied above.)
        var c = sick[k];
        if (c == 0u && germ_in[here] > P.catch_at) { c = P.sick_steps + P.immune; }
        sick[k] = c;
        if (c > P.immune) { st *= P.sick_speed; so *= P.sick_reach; ra *= P.sick_turn; }
    }
    let gw = graze_of(sp);
    // Hunger (`hunger_scent`, the CPU's `hunger`): 1 - energy / `hunger_at`,
    // floored at 0, from the energy as the tick began.
    var sw = 1.0;
    if (growth() && P.hscent > 0.0 && P.hat > 0.0) { sw = 1.0 + P.hscent * max(1.0 - energy[k] / P.hat, 0.0); }
    // A cut-off walker panics: its sensors read only the way to its
    // target, and its step is `panic` times as long (the CPU's order:
    // after the sick walker's changes).
    let goal = panic_to(k, here);
    let scared = goal.x >= 0.0;
    if (scared) { st *= goal.z; }
    var f: f32; var fl: f32; var fr: f32;
    if (scared) {
        f = panic_at(wk.x, wk.y, h, so, goal);
        fl = panic_at(wk.x, wk.y, h - sa, so, goal);
        fr = panic_at(wk.x, wk.y, h + sa, so, goal);
    } else {
        f = sense_at(wk.x, wk.y, h, so, sso, sp, gw, sw);
        fl = sense_at(wk.x, wk.y, h - sa, so, sso, sp, gw, sw);
        fr = sense_at(wk.x, wk.y, h + sa, so, sso, sp, gw, sw);
        // Explorers steer to the lowest reading; a wall stays a wall.
        if (ex) {
            if (f > -1.0e5) { f = -f; }
            if (fl > -1.0e5) { fl = -fl; }
            if (fr > -1.0e5) { fr = -fr; }
        }
    }
    if (!scared && P.wander > 0.0 && unit(keyed(P.s_wander, t, k | 0x80000000u)) < P.wander) {
        h = unit(keyed(P.s_wander, t, k | 0x40000000u)) * TAU;
    } else if (!scared && P.sfloor > 0.0 && max(max(f, fl), fr) < P.sfloor) {
        if ((keyed(P.s_wander, t, k) & 1u) == 0u) { h += ra; } else { h -= ra; }
    } else if (!scared && chance() > 0.0) {
        // The chance turn: a side by a draw, then toward it with chance
        // g^e / (f^e + g^e), readings floored at 0 (the CPU's `chance_exp`).
        let left = (keyed(P.s_chance, t, k) & 1u) == 0u;
        let fe = pow0(max(f, 0.0), chance());
        let ge = pow0(max(select(fr, fl, left), 0.0), chance());
        var turn = 0.5;
        if (fe + ge > 0.0) { turn = ge / (fe + ge); }
        if (unit(keyed(P.s_chance, t, k | 0x80000000u)) < turn) {
            if (left) { h -= ra; } else { h += ra; }
        }
    } else if (f > fl && f > fr) {
    } else if (f < fl && f < fr) {
        if ((keyed(P.s_turn, t, k) & 1u) == 0u) { h += ra; } else { h -= ra; }
    } else if (fl < fr) {
        h += ra;
    } else if (fr < fl) {
        h -= ra;
    }
    if (P.lone > 0.0 && !scared) {
        // `occ` is as the step began here: `walk` moves walkers after.
        let cx = i32(here % P.w); let cy = i32(here / P.w); let r = i32(P.lone_r);
        var n = 0u;
        for (var y = max(cy - r, 0); y <= min(cy + r, i32(P.h) - 1); y++) {
            for (var x = max(cx - r, 0); x <= min(cx + r, i32(P.w) - 1); x++) {
                n += atomicLoad(&occ[u32(y) * P.w + u32(x)]);
            }
        }
        st *= clamp(f32(max(n, 1u) - 1u) / P.lone, min(P.lfloor, 1.0), 1.0);
    }
    let nx = wk.x + cos(h) * st;
    let ny = wk.y + sin(h) * st;
    let there = cell(nx, ny);
    var aim = NONE;
    if (there != NONE) {
        if (there == here) {
            aim = there;
        } else if (agar[there] != 0u && (crowd_for(k, sp) == 0u || atomicLoad(&occ[there]) < crowd_for(k, sp))) {
            aim = there;
            if (crowd_of(sp) > 0u) { atomicMin(&claim[there], keyed(P.s_claim, t, k)); }
        }
    }
    intent[k] = vec4<f32>(nx, ny, h, 0.0);
    icell[k] = aim;
}

@compute @workgroup_size(256)
fn walk(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n || !lives(k)) { return; }
    // Walkers that do not move this tick (the life cycle): no step, claim,
    // deposit or grazing. The dead and those that fruit or fall dormant
    // count their counters down here, as the CPU counts every walker's;
    // the dormant and spores count theirs in `fate` and `alight`, after
    // they may wake or land (an ordinary walker then is sick, an explorer,
    // by the counters before the count).
    if (cycles()) {
        let w = spec[k];
        let st = (w >> 9u) & 3u;
        if (st != L_WALKER || (w & (FRUIT | DOZE)) != 0u) {
            if (st == L_WALKER || st == L_DEAD) { tally(k); }
            return;
        }
    }
    let t = S.t;
    let aim = icell[k];
    let wk = walkers[k];
    let here = cell(wk.x, wk.y);
    var wins = false;
    if (aim != NONE) {
        wins = aim == here || crowd_of(species_of(k)) == 0u || atomicLoad(&claim[aim]) == keyed(P.s_claim, t, k);
    }
    var at = here;
    if (wins) {
        if (aim != here) {
            atomicSub(&occ[here], 1u);
            atomicAdd(&occ[aim], 1u);
        }
        walkers[k] = intent[k];
        at = aim;
    } else {
        // Blocked: Jones's rule, a new heading; or (`bkeep`) the heading it
        // chose, bar a reset with chance `breset` (the CPU's `blocked_keep`).
        var bh = unit(keyed(P.s_blocked, t, k)) * TAU;
        if (bkeep() != 0u && unit(keyed(P.s_keep, t, k)) >= P.breset) { bh = intent[k].z; }
        // A walker a wall blocks bites the wall cell it tried to enter
        // (the CPU's `bite_walls`; the `walls` pass applies the bites).
        if (world(WALLS)) {
            let tc = cell(intent[k].x, intent[k].y);
            if (tc != NONE && (ground[tc] & WALL) != 0u) { atomicAdd(&bites[tc], 1u); }
        }
        walkers[k] = vec4<f32>(wk.x, wk.y, bh, 0.0);
    }
    // Trail: a walker that moved lays it where it landed; with the capped
    // square-root deposit every walker counts where it stands after the
    // move (`trail` shares the cell's deposit among them). Sick walkers and
    // explorers are counted apart, for `sick_trail` and `edep`, each in its
    // species' block; with `bshare` (and no cap) a blocked walker counts
    // where it stands.
    var ill = false;
    if (P.sick_steps > 0u) { ill = sick[k] > P.immune; }
    var ex = false;
    if (explore() > 0u) { ex = explores(k); }
    let c = P.w * P.h;
    let o = blk(species_of(k), kind_of(k)) * c + at;
    // Grazing: every walker that eats (`eats`) eats where it stands after
    // the move.
    if (grazes() && eats(k)) { atomicAdd(&gcount[at], 1u); }
    if (P.cap > 0u || wins) {
        if (ex) {
            atomicAdd(&edep[o + select(0u, nblk() * c, ill)], 1u);
        } else if (ill) {
            atomicAdd(&sdep[o], 1u);
        } else {
            atomicAdd(&dep[o], 1u);
        }
    } else if (bshare() > 0.0) {
        atomicAdd(&bdep[o + select(0u, nblk() * c, ex)], 1u);
    }
    // Sick walkers lay germ where they stand, then every counter falls by
    // one (before any jump, as on the CPU).
    if (P.sick_steps > 0u) {
        if (ill) { atomicAdd(&gdep[at], 1u); }
        let n = sick[k];
        if (n > 0u) { sick[k] = n - 1u; }
    }
    // Ages count up after the deposit, before any jump, in the CPU's u16.
    if (explore() > 0u) { age[k] = min(age[k] + 1u, 65535u); }
}

// Growth (Larder World W0), after the move, three passes, as the CPU's
// `live_and_die`. `life`: each walker gains its equal share of what its
// cell's grass loses this tick (min(grass / n, eat) for the n walkers on
// it; the trail pass eats it after), pays `upkeep`, and at zero dies with
// chance `starve`, leaving its cell (and clearing the cell's claim, so a
// birth can claim it this tick as on the CPU, which clears move claims
// before births). Only dispatched with growth on.
@compute @workgroup_size(256)
fn life(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    // Only ordinary walkers eat, pay upkeep and starve: a spore or a
    // dormant walker keeps its energy (the CPU's `live_and_die`).
    if (k >= P.n || !ordinary(k)) { return; }
    let wk = walkers[k];
    let i = cell(wk.x, wk.y);
    // One body's sums are read (`egain`); clear them for the next tick.
    // Every cell `esum` added to holds an ordinary walker, which runs here.
    if (P.eflow > 0.0) { atomicStore(&body[i], 0u); atomicStore(&body[P.w * P.h + i], 0u); }
    // A part cut off from its food withers (`wither`, before the upkeep as
    // on the CPU): at each check a walker that has heard of no food for
    // its lag may die, whatever it still holds.
    if (P.wither > 0.0 && S.t % max((P.wclock >> 16u) & 0xFFu, 1u) == 0u && S.t + 1u > atomicLoad(&news[k]) + (P.wclock & 0xFFFFu)) {
        if (unit(keyed(P.s_wither, S.t, k)) < P.wither) {
            atomicSub(&occ[i], 1u);
            atomicStore(&claim[i], NONE);
            energy[k] = -1.0;
            return;
        }
    }
    let n = f32(atomicLoad(&occ[i]));
    var g = 0.0;
    if (eats(k)) { g = min(grass[i] / n, P.graze_eat); }
    // Barren ground costs `barren_up` times the upkeep (WM3).
    var up = P.upkeep;
    if (soil(i) == SOIL_BARREN) { up = P.upkeep * P.barren_up; }
    let e = (energy[k] + g + share[i]) - up;
    if (e > 0.0) { energy[k] = e; return; }
    if (unit(keyed(P.s_starve, S.t, k)) < P.starve) {
        atomicSub(&occ[i], 1u);
        atomicStore(&claim[i], NONE);
        energy[k] = -1.0;
    } else {
        energy[k] = 0.0;
    }
}

// One body (`energy_flow`, the CPU's `flow_energy`), after eating and
// before `life`: `esum` adds each ordinary walker's energy and a count to
// its cell; `egain` moves each walker's energy by `eflow` x the sum over
// the eight cells round its own that hold walkers of (their mean - its
// cell's mean), over its cell's count, floored at 0. Pairs give and take
// alike, so energy is kept (to the fixed point's rounding), unless
// `eloss` (the CPU's `flow_loss`, 2026-11-13): then what a cell takes from
// a richer neighbour arrives x 1 - `eloss` while the giver pays in full, so
// food has a reach and a part cut off from every crystal holds little in
// reserve (Reports/fed-mould-2026-11-13/README.md, *Cutting it off*). Only
// dispatched with growth and `energy_flow` on.
@compute @workgroup_size(256)
fn esum(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n || !ordinary(k)) { return; }
    let wk = walkers[k];
    let i = cell(wk.x, wk.y);
    atomicAdd(&body[i], u32(round(max(energy[k], 0.0) * ESCALE)));
    atomicAdd(&body[P.w * P.h + i], 1u);
}

fn emean(i: u32) -> f32 {
    return f32(atomicLoad(&body[i])) / ESCALE / f32(atomicLoad(&body[P.w * P.h + i]));
}

@compute @workgroup_size(256)
fn egain(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n || !ordinary(k)) { return; }
    let wk = walkers[k];
    let i = cell(wk.x, wk.y);
    let c = P.w * P.h;
    let m = emean(i);
    let x = i32(i % P.w); let y = i32(i / P.w);
    var f = 0.0;
    for (var q = 0u; q < 8u; q++) {
        let d = NEIGH8[q];
        let nx = x + d.x; let ny = y + d.y;
        if (nx < 0 || ny < 0 || nx >= i32(P.w) || ny >= i32(P.h)) { continue; }
        let j = u32(ny) * P.w + u32(nx);
        if (atomicLoad(&body[c + j]) > 0u) {
            let df = emean(j) - m;
            f += select(df, df * (1.0 - clamp(P.eloss, 0.0, 1.0)), df > 0.0);
        }
    }
    energy[k] = max(energy[k] + P.eflow * f / f32(atomicLoad(&body[c + i])), 0.0);
}
// The CPU's order round a cell.
const NEIGH8 = array<vec2<i32>, 8>(vec2(1, 0), vec2(1, 1), vec2(0, 1), vec2(-1, 1), vec2(-1, 0), vec2(-1, -1), vec2(0, -1), vec2(1, -1));

// Food's news (`wither`, the CPU's `hear_food`), after eating and before
// `life`. `ngather`: a walker that took crystal this step (its cell's
// `share`) hears it now, and each cell gets the latest news its walkers
// carry (at least 1, so a cell with walkers is never 0). Then `nspread`
// and its turns: a cell with walkers takes the latest of its own and the
// eight round it, a cell with none stays 0, `wither_reach` times. Then
// `nhear`: each walker takes its cell's news if later than its own.
// Whole numbers and a max, so the order the threads run in changes
// nothing.
@compute @workgroup_size(256)
fn ngather(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n || !ordinary(k)) { return; }
    let wk = walkers[k];
    let i = cell(wk.x, wk.y);
    if (share[i] > 0.0) { atomicStore(&news[k], S.t + 1u); }
    atomicMax(&news[P.n + i], atomicLoad(&news[k]));
}

// One turn of the spread, from block `src` into block `dst` of the cells'
// blocks (0 the gathered one, 1 and 2 the turns'), a thread a cell.
fn nturn(i: u32, src: u32, dst: u32) {
    let c = P.w * P.h;
    let v = atomicLoad(&news[P.n + src * c + i]);
    var best = v;
    if (v > 0u) {
        let x = i32(i % P.w); let y = i32(i / P.w);
        for (var q = 0u; q < 8u; q++) {
            let d = NEIGH8[q];
            let nx = x + d.x; let ny = y + d.y;
            if (nx < 0 || ny < 0 || nx >= i32(P.w) || ny >= i32(P.h)) { continue; }
            best = max(best, atomicLoad(&news[P.n + src * c + u32(ny) * P.w + u32(nx)]));
        }
    }
    atomicStore(&news[P.n + dst * c + i], best);
}

@compute @workgroup_size(256)
fn nspread(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= P.w * P.h) { return; }
    nturn(gid.x, 0u, 1u);
}

@compute @workgroup_size(256)
fn nab(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= P.w * P.h) { return; }
    nturn(gid.x, 1u, 2u);
}

@compute @workgroup_size(256)
fn nba(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= P.w * P.h) { return; }
    nturn(gid.x, 2u, 1u);
}

@compute @workgroup_size(256)
fn nhear(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n || !ordinary(k)) { return; }
    let wk = walkers[k];
    let i = cell(wk.x, wk.y);
    let c = P.w * P.h;
    // The spread ends in its second turn's block with an even reach.
    let last = select(1u, 2u, ((P.wclock >> 24u) & 1u) == 1u);
    atomicMax(&news[k], atomicLoad(&news[P.n + last * c + i]));
    // The gathered block was read by `nspread` only; every cell with news
    // in it holds a walker that runs here, so it is clear for next step.
    atomicStore(&news[P.n + i], 0u);
}

// The slot a parent's child takes this tick: k xor one keyed mask, so each
// empty slot has one parent.
fn child_of(k: u32, t: u32) -> u32 {
    return k ^ (keyed(P.s_birth, t, 0xFFFFFFFFu) & P.bmask);
}

// Where parent k's child lands: one of the eight cells beside it.
fn birth_spot(k: u32, t: u32) -> vec2<f32> {
    let d = keyed(P.s_birth, t, k) % 8u;
    var dx = array<f32, 8>(1.0, 1.0, 0.0, -1.0, -1.0, -1.0, 0.0, 1.0);
    var dy = array<f32, 8>(0.0, 1.0, 1.0, 1.0, 0.0, -1.0, -1.0, -1.0);
    let wk = walkers[k];
    return vec2<f32>(wk.x + dx[d], wk.y + dy[d]);
}

// `breed`: a parent with `birth_at` energy or more whose child slot is
// empty aims at a free agar cell beside it (claimed, lowest key wins, with
// the crowd limit) and keeps it in `icell`; `born` lands the winners.
@compute @workgroup_size(256)
fn breed(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    icell[k] = NONE;
    let t = S.t;
    let b = child_of(k, t);
    if (!ordinary(k) || energy[k] < P.birth_at || b >= P.n || lives(b)) { return; }
    let at = birth_spot(k, t);
    let j = cell(at.x, at.y);
    // The parent's species' crowd limit, + `fed_crowd` if it is fed
    // (`crowd_for`).
    let cr = crowd_for(k, species_of(k));
    // No child lands on barren ground (WM3).
    if (j == NONE || agar[j] == 0u || (cr > 0u && atomicLoad(&occ[j]) >= cr) || soil(j) == SOIL_BARREN) { return; }
    if (cr > 0u) { atomicMin(&claim[j], keyed(P.s_bclaim, t, k)); }
    icell[k] = j;
}

@compute @workgroup_size(256)
fn born(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let j = icell[k];
    let t = S.t;
    if (j == NONE || (crowd_of(species_of(k)) > 0u && atomicLoad(&claim[j]) != keyed(P.s_bclaim, t, k))) { return; }
    let b = child_of(k, t);
    let half = max((energy[k] - P.birth_cost) * 0.5, 0.0);
    energy[k] = half;
    energy[b] = half;
    atomicAdd(&occ[j], 1u);
    let at = birth_spot(k, t);
    walkers[b] = vec4<f32>(at, unit(keyed(P.s_birth, t, k | 0x80000000u)) * TAU, 0.0);
    if (explore() > 0u) { age[b] = 0u; }
    if (P.sick_steps > 0u) { sick[b] = 0u; }
    // The child is its parent's species and kind, and an ordinary walker
    // with its timer at 0.
    if (nsp() > 0u || mixes() || cycles()) { spec[b] = spec[k] & 0x1FFu; }
    // A child knows what its parent heard (`wither`).
    if (P.wither > 0.0 || P.panic > 0.0) { atomicStore(&news[b], atomicLoad(&news[k])); }
}

// Conversion (PR #56), after the move and its deposit, before any jump: a
// walker joins the species with the most trail at its cell (with cyclic
// species, only the previous one, which chases it) if that trail outweighs
// its own `convert` times. The trail is the field after this tick's
// deposit and before the spread, as the CPU reads it. Only dispatched with
// species and `convert` on.
@compute @workgroup_size(256)
fn convert(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n || !lives(k)) { return; }
    let wk = walkers[k];
    let i = cell(wk.x, wk.y);
    var all = 0.0;
    if (P.cap > 0u) { all = counted(i); }
    let own = species_of(k);
    let ov = weigh(own, i, all);
    var best = own;
    var bv = ov;
    if (P.cycle != 0u && nsp() >= 3u) {
        best = (own + nsp() - 1u) % nsp();
        bv = weigh(best, i, all);
    } else {
        for (var s = 0u; s < nsp(); s++) {
            if (s != own) {
                let v = weigh(s, i, all);
                if (v > bv) { best = s; bv = v; }
            }
        }
    }
    // The kind (bit 8) stays.
    if (best != own && bv > P.convert * ov) { spec[k] = (spec[k] & 0xFFFFFF00u) | best; }
}

// Species s's trail as conversion weighs it at cell i: the cell's, after
// this tick's deposit; or with `conv_r` the sum over the (2r+1)-square
// round it, clipped at the dish's edge (the CPU's summed-area tables, in
// f64 there; here the `rows` pass's row sums in `sp_out`, which the trail
// pass overwrites after, summed down the column).
fn weigh(s: u32, i: u32, all: f32) -> f32 {
    let c = P.w * P.h;
    let r = conv_r();
    if (r == 0u) { return sp_in[s * c + i] + laid_by(s, i, all); }
    let x = i % P.w; let y = i / P.w;
    var sum = 0.0;
    for (var yy = y - min(y, r); yy < min(y + r + 1u, P.h); yy++) { sum += sp_out[s * c + yy * P.w + x]; }
    return sum;
}

// Conversion's square, first half: each species' trail after this tick's
// deposit, summed along the cell's row over x - r to x + r (clipped), into
// `sp_out`. Only dispatched with conversion on and `conv_r` above 0. A
// workgroup takes 256 cells of one row (`ROW_CHUNK`): it works out each
// cell's trail after the deposit once, for its cells and r either side,
// into `tile`, then each cell sums its row from there. (A first version
// had each cell work out all 2r + 1 of its row's values itself: at r 8 the
// pass took longer than the rest of the tick, 136-142 ms of GPU ticks a
// frame against 61-63 without the square on the software GPU.)
const ROW_CHUNK: u32 = 256u;
const ROW_R: u32 = 64u;
var<workgroup> tile: array<f32, 1536>;
@compute @workgroup_size(256)
fn rows(@builtin(workgroup_id) wg: vec3<u32>, @builtin(local_invocation_id) li: vec3<u32>) {
    let chunks = (P.w + ROW_CHUNK - 1u) / ROW_CHUNK;
    let y = wg.x / chunks;
    let x0 = (wg.x % chunks) * ROW_CHUNK;
    let r = min(conv_r(), ROW_R);
    let c = P.w * P.h;
    // Cell x0 - r + q of the row in tile[4q + s], 0 off the dish.
    for (var q = li.x; q < ROW_CHUNK + 2u * r; q += ROW_CHUNK) {
        let gx = i32(x0 + q) - i32(r);
        var all = 0.0;
        let on = gx >= 0 && gx < i32(P.w) && y < P.h;
        let j = y * P.w + u32(max(gx, 0));
        if (on && P.cap > 0u) { all = counted(j); }
        for (var s = 0u; s < 4u; s++) {
            var v = 0.0;
            if (on && s < nsp()) { v = sp_in[s * c + j] + laid_by(s, j, all); }
            tile[4u * q + s] = v;
        }
    }
    workgroupBarrier();
    let x = x0 + li.x;
    if (x >= P.w || y >= P.h) { return; }
    let i = y * P.w + x;
    var sums = array<f32, 4>(0.0, 0.0, 0.0, 0.0);
    for (var xx = x - min(x, r); xx < min(x + r + 1u, P.w); xx++) {
        let q = xx + r - x0;
        for (var s = 0u; s < nsp(); s++) { sums[s] += tile[4u * q + s]; }
    }
    for (var s = 0u; s < nsp(); s++) { sp_out[s * c + i] = sums[s]; }
}

// Where walker k jumps this tick, if it does (w = 1): the pull (to a disc
// at the mouse) first, then respawn (anywhere), each by its own keyed
// draws; the heading by respawn's (the CPU's `respawn_walkers`).
fn jump_of(k: u32, t: u32) -> vec4<f32> {
    var at = vec2<f32>(0.0, 0.0);
    if (P.pull > 0.0 && unit(keyed(P.s_pull, t, 4u * k)) < P.pull) {
        let a = unit(keyed(P.s_pull, t, 4u * k + 1u)) * TAU;
        let r = sqrt(unit(keyed(P.s_pull, t, 4u * k + 2u))) * P.br;
        at = vec2<f32>(P.bx + cos(a) * r, P.by + sin(a) * r);
    } else if (P.respawn > 0.0 && unit(keyed(P.s_jump, t, 4u * k)) < P.respawn) {
        at = vec2<f32>(unit(keyed(P.s_jump, t, 4u * k + 1u)) * f32(P.w), unit(keyed(P.s_jump, t, 4u * k + 2u)) * f32(P.h));
    } else {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
    return vec4<f32>(at, unit(keyed(P.s_jump, t, 4u * k + 3u)) * TAU, 1.0);
}

// Jumps come after the move, as on the CPU: a walker steps (and lays its
// trail where it stepped), then may jump. A jump off the agar is lost. With
// no crowd limit it lands here; with one, it claims its cell (lowest walker
// wins, the CPU's order) and `land` moves the winners. Only dispatched while
// respawn or the pull is on.
@compute @workgroup_size(256)
fn jump(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    icell[k] = NONE;
    // Only ordinary walkers jump (the CPU's `respawn_walkers`).
    if (!ordinary(k)) { return; }
    let j = jump_of(k, S.t);
    if (j.w == 0.0) { return; }
    let to = cell(j.x, j.y);
    if (to == NONE || agar[to] == 0u) { return; }
    let wk = walkers[k];
    let here = cell(wk.x, wk.y);
    let cr = crowd_of(species_of(k));
    if (cr == 0u && per()) {
        // With species' own crowd limits a walker of a species with none
        // claims the cell it jumps to as well and lands in `land` with the
        // limited ones (always dispatched with `per`), so a limited walker
        // later in the CPU's walker order loses the cell to it, as on the
        // CPU, while one earlier wins it and both land, as there; and every
        // jump here reads the cells as the pass began, so none races
        // another's move. (A first version moved them here: a limited
        // walker could then read a cell before or after an unlimited one
        // left it.)
        atomicMin(&claim[to], k);
        icell[k] = to;
        return;
    }
    if (cr == 0u) {
        if (to != here) {
            atomicSub(&occ[here], 1u);
            atomicAdd(&occ[to], 1u);
        }
        walkers[k] = vec4<f32>(j.xyz, 0.0);
        // A walker that jumps starts its life again (the CPU's respawn).
        if (explore() > 0u) { age[k] = 0u; }
        return;
    }
    // A cell claimed by a step this tick now holds its winner, so it is
    // full and never claimed here: the claim's stale keys cannot win.
    if (atomicLoad(&occ[to]) >= cr) { return; }
    atomicMin(&claim[to], k);
    icell[k] = to;
}

@compute @workgroup_size(256)
fn land(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let to = icell[k];
    // A walker of a species with no crowd limit lands whatever the claim.
    if (to == NONE || (crowd_of(species_of(k)) > 0u && atomicLoad(&claim[to]) != k)) { return; }
    let wk = walkers[k];
    atomicSub(&occ[cell(wk.x, wk.y)], 1u);
    atomicAdd(&occ[to], 1u);
    walkers[k] = vec4<f32>(jump_of(k, S.t).xyz, 0.0);
    if (explore() > 0u) { age[k] = 0u; }
}

// The cell's walkers that share a capped deposit: every species and class
// (with the cap each walker counts where it stands). Integer counts, so
// the sum is exact in any order.
fn counted(j: u32) -> f32 {
    let c = P.w * P.h;
    let m = nblk();
    var n = 0.0;
    for (var s = 0u; s < m; s++) {
        let o = s * c + j;
        n += f32(atomicLoad(&dep[o]));
        if (P.sick_steps > 0u) { n += f32(atomicLoad(&sdep[o])); }
        if (explore() > 0u) { n += f32(atomicLoad(&edep[o])) + f32(atomicLoad(&edep[o + m * c])); }
    }
    // The cell's dormant walkers: the CPU shares by `occ`, which holds them.
    if (cycles()) { n += f32(atomicLoad(&gdep[2u * c + j])); }
    return n;
}

// The trail species s gets on cell j this tick: `deposit` a walker that
// moved (the sick lay `sick_trail` times as much, explorers `edep` times),
// with `bshare` that share of it a blocked walker, or with `cap` the CPU's
// capped square-root deposit, deposit x sqrt(min(n, cap)) shared by the
// cell's n walkers (`all`, from `counted`). With the outside rules off the
// expressions are the looks build's, so its dishes run as they did. The
// deposit and blocked share are the species' own with `per`, and the
// second kind's walkers (`mix_*`) lay `mdep` times as much.
fn laid(s: u32, j: u32, all: f32) -> f32 {
    var l = laid_kind(s, 0u, j, all);
    if (mixes()) { l += P.mdep * laid_kind(s, 1u, j, all); }
    return l;
}

fn laid_kind(s: u32, kd: u32, j: u32, all: f32) -> f32 {
    var l = moved_kind(s, kd, j, all);
    // The life cycle: a spore lays `spore_dep` of the global deposit where
    // it flew to, a dormant walker `sleep_dep` where it lies (the CPU's
    // `life_step`, through `lay`).
    if (cycles()) {
        let c = P.w * P.h;
        let o = blk(s, kd) * c + j + 2u * nblk() * c;
        l += P.deposit * (P.spore_dep * f32(atomicLoad(&bdep[o])) + P.sleep_dep * f32(atomicLoad(&bdep[o + nblk() * c])));
    }
    return l;
}

fn moved_kind(s: u32, kd: u32, j: u32, all: f32) -> f32 {
    let c = P.w * P.h;
    let m = nblk();
    let o = blk(s, kd) * c + j;
    let dp = deposit_of(s);
    let a = f32(atomicLoad(&dep[o]));
    if (P.cap == 0u && P.sick_steps == 0u && explore() == 0u && bshare() == 0.0) { return dp * a; }
    var b = 0.0;
    if (P.sick_steps > 0u) { b = f32(atomicLoad(&sdep[o])); }
    var w = a + P.sick_trail * b;
    if (explore() > 0u) {
        w += P.edep * (f32(atomicLoad(&edep[o])) + P.sick_trail * f32(atomicLoad(&edep[o + m * c])));
    }
    if (P.cap == 0u) {
        var l = dp * w;
        let bs = bshare_of(s);
        if (bs > 0.0) {
            l += dp * bs * (f32(atomicLoad(&bdep[o])) + P.edep * f32(atomicLoad(&bdep[o + m * c])));
        }
        return l;
    }
    if (all == 0.0) { return 0.0; }
    return dp * sqrt(min(all, f32(P.cap))) / all * w;
}

// The trail species s lays on cell j this tick, with species 0's boost
// (the CPU's `species_boost`, applied to each of its walkers' deposits).
fn laid_by(s: u32, j: u32, all: f32) -> f32 {
    let l = laid(s, j, all);
    if (s == 0u) { return l * P.boost; }
    return l;
}

@compute @workgroup_size(256)
fn trail(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    let c = P.w * P.h;
    if (i >= c) { return; }
    // The germ spreads as the trail does, with its own keep; so does the
    // alarm (the germ fields' second block), with each waking walker's
    // relay laid first (the CPU adds them in `life_step`, before the spread).
    let germs = P.sick_steps > 0u;
    let al = alarms();
    // A pile is reached once tube lies on it: the CPU marks it at the end
    // of a tick from that tick's trail, which is this tick's `trail_in`, so
    // the scent pass below lays this tick's food as the CPU's next tick does.
    let pi = pile[i];
    if (pi != 0u && trail_in[i] > P.tube) { atomicMax(&reached[pi - 1u], 1u); }
    // Grazing: the walkers on the cell each eat `graze_eat` (floored at 0,
    // one at a time, as the CPU does), then the grass regrows. In place:
    // only the next tick's sense pass reads it.
    if (grazes() && agar[i] != 0u) {
        var g = grass[i];
        let eaters = atomicLoad(&gcount[i]);
        for (var q = 0u; q < eaters && g > 0.0; q++) { g = max(g - P.graze_eat, 0.0); }
        grass[i] = g + P.graze_regrow * (1.0 - g);
    }
    let x = i % P.w; let y = i / P.w;
    var gone = agar[i] == 0u || x == 0u || y == 0u || x + 1u == P.w || y + 1u == P.h;
    // The right mouse wipes trail and germ, as the looks report's knife.
    if (!gone && P.wipe != 0u) {
        let dx = f32(x) + 0.5 - P.bx; let dy = f32(y) + 0.5 - P.by;
        gone = dx * dx + dy * dy <= P.br * P.br;
    }
    // Shots: a hit cuts as the wipe does, and the cell's wound takes the
    // thickness it cut (what the shot destroyed, not a drawn splash). The
    // wound fades about 0.8% a tick (half in about 0.75 s).
    var wound = (look[i] >> 16u) & 255u;
    wound -= min(wound, max(wound / 128u, 1u));
    for (var q = 0u; q < P.nhits; q++) {
        let hp = hits[q];
        let dx = f32(x) + 0.5 - hp.x; let dy = f32(y) + 0.5 - hp.y;
        if (agar[i] != 0u && dx * dx + dy * dy <= hp.z * hp.z) {
            wound = max(wound, u32(bright(trail_in[i]) * 255.0));
            gone = true;
        }
    }
    look[i] = (look[i] & 0xFF00FFFFu) | (wound << 16u);
    if (gone) {
        trail_out[i] = 0.0;
        if (germs) { germ_out[i] = 0.0; }
        if (al) { germ_out[c + i] = 0.0; }
        for (var s = 0u; s < nsp(); s++) { sp_out[s * c + i] = 0.0; }
        if ((P.scorched & 1u) != 0u) {
            let left = crater_at(i);
            if (left > 0.0) { crater_set(i, left - 1.0); }
        }
        return;
    }
    let capped = P.cap > 0u;
    var gsum = 0.0; var asum = 0.0; var n = 0.0;
    if (nsp() == 0u) {
        var sum = 0.0;
        for (var dy = 0u; dy < 3u; dy++) {
            for (var dx = 0u; dx < 3u; dx++) {
                let j = (y + dy - 1u) * P.w + x + dx - 1u;
                if (agar[j] != 0u) {
                    var all = 0.0;
                    if (capped) { all = counted(j); }
                    sum += trail_in[j] + laid(0u, j, all);
                    if (germs) { gsum += germ_in[j] + P.sick_dep * f32(atomicLoad(&gdep[j])); }
                    if (al) { asum += germ_in[c + j] + P.relay * f32(atomicLoad(&gdep[c + j])); }
                    n += 1.0;
                }
            }
        }
        trail_out[i] = sum / n * P.keep * soil_keep(i);
    } else {
        // Each species' field spreads as the trail would; the trail is
        // their sum, in species order (the CPU's `tick`).
        var sums = array<f32, 4>(0.0, 0.0, 0.0, 0.0);
        for (var dy = 0u; dy < 3u; dy++) {
            for (var dx = 0u; dx < 3u; dx++) {
                let j = (y + dy - 1u) * P.w + x + dx - 1u;
                if (agar[j] != 0u) {
                    var all = 0.0;
                    if (capped) { all = counted(j); }
                    for (var s = 0u; s < nsp(); s++) { sums[s] += sp_in[s * c + j] + laid_by(s, j, all); }
                    if (germs) { gsum += germ_in[j] + P.sick_dep * f32(atomicLoad(&gdep[j])); }
                    if (al) { asum += germ_in[c + j] + P.relay * f32(atomicLoad(&gdep[c + j])); }
                    n += 1.0;
                }
            }
        }
        // Soil after the sum, as the CPU: each field and the trail x the
        // cell's keep.
        let f = soil_keep(i);
        var total = 0.0;
        for (var s = 0u; s < nsp(); s++) {
            let v = sums[s] / n * keep_of(s);
            sp_out[s * c + i] = v * f;
            total += v;
        }
        trail_out[i] = total * f;
    }
    if (germs) { germ_out[i] = gsum / n * P.sick_keep; }
    if (al) { germ_out[c + i] = asum / n * P.alarm_keep; }
    // The crater, after the soil, on the trail and each species' field.
    if ((P.scorched & 1u) != 0u) {
        let left = crater_at(i);
        if (left > 0.0) {
            let f = 1.0 - (1.0 - P.scorch_keep) * min(left / P.scorch_full, 1.0);
            trail_out[i] *= f;
            for (var s = 0u; s < nsp(); s++) { sp_out[s * c + i] *= f; }
            crater_set(i, left - 1.0);
        }
    }
}

@compute @workgroup_size(256)
fn scent(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= P.w * P.h) { return; }
    atomicStore(&claim[i], NONE);
    if (P.sick_steps > 0u) { atomicStore(&gdep[i], 0u); }
    if (cycles()) { atomicStore(&gdep[P.w * P.h + i], 0u); atomicStore(&gdep[2u * P.w * P.h + i], 0u); }
    // Cleared whenever the walk pass counts (`grazes`), not only with
    // `graze_w`: with growth on and grazing's weight 0 the counts piled up
    // tick on tick, so every walker that had ever stood on a cell ate its
    // grass again (found 2026-11-11 reading for the species port; the
    // one-tick twin started from fresh counts and could not see it).
    if (grazes()) { atomicStore(&gcount[i], 0u); }
    // The counts, a block a species (and a class).
    let c = P.w * P.h;
    let m = nblk();
    for (var s = 0u; s < m; s++) {
        let o = s * c + i;
        atomicStore(&dep[o], 0u);
        if (P.sick_steps > 0u) { atomicStore(&sdep[o], 0u); }
        if (explore() > 0u) { atomicStore(&edep[o], 0u); atomicStore(&edep[o + m * c], 0u); }
        if (bshare() > 0.0) { atomicStore(&bdep[o], 0u); atomicStore(&bdep[o + m * c], 0u); }
        if (cycles()) { atomicStore(&bdep[o + 2u * m * c], 0u); atomicStore(&bdep[o + 3u * m * c], 0u); }
    }
    if (i == 0u) { S.t = S.t + 1u; }
    let x = i % P.w; let y = i / P.w;
    if (!open(i) || x == 0u || y == 0u || x + 1u == P.w || y + 1u == P.h) { scent_out[i] = 0.0; return; }
    var sum = 0.0; var n = 0.0;
    for (var dy = 0u; dy < 3u; dy++) {
        for (var dx = 0u; dx < 3u; dx++) {
            let j = (y + dy - 1u) * P.w + x + dx - 1u;
            if (open(j)) {
                // A reached pile lays `fed` of its scent (the CPU's
                // `fed_scent`), and a crystal its share left of that.
                var l = lay[j];
                let pj = pile[j];
                if (l != 0.0 && pj != 0u) {
                    if (atomicLoad(&reached[pj - 1u]) != 0u) { l *= P.fed; }
                    if (world(CRYSTALS)) { l *= piles[pj - 1u].left; }
                }
                sum += scent_in[j] + l;
                n += 1.0;
            }
        }
    }
    var v = sum / n * P.scent_keep;
    // A wall lets `wall_scent` of the scent spread into it stay.
    if (world(WALLS) && (ground[i] & SCENT_WALL) != 0u) { v *= P.wall_scent; }
    scent_out[i] = v;
}

// What a pixel is drawn from: the trail, or walkers a cell.
fn shown(i: u32) -> f32 {
    if (P.field == 1u) { return f32(atomicLoad(&occ[i])); }
    return trail_in[i];
}

// The soft curve's reference, p99.9 of the shown field over cells above 0:
// a 512-bin histogram of log2 (8 bins an octave from 2^-16 to 2^48), summed
// in workgroup memory first so the global bins see one add a workgroup. (It
// had 256 bins, to 2^16, until the review of 2026-11-11: a trail laid at
// 2,000 passed that and 0.66% of cells clipped to full brightness.)
const BINS: u32 = 512u;
var<workgroup> local: array<atomic<u32>, 512>;

@compute @workgroup_size(256)
fn histogram(@builtin(global_invocation_id) gid: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    atomicStore(&local[li], 0u);
    atomicStore(&local[li + 256u], 0u);
    workgroupBarrier();
    let i = gid.x;
    if (i < P.w * P.h) {
        let v = shown(i);
        if (v > 0.0) {
            let b = u32(clamp((log2(v) + 16.0) * 8.0, 0.0, f32(BINS - 1u)));
            atomicAdd(&local[b], 1u);
        }
    }
    workgroupBarrier();
    for (var j = li; j < BINS; j += 256u) {
        let c = atomicLoad(&local[j]);
        if (c > 0u) { atomicAdd(&hist[j], c); }
    }
}

@compute @workgroup_size(1)
fn top() {
    var total = 0u;
    for (var j = 0u; j < BINS; j++) { total += atomicLoad(&hist[j]); }
    let want = max(total / 1000u, 1u);
    var seen = 0u;
    var b = BINS;
    loop {
        if (b == 0u) { break; }
        b -= 1u;
        seen += atomicLoad(&hist[b]);
        if (seen >= want) { break; }
    }
    // The bin's upper edge.
    S.top = exp2((f32(b) + 1.0) / 8.0 - 16.0);
    for (var j = 0u; j < BINS; j++) { atomicStore(&hist[j], 0u); }
}

// A value's brightness, 0 to 1, by the picked curve.
fn bright(v: f32) -> f32 {
    if (v <= 0.0) { return 0.0; }
    if (P.curve == 0u) { return 1.0 - exp(-v / P.knee); }
    return pow(clamp(v / S.top, 0.0, 1.0), P.gamma);
}

@compute @workgroup_size(256)
fn draw(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= P.w * P.h) { return; }
    let v = shown(i);
    let g = bright(v);
    var c = vec3<f32>(0.0, 0.0, 0.0);
    if (agar[i] == 0u) {
        if (P.colour == 0u) {
            c = vec3<f32>(8.0, 14.0, 18.0);
            // The Larder World as the CPU's `BigDish::image` draws it: a
            // wall's brightness is its health (a worn wall dims toward the
            // rock colour), rock apart from the world's edge.
            if (P.world != 0u) {
                let g = ground[i];
                if ((g & WALL) != 0u) {
                    var left = 1.0;
                    if (P.hp > 0.0) { left = min(health[i] / P.hp, 1.0); }
                    let a = 0.25 + 0.75 * max(left, 0.0);
                    c = vec3<f32>(34.0 + 116.0 * a, 30.0 + 120.0 * a, 28.0 + 130.0 * a);
                } else if ((g & ROCK) != 0u) {
                    c = vec3<f32>(34.0, 30.0, 28.0);
                }
            }
        }
    } else if (nsp() > 0u && P.field == 0u) {
        // Species: each field in its own colour through the same curve
        // (the soft curve's reference is the total's), the colours added
        // (examples/bigdish.rs `style=species`), on the amber ground or black.
        if (P.colour == 0u) {
            let sg = 1.0 - exp(-scent_in[i] / 60.0);
            c = vec3<f32>(12.0 + 30.0 * sg, 22.0 + 26.0 * sg, 26.0 + 10.0 * sg);
        }
        let cols = array<vec3<f32>, 4>(vec3<f32>(255.0, 170.0, 60.0), vec3<f32>(60.0, 200.0, 220.0), vec3<f32>(190.0, 110.0, 255.0), vec3<f32>(120.0, 230.0, 90.0));
        for (var s = 0u; s < nsp(); s++) { c += cols[s] * bright(sp_in[s * P.w * P.h + i]); }
    } else if (P.colour == 0u) {
        // The game's amber (`BigDish::image`): scent tints the ground.
        let sg = 1.0 - exp(-scent_in[i] / 60.0);
        var base = vec3<f32>(12.0 + 30.0 * sg, 22.0 + 26.0 * sg, 26.0 + 10.0 * sg);
        // The Larder World (`BigDish::image`): wet ground bluer; a
        // crystal's glow is its charge, full bright cyan, drained ground
        // again, with nothing drawn in between.
        if (P.world != 0u) {
            if ((ground[i] & WET) != 0u) { base = vec3<f32>(base.x * 0.7, base.y * 0.9, base.z + 16.0); }
            let pg = ground[i] & PILE_BITS;
            if (world(CRYSTALS) && pg != 0u && piles[pg - 1u].full > 0.0) { base += (vec3<f32>(140.0, 235.0, 255.0) - base) * (charge[i] / piles[pg - 1u].full); }
        }
        c = base + (vec3<f32>(255.0, 190.0, 90.0) - base) * g;
    } else {
        c = vec3<f32>(255.0 * g);
    }
    // Grazing: the grass in green under the mould, so the bare ground a
    // body leaves behind shows.
    if (graze_w() != 0.0 && agar[i] != 0u) { c += vec3<f32>(6.0, 30.0, 10.0) * grass[i]; }
    // With contagion on, the germ in red, full at four times the catch
    // level (the looks report's `style=sick`).
    if (P.sick_steps > 0u && agar[i] != 0u) {
        let gs = sqrt(clamp(germ_in[i] / (4.0 * max(P.catch_at, 1.0e-6)), 0.0, 1.0));
        c = vec3<f32>(max(c.x, 255.0 * gs), c.y * (1.0 - 0.8 * gs), c.z * (1.0 - 0.8 * gs));
    }
    // The look the view shades from (byte 2, the wound, is the trail pass's).
    let thick = u32(clamp(g, 0.0, 1.0) * 255.0);
    let crowd = min(atomicLoad(&occ[i]), 255u);
    let smell = u32(clamp(1.0 - exp(-scent_in[i] / 60.0), 0.0, 1.0) * 255.0);
    look[i] = thick | (crowd << 8u) | (look[i] & 0x00FF0000u) | (smell << 24u);
    // The panel, premultiplied, over the dish.
    let o = hud[i];
    let a = f32(o >> 24u) / 255.0;
    let hc = vec3<f32>(f32(o & 255u), f32((o >> 8u) & 255u), f32((o >> 16u) & 255u));
    c = hc + c * (1.0 - a);
    let q = vec3<u32>(clamp(c, vec3<f32>(0.0), vec3<f32>(255.0)));
    frame[i] = q.x | (q.y << 8u) | (q.z << 16u) | (255u << 24u);
}

// ---- The Larder World's passes (W1b, 2026-11-11) ----
// The CPU's order in a fine step: the move (blocked walkers bite walls),
// then eating, then conversion and jumps, then the trail and the scent.
// Here: `walls` and the crystal passes run after `walk` and before
// `convert` and the jumps, each only while the world has what it acts on.

// The bites a wall cell took this tick, applied as the CPU's `bite_walls`
// does (one `wall_bite` at a time); at 0 it turns to rubble: agar again,
// wet, open to scent. A thread a cell; only dispatched once a wall exists.
@compute @workgroup_size(256)
fn walls(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= P.w * P.h) { return; }
    let n = atomicLoad(&bites[i]);
    if (n == 0u) { return; }
    atomicStore(&bites[i], 0u);
    atomicAdd(&wstat[0], n);
    var hp = health[i];
    for (var q = 0u; q < n; q++) { hp -= P.bite; }
    if (hp <= 0.0 && (ground[i] & WALL) != 0u) {
        ground[i] = (ground[i] & ~(WALL | SCENT_WALL)) | WET;
        agar[i] = 1u;
        health[i] = 0.0;
        atomicAdd(&wstat[1], 1u);
    } else {
        health[i] = hp;
    }
}

// Eating (the CPU's `eat_crystals`): the n walkers on a crystal cell each
// take min(`eat`, charge / n), from the charge as the pass began, one
// after another floored at 0. A thread a listed crystal cell.
@compute @workgroup_size(256)
fn eat(@builtin(global_invocation_id) gid: vec3<u32>) {
    let q = gid.x;
    // In a garden every cell (a thread each), else the crystal cells.
    var i = q;
    if (garden()) {
        if (q >= P.w * P.h) { return; }
    } else {
        if (q >= P.npc) { return; }
        i = pcell[q];
    }
    var t = 0.0;
    let n = atomicLoad(&occ[i]);
    // With the life cycle a dormant walker counts in n, as on the CPU, but
    // takes nothing (only ordinary walkers eat).
    var eaters = n;
    if (cycles()) { eaters = n - atomicLoad(&gdep[2u * P.w * P.h + i]); }
    if (((ground[i] & PILE_BITS) != 0u || garden()) && n > 0u) {
        var c = charge[i];
        t = min(P.eat, c / f32(n));
        if (t > 0.0) {
            for (var m = 0u; m < eaters; m++) { c = max(c - t, 0.0); }
            charge[i] = c;
        }
    }
    share[i] = t;
}

// Each pile's charge and cells still crystal, summed in a workgroup (one a
// pile): each thread a stride of the pile's run, then a tree. The CPU sums
// in f64; this f32 tree is within a few parts in 10^7 of it. Also the
// share left, for the scent.
var<workgroup> psum: array<f32, 256>;
var<workgroup> pcnt: array<u32, 256>;

@compute @workgroup_size(256)
fn pilesum(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    let p = wid.x;
    let start = piles[p].start;
    let count = piles[p].count;
    var s = 0.0;
    var c = 0u;
    for (var q = li; q < count; q += 256u) {
        let i = pcell[start + q];
        if ((ground[i] & PILE_BITS) == p + 1u) {
            s += charge[i];
            c += 1u;
        }
    }
    psum[li] = s;
    pcnt[li] = c;
    workgroupBarrier();
    for (var k = 128u; k > 0u; k >>= 1u) {
        if (li < k) {
            psum[li] += psum[li + k];
            pcnt[li] += pcnt[li + k];
        }
        workgroupBarrier();
    }
    if (li == 0u) {
        let full = piles[p].full * f32(pcnt[0]);
        piles[p].sum = psum[0];
        piles[p].cells = pcnt[0];
        var left = 1.0;
        if (full > 0.0) { left = max(psum[0] / full, 0.0); }
        piles[p].left = left;
    }
}

// Flow (`crystal_flow`): each crystal cell's charge relaxes toward its
// pile's mean, then regrows (`crystal_regrow`) toward full, as the CPU's
// `eat_crystals` does after the eating. A thread a listed crystal cell.
@compute @workgroup_size(256)
fn flow(@builtin(global_invocation_id) gid: vec3<u32>) {
    let q = gid.x;
    if (q >= P.npc) { return; }
    let i = pcell[q];
    let p = ground[i] & PILE_BITS;
    if (p == 0u) { return; }
    var c = charge[i];
    if (P.flow > 0.0) {
        let mean = piles[p - 1u].sum / f32(piles[p - 1u].cells);
        c += P.flow * (mean - c);
    }
    let full = piles[p - 1u].full;
    if (P.regrow > 0.0 && c < full) { c += P.regrow * (full - c); }
    charge[i] = c;
}

// A wall (`Engine::build_wall`, the CPU's `build_wall`): the agar cells
// whose centres lie within `wb.r` of the segment leave the agar and lose
// their trail and charge (a crystal cell leaves its pile); scent cells
// under it lose their scent and, unless the wall lets scent through,
// close. A thread a cell of the wall's box.
@compute @workgroup_size(256)
fn build(@builtin(global_invocation_id) gid: vec3<u32>) {
    let q = gid.x;
    if (q >= wb.bw * wb.bh) { return; }
    let x = wb.x0 + q % wb.bw;
    let y = wb.y0 + q / wb.bw;
    if (x >= P.w || y >= P.h) { return; }
    let i = y * P.w + x;
    let px = f32(x) + 0.5;
    let py = f32(y) + 0.5;
    let dx = wb.bx - wb.ax;
    let dy = wb.by - wb.ay;
    let len2 = max(dx * dx + dy * dy, 1.0e-6);
    let t = clamp(((px - wb.ax) * dx + (py - wb.ay) * dy) / len2, 0.0, 1.0);
    let ex = px - wb.ax - t * dx;
    let ey = py - wb.ay - t * dy;
    if (sqrt(ex * ex + ey * ey) > wb.r) { return; }
    let was_open = agar[i] != 0u || (ground[i] & SCENT_WALL) != 0u;
    if (agar[i] != 0u) {
        agar[i] = 0u;
        var g = (ground[i] & ~PILE_BITS) | WALL;
        if (wb.scent != 0u) { g |= SCENT_WALL; }
        ground[i] = g;
        health[i] = wb.hp;
        charge[i] = 0.0;
        trail_in[i] = 0.0;
        trail_out[i] = 0.0;
        let c = P.w * P.h;
        for (var s = 0u; s < nsp(); s++) {
            sp_in[s * c + i] = 0.0;
            sp_out[s * c + i] = 0.0;
        }
    }
    if (was_open) {
        scent_in[i] = 0.0;
        scent_out[i] = 0.0;
    }
}

// A walker a new wall covers looks for the nearest free agar cell, ring by
// ring out to `wb.reach`, in the CPU's order within a ring, and claims it
// (lowest walker wins); `settle` moves the winners to its centre and
// `unclaim` clears the claims. `Engine::build_wall` runs rounds until no
// walker finds a cell, so walkers that lost one try the next; one with
// nowhere free stays, as on the CPU. The CPU moves them one at a time in walker order, so the
// cells they land on can differ.
@compute @workgroup_size(256)
fn evict(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    icell[k] = NONE;
    // Only a living walker, as the CPU's `build_wall`: an empty slot
    // (growth) keeps a stale place and no count. (Before 2026-11-11 an
    // empty slot under a new wall moved too, taking 1 from a count that did
    // not hold it, which could wrap to 2^32 - 1 and close the cell for
    // good, and adding a walker nobody was to the cell it found.) Spores
    // and the dead hold no cell (the life cycle).
    if (!lives(k)) { return; }
    let st = (spec[k] >> 9u) & 3u;
    if (P.cyc != 0u && (st == L_SPORE || st == L_DEAD)) { return; }
    let wk = walkers[k];
    let here = cell(wk.x, wk.y);
    if (here == NONE || agar[here] != 0u) { return; }
    let cx = i32(wk.x);
    let cy = i32(wk.y);
    let reach = i32(wb.reach);
    for (var d = 1; d <= reach; d++) {
        for (var oy = -d; oy <= d; oy++) {
            let edge = oy == -d || oy == d;
            var ox = -d;
            loop {
                if (ox > d) { break; }
                let nx = cx + ox;
                let ny = cy + oy;
                if (nx >= 0 && ny >= 0 && nx < i32(P.w) && ny < i32(P.h)) {
                    let j = u32(ny) * P.w + u32(nx);
                    if (agar[j] != 0u && (P.crowd == 0u || atomicLoad(&occ[j]) < P.crowd)) {
                        if (P.crowd > 0u) { atomicMin(&claim[j], k); }
                        icell[k] = j;
                        // Walkers that found a cell this round (`build_wall`
                        // runs rounds until none does).
                        atomicAdd(&wstat[2], 1u);
                        return;
                    }
                }
                if (edge) { ox += 1; } else { ox += 2 * d; }
            }
        }
    }
}

@compute @workgroup_size(256)
fn settle(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let j = icell[k];
    if (j == NONE) { return; }
    if (P.crowd > 0u && atomicLoad(&claim[j]) != k) { return; }
    let wk = walkers[k];
    atomicSub(&occ[cell(wk.x, wk.y)], 1u);
    atomicAdd(&occ[j], 1u);
    walkers[k] = vec4<f32>(f32(j % P.w) + 0.5, f32(j / P.w) + 0.5, wk.z, 0.0);
    icell[k] = NONE;
}

@compute @workgroup_size(256)
fn unclaim(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= P.w * P.h) { return; }
    atomicStore(&claim[i], NONE);
}

// ---------------------------------------------- the squad (Larder World W2a)
// The CPU twins and the order of a frame: `src/mould/squad.rs`. Before the
// frame's first tick: `shots`, then `strike` and `cut`, then `smell`; after
// its last: `units` and `harvest`.

// The largest r with r * r <= r2.
fn isqrt(r2: u32) -> i32 {
    var r = 0;
    loop {
        let n = r + 1;
        if (u32(n * n) > r2) { break; }
        r = n;
    }
    return r;
}

// Whether a shot stops at cell i: slime, or a walker in a crater (which
// holds no trail; `squad::shots`).
fn stops(i: u32) -> bool {
    return enemy_trail(i) >= P.slime || ((P.scorched & 1u) != 0u && crater_at(i) > 0.0 && enemy_occ(i) > 0u);
}

// Each shot walks its cells and stops at the first off the agar (a wall or
// rock: no hit) or the first that looks like slime (the hit), or a walker
// in a crater (2026-11-12). The owner, 2026-11-11 16:48Z: "Gun fire should
// hit the first slime in its path". With the sink (`shot_sink`, 2026-11-12)
// the blast then sinks in: on along the line while the cells are agar and
// within the sink's reach of the first hit, to the last that stops a shot
// (`squad::sink`).
@compute @workgroup_size(256)
fn shots(@builtin(global_invocation_id) gid: vec3<u32>) {
    let q = gid.x;
    if (q >= P.nshots) { return; }
    let s = SQ.shots[q];
    var hit = NONE;
    var m = 0u;
    for (; m < s.count; m++) {
        let i = SQ.path[s.start + m];
        if (agar[i] == 0u) { break; }
        if (stops(i)) { hit = i; break; }
    }
    if (hit != NONE && P.shot_sink > 0.0) {
        let reach2 = u32(floor(P.shot_sink * f32(s.r2)));
        let hx = i32(hit % P.w);
        let hy = i32(hit / P.w);
        var deep = hit;
        for (var n = m + 1u; n < s.count; n++) {
            let c = SQ.path[s.start + n];
            let dx = i32(c % P.w) - hx;
            let dy = i32(c / P.w) - hy;
            if (agar[c] == 0u || u32(dx * dx + dy * dy) > reach2) { break; }
            if (stops(c)) { deep = c; }
        }
        hit = deep;
    }
    SQ.sout[q].hit = hit;
}

// Whether cell c is inside shot q's strike.
fn struck(c: u32, q: u32) -> bool {
    let hit = SQ.sout[q].hit;
    if (hit == NONE) { return false; }
    let dx = i32(c % P.w) - i32(hit % P.w);
    let dy = i32(c / P.w) - i32(hit / P.w);
    return u32(dx * dx + dy * dy) <= SQ.shots[q].r2;
}

// A living walker on a struck cell dies as a starving one does (`life`),
// counted for the first shot that struck it. Growth must be on: an empty
// slot is how a walker dies here.
@compute @workgroup_size(256)
fn strike(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n || !lives(k)) { return; }
    // Spores in flight and the dead are missed (`squad::shots`); a struck
    // walker is dead as well as gone (the CPU's `kill`).
    let st = state(k);
    if (st == L_SPORE || st == L_DEAD) { return; }
    // In a garden the player's walkers are not struck (`Garden::shots`).
    if (garden() && species_of(k) == OWN_SP) { return; }
    let wk = walkers[k];
    let c = cell(wk.x, wk.y);
    for (var q = 0u; q < P.nshots; q++) {
        if (struck(c, q)) {
            atomicSub(&occ[c], 1u);
            atomicStore(&claim[c], NONE);
            energy[k] = -1.0;
            if (cycles()) { spec[k] = (spec[k] & 0xFFFF01FFu) | (L_DEAD << 9u); }
            atomicAdd(&kills[q], 1u);
            return;
        }
    }
}

// A strike clears the trail on its cells: a workgroup a shot, a thread a
// cell of the 15 x 15 box round its hit (radius 7 at most, `squad::R_MAX`),
// so the pass costs the shots, not the world. Two shots clearing one cell
// write the same 0.
@compute @workgroup_size(256)
fn cut(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    let q = wid.x;
    let hit = SQ.sout[q].hit;
    if (q >= P.nshots || hit == NONE || li >= 225u) { return; }
    let x = i32(hit % P.w) + i32(li % 15u) - 7;
    let y = i32(hit / P.w) + i32(li / 15u) - 7;
    if (x < 0 || y < 0 || x >= i32(P.w) || y >= i32(P.h)) { return; }
    let i = u32(y) * P.w + u32(x);
    if (struck(i, q)) {
        // The look's wound (byte 2, which the trail pass fades): the
        // thickness the strike cut, as a stand-in hit's (`set_hits`). The
        // game's shots cut here, not in the trail pass, so without this
        // the window showed the cut but no wound (W2c, 2026-11-12). Two
        // strikes on one cell may race on it; only the picture differs.
        let cut = u32(bright(enemy_trail(i)) * 255.0);
        let had = (look[i] >> 16u) & 255u;
        look[i] = (look[i] & 0xFF00FFFFu) | (max(had, cut) << 16u);
        if (garden()) {
            // Only the enemy's trail goes: every species' but the player's,
            // and the shared trail down to the player's.
            let c = P.w * P.h;
            for (var s = 0u; s < nsp(); s++) {
                if (s != OWN_SP) { sp_in[s * c + i] = 0.0; }
            }
            trail_in[i] = sp_in[OWN_SP * c + i];
        } else {
            trail_in[i] = 0.0;
        }
        // The crater (`squad::Frame::crater`) on the struck agar cells, the
        // longer of the two. Without the cone every shot in a frame writes
        // the same steps.
        // With the cone (flag bit 1, `BigParams::scorch_cone`) a cell gets
        // fewer steps by its squared distance from the hit, in whole
        // numbers as the CPU's `shots` computes them.
        if (P.crater > 0.0 && agar[i] != 0u) {
            var steps = P.crater;
            if ((P.scorched & 2u) != 0u) {
                let dx = x - i32(hit % P.w);
                let dy = y - i32(hit / P.w);
                let c = u32(P.crater);
                steps = f32(c - c * u32(dx * dx + dy * dy) / (SQ.shots[q].r2 + 1u));
            }
            // Steps are whole and at least 0, so their bits sort as they
            // do and two overlapping cones keep the larger, in any order.
            atomicMax(&crater[i], bitcast<u32>(steps));
        }
    }
}

// A hit lays the alarm (`squad::SHOT_ALARM`) on each agar cell within
// `SHOT_ALARM_R` times its radius (`ar2`), as the CPU's `shots` does after
// the cut, with the alarm field on. One workgroup, the shots in order, and
// each cell always added to by the same thread (`owned_from`), so two
// discs on one cell add in the CPU's order and never race.
// Until 2026-11-12 a thread took a cell of the shot's box by its place in
// the box, with a storage barrier after each shot; on the GTX 1070 threads
// that met on a cell that way lost each other's adds (the smell and harvest
// passes below, written the same way, changed the farm's game from run to
// run there), though llvmpipe hid it.
@compute @workgroup_size(256)
fn shout(@builtin(local_invocation_index) li: u32) {
    let c = P.w * P.h;
    let tx = i32(li % 16u);
    let ty = i32(li / 16u);
    for (var q = 0u; q < P.nshots; q++) {
        let hit = SQ.sout[q].hit;
        if (hit == NONE) { continue; }
        let ar2 = SQ.shots[q].ar2;
        let r = isqrt(ar2);
        let hx = i32(hit % P.w);
        let hy = i32(hit / P.w);
        for (var y = owned_from(hy - r, ty); y <= hy + r; y += 16) {
            for (var x = owned_from(hx - r, tx); x <= hx + r; x += 16) {
                let dx = x - hx;
                let dy = y - hy;
                if (x >= 0 && y >= 0 && x < i32(P.w) && y < i32(P.h) && u32(dx * dx + dy * dy) <= ar2) {
                    let i = u32(y) * P.w + u32(x);
                    if (agar[i] != 0u) { germ_in[c + i] += P.shot_alarm; }
                }
            }
        }
    }
}

// The first whole number at or after `lo` that is `t` mod 16: a thread's
// first row or column in a box, when thread `tx + 16 ty` owns every cell
// whose x is `tx` mod 16 and y is `ty` mod 16. A pass that adds to cells
// from many discs in one workgroup goes through its own cells only, so a
// cell is read and written by one thread, in order, and needs no barrier.
fn owned_from(lo: i32, t: i32) -> i32 {
    return lo + ((t - lo) % 16 + 16) % 16;
}

// The squad's three unit passes (`smell`, `units`, `harvest`) each ran on
// one thread until 2026-11-12, a unit's whole disc or sight walked alone:
// at scale 8 a unit's sight is about 3,700 blocks of 16 cells, and on the
// GTX 1070 the `units` pass alone took 10.4 ms of a 100k step's 31.7 ms,
// `smell` 3.9 ms and `harvest` 0.8 ms (`bench passes=N`). Now the threads of
// a workgroup share the cells; each result is the same as the one
// thread's, bit for bit (the squad twin tests in `mouldgpu`). A cell that
// several units' discs add to or take from is always one thread's
// (`owned_from`, and the harvest's columns): the first version shared
// them out by place in each disc's box with a storage barrier between
// units, which llvmpipe ran right and the 1070 did not (the farm's 20 s
// harvest at scale 2 came out 1,478 to 1,785 against 1,953, run to run).
// The one-thread passes are kept (`smell1`, `units1`, `harvest1`,
// `Engine::squad_one`) to compare on a GPU.

// Each unit's smell on the agar cells of its disc. One workgroup, the units
// in order; each thread adds to its own cells of each disc (`owned_from`),
// so overlapping discs add up in the CPU's order without a race.
@compute @workgroup_size(256)
fn smell(@builtin(local_invocation_index) li: u32) {
    let tx = i32(li % 16u);
    let ty = i32(li / 16u);
    for (var u = 0u; u < P.nunits; u++) {
        let un = SQ.units[u];
        if (un.smell <= 0.0) { continue; }
        let r = isqrt(un.smell2);
        let cx = i32(un.x);
        let cy = i32(un.y);
        for (var y = owned_from(cy - r, ty); y <= cy + r; y += 16) {
            for (var x = owned_from(cx - r, tx); x <= cx + r; x += 16) {
                let dx = x - cx;
                let dy = y - cy;
                if (x >= 0 && y >= 0 && x < i32(P.w) && y < i32(P.h) && u32(dx * dx + dy * dy) <= un.smell2) {
                    let i = u32(y) * P.w + u32(x);
                    if (agar[i] != 0u) { scent_in[i] += un.smell; }
                }
            }
        }
    }
}

// Whether every cell after the first on the line from (cx, cy) to (tx, ty)
// is agar: max(|dx|, |dy|) whole-cell steps, offsets truncated toward zero
// (`squad::sees`, the same cells). A unit aims only at a block it sees
// (2026-11-13): through a wall the game held the shot, so a squad beside a
// wall with thicker slime behind it fired at nothing in front.
fn sees(cx: i32, cy: i32, tx: i32, ty: i32) -> bool {
    let dx = tx - cx;
    let dy = ty - cy;
    let n = max(max(abs(dx), abs(dy)), 1);
    for (var k = 1; k <= n; k++) {
        let x = cx + dx * k / n;
        let y = cy + dy * k / n;
        if (agar[u32(y) * P.w + u32(x)] == 0u) { return false; }
    }
    return true;
}

var<workgroup> rsum: array<u32, 256>;
var<workgroup> rbest: array<u32, 256>;
var<workgroup> rbi: array<u32, 256>;

// Each unit's walkers within `touch`, and the 4 x 4 block with the most
// walkers whose centre is within `sight` and that it sees (`sees`; ties to
// the first, row-major; the line is walked only for a block that would
// beat the thread's best, which picks the same block). A workgroup a unit: each thread takes every 256th cell of the touch disc's
// box and every 256th block of the sight's, in row-major order (so its own
// best is its first), then the threads reduce: the sums add, and the best
// block is the most walkers, ties to the lower index, as one thread
// walking them in order keeps.
@compute @workgroup_size(256)
fn units(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    let u = wid.x;
    if (u >= P.nunits) { return; }
    let un = SQ.units[u];
    let cx = i32(un.x);
    let cy = i32(un.y);
    let w = i32(P.w);
    let h = i32(P.h);
    // Whole walkers, as `squad::read`; the game weighs them per rule (`Rules::mass`).
    var touching = 0u;
    let r = isqrt(un.touch2);
    let side = u32(2 * r + 1);
    for (var j = li; j < side * side; j += 256u) {
        let dx = i32(j % side) - r;
        let dy = i32(j / side) - r;
        let x = cx + dx;
        let y = cy + dy;
        if (x < 0 || y < 0 || x >= w || y >= h || u32(dx * dx + dy * dy) > un.touch2) { continue; }
        touching += enemy_occ(u32(y * w + x));
    }
    let b = 4;
    let nbw = (w + b - 1) / b;
    let nbh = (h + b - 1) / b;
    let rs = isqrt(un.sight2) + 2;
    let by0 = max(cy - rs, 0) / b;
    let by1 = min((cy + rs) / b, nbh - 1);
    let bx0 = max(cx - rs, 0) / b;
    let bx1 = min((cx + rs) / b, nbw - 1);
    var best = 0u;
    var bi = NONE;
    if (by1 >= by0 && bx1 >= bx0) {
        let bw = u32(bx1 - bx0 + 1);
        let nb = bw * u32(by1 - by0 + 1);
        for (var j = li; j < nb; j += 256u) {
            let bx = bx0 + i32(j % bw);
            let by = by0 + i32(j / bw);
            let mx = bx * b + b / 2 - cx;
            let my = by * b + b / 2 - cy;
            if (u32(mx * mx + my * my) > un.sight2) { continue; }
            var n = 0u;
            for (var y = by * b; y < min((by + 1) * b, h); y++) {
                for (var x = bx * b; x < min((bx + 1) * b, w); x++) {
                    n += enemy_occ(u32(y * w + x));
                }
            }
            if (n > best && sees(cx, cy, min(bx * b + b / 2, w - 1), min(by * b + b / 2, h - 1))) { best = n; bi = u32(by * nbw + bx); }
        }
    }
    rsum[li] = touching;
    rbest[li] = best;
    rbi[li] = bi;
    workgroupBarrier();
    for (var s = 128u; s > 0u; s = s >> 1u) {
        if (li < s) {
            rsum[li] += rsum[li + s];
            let ob = rbest[li + s];
            let oi = rbi[li + s];
            if (ob > rbest[li] || (ob == rbest[li] && oi < rbi[li])) {
                rbest[li] = ob;
                rbi[li] = oi;
            }
        }
        workgroupBarrier();
    }
    if (li == 0u) {
        SQ.uout[u].touching = rsum[0];
        SQ.uout[u].aim = rbi[0];
        SQ.uout[u].dense = rbest[0];
    }
}

var<workgroup> hc: array<f32, 256>;
var<workgroup> hrem: f32;
var<workgroup> hpieces: u32;

// The harvest: in unit order, each unit with `take` takes up to that much
// charge from the crystal cells of its disc, cell by cell (row-major), as
// the CPU does. One workgroup, the units in order, and a disc's box in
// pieces of a row each (256 cells of it at most): the threads load the
// piece, thread 0 takes from it in order, the threads write back what it
// took, and the unit stops at the piece that filled it. A cell is always
// loaded and written by the thread whose number is its x mod 256, so two
// units on one crystal take in order, the second seeing what the first
// left, without a race.
@compute @workgroup_size(256)
fn harvest(@builtin(local_invocation_index) li: u32) {
    for (var u = 0u; u < P.nunits; u++) {
        let un = SQ.units[u];
        let on = un.take > 0.0 && world(CRYSTALS);
        let r = isqrt(un.take2);
        let side = u32(2 * r + 1);
        let segs = (side + 255u) / 256u;
        let cx = i32(un.x);
        let cy = i32(un.y);
        if (li == 0u) {
            hrem = un.take;
            hpieces = select(0u, side * segs, on);
        }
        let pieces = workgroupUniformLoad(&hpieces);
        for (var q = 0u; q < pieces; q++) {
            // Piece q: row q / segs of the box, its 256-cell stretch q % segs;
            // this thread's cell of it, k along, is the one whose x is li
            // mod 256.
            let y = cy - r + i32(q / segs);
            let xs = cx - r + 256 * i32(q % segs);
            let x = xs + ((i32(li) - xs) % 256 + 256) % 256;
            let k = u32(x - xs);
            let dx = x - cx;
            let dy = y - cy;
            var i = NONE;
            var c = 0.0;
            if (x <= cx + r && x >= 0 && y >= 0 && x < i32(P.w) && y < i32(P.h) && u32(dx * dx + dy * dy) <= un.take2) {
                i = u32(y) * P.w + u32(x);
                c = charge[i];
            }
            hc[k] = c;
            workgroupBarrier();
            // Only the stretch's own cells (a box narrower than 256 leaves
            // the rest at 0): over all 256 the pass took 0.49 ms a step at
            // 5k walkers on the 1070, against 0.09 for the one-thread pass
            // (harvest1), and 1.33 against 0.77 at 100k (2026-11-12).
            let n = u32(min(256, cx + r - xs + 1));
            if (li == 0u) {
                var rem = hrem;
                for (var j = 0u; j < n && rem > 0.0; j++) {
                    let cj = hc[j];
                    if (cj > 0.0) {
                        let t = min(cj, rem);
                        hc[j] = cj - t;
                        rem = rem - t;
                    }
                }
                hrem = rem;
            }
            workgroupBarrier();
            if (i != NONE && c > 0.0) { charge[i] = hc[k]; }
            if (workgroupUniformLoad(&hrem) <= 0.0) { break; }
        }
        if (li == 0u) { SQ.uout[u].took = select(0.0, un.take - hrem, on); }
    }
}

// The three unit passes as they were before 2026-11-12, one thread each
// (`Engine::squad_one`): kept to compare the two on a GPU.
// Each unit's smell on the agar cells of its disc. One thread, units in
// order, so overlapping discs add up without a race.
@compute @workgroup_size(1)
fn smell1(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x != 0u) { return; }
    for (var u = 0u; u < P.nunits; u++) {
        let un = SQ.units[u];
        if (un.smell <= 0.0) { continue; }
        let r = isqrt(un.smell2);
        for (var dy = -r; dy <= r; dy++) {
            for (var dx = -r; dx <= r; dx++) {
                let x = i32(un.x) + dx;
                let y = i32(un.y) + dy;
                if (x < 0 || y < 0 || x >= i32(P.w) || y >= i32(P.h) || u32(dx * dx + dy * dy) > un.smell2) { continue; }
                let i = u32(y) * P.w + u32(x);
                if (agar[i] != 0u) { scent_in[i] += un.smell; }
            }
        }
    }
}

// Each unit's walkers within `touch`, and the 4 x 4 block with the most
// walkers whose centre is within `sight` and that it sees (ties to the
// first, row-major).
@compute @workgroup_size(64)
fn units1(@builtin(global_invocation_id) gid: vec3<u32>) {
    let u = gid.x;
    if (u >= P.nunits) { return; }
    let un = SQ.units[u];
    let cx = i32(un.x);
    let cy = i32(un.y);
    let w = i32(P.w);
    let h = i32(P.h);
    // Whole walkers, as `squad::read`; the game weighs them per rule (`Rules::mass`).
    var touching = 0u;
    let r = isqrt(un.touch2);
    for (var dy = -r; dy <= r; dy++) {
        for (var dx = -r; dx <= r; dx++) {
            let x = cx + dx;
            let y = cy + dy;
            if (x < 0 || y < 0 || x >= w || y >= h || u32(dx * dx + dy * dy) > un.touch2) { continue; }
            touching += atomicLoad(&occ[u32(y * w + x)]);
        }
    }
    let b = 4;
    let nbw = (w + b - 1) / b;
    let nbh = (h + b - 1) / b;
    let rs = isqrt(un.sight2) + 2;
    let by0 = max(cy - rs, 0) / b;
    let by1 = min((cy + rs) / b, nbh - 1);
    let bx0 = max(cx - rs, 0) / b;
    let bx1 = min((cx + rs) / b, nbw - 1);
    var best = 0u;
    var bi = NONE;
    for (var by = by0; by <= by1; by++) {
        for (var bx = bx0; bx <= bx1; bx++) {
            let mx = bx * b + b / 2 - cx;
            let my = by * b + b / 2 - cy;
            if (u32(mx * mx + my * my) > un.sight2) { continue; }
            var n = 0u;
            for (var y = by * b; y < min((by + 1) * b, h); y++) {
                for (var x = bx * b; x < min((bx + 1) * b, w); x++) {
                    n += atomicLoad(&occ[u32(y * w + x)]);
                }
            }
            if (n > best && sees(cx, cy, min(bx * b + b / 2, w - 1), min(by * b + b / 2, h - 1))) { best = n; bi = u32(by * nbw + bx); }
        }
    }
    SQ.uout[u].touching = touching;
    SQ.uout[u].aim = bi;
    SQ.uout[u].dense = best;
}

// The harvest: in unit order, each unit with `take` takes up to that much
// charge from the crystal cells of its disc, cell by cell. One thread, so
// two units on one crystal take in order.
@compute @workgroup_size(1)
fn harvest1(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x != 0u) { return; }
    for (var u = 0u; u < P.nunits; u++) {
        let un = SQ.units[u];
        var took = 0.0;
        if (un.take > 0.0 && world(CRYSTALS)) {
            var rem = un.take;
            let r = isqrt(un.take2);
            var done = false;
            for (var dy = -r; dy <= r && !done; dy++) {
                for (var dx = -r; dx <= r; dx++) {
                    let x = i32(un.x) + dx;
                    let y = i32(un.y) + dy;
                    if (x < 0 || y < 0 || x >= i32(P.w) || y >= i32(P.h) || u32(dx * dx + dy * dy) > un.take2) { continue; }
                    let i = u32(y) * P.w + u32(x);
                    let c = charge[i];
                    if (c > 0.0) {
                        let t = min(c, rem);
                        charge[i] = c - t;
                        rem = rem - t;
                        if (rem <= 0.0) { done = true; break; }
                    }
                }
            }
            took = un.take - rem;
        }
        SQ.uout[u].took = took;
    }
}

// ------------------------------------------- the life cycle (2026-11-11)
// The CPU's `life_step`, after the move (and any wall bites) and before
// eating: `fate` (timers, the walkers that fruit or fall dormant, the
// dormant that lay and wake), `fly` (spores steer, fly, lay and claim a
// landing) and `alight` (the winners land). The
// strike (`Engine::strike`, the CPU's `BigDish::strike`) is `hit` and
// `kill`, on their own between ticks.

// A walker's counters after the tick's deposit (the CPU counts every
// walker's sick counter down and every living one's age up).
fn tally(k: u32) {
    if (P.sick_steps > 0u) {
        let n = sick[k];
        if (n > 0u) { sick[k] = n - 1u; }
    }
    if (explore() > 0u) { age[k] = min(age[k] + 1u, 65535u); }
}

// What a walker that became an ordinary one after the move (it woke or
// landed) does where it stands, as an ordinary walker does in `walk`: it
// counts for the capped deposit, grazes, and lays germ if sick. (Without
// the cap it lays no trail: on the CPU only a walker that moved does.)
fn now_active(k: u32, at: u32, ill: bool, ex: bool) {
    let c = P.w * P.h;
    let o = blk(species_of(k), kind_of(k)) * c + at;
    if (grazes() && eats(k)) { atomicAdd(&gcount[at], 1u); }
    if (P.cap > 0u) {
        if (ex) {
            atomicAdd(&edep[o + select(0u, nblk() * c, ill)], 1u);
        } else if (ill) {
            atomicAdd(&sdep[o], 1u);
        } else {
            atomicAdd(&dep[o], 1u);
        }
    }
    if (P.sick_steps > 0u && ill) { atomicAdd(&gdep[at], 1u); }
}

fn ill_of(k: u32) -> bool { return P.sick_steps > 0u && sick[k] > P.immune; }
fn ex_of(k: u32) -> bool { return explore() > 0u && explores(k); }

@compute @workgroup_size(256)
fn fate(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let w = spec[k];
    var st = (w >> 9u) & 3u;
    var tm = w >> 16u;
    // Every ordinary or dormant slot's timer counts down, an empty one's
    // too, as on the CPU (a birth sets it to 0).
    if (st == L_WALKER || st == L_DORMANT) { tm = max(tm, 1u) - 1u; }
    if (!lives(k)) {
        spec[k] = (w & 0xFFFFu) | (tm << 16u);
        return;
    }
    let wk = walkers[k];
    let here = cell(wk.x, wk.y);
    let c = P.w * P.h;
    var mark = 0u;
    if ((w & FRUIT) != 0u) {
        // It leaves its cell and takes a new heading; it flies from the
        // next tick.
        atomicSub(&occ[here], 1u);
        st = L_SPORE;
        tm = P.spore_steps;
        mark = FRUIT;
        walkers[k] = vec4<f32>(wk.x, wk.y, unit(keyed(P.s_spore_h, S.t, k)) * TAU, 0.0);
    } else if ((w & DOZE) != 0u) {
        st = L_DORMANT;
        tm = P.sleep_rest;
        atomicAdd(&gdep[2u * c + here], 1u);
    } else if (st == L_DORMANT && P.sleep_death > 0.0 && unit(keyed(P.s_sleep_die, S.t, k)) < P.sleep_death) {
        // It dies where it lies (`BigParams::sleep_death`, the CPU's draw
        // `DRAW_SLEEP_DIE`): its cell is freed, and its claim as a starving
        // walker's is (`life`), so a birth can take it this tick; with
        // growth its slot too. It lays nothing and is not counted dormant.
        // Its sick counter counts down as every slot's does on the CPU; it
        // ages only while its slot lives (without growth).
        atomicSub(&occ[here], 1u);
        atomicStore(&claim[here], NONE);
        st = L_DEAD;
        if (growth()) {
            energy[k] = -1.0;
            if (P.sick_steps > 0u && sick[k] > 0u) { sick[k] -= 1u; }
        } else {
            tally(k);
        }
    } else if (st == L_DORMANT) {
        // It lays, then wakes on scent, or on alarm once its timer is out
        // (every wake decided from the alarm as the tick began; the relays
        // add to it in `trail`).
        atomicAdd(&bdep[blk(species_of(k), kind_of(k)) * c + here + 3u * nblk() * c], 1u);
        let ill = ill_of(k);
        let ex = ex_of(k);
        if (scent_at(wk.x, wk.y) > P.wake_scent || (alarm_at(here) > P.wake_alarm && tm == 0u)) {
            st = L_WALKER;
            tm = P.wake_steps;
            if (alarms()) { atomicAdd(&gdep[c + here], 1u); }
            now_active(k, here, ill, ex);
        } else {
            atomicAdd(&gdep[2u * c + here], 1u);
        }
        tally(k);
    }
    spec[k] = (w & 0x1FFu) | (st << 9u) | mark | (min(tm, 65535u) << 16u);
}

// What a spore's sensor reads: the scent `spore_reach` out, a wall or the
// dish's edge -1e6.
fn spore_sense(x: f32, y: f32, a: f32) -> f32 {
    let i = cell(x + cos(a) * P.spore_reach, y + sin(a) * P.spore_reach);
    if (i == NONE || agar[i] == 0u) { return -1.0e6; }
    return scent_in[i];
}

@compute @workgroup_size(256)
fn fly(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    icell[k] = NONE;
    if (!lives(k)) { return; }
    let w = spec[k];
    if (((w >> 9u) & 3u) != L_SPORE) { return; }
    if ((w & FRUIT) != 0u) { spec[k] = w & ~FRUIT; return; }
    let t = S.t;
    let wk = walkers[k];
    var h = wk.z;
    // T, before this tick's count: the first half of the flight straight,
    // the second steering by scent; at 0 it drifts straight.
    let tt = w >> 16u;
    let homing = tt > 0u && tt <= P.spore_steps / 2u;
    if (P.spore_reach > 0.0 && homing) {
        let f = spore_sense(wk.x, wk.y, h);
        let fl = spore_sense(wk.x, wk.y, h - P.sa);
        let fr = spore_sense(wk.x, wk.y, h + P.sa);
        if (f > fl && f > fr) {
        } else if (f < fl && f < fr) {
            if ((keyed(P.s_spore_h, t, k | 0x80000000u) & 1u) == 0u) { h += P.ra; } else { h -= P.ra; }
        } else if (fl < fr) {
            h += P.ra;
        } else if (fr < fl) {
            h -= P.ra;
        }
    }
    // It flies `spore_speed` x `step` ignoring the crowd, or, off the agar,
    // stays with a new heading.
    let nx = wk.x + cos(h) * P.spore_st;
    let ny = wk.y + sin(h) * P.spore_st;
    let j = cell(nx, ny);
    var at = vec4<f32>(nx, ny, h, 0.0);
    if (j == NONE || agar[j] == 0u) { at = vec4<f32>(wk.x, wk.y, unit(keyed(P.s_spore_h, t, k | 0x40000000u)) * TAU, 0.0); }
    walkers[k] = at;
    let i = cell(at.x, at.y);
    let c = P.w * P.h;
    atomicAdd(&bdep[blk(species_of(k), kind_of(k)) * c + i + 2u * nblk() * c], 1u);
    let t2 = max(tt, 1u) - 1u;
    spec[k] = (w & 0xFFFFu) | (t2 << 16u);
    // It lands with its timer out, or homing over `spore_land` of scent:
    // on agar under its species' crowd limit, the lowest keyed claim
    // winning (`CLAIM_LAND`), in `alight`. Every lander claims, as on the
    // CPU, a species without a limit too: its claim can beat a limited
    // species' spore to the cell, though it lands either way. (Claiming
    // only under a limit, the first version, let both land where species
    // with their own limits met: 13 walkers and 21 counts off in a burst
    // that lands at once.)
    let lands = t2 == 0u || (homing && P.spore_land > 0.0 && scent_at(at.x, at.y) > P.spore_land);
    let cr = crowd_of(species_of(k));
    if (lands && agar[i] != 0u && (cr == 0u || atomicLoad(&occ[i]) < cr)) {
        atomicMin(&claim[i], keyed(P.s_land, t, k));
        icell[k] = i;
        return;
    }
    tally(k);
}

@compute @workgroup_size(256)
fn alight(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let i = icell[k];
    if (i == NONE) { return; }
    let cr = crowd_of(species_of(k));
    let ill = ill_of(k);
    let ex = ex_of(k);
    let key = keyed(P.s_land, S.t, k);
    let won = atomicLoad(&claim[i]) == key;
    if (cr == 0u || won) {
        // The lowest claim clears the cell's: a loser reads the winner's
        // key or none, so loses either way, and the passes after meet fresh
        // claims. A spore of a species without a limit lands either way.
        if (won) { atomicStore(&claim[i], NONE); }
        atomicAdd(&occ[i], 1u);
        spec[k] = (spec[k] & 0x1FFu) | (min(P.spore_rest, 65535u) << 16u);
        now_active(k, i, ill, ex);
    }
    tally(k);
}

// The strike's cells: within `kr` its trail (each species' too) is
// cleared, within `kar` `kalarm` is added to the alarm on agar. As the
// CPU, the struck cells hold the points at whole-cell offsets (dx, dy)
// from the centre, by dx^2 + dy^2.
@compute @workgroup_size(256)
fn hit(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    let c = P.w * P.h;
    if (i >= c) { return; }
    let dx = f32(i32(i % P.w) - i32(P.kx));
    let dy = f32(i32(i / P.w) - i32(P.ky));
    let d2 = dx * dx + dy * dy;
    if (P.kr > 0.0 && d2 <= P.kr * P.kr) {
        // A burst takes both moulds alike in a garden too, as the CPU's
        // (`Garden::burst`, `Garden::cord`): the det cord is the player's
        // cut, for shaping their own mould as well as the enemy's.
        trail_in[i] = 0.0;
        for (var s = 0u; s < P.nsp; s++) { sp_in[s * c + i] = 0.0; }
    }
    if (P.alarm_on != 0u && d2 <= P.kar * P.kar && agar[i] != 0u) { germ_in[c + i] += P.kalarm; }
    // A burst's crater: the longer of the cell's and the burst's, on agar,
    // as `scorch_disc` (its own radius: a kit blast's crater, 7 dish
    // cells, is wider than its kill, 6).
    if (P.kcs > 0.0 && d2 <= P.kcr * P.kcr && agar[i] != 0u) { atomicMax(&crater[i], bitcast<u32>(P.kcs)); }
}

// The strike's walkers: every living one within `kr` that holds a cell
// dies (spores in flight are missed); with growth its slot is freed.
@compute @workgroup_size(256)
fn kill(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n || P.kr <= 0.0) { return; }
    if (P.upkeep > 0.0 && energy[k] < 0.0) { return; }
    let w = spec[k];
    let st = (w >> 9u) & 3u;
    if (st == L_SPORE || st == L_DEAD) { return; }
    let wk = walkers[k];
    let dx = wk.x - P.kx;
    let dy = wk.y - P.ky;
    if (sqrt(dx * dx + dy * dy) > P.kr) { return; }
    atomicSub(&occ[cell(wk.x, wk.y)], 1u);
    spec[k] = (w & 0xFFFF01FFu) | (L_DEAD << 9u);
    if (P.upkeep > 0.0) { energy[k] = -1.0; }
    atomicAdd(&wstat[3], 1u);
}

// The living walkers (the CPU's `living`: a slot alive, with growth, and
// not dead) into `wstat[4]`, and with `grid` the mould grid (the CPU's
// `mould_grid`: each `grid`-cell square's walkers holding a cell, its
// `occ`) into `wstat[8..]`, row by row; `Engine::record_count` clears both
// first and dispatches a thread for each slot and each cell. The game's
// packed readout copies them, so neither costs a read-back of its own (the
// count read the energy and `spec` back, 8 MB at 1M slots, until
// 2026-11-12). Energy is below 0 only with growth on (an empty or dead
// slot), and the state bits are 0 with the life cycle off. One atomic a
// living walker: a workgroup's sum through shared memory cost 1.5 ms a
// game step on software Vulkan (its barriers), this nothing measurable.
@compute @workgroup_size(256)
fn count(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k < P.n && energy[k] >= 0.0 && ((spec[k] >> 9u) & 3u) != L_DEAD) {
        atomicAdd(&wstat[4], 1u);
        // The player's own mould's share of them, in a garden.
        if (garden() && species_of(k) == OWN_SP) { atomicAdd(&wstat[5], 1u); }
    }
    let g = P.grid;
    if (g > 0u && k < P.w * P.h) {
        let o = atomicLoad(&occ[k]);
        if (o > 0u) {
            let gw = (P.w + g - 1u) / g;
            atomicAdd(&wstat[8u + (k / P.w / g) * gw + (k % P.w) / g], o);
        }
    }
}

// ---------------------------------------------- the gardeners (own_mould)
// `Garden::do_task` on the GPU, before the frame's shots as the CPU's
// `World::tend` runs before its frame: each tend entry (`squad::TEND`, eight
// words: fine cell x, y, the task, the disc's squared radius, the trail a
// Road lays on each agar cell, the charge a Feed spreads on each) on the
// agar of its disc. One workgroup, entries in order, each cell one
// thread's (`owned_from`), so overlapping discs add in the CPU's order.
// Cut is not here: on the GPU the cut tool is the det cord, a burst.
@compute @workgroup_size(256)
fn tend(@builtin(local_invocation_index) li: u32) {
    let tx = i32(li % 16u);
    let ty = i32(li / 16u);
    let c = P.w * P.h;
    for (var j = 0u; j < P.ntend; j++) {
        let b = P.tend_at + 8u * j;
        let cx = i32(SQ.path[b]);
        let cy = i32(SQ.path[b + 1u]);
        let task = SQ.path[b + 2u];
        let r2 = SQ.path[b + 3u];
        let road = bitcast<f32>(SQ.path[b + 4u]);
        let feed = bitcast<f32>(SQ.path[b + 5u]);
        let r = isqrt(r2);
        for (var y = owned_from(cy - r, ty); y <= cy + r; y += 16) {
            for (var x = owned_from(cx - r, tx); x <= cx + r; x += 16) {
                let dx = x - cx;
                let dy = y - cy;
                if (x >= 0 && y >= 0 && x < i32(P.w) && y < i32(P.h) && u32(dx * dx + dy * dy) <= r2) {
                    let i = u32(y) * P.w + u32(x);
                    if (agar[i] == 0u) { continue; }
                    if (task == 1u) {
                        sp_in[OWN_SP * c + i] += road;
                        trail_in[i] += road;
                    } else if (task == 2u) {
                        charge[i] += feed;
                    }
                }
            }
        }
    }
}

// The player's walkers a cell into `gcount` (cleared first, `ownclear`):
// living, not a spore or dead, as `Garden::enemy_occ` leaves them out.
@compute @workgroup_size(256)
fn ownclear(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x < P.w * P.h) { atomicStore(&gcount[gid.x], 0u); }
}

@compute @workgroup_size(256)
fn owncount(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n || !lives(k) || species_of(k) != OWN_SP) { return; }
    let st = state(k);
    if (st == L_SPORE || st == L_DEAD) { return; }
    let wk = walkers[k];
    atomicAdd(&gcount[cell(wk.x, wk.y)], 1u);
}
