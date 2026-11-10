# Brainstorm: beyond the first design (2026-11-10)

**Status: brainstorm only, nothing built.** This is the repo copy of the
"Brainstorm" tab of the Claude Doc
https://claude.ai/artifact/EDygsdTpEg8vK7HZzTXhdp, which extends
[`../../rts-design-2026-11-10.md`](../../rts-design-2026-11-10.md). The owner
asked for these six agents on 2026-11-10: two inspiration agents (Creeper
World and Particle Fleet: Emergence; RTS design craft), then a survey of
simulation-based strategy games, a card-mechanics agent, and two clean-slate
agents. The clean-slate agents were added after the owner noticed that the
first four were all re-using the demo's tools. A verification pass then
re-fetched the pages the agents had not been able to read. A red team then
attacked the directions and ran probes on today's dish
([`../red-team/README.md`](../red-team/README.md)); its results are folded in
below, marked *(red team)*.

The strongest new directions came from the two clean-slate agents, which set
the demo's five tools aside: a contagion war over one neutral mould, timed
food waves (Squeeze), and tension on the food sources (Taut). The research
agents agree on the core risk, a match decided in its opening. The card agent
found that the food store is never a bank, which breaks Bloomfront's spending
rule as written.

**After the red team (2026-11-10, later).** Taut fails on today's dish: its
pull collapses the mould onto its piles instead of straightening a road. A
squeeze from home moves food share by nothing, and the pattern counters form a
straight ladder with no cycle at today's density. Two Fevers is the strongest
direction left, and the least built. Four of the five directions score food
share, which mostly counts heads on piles, so the first test for any
direction is whether its verb can move the score at all.

## Directions on the table

| Direction | The player is | Core verb | Why the opening does not decide it | Builds on | Cost | First test (scripted hands, 16 paired seeds) | Red team (2026-11-10, later) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Bloomfront (first draft) | A keeper above the plate | Lure, push, wall, cut, water, each paid from food | Untested, and its currency does not work yet | Today's tools and The Wild One | Medium | Equal hands split food 45-55% | Weak as a direction, useful as a harness. Stage 0's gates would pass by symmetry without testing the opening |
| Squeeze ([clean-slate-player.md](clean-slate-player.md)) | The mould's heartbeat | Tap a held pile to send a wave of sap down your tubes; hold it to charge a bigger one; pinch a tube to send it back | Waves arrive later and thinner over long routes; piles run dry; a charge shows as dimming tubes and invites a counter | Food flow, sap, the pulse, tube walls, the rival. *Corrected (red team):* these exist separately; flow is off by default and never ran with a rival, the pulse is one clock for both moulds, and the fight ignores load | Medium to large | Timed squeezes beat the same squeezes made at random moments on 13 or more | A home squeeze changed share by 0.000 on 3 of 3 seeds; one at the seam pile by +0.001 to +0.004. Flow on two moulds costs 2.6 times the step time |
| Contagion war: Tincture ([clean-slate-phenomena.md](clean-slate-phenomena.md)) and Two Fevers ([clean-slate-player.md](clean-slate-player.md)) | A dye or fever inside one neutral mould nobody owns | Seed your strain, choose the traits that change how carriers move, cut a firebreak | *Corrected (red team):* only Two Fevers has immunity (ground the leader has held turns resistant to them and open to the trailer). Tincture converts by majority, which feeds the leader. Both rely on the host rewiring | Contagion (the GPU plan's is one strain and one bit, no immunity or traits), trait `Params` | Medium to large | Seeding at chokepoints beats random seeding; the early leader sometimes loses | Strongest, as Two Fevers: its verb rides the model and its score is host share. Unbuilt and untested; watch for a comeback rule so strong that skill washes out |
| Taut ([clean-slate-phenomena.md](clean-slate-phenomena.md)) | The pull of the food sources | Set each source slack, even or taut; slack roads bow and loop, taut ones straighten | Sources dry up on a schedule; branches nothing pulls are withdrawn; taut leads are brittle and slack ones costly | The rival, food flow, scent (runs on today's dish) | Medium | A road visibly straightens within 5 s, then a tension planner beats an always-even dial | **Fails.** Raising the pull shrank the network to a third within 5 s with walkers piled on the food (4 of 4 seeds); lowering it starved the mould. A recorded dead end |
| Pattern counters: Grain ([clean-slate-phenomena.md](clean-slate-phenomena.md)) and Grafts ([clean-slate-player.md](clean-slate-player.md)) | A curator of the mould's rules | Give each region its own sensing rules; capture a wild colony's rules and brush them on | Every pattern loses to some other pattern | Per-region or per-walker `Params`; the GPU plan for patterns that look distinct | Large | A no-input matchup grid: every pattern loses to some other on 13 or more | A straight ladder in 164 games, no cycle. The dish is about 5% filled against Jones's 15%, so this says nothing about GPU scale |

Both clean-slate agents reached the contagion war and the pattern counters
without seeing each other's work. Only the knife survived from the old
toolbox, as the contagion war's firebreak. *(Red team:)* both agents had read
the Jenson report, which names exactly these two next steps, so the agreement
is shared input rather than two confirmations.

## Where the research agrees

- **The opening decides too much.** Critics say it of Particle Fleet:
  Emergence and Creeper World 4 ([creeper-fleet.md](creeper-fleet.md)). RTS
  theory calls it snowballing ([rts-craft.md](rts-craft.md)). Our review 2
  measured it in The Wild One. *(Red team:)* review 2's rival had no hands.
  In mirror games on today's dish the likelier failure is a dead draw: two
  cord or two bead moulds ended at exactly 0.500 on every seed because they
  never touched, and catch-up rules would deepen that.
- **Indirect control has to answer reliably and be foreseeable.** Majesty and
  From Dust were faulted where it was not ([rts-craft.md](rts-craft.md),
  [sim-rts.md](sim-rts.md)).
- **Attention should be the scarce resource, not aim.** Today's lamp is held
  for most of a match and needs aim within about 2 cells.
- **Announce every event, early, and in the simulation itself.** From Dust drew
  complaints for threats it did not always warn of (corrected in the
  verification pass). A bloom whose scent arrives before its food is a preview
  made of data.
- **Cuts must bite.** Today a cut heals in 4 s, before anything beyond it
  starves.
- **Pay with the mould nearest the click.** Spending your own mass, as in Osmos,
  thins that patch visibly and answers the store finding below
  ([sim-rts.md](sim-rts.md)).
- **The open gap.** None of the six slime or fungal games surveyed is a
  real-time contest, and none has an army that is also its own supply road
  ([sim-rts.md](sim-rts.md)). *(Red team:)* two titles the survey found,
  Sprawl Grid and Slimepolis, were never opened.

## A card layer, on top of any direction

[cards-crafting.md](cards-crafting.md) recommends **the Rack**. Each player
brings a rack of about ten cards and holds three in a tray; a played card goes
to the back. Cards can be played only near your own fed tube, so a cut also
removes where the rival can act. A few modifier cards bind to the next card
played, the way Noita's wand modifiers do.

Whether the rack is shuffled is open. The agent wanted a fixed order so the
draw is never luck. The owner (2026-11-10 18:25Z) pointed out that luck is part
of the fun of card games. Luck you see before you decide, such as a shuffled
hand, is different from luck that lands after you commit, which the
simulation already supplies.

*(Red team:)* a fixed rack is a sequence a census can solve offline, which
shuffling fixes. Income from eating and play only near your own fed tube are
two snowballs that stack. Many cards will move nothing if share counts heads
on piles, and the layer multiplies verbs before one verb is shown to work.

The runner-up is **the Strain**, a species built from trait cards drafted
between matches. **Recipes**, tools composed in advance like a wand, are the
deepest and the costliest. The agent's example cards lean on the old tools,
so they would be redrawn for whichever direction wins.

## Corrections to the first draft

- **The food store is never a bank.** In a trace of The Wild One every 10 s
  ([storetrace.txt](storetrace.txt);
  `LEVELTRACE=1 mouldprobe level=5 players=idle,warden seeds=1,2`),
  `Dish::store` sat at a median of 0 and never rose above 108, against 1,500
  at the start. With `Params::pieces` on, each piece turns any surplus into
  walkers at once (`src/mould/mod.rs`, where the census shares out food and
  pays for births). So any spending rule needs its own currency first.
- **Verification.** 38 claims the agents could not first source were
  re-fetched: 17 confirmed, 3 confirmed in part, 5 corrected and 13 still
  unverified. None of the corrections changes a ranking. Each note carries a
  "Verification pass" table at its end, and the unverified items stay marked
  in place.

## The cheapest questions to ask next

*Superseded by the red team (2026-11-10, later).* The first list was: a
no-input pattern-counter grid ("if one pattern beats all the others, two
directions fall at once"), a Squeeze timing probe, then a contagion test. The
red team ran the grid: a ladder, but at today's density, where the patterns
do not form, so it cannot kill Grain or Grafts at the scale they need. The
Squeeze probe is not small (five new rules and flow on both moulds), and
timing cannot matter until a squeeze moves the seam.

The red team's order instead:

1. **A paired-clone verb census.** Step The Wild One to 60 s, clone it, apply
   one direction's core verb in one copy only, and run both 30 s on 16 seeds.
   The deciding measure is food share from the action to +30 s, verb copy
   minus control (host share for contagion). To pass: a visible change by
   +5 s on 16 of 16; share moves at least 0.02 the intended way on 13 of 16
   (p 0.02; mirror noise is 0.48-0.52); the same verb at a random place and
   time moves share less on 13 of 16; Holm across the verbs. It needs no new
   rule for three of the five directions, and it killed Taut in minutes.
2. **One contagion strain on the CPU.** Look at front speed and whether a
   front stops at a wound, on 16 seeds. Add the second strain and immunity
   only if fronts travel at a readable 10-40 cells a second and stop at gaps.
3. **In parallel, the pattern grid at Jones's density** (raise the table or
   the inoculum), the only way the counters can still live.

**Missing from every direction** *(red team)*: player count, match length and
a food supply that lasts it (The Wild One runs dry at about 2:10-2:20),
onboarding, why play twice, and whether the rival is an AI or a second human
on the same screen or a network. No human has played The Wild One yet, so
every claim about fun here rests on scripts. Which question to fund first is
the owner's call.

## The notes

| Note | Brief | Sources |
| --- | --- | --- |
| [creeper-fleet.md](creeper-fleet.md) | Creeper World and Particle Fleet: Emergence as inspiration; what not to copy | About 45 pages fetched |
| [rts-craft.md](rts-craft.md) | RTS design philosophy; critique of Bloomfront; 14 proposals | Talks, essays, reviews |
| [sim-rts.md](sim-rts.md) | 20 simulation-based strategy games and their lessons | 23 pages, then the verification pass |
| [cards-crafting.md](cards-crafting.md) | Noita's wands, deckbuilders, real-time card games; the Rack | 15 pages fetched |
| [clean-slate-player.md](clean-slate-player.md) | 21 control models; Squeeze, Two Fevers, Grafts | The model and the Jenson report; 1 page |
| [clean-slate-phenomena.md](clean-slate-phenomena.md) | 21 phenomena, 18 concepts; Tincture, Taut, Grain | The model, the Jenson report and the Jones papers |
| [../red-team/README.md](../red-team/README.md) | Attacks every direction; probes on today's dish | Code, reports, about 200 runs of its own harness |
