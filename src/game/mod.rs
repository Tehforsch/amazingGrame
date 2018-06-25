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

const SPRING_STRENGTH : f64 = 5.0;
const SPRING_REST_LENGTH : f64 = 50.0;

const NUM_SHIPS : usize = 1;
const NUM_STARS : usize = 10;

pub struct Game {
    pub input_controller: InputController,
    pub objects: Vec<Object>,
    pub springs: Vec<Spring>,
    pub sim: simulation::Simulation
}

impl Game {
    pub fn new() -> Game{
        let mut bodies = get_ships(NUM_SHIPS);
        bodies.append(&mut get_random_bodies(NUM_STARS));
        let mut objects = vec![];
        for (i, b) in bodies.iter().enumerate() {
            if i < NUM_SHIPS {
                objects.push(Object::new(i, ObjectType::Ship));
            }
            else {
                objects.push(Object::new(i, ObjectType::Star));
            }
        }
        for i in 0..NUM_SHIPS {
            bodies[i].gravity_flag = true;
        }
        Game {
            input_controller: InputController::new(),
            objects: objects,
            sim: simulation::Simulation::new(bodies),
            springs: vec![]
        }
    }

    pub fn rem_object(&mut self, object: &mut Object) {
        object.should_be_removed = true;
        self.sim.get_body_mut(object.body).should_be_removed = true;
    }

    pub fn timestep(&mut self) {
        self.control();
        self.sim.timestep();
        self.handle_bullets();
        self.handle_springs();
        self.remove_objects();
    }

    pub fn remove_objects(&mut self) {
        self.objects.retain(|o| !o.should_be_removed);
    }

    pub fn handle_springs(&mut self) {
        for spring in self.springs.iter_mut() {
            let body1 = self.sim.get_body(spring.body1);
            let body2 = self.sim.get_body(spring.body2);
            let distance = self.sim.get_body(spring.body1).pos - self.sim.get_body(spring.body2).pos;
            let length = distance.norm();
            let force = -SPRING_STRENGTH * (length - SPRING_REST_LENGTH) * distance.normalized();
            spring.force = force;
            // self.sim.get_body_mut(spring.body1).apply_force(force);
            // self.sim.get_body_mut(spring.body2).apply_force(-force);
        }
        for spring in self.springs.iter() {
            self.sim.get_body_mut(spring.body1).apply_force(spring.force);
        }
        for spring in self.springs.iter() {
            self.sim.get_body_mut(spring.body2).apply_force(-spring.force);
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
                            self.sim.get_body_mut(bullet.body).should_be_removed = true;
                        }
                        _ => {}
                    }
                },
                _ => {}
            }
        }
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

pub fn get_random_bodies(num_bodies: usize) -> Vec<Body> {
    let mut bodies : Vec<Body> = vec![];
    for _ in 0..num_bodies {
        let x = rand::random::<f64>() * 1000.0+500.0;
        let y = rand::random::<f64>() * 1000.0+100.0;
        let mass = rand::random::<f64>() * 15.0 + 30.0;
        let radius = 10.0 * mass.sqrt();
        bodies.push(Body::new(Point{x: x, y: y}, mass, radius))
    }
    bodies
}
