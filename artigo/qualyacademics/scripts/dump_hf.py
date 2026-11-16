import os
from docx import Document
BASE = r'C:\Users\acemilton\Documents\presorting-algorithm'
d = Document(os.path.join(BASE, 'artigo', 'qualyacademics', 'modelo.docx'))
for name, part in [('header2', d.sections[0].header), ('header3', d.sections[0].first_page_header),
                   ('footer3', d.sections[0].first_page_footer), ('footer2', d.sections[0].footer)]:
    print('=====', name)
    for i, p in enumerate(part.paragraphs):
        for j, r in enumerate(p.runs):
            if r.text:
                print(i, j, repr(r.text))
