// src/data_pipeline/discovery/mod.rs

pub mod orca;
pub mod raydium;

// --- Structure Unifiée ---
// Représente les informations minimales dont nous avons besoin d'un pool,
// quelle que soit sa source (Raydium API, Orca API, etc.).
#[derive(Debug, Clone)]
pub struct GenericPoolInfo {
    pub id: String,
    pub mint_a: String,
    pub mint_b: String,
    pub source: String, // "Raydium" ou "Orca"
    pub program_id: String,
}