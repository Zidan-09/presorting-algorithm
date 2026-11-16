import pypdfium2 as pdfium, os, sys
from PIL import Image

pdf = pdfium.PdfDocument(sys.argv[1])
outdir = sys.argv[2]
os.makedirs(outdir, exist_ok=True)
# page, top, bottom, left, right at 300 dpi (inclusive), with margin
FIGS = {
    'fig1_inversoes': (13, 595, 1287, 415, 2081),
    'fig2_custo': (16, 613, 1359, 440, 2037),
    'fig3_tempos': (19, 527, 2193, 436, 2060),
    'fig4_ganho_n': (20, 900, 2384, 395, 2082),
}
SCALE = 600 / 72
M = 15  # px at 600 dpi
for name, (pno, top, bot, left, right) in FIGS.items():
    page = pdf[pno - 1]
    bmp = page.render(scale=SCALE)
    img = bmp.to_pil()
    f = 2  # 300 -> 600 dpi
    box = (max(0, left * f - M), max(0, top * f - M),
           min(img.width, right * f + M), min(img.height, bot * f + M))
    crop = img.crop(box)
    path = os.path.join(outdir, name + '.png')
    crop.save(path, dpi=(600, 600))
    print(name, crop.size, path)
