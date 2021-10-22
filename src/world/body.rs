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

                        let diff_x = *s_radius + hrd.x - d.x.abs();
                        let diff_y = *s_radius + hrd.y - d.y.abs();

                        let collision_point_x = d.x.abs() - (s_radius.powf(2.0) - (hrd.y - d.y.abs()).powf(2.0)).sqrt();
                        let collision_point_y = d.y.abs() - (s_radius.powf(2.0) - (hrd.x - d.x.abs()).powf(2.0)).sqrt();
                        if diff_x > 0.0 && collision_point_y < hrd.y {
                            if !collision_point_x.is_nan() && collision_point_x < hrd.x && d.y.abs() > hrd.y {
                                s_pos.x -= (hrd.x - collision_point_x) * d.x.signum();
                            } else {
                                s_pos.x -= diff_x * d.x.signum();
                            }
                            let speed = s_movement.magnitude();
                            let d = (*s_pos - *o_pos).normalize();
                            *s_movement = d * (speed + BALL_COLLISION_SPEED_INCREMENT);
                            other.after_collide();
                        } else if diff_y > 0.0 && collision_point_x < hrd.x {
                            if !collision_point_y.is_nan() && collision_point_y < hrd.y && d.x.abs() > hrd.x {
                                s_pos.y -= (hrd.y - collision_point_y) * d.y.signum();
                            } else {
                                s_pos.y -= diff_y * d.y.signum();
                            }
                            s_movement.y = -(s_movement.y + s_movement.y.signum() * BALL_COLLISION_SPEED_INCREMENT);
                            other.after_collide();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    fn after_collide(&mut self, );
    fn draw(&self, ctx: &mut Context) -> GameResult;
    fn shape(&mut self) -> (Shape, &mut Vector2<f32>);
}

pub enum Shape<'a> {
    Rectangle(&'a mut Vector2<f32>, &'a mut Vector2<f32>),
    Circle(&'a mut Vector2<f32>, &'a mut f32),
}

