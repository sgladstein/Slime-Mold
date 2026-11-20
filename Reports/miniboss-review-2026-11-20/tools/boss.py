import re,sys,glob,math
pat=re.compile(r"boss t (\d+) s: lump (\d) (\w+) (\w+)(?:\(\(([\d.]+), ([\d.]+)\)(?:, \d+)?\)|\(Some\((\d+)\)\)|\(None\))?, (\d+) members \(first (\d+), (\d+) hurt\), at (\d+),(\d+), spread ([\d.]+), stolen ([\d.]+); nearest soldier ([\d.inf]+), (\d+) alive")
form=re.compile(r"boss t ([\d.]+) s: (\w+) (\w+) formed on lump (\d) at (\d+),(\d+).*grade ([\d.]+)")
end=re.compile(r"boss t (\d+) s: lump (\d) (\w+) ended (\w+) \((\d+) members of (\d+)")
for f in sys.argv[1:]:
  L=open(f).read().splitlines()
  live={}
  print("==",f.split('/')[-2])
  for l in L:
    m=form.search(l)
    if m:
      t,cause,kind,lump,x,y,g=m.groups(); live[lump]=dict(t=float(t),cause=cause,kind=kind,g=float(g),rows=[])
      continue
    m=pat.search(l)
    if m and m.group(2) in live:
      t=int(m.group(1)); ph=m.group(4); n=int(m.group(8)); x,y=int(m.group(11)),int(m.group(12)); near=float(m.group(15)); alive=int(m.group(16))
      live[m.group(2)]['rows'].append((t,ph,n,x,y,near,alive,float(m.group(14)),int(m.group(10))))
      continue
    m=end.search(l)
    if m and m.group(2) in live:
      b=live.pop(m.group(2)); r=b['rows']
      if not r: continue
      ph=[];
      for row in r:
        if not ph or ph[-1][0]!=row[1]: ph.append([row[1],row[0]])
      # stuck seconds: moved <3 units over previous 10 s while not Sit/Gather
      stuck=sum(1 for i in range(10,len(r)) if r[i][1] in('Go','Hunt') and math.hypot(r[i][3]-r[i-10][3],r[i][4]-r[i-10][4])<3)
      path=sum(math.hypot(r[i][3]-r[i-1][3],r[i][4]-r[i-1][4]) for i in range(1,len(r)))
      print(f"  {b['kind']:8s} {b['cause']:5s} g{b['g']:.2f} t{b['t']:.0f}-{m.group(1)} {m.group(4):7s} members {r[1][2] if len(r)>1 else 0}->{m.group(5)} nearest-min {min(x[5] for x in r):.0f} alive {r[0][6]}->{r[-1][6]} stolen {r[-1][7]:.0f} path {path:.0f} stuck-s {stuck} phases {' '.join(p[0]+'@'+str(p[1]) for p in ph)}")
  for k,b in live.items():
    r=b['rows']
    if r: print(f"  {b['kind']:8s} {b['cause']:5s} g{b['g']:.2f} t{b['t']:.0f}- (alive at end) members {r[-1][2]} nearest-min {min(x[5] for x in r):.0f} alive {r[0][6]}->{r[-1][6]}")
