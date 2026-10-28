use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i64,
}

struct Dinic {
    graph: Vec<Vec<Edge>>,
    level: Vec<i32>,
    ptr: Vec<usize>,
}

impl Dinic {
    fn new(n: usize) -> Self {
        Self {
            graph: vec![Vec::new(); n],
            level: vec![-1; n],
            ptr: vec![0; n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize, cap: i64) {
        let rev_u = self.graph[v].len();
        let rev_v = self.graph[u].len();

        self.graph[u].push(Edge {
            to: v,
            rev: rev_u,
            cap,
        });

        self.graph[v].push(Edge {
            to: u,
            rev: rev_v,
            cap: 0,
        });
    }

    fn bfs(&mut self, s: usize, t: usize) -> bool {
        self.level.fill(-1);

        let mut q = VecDeque::new();
        self.level[s] = 0;
        q.push_back(s);

        while let Some(u) = q.pop_front() {
            for v in &self.graph[u] {
                if v.cap > 0 && self.level[v.to] == -1 {
                    self.level[v.to] = self.level[u] + 1;
                    q.push_back(v.to);
                }
            }
        }

        self.level[t] != -1
    }

    fn dfs(&mut self, u: usize, t: usize, pushed: i64) -> i64 {
        if pushed == 0 {
            return 0;
        }
        if u == t {
            return pushed;
        }

        while self.ptr[u] < self.graph[u].len() {
            let i = self.ptr[u];

            let (to, rev, cap) = {
                let v = &self.graph[u][i];
                (v.to, v.rev, v.cap)
            };

            if cap > 0 && self.level[to] == self.level[u] + 1 {
                let tr = self.dfs(to, t, pushed.min(cap));
                if tr > 0 {
                    self.graph[u][i].cap -= tr;
                    self.graph[to][rev].cap += tr;
                    return tr;
                }
            }
            self.ptr[u] += 1;
        }

        0
    }

    fn max_flow(&mut self, s: usize, t: usize) -> i64 {
        let mut flow = 0;

        while self.bfs(s, t) {
            self.ptr.fill(0);

            loop {
                let pushed = self.dfs(s, t, i64::MAX);
                if pushed == 0 {
                    break;
                }

                flow += pushed;
            }
        }

        flow
    }

    fn min_cut(&self, s: usize) -> Vec<bool> {
        let n = self.graph.len();
        let mut seen = vec![false; n];
        let mut q = VecDeque::new();

        seen[s] = true;
        q.push_back(s);

        while let Some(u) = q.pop_front() {
            for v in &self.graph[u] {
                if v.cap > 0 && !seen[v.to] {
                    seen[v.to] = true;
                    q.push_back(v.to);
                }
            }
        }

        seen
    }
}

fn main() {}
