set -x
for b in main new; do
 for z in 6 12; do ./lw-$b look=f-$b-$z hand=farm seed=1 map=fork centre=420,232,$z at=150.0,150.1,150.2,150.3,150.4,150.5,150.6,150.7,150.8,150.9,151.0,151.1,151.2,151.3,151.4,151.5,151.6,151.7,151.8,151.9,152.0,152.1,152.2,152.3,152.4,152.5,152.6,152.7,152.8,152.9,153.0 > f-$b-$z.log 2>&1; done
 ./lw-$b look=f-$b-cut hand=farm seed=1 map=fork cut=150,365,240,32 cutsquad=365,264,150,4 centre=365,238,6 at=156.0,156.1,156.2,156.3,156.4,156.5,156.6,156.7,156.8,156.9,157.0,157.1,157.2,157.3,157.4,157.5,157.6,157.7,157.8,157.9,158.0 > f-$b-cut.log 2>&1
done
for r in 1 2; do for b in main new; do ./lw-$b bench=20 hand=farm seed=1 > bench-$b-$r.log 2>&1; done; done
mkdir -p gf
for z in 6 12; do python3 mkgif2.py gf/body$z.gif f-main-$z f-new-$z 160,60,1120,660 150 31 "main" "now, zoom $z"; done
python3 mkgif2.py gf/tail6.gif f-main-6 f-new-6 0,300,480,640 150 31 "main" "now, zoom 6"
python3 mkgif2.py gf/grey6.gif f-main-6 f-new-6 160,60,1120,660 150 31 "main" "now" grey
python3 mkgif2.py gf/cut.gif f-main-cut f-new-cut 420,190,860,500 156 21 "main" "now"
python3 mkgif2.py gf/cut-grey.gif f-main-cut f-new-cut 420,190,860,500 156 21 "main" "now" grey
touch fin.done
