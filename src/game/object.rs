#[derive(Clone, Copy)]
pub struct Object {
    pub body: usize,
    pub type_: ObjectType,
    pub should_be_removed: bool
}

impl Object {
    pub fn new(body: usize, type_: ObjectType) -> Object {
        Object{body: body, type_: type_, should_be_removed: false}
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ObjectType {
    Star, Bullet(usize, f64), Ship, BlackHole, Mothership
}
