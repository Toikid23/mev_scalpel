// src/strategies/spfa_arb.rs

use crate::decoders::PoolOperations;
use crate::state::MarketGraph;
use fixed::types::I80F48;

/// Exécute l'algorithme SPFA pour trouver des cycles de poids négatif (opportunités d'arbitrage).
pub fn find_negative_cycle(graph: &MarketGraph, start_node_idx: usize) -> Option<Vec<usize>> {
    let num_nodes = graph.nodes.len();
    if num_nodes == 0 { return None; }

    let mut dist: Vec<I80F48> = vec![I80F48::MAX; num_nodes];
    let mut predecessor: Vec<Option<usize>> = vec![None; num_nodes];
    let mut in_queue_count: Vec<usize> = vec![0; num_nodes];
    let mut queue: std::collections::VecDeque<usize> = std::collections::VecDeque::new();

    dist[start_node_idx] = I80F48::ZERO;
    queue.push_back(start_node_idx);
    in_queue_count[start_node_idx] = 1;

    const TEST_SWAP_AMOUNT: u64 = 1_000_000_000; // 1 SOL

    while let Some(u) = queue.pop_front() {
        // --- LA LIGNE FINALE ET CORRECTE (DICTÉE PAR LE COMPILATEUR) ---
        let u_mint = graph.token_map.iter().find(|&(_, &v)| v == u).unwrap().0;

        for edge in &graph.nodes[u] {
            let v = edge.destination;

            let weight = match &edge.pool {
                crate::decoders::Pool::RaydiumAmm(pool) => {
                    if let Ok(amount_out) = pool.get_quote(u_mint, TEST_SWAP_AMOUNT) {
                        if amount_out > 0 {
                            let rate = I80F48::from_num(amount_out) / I80F48::from_num(TEST_SWAP_AMOUNT);
                            let rate_f64 = rate.to_num::<f64>();
                            I80F48::from_num(-rate_f64.ln())
                        } else { I80F48::MAX }
                    } else { I80F48::MAX }
                },
                _ => I80F48::MAX,
            };

            if dist[u].checked_add(weight).unwrap_or(I80F48::MAX) < dist[v] {
                dist[v] = dist[u] + weight;
                predecessor[v] = Some(u);

                if !queue.contains(&v) {
                    queue.push_back(v);
                    in_queue_count[v] += 1;

                    if in_queue_count[v] >= num_nodes {
                        let mut path = vec![v];
                        let mut current = predecessor[v].unwrap();
                        while !path.contains(&current) {
                            path.push(current);
                            current = predecessor[current].unwrap();
                        }
                        path.push(current);
                        path.reverse();
                        let cycle_start_index = path.iter().position(|&x| x == current).unwrap_or(0);
                        let cycle = path[cycle_start_index..].to_vec();
                        return Some(cycle);
                    }
                }
            }
        }
    }

    None
}