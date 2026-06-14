use macroquad::prelude::*;

use crate::draw::draw_wrapped_circle;
use crate::world::wrap_to_world_coordinates;

pub struct Meteor {
    position: Vec2,
    speed: Vec2,
    radius: f32,
    color: Color,
}

impl Meteor {
    pub fn new(position: Vec2, radius: f32, color: Color, speed: Vec2) -> Self {
        Self {
            position,
            radius,
            color,
            speed,
        }
    }

    /// Getter method for logging
    #[allow(dead_code)]
    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn update(&mut self) {
        self.position = wrap_to_world_coordinates(self.position + self.speed * get_frame_time());
    }

    pub fn draw(&self) {
        draw_wrapped_circle(self.position, self.radius, self.color);
    }
}
