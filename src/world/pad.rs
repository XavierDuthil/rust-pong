use crate::config::*;
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, GameResult};
use cgmath::*;
use crate::world::body::{Body, Shape};
use crate::world::intoMintVec2;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Action {
    Idle,
    MoveUp,
    MoveDown,
}

#[derive(Clone, Debug)]
pub struct Pad {
    pub pos: Vector2<f32>,
    pub movement: Vector2<f32>,
    pub size: Vector2<f32>,
    pub action: Action,
}

impl Pad {
    pub fn move_down(&mut self) {
        self.action = Action::MoveDown;
    }

    pub fn move_up(&mut self) {
        self.action = Action::MoveUp;
    }

    pub fn stop_move_up(&mut self) {
        if self.action == Action::MoveUp {
            self.action = Action::Idle
        }
    }

    pub fn stop_move_down(&mut self) {
        if self.action == Action::MoveDown {
            self.action = Action::Idle
        }
    }
}

impl Body for Pad {
    fn other_side_positions(&self) -> Vec<Vector2<f32>> {
        let radius = self.pos.x + self.pos.y;
        let mut other_positions = vec![];
        if self.pos.x - radius < 0.0 {
            other_positions.push(vec2(SCREEN_WIDTH + self.pos.x, self.pos.y));
        } else if self.pos.x + radius > SCREEN_WIDTH {
            other_positions.push(vec2(self.pos.x - SCREEN_WIDTH, self.pos.y));
        }

        if self.pos.y - radius < 0.0 {
            other_positions.push(vec2(self.pos.x, SCREEN_HEIGHT + self.pos.y));
        } else if self.pos.y + radius > SCREEN_HEIGHT {
            other_positions.push(vec2(self.pos.x, self.pos.y - SCREEN_HEIGHT));
        }

        if other_positions.len() == 2 {
            other_positions.push(other_positions[0] + other_positions[1] - self.pos);
        }
        other_positions
    }

    fn update(&mut self, dt: f32) {
        match self.action {
            Action::MoveUp => self.pos.y -= self.movement.y * dt,
            Action::MoveDown => self.pos.y += self.movement.y * dt,
            Action::Idle => (),
        }

        self.pos.x = self.pos.x.rem_euclid(SCREEN_WIDTH);
        self.pos.y = self.pos.y.rem_euclid(SCREEN_HEIGHT);
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect{x: -self.size.x/2.0, y: -self.size.y/2.0, w: self.size.x, h: self.size.y},
            Color::GREEN,
        )?;
        graphics::draw(ctx, &circle, (intoMintVec2(self.pos),))?;

        for other_pos in self.other_side_positions() {
            graphics::draw(ctx, &circle, (intoMintVec2(other_pos),))?;
        }
        Ok(())
    }

    fn shape(&mut self) -> (Shape, &mut Vector2<f32>) {
       (Shape::Rectangle(&mut self.pos, &mut self.size), &mut self.movement)
    }
}
