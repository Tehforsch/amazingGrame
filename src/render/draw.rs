use piston_window;
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

pub fn line(start: Point, end: Point, color: Color, context: Context, gl: &mut GlGraphics) {
    piston_window::line(color, 1., [ start.x, start.y, end.x, end.y ], context.transform, gl);
}
