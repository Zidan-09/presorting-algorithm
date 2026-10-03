# Problemas de conteúdo e forma

## Corrija antes de enviar:

- Formatação numérica: o PDF mostra 45, 19%, ×2, 280, −14, 2% com espaço depois da vírgula. É a vírgula decimal em modo matemático do LaTeX. Use {,} ou o pacote siunitx, incluindo no abstract em inglês.
- Figura 2: os eixos de Merge e Quicksort vão de 10⁰ a 10², em µs, mas o texto reporta 360,74 µs para o Quicksort puro em n=10⁴. Confira unidade e escala.
- Figura 3: o rótulo "Padrão/Aleatório" é ambíguo. O texto fala de sort_unstable, então use esse nome.
- Selection Sort no Invertido (4.3): "19.150,38 vs 20.900,45 µs" não diz qual é o puro e qual é o com pré. Pelos números, o puro é o menor, mas deixe explícito.
- Abstract: diz "100% nas alinhadas", mas a Tabela 3 mostra 99,99% no Zigue-zague.
- Afirmação sobre Mannila (2.4): o texto diz que o Insertion Sort é "ótimo" para inversões. Pelo que lembro, o limite ótimo de comparações para I é O(n log(1+I/n)), que o Insertion Sort simples (O(n+I)) não atinge. Confira no artigo original antes de manter a frase.

## Fragilidades que um revisor pode apontar:

- Tipo de pesquisa "explicativa": você diz explicar "como e por que", mas os mecanismos (predição de desvios, cache) ficam como hipótese. Considere "exploratória/descritiva" ou suavize a classificação.
- Hipóteses H1–H3: as exceções (Tubo, Runs, Merge em entradas alinhadas) já refletem os resultados, o que parece ajuste posterior. Reformule como hipóteses gerais ou diga que foram refinadas após a exploração.
- Quicksort: a implementação já é patológica sem pré-processamento (4.070 ms no Tubo). Você reconhece isso, mas o revisor vai pedir comparação com pdqsort. Reduzir o peso dessa parte no texto ajuda.
- Ablação do passo espelhado isolado: é barata de fazer e testaria a previsão n²/12 contra os 11,3M medidos. Resolveria uma pendência que você hoje deixa como trabalho futuro.
- Densidade: há muitas ressalvas ("permanece hipótese", "não é derivação") e muitas remissões ao material suplementar. Para um periódico de graduação, consolide as ressalvas na Seção 3.5 e deixe os resultados mais limpos.
- Seções finais: não há Agradecimentos nem Financiamento. São opcionais, mas inclua se houver bolsa ou apoio.