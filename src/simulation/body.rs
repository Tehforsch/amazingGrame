use point::Point;

#[derive(Clone, Debug)]
pub struct Body {
    pub pos: Point,
    pub vel: Point,
    pub acc: Point,
    pub apos: f64,
    pub avel: f64,
    pub aacc: f64,
    pub mass: f64,
    pub radius: f64,
    pub moment_of_inertia: f64,
    pub gravity_flag: i32,
    pub did_collide: Vec<usize>,
    pub id: usize,
    pub should_be_removed: bool
}

impl Body {
    pub fn integrate(&mut self, dt : f64) {
        self.vel += self.acc * dt;
        self.pos += self.vel * dt;
        self.acc = Point { x: 0.0, y: 0.0 };
        self.avel += self.aacc * dt;
        self.apos += self.avel * dt;
        self.aacc = 0.0;
    }

    pub fn apply_force(&mut self, force: Point) {
        self.acc += force / self.mass;
    }

    pub fn apply_torque(&mut self, torque: f64) {
        self.aacc += torque / self.moment_of_inertia;
    }

    pub fn apply_force_at(&mut self, force: Point, pos: Point) {
        self.acc += force / self.mass;
        self.aacc += (pos - self.pos).cross(force) / self.moment_of_inertia;
    }

    pub fn apply_impulse(&mut self, impulse: Point) {
        self.vel += impulse / self.mass;
    }

    pub fn apply_impulse_at(&mut self, impulse: Point, pos: Point) {
        self.vel += impulse / self.mass;
        self.avel += (pos - self.pos) * impulse / self.moment_of_inertia;
    }

    pub fn new(pos: Point, mass: f64, radius: f64) -> Body {
        let moment_of_inertia = mass * radius * radius * 0.5;
        Body {
            pos: pos,
            vel: Point { x: 0.0, y: 0.0 },
            acc: Point { x: 0.0, y: 0.0 },
            apos: 0.0,
            avel: 0.0,
            aacc: 0.0,
            mass: mass,
            radius: radius,
            moment_of_inertia: moment_of_inertia,
            gravity_flag: 0,
            did_collide: vec![],
            id: 0,
            should_be_removed: false
        }
    }
}
