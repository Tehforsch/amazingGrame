use ::point::Point;

#[derive(Debug)]
pub struct Spring {
    pub body1: usize,
    pub body2: usize,
    pub force: Point,
    pub should_be_removed: bool
}

impl Spring {
    pub fn new(body1: usize, body2: usize) -> Spring {
        Spring{body1: body1, body2: body2, force: Point{x:0.0, y:0.0}, should_be_removed: false}
    }
}
