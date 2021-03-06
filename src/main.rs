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
use std::rand::{task_rng, Rng};
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

const SPEED_AMPLITUDE: f64 = 100.0;


fn make_boids(num: int) -> Vec<Boid> {
    // makes num^2 boids
    let mut v = Vec::new();
    for x in range(1i, num + 1) {
        for y in range(1i, num + 1) {
            let b = Boid {
                position: Point { x: MARGIN * x as f64, y: MARGIN * y as f64, z: 0.0 },
                velocity: random_speed(),
                ..Default::default()
            };
            v.push(b);
        }
    }
    v
}

fn random_speed() -> Point {
    let mut rng = task_rng();
    Point { x: rng.gen_range(-SPEED_AMPLITUDE, SPEED_AMPLITUDE),
            y: rng.gen_range(-SPEED_AMPLITUDE, SPEED_AMPLITUDE),
            z: 0.0 }
}


fn main() {
    let window = Sdl2Window::new(
        OpenGL_3_2,
        piston::WindowSettings::default()
    );
    let window_args = piston::WindowSettings::default();

    let mut app = App { gl: Gl::new(OpenGL_3_2),
                        boids: make_boids(13),
                        width: window_args.size[0],
                        height: window_args.size[1] };

    let window = RefCell::new(window);
    for e in Events::new(&window) {
        e.render(|r| app.render(window.borrow_mut().deref_mut(), r));
        e.update(|u| app.update(window.borrow_mut().deref_mut(), u));
    }
}
