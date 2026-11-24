import os, subprocess, glob
from PIL import Image, ImageDraw, ImageFont
D='/tmp/claude-0/lab/rescuefilm'; C=D+'/clips'
try: font=ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf', 22)
except Exception: font=ImageFont.load_default()
def clock(t): return f'{t//60}:{t%60:02d}'
def lab(im, text, t):
    d=ImageDraw.Draw(im); s=f'{text}   {clock(t)}'
    d.rectangle([8,8,8+14*len(s),40], fill=(0,0,0)); d.text((14,11), s, fill=(255,255,255), font=font); return im
def make(name, sub, text, a, b, fps=6, size=(960,526)):
    tmp=f'{C}/{name}'; os.makedirs(tmp, exist_ok=True)
    i=0
    for t in range(a,b+1):
        f=glob.glob(f'{D}/{sub}/planet-1-1-*-{t:03d}s-near.png')
        if not f: continue
        im=Image.open(f[0]).convert('RGB').crop((0,0,1280,700)).resize(size)
        lab(im,text,t).save(f'{tmp}/f{i:03d}.png'); i+=1
    out=f'{C}/{name}.mp4'
    subprocess.run(['ffmpeg','-y','-loglevel','error','-framerate',str(fps),'-i',f'{tmp}/f%03d.png','-c:v','libx264','-pix_fmt','yuv420p','-crf','30','-preset','slow',out],check=True)
    print(name, i, 'frames', os.path.getsize(out)//1024,'KB')
import sys
for job in sys.argv[1:]:
    if job=='win': make('rescue-win','win','sent at 1:30, reached 1:40, home 2:30',86,152)
    if job=='greedy': make('greedy-loss','greedy','never sent: the ring gives way',240,282)
    if job=='late': make('late-loss','late2','sent at 3:00: left behind on the way home',184,210)
