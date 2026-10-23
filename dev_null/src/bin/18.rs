use std::collections::HashSet;

fn de_bruijn_euler(k: usize, n: usize) -> Vec<usize> {
    assert!(k >= 1 && n >= 1);

    if n == 1 {
        return (0..k).collect();
    }

    let nv = k.pow(n as u32 - 1);
    let mut next = vec![0usize; nv];
    let mut labels = Vec::with_capacity(nv * k);

    fn dfs(v: usize, k: usize, nv: usize, next: &mut [usize], labels: &mut Vec<usize>) {
        while next[v] < k {
            let c = next[v];
            next[v] += 1;

            let u = (v * k + c) % nv;
            dfs(u, k, nv, next, labels);
            labels.push(c);
        }
    }

    dfs(0, k, nv, &mut next, &mut labels);
    labels.reverse();
    labels
}

fn de_bruijn_linear(k: usize, n: usize) -> Vec<usize> {
    let mut s = vec![0; n - 1];
    s.extend(de_bruijn_euler(k, n));
    s
}

fn is_de_bruijn(seq: &[usize], k: usize, n: usize) -> bool {
    if seq.len() != k.pow(n as u32) {
        return false;
    }

    let len = seq.len();
    let grams: HashSet<Vec<usize>> = (0..len)
        .map(|i| (0..n).map(|j| seq[(i + j) % len]).collect())
        .collect();

    grams.len() == len
}

fn main() {
    let (k, n) = (2, 3);

    let a = de_bruijn_euler(k, n);
    println!("euler: {a:?}");
    println!("linear (k=2, n=3): {:?}", de_bruijn_linear(k, n));

    assert!(is_de_bruijn(&a, k, n));
    assert!(is_de_bruijn(&de_bruijn_euler(3, 4), 3, 4));
}
