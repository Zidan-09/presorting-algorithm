import re, subprocess, zipfile, os, sys
from lxml import etree

M = 'http://schemas.openxmlformats.org/officeDocument/2006/math'
W = 'http://schemas.openxmlformats.org/wordprocessingml/2006/main'

src = open(sys.argv[1], encoding='utf-8').read()
body = src[src.index(r'\begin{document}'):]
maths = re.findall(r'(?<!\\)\$(.+?)(?<!\\)\$|\\\((.+?)\\\)', body, flags=re.S)
maths = [a or b for a, b in maths]
uniq = []
for m in maths:
    m = m.strip()
    if m not in uniq:
        uniq.append(m)
# special markers handled elsewhere
uniq = [m for m in uniq if m not in ('^{*}', '^{\\dagger}')]
print('unique', len(uniq))

md = '\n\n'.join('$' + m + '$' for m in uniq)
open('_math_all.md', 'w', encoding='utf-8').write(md + '\n')
subprocess.run([r'C:\Users\acemilton\AppData\Local\Pandoc\pandoc.exe', '_math_all.md', '-o', '_math_all.docx'], check=True)
z = zipfile.ZipFile('_math_all.docx')
root = etree.fromstring(z.read('word/document.xml'))
paras = root.findall('.//{%s}p' % W)
print('paras', len(paras))
oms = []
for p in paras:
    om = p.find('.//{%s}oMath' % M)
    if om is None:
        print('NO MATH:', ''.join(p.itertext())[:80])
        oms.append(None)
    else:
        oms.append(om)
print('oms', sum(o is not None for o in oms))
# dump mapping
out = {}
for m, om in zip(uniq, oms):
    if om is not None:
        out[m] = etree.tostring(om, encoding='unicode')
import json
json.dump(out, open('_omml.json', 'w', encoding='utf-8'), ensure_ascii=False, indent=0)
print('saved')
