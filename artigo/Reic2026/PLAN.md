# PLAN — Adaptação do artigo para a REIC (artigo/Reic2026/)

## 2.1 Requisitos da REIC (docs/REIC.md — fonte de verdade)

- O template oficial da REIC no projeto prevalece sobre qualquer regra externa e sobre este próprio guia em caso de conflito.
- Não recriar classe/template; partir do template oficial; não modificar o template original para facilitar.
- Preservar título, autores, afiliações, resumo, abstract, palavras-chave, seções, texto, equações, tabelas, figuras, resultados, conclusões, referências.
- Estrutura: mapear cada seção original ao equivalente no template; usar só comandos estruturais do template (`\section`, `\subsection`, etc.).
- Título/autoria: migrar para o mecanismo do template (título, título em inglês se exigido, nomes completos, ordem, afiliações, e-mails, ORCID se exigido, instituição, cidade/estado/país se exigido). Não alterar ordem; não anonimizar por inferência (o guia não exige double-blind).
- Resumo/abstract/palavras-chave/keywords: usar exatamente a estrutura do template (`abstract-pt`, `abstract-en`, `pchaves`, `keywords`); respeitar separadores e limites do template sem reescrever o conteúdo.
- Figuras/tabelas/equações/código: migrar todas; corrigir caminhos; preservar legendas, dados e referências no texto; numeração automática; adequar largura à coluna.
- Referências/citações: migrar para o sistema do template (`apalike-sol` + `natbib`/`\citep`); checar `??`, chaves inexistentes/duplicadas, DOI.
- Não adicionar pacotes arbitrariamente (só após verificar que o template não cobre).
- Compilar pelo método do template; PDF sem erros, sem refs indefinidas, sem figuras ausentes; revisão visual página a página.
- Limite de páginas: determinado pelo template/diretrizes vigentes no projeto — **nenhum limite explícito encontrado**; apenas verificar o nº final.
- Elementos editoriais/licença/copyright/declarações: preencher os obrigatórios do template; não inventar valores do editor (manter placeholders de `dates`/`doi`).
- Declaração de IA: só se o template exigir com mecanismo próprio — **não há exigência explícita**; usar `furtherinformation` apenas para fato verificável.
- Relatório final obrigatório ao término.

## 2.2 Requisitos do modelo (artigo/modelos/REIC modelo/)

- Arquivo principal: `main-pt.tex` (artigo em português); classe `sbc2025.cls` (`\documentclass[portuguese]{sbc2025}`), 2 colunas, A4.
- Classe já carrega: xcolor, graphicx, amsmath/amssymb, hyperref, babel, fontspec (lualatex/xelatex) ou fontenc+tgtermes+FiraSans (pdflatex), natbib, geometry, caption, fancyhdr. **Não recarregar** esses pacotes.
- Compilação: **lualatex ou xelatex** (pdflatex emite erro por causa das fontes). Disponível localmente: lualatex (MiKTeX) — usar `lualatex + bibtex + lualatex + lualatex`.
- Pacotes extras do template: `aas_macros`, `footmisc`, `tabularray`, `afterpage`, `url`, `pifont`, `\setcitestyle{square}`.
- Metadados: `\jid{REIC}`, `\jtitle{...}`, `\issn{3085-8461}`, `\doi{...}`, `\copyrightstatement{...}`, `\jyear{...}`, `\category{...}`, `\title[short]{long}`, `\engtitle{...}`.
- Autores: `\author[short]{\affil{\textbf{Nome}~\orcidlink{...}... [\textit{afiliação} | email]}}` — ORCID indicado como obrigatório no exemplo.
- Frontmatter: `frontmatter` + `\maketitle` + `mail` + `abstract-pt` + `abstract-en` + `pchaves` + `keywords` + `dates` + `declarations` (acknowledgements/funding opcionais; contributions/interests/materials obrigatórias; furtherinformation desejável).
- Observação: comentário no template diz que o resumo-pt deve ter "até 150 caracteres ou 10 linhas" (texto herdado de anais SBC OpenLib; inconsistente com artigo de revista) — ver 2.7.
- Citações: `natbib` estilo autor-data → `\citep{...}`; bibliografia `\bibliographystyle{apalike-sol}` + `\bibliography{refs}`.
- Figuras: `\includegraphics[width=\columnwidth]` (1 col.) ou `figure*` + `width=30pc` (2 col.); tabelas: `table`/`table*`, legenda acima via `\caption` antes do `tabular`.
- Equações numeradas com `\label`/`\ref`.

## 2.3 Estrutura atual (artigo/main.tex — SBC, 1 coluna, 12pt)

1. Título PT + 2 autores + afiliação UESPI + 2 e-mails
2. `abstract` (EN) + `resumo` (PT) — sem keywords no original
3. 1 Introdução (5 parágrafos + estrutura do artigo)
4. 2 Fundamentação Teórica (2.1 Inversões e Ordenação Adaptativa; 2.2 Algoritmos Avaliados; 2.3 Pré-Processamento Simétrico; 2.4 Trabalhos Relacionados)
5. 3 Métodos e Técnicas (classificação da pesquisa; 3.1 Configuração + Tab. ambiente + Tab. datasets; 3.2 Especificação e Implementação — quadráticos/quasilineares/baselines; 3.3 Protocolo Experimental e Cálculo de Inversões; 3.4 Formalização + Algoritmo 1 (algorithm2e) + regra ΔI≳5n; 3.5 Ameaças à Validade)
6. 4 Resultados e Discussão (4.1 Redução de Inversões — Fig. barras log + Tab. inversões; 4.2 Impacto no Tempo — Tab. faixa clássicas, Tab. baselines, Tab. faixa novas, Fig. custo linear + Tab. precusto + Tab. cpre; 4.3 Tempo Total e Análise Individual — Fig. groupplot 2×3 + Tab. 1M + Fig. ganho×n)
7. 5 Considerações Finais (4 parágrafos + 3 trabalhos futuros)
8. Apêndice: Tab. apA (clássicas 30 linhas), apB (novas ~32 linhas), apC (1M ~30 linhas)
9. Referências: 13 chaves citadas (`\cite` numérico SBC) de um .bib com ~30 entradas
10. Recursos: 4 figs TikZ/pgfplots lendo `resultados/*.csv`; 7 `\input{resultados/tab*_pt_tabular}`; pacotes extras: tikz, pgfplots, algorithm2e, microtype

## 2.4 Estrutura final (artigo/Reic2026/main.tex)

- Preâmbulo copiado de `main-pt.tex` + apenas: tikz, pgfplots(+groupplots), algorithm2e, microtype, booktabs-compatível (verificar). Classe `sbc2025`, opção `portuguese`, compilação **lualatex**.
- `frontmatter`: título PT original; `engtitle` = tradução editorial do título; autores/afiliação/e-mails originais (sem ORCID — ver 2.7); `mail` = endereço UESPI do original; `abstract-pt`/`abstract-en` = resumo/abstract originais verbatim; `pchaves`/`keywords` = 4 termos derivados do conteúdo (original não tem); `dates` placeholder do template; `declarations` obrigatórias preenchidas com fatos do artigo (ver 2.7).
- Corpo: seções 1–5 + Apêndice idênticos em ordem e conteúdo; `\cite`→`\citep`; figs em `figure*` com larguras de coluna; tabs faixa em `table`, tabs cheias do apêndice em `table*` `\footnotesize`; Algoritmo 1 mantido (algorithm2e); equações $C_{pre}+C_{sort}<C_{original}$, $\Delta I\gtrsim 5n$ mantidas inline como no original.
- `referencias.bib` = cópia integral do original; `\bibliographystyle{apalike-sol}`.

## 2.5 Mapeamento

| Conteúdo original | Seção REIC | Ação |
|---|---|---|
| Título PT | `\title` + `\engtitle` (tradução editorial) | adaptar |
| Autores/afiliação/e-mails UESPI | `\author`/`\affil`/`mail` | adaptar (ORCID pendente) |
| `abstract` EN | `abstract-en` | manter |
| `resumo` PT | `abstract-pt` | manter |
| (ausente) keywords | `pchaves`/`keywords` | adaptar (termos do conteúdo) |
| Introdução | 1 Introdução | manter |
| Fundamentação (2.1–2.4) | 2 Fundamentação Teórica + subseções | manter |
| Métodos (classificação + 3.1–3.5) + Tabs ambiente/datasets | 3 Métodos e Técnicas + subseções | manter |
| Algoritmo 1 (algorithm2e) + equações | mesma posição | manter (+ pacote) |
| Resultados (4.1–4.3) + 4 figs TikZ | 4 Resultados e Discussão | manter; adaptar larguras p/ 2 col. |
| Tabs inversões/faixas/precusto/cpre/baselines | `table` 1 col. | manter dados; adaptar fonte |
| Tabs apA/apB/apC | Apêndice `table*` | manter dados; adaptar fonte |
| Considerações Finais | 5 Considerações Finais | manter |
| 13 citações `\cite` | `\citep` natbib | adaptar comando |
| `referencias.bib` (30 entradas) | `referencias.bib` + `apalike-sol` | manter (cópia integral) |
| `dates`/doi/editoriais | placeholders do template | manter (editor preenche) |
| contributions/interests/materials | `declarations` | adaptar (fatos do artigo; lacunas em 2.7) |

## 2.6 Alterações necessárias

1. Novo preâmbulo `sbc2025` (sem sbc-template, inputenc, fontenc, lmodern, babel manual).
2. `\cite{}` → `\citep{}` (13 chaves, sem troca de chaves).
3. Bloco de autoria/frontmatter refeito no mecanismo do template.
4. Keywords PT/EN criadas a partir do conteúdo (original não possui).
5. `engtitle` traduzido editorialmente (original não possui).
6. Figuras: `figure`→`figure*` onde largo; `width=0.95\textwidth`→`\columnwidth`/`\textwidth`-relativo; groupplot 2×3 reescalado.
7. Tabelas cheias → `table*` + `\footnotesize`; sem tocar em valores.
8. Adicionar algorithm2e/pgfplots ao preâmbulo (ausentes no template).
9. Bibliografia → `apalike-sol`.
10. Copiar `sbc2025.cls`, `apalike-sol.bst`, `resultados/*.csv`, `resultados/tab*.tex` para `Reic2026/`.

## 2.7 Possíveis problemas (revisão manual)

- **ORCID**: template marca ORCID como obrigatório; artigo original não informa ORCID dos autores. NÃO inventado — campo omitido; autores devem fornecer.
- **DOI/`dates`/`jtitle`**: pertencem ao editor; mantidos placeholders `202X/XXXXXX/DD Month YYYY`.
- **Limite do resumo** (150 caracteres ou 10 linhas, comentário herdado no template): incompatível com resumo científico real; conteúdo original preservado integralmente e ponto registrado para o editor.
- **Contribuições (CRediT)**: original não declara; texto genérico incluído e sinalizado como REVISÃO MANUAL.
- **Conflito de interesses**: original não declara; incluída declaração padrão de inexistência — REVISÃO MANUAL.
- **Financiamento**: original não informa; seção omitida (opcional).
- **Largura das tabelas do Apêndice** em 2 colunas: verificar visualmente no PDF; possível necessidade de `\tiny` ou quebra — sem alterar dados.
- **Duas notas de rodapé** (`\footnote`) no corpo: classe usa `footmisc[bottom]`; checar renderização.
