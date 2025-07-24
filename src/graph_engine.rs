// src/graph_engine.rs

use crate::data_pipeline::discovery::GenericPoolInfo;
use crate::decoders::{Pool, RaydiumAmmPool, OrcaWhirlpoolPool};
use crate::state::{Edge, MarketGraph};
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::str::FromStr;

// Les adresses officielles des programmes que nous gérons
const RAYDIUM_AMM_V4_PROGRAM_ID: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
const ORCA_WHIRLPOOL_PROGRAM_ID: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";

pub fn build_graph_from_unified_pools(pools: &[GenericPoolInfo]) -> (MarketGraph, Vec<Pubkey>) {
    let mut graph = MarketGraph::default();
    let mut token_indices: HashMap<Pubkey, usize> = HashMap::new();
    let mut nodes: Vec<Vec<Edge>> = Vec::new();
    let mut accounts_to_hydrate = Vec::new(); // Uniquement pour Raydium

    for pool_info in pools {
        let pool_id = Pubkey::from_str(&pool_info.id).unwrap();
        let mint_a = Pubkey::from_str(&pool_info.mint_a).unwrap();
        let mint_b = Pubkey::from_str(&pool_info.mint_b).unwrap();

        let idx_a = *token_indices.entry(mint_a).or_insert_with(|| { let i = nodes.len(); nodes.push(Vec::new()); i });
        let idx_b = *token_indices.entry(mint_b).or_insert_with(|| { let i = nodes.len(); nodes.push(Vec::new()); i });

        // --- LOGIQUE MULTI-DEX CORRIGÉE ---
        let pool_enum = if pool_info.program_id == RAYDIUM_AMM_V4_PROGRAM_ID {
            accounts_to_hydrate.push(pool_id);
            Some(Pool::RaydiumAmm(RaydiumAmmPool {
                id: pool_id, mint_a, mint_b,
                mint_a_reserve: 0, mint_b_reserve: 0,
                base_vault: Pubkey::default(), quote_vault: Pubkey::default(),
            }))
        } else if pool_info.program_id == ORCA_WHIRLPOOL_PROGRAM_ID {
            Some(Pool::OrcaWhirlpool(OrcaWhirlpoolPool {
                id: pool_id, mint_a, mint_b,
                mint_a_reserve: 0, mint_b_reserve: 0,
            }))
        } else {
            None // On ignore tous les autres types de pools
        };

        if let Some(pool) = pool_enum {
            nodes[idx_a].push(Edge { destination: idx_b, pool: pool.clone() });
            nodes[idx_b].push(Edge { destination: idx_a, pool });
        }
    }

    graph.token_map = token_indices;
    graph.nodes = nodes;

    let total_pools_in_graph = graph.nodes.iter().map(|e| e.len()).sum::<usize>() / 2;
    println!("\nGraph built with {} total pools ({} from Raydium to hydrate).", total_pools_in_graph, accounts_to_hydrate.len());
    (graph, accounts_to_hydrate)
}