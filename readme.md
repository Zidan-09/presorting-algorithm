# 📊 Presort Algorithm Benchmark

Este projeto tem como objetivo avaliar o impacto de um algoritmo de **pré-processamento (presort)** no desempenho de diferentes algoritmos de ordenação.

A ideia principal é simples:

> Antes de ordenar completamente um array, aplicamos um passo linear que reduz desordens (inversões), tornando o trabalho dos algoritmos de ordenação mais eficiente — especialmente os de complexidade quadrática.

---

# 🚀 Propósito

Algoritmos como Bubble Sort, Insertion Sort e Selection Sort possuem complexidade **O(n²)**, mas seu desempenho real depende muito da **quantidade de desordem** no array.

Este projeto busca responder:

* Um pré-processamento O(n) pode melhorar algoritmos O(n²)?
* Quanto reduzimos comparações e trocas?
* Quais tipos de arrays mais se beneficiam?

---

# 🧠 Algoritmos suportados

Você pode testar os seguintes algoritmos:

* `bubble` → Bubble Sort
* `insertion` → Insertion Sort
* `selection` → Selection Sort
* `merge` → Merge Sort
* `quick` → Quick Sort

---

# 🧪 Tipos de arrays disponíveis

O sistema gera diferentes padrões de entrada para testar cenários reais:

* `random` → valores aleatórios
* `inverted` → totalmente invertido
* `zigzag` → alternância alto/baixo
* `turtles` → valores pequenos no final (caso ruim clássico)
* `duplicates` → muitos valores repetidos
* `almostsorted` → quase ordenado

---

# ⚙️ Como executar

## 1. Instalar dependências

Se estiver usando Node + TypeScript:

```bash
npm install
```

---

## 2. Compilar o projeto

```bash
npx tsc
```

---

## 3. Executar

```bash
node dist/index.js [algoritmo] [tipo_array] [tamanho]
```

---

# 📌 Parâmetros

| Parâmetro  | Descrição               | Exemplo  |
| ---------- | ----------------------- | -------- |
| algoritmo  | Tipo de ordenação       | `bubble` |
| tipo_array | Tipo de entrada         | `random` |
| tamanho    | Quantidade de elementos | `10000`  |

---

# ✅ Exemplos de uso

### Teste padrão (valores default)

```bash
node dist/index.js
```

### Bubble Sort com array aleatório

```bash
node dist/index.js bubble random 10000
```

### Insertion Sort com array quase ordenado

```bash
node dist/index.js insertion almostsorted 5000
```

### Quick Sort com array invertido

```bash
node dist/index.js quick inverted 20000
```

---

# ⚠️ Valores padrão

Caso você não informe os parâmetros:

* Algoritmo → `bubble`
* Tipo de array → `random`
* Tamanho → `10000`

---

# 📈 O que é analisado

O método `sortService` é responsável por executar:

* Tempo de execução
* Número de comparações
* Número de trocas
* Impacto do presort

---

# 💡 Ideia central

O presort atua como um **otimizador de entrada**, reduzindo o trabalho necessário para ordenar.

* Custo: O(n)
* Benefício: redução de inversões
* Resultado: algoritmos quadráticos mais rápidos na prática

---

# 🧪 Sugestões de experimentos

* Testar `almostsorted` vs `random`
* Avaliar impacto em arrays grandes (100k+)
* Comparar com algoritmos O(n log n)

---

# 📚 Possível contribuição acadêmica

Este projeto pode evoluir para:

* Artigo sobre pré-processamento em ordenação
* Novo algoritmo híbrido
* Estudo de redução de inversões