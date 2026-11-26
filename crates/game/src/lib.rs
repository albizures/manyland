//! The game. Depends on `engine` only (ADR-0002).
//!
//! For now a placeholder, just enough to replay: a square that starts at a
//! seeded random position, moves with the D-pad and changes color on A.

use engine::{Button, Color, Ctx, Game, Gfx, SCREEN_HEIGHT, SCREEN_WIDTH};

/// Side of the square, in pixels: one tile (ADR-0003).
pub const SIZE: i32 = 16;

const MAX_X: i32 = SCREEN_WIDTH - SIZE;
const MAX_Y: i32 = SCREEN_HEIGHT - SIZE;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Square {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Game for Square {
    fn init(ctx: &mut Ctx) -> Self {
        Square {
            x: ctx.rng.range(0..MAX_X + 1),
            y: ctx.rng.range(0..MAX_Y + 1),
            color: random_color(ctx),
        }
    }

    fn update(&mut self, ctx: &mut Ctx) {
        let input = &ctx.input;
        let dx = i32::from(input.held(Button::Right)) - i32::from(input.held(Button::Left));
        let dy = i32::from(input.held(Button::Down)) - i32::from(input.held(Button::Up));
        self.x = (self.x + dx).clamp(0, MAX_X);
        self.y = (self.y + dy).clamp(0, MAX_Y);
        if input.pressed(Button::A) {
            self.color = random_color(ctx);
        }
    }

    fn draw(&self, gfx: &mut Gfx) {
        gfx.cls(0);
        gfx.rectfill(self.x, self.y, SIZE, SIZE, self.color);
        gfx.spr(0, self.x, self.y);
    }
}

/// A color from 1–15; 0 is the background.
fn random_color(ctx: &mut Ctx) -> Color {
    Color::try_from(ctx.rng.range(1..16)).expect("1..16 fits a color")
}

#[cfg(test)]
mod tests {
    use super::*;
    use engine::{ButtonState, Input, Rng};

    fn ctx(seed: u64) -> Ctx {
        Ctx {
            tick: 0,
            input: Input::default(),
            rng: Rng::new(seed),
        }
    }

    fn hold(ctx: &mut Ctx, buttons: &[Button]) {
        ctx.input
            .advance(buttons.iter().copied().collect::<ButtonState>());
    }

    #[test]
    fn starts_fully_on_screen_with_a_foreground_color() {
        for seed in 0..200 {
            let square = Square::init(&mut ctx(seed));
            assert!((0..=MAX_X).contains(&square.x), "seed {seed}: {square:?}");
            assert!((0..=MAX_Y).contains(&square.y), "seed {seed}: {square:?}");
            assert!((1..16).contains(&square.color), "seed {seed}: {square:?}");
        }
    }

    #[test]
    fn d_pad_moves_one_pixel_and_stops_at_edges() {
        let mut ctx = ctx(0);
        let mut square = Square {
            x: 1,
            y: MAX_Y,
            color: 1,
        };
        for _ in 0..3 {
            hold(&mut ctx, &[Button::Left, Button::Down]);
            square.update(&mut ctx);
        }
        assert_eq!((square.x, square.y), (0, MAX_Y));

        hold(&mut ctx, &[Button::Right, Button::Up]);
        square.update(&mut ctx);
        assert_eq!((square.x, square.y), (1, MAX_Y - 1));
    }

    #[test]
    fn color_changes_only_when_a_is_pressed() {
        let mut ctx = ctx(0);
        let mut square = Square::init(&mut ctx);
        let rng_before = ctx.rng.clone();

        hold(&mut ctx, &[Button::B]);
        square.update(&mut ctx);
        assert_eq!(ctx.rng, rng_before, "no randomness without A");

        hold(&mut ctx, &[Button::A]);
        square.update(&mut ctx);
        assert_ne!(ctx.rng, rng_before, "pressing A draws a color");

        let after_press = ctx.rng.clone();
        hold(&mut ctx, &[Button::A]);
        square.update(&mut ctx);
        assert_eq!(ctx.rng, after_press, "holding A is not pressing it");
    }
}
