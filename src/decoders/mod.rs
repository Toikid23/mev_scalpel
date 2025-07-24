// src/decoders/mod.rs

use solana_sdk::pubkey::Pubkey;
use anyhow::Result;

pub mod raydium_amm;
pub mod raydium_clmm;
// pub mod orca_whirlpool; // Le fichier n'existe pas encore, mais le type oui.

#[derive(Debug, Clone)]
pub enum Pool {
    RaydiumAmm(RaydiumAmmPool),
    RaydiumClmm(RaydiumClmmPool),
    // --- AJOUT DE LA VARIANTE ---
    OrcaWhirlpool(OrcaWhirlpoolPool),
}

pub trait PoolOperations {
    fn get_mints(&self) -> (Pubkey, Pubkey);
    fn get_quote(&self, token_in_mint: &Pubkey, amount_in: u64) -> Result<u64>;
}

#[derive(Debug, Clone)]
pub struct RaydiumAmmPool {
    pub id: Pubkey, pub mint_a: Pubkey, pub mint_b: Pubkey,
    pub mint_a_reserve: u64, pub mint_b_reserve: u64,
    pub base_vault: Pubkey, pub quote_vault: Pubkey,
}

#[derive(Debug, Clone)]
pub struct RaydiumClmmPool {
    pub id: Pubkey, pub mint_a: Pubkey, pub mint_b: Pubkey,
    pub current_sqrt_price: u128, pub current_tick: i32,
}

// --- AJOUT DE LA STRUCT ---
// Squelette pour les pools Orca.
#[derive(Debug, Clone)]
pub struct OrcaWhirlpoolPool {
    pub id: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub mint_a_reserve: u64,
    pub mint_b_reserve: u64,
}