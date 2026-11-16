from PIL import Image
im = Image.open(r'C:\Users\acemilton\Documents\presorting-algorithm\artigo\qualyacademics\scripts\_pages\p11.png')
box = (40, 660, 930, 730)
c = im.crop(box).resize(((box[2]-box[0])*2, (box[3]-box[1])*2), Image.LANCZOS)
c.save(r'C:\Users\acemilton\Documents\presorting-algorithm\artigo\qualyacademics\scripts\_pages\crop_fp2.png')
