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
    vec![]
}
fn main() {}
