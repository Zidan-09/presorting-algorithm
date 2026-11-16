import json, os
from lxml import etree
BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
om = json.load(open(os.path.join(BASE, 'qualyacademics', 'scripts', '_omml.json'), encoding='utf-8'))
for k in ['O(n)', 'C_{\\text{pre}}', '10^7']:
    print('KEY', k)
    x = etree.fromstring(om[k])
    print(etree.tostring(x, pretty_print=True).decode()[:2000])
    print('=====')
