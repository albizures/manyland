//! Replays: a seed, the button-state changes, and the run's length, as text.
//!
//! ```text
//! # walk right, then press A
//! seed 42
//! 0 right
//! 30 right a
//! 45 -
//! 120 end
//! ```
//!
//! `seed <u64>` comes first. Each later line is `<tick> <buttons…>` (names from
//! [`Button::name`], any order) or `<tick> -` for none; ticks strictly increase,
//! and a state holds until the next line (nothing is held before the first).
//! `<n> end` comes last and runs ticks `0..n`. `#` starts a comment.

use crate::input::{Button, ButtonState};
use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Replay {
    pub seed: u64,
    /// Button-state changes, in strictly increasing tick order, all before `end`.
    pub changes: Vec<Change>,
    /// The run covers ticks `0..end`.
    pub end: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Change {
    pub tick: u32,
    pub state: ButtonState,
}

/// A replay syntax error. Displays as `<line>: <message>`; callers that know
/// the file prefix it to get `file:line: message` (ADR-0004).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    /// 1-based line number.
    pub line: usize,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.line, self.message)
    }
}

impl std::error::Error for ParseError {}

impl Replay {
    pub fn parse(text: &str) -> Result<Replay, ParseError> {
        let mut lines = (1..)
            .zip(text.lines())
            .map(|(n, line)| (n, strip_comment(line)))
            .filter(|(_, line)| !line.is_empty());

        let seed = match lines.next() {
            Some((n, line)) => parse_seed(n, line)?,
            None => return Err(error(1, "expected `seed <u64>`, found an empty replay")),
        };

        let mut changes: Vec<Change> = Vec::new();
        let mut last: Option<(usize, u32)> = None;
        for (n, line) in lines.by_ref() {
            let (tick_word, rest) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
            let tick: u32 = tick_word
                .parse()
                .map_err(|_| error(n, format!("expected a tick number, found `{tick_word}`")))?;
            if let Some((_, previous)) = last
                && tick <= previous
            {
                return Err(error(
                    n,
                    format!("tick {tick} must be greater than the previous tick {previous}"),
                ));
            }
            last = Some((n, tick));

            let words: Vec<&str> = rest.split_whitespace().collect();
            if words == ["end"] {
                if let Some((extra, _)) = lines.next() {
                    return Err(error(extra, "nothing may follow the `end` line"));
                }
                return Ok(Replay {
                    seed,
                    changes,
                    end: tick,
                });
            }
            changes.push(Change {
                tick,
                state: parse_state(n, &words)?,
            });
        }

        let n = last.map_or(1, |(n, _)| n);
        Err(error(n, "missing the final `<tick> end` line"))
    }

    /// The button state for each tick `0..end`, in order.
    pub fn states(&self) -> impl Iterator<Item = ButtonState> + '_ {
        let mut changes = self.changes.iter().peekable();
        let mut state = ButtonState::NONE;
        (0..self.end).map(move |tick| {
            if let Some(change) = changes.next_if(|c| c.tick == tick) {
                state = change.state;
            }
            state
        })
    }
}

fn strip_comment(line: &str) -> &str {
    line.split_once('#').map_or(line, |(code, _)| code).trim()
}

fn parse_seed(n: usize, line: &str) -> Result<u64, ParseError> {
    match line.split_whitespace().collect::<Vec<_>>()[..] {
        ["seed", value] => value
            .parse()
            .map_err(|_| error(n, format!("seed must be a u64, found `{value}`"))),
        _ => Err(error(n, format!("expected `seed <u64>`, found `{line}`"))),
    }
}

fn parse_state(n: usize, words: &[&str]) -> Result<ButtonState, ParseError> {
    match words {
        [] => Err(error(
            n,
            "expected button names, `-` or `end` after the tick",
        )),
        ["-"] => Ok(ButtonState::NONE),
        _ if words.contains(&"-") => Err(error(
            n,
            "`-` means no buttons and can't be combined with button names",
        )),
        _ => {
            let mut state = ButtonState::NONE;
            for &word in words {
                let button = Button::from_name(word).ok_or_else(|| {
                    let names: Vec<&str> = Button::ALL.into_iter().map(Button::name).collect();
                    error(
                        n,
                        format!(
                            "unknown button `{word}` (expected `-` or one of: {})",
                            names.join(" ")
                        ),
                    )
                })?;
                if state.held(button) {
                    return Err(error(n, format!("button `{word}` is listed twice")));
                }
                state = state.with(button);
            }
            Ok(state)
        }
    }
}

fn error(line: usize, message: impl Into<String>) -> ParseError {
    ParseError {
        line,
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        "# walk right, then press A\nseed 42\n0 right\n30 right a\n45 -\n120 end\n";

    fn state(buttons: &[Button]) -> ButtonState {
        buttons.iter().copied().collect()
    }

    #[test]
    fn parses_the_example() {
        let replay = Replay::parse(EXAMPLE).expect("valid");
        assert_eq!(replay.seed, 42);
        assert_eq!(replay.end, 120);
        assert_eq!(
            replay.changes,
            [
                Change {
                    tick: 0,
                    state: state(&[Button::Right])
                },
                Change {
                    tick: 30,
                    state: state(&[Button::A, Button::Right])
                },
                Change {
                    tick: 45,
                    state: ButtonState::NONE
                },
            ]
        );
    }

    #[test]
    fn states_hold_until_the_next_change() {
        let replay = Replay::parse("seed 1\n2 a # press\n4 -\n6 end").expect("valid");
        let states: Vec<String> = replay.states().map(|s| s.to_string()).collect();
        assert_eq!(states, ["-", "-", "a", "a", "-", "-"]);
    }

    #[test]
    fn empty_run_is_allowed() {
        let replay = Replay::parse("seed 0\n0 end\n").expect("valid");
        assert_eq!(replay.states().count(), 0);
    }

    #[test]
    fn reports_errors_with_line_numbers() {
        let cases = [
            ("", "1: expected `seed <u64>`, found an empty replay"),
            ("0 a\n1 end", "1: expected `seed <u64>`, found `0 a`"),
            ("seed -1\n1 end", "1: seed must be a u64, found `-1`"),
            (
                "seed 1\n\n# c\nx a\n",
                "4: expected a tick number, found `x`",
            ),
            (
                "seed 1\n5 a\n5 b\n9 end",
                "3: tick 5 must be greater than the previous tick 5",
            ),
            (
                "seed 1\n5 a\n3 end",
                "3: tick 3 must be greater than the previous tick 5",
            ),
            (
                "seed 1\n0 jump\n9 end",
                "2: unknown button `jump` (expected `-` or one of: up down left right a b start select)",
            ),
            ("seed 1\n0 a a\n9 end", "2: button `a` is listed twice"),
            (
                "seed 1\n0 - a\n9 end",
                "2: `-` means no buttons and can't be combined with button names",
            ),
            (
                "seed 1\n0\n9 end",
                "2: expected button names, `-` or `end` after the tick",
            ),
            ("seed 1\n0 a\n", "2: missing the final `<tick> end` line"),
            (
                "seed 1\n9 end\n10 a",
                "3: nothing may follow the `end` line",
            ),
        ];
        for (text, expected) in cases {
            let err = Replay::parse(text).expect_err(text);
            assert_eq!(err.to_string(), expected, "for {text:?}");
        }
    }
}
