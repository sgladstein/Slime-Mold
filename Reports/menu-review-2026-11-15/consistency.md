# Reviewer 3: interface consistency and structure

Larder World's menus and HUD as of `77101557` (the merge of the one-row bar and UI-size work, 11-15). Code is in `/home/user/Slime-Mold`. Line numbers are from that checkout. The worktree `d5af42ab` changes no UI layout.

## The big picture

**The approach is right.** The HUD is a rules-free `Model`, it sends `HudEvent`s back, and one UI unit scales everything. The start menu is a separate module drawn over the live dish. That structure is sound for a game and worth keeping.

**What is wrong is that there is no shared vocabulary.** Each screen was fixed on its own, in answer to its own complaint. So each one fits, but the screens no longer agree with each other:

| Thing | How many ways there are now |
|---|---|
| Text sizes | 20 distinct sizes across the two files: 10.5 to 24 units, plus the 44-unit title. Every half-step from 10.5 to 17 is used. In `ui.rs` that is 17 sizes in 24 size-and-weight pairs; in `start.rs`, 15 sizes in 22 pairs. |
| Colours | 85 hard-coded RGBA literals (about 70 distinct), on top of the 17 theme tokens. The primary-button gradient is copied in two files. |
| Corner radii | About a dozen, from 2.5 to 16 units |
| Button heights | 22, 28, 38, 42, 44, 50, 52-78 and 66 |
| Showing a hotkey | 6 (listed in finding 2) |
| Where "Back" is | 5 |
| What happens when space runs out | 3: shrink, scroll, or truncate with "…" |
| Key cards | 2, built in different ways. One of them is a hand-written list. |

**The shrinking is why "the text is too small" keeps coming back.** Almost every fitting path makes text smaller: `fit_px`, the gun strip's 60% floor, the squad header's 13.5→12 step, the in-game key card's `s *= 0.92` loop (which goes down to 0.4×), and campaign rows that shrink before they scroll. The clearest case:

- On 1920×1080, the gun name is **22 px at 150%** but **17 px at 200%**.
- That is because the strip draws at 0.6× of the bar unit once it is squeezed.
- So turning the UI size up makes the gun's name smaller.

**What is missing:**
1. A very small design system:
   - text roles
   - a spacing scale
   - one button component and one keycap
   - one tooltip
   - one page frame (header, footer, scrolling body)
2. One table of key bindings that feeds the buttons, the tooltips and both key cards. Then a key that is shown is a key that works.

**What I would do next, in order:**
1. Make the keys shown on the HUD tell the truth (finding 1).
2. Stop the bar shrinking text below its role size (finding 3).
3. Use one keycap and one button component across the HUD (findings 2 and 5).
4. Lay out the command card natively as 6×2, grouped by kind (finding 4).
5. Give the start pages one page frame: compact header, fixed footer with Back and hints, scrollbar beside the panel, and a detail panel instead of cut-off notes (findings 6 and 7).
6. Build both key cards from one bindings table (finding 9).
7. Make Back, Esc and right-click mean "back one level" everywhere (finding 10).
8. Do the mechanical token pass for sizes, colours and radii (finding 15).

## What I checked and what I guessed

**Checked by looking at renders.** I looked at every picture in `review/renders`: main, campaign, options, keys, shop and range at 100% and 200%; the HUD at 1280×800, 1920×1080 (100/150/200%) and 2560×1440 200%; the build page; and RTS mode.

I also rendered three more scenes (two pictures each) into `review/extra/`:
- `shopbuild/`: the shop's Buildings folder.
- `tipcmd/`: the command tooltip, hovering Attack-move.
- `tipsquad/`: the squad header's tooltip, hovering Split.

Bottom-bar crops of the last two are in `scratchpad/r3crops/`.

**Checked by reading code.**
- All of `src/world/ui.rs` and `src/world/start.rs`.
- In `draw.rs`: the command cards and their pages, the help rows and the loadout.
- `squadmode.rs` (keys, help, squad panel), `menu.rs` (options, the key list), `events.rs` (key routing), `input.rs` (H, Shift+H, Ctrl+H) and `settings.rs` (`UiSize::px`).

**Not rendered, read from code only.** There is no headless switch for these, so I did not see them:
- the pause menu
- the in-game key card (F1)
- the end screen
- the hover card
- the squad-mode build page

**The binary is older than HEAD.** `target/release/larder-world` was built at 08:50 and does not contain the squad panel's **Heal** button, which merged in `4a9a28d6` (strings check: 0 hits for "Heal: the squad stops"). The current code adds a fifth header button that the renders do not show.

**Guessed.** How crowded the squad header is in HEAD with Heal added. I worked it out from the step-down code (`ui.rs:1328-1361`): at 100% on 1080, Heal fits only by pushing the "commanding Squad 1…" subtitle out.

## Screen map

Text sizes are in UI units: 1 unit is 1 px at 1080 lines and 100%.

| Screen | How you get there | How you leave | Keys | Text sizes and colours | Buttons |
|---|---|---|---|---|---|
| **Start: Main** (`start.rs:776`) | Launch; Main menu from pause or end screen; Esc from a sub-page | Pick a row; Quit | Up/Down or W/S, Enter or Space, F1 or ?, Ctrl ± (Esc does nothing) | Title 44; subtitle 17 MUTED; row 18 SemiBold over 14 MUTED; hint 13.5 MUTED | 9 rows, 500 wide, 66 tall (Quit 50). Sandbox is always primary blue. Scrolls at 200% on 1080: 4.5 rows visible. |
| **Campaign** (`:811`) | Main › Campaign | Back button below the list (secondary); Esc; right-click | 1-9 or Enter starts; arrows | Rows 18/14; map and enemy tag 13 MUTED at top right, hidden when the row is under 70 units | 8 rows 52-78 tall (they shrink before scrolling); Back 140×42 |
| **Options** (`:834`) | Main › Options | Back (primary, below the panel); Esc; right-click | Left/Right or A/D change; Backspace resets; Enter turns forward; **a click on the row resets it** | Label 16.5; note 13.5 MUTED, cut with "…"; value 15 (WARN when changed); "default" 12.5 FAINT | 15 rows × 62; `<` `>` 30×28; Back; All to defaults. **UI size is the last row.** |
| **Keys** (`:1270`) | Main › Keys; F1 or ? from any start page | Back (primary, always drawn focused); Esc; F1; right-click | — | Key 15 ACCENT; action 15 TEXT | 2 columns of at least 480 units (right column cut with "…" at 100%), otherwise 1 column and it scrolls. Hand-written list (`menu.rs:118-164`). |
| **Run shop** (`:971`) | Main › Run; end screen "On to the shop" | Land on the next planet (primary); Give up the run; Esc goes to Main | Arrows or WASD move linearly through a 6-column grid; Enter buys; Q/E move a card | Heading 20; lines 14.5 MUTED; purse 22 ACCENT; section heads 14.5 MUTED; tile 12.5→11.5; gun name 17, bars 12.5; bullet name 13.5→11.5; detail 17/14 | Laid out at 1.3× and shrunk to 1.0×, then scrolls. At 200% on 1080 the armory goes **below the fold**. |
| **Shop folder** | Click the Buildings or Tools tile | **Back tile, top left** (X icon, ‹); Esc; right-click | as the shop | breadcrumb "Upgrades › Buildings" 14.5 MUTED | Same tiles as the shop |
| **Gun range armory** | Main › Gun range, then H in the round | Back to the range (primary); Leave the range; Esc or right-click returns to the range | as the shop | as the shop; "Free" where the purse goes; heading "Bullets" where the shop says "For sale" | 5 action tiles drawn as upgrade tiles ("Empty the gun" cramped at 11.5) |
| **HUD top bar** (`ui.rs:1108`) | in play | — | — | Values 15 SemiBold; rate 12; objective 14; resource labels only in a hover tip | Resource pill, objective pill, clock with speed chip, ☰ button |
| **Alerts** (`:1824`) | events, refusals, toggles, the round's intro | fade after 6 s | Space jumps to one (RTS only; in squad mode Space is taken first) | 14; **always the ⚠ icon** | up to 5 |
| **Bottom bar: gun strip** (`:1610`) | always | — | **H** (its own button) | Name 15, state 13, both × `gb`, which can fall to 0.6× | 4 slots of 46; H 30×30 |
| **Bottom bar: squad panel** (`:1310`) | squad mode | — | Y, J, I, (L), Esc on the buttons; 1-9 on the tiles | Header 16; subtitle 13.5; buttons 13.5→12; tile key 13, count 19, state 12.5, tag 11.5 | Guards, Split, Combine, Let go, (Heal), stepping down by width (`:1328`); tiles 86-136 × 54 |
| **Bottom bar: RTS selection** (`:1183`) | RTS mode | — | 1-9 groups (chips above the panel) | Header 16; subtitle 13.5; unit cards 54 | group and idle chips 24 tall |
| **Command card** (`:1498`) | always | Pages: Build (Q) and Tools (E); Back is the same key, **at the bottom right** | its letters | Labelled form: name 13→11.5, **key 11.5 bottom right**, cost 12. Square form (150% and up): **key 11 top left**, cost 10.5 | 6×2; 116×42 labelled, or 42 square |
| **Targeting** (`:1854`) | a Point, Area or Line command | click places; right-click or Esc cancels | — | cursor chip 13.5 ("Turret: click to place, right-click to cancel") | — |
| **Tooltips** | hover | — | — | 5 styles (finding 5) | — |
| **Pause menu** (`:1957`) | Esc or ☰ | Resume (primary); Esc | **Esc or F1 only: no arrows, no Enter** | Title "Paused" 22; "Esc to resume" 14; buttons 15.5 | Resume, Controls, Restart, Main menu, Quit (340-wide modal). **No Options or UI size.** |
| **In-game key card** (`:1974`) | F1 or ?; Pause › Controls | Back (primary, **bottom right** of the modal); Esc; F1 | — | Title "Controls" 20; rows 14.5 | Up to 3 columns of 600; **shrinks to fit, down to 0.4×** |
| **End screen** (`:2029`) | the round ends | Play again, On to the shop or End the run (primary); Main menu; Quit | **takes every key and acts on none** (`:906`) | Title 24; lines 13.5 MUTED, centred | 420-wide modal, buttons 42 tall |

## Findings, ranked by how much fixing them would improve play

### 1. Some keys shown on the HUD do not do what they say (squad mode, every round)

`squadmode.rs:74-80`, `draw.rs:1004-1018`, `events.rs:134-150`.

The card's letters are routed before the squad's own keys. The build page (Q) uses **T, L, U, I, J, Y**: Turret, Lure, Pump, Outpost, Lures on/off, Siren. Those are also the squad keys Formation (T), Heal (L), Guards (I), Combine (J) and Split (Y).

**What a player sees:** while the build page is open, the squad panel still shows "Split Y", "Combine J" and "Guards I" (and "Heal L" in HEAD). Pressing Y places a Siren, J switches the lures off, I places an Outpost. The comment at `squadmode.rs:79` already notes the L clash.

**Other keys with more than one meaning:**

| Key | Meanings |
|---|---|
| R | Rally point (RTS); Attack-move (squads); Mine (squad tools page) |
| C | Free fire; Det cord (tools page); Cancel build (build page) |
| Esc | Cancel placing; let go of the squad; open the menu |
| Space | Jump to the newest alert (RTS); take command again or bring the camera back (squads) |
| H | Next loadout (sandbox); open the armory (range); refuse with a warning (run). See finding 8. |

**Fix:**
- Make it a rule: a key glyph is drawn only where that key does that thing on the current screen. While a card page is open, `squad_tiles` draws its buttons without keys, or greys the keys.
- Or move the build page off those letters.
- Add a test that walks every card page and asserts no shown key is shadowed by another.

### 2. Hotkeys are shown six different ways, and their place moves with the UI size

| Where | How the key is shown | Code |
|---|---|---|
| Command card, labelled | bottom right, 11.5, MUTED | `ui.rs:1546` |
| Command card, square (150% and up) | top left, 11, TEXT at 72% | `ui.rs:1572` |
| Squad header buttons | inline, "Split  Y", in the same colour as the label | `ui.rs:1320` |
| Gun strip | the key is the whole button: "H" | `ui.rs:1696` |
| Command tooltip | a keycap chip with an outline | `ui.rs:1789-1795` |
| Shop, pause menu, start menu rows | no key shown; keys only in a hint line | — |

**What a player sees:** going from 100% to 150% moves every key letter on the card from the bottom-right corner to the top-left one. Squad buttons read like two words ("Let go Esc").

**Fix:** one `keycap(c, rect_corner, key)` glyph: Caption size (12), MUTED on a faint fill. Always in the **top-right corner** of a button that has a key, or trailing a text button. Use the same glyph in the tooltip and on the key cards. Add keycaps to the pause menu (Esc on Resume) and to the campaign rows (1-8).

### 3. Text on the bar shrinks below a readable size, against the owner's 07:56 ruling

The 07:56 ruling says a page taller than the window scrolls rather than shrinking.

| What shrinks | Where | Effect at 200% on 1080 |
|---|---|---|
| Gun strip | `ui.rs:1056-1064`, down to 0.6× of the bar unit | name 17 px and state 15 px, while the squad buttons beside it are 23 px |
| Squad header | `ui.rs:1328-1332` | drops "1 squad", then its keys, then whole buttons (Guards first) |
| Command labels | `fit_px` 13→11.5 (`:1553`) | — |
| Shop tile and bullet names | `start.rs:1148`, `:1225`, down to 11.5 | — |
| In-game key card | `ui.rs:2003-2010`, `s *= 0.92` down to 0.4 | — |

At 2560×1440 200%, the header dropped "1 squad" to make room, but then filled that room with the subtitle cut to **"commandin…"** (`:1333`, `:1363-1369`). So the room it freed was spent on a word fragment.

**Fix:**
- Give every text role a floor. When there is no room: drop secondary text first, then wrap, then elide with "…" and put the whole text in the tooltip. Never draw under the role size.
- Gun strip: keep the one row (owner 07:51Z) by shrinking the **slot cells** and hiding the name column below a width. The tooltip already carries the name. The text keeps its size.
- Squad header: never draw a subtitle once the header word has been dropped.
- In-game key card: scroll it, as the start pages do.

### 4. The command card is a 4×3 grid poured into 6×2, so groups break across rows

`ui.rs:121-124` (`card_slot`), `draw.rs:916-967` and `1005-1018`.

**Main page:**
- The three stances are split up: Hold fire (Z) and Defend (X) end row 1, and Free fire (C) starts row 2.
- In squad mode two slots on the left of the card are empty (where Stop and Rally were), so the most-used verbs sit next to blank holes.

**Build page:**
- Row 1 is Turret, Lure, Pump, Outpost, **Lures on/off**, Siren. The toggle sits between two buildings.
- Row 2 is blank, blank, **Cancel build**, blank, blank, Back.
- Back is one column right of where Build was, so clicking the same spot twice does not toggle the page. On Tools it does.

**Fix:** give `Command::slot` 6×2 coordinates and lay out by kind.
- **Row 1 is orders:** Attack-move, Stop or (squads) Heal, Drain/Harvest, Burn, Buy, Wall.
- **Row 2 starts with the stances Z, X, C**, which matches the keyboard's bottom row. Then Rally or (squads) Guards, then Build (Q) and Tools (E) at the right.
- On a page: things to place go on row 1; page actions go on row 2 (toggle, Cancel, Back), with **Back in the slot of the button that opened the page**.
- Mark Build and Tools as folders, with the shop folder tile's ›, so they look like they open a page.
- Put a small page title on the card ("Build ›").

Moving Heal and Guards into the card is the owner's call: the 08:22Z ruling asked for Split and Combine buttons on the squad panel.

### 5. Five tooltip styles for one job

| Tooltip | Size | What it has | Code |
|---|---|---|---|
| Command | 340 wide | title 15, keycap, cost, body 13.5, red reason, "Click to place it" | `ui.rs:1776` |
| Gun card | 320 wide | coloured frame and a kind chip | `:1744` |
| Loadout | 300 wide | — | `:1708` |
| `tip_small` | one line, no title | — | `:1175` |
| Hover card | — | fades after 2 s | `:1903` |

`tip_small` is used for the squad header buttons and tiles, resources, group chips and the idle chip. For Split it is a single 510-px line across the bar, with no title and no key (render `tipsquad`). The command tooltip is a titled card above the command card (render `tipcmd`). The shop has no tooltips; it has a docked detail box instead (`start.rs:1243`).

**Fix:** one `tooltip(title, key, cost, body, reason, hint)` component, used for every button, tile, chip and card. The shop's detail box becomes the same component, docked. Keep `tip_small` only for the cursor chip while placing.

### 6. Start pages spend the top of the screen on the brand, and the most-wanted option is last

**The title takes the top of every page.** `start.rs:668-678` draws "Larder World" at 44 plus a subtitle on every page, and content starts at 160 units. That is 15% of a 1080-line screen at 100% and about 28% at 200% (304 px). In the shop, the heading and its two lines take another ~100 units.

**What a player sees at 200% on 1080:**
- The main menu shows 4½ of its 9 rows and scrolls.
- The shop's armory is entirely below the fold.
- At 2560×1440 200% the right half of the screen is empty.

**Fix:**
- Show the 44-unit title on Main only.
- Sub-pages get a one-line header: "‹ Options" at 24, clickable as Back.
- A fixed **footer bar** holds Back, the page's primary action and the hint, instead of a Back button after the content that scrolls away (`:830`, `:879`, `:1294`).

**Options page problems (`start.rs:834`):**
- "UI and text size" (`menu.rs:110`) is the **15th of 15 rows**. At 200% you scroll the full page to reach the setting you were changing.
- Notes are cut with "…" while 1,100 px to the right are empty.
- **A click on a row resets it to its default** (`start.rs:558-563`). Everywhere else a click activates.

**Options page fix:**
- Group the rows: Display (UI size) first, then Play (Control, Map, Planet, Own mould, Rounds), Slime (Bullet speed, Panic, Wither, Shared food, Condense, Breakaway, Plants), Developer (Dev mode). Use section heads in the shop's heading style.
- A click turns the value forward. Reset is a small ↺ beside the value, plus Backspace.
- Show the focused row's full note in a **detail panel** to the right, as the shop does. The same pattern serves the main menu: short rows, with the focused row's description in the panel. Then 9 rows fit at 200%.

**The Sandbox row shows command-line key names.** `start.rs:777-778` lists *every* non-default option by its key: "Free play, with ui 200%", "mode rts", "energy_flow 0.1". UI size is not a sandbox setting. Use the labels, and leave out display and dev settings.

### 7. Scrolling and scroll cues are inconsistent

- **Three overflow strategies.** The start pages scroll. The campaign rows shrink (52-78) and then scroll (`:815`). The shop shrinks from 1.3× to 1.0× and then scrolls. The in-game key card shrinks to 0.4×. The keys and options pages cut text with "…".
- **The scrollbar is at the screen's right edge** (`start.rs:716`, `sw - 12·s`). At 200% on 1080 that is about 860 px from the main menu's list: the list ends at x≈1041 and the bar is at x≈1897.
- The only sign of more content is a row cut off at the clip.

**Fix:**
- One rule: never draw below the UI size; scroll.
- The scrollbar sits on the scrolling panel's right edge.
- Add a 24-unit fade at the clip edges.
- The wheel scrolls the panel under the pointer.
- The keyboard focus follows into view, as it already does.

### 8. The gun strip's H button lies in a run

`draw.rs:1216` always sets `cycle: Some(H)`. In a run, pressing H only raises a warning ("In a run the gun holds the cards you bought…", `input.rs:20-23`). The empty strip's tooltip note still says "No cards: H or the button for a loadout" (`draw.rs:1212`). In the gun range the same button opens the armory instead.

**What a player sees in a run:** an enabled button that only refuses.

**Fix:** in a run, draw no swap button, or a disabled one whose tooltip reason is "Change the gun in the shop between planets". In the range, give it a label ("Armory"). Otherwise follow the keycap rule.

The strip is also missing the **gun's name and budget**. The shop leads with "Long gun · carried · budget 8 of 16"; the HUD shows only a 18-unit grey icon and "Plain / Plain shots", which says the same thing twice. Show "Long gun · Plain" on line 1 and "budget 0 of 10" on line 2.

### 9. Two key cards, one of them hand-written; duplicate rows; three names

**Start menu Keys** is a literal list (`menu.rs:118-164`). It can drift from the real bindings. For example, its "E Tools page: blast, det cord, mine" leaves out germ and sleep gas. It has "Squad mode" and "RTS round" as **rows styled as keys** where section heads should be, and Esc and Space each appear twice with different meanings.

**In-game F1** (`ui.rs:1974-2025`) is assembled from `help_base`, a fixed tail, the current card page's commands and `m.help` (`draw.rs:1142-1162`). In squad mode that gives **Esc twice** (`squad_help` plus the tail, `ui.rs:1987`), **Q twice** and **E twice** (the card plus the help rows). The rows also change with whichever card page is open.

**Names:** the start menu says "Keys"; the pause menu and the card's title say "Controls"; the hint and the rows say "key card".

**Fix:**
- One bindings table, holding each key's label, action and section (Squad, Orders, Stances, Build, Tools, Camera, Game), feeds the start page, F1, the buttons' keycaps and the tooltips.
- On the start page, show only the active Control's set, with a toggle for the other.
- Call it "Keys" everywhere.
- F1 scrolls instead of shrinking.

### 10. Back, Esc and right-click mean different things on each screen

**Back is in five places:**
- after the content (campaign, options, keys; it scrolls out of view)
- pinned buttons at the bottom (shop, range)
- a grid tile at the top left (shop folder, `start.rs:1031`)
- a card button at the bottom right (HUD build and tools pages)
- the modal's bottom right (F1 card)

**Keyboard:** the pause menu ignores arrows and Enter (`ui.rs:953-955`). The end screen takes every key and acts on none (`:906-908`). The start menu is fully keyboard-driven. **Right-click** goes back on start pages, but not in the pause menu or on the F1 card.

**Fix:** one rule.
- Esc and right-click go back one level everywhere. On the end screen, Esc leads to Main menu and Enter takes the primary button.
- Every modal and page walks with Up/Down and Enter, sharing `StartMenu`'s focus code.
- Back is always in the header (start pages, shop folders: make the breadcrumb clickable) or in the opener's slot (card pages).

### 11. Focus, selection and "primary" overlap

**Four focus styles:**
- start items: an ACCENT outline at −2, radius 12
- options rows: a white fill plus an outline
- shop tiles: an outline at −2, radius 11
- the HUD: no focus at all

**Primary blue doubles as a highlight.** Sandbox stays filled blue when the focus moves to Run, so two rows are lit. The focus ring also starts on the first item and stays visible while you use the mouse, so the hover and the ring are on different items.

**Fix:**
- Primary fill only for a page's one committing action: Land, Resume, Play again. On Main that is no row, or Run when a run is live.
- Focus is one style: a 1.5-unit ACCENT ring at −2, shown only after a key press, like CSS `:focus-visible`.
- Hover is the lighter face.
- Selected or on is an accent edge with a tinted fill (the command card's `on`).

### 12. Locked looks the same as unaffordable

`draw.rs:1052-1055`: a locked building gets `ready = false`, so its price is drawn **red**, exactly like "not enough food".

**What a player sees** (render `hud-build`): Turret 500 in red while the bank holds 500. Pump 500, which is unlocked, is cyan. The reason appears only after a press, as an alert.

**Fix:** draw a lock glyph instead of the price on locked commands (the shop already uses ✓ for owned). Keep red for "cannot afford" only.

### 13. Icons and alerts carry more than one meaning

- **✕ (`Icon::Close`):** Hold fire, Back on both card pages, Cancel build, the shop folder's Back tile, and the range's "Empty the gun".
- **◎ (`Icon::Target`):** the objective, the share %, Free fire and Mine.
- **Crystal:** food, Harvest/Drain, Pump and Sow.

**Every alert uses the ⚠ icon** (`ui.rs:1837`), including neutral feedback ("Guards while working: some", "Formation: line") and the green "core taken" alert.

**Fix:**
- One meaning per icon on a screen: ‹ for Back; a crossed crosshair for Hold fire; a pie or flag for the share.
- The alert icon goes by kind: info ⓘ, good ✓, warning ⚠, danger ⚠ in red.
- A toggle's state belongs on its button or tile, not in an alert. Formation has **no control at all**; it shows only on the tile's state text and in an alert. Give it a cycle button beside Guards.

### 14. The same information is shown twice, and some is missing

**Shown twice:**
- The round's objective is in the top pill *and* in the first alert ("Planet 1 of 8, Dreskor: Wipe out…", `main.rs:1047-1048`), on screen together for 6 s.
- The squad header's "commanding Squad 1, 4 soldiers" repeats the tile beside it (1, 4, YOU).
- "Plain" and "Plain shots" say the same thing.
- The shop's heading "Planet 4 of 8, Nezanus next:" is followed by "…Next: Nezanus, a river valley planet".
- "Larder World" is on every page.

**Missing:**
- Feedback when the UI size changes. Ctrl ± changes it silently (`uisize.rs:47-55`). On 1080 lines **250% and 300% look the same as 200%** (capped at 1.9× by `UiSize::px`'s `most`), while Options shows "300%".
- A way to change the UI size from the pause menu. It has no Options entry.
- The current card page has no title.
- The campaign shows no completion.
- The HUD shows no gun budget.

**Fix:**
- Show a toast "UI 125%", with "(this screen tops out at 190%)" when capped.
- Add Options to the pause menu (UI size at least).
- Take the objective out of the intro alert.
- Drop the squad header's subtitle; the tile says it.

### 15. Tokens: sizes, colours, radii, words

- **Colours:** 57 literal colours in `ui.rs` (48 distinct) and 28 in `start.rs` (25 distinct).
  - The primary button is copied in `ui.rs:1944` and `start.rs:754`.
  - The tooltip face is copied 4 times (`ui.rs:1732`, `1755`, `1786`, `1913`).
  - A price you can afford is ENERGY on HUD costs and shop tiles, but ACCENT on gun cards, bullet cards and the detail box.
- **Words:**
  - Keys / Controls / key card
  - Combine / merge (the Options note "split and merge") / join
  - Rounds (option) / loadout / cards / bullets / shots
  - Sandbox / Free play
  - "Squad mode" / Control: squads
  - Range "Bullets" / shop "For sale"

**Fix:**
- A `theme` with roles: PRIMARY(+HOVER), TIP_TOP/BOTTOM, ON_TOP/BOTTOM, FACE_OFF, PRICE_OK=ENERGY, PRICE_NO=BAD.
- A glossary row in `wiki/`, using the owner's words: Keys, Combine, Bullets.

## A small set of shared rules

1. **Text roles** (UI units at 100%). Nothing is drawn below its role.

   | Role | Size and weight | Used for |
   |---|---|---|
   | Caption | 12 | key glyphs, costs, tags, pips' labels |
   | Body | 14 Medium | descriptions, tooltips' bodies, hints, notes |
   | Label | 15 SemiBold | buttons, tiles, rows, values |
   | Title | 18 SemiBold | panel and tooltip titles, the gun's name |
   | Heading | 24 SemiBold | page and modal titles |
   | Display | 40 | Main's title only |

   The 20 sizes in use snap to these six.
2. **Spacing and shape.**
   - Spacing steps of 4: 4, 8, 12, 16, 24, 32.
   - Panel padding 12; gap between panels 8; between buttons 6; between sections 16.
   - Radii: 6 for small controls (chips, keycaps, arrows), 10 for buttons, tiles, cards and tooltips, 14 for panels and modals.
3. **One button component** in a shared `widgets` module used by both `ui.rs` and `start.rs`.
   - Kinds: primary (one per screen), secondary, tile (icon + label), folder tile (›), text chip.
   - States: hover (lighter face), focus (ring, after a key press only), on (accent edge and tint), disabled (40% ink, a lock or reason in the tooltip), pressed.
   - Heights: 28 (chip or arrow), 42 (button or card cell), 66 (menu row with sub-line).
4. **Keys.** One keycap glyph, top-right of its button. It is shown only where it works now. Every key comes from one bindings table, which also builds both key cards.
5. **Back.** Esc and right-click go back one level everywhere. Back sits in the page header, or in the slot of the button that opened the page. Every modal and page can be walked with Up/Down and Enter.
6. **Overflow.** Drop secondary text, then wrap to two lines, then elide with the full text in the tooltip, then scroll the page. Never shrink below the role size. The scrollbar sits on the panel edge, with a fade at the clip.
7. **Pages.** On start pages, the header is "‹ Page" at Heading size (Main has the Display title). The body scrolls. The footer is fixed and holds Back, the primary action and a hint built from the bindings. Description goes in a docked **detail panel** (shop, options, main, campaign), not in cut sub-lines.
8. **Tooltips.** One component, title + keycap + cost / body / reason / how-to, anchored above the element and kept on screen. The shop's detail box is the docked form.
9. **Colours and icons** come from theme roles only, with one meaning per icon on a screen. An alert's icon follows its kind.

## The order to bring each screen to the rules

The order is by how much each step improves play, with the cheap steps first within each.

1. **HUD bottom bar.** Rules 4, 1 and 6, then 3. Fixes findings 1, 2, 3, 8, 12 and 4.
   - Keys that tell the truth; the keycap glyph; the floors (gun strip slots shrink, text does not); the H button in a run; lock glyphs.
   - Then the native 6×2 card, the folder look for Build and Tools, and the squad header in the shared button component with the same tooltip.
   - This is the screen played every minute. A test should assert that no text on the bar draws under 12 units at 100%–200% on 1080 and 1440 lines.
2. **Pause menu, F1 key card, end screen.** Rules 4, 5 and 6. Fixes findings 9, 10 and 14.
   - The bindings table, sectioned and de-duplicated; F1 scrolls; keyboard walking; Enter and Esc on the end screen; Options (UI size) in the pause menu; the UI-size toast.
3. **Start pages: Main, Options, Keys, Campaign.** Rules 7, 6 and 3. Fixes findings 6, 7 and 11.
   - The compact header; the footer with Back, primary action and hint; the scrollbar on the panel; the detail panel for Main and Options; options grouped with Display first; click turns and ↺ resets; Sandbox's line uses labels; focus-visible.
4. **Run shop and gun range.** Rules 7, 1 and 9.
   - At 200% the page is only about 1,000 units wide: 1,010 on 1920×1080 at 1.9×, and 1,004 on 2560×1440 at 2.55×. The stacking rule (`start.rs:918`) needs 559 units beside the 531-unit upgrade panel, so the armory always goes below the fold. A smaller header does not change that.
   - The width problem has two fixes. One is narrower parts: a 4-column upgrade grid (361 units) and 220-unit gun cards fit side by side in about 1,000 units. The other is two tabs, Upgrades and Armory, at large sizes. Either is better than an armory you have to scroll to find.
   - Folder Back becomes the clickable breadcrumb.
   - Prices are ENERGY or BAD only.
   - "Bullets" and "For sale" agree.
   - The range's action tiles become secondary buttons with Label text.
5. **Token pass across all screens.** Rules 1, 2 and 9, findings 13 and 15. Mechanical, and safe to land alongside steps 1-4: sizes to roles, colours to theme roles, radii to 3, icons de-overloaded, alert icons by kind, the glossary.

Each step is a look change, so it lands with tests and renders at 100%, 150% and 200% on 1080 and 1440 lines. No census is needed (build first, balance later). Quote the HUD's frame cost before and after step 1, as the 11-15 review did (0.155 ms).
