import sys, os, subprocess, tempfile
from PIL import Image, ImageDraw, ImageFont
font = ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf', 15)
ts=[0.2,0.5,1,1.5,2,3,4,5,6,8,10,12]
def name(t):
    ms=int(round(t*1000)); return f'{ms//1000:03d}s' if ms%1000==0 else f'{ms:05d}ms'
for m,f in [('planet_2_1_1','planet-2-1-1'),('planet_2_2_1','planet-2-2-1'),('fork','fork')]:
    tmp=tempfile.mkdtemp(dir='.')
    for i,t in enumerate(ts):
        ims=[Image.open(f'f-{b}-{m}/{f}-idle-{name(t)}-all.png').convert('RGB').crop((40,20,1240,700)).resize((480,272),Image.LANCZOS) for b in ('base','final')]
        cv=Image.new('RGB',(964,272),(17,26,31)); cv.paste(ims[0],(0,0)); cv.paste(ims[1],(484,0))
        d=ImageDraw.Draw(cv)
        for x,tag in ((0,'before'),(484,'now')):
            d.rectangle([x,0,x+150,22],fill=(17,26,31)); d.text((x+6,3),f'{tag} · {t:g} s',fill=(227,234,236),font=font)
        cv.save(f'{tmp}/f{i:03d}.png')
    out=f'start-{m}.gif'
    subprocess.run(['ffmpeg','-y','-loglevel','error','-framerate','3','-i',f'{tmp}/f%03d.png','-vf','split[a][b];[a]palettegen=max_colors=160[p];[b][p]paletteuse=dither=bayer:bayer_scale=3','-loop','0',out],check=True)
    subprocess.run(['rm','-r',tmp]); print(out, os.path.getsize(out))
