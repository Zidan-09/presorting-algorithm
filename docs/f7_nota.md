# F7 — Anonimização e submissão (nota de fechamento)

Data: 02/10/2026. Feito técnico: higiene, pipeline, pacote anon TESTADO,
metadados do PDF, checklist §10 do `docs/plan.md`.

## Entregas técnicas (neste repo, não commitadas — ver `git status`)

- `presorting-algorithm/scripts/pipeline_f7.ps1` (+ `.sh` p/ Linux):
  F0 → bench (`smoke` default, `full` = fila F3+F4+export) → figuras EN →
  PDF. `smoke` validado em conceito (etapas = comandos já executados em
  F3–F5); `full` NÃO re-executado aqui (~15–20 h).
- `presorting-algorithm/scripts/publicar_anonimo.ps1`: monta árvore limpa
  (código+tex+dados, sem target/logs/docs internos), manifesto
  `dados_brutos/SHA256SUMS.txt` (4015 hashes: `new/estimates.json` 5 seeds
  + companions + CSVs), `git init` identidade genérica. TESTADO em
  `Temp\opencode\anonimo`: commit único `Bench 2026 Authors
  <noreply@example.com>`, `git status` limpo, sem identificadores.
  Armadilhas achadas: PS 5.1 não tolera UTF-8 em `.ps1` (convertidos p/
  ASCII), alias `Cp` sombreia função (renomeado `CopyTree`), `Copy-Item
  -Recurse` arquivo→arquivo é instável (robocopy p/ dirs).
- `README_ANON.md` (vira `README.md` no pacote), `.gitignore` (raiz).
- PDF: `pdfinfo` → Author/Title/Subject/Keywords vazios; Creator/Producer
  só toolchain; 12 pp., 661–677 KB. Tentativa de esvaziar Creator/Producer
  via `\pdfinfo` CORROMPEU o PDF (revertido; docado).
- [14] mantido como placeholder anônimo (`Anonymous repository for
  double-blind review`, sem URL) — seguro p/ double-blind até o link existir.

## Checklist §10 (plan.md) — estado

1. Site PEvaluation (template/limite/anonimato/prazo/registro) → USUÁRIO
   (logado). Template TBench já em uso; placeholders DOI/recebido mantidos.
2. Enquadramento Evaluation Science → OK (§3: cost-benefit, confundidores, H1–H3).
3. Topologias formais → OK (10 fórmulas, Tab 2).
4. Validar Quicksort → OK (F2 + §4.3).
5. Baselines → OK (Tab 4b).
6. Não-alinhadas + n pequeno → OK (4 novas + real + 16–512; motivação mantida).
7. Cpre por topologia → OK (Tab `tab:cpre`).
8. Varredura n em figura → OK (Fig gainn 16–10⁷).
9. Pareada + significância + Selection → OK (F4, `*/†`, zigzag resolvido).
10. Bubble deslocamento → OK.
11. Abstract/conclusão → OK (F6).
12. Protocolo + repo + lock + scripts → OK técnico; push anon = USUÁRIO.
13. §2.4 teórica → OK (n²/12 + breakeven ΔI≳5n).
14. Threats → OK (5 categorias, só resíduo).
15. Xrefs/[6]/[11]/[13]/nota-1 → OK (verificados: ESA/LNCS, ICALP→journal,
    Timsort-only, sem "3.6"); metadados OK; P&B por construção (marcas e
    traços distintos, tons de cinza) — teste de impressão = USUÁRIO.

## Ações do usuário (exigem login/decisão)

1. Confirmar no site do PEvaluation (logado): template, limite de páginas,
   double-blind, prazo 1º nov 2026 AoE, registro/apresentação, artefatos,
   originalidade/revisão paralela; ajustar DOI/recebido se o template mandar.
2. Criar repo anônimo vazio → rodar `publicar_anonimo.ps1` → push → colar URL
   em [14] (ou remover, conforme regra do site).
3. Teste de impressão P&B das figuras.
4. Commitar F7 (e F6/F5 se ainda pendentes) e submeter cedo (prazo = aceite).
