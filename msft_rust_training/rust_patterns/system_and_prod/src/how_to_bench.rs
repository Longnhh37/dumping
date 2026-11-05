// src/lib.rs
pub fn sort_unique(mut v: Vec<i32>) -> Vec<i32> {
    v.sort_unstable();
    v.dedup();
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn output_is_strictly_increasing(v in prop::collection::vec(any::<i32>(), 0..100)) {
            let out = sort_unique(v);
            prop_assert!(out.windows(2).all(|w| w[0] < w[1]));
        }

        #[test]
        fn is_idempotent(v in prop::collection::vec(any::<i32>(), 0..100)) {
            let once = sort_unique(v);
            let twice = sort_unique(once.clone());
            prop_assert_eq!(once, twice);
        }

        #[test]
        fn matches_btreeset_oracle(v in prop::collection::vec(any::<i32>(), 0..100)) {
            let expected: Vec<i32> = v
                .iter()
                .copied()
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect();
            prop_assert_eq!(sort_unique(v), expected);
        }
    }
}
