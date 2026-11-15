# Brief: how should Larder World's menus and shop fit one screen?

The owner (2026-11-15, 17:31Z) on the start menu, the run's shop and the gun
range's armory, verbatim:

> The new menus are much more readable text size but oh my gosh there are
> some issues. Having to scroll is horrible. That's not professional. How
> many professional games were the literally the main menu on the home
> screen? You have to scroll to get to any of the selections? The shop screen
> also horrible. You can only see the guns in like the first part and you
> have to scroll the bottom to get to anything else. When you click on any
> of the bullets you can't read the descriptions because the part with the
> descriptions is been scrolled away. If there's not enough room for
> everything in one screen then there should be different sub menus. [...]
> You could also really shrink things and then have an info bar give all the
> info [...] I'm sure there are other potential options view. How other games
> do menus and shops and things like this because this is definitely not how
> it's normally done

And a minute later: "That suggestion was just one. Definitely do some
research before you just decide on my first thought as the best option."

**Do not treat the owner's sub-menu or info-bar ideas as the answer.** They
are two candidates among many. Your job is to find out how professional
games solve this, then propose what fits this game.

## The game

Larder World is a native PC game (mouse and keyboard, a GPU-drawn UI of its
own, no web or engine widgets). The player commands squads of soldiers who
shoot a growing slime mould. The main mode is a run: a series of generated
planets, with a shop between planets. The shop sells:

- upgrades with levels: armour, heavier shots, faster trigger, longer
  barrels, bigger landing, supply drop, quick hands, more cards on offer, and
  two folders, Buildings and Tools, that open sub-lists;
- guns, each with five stats (range, rate, aim, budget, slots);
- bullet and modifier cards that go into the carried gun's slots. The gun
  fires them left to right, over and over, Noita-style. The player drags
  cards into slots and between slots, and has a bag of spare cards.

The gun range has an armory with the same layout where everything is free.

The start menu's main page has nine entries: Run, Sandbox, Campaign, Random
planet, Gun range, Options, Keys, Edit maps and Quit, each with a one-line
subtitle. Options has 15 settings, each a row with a left/right value
picker, a note and a "back to the default" link. There is also a list of 8
missions and a key list.

## The hard constraint

The player picks a UI size from 75% to 300%, and the owner wants big,
readable text. One UI unit is a pixel at 1080 lines, times the size. The
size is capped so the screen is never less than about 1000 units wide.

- **The worst case** is about **1000 x 565 units**: 1920x1080 at 200% or
  more, 2560x1440 at 250% or more, or 1280x800 at 175% or more.
- **At 150% on 1920x1080** the screen is 1280 x 720 units.
- **At 100%** it is 1920 x 1080.

The game opens at 1600x972 on a 1080p monitor.

**Everything must fit one screen at every size, with no scrolling**, and
text must stay at the chosen size; the rule today is pages scroll rather
than shrink, and the owner hates the scrolling.

## Today's screens

Renders at several sizes are in
`/tmp/claude-0/-home-user/44a0ac59-f077-5adf-a8b1-37721f37a933/scratchpad/today/`:

- `main-*`: the main menu;
- `options-*`: the Options page;
- `shop-*`: the run's shop, three planets in;
- `armory-*`: the gun range's armory;
- `missions-*` and `keys-*`: the mission list and the key list.

File names end in the screen size and the UI size. Look at them first. Do
not read the game's source code; this is about what the player sees and
what other games do.

## What to do, big picture first

1. **Research how professional games solve this.** Use web search and
   pages such as the Game UI Database (gameuidatabase.com), developer
   talks, wikis and articles. Cover main menus, settings screens, and
   shops, loadout or inventory screens. Look at how they keep everything on
   one screen as content grows, for example:
   - tabs and categories;
   - pages or carousels;
   - sub-screens;
   - master-detail panes;
   - compact tiles with a detail pane or tooltip;
   - grids versus lists;
   - pinned regions;
   - modal pop-ups;
   - fixed layouts designed for a 16:9 frame;
   - how they handle big-text or accessibility scaling.

   Name concrete games and describe their screens. Mark what you verified in
   a source and what you recall from memory.
2. **Propose two or three distinct layouts** for each of these:
   - (a) the main menu;
   - (b) Options;
   - (c) the shop and armory, which is the hard one. It must show the
     carried gun's slots, the bag, what is for sale, and a readable
     description of whatever card is selected or hovered, all at once.

   For each layout:
   - give a box wireframe in text at 1280 x 720 units, with sizes in units;
   - show how it holds at 1000 x 565;
   - say what it costs the player (clicks, hidden information, learning)
     and what can go wrong.
3. **Recommend one layout per screen for this game**, and say why. Say what
   you would test with the owner to decide.
4. **Say what the brief got wrong or missed.**

## Your source pool

Each researcher gets a different pool of games as their main source, so the
proposals are independent. Your pool is given below the brief. You may draw
on others, but say when you do.

Write your full report to the file path given below the brief, in plain
Markdown. Write short sentences and use lists. Your reply should be a summary
of 300 words or less: your recommended layout for each screen, the three
strongest precedents, and the path.
