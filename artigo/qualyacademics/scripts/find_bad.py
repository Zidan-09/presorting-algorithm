import json, re, os
BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
b = json.load(open(os.path.join(BASE, 'qualyacademics', 'scripts', '_blocks.json'), encoding='utf-8'))
om = json.load(open(os.path.join(BASE, 'qualyacademics', 'scripts', '_omml.json'), encoding='utf-8'))
texts = []
for x in b:
    if x['type'] in ('para', 'abstract_en', 'abstract_pt'):
        texts.append(x['tex'])
    elif x['type'] == 'list':
        texts.extend(x['items'])
    elif x['type'] == 'table':
        for r in x['rows']:
            texts.extend(r)
        texts.append(x.get('note', ''))
    elif x['type'] in ('figure', 'algorithm'):
        texts.append(x.get('caption', ''))
        for ln in x.get('lines', []):
            texts.append(ln)

bad = []
for t in texts:
    # strip math spans, then look for math commands left outside
    outside = re.sub(r'(?<!\\)\$.+?(?<!\\)\$', ' ', t, flags=re.S)
    for m in re.finditer(r'\\(approx|geq|mu|KwTo)\b', outside):
        s = max(0, m.start() - 60)
        bad.append((m.group(1), t[s:m.end() + 40]))
for k, ctx in bad:
    print('==', k, '==')
    print(ctx)
    print()

print('MATH SPANS NOT IN OMML:')
for t in texts:
    for m in re.findall(r'(?<!\\)\$(.+?)(?<!\\)\$', t, flags=re.S):
        e = m.strip()
        if e not in om and e not in ('^{*}', '^{\\dagger}'):
            print(repr(e), '   in:', repr(t[:90]))
