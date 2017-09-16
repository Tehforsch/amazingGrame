use point::Point;

pub struct Body {
    pub pos: Point,
    pub vel: Point,
    pub acc: Point,
    pub mass: f64,
    pub radius: f64
}

impl Body {
    pub fn timestep(&mut self, dt : f64) {
        self.vel += self.acc * dt;
        self.pos += self.vel * dt;
        self.acc = Point { x: 0.0, y: 0.0 };
    }

    pub fn apply_force(&mut self, force : Point) {
        self.acc += force / self.mass;
    }

    pub fn apply_impulse(&mut self, impulse : Point) {
        self.vel += impulse / self.mass;
    }
}

pub fn get_body(pos: Point, mass: f64) -> Body {
    Body {
        pos: pos,
        vel: Point { x: 0.0, y: 0.0 },
        acc: Point { x: 0.0, y: 0.0 },
        mass: mass,
        radius: 20.0 * mass.sqrt()
    }
}
