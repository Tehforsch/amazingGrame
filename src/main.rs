extern crate piston_window;
extern crate opengl_graphics;

use piston_window::{EventLoop, Input, OpenGL, PistonWindow, WindowSettings};
use opengl_graphics::GlGraphics;

mod point;
mod simulation;
mod render;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: PistonWindow = WindowSettings::new(
        "Template", [1024 as u32, 600 as u32])
        .opengl(opengl).samples(8).exit_on_esc(true).build().unwrap();

    window.set_ups(60);
    window.set_max_fps(60);

    let mut gl = GlGraphics::new(opengl);

    let mut sim = simulation::initialize_random_sim(50);

    while let Some(e) = window.next() {
        match e {
            Input::Update(_) => {
                sim.timestep();
            }

            Input::Render(args) => {
                gl.draw(args.viewport(), |context, gl| render::render(context, gl, &mut sim));
            }

            _ => {}
        }
    }
}
