import os
def fname(d,t,kind='near'):
    ms=int(round(t*1000))
    if ms%1000==0: return f'{d}/fork-farm-{ms//1000:03d}s-{kind}.png'
    return f'{d}/fork-farm-{ms}ms-{kind}.png'
