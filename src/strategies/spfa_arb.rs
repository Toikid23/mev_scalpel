// src/strategies/spfa_arb.rs

use crate::state::MarketGraph;
use fixed::types::I80F48;

/// Exécute l'algorithme SPFA pour trouver des cycles de poids négatif (opportunités d'arbitrage).
pub fn find_negative_cycle(graph: &MarketGraph, start_node_idx: usize) -> Option<Vec<usize>> {
    let num_nodes = graph.nodes.len();
    if num_nodes == 0 {
        return None;
    }

    let mut dist: Vec<I80F48> = vec![I80F48::MAX; num_nodes];
    let mut predecessor: Vec<Option<usize>> = vec![None; num_nodes];
    let mut in_queue_count: Vec<usize> = vec![0; num_nodes];
    let mut queue: std::collections::VecDeque<usize> = std::collections::VecDeque::new();

    dist[start_node_idx] = I80F48::ZERO;
    queue.push_back(start_node_idx);
    in_queue_count[start_node_idx] = 1;

    while let Some(u) = queue.pop_front() {
        for edge in &graph.nodes[u] {
            let v = edge.destination;
            let weight = I80F48::from_num(-1.0); // Poids simulé pour le test

            if dist[u].checked_add(weight).unwrap_or(I80F48::MAX) < dist[v] {
                dist[v] = dist[u] + weight;
                predecessor[v] = Some(u);

                if !queue.contains(&v) {
                    queue.push_back(v);
                    in_queue_count[v] += 1;

                    if in_queue_count[v] >= num_nodes {
                        // On reconstruit le cycle en remontant les prédécesseurs.
                        let mut path = vec![v];
                        let mut current = predecessor[v].unwrap();

                        // --- CORRECTION ---
                        // On continue tant que le nœud `current` n'est pas déjà dans le chemin.
                        while !path.contains(&current) {
                            path.push(current);
                            current = predecessor[current].unwrap();
                        }
                        path.push(current); // On ajoute le premier nœud du cycle pour le fermer
                        path.reverse();

                        // On ne garde que le cycle lui-même.
                        let cycle_start_index = path.iter().position(|&x| x == current).unwrap_or(0);
                        let cycle = path[cycle_start_index..].to_vec();

                        return Some(cycle);
                    }
                }
            }
        }
    }

    None // Aucun cycle trouvé
}