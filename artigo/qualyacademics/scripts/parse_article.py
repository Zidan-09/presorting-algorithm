import re, json, os, sys

BASE = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
TEX = os.path.join(BASE, 'main.tex')
RES = os.path.join(BASE, 'resultados')


def read_tex(path):
    return open(path, encoding='utf-8').read()


def strip_caption(line):
    s = line.strip()
    assert s.startswith(r'\caption{'), s
    s = s[len(r'\caption{'):]
    # remove final closing brace (caption is one line)
    if s.endswith('}'):
        s = s[:-1]
    return s


def parse_tabular(text):
    """Parse a LaTeX tabular into rows of cell strings plus rule positions."""
    m = re.search(r'\\begin\{tabular\}\{[^}]*\}(.*?)\\end\{tabular\}', text, re.S)
    if not m:
        return [], []
    body = m.group(1)
    rows = []
    rules = []
    for raw in body.split(r'\\'):
        line = raw.strip()
        had_rule = r'\hline' in line
        line = line.replace(r'\hline', '').strip()
        if not line:
            continue
        if had_rule and rows:
            rules.append(len(rows))
        cells = [c.strip() for c in line.split('&')]
        rows.append(cells)
    return rows, rules


def collect_env(lines, i, env):
    """Return (content_lines, next_index) for an environment, excluding begin/end."""
    content = []
    i += 1
    while i < len(lines):
        if lines[i].strip().startswith('\\end{%s}' % env):
            return content, i + 1
        content.append(lines[i])
        i += 1
    raise RuntimeError('unterminated env ' + env)


def parse_table(block):
    text = '\n'.join(block)
    cap = ''
    label = ''
    for ln in block:
        if ln.strip().startswith(r'\caption{'):
            cap = strip_caption(ln)
        lm = re.search(r'\\label\{([^}]*)\}', ln)
        if lm and not label:
            label = lm.group(1)
    inm = re.search(r'\\input\{([^}]*)\}', text)
    if inm:
        tabpath = os.path.join(BASE, inm.group(1))
        if not tabpath.endswith('.tex'):
            tabpath += '.tex'
        tabtext = read_tex(tabpath)
        # remove size commands
        tabtext = re.sub(r'\\(footnotesize|small|scriptsize)', '', tabtext)
        rows, rules = parse_tabular(tabtext)
        after = text.split(inm.group(0), 1)[1]
        after = after.replace(r'\end{table}', '')
        note_lines = []
        for ln in after.splitlines():
            s = ln.strip()
            if not s or s.startswith(r'\vspace') or s.startswith(r'\centering') or s == '}':
                continue
            note_lines.append(s)
        note = ' '.join(note_lines)
        note = re.sub(r'\\(footnotesize|small|scriptsize|tiny|normalsize)', '', note)
        note = note.strip()
        if note.startswith('{'):
            note = note[1:]
        if note.endswith('}'):
            note = note[:-1]
        return {'type': 'table', 'caption': cap, 'label': label, 'rows': rows,
                'rules': rules, 'note': note}
    else:
        rows, rules = parse_tabular(text)
        return {'type': 'table', 'caption': cap, 'label': label, 'rows': rows,
                'rules': rules, 'note': ''}


def parse_article():
    src = read_tex(TEX)
    body = src[src.index(r'\begin{document}') + len(r'\begin{document}'):src.index(r'\end{document}')]
    lines = body.splitlines()
    blocks = []
    para = []
    appendix = False
    i = 0

    def flush():
        nonlocal para
        if para:
            text = ' '.join(s.strip() for s in para if s.strip())
            if text:
                blocks.append({'type': 'para', 'tex': text})
            para = []

    while i < len(lines):
        raw = lines[i]
        s = raw.strip()
        if not s:
            flush()
            i += 1
            continue
        if s.startswith('%'):
            i += 1
            continue

        m = re.match(r'\\section\{(.*?)\}(?:\\label\{([^}]*)\})?', s)
        if m:
            flush()
            blocks.append({'type': 'section', 'title': m.group(1), 'label': m.group(2) or '',
                           'level': 1, 'appendix': appendix})
            i += 1
            continue
        m = re.match(r'\\subsection\{(.*?)\}(?:\\label\{([^}]*)\})?', s)
        if m:
            flush()
            blocks.append({'type': 'section', 'title': m.group(1), 'label': m.group(2) or '',
                           'level': 2, 'appendix': appendix})
            i += 1
            continue
        m = re.match(r'\\subsubsection\{(.*?)\}(?:\\label\{([^}]*)\})?', s)
        if m:
            flush()
            blocks.append({'type': 'section', 'title': m.group(1), 'label': m.group(2) or '',
                           'level': 3, 'appendix': appendix})
            i += 1
            continue

        if s.startswith(r'\begin{abstract}'):
            flush()
            content, i = collect_env(lines, i, 'abstract')
            blocks.append({'type': 'abstract_en', 'tex': ' '.join(x.strip() for x in content if x.strip())})
            continue
        if s.startswith(r'\begin{resumo}'):
            flush()
            content, i = collect_env(lines, i, 'resumo')
            blocks.append({'type': 'abstract_pt', 'tex': ' '.join(x.strip() for x in content if x.strip())})
            continue
        if s.startswith(r'\begin{table}'):
            flush()
            content, i = collect_env(lines, i, 'table')
            blocks.append(parse_table(content))
            continue
        if s.startswith(r'\begin{figure}'):
            flush()
            content, i = collect_env(lines, i, 'figure')
            text = '\n'.join(content)
            cap = ''
            for ln in content:
                if ln.strip().startswith(r'\caption{'):
                    cap = strip_caption(ln)
            lm = re.search(r'\\label\{([^}]*)\}', text)
            blocks.append({'type': 'figure', 'caption': cap, 'label': lm.group(1) if lm else ''})
            continue
        if s.startswith(r'\begin{algorithm}'):
            flush()
            content, i = collect_env(lines, i, 'algorithm')
            cap = ''
            for ln in content:
                if ln.strip().startswith(r'\caption{'):
                    cap = strip_caption(ln)
            lm = re.search(r'\\label\{([^}]*)\}', '\n'.join(content))
            algo_lines = []
            for ln in content:
                t = ln.strip()
                if not t or t.startswith(r'\caption') or t.startswith(r'\label') or t.startswith(r'\SetAlgo'):
                    continue
                algo_lines.append(t)
            blocks.append({'type': 'algorithm', 'caption': cap, 'label': lm.group(1) if lm else '',
                           'lines': algo_lines})
            continue
        if s.startswith(r'\begin{itemize}') or s.startswith(r'\begin{enumerate}'):
            flush()
            ordered = s.startswith(r'\begin{enumerate}')
            env = 'enumerate' if ordered else 'itemize'
            content, i = collect_env(lines, i, env)
            items = []
            cur = []
            for ln in content:
                t = ln.strip()
                if t.startswith(r'\item'):
                    if cur:
                        items.append(' '.join(cur))
                    cur = [t[len(r'\item'):].strip()]
                elif t:
                    cur.append(t)
            if cur:
                items.append(' '.join(cur))
            blocks.append({'type': 'list', 'ordered': ordered, 'items': items})
            continue
        if s.startswith(r'\appendix'):
            flush()
            appendix = True
            i += 1
            continue
        if s.startswith(r'\bibliographystyle') or s.startswith(r'\bibliography'):
            flush()
            if not any(b['type'] == 'refs' for b in blocks):
                blocks.append({'type': 'refs'})
            i += 1
            continue
        if s.startswith(r'\clearpage'):
            flush()
            blocks.append({'type': 'pagebreak'})
            i += 1
            continue
        if s.startswith(r'\maketitle') or s.startswith(r'\sloppy') or s.startswith(r'\setlength'):
            i += 1
            continue

        para.append(s)
        i += 1

    flush()
    return blocks


if __name__ == '__main__':
    blocks = parse_article()
    json.dump(blocks, open(os.path.join(os.path.dirname(__file__), '_blocks.json'), 'w', encoding='utf-8'),
              ensure_ascii=False, indent=1)
    from collections import Counter
    print(Counter(b['type'] for b in blocks))
    for n, b in enumerate(blocks):
        if b['type'] in ('section', 'table', 'figure', 'algorithm', 'list', 'pagebreak', 'refs'):
            print(n, b['type'], b.get('level', ''), b.get('title', b.get('label', ''))[:90])
