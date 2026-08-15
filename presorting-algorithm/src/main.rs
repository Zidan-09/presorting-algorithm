use clap::Parser;

use algoritmo::utils::{ArrayType, SortType};
use algoritmo::services::BenchmarkService;

#[derive(Parser, Debug)]
#[command(author, version, about = "Benchmark para Artigo de Ordenação")]
struct Args {
    #[arg(short = 's', long, default_value_t = 10000)]
    size: usize,

    #[arg(short = 'a', long, value_enum, default_value = "random")]
    array: ArrayType,

    #[arg(short = 'o', long, value_enum, default_value = "insertion")]
    sort: SortType,

    #[arg(long, default_value_t = 42)]
    seed: u64,

    #[arg(long, default_value_t = 7)]
    repeticoes: usize,
}


fn main() {
    let args = Args::parse();

    println!("==================================================");
    println!("🧪 INICIANDO EXPERIMENTO CIENTÍFICO COMPLETO");
    println!("Tamanho do Vetor: {}", args.size);
    println!("Tipo de Array:    {:?}", args.array);
    println!("Algoritmo:        {:?}", args.sort);
    println!("Seed:             {} (reproduzível)", args.seed);
    println!("Repetições:       {}", args.repeticoes);
    println!("==================================================");

    let res = BenchmarkService::executar_teste(args.size, args.array, args.sort, args.seed, args.repeticoes);

    println!("📊 METRICAS COLETADAS (média de {} repetições):", res.repeticoes);
    println!("--------------------------------------------------");
    println!("  -> Inversões Iniciais:            {}", res.inversoes_iniciais);
    println!("  -> Inversões Pós-Pré:             {}", res.inversoes_pos_pre_processamento);
    println!("--------------------------------------------------");
    println!("Abordagem 1: Ordenação Pura (somente o sort)");
    println!("  -> Tempo médio:                   {:?}", res.puro.tempo);
    println!("  -> Tempo mediano:                 {:?}", res.puro.tempo_mediana);
    println!("  -> Memória Alocada na Heap:       {} bytes", res.puro.memoria_alocada_bytes);
    if cfg!(target_os = "linux") {
        println!("  -> Cache Misses:                  {}", res.puro.cache_misses);
    } else {
        println!("  -> Cache Misses:                  [Disponível apenas no Linux]");
    }
    println!("  -> Validação:                     {}", if res.puro.valido { "OK ✅" } else { "FALHOU ❌" });

    println!("--------------------------------------------------");
    println!("Abordagem 2: Com Pré-processamento Simétrico (pré + sort)");
    println!("  -> Tempo do Pré-Processamento:   {:?}", res.tempo_so_pre_processamento);
    println!("  -> Tempo Total (pré + sort):     {:?}", res.com_pre.tempo);
    println!("  -> Tempo mediano:                {:?}", res.com_pre.tempo_mediana);
    println!("  -> Memória Alocada na Heap:      {} bytes", res.com_pre.memoria_alocada_bytes);
    if cfg!(target_os = "linux") {
        println!("  -> Cache Misses:                  {}", res.com_pre.cache_misses);
    }
    println!("  -> Validação:                     {}", if res.com_pre.valido { "OK ✅" } else { "FALHOU ❌" });
    println!("==================================================");

    let t_puro = res.puro.tempo.as_nanos() as f64;
    let t_total_pre = res.com_pre.tempo.as_nanos() as f64;
    let diferenca = ((t_puro - t_total_pre) / t_puro) * 100.0;

    if t_total_pre < t_puro {
        println!("🚀 O algoritmo proposto reduziu o tempo em {:.2}%", diferenca);
    } else {
        println!("⚠️ O algoritmo proposto aumentou o tempo em {:.2}%", diferenca.abs());
    }
    println!("==================================================");

    println!("📈 DADOS POR REPETIÇÃO (seed={}, n={}):", res.seed, res.repeticoes);
    println!("rep\tpuro_ns\tcom_pre_ns\tpre_ns");
    for (i, ((tp, tc), tpre)) in res.puro.tempos.iter()
        .zip(res.com_pre.tempos.iter())
        .zip(res.tempos_so_pre_processamento.iter())
        .enumerate()
    {
        println!("{}\t{}\t{}\t{}", i + 1, tp.as_nanos(), tc.as_nanos(), tpre.as_nanos());
    }

    println!("==================================================");
    println!("CSV_PURO,{}", res.puro.tempos.iter().map(|t| t.as_nanos().to_string()).collect::<Vec<_>>().join(","));
    println!("CSV_COM_PRE,{}", res.com_pre.tempos.iter().map(|t| t.as_nanos().to_string()).collect::<Vec<_>>().join(","));
    println!("CSV_PRE_SOZINHO,{}", res.tempos_so_pre_processamento.iter().map(|t| t.as_nanos().to_string()).collect::<Vec<_>>().join(","));
    println!("CSV_MEMORIA_PURO,{}", res.puro.memoria_alocada_bytes);
    println!("CSV_MEMORIA_COM_PRE,{}", res.com_pre.memoria_alocada_bytes);
    if cfg!(target_os = "linux") {
        println!("CSV_CACHE_PURO,{}", res.puro.cache_misses);
        println!("CSV_CACHE_COM_PRE,{}", res.com_pre.cache_misses);
    }
    println!("CSV_INVERSOES_INICIAIS,{}", res.inversoes_iniciais);
    println!("CSV_INVERSOES_POS_PRE,{}", res.inversoes_pos_pre_processamento);
}
