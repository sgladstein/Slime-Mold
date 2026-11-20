# Re-rank after One burst per bullet (#532), PC, seeds 1-6

Moving arena (`pairs_table.py`) then defend (`defend_table.py`), from `raw/rerank-pc.txt` (build d045b77, which also has 5be6b0b, shots stop at set slime).

| deck | planet 1 | planet 3 | planet 4 |
|---|---|---|---|
| wide+pierce+blast.needle | +12184 (6/6) vs blast.needle +1806 (6/6) | +16846 (6/6) vs blast.needle +4960 (6/6) | +21348 (6/6) vs blast.needle +9174 (6/6) |
| wide+rate+pierce+blast.needle | +12185 (6/6) vs blast.needle +1808 (6/6) | +17723 (6/6) vs blast.needle +5052 (5/6) | +19403 (6/6) vs blast.needle +7154 (6/6) |
| wide+blast.needle | +12185 (6/6) vs blast.needle +1806 (6/6) | +16372 (6/6) vs blast.needle +4960 (6/6) | +19924 (6/6) vs blast.needle +1674 (6/6) |
| seek+wide+rate+blast.needle | +10308 (6/6) vs blast.needle +1509 (5/6) | +13784 (6/6) vs blast.needle +2554 (5/6) | +23585 (6/6) vs blast.needle +12710 (6/6) |
| wide+rate+blast.needle | +12185 (6/6) vs blast.needle +1808 (6/6) | +17723 (6/6) vs blast.needle +5052 (5/6) | +17752 (6/6) vs blast.needle +6819 (6/6) |
| seek+wide+smother+blast.needle | +13961 (6/6) vs blast.needle +4333 (6/6) | +10352 (6/6) vs blast.needle +659 (4/6) | +20338 (6/6) vs blast.needle +4539 (6/6) |
| wide+split+blast.needle | +13959 (6/6) vs blast.needle +4333 (5/6) | +11640 (6/6) vs blast.needle +1266 (4/6) | +18902 (6/6) vs blast.needle +5990 (5/6) |
| wide+trigger+blast.needle+torch | +12166 (6/6) vs blast.needle +826 (6/6) | +13916 (6/6) vs blast.needle +2162 (4/6) | +17386 (6/6) vs blast.needle +5792 (5/6) |
| wide+scorch+blast.needle | +12184 (6/6) vs blast.needle +1808 (5/6) | +8577 (6/6) vs blast.needle +287 (4/6) | +19022 (6/6) vs blast.needle +5442 (5/6) |
| seek+wide+split+blast.needle | +11902 (6/6) vs blast.needle +1808 (6/6) | +8242 (6/6) vs blast.needle +1467 (5/6) | +19632 (6/6) vs blast.needle +6868 (5/6) |
| seek+wide+blast.needle | +12183 (6/6) vs blast.needle +1806 (6/6) | +7697 (6/6) vs blast.needle +224 (4/6) | +18932 (6/6) vs blast.needle +3716 (4/6) |
| seek+blast.needle | +11604 (6/6) vs blast.needle +1112 (4/6) | +9306 (6/6) vs blast.needle -330 (3/6) | +15994 (6/6) vs blast.needle +1665 (4/6) |
| seek+scatter+germ.needle | +14762 (6/6) vs germ.needle +0 (1/6) | +12302 (6/6) vs germ.needle +2458 (4/6) | +9065 (6/6) vs germ.needle -84 (3/6) |
| wide+range+germ.needle | +15183 (6/6) vs germ.needle +0 (1/6) | +13719 (6/6) vs germ.needle +6840 (5/6) | +6819 (4/6) vs germ.needle -1806 (3/6) |
| wide+blast | +13758 (6/6) | +11668 (6/6) | +8448 (6/6) |
| trigger+blast.standard+gas.standard | +12136 (6/6) vs blast.standard +9461 (5/6) | +5600 (6/6) vs blast.standard +420 (3/6) | +14882 (5/6) vs blast.standard +11510 (5/6) |
| germ.needle | +15183 (6/6) | +8858 (6/6) | +8568 (6/6) |
| blast.needle | +9822 (6/6) | +10558 (6/6) | +11982 (6/6) |
| germ.slug | +14896 (6/6) | +6595 (5/6) | +70 (3/6) |
| plain.needle | +5567 (5/6) | +3098 (6/6) | +9898 (6/6) |
| scatter+blast | +5946 (6/6) | +9127 (6/6) | +2757 (6/6) |
| blast.standard | +5404 (5/6) | +7680 (6/6) | +3780 (6/6) |
| trigger+blast | +5404 (5/6) | +5484 (6/6) | +3048 (6/6) |
| thrift+blast | +5404 (5/6) | +5484 (6/6) | +3048 (6/6) |
| multicast+blast | +5404 (5/6) | +5484 (6/6) | +3048 (6/6) |
| expire+blast | +5404 (5/6) | +5484 (6/6) | +3048 (6/6) |
| quiet+blast | +4754 (5/6) | +5172 (6/6) | +3288 (6/6) |
| germ.standard | +12946 (6/6) | +428 (4/6) | -404 (3/6) |
| bomb.needle | +1258 (4/6) | +4708 (5/6) | +5760 (5/6) |
| delay+blast | +2428 (5/6) | +5800 (5/6) | +2644 (6/6) |
| thick+blast | +1304 (6/6) | +5199 (6/6) | +2063 (6/6) |
| split | +1454 (5/6) | +792 (6/6) | +5063 (6/6) |
| scorch | +1663 (5/6) | +4729 (6/6) | +806 (5/6) |
| pull+blast | +1242 (5/6) | +4275 (5/6) | +1134 (4/6) |
| torch | +114 (4/6) | +6104 (6/6) | -601 (3/6) |
| gas.needle | +3267 (5/6) | +312 (3/6) | +753 (5/6) |
| germ.buckshot | +10464 (5/6) | -1447 (0/6) | -4885 (0/6) |
| rate | +262 (4/6) | +320 (4/6) | +1524 (5/6) |
| pierce | +464 (5/6) | +84 (4/6) | +734 (3/6) |
| still | -120 (2/6) | +36 (3/6) | +86 (3/6) |
| tinder | +0 (0/6) | +0 (0/6) | +0 (0/6) |
| shatter | +0 (0/6) | +0 (0/6) | +0 (0/6) |
| detonate | +0 (0/6) | +0 (0/6) | +0 (0/6) |
| bait | +514 (5/6) | +180 (4/6) | -1159 (1/6) |
| provoke | +138 (5/6) | -120 (2/6) | -638 (1/6) |
| spin | +0 (3/6) | -280 (0/6) | -408 (2/6) |
| flare | -1563 (1/6) | -118 (2/6) | +728 (4/6) |
| range | +395 (5/6) | -243 (2/6) | -1240 (2/6) |
| sour | -130 (1/6) | +102 (3/6) | -1060 (0/6) |
| blast.slug | -1213 (1/6) | +811 (3/6) | -701 (1/6) |
| mist | -96 (2/6) | -289 (0/6) | -1062 (1/6) |
| seek | +32 (4/6) | -93 (3/6) | -1389 (2/6) |
| smother | -73 (1/6) | -310 (1/6) | -1190 (2/6) |
| gas.standard | -73 (2/6) | -1230 (1/6) | -1644 (1/6) |
| blast.buckshot | +52 (4/6) | -828 (2/6) | -2486 (0/6) |
| blind | -301 (0/6) | -222 (2/6) | -3568 (0/6) |
| plain.slug | -1600 (1/6) | -708 (1/6) | -2212 (0/6) |
| gas.slug | -1600 (1/6) | -742 (0/6) | -2209 (1/6) |
| bomb.standard | -2142 (1/6) | -1334 (1/6) | -1582 (1/6) |
| plain.buckshot | -352 (0/6) | -1484 (0/6) | -4516 (0/6) |
| gas.buckshot | -734 (0/6) | -1786 (0/6) | -4029 (0/6) |
| bomb.slug | -4633 (1/6) | -1196 (1/6) | -2124 (1/6) |
| heave+blast | -3186 (1/6) | +318 (3/6) | -5838 (0/6) |
| bomb.buckshot | -758 (0/6) | -2514 (0/6) | -6114 (0/6) |

unfit: 

| deck | planet 1 | planet 2 | planet 3 | planet 4 |
|---|---|---|---|---|
| wide+rate+pierce+blast.needle | +2170 (5/1) crystal -90 hp -7.0 alive 8 | unfit | +14118 (5/1) crystal -190 hp -0.4 alive 6 | +24406 (6/0) crystal -916 hp -6.9 alive 9 |
| wide+rate+blast.needle | +2218 (6/0) crystal -88 hp -6.9 alive 9 | unfit | +13390 (6/0) crystal -190 hp -1.3 alive 8 | +24942 (6/0) crystal -904 hp -6.4 alive 9 |
| wide+split+blast.needle | +2138 (6/0) crystal -88 hp -5.9 alive 9 | unfit | +13216 (6/0) crystal -189 hp -0.1 alive 7 | +24790 (6/0) crystal -894 hp -5.3 alive 9 |
| wide+pierce+blast.needle | +2051 (6/0) crystal -88 hp -5.9 alive 9 | unfit | +13566 (6/0) crystal -192 hp +0.8 alive 6 | +23963 (6/0) crystal -871 hp -4.4 alive 8 |
| seek+wide+rate+blast.needle | +2094 (6/0) crystal -89 hp -7.6 alive 9 | unfit | +13467 (6/0) crystal -192 hp -0.9 alive 8 | +23568 (6/0) crystal -908 hp -6.7 alive 9 |
| wide+trigger+blast.needle+torch | +2180 (6/0) crystal -89 hp -6.0 alive 8 | unfit | +13731 (4/2) crystal -261 hp +0.6 alive 4 | +23164 (6/0) crystal -897 hp -4.2 alive 9 |
| seek+wide+blast.needle | +1849 (5/1) crystal -87 hp -5.9 alive 9 | unfit | +13225 (6/0) crystal -255 hp -0.3 alive 6 | +23986 (6/0) crystal -830 hp -3.3 alive 6 |
| wide+blast.needle | +2144 (6/0) crystal -86 hp -5.0 alive 9 | unfit | +12654 (5/1) crystal -177 hp -0.5 alive 8 | +23940 (6/0) crystal -865 hp -4.2 alive 8 |
| seek+wide+split+blast.needle | +1798 (6/0) crystal -88 hp -6.6 alive 9 | unfit | +12280 (5/1) crystal -190 hp +0.5 alive 7 | +24421 (6/0) crystal -888 hp -5.1 alive 8 |
| blast.needle | +1976 (5/1) crystal -86 hp -4.6 alive 9 | unfit | +12738 (5/1) crystal -180 hp +0.7 alive 7 | +23711 (6/0) crystal -838 hp -2.7 alive 8 |
| seek+blast.needle | +2080 (5/1) crystal -86 hp -4.7 alive 8 | unfit | +12412 (5/1) crystal -177 hp +0.1 alive 7 | +23810 (6/0) crystal -788 hp -2.2 alive 6 |
| wide+scorch+blast.needle | +2195 (6/0) crystal -90 hp -6.9 alive 9 | unfit | +11148 (4/2) crystal -262 hp +0.8 alive 2 | +23304 (6/0) crystal -882 hp -4.3 alive 8 |
| seek+wide+smother+blast.needle | +1951 (5/1) crystal -87 hp -6.0 alive 9 | unfit | +11314 (5/1) crystal -203 hp -0.1 alive 6 | +23310 (6/0) crystal -853 hp -3.6 alive 8 |
| germ.needle | +2946 (6/0) crystal -88 hp -6.4 alive 9 | unfit | +11057 (6/0) crystal -183 hp -1.1 alive 8 | +18008 (6/0) crystal -662 hp -1.1 alive 3 |
| seek+scatter+germ.needle | +2951 (6/0) crystal -88 hp -6.2 alive 9 | unfit | +11208 (6/0) crystal -188 hp -0.9 alive 8 | +16799 (6/0) crystal -658 hp -1.2 alive 3 |
| wide+range+germ.needle | +9581 (6/0) crystal -87 hp -6.6 alive 9 | unfit | +14958 (6/0) crystal -28 hp -0.2 alive 6 | +6414 (6/0) crystal -26 hp +0.0 alive 0 |
| plain.needle | +1957 (5/1) crystal -84 hp -3.9 alive 9 | unfit | +9436 (5/1) crystal -183 hp -0.6 alive 8 | +18664 (6/0) crystal -764 hp -1.2 alive 4 |
| trigger+blast.standard+gas.standard | +1846 (5/1) crystal -88 hp -6.4 alive 9 | unfit | +9626 (5/1) crystal -218 hp -0.7 alive 8 | +17918 (5/1) crystal -770 hp -3.0 alive 6 |
| wide+blast | +1362 (6/0) crystal -48 hp -2.3 alive 9 | unfit | +7153 (6/0) crystal -90 hp +0.3 alive 4 | -24 (2/4) crystal +0 hp +0.0 alive 0 |
| blast.standard | +656 (5/1) crystal -48 hp -1.7 alive 8 | unfit | +6472 (4/2) crystal -78 hp +0.5 alive 2 | +214 (4/2) crystal +0 hp +0.0 alive 0 |

planet 1 empty deck: removed -388, crystal 95, hp lost 8.5, alive 2
