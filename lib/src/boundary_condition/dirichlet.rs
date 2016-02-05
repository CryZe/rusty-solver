use BoundaryCondition;

pub struct Dirichlet(pub f32);

impl BoundaryCondition for Dirichlet {
    fn calculate_boundary(&self, _: f32) -> f32 {
        let &Dirichlet(value) = self;
        value
    }
}
