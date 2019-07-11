#[macro_use]
extern crate glium;
extern crate nalgebra_glm as glm;
#[macro_use]
extern crate serde_derive;

mod entity;
mod game;
mod level;
mod player;
mod resources;
mod sprite;

use crate::game::Game;
use crate::resources::Resources;

const GAME_WIDTH: u32 = 1024;
const GAME_HEIGHT: u32 = 768;

fn main() {
    use glium::glutin::{
        dpi::PhysicalSize, ContextBuilder, Event, EventsLoop, WindowBuilder, WindowEvent,
    };
    use glium::{Display, Rect};
    use nalgebra::Vector2;

    let mut events_loop = EventsLoop::new();
    let primary_monitor = events_loop.get_primary_monitor();
    let dpi_factor = primary_monitor.get_hidpi_factor();
    let dimensions: PhysicalSize = (GAME_WIDTH, GAME_HEIGHT).into();

    let wb = WindowBuilder::new()
        .with_dimensions(dimensions.to_logical(dpi_factor))
        .with_resizable(false)
        .with_title("Breakout");
    let cb = ContextBuilder::new();
    let display = Display::new(wb, cb, &events_loop).unwrap();

    let mut game = Game::new(&display);

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        let mut renderer = game.get_renderer(&mut target);
        game.render(&mut renderer);
        target.finish().unwrap();

        events_loop.poll_events(|event| match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => closed = true,
                _ => (),
            },
            _ => (),
        });
    }
}
