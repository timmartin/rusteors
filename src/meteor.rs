use macroquad::prelude::*;

use crate::draw::draw_wrapped_texture;
use crate::world::wrap_to_world_coordinates;

pub struct Meteor {
    position: Vec2,
    speed: Vec2,
    radius: f32,
    texture: Texture2D,
}

impl Meteor {
    pub fn new(position: Vec2, radius: f32, speed: Vec2, texture: Texture2D) -> Self {
        Self {
            position,
            radius,
            speed,
            texture,
        }
    }

    /// Getter method for logging
    #[allow(dead_code)]
    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn collision_radius(&self) -> f32 {
        self.radius
    }

    pub fn update(&mut self) {
        self.position = wrap_to_world_coordinates(self.position + self.speed * get_frame_time());
    }

    pub fn draw(&self) {
        draw_wrapped_texture(self.position, self.radius, &self.texture);
    }
}
