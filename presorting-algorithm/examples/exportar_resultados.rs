use serde::Deserialize;
use std::fs;
use std::path::Path;

// ---------------------------------------------------------------------------
// Caminhos reais do Criterion (diretório gerado pelo harness, minúsculo).
// O harness escreve os companions de inversões ("_companion") e os marcadores
// de validação ("_validacao") em target/criterion/... — a partir deles as
// tabelas de inversões são construídas SEM usar dados antigos do CLI.
// ---------------------------------------------------------------------------
const RAIZ_CRITERION: &str = "target/criterion";
const GRUPO_CRITERION: &str = "experimentos_ordenacao";
const RAIZ_SAIDA: &str = "../artigo/resultados";
const RAIZ_CLI: &str = "../artigo/resultados/cli";

const SORTS: [&str; 5] = ["merge", "quick", "insertion", "bubble", "selection"];
const TIPOS: [&str; 6] = ["random", "turtles", "zigzag", "almostsorted", "duplicates", "inverted"];
const TAMANHOS: [usize; 5] = [1000, 5000, 10000, 100000, 1000000];
const TAMANHOS_CLI: [usize; 6] = [1000, 5000, 10000, 20000, 100000, 1000000];

#[derive(Deserialize)]
struct Estimates {
    mean: ValorComCI,
    median: ValorComCI,
    #[serde(rename = "median_abs_dev")]
    mad: ValorComCI,
    #[serde(rename = "std_dev")]
    std: ValorComCI,
}

#[derive(Deserialize)]
struct ValorComCI {
    #[serde(rename = "point_estimate")]
    valor: f64,
    #[serde(rename = "confidence_interval")]
    ic: Intervalo,
}

#[derive(Deserialize)]
struct Intervalo {
    #[serde(rename = "lower_bound")]
    inferior: f64,
    #[serde(rename = "upper_bound")]
    superior: f64,
}

#[derive(Deserialize)]
struct Amostra {
    iters: Vec<f64>,
    times: Vec<f64>,
}

fn nome_sorte(s: &str) -> &'static str {
    match s {
        "merge" => "Merge",
        "quick" => "Quicksort",
        "insertion" => "Inserção",
        "bubble" => "Bubblesort",
        "selection" => "Seleção",
        _ => "?",
    }
}

fn nome_tipo(t: &str) -> &'static str {
    match t {
        "random" => "Aleatório",
        "turtles" => "Tartarugas",
        "zigzag" => "Zigue-zague",
        "almostsorted" => "Quase ordenado",
        "duplicates" => "Duplicados",
        "inverted" => "Invertido",
        _ => "?",
    }
}

/// Matriz planejada: O(n²) não é medido em n = 1.000.000 (ver docs/plan.md).
fn celula_planejada(sort: &str, tamanho: usize) -> bool {
    if tamanho >= 1_000_000 {
        matches!(sort, "merge" | "quick")
    } else {
        true
    }
}

/// Escrita atômica: grava em arquivo temporário e depois renomeia.
fn gravar(path: &str, conteudo: &str) {
    let tmp = format!("{path}.tmp");
    fs::write(&tmp, conteudo).unwrap();
    if Path::new(path).exists() {
        fs::remove_file(path).unwrap();
    }
    fs::rename(tmp, path).unwrap();
}

fn ler_estimates(dir: &str) -> Option<Estimates> {
    let p = format!("{dir}/new/estimates.json");
    let texto = fs::read_to_string(&p).ok()?;
    serde_json::from_str(&texto).ok()
}

fn quartis(dir: &str) -> (f64, f64, f64, f64, f64) {
    let p = format!("{dir}/new/sample.json");
    let texto = match fs::read_to_string(&p) {
        Ok(t) => t,
        Err(_) => return (0.0, 0.0, 0.0, 0.0, 0.0),
    };
    let amostra: Amostra = match serde_json::from_str(&texto) {
        Ok(a) => a,
        Err(_) => return (0.0, 0.0, 0.0, 0.0, 0.0),
    };
    let mut por_iter: Vec<f64> = amostra
        .iters
        .iter()
        .zip(amostra.times.iter())
        .filter(|(i, _)| **i > 0.0)
        .map(|(i, t)| t / i)
        .collect();
    por_iter.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if por_iter.is_empty() {
        return (0.0, 0.0, 0.0, 0.0, 0.0);
    }
    let q = |p: f64| {
        let idx = ((por_iter.len() as f64 - 1.0) * p).round() as usize;
        por_iter[idx.min(por_iter.len() - 1)]
    };
    (q(0.0), q(0.25), q(0.5), q(0.75), q(1.0))
}

fn ler_cli_serie(arquivo: &str, chave: &str) -> Vec<f64> {
    let texto = fs::read_to_string(format!("{RAIZ_CLI}/{arquivo}")).unwrap_or_default();
    for linha in texto.lines() {
        if let Some(valor) = linha.strip_prefix(chave) {
            return valor
                .trim()
                .split(',')
                .filter_map(|v| v.parse::<f64>().ok())
                .collect();
        }
    }
    Vec::new()
}

fn media(vals: &[f64]) -> f64 {
    if vals.is_empty() {
        0.0
    } else {
        vals.iter().sum::<f64>() / vals.len() as f64
    }
}

/// Lê o companion de inversões do harness (arrays do próprio benchmark).
fn ler_companion_inversoes(tipo: &str, tamanho: usize) -> Option<(f64, f64)> {
    let p = format!(
        "{RAIZ_CRITERION}/{GRUPO_CRITERION}/_companion/inversoes_{tipo}_{tamanho}.csv"
    );
    let texto = fs::read_to_string(&p).ok()?;
    let mut iniciais = Vec::new();
    let mut pos_pre = Vec::new();
    for linha in texto.lines().skip(1) {
        let cols: Vec<&str> = linha.split(',').collect();
        if cols.len() >= 3
            && let (Ok(i), Ok(p)) = (cols[1].parse::<f64>(), cols[2].parse::<f64>())
        {
            iniciais.push(i);
            pos_pre.push(p);
        }
    }
    if iniciais.is_empty() {
        None
    } else {
        Some((media(&iniciais), media(&pos_pre)))
    }
}

fn mil(n: f64) -> String {
    let s = format!("{:.0}", n.round());
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            out.push('.');
        }
        out.push(c);
    }
    out
}

fn decimal(x: f64, casas: usize) -> String {
    format!("{:.1$}", x, casas).replace('.', ",")
}

fn principal() {
    use std::collections::HashMap;

    fs::create_dir_all(RAIZ_SAIDA).unwrap();

    let dir_grupo = format!("{RAIZ_CRITERION}/{GRUPO_CRITERION}");

    // ---------------------------------------------------------------
    // 1) Benchmark consolidado (Criterion — dados novos)
    // ---------------------------------------------------------------
    let mut csv_bench = String::from("sort,tipo,tamanho,branch,mean_ns,ci_lo_ns,ci_hi_ns,median_ns,mad_ns,std_ns,min_ns,q1_ns,q3_ns,max_ns\n");
    let mut csv_ganho = String::from("sort,tipo,tamanho,ganho_pct\n");
    let mut linhas_t4 = Vec::new();
    let mut puro_10k: HashMap<(String, String), f64> = HashMap::new();
    let mut pre_10k: HashMap<(String, String), f64> = HashMap::new();
    let mut ganho_10k: HashMap<(String, String), f64> = HashMap::new();
    let mut celulas_executadas: Vec<String> = Vec::new();
    let mut celulas_faltando: Vec<String> = Vec::new();

    for sort in SORTS {
        for tipo in TIPOS {
            let dir_base = format!("{dir_grupo}/{sort}_{tipo}");
            for tamanho in TAMANHOS {
                if !celula_planejada(sort, tamanho) {
                    continue;
                }
                let dir_puro = format!("{dir_base}/tamanho_{tamanho}_puro");
                let dir_pre = format!("{dir_base}/tamanho_{tamanho}_com_pre");
                let puro = ler_estimates(&dir_puro);
                let pre = ler_estimates(&dir_pre);
                if puro.is_none() || pre.is_none() {
                    celulas_faltando.push(format!("{sort}_{tipo}_{tamanho}"));
                    continue;
                }
                let p = puro.unwrap();
                let c = pre.unwrap();
                let (min_p, q1_p, _, q3_p, max_p) = quartis(&dir_puro);
                let (min_c, q1_c, _, q3_c, max_c) = quartis(&dir_pre);

                csv_bench.push_str(&format!(
                    "{sort},{tipo},{tamanho},puro,{:.0},{:.0},{:.0},{:.0},{:.0},{:.0},{:.0},{:.0},{:.0},{:.0}\n",
                    p.mean.valor, p.mean.ic.inferior, p.mean.ic.superior, p.median.valor, p.mad.valor, p.std.valor, min_p, q1_p, q3_p, max_p
                ));
                csv_bench.push_str(&format!(
                    "{sort},{tipo},{tamanho},com_pre,{:.0},{:.0},{:.0},{:.0},{:.0},{:.0},{:.0},{:.0},{:.0},{:.0}\n",
                    c.mean.valor, c.mean.ic.inferior, c.mean.ic.superior, c.median.valor, c.mad.valor, c.std.valor, min_c, q1_c, q3_c, max_c
                ));
                let ganho = (p.mean.valor - c.mean.valor) / p.mean.valor * 100.0;
                csv_ganho.push_str(&format!("{sort},{tipo},{tamanho},{:.2}\n", ganho));
                celulas_executadas.push(format!("{sort}_{tipo}_{tamanho}"));
                if tamanho == 10000 {
                    puro_10k.insert((sort.to_string(), tipo.to_string()), p.mean.valor / 1000.0);
                    pre_10k.insert((sort.to_string(), tipo.to_string()), c.mean.valor / 1000.0);
                    ganho_10k.insert((sort.to_string(), tipo.to_string()), ganho);
                    let p_hw = (p.mean.ic.superior - p.mean.ic.inferior) / 2.0 / 1000.0;
                    let c_hw = (c.mean.ic.superior - c.mean.ic.inferior) / 2.0 / 1000.0;
                    let sinal = if ganho >= 0.0 { "+" } else { "-" };
                    linhas_t4.push(format!(
                        "{} & {} & {} $\\pm$ {} & {} $\\pm$ {} & {}{}\\% \\\\",
                        nome_sorte(sort),
                        nome_tipo(tipo),
                        decimal(p.mean.valor / 1000.0, 2),
                        decimal(p_hw, 2),
                        decimal(c.mean.valor / 1000.0, 2),
                        decimal(c_hw, 2),
                        sinal,
                        decimal(ganho.abs(), 1)
                    ));
                }
            }
        }
    }

    if !celulas_faltando.is_empty() {
        panic!(
            "DADOS AUSENTES no Criterion (matriz planejada) — {}\ncélulas: {}",
            celulas_faltando.len(),
            celulas_faltando.join(", ")
        );
    }

    // ---------------------------------------------------------------
    // 2) Inversões (companion do harness — mesmos arrays do benchmark)
    // ---------------------------------------------------------------
    let mut csv_inversoes = String::from("tipo,tamanho,inversoes_iniciais,inversoes_pos_pre,reducao_pct\n");
    let mut linhas_t3 = Vec::new();
    let mut inv_10k: HashMap<String, (f64, f64)> = HashMap::new();
    for tipo in TIPOS {
        for &tamanho in &[1000usize, 5000, 10000, 100000, 1000000] {
            match ler_companion_inversoes(tipo, tamanho) {
                Some((ini, pos)) => {
                    let reducao = if ini > 0.0 { (ini - pos) / ini * 100.0 } else { 0.0 };
                    csv_inversoes.push_str(&format!(
                        "{tipo},{tamanho},{ini:.0},{pos:.0},{reducao:.2}\n"
                    ));
                    if tamanho == 10000 {
                        inv_10k.insert(tipo.to_string(), (ini, pos));
                        linhas_t3.push(format!(
                            "{} & {} & {} & {}\\% \\\\",
                            nome_tipo(tipo),
                            mil(ini),
                            mil(pos),
                            decimal(reducao, 1)
                        ));
                    }
                }
                None => {
                    if celula_planejada("merge", tamanho) {
                        panic!("COMPANION DE INVERSÕES AUSENTE para tipo={tipo} tamanho={tamanho}");
                    }
                }
            }
        }
    }

    // ---------------------------------------------------------------
    // 3) Custo do pré-processamento (CLI — seed 42; médias das repetições)
    // ---------------------------------------------------------------
    let mut csv_pre_custo = String::new();
    let mut precost_por_tamanho: HashMap<usize, Vec<f64>> = HashMap::new();
    let mut linhas_t5 = Vec::new();
    for tipo in TIPOS {
        for &tamanho in &TAMANHOS_CLI {
            let arquivo = format!("{tipo}_{tamanho}.txt");
            let pre = ler_cli_serie(&arquivo, "CSV_PRE_SOZINHO,");
            if pre.is_empty() {
                panic!("SHH CLI AUSENTE para tipo={tipo} tamanho={tamanho} (arquivo {arquivo})");
            }
            let pre_med = media(&pre);
            precost_por_tamanho.entry(tamanho).or_default().push(pre_med);
        }
    }
    let mut precost_pares: Vec<(usize, f64)> = Vec::new();
    for (tamanho, vals) in precost_por_tamanho.iter() {
        let media_pre = media(vals);
        csv_pre_custo.push_str(&format!("{tamanho},{media_pre:.0}\n"));
        precost_pares.push((*tamanho, media_pre));
    }
    precost_pares.sort_by_key(|(tamanho, _)| *tamanho);
    for (tamanho, media_pre) in &precost_pares {
        let por_elem = media_pre / *tamanho as f64;
        linhas_t5.push(format!(
            "{} & {} & {} \\\\",
            mil(*tamanho as f64),
            decimal(media_pre / 1000.0, 2),
            decimal(por_elem, 2)
        ));
    }

    // ---------------------------------------------------------------
    // 4) Cruzamento CLI (sort @ 10.000; médias das repetições)
    // ---------------------------------------------------------------
    let mut csv_cross = String::from("sort,tipo,tamanho,puro_ns,com_pre_ns,pre_ns\n");
    for sort in SORTS {
        for tipo in TIPOS {
            let arquivo = format!("{sort}_{tipo}_10000.txt");
            let puro = ler_cli_serie(&arquivo, "CSV_PURO,");
            let com = ler_cli_serie(&arquivo, "CSV_COM_PRE,");
            let pre = ler_cli_serie(&arquivo, "CSV_PRE_SOZINHO,");
            if puro.is_empty() {
                panic!("SHH CLI AUSENTE para {arquivo} (CSV_PURO vazio)");
            }
            let mp = media(&puro);
            let mc = if com.is_empty() { 0.0 } else { media(&com) };
            let mpr = if pre.is_empty() { 0.0 } else { media(&pre) };
            csv_cross.push_str(&format!("{sort},{tipo},10000,{mp:.0},{mc:.0},{mpr:.0}\n"));
        }
    }

    // ---------------------------------------------------------------
    // 5) Figuras
    // ---------------------------------------------------------------
    let mut csv_fig_inversoes = String::from("tipo,iniciais,pospre\n");
    for tipo in TIPOS {
        if let Some((ini, pos)) = inv_10k.get(tipo) {
            csv_fig_inversoes.push_str(&format!("{},{:.0},{:.0}\n", nome_tipo(tipo), ini, pos));
        }
    }

    for sort in SORTS {
        let mut csv = String::from("tipo,puro_us,com_pre_us\n");
        for tipo in TIPOS {
            let p = puro_10k.get(&(sort.to_string(), tipo.to_string())).copied().unwrap_or(0.0);
            let c = pre_10k.get(&(sort.to_string(), tipo.to_string())).copied().unwrap_or(0.0);
            csv.push_str(&format!("{},{:.3},{:.3}\n", nome_tipo(tipo), p, c));
        }
        gravar(&format!("{RAIZ_SAIDA}/fig_tempos_{sort}.csv"), &csv);
    }

    let mut csv_fig_ganho = String::from("tipo,merge,quick,insertion,bubble,selection\n");
    for tipo in TIPOS {
        let gs: Vec<String> = SORTS
            .iter()
            .map(|s| {
                let g = ganho_10k.get(&(s.to_string(), tipo.to_string())).copied().unwrap_or(0.0);
                format!("{g:.2}")
            })
            .collect();
        csv_fig_ganho.push_str(&format!("{},{}\n", nome_tipo(tipo), gs.join(",")));
    }

    let mut csv_fig_precusto = String::from("tamanho,pre_us\n");
    for (tamanho, media_pre) in &precost_pares {
        csv_fig_precusto.push_str(&format!("{},{:.3}\n", tamanho, media_pre / 1000.0));
    }

    // ---------------------------------------------------------------
    // 6) Gravação atômica dos artefatos
    // ---------------------------------------------------------------
    gravar(&format!("{RAIZ_SAIDA}/fig_inversoes.csv"), &csv_fig_inversoes);
    gravar(&format!("{RAIZ_SAIDA}/fig_ganho.csv"), &csv_fig_ganho);
    gravar(&format!("{RAIZ_SAIDA}/fig_precusto.csv"), &csv_fig_precusto);
    gravar(&format!("{RAIZ_SAIDA}/benchmark_consolidado.csv"), &csv_bench);
    gravar(&format!("{RAIZ_SAIDA}/ganho.csv"), &csv_ganho);
    gravar(&format!("{RAIZ_SAIDA}/inversoes.csv"), &csv_inversoes);
    gravar(&format!("{RAIZ_SAIDA}/pre_custo.csv"), &csv_pre_custo);
    gravar(&format!("{RAIZ_SAIDA}/cli_cross.csv"), &csv_cross);

    let tabelas = format!(
        r#"%%
%% Tabela 3: redução de inversões (tamanho 10.000; média dos 50 vetores do pool)
%%
\begin{{table}}[ht]
\centering
\caption{{Redução de inversões proporcionada pelo pré-processamento simétrico em vetores de 10.000 elementos (média dos 50 vetores usados no benchmark).}}
\label{{tab:inversoes}}
\begin{{tabular}}{{lrrr}}
\hline
\textbf{{Tipo}} & \textbf{{Inversões}} & \textbf{{Pós-pré}} & \textbf{{Redução}} \\
\hline
{body3}
\hline
\end{{tabular}}
\end{{table}}
%%
%% Tabela 4: tempos médios e ganho (tamanho 10.000)
%%
\begin{{table}}[ht]
\centering
\caption{{Tempo médio por execução (em \textmu{{}}s, média $\pm$ meia-largura do IC 95\%) e ganho do pré-processamento para vetores de 10.000 elementos. Fonte: Criterion, 50 amostras por cenário.}}
\label{{tab:tempos}}
\small
\begin{{tabular}}{{llrrr}}
\hline
\textbf{{Algoritmo}} & \textbf{{Tipo}} & \textbf{{Puro}} & \textbf{{Com pré}} & \textbf{{Ganho}} \\
\hline
{body4}
\hline
\end{{tabular}}
\end{{table}}
%%
%% Tabela 5: custo do pré-processamento (escala; medido via CLI, seed 42)
%%
\begin{{table}}[ht]
\centering
\caption{{Custo médio do pré-processamento simétrico em função do tamanho do vetor (média sobre os seis tipos; medido com a ferramenta CLI do projeto, seed 42).}}
\label{{tab:precusto}}
\begin{{tabular}}{{rrr}}
\hline
\textbf{{Tamanho}} & \textbf{{Pré (\textmu{{}}s)}} & \textbf{{ns/elemento}} \\
\hline
{body5}
\hline
\end{{tabular}}
\end{{table}}
"#,
        body3 = linhas_t3.join("\n"),
        body4 = linhas_t4.join("\n"),
        body5 = linhas_t5.join("\n"),
    );
    gravar(&format!("{RAIZ_SAIDA}/tabelas.tex"), &tabelas);

    // ---------------------------------------------------------------
    // 7) Manifesto (reprodutibilidade)
    // ---------------------------------------------------------------
    let commit = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "desconhecido".to_string());
    let mut manifest = String::new();
    manifest.push_str(&format!("células executadas: {}\n", celulas_executadas.len()));
    manifest.push_str(&format!("commit: {commit}\n"));
    manifest.push_str("matriz: O(n²) até n=100000; merge/quick até n=1000000\n");
    gravar(&format!("{RAIZ_SAIDA}/manifesto.txt"), &manifest);

    println!(
        "OK: artefatos gravados em {RAIZ_SAIDA} ({obj} cenários; {cels} células)",
        obj = TAMANHOS.len() * SORTS.len() * TIPOS.len(),
        cels = celulas_executadas.len()
    );
}

fn main() {
    principal();
}