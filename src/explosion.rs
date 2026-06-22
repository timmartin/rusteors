use macroquad::prelude::*;
use macroquad_particles;

pub fn particle_explosion() -> macroquad_particles::EmitterConfig {
    macroquad_particles::EmitterConfig {
        local_coords: false,
        lifetime: 0.5,
        lifetime_randomness: 0.4,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 120.0,
        initial_velocity_randomness: 1.0,
        size_randomness: 0.5,
        shape: macroquad_particles::ParticleShape::Circle { subdivisions: 10 },
        emitting: false,
        colors_curve: macroquad_particles::ColorCurve {
            start: RED,
            mid: ORANGE,
            end: RED,
        },
        ..Default::default()
    }
}
