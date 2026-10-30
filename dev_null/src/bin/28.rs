use std::cmp::Reverse;
use std::collections::BinaryHeap;

// ===============================================================
// dijkstra
//      - non negative weights
// ===============================================================
type Graph = Vec<Vec<(usize, u64)>>;

fn dijkstra(graph: &Graph, src: usize) -> Vec<Option<u64>> {
    let n = graph.len();
    let mut dist: Vec<Option<u64>> = vec![None; n];
    let mut heap = BinaryHeap::new();

    dist[src] = Some(0);
    heap.push(Reverse((0u64, src)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if dist[u].is_some_and(|best| d > best) {
            continue;
        }
        for &(v, w) in &graph[u] {
            let nd = w + d;
            if dist[v].is_some_and(|best| nd < best) {
                dist[v] = Some(nd);
                heap.push(Reverse((nd, v)));
            }
        }
    }

    dist
}

// ===============================================================
// Bellman Ford
//      - negative weights
//      - detect negative cycle
// ===============================================================

#[derive(Debug, Clone, Copy)]
struct Edge {
    from: usize,
    to: usize,
    weight: i64,
}

#[derive(Debug)]
enum BellmanFordError {
    NegativeCycle,
}

fn bellman_ford(
    n: usize,
    edges: &[Edge],
    src: usize,
) -> Result<Vec<Option<i64>>, BellmanFordError> {
    let mut dist: Vec<Option<i64>> = vec![None; n];
    dist[src] = Some(0);

    for _ in 1..n {
        let mut changed = false;
        for &Edge { from, to, weight } in edges {
            if let Some(du) = dist[from] {
                let nd = du + weight;
                if dist[to].is_none_or(|dv| nd < dv) {
                    dist[to] = Some(nd);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    for &Edge { from, to, weight } in edges {
        if let Some(du) = dist[from]
            && dist[to].is_none_or(|dv| du + weight < dv)
        {
            return Err(BellmanFordError::NegativeCycle);
        }
    }

    Ok(dist)
}

// ===============================================================
// Floyd Warshall
//      - all-pairs shortest path
//      - negative weights
// ===============================================================
type Matrix = Vec<Vec<Option<i64>>>;

fn floyd_warshall(mut dist: Matrix) -> Matrix {
    let n = dist.len();

    for (i, row) in dist.iter_mut().enumerate() {
        row[i] = Some(0);
    }

    for k in 0..n {
        for i in 0..n {
            let Some(d_ik) = dist[i][k] else { continue };
            for j in 0..n {
                let Some(d_kj) = dist[k][j] else { continue };
                let through_k = d_ik + d_kj;
                if dist[i][j].is_none_or(|d_ij| through_k < d_ij) {
                    dist[i][j] = Some(through_k);
                }
            }
        }
    }

    dist
}

fn has_negative_cycle(dist: &Matrix) -> bool {
    dist.iter()
        .enumerate()
        .any(|(i, row)| row[i].is_some_and(|d| d < 0))
}

fn main() {}
