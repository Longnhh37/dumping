struct TwoSat {
    n: usize,
    adj: Vec<Vec<usize>>,
}

impl TwoSat {
    fn new(n: usize) -> Self {
        Self {
            n,
            adj: vec![Vec::new(); 2 * n],
        }
    }

    fn node(var: usize, value: bool) -> usize {
        2 * var + usize::from(!value)
    }

    fn add_clause(&mut self, a: usize, va: bool, b: usize, vb: bool) {
        let (x, y) = (Self::node(a, va), Self::node(b, vb));
        self.adj[x ^ 1].push(y);
        self.adj[y ^ 1].push(x);
    }

    fn solve(&self) -> Option<Vec<bool>> {
        let sccs = kosaraju(&self.adj);

        let mut comp = vec![0; 2 * self.n];
        for (id, scc) in sccs.iter().enumerate() {
            for &v in scc {
                comp[v] = id;
            }
        }

        let mut res = vec![false; self.n];
        for i in 0..self.n {
            let (t, f) = (2 * i, 2 * i + 1);
            if comp[t] == comp[f] {
                return None;
            }
            res[i] = comp[t] > comp[f];
        }

        Some(res)
    }
}

fn kosaraju(adj: &[Vec<usize>]) -> Vec<Vec<usize>> {
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
    // (x0 ∨ x1) ∧ (¬x0 ∨ x1) ∧ (¬x1 ∨ x2)
    let mut sat = TwoSat::new(3);
    sat.add_clause(0, true, 1, true);
    sat.add_clause(0, false, 1, true);
    sat.add_clause(1, false, 2, true);

    match sat.solve() {
        Some(a) => println!("Satisfiable: {a:?}"),
        None => println!("Unsatisfiable"),
    }

    // x0 ∧ ¬x0 viết dưới dạng (x0 ∨ x0) ∧ (¬x0 ∨ ¬x0)
    let mut bad = TwoSat::new(1);
    bad.add_clause(0, true, 0, true);
    bad.add_clause(0, false, 0, false);
    println!("{:?}", bad.solve());
}
