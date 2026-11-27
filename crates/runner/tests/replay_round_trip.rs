//! Property: a replay printed as text parses back to the same replay.
//!
//! Proptest picks a fresh random seed each run. Failures are persisted to
//! `replay_round_trip.proptest-regressions` next to this file; commit it when
//! it appears so the failing case replays on every machine.

mod support;

use engine::Replay;
use proptest::prelude::*;
use proptest::test_runner::FileFailurePersistence;

proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
        ..ProptestConfig::default()
    })]

    #[test]
    fn printed_replay_parses_back_unchanged(replay in support::replay()) {
        let text = replay.to_string();
        let parsed = Replay::parse(&text);
        prop_assert_eq!(parsed.as_ref(), Ok(&replay), "replay text:\n{}", text);
    }
}
