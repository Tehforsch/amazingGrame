mod draw;

use opengl_graphics::GlGraphics;
use piston_window::{self, Context};

use self::draw::circle;
use simulation::Simulation;

pub fn render(context: Context, gl: &mut GlGraphics, sim: &mut Simulation) {
    piston_window::clear([0.0, 0.0, 0.0, 1.0], gl);
    for body in &sim.bodies {
        circle(body.pos, body.radius, [1.0, 0.0, 0.0, 1.0], context, gl);
    }
}
