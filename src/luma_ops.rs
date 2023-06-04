use crate::color::{CFAPattern, Color};
use image::{ImageBuffer, Luma};

pub fn black_point(image: &ImageBuffer<Luma<u16>, Vec<u16>>, black: u16, white: u16)
                -> ImageBuffer<Luma<f32>, Vec<f32>> {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);
    for (x, y, val) in image.enumerate_pixels() {
        let new_val: f32 = (val.0[0] - black) as f32 / white as f32;
        out.put_pixel(x, y, Luma([new_val]));
    }
    out
}

pub fn exposure(image: &ImageBuffer<Luma<f32>, Vec<f32>>, ev: f32)
                -> ImageBuffer<Luma<f32>, Vec<f32>> {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);
    for (x, y, val) in image.enumerate_pixels() {
        let new_val = val.0[0] * 2.0_f32.powf(ev);
        out.put_pixel(x, y, Luma([new_val]));
    }
    out
}

pub fn white_balance(image: &ImageBuffer<Luma<f32>, Vec<f32>>, cfa: &CFAPattern,
                r: f32, g: f32, b: f32) -> ImageBuffer<Luma<f32>, Vec<f32>> {
    let (width, height) = image.dimensions();
    let mut out = ImageBuffer::new(width, height);
    for (x, y, val) in image.enumerate_pixels() {
        let new_val = match cfa.get_col(x, y) {
            Color::Red   => val.0[0] * r,
            Color::Green => val.0[0] * g,
            Color::Blue  => val.0[0] * b,
            _            => val.0[0],
        };
       out.put_pixel(x, y, Luma([new_val]));
    }
    out
}
