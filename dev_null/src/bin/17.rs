fn hamiltonian_path_backtrack(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    fn dfs(u: usize, adj: &[Vec<usize>], visited: &mut [bool], path: &mut Vec<usize>) -> bool {
        visited[u] = true;
        path.push(u);

        if path.len() == adj.len() {
            return true;
        }

        for &v in &adj[u] {
            if !visited[v] && dfs(v, adj, visited, path) {
                return true;
            }
        }

        path.pop();
        visited[u] = false;
        false
    }

    let n = adj.len();
    let mut visited = vec![false; n];
    let mut path = Vec::with_capacity(n);

    (0..n)
        .any(|s| dfs(s, adj, &mut visited, &mut path))
        .then_some(path)
}

fn main() {
    // Directed: 0->1, 1->2, 2->3, 0->2, 3->1
    let adj = vec![vec![1, 2], vec![2], vec![3], vec![1]];
    println!("backtrack: {:?}", hamiltonian_path_backtrack(&adj)); // Some([0, 1, 2, 3])

    // Undirected: star graph (0 nối 1, 2, 3) -> không có Hamiltonian path
    let mut star = vec![vec![]; 4];
    for v in 1..4 {
        star[0].push(v);
        star[v].push(0);
    }
    println!("backtrack: {:?}", hamiltonian_path_backtrack(&star)); // None
}
