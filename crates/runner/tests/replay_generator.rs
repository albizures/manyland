//! The shared replay generator stays within its bounds and covers the edge
//! cases property tests rely on, and failing cases shrink to a minimal replay.
//! Uses a fixed seed so it never flakes.

mod support;

use engine::{Button, Replay};
use proptest::prop_assert;
use proptest::strategy::{Strategy, ValueTree};
use proptest::test_runner::{Config, RngAlgorithm, TestError, TestRng, TestRunner};
use std::collections::HashSet;

#[test]
fn generator_covers_edge_cases_within_bounds() {
    let mut runner = TestRunner::deterministic();
    let strategy = support::replay();

    let mut saw_empty_run = false;
    let mut saw_no_changes_with_ticks = false;
    let mut saw_repeat = false;
    let mut states = HashSet::new();

    for _ in 0..2000 {
        let replay: Replay = strategy.new_tree(&mut runner).expect("generates").current();

        assert!(replay.end <= support::MAX_END, "{replay}");
        assert!(replay.changes.len() <= support::MAX_CHANGES, "{replay}");
        assert!(
            replay.changes.iter().all(|c| c.tick < replay.end),
            "{replay}"
        );
        assert!(
            replay.changes.windows(2).all(|w| w[0].tick < w[1].tick),
            "{replay}"
        );

        saw_empty_run |= replay.end == 0;
        saw_no_changes_with_ticks |= replay.end > 0 && replay.changes.is_empty();
        saw_repeat |= replay.changes.windows(2).any(|w| w[0].state == w[1].state);
        states.extend(replay.changes.iter().map(|c| c.state.to_string()));
    }

    assert!(saw_empty_run, "never generated `end = 0`");
    assert!(
        saw_no_changes_with_ticks,
        "never generated a run with no changes"
    );
    assert!(saw_repeat, "never repeated the held state");
    assert_eq!(
        states.len(),
        1 << Button::ALL.len(),
        "button states covered"
    );
}

#[test]
fn failing_replays_shrink_to_a_minimal_case() {
    let config = Config {
        failure_persistence: None,
        ..Config::default()
    };
    let rng = TestRng::deterministic_rng(RngAlgorithm::ChaCha);
    let mut runner = TestRunner::new_with_rng(config, rng);

    // Fails whenever the replay has any change at all.
    let result = runner.run(&support::replay(), |replay| {
        prop_assert!(replay.changes.is_empty());
        Ok(())
    });

    let Err(TestError::Fail(_, shrunk)) = result else {
        panic!("property should fail: {result:?}");
    };
    assert_eq!(
        shrunk.to_string(),
        "seed 0\n0 -\n1 end\n",
        "shrunk to:\n{shrunk}"
    );
}
