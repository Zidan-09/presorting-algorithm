use clap::Parser;

mod tipos;
mod gerador;
mod algoritmos;
mod service;

use tipos::{ArrayType, SortType};
use service::BenchmarkService;

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
    println!("🧪 INICIANDO EXPERIMENTO CIENTÍFICO");
    println!("Tamanho do Vetor: {}", args.size);
    println!("Tipo de Array:    {:?}", args.array);
    println!("Algoritmo:        {:?}", args.sort);
    println!("==================================================");

    let resultado = BenchmarkService::executar_teste(args.size, args.array, args.sort);

    println!("📊 RESULTADOS DO BENCHMARK:");
    println!("--------------------------------------------------");
    println!("Abordagem 1: Ordenação Pura");
    println!("  -> Tempo de Execução:             {:?}", resultado.tempo_puro);
    println!("  -> Status de Validação:           {}", if resultado.ordenacao_pura_valida { "OK ✅" } else { "FALHOU ❌" });
    println!("--------------------------------------------------");
    println!("Abordagem 2: Com Pré-processamento Simétrico");
    println!("  -> Tempo do Pré-processamento:    {:?}", resultado.tempo_pre_processamento);
    println!("  -> Tempo da Ordenação Pós-Pré:    {:?}", resultado.tempo_ordenacao_com_pre);
    println!("  -> Tempo Total Combinado:         {:?}", resultado.tempo_total_com_pre);
    println!("  -> Status de Validação:           {}", if resultado.ordenacao_pre_valida { "OK ✅" } else { "FALHOU ❌" });
    println!("==================================================");

    let t_puro = resultado.tempo_puro.as_nanos() as f64;
    let t_com_pre = resultado.tempo_total_com_pre.as_nanos() as f64;
    
    if t_com_pre < t_puro {
        let ganho = ((t_puro - t_com_pre) / t_puro) * 100.0;
        println!("🚀 O pré-processamento reduziu o tempo total em {:.2}%", ganho);
    } else {
        let perda = ((t_com_pre - t_puro) / t_puro) * 100.0;
        println!("⚠️ O pré-processamento gerou um overhead de {:.2}% neste cenário.", perda);
    }
    println!("==================================================");
}