extern crate graphics;

use point::Point;
use app::App;
use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
    RelativeTransform2d
};
use opengl_graphics::Gl;
use piston::{
    RenderArgs,
    UpdateArgs
};

#[deriving(Show, Default)]
pub struct Boid {
    pub position: Point,
    pub velocity: Point,
    pub acceleration: Point
}

impl Boid {
    pub fn render(&self, context: &Context, gl: &mut Gl, args: &RenderArgs) {
        context
            .trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rect(0.0, 0.0, 50.0, 50.0)
            .rgba(1.0, 0.0, 0.0, 1.0)
            .trans(-25.0, -25.0)
            .draw(gl);
    }
}
