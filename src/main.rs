extern crate piston_window;
extern crate opengl_graphics;

use piston_window::{Button,EventLoop, Input, OpenGL, PistonWindow, WindowSettings,Motion, Key};
use opengl_graphics::GlGraphics;
use self::resources::Resources;
use game::input::{InputController, Actions};
use self::point::Point;

mod point;
mod simulation;
mod render;
mod game;
mod resources;

const NUM_PLAYERS: usize = 2;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut settings = WindowSettings::new(
        "Amazing Grame", [1920 as u32, 1080 as u32])
        .opengl(opengl).samples(8).fullscreen(false);
    let mut window: PistonWindow = settings.build().unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);
    let mut resources = Resources::new();
    let dimensions = window.output_color.get_dimensions();
    let arena_size = Point{x: (dimensions.0 as f64), y: (dimensions.1 as f64)};
    let mut game = game::Game::new(arena_size);

    let mut input_controller = InputController::new(NUM_PLAYERS);

    while let Some(e) = window.next() {
        match e {
            Input::Press(Button::Keyboard(key)) => {
                match key {
                    Key::F8 => {
                        let dimensions = window.output_color.get_dimensions();
                        let arena_size = Point{x: (dimensions.0 as f64), y: (dimensions.1 as f64)};
                        game = game::Game::new(arena_size);
                    }
                    _ => {}
                }
                input_controller.key_press(key);
            }

            Input::Release(Button::Keyboard(key)) => {
                input_controller.key_release(key);
            }

            Input::Update(_) => {
                game.control(input_controller.actions());
                game.timestep();
                input_controller.reset();
            }

            Input::Render(args) => {
                gl.draw(args.viewport(), |context, gl| render::render(context, gl, &mut resources, &game, input_controller.draw_help));
            }

            _ => {}
        }
    }
}
