use glium::draw_parameters::{Blend, DrawParameters};
use glium::index::{NoIndices, PrimitiveType};
use glium::{Display, Frame, Program, Surface, Texture2d, VertexBuffer};
use nalgebra::{Matrix4, Vector2, Vector3};

use crate::game::Game;
use crate::resources::Resources;

pub struct SpriteRenderer<'a, 'b> {
    target: &'a mut Frame,
    display: &'b Display,
    program: &'b Program,
    resources: &'b Resources,
}

impl<'a, 'b> SpriteRenderer<'a, 'b> {
    pub fn new(game: &'b Game, target: &'a mut Frame) -> Self {
        let program = game.resources.get_shader("sprite").unwrap();
        let display = &game.display;
        let resources = &game.resources;
        SpriteRenderer {
            target,
            program,
            display,
            resources,
        }
    }

    pub fn render_sprite_by_name(
        &mut self,
        name: impl AsRef<str>,
        position: impl Into<Vector2<f32>>,
        size: impl Into<Vector2<f32>>,
        color: impl Into<Vector3<f32>>,
    ) {
        let texture = self.resources.get_texture(name.as_ref()).unwrap();
        self.render_sprite(texture, position, size, color)
    }

    pub fn render_sprite(
        &mut self,
        texture: &Texture2d,
        position: impl Into<Vector2<f32>>,
        size: impl Into<Vector2<f32>>,
        color: impl Into<Vector3<f32>>,
    ) {
        let position = position.into();
        let size = size.into();
        let color = color.into();

        #[derive(Copy, Clone)]
        struct Vertex {
            position: [f32; 2],
            tex_coords: [f32; 2],
        }

        implement_vertex!(Vertex, position, tex_coords);
        let vertex_buffer = VertexBuffer::new(
            self.display,
            &vec![
                Vertex {
                    position: [0.0, 1.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [1.0, 0.0],
                    tex_coords: [1.0, 0.0],
                },
                Vertex {
                    position: [0.0, 0.0],
                    tex_coords: [0.0, 0.0],
                },
                Vertex {
                    position: [0.0, 1.0],
                    tex_coords: [0.0, 1.0],
                },
                Vertex {
                    position: [1.0, 1.0],
                    tex_coords: [1.0, 1.0],
                },
                Vertex {
                    position: [1.0, 0.0],
                    tex_coords: [1.0, 0.0],
                },
            ],
        )
        .unwrap();
        let indices = NoIndices(PrimitiveType::TrianglesList);

        let projection = glm::ortho::<f32>(0.0, 1024.0, 768.0, 0.0, -1.0, 1.0);

        let mut matrix = Matrix4::<f32>::identity();
        matrix = matrix.append_nonuniform_scaling(&[size[0], size[1], 1.0].into());
        matrix = matrix.append_translation(&[position[0], position[1], 0.0].into());

        let uniforms = uniform! {
            matrix: *matrix.as_ref(),
            projection: *projection.as_ref(),
            tint: *color.as_ref(),
            tex: texture,
        };
        self.target
            .draw(
                &vertex_buffer,
                &indices,
                &self.program,
                &uniforms,
                &DrawParameters {
                    blend: Blend::alpha_blending(),
                    ..Default::default()
                },
            )
            .unwrap();
    }
}
