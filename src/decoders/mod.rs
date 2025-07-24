// src/decoders/mod.rs

use solana_sdk::pubkey::Pubkey;
use anyhow::Result;

// 1. Déclarer les modules
pub mod raydium_amm;
pub mod raydium_clmm;

// 2. Rendre publiques les structs définies dans les enfants
pub use raydium_amm::RaydiumAmmPool;
pub use raydium_clmm::RaydiumClmmPool;

// 3. Définir l'enum qui n'utilise QUE les structs que nous avons.
#[derive(Debug, Clone)]
pub enum Pool {
    RaydiumAmm(RaydiumAmmPool),
    RaydiumClmm(RaydiumClmmPool),
}

// 4. Définir le Trait
pub trait PoolOperations {
    fn get_mints(&self) -> (Pubkey, Pubkey);
    fn get_quote(&self, token_in_mint: &Pubkey, amount_in: u64) -> Result<u64>;
}

// 5. Implémenter le Trait pour l'Enum
impl PoolOperations for Pool {
    fn get_mints(&self) -> (Pubkey, Pubkey) {
        match self {
            Pool::RaydiumAmm(pool) => pool.get_mints(),
            Pool::RaydiumClmm(pool) => pool.get_mints(),
        }
    }

    fn get_quote(&self, token_in_mint: &Pubkey, amount_in: u64) -> Result<u64> {
        match self {
            Pool::RaydiumAmm(pool) => pool.get_quote(token_in_mint, amount_in),
            Pool::RaydiumClmm(pool) => pool.get_quote(token_in_mint, amount_in),
        }
    }
}