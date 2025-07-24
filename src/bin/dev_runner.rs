// src/bin/dev_runner.rs

use mev_scalpel::{
    config::Config,
    execution::optimizer::{self, ArbitragePath},
    graph_engine, // On importe le module graph_engine
    state::MarketGraph,
    strategies,
    decoders::PoolOperations
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use anyhow::Result;

const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
const SOL_DECIMALS: u32 = 9;

#[tokio::main]
async fn main() {
    println!("--- DEVELOPMENT RUNNER (STABILIZED) ---");

    let config = Config::load().expect("Failed to load config");
    let rpc_client = RpcClient::new(config.solana_rpc_url);

    // 1. On appelle le graph_engine pour construire notre graphe de test.
    // Cette fonction contient maintenant toute la logique de fetch, decode, et hydrate.
    let graph = graph_engine::build_hydrated_test_graph(&rpc_client);

    println!("\n--- MINI-GRAPH BUILT SUCCESSFULLY ---");
    println!("Total pools hydrated: {}", graph.nodes.iter().map(|e| e.len()).sum::<usize>() / 2);
    println!("Total tokens in graph: {}", graph.token_map.len());
    println!("-------------------------------------\n");

    // 2. Lancement de la stratégie
    let wsol_mint = Pubkey::from_str(SOL_MINT).unwrap();
    if let Some(start_node_idx) = graph.token_map.get(&wsol_mint) {
        println!("Running SPFA starting from WSOL...");
        match strategies::spfa_arb::find_negative_cycle(&graph, *start_node_idx) {
            Some(cycle_indices) => {
                println!("\n--- !!! OPPORTUNITY FOUND !!! ---");
                if let Ok(path) = build_path_for_optimizer(&graph, &cycle_indices) {
                    let max_trade_amount = 100 * 10u64.pow(SOL_DECIMALS);
                    if let Ok((optimal_amount, max_profit)) = optimizer::find_optimal_amount(&path, max_trade_amount) {
                        println!("\n--- OPTIMIZATION COMPLETE ---");
                        println!("Optimal trade amount: {} SOL", optimal_amount as f64 / 10f64.powi(SOL_DECIMALS as i32));
                        println!("Predicted profit:     {} SOL", max_profit as f64 / 10f64.powi(SOL_DECIMALS as i32));
                        println!("-----------------------------");
                    }
                }
            },
            None => println!("--- No opportunity found. ---"),
        }
    }
}

/// Construit le chemin pour l'optimiseur.
fn build_path_for_optimizer<'a>(
    graph: &'a MarketGraph,
    cycle_indices: &[usize],
) -> Result<Vec<ArbitragePath<'a>>> {
    let mut path = Vec::new();
    for i in 0..cycle_indices.len() - 1 {
        let u_idx = cycle_indices[i];
        let v_idx = cycle_indices[i + 1];

        let edge = graph.nodes[u_idx]
            .iter()
            .find(|e| e.destination == v_idx)
            .ok_or_else(|| anyhow::anyhow!("Could not find edge in graph"))?;

        let (input_mint, output_mint) = edge.pool.get_mints();
        let u_mint = graph.token_map.iter().find(|&(_, &v)| v == u_idx).unwrap().0;

        path.push(ArbitragePath {
            pool: &edge.pool,
            input_mint: *u_mint,
            output_mint: if input_mint == *u_mint { output_mint } else { input_mint },
        });
    }
    Ok(path)
}