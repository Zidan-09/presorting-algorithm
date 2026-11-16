import json, re, os
BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
b = json.load(open(os.path.join(BASE, 'qualyacademics', 'scripts', '_blocks.json'), encoding='utf-8'))
texts = []
for x in b:
    if x['type'] in ('para', 'abstract_en', 'abstract_pt'):
        texts.append(x['tex'])
    elif x['type'] == 'list':
        texts.extend(x['items'])
for t in texts:
    for m in re.findall(r'\\cite\{([^}]*)\}', t):
        print(m)
print('--- refs ---')
for t in texts:
    for m in re.finditer(r'[^.]*\\ref\{[^}]*\}[^.]*\.', t):
        print(m.group(0)[:300])
