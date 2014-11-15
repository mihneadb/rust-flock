use point::Point;
use boid::Boid;
use std::default::Default;

mod point;
mod boid;

fn make_boids() -> Vec<Boid> {
    let mut v = Vec::new();
    for x in range(0i, 20) {
        for y in range(0i, 20) {
            let b = Boid {
                position: Point { x: 5 * x, y: 5 * y, z: 0i },
                ..Default::default()
            };
            v.push(b);
        }
    }
    v
}


fn main() {
    let boids = make_boids();
    println!("{}", boids);
}
