//! Enforces the crate dependency rule of ADR-0002.
//!
//! Only direct edges between workspace members are checked. That is enough to
//! rule out transitive paths too: our crates are unpublished path dependencies,
//! so no external crate can depend on them, and any path from one member to
//! another is made of member-to-member edges, one of which would be forbidden.
//! External dependencies are out of scope here.

use std::process::Command;

/// The rule of ADR-0002. Every dependency kind (normal, build, dev) counts.
const ALLOWED: Allowlist = &[
    ("engine", &[]),
    ("graphics", &["engine"]),
    ("game", &["engine"]),
    ("runner", &["engine", "graphics", "game"]),
];

/// For each workspace member, the workspace members it may depend on directly.
type Allowlist<'a> = &'a [(&'a str, &'a [&'a str])];

/// Returns one message per broken rule; empty means the graph is allowed.
fn violations(members: &[&str], edges: &[(&str, &str)], allowed: Allowlist) -> Vec<String> {
    let unknown = members
        .iter()
        .filter(|member| allowed_deps(allowed, member).is_none())
        .map(|member| format!("`{member}` is not in the allowlist: add it here and to ADR-0002"));
    let forbidden = edges
        .iter()
        .filter(|(from, to)| !allowed_deps(allowed, from).is_some_and(|deps| deps.contains(to)))
        .map(|(from, to)| format!("`{from}` → `{to}` is not allowed (ADR-0002)"));
    unknown.chain(forbidden).collect()
}

fn allowed_deps<'a>(allowed: Allowlist<'a>, member: &str) -> Option<&'a [&'a str]> {
    allowed
        .iter()
        .find(|(name, _)| *name == member)
        .map(|(_, deps)| *deps)
}

const TEST_ALLOWED: Allowlist = &[
    ("engine", &[]),
    ("graphics", &["engine"]),
    ("game", &["engine"]),
];

#[test]
fn reports_forbidden_edge() {
    let found = violations(
        &["engine", "graphics", "game"],
        &[("game", "engine"), ("game", "graphics")],
        TEST_ALLOWED,
    );
    assert_eq!(found, ["`game` → `graphics` is not allowed (ADR-0002)"]);
}

#[test]
fn reports_member_missing_from_allowlist() {
    let found = violations(&["engine", "audio"], &[], TEST_ALLOWED);
    assert_eq!(
        found,
        ["`audio` is not in the allowlist: add it here and to ADR-0002"]
    );
}

#[test]
fn accepts_allowed_graph() {
    let found = violations(
        &["engine", "graphics", "game"],
        &[("graphics", "engine"), ("game", "engine")],
        TEST_ALLOWED,
    );
    assert!(found.is_empty(), "{found:?}");
}

#[test]
fn workspace_follows_adr_0002() {
    let (members, edges) = workspace_graph();

    let mut sorted = members.clone();
    sorted.sort();
    assert_eq!(sorted, ["engine", "game", "graphics", "runner"]);
    for (from, deps) in ALLOWED {
        for to in *deps {
            assert!(
                edges.contains(&(from.to_string(), to.to_string())),
                "expected edge `{from}` → `{to}` not found; is the metadata parsed?"
            );
        }
    }

    let members: Vec<&str> = members.iter().map(String::as_str).collect();
    let edges: Vec<(&str, &str)> = edges
        .iter()
        .map(|(a, b)| (a.as_str(), b.as_str()))
        .collect();
    let found = violations(&members, &edges, ALLOWED);
    assert!(
        found.is_empty(),
        "crate dependency rule broken:\n{}",
        found.join("\n")
    );
}

/// Workspace member names, and direct dependency edges between members
/// (all kinds, all features), read from `cargo metadata`.
fn workspace_graph() -> (Vec<String>, Vec<(String, String)>) {
    let output = Command::new(env!("CARGO"))
        .args(["metadata", "--format-version", "1", "--all-features"])
        .current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
        .output()
        .expect("run cargo metadata");
    assert!(
        output.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let meta: serde_json::Value = serde_json::from_slice(&output.stdout).expect("parse metadata");

    let member_ids: Vec<&str> = meta["workspace_members"]
        .as_array()
        .expect("workspace_members")
        .iter()
        .map(|id| id.as_str().expect("member id"))
        .collect();
    let name_of = |id: &str| -> String {
        meta["packages"]
            .as_array()
            .expect("packages")
            .iter()
            .find(|p| p["id"] == id)
            .and_then(|p| p["name"].as_str())
            .expect("package for id")
            .to_string()
    };

    let mut edges = Vec::new();
    for node in meta["resolve"]["nodes"].as_array().expect("resolve.nodes") {
        let from = node["id"].as_str().expect("node id");
        if !member_ids.contains(&from) {
            continue;
        }
        for dep in node["deps"].as_array().expect("node deps") {
            let to = dep["pkg"].as_str().expect("dep pkg");
            if member_ids.contains(&to) {
                edges.push((name_of(from), name_of(to)));
            }
        }
    }
    let members = member_ids.iter().map(|id| name_of(id)).collect();
    (members, edges)
}
