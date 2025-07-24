// src/bin/analyze_pools.rs

// On importe les modules dont on a besoin depuis notre librairie `mev_scalpel`
use mev_scalpel::{config::Config, data_pipeline, decoders::RaydiumAmmPool};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    println!("--- GOLDEN SAMPLE PIPELINE TEST ---");

    // 1. Charger la configuration
    let config = Config::load().expect("Failed to load config");
    let rpc_client = RpcClient::new(config.solana_rpc_url);

    // 2. Définir notre "Golden Sample" : le pool Raydium AMM v4 SOL-USDC
    // C'est un pool dont nous sommes certains que c'est un AMM v4.
    let pool_id = Pubkey::from_str("58fzJMbX5PatnfJPqWWsqkVFPRKptkbb5r2vCw4Qq3z9").unwrap();
    println!("Testing with known valid pool: {}", pool_id);

    // 3. Récupérer les données du compte de pool
    let pool_account = rpc_client.get_account(&pool_id).expect("Failed to get pool account");
    println!("Successfully fetched pool account data.");

    // 4. Décoder le pool pour trouver les vaults
    // CORRECTION: On utilise le chemin correct `mev_scalpel::decoders::...`
    let mut decoded_pool = mev_scalpel::decoders::raydium_amm::decode_raydium_amm(&pool_id, &pool_account.data)
        .expect("Failed to decode pool");
    println!("Successfully decoded pool. Vaults found: {}, {}", decoded_pool.base_vault, decoded_pool.quote_vault);

    // 5. Hydrater ce pool unique avec les soldes des vaults
    // CORRECTION: On utilise le chemin correct `data_pipeline::data_scraper::...`
    data_pipeline::data_scraper::hydrate_single_pool(&mut decoded_pool, &rpc_client)
        .expect("Failed to hydrate pool");
    println!("Successfully hydrated pool with vault balances.");

    // 6. Afficher le résultat final
    println!("\n--- FINAL RESULT ---");
    println!("Pool: {}", decoded_pool.id);
    println!("Mint A (SOL): {}", decoded_pool.mint_a);
    println!("  -> Reserve: {}", decoded_pool.mint_a_reserve);
    println!("Mint B (VINE): {}", decoded_pool.mint_b);
    println!("  -> Reserve: {}", decoded_pool.mint_b_reserve);
    println!("--------------------");
}