use crate::meteor::Meteor;
use macroquad::prelude::*;

pub struct World {
    meteors: Vec<Meteor>,
}

const INITIAL_METEOR_RADIUS: f32 = 25.0;

impl World {
    pub fn new() -> Self {
        let mut meteors = Vec::new();
        meteors.push(Meteor::new(
            Vec2::new(screen_width() / 2.0, screen_height() / 2.0),
            INITIAL_METEOR_RADIUS,
            BLUE,
            Vec2::new(160.0, -40.0),
        ));
        meteors.push(Meteor::new(
            Vec2::new(screen_width() / 4.0, screen_height() / 4.0),
            INITIAL_METEOR_RADIUS,
            RED,
            Vec2::new(-120.0, 80.0),
        ));
        meteors.push(Meteor::new(
            Vec2::new(screen_width() * 3.0 / 4.0, screen_height() * 3.0 / 4.0),
            INITIAL_METEOR_RADIUS,
            GREEN,
            Vec2::new(60.0, 100.0),
        ));
        Self { meteors }
    }

    pub fn update(&mut self) {
        for meteor in &mut self.meteors {
            meteor.update();
        }
    }

    pub fn draw(&self) {
        for meteor in &self.meteors {
            meteor.draw();
        }
    }
}
