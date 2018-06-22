use simulation::body::Body;

#[derive(Clone, Copy)]
pub struct Object {
    pub body: usize,
    pub type_: ObjectType
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ObjectType {
    Star, Bullet, Ship
}
