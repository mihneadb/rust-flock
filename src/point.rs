use std::num::Float;

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

impl Mul<f64, Point> for Point {
    fn mul(&self, other: &f64) -> Point {
        Point { x: self.x * *other,
                y: self.y * *other,
                z: self.z * *other }
    }
}

impl Point {
    fn distance_to(&self, other: &Point) -> f64 {
        (self.x - other.x).powi(2) +
            (self.y - other.y).powi(2) +
            (self.z - other.z).powi(2)
    }
}
