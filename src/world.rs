use crate::bullet::Bullet;
use crate::explosion::particle_explosion;
use crate::meteor::{Meteor, MeteorSize};
use crate::ship::Ship;
use crate::textures::Textures;
use macroquad::prelude::*;
use macroquad_particles::Emitter;

pub struct World {
    ship: Ship,
    meteors: Vec<Meteor>,
    bullets: Vec<Bullet>,
    explosion_emitter: Emitter,
}

impl World {
    pub fn new(textures: Textures) -> Self {
        let meteor_texture = textures.meteor;
        let mut meteors = Vec::new();
        meteors.push(Meteor::new(
            Vec2::new(screen_width() / 3.0, screen_height() / 2.0),
            MeteorSize::Large,
            Vec2::new(160.0, -40.0),
            meteor_texture.clone(),
        ));
        meteors.push(Meteor::new(
            Vec2::new(screen_width() / 4.0, screen_height() / 4.0),
            MeteorSize::Large,
            Vec2::new(-120.0, 80.0),
            meteor_texture.clone(),
        ));
        meteors.push(Meteor::new(
            Vec2::new(screen_width() * 3.0 / 4.0, screen_height() * 3.0 / 4.0),
            MeteorSize::Large,
            Vec2::new(60.0, 100.0),
            meteor_texture,
        ));
        Self {
            ship: Ship::new(Vec2::new(screen_width() / 2.0, screen_height() / 2.0)),
            meteors,
            bullets: Vec::new(),
            explosion_emitter: Emitter::new(particle_explosion()),
        }
    }

    pub fn update(&mut self) {
        self.ship.update();

        if let Some((position, direction)) = self.ship.try_fire() {
            self.bullets.push(Bullet::new(position, direction));
        }

        let mut additional_meteors: Vec<Meteor> = Vec::new();

        for bullet in &mut self.bullets {
            bullet.update();

            for meteor in &mut self.meteors {
                if is_colliding(
                    bullet.position(),
                    bullet.radius(),
                    meteor.position(),
                    meteor.radius(),
                ) {
                    self.explosion_emitter.emit(bullet.position(), 10);
                    bullet.collided();
                    additional_meteors.extend(meteor.handle_bullet_hit());
                    break;
                }
            }
        }

        self.meteors.extend(additional_meteors);

        self.bullets.retain(|bullet| !bullet.is_expired());
        self.meteors.retain(|meteor| !meteor.is_expired());

        let mut crashed = false;

        for meteor in &mut self.meteors {
            meteor.update();
            if !self.ship.invulnerable()
                && is_colliding(
                    self.ship.position(),
                    self.ship.collision_radius(),
                    meteor.position(),
                    meteor.collision_radius(),
                )
            {
                crashed = true;
            }
        }

        if crashed {
            self.crash();
        }
    }

    pub fn draw(&mut self) {
        self.ship.draw();
        for meteor in &self.meteors {
            meteor.draw();
        }
        for bullet in &self.bullets {
            bullet.draw();
        }
        self.explosion_emitter.draw(Vec2::ZERO);
    }

    fn crash(&mut self) {
        self.explosion_emitter.emit(self.ship.position(), 50);

        self.ship.reset_position();
        self.ship.set_invulnerable();
    }
}

pub fn wrap_to_world_coordinates(position: Vec2) -> Vec2 {
    let w = screen_width();
    let h = screen_height();
    Vec2::new(position.x.rem_euclid(w), position.y.rem_euclid(h))
}

/// Check whether two elements are colliding. For simplicity, we model both
/// elements as circles (elements like the space ship can be modeled as a smaller
/// circle which will give some false negatives and some false positives, but
/// we'll tune this so that it's mostly in the favor of the player).
pub fn is_colliding(a: Vec2, a_radius: f32, b: Vec2, b_radius: f32) -> bool {
    a.distance(b) <= a_radius + b_radius
}
