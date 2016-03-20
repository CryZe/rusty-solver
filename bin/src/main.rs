extern crate lib_rusty_solver as solver;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;
extern crate palette;

mod switchable_boundary_condition;
mod utils;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;

// mod thermal_conduction_app;
// use thermal_conduction_app::ThermalConductionApp as App;

mod phasefield_app;
use phasefield_app::PhaseFieldApp as App;

fn main() {
    let opengl = OpenGL::V2_1;

    let dimensions = (200, 200);
    let (nx, ny) = dimensions;

    let mut window: Window = WindowSettings::new("Rusty Solver", [4 * nx as u32, 4 * ny as u32])
                                 .opengl(opengl)
                                 .exit_on_esc(true)
                                 .build()
                                 .unwrap();

    let mut app = App::new(opengl, dimensions);

    let mut event_loop = window.events();
    event_loop.set_max_fps(60);
    event_loop.set_ups(200);

    while let Some(e) = event_loop.next(&mut window) {
        match e {
            Event::Render(r) => app.render(&r),
            Event::Update(u) => app.update(&u),
            Event::Input(i) => app.handle_input(&i),
            _ => {}
        }
    }
}
