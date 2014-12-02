extern crate graphics;

use point::Point;
use graphics::{
    Context,
    AddColor,
    Draw,
    RelativeTransform,
    AddEllipse
};
use opengl_graphics::Gl;
use piston::{
    RenderArgs,
    UpdateArgs
};
use std::default::Default;


const RADIUS: f64 = 3.0;
pub const MARGIN: f64 = 20.0;
const NEIGHBORHOOD: f64 = 100.0;

#[deriving(Show, Default)]
pub struct Boid {
    pub position: Point,
    pub velocity: Point,
}

impl Boid {
    pub fn render(&self, context: &Context, gl: &mut Gl, _args: &RenderArgs) {
        context
            .trans(self.position.x, self.position.y)
            .circle(0.0, 0.0, RADIUS)
            .rgba(1.0, 1.0, 1.0, 1.0)
            .draw(gl);
    }

    pub fn update_position(&mut self, args: &UpdateArgs, width: u32,
                           height: u32, boids: &[Boid], idx: uint) {
        let width = width as f64;
        let height = height as f64;

        // Perform wrapping.
        if self.position.x <= 0.0 {
            self.position.x = width;
        } else if self.position.x >= width {
            self.position.x = 0.0;
        }

        if self.position.y <= 0.0 {
            self.position.y = height;
        } else if self.position.y >= height {
            self.position.y = 0.0
        }

        self.velocity = self.velocity + self.compute_alignment(boids, idx);
        self.velocity = self.velocity + self.compute_cohesion(boids, idx);
        self.velocity = self.velocity + self.compute_separation(boids, idx);

        self.velocity = self.velocity.normalize() * 100.0;

        self.position = self.position + self.velocity * args.dt;
    }

    pub fn distance_to(&self, other: &Boid) -> f64 {
        self.position.distance_to(&other.position)
    }

    fn compute_alignment(&self, boids: &[Boid], idx: uint) -> Point {
        let mut alignment = Point { ..Default::default() };
        let mut neighbours = 0.0;

        for i in range(0u, boids.len()) {
            let boid = boids[i];
            if i != idx && self.distance_to(&boid) <= NEIGHBORHOOD {
                alignment = alignment + boid.velocity;
                neighbours += 1.0;
            }
        }

        if neighbours == 0.0 {
            return alignment;
        }

        alignment.x = alignment.x / neighbours;
        alignment.y = alignment.y / neighbours;
        alignment.z = alignment.z / neighbours;

        // Might want to normalize it.
        alignment
    }

    fn compute_cohesion(&self, boids: &[Boid], idx: uint) -> Point {
        let mut center = Point { ..Default::default() };
        let mut neighbours = 0.0;

        for i in range(0u, boids.len()) {
            let boid = boids[i];
            if i != idx && self.distance_to(&boid) <= NEIGHBORHOOD {
                center = center + boid.position;
                neighbours += 1.0;
            }
        }

        if neighbours == 0.0 {
            return center;
        }

        center.x = center.x / neighbours;
        center.y = center.y / neighbours;
        center.z = center.z / neighbours;

        // Need distance towards center.
        let cohesion = Point {
            x: center.x - self.position.x,
            y: center.y - self.position.y,
            z: center.z - self.position.z
        };

        // might want to normalize it
        cohesion
    }

    fn compute_separation(&self, boids: &[Boid], idx: uint) -> Point {
        let mut separation = Point { ..Default::default() };
        let mut neighbours = 0.0;

        for i in range(0u, boids.len()) {
            let boid = boids[i];
            if i != idx && self.distance_to(&boid) <= NEIGHBORHOOD {
                separation = separation + (boid.position - self.position) +
                    Point { x: RADIUS, y: RADIUS, z: 0.0 };
                neighbours += 1.0;
            }
        }

        if neighbours == 0.0 {
            return separation;
        }

        separation.x = separation.x / neighbours;
        separation.y = separation.y / neighbours;
        separation.z = separation.z / neighbours;

        // Separation means keeping away from others.
        separation = separation * -1.0;

        // Might want to normalize it.
        separation
    }
}
