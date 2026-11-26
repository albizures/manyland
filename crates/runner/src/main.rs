//! Wires `game` to a renderer from `graphics` (ADR-0002).
//!
//! For now: `runner --replay <file>` runs the game headless through the replay
//! and prints the command dump to stdout. `--renderer` arrives with the first
//! real renderer.

use std::process::ExitCode;

const USAGE: &str = "usage: runner --replay <file>";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = match args.as_slice() {
        [flag, path] if flag == "--replay" => path,
        _ => {
            eprintln!("{USAGE}");
            return ExitCode::from(2);
        }
    };
    match replay_dump(path) {
        Ok(dump) => {
            print!("{dump}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

/// The command dump for the replay at `path`, or an error naming `path`.
fn replay_dump(path: &str) -> Result<String, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let replay = engine::Replay::parse(&text).map_err(|e| format!("{path}:{e}"))?;
    Ok(engine::dump(&engine::run::<game::Square>(&replay)))
}
