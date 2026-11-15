# Menus and shop on one screen: what roguelikes do

Researcher: roguelikes, roguelites and deckbuilders (2026-11-15).
Brief: [brief.md](brief.md). One of three independent reports.

## Summary

- **Main menu:** a column of the five ways to play, each with its one-line
  subtitle, plus one small row for Options, Keys, Edit maps and Quit. It fits
  at 1000 x 565 with about 30 units to spare. It needs no extra clicks.
- **Options:** tabs ("Screen and controls", "The mould", "Keys"). Each row is
  one line. The focused row's note and its reset button sit in a fixed note
  pane beside the list. Keys becomes a tab.
- **Shop:** one fixed frame with no tabs. Your gun's slots and the bag stay
  pinned at the top, the shelf of everything for sale goes below, and a
  fixed info pane on the right shows whatever is hovered or focused. Tiles
  shrink to about half today's size. The names stay on the tiles, but the
  stats and descriptions move to the pane.
- **Armory:** the same frame. The shelf gets tabs because it is a catalogue,
  not an offer. Bullets go in a shape-by-payload grid.
- **Strongest precedents:** the Balatro shop, the Noita wand inventory, and
  Hades II's Arcana / Vampire Survivors' PowerUps (a grid of tiles with one
  fixed detail area).
- **Biggest cheap win:** drop the "Larder World" title block from every page
  except the main menu. Together with the briefing lines, it takes about 220
  of the 565 units on the shop at 200%.

## 1. What today's screens show

I looked at the renders before researching. Units are screen pixels divided
by the UI scale.

- **Main menu.** Nine cards, each about 65 units tall at 150%, plus a 170-unit
  title block. At 150% only six entries show. At 200% only four and a half
  show. The bottom card is cut by the hint line.
- **Options.** Fifteen two-line rows, about 62 units each. It scrolls even at
  100% on 1080p. Notes are cut off with "..." ("anyw...", "Thi...").
- **Shop at 150%** (1280 x 720 units). The title, subtitle and planet
  briefing use about 220 units, which is 31% of the height. Upgrades and
  four big gun cards fill the rest. The gun's slots, the cards for sale and
  the bag are all below the fold. The description box belongs to the
  Upgrades panel, top left, so it is far from the cards.
- **Shop at 200%** (about 1000 x 562). The header uses about 219 units (39%)
  and the buttons and hints about 104. Only the upgrades show. No guns, no
  slots, no cards, no bag.
- **Armory at 200%.** Only the title, the range tools and the description
  box show. At 100% it still scrolls, because the bullet grid has 18 or
  more visible tiles and goes on below the fold.
- **The layout runs the wrong way.** Every page is one column stacked down
  the screen, with wide empty margins to the right. At 16:9 with big text,
  height is the scarce axis, not width.
- **Tiles are bigger than their content needs.** A bullet tile is about
  190 x 62 units at 100% and holds an icon, a name, some pips and a price.
  The owner's 17:41Z note ("the bullet tiles could easily be half the size
  without even losing any info") matches what I measured. A 110 x 56 tile
  holds the same four things if the name wraps to two lines. Gun cards are
  about 290 x 180 because each one draws five stat bars *and* five numbers.

## 2. How the games in my pool do it

V = verified in a source (linked in section 8). M = from memory, not checked
in a source this session. I could not open the Game UI Database
(gameuidatabase.com refuses automated fetches) or Steam screenshots (blocked
here). So most screen *layouts* below are from memory. The verified parts
are mostly counts and facts that appear in text.

### Shops between fights

**Balatro (the shop).**
- V: the SHOP sign is at the top left. The jokers you own are a row across
  the top, with a counter such as 4/5. The money is in a left panel under
  Hands and Discards.
- V: the shop tray is in the middle and has price tabs above each card. The
  voucher has its own compartment, with the booster packs to its right. The
  green Reroll button, which shows its cost, sits under the red Next Round
  button on the left of the tray.
- V: a reviewer: "it's easy to always hover over a card to get full details
  about what it can do."
- V: a mod, JokerDisplay, adds text under each joker that the base game does
  not show. This is the hidden cost of putting details only on hover.
- M: the descriptions pop up beside the card. Clicking a card shows a "Buy"
  button next to it. Opening a booster pack *replaces* the shop tray with
  the pack's cards until you pick one. Nothing scrolls. Nothing ever moves
  between visits.
- **The lesson:**
  - Keep what you own pinned and small, keep the offer small, and keep run
    information in a fixed side region.
  - Show details on demand, in one place.
  - Open a sub-list in place instead of opening a new page.

**Slay the Spire (the merchant).**
- V: the merchant sells 5 coloured cards, 2 colourless cards, 3 relics,
  3 potions and one card removal. That is 13 items and a service.
- M: everything is on one screen. The cards are drawn big enough to read
  their text on the card, so they need no tooltip. Relics and potions are
  small icons with hover tooltips.
- M: the deck view is a separate, scrolling screen. Shopping does not
  require it.
- **The lesson:** a run shop shows the *whole offer at once*. The game keeps
  the offer small so that it fits.

**Brotato (the shop between waves).**
- V: the shop offers up to 4 items, each with a lock under it. The wave
  number is at the top left. Stats have a primary tab and a secondary tab.
- M: the four offers are big cards across the top, with their full text.
  The weapons (6 slots) and items you own are small icons at the bottom
  left, and hovering one opens a popup. The stats panel is pinned on the
  right.
- **The lesson:** offers are big and owned things are small. The player
  reads what they might buy and recognises what they already have.

**Backpack Battles.**
- V: you buy items "to put into their backpack". You can sell, and reroll
  the shop.
- M: the shop sits beside your backpack grid. You buy by dragging an item
  from the shop straight into place, and there is a storage area for
  spares.
- **The lesson:** buying and placing can be one gesture, because the place
  it goes to is on the same screen.

**Monster Train (the merchants).**
- M: each merchant is a separate stop on the route. When you buy a card
  upgrade, the game opens a grid of your deck so you can pick the target
  card.
- **The lesson:** separate "stalls" work when each one is small. The cost
  is a second screen for the follow-up choice.

### Build and loadout screens

**Noita (the inventory and wand editing).**
- V: you can carry up to four wands, each with its own spells. Each wand
  lists its stats with an icon and a label (Shuffle, Spells/Cast, Cast
  delay, Recharge, Mana max, Mana charge, Capacity, Spread). Capacity sets
  how many spell slots the wand has.
- V: you edit wands in the inventory (I or Tab), usually only in the Holy
  Mountain. You can drag spells, or double-click or shift-click one to put
  it in the selected wand.
- M: each wand is a compact panel with its stat list and a row of small
  square slots. The spare spells are a row of small squares above. Hovering
  a spell opens a box with its full stats. The world stays visible to the
  right. The shop is separate: items on pedestals in the world.
- **The lesson:** this is the closest mechanic to ours (cards fired left to
  right). Slots and spares are icon-sized squares, and the detail lives in
  the hover box. A whole build fits in a corner of the screen.

**Hades II (the Arcana).**
- V: there are 25 cards, with the grid on the left and the Grasp total on
  the right (example: "26 Grasp is used out of a maximum of 27"). Each card
  costs 0 to 6 Grasp. Active cards have a highlighted border. One card
  affects "all 5 cards in a row or column", so the grid is 5 x 5.
- M: the right side shows the focused card's name and full description.
- **The lesson:** a fixed grid of compact tiles plus one fixed detail area
  and a budget meter. That is the same problem as our gun's budget and
  slots.

**Vampire Survivors (the PowerUp screen).**
- M: about 20 to 30 small icon tiles in a grid. The selected one's name,
  description, rank and price, and the Buy button, sit in one fixed bar.
  Refund is a separate button.
- **The lesson:** with many items, use small tiles and one detail area.
  The details don't have to be on the tiles.

**Hades (the Pact of Punishment).**
- V: there are 15 conditions with ranks, and each one adds Heat. You open it
  at a window in the armoury before a run.
- M: it is one list, with the focused condition's description and the total
  Heat shown beside it. Nothing scrolls.
- **The lesson:** run-rule knobs live on a "set up this run" screen next to
  the start button, not in Options.

**Slay the Spire (Custom Mode).**
- V: Custom Mode sets the seed, the character and the Ascension level, and
  lets you "enable any number of modifiers". Mutually exclusive modifiers
  switch each other off.
- **The lesson:** run modifiers go with the mode, not in global Options.

**Rogue Legacy 2 (the Blacksmith).**
- M: category tabs for weapon, head, chest, cape and trinket. A grid for the
  chosen category. A detail pane that compares the item with what you have
  equipped.
- **The lesson:** big catalogues use tabs, and the comparison sits in the
  detail pane.

**Places where these games do scroll (M).**
- The Slay the Spire deck view and Compendium.
- The Dead Cells Collector's list of blueprints.
- **The lesson:** pros scroll only long *catalogues* that you browse, never
  a decision screen.

### Main menus and settings (all M)

- **Slay the Spire.** A short text list at the bottom left: Play, Compendium,
  Statistics, Settings, Patch Notes, Quit. Play opens Standard, Daily Climb
  and Custom, each with its description.
- **Balatro.** A row of a few big buttons (Play, Options, Quit, Collection).
  Options is a pop-up with tabs. The Collection uses page arrows ("page 1 of
  N"), not scrolling.
- **Hades.** A short vertical text list.
- **Vampire Survivors.** One big Start button. Power Up, Collection and
  Unlocks are smaller, and Options and Quit are smaller still.
- **Risk of Rain 2.** Settings sit under tabs across the top. I recall a
  description box for the hovered setting, but I am not sure of it.
- **The common rules:**
  - Main menus are short and ranked by how often each entry is used.
  - Rarely used items are smaller.
  - Descriptions appear on the mode-select step or for the focused item,
    not on every row.

### Big text and scaling

- V: Xbox Accessibility Guideline 101 sets a minimum of 18 px at 1080p on
  PC (26 px on console). It asks for scaling "up to 200 percent ... without
  the loss of content, functionality, or meaning". It also allows scrolling
  "in one direction".
- V: Family Gaming Database on Slay the Spire: "the text is quite small".
- M: most games in my pool scale one fixed 16:9 canvas as a whole and offer
  no text-size option.
- **So:** a 75% to 300% UI scale with no scrolling is stricter than both the
  pool and the guideline. It can be done, but only with a fixed frame
  designed for the smallest screen (1000 x 565). At that size some
  information has to move behind hover or focus.

## 3. Rules the precedents agree on

1. **A decision screen never scrolls; a catalogue may page.** The run shop is
   a decision screen. The armory is a catalogue.
2. **The run shop shows the whole offer at once** (Slay the Spire, Balatro,
   Brotato). Fit it by making tiles smaller, not by hiding stock.
3. **Pin what the decision depends on.** Our purchases go into the gun, so
   the gun's slots and the bag stay on screen while you shop (Balatro's
   joker row, Brotato's weapon row, the Backpack Battles backpack).
4. **One fixed detail area.** Hover fills it. With no hover, keyboard focus
   fills it, and after that the selected item.
5. **Fixed positions.** Nothing moves between visits. Folders open in place
   (Balatro's booster pack). No accordions.
6. **Use the width.** Put things side by side. Height is what runs out.
7. **Cap anything that grows** (Noita's four wands; M: Balatro's five joker
   slots, Brotato's six weapons). An unbounded row will break any no-scroll
   layout.

## 4. Proposed layouts

### Common assumptions

- Text sizes are in units and are the same at every screen size: titles 36
  to 48, headings 20 to 22, body 16, small text 14.
- A body line is 24 units tall. A clickable row is 40 to 44 units tall.
- At 16 units an average character is about 8 units wide.
- Between the screen sizes, only padding, tile width and whether names are
  shown change. Text never shrinks.

### (a) Main menu

**M1. Short list with a caption pane (Slay the Spire, Hades).**

```
1280 x 720
+--------------------------------------------------------------------+
| y48  LARDER WORLD (48u)                                             |
| y104 A mould that grows, feeds and fights back (18u)                |
|                                                                     |
| y160 +--------------------+   +-----------------------------+       |
|      |> Run          44u  |   | Run                    20u  |       |
|      |  Campaign          |   | Planet after planet toward  |       |
|      |  Random planet     |   | the slime's home world;     |       |
|      |  Sandbox           |   | upgrades between      16u   |       |
|      |  Gun range         |   +-----------------------------+       |
|      |  ----------- 16u   |   caption pane x448 w400 h120           |
|      |  Options           |                                         |
|      |  Keys              |        (live dish behind)               |
|      |  Edit maps         |                                         |
|      |  ----------- 16u   |                                         |
|      |  Quit              |                                         |
|      +--------------------+  x64 w320, 9 x 44 + 2 x 16 = 428       |
| y680 Up/Down and Enter, or click (14u)                              |
+--------------------------------------------------------------------+
```

- **At 1000 x 565:**
  - The 36-unit title is at y 24 to 60, with no tagline.
  - The list runs from y 84: 9 x 40 + 2 x 12 = 384, so it ends at 468.
  - The caption pane is at x 360, 360 units wide.
  - The hints are at y 530. It fits.
- **Cost:**
  - You read a mode's subtitle only by hovering it or moving to it.
  - It is the most compact layout and it scales to more entries.
- **What can go wrong:** first-time players skim names they don't know
  ("Random planet") without ever seeing what they mean.

**M2. Play stack plus a utility row (Vampire Survivors, Balatro). Recommended.**

```
1280 x 720
+--------------------------------------------------------------------+
| y48  LARDER WORLD (48u)                                             |
| y104 A mould that grows, feeds and fights back (18u)                |
|                                                                     |
| y160 +------------------------------------------------+             |
|      | Run                                       22u  |  72u        |
|      | Planet after planet to the slime's home   16u  |             |
|      +------------------------------------------------+  8u gap     |
|      | Campaign  / 8 missions, one new verb each      |             |
|      | Random planet / ...                            |             |
|      | Sandbox / ...                                  |             |
|      | Gun range / ...                                |             |
|      +------------------------------------------------+  ends y552  |
| y576 [Options][Keys][Edit maps]            [Quit]   44u tall        |
|      3 x 140 + 2 x 10 = 440                120 wide                 |
| y680 hints (14u)                                                    |
+--------------------------------------------------------------------+
  column x64, width 600; the dish shows on the right 600 units
```

- **At 1000 x 565:**
  - The 36-unit title is at y 24 to 60.
  - The column is x 32, 560 wide. Five entries at 64 + 6 = 344, from y 84
    to 428.
  - The utility row is at y 448 to 488: 3 buttons x 130, then Quit 110
    wide.
  - The hints are at y 516. It ends at 532, with about 30 units to spare.
- **Cost:**
  - None in clicks.
  - Subtitles must be 60 characters or fewer to fit one line at 520 units
    ("Planet after planet toward the slime's home world; upgrades between"
    is 67).
- **What can go wrong:**
  - A sixth mode needs 70 more units. There is room for one more, not two.
  - After that, switch to M1 or M3.

**M3. Hub, then mode cards (the Slay the Spire Play step).**

- **The main page has six entries:** Play, Gun range, Options, Keys, Edit
  maps, Quit.
- **Play opens a page of four mode cards.** Each card is 280 x 360 at
  1280 x 720 and 230 x 320 at 1000 x 565. Each holds a name, a picture, a
  description and the mode's own setup, such as the Campaign's mission list
  or the Sandbox's world rules.
- **Cost:** one more click to start anything, including the most common
  choice (Run).
- **What can go wrong:** it feels slow for a game you restart often.
- **The gain:** a natural home for per-mode settings (see Options O3).

### (b) Options

**O1. Two columns of one-line rows plus a note bar.**

```
1280 x 720
+--------------------------------------------------------------------+
| y24  Options (28u)                                       [Back]     |
| y88  +---- col A x64 w560 ----------+ +---- col B x656 w560 -------+ |
|      | UI and text size  [<] 100% [>]| | Wither        [<] 0.01 [>]| |
|      | Sandbox map       [<] fork [>]| | Shared food   [<] 0.05 [>]| |
|      | ...  8 rows x 40u = 320       | | ...  7 rows               | |
|      +-------------------------------+ +---------------------------+ |
| y432 +---------------- note bar x64 w1152 h120 -------------------+ |
|      | Surge (20u)                                                | |
|      | When the slime has eaten enough from its crystals, the     | |
|      | slime near your soldiers ... (16u, full text, no "...")     | |
|      | Default 1800   [Back to default]                           | |
|      +------------------------------------------------------------+ |
| y680 hints                                                          |
+--------------------------------------------------------------------+
```

- **At 1000 x 565:**
  - The columns are 460 wide at x 32 and x 508. Each row has a 260-unit
    label and a 180-unit picker.
  - Eight rows x 40 run from y 64 to 384.
  - The note bar is at y 400 to 490. The hints are at 520. It fits.
- **Cost:**
  - You read a note only by focusing its row.
  - Keyboard order across two columns needs a rule: Up and Down move within
    a column and wrap to the other.
- **What can go wrong:**
  - Sixteen or more settings no longer fit.
  - There are no groups, so a UI setting sits next to a mould rule.

**O2. Tabs, a one-line list and a note pane (Balatro, Risk of Rain 2). Recommended.**

```
1280 x 720
+--------------------------------------------------------------------+
| y24  Options (28u)                                       [Back]     |
| y88  [Screen and controls] [The mould] [Keys]     tabs 40u          |
| y144 +--- list x64 w600 -------------+  +--- note pane x688 w528 -+ |
|      | UI and text size  [<] 100% [>]|  | Control (20u)           | |
|      | Control           [<]  rts [>]|  | Squads: steer one squad | |
|      | Rounds            [<] plain[>]|  | with WASD, fire at the  | |
|      | Bullet speed      [<]  45  [>]|  | cursor ... (16u, full)  | |
|      | ...  up to 8 rows x 44 = 352  |  |                         | |
|      |                               |  | Default: squads         | |
|      |                               |  | [Back to default]  40u  | |
|      +-------------------------------+  +-------------------------+ |
| y680 hints                                                          |
+--------------------------------------------------------------------+
```

- **At 1000 x 565:**
  - The title and tabs share one 40-unit line at y 16 (title 24u, tabs from
    x 180).
  - The list is x 32, 520 wide, y 72 to 424.
  - The note pane is x 568, 400 wide.
  - The hints are at 520.
- **Keys tab:**
  - The group names (Orders, Building, Squads, Camera and menus) are a
    vertical list on the left, 160 wide.
  - The chosen group's keys are a table in two columns of rows 32 units
    tall. Up to 12 rows show at 565, so each group must hold 24 keys or
    fewer.
  - F1 still opens it directly.
- **Cost:**
  - One click to reach the mould rules.
  - The player has to guess which tab holds a setting. Two or three plainly
    named tabs keep the guess easy.
- **What can go wrong:**
  - A setting in the wrong tab gets lost.
  - A tab that grows past 8 rows needs a split.
- **Why I recommend it:**
  - It is the standard pattern on PC.
  - It separates player settings from world rules.
  - It leaves room for new settings.
  - It fixes the cut-off notes.
  - It takes Keys off the main menu. If Keys moves into Options, the M2
    utility row loses a button.

**O3. Options for the player, and a setup page for the world (Slay the Spire Custom Mode, Hades Pact).**

- **Options keeps only:** UI size, Control, Rounds and Bullet speed, plus
  the Keys tab.
- **Sandbox and Random planet open a "Planet setup" page.**
  - The left column has the map and planet number.
  - The right column has the mould rules (Panic, Wither, Shared food,
    Condense, Breakaway, Thicken, Surge and so on) as one-line rows, with
    the same note pane.
  - A big Start button sits at the bottom right.
- **At 1000 x 565:** it is the same as O2 with one more button row (44).
  It fits.
- **Cost:** one more step before a sandbox game.
- **What can go wrong:**
  - The owner tunes these knobs often, and every launch would now pass
    through the setup page.
  - Does Run use any of these knobs? I can't tell from the screens.
- **When to choose it:** if the world rules turn out to apply only to
  Sandbox and Random planet. The main menu's "(Options: Planet)" hint
  suggests that.

### (c) The shop and the armory

**S1. Bench, shelf and info pane, all on one screen (Balatro, Noita, Brotato, Backpack Battles). Recommended for the shop.**

```
1280 x 720
+------------------------------------------------------------------------+
| y0-48  Planet 4 of 8 . Nezanus next . Cut the slime to 2% of its peak    |
|                                                     670 points (22u)    |
+------------------------------------------------------+-----------------+
| x24 gutter 72 | content x104-856 (w752)              | INFO PANE       |
|               |                                      | x872 w384       |
| Gun  y60      | Long gun  Range 35 . Rate 0.6/s .    | y60-636         |
|               | Aim 0.3 . Budget 8/16 . Slots 3 (16u)|                 |
|               | [1 Thrift][2 Slug bomb][3 Std blast] | [icon] Scorch   |
|               |  tiles 144x72, up to 5 = 752         | Modifier . 150  |
| Bag  y184     | [Buckshot gas][    ][    ]  144x72   |                 |
|---- For sale -+--------------------------------------| description     |
| Cards y296    | [Needle][Standard][Scorch][Smother]  | 16u, wrapped,   |
|               | [Wide]   5 x 144x72                  | up to 12 lines  |
| Guns  y380    | [Rifle 240][Carbine 290][Carbine 240]| -------------   |
|               |  3 x 240x72: icon, name, price       | for a gun:      |
| Upgr. y464    | [Armour][Heavier][Faster][Longer]    | Range 19  (-16) |
|               | [Bigger] 5 x 144x72, 2 rows          | Rate 2.2/s (+1.6|
|               | [Supply][Quick][More cards]          | ...             |
|               | [Buildings >][Tools >]  ends y616    | [ Buy . 150 ]   |
+---------------+--------------------------------------+-----------------+
| y652-704 [Land on the next planet 280x48] [Give up the run 200x48] hints|
+------------------------------------------------------------------------+
```

- **What each part does:**
  - **The info pane** follows the hover. With no hover it shows the
    keyboard focus, then the last selected item.
  - **When nothing is chosen** the pane shows the planet briefing. That is
    where today's two briefing lines go.
  - **For a gun for sale,** the pane lists its five stats with the
    difference from your carried gun. The stat bars go.
  - **Buying:**
    - Click a tile to select it, then press Buy (or Enter). The item goes
      into the bag, or into the first free slot.
    - Drag from the shelf to a slot to buy it and put it in place in one
      gesture (Backpack Battles).
    - Drag between slots and the bag to rearrange, as today.
  - **Buildings and Tools** open in place: the upgrade rows become that
    folder's items plus a "Back" tile (Balatro's booster pack). Nothing
    scrolls, and the gun stays visible.

- **At 1000 x 565:**

```
+-------------------------------------------------------------+
| y0-40  Planet 4/8 . Nezanus . cut it to 2% (16u)  670 points |
+----------------------------------------+--------------------+
| gutter 56 | content x72-644 (w572)     | INFO x656 w332     |
| Gun y48   | name + stats line (22u)    | y48-509            |
|           | [slot][slot][slot] 110x56  | name 20u           |
| Bag y140  | [ ][ ][ ][ ][ ] 5 x 110x56 | kind . price       |
| Cards y208| [ ][ ][ ][ ][ ] 5 x 110x56 | description 16u,   |
| Guns y272 | [      ][      ][      ]   | ~11 lines          |
|           |   3 x 186x56               | stats/compare      |
| Upgr y336 | [ ][ ][ ][ ][ ] 110x64     | [Buy . 150] 40u    |
|      y406 | [ ][ ][ ][ ][ ]  ends 470  |                    |
+-----------+----------------------------+--------------------+
| y517-561 [Land on the next planet 240x44] [Give up 150x44]   |
+-------------------------------------------------------------+
```

  - A 110 x 56 card tile holds a 24-unit icon, the name on two lines at 14u,
    the pips and the price. That is about half of today's 190 x 62 tile,
    as the owner suggested.
  - A 110 x 64 upgrade tile holds an icon, a two-line name ("More cards /
    on offer"), the pips and the price.
  - The columns end at 470, leaving 39 units to spare above the bottom bar.
  - At 1920 x 1080 units the same frame keeps the info pane's width and
    gives the extra width to the tiles. Tiles grow to show a one-line
    effect, and gun tiles get their stat bars back.
- **Cost:**
  - Stats and descriptions take a hover or a focus to read, though always
    in the same place.
  - Upgrade and card names are small and wrap.
  - New players have more to learn from icons.
- **What can go wrong:**
  - **The info pane flickers** as the mouse crosses tiles. Fix it with a
    short hold, so the pane keeps the last item for about 150 ms when the
    mouse is over a gap.
  - **The bag outgrows its row.** Five tiles fit at 565. Either cap the bag
    or let its tiles go icon-only at 64 x 56 (8 fit). Decide the cap
    first.
  - **"More cards on offer" adds a sixth card.** The card row then needs
    tiles 92 wide, or a second row, which takes the 39 spare units.
  - **A gun with 6 or more slots** needs slot tiles 92 wide.

**S2. Stalls: a tabbed shelf under a pinned gun and info pane (Rogue Legacy 2, Monster Train).**

- **The frame is the same as S1:** the top bar, the gun row, the bag row,
  the info pane and the bottom bar.
- **The shelf has tabs:** Cards 5 | Guns 3 | Upgrades 8 | Buildings |
  Tools.
- **The tab bar is 40 units tall.** The shelf content is 320 units at
  1280 x 720 and 260 at 1000 x 565.
- **One category at a time leaves room for full tiles:**
  - gun cards about 186 x 220, with stat bars, at 565;
  - card tiles 144 x 96, with a one-line effect;
  - upgrade tiles 180 x 96, with a one-line effect.
- **Cost:**
  - One click per category.
  - Stock in the other tabs is hidden while you shop. Show counts and a dot
    for "you can afford something here" on each tab.
- **What can go wrong:**
  - Players skip tabs and miss the gun that would have won the run.
  - The shop starts to feel like a store menu instead of a decision.
  - It breaks rule 2 above: no run shop in my pool hides its offer behind
    tabs.

**S3. Two pages: Market and Bench (Noita's in-world shop versus its inventory; Slay the Spire's shop versus its deck view).**

- **Market page:** every offer, with names and one-line effects, plus the
  info pane.
- **Bench page:** the gun with its full stat bars, big slots that each show
  a one-line effect, the bag and the info pane.
- **Switching:** Tab toggles the page, and there are two big tab buttons in
  the top bar. A purchase lands in the bag.
- **At 1000 x 565:** each page has the whole middle (about 460 units) for
  one job. It fits easily.
- **Cost:**
  - Buying then slotting means switching pages.
  - You can't drag from the shelf to a slot.
  - You choose what to buy without seeing the gun it goes into.
- **What can go wrong:** the obvious fix is "show a mini-strip of slots on
  the Market page", and that turns S3 back into S1.

**The armory: the S1 frame with an S2-style shelf.**

- **Why it differs:** the armory is a free catalogue (8 guns, 20 or more
  bullets, modifiers, range tools), not a 10-to-20-item offer. Catalogues
  are where pros use tabs and pages.
- **Shelf tabs:** Guns | Bullets | Modifiers.
- **The five range tools** (New rolls, Deeper, Shallower, Empty the gun,
  Empty the bag) move to the bottom bar as buttons.
- **Bullets go in a grid:**
  - The columns are the shapes: Standard, Slug, Needle, Buckshot.
  - The rows are the payloads: bullet, blast, bomb, gas, germ and so on.
  - The tiles are 64 x 48 with an icon and pips. The headers carry the
    name, so each tile needs no name.
  - At 565, five payload rows fit (48 x 5 + a 20-unit header = 260). More
    rows go on a second page with page arrows (Balatro's Collection).
- **This is the one place where a grid saves the most space.** Today's list
  repeats the shape and payload words on every tile.

## 5. Recommendation

| Screen | Layout | Why |
|---|---|---|
| Main menu | **M2** | No extra clicks, the subtitles stay readable, it fits at 565 with room for one more mode. |
| Options | **O2** (move to O3 if the world rules are sandbox-only) | The standard PC pattern. It separates player settings from world rules, fixes the cut-off notes, absorbs Keys and has room to grow. |
| Shop | **S1** | Every run shop in my pool shows its whole offer at once. The gun stays visible because every purchase goes into it. One fixed info pane fixes "the description scrolled away". |
| Armory | **S1 frame, tabbed shelf, bullet grid** | It is a catalogue, so tabs and pages are the norm. The bullet grid removes repeated words. |

- **On the owner's ideas:**
  - **"Compact tiles plus an info bar"** is close to S1. Independent
    precedent supports it (Noita, Hades II, Vampire Survivors, Balatro).
  - Four refinements:
    - the pane is pinned, not floating, because at 200% a floating tooltip
      would cover the slots you are dragging to;
    - it follows the keyboard as well as the mouse;
    - it shows the planet briefing when nothing is selected;
    - it compares a gun for sale with the carried gun.
  - **"Expand and shrink sub-menus"** is an accordion. I found no shop in my
    pool that uses one. It moves things around, which breaks fixed
    positions, and it hides stock. For the run shop I recommend against it.
    Opening a folder in place (Buildings, Tools) is the acceptable form of
    it.

### What I would test with the owner

1. **A blind A/B of S1 against S2,** rendered at the owner's real setting (1920 x
   1080 at 200%, about 1000 x 562), with the key kept off the page.
2. **Five tasks on each layout, counting clicks and wrong turns:**
   - read what Slug bomb does;
   - buy Scorch and put it in slot 2;
   - say whether the Carbine for sale beats your gun;
   - buy one Tool;
   - find how many points you have left.
3. **Whether a pane that follows the hover feels jumpy,** compared with one
   that changes only on click.
4. **M2 against M1** for the main menu: one quick look and one question.
5. **Options:** "change Surge and put it back". This tests whether the note
   pane is found and whether the tab names say where things are.
6. **The owner's cap for the bag.** S1 needs a number.

## 6. The path

1. **Add a fit test first.** Render every page at 1000 x 565, 1280 x 720 and
   1920 x 1080 units. Fail when anything is clipped or a scroll bar appears.
   That stops "no scrolling" from quietly coming back.
2. **Drop the "Larder World" title and subtitle from every inner page.** Use
   a 40-to-48-unit top bar (page name, key facts, points). This alone gives
   back about 130 to 220 units on every page.
3. **Main menu to M2.** It is small and gives quick proof that the frame
   works.
4. **Options to O2 with the note pane,** and Keys as a tab.
5. **Shop to S1, in this order:**
   1. the pinned info pane, which fixes the owner's specific complaint;
   2. compact tiles and the side-by-side frame;
   3. Buildings and Tools opening in place;
   4. drag from the shelf into a slot.
6. **The armory on the same frame,** with the tabbed shelf, the bullet grid
   and the range tools in the bottom bar.
7. **Run the blind A/B of S1 against S2** (step 1 of section 5) as soon as
   both can be rendered. Per the house rule, a visual choice goes up blind.
   S2 reuses S1's frame, so only the shelf would need redoing.

## 7. What the brief got wrong or missed

1. **The title block.** Repeating "Larder World" and a subtitle on every
   page uses about 23% of the height at 200%. With the shop's briefing lines
   it is 39% at 200% and 31% at 150%, all before any content. Professional games
   show the logo on the main menu only. This is the largest single cause of
   the scrolling.
2. **Height is the binding axis.** At 1000 x 565 the screen is 1.77 times
   wider than it is tall. Today's pages stack in one column and leave the
   width empty. Side-by-side regions do most of the fitting.
3. **The shop and the armory have different jobs.** "The same layout" should
   mean the same frame (gun, bag, info pane, bottom bar), not the same
   shelf. A small offer you decide on is not a catalogue you browse.
4. **Things that grow need caps.** The bag, "more cards on offer", folder
   sub-lists, the number of slots and future guns all grow. No no-scroll
   layout survives an unbounded row. Each one needs a cap or a page rule.
   That is partly a game-design decision for the owner.
5. **Keyboard focus.** The game is driven by the keyboard too (Up/Down,
   Enter, Q/E). Details that appear only on hover would shut keyboard
   players out, so the info pane must follow focus.
6. **Options mixes player settings and world rules.** The main menu sends
   Random planet players to Options ("Options: Planet"). In the pool, run
   rules live with the mode (Slay the Spire Custom Mode, the Hades Pact).
   Keys is a settings page, not a main-menu entry.
7. **No scrolling is stricter than accessibility guidance.** XAG 101 allows
   scrolling in one direction. The owner's rule is a fair quality bar. Its
   price is that at 200% some information must sit behind hover or focus.
   The brief should accept that trade openly.
8. **Each gun card shows the same data twice.** It draws five bars *and*
   five numbers. Comparing a gun with the one you carry is what decides a
   purchase. Nobody needs to read four guns' absolute values side by side.
9. **The brief lists no check that the layout fits.** Without an automated
   check at the three sizes, the next added setting or upgrade brings the
   scrolling back.

## 8. Sources

Verified in these sources this session:

- Balatro shop layout: [Pixel Twelve, Balatro shop guide](https://pixeltwelve.com/guides/balatro-shop-prices-slots-rerolls).
- Balatro hover details: [Softpedia review](https://www.softpedia.com/reviews/games/pc/balatro-review-537494.shtml).
- Balatro JokerDisplay mod: [mod description](https://smallgit.dasguney.com/skyline/balatro-mod-index/media/branch/main/mods/nh6574@JokerDisplay/description.md).
- Slay the Spire shop contents: [Slay the Spire wiki, Shop](https://slaythespire.wiki.gg/wiki/Shop).
- Slay the Spire Custom Mode: [Slay the Spire wiki, Custom Mode](https://slaythespire.wiki.gg/wiki/Custom_Mode).
- Noita wands and stats: [Noita wiki, Wands](https://noita.wiki.gg/wiki/Wands).
- Noita controls and wand editing: [Noita wiki, How to Play](https://noita.wiki.gg/wiki/Guide:_How_to_Play).
- Hades II Arcana grid and Grasp: [Maxroll Arcana guide](https://maxroll.gg/hades-2/guides/hades-2-arcana-guide).
- Hades Pact of Punishment: [RPG Site](https://www.rpgsite.net/feature/10287-hades-pact-of-punishment-heat-modifiers-and-how-to-maximize-your-rewards).
- Brotato shop rules: [Brotato wiki, Shop](https://brotato.wiki.spellsandguns.com/Shop).
- Brotato stats: [Brotato wiki, Stats](https://brotato.wiki.spellsandguns.com/Stats).
- Backpack Battles: [Wikipedia](https://en.wikipedia.org/wiki/Backpack_Battles).
- Text size: [Xbox Accessibility Guideline 101](https://learn.microsoft.com/en-us/gaming/accessibility/xbox-accessibility-guidelines/101).
- Slay the Spire text size: [Family Gaming Database](https://www.familygamingdatabase.com/accessibility/Slay+the+Spire).
- Balatro accessibility: [Family Gaming Database](https://www.familygamingdatabase.com/accessibility/Balatro).

Tried without result:

- gameuidatabase.com: the fetch was refused.
- Steam store screenshots: the connection was refused here.
- interfaceingame.com: has none of these games.
- The Vampire Survivors and Rogue Legacy 2 wiki pages do not describe their
  screens.

Everything marked M is my recollection of the games. It should be checked
against screenshots before anyone quotes it as fact.
