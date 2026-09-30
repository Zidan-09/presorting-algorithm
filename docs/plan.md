# Adaptation Plan for Bench 2026 / PEvaluation Submission

## 1. Event Requirements (from `docs/`)

### Submission Language
- English only

### Submission Format
- Official PEvaluation manuscript template (TBench-template.cls)
- Template options: `namedate,webpdf,contemporary,large`
- Must compile to printable PDF

### Review Policy (Double-Blind)
- **Must anonymize completely:**
  - Author names → remove
  - Affiliations → remove
  - Acknowledgments → remove
  - Funding information → remove
  - Other identifying information → remove
- Self-citations: adapt to third person (e.g., "Nascimento and Rocha [2026]" → "Anonymous Authors [2026]" or similar)

### Submission Requirements
- Original work, not published elsewhere
- Not under review elsewhere
- PDF with page numbers
- Figures/tables readable in black & white
- References: complete author lists, avoid unnecessary "et al."

### Topics of Interest (Relevant to This Work)
- Evaluation methodology and theoretical foundations
- Benchmark design and construction
- Algorithm performance analysis
- Performance modeling and bottleneck analysis
- Scalability and efficiency evaluation
- Reproducible measurement practices
- Benchmark-driven algorithm evaluation
- Standardized evaluation of algorithms
- Accuracy-cost and efficiency-quality trade-off analysis
- Reproducibility of algorithm evaluation

## 2. Template Requirements (from `artigo/modelos/TBench_template_6.24/`)

### Document Class
```latex
\documentclass[namedate,webpdf,contemporary,large]{TBench-template}
```

### Required Metadata Commands
- `\journaltitle{BenchCouncil Transactions on Benchmarks, Standards and Evaluations}`
- `\DOI{DOI HERE}` (placeholder for submission)
- `\copyrightyear{2026}`
- `\pubyear{2026}`
- `\appnotes{Original Article}`
- `\firstpage{1}`
- `\title[Short Title]{Full Title}`
- `\author[affil_id]{Author Name}` (will be anonymized)
- `\address[affil_id]{\orgdiv{...}, \orgname{...}, \orgaddress{...}}` (will be anonymized)
- `\corresp[*]{Corresponding author. \href{mailto:...}{...}}` (will be anonymized)
- `\received{Date}{Month}{Year}`
- `\accepted{Date}{Month}{Year}`
- `\abstract{...}` (single paragraph, no citations)
- `\keywords{Keyword1, Keyword2, Keyword3, Keyword4}`

### Structure
- `\maketitle` after metadata
- Sections: `\section`, `\subsection`, `\subsubsection`, `\paragraph`
- Tables: `tabular*` with `@{\extracolsep\fill}`, booktabs (`\toprule`, `\midrule`, `\botrule`), `tablenotes` for footnotes
- Figures: `graphicx` package, `figure`/`figure*` environments
- Algorithms: `algorithm` + `algorithmicx` + `algpseudocode` (not `algorithm2e`)
- Bibliography: `\bibliographystyle{TBench}`, `\bibliography{reference}`

### Formatting Details
- Page numbers in footer (centered)
- Two-column option available via class
- Captions: "Fig." and "Table" prefixes
- References: numbered, square brackets, sort&compress

## 3. Current Article Structure (from `artigo/main.tex`)

| Section | Content |
|---------|---------|
| Title | Portuguese: "Análise do Impacto de um Pré-Processamento Simétrico O(n) na Redução de Inversões e Eficiência de Algoritmos de Ordenação Adaptativos e Não Adaptativos" |
| Authors | Samuel da Penha Nascimento, Francisco das Chagas Rocha (UESPI) |
| Abstract | English + Portuguese (resumo) |
| 1. Introdução | Context, motivation, research question, structure |
| 2. Fundamentação Teórica | Inversões e Ordenação Adaptativa; Algoritmos Avaliados; Pré-Processamento Simétrico; Trabalhos Relacionados |
| 3. Métodos e Técnicas | Classificação da pesquisa; Configuração; Especificação dos Algoritmos; Protocolo Experimental; Formalização do Pré-Processamento; Ameaças à Validade |
| 4. Resultados e Discussão | Redução de Inversões; Impacto no Tempo; Tempo Total e Análise Individual |
| 5. Considerações Finais | Summary, limitations, future work |
| References | 40+ entries in `referencias.bib` (SBC style, author-year) |
| Figures | 3 TikZ/pgfplots figures (inversões, precusto, tempos) |
| Tables | 4 tables (ambiente, datasets, inversões, tempos n=10k, tempos n=1M, precusto) |
| Algorithm | 1 algorithm (Pre-processamento Simétrico) using algorithm2e |

## 4. Proposed Structure Mapping

| Original Content | New Section (Template) | Action |
|------------------|------------------------|--------|
| Title (PT) | `\title[Short]{Impact of O(n) Symmetric Preprocessing on Inversion Reduction and Sorting Efficiency}` | Translate, adapt |
| Authors/Affiliations | `\author`, `\address` | **Anonymize** (placeholder) |
| Abstract (EN) + Resumo (PT) | `\abstract{...}` | Keep English abstract only, translate PT parts, single paragraph |
| Keywords | `\keywords{...}` | Extract from abstract, 4-5 keywords |
| 1. Introdução | 1. Introduction | Translate, adapt |
| 2.1 Inversões e Ordenação Adaptativa | 2.1 Inversions and Adaptive Sorting | Translate |
| 2.2 Algoritmos Avaliados | 2.2 Evaluated Algorithms | Translate |
| 2.3 Pré-Processamento Simétrico | 2.3 Symmetric Preprocessing | Translate |
| 2.4 Trabalhos Relacionados | 2.4 Related Work | Translate |
| 3.1 Classificação da Pesquisa | 3.1 Research Classification | Condense/integrate into 3.1 |
| 3.2 Configuração | 3.2 Experimental Configuration | Translate, adapt table format |
| 3.3 Especificação dos Algoritmos | 3.3 Algorithm Specifications | Translate, adapt algorithm format |
| 3.4 Protocolo Experimental | 3.4 Experimental Protocol | Translate |
| 3.5 Formalização do Pré-Processamento | 3.5 Symmetric Preprocessing Formalization | Translate, convert to algorithmicx |
| 3.6 Ameaças à Validade | 3.6 Threats to Validity | Translate |
| 4.1 Redução de Inversões | 4.1 Inversion Reduction Analysis | Translate |
| 4.2 Impacto no Tempo | 4.2 Execution Time Impact | Translate |
| 4.3 Tempo Total e Análise | 4.3 Comprehensive Time Analysis | Translate |
| 5. Considerações Finais | 5. Conclusion | Translate |
| References | References | Convert to TBench style (numbered), update citations |
| Figures (3) | Figures | Recreate with template-compatible TikZ, ensure B&W readable |
| Tables (6) | Tables | Convert to `tabular*` + booktabs + `tablenotes` |

## 5. Anonymization Checklist

- [ ] Remove author names from `\author`
- [ ] Remove affiliations from `\address`
- [ ] Remove email from `\corresp`
- [ ] Remove acknowledgments section (not present but verify)
- [ ] Remove funding information (not present but verify)
- [ ] Anonymize self-citation: `nascimento_repositorio_2026` → reference as "Anonymous Authors [2026]" or similar
- [ ] Check text for implicit identification (e.g., "our previous work", "in our repository")
- [ ] Check figure/table captions for identifying info
- [ ] Check bibliography for self-citations that reveal identity
- [ ] Remove/replace repository URL if it identifies authors
- [ ] Check LaTeX comments for identifying info

## 6. Content Reduction Strategy (if page limit exceeded)

Priority for condensation (lowest impact first):
1. **Research classification subsection (3.1)** - can be condensed to a paragraph
2. **Related work (2.4)** - keep only most relevant citations
3. **Algorithm implementation details (3.3)** - condense descriptions, keep pseudocode
4. **Threats to validity (3.6)** - shorten
5. **Detailed per-topology discussion in Results** - summarize patterns, keep key numbers
6. **Future work** - condense to 2-3 items

**Must preserve entirely:**
- All numerical results (tables 4.1, 4.2, 4.3, 4.4, 4.5)
- All statistical criteria (95% CI non-overlap)
- Algorithm pseudocode
- Experimental configuration (hardware, software, seeds)
- Core contributions and conclusions

## 7. Risks and Manual Verification Items

| Risk | Description | Mitigation |
|------|-------------|------------|
| Page limit unknown | Template doesn't specify max pages | Target ~10-12 pages, check compiled PDF |
| Algorithm environment | Original uses `algorithm2e`, template expects `algorithmicx` | Convert pseudocode syntax |
| Bibliography style | Original SBC (author-year), template uses TBench (numbered) | Regenerate .bib with numbered style, update all `\cite` |
| Figure format | TikZ/pgfplots with groupplots | Ensure compiles with template class, test B&W readability |
| Table width | Some tables wide (6 columns) | Use `tabular*{\columnwidth}` or `table*` for full width |
| Self-citation anonymization | Repository reference identifies authors | Replace with generic reference or remove |
| Double-blind compliance | Must ensure no implicit identification | Full text review after adaptation |
| Compilation errors | Template class has specific requirements | Test compile iteratively |
| Figure data files | CSV files referenced relatively (`resultados/fig_*.csv`) | Copy data files to new directory or embed coordinates |

---

## Implementation Steps

1. **Create directory structure:** `artigo/bench2026/` with `main.tex`, `reference.bib`, `img/` (for TBench logo), data files
2. **Copy template class:** Copy `TBench-template.cls` to `artigo/bench2026/`
3. **Create `main.tex`** following template structure with adapted content
4. **Create `reference.bib`** with all references converted to numbered format
5. **Copy/regenerate figure data** (CSV files for pgfplots)
6. **Compile and fix** iteratively: `pdflatex main; bibtex main; pdflatex main; pdflatex main`
7. **Verify** all checklist items
8. **Generate final report**