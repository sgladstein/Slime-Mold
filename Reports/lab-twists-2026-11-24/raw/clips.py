import os, subprocess, sys
from PIL import Image, ImageDraw, ImageFont
D='/tmp/claude-0/lab/twistfilm'; C=D+'/clips'
try: font=ImageFont.truetype('/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf', 22)
except Exception: font=ImageFont.load_default()
label={'base':'as it is','cold':'cold snap from 0:10','night':'night from 0:10','wind':'wind (east) from 0:10'}
def lab(im, text, t):
    d=ImageDraw.Draw(im); s=f'{text}   0:{t:02d}' if t<60 else f'{text}   1:{t-60:02d}'
    d.rectangle([8,8,8+14*len(s),40], fill=(0,0,0)); d.text((14,11), s, fill=(255,255,255), font=font); return im
def frame(a,t,size): 
    im=Image.open(f'{D}/{a}/planet-1-1-idle-{t:03d}s-near.png').convert('RGB').crop((0,0,1280,700)).resize(size)
    return lab(im,label[a],t)
def make(name, arms, cols, size):
    tmp=f'{C}/{name}'; os.makedirs(tmp, exist_ok=True)
    rows=(len(arms)+cols-1)//cols
    for i,t in enumerate(range(5,71)):
        c=Image.new('RGB',(cols*size[0], rows*size[1]))
        for j,a in enumerate(arms): c.paste(frame(a,t,size),((j%cols)*size[0],(j//cols)*size[1]))
        c.save(f'{tmp}/f{i:03d}.png')
    out=f'{C}/{name}.mp4'
    subprocess.run(['ffmpeg','-y','-loglevel','error','-framerate','6','-i',f'{tmp}/f%03d.png','-c:v','libx264','-pix_fmt','yuv420p','-crf','30','-preset','slow',out],check=True)
    print(name, os.path.getsize(out)//1024,'KB')
make('twists-grid',['base','cold','night','wind'],2,(640,350))
for a in ['cold','night','wind']: make(f'{a}-vs-base',['base',a],2,(640,350))
