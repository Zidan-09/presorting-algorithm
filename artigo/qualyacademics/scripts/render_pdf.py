import os, sys
import pypdfium2 as pdfium
BASE = r'C:\Users\acemilton\Documents\presorting-algorithm'
QC = os.path.join(BASE, 'artigo', 'qualyacademics')
pdf = pdfium.PdfDocument(os.path.join(QC, 'artigo_qualyacademics.pdf'))
out = os.path.join(QC, 'scripts', '_pages')
os.makedirs(out, exist_ok=True)
for i, page in enumerate(pdf):
    img = page.render(scale=1.6).to_pil()
    img.save(os.path.join(out, 'p%02d.png' % (i + 1)))
    txt = page.get_textpage().get_text_range()
    first = ' '.join(txt.split())[:110]
    print('%02d %s' % (i + 1, first))
print('pages', len(pdf))
