fn permutations<T: Clone>(nums: &[T]) -> Vec<Vec<T>> {
    fn backtrack<T: Clone>(
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
    backtrack(nums, &mut vec![false; nums.len()], &mut Vec::new(), &mut res);
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
    backtrack(&sorted, &mut vec![false; sorted.len()], &mut Vec::new(), &mut res);
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
    println!("subsets_backtrack([1,2,3]) = {:?}", subsets_backtrack(&[1, 2, 3]));
    println!("combinations([1,2,3,4], 2) = {:?}", combinations(&[1, 2, 3, 4], 2));
}
