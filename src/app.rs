use sdl2_window::Sdl2Window;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL_3_2;
use std::cell::RefCell;
use piston::{
    RenderArgs,
    UpdateArgs
};
use graphics::{
    Context,
    AddRectangle,
    AddColor,
    Draw,
    RelativeTransform2d
};
use event::{
    Events,
    Window,
    RenderEvent,
    UpdateEvent
};
use boid::Boid;

pub struct App {
    pub gl: Gl,
    pub boids: Vec<Boid>
}

impl App {
    pub fn render<W: Window>(&mut self, _: &mut W, args: &RenderArgs) {
        let context = &Context::abs(args.width as f64, args.height as f64);
        context.rgba(0.0, 1.0, 0.0, 1.0).draw(&mut self.gl);

        //context
            //.trans((args.width / 2) as f64, (args.height / 2) as f64)
            //.rect(0.0, 0.0, 50.0, 50.0)
            //.rgba(1.0, 0.0, 0.0, 1.0)
            //.trans(-25.0, -25.0)
            //.draw(&mut self.gl);

        self.boids[0].render(context, &mut self.gl, args);
    }

    pub fn update<W: Window>(&mut self, _: &mut W, args: &UpdateArgs) {
    }
}
