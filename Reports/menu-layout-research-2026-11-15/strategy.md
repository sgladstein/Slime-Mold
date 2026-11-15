# Menu, Options and shop layout: what strategy and squad games do

Researcher: strategy / RTS / tactics / squad-shooter pool. Written 2026-11-15.

Brief: [brief.md](brief.md). Source pool: StarCraft II, XCOM 2, Into the Breach,
Helldivers 2, Deep Rock Galactic, Mindustry, Factorio, Age of Empires IV,
Battletech, Phoenix Point, They Are Billions. I also used the Xbox Accessibility
Guidelines, which are outside the pool.

**Marks.** **[V]** means I checked it in a source listed at the end. **[R]**
means I recall it from memory and did not check it this session. Treat [R]
layouts as likely, not as quoted fact. I could not open the Game UI Database
(its robots file blocks fetching). Most screen layouts below are therefore [R],
and I checked the parts I could in wikis, articles, developer quotes and one
open-source game (Mindustry).

## 0. What I saw in today's renders

- **The main menu** has 9 entries. Each is about 66 units tall plus a 12-unit
  gap, with a two-line subtitle. The title block takes about 130 units. Nine
  entries need about 700 units, but the worst-case frame is 565 units tall.
  At 150% on 1080p, Keys and everything below it are already cut off.
- **Options** has 15 rows of 62 to 93 units. The page is cut off at 150%, and
  at 100% only the 14th row is partly visible.
- **Every sub-screen repeats the big "Larder World" title and a subtitle**.
  That takes about 130 to 240 units at 100% and more at larger sizes.
- **The shop** has these parts:
  - the title;
  - a three-line planet briefing;
  - an Upgrades panel with 10 tiles and a detail box;
  - a Guns panel with four gun cards, each about 290 x 183 units with five stat
    bars;
  - the gun's slots, the cards for sale and the bag, all below the guns.

  At 150%, everything below the gun cards scrolls off. The only description box
  is in the Upgrades panel, top left. So when you scroll down to a bullet, its
  description is off screen. That is the owner's complaint, exactly.
- **The armory** has 8 guns (rolled, so two Rifles differ) and about 25 to 30
  bullets. The bullets form a grid of shapes (standard, slug, needle,
  buckshot) by payloads (bullet, blast, bomb, gas, germ and maybe more). The
  modifiers are on top of that. The five range actions take a whole panel.
- **Card tiles** are about 200 x 70 units and show an icon, a name, dots, a
  price and a shape glyph. The owner's later remark, that "the bullet tiles
  could easily be half the size without even losing any info", fits what I
  measured. Most of each tile is empty space.

## 1. What the games in my pool do

### 1.1 The precedents, one by one

**StarCraft II: Wings of Liberty, the Armory on the Hyperion**
- [V] It is a "tech purchase" screen laid out as "a table divided by rows and
  columns".
- [V] Seven category tabs run across the top: Base, Infantry, Vehicles,
  Starships, Mid-Tech, Ghost/Spectre and Dominion.
- [V] Rows are grouped by unit (SCV, Bunker, Marine...), with upgrade buttons
  in the columns.
- [V] Clicking an upgrade fills the bottom of the page. An animation plays at
  the bottom left. The cost, the description and a **Purchase button** sit at
  the bottom right.
- **Lesson:** tabs by category, compact buttons, and one fixed detail area
  that holds the buy button. The description never moves.

**StarCraft II: Wings of Liberty, the Research lab**
- [V] Protoss research is on the left and Zerg research on the right.
- [V] There are five tiers, each with two options side by side. Each option
  shows an icon, a name, a description and a short list of effects in its own
  cell.
- **Lesson:** when choices are few and paired, the description can live in
  the cell. Position carries meaning: the row is the tier and the column is
  the option.

**StarCraft II: Heart of the Swarm, the Evolution pit**
- [V] Mutations are grouped by unit. Each unit has three, each with an icon,
  name and effect. Choices "switch freely between missions".
- [R] The unit list runs down the left. The selected unit's three choices are
  shown large in the middle.
- **Lesson:** master and detail. Pick a group on the left and see only its
  choices.

**StarCraft II, the 2013 menu rework**
- [V] Blizzard's goals were to "simplify navigation, increase consistency, and
  improve usability". They removed redundant screens and moved Custom Game
  onto the main menu.
- [V] Blizzard said players spend about a third of their time in menus.
- [R] Today's StarCraft II main menu is a top bar of mode tabs. Each tab is
  its own full screen.

**XCOM 2, the Armory and loadout**
- [R] The soldier stands in the middle. A column on the left lists the
  equipment slots: primary, secondary, armour, utility, grenade and heavy.
- [R] Clicking a slot opens a list of only the items that fit that slot,
  beside it. The hovered item's stats show with the change against what is
  equipped.
- [V] The vanilla list becomes "horrible to work with" once mods add many
  items. A popular mod, Tactical Armory UI, exists only to add sorting and
  filters.
- **Lessons:**
  - The current loadout is always visible.
  - Picking starts from a slot, which filters the catalogue.
  - One long list does not scale.

**Helldivers 2, the Armory, Acquisitions and Warbonds**
- [V] The Armory uses tabs: Weaponry, Armory (armour), character features and
  Stratagems.
- [V] The Acquisition Center has three tabs: Warbonds, Superstore and Super
  Credits.
- [V] Warbonds are split into **pages** ("Page 1 (of 10)") that unlock in turn.
- [R] Each Warbond page is a fixed grid of item tiles, with the selected
  item's preview and details on the right. Pages turn with the shoulder
  buttons or Q/E.
- [R] The stratagem picker is a grid of icons grouped by department, with the
  hovered stratagem's details in a side panel.
- **Lesson:** use pages, not scrolling, for long catalogues. Use icon grids
  with one detail panel.

**Deep Rock Galactic, the Equipment Terminal**
- [V] Each weapon has five tiers of modifications. Players write a build as
  five letters, for example "BAAAC".
- [R] The terminal is one screen with no scroll:
  - the gear list on the left;
  - a grid of tiers in the middle, with two or three tiles in each tier row;
  - a stats panel on the right that shows green and red changes when you
    hover a tile;
  - an overclock slot.
- **Lesson:** a compact grid where position means something, plus deltas on
  hover.

**Into the Breach**
- [V] Justin Ma: "we would sacrifice cool ideas for the sake of clarity every
  time."
- [V] On animated tooltips: showing the animation "is a thousand times more
  effective" than text. He called it "the most important decision we made".
- [V] Cutting 60% of the planned content was "the closest thing to our eureka
  moment".
- [R] Every screen is a fixed 16:9 frame:
  - The squad screen shows three mechs, each with a pilot, two weapon slots
    and reactor cores, with storage below.
  - The reward shop is one small panel.
  - Nothing scrolls, because the content was budgeted to fit.
- **Lesson:** a content budget is a design decision. The detail panel can show
  behaviour, not just text.

**Mindustry**
- [V] I read this in its open source code. The desktop main menu is one left
  column of 230 x 70 buttons, each with an icon and a label and no subtitle:
  Play, Database, Editor, Workshop, Mods, Settings and Quit.
- [V] **Play opens a second column beside the first**, holding Campaign, Join
  game, Custom game and Load game. Database does the same.
- [V] Settings is a column of categories (Game, Graphics, Sound, Language,
  Controls, Data) next to a pane of settings, and that pane does scroll.
- [V] The UI scale slider runs from **25% to 300%**, the same span as Larder
  World's.
- [R] The in-game build menu is a column of category icons next to a grid of
  blocks, with the hovered block's details shown above it.
- **Lessons:**
  - Use a fly-out sub-column for grouped modes.
  - Even a game with a 300% UI scale still lets its settings pane scroll.

**Factorio**
- [V] Friday Facts #212 says the GUI rework planned for extremes: "20+ module
  slots", "recipes with 20+ ingredients" and "200+ inventory slots".
- [R] The main menu is a small centred box (Play, Settings, Mods, Exit). Play
  opens a sub-page.
- [R] Settings use category tabs.
- [R] The crafting window has four big group tabs (Logistics, Production,
  Intermediate products, Combat) above an icon grid. Each icon has a rich
  tooltip.
- **Lessons:**
  - Design for the largest content, not the typical case.
  - Use big group tabs above an icon grid.

**Age of Empires IV**
- [V] Text scaling has three presets (Default, Large, Largest) running from
  100% to 150%, and it applies to only some HUD and menu text.
- **Lesson:** a big commercial RTS caps text at 150%. Larder World's 300% is
  unusually generous, so the layouts must be designed for it.

**Xbox Accessibility Guideline 101 (outside the pool)**
- [V] On PC, body text should be at least 18 px at 1080p.
- [V] Players should be able to resize text to 200% "without losing content".
- [V] "Scaling never forces two-axis scrolling". Scrolling in one direction is
  allowed.
- **Lesson:** the owner's no-scroll rule is stricter than the industry
  guideline. It can still be met, but only with content budgets.

**Battletech, the Mechlab**
- [R] The mech's paper doll is in the middle, with its locations and
  hardpoints. An inventory list on the left has filter buttons (weapons, ammo,
  equipment). The stats summary is at the top. You drag and drop, with
  tooltips.
- [R] The inventory list scrolls inside a fixed frame.
- **Lesson:** the norm for big inventories is a fixed frame with one inner
  list. What the player builds stays pinned.

**Phoenix Point, the soldier equipment screen**
- [R] It has the soldier's slots (hands, ready, backpack) and a storage list
  with category filters. You drag and drop.
- **Lesson:** the same as Battletech.

**They Are Billions, the research screen**
- [R] All technologies fit one screen as a grid of tiers, with the hovered
  technology's details shown.
- **Lesson:** a fixed grid where position shows the tier.

### 1.2 The patterns, ranked by how often my pool uses them

1. **A pinned detail panel, or "inspector"**: StarCraft II Armory [V],
   XCOM 2 [R], Deep Rock Galactic [R], Helldivers 2 [R]. It shows the hovered
   or selected item and usually holds the buy or equip button. This alone
   fixes the owner's "description scrolled away" complaint.
2. **What you carry is always on screen**: XCOM 2 slots [R], Into the Breach's
   mech panel [R], Battletech's paper doll [R], Deep Rock's current build [R].
3. **Category tabs over a catalogue**: StarCraft II's seven tabs [V],
   Helldivers 2 [V], Factorio's crafting groups [R], Mindustry's block
   categories [R].
4. **Compact tiles where position means something**: the StarCraft II tables
   [V], Deep Rock's tier grid [R], the They Are Billions research grid [R].
5. **Short main menus with no subtitles, and modes nested under Play**:
   Mindustry [V], Factorio [R].
6. **Pages instead of scrolling** for long catalogues: Helldivers 2 Warbonds
   [V].
7. **Comparison with deltas against what you carry**: Deep Rock [R], XCOM 2
   [R], Helldivers 2 [R].
8. **Content budgets**: Into the Breach [V], Factorio's plan for extremes [V].

**What none of them do:** a page-length list with a scroll bar for the main
menu or the shop. Inner lists do scroll in some games (Battletech, Mindustry
settings, XCOM 2 item lists). But the frame, the current build, the detail
panel and the action buttons never move.

## 2. Ground rules for every layout below

- **Design against a 1000 x 565 safe frame.** Anchor regions to its corners.
  A bigger screen adds margin, and the inspector and tiles grow up to a cap. A
  bigger screen may wrap more tiles per row. It never needs a region that the
  small frame lacks.
- **Sub-screens get a compact header bar**, 40 to 48 units tall, holding
  "Back", the screen name and any tabs. The big title stays on the main menu
  only.
- **Actions are pinned** in a bottom bar 40 to 44 units tall: Land, Give up,
  Back.
- **Text sizes are kept** as the renders show them at 100%:
  - body text, about 13 to 14 units;
  - labels, about 16;
  - headings, about 20;
  - the title, about 40.

  A single-line control row is 36 to 40 units. A named card tile is 96 x 56,
  which is 38% of today's 200 x 70. An icon-only tile is 56 x 56.
- **The inspector follows the hovered tile.** When nothing is hovered, it
  follows the selected tile. When nothing is selected, it shows a default,
  such as the planet briefing. It also follows keyboard focus. The Buy button
  acts on the **selected** item, so the panel does not change while the mouse
  travels to the button.

## 3. Main menu

### M-A. One grouped column with a description panel (recommended)

The nine entries become single-line rows grouped into three blocks. Subtitles
move into one description panel beside the list, which shows the focused
entry. This is the Mindustry and Factorio style of short labels with no
subtitles, plus the inspector pattern from the shop.

At 1280 x 720:

```
x: 48                         368  400                          800        1280
y:0 +------------------------------------------------------------------------+
 48 | Larder World                (title 40u, block 48)                      |
 96 | A mould that grows, feeds and fights back   (14u, 20 tall)             |
140 | PLAY                       (section label 12u, 20 tall)                |
160 | [ Run                    ]  320 x 40      +-- description 400 x 120 --+ |
204 | [ Campaign               ]  pitch 44      | Run                       | |
248 | [ Random planet          ]                | Planet after planet toward| |
292 | [ Sandbox                ]                | the slime's home world;   | |
340 | MAKE AND PRACTISE                         | upgrades between.         | |
360 | [ Gun range              ]                +---------------------------+ |
404 | [ Edit maps              ]                                             |
456 | [ Options                ]                                             |
500 | [ Keys                   ]                 (live dish shows on the     |
544 | [ Quit                   ]                  right, as today)           |
668 | Up/Down and Enter, or click  .  F1 or ? keys   (12u)                   |
720 +------------------------------------------------------------------------+
```

At 1000 x 565 (24-unit margins, 36-unit rows at a 40-unit pitch, 16-unit
section labels):

| Element | x | y |
|---|---|---|
| Title | 24 | 24 to 68 |
| Tagline | 24 | 68 to 86 |
| PLAY label | 24 | 100 to 116 |
| Run, Campaign, Random planet, Sandbox | 24 to 284 | 116 to 272 |
| MAKE AND PRACTISE label | 24 | 280 to 296 |
| Gun range, Edit maps | 24 to 284 | 296 to 372 |
| Options, Keys, Quit | 24 to 284 | 380 to 496 |
| Description panel | 300 to 700 | 116 to 216 |
| Footer hint | 24 | 525 to 541 |

The total is 541 of 565, so it **fits**. If space gets short, drop the section
labels and use 10-unit gaps instead, which saves 30 units.

- **Cost:** no extra clicks, and every entry stays visible. A first-time
  player reads one subtitle at a time instead of all nine.
- **Can go wrong:** a 10th or 11th entry pushes it to the limit. The rule is
  at most 10 entries at the 565 frame; past that, switch to M-B.
- **Extra:** if a run is in progress, the top row should read "Continue run:
  planet 4 of 8" [R; Into the Breach and others put Continue first]. The
  description panel can show the run's state.

### M-B. A short column with a Play fly-out (Mindustry [V])

The top level has six entries: **Play >**, Gun range, Edit maps, Options, Keys
and Quit. Clicking Play opens a second column beside it holding Run, Campaign,
Random planet and Sandbox. These keep their two-line subtitles, because there
is room.

At 1280 x 720:

```
x: 48             288 300                       700
160 [ > Play       ]   [ Run                       ]  400 x 72
220 [   Gun range  ]   [  Planet after planet ...  ]
280 [   Edit maps  ]   [ Campaign                  ]
340 [   Options    ]   [  8 missions, one new verb ]
400 [   Keys       ]   [ Random planet ...         ]
460 [   Quit       ]   [ Sandbox ...               ]
    column 1: 240 x 56 rows, pitch 60        column 2 opens only on Play
```

At 1000 x 565, column 1 runs from x 24 to 224 with a 44-unit pitch, so its six
rows take 264 units. Column 2 runs from x 236 to 636, with four rows of 64 from
y 100 to 356. It **fits** with room to spare.

- **Cost:** one more click (or Enter) before every game start. The four modes
  are hidden until you open Play.
- **Can go wrong:** the owner starts runs often, and an extra step on the most
  used path feels slower. Mindustry gets away with it because Play is one
  click and the fly-out opens in place.

### M-C. A hero card and mode cards, with a utility bar (StarCraft II and Deep Rock style [R])

A large "Run" card (or "Continue run") is the call to action. Campaign, Random
planet, Sandbox and Gun range are smaller cards in a 2 x 2 grid below it.
Options, Keys, Edit maps and Quit become small buttons in a top-right bar.

At 1280 x 720:

```
y  24  Larder World                          [Options][Keys][Edit maps][Quit]  4 x 100 x 36
y 200  +-------- Run (or Continue run) -- 520 x 150 --------+
       | Planet after planet toward the slime's home world |
       +---------------------------------------------------+
y 370  [ Campaign       256 x 80 ] [ Random planet  256 x 80 ]
y 460  [ Sandbox        256 x 80 ] [ Gun range      256 x 80 ]
```

At 1000 x 565, the hero is 460 x 120 at y 96, and the 2 x 2 grid is 226 x 72 at
y 228 to 380. It **fits**.

- **Cost:** the look is more modern, but it is less linear with the keyboard.
  The small utility buttons are easy to miss, and the screen needs card art to
  look finished.
- **Can go wrong:** it looks like a storefront if the cards have no art.

**Recommendation: M-A.** It is the smallest change. It needs no extra clicks.
It matches what I verified in Mindustry (short labels, no subtitles), and the
description panel teaches the same habit the shop will use. Keep M-B as the
fallback if the menu grows past 10 entries.

## 4. Options

Most of today's 15 rows are not player settings. Only UI and text size, and
probably Control, are settings. Sandbox map, Planet, Your own mould, Rounds,
Bullet speed, Panic, Wither, Shared food, Condense, Breakaway, Thicken, Surge
and the rest are **game rules**. Strategy games put rules in the setup screen
of the mode they change [R: a skirmish lobby, Factorio's map generator at New
game, Mindustry's custom-game rules]. That decision belongs to the owner, so
O-C asks it and O-A works either way.

### O-A. A category rail, single-line rows and a description panel (recommended)

The categories are a column on the left, as Mindustry does it [V]. Each
category holds at most 8 single-line rows. A description panel under the rows
shows the focused row's note and its default.

Suggested categories:
- Display: UI and text size.
- Controls: Control, Rounds.
- Keys: the key list, as a tab of Options.
- Sandbox: map, planet, your own mould, bullet speed.
- Mould rules: panic, wither, shared food, condense, breakaway, thicken, surge
  and the 15th row.

At 1280 x 720:

```
y0   +------------------------------------------------------------------------------+
     | < Back    Options                                          header bar 48u    |
y48  +------------------------------------------------------------------------------+
y64  | [Display    ]  | UI and text size            [<]   150%   [>]   (reset)      |
     | [Controls   ]  | ... rows 40 tall, pitch 44, at most 8 = 352u               |
     | [Keys       ]  |    name 16u left; picker block 220u: [<]36 value148 [>]36    |
     | [Sandbox    ]  |    reset icon 28u, shown only when off the default          |
     | [Mould rules]  |                                                              |
     | rail 200 x 44  | content x 248..1232                                          |
y440 |                | +-- description 984 x 96 ----------------------------------+ |
     |                | | Every panel, card, tooltip and this menu, over the       | |
     |                | | screen's own scale; Ctrl + and Ctrl - anywhere.          | |
     |                | | Default 100%. Backspace resets.                          | |
     |                | +----------------------------------------------------------+ |
y668 | Left/Right change . Backspace resets . Tab next category . Esc back          |
y720 +------------------------------------------------------------------------------+
```

At 1000 x 565:

| Element | x | y |
|---|---|---|
| Header bar | full width | 0 to 40 |
| Rail (5 rows of 40) | 16 to 184 | from 52 |
| Rows (8 of 36 at a 40 pitch) | 200 to 984 | 52 to 372 |
| Description | 200 to 984 | 384 to 460 |
| Hint | | 533 |

It **fits** with about 60 units spare.

- **Cost:** one click, or Tab, to reach another category. A setting is hidden
  until you open its category.
- **Can go wrong:** a player looks for a setting in the wrong category. Keep
  the categories few and obvious. Mould rules can live in one long category
  only while it holds 8 or fewer rows.
- **Notes:**
  - The "back to the default" link becomes a reset icon on the row, and the
    description panel states the default.
  - Truncated notes ("guard or drai...") stop being a problem, because the
    full note goes in the panel.

### O-B. All 15 rows in two columns, with a description strip

All 15 single-line rows sit in two columns of 8, with one description strip
below.

- At 1280 x 720, the columns are each 604 units wide.
- At 1000 x 565, each column is 476 wide: a 230-unit name and a 200-unit
  picker. Eight rows of 40 run from y 52 to 372, and the strip from y 384 to
  460. It **fits**.
- **Cost:** no clicks and everything visible, but the page is dense. The
  keyboard order (down, then across) is less obvious.
- **Can go wrong:** setting 17 breaks it, so there is no room to grow.

### O-C. Split by purpose: settings in Options, rules at the start of a mode

- Options keeps only player settings: Display, Controls and Keys (and later
  audio and the window).
- The mould and sandbox rules move to a "Rules" panel. It shows when you start
  Sandbox or Random planet, next to a Start button, and stays reachable from
  in-game pause [R: the lobby pattern].
- Inside that panel, use O-A's rail or O-B's columns.
- **Cost:** one more screen on the way into Sandbox or Random planet. Rules
  are no longer one click from the main menu.
- **Can go wrong:** the owner and the threads use these rules as tuning knobs
  and expect them in one place. Ask first.

**Recommendation: O-A now, and ask the owner about O-C.** O-A fixes the
scrolling, puts the long notes in a panel, and already separates settings from
rules, so moving to O-C later is just moving two tabs.

**The Keys page** has about 40 rows in two columns, and it overflows at 150%
today. Use the same rail with contexts: Orders, Building and tools, Squad
mode, Camera and UI, Gun range. Each context has at most 10 rows.

**The missions list** has 8 two-line rows, about 624 units, so it overflows
the 565 frame. Use 8 single-line rows (320 units) and a description panel
holding the goal line.

## 5. The shop and armory

What must be visible all at once:
- the carried gun and its slots;
- the bag;
- what is for sale;
- a readable description of the hovered or selected item;
- the points;
- the Land button.

Every layout below shares one skeleton:

- **A top bar, 36 to 44 units:**
  - left: the one-line goal, "Planet 4 of 8 . Nezanus . Wipe out the slime:
    cut it to 2% of its peak";
  - right: the points.

  The two briefing lines move into the inspector's default view.
- **The bench**, always visible: the carried gun's name, budget and slot
  count, then its slots as named tiles in firing order, then the bag.
- **The inspector**, always visible on the right. It shows the hovered,
  selected or focused item, or the next-planet briefing when there is none.
  For a gun, it shows stat bars with up and down changes against the carried
  gun. It holds the **Buy** button, as the StarCraft II Armory does [V].
- **A bottom bar, 40 to 44 units:** Land on the next planet, Give up the run,
  and key hints. In the armory: Back to the range, Leave, and the five range
  actions as icon buttons that explain themselves in the inspector.
- **Tiles:**
  - Cards are 96 x 56: icon, name on up to two lines, pips and price. That is
    38% of today's 200 x 70 tile, in line with the owner's "half the size".
  - Guns are rows in a **comparison table**, not 290 x 183 cards. A table row
    is about 566 x 26 units, 28% of a gun card's area, and you compare by
    reading down a column.

### S-A. Shelves: everything for sale visible, in compact rows

The catalogue is a set of labelled one-row shelves, all visible: guns (as a
comparison table), cards for sale and upgrades. Buildings and Tools are folder
tiles that swap the upgrade shelf for their own list. This is the owner's
"shrink things and add an info bar" idea, close to Mindustry's build menu and
Factorio's crafting grid [R].

At 1280 x 720:

```
y0   +--------------------------------------------------------------------------------------+
     | Planet 4 of 8 . Nezanus . Wipe out the slime: cut it to 2% of its peak    670 points |  44
y56  +-------------------------------------------------------+------------------------------+
     | GUNS        range rate aim budget slots  price        | INSPECTOR  376 x 548         |
     | Long gun (carried)  35  0.6  0.3  16  3   --          | Needle bullet                |
     | Rifle (armory)      24  1.0  0.5  10  4   --          | bullet . 100 points          |
     | Carbine             19  2.2  1.1  15  5   290         | Fires a thin fast round      |
     | Carbine             18  3.6  1.8  10  5   240         | that ... (up to 6 lines)     |
y176 |   table: header 20 + 4 rows x 25 = 120, width 566     |                              |
y184 | IN THE GUN  budget 8/16 . 3/3 slots . left to right   | budget cost 1 . shape needle |
     | [1 Thrift][2 Slug bomb][3 Std blast]    112 x 64      |                              |
y272 | FOR SALE                                              |                              |
     | [Needle bullet][Std bullet][Scorch][Smother][Wide]    |                              |
y360 | BAG                                                   |                              |
     | [Buckshot gas]                                        |                              |
y448 | UPGRADES  10 tiles 76 x 64 (icon, pips, price)        |                              |
     | [Arm][Hvy][Trg][Brl][Lnd][Drp][Hnd][Crd][Bld>][Tls>]  | [ Buy . 100 points ]   44    |
y604 +-------------------------------------------------------+------------------------------+
     | [Land on the next planet]  [Give up the run]      click buys . drag to order . Esc  |  44
y660 +--------------------------------------------------------------------------------------+
     left column x 24..868 (844 wide)                         inspector x 880..1256
```

At 1000 x 565, the body runs from y 44 to 505 (461 units). The left column is
x 16 to 652 (636 wide), and the inspector is x 664 to 984 (320 wide).

| Region | y | Contents |
|---|---|---|
| Guns table | 44 to 158 | header 18, then 4 rows of 24 |
| In the gun | 164 to 236 | 6 slots of 96 x 56 = 620 wide |
| For sale | 242 to 314 | up to 6 named tiles |
| Bag | 320 to 392 | up to 6 named tiles, or 10 icon tiles |
| Upgrades | 398 to 470 | 10 icon tiles of 56; names only in the inspector |
| Spare | 470 to 505 | 35 units |

It **fits the run's shop, just**.

- **It does not fit the armory.** Eight guns take 210 units of table, and
  about 30 cards need about 4 rows. That breaks it, so the armory would need
  tabs anyway.
- **Cost:**
  - no extra clicks;
  - every category is visible, so trade-offs across categories are easy;
  - upgrades lose their names at the worst frame and must be learnt by icon
    or hover.
- **Can go wrong:**
  - "More cards on offer" or a bag past 6 cards overflows a shelf;
  - the screen is the densest of the three, and the owner may find it busy;
  - the armory and the shop end up looking different.

### S-B. Bench, tabbed catalogue and inspector (recommended)

The bench (slots and bag) and the inspector stay pinned. The catalogue sits
below the bench, under tabs:

- **Cards** (the default): the cards for sale.
- **Guns**: the comparison table, with the carried gun pinned as its first
  row.
- **Squad**: the upgrades, then Buildings and Tools as their own rows on the
  same tab, so the two folders go away.

Each tab shows a count and an "affordable" badge. This is the StarCraft II
Armory (tabs, a fixed detail area and a Purchase button) [V], plus the
Helldivers 2 armory tabs [V], plus the XCOM 2 rule of keeping the current
loadout visible [R].

At 1280 x 720:

```
y0   +--------------------------------------------------------------------------------------+
     | Planet 4 of 8 . Nezanus . Wipe out the slime: cut it to 2% of its peak    670 points |  44
y56  +-------------------------------------------------------+------------------------------+
     | Long gun (carried) . budget 8 of 16 . 3 of 3 slots    | INSPECTOR  376 x 548         |
y84  | [1 Thrift ][2 Slug bomb][3 Std blast][   ][   ]       | (nothing hovered:            |
     |   slots 112 x 64, pitch 120, up to 7                  |  Next: Nezanus, river valley |
y160 | BAG [Buckshot gas][        ][        ]  112 x 64      |  The Swarmer, 2.2x walkers;  |
y240 +-------------------------------------------------------+  21:00; slime spreads 11 s  |
     | [ Cards  5 ][ Guns  2 ][ Squad  3 affordable ]   36   |  before you land ...)        |
y284 | CATALOGUE  844 x 320                                  |                              |
     |  Cards: rows of up to 7 tiles 112 x 64                |  hovered card:               |
     |  Guns:  comparison table, carried row pinned on top   |  name 20u, kind, price       |
     |  Squad: Upgrades (2 rows) / Buildings / Tools         |  description 4-6 lines       |
     |                                                       |  gun: 6 stat bars + deltas   |
     |                                                       | [ Buy . 150 points ]   44    |
y604 +-------------------------------------------------------+------------------------------+
     | [Land on the next planet]  [Give up the run]   1-3 tabs . drag to order . Esc back    |  44
y660 +--------------------------------------------------------------------------------------+
```

At 1000 x 565:

```
y0    top bar 36  (goal line 16u, points right)
y44   bench header 18   x16..652           | inspector x664..984 (320 x 461)
y64   slots row 56: 6 x (96 x 56), pitch 104 = 620
y128  bag row 56: "Bag" 40u + 5 named tiles, or 10 icon tiles of 56
y196  tabs 32
y236  catalogue 636 x 269  (y236..505)
y513  bottom bar 40
```

The catalogue tabs at the worst frame (636 x 269):

| Tab | Height needed | Layout |
|---|---|---|
| Cards (shop) | 248 of 269 | 4 rows x 6 tiles, so up to 24 cards |
| Guns (armory) | 254 of 269 | header 20 + 9 rows of 26 (8 guns + the carried row); columns name 150, price 56, range 64, rate 64, aim 56, budget 64, slots 56, shots 56 = 566 wide |
| Squad (shop) | 268 of 269 | Upgrades 2 rows of named 96 x 56 tiles (120), Buildings 18 + 56, Tools 18 + 56 |
| Bullets (armory) | 258 of 269 | a **matrix**: rows are the 4 shapes, columns the payloads, with 18-unit column headers and a 72-unit row header; 6 columns of 88 x 56 |
| Modifiers (armory) | fits | a grid of named tiles |

Matrix tiles drop the name, because the row and column headers carry it
("Needle" row, "Gas" column).

The inspector at the worst frame (320 x 461):

| Part | Units |
|---|---|
| Name | 22 |
| Kind, price and level | 18 |
| Description, 5 lines | 90 |
| Stats, 6 rows | 132 |
| Buy button | 40 |
| Padding | rest |

That is about 340 units, so it **fits**.

The armory's tabs are **Guns, Bullets, Modifiers**. The shop's tabs are
**Cards, Guns, Squad**. The skeleton is the same.

- **Cost:**
  - one click, or a number key, to change category;
  - while you look at Cards, the guns and upgrades for sale are hidden, though
    the bench always shows your own gun's budget and slots;
  - two of the three things for sale are always one tab away.
- **Can go wrong:**
  - the player forgets the Squad tab and never buys armour. Count badges and
    "affordable" badges on the tabs reduce that [R: Helldivers and StarCraft
    II use badges for new or available items].
  - Q and E already reorder cards, so tabs need other keys: 1, 2, 3, or Tab.
  - the Squad tab at the worst frame is full. A third Buildings row would
    break it, so give it a budget.

### S-C. Two stations: the Gun bench and the Squad

The shop has two pages, switched in the top bar: **Gun bench** and **Squad**.
This is the owner's sub-menu idea, cut along the player's two decisions:
build the gun, or improve the squad. It works like Deep Rock's terminal, with
one page per gear item [R], and XCOM 2's armory sub-screens [R].

- **Gun bench page at 1000 x 565.** The left column (636 wide) holds, top to
  bottom:
  - the header (18);
  - the slots (56);
  - the bag (16 + 56);
  - the cards for sale, 2 rows (16 + 120);
  - the guns table, 4 rows (114).

  That totals about 428 of 461, so it **fits the shop**. The inspector is on
  the right.
- **Squad page.** It holds the upgrades, Buildings and Tools as named tiles,
  plus a large planet briefing. It has its own inspector.
- **The armory** has only a Gun bench page. With 8 guns and about 30 cards it
  does not fit, so it needs the S-B tabs anyway.
- **Cost:**
  - one page switch;
  - upgrades are hidden while you build the gun, which is fine because the
    two decisions share only the points, and those are in the top bar.
- **Can go wrong:**
  - it becomes S-B with fewer tabs and larger jumps;
  - a player who wants to compare "armour or a new gun" has to flip between
    the two pages.

### Recommendation for the shop and armory: S-B

- **It is the only layout of the three that holds both screens at 1000 x 565
  with headroom.** S-A fits only the shop, with about 35 units spare, and
  breaks when the shop grows. S-C fits only the shop.
- **It is the most common pattern in my pool**: the StarCraft II Armory [V],
  the Helldivers 2 Armory and Acquisitions [V], and the XCOM 2 loadout [R].
- **It fixes both complaints by construction:**
  - the description is in a pinned inspector, so it can never scroll away;
  - the slots, the bag, the Land button and the points never move.
- **It takes both of the owner's ideas without depending on either alone.**
  The sub-menus become tabs, and the info bar becomes the inspector. The
  owner's "half the size" tile remark is honoured: 96 x 56 named tiles, and
  88 x 56 unnamed tiles in the armory matrix.

**Two smaller decisions inside S-B:**
- **The armory bullets as a shape x payload matrix.** It teaches the system at
  a glance and frees the tiles from names. This is the StarCraft II
  rows-and-columns table [V] and Deep Rock's tier grid [R].
- **Guns as a comparison table.** Comparing guns is the job of that region,
  and four 290 x 183 cards with five bars each do it worse than one table
  plus deltas in the inspector [R: Deep Rock and XCOM 2 show deltas against
  what is equipped].

**A later idea, not part of the fix.** Into the Breach's most important UI
decision was an animated preview in the tooltip [V]. The inspector could one
day show a card's firing pattern, in the same way. The gun range is the live
version of that today.

## 6. What I would test with the owner

1. **Shop: S-A against S-B, as a blind A/B.** Make two static mockups each at
   1000 x 565 and at the owner's usual size (1600 x 972 at 125%, or 1080p at
   150 to 200%). Give the owner three tasks: buy the cheaper Carbine, put Needle
   bullet in slot 2 and read its description, and buy Armour level 2. Ask
   which felt faster and whether anything felt hidden. Under the repo's
   rules, the key stays off the page.
2. **Tile size.** Compare the 96 x 56 named tile with a 56 x 56 icon tile,
   for the bag and the cards for sale. The owner's remark points to the named
   tile, so confirm it.
3. **One question for Options:** may the mould and sandbox rules leave Options
   for a Rules panel at the start of Sandbox and Random planet (O-C)?
4. **The main menu needs no A/B.** Ship M-A and show it. Offer M-B only if the owner
   wants fewer top-level entries.

## 7. What the brief got wrong or missed

1. **The size cap looks only at width, so a wide monitor breaks the worst
   case.** The UI unit is set by the screen's height (1080 lines), but the
   cap keeps the width at 1000 or more:
   - 2560 x 1080 (21:9) at 200% gives **1280 x 540** units;
   - the same screen at the cap (about 256%) gives **1000 x 422**;
   - a 32:9 screen at 300% gives about 1280 x 360;
   - a resized window can be any shape.

   The cap should use both axes, keeping the frame at least 1000 wide **and**
   565 tall. Otherwise no layout in this report fits.
2. **Options is mostly not options.** About 13 of the 15 rows are game rules.
   The brief treats it as a settings page.
3. **"No scrolling" is stricter than practice.**
   - The Xbox guideline allows scrolling in one direction [V].
   - Mindustry's settings pane scrolls [V], and Battletech's and XCOM 2's
     inventory lists scroll inside fixed frames [R].

   The owner's real pain is that the description, the bench and the buttons
   scroll away. Keeping the stricter rule is fine, but then **every region
   needs a content budget**, and the brief sets none. Examples: the most
   cards on offer, the bag's size, the most slots, the most upgrades, at most
   8 rows per Options category, at most 10 main-menu entries. Into the Breach
   made budgets a design principle [V].
4. **There is no automatic check.** Each screen should be rendered at
   1000 x 565, 1280 x 540, 1280 x 720 and 1920 x 1080 by a test that fails if
   anything would need to scroll. Without that, the next feature quietly
   brings the scrolling back.
5. **The big title on every sub-screen** costs about 130 to 240 units of
   height and carries no information there.
6. **Hover-only information** needs rules:
   - keyboard focus must drive the inspector too;
   - the Buy button should act on the selected item, so the inspector does not
     change while the mouse travels to it.
7. **Keys clash.** Q and E reorder cards, so shop tabs need other keys.
8. **The Keys page and the missions list overflow too.** The brief names them
   but asks for no layout. Section 4 sketches both.
9. **Base text size.** At 100%, body text is about 13 units, below the Xbox
   PC minimum of 18 px [V]. The owner's "readable" setting is probably 150% to
   200%, which is why the worst case matters. AoE IV caps text at 150% [V]. A
   cap of 300% with a width cap is generous, so check which size the owner
   actually plays at, and test that size first.
10. **The bullets are a matrix (shape x payload), not a flat list.** The
    layout can use that structure.

## 8. The path

Each step can be shipped and seen on its own.

1. **Fix the cap** so the frame is never under about 1000 x 565 on either
   axis. Add the **fit test** at 1000 x 565, 1280 x 540, 1280 x 720 and
   1920 x 1080, failing on any overflow.
2. **The shared shell:**
   - a compact header bar on sub-screens;
   - a pinned bottom bar;
   - the inspector component, used by the menu, Options and the shop.
3. **Main menu M-A.** It is small and highly visible.
4. **Shop, step 1:** add the pinned inspector and cut tiles to 96 x 56. This
   removes the "description scrolled away" bug at once, even before the rest.
5. **Shop, step 2:** the guns comparison table, the catalogue tabs (Cards,
   Guns, Squad), Buildings and Tools inline on the Squad tab, and the
   briefing in the inspector's default view.
6. **Armory:** the same shell. Tabs for Guns, Bullets (the matrix) and
   Modifiers, with the range actions in the bottom bar.
7. **Options O-A**, the Keys page by context, and the missions list as single
   rows with a description panel. Then ask the O-C question.
8. **The owner's A/B** on S-A against S-B, if the owner wants to see the shelves
   version, before balancing tile sizes.

## Sources

Verified this session:

- StarCraft II Armory hands-on (Blizzplanet):
  https://www.blizzplanet.com/blog/comments/starcraft_ii_single_player_hands_on_the_armory_room
- StarCraft II research lab (Blizzplanet):
  https://www.blizzplanet.com/blog/comments/starcraft_ii_wings_of_liberty___research_lab
- Evolution pit (StarCraft Wiki): https://starcraft.fandom.com/wiki/Evolution_pit
- Heart of the Swarm UI update (Blizzard):
  https://news.blizzard.com/en-gb/starcraft2/10054118/heart-of-the-swarm-ui-update
- XCOM 2 Tactical Armory UI mod (Steam Workshop):
  https://steamcommunity.com/sharedfiles/filedetails/comments/1962124733
- XCOM 2 armory (wikis, with little layout detail): https://xcom.fandom.com/wiki/Armory
  and https://xcom2.wiki.fextralife.com/Armory
- Helldivers 2 armory tabs (The Review Geek):
  https://www.thereviewgeek.com/helldivers2-charactercustomization/
- Helldivers 2 Acquisition Center (wiki.gg): https://helldivers.wiki.gg/wiki/Acquisition_Center
- Helldivers 2 Warbond pages (Fandom): https://helldivers.fandom.com/wiki/Warbonds
- Deep Rock Galactic mod tiers and five-letter builds:
  https://ljvmiranda921.github.io/notebook/2022/12/02/drg
- Deep Rock Galactic Equipment (wiki.gg): https://deeprockgalactic.wiki.gg/wiki/Equipment
- Into the Breach UI (Game Developer):
  https://gamedeveloper.com/design/-i-into-the-breach-i-dev-on-ui-design-sacrifice-cool-ideas-for-the-sake-of-clarity-every-time-
- Into the Breach cut content (PC Gamer):
  https://www.pcgamer.com/into-the-breach-originally-had-city-building-and-tech-trees-creators-reveal
- Mindustry main menu source (MenuFragment.java):
  https://raw.githubusercontent.com/Anuken/Mindustry/master/core/src/mindustry/ui/fragments/MenuFragment.java
- Mindustry settings source (SettingsMenuDialog.java):
  https://raw.githubusercontent.com/Anuken/Mindustry/master/core/src/mindustry/ui/dialogs/SettingsMenuDialog.java
- Factorio Friday Facts #212: https://factorio.com/blog/post/fff-212
- Age of Empires IV accessibility: https://www.ageofempires.com/age-iv-accessibility/
- Xbox Accessibility Guideline 101:
  https://devdocs.xbox.com/build/game-principles/accessibility/xag-deep-dives/xag-101-text-display.md

Recalled, not verified this session:

- the XCOM 2 loadout layout;
- the Helldivers 2 page grid and stratagem picker;
- the Deep Rock Galactic terminal layout;
- the Into the Breach fixed frames;
- the Factorio menus and crafting tabs;
- the Battletech Mechlab;
- the Phoenix Point equipment screen;
- the They Are Billions research screen;
- the current StarCraft II top bar.

The Game UI Database (gameuidatabase.com) blocks automated fetching, so I could
not use its screenshots.
