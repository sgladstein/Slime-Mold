//! **Squads** (owner, 2026-11-14 04:08Z: "the unit control should be squad
//! based from the ground up. All soldiers in a squad stick together at
//! least as well as they can ... when you are controlling a specific squad
//! your mouse is the aim for the guns and they all fire in unison at the
//! mouse. maybe whoever you are actively controlling is stronger"). The
//! owner's answers to the feel questions (04:18Z): steer with WASD *and*
//! send with a right click; hold the left button to fire one volley at the
//! cursor; squads you are not controlling keep their last order and fire on
//! their own; 1-9 or the wheel switch, X splits, a box pulls soldiers out, G
//! merges; the controlled squad fires faster and hits harder; a tight blob
//! facing the cursor by default, other formations to choose; bought
//! soldiers join the controlled squad. (After the owner's playtest, 08:01Z,
//! split and merge moved to Y and J and switching to Ctrl + wheel, so the
//! command card's G and X and the wheel's zoom work: `squadmode`, bug
//! sweep 2026-11-14.)
//!
//! **Game state, not window state.** Every soldier belongs to one squad
//! (`Soldier::squad`) and every squad command is a `Cmd`, so a replay gives
//! the same game and the player AI and missions, which order soldiers one
//! by one (`Cmd::Order`), still work: a squad only takes over its members'
//! orders while it is steered or sent somewhere as a squad.
//!
//! **How a squad sticks together.** A steered or sent squad has an
//! *anchor* (`Squad::anchor`) and each member walks to its place in the
//! formation round it (`Game::place`). The anchor goes at `PACE` of a
//! soldier's speed times the wade of its slowest member, so members have
//! speed in hand to catch up their places, and it waits whenever it gets
//! more than `LEAD` ahead of the squad's middle: the squad goes at the pace
//! of its slowest soldiers. One left far behind (`STRAY`) stops counting,
//! so a soldier trapped in the slime does not pin the rest; it keeps
//! walking after them. (First version: the anchor was always the lead
//! ahead of the middle; every member then ran at full speed, so the ones
//! that started behind never caught up and an 8-soldier blob moved 10-12
//! cells wide instead of 5.)
//!
//! **The volley.** A squad given a fire point (`Cmd::Fire`) fires every
//! live member at it on the same tick, each from where it stands, and fires
//! nothing else meanwhile. The point is where the player aims, so it does
//! not need slime under it (`Rules::aim_min`) or a clear line: the bullets
//! stop at the first wall or slime as every shot does.
//!
//! **The controlled squad** (`Game::active`) fires `LeadRules::rate` times
//! as often, with a strike `LeadRules::strike` times wider. Both show in
//! the simulation itself: more bullets and bigger holes. Not volleying, it
//! fires only in self-defence (the defend stance), since its guns are the
//! player's to aim.

use super::*;

/// Most squads at once (ids are reused once a squad is empty).
pub const SQUADS_MAX: usize = 16;
/// How far ahead of its middle a steered or sent squad's anchor goes, dish
/// cells.
pub const LEAD: f32 = 4.0;
/// A member further than this from its squad's middle, dish cells, stops
/// counting toward the middle.
pub const STRAY: f32 = 14.0;
/// Room between soldiers in a formation with elbow room off, dish cells;
/// with it on, the sunflower's ring (`Game::gap`).
pub const GAP: f32 = 1.6;
/// A moving squad's anchor goes at this share of a soldier's speed.
pub const PACE: f32 = 0.75;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Formation {
    /// A tight sunflower round the anchor (the default).
    #[default]
    Blob,
    /// Shoulder to shoulder across the facing, rows of `LINE_ROW`.
    Line,
    /// A V with its point toward the facing.
    Wedge,
    /// A circle facing out, for holding ground.
    Ring,
}

/// Soldiers in a row of `Formation::Line`.
pub const LINE_ROW: usize = 8;

impl Formation {
    pub const ALL: [Formation; 4] = [Formation::Blob, Formation::Line, Formation::Wedge, Formation::Ring];

    pub fn name(self) -> &'static str {
        match self {
            Formation::Blob => "blob",
            Formation::Line => "line",
            Formation::Wedge => "wedge",
            Formation::Ring => "ring",
        }
    }

    pub fn parse(s: &str) -> Option<Formation> {
        Formation::ALL.into_iter().find(|f| f.name() == s)
    }

    pub fn next(self) -> Formation {
        let i = Formation::ALL.iter().position(|&f| f == self).unwrap_or(0);
        Formation::ALL[(i + 1) % Formation::ALL.len()]
    }

    /// Place `r` of `n`, dish cells, with the facing along +x, `gap`
    /// between neighbours.
    pub fn offset(self, r: usize, n: usize, gap: f32) -> (f32, f32) {
        let (x, y) = self.unit_offset(r, n);
        (x * gap, y * gap)
    }

    /// `offset` with a gap of 1 (the local `GAP` shadows the module's).
    fn unit_offset(self, r: usize, n: usize) -> (f32, f32) {
        const GAP: f32 = 1.0;
        match self {
            Formation::Blob => slot(r, GAP),
            Formation::Line => {
                let row = r / LINE_ROW;
                let in_row = (n - row * LINE_ROW).min(LINE_ROW);
                let c = (r % LINE_ROW) as f32 - (in_row as f32 - 1.0) / 2.0;
                (-(row as f32) * GAP, c * GAP)
            }
            Formation::Wedge => {
                if r == 0 {
                    return (0.0, 0.0);
                }
                let back = r.div_ceil(2) as f32;
                let side = if r % 2 == 1 { 1.0 } else { -1.0 };
                (-back * GAP * 0.8, side * back * GAP * 0.8)
            }
            Formation::Ring => {
                if n <= 1 {
                    return (0.0, 0.0);
                }
                // Round enough that neighbours stand a gap apart.
                let rad = (n as f32 * GAP / std::f32::consts::TAU).max(GAP);
                let a = r as f32 / n as f32 * std::f32::consts::TAU;
                (rad * a.cos(), rad * a.sin())
            }
        }
    }
}

/// The controlled squad's edge (`Rules::lead`).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LeadRules {
    /// Shots as often as the others, times this (2: twice the rate).
    pub rate: f32,
    /// Strike radius times this (capped at `squad::R_MAX`).
    pub strike: f32,
}

impl Default for LeadRules {
    fn default() -> Self {
        LeadRules { rate: 2.0, strike: 1.4 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Squad {
    /// Steering (WASD), a unit vector or zero.
    pub steer: (f32, f32),
    /// Where it was sent as a squad (`Cmd::SquadOrder` move or fight), and
    /// whether it fights on the way.
    pub dest: Option<(f32, f32)>,
    pub fight: bool,
    /// Which way its formation faces, a unit vector.
    pub face: (f32, f32),
    pub formation: Formation,
    /// The volley's point, while the player holds fire (`Cmd::Fire`).
    pub fire: Option<(f32, f32)>,
    /// Tick of its next volley.
    pub next_volley: u64,
    /// The point its formation stands round while it moves as one, fine
    /// cells (None when it is not steered or sent).
    pub anchor: Option<(f32, f32)>,
    /// The crystal or building it works (`Order::Clear` or `Order::Build`),
    /// while some of it does (`Game::cover_turn`).
    pub work: Option<Order>,
    /// How many stand guard while it works.
    pub cover: Cover,
    /// Orders it does next (Shift + an order, `Cmd::SquadQueue`).
    pub queue: [Order; SQUAD_QUEUE],
    pub queued: u8,
    /// The squad it is linked through (`Cmd::Link`), the lowest id of its
    /// link; `NO_LINK` alone.
    pub link: u8,
    /// Its gun (owner 11-14 16:46Z: "The squad owns the gun"); None fires
    /// the standard rifle (`Gun::standard`), so a game that hands out no
    /// guns plays as before. Read it through `Game::gun`.
    pub gun: Option<Gun>,
    /// Its own cards (`Cmd::Load`); None fires the army's round (the H
    /// loadout, `Rounds::round`). Read it through `Game::mods` and
    /// `Rounds::cards_of`.
    pub deck: Option<Deck>,
    /// Told to heal (`Cmd::Heal`): it stands where it is and its members
    /// heal for food (`Game::heal`) till it gets another order or all are
    /// whole. Nothing heals otherwise (owner 2026-11-15 07:41Z: "Healing
    /// should be a specific action").
    pub heal: bool,
    /// Told to guard this point (`Order::Guard` as a squad order, owner
    /// 2026-11-15 19:51Z): it goes there and its members shoot slime in
    /// the zone (`Game::guard_zone`) on top of their own defence, till it
    /// is steered or given another order.
    pub guard: Option<(f32, f32)>,
}

/// Orders a squad holds queued.
pub const SQUAD_QUEUE: usize = 6;
/// `Squad::link` of a squad not linked.
pub const NO_LINK: u8 = u8::MAX;

impl Default for Squad {
    fn default() -> Self {
        Squad {
            steer: (0.0, 0.0),
            dest: None,
            fight: false,
            face: (1.0, 0.0),
            formation: Formation::Blob,
            fire: None,
            next_volley: 0,
            anchor: None,
            work: None,
            cover: Cover::Some,
            queue: [Order::Hold; SQUAD_QUEUE],
            queued: 0,
            link: NO_LINK,
            gun: None,
            deck: None,
            heal: false,
            guard: None,
        }
    }
}

impl Squad {
    /// A new squad made from this one (a split, a soldier joining a free
    /// id): its formation and settings, its gun and its cards, nothing it
    /// was doing and no link. Owner 18:05Z: "Both halves get a copy of the
    /// gun." (From 17:40Z to then the new half took the standard rifle and
    /// the army's round, so a great gun was never copied.)
    fn offshoot(&self) -> Squad {
        Squad { formation: self.formation, face: self.face, cover: self.cover, fight: self.fight, next_volley: self.next_volley, gun: self.gun, deck: self.deck, ..Squad::default() }
    }
}

/// A formation's place `o` turned to `face`. A blob and a ring look the
/// same whichever way they face, so they don't turn: turning one only sent
/// every soldier to a new place (bug sweep, 2026-11-14).
fn turned(f: Formation, o: (f32, f32), face: (f32, f32)) -> (f32, f32) {
    match f {
        Formation::Blob | Formation::Ring => o,
        Formation::Line | Formation::Wedge => (o.0 * face.0 - o.1 * face.1, o.0 * face.1 + o.1 * face.0),
    }
}

fn unit(v: (f32, f32)) -> Option<(f32, f32)> {
    let d = v.0.hypot(v.1);
    (d > 1e-4).then(|| (v.0 / d, v.1 / d))
}

impl<W: World> Game<W> {
    /// Room between neighbours in a formation, dish cells: the sunflower's
    /// ring (`Game::slot`: a little over elbow room's separation, so elbow
    /// room never pushes a squad at rest off its places), at least `GAP`.
    pub(super) fn gap(&self) -> f32 {
        (self.rules.elbow * self.rules.touch * 1.05).max(GAP)
    }

    /// Live members of squad `q`, by soldier index.
    pub fn members(&self, q: usize) -> Vec<usize> {
        (0..self.soldiers.len()).filter(|&k| self.soldiers[k].alive && self.soldiers[k].squad as usize == q).collect()
    }

    /// The squads linked with `q` (`Cmd::Link`), `q` among them, in id order.
    pub fn linked(&self, q: usize) -> Vec<usize> {
        let g = self.squads[q].link;
        if g == NO_LINK {
            return vec![q];
        }
        self.live_squads().into_iter().filter(|&r| r == q || self.squads[r].link == g).collect()
    }

    /// Whether squad `q` is the controlled one or linked with it: its shots
    /// get the lead's edge (`LeadRules`).
    pub fn leads(&self, q: usize) -> bool {
        self.active.is_some_and(|a| a == q || (self.squads[q].link != NO_LINK && self.squads[a].link == self.squads[q].link))
    }

    /// Whether squad `q` is doing nothing: not steered, sent or working.
    pub fn idle(&self, q: usize) -> bool {
        let sq = &self.squads[q];
        sq.steer == (0.0, 0.0) && sq.dest.is_none() && sq.work.is_none()
    }

    /// Squad `q` starts its next queued order, if it has one.
    pub(super) fn squad_next(&mut self, q: usize) -> bool {
        let sq = &mut self.squads[q];
        if sq.queued == 0 {
            return false;
        }
        let o = sq.queue[0];
        sq.queue.copy_within(1.., 0);
        sq.queued -= 1;
        self.send_squad(q, o);
        true
    }

    /// Squads with a live member, in id order.
    pub fn live_squads(&self) -> Vec<usize> {
        let mut seen = [false; SQUADS_MAX];
        for s in self.soldiers.iter().filter(|s| s.alive) {
            seen[s.squad as usize] = true;
        }
        (0..SQUADS_MAX).filter(|&q| seen[q]).collect()
    }

    /// The id a new squad would take: the first with no live member, or
    /// None at `SQUADS_MAX`.
    pub fn free_squad(&self) -> Option<usize> {
        let live = self.live_squads();
        (0..SQUADS_MAX).find(|q| !live.contains(q))
    }

    /// The middle of squad `q` (its members within `STRAY` of their plain
    /// mean, else the plain mean), fine cells.
    pub fn middle(&self, q: usize) -> Option<(f32, f32)> {
        let m = self.members(q);
        if m.is_empty() {
            return None;
        }
        let mean = |ks: &[usize]| {
            let n = ks.len() as f32;
            let (x, y) = ks.iter().fold((0.0, 0.0), |a, &k| (a.0 + self.soldiers[k].x, a.1 + self.soldiers[k].y));
            (x / n, y / n)
        };
        let all = mean(&m);
        let stray = STRAY * self.s();
        let near: Vec<usize> = m.iter().copied().filter(|&k| (self.soldiers[k].x - all.0).hypot(self.soldiers[k].y - all.1) <= stray).collect();
        Some(if near.is_empty() { all } else { mean(&near) })
    }

    /// Soldier `k`'s place in its squad's formation, as an offset from the
    /// anchor, dish cells, turned to the squad's facing (`turned`). Its
    /// rank is the one `rank_squad` matched it to, or its index among the
    /// members while the squad's size has changed since.
    ///
    /// (Before the bug sweep, 2026-11-14, the rank was always the index and
    /// every formation turned with the facing, which follows the cursor
    /// for the controlled squad: the owner's playtest found "the soldiers
    /// are constantly shuffling around in their formation". Moving the
    /// mouse turned a resting blob round its middle, and every soldier
    /// walked to its new place; a death renumbered everyone behind it.)
    pub(super) fn place(&self, k: usize) -> (f32, f32) {
        if !self.rules.squads {
            return self.slot(k);
        }
        let s = self.soldiers[k];
        let q = s.squad as usize;
        let m = self.members(q);
        let n = m.len();
        let r = match self.ranks.get(k) {
            Some(&r) if self.ranked[q] as usize == n && (r as usize) < n => r as usize,
            _ => m.iter().position(|&j| j == k).unwrap_or(0),
        };
        if let Some(o) = self.column_place(k, r, n) {
            return o;
        }
        let sq = self.squads[q];
        turned(sq.formation, sq.formation.offset(r, n, self.gap()), sq.face)
    }

    /// Matches squad `q`'s members to its formation's places round
    /// `centre` (fine cells), nearest pair first, so each soldier takes the
    /// free place nearest it: a formation that turns, grows or loses a
    /// member moves each soldier as little as it can, instead of handing
    /// out places by index. Ties go to the lower soldier and place, so it
    /// is the same on every run.
    pub(super) fn rank_squad(&mut self, q: usize, centre: (f32, f32)) {
        let m = self.members(q);
        let n = m.len();
        if n == 0 || n > u16::MAX as usize {
            return;
        }
        if self.ranks.len() < self.soldiers.len() {
            self.ranks.resize(self.soldiers.len(), 0);
        }
        let sc = self.s();
        let gap = self.gap();
        let sq = self.squads[q];
        let col = &self.columns[q];
        let places: Vec<(f32, f32)> = if col.on && col.places.len() == n {
            col.places.iter().map(|o| (centre.0 + o.0 * sc, centre.1 + o.1 * sc)).collect()
        } else {
            (0..n)
            .map(|r| {
                let o = turned(sq.formation, sq.formation.offset(r, n, gap), sq.face);
                (centre.0 + o.0 * sc, centre.1 + o.1 * sc)
            })
            .collect()
        };
        let mut pairs: Vec<(f32, usize, usize)> = Vec::with_capacity(n * n);
        for (i, &k) in m.iter().enumerate() {
            let s = self.soldiers[k];
            for (r, p) in places.iter().enumerate() {
                pairs.push(((s.x - p.0).powi(2) + (s.y - p.1).powi(2), i, r));
            }
        }
        pairs.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
        let (mut placed, mut taken) = (vec![false; n], vec![false; n]);
        let mut left = n;
        for (_, i, r) in pairs {
            if placed[i] || taken[r] {
                continue;
            }
            placed[i] = true;
            taken[r] = true;
            self.ranks[m[i]] = r as u16;
            left -= 1;
            if left == 0 {
                break;
            }
        }
        self.ranked[q] = n as u16;
    }

    /// Up to `lead` fine cells from `from` toward `to` along a walkable way:
    /// straight if the line is open, else down `to`'s distance map, and
    /// never off standable ground from standable ground (`slide`).
    ///
    /// `sent`: the squad was sent to `to`, so the line must be open all the
    /// way there. Before 2026-11-14 the straight step was taken whenever
    /// the step itself was open: in a dead end facing the goal, such as the
    /// inside of the Fork's V wall with a crystal beyond it, the map's step
    /// back out and the straight step back in alternated, and a sent squad
    /// stood at the wall for good (the squad thread's film: parked at
    /// (258, 190) from 8 s on, 65 shots by 24 s; now round the wall by 12 s,
    /// 244 shots by 24 s). A steered squad keeps the step's own check: its
    /// `to` is 40 cells ahead and moves every step, so the whole line would
    /// build a walking map (15 ms at scale 2) each step it is not clear,
    /// and a player steering at a wall expects to walk up to it.
    fn guide(&mut self, from: (f32, f32), to: (f32, f32), lead: f32, sent: bool) -> (f32, f32) {
        let d = (to.0 - from.0).hypot(to.1 - from.1);
        if d <= lead {
            return self.slide(from, to);
        }
        let straight = (from.0 + (to.0 - from.0) / d * lead, from.1 + (to.1 - from.1) / d * lead);
        if self.walk_line(from, if sent { to } else { straight }) {
            return straight;
        }
        let (w, h) = self.world.size();
        let gi = cell(w, h, to);
        // A steered squad's `to` moves with it: a map to a point within
        // `STEER_NEAR` serves it. Half a lead, at first, was less than the
        // `to` moves each time the anchor does, so every move asked for a
        // new map and the anchor waited for it: a squad steered into rock
        // took 26 s to get round a bar at 1x against 13 s with whole maps,
        // and stood still 798 of 900 steps at 6x (technical review,
        // 2026-11-15; 13.6 and 16.4 s at 10 dish cells).
        let near = if sent { MAP_NEAR * self.s() } else { STEER_NEAR * self.s() };
        let Some(key) = self.map_for(gi, to, near) else {
            // Its map is being built (`pump_maps`): the anchor waits.
            return from;
        };
        let map = &self.paths.maps[&key].0;
        let mut at = cell(w, h, from);
        if map[at] == u32::MAX {
            // The middle is on no walkable way to the point (it sits on a
            // crystal the squad stands round, or the point is in rock):
            // straight on, and each soldier finds its own way. Not into
            // rock or past the map's edge, though (`slide`): steered at the
            // edge or into rock, the anchor went straight in, got more than
            // `LEAD` ahead of soldiers who could not follow, and waited for
            // them for good, so the squad never moved again, whichever way
            // it was steered (owner's playtest, 2026-11-14: "my unit got
            // stuck and cannot move anymore"; 15 of 32 squads steered into
            // the edge or rock for 25 s on seeds 1-4).
            return self.slide(from, straight);
        }
        // `lead` along the way, through the cells' centres: whole cells
        // ran the anchor a cell a step (full pace) whatever its squad's
        // slowest member could do, which only showed once a sent squad took
        // the map whenever its line was shut (2026-11-14).
        let (mut p, mut left) = (from, lead);
        for _ in 0..lead.ceil() as usize + 2 {
            let n = downhill(w, h, map, at, self.rules.corners);
            if n == at {
                break;
            }
            let c = ((n % w) as f32 + 0.5, (n / w) as f32 + 0.5);
            let dd = (c.0 - p.0).hypot(c.1 - p.1);
            if dd >= left {
                return (p.0 + (c.0 - p.0) / dd * left, p.1 + (c.1 - p.1) / dd * left);
            }
            (p, left, at) = (c, left - dd, n);
        }
        p
    }

    /// `to` if a soldier can stand there, else as far as it can go along
    /// one axis from `from` (as `walk` does), else `from`. From ground a
    /// soldier can't stand on, `to` as it is (the way out).
    fn slide(&self, from: (f32, f32), to: (f32, f32)) -> (f32, f32) {
        if self.standable(to.0, to.1) || !self.standable(from.0, from.1) {
            to
        } else if self.standable(to.0, from.1) {
            (to.0, from.1)
        } else if self.standable(from.0, to.1) {
            (from.0, to.1)
        } else {
            from
        }
    }

    /// Each squad's turn, before its soldiers walk: a steered or sent squad
    /// moves its anchor and orders its members to their places round it.
    /// Every squad's members are matched to their places first
    /// (`rank_squad`): round the anchor while it moves, round the point
    /// they all stand round while it rests.
    pub(super) fn squad_turn(&mut self, dt: f32) {
        let sc = self.s();
        for q in self.live_squads() {
            let sq = self.squads[q];
            let Some(mid) = self.middle(q) else { continue };
            let steering = sq.steer != (0.0, 0.0);
            if !steering && sq.dest.is_none() {
                self.squads[q].anchor = None;
                self.funnel_stop(q);
                // At rest: round the point every member was last sent to.
                let m = self.members(q);
                if let Some(&k0) = m.first() {
                    if let o @ (Order::Move(x, y) | Order::Fight(x, y)) = self.soldiers[k0].order {
                        if m.iter().all(|&k| self.soldiers[k].order == o) {
                            self.rank_squad(q, (x, y));
                        }
                    }
                }
                continue;
            }
            // Never round a crystal's middle (`Rules::crystal_solid`): a
            // formation there stands round its rim, every soldier on its
            // own side, 10 cells wide. Just past the near rim instead.
            if let Some(d) = sq.dest.filter(|&d| self.solid_at(d).is_some()) {
                self.squads[q].dest = Some(self.rim(d, sq.anchor.unwrap_or(mid)));
            }
            // A dest in rock or a wall (a right click on one) is the nearest
            // ground, or the anchor would wait at the rock's edge for good.
            if let Some(d) = self.squads[q].dest.filter(|&d| !self.world.open(d.0, d.1)) {
                self.squads[q].dest = Some(self.nearest_standable(d, GOAL_SEARCH * sc).unwrap_or(mid));
            }
            let sq = self.squads[q];
            let at = sq.anchor.unwrap_or(mid);
            let at = if self.solid_at(at).is_some() { self.rim(at, mid) } else { at };
            // An anchor off the ground (an old save, rock grown round it)
            // comes back to the nearest ground, or to the middle.
            let at = if self.world.open(at.0, at.1) { at } else { self.nearest_standable(at, GOAL_SEARCH * sc).unwrap_or(mid) };
            // Waits for the squad once it is the lead ahead of its middle,
            // but never to come back toward it: a squad pressed against
            // rock and steered away goes (before the bug sweep, 2026-11-14,
            // an anchor that ahead never moved again).
            // Measured from where the anchor stands when every member is in
            // its place, not from the members' middle: a wedge or a line
            // stands behind its anchor, so its middle sits a few cells back
            // even at rest. Measured from the middle, a big wedge (8 or more)
            // was "ahead" while it stood in its places and never moved again,
            // and a sent wedge never arrived (bug sweep, 2026-11-14: seed 4's
            // wedge stood 4.1 cells from its middle, the lead 4, for good).
            let rest = self.shape_off(q, at);
            let mid = (mid.0 - rest.0, mid.1 - rest.1);
            let gap = |p: (f32, f32)| (p.0 - mid.0).hypot(p.1 - mid.1);
            // A funnelling squad's head (`funnel`) waits only for the
            // front of its queue: its places reach back along the way it
            // came, and the middle of a squad queuing at a door lags its
            // places' middle by more than the lead while it all goes well.
            // Measured from the middle, the head stood at the mouth of the
            // door until the squad had squeezed up behind it (door 6 cells
            // wide, 24 soldiers: 0.6 s still at the mouth).
            let ahead = if self.columns[q].on {
                self.members(q).iter().map(|&k| (self.soldiers[k].x - at.0).hypot(self.soldiers[k].y - at.1)).fold(f32::MAX, f32::min) > LEAD * sc
            } else {
                gap(at) > LEAD * sc
            };
            let step = self.rules.speed * sc * dt * PACE * self.pace(q);
            let target = if steering { (at.0 + sq.steer.0 * 40.0 * sc, at.1 + sq.steer.1 * 40.0 * sc) } else { sq.dest.unwrap_or(at) };
            let a = if step > 0.0 { self.guide(at, target, step, !steering) } else { at };
            let a = if ahead && gap(a) >= gap(at) - 1e-3 { at } else { a };
            // Narrow ground ahead: a column (`funnel`). Looks a formation's
            // half width and a gap along the way, and starts the column at
            // the squad's front (its furthest member along the way).
            let a = if self.rules.funnel && step > 0.0 {
                let look = (self.formation_width(q) + self.gap()) * sc;
                let far = self.guide(at, target, look, !steering);
                let dir = unit((far.0 - at.0, far.1 - at.1)).unwrap_or(sq.face);
                let lead = self.members(q).iter().map(|&k| (self.soldiers[k].x - at.0) * dir.0 + (self.soldiers[k].y - at.1) * dir.1).fold(0.0, f32::max);
                let front = if self.columns[q].on || lead <= 0.0 { a } else { self.guide(at, target, lead.min(look), !steering) };
                self.funnel(q, at, a, far, front)
            } else {
                a
            };
            if let Some(d) = sq.dest.filter(|_| !steering) {
                if (d.0 - a.0).hypot(d.1 - a.1) < 1e-3 && gap(d) <= 2.0 * sc {
                    // Arrived: the members keep their places round it, or
                    // it starts what it has queued.
                    self.squads[q].dest = None;
                    self.funnel_stop(q);
                    self.squad_next(q);
                    continue;
                }
            }
            self.squads[q].anchor = Some(a);
            // Facing: the controlled squad faces the cursor (`Cmd::Face`);
            // a steered or sent one faces where it goes unless it is
            // volleying.
            if sq.fire.is_none() && Some(q) != self.active {
                if let Some(f) = unit((a.0 - at.0, a.1 - at.1)) {
                    self.squads[q].face = f;
                }
            }
            self.rank_squad(q, a);
            let o = if sq.fight && !steering { Order::Fight(a.0, a.1) } else { Order::Move(a.0, a.1) };
            // Moving as one, nobody works or stands guard.
            self.squads[q].work = None;
            for k in self.members(q) {
                let s = &mut self.soldiers[k];
                s.order = o;
                s.cover = false;
                // Steering drops what was queued; a squad sent somewhere
                // keeps it (Shift + a build while it walks: the build comes
                // once it is there; `Cmd::SquadOrder` drops the old queue).
                if steering {
                    s.queued = 0;
                }
                s.homing = false;
            }
        }
    }

    /// How far squad `q`'s middle stands from an anchor at `at` once every
    /// member is in its place (`Game::place_at`), fine cells. About zero
    /// for a blob or a ring; a wedge or a line stands behind its anchor. A
    /// place in rock or past a wall is the anchor itself, as its member's
    /// order is: the formation's own places alone (tried first) froze a
    /// wedge walking round rock with places in it (seed 3, 2 s).
    pub(super) fn shape_off(&self, q: usize, at: (f32, f32)) -> (f32, f32) {
        let m = self.members(q);
        if m.is_empty() {
            return (0.0, 0.0);
        }
        let (x, y) = m.iter().map(|&k| self.place_at(k, at)).fold((0.0, 0.0), |a, p| (a.0 + p.0 - at.0, a.1 + p.1 - at.1));
        (x / m.len() as f32, y / m.len() as f32)
    }

    /// The pace of squad `q` while it moves as one: its slowest member's
    /// wade (`Game::wade`), not counting those left past `STRAY`.
    pub(super) fn pace(&self, q: usize) -> f32 {
        let Some(mid) = self.middle(q) else { return 1.0 };
        let stray = STRAY * self.s();
        self.members(q).into_iter().filter(|&j| (self.soldiers[j].x - mid.0).hypot(self.soldiers[j].y - mid.1) <= stray).map(|j| self.wade(j)).fold(1.0, f32::min)
    }

    /// Whether soldier `k` fires a volley shot this step, and at what.
    pub(super) fn volley(&self, k: usize, t: u64) -> Option<(f32, f32)> {
        let sq = self.squads[self.soldiers[k].squad as usize];
        sq.fire.filter(|_| t >= sq.next_volley)
    }

    /// After the soldiers' shots: squads that volleyed this step wait for
    /// the next.
    pub(super) fn volley_done(&mut self, t: u64) {
        for q in self.live_squads() {
            let sq = self.squads[q];
            if sq.fire.is_some() && t >= sq.next_volley {
                // On the beat (a gap that is not whole frames would
                // otherwise round every wait up), unless it fell behind.
                let gap = self.shot_gap(q);
                let next = sq.next_volley + gap;
                self.squads[q].next_volley = if next > t { next } else { t + gap };
            }
        }
    }

    /// Ticks between squad `q`'s shots: its gun's (`Gun::rate`; the
    /// standard rifle's is `Rules::shot_every`), the controlled squad's
    /// `LeadRules::rate` times as often.
    pub(super) fn shot_gap(&self, q: usize) -> u64 {
        let every = self.gun(q).every() / if self.leads(q) { self.rules.lead.rate.max(0.01) } else { 1.0 };
        // The rounds' `Rate` cards (11-14): x (1 + rate_card) a card.
        let every = every / (1.0 + self.rules.rate_card * self.mods(q).rate as f32).max(0.01);
        self.secs(every).max(1)
    }

    /// Strike radius of squad `q`'s shots, fine cells.
    pub(super) fn strike_of(&self, q: usize) -> f32 {
        let lead = if self.leads(q) { self.rules.lead.strike } else { 1.0 };
        (self.rules.strike * self.s() * lead).min(squad::R_MAX)
    }

    /// Applies a squad command (`Game::apply`). True if it took effect.
    /// Moving, ordering, firing and facing a linked squad does the same to
    /// every squad linked with it: a move sends each to the point offset by
    /// where it stands in the link, so the link arrives as it walked.
    pub(super) fn squad_cmd(&mut self, c: Cmd) -> bool {
        let q = match c {
            Cmd::Steer(q, _) | Cmd::SquadOrder(q, _) | Cmd::SquadQueue(q, _) | Cmd::Fire(q, _) | Cmd::Face(q, _) | Cmd::Heal(q) if (q as usize) < SQUADS_MAX => q as usize,
            _ => return self.squad_one(c),
        };
        let group = self.linked(q);
        if group.len() < 2 {
            return self.squad_one(c);
        }
        let mids: Vec<Option<(f32, f32)>> = group.iter().map(|&r| self.middle(r)).collect();
        let at = mids.iter().flatten().fold((0.0f32, 0.0f32, 0.0f32), |a, m| (a.0 + m.0, a.1 + m.1, a.2 + 1.0));
        let centre = (at.0 / at.2.max(1.0), at.1 / at.2.max(1.0));
        let shift = |o: Order, m: Option<(f32, f32)>| match (o, m) {
            (Order::Move(x, y), Some(m)) => Order::Move(x + m.0 - centre.0, y + m.1 - centre.1),
            (Order::Fight(x, y), Some(m)) => Order::Fight(x + m.0 - centre.0, y + m.1 - centre.1),
            _ => o,
        };
        let mut any = false;
        for (&r, &m) in group.iter().zip(&mids) {
            let r8 = r as u8;
            let one = match c {
                Cmd::Steer(_, d) => Cmd::Steer(r8, d),
                Cmd::SquadOrder(_, o) => Cmd::SquadOrder(r8, shift(o, m)),
                Cmd::SquadQueue(_, o) => Cmd::SquadQueue(r8, shift(o, m)),
                Cmd::Fire(_, p) => Cmd::Fire(r8, p),
                Cmd::Face(_, p) => Cmd::Face(r8, p),
                Cmd::Heal(_) => Cmd::Heal(r8),
                _ => unreachable!(),
            };
            any |= self.squad_one(one);
        }
        any
    }

    /// Squad `q` takes order `o` (what `Cmd::SquadOrder` does, bar dropping
    /// its queue): a move or fight sends it there in formation; a harvest
    /// or a build sets it to work (`Game::cover_turn` calls its defenders).
    pub(super) fn send_squad(&mut self, q: usize, o: Order) {
        self.squads[q].steer = (0.0, 0.0);
        self.squads[q].work = None;
        self.squads[q].heal = false;
        self.squads[q].guard = None;
        match o {
            // A guard walks there as a squad, as a move does, and keeps the
            // zone once there (`Squad::guard`).
            Order::Guard(x, y) => {
                self.squads[q].guard = Some((x, y));
                self.squads[q].dest = Some((x, y));
                self.squads[q].fight = false;
                for k in self.members(q) {
                    self.soldiers[k].queued = 0;
                    self.soldiers[k].cover = false;
                }
            }
            Order::Move(x, y) | Order::Fight(x, y) => {
                self.squads[q].dest = Some((x, y));
                self.squads[q].fight = matches!(o, Order::Fight(..));
                for k in self.members(q) {
                    self.soldiers[k].queued = 0;
                    self.soldiers[k].cover = false;
                }
            }
            _ => {
                self.squads[q].dest = None;
                for k in self.members(q) {
                    let s = &mut self.soldiers[k];
                    s.order = o;
                    s.queued = 0;
                    s.homing = false;
                    s.cover = false;
                }
                if matches!(o, Order::Clear(..) | Order::Build(_)) {
                    self.squads[q].work = Some(o);
                    self.cover_squad(q);
                }
            }
        }
    }

    fn squad_one(&mut self, c: Cmd) -> bool {
        let ok = |q: u8| (q as usize) < SQUADS_MAX;
        match c {
            Cmd::Steer(q, d) if ok(q) => {
                let q = q as usize;
                let was = self.squads[q].steer;
                let d = unit(d).unwrap_or((0.0, 0.0));
                self.squads[q].steer = d;
                if d != (0.0, 0.0) {
                    self.squads[q].dest = None;
                    self.squads[q].fight = false;
                    self.squads[q].heal = false;
                    self.squads[q].guard = None;
                } else if was != (0.0, 0.0) {
                    // Let go: settle round where it stands.
                    self.squads[q].dest = self.middle(q);
                }
                true
            }
            Cmd::SquadOrder(q, o) if ok(q) => {
                let q = q as usize;
                self.squads[q].queued = 0;
                self.send_squad(q, o);
                true
            }
            Cmd::SquadQueue(q, o) if ok(q) => {
                let q = q as usize;
                if self.members(q).is_empty() {
                    return false;
                }
                if self.idle(q) {
                    self.send_squad(q, o);
                    return true;
                }
                let sq = &mut self.squads[q];
                if sq.queued as usize >= SQUAD_QUEUE {
                    return false;
                }
                sq.queue[sq.queued as usize] = o;
                sq.queued += 1;
                true
            }
            Cmd::Link(a, b) if ok(a) && ok(b) && a != b => {
                let (a, b) = (a as usize, b as usize);
                if self.members(a).is_empty() || self.members(b).is_empty() {
                    return false;
                }
                let both: Vec<usize> = self.linked(a).into_iter().chain(self.linked(b)).collect();
                let g = *both.iter().min().unwrap() as u8;
                for r in both {
                    self.squads[r].link = g;
                }
                true
            }
            Cmd::Unlink(q) if ok(q) => {
                let q = q as usize;
                let group = self.linked(q);
                if group.len() < 2 {
                    return false;
                }
                self.squads[q].link = NO_LINK;
                let rest: Vec<usize> = group.into_iter().filter(|&r| r != q).collect();
                // The rest stay linked through their lowest id; one left
                // alone is not linked.
                let g = if rest.len() < 2 { NO_LINK } else { rest[0] as u8 };
                for r in rest {
                    self.squads[r].link = g;
                }
                true
            }
            Cmd::Heal(q) if ok(q) => {
                // Stop where it stands and heal: no work, no guards, nothing
                // queued.
                let q = q as usize;
                let m = self.members(q);
                if m.is_empty() {
                    return false;
                }
                let sq = &mut self.squads[q];
                (sq.steer, sq.dest, sq.work, sq.queued, sq.heal) = ((0.0, 0.0), None, None, 0, true);
                for k in m {
                    let s = &mut self.soldiers[k];
                    s.order = Order::Hold;
                    s.cover = false;
                    s.queued = 0;
                    s.homing = false;
                }
                true
            }
            Cmd::Cover(q, c) if ok(q) => {
                self.squads[q as usize].cover = c;
                if self.squads[q as usize].work.is_some() {
                    self.cover_squad(q as usize);
                }
                true
            }
            Cmd::Fire(q, at) if ok(q) => {
                let q = q as usize;
                let t = self.world.ticks();
                let sq = &mut self.squads[q];
                if sq.fire.is_none() && at.is_some() {
                    // A press fires at once if the guns are ready: the
                    // clock only waits out the last volley's gap.
                    sq.next_volley = sq.next_volley.max(t);
                }
                sq.fire = at;
                if let Some(p) = at {
                    self.face_to(q, p);
                }
                true
            }
            Cmd::Face(q, p) if ok(q) => {
                self.face_to(q as usize, p);
                true
            }
            Cmd::Split(q, p) if ok(q) => self.split(q as usize, p).is_some(),
            Cmd::Join(k, q) if ok(q) && k < self.soldiers.len() && self.soldiers[k].alive => {
                let from = self.soldiers[k].squad as usize;
                let q = q as usize;
                if self.members(q).is_empty() && from != q {
                    // A new squad starts as the one it leaves.
                    self.squads[q] = self.squads[from].offshoot();
                }
                self.soldiers[k].squad = q as u8;
                true
            }
            Cmd::Merge(a, b) if ok(a) && ok(b) && a != b => {
                let ms = self.members(b as usize);
                if ms.is_empty() || self.members(a as usize).is_empty() {
                    return false;
                }
                for k in ms {
                    self.soldiers[k].squad = a;
                }
                // A gun and cards are not lost to a merge: a squad with
                // none of its own takes the joining squad's.
                let (ga, gb) = (self.squads[a as usize], self.squads[b as usize]);
                self.squads[a as usize].gun = ga.gun.or(gb.gun);
                self.squads[a as usize].deck = ga.deck.or(gb.deck);
                if self.active == Some(b as usize) {
                    self.active = Some(a as usize);
                }
                // Gather the joined squad into one formation.
                if self.squads[a as usize].steer == (0.0, 0.0) && self.squads[a as usize].dest.is_none() {
                    self.squads[a as usize].dest = self.middle(a as usize);
                }
                true
            }
            Cmd::Active(q) => {
                // u8::MAX lets go of every squad (owner 2026-11-15 02:58Z:
                // deselect, so every squad runs on its own). The one let go
                // stops steering and volleying, as on a switch, and keeps
                // its other orders; it no longer faces the cursor.
                if q == u8::MAX {
                    if let Some(old) = self.active.take() {
                        for r in self.linked(old) {
                            self.squads[r].fire = None;
                            if self.squads[r].steer != (0.0, 0.0) {
                                self.squads[r].steer = (0.0, 0.0);
                                self.squads[r].dest = self.middle(r);
                            }
                        }
                    }
                    return true;
                }
                if (q as usize) >= SQUADS_MAX || self.members(q as usize).is_empty() {
                    return false;
                }
                // A link is commanded as one (owner 15:49Z: "When they are
                // linked, I cannot choose who i am commanding between them.
                // I only command all of them").
                // A dead controlled squad holds nothing: its link still
                // names the live ones, and refusing them left nothing to
                // command when the controlled squad of a link was wiped out.
                if self.active.is_some_and(|a| !self.members(a).is_empty() && self.linked(a).contains(&(q as usize))) {
                    return false;
                }
                if let Some(old) = self.active.filter(|&o| o != q as usize) {
                    // The one let go stops where it is and stops volleying,
                    // and so does its link, unless the new one is in it.
                    let keep = self.linked(q as usize);
                    for old in self.linked(old).into_iter().filter(|r| !keep.contains(r)) {
                        self.squads[old].fire = None;
                        if self.squads[old].steer != (0.0, 0.0) {
                            self.squads[old].steer = (0.0, 0.0);
                            self.squads[old].dest = self.middle(old);
                        }
                    }
                }
                self.active = Some(q as usize);
                true
            }
            Cmd::Formation(q, f) if ok(q) => {
                self.squads[q as usize].formation = f;
                true
            }
            _ => false,
        }
    }

    fn face_to(&mut self, q: usize, p: (f32, f32)) {
        if let Some(m) = self.middle(q) {
            if let Some(f) = unit((p.0 - m.0, p.1 - m.1)) {
                self.squads[q].face = f;
            }
        }
    }

    /// Splits squad `q` in two: the half of its members nearer `p` (at least
    /// one, and only if it has two) become a new squad, which is controlled
    /// next if `q` was. Returns the new squad.
    pub fn split(&mut self, q: usize, p: (f32, f32)) -> Option<usize> {
        let mut m = self.members(q);
        if m.len() < 2 {
            return None;
        }
        let n = self.free_squad()?;
        let d = |s: &Soldier| (s.x - p.0).hypot(s.y - p.1);
        m.sort_by(|&a, &b| d(&self.soldiers[a]).total_cmp(&d(&self.soldiers[b])).then(a.cmp(&b)));
        self.squads[n] = self.squads[q].offshoot();
        self.squads[q].anchor = None;
        for &k in &m[..m.len() / 2] {
            self.soldiers[k].squad = n as u8;
        }
        // Both halves gather round their own middles.
        for r in [q, n] {
            if self.squads[r].steer == (0.0, 0.0) {
                self.squads[r].dest = self.middle(r);
            }
        }
        if self.active == Some(q) {
            self.active = Some(n);
            self.squads[q].steer = (0.0, 0.0);
            self.squads[q].fire = None;
            self.squads[q].dest = self.middle(q);
        }
        Some(n)
    }
}
