use std::collections::HashMap;

use glium::Texture2d;
use nalgebra::Vector2;

use crate::entity::Entity;
use crate::resources::Resources;
use crate::sprite::SpriteRenderer;

pub struct Level {
    map: HashMap<(usize, usize), Brick>,
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
        for (i, row) in data.map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let brick = Brick {
                    position: [100.0 * i as f32, 100.0 * j as f32].into(),
                    size: [100.0, 100.0].into(),
                    destructible: *cell == 1,
                    color: *cell,
                    destroyed: false,
                };
                map.insert((i, j), brick);
            }
        }

        Level { map }
    }

    pub fn render(&self, renderer: &mut SpriteRenderer) {
        for ((x, y), brick) in self.map.iter() {
            brick.render(renderer);
        }
    }
}

struct Brick {
    position: Vector2<f32>,
    size: Vector2<f32>,
    destructible: bool,
    color: u32,
    destroyed: bool,
}

impl Entity for Brick {
    fn get_sprite(&self) -> &str {
        "block"
    }

    fn get_position(&self) -> Vector2<f32> {
        self.position
    }

    fn get_size(&self) -> Vector2<f32> {
        self.size
    }
}
