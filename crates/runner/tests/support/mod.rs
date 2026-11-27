//! Shared support for `runner`'s property tests. Include it from a test file
//! with `mod support;`, use [`replay`] as the strategy for valid replays and
//! [`config`] as the `proptest_config`.

// Each test file compiles this module separately and uses only part of it.
#![allow(dead_code)]

use engine::{Button, ButtonState, Change, Replay};
use proptest::prelude::*;
use proptest::sample::subsequence;
use proptest::test_runner::FileFailurePersistence;

/// Longest generated run, in ticks.
pub const MAX_END: u32 = 600;
/// Most button-state changes in a generated replay.
pub const MAX_CHANGES: usize = 32;

/// Config shared by every property.
///
/// Proptest picks a fresh random seed each run. Failures are persisted to
/// `<test>.proptest-regressions` next to the test file; commit it when it
/// appears so the failing case replays on every machine.
pub fn config() -> ProptestConfig {
    ProptestConfig {
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource(
            "proptest-regressions",
        ))),
        ..ProptestConfig::default()
    }
}

/// Replays the parser accepts: any `u64` seed, `end` in `0..=MAX_END`, and
/// up to `MAX_CHANGES` changes at strictly increasing ticks below `end`, each
/// to any of the 256 button states (repeating the held state is allowed).
///
/// The strategy is flat (no `prop_flat_map`), so every part shrinks
/// independently and failures shrink to a small replay.
pub fn replay() -> impl Strategy<Value = Replay> {
    // `end = 0` (the empty run) gets its own weight; uniform picks would
    // almost never hit it.
    let end = prop_oneof![1 => Just(0u32), 9 => 0..=MAX_END];
    let changes = prop::collection::vec((0..MAX_END, button_state()), 0..=MAX_CHANGES);
    // `changes` comes before `end`: tuples shrink left to right, so the ticks
    // shrink first and `end` can then shrink down to just past them.
    (any::<u64>(), changes, end).prop_map(|(seed, changes, end)| {
        let mut changes: Vec<Change> = changes
            .into_iter()
            .filter(|&(tick, _)| tick < end)
            .map(|(tick, state)| Change { tick, state })
            .collect();
        changes.sort_by_key(|change| change.tick);
        changes.dedup_by_key(|change| change.tick);
        Replay { seed, changes, end }
    })
}

/// Any subset of the buttons, biased towards a few small ones so that
/// consecutive changes often repeat the state already held.
fn button_state() -> impl Strategy<Value = ButtonState> {
    let all = Button::ALL.to_vec();
    let first = Button::ALL[..1].to_vec();
    prop_oneof![
        3 => subsequence(all, 0..=Button::ALL.len()),
        1 => subsequence(first, 0..=1),
    ]
    .prop_map(|buttons| buttons.into_iter().collect())
}
