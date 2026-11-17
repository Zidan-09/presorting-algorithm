# Relatório de Revisão — artigo_qualyacademics.docx

**Artigo:** NASCIMENTO, Samuel da Penha; ROCHA, Francisco das Chagas. *Análise do Impacto de um Pré-Processamento Simétrico O(n) na Redução de Inversões e Eficiência de Algoritmos de Ordenação.* Revista QUALYACADEMICS.

**Data da revisão:** 17/11/2026

**Arquivos entregues nesta pasta**

| Arquivo | Descrição |
|---|---|
| `artigo_revisado.docx` | Versão revisada (editada in place a partir do original) |
| `artigo_revisado.pdf` | Conversão fiel do DOCX (LibreOffice 24.x headless) |
| `relatorio_revisao.md` | Este relatório |

O arquivo original `../artigo_qualyacademics.docx` **não foi modificado**.

---

## 1. Resultado em números

| Métrica | Original | Revisado | Variação |
|---|---|---|---|
| Páginas (PDF) | 32 | **24** | −8 (−25%) |
| Palavras (parágrafos do corpo) | 8.046 | **5.460** | −2.586 (−32,1%) |
| Parágrafos | 155 | 132 | −23 |
| Tabelas | 13 | 13 | 0 |
| Figuras | 4 | 4 | 0 |
| Referências | 9 | 9 | 0 |
| Equações OMML (`m:oMath`) | 633 | 628 | −5 (ver §4.3) |

**Aderência à meta:** o limite rígido de **25 páginas foi respeitado** (24 páginas). A faixa ideal de 20–23 páginas **não foi atingida por uma página**. O motivo é estrutural e está quantificado na §5: quatro figuras em largura total (duas delas com 411 pt e 371 pt de altura), quebras de página forçadas (apêndice e referências) e quatro páginas de tabelas por célula no apêndice. Faltar para 23 exigiria remover ~400 palavras adicionais de resultados (≈8% do texto restante), o que contrariaria a prioridade estabelecida **integridade > concisão**.

---

## 2. O que foi condensado (palavras por seção)

| Seção | Original | Revisado | Redução |
|---|---:|---:|---:|
| RESUMO | 177 | 111 | −37% |
| ABSTRACT | 164 | 101 | −38% |
| 1. INTRODUÇÃO | 542 | 107 | −80%¹ |
| 2. REFERENCIAL TEÓRICO (texto de abertura) | 42 | 0 | −100%¹ |
| 2.1 Inversões, Ordenação Adaptativa e Algoritmos | 226 | 98 | −57% |
| 2.2 Pré-Processamento Simétrico | 148 | 89 | −40% |
| 2.3 Trabalhos Relacionados | 310 | 121 | −61% |
| 3. MÉTODOS E TÉCNICAS (abertura) | 104 | 57 | −45% |
| 3.1 Configuração | 446 | 282 | −37% |
| 3.2 Especificação e Implementação | 520 | 336 | −35% |
| 3.3 Protocolo Experimental e Raciocínio de Cálculo | 934 | 563 | −40% |
| 3.4 Formalização do Pré-Processamento | 615 | 482 | −22% |
| 3.5 Ameaças à Validade | 178 | 154 | −13% |
| 4. RESULTADOS E DISCUSSÃO (abertura) | 50 | 0 | −100%¹ |
| 4.1 Redução de Inversões e Viés Estrutural | 400 | 307 | −23% |
| 4.2 Impacto no Tempo e Custo do Pré | 689 | 625 | −9% |
| 4.3 Tempo Total e Análise Individual | 1.044 | 821 | −21% |
| 5. CONSIDERAÇÕES FINAIS | 800 | 597 | −25% |
| 6. USO DE INTELIGÊNCIA ARTIFICIAL | 96 | 78 | −19% |
| 7. REPOSITÓRIO DA PESQUISA | 75 | 48 | −36% |
| A. TABELAS COMPLETAS | 143 | 140 | −2% |
| REFERÊNCIAS | 173 | 173 | 0% |

¹ Redução decorrente da remoção de parágrafos de enquadramento/roteiro sem conteúdo científico (ver §4.1).

**Critérios de condensação aplicados**

- Fusão de frases repetitivas e remoção de conectivos redundantes, mantendo sujeito, verbo e complemento de cada afirmação.
- Remoção de duplicações internas (a mesma informação aparecia no resumo, no método e na conclusão).
- Encurtamento de explicações metodológicas já cobertas por tabela, figura, caption ou arquivo de código citado.
- **Nenhum valor numérico, faixa, tempo, percentual, configuração, seed, tamanho de amostra ou referência foi alterado.**
- Os avisos de transparência sobre Hwang, Yang e Yeh (2000) permanecem íntegros e em dobro (§2.3 e §3.4), bem como a nota de verificação via Crossref/OpenAlex.

---

## 3. O que NÃO foi alterado

**Formatação (verificada no DOCX):**

- Margens, tamanho e tipo de página, seções (`sectPr`) e quebras de seção.
- Fonte Times New Roman, corpo 12 pt, espaçamento 1,5, recuos, alinhamento e estilos de parágrafo.
- Cabeçalhos, rodapés e numeração de páginas.
- Propriedades de parágrafo (`pPr`) dos parágrafos editados foram preservadas; apenas o conteúdo textual (`runs`) foi substituído.
- Fórmulas OMML mantidas como objetos nativos (não convertidas em imagem/texto); os trechos de código seguem em Courier New 10 pt (`sz=22`).

**Conteúdo científico:**

- 13 tabelas e 13 legendas, incluindo as três tabelas por célula do Apêndice A (Tabelas 10, 11 e 12).
- 4 figuras e 4 legendas (Figuras 1–4), preservadas nos parágrafos que as contêm.
- Lista de referências completa e inalterada (9 entradas).
- Todos os valores experimentais: tempos em µs/ms, speedups, percentuais de redução de inversões, ICs, breakevens, contagens de inversões e parâmetros (FP_R=50, FP_BOOT=10.000, seeds 42–46, 50 amostras, warm-up/medição).
- Algoritmo 1 (descrição em passos e complexidades).
- Ressalvas de validade: ausência de contadores de hardware, Quicksort sem validação contra referência externa (pdqsort), Zigue-zague sem caracterização teórica, hipótese interpretativa do excedente sobre o termo de primeira ordem.

---

## 4. Remoções (23 parágrafos)

### 4.1 Parágrafos de enquadramento/roteiro (6)

| Original | Conteúdo | Justificativa |
|---|---|---|
| 18 | Roteiro/roadmap do artigo | Metatexto: repete a ordem das seções, sem informação científica |
| 19 | Delimitação/escopo declarado | Repetido em §1 (objetivo) e §3.5 (ameaças) |
| 20 | Frase de transição/roteiro | Metatexto |
| 22 | Abertura do §2 ("este capítulo...") | Metatexto; §2 inicia diretamente em 2.1 |
| 32 | Posicionamento/diferenciação genérica | Repetido em §2.3 (contribuição = heurística + termo próprio) |
| 80 | Abertura do §4 ("a análise...") | Metatexto; §4 inicia em 4.1 |

O parágrafo 32 continha **uma** equação inline `O(n)`, perdida com a frase removida.

### 4.2 Parágrafos de conteúdo redundante (5)

| Original | Conteúdo | Justificativa |
|---|---|---|
| 16 | Definição de inversão na Introdução | Definição formal idêntica permanece em §2.1 (parágrafo 24), com as mesmas citações (Knuth; Sedgewick; Wayne; Mannila; Estivill-Castro; Wood) |
| 25 | Nomes dos três quadráticos avaliados | Coberto por §3.2.1 (bullets) e pela Tabela 4 |
| 36 | Ambiente controlado (plano de energia, `opt-level=3`, 50 amostras, 5 pools) | Duplicado em §3.2 (bullets), §3.3 (amostragem) e §3.5 (ambiente) |
| 46 | "Implementado em Rust 1.96.0 sobre `&mut [i32]`" | Toolchain 1.96.0 e i32 permanecem em §3.2 e §3.5 |
| 117 | Frase-ponte "A Figura 3 sintetiza... Tabela 9 complementa para n=10⁶" | Ponte redundante: a Figura 3 tem legenda própria e a Tabela 9 é citada em §4.3 |

### 4.3 Equações OMML perdidas (−5)

| Expressão | Origem | Situação |
|---|---|---|
| `O(n)` | Parágrafo 32 (removido) | Frase removida; `O(n)` ocorre em outros 19 lugares |
| `(i,j)` | Parágrafo 16 (removido) | Duplicata exata: permanece em §2.1 e três vezes em §3.4 |
| `i < j` | Parágrafo 16 (removido) | Duplicata exata: permanece em §2.1 |
| `A[i] > A[j]` | Parágrafo 16 (removido) | Duplicata exata: permanece em §2.1 e no Algoritmo 1 |
| `n = 10⁶` | Parágrafo 117 (removido) | Permanece em outros 14 lugares do texto |

Nenhuma equação única foi eliminada: as 5 perdas são 1 expressão cuja frase foi cortada e 4 duplicatas exatas de expressões mantidas.

### 4.4 Espaçadores vazios (12)

Parágrafos **sem texto, sem imagem, sem equação e sem `sectPr`**: 38, 43, 86, 94, 96, 99, 105, 107, 121, 140, 142 (e 144, artefato de página em branco).

Verificação automática confirmou que nenhum deles continha quebra de seção, `pageBreakBefore`, desenho/imagem ou equação — eram apenas espaçamento vertical. **Não** foram removidos os quatro parágrafos que contêm as figuras (83, 102, 118, 124).

---

## 5. Por que 24 páginas (e não 23)

Análise da densidade e das posições no PDF final:

| Página | Palavras | Observação |
|---|---:|---|
| 17 | ~130 | Fim da §4.3 com Tabela 9 + legenda; a Figura 4 (371 pt) não cabe nos 290 pt livres → quebra |
| 20 | — | Início forçado do Apêndice A (`pageBreakBefore`) |
| 23 | — | Última página do Apêndice A |
| 24 | 197 | REFERÊNCIAS (início forçado) |

Espaço não recuperável por edição de texto:

1. **Quatro figuras em largura total** — as Figuras 3 e 4 ocupam 411 pt e 371 pt; blocos inline não são fragmentáveis.
2. **Quebras de página obrigatórias** — Apêndice A e REFERÊNCIAS iniciam em página nova (padrão editorial, mantido).
3. **Apêndice A** — três tabelas por célula (Tabelas 10–12) ocupam 4 páginas; seus valores são exigidos pela integridade.
4. **Página 17** — desperdício de 290 pt causado pela altura da Figura 4; nenhum texto adicional cabe ali sem mover a figura de posição (o que alteraria o leiaute).

Remover mais uma página exigiria cortar ~400 palavras de resultados (§4) ou das considerações finais (§5) — cerca de 8% do texto restante, com perda de desfechos ou de ressalvas. Como a prioridade definida foi **integridade > concisão**, a revisão parou em 24 páginas, dentro do limite máximo de 25.

---

## 6. Verificações executadas

| Verificação | Método | Resultado |
|---|---|---|
| Tabelas preservadas | contagem de `w:tbl` no DOCX | 13 = 13 ✔ |
| Figuras preservadas | contagem de imagens (`w:drawing` + mídia) | 4 = 4 ✔ |
| Referências | contagem de entradas na lista | 9 = 9 ✔ |
| Equações | multiconjunto de assinaturas de todos os `m:oMath` (original vs. revisado) | 5 diferenças, todas rastreadas (§4.3) ✔ |
| Ressalva Hwang | busca textual | 2 ocorrências (§2.3 e §3.4) ✔ |
| Nota Crossref/OpenAlex | busca textual | 1 ocorrência ✔ |
| Seções e títulos | busca por todos os títulos | 1. INTRODUÇÃO, 2., 2.1–2.3, 3., 3.1–3.5, 4., 4.1–4.3, 5., 6., 7., A., REFERÊNCIAS ✔ |
| Títulos órfãos | títulos seguidos de texto | nenhum (títulos-pai seguidos de subseção, como no original) ✔ |
| Saída DOCX | reabertura e reconversão | sem erros; PDF com 24 páginas ✔ |

---

## 7. Notas de reprodução

- Pipeline: cópia do original → substituição dos parágrafos-alvo (placeholders `[[Mn]]` para as equações, `{code}...{/code}` para trechos monoespaçados) → gravação do DOCX → conversão para PDF com LibreOffice (`soffice.com --headless`).
- As equações foram reinseridas como os objetos OMML originais do próprio parágrafo, preservando ordem e formatação matemática.
- O PDF serve como referência de paginação; o DOCX é o artefato editável.
