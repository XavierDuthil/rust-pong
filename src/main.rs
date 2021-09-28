//! The simplest possible example that does something.
#![allow(clippy::unnecessary_wraps)]

use ggez::conf::WindowMode;
use ggez::event;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;

const SCREEN_HEIGHT: f32 = 1000.0;
const SCREEN_WIDTH: f32 = 1000.0;

#[derive(Clone, Debug)]
struct Ball {
    pos: Vec2,
    direction: f32,
    speed: f32,
    radius: f32,
    clipped: bool,
}

impl Ball {
    fn is_partially_out_of_screen(&self) -> Option<Vec2> {
        if self.clipped {
            None
        } else if self.pos.x - self.radius < 0.0 && self.direction.cos() < 0.0 {
            Some(Vec2::new(SCREEN_WIDTH + self.pos.x, self.pos.y))
        } else if self.pos.y - self.radius < 0.0 && self.direction.sin() < 0.0 {
            Some(Vec2::new(self.pos.x, SCREEN_HEIGHT + self.pos.y))
        } else if self.pos.x + self.radius > SCREEN_WIDTH && self.direction.cos() > 0.0 {
            Some(Vec2::new(self.pos.x - SCREEN_WIDTH, self.pos.y))
        } else if self.pos.y + self.radius > SCREEN_HEIGHT && self.direction.sin() > 0.0 {
            Some(Vec2::new(self.pos.x, self.pos.y - SCREEN_HEIGHT))
        } else {
            None
        }
    }
    fn is_completely_out_of_screen(&self) -> bool {
        self.pos.x + self.radius < 0.0
            || self.pos.y + self.radius < 0.0
            || self.pos.x - self.radius > SCREEN_WIDTH
            || self.pos.y - self.radius > SCREEN_HEIGHT
    }
    fn clip(self, new_pos: Vec2) -> Vec<Self> {
        let mut s = self.clone();
        s.clipped = true;
        vec![
            s,
            Ball {
                pos: new_pos,
                ..self
            },
        ]
    }
}

#[derive(Clone, Debug)]
struct Pad {}

#[derive(Debug)]
struct World {
    pads: [Pad; 2],
    balls: Vec<Ball>,
}

impl World {
    fn new() -> GameResult<World> {
        let p1 = Pad {};
        let p2 = Pad {};
        let b1 = Ball {
            pos: Vec2::new(30.0, 300.0),
            direction: std::f32::consts::PI,
            speed: 15.0,
            radius: 10.0,
            clipped: false,
        };

        let s = World {
            pads: [p1, p2],
            balls: vec![b1],
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for World {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for b in &mut self.balls {
            b.pos += Vec2::new(b.direction.cos(), -b.direction.sin()) * b.speed;
        }

        self.balls = self
            .balls
            .clone()
            .into_iter()
            .flat_map(|b| {
                if b.is_completely_out_of_screen() {
                    vec![]
                } else if let Some(new_pos) = b.is_partially_out_of_screen() {
                    b.clip(new_pos)
                } else {
                    vec![b]
                }
            })
            .collect();

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for b in &self.balls {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                b.pos,
                b.radius,
                2.0,
                Color::WHITE,
            )?;
            graphics::draw(ctx, &circle, (Vec2::ZERO,))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let mut cb = ggez::ContextBuilder::new("super_simple", "ggez").window_mode(WindowMode {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        ..Default::default()
    });
    let (ctx, event_loop) = cb.build()?;
    let state = World::new()?;
    event::run(ctx, event_loop, state)
}
