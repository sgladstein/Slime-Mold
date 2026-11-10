# Brief: playtest two of the eight new-game prototypes (2026-11-08)

You are a play tester for Scott, the owner of this repo (/home/user/Pixel_Physics, a falling-sand
engine). On 11-07 eight small prototype games were built on it, each its own binary, all on
`main` (the tree is checked out at origin/main). Scott has no time to play them. Your job: **play
your two demos for a while the way a fresh player would, answer each demo's open question as
well as a tester can, and suggest concrete improvements.** Suggest only: do NOT change the
games' code, do not commit, do not push, do not open PRs.

## Read first (and only these, plus the demo's own code)
- `/mnt/project-files/game-design/new-game-brainstorm-2026-11-07/prototypes/common.md` (the bar
  every probe was held to: visible consequence within a second, plain words on screen, doing
  nothing loses, random loses to a plan, graded outcomes, always a next thing to do).
- Your demo's brief in the same folder (`N-name.md`).
- Your demo's report `Reports/<name>-probe-2026-11-07.md` (the builder's own findings: read it,
  but test it, don't take it on faith).
- The demo's code: `src/bin/<name>.rs`, `src/<name>/`, and its harness `examples/<name>probe.rs`.
- Do NOT read README.md, PLAN.md, Reports/dead-ends.md or Reports/open-bugs-handoff.md whole; they
  are huge. CLAUDE.md's "it has to feel satisfying" section is the spirit to judge by.

## What "play it" means here
There is no human at the keys, so play through the code:
1. **Look first.** Render the real window's drawing (the harness's gif/png options draw the
   game's own frame) for a do-nothing run and the builder's planned run. Look at the frames
   yourself (Read the PNGs) before forming a view.
2. **Play as a fresh player.** Write your own scripted player(s) against the game's API: what a
   new person does in their first two minutes after reading only the on-screen text (e.g. click
   on the obvious thing, spam the first tool, try the verb at the wrong time). Also try at least
   one "clever" idea of your own that the builder's planned player does not do, and one exploit
   or cheese (does some dumb strategy beat the intended one?). Count outcomes over a few paired
   seeds (stars/score per seed, side by side with do-nothing and the builder's plan).
   - Put scratch harnesses in `examples/zzreview_<name>.rs` (new files only; never edit the
     shipped files). Delete them when you are done.
3. **Read the screen as a player.** Is it clear what each key/tool does, what the goal is, and
   what just happened? Is there a visible answer within a second of each action? Note anything
   confusing, ugly, or inert.
4. A real-window screenshot is optional: `xvfb-run -a -s "-screen 0 1280x800x24" env
   VK_ICD_FILENAMES=/usr/share/vulkan/icd.d/lvp_icd.json PIXEL_PHYSICS_SCREENSHOT_AFTER_FRAMES=60
   ./target/release/<name>` (may need `apt-get install -y libxkbcommon-x11-0 mesa-vulkan-drivers`;
   it never exits on its own: kill it with `for p in $(pgrep -x <name>); do kill $p; done`, never
   `pkill -f`). Skip it if it costs more than 10 minutes.

## Shared machine: 4 CPUs, other testers running at the same time
- The binaries and the eight `<name>probe` examples are being built now. **Wait until
  `/tmp/claude-0/-home-user-Pixel-Physics/6e0126ea-9202-5fd8-b29f-70d21d70b47d/scratchpad/build.log`
  contains a line starting `EXIT`** (check every minute or so with a short `sleep 60` loop
  inside one Bash call; read code meanwhile). If it says `EXIT` non-zero, report that and stop.
- Wrap every cargo build you run in the lock:
  `flock /tmp/claude-0/-home-user-Pixel-Physics/6e0126ea-9202-5fd8-b29f-70d21d70b47d/scratchpad/build.lock cargo build --release --example zzreview_<name>`
  Build only your own example by name; `--examples` builds ~100 and takes over an hour.
  Use `set -o pipefail` when piping cargo.
- Run sims with `RAYON_NUM_THREADS=1` and at most 2 at once. Run from the repo root.
- Never `git add`, `git stash`, `git checkout` or `cargo fmt`; other testers share this tree.
- Never `pkill -f` / `pgrep -f`; use `pgrep -x`.
- Budget: about 60-90 minutes of wall time for both demos. A fast clear answer beats an
  exhaustive one.

## Output (per demo), in `/tmp/claude-0/-home-user-Pixel-Physics/6e0126ea-9202-5fd8-b29f-70d21d70b47d/scratchpad/review/<name>/`
- 2 to 4 pictures that a phone reader can follow: PNG stills or small GIFs (**under 3 MB each**;
  crop, drop frames, or scale down; ffmpeg is installed). Name them so the order reads, e.g.
  `1-nothing.gif`, `2-plan.gif`, `3-fresh-player.png`.
- `review.md`, plain words, short (aim for ~400-600 words), with these sections:
  1. **The open question and my answer** (the question the builder asked Scott, and your best
     tester's answer: yes / no / partly, with the evidence).
  2. **What I played and what happened**: a small table, per strategy (do nothing, builder's
     plan, your fresh player, your clever idea, your cheese) with paired per-seed counts.
  3. **Captions** for each picture: one line each, with the event counts that belong beside it
     (stars, score, how many strikes/cracks/burns fired).
  4. **What feels good / what feels dead** (a few bullets each, judged against common.md's bar).
  5. **Improvements**, ranked, 3 to 6, each concrete enough to build (what to change, why,
     what it should do on screen). Mark the one you would do first.
  6. **Promise score 1-5** (how likely this becomes a fun game) with one sentence why.
- Final message back to me: the two promise scores, the one-line answers, and the file paths.
