//! The shapes on the shapes page (`Reports/slime-questions-2026-11-12/`):
//! each one's start walkers at scale 2 and its engine keys. Moved here from
//! the larder-world window (2026-11-14, cloud plan D1) so the census
//! (`examples/larderworld.rs`) plays the window's shapes, C by default.
//! The window applies the keys through `gpu::set`; the census through
//! `BigParams` and `tiers::set_key`.

/// The engine keys of a shape on the shapes page (`shape=`), with its start
/// walkers. The films held one walker a cell with `capacity=50000`.
pub fn shape_keys(name: &str) -> Option<(usize, &'static [(&'static str, &'static str)])> {
    const DENSE: [(&str, &str); 2] = [("upkeep", "0.0002"), ("capacity", "50000")];
    // C shares its crystals' food through the body (`energy_flow`, the one
    // body thread's 0.05; 2026-11-13, the slime-enemy thread). Why: the
    // owner, 22:22Z, playing C: "I cannot tell if it is getting stronger by
    // holding the crystals." It was: with crystals that feed nothing C dies
    // by 2:00 (CPU, 4 seeds). But what it ate stayed with the walkers on the
    // crystals, whose cells were full, so it bred few walkers and the
    // screen showed nothing (`examples/fedprobe.rs`). With the food shared,
    // walkers along the network breed where there is room, and the body
    // swells round the crystals it holds: on Fork seed 1 (farm hand, the
    // window on software Vulkan) 45,237 walkers at 5:00 against 13,956, and
    // mould on 17.1% of the map's screen against 7.0% (1:45 8.9% against
    // 4.6%). Tried first: fed walkers packing two a cell
    // (`fed_crowd` 1, deleted 11-17): 21,053 walkers but 7.6% of the screen, as
    // the look's thickness is full at one walker a cell.
    // A part cut off from every crystal withers (`BigParams::wither` 0.01,
    // 2026-11-13). Why: with the food shared, a cut-off part pooled its
    // energy, stayed whole for 75 s or more and then died at once, so a cut
    // read as nothing happening. With withering a part that has heard no
    // news of food for 5 s thins steadily while the body joined to its
    // crystals keeps every walker (uncut C the same to the walker on 4 of
    // 4 CPU seeds). The owner, 11-13 15:51Z, on the blind A/B/C: "B is the
    // best but still too fast" (B was 0.02, half the piece gone at about
    // 25 s), then 15:53Z: "Lets just double it and call it good. I will
    // have to playtest." At 0.01 a held ring cut on the CPU leaves 74% of
    // the piece at 20 s, 41% at 45 s and 15% at 90 s
    // (Reports/fed-mould-2026-11-13/README.md, *A cut-off part withers*).
    // A cut-off part also panics (`BigParams::panic` 2, 2026-11-13). The
    // owner, 15:57Z: "It shouldn't have a stronger attack but it should act
    // more like a scared animal. Either run away back to the main body or
    // go right for the soliders. It should be more reckless." A walker with
    // no news for 5 s heads straight for the nearest unit within reach (5
    // cells past the gun's range, `Rules::panic_margin`, since 11-14: 29),
    // or else the nearest crystal with food, creeping at half its pace for
    // 1.5 s (the tell; `panic_creep` 0.5, the BigParams default since the
    // game designer's review of the last-stand film, 11-13) and then twice
    // its pace, for 0.5 s at a unit before it creeps again (`panic_dash`
    // 15, so a squad that pulls back in time gets clear); the bite is
    // unchanged. On the CPU race world
    // the scores are unchanged (smoke, 4 seeds x 3 hands); in the window
    // (Fork seed 1, farm) the squad has 30 soldiers at 2:30 against 33, as
    // bits its own shots knock off charge it and are shot (#198). Before
    // #198 soldiers could not shoot the bits on them and it fell to 1.
    // `panic=0` turns it off. The report's *A cut-off part panics* has
    // the films.
    // A fed walker that cannot divide thickens (`BigParams::thicken` 2,
    // 2026-11-15). The owner, 11-15 08:34Z: the slime should "keep getting
    // stronger by holding onto its crystals and getting more of them (which
    // increases how fast it multiplies)". Without it C fills its 50,000
    // slots by 3:00 on the CPU race world (seed 1) and then gets no
    // stronger, though it eats three times its upkeep. With it, a walker
    // that has banked twice a birth's energy (it had no slot or no room to
    // divide into) turns condensed where it stands: two lives, twice the
    // bite and upkeep, drawn bronze. Holding every crystal, C's mass is
    // 90,739 at 7:00 and still rising (50,000 without); held to its 4
    // nearest crystals, 30,338 (Reports/thicken-2026-11-15/README.md).
    // **Off since 2026-11-16**: the owner, 04:58Z, "Remove the bronze,
    // keep the specks"; the thickened bronze went from C and the run, and
    // the rule was deleted 11-17 (Reports/dead-ends.md).
    // The body surges (`BigParams::surge_every` 900, 2026-11-15). The
    // owner, 11-15 08:30Z: some slimes "just hang at their home crystal
    // ... So their is no rush ... no need to defend", and the enemy needs
    // to "increase in challenge ... especially when your guns get
    // stronger". Every 30 s the slime within the gun's range plus 5 of a
    // soldier or building creeps toward it for 1.5 s, then lunges for 1 s
    // at twice its pace. In the window (Fork, squad mode, the edge hand,
    // seed 1) the first surge, at 31.5 s, took 2 of 9 soldiers, where
    // without it all 9 stood at 34 s; the second, at 61.5 s, wiped the
    // last 3. With the scripted squads hand and four pumps, the pumps fed
    // 1,070 food against 1,252 (Reports/surge-2026-11-15/README.md).
    // `surge=0` turns it off. Since food-driven surges (later on 11-15) the
    // game starts each surge once the slime has eaten `Rules::surge_food`
    // (150 food) from its crystals, at least 10 s after the last, and
    // `surge` is the longest wait: 1800 steps, 60 s. On the Fork the slime
    // eats 2.3 food a second at the start, 6.9 at a minute and 11 at three
    // as it spreads over its crystals, so the first surge comes at about
    // 43 s and the gaps shrink to about 14 s; a slime cut off its crystals
    // waits the whole minute.
    // C flinches from fire (`BigParams::flinch` 0.9, 2026-11-15, the
    // whole-game review's W19: "A hit should make the body flinch"): a
    // shot's alarm is ground the body backs off for a moment, so a ring
    // opens round a squad that keeps firing and closes when it stops. A
    // tier with an alarm of its own keeps it (the Swarmer still floods to
    // a wound). On planet 1, squad hand, seeds 1-5 to 2:00: soldiers left
    // 1, 5, 0, 0, 7 against 0, 8, 3, 1, 7 without it, walkers killed more
    // on 3 of 5, so no steady change in who wins; the old flee (keep
    // 0.98) failed the census gate in 2026-11-12's rules
    // (`Reports/flinch-2026-11-15/README.md`).
    // C resists the germ (`BigParams::germ_resist` 10, 2026-11-17, the
    // game designer's call, 05:40Z: the plain mould, the Sprawler and the
    // Trunk should lose 25-35% to one germ round in 20 s). With the
    // rounds' upkeep of 10 one round on the Fork lobe (race world, seed 0)
    // took 36% of C at 1, 35% at 3, 34% at 6, 33% at 8, 32% at 10 and 20%
    // at 13.5. (13.5 was first picked from runs that carried the GPU
    // world's middle-seeding bug, 32.5% there.) A tier that sets its own
    // (the Sporer, Drifter and Sleeper below it, the Swarmer and Grudge
    // 15) wins, as every tier key does. It also weakens the toolkit's
    // thrown germ on C (`Reports/gun-route-2026-11-17/`).
    const C: [(&str, &str); 8] = [DENSE[0], DENSE[1], ("energy_flow", "0.05"), ("wither", "0.01"), ("panic", "2"), ("surge", "1800"), ("flinch", "0.9"), ("germ_resist", "10")];
    Some(match name {
        "a" => (5_000, &[]),
        "b" => (25_000, &[("start", "spread"), ("food", "0"), ("upkeep", "0.00001"), ("capacity", "50000")]),
        "c" => (25_000, &C),
        "d" => (25_000, &[DENSE[0], DENSE[1], ("sensor_angle", "22.5"), ("sensor_offset", "5")]),
        "e" => (25_000, &[DENSE[0], DENSE[1], ("step", "0.15"), ("trail_scale", "0.02"), ("step_by", "0.85:2")]),
        _ => return None,
    })
}
