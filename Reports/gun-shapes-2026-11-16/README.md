# Bullet shapes: step A of the gun-card plan

2026-11-16. Step A of the [gun-system review](../gun-system-review-2026-11-16/README.md)'s
plan ("Fix the sameness"), which the owner asked for at 03:37Z: "A full
written proposal on how to explore all of this. Then decide if you need to
dig deeper on an area before building do it otherwise start
implementing/prototyping." The complaint it answers (owner, 02:34Z): "we have
five different categories of bullets and two of them are just two different
sized explosions ... within those all the cards in each category are
relatively similar. Just changing in some of the base stats".

Page: [Bullet Shapes](https://claude.ai/artifact/N7arMoib2upstzMzdf9rMF)
(source in `page/`).

## What changed

- **Each bullet set has a shape** (`bullets::Shape`, `Set::shape`). Until
  now a set only leaned its rolls (`Set::lean`); at a front that showed as a
  hole 1.26x (slug), 0.77x (needle) or 0.89x (buckshot) as wide as a standard
  one.
  - **Slug:** every slime hit shoves (`Rounds::shove`): a fling with no kill
    (`Burst::fling`), its ring centred one strike radius back toward the gun
    and reaching 3 strike radii past the hit, the walkers at the hole thrown
    furthest (0.25 s, about 5 dish cells). Centred on the hit, the ring's near
    half would throw slime at the shooter.
  - **Needle:** two more slime hits on top of any `Pierce` cards
    (`Shape::pierce`, read in `Game::fly`).
  - **Buckshot:** each of the gun's shots is four pellets fanned 0.25 rad
    either side of the aim, each half the strike radius (area 0.25 each, a
    standard hit in all), in `Game::step`'s firing.
  - **Standard:** unchanged (the firing code path is bit-identical).
  - **Payloads take their set's shape:** every pellet and every pierce hit
    casts, and a buckshot burst is half the radius (`bursts`). A needle blast
    blasts three times in a line; a buckshot gas is four small clouds.
- **Blast and bomb apart** (`RoundRules`). The blast is unchanged: kill 6
  cells, crater 7 cells for 20 s, so it holds ground. The bomb kills 2.5 cells
  (was 4), throws the ring out to 10 cells (the same reach: ring 4x, was 2.5x)
  for 0.7 s (was 0.5), and keeps a 2.5-cell crater 1 s (was 4 cells, 6 s). Its
  throw is a burst of its own centred a kill radius back toward the gun
  (`Rounds::fire`, `Pop::throw`), so the slime flies away from the squad. The
  bomb is drawn ice blue (`card_look`), the blast orange as before.
- **The germ bullet is sold** from planet 4, with the bomb (`run::shells_at`,
  `card_power`). It was held back because "the GPU world casts none yet", but
  the GPU world has cast it since 11-14 (`GpuWorld::has_germ`). The options
  menu lists the germ loadouts again for the same reason.
- **Card text** names each set's shape ("Its hit shoves the slime back.",
  "Hits up to 3 times in a line.", "Fires 4 small pellets in a fan.") and the
  new bomb ("Throws slime 10 cells out; kills little.").
- **Test loadouts** (H): `slug`, `needle`, `buckshot` (standard numbers),
  `needle-blast`, `buck-gas`. A film key `load=T,NAME` loads every squad with
  a loadout at game second T, so films of several loadouts share one game up
  to T.

## Tests

- `rounds::tests::each_set_has_its_own_shape` (seed 2, 30 s, all four sets in
  one gun): buckshot fires 4 bullets a shot at half radius, needles carry 2
  pierces, every slug hit shoves and no shove kills. Counts: shots 23, 22, 20,
  19; bullets 23, 22, 20, 76; hits 18, 20, 50, 70 (standard, slug, needle,
  buckshot): 50 needle hits from 20 needle bullets. 20 shoves threw 2,931
  walkers.
- `rounds::tests::a_slug_shoves_and_a_bomb_throws_away_from_the_gun`: the
  first lone hit that throws 40 or more walkers; share moved away from the gun
  along the bullet's heading: slug 239 of 265 (seed 2), 39 of 46 (seed 3);
  bomb 80 of 93, 50 of 60.
- `rounds::tests::a_blast_holds_and_a_bomb_throws` (seed 3, at the mould's
  middle): blast killed 360 and threw 0; bomb killed 57 and threw 959.
- `run` test: a germ bullet turns up in planet-5 shops (60 seeds).
- The GPU twin `bomb_rounds_fling_the_same_mould_on_the_engine_as_on_the_cpu`
  applies a pop's throw as its own burst.

## Films

`hand=hold@150,265 bank=8000`, the Fork, squad mode, llvmpipe, near view
`centre=176,266,12`, 960 x 600, 30 frames a second (`raw/film.sh`,
`raw/make_clips.py`). Paired clips: `rounds=plain load=29,NAME`, frames
29.0-32.0 s: one game to 29 s, then each loadout for 3 s. Counts are the
game's over the clip (8 soldiers, 24 shots each clip).

| Loadout (paired, 3 s) | Bullets | Killed | Squad health at 32 s |
|---|---|---|---|
| standard (plain) | 24 | 347 | 73 |
| slug | 24 | 138 | 82 |
| needle | 24 | 1,265 | 68 |
| buckshot | 96 | 179 | 75 |
| blast | 24 | 1,694 | 80 |
| bomb | 24 | 161 | 84 |
| needle blast | 24 | 1,922 | 81 |
| buckshot gas | 96 | 213 | 84 |

**The blind clips' key** (kept off the page): clip 1 needle, clip 2
standard, clip 3 buckshot, clip 4 slug. The deciding measure set in the plan:
can the owner name the set from the clip with the labels off.

Whole games with one loadout from the start (`rounds=NAME`, at 33 s, one
seed), for the trade each shape makes:

| Loadout | Killed by 33 s | Squad health | Thrown by 33 s |
|---|---|---|---|
| standard | 1,248 | 72 | |
| slug | 374 | 95 | 10,954 |
| needle | 3,549 | 77 | |
| buckshot | 612 | 85 | |
| blast | 4,130 | 84 | |
| needle blast | 4,464 | 88 | |
| bomb, main | 2,515 | 76 | 10,903 |
| bomb, first try (thrown all round) | 1,096 | 49 | 11,024 |
| bomb, this change | 861 | 67 | 15,175 |

Bomb clip (30-33 s): main killed 335 and threw 1,102; this change killed 147
and threw 3,296.

## Tried and dropped

- **The step-A bomb thrown all round its hit**, as the 11-14 bomb was. With
  the smaller kill and longer throw, the near half landed on the squad: health
  49 at 33 s against main's 76 (one seed). The throw now goes away from the
  gun (health 67; it still kills a third of what main's bomb did, so more
  slime reaches the squad). `../dead-ends.md` has the entry.

## Not settled

- **The needle is strong**: 1,265 killed in the paired 3 s against 347 for a
  standard bullet, and 3,549 against 1,248 over a whole game, since its later
  hits land in the thick of the body. Its price comes only from its rolled
  numbers (`Ammo::grade`). A note for the balance pass (build first, balance
  later); no census was run.
- **Slug and buckshot kill less** (138 and 179). The slug trades kills for
  holding the front back (health 95 against 72 over a whole game, one seed).
- **Frame cost**: standard bullets run exactly as before. Buckshot traces four
  rays a shot. Each slug hit adds one throw burst; on the GPU that is one fling
  pass over the walkers and one read-back of the count (`Engine::fling`), as a
  bomb's throw is, up to about 8 a second for a squad of 8 slugs. Not timed on
  the GTX 1070; the read-back batching planned for step B covers it.
- The owner's blind read of the four set clips.

## Files

- `page/index.html` and its clips: the Artifact page's source. The blind
  clips are `quiz-1.mp4` to `quiz-4.mp4` (key above).
- `raw/film.sh`: one clip's frames (`START`, `CENTRE` from the environment).
- `raw/make_clips.py`: frames to H.264 MP4 and the counts as JSON.
