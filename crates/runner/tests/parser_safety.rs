//! Property: the replay parser never panics on any input string.
//!
//! Proptest picks a fresh random seed each run. Failures are persisted to
//! `parser_safety.proptest-regressions` next to this file; commit it when it
//! appears so the failing case replays on every machine.

use engine::Replay;
use proptest::prelude::*;
use proptest::test_runner::FileFailurePersistence;

/// Arbitrary strings, plus strings built from replay tokens and replay-shaped
/// lines so generated input reaches past the first line of the parser.
fn replay_like_text() -> impl Strategy<Value = String> {
    prop_oneof![
        any::<String>(),
        "(seed|end|up|down|left|right|a|b|start|select|-|#|[0-9]{1,21}| |\t|\n|\r\n){0,64}",
        "seed [0-9]{1,21}\n(([0-9]{1,3}|-)( (up|down|left|right|a|b|start|select|-|end|x))*( #.*)?\n){0,8}[0-9]{1,3} end\n?",
    ]
}

proptest! {
    #![proptest_config(ProptestConfig {
        failure_persistence: Some(Box::new(FileFailurePersistence::WithSource("proptest-regressions"))),
        ..ProptestConfig::default()
    })]

    #[test]
    fn parse_returns_ok_or_err_for_any_string(text in replay_like_text()) {
        // A panic inside `parse` fails the property; any `Result` passes.
        let _ = Replay::parse(&text);
    }
}
