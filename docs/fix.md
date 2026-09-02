# Parecer Acadêmico — "Análise do Impacto de um Pré-Processamento Simétrico O(n) na Redução de Inversões e Eficiência de Algoritmos de Ordenação Adaptativos e Não Adaptativos"

**Autores:** Samuel da Penha Nascimento, Francisco das Chagas Rocha (UESPI)
**Documento avaliado:** `main.tex` (554 linhas)

## Nota metodológica sobre esta avaliação

Não foram fornecidos `referencias.bib` nem os arquivos `resultados/*.csv` que alimentam tabelas e figuras. Por isso:
- **Verifiquei rigorosamente**: a consistência aritmética entre texto, tabelas e figuras (recalculei manualmente todos os percentuais apresentados); a corretude lógica do Algoritmo 1 (tracei sua execução em casos de teste); a coerência entre objetivos, metodologia e conclusões; e a existência/adequação das referências citadas via busca externa.
- **Não pude verificar**: a formatação ABNT/SBC completa da lista de referências; os dados brutos por trás dos CSVs (apenas a consistência interna dos números reportados no `.tex`); a compilação/renderização final do PDF (sem `sbc-template.cls`).

---

## Síntese do Documento

Artigo experimental (padrão SBC) que propõe um pré-processamento O(n) de "trocas simétricas" para reduzir inversões antes de aplicar cinco algoritmos de ordenação (Bubble, Insertion, Selection, Merge, Quicksort de três vias) sobre seis topologias de vetores. Conclui que a técnica beneficia fortemente os quadráticos adaptativos, é neutra para o Selection Sort e prejudica ou tem efeito misto nos O(n log n). O texto é bem escrito, com boa fundamentação teórica e discussão analítica cuidadosa; os principais problemas encontrados são de **transparência metodológica/reprodutibilidade** e de **precisão entre o que o abstract afirma e o que os dados de fato mostram** — não de erros matemáticos ou de implementação.

---

## Problemas Identificados

### 🟠 Importante

**1. Localização:** Abstract (l.56) e Resumo (l.63) — trecho sobre Selection Sort.
**Problema:** O abstract afirma *"For the non-adaptive Selection Sort no significant change was observed"*, mas a Tabela `tab:tempos` e o texto da Seção 4.3 (l.465) mostram uma degradação de −12,3% no cenário *Invertido* em n=10.000, **replicada** em n=100.000 (−12,8%), com intervalos de confiança de 95% que não se sobrepõem (18.140,82–18.384,52 μs vs. 20.465,81–20.565,17 μs) — ou seja, uma diferença estatisticamente robusta, não ruído de medição.
**Justificativa:** Isso é exatamente uma afirmação do abstract não sustentada integralmente pelos resultados apresentados no corpo do artigo (o abstract generaliza demais um resultado que o próprio artigo trata com nuance).
**Correção recomendada:** Reformular o trecho do abstract/resumo para refletir a exceção, por exemplo:
> *"For the non-adaptive Selection Sort, results were largely unchanged across topologies, with the exception of a consistent ~12% slowdown on the fully inverted input, attributed to branch-prediction effects."*

**2. Localização:** Abstract (l.56) — trecho sobre Merge Sort.
**Problema:** A frase *"the O(n log n) methods did not benefit"* é seguida, na mesma sentença, por *"Merge Sort showed mixed results"* — mas "mixed results" inclui ganhos reais de até +14,8% (n=10.000) e até ≈+18% (n=10⁶, conforme texto da Seção 4.3, l.463). Dizer que o método "não se beneficiou" e, na sequência, admitir ganhos de quase 20% é uma contradição leve.
**Justificativa:** Overclaiming no resumo executivo, o trecho que a maioria dos leitores/avaliadores lerá primeiro.
**Correção recomendada:**
> *"...whereas the O(n log n) methods showed inconsistent or negligible benefit: Merge Sort's gains were confined to inputs where the technique fully or partially removed structural disorder, while the three-way Quicksort consistently degraded."*

**3. Localização:** Tabela `tab:precusto` (l.441–457), linha "20.000".
**Problema:** A Seção 3.2 (l.175) declara explicitamente os tamanhos testados: *"n ∈ {1.000, 5.000, 10.000, 100.000, 1.000.000}"* para os quasilineares, e "n ≤ 100.000" para os quadráticos. O valor **n = 20.000** não pertence a nenhum desses conjuntos declarados, mas aparece como linha na Tabela `tab:precusto` — e não aparece na Figura `fig:precusto_linear`, que usa exatamente os 5 tamanhos declarados.
**Justificativa:** Inconsistência numérica entre o protocolo experimental descrito e os dados apresentados — o leitor não consegue saber de onde veio esse ponto adicional nem se o mesmo protocolo (seeds, 50 vetores etc.) foi aplicado a ele.
**Correção recomendada:** Ou (a) adicionar n=20.000 à enumeração de tamanhos testados na Seção 3.2, explicando por que foi incluído apenas para o custo do pré-processamento, ou (b) remover a linha da tabela se for um resíduo de uma versão anterior do experimento.

**4. Localização:** Tabela `tab:inversoes` (l.331–347) e metodologia (l.173).
**Problema:** A Seção 3.2 informa que, para cada combinação (topologia, tamanho), são gerados **50 vetores**. A Tabela `tab:inversoes` apresenta um único valor de inversões por topologia (ex.: "24.960.120" para Aleatório), sem indicar se é a contagem de um vetor representativo, a média dos 50, ou a soma.
**Justificativa:** Sem essa informação, o número não pode ser interpretado estatisticamente (uma contagem de inversões de um único vetor aleatório tem variância; a média de 50 vetores é outra grandeza) nem reproduzido de forma inequívoca.
**Correção recomendada:** Especificar explicitamente na legenda/texto da tabela: "valores médios sobre os 50 vetores gerados por topologia" (ou o que for o caso), preferencialmente acompanhados de desvio-padrão ou IC, como já é feito na Tabela `tab:tempos`.

**5. Localização:** Ao longo da Seção 4 (ex.: l.358, l.463, abstract l.56).
**Problema:** Termos como "resultados mistos", "não foi observada mudança significativa" e "degradação sistemática" são usados repetidamente sem que o artigo declare, em nenhum ponto, o **critério estatístico** empregado para julgar "significância" (não sobreposição de IC 95%? um limiar percentual arbitrário?).
**Justificativa:** A metodologia (Seção 3.6) descreve corretamente o cálculo de IC 95%, mas não conecta esse cálculo ao critério de decisão usado na discussão — a inferência de "significativo"/"não significativo" fica implícita.
**Correção recomendada:** Acrescentar uma frase na Seção 3.6 ou no início da Seção 4, por exemplo: *"Considerou-se uma variação relevante quando os intervalos de confiança de 95% das duas condições (pura e com pré-processamento) não se sobrepunham."* — e aplicar esse critério de forma consistente nas afirmações qualitativas.

**6. Localização:** Tabela `tab:datasets` (l.177–193), topologias "Tartarugas", "Zigue-zague" e "Duplicados".
**Problema:** As descrições são qualitativas demais para reprodução independente: "Tartarugas" não define a distribuição de valores em cada metade nem se há embaralhamento interno; "Zigue-zague" não define o tamanho dos blocos de alternância; "Duplicados" não informa se a atribuição dos valores {0,1,2} é uniforme, sequencial ou correlacionada espacialmente (isso é relevante — ver item 15 abaixo, sobre o tempo excepcionalmente baixo do Quicksort puro nesta topologia). Compare com "Quase ordenado" (n/100 trocas aleatórias) e "Invertido" (estritamente decrescente), que são precisas.
**Justificativa:** O próprio pedido do artigo é reprodutibilidade ("Todos os experimentos podem ser reproduzidos..."); a reprodução via apenas o texto do artigo, sem consultar o repositório de código, não é possível para essas três topologias — o que compromete a autossuficiência do manuscrito como registro científico.
**Correção recomendada:** Incluir, mesmo que em uma nota de rodapé ou apêndice, a definição algorítmica exata (pseudocódigo curto ou fórmula) de cada topologia, análoga ao que já foi feito para "Quase ordenado".

**7. Localização:** Seção 4.3 (l.466–469), explicação sobre *branch predictor*.
**Problema:** A hipótese sobre o preditor de desvios como causa da degradação do Selection Sort e do Quicksort em entradas aleatórias é apresentada sem nenhuma citação de apoio — nem da literatura de arquitetura de computadores sobre previsão de desvio em geral, nem de trabalhos que já relacionam esse efeito a algoritmos de ordenação especificamente.
**Justificativa:** É exatamente o tipo de "afirmação que precisa de referência" solicitado nesta análise: a hipótese é plausível e devidamente qualificada como especulativa ("parece decorrer"), mas fica sem lastro bibliográfico, o que a torna mais vulnerável a questionamento de revisores — sobretudo por não haver medição direta de *branch mispredictions* (reconhecidamente indisponível no Windows, conforme a própria Seção 3.7).
**Correção recomendada:** Adicionar ao menos uma referência de apoio sobre o impacto do preditor de desvios em algoritmos com ramificação dependente de dados (há literatura estabelecida sobre isso, inclusive específica a algoritmos de ordenação e busca).

**8. Localização:** Seção 5 — Considerações Finais (l.543–549).
**Problema:** A metodologia (Seção 3.1, l.123) define explicitamente **três dimensões** de análise: quadráticos adaptativos, quadrático não adaptativo (Selection Sort) e quasilineares. As Considerações Finais discutem apenas duas dessas três dimensões (adaptativos quadráticos e quasilineares); o Selection Sort — resultado central do "grupo de controle" do desenho experimental — não é mencionado nem uma vez na conclusão.
**Justificativa:** Quebra de coerência entre o que a metodologia promete investigar e o que a conclusão de fato sintetiza; um leitor da conclusão isoladamente não saberia que o experimento incluiu um algoritmo não adaptativo como contraponto, nem qual foi o resultado.
**Correção recomendada:** Acrescentar um parágrafo curto às Considerações Finais confirmando a hipótese de neutralidade do Selection Sort e mencionando a exceção do cenário Invertido (ligando-se ao item 1 acima).

**9. Localização:** Seção 2.3 (Introdução, l.80) vs. Tabela `tab:datasets`.
**Problema:** A Introdução afirma que os experimentos abrangem *"situações de melhor caso, pior caso e ordenação parcial"*. As seis topologias definidas cobrem claramente o pior caso (Invertido) e ordenação parcial (Quase ordenado), mas **nenhuma topologia corresponde ao melhor caso literal** (vetor já totalmente ordenado, 0 inversões) — "Quase ordenado" é a mais próxima, mas não é o melhor caso.
**Justificativa:** Inconsistência entre o que a introdução promete metodologicamente e o desenho experimental de fato realizado.
**Correção recomendada:** Ou adicionar uma topologia "Ordenado" (vetor já em ordem, útil inclusive como baseline de custo mínimo do pré-processamento sobre entrada já ótima) ou suavizar a frase da introdução para não prometer cobertura de melhor caso.

**10. Localização:** Seção 4.3 (l.463), menções a n=10⁶ ("ganhos crescentes até ≈+18%... em n=10⁶"; "+11,1%" para Quicksort em Quase ordenado, n=10⁶).
**Problema:** Essas afirmações quantitativas específicas não têm tabela ou figura de apoio no manuscrito — todas as tabelas/figuras de tempo mostram apenas n=10.000.
**Justificativa:** O leitor/revisor não consegue verificar essas afirmações a partir do artigo em si.
**Correção recomendada:** Incluir uma tabela (mesmo que resumida, só para n=10⁶) ou, no mínimo, referenciar explicitamente material suplementar/repositório onde esses números podem ser conferidos.

### 🟡 Moderado

**11. Localização:** Introdução (l.82) — "Este artigo está estruturado em cinco seções, além desta introdução."
**Problema:** O documento tem apenas **quatro** seções após a Introdução (Fundamentação Teórica, Metodologia, Resultados e Discussão, Considerações Finais), não cinco. Além disso, a Introdução chama a Seção 2 de "Referencial Teórico", mas o título real da seção (l.84) é "Fundamentação Teórica".
**Justificativa:** Pequena inconsistência interna que afeta a navegabilidade e passa impressão de descuido na revisão final.
**Correção recomendada:** Corrigir para "quatro seções" e uniformizar o nome da Seção 2 entre a Introdução e o título real.

**12. Localização:** Seção 2.3 — citação `\cite{mannila_measures_1984}`.
**Problema:** O artigo clássico de Mannila sobre medidas de pré-ordenação ("Measures of Presortedness and Optimal Sorting Algorithms") foi publicado em *IEEE Transactions on Computers* em **1985** (vol. C-34, pp. 318–325); existe também um resumo estendido no ICALP de 1984 com título semelhante, mas é uma publicação distinta e menos citada.
**Justificativa:** Não tenho acesso ao `referencias.bib`, então não posso confirmar se o erro está de fato na entrada bibliográfica ou se os autores realmente pretendem citar a versão de 1984 — mas o ano "1984" é suspeito dado que o conteúdo descrito no texto (algoritmo ótimo para três medidas de pré-ordenação) corresponde à versão de 1985, amplamente citada dessa forma na literatura de ordenação adaptativa.
**Correção recomendada:** Conferir a entrada no `.bib` contra a fonte original e corrigir o ano/veículo se necessário.

**13. Localização:** Seção 2.3 (Trabalhos Relacionados) e Seção 3.4 (Formalização).
**Problema:** O artigo cita Hwang, Yang e Yeh (2000), que fornecem exatamente o arcabouço analítico (esperança e variância da redução de inversões para operações de pré-ordenação simples, incluindo comparação de elementos extremos) que seria diretamente aplicável para derivar teoricamente a redução esperada de inversões do próprio algoritmo proposto — mas essa derivação nunca é feita; a redução observada (ex.: 45,2% em entrada aleatória) permanece puramente empírica.
**Justificativa:** É uma oportunidade de fortalecer a contribuição teórica do artigo — atualmente o trabalho relacionado citado não é explorado além da menção descritiva.
**Correção recomendada:** Se o tempo/escopo permitir, incluir uma derivação (ainda que aproximada) da redução esperada de inversões sob entrada aleatória, usando o mesmo formalismo de Hwang et al.; caso contrário, explicitar essa lacuna como trabalho futuro.

**14. Localização:** Tabela `tab:inversoes`, coluna "Redução", linhas "Zigue-zague" e "Invertido".
**Problema:** Ambas as linhas mostram "100,0%", mas são qualitativamente diferentes: Invertido chega a exatamente 0 inversões (100% exato), enquanto Zigue-zague chega a 2.501 inversões restantes (99,99%, que arredonda para 100,0% com uma casa decimal). O texto (l.351) esclarece a diferença, mas a tabela sozinha não permite distinguir os dois casos.
**Justificativa:** Pequena perda de informação/precisão na tabela que pode levar um leitor apressado a pensar que ambos os casos zeraram as inversões.
**Correção recomendada:** Usar mais casas decimais na tabela (ex.: "99,99%" vs. "100,00%") ou marcar com nota de rodapé o caso de zero exato.

**15. Localização:** Seção 4.3 (l.463) — trecho sobre Quicksort em "Duplicados".
**Problema:** O texto explica a degradação do Quicksort em Duplicados dizendo que o pré-processamento "destrói parcialmente o agrupamento" de chaves que tornava o particionamento de três vias eficiente no vetor puro (44,92 μs, "o mais rápido de todos"). Isso pressupõe que o vetor "Duplicados" original já possui alguma estrutura espacial (agrupamento) nos valores {0,1,2} — mas a definição da topologia na Tabela `tab:datasets` ("baixa cardinalidade de chaves") não deixa claro se a atribuição desses valores é aleatória uniforme ou segue algum padrão espacial.
**Justificativa:** A explicação causal apresentada depende criticamente de um detalhe de geração de dados que não está especificado (ligado ao item 6) — sem saber exatamente como "Duplicados" é gerado, não é possível avaliar se essa explicação é a mais provável.
**Correção recomendada:** Esclarecer a forma de geração de "Duplicados" (ver item 6) e, se possível, mostrar o número de inversões inicial desse vetor para contextualizar por que já era tão favorável ao particionamento de três vias.

**16. Localização:** Seção 2.3 (Trabalhos Relacionados), como um todo.
**Problema:** Apenas três referências são discutidas (Mannila 1984/1985, Estivill-Castro & Wood 1992, Hwang et al. 2000), todas anteriores a 2000. Não há discussão de literatura mais recente sobre ordenação adaptativa na prática, efeitos de cache/branch prediction em ordenação (tema central da própria discussão do artigo, ver item 7), ou técnicas de pré-processamento/otimização de baixo custo mais atuais.
**Justificativa:** Para um artigo submetido em 2026, uma revisão da literatura concentrada inteiramente no período 1985–2000 é incomum e enfraquece o posicionamento da contribuição frente ao estado da arte atual.
**Correção recomendada:** Incluir pelo menos 2–3 referências de trabalhos mais recentes (2010–2025) sobre desempenho prático de algoritmos de ordenação, efeitos microarquiteturais em comparação/ramificação, ou pré-processamento/heurísticas de baixo custo.

**17. Localização:** Tabela `tab:datasets` — topologias "Tartarugas" e "Zigue-zague".
**Problema:** Essas topologias lembram fortemente os padrões clássicos de teste adversarial para Quicksort descritos por Bentley & McIlroy ("Engineering a Sort Function", 1993) — "organ-pipe" (metade crescente/metade decrescente, análogo a "Tartarugas") e "sawtooth" (alternância sistemática, análogo a "Zigue-zague") — amplamente usados na literatura de engenharia de algoritmos de ordenação, mas essa fonte não é citada.
**Justificativa:** Se as topologias foram inspiradas nesse (ou outro) trabalho, a citação é esperada academicamente; se são originais dos autores, isso deveria ser dito explicitamente para evitar a impressão de que se trata de um padrão já estabelecido sem crédito.
**Correção recomendada:** Verificar a origem das topologias e citar a fonte, se aplicável, ou declarar explicitamente que são definições originais dos autores.

### 🟢 Menor

**18. Localização:** Ao longo de todo o artigo (abstract, resumo, tabelas, Seção 4).
**Problema:** Nomenclatura inconsistente dos algoritmos: "Bubble Sort" (abstract) / "Bubblesort" (tabelas, resumo) / "BubbleSort" (l.461, junto); "Insertion Sort" / "Inserção" (tabelas) / "InsertionSort" (l.461); "Selection Sort" / "Seleção" (tabelas).
**Correção recomendada:** Padronizar um único formato (recomenda-se manter os nomes em inglês com espaço — "Bubble Sort", "Insertion Sort", "Selection Sort" — e usar essa forma também nos cabeçalhos das tabelas, já em português no restante do texto).

**19. Localização:** Abstract (l.56) — "at approximately 1,2 ns/element".
**Problema:** Texto em inglês usando vírgula decimal (convenção PT-BR) em vez de ponto decimal (convenção padrão em inglês: "1.2 ns/element").
**Correção recomendada:** Ajustar para "1.2 ns/element" no abstract em inglês; manter "1,2 ns/elemento" no resumo em português.

**20. Localização:** Preâmbulo (l.31) — `\usepackage{placeins}`.
**Problema:** O pacote é importado, mas nenhum comando `\FloatBarrier` é utilizado no corpo do documento.
**Correção recomendada:** Remover o import se não for necessário, ou usar `\FloatBarrier` nos pontos onde o posicionamento de figuras/tabelas precisar ser forçado antes de mudar de seção (comum em templates SBC para evitar floats "vazando" entre seções).

**21. Localização:** Diversos trechos discursivos da Seção 4 (ex.: l.471, "Toda essa dinâmica... encontra-se visualmente consolidada").
**Problema:** Registro ocasionalmente um pouco mais coloquial/retórico do que o esperado em escrita científica formal (não chega a ser erro, mas destoa do restante do texto, mais seco e técnico).
**Correção recomendada:** Revisar esses trechos para um registro mais direto e objetivo, se o periódico/evento-alvo exigir tom mais conciso.

---

## Pontos Fortes (verificados, não apenas presumidos)

- **Todos os percentuais reportados nas Tabelas `tab:inversoes`, `tab:tempos` e `tab:precusto` foram recalculados manualmente e conferem** com os valores brutos apresentados — não foi encontrada nenhuma inconsistência aritmética entre números brutos e percentuais derivados (com exceção pontual do item 3, sobre a origem do dado n=20.000).
- **O Algoritmo 1 foi verificado por traço manual** em vetores estritamente decrescentes de tamanho par e ímpar: a afirmação de que a etapa de troca extrema, por si só, reverte completamente um vetor invertido em O(n), resultando em zero inversões, está **matematicamente correta**, e a lógica de fronteira (condições `i+1<meio` e `j-1>meio`) está livre de erros de índice fora dos limites ou de sobreposição entre as metades esquerda/direita.
- **A análise de complexidade** (O(n) temporal, O(1) espacial) do pré-processamento está corretamente justificada a partir da estrutura do laço.
- **As referências centrais ao tema (Mannila; Estivill-Castro & Wood; Hwang, Yang & Yeh) existem e correspondem tematicamente** ao que o texto descreve sobre cada uma (conferido via busca externa), com a ressalva do possível erro de ano no caso de Mannila (item 12).
- **Ambiente experimental bem documentado** (SO, CPU, RAM, versão do Rust, flags de compilação, seed de geração determinística) — acima da média em transparência computacional.
- **O tratamento do caso "100% de redução" no cenário Invertido é epistemologicamente cuidadoso**: os autores não tratam isso como uma vitória genérica da técnica, mas corretamente atribuem o resultado à geometria específica da transformação coincidindo com a estrutura da entrada — evitando uma generalização indevida.

---

## Veredito Geral

**Necessitando de revisão substancial.**

O núcleo científico do artigo é sólido: a técnica é corretamente formalizada, sua complexidade está corretamente derivada, e os números apresentados são internamente consistentes onde pude verificá-los. Não há erro de implementação, de matemática ou de lógica experimental que invalide a contribuição central. Contudo, há um conjunto real de problemas de **transparência metodológica e de precisão entre abstract/conclusão e os dados detalhados** (itens 1–10) que precisam ser corrigidos antes da submissão — a maioria são ajustes textuais/de tabela, mas alguns exigem retornar aos dados brutos (itens 3, 4, 6, 10) para esclarecer origem e agregação de valores, o que está além de uma simples revisão de texto.

## Principais Correções Necessárias (em ordem de prioridade)

1. Corrigir a contradição abstract × resultados sobre o Selection Sort e o Merge Sort (itens 1–2).
2. Esclarecer a origem do ponto n=20.000 na Tabela `tab:precusto` frente ao protocolo declarado (item 3).
3. Especificar se os valores da Tabela `tab:inversoes` são médias, somas ou amostra única sobre os 50 vetores gerados (item 4).
4. Definir explicitamente o critério estatístico usado para "significância"/"resultados mistos" (item 5).
5. Completar a especificação de geração das topologias "Tartarugas", "Zigue-zague" e "Duplicados" (item 6).
6. Adicionar referência de apoio à hipótese de branch prediction (item 7).
7. Incluir o Selection Sort nas Considerações Finais (item 8).
8. Ajustar a frase da Introdução sobre cobertura de "melhor caso" (item 9).
9. Disponibilizar/tabelar os dados de n=10⁶ mencionados na discussão (item 10).
10. Corrigir contagem/nome de seções na Introdução (item 11) e demais itens moderados/menores conforme tempo disponível.

## Checklist Final

| Aspecto | Status |
|---|---|
| Corretude matemática/lógica do algoritmo proposto | ✅ Adequado (verificado por traço manual) |
| Consistência aritmética texto ↔ tabelas ↔ figuras | ✅ Adequado, exceto item 3 (n=20.000) |
| Coerência objetivos → metodologia → resultados → conclusão | ⚠️ Requer ajustes (itens 8, 9) |
| Sustentação das afirmações do abstract pelos resultados | ⚠️ Requer ajustes (itens 1, 2) |
| Reprodutibilidade a partir do texto do artigo (sem o repositório) | ⚠️ Requer ajustes (itens 4, 6) |
| Definição de critérios estatísticos | ⚠️ Ausente (item 5) |
| Fundamentação teórica dos mecanismos causais propostos | ⚠️ Requer referência (item 7) |
| Atualidade da revisão de literatura | 🟡 Melhorável (item 16) |
| Documentação do ambiente experimental | ✅ Adequado |
| Referências centrais existem e são tematicamente corretas | ✅ Adequado, exceto possível ano (item 12) |
| Formatação/nomenclatura/estilo | 🟢 Pequenos ajustes (itens 18–21) |
| Conformidade ABNT/SBC completa das referências | ❓ Não verificável sem `referencias.bib` |

Não há problema classificado como **🔴 Crítico**: nenhum dos pontos encontrados compromete, por si só, a validade científica central do trabalho ou impediria a publicação — mas o volume de itens 🟠 Importante justifica uma rodada de revisão substancial antes da submissão.