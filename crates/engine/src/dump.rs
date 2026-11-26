//! The command dump: a run's frame descriptions as text.
//!
//! One line per draw command, prefixed by its tick (`0 cls 0`). A tick that
//! draws nothing writes `<tick> -`, so every tick appears.

use crate::draw::Frame;
use core::fmt::Write;

/// Serializes `frames`, the first being tick 0.
pub fn dump(frames: &[Frame]) -> String {
    let mut out = String::new();
    for (tick, frame) in (0_u32..).zip(frames) {
        if frame.is_empty() {
            writeln!(out, "{tick} -").expect("write to String");
        }
        for command in frame {
            writeln!(out, "{tick} {command}").expect("write to String");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::draw::DrawCommand;

    #[test]
    fn prefixes_ticks_and_marks_empty_frames() {
        let frames = vec![
            vec![
                DrawCommand::Cls { color: 1 },
                DrawCommand::Spr { id: 2, x: 3, y: 4 },
            ],
            vec![],
            vec![DrawCommand::Cls { color: 0 }],
        ];
        assert_eq!(dump(&frames), "0 cls 1\n0 spr 2 3 4\n1 -\n2 cls 0\n");
    }
}
