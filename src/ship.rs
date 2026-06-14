use macroquad::prelude::*;

use crate::draw::draw_wrapped_triangle;
use crate::world::wrap_to_world_coordinates;

pub struct Ship {
    position: Vec2,
    velocity: Vec2,
    angle: f32,
}

const SHIP_SIZE: f32 = 15.0;
const ROTATION_SPEED: f32 = 3.0;
const THRUST_ACCELERATION: f32 = 300.0;

impl Ship {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            angle: -std::f32::consts::FRAC_PI_2,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        if is_key_down(KeyCode::Left) {
            self.angle -= ROTATION_SPEED * dt;
        }
        if is_key_down(KeyCode::Right) {
            self.angle += ROTATION_SPEED * dt;
        }
        if is_key_down(KeyCode::Space) {
            let direction = Vec2::new(self.angle.cos(), self.angle.sin());
            self.velocity += direction * THRUST_ACCELERATION * dt;
        }

        self.position = wrap_to_world_coordinates(self.position + self.velocity * dt);
    }

    pub fn draw(&self) {
        let (nose, wing1, wing2) = self.triangle_vertices();
        draw_wrapped_triangle(self.position, SHIP_SIZE, nose, wing1, wing2, WHITE);
    }

    fn triangle_vertices(&self) -> (Vec2, Vec2, Vec2) {
        let cos = self.angle.cos();
        let sin = self.angle.sin();
        let nose = self.position + Vec2::new(cos, sin) * SHIP_SIZE;
        let wing1 = self.position
            + Vec2::new((self.angle + 2.4).cos(), (self.angle + 2.4).sin()) * SHIP_SIZE * 0.6;
        let wing2 = self.position
            + Vec2::new((self.angle - 2.4).cos(), (self.angle - 2.4).sin()) * SHIP_SIZE * 0.6;
        (nose, wing1, wing2)
    }
}
