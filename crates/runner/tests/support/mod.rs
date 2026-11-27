//! Shared support for `runner`'s property tests. Include it from a test file
//! with `mod support;` and use [`replay`] as the strategy for valid replays.

use engine::{Button, ButtonState, Change, Replay};
use proptest::prelude::*;
use proptest::sample::subsequence;

/// Longest generated run, in ticks.
pub const MAX_END: u32 = 600;
/// Most button-state changes in a generated replay.
pub const MAX_CHANGES: usize = 32;

/// Replays the parser accepts: any `u64` seed, `end` in `0..=MAX_END`, and
/// up to `MAX_CHANGES` changes at strictly increasing ticks below `end`, each
/// to any of the 256 button states (repeating the held state is allowed).
pub fn replay() -> impl Strategy<Value = Replay> {
    // `end = 0` (the empty run) gets its own weight; uniform picks would
    // almost never hit it.
    let end = prop_oneof![1 => Just(0u32), 9 => 0..=MAX_END];
    (any::<u64>(), end)
        .prop_flat_map(|(seed, end)| {
            let max_changes = MAX_CHANGES.min(end as usize);
            let ticks = subsequence((0..end).collect::<Vec<u32>>(), 0..=max_changes);
            (Just(seed), Just(end), ticks)
        })
        .prop_flat_map(|(seed, end, ticks)| {
            let states = prop::collection::vec(button_state(), ticks.len());
            (Just(seed), Just(end), Just(ticks), states)
        })
        .prop_map(|(seed, end, ticks, states)| Replay {
            seed,
            changes: ticks
                .into_iter()
                .zip(states)
                .map(|(tick, state)| Change { tick, state })
                .collect(),
            end,
        })
}

/// Any of the 256 button states, biased towards a few small ones so that
/// consecutive changes often repeat the state already held.
fn button_state() -> impl Strategy<Value = ButtonState> {
    prop_oneof![3 => any::<u8>(), 1 => 0u8..2].prop_map(|bits| {
        Button::ALL
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| bits & (1 << i) != 0)
            .map(|(_, button)| button)
            .collect()
    })
}
