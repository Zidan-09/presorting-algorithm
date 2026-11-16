import re
src = open(r'..\..\main.tex', encoding='utf-8').read()
body = src[src.index(r'\begin{document}'):]
for m in re.finditer(r'\\cite\{([^}]*)\}', body):
    start = max(0, m.start() - 90)
    ctx = body[start:m.end() + 30].replace('\n', ' ')
    print(ctx)
    print('---')
