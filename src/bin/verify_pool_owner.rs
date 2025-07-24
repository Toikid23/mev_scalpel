// src/bin/verify_pool_owner.rs

use mev_scalpel::{config::Config, data_pipeline};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    println!("--- AMM V4 POOL OWNER VERIFIER ---");

    // 1. Charger la configuration (un bon RPC est recommandé, mais le public devrait marcher)
    let config = Config::load().expect("Failed to load config");
    let rpc_client = RpcClient::new(config.solana_rpc_url);
    println!("Using RPC URL: {}", rpc_client.url());

    // L'adresse du programme "Legacy AMM v4" que nous cherchons.
    let amm_v4_program_id = Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").unwrap();
    println!("Searching for a pool owned by program: {}", amm_v4_program_id);

    // 2. On télécharge la liste complète des pools.
    match data_pipeline::market_discovery::fetch_initial_markets().await {
        Ok(all_pools) => {
            println!("Fetched {} pools. Now verifying owner for each pool one by one...", all_pools.len());

            let standard_pools: Vec<_> = all_pools.into_iter().filter(|p| p.pool_type == "Standard").collect();
            println!("Found {} 'Standard' pools to check.", standard_pools.len());

            // 3. On parcourt la liste et on vérifie le propriétaire de chaque compte.
            for (index, pool_info) in standard_pools.iter().enumerate() {
                let pool_pubkey = Pubkey::from_str(&pool_info.id).unwrap();

                print!("Checking pool {}/{} ({})... ", index + 1, standard_pools.len(), pool_pubkey);

                match rpc_client.get_account(&pool_pubkey) {
                    Ok(account) => {
                        if account.owner == amm_v4_program_id {
                            println!("MATCH FOUND!");
                            println!("\n--- SUCCESS! FOUND A VALID AMM V4 POOL ---");
                            println!("Pool ID:      {}", pool_info.id);
                            println!("Pool Type:    {}", pool_info.pool_type);
                            println!("Program ID (from API): {}", pool_info.program_id);
                            println!("VERIFIED OWNER ON-CHAIN: {}", account.owner);
                            println!("Mint A:       {}", pool_info.mint_a.address);
                            println!("Mint B:       {}", pool_info.mint_b.address);
                            println!("--------------------------------------------------");
                            println!("Please use this Pool ID in the 'test_real_amm_v4.rs' binary.");
                            return; // On a trouvé ce qu'on cherchait, on arrête le programme.
                        } else {
                            println!("Owner mismatch.");
                        }
                    },
                    Err(_) => {
                        println!("RPC error or account not found.");
                    }
                }
                // Petite pause pour ne pas surcharger le RPC
                thread::sleep(Duration::from_millis(50));
            }

            println!("\n--- FAILED ---");
            println!("Scanned all 'Standard' pools and none were owned by the AMM v4 program.");
            println!("This is extremely unusual and points to a fundamental issue with the API data or our understanding.");
            println!("--------------");
        },
        Err(e) => {
            eprintln!("\nError during market discovery: {}", e);
        }
    }
}