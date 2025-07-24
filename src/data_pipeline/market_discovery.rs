// src/data_pipeline/market_discovery.rs

use super::discovery::{self, GenericPoolInfo};
use anyhow::Result;

const RAYDIUM_AMM_V4_PROGRAM_ID: &str = "675kPX9MHTjS2zt1qfr1NYHuzELxQfM9H24wFSut1Mp8";
const ORCA_WHIRLPOOL_PROGRAM_ID: &str = "whirLbMiicVdio4iTSEveBxE8hSdrjvrnPAcKGAJqgM";

/// Fetches and unifies pools from all configured DEX APIs.
pub async fn fetch_initial_markets() -> Result<Vec<GenericPoolInfo>> {
    println!("Starting market discovery from all sources...");

    let (raydium_results, orca_results) = tokio::join!(
        discovery::raydium::fetch_raydium_pools(),
        discovery::orca::fetch_orca_pools()
    );

    let mut unified_pools = Vec::new();

    // Traitement des pools Raydium (SANS FILTRE)
    if let Ok(pools) = raydium_results {
        for pool in pools {
            unified_pools.push(GenericPoolInfo {
                id: pool.id,
                mint_a: pool.mint_a.address,
                mint_b: pool.mint_b.address,
                source: "Raydium".to_string(),
                program_id: pool.program_id, // On garde le program_id fourni par l'API
            });
        }
    }

    // Traitement des pools Orca
    if let Ok(pools) = orca_results {
        for pool in pools {
            unified_pools.push(GenericPoolInfo {
                id: pool.address,
                mint_a: pool.token_mint_a,
                mint_b: pool.token_mint_b,
                source: "Orca".to_string(),
                program_id: ORCA_WHIRLPOOL_PROGRAM_ID.to_string(),
            });
        }
    }

    println!("Total unified pools found from all sources: {}", unified_pools.len());
    Ok(unified_pools)
}