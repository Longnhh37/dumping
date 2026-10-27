use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("friends.txt").unwrap();
    let mut name_to_id = HashMap::new();
    let mut id_to_name = HashMap::new();
    let mut edges = Vec::new();

    for line in input.lines() {
        let (a, b) = line.split_once('\t').unwrap();
        let u = intern(&mut name_to_id, &mut id_to_name, a);
        let v = intern(&mut name_to_id, &mut id_to_name, b);
        edges.push((u, v));
    }

    let n = name_to_id.len();
    let mut adj = vec![Vec::new(); n];
    for e in &edges {
        let (u, v) = (e.0, e.1);
        adj[u].push(v);
        adj[v].push(u);
    }

    // a.1
    println!("a.");
    let res = within_n_degree(&adj, *name_to_id.get(&"E").unwrap(), 1);
    let names: Vec<_> = res.iter().map(|x| id_to_name[x]).collect();
    println!("{:?}", names);

    // a.2
    let res = within_n_degree(&adj, *name_to_id.get(&"E").unwrap(), 2);
    let names: Vec<_> = res.iter().map(|x| id_to_name[x]).collect();
    println!("{:?}", names);

    // a.3
    let res = within_n_degree(&adj, *name_to_id.get(&"E").unwrap(), 3);
    let names: Vec<_> = res.iter().map(|x| id_to_name[x]).collect();
    println!("{:?}", names);

    // b.
    println!("\nb.");
    let res = exact_n_degree(&adj, *name_to_id.get(&"E").unwrap(), 2);
    let names: Vec<_> = res.iter().map(|x| id_to_name[x]).collect();
    println!("{:?}", names);

    let res = exact_n_degree(&adj, *name_to_id.get(&"E").unwrap(), 3);
    let names: Vec<_> = res.iter().map(|x| id_to_name[x]).collect();
    println!("{:?}", names);

    // c.
    println!("\nc.");
    let start = name_to_id[&"A"];
    let end = name_to_id[&"E"];
    let res = get_distance(&adj, start, end);
    println!("{res}");

    let start = name_to_id[&"B"];
    let end = name_to_id[&"G"];
    let res = get_distance(&adj, start, end);
    println!("{res}");
}

fn within_n_degree(adj: &[Vec<usize>], start: usize, degree: usize) -> Vec<usize> {
    let mut q = VecDeque::from([start]);
    let mut res = Vec::new();
    let mut k = 0;

    let mut visited = vec![false; adj.len()];
    visited[start] = true;

    while !q.is_empty() && k <= degree {
        for _ in 0..q.len() {
            let u = q.pop_front().unwrap();
            if u != start {
                res.push(u);
            }
            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    q.push_back(v);
                }
            }
        }
        k += 1;
    }

    res
}

fn exact_n_degree(adj: &[Vec<usize>], start: usize, degree: usize) -> Vec<usize> {
    let mut q = VecDeque::from([start]);
    let mut res = Vec::new();
    let mut k = 0;

    let mut visited = vec![false; adj.len()];
    visited[start] = true;

    while !q.is_empty() && k <= degree {
        for _ in 0..q.len() {
            let u = q.pop_front().unwrap();
            if k == degree && k != start {
                res.push(u);
            }
            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    q.push_back(v);
                }
            }
        }
        k += 1;
    }

    res
}

fn get_distance(adj: &[Vec<usize>], start: usize, end: usize) -> usize {
    let mut q = VecDeque::from([start]);
    let mut visited = vec![false; adj.len()];
    visited[start] = true;
    let mut k = 0;

    while !q.is_empty() {
        k += 1;
        for _ in 0..q.len() {
            let u = q.pop_front().unwrap();
            if u == end {
                return k - 1;
            }
            for &v in &adj[u] {
                if !visited[v] {
                    visited[v] = true;
                    q.push_back(v);
                }
            }
        }
    }

    unreachable!()
}

fn intern<'a>(
    name_to_id: &mut HashMap<&'a str, usize>,
    id_to_name: &mut HashMap<usize, &'a str>,
    name: &'a str,
) -> usize {
    if let Some(&id) = name_to_id.get(name) {
        id
    } else {
        let len = name_to_id.len();
        name_to_id.insert(name, len);
        id_to_name.insert(len, name);
        len
    }
}
