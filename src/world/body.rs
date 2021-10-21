use cgmath::*;
use cgmath::num_traits::Inv;
use collision::*;
use collision::primitive::Circle;
use ggez::{Context, GameResult};
use crate::config::BALL_COLLISION_SPEED_INCREMENT;

pub trait Body {
    fn other_side_positions(&self) -> Vec<Vector2<f32>>;
    fn update(&mut self, dt: f32);
    fn collide(&mut self, other: &mut dyn Body) {
        match self.shape() {
            (Shape::Circle(s_pos, s_radius), s_movement) => {
                match other.shape() {
                    (Shape::Rectangle(o_pos, o_size), o_movement) => {
                        let d = *o_pos - *s_pos;
                        let hrd = *o_size / 2.0; // Half ot the rectangle diagonal

                        let diff_x = d.x.abs() - *s_radius - hrd.x;
                        let diff_y = d.y.abs() - *s_radius - hrd.y;

                        // TODO: LOL WE CAN'T
                        let collision_point_x = d.x.abs() - (s_radius.powf(2.0) - (hrd.y - d.y.abs()).powf(2.0)).sqrt();
                        let collision_point_y = d.y.abs() - (s_radius.powf(2.0) - (hrd.x - d.x.abs()).powf(2.0)).sqrt();
                        if diff_x < 0.0 && collision_point_y < hrd.y {
                            s_pos.x += diff_x * d.x.signum();
                            s_movement.x = -(s_movement.x + s_movement.x.signum() * BALL_COLLISION_SPEED_INCREMENT);
                            // TODO: Set direction relative to impact point
                        } else if diff_y < 0.0 && collision_point_x < hrd.x {
                            s_pos.y += diff_y * d.y.signum();
                            s_movement.y = -(s_movement.y + s_movement.y.signum() * BALL_COLLISION_SPEED_INCREMENT);
                            // TODO: Set direction relative to impact point
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    fn draw(&self, ctx: &mut Context) -> GameResult;
    fn shape(&mut self) -> (Shape, &mut Vector2<f32>);
}

pub enum Shape<'a> {
    Rectangle(&'a mut Vector2<f32>, &'a mut Vector2<f32>),
    Circle(&'a mut Vector2<f32>, &'a mut f32),
}

