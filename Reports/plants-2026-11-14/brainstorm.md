# Plants: a new simulation (brainstorm, 2026-11-14)

Owner, 2026-11-14 17:02Z (levelgen thread): plants are **fully new**, not
the engine's old grazing grass (`BigParams::graze_*`, a uniform field that
starts at 1 and regrows in place); biomes roll their own patterns with
biome weights; every map stays beatable by any build (guns, turrets and
kill boxes, sneaking, sniping, artillery, jumping).

## The model, neutrally

The world is a grid of fine cells. Each cell holds a ground word (soil
kind, wet, rock), and timed fields: trail, scent, scorch, wall health.
Walkers sense fields ahead of them, turn, step, deposit trail, eat from
crystals, are born from stored energy and starve without it. Soldiers walk
on open ground, shoot along lines that stop at the first walker, build,
and burst (kill, clear, crater, burn) at points.

A plant is a **new per-cell field, 0-1 (density), with a life of its own**:
it grows where the ground lets it, spreads to neighbours, and dies back. On
its own it already makes moving pictures: fronts that creep across damp
soil, bare rings where something eats it, burnt scars that green over. The
question is what it couples to.

## What a plant field could do

Each idea is one coupling; any can be on or off.

| # | Coupling | What you'd see | Strategies it serves |
|---|---|---|---|
| 1 | **Food that regrows**: walkers on a plant cell eat it; it regrows by spreading from what is left | Slime grazes meadows bare in rings and moves on; green comes back behind it | Slime can live off crystals for a while, so cutting it off is slower on green ground; burning or clearing meadows starves it; bait fields lure it into kill boxes |
| 2 | **Cover**: soldiers in dense plants are hidden (smell muffled; panic charges don't see them) | Soldiers half-sunk in brush; slime walks past a squad in a thicket | Sneaking, ambush, sniping from a hedge |
| 3 | **Slow ground**: thick brush slows soldiers and walkers; walking through tramples a path | Trampled lanes that stay for a while and close up | Route choice; the slime and the squad both carve paths |
| 4 | **Fuel for fire**: a fire field spreads through dense dry plants, kills walkers in it, leaves scorch; wet ground doesn't burn | Fire fronts racing through a meadow, black scar that greens over | Artillery and incendiary rounds that become area denial; firebreaks; scorch rounds light the grass |
| 5 | **Blocks sight**: tall plants stop shots like slime does | Shots thud into a hedge | Clearing firing lanes; artillery arcs over; snipers pick spots |
| 6 | **Thickets as barriers**: dense plants act like soft wall; walkers must eat through | Slime stalled at a hedge, chewing a gap | Grown walls, mazes that regrow |
| 7 | **Slime-eating plants** (fungus, thorns): walkers on them lose energy; the plant grows from what it kills | A ring of thorns that thickens where slime dies on it | Natural kill boxes; luring slime into a thorn field |
| 8 | **Slime feeds the soil**: dead walkers and scorch make plants grow faster | Battlefields bloom after a fight | Turns past fights into future cover or fuel |
| 9 | **Harvest**: soldiers cut plants for a small, renewable energy income | Cut stubble around the base | An economy that isn't only crystals |
| 10 | **Sow your own**: a building or card that seeds a patch (a hedge, a bait field) | A planted line greening up over a minute | Builder strategies; grown defences |

Biomes would differ in which kinds grow and how: meadow grass everywhere
and fast; jungle thick and tall (cover, sight); fen reeds on wet ground
(don't burn); barrens sparse scrub in clumps; highlands patches in hollows;
caves glowing fungus (maybe the slime-eating kind); river valley bands
along water. The level file gives each cell a start density (B channel,
0-255, written by levelgen) and the soil decides how it regrows.

## Recommendation

One plant field, its growth and spread set by soil, wet and biome, with
three couplings first: **food (1), cover (2) and fire (4)**. They pull
the slime one way (grazing toward green), the squad another (hiding in
green) and give both a way to destroy it (fire), so the meadow becomes a
contested resource rather than decoration, and each serves a different
build: food for luring and starving, cover for sneaking and sniping, fire
for artillery. Slow ground (3) and sight (5) are cheap follow-ups once the
field is in; slime-eating plants (7) and sowing (10) are second species
or buildings for later.

Order: the field and its growth on CPU and GPU (seen in a film), then
grazing, then cover, then fire; each lands with tests and a film. The
look (how plants are drawn from the field) goes to the graphics lead
before `view.wgsl` changes.
