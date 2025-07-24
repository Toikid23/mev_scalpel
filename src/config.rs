// src/config.rs

use serde::Deserialize;
use anyhow::Result;

// Une struct qui contient toute la configuration nécessaire pour notre bot.
// `#[derive(Deserialize)]` permet à la librairie `envy` de peupler
// automatiquement cette struct depuis les variables d'environnement.
#[derive(Deserialize, Debug)]
pub struct Config {
    // L'URL du noeud RPC Solana que nous allons interroger.
    pub solana_rpc_url: String,
    // Plus tard, nous ajouterons ici la clé privée du trader, etc.
    // pub trader_private_key: String,
}

impl Config {
    /// Charge la configuration depuis les variables d'environnement.
    /// Lit d'abord le fichier .env, puis utilise `envy` pour parser.
    pub fn load() -> Result<Self> {
        // Charge les variables du fichier .env dans l'environnement du processus.
        dotenvy::dotenv().ok();

        // Demande à `envy` de peupler une struct `Config` depuis les variables d'environnement.
        // `envy` est pratique car il gère la conversion des types.
        let config = envy::from_env::<Config>()?;

        Ok(config)
    }
}

