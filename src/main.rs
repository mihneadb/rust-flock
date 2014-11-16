extern crate graphics;
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate event;

use point::Point;
use boid::Boid;
use app::App;
use std::default::Default;
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

mod point;
mod boid;
mod app;


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
    let window = Sdl2Window::new(
        OpenGL_3_2,
        piston::WindowSettings::default()
    );

    let mut app = App { gl: Gl::new(OpenGL_3_2), boids: make_boids() };

    let window = RefCell::new(window);
    for e in Events::new(&window) {
        e.render(|r| app.render(window.borrow_mut().deref_mut(), r));
        e.update(|u| app.update(window.borrow_mut().deref_mut(), u));
    }
}
