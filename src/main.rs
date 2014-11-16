extern crate graphics;
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate event;

use point::Point;
use boid::{
    Boid,
    MARGIN
};
use app::App;
use std::default::Default;
use sdl2_window::Sdl2Window;
use opengl_graphics::Gl;
use shader_version::opengl::OpenGL_3_2;
use std::cell::RefCell;
use event::{
    Events,
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
                position: Point { x: MARGIN * x as f64, y: MARGIN * y as f64, z: 0.0 },
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
