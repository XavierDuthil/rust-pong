use crate::config::*;
use ggez::{event, GameError};
use ggez::graphics::{self};
use ggez::{Context, GameResult};
use ggez::event::{Axis, Button, ErrorOrigin, GamepadId, KeyMods, MouseButton, KeyCode};
use ggez::winit::event::VirtualKeyCode;
use ggez::timer;
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
        let p1 = Pad {
            pos: Vec2::new(INITIAL_PAD1_POSITION_X,INITIAL_PAD1_POSITION_Y),
            size: Vec2::new(PAD_SIZE_X, PAD_SIZE_Y),
            action: Action::Idle,
            speed: PAD_SPEED,
        };
        let p2 = Pad {
            pos: Vec2::new(INITIAL_PAD2_POSITION_X,INITIAL_PAD2_POSITION_Y),
            size: Vec2::new(PAD_SIZE_X, PAD_SIZE_Y),
            action: Action::Idle,
            speed: PAD_SPEED,
        };
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
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = timer::delta(ctx).as_secs_f32();

        for b in &mut self.balls {
            b.update(dt);
        }

        for p in &mut self.pads {
            p.update(dt);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for b in &self.balls {
            b.draw(ctx)?;
        }

        for p in &self.pads {
            p.draw(ctx)?;
        }

        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up => self.pads[1].move_up(),
            KeyCode::Down => self.pads[1].move_down(),
            KeyCode::Z => self.pads[0].move_up(),
            KeyCode::D => self.pads[0].move_down(),
            _ => (),
        }
    }
    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::Up => self.pads[1].stop_move_up(),
            KeyCode::Down => self.pads[1].stop_move_down(),
            KeyCode::Z => self.pads[0].stop_move_up(),
            KeyCode::D => self.pads[0].stop_move_down(),
            _ => (),
        }
    }
}
