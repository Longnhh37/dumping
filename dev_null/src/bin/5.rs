fn smallest_reachable(coins: &[u32], target: i64) -> Option<u32> {
    let target = target.max(0) as u32;
    if coins.iter().sum::<u32>() < target {
        return None;
    }

    let mut reachable: u32 = 1;
    for &c in coins {
        reachable |= reachable << c;
    }
    (target..).find(|&b| (reachable >> b) & 1 == 1)
}
fn main() {
    let coins = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 12;

    let res = smallest_reachable(&coins, target);
    match res {
        None => println!("unreachable"),
        Some(v) => println!("{v}"),
    }
}
