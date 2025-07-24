// src/data_pipeline/discovery/orca.rs

use anyhow::Result;
use serde::Deserialize;

const ORCA_API_URL: &str = "https://api.orca.so/v2/solana/pools";

#[derive(Deserialize, Debug)]
struct OrcaApiV2Response { data: Vec<OrcaPoolInfo>, meta: Option<ApiMeta> }
#[derive(Deserialize, Debug)]
struct ApiMeta { next: Option<String> }

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrcaPoolInfo {
    pub address: String,
    pub token_mint_a: String,
    pub token_mint_b: String,
}

pub async fn fetch_orca_pools() -> Result<Vec<OrcaPoolInfo>> {
    println!("Fetching all whirlpools from Orca V2 API (with pagination)...");
    let mut all_pools = Vec::new();
    let mut next_cursor: Option<String> = None;
    let client = reqwest::Client::new();

    loop {
        let mut url = format!("{}?size=3000", ORCA_API_URL); // On demande le max par page
        if let Some(cursor) = &next_cursor {
            url.push_str(&format!("&next={}", cursor));
        }

        let response = client.get(&url).send().await?.json::<OrcaApiV2Response>().await?;
        let num_fetched = response.data.len();
        all_pools.extend(response.data);

        if let Some(meta) = response.meta {
            if let Some(next) = meta.next {
                next_cursor = Some(next);
            } else { break; }
        } else { break; }

        if num_fetched == 0 { break; }
    }

    println!("Successfully fetched a total of {} pools from Orca.", all_pools.len());
    Ok(all_pools)
}