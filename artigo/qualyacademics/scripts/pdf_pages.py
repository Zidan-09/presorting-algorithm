import pypdfium2 as pdfium, os, sys

pdf_path, out_dir = sys.argv[1], sys.argv[2]
os.makedirs(out_dir, exist_ok=True)
pdf = pdfium.PdfDocument(pdf_path)
print('pages', len(pdf))
for i, page in enumerate(pdf):
    txt = page.get_textpage().get_text_range()
    print('=' * 20, 'PAGE', i + 1, '=' * 20)
    print(txt[:1500])
    if '--render' in sys.argv:
        bmp = page.render(scale=100 / 72)
        img = bmp.to_pil()
        img.save(os.path.join(out_dir, f'page_{i+1:02d}.png'))
