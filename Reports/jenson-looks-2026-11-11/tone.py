import sys, numpy as np
from PIL import Image
# tone.py out.png gamma file1.f32 [file2 ...] : trail, log curve over p99.9, side by side, down 2
g=float(sys.argv[2]); ims=[]
for f in sys.argv[3:]:
    a=np.fromfile(f,dtype='<f4'); v=a[:1024*640].reshape(640,1024)
    ref=np.percentile(v[v>0],99.9)
    x=np.clip(np.log1p(v/ref*g)/np.log1p(g),0,1)
    ims.append(Image.fromarray((x*255).astype(np.uint8)).resize((512,320)))
out=Image.new('L',(512*len(ims),320))
for i,im in enumerate(ims): out.paste(im,(512*i,0))
out.save(sys.argv[1])
