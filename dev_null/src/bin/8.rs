//! Tổng hợp các dạng DP kiểu knapsack (rolling array 1D).
//!
//! Quy tắc cốt lõi khi nén DP 2D xuống 1D `dp[w]`:
//!   - Duyệt `w` REVERSE (giảm dần): `dp[w - wt]` còn là giá trị CŨ (chưa dùng item hiện tại)
//!     => mỗi item dùng tối đa 1 lần (0/1).
//!   - Duyệt `w` FORWARD (tăng dần): `dp[w - wt]` đã là giá trị MỚI (có thể đã dùng item hiện tại)
//!     => item dùng được nhiều lần (unbounded).
//!
//! Giả định: mọi weight `wt >= 1`.
//!
//! Chạy test: `rustc --edition 2024 --test dp_knapsack.rs -o dp && ./dp`
//! Chạy main: `rustc --edition 2024 dp_knapsack.rs -o dp && ./dp`

const INF: i64 = i64::MAX;

// ============================================================================
// 1. 0/1 KNAPSACK (mỗi item dùng tối đa 1 lần)
//    Vòng ngoài: items | Vòng trong: capacity, REVERSE
// ============================================================================
/// `items`: (weight, value). Trả về tổng value lớn nhất với tổng weight <= cap.
fn knapsack_01(items: &[(usize, i64)], cap: usize) -> i64 {
    let mut dp = vec![0i64; cap + 1]; // dp[w] = value tối đa với capacity w

    for &(wt, val) in items {
        // REVERSE: dp[w - wt] chưa bị item này đụng tới => chỉ dùng item 1 lần.
        // Nếu wt > cap thì range rỗng, item bị bỏ qua (đúng).
        for w in (wt..=cap).rev() {
            dp[w] = dp[w].max(dp[w - wt] + val);
        }
    }
    dp[cap]
}

// Lưu ý: KHÔNG được đảo vòng (capacity ngoài, items trong) với mảng 1D cho 0/1.
// Khi đó mỗi dp[w] chỉ chọn được 1 item rồi cộng vào dp[w - wt] => mất ràng buộc.

// ============================================================================
// 2. UNBOUNDED KNAPSACK (mỗi item dùng không giới hạn)
//    Vòng trong: capacity, FORWARD
// ============================================================================

/// 2a. Unbounded, tối đa hoá value, tổng weight <= cap.
fn unbounded_max(items: &[(usize, i64)], cap: usize) -> i64 {
    let mut dp = vec![0i64; cap + 1];

    for &(wt, val) in items {
        // FORWARD: dp[w - wt] có thể đã chứa item này => dùng lại được nhiều lần.
        for w in wt..=cap {
            dp[w] = dp[w].max(dp[w - wt] + val);
        }
    }
    dp[cap]
}

/// 2b. Unbounded, cost nhỏ nhất để lấp ĐÚNG `target` (ví dụ: coin change, Homer Simpson).
/// `items`: (weight, cost). Trả về `None` nếu không thể lấp đúng.
/// Với min/max thuần tuý, thứ tự vòng ngoài/trong không ảnh hưởng kết quả.
fn unbounded_min_exact(items: &[(usize, i64)], target: usize) -> Option<i64> {
    let mut dp = vec![INF; target + 1]; // INF = không khả thi
    dp[0] = 0;

    for w in 1..=target {
        for &(wt, cost) in items {
            // Kiểu "pull": tính dp[w] từ các ô nhỏ hơn đã xong.
            if w >= wt && dp[w - wt] != INF {
                dp[w] = dp[w].min(dp[w - wt] + cost);
            }
        }
    }
    (dp[target] != INF).then_some(dp[target])
}

/// 2c. Unbounded, cost nhỏ nhất để mua ÍT NHẤT `target` items (cho phép mua dư).
/// Đây là bài UVa 10980 (Lowest Price in Town). `items`: (số lượng, giá).
/// Mua dư: combo lớn hơn `i` thì `saturating_sub` đưa về dp[0].
fn unbounded_min_cover(items: &[(usize, i64)], target: usize) -> i64 {
    let mut dp = vec![INF; target + 1];
    dp[0] = 0;

    for i in 1..=target {
        for &(n, p) in items {
            // n >= 1 nên i.saturating_sub(n) < i => dp[...] đã tính xong và hữu hạn
            // (miễn là có ít nhất một combo, ví dụ mua lẻ (1, unit)).
            dp[i] = dp[i].min(dp[i.saturating_sub(n)] + p);
        }
    }
    dp[target]
}

// ============================================================================
// 3. ĐẾM SỐ CÁCH (thứ tự hai vòng lặp QUYẾT ĐỊNH đếm cái gì)
// ============================================================================

/// 3a. Đếm TỔ HỢP (combinations): {1,2} và {2,1} tính là MỘT cách.
/// Vòng ngoài: items | Vòng trong: capacity (FORWARD).
/// Items ngoài => các coin được "thêm vào" theo thứ tự cố định => mỗi tổ hợp sinh đúng 1 lần.
fn count_combinations(coins: &[usize], target: usize) -> u64 {
    let mut dp = vec![0u64; target + 1];
    dp[0] = 1; // 1 cách tạo ra 0: không chọn gì

    for &c in coins {
        for w in c..=target {
            dp[w] += dp[w - c];
        }
    }
    dp[target]
}

/// 3b. Đếm HOÁN VỊ (permutations): {1,2} và {2,1} tính là HAI cách.
/// Vòng ngoài: capacity | Vòng trong: items.
/// Ở mỗi mức w, coin nào làm "bước cuối" cũng được => thứ tự khác nhau là cách khác nhau.
fn count_permutations(coins: &[usize], target: usize) -> u64 {
    let mut dp = vec![0u64; target + 1];
    dp[0] = 1;

    for w in 1..=target {
        for &c in coins {
            if w >= c {
                dp[w] += dp[w - c];
            }
        }
    }
    dp[target]
}

// ============================================================================
// 4. BOUNDED KNAPSACK (mỗi item có tối đa `count` cái)
// ============================================================================

/// 4a. Binary splitting: chuyển về 0/1 rồi chạy REVERSE. Độ phức tạp O(W * n * log c).
/// Ví dụ count = 13 => các gói 1, 2, 4, 6; tổ hợp các gói này tạo được mọi số 0..=13.
/// `items`: (weight, value, count).
fn bounded_binary_splitting(items: &[(usize, i64, usize)], cap: usize) -> i64 {
    // Bước 1: tách mỗi item thành các item 0/1 "gói".
    let mut packs: Vec<(usize, i64)> = Vec::new();
    for &(wt, val, mut remaining) in items {
        let mut k = 1usize;
        while remaining > 0 {
            let take = k.min(remaining); // gói cuối lấy phần còn lại
            packs.push((wt * take, val * take as i64));
            remaining -= take;
            k <<= 1;
        }
    }
    // Bước 2: chạy 0/1 knapsack thông thường.
    knapsack_01(&packs, cap)
}

/// 4b. Thêm vòng lặp đếm `k`: O(W * c) mỗi item, dùng khi `count` nhỏ.
/// Vẫn REVERSE vì dp[w - k*wt] phải là giá trị CŨ (trước khi xét item này).
fn bounded_counting_loop(items: &[(usize, i64, usize)], cap: usize) -> i64 {
    let mut dp = vec![0i64; cap + 1];

    for &(wt, val, count) in items {
        for w in (0..=cap).rev() {
            // Lấy k cái của item này, k tối đa là count và w / wt.
            for k in 1..=count.min(w / wt) {
                dp[w] = dp[w].max(dp[w - k * wt] + k as i64 * val);
            }
        }
    }
    dp[cap]
}

// (4c. Monotonic queue theo từng lớp đồng dư `w mod wt`, O(W * n):
//  chỉ cần khi count và cap đều rất lớn, không cài ở đây.)

// ============================================================================
// 5. 0/1 VỚI HAI RÀNG BUỘC (ví dụ: cân nặng và thể tích)
//    Vòng ngoài: items | Vòng trong: CẢ HAI capacity, đều REVERSE
// ============================================================================
/// `items`: (weight, volume, value).
fn knapsack_01_two_constraints(
    items: &[(usize, usize, i64)],
    cap_w: usize,
    cap_v: usize,
) -> i64 {
    let mut dp = vec![vec![0i64; cap_v + 1]; cap_w + 1]; // dp[w][v]

    for &(wt, vol, val) in items {
        // Cả hai chiều đều reverse để mọi ô nguồn còn là giá trị CŨ.
        for w in (wt..=cap_w).rev() {
            for v in (vol..=cap_v).rev() {
                dp[w][v] = dp[w][v].max(dp[w - wt][v - vol] + val);
            }
        }
    }
    dp[cap_w][cap_v]
}

// ============================================================================
// 6. GROUP KNAPSACK (mỗi nhóm chọn TỐI ĐA 1 item)
//    Thứ tự: for group -> for w (REVERSE) -> for item trong nhóm
// ============================================================================
/// `groups`: mỗi nhóm là danh sách (weight, value).
fn group_knapsack(groups: &[Vec<(usize, i64)>], cap: usize) -> i64 {
    let mut dp = vec![0i64; cap + 1];

    for group in groups {
        for w in (0..=cap).rev() {
            // Vòng items nằm TRONG vòng w: với mỗi w, thử từng item của nhóm,
            // và mọi lần đọc dp[w - wt] đều là giá trị CŨ (index nhỏ hơn, chưa bị ghi
            // trong lượt nhóm này) => mỗi nhóm đóng góp tối đa 1 item.
            for &(wt, val) in group {
                if w >= wt {
                    dp[w] = dp[w].max(dp[w - wt] + val);
                }
            }
        }
    }
    dp[cap]
}

// ============================================================================
// 7. DP 2D (tường minh): không cần nghĩ về reverse/forward
//    Reverse/forward chỉ là hệ quả của việc nén hàng i-1 và i vào cùng một mảng.
//    Chỗ nào đọc hàng i-1 => reverse. Chỗ nào đọc hàng i => forward.
// ============================================================================

/// 0/1 dạng 2D: dp[i][w] đọc từ hàng i-1 (hàng của item trước đó).
fn knapsack_01_2d(items: &[(usize, i64)], cap: usize) -> i64 {
    let n = items.len();
    let mut dp = vec![vec![0i64; cap + 1]; n + 1];

    for i in 1..=n {
        let (wt, val) = items[i - 1];
        for w in 0..=cap {
            dp[i][w] = dp[i - 1][w]; // không lấy item i
            if w >= wt {
                dp[i][w] = dp[i][w].max(dp[i - 1][w - wt] + val); // lấy item i (từ hàng i-1)
            }
        }
    }
    dp[n][cap]
}

/// Unbounded dạng 2D: dp[i][w] đọc từ chính hàng i (cho phép lấy item i nhiều lần).
fn unbounded_2d(items: &[(usize, i64)], cap: usize) -> i64 {
    let n = items.len();
    let mut dp = vec![vec![0i64; cap + 1]; n + 1];

    for i in 1..=n {
        let (wt, val) = items[i - 1];
        for w in 0..=cap {
            dp[i][w] = dp[i - 1][w]; // không lấy item i
            if w >= wt {
                dp[i][w] = dp[i][w].max(dp[i][w - wt] + val); // lấy item i (từ CHÍNH hàng i)
            }
        }
    }
    dp[n][cap]
}

// ============================================================================
// Demo
// ============================================================================
fn main() {
    let items = [(2, 3), (3, 5)];
    println!("0/1 cap=7:               {}", knapsack_01(&items, 7)); // 8
    println!("unbounded max cap=7:     {}", unbounded_max(&items, 7)); // 11
    println!(
        "min coins exact 11:      {:?}",
        unbounded_min_exact(&[(3, 1), (5, 1)], 11)
    ); // Some(3)
    println!(
        "min cover K=3:           {}",
        unbounded_min_cover(&[(1, 2200), (2, 2200), (4, 4000)], 3)
    ); // 4000
    println!("combinations:            {}", count_combinations(&[1, 2, 5], 5)); // 4
    println!("permutations:            {}", count_permutations(&[1, 2, 3], 4)); // 7
    println!(
        "bounded (split / loop):  {} / {}",
        bounded_binary_splitting(&[(2, 3, 2), (3, 5, 1)], 7),
        bounded_counting_loop(&[(2, 3, 2), (3, 5, 1)], 7)
    ); // 11 / 11
    println!(
        "two constraints:         {}",
        knapsack_01_two_constraints(&[(1, 1, 3), (1, 1, 4), (2, 2, 6)], 2, 2)
    ); // 7
    println!(
        "group knapsack:          {}",
        group_knapsack(&[vec![(2, 3), (3, 5)], vec![(2, 4)]], 5)
    ); // 9
    println!(
        "2D 0/1 / 2D unbounded:   {} / {}",
        knapsack_01_2d(&items, 7),
        unbounded_2d(&items, 7)
    ); // 8 / 11
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_one_vs_unbounded() {
        let items = [(2, 3), (3, 5)];
        assert_eq!(knapsack_01(&items, 7), 8);
        assert_eq!(unbounded_max(&items, 7), 11);
    }

    #[test]
    fn min_exact_and_cover() {
        assert_eq!(unbounded_min_exact(&[(3, 1), (5, 1)], 11), Some(3));
        assert_eq!(unbounded_min_exact(&[(3, 1), (5, 1)], 7), None);
        assert_eq!(unbounded_min_cover(&[(1, 2200), (2, 2200), (4, 4000)], 3), 4000);
    }

    #[test]
    fn counting() {
        assert_eq!(count_combinations(&[1, 2, 5], 5), 4);
        assert_eq!(count_permutations(&[1, 2, 3], 4), 7);
    }

    #[test]
    fn bounded() {
        let a = [(2, 3, 2), (3, 5, 1)];
        assert_eq!(bounded_binary_splitting(&a, 7), 11);
        assert_eq!(bounded_counting_loop(&a, 7), 11);
        let b = [(2, 3, 1), (3, 5, 1)];
        assert_eq!(bounded_binary_splitting(&b, 7), 8);
        assert_eq!(bounded_counting_loop(&b, 7), 8);
    }

    #[test]
    fn multi_constraint_and_group() {
        assert_eq!(
            knapsack_01_two_constraints(&[(1, 1, 3), (1, 1, 4), (2, 2, 6)], 2, 2),
            7
        );
        assert_eq!(group_knapsack(&[vec![(2, 3), (3, 5)], vec![(2, 4)]], 5), 9);
    }

    #[test]
    fn two_d_matches_one_d() {
        let items = [(2, 3), (3, 5), (4, 6)];
        for cap in 0..=15 {
            assert_eq!(knapsack_01_2d(&items, cap), knapsack_01(&items, cap));
            assert_eq!(unbounded_2d(&items, cap), unbounded_max(&items, cap));
        }
    }
}
