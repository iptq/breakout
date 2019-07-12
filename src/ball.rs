use std::time::Duration;

use nalgebra::Vector2;

use crate::entity::Entity;
use crate::level::Brick;
use crate::{GAME_HEIGHT, GAME_WIDTH};

pub struct Ball {
    position: Vector2<f32>,
    velocity: Vector2<f32>,
    radius: f32,
    stuck: bool,
}

impl Ball {
    pub fn new(player: impl Into<Vector2<f32>>) -> Self {
        let player = player.into();
        let radius = 12.5;
        Ball {
            velocity: [100.0, -350.0].into(),
            position: [(GAME_WIDTH as f32 - radius) / 2.0, player[1] - radius * 2.0].into(),
            radius,
            stuck: true,
        }
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn unstick(&mut self) {
        self.stuck = false;
    }

    pub fn move_by(&mut self, by: f32) {
        if self.stuck {
            self.position[0] += by;
        }
    }

    pub fn update_position(&mut self, delta: Duration) {
        let delta = delta.as_millis() as f32 / 1000.0;
        if !self.stuck {
            self.position += self.velocity * delta;

            if self.position[0] <= 0.0 {
                self.velocity[0] = -self.velocity[0];
                self.position[0] = 0.0;
            } else if self.position[0] + self.radius >= GAME_WIDTH as f32 {
                self.velocity[0] = -self.velocity[0];
                self.position[0] = GAME_WIDTH as f32 - self.radius;
            }

            if self.position[1] <= 0.0 {
                self.velocity[1] = -self.velocity[1];
                self.position[1] = 0.0;
            }
        }
    }
}

impl Entity for Ball {
    fn get_sprite(&self) -> &str {
        "ball"
    }

    fn get_position(&self) -> Vector2<f32> {
        self.position
    }

    fn get_size(&self) -> Vector2<f32> {
        [self.radius * 2.0, self.radius * 2.0].into()
    }
}
