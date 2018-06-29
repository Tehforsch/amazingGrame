extern crate piston_window;
extern crate opengl_graphics;

use piston_window::{Button,EventLoop, Input, OpenGL, PistonWindow, WindowSettings,Motion, Key};
use opengl_graphics::GlGraphics;
use self::resources::Resources;
use game::input::{InputController, Actions};

mod point;
mod simulation;
mod render;
mod game;
mod resources;

const NUM_PLAYERS: usize = 2;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
        "Template", [1024 as u32, 600 as u32])
        .opengl(opengl).samples(8).exit_on_esc(true).build().unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);
    let mut resources = Resources::new();
    let mut game = game::Game::new();

    let mut input_controller = InputController::new(NUM_PLAYERS);

    while let Some(e) = window.next() {
        match e {
            Input::Press(Button::Keyboard(key)) => {
                match key {
                    Key::F8 => {
                        game = game::Game::new();
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
            }

            Input::Render(args) => {
                gl.draw(args.viewport(), |context, gl| render::render(context, gl, &mut resources, &game, input_controller.draw_help));
            }

            _ => {}
        }
    }
}
