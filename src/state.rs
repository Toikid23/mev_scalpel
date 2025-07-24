// src/state.rs

use crate::decoders::Pool;
use arc_swap::ArcSwap;
use solana_sdk::pubkey::Pubkey;
use std::collections::HashMap;
use std::sync::Arc;

/// Représente une arête dans notre graphe de marché.
/// Une arête est une connexion unidirectionnelle d'un token vers un autre via un pool.
#[derive(Debug, Clone)]
pub struct Edge {
    /// L'index du token de destination dans notre liste de nœuds.
    pub destination: usize,
    /// Le pool de liquidité qui facilite cet échange.
    /// Il contient toute la logique pour calculer les prix.
    pub pool: Pool,
}

/// La structure principale de notre graphe de marché.
#[derive(Debug, Clone, Default)]
pub struct MarketGraph {
    /// Fait le lien entre la Pubkey d'un token et son index (0, 1, 2...) dans notre graphe.
    /// C'est notre annuaire de tokens.
    pub token_map: HashMap<Pubkey, usize>,

    /// La liste d'adjacence qui représente le graphe.
    /// L'index de ce vecteur correspond à l'index du token.
    /// `nodes[i]` contient un vecteur de toutes les arêtes (tous les swaps possibles)
    /// qui partent du token `i`.
    pub nodes: Vec<Vec<Edge>>,
}

/// La structure d'état global de l'application, conçue pour être partagée
/// de manière très performante entre les threads.
#[derive(Clone)]
pub struct AppState {
    /// Un pointeur intelligent vers notre graphe de marché.
    /// `Arc`: Permet à plusieurs threads de posséder une référence au graphe.
    /// `ArcSwap`: Permet de remplacer atomiquement le graphe entier par une nouvelle version
    ///            sans jamais bloquer les threads qui sont en train de le lire.
    /// C'est l'architecture idéale pour notre cas d'usage :
    /// - Le thread de données écrit (swap) une nouvelle version du graphe.
    /// - Les threads de stratégie lisent (load) la version la plus récente.
    pub graph: Arc<ArcSwap<MarketGraph>>,
}

impl AppState {
    /// Crée un nouvel état d'application avec un graphe vide.
    pub fn new() -> Self {
        Self {
            graph: Arc::new(ArcSwap::from(Arc::new(MarketGraph::default()))),
        }
    }
}