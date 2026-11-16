import pypdfium2 as pdfium, numpy as np, sys
from PIL import Image

pdf = pdfium.PdfDocument(sys.argv[1])
pages = [int(p) for p in sys.argv[2:]]
for pno in pages:
    page = pdf[pno - 1]
    bmp = page.render(scale=300 / 72)
    img = bmp.to_pil().convert('L')
    a = np.array(img)
    ink = (a < 245).sum(axis=1)
    rows = np.where(ink > 2)[0]
    blocks = []
    if len(rows):
        start = prev = rows[0]
        for r in rows[1:]:
            if r - prev > 12:
                blocks.append((start, prev))
                start = r
            prev = r
        blocks.append((start, prev))
    print('PAGE', pno, 'size', img.size)
    for b in blocks:
        # x extent in block
        seg = (a[b[0]:b[1] + 1] < 245)
        cols = np.where(seg.any(axis=0))[0]
        print('  rows %5d-%5d h=%4d cols %4d-%4d  y_cm %.2f-%.2f' % (
            b[0], b[1], b[1] - b[0] + 1, cols[0] if len(cols) else -1, cols[-1] if len(cols) else -1,
            b[0] / 300 * 2.54, b[1] / 300 * 2.54))
