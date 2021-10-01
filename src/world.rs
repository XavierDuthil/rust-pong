use crate::config::*;
use ggez::conf::WindowMode;
use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;

pub mod ball;
pub mod pad;

use ball::*;
use pad::*;

#[derive(Debug)]
pub struct World {
    pads: [Pad; 2],
    balls: Vec<Ball>,
}

impl World {
    pub fn new() -> GameResult<World> {
        let p1 = Pad {};
        let p2 = Pad {};
        let b1 = Ball {
            pos: Vec2::new(INITIAL_BALL_POSITION_X, INITIAL_BALL_POSITION_Y),
            direction: INITIAL_BALL_DIRECTION,
            speed: INITIAL_BALL_SPEED,
            radius: BALL_RADIUS,
        };

        let s = World {
            pads: [p1, p2],
            balls: vec![b1],
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for World {
    fn update(&mut self, _: &mut Context) -> GameResult {
        for b in &mut self.balls {
            b.update();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for b in &self.balls {
            b.draw(ctx)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
