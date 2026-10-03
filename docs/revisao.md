Já consultei as normas oficiais da REIC e agora estou inspecionando o PDF em si (páginas, fontes, layout) antes de montar a auditoria.# 1. DIAGNÓSTICO EXECUTIVO

**NÃO PRONTO PARA SUBMISSÃO**

O PDF tem **14 páginas**, e a REIC exige entre 6 e 12 páginas incluindo as referências, e declara que o descumprimento de qualquer item invalida a submissão. Além disso, há duas falhas científicas que um revisor encontra rápido:

- A expressão E[ΔI] = n²/12 + O(n) e a explicação para os 45,19% estão erradas. Os números do próprio artigo e a minha simulação mostram isso.
- A conclusão central ("o método jamais supera o ordenador padrão") é contradita por uma linha das suas Tabelas 5 e 10.

**Como verifiquei**
- **Normas:** li as páginas oficiais atuais da REIC (journals-sol.sbc.org.br). Não consegui abrir o modelo do Overleaf, então a conformidade exata com o template ficou **INCERTO**.
- **Conflito entre páginas oficiais:** páginas antigas no domínio UFRGS dizem "somente português" e "referências sem limite de páginas". As páginas do SOL (atuais, com marcações **[NOVO]**) dizem português, inglês ou espanhol e 6 a 12 páginas no total. Segui as do SOL, porque a página antiga avisa que a revista mudou de endereço.
- **Edição 2026:** não encontrei regras específicas dela. A REIC tem periodicidade trimestral e avaliação contínua. A página do CSBC 2026 informa que os trabalhos aceitos no CTIC saem como edição especial da REIC. Se o seu envio for por esse concurso, confira o edital dele.
- **PDF:** li o texto inteiro. Inspecionei visualmente as páginas 1 e 11 (em baixa resolução) e verifiquei o PDF tecnicamente: A4, fontes embutidas, figuras vetoriais. O restante do layout verifiquei só pela camada de texto.
- **Reimplementação:** reimplementei o Algoritmo 1 e rodei em n = 10.000. Os resultados bateram **exatamente** com a Tabela 3 em Tartarugas, Zigue-zague, Invertido, Serra e Tubo. Aleatório (45,6% contra 45,19%) e Duplicados (62,13% contra 62,12%) também ficaram coerentes. Isso é um bom sinal para a corretude do que você mediu.

---

# 2. PROBLEMAS CRÍTICOS

| Página/Seção | Problema | Evidência | Impacto | Correção necessária |
|---|---|---|---|---|
| Documento inteiro | 14 páginas, acima do máximo de 12 (incluindo referências) | O Apêndice (Tabelas 10–12) ocupa as páginas 13–14 e as referências terminam na 14 | Invalidação da submissão, ou desk reject (a REIC verifica o formato nessa triagem, segundo a SBC) | Cortar pelo menos 2 páginas. Mover as Tabelas 10–12 para o repositório. Condensar a Seção 4.3 e as Considerações Finais, que repetem números |
| §2.4, §3.4, §5 (n²/12) | A fórmula E[ΔI] = n²/12 + O(n) é apresentada como derivada, mas o resultado medido a contradiz em ordem de grandeza | Medido: ΔI = 24.960.120 − 13.681.366 ≈ 11,28 M ≈ 0,113·n². Previsto: 0,083·n². A diferença é ≈ 2,9 M. Cada troca adjacente remove exatamente 1 inversão, então os ajustes adjacentes contribuem no máximo ≈ n = 10⁴. A explicação "ajustes adjacentes complementam o termo de primeira ordem" erra por duas ordens de grandeza | Resultado teórico falso, repetido em três seções, e usado para "justificar" o dado empírico | Remover a fórmula, ou reformulá-la como análise do passo espelhado isolado sem os ajustes. Hipótese provável para o desvio: o ajuste adjacente carrega o maior valor para a direita (e o menor para a esquerda) antes da comparação espelhada seguinte. Isso viola a independência uniforme assumida e eleva P(A[i] > A[j]) acima de 1/2. Rotule como hipótese, ou verifique por simulação |

---

# 3. PROBLEMAS DE ALTA PRIORIDADE

| Página/Seção | Problema | Evidência | Impacto | Correção necessária |
|---|---|---|---|---|
| §4.2 e §5 | "O método jamais supera o ordenador padrão" é contradito pelos seus dados | Zigue-zague, n = 10⁴: Insertion+pré = 13,60 µs e Bubble+pré = 16,00 µs (Tab. 10). `sort_unstable` puro = 70,63 µs e `sort` puro = 93,45 µs (Tab. 5) | Conclusão principal falsa como escrita | Qualificar: vale exceto em Zigue-zague, topologia construída para casar com a varredura. Dizer em qual n |
| §4.2 | "A detecção de run captura sozinha todo o ganho das entradas alinhadas" vale só para Invertido | Zigue-zague: `desc_reverse` puro = 5.299 µs contra 13,42 µs com pré (Tab. 5) | Afirmação exagerada, repetida nas Considerações Finais | Restringir a afirmação ao Invertido |
| Resumo, Abstract, §5 (Quicksort) | A degradação do Quicksort é generalizada, mas a implementação-base é patológica em várias topologias mesmo sem pré-processamento | Quicksort puro em n = 10⁶: Tubo 4.070 ms, Zigue-zague 315 ms, Invertido 268 ms, contra 53 ms em Aleatório (Tab. 12). O próprio texto adia a validação contra pdqsort para trabalhos futuros | Os resultados de Quicksort refletem a implementação e não a técnica. O abstract diz apenas "degradando o Quicksort" | Qualificar no abstract ("na implementação avaliada"). Discutir a patologia do baseline (ela é esperada para o particionamento de Dijkstra com mediana de três em entradas estruturadas) |
| §4.3 (diagnósticos) | Evidências citadas mas não mostradas ou descritas | Contagens de comparações/(n ln n) (11,4→70,6), "entrada já ordenada" (que não é uma das 10 topologias), profundidade de pilha, razões 465× e 749×, assembly. A nota 1 fala em "diagnóstico F2" | Impossível verificar. As razões 465× e 749× não saem das Tabelas 9 e 12 (por exemplo, 368,64 / 0,76 ≈ 485 e 267,65 / 0,40 ≈ 669). Reproducibilidade comprometida | Mostrar uma tabela de diagnóstico e descrever o harness instrumentado na Metodologia, ou remover as afirmações |
| §3.5 contra §2.4, §4.2, §4.3 | Contradição sobre predição de desvios | §3.5 diz que não há contadores e que isso são "hipóteses". Porém §4.2 diz que entradas aleatórias "paralisam o pipeline". §4.3 diz que a hipótese de erro de predição "mostra-se imprecisa" e que o layout "confirma" custos assimétricos (0,36 contra 0,40 ns, valores não mostrados). §2.4 cita Edelkamp como fundamento para a degradação "observada" por falha de predição | Mecanismo sustentado por inferência sem medição. A conclusão de que não é predição é tão frágil quanto a de que é | Tratar tudo como hipótese. Sem contadores de hardware, não se exclui nem se confirma predição |
| §3 (protocolo) | Protocolo de medição insuficiente para reprodução | Não está dito: como cada iteração recebe uma cópia nova do vetor (e se a cópia está dentro da região cronometrada), uso de `black_box`, como se obtém tempo "por vetor" a partir do Criterion (que mede amostras com várias iterações), como a "ordem dos braços alternada" é implementada, a lista exata de tamanhos ("16, ..., 10.000.000"), as sementes dos pools 43–46, e que a mistura "42 ⊕ n ⊕ tipo" pode colidir. Em n = 10⁴ (sort ≈ 15–80 µs), o custo de cópia importa | Outro pesquisador não reproduz o desenho pareado | Detalhar o harness em um parágrafo e, se possível, um trecho de pseudocódigo |
| Tab. 2 e §3.1 ("Real") | O conjunto "Real" não é identificado | "Prefixo de n bytes de asset congelado (N = 477.104)". Não se diz que arquivo é. Valores de byte (0–255) implicam muitos duplicados. O Quicksort Real roda em 31,78 µs contra 360 µs em Aleatório | Não é um "cenário real" representativo. Não é reproduzível sem o asset | Identificar a origem e a licença, descrever a distribuição de valores e relativizar o rótulo |
| Resumo, Abstract, §1 contra Resultados | Escopo anunciado maior que a evidência exibida | "Vetores de até 10⁷" no abstract, mas as reduções de inversões estão só em n = 10⁴ (Tab. 3). Os speedups de ×2.280 e ×386 são só em n = 10⁴. Os pontos em 10⁷ do texto da Fig. 4 não aparecem em tabela, e o eixo da Fig. 4 termina em 10⁶. O abstract não diz que os quadráticos ficam em ≤ 10⁵. O resumo não define "alinhadas" e "desalinhadas" | Leitor infere generalidade que o artigo não sustenta | Indicar n junto aos números do abstract. Mostrar os dados de 10⁷ ou retirar o texto sobre eles. Definir os termos |
| §3.1 e Tab. 2 | Topologias projetadas pelos autores e pseudo-réplicas | Invertido, Zigue-zague e Tubo foram desenhadas em torno da simetria da varredura (Tartarugas e Serra são sintéticas). Nas topologias determinísticas, as "50 réplicas" são cópias idênticas e os "5 pools independentes" também. O "n = 50" inferencial vale só para as estocásticas | Viés de seleção, e as inferências por bootstrap para células determinísticas só medem ruído de tempo | Dizer isso explicitamente. Acrescentar topologias padrão da literatura (ex.: k-ordenado, perturbações locais, benchmarks de pdqsort/ipnsort) |
| §2.2, §3.4, §5 | Bubble Sort tratado como adaptativo em I e a regra ΔI ≳ 5n estendida a "métodos adaptativos" | O próprio §4.3 mostra que o tempo do Bubble não acompanha I (+14,9% com −45% de inversões). A constante c = 0,21 ns foi medida só no Insertion | Regra acionável supergeneralizada | Restringir a regra ao Insertion Sort, e dizer que o Bubble é adaptativo em outra medida (deslocamento máximo) |
| §2.4 (trabalhos relacionados) | Atribuições a verificar e lacunas de posicionamento | (a) O resultado "E[ΔI] = n²/12" é encaixado no parágrafo de Hwang et al., mas esse artigo não analisa o seu algoritmo. (b) "Insertion Sort ótimo para três medidas" (Mannila): conferir. (c) Auger et al.: "estabilidade" não parece ser contribuição deles. (d) Edelkamp e Weiß: "máscaras de bits" parece impreciso (BlockQuicksort usa buffers de índices). Nenhuma comparação com outras passadas O(n) (uma passada de bolha, uma passada tipo Shell) nem ablação (só espelhado contra só adjacente) | A novidade e a contribuição ficam sem suporte | Reler as quatro fontes e corrigir. Incluir ao menos uma ablação e uma passada O(n) alternativa |
| §1, §3.3, Tab. 11–12 | Resíduos de histórico de edição | "regra conservadora anterior", "novas topologias", "diagnóstico F2", "A comparação é estendida..." (frase solta na Introdução) | O leitor não conhece versões anteriores. Parece rascunho | Reescrever em tom neutro |

---

# 4. PROBLEMAS MÉDIOS

| Página/Seção | Problema | Impacto | Correção recomendada |
|---|---|---|---|
| §3.4 e §3.2.3 | O pré-processamento perde estabilidade. Trocas espelhadas ultrapassam chaves iguais, e o `sort` estável é usado como referência | Limita a aplicabilidade a registros com chave e carga | Incluir como limitação |
| Tab. 7 contra Tab. 8 | Tab. 7 diz "média sobre os dez tipos" em n = 10⁶, mas Real não existe nesse n. A média da Tab. 8 (nove tipos, ≈ 1.075 µs) difere da Tab. 7 (946 µs) | Inconsistência numérica | Corrigir a descrição e explicar a diferença CLI contra Criterion |
| §3.3 e tabelas | Com ICs estreitos, marcam-se como significativos efeitos de 0,0% (Merge/Real, Quicksort/Tubo em 10⁶) | Significância estatística sem relevância prática | Adotar um limiar prático (por exemplo, ±2%) além do IC |
| §3.3 | Centenas de células testadas sem comentário sobre comparações múltiplas | Risco de falsos positivos nas células de efeito pequeno | Notar que a estabilidade em 5 pools mitiga, mas não elimina, o problema |
| Tab. 10–12 | Estabilidade entre pools (†) não é exibida | Leitor não verifica | Tabela suplementar no repositório |
| §4.3 (Merge) | Ganhos do Merge Sort em Zigue-zague e Invertido ficam sem mecanismo | Mecanismo mais provável é previsibilidade de desvios na intercalação, e não "inversões" (hipótese a testar) | Discutir como hipótese |
| §1 | Introdução promete "identificar rigorosamente as condições teóricas" | Só há regra empírica, ajustada no Insertion | Moderar a promessa |
| §1 | A contribuição e a lacuna não estão explícitas, e a Introdução afirma que algoritmos quadráticos têm "baixa capacidade de adaptação", o que conflita com §2.1 | Progressão contexto → lacuna fica confusa | Reescrever o 3º parágrafo e listar contribuições |
| §3 (abertura) | Classificações da pesquisa (Gil, Sampieri) citam "testar hipóteses" e "explicar por quê", mas não há hipóteses formuladas e os mecanismos são hipóteses | Incoerência de enquadramento | Formular H1–H3 ou reduzir o enquadramento |
| §3.2.3 | `sort_unstable` descrito como "mais rápido disponível" e "estado da arte", sem fonte | Não considera, por exemplo, radix sort para i32. Além disso, o `sort` da biblioteca padrão já detecta runs, o que explica boa parte do resultado | Citar o algoritmo da biblioteca (ipnsort/driftsort) e relativizar |
| §3.4 e §4.2 | Referências cruzadas erradas | Nota 1 aponta §3.4 (o certo é §3.5). "c ≈ 0,21 ns medidos, Seção 4.2" (o certo é §4.3) | Corrigir |
| Fig. 3 | Barras hachuradas em escala log não permitem ler efeitos de ±5–15%, a legenda diz "Iniciais" (deveria ser "Puro") e não há barras de erro | Figura pouco informativa para o ponto central | Plotar ganho (%) ou razão |
| Fig. 4 e texto (p. 11) | Texto: StdUnstable/Aleatório chega a −77,5% em n = 1.000. Na figura, o mínimo parece estar em n ≈ 10² (verificar) | Possível discrepância texto/figura | Conferir |
| §4.1 | "Redução absoluta Θ(n²)" para todas as não alinhadas, mas só n = 10⁴ foi reportado | Afirmação assintótica sem série de n | Reportar uma série de n ou moderar |
| §2.2 | Seleção: afirma que os algoritmos "selecionados cobrem adaptabilidade" sem tratar a motivação de recursos restritos | A motivação (memória, sistemas embarcados) nunca é testada | Remover da motivação ou medir |

---

# 5. PROBLEMAS BAIXOS E COSMÉTICOS

**Baixos**
- A ordem de citação de tabelas e figuras não segue a numeração: a Tab. 7 e a Fig. 2 aparecem antes da Tab. 5 e da Tab. 3.
- "Resultados" pressupõe a Seção 3.4, que vem depois do protocolo; o Algoritmo 1 poderia ser apresentado mais cedo, já que a Introdução o chama de "proposta".
- A Introdução chama a Seção 3 de "Metodologia", mas o título é "Métodos e Técnicas".
- A reference de Mannila diz "versão estendida publicada originalmente como resumo no ICALP", o que é confuso. A ordem real é conferência primeiro, periódico depois.
- Palavras-chave genéricas ("Avaliação experimental") e repetição de termos do título.

**Cosméticos**
- Vírgula decimal com espaço, presente no PDF todo: "45, 19%", "+43, 9%", "×2, 280" (no abstract em inglês deveria ser 2,280).
- Vírgulas soltas antes de unidades: "1,ns/elemento", "0, 21,ns/inversão", "16–23,ms", "368, 64,ms".
- Concordância em §2.2: "complexidade temporal quadrático".
- Identificadores de código nas tabelas e figuras: `sort_un`, `desc_rev`, `sort_uns`, `StdUnstable`, `DescReverse`.
- Título do apêndice "A Tabelas completas de tempos de execução" quebrado.
- Metadados do PDF vazios (título e autor).
- Marcadores do template ainda no cabeçalho: DOI "XXXXXX", datas "DD Month YYYY", volume "XX:1" (INCERTO se devem ficar).

---

# 6. CONFORMIDADE COM A REIC 2026

Fonte principal: https://journals-sol.sbc.org.br/index.php/reic/about/submissions

| Regra/Requisito | Status | Evidência no artigo | Ação |
|---|---|---|---|
| Área de Computação | OK | Algoritmos de ordenação | Nenhuma |
| Idioma pt, en ou es | OK | Português, com abstract em inglês (conforme template) | Nenhuma |
| Arquivo em PDF | OK | A4, fontes embutidas | Nenhuma |
| 6–12 páginas incluindo referências | **PROBLEMA** | 14 páginas | Cortar ≥ 2 páginas |
| Resumo ≤ 150 palavras ou 10 linhas | OK | 98 palavras (PT), 96 (EN) | Nenhuma |
| Modelo da REIC ("novo formato") | INCERTO | O cabeçalho e as fontes parecem estilo SBC, mas não consegui abrir o modelo do Overleaf | Comparar com o modelo |
| Autores no PDF e nos metadados, mesma ordem | INCERTO | Ordem no PDF: Nascimento, Rocha. Metadados não verificáveis | Conferir no sistema |
| Estudantes de graduação como autores principais, com orientador na lista | INCERTO | O PDF não diz quem é graduando. Rocha aparece como orientador | Declarar em "Comentários ao Editor" |
| "Comentários ao Editor" com tipo de trabalho, data, titulações e instituições | INCERTO | Não consta no PDF | Preencher no envio |
| Revisão simples cega | OK | Autores identificados | Nenhuma |
| DOIs nas referências quando disponíveis | OK, com ressalva | DOIs presentes nos artigos de periódico e conferência. Livros sem DOI | Verificar se a 4ª edição do Cormen tem DOI |
| Fontes em LaTeX se aceito | OK | O PDF foi gerado com pdfTeX | Nenhuma |
| Inédito e não em avaliação por outro periódico | INCERTO | Se este manuscrito também for ao Bench 2026, justifique em "Comentários ao Editor". Não consegui confirmar se a regra se aplica a conferências | Declarar no envio |
| Código de Conduta SBC | INCERTO | Nenhum indício de problema visível | Revisar o código |
| Ética | OK | Declara que não há sujeitos humanos | Nenhuma |
| Licença | OK, com conflito entre páginas | O PDF e a página de Submissões dizem CC BY 4.0. A página "Sobre" diz CC BY-NC 4.0 | Seguir o modelo (CC BY 4.0) |
| Regras específicas da edição 2026 | INCERTO | Nenhuma encontrada | Conferir o edital, se for pelo CTIC |

---

# 7. AUDITORIA CIENTÍFICA

**Problema e objetivos**
- *Pontos fortes:* pergunta clara (Cpre + Csort < Coriginal) e formalização simples.
- *Problemas:* a lacuna não é explicitada. A contribuição é difusa entre "propor um algoritmo" e "avaliar condições de uso". Falta hipótese formal.

**Fundamentação e trabalhos relacionados**
- *Pontos fortes:* definições de inversão e adaptatividade corretas. Boa seleção de textos clássicos.
- *Problemas:* ver itens de atribuição e de ausências. Referências aparentemente ausentes (verificar bibliografia antes de incluir): Hoare (Quicksort), Bentley e McIlroy (1993, função de ordenação), Musser (introsort, 1997), Peters (pdqsort, 2021), Petersson e Moffat (1995, ordenação adaptativa), documentação do ipnsort/driftsort.

**Metodologia e benchmark**
- *Pontos fortes:* pareamento de vetores, verificação de corretude fora da região cronometrada, bootstrap pareado com estabilidade entre pools, ambiente descrito, reprodutibilidade via repositório.
- *Problemas:* ver os itens ALTOS de protocolo, "Real", pseudo-réplicas, topologias construídas, Quicksort patológico e baselines limitados.

**Resultados e estatística**
- *Pontos fortes:* as tabelas de faixa (4, 6, 9) são consistentes com as de células. Conferi os resumos de ganho e as razões 36× e 8,1×. Os valores de Cpre e do breakeven por topologia batem. A análise por topologia é honesta ao admitir não robustez em várias células.
- *Problemas:* análise extra recomendável (não obrigatória) para o limiar prático de efeito. Testes adicionais de significância seriam desnecessários para os efeitos grandes (≥ 14%).

**Discussão, ameaças e conclusão**
- *Pontos fortes:* as ameaças em §3.5 são abrangentes (hardware, SO, medição). A conclusão delimita escopo e reconhece trabalho futuro.
- *Problemas:* §3.5 é um parágrafo denso, e faltam ameaças sobre a topologia construída, o Quicksort e o "Real". A conclusão tem afirmações mais fortes do que a evidência ("jamais", "confirmou").

---

# 8. AUDITORIA DO PDF

- **Págs. 1–14:** vírgula decimal espaçada e vírgulas soltas antes de unidades, principalmente nas págs. 6–11.
- **Pág. 1:** DOI e datas com marcador do template.
- **Pág. 11:** Fig. 3 com barras hachuradas e minúsculas, legenda "Iniciais". Fig. 4 com eixo até 10⁶ (texto e legenda citam até 10⁷) e rótulos de código ("StdUnstable").
- **Págs. 12–14:** o apêndice aparece antes das referências e as tabelas 10–12 caem depois delas, de modo que a lista de referências é interrompida por tabelas e termina na pág. 14.
- **Verificação técnica:** fontes embutidas, sem imagens rasterizadas, A4.

---

# 9. INCONSISTÊNCIAS INTERNAS

- E[ΔI] teórico (≈ 33% de redução) contra medido (45%), com explicação que não fecha (§2.4/§3.4).
- "Jamais supera o padrão" contra Zigue-zague (Tab. 5 e 10).
- Run detection "captura todo o ganho" contra Zigue-zague.
- §2.4 e §4.3 sobre predição de desvios, e §3.5 que a chama de hipótese.
- Tab. 7 (dez tipos em 10⁶) contra a inexistência de Real em 10⁶.
- Razões 465× e 749× contra as Tabelas 9 e 12.
- Eixo da Fig. 4 (até 10⁶) contra texto (até 10⁷).
- Cross-refs §3.4 e §4.2 incorretas.
- "Seção 3: Metodologia" contra "Métodos e Técnicas".

---

# 10. PERGUNTAS QUE UM REVISOR PODERIA FAZER

| Pergunta | O artigo responde? | Onde |
|---|---|---|
| Por que usar Insertion Sort+pré se `sort_unstable` é ≈ 36× mais rápido? | Parcialmente | §4.2 admite, mas a motivação (recursos restritos) não é testada |
| Uma passada de bolha ou de Shell não faria o mesmo? | Não | Sem ablação |
| O Quicksort é um baseline válido? | Parcialmente | §4.3 e trabalhos futuros |
| A fórmula n²/12 está correta? | Não | §3.4 |
| As topologias favorecem a técnica? | Parcialmente | Reconhece "alinhadas", mas não o viés de seleção |
| O que é o dataset "Real"? | Não | Tab. 2 |
| A cópia do vetor entra no tempo? | Não | §3.3 |
| O resultado vale para outros n, máquinas, SO? | Parcialmente | §3.5 e trabalhos futuros |
| O que ocorre com chaves com carga útil (estabilidade)? | Não | Não discutido |

---

# 11. CHECKLIST OBRIGATÓRIO ANTES DA SUBMISSÃO

1. Reduzir para ≤ 12 páginas, incluindo referências (por exemplo, mover as Tabelas 10–12 para o repositório e condensar §4.3 e Considerações Finais).
2. Corrigir ou remover a derivação n²/12 e a explicação dos 45,19% em §2.4, §3.4 e §5.
3. Corrigir "jamais supera" e "run detection captura todo o ganho" para refletir Zigue-zague.
4. Ajustar abstract, introdução e conclusão ao escopo exibido (n, implementação de Quicksort, topologias).
5. Mostrar ou remover os dados de 10⁷ e os diagnósticos de §4.3 (comparações/(n ln n), profundidade, razões 465×/749×, assembly).
6. Descrever o protocolo de medição (reset do vetor, tempo por vetor, `black_box`, sizes, seeds, ordem dos braços) e identificar o asset "Real".
7. Reconciliar as afirmações sobre predição de desvios com §3.5 (hipótese).
8. Remover resíduos de histórico ("novas topologias", "regra anterior", "diagnóstico F2").
9. Corrigir a Tab. 7/Tab. 8 e as referências cruzadas (§3.5 e §4.3).
10. Verificar as atribuições a Hwang et al., Mannila, Auger et al. e Edelkamp e Weiß.
11. Preencher "Comentários ao Editor" e conferir ordem dos autores nos metadados.

---

# 12. MELHORIAS OPCIONAIS

- Ablação (só espelhado, só adjacente) e uma passada O(n) alternativa como baseline.
- Comparar Quicksort com pdqsort e incluir Timsort.
- Topologias padrão da literatura e um dataset real identificado.
- Plotar ganho (%) em vez de tempos em escala log (Fig. 3).
- Limiar prático de efeito (±2%).
- Segunda máquina ou Linux com perf.
- Arquivar o repositório com commit fixo (por exemplo, Zenodo).
- Corrigir a formatação decimal e os rótulos de código.

---

# 13. DIAGNÓSTICO FINAL

1. **Problema crítico?** Sim: as 14 páginas e a derivação n²/12 incorreta.
2. **Problema metodológico sério?** Sim: Quicksort patológico como baseline, topologias construídas e pseudo-réplicas.
3. **Problema experimental?** Sim, nos mesmos pontos acima, mais baselines limitados e ausência de ablação.
4. **Problema de reprodutibilidade?** Sim: protocolo de medição, "Real", sizes e seeds.
5. **Problema de conformidade com a REIC 2026?** Sim: limite de páginas. Outros itens estão INCERTOS (template, metadados, Comentários ao Editor, regras específicas da edição).
6. **Problema estrutural?** Sim, moderado: Algoritmo 1 tardio, apêndice antes das referências, resíduos de edição.
7. **Problema importante nas referências?** Sim: atribuições a verificar e ausências. As 13 referências estão todas citadas, e todas as citações têm referência.
8. **Problema visual no PDF?** Sim, moderado: Fig. 3, Fig. 4, vírgulas decimais, tabelas depois das referências.
9. **Correções obrigatórias:** os 11 itens da seção 11.