## Nível 1 — Relatório Geral de Avaliação

### a) Síntese do Documento

O manuscrito avaliado é um artigo científico experimental na área de Ciência da Computação (Análise de Algoritmos e Estruturas de Dados), redigido em português brasileiro no formato típico da Sociedade Brasileira de Computação (SBC). O estudo propõe, formaliza e avalia empiricamente uma heurística determinística de pré-processamento simétrico em passada única, com complexidade temporal $O(n)$ e espaço auxiliar $O(1)$, voltada à redução do número de inversões $I$ antes da execução de algoritmos de ordenação. A investigação abrange algoritmos quadráticos adaptativos (*Insertion Sort*, *Bubble Sort*), quadrático não adaptativo (*Selection Sort*), quasilineares (*Merge Sort*, *Quicksort*) e *baselines* da biblioteca padrão do Rust (`sort_unstable`, `sort` e `desc_reverse`), testados sobre nove topologias sintéticas e um vetor operacional de bytes de código-fonte ("Real") em diferentes escalas de entrada.

---

### b) Avaliação por Dimensão Acadêmica

* **Estrutura e Organização:** O artigo apresenta macroestrutura lógica e aderente ao padrão de artigos experimentais em Computação (Resumo/Abstract, Introdução, Referencial Teórico, Métodos, Resultados e Discussão, Considerações Finais, Declaração de IA, Repositório e Apêndice). Contudo, há desequilíbrio interno e redundâncias pontuais: os dois últimos parágrafos da Seção 2.3 repetem quase literalmente o mesmo posicionamento do trabalho frente à literatura, e a Seção 3 inicia com quatro parágrafos de taxonomia metodológica básica (citando manuais introdutórios de metodologia científica) que destoam do rigor conciso esperado em periódicos e conferências de Ciência da Computação.


* **Argumentação e Coerência:** A linha argumentativa é notavelmente honesta, cautelosa e bem delimitada. Os autores evitam reivindicações exageradas, demonstram que ganhos expressivos restringem-se aos métodos quadráticos adaptativos e a topologias geometricamente alinhadas à varredura (Invertido e Zigue-zague), e reconhecem explicitamente que nenhum híbrido supera os ordenadores padrão em entradas aleatórias. Há, porém, uma inconsistência numérica grave entre o texto da Seção 4.3 (`4.070,08 ms` para *Quicksort*/Tubo em $n=10^6$) e a Tabela 12 (`$1.070,08 \pm 9,70$` ms), além de diversos valores truncados nas tabelas do Apêndice que comprometem a leitura dos dados.


* **Rigor Metodológico:** Este é o ponto mais sólido do manuscrito. O delineamento experimental controla aquecimento de cache, rotação circular de clones fora da região cronometrada, pareamento estrito de entradas, replicação em cinco *pools* independentes (*seeds* 42–46) e inferência estatística pareada por *bootstrap* ($B=10.000$). A inclusão de *baselines* competitivos (`sort_unstable`, `sort` e detecção $O(n)$ de *run* descendente `desc_reverse`) e a delimitação explícita das ameaças à validade conferem alta credibilidade científica ao trabalho.


* **Revisão de Literatura:** A fundamentação clássica está bem ancorada (Knuth, Mannila, Estivill-Castro e Wood, Cormen et al.). Todavia, a Seção 2.3 apresenta uma lacuna temporal severa: não há nenhuma referência acadêmica publicada nos últimos trinta anos (a mais recente na revisão de trabalhos relacionados é de 1992). Embora algoritmos modernos como `ipnsort`, `driftsort` e `pdqsort` sejam mencionados no texto, eles não recebem citações científicas formais, nem há diálogo com pesquisas contemporâneas sobre ordenação adaptativa moderna ou pré-processamento híbrido.


* **Linguagem e Estilo Acadêmico:** O registro formal em português brasileiro é competente e preciso na maior parte do texto, mas o documento sofre com problemas de revisão tipográfica e falhas de compilação LaTeX: há um fragmento de frase deslocado na Seção 3.3 (`"não o"`), erros recorrentes de codificação onde o tipo `i32` do Rust aparece impresso como `132`, falhas de modo matemático (palavras como `e` e `de` em itálico matemático sem espaçamento), aspas corrompidas (`${}^{6}50$`) e erros de digitação (`ê O(1)`, `C_{origina]}`).


* **Conformidade Normativa e Tipográfica:** O manuscrito utiliza o sistema de chamada `[Autor Ano]`, compatível com o modelo da SBC, mas não com a ABNT NBR 10520 (`(AUTOR, ano)`). O problema normativo/tipográfico mais crítico reside nas **Tabelas 5, 10, 11 e 12**: o excesso de largura do conteúdo em relação às colunas fez com que dezenas de dígitos iniciais fossem suprimidos ou sobrepostos na renderização do PDF (ex.: `$40,99` em vez de `$140,99`; `$.826,81` em vez de `$3.826,81`; `$22,77` em vez de `$122,77`). Além disso, há alternância irregular entre ponto e vírgula como separador decimal nas tabelas, e o símbolo de significância `†` aparece renderizado como `+` ao lado de percentuais positivos (ex.: `+6,4%+`), gerando ambiguidade visual.


* **Originalidade e Contribuição:** A contribuição é clara e bem delimitada: a proposição da passada simétrica combinada (trocas espelhadas + ajustes locais adjacentes), a derivação analítica do termo de primeira ordem $\mathbb{E}[\Delta I_{esp}] = n^2/12 + O(n)$ para a componente espelhada isolada, a regra empírica de ponto de equilíbrio ($\Delta I \ge 5n$) e o mapeamento sistemático de suas fronteiras de aplicabilidade.



---

### c) Pontos Fortes

* **Rigor estatístico e experimental exemplar:** Uso de inferência pareada por *bootstrap* sobre *pools* compartilhados, separação rigorosa entre alocação/clonagem e medição temporal, e verificação de estabilidade entre múltiplas sementes.


* **Integridade científica e transparência:** Relato explícito de resultados negativos (degradação no *Quicksort* e nos ordenadores padrão), distinção rigorosa entre o que foi demonstrado analiticamente e o que constitui hipótese interpretativa, e disponibilização integral do código e dos CSVs para reprodutibilidade.


* **Escolha criteriosa de *baselines*:** A inclusão de `desc_reverse` desmonta o viés geométrico do cenário Invertido, demonstrando maturidade analítica incomum.



---

### d) Pontos Críticos e Prioridades de Melhoria

1. **[Prioridade Alta] Corrigir o truncamento e a sobreposição de valores nas Tabelas 5, 10, 11 e 12:** Dezenas de células perderam o primeiro dígito ou apresentam texto sobreposto na compilação do PDF. Todos os números devem ser auditados contra os CSVs originais e a largura/alinhamento das colunas LaTeX deve ser reconfigurada.


2. **[Prioridade Alta] Resolver a discrepância numérica em *Quicksort*/Tubo ($n=10^6$):** O texto da Seção 4.3 cita `4.070,08 ms`, enquanto a Tabela 12 registra `$1.070,08 \pm 9,70$` ms. É indispensável verificar o dado bruto e alinhar texto e tabela.


3. **[Prioridade Alta] Corrigir erros tipográficos, matemáticos e de pseudocódigo:** Corrigir o Algoritmo 1 (sinal ausente em `meio - 1`, seta de atribuição ausente em `j <- ultimo - i`, e conferir no código Rust se a borda direita é `j - 1 > meio` ou `j - 1 >= meio`), corrigir o fragmento deslocado `"não o"` na Seção 3.3 e substituir todas as ocorrências de `132` por `\texttt{i32}`.


4. **[Prioridade Alta] Atualizar a Revisão de Literatura (Seção 2.3):** Incorporar referências científicas revisadas por pares dos últimos 10–15 anos sobre algoritmos de ordenação adaptativa modernos (como Timsort, Powersort, pdqsort, driftsort/ipnsort) e estratégias híbridas de pré-ordenação, eliminando o hiato bibliográfico pós-1992.


5. **[Prioridade Média] Adequar o estilo da Seção 3 e eliminar redundâncias:** Condensar os quatro parágrafos iniciais da Seção 3 (classificação metodológica via Gil e Sampieri) para um parágrafo técnico objetivo, fundir os dois últimos parágrafos redundantes da Seção 2.3 e enxugar a repetição sobre o funcionamento dos *pools* entre as Seções 3.1 e 3.3.


6. **[Prioridade Média] Padronizar símbolos estatísticos e separadores decimais:** Substituir `+` por `\textsuperscript{\dagger}` nas marcas de estabilidade entre *pools*, trocar `*`/`**` da Tabela 3 por letras (`a`, `b`) para evitar colisão semântica, e padronizar a vírgula como separador decimal em todas as tabelas.


7. **[Prioridade Baixa] Verificar requisitos do veículo de submissão (Anonimização e Palavras-chave):** Caso o evento ou periódico exija revisão duplo-cega (*double-blind*), remover nomes, afiliações, e-mails e o link direto do GitHub (`Zidan-09`) na versão de submissão, além de incluir *Palavras-chave* e *Keywords* se exigidos pelo *template*.



---

### e) Veredito Geral

**Classificação: Próximo da Versão Final.**
O núcleo científico, a engenharia experimental e a análise estatística possuem qualidade publicável. Contudo, o manuscrito **não deve ser submetido no estado atual** devido aos defeitos graves de renderização nas tabelas (supressão de dígitos), à discrepância numérica na Seção 4.3, às falhas tipográficas no Algoritmo 1 e no texto e à ausência de referências contemporâneas na Seção 2.3.

---

## Nível 2 — Sugestões Inline Trecho a Trecho

* **Trecho original (Resumo / Abstract, p. 1):**
> "Inverted and Zigzag topologies achieve near-total elimination with speedups of 386-2,280, but these gains do not extend to random or almost sorted inputs" / "As topologias Invertido e Zigue-zague alcançam eliminação quase total com speedups de 386-2.280"
> 
> 


* **Diagnóstico:** Ausência do símbolo multiplicativo (`×` ou `x`) nos valores de *speedup* em ambos os idiomas, além da necessidade de explicitar na primeira menção em inglês que *Inverted* corresponde a *reverse-sorted*.
* **Sugestão de Substituição:**
> "Reverse-sorted (Inverted) and Zigzag topologies achieve near-total elimination with speedups of $386\times$–$2,280\times$, but these gains do not extend to random or almost sorted inputs" / "As topologias Invertido e Zigue-zague alcançam eliminação quase total com *speedups* de $386\times$ a $2.280\times$;"


* **Justificativa:** Precisão terminológica e rigor na representação de razões de aceleração (*speedup*) em resumos científicos.

---

* **Trecho original (Seção 2.3 — Trabalhos Relacionados, p. 4):**
> "Em relação a esses trabalhos, a contribuição deste artigo não está em propor novas medidas de desordem nem em caracterizar classes de algoritmos adaptativos, mas em propor, implementar e avaliar experimentalmente uma heurística simétrica específica de passada única $O(n)$, além de derivar, na Seção 3.4, um termo de primeira ordem próprio para a componente de trocas espelhadas considerada isoladamente. [...] Diferenciando-se das medidas e da análise de otimalidade de Mannila e da caracterização de algoritmos adaptativos do survey, este trabalho objetiva avaliar uma heurística de pré-processamento simétrico projetada para a mitigação direta de inversões em tempo linear $O(n)$, investigando implicações teóricas e práticas em dez topologias de entrada e delimitando o escopo de aplicabilidade ao isolar cenários em que o ganho pela redução de inversões supera o custo linear da etapa preliminar."
> 
> 


* **Diagnóstico:** Redundância estrutural severa: dois parágrafos consecutivos repetem exatamente o mesmo contraste entre o artigo e os trabalhos de Mannila e Estivill-Castro & Wood.
* **Sugestão de Substituição:**
> "Diferenciando-se da formalização de medidas de desordem de Mannila [Mannila 1985] e da taxonomia de algoritmos adaptativos de Estivill-Castro e Wood [Estivill-Castro and Wood 1992], a contribuição deste estudo consiste em propor, implementar e avaliar experimentalmente uma heurística simétrica de passada única $O(n)$ voltada à mitigação direta de inversões. Além disso, deriva-se analiticamente um termo de primeira ordem para a componente isolada de trocas espelhadas (Seção 3.4) e delimita-se empiricamente, em dez topologias e múltiplas escalas, a fronteira de viabilidade em que a redução de inversões compensa o custo linear da etapa preliminar."


* **Justificativa:** Concisão, coesão textual e eliminação de repetição argumentativa.

---

* **Trecho original (Seção 3 — Métodos e Técnicas da Pesquisa, p. 4–5):**
> "Quanto à natureza, a presente pesquisa classifica-se como aplicada. Gil [Gil 2019] caracteriza a pesquisa aplicada pela geração de conhecimentos orientados à solução de problemas específicos, em contraste com a pesquisa básica [...]. Quanto à abordagem do problema, a pesquisa é quantitativa. Segundo Sampieri, Collado e Lucio [Sampieri et al. 2021] [...] Quanto aos objetivos, a pesquisa é exploratória e descritiva. Gil [Gil 2019] define [...] Quanto aos procedimentos técnicos, a pesquisa é experimental. Para Gil [Gil 2019], o experimento consiste..."
> 
> 


* **Diagnóstico:** Linguagem didática excessivamente escolarizada, típica de trabalhos de conclusão de curso (TCC), inadequada para artigos de periódicos e conferências em Ciência da Computação.
* **Sugestão de Substituição:**
> "Este estudo caracteriza-se como uma investigação experimental, quantitativa e de caráter exploratório-descritivo. O delineamento manipula como variáveis independentes a topologia estrutural e o tamanho $n$ dos vetores de entrada, mensurando como variáveis dependentes o número residual de inversões $I$ e o tempo de execução. O controle experimental assegura reprodutibilidade por meio de sementes determinísticas fixas, pareamento estrito das instâncias entre variantes, ciclos de aquecimento de cache e análise estatística inferencial baseada em intervalos de confiança e *bootstrap* pareado."


* **Justificativa:** Adequação ao registro científico e ao estilo conciso da comunidade de Ciência da Computação, mantendo apenas as informações metodológicas operacionalmente relevantes.

---

* **Trecho original (Seção 3.1 e Seção 3.5, p. 6, 12, 23):**
> "Os pontos em $n=10^{7}$ correspondem a $\approx40$ MB por vetor 132" / "conversão de 1 byte para 1 132" / "e dados 132 únicos" / "e a chaves 132"
> 
> 


* **Diagnóstico:** Erro recorrente de codificação/OCR em que o tipo primitivo inteiro de 32 bits com sinal do Rust (`i32`) foi impresso como `132`.
* **Sugestão de Substituição:**
> Substituir todas as ocorrências de `132` referentes ao tipo de dado por `\texttt{i32}` (ex.: "vetor \texttt{i32}", "1 byte para 1 \texttt{i32}", "chaves \texttt{i32}").


* **Justificativa:** Precisão terminológica e correção de falha tipográfica crítica.

---

* **Trecho original (Seção 3.3 — Protocolo Experimental, p. 8–9):**
> "não o \n A configuração das medições foi ajustada conforme o tamanho e a complexidade das entradas. Aqui, "amostras" designa exclusivamente o parâmetro sample_size do Criterion, isto é, o número de medições repetidas que o Criterion coleta por braço (puro, com pré, ou pré isolado) para estimar médias e ICs \n número de vetores de entrada distintos."
> 
> 


* **Diagnóstico:** Erro de diagramação/quebra de linha no código-fonte LaTeX que deslocou o segmento `"não o"` para o início do parágrafo.
* **Sugestão de Substituição:**
> "A configuração das medições foi ajustada conforme o tamanho e a complexidade das entradas. Neste trabalho, o termo 'amostras' designa exclusivamente o parâmetro `sample_size` do Criterion — isto é, o número de medições repetidas coletadas por braço (puro, com pré-processamento ou pré-processamento isolado) para estimar médias e intervalos de confiança —, e não o número de vetores de entrada distintos."


* **Justificativa:** Restauração da sintaxe, da coesão e da clareza do período.

---

* **Trecho original (Seção 3.4 — Algoritmo 1, p. 10):**
> "`for i <- 0 to meio 1 do` \n `j ultimo - i;` \n ... \n `if j - 1 > meio then`"
> 
> 


* **Diagnóstico:** Falhas sintáticas no pseudocódigo (ausência do sinal `-` em `meio - 1` e do operador `\leftarrow` em `j \leftarrow ultimo - i`), além de possível divergência na condição de contorno da metade direita (`j - 1 > meio` omite a comparação mais interna quando $n$ é par; para $n=4$, $meio=2$, $i=0 \implies j=3 \implies j-1=2 \not> 2$).
* **Sugestão de Substituição:**
> "`for i <- 0 to meio - 1 do` \n `    j <- ultimo - i;` \n ... \n (Verificar no código-fonte em Rust se a condição é `j - 1 > meio` ou `j - 1 >= meio` e uniformizar o Algoritmo 1 e o item 3 da página 11)."


* **Justificativa:** Rigor formal na especificação algorítmica e garantia de fidelidade entre o pseudocódigo publicado e a implementação auditável.

---

* **Trecho original (Seção 5 — Considerações Finais, p. 22):**
> "satisfazendo $C_{pre}+C_{sort}<C_{origina]}$;"
> 
> 


* **Diagnóstico:** Erro de digitação no subscrito matemático (`origina]` em vez de `original`).
* **Sugestão de Substituição:**
> "satisfazendo $C_{\text{pre}} + C_{\text{sort}} < C_{\text{original}}$;"


* **Justificativa:** Correção tipográfica e padronização de subscritos textuais em modo matemático (`\text{...}`).