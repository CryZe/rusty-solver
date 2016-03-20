use DataField;
use DifferentialEquation;

pub struct ThermalConduction;

impl DifferentialEquation for ThermalConduction {
    fn solve(&self, field: &DataField, (x, y): (usize, usize), delta_t: f32, h: f32) -> f32 {
        let center = field[(x, y)];
        let left = field[(x - 1, y)];
        let right = field[(x + 1, y)];
        let up = field[(x, y - 1)];
        let down = field[(x, y + 1)];

        let sum = left + right + up + down - 4.0 * center;

        center + delta_t * sum / (h * h)
    }
}
