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
    graze_regrow: f32, boost: f32, _q1: u32, _q2: u32,
};
struct State { t: u32, top: f32, _a: u32, _b: u32 };

@group(0) @binding(0) var<uniform> P: Params;
@group(0) @binding(1) var<storage, read_write> S: State;
@group(0) @binding(2) var<storage, read_write> walkers: array<vec4<f32>>;
@group(0) @binding(3) var<storage, read_write> intent: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read_write> icell: array<u32>;
@group(0) @binding(5) var<storage, read_write> occ: array<atomic<u32>>;
@group(0) @binding(6) var<storage, read_write> claim: array<atomic<u32>>;
@group(0) @binding(7) var<storage, read_write> dep: array<atomic<u32>>;
@group(0) @binding(8) var<storage, read> agar: array<u32>;
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
const TAU: f32 = 6.2831855;

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

fn sense_at(x: f32, y: f32, a: f32, so: f32, sp: u32) -> f32 {
    let c = cos(a); let s = sin(a);
    let i = cell(x + c * so, y + s * so);
    if (i == NONE || agar[i] == 0u) { return -1.0e6; }
    var v = trail_at(i, sp) + scent_at(x + c * P.sso, y + s * P.sso);
    // Grazing: fresh grass smells good (the CPU's `graze_weight`).
    if (graze_w() != 0.0) { v += graze_w() * grass[i]; }
    return v;
}

// Walker k's species: its own (`spec`, which conversion changes), or the
// one plain field.
fn species_of(k: u32) -> u32 {
    if (nsp() == 0u) { return 0u; }
    return spec[k];
}

// Whether walker k is still an explorer: younger than `explore` steps, or
// with `espread` its own share of them, from a draw on k alone (the CPU's
// `explores`).
fn explores(k: u32) -> bool {
    var n = f32(explore());
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

@compute @workgroup_size(256)
fn sense(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let t = S.t;
    let wk = walkers[k];
    var h = wk.z;
    let here = cell(wk.x, wk.y);
    // The CPU's `move_walkers` order: the walker's size, trail-set settings
    // from the trail where it stands, then catching and the sick walker's
    // changes.
    let sp = species_of(k);
    var so = P.so; var sa = P.sa; var ra = P.ra; var st = P.st;
    if (size_sp() > 0.0) {
        let sc = exp2(size_sp() * (2.0 * unit(keyed(P.s_size, 0u, k)) - 1.0));
        so *= sc; st *= sc;
    }
    var ex = false;
    if (explore() > 0u) { ex = explores(k); }
    if (P.ss > 0.0) {
        let v = min(trail_in[here] * P.ss, 1.0);
        so = so + P.om_a * pow0(v, P.om_e);
        sa = sa + P.am_a * pow0(v, P.am_e);
        ra = ra + P.tm_a * pow0(v, P.tm_e);
        st = max(st + P.sm_a * pow0(v, P.sm_e), 0.0);
    }
    if (P.sick_steps > 0u) {
        var c = sick[k];
        if (P.infect != 0u) {
            let dx = wk.x - P.ix; let dy = wk.y - P.iy;
            if (dx * dx + dy * dy <= P.ir * P.ir) { c = P.sick_steps + P.immune; }
        }
        if (c == 0u && germ_in[here] > P.catch_at) { c = P.sick_steps + P.immune; }
        sick[k] = c;
        if (c > P.immune) { st *= P.sick_speed; so *= P.sick_reach; ra *= P.sick_turn; }
    }
    var f = sense_at(wk.x, wk.y, h, so, sp);
    var fl = sense_at(wk.x, wk.y, h - sa, so, sp);
    var fr = sense_at(wk.x, wk.y, h + sa, so, sp);
    // Explorers steer to the lowest reading; a wall stays a wall.
    if (ex) {
        if (f > -1.0e5) { f = -f; }
        if (fl > -1.0e5) { fl = -fl; }
        if (fr > -1.0e5) { fr = -fr; }
    }
    if (P.wander > 0.0 && unit(keyed(P.s_wander, t, k | 0x80000000u)) < P.wander) {
        h = unit(keyed(P.s_wander, t, k | 0x40000000u)) * TAU;
    } else if (P.sfloor > 0.0 && max(max(f, fl), fr) < P.sfloor) {
        if ((keyed(P.s_wander, t, k) & 1u) == 0u) { h += ra; } else { h -= ra; }
    } else if (chance() > 0.0) {
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
    let nx = wk.x + cos(h) * st;
    let ny = wk.y + sin(h) * st;
    let there = cell(nx, ny);
    var aim = NONE;
    if (there != NONE) {
        if (there == here) {
            aim = there;
        } else if (agar[there] != 0u && (P.crowd == 0u || atomicLoad(&occ[there]) < P.crowd)) {
            aim = there;
            if (P.crowd > 0u) { atomicMin(&claim[there], keyed(P.s_claim, t, k)); }
        }
    }
    intent[k] = vec4<f32>(nx, ny, h, 0.0);
    icell[k] = aim;
}

@compute @workgroup_size(256)
fn walk(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let t = S.t;
    let aim = icell[k];
    let wk = walkers[k];
    let here = cell(wk.x, wk.y);
    var wins = false;
    if (aim != NONE) {
        wins = aim == here || P.crowd == 0u || atomicLoad(&claim[aim]) == keyed(P.s_claim, t, k);
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
    let o = species_of(k) * c + at;
    // Grazing: every walker eats where it stands after the move.
    if (graze_w() != 0.0) { atomicAdd(&gcount[at], 1u); }
    if (P.cap > 0u || wins) {
        if (ex) {
            atomicAdd(&edep[o + select(0u, nsp1() * c, ill)], 1u);
        } else if (ill) {
            atomicAdd(&sdep[o], 1u);
        } else {
            atomicAdd(&dep[o], 1u);
        }
    } else if (bshare() > 0.0) {
        atomicAdd(&bdep[o + select(0u, nsp1() * c, ex)], 1u);
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

// Conversion (PR #56), after the move and its deposit, before any jump: a
// walker joins the species with the most trail at its cell (with cyclic
// species, only the previous one, which chases it) if that trail outweighs
// its own `convert` times. The trail is the field after this tick's
// deposit and before the spread, as the CPU reads it. Only dispatched with
// species and `convert` on.
@compute @workgroup_size(256)
fn convert(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let wk = walkers[k];
    let i = cell(wk.x, wk.y);
    let c = P.w * P.h;
    var all = 0.0;
    if (P.cap > 0u) { all = counted(i); }
    let own = spec[k];
    let ov = sp_in[own * c + i] + laid_by(own, i, all);
    var best = own;
    var bv = ov;
    if (P.cycle != 0u && nsp() >= 3u) {
        best = (own + nsp() - 1u) % nsp();
        bv = sp_in[best * c + i] + laid_by(best, i, all);
    } else {
        for (var s = 0u; s < nsp(); s++) {
            if (s != own) {
                let v = sp_in[s * c + i] + laid_by(s, i, all);
                if (v > bv) { best = s; bv = v; }
            }
        }
    }
    if (best != own && bv > P.convert * ov) { spec[k] = best; }
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
    let j = jump_of(k, S.t);
    if (j.w == 0.0) { return; }
    let to = cell(j.x, j.y);
    if (to == NONE || agar[to] == 0u) { return; }
    let wk = walkers[k];
    let here = cell(wk.x, wk.y);
    if (P.crowd == 0u) {
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
    if (atomicLoad(&occ[to]) >= P.crowd) { return; }
    atomicMin(&claim[to], k);
    icell[k] = to;
}

@compute @workgroup_size(256)
fn land(@builtin(global_invocation_id) gid: vec3<u32>) {
    let k = gid.x;
    if (k >= P.n) { return; }
    let to = icell[k];
    if (to == NONE || atomicLoad(&claim[to]) != k) { return; }
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
    let m = nsp1();
    var n = 0.0;
    for (var s = 0u; s < m; s++) {
        let o = s * c + j;
        n += f32(atomicLoad(&dep[o]));
        if (P.sick_steps > 0u) { n += f32(atomicLoad(&sdep[o])); }
        if (explore() > 0u) { n += f32(atomicLoad(&edep[o])) + f32(atomicLoad(&edep[o + m * c])); }
    }
    return n;
}

// The trail species s gets on cell j this tick: `deposit` a walker that
// moved (the sick lay `sick_trail` times as much, explorers `edep` times),
// with `bshare` that share of it a blocked walker, or with `cap` the CPU's
// capped square-root deposit, deposit x sqrt(min(n, cap)) shared by the
// cell's n walkers (`all`, from `counted`). With the outside rules off the
// expressions are the looks build's, so its dishes run as they did.
fn laid(s: u32, j: u32, all: f32) -> f32 {
    let c = P.w * P.h;
    let m = nsp1();
    let o = s * c + j;
    let a = f32(atomicLoad(&dep[o]));
    if (P.cap == 0u && P.sick_steps == 0u && explore() == 0u && bshare() == 0.0) { return P.deposit * a; }
    var b = 0.0;
    if (P.sick_steps > 0u) { b = f32(atomicLoad(&sdep[o])); }
    var w = a + P.sick_trail * b;
    if (explore() > 0u) {
        w += P.edep * (f32(atomicLoad(&edep[o])) + P.sick_trail * f32(atomicLoad(&edep[o + m * c])));
    }
    if (P.cap == 0u) {
        var l = P.deposit * w;
        if (bshare() > 0.0) {
            l += P.deposit * bshare() * (f32(atomicLoad(&bdep[o])) + P.edep * f32(atomicLoad(&bdep[o + m * c])));
        }
        return l;
    }
    if (all == 0.0) { return 0.0; }
    return P.deposit * sqrt(min(all, f32(P.cap))) / all * w;
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
    // The germ spreads as the trail does, with its own keep.
    let germs = P.sick_steps > 0u;
    // A pile is reached once tube lies on it: the CPU marks it at the end
    // of a tick from that tick's trail, which is this tick's `trail_in`, so
    // the scent pass below lays this tick's food as the CPU's next tick does.
    let pi = pile[i];
    if (pi != 0u && trail_in[i] > P.tube) { atomicMax(&reached[pi - 1u], 1u); }
    // Grazing: the walkers on the cell each eat `graze_eat` (floored at 0,
    // one at a time, as the CPU does), then the grass regrows. In place:
    // only the next tick's sense pass reads it.
    if (graze_w() != 0.0 && agar[i] != 0u) {
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
    if (gone) {
        trail_out[i] = 0.0;
        if (germs) { germ_out[i] = 0.0; }
        for (var s = 0u; s < nsp(); s++) { sp_out[s * c + i] = 0.0; }
        return;
    }
    let capped = P.cap > 0u;
    var gsum = 0.0; var n = 0.0;
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
                    n += 1.0;
                }
            }
        }
        trail_out[i] = sum / n * P.keep;
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
                    n += 1.0;
                }
            }
        }
        var total = 0.0;
        for (var s = 0u; s < nsp(); s++) {
            let v = sums[s] / n * P.keep;
            sp_out[s * c + i] = v;
            total += v;
        }
        trail_out[i] = total;
    }
    if (germs) { germ_out[i] = gsum / n * P.sick_keep; }
}

@compute @workgroup_size(256)
fn scent(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= P.w * P.h) { return; }
    atomicStore(&claim[i], NONE);
    if (P.sick_steps > 0u) { atomicStore(&gdep[i], 0u); }
    if (graze_w() != 0.0) { atomicStore(&gcount[i], 0u); }
    // The counts, a block a species (and a class).
    let c = P.w * P.h;
    let m = nsp1();
    for (var s = 0u; s < m; s++) {
        let o = s * c + i;
        atomicStore(&dep[o], 0u);
        if (P.sick_steps > 0u) { atomicStore(&sdep[o], 0u); }
        if (explore() > 0u) { atomicStore(&edep[o], 0u); atomicStore(&edep[o + m * c], 0u); }
        if (bshare() > 0.0) { atomicStore(&bdep[o], 0u); atomicStore(&bdep[o + m * c], 0u); }
    }
    if (i == 0u) { S.t = S.t + 1u; }
    let x = i % P.w; let y = i / P.w;
    if (agar[i] == 0u || x == 0u || y == 0u || x + 1u == P.w || y + 1u == P.h) { scent_out[i] = 0.0; return; }
    var sum = 0.0; var n = 0.0;
    for (var dy = 0u; dy < 3u; dy++) {
        for (var dx = 0u; dx < 3u; dx++) {
            let j = (y + dy - 1u) * P.w + x + dx - 1u;
            if (agar[j] != 0u) {
                // A reached pile lays `fed` of its scent (the CPU's `fed_scent`).
                var l = lay[j];
                if (l != 0.0 && pile[j] != 0u && atomicLoad(&reached[pile[j] - 1u]) != 0u) { l *= P.fed; }
                sum += scent_in[j] + l;
                n += 1.0;
            }
        }
    }
    scent_out[i] = sum / n * P.scent_keep;
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
        if (P.colour == 0u) { c = vec3<f32>(8.0, 14.0, 18.0); }
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
        let base = vec3<f32>(12.0 + 30.0 * sg, 22.0 + 26.0 * sg, 26.0 + 10.0 * sg);
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
    // The panel, premultiplied, over the dish.
    let o = hud[i];
    let a = f32(o >> 24u) / 255.0;
    let hc = vec3<f32>(f32(o & 255u), f32((o >> 8u) & 255u), f32((o >> 16u) & 255u));
    c = hc + c * (1.0 - a);
    let q = vec3<u32>(clamp(c, vec3<f32>(0.0), vec3<f32>(255.0)));
    frame[i] = q.x | (q.y << 8u) | (q.z << 16u) | (255u << 24u);
}
