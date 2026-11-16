# -*- coding: utf-8 -*-
"""Ensure every math span used by the parsed blocks has an OMML entry."""
import json, os, re, subprocess, zipfile
from lxml import etree

BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SCR = os.path.join(BASE, 'qualyacademics', 'scripts')
PANDOC = r'C:\Users\acemilton\AppData\Local\Pandoc\pandoc.exe'
M = 'http://schemas.openxmlformats.org/officeDocument/2006/math'
W = 'http://schemas.openxmlformats.org/wordprocessingml/2006/main'

blocks = json.load(open(os.path.join(SCR, '_blocks.json'), encoding='utf-8'))
omml = json.load(open(os.path.join(SCR, '_omml.json'), encoding='utf-8'))

texts = []
for b in blocks:
    if b['type'] in ('para', 'abstract_en', 'abstract_pt'):
        texts.append(b['tex'])
    elif b['type'] == 'list':
        texts.extend(b['items'])
    elif b['type'] == 'table':
        for r in b['rows']:
            texts.extend(r)
        texts.append(b.get('note', ''))
    elif b['type'] in ('figure', 'algorithm'):
        texts.append(b.get('caption', ''))
        texts.extend(b.get('lines', []))

exprs = []
for t in texts:
    for m in re.findall(r'(?<!\\)\$(.+?)(?<!\\)\$|\\\((.+?)\\\)', t, flags=re.S):
        e = (m[0] or m[1]).strip()
        if e and e not in ('^{*}', '^{\\dagger}') and e not in exprs:
            exprs.append(e)

missing = [e for e in exprs if e not in omml]
print('total math', len(exprs), 'missing', len(missing))
if not missing:
    raise SystemExit(0)
for e in missing:
    print('  ', repr(e))

md = '\n\n'.join('$' + e + '$' for e in missing)
mdpath = os.path.join(SCR, '_math_missing.md')
docxpath = os.path.join(SCR, '_math_missing.docx')
open(mdpath, 'w', encoding='utf-8').write(md + '\n')
subprocess.run([PANDOC, mdpath, '-o', docxpath], check=True)
z = zipfile.ZipFile(docxpath)
root = etree.fromstring(z.read('word/document.xml'))
paras = root.findall('.//{%s}p' % W)
assert len(paras) == len(missing), (len(paras), len(missing))
for e, p in zip(missing, paras):
    om = p.find('.//{%s}oMath' % M)
    assert om is not None, e
    omml[e] = etree.tostring(om, encoding='unicode')
json.dump(omml, open(os.path.join(SCR, '_omml.json'), 'w', encoding='utf-8'),
          ensure_ascii=False, indent=0)
print('added', len(missing), 'now', len(omml))
