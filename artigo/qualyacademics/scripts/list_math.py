import re, sys

s = open(sys.argv[1], encoding='utf-8').read()
body = s[s.index(r'\begin{document}'):]
maths = re.findall(r'(?<!\\)\$(.+?)(?<!\\)\$', body, flags=re.S)
print('MATH COUNT', len(maths))
u = sorted(set(m.strip() for m in maths))
print('UNIQUE', len(u))
for m in u:
    print(repr(m))
print('==== COMMANDS ====')
cmds = {}
for m in re.finditer(r'\\([A-Za-z@]+)', body):
    cmds[m.group(1)] = cmds.get(m.group(1), 0) + 1
for k, v in sorted(cmds.items(), key=lambda x: -x[1]):
    print(k, v)
