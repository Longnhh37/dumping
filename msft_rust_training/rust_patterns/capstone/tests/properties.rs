use std::collections::HashSet;

use proptest::prelude::*;
use task_scheduler::{Scheduler, WorkItem};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(32))]

    #[test]
    fn returns_exactly_n_results(
        n in 1usize..100,
        workers in 1usize..8,
        fail_every in 2usize..10,
    ) {
        let scheduler = Scheduler::<usize>::new(workers);
        let mut submitted = HashSet::new();

        for i in 0..n {
            let item = WorkItem::new(format!("t{i}"), move || {
                if i % fail_every == 0 { Err("x".to_string()) } else { Ok(i) }
            });
            submitted.insert(item.id());
            scheduler.submit(item).unwrap();
        }

        let results = scheduler.shutdown();

        prop_assert_eq!(results.len(), n);
        let returned: HashSet<_> = results.iter().map(|r| r.id()).collect();
        prop_assert_eq!(returned, submitted);

        let failures = results.iter().filter(|r| r.is_err()).count();
        let expected = (0..n).filter(|i| i % fail_every == 0).count();
        prop_assert_eq!(failures, expected);
    }
}
