// ========================================================
// ===                   permutations                   ===
// ========================================================

fn permutations<T: Clone>(nums: &[T]) -> Vec<Vec<T>> {
    fn backtrack<T: Clone>(nums: &[T], used: &mut [bool], cur: &mut Vec<T>, res: &mut Vec<Vec<T>>) {
        if cur.len() == nums.len() {
            res.push(cur.clone());
            return;
        }
        for i in 0..nums.len() {
            if used[i] {
                continue;
            }
            used[i] = true;
            cur.push(nums[i].clone());
            backtrack(nums, used, cur, res);
            cur.pop();
            used[i] = false;
        }
    }

    let mut res = Vec::new();
    backtrack(
        nums,
        &mut vec![false; nums.len()],
        &mut Vec::new(),
        &mut res,
    );
    res
}

fn permutations_unique<T: Clone + Ord>(nums: &[T]) -> Vec<Vec<T>> {
    fn backtrack<T: Clone + Ord>(
        nums: &[T],
        used: &mut [bool],
        cur: &mut Vec<T>,
        res: &mut Vec<Vec<T>>,
    ) {
        if cur.len() == nums.len() {
            res.push(cur.clone());
            return;
        }
        for i in 0..nums.len() {
            if used[i] || (i > 0 && nums[i] == nums[i - 1] && !used[i - 1]) {
                continue;
            }
            used[i] = true;
            cur.push(nums[i].clone());
            backtrack(nums, used, cur, res);
            cur.pop();
            used[i] = false;
        }
    }

    let mut sorted = nums.to_vec();
    sorted.sort();
    let mut res = Vec::new();
    backtrack(
        &sorted,
        &mut vec![false; sorted.len()],
        &mut Vec::new(),
        &mut res,
    );
    res
}

fn next_permutation<T: Ord>(a: &mut [T]) -> bool {
    let n = a.len();
    if n < 2 {
        return false;
    }
    let Some(i) = (0..n - 1).rev().find(|&i| a[i] < a[i + 1]) else {
        a.reverse();
        return false;
    };
    let j = (i + 1..n).rev().find(|&j| a[j] > a[i]).unwrap();
    a.swap(i, j);
    a[i + 1..].reverse();
    true
}

// ========================================================
// ===                   subsets                        ===
// ========================================================

fn subsets<T: Clone>(nums: &[T]) -> Vec<Vec<T>> {
    let n = nums.len();
    assert!(n < 64, "n phải < 64");
    (0..1u64 << n)
        .map(|mask| {
            (0..n)
                .filter(|&i| (mask >> i) & 1 == 1)
                .map(|i| nums[i].clone())
                .collect()
        })
        .collect()
}

fn subsets_backtrack<T: Clone>(nums: &[T]) -> Vec<Vec<T>> {
    fn backtrack<T: Clone>(nums: &[T], start: usize, cur: &mut Vec<T>, res: &mut Vec<Vec<T>>) {
        res.push(cur.clone());
        for i in start..nums.len() {
            cur.push(nums[i].clone());
            backtrack(nums, i + 1, cur, res);
            cur.pop();
        }
    }

    let mut res = Vec::new();
    backtrack(nums, 0, &mut Vec::new(), &mut res);
    res
}

fn subsets_unique<T: Clone + Ord>(nums: &[T]) -> Vec<Vec<T>> {
    fn backtrack<T: Clone + Ord>(
        nums: &[T],
        start: usize,
        cur: &mut Vec<T>,
        res: &mut Vec<Vec<T>>,
    ) {
        res.push(cur.clone());
        for i in start..nums.len() {
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            cur.push(nums[i].clone());
            backtrack(nums, i + 1, cur, res);
            cur.pop();
        }
    }

    let mut sorted = nums.to_vec();
    sorted.sort();
    let mut res = Vec::new();
    backtrack(&sorted, 0, &mut Vec::new(), &mut res);
    res
}

// ========================================================
// ===                combinations                      ===
// ========================================================

fn combinations<T: Clone>(nums: &[T], k: usize) -> Vec<Vec<T>> {
    fn backtrack<T: Clone>(
        nums: &[T],
        k: usize,
        start: usize,
        cur: &mut Vec<T>,
        res: &mut Vec<Vec<T>>,
    ) {
        if cur.len() == k {
            res.push(cur.clone());
            return;
        }
        let need = k - cur.len();
        for i in start..nums.len() {
            if nums.len() - i < need {
                break;
            }
            cur.push(nums[i].clone());
            backtrack(nums, k, i + 1, cur, res);
            cur.pop();
        }
    }

    let mut res = Vec::new();
    if k <= nums.len() {
        backtrack(nums, k, 0, &mut Vec::new(), &mut res);
    }
    res
}

// ========================================================
// ===                combination sum                   ===
// ========================================================

fn combination_sum(candidates: &[u32], target: u32) -> Vec<Vec<u32>> {
    fn backtrack(
        cands: &[u32],
        remain: u32,
        start: usize,
        cur: &mut Vec<u32>,
        res: &mut Vec<Vec<u32>>,
    ) {
        if remain == 0 {
            res.push(cur.clone());
            return;
        }
        for (i, &c) in cands.iter().enumerate().skip(start) {
            if c > remain {
                break;
            }
            cur.push(c);
            backtrack(cands, remain - c, i, cur, res);
            cur.pop();
        }
    }

    let mut sorted = candidates.to_vec();
    sorted.retain(|&c| c > 0);
    sorted.sort_unstable();
    sorted.dedup();

    let mut res = Vec::new();
    backtrack(&sorted, target, 0, &mut Vec::new(), &mut res);
    res
}

fn combination_sum_unique(nums: &[u32], target: u32) -> Vec<Vec<u32>> {
    fn backtrack(
        cands: &[u32],
        remain: u32,
        start: usize,
        cur: &mut Vec<u32>,
        res: &mut Vec<Vec<u32>>,
    ) {
        if remain == 0 {
            res.push(cur.clone());
            return;
        }
        for (i, &c) in cands.iter().enumerate().skip(start) {
            if c > remain {
                break;
            }
            cur.push(c);
            backtrack(cands, remain - c, i + 1, cur, res);
            cur.pop();
        }
    }

    let mut sorted: Vec<u32> = nums.iter().copied().filter(|&n| n > 0).collect();
    sorted.sort_unstable();
    sorted.dedup();

    let mut res = Vec::new();
    backtrack(&sorted, target, 0, &mut Vec::new(), &mut res);
    res
}

fn combination_sum2(candidates: &[u32], target: u32) -> Vec<Vec<u32>> {
    fn backtrack(
        cands: &[u32],
        remain: u32,
        start: usize,
        cur: &mut Vec<u32>,
        res: &mut Vec<Vec<u32>>,
    ) {
        if remain == 0 {
            res.push(cur.clone());
            return;
        }
        for i in start..cands.len() {
            if cands[i] > remain {
                break;
            }
            if i > start && cands[i] == cands[i - 1] {
                continue;
            }
            cur.push(cands[i]);
            backtrack(cands, remain - cands[i], i + 1, cur, res);
            cur.pop();
        }
    }

    let mut sorted = candidates.to_vec();
    sorted.sort_unstable();

    let mut res = Vec::new();
    backtrack(&sorted, target, 0, &mut Vec::new(), &mut res);
    res
}

fn combination_sum_bounded(nums: &[u32], target: u32) -> Vec<Vec<u32>> {
    fn backtrack(
        groups: &[(u32, usize)],
        remain: u32,
        start: usize,
        cur: &mut Vec<u32>,
        res: &mut Vec<Vec<u32>>,
    ) {
        if remain == 0 {
            res.push(cur.clone());
            return;
        }
        for (i, &(v, cnt)) in groups.iter().enumerate().skip(start) {
            if v > remain {
                break;
            }
            let base = cur.len();
            let mut left = remain;
            for _ in 0..cnt {
                if v > left {
                    break;
                }
                cur.push(v);
                left -= v;
                backtrack(groups, left, i + 1, cur, res);
            }
            cur.truncate(base);
        }
    }

    let mut sorted: Vec<u32> = nums.iter().copied().filter(|&n| n > 0).collect();
    sorted.sort_unstable();

    let mut groups: Vec<(u32, usize)> = Vec::new();
    for n in sorted {
        match groups.last_mut() {
            Some((v, cnt)) if *v == n => *cnt += 1,
            _ => groups.push((n, 1)),
        }
    }

    let mut res = Vec::new();
    backtrack(&groups, target, 0, &mut Vec::new(), &mut res);
    res
}

// ========================================================
// ===                       main                       ===
// ========================================================

fn main() {
    println!("permutations([1,2,3]):");
    for p in permutations(&[1, 2, 3]) {
        println!("  {p:?}");
    }

    println!("permutations_unique([1,1,2]):");
    for p in permutations_unique(&[1, 1, 2]) {
        println!("  {p:?}");
    }

    println!("next_permutation:");
    let mut a = [1, 2, 3];
    loop {
        println!("  {a:?}");
        if !next_permutation(&mut a) {
            break;
        }
    }
    println!("  (sau khi hết) -> {a:?}");

    println!("subsets(['a','b','c']) = {:?}", subsets(&['a', 'b', 'c']));
    println!(
        "subsets_backtrack([1,2,3]) = {:?}",
        subsets_backtrack(&[1, 2, 3])
    );
    println!("subsets_unique([1,2,2]) = {:?}", subsets_unique(&[1, 2, 2]));

    println!(
        "combinations([1,2,3,4], 2) = {:?}",
        combinations(&[1, 2, 3, 4], 2)
    );

    let nums = [10, 1, 2, 7, 6, 1, 5];
    println!(
        "combination_sum([2,3,6,7], 7) = {:?}",
        combination_sum(&[2, 3, 6, 7], 7)
    );
    println!(
        "combination_sum_unique({nums:?}, 8) = {:?}",
        combination_sum_unique(&nums, 8)
    );
    println!(
        "combination_sum2({nums:?}, 8) = {:?}",
        combination_sum2(&nums, 8)
    );
    println!(
        "combination_sum_bounded({nums:?}, 8) = {:?}",
        combination_sum_bounded(&nums, 8)
    );
}
