# The Grudge's readout: hits shrugged off (2026-11-25)

Squad's film (#847) found the Grudge's trait did not read: holding one spot
on planet 7 for 3:00, the HUD's share of the slime on full scar read 0% at
1:00 and 2:00 and 1% at 3:00. Direction asked for the scar to show within
two minutes, around 10-20%.

Setup for every arm: `larder-world look=DIR run=1:7 mode=squads
hand=hold@760,436 set=twist=0 set=boss_first=9999`, 8 soldiers, lavapipe.

**The share of the body on full scar** (the old readout) at 2:00:

| arm | on full scar |
|---|---|
| `scar_tough_full` 2,000 (main) | 0% (1% at 3:00, Squad's film) |
| 500 | 0.5% |
| 250 | 0.6% (1.0% at 2:20) |
| 500, `scar_gain` 0.04 (4x) | 0.9% |
| 500, `alarm_keep` 0.995 (a wider alarm, as a wider scar) | 1.5% |

The scar does form and show on the ground (the dark ribbed patch under the
fight by 1:00), but that patch is the kill zone: the slime on it at any
moment is the 600-1,800 walkers streaming into the guns, out of a planet's
85,000-167,000. No number reaches 10% without scarring a tenth of the map.

**The readout now** (Direction's pick, 11-25): the share of the squad's hits
in the last 10 s that scar tissue shrugged off. The engine counts each blow
the scar turns aside (`wstat[14]`); kills are the shots' and the struck
counter's. Same film: 21% at 0:30, 47% at 0:40, 47-52% from then to 3:00
([still at 2:00](hold-120s.jpg)). No number changed.
