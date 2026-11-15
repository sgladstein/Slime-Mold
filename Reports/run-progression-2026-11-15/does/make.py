import glob, os, subprocess, sys
from PIL import Image, ImageDraw, ImageFont
F='/tmp/claude-0/-home-user/436aa0b5-2e2a-5fcc-a830-4e57c114a955/scratchpad/film'
OUT='/home/user/Slime-Mold/Reports/run-progression-2026-11-15/does'
view={1:'all',2:'near',3:'all',4:'near',5:'near',6:'near'}
try: font=ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf',22)
except Exception: font=ImageFont.load_default()
for d in range(1,7):
    v=view[d]; tmp=f'{F}/frames{d}'; os.makedirs(tmp,exist_ok=True)
    bs=sorted(glob.glob(f'{F}/d{d}-base/*-{v}.png')); hs=sorted(glob.glob(f'{F}/d{d}-hot/*-{v}.png'))
    for i,(b,h) in enumerate(zip(bs,hs)):
        t=int(b.split('-')[-2][:-1])
        W=640; c=Image.new('RGB',(2*W+8,400),(20,20,20))
        for j,(p,lab) in enumerate([(b,'Before'),(h,'With the planet\'s thing')]):
            im=Image.open(p).convert('RGB').resize((W,400)); c.paste(im,(j*(W+8),0))
            dr=ImageDraw.Draw(c); dr.rectangle((j*(W+8),0,j*(W+8)+330,34),fill=(0,0,0)); dr.text((j*(W+8)+8,5),f'{lab}  {t//60}:{t%60:02d}',font=font,fill=(255,255,255))
        c.save(f'{tmp}/{i:03d}.png')
        if i==len(bs)-1: c.convert('RGB').save(f'{OUT}/d{d}-last.jpg',quality=82)
    subprocess.run(['ffmpeg','-y','-loglevel','error','-framerate','4','-i',f'{tmp}/%03d.png','-c:v','libx264','-pix_fmt','yuv420p','-crf','26','-vf','scale=trunc(iw/2)*2:trunc(ih/2)*2',f'{OUT}/d{d}.mp4'],check=True)
    print(d,len(bs))
