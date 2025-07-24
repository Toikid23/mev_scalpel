// src/lib.rs

// On déclare tous nos modules principaux pour les rendre publics et
// utilisables par nos programmes binaires (main.rs, analyze_pools.rs, etc.).
pub mod config;
pub mod error;
pub mod state;
pub mod graph_engine;
pub mod data_pipeline;
pub mod decoders;
pub mod strategies;
pub mod execution;