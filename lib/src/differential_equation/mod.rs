mod thermal_conduction;
mod phasefield_equation;

pub use self::thermal_conduction::ThermalConduction;
pub use self::phasefield_equation::PhaseFieldEquation;

use DataField;

pub trait DifferentialEquation {
    fn solve(&mut self, field: &DataField, coord: (usize, usize), delta_t: f32, h: f32) -> f32;
}
