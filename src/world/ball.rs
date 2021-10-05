use crate::config::*;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use glam::*;

#[derive(Clone, Debug)]
pub struct Ball {
    pub pos: Vec2,
    pub direction: f32,
    pub speed: f32,
    pub radius: f32,
}

impl Ball {
    pub fn other_side_positions(&self) -> Vec<Vec2> {
        let mut other_positions = vec![];
        if self.pos.x - self.radius < 0.0 {
            other_positions.push(Vec2::new(SCREEN_WIDTH + self.pos.x, self.pos.y));
        } else if self.pos.x + self.radius > SCREEN_WIDTH {
            other_positions.push(Vec2::new(self.pos.x - SCREEN_WIDTH, self.pos.y));
        }

        if self.pos.y - self.radius < 0.0 {
            other_positions.push(Vec2::new(self.pos.x, SCREEN_HEIGHT + self.pos.y));
        } else if self.pos.y + self.radius > SCREEN_HEIGHT {
            other_positions.push(Vec2::new(self.pos.x, self.pos.y - SCREEN_HEIGHT));
        }

        if other_positions.len() == 2 {
            other_positions.push(other_positions[0] + other_positions[1] - self.pos);
        }
        other_positions
    }

    pub fn update(&mut self, dt: f32) {

        self.pos += Vec2::new(self.direction.cos(), -self.direction.sin()) * self.speed * dt;
        self.pos.x = self.pos.x.rem_euclid(SCREEN_WIDTH);
        self.pos.y = self.pos.y.rem_euclid(SCREEN_HEIGHT);
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::ZERO,
            self.radius,
            0.1,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &circle, (self.pos,))?;

        for other_pos in self.other_side_positions() {
            graphics::draw(ctx, &circle, (other_pos,))?;
        }
        Ok(())
    }
}
