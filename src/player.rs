use std::time::Duration;

use nalgebra::Vector2;

use crate::entity::Entity;
use crate::sprite::SpriteRenderer;
use crate::{GAME_HEIGHT, GAME_WIDTH};

pub struct Player {
    position: Vector2<f32>,
    size: Vector2<f32>,
    velocity: f32,
}

impl Player {
    pub fn new() -> Self {
        let size: Vector2<f32> = [100.0, 20.0].into();
        let position = [
            (GAME_WIDTH as f32 - size[0]) / 2.0,
            GAME_HEIGHT as f32 - size[1],
        ]
        .into();
        Player {
            position,
            size,
            velocity: 500.0,
        }
    }

    pub fn move_left(&mut self, delta: Duration) -> f32 {
        let velocity = self.velocity * delta.as_millis() as f32 / 1000.0;
        if self.position[0] > 0.0 {
            self.position[0] -= velocity;
            return -velocity;
        }
        0.0
    }

    pub fn move_right(&mut self, delta: Duration) -> f32 {
        let velocity = self.velocity * delta.as_millis() as f32 / 1000.0;
        if self.position[0] < (GAME_WIDTH as f32 - self.size[0]) {
            self.position[0] += velocity;
            return velocity;
        }
        0.0
    }
}

impl Entity for Player {
    fn get_sprite(&self) -> &str {
        "paddle"
    }

    fn get_position(&self) -> Vector2<f32> {
        self.position
    }

    fn get_size(&self) -> Vector2<f32> {
        self.size
    }
}
