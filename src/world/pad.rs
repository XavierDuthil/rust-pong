use crate::config::*;
use ggez::conf::WindowMode;
use ggez::event;
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, GameResult};
use glam::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Action {
    Idle,
    MoveUp,
    MoveDown,
}

#[derive(Clone, Debug)]
pub struct Pad {
    pub pos: Vec2,
    pub speed: f32,
    pub size: Vec2,
    pub action: Action,
}

impl Pad {
    pub fn other_side_positions(&self) -> Vec<Vec2> {
        let radius = self.pos.x + self.pos.y;
        let mut other_positions = vec![];
        if self.pos.x - radius < 0.0 {
            other_positions.push(Vec2::new(SCREEN_WIDTH + self.pos.x, self.pos.y));
        } else if self.pos.x + radius > SCREEN_WIDTH {
            other_positions.push(Vec2::new(self.pos.x - SCREEN_WIDTH, self.pos.y));
        }

        if self.pos.y - radius < 0.0 {
            other_positions.push(Vec2::new(self.pos.x, SCREEN_HEIGHT + self.pos.y));
        } else if self.pos.y + radius > SCREEN_HEIGHT {
            other_positions.push(Vec2::new(self.pos.x, self.pos.y - SCREEN_HEIGHT));
        }

        if other_positions.len() == 2 {
            other_positions.push(other_positions[0] + other_positions[1] - self.pos);
        }
        other_positions
    }

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

    pub fn update(&mut self, dt: f32) {
        match self.action {
            Action::MoveUp => self.pos.y -= self.speed * dt,
            Action::MoveDown => self.pos.y += self.speed * dt,
            Action::Idle => (),
        }

        self.pos.x = self.pos.x.rem_euclid(SCREEN_WIDTH);
        self.pos.y = self.pos.y.rem_euclid(SCREEN_HEIGHT);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect{x: -self.size.x/2.0, y: -self.size.y/2.0, w: self.size.x, h: self.size.y},
            Color::GREEN,
        )?;
        graphics::draw(ctx, &circle, (self.pos,))?;

        for other_pos in self.other_side_positions() {
            graphics::draw(ctx, &circle, (other_pos,))?;
        }
        Ok(())
    }
}
