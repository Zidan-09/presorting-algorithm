# -*- coding: utf-8 -*-
import os, re, json, zipfile
from docx import Document
from lxml import etree

BASE = r'C:\Users\acemilton\Documents\presorting-algorithm'
QC = os.path.join(BASE, 'artigo', 'qualyacademics')
DOCX = os.path.join(QC, 'artigo_qualyacademics.docx')
W = 'http://schemas.openxmlformats.org/wordprocessingml/2006/main'
M = 'http://schemas.openxmlformats.org/officeDocument/2006/math'

d = Document(DOCX)
texts = []
for p in d.paragraphs:
    texts.append(p.text)
for t in d.tables:
    for row in t.rows:
        for cell in row.cells:
            for p in cell.paragraphs:
                texts.append(p.text)
full = '\n'.join(texts)

print('paragraphs', len(d.paragraphs), 'tables', len(d.tables),
      'inline_shapes', len(d.inline_shapes))
print('sections', len(d.sections))

z = zipfile.ZipFile(DOCX)
docxml = z.read('word/document.xml').decode('utf-8')
print('oMath count', docxml.count('<m:oMath'))
print('drawing count', docxml.count('<w:drawing'))
print('media', [n for n in z.namelist() if n.startswith('word/media/')])

leftovers = []
for pat in [r'\\[A-Za-z]+', r'\$', r'\\%', r'\\&', r'\\_', r'\\ref', r'\\cite']:
    hits = re.findall(pat, full)
    if hits:
        leftovers.append((pat, len(hits), hits[:5]))
print('LEFTOVERS:', leftovers)

checks = [
    'Análise do Impacto de um Pré-Processamento Simétrico',
    'Analysis of the Impact',
    'RESUMO', 'ABSTRACT', 'Palavras-chave:', 'Keywords:',
    '1. INTRODUÇÃO', '2. REFERENCIAL TEÓRICO',
    '2.1 Inversões, Ordenação Adaptativa e Algoritmos Avaliados',
    '3. MÉTODOS E TÉCNICAS DA PESQUISA',
    '3.2.1 Algoritmos Quadráticos',
    '4. RESULTADOS E DISCUSSÃO', '5. CONSIDERAÇÕES FINAIS',
    '6. USO DE INTELIGÊNCIA ARTIFICIAL', '7. REPOSITÓRIO DA PESQUISA',
    'A. TABELAS COMPLETAS DE TEMPOS DE EXECUÇÃO', 'REFERÊNCIAS',
    'Tabela 1.', 'Tabela 12.', 'Figura 1.', 'Figura 4.', 'Algoritmo 1.',
    'CORMEN, Thomas H.', 'ZIVIANI, Nivio.', '10.1145/146370.146381',
    'Quadratic algorithms are useful', 'Algoritmos quadráticos são úteis',
]
for c in checks:
    print('OK ' if c in full else 'MISSING ', repr(c))

# sectPr sanity
sec = d.sections[0]
print('page', sec.page_width.twips, sec.page_height.twips,
      'margins', sec.top_margin.twips, sec.right_margin.twips,
      sec.bottom_margin.twips, sec.left_margin.twips)
print('header', repr(' | '.join(p.text for p in sec.header.paragraphs)))
print('first_header', repr(' | '.join(p.text for p in sec.first_page_header.paragraphs)))
print('first_footer', repr(' | '.join(p.text for p in sec.first_page_footer.paragraphs)))
print('footer', repr(' | '.join(p.text for p in sec.footer.paragraphs)))
