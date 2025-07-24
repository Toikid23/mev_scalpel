// src/decoders/raydium_amm.rs

// On importe SEULEMENT le trait `PoolOperations` depuis le module parent (`super` = `decoders`).
use super::PoolOperations;
use anyhow::{anyhow, Result};
use bytemuck::{from_bytes, Pod, Zeroable};
use solana_sdk::pubkey::Pubkey;

// --- DÉFINITION DE LA STRUCT PUBLIQUE ---
// C'est la struct que le reste de notre application utilisera.
// Elle est maintenant définie ici, dans son propre module.
#[derive(Debug, Clone)]
pub struct RaydiumAmmPool {
    pub id: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub mint_a_reserve: u64,
    pub mint_b_reserve: u64,
    pub base_vault: Pubkey,
    pub quote_vault: Pubkey,
}


// --- STRUCTURES DE DÉCODAGE PRIVÉES ---
// Celles-ci ne sont utilisées qu'à l'intérieur de ce fichier.

#[repr(C, packed)]
#[derive(Clone, Copy, Zeroable, Pod, Debug)]
struct Fees {
    min_separate_numerator: u64, min_separate_denominator: u64,
    trade_fee_numerator: u64, trade_fee_denominator: u64,
    pnl_numerator: u64, pnl_denominator: u64,
    swap_fee_numerator: u64, swap_fee_denominator: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Zeroable, Pod, Debug)]
struct StateData {
    need_take_pnl_coin: u64, need_take_pnl_pc: u64, total_pnl_pc: u64,
    total_pnl_coin: u64, pool_open_time: u64, padding: [u64; 2],
    orderbook_to_init_time: u64, swap_coin_in_amount: u128,
    swap_pc_out_amount: u128, swap_acc_pc_fee: u64, swap_pc_in_amount: u128,
    swap_coin_out_amount: u128, swap_acc_coin_fee: u64,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Zeroable, Pod, Debug)]
struct AmmInfo {
    status: u64, nonce: u64, order_num: u64, depth: u64, coin_decimals: u64,
    pc_decimals: u64, state: u64, reset_flag: u64, min_size: u64,
    vol_max_cut_ratio: u64, amount_wave: u64, coin_lot_size: u64,
    pc_lot_size: u64, min_price_multiplier: u64, max_price_multiplier: u64,
    sys_decimal_value: u64, fees: Fees, state_data: StateData,
    coin_vault: Pubkey, pc_vault: Pubkey, coin_vault_mint: Pubkey,
    pc_vault_mint: Pubkey, lp_mint: Pubkey, open_orders: Pubkey, market: Pubkey,
    market_program: Pubkey, target_orders: Pubkey, padding1: [u64; 8],
    amm_owner: Pubkey, lp_amount: u64, client_order_id: u64,
    recent_epoch: u64, padding2: u64,
}

/// La fonction de décodage, qui est maintenant publique.
pub fn decode_raydium_amm(id: &Pubkey, data: &[u8]) -> Result<RaydiumAmmPool> {
    let amm_info_size = std::mem::size_of::<AmmInfo>();
    if data.len() < amm_info_size {
        return Err(anyhow!("Data too short for AmmInfo"));
    }
    let amm_info_slice = &data[data.len() - amm_info_size..];
    let amm_info: &AmmInfo = from_bytes(amm_info_slice);

    Ok(RaydiumAmmPool {
        id: *id,
        mint_a: amm_info.coin_vault_mint,
        mint_b: amm_info.pc_vault_mint,
        mint_a_reserve: 0,
        mint_b_reserve: 0,
        base_vault: amm_info.coin_vault,
        quote_vault: amm_info.pc_vault,
    })
}

// L'implémentation du trait pour notre struct publique.
impl PoolOperations for RaydiumAmmPool {
    fn get_mints(&self) -> (Pubkey, Pubkey) {
        (self.mint_a, self.mint_b)
    }

    fn get_quote(&self, token_in_mint: &Pubkey, amount_in: u64) -> Result<u64> {
        if self.mint_a_reserve == 0 || self.mint_b_reserve == 0 {
            return Err(anyhow!("Pool has no liquidity data yet."));
        }
        let (in_reserve, out_reserve) = if *token_in_mint == self.mint_a {
            (self.mint_a_reserve, self.mint_b_reserve)
        } else if *token_in_mint == self.mint_b {
            (self.mint_b_reserve, self.mint_a_reserve)
        } else {
            return Err(anyhow!("Input token does not belong to this pool."));
        };
        let fee_numerator = 25;
        let fee_denominator = 10000;
        let amount_in_with_fee = amount_in * (fee_denominator - fee_numerator);
        let numerator = (amount_in_with_fee as u128) * (out_reserve as u128);
        let denominator = ((in_reserve as u128) * (fee_denominator as u128)) + (amount_in_with_fee as u128);
        let amount_out = (numerator / denominator) as u64;
        Ok(amount_out)
    }
}