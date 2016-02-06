use solver::prelude::*;
use solver::boundary_condition::Neumann;
use solver::DataField;

use image::RgbaImage;
use palette::{Gradient, Hsv, Rgb, RgbHue};

pub fn draw_cube(field: &mut DataField, (x, y): (isize, isize), temperature: f32) {
    for y in y - 5..y + 5 {
        for x in x - 5..x + 5 {
            if y >= 0 && x >= 0 {
                let (x, y) = (x as usize, y as usize);
                field.set((x, y), temperature);
            }
        }
    }
}

pub fn draw_neumann_rectangle(field: &mut DataField,
                          (x, y): (usize, usize),
                          (width, height): (usize, usize)) {
    let neumann = Neumann;

    let (x_end, y_end) = (x + width - 1, y + height - 1);

    for y in y..y + height {
        // Left
        let other = field[(x - 1, y)];
        let value = neumann.calculate_boundary(other);
        field.set((x, y), value);

        // Right
        let other = field[(x_end + 1, y)];
        let value = neumann.calculate_boundary(other);
        field.set((x_end, y), value);
    }

    for x in x..x + width {
        // Up
        let other = field[(x, y - 1)];
        let value = neumann.calculate_boundary(other);
        field.set((x, y), value);

        // Down
        let other = field[(x, y_end + 1)];
        let value = neumann.calculate_boundary(other);
        field.set((x, y_end), value);
    }
}

pub fn to_image(field: &DataField) -> RgbaImage {
    let (nx, ny) = field.dimensions;
    let mut image = RgbaImage::new(nx as u32, ny as u32);

    let red: Rgb<_> = Hsv::new(RgbHue::from_radians(0.0), 1.0, 1.0).into();
    let orange: Rgb<_> = Hsv::new(RgbHue::from_radians(0.6), 1.0, 1.0).into();
    let yellow: Rgb<_> = Hsv::new(RgbHue::from_radians(0.9), 1.0, 1.0).into();
    let greenish: Rgb<_> = Hsv::new(RgbHue::from_radians(2.6), 0.3, 0.8).into();
    let blue: Rgb<_> = Hsv::new(RgbHue::from_radians(3.9), 1.0, 0.8).into();

    let gradient = Gradient::new(vec![blue, greenish, yellow, orange, red]);

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let value = field[(x as usize, y as usize)] / 30.0;
        pixel.data = gradient.get(value).to_pixel();
    }

    image
}
