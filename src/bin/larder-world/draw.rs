//! What the window draws from the game: marks, the HUD's model, the
//! ground, the end and mission screens. Moved out of `larder-world.rs`
//! unchanged (C9, 2026-11-14).

use super::*;
use slime_mould::world::ui::Ending;

/// The look's ground field from the game's world: rock, wet and agar from
/// the engine's CPU mirror; crystals from its own cells (`pile_of`, so
/// each map crystal's size and shape is the one the mould eats); with
/// `charge`, each crystal cell's share of its full charge; with `health`,
/// each wall cell's share of its full health, and a wall the GPU has worn
/// through drawn as rubble (the mirror keeps it a wall); and the map's
/// soil, which the engine plays since 2026-11-12 (WM3: sand thins the
/// trail, clay keeps it, barren ground starves and takes no births), so
/// the look draws it (#84). `depth` is the crystals' depth
/// (`App::depth`), worked out once a world.
pub(super) fn ground_codes(g: &Game<GpuWorld>, depth: &[u8], charge: Option<&[f32]>, health: Option<&[f32]>) -> Vec<u32> {
    let d = &g.world.e.dish;
    let cells = d.w * d.h;
    let pile = |i: usize| d.pile_of.get(i).copied().unwrap_or(0) as usize;
    let charge_share: Vec<f32> = match charge {
        Some(c) => (0..cells).map(|i| if pile(i) > 0 { c[i] / d.pile_fill.get(pile(i) - 1).copied().unwrap_or(d.p.crystal_charge).max(1e-6) } else { 0.0 }).collect(),
        None => Vec::new(),
    };
    let broken = |i: usize| health.is_some_and(|h| d.wall.get(i).copied().unwrap_or(false) && h[i] <= 0.0);
    let wall_share: Vec<f32> = match health {
        Some(h) => (0..cells).map(|i| if d.wall.get(i).copied().unwrap_or(false) { let full = d.wall_health.get(i).copied().unwrap_or(f32::INFINITY); if full.is_finite() && full > 0.0 { h[i] / full } else { 1.0 } } else { 0.0 }).collect(),
        None => Vec::new(),
    };
    let wall: Vec<bool> = (0..cells).map(|i| d.wall.get(i).copied().unwrap_or(false) && !broken(i)).collect();
    let agar: Vec<bool> = (0..cells).map(|i| d.agar[i] || broken(i)).collect();
    let mut codes = look::ground(d.w, cells, &Fields { agar: &agar, rock: &d.rock, wet: &d.wet, wall: &wall, wall_share: &wall_share, crystal: &d.pile_of, charge_share: &charge_share, elev: &[], soil: &d.soil, depth });
    for (i, c) in codes.iter_mut().enumerate() {
        if broken(i) {
            *c = look::code(Ground::Rubble, 0.0);
        } else if wall[i] && g.doors.get(i).copied().unwrap_or(false) {
            // A door's cell: a wall's code, health and all, of its own kind.
            *c = (*c & !15) | Ground::Door as u32;
        }
    }
    look::mark_far(d.w, &mut codes);
    codes
}

/// The side of the tiles a new wall's cells are grouped in, to re-mark
/// the ground's `look::FAR` bit round them (`refresh_ground`).
const WALL_TILE: usize = 32;

/// The cells a timed ground refresh can change (`App::refresh_ground`):
/// the crystals' (their charge) and the walls' (their health), row-major,
/// gathered on the GPU and read back alone (`gpu::Gather`). Everything
/// else the ground draws stays put between walls going up.
#[derive(Default)]
pub(super) struct Live {
    pub cells: Vec<u32>,
    pub gather: Option<slime_mould::gpu::Gather>,
}

pub(super) fn live_cells(g: &Game<GpuWorld>, old: &Live) -> Live {
    let e = &g.world.e;
    let d = &e.dish;
    let cells: Vec<u32> = (0..d.w * d.h).filter(|&i| d.wall.get(i).copied().unwrap_or(false) || d.pile_of.get(i).copied().unwrap_or(0) > 0).map(|i| i as u32).collect();
    let gather = slime_mould::gpu::Gather::new(&e.device, &e.queue, &cells, old.gather.as_ref());
    Live { cells, gather }
}

/// A crystal or wall cell's code as `ground_codes` makes it from `charge`
/// and `health` (None: no walls), keeping `old`'s `look::FAR` bit: a
/// crystal is never hard terrain and a wall stays hard as it wears to
/// rubble, so no cell's neighbourhood changes.
pub(super) fn live_code(g: &Game<GpuWorld>, depth: &[u8], i: usize, old: u32, charge: f32, health: Option<f32>) -> u32 {
    let d = &g.world.e.dish;
    let is_wall = d.wall.get(i).copied().unwrap_or(false);
    if is_wall && health.is_some_and(|h| h <= 0.0) {
        return look::code(Ground::Rubble, 0.0) | (old & look::FAR);
    }
    let base = if is_wall {
        let share = match health {
            Some(h) => {
                let full = d.wall_health.get(i).copied().unwrap_or(f32::INFINITY);
                if full.is_finite() && full > 0.0 {
                    h / full
                } else {
                    1.0
                }
            }
            None => 1.0,
        };
        // A door's cells (`Game::doors`) wear as a wall's; their own kind
        // draws them as a gate. Re-coded here once a door is cut into a
        // standing wall too, since the timed pass runs when walls change.
        look::code(if g.doors.get(i).copied().unwrap_or(false) { Ground::Door } else { Ground::Wall }, share)
    } else {
        let p = d.pile_of.get(i).copied().unwrap_or(0) as usize;
        look::code(Ground::Crystal, charge / d.pile_fill.get(p - 1).copied().unwrap_or(d.p.crystal_charge).max(1e-6))
    };
    base | (depth.get(i).copied().unwrap_or(0) as u32) << 16 | ((d.soil.get(i).copied().unwrap_or(0) & 7) as u32) << 4 | (old & look::FAR)
}

/// A round's end, not a mission's (the sandbox): what ended it in the
/// title, the round's numbers, and the score. `place` is the map.
pub(super) fn end_screen(end: End, g: &Game<GpuWorld>, t: f32, place: String) -> Ending {
    let (title, won, at) = match end {
        End::MouldWiped(t) => ("The mould is wiped out", Some(true), t),
        End::SquadWiped(t) => ("The squad is lost", Some(false), t),
        End::Overrun(t) => ("The mould overran the larder", Some(false), t),
        End::Cap(s) if s > 0.5 => ("Time: you took more of the crystals", Some(true), t),
        End::Cap(s) if s < 0.5 => ("Time: the mould took more of the crystals", Some(false), t),
        End::Cap(_) => ("Time: an even race", None, t),
    };
    let mut lines = Vec::new();
    if let End::Cap(s) = end {
        lines.push(format!("You took {:.0}% of the charge gone from the crystals", s * 100.0));
    }
    lines.push(format!("Score {:+.2} (a win is over 1, a loss under -1)", g.score().unwrap_or(0.0)));
    Ending { place, title: title.into(), won, stars: None, says: String::new(), tiles: end_tiles(g, at), lines }
}

/// A mission's end: the cause in the title (stars when won), the round's
/// numbers, and how it ended. A run's planet adds its own (`App::model`).
pub(super) fn mission_screen(tr: &Tracker, g: &Game<GpuWorld>) -> Ending {
    let v = tr.verdict.expect("a verdict");
    use slime_mould::mould::campaign::Cause;
    let title = if v.cause == Cause::Cap {
        // A stalemate's limit (`run::limit`), or a mission's cap: the slime
        // was not beaten, whatever the star (balance review, 11-20: "Mission
        // won" over a planet still holding 22,090 walkers).
        "The slime held on".to_string()
    } else if v.stars > 0 {
        "Mission won".to_string()
    } else {
        match v.cause {
            Cause::SquadWiped => "The squad is lost",
            Cause::Overrun => "The slime took the larder",
            Cause::Stuck => "Out of food",
            Cause::MouldWiped => "The slime is gone, the goal missed",
            Cause::Cap | Cause::Goal => "Out of time",
        }
        .to_string()
    };
    Ending { place: tr.mission.name.clone(), title, won: Some(v.stars > 0), stars: Some(v.stars), says: String::new(), tiles: end_tiles(g, v.at), lines: vec![tr.cause_line()] }
}

/// The end screen's numbers (Look and interface, 2026-11-18): how long
/// the round ran, the walkers killed, the shots and kills a shot, the
/// soldiers standing and lost, the food harvested, and the slime left
/// against its peak. Before they were one grey line, cut off at 100%.
fn end_tiles(g: &Game<GpuWorld>, at: f32) -> Vec<(String, String)> {
    let lost = g.soldiers.iter().filter(|s| !s.alive).count();
    let shots = if g.shots > 0 { format!("shots, {:.1} kills each", g.killed as f32 / g.shots as f32) } else { "shots".into() };
    vec![
        (format!("{}:{:02}", at as u32 / 60, at as u32 % 60), "played".into()),
        (grouped(g.killed as usize), "walkers killed".into()),
        (grouped(g.shots as usize), shots),
        (format!("{}", g.alive()), if lost == 0 { "soldiers standing, none lost".into() } else { format!("soldiers standing, {lost} lost") }),
        (grouped(g.harvested.round() as usize), "food harvested".into()),
        (format!("{} of {}", walkers(g.mould), walkers(g.peak.max(g.mould))), "slime left, of its peak".into()),
    ]
}

/// Walkers as the top bar counts them from 10,000 ("42.3k"), else whole
/// ("1,558").
fn walkers(n: usize) -> String {
    if n >= 10_000 {
        thousands(n)
    } else {
        grouped(n)
    }
}

/// A count with its thousands set off ("12,630").
fn grouped(n: usize) -> String {
    let d = n.to_string();
    let mut out = String::new();
    for (i, ch) in d.chars().enumerate() {
        if i > 0 && (d.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(ch);
    }
    out
}

pub(super) fn summary(g: &Game<GpuWorld>) -> String {
    format!("{} soldiers left, {:.0} food harvested, {} walkers killed with {} shots, {} walls", g.alive(), g.harvested, thousands(g.killed as usize), g.shots, g.walls)
}

/// A turret's lamp from the game's state: dark while rising or with no
/// mould in reach, red when the bank can't pay for a shot (`Rules::price`;
/// shots are free by default, so a turret is never dry then), amber when
/// it has mould to shoot at (`Building::aim`, as `Game` fires).
pub(super) fn turret_state<W: World>(g: &Game<W>, b: &Building) -> u32 {
    if !b.up() {
        TURRET_IDLE
    } else if g.bank < g.rules.price {
        TURRET_DRY
    } else if b.aim.is_some() && b.dense >= g.rules.aim_min {
        TURRET_TRACKING
    } else {
        TURRET_IDLE
    }
}

/// An order line from `from` through each order's point: the first as a
/// line, each queued one (Shift) a fainter line ending in a small ring, so a
/// queue of moves reads as the whole path. Red for fighting (attack-move,
/// the core), white else. `homing`: the first order is a harvest carrying
/// home, so it heads for the larder. A first order closer than a dish
/// cell (a squad settling where it was let go) draws nothing.
fn order_chain(marks: &mut Marks, g: &Game<GpuWorld>, from: (f32, f32), orders: impl Iterator<Item = (Order, bool)>, homing: bool, sc: f32) {
    let mut from = from;
    for (n, (o, queued)) in orders.enumerate() {
        let to = match o {
            Order::Move(x, y) | Order::Fight(x, y) => Some((x, y)),
            Order::Clear(..) if n == 0 && homing => Some(g.larder),
            Order::Clear(x, y) => Some((x, y)),
            Order::Attack => Some(g.core),
            Order::Burn(a, b) => Some(((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0)),
            Order::Guard(x, y) => Some((x, y)),
            Order::Build(b) => g.buildings.get(b).map(|b| b.at),
            Order::Hold => None,
        };
        let Some(to) = to else { continue };
        if n == 0 && !queued && (to.0 - from.0).hypot(to.1 - from.1) < sc {
            continue;
        }
        let later = n > 0 || queued;
        let a = if later { 150 } else { 210 };
        let colour = if matches!(o, Order::Attack | Order::Fight(..)) { [255, 110, 110, a] } else { [255, 255, 255, a - 20] };
        marks.line(from, to, 2.0, colour);
        marks.ring(to, if later { 1.0 } else { 1.4 } * sc, colour);
        from = to;
    }
}

/// Turret `k` (`Marks::turret`), drawn `half` world units, and the light
/// of its shot at the muzzle, as a soldier's lights the ground.
fn turret_mark(marks: &mut Marks, lights: &mut Vec<Light>, looks: &[BuildingLook], g: &Game<GpuWorld>, k: usize, half: f32) {
    let b = &g.buildings[k];
    let look = looks.get(k).copied().unwrap_or_default();
    let built = if b.up() { 1.0 } else { b.built };
    let fl = look.shot / TURRET_SHOT;
    let m = (b.at.0 + look.facing.cos() * half * 1.8, b.at.1 + look.facing.sin() * half * 1.8);
    // Its muzzle flash, under the head so the barrels cover its root (gun
    // look, 2026-11-15), as a soldier's: a rifle's jets in a plain shot's
    // gold, for the first `FLASH` of the kick.
    let jf = ((look.shot - (TURRET_SHOT - FLASH)) / FLASH).clamp(0.0, 1.0);
    if jf > 0.0 {
        marks.muzzle_flash(m, half * 0.9, look.facing, 0, k as u32 * 13, jf, flame(&card_look(None)), MUZZLE_CORE);
    }
    if b.kind == Kind::Mortar {
        marks.mortar(b.at, half, built, b.hp / b.max_hp.max(1.0), look.facing, fl, turret_state(g, b));
    } else {
        marks.turret(b.at, half, built, b.hp / b.max_hp.max(1.0), look.facing, fl, turret_state(g, b));
    }
    if fl > 0.0 {
        let sc = g.world.scale();
        lights.push(Light { at: m, reach: MUZZLE_LIGHT * sc, lift: 3.0, colour: [255.0, 220.0, 170.0], strength: 0.6 * fl });
    }
}

/// A building's mark colour; None for walls, which the world draws.
pub(super) fn building_colour(k: Kind) -> Option<[u8; 4]> {
    match k {
        Kind::Turret => Some([150, 200, 255, 255]),
        Kind::Mortar => Some([190, 175, 255, 255]),
        Kind::Lure => Some([255, 196, 90, 255]),
        Kind::Pump => Some([90, 220, 230, 255]),
        Kind::Outpost => Some([214, 196, 150, 255]),
        Kind::Siren => Some([255, 120, 120, 255]),
        Kind::Wall { .. } => None,
    }
}

pub(super) fn crystal_name(i: usize) -> String {
    if i == 0 {
        "The mould's home crystal".into()
    } else {
        format!("Crystal {i}")
    }
}

pub(super) fn stance_name(s: Stance) -> &'static str {
    match s {
        Stance::HoldFire => "hold fire",
        Stance::Defend => "defend",
        Stance::Free => "free fire",
    }
}

/// A round's map for the window's title and opening alert: a generated
/// planet by its own name ("Vesk, a fen planet (planet 3)"), any other
/// map as `title_case` says it.
pub(super) fn map_title(name: &str, seed: u64) -> String {
    match slime_mould::mould::levelgen::spec_planet(name, seed) {
        Some(Ok(p)) => format!("{}, a {} planet (planet {})", p.name, p.biome.name(), p.depth),
        _ => title_case(name),
    }
}

/// A map's name as part of a file name: no folders, and no colons
/// (`planet:3:42`), which Windows refuses.
pub(super) fn file_part(name: &str) -> String {
    name.replace(['/', '\\'], "_").replace(':', "-")
}

pub(super) fn title_case(s: &str) -> String {
    let s = s.rsplit('/').next().unwrap_or(s);
    let mut c = s.chars();
    match c.next() {
        Some(f) => format!("The {}{}", f.to_uppercase(), c.as_str()),
        None => String::new(),
    }
}

/// "trap" as "Trap".
pub(super) fn capital(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or(String::new(), |f| f.to_uppercase().chain(c).collect())
}

pub(super) fn thousands(n: usize) -> String {
    if n >= 10_000 {
        format!("{:.1}k", n as f32 / 1000.0)
    } else {
        format!("{n}")
    }
}

impl App {
    /// The look's ground from the world's cells: once a game second (the
    /// crystals drain, the walls wear), and at once when a wall goes up.
    pub(super) fn refresh_ground(&mut self) {
        let (Some(g), Some(buf)) = (&self.game, &self.ground) else { return };
        let t = g.time();
        // A step still running would make the read-back below wait for it
        // (on llvmpipe the split's prepare p95 was 14.9 ms against 1.3 ms
        // for whole steps, 2026-11-12), so the timed refresh waits for a
        // frame between steps, up to a second late; a new wall reads at once.
        let late = t - self.ground_at;
        if (late < GROUND_EVERY || (g.in_flight() && late < 2.0 * GROUND_EVERY)) && g.walls == self.ground_walls {
            return;
        }
        let e = &g.world.e;
        let t0 = Instant::now();
        let ms = |t: Instant| t.elapsed().as_secs_f64() * 1e3;
        let floats = |b: Vec<u8>| -> Vec<f32> { b.chunks_exact(4).map(|c| f32::from_le_bytes(c.try_into().unwrap())).collect() };
        let d = &e.dish;
        let any_wall = d.wall.iter().any(|&w| w);
        let cells = d.w * d.h;
        let is_wall = |i: usize| d.wall.get(i).copied().unwrap_or(false);
        let read;
        if self.codes.len() != cells || self.wall_mask.len() != cells {
            // A new world: every cell.
            let charge = floats(read_back(&e.device, &e.queue, &e.charge));
            let health = any_wall.then(|| floats(read_back(&e.device, &e.queue, &e.health)));
            read = ms(t0);
            self.codes = ground_codes(g, &self.depth, Some(&charge), health.as_deref());
            e.queue.write_buffer(buf, 0, &self.codes.iter().flat_map(|c| c.to_le_bytes()).collect::<Vec<u8>>());
            self.live = live_cells(g, &self.live);
            self.wall_mask = (0..cells).map(is_wall).collect();
        } else {
            // A wall went up (technical lead, 2026-11-14: rebuilding every
            // cell for it was the 25-32 ms hitch at 6x a planet's area, and
            // the AI and the player both raise walls in play): its cells
            // join the gathered ones, and the 32-cell tiles it touches are
            // re-marked (`mark_far_box`), each grown by the mark's reach.
            let mut tiles: Vec<(usize, usize)> = Vec::new();
            if g.walls != self.ground_walls {
                let changed: Vec<usize> = (0..cells).filter(|&i| is_wall(i) != self.wall_mask[i]).collect();
                if !changed.is_empty() {
                    for &i in &changed {
                        self.wall_mask[i] = is_wall(i);
                    }
                    self.live = live_cells(g, &self.live);
                    tiles = changed.iter().map(|&i| ((i % d.w) / WALL_TILE, (i / d.w) / WALL_TILE)).collect();
                    tiles.sort_unstable();
                    tiles.dedup();
                    // A cell that stopped being a wall (the CPU dish's
                    // rubble; the GPU's walls wear to rubble but stay
                    // walls) and holds no crystal: its code from the dish.
                    let f = look::Fields { agar: &d.agar, rock: &d.rock, wet: &d.wet, wall: &d.wall, crystal: &d.pile_of, soil: &d.soil, ..Default::default() };
                    for &i in changed.iter().filter(|&&i| !is_wall(i) && d.pile_of.get(i).copied().unwrap_or(0) == 0) {
                        self.codes[i] = look::cell(&f, i, self.depth.get(i).copied().unwrap_or(0));
                    }
                }
            }
            // Once a game second: only the crystal and wall cells, read
            // back alone, and only the runs where a code changed written
            // (technical lead, 2026-11-14: the whole field read back and
            // rebuilt took 4.6 ms at today's area and 32.9 ms at 6x on
            // the 1070, a dropped frame a second from 4x).
            let (charge, health) = self.live.gather.as_ref().map_or((Vec::new(), Vec::new()), |q| q.read(&e.device, &e.queue, &e.charge, &e.health));
            let (charge, health) = (floats(charge), any_wall.then(|| floats(health)));
            read = ms(t0);
            let write = |codes: &[u32], a: usize, b: usize| e.queue.write_buffer(buf, a as u64 * 4, &codes[a..b].iter().flat_map(|c| c.to_le_bytes()).collect::<Vec<u8>>());
            // Rows of changed cells written as runs: a crystal eaten
            // changes a few rows of it, not the buffer.
            let mut run: Option<(usize, usize)> = None;
            for (k, &i) in self.live.cells.iter().enumerate() {
                let i = i as usize;
                let c = live_code(g, &self.depth, i, self.codes[i], charge[k], health.as_ref().map(|h| h[k]));
                if c == self.codes[i] {
                    continue;
                }
                self.codes[i] = c;
                run = match run {
                    Some((a, b)) if b == i => Some((a, i + 1)),
                    _ => {
                        if let Some((a, b)) = run {
                            write(&self.codes, a, b);
                        }
                        Some((i, i + 1))
                    }
                };
            }
            if let Some((a, b)) = run {
                write(&self.codes, a, b);
            }
            // The new wall's tiles, marked again and written row by row.
            let r = look::FAR_REACH;
            for &(tx, ty) in &tiles {
                let xs = ((tx * WALL_TILE).saturating_sub(r), ((tx + 1) * WALL_TILE + r).min(d.w));
                let ys = ((ty * WALL_TILE).saturating_sub(r), ((ty + 1) * WALL_TILE + r).min(d.h));
                look::mark_far_box(d.w, &mut self.codes, xs, ys);
                for y in ys.0..ys.1 {
                    write(&self.codes, y * d.w + xs.0, y * d.w + xs.1);
                }
            }
        }
        (self.ground_at, self.ground_walls) = (t, g.walls);
        let (n, sum, most, most_read) = self.ground_cost;
        let all = ms(t0);
        self.ground_cost = (n + 1, sum + all, most.max(all), most_read.max(read));
    }

    pub(super) fn marks(&mut self) {
        self.marks.clear();
        let Some(g) = &self.game else { return };
        let sc = g.world.scale();
        let zoom = self.camera.zoom;
        // The squad's look and its barrels' mouths, by the run's Armour.
        let skin = armour_skin(self.run.as_ref().map_or(0, |r| r.level_of("hp")));
        let mouth = MUZZLE[skin as usize];
        // The larder (the squad's base: bought soldiers stand there and
        // carried food is banked there) and the core's ring (stand in it
        // with no mould near for 5 s to take it). Game objects, not the
        // simulation, so marks.
        if g.rules.larder {
        // The larder is a sprite in soldier A's style (art review
        // 2026-11-14: it was a plain beige square), drawn from the game's
        // state: its charge cells lit while the bank holds food, darkening
        // as the mould holds it with the bank dry (the overrun count). The
        // faint ring is the ground a walker must reach to hold it. Drawn
        // 2.2 dish cells (was 1.6) since the buildings went soldier-scale
        // (11-14), so the store stays bigger than an outpost.
        self.marks.building_pulse(g.larder, 2.2 * sc, LARDER_SPRITE, 1.0, 1.0 - g.overrun_share(), g.bank >= 1.0, self.banked / BANKED);
        self.marks.ring(g.larder, 3.0 * sc, [214, 196, 150, 90]);
        } else {
            // No larder (the squad mode): each guarded crystal's ring, the
            // game's count of who holds it (`Game::guarded`).
            for &i in &g.guarded {
                let c = g.world.crystals()[i].0;
                self.marks.ring(c, g.world.crystal_r(i) + 2.0 * sc, [94, 214, 132, 170]);
            }
        }
        if g.rules.squads {
            // Each squad's number over its middle (the key that controls
            // it: its id + 1, kept while it lives), the controlled one bright,
            // the rest "auto": they run on their own orders (owner
            // 2026-11-15 02:58Z: "easier to confirm that the squad is now
            // running on its own"). A squad linked with the controlled one
            // is commanded with it, so it is not auto.
            for q in g.live_squads() {
                let n = q;
                if let Some(m) = g.middle(q) {
                    let lit = g.active == Some(q);
                    let colour = if lit { [255, 255, 255, 255] } else { [200, 210, 220, 170] };
                    let label = format!("{}{}", n + 1, if lit { " *" } else if g.leads(q) { "" } else { " auto" });
                    // Above its topmost soldier, so the soldiers' sprites
                    // don't cover the number (the longer "N auto" label of
                    // 11-15 showed only its tail over a spread squad). Over
                    // the soldiers round the one nearest its middle (play-zoom
                    // pass, 11-20): a squad in two groups put its number at
                    // the middle's x and the far group's top, over empty
                    // ground or water between them.
                    let members = g.members(q);
                    let d = |k: usize| (g.soldiers[k].x - m.0).hypot(g.soldiers[k].y - m.1);
                    let core = members.iter().copied().min_by(|&a, &b| d(a).total_cmp(&d(b)));
                    // Its group: the soldiers linked to that one through
                    // neighbours at most `LABEL_LINK` cells apart. The number
                    // goes over that soldier, above the group's soldiers in
                    // its column (`LABEL_COLUMN`), so a squad strung out on a
                    // slant has it over its middle, not over empty ground
                    // level with its top end.
                    let (x, top) = core.map_or((m.0, m.1), |c| {
                        let at: Vec<(f32, f32)> = members.iter().map(|&k| (g.soldiers[k].x, g.soldiers[k].y)).collect();
                        let first = members.iter().position(|&k| k == c).unwrap_or(0);
                        let x = at[first].0;
                        let mut seen = vec![false; at.len()];
                        let mut todo = vec![first];
                        seen[first] = true;
                        let mut top = at[first].1;
                        while let Some(i) = todo.pop() {
                            if (at[i].0 - x).abs() <= LABEL_COLUMN * sc {
                                top = top.min(at[i].1);
                            }
                            for j in 0..at.len() {
                                if !seen[j] && (at[j].0 - at[i].0).hypot(at[j].1 - at[i].1) <= LABEL_LINK * sc {
                                    seen[j] = true;
                                    todo.push(j);
                                }
                            }
                        }
                        (x, top)
                    });
                    self.marks.label((x, top), (-4.0, -(3.0 * sc * zoom).clamp(24.0, 48.0)), &label, if lit { 2.0 } else { 1.5 }, colour);
                }
            }
        }
        if self.larder_sel {
            self.marks.selected(g.larder, 3.6 * sc * zoom);
        }
        // The building selected (option b), ringed as a soldier is.
        if let Some(bd) = self.building_sel.and_then(|b| g.buildings.get(b)).filter(|bd| bd.alive && self.controls.selected.is_empty()) {
            let at = match bd.kind {
                Kind::Wall { to } => ((bd.at.0 + to.0) / 2.0, (bd.at.1 + to.1) / 2.0),
                _ => bd.at,
            };
            self.marks.selected(at, (g.rules.build.touch + 1.0) * sc * zoom);
        }
        // What is guarded (`Order::Guard`, `Squad::guard`): the zone its
        // guards keep, a ring in the order line's white, so the player sees
        // what the order covers.
        let mut zones: Vec<((f32, f32), f32)> = g.soldiers.iter().filter_map(|s| match s.order {
            Order::Guard(x, y) if s.alive => Some(g.zone_at((x, y))),
            _ => None,
        }).collect();
        if g.rules.squads {
            zones.extend(g.live_squads().into_iter().filter_map(|q| g.squads[q].guard.map(|p| g.zone_at(p))));
        }
        zones.dedup();
        for (p, r) in zones {
            self.marks.ring(p, r, [200, 230, 255, 150]);
        }
        if let Order::Move(x, y) | Order::Clear(x, y) = g.rally {
            if self.larder_sel || !self.controls.selected.is_empty() {
                self.marks.line(g.larder, (x, y), 2.0, [214, 196, 150, 110]);
                self.marks.ring((x, y), 1.2 * sc, [214, 196, 150, 160]);
            }
        }
        // The core ring marks a home that can be taken, so a headless mould
        // (the default since 11-12 20:21Z: no king) shows none.
        if g.core_rule.kind != CoreKind::Headless {
            let core = if g.core_down.is_some() { [94, 214, 132, 200] } else { [242, 98, 98, 150] };
            self.marks.ring(g.core, g.rules.core_r * sc, core);
        }
        // Buildings: sprites in soldier A's style (`Marks::building`,
        // src/world/sprites/buildings.svg), drawn from the game's state:
        // rising as they are built, darkening as they are hurt, the lamp
        // lit while working. Their effects (a lure's scent, a pump's
        // dimming crystal, a turret's hits) are the simulation's own fields;
        // walls are ground. The bars stay as UI.
        // A turret's reach (art director, 11-14): a faint dashed ring round
        // the one under the cursor, and round the spot while placing one.
        // Buildings can't be selected, so hovering stands in for that. A
        // dashed ring of lines claims only its own pixels (one big ring
        // mark walks its whole box: 0.7 ms at 480 pixels).
        let cursor = self.controls.cursor.filter(|&p| !self.hud.over(p));
        // A mortar's two rings: its reach and its blind ring inside.
        let bld = g.rules.build;
        let rings = |k: Kind| match k {
            Kind::Mortar => vec![bld.mortar_range, bld.mortar_min],
            _ => vec![bld.turret_range],
        };
        let placing = self.hud.targeting().and_then(|t| build_kind(t.0).filter(|k| matches!(k, Kind::Turret | Kind::Mortar))).zip(cursor.and_then(|p| self.hud.aim(p, &self.camera))).map(|(k, at)| (at, k));
        let over = cursor.and_then(|p| self.camera.world_at(p)).and_then(|w| {
            g.buildings.iter().find(|b| b.alive && matches!(b.kind, Kind::Turret | Kind::Mortar) && (b.at.0 - w.0).hypot(b.at.1 - w.1) < g.rules.build.touch * sc * 1.3).map(|b| (b.at, b.kind))
        });
        for (at, reach) in placing.into_iter().chain(over).flat_map(|(at, k)| rings(k).into_iter().map(move |r| (at, r))) {
            let reach = reach * sc;
            let n = if reach > 60.0 { 180 } else { 72 };
            for i in 0..n {
                let (a0, a1) = (i as f32 / n as f32 * std::f32::consts::TAU, (i as f32 + 0.5) / n as f32 * std::f32::consts::TAU);
                self.marks.line((at.0 + reach * a0.cos(), at.1 + reach * a0.sin()), (at.0 + reach * a1.cos(), at.1 + reach * a1.sin()), 0.0, [240, 232, 200, 110]);
            }
        }
        let mut turrets = Vec::new();
        // The buildings' lights, lit after the frame's lights are cleared.
        let mut glows: Vec<Light> = Vec::new();
        for (k, b) in g.buildings.iter().enumerate() {
            if !b.alive {
                continue;
            }
            // A ghost's progress (owner's playtest 2026-11-12: "a faint
            // building with a progress bar that fills in as it builds so I
            // can tell how long until it is complete"): the bar is
            // `built`, and beside it the seconds left at the rate its
            // builders raise it now (`Game::raise_left`), "on the way"
            // while they walk, "no builder" when nobody is sent.
            // The bar spans the building on screen (10-40 pixels each side).
            let wide = (g.rules.build.touch * sc * zoom).clamp(10.0, 40.0);
            let progress = |marks: &mut Marks, at: (f32, f32), below: f32| {
                marks.bar(at, below, (wide, 2.0), b.built, [235, 235, 235, 255], [40, 40, 40, 220]);
                // The seconds left as text; "on the way" and "no builder"
                // as a figure striding, or standing struck through (art
                // director, 11-14: two ghosts side by side ran their words
                // into each other).
                let (x, y) = (wide + 4.0, below - 5.0);
                let ink = [235, 235, 235, 230];
                match (g.raise_left(k), g.raising(k)) {
                    (Some(t), _) => marks.label(at, (x, y), &format!("{}s", t.ceil() as u32), 1.5, ink),
                    (None, (_, n)) if n > 0 => marks.icon(at, (x, y), WALKING, 1.5, ink),
                    _ => {
                        marks.icon(at, (x, y), STANDING, 1.5, [200, 200, 200, 200]);
                        marks.icon(at, (x, y), SLASH, 1.5, [235, 80, 70, 255]);
                    }
                }
            };
            if let Kind::Wall { to } = b.kind {
                // A wall is ground once whole; until then a faint dashed
                // line where it will stand, with its bar at the middle.
                if !b.up() {
                    self.marks.line(b.at, to, 3.0, [235, 220, 180, 170]);
                    progress(&mut self.marks, ((b.at.0 + to.0) / 2.0, (b.at.1 + to.1) / 2.0), 6.0);
                }
                continue;
            }
            if building_colour(b.kind).is_none() {
                continue;
            }
            let Some(kind) = sprite_kind(b.kind) else { continue };
            // Every building soldier-scale (building looks, 11-14; they were
            // 0.7 touch radii, smaller than a soldier's 1.15, and the turret
            // went to a whole one first).
            let half = (g.rules.build.touch * sc).max(11.0 / zoom);
            // A lure switched off is not working.
            let working = b.working && (!matches!(b.kind, Kind::Lure | Kind::Siren) || b.on);
            let built = if b.up() { 1.0 } else { b.built };
            if matches!(b.kind, Kind::Turret | Kind::Mortar) {
                // A gun that reads at zoom 6 as clearly as a soldier (owner's
                // playtest 11-14 08:15Z: "turrets need to look better"; it
                // was drawn 0.7 touch radii, smaller than a soldier's 1.15):
                // its head turned to its aim, kicking as it fires, its lamp
                // the game's state (mould in reach, or a bank too low to pay
                // for a shot).
                // A standing one is drawn after the bullets, with its bar,
                // below; a ghost here, under its progress bar.
                if b.up() {
                    turrets.push((k, half));
                    continue;
                }
                turret_mark(&mut self.marks, &mut self.lights, &self.blooks, g, k, half);
            } else {
                let n = self.blooks.get(k).copied().unwrap_or_default();
                let health = b.hp / b.max_hp.max(1.0);
                match b.kind {
                    // A burning lure's lamp, and its scent lit where the
                    // engine's scent field really is (a scent light, lit
                    // by the look's scent byte): it spreads while the lure
                    // burns and fades with the field once it stops.
                    Kind::Lure => {
                        self.marks.building(b.at, half, kind, built, health, working);
                        if working && b.up() {
                            glows.push(Light { at: b.at, reach: g.rules.build.lure_r * sc * 1.6, lift: -2.0, colour: [150.0, 235.0, 80.0], strength: 1.3 });
                        }
                    }
                    // A rig built round its crystal (owner, 2026-11-15: "it
                    // should probably just change the whole appearance of
                    // the crystal like you've built some sort of structure
                    // around it"), the pump's head on the crystal's middle,
                    // lit by its draw this frame (`Building::drew`) of its
                    // full draw (`Game::pump_draw`): a crystal running dry
                    // dims it. Until 11-15 the pump stood by the crystal
                    // with its hose turned in and a drawn line of charge.
                    Kind::Pump => {
                        let frame = g.rules.frame_ticks as f32 / g.rules.tps as f32;
                        let full = if b.up() { (b.drew / (g.pump_draw(k) * g.pump_yield(k) * frame).max(1e-6)).clamp(0.0, 1.0) } else { 0.0 };
                        let c = b.crystal.and_then(|c| g.world.crystals().get(c).map(|x| (c, x.0)));
                        if let Some((ci, cat)) = c {
                            self.marks.pump_rig(cat, pump_rig_r(g, ci), built, health, working, full, b.level);
                            if full > 0.0 {
                                glows.push(Light { at: cat, reach: 5.0 * sc, lift: 3.0, colour: [92.0, 224.0, 232.0], strength: 0.8 * full });
                            }
                        }
                        // The head faces up the screen, its hose down into
                        // the crystal under it.
                        self.marks.building_facing(b.at, half, kind, built, health, working, 0.0, -std::f32::consts::FRAC_PI_2);
                        if b.up() && b.level > 1 {
                            let below = (half * zoom).max(2.0) + 5.0;
                            self.marks.label(b.at, (-6.0, -below - 10.0), &"I".repeat(b.level as usize), 1.5, [126, 232, 240, 255]);
                        }
                    }
                    // A mini larder: its cells and lamp flare as a carrier
                    // banks a load there.
                    Kind::Outpost => self.marks.building_pulse(b.at, half, kind, built, health, working, n.bank / BANKED),
                    // A call: the lamp and horns flare red with a red light
                    // for `SIREN_CALL`; the call itself is the engine's alarm
                    // field, which the view draws spreading and fading
                    // (`View::set_alarm`; before, rings timed from the call
                    // ran out to `siren_r`, a drawn effect).
                    Kind::Siren => {
                        let f = n.call / SIREN_CALL;
                        self.marks.building_pulse(b.at, half, kind, built, health, f > 0.0, f);
                        if f > 0.0 {
                            glows.push(Light { at: b.at, reach: g.rules.build.siren_r * sc * 0.6, lift: 4.0, colour: [255.0, 90.0, 60.0], strength: 1.2 * f });
                        }
                    }
                    _ => self.marks.building(b.at, half, kind, built, health, working),
                }
            }
            let below = (half * zoom).max(2.0) + 5.0;
            // Health once it stands (a ghost's lower hp is its progress,
            // shown by the bar below and the sprite's seam).
            if b.up() {
                self.marks.health(b.at, below, b.hp / b.max_hp.max(1.0));
            }
            if !b.up() {
                progress(&mut self.marks, b.at, below + 4.0);
            }
        }
        // Starfall and boss cores (`Game::starfall`, 2026-11-20): what is
        // coming down and what lies on the ground, drawn from the game's
        // own drops. A fall's shadow darkens and its ring tightens as it
        // comes, the ore itself a streak dropping onto it; down, ore is a
        // hot gold lump and a core a red one, each shrinking with what is
        // left in it and dimming as it cools, with its points beside it.
        // The impact itself is the engine's blast and crater.
        {
            use slime_mould::mould::larder_world::DropKind;
            let now = g.world.ticks();
            let star = g.rules.star;
            for d in &g.starfall.live {
                let w = d.warmth(now);
                if !d.down {
                    let hit = star.kill_r * sc;
                    self.marks.disc(d.at, hit * (0.35 + 0.65 * w), [8, 6, 16, (50.0 + 110.0 * w) as u8], false);
                    self.marks.ring(d.at, hit, [255, 196, 110, (70.0 + 160.0 * w) as u8]);
                    // Coming in from high up and to the right, fast at the end.
                    let fall = (1.0 - w).powf(1.6) * 70.0 * sc;
                    let head = (d.at.0 + 0.55 * fall, d.at.1 - fall);
                    let tail = (head.0 + 0.55 * 9.0 * sc, head.1 - 9.0 * sc);
                    self.marks.streak(head, tail, 3.0, [255, 150, 60, 150], [255, 240, 200, 255]);
                    glows.push(Light { at: head, reach: 6.0 * sc, lift: 6.0, colour: [255.0, 170.0, 90.0], strength: 0.6 + 0.8 * w });
                    continue;
                }
                let full = (d.left / d.worth.max(1e-3)).clamp(0.0, 1.0);
                let (glow, core, light) = match d.kind {
                    DropKind::Ore => ([255, 190, 70, 200], [255, 244, 200, 255], [255.0, 200.0, 90.0]),
                    DropKind::Core => ([235, 60, 80, 200], [255, 210, 220, 255], [240.0, 70.0, 90.0]),
                };
                let heat = 0.35 + 0.65 * w;
                let r = d.r * (0.45 + 0.55 * full.sqrt());
                let fade = |c: [u8; 4], a: f32| [c[0], c[1], c[2], (c[3] as f32 * a) as u8];
                self.marks.disc(d.at, r * 1.25, fade(glow, 0.55 * heat), true);
                self.marks.disc(d.at, r * 0.6, fade(core, heat), false);
                // How long it lasts: a ring that closes as it cools.
                self.marks.ring(d.at, d.r * (1.0 + 1.2 * w), fade(glow, 0.5));
                glows.push(Light { at: d.at, reach: (4.0 + 4.0 * full) * sc, lift: 3.0, colour: light, strength: 1.4 * heat * full.max(0.2) });
                let below = (d.r * zoom).max(4.0) + 4.0;
                self.marks.label(d.at, (-6.0, -below - 10.0), &format!("{}", d.left.ceil() as u32), 1.5, fade(core, 0.95));
            }
        }
        for (k, s) in g.soldiers.iter().enumerate() {
            if !s.alive {
                continue;
            }
            if let Order::Burn(a, b) = s.order {
                self.marks.line(a, b, 4.0, [255, 150, 60, 200]);
            }
            // In the squad mode the squad's own line is drawn below, one
            // for the squad rather than one per soldier.
            if self.controls.selected.contains(&k) && !g.rules.squads {
                let orders = std::iter::once(s.order).chain(s.queue[..s.queued as usize].iter().copied());
                order_chain(&mut self.marks, g, (s.x, s.y), orders.map(|o| (o, false)), s.homing, sc);
            }
        }
        // The controlled squad's order and its queue (Shift), and those of
        // the squads linked to it, as one chain each (owner's playtest,
        // 2026-11-14 20:12Z and 20:15Z: "there should be a line showing
        // where the selected squad is moving ... same with attack move and
        // other commands ... if I set up three moves, I should see the full
        // path"). Before, a squad sent somewhere drew nothing: its soldiers
        // hold no order of their own while the squad moves (`Squad::dest`),
        // and a drain drew a line from every soldier.
        if let Some(q) = g.active.filter(|_| g.rules.squads) {
            for r in g.linked(q) {
                let (Some(mid), sq) = (g.middle(r), &g.squads[r]) else { continue };
                let lead = g.members(r).first().map(|&k| g.soldiers[k].order);
                let now = match (sq.dest, sq.work, lead) {
                    (Some((x, y)), _, _) if sq.steer == (0.0, 0.0) => Some(if sq.fight { Order::Fight(x, y) } else { Order::Move(x, y) }),
                    (None, Some(o), _) => Some(o),
                    (None, None, Some(o @ (Order::Attack | Order::Burn(..)))) => Some(o),
                    _ => None,
                };
                let orders = now.map(|o| (o, false)).into_iter().chain(sq.queue[..sq.queued as usize].iter().map(|&o| (o, true)));
                order_chain(&mut self.marks, g, mid, orders, false, sc);
            }
        }
        // A shot's line, only while shots are rays: with bullets flying
        // (`bullet_speed` > 0) the bullet's own streak shows its path, and
        // the line beside it was a painted effect.
        for t in self.tracers.iter().filter(|_| g.rules.bullet_speed <= 0.0) {
            let a = (255.0 * (1.0 - t.age / TRACER)).clamp(0.0, 255.0) as u8;
            self.marks.line(t.from, t.to, 0.0, if t.hit { [255, 236, 170, a] } else { [200, 200, 200, a / 2] });
        }
        // Each bullet in flight (`Game::bullets`) as Noita draws a spell
        // (Reports/noita-spell-look-2026-11-14.md): a streak along this
        // frame's travel (`draw_as_long`), in the colour of the payload it
        // carries (`Bullet::cast`, `Rounds::cast_of`; a plain shot, a
        // turret's or a split piece is pale gold; until the recharge went,
        // 18:05Z, it took its soldier's next cast), and a light that lights
        // the real slime as it flies over it.
        // Its shape is its cards: wide is bigger, quiet dimmer, pierce a
        // long thin lance, split a fatter head, scatter three heads.
        // (Before 11-14: a 3-pixel disc and a line, the same for every
        // card; the owner's playtest: "really bad".)
        self.lights.clear();
        self.lights.append(&mut glows);
        let t = g.world.ticks();
        let tps = g.rules.tps as f32;
        // A tracer as long as 0.08 s of its flight at its own speed (gun
        // look, 2026-11-15; was 0.06 s at the rules' speed for every
        // bullet), so a slug's and a needle's read apart.
        let travel = g.rules.bullet_speed * sc * TRACE_SECS;
        let body_r = soldier_r(g, zoom);
        // Each bullet glides along the leg it flew in the last step, by how
        // far the frame is toward the next (`App::between`), at its own
        // speed, and stops at what it hit.
        let between = self.between();
        let step = g.rules.frame_ticks as f32 / g.rules.tps as f32;
        for (leg, b) in g.last_shots.iter().zip(&g.flown) {
            let end = leg.hit.unwrap_or(leg.to);
            let gone = (end.0 - leg.from.0).hypot(end.1 - leg.from.1);
            let flew = (between * g.rules.bullet_speed * b.speed * sc * step).min(gone);
            let b = &slime_mould::mould::larder_world::Bullet { at: (leg.from.0 + b.dir.0 * flew, leg.from.1 + b.dir.1 * flew), ..*b };
            // A payload this world can't cast yet flies plain, as it lands.
            let castable = |c: &rounds::Cast| match c.payload {
                rounds::Card::Gas => g.world.has_gas(),
                rounds::Card::Germ => g.world.has_germ(),
                _ => true,
            };
            let cast = b.by.and_then(|k| g.soldiers.get(k)).and_then(|s| self.rounds.cast_of(g, s.squad as usize, b.cast)).filter(castable);
            let look = card_look(cast.map(|c| c.payload));
            // Wide, and the bullet's damage (`Ammo::reach`), widen it.
            let wide = rounds::RoundRules::default().wide.powi(cast.map_or(0, |c| c.wide) as i32) * cast.map_or(1.0, |c| c.ammo.reach());
            let quiet = cast.is_some_and(|c| c.quiet);
            let core = (0.22 * sc * zoom).clamp(1.6, 5.0) * wide * if b.split > 0 { 1.3 } else { 1.0 } * if b.pierce > 0 { 0.8 } else { 1.0 };
            let mut len = travel * b.speed * if b.pierce > 0 { 2.4 } else { 1.0 };
            // A soldier's bullet comes out of its barrel (gun look,
            // 2026-11-15): hidden inside the gun, its tail never further
            // back than the muzzle, and drawn the barrel's offset to the
            // side of the aim line (the game fires from the soldier's
            // middle), closing on the line over its first four body radii.
            // Before, a fresh bullet's streak lay over the soldier's own
            // body and left it beside the gun.
            let mut head = b.at;
            // Its light comes up over its first three body radii out of the
            // barrel: a scattergun's pellets all leave the mouth together,
            // and their lights there would add up over the soldier; the
            // mouth's light is the flash's (`MUZZLE_LIGHT`).
            let mut lit = 1.0;
            if let Some(s) = b.by.and_then(|k| g.soldiers.get(k)).filter(|s| s.alive) {
                let from = (b.at.0 - s.x).hypot(b.at.1 - s.y);
                let out = from - mouth.0 * body_r;
                if out <= 0.0 {
                    continue;
                }
                lit = (out / (3.0 * body_r)).clamp(0.0, 1.0);
                len = len.min(out);
                let side = (-b.dir.1 * mouth.1 * body_r, b.dir.0 * mouth.1 * body_r);
                let near = |d: f32| (1.0 - d / (4.0 * body_r)).clamp(0.0, 1.0);
                head = (b.at.0 + side.0 * near(out), b.at.1 + side.1 * near(out));
                let back = (b.at.0 - b.dir.0 * len, b.at.1 - b.dir.1 * len);
                let tail = (back.0 + side.0 * near(out - len), back.1 + side.1 * near(out - len));
                let glow = [look.glow[0], look.glow[1], look.glow[2], if quiet { 90 } else { 235 }];
                self.marks.streak(head, tail, core, glow, look.core);
            } else {
                let tail = (b.at.0 - b.dir.0 * len, b.at.1 - b.dir.1 * len);
                let glow = [look.glow[0], look.glow[1], look.glow[2], if quiet { 90 } else { 235 }];
                self.marks.streak(b.at, tail, core, glow, look.core);
            }
            let glow = [look.glow[0], look.glow[1], look.glow[2], if quiet { 90 } else { 235 }];
            // A carrier (`Bullet::carry`, the Trigger card, 2026-11-16)
            // shows what it carries: a short bead at its head in the carried
            // cast's colour, so a plain bullet carrying a blast reads as one.
            if b.carry > 0 {
                if let Some(c) = b.by.and_then(|k| g.soldiers.get(k)).and_then(|s| self.rounds.cast_of(g, s.squad as usize, b.carry)).filter(castable) {
                    let l = card_look(Some(c.payload));
                    let bead = core * 0.7;
                    let tail = (head.0 - b.dir.0 * 3.0 * bead, head.1 - b.dir.1 * 3.0 * bead);
                    self.marks.streak(head, tail, bead, [l.glow[0], l.glow[1], l.glow[2], 255], l.core);
                }
            }
            if cast.is_some_and(|c| c.scatter) {
                let side = (-b.dir.1 * 0.9 * sc, b.dir.0 * 0.9 * sc);
                for f in [-1.0, 1.0] {
                    let h = (b.at.0 + side.0 * f - b.dir.0 * 0.6 * sc, b.at.1 + side.1 * f - b.dir.1 * 0.6 * sc);
                    self.marks.streak(h, (h.0 - b.dir.0 * len * 0.6, h.1 - b.dir.1 * len * 0.6), core * 0.6, glow, look.core);
                }
            }
            // The bullet's own light: its strike's disc (`Bullet::r`), what
            // it kills (owner 20:22Z: "only the size of the damage. The same
            // for all affects"). It reached 7 cells at 0.9 until 11-14, then
            // 2.5 (#296); with every shot
            // carrying a card (no recharge, #288) and pierce flying on
            // through the body, each hit swept a disc 14 cells across over
            // the slime (owner 20:09Z: "each bullet that hit has a huge wave
            // that goes out. it is too much").
            self.lights.push(Light { at: head, reach: b.r, lift: 3.0, colour: look.light, strength: lit * if quiet { 0.2 } else { 0.5 } });
        }
        // Each soldier's muzzle flash while it burns (`FLASH`), at the
        // mouth of its kicked-back barrel, under the soldiers so the
        // barrel is drawn over the flash's root. Its colour is the cast the
        // shot carried, its shape the squad's gun (`Marks::muzzle_flash`).
        for (k, s) in g.soldiers.iter().enumerate() {
            let Some(l) = self.looks.get(k).filter(|l| s.alive && l.flash > 0.0) else { continue };
            let kind = g.squads.get(s.squad as usize).and_then(|q| q.gun).map_or(GunKind::Rifle, |gun| gun.kind);
            let look = card_look(self.rounds.cast_of(g, s.squad as usize, l.cast).map(|c| c.payload));
            let at = drawn_at(s, l, body_r);
            let tip = muzzle(at, l.facing, body_r, mouth);
            let (len, id) = match kind {
                GunKind::Rifle => (1.15, 0),
                GunKind::Carbine => (1.0, 1),
                GunKind::Long => (1.5, 2),
                GunKind::Scatter => (1.1, 3),
            };
            let len = (len * body_r).max(10.0 / zoom);
            let seed = l.shots.wrapping_mul(7).wrapping_add(k as u32 * 37);
            self.marks.muzzle_flash(tip, len, l.facing, id, seed, l.flash / FLASH, flame(&look), MUZZLE_CORE);
        }
        // Mortar shells in the air (`Game::shells`), each where its flight
        // puts it this frame: the game holds where it was fired, where it
        // comes down and when, and the shell rides an arc between them,
        // lifted toward the top of the screen, over its shadow on the
        // ground, which darkens and shrinks as it drops.
        let now = g.settled_ticks() as f32 + between * g.rules.frame_ticks as f32;
        let plain = card_look(None);
        for sh in &g.shells {
            let span = sh.lands.saturating_sub(sh.fired).max(1) as f32;
            let at = |f: f32| {
                let d = (sh.to.0 - sh.from.0).hypot(sh.to.1 - sh.from.1);
                let lift = 4.0 * MORTAR_ARC * d * f * (1.0 - f);
                ((sh.from.0 + (sh.to.0 - sh.from.0) * f, sh.from.1 + (sh.to.1 - sh.from.1) * f), lift)
            };
            let f = ((now - sh.fired as f32) / span).clamp(0.0, 1.0);
            let (ground, lift) = at(f);
            let (back, back_lift) = at((f - 0.05).max(0.0));
            let shade = (70.0 + 110.0 * f) as u8;
            self.marks.disc(ground, (0.5 + 0.4 * (1.0 - f)) * sc, [8, 8, 6, shade], false);
            let head = (ground.0, ground.1 - lift);
            let tail = (back.0, back.1 - back_lift);
            let core = (0.3 * sc * zoom).clamp(2.0, 6.0);
            self.marks.streak(head, tail, core, [plain.glow[0], plain.glow[1], plain.glow[2], 235], plain.core);
        }
        // Turrets over the bullets, so a shot leaving the barrel tip
        // (`TURRET_MUZZLE`; mould at its foot is shot from nearer) comes
        // out of the barrels rather than lying over them (turret look,
        // 11-14).
        for (k, half) in turrets {
            turret_mark(&mut self.marks, &mut self.lights, &self.blooks, g, k, half);
            let b = &g.buildings[k];
            self.marks.health(b.at, (half * zoom).max(2.0) + 5.0, b.hp / b.max_hp.max(1.0));
        }
        // A burst lights the world for its first tenth of a second (Noita's
        // explosion light lasts 0.08 s at 8 radii), brighter the more it
        // killed; after that only what it did stays: the hole, the crater,
        // the sleepers, the sick colour.
        for p in self.rounds.pops.iter().rev() {
            let age = (t.saturating_sub(p.tick)) as f32 / tps;
            if age > BURST_LIGHT {
                break;
            }
            let f = 1.0 - age / BURST_LIGHT;
            let look = card_look(Some(p.payload));
            let at = (p.burst.at.0 * sc, p.burst.at.1 * sc);
            // A bomb's light reaches its throw ring and grows with the walkers
            // thrown, so the flash and the spray read as one (#225).
            let thrown = (p.flung as f32 / 400.0).min(1.0);
            // Its own radius, what it kills, sleeps or sickens (owner 20:22Z:
            // "the blast light affect should only be the size of the damage.
            // The same for all affects"); a bomb's throw ring is not damage.
            // Was 3 radii after Noita's 8, then 1.5 (#296): every shot now
            // casts, so a payload squad lit the whole front each volley.
            self.lights.push(Light { at, reach: p.burst.r * sc, lift: 4.0, colour: look.light, strength: f * f * (1.0 + (p.killed as f32 / 200.0).min(1.0) + 0.5 * thrown) });
        }
        // A burning flare (`Card::Flare`, `Rounds::burning`) is an ember
        // where it burns and a hot red light round it, for as long as it
        // burns, guttering out over its last `FLARE_GUTTER`: game state,
        // not a painted pulse. The light reaches half as far as the alarm,
        // so it is never bigger than what the flare does (the owner's rule
        // above) and marks the flare more than it washes the ground. Until
        // 2026-11-17 a flare's only sign was the alarm field's haze on the
        // ground it lay on; the view now shows the alarm on slime alone
        // (`view.wgsl`, `flush_at`), and a flare's strike has no radius for
        // the burst light above, so one cast onto bare ground to draw a
        // Swarmer would have shown nothing.
        let flare = card_look(Some(rounds::Card::Flare));
        for (at, reach, left) in self.rounds.burning(t) {
            let f = (left as f32 / tps / FLARE_GUTTER).min(1.0);
            let at = (at.0 * sc, at.1 * sc);
            self.marks.streak(at, at, (0.3 * sc * zoom).clamp(1.5, 6.0), [flare.glow[0], flare.glow[1], flare.glow[2], (255.0 * f) as u8], flare.core);
            self.lights.push(Light { at, reach: 0.5 * reach * sc, lift: 3.0, colour: flare.light, strength: FLARE_LIGHT * f });
        }
        // Plain hits (a shot with no cast) light their spot as the tracer
        // fades, and a muzzle lights round the soldier as it fires.
        // Once the drawn bullet gets there (`Tracer::delay`), white-hot for
        // its first moment, then gold, falling as the square of what is
        // left (gun look, 2026-11-15; was gold at 0.5 falling straight from
        // the step's start, before the bullet arrived). 1.4 at the peak lit
        // a soldier white when slime at its feet was hit; 0.9 is a flash
        // that still reads over the bright slime.
        // One volley's hits share one hit's light: a scattergun's pellets
        // (pushed together, from one mouth in one step) each light their
        // share, so a fan landing on one spot lights it as one hit does.
        // Summed, four pellets 2 cells out lit the squad white (gun look,
        // 2026-11-15).
        let ts = &self.tracers;
        let mut i = 0;
        while i < ts.len() {
            let n = ts[i..].iter().take_while(|t| t.from == ts[i].from && t.age == ts[i].age).count();
            let volley = &ts[i..i + n];
            i += n;
            let share = 1.0 / volley.iter().filter(|t| t.hit).count().max(1) as f32;
            for tr in volley.iter().filter(|tr| tr.hit) {
                let age = tr.age - tr.delay;
                if !(0.0..TRACER).contains(&age) {
                    continue;
                }
                let f = 1.0 - age / TRACER;
                let hot = (f * 2.0 - 1.0).max(0.0);
                let colour = [255.0, 214.0 + 41.0 * hot, 150.0 + 100.0 * hot];
                self.lights.push(Light { at: tr.to, reach: tr.r, lift: 3.0, colour, strength: 0.9 * f * f * share });
            }
        }
        for (k, s) in g.soldiers.iter().enumerate() {
            let fl = self.looks.get(k).map_or(0.0, |l| l.flash / FLASH);
            if let Some(l) = self.looks.get(k).filter(|_| s.alive && fl > 0.0) {
                // At the barrel's mouth (gun look, 2026-11-15; was at the
                // soldier's middle, under its body).
                let tip = muzzle(drawn_at(s, l, body_r), l.facing, body_r, mouth);
                self.lights.push(Light { at: tip, reach: MUZZLE_LIGHT * sc, lift: 3.0, colour: [255.0, 220.0, 170.0], strength: 0.6 * fl });
            }
            // The slime biting it (the owner, 2026-11-18 18:04Z and
            // 18:08Z: "it just looks like more of the same slime"): while
            // any walker is within its touch (`Soldier::touching`, the
            // walkers that bite it, `squad::read`), that disc, whose slime
            // the view draws in its own crimson (`Light::bite`). Until
            // 11-18 a bite was an orange light on the bitten side, 3.5 dish
            // cells across the radius (2.3 touches; cut to the touch in
            // #498 that day), on gold slime: it read as more slime, and lit
            // a wader's squad in an orange wash.
            // It throbs with each flinch (every `FLINCH_HP` of damage).
            if s.alive && s.touching > 0 {
                let f = self.looks.get(k).map_or(0.0, |l| l.flinch / FLINCH);
                self.lights.push(Light::bite((s.x, s.y), g.rules.touch * sc, 0.75 + 0.25 * f));
            }
        }
        // A lodged cast (delay, thick) is an ember where it waits, its light
        // swelling as its fuse burns down (delay) or steady (thick).
        for (c, at, due) in self.rounds.lodged() {
            let look = card_look(Some(c.payload));
            let left = due.saturating_sub(t) as f32 / tps;
            let f = if c.trigger == Some(rounds::Card::Delay) { 1.0 - (left / self.rounds.rules.delay_secs).clamp(0.0, 1.0) } else { 0.4 };
            self.marks.streak(at, at, (0.18 * sc * zoom).clamp(1.2, 4.0), [look.glow[0], look.glow[1], look.glow[2], (90.0 + 150.0 * f) as u8], look.core);
            self.lights.push(Light { at, reach: MUZZLE_LIGHT * sc, lift: 2.0, colour: look.light, strength: 0.3 + 0.7 * f });
        }
        // A soldier is drawn its touch wide (walkers that close drain it),
        // and at least 8 pixels across the radius, so it reads at any zoom
        // (at 0.9 dish cells, the stand-in's size, it was a 4-pixel dot at
        // the whole map's zoom).
        // Drawn 1.15 times that and at least 10 pixels since the art review
        // (2026-11-14: a 20-pixel olive disc at zoom 6); elbow room
        // (`Rules::elbow`, 1.6 touch radii) keeps the bodies apart. The
        // dead first, so the living are drawn over them.
        let r = body_r;
        let since = g.time();
        // The landing (Look D, 2026-11-20): each soldier drops in, its
        // shadow on the ground firming under it (`landing`).
        if since < LAND_SECS + LAND_STAGGER * g.soldiers.len() as f32 {
            for (k, s) in g.soldiers.iter().enumerate().filter(|(_, s)| s.alive) {
                let (_, _, u) = landing(k, since, r);
                if u < 1.0 {
                    let a = (40.0 + 110.0 * u) as u8;
                    self.marks.disc((s.x, s.y), r * (0.55 + 0.35 * u), [0, 0, 0, a], false);
                }
            }
        }
        let order = g.soldiers.iter().enumerate().filter(|(_, s)| !s.alive).chain(g.soldiers.iter().enumerate().filter(|(_, s)| s.alive));
        for (k, s) in order {
            let l = self.looks.get(k).copied().unwrap_or(Look::new(s));
            // The load as charge in its pack, the take as light at its
            // hands (harvest look, 11-14; the carry bar is gone).
            let carry = if g.rules.carry > 0.0 { s.carry / g.rules.carry } else { 0.0 };
            let at = if s.alive { drawn_at(s, &l, r) } else { (s.x, s.y) };
            let bite = if s.alive { l.flinch / FLINCH } else { 0.0 };
            // The fallen lie as they fell, in a pool of blood that spreads
            // over `POOL_GROW` (the owner's playtest, 2026-11-18 16:12Z).
            if !s.alive {
                let n = self.marks.len();
                self.marks.fallen(at, r, l.facing, team_colour(0), (l.fallen / POOL_GROW).min(1.0), k as u32 * 97 + 13);
                self.marks.dress_since(n, skin);
                continue;
            }
            // No sprite flash: the muzzle flash is its own mark, drawn
            // above as light (gun look, 2026-11-15).
            let n = self.marks.len();
            let (lift, grow, _) = landing(k, since, r);
            let at = (at.0, at.1 - lift);
            self.marks.soldier_bitten(at, r * grow, l.facing, team_colour(0), 0.0, s.alive, carry, l.take / TAKE, bite, l.bite);
            self.marks.dress_since(n, skin);
            self.marks.doze_since(n, doze_tint(s.doze, g.rules.doze), g.dozing(k));
        }
        // Every living soldier's head again, above every body, so a packed
        // squad shows each one (Look A3, a squad of 40 you can count: at
        // planet 1's first crystal 35 soldiers stood 3.3 fine cells from
        // their nearest, under half a figure across, and later figures'
        // shoulders hid earlier ones' heads).
        for (k, s) in g.soldiers.iter().enumerate().filter(|(_, s)| s.alive) {
            let l = self.looks.get(k).copied().unwrap_or(Look::new(s));
            let carry = if g.rules.carry > 0.0 { s.carry / g.rules.carry } else { 0.0 };
            let n = self.marks.len();
            let (lift, grow, _) = landing(k, since, r);
            let at = drawn_at(s, &l, r);
            self.marks.soldier_head((at.0, at.1 - lift), r * grow, l.facing, team_colour(0), 0.0, carry, l.take / TAKE, l.flinch / FLINCH, l.bite);
            self.marks.dress_since(n, skin);
            self.marks.doze_since(n, doze_tint(s.doze, g.rules.doze), g.dozing(k));
        }
        // The bars after every figure, so a neighbour never hides one.
        for s in &g.soldiers {
            if s.alive {
                // Health only when hurt (art direction rule 6).
                let below = (r * zoom * 0.9).max(2.0) + 5.0;
                self.marks.health((s.x, s.y), below, s.hp / g.rules.hp);

            }
        }
        let units = self.units();
        self.controls.prune(&units);
        self.controls.marks(&units, &self.camera, r, &mut self.marks);
    }

    pub(super) fn model(&self, hover: Option<(f32, f32)>, ghost: Option<Ghost>) -> Model {
        let Some(g) = &self.game else { return Model::default() };
        let r = g.rules;
        let sc = g.world.scale();
        let sel = self.living(&self.controls.selected);
        // Held at the end through its beat (`ending`).
        let t = self.clock_now(g);
        let clock = |t: f32| format!("{}:{:02}", t as u32 / 60, t as u32 % 60);
        // Income only (owner, 2026-11-15: the flow "should always be
        // positive"; build costs "should just come out of the bank"): the
        // rate food came in over the last 10 s (`Game::income`). It was
        // the bank's net change, so a placed building showed as "-50/s"
        // for the next 10 s, and stayed there after it stood.
        let rate = self.bank_log.front().filter(|f| t - f.0 >= 2.0).map(|&(t0, i0)| ((g.income() - i0) / (t - t0)).max(0.0));
        let lost = g.soldiers.iter().filter(|s| !s.alive).count();
        let took = g.take_share();
        let mut m = Model {
            resources: vec![
                Resource {
                    icon: Icon::Crystal,
                    colour: theme::ENERGY,
                    label: "Food: the bank, and what comes in a second (pumps, draining and guarding crystals). Soldiers, walls and buildings cost it once, straight from the bank".to_string(),
                    value: format!("{:.0}", g.bank),
                    rate: rate.map(|v| format!("+{v:.0}/s")),
                },
                Resource { icon: Icon::Soldier, colour: theme::TEXT, label: format!("Soldiers alive (at most {})", r.max_squad), value: format!("{}", g.alive()), rate: (lost > 0).then(|| format!("-{lost} lost")) },
                Resource { icon: Icon::Mould, colour: [196, 220, 120, 255], label: if g.world.garden.is_some() { "The enemy mould: walkers alive".into() } else { "The mould: walkers alive".into() }, value: thousands(g.mould), rate: g.core_down.map(|_| "core taken".into()) },
                Resource { icon: Icon::Target, colour: theme::ACCENT, label: "Your share of the charge gone from the crystals (with the mould's size, the score at 15:00)".into(), value: format!("{:.0}%", took * 100.0), rate: None },
            ],
            // The gun range has no time limit to show. A run's planet counts
            // up (owner, 2026-11-20 14:10Z: "there should not be a countdown
            // clock"): the time, then the speed star's time until it passes,
            // and the stalemate's limit only in its last two minutes.
            clock: if self.args.range.is_some() {
                clock(t)
            } else if r.par > 0.0 {
                if t < r.par {
                    format!("{}  \u{b7}  star by {}", clock(t), clock(r.par))
                } else if t >= r.cap - 120.0 {
                    format!("{}  \u{b7}  the planet ends at {}", clock(t), clock(r.cap))
                } else {
                    clock(t)
                }
            } else {
                format!("{} / {}", clock(t), clock(r.cap))
            },
            speed: if self.paused { Some("Paused".into()) } else if self.fast { Some("x4".into()) } else { None },
            // The switches this round plays off their default (the play
            // log's `switches=` says the same).
            trying: slime_mould::build::switches::state(&self.args.keys).into_iter().map(|(s, choice)| format!("{} {choice}", s.label)).collect(),
            // Every switch, for the pause menu's Experiments page.
            experiments: experiments::rows(&self.args.keys, &self.exp_next),
            most_off: slime_mould::build::switches::MOST_OFF,
            objective: Some(if let Some(tr) = &self.mission {
                tr.progress_line()
            } else if self.args.range.is_some() {
                "Gun range: H opens the armory, B buys soldiers, N a new map".into()
            } else if g.core_rule.kind == CoreKind::Headless {
                "Wipe out the mould, or out-harvest it and cut it down by 15:00".into()
            } else if g.core_down.is_some() {
                "Core taken: wipe out the mould, or cut it down by 15:00".into()
            } else {
                "Take the mould's core, or out-harvest it and cut it down by 15:00".into()
            }),
            idle: g.soldiers.iter().filter(|s| s.alive && s.order == Order::Hold).count(),
            // No hover card while WASD steers (the screen moves under a still cursor).
            aiming: self.sq.as_ref().is_some_and(|q| q.steering()),
            ..Model::default()
        };
        m.selection = sel.iter().map(|&k| {
            let s = g.soldiers[k];
            let badge = match s.order {
                Order::Hold => None,
                Order::Move(..) => Some(Icon::Move),
                Order::Fight(..) => Some(Icon::Attack),
                Order::Build(..) => Some(Icon::Wall),
                Order::Clear(..) => Some(Icon::Crystal),
                Order::Burn(..) => Some(Icon::Burn),
                Order::Guard(..) => Some(Icon::Shield),
                Order::Attack => Some(Icon::Attack),
            };
            Card { id: k, icon: Icon::Soldier, team: 0, hp: s.hp / r.hp, badge }
        }).collect();
        if sel.len() == 1 {
            let k = sel[0];
            let s = g.soldiers[k];
            let doing = match s.order {
                Order::Hold => "holding",
                Order::Move(..) => "moving",
                Order::Fight(..) if s.dense > 0 => "fighting its way",
                Order::Fight(..) => "attack-moving",
                Order::Build(b) => match g.buildings.get(b) {
                    Some(bd) => match bd.kind {
                        Kind::Turret => "building a turret",
                        Kind::Mortar => "building a mortar",
                        Kind::Lure => "building a lure",
                        Kind::Pump => "building a pump",
                        Kind::Outpost => "building an outpost",
                        Kind::Siren => "building a siren",
                        Kind::Wall { .. } => "building a wall",
                    },
                    None => "building",
                },
                Order::Clear(..) if s.homing => "carrying charge home",
                Order::Clear(..) => "harvesting a crystal",
                Order::Burn(..) => "holding a burn line",
                Order::Guard(..) => "guarding",
                Order::Attack => "attacking the core",
            };
            m.info = Some(Info {
                icon: Icon::Soldier,
                colour: team_colour(0),
                title: format!("Soldier {}", k + 1),
                subtitle: format!("{doing}, {}", stance_name(s.stance)),
                hp: Some((s.hp / r.hp, format!("{:.0} / {:.0}", s.hp, r.hp))),
                stats: vec![
                    ("Carrying".into(), format!("{:.0} / {:.0}", s.carry, r.carry)),
                    ("Stance".into(), stance_name(s.stance).into()),
                    ("Gun".into(), g.gun(s.squad as usize).title()),
                    ("Range".into(), format!("{:.0} cells", g.gun_range(s.squad as usize))),
                    ("A shot".into(), if r.price > 0.0 { format!("{:.0} food", r.price) } else { format!("free, one every {:.1} s", g.gun(s.squad as usize).every()) }),
                    ("Mould on it".into(), format!("{}", s.touching)),
                    ("Tools held".into(), format!("{} / {}", self.kit.held(k), self.kit.rules.carry_max)),
                ],
                note: None,
            });
        }
        if let Some(bk) = self.building_sel.filter(|_| sel.is_empty()).filter(|&bk| g.buildings.get(bk).is_some_and(|bd| bd.alive)) {
            m.info = Some(self.building_info(g, bk));
        }
        if self.larder_sel && sel.is_empty() {
            m.info = Some(Info {
                icon: Icon::Hold,
                colour: [214, 196, 150, 255],
                title: "The larder".into(),
                subtitle: match g.rally {
                    Order::Clear(..) => "new soldiers go to harvest".into(),
                    Order::Move(..) => "new soldiers go to the rally point".into(),
                    _ => "new soldiers wait here".into(),
                },
                hp: None,
                stats: vec![("Bank".into(), format!("{:.0}", g.bank)), ("Harvested".into(), format!("{:.0}", g.harvested)), ("Eaten by the mould".into(), format!("{:.0}", g.stolen)), ("Soldiers".into(), format!("{} / {}", g.alive(), r.max_squad))],
                note: Some("B buys a soldier. Right-click sets where new soldiers go; on a crystal they start harvesting it.".into()),
            });
        }
        // The hover card: a crystal or the larder under the cursor.
        if let Some(p) = hover {
            if (p.0 - g.larder.0).hypot(p.1 - g.larder.1) <= 3.0 * sc {
                m.hover = Some(Info {
                    icon: Icon::Hold,
                    colour: [214, 196, 150, 255],
                    title: "The larder".into(),
                    subtitle: "your base".into(),
                    hp: None,
                    stats: vec![("Bank".into(), format!("{:.0}", g.bank)), ("Harvested".into(), format!("{:.0}", g.harvested)), ("Eaten by the mould".into(), format!("{:.0}", g.stolen))],
                    note: Some("Harvesters bring charge here; bought soldiers stand here. Mould touching it eats the bank.".into()),
                });
            } else if let Some((i, c)) = g.crystal_at(p) {
                let charge = g.world.crystals()[i].1;
                let full = self.charge0.get(i).copied().unwrap_or(1.0).max(1.0);
                m.hover = Some(Info {
                    icon: Icon::Crystal,
                    colour: theme::ENERGY,
                    title: crystal_name(i),
                    subtitle: format!("{:.0}% of its charge left", 100.0 * charge / full),
                    hp: Some((charge / full, format!("{:.0} food", charge / r.charge))),
                    stats: vec![("Size".into(), format!("{:.0} cells across", 2.0 * g.world.crystal_r(i) / sc))],
                    note: Some(if (c.0 - g.core.0).hypot(c.1 - g.core.1) < 1.0 { "Where the mould grew from. Stand on it with no mould near for 5 s to take it.".into() } else { "Right-click with soldiers selected to harvest it and hold it.".into() }),
                });
            }
        }
        let any = !sel.is_empty();
        let squads = self.sq.is_some();
        let stance = sel.first().map(|&k| g.soldiers[k].stance).filter(|&f| sel.iter().all(|&k| g.soldiers[k].stance == f));
        let need = |cost: f32| (g.bank < cost).then(|| format!("Need {:.0} more food", cost - g.bank));
        let no_sel = || Some(if squads { "Take command of a squad first".to_string() } else { "Select soldiers first".to_string() });
        let full = g.alive() >= r.max_squad;
        let b = r.build;
        let room = g.standing() < b.max;
        // A building placed by a click (the build page's, and Pump on the
        // main card, owner's Card 2 of 2026-11-15 19:38Z).
        let place = |id: u32, slot: (u8, u8), icon: Icon, name: &str, key: KeyCode, cost: f32, about: String| Command {
            target: Target::Point { radius: b.touch * sc },
            cost: Some(format!("{cost:.0}")),
            ready: g.bank >= cost && room,
            why_not: if room { need(cost) } else { Some(format!("At most {} buildings", b.max)) },
            about,
            ..Command::new(id, slot, icon, name, key)
        };
        let pump = |slot: (u8, u8)| place(PUMP, slot, Icon::Crystal, "Pump", KeyCode::KeyU, b.pump_cost, format!("The main income. Snaps onto a crystal and draws it into the bank, more from bigger crystals ({:.1}% of its food a second, at least {:.0}). Click a pumped crystal to upgrade it: {:.0} food, {:.1}x the draw and {:.2}x the food a level, 3 levels.", b.pump_share * 100.0, b.pump_rate, b.pump_up_cost, b.pump_up_rate, b.pump_up_yield));
        // The card's slots are (row, column) on its 5 x 2 grid (`ui::CARD_COLS`).
        // Card 2 (owner 2026-11-15 19:38Z): one fire button that cycles,
        // Pump on the main card, Heal and Lookouts off the squad panel onto
        // it, the wall on the Build page, no empty tile. Burn a line went
        // (owner 19:51Z) and Guard took its F.
        let at = |rts: (u8, u8), sq: (u8, u8)| if squads { sq } else { rts };
        let fire_name = match stance {
            Some(Stance::HoldFire) => "Fire: hold",
            Some(Stance::Free) => "Fire: free",
            Some(Stance::Defend) => "Fire: defend",
            None if any => "Fire: mixed",
            None => "Fire",
        };
        let fire_about = format!(
            "Click to cycle what the soldiers shoot by themselves: hold fire (nothing), defend (only mould within {:.0} cells, on a crystal it works or on what it guards; the default) and free fire (the thickest mould in reach, {:.0} cells{}). Z, X and C pick one.",
            r.defend,
            g.gun_range(sel.first().map_or(0, |&k| g.soldiers[k].squad as usize)),
            if r.price > 0.0 { format!("; every shot costs {:.0} food", r.price) } else { String::new() }
        );
        let mut cmds = vec![
            Command { target: Target::Point { radius: 2.0 * sc }, ready: any, why_not: no_sel(), about: format!("Walk there, stopping to fire freely at any mould in reach on the way. Click the core to attack it: stand on it with no mould near for {:.0} s and it can grow no more. Shift queues it.", r.core_secs), ..Command::new(ATTACK, (0, 0), Icon::Attack, "Attack-move", KeyCode::KeyA) },
            Command { ready: any, why_not: no_sel(), about: "Drop the order and stand still; still fires by its stance.".into(), ..Command::new(STOP, (0, 1), Icon::Stop, "Stop", KeyCode::KeyS) },
            Command { target: Target::Point { radius: 2.0 * sc }, ready: any, why_not: no_sel(), about: format!("Go to a crystal, hold it, and carry its charge to the larder, {:.0} food a load. Right-click on a crystal does the same.", r.carry), ..Command::new(HARVEST, at((0, 2), (0, 2)), Icon::Crystal, "Harvest a crystal", KeyCode::KeyG) },
            Command {
                target: Target::Point { radius: r.guard_r * sc },
                ready: any,
                why_not: no_sel(),
                about: format!("Click a spot, a building or a crystal: the {} goes there and shoots mould coming onto it ({:.0} cells round a spot, {:.0} past a crystal's or a building's edge), on top of its own defence, till it gets another order. Not on hold fire.", if squads { "squad" } else { "soldier" }, r.guard_r, GUARD_RIM),
                ..Command::new(GUARD, at((0, 3), (0, 1)), Icon::Shield, "Guard", KeyCode::KeyF)
            },
            Command {
                ready: any,
                why_not: no_sel(),
                about: fire_about,
                also: vec![(KeyCode::KeyZ, HOLD_FIRE), (KeyCode::KeyX, DEFEND), (KeyCode::KeyC, FREE)],
                ..Command::new(FIRE, at((0, 4), (0, 3)), match stance {
                    Some(Stance::HoldFire) => Icon::Close,
                    Some(Stance::Free) => Icon::Target,
                    _ => Icon::Eye,
                }, fire_name, KeyCode::KeyX)
            },
            Command {
                cost: Some(format!("{:.0}", r.soldier_cost)),
                ready: g.bank >= r.soldier_cost && !full,
                why_not: if full { Some(format!("The squad is at its most, {}", r.max_squad)) } else { need(r.soldier_cost) },
                about: "A new soldier at the larder.".into(),
                ..Command::new(BUY, (1, 0), Icon::Recruit, "Buy a soldier", KeyCode::KeyB)
            },
            pump((1, 1)),
            Command { about: "Turrets, pumps, walls, lures and sirens.".into(), ..Command::new(BUILD_PAGE, at((1, 3), (1, 2)), Icon::Turret, "Build", KeyCode::KeyQ) },
            Command {
                about: "The squad's tools: blast, det cord, mine, germ, sleep gas. Every soldier at the larder picks one up; a selected soldier there takes up to five, and each one held costs carry.".into(),
                ..Command::new(TOOLS_PAGE, at((1, 4), (1, 3)), Icon::Grenade, "Tools", KeyCode::KeyE)
            },
        ];
        // The squad's own orders, off the squad panel (Card 2): Heal and
        // its lookouts while it works.
        if let (true, Some(a)) = (squads, g.active) {
            let heal_off = super::squadmode::heal_off(g, a);
            cmds.push(Command { ready: heal_off.is_none(), why_not: heal_off, about: "The squad stops where it is and heals, paying food from the bank: fast by a crystal you hold, slow elsewhere. Any order ends it.".into(), ..Command::new(HEAL_ORDER, (0, 4), Icon::Heart, "Heal", super::squadmode::HEAL_KEY) });
            let cover = g.squads[a].cover;
            cmds.push(Command {
                about: format!("How many of the squad keep watch with slime near while it drains or builds: none, some or more (now {}). Click to change. Lookouts work slower but shoot what comes.", cover.name()),
                ..Command::new(LOOKOUTS, (1, 4), Icon::Eye, &format!("Lookouts: {}", cover.name()), super::squadmode::LOOKOUTS_KEY)
            });
        } else if squads {
            cmds.push(Command { ready: false, why_not: no_sel(), about: "The squad stops where it is and heals for food.".into(), ..Command::new(HEAL_ORDER, (0, 4), Icon::Heart, "Heal", super::squadmode::HEAL_KEY) });
            cmds.push(Command { ready: false, why_not: no_sel(), about: "How many of the squad keep watch while it drains or builds.".into(), ..Command::new(LOOKOUTS, (1, 4), Icon::Eye, "Lookouts", super::squadmode::LOOKOUTS_KEY) });
        }
        // The rally point at (1, 2), or on the build page with the larder's
        // other places when sowing has that slot (own=1 fills the main
        // page). The squad mode has none (`squad_card_key`).
        let rally = Command {
            target: Target::Point { radius: 2.0 * sc },
            ready: true,
            about: "Where bought soldiers go: on a crystal they start harvesting it, elsewhere they walk there. Also: select the larder and right-click.".into(),
            ..Command::new(RALLY, (1, 2), Icon::Move, "Rally point", KeyCode::KeyR)
        };
        let mut rally = Some(rally);
        if g.world.garden.is_none() {
            cmds.extend(rally.take());
        }
        // The player's own mould (own=1): Sow on the main page in the RTS
        // round (the rally point's slot), on the tools page in the squad
        // mode (its main page is full); Road on the tools page.
        let mut road: Option<Command> = None;
        let mut sow: Option<Command> = None;
        if let Some(gd) = &g.world.garden {
            let all = |t: Task| any && sel.iter().all(|&k| gd.tasks.get(k) == Some(&t));
            let s = Command { ready: any, on: all(Task::Feed), why_not: no_sel(), about: format!("Sow your own mould (green): while on, the soldier spends {:.0} food a second from the bank, spread as charge where it walks, which your mould eats and grows on. Walk it where you want the mould to go. Press again to stop.", gd.tend.feed), ..Command::new(SOW, if squads { (1, 1) } else { (1, 2) }, Icon::Crystal, "Sow your mould", KeyCode::KeyW) };
            if squads {
                sow = Some(s);
            } else {
                cmds.push(s);
            }
            road = Some(Command { ready: any, on: all(Task::Road), why_not: no_sel(), about: "Lay a road of your mould's trail where the soldier walks, free: your mould follows it. Lead it to food or into the enemy, where it converts the enemy's slime it outweighs. Press again to stop.".into(), ..Command::new(ROAD, (1, 0), Icon::Move, "Lay a road", KeyCode::KeyY) });
        }
        if self.tools_page {
            let k = &self.kit;
            let kr = k.rules;
            let held: u32 = (0..g.soldiers.len()).filter(|&i| g.soldiers[i].alive).map(|i| k.held(i)).sum();
            let tool = |id: u32, slot: (u8, u8), icon: Icon, name: &str, key: KeyCode, target: Target, price: f32, works: bool, about: String| Command {
                target,
                cost: Some(format!("{price:.0}")),
                ready: works && held > 0 && g.bank >= price,
                why_not: if !works { Some("Not on the GPU world yet".into()) } else if held == 0 { Some("No soldier holds a tool; they take them at the larder".into()) } else { need(price) },
                about,
                ..Command::new(id, slot, icon, name, key)
            };
            cmds = vec![
                tool(T_BLAST, (0, 0), Icon::Grenade, "Blast", KeyCode::KeyB, Target::Point { radius: kr.blast_r * sc }, kr.blast_price, true, format!("Kills the mould within {:.0} cells and leaves a crater it can't hold trail in for {:.0} s. The blast's alarm wakes and stirs the mould nearby. Thrown by a soldier holding a tool within {:.0} cells.", kr.blast_r, kr.crater_secs, kr.reach)),
                tool(T_CORD, (0, 1), Icon::Burn, "Det cord", KeyCode::KeyC, Target::Line { width: 2.0 * kr.cord_r * sc }, kr.cord_price, true, format!("A line of small blasts, each with its crater: a cut the mould can't knit for {:.0} s. {:.0} food a cell, at most {:.0} cells.", kr.crater_secs, kr.cord_price, kr.cord_max)),
                tool(T_MINE, (0, 2), Icon::Target, "Mine", KeyCode::KeyD, Target::Point { radius: kr.mine_r * sc }, kr.mine_price, true, format!("Goes off as a blast when {} walkers stand within {:.0} cells of it.", k.mine_trigger(g), kr.mine_r)),
                tool(T_GERM, (0, 3), Icon::Spore, "Germ", KeyCode::KeyG, Target::Point { radius: kr.germ_r * sc }, kr.germ_price, g.world.has_germ(), format!("Makes the mould within {:.0} cells sick; sickness travels along its tubes and starves what it passes.", kr.germ_r)),
                tool(T_GAS, (0, 4), Icon::Pause, "Sleep gas", KeyCode::KeyS, Target::Point { radius: kr.gas_r * sc }, kr.gas_price, g.world.has_gas(), format!("Puts the mould within {:.0} cells to sleep for {:.0} s: it doesn't eat or bite. A blast or gunfire nearby wakes it.", kr.gas_r, kr.gas_secs)),
                // Provoke and Still (the simulation-use plan's B1 and B2,
                // 2026-11-17): test devices for the Mind statuses the gun
                // cards will cast; runs leave them out (`run_hidden`).
                tool(T_PROVOKE, (1, 2), Icon::Alert, "Provoke", KeyCode::KeyX, Target::Point { radius: kr.mind_r * sc }, kr.provoke_price, true, format!("The mould within {:.0} cells hears no news of food for {:.0} s: after 5 s it believes it is cut off, greys and charges the nearest soldier, or runs for a crystal.", kr.mind_r, kr.provoke_secs)),
                tool(T_STILL, (1, 3), Icon::Hold, "Still", KeyCode::KeyZ, Target::Point { radius: kr.mind_r * sc }, kr.still_price, true, format!("The mould within {:.0} cells stops charging for {:.0} s: no surge, panic or raid. It eats nothing meanwhile, so held long it starves and greys.", kr.mind_r, kr.still_secs)),
                Command { about: "Back to the soldiers' commands.".into(), ..Command::new(T_BACK, (1, 4), Icon::Close, "Back", KeyCode::KeyE) },
            ];
            cmds.extend(road);
            cmds.extend(sow);
        }
        if self.build_page {
            let lures = g.buildings.iter().filter(|q| q.alive && matches!(q.kind, Kind::Lure | Kind::Siren)).count();
            // Option 1 of the build page (owner 2026-11-15 19:53Z): what you
            // place along the top, the wall among them, most built first;
            // the switch and Back below. Cancel a build left it: click the
            // building instead (`App::building_sel`).
            cmds = vec![
                place(TURRET, (0, 0), Icon::Turret, "Turret", KeyCode::KeyT, b.turret_cost, format!("A gun that does not move: fires at the thickest mould within {:.0} cells every {:.1} s. Its hits call a Swarmer. Mould touching it wears it down. Select a turret to make it render: {:.0} food, then slime killed in its range pays {:.2} food a walker.", b.turret_range, b.turret_every, b.render_cost, b.render_food)),
                pump((0, 1)),
                place(MORTAR, (1, 2), Icon::Bomb, "Mortar", KeyCode::KeyB, b.mortar_cost, format!("Lobs a shell over walls at the thickest mould it sees within {:.0} cells, the longest reach there is, every {:.1} s; the shell lands a second or two later as a wide strike. It can't hit mould within {:.0} cells of itself. Fires the turrets' cards.", b.mortar_range, b.mortar_every, b.mortar_min)),
                Command {
                    target: Target::Line { width: r.wall_width * sc },
                    cost: Some(format!("{:.0}/cell", r.wall_cost)),
                    ready: g.bank >= r.wall_cost,
                    why_not: need(r.wall_cost),
                    about: "A wall the mould has to wear through, slowly. Both ends within a soldier's range; costs by its length.".into(),
                    ..Command::new(WALL, (0, 2), Icon::Wall, "Wall", KeyCode::KeyV)
                },
                Command {
                    target: Target::Line { width: r.wall_width * sc },
                    cost: Some(format!("{:.0}/cell", r.wall_cost)),
                    ready: g.bank >= r.wall_cost,
                    why_not: need(r.wall_cost),
                    about: "A door: a stretch of wall your soldiers walk through. To the slime it is wall, eaten at a wall's pace, and shots stop at it. Draw it along a wall to cut it in (at once, as thick as the wall), or across a gap to build it. Costs as a wall that long.".into(),
                    ..Command::new(DOOR, (1, 3), Icon::Wall, "Door", KeyCode::KeyG)
                },
                place(LURE, (0, 3), Icon::Bait, "Lure", KeyCode::KeyL, b.lure_cost, "Calm slime streams to it, round rock and walls, and eats it: draw it into your guns or off a crystal. Charging slime ignores it. Free to keep on.".to_string()),
                place(SIREN, (0, 4), Icon::Alert, "Siren", KeyCode::KeyY, b.siren_cost, format!("Calls every {:.0} s with the alarm the mould's kinds answer: a Sleeper wakes, a Sporer bursts, a Swarmer floods in. The plain mould ignores it.", b.siren_every)),
                Command { ready: lures > 0, why_not: Some("No lures or sirens".into()), about: "Switch every lure and siren on, or every one off if all are on.".into(), ..Command::new(LURES, (1, 0), Icon::Bait, "Lures, sirens on/off", KeyCode::KeyJ) },
                place(OUTPOST, (1, 1), Icon::Hold, "Outpost", KeyCode::KeyI, b.outpost_cost, "A second drop-off for carried charge. It smells of your bank, and mould touching it eats from the bank.".into()),
                Command { about: "Back to the soldiers' commands.".into(), ..Command::new(BACK, (1, 4), Icon::Close, "Back", KeyCode::KeyQ) },
            ];
            // No Outpost without the larder (the squad mode): with nothing
            // carried home its only effects were smelling of the bank and
            // letting the mould eat from it (bug sweep's review, 11-14).
            if !g.rules.larder {
                cmds.retain(|c| c.id != OUTPOST);
            }
            // The rally point comes here only in the RTS round with the
            // player's own mould (sowing has its main-page slot): then every
            // slot is taken, and the mortar, the newest, gives up its own
            // (1, 2). Runs, and the squad mode, have no rally point.
            if rally.is_some() {
                cmds.retain(|c| c.id != MORTAR);
            }
            cmds.extend(rally.take());
        }
        // The squad mode: the card's letters off WASD (`squad_card_key`),
        // its words for a game with no larder.
        if squads {
            cmds = cmds
                .into_iter()
                .filter_map(|mut c| {
                    c.key = squad_card_key(c.id, c.key)?;
                    match c.id {
                        HARVEST => {
                            c.name = "Drain a crystal".into();
                            c.about = "The squad drains the crystal into the bank, fast, until it is gone for both sides. Ctrl + right click does the same; a plain right click guards it instead.".into();
                        }
                        ATTACK => c.about = "Send the squad there, stopping to fire at any mould in reach on the way; on the core, attack it.".into(),
                        BUY => c.about = "A new soldier joins the squad you control, at its side.".into(),
                        _ => {}
                    }
                    Some(c)
                })
                .collect();
        }
        // A building clicked with no soldiers or squad (owner 2026-11-15
        // 19:51Z, option b): its own card.
        if let Some(bk) = self.building_sel.filter(|_| sel.is_empty()).filter(|&bk| g.buildings.get(bk).is_some_and(|bd| bd.alive)) {
            cmds = self.building_card(g, bk);
        }
        // A run's locks (`Run::has`): what its shop has not unlocked shows,
        // dimmed, with where to get it.
        if let Some(run) = &self.args.run {
            cmds.retain(|c| !run_hidden(c.id));
            for c in &mut cmds {
                // A run has no sirens (`run_hidden`): the switch is the
                // lures' alone.
                if c.id == LURES {
                    c.name = "Lures on/off".into();
                    c.about = "Switch every lure on, or every one off if all are on.".into();
                    c.why_not = Some("No lures".into());
                }
                if run_lock(c.id).is_some_and(|k| !run.has(k)) {
                    c.ready = false;
                    c.locked = true;
                    c.why_not = Some("Locked: unlock it in the shop between planets".into());
                    c.about = format!("Locked in this run (the shop sells it). {}", c.about);
                }
            }
        }
        for c in &mut cmds {
            if c.ready {
                c.why_not = None;
            }
        }
        if g.world.garden.is_some() {
            // The player's own mould beside the enemy's, in its colour.
            let own = g.world.counts_by_side().0;
            m.resources.insert(3, Resource { icon: Icon::Mould, colour: [120, 230, 90, 255], label: format!("Your own mould (green): walkers alive. Sow it ({}); lay its roads from the tools page (E, then Y); it converts enemy slime it outweighs", if self.sq.is_some() { "U" } else { "W" }), value: thousands(own), rate: None });
        }
        m.commands = cmds;
        m.buildings = g
            .buildings
            .iter()
            .filter(|b| b.alive)
            .map(|b| Site {
                kind: b.kind.name(),
                at: b.at,
                to: if let Kind::Wall { to } = b.kind { Some(to) } else { None },
                r: r.build.touch * sc,
                hp: b.hp,
                max_hp: b.max_hp,
                built: b.built,
                working: b.working,
            })
            .collect();
        let throw = ghost.and_then(|(id, at, from)| self.throw_ghost(g, id, at, from));
        (m.ghost_ok, m.ghost_note) = match ghost {
            _ if throw.is_some() => {
                let (_, ok, note) = throw.as_ref().unwrap();
                (*ok, Some(note.clone()))
            }
            Some((HARVEST, at, _)) => match g.crystal_at(at) {
                Some((i, _)) => (true, Some(format!("harvest {}", crystal_name(i).to_lowercase()))),
                None => (false, Some("not on a crystal".into())),
            },
            Some((WALL, at, Some(from))) => {
                let price = g.wall_price(from, at);
                if g.bank < price {
                    (false, Some(format!("{price:.0} food, {:.0} short", price - g.bank)))
                } else if let Some(why) = g.build_blocked(Kind::Wall { to: at }, from) {
                    (false, Some(why.into()))
                } else {
                    (true, Some(format!("{price:.0} food, release to lay")))
                }
            }
            Some((WALL, at, None)) if !g.rules.build.anywhere && !g.reachable(at) => (false, Some("out of every soldier's range".into())),
            // A door: where it goes (`Game::door_line`), and so its price.
            Some((DOOR, at, Some(from))) => {
                let (a, b, host) = g.door_line(from, at);
                let price = g.wall_price(a, b);
                if g.bank < price {
                    (false, Some(format!("{price:.0} food, {:.0} short", price - g.bank)))
                } else if let Some(why) = g.build_blocked(Kind::Wall { to: b }, a) {
                    (false, Some(why.into()))
                } else if host.is_some() {
                    (true, Some(format!("{price:.0} food, release to cut a door in this wall")))
                } else {
                    (true, Some(format!("{price:.0} food, release to build a door")))
                }
            }
            Some((DOOR, at, None)) if !g.rules.build.anywhere && !g.reachable(at) => (false, Some("out of every soldier's range".into())),
            // On a crystal with a pump: the pump's upgrade (owner,
            // 2026-11-15: "you should be able to upgrade them").
            Some((PUMP, at, _)) if g.pump_crystal(at).and_then(|c| g.pump_on(c)).is_some() => {
                let b = g.pump_crystal(at).and_then(|c| g.pump_on(c)).unwrap();
                let level = g.buildings[b].level;
                match g.upgrade_price(b) {
                    Some(p) if g.bank >= p => (true, Some(format!("upgrade to level {}: {p:.0} food, click", level + 1))),
                    Some(p) => (false, Some(format!("upgrade to level {}: {p:.0} food, {:.0} short", level + 1, p - g.bank))),
                    None if !g.buildings[b].up() => (false, Some("its pump is still being built".into())),
                    None => (false, Some(format!("its pump is at the top level ({level})"))),
                }
            }
            // On a standing turret: its render upgrade (the kill box pays).
            Some((TURRET, at, _)) if g.turret_at(at).is_some() => {
                let b = g.turret_at(at).unwrap();
                match g.upgrade_price(b) {
                    Some(p) if g.bank >= p => (true, Some(format!("render: {p:.0} food, click"))),
                    Some(p) => (false, Some(format!("render: {p:.0} food, {:.0} short", p - g.bank))),
                    None if !g.buildings[b].up() => (false, Some("its turret is still being built".into())),
                    None => (false, Some("this turret already renders".into())),
                }
            }
            Some((id, at, _)) if build_kind(id).is_some() => {
                let kind = build_kind(id).unwrap();
                match g.build_blocked(kind, at) {
                    Some(why) => (false, Some(why.into())),
                    None if g.reachable(at) => (true, Some(format!("{:.0} food, click to build", g.build_price(kind, at)))),
                    None => (true, Some(format!("{:.0} food, click: a soldier walks there", g.build_price(kind, at)))),
                }
            }
            _ => (true, None),
        };
        m.throw = throw.map(|t| t.0);
        m.ghost_snap = match ghost {
            Some((PUMP, at, _)) => g.pump_crystal(at).map(|c| (g.world.crystals()[c].0, pump_rig_r(g, c))),
            _ => None,
        };
        // The card waits while the end plays out in the picture (`ending`).
        let shown = self.end_shown();
        if let Some(tr) = self.mission.as_ref().filter(|t| shown && t.verdict.is_some()) {
            let mut end = mission_screen(tr, g);
            if let (Some(r), Some(v)) = (&self.args.run, tr.verdict) {
                use slime_mould::mould::run;
                let last = r.depth >= run::PLANETS;
                // A run's planet: which, its slime and what that slime
                // does to you (`run::does_to_you`, the line the landing
                // alert says), so the end names what happened to you.
                let tier = r.level().tier;
                end.place = format!("Planet {} of {} · {} · {}", r.depth, run::PLANETS, r.planet().name, title_case(tier.name()));
                end.says = run::does_to_you(tier).says.to_string();
                // A stalemate's limit keeps its own title ("The slime held
                // on"): the run goes on, but the planet is not freed.
                if v.stars > 0 && v.cause != slime_mould::mould::campaign::Cause::Cap {
                    end.title = if last { "The home world is free".into() } else { format!("Planet {} freed", r.depth) };
                }
                let (label, line) = match (v.stars > 0, last) {
                    (true, true) => ("The home world is free", format!("The run is won: {} planets", r.depth)),
                    (true, false) => {
                        // Starfall and boss cores (`Game::starfall`) on top of the win's.
                        let (won, found) = (run::reward(r.depth, v.stars), g.starfall.earned());
                        let line = if found > 0 { format!("+{won} points for the planet, +{found} from starfall and cores ({} now)", r.points + won + found) } else { format!("+{won} points for the shop ({} now)", r.points + won) };
                        ("On to the shop", line)
                    }
                    (false, _) => ("End the run", format!("The run ends on planet {}", r.depth)),
                };
                end.lines.push(line);
                m.carry_on = Some(label.into());
            }
            m.end = Some(end);
        } else if let Some(end) = g.end.filter(|_| shown) {
            m.end = Some(end_screen(end, g, t, map_title(&self.name, self.args.seed)));
        }
        m.help = if self.sq.is_some() {
            m.help_base = Some(squad_help());
            vec![
                ("Place a building".into(), "The squad walks there and raises it".into()),
                ("P".into(), "Pause".into()),
                ("Tab".into(), "Four times faster, or back".into()),
                ("N".into(), "Start the next map".into()),
                ("Q".into(), "Build page: turret, lure, pump, outpost; Q again goes back".into()),
                ("E".into(), "Tools page: blast, det cord, mine, germ, sleep gas; E again goes back".into()),
            ]
        } else { vec![
            ("Right click".into(), "Move there; on a crystal, harvest it".into()),
            ("Shift + order".into(), "Queue it after the soldiers' current orders".into()),
            ("Place a building".into(), "The selected soldiers walk there and raise it; idle soldiers nearby help".into()),
            ("A, click".into(), "Attack-move there; on the core, attack it".into()),
            ("Click the larder".into(), "Select it: B buys, right click sets the rally point".into()),
            ("P".into(), "Pause".into()),
            ("Tab".into(), "Four times faster, or back".into()),
            ("N".into(), "Start the next map".into()),
            ("Q".into(), "Build page: turret, lure, pump, outpost; Q again goes back".into()),
            ("E".into(), "Tools page: blast, det cord, mine, germ, sleep gas; E again goes back".into()),
        ] };
        m.main_menu = self.base.is_some();
        m.loadout = Some(self.loadout(g, &sel));
        // A building selected takes the panel for its own state (option
        // b); a click on open ground, a tile's number or Space gives the
        // squads back.
        m.squads = if self.building_sel.is_some() && m.info.is_some() && sel.is_empty() { None } else { self.squad_panel() };
        m
    }

    /// The loadout panel (`ui::Loadout`): the squad's round, with the gun
    /// of the one selected soldier (or the first selected, or the first
    /// alive) lit at the cast its next shot fires.
    fn loadout(&self, g: &Game<GpuWorld>, sel: &[usize]) -> Loadout {
        let rd = &self.rounds;
        let scope: Vec<usize> = if sel.is_empty() { (0..g.soldiers.len()).filter(|&k| g.soldiers[k].alive).collect() } else { sel.to_vec() };
        let lead = scope.first().copied();
        // The squad owns the gun and its cards (owner 11-14 16:46Z): the
        // controlled squad's, else the lead soldier's.
        let q = g.active.or(lead.map(|k| g.soldiers[k].squad as usize)).unwrap_or(0);
        let gun = g.gun(q);
        let round = rd.cards_of(g, q);
        let next = lead.and_then(|k| rd.next(g, k));
        let cards = gun_cards(&round, &rd.rules, &g.rules, next, |c| match c {
            rounds::Card::Gas => g.world.has_gas(),
            rounds::Card::Germ => g.world.has_germ(),
            _ => true,
        });
        let name = match LOADOUTS.iter().find(|(_, c)| rounds::parse(c).unwrap_or_default() == round) {
            Some((n, _)) => n.to_string(),
            // Rolled bullets (Ctrl+H, a run's): their words are numbers.
            None if round.iter().any(|c| matches!(c, rounds::Card::Ammo(_))) => "rolled bullets".into(),
            None => rounds::name(&round),
        };
        let whose = match (sel.len(), lead) {
            (_, None) => "no soldiers".to_string(),
            (1, Some(k)) => format!("Soldier {}", k + 1),
            (0, Some(k)) => format!("Soldier {} (squad)", k + 1),
            (n, Some(k)) => format!("Soldier {} (of {n})", k + 1),
        };
        let own = g.squads.get(q).is_some_and(|s| s.deck.is_some());
        let whose = if own { format!("{whose}, squad {}'s own cards", q + 1) } else { whose };
        Loadout {
            name: if round.is_empty() { "Plain".into() } else { capital(&name) },
            whose,
            cards,
            slots: (gun.slots as usize).max(1),
            gun: Some((gun_icon(gun.kind), capital(&gun.title()), gun.line())),
            // The budget (owner's card, 11-14 15:34Z) under the cards.
            note: if self.args.range.is_some() {
                Some(if round.is_empty() { "No cards: H, the armory".into() } else { format!("Cost {} of {}  ·  H, the armory", rounds::cost(&round), gun.budget) })
            } else if let Some(r) = self.args.run.as_ref() {
                // A run's loadouts: H opens their panel (2026-11-19; before,
                // H only refused in a run, so the strip had no button).
                let l = r.loadout_name(g.squads.get(q).map_or(0, |s| s.loadout as usize));
                Some(if round.is_empty() { format!("{l}, no cards  ·  H, loadouts") } else { format!("{l}  ·  cost {} of {}  ·  H, loadouts", rounds::cost(&round), gun.budget) })
            } else if round.is_empty() {
                Some("No cards: H or the button for a loadout".into())
            } else {
                Some(format!("Cost {} of {}", rounds::cost(&round), gun.budget))
            },
            cycle: Some((NEXT_ROUND, KeyCode::KeyH)),
            // Every squad's gun, so the strip keeps its width as the
            // commanded squad changes (the steady-HUD pass, 11-20).
            widest: (0..g.squads.len()).map(|q| g.gun(q).slots as usize).max().unwrap_or(0),
        }
    }
}

/// A gun kind's icon (`mould::guns`): the HUD's loadout strip, and the
/// run's shop (the roguelike thread draws guns with it).
pub(super) fn gun_icon(k: GunKind) -> Icon {
    match k {
        GunKind::Rifle => Icon::Rifle,
        GunKind::Carbine => Icon::Carbine,
        GunKind::Long => Icon::LongGun,
        GunKind::Scatter => Icon::Scattergun,
    }
}

/// A payload's colour in the panel: its bullets' glow (`card_look`), so a
/// card and its shot in the world match.
pub(super) fn payload_colour(c: rounds::Card) -> [u8; 4] {
    // A rolled bullet in its payload's colour.
    let c = c.ammo().map_or(c, |a| a.shell.card());
    let g = card_look(Some(c)).glow;
    [g[0], g[1], g[2], 255]
}

/// A card's glyph, frame and words for the loadout panel and the shop,
/// with the round's own numbers: a short sentence, the kind said beside
/// it (`CardKind::label`). The owner, 11-15 02:57Z: "The text for all the
/// gun cards is too much. They should be simplified and clarified". Until
/// then each card had one to three sentences, a rolled bullet its numbers
/// twice with a standard bullet's beside them, and the shop added how to
/// click and drag to every card.
impl App {
    /// A toolkit tool's aim (`Model::throw`; owner 2026-11-16: a better
    /// visual "for aiming and range" when throwing an explosive): its
    /// thrower (`Kit::thrower`, the soldier whose held tool `use_tool`
    /// spends), reach, colour and rings, whether it can go (so a click out
    /// of reach is refused with the reason, as a building's red ghost is,
    /// where it was taken and then alerted "Too far"), and the note by the
    /// cursor. None for anything but the thrown tools, or with no soldier
    /// able to throw at all.
    pub(super) fn throw_ghost(&self, g: &Game<GpuWorld>, id: u32, at: (f32, f32), from: Option<(f32, f32)>) -> Option<(Throw, bool, String)> {
        let kr = self.kit.rules;
        let sc = g.world.scale();
        // The tools' colours: fire for the blasts, amber for a mine that
        // waits, sickly green for the germ, the gas's lavender.
        let (tint, rings, price) = match id {
            T_BLAST => ([255, 168, 72, 255], vec![(kr.crater_r * sc, 150), (2.0 * kr.blast_r * sc, 70)], kr.blast_price),
            T_CORD => ([255, 168, 72, 255], Vec::new(), from.map_or(0.0, |f| kr.cord_price * (f.0 - at.0).hypot(f.1 - at.1))),
            T_MINE => ([240, 204, 96, 255], vec![(kr.blast_r * sc, 150)], kr.mine_price),
            T_GERM => ([176, 226, 96, 255], Vec::new(), kr.germ_price),
            T_GAS => ([176, 156, 255, 255], Vec::new(), kr.gas_price),
            // Provoke an alarm red, Still a cool blue apart from the gas's
            // lavender.
            T_PROVOKE => ([255, 120, 96, 255], Vec::new(), kr.provoke_price),
            T_STILL => ([120, 200, 230, 255], Vec::new(), kr.still_price),
            _ => return None,
        };
        let land = match (id, from) {
            (T_CORD, Some(f)) => ((f.0 + at.0) / 2.0, (f.1 + at.1) / 2.0),
            _ => at,
        };
        let (k, near) = self.kit.thrower(g, land)?;
        let s = g.soldiers[k];
        let reach = kr.reach * sc;
        let d = (s.x - land.0).hypot(s.y - land.1);
        let (ok, note) = if !near {
            (false, format!("{:.0} cells out of reach", ((d - reach) / sc).max(1.0)))
        } else if let (T_CORD, Some(f)) = (id, from) {
            let len = (f.0 - at.0).hypot(f.1 - at.1);
            if len > kr.cord_max * sc {
                (false, format!("a cord is at most {:.0} cells", kr.cord_max))
            } else if g.bank < price {
                (false, format!("{price:.0} food, {:.0} short", price - g.bank))
            } else {
                (true, format!("{price:.0} food, release to lay"))
            }
        } else if id == T_CORD {
            (true, format!("{:.0} food a cell, drag to lay", kr.cord_price))
        } else if !g.world.open(land.0, land.1) {
            (false, "not on open ground".to_string())
        } else if g.bank < price {
            (false, format!("{price:.0} food, {:.0} short", price - g.bank))
        } else {
            (true, format!("{price:.0} food, click to throw"))
        };
        let at = (land != at).then_some(land);
        Some((Throw { from: (s.x, s.y), from_r: 1.3 * soldier_r(g, self.camera.zoom), reach, tint, at, rings }, ok, note))
    }
}

pub(super) fn card_face(c: rounds::Card, r: &RoundRules, w: &Rules) -> (Icon, CardKind, String) {
    let split_n = w.split_n;
    use rounds::Card as C;
    let k = &r.kit;
    match c {
        C::Plain => (Icon::Bullet, CardKind::Payload, "Just its hit.".into()),
        // A rolled bullet (owner 16:46Z): its numbers, then what its
        // payload does; its set's glyph when it is plain.
        C::Ammo(a) => {
            let (icon, words) = match a.shell {
                rounds::Shell::Plain => (ammo_icon(a.set), String::new()),
                s => {
                    let (i, _, w) = card_face(s.card(), r, w);
                    (i, format!(" {w}"))
                }
            };
            // Its set's shape (`bullets::Shape`, 2026-11-16) after the numbers.
            let shape = match a.set {
                rounds::Set::Standard => String::new(),
                _ => format!(" {}", rounds::Card::Ammo(a).about()),
            };
            (icon, CardKind::Payload, format!("{}.{shape}{words}", ammo_numbers(&a)))
        }
        C::Blast => (Icon::Blast, CardKind::Payload, format!("Kills slime within {:.0} cells; its crater holds {:.0} s.", k.blast_r, k.crater_secs)),
        C::Gas => (Icon::Gas, CardKind::Payload, format!("Puts slime within {:.0} cells to sleep.", k.gas_r)),
        C::Germ => (Icon::Germ, CardKind::Payload, format!("Infects slime within {:.0} cells; the sickness spreads.", k.germ_r)),
        // Blast and bomb apart (2026-11-16): the bomb throws, the blast holds.
        C::Bomb => (Icon::Bomb, CardKind::Payload, format!("Throws slime {:.0} cells out; kills little.", r.bomb_r * r.bomb_ring)),
        C::Wide => (Icon::Wide, CardKind::Modifier, format!("Its hit is {:.1}x wider.", r.wide)),
        C::Scatter => (Icon::Scatter, CardKind::Modifier, format!("Its hit becomes {} smaller ones.", r.scatter_n)),
        C::Quiet => (Icon::Quiet, CardKind::Modifier, "It doesn't alarm the slime.".into()),
        C::Delay => (Icon::Delay, CardKind::Trigger, format!("It goes off {:.0} s after landing.", r.delay_secs)),
        C::Thick => (Icon::Thick, CardKind::Trigger, format!("It waits until slime crowds round it (up to {:.0} s).", r.thick_secs)),
        C::Pierce => (Icon::Pierce, CardKind::Flight, "Bullets go through one slime hit.".into()),
        C::Split => (Icon::Split, CardKind::Flight, format!("Bullets split in {split_n} where they land.")),
        // Trails (2026-11-14): each reuses the glyph of the field it lays.
        C::Scorch => (Icon::Burn, CardKind::Flight, format!("Bullets burn a line the slime avoids for {:.0} s.", r.trail_crater_secs)),
        C::Bait => (Icon::Bait, CardKind::Flight, "Bullets leave a scent line the slime follows.".into()),
        C::Mist => (Icon::Gas, CardKind::Flight, "Bullets leave a thin sleeping gas.".into()),
        // Range (owner, 11-14 08:39Z): reuses free fire's target glyph.
        C::Range => (Icon::Target, CardKind::Flight, format!("Shoots {:.0}% farther ({:.0} cells).", 100.0 * w.range_card, w.range * w.range_card)),
        // The owner's notes (11-14 15:06Z): rate of fire, reduce cost, disrupt scent.
        C::Rate => (Icon::Fast, CardKind::Flight, format!("Fires {:.0}x as often.", 1.0 + w.rate_card)),
        C::Thrift => (Icon::Upgrade, CardKind::Modifier, format!("The next card costs {} less budget.", rounds::THRIFT)),
        C::Smother => (Icon::Close, CardKind::Flight, "Bullets wipe the scent slime follows to food.".into()),
        // The element family's second card (2026-11-17).
        C::Tinder => (Icon::Tinder, CardKind::Flight, "Bullets lay a line of dry brush that fire runs along.".into()),
        // The carriers (2026-11-16, step B of the gun-system review).
        C::Trigger => (Icon::Carry, CardKind::Trigger, "It carries the one after it, let out where it hits.".into()),
        C::Expire => (Icon::CarryStop, CardKind::Trigger, "It carries the one after it, let out where it stops.".into()),
        C::Multicast => (Icon::Multicast, CardKind::Trigger, "It fires the one after it too, in the same shot.".into()),
        C::Seek => (Icon::Seek, CardKind::Flight, "Bullets that just miss strike slime beside them.".into()),
        // The shove cards (owner 2026-11-16 05:11Z: "corral and push the
        // slime onto areas. Into a kill zone").
        // One card since 2026-11-17 (Heave and Corral merged).
        C::Heave => (Icon::Heave, CardKind::Modifier, format!("Shoves slime onto your nearest turret or back, {:.1}x a card.", r.heave)),
        C::Pull => (Icon::Pull, CardKind::Modifier, "Its hit drags slime toward the gun.".into()),
        // The Mind bullets (2026-11-16, step D): a status, no kill.
        C::Blind => (Icon::Blind, CardKind::Payload, format!("Slime within {:.0} cells is blind for {:.0} s: it walks straight on.", r.mind_r, r.mind_secs)),
        C::Sour => (Icon::Sour, CardKind::Payload, format!("Slime within {:.0} cells turns from its own trail for {:.0} s.", r.mind_r, r.mind_secs)),
        C::Spin => (Icon::Spin, CardKind::Payload, format!("Slime within {:.0} cells turns in tight rings for {:.0} s.", r.mind_r, r.mind_secs)),
        // Provoke and Still (2026-11-17): the devices' statuses as cards,
        // for the devices' seconds.
        C::Provoke => (Icon::Provoke, CardKind::Payload, format!("Slime within {:.0} cells feels cut off for {:.0} s and charges.", r.mind_r, r.kit.provoke_secs)),
        C::Still => (Icon::Still, CardKind::Payload, format!("Slime within {:.0} cells is calm for {:.0} s: no charge, no eating.", r.mind_r, r.kit.still_secs)),
        // The readers (2026-11-16, step D): a kill of one state only.
        C::Shatter => (Icon::Shatter, CardKind::Payload, format!("Kills the slime gas put to sleep within {:.0} cells.", r.read_r)),
        C::Detonate => (Icon::Detonate, CardKind::Payload, format!("Kills the sick slime within {:.0} cells; let it spread first.", r.detonate_r)),
        // The route family (2026-11-17, step D): moves the slime, kills none.
        C::Flare => (Icon::Flare, CardKind::Payload, format!("Burns {:.0} s, alarming slime within {:.0} cells; kills none.", r.flare_secs, r.flare_r)),
        // The element family (2026-11-17, step D): the fire kills, not the hit.
        C::Torch => (Icon::Torch, CardKind::Payload, format!("Sets plants within {:.0} cells on fire; the fire kills slime.", r.torch_r)),
    }
}

/// A rolled bullet's numbers in a line: "Damage 6.3, speed 1.9x, range
/// 1.7x, spread 0.6x".
pub(super) fn ammo_numbers(a: &rounds::Ammo) -> String {
    format!("Damage {:.1}, speed {:.1}x, range {:.1}x, spread {:.1}x", a.damage(), a.speed(), a.range(), a.spread())
}

/// A card's words with its kind before them ("Next bullet: its hit is
/// 1.6x wider."), as the shop's detail box says them; Thrift works on a
/// card, not a bullet, so says only its own.
pub(super) fn card_words(c: rounds::Card, kind: CardKind, words: &str) -> String {
    if c == rounds::Card::Thrift {
        return words.to_string();
    }
    let mut w = words.chars();
    let first = w.next().map(|f| f.to_lowercase().collect::<String>()).unwrap_or_default();
    format!("{}: {first}{}", kind.label(), w.as_str())
}

/// A bullet set's glyph (`mould::bullets::Set`): the shop's and the
/// loadout strip's.
pub(super) fn ammo_icon(s: rounds::Set) -> Icon {
    match s {
        rounds::Set::Standard => Icon::Bullet,
        rounds::Set::Slug => Icon::Slug,
        rounds::Set::Needle => Icon::Needle,
        rounds::Set::Buckshot => Icon::Buckshot,
    }
}

/// The loadout panel's cards: each card in order, the run of cards that
/// makes the cast `next` lit (a cast is the modifiers and trigger before a
/// payload, and the payload), cards after the last payload idle. Flight
/// cards ride every bullet, so they are never idle.
pub(super) fn gun_cards(round: &[rounds::Card], r: &RoundRules, w: &Rules, next: Option<usize>, castable: impl Fn(rounds::Card) -> bool) -> Vec<GunCard> {
    let payloads = round.iter().filter(|c| c.kind() == rounds::Kind::Payload).count();
    let mut cast = 0;
    let mut out = Vec::new();
    // Each card's cast: the payload it goes with.
    let mut runs = Vec::new();
    for &c in round {
        runs.push(cast);
        if c.kind() == rounds::Kind::Payload {
            cast += 1;
        }
    }
    let payload_of = |j: usize| round.iter().filter(|c| c.kind() == rounds::Kind::Payload).nth(j).copied();
    // The next cast, the casts it carries (`Card::Trigger`) and those fired
    // beside it (`Card::Multicast`, 2026-11-16) light together: they go
    // out in one shot.
    let links = rounds::casts(round);
    let mut lit: Vec<usize> = next.into_iter().collect();
    let mut k = 0;
    while k < lit.len() {
        for j in links.get(lit[k]).into_iter().flat_map(|c| c.child.into_iter().chain(c.with)).map(usize::from) {
            if !lit.contains(&j) {
                lit.push(j);
            }
        }
        k += 1;
    }
    for (i, &c) in round.iter().enumerate() {
        let (icon, kind, mut about) = card_face(c, r, w);
        about = format!("{about} Budget {}.", c.cost());
        let flight = kind == CardKind::Flight;
        let idle = !flight && runs[i] >= payloads;
        let goes_with = if flight { None } else { payload_of(runs[i]) };
        let colour = match (kind, goes_with) {
            (CardKind::Flight, _) => [96, 214, 214, 255],
            (_, Some(p)) => payload_colour(p),
            (_, None) => [146, 156, 168, 255],
        };
        if kind == CardKind::Payload && !castable(c.ammo().map_or(c, |a| a.shell.card())) {
            about.push_str(" Not on this world yet.");
        }
        // A rolled payload bullet's set in the corner (its glyph is its
        // payload's); a plain one's glyph is already its set.
        let badge = match c {
            rounds::Card::Ammo(a) if a.shell != rounds::Shell::Plain => Some(ammo_icon(a.set)),
            _ => None,
        };
        out.push(GunCard {
            icon: Some(icon),
            badge,
            name: c.title(),
            kind,
            colour,
            about,
            next: !flight && !idle && lit.contains(&runs[i]),
            idle,
        });
    }
    out
}

/// A burning flare's light (`Card::Flare`): its strength at the flare (a
/// siren's call is 1.2, a burst's first frame 1 and up; at 1 a flare on
/// the dark ground did not read at zoom 6), and the seconds over which it
/// gutters out at the end of its burn.
const FLARE_LIGHT: f32 = 2.5;
const FLARE_GUTTER: f32 = 0.5;

/// How long a burst lights the world, game seconds.
const BURST_LIGHT: f32 = 0.12;
/// How far a light with no damage of its own reaches, in cells: a
/// soldier's or turret's muzzle, a lodged cast's ember. Every light that
/// does damage reaches only as far as its damage: a bullet and a hit their
/// strike (`Bullet::r`, `Shot::r`), a burst its radius (owner 20:22Z).
/// Until 11-14 a bullet's lit 7 cells, a hit's 4, a muzzle's 5, a turret's
/// 6 and a burst 3 radii, which over the slime read as a wave from every
/// hit once every shot carried a card (owner 20:09Z: "it is too much");
/// #296 cut them to 2.5 cells and 1.5 radii.
const MUZZLE_LIGHT: f32 = 1.0;
/// The mouth of a soldier's barrel in its sprite's frame, body radii, for
/// each Armour look (`armour_skin`): olive, slate, dark camo. Measured by
/// src/world/sprites/cut.py, which puts every gun's tip at x 1.45; across,
/// on the right hand's side. Until 11-19 it was one point, (1.45, 0.305),
/// sprite A's rifle (soldier.svg: y 0.22 to 0.39), which no living soldier
/// wears now.
const MUZZLE: [(f32, f32); 3] = [(1.45, 0.11), (1.45, 0.12), (1.45, 0.23)];
/// A muzzle flash's white-hot core.
const MUZZLE_CORE: [u8; 4] = [255, 250, 232, 255];

/// A muzzle flash's flame: the cast's glow burnt warmer (less green and
/// blue), so the jets read as fire round the white core rather than as
/// one white blot.
fn flame(look: &CardLook) -> [u8; 4] {
    [look.glow[0], (look.glow[1] as f32 * 0.75) as u8, (look.glow[2] as f32 * 0.5) as u8, 255]
}
/// Seconds of a bullet's flight its tracer shows.
const TRACE_SECS: f32 = 0.08;
/// A mortar shell's arc: its peak lifts it this share of its flight's
/// length toward the top of the screen (the view is from above, so the
/// lift and the shadow under it are what read as height).
const MORTAR_ARC: f32 = 0.25;

/// The soldiers' look by the run's Armour upgrade (`run::ITEMS` "hp", 0 to
/// 4 levels; owner, 2026-11-19 14:41Z: "our base soldier should be olive
/// that upgraded to slate that upgrades to heavy", and 15:11Z: "Replace
/// heavy with dark camo"): olive with none, slate at 1 and 2, dark camo at
/// 3 and 4, so the squad shows what you bought (`Marks::dress_since`).
pub(super) fn armour_skin(level: u8) -> u32 {
    match level {
        0 => 0,
        1 | 2 => 1,
        _ => 2,
    }
}

/// A soldier's drawn radius, world cells: its touch 1.15 times over, and
/// at least 10 pixels, so it reads at any zoom (see `marks`).
/// How much of the sleep gas's lavender a soldier takes (`Marks::doze_since`,
/// 2026-11-19), from the cloud's level at its feet (`Soldier::doze`) over
/// the level that puts it out (`Rules::doze`, `at`): a faint quarter as the
/// cloud reaches toward that level, then from 0.4 while it is out up to all
/// of it at `DOZE_DEEP` times the level, so a gassed squad shows how deep it
/// sleeps and clears as the cloud thins (a cast's cloud starts near 20
/// times the level and thins to it in about 15 s).
fn doze_tint(doze: f32, at: f32) -> f32 {
    if at <= 0.0 || doze <= 0.0 {
        return 0.0;
    }
    let l = doze / at;
    if l < 1.0 { 0.25 * l } else { 0.4 + 0.6 * ((l - 1.0) / (DOZE_DEEP - 1.0)).min(1.0) }
}

/// The cloud's level, in times the level that puts a soldier out, at which
/// it takes all of the gas's lavender (`doze_tint`).
const DOZE_DEEP: f32 = 8.0;

pub(super) fn soldier_r<W: World>(g: &Game<W>, zoom: f32) -> f32 {
    (g.rules.touch * g.world.scale() * 1.15).max(10.0 / zoom)
}

/// Where a living soldier is drawn, `r` world cells across the radius: its
/// place, jerked back from a bite (a fifth of `r` at once, easing home over
/// `FLINCH`) and kicked back along its aim by its last shot (gun look,
/// 2026-11-15: a fifth of `r` at once, easing home over `KICK`).
/// The landing (Look D, 2026-11-20: a planet began with the squad simply
/// there; the brief's "a landing that happens in the picture"): soldier
/// `k` drops over `LAND_SECS`, one after another `LAND_STAGGER` apart, from
/// `LAND_LIFT` body radii up the screen and half again its size, easing in
/// as it falls, over its shadow on the ground, which firms as it comes
/// down. Only the drawing: the soldier stands where it is from the first
/// tick. Tried and dropped: a ring of dust spreading from each soldier's
/// feet, which read as a drawn pulse (art direction rule 1), a UI ring,
/// not dust.
const LAND_SECS: f32 = 0.55;
const LAND_STAGGER: f32 = 0.04;
const LAND_LIFT: f32 = 7.0;

/// How far apart two soldiers of a squad may stand and still count as one
/// group for its number (`App::marks`' squad labels), in dish cells.
const LABEL_LINK: f32 = 4.0;
/// Half the width of the column over that soldier whose soldiers the
/// number clears, in dish cells.
const LABEL_COLUMN: f32 = 1.5;

/// Soldier `k`'s landing `since` game seconds after the landing, `r` its
/// body radius: how far up it is drawn (world cells), its size's factor,
/// and how far down it has come (0-1).
pub(super) fn landing(k: usize, since: f32, r: f32) -> (f32, f32, f32) {
    let u = ((since - LAND_STAGGER * k as f32) / LAND_SECS).clamp(0.0, 1.0);
    let e = (1.0 - u) * (1.0 - u);
    (LAND_LIFT * r * e, 1.0 + 0.5 * e, u)
}

pub(super) fn drawn_at(s: &slime_mould::mould::larder_world::Soldier, l: &Look, r: f32) -> (f32, f32) {
    let jerk = (l.flinch / FLINCH).powi(2) * 0.2 * r;
    let kick = (l.kick / KICK).powi(2) * 0.2 * r;
    (s.x - l.bite.cos() * jerk - l.facing.cos() * kick, s.y - l.bite.sin() * jerk - l.facing.sin() * kick)
}

/// The mouth of the barrel of a soldier drawn at `at`, facing `facing`,
/// `r` world cells across the radius, `mouth` in its frame (`MUZZLE`).
pub(super) fn muzzle(at: (f32, f32), facing: f32, r: f32, mouth: (f32, f32)) -> (f32, f32) {
    let (c, s) = (facing.cos(), facing.sin());
    (at.0 + r * (mouth.0 * c - mouth.1 * s), at.1 + r * (mouth.0 * s + mouth.1 * c))
}

/// A payload's colours: the streak's glow and hot core, and its light.
pub(super) struct CardLook {
    pub glow: [u8; 3],
    pub core: [u8; 4],
    pub light: [f32; 3],
}

/// Each payload in its own colour, as each Noita spell is (blast hot
/// orange, gas violet, germ acid green, bomb ice blue; none, a plain shot,
/// pale gold).
pub(super) fn card_look(payload: Option<rounds::Card>) -> CardLook {
    let (glow, core, light): ([u8; 3], [u8; 3], [f32; 3]) = match payload {
        Some(rounds::Card::Blast) => ([255, 120, 40], [255, 236, 190], [255.0, 160.0, 80.0]),
        Some(rounds::Card::Gas) => ([165, 110, 255], [236, 222, 255], [180.0, 140.0, 255.0]),
        Some(rounds::Card::Germ) => ([110, 255, 70], [226, 255, 196], [140.0, 255.0, 110.0]),
        // A bomb throws and a blast burns (2026-11-16, the gun-system
        // review's step A): ice blue, so the two read apart. It burned
        // redder than the blast's orange (#225), and the two were hard to
        // tell apart at a front.
        Some(rounds::Card::Bomb) => ([90, 200, 255], [222, 246, 255], [130.0, 210.0, 255.0]),
        // The Mind bullets (2026-11-16): one rose-magenta family, so a
        // status shot reads apart from a kill.
        Some(rounds::Card::Blind | rounds::Card::Sour | rounds::Card::Spin | rounds::Card::Provoke | rounds::Card::Still) => ([255, 90, 190], [255, 220, 242], [255.0, 130.0, 210.0]),
        // The readers (2026-11-16): a paler flash of what each reads, the
        // gas's violet and the germ's green gone toward white.
        Some(rounds::Card::Shatter) => ([205, 170, 255], [248, 244, 255], [215.0, 190.0, 255.0]),
        Some(rounds::Card::Detonate) => ([215, 255, 60], [250, 255, 220], [225.0, 255.0, 110.0]),
        // The flare (2026-11-17): a signal flare's hot red, apart from
        // the blast's orange.
        Some(rounds::Card::Flare) => ([255, 60, 70], [255, 226, 220], [255.0, 90.0, 90.0]),
        // The torch (2026-11-17): a flame's yellow, more saturated than a
        // plain shot's pale gold and apart from the blast's orange.
        Some(rounds::Card::Torch) => ([255, 225, 0], [255, 252, 210], [255.0, 235.0, 70.0]),
        _ => ([255, 200, 110], [255, 248, 222], [255.0, 214.0, 150.0]),
    };
    CardLook { glow, core: [core[0], core[1], core[2], 255], light }
}

/// The run's unlock (`mould::run::ITEMS`) a command needs, if any: the
/// buildings and the tools the shop sells.
/// The outer radius of pump rig round crystal `c` (`Marks::pump_rig`),
/// world cells: the crystal's hexagon is 1/1.3 of it, so the frame sits
/// just outside the crystal and its pylons a little further.
pub(super) fn pump_rig_r<W: World>(g: &Game<W>, c: usize) -> f32 {
    g.world.crystal_r(c) * 1.3
}

/// Buildings a run never offers (owner, 2026-11-15 21:41Z, D10): the
/// outpost is out, and the siren stays out until it visibly draws the
/// slime. The shop doesn't sell them (`run::ITEMS`). The lure and the
/// lures' switch were hidden with them until 2026-11-18 (owner's card,
/// 03:03Z: "Lure now, in the three planets"); it needs no unlock
/// (`run_lock`).
pub(super) fn run_hidden(id: u32) -> bool {
    // Provoke and Still are test devices until the gun thread's cards.
    matches!(id, SIREN | OUTPOST | T_PROVOKE | T_STILL)
}

pub(super) fn run_lock(id: u32) -> Option<&'static str> {
    Some(match id {
        WALL | DOOR => "walls",
        TURRET => "turret",
        MORTAR => "mortar",
        // The lure is free from the first planet (Direction, 2026-11-18
        // 08:38Z, on the owner's "Lure now, in the three planets"): for an
        // hour it was a shop unlock, which left planet 1 without it.
        LURE | LURES => return None,
        // Pumps are the main income: never locked (owner, 2026-11-15).
        PUMP => return None,
        OUTPOST => "outpost",
        SIREN => "siren",
        T_BLAST => "blast",
        T_MINE => "mine",
        T_CORD => "cord",
        T_GAS => "gas",
        // Never sold: the GPU world casts no germ yet.
        T_GERM => "germ",
        _ => return None,
    })
}

/// A building's icon on its card and panel.
fn building_icon(k: Kind) -> Icon {
    match k {
        Kind::Turret => Icon::Turret,
        Kind::Mortar => Icon::Bomb,
        Kind::Lure => Icon::Bait,
        Kind::Pump => Icon::Crystal,
        Kind::Outpost => Icon::Hold,
        Kind::Siren => Icon::Alert,
        Kind::Wall { .. } => Icon::Wall,
    }
}

/// A building's name with a capital, for the panel's title.
fn building_title(k: Kind) -> String {
    let n = match k {
        Kind::Wall { .. } => "wall",
        k => k.name(),
    };
    let mut c = n.chars();
    c.next().map_or(String::new(), |f| f.to_uppercase().chain(c).collect())
}

/// The selected building (owner 2026-11-15 19:51Z, option b: "Click a
/// building to select it when no squad is selected"): its panel and its
/// card. The research's take (Reports/menu-layout-research-2026-11-15):
/// in the squad mode the left button fires and the right sends, so the
/// click selects only with no squad commanded.
impl App {
    /// The panel: health, level and state.
    pub(super) fn building_info(&self, g: &Game<GpuWorld>, bk: usize) -> Info {
        let bd = g.buildings[bk];
        let mut stats = Vec::new();
        let subtitle = if !bd.up() {
            stats.push(("Built".into(), format!("{:.0}%", bd.built * 100.0)));
            let builders = g.soldiers.iter().filter(|s| s.alive && s.order == Order::Build(bk)).count();
            stats.push(("Builders".into(), format!("{builders}")));
            if builders == 0 { "being built, nobody on it".to_string() } else { "being built".to_string() }
        } else {
            match bd.kind {
                Kind::Pump => {
                    stats.push(("Level".into(), format!("{} of {PUMP_LEVELS}", bd.level)));
                    stats.push(("Draws".into(), format!("{:.0} food a second", g.pump_draw(bk) * g.pump_yield(bk))));
                    // The level in the subtitle: the panel's stats show only
                    // where it is wide.
                    let doing = if bd.crystal.is_some_and(|c| g.pump_flows(c)) { format!("pumping {:.0} food a second", g.pump_draw(bk) * g.pump_yield(bk)) } else { "its crystal is spent".into() };
                    format!("level {} of {PUMP_LEVELS}, {doing}", bd.level)
                }
                Kind::Turret => {
                    stats.push(("Range".into(), format!("{:.0} cells", g.rules.build.turret_range)));
                    if bd.working { "firing".into() } else { "watching".into() }
                }
                Kind::Mortar => {
                    let b = &g.rules.build;
                    stats.push(("Range".into(), format!("{:.0} to {:.0} cells", b.mortar_min, b.mortar_range)));
                    let up = g.shells.iter().filter(|s| s.mortar == bk).count();
                    stats.push(("Shells in the air".into(), format!("{up}")));
                    if up > 0 { "shelling".into() } else { "watching".into() }
                }
                Kind::Lure | Kind::Siren => {
                    if bd.on { "on".into() } else { "off".into() }
                }
                Kind::Outpost => "standing".into(),
                Kind::Wall { .. } => {
                    let level = g.buildings[g.wall_of(bk)].level;
                    let cells = g.rules.wall_width * f32::from(level);
                    stats.push(("Thickness".into(), format!("{cells:.0} cells, level {level} of {WALL_LEVELS}")));
                    if bd.door { format!("{cells:.0} cells thick; your soldiers walk through, the mould eats it as wall") } else { format!("{cells:.0} cells thick; the mould wears it slowly") }
                }
            }
        };
        stats.push(("Mould on it".into(), format!("{}", bd.touching)));
        Info {
            icon: building_icon(bd.kind),
            colour: theme::TEXT,
            title: if bd.door { "Door".into() } else { building_title(bd.kind) },
            subtitle,
            hp: Some((bd.hp / bd.max_hp.max(1.0), format!("{:.0} / {:.0}", bd.hp, bd.max_hp))),
            stats,
            note: Some("Its actions are on the card. Click open ground to put it down.".into()),
        }
    }

    /// The card: call off a ghost, pull down a standing building (half its
    /// price back, owner 2026-11-15 21:37Z),
    /// upgrade a pump, switch a lure or siren.
    pub(super) fn building_card(&self, g: &Game<GpuWorld>, bk: usize) -> Vec<Command> {
        let bd = g.buildings[bk];
        let price = g.build_price(bd.kind, bd.at);
        let mut cmds = Vec::new();
        if !bd.up() {
            cmds.push(Command {
                cost: Some(format!("+{price:.0}")),
                about: format!("Call it off: its whole price, {price:.0} food, comes back, and its builders go on to their next order."),
                ..Command::new(CANCEL_BUILD, (0, 0), Icon::Close, "Call it off", KeyCode::Delete)
            });
        } else {
            let wall = matches!(bd.kind, Kind::Wall { .. });
            let back = g.rules.build.demolish * price;
            cmds.push(Command {
                cost: (!wall).then(|| format!("+{back:.0}")),
                ready: !wall,
                why_not: wall.then(|| "A wall can't be pulled down yet: the mould wears it away".to_string()),
                about: format!("Pull it down: {:.0}% of its price, {back:.0} food, comes back.", g.rules.build.demolish * 100.0),
                ..Command::new(DEMOLISH, (0, 0), Icon::Close, "Demolish", KeyCode::Delete)
            });
        }
        if bd.kind == Kind::Pump {
            let up = g.upgrade_price(bk);
            let need = up.filter(|&p| g.bank < p);
            cmds.push(Command {
                cost: up.map(|p| format!("{p:.0}")),
                ready: up.is_some() && need.is_none(),
                why_not: Some(match (up, need) {
                    (_, Some(p)) => format!("Need {:.0} more food", p - g.bank),
                    (None, _) if !bd.up() => "Still being built".to_string(),
                    _ => "At its top level".to_string(),
                }),
                about: format!("The next level: {:.1}x the draw and {:.2}x the food from it. {PUMP_LEVELS} levels.", g.rules.build.pump_up_rate, g.rules.build.pump_up_yield),
                ..Command::new(UPGRADE, (0, 1), Icon::Upgrade, "Upgrade", KeyCode::KeyU)
            });
        }
        // A turret's one upgrade renders it (the kill box pays, owner
        // 2026-11-15 21:15Z: "only with an upgrade").
        if bd.kind == Kind::Turret {
            let up = g.upgrade_price(bk);
            let need = up.filter(|&p| g.bank < p);
            let b = &g.rules.build;
            cmds.push(Command {
                cost: up.map(|p| format!("{p:.0}")),
                ready: up.is_some() && need.is_none(),
                why_not: Some(match (up, need) {
                    (_, Some(p)) => format!("Need {:.0} more food", p - g.bank),
                    (None, _) if !bd.up() => "Still being built".to_string(),
                    _ => "It already renders".to_string(),
                }),
                about: format!("Render: every walker killed within {:.0} cells of it, by any shot, pays {:.2} food.", b.turret_range, b.render_food),
                ..Command::new(UPGRADE, (0, 1), Icon::Upgrade, "Render", KeyCode::KeyU)
            });
        }
        // Its loadout in a run (owner 2026-11-19 19:22Z: "a default loadout
        // for turrets and then it can be changed one by one in the game"):
        // each press the next; H edits what they hold.
        if let (Some(r), true) = (self.args.run.as_ref(), matches!(bd.kind, Kind::Turret | Kind::Mortar)) {
            let now = bd.loadout.map_or(r.turret_loadout, usize::from);
            let cards: Vec<String> = r.turret_deck_of(now).iter().map(|c| c.title()).collect();
            let cards = if cards.is_empty() { "no cards".to_string() } else { cards.join(", ") };
            let follows = if bd.loadout.is_none() { ", the turrets' default" } else { "" };
            cmds.push(Command {
                ready: r.loadouts.len() > 1,
                why_not: Some("One loadout only: make another with H".into()),
                about: format!("It fires {}{follows} ({cards}). Click for the next loadout; H edits them.", r.loadout_name(now)),
                ..Command::new(TURRET_LOADOUT, (0, 2), Icon::Upgrade, &format!("Loadout {}", now + 1), KeyCode::KeyL)
            });
        }
        // Thicken (owner 2026-11-16 14:25Z): a band of a wall's width on
        // its home side, at the wall's first price; a door cut into a wall
        // thickens its wall, and itself with it.
        if matches!(bd.kind, Kind::Wall { .. }) && bd.up() {
            let w = g.wall_of(bk);
            let up = g.upgrade_price(w);
            let need = up.filter(|&p| g.bank < p);
            let level = g.buildings[w].level;
            let ww = g.rules.wall_width;
            cmds.push(Command {
                cost: up.map(|p| format!("{p:.0}")),
                ready: up.is_some() && need.is_none(),
                why_not: Some(match (up, need) {
                    (_, Some(p)) => format!("Need {:.0} more food", p - g.bank),
                    _ if level >= WALL_LEVELS => format!("At its thickest, {:.0} cells", ww * f32::from(level)),
                    _ => "This wall can't be thickened".to_string(),
                }),
                about: format!(
                    "Thicken: {:.0} more cells of wall on your side, from {:.0} to {:.0}, at once, for its first price. The mould eats one face at a time, so each band buys about as long again. {WALL_LEVELS} levels.",
                    ww,
                    ww * f32::from(level),
                    ww * f32::from(level + 1)
                ),
                ..Command::new(UPGRADE, (0, 1), Icon::Upgrade, "Thicken", KeyCode::KeyU)
            });
        }
        if matches!(bd.kind, Kind::Lure | Kind::Siren) {
            cmds.push(Command {
                about: if bd.on { "Switch it off: no scent, no calls, till you switch it on.".into() } else { "Switch it on again.".into() },
                ..Command::new(SWITCH, (0, 1), Icon::Bait, if bd.on { "Switch off" } else { "Switch on" }, KeyCode::KeyJ)
            });
        }
        for c in &mut cmds {
            if c.ready {
                c.why_not = None;
            }
        }
        cmds
    }
}
