use std::ops::Index;
use BoundaryCondition;
use BoundaryConditions;
use DataField;
use DifferentialEquation;
use PreCondition;

pub struct Solver<D, BU, BD, BL, BR>
    where D: DifferentialEquation,
          BU: BoundaryCondition,
          BD: BoundaryCondition,
          BL: BoundaryCondition,
          BR: BoundaryCondition
{
    differential_equation: D,
    boundary_conditions: BoundaryConditions<BU, BD, BL, BR>,
    dimensions: (usize, usize),
    delta_t: f32,
    h: f32,
    fields: [DataField; 2],
    current_field: usize,
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
    where D: DifferentialEquation,
          BU: BoundaryCondition,
          BD: BoundaryCondition,
          BL: BoundaryCondition,
          BR: BoundaryCondition
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
            dimensions: dimensions,
            delta_t: delta_t,
            h: h,
            fields: [DataField::new(dimensions), DataField::new(dimensions)],
            current_field: 0,
        }
    }

    pub fn execute_precondition<P: PreCondition + ?Sized>(&mut self, precondition: &P) {
        let (nx, ny) = self.dimensions;
        let current_field = &mut self.fields[self.current_field];

        for y in 0..ny {
            for x in 0..nx {
                let coord = (x, y);
                current_field[coord] = precondition.precondition(coord);
            }
        }
    }

    pub fn solve_next_frame(&mut self) {
        // Prepare solving
        let (nx, ny) = self.dimensions;
        let (end_x, end_y) = (nx - 1, ny - 1);
        let (field_a, field_b) = self.fields.split_at_mut(1);
        let (current_field, target_field) = if self.current_field == 0 {
            (&mut field_a[0], &mut field_b[0])
        } else {
            (&mut field_b[0], &mut field_a[0])
        };

        // Execute Boundary Conditions
        self.boundary_conditions.calculate_boundaries(current_field);

        // Solve the differential equation for the whole field
        for y in 1..end_y {
            for x in 1..end_x {
                let coord = (x, y);
                target_field[coord] = self.differential_equation
                                          .solve(current_field, coord, self.delta_t, self.h);
            }
        }

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
