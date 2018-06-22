extern crate piston_window;
extern crate opengl_graphics;

use piston_window::{Button,EventLoop, Input, OpenGL, PistonWindow, WindowSettings,Motion};
use opengl_graphics::GlGraphics;

mod point;
mod simulation;
mod render;
mod game;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
        "Template", [1024 as u32, 600 as u32])
        .opengl(opengl).samples(8).exit_on_esc(true).build().unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);


    let mut game = game::Game::new();

    while let Some(e) = window.next() {
        match e {
            Input::Press(Button::Keyboard(key)) => {
                game.input_controller.key_press(key);
            }

            Input::Release(Button::Keyboard(key)) => {
                game.input_controller.key_release(key);
            }

            Input::Press(Button::Controller(button)) => {
                game.input_controller.button_press(button);
            }

            Input::Release(Button::Controller(button)) => {
                game.input_controller.button_release(button);
            }

            Input::Move(Motion::ControllerAxis(axis)) => {
                game.input_controller.handle_axis(axis);
            }

            Input::Update(_) => {
                game.timestep();
            }

            Input::Render(args) => {
                gl.draw(args.viewport(), |context, gl| render::render(context, gl, &game));
            }

            _ => {}
        }
    }
}
