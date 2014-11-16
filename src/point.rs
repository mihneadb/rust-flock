
#[deriving(Show, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Add<Point, Point> for Point {
    fn add(&self, other: &Point) -> Point {
        Point { x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z }
    }
}

impl Sub<Point, Point> for Point {
    fn sub(&self, other: &Point) -> Point {
        Point { x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z }
    }
}
