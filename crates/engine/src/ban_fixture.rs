//! Deliberate violations of the determinism bans (ADR-0001), one per marked line.
//!
//! Compiled only under `--cfg ban_fixture`, by `crates/runner/tests/determinism_bans.rs`,
//! which checks that each `// ban: <lint>` line is reported. Never use this module.
#![allow(dead_code)]

pub fn floats(
    a: f32, // ban: clippy::disallowed_types
    b: f64, // ban: clippy::disallowed_types
    c: f64, // ban: clippy::disallowed_types
) -> bool {
    let _sum = a + a; // ban: clippy::float_arithmetic
    let _fallback = 1.5; // ban: clippy::default_numeric_fallback
    let _suffixed = 2.5f64; // ban: scan::suffixed_float
    let _whole = 1f32; // ban: scan::suffixed_float
    b == c // ban: clippy::float_cmp
}

pub fn casts() -> u32 {
    3u8 as u32 // ban: clippy::as_conversions
}

pub fn collections(
    _map: std::collections::HashMap<u8, u8>, // ban: clippy::disallowed_types
    _set: std::collections::HashSet<u8>,     // ban: clippy::disallowed_types
) {
}

pub fn wall_clock(
    _instant: std::time::Instant,        // ban: clippy::disallowed_types
    _system_time: std::time::SystemTime, // ban: clippy::disallowed_types
) {
    let _now = std::time::Instant::now(); // ban: clippy::disallowed_methods
    let _system_now = std::time::SystemTime::now(); // ban: clippy::disallowed_methods
}
