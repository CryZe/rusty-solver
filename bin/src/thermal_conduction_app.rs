use solver::prelude::*;
use solver::differential_equation::ThermalConduction;
use solver::precondition::ConstantPreCondition;
use solver::boundary_condition::{Neumann, Dirichlet};
use switchable_boundary_condition::SwitchableBoundaryCondition;

use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use graphics::Transformed;

use utils::{draw_cube, draw_neumann_rectangle, to_temperature_image};
use super::graphics;

pub struct ThermalConductionApp {
    gl: GlGraphics,
    solver: Solver<ThermalConduction,
                   SwitchableBoundaryCondition<Dirichlet, Neumann>,
                   SwitchableBoundaryCondition<Dirichlet, Neumann>,
                   SwitchableBoundaryCondition<Dirichlet, Neumann>,
                   Neumann>,
    mouse_coord: (f64, f64),
    mouse_down: bool,
    window_scale: f64,
    pipe_open: bool,
}

impl ThermalConductionApp {
    pub fn new(opengl: OpenGL, dimensions: (usize, usize)) -> Self {
        let ref precondition = ConstantPreCondition(0.0);
        let equation = ThermalConduction;

        let boundary_conditions =
            BoundaryConditions::new(SwitchableBoundaryCondition::new(Dirichlet(0.0), Neumann),
                                    SwitchableBoundaryCondition::new(Dirichlet(10.0), Neumann),
                                    SwitchableBoundaryCondition::new(Dirichlet(5.0), Neumann),
                                    Neumann);

        let mut solver = Solver::new(equation, boundary_conditions, dimensions, 0.1, 1.0);

        solver.execute_precondition(precondition);

        ThermalConductionApp {
            gl: GlGraphics::new(opengl),
            solver: solver,
            mouse_coord: (0.0, 0.0),
            mouse_down: false,
            window_scale: 5.0,
            pipe_open: false,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let solver = &mut self.solver;

        let scale = &mut self.window_scale;

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
            let field = solver.get_field();
            let field_image = to_temperature_image(field);
            let texture = Texture::from_image(&field_image);
            let image = graphics::Image::new();
            let w = args.width as f64 / 200.0;
            let h = args.height as f64 / 200.0;
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

    pub fn update(&mut self, _: &UpdateArgs) {
        let (mouse_x, mouse_y) = self.mouse_coord;
        let cube_coord = ((mouse_x / self.window_scale) as isize,
                          (mouse_y / self.window_scale) as isize);
        let cube_temperature = if self.mouse_down {
            10.0
        } else {
            0.0
        };
        draw_cube(self.solver.get_field_mut(), cube_coord, cube_temperature);

        draw_cube(self.solver.get_field_mut(), (105, 115), 1000.0);
        draw_neumann_rectangle(self.solver.get_field_mut(), (100, 100), (50, 10));
        draw_neumann_rectangle(self.solver.get_field_mut(), (100, 120), (70, 10));
        draw_neumann_rectangle(self.solver.get_field_mut(), (90, 100), (10, 30));
        draw_neumann_rectangle(self.solver.get_field_mut(), (170, 60), (10, 70));
        draw_neumann_rectangle(self.solver.get_field_mut(), (150, 60), (10, 50));
        if !self.pipe_open {
            draw_neumann_rectangle(self.solver.get_field_mut(), (160, 60), (10, 10));
        }

        self.solver.solve_next_frame();
    }

    fn handle_key_press(&mut self, key: Key) {
        match key {
            Key::Down => {
                self.solver.boundary_conditions.down.toggle();
            }
            Key::Up => {
                self.solver.boundary_conditions.up.toggle();
            }
            Key::Left => {
                self.solver.boundary_conditions.left.toggle();
            }
            Key::Space => {
                self.pipe_open = !self.pipe_open;
            }
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

    pub fn handle_input(&mut self, input: &Input) {
        match input {
            &Input::Press(Button::Keyboard(x)) => self.handle_key_press(x),
            &Input::Press(Button::Mouse(x)) => self.handle_mouse_click(x, true),
            &Input::Release(Button::Mouse(x)) => self.handle_mouse_click(x, false),
            &Input::Move(x) => self.handle_mouse_move(x),
            _ => {}
        }
    }
}
