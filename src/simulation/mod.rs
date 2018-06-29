pub mod body;

use ::point::Point;
use self::body::Body;
use ::game::{ARENA_HEIGHT,ARENA_WIDTH,G,DISTANCE_SCALING,WALL_RESTITUTION,FRICTION};

const DT : f64 = 0.01;
// const G : f64 = 2000000.0;
const ANGULAR_FRICTION : f64 = 0.0;
const CLAMP_IMPULSES : bool = false;
const BAUMGARTE_CORRECTION_STRENGTH: f64 = 10.0;

pub struct Wall {
    pos: Point,
    normal: Point
}

pub struct Simulation {
    pub bodies: Vec<Body>,
    pub next_id: usize,
    pub walls: Vec<Wall>,
    pub time: f64
}

impl Simulation {
    pub fn timestep(&mut self) {
        self.gravity();
        self.collisions();
        self.wall_collisions();
        self.integrate();
        self.friction();
        self.remove_bodies();
        self.time += DT;
    }

    pub fn remove_bodies(&mut self) {
        self.bodies.retain(|b| !b.should_be_removed);
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
                if b1.gravity_flag == 2 && b2.gravity_flag >= 1 || b2.gravity_flag == 2 && b1.gravity_flag >= 1{
                    apply_gravity(b1, b2);
                }
            }
        }
    }

    pub fn collisions(&mut self) {
        for body in self.bodies.iter_mut() {
            body.did_collide = vec![];
        }
        let mut slice = &mut self.bodies[..];
        let length = slice.len();
        for i in 1..length {
            let (mut first, second) = slice.split_at_mut(i);
            let first_length = first.len();
            let mut b1 = &mut first[first_length-1];
            for mut b2 in second {
                handle_collisions(b1, b2);
            }
        }
    }

    fn wall_collisions(&mut self) {
        for body in self.bodies.iter_mut() {
            for wall in self.walls.iter() {
                let projection_body = body.pos * wall.normal;
                let projection_wall = wall.pos * wall.normal;
                let depth = projection_wall - projection_body + body.radius;
                if depth > 0.0 {
                    let normal_impulse = body.mass * body.vel * wall.normal;
                    body.apply_impulse(wall.normal * (normal_impulse * (-1.0 - WALL_RESTITUTION) + depth * BAUMGARTE_CORRECTION_STRENGTH));
                }
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

    pub fn get_body(&self, id: usize) -> &Body {
        self.bodies.iter().filter(|b| b.id == id).next().unwrap()
    }

    pub fn get_body_mut(&mut self, id: usize) -> &mut Body {
        self.bodies.iter_mut().filter(|b| b.id == id).next().unwrap()
    }

    pub fn add_body(&mut self, mut body: Body) -> usize {
        body.id = self.next_id;
        self.next_id = self.next_id + 1;
        self.bodies.push(body);
        self.next_id - 1
    }

    pub fn new(mut bodies: Vec<Body>) -> Simulation {
        for (i, body) in bodies.iter_mut().enumerate() {
            body.id = i;
        }
        let next_id = bodies.len();
        let walls = vec![
            Wall{pos: Point{x:0.0, y:0.0}, normal: Point{x:1.0, y:0.0}},
            Wall{pos: Point{x:ARENA_WIDTH, y:0.0}, normal: Point{x:-1.0, y:0.0}},
            Wall{pos: Point{x:0.0, y:0.0}, normal: Point{x:0.0, y:1.0}},
            Wall{pos: Point{x:0.0, y:ARENA_HEIGHT}, normal: Point{x:0.0, y:-1.0}},
        ];
        Simulation{
            bodies: bodies,
            next_id: next_id,
            walls: walls,
            time: 0.0
        }
    }
}

fn apply_gravity(body1 : &mut Body, body2 : &mut Body) {
    let distance = body1.pos - body2.pos;
    let length = distance.norm();
    let force = -G * body1.mass * body2.mass * distance / length.powi(DISTANCE_SCALING);
    body1.apply_force(force);
    body2.apply_force(-force);
}


fn handle_collisions(body1 : &mut Body, body2 : &mut Body) {
    let distance = body1.pos - body2.pos;
    let length = distance.norm();
    let collision_normal = distance / length;
    if body1.radius + body2.radius > length {
        let impulse = (body1.mass * body2.mass) / (body1.mass + body2.mass) * collision_normal * (body1.vel - body2.vel) - BAUMGARTE_CORRECTION_STRENGTH * (body1.radius + body2.radius - length);
        if !CLAMP_IMPULSES || impulse > 0.0 {
            body1.apply_impulse(-collision_normal * impulse);
            body2.apply_impulse(collision_normal * impulse);
        }
        body1.did_collide.push(body2.id);
        body2.did_collide.push(body1.id);
    }
}
