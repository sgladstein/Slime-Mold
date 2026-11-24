//! **Autocast** (gun step E4, 2026-11-24; the gun-system plan's "each body
//! has a free autocast switch with one rule", Reports/gun-system-review-
//! 2026-11-16, and Direction 11-24 06:44Z: off per body, its state on the
//! device button).
//!
//! The rule is the plan's first: **defend against a surge in reach**. Once
//! a game second, each squad whose device slot is ready looks for the
//! thickest slime within its body's reach that is at least
//! `KitRules::auto_gap` (12) dish cells from every soldier of the squad,
//! and if `Kit::mine_trigger` walkers or more stand within a blast's
//! radius of it (the crowd that sets a mine off), the first body on
//! autocast goes off there through the squad's slot (`Game::device`), as
//! a click would: a Grenade or a Canister on that spot, a Mine on it, a
//! Line laid `auto_line` (16) cells across the way from the squad to it.
//! Order: Grenade, Canister, Mine, Line. The gap keeps a Torch Grenade
//! from burning its own squad (a squad thick in slime holds instead, and
//! the player throws).
//!
//! It reads the mould grid (`World::mould_grid`, a dish cell a square),
//! which the GPU world reads back with each frame; a world without one
//! never autocasts. Cost: only with a body on autocast and a slot ready,
//! once a game second, at most the 65 x 65 squares in a 32-cell reach,
//! each with slime summing the 13 x 13 squares of a blast's 6-cell
//! radius: under 750,000 additions a squad, about half a millisecond.

use super::*;
use crate::mould::kit::Tool;

/// The tools autocast tries, in order (`kit::TOOLS` indices): Grenade
/// (blast), Canister (gas), Mine, Line (cord).
const ORDER: [usize; 4] = [0, 4, 1, 2];

impl<W: World> Game<W> {
    /// Once a game second, each squad's ready slot sets off its first
    /// body on autocast at a surge in reach (module doc).
    pub(super) fn autocast_turn(&mut self) {
        if self.autocast == 0 || !self.rules.squads {
            return;
        }
        self.autocast_clock -= self.rules.frame_ticks as f32 / self.rules.tps.max(1) as f32;
        if self.autocast_clock > 0.0 {
            return;
        }
        self.autocast_clock = 1.0;
        let s = self.world.scale();
        let bin = (s as usize).max(1);
        let Some(grid) = self.world.mould_grid(bin) else { return };
        let gw = self.world.size().0.div_ceil(bin);
        let now = self.world.ticks();
        let kr = self.kit.rules;
        let crowd = self.kit.mine_trigger(self);
        let on = self.autocast;
        for q in self.live_squads() {
            if self.squads[q].device_at > now {
                continue;
            }
            let members = self.members(q);
            let Some(m) = self.middle(q) else { continue };
            for t in ORDER.into_iter().filter(|&t| on & (1 << t) != 0) {
                let reach = if kr.reaches[t] > 0.0 { kr.reaches[t] } else { kr.reach };
                let Some(at) = surge(&grid, gw, bin, &self.soldiers, &members, m, reach * s, kr.auto_gap * s, kr.blast_r * s, crowd) else { continue };
                let tool = match t {
                    0 => Tool::Blast(at),
                    4 => Tool::Gas(at),
                    1 => Tool::Mine(at),
                    _ => {
                        // Across the way in: square to the line from the
                        // squad's middle to the surge.
                        let (dx, dy) = (at.0 - m.0, at.1 - m.1);
                        let d = dx.hypot(dy).max(1e-3);
                        let h = 0.5 * kr.auto_line.min(kr.cord_max) * s;
                        let (nx, ny) = (-dy / d * h, dx / d * h);
                        Tool::Cord((at.0 + nx, at.1 + ny), (at.0 - nx, at.1 - ny))
                    }
                };
                if self.device(q, tool) {
                    self.autocasts += 1;
                    break;
                }
            }
        }
    }
}

/// The square, fine cells, with the most walkers within `r` of it, of
/// those within `reach` of the squad's middle `m` and at least `gap` from
/// every soldier in `members`, if `crowd` or more stand there.
#[allow(clippy::too_many_arguments)]
fn surge(grid: &[u32], gw: usize, bin: usize, soldiers: &[Soldier], members: &[usize], m: (f32, f32), reach: f32, gap: f32, r: f32, crowd: u32) -> Option<(f32, f32)> {
    let b = bin as f32;
    let gh = grid.len() / gw.max(1);
    let cell = |v: f32, n: usize| ((v / b).floor().max(0.0) as usize).min(n.saturating_sub(1));
    let (cx, cy) = (cell(m.0, gw), cell(m.1, gh));
    let span = (reach / b).ceil() as usize;
    let rr = (r / b).ceil() as isize;
    let centre = |i: usize, j: usize| ((i as f32 + 0.5) * b, (j as f32 + 0.5) * b);
    let mut best: Option<((f32, f32), u32)> = None;
    for j in cy.saturating_sub(span)..(cy + span + 1).min(gh) {
        for i in cx.saturating_sub(span)..(cx + span + 1).min(gw) {
            // The square itself must hold slime: a surge is where it is.
            if grid[j * gw + i] == 0 {
                continue;
            }
            let p = centre(i, j);
            if (p.0 - m.0).hypot(p.1 - m.1) > reach || members.iter().any(|&k| (soldiers[k].x - p.0).hypot(soldiers[k].y - p.1) < gap) {
                continue;
            }
            let mut n = 0u32;
            for dj in -rr..=rr {
                for di in -rr..=rr {
                    let (x, y) = (i as isize + di, j as isize + dj);
                    if x < 0 || y < 0 || x as usize >= gw || y as usize >= gh || ((di * di + dj * dj) as f32) * b * b > r * r {
                        continue;
                    }
                    n += grid[y as usize * gw + x as usize];
                }
            }
            if n >= crowd && best.is_none_or(|(_, v)| n > v) {
                best = Some((p, n));
            }
        }
    }
    best.map(|(p, _)| p)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The surge is the thickest square in reach, but never within the gap
    /// of a soldier of the squad: slime piled on the squad itself is left
    /// to the player (a Torch Grenade there burns it), and slime under the
    /// crowd is no surge.
    #[test]
    fn the_surge_is_the_thickest_slime_in_reach_clear_of_the_squad() {
        let gw = 60;
        let mut grid = vec![0u32; gw * 40];
        grid[20 * gw + 12] = 90; // on the squad
        grid[20 * gw + 30] = 50; // 18 cells off
        grid[20 * gw + 50] = 80; // out of reach
        let mut g = Game::new(race_world(3, 1, 200), Rules::default(), 3);
        assert!(g.buy());
        let soldiers = vec![Soldier { x: 10.5, y: 20.5, ..g.soldiers[0] }];
        let at = |crowd| surge(&grid, gw, 1, &soldiers, &[0], (10.5, 20.5), 24.0, 12.0, 4.0, crowd);
        assert_eq!(at(40), Some((30.5, 20.5)));
        assert_eq!(at(60), None, "too thin to be a surge");
    }

    /// **Autocast** on the race world: a squad walked up to the slime,
    /// its Grenade on autocast (`Cmd::Auto(1)`, which a run's save line
    /// carries as `auto 1`), throws on its own; with it off it throws
    /// nothing in the same game.
    #[test]
    fn a_body_on_autocast_goes_off_on_its_own() {
        assert_eq!(Cmd::parse(&Cmd::Auto(17).to_line()), Some(Cmd::Auto(17)));
        let play = |mask: u8| {
            let rules = Rules { squads: true, larder: false, seek: false, overrun: 0.0, max_squad: 8, start_bank: 1e9, ..Rules::default() };
            let mut g = Game::new(race_world(3, 1, 2_000), rules, 3);
            while g.alive() < 6 && g.buy() {}
            for k in 0..g.soldiers.len() {
                g.apply(Cmd::Join(k, 0));
            }
            assert!(g.apply(Cmd::Auto(mask)));
            g.step();
            let s = g.world.scale();
            let bin = s as usize;
            let grid = g.world.mould_grid(bin).unwrap();
            let gw = g.world.size().0.div_ceil(bin);
            let (i, _) = grid.iter().enumerate().max_by_key(|&(_, &n)| n).unwrap();
            let slime = (((i % gw) as f32 + 0.5) * s, ((i / gw) as f32 + 0.5) * s);
            let m = g.middle(0).unwrap();
            let d = (slime.0 - m.0).hypot(slime.1 - m.1).max(1.0);
            let stop = 20.0 * s;
            let to = (slime.0 - (slime.0 - m.0) / d * stop, slime.1 - (slime.1 - m.1) / d * stop);
            g.apply(Cmd::SquadOrder(0, Order::Move(to.0, to.1)));
            while g.time() < 40.0 && g.end.is_none() && g.autocasts == 0 {
                g.step();
            }
            (g.autocasts, g.kit.uses[0])
        };
        let (on, off) = (play(1), play(0));
        eprintln!("autocast on: {on:?}, off: {off:?} (autocasts, blasts)");
        assert!(on.0 > 0 && on.1 == on.0, "the Grenade went off on its own");
        assert_eq!(off, (0, 0), "nothing goes off with autocast off");
    }
}
