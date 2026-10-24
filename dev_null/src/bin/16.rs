// adj[u] = adj[vec<v, edge_id>]
type UGraph = Vec<Vec<(usize, usize)>>;

fn build_undirected(n: usize, edges: &[(usize, usize)]) -> UGraph {
    let mut adj: UGraph = vec![Vec::new(); n];
    for (id, &(u, v)) in edges.iter().enumerate() {
        adj[u].push((v, id));
        adj[v].push((u, id));
    }

    adj
}

fn hierholzer_undirected_dfs(mut adj: UGraph, start: usize, edges: usize) -> Option<Vec<usize>> {
    fn dfs(u: usize, adj: &mut UGraph, used: &mut [bool], path: &mut Vec<usize>) {
        while let Some((v, id)) = adj[u].pop() {
            if used[id] {
                continue;
            }
            used[id] = true;
            dfs(v, adj, used, path);
        }
        path.push(u);
    }

    let mut used = vec![false; edges];
    let mut path = Vec::with_capacity(edges + 1);

    dfs(start, &mut adj, &mut used, &mut path);

    if path.len() != edges + 1 {
        return None;
    }

    path.reverse();
    Some(path)
}

fn hierholzer_undirected_iter_dfs(
    mut adj: UGraph,
    start: usize,
    edges: usize,
) -> Option<Vec<usize>> {
    let mut used = vec![false; edges];
    let mut stack = vec![start];
    let mut path = Vec::with_capacity(edges + 1);

    while let Some(&u) = stack.last() {
        let mut next = None;
        while let Some((v, id)) = adj[u].pop() {
            if !used[id] {
                used[id] = true;
                next = Some(v);
                break;
            }
        }
        match next {
            Some(u) => stack.push(u),
            None => {
                path.push(u);
                stack.pop();
            }
        }
    }

    if path.len() != edges + 1 {
        return None;
    }
    path.reverse();
    Some(path)
}

// Eulerian circuit (undirected): all vertexes have even degree
fn undirected_circuit(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    if edges.is_empty() {
        return Some(Vec::new());
    }
    let adj = build_undirected(n, edges);
    if adj.iter().any(|a| a.len() & 1 != 0) {
        return None;
    }
    let start = adj.iter().position(|a| !a.is_empty())?;
    hierholzer_undirected_iter_dfs(adj, start, edges.len())
}

// Eulerian path (undiretcted): 0 or 2 vertex with odd degree
fn undirected_path(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    if edges.is_empty() {
        return Some(Vec::new());
    }
    let adj = build_undirected(n, edges);
    let odd: Vec<usize> = (0..n).filter(|&u| adj[u].len() & 1 == 1).collect();

    let start = match odd.as_slice() {
        [] => adj.iter().position(|a| !a.is_empty())?, // a circuit
        [a, _] => *a,
        _ => return None,
    };
    hierholzer_undirected_iter_dfs(adj, start, edges.len())
}

fn main() {
    // Undirected: hai tam giác chung vertex 1 (circuit)
    let e = [(0, 1), (1, 2), (2, 0), (1, 3), (3, 4), (4, 1)];
    println!("undirected circuit: {:?}", undirected_circuit(5, &e));
    println!("undirected path:    {:?}", undirected_path(5, &e));

    // Undirected path (2 vertex lẻ): 0-1, 1-2
    let e2 = [(0, 1), (1, 2)];
    println!("undirected circuit: {:?}", undirected_circuit(3, &e2)); // None
    println!("undirected path:    {:?}", undirected_path(3, &e2)); // Some([0, 1, 2]) hoặc [2, 1, 0]
}
