use macroquad::prelude::*;

use crate::draw::draw_wrapped_texture;
use crate::world::wrap_to_world_coordinates;

#[derive(Clone, Copy, PartialEq)]
pub enum MeteorSize {
    Large,
    Medium,
    Small,
}

impl MeteorSize {
    pub fn radius(&self) -> f32 {
        match self {
            MeteorSize::Large => 25.0,
            MeteorSize::Medium => 15.0,
            MeteorSize::Small => 10.0,
        }
    }
}

pub struct Meteor {
    position: Vec2,
    speed: Vec2,
    size: MeteorSize,

    /// Whether the meteor has been destroyed. This could live outside Meteor,
    /// but I'm considering having different meteor behavior e.g. where the
    /// meteor might be hit and throw off shards, but not be totally destroyed.
    /// Therefore the Meteor will be responsible for deciding when it's expired.
    is_expired: bool,

    texture: Texture2D,
}

impl Meteor {
    pub fn new(position: Vec2, size: MeteorSize, speed: Vec2, texture: Texture2D) -> Self {
        Self {
            position,
            size,
            speed,
            texture,
            is_expired: false,
        }
    }

    pub fn position(&self) -> Vec2 {
        self.position
    }

    pub fn radius(&self) -> f32 {
        self.size.radius()
    }

    pub fn is_expired(&self) -> bool {
        self.is_expired
    }

    pub fn handle_bullet_hit(&mut self) -> Vec<Meteor> {
        self.is_expired = true;

        let shard_split_angle: f32 = rand::gen_range(0.0, 2.0) * std::f32::consts::PI;

        let shard_directions = vec![
            Mat2::from_angle(shard_split_angle),
            Mat2::from_angle(shard_split_angle + std::f32::consts::PI),
        ];

        if self.size == MeteorSize::Large {
            shard_directions
                .iter()
                .map(|direction| {
                    Meteor::new(
                        self.position,
                        MeteorSize::Medium,
                        *direction * self.speed,
                        self.texture.clone(),
                    )
                })
                .collect()
        } else if self.size == MeteorSize::Medium {
            shard_directions
                .iter()
                .map(|direction| {
                    Meteor::new(
                        self.position,
                        MeteorSize::Small,
                        *direction * self.speed,
                        self.texture.clone(),
                    )
                })
                .collect()
        } else {
            vec![]
        }
    }

    pub fn collision_radius(&self) -> f32 {
        self.size.radius()
    }

    pub fn update(&mut self) {
        self.position = wrap_to_world_coordinates(self.position + self.speed * get_frame_time());
    }

    pub fn draw(&self) {
        draw_wrapped_texture(self.position, self.radius(), &self.texture);
    }
}
