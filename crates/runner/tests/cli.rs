//! `runner --replay <file>`: prints the command dump, reports errors as `file:line`.

use std::path::Path;
use std::process::{Command, Output};

fn runner(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_runner"))
        .args(args)
        .output()
        .expect("run runner")
}

fn golden(name: &str) -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../game/tests/golden")
        .join(name)
        .to_string_lossy()
        .into_owned()
}

#[test]
fn prints_the_golden_dump() {
    let out = runner(&["--replay", &golden("walk.replay")]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let expected = std::fs::read(golden("walk.dump")).expect("read golden dump");
    assert_eq!(out.stdout, expected);
}

#[test]
fn reports_replay_errors_as_file_and_line() {
    let path = Path::new(env!("CARGO_TARGET_TMPDIR")).join("bad.replay");
    std::fs::write(&path, "seed 1\n0 jump\n5 end\n").expect("write replay");
    let path = path.to_string_lossy().into_owned();

    let out = runner(&["--replay", &path]);
    assert_eq!(out.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.starts_with(&format!("{path}:2: unknown button `jump`")),
        "{stderr}"
    );
}

#[test]
fn rejects_bad_arguments_with_usage() {
    let out = runner(&[]);
    assert_eq!(out.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&out.stderr).contains("usage: runner --replay <file>"));
}
