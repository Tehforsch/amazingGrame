
extern crate rand;

use point::Point;

pub mod body;

const DT : f64 = 0.01;
const G : f64 = 100.0;

pub struct Simulation {
    pub bodies : Vec<body::Body>
}

impl Simulation {
    pub fn timestep(&mut self) {
        self.gravity();
        self.integrate();
    }
    pub fn integrate(&mut self) {
        for body in self.bodies.iter_mut() {
            body.timestep(DT);
        }
    }
    pub fn gravity(&mut self) {
        let mut slice = &mut self.bodies[..];
        let length = slice.len();
        for i in 1..length {
            let (mut first, second) = slice.split_at_mut(i);
            let first_length = first.len();
            let mut b1 = &mut first[first_length-1];
            for mut b2 in second {
                apply_gravity(b1, b2);
                handle_collisions(b1, b2);
            }
        }
    }
}

fn apply_gravity(body1 : &mut body::Body, body2 : &mut body::Body) {
    let distance = body1.pos - body2.pos;
    let length = distance.norm();
    let force = -G * body1.mass * body2.mass * distance / length.powi(2);
    body1.apply_force(force);
    body2.apply_force(-force);
}

fn handle_collisions(body1 : &mut body::Body, body2 : &mut body::Body) {
    let distance = body1.pos - body2.pos;
    let length = distance.norm();
    let collision_normal = distance / length;
    if body1.radius + body2.radius > length {
        let impulse = (body1.mass * body2.mass) / (body1.mass + body2.mass) * collision_normal * (body1.vel - body2.vel) - (body1.radius + body2.radius - length);
        body1.apply_impulse(-collision_normal * impulse);
        body2.apply_impulse(collision_normal * impulse);
    }
}

pub fn initialize_random_sim(num_bodies: i64) -> Simulation {
    let mut bodies : Vec<body::Body> = vec![];
    for _ in 0..num_bodies {
        let x = rand::random::<f64>() * 1600.0-400.0;
        let y = rand::random::<f64>() * 1600.0-400.0;
        let mass = rand::random::<f64>() * 3.0 + 0.3;
        bodies.push(body::get_body(Point{x: x, y: y}, mass));
    }
    let mut sim = Simulation {
        bodies: bodies
    };
    sim
}
