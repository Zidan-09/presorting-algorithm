Vou começar consultando as regras oficiais do Bench 2026 e o que já está registrado sobre o seu artigo.# 1. DIAGNÓSTICO EXECUTIVO

**NÃO PRONTO PARA SUBMISSÃO**

**Limites desta auditoria**
- **Regras oficiais do Bench 2026.** Só consegui confirmar o modelo geral. O Bench 2026 adota submissão contínua, com prazo de 1º de novembro de 2026 (AoE) para aceitação e inclusão no evento, e o site de submissão é o do PEvaluation. A revisão é double-blind, e os artigos aceitos são publicados no PEvaluation. Artigos de destaque serão recomendados ao TBench.
- **O que não consegui abrir.** Não obtive a página oficial de submissão do PEvaluation: a URL não abriu e `/PEvaluation/index` retornou 404. A página do evento no BenchCouncil Press mostra "No event description available yet" e tem um item "Call for Contributions" que não consegui abrir. Não encontrei template, limite de páginas, lista de tópicos nem regras de anonimização específicos de 2026.
- **Conflito entre fontes.** O snapshot da home do BenchCouncil que carreguei termina a seção "What's New" em 29/06/2026 e não traz o item de 18/08/2026 sobre o CFP. Esse item aparece apenas nos trechos de busca da mesma home. Trato o modelo de submissão contínua como informação oficial mas não totalmente verificada.
- **Regras antigas.** As regras de 2022 a 2025 (LNCS, 15 páginas, pré-registro) aparecem só como contexto, nunca como exigência de 2026.
- **Análise visual.** Li o texto extraído de todas as 9 páginas e vi a renderização da página 1. As figuras 1 a 3 e as cores não foram inspecionadas em pixels. O que depende disso está marcado como INCERTO.

**Principais motivos**
1. Os números de destaque do abstract (até 99,9% e 100%) vêm de entradas que coincidem por construção com a geometria do algoritmo (Reversed e Zigzag). Em entradas não alinhadas, o ganho é de 13% a 42% e só ocorre em algoritmos quadráticos.
2. Não há baseline competitivo. O ganho é medido contra o mesmo algoritmo sem pré-processamento. Insertion Sort com pré-processamento em Random (3.181 µs) continua cerca de 8,7× mais lento que Quicksort puro (367 µs).
3. Há evidências, nas suas próprias tabelas, de que a explicação para Bubble Sort está errada, de que o Quicksort tem comportamento anômalo em entradas ordenadas, e de que a regra de significância estatística é violada pelo texto.
4. Faltam elementos de reprodutibilidade: Turtles e Zigzag sem definição formal, repositório indisponível e protocolo de amostragem do pool de vetores não descrito.
5. A conexão com Evaluation Science and Engineering é implícita. O artigo lê-se como engenharia de algoritmo, o que é risco de escopo, mas não consegui confirmar o critério de 2026.

As correções exigem novos experimentos (baselines e varredura de tamanhos), mas são factíveis com o harness Rust que você já tem.

---

# 2. PROBLEMAS CRÍTICOS

| Local | Problema | Evidência | Impacto | Correção necessária |
|---|---|---|---|---|
| Abstract; §4.1; Tabelas 3–4; Conclusão (p.1, 5–7) | Os ganhos de destaque vêm de entradas alinhadas com a simetria do algoritmo. | Em Reversed o algoritmo executa uma reversão completa do vetor (o §4.1 admite isso), então "100%" é tautológico. Zigzag tem 24.997.500 inversões → 2.501, mas não tem definição formal (Tabela 2). Todos os ganhos ≥ 99,7% ocorrem só nessas duas topologias. Em Random, Insertion +42,2%, Bubble +13,3%, Selection +0,5%, Merge −3,5% e Quick −4,1%. | Validade de construção. O abstract sugere generalidade que os dados não sustentam. A conclusão de "viabilidade prática" depende de inputs escolhidos. | Dizer no abstract que os ganhos extremos ocorrem em entradas alinhadas. Incluir topologias não alinhadas (Bentley–McIlroy, sawtooth, poucos runs descendentes, organ-pipe, dados reais). Tratar Random e Almost Sorted como resultado principal e definir Zigzag e Turtles por fórmula. |
| Tabela 4; §1; §4.3; Conclusão | O ganho é medido só contra o mesmo algoritmo sem pré-processamento, sem o melhor competidor disponível. | Com n=10⁴ em Random, Insertion+pre = 3.181,76 µs, Bubble+pre = 20.530 µs e Quicksort puro = 367,27 µs, então Insertion+pre fica 8,7× mais lento que Quicksort puro. Em Reversed, Insertion+pre (13,04 µs) bate Merge (129 µs), mas qualquer detecção de run descendente com reversão O(n), como em Timsort, dá o mesmo resultado sem o método. Não há `sort`/`sort_unstable` do Rust como referência. A motivação é "sistemas com restrição de recursos" e "conjuntos pequenos", mas todos os testes usam n ≥ 1.000 num Ryzen de mesa. | A pergunta "Cpre + Csort < Coriginal" é respondida numa comparação que nenhum praticante faria. A relevância prática fica indemonstrada. | Adicionar baselines: `sort_unstable`/`sort` do Rust e "inverter se descendente" (O(n)). Incluir n pequeno (16–512) ou remover a motivação de sistemas restritos/pequenos. Reescrever a conclusão em termos de "menor tempo total entre todas as alternativas". |

---

# 3. PROBLEMAS DE ALTA PRIORIDADE

| Local | Problema | Evidência | Impacto | Correção necessária |
|---|---|---|---|---|
| §2.2; §4.2–4.3; Tabela 4 (p.2, 5–6) | Bubble Sort é tratado como sensível ao total de inversões I, e os dados contradizem isso. | O tempo do Insertion Sort é proporcional a I: ≈0,22 ns/inversão em Random (5.506,86 µs/24.960.120), Reversed (0,218), Zigzag (0,221), Turtles (0,216) e Duplicates (0,220). O tempo do Bubble é quase plano (16–24 ms) em entradas com I de 6,5·10⁵ a 5·10⁷. Almost Sorted: Bubble 16.466 µs contra Insertion 183 µs (90×). Reversed tem o dobro das inversões de Random e é mais rápido no Bubble. O número de passadas do Bubble depende do maior deslocamento à esquerda, não de I. | A explicação "eliminação drástica de inversões" vale para Insertion e não para Bubble. "Justificado matematicamente" (§4.3) não tem matemática apresentada. | Caracterizar Bubble por passadas (deslocamento máximo). Explicar o ganho no Zigzag/Reversed por deslocamento máximo ≈1 após o pré-processamento. Aproveitar a regularidade de 0,22 ns/inversão do Insertion como evidência mecanística. |
| Tabelas 4 e 6; §4.3 | O baseline Quicksort tem comportamento anômalo, e a explicação para Reversed contradiz a teoria. | De n=10⁴ para 10⁶ o tempo cresce 147× em Random (expoente ≈1,08, coerente com n log n). Em Reversed (884×), Zigzag (849×) e Turtles (766×) o expoente empírico é ≈1,45–1,48. Com Reversed pré-processado (vetor ordenado), 591,8 ms contra 55,75 ms em Random. Mediana de três é favorável a entradas ordenadas, então "remove a estrutura explorada pela mediana de três" não se sustenta. O §3.2.2 afirma que a partição de três vias "garante" O(n log n), o que não vale com mediana de três. | Um Quicksort que leva 10× mais tempo em vetor já ordenado levanta suspeita de bug ou patologia de implementação. As conclusões sobre Quicksort (−30% a −42%) ficam comprometidas. | Validar a implementação contra `sort_unstable` e contra um quicksort padrão (pdqsort). Medir contagens de comparações/trocas. Investigar a partição DNF em entradas ordenadas. Corrigir ou declarar a patologia. |
| Tabelas 4–6; §4.2–4.3 | Cpre e Csort não são separados, e boa parte da "degradação" é só o custo do pré-processamento. | Quicksort Duplicates: Δ = +14,08 µs em n=10⁴ contra Cpre médio de 12,44 µs, e +1,31 ms contra 1,18 ms em n=10⁶. Merge Random em n=10⁴: Δ = +15,1 µs, também da ordem de Cpre. A hipótese de que a reorganização "perturba a distribuição de chaves iguais" é desnecessária como explicação. Bubble/Reversed totaliza 10,28 µs, abaixo do Cpre médio de 12,44 µs (Tabela 5), então o custo varia por topologia ou a ferramenta CLI difere do Criterion. | As explicações causais do Quicksort (Duplicates, Reversed) ficam sem apoio. A Tabela 5 esconde heterogeneidade. | Reportar Cpre por topologia com o mesmo harness (Criterion), e Csort pós-pré-processamento separadamente. Reescrever a análise com base em ΔCsort. |
| §3.3; Tabela 4; §4.3; Conclusão | O critério estatístico é mal aplicado, e o texto viola a própria regra. | Selection Turtles: 21.429,88±171,35 → 20.717,84±122,11 tem ICs disjuntos (+3,3%). Selection Zigzag: 21.382,51±180,13 → 20.975,92±90,55 também é disjunto (+1,9%). O texto diz "ICs sobrepostos na maioria" e cita "−1,1% a +3,3%" como variações neutras. O pareamento de vetores é anunciado, mas a análise trata as amostras como independentes. O critério de sobreposição de ICs é conservador e assimétrico. | A "neutralidade" do Selection Sort fica incorreta pela própria regra do artigo. Gera risco de crítica por incoerência estatística. | Usar IC bootstrap pareado da razão (ou log-razão) dos tempos por vetor, reportando efeito com IC. Marcar significância na Tabela 4. Corrigir os textos sobre Selection. |
| §3.1; §4; Abstract | Só dois tamanhos aparecem nos resultados. | Tabela 4 (n=10⁴, todos os algoritmos) e Tabela 6 (n=10⁶, só quasilineares). Os tamanhos 1.000, 5.000 e 100.000 foram coletados, mas não aparecem, exceto dois números do Selection em n=10⁵. A frase "as ordens de grandeza do ganho já se estabilizam em n=100.000" não tem dados apresentados. O abstract diz "até 10⁶" mas os algoritmos adaptativos ficam limitados a 10⁵. | Afirmação central sem evidência visível. Não se vê a tendência com n (Quicksort Almost Sorted muda de −3,0% para +11,1%). | Incluir figura de ganho (razão) contra n por algoritmo e topologia. Esclarecer no abstract o alcance por classe de algoritmo. |
| §3.1–3.3; Tabela 2; [14] | Lacunas de reprodutibilidade relevantes. | Zigzag ("element-wise alternation") e Turtles ("elevated increasing upper half; low cyclic lower half") não têm fórmula. Não está claro como o pool de 50 vetores se mapeia às 50 amostras do Criterion, nem se a cópia do vetor é cronometrada. Faltam frequência/turbo, SMT, afinidade de CPU, flags (target-cpu, LTO, codegen-units) e verificação de bounds-check. O §3.5 cita "execution-order alternation" sem descrever. [14] diz "available upon acceptance" e não traz link acessível. | Um pesquisador não reproduziria os números. Num venue de avaliação, é falha de conteúdo. A missão do BenchCouncil inclui benchmarks padronizados que garantam resultados reproduzíveis. O TBench prioriza reprodutibilidade e incentiva artefatos abertos. | Dar fórmulas ou pseudocódigo para as seis topologias. Descrever o protocolo de pool e de setup (`iter_batched`). Fornecer repositório anonimizado acessível aos revisores, com `Cargo.lock`, toolchain fixo, scripts e dados brutos. |
| §1; §2.4; §3 | A promessa teórica não é cumprida, e a estimativa em §2.4 é inconsistente. | O §1 promete "rigorosamente identificar as condições teóricas e práticas" e "modelagem formal", mas só há pseudocódigo. O §2.4 diz que a derivação sugere "ganho linear" e que 45% é "compatível com a ordem de grandeza prevista", enquanto 45% de Θ(n²) é uma redução Θ(n²), não linear. | Promessa não cumprida e argumento teórico frouxo. | Provar o lema: trocar um par invertido nunca aumenta I (reduz em 1 + 2·#{k entre: a_j<a_k<a_i}, mais 1 por cada chave igual intermediária). Uma estimativa de primeira ordem dá ≈ n²/12 (≈1/3 de n²/4), que você pode comparar com os 45% medidos; ignora a interação entre trocas e deve ser verificada. Derivar o ponto de equilíbrio ΔI ≳ Cpre/(0,22 ns) ≈ 5–6·n (ordem de grandeza, a partir das suas tabelas). |
| §2.4; §5; Referências | Novidade e posicionamento não estão demonstrados. | O passo espelhado lembra uma compare-exchange de Shell (Hwang et al. analisam Shellsort como presorting), mas a comparação não é feita. Não há menção a pdqsort, driftsort (Rust std), ips4o, Powersort nem Bentley–McIlroy (padrões de teste). "Filling a low-cost optimization gap" não tem busca sistemática de literatura. A conclusão cita "Powersort/Timsort [11]", mas [11] (Auger et al.) trata apenas de Timsort. | Risco de crítica de originalidade. Citação incorreta. | Comparar com passada tipo Shell e detecção de runs. Citar Munro & Wild para Powersort. Adicionar os trabalhos ausentes e justificar ou moderar a afirmação de lacuna. |
| Escopo (§1; §3; Keywords) | A conexão explícita com Evaluation Science and Engineering é fraca. | O artigo não discute condições de avaliação equivalentes, rastreabilidade, confundidores nem metodologia de avaliação como contribuição. As "Keywords" citam Benchmarking/Performance evaluation/Experimental methodology, mas o corpo é otimização de algoritmo. O Bench 2026 foca em aplicar princípios e metodologias unificadas de avaliação e avançar práticas de engenharia de avaliação. A lista de tópicos de 2026 não foi localizada (INCERTO). | Risco de ser lido como artigo genérico de algoritmos num evento de avaliação. | Reenquadrar como estudo de avaliação: como medir o custo-benefício de pré-passadas, que confundidores (custo de cópia, comportamento de predição de desvios, escolha de topologias) enviesam a conclusão e como o desenho pareado os controla. |

---

# 4. PROBLEMAS MÉDIOS

| Local | Problema | Impacto | Correção recomendada |
|---|---|---|---|
| §4.3 (Selection Sort) | O Selection Reversed "pure" é o outlier rápido (18,26 ms), enquanto os outros 11 casos ficam em 20,5–21,5 ms. A narrativa de "degradação pelo pré-processamento" inverte isso. Os padrões de desvio de ambos os casos (sempre verdadeiro e sempre falso) são previsíveis, o que enfraquece a explicação de preditor de desvios. O §5 escreve "attributed to branch-prediction effects", enquanto o §4.3 diz "plausible hypothesis". | Hipótese promovida a atribuição na conclusão. | Verificar o assembly (cmov vs. jump) e contar atualizações de mínimo/trocas por instrumentação. Se o AMD uProf estiver disponível no seu Ryzen sob Windows, usar contadores de hardware (confirme). Rebaixar o texto da conclusão. |
| §4.2 | "Sem gargalos de localidade de memória em escala" não tem suporte. n=10⁶ de i32 ocupa 4 MB, cache-resident num 8700G. | Afirmação de escala sem teste fora da cache. | Remover, ou testar n≥10⁷–10⁸. Informar tamanhos de L2/L3. |
| §3.5 | A seção de ameaças à validade tem quatro frases. Faltam: máquina/OS/compilador únicos, só i32, dados sintéticos, uma implementação por algoritmo, uma semente mestre, extrapolação dos quadráticos (n ≤ 10⁵), k=3 em Duplicates, timer do Windows. | O leitor não sabe os limites de generalização. | Ampliar e estruturar a seção (interna, externa, de construção, estatística). |
| Tabela 4; Abstract | O ganho percentual satura (99,7%, 99,9%, 100,0% escondem fatores de 300× a 2.000×) e é assimétrico (−36,8% vs. +99,9%). "Gain" não é definido. Um ganho de "100,0%" é impossível. | Métrica que distorce a leitura. | Definir "gain". Reportar speedup (T_pure/T_pre) em escala log, com IC. |
| §3.3 | "Bootstrapping mitiga a interrupção de hardware" é tecnicamente incorreto (o bootstrap estima IC). "Fractions of a second" contradiz o Selection com 1,85–2,09 s em n=10⁵. A janela de 5 s não comporta 30 amostras de ≈2 s, então o Criterion estende o tempo, mas os parâmetros reportados parecem configurados. Só médias são reportadas. | Descrição imprecisa do método. | Corrigir a redação e reportar os parâmetros efetivos e o número real de iterações. Reportar mediana e outliers. |
| §3.1 | Uma só semente (42) e combinação por XOR (42 ⊕ n ⊕ type). | Os estocásticos dependem de um único conjunto de 50 vetores. | Usar várias sementes independentes (por ex., 5 pools) e justificar a derivação. |
| §3 (abertura) | Os quatro parágrafos de classificação de pesquisa (aplicada, quantitativa, explicativa, experimental, com Gil e Sampieri) consomem espaço sem valor para o venue. "Test hypotheses" e "explanatory" não se sustentam, pois não há hipóteses declaradas e as explicações ficam como hipóteses. | Diluição e promessas de método não cumpridas. | Se esses quatro parágrafos forem exigência da sua instituição, mantenha-os curtos. Para o Bench, troque-os por declaração de hipóteses e desenho experimental. |
| Algoritmo 1; §3.4 | A condição `j − 1 > mid` exclui o par (mid, mid+1) quando n é par, enquanto a esquerda trata o par (mid−2, mid−1). O algoritmo não é exatamente simétrico nesse caso. Verificar se é intencional. | Possível assimetria não documentada. | Documentar ou corrigir. |
| Referências; §2.4 | [13] está como "J. Exp. Algorithmics, vol. 11, 2006" com DOI ACM. Os registros que encontrei indicam que o artigo de Kaligosi e Sanders é do ESA 2006, LNCS vol. 4168. Em páginas 780–791. [6] diz "extended version originally published as abstract at ICALP 1984", invertendo conferência e periódico. Os comentários bibliográficos dentro de §2.4 pertencem à lista de referências. Knuth [1] cita a edição de 1973. | Erros bibliográficos e ruído no texto. | Corrigir [13] e [6], revisar DOIs e edições. Não verifiquei os demais DOIs nesta sessão. |

---

# 5. CONFORMIDADE COM O BENCH 2026

| Requisito oficial | Status | Evidência no artigo | Ação |
|---|---|---|---|
| Modelo de submissão contínua; prazo de 1º nov. 2026 (AoE) para aceitação e inclusão (home BenchCouncil; trechos de busca) | INCERTO | n/a. O snapshot da home não mostrava o item de 18/08/2026, e a página do PEvaluation não abriu. | Abrir o site de submissão logado. A data refere-se à **aceitação** para inclusão, não à submissão, então submeter cedo reduz o risco de prazo. |
| Publicação no PEvaluation (home BenchCouncil) | INCERTO | O cabeçalho do PDF traz "BenchCouncil Transactions on Benchmarks, Standards and Evaluations, 2026", "DOI HERE", "Received on Date Month Year". | Confirmar se o template do PEvaluation é o mesmo do TBench. |
| Double-blind (home BenchCouncil) | OK | Página 1 sem autores, afiliações, funding ou acknowledgments. Rodapé "© The Author 2026". | Manter. |
| Anonimização de repositório [14] | INCERTO | "Anonymous repository… available upon acceptance", sem link. | Fornecer repositório anonimizado acessível, se as regras permitirem. |
| Metadados do PDF e nome do arquivo | INCERTO | Nome `main.pdf` é neutro. Metadados não verificáveis. | Limpar autor/título no XMP e nas propriedades antes de enviar. |
| Tipo de submissão e limite de páginas (2026) | INCERTO | Corpo ≈ 7 páginas mais a Fig. 3 e referências (p.8–9). | Não aplicar os limites de 2025 (LNCS). O TBench limita artigos de pesquisa a 12 páginas de duas colunas, sem contar referências. Isso serve só como referência, não como regra do PEvaluation. |
| Escopo e tópicos 2026 | INCERTO | Ver ALTO (escopo). | Verificar a lista de tópicos na página oficial. |
| Idioma | INCERTO | PDF em inglês. | Confirmar na página oficial. |
| Numeração de páginas e legibilidade em preto e branco | INCERTO | Páginas numeradas. Figuras não inspecionadas em P&B. | Regras de 2024/2025 exigiam; não aplicar sem confirmar para 2026. Testar impressão em P&B de qualquer forma. |
| Pré-registro e apresentação por ao menos um autor | INCERTO | n/a. | Os artigos aceitos serão apresentados na conferência. Confirmar registro exigido. |
| Recomendação ao TBench / número especial | INCERTO | n/a. | Não encontrei página de special issue de 2026. O TBench exige que ao menos um autor se registre e apresente no Bench. |
| Originalidade / não estar em revisão paralela | INCERTO | n/a. | Confirmar a política de 2026. |
| Campos de template ("DOI HERE", "Received on Date Month Year") | INCERTO | Placeholders na p.1. | Seguir a instrução do template. |

---

# 6. AUDITORIA CIENTÍFICA

**Contribuição**
**Pontos fortes:** o algoritmo é simples e bem definido (Algoritmo 1). Há análise de custo O(n) e O(1) em espaço. O relato inclui resultados negativos.
**Problemas:** a contribuição é uma heurística de pré-passada com evidência empírica limitada a duas topologias alinhadas e algoritmos quadráticos. A novidade não é posicionada contra passadas de Shell, detecção de runs ou sorters modernos (ALTO).

**Originalidade**
**Pontos fortes:** o uso de Hwang et al. como referência de presorting é pertinente.
**Problemas:** "low-cost optimization gap" não é demonstrado. A literatura citada é fundamental e antiga; faltam pdqsort, driftsort, Powersort e Bentley–McIlroy.

**Problema e pergunta de pesquisa**
**Pontos fortes:** a pergunta é clara e testável: em que medida Cpre + Csort < Coriginal.
**Problemas:** a motivação (sistemas restritos, conjuntos pequenos) não tem relação com o desenho experimental (n ≥ 10³, desktop). O artigo promete condições teóricas e entrega tabelas empíricas (X → Y → Z).

**Metodologia**
**Pontos fortes:** vetores pareados entre algoritmos e variantes. Correção verificada fora da região cronometrada (ordenação comparada com `sort_unstable`). Custo total incluindo o pré-processamento. Contagem exata de inversões em O(n log n).
**Problemas:** Zigzag e Turtles sem definição. Protocolo de pool/setup, flags de compilação e controle de frequência ausentes. Medição de Cpre com outra ferramenta (CLI). Duas topologias são alinhadas ao método por construção.

**Benchmark**
**Pontos fortes:** seis topologias com níveis variados de desordem. Medição via Criterion. Configuração do Criterion por perfil de tamanho.
**Problemas:** sem baselines competitivos, sem n pequeno, sem dados reais, um hardware, n=10⁶ cache-resident. O baseline Quicksort tem anomalias de escala.

**Estatística**
**Pontos fortes:** ICs de 95% e regra explícita de relevância.
**Problemas:** análise não pareada apesar do desenho pareado, regra de sobreposição conservadora, violação pelo Selection (Turtles, Zigzag), ausência de marcação de significância, somente médias.

**Resultados**
**Pontos fortes:** os percentuais recalculados a partir das tabelas conferem (por ex., Insertion Random +42,2%, Quick Reversed −36,8%, Merge Zigzag +14,8%, Quick Reversed 10⁶ −42,1%).
**Problemas:** a separação entre observação, interpretação e hipótese é feita em notas de rodapé, mas a conclusão volta a atribuir causa ("attributed to branch-prediction effects"). Ver inconsistências na seção 8.

**Trabalhos relacionados**
**Pontos fortes:** boa síntese de Mannila, Estivill-Castro & Wood e Hwang et al.; reconhece efeitos microarquiteturais (Edelkamp & Weiß, LaMarca & Ladner).
**Problemas:** ver Originalidade. Erros em [6], [11] (uso) e [13].

**Reprodutibilidade**
**Pontos fortes:** hardware e versões de Rust e Criterion informados. Semente fixa. Uso de `cargo bench`.
**Problemas:** ver ALTO (reprodutibilidade). Repositório indisponível.

**Ameaças à validade**
**Pontos fortes:** reconhece a ausência de contadores de hardware e o ambiente Windows.
**Problemas:** cobertura mínima (ver MÉDIO).

**Conclusões**
**Pontos fortes:** a conclusão admite a limitação de aplicabilidade a quadráticos adaptativos.
**Problemas:** "viabilidade prática" sem baseline competitivo. Faixas numéricas inconsistentes com a Tabela 6. Atribuição causal não verificada.

---

# 7. AUDITORIA DO PDF

Limite: análise textual de todas as páginas e renderização só da p.1.

- **p.1** (COSMÉTICO/INCERTO): placeholders "DOI HERE" e "Received on Date Month Year; Accepted on Date Month Year".
- **p.3** (MÉDIO): a Tabela 2 descreve Zigzag e Turtles sem fórmula, prejudicando a leitura e a reprodução.
- **p.4, §3.3** (MÉDIO, verificar no PDF): "2,s", "5,s", "1,s", "10,s" parecem unidades quebradas (possivelmente `2\,s` escrito com vírgula).
- **p.4** (COSMÉTICO): "modifiedMergeSort-based" sem espaço. "sort unstable" deveria ser `sort_unstable`.
- **p.5, Fig. 1** (MÉDIO/INCERTO): o resultado de Reversed (pós = 0) não aparece na escala log. Anotar explicitamente "0".
- **p.6, Fig. 2** (INCERTO): na extração, o eixo x mostra só 1.000 e 10.000, embora os dados vão até 10⁶. Verificar os rótulos de ticks.
- **p.6, Tabela 4** (BAIXO): 30 linhas sem marcação de significância, nem negrito.
- **p.6, nota de rodapé 1** (BAIXO): refere-se a "Section 3.6", inexistente.
- **p.8, Fig. 3** (BAIXO): aparece após a Conclusão, longe do texto que a discute (p.7). Em escala log, diferenças de 3–5% ficam invisíveis. Há cinco painéis com faixas y diferentes e sem barras de erro (INCERTO). Legibilidade em P&B: INCERTO.
- **p.8–9** (BAIXO): comentários bibliográficos nas referências [6]. A p.9 fica quase vazia (aceitável).

---

# 8. INCONSISTÊNCIAS INTERNAS

1. **Referências cruzadas (BAIXO):** §4.2 cita "Section 3.4" para o critério estatístico, que está no §3.3. §4.3 e a nota 1 citam "Section 3.6", inexistente (o correto é §3.5).
2. **Abstract vs. Tabela 4 (BAIXO):** abstract diz "up to 99.9%", enquanto a Tabela 4 traz +100,0% (Bubble/Reversed); a conclusão usa "13%–100%".
3. **Abstract vs. corpo (MÉDIO):** o abstract diz "inconsistent results" para O(n log n), mas o corpo descreve "systematic degradation" do Quicksort (11 dos 12 casos negativos nas Tabelas 4 e 6).
4. **Selection (ALTO):** o texto diz ICs sobrepostos "na maioria", e a conclusão trata −1,1% a +3,3% como neutro. Turtles (+3,3%) e Zigzag (+1,9%) têm ICs disjuntos pela própria regra.
5. **Conclusão vs. Tabela 6 (MÉDIO):** "31%–42% em Duplicates e Reversed", mas Duplicates em 10⁶ é −29,9%. "3%–5% na maioria das topologias", mas Zigzag em 10⁶ é −10,4%.
6. **§3.1 vs. §4.3 (BAIXO):** "each iteration lasting fractions of a second" versus Selection em n=10⁵ com 1,85–2,09 s.
7. **Tabela 5 vs. Tabela 4 (ALTO):** Cpre médio de 12,44 µs excede o total de Bubble/Reversed (10,28 µs), e Insertion/Reversed (13,04 µs) implica Csort ≈ 0,6 µs após o pré-processamento.
8. **§2.4 vs. Tabela 3 (MÉDIO):** "ganho linear" contra redução de 45% de Θ(n²) inversões.
9. **§2.2 vs. Tabela 4 (ALTO):** Bubble Sort "adaptativo" contra tempo quase plano com I variando em duas ordens de grandeza.
10. **§3.2.2 vs. Tabela 6 (ALTO):** "garante O(n log n)" contra escala de ≈ n^1,45 em Reversed, Zigzag e Turtles.
11. **§4.3 vs. §5 (MÉDIO):** hipótese microarquitetural virou "attributed to" na conclusão.
12. **§1 vs. corpo (ALTO):** promessa de "condições teóricas e práticas" e "modelagem formal" sem entrega.
13. **Abstract vs. resultados (MÉDIO):** "até 10⁶" convive com resultados dos adaptativos só em n ≤ 10⁵ e reportados apenas em n=10⁴.
14. **§3.5 (BAIXO):** "execution-order alternation" não aparece em nenhuma outra parte.

---

# 9. PERGUNTAS QUE UM REVISOR PODERIA FAZER

**Pergunta:** O método vence `sort_unstable` do Rust ou uma detecção de runs com reversão O(n)?
**O artigo responde?** Não.
**Onde?** n/a

**Pergunta:** Os ganhos de ≈100% acontecem fora de entradas alinhadas com a simetria do algoritmo?
**O artigo responde?** Parcialmente.
**Onde?** §4.1 admite o alinhamento; Tabela 4 mostra Random/Turtles.

**Pergunta:** Como Zigzag e Turtles são gerados, exatamente?
**O artigo responde?** Não.
**Onde?** Tabela 2 (descrição informal).

**Pergunta:** Por que o Quicksort leva 592 ms para um vetor já ordenado de 10⁶ elementos?
**O artigo responde?** Não.
**Onde?** Tabela 6; §4.3 oferece explicação contrária à teoria.

**Pergunta:** Qual parte da degradação de Quicksort e Merge é só o custo Cpre?
**O artigo responde?** Não.
**Onde?** n/a (Tabela 5 dá a média, mas não por topologia).

**Pergunta:** O resultado vale para n pequeno, onde Insertion Sort é realmente usado?
**O artigo responde?** Não.
**Onde?** n/a

**Pergunta:** Como o pool de 50 vetores se relaciona com as 50 amostras do Criterion, e a cópia do vetor é cronometrada?
**O artigo responde?** Não.
**Onde?** §3.1, §3.3 (descrição incompleta).

**Pergunta:** O efeito persiste em outras CPUs, compiladores e sistemas operacionais?
**O artigo responde?** Não.
**Onde?** §3.5 (só cita a ausência de perf).

**Pergunta:** Por que Bubble Sort ganha tão pouco em Random se 45% das inversões são eliminadas?
**O artigo responde?** Não.
**Onde?** n/a

**Pergunta:** Qual é a redução esperada em entrada aleatória, e qual é o ponto de equilíbrio entre Cpre e economia em Csort?
**O artigo responde?** Não.
**Onde?** §2.4 deixa a derivação como trabalho futuro.

**Pergunta:** A análise estatística é coerente com o pareamento e com a regra declarada?
**O artigo responde?** Parcialmente.
**Onde?** §3.3; Selection Turtles e Zigzag violam a regra.

**Pergunta:** Como os resultados se conectam a uma contribuição de Evaluation Science?
**O artigo responde?** Não.
**Onde?** n/a

---

# 10. CHECKLIST OBRIGATÓRIO ANTES DA SUBMISSÃO

Ordenada por dependência lógica.

- [ ] 1. Abrir o site oficial de submissão do PEvaluation logado e confirmar template, limite de páginas, regras de anonimização, prazo e exigências de registro e apresentação.
- [ ] 2. Decidir o enquadramento em Evaluation Science and Engineering (ex.: desenho de avaliação de pré-passadas e confundidores) e ajustar título, abstract e introdução de acordo.
- [ ] 3. Definir formalmente as seis topologias (fórmulas/pseudocódigo), incluindo Zigzag e Turtles.
- [ ] 4. Validar a implementação do Quicksort (comparar com `sort_unstable` ou pdqsort; explicar o expoente ≈1,45 e o custo em vetor ordenado) e corrigir ou declarar a patologia.
- [ ] 5. Adicionar baselines competitivos: `sort_unstable` e `sort` do Rust, e "inverter se descendente" em O(n).
- [ ] 6. Adicionar topologias não alinhadas (e/ou dados reais) e n pequeno (ou remover a motivação de conjuntos pequenos e sistemas restritos).
- [ ] 7. Reportar Cpre por topologia com o mesmo harness (Criterion) e separar ΔCsort.
- [ ] 8. Incluir varredura de tamanhos (1.000 a 10⁶) em figura de speedup contra n.
- [ ] 9. Refazer a análise estatística de forma pareada (IC da razão), marcar significância e corrigir os textos sobre Selection.
- [ ] 10. Corrigir a explicação de Bubble Sort (passadas/deslocamento máximo) e remover "justificado matematicamente".
- [ ] 11. Reescrever o abstract e a conclusão conforme os novos resultados (fim da generalização a partir de entradas alinhadas; hipóteses apresentadas como hipóteses; faixas numéricas consistentes).
- [ ] 12. Completar o protocolo de reprodução (pool, setup, flags, frequência/SMT, sementes) e disponibilizar repositório anonimizado com `Cargo.lock` e scripts.
- [ ] 13. Corrigir §2.4 (ganho linear vs. Θ(n²)); incluir o lema de monotonicidade ou remover a promessa teórica.
- [ ] 14. Expandir a seção de ameaças à validade.
- [ ] 15. Corrigir referências cruzadas, "2,s", [11], [13], [6] e a nota de rodapé 1; limpar metadados do PDF e rever Fig. 1–3 em P&B.

---

# 11. MELHORIAS OPCIONAIS

- Reportar ns por inversão do Insertion Sort (≈0,22) como validação de métrica e derivar o ponto de equilíbrio ΔI ≈ 5–6·n.
- Medir contadores de hardware (AMD uProf no Windows, se disponível, ou Linux) ou contar trocas/comparações por instrumentação.
- Repetir em mais de uma máquina e com outro compilador.
- Testar n ≥ 10⁷ para sair da cache e i64 ou structs.
- Mostrar medianas e outliers em apêndice.
- Substituir a Fig. 3 por um gráfico de razões com ICs.
- Usar entradas reais (logs, sensores, dados quase ordenados).
- Encurtar os parágrafos de classificação da pesquisa.
- Empacotar o artefato para avaliação (TBench incentiva artefatos abertos).

---

# 12. DIAGNÓSTICO FINAL

1. **Problema crítico?** Sim: a dependência das conclusões de topologias alinhadas ao método e a ausência de baseline competitivo.
2. **Problema metodológico sério?** Sim: estatística não pareada e violada pelo próprio texto (Selection), Cpre e Csort não separados, e anomalia de escala no Quicksort.
3. **Problema de benchmark?** Sim: Zigzag e Turtles indefinidos, sem n pequeno nem dados reais, baselines ausentes e um só hardware.
4. **Problema de reprodutibilidade?** Sim: topologias sem fórmula, protocolo de pool/setup, flags, frequência e repositório indisponíveis.
5. **Problema de conformidade com o Bench 2026?** INCERTO. Confirmei o modelo geral (submissão contínua, PEvaluation, double-blind), mas não template, limite de páginas ou escopo. O cabeçalho do PDF menciona o TBench, e o template do PEvaluation precisa ser confirmado.
6. **Problema de anonimização/double-blind?** Nenhum identificado no corpo. Metadados do PDF e o repositório [14] são INCERTO.
7. **Inconsistência científica?** Sim, várias (seção 8), sobretudo Bubble "adaptativo", Selection "neutro" e a promessa teórica.
8. **Problema importante no PDF?** Nenhum confirmado. Itens a verificar: "2,s"/"5,s", ticks da Fig. 2, anotação do zero na Fig. 1, legibilidade em P&B.
9. **Correções obrigatórias antes de submeter:** a checklist da seção 10.