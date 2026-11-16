import zipfile, re, json, os
BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
z = zipfile.ZipFile(os.path.join(BASE, 'qualyacademics', 'modelo.docx'))
x = z.read('word/styles.xml').decode('utf-8')
print('STYLE IDS:', re.findall(r'w:styleId="([^"]+)"', x))
for sid in ('Normal', 'Ttulo1', 'Ttulo2', 'Ttulo3'):
    mm = re.search(r'<w:style [^>]*w:styleId="%s".*?</w:style>' % sid, x, re.S)
    print('==', sid, '==')
    print(mm.group(0)[:800] if mm else 'none')

print('\n=== OMML sample ===')
om = json.load(open(os.path.join(BASE, 'qualyacademics', 'scripts', '_omml.json'), encoding='utf-8'))
print(type(om), len(om))
for k in list(om)[:3]:
    print('KEY:', repr(k)[:120])
    print('VAL:', om[k][:400])
    print('---')
