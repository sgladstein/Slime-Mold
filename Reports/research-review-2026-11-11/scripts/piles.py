import math
M=0xFFFFFFFF
def lb(x):
    x&=M; x^=x>>16; x=(x*0x7FEB352D)&M; x^=x>>15; x=(x*0x846CA68B)&M; return x^(x>>16)
def keyed(seed,t,use,w):
    s=((seed&M)^((seed>>32)&M)^((use*0x9E3779B9)&M))&M
    step=lb(lb(s)^(t&M)); return lb((w&M)^step)
def unit(x): return (x>>8)/(1<<24)
W,H=256,160; DRAW_PILES=105
def piles(seed,n,r=6.0):
    cx,cy,rx,ry=W/2,H/2,W/2-3,H/2-3
    inside=lambda x,y: ((x-cx)/(rx-r-4))**2+((y-cy)/(ry-r-4))**2<=1
    P=[(W*0.22,H*0.5)]; k=0
    while len(P)<n and k<100000:
        x=unit(keyed(seed,0,DRAW_PILES,2*k))*W; y=unit(keyed(seed,0,DRAW_PILES,2*k+1))*H; k+=1
        if inside(x,y) and all(math.hypot(px-x,py-y)>=28 for px,py in P): P.append((x,y))
    return P
for name,blob in [('net blob_r 20',20),('drop blob_r 50',50)]:
    print(name)
    for s in range(1,9):
        P=piles(s,12)
        d=[math.hypot(x-128,y-80) for x,y in P]
        over=[i for i,v in enumerate(d) if v<blob+6]
        near=[i for i,v in enumerate(d) if blob+6<=v<blob+6+9]  # within scent reach (36 fine = 9 dish) of disc edge
        # distance of pile centre to agar rim (approx, normalised ellipse)
        rim=sum(1 for x,y in P if ((x-128)/125)**2+((y-80)/77)**2>0.6)
        print(f'  seed {s}: {len(P)} piles; overlapping start disc {len(over)} {over}; within smell of disc edge {len(near)}; nearest pile centre {min(d):.1f} dish cells; piles in outer band (ellip r>0.77) {rim}')
