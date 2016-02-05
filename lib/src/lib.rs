pub mod boundary_condition;
mod data_field;
pub mod differential_equation;
mod solver;
pub mod precondition;

pub use boundary_condition::{BoundaryCondition, BoundaryConditions};
pub use data_field::DataField;
pub use differential_equation::DifferentialEquation;
pub use precondition::PreCondition;
pub use solver::Solver;

pub mod prelude {
    pub use BoundaryCondition;
    pub use BoundaryConditions;
    pub use DifferentialEquation;
    pub use PreCondition;
    pub use Solver;
}
