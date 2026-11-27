//! Property: a replay printed as text parses back to the same replay.

mod support;

use engine::Replay;
use proptest::prelude::*;

proptest! {
    #![proptest_config(support::config())]

    #[test]
    fn printed_replay_parses_back_unchanged(replay in support::replay()) {
        let text = replay.to_string();
        let parsed = Replay::parse(&text);
        prop_assert_eq!(parsed.as_ref(), Ok(&replay), "replay text:\n{}", text);
    }
}
