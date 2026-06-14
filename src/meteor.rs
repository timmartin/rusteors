use macroquad::prelude::*;

use crate::draw::draw_wrapped_circle;

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

    /// Getter method for logging
    #[allow(dead_code)]
    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn update(&mut self) {
        fn wrap(value: f32, max: f32) -> f32 {
            let wrapped = value % max;
            if wrapped < 0.0 {
                wrapped + max
            } else {
                wrapped
            }
        }

        self.position = Vec2::new(
            wrap(self.position.x + self.speed.x * get_frame_time(), screen_width()),
            wrap(self.position.y + self.speed.y * get_frame_time(), screen_height()),
        );
    }

    pub fn draw(&self) {
        draw_wrapped_circle(self.position, self.radius, self.color);
    }
}
