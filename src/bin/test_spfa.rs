// src/bin/test_spfa.rs

use mev_scalpel::{config::Config, data_pipeline, graph_engine, strategies};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    println!("--- MULTI-DEX SPFA STRATEGY TESTER ---");

    let config = Config::load().expect("Failed to load config");
    let rpc_client = RpcClient::new(config.solana_rpc_url);

    // 1. Découvrir les pools de TOUS les DEX
    let unified_pools = data_pipeline::market_discovery::fetch_initial_markets().await.unwrap();

    // 2. Construire le graphe en filtrant les pools que nous savons gérer
    let (mut graph, accounts_to_hydrate) = graph_engine::build_graph_from_unified_pools(&unified_pools);

    // 3. Hydrater les pools Raydium
    if !accounts_to_hydrate.is_empty() {
        data_pipeline::data_scraper::hydrate_graph_liquidity(&mut graph, accounts_to_hydrate, &rpc_client).unwrap();
    }

    // ... (le reste du code pour lancer SPFA reste le même)
    println!("\nRunning SPFA to find arbitrage opportunities...");
    let wsol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
    if let Some(start_node_idx) = graph.token_map.get(&wsol_mint) {
        // ... (logique SPFA)
    } else {
        println!("\nWSOL not found in the filtered graph.");
    }
}