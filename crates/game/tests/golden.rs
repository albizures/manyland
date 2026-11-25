//! Golden tests: a replay produces a byte-identical command dump (ADR-0001).
//!
//! Each `tests/golden/<name>.replay` has a `<name>.dump` beside it. Run with
//! `BLESS=1` to rewrite the dumps after an intended change, then review the diff.

use engine::{Replay, dump, run};
use game::Square;
use std::fs;
use std::path::{Path, PathBuf};

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden")
}

fn dump_of(text: &str) -> String {
    let replay = Replay::parse(text).unwrap_or_else(|e| panic!("replay: {e}"));
    dump(&run::<Square>(&replay))
}

#[test]
fn replays_match_their_dumps() {
    let bless = std::env::var_os("BLESS").is_some();
    let mut replays: Vec<PathBuf> = fs::read_dir(golden_dir())
        .expect("read tests/golden")
        .map(|entry| entry.expect("dir entry").path())
        .filter(|path| path.extension().is_some_and(|e| e == "replay"))
        .collect();
    replays.sort();
    assert!(!replays.is_empty(), "no .replay files in tests/golden");

    let mut mismatched = Vec::new();
    for replay in replays {
        let text = fs::read_to_string(&replay).expect("read replay");
        let replay_name = replay
            .file_name()
            .expect("file name")
            .to_string_lossy()
            .into_owned();
        let actual = Replay::parse(&text)
            .map(|r| dump(&run::<Square>(&r)))
            .unwrap_or_else(|e| panic!("tests/golden/{replay_name}:{e}"));
        let dump_path = replay.with_extension("dump");
        if bless {
            fs::write(&dump_path, &actual).expect("write dump");
            continue;
        }
        let expected = fs::read_to_string(&dump_path).unwrap_or_default();
        if actual != expected {
            mismatched.push(first_difference(&dump_path, &expected, &actual));
        }
    }
    assert!(
        mismatched.is_empty(),
        "command dumps differ (rerun with BLESS=1 if the change is intended):\n{}",
        mismatched.join("\n")
    );
}

fn first_difference(path: &Path, expected: &str, actual: &str) -> String {
    let name = path.file_name().expect("file name").to_string_lossy();
    let mut expected_lines = expected.lines();
    let mut actual_lines = actual.lines();
    for line in 1_u32.. {
        match (expected_lines.next(), actual_lines.next()) {
            (None, None) => break,
            (e, a) if e == a => continue,
            (e, a) => {
                return format!(
                    "  tests/golden/{name}:{line}: expected {:?}, got {:?}",
                    e.unwrap_or("<end>"),
                    a.unwrap_or("<end>")
                );
            }
        }
    }
    format!("  tests/golden/{name}: differs only in the final newline")
}

const BASE: &str = "seed 42\n0 right\n30 right a\n45 -\n120 end\n";

#[test]
fn same_replay_gives_the_same_dump() {
    assert_eq!(dump_of(BASE), dump_of(BASE));
}

#[test]
fn changing_the_seed_changes_the_dump() {
    assert_ne!(dump_of(BASE), dump_of(&BASE.replace("seed 42", "seed 43")));
}

#[test]
fn changing_one_input_changes_the_dump() {
    assert_ne!(dump_of(BASE), dump_of(&BASE.replace("0 right", "0 left")));
    assert_ne!(
        dump_of(BASE),
        dump_of(&BASE.replace("30 right a", "30 right"))
    );
}
