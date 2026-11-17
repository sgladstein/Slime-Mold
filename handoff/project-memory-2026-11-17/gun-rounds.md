---
name: gun-rounds
description: Gun cards, squad guns and rolled bullets: #291 rolled bullets; #303 gun order drag; #318 short card text; gun range #325; gun look in [[gun-look]]; owner rulings 11-14/15
metadata:
  type: project
  modified: 2026-11-14T19:22:11.393Z
---

Rounds = Noita-style gun cards, src/mould/rounds.rs. Guns = src/mould/guns.rs. Bullets = src/mould/bullets.rs. Details live in the reports; this is the index.

Merged: bullets #191, Noita look #219/#236, trails #244, Range #257, scorch burn + bait #273, budget #275 (Card::cost, rounds::fit, HUD "Cost N of B"), rate/thrift/smother #278, squad guns #283 (GunKind, Gun::roll(seed,depth), Squad::gun/deck, Cmd::Arm/Load, gun=, Shift+H, rounds::Deck, guns::Loadout), no recharge #288 (Soldier::next, Bullet::cast, free fire, split copies gun, normal rolls, no lucky; page YPVWVXcwbPaPQsTGzRvDZF).

Owner rulings 11-14 (rows in wiki/decisions-game.md):
- 16:46Z squad owns the gun; deeper planets roll bigger budgets, better stats. Bullets: damage + 0.25-4x multipliers (speed, range, spread), sets bias them, cost from the roll plus luck.
- 18:05Z: payloads ARE bullets. NO RECHARGE. Both halves of a split copy the gun.
- 18:07Z: luck = the normal distribution itself. 18:08Z: scattergun 2-6 bullets. 18:18Z: "Nothing needs to be labeled lucky".
- 18:38Z: "The bank is just to build the gun." Firing costs no bank.
- H loadouts stay as test presets.

Rolled bullets #291 merged 19:40Z; owner playtesting. Card::Ammo(Ammo) = shell (plain/blast/bomb/gas/germ) + set (standard/slug/needle/buckshot) + damage, speed/range/spread; cost = base x grade^1.5 x luck; Card::Plain new; old payload cards = standard bullets. Ctrl+H rolls a test load; ammo=N or ammo=ROUND for films. Page 5LDB46fERtjeUTL9Bexz6U, report rolled-bullets-2026-11-14.md. Roguelike thread was sent the API.
Open: plain bullets cost only 1-2 on budget-10 scale (finer budget for balance pass); plain-set differences hard to see at point-blank fronts; GPU strike box caps heavy plain strikes at ~1.4x (R_MAX 7).
Owner 20:09Z: hit effects "too much" -> PR #296 shot lights 2.5 cells (SHOT_LIGHT; bullet was 7), bursts 1.5 radii; page 5AXYwqMaM1ztLkcm83bX8v. Other wave = plant fire from every blast (#290 + no recharge), sent to plants thread cmsg_01PcwPdssrGxiXYvyCG8WaUXWqAyC8qQ4TM2NdtPPPpKm8; plants=0 turns plants off.
Owner 20:22Z: every effect's light = its damage size -> #299 merged 21:03Z (Shot::r, MUZZLE_LIGHT 1).
Owner 20:32Z "I don't see how to move cards around" -> gun fires slots left to right (Run::gun unsorted; Run::put_in keeps the old sorted place for clicks/buys), shop drag to slot / bag, for-sale drag buys into a slot (Run::buy_into), Q/E; merged #303 21:46Z; headless drag=FROM>TO films; page BY96kekDTamkdCkQSFSPgu, report gun-order-2026-11-14.md.
Owner 11-15 02:57Z "text for all the gun cards is too much" -> #318 merged 11-15 ~03:30Z: one sentence a card (draw::card_face), kinds Bullet/Next bullet/Always (CardKind::label); page RLCRS3fDkoTjrDyryTf8J3. UI thread (cmsg_01PcwPdssrGxiXYvyCG8WaUXDJ58rTuLEuNfxu3qt3aNLi) owns start.rs/ui.rs layout and font sizes; we own wording; new shop buttons go in shop_page's foot item() calls.
Owner 11-15 02:55Z "a test environment ... play with the guns and cards and bullets" -> gun range: Run::range (free armory, rack guns swap per kind, New/Deeper/Shallower rolls, Empty gun/bag), start-menu row Gun range, Args.range / range=SEED, H in the range opens the armory over the paused round (App.armory), Back re-arms every squad. PR #325 merged 11-15 04:51Z; page LJVbRQFTfJc52NLRyWHpsD; report gun-range-2026-11-15.md; films: range=7 menu=shop drag=...go (go closes the armory at the first whole second; never pause the headless armory: App::step holds still).
Gun look pass 11-15: see [[gun-look]].
Next: trigger card. Untimed on 1070: Range's big sight disc, scattergun on blast.
