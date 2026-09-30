# Plano de Finalização — Bench 2026 / PEvaluation (`plan2.md`)

Objetivo: concluir a adaptação executando os 6 itens pendentes de `docs/temp.md`
(T1–T6), que exigem **novos experimentos e dados reais**. Nada aqui deve ser
inventado: cada fase produz artefatos medidos que alimentam o artigo.

Estado atual: `artigo/bench2026/main.tex` + `main.pdf` (9 pp.) com todas as
correções textuais aplicadas; `docs/temp.md` lista o que ficou pendente.

Mapa T1–T6 → fases:

| Item `temp.md` | Fase(s) |
|---|---|
| T1. Baselines `sort_unstable`/`sort` + "inverter se descendente" | F1 (harness), F3 (medição), F6 (texto) |
| T2. Topologias não alinhadas, dados reais, n pequeno, n≥10⁷ | F1 (geradores), F3 (medição), F6 (texto) |
| T3. Cpre por topologia no Criterion + separação ΔCsort | F1 (braço de medição), F3, F5 (export), F6 |
| T4. Bootstrap pareado, multi-seed, 2º hardware/compilador, contadores | F2 (diagnóstico hw), F3, F4 (estatística) |
| T5. Validação do Quicksort + assembly do Selection | F2 (diagnóstico), F6 (texto) |
| T6. Repo anonimizado, Cargo.lock/toolchain/scripts, metadados PDF, confirmações PEvaluation | F0 (ambiente), F7 (submissão) |

Ordem de execução: F0 → F1 → F2 → F3 → F4 → F5 → F6 → F7.

---

## F0. Congelamento do ambiente (pré-requisito de tudo; ~1h)

1. `git status` limpo; anotar commit base no manifesto.
2. Criar `presorting-algorithm/rust-toolchain.toml` fixando o toolchain
   (ex.: `channel = "1.96.0"` — mesma versão do artigo) e commitar
   `Cargo.lock` (hoje ausente do repo — verificar; se ausente, gerar com
   build travado e commitar).
3. Registrar no manifesto: `rustc --version -v`, `cargo --version`,
   CPU (modelo, L2/L3, SMT on/off, afinidade), governador de frequência,
   SO/build, flags de compilação. Hoje o perfil é só `opt-level = 3`
   (citado no artigo); decidir e travar no `Cargo.toml`:
   `[profile.release] opt-level = 3, lto = ?, codegen-units = ?`
   (qualquer escolha vale, desde que registrada e mantida até a submissão).
4. Critério de aceite: `cargo bench -- --help` roda no toolchain pinado;
   `rustc --version -v` arquivado em `artigo/resultados/manifesto.txt`
   (estender o manifesto do script de exportação).

## F1. Extensões do harness (sem mudar o protocolo existente; ~1 dia)

Arquivos: `src/utils/tipos.rs`, `src/utils/gerador.rs`,
`src/services/servicebench.rs`, `benches/benchmark.rs`,
`examples/exportar_resultados.rs`.

1. **Novos `ArrayType`** (fórmulas exatas, determinísticas, 0-based):
   - `Sawtooth`: `a[i] = (i % 16) as i32` (padrão clássico de testbeds,
     cf. bancada de testes de Bentley–McIlroy~\cite{bentley_engineering_1993},
     já nas referências).
   - `OrganPipe`: `a[i] = i` se `i < n/2`, senão `a[i] = n - i`.
   - `FewRuns`: `[0,n)` ordenado com 4 blocos disjuntos revertidos
     (posições fixas: quartis do vetor) → 4 runs descendentes.
   - `Real`: vetor carregado de asset binário versionado
     (ex.: timestamps; gravar SHA-256 no manifesto; tamanho documentado).
2. **Novos baselines** (braços de comparação, não variantes do método):
   - `StdUnstable`: `slice::sort_unstable`.
   - `StdStable`: `slice::sort`.
   - `DescReverse`: varredura O(n) — se estritamente decrescente, reverte
     in-place e então aplica Insertion Sort; senão, Insertion direto.
     (Testa se detecção trivial de run explica os ganhos em Reversed.)
   - Implementar como variantes de `SortType` (CLI ganha as opções
     automaticamente via `ValueEnum`) ou como funções separadas no
     `servicebench.rs` — preferir o que minimizar difusão; atualizar
     `nome_sort`, `aplicar_ordenacao` e a validação fora do timing.
3. **Braço `Cpre` no Criterion**: por célula (tipo, tamanho), terceiro
   benchmark cronometrando **só** `pre_processamento_simetrico` no mesmo
   pool e mesmo `iter_batched` (clone fora do timing). Permite
   $\Delta C_{\text{sort}} = C_{\text{total}} - C_{\text{pre}}$ por célula.
4. **Tamanhos**: estender `TAMANHOS` para
   `[16, 32, 64, 128, 256, 512, 1000, 5000, 10000, 100000, 1000000]`
   (quadráticos até 100k; baselines std em todos; `10000000` só para
   Merge/Quicksort/Std em avaliação de fora-da-cache — checar RAM/tempo
   antes).
5. **Exportação**: estender `SORTS/TIPOS/TAMANHOS` em
   `exportar_resultados.rs`; emitir `cpre_consolidado.csv`
   (por célula: média±IC do braço Cpre) e incluir novas topologias nas
   figuras; manter nomes de coluna existentes (compatibilidade com as
   figuras atuais).
6. Critério de aceite: `cargo bench` completo passa com
   `validar_cenario` OK em todas as células novas; companions de inversões
   gerados para Sawtooth/OrganPipe/FewRuns/Real.

## F2. Diagnóstico Quicksort + Selection (antes da matriz cheia; ~1 dia)

1. **Quicksort**: instrumentar contadores de comparações/trocas e chamadas
   recursivas (feature `#[cfg]` ou wrapper, sem alterar o algoritmo padrão);
   medir em Reversed/ordenado/Zigzag n=10⁴–10⁶: (a) contagens por n,
   (b) tempo vs `sort_unstable` no mesmo pool. Pergunta a responder com
   números: a escala ~n¹·⁴⁵ vem de particionamento degenerado, de
   profundidade de recursão, ou de outro gargalo? Registrar conclusão
   (inclusive se for patologia a corrigir — caso em que a matriz F3 usa a
   versão corrigida **e** o artigo declara a troca de versão).
2. **Selection**: `cargo asm`/`objdump` no loop interno (cmov vs. jump);
   contar atualizações de mínimo por instrumentação em
   Random vs. Reordered. Pergunta: a degradação de ~12% em Reversed
   sobrevive como efeito de preditor ou cai como artefato de compilação?
3. **Contadores de hardware**: no Linux, ativar o plumbing `perf-event`
   já existente em `service.rs` (cache-misses, branch-misses) em rodada
   piloto; no Windows, tentar AMD uProf no Ryzen. Se indisponível,
   registrar como limitação permanente (já declarado no artigo).
4. Critério de aceite: relatório de 1 página com números por (b) acima,
   decidindo o texto final do §4.3 (causa confirmada vs. hipótese mantida).

## F3. Matriz de medição (tempo de máquina; ~2–5 dias)

1. Fixar CPU (afinar governador/SMT/afinação documentada em F0) e rodar:
   `cargo bench --bench benchmark` na matriz estendida (usar filtros
   `BN_SORTS/BN_TIPOS/BN_TAMANHOS` para lotes retomáveis).
2. Multi-seed: repetir o núcleo da matriz (n=10⁴ todos; n=10⁶ quasilineares)
   com 5 pools (seeds 42–46), via parâmetro de seed no harness
   (hoje fixo em 42 — parametrizar).
3. Segunda máquina e/ou Linux: repetir pelo menos n=10⁴ completo +
   n=10⁶ quasilinear; arquivar `estimates.json` + companions + manifesto
   por máquina.
4. n=10⁷: Merge/Quicksort/Std + Cpre (checar ~40 MB por vetor i32 —
   fora da cache do 8700G); quadráticos excluídos (documentar).
5. Critério de aceite: `exportar_resultados.rs` roda sem `panic!` de
   "DADOS AUSENTES"/"COMPANION AUSENTE"; `benchmark_consolidado.csv`,
   `inversoes.csv`, `cpre_consolidado.csv` e `ganho.csv` regenerados.

## F4. Estatística pareada (~meio dia)

1. Experimento pareado offline por pool-slot k (reutilizar
   `generate_test_array` + `BenchmarkServiceBench` em example dedicado):
   para cada célula, R repetições de (puro vs. pré+sort) **no mesmo vetor**,
   estatística = log-razão por vetor; IC por bootstrap percentílico.
2. Substituir a regra "ICs não sobrepostos" por efeito pareado + IC;
   republicar significância nas Tabelas 4/6 (marcar células).
3. Quantificar variação entre pools (5 seeds): o efeito muda de sinal em
   alguma célula? (Atenção especial: Quicksort/AlmostSorted.)
4. Critério de aceite: script + CSV de efeitos arquivados; texto do §3.3
   atualizado com o procedimento exato.

## F5. Regeneração de figuras/tabelas (~meio dia)

1. Rodar export; copiar `fig_*.csv` para `artigo/bench2026/resultados/`
   com rótulos em inglês (etapa já documentada; automatizar em script
   para não repetir à mão, incluindo `fig_gain_vs_n.csv` estendido com
   novas curvas: StdUnstable como referência, Sawtooth/OrganPipe,
   n pequenos).
2. Atualizar no `main.tex`: Tabela 2 (novas linhas), nova tabela de
   baselines competitivos (separada — não mexer nas 30 linhas da Tabela 4),
   Tabela de Cpre por topologia (novo `cpre_consolidado.csv`), Fig. ganho×n
   estendida, mediana/outliers em apêndice (opcional).
3. Critério de aceite: `pdflatex+bibtex` limpo; todas as refs cruzadas
   resolvidas; nenhum número antigo restante no texto.

## F6. Reescrita do artigo (sobre os novos dados; ~1–2 dias)

1. Abstract e Conclusão: reescrever **após** F3–F4 — ganhos típicos passam
   a incluir topologias não alinhadas novas e n pequeno; declarar onde o
   método vence/perde contra `sort_unstable` (fim da lacuna "8,7×").
2. §4.3: trocar "hipótese de preditor" por resultado de F2 (confirmado ou
   descartado); Quicksort com diagnóstico de F2; Bubble com mecanismo de
   deslocamento (já no texto) revalidado nas novas topologias.
3. §2.4: tônica teórica mantida + derivação de primeira ordem
   (~n²/12) confrontada com o medido; Shell-pass e run-detection entram
   como comparação (não mais "futuro" se F3 as incluir como baselines).
4. Ameaças à validade: remover o que F0–F4 tiver resolvido; manter só o
   resíduo real.
5. Checagem anti-invenção: cada número novo no texto deve rastrear a um
   CSV/companion commitado.

## F7. Anonimização e submissão (~meio dia)

1. Repo anonimizado acessível a revisores (sem nomes/e-mails),
   com `Cargo.lock`, `rust-toolchain.toml`, scripts de ponta a ponta
   (bench → export → figuras EN → PDF) e dados brutos
   (`estimates.json`, companions) + SHA-256.
2. No artigo, trocar `[14]` para o link anonimizado (ou remover, conforme
   regra vigente do PEvaluation).
3. Limpar metadados do PDF: `\hypersetup{pdfauthor={},pdftitle={}}`,
   e pós-processar (ex.: `exiftool -all:all= -tagsfromfile @ -Title -Author`
   — validar que título/autor fiquem vazios; checar `pdfinfo`).
4. Confirmar no site do PEvaluation (logado): template vigente, limite de
   páginas, regras double-blind, prazo de aceitação (1º nov 2026 AoE),
   registro/apresentação obrigatória, política de artefatos e de
   originalidade/revisão paralela. Ajustar cabeçalho/placeholders
   ("DOI HERE", received/accepted) conforme instrução do template.
5. Teste P&B das figuras + `pdfinfo` (páginas, tamanho) + checklist final
   da Seção 10 do `docs/plan.md` (auditoria) item a item.

---

## Riscos e notas
- n=10⁷ nos quadráticos é inviável (manter exclusão documentada).
- Se F2 concluir por patologia no Quicksort, a matriz F3 deve usar a versão
  corrigida e o artigo deve datar a mudança (rastreabilidade).
- Não aplicar limites de 2022–2025 (LNCS/15 pp./pré-registro) — valem só as
   regras 2026 confirmadas em F7.
- Estimativa total: ~1 semana de trabalho + tempo de máquina do bench.
