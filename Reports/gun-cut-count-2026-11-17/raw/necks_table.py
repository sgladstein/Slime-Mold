# necks_table.py DIR: the table of necks.sh's cut count. Per planet, hand
# and time: the walkers, those already without food, and for necks 2 and 4
# dish cells wide the walkers a cut would leave without food (and their
# share), the largest such part and the parts of 500 walkers or more.
# Then, over all counts: how many had any part of 500+, and the median and
# largest shares.
import os
import re
import sys

D = sys.argv[1]
LINE = re.compile(r"necks t (\d+) s: (\d+) walkers, (\d+) without food, (\d+) crystals with charge; "
                  r"necks 2: (\d+) cuttable \(([\d.]+)%\), largest (\d+), (\d+) parts of 500\+; "
                  r"necks 4: (\d+) cuttable \(([\d.]+)%\), largest (\d+), (\d+) parts of 500\+")
END = re.compile(r"t +(\d+) s: .*ended: ([a-z ]+)")
# pictures.sh's runs also print the count with pinholes left open (the
# first census's rule), to compare.
RAW = re.compile(r"necks t (\d+) s, pinholes open: necks 2: (\d+) cuttable, largest (\d+), (\d+) parts of 500\+; "
                 r"necks 4: (\d+) cuttable, largest (\d+), (\d+) parts of 500\+")
rows = []
print(f"{'run':16} {'t':>4} {'walkers':>8} {'no food':>8} {'cryst':>5}  {'neck2 cut':>9} {'%':>5} {'largest':>7} {'500+':>4}  {'neck4 cut':>9} {'%':>5} {'largest':>7} {'500+':>4}")
for name in sorted((n for n in os.listdir(D) if re.match(r"p\d+-", n)), key=lambda n: (int(re.match(r"p(\d+)", n).group(1)), n)):
    p = os.path.join(D, name, "log.txt")
    if not os.path.exists(p):
        continue
    text = open(p).read()
    for m in LINE.finditer(text):
        t, tot, unfed, cr, c2, p2, l2, b2, c4, p4, l4, b4 = m.groups()
        rows.append((name, int(t), int(tot), float(p2), int(b2), float(p4), int(b4), int(l2), int(l4)))
        print(f"{name:16} {t:>4} {tot:>8} {unfed:>8} {cr:>5}  {c2:>9} {p2:>5} {l2:>7} {b2:>4}  {c4:>9} {p4:>5} {l4:>7} {b4:>4}")
        for r in RAW.finditer(text):
            if r.group(1) == t:
                rc2, rl2, rb2, rc4, rl4, rb4 = r.groups()[1:]
                print(f"{'  pinholes open':16} {'':>4} {'':>8} {'':>8} {'':>5}  {rc2:>9} {100 * int(rc2) / int(tot):>5.1f} {rl2:>7} {rb2:>4}  {rc4:>9} {100 * int(rc4) / int(tot):>5.1f} {rl4:>7} {rb4:>4}")
    e = END.search(text)
    if e:
        print(f"{name:16} ended at {e.group(1)} s: {e.group(2)}")
print()
n = len(rows)
if n:
    def med(v):
        v = sorted(v)
        return v[len(v) // 2]
    print(f"{n} counts")
    print(f"necks 2: a part of 500+ in {sum(r[4] > 0 for r in rows)} of {n}; cuttable share median {med([r[3] for r in rows]):.1f}%, largest {max(r[3] for r in rows):.1f}%; largest part {max(r[7] for r in rows)} walkers")
    print(f"necks 4: a part of 500+ in {sum(r[6] > 0 for r in rows)} of {n}; cuttable share median {med([r[5] for r in rows]):.1f}%, largest {max(r[5] for r in rows):.1f}%; largest part {max(r[8] for r in rows)} walkers")
    for k, i in [("2", 7), ("4", 8)]:
        big = [100 * r[i] / r[2] for r in rows]
        print(f"necks {k}: largest part 2%+ of the slime in {sum(b >= 2 for b in big)} of {n}, 5%+ in {sum(b >= 5 for b in big)}; largest {max(big):.1f}%")
    for h in ["idle", "squadwork"]:
        hr = [r for r in rows if r[0].endswith(h)]
        if hr:
            print(f"  {h}: necks 4 part of 500+ in {sum(r[6] > 0 for r in hr)} of {len(hr)}; share median {med([r[5] for r in hr]):.1f}%")
