// src/bin/test_real_amm_v4.rs

use mev_scalpel::config::Config;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    println!("--- REAL AMM V4 POOL TESTER ---");

    // 1. Charger la configuration
    let config = Config::load().expect("Failed to load config");
    let rpc_client = RpcClient::new(config.solana_rpc_url);
    println!("Using RPC URL: {}", rpc_client.url());

    // 2. Définir l'adresse du VRAI pool AMM V4 (SOL-USDC)
    let pool_pubkey = Pubkey::from_str("58fzJMbX5PatnfJPqWWsqkVFPRKptkbb5r2vCw4Qq3z9").unwrap();
    println!("Attempting to fetch pool account: {}", pool_pubkey);

    // 3. Récupérer les données brutes du compte de POOL
    match rpc_client.get_account(&pool_pubkey) {
        Ok(account) => {
            println!("Successfully fetched pool account data ({} bytes).", account.data.len());

            // 4. Utiliser notre décodeur AMM V4
            println!("Decoding pool data to find vault addresses...");
            match mev_scalpel::decoders::raydium_amm::decode_raydium_amm(&pool_pubkey, &account.data) {
                Ok(decoded_pool) => {
                    println!("\n--- SUCCESS! ---");
                    println!("Pool: {}", decoded_pool.id);
                    println!("-> Mint A (SOL): {}", decoded_pool.mint_a);
                    println!("-> Mint B (USDC): {}", decoded_pool.mint_b);
                    println!("\nFOUND VAULTS:");
                    println!("-> Base Vault (SOL): {}", decoded_pool.base_vault);
                    println!("-> Quote Vault (USDC): {}", decoded_pool.quote_vault);
                    println!("----------------");

                    println!("\n--- KNOWN ADDRESSES FOR COMPARISON ---");
                    println!("Official SOL Mint: So11111111111111111111111111111111111111112");
                    println!("Official USDC Mint: EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
                    println!("--------------------------------------\n");
                },
                Err(e) => {
                    println!("\n--- DECODING FAILED ---");
                    println!("Error: {:?}", e);
                    println!("-----------------------");
                }
            }
        },
        Err(e) => {
            println!("\n--- RPC ERROR ---");
            println!("Error: {:?}", e);
            println!("-----------------");
        }
    }
}