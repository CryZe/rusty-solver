use DataField;
use DifferentialEquation;

pub struct PhaseFieldEquation {
    pub t: f32,
    pub gamma: f32,
    pub tau: f32,
    pub epsilon: f32,
    pub tm: f32,
    pub la: f32,
}

impl DifferentialEquation for PhaseFieldEquation {
    fn solve(&self, field: &DataField, (x, y): (usize, usize), delta_t: f32, h: f32) -> f32 {
        let center = field[(x, y)];
        let left = field[(x - 1, y)];
        let right = field[(x + 1, y)];
        let up = field[(x, y - 1)];
        let down = field[(x, y + 1)];

        let center2 = center * center;
        let center3 = center2 * center;

        let l = self.la * (self.tm - self.t) / self.tm;

        let a = 2.0 * self.gamma * (up + down + left + right - 4.0 * center) / (h * h);
        let w = 18.0 / (self.epsilon * self.epsilon) * self.gamma *
                (2.0 * center3 - 3.0 * center2 + center);
        let f = l / self.epsilon * 6.0 * center * (1.0 - center);

        center + delta_t / self.tau * (a - w + f)
    }
}
