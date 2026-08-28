# Parecer Acadêmico — Análise Crítica do Manuscrito

**Título avaliado:** "Análise do Impacto de um Pré-Processamento Simétrico O(n) na Redução de Inversões e Eficiência de Algoritmos de Ordenação Adaptativos e Não Adaptativos"
**Autores:** Samuel da Penha Nascimento, Francisco das Chagas Rocha (UESPI)
**Metodologia de avaliação:** leitura integral do `.tex`, simulação computacional do algoritmo de pré-processamento (verificação exaustiva de corretude para vetores de tamanho 2–8 e testes aleatórios para tamanho até 60), conferência cruzada de todos os números do texto contra as Tabelas 1–4 e recontagem manual de percentuais de ganho.

---

## 1. Síntese do Documento

Artigo experimental em Ciência da Computação (template SBC) que propõe um algoritmo de pré-processamento simétrico de complexidade $O(n)$/espaço $O(1)$, projetado para reduzir o número de inversões em vetores antes da aplicação de cinco algoritmos de ordenação (Insertion, Bubble, Selection, Merge, Quicksort). O estudo é quantitativo/experimental, implementado em Rust, benchmarkado com Criterion sobre seis topologias de entrada e quatro tamanhos de vetor ($n \le 20.000$). Conclui que o pré-processamento beneficia fortemente algoritmos quadráticos adaptativos e é de eficácia mista/marginal em algoritmos $O(n\log n)$.

A estrutura segue o padrão esperado (Introdução, Fundamentação, Metodologia, Resultados, Conclusão), mas **não há seção de Trabalhos Relacionados**, o que compromete a avaliação de originalidade (ver item C.6).

---

## 2. Problemas Identificados

### A. Consistência entre objetivos, resultados e conclusões

**A.1 — 🔴 Crítico — Generalização não sustentada sobre "métodos quasilineares"**
- **Localização:** Resumo/Abstract ("remaining ineffective for quasi-linear methods"); Seção 4.3, parágrafo de abertura ("...contraste entre o expressivo aproveitamento lógico nos métodos quadráticos e a limitação de *hardware* nos algoritmos de ordem $O(n\log n)$").
- **Problema:** O artigo classifica Merge Sort *e* Quicksort como "quasilineares" (Seção 2.2). Contudo, a Tabela 2 mostra que o Quicksort obtém ganhos expressivos e estatisticamente relevantes em 3 das 6 topologias — Tartarugas (+75,3%), Zigue-zague (+53,7%) e Invertido (+48,9%) — contradizendo diretamente a afirmação de que o método é "ineficaz para métodos quasilineares". Apenas o Merge Sort exibe variações marginais (-14,0% a +14,5%) em todas as topologias.
- **Justificativa:** Uma afirmação central do Resumo e da síntese de resultados (Seção 4.3) não é sustentada pelos próprios dados apresentados na Tabela 2. A seção "Considerações Finais" (Seção 5), de forma mais cautelosa, restringe corretamente a limitação a "entradas aleatórias" — mas essa correção não é propagada ao Resumo nem à Seção 4.3, gerando inconsistência interna entre partes do artigo.
- **Correção recomendada:** Reformular o Resumo e a Seção 4.3 para diferenciar explicitamente o comportamento de Merge (consistentemente marginal) do de Quicksort (fortemente positivo em entradas estruturadas, negativo apenas em entradas aleatórias/quase ordenadas/com duplicatas). Sugestão de reescrita para o Resumo:
  > "...achieving speedups of up to 99.9% in adaptive quadratic algorithms, with mixed effects on $O(n\log n)$ methods — substantial gains for Quicksort on structured inputs, but negligible or negative impact on Merge Sort and on Quicksort under uniformly random inputs."

**A.2 — 🟠 Importante — Resultados do Selection Sort não discutidos e aparentemente inconsistentes com a caracterização teórica**
- **Localização:** Seção 3.2 (caracterização do Selection Sort como "não adaptativo... mantém um número fixo de comparações independentemente da configuração inicial"); Tabela 2, linhas "Seleção".
- **Problema:** Se o número de comparações do Selection Sort é realmente fixo e independente da ordem de entrada, o tempo de execução não deveria variar de forma expressiva entre a versão pura e a pré-processada. No entanto, a Tabela 2 mostra variações estatisticamente relevantes (considerando os IC de 95% informados): Zigue-zague +22,1%, Invertido -11,8%, Duplicados -8,3%. O texto da Seção 4 nunca comenta esses resultados — apenas Insertion, Bubble, Merge e Quicksort são discutidos.
- **Justificativa:** Há uma lacuna de análise sobre um resultado que aparentemente contradiz a premissa teórica estabelecida pelos próprios autores. Isso pode indicar (i) efeitos de localidade de cache/predição de desvio nas trocas (não comparações), não contemplados na explicação teórica, ou (ii) uma característica da implementação (ex.: verificação condicional antes da troca) que altera o número de escritas em memória. Sem discussão, o leitor não sabe se é ruído, efeito real ou erro de implementação.
- **Correção recomendada:** Incluir na Seção 4 uma discussão específica sobre os resultados do Selection Sort, explicando a fonte provável da variação (por exemplo, custo de escrita/troca em memória versus custo de comparação) ou, alternativamente, investigar se há uma anomalia de medição.

**A.3 — 🟠 Importante — Explicação causal (localidade de cache) apresentada sem evidência direta**
- **Localização:** Seção 4.3, parágrafos de "Impacto no Tempo de Execução..." e "Tempo Total de Ordenação e Análise de Localidade de Caches" ("A principal justificativa arquitetural para esse *overhead* reside na localidade espacial de caches...").
- **Problema:** O artigo afirma categoricamente que a causa da degradação de desempenho em Merge/Quicksort sobre entradas aleatórias é o padrão de acesso "bifronte" à memória e a perda de eficácia do *prefetching* de hardware. Entretanto, a própria Seção 3.5 ("Ameaças à Validade") reconhece que **não há coleta de métricas de hardware (cache misses, branch mispredictions)** por indisponibilidade do `perf_event_open` no Windows.
- **Justificativa:** Uma afirmação causal específica sobre o mecanismo microarquitetural é apresentada como conclusão estabelecida, mas não há dado empírico do próprio estudo que a sustente — é uma inferência plausível, não um resultado verificado.
- **Correção recomendada:** Reformular a linguagem para deixar claro o caráter hipotético da explicação (ex.: "uma hipótese plausível, compatível com a literatura, é que..." em vez de "a principal justificativa..."), ou remover a atribuição causal específica e limitar-se a reportar o efeito observado, remetendo a métricas de cache como trabalho futuro (o que, de fato, já é proposto na Seção 5).

**A.4 — 🟡 Moderado — Redução de inversões no cenário Zigue-zague não é explicada**
- **Localização:** Seção 4.1 ("Redução de Inversões e Viés Estrutural do Caso Invertido").
- **Problema:** A seção oferece uma explicação geométrica detalhada para a redução de 100% no caso Invertido, mas não oferece nenhuma explicação para a redução quase completa (99,9%, de 24.997.500 para 2.501) no caso Zigue-zague — resultado igualmente extremo e usado como base para os maiores ganhos de tempo relatados no artigo (Inserção +99,8%, Bubblesort +99,9%).
- **Justificativa:** Tratamento assimétrico da discussão: o resultado mais explorado no restante do artigo (Zigue-zague) é o menos explicado analiticamente.
- **Correção recomendada:** Adicionar um parágrafo explicando por que a topologia Zigue-zague responde tão bem à simetria do algoritmo (provavelmente porque a alternância de blocos crescentes/decrescentes coincide, ao menos parcialmente, com a geometria de espelhamento do pré-processamento — de forma análoga, mas não idêntica, ao caso Invertido).

### B. Metodologia e reprodutibilidade

**B.1 — 🟠 Importante — Estratégia de pivô do Quicksort não informada**
- **Localização:** Seção 3.2 e Seção 3 (Metodologia) como um todo.
- **Problema:** O artigo não especifica a estratégia de seleção de pivô usada na implementação do Quicksort (primeiro elemento, mediana de três, aleatório, ou uso da função de ordenação padrão do Rust — que na verdade é um *pattern-defeating quicksort/introsort*, não um quicksort puro). Isso é tecnicamente relevante: o tempo do Quicksort no cenário Invertido (144,62 µs) é da mesma ordem de grandeza do cenário Aleatório (334,51 µs) — comportamento incompatível com um Quicksort ingênuo de pivô fixo, que sofreria degradação para $O(n^2)$ em entrada invertida.
- **Justificativa:** Sem essa informação, não é possível reproduzir o experimento nem interpretar corretamente por que o Quicksort não exibe o pior caso teórico esperado sobre entrada invertida — o que também afeta a interpretação dos ganhos atribuídos ao pré-processamento.
- **Correção recomendada:** Declarar explicitamente a implementação usada (biblioteca padrão Rust `sort_unstable`? implementação própria? qual estratégia de pivô?). Se for a função padrão do Rust, isso deve ser dito claramente, pois ela não é um "quicksort" na acepção clássica descrita na Seção 2.2.

**B.2 — 🟠 Importante — Procedimento de geração das topologias insuficientemente detalhado**
- **Localização:** Tabela 1 (Seção 3.1).
- **Problema:** As descrições são conceituais, mas faltam parâmetros exatos de geração: como os valores são sorteados dentro de cada metade em "Tartarugas"; tamanho dos blocos em "Zigue-zague"; distribuição exata dos valores {0,1,2} em "Duplicados" (uniforme? enviesada?).
- **Justificativa:** Sem esses parâmetros, outro pesquisador não consegue reproduzir exatamente as mesmas entradas (mesmo com a seed 42 informada, pois a seed sozinha não define o algoritmo de construção do vetor).
- **Correção recomendada:** Incluir pseudocódigo curto ou fórmula explícita de geração para cada topologia, ou apontar diretamente para a função correspondente no repositório citado (`nascimento_repositorio_2026`).

**B.3 — 🟡 Moderado — Parâmetros do Criterion não especificados**
- **Localização:** Seção 3.3 (Protocolo de medição).
- **Problema:** Não são informados o número de amostras, o tempo de warm-up nem o tempo de medição usados pelo Criterion (os valores padrão da biblioteca podem ser usados, mas isso deveria ser explicitado, já que afeta diretamente a largura dos intervalos de confiança reportados).
- **Correção recomendada:** Acrescentar uma frase como "Utilizou-se configuração padrão do Criterion (100 amostras, 3s de warm-up, 5s de medição)" — ou os valores efetivamente usados, se customizados.

**B.4 — 🟡 Moderado — "Ameaças à Validade" é breve e omite ameaças relevantes**
- **Localização:** Seção 3.5.
- **Problema:** A seção cobre apenas ruído de medição temporal e ausência de métricas de cache. Não discute: (i) validade externa — apenas uma máquina/SO testados, sem replicação em outro hardware; (ii) uso exclusivo de dados sintéticos, sem validação em dados reais; (iii) ausência de teste de hipótese estatístico formal (apenas IC de 95% descritivos); (iv) $n$ máximo de 20.000 pode ser pequeno demais para que diferenças assintóticas $O(n\log n)$ se manifestem plenamente frente ao $O(n)$ do pré-processamento.
- **Correção recomendada:** Expandir a seção para cobrir essas ameaças, ao menos brevemente, e retomar essa discussão nas Considerações Finais.

**B.5 — 🟡 Moderado — Ausência de fundamentação analítica sobre a redução de inversões**
- **Localização:** Seção 3.4 (Formalização do Pré-Processamento Simétrico).
- **Problema:** A seção apresenta apenas a análise de complexidade temporal/espacial ($O(n)$/$O(1)$), mas nenhuma prova ou cota analítica de quanto o algoritmo reduz o número de inversões no pior caso ou caso médio — a eficácia da redução é conhecida apenas empiricamente (Tabela 1). Executei uma verificação computacional exaustiva (permutações de tamanho 2 a 8) e testes aleatórios (tamanho até 60): em nenhum caso testado o algoritmo *aumentou* o número de inversões, o que sugere uma propriedade monotônica não trivial que os autores poderiam ter explorado analiticamente, mas que não está demonstrada no texto.
- **Correção recomendada:** Se possível, incluir um lema com prova (mesmo que informal) de que o algoritmo nunca aumenta o número de inversões, ou ao menos reconhecer explicitamente essa lacuna como limitação teórica.

### C. Fundamentação teórica e trabalhos relacionados

**C.6 — 🟠 Importante — Ausência de seção de Trabalhos Relacionados**
- **Localização:** Estrutura geral do artigo (falta uma seção entre Introdução/Fundamentação e Metodologia).
- **Problema:** O artigo cita genericamente `hwang_presorting_2000` (técnicas de *presorting*) e `estivill-castro_survey_1992` (survey de ordenação adaptativa), mas nunca posiciona a técnica proposta frente a alternativas conhecidas — por exemplo, *cocktail sort* (bubble sort bidirecional), *comb sort*, ou os próprios métodos de *presorting* citados. A técnica proposta tem semelhança estrutural com uma passagem única de *cocktail sort*, e essa relação não é discutida.
- **Justificativa:** A dimensão "Originalidade e Contribuição" não pode ser adequadamente avaliada sem uma comparação explícita com técnicas correlatas — é essencial para justificar a contribuição científica do trabalho perante revisores.
- **Correção recomendada:** Adicionar uma subseção (ou parágrafo robusto na Fundamentação) comparando a proposta com técnicas de pré-ordenação/pré-processamento já existentes na literatura citada, destacando similaridades e diferenças (ex.: custo, garantias teóricas, aplicabilidade).

### D. Consistência numérica e apresentação gráfica

**D.7 — 🟡 Moderado — Inconsistência entre "up to 99,9%" (Resumo) e o valor máximo real (100,0%) da Tabela 2**
- **Localização:** Resumo/Abstract ("speedups up to 99.9%"); Tabela 2, linha "Bubblesort/Invertido" (+100,0%).
- **Problema:** O maior ganho relatado na Tabela 2 é justamente +100,0% (Bubblesort, Invertido: de 19.816,65 µs para 9,17 µs — uma redução de 99,95%, arredondada para 100,0% na tabela). O Resumo, porém, cita "up to 99,9%" como valor máximo, o que não reflete o dado de maior magnitude da própria tabela.
- **Correção recomendada:** Uniformizar a casa decimal usada em todas as citações de percentuais (uma casa decimal) e conferir que o "valor máximo" citado no Resumo corresponda de fato ao maior valor da Tabela 2.

**D.8 — 🟡 Moderado — Figura de inversões (escala logarítmica) não pode representar corretamente o valor zero**
- **Localização:** Figura 2 (`fig:inversoespos`), eixo Y em escala logarítmica com `ymin=100`.
- **Problema:** A Tabela 1 mostra que, no cenário Invertido, o número de inversões após o pré-processamento é exatamente 0. Um valor zero não pode ser plotado em escala logarítmica — a barra correspondente ficará ausente ou visualmente cortada pelo `ymin=100`, o que pode induzir o leitor a interpretar erroneamente o resultado mais expressivo do estudo (eliminação total das inversões) como dado ausente.
- **Correção recomendada:** Usar uma escala linear para essa figura específica, ou adicionar uma anotação textual/marcador explícito indicando "0" sobre a barra ausente do cenário Invertido.

**D.9 — 🟢 Menor — Inconsistência na contagem de seções**
- **Localização:** Introdução, último parágrafo ("Este artigo está estruturado em cinco seções, além desta introdução...").
- **Problema:** O texto anuncia "cinco seções, além desta introdução", mas em seguida descreve apenas quatro (Seção 2 — Fundamentação; Seção 3 — Metodologia; Seção 4 — Resultados; Seção 5 — Considerações Finais).
- **Correção recomendada:** Corrigir para "quatro seções, além desta introdução" (ou renumerar caso se pretenda incluir a seção de Trabalhos Relacionados sugerida em C.6, o que tornaria "cinco seções" correto).

### E. Estilo, redação e idioma

**E.10 — 🟢 Menor — Título excessivamente longo**
- **Localização:** Título do artigo (24 palavras).
- **Problema:** Título extenso dificulta indexação e leitura rápida por parte de revisores/leitores, embora não viole nenhuma norma específica.
- **Correção recomendada:** Uma versão mais concisa, por exemplo: *"Pré-Processamento Simétrico $O(n)$ para Redução de Inversões: Impacto em Algoritmos de Ordenação Adaptativos e Não Adaptativos"*.

**E.11 — 🟢 Menor — Ajustes no Abstract em inglês**
- **Localização:** Abstract.
- **Problema:** "This paper investigates if a low-cost..." — em inglês formal/acadêmico, o uso correto é "whether", não "if", em orações substantivas subordinadas. "physical hardware performance" é uma construção pouco natural.
- **Correção recomendada:**
  > "This paper investigates whether a low-cost pre-processing technique can reduce inversions such that $C_{pre} + C_{sort} < C_{original}$."
  > Trocar "severely degrade their physical hardware performance" por algo como "severely degrade their real-world execution performance".

**E.12 — 🟢 Menor — Redundância entre trabalhos futuros já citados na Metodologia e repetidos nas Considerações Finais**
- **Localização:** Seção 3.3 (parágrafo sobre árvore Fenwick) vs. Seção 5 (parágrafo final).
- **Problema:** A proposta de substituir a contagem $O(n^2)$ por uma árvore Fenwick/MergeSort modificado já é apresentada com riqueza de detalhes na Metodologia (Seção 3.3) e é repetida quase integralmente nas Considerações Finais.
- **Correção recomendada:** Nas Considerações Finais, apenas retomar brevemente o ponto, sem repetir a explicação técnica já dada anteriormente.

### F. Referências

**F.13 — ⚪ Não verificável — Arquivo `referencias.bib` não fornecido**
- **Localização:** Todo o artigo (comando `\bibliography{referencias}`).
- **Problema:** Sem o arquivo `.bib`, não é possível conferir conformidade com a NBR 6023 (formatação de referências), verificar se todas as citações no texto possuem entrada correspondente, nem avaliar a atualidade/adequação das fontes.
- **Correção recomendada:** Caso deseje essa verificação, envie o arquivo `referencias.bib` (ou a lista de referências) para análise específica de conformidade ABNT.

---

## 3. Pontos Fortes

- Pergunta de pesquisa clara, formalizada por meio da condição $C_{pre} + C_{sort} < C_{original}$, mantida como fio condutor consistente ao longo do artigo.
- Protocolo experimental bem controlado: seed fixa (ChaCha8Rng), hardware/SO documentados, uso de Criterion com IC de 95%, seis topologias de entrada bem escolhidas para cobrir casos representativos (aleatório, adversarial, quase ordenado, com duplicatas).
- Análise de complexidade do algoritmo proposto (Seção 3.4) correta e verificada por mim computacionalmente: o algoritmo de fato executa $\lfloor n/2 \rfloor$ iterações com número constante de operações por iteração, resultando em $O(n)$ tempo e $O(1)$ espaço.
- Números apresentados no texto (percentuais, tempos) conferem, em sua grande maioria, com os valores exatos das tabelas — não há evidência de fabricação ou erro grosseiro de transcrição de dados (à exceção da imprecisão de arredondamento apontada em D.7).
- Transparência ao reconhecer explicitamente, na Seção 3.5, a limitação de não haver métricas de hardware (cache) devido à indisponibilidade do `perf_event_open` no Windows.
- Repositório do projeto citado como fonte para reprodução do código (`nascimento_repositorio_2026`).

---

## 4. Pontos Críticos e Prioridades de Melhoria

| Prioridade | Item | Resumo |
|---|---|---|
| Alta | A.1 | Generalização não sustentada sobre eficácia em métodos "quasilineares" (Quicksort contradiz a afirmação) |
| Alta | A.2 | Resultados do Selection Sort não discutidos e aparentemente inconsistentes com a teoria apresentada |
| Alta | B.1 | Estratégia de pivô do Quicksort não informada — compromete reprodutibilidade e interpretação dos resultados |
| Alta | C.6 | Ausência de seção de Trabalhos Relacionados — compromete avaliação de originalidade |
| Média | A.3 | Explicação causal (cache) apresentada sem evidência direta coletada no estudo |
| Média | B.2 | Procedimento de geração das topologias de entrada insuficientemente detalhado |
| Média | A.4 | Redução de inversões no caso Zigue-zague não explicada |
| Média | B.4 | Seção de Ameaças à Validade incompleta |
| Média | B.3 | Parâmetros do Criterion não especificados |
| Média | B.5 | Ausência de cota analítica sobre a eficácia da redução de inversões |
| Média | D.7 / D.8 | Ajustar percentual máximo citado no Resumo; corrigir escala da Figura 2 |
| Baixa | D.9, E.10, E.11, E.12 | Correções textuais e editoriais |
| — | F.13 | Verificar conformidade ABNT das referências (requer `.bib`) |

---

## 5. Veredito Geral

**Classificação: Necessitando de Revisão Substancial.**

O artigo tem mérito científico real — o protocolo experimental é sólido, os dados numéricos são majoritariamente consistentes entre si, e a proposta é simples e bem implementada. No entanto, **a conclusão central do Resumo (ineficácia em métodos "quasilineares") não é sustentada pelos próprios dados do Quicksort**, há resultados relevantes (Selection Sort) que não são analisados, faltam informações essenciais de reprodutibilidade (estratégia de pivô, geração exata das topologias) e não há seção de Trabalhos Relacionados para posicionar a contribuição frente à literatura de *presorting* já citada. Esses pontos, tomados em conjunto, são suficientes para comprometer a validade da narrativa apresentada e devem ser corrigidos antes da submissão — não se trata de ajustes cosméticos, mas de revisão da interpretação dos próprios resultados.

## 6. Principais Correções Necessárias (em ordem de prioridade)

1. Reformular o Resumo e a Seção 4.3 para refletir com precisão o comportamento do Quicksort (não generalizar para "métodos quasilineares" como um todo) — item A.1.
2. Adicionar discussão sobre os resultados do Selection Sort na Seção 4 — item A.2.
3. Especificar a estratégia de pivô/implementação do Quicksort utilizada — item B.1.
4. Adicionar uma seção (ou subseção) de Trabalhos Relacionados posicionando a proposta frente a técnicas de *presorting* já citadas — item C.6.
5. Suavizar/reformular a atribuição causal sobre localidade de cache, deixando claro que é hipótese não verificada diretamente — item A.3.
6. Detalhar o procedimento exato de geração de cada topologia de vetor — item B.2.
7. Explicar a redução quase total de inversões no cenário Zigue-zague — item A.4.
8. Expandir a seção "Ameaças à Validade" — item B.4.
9. Especificar parâmetros do Criterion (amostras, warm-up, tempo de medição) — item B.3.
10. Corrigir a escala da Figura 2 (valor zero em escala log) e uniformizar o percentual máximo citado no Resumo — itens D.7/D.8.
11. Corrigir a contagem de seções na Introdução e demais ajustes editoriais — itens D.9, E.10–E.12.

## 7. Checklist Final

**Já adequado:**
- [x] Estrutura geral do artigo (Introdução → Fundamentação → Metodologia → Resultados → Conclusão)
- [x] Análise de complexidade do algoritmo proposto (verificada como correta)
- [x] Consistência numérica entre a maior parte do texto e as Tabelas 1–2
- [x] Documentação do ambiente experimental (hardware, SO, seed, linguagem)
- [x] Transparência quanto à limitação de métricas de hardware indisponíveis
- [x] Formalismo matemático da condição $C_{pre}+C_{sort}<C_{original}$ usado de forma consistente

**Ainda precisa de correção:**
- [ ] Generalização indevida sobre métodos "quasilineares" no Resumo/Seção 4.3
- [ ] Discussão ausente sobre resultados do Selection Sort
- [ ] Informações de implementação do Quicksort (pivô)
- [ ] Seção de Trabalhos Relacionados
- [ ] Rigor na atribuição causal sobre cache/hardware
- [ ] Detalhamento do procedimento de geração das topologias de entrada
- [ ] Explicação do resultado Zigue-zague
- [ ] Expansão das Ameaças à Validade
- [ ] Parâmetros do Criterion
- [ ] Escala da Figura 2 e uniformização do percentual máximo no Resumo
- [ ] Consistência da contagem de seções e pequenos ajustes de redação/inglês
- [ ] Conformidade ABNT das referências (não verificável sem o `.bib`)

---

*Observação metodológica: para o item B.5, realizei verificação computacional própria (não fornecida pelos autores) simulando o algoritmo descrito no Algoritmo 1 em Python, testando exaustivamente todas as permutações de vetores de tamanho 2 a 8 e 3.000 vetores aleatórios de tamanho até 60. Em nenhum caso o algoritmo aumentou o número de inversões — isso corrobora a plausibilidade dos resultados empíricos reportados, mas não substitui uma prova formal, que o artigo não apresenta.*
