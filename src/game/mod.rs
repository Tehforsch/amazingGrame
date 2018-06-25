extern crate rand;

pub mod input;
pub mod object;
pub mod spring;

use self::input::{InputController, Actions};
use self::object::{Object,ObjectType};
use self::spring::Spring;
use ::simulation;
use ::simulation::body::Body;
use ::point::Point;

const TURN_VEL : f64 = 5.0;
const MOVE_STRENGTH : f64 = 10.0;
const TURN_VEL_DECAY : f64 = 0.05;

const BULLET_VEL : f64 = 1000.0;
const BULLET_RADIUS : f64 = 10.0;

const SHIP_MASS : f64 = 1.0;
const SHIP_RADIUS : f64 = 25.0;

const STAR_MASS : f64 = 0.3;
const STAR_RADIUS : f64 = 25.0;

const MIN_MASS_BLACKHOLE: f64 = 70.0;
const MAX_MASS_BLACKHOLE: f64 = 100.0;

const SPRING_STRENGTH : f64 = 5.0;
const SPRING_REST_LENGTH : f64 = 50.0;

const NUM_SHIPS : usize = 1;
const NUM_STARS : usize = 10;
const NUM_BLACKHOLES : usize = 4;

pub struct Game {
    pub input_controller: InputController,
    pub objects: Vec<Object>,
    pub springs: Vec<Spring>,
    pub sim: simulation::Simulation,
    pub game_over: bool,
    pub score: i32
}

impl Game {
    pub fn new() -> Game{
        let mut bodies = get_ships(NUM_SHIPS);
        bodies.append(&mut get_stars(NUM_STARS));
        bodies.append(&mut get_black_holes(NUM_BLACKHOLES));
        bodies.push(get_mothership());
        let mut objects = vec![];
        for (i, b) in bodies.iter().enumerate() {
            if i < NUM_SHIPS {
                objects.push(Object::new(i, ObjectType::Ship));
            }
            else if i < NUM_SHIPS + NUM_STARS {
                objects.push(Object::new(i, ObjectType::Star));
            }
            else if i < NUM_SHIPS + NUM_STARS + NUM_BLACKHOLES {
                objects.push(Object::new(i, ObjectType::BlackHole));
            }
            else {
                objects.push(Object::new(i, ObjectType::Mothership));
            }
        }
        for i in 0..NUM_SHIPS {
            bodies[i].gravity_flag = true;
        }
        Game {
            input_controller: InputController::new(),
            objects: objects,
            sim: simulation::Simulation::new(bodies),
            springs: vec![],
            game_over: false,
            score: 0
        }
    }

    pub fn timestep(&mut self) {
        self.control();
        self.handle_springs();
        self.handle_bullets();
        self.handle_stars();
        self.remove_objects();
        self.sim.timestep();
    }

    pub fn remove_objects(&mut self) {
        for object in self.objects.iter_mut() {
            if object.should_be_removed {
                self.sim.get_body_mut(object.body).should_be_removed = true;
            }
        }
        self.objects.retain(|o| !o.should_be_removed);
        self.springs.retain(|s| !s.should_be_removed);
    }

    pub fn handle_springs(&mut self) {
        for spring in self.springs.iter_mut() {
            let body1 = self.sim.get_body(spring.body1);
            let body2 = self.sim.get_body(spring.body2);
            let distance = self.sim.get_body(spring.body1).pos - self.sim.get_body(spring.body2).pos;
            let length = distance.norm();
            let force = -SPRING_STRENGTH * (length - SPRING_REST_LENGTH) * distance.normalized();
            spring.force = force;
        }
        for spring in self.springs.iter() {
            self.sim.get_body_mut(spring.body1).apply_force(spring.force);
        }
        for spring in self.springs.iter() {
            self.sim.get_body_mut(spring.body2).apply_force(-spring.force);
        }
    }

    pub fn handle_stars(&mut self) {
        let mothership_id = self.get_mothership().body;
        for star in self.objects.iter_mut() {
            match star.type_ {
                ObjectType::Star => {
                    match self.sim.get_body(star.body).did_collide {
                        Some(body) => {
                            if body == mothership_id {
                                star.should_be_removed = true;
                                for spring in self.springs.iter_mut() {
                                    if spring.body1 == star.body || spring.body2 == star.body {
                                        spring.should_be_removed = true;
                                    }
                                    self.score += 1;
                                }
                            }
                        }
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    pub fn handle_bullets(&mut self) {
        for bullet in self.objects.iter_mut() {
            match bullet.type_ {
                ObjectType::Bullet(ship) => {
                    match self.sim.get_body(bullet.body).did_collide {
                        Some(body) => {
                            self.springs.push(Spring::new(body, ship));
                            bullet.should_be_removed = true;
                        }
                        _ => {}
                    }
                },
                _ => {}
            }
        }
    }

    pub fn get_mothership(&self) -> &Object {
        self.objects.iter().filter(|o| match o.type_ { ObjectType::Mothership => true, _ => false }).next().unwrap()
    }

    pub fn control(&mut self) {
        let actions = self.input_controller.actions();
        for ship in 0..NUM_SHIPS {
            Game::control_turning(&mut self.sim.get_body_mut(ship), actions);
            Game::control_moving(&mut self.sim.get_body_mut(ship), actions);
            self.control_shooting(ship, actions);
        }
    }

    fn control_turning(ship: &mut Body, actions: Actions) {
        if actions.rotate_left {
            ship.avel = -TURN_VEL;
        }
        else if actions.rotate_right {
            ship.avel = TURN_VEL;
        }
        else {
            ship.avel *= 1.0-TURN_VEL_DECAY;
        }
    }

    fn control_moving(ship: &mut Body, actions: Actions) {
        let direction = Point::from_angle(ship.apos);
        if actions.boost {
            ship.apply_impulse(direction * MOVE_STRENGTH);
        }
    }

    fn control_shooting(&mut self, ship: usize, actions: Actions) {
        if actions.shoot {
            for object in self.objects.iter() {
                match object.type_ {
                    ObjectType::Bullet(ship_) => {
                        if ship_ == ship {
                            return;
                        }
                    },
                    _ => {}
                }
            }
            let direction = Point::from_angle(self.sim.get_body(ship).apos);
            let spawn_pos = self.sim.get_body(ship).pos + direction * (self.sim.get_body(ship).radius + BULLET_RADIUS * 1.1);
            let mut bullet = Body::new(spawn_pos, 1.0, 10.0);
            bullet.vel = direction * BULLET_VEL;
            let index = self.sim.add_body(bullet);
            self.objects.push(Object::new(index, ObjectType::Bullet(ship)));
        }
    }

}

pub fn get_ships(num_ships: usize) -> Vec<Body> {
    let mut bodies : Vec<Body> = vec![];
    for i in 0..num_ships {
        let x = rand::random::<f64>() * 1000.0;
        let y = 0.0;
        bodies.push(
            Body::new(Point{x: x, y: y}, SHIP_MASS, SHIP_RADIUS),
        )
    }
    bodies
}

pub fn get_stars(num_bodies: usize) -> Vec<Body> {
    let mut bodies : Vec<Body> = vec![];
    for _ in 0..num_bodies {
        let x = rand::random::<f64>() * 1000.0+500.0;
        let y = rand::random::<f64>() * 1000.0+100.0;
        let mass = STAR_MASS;
        let radius = STAR_RADIUS;
        bodies.push(Body::new(Point{x: x, y: y}, mass, radius))
    }
    bodies
}

pub fn get_mothership() -> Body {
    let mut bodies : Vec<Body> = vec![];
    let x = 100.0;
    let y = 100.0;
    let mass = 1000.0;
    let radius = 50.0;
    Body::new(Point{x: x, y: y}, mass, radius)
}

pub fn get_black_holes(num_bodies: usize) -> Vec<Body> {
    let mut bodies : Vec<Body> = vec![];
    for _ in 0..num_bodies {
        let x = rand::random::<f64>() * 1000.0+500.0;
        let y = rand::random::<f64>() * 1000.0+100.0;
        let mass = rand::random::<f64>() * (MAX_MASS_BLACKHOLE-MIN_MASS_BLACKHOLE) + MIN_MASS_BLACKHOLE;
        let radius = 10.0 * mass.sqrt();
        bodies.push(Body::new(Point{x: x, y: y}, mass, radius))
    }
    bodies
}
