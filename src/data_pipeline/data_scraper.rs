// src/data_pipeline/data_scraper.rs

use crate::decoders::RaydiumAmmPool;
use anyhow::Result;
use solana_client::rpc_client::RpcClient;

pub fn hydrate_single_pool(pool: &mut RaydiumAmmPool, rpc_client: &RpcClient) -> Result<()> {
    let vaults_to_fetch = [pool.base_vault, pool.quote_vault];
    let accounts = rpc_client.get_multiple_accounts(&vaults_to_fetch)?;

    if let Some(Some(account)) = accounts.get(0) {
        if account.owner == spl_token::id() && account.data.len() >= 165 {
            pool.mint_a_reserve = u64::from_le_bytes(account.data[64..72].try_into()?);
        }
    }
    if let Some(Some(account)) = accounts.get(1) {
        if account.owner == spl_token::id() && account.data.len() >= 165 {
            pool.mint_b_reserve = u64::from_le_bytes(account.data[64..72].try_into()?);
        }
    }
    Ok(())
}