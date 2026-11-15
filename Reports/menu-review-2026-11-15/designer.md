# Reviewer 1: game designer and player — menus and HUD after the 11-15 UI work

Source read: `/home/user/wt-pr1` at d5af42ab (ui.rs, start.rs and draw.rs are the same as
`/home/user/Slime-Mold` at 77101557). Pictures: the brief's renders, plus five frames
I rendered with the release binary (paths at the end).

## The big picture

**Is the approach right?** The engineering approach is sound. There is one layout,
measured in UI units, every page follows the UI size, and a page that is too tall
scrolls instead of shrinking. The weak point is that the layout has no stated
priority for what the player needs. When room runs short, items give way in the
order the layout code happens to reach them. As a result, the things a player
uses most go first, and low-value items keep their space:

- **The command card loses its words at 125% on 1080p.** Its labels are dropped,
  but the squad panel beside it keeps about 450 px of empty glass (my render
  `b-squads-125`). The owner says the text is too small, so the size they are
  most likely to pick removes the most useful text from the HUD.
- **Heal is the first squad button dropped.** The owner made healing an explicit
  order (07:41Z), but it lives in the squad panel's header and is the first button
  left out when the header is short.
- **The gun strip gets smaller than the rest of the bar at large sizes.** It draws
  at 60% of the bar's size, so its words end up smaller than everything around it.
  Its space goes to a preset name ("Plain"), a constant line ("A card a shot"),
  empty card slots and an H button that does nothing in a run.
- **The menus waste the top of the page.** Every menu page, the shop and the gun
  range spend the top 160 units (240 px at 150%) on "Larder World" plus a
  subtitle. At 150% the shop's build area (the gun's slots and the cards) is
  pushed below the fold.

**What is missing for the decisions the game asks of the player:**

1. **The goal and the stars of a run planet are not shown in a usable form.** Run
   planets and missions are the core of the game, and the player cannot see them
   while playing.
2. **In squad mode, nothing tells the player which gun they carry.** The strip says
   "Plain", not "Rifle".
3. **In the shop and the gun range, the gun's slots and the cards are not on screen
   together at 150% and above.** Building the gun is the shop's main decision, and
   you build it by dragging cards into those slots.
4. **Several one-click actions cannot be undone and have no confirmation:**
   - Give up the run.
   - Restart (in a run, this also gives a free retry of the planet).
   - Quit (a run is not saved anywhere).

**What I would do next, in this order:**

1. **Goal bar and star conditions on the HUD** for run planets and missions. This
   replaces the crystal-share pill there. (Small to medium effort; the biggest gain
   in play.)
2. **A priority order for the squad-mode bar, which is the default HUD:**
   - Put Heal and Guards in the card's two empty squad-mode slots.
   - In the gun strip, name the gun, move the loadout name to the tooltip, and hide
     H in a run.
   - Drop the squad header's sub-line.
   - Fix the AUTO tile's state word.
   - Then check that labels survive at 125% on 1080p and on 1440p.

   (Medium effort.)
3. **The hover card in squad mode:** never show it while a mouse button is held,
   and show it in a fixed place rather than by the cursor that is aiming. (Small.)
4. **Shop and gun range:**
   - Pin the gun's slot row above the buttons.
   - Make the gun cards compact, showing the difference from the carried gun.
   - Show the "Larder World" title only on the main page.
   - Show a card's description next to the card.

   (Medium to large.)
5. **Confirmations:** Give up the run, and Restart and Quit during a run. Ask the
   owner whether Restart should exist in a run at all. (Small.)
6. **Menus at big sizes:**
   - Main menu: show a description only for the focused item.
   - Options: UI size first, rows in groups, hide "default".
   - The Sandbox line should not list the UI size as a rule change.

   (Small.)
7. **Clearer symbols:**
   - Separate icons for Hold fire, Free fire, the objective and the crystal share.
   - A lock glyph for locked items, so they don't look like the red "can't afford"
     price.
   - Budget pips that don't look like prices.
   - Rename "Aim" to "Spread".

   (Small.)
8. **Onboarding:**
   - Label the campaign as RTS controls.
   - Give a squad-mode player three first-minute hints.
   - Put real headings on the key page.

   (Small.)
9. **End screen:**
   - The stars earned and the condition that was missed.
   - The run's progress.
   - A small chart of the slime's size over the round, drawn from the
     simulation's own numbers.

   (Medium.)

**Type floor.** Nothing a player must read in play should be under about 14 px at
100% on 1080p. Today some is:

| Text | Size today |
|---|---|
| Key letters | 11.5 |
| AUTO / YOU tags | 11.5 |
| Costs | 12 |
| Hover-card lines | 12 |
| Tile state | 12.5 |
| Gun-strip state | 13 |

Once step 2 keeps the labels at 125%, offer the owner 125% as the default on
1080p. That is the cheapest answer to "the text seems too small". (The sizes are
measured in code; the 14 px floor is my judgement, not a measurement.)

## What I checked and what I guessed

**Checked by looking:** every brief render, plus five frames I rendered:

- squad-mode fight at 30 s with two squads and a three-card gun, at 100%, 125% and
  200% on 1920x1080;
- the run shop at 150%;
- the gun range at 150%.

**Checked in code, with file:line:** every claim below that cites a line.

**Read in code only, not rendered:** the pause menu, the in-game F1 card, the end
screen and the hover card. Its timing depends on wall-clock time, so a
no-window render does not show it reliably.

**Guessed:**

- the type-size norms;
- how the comparison games lay out the same things (from memory of SC2, AoE4,
  Company of Heroes, They Are Billions, Into the Breach, Slay the Spire, Balatro,
  Noita, Hades);
- the claim that quitting loses a run. I found no run save: only the replay and
  the UI size are written (`settings.rs:134`, `main.rs` `save_replay`).

The release binary (08:50) predates the Heal button, so no picture shows Heal.
What I say about Heal comes from the code.

## Findings, ranked by how much they would improve play

### 1. The run's goal and stars cannot be read while playing (run planets, missions)

- **Where:**
  - `draw.rs:795`: the fourth top-bar pill, the crystal share, with the Target icon.
  - `ui.rs:1166-1171`: the objective pill, text only.
  - `campaign.rs:284` and `596-606`: the stars are 1 for the goal, +1 before 4/5 of
    the clock, +1 for the bonus.
- **What the player sees** on planet 1, from `hud-1920x1080-100` and my crop
  `extra/crops/top100.png`:
  - The objective pill says "Wipe out the slime: cut it to 2% of its peak (0%)".
  - The top-left pill also says "0%", with the same target icon. It is your
    share of the crystals' charge, which has nothing to do with a run planet's
    goal.
  - The slime pill says "3088" but gives no peak and no target.
  - Nothing shows the star conditions. The second star needs the goal by 12:00
    on a 15:00 clock. The bonus might be "lose at most N".

  A run pays points by stars (`draw.rs:1131-1135`), so the player is playing for
  stars without seeing them. And because `SHRINK_FROM` is 60 s, the goal reads
  "(0%)" for the first minute with no explanation.
- **Fix:**
  1. When a mission or run tracker exists, replace the crystal-share pill with a
     goal bar: "Slime 3,088, peak 3,088, goal 62", with the 2% mark on the bar.
     The objective pill can keep its sentence and drop the "(0%)".
  2. Beside the clock, show the stars as three pips with their conditions on
     hover, for example "★ goal  ★ by 12:00  ★ lose ≤ 2". Turn a star grey the
     moment it is lost.
  3. Before 60 s, the bar can say "counting from 1:00".
- **Compare:** Into the Breach keeps the mission and bonus objectives on screen,
  and greys a star out as it is lost. They Are Billions always shows its single
  survival goal at the top.

### 2. The squad-mode bar gives its room to the wrong things (the default HUD)

Measured on my renders `a-squads-100` and `b-squads-125` (crops
`extra/crops/a100-bottom.png` and `b125-bottom.png`), and on `e-squads-200`.

**a) The labels go first.**
- **Where:** the decision is made at `ui.rs:1042-1044`:
  `right - wide - 24b - gun_w0 >= LABEL_ROOM (340)`. The gun strip reserves
  `loadout_w`, about 385 units with 4 slots, including a 124-unit name column
  (`ui.rs:109-117`).
- **The arithmetic at 125% on 1080p (1,536 units across):**
  - 1366 (the room left of the minimap) - 737 (labelled card) - 24 (gaps) - 393
    (gun strip) = 212 units, which is under 340, so the card goes square.
  - The squad panel then gets about 830 px, of which the two tiles use about 280.
  - On 1440p the same happens at 125%, where s = 1.67.
- **Fix:** give way in a play-first order: first the squad header's sub-line,
  then the gun strip's name column, then the empty-slot boxes, then the chips'
  key letters, and only then the command labels.
  - With the name column moved to the strip's tooltip, the strip is about
    261 + 8 units, and the labels stay at 125%. (It misses by about 4 units with
    a 4-slot gun, so also trim the 10-unit gap or lower `LABEL_ROOM` in squad
    mode to what the header and two tiles need, about 300.)
  - At 150% and above on 1080p, the labelled card cannot fit in one row. Icons
    plus a visible key letter is an acceptable fallback; SC2's card has no
    labels either. But then the icons must be distinct (see finding 7).

**b) Heal is dropped first, and the card has two empty slots.**
- **Where:**
  - Heal is the fifth header button (`ui.rs:1344`), drawn at the far left and
    skipped when it does not fit (`ui.rs:1348`).
  - The fitting test that steps the header down counts only the other four
    (`ui.rs:1324-1327`), so the header picks sizes that leave Heal no room.
  - Meanwhile `squad_card_key` turns Stop and Rally off in squad mode
    (`squadmode.rs:96`), leaving two empty slots in column 2 of every
    squad-mode render.
- **Fix:** put Heal (L) and Guards (I) in those two slots. The owner ruled that
  Combine and Split stay on the squad panel (11-14 08:22Z); Heal and Guards are
  not covered by that ruling. A hurt squad then has its order on the card in a
  fixed place, greyed with the reason "everyone at full health" when idle. This
  also frees about 130 units of header.
- **Compare:** Company of Heroes, a squad RTS, puts every squad ability on the
  command grid and leaves the squad portraits for selection and status.

**c) The AUTO tile shows the wrong word.**
- **Where:** a squad on its own reads "on its ..." next to its AUTO tag, where its
  activity should be ("firing", "working", "moving"). The tile takes the first
  word of the note (`ui.rs:1422`), and the note starts with "on its own" for an
  AUTO squad (`squadmode.rs:662`). It starts with the formation for a squad in
  line or wedge.
- **Fix:** give `SquadRow` its own `state` field and show that.

**d) The header sub-line repeats the tile.**
- **Where:** "commanding Squad 2, 4 soldiers" (`ui.rs:1363`) repeats what the
  outlined YOU tile shows. It is also cut to "4 s..." at 100%.
- **Fix:** drop it.

**e) The gun strip in a run names a preset, not the gun.**
- **Where:** on run planet 1 it says "Plain / Plain shots" (`draw.rs:1203`), with
  four empty slot boxes. The squad carries a Rifle, but the word "Rifle" is
  nowhere on the bar. The H button is always drawn (`draw.rs:1216`), and in a run
  pressing it only raises "change it in the shop" (`input.rs:20-22`). Its
  tooltip also says "No cards: H or the button for a loadout", which is wrong in
  a run.
- **Fix:**
  - Name the gun ("Rifle") and its two key numbers (range, rate), or the next
    card's name, which is the live information.
  - Set `cycle: None` in a run.
  - In play, show empty slots as thin stubs, since only the shop fills them.

**f) At 200% the gun strip shrinks to 60% inside a full-height panel.**
- **Where:** `ui.rs:1059-1061`.
- **What the player sees** (`e-squads-200`): "Lance / A card a shot" at about
  15 px beside 36 px squad counts, in a tall panel that is mostly empty.
- **Fix:** once (e) removes the name column, the strip rarely needs to shrink.
  When it does, shrink the empty slots first, never the words.

### 3. The hover card is still in the way while aiming (squad mode)

- **Where:**
  - `ui.rs:615-633`: wait 0.35 s, show 2 s, fade.
  - `ui.rs:744-745`: a press puts away the card for the thing under the cursor.
  - `ui.rs:1089`: the key does not depend on a button being held.
- **What the player sees:** suppose a volley starts on bare slime and the cursor
  slides onto the home crystal while the button is held. The key changes, the
  rest timer restarts, and 0.35 s later the card appears by the cursor over the
  slime being shot. That is the 07:54Z complaint in a different order of events.
  The card's text is also RTS wording in the default squad mode: "Right-click
  with soldiers selected to harvest it and hold it" (`draw.rs:906`). In squad
  mode a right click guards and Ctrl+right click drains.
- **Fix:**
  1. No hover card while any mouse button is held.
  2. In squad mode, where the cursor is the gun, show the hover information in a
     fixed spot (a line under the objective pill, or the squad panel's spare
     room) rather than beside the cursor.
  3. Use the squad-mode wording.

### 4. Shop and gun range: the build area is below the fold at 150% and above

- **What the player sees:**
  - At 150% (my renders `c-shop-150` and `d-range-150`), the first screen shows
    the title block, the upgrades and the gun cards (each 262 x 166 units with
    five bars).
  - The "In the gun" slots and every bullet card are below the fold. In the
    gun range, eight gun cards come before them.
  - At 200% (`shop-1920x1080-200`, `range-1920x1080-200`), the armory stacks
    under the upgrades (`start.rs:917-918`), and the first screen shows no gun
    at all.
- **Why it matters:** cards are dragged into slots. If the slots and the cards
  are not on screen together, a drag has to cross a scroll.
- **Where:**
  - `start.rs:668` and `678`: the 44-unit "Larder World" plus the subtitle; the
    page starts at 160 units on every page.
  - `start.rs:917`: the gun card size.
  - `start.rs:1243`: the detail box sits under the upgrades on the far left,
    about 800 px from the card being hovered.
- **Fix:**
  1. Pin the "In the gun" slot row, with its budget meter, just above the
     Land / Back buttons. It should not scroll, so it is always a drop target.
  2. Compact gun cards: icon, name, state, and three numbers on one line with
     green or red differences against the carried gun. Expand them on hover.
  3. On the shop and range pages, replace the title block with a one-line
     header ("Run · Planet 4 of 8 · 670 points").
  4. Show a card's description as a tooltip by the card. The HUD already has
     `card_tip` for this. Keep the left box for keyboard focus only.
- **Compare:**
  - Balatro keeps the jokers you own on screen while you shop.
  - Noita's wand editor shows the wand's slots and the spell inventory together.
    The owner already pointed the team at Noita for spells.
  - Diablo and Borderlands compare an item with the equipped one using
    differences, not separate bar charts.

### 5. Irreversible actions take one click

- **Give up the run** ends the run at once (`menu.rs:569-577`). The button sits
  12 units from "Land on the next planet" (`start.rs:727`). It is also in the
  keyboard's walk order straight after it (`start.rs:434`), so Enter or a slip
  of the mouse can throw away the run.
- **The pause menu's Restart and Quit** act at once (`input.rs:305-313`).
  - In a run, Restart replays the same planet with nothing lost. That is a free
    retry, which a roguelike usually does not allow. **Owner question.**
  - Quit loses the run, because nothing saves it (guessed, see above).
- **Fix:**
  - A confirm step ("Give up the run? You keep nothing. Yes / No"), as Slay the
    Spire does for "Abandon run".
  - In a run, label Restart as "Restart this planet" and ask the owner whether
    it should cost something or be removed.
  - Later: save and quit for a run.

### 6. The menus at big sizes, and the first launch

- **Main menu.** At 200% on 1080p and 1440p, only 4.5 of its 9 items fit
  (`main-1920x1080-200`). The scroll bar sits at the far right edge, about
  850 px from the list.
  - **Fix:** show the sub-line only on the focused item, and fold the list into
    two short groups: Play (Run, Campaign, Sandbox, Random planet, Gun range)
    and the rest (Options, Keys, Edit maps, Quit). At 200%, all nine fit.
- **Sandbox line.** At 200% the Sandbox item reads "Free play, with ui 200%"
  (`start.rs:777-778`). The UI size is not a rule change.
  - **Fix:** exclude display options from `changed`.
- **Options page.** "UI and text size" is the last of 15 options
  (`menu.rs:104-110`), below the fold at 100%. It is the one option the owner
  needed. Under every value, "default" is printed fifteen times
  (`start.rs:872`). Rules of the simulation (Panic, Wither, Shared food,
  Condense, Breakaway, Plants), game setup (map, planet, control, rounds),
  developer (Dev mode) and display are all mixed in one list.
  - **Fix:** put Display first, then Controls, Sandbox setup, Mould rules (lab)
    and Developer as headed groups. Show "default X" only when the value has
    changed.
- **First launch.** The highlighted, Enter-to-start item is Sandbox. A new
  player who goes to the Campaign to learn ("one new verb each") learns RTS
  controls: the missions stay RTS by ruling (07:57Z), where A attack-moves and
  S stops. Everywhere else squad mode is the default, where WASD steers, R
  attack-moves and Esc lets go.
  - **Fix, within the ruling:**
    - The Campaign's sub-line says "RTS controls".
    - The first squad-mode round of a session shows three timed hints: "WASD
      steer", "hold left button to fire", "right click a crystal to guard it".
- **Keys page.** The RTS and squad lists run into one another, with "Squad mode"
  and "RTS round" as ordinary rows mid-column (`menu.rs:152-159`). Both "Esc"
  and both "Right click" rows appear with different meanings.
  - **Fix:** real section headings. Lead with the mode the Options page has set.

### 7. Symbols that mean two things (matters most when the card is icons only)

- **One icon, several meanings:**
  - `Icon::Close` (an X) is Hold fire (`draw.rs:920`), and also Back, Cancel a
    build, the Smother card, Empty the gun and the shop's folder Back.
  - `Icon::Target` is the objective, the crystal share, Free fire and Mine.
  - **Fix:** a hand or crossed-gun icon for Hold fire, and a separate icon for
    the crystal share or the goal.
- **Locked and unaffordable look the same.** On the build page
  (`hud-build-1920x1080-100`, crop `extra/crops/build-card.png`), a locked
  Turret shows its price in red while the bank holds exactly 500. A player reads
  that as "can't afford", and the real reason ("Locked: unlock it in the shop")
  is only in the tooltip.
  - **Fix:** a lock glyph and a grey price for locked items. Keep red for "need
    more food".
- **In the shop:**
  - Budget pips use the same cyan as prices. Give them a different shape or
    colour, and add a budget meter on the slot row.
  - "Aim 0.0" with a full bar is spread, where lower is better. Rename it
    "Spread".
  - The gun state "armory" means "owned". Call it "owned", because the gun
    range already uses "armory" for the whole page.

### 8. End screen (read in code: `ui.rs:2029-2052`, `draw.rs:94-123`)

- **What it shows:** a 420-unit modal with a title, a summary line (soldiers
  left, food, walkers killed, shots, walls) and, for a sandbox round, "Score
  +0.53 (a win is over 1, a loss under -1)".
  - A lost mission says "Mission lost" plus the cause.
  - A won run planet says "On to the shop" and the points earned.
- **What a player wants from it:**
  - which stars they earned and which condition they missed;
  - the run's progress (planet 4 of 8, stars so far);
  - for a loss, what got them: when the squad fell and how big the slime was at
    the time.
- **Fix:**
  - Lines for the three stars, each ticked or crossed with its condition.
  - A small line chart of the slime's size against the goal line over the round.
    This is the simulation's own counts, so it is in keeping with "the picture
    is the data".
  - The run strip as eight dots.
- **Compare:** Into the Breach's mission result, and the run summary in Slay the
  Spire and Hades.

### 9. Smaller notes

- **The pause menu** shows no run state. A line with the planet, goal and stars
  would help (Hades shows your boons on pause).
- **The objective at round start.** For six seconds the objective appears three
  times: the pill, the alert, and the 0% pill. Once the goal bar from finding 1
  exists, the alert can say only what is new ("Planet 1 of 8, Dreskor").
- **The minimap** is about 150 x 89 px at 100% on 1080p, which is small to click
  on. The owner ruled that it stays in the row at the row's height
  (11-14 20:19Z). Raise it only as a question.
- **The squad tile's link and + buttons** are 17-unit squares with 13-unit
  icons, which are small targets for a mouse player. Make them 22 units.
- **At 1280x800 100%** (s = 0.75), the squad header word is gone and the words
  are about 9-10 px. The owner plays on 1080p or larger, so this is low
  priority, but the type floor from the big picture would cover it.

## Extra pictures (all under `/tmp/claude-0/-home-user/44a0ac59-f077-5adf-a8b1-37721f37a933/scratchpad/review/extra/`)

- `a-squads-100/fork-squads-030s-near.png`: squad mode, 30 s, two squads, the
  Lance gun, 100%.
  `larder-world look=… screen=1920x1080 ui=100% mode=squads select=1 hand=squads rounds=lance at=30`
- `b-squads-125/fork-squads-030s-near.png`: the same at 125%. The card goes to
  icons, and the squad panel has about 450 px empty.
- `e-squads-200/fork-squads-030s-near.png`: the same at 200%. The gun strip is
  at 60%.
- `c-shop-150/planet-1-1-rush-001s-near.png`: run shop (`run=1 menu=shop-mid`)
  at 150%. The slots and cards are below the fold.
- `d-range-150/fork-rush-001s-near.png`: gun range (`range=1 menu=shop`) at
  150%. Eight gun cards come before the slots.
- `crops/`: bar crops (`hud100-bottom`, `hud200-bottom`, `hud2560-bottom`,
  `a100-bottom`, `b125-bottom`, `e200-bottom`), the top bar (`top100`), and the
  build card (`build-card`).

Each frame took about 20 s on llvmpipe. Model and HUD cost 0.112 ms a frame at
100% and 0.142 ms at 125% (printed by the runs).
