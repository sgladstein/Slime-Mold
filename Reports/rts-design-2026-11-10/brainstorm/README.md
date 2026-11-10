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
re-fetched the pages the agents had not been able to read.

The strongest new directions came from the two clean-slate agents, which set
the demo's five tools aside: a contagion war over one neutral mould, timed
food waves (Squeeze), and tension on the food sources (Taut). The research
agents agree on the core risk, a match decided in its opening. The card agent
found that the food store is never a bank, which breaks Bloomfront's spending
rule as written.

## Directions on the table

| Direction | The player is | Core verb | Why the opening does not decide it | Builds on | Cost | First test (scripted hands, 16 paired seeds) |
| --- | --- | --- | --- | --- | --- | --- |
| Bloomfront (first draft) | A keeper above the plate | Lure, push, wall, cut, water, each paid from food | Untested, and its currency does not work yet | Today's tools and The Wild One | Medium | Equal hands split food 45-55% |
| Squeeze ([clean-slate-player.md](clean-slate-player.md)) | The mould's heartbeat | Tap a held pile to send a wave of sap down your tubes; hold it to charge a bigger one; pinch a tube to send it back | Waves arrive later and thinner over long routes; piles run dry; a charge shows as dimming tubes and invites a counter | Food flow, sap, the pulse, tube walls, the rival (all exist) | Medium | Timed squeezes beat the same squeezes made at random moments on 13 or more |
| Contagion war: Tincture ([clean-slate-phenomena.md](clean-slate-phenomena.md)) and Two Fevers ([clean-slate-player.md](clean-slate-player.md)) | A dye or fever inside one neutral mould nobody owns | Seed your strain, choose the traits that change how carriers move, cut a firebreak | Immunity: ground the leader has held turns resistant to them and open to the trailer; the host keeps rewiring | Contagion (on the GPU plan), trait `Params` | Medium to large | Seeding at chokepoints beats random seeding; the early leader sometimes loses |
| Taut ([clean-slate-phenomena.md](clean-slate-phenomena.md)) | The pull of the food sources | Set each source slack, even or taut; slack roads bow and loop, taut ones straighten | Sources dry up on a schedule; branches nothing pulls are withdrawn; taut leads are brittle and slack ones costly | The rival, food flow, scent (runs on today's dish) | Medium | A road visibly straightens within 5 s, then a tension planner beats an always-even dial |
| Pattern counters: Grain ([clean-slate-phenomena.md](clean-slate-phenomena.md)) and Grafts ([clean-slate-player.md](clean-slate-player.md)) | A curator of the mould's rules | Give each region its own sensing rules; capture a wild colony's rules and brush them on | Every pattern loses to some other pattern | Per-region or per-walker `Params`; the GPU plan for patterns that look distinct | Large | A no-input matchup grid: every pattern loses to some other on 13 or more |

Both clean-slate agents reached the contagion war and the pattern counters
without seeing each other's work. Only the knife survived from the old
toolbox, as the contagion war's firebreak.

## Where the research agrees

- **The opening decides too much.** Critics say it of Particle Fleet:
  Emergence and Creeper World 4 ([creeper-fleet.md](creeper-fleet.md)). RTS
  theory calls it snowballing ([rts-craft.md](rts-craft.md)). Our review 2
  measured it in The Wild One.
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
  ([sim-rts.md](sim-rts.md)).

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

1. **Do pattern counters exist?** Run the no-input matchup grid as a sweep of
   the rival's own `Params`. No new rules are needed, and if one pattern beats
   all the others, two directions fall at once.
2. **Does timing beat randomness?** Build a small Squeeze probe on today's
   dish, since food flow, the pulse and the rival all exist.
3. **Can a plan beat chance in a self-rewiring host?** Run a contagion test,
   on the CPU dish or once GPU phase 1 lands.

A red-team review attacks these directions before anything is built. Which
question to fund first is the owner's call.

## The notes

| Note | Brief | Sources |
| --- | --- | --- |
| [creeper-fleet.md](creeper-fleet.md) | Creeper World and Particle Fleet: Emergence as inspiration; what not to copy | About 45 pages fetched |
| [rts-craft.md](rts-craft.md) | RTS design philosophy; critique of Bloomfront; 14 proposals | Talks, essays, reviews |
| [sim-rts.md](sim-rts.md) | 20 simulation-based strategy games and their lessons | 23 pages, then the verification pass |
| [cards-crafting.md](cards-crafting.md) | Noita's wands, deckbuilders, real-time card games; the Rack | 15 pages fetched |
| [clean-slate-player.md](clean-slate-player.md) | 21 control models; Squeeze, Two Fevers, Grafts | The model and the Jenson report; 1 page |
| [clean-slate-phenomena.md](clean-slate-phenomena.md) | 21 phenomena, 18 concepts; Tincture, Taut, Grain | The model, the Jenson report and the Jones papers |
