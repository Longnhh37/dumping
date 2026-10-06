// ============================================================================
//  KNAPSACK DP CHEAT SHEET (1D dp)
//  Part A: tối ưu hoá giá trị (max value)
//  Part B: đếm số cách (count ways)
// ============================================================================

const MOD: u64 = 1_000_000_007;

// ============================================================================
//  PART A: MAX VALUE
//  dp[c] = tổng value lớn nhất với capacity <= c
//  init: dp = [0; cap + 1]
// ============================================================================

// ----------------------------------------------------------------------------
//  A1. 0/1 Knapsack (mỗi item dùng tối đa 1 lần)
//  Traverse order:
//    - outer: items
//    - inner: capacity c từ cap xuống w (NGƯỢC)
//    - ngược để dp[c - w] vẫn là giá trị của vòng item trước
//      => item hiện tại không bị dùng 2 lần
// ----------------------------------------------------------------------------
fn knapsack_01(items: &[(usize, u64)], cap: usize) -> u64 {
    // items: (weight, value)
    let mut dp = vec![0u64; cap + 1];
    for &(w, v) in items {
        for c in (w..=cap).rev() {
            dp[c] = dp[c].max(dp[c - w] + v);
        }
    }
    dp[cap]
}

// ----------------------------------------------------------------------------
//  A2. Unbounded Knapsack (dùng bao nhiêu lần cũng được)
//  Traverse order:
//    - outer: items
//    - inner: capacity c từ w lên cap (XUÔI)
//    - xuôi để dp[c - w] đã được cập nhật bởi chính item này
//      => item được dùng lại nhiều lần
// ----------------------------------------------------------------------------
fn knapsack_unbounded(items: &[(usize, u64)], cap: usize) -> u64 {
    let mut dp = vec![0u64; cap + 1];
    for &(w, v) in items {
        for c in w..=cap {
            dp[c] = dp[c].max(dp[c - w] + v);
        }
    }
    dp[cap]
}

// ----------------------------------------------------------------------------
//  A3. Bounded Knapsack (mỗi item tối đa k lần)
//  Binary splitting: k copy -> các gói 1, 2, 4, ..., phần dư
//  rồi đưa về 0/1 knapsack.
//  Traverse order: giống A1 (outer items, inner capacity NGƯỢC)
//  Complexity: O(cap * sum(log k_i))
// ----------------------------------------------------------------------------
fn knapsack_bounded(items: &[(usize, u64, usize)], cap: usize) -> u64 {
    // items: (weight, value, count)
    let mut split: Vec<(usize, u64)> = Vec::new();
    for &(w, v, mut k) in items {
        let mut p = 1;
        while k > 0 {
            let t = p.min(k);
            split.push((w * t, v * t as u64));
            k -= t;
            p <<= 1;
        }
    }
    knapsack_01(&split, cap)
}

// ============================================================================
//  PART B: COUNT WAYS (tổng đúng bằng target)
//  dp[c] = số cách tạo tổng đúng bằng c
//  init: dp[0] = 1, còn lại 0
//  (nếu đề hỏi "không vượt quá target" thì cộng dp[0..=target])
// ============================================================================

// ----------------------------------------------------------------------------
//  B1. Unbounded, đếm COMBINATIONS (2223 và 2322 tính là 1)
//  Traverse order:
//    - outer: items
//    - inner: capacity c từ w lên target (XUÔI)
//    - items ở ngoài => mỗi multiset chỉ sinh ra 1 lần
//      (tương đương dãy sorted)
//  CSES: Coin Combinations II
// ----------------------------------------------------------------------------
fn count_combinations(weights: &[usize], target: usize) -> u64 {
    let mut dp = vec![0u64; target + 1];
    dp[0] = 1;
    for &w in weights {
        for c in w..=target {
            dp[c] = (dp[c] + dp[c - w]) % MOD;
        }
    }
    dp[target]
}

// ----------------------------------------------------------------------------
//  B2. Unbounded, đếm PERMUTATIONS (2223 và 2322 tính là 2)
//  Traverse order:
//    - outer: capacity c từ 1 lên target
//    - inner: items
//    - capacity ở ngoài => mỗi c thử mọi item làm bước cuối
//      => các thứ tự khác nhau được đếm riêng
//  CSES: Coin Combinations I
// ----------------------------------------------------------------------------
fn count_permutations(weights: &[usize], target: usize) -> u64 {
    let mut dp = vec![0u64; target + 1];
    dp[0] = 1;
    for c in 1..=target {
        for &w in weights {
            if c >= w {
                dp[c] = (dp[c] + dp[c - w]) % MOD;
            }
        }
    }
    dp[target]
}

// ----------------------------------------------------------------------------
//  B3. 0/1, đếm số SUBSETS (mỗi item tối đa 1 lần)
//  Traverse order:
//    - outer: items
//    - inner: capacity c từ target xuống w (NGƯỢC)
// ----------------------------------------------------------------------------
fn count_subsets(weights: &[usize], target: usize) -> u64 {
    let mut dp = vec![0u64; target + 1];
    dp[0] = 1;
    for &w in weights {
        for c in (w..=target).rev() {
            dp[c] = (dp[c] + dp[c - w]) % MOD;
        }
    }
    dp[target]
}

// ============================================================================
//  QUICK REFERENCE: traverse order
//  ------------------------------------------------------------------------
//  | Bài toán             | Outer    | Inner                | Kết quả       |
//  |----------------------|----------|----------------------|---------------|
//  | 0/1 max value        | items    | capacity NGƯỢC       | max           |
//  | Unbounded max value  | items    | capacity XUÔI        | max           |
//  | Bounded max value    | items    | capacity NGƯỢC (*)   | max           |
//  | Count combinations   | items    | capacity XUÔI        | 2223 == 2322  |
//  | Count permutations   | capacity | items                | 2223 != 2322  |
//  | Count subsets (0/1)  | items    | capacity NGƯỢC       | mỗi item <= 1 |
//  (*) sau khi binary splitting
// ============================================================================

fn main() {
    // ---- Part A ----
    let items = [(2, 3), (3, 4)];
    println!("0/1:       {}", knapsack_01(&items, 6)); // 7
    println!("unbounded: {}", knapsack_unbounded(&items, 6)); // 9
    println!("bounded:   {}", knapsack_bounded(&[(2, 3, 2), (3, 4, 1)], 6)); // 7

    // ---- Part B ----
    let w = [2, 3];
    println!("combinations: {}", count_combinations(&w, 9)); // 2
    println!("permutations: {}", count_permutations(&w, 9)); // 5
    println!("subsets:      {}", count_subsets(&w, 5)); // 1
}
