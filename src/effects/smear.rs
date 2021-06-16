use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use crate::Canvas;

pub fn build(img: &mut DynamicImage, canvas: Canvas, smear_down: bool) -> &mut DynamicImage {
    let base_row = get_base_row(img, &canvas, smear_down);

    for y in canvas.row_min..canvas.row_max + 1 {
        for x in canvas.col_min..canvas.col_max + 1 {
            let ix = (x - canvas.col_min) as usize;
            img.put_pixel(x, y, base_row[ix])
        }
    }

    img
}

/// Get the row to smear with.
fn get_base_row(img: &mut DynamicImage, canvas: &Canvas, smear_down: bool) -> Vec<Rgba<u8>> {
    let mut vec = Vec::new();

    let y = if smear_down {
        canvas.row_min
    } else {
        canvas.row_max
    };

    for x in canvas.col_min..canvas.col_max + 1 {
        vec.push(img.get_pixel(x, y))
    }

    vec
}
