//! The recording draw API: draw calls become draw commands, not pixels (ADR-0002).

use core::fmt;

/// Palette index, `0..16` (ADR-0003). No palette is chosen yet.
pub type Color = u8;

/// One entry in a frame description.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrawCommand {
    /// Clear the screen to `color`.
    Cls { color: Color },
    /// Filled rectangle, top-left at `x, y`.
    RectFill {
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        color: Color,
    },
    /// Sprite `id`, top-left at `x, y`.
    Spr { id: u16, x: i32, y: i32 },
}

/// Command dump syntax, without the tick.
impl fmt::Display for DrawCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DrawCommand::Cls { color } => write!(f, "cls {color}"),
            DrawCommand::RectFill { x, y, w, h, color } => {
                write!(f, "rectfill {x} {y} {w} {h} {color}")
            }
            DrawCommand::Spr { id, x, y } => write!(f, "spr {id} {x} {y}"),
        }
    }
}

/// The draw commands recorded for one tick.
pub type Frame = Vec<DrawCommand>;

/// What `Game::draw` gets: records draw calls into a frame.
#[derive(Debug, Default)]
pub struct Gfx {
    frame: Frame,
}

impl Gfx {
    pub fn cls(&mut self, color: Color) {
        self.push(DrawCommand::Cls {
            color: checked(color),
        });
    }

    pub fn rectfill(&mut self, x: i32, y: i32, w: i32, h: i32, color: Color) {
        self.push(DrawCommand::RectFill {
            x,
            y,
            w,
            h,
            color: checked(color),
        });
    }

    pub fn spr(&mut self, id: u16, x: i32, y: i32) {
        self.push(DrawCommand::Spr { id, x, y });
    }

    pub(crate) fn into_frame(self) -> Frame {
        self.frame
    }

    fn push(&mut self, command: DrawCommand) {
        self.frame.push(command);
    }
}

/// Palettes have 16 colors (ADR-0003); a larger index is a bug in the caller.
fn checked(color: Color) -> Color {
    assert!(color < 16, "color {color} is not a palette index (0..16)");
    color
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_calls_in_order() {
        let mut gfx = Gfx::default();
        gfx.cls(0);
        gfx.rectfill(-4, 5, 16, 16, 15);
        gfx.spr(3, 1, 2);
        let lines: Vec<String> = gfx.into_frame().iter().map(ToString::to_string).collect();
        assert_eq!(lines, ["cls 0", "rectfill -4 5 16 16 15", "spr 3 1 2"]);
    }

    #[test]
    #[should_panic(expected = "not a palette index")]
    fn rejects_color_outside_palette() {
        Gfx::default().cls(16);
    }
}
