import re,sys,glob
from PIL import Image, ImageDraw, ImageFont
def parse(path):
    frames=[];cur=None
    for line in open(path):
        m=re.match(r'\s+t (\d+) s:',line)
        if m: cur={'built':[]}; frames.append(cur); continue
        m=re.match(r'\s+turret at \(.*built ([\d.]+)',line)
        if m and cur is not None: cur['built'].append(float(m.group(1)))
    return frames
A=parse('clip-main/t-lines.txt'); B=parse('clip-new/t-lines.txt')
def key(p):
    m=re.search(r'-(\d+)(ms|s)-near',p); v=int(m.group(1)); return v/1000 if m.group(2)=='ms' else v
fa=sorted(glob.glob('clip-main/*-near.png'),key=key); fb=sorted(glob.glob('clip-new/*-near.png'),key=key)
print(len(A),len(B),len(fa),len(fb))
times=[2+0.25*i for i in range(41)]
font=ImageFont.load_default(size=18) if hasattr(ImageFont,'load_default') else None
out=[]
box=(150,20,810,300)
for i,(a,b) in enumerate(zip(fa,fb)):
    ia=Image.open(a).convert('RGB').crop(box); ib=Image.open(b).convert('RGB').crop(box)
    w,h=ia.size
    c=Image.new('RGB',(w*2+10,h+60),(20,20,24))
    c.paste(ia,(0,60)); c.paste(ib,(w+10,60))
    d=ImageDraw.Draw(c)
    def lab(fr): 
        bl=fr['built'] if fr else []
        names=['middle','left','right']
        return '  '.join(f"{n} {v*100:.0f}%" for n,v in zip(names,bl)) if bl else 'no ghosts yet'
    d.text((8,6),f"BEFORE (main)   t {times[i]:.2f} s",fill=(255,255,255),font=font)
    d.text((8,32),lab(A[i] if i<len(A) else None),fill=(230,210,150),font=font)
    d.text((w+18,6),f"AFTER   t {times[i]:.2f} s",fill=(255,255,255),font=font)
    d.text((w+18,32),lab(B[i] if i<len(B) else None),fill=(230,210,150),font=font)
    out.append(c.resize((c.width*3//4,c.height*3//4)))
out[0].save(sys.argv[1],save_all=True,append_images=out[1:],duration=250,loop=0)
