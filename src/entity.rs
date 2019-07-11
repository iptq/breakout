use glium::Texture2d;
use nalgebra::Vector2;

use crate::sprite::SpriteRenderer;

pub trait Entity {
    fn get_sprite(&self) -> &Texture2d;
    fn get_position(&self) -> Vector2<f32>;
    fn get_size(&self) -> Vector2<f32>;

    fn render(&self, renderer: &mut SpriteRenderer) {
        let sprite = self.get_sprite();
        let position = self.get_position();
        let size = self.get_size();
        renderer.render_sprite(sprite, position, size);
    }
}
