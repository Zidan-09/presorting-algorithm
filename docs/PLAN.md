# Plano de Revisão do Artigo (`artigo/main.tex`)

Revisão para garantir conformidade com `docs/guia_escrita_artigo_tcc.md`, incluindo
compilação e verificação visual do PDF (`artigo/main.pdf`).

## Decisões já tomadas

- **Hwang et al. (2000) será substituído** por Estivill-Castro & Wood (1992), cujo
  *abstract* está em `artigo/referencias.bib` e permite descrever com segurança
  **objetivo, métodos e resultados**.
- **Seções 6 e 7 permanecem numeradas** ("Uso de Inteligência Artificial" e
  "Repositório da Pesquisa"). A frase da introdução ("seis seções, além desta
  introdução") está internamente correta e será apenas reconferida.
- **MiKTeX será instalado via winget** para o ciclo de compilação.

## Diagnóstico inicial

Verificado em `artigo/main.tex`, `artigo/main.log` e no PDF atual (26 páginas).

| Item do guia | Situação | Evidência |
|---|---|---|
| Resumo/Abstract: parágrafo único, **≤10 linhas**, 5 orações na ordem contexto → problema → objetivo → metodologia → resultados | ❌ | **15 linhas** no Abstract e **17** no Resumo; **6 orações** em cada; falta oração explícita de problema |
| Introdução ≤3 páginas | ✅ | ocupa p. 2–3 (~1,6 página) |
| Último parágrafo da introdução apresenta a estrutura | ✅ | "seis seções, além desta introdução" corresponde às Seções 2–7 |
| 2.1 e 2.2 baseadas nos termos do título | ⚠️ | 2.1 agrupa "Inversões, Ordenação Adaptativa e Algoritmos"; convém renomear |
| 2.3 Trabalhos Relacionados: objetivo + métodos + resultados para **cada** trabalho | ❌ | Mannila atende; **Hwang et al. não** (o texto admite que o conteúdo não foi verificado); há só 2 trabalhos em 2.3 |
| Métodos: **mínimo 4 parágrafos de classificação** (tipo + fundamentação + porquê) | ❌ | há **um único parágrafo** (linha 112) juntando natureza, abordagem, objetivos e procedimentos |
| Seções técnicas (Estrutura A ou B) | ✅ | 3.1 Configuração, 3.2 Especificação, 3.3 Protocolo, 3.4 Formalização, 3.5 Ameaças |
| Resultados discutidos **com a literatura**, não só descritos | ❌/⚠️ | toda a Seção 4 tem **uma única citação** (Estivill-Castro, no Selection) |
| Considerações Finais: 5 itens obrigatórios | ⚠️ | contexto, resultados, limitações e trabalhos futuros presentes; **objetivo geral não é reafirmado explicitamente** |
| Formatação | ❌ | *Overfull* de **388 pt** na Tabela 2 (linhas 147–163); *overfull* de 24,7 pt no `\maketitle`; *underfull* no parágrafo do protocolo |
| Citações/referências | ✅ | nenhuma referência ou citação indefinida no log |

Observação: o PDF tem 26 páginas, infladas pelo Apêndice. O guia não fixa limite
total, mas convém confirmar o limite da revista/alvo.

## Limitações do ambiente

1. **Não há LaTeX instalado** nesta máquina (o `main.log` veio de outra máquina,
   `C:\Users\zidan`). Será instalado o MiKTeX (`winget install MiKTeX.MiKTeX`).
2. **O PDF não é lido diretamente pelo agente.** A verificação visual será feita
   renderizando cada página em PNG com PyMuPDF (já instalado) e inspecionando as
   imagens; a extração de texto será usada para métricas (linhas do resumo,
   páginas da introdução, *overfull boxes*).

## Plano de execução

### Fase 0 — Toolchain e baseline

1. `winget install MiKTeX.MiKTeX`.
2. Compilar em `artigo/`: `pdflatex → bibtex → pdflatex → pdflatex`.
3. Confirmar que o PDF reproduz os avisos atuais (baseline): *overfull* de 388 pt
   na Tabela 2, 24,7 pt no título, *underfull* no protocolo.
4. Guardar o `git diff` para revisão incremental.

### Fase 1 — Resumo e Abstract (guia §3)

- Reescrever os dois em **5 orações exatas**, na ordem contexto → problema →
  objetivo → metodologia → resultados, parágrafo único, sem citações.
- Reduzir de 161/176 para ~105–115 palavras (hoje: 15 e 17 linhas; limite: 10).
- Preservar os números essenciais: 40–62% de redução, 14–60% de ganho,
  ×386–2.280, e "nenhum híbrido supera o `sort_unstable` no Aleatório".
- Medir as linhas no PDF via PyMuPDF e iterar até ≤10.

### Fase 2 — Introdução (guia §4)

- Confirmar ≤3 páginas (hoje p. 2–3).
- Manter a sequência e o último parágrafo; apenas conferir a coerência com a
  numeração 6/7 escolhida.
- Reforçar, se necessário, a frase de problema no 2º parágrafo, sem reescrever a
  estrutura.

### Fase 3 — Referencial Teórico (guia §5)

- Renomear 2.1 para refletir o título:
  **"Inversões e Eficiência de Algoritmos de Ordenação"**.
- Reescrever 2.3 com **dois trabalhos, cada um com a tríade completa**:
  - **Mannila (1985):** objetivo (medidas de pré-ordenação), métodos
    (formalização/análise), resultados (Insertion Sort ótimo para três medidas
    naturais).
  - **Estivill-Castro & Wood (1992):** objetivo (survey de ordenação adaptativa),
    métodos (revisão e classificação por medidas de desordem, análise de
    complexidade), resultados (relações entre medidas, provas de adaptatividade
    máxima, algoritmos em uso adaptativos e mais rápidos em quase ordenadas).
- Remover **todas** as menções a Hwang (linhas 79, 106, 108, 270 e 621),
  inclusive os *disclaimers* e a atribuição no termo de primeira ordem; a
  derivação passa a ser apenas "própria deste trabalho".
- Ajustar a frase de contribuição original: contraste com Mannila
  (medidas/otimalidade) e com o survey (caracterização de algoritmos
  adaptativos), não com "análise de caso médio de presorting".
- Remover `hwang_presorting_2000` do `.bib` (ou deixá-lo sem citação, já que o
  BibTeX não o incluiria).

### Fase 4 — Métodos (guia §6)

- Dividir o parágrafo único da linha 112 em **4 parágrafos de classificação**,
  cada um com tipo + fundamentação + justificativa:
  1. Natureza: aplicada (Gil).
  2. Abordagem: quantitativa (Sampieri).
  3. Objetivos: exploratória e descritiva (Gil).
  4. Procedimentos: experimental (Gil).
- Manter 3.1–3.5 como estão.

### Fase 5 — Resultados (guia §7)

- Acrescentar discussão explícita com a literatura, usando apenas o que é
  verificável:
  - Insertion ~0,21 ns/inversão e O(n+I) → convergente com Mannila.
  - Invariância do Selection → convergente com a classificação de não adaptativo
    de Estivill-Castro.
  - Adaptatividade dos ordenadores padrão a runs ordenadas/revertidas → explica
    o overhead nas demais topologias.
- Amarrar explicitamente os objetivos da introdução aos resultados
  (rastreabilidade, guia §10.2).
- Conferir que todas as figuras/tabelas são discutidas, não apenas citadas.

### Fase 6 — Considerações Finais (guia §8)

- Abrir com reafirmação explícita do objetivo geral ("Este trabalho teve como
  objetivo propor, implementar e avaliar…").
- Garantir os 5 itens: contexto, objetivo, resultados/contribuições, limitações,
  trabalhos futuros.
- Não introduzir resultados novos.

### Fase 7 — Layout

- Tabela 2: trocar `ll` por `p{0.22\linewidth}p{0.68\linewidth}` + `\small`
  (corrige o *overfull* de 388 pt).
- Título: quebra manual em ponto natural ou ajuste de fonte (corrige 24,7 pt).
- Reavaliar *underfull* do parágrafo do protocolo.
- Conferir posicionamento de figuras/tabelas e ausência de "??".

### Fase 8 — Verificação final

- Recompilar 4×: 0 erros, 0 referências indefinidas, 0 *overfull* >5 pt.
- Script de checagem: Abstract/Resumo ≤10 linhas; introdução ≤3 páginas; todas
  as `\cite` no `.bib`; todos os `\ref` resolvidos.
- Renderizar as páginas em PNG com PyMuPDF e inspecionar visualmente: capa,
  resumo, Tabela 2, tabelas de resultados e apêndice.
- Entregar checklist final item do guia → status → correção.

## Critérios de aceite

- Resumo/Abstract: 10 linhas ou menos, 5 orações na ordem exigida, sem citações.
- Introdução ≤3 páginas.
- 2.3 com tríade completa para cada trabalho citado (Mannila e
  Estivill-Castro).
- Métodos com 4 parágrafos de classificação.
- Seção 4 com comparações explícitas à literatura.
- Considerações Finais com os 5 itens e objetivo reafirmado.
- PDF sem *overfull* relevante e sem referências quebradas.

## Observações

- A "visualização" do PDF será feita por renderização das páginas em PNG
  (PyMuPDF) e leitura das imagens, complementada por extração de texto para
  métricas.
- O Apêndice mantém o PDF em 26 páginas; o guia não fixa limite total, mas
  convém confirmar o limite da revista/alvo.
