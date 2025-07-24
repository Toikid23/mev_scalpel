// src/bin/dev_runner.rs

// --- CORRECTION : On importe `Edge` depuis le module `state` ---
use mev_scalpel::{config::Config, data_pipeline, decoders::Pool, state::{MarketGraph, Edge}};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::str::FromStr;

// --- VOS "GOLDEN SAMPLES" ---
// Remplacez ces adresses par celles que vous avez trouvées sur Solscan.
const GOLDEN_SAMPLES: &[&str] = &[
    "58fzJMbX5PatnfJPqWWsqkVFPRKptkbb5r2vCw4Qq3z9",
    "58oQChx4yWmvKdwLLZzBi4ChoCc2fqCUWBkwMihLYQo2",
    "GhpsDfY5gPX9N7Txf2Q4WvKKLUuAFhLDysZ1g6tPZ6jy",
];

#[tokio::main]
async fn main() {
    println!("--- DEVELOPMENT RUNNER (FAST STARTUP) ---");

    let config = Config::load().expect("Failed to load config");
    let rpc_client = RpcClient::new(config.solana_rpc_url);

    println!("Fetching {} golden sample pools...", GOLDEN_SAMPLES.len());
    let pool_pubkeys: Vec<Pubkey> = GOLDEN_SAMPLES.iter().map(|s| Pubkey::from_str(s).unwrap()).collect();
    let pool_accounts = rpc_client.get_multiple_accounts(&pool_pubkeys).unwrap();

    let mut graph = MarketGraph::default();
    let mut token_indices: HashMap<Pubkey, usize> = HashMap::new();
    let mut nodes: Vec<Vec<Edge>> = Vec::new();
    let mut hydrated_pools = 0;

    for (i, maybe_account) in pool_accounts.into_iter().enumerate() {
        if let Some(account) = maybe_account {
            let pool_id = pool_pubkeys[i];
            if let Ok(mut decoded_pool) = mev_scalpel::decoders::raydium_amm::decode_raydium_amm(&pool_id, &account.data) {
                if data_pipeline::data_scraper::hydrate_single_pool(&mut decoded_pool, &rpc_client).is_ok() {
                    if decoded_pool.mint_a_reserve > 0 {
                        hydrated_pools += 1;
                        let mint_a = decoded_pool.mint_a;
                        let mint_b = decoded_pool.mint_b;

                        let idx_a = *token_indices.entry(mint_a).or_insert_with(|| { let i = nodes.len(); nodes.push(Vec::new()); i });
                        let idx_b = *token_indices.entry(mint_b).or_insert_with(|| { let i = nodes.len(); nodes.push(Vec::new()); i });

                        let pool_enum = Pool::RaydiumAmm(decoded_pool);
                        nodes[idx_a].push(Edge { destination: idx_b, pool: pool_enum.clone() });
                        nodes[idx_b].push(Edge { destination: idx_a, pool: pool_enum });
                    }
                }
            }
        }
    }

    graph.token_map = token_indices;
    graph.nodes = nodes;

    println!("\n--- MINI-GRAPH BUILT SUCCESSFULLY ---");
    println!("Total pools hydrated: {}", hydrated_pools);
    println!("Total tokens in graph: {}", graph.token_map.len());
    println!("-------------------------------------\n");
}