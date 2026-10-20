use std::collections::HashSet;

fn de_bruijn_euler(k: usize, n: usize) -> Vec<usize> {
    assert!(k >= 1 && n >= 1);
    if n == 1 {
        return (0..k).collect();
    }

    let nv = k.pow(n as u32 - 1);
    let mut next = vec![0usize; nv];
    let mut stack: Vec<(usize, usize)> = vec![(0, 0)];
    let mut labels = Vec::with_capacity(nv * k + 1);

    while let Some(&(v, d)) = stack.last() {
        if next[v] < k {
            let c = next[v];
            next[v] += 1;
            stack.push(((v * k + c) % nv, c));
        } else {
            labels.push(d);
            stack.pop();
        }
    }

    labels.pop();
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
    println!("euler: {a:?}"); // một sequence hợp lệ, ví dụ [0, 0, 0, 1, 1, 1, 0, 1] hoặc dạng tương đương
    println!("linear (k=2, n=3): {:?}", de_bruijn_linear(k, n));

    assert!(is_de_bruijn(&a, k, n));
    assert!(is_de_bruijn(&de_bruijn_euler(3, 4), 3, 4));
}
