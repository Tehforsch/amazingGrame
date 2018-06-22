mod draw;

use opengl_graphics::GlGraphics;
use piston_window::{self, Context, Transformed};

use self::draw::color;
use self::draw::circle;
use simulation::body::Body;
use ::point::Point;
use game::object::Object;
use game::object::ObjectType::*;
use game::Game;

pub const SHIP_POLYGON: &'static [[f64; 2]] = &[
    [0.0, -8.0],
    [20.0, 0.0],
    [0.0, 8.0]
];

pub fn render(context: Context, gl: &mut GlGraphics, game: &Game) {
    piston_window::clear([0.0, 0.0, 0.0, 1.0], gl);
    for object in game.objects.iter() {
        match object.type_ {
            Ship => render_ship(context, gl, &game.sim.bodies[object.body]),
            Star => render_body(context, gl, &game.sim.bodies[object.body]),
            Bullet => render_body(context, gl, &game.sim.bodies[object.body])
        }
    }
}

fn render_body(context: Context, gl: &mut GlGraphics, body: &Body) {
    circle(body.pos, body.radius, [1.0, 0.0, 0.0, 1.0], context, gl);
    circle(body.pos + Point{x: body.radius, y: 0.0}.rotate(body.apos), 5.0, [0.0, 0.0, 0.0, 1.0], context, gl);
}

fn render_ship(context: Context, gl: &mut GlGraphics, ship: &Body) {
    // Set the center of the player as the origin and rotate it
    let transform = context.transform
        .trans(ship.pos.x, ship.pos.y)
        .rot_rad(ship.apos);
    piston_window::polygon(color::RED, SHIP_POLYGON, transform, gl);
}
