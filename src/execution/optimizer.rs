// src/execution/optimizer.rs

// On importe les traits et structs dont on a VRAIMENT besoin.
use crate::decoders::{Pool, PoolOperations};
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;

// La struct utilise bien la lifetime 'a pour la référence au pool.
pub struct ArbitragePath<'a> {
    pub pool: &'a Pool,
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
}

/// Simule un trade à travers un cycle d'arbitrage et retourne le profit.
// La fonction est maintenant complète et correcte.
pub fn simulate_path_profit(
    initial_amount: u64,
    path: &[ArbitragePath],
) -> Result<i64> {
    let mut current_amount = initial_amount;
    let mut current_mint = path[0].input_mint;

    for step in path {
        if step.input_mint != current_mint {
            return Err(anyhow::anyhow!("Mismatched mints in arbitrage path"));
        }

        // On utilise le trait PoolOperations pour appeler get_quote
        current_amount = step.pool.get_quote(&step.input_mint, current_amount)?;
        current_mint = step.output_mint;
    }

    Ok(current_amount as i64 - initial_amount as i64)
}

/// Trouve le montant d'entrée optimal pour maximiser le profit.
pub fn find_optimal_amount(
    path: &[ArbitragePath],
    max_amount: u64,
) -> Result<(u64, i64)> {
    if path.is_empty() {
        return Err(anyhow::anyhow!("Arbitrage path cannot be empty"));
    }

    let mut low = 0;
    let mut high = max_amount;
    let mut optimal_amount = 0;
    let mut max_profit = 0;

    println!("\n--- Starting Optimization Search (0 -> {} lamports) ---", max_amount);
    for i in 0..100 {
        if low > high {
            break;
        }

        let m1 = low + (high - low) / 3;
        let m2 = high - (high - low) / 3;

        let profit1 = simulate_path_profit(m1, path)?;
        let profit2 = simulate_path_profit(m2, path)?;

        if i < 10 {
            println!("Iter {}: m1={}, profit1={} | m2={}, profit2={}", i, m1, profit1, m2, profit2);
        }

        if profit1 > max_profit {
            max_profit = profit1;
            optimal_amount = m1;
        }
        if profit2 > max_profit {
            max_profit = profit2;
            optimal_amount = m2;
        }

        if profit1 < profit2 {
            low = m1 + 1;
        } else {
            high = m2 - 1;
        }
    }

    Ok((optimal_amount, max_profit))
}