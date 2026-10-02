#[derive(Clone, Copy, Debug)]
struct Item {
    weight: usize,
    value: u64,
    count: usize,
}

fn bounded(items: &[Item], cap: usize) -> u64 {
    let mut dp = vec![0u64; cap + 1];
    for &Item {
        weight: w,
        value: v,
        count,
    } in items
    {
        assert!(w > 0, "weight phải > 0");
        for c in (w..=cap).rev() {
            let max_k = count.min(c / w);
            for k in 1..=max_k {
                dp[c] = dp[c].max(dp[c - k * w] + k as u64 * v);
            }
        }
    }
    dp[cap]
}

fn unbounded(items: &[Item], cap: usize) -> u64 {
    let mut dp = vec![0u64; cap + 1];
    for &Item {
        weight: w,
        value: v,
        ..
    } in items
    {
        assert!(w > 0, "weight phải > 0");
        for c in w..=cap {
            dp[c] = dp[c].max(dp[c - w] + v);
        }
    }
    dp[cap]
}

fn bounded_binary(items: &[Item], cap: usize) -> u64 {
    let mut dp = vec![0u64; cap + 1];
    for &Item {
        weight: w,
        value: v,
        count,
    } in items
    {
        let mut left = count;
        let mut k = 1;
        while left > 0 {
            let take = k.min(left);
            let (cw, cv) = (take * w, take as u64 * v);
            for c in (cw..=cap).rev() {
                dp[c] = dp[c].max(dp[c - cw] + cv);
            }
            left -= take;
            k <<= 1;
        }
    }
    dp[cap]
}

fn main() {
    let items = [
        Item {
            weight: 2,
            value: 3,
            count: 2,
        },
        Item {
            weight: 3,
            value: 4,
            count: 1,
        },
        Item {
            weight: 4,
            value: 5,
            count: 3,
        },
    ];
    let cap = 10;

    let b = bounded(&items, cap);
    let bb = bounded_binary(&items, cap);
    let u = unbounded(&items, cap);

    println!("bounded        = {b}");
    println!("bounded_binary = {bb}");
    println!("unbounded      = {u}");

    assert_eq!(b, bb);
    assert!(u >= b);
}
