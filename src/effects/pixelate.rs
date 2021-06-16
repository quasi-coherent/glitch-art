use image::{DynamicImage, GenericImage, GenericImageView, Rgba};

use crate::Canvas;

pub struct Pixelate<'a> {
    pub canvas: &'a Canvas,
    pub num_pixels: u32,
}

struct PixelRegion {
    side_len: u32,
    num_rows: u32,
    num_cols: u32,
}

pub fn build<'a>(img: &'a mut DynamicImage, pixelate: &Pixelate) -> &'a mut DynamicImage {
    let reg = get_pixel_size(pixelate);

    for r in 0..reg.num_rows {
        println!("r={}", r);
        for c in 0..reg.num_cols {
            println!("c={}", c);
            let x1 = r * reg.side_len + pixelate.canvas.col_min;
            let x2 = (r + 1) * reg.side_len + pixelate.canvas.col_min;
            let y1 = c * reg.side_len + pixelate.canvas.row_min;
            let y2 = (c + 1) * reg.side_len + pixelate.canvas.row_min;
            let p = get_avg_pixel(img, x1, x2, y1, y2);

            write_pixel_region(img, p, x1, x2, y1, y2)
        }
    }

    img
}

/// Write the pixel (average of all pixels in the region) to the region.
fn write_pixel_region(img: &mut DynamicImage, p: Rgba<u8>, x1: u32, x2: u32, y1: u32, y2: u32) {
    for y in y1..y2 + 1 {
        for x in x1..x2 + 1 {
            img.put_pixel(x, y, p)
        }
    }
}

/// Calculate the size of each pixel.
fn get_pixel_size(pixelate: &Pixelate) -> PixelRegion {
    let w = pixelate.canvas.col_max - pixelate.canvas.col_min;
    let h = pixelate.canvas.row_max - pixelate.canvas.row_min;
    let area = w * h;
    let pixel_area = area / pixelate.num_pixels;
    let len = (pixel_area as f32).sqrt() as u32;
    let nrows = w / len;
    let ncols = h / len;

    PixelRegion {
        side_len: len,
        num_rows: nrows,
        num_cols: ncols,
    }
}

/// Get the average pixel in a region.
fn get_avg_pixel(img: &mut DynamicImage, x1: u32, x2: u32, y1: u32, y2: u32) -> Rgba<u8> {
    let mut vec = Vec::new();

    for y in y1..y2 + 1 {
        for x in x1..x2 + 1 {
            vec.push(img.get_pixel(x, y))
        }
    }

    let len = vec.len();

    let (sum_r, sum_g, sum_b, sum_a) = vec.into_iter().fold(
        (0, 0, 0, 0),
        |(acc_r, acc_g, acc_b, acc_a), Rgba([r, g, b, a])| {
            (
                acc_r + r as usize,
                acc_g + g as usize,
                acc_b + b as usize,
                acc_a + a as usize,
            )
        },
    );

    let avg_r = sum_r as usize / len;
    let avg_g = sum_g as usize / len;
    let avg_b = sum_b as usize / len;
    let avg_a = sum_a as usize / len;

    Rgba([avg_r as u8, avg_g as u8, avg_b as u8, avg_a as u8])
}
