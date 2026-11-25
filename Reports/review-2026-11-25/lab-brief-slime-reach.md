# Lab brief: a body that reaches a moving squad

For the Lab lane. Step A of the owner's order (11-25 21:35Z). This is a
prototype on the Lab's own example or branch, not the game. The brief
describes the model, not the game's tools, per the brainstorm rule.

## The problem in one line

A squad that keeps walking is safe from the slime's body. Its own reach is
the panic charge within the guns' range plus about 5 cells (threat audit,
beside this file). Every later fix was a scheduled event added on top.
The balance review of 11-24 said it: "the slime can't reach a squad he
steers."

## The model, described neutrally

- Hundreds of thousands of walkers on a grid. Each senses fields ahead of
  it (its own trail, food scent, alarm, scent with soldiers in it), turns,
  steps and lays trail. Trail and scents diffuse and decay.
- Walkers are born where they are fed, up to a ceiling set by the charge of
  the crystals the body holds. They starve where cut off.
- A tether keeps a walker beside its neighbours, so the body stays one
  piece. Lanes let a walker step into a full cell along heavy trail (the
  Sprawler's threads).
- Shots leave alarm. Nearby walkers flinch or panic-charge within a short
  reach.
- Soldiers walk at speed 10 (world units a second). A walker's step is a
  fixed fraction of a cell each tick. The twist test measured the farthest
  walkers at about 4.9 cells a second.

## The question

From these rules alone, can the body move its mass toward a squad faster
than a squad can leave, so that kiting costs soldiers? It should do this
without a clock or a trigger, and the move should show on screen as slime
streaming.

Directions to try, from the model and from biology as inspiration:

1. **Flow along tubes.** Physarum moves mass by shuttle streaming inside
   its tubes, much faster than its front grows. Let a walker on heavy trail
   step farther (pace scales with trail under it), so the thick roads become
   fast lanes and the mass pours down them.
2. **Wound scent.** Where the body is hurt, lay an attractant for the
   walkers behind the hurt front, not only alarm for the walkers at it. The
   body then sends mass toward its losses, the way it re-routes to food.
3. **Prey scent that leads.** The soldiers' smell is already in the food
   scent. Let it diffuse farther and decay slower than food, so the body
   tracks a squad that moved a while ago, and loses it when the squad gets
   far enough.
4. **Giving up.** A chase that never ends is a timer by another name. The
   body should stop when the scent runs cold or the cost (walkers lost) is
   too high, and the player should be able to see that.

Pick one or two. Don't try all four.

## Deciding measure, named before the run

- **Rig:** planet 2 (Sprawler), run seeds 1 to 6, a squad of 15 with the
  plain rifle, raids, strikers, waves, fruiting, mini-bosses, the late
  lunge and the last stand all off. Only the body's own rules play.
- **Hands:** a kiting hand (attack-move to a crystal, walk away when the
  body comes within the guns' reach plus 10, come back later; the Lab writes
  it), plus idle and greedy as controls.
- **Measure:** soldiers lost to bites by 5:00, kiting against today's body,
  paired by seed, with a sign test. Also slime mass within 20 cells of the
  squad per minute.
- **Pass:** kiting loses soldiers it can't avoid by moving on at least 5 of
  6 seeds. Idle still loses. The streaming is visible at zoom 6 and 12 in
  a clip.
- **Cost:** quote the frame cost of any new field or per-step work.

## Then

Show the owner a clip and a recommendation. If it passes, Direction and
Enemy use the threat audit to merge or cut the comeback events it replaces.
Each one cut gets its dead-ends line.
