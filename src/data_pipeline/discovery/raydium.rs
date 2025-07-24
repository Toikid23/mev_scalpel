// src/data_pipeline/discovery/raydium.rs

use anyhow::Result;
use serde::Deserialize;

// --- STRUCTS (Version Finale, Robuste et Flexible) ---

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RaydiumApiV3Response<T> {
    success: bool,
    data: Option<T>,
    msg: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ApiPoolsData {
    pub count: i64,
    pub data: Vec<PoolInfo>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolInfo {
    pub id: String,
    pub program_id: String,
    #[serde(rename = "type")]
    pub pool_type: String,
    pub mint_a: MintInfo,
    pub mint_b: MintInfo,
    // CORRECTION FINALE ET DÉFINITIVE: 'config' est aussi optionnel.
    pub config: Option<PoolConfig>,
    pub observation_id: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MintInfo {
    pub address: String,
    pub program_id: String,
    pub decimals: i32,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolConfig {
    pub index: i32,
    pub protocol_fee_rate: i64,
    pub trade_fee_rate: i64,
}

const PAGE_SIZE: i32 = 1000;

/// Fetches all market pools from Raydium's V3 API, handling pagination.
pub async fn fetch_raydium_pools() -> Result<Vec<PoolInfo>> {
    let client = reqwest::Client::new();
    let mut all_pools: Vec<PoolInfo> = Vec::new();
    let mut current_page = 1;

    loop {
        let url = format!(
            "https://api-v3.raydium.io/pools/info/list?poolType=all&poolSortField=default&sortType=desc&pageSize={}&page={}",
            PAGE_SIZE, current_page
        );

        // --- AMÉLIORATION : On logue avant la requête pour voir ce qu'on fait ---
        println!("Fetching page {}...", current_page);

        let response = client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("API request failed with status: {}", response.status()));
        }

        let raw_text = response.text().await?;
        let response_body: RaydiumApiV3Response<ApiPoolsData> = match serde_json::from_str(&raw_text) {
            Ok(body) => body,
            Err(e) => {
                println!("--- FAILED TO DECODE JSON ---");
                println!("Raw JSON response (first 2000 chars): {}", &raw_text[..std::cmp::min(2000, raw_text.len())]);
                println!("-----------------------------");
                return Err(e.into());
            }
        };

        if !response_body.success {
            let error_msg = response_body.msg.unwrap_or_else(|| "Unknown API error".to_string());
            return Err(anyhow::anyhow!("Raydium API returned an error: {}", error_msg));
        }

        if let Some(api_data) = response_body.data {
            if current_page == 1 {
                println!("Total pools available according to API: {}", api_data.count);
            }
            let num_pools_on_page = api_data.data.len();
            all_pools.extend(api_data.data);

            // --- AMÉLIORATION : On logue le résultat de la page ---
            println!("-> Page {} fetched with {} pools. Total so far: {}", current_page, num_pools_on_page, all_pools.len());

            if num_pools_on_page < PAGE_SIZE as usize {
                break;
            }
        } else {
            break;
        }
        current_page += 1;
    }
    Ok(all_pools)
}