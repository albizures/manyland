//! Headless core: tick, input, seeded RNG, and the recording draw API (ADR-0002).

pub mod draw;
pub mod dump;
pub mod game;
pub mod input;
pub mod replay;
pub mod rng;

pub use draw::{Color, DrawCommand, Frame, Gfx};
pub use dump::dump;
pub use game::{Ctx, Game, run};
pub use input::{Button, ButtonState, Input};
pub use replay::{Change, ParseError, Replay};
pub use rng::Rng;

/// Screen size in pixels (ADR-0003).
pub const SCREEN_WIDTH: i32 = 240;
pub const SCREEN_HEIGHT: i32 = 160;

#[cfg(ban_fixture)]
mod ban_fixture;
