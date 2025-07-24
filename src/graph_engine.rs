// src/graph_engine.rs

use crate::{
    data_pipeline,
    decoders::{Pool, RaydiumAmmPool},
    state::{Edge, MarketGraph},
};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::str::FromStr;

const DEV_POOLS: &[&str] = &[
    "58oQChx4yWmvKdwLLZzBi4ChoCc2fqbAaGvVwvVoYDLw", // SOL-USDC
    "6UmmUiYoBjSrhakAobJw8BvkmJtDVxaeBtbt7rxWo1mg", // USDC-RAY
    "AVs9TA4nWDzfPJE9gGVNJMVhcQy3V9PGazuz33BfG2RA", // RAY-SOL
];

const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
const USDC_DECIMALS: u32 = 6;
const SOL_DECIMALS: u32 = 9;

/// Construit et hydrate le graphe de test (réel + faux pool).
pub fn build_hydrated_test_graph(rpc_client: &RpcClient) -> MarketGraph {
    let pool_pubkeys: Vec<Pubkey> = DEV_POOLS.iter().map(|s| Pubkey::from_str(s).unwrap()).collect();
    let pool_accounts = rpc_client.get_multiple_accounts(&pool_pubkeys).unwrap();

    let mut graph = MarketGraph::default();
    let mut token_indices = HashMap::new();
    let mut nodes: Vec<Vec<Edge>> = Vec::new();

    for (i, maybe_account) in pool_accounts.into_iter().enumerate() {
        if let Some(account) = maybe_account {
            let pool_id = pool_pubkeys[i];
            if let Ok(mut pool) = crate::decoders::raydium_amm::decode_raydium_amm(&pool_id, &account.data) {
                if data_pipeline::data_scraper::hydrate_single_pool(&mut pool, rpc_client).is_ok() && pool.mint_a_reserve > 0 {
                    let mint_a = pool.mint_a; let mint_b = pool.mint_b;
                    let idx_a = *token_indices.entry(mint_a).or_insert_with(|| { let i = nodes.len(); nodes.push(Vec::new()); i });
                    let idx_b = *token_indices.entry(mint_b).or_insert_with(|| { let i = nodes.len(); nodes.push(Vec::new()); i });
                    let pool_enum = Pool::RaydiumAmm(pool);
                    nodes[idx_a].push(Edge { destination: idx_b, pool: pool_enum.clone() });
                    nodes[idx_b].push(Edge { destination: idx_a, pool: pool_enum });
                }
            }
        }
    }

    let wsol_mint = Pubkey::from_str(SOL_MINT).unwrap();
    let usdc_mint = Pubkey::from_str(USDC_MINT).unwrap();
    if let (Some(&idx_sol), Some(&idx_usdc)) = (token_indices.get(&wsol_mint), token_indices.get(&usdc_mint)) {
        let fake_pool = Pool::RaydiumAmm(RaydiumAmmPool {
            id: Pubkey::new_unique(), mint_a: usdc_mint, mint_b: wsol_mint,
            mint_a_reserve: 149_000_000 * 10u64.pow(USDC_DECIMALS),
            mint_b_reserve: 1_000_000 * 10u64.pow(SOL_DECIMALS),
            base_vault: Pubkey::new_unique(), quote_vault: Pubkey::new_unique(),
        });
        nodes[idx_usdc].push(Edge { destination: idx_sol, pool: fake_pool.clone() });
        nodes[idx_sol].push(Edge { destination: idx_usdc, pool: fake_pool });
    }

    graph.token_map = token_indices;
    graph.nodes = nodes;
    graph
}