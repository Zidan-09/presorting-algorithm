# Dados reais — `ArrayType::Real`

## v1 (congelado, 2026-09-30)
- Arquivo: `real_bytes_v1.bin` — bytes crus concatenados (1 byte → 1 `i32`, faixa 0–255).
- Fonte: 40 arquivos de texto do próprio repo (`.tex/.md/.rs/.toml/.bib/.cls/.bst/.py`),
  na ordem do `git ls-files`, excluindo saídas de medição (`artigo/resultados/`)
  e derivados (`artigo/bench2026/`).
- Commit de origem: `c3b0bbe` (lista exata em `real_bytes_v1.files.txt`).
- Tamanho nativo N = 477.104 bytes; SHA-256:
  `0bd1921eb26fcbc507ab524a41a49aba3db3274093326764ffdf50b41479067b`
- Uso no gerador: prefixo `[0..n)` dos bytes como `i32`; células com
  `n > N` são **excluídas** da matriz (sem tiling — documentado).
  Com N = 477.104, `Real` participa de n = 16…100.000 (não de 10⁶).
- Geração: `python3 presorting-algorithm/dados_reais/gerar_asset_v1.py`
  (requer árvore no commit de origem para reproduzir o hash).

## Política
Nunca regenerar este arquivo no lugar. Nova fonte ou novo commit de
origem → novo asset versionado (`real_bytes_v2.bin` + constante no código).
