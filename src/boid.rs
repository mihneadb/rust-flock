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


const RADIUS: f64 = 3.0;
pub const MARGIN: f64 = 20.0;

#[deriving(Show, Default)]
pub struct Boid {
    pub position: Point,
    pub velocity: Point,
    pub acceleration: Point
}

impl Boid {
    pub fn render(&self, context: &Context, gl: &mut Gl, _args: &RenderArgs) {
        context
            .trans(self.position.x, self.position.y)
            .circle(0.0, 0.0, RADIUS)
            .rgba(1.0, 1.0, 1.0, 1.0)
            .draw(gl);
    }

    pub fn update_position(&mut self, args: &UpdateArgs, width: u32, height: u32) {
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

        self.position = self.position + self.velocity * args.dt;
    }
}
