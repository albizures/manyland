//! Property: the replay parser never panics on any input string.

mod support;

use engine::{Button, Replay};
use proptest::prelude::*;
use proptest::string::string_regex;

/// Arbitrary strings, plus strings built from replay tokens and replay-shaped
/// lines so generated input reaches past the first line of the parser.
fn replay_like_text() -> impl Strategy<Value = String> {
    let names = Button::ALL.map(Button::name).join("|");
    let tokens = format!("(seed|end|{names}|-|#|[0-9]{{1,21}}| |\t|\n|\r\n){{0,64}}");
    let lines = format!(
        "seed [0-9]{{1,21}}\n(([0-9]{{1,3}}|-)( ({names}|-|end|x))*( #.*)?\n){{0,8}}[0-9]{{1,3}} end\n?"
    );
    prop_oneof![
        any::<String>(),
        string_regex(&tokens).expect("valid regex"),
        string_regex(&lines).expect("valid regex"),
    ]
}

proptest! {
    #![proptest_config(support::config())]

    #[test]
    fn parse_returns_ok_or_err_for_any_string(text in replay_like_text()) {
        // A panic inside `parse` fails the property; any `Result` passes.
        let _ = Replay::parse(&text);
    }
}
