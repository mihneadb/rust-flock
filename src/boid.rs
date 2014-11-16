extern crate graphics;

use point::Point;
use graphics::{
    Context,
    AddColor,
    Draw,
    RelativeTransform2d,
    AddEllipse
};
use opengl_graphics::Gl;
use piston::{
    RenderArgs,
    UpdateArgs
};


const RADIUS: f64 = 5.0;
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
            .trans(MARGIN + self.position.x, MARGIN + self.position.y)
            .circle(0.0, 0.0, RADIUS)
            .rgba(1.0, 1.0, 1.0, 1.0)
            .draw(gl);
    }

    pub fn update_position(&mut self, args: &UpdateArgs) {
        self.position = self.position + self.velocity * args.dt;
    }
}
