fn dfs(graph: &[Vec<usize>], u: usize, seen: &mut [bool]) {
    seen[u] = true;
    println!("visit {u}");
    for &v in &graph[u] {
        if !seen[v] {
            dfs(graph, v, seen);
        }
    }
}

fn dfs_iter(graph: &[Vec<usize>], start: usize) {
    let mut seen = vec![false; graph.len()];
    let mut stack: Vec<(usize, usize)> = Vec::new(); // (node, next edge index)

    seen[start] = true;
    println!("visit {start}");
    stack.push((start, 0));

    while let Some((u, i)) = stack.last_mut() {
        if let Some(&v) = graph[*u].get(*i) {
            *i += 1;
            if !seen[v] {
                seen[v] = true;
                println!("visit {v}");
                stack.push((v, 0)); // dfs(v)
            } else {
                stack.pop(); // finished all edges
            }
        }
    }
}

fn main() {
    let graph = vec![vec![1, 2], vec![3], vec![3], vec![]];
    let mut seen = vec![false; graph.len()];

    dfs(&graph, 0, &mut seen);
    dfs_iter(&graph, 0);
}
