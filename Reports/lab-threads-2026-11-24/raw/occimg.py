import sys, numpy as np
from PIL import Image
sys.path.insert(0,'/tmp/claude-0/lab'); from measure import load
d,sec,out=sys.argv[1],int(sys.argv[2]),sys.argv[3]
ag,o=load(d,sec)
img=np.zeros(o.shape+(3,),np.uint8); img[ag>0]=(40,40,40); img[o>0]=(255,160,0); img[o>1]=(255,240,120)
Image.fromarray(img).resize((o.shape[1]*2,o.shape[0]*2),Image.NEAREST).save(out)
