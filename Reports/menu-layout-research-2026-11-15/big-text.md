# Menus that fit one screen: the big-text and console view

Researcher: big-text pool (console and TV "10-foot" UI, Steam Deck,
games known for text size and accessibility, published guidance).
Date: 2026-11-15. Brief: [brief.md](brief.md).

Labels used below:

- **[V]** I checked this in a source during this research (link in
  [Sources](#sources)).
- **[R]** I recall this from memory. I did not check it. Treat it as likely,
  not proven.
- **[O]** A game from another researcher's pool. I name it once for
  contrast.

The Game UI Database (gameuidatabase.com) blocks automated fetches, so I
could not use it. The screen descriptions here come from developer docs,
wikis, reviews and memory.

---

## 1. The answer in brief

- **The worst case is a TV screen, and consoles solved it long ago.**
  1000 x 565 units is almost exactly the 960 x 540 "effective pixel" canvas
  that Microsoft gives every Xbox app [V]. Body text of about 14 to 15
  units at that size is about 28 px on a 1080p panel. That is the
  10-foot minimum in the Game Accessibility Guidelines [V]. So the game has
  to be laid out like a console game, not shrunk from a desktop layout.
- **Most of the space today goes to chrome, not content.** On the shop at
  1920x1080, 200% (about 1000 x 565 units), the title, subtitle and planet
  briefing take about 220 units. The buttons and hint line take about 107.
  That leaves about 240 of 565 units (42%) for the shop itself. Cutting
  the chrome to about 100 units does more than any other single change.
- **Main menu:** use a short list with one caption. The rows show labels
  only, about 38 to 44 units tall. The subtitle of the focused or hovered
  entry shows once, in a caption box beside the list. Keys moves into
  Options as a tab. This is how Hollow Knight, Celeste and Hades lay out
  their main menus [R].
- **Options:** use tabs, compact rows and a description pane. That means
  tabs across the top, at most 8 rows per tab, and the note, the default
  and a reset button in a fixed pane on the right. This is the console
  standard, as in Forza Horizon 5 [V] and The Last of Us Part II [V/R].
- **Shop and armory:** use a bench on top, a tabbed counter below and a
  pinned detail pane on the right.
  - The bench is the carried gun's slots and the bag. It stays on screen
    always.
  - The counter has tabs: Cards, Guns, Upgrades. In the armory they are
    Bullets, Modifiers, Guns and Range.
  - The tiles are compact, about half today's size.
  - One fixed detail pane shows the hovered or selected item, with its
    stats compared to the carried gun.

  Hollow Knight's charm screen [V] is the closest match. Destiny 2 [V/R]
  and Diablo IV [V] show the parts: tabs, capped slot counts and a
  compare panel.
- **The rule to adopt** is stricter than XAG and more exact than "no
  scrolling":
  - The frame never scrolls.
  - The header, the detail pane and the action buttons are pinned.
  - Every list has a fixed maximum. A catalogue that can outgrow its frame
    gets tabs or pages, never a scrollbar.

---

## 2. What today's renders show

I measured these on the renders in the brief's `today/` folder. Units are
pixels divided by the scale. At 1920x1080 and 200%, one unit is about 1.91
pixels.

| Screen | What eats the height | Units at worst case |
|---|---|---|
| Main menu | Each entry is a full-width card with a title and subtitle, about 78 units from one card to the next. Nine entries need about 700 units. | Needs 700+, has 565 |
| Options | Each setting row has a note, about 62 units per row. Fifteen rows need about 930 units. Notes are cut off with "..." even at 100%. | Needs 930+, has 565 |
| Shop | Title block about 125 units, briefing about 95, buttons and hint about 107. Each gun tile has five stat bars, about 175 units tall at 100%. | Content gets about 240 |
| Armory | The same chrome. Eight gun tiles, then the slots, then 20 or more bullet tiles at about 185 x 62 each at 100%. It scrolls even at 100% on 1920x1080. | Scrolls at every size |

More observations:

- **The game title "Larder World" sits on every inner page at full
  size.** Console games show the logo only on the title screen. Inner pages
  carry a short page name or breadcrumb [R: Destiny 2, Diablo IV, Forza].
- **The shop's description box belongs to the Upgrades column.** It is
  about 140 units tall and shows one line. It is not a global pane, so a
  card's description can sit far from the card and scroll away. That is
  the owner's complaint word for word.
- **Gun tiles show every stat at all times.** Console games show a summary
  on the tile and the full stats in a tooltip or pane, compared to what
  you carry [R: Destiny 2, Diablo IV].
- **The owner's 17:41 remark fits the evidence.** The owner said the bullet tiles
  "could easily be half the size without even losing any info." A bullet
  tile today holds an icon, a name, a type glyph, cost pips and a price in
  about 185 x 62 units at 100%. An icon, a name and a price fit in about
  134 x 44. The pips and glyph can move to the pane or shrink. But halving
  the bullet tiles alone does not fit the shop into 565. The chrome and
  the gun tiles are the bigger costs. Small tiles are needed, but they are
  not enough.

---

## 3. The frame: what "big text" means in numbers

| Source | Rule | What it means here |
|---|---|---|
| Microsoft, *Designing for Xbox and TV* [V] | Xbox apps render at 1080p scaled 200%. Lay out for **960 x 540 effective px**. Main text at least **15 epx**, secondary text at least **12 epx**. Interactive items at least **32 epx** tall. Keep essentials 48 epx from the sides and 27 from the top and bottom. "The amount of information displayed on a TV should be comparable to what you'd see on a mobile phone, rather than on a desktop." | The worst case of 1000 x 565 units is this canvas. Plan phone-level density: one main list, one detail, a few actions. |
| Xbox Accessibility Guideline 101 [V] | At least **26 px** text on console and **18 px** on PC, at 1080p. Players can resize text "up to 200% ... without losing content, functionality, or meaning". "Scaling never forces two-axis scrolling ... One-direction scrolling is fine." | XAG allows one-way scrolling. The owner's rule is stricter, and it can be met. |
| Game Accessibility Guidelines, "Use an easily readable default font size" [V] | "use 28px as a minimum rather than a target", based on 10-foot TV guidance at 1080p. | 14 to 15 units at about 1.9x is 27 to 29 px. |
| Steam Deck compatibility [V] | No glyph under **9 px at 1280x800**, and Valve recommends 12 px. Text must be readable at 30 cm. | The game's 1280x800 at 175% case is about 990 x 620 units, so taller than the 16:9 worst case. |

**Proposed type and size rules for the layouts below.** I assume the
game's text sizes stay roughly as they are today; tune them, but keep the
ratios.

- Page header: 20 units. The title screen logo is the only exception and
  is larger.
- Item and row label: 16 to 18 units.
- Body and description: 14 to 15 units.
- Small (prices, hints): 12 to 13 units, never below 12.
- Rows and buttons: at least 32 units tall, 38 to 44 when comfortable.
- Outer margin: 32 x 24 units at 1000 x 565, and 48 x 32 at 1280 x 720.

---

## 4. Precedents

### 4.1 Guidance and platforms

- **Microsoft TV and Xbox design** [V]: see section 3. Two more lines
  matter here.
  - "Give people the shortest path to what they want to do."
  - Crossing the screen edge to edge should take "no more than six clicks".

  Microsoft does allow lists that run off screen and scroll. Its guidance
  is about keeping the focused item and the frame steady, not about
  banning scrolling.
- **XAG 101** [V]: see section 3. It also says: "Text that scales
  off-screen has a way to be read (scrolling text, popouts, or appropriate
  abbreviations)". The popout matches the detail pane below.
- **Indie menu guidance** (indieklem, *Creating an intuitive in-game
  menu*) [V]:
  - "Try to get to fewer than 6 or 7 items" at the top level.
  - "Group related menu items together".
  - Reveal advanced options gradually: progressive disclosure.

### 4.2 Main menus and big menus

- **Super Smash Bros. Ultimate (Switch).** The top level is Smash,
  Spirits, Games & More, Vault, Online, plus a Dashboard and Other.
  Dozens of modes sit one level down. **Options lives under Dashboard**,
  not at the top level [V, SmashWiki]. The top level is big tiles with a
  caption for the focused tile [R]. *Lesson:* a game with far more modes
  than Larder World keeps five to seven top entries by grouping.
- **Hollow Knight (PC, Switch).** The main menu has about five short
  labels: Start Game, Options, Achievements, Extras, Quit Game. Options
  opens sub-pages: Game, Audio, Video, Controller, Keyboard [R].
- **Celeste.** The main menu is a short vertical list of about five
  entries (Climb, Options, Credits, Exit, plus Pico-8) [R]. *Lesson:* even
  small studios keep the title list to single-word labels, with no
  subtitles.
- **Destiny 2.** The pause menu is five pages along the top: Clan,
  Collections, Journey, Character, Inventory [V, Bungie help]. *Lesson:*
  related screens become tabs of one frame, not a stack.

### 4.3 Settings screens with many options

- **Forza Horizon 5.** Settings are split into tabs: Accessibility,
  Difficulty, HUD and Gameplay, Video, Controls and others. On PC, "Page
  Up and Page Down switch tabs" [V, Forza support]. It has an "Increased
  Text Size" option that "increases the overall size of all menu text" [V].
  *Lesson:* big text plus tabs is the standard pairing.
- **The Last of Us Part II.** It has more than 60 accessibility settings
  in named sections and three presets (Vision, Hearing, Motor). Each
  option has a short description [V, PlayStation Blog and Naughty Dog].
  In the game the description, and for some settings a short preview
  clip, shows in a pane for the focused row only [R]. *Lesson:* the row
  shows the name and value; the explanation lives in one place.
- **Hollow Knight.** Options is a list of category pages [R]. This is the
  sub-menu version.
- **Celeste.** The Options page is one list that scrolls when it is long
  [R]. This is a counter-example: it works because the list is the only
  thing on the page, and nothing pinned scrolls away.

### 4.4 Shops, loadouts and inventories

- **Hollow Knight's charm screen. This is the closest match to the
  armory.** [V, secondary source with screenshot]
  - The equipped charms are a row at the top left.
  - Under it, a notch row shows the cost used against the capacity, such
    as 5 of 11.
  - Below, all owned charms sit in one fixed grid. There are 45 charms and
    at most 40 can be held [V, wiki].
  - On the right, a description panel shows the selected charm's name,
    notch cost and description.

  Map it onto this game:
  - Equipped charms are the gun's slots.
  - Notches are the gun's budget.
  - The charm grid is the bag or the catalogue.
  - The description panel is the detail pane.

  It fits on one screen because the counts are capped and the grid tiles
  are icons with no text.
- **Destiny 2's character screen.**
  - Weapon and armour slots flank the character [V, Medium and
    gamepressure].
  - Each gear slot holds "up to 10 of each item" [V, Bungie help]. That
    cap is why the per-slot pop-out grid always fits.
  - Hovering a slot opens a small grid of the spares for that slot [R:
    3 x 3]. Hovering an item shows a tooltip with stats and green or red
    compare bars against the equipped item [V: "graphical stat indicators
    that show differences between items"].
  - The full item details are one step deeper, on an inspect screen [R].

  *Lesson:* small tiles, a capped count, and a rich detail with
  comparison.
- **Diablo IV (PC and console share one UI).**
  - The inventory has tabs: Equipment, Consumables, Quests, Aspects. Each
    has limited capacity [V, gamepressure].
  - The developers wrote: "A unified UI means our layouts are more
    grid-based for ease of navigation" [V, Blizzard quarterly update,
    February 2020].
  - Item tooltips compare against the equipped item side by side [R].

  *Lesson:* split the categories into tabs with grids of fixed capacity.
  The same layout then serves mouse and keyboard.
- **Elden Ring and Persona 5 shops** [R]: a list on the left with a
  pinned item detail on the right or bottom. The list scrolls but the
  detail never does.
- **Noita** [O, R]: four wands, each showing its row of slots, and a spell
  inventory of fixed size (16) [R]. Hovering shows a tooltip. Even Noita
  caps the bag so the screen is fixed.

### 4.5 What goes wrong: PC layouts scaled up or shrunk onto small screens

- **Hades on Switch.** A review says "like most Switch ports, the game's
  text in handheld mode is almost unreadable" [V, Vooks]. Readers report
  that "the descriptions of boons can be difficult to read" [V, Nintendo
  Life comments].
- **Nintendo Life's list of Switch games with small text** includes
  Hollow Knight, Slay the Spire, Diablo III and Darkest Dungeon [V]. All
  of them kept a desktop layout and shrank it.
- **Star Vortex's Steam Deck patch** increased the text size across the
  whole UI. Its comparison tooltips then "no longer scale with UI scale
  since they are too big" [V, SteamDeckHQ]. *Lesson:* floating tooltips
  break at big UI sizes. A pinned pane with a fixed footprint does not.
- **The Outer Worlds** added font scaling after launch. A UI programmer
  said the team "didn't know how much of an issue it would be" [V, Game
  Developer]. *Lesson:* text size is a layout problem, and it is cheaper
  to plan the layout for it than to patch it later.

---

## 5. Patterns that keep big text on one screen

Ranked by how often the big-text pool uses them and how well they suit
this game:

1. **One detail pane per screen, pinned.**
   - Seen in: Hollow Knight charms, TLOU2 settings, Elden Ring and
     Persona shops, and XAG's "popouts".
   - Why: it lets every other element shrink to a label or icon.
   - Fit here: high. It fixes the owner's main complaint directly.
2. **Tabs for categories, switched with keys and click.**
   - Seen in: Forza Horizon 5, Diablo IV, Destiny 2.
   - Why: tabs keep the frame still and swap only the contents.
   - Fit here: high, for Options and for the shop counter.
3. **A fixed maximum for every list.**
   - Seen in: Destiny 2's 10 per slot, Hollow Knight's 40 charms,
     Noita's 16 spells, and Diablo IV's limited tab capacity.
   - Why: fixed counts allow a fixed layout.
   - Fit here: required for the bag and the counter.
4. **Grouping the top level to five to seven entries.**
   - Seen in: Smash Ultimate, Hollow Knight, Celeste.
   - Fit here: medium. Nine entries fit at 565 if the rows are label-only.
5. **Labels only on rows, with explanations in a caption.**
   - Seen in: every console main menu and settings page I know.
   - Fit here: high.
6. **Tiles in a grid instead of rows of full width.**
   - Seen in: Diablo IV ("more grid-based"), Smash Ultimate.
   - Fit here: high for cards and upgrades; lower for settings.
7. **Pages (◀ 1/2 ▶) instead of scrolling** for catalogues that really
   are long.
   - Seen in: Pokémon storage boxes and Destiny's vault pages [R].
   - Fit here: a fallback for the armory if the card set grows.
8. **Accordions (expand one section, collapse the others).**
   - I found **no big-text precedent** for a main shop or settings layout
     built this way. Destiny's pop-out is the nearest thing, and it is an
     overlay, not a reflow.
   - The cost: opening a section moves everything below it, which breaks
     spatial memory and drag targets.
   - Fit here: low. This is the owner's "expand guns, shrink them" idea.
     Tabs give the same result without the reflow.

---

## 6. Proposed layouts

All wireframes are at 1280 x 720 units. Each has a "holds at 1000 x 565"
check that adds up the heights. A "u" after a number means units.

### 6a. Main menu

#### A1. Short list with one caption (recommended)

```
1280 x 720
+------------------------------------------------------------------------------+
|  Larder World                                   (logo, 40u)                  |
|  A mould that grows, feeds and fights back      (16u)            y 40..110   |
|                                                                              |
|  +------------------------------+    +-----------------------------------+   |
|  |# Run                    44u  |    | Run                         (18u) |   |
|  |  Campaign                    |    | Planet after planet toward the    |   |
|  |  Random planet               |    | slime's home world; upgrades      |   |
|  |  Sandbox                     |    | between                     (15u) |   |
|  |  Gun range                   |    |                                   |   |
|  |  Edit maps                   |    | caption 420 x ~140, top-aligned   |   |
|  |  Options                     |    | with the list; follows hover and  |   |
|  |  Quit                        |    | the arrow keys                    |   |
|  +------------------------------+    +-----------------------------------+   |
|   list 360 wide: 8 x 44 + 7 x 4 = 380u          (the live dish shows behind) |
|                                                                              |
|  Up/Down and Enter, or click  ·  F1 keys        (13u)               y 676    |
+------------------------------------------------------------------------------+
```

- **Content:** eight entries. Keys becomes a tab in Options and stays on
  F1 everywhere. The order puts the modes first and Quit last. Nothing is
  hidden.
- **Holds at 1000 x 565:**
  - The logo block is 64u: a 32u logo plus the tagline.
  - The list is 8 x 38 + 7 x 4 = 332u.
  - The total is 24 + 64 + 20 + 332 + 16 + 16 (hint) + 24 = **496 ≤ 565**.
  - If Keys stays on the list, it is 9 rows: 374u, total **538**.
  - The caption box sits at x = 376 and is 360u wide.
- **Costs:**
  - The subtitles of entries you are not pointing at are hidden.
  - A new player hovers to learn what each one does. That is one glance,
    not a click.
- **What can go wrong:**
  - The caption must not jump. Keep it at a fixed spot, not glued to the
    focused row.
  - Mouse and keyboard focus must agree: hovering moves the focus.

#### A2. Hero tile plus a grid of mode tiles (Smash Ultimate style)

```
1280 x 720
+------------------------------------------------------------------------------+
| Larder World  /  tagline                                         y 40..110   |
| +-----------------------------+ +---------------------+ +------------------+ |
| | RUN                         | | Campaign            | | Gun range        | |
| | Planet after planet toward  | | 8 missions, one new | | Every gun, card  | |
| | the slime's home world;     | | verb each           | | and bullet, free | |
| | upgrades between            | +---------------------+ +------------------+ |
| |                             | | Random planet       | | Sandbox          | |
| | 460 x 330                   | | 300 x 157           | | 300 x 157        | |
| +-----------------------------+ +---------------------+ +------------------+ |
|  [ Options ]  [ Keys ]  [ Edit maps ]                             [ Quit ]  | 44u
|  hint                                                                        |
+------------------------------------------------------------------------------+
```

- **Holds at 1000 x 565:**
  - The hero tile is 340 x 236.
  - The mode tiles are 2 x 2, each 286 x 114, with a two-line subtitle at
    14u.
  - The utility row is 36u.
  - The total is 24 + 64 + 16 + 236 + 12 + 36 + 12 + 16 + 24 = **440**.
- **Costs:**
  - The arrow keys move in two dimensions.
  - It looks more like a launcher or a console dashboard.
  - Every subtitle shows, which is good for a first visit.
- **What can go wrong:**
  - A sixth mode breaks the 2 x 2 grid.
  - Long subtitles wrap to three lines at 300%.
  - It can feel heavy for nine items.

#### A3. Two levels: Play opens a side column

```
+------------------------------------------------------------------------+
| Larder World                                                           |
| +-------------------+  +--------------------------------------------+  |
| |# Play          >  |  | Run            Planet after planet ...     |  |
| |  Gun range        |  | Campaign       8 missions, one new verb    |  |
| |  Edit maps        |  | Random planet  A new generated planet ...  |  |
| |  Options          |  | Sandbox        Free play, with Control rts |  |
| |  Quit             |  |  (4 rows x 64u, subtitle on every row)     |  |
| +-------------------+  +--------------------------------------------+  |
+------------------------------------------------------------------------+
```

- **Holds at 1000 x 565:** five rows of 44 is 236u. The side column is
  4 x 64 = 268u. It fits easily.
- **Costs:** every mode except Gun range is one click deeper. Campaign
  becomes three levels deep, because it has its own mission list.
- **What can go wrong:** the main mode, Run, is hidden one level down.
  That is the opposite of what a title screen is for.

### 6b. Options (15 settings)

The groups below come from the setting names in the renders. The fifteenth
setting is cut off in the renders.

- **Game:** UI and text size, Control, Rounds, Bullet speed.
- **Sandbox:** Sandbox map, Planet, Your own mould.
- **Mould:** Panic, Wither, Shared food, Condense, Breakaway, Thicken,
  Surge, and the fifteenth setting. That is eight rows at most.
- **Keys:** the key list.

#### B1. Tabs, compact rows and a description pane (recommended)

```
1280 x 720
+------------------------------------------------------------------------------+
| < Options                                                   (20u)    y 32    |
| [ Game ]  [ Sandbox ]  [ Mould ]  [ Keys ]     tabs 40u · Q/E or Tab         |
+--------------------------------------------------+---------------------------+
| UI and text size      [<]   150%    [>]   ↺      | UI and text size   (18u)  |
| Control               [<]   rts     [>]   ↺      | Every panel, card,        |
| Rounds                [<]   plain   [>]          | tooltip and this menu,    |
| Bullet speed          [<]   45      [>]          | over the screen's own     |
|                                                  | scale; Ctrl + and Ctrl -  |
|                                                  | anywhere            (15u) |
|  rows 44u; list 700 wide:                        |                           |
|  label 360 | picker 240 | reset 40               | Default: 100%  [ Reset ]  |
|  at most 8 rows per tab                          | pane 440 x 480            |
+--------------------------------------------------+---------------------------+
| Left/Right change · Backspace resets · Q/E tabs · Esc back          (13u)    |
+------------------------------------------------------------------------------+
```

- **Holds at 1000 x 565:**
  - The header is 36, the tabs 36, the rows 8 x 38 + 7 x 4 = 332, and the
    footer 16.
  - The total is 24 + 36 + 8 + 36 + 12 + 332 + 12 + 16 + 24 = **500**.
  - Width: the list is 560u (label 270, picker 210, reset 32). Then a 16u
    gap and a 360u pane.
- **Costs:**
  - You can see only one tab at a time.
  - A row's note shows only when the row is focused or hovered.
  - Today's "back to the default" link becomes a ↺ icon on rows that
    differ from the default, plus a Reset button in the pane.
- **What can go wrong:**
  - A tab that grows past 8 rows. Make 8 a hard cap, and add a tab rather
    than a scrollbar.
  - Players hunting for a setting across tabs. Keep the grouping obvious,
    and show the changed rows (↺) so the tab badges can say "2 changed".

#### B2. All 15 settings in two columns, notes in a bottom bar

```
1280 x 720
+------------------------------------------------------------------------------+
| < Options                                                                    |
| GAME AND SANDBOX                        | MOULD                              |
| UI and text size   [<] 150%  [>]  ↺     | Panic         [<]  2    [>]        |
| Control            [<] rts   [>]  ↺     | Wither        [<] 0.01  [>]        |
| Rounds             [<] plain [>]        | Shared food   [<] 0.05  [>]        |
| ... 8 rows x 44u                        | ... 8 rows x 44u                   |
+------------------------------------------------------------------------------+
| Note: Every panel, card, tooltip and this menu ...   Default 100% [Reset]    | 2 lines
+------------------------------------------------------------------------------+
```

- **Holds at 1000 x 565:**
  - Two columns of 460u. The rows are 8 x 38 + 28 = 332u.
  - The note bar is 56u.
  - The total is 24 + 36 + 18 + 332 + 12 + 56 + 16 + 16 + 24 = **534**.
- **Costs:** none hidden; everything is visible in one look. The trade is
  density.
- **What can go wrong:**
  - The layout holds 16 settings at most. The next research knob breaks
    it.
  - Player settings and research knobs sit side by side.
  - Labels are limited to about 26 characters at 14u.

#### B3. Category list, then a sub-page (Hollow Knight style)

- The Options page shows four big entries: Display and controls, Sandbox,
  Mould, Keys.
- Each opens a full page of at most 6 rows, with the note under each row,
  as today: 6 x 56 = 336u.
- **Holds at 1000 x 565:** yes.
- **Costs:** one more click for every setting, and no overview.
- **What can go wrong:**
  - "Where was Wither?" takes a guess.
  - The Back paths get longer.

### 6c. Shop and armory (the hard one)

These must show at once:

- the carried gun and its slots, with budget and slot counts;
- the bag;
- what is for sale: upgrades, guns, cards;
- a readable description of the hovered or selected item;
- the points;
- Land and Give up.

The armory also needs the range tools: New rolls, Deeper, Shallower, Empty
the gun, Empty the bag.

#### C1. Bench on top, tabbed counter, pinned detail pane (recommended)

```
1280 x 720
+------------------------------------------------------------------------------+
| Shop · Planet 4 of 8, Nezanus: wipe out the slime   670 points  [ Land > ]   | 48u
+-----------------------------------------------------+------------------------+
| THE GUN  Long gun · budget 8 of 16 · 3 of 3 slots   | DETAIL PANE            |
| +--------+ +--------+ +--------+ +--------+ +------+| [icon] Slug bomb (20u) |
| |1 Thrift| |2 Slug  | |3 Stand.| |   --   | |  --  || Bomb · 6 budget · 150  |
| |   free | |   bomb | |  blast | |        | |      || Explodes where it      |
| +--------+ +--------+ +--------+ +--------+ +------+| hits; ... 4 to 6 lines |
|   slot tiles 140 x 60                               | (15u)                  |
| BAG  [Buckshot gas] [     ] [     ] [     ] [     ] |                        |
|   bag tiles 140 x 48, fixed capacity                | In your gun: budget    |
+-----------------------------------------------------+ 8 -> 14 of 16          |
| [ Cards 5 ]  [ Guns 2 ]  [ Upgrades 10 ]   tabs 40u |                        |
| +----------+ +----------+ +----------+ +----------+ | For a gun: five bars,  |
| |> Needle  | |> Standard| |o Scorch  | |x Smother | | carried vs this one,   |
| |  bullet  | |  bullet  | |          | |          | | with green or red      |
| |   . 100  | |   .  90  | |  ... 150 | |  ..  150 | | deltas                 |
| +----------+ +----------+ +----------+ +----------+ |                        |
| |+ Wide 80 | |          |  tiles 176 x 64           | [ Buy 150 ]            |
| +----------+                                        | pane 440 wide          |
+-----------------------------------------------------+------------------------+
| Click buys · drag to the gun or bag · Q/E reorder · Tab next tab · Esc back  | 24u
+------------------------------------------------------------------------------+
```

- **The header** is one line: where you are going and the goal. The full
  briefing (biome, slime type, clock, surge) is what the detail pane shows
  when nothing is hovered. So the pane is never empty, and the briefing
  stops costing about 95 units. **Give up the run** moves to Esc, into a
  small menu with a confirmation.
- **The bench** (gun slots and bag) is always on screen. Every purchase
  decision depends on the gun's free slots and budget.
- **The counter tabs:**
  - Each tab shows its count and a dot when it holds something new or
    affordable.
  - Upgrade folders (Buildings, Tools) open in place, with a breadcrumb:
    "Upgrades › Buildings", plus a back tile.
  - Gun tiles are compact: icon, name, price, and chips for "3 slots ·
    budget 16". The five stats move to the pane, compared to the carried
    gun.
- **The detail pane:**
  - It follows the hover.
  - With no hover, it shows the last item you clicked or focused with the
    keyboard. That fixes "I clicked a bullet and can't read it".
  - While you drag a card, it shows that card and what it does to the
    gun's budget.
- **Holds at 1000 x 565:**
  - Margins are 32 x 24. The width of 936u splits into a 560u left side,
    a 16u gap and a 360u pane.
  - The header is 40u.
  - The gun block is 70u: an 18u label plus 52u tiles. Five slots: 5 x 106
    + 4 x 6 = 554u.
  - The bag is 62u: an 18u label plus 44u tiles. That is five named tiles
    of 106u, or eight icon tiles of 64u.
  - The tabs are 34u.
  - The counter is 4 rows x 48 + 3 x 6 = 210u. Card tiles are 4 columns of
    134u.
  - The footer is 16u.
  - The total is 24 + 40 + 8 + 70 + 8 + 62 + 10 + 34 + 8 + 210 + 8 + 16 + 24
    = **522 ≤ 565**.
  - The pane gets 445u of height. Its content needs about 392u:
    - a 28u title and a 20u line for type and cost;
    - six lines of description at 20u;
    - a comparison of five rows at 24u;
    - a 40u Buy button;
    - padding.
  - The Upgrades tab at this size is 5 x 2 tiles of 106 x 88u (icon, name
    on up to two lines, pips, price): 182u, under 210.
  - The Guns tab is 2 columns x up to 4 rows of 276 x 48u, so 8 guns.
- **The armory uses the same frame.**
  - The header shows "Gun range · Free" and the range tools as five small
    buttons.
  - The tabs are Bullets, Modifiers, Guns.
  - Twenty bullets are too many for a plain grid at 565: 6 rows of 31u is
    under the 32u minimum. Lay the Bullets tab out as a **matrix**
    instead. The columns are Standard, Slug, Needle, Buckshot. The rows
    are bullet, blast, bomb, gas, germ. The tiles are 120 x 36u with icon
    and pips, and no name is needed because the headers name the cell. The
    size is 72 + 4 x 120 = 552u wide and 18 + 5 x 36 + 4 x 4 = 214u tall.
    It fits.
  - I am inferring the bullet names from the renders. If the card set is
    not a clean grid, use pages (◀ 1/2 ▶) inside the tab.
- **Costs:**
  - Only one counter category shows at a time, so comparing an upgrade
    with a card takes a tab switch.
  - Tile names shrink to one or two short lines. The full text is in the
    pane.
- **What can go wrong:**
  - A list without a cap. The bag, the cards on offer and the slots each
    need a fixed maximum, set to the screen.
  - Tabs that hide something new. Use badges, and open the shop on the
    tab with new stock.
  - Q/E is already the reorder key. Pick a distinct tab key, such as Tab
    and Shift+Tab, plus clicking.

#### C2. One board, no tabs (everything for sale visible, compact)

```
1000 x 565
+------------------------------------------------------------------------+
| Shop · Planet 4 of 8 Nezanus ...                 670 pts  [ Land > ]   | 40u
+------------------------------------------------+-----------------------+
| GUN    [1 Thrift][2 Slug bomb][3 Std blast][ ][ ]    52u  | DETAIL     |
| BAG    [Bshot gas][ ][ ][ ][ ][ ][ ][ ]               40u  | PANE       |
| UPGR   [♥ 240][⌖ 120][» 120][◉ 100][+ 150]                 | 320 wide   |
|        [❄ 100][⌃ 100][≡ 100][Bldg >][Tools >]  2 x 44u     |            |
| GUNS   [Carbine 290][Carbine 240][Rifle armory]       44u  |            |
| CARDS  [Needle 100][Std 90][Scorch 150][Smother 150]       |            |
|        [Wide 80][  ][  ][  ]                       2 x 44u |            |
+------------------------------------------------+-----------------------+
| hints                                                                  |
+------------------------------------------------------------------------+
```

- **Holds at 1000 x 565:** 24 + 40 + 8 + 52 + 6 + 40 + 6 + 94 + 6 + 44 + 6
  + 94 + 8 + 16 + 24 = **468**. It fits, but only with icon-only upgrade
  tiles and two-line card names at 12 to 13u.
- **Costs:**
  - Upgrade names are hidden; you hover to read them.
  - The board is dense.
  - The armory cannot use this layout, because its catalogue is three to
    four times larger. So the two screens would differ.
- **What can go wrong:**
  - "More cards on offer" or a new upgrade overflows it.
  - Drag targets get small at 75 to 100%.
- **Why test it anyway:** it is the "everything at a glance" shop that
  card roguelikes use [O: Slay the Spire, Balatro]. The owner may prefer
  seeing every price at once.

#### C3. Two sub-screens: Buy and Build (the owner's sub-menu idea, as screens)

```
[ Buy ]  [ Build the gun ]      (big tabs at the top; Tab switches)

Buy:   upgrades 5 x 2 at 140 x 96 | guns 2-4 at 270 x 96 with 3 key stats |
       cards 4 x 2 at 170 x 56 | detail pane at the right; a read-only strip
       of the gun's slots ("would go in slot 4")
Build: the carried gun large (5 slots at 150 x 80, with names and order
       arrows) | bag 8 at 120 x 60 | rack of owned guns | detail pane
```

- **Holds at 1000 x 565:** with room to spare on both pages. The tiles are
  the largest of the three options.
- **Costs:**
  - Buy a card, then switch pages to place it. A bought card could go to
    the first free slot or the bag by default.
  - The bag is hidden while you buy.
  - There are two modes to learn.
- **What can go wrong:**
  - Players buy cards that do not fit the budget, because the budget is
    on the other page.
  - Destiny 2 and Monster Hunter separate the vendor from the loadout [R],
    but their loadouts do not drive every purchase the way this gun does.

---

## 7. Recommendations

| Screen | Pick | Why |
|---|---|---|
| Main menu | **A1**, a short list with one caption, with Keys moved into Options | It is the console default. Every mode is one click away. It fits at 565 with about 70u spare. It is the cheapest change. |
| Options | **B1**, tabs, compact rows and a description pane | It is the console and accessibility standard (FH5, TLOU2). It survives new settings. The notes stop being cut off. |
| Shop | **C1**, bench, tabbed counter and pinned pane | The bench stays visible because buying depends on the gun. The pinned pane fixes the reported bug. The tabs follow Diablo IV, Destiny and FH5. It fits at 565 with about 40u spare. |
| Armory | **C1 with a Bullets matrix** | It is the same frame as the shop, so it is learned once. Hollow Knight's charm screen shows that a full catalogue in a fixed grid plus a detail panel works. |
| Missions, Keys | The same skeleton | Missions: 8 label-only rows plus a pane for the objective (8 rows with subtitles need about 572u, just over 565). Keys: tabs by topic. |

Rules that apply to every screen:

1. **The frame never scrolls.** The header, the detail pane and the
   actions are pinned.
2. **Every list has a maximum, set to the 1000 x 565 frame.** If content
   can pass it, it gets tabs or pages.
3. **Same regions, same places, at every UI size.** On larger screens the
   grids gain columns and the pane gets wider. Nothing appears, vanishes
   or moves to another region with size, so the player's spatial memory
   holds.
4. **Logo and headings do not scale up like body text.** XAG asks only
   that headers "remain visually differentiated", not that they grow
   with the body text. The "Larder World" logo appears on the title screen
   only.
5. **Focus follows hover, and the detail pane follows focus.** Mouse and
   keyboard show the same thing.

**The owner's two ideas, weighed against the evidence:**

- **"Sub-menus: expand the guns and shrink them."** The grouping is right.
  The accordion form has no big-text precedent I could find, and it moves
  the layout under the cursor. **Tabs** (C1) or **sub-screens** (C3) give
  the grouping without the reflow.
- **"Shrink things and have an info panel."** This is strongly supported.
  It is the single most common pattern in my pool. Make it a **pane** (a
  column about 360 to 440u wide), not a thin bar, so gun stats can show
  with comparison bars.
- **"The bullet tiles could easily be half the size" (17:41).** This is
  consistent with C1's 134 x 48u card tiles. It is necessary but not
  sufficient: the chrome and the gun tiles are the larger costs.

---

## 8. What to test with the owner

1. **Blind A/B pages** (per CLAUDE.md, with the key kept off the page).
   Show each at 1000 x 565 and at 1600 x 972, 125%:
   - main menu: A1 vs A2;
   - Options: B1 vs B2;
   - shop: C1 vs C2.

   Static renders are enough for the main menu and Options. The shop
   should be a playable build.
2. **Shop tasks, timed and counted** (clicks, mistakes, "where is it?"
   moments):
   - Read what Smother does.
   - Buy the cheapest bomb and put it first in the gun.
   - Which gun for sale has the longest range, and is it better than
     yours?
   - Buy one level of Armour.
   - Open Buildings and come back.

   Then ask which version felt like a professional game.
3. **An automatic fit check.** Render every screen at:
   - 1000 x 565;
   - 1280 x 720;
   - 1920 x 1080 at 100%;
   - 1280 x 800 at 175%;
   - a 4K screen at 300%.

   Fail if any widget lies outside the frame, overlaps another, or cuts
   its text. This keeps "no scrolling" true as content grows.
4. **One question for the owner:** do the mould research knobs (Panic,
   Wither, Condense...) belong in Options at all? See section 9, item 2.

**Path:**

1. Write the frame rules from section 3 and the rules in section 7 into
   the UI code's comments and the wiki.
2. Cut the chrome. Take the logo off inner pages, make the shop header one
   line, and move the buttons and hints into the header and footer. This
   is cheap and it is the biggest win.
3. Build the shop's pinned detail pane, with hover, focus and drag states.
   It fixes the reported bug even before the rest lands.
4. Build the main menu as A1 and Options as B1, with Keys as a tab.
5. Build the shop and armory as C1: bench, tabs, compact tiles and the
   Bullets matrix.
6. Run the A/B with the owner. Add the fit check to CI.

---

## 9. What the brief got wrong or missed

1. **It treats this as fitting the content. A large part is too much
   chrome.** About 58% of the shop's height at worst case is title,
   briefing, buttons and hints.
2. **Options mixes two kinds of setting.**
   - Player settings: UI size, Control, Keys.
   - Simulation research knobs: Panic, Wither, Condense, Breakaway,
     Thicken, Surge, Shared food.

   Console games put the knobs in a custom-game or sandbox setup step,
   reached from the mode that uses them [R: Halo custom games,
   Civilization's advanced setup]. Moving them would shrink Options to
   about five rows and make the knobs easier to find for the modes that
   use them.
3. **"No scrolling" is stricter than the guidance.** XAG allows scrolling
   in one direction. Microsoft lets TV lists run off the edge. The owner's
   complaint is about the **frame** scrolling: the description and buttons
   left behind. The rule should say that, and then add caps so even lists
   never need to scroll.
4. **Capacities are not stated.** Fixed layouts need fixed maxima:
   - slots per gun (5?);
   - bag size (unbounded today?);
   - cards on offer at the maximum "More cards on offer" level;
   - guns for sale;
   - entries in the Buildings and Tools folders.

   Destiny 2 (10 per slot), Hollow Knight (40 charms) and Noita (16
   spells) all cap.
5. **The armory is a catalogue, not a shop.** It holds about three to four
   times the items. "Same layout" must mean the same frame, not the same
   tile flow.
6. **The detail pane's states are not specified:**
   - hover;
   - keyboard focus;
   - after a click;
   - during a drag;
   - nothing focused.

   Comparison against the carried gun, and the budget a card would use, is
   the information that matters most for a buying decision.
7. **Larger screens are not addressed.** At 100% on 1080p the frame is
   1920 x 1080 units. The layout must not just stretch. Give regions
   minimum and maximum widths, and anchor them.
8. **The default UI size.** At 100% on 1080p the body text is about 13 px.
   That is below XAG's 18 px PC minimum. If the default is 100%, the frame
   most players see is not the one the owner found readable. Check what
   the game opens at.
9. **The mission list and the key list** are named but not given a target.
   The mission list with subtitles overflows at 565 by a few units. The
   key list (about 40 rows) needs tabs.

---

## Sources

Guidance:

- Microsoft, Designing for Xbox and TV (960 x 540 epx, type and target
  minimums, TV-safe area, density):
  https://learn.microsoft.com/pl-pl/windows/apps/design/devices/designing-for-tv
- Xbox Accessibility Guideline 101, Text display:
  https://devdocs.xbox.com/build/game-principles/accessibility/xag-deep-dives/xag-101-text-display.md
- Steam Deck compatibility review (9 px at 1280x800):
  https://partner.steamgames.com/doc/steamdeck/compat
- Game Accessibility Guidelines, "Use an easily readable default font
  size" (28 px): https://gameaccessibilityguidelines.com/?p=3285
- Game Accessibility Guidelines, font size options:
  https://gameaccessibilityguidelines.com/?p=285
- indieklem, Creating an intuitive in-game menu:
  https://indieklem.substack.com/p/9-creating-an-intuitive-in-game-menu

Games:

- Forza Horizon 5 accessibility support (Settings tabs, Page Up and Page
  Down, Increased Text Size):
  https://support.forza.net/hc/en-us/articles/4409280058259-Forza-Horizon-5-Accessibility-Support
- Xbox Wire, Forza Horizon 5 accessibility:
  https://news.xbox.com/en-us/2021/11/04/gaming-for-everyone-the-accessibility-features-of-forza-horizon-5/
- PlayStation Blog, The Last of Us Part II accessibility:
  https://blog.playstation.com/2020/06/09/the-last-of-us-part-ii-accessibility-features-detailed
- Naughty Dog, The Last of Us Part II accessibility:
  https://www.naughtydog.com/blog/THE_LAST_OF_US_PART_II_ACCESSIBILITY_FEATURES_DETAILED
- Bungie help, Navigating the menu screen (five top pages):
  https://help.bungie.net/hc/en-us/articles/45753961638292--6-Navigating-The-Menu-Screen
- Bungie help, Inventory management ("up to 10 of each item" per slot):
  https://help.bungie.net/hc/articles/360049199931
- UI of Destiny 2: The Character Screen (Medium):
  https://medium.com/@saoliveri95/ui-of-destiny-2-the-character-screen-3463f8fc0e3a
- Gamepressure, Destiny 2 inventory screen:
  https://www.gamepressure.com/destiny-2/inventory-screen/z5e484
- Blizzard, Diablo IV quarterly update, February 2020 ("more grid-based"):
  https://news.blizzard.com/en-us/article/23308274/diablo-iv-quarterly-updatefebruary-2020
- Gamepressure, Diablo 4 interface (inventory tabs):
  https://www.gamepressure.com/diablo-iv/interface/z0109a5
- Hollow Knight charms and notches (screen layout):
  https://saschb2b.github.io/game-mechanics/games/hollow-knight/charms-and-notches
- Hollow Knight wiki, Charms (45 total, 40 held):
  https://hollowknight.wiki/w/Charms
- SmashWiki, Menu (Ultimate's top level and nesting):
  https://www.ssbwiki.com/Menu

Counter-examples:

- Vooks, Hades Switch review (handheld text):
  https://vooks.net/hades-switch-review
- Nintendo Life, Switch games with small text:
  https://nintendolife.com/guides/nintendo-switch-games-with-small-text-the-worst-offenders-how-to-use-zoom-on-switch
- SteamDeckHQ, Star Vortex UI text overhaul:
  https://steamdeckhq.com/news/star-vortex-gets-ui-text-overhaul-for-steam-deck/
- Game Developer, The Outer Worlds font scaling:
  https://gamedeveloper.com/design/an-upcoming-ui-tweak-will-make-i-the-outer-worlds-i-infinitely-more-accessible
- Game Developer, Legible font still one of the biggest accessibility
  issues: https://gamedeveloper.com/design/legible-font-still-one-of-the-biggest-accessibility-issues-in-games
