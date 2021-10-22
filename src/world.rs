use crate::config::*;
use ggez::event;
use ggez::graphics::{self};
use ggez::{Context, GameResult};
use ggez::event::{KeyMods, KeyCode};
use ggez::timer;
use cgmath::*;

pub mod ball;
pub mod pad;
pub mod body;

use ball::*;
use pad::*;
use body::*;

#[derive(Debug)]
pub struct World {
    pads: [Pad; 2],
    balls: Vec<Ball>,
}

impl World {
    pub fn new() -> GameResult<World> {
        let p1 = Pad {
            pos: vec2(INITIAL_PAD1_POSITION_X,INITIAL_PAD1_POSITION_Y),
            size: vec2(PAD_SIZE_X, PAD_SIZE_Y),
            action: Action::Idle,
            movement: vec2(0.0, PAD_SPEED),
            score: 0,
        };
        let p2 = Pad {
            pos: vec2(INITIAL_PAD2_POSITION_X,INITIAL_PAD2_POSITION_Y),
            size: vec2(PAD_SIZE_X, PAD_SIZE_Y),
            action: Action::Idle,
            movement: vec2(0.0, PAD_SPEED),
            score: 0,
        };
        let b1 = Ball {
            pos: vec2(INITIAL_BALL_POSITION_X, INITIAL_BALL_POSITION_Y),
            movement: vec2(INITIAL_BALL_MOVEMENT_X, INITIAL_BALL_MOVEMENT_Y),
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

        for b in &mut self.balls {
            for p in &mut self.pads {
                b.collide(p);
            }
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

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {
        match keycode {
            KeyCode::Up => self.pads[1].move_up(),
            KeyCode::Down => self.pads[1].move_down(),
            KeyCode::Z => self.pads[0].move_up(),
            KeyCode::S => self.pads[0].move_down(),
            _ => (),
        }
    }
    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::Up => self.pads[1].stop_move_up(),
            KeyCode::Down => self.pads[1].stop_move_down(),
            KeyCode::Z => self.pads[0].stop_move_up(),
            KeyCode::S => self.pads[0].stop_move_down(),
            _ => (),
        }
    }
}

fn intoMintVec2(v: Vector2<f32>) -> ggez::mint::Vector2<f32> {
    ggez::mint::Vector2{x:v.x, y:v.y}
}
