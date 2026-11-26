//! How a game plugs into the engine, and the headless run loop.

use crate::draw::{Frame, Gfx};
use crate::input::Input;
use crate::replay::Replay;
use crate::rng::Rng;

/// A game driven by the engine. The engine owns the loop: each tick it calls
/// [`Game::update`] then [`Game::draw`].
pub trait Game {
    /// Builds the starting state. `ctx` is at tick 0 with no buttons held.
    fn init(ctx: &mut Ctx) -> Self;

    /// Advances one tick.
    fn update(&mut self, ctx: &mut Ctx);

    /// Records this tick's frame. Gets only `&self` and the recorder, so
    /// drawing can't change state or consume randomness.
    fn draw(&self, gfx: &mut Gfx);
}

/// What `init` and `update` see.
#[derive(Debug)]
pub struct Ctx {
    /// The current tick; time is the tick count (ADR-0001).
    pub tick: u32,
    pub input: Input,
    /// The only randomness a game may use (ADR-0001).
    pub rng: Rng,
}

/// Runs `G` headless through `replay`, returning one frame per tick.
pub fn run<G: Game>(replay: &Replay) -> Vec<Frame> {
    let mut ctx = Ctx {
        tick: 0,
        input: Input::default(),
        rng: Rng::new(replay.seed),
    };
    let mut game = G::init(&mut ctx);
    replay
        .states()
        .zip(0..)
        .map(|(state, tick)| {
            ctx.tick = tick;
            ctx.input.advance(state);
            game.update(&mut ctx);
            let mut gfx = Gfx::default();
            game.draw(&mut gfx);
            gfx.into_frame()
        })
        .collect()
}
