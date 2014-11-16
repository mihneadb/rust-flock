use opengl_graphics::Gl;
use piston::{
    RenderArgs,
    UpdateArgs
};
use graphics::{
    Context,
};
use event::{
    Window,
    RenderEvent,
};
use boid::Boid;

pub struct App {
    pub gl: Gl,
    pub boids: Vec<Boid>
}

impl App {
    pub fn render<W: Window>(&mut self, _: &mut W, args: &RenderArgs) {
        let context = &Context::abs(args.width as f64, args.height as f64);

        for boid in self.boids.iter() {
            boid.render(context, &mut self.gl, args);
        }
    }

    pub fn update<W: Window>(&mut self, _: &mut W, _args: &UpdateArgs) {
    }
}
