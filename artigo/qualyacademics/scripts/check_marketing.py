import pypdfium2 as pdfium

d = pdfium.PdfDocument(r'artigo/qualyacademics/artigo_qualyacademics.pdf')
found = False
for i, p in enumerate(d):
    t = p.get_textpage().get_text_range()
    if 'DESEJA VER' in t:
        print(f'Page {i+1}: FOUND')
        found = True
if not found:
    print('Not found in PDF')
