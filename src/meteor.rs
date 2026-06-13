use macroquad::prelude::*;

use crate::draw_wrapped_circle;

pub struct Meteor {
    position: Vec2,
    speed: Vec2,
    radius: f32,
    color: Color,
}

impl Meteor {
    pub fn new(position: Vec2, radius: f32, color: Color, speed: Vec2) -> Self {
        Self { position, radius, color, speed }
    }

    pub fn update(&mut self) {
        self.position += self.speed * get_frame_time();
    }

    pub fn draw(&self) {
        draw_wrapped_circle(self.position.x, self.position.y, self.radius, self.color);
    }
}
