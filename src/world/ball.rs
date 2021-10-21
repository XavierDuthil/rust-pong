use crate::config::*;
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult};
use cgmath::*;
use crate::world::body::{Body, Shape};
use crate::world::intoMintVec2;

#[derive(Clone, Debug)]
pub struct Ball {
    pub pos: Vector2<f32>,
    pub movement: Vector2<f32>,
    pub radius: f32,
}

impl Body for Ball {
    fn other_side_positions(&self) -> Vec<Vector2<f32>> {
        let mut other_positions = vec![];
        if self.pos.x - self.radius < 0.0 {
            other_positions.push(vec2(SCREEN_WIDTH + self.pos.x, self.pos.y));
        } else if self.pos.x + self.radius > SCREEN_WIDTH {
            other_positions.push(vec2(self.pos.x - SCREEN_WIDTH, self.pos.y));
        }

        if self.pos.y - self.radius < 0.0 {
            other_positions.push(vec2(self.pos.x, SCREEN_HEIGHT + self.pos.y));
        } else if self.pos.y + self.radius > SCREEN_HEIGHT {
            other_positions.push(vec2(self.pos.x, self.pos.y - SCREEN_HEIGHT));
        }

        if other_positions.len() == 2 {
            other_positions.push(other_positions[0] + other_positions[1] - self.pos);
        }
        other_positions
    }

    fn update(&mut self, dt: f32) {
        self.pos += self.movement * dt;
        self.pos.x = self.pos.x.rem_euclid(SCREEN_WIDTH);
        self.pos.y = self.pos.y.rem_euclid(SCREEN_HEIGHT);
    }

    fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            ggez::mint::Vector2{x:0.0, y:0.0},
            self.radius,
            0.1,
            Color::WHITE,
        )?;
        graphics::draw(ctx, &circle, (intoMintVec2(self.pos),))?;

        for other_pos in self.other_side_positions() {
            graphics::draw(ctx, &circle, (intoMintVec2(other_pos),))?;
        }
        Ok(())
    }

    fn shape(&mut self) -> (Shape, &mut Vector2<f32>) {
        (Shape::Circle(&mut self.pos, &mut self.radius), &mut self.movement)
    }
}
