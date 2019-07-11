use glium::{Display, Frame, Texture2d};
use nalgebra::{Matrix4, Orthographic3, Vector2, Vector3};

use crate::player::Player;
use crate::resources::Resources;
use crate::sprite::SpriteRenderer;

const BACKGROUND_IMAGE: &[u8] = include_bytes!("../textures/background.jpg");
const PADDLE_IMAGE: &[u8] = include_bytes!("../textures/paddle.png");
const SPRITE_VERT: &str = include_str!("../shaders/sprite.vs");
const SPRITE_FRAG: &str = include_str!("../shaders/sprite.fs");

pub struct Game<'a> {
    pub resources: Resources,
    pub display: &'a Display,
    player: Player,
    state: GameState,
    level: usize,
}

impl<'a> Game<'a> {
    pub fn new(display: &'a Display) -> Self {
        let mut resources = Resources::default();
        resources.load_image_from_memory(display, "background", &BACKGROUND_IMAGE, false);
        resources.load_image_from_memory(display, "paddle", &PADDLE_IMAGE, true);
        resources.load_shader(display, "sprite", &SPRITE_VERT, &SPRITE_FRAG);

        let player = Player::new();

        Game {
            resources,
            display,
            player,
            state: GameState::Active,
            level: 0,
        }
    }

    pub fn get_renderer<'b>(&self, target: &'b mut Frame) -> SpriteRenderer<'b, '_> {
        let program = self.resources.get_shader("sprite").unwrap();
        SpriteRenderer::new(self, target)
    }

    pub fn render(&self, renderer: &mut SpriteRenderer) {
        match &self.state {
            GameState::Active => {}
            GameState::Menu => {}
            GameState::Win => {}
        }
        renderer.render_sprite_by_name("background", [0.0, 0.0], [1024.0, 768.0]);
    }
}

enum GameState {
    Active,
    Menu,
    Win,
}
