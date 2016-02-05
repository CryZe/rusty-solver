mod thermal_conduction;

pub use self::thermal_conduction::ThermalConduction;

use DataField;

pub trait DifferentialEquation {
    fn solve(&mut self, field: &DataField, coord: (usize, usize), delta_t: f32, h: f32) -> f32;
}
