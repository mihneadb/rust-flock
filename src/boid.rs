use point::Point;

#[deriving(Show, Default)]
pub struct Boid {
    pub position: Point,
    pub velocity: Point,
    pub acceleration: Point
}

