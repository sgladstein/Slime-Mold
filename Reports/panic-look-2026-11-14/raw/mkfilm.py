import sys, os, subprocess, tempfile
from PIL import Image, ImageDraw, ImageFont
from frame import fname
# mkfilm.py SRC OUT CROP(x0,y0,x1,y1) T0 T1 CUT POSTER
src,out=sys.argv[1],sys.argv[2]
box=tuple(int(v) for v in sys.argv[3].split(','))
t0,t1,cut,poster=map(float,sys.argv[4:8])
try: font=ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf',18)
except Exception: font=ImageFont.load_default()
tmp=tempfile.mkdtemp(dir='.')
n=0; t=t0
while t<=t1+1e-6:
    im=Image.open(fname(src,t)).convert('RGB').crop(box)
    d=ImageDraw.Draw(im)
    dt=t-cut
    lab=f'{int(t)//60}:{int(t)%60:02d}' + ('.5' if abs(t-int(t)-0.5)<1e-6 else '') + (f'   cut {dt:+.1f} s' if dt>=0 else '')
    d.rectangle([0,0,240,28],fill=(17,26,31))
    d.text((8,4),lab,fill=(227,234,236),font=font)
    im.save(f'{tmp}/f{n:04d}.png')
    if abs(t-poster)<1e-6: im.save(out+'.jpg',quality=88)
    n+=1; t+=0.5
subprocess.run(['ffmpeg','-y','-loglevel','error','-framerate','6','-i',f'{tmp}/f%04d.png','-c:v','libx264','-pix_fmt','yuv420p','-crf','22','-movflags','+faststart',out+'.mp4'],check=True)
subprocess.run(['rm','-r',tmp])
print(out, n, 'frames', os.path.getsize(out+'.mp4'))
