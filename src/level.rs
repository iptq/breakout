use std::collections::HashMap;

use glium::Texture2d;
use nalgebra::{Vector2, Vector3};

use crate::ball::Ball;
use crate::entity::Entity;
use crate::math::calculate_vector_direction;
use crate::resources::Resources;
use crate::sprite::SpriteRenderer;
use crate::{GAME_HEIGHT, GAME_WIDTH};

pub struct Level {
    pub map: HashMap<(usize, usize), Brick>,
}

#[derive(Serialize, Deserialize)]
struct LevelData {
    rows: u32,
    cols: u32,
    map: Vec<Vec<u32>>,
}

impl Level {
    pub fn from_json(data: impl AsRef<str>) -> Self {
        let data = serde_json::from_str::<LevelData>(data.as_ref()).unwrap();
        let mut map = HashMap::new();

        let unit_width = GAME_WIDTH as f32 / data.rows as f32;
        let unit_height = GAME_HEIGHT as f32 / 2.0 / data.cols as f32;

        for (i, row) in data.map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell != 0 {
                    let color = match *cell {
                        1 => [0.8, 0.8, 0.7].into(),
                        2 => [0.2, 0.6, 1.0].into(),
                        3 => [0.0, 0.7, 0.0].into(),
                        4 => [0.8, 0.8, 0.4].into(),
                        5 => [1.0, 0.5, 0.0].into(),
                        _ => unreachable!(),
                    };
                    let brick = Brick {
                        position: [unit_width * j as f32, unit_height * i as f32].into(),
                        size: [unit_width, unit_height].into(),
                        destructible: *cell != 1,
                        color,
                        destroyed: false,
                    };
                    map.insert((i, j), brick);
                }
            }
        }

        Level { map }
    }

    pub fn render(&self, renderer: &mut SpriteRenderer) {
        for ((x, y), brick) in self.map.iter() {
            if !brick.is_destroyed() {
                brick.render(renderer);
            }
        }
    }

    pub fn perform_collisions(&mut self, position: Vector2<f32>, radius: f32) -> CollisionResult {
        for brick in self.map.values_mut() {
            if brick.is_destructible() && !brick.is_destroyed() {
                if let CollisionResult::Hit(direction, asdf) = brick.collides_with(position, radius)
                {
                    brick.destroy();
                }
            }
        }
    }
}

pub struct Brick {
    position: Vector2<f32>,
    size: Vector2<f32>,
    destructible: bool,
    color: Vector3<f32>,
    destroyed: bool,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum CollisionResult {
    Hit(Direction, Vector2<f32>),
    Miss,
}

impl Brick {
    pub fn destroy(&mut self) {
        self.destroyed = true;
    }

    pub fn is_destroyed(&self) -> bool {
        self.destroyed
    }

    pub fn is_destructible(&self) -> bool {
        self.destructible
    }

    pub fn collides_with(&self, position: Vector2<f32>, radius: f32) -> CollisionResult {
        let center = position + radius * Vector2::identity();
        let half_extents = self.get_size() / 2.0;
        let brick_center = self.get_position() + half_extents;

        let difference = center - brick_center;
        let clamped = glm::clamp_vec(&difference, &-half_extents, &half_extents);
        let closest = brick_center + clamped;
        let difference = closest - center;

        if glm::length(&difference) < radius {
            let direction = calculate_vector_direction(&difference);
            CollisionResult::Hit(direction, difference)
        } else {
            CollisionResult::Miss
        }
    }
}

impl Entity for Brick {
    fn get_sprite(&self) -> &str {
        if self.destructible {
            "block"
        } else {
            "block_solid"
        }
    }

    fn get_position(&self) -> Vector2<f32> {
        self.position
    }

    fn get_size(&self) -> Vector2<f32> {
        self.size
    }

    fn get_color(&self) -> Vector3<f32> {
        self.color
    }
}
