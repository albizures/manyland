//! Enforces the determinism bans of ADR-0001 on `engine` and `game`.
//!
//! The ban lists live in the root `clippy.toml` and are denied in the
//! `[lints]` tables of `engine` and `game`. A ban is a lint `engine` denies
//! that `[workspace.lints.clippy]` doesn't; the rest of `engine`'s table
//! repeats the workspace lints. These tests prove four things:
//!
//! - the rules fire: every ban has a violation marked in
//!   `crates/engine/src/ban_fixture.rs`, and clippy, run with
//!   `--cfg ban_fixture`, reports every marked violation;
//! - the real code obeys them: clippy is clean on the whole workspace;
//! - `game` is held to the same rules: its `[lints]` table is byte-identical to `engine`'s;
//! - the root `clippy.toml` is the only one, so no crate can bypass it.
//!
//! A source scan flags every float literal (`1.5`, `1e3`, `2.5f64`): no clippy
//! lint catches them all without also flagging typed integer literals.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Crates the bans apply to (ADR-0001).
const BANNED_CRATES: [&str; 2] = ["engine", "game"];

/// Marker kind for violations that only the source scan catches.
const SCAN_FLOAT_LITERAL: &str = "scan::float_literal";

const FIXTURE: &str = "crates/engine/src/ban_fixture.rs";

const WORKSPACE_LINTS: &str = "[workspace.lints.clippy]";
const CRATE_LINTS: &str = "[lints.clippy]";

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

    let run = clippy(&["-p", "engine"], Some("ban_fixture"), "fixture");
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
fn every_ban_has_a_fixture_marker() {
    let marked: BTreeSet<String> = markers(&read(FIXTURE))
        .into_iter()
        .map(|(_, lint)| lint)
        .collect();
    let bans = bans();
    assert!(
        !bans.is_empty(),
        "crates/engine/Cargo.toml denies no lints beyond [workspace.lints.clippy]"
    );
    for lint in bans {
        let code = format!("clippy::{lint}");
        assert!(
            marked.contains(&code),
            "`{code}` is a ban but {FIXTURE} has no violation marked `// ban: {code}`"
        );
    }
}

#[test]
fn workspace_passes_clippy() {
    let run = clippy(&["--workspace"], None, "clean");
    let errors: Vec<String> = run
        .diagnostics
        .iter()
        .filter(|d| d.level == "error")
        .map(|d| d.rendered.clone())
        .collect();
    assert!(
        run.success && errors.is_empty(),
        "clippy fails on the workspace (determinism bans are ADR-0001):\n{}\n{}",
        errors.join("\n"),
        run.stderr
    );
}

#[test]
fn game_lints_match_engine() {
    assert_eq!(
        lints_table(&read("crates/game/Cargo.toml")),
        lints_table(&read("crates/engine/Cargo.toml")),
        "the `[lints]` table of crates/game/Cargo.toml must be byte-identical to engine's (ADR-0001)"
    );
}

#[test]
fn root_clippy_toml_is_the_only_one() {
    assert!(
        root().join("clippy.toml").is_file(),
        "the root clippy.toml holds the ADR-0001 ban lists and is missing"
    );
    // Clippy reads only the nearest clippy.toml, so a crate's own file would
    // replace the root one, ban lists included.
    let crate_configs: Vec<String> = fs::read_dir(root().join("crates"))
        .expect("read crates/")
        .map(|entry| entry.expect("dir entry").path().join("clippy.toml"))
        .filter(|path| path.exists())
        .map(|path| {
            path.strip_prefix(root())
                .expect("under root")
                .display()
                .to_string()
        })
        .collect();
    assert!(
        crate_configs.is_empty(),
        "crate-level clippy.toml files bypass the root one: {crate_configs:?}"
    );
}

#[test]
fn engine_repeats_the_workspace_lints() {
    let workspace = read("Cargo.toml");
    let engine = read("crates/engine/Cargo.toml");
    let workspace_denied = lints_at_level(&workspace, WORKSPACE_LINTS, "deny");
    assert!(
        !workspace_denied.is_empty(),
        "no denied lints in {WORKSPACE_LINTS}"
    );
    let engine_denied = lints_at_level(&engine, CRATE_LINTS, "deny");
    let missing: Vec<&String> = workspace_denied.difference(&engine_denied).collect();
    assert!(
        missing.is_empty(),
        "crates/engine/Cargo.toml must deny what {WORKSPACE_LINTS} denies; missing: {missing:?}"
    );
    assert_eq!(
        lints_at_level(&engine, CRATE_LINTS, "warn"),
        lints_at_level(&workspace, WORKSPACE_LINTS, "warn"),
        "crates/engine/Cargo.toml must warn on the same lints as {WORKSPACE_LINTS}"
    );
}

#[test]
fn no_float_literals_in_banned_crates() {
    let mut found = Vec::new();
    for krate in BANNED_CRATES {
        for path in rust_files(&root().join("crates").join(krate).join("src")) {
            let rel = path.strip_prefix(root()).expect("under root").to_path_buf();
            if rel == Path::new(FIXTURE) {
                continue;
            }
            let source = fs::read_to_string(&path).expect("read source");
            for (line, literal) in float_literals(&source) {
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
        .filter(|(_, lint)| lint == SCAN_FLOAT_LITERAL)
        .map(|(line, _)| line)
        .collect();
    assert!(
        !expected.is_empty(),
        "no `// ban: {SCAN_FLOAT_LITERAL}` markers in {FIXTURE}"
    );
    let found: BTreeSet<u64> = float_literals(&source)
        .into_iter()
        .map(|(l, _)| l)
        .collect();
    assert_eq!(found, expected);
}

#[test]
fn scan_flags_float_literals_only() {
    let floats = [
        "let a = 1.5;",
        "let b = 1.;",
        "let c = f(1.);",
        "let d = 1e3;",
        "let e = 1E-3;",
        "let f = 2.5e+10;",
        "let g = 2.5f64;",
        "let h = 1_f32;",
        "let i = 1e3f64;",
    ];
    let non_floats = [
        "let a = 0x1f32;",
        "let b = 0xE3;",
        "let c = 0xe-3;",
        "let d = 0o17;",
        "let e = 0b1010;",
        "let f = x.0;",
        "let g = x.0.1;",
        "for i in 0..5 {}",
        "let h = 0..=5;",
        "let i = 1.max(2);",
        "// 3.0 and 1e3 in a comment",
        "let j = name_f32;",
        "let k = 7u32;",
        "let l = 1usize;",
        "let m = 2isize;",
    ];
    for line in floats {
        assert_eq!(
            float_literals(line).len(),
            1,
            "`{line}` has a float literal"
        );
    }
    for line in non_floats {
        let found = float_literals(line);
        assert!(
            found.is_empty(),
            "`{line}` has no float literal, found {found:?}"
        );
    }
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

/// Runs `cargo clippy` on the packages `scope` selects (`-p engine`,
/// `--workspace`; all targets), optionally with a `--cfg`, in its own target
/// dir so it neither waits on nor invalidates the outer build.
fn clippy(scope: &[&str], cfg: Option<&str>, target: &str) -> ClippyRun {
    let mut command = Command::new(env!("CARGO"));
    command
        .arg("clippy")
        .args(scope)
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

/// `(line, literal)` for each float literal, ignoring `//` comments.
///
/// A float literal is a decimal literal with a fractional part (`1.5`, `1.`),
/// an exponent (`1e3`, `1E-3`) or an `f32`/`f64` suffix (`1f32`). Hex, octal
/// and binary literals are integers; `x.0`, `0..5` and `1.max(2)` are not
/// fractional parts.
fn float_literals(source: &str) -> Vec<(u64, String)> {
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
            let (end, float) = scan_number(&chars, i);
            if float {
                found.push((n, chars[i..end].iter().collect()));
            }
            i = end;
        }
    }
    found
}

/// Scans the numeric literal starting at `start`: returns the index just past
/// it and whether it is a float literal.
fn scan_number(chars: &[char], start: usize) -> (usize, bool) {
    let radix_prefixed = chars[start] == '0'
        && chars
            .get(start + 1)
            .is_some_and(|c| matches!(c, 'x' | 'o' | 'b'));
    let mut float = false;
    let mut i = start;
    while i < chars.len() {
        let c = chars[i];
        if is_ident(c) {
            let signed = chars.get(i + 1).is_some_and(|c| matches!(c, '+' | '-'))
                && next_is_digit(chars, i + 1);
            // `e` in a suffix (`1usize`) is not an exponent.
            let exponent = !radix_prefixed
                && matches!(c, 'e' | 'E')
                && (signed || next_is_digit(chars, i) || chars.get(i + 1) == Some(&'_'));
            float |= exponent;
            i += if exponent && signed { 2 } else { 1 };
        } else if c == '.' && !radix_prefixed && !float && starts_fraction(chars, i) {
            float = true;
            i += 1;
        } else {
            break;
        }
    }
    let literal: String = chars[start..i].iter().collect();
    let suffixed = literal.ends_with("f32") || literal.ends_with("f64");
    (i, !radix_prefixed && (float || suffixed))
}

/// Whether the `.` at `i` belongs to the literal before it: not a range
/// (`0..5`), a method call or field (`1.max(2)`, `1.e3`), as in Rust's lexer.
fn starts_fraction(chars: &[char], i: usize) -> bool {
    chars
        .get(i + 1)
        .is_none_or(|&c| c != '.' && c != '_' && !c.is_alphabetic())
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

/// The determinism bans: lints `engine` denies beyond the workspace's.
fn bans() -> BTreeSet<String> {
    let workspace = lints_at_level(&read("Cargo.toml"), WORKSPACE_LINTS, "deny");
    lints_at_level(&read("crates/engine/Cargo.toml"), CRATE_LINTS, "deny")
        .difference(&workspace)
        .cloned()
        .collect()
}

/// Names of the lints set to `level` in the section with this header.
fn lints_at_level(manifest: &str, header: &str, level: &str) -> BTreeSet<String> {
    let level = format!("\"{level}\"");
    toml_section(manifest, header)
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim_start().starts_with('#'))
        .filter_map(|l| l.split_once('='))
        .filter(|(_, value)| value.trim() == level)
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
