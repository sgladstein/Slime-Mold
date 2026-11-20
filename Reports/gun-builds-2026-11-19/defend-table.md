# Defend test (PC, seeds 1-6, arena=9:guard, 90 s)

Made by `python3 defend_table.py raw/defend-pc.txt`. Nine soldiers hold the slimiest crystal with the game's Guard order and Defend stance. Each cell: median slime removed against the empty deck (seeds better/worse), median change in walkers on the crystal (lower is better), health lost, and soldiers alive at 90 s (the deck's own median, not a change).

| deck | planet 1 | planet 2 | planet 3 | planet 4 |
|---|---|---|---|---|
| seek+scatter+germ.needle | +10238 (6/0) crystal -89 hp -7.0 alive 9 | +1690 (5/1) crystal -6 hp -2.7 alive 9 | +14860 (6/0) crystal -199 hp -2.8 alive 8 | +19984 (6/0) crystal -737 hp -2.0 alive 6 |
| wide+rate+pierce+blast.needle | +2390 (6/0) crystal -91 hp -7.8 alive 9 | +474 (5/1) crystal -9 hp -6.5 alive 9 | +14642 (5/1) crystal -288 hp -0.3 alive 8 | +24775 (6/0) crystal -948 hp -7.9 alive 9 |
| germ.needle | +7587 (6/0) crystal -89 hp -6.6 alive 9 | +705 (5/1) crystal -6 hp -1.1 alive 9 | +14542 (4/2) crystal -168 hp -1.3 alive 6 | +19419 (6/0) crystal -722 hp -1.3 alive 4 |
| wide+trigger+blast.needle+torch | +2496 (6/0) crystal -91 hp -7.4 alive 9 | +715 (5/1) crystal -8 hp -6.1 alive 9 | +14641 (5/1) crystal -250 hp -0.2 alive 7 | +24282 (6/0) crystal -941 hp -7.5 alive 9 |
| seek+wide+rate+blast.needle | +2476 (6/0) crystal -91 hp -8.0 alive 9 | +777 (5/1) crystal -9 hp -6.8 alive 9 | +13938 (5/1) crystal -248 hp -0.4 alive 7 | +24934 (6/0) crystal -948 hp -8.0 alive 9 |
| seek+wide+blast.needle | +2387 (6/0) crystal -91 hp -7.9 alive 9 | +800 (5/1) crystal -8 hp -6.7 alive 9 | +14125 (5/1) crystal -226 hp +0.0 alive 6 | +24664 (6/0) crystal -945 hp -7.5 alive 9 |
| wide+scorch+blast.needle | +2442 (6/0) crystal -91 hp -7.5 alive 9 | +764 (5/1) crystal -8 hp -6.4 alive 9 | +14202 (5/1) crystal -222 hp +0.3 alive 4 | +24529 (6/0) crystal -944 hp -7.0 alive 9 |
| wide+rate+blast.needle | +2379 (6/0) crystal -91 hp -7.8 alive 9 | +450 (5/1) crystal -9 hp -6.5 alive 9 | +14116 (5/1) crystal -284 hp -0.3 alive 5 | +24688 (6/0) crystal -946 hp -7.8 alive 9 |
| wide+blast.needle | +2418 (6/0) crystal -91 hp -7.5 alive 9 | +644 (5/1) crystal -9 hp -6.0 alive 9 | +13926 (5/1) crystal -244 hp -0.4 alive 6 | +24616 (6/0) crystal -940 hp -7.6 alive 9 |
| seek+wide+split+blast.needle | +2420 (6/0) crystal -91 hp -7.9 alive 9 | +846 (5/1) crystal -8 hp -6.7 alive 9 | +14179 (5/1) crystal -223 hp -0.2 alive 7 | +24156 (6/0) crystal -946 hp -7.5 alive 9 |
| wide+split+blast.needle | +2244 (6/0) crystal -91 hp -7.7 alive 9 | +686 (5/1) crystal -9 hp -5.9 alive 9 | +14244 (5/1) crystal -284 hp -0.3 alive 6 | +24387 (6/0) crystal -941 hp -7.4 alive 9 |
| seek+wide+smother+blast.needle | +2344 (5/1) crystal -90 hp -7.9 alive 9 | +756 (4/2) crystal -9 hp -6.5 alive 9 | +13992 (5/1) crystal -229 hp -0.3 alive 6 | +24077 (6/0) crystal -941 hp -7.4 alive 9 |
| wide+pierce+blast.needle | +2408 (6/0) crystal -91 hp -7.5 alive 9 | +534 (5/1) crystal -10 hp -6.1 alive 9 | +13838 (5/1) crystal -246 hp -0.2 alive 7 | +24334 (6/0) crystal -946 hp -7.5 alive 9 |
| blast.needle | +2234 (6/0) crystal -90 hp -6.6 alive 9 | +460 (4/2) crystal -8 hp -4.7 alive 9 | +13342 (5/1) crystal -280 hp -0.1 alive 8 | +24396 (6/0) crystal -924 hp -6.2 alive 9 |
| wide+range+germ.needle | +10616 (6/0) crystal -88 hp -7.4 alive 9 | +5014 (6/0) crystal -8 hp -4.3 alive 9 | +17093 (6/0) crystal -34 hp -1.0 alive 6 | +7270 (6/0) crystal -63 hp +0.0 alive 0 |
| scatter+blast | +1844 (6/0) crystal -88 hp -5.9 alive 9 | +639 (4/2) crystal -5 hp -3.9 alive 9 | +13130 (5/1) crystal -206 hp +0.3 alive 4 | +23850 (6/0) crystal -872 hp -2.4 alive 7 |
| seek+blast.needle | +2193 (6/0) crystal -90 hp -7.2 alive 9 | +476 (4/2) crystal -8 hp -6.1 alive 9 | +14244 (5/1) crystal -278 hp -0.5 alive 6 | +22426 (6/0) crystal -924 hp -6.7 alive 9 |
| wide+blast | +2110 (5/1) crystal -90 hp -7.4 alive 9 | +312 (4/2) crystal -7 hp -6.0 alive 9 | +12626 (5/1) crystal -216 hp +0.3 alive 4 | +23836 (6/0) crystal -911 hp -5.6 alive 9 |
| blast.standard | +1614 (6/0) crystal -88 hp -6.0 alive 9 | +260 (4/2) crystal -5 hp -4.2 alive 9 | +12448 (4/2) crystal -202 hp +0.0 alive 4 | +23531 (6/0) crystal -864 hp -3.2 alive 6 |
| trigger+blast | +1614 (6/0) crystal -88 hp -6.0 alive 9 | +432 (5/1) crystal -6 hp -4.1 alive 9 | +12174 (5/1) crystal -238 hp +0.3 alive 2 | +22064 (6/0) crystal -861 hp -2.5 alive 6 |
| thrift+blast | +1614 (6/0) crystal -88 hp -6.0 alive 9 | +432 (5/1) crystal -6 hp -4.1 alive 9 | +12174 (5/1) crystal -238 hp +0.3 alive 2 | +22064 (6/0) crystal -861 hp -2.5 alive 6 |
| multicast+blast | +1614 (6/0) crystal -88 hp -6.0 alive 9 | +432 (5/1) crystal -6 hp -4.1 alive 9 | +12174 (5/1) crystal -238 hp +0.3 alive 2 | +22064 (6/0) crystal -861 hp -2.5 alive 6 |
| expire+blast | +1614 (6/0) crystal -88 hp -6.0 alive 9 | +432 (5/1) crystal -6 hp -4.1 alive 9 | +12174 (5/1) crystal -238 hp +0.3 alive 2 | +22064 (6/0) crystal -861 hp -2.5 alive 6 |
| quiet+blast | +2179 (6/0) crystal -89 hp -6.1 alive 9 | +441 (5/1) crystal -7 hp -4.3 alive 9 | +11044 (5/1) crystal -200 hp +0.3 alive 1 | +21846 (6/0) crystal -852 hp -2.7 alive 7 |
| pull+blast | +1502 (5/1) crystal -88 hp -5.6 alive 9 | +182 (4/2) crystal -6 hp -3.6 alive 9 | +9787 (4/2) crystal -186 hp +0.3 alive 2 | +21663 (6/0) crystal -838 hp -1.8 alive 6 |
| germ.slug | +6434 (6/0) crystal -88 hp -7.8 alive 9 | -593 (1/5) crystal -4 hp -5.1 alive 9 | +11079 (6/0) crystal -229 hp -6.6 alive 9 | +15344 (5/1) crystal -664 hp -5.7 alive 8 |
| thick+blast | +1886 (4/2) crystal -85 hp -4.2 alive 9 | +396 (5/1) crystal -4 hp -0.8 alive 8 | +4428 (5/1) crystal -234 hp +0.3 alive 0 | +22080 (6/0) crystal -832 hp -1.6 alive 4 |
| blast.slug | +696 (3/3) crystal -86 hp -5.8 alive 9 | -701 (0/6) crystal -4 hp -4.7 alive 9 | +9402 (4/2) crystal -212 hp +0.3 alive 2 | +18104 (6/0) crystal -823 hp -1.6 alive 4 |
| bomb.needle | +828 (5/1) crystal -85 hp -6.2 alive 9 | -1060 (0/6) crystal -7 hp -5.5 alive 9 | +6802 (5/1) crystal -287 hp -1.7 alive 8 | +19219 (5/1) crystal -900 hp -6.5 alive 9 |
| plain.needle | +1957 (5/1) crystal -84 hp -3.9 alive 9 | +324 (4/2) crystal -6 hp -1.2 alive 9 | +4596 (4/2) crystal -166 hp +0.3 alive 4 | +18664 (6/0) crystal -764 hp -1.2 alive 4 |
| gas.needle | +1422 (4/2) crystal -87 hp -5.9 alive 9 | -697 (2/4) crystal -6 hp -4.7 alive 9 | +6464 (4/2) crystal -62 hp +0.0 alive 2 | +16498 (6/0) crystal -742 hp -2.9 alive 8 |
| trigger+blast.standard+gas.standard | +1772 (5/1) crystal -90 hp -4.6 alive 9 | +1356 (5/1) crystal +0 hp +1.7 alive 0 | +7286 (5/1) crystal -186 hp +1.2 alive 0 | +11034 (6/0) crystal -556 hp +0.0 alive 0 |
| heave+blast | +840 (3/3) crystal -86 hp -5.6 alive 9 | -1610 (0/6) crystal -6 hp -5.1 alive 9 | +5157 (4/2) crystal -220 hp +0.3 alive 4 | +11526 (5/1) crystal -869 hp -4.9 alive 9 |
| pierce | +1136 (5/1) crystal -85 hp -4.0 alive 9 | +354 (5/1) crystal -4 hp -1.5 alive 9 | +3119 (5/1) crystal -95 hp -0.3 alive 5 | +9296 (6/0) crystal -468 hp +0.0 alive 0 |
| germ.standard | +2958 (6/0) crystal -82 hp -5.2 alive 9 | +95 (3/3) crystal +0 hp -0.3 alive 6 | +7380 (6/0) crystal -3 hp -0.1 alive 5 | -83 (0/6) crystal +0 hp +0.0 alive 0 |
| split | +1265 (5/1) crystal -84 hp -4.5 alive 9 | +382 (5/1) crystal -5 hp -2.5 alive 9 | +2849 (6/0) crystal -65 hp -1.4 alive 6 | +5828 (6/0) crystal -398 hp +0.0 alive 0 |
| scorch | +1622 (5/1) crystal -84 hp -3.2 alive 9 | +342 (5/1) crystal -4 hp -3.1 alive 9 | +7615 (5/1) crystal -12 hp +0.3 alive 1 | +198 (3/3) crystal +0 hp +0.0 alive 0 |
| delay+blast | +1740 (5/1) crystal -83 hp -5.4 alive 9 | +518 (6/0) crystal -2 hp -2.9 alive 9 | +1450 (4/2) crystal -6 hp +0.3 alive 0 | +3917 (5/1) crystal -105 hp +0.0 alive 0 |
| blast.buckshot | +1292 (4/2) crystal -86 hp -4.2 alive 9 | +261 (5/1) crystal -4 hp -2.0 alive 9 | +4000 (4/2) crystal -19 hp +0.5 alive 1 | +1559 (6/0) crystal -7 hp +0.0 alive 0 |
| rate | +985 (6/0) crystal -76 hp -3.0 alive 9 | +330 (5/1) crystal -4 hp -2.8 alive 9 | +1889 (6/0) crystal -6 hp -0.4 alive 6 | +2577 (6/0) crystal -40 hp +0.0 alive 0 |
| bomb.standard | -649 (2/4) crystal -66 hp -5.5 alive 9 | -1382 (0/6) crystal -4 hp -4.7 alive 9 | +220 (3/3) crystal -246 hp -1.1 alive 8 | +6697 (4/2) crystal -698 hp -5.1 alive 9 |
| bomb.slug | -960 (1/5) crystal -71 hp -6.1 alive 8 | -1540 (0/6) crystal -4 hp -5.4 alive 9 | -582 (2/4) crystal -229 hp -1.0 alive 8 | +7030 (5/1) crystal -686 hp -5.1 alive 9 |
| bait | +1479 (5/1) crystal -4 hp -0.5 alive 4 | +80 (4/2) crystal +0 hp +0.1 alive 4 | +545 (4/2) crystal +0 hp +0.3 alive 1 | -72 (3/3) crystal +0 hp +0.0 alive 0 |
| provoke | +851 (5/1) crystal -14 hp +0.0 alive 0 | -48 (3/3) crystal +0 hp +0.2 alive 4 | +614 (4/2) crystal -5 hp +0.0 alive 2 | +56 (5/1) crystal +0 hp +0.0 alive 0 |
| still | +620 (6/0) crystal -23 hp -1.2 alive 6 | -69 (2/4) crystal -0 hp -0.4 alive 8 | +121 (3/3) crystal +0 hp +0.0 alive 3 | +89 (4/2) crystal +0 hp +0.0 alive 0 |
| range | +70 (4/2) crystal +2 hp +0.0 alive 0 | +410 (4/2) crystal +0 hp +0.0 alive 2 | +192 (3/3) crystal -2 hp +0.4 alive 2 | -115 (2/4) crystal +0 hp +0.0 alive 0 |
| gas.standard | +232 (4/2) crystal -43 hp -2.0 alive 8 | -1166 (1/5) crystal -2 hp -3.2 alive 9 | +1019 (3/3) crystal +0 hp +0.0 alive 1 | +80 (4/2) crystal +0 hp +0.0 alive 0 |
| mist | -10 (2/4) crystal -18 hp -0.5 alive 4 | -388 (1/5) crystal -0 hp -0.8 alive 8 | +386 (5/1) crystal -4 hp +0.2 alive 3 | +160 (5/1) crystal +0 hp +0.0 alive 0 |
| tinder | +0 (0/0) crystal +0 hp +0.0 alive 2 | +0 (0/0) crystal +0 hp +0.0 alive 6 | +0 (0/0) crystal +0 hp +0.0 alive 4 | +0 (0/0) crystal +0 hp +0.0 alive 0 |
| shatter | +0 (0/0) crystal +0 hp +0.0 alive 2 | +0 (0/0) crystal +0 hp +0.0 alive 6 | +0 (0/0) crystal +0 hp +0.0 alive 4 | +0 (0/0) crystal +0 hp +0.0 alive 0 |
| detonate | +0 (0/0) crystal +0 hp +0.0 alive 2 | +0 (0/0) crystal +0 hp +0.0 alive 6 | +0 (0/0) crystal +0 hp +0.0 alive 4 | +0 (0/0) crystal +0 hp +0.0 alive 0 |
| seek | +0 (2/1) crystal +0 hp -0.0 alive 3 | -18 (1/5) crystal -0 hp -0.3 alive 8 | -96 (2/4) crystal -3 hp +0.0 alive 6 | +0 (2/1) crystal +0 hp +0.0 alive 0 |
| smother | -36 (3/3) crystal -2 hp -0.1 alive 3 | -160 (2/4) crystal +0 hp -0.1 alive 7 | +9 (3/3) crystal -0 hp +0.5 alive 0 | -56 (2/4) crystal +0 hp +0.0 alive 0 |
| gas.slug | -186 (2/4) crystal -74 hp -7.5 alive 9 | -1684 (0/6) crystal -5 hp -6.0 alive 9 | -2508 (1/5) crystal -199 hp -6.6 alive 9 | +4107 (4/2) crystal -561 hp -6.0 alive 8 |
| sour | +425 (4/2) crystal -59 hp -1.5 alive 8 | -549 (1/5) crystal -2 hp -0.6 alive 8 | +172 (3/3) crystal -4 hp +0.1 alive 3 | -448 (3/3) crystal -72 hp +0.0 alive 0 |
| germ.buckshot | +2916 (6/0) crystal -63 hp -2.6 alive 7 | -214 (2/4) crystal +38 hp +0.8 alive 0 | -519 (2/4) crystal +0 hp +1.2 alive 0 | -2728 (0/6) crystal +0 hp +0.0 alive 0 |
| spin | +20 (3/3) crystal -20 hp -0.2 alive 2 | -234 (1/5) crystal -2 hp -0.7 alive 8 | -66 (2/4) crystal +0 hp +0.0 alive 2 | -301 (1/5) crystal +0 hp +0.0 alive 0 |
| torch | +320 (5/1) crystal -14 hp +0.1 alive 1 | -199 (2/4) crystal +0 hp -0.1 alive 4 | +7 (3/3) crystal -0 hp +1.7 alive 0 | -850 (0/6) crystal +0 hp +0.0 alive 0 |
| plain.slug | -455 (2/4) crystal -62 hp -6.7 alive 9 | -1282 (0/6) crystal -2 hp -4.9 alive 9 | -3364 (1/5) crystal -202 hp -6.6 alive 9 | +4030 (4/2) crystal -584 hp -4.8 alive 8 |
| blind | -108 (2/4) crystal -19 hp -0.1 alive 1 | -632 (1/5) crystal -0 hp -0.1 alive 6 | -644 (2/4) crystal +0 hp +0.3 alive 0 | -556 (0/6) crystal -20 hp +0.0 alive 0 |
| flare | -774 (1/5) crystal -80 hp -4.6 alive 9 | -1234 (1/5) crystal -1 hp -1.7 alive 9 | -762 (3/3) crystal -0 hp +0.0 alive 2 | -680 (1/5) crystal -0 hp +0.0 alive 0 |
| gas.buckshot | -726 (1/5) crystal +16 hp -0.0 alive 0 | -332 (2/4) crystal +28 hp +0.0 alive 4 | -332 (2/4) crystal +0 hp +0.3 alive 2 | -2787 (0/6) crystal +0 hp +0.0 alive 0 |
| plain.buckshot | +0 (3/3) crystal +3 hp +0.0 alive 0 | -340 (2/4) crystal +37 hp +0.8 alive 0 | -1673 (1/5) crystal +0 hp +1.4 alive 0 | -2757 (0/6) crystal +0 hp +0.0 alive 0 |
| bomb.buckshot | -1691 (1/5) crystal -1 hp -1.3 alive 6 | -1835 (0/6) crystal +19 hp +0.0 alive 2 | +965 (3/3) crystal -43 hp -1.3 alive 8 | -6841 (0/6) crystal -120 hp +0.0 alive 0 |

planet 1 empty deck: removed -388, crystal 95, hp lost 8.5, alive 2
planet 2 empty deck: removed -458, crystal 25, hp lost 7.3, alive 6
planet 3 empty deck: removed 3670, crystal 387, hp lost 7.3, alive 4
planet 4 empty deck: removed 2984, crystal 1148, hp lost 9.0, alive 0
