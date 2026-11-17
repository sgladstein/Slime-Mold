//! **Necks**: how much slime one thin cut would leave without food (the
//! gun-card plan's cut count, 2026-11-17, before any Cut card is built:
//! `Reports/gun-system-review-2026-11-16/README.md`, "Cut goes last").
//! The owner (11-16 03:44Z) rarely finds "a large group of slime that can
//! be separated" because it sits on crystals all over; this measures it.
//!
//! On a grid of walker counts (one cell a dish cell), the slime is every
//! cell with a walker. A part is cuttable at width `2r` when it is joined
//! to food only through necks no wider than that: the slime is eroded by
//! `r` (a cell stays if every cell within `r`, Chebyshev, is slime), so a
//! neck of `2r` cells or fewer vanishes and the bodies on either side fall
//! apart; each body takes back the slime nearest it (a breadth-first grow
//! through the slime from all bodies at once), and a body whose share
//! touches no food cell, though the whole slime it sits in does, is a part
//! a cut there would starve. Slime already cut off from food is counted
//! apart (`unfed`). A part tied to food by two thin necks counts too,
//! though it takes two cuts: an upper bound on what one cut gives.
//!
//! **Pinholes are filled first.** A sheet of slime is speckled with empty
//! cells, and eroding by 2 through speckle broke wide sheets into bodies
//! that looked cuttable: in the first census the Sleeper's planet showed
//! 30-56% of its slime behind necks 4 wide, most of it a sheet joined to
//! its crystals along a long front (seen on the map). So an empty cell on
//! open ground with 5 or more slime cells of its 8 counts as slime for the
//! shape (it adds no walkers), twice over; rock is never filled, so a gap
//! in the rock stays a gap.

/// One count: walkers in all, in slime already without food, and in the
/// parts thin necks alone tie to food (all of them, the largest, and how
/// many hold at least `big` walkers).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Necks {
    pub total: u32,
    pub unfed: u32,
    pub cuttable: u32,
    pub largest: u32,
    pub big_parts: u32,
}

/// Count the necks of width `2r` on `grid` (`w` x `h` walker counts) with
/// `food` marking the cells that feed (a crystal with charge and the ring
/// round it) and `open` the cells slime can stand on (pinholes are filled
/// only there; all false leaves them). Parts of `big` walkers or more
/// count in `big_parts`.
pub fn necks(grid: &[u32], w: usize, h: usize, food: &[bool], open: &[bool], r: usize, big: u32) -> Necks {
    necks_map(grid, w, h, food, open, r, big).0
}

/// What a cell is, in [`necks_map`]'s picture.
pub const BARE: u8 = 0;
/// Slime that food feeds and no thin cut parts off.
pub const FED: u8 = 1;
/// Slime already without food.
pub const UNFED: u8 = 2;
/// Slime in a part a cut of width `2r` would leave without food.
pub const CUT: u8 = 3;

/// [`necks`] and each cell's kind ([`BARE`], [`FED`], [`UNFED`] or
/// [`CUT`]), for a picture.
pub fn necks_map(grid: &[u32], w: usize, h: usize, food: &[bool], open: &[bool], r: usize, big: u32) -> (Necks, Vec<u8>) {
    let n = w * h;
    assert!(grid.len() == n && food.len() == n && open.len() == n);
    let mut slime: Vec<bool> = grid.iter().map(|&c| c > 0).collect();
    for _ in 0..2 {
        slime = (0..n).map(|i| slime[i] || (open[i] && around(i, w, h).filter(|&k| slime[k]).count() >= 5)).collect();
    }
    let total = grid.iter().sum();
    // Slime joined to food at all (8-neighbours).
    let fed = flood(&slime, w, h, |i| slime[i] && food[i]);
    let unfed = (0..n).filter(|&i| slime[i] && !fed[i]).map(|i| grid[i]).sum();
    // The bodies left when necks of 2r or fewer cells go.
    let core: Vec<bool> = (0..n)
        .map(|i| {
            let (x, y) = ((i % w) as i64, (i / w) as i64);
            fed[i] && (-(r as i64)..=r as i64).all(|dy| (-(r as i64)..=r as i64).all(|dx| at(&fed, w, h, x + dx, y + dy)))
        })
        .collect();
    let mut label = vec![u32::MAX; n];
    let mut bodies = 0u32;
    for i in 0..n {
        if core[i] && label[i] == u32::MAX {
            let mut stack = vec![i];
            label[i] = bodies;
            while let Some(j) = stack.pop() {
                for k in around(j, w, h) {
                    if core[k] && label[k] == u32::MAX {
                        label[k] = bodies;
                        stack.push(k);
                    }
                }
            }
            bodies += 1;
        }
    }
    // Each body takes back the fed slime nearest it, all growing at once.
    let mut queue: std::collections::VecDeque<usize> = (0..n).filter(|&i| label[i] != u32::MAX).collect();
    while let Some(j) = queue.pop_front() {
        for k in around(j, w, h) {
            if fed[k] && label[k] == u32::MAX {
                label[k] = label[j];
                queue.push_back(k);
            }
        }
    }
    let b = bodies as usize;
    let (mut mass, mut feeds) = (vec![0u32; b], vec![false; b]);
    for i in 0..n {
        if label[i] != u32::MAX {
            let l = label[i] as usize;
            mass[l] += grid[i];
            feeds[l] |= food[i];
        }
    }
    let parts: Vec<u32> = (0..b).filter(|&l| !feeds[l]).map(|l| mass[l]).collect();
    let kind = (0..n)
        .map(|i| match label[i] {
            _ if !slime[i] => BARE,
            _ if !fed[i] => UNFED,
            l if l != u32::MAX && !feeds[l as usize] => CUT,
            _ => FED,
        })
        .collect();
    let counts = Necks {
        total,
        unfed,
        cuttable: parts.iter().sum(),
        largest: parts.iter().copied().max().unwrap_or(0),
        big_parts: parts.iter().filter(|&&m| m >= big).count() as u32,
    };
    (counts, kind)
}

fn at(v: &[bool], w: usize, h: usize, x: i64, y: i64) -> bool {
    x >= 0 && y >= 0 && (x as usize) < w && (y as usize) < h && v[y as usize * w + x as usize]
}

fn around(i: usize, w: usize, h: usize) -> impl Iterator<Item = usize> {
    let (x, y) = ((i % w) as i64, (i / w) as i64);
    (-1i64..=1).flat_map(move |dy| (-1i64..=1).map(move |dx| (x + dx, y + dy))).filter(move |&(a, b)| (a, b) != (x, y) && a >= 0 && b >= 0 && (a as usize) < w && (b as usize) < h).map(move |(a, b)| b as usize * w + a as usize)
}

/// The cells of `on` reachable (8-neighbours) from the cells where `seed`.
fn flood(on: &[bool], w: usize, h: usize, seed: impl Fn(usize) -> bool) -> Vec<bool> {
    let mut out = vec![false; on.len()];
    let mut stack: Vec<usize> = (0..on.len()).filter(|&i| seed(i)).collect();
    for &i in &stack {
        out[i] = true;
    }
    while let Some(j) = stack.pop() {
        for k in around(j, w, h) {
            if on[k] && !out[k] {
                out[k] = true;
                stack.push(k);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two 10 x 10 blobs, one on food, joined by a neck `width` cells wide
    /// and 6 long, and a third blob off by itself: at r 1 a neck of 2 parts
    /// the far blob (100 cells) off, a neck of 4 does not; at r 2 it does.
    /// The lone blob is unfed whatever r.
    #[test]
    fn a_blob_behind_a_thin_neck_is_cuttable_and_a_thick_one_is_not() {
        let (w, h) = (40, 30);
        let scene = |width: usize| {
            let mut g = vec![0u32; w * h];
            let mut food = vec![false; w * h];
            for y in 5..15 {
                for x in 2..12 {
                    g[y * w + x] = 1;
                    food[y * w + x] = x < 4;
                }
                for x in 18..28 {
                    g[y * w + x] = 1;
                }
            }
            for y in 8..8 + width {
                for x in 12..18 {
                    g[y * w + x] = 1;
                }
            }
            for y in 20..25 {
                for x in 32..37 {
                    g[y * w + x] = 2;
                }
            }
            (g, food)
        };
        let open = vec![true; w * h];
        let (g, food) = scene(2);
        let (n, kind) = necks_map(&g, w, h, &food, &open, 1, 50);
        assert_eq!((n.total, n.unfed), (200 + 12 + 50, 50));
        assert!(n.cuttable >= 100 && n.cuttable <= 112 && n.largest == n.cuttable && n.big_parts == 1, "{n:?}");
        // The map marks the far blob, not the fed one or the lone one.
        assert_eq!((kind[10 * w + 22], kind[10 * w + 5], kind[22 * w + 34], kind[0]), (CUT, FED, UNFED, BARE));
        assert_eq!(kind.iter().zip(&g).filter(|(k, _)| **k == CUT).map(|(_, c)| c).sum::<u32>(), n.cuttable);
        let (g, food) = scene(4);
        let thick = necks(&g, w, h, &food, &open, 1, 50);
        assert_eq!((thick.cuttable, thick.big_parts, thick.unfed), (0, 0, 50), "{thick:?}");
        let wide = necks(&g, w, h, &food, &open, 2, 50);
        assert!(wide.cuttable >= 100 && wide.big_parts == 1, "{wide:?}");
        // No food anywhere: all of it is unfed, nothing to cut.
        let none = necks(&g, w, h, &vec![false; w * h], &open, 1, 50);
        assert_eq!((none.unfed, none.cuttable), (none.total, 0));
    }

    /// A wide sheet on food with a seam of lone empty cells down its middle
    /// (every other cell): nothing to cut once pinholes are filled; left
    /// open, eroding breaks the sheet at the seam into a "part", the first
    /// census's artefact. Behind a wall of rock with one gap 2 wide, the
    /// far side is cut.
    #[test]
    fn pinholes_in_a_sheet_are_not_necks_but_a_gap_in_rock_is() {
        let (w, h) = (40, 24);
        let mut g = vec![0u32; w * h];
        let mut food = vec![false; w * h];
        let mut open = vec![true; w * h];
        for y in 2..20 {
            for x in 2..36 {
                g[y * w + x] = u32::from(x != 18 || y % 2 == 1);
                food[y * w + x] = x < 4;
            }
        }
        for r in [1, 2] {
            let filled = necks(&g, w, h, &food, &open, r, 50);
            assert_eq!(filled.cuttable, 0, "{filled:?}");
        }
        let raw = necks(&g, w, h, &food, &vec![false; w * h], 1, 50);
        assert!(raw.cuttable > 200, "{raw:?}");
        // A rock wall down x 20, open only at y 10..12: the slime either
        // side of it stops there, and nothing fills the rock.
        for y in 0..h {
            if !(10..12).contains(&y) {
                g[y * w + 20] = 0;
                open[y * w + 20] = false;
            }
        }
        let walled = necks(&g, w, h, &food, &open, 1, 50);
        assert!(walled.cuttable > 200 && walled.big_parts == 1, "{walled:?}");
    }
}
