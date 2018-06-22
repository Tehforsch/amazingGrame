extern crate rand;

pub mod input;
pub mod object;

use self::input::{InputController, Actions};
use self::object::{Object,ObjectType};
use ::simulation;
use ::simulation::body::Body;
use ::simulation::body;
use ::point::Point;

const TURN_VEL : f64 = 5.0;
const MOVE_STRENGTH : f64 = 10.0;
const TURN_VEL_DECAY : f64 = 0.05;
const SHOOT_VEL : f64 = 1000.0;
const BULLET_RADIUS : f64 = 10.0;

const NUM_SHIPS : usize = 2;

pub struct Game {
    pub input_controller: InputController,
    pub objects: Vec<Object>,
    pub sim: simulation::Simulation
}

impl Game {
    pub fn new() -> Game{
        let bodies = get_random_bodies(20);
        let mut objects = vec![];
        for (i, _) in bodies[NUM_SHIPS..].iter().enumerate() {
            objects.push(Object{body: i+NUM_SHIPS, type_: ObjectType::Star});
        }
        objects.push(Object{body: 0, type_: ObjectType::Ship});
        Game {
            input_controller: InputController::new(),
            objects: objects,
            sim: simulation::Simulation{bodies}
        }
    }

    pub fn timestep(&mut self) {
        self.control();
        self.sim.timestep();
    }

    pub fn control(&mut self) {
        let actions = self.input_controller.actions();
        let ships = self.objects.iter().filter(|o| o.type_ == ObjectType::Ship);
        for ship in ships {
            Game::control_turning(&mut self.sim.bodies[ship.body], actions);
            Game::control_shooting(&mut self.sim.bodies[ship.body], actions);
            Game::control_moving(&mut self.sim.bodies[ship.body], actions);
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

    fn control_shooting(ship: &mut Body, actions: Actions) {
        if actions.shoot {
            let direction = Point::from_angle(ship.apos);
            let spawn_pos = ship.pos + direction * (ship.radius + BULLET_RADIUS * 1.1);
            let mut bullet = Body::new(spawn_pos, 0.5, BULLET_RADIUS);
            bullet.vel = direction * SHOOT_VEL;
            // self.sim.bodies.push(bullet);
        }
    }
    
}

pub fn get_random_bodies(num_bodies: i64) -> Vec<Body> {
    let mut bodies : Vec<Body> = vec![];
    for _ in 0..num_bodies {
        let x = rand::random::<f64>() * 1000.0+500.0;
        let y = rand::random::<f64>() * 1000.0+100.0;
        let mass = rand::random::<f64>() * 3.0 + 0.3;
        bodies.push(
            body::get_body(Point{x: x, y: y}, mass),
        )
    }
    bodies
}
