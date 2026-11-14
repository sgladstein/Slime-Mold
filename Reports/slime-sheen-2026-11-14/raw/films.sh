set -x
for b in main2 p3; do
 for z in 6 12; do ./lw-$b look=g-$b-$z hand=farm seed=1 map=fork centre=420,232,$z at=150.0,150.1,150.2,150.3,150.4,150.5,150.6,150.7,150.8,150.9,151.0,151.1,151.2,151.3,151.4,151.5,151.6,151.7,151.8,151.9,152.0,152.1,152.2,152.3,152.4,152.5,152.6,152.7,152.8,152.9,153.0 > g-$b-$z.log 2>&1; done
done
for r in 1 2; do for b in main2 p3; do ./lw-$b bench=20 hand=farm seed=1 > bench3-$b-$r.log 2>&1; done; done
mkdir -p gp
for z in 6 12; do python3 mkgif2.py gp/body$z.gif g-main2-$z g-p3-$z 160,60,1120,660 150 31 "main" "now, zoom $z"; done
python3 mkgif2.py gp/front6.gif g-main2-6 g-p3-6 0,200,640,640 150 31 "main" "now, zoom 6"
python3 mkgif2.py gp/grey6.gif g-main2-6 g-p3-6 160,60,1120,660 150 31 "main" "now" grey
touch p3.done
