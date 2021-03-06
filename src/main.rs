//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::GameResult;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event;

use rust_game::config::*;
use rust_game::world::World;

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez")
        .window_mode(WindowMode {
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..Default::default()
        })
        .window_setup(
            WindowSetup {
                vsync: false,
                ..Default::default()
            }
        );
    let (ctx, event_loop) = cb.build()?;
    let state = World::new()?;
    event::run(ctx, event_loop, state)
}
