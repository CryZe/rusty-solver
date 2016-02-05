use BoundaryCondition;

pub struct Neumann;

impl BoundaryCondition for Neumann {
    fn calculate_boundary(&self, other: f32) -> f32 {
        other
    }
}
