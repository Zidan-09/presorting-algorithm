import json, re, os, collections
BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
b = json.load(open(os.path.join(BASE, 'qualyacademics', 'scripts', '_blocks.json'), encoding='utf-8'))
cmds = collections.Counter()
texts = []
for x in b:
    if x['type'] in ('para', 'abstract_en', 'abstract_pt', 'list'):
        if x['type'] == 'list':
            texts.extend(x['items'])
        else:
            texts.append(x['tex'])
    elif x['type'] == 'table':
        for r in x['rows']:
            texts.extend(r)
        texts.append(x.get('note', ''))
for t in texts:
    for m in re.findall(r'\\([A-Za-z]+)', t):
        cmds[m] += 1
print('COMMANDS:')
for k, v in cmds.most_common():
    print(f'  {k:22s} {v}')

print('\nSPECIAL chars/patterns:')
for pat in ['---', '--', '~', '\\\\%', '\\\\&', '$', '\\{', '\\}', '``', "''"]:
    print(repr(pat), sum(t.count(pat) for t in texts))

print('\nTABLE CELL SAMPLES:')
for x in b:
    if x['type'] == 'table':
        print('==', x['label'], '==')
        for r in x['rows'][:3]:
            print(' | '.join(repr(c) for c in r))
