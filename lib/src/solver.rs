use std::ops::Index;
use BoundaryCondition;
use BoundaryConditions;
use DataField;
use DifferentialEquation;
use PreCondition;
use scoped_threadpool::Pool;
use num_cpus;

pub struct Solver<D, BU, BD, BL, BR>
    where D: DifferentialEquation,
          BU: BoundaryCondition,
          BD: BoundaryCondition,
          BL: BoundaryCondition,
          BR: BoundaryCondition
{
    differential_equation: D,
    pub boundary_conditions: BoundaryConditions<BU, BD, BL, BR>,
    delta_t: f32,
    h: f32,
    fields: [DataField; 2],
    current_field: usize,
    thread_pool: Pool,
}

impl<D, BU, BD, BL, BR> Index<(usize, usize)> for Solver<D, BU, BD, BL, BR>
    where D: DifferentialEquation,
          BU: BoundaryCondition,
          BD: BoundaryCondition,
          BL: BoundaryCondition,
          BR: BoundaryCondition
{
    type Output = f32;

    fn index(&self, index: (usize, usize)) -> &f32 {
        &self.fields[self.current_field][index]
    }
}

impl<D, BU, BD, BL, BR> Solver<D, BU, BD, BL, BR>
    where D: DifferentialEquation + Sync + Send,
          BU: BoundaryCondition + Sync + Send,
          BD: BoundaryCondition + Sync + Send,
          BL: BoundaryCondition + Sync + Send,
          BR: BoundaryCondition + Sync + Send
{
    pub fn new(equation: D,
               boundary_conditions: BoundaryConditions<BU, BD, BL, BR>,
               dimensions: (usize, usize),
               delta_t: f32,
               h: f32)
               -> Self {
        Solver {
            differential_equation: equation,
            boundary_conditions: boundary_conditions,
            delta_t: delta_t,
            h: h,
            fields: [DataField::new(dimensions), DataField::new(dimensions)],
            current_field: 0,
            thread_pool: Pool::new(num_cpus::get() as u32),
        }
    }

    pub fn execute_precondition<P: PreCondition + ?Sized + Sync>(&mut self, precondition: &P) {
        let current_field = &mut self.fields[self.current_field];
        let thread_count = self.thread_pool.thread_count() as usize;

        self.thread_pool.scoped(|scope| {
            for chunk in current_field.chunks_mut(thread_count) {
                scope.execute(move || {
                    for (coord, cell) in chunk {
                        *cell = precondition.precondition(coord);
                    }
                });
            }
        });
    }

    pub fn solve_next_frame(&mut self) {
        // Prepare solving
        let (field_a, field_b) = self.fields.split_at_mut(1);
        let (current_field, target_field) = if self.current_field == 0 {
            (&mut field_a[0], &mut field_b[0])
        } else {
            (&mut field_b[0], &mut field_a[0])
        };
        let equation = &self.differential_equation;
        let delta_t = self.delta_t;
        let h = self.h;
        let thread_count = self.thread_pool.thread_count() as usize;

        // Execute Boundary Conditions
        self.boundary_conditions.calculate_boundaries(current_field);

        // Solve the differential equation for the whole field
        self.thread_pool.scoped(|scope| {
            let current_field = &current_field;
            for chunk in target_field.chunks_inner_mut(thread_count) {
                scope.execute(move || {
                    for (coord, cell) in chunk {
                        *cell = equation.solve(current_field, coord, delta_t, h);
                    }
                });
            }
        });

        // Swap the fields
        self.current_field ^= 1;
    }

    pub fn get_field(&self) -> &DataField {
        &self.fields[self.current_field]
    }

    pub fn get_field_mut(&mut self) -> &mut DataField {
        &mut self.fields[self.current_field]
    }
}
