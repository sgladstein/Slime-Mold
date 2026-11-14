"""Width of each water across its own course: twice the distance from the
water's middle line to the nearest dry cell, on a map file's G byte (wet
bit). Prints the 25th percentile and the median.

  python3 medial.py MAPS planet-4-1 planet-5-2 ...
"""
import sys
from PIL import Image
def widths(path):
    g=Image.open(path).getchannel(1); w,h=g.size; px=g.load()
    INF=10**9
    d=[[0 if not px[x,y]&1 else INF for x in range(w)] for y in range(h)]
    for y in range(h):
        for x in range(w):
            if d[y][x]:
                v=d[y][x]
                if x>0: v=min(v,d[y][x-1]+1)
                if y>0: v=min(v,d[y-1][x]+1)
                d[y][x]=v
    for y in range(h-1,-1,-1):
        for x in range(w-1,-1,-1):
            if d[y][x]:
                v=d[y][x]
                if x<w-1: v=min(v,d[y][x+1]+1)
                if y<h-1: v=min(v,d[y+1][x]+1)
                d[y][x]=v
    r=[]
    for y in range(1,h-1,2):
        for x in range(1,w-1,2):
            v=d[y][x]
            if v and v>=max(d[y][x-1],d[y][x+1]) and v>=max(d[y-1][x],d[y+1][x]):
                r.append(2*v-1)
    r.sort(); return r
for name in sys.argv[2:]:
    r=widths(f'{sys.argv[1]}/{name}.png')
    print(name,'p25',r[len(r)//4],'median',r[len(r)//2])
