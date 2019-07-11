use std::collections::HashMap;
use std::time::Duration;

use glium::glutin::{Event, VirtualKeyCode};
use glium::{Display, Frame, Texture2d};
use nalgebra::{Matrix4, Orthographic3, Vector2, Vector3};

use crate::entity::Entity;
use crate::level::Level;
use crate::player::Player;
use crate::resources::Resources;
use crate::sprite::SpriteRenderer;

const BACKGROUND_IMAGE: &[u8] = include_bytes!("../textures/background.jpg");
const PADDLE_IMAGE: &[u8] = include_bytes!("../textures/paddle.png");
const BLOCK_IMAGE: &[u8] = include_bytes!("../textures/block.png");
const BLOCK_SOLID_IMAGE: &[u8] = include_bytes!("../textures/block_solid.png");
const SPRITE_VERT: &str = include_str!("../shaders/sprite.vs");
const SPRITE_FRAG: &str = include_str!("../shaders/sprite.fs");

const LEVEL_1: &str = include_str!("../levels/level1.json");
const LEVEL_2: &str = include_str!("../levels/level2.json");
const LEVEL_3: &str = include_str!("../levels/level3.json");
const LEVEL_4: &str = include_str!("../levels/level4.json");

pub struct Game<'a> {
    pub resources: Resources,
    pub display: &'a Display,
    pub levels: Vec<Level>,
    keymap: HashMap<VirtualKeyCode, bool>,
    player: Player,
    state: GameState,
    level: usize,
}

impl<'a> Game<'a> {
    pub fn new(display: &'a Display) -> Self {
        let mut resources = Resources::default();
        resources.load_image_from_memory(display, "background", &BACKGROUND_IMAGE, false);
        resources.load_image_from_memory(display, "paddle", &PADDLE_IMAGE, true);
        resources.load_image_from_memory(display, "block", &BLOCK_IMAGE, false);
        resources.load_image_from_memory(display, "block_solid", &BLOCK_SOLID_IMAGE, false);
        resources.load_shader(display, "sprite", &SPRITE_VERT, &SPRITE_FRAG);

        let levels = vec![
            Level::from_json(&LEVEL_1),
            Level::from_json(&LEVEL_2),
            Level::from_json(&LEVEL_3),
            Level::from_json(&LEVEL_4),
        ];
        let player = Player::new();

        Game {
            resources,
            display,
            levels,
            keymap: HashMap::new(),
            player,
            state: GameState::Active,
            level: 0,
        }
    }

    pub fn get_current_level(&self) -> &Level {
        self.levels.iter().nth(self.level).unwrap()
    }

    pub fn get_renderer<'b>(&self, target: &'b mut Frame) -> SpriteRenderer<'b, '_> {
        let program = self.resources.get_shader("sprite").unwrap();
        SpriteRenderer::new(self, target)
    }

    pub fn handle_event(&mut self, event: Event) {
        use glium::glutin::{ElementState, WindowEvent};
        match event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                if let Some(code) = input.virtual_keycode {
                    self.keymap.insert(
                        code,
                        match &input.state {
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                        },
                    );
                }
            }
            _ => (),
        }
    }

    fn is_key_pressed(&self, key: &VirtualKeyCode) -> bool {
        self.keymap.get(key).cloned().unwrap_or_else(|| false)
    }

    pub fn update(&mut self, delta: Duration) {
        match &self.state {
            GameState::Active => {
                if self.is_key_pressed(&VirtualKeyCode::Left) {
                    self.player.move_left(delta);
                } else if self.is_key_pressed(&VirtualKeyCode::Right) {
                    self.player.move_right(delta);
                }
            }
            GameState::Menu => {}
            GameState::Win => {}
        }
    }

    pub fn render(&self, renderer: &mut SpriteRenderer) {
        renderer.render_sprite_by_name("background", [0.0, 0.0], [1024.0, 768.0]);
        match &self.state {
            GameState::Active => {
                let level = self.get_current_level();
                level.render(renderer);
                self.player.render(renderer);
            }
            GameState::Menu => {}
            GameState::Win => {}
        }
    }
}

enum GameState {
    Active,
    Menu,
    Win,
}
