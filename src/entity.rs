use glium::Texture2d;
use nalgebra::{Vector2, Vector3};

use crate::sprite::SpriteRenderer;

pub trait Entity {
    fn get_sprite(&self) -> &str;
    fn get_position(&self) -> Vector2<f32>;
    fn get_size(&self) -> Vector2<f32>;

    fn get_color(&self) -> Vector3<f32> {
        [1.0, 1.0, 1.0].into()
    }

    fn render(&self, renderer: &mut SpriteRenderer) {
        let sprite = self.get_sprite();
        let position = self.get_position();
        let size = self.get_size();
        let color = self.get_color();
        renderer.render_sprite_by_name(sprite, position, size, color);
    }
}
