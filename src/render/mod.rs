mod draw;

use opengl_graphics::GlGraphics;
use piston_window::{self, Context, Transformed};

use self::draw::color;
use self::draw::circle;
use self::draw::line;
use simulation::body::Body;
use game::object::ObjectType::*;
use game::Game;

pub const SHIP_POLYGON: &'static [[f64; 2]] = &[
    [2.0, 0.0], [-0.4999999999999998, 0.8660254037844387], [-0.5000000000000004, -0.8660254037844384]
];

pub const STAR_POLYGON: &'static [[f64; 2]] = &[
    [1.0, 0.0], [-0.4999999999999998, 0.8660254037844387], [-0.5000000000000004, -0.8660254037844384]
];


pub fn render(context: Context, gl: &mut GlGraphics, game: &Game) {
    piston_window::clear([0.0, 0.0, 0.0, 1.0], gl);
    for object in game.objects.iter() {
        match object.type_ {
            Ship => render_ship(context, gl, game.sim.get_body(object.body)),
            Star => render_body(context, gl, game.sim.get_body(object.body)),
            Bullet(ship) => render_bullet(context, gl, game.sim.get_body(object.body), game.sim.get_body(ship))
        }
    }
    for spring in game.springs.iter() {
        render_spring(context, gl, game.sim.get_body(spring.body1), game.sim.get_body(spring.body2));
    }
}

fn render_bullet(context: Context, gl: &mut GlGraphics, body: &Body, ship: &Body) {
    circle(body.pos, body.radius, [0.0, 0.0, 1.0, 1.0], context, gl);
    line(body.pos, ship.pos, [0.0, 0.0, 1.0, 1.0], context, gl);
}

fn render_body(context: Context, gl: &mut GlGraphics, body: &Body) {
    // circle(body.pos + Point{x: body.radius, y: 0.0}.rotate(body.apos), 5.0, [0.0, 0.0, 0.0, 1.0], context, gl);
    let transform = context.transform
        .trans(body.pos.x, body.pos.y)
        .rot_rad(body.apos)
        .scale(body.radius, body.radius);
    piston_window::polygon(color::YELLOW, STAR_POLYGON, transform, gl);
    let transform = context.transform
        .trans(body.pos.x, body.pos.y)
        .rot_rad(body.apos+3.1415/3.0)
        .scale(body.radius, body.radius);
    piston_window::polygon(color::YELLOW, STAR_POLYGON, transform, gl);
}

fn render_ship(context: Context, gl: &mut GlGraphics, ship: &Body) {
    // Set the center of the player as the origin and rotate it
    let transform = context.transform
        .trans(ship.pos.x, ship.pos.y)
        .rot_rad(ship.apos)
        .scale(ship.radius, ship.radius);
    piston_window::polygon(color::RED, SHIP_POLYGON, transform, gl);
}

fn render_spring(context: Context, gl: &mut GlGraphics, body1: &Body, body2: &Body) {
    line(body1.pos, body2.pos, [0.0, 0.0, 1.0, 1.0], context, gl);
}
