# Independent review brief: turrets, tech tree and cards (2026-11-16)

You are reviewing another Claude's work for the owner of a game built on a
slime-mould simulation (repo at /home/user/Slime-Mold, Rust). Read
CLAUDE.md first. The owner wants a review that improves the work, not one
that validates it. Lead with the big picture.

## What was done
- The owner asked to explore two ideas for turrets: a tech tree giving new
  turret types and abilities, or turrets that take the gun cards.
- The proposal and the owner's picks are in
  Reports/turret-growth-2026-11-16/README.md and page.html. The owner picked
  "both": a run-shop tree unlocks turret bodies (proposed: mortar, sprayer,
  fan), and gun cards fill each body's barrel.
- First step built (PR #400, branch claude/project-thread-nvxdod; diff it
  against origin/main, but ignore the walls commits from PR #398 it sits
  on): every turret fires one shared turret deck (Game::turret_deck,
  Cmd::Load(TURRET_DECK,..)). Files: src/mould/larder_world.rs,
  larder_world/buildings.rs, shots.rs, turn.rs, src/mould/rounds.rs, and
  src/bin/larder-world/{main,headless}.rs.
- Context: Reports/gun-system-review-2026-11-16/README.md (the card system's
  plan) and wiki/world/larder.md (buildings, raids, the kill box).

## Questions, in order of importance
1. Is the design direction right for the owner's goals? The goals are fun
   over fidelity, the mould integral to play, several strategies that
   counter each other, Noita-like combos, and no turtle-only win. What would
   you change about the bodies, the shared deck, the card bag, or the order
   of steps? What did the proposal miss: other directions, precedents from
   tower-defence, RTS and roguelike games, or simulation-native turret ideas?
2. What do the tests and films miss? There was one game per deck and one
   scene. Name the confounds and the control or paired measure that should
   come next, and how big a run it would need.
3. Correctness of the code: bugs, edge cases (dead soldiers, `by` None
   meaning a turret everywhere it is read, the squads' own decks, replays,
   GPU vs CPU worlds, budget/slots, multicast/trigger/expire on turrets,
   Pull on turret shots), and anything that silently changed the plain
   turret's behaviour.
4. Ranked next steps, each with a first experiment or build.

Biology is inspiration, not the bar. Don't edit files or push. Report in
plain prose under 900 words, findings ranked, with file:line for code
issues.
