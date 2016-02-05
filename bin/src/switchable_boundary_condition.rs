use solver::prelude::*;

pub struct SwitchableBoundaryCondition<F, S>
    where F: BoundaryCondition,
          S: BoundaryCondition
{
    first: F,
    second: S,
    use_second: bool,
}

impl<F, S> SwitchableBoundaryCondition<F, S>
    where F: BoundaryCondition,
          S: BoundaryCondition
{
    pub fn new(first: F, second: S) -> Self {
        SwitchableBoundaryCondition {
            first: first,
            second: second,
            use_second: false,
        }
    }

    pub fn toggle(&mut self) {
        self.use_second = !self.use_second;
    }
}

impl<F, S> BoundaryCondition for SwitchableBoundaryCondition<F, S>
    where F: BoundaryCondition,
          S: BoundaryCondition
{
    fn calculate_boundary(&self, other: f32) -> f32 {
        if self.use_second {
            self.second.calculate_boundary(other)
        } else {
            self.first.calculate_boundary(other)
        }
    }
}
