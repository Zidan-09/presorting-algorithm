# Guia para Transcrição do Artigo para o Formato da REIC

## Objetivo

Adaptar o artigo científico existente para o **formato exigido pela Revista Eletrônica de Iniciação Científica em Computação (REIC)**.

A tarefa é de **transcrição e adequação editorial/formatacional**, não de reescrita científica.

O conteúdo científico existente deve ser preservado. O trabalho deve resultar em uma nova versão do artigo completamente compatível com o **template oficial da REIC fornecido no projeto**.

---

# 1. Regra principal

> **O template oficial da REIC presente no projeto é a fonte de verdade para toda a formatação.**

Não utilizar como referência:

* templates genéricos da SBC;
* templates de eventos da SBC;
* templates de outras revistas da SBC;
* modelos encontrados na internet;
* artigos de outras conferências;
* versões antigas da REIC;
* suposições sobre como a SBC normalmente formata artigos.

Caso exista conflito entre uma regra deste documento e o template oficial da REIC, **o template da REIC prevalece**.

Caso exista alguma regra no template cuja interpretação não seja clara, **não assumir**. Registrar a dúvida no relatório final.

---

# 2. Arquivos de entrada

Identificar primeiro os arquivos que representam:

### Artigo original

O artigo científico atualmente desenvolvido deve ser utilizado como fonte primária do conteúdo.

Preservar:

* título;
* autores;
* afiliações;
* resumo;
* abstract;
* palavras-chave;
* seções;
* subseções;
* texto;
* equações;
* tabelas;
* figuras;
* resultados;
* conclusões;
* referências.

### Template da REIC

Localizar o template oficial da REIC fornecido no projeto.

Analisar:

* arquivo `.tex` principal;
* arquivos `.cls`;
* arquivos `.sty`;
* arquivos de exemplo;
* bibliografia de exemplo;
* arquivos auxiliares;
* instruções fornecidas pelo próprio template.

Não modificar o template original para "facilitar" a migração.

---

# 3. Primeira etapa — analisar o template

Antes de alterar o artigo:

* [ ] Identificar o arquivo `.tex` principal do template.
* [ ] Identificar a classe LaTeX utilizada.
* [ ] Identificar os pacotes carregados.
* [ ] Identificar comandos personalizados.
* [ ] Identificar estrutura de título.
* [ ] Identificar estrutura de autores.
* [ ] Identificar afiliações.
* [ ] Identificar formato do resumo.
* [ ] Identificar palavras-chave.
* [ ] Identificar abstract.
* [ ] Identificar keywords.
* [ ] Identificar estrutura das seções.
* [ ] Identificar regras para figuras.
* [ ] Identificar regras para tabelas.
* [ ] Identificar regras para equações.
* [ ] Identificar formato das referências.
* [ ] Identificar informações obrigatórias de publicação.
* [ ] Identificar declarações obrigatórias.
* [ ] Identificar informações de licença/copyright.
* [ ] Identificar qualquer elemento editorial próprio da REIC.

O template deve ser entendido antes de iniciar a migração.

---

# 4. Não reconstruir o template

Não criar uma nova classe ou um novo template.

Não:

* copiar apenas a aparência visual;
* recriar margens manualmente;
* recriar cabeçalhos;
* recriar rodapés;
* recriar estilos de títulos;
* recriar o sistema de referências;
* substituir a classe por outra;
* remover comandos do template apenas porque parecem desnecessários.

A adaptação deve partir diretamente do template oficial.

---

# 5. Criar uma nova versão do artigo

Não destruir o artigo original.

Criar uma nova estrutura para a versão da REIC.

Exemplo:

```text
artigo/
├── versão-original/
│   ├── main.tex
│   ├── main.pdf
│   └── ...
│
└── reic/
    ├── main.tex
    ├── referencias.bib
    ├── figuras/
    └── ...
```

Os nomes e diretórios devem ser adaptados à estrutura real do projeto.

A regra importante é:

> **A versão atualmente utilizada do artigo deve permanecer intacta.**

---

# 6. Preservação do conteúdo científico

Durante a migração:

* não alterar resultados;
* não alterar valores;
* não alterar experimentos;
* não alterar metodologia;
* não alterar conclusões;
* não remover limitações;
* não inventar informações;
* não adicionar resultados;
* não adicionar referências inexistentes;
* não alterar o significado das afirmações.

Pequenas alterações linguísticas são permitidas somente quando necessárias para:

* adequação ao idioma;
* correção de erro evidente;
* adequação a uma estrutura exigida pelo template;
* correção de comandos LaTeX;
* adaptação de títulos de seções;
* adequação editorial.

Se uma alteração modificar o significado científico, **não realizá-la automaticamente**.

---

# 7. Estrutura do artigo

Mapear a estrutura atual do artigo para a estrutura exigida pela REIC.

Para cada seção:

```text
ARTIGO ORIGINAL
        ↓
IDENTIFICAR EQUIVALENTE
        ↓
TEMPLATE REIC
```

Exemplo conceitual:

```text
Introdução
    ↓
Seção correspondente no template REIC

Fundamentação
    ↓
Seção correspondente no template REIC

Metodologia
    ↓
Seção correspondente no template REIC

Resultados
    ↓
Seção correspondente no template REIC

Conclusão
    ↓
Seção correspondente no template REIC
```

Não criar seções novas apenas por preferência pessoal.

Não remover seções científicas relevantes apenas para reduzir o tamanho do documento.

---

# 8. Título e autoria

Migrar para o mecanismo de autoria utilizado pelo template.

Verificar:

* [ ] título;
* [ ] título em inglês, se exigido;
* [ ] nome completo dos autores;
* [ ] ordem dos autores;
* [ ] afiliações;
* [ ] e-mails;
* [ ] ORCID, se o template exigir;
* [ ] instituição;
* [ ] cidade/estado/país, se exigido.

Não alterar a ordem dos autores.

Não abreviar nomes sem que o template determine isso.

---

# 9. Resumo e palavras-chave

Migrar o resumo para exatamente a estrutura utilizada pelo template.

Verificar:

* [ ] resumo;
* [ ] abstract;
* [ ] palavras-chave;
* [ ] keywords;
* [ ] separadores;
* [ ] quantidade permitida;
* [ ] nomenclatura utilizada pela REIC.

Não reescrever o resumo apenas para "ficar melhor".

Caso o template imponha limite ou estrutura específica, adaptar respeitando o conteúdo original.

---

# 10. Seções e subseções

Utilizar exclusivamente os comandos estruturais fornecidos pelo template.

Exemplo:

```latex
\section{...}
\subsection{...}
\subsubsection{...}
```

ou os comandos específicos fornecidos pelo template.

Não aplicar formatação manual como:

```latex
\textbf{1. Introdução}
```

quando o template possuir comandos próprios para seções.

---

# 11. Figuras

Migrar todas as figuras existentes.

Para cada figura:

* [ ] arquivo original preservado;
* [ ] caminho corrigido;
* [ ] formato compatível;
* [ ] legenda preservada;
* [ ] referência no texto preservada;
* [ ] numeração automática utilizada;
* [ ] posicionamento compatível com o template;
* [ ] tamanho compatível com a largura disponível;
* [ ] texto interno permanece legível.

Não converter ou redesenhar uma figura apenas por preferência estética.

Não remover uma figura porque ela não se encaixa inicialmente.

Primeiro adequar seu tamanho e posicionamento ao template.

---

# 12. Tabelas

Migrar todas as tabelas.

Verificar:

* [ ] largura;
* [ ] alinhamento;
* [ ] legenda;
* [ ] numeração automática;
* [ ] referência no texto;
* [ ] unidades;
* [ ] legibilidade;
* [ ] quebra de página;
* [ ] compatibilidade com a largura da coluna.

Se uma tabela ultrapassar a largura disponível:

1. tentar adequar a tabela ao formato do template;
2. reduzir conteúdo redundante;
3. ajustar colunas;
4. utilizar os mecanismos de tabela suportados pelo template.

Não simplesmente reduzir a fonte até tornar a tabela ilegível.

---

# 13. Equações

Preservar todas as equações.

Verificar:

* [ ] numeração;
* [ ] referências;
* [ ] alinhamento;
* [ ] símbolos;
* [ ] unidades;
* [ ] compatibilidade com o template.

Não converter equações em imagens.

---

# 14. Código e algoritmos

Caso existam trechos de código:

* preservar o conteúdo;
* utilizar o mecanismo suportado pelo template;
* verificar largura;
* verificar fonte;
* verificar quebra de linhas;
* verificar referências.

Não remover código relevante apenas para solucionar problemas de layout.

---

# 15. Referências

Migrar a bibliografia para o sistema utilizado pelo template da REIC.

Verificar cuidadosamente:

* [ ] todas as citações do artigo possuem referência;
* [ ] todas as referências utilizadas são reais;
* [ ] autores corretos;
* [ ] título correto;
* [ ] ano correto;
* [ ] periódico/conferência correto;
* [ ] volume;
* [ ] número;
* [ ] páginas;
* [ ] DOI;
* [ ] URL quando apropriado;
* [ ] formato exigido pelo template.

Não modificar uma referência apenas para fazê-la "parecer" compatível.

A referência deve ser semanticamente correta e formatada pelo mecanismo bibliográfico esperado pelo template.

---

# 16. Citações

Verificar todas as citações no texto.

Procurar:

```text
??
```

e problemas equivalentes.

Também verificar:

* citações sem referência;
* referências sem citação;
* citações duplicadas;
* chaves BibTeX inexistentes;
* chaves BibTeX duplicadas;
* referências quebradas;
* DOI incorreto.

---

# 17. Figuras, tabelas e referências cruzadas

Nenhuma referência cruzada deve permanecer quebrada.

Procurar no PDF final:

```text
??
Figure ??
Table ??
Section ??
```

ou equivalentes produzidos pelo sistema de referências utilizado.

Todos os elementos devem ser numerados automaticamente.

Não inserir números manualmente no texto.

---

# 18. Compatibilidade com o template

Não introduzir pacotes arbitrariamente.

Antes de adicionar qualquer pacote:

1. verificar se o template já fornece a funcionalidade;
2. verificar se outro pacote já carregado resolve o problema;
3. verificar se o pacote é compatível com a classe;
4. somente então adicionar o pacote, se realmente necessário.

Evitar alterações na infraestrutura do template.

---

# 19. Compilação

Compilar a nova versão utilizando o método recomendado pelo próprio template.

Se o template fornecer instruções específicas, segui-las.

Após a compilação:

* [ ] PDF gerado;
* [ ] sem erros;
* [ ] sem warnings críticos;
* [ ] sem referências indefinidas;
* [ ] sem citações indefinidas;
* [ ] sem arquivos ausentes;
* [ ] sem fontes ausentes;
* [ ] sem figuras ausentes.

Warnings puramente tipográficos devem ser analisados individualmente.

Não tentar eliminar todos os warnings automaticamente se isso comprometer o conteúdo ou a formatação.

---

# 20. Revisão visual do PDF

O PDF deve ser inspecionado página por página.

Verificar especialmente:

### Margens

* [ ] nenhum conteúdo ultrapassa as margens;
* [ ] nenhuma tabela invade outra coluna;
* [ ] nenhuma figura ultrapassa a área disponível.

### Texto

* [ ] nenhuma linha está cortada;
* [ ] nenhuma palavra está sobreposta;
* [ ] nenhum caractere está corrompido;
* [ ] não existem fontes inesperadas.

### Figuras

* [ ] todas aparecem;
* [ ] todas são legíveis;
* [ ] legendas estão corretas;
* [ ] referências aparecem.

### Tabelas

* [ ] todas aparecem;
* [ ] não estão cortadas;
* [ ] não ultrapassam as margens;
* [ ] conteúdo está legível.

### Referências

* [ ] não estão cortadas;
* [ ] não existem referências duplicadas inesperadamente;
* [ ] DOI/URLs não quebram o layout.

---

# 21. Limite de páginas

O limite de páginas deve ser determinado **pelo template/diretrizes atuais da REIC fornecidos no projeto**.

Não assumir um limite baseado em:

* artigos antigos;
* chamadas de eventos;
* CTIC;
* outras revistas;
* versões antigas da REIC.

Depois da migração:

* [ ] verificar o número final de páginas;
* [ ] comparar com a regra vigente;
* [ ] ajustar o conteúdo somente se necessário.

Se o artigo ultrapassar o limite:

> Não cortar conteúdo científico automaticamente.

Primeiro registrar o problema e identificar quais ajustes editoriais são possíveis sem prejudicar o artigo.

---

# 22. Elementos editoriais da REIC

O template pode possuir elementos que não existiam no artigo original.

Identificar e preencher corretamente tudo que for obrigatório, como:

* informações editoriais;
* licença;
* copyright;
* DOI;
* dados de publicação;
* declarações;
* informações de autoria;
* identificadores;
* outros metadados.

Não inventar valores para campos que pertencem ao editor.

Se determinado campo for preenchido posteriormente pela REIC, manter exatamente o mecanismo fornecido pelo template.

---

# 23. Licença e copyright

Não substituir, remover ou modificar automaticamente:

* licença;
* copyright;
* informações editoriais;
* textos legais.

Esses elementos pertencem ao template/editorial da REIC.

Utilizar exatamente a configuração fornecida pelo modelo oficial.

---

# 24. Uso de IA

Se o template/diretrizes atuais da REIC exigirem uma declaração sobre uso de inteligência artificial:

* localizar o mecanismo fornecido pelo template;
* verificar a redação exigida;
* preencher somente com informações verdadeiras;
* não inventar ferramentas utilizadas;
* não atribuir autoria a ferramentas de IA;
* preservar a responsabilidade dos autores pelo conteúdo.

Não adicionar uma declaração genérica de IA caso o template não a exija.

---

# 25. O que NÃO fazer

Durante a migração, é proibido:

* reescrever o artigo inteiro;
* mudar a metodologia;
* alterar resultados;
* inventar referências;
* inventar dados;
* modificar valores experimentais;
* alterar conclusões;
* remover conteúdo científico sem justificativa;
* adicionar conteúdo científico não existente;
* trocar o template da REIC pelo template geral da SBC;
* criar uma formatação própria;
* alterar a classe do template sem necessidade;
* modificar a identidade visual da revista;
* remover informações editoriais;
* ignorar regras presentes no template.

---

# 26. Processo de execução

Executar nesta ordem:

```text
1. Localizar artigo original
        ↓
2. Localizar template oficial da REIC
        ↓
3. Analisar completamente o template
        ↓
4. Documentar sua estrutura e regras
        ↓
5. Criar cópia/versionamento do artigo
        ↓
6. Migrar preâmbulo
        ↓
7. Migrar título/autores/afiliação
        ↓
8. Migrar resumo/abstract/keywords
        ↓
9. Migrar seções
        ↓
10. Migrar figuras
        ↓
11. Migrar tabelas
        ↓
12. Migrar equações
        ↓
13. Migrar referências
        ↓
14. Inserir elementos editoriais obrigatórios
        ↓
15. Compilar
        ↓
16. Corrigir erros de LaTeX
        ↓
17. Corrigir problemas de layout
        ↓
18. Revisar PDF página por página
        ↓
19. Conferir todas as referências cruzadas
        ↓
20. Conferir regras da REIC
        ↓
21. Gerar versão final
```

---

# 27. Critério de conclusão

A tarefa somente está concluída quando:

* [ ] o artigo utiliza o template oficial da REIC;
* [ ] o conteúdo científico original foi preservado;
* [ ] todos os elementos foram migrados;
* [ ] todas as figuras estão corretas;
* [ ] todas as tabelas estão corretas;
* [ ] todas as equações estão corretas;
* [ ] todas as citações estão corretas;
* [ ] todas as referências estão corretas;
* [ ] todos os elementos obrigatórios do template foram preenchidos;
* [ ] o PDF compila sem erros;
* [ ] não existem referências cruzadas quebradas;
* [ ] não existem elementos cortados;
* [ ] não existem problemas visuais evidentes;
* [ ] o número de páginas atende às regras vigentes;
* [ ] a versão final está pronta para submissão.

---

# 28. Relatório final obrigatório

Ao terminar, produzir um relatório contendo:

## Arquivos alterados

Listar todos os arquivos modificados/criados.

## Adaptações realizadas

Descrever objetivamente:

* alterações de estrutura;
* alterações de formatação;
* adaptações de tabelas;
* adaptações de figuras;
* adaptações de referências;
* alterações necessárias para compatibilidade com o template.

## Problemas encontrados

Listar qualquer problema que não tenha sido possível resolver automaticamente.

## Alterações de conteúdo

Se qualquer trecho textual tiver sido alterado além da simples adequação LaTeX/formatação:

* indicar o trecho;
* explicar o motivo;
* mostrar o que foi alterado.

Não esconder alterações de conteúdo.

## Validação final

Informar:

```text
Compilação: OK/ERRO
Referências: OK/ERRO
Citações: OK/ERRO
Figuras: OK/ERRO
Tabelas: OK/ERRO
Equações: OK/ERRO
Layout: OK/ERRO
Número de páginas: OK/ERRO
Regras do template: OK/ERRO
PDF final: OK/ERRO
```

---

# Regra final

> **O objetivo não é fazer o artigo "parecer" um artigo da REIC. O objetivo é transcrever o artigo existente para a estrutura oficial da REIC, utilizando o próprio template da revista e preservando integralmente a validade científica do trabalho.**

Se houver dúvida entre **alterar o conteúdo** ou **preservar o conteúdo**, preservar o conteúdo e registrar a questão no relatório final.
