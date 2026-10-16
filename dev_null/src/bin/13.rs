fn main() {
    let a = vec![1, 2, 3];

    // 1D
    let mut pre = vec![0i64; a.len() + 1];
    for i in 0..a.len() {
        pre[i + 1] = pre[i] + a[i];
    }
    let sum = |l: usize, r: usize| pre[r] - pre[l]; // [l, r)

    let a = vec![vec![1, 2, 3]; 3];
    let (rows, cols) = (a.len(), a[0].len());

    // 2D
    let mut pre = vec![vec![0i64; cols + 1]; rows + 1];
    for r in 0..rows {
        for c in 0..cols {
            pre[r + 1][c + 1] = a[r][c] + pre[r][c + 1] + pre[r + 1][c] - pre[r][c];
        }
    }
    let sum = |r1: usize, c1: usize, r2: usize, c2: usize| {
        pre[r2][c2] - pre[r1][c2] - pre[r2][c1] + pre[r1][c1]
    }; // [r1, r2) x [c1, c2)
}
