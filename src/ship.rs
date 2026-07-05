use macroquad::prelude::*;

use crate::draw::draw_wrapped_triangle;
use crate::world::wrap_to_world_coordinates;

pub struct Ship {
    position: Vec2,
    velocity: Vec2,
    angle: f32,
    invulnerable_time: Option<f32>,
    fire_cooldown: f32,
}

const SHIP_SIZE: f32 = 15.0;
const WING_SPREAD: f32 = 2.4;
const ROTATION_SPEED: f32 = 3.0;
const THRUST_ACCELERATION: f32 = 300.0;
const BLINK_INTERVAL: f32 = 0.1;
const FIRE_COOLDOWN: f32 = 0.25;

impl Ship {
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            angle: -std::f32::consts::FRAC_PI_2,
            invulnerable_time: None,
            fire_cooldown: 0.0,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    /// The radius of the ship's collision circle.
    ///
    /// The ship is not a circle of course, so we deliberately use a smaller
    /// radius to avoid false positives. The assumption is that a user getting
    /// away with a false negative will be happier than a user getting caught by a
    /// false positive.
    pub fn collision_radius(&self) -> f32 {
        SHIP_SIZE * 0.7
    }

    pub fn reset_position(&mut self) {
        self.position = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        self.velocity = Vec2::ZERO;
        self.angle = -std::f32::consts::FRAC_PI_2;
    }

    pub fn invulnerable(&self) -> bool {
        self.invulnerable_time.is_some()
    }

    pub fn set_invulnerable(&mut self) {
        self.invulnerable_time = Some(3.0);
    }

    /// Attempt to fire a bullet. Returns spawn position and direction if the
    /// cooldown has elapsed and space is held.
    pub fn try_fire(&mut self) -> Option<(Vec2, Vec2)> {
        if self.fire_cooldown > 0.0 || !is_key_down(KeyCode::Space) {
            return None;
        }

        self.fire_cooldown = FIRE_COOLDOWN;
        let direction = Vec2::from_angle(self.angle);
        let spawn_position = self.position + direction * SHIP_SIZE;
        Some((spawn_position, direction))
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        if is_key_down(KeyCode::Left) {
            self.angle -= ROTATION_SPEED * dt;
        }
        if is_key_down(KeyCode::Right) {
            self.angle += ROTATION_SPEED * dt;
        }
        if is_key_down(KeyCode::Up) {
            let direction = Vec2::from_angle(self.angle);
            self.velocity += direction * THRUST_ACCELERATION * dt;
        }

        self.position = wrap_to_world_coordinates(self.position + self.velocity * dt);

        self.fire_cooldown = (self.fire_cooldown - dt).max(0.0);

        if let Some(invulnerable_time) = self.invulnerable_time {
            self.invulnerable_time = Some(invulnerable_time - dt);
            if self.invulnerable_time <= Some(0.0) {
                self.invulnerable_time = None;
            }
        }
    }

    pub fn draw(&self) {
        let (nose, wing1, wing2) = self.triangle_vertices();

        let draw = if let Some(invulnerable_time) = self.invulnerable_time {
            ((invulnerable_time / BLINK_INTERVAL) as i32).rem_euclid(2) == 0
        } else {
            true
        };

        if draw {
            draw_wrapped_triangle(self.position, SHIP_SIZE, nose, wing1, wing2, WHITE);
        }
    }

    fn local_vertices() -> [Vec2; 3] {
        [
            Vec2::new(SHIP_SIZE, 0.0),
            Vec2::from_angle(WING_SPREAD) * SHIP_SIZE * 0.6,
            Vec2::from_angle(-WING_SPREAD) * SHIP_SIZE * 0.6,
        ]
    }

    fn triangle_vertices(&self) -> (Vec2, Vec2, Vec2) {
        let rotation = Mat2::from_angle(self.angle);
        let [nose, wing1, wing2] =
            Self::local_vertices().map(|vertex| self.position + rotation * vertex);
        (nose, wing1, wing2)
    }
}
