
#[deriving(Show)]
pub struct Point {
    pub x: int,
    pub y: int,
    pub z: int
}

impl Add<Point, Point> for Point {
    fn add(&self, other: &Point) -> Point {
        Point { x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z }
    }
}
