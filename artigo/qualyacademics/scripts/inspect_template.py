import sys
from lxml import etree

W = 'http://schemas.openxmlformats.org/wordprocessingml/2006/main'
def q(t): return '{%s}%s' % (W, t)

def ppr_summary(p):
    ppr = p.find(q('pPr'))
    if ppr is None: return '{}'
    out = {}
    st = ppr.find(q('pStyle'))
    if st is not None: out['style'] = st.get(q('val'))
    jc = ppr.find(q('jc'))
    if jc is not None: out['jc'] = jc.get(q('val'))
    sp = ppr.find(q('spacing'))
    if sp is not None: out['spacing'] = {k.split('}')[-1]: v for k, v in sp.attrib.items()}
    ind = ppr.find(q('ind'))
    if ind is not None: out['ind'] = {k.split('}')[-1]: v for k, v in ind.attrib.items()}
    for tag in ('keepNext','keepLines','pageBreakBefore'):
        e = ppr.find(q(tag))
        if e is not None: out[tag] = e.get(q('val'), 'true')
    nb = ppr.find(q('numPr'))
    if nb is not None:
        ilvl = nb.find(q('ilvl')); numid = nb.find(q('numId'))
        out['numPr'] = (ilvl.get(q('val')) if ilvl is not None else None,
                        numid.get(q('val')) if numid is not None else None)
    shd = ppr.find(q('shd'))
    if shd is not None: out['shd'] = shd.get(q('fill'))
    pb = ppr.find(q('pBdr'))
    if pb is not None:
        out['pBdr'] = [(c.tag.split('}')[-1], c.get(q('val'))) for c in pb]
    return out

def rpr_summary(r):
    rpr = r.find(q('rPr'))
    if rpr is None: return {}
    out = {}
    rf = rpr.find(q('rFonts'))
    if rf is not None: out['fonts'] = {k.split('}')[-1]: v for k, v in rf.attrib.items()}
    for tag in ('b','i','u','caps','smallCaps','strike','vertAlign','vanish'):
        e = rpr.find(q(tag))
        if e is not None: out[tag] = e.get(q('val'), 'true')
    for tag in ('sz','szCs','color','highlight'):
        e = rpr.find(q(tag))
        if e is not None: out[tag] = e.get(q('val'))
    return out

def dump(path):
    t = etree.parse(path)
    root = t.getroot()
    body = root.find(q('body'))
    n = 0
    for child in body:
        tag = child.tag.split('}')[-1]
        if tag == 'p':
            txt = ''.join(child.itertext())
            print(f'[{n}] {ppr_summary(child)}')
            if txt.strip():
                print('    TEXT:', repr(txt))
            for r in child.findall(q('r')):
                rt = ''.join(r.itertext())
                if rt:
                    print('     RUN', rpr_summary(r), repr(rt[:100]))
            for br in child.findall('.//' + q('br')):
                print('     BR', br.get(q('type')))
            n += 1
        elif tag == 'tbl':
            print(f'[{n}] TABLE')
            n += 1
    sp = body.find(q('sectPr'))
    if sp is not None:
        print('SECTPR:')
        for c in sp:
            print('  ', c.tag.split('}')[-1], {k.split('}')[-1]: v for k, v in c.attrib.items()})

if __name__ == '__main__':
    dump(sys.argv[1])
