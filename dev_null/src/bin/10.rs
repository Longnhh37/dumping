// kosaraju
fn kosaraju(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = adj.len();

    // pass 1
    let mut visited = vec![false; n];
    let mut order = Vec::with_capacity(n);

    for s in 0..n {
        if visited[s] {
            continue;
        }
        visited[s] = true;
        let mut stack = vec![(s, 0usize)];

        while let Some((u, i)) = stack.last_mut() {
            if let Some(&v) = adj[*u].get(*i) {
                *i += 1;
                if !visited[v] {
                    visited[v] = true;
                    stack.push((v, 0));
                }
            } else {
                order.push(*u);
                stack.pop();
            }
        }
    }

    // transpose
    let mut radj = vec![Vec::new(); n];
    for (u, vs) in adj.iter().enumerate() {
        for &v in vs {
            radj[v].push(u);
        }
    }

    // pass 2
    visited.fill(false);
    let mut sccs = Vec::new();

    for &s in order.iter().rev() {
        if visited[s] {
            continue;
        }
        visited[s] = true;
        let mut scc = Vec::new();
        let mut stack = vec![s];

        while let Some(u) = stack.pop() {
            scc.push(u);
            for &v in &radj[u] {
                if !visited[v] {
                    visited[v] = true;
                    stack.push(v);
                }
            }
        }
        sccs.push(scc);
    }

    sccs
}

fn main() {
    let n = 7;
    let edges = [
        (0, 1),
        (1, 2),
        (2, 0),
        (2, 3),
        (3, 4),
        (4, 5),
        (5, 3),
        (5, 6),
    ];

    let mut adj = vec![Vec::new(); n];
    for (u, v) in edges {
        adj[u].push(v);
    }

    for (i, scc) in kosaraju(&adj).iter().enumerate() {
        println!("SCC {i}: {scc:?}");
    }
}
