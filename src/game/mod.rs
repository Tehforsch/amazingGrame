extern crate rand;

pub mod input;
pub mod object;
pub mod spring;

use self::input::{ Actions};
use self::object::{Object,ObjectType};
use self::spring::Spring;
use ::simulation;
use ::simulation::body::Body;
use ::point::Point;
use ::simulation::Wall;

const TURN_VEL : f64 = 5.0;
const MOVE_STRENGTH : f64 = 10.0;
const TURN_VEL_DECAY : f64 = 0.15;

const BULLET_VEL : f64 = 2000.0;
const BULLET_RADIUS : f64 = 4.0;
const BULLET_MASS: f64 = 0.1;
const BULLET_LIFETIME: f64 = 1.0;

const SHIP_MASS : f64 = 0.3;
const SHIP_RADIUS : f64 = 35.0;

const STAR_MASS : f64 = 0.3;
const STAR_RADIUS : f64 = 25.0;

const MIN_MASS_BLACKHOLE: f64 = 70.0;
const MAX_MASS_BLACKHOLE: f64 = 100.0;

const SPRING_STRENGTH : f64 = 5.0;
const SPRING_REST_LENGTH : f64 = 50.0;

const NUM_SHIPS : usize = 2;
const NUM_STARS : usize = 40;
const NUM_BLACKHOLES : usize = 4;

const STAR_SCORE: i32 = 100;
const BLACKHOLE_SCORE: i32 = -100;

const TOP_MARGIN: f64 = 200.0;
const LEFT_MARGIN: f64 = 200.0;

pub const DISTANCE_SCALING: i32 = 2;
pub const WALL_RESTITUTION: f64 = 0.5;
pub const G : f64 = 1000.0;
pub const FRICTION : f64 = 0.2;

pub struct Game {
    pub objects: Vec<Object>,
    pub springs: Vec<Spring>,
    pub sim: simulation::Simulation,
    pub game_over: bool,
    pub score: Vec<i32>,
    pub should_respawn: Vec<bool>,
    pub arena_size: Point
}

impl Game {
    pub fn new(arena_size: Point) -> Game{
        let mut bodies = get_ships(NUM_SHIPS, arena_size);
        bodies.append(&mut get_stars(NUM_STARS, arena_size));
        bodies.append(&mut get_black_holes(NUM_BLACKHOLES, arena_size));
        bodies.push(get_mothership());
        let mut objects = vec![];
        for (i, b) in bodies.iter().enumerate() {
            if i < NUM_SHIPS {
                objects.push(Object::new(i, ObjectType::Ship(i)));
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
        let walls = vec![
            Wall{pos: Point{x:0.0, y:0.0}, normal: Point{x:1.0, y:0.0}},
            Wall{pos: Point{x:arena_size.x, y:0.0}, normal: Point{x:-1.0, y:0.0}},
            Wall{pos: Point{x:0.0, y:0.0}, normal: Point{x:0.0, y:1.0}},
            Wall{pos: Point{x:0.0, y:arena_size.y}, normal: Point{x:0.0, y:-1.0}},
        ];
        let mut sim = simulation::Simulation::new(bodies, walls);
        Game {
            objects: objects,
            sim: sim,
            springs: vec![],
            game_over: false,
            score: vec![0, 0],
            should_respawn: vec![false, false],
            arena_size: arena_size
        }
    }

    pub fn timestep(&mut self) {
        self.handle_springs();
        self.handle_bullets();
        self.handle_stars();
        self.handle_blackholes();
        self.handle_respawning();
        self.remove_objects();
        self.sim.timestep();
    }

    pub fn remove_objects(&mut self) {
        for object in self.objects.iter_mut() {
            if object.should_be_removed {
                self.sim.get_body_mut(object.body).should_be_removed = true;
                for spring in self.springs.iter_mut() {
                    if spring.body1 == object.body || spring.body2 == object.body {
                        spring.should_be_removed = true;
                    }
                }
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
        let ship_bodies = vec![self.get_ship(0).body, self.get_ship(1).body];
        for star in self.objects.iter_mut() {
            match star.type_ {
                ObjectType::Star => {
                    for &body in self.sim.get_body(star.body).did_collide.iter() {
                        if body == mothership_id {
                            star.should_be_removed = true;
                            for spring in self.springs.iter() {
                                if spring.body1 == star.body {
                                    if ship_bodies[0] == spring.body2 { 
                                        self.score[0] += STAR_SCORE
                                    }
                                    if ship_bodies[1] == spring.body2 { 
                                        self.score[1] += STAR_SCORE
                                    }
                                }
                            }
                        }
                    }
                },
                _ => {}
            }
        }
    }

    pub fn handle_blackholes(&mut self) {
        let ship_bodies = vec![self.get_ship(0).body, self.get_ship(1).body];
        for blackhole in self.objects.iter_mut() {
            match blackhole.type_ {
                ObjectType::BlackHole => {
                    for body in self.sim.get_body(blackhole.body).did_collide.iter() {
                        let ship = ship_bodies.iter().position(|b| b == body);
                        match ship {
                            Some(ship_num) => {
                                self.score[ship_num] += BLACKHOLE_SCORE;
                                self.should_respawn[ship_num] = true;
                            }
                            _ => {}
                        }
                    }
                },
                _ => {}
            }
        }
    }

    pub fn handle_bullets(&mut self) {
        let ship_bodies = vec![self.get_ship(0).body, self.get_ship(1).body];
        let bullet_bodies: Vec<usize> = self.objects.iter().filter(|o| match o.type_ { ObjectType::Bullet(_, _) => true, _ => false} ).map(|o| o.body).collect();
        for bullet in self.objects.iter_mut() {
            match bullet.type_ {
                ObjectType::Bullet(ship_num, time) => {
                    for &body in self.sim.get_body(bullet.body).did_collide.iter() {
                        bullet.should_be_removed = true;
                        if bullet_bodies.contains(&body) {
                            continue;
                        }
                        let ship_body = ship_bodies[ship_num];
                        if body != ship_body {
                            let mut add_spring = true;
                            for spring in self.springs.iter() {
                                if spring.body1 == body && spring.body2 == ship_body {
                                    add_spring = false;
                                }
                            }
                            if add_spring {
                                self.springs.push(Spring::new(body, ship_body));
                            }
                        }
                    }
                    if self.sim.time - time > BULLET_LIFETIME {
                        bullet.should_be_removed = true;
                    }
                },
                _ => {}
            }
        }
    }

    pub fn handle_respawning(&mut self) {
        for ship_num in 0..NUM_SHIPS {
            if self.should_respawn[ship_num] {
                self.respawn_ship(ship_num);
            }
            self.should_respawn[ship_num] = false;
        }

    }

    pub fn get_mothership(&self) -> &Object {
        self.objects.iter().filter(|o| match o.type_ { ObjectType::Mothership => true, _ => false }).next().unwrap()
    }

    pub fn control(&mut self, actions: Vec<Actions>) {
        for ship_num in 0..NUM_SHIPS {
            let ship = self.get_ship(ship_num);
            Game::control_turning(&mut self.sim.get_body_mut(ship.body), actions[ship_num]);
            Game::control_moving(&mut self.sim.get_body_mut(ship.body), actions[ship_num]);
            self.control_shooting(ship_num, actions[ship_num]);
            self.control_respawning(ship_num, actions[ship_num]);
        }
    }

    pub fn get_ship(&self, ship_num: usize) -> Object {
        let index = self.get_ship_index(ship_num);
        self.objects[index]
    }

    pub fn get_ship_index(&self, ship_num: usize) -> usize {
        let index = self.objects.iter().enumerate().filter(|&(i, o)| o.type_ == ObjectType::Ship(ship_num)).next().unwrap().0;
        index
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

    fn control_shooting(&mut self, ship_num: usize, actions: Actions) {
        if actions.shoot {
            for object in self.objects.iter() {
                match object.type_ {
                    ObjectType::Bullet(ship_num_, _) => {
                        if ship_num == ship_num_ {
                            return;
                        }
                    },
                    _ => {}
                }
            }
            let ship = self.get_ship(ship_num);
            let direction = Point::from_angle(self.sim.get_body(ship.body).apos);
            let spawn_pos = self.sim.get_body(ship.body).pos + direction * (self.sim.get_body(ship.body).radius + BULLET_RADIUS * 1.1);
            let mut bullet = Body::new(spawn_pos, BULLET_MASS, BULLET_RADIUS);
            bullet.vel = direction * BULLET_VEL;
            self.sim.get_body_mut(ship.body).apply_impulse(-bullet.vel * BULLET_MASS);
            let index = self.sim.add_body(bullet);
            self.objects.push(Object::new(index, ObjectType::Bullet(ship_num, self.sim.time)));
        }
    }

    pub fn control_respawning(&mut self, ship_num: usize, actions: Actions) {
        if actions.respawn {
            self.should_respawn[ship_num] = true;
        }
    }

    pub fn respawn_ship(&mut self, ship_number: usize) {
        let mut new_ship = get_ship_body(self.arena_size);
        let index = self.get_ship_index(ship_number);
        self.sim.get_body_mut(self.objects[index].body).should_be_removed = true;
        self.objects[index].should_be_removed = true;
        let index = self.sim.add_body(new_ship);
        self.objects.push(Object::new(index, ObjectType::Ship(ship_number)));
        for bullet in self.objects.iter_mut() {
            match bullet.type_ {
                ObjectType::Bullet(ship_number_, _) => {
                    if ship_number_ == ship_number {
                        bullet.should_be_removed = true;
                    }
                }
                _ => {}
            }
        }
    }

}

pub fn get_ship_body(arena_size: Point) -> Body {
    let x = rand::random::<f64>() * (arena_size.x-LEFT_MARGIN) + LEFT_MARGIN;
    let y = 50.0;
    let mut b = Body::new(Point{x: x, y: y}, SHIP_MASS, SHIP_RADIUS);
    b.gravity_flag = 2;
    b
}

pub fn get_ships(num_ships: usize, arena_size: Point) -> Vec<Body> {
    let mut bodies : Vec<Body> = vec![];
    for i in 0..num_ships {
        bodies.push(get_ship_body(arena_size))
    }
    bodies
}

pub fn get_stars(num_bodies: usize, arena_size: Point) -> Vec<Body> {
    let mut bodies : Vec<Body> = vec![];
    for _ in 0..num_bodies {
        let x = rand::random::<f64>() * (arena_size.x-LEFT_MARGIN) + LEFT_MARGIN;
        let y = rand::random::<f64>() * (arena_size.y-TOP_MARGIN) + TOP_MARGIN;
        let mass = STAR_MASS;
        let radius = STAR_RADIUS;
        bodies.push(Body::new(Point{x: x, y: y}, mass, radius))
    }
    bodies
}

pub fn get_mothership() -> Body {
    let mut bodies : Vec<Body> = vec![];
    let x = 150.0;
    let y = 150.0;
    let mass = 1000.0;
    let radius = 50.0;
    Body::new(Point{x: x, y: y}, mass, radius)
}

pub fn get_black_holes(num_bodies: usize, arena_size: Point) -> Vec<Body> {
    let mut bodies : Vec<Body> = vec![];
    for _ in 0..num_bodies {
        let x = rand::random::<f64>() * (arena_size.x-LEFT_MARGIN) + LEFT_MARGIN;
        let y = rand::random::<f64>() * (arena_size.y-TOP_MARGIN) + TOP_MARGIN;
        let mass = rand::random::<f64>() * (MAX_MASS_BLACKHOLE-MIN_MASS_BLACKHOLE) + MIN_MASS_BLACKHOLE;
        let radius = 10.0 * mass.sqrt();
        let mut b = Body::new(Point{x: x, y: y}, mass, radius);
        b.gravity_flag = 1;
        bodies.push(b)
    }
    bodies
}
