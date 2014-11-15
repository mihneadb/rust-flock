use point::Point;
mod point;

#[deriving(Show)]
pub struct Boid {
    pub position: Point,
    pub velocity: Point,
    pub acceleration: Point
}

