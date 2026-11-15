# Menus, shop and command card on one screen: research and options (2026-11-15)

The owner's playtest notes (11-15, relayed):

- **17:31Z:** "Having to scroll is horrible. That's not professional." The
  main menu scrolls, the shop shows only its first part, and a bullet's
  description scrolls away when you click it. Sub-menus and an info bar were
  offered as one idea, and in the owner's words a minute later: "Definitely
  do some research before you just decide on my first thought as the best
  option." The owner also suggested multi-agent reviews.
- **17:41Z:** "The bullet tiles could easily be half the size without even
  losing any info."
- **17:51Z, the main HUD:**
  - Hold fire, Defend and Free fire should sit together, or become one
    button that cycles.
  - Is Burn a line needed?
  - Fill the two empty tiles.
  - Walls go to the build menu, and pumps get a main slot.
  - Defend as it is, or click to defend a spot or building?
  - A Cancel button, or click a building to cancel, destroy and see its
    status?
  - A build or tool that stays selected.
  - Reorganise the build menu.
  - Heal joins the card's actions.

Page for the owner to pick from: https://claude.ai/artifact/Su1g5MdjbDU2P6ic8YhoLT,
source in `page/`.

## How it was done

Three researchers worked independently from one brief
([brief.md](brief.md)). Each had a different pool of games as its main
source, and none read the game's code; they looked at renders of today's
screens at five sizes. Each marks what it checked in a source and what it
recalls. Their reports:

- [roguelikes.md](roguelikes.md): roguelikes, roguelites and deckbuilders.
- [strategy.md](strategy.md): strategy and squad games.
- [big-text.md](big-text.md): console, TV and accessibility guidance, and
  games played from a sofa.

The brief said 15 settings in Options; the page has 17 (Flinch and Plants
came after the count).

## What all three found

- **The worst case is a console's frame.** 1000 x 565 units (1920x1080 at
  200%) is almost exactly the 960 x 540 canvas Microsoft gives Xbox apps;
  the screens need a TV layout, not a shrunk desktop one (big-text).
- **Most of the height goes to chrome.** On the shop at 200%, the title,
  subtitle and briefing take about 220 of 565 units, and the buttons and
  hints about 105. Only the upgrades show.
- **Agreed changes:**
  - the title on the main menu only;
  - one pinned info pane (hover, then keyboard focus, then the selection,
    then the planet briefing);
  - the gun's slots and the bag pinned in the shop;
  - tiles about half size, with stats in the pane and guns compared with
    the carried one;
  - the armory with tabs and a shape-by-payload bullet grid;
  - Options with tabs, one-line rows and a note pane, and Keys as a tab;
  - a maximum for every list;
  - a fit test at the worst sizes.
- **Expanding sub-menus (an accordion):** none of the three found a
  professional shop that uses one. Tabs, or folders that open in place,
  give the grouping without moving things under the cursor.
- **The size cap only checks width** (strategy): a 21:9 monitor at 200%
  gives 1280 x 540 units, and 1000 x 422 at the cap.
- **Options is mostly game rules** (two of three): the mould's rules may
  belong on a setup page at the start of Sandbox or Random planet. That is
  a question for the owner.

## Where they split

The page shows these as blind pairs, so the key stays off it. The key:

| Pair | A | B |
|---|---|---|
| Shop | One board, everything for sale visible: roguelikes' S1 (its pick), strategy's S-A, big-text's C2 | A tabbed counter: strategy's S-B and big-text's C1 (both their picks), roguelikes' S2 |
| Main menu | Names only with one caption box: strategy's M-A and big-text's A1 (both their picks), roguelikes' M1 | The ways to play as cards with subtitles and a utility row: roguelikes' M2 (its pick) |

So two of three picked the tabbed shop and the caption-box menu.

## The command card

Those options are on the page with my takes. The command card has no
researcher behind it: the takes come from reading the code and from games
recalled, not checked.

- **The clear asks are in both cards:**
  - the fire modes side by side;
  - Lay a wall on the Build page;
  - Pump on the main card;
  - Heal, and Guards with it, on the card, which frees about 150 units of
    the bar;
  - no empty tile.
- **Card 1** has three fire buttons, 6 x 2. **Card 2** has one cycling
  fire button, 5 x 2. Take: Card 1.
- **Burn a line** (`Order::Burn`, `Game::target` in
  `src/mould/larder_world/cmd.rs`):
  - The soldiers stand back 0.6 of their range from the line's middle
    (`paths.rs`).
  - They fire at five points along it in turn, whether slime is there or
    not.
  - Parked on Defend, soldiers shoot only slime within `rules.defend` (7.5
    cells). On Free fire they shoot the thickest slime in reach.

  So Burn is the one order that picks where the fire goes. Take: fold Burn
  and "defend a spot" into one Guard order. Click a spot, building or
  crystal to guard it, or drag a line. That order goes to the squad thread.
- **Cancel:**
  - In squad mode a left click fires and a right click sends the squad, so
    clicking to select a building clashes.
  - Take: a hover card plus keys (Delete cancels or demolishes, U upgrades a
    pump, J switches a lure), and click-select when no squad is commanded.
  - Open question for the owner: what demolishing pays back.
- **Sticky build:** Take: not now. Pump on the main card and Shift-click
  (which keeps the tool armed, `Hud::button`) cover it.
- **Build page:** Take: merge Build and Tools into one page. In squad mode
  the 6 build tiles, 5 tools and Back fill the 12 slots, and the keys don't
  clash.

## Files

- `brief.md`: the brief the three researchers had.
- `roguelikes.md`, `strategy.md`, `big-text.md`: their reports, with sources.
- `raw/mockups.py`: draws every mockup as SVG in UI units, to scale. It
  checks each label against its box and writes `page/index.html` from
  `raw/page.html`.
- `page/`: the page, today's screens at 200% (`today-*.webp`, from
  `larder-world look=DIR` with `menu=` and `screen=1920x1080 ui=200%`),
  today's card at 125% (cropped from `../bar-allocator-2026-11-15/page/`),
  and the dish behind the mockups (`dish.webp`, the right side of today's
  start menu at 100%).

## Next

The owner picks. Then:

1. The fit test and the height cap.
2. The shared frame: a one-line header, a bottom bar and the info pane.
3. The pages one at a time, with before and after GIFs.
4. The card.
5. Later: Guard with the squad thread, and pointing at a building with the
   build thread.
