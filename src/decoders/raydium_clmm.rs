// src/decoders/raydium_clmm.rs

// On importe SEULEMENT ce dont on a besoin du module parent.
use super::PoolOperations;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;

// La VRAIE définition de la struct, rendue publique.
#[derive(Debug, Clone)]
pub struct RaydiumClmmPool {
    pub id: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub current_sqrt_price: u128,
    pub current_tick: i32,
}

// L'implémentation du trait (avec des placeholders pour l'instant)
impl PoolOperations for RaydiumClmmPool {
    fn get_mints(&self) -> (Pubkey, Pubkey) { (self.mint_a, self.mint_b) }
    fn get_quote(&self, _token_in_mint: &Pubkey, _amount_in: u64) -> Result<u64> { Ok(0) } // Placeholder
}