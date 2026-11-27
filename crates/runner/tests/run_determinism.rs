//! Property: running a replay twice gives the same command dump, and neither
//! run panics (overflow included, since `overflow-checks = true`). ADR-0001.
//!
//! Proptest picks a fresh random seed each run. Failures are persisted to
//! `run_determinism.proptest-regressions` next to this file; commit it when
//! it appears so the failing case replays on every machine.

mod support;

use std::panic::{AssertUnwindSafe, catch_unwind};

use engine::{Replay, dump, run};
use game::Square;
use proptest::prelude::*;
use proptest::test_runner::FileFailurePersistence;

/// The replay's command dump, or `None` if the run panicked.
fn dump_of(replay: &Replay) -> Option<String> {
    catch_unwind(AssertUnwindSafe(|| dump(&run::<Square>(replay)))).ok()
}

proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
        ..ProptestConfig::default()
    })]

    #[test]
    fn runs_are_deterministic_and_never_panic(replay in support::replay()) {
        let first = dump_of(&replay);
        prop_assert!(first.is_some(), "first run panicked; replay text:\n{}", replay);
        let second = dump_of(&replay);
        prop_assert!(second.is_some(), "second run panicked; replay text:\n{}", replay);
        // Not `prop_assert_eq`: two full dumps would bury the replay text.
        prop_assert!(first == second, "dumps differ; replay text:\n{}", replay);
    }
}
