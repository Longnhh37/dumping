use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

const INF: i32 = i32::MAX / 2;

#[inline(always)]
fn ix(x: u32) -> usize {
    x as usize
}

// ───────────────────────────── 1. Backtrack ─────────────────────────────
mod backtrack {
    pub fn n_queens(n: u32) -> u32 {
        fn go(n: u32, row: u32, cols: u32, d1: u32, d2: u32) -> u32 {
            if row == n {
                return 1;
            }
            let full = (1u32 << n) - 1;
            let mut avail = !(cols | d1 | d2) & full;
            let mut count = 0;
            while avail != 0 {
                let bit = avail & avail.wrapping_neg();
                avail ^= bit;
                count += go(
                    n,
                    row + 1,
                    cols | bit,
                    ((d1 | bit) << 1) & full,
                    (d2 | bit) >> 1,
                );
            }
            count
        }
        go(n, 0, 0, 0, 0)
    }
}

// ───────────────────────────── 2. 0/1 Knapsack (DP) ─────────────────────
mod knapsack {
    pub fn solve(items: &[(u32, i32)], cap: u32) -> i32 {
        let mut dp = vec![0i32; super::ix(cap) + 1];
        for &(w, v) in items {
            if w > cap {
                continue;
            }
            for c in (w..=cap).rev() {
                let cand = dp[super::ix(c - w)] + v;
                if cand > dp[super::ix(c)] {
                    dp[super::ix(c)] = cand;
                }
            }
        }
        dp[super::ix(cap)]
    }
}

// ───────────────────────────── 3. LIS O(n log n) ────────────────────────
mod lis {
    pub fn length(a: &[i32]) -> u32 {
        let mut tails: Vec<i32> = Vec::new();
        for &x in a {
            let pos = tails.partition_point(|&t| t < x);
            if pos == tails.len() {
                tails.push(x);
            } else {
                tails[pos] = x;
            }
        }
        tails.len() as u32
    }
}

// ───────────────────────────── 4. Union-Find + Kruskal ──────────────────
mod dsu_kruskal {
    pub struct Dsu {
        parent: Vec<u32>,
        size: Vec<u32>,
    }

    impl Dsu {
        pub fn new(n: u32) -> Self {
            Self {
                parent: (0..n).collect(),
                size: vec![1; super::ix(n)],
            }
        }

        pub fn find(&mut self, mut x: u32) -> u32 {
            let mut root = x;
            while self.parent[super::ix(root)] != root {
                root = self.parent[super::ix(root)];
            }
            while self.parent[super::ix(x)] != root {
                let next = self.parent[super::ix(x)];
                self.parent[super::ix(x)] = root;
                x = next;
            }
            root
        }

        pub fn union(&mut self, a: u32, b: u32) -> bool {
            let (mut a, mut b) = (self.find(a), self.find(b));
            if a == b {
                return false;
            }
            if self.size[super::ix(a)] < self.size[super::ix(b)] {
                std::mem::swap(&mut a, &mut b);
            }
            self.parent[super::ix(b)] = a;
            self.size[super::ix(a)] += self.size[super::ix(b)];
            true
        }
    }

    pub fn kruskal(n: u32, edges: &[(u32, u32, i32)]) -> (i32, u32) {
        let mut sorted = edges.to_vec();
        sorted.sort_unstable_by_key(|&(_, _, w)| w);
        let mut dsu = Dsu::new(n);
        let (mut total, mut used) = (0i32, 0u32);
        for (u, v, w) in sorted {
            if dsu.union(u, v) {
                total += w;
                used += 1;
                if used + 1 == n {
                    break;
                }
            }
        }
        (total, used)
    }
}

// ───────────────────────────── 5. Topological sort (Kahn) ───────────────
mod topo {
    use super::*;

    pub fn kahn(n: u32, edges: &[(u32, u32)]) -> Option<Vec<u32>> {
        let mut adj: Vec<Vec<u32>> = vec![Vec::new(); ix(n)];
        let mut indeg = vec![0u32; ix(n)];
        for &(u, v) in edges {
            adj[ix(u)].push(v);
            indeg[ix(v)] += 1;
        }
        let mut queue: VecDeque<u32> = (0..n).filter(|&v| indeg[ix(v)] == 0).collect();
        let mut order = Vec::with_capacity(ix(n));
        while let Some(u) = queue.pop_front() {
            order.push(u);
            for &v in &adj[ix(u)] {
                indeg[ix(v)] -= 1;
                if indeg[ix(v)] == 0 {
                    queue.push_back(v);
                }
            }
        }
        (order.len() == ix(n)).then_some(order)
    }
}

// ───────────────────────────── 6. Bellman-Ford ──────────────────────────
mod bellman_ford {
    use super::*;

    pub fn run(n: u32, edges: &[(u32, u32, i32)], src: u32) -> Option<Vec<i32>> {
        let mut dist = vec![INF; ix(n)];
        dist[ix(src)] = 0;
        for _ in 0..n - 1 {
            let mut changed = false;
            for &(u, v, w) in edges {
                if dist[ix(u)] < INF && dist[ix(u)] + w < dist[ix(v)] {
                    dist[ix(v)] = dist[ix(u)] + w;
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        for &(u, v, w) in edges {
            if dist[ix(u)] < INF && dist[ix(u)] + w < dist[ix(v)] {
                return None;
            }
        }
        Some(dist)
    }
}

// ───────────────────────────── 7. Dijkstra ──────────────────────────────
mod dijkstra {
    use super::*;

    pub fn run(adj: &[Vec<(u32, i32)>], src: u32) -> Vec<i32> {
        let mut dist = vec![INF; adj.len()];
        let mut heap = BinaryHeap::new();
        dist[ix(src)] = 0;
        heap.push(Reverse((0i32, src)));
        while let Some(Reverse((d, u))) = heap.pop() {
            if d > dist[ix(u)] {
                continue;
            }
            for &(v, w) in &adj[ix(u)] {
                let nd = d + w;
                if nd < dist[ix(v)] {
                    dist[ix(v)] = nd;
                    heap.push(Reverse((nd, v)));
                }
            }
        }
        dist
    }
}

// ───────────────────────────── 8. Segment tree (sum) ────────────────────
mod segtree {
    use super::ix;

    pub struct SegTree {
        n: u32,
        t: Vec<i32>,
    }

    impl SegTree {
        pub fn new(a: &[i32]) -> Self {
            let n = a.len() as u32;
            let mut t = vec![0i32; 2 * ix(n)];
            t[ix(n)..].copy_from_slice(a);
            for i in (1..n).rev() {
                t[ix(i)] = t[ix(2 * i)] + t[ix(2 * i + 1)];
            }
            Self { n, t }
        }

        pub fn set(&mut self, pos: u32, val: i32) {
            let mut p = pos + self.n;
            self.t[ix(p)] = val;
            while p > 1 {
                p >>= 1;
                self.t[ix(p)] = self.t[ix(2 * p)] + self.t[ix(2 * p + 1)];
            }
        }

        pub fn query(&self, l: u32, r: u32) -> i32 {
            let (mut l, mut r) = (l + self.n, r + self.n);
            let mut sum = 0i32;
            while l < r {
                if l & 1 == 1 {
                    sum += self.t[ix(l)];
                    l += 1;
                }
                if r & 1 == 1 {
                    r -= 1;
                    sum += self.t[ix(r)];
                }
                l >>= 1;
                r >>= 1;
            }
            sum
        }
    }
}

// ───────────────────────────── 9. Ford-Fulkerson (DFS) ──────────────────
mod ford_fulkerson {
    use super::*;

    pub struct Graph {
        to: Vec<u32>,
        cap: Vec<i32>,
        adj: Vec<Vec<u32>>,
    }

    impl Graph {
        pub fn new(n: u32) -> Self {
            Self {
                to: vec![],
                cap: vec![],
                adj: vec![Vec::new(); ix(n)],
            }
        }

        pub fn add_edge(&mut self, u: u32, v: u32, c: i32) {
            let id = self.to.len() as u32;
            self.to.push(v);
            self.cap.push(c);
            self.adj[ix(u)].push(id);
            self.to.push(u);
            self.cap.push(0);
            self.adj[ix(v)].push(id + 1);
        }

        fn dfs(&mut self, u: u32, t: u32, pushed: i32, seen: &mut [bool]) -> i32 {
            if u == t {
                return pushed;
            }
            seen[ix(u)] = true;
            for k in 0..self.adj[ix(u)].len() {
                let e = self.adj[ix(u)][k];
                let v = self.to[ix(e)];
                if !seen[ix(v)] && self.cap[ix(e)] > 0 {
                    let got = self.dfs(v, t, pushed.min(self.cap[ix(e)]), seen);
                    if got > 0 {
                        self.cap[ix(e)] -= got;
                        self.cap[ix(e ^ 1)] += got;
                        return got;
                    }
                }
            }
            0
        }

        pub fn max_flow(&mut self, s: u32, t: u32) -> i32 {
            let mut flow = 0;
            loop {
                let mut seen = vec![false; self.adj.len()];
                let f = self.dfs(s, t, INF, &mut seen);
                if f == 0 {
                    return flow;
                }
                flow += f;
            }
        }
    }
}

// ───────────────────────────── 10. Dinic ────────────────────────────────
mod dinic {
    use super::*;

    pub struct Dinic {
        to: Vec<u32>,
        cap: Vec<i32>,
        adj: Vec<Vec<u32>>,
        level: Vec<i32>,
        it: Vec<u32>,
    }

    impl Dinic {
        pub fn new(n: u32) -> Self {
            Self {
                to: vec![],
                cap: vec![],
                adj: vec![Vec::new(); ix(n)],
                level: vec![-1; ix(n)],
                it: vec![0; ix(n)],
            }
        }

        pub fn add_edge(&mut self, u: u32, v: u32, c: i32) {
            let id = self.to.len() as u32;
            self.to.push(v);
            self.cap.push(c);
            self.adj[ix(u)].push(id);
            self.to.push(u);
            self.cap.push(0);
            self.adj[ix(v)].push(id + 1);
        }

        fn bfs(&mut self, s: u32, t: u32) -> bool {
            self.level.fill(-1);
            self.level[ix(s)] = 0;
            let mut q = VecDeque::from([s]);
            while let Some(u) = q.pop_front() {
                for &e in &self.adj[ix(u)] {
                    let v = self.to[ix(e)];
                    if self.cap[ix(e)] > 0 && self.level[ix(v)] < 0 {
                        self.level[ix(v)] = self.level[ix(u)] + 1;
                        q.push_back(v);
                    }
                }
            }
            self.level[ix(t)] >= 0
        }

        fn dfs(&mut self, u: u32, t: u32, pushed: i32) -> i32 {
            if u == t {
                return pushed;
            }
            while ix(self.it[ix(u)]) < self.adj[ix(u)].len() {
                let e = self.adj[ix(u)][ix(self.it[ix(u)])];
                let v = self.to[ix(e)];
                if self.cap[ix(e)] > 0 && self.level[ix(v)] == self.level[ix(u)] + 1 {
                    let got = self.dfs(v, t, pushed.min(self.cap[ix(e)]));
                    if got > 0 {
                        self.cap[ix(e)] -= got;
                        self.cap[ix(e ^ 1)] += got;
                        return got;
                    }
                }
                self.it[ix(u)] += 1;
            }
            0
        }

        pub fn max_flow(&mut self, s: u32, t: u32) -> i32 {
            let mut flow = 0;
            while self.bfs(s, t) {
                self.it.fill(0);
                loop {
                    let f = self.dfs(s, t, INF);
                    if f == 0 {
                        break;
                    }
                    flow += f;
                }
            }
            flow
        }
    }
}

// ───────────────────────────── main: quick test ─────────────────────────
fn main() {
    // 1. Backtrack
    assert_eq!(backtrack::n_queens(8), 92);
    println!("1. N-Queens(8) = {}", backtrack::n_queens(8));

    // 2. Knapsack
    let items = [(1u32, 15i32), (3, 20), (4, 30)];
    assert_eq!(knapsack::solve(&items, 4), 35);
    println!("2. Knapsack = {}", knapsack::solve(&items, 4));

    // 3. LIS
    let a = [10, 9, 2, 5, 3, 7, 101, 18];
    assert_eq!(lis::length(&a), 4);
    println!("3. LIS = {}", lis::length(&a));

    // 4. DSU + Kruskal
    let edges = [
        (0u32, 1u32, 4i32),
        (0, 2, 1),
        (1, 2, 2),
        (1, 3, 5),
        (2, 3, 8),
    ];
    let (w, used) = dsu_kruskal::kruskal(4, &edges);
    assert_eq!((w, used), (8, 3));
    println!("4. Kruskal MST weight = {w}, edges = {used}");

    // 5. Topo sort
    let order = topo::kahn(6, &[(5, 2), (5, 0), (4, 0), (4, 1), (2, 3), (3, 1)]);
    println!("5. Topo order = {order:?}");
    assert!(topo::kahn(3, &[(0, 1), (1, 2), (2, 0)]).is_none());

    // 6. Bellman-Ford
    let neg = [(0u32, 1u32, 4i32), (0, 2, 5), (1, 2, -3), (2, 3, 2)];
    let d = bellman_ford::run(4, &neg, 0).unwrap();
    assert_eq!(d, vec![0, 4, 1, 3]);
    println!("6. Bellman-Ford dist = {d:?}");
    assert!(bellman_ford::run(3, &[(0, 1, 1), (1, 2, -2), (2, 1, 1)], 0).is_none());

    // 7. Dijkstra
    let mut adj: Vec<Vec<(u32, i32)>> = vec![Vec::new(); 5];
    for &(u, v, w) in &[
        (0u32, 1u32, 2i32),
        (0, 2, 5),
        (1, 2, 1),
        (1, 3, 4),
        (2, 3, 1),
        (3, 4, 3),
    ] {
        adj[ix(u)].push((v, w));
        adj[ix(v)].push((u, w));
    }
    let d = dijkstra::run(&adj, 0);
    assert_eq!(d, vec![0, 2, 3, 4, 7]);
    println!("7. Dijkstra dist = {d:?}");

    // 8. Segment tree
    let mut st = segtree::SegTree::new(&[1, 2, 3, 4, 5]);
    assert_eq!(st.query(1, 4), 9);
    st.set(2, 10);
    assert_eq!(st.query(1, 4), 16);
    println!("8. SegTree sum[1,4) sau update = {}", st.query(1, 4));

    // 9 & 10. Max flow
    let flow_edges = [
        (0u32, 1u32, 16i32),
        (0, 2, 13),
        (1, 2, 10),
        (2, 1, 4),
        (1, 3, 12),
        (3, 2, 9),
        (2, 4, 14),
        (4, 3, 7),
        (3, 5, 20),
        (4, 5, 4),
    ];
    let mut ff = ford_fulkerson::Graph::new(6);
    let mut di = dinic::Dinic::new(6);
    for &(u, v, c) in &flow_edges {
        ff.add_edge(u, v, c);
        di.add_edge(u, v, c);
    }
    let (f1, f2) = (ff.max_flow(0, 5), di.max_flow(0, 5));
    assert_eq!((f1, f2), (23, 23));
    println!("9. Ford-Fulkerson = {f1}, 10. Dinic = {f2}");
}
