//! Enforces the determinism bans of ADR-0001 on `engine` and `game`.
//!
//! The bans live in `crates/{engine,game}/clippy.toml` and in each crate's
//! `[lints]` table. These tests prove three things:
//!
//! - the rules fire: clippy, run with `--cfg ban_fixture`, reports every
//!   violation marked in `crates/engine/src/ban_fixture.rs`;
//! - the real code obeys them: clippy is clean on `engine` and `game`;
//! - `game` is held to the same rules: its config is byte-identical to `engine`'s.
//!
//! Clippy misses suffixed float literals (`2.5f64`), so a source scan covers them.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Crates the bans apply to (ADR-0001).
const BANNED_CRATES: [&str; 2] = ["engine", "game"];

/// Marker kind for violations that only the source scan catches.
const SCAN_SUFFIXED_FLOAT: &str = "scan::suffixed_float";

const FIXTURE: &str = "crates/engine/src/ban_fixture.rs";

#[test]
fn clippy_reports_every_marked_fixture_violation() {
    let expected: BTreeSet<(u64, String)> = markers(&read(FIXTURE))
        .into_iter()
        .filter(|(_, lint)| lint.starts_with("clippy::"))
        .collect();
    assert!(
        !expected.is_empty(),
        "no `// ban: clippy::…` markers in {FIXTURE}"
    );

    let run = clippy(&["engine"], Some("ban_fixture"), "fixture");
    let reported: BTreeSet<(u64, String)> = run
        .diagnostics
        .iter()
        .filter(|d| d.file == FIXTURE)
        .map(|d| (d.line, d.code.clone()))
        .collect();

    let missing: Vec<String> = expected
        .difference(&reported)
        .map(|(line, lint)| format!("{FIXTURE}:{line}: `{lint}` not reported"))
        .collect();
    assert!(
        missing.is_empty(),
        "a determinism ban no longer fires (ADR-0001):\n{}\n\nclippy reported: {reported:?}",
        missing.join("\n")
    );
}

#[test]
fn every_denied_lint_has_a_fixture_marker() {
    let marked: BTreeSet<String> = markers(&read(FIXTURE))
        .into_iter()
        .map(|(_, lint)| lint)
        .collect();
    let denied = denied_clippy_lints(&read("crates/engine/Cargo.toml"));
    assert!(
        !denied.is_empty(),
        "no denied clippy lints in crates/engine/Cargo.toml"
    );
    for lint in denied {
        let code = format!("clippy::{lint}");
        assert!(
            marked.contains(&code),
            "`{code}` is denied but {FIXTURE} has no violation marked `// ban: {code}`"
        );
    }
}

#[test]
fn engine_and_game_pass_clippy() {
    let run = clippy(&BANNED_CRATES, None, "clean");
    let errors: Vec<String> = run
        .diagnostics
        .iter()
        .filter(|d| d.level == "error")
        .map(|d| d.rendered.clone())
        .collect();
    assert!(
        run.success && errors.is_empty(),
        "`engine`/`game` break a determinism ban (ADR-0001):\n{}\n{}",
        errors.join("\n"),
        run.stderr
    );
}

#[test]
fn game_config_matches_engine() {
    assert_eq!(
        read("crates/game/clippy.toml"),
        read("crates/engine/clippy.toml"),
        "crates/game/clippy.toml must be byte-identical to crates/engine/clippy.toml (ADR-0001)"
    );
    assert_eq!(
        lints_table(&read("crates/game/Cargo.toml")),
        lints_table(&read("crates/engine/Cargo.toml")),
        "the `[lints]` table of crates/game/Cargo.toml must be byte-identical to engine's (ADR-0001)"
    );
}

#[test]
fn no_suffixed_float_literals_in_banned_crates() {
    let mut found = Vec::new();
    for krate in BANNED_CRATES {
        for path in rust_files(&root().join("crates").join(krate).join("src")) {
            let rel = path.strip_prefix(root()).expect("under root").to_path_buf();
            if rel == Path::new(FIXTURE) {
                continue;
            }
            let source = fs::read_to_string(&path).expect("read source");
            for (line, literal) in suffixed_floats(&source) {
                found.push(format!("{}:{line}: `{literal}`", rel.display()));
            }
        }
    }
    assert!(
        found.is_empty(),
        "float literals are banned in `engine`/`game` (ADR-0001):\n{}",
        found.join("\n")
    );
}

#[test]
fn scan_finds_every_marked_fixture_float() {
    let source = read(FIXTURE);
    let expected: BTreeSet<u64> = markers(&source)
        .into_iter()
        .filter(|(_, lint)| lint == SCAN_SUFFIXED_FLOAT)
        .map(|(line, _)| line)
        .collect();
    assert!(
        !expected.is_empty(),
        "no `// ban: {SCAN_SUFFIXED_FLOAT}` markers in {FIXTURE}"
    );
    let found: BTreeSet<u64> = suffixed_floats(&source)
        .into_iter()
        .map(|(l, _)| l)
        .collect();
    assert_eq!(found, expected);
}

#[test]
fn scan_flags_suffixed_floats_only() {
    let source = "let a = 2.5f64;\nlet b = 1_f32;\nlet c = 1e3f64;\n\
                  let d = 0x1f32;\nlet e = 7u32;\nlet f = x.0;\n// 3.0f32 in a comment\nlet g = name_f32;\n";
    let found: Vec<u64> = suffixed_floats(source)
        .into_iter()
        .map(|(l, _)| l)
        .collect();
    assert_eq!(found, [1, 2, 3]);
}

#[test]
fn every_profile_checks_overflow() {
    let manifest = read("Cargo.toml");
    for profile in ["dev", "release"] {
        let section = toml_section(&manifest, &format!("[profile.{profile}]"))
            .unwrap_or_else(|| panic!("Cargo.toml has no [profile.{profile}] (ADR-0001)"));
        assert!(
            section
                .lines()
                .any(|l| l.trim() == "overflow-checks = true"),
            "[profile.{profile}] must set `overflow-checks = true` (ADR-0001)"
        );
    }
}

// --- clippy ---------------------------------------------------------------

struct ClippyRun {
    success: bool,
    diagnostics: Vec<Diagnostic>,
    stderr: String,
}

struct Diagnostic {
    /// Lint code, e.g. `clippy::disallowed_types`.
    code: String,
    level: String,
    /// Path relative to the workspace root.
    file: String,
    line: u64,
    rendered: String,
}

/// Runs `cargo clippy` on `packages` (all targets), optionally with a `--cfg`,
/// in its own target dir so it neither waits on nor invalidates the outer build.
fn clippy(packages: &[&str], cfg: Option<&str>, target: &str) -> ClippyRun {
    let mut command = Command::new(env!("CARGO"));
    command
        .arg("clippy")
        .args(packages.iter().flat_map(|p| ["-p", p]))
        .args(["--all-targets", "--message-format=json", "--target-dir"])
        .arg(
            Path::new(env!("CARGO_TARGET_TMPDIR"))
                .join("clippy")
                .join(target),
        )
        .current_dir(root())
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("CARGO_ENCODED_RUSTFLAGS");
    match cfg {
        Some(cfg) => command.env("RUSTFLAGS", format!("--cfg {cfg}")),
        None => command.env_remove("RUSTFLAGS"),
    };
    let output = command.output().expect("run cargo clippy");

    let diagnostics = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
        .filter(|msg| msg["reason"] == "compiler-message")
        .filter_map(|msg| diagnostic(&msg["message"]))
        .collect();
    ClippyRun {
        success: output.status.success(),
        diagnostics,
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
}

fn diagnostic(message: &serde_json::Value) -> Option<Diagnostic> {
    let span = message["spans"]
        .as_array()?
        .iter()
        .find(|s| s["is_primary"] == true)?;
    Some(Diagnostic {
        code: message["code"]["code"]
            .as_str()
            .unwrap_or_default()
            .to_string(),
        level: message["level"].as_str().unwrap_or_default().to_string(),
        file: span["file_name"].as_str()?.to_string(),
        line: span["line_start"].as_u64()?,
        rendered: message["rendered"].as_str().unwrap_or_default().to_string(),
    })
}

// --- source parsing -------------------------------------------------------

/// `(line, lint)` for each `// ban: <lint>` marker, lines numbered from 1.
fn markers(source: &str) -> Vec<(u64, String)> {
    numbered(source)
        .filter_map(|(n, line)| {
            let (_, lint) = line.split_once("// ban: ")?;
            Some((n, lint.trim().to_string()))
        })
        .collect()
}

/// `(line, literal)` for each float literal with an `f32`/`f64` suffix,
/// ignoring `//` comments. Hex, octal and binary literals are integers.
fn suffixed_floats(source: &str) -> Vec<(u64, String)> {
    let mut found = Vec::new();
    for (n, line) in numbered(source) {
        let code = line.split_once("//").map_or(line, |(code, _)| code);
        let chars: Vec<char> = code.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let starts_token = i == 0 || !is_ident(chars[i - 1]) && chars[i - 1] != '.';
            if !(starts_token && chars[i].is_ascii_digit()) {
                i += 1;
                continue;
            }
            let start = i;
            while i < chars.len()
                && (is_ident(chars[i]) || chars[i] == '.' && next_is_digit(&chars, i))
            {
                i += 1;
            }
            let literal: String = chars[start..i].iter().collect();
            let radix_prefixed = ["0x", "0o", "0b"].iter().any(|p| literal.starts_with(p));
            if !radix_prefixed && (literal.ends_with("f32") || literal.ends_with("f64")) {
                found.push((n, literal));
            }
        }
    }
    found
}

fn is_ident(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn next_is_digit(chars: &[char], i: usize) -> bool {
    chars.get(i + 1).is_some_and(char::is_ascii_digit)
}

fn numbered(source: &str) -> impl Iterator<Item = (u64, &str)> {
    (1..).zip(source.lines())
}

/// Names of the lints set to `"deny"` under `[lints.clippy]`.
fn denied_clippy_lints(manifest: &str) -> Vec<String> {
    toml_section(manifest, "[lints.clippy]")
        .unwrap_or_default()
        .lines()
        .filter_map(|l| l.split_once('='))
        .filter(|(_, level)| level.trim() == "\"deny\"")
        .map(|(name, _)| name.trim().to_string())
        .collect()
}

/// Everything from the first `[lints` header to the next non-lints header.
fn lints_table(manifest: &str) -> String {
    let mut table = String::new();
    let mut inside = false;
    for line in manifest.lines() {
        if line.starts_with('[') {
            inside = line.starts_with("[lints");
        }
        if inside {
            table.push_str(line);
            table.push('\n');
        }
    }
    table
}

/// The body of the section with exactly this header, if present.
fn toml_section(manifest: &str, header: &str) -> Option<String> {
    let mut lines = manifest.lines().skip_while(|l| l.trim() != header);
    lines.next()?;
    let body = lines
        .take_while(|l| !l.starts_with('['))
        .collect::<Vec<_>>();
    Some(body.join("\n"))
}

// --- files ----------------------------------------------------------------

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root")
}

fn read(relative: &str) -> String {
    fs::read_to_string(root().join(relative)).unwrap_or_else(|e| panic!("read {relative}: {e}"))
}

fn rust_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).expect("read dir") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            files.extend(rust_files(&path));
        } else if path.extension().is_some_and(|e| e == "rs") {
            files.push(path);
        }
    }
    files.sort();
    files
}
