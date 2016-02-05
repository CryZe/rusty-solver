extern crate lib_rusty_solver as solver;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate image;
extern crate palette;

use solver::prelude::*;
use solver::differential_equation::ThermalConduction;
use solver::precondition::ConstantPreCondition;
use solver::boundary_condition::{Neumann, Dirichlet};
use solver::DataField;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use graphics::Transformed;
use image::RgbaImage;
use palette::{Gradient, Hsv, Rgb, RgbHue};

fn draw_cube(field: &mut DataField, (x, y): (isize, isize), temperature: f32) {
    for y in y - 5..y + 5 {
        for x in x - 5..x + 5 {
            if y >= 0 && x >= 0 {
                let (x, y) = (x as usize, y as usize);
                if field.contains((x, y)) {
                    field[(x, y)] = temperature;
                }
            }
        }
    }
}

fn to_image(field: &DataField) -> RgbaImage {
    let (nx, ny) = field.dimensions;
    let mut image = RgbaImage::new(nx as u32, ny as u32);

    let red: Rgb<_> = Hsv::new(RgbHue::from_radians(0.0), 1.0, 1.0).into();
    let orange: Rgb<_> = Hsv::new(RgbHue::from_radians(0.6), 1.0, 1.0).into();
    let yellow: Rgb<_> = Hsv::new(RgbHue::from_radians(0.9), 1.0, 1.0).into();
    let greenish: Rgb<_> = Hsv::new(RgbHue::from_radians(2.6), 0.3, 0.8).into();
    let blue: Rgb<_> = Hsv::new(RgbHue::from_radians(3.9), 1.0, 0.8).into();

    let gradient = Gradient::new(vec![blue, greenish, yellow, orange, red]);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let value = field[(x as usize, y as usize)] / 10.0;
        pixel.data = gradient.get(value).to_pixel();
    }

    image
}

pub struct App {
    gl: GlGraphics,
    solver: Solver<ThermalConduction,
                   Dirichlet,
                   Dirichlet,
                   Dirichlet,
                   Neumann>,
    mouse_coord: (f64, f64),
    mouse_down: bool,
    window_scale: f64,
}

impl App {
    fn new(opengl: OpenGL, dimensions: (usize, usize)) -> Self {
        let ref precondition = ConstantPreCondition(0.0);
        let equation = ThermalConduction;

        let boundary_conditions = BoundaryConditions::new(Dirichlet(0.0),
                                                          Dirichlet(10.0),
                                                          Dirichlet(5.0),
                                                          Neumann);

        let mut solver = Solver::new(equation, boundary_conditions, dimensions, 0.1, 1.0);

        solver.execute_precondition(precondition);

        App {
            gl: GlGraphics::new(opengl),
            solver: solver,
            mouse_coord: (0.0, 0.0),
            mouse_down: false,
            window_scale: 5.0,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        let solver = &mut self.solver;

        let scale = &mut self.window_scale;

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
            let field = solver.get_field();
            let field_image = to_image(field);
            let texture = Texture::from_image(&field_image);
            let image = graphics::Image::new();
            let w = args.width as f64 / 100.0;
            let h = args.height as f64 / 100.0;
            *scale = if w < h {
                w
            } else {
                h
            };
            image.draw(&texture,
                       graphics::default_draw_state(),
                       c.transform.scale(*scale, *scale),
                       gl);
        });
    }

    fn update(&mut self, _: &UpdateArgs) {
        let (mouse_x, mouse_y) = self.mouse_coord;
        let cube_coord = ((mouse_x / self.window_scale) as isize,
                          (mouse_y / self.window_scale) as isize);
        let cube_temperature = if self.mouse_down {
            10.0
        } else {
            0.0
        };
        draw_cube(self.solver.get_field_mut(), cube_coord, cube_temperature);

        self.solver.solve_next_frame();
    }

    fn handle_key_press(&mut self, key: Key) {
        match key {
            _ => {}
        }
    }

    fn handle_mouse_click(&mut self, button: MouseButton, press: bool) {
        match button {
            MouseButton::Left => {
                self.mouse_down = press;
            }
            _ => {}
        }
    }

    fn handle_mouse_move(&mut self, motion: Motion) {
        match motion {
            Motion::MouseCursor(x, y) => {
                self.mouse_coord = (x, y);
            }
            _ => {}
        }
    }

    fn handle_input(&mut self, input: &Input) {
        match input {
            &Input::Press(Button::Keyboard(x)) => self.handle_key_press(x),
            &Input::Press(Button::Mouse(x)) => self.handle_mouse_click(x, true),
            &Input::Release(Button::Mouse(x)) => self.handle_mouse_click(x, false),
            &Input::Move(x) => self.handle_mouse_move(x),
            _ => {}
        }
    }
}

fn main() {
    let opengl = OpenGL::V2_1;

    let dimensions = (100, 100);
    let (nx, ny) = dimensions;

    let mut window: Window = WindowSettings::new("Rusty Solver", [6 * nx as u32, 6 * ny as u32])
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
