use clap::Parser;

use algoritmo::tipos::{ArrayType, SortType};
use algoritmo::service::BenchmarkService;

#[derive(Parser, Debug)]
#[command(author, version, about = "Benchmark para Artigo de Ordenação")]
struct Args {
    #[arg(short = 's', long, default_value_t = 10000)]
    size: usize,

    #[arg(short = 'a', long, value_enum, default_value = "random")]
    array: ArrayType,

    #[arg(short = 'o', long, value_enum, default_value = "insertion")]
    sort: SortType,
}


fn main() {
    let args = Args::parse();

    println!("==================================================");
    println!("🧪 INICIANDO EXPERIMENTO CIENTÍFICO COMPLETO");
    println!("Tamanho do Vetor: {}", args.size);
    println!("Tipo de Array:    {:?}", args.array);
    println!("Algoritmo:        {:?}", args.sort);
    println!("==================================================");

    let res = BenchmarkService::executar_teste(args.size, args.array, args.sort);

    println!("📊 METRICAS COLETADAS:");
    println!("--------------------------------------------------");
    println!("  -> Inversões Iniciais:            {}", res.inversoes_iniciais);
    println!("--------------------------------------------------");
    println!("Abordagem 1: Ordenação Pura");
    println!("  -> Tempo:                         {:?}", res.puro.tempo);
    println!("  -> Memória Alocada na Heap:       {} bytes", res.puro.memoria_alocada_bytes);
    if cfg!(target_os = "linux") {
        println!("  -> Cache Misses:                  {}", res.puro.cache_misses);
    } else {
        println!("  -> Cache Misses:                  [Disponível apenas no Linux]");
    }
    println!("  -> Validação:                     {}", if res.puro.valido { "OK ✅" } else { "FALHOU ❌" });

    println!("--------------------------------------------------");
    println!("Abordagem 2: Com Pré-processamento Simétrico");
    println!("  -> Tempo do Pré-Processamento:   {:?}", res.tempo_so_pre_processamento);
    println!("  -> Tempo da Ordenação Pós-Pré:    {:?}", res.com_pre.tempo);
    println!("  -> Tempo Total Combinado:         {:?}", res.tempo_so_pre_processamento + res.com_pre.tempo);
    println!("  -> Memória Alocada na Heap:       {} bytes", res.com_pre.memoria_alocada_bytes);
    if cfg!(target_os = "linux") {
        println!("  -> Cache Misses:                  {}", res.com_pre.cache_misses);
    }
    println!("  -> Inversões Pos-Pré:             {}", res.inversoes_pos_pre_processamento);
    println!("  -> Validação:                     {}", if res.com_pre.valido { "OK ✅" } else { "FALHOU ❌" });
    println!("==================================================");

    let t_puro = res.puro.tempo.as_nanos() as f64;
    let t_total_pre = (res.tempo_so_pre_processamento + res.com_pre.tempo).as_nanos() as f64;
    let diferenca = ((t_puro - t_total_pre) / t_puro) * 100.0;

    if t_total_pre < t_puro {
        println!("🚀 O algoritmo proposto reduziu o tempo em {:.2}%", diferenca);
    } else {
        println!("⚠️ O algoritmo proposto aumentou o tempo em {:.2}%", diferenca.abs());
    }
    println!("==================================================");
}