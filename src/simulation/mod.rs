
pub mod body;

use point::Point;
use self::body::Body;
use game::object::Object;

const DT : f64 = 0.01;
const G : f64 = 200.0;
const FRICTION : f64 = 0.0;
const ANGULAR_FRICTION : f64 = 0.0;


pub struct Simulation {
    pub bodies: Vec<Body>
}

impl Simulation {
    pub fn timestep(&mut self) {
        self.gravity();
        self.integrate();
        self.friction();
    }

    pub fn integrate(&mut self) {
        for body in self.bodies.iter_mut() {
            body.integrate(DT);
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

    pub fn friction(&mut self) {
        for mut b in self.bodies.iter_mut() {
            let friction = b.vel * -FRICTION;
            let afriction = b.avel * -ANGULAR_FRICTION;
            b.apply_force(friction);
            b.apply_torque(afriction);
        }
    }
}

fn apply_gravity(body1 : &mut Body, body2 : &mut Body) {
    let distance = body1.pos - body2.pos;
    let length = distance.norm();
    let force = -G * body1.mass * body2.mass * distance / length.powi(2);
    body1.apply_force(force);
    body2.apply_force(-force);
}

fn handle_collisions(body1 : &mut Body, body2 : &mut Body) {
    let distance = body1.pos - body2.pos;
    let length = distance.norm();
    let collision_normal = distance / length;
    if body1.radius + body2.radius > length {
        let impulse = (body1.mass * body2.mass) / (body1.mass + body2.mass) * collision_normal * (body1.vel - body2.vel) - (body1.radius + body2.radius - length);
        body1.apply_impulse(-collision_normal * impulse);
        body2.apply_impulse(collision_normal * impulse);
    }
}
