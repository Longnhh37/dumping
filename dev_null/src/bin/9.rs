use std::io::{self, Read};

#[derive(Clone, Copy, PartialEq)]
enum Color {
    White,
    Gray,
    Black,
}

fn dfs(u: usize, adj: &[Vec<usize>], color: &mut [Color], order: &mut Vec<usize>) -> bool {
    color[u] = Color::Gray;

    for &v in &adj[u] {
        match color[v] {
            Color::White => {
                if !dfs(v, adj, color, order) {
                    return false;
                }
            }
            Color::Gray => return false,
            Color::Black => {}
        }
    }

    color[u] = Color::Black;
    order.push(u);
    true
}

fn main() {
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}

fn run() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    let n = it.next().unwrap();
    let m = it.next().unwrap();

    let mut adj = vec![Vec::new(); n + 1];
    for _ in 0..m {
        let u = it.next().unwrap();
        let v = it.next().unwrap();
        adj[u].push(v);
    }

    let mut color = vec![Color::White; n + 1];
    let mut order = Vec::with_capacity(n);

    for s in 1..=n {
        if color[s] == Color::White && !dfs(s, &adj, &mut color, &mut order) {
            println!("IMPOSSIBLE");
            return;
        }
    }

    order.reverse();
    let out: Vec<_> = order.iter().map(ToString::to_string).collect();
    println!("{}", out.join(" "));
}
