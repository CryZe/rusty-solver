use std::ops::{Index, IndexMut};

pub struct DataField {
    pub dimensions: (usize, usize),
    field: Vec<f32>,
}

impl Index<(usize, usize)> for DataField {
    type Output = f32;

    fn index(&self, (x, y): (usize, usize)) -> &f32 {
        let (nx, _) = self.dimensions;
        let index = x + nx * y;
        &self.field[index]
    }
}

impl IndexMut<(usize, usize)> for DataField {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut f32 {
        let (nx, _) = self.dimensions;
        let index = x + nx * y;
        &mut self.field[index]
    }
}

impl DataField {
    pub fn new((nx, ny): (usize, usize)) -> Self {
        DataField {
            dimensions: (nx, ny),
            field: vec![0.0; nx * ny],
        }
    }

    pub fn contains(&self, (x, y): (usize, usize)) -> bool {
        let (nx, ny) = self.dimensions;
        x < nx && y < ny
    }

    pub fn set(&mut self, coord: (usize, usize), value: f32) {
        if self.contains(coord) {
            self[coord] = value;
        }
    }
}
