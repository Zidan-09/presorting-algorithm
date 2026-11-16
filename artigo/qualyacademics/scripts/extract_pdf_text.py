import os
import pypdfium2 as pdfium

BASE = r'C:\Users\acemilton\Documents\presorting-algorithm'
PDF = os.path.join(BASE, 'artigo', 'qualyacademics', 'artigo_qualyacademics.pdf')
OUT = os.path.join(BASE, 'artigo', 'qualyacademics', 'scripts', '_pdf_text.txt')

pdf = pdfium.PdfDocument(PDF)
lines = []
for i, page in enumerate(pdf):
    tp = page.get_textpage()
    txt = tp.get_text_range()
    lines.append('===== PAGE %d =====' % (i + 1))
    lines.append(txt)
open(OUT, 'w', encoding='utf-8').write('\n'.join(lines))
print('pages', len(pdf))
print('chars', sum(len(t) for t in lines))
# report suspicious glyphs
bad = {}
for i, page in enumerate(pdf):
    t = page.get_textpage().get_text_range()
    for ch in t:
        o = ord(ch)
        if o == 0xFFFE or (0xE000 <= o <= 0xF8FF) or o == 0x25C6:
            bad.setdefault(i + 1, set()).add(ch)
print('bad glyphs per page:', {k: sorted(v) for k, v in bad.items()})
