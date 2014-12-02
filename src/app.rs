use opengl_graphics::Gl;
use piston::{
    RenderArgs,
    UpdateArgs
};
use graphics::{
    Context,
    AddColor,
    Draw,
};
use event::{
    Window,
    RenderEvent,
};
use boid::Boid;

pub struct App {
    pub gl: Gl,
    pub boids: Vec<Boid>,
    pub width: u32,
    pub height: u32
}

impl App {
    pub fn render<W: Window>(&mut self, _: &mut W, args: &RenderArgs) {
        let context = &Context::abs(args.width as f64, args.height as f64);
        context
            .rgba(0.0, 0.0, 0.0, 1.0)
            .draw(&mut self.gl);

        for boid in self.boids.iter() {
            boid.render(context, &mut self.gl, args);
        }
    }

    pub fn update<W: Window>(&mut self, _: &mut W, args: &UpdateArgs) {
        let boids = self.boids.clone();
        for i in range(0u, self.boids.len()) {
            let mut boid = &mut self.boids[i];
            boid.update_position(args, self.width, self.height, boids, i);
        }
    }
}
