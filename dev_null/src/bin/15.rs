// ==================================================================
// Directed
// ==================================================================

fn hierholzer_directed(mut adj: Vec<Vec<usize>>, start: usize, edges: usize) -> Option<Vec<usize>> {
    let mut stack = vec![start];
    let mut path = Vec::with_capacity(edges + 1);

    while let Some(&v) = stack.last() {
        if let Some(u) = adj[v].pop() {
            stack.push(u);
        } else {
            path.push(v);
            stack.pop();
        }
    }

    if path.len() != edges + 1 {
        return None;
    }
    path.reverse();
    Some(path)
}

fn degrees(adj: &[Vec<usize>]) -> (Vec<usize>, Vec<usize>, usize) {
    let mut in_deg = vec![0; adj.len()];
    let out_deg: Vec<usize> = adj.iter().map(Vec::len).collect();
    for &v in adj.iter().flatten() {
        in_deg[v] += 1;
    }
    let m = out_deg.iter().sum();
    (in_deg, out_deg, m)
}

// Directed Eulierian circuit (all node: in_deg == out_deg)
fn directed_circuit(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let (in_deg, out_deg, m) = degrees(adj);
    if m == 0 {
        return Some(vec![]);
    }
    if (0..adj.len()).any(|u| in_deg[u] != out_deg[u]) {
        return None;
    }
    let start = (0..adj.len()).find(|&u| out_deg[u] > 0)?;
    hierholzer_directed(adj.to_vec(), start, m)
}

// Diretec Eulerian path (all in_deg == out deg) or (start: out - in = 1, end: in - out = 1)
fn directed_path(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let (in_deg, out_deg, m) = degrees(adj);
    if m == 0 {
        return Some(vec![]);
    }

    let (mut starts, mut ends) = (0, 0);
    let mut start = None;
    for u in 0..adj.len() {
        match out_deg[u] as isize - in_deg[u] as isize {
            0 => {}
            1 => {
                starts += 1;
                start = Some(u);
            }
            -1 => ends += 1,
            _ => return None,
        }
    }
    if starts > 1 || starts != ends {
        return None;
    }

    let start = start.or_else(|| (0..adj.len()).find(|&u| out_deg[u] > 0))?;
    hierholzer_directed(adj.to_vec(), start, m)
}

fn main() {
    // Directed: 0->1, 1->2, 2->0, 1->3, 3->1  (circuit)
    let d = vec![vec![1], vec![2, 3], vec![0], vec![1]];
    println!("directed circuit: {:?}", directed_circuit(&d));
    println!("directed path:    {:?}", directed_path(&d));

    // Directed path (không phải circuit): 0->1, 1->2
    let d2 = vec![vec![1], vec![2], vec![]];
    println!("directed circuit: {:?}", directed_circuit(&d2)); // None
    println!("directed path:    {:?}", directed_path(&d2)); // Some([0, 1, 2])
}
