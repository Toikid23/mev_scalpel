// src/data_scraper.rs

use crate::decoders::{Pool, RaydiumAmmPool};
use crate::state::MarketGraph;
use anyhow::{Result};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::collections::{HashMap, HashSet};
use std::thread;
use std::time::Duration;

// --- Fonction pour hydrater un seul pool (pour le développement rapide) ---
pub fn hydrate_single_pool(pool: &mut RaydiumAmmPool, rpc_client: &RpcClient) -> Result<()> {
    let vaults_to_fetch = [pool.base_vault, pool.quote_vault];
    let accounts = rpc_client.get_multiple_accounts(&vaults_to_fetch)?;

    if let Some(Some(base_vault_account)) = accounts.get(0) {
        if base_vault_account.owner == spl_token::id() && base_vault_account.data.len() >= 165 {
            pool.mint_a_reserve = u64::from_le_bytes(base_vault_account.data[64..72].try_into()?);
        }
    }
    if let Some(Some(quote_vault_account)) = accounts.get(1) {
        if quote_vault_account.owner == spl_token::id() && quote_vault_account.data.len() >= 165 {
            pool.mint_b_reserve = u64::from_le_bytes(quote_vault_account.data[64..72].try_into()?);
        }
    }
    Ok(())
}


// --- Fonction pour hydrater un graphe entier (pour plus tard) ---
const BATCH_SIZE: usize = 100;
const PAUSE_DURATION: Duration = Duration::from_millis(250);

// CORRECTION: La signature de la fonction est maintenant correcte.
pub fn hydrate_graph_liquidity(
    graph: &mut MarketGraph,
    accounts_to_fetch: Vec<Pubkey>,
    rpc_client: &RpcClient,
) -> Result<()> {
    let mut all_vaults_to_fetch = HashSet::new();
    let mut pool_id_to_vaults_map = HashMap::new();

    for pool_chunk in accounts_to_fetch.chunks(BATCH_SIZE) {
        let pool_accounts = rpc_client.get_multiple_accounts(pool_chunk)?;
        for (i, maybe_account) in pool_accounts.into_iter().enumerate() {
            if let Some(account) = maybe_account {
                let pool_id = pool_chunk[i];
                if let Ok(decoded_pool) = crate::decoders::raydium_amm::decode_raydium_amm(&pool_id, &account.data) {
                    all_vaults_to_fetch.insert(decoded_pool.base_vault);
                    all_vaults_to_fetch.insert(decoded_pool.quote_vault);
                    pool_id_to_vaults_map.insert(pool_id, (decoded_pool.base_vault, decoded_pool.quote_vault));
                }
            }
        }
        thread::sleep(PAUSE_DURATION);
    }
    println!("Found {} unique vaults to fetch.", all_vaults_to_fetch.len());

    if all_vaults_to_fetch.is_empty() { return Ok(()); }

    let vault_pubkeys: Vec<Pubkey> = all_vaults_to_fetch.into_iter().collect();
    let mut vault_balances = HashMap::new();

    for vault_chunk in vault_pubkeys.chunks(BATCH_SIZE) {
        let vault_accounts_data = rpc_client.get_multiple_accounts(vault_chunk)?;
        for (i, maybe_account) in vault_accounts_data.into_iter().enumerate() {
            if let Some(account) = maybe_account {
                let vault_id = vault_chunk[i];
                if account.owner == spl_token::id() && account.data.len() >= 165 {
                    if let Ok(balance_bytes) = account.data[64..72].try_into() {
                        let balance = u64::from_le_bytes(balance_bytes);
                        vault_balances.insert(vault_id, balance);
                    }
                }
            }
        }
        thread::sleep(PAUSE_DURATION);
    }
    println!("Successfully fetched balances for {} token vaults.", vault_balances.len());

    for node_edges in graph.nodes.iter_mut() {
        for edge in node_edges.iter_mut() {
            if let Pool::RaydiumAmm(pool) = &mut edge.pool {
                if let Some((base_vault, quote_vault)) = pool_id_to_vaults_map.get(&pool.id) {
                    if let (Some(reserve_a), Some(reserve_b)) = (vault_balances.get(base_vault), vault_balances.get(quote_vault)) {
                        pool.mint_a_reserve = *reserve_a;
                        pool.mint_b_reserve = *reserve_b;
                    }
                }
            }
        }
    }
    Ok(())
}