use macroquad::prelude::*;

use crate::draw_wrapped_circle;

pub struct Meteor {
    x: f32,
    y: f32,
    speed: f32,
    radius: f32,
    color: Color,
}

impl Meteor {
    pub fn new(x: f32, y: f32, radius: f32, color: Color, speed: f32) -> Self {
        Self { x, y, radius, color, speed }
    }

    pub fn update(&mut self) {
        self.x = (self.x + self.speed * get_frame_time()) % screen_width();
    }

    pub fn draw(&self) {
        draw_wrapped_circle(self.x, self.y, self.radius, self.color);
    }
}
