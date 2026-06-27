use macroquad::prelude::*;

use crate::draw::draw_wrapped_circle;
use crate::world::wrap_to_world_coordinates;

pub struct Bullet {
    position: Vec2,
    velocity: Vec2,
    lifetime: f32,
}

const BULLET_RADIUS: f32 = 2.0;
const BULLET_SPEED: f32 = 400.0;
const BULLET_LIFETIME: f32 = 1.0;

impl Bullet {
    pub fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            velocity: direction.normalize() * BULLET_SPEED,
            lifetime: BULLET_LIFETIME,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn radius(&self) -> f32 {
        BULLET_RADIUS
    }

    pub fn update(&mut self) {
        self.position = wrap_to_world_coordinates(self.position + self.velocity * get_frame_time());
        self.lifetime -= get_frame_time();
    }

    pub fn is_expired(&self) -> bool {
        self.lifetime <= 0.0
    }

    pub fn collided(&mut self) {
        self.lifetime = 0.0;
    }

    pub fn draw(&self) {
        draw_wrapped_circle(self.position, BULLET_RADIUS, WHITE);
    }
}
