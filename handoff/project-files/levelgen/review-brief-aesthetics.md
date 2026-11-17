# AESTHETICS section, revised by the graphics lead (2026-11-15)

Replaces the `### AESTHETICS` section of review-brief.md. The owner: "Looks
is my number one goal/concern". Also add `wiki/looks/art-direction.md` to the
shared "Read first" list (all three reviewers should know the look rules).

### AESTHETICS

You review only how the planets look. Leave fun and code to the other two.
Looks are the owner's first concern, so be exacting: judge it as a shipped
game's art, not as a prototype.

**The bar.** The game's style guide is `wiki/looks/art-direction.md` ("living
light on dark ground"); read it before rendering anything. Judge against its
rules, and say which rule each finding breaks:
- the picture is the data: every pixel from a field or game state, no
  painted decoration, no thresholded edges on graded quantities;
- value order, darkest to brightest: ground and soils, rock and walls, thin
  slime, crystals, thick slime, units, light. Check it in grey;
- one hue a role: the slime gold to amber, crystals the only cyan, plants
  green, water dark blue, the team cream and olive, red for danger only;
- sharp at every zoom from 1 to 24: no cell stairs, seams or square grain.
A fix you propose must keep these, so a mood or palette change per planet
must come from something the simulation has (ground, water, plants, light
from the slime and crystals), not a tint painted over the screen.

**What is already known** (don't spend time re-finding it; say if it is
worse than described): the core world was too busy for its route to read;
plants covered bare biomes edge to edge; reviews and their pictures are in
`Reports/levelgen-review-2026-11-14/` and `Reports/levelgen-2026-11-14/`
(pages 1-3), `Reports/soil-materials-2026-11-14/`,
`Reports/river-look-2026-11-14/`. Check whether each was fixed.

**Look first, in motion, at the owner's zoom.** The owner plays zoomed in
and judges looks from moving pictures, never stills alone. For every planet
1-8, seeds 1 and 2:
- whole map at 1 s and 60 s (the slime grown);
- close views at zoom 6, 12 and 24 (`centre=X,Y,ZOOM`, world units; pick a
  shore, a rock edge, a plant patch's edge, a nest and the landing);
- a short GIF (3-5 s, 10 fps) at zoom 12 on at least four planets: the slime
  front crossing plants, soil and water, and one fire if a planet has one.

**Tests to run and quote** (numbers, not impressions):
1. *Grey thumbnail test:* every planet's 1 s frame shrunk to 266x164 in
   grey. Shown one, can you name the biome? Show a contact sheet.
2. *Close-zoom identity:* a 1280x800 frame at zoom 12 with no nest or
   crystal in it. Can you still tell the biome? The owner spends most of
   the game here, so this matters more than the overview.
3. *Slime stands out on every ground:* grey luminance of thick slime, thin
   slime film and the ground it covers, per biome. Thin slime must stay
   above every soil, rock, wall and plant; flag any biome where it doesn't.
4. *Crystals pop:* crystal against its surroundings in colour and grey on
   every biome; nothing else on the map near cyan.
5. *Seed variety:* two seeds of one biome side by side. Same place, a
   different map?

**Progression.** Played 1 to 8, does the look build toward the slime's home
world (rising strangeness, threat, density, a shift in palette or light)?
Or is it eight unrelated maps? Where does it sag or jump? Does the core
world read as a climax in the first second you see it? At 6x, what does the
whole-map view and the minimap show?

**Craft.** Edges at zoom 24 (rock, water, shore, plant patches, hedges,
bridges); seams, stair steps, tiling or repetition, noise that reads as
noise, flat empty stretches with nothing to look at, things that read as UI
(rings, bars, discs), anything gold that isn't slime or cyan that isn't a
crystal.

**Inspiration.** Top-down games that give each biome or floor its own look:
RimWorld's biomes, They Are Billions' map themes, The Riftbreaker's biomes,
Against the Storm's biomes, Mindustry's generated sectors, Don't Starve's
biomes, and the floor changes in The Binding of Isaac, Enter the Gungeon and
Nuclear Throne. Also Noita, which the owner cites: its look comes from the
world changing (burning, flowing, collapsing), not from decoration. Say
what they do that this doesn't, and how it could be done from this
simulation's fields.

**Output.** Picture paths for every claim (contact sheets in colour and
grey, close crops, GIFs). The three biome pairs that look most alike, at
zoom 1 and at zoom 12. The style-guide rule each finding breaks. Ranked
changes, each with the smallest visible fix and who would make it: the map
(levelgen) or the drawing (view.wgsl, the ground thread).
