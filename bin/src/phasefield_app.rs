use solver::prelude::*;
use solver::differential_equation::PhaseFieldEquation;
use solver::precondition::ConstantPreCondition;
use solver::boundary_condition::{Neumann, Dirichlet};
use switchable_boundary_condition::SwitchableBoundaryCondition;

use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL, Texture};
use graphics::Transformed;

use utils::{draw_cube, to_greyscale_image};
use super::graphics;

pub struct PhaseFieldApp {
    gl: GlGraphics,
    solver: Solver<PhaseFieldEquation,
                   SwitchableBoundaryCondition<Dirichlet, Neumann>,
                   SwitchableBoundaryCondition<Dirichlet, Neumann>,
                   SwitchableBoundaryCondition<Dirichlet, Neumann>,
                   Dirichlet>,
    mouse_coord: (f64, f64),
    left_mouse_down: bool,
    right_mouse_down: bool,
    window_scale: f64,
}

impl PhaseFieldApp {
    pub fn new(opengl: OpenGL, dimensions: (usize, usize)) -> Self {
        let ref precondition = ConstantPreCondition(0.0);
        let equation = PhaseFieldEquation {
            t: 0.9,
            gamma: 1.0,
            tau: 2.0,
            epsilon: 5.0,
            tm: 1.0,
            la: 0.5,
        };

        let boundary_conditions =
            BoundaryConditions::new(SwitchableBoundaryCondition::new(Dirichlet(0.0), Neumann),
                                    SwitchableBoundaryCondition::new(Dirichlet(0.0), Neumann),
                                    SwitchableBoundaryCondition::new(Dirichlet(0.0), Neumann),
                                    Dirichlet(0.0));

        let mut solver = Solver::new(equation, boundary_conditions, dimensions, 0.1, 1.0);

        solver.execute_precondition(precondition);

        PhaseFieldApp {
            gl: GlGraphics::new(opengl),
            solver: solver,
            mouse_coord: (0.0, 0.0),
            left_mouse_down: false,
            right_mouse_down: false,
            window_scale: 5.0,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        let solver = &mut self.solver;

        let scale = &mut self.window_scale;

        self.gl.draw(args.viewport(), |c, gl| {
            graphics::clear([0.0, 0.0, 0.0, 1.0], gl);
            let field = solver.get_field();
            let field_image = to_greyscale_image(field);
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
        if self.left_mouse_down || self.right_mouse_down {
            let cube_phi = if self.left_mouse_down {
                1.0
            } else {
                0.0
            };
            draw_cube(self.solver.get_field_mut(), cube_coord, cube_phi);
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
            _ => {}
        }
    }

    fn handle_mouse_click(&mut self, button: MouseButton, press: bool) {
        match button {
            MouseButton::Left => {
                self.left_mouse_down = press;
            }
            MouseButton::Right => {
                self.right_mouse_down = press;
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
