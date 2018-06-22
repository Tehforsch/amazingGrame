use piston_window::ellipse::Ellipse;
use piston_window::{Context, Transformed};
use piston_window::types::Color;
use opengl_graphics::GlGraphics;

use ::point::Point;

pub fn circle(pos: Point, radius: f64, color: Color, context: Context, gl: &mut GlGraphics) {
    Ellipse {
            color: color,
            border: None,
            resolution: 16,
    }.draw(
        [0.0, 0.0, 2.0*radius, 2.0*radius],
        &Default::default(),
        context.trans(pos.x-radius, pos.y-radius).transform,
        gl);
}

pub mod color {
    pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    pub const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
    pub const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 1.0];
    pub const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    pub const VIOLET: [f32; 4] = [0.6, 0.0, 1.0, 1.0];
    pub const YELLOW: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
}
