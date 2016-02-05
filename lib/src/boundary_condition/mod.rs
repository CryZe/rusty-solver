mod dirichlet;
mod neumann;

use DataField;

pub use self::dirichlet::Dirichlet;
pub use self::neumann::Neumann;

pub trait BoundaryCondition {
    fn calculate_boundary(&self, other: f32) -> f32;
}

pub struct BoundaryConditions<BU, BD, BL, BR>
    where BU: BoundaryCondition,
          BD: BoundaryCondition,
          BL: BoundaryCondition,
          BR: BoundaryCondition
{
    up: BU,
    down: BD,
    left: BL,
    right: BR,
}

impl<BU, BD, BL, BR> BoundaryConditions<BU, BD, BL, BR>
    where BU: BoundaryCondition,
          BD: BoundaryCondition,
          BL: BoundaryCondition,
          BR: BoundaryCondition
{
    pub fn new(up: BU, down: BD, left: BL, right: BR) -> Self {
        BoundaryConditions {
            up: up,
            down: down,
            left: left,
            right: right,
        }
    }

    pub fn calculate_boundaries(&self, field: &mut DataField) {
        let (nx, ny) = field.dimensions;
        let (end_x, end_y) = (nx - 1, ny - 1);
        let (end_other_x, end_other_y) = (nx - 2, ny - 2);

        for y in 0..ny {
            // Left
            let other = field[(1, y)];
            field[(0, y)] = self.left.calculate_boundary(other);

            // Right
            let other = field[(end_other_x, y)];
            field[(end_x, y)] = self.right.calculate_boundary(other);
        }

        for x in 0..nx {
            // Up
            let other = field[(x, 1)];
            field[(x, 0)] = self.up.calculate_boundary(other);

            // Down
            let other = field[(x, end_other_y)];
            field[(x, end_y)] = self.down.calculate_boundary(other);
        }
    }
}
