mod draw;

use opengl_graphics::GlGraphics;
use piston_window::{self, Context, Transformed};

use self::draw::circle;
use self::draw::line;
use simulation::body::Body;
use game::object::ObjectType::*;
use game::Game;
use resources::Resources;

pub const SHIP_POLYGON: &'static [[f64; 2]] = &[
    [1.0, 0.0], [-0.25, 0.433], [-0.25, -0.433]
];

pub const STAR_POLYGON: &'static [[f64; 2]] = &[
    [1.0, 0.0], [-0.4999999999999998, 0.8660254037844387], [-0.5000000000000004, -0.8660254037844384]
];

const BACKGROUND_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const BLACK_HOLE_COLOR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
const BULLET_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const SPRING_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
const STAR_COLOR: [f32; 4] = [1.0, 1.0, 0.0, 1.0];
const SHIP_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const MOTHERSHIP_COLOR: [f32; 4] = [0.0, 1.0, 0.3, 1.0];
const SCORE_COLOR: [f32; 4] = [1.0, 0.5, 0.5, 1.0];
const HELP_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub fn render(context: Context, gl: &mut GlGraphics, resources: &mut Resources, game: &Game, draw_help: bool) {
    piston_window::clear(BACKGROUND_COLOR, gl);
    for object in game.objects.iter() {
        match object.type_ {
            Ship => render_ship(context, gl, game.sim.get_body(object.body)),
            Star => render_body(context, gl, game.sim.get_body(object.body)),
            BlackHole => render_black_hole(context, gl, game.sim.get_body(object.body)),
            Mothership => render_mothership(context, gl, game.sim.get_body(object.body)),
            Bullet(ship, _) => render_bullet(context, gl, game.sim.get_body(object.body), game.sim.get_body(ship))
        }
    }
    for spring in game.springs.iter() {
        render_spring(context, gl, game.sim.get_body(spring.body1), game.sim.get_body(spring.body2));
    }
    // Score
    piston_window::text(SCORE_COLOR,
            22,
            &format!("Score: {}", game.score),
            &mut resources.font,
            context.trans(600.0, 20.0).transform,
            gl);
    // Help text
    if draw_help {
        print_help(context, gl, resources);
    }
    else {
        piston_window::text(HELP_COLOR,
            22,
            &format!("Press F1 for help."),
            &mut resources.font,
            context.trans(10.0, 20.0).transform,
            gl);
    }
}

fn print_help(context: Context, gl: &mut GlGraphics, resources: &mut Resources) {
    let help_text = "F1: hide help\nF8: restart\n\nPlayer one:\nw: forward\na: turn left\nd: turn right\nleft shift: shoot\n\nPlayer two:\nUp: forward\nLeft: turn left\nRight: turn right\nright shift: shoot\nGoal: \nBring the stars (yellow) to the mothership (green)\nDon't crash into the black holes (grey)\n";
    for (i, line) in help_text.split("\n").enumerate() {
        piston_window::text(HELP_COLOR,
            22,
            line,
            &mut resources.font,
            context.trans(10.0, 20.0 + (i as f64) * 30.0).transform,
            gl);
    }
}

fn render_mothership(context: Context, gl: &mut GlGraphics, body: &Body) {
    circle(body.pos, body.radius, MOTHERSHIP_COLOR, context, gl);
}

fn render_black_hole(context: Context, gl: &mut GlGraphics, body: &Body) {
    circle(body.pos, body.radius, BLACK_HOLE_COLOR, context, gl);
}

fn render_bullet(context: Context, gl: &mut GlGraphics, body: &Body, ship: &Body) {
    circle(body.pos, body.radius, BULLET_COLOR, context, gl);
    line(body.pos, ship.pos, BULLET_COLOR, context, gl);
}

fn render_body(context: Context, gl: &mut GlGraphics, body: &Body) {
    let transform = context.transform
        .trans(body.pos.x, body.pos.y)
        .rot_rad(body.apos)
        .scale(body.radius, body.radius);
    piston_window::polygon(STAR_COLOR, STAR_POLYGON, transform, gl);
    let transform = context.transform
        .trans(body.pos.x, body.pos.y)
        .rot_rad(body.apos+3.1415/3.0)
        .scale(body.radius, body.radius);
    piston_window::polygon(STAR_COLOR, STAR_POLYGON, transform, gl);
}

fn render_ship(context: Context, gl: &mut GlGraphics, ship: &Body) {
    // Set the center of the player as the origin and rotate it
    let transform = context.transform
        .trans(ship.pos.x, ship.pos.y)
        .rot_rad(ship.apos)
        .scale(ship.radius, ship.radius);
    piston_window::polygon(SHIP_COLOR, SHIP_POLYGON, transform, gl);
}

fn render_spring(context: Context, gl: &mut GlGraphics, body1: &Body, body2: &Body) {
    line(body1.pos, body2.pos, SPRING_COLOR, context, gl);
}
