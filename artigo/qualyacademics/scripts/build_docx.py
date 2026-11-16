# -*- coding: utf-8 -*-
"""Build artigo_qualyacademics.docx from the official template package."""
import os, re, json, copy, datetime, sys
from docx import Document
from docx.shared import Pt, Cm, Twips
from docx.enum.text import WD_ALIGN_PARAGRAPH
from docx.enum.table import WD_TABLE_ALIGNMENT, WD_CELL_VERTICAL_ALIGNMENT
from docx.oxml.ns import qn
from docx.oxml import OxmlElement
from lxml import etree

BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
QC = os.path.join(BASE, 'qualyacademics')
SCR = os.path.join(QC, 'scripts')
TEMPLATE = os.path.join(QC, 'modelo.docx')
OUT = os.path.join(QC, 'artigo_qualyacademics.docx')
FIGDIR = os.path.join(QC, 'figuras')

TEXT_WIDTH = 9214  # twips: 11909 - 1279 - 1416

MISSING_MATH = set()
UNKNOWN_CMDS = set()

# ---------------------------------------------------------------- constants
TITLE_PT = ('Análise do Impacto de um Pré-Processamento Simétrico $O(n)$ na '
            'Redução de Inversões e Eficiência de Algoritmos de Ordenação')
TITLE_EN = ('Analysis of the Impact of an $O(n)$ Symmetric Preprocessing on '
            'Inversion Reduction and Sorting Algorithm Efficiency')
KEYWORDS_PT = ('Ordenação; Inversões; Pré-processamento simétrico; '
               'Algoritmos adaptativos; Quicksort')
KEYWORDS_EN = ('Sorting; Inversions; Symmetric preprocessing; '
               'Adaptive algorithms; Quicksort')
AUTHORS = [('Samuel da Penha Nascimento', 'samuelpnascimentodev@gmail.com'),
           ('Francisco das Chagas Rocha', 'rocha@phb.uespi.br')]
AFFIL = ['Coordenação do Curso de Sistemas de Computação, Universidade Estadual do Piauí',
         'Campus Prof. Alexandre Alves de Oliveira',
         'Parnaíba, Piauí - Brasil',
         'E-mails: samuelpnascimentodev@gmail.com; rocha@phb.uespi.br']

CITE = {
    'cormen_introduction_2022': ('CORMEN et al.', '2022'),
    'estivill-castro_survey_1992': ('ESTIVILL-CASTRO; WOOD', '1992'),
    'sedgewick_algorithms_2011': ('SEDGEWICK; WAYNE', '2011'),
    'knuth_art_1973': ('KNUTH', '1973'),
    'mannila_measures_1984': ('MANNILA', '1985'),
    'ziviani_projeto_2021': ('ZIVIANI', '2021'),
    'gil_metodos_2019': ('GIL', '2019'),
    'sampieri_metodologia_2021': ('SAMPIERI; COLLADO; LUCIO', '2021'),
    'hwang_presorting_2000': ('HWANG; YANG; YEH', '2000'),
}

REFMAP = {
    'sec:fund': '2', 'sec:met': '3', 'sec:config': '3.1', 'sec:protocolo': '3.3',
    'sec:formal': '3.4', 'sec:ameacas': '3.5', 'sec:res': '4', 'sec:reducao': '4.1',
    'sec:custo': '4.2', 'sec:analise': '4.3', 'sec:ia': '6', 'sec:repositorio': '7',
    'tab:ambiente': '1', 'tab:datasets': '2', 'tab:inversoes': '3', 'tab:tempos': '4',
    'tab:baselines': '5', 'tab:novastopologias': '6', 'tab:precusto': '7',
    'tab:cpre': '8', 'tab:tempos1M': '9', 'tab:apA': '10', 'tab:apB': '11',
    'tab:apC': '12',
    'fig:inversoespos': '1', 'fig:precusto_linear': '2', 'fig:tempos': '3',
    'fig:ganho_n': '4', 'alg:preproc': '1',
}

TABLE_ALIGN = {
    'tab:ambiente': 'LL', 'tab:datasets': 'LL', 'tab:inversoes': 'LRRR',
    'tab:tempos': 'LRR', 'tab:baselines': 'LLRRR', 'tab:novastopologias': 'LRR',
    'tab:precusto': 'RRR', 'tab:cpre': 'LRR', 'tab:tempos1M': 'LRR',
    'tab:apA': 'LLRRR', 'tab:apB': 'LLRRR', 'tab:apC': 'LLRRR',
}
TABLE_FRACS = {
    'tab:ambiente': [0.30, 0.70], 'tab:datasets': [0.28, 0.72],
    'tab:inversoes': [0.28, 0.24, 0.24, 0.24], 'tab:tempos': [0.36, 0.32, 0.32],
    'tab:baselines': [0.20, 0.18, 0.20, 0.20, 0.22],
    'tab:novastopologias': [0.36, 0.32, 0.32],
    'tab:precusto': [0.30, 0.35, 0.35], 'tab:cpre': [0.28, 0.36, 0.36],
    'tab:tempos1M': [0.36, 0.32, 0.32],
    'tab:apA': [0.20, 0.18, 0.20, 0.20, 0.22],
    'tab:apB': [0.20, 0.18, 0.20, 0.20, 0.22],
    'tab:apC': [0.20, 0.18, 0.20, 0.20, 0.22],
}
FIG_FILES = {
    'fig:inversoespos': 'fig1_inversoes.png',
    'fig:precusto_linear': 'fig2_custo.png',
    'fig:tempos': 'fig3_tempos.png',
    'fig:ganho_n': 'fig4_ganho_n.png',
}

# ---------------------------------------------------------------- helpers
def q(tag):
    return qn('w:' + tag)


def read_group(s, j):
    n = len(s)
    while j < n and s[j] in ' \t\r\n':
        j += 1
    if j >= n or s[j] != '{':
        return None, j
    depth = 0
    k = j
    while k < n:
        if s[k] == '{':
            depth += 1
        elif s[k] == '}':
            depth -= 1
            if depth == 0:
                return s[j + 1:k], k + 1
        k += 1
    return None, j


def fmt_cite(arg):
    keys = [k.strip() for k in arg.split(',') if k.strip()]
    parts = []
    for k in keys:
        if k in CITE:
            a, y = CITE[k]
            parts.append('%s, %s' % (a, y))
        else:
            UNKNOWN_CMDS.add('cite:' + k)
            parts.append(k.upper() + ', ?')
    parts.sort()
    return '(' + '; '.join(parts) + ')'


def prep(s):
    s = s.replace(r'Mannila~\cite{mannila_measures_1984}', 'Mannila (1985)')
    s = s.replace(r'Hwang, Yang e Yeh~\cite{hwang_presorting_2000}',
                  'Hwang, Yang e Yeh (2000)')
    return s


ESCAPES = {'%': '%', '&': '&', '_': '_', '#': '#', '{': '{', '}': '}',
           '$': '$', ' ': ' '}
SIMPLE_CMDS = {'ldots': '\u2026', 'dots': '\u2026', 'textmu': '\u00b5',
               'textendash': '\u2013', 'textemdash': '\u2014',
               'dagger': '\u2020', 'textdagger': '\u2020'}


def tokenize(s, fmt):
    """Return list of ('t', text, fmt) and ('m', expr, fmt)."""
    nodes = []
    buf = []

    def flush():
        if buf:
            txt = ''.join(buf)
            txt = re.sub(r'[ \t\r\n]+', ' ', txt)
            if txt:
                nodes.append(('t', txt, fmt))
            buf.clear()

    i = 0
    n = len(s)
    while i < n:
        c = s[i]
        if s.startswith('\\(', i):
            k = s.find('\\)', i + 2)
            if k != -1:
                flush()
                nodes.append(('m', s[i + 2:k].strip(), fmt))
                i = k + 2
                continue

        if c == '\\':
            m = re.match(r'\\([A-Za-z]+)', s[i:])
            if m:
                name = m.group(1)
                j = i + m.end()
                if name in ('textbf', 'emph', 'textit', 'texttt', 'textsuperscript',
                            'text', 'url', 'mathrm', 'mathbf', 'mathit',
                            'textnormal', 'textrm', 'operatorname'):
                    arg, j2 = read_group(s, j)
                    if arg is None:
                        i = j
                        continue
                    flush()
                    if name == 'textbf':
                        nodes.extend(tokenize(arg, (True, fmt[1], fmt[2], fmt[3])))
                    elif name in ('emph', 'textit'):
                        nodes.extend(tokenize(arg, (fmt[0], True, fmt[2], fmt[3])))
                    elif name == 'texttt':
                        nodes.extend(tokenize(arg, (fmt[0], fmt[1], True, fmt[3])))
                    elif name == 'textsuperscript':
                        nodes.extend(tokenize(arg, (fmt[0], fmt[1], fmt[2], True)))
                    elif name == 'url':
                        flush()
                        nodes.append(('t', arg.replace('\\_', '_'), fmt))
                    else:
                        nodes.extend(tokenize(arg, fmt))
                    i = j2
                    continue
                if name == 'cite':
                    arg, j2 = read_group(s, j)
                    if arg is not None:
                        flush()
                        nodes.append(('t', fmt_cite(arg), fmt))
                        i = j2
                        continue
                if name == 'ref':
                    arg, j2 = read_group(s, j)
                    if arg is not None:
                        flush()
                        nodes.append(('t', REFMAP.get(arg, '?'), fmt))
                        i = j2
                        continue
                if name in ('label', 'footnotesize', 'small', 'scriptsize',
                            'centering', 'noindent', 'vspace', 'hspace'):
                    arg, j2 = read_group(s, j)
                    i = j2 if arg is not None else j
                    continue
                if name in SIMPLE_CMDS:
                    buf.append(SIMPLE_CMDS[name])
                    i = j
                    continue
                UNKNOWN_CMDS.add(name)
                i = j
                continue
            # escaped single character
            ch = s[i + 1] if i + 1 < n else ''
            if ch in ESCAPES:
                buf.append(ESCAPES[ch])
            elif ch == ',':
                buf.append('\u2009')
            elif ch in (';', ':', '\\'):
                buf.append(' ')
            else:
                buf.append(ch)
            i += 2
            continue

        if c == '$':
            k = i + 1
            while k < n and not (s[k] == '$' and s[k - 1] != '\\'):
                k += 1
            if k < n:
                flush()
                nodes.append(('m', s[i + 1:k].strip(), fmt))
                i = k + 1
                continue
            buf.append('$')
            i += 1
            continue
        if c == '~':
            buf.append('\u00a0')
            i += 1
            continue
        if c in '{}':
            i += 1
            continue
        if s.startswith('---', i):
            buf.append('\u2014')
            i += 3
            continue
        if s.startswith('--', i):
            buf.append('\u2013')
            i += 2
            continue
        if s.startswith('``', i):
            buf.append('\u201c')
            i += 2
            continue
        if s.startswith("''", i):
            buf.append('\u201d')
            i += 2
            continue
        buf.append(c)
        i += 1
    flush()
    return nodes


def style_run(run, size, bold=False, italic=False, mono=False, sup=False):
    fname = 'Courier New' if mono else 'Times New Roman'
    run.font.name = fname
    rPr = run._element.get_or_add_rPr()
    rf = rPr.find(q('rFonts'))
    if rf is not None:
        rf.set(q('eastAsia'), fname)
        rf.set(q('cs'), fname)
    run.font.size = Pt(size if not mono else max(8, size - 1))
    run.bold = bool(bold)
    run.italic = bool(italic)
    if sup:
        run.font.superscript = True


def add_run(p, text, size=12, bold=False, italic=False, mono=False, sup=False):
    if text == '':
        return None
    r = p.add_run(text)
    style_run(r, size, bold, italic, mono, sup)
    return r


def style_math(om, size, fmt):
    bold, italic = fmt[0], fmt[1]
    hp = str(int(round(size * 2)))
    for mr in om.iter(qn('m:r')):
        mrpr = mr.find(qn('m:rPr'))
        wrpr = mr.find(qn('w:rPr'))
        if wrpr is None:
            wrpr = OxmlElement('w:rPr')
            if mrpr is not None:
                mr.insert(1, wrpr)
            else:
                mr.insert(0, wrpr)
        for tag in ('w:b', 'w:bCs', 'w:i', 'w:iCs', 'w:sz', 'w:szCs'):
            for e in wrpr.findall(qn(tag)):
                wrpr.remove(e)
        if bold:
            for tag in ('w:b', 'w:bCs'):
                e = OxmlElement(tag)
                wrpr.append(e)
        if italic:
            for tag in ('w:i', 'w:iCs'):
                e = OxmlElement(tag)
                wrpr.append(e)
        for tag in ('w:sz', 'w:szCs'):
            e = OxmlElement(tag)
            e.set(qn('w:val'), hp)
            wrpr.append(e)


def emit_math(p, expr, fmt, size):
    expr = expr.strip()
    if expr in ('^{*}', '^{\\ast}'):
        add_run(p, '*', size, fmt[0], fmt[1], fmt[2], True)
        return
    if expr == '^{\\dagger}':
        add_run(p, '\u2020', size, fmt[0], fmt[1], fmt[2], True)
        return
    if re.match(r'^=[0-9., ]+$', expr):
        add_run(p, expr, size, fmt[0], fmt[1], fmt[2], fmt[3])
        return
    xml = OMML.get(expr)
    if not xml:
        MISSING_MATH.add(expr)
        add_run(p, expr, size, fmt[0], fmt[1], fmt[2], fmt[3])
        return
    om = etree.fromstring(xml.encode('utf-8'))
    style_math(om, size, fmt)
    p._p.append(om)


def add_rich(p, s, size=12, bold=False, italic=False, mono=False, sup=False):
    for kind, val, fmt in tokenize(prep(s), (bold, italic, mono, sup)):
        if kind == 't':
            add_run(p, val, size, fmt[0], fmt[1], fmt[2], fmt[3])
        else:
            emit_math(p, val, fmt, size)


def set_pf(p, jc='both', first=0, left=0, before=0, after=0, line=1.5,
           keep_next=False, keep_together=False, page_break=False):
    pf = p.paragraph_format
    pf.alignment = {'both': WD_ALIGN_PARAGRAPH.JUSTIFY,
                    'left': WD_ALIGN_PARAGRAPH.LEFT,
                    'center': WD_ALIGN_PARAGRAPH.CENTER,
                    'right': WD_ALIGN_PARAGRAPH.RIGHT}[jc]
    pf.first_line_indent = Twips(first)
    pf.left_indent = Twips(left)
    pf.space_before = Pt(before)
    pf.space_after = Pt(after)
    pf.line_spacing = line
    pf.keep_with_next = bool(keep_next)
    pf.keep_together = bool(keep_together)
    if page_break:
        pf.page_break_before = True


def add_body(text, first=709, before=0, after=0, line=1.5, page_break=False,
             keep_next=False):
    p = doc.add_paragraph()
    set_pf(p, 'both', first, 0, before, after, line, keep_next, False, page_break)
    add_rich(p, text)
    return p


def add_heading(text, level, page_break=False):
    p = doc.add_paragraph()
    if level == 1:
        set_pf(p, 'left', 0, 0, 12, 6, 1.5, True, False, page_break)
        add_rich(p, text.upper(), size=16, bold=True)
    elif level == 2:
        set_pf(p, 'left', 0, 0, 9, 3, 1.5, True, False, page_break)
        add_rich(p, text, size=12, bold=True)
    else:
        set_pf(p, 'left', 0, 0, 6, 3, 1.5, True, False, page_break)
        add_rich(p, text, size=12, bold=True, italic=True)
    return p


def add_caption(prefix, text, above=True):
    p = doc.add_paragraph()
    set_pf(p, 'center', 0, 0, 6 if above else 3, 3 if above else 10, 1.0,
           above, True, False)
    add_run(p, prefix, 10, True)
    add_rich(p, text, size=10)
    return p


def set_table_borders(table):
    tblPr = table._tbl.tblPr
    for e in tblPr.findall(q('tblBorders')):
        tblPr.remove(e)
    borders = OxmlElement('w:tblBorders')
    for edge, val, sz in (('top', 'single', '8'), ('left', 'nil', '0'),
                          ('bottom', 'single', '8'), ('right', 'nil', '0'),
                          ('insideH', 'nil', '0'), ('insideV', 'nil', '0')):
        e = OxmlElement('w:' + edge)
        e.set(qn('w:val'), val)
        e.set(qn('w:sz'), sz)
        e.set(qn('w:space'), '0')
        e.set(qn('w:color'), 'auto')
        borders.append(e)
    tblPr.insert_element_before(borders, 'w:shd', 'w:tblLayout', 'w:tblCellMar',
                                'w:tblLook', 'w:tblCaption', 'w:tblDescription',
                                'w:tblPrChange')


def set_cell_border(cell, edge, sz='8'):
    tcPr = cell._tc.get_or_add_tcPr()
    tcb = tcPr.find(q('tcBorders'))
    if tcb is None:
        tcb = OxmlElement('w:tcBorders')
        tcPr.insert_element_before(tcb, 'w:shd', 'w:noWrap', 'w:tcMar',
                                   'w:textDirection', 'w:tcFitText', 'w:vAlign',
                                   'w:hideMark', 'w:headers', 'w:cellIns',
                                   'w:cellDel', 'w:cellMerge', 'w:tcPrChange')
    for e in tcb.findall(q(edge)):
        tcb.remove(e)
    e = OxmlElement('w:' + edge)
    e.set(qn('w:val'), 'single')
    e.set(qn('w:sz'), sz)
    e.set(qn('w:space'), '0')
    e.set(qn('w:color'), 'auto')
    tcb.append(e)


def set_table_layout(table, widths):
    tbl = table._tbl
    tblPr = tbl.tblPr
    tblW = tblPr.find(q('tblW'))
    if tblW is None:
        tblW = OxmlElement('w:tblW')
        tblPr.insert_element_before(tblW, 'w:jc', 'w:tblCellSpacing', 'w:tblInd',
                                    'w:tblBorders', 'w:shd', 'w:tblLayout',
                                    'w:tblCellMar', 'w:tblLook')
    tblW.set(qn('w:w'), str(sum(widths)))
    tblW.set(qn('w:type'), 'dxa')
    layout = tblPr.find(q('tblLayout'))
    if layout is None:
        layout = OxmlElement('w:tblLayout')
        tblPr.insert_element_before(layout, 'w:tblCellMar', 'w:tblLook')
    layout.set(qn('w:type'), 'fixed')
    mar = tblPr.find(q('tblCellMar'))
    if mar is None:
        mar = OxmlElement('w:tblCellMar')
        tblPr.insert_element_before(mar, 'w:tblLook')
    for child in list(mar):
        mar.remove(child)
    for side, w in (('top', 30), ('left', 80), ('bottom', 30), ('right', 80)):
        e = OxmlElement('w:' + side)
        e.set(qn('w:w'), str(w))
        e.set(qn('w:type'), 'dxa')
        mar.append(e)
    for j, w in enumerate(widths):
        table.columns[j].width = Twips(w)
        for cell in table.columns[j].cells:
            cell.width = Twips(w)


def add_table_block(block):
    label = block['label']
    n = TABLE_NO[label]
    rows = block['rows']
    ncols = len(rows[0])
    add_caption('Tabela %d. ' % n, block['caption'], above=True)
    table = doc.add_table(rows=len(rows), cols=ncols)
    table.alignment = WD_TABLE_ALIGNMENT.CENTER
    table.autofit = False
    widths = [int(TEXT_WIDTH * f) for f in TABLE_FRACS[label]]
    widths[-1] = TEXT_WIDTH - sum(widths[:-1])
    set_table_layout(table, widths)
    set_table_borders(table)
    aligns = TABLE_ALIGN[label]
    cell_size = 10 if label in ('tab:ambiente', 'tab:datasets') else 9
    amap = {'L': 'left', 'R': 'right', 'C': 'center'}
    for ri, row in enumerate(rows):
        tr = table.rows[ri]
        trPr = tr._tr.get_or_add_trPr()
        trPr.append(OxmlElement('w:cantSplit'))
        if ri == 0:
            trPr.append(OxmlElement('w:tblHeader'))
        for ci, txt in enumerate(row):
            cell = table.cell(ri, ci)
            cell.vertical_alignment = WD_CELL_VERTICAL_ALIGNMENT.CENTER
            p = cell.paragraphs[0]
            set_pf(p, amap[aligns[ci]], 0, 0, 0, 0, 1.0, False, False, False)
            add_rich(p, txt, size=cell_size, bold=(ri == 0))
        if ri == 0 or ri in block.get('rules', [])[1:]:
            for ci in range(ncols):
                set_cell_border(table.cell(ri, ci), 'bottom' if ri == 0 else 'top')
    p = doc.add_paragraph()
    set_pf(p, 'both', 0, 0, 0, 0, 1.0, False, False, False)
    add_run(p, ' ', size=2)
    if block.get('note'):
        p = doc.add_paragraph()
        set_pf(p, 'both', 0, 0, 3, 8, 1.0, False, False, False)
        add_rich(p, block['note'], size=9)


def add_figure_block(block):
    label = block['label']
    n = FIG_NO[label]
    path = os.path.join(FIGDIR, FIG_FILES[label])
    from PIL import Image
    px = Image.open(path).size[0]
    w_cm = min(16.0, px / 600.0 * 2.54)
    p = doc.add_paragraph()
    set_pf(p, 'center', 0, 0, 6, 2, 1.0, True, True, False)
    run = p.add_run()
    run.add_picture(path, width=Cm(w_cm))
    add_caption('Figura %d. ' % n, block['caption'], above=False)


def algo_line(raw):
    line = raw.strip()
    m = re.match(r'\\KwIn\{(.*)\}$', line)
    if m:
        return r'\textbf{Input:} ' + m.group(1)
    m = re.match(r'\\KwOut\{(.*)\}$', line)
    if m:
        return r'\textbf{Output:} ' + m.group(1)
    m = re.match(r'\\If\{(.*)\}$', line)
    if m:
        return r'\textbf{if} ' + m.group(1) + r' \textbf{then}'
    m = re.match(r'\\For\{(.*)\}$', line)
    if m:
        cond = m.group(1).replace(r'\KwTo', r'\textbf{to}')
        return r'\textbf{for} ' + cond + r' \textbf{do}'
    line = line.replace(r'\KwTo', r'\textbf{to}')
    line = line.replace(r'\Return', r'\textbf{return}')
    line = line.replace(r'\;', '')
    return line


def add_algorithm_block(block):
    n = ALG_NO[block['label']]
    add_caption('Algoritmo %d. ' % n, block['caption'], above=True)
    table = doc.add_table(rows=1, cols=1)
    table.alignment = WD_TABLE_ALIGNMENT.CENTER
    table.autofit = False
    set_table_layout(table, [TEXT_WIDTH])
    set_table_borders(table)
    cell = table.cell(0, 0)
    cell.vertical_alignment = WD_CELL_VERTICAL_ALIGNMENT.CENTER
    depth = 0
    first = True
    for raw in block['lines']:
        line = raw.strip()
        if line == '}':
            depth = max(0, depth - 1)
            continue
        opens = line.endswith('{')
        if opens:
            line = line[:-1].rstrip()
        text = algo_line(line)
        p = cell.paragraphs[0] if first else cell.add_paragraph()
        first = False
        set_pf(p, 'left', 0, depth * 283, 0, 0, 1.0, False, False, False)
        add_rich(p, text, size=10)


def add_refs():
    add_heading('Referências', 1, page_break=True)
    for segs in REFS:
        p = doc.add_paragraph()
        set_pf(p, 'left', 0, 0, 0, 10, 1.0, False, False, False)
        for txt, bold, italic in segs:
            add_run(p, txt, 12, bold, italic)


# ---------------------------------------------------------------- data
blocks = json.load(open(os.path.join(SCR, '_blocks.json'), encoding='utf-8'))
OMML = json.load(open(os.path.join(SCR, '_omml.json'), encoding='utf-8'))

TABLE_NO = {}
FIG_NO = {}
ALG_NO = {}
_t = _f = _a = 0
for b in blocks:
    if b['type'] == 'table':
        _t += 1
        TABLE_NO[b['label']] = _t
    elif b['type'] == 'figure':
        _f += 1
        FIG_NO[b['label']] = _f
    elif b['type'] == 'algorithm':
        _a += 1
        ALG_NO[b['label']] = _a

REFS = [
    [('CORMEN, Thomas H. et al. ', False, False),
     ('Introduction to Algorithms', True, False),
     ('. 4. ed. MIT Press, 2022.', False, False)],
    [('ESTIVILL-CASTRO, Vladmir; WOOD, Derick. A survey of adaptive sorting algorithms. ', False, False),
     ('ACM Computing Surveys', True, False),
     (', v. 24, n. 4, p. 441-476, 1992. DOI: 10.1145/146370.146381.', False, False)],
    [('GIL, Antonio Carlos. ', False, False),
     ('Métodos e técnicas de pesquisa social', True, False),
     ('. Editora Atlas Ltda, 2019. ISBN 978-85-97-02057-1.', False, False)],
    [('HWANG, Hsien-Kuei; YANG, Bo-Yin; YEH, Yeong-Nan. Presorting algorithms: An average-case point of view. ', False, False),
     ('Theoretical Computer Science', True, False),
     (', v. 242, n. 1-2, p. 29-40, 2000. DOI: 10.1016/S0304-3975(98)00181-9.', False, False)],
    [('KNUTH, Donald E. ', False, False),
     ('The Art of Computer Programming, Volume 3: Sorting and Searching', True, False),
     ('. Reading, MA: Addison-Wesley, 1973.', False, False)],
    [('MANNILA, Heikki. Measures of presortedness and optimal sorting algorithms. ', False, False),
     ('IEEE Transactions on Computers', True, False),
     (', v. C-34, n. 4, p. 318-325, 1985. DOI: 10.1109/TC.1985.5009382. '
      'Versão estendida publicada originalmente como resumo no ICALP 1984 '
      '(LNCS 172, pp. 324-336).', False, False)],
    [('SAMPIERI, Roberto Hernandez; COLLADO, Carlos Fernández; LUCIO, María del Pilar Baptista. ', False, False),
     ('Metodologia de pesquisa', True, False),
     ('. Penso, 2021. ISBN 978-85-65848-28-2.', False, False)],
    [('SEDGEWICK, Robert; WAYNE, Kevin Daniel. ', False, False),
     ('Algorithms', True, False),
     ('. 4. ed. Addison-Wesley, 2011. ISBN 978-0-321-57351-3.', False, False)],
    [('ZIVIANI, Nivio. ', False, False),
     ('Projeto de Algoritmos com Implementações em Java e C++', True, False),
     ('. Cengage Learning, 2021. ISBN 978-85-221-0525-0.', False, False)],
]

# ---------------------------------------------------------------- build
doc = Document(TEMPLATE)
# habilitar hifenização automática no settings.xml
settings = doc.settings.element
ah = settings.find(qn('w:autoHyphenation'))
if ah is None:
    ah = OxmlElement('w:autoHyphenation')
    settings.append(ah)
ah.set(qn('w:val'), 'true')
body = doc.element.body
for child in list(body):
    if child.tag == qn('w:sectPr'):
        continue
    body.remove(child)

# ----- title page
p = doc.add_paragraph()
set_pf(p, 'center', 0, 0, 0, 6, 1.0, False, False, False)
add_rich(p, TITLE_PT, size=16, bold=True)

p = doc.add_paragraph()
set_pf(p, 'center', 0, 0, 0, 6, 1.0, False, False, False)
add_rich(p, TITLE_EN, size=16, italic=True)

for name, _email in AUTHORS:
    p = doc.add_paragraph()
    set_pf(p, 'right', 0, 0, 0, 0, 1.0, False, False, False)
    add_run(p, name, 12, True)

for line in AFFIL:
    p = doc.add_paragraph()
    set_pf(p, 'right', 0, 0, 0, 0, 1.0, False, False, False)
    add_run(p, line, 10)

p = doc.add_paragraph()
set_pf(p, 'center', 0, 0, 12, 6, 1.0, True, False, False)
add_run(p, 'RESUMO', 16, True)

ABSTRACT_PT = [b for b in blocks if b['type'] == 'abstract_pt'][0]['tex']
ABSTRACT_EN = [b for b in blocks if b['type'] == 'abstract_en'][0]['tex']
p = doc.add_paragraph()
set_pf(p, 'both', 0, 0, 0, 0, 1.0, False, False, False)
add_rich(p, ABSTRACT_PT, size=12)

p = doc.add_paragraph()
set_pf(p, 'both', 0, 0, 6, 12, 1.0, False, False, False)
add_rich(p, r'\textbf{Palavras-chave:} ' + KEYWORDS_PT + '.', size=12)

p = doc.add_paragraph()
set_pf(p, 'center', 0, 0, 12, 6, 1.0, True, False, False)
add_run(p, 'ABSTRACT', 16, True)

p = doc.add_paragraph()
set_pf(p, 'both', 0, 0, 0, 0, 1.0, False, False, False)
add_rich(p, ABSTRACT_EN, size=12)

p = doc.add_paragraph()
set_pf(p, 'both', 0, 0, 6, 0, 1.0, False, False, False)
add_rich(p, r'\textbf{Keywords:} ' + KEYWORDS_EN + '.', size=12)

# ----- sections
lvl1 = lvl2 = lvl3 = 0
pending_break = False
for b in blocks:
    t = b['type']
    if t in ('abstract_pt', 'abstract_en'):
        continue
    if t == 'pagebreak':
        pending_break = True
        continue
    if t == 'section':
        lvl = b['level']
        if lvl == 1:
            if b['appendix']:
                prefix = 'A. '
            else:
                lvl1 += 1
                lvl2 = lvl3 = 0
                prefix = '%d. ' % lvl1
            add_heading(prefix + b['title'], 1, page_break=pending_break)
        elif lvl == 2:
            lvl2 += 1
            lvl3 = 0
            add_heading('%d.%d %s' % (lvl1, lvl2, b['title']), 2,
                        page_break=pending_break)
        else:
            lvl3 += 1
            add_heading('%d.%d.%d %s' % (lvl1, lvl2, lvl3, b['title']), 3,
                        page_break=pending_break)
        pending_break = False
        continue
    if t == 'para':
        add_body(b['tex'], page_break=pending_break)
        pending_break = False
    elif t == 'list':
        for k, item in enumerate(b['items'], 1):
            p = doc.add_paragraph()
            set_pf(p, 'both', 0, 709, 0, 0, 1.5, False, False, pending_break)
            pending_break = False
            pf = p.paragraph_format
            pf.first_line_indent = Twips(-283)
            marker = ('%d.\t' % k) if b['ordered'] else '\u2022\t'
            add_run(p, marker)
            add_rich(p, item)
    elif t == 'table':
        add_table_block(b)
        pending_break = False
    elif t == 'figure':
        add_figure_block(b)
        pending_break = False
    elif t == 'algorithm':
        add_algorithm_block(b)
        pending_break = False
    elif t == 'refs':
        add_refs()
        pending_break = False

# ----- headers / footers
sec = doc.sections[0]
for part, old, new in (
        (sec.header, 'ESTUDO DE CASO', 'ARTIGO'),
        (sec.first_page_header, 'TEMPLATE', 'ARTIGO')):
    for para in part.paragraphs:
        for run in para.runs:
            if old in run.text:
                run.text = run.text.replace(old, new)

footer = sec.first_page_footer
repl = {
    '(Esses dados serão colocados pela editora)': '',
    ' FULANO,': ' NASCIMENTO,',
    'Beltrano de Lima; SEGUNDO, Autor se tiver':
        'Samuel da Penha; ROCHA, Francisco das Chagas',
    'Título do artigo aqui':
        'Análise do Impacto de um Pré-Processamento Simétrico O(n) na Redução '
        'de Inversões e Eficiência de Algoritmos de Ordenação',
    ': ': '',
    'subtítulo e': '',
}
for para in footer.paragraphs:
    for run in para.runs:
        if run.text in repl:
            run.text = repl[run.text]

# ----- core properties
cp = doc.core_properties
cp.title = TITLE_PT.replace('$O(n)$', 'O(n)')
cp.author = '; '.join(a for a, _ in AUTHORS)
cp.subject = 'Artigo'
cp.keywords = KEYWORDS_PT
cp.comments = ('Documento produzido a partir do template oficial da Revista '
               'QUALYACADEMICS (Editora UNISV).')
cp.last_modified_by = AUTHORS[0][0]
cp.category = 'Artigo'
cp.modified = datetime.datetime.now(datetime.timezone.utc)

doc.save(OUT)

print('saved', OUT)
print('unknown commands:', sorted(UNKNOWN_CMDS))
print('missing math:', sorted(MISSING_MATH))
print('tables', len(TABLE_NO), 'figures', len(FIG_NO), 'algorithm', len(ALG_NO))
