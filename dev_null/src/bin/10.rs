// iterative dfs
fn kosaraju1(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
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

fn kosaraju2(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
    // pass 1
    fn dfs1(u: usize, adj: &[Vec<usize>], visited: &mut [bool], order: &mut Vec<usize>) {
        visited[u] = true;
        for &v in &adj[u] {
            if !visited[v] {
                dfs1(v, adj, visited, order);
            }
        }
        order.push(u);
    }

    // pass 2
    fn dfs2(u: usize, radj: &[Vec<usize>], visited: &mut [bool], scc: &mut Vec<usize>) {
        visited[u] = true;
        scc.push(u);
        for &v in &radj[u] {
            if !visited[v] {
                dfs2(v, radj, visited, scc);
            }
        }
    }

    let n = adj.len();
    let mut visited = vec![false; n];
    let mut order = Vec::with_capacity(n);

    // pass 1
    for s in 0..n {
        if !visited[s] {
            dfs1(s, adj, &mut visited, &mut order);
        }
    }

    // transpose graph
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
        if !visited[s] {
            let mut scc = Vec::new();
            dfs2(s, &radj, &mut visited, &mut scc);
            sccs.push(scc);
        }
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

    for (i, scc) in kosaraju1(&adj).iter().enumerate() {
        println!("SCC {i}: {scc:?}");
    }

    // avoid stack overflow
    let res = std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(move || kosaraju2(&adj))
        .unwrap()
        .join()
        .unwrap();

    for (i, scc) in res.iter().enumerate() {
        println!("SCC {i}: {scc:?}");
    }
}
