use crate::color::{CFAPattern, BayerPos};
use image::{Rgb32FImage, ImageBuffer, Rgb, Luma, };

pub fn g_demos(image: &ImageBuffer<Luma<f32>, Vec<f32>>) ->
   Rgb32FImage {
    let (width, height) = image.dimensions();
    let mut out = Rgb32FImage::new(width, height);
    for (x, y, val) in image.enumerate_pixels() {
        out.put_pixel(x, y, Rgb([val.0[0]; 3]));
    }
    out
}

pub fn rgb_demos(image: &ImageBuffer<Luma<f32>, Vec<f32>>, cfa: &CFAPattern) ->
   Rgb32FImage {

    let (width, height) = image.dimensions();
    let mut out = Rgb32FImage::new(width - 1, height - 1);

    for x in 1..width {
        for y in 1..height {
            let vals = (
                image.get_pixel(x-1, y-1).0[0], image.get_pixel(x, y-1).0[0],
                image.get_pixel(x-1, y).0[0],   image.get_pixel(x, y).0[0]);

            let (r, g, b, g2) = match cfa.get_bayer_pos(x, y) {
                BayerPos::R => (vals.3, vals.2, vals.0, vals.1),
                BayerPos::G => (vals.2, vals.3, vals.1, vals.0),
                BayerPos::B => (vals.0, vals.1, vals.3, vals.2),
                BayerPos::G2 => (vals.1, vals.0, vals.2, vals.3),
            };

            let new_pixel = Rgb([r, (g + g2) / 2.0, b]);
            out.put_pixel(x - 1, y - 1, new_pixel);
        }
    }
    out
}

pub fn passthrough_demos(image: &ImageBuffer<Luma<f32>, Vec<f32>>, cfa: &CFAPattern) ->
Rgb32FImage {

    let (width, height) = image.dimensions();
    let mut out = Rgb32FImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let new_pixel = match cfa.get_bayer_pos(x, y) {
                BayerPos::B => Rgb([0.0, 0.0, image.get_pixel(x, y).0[0]]),
                BayerPos::G => Rgb([0.0, image.get_pixel(x, y).0[0], 0.0]),
                BayerPos::R => Rgb([image.get_pixel(x, y).0[0], 0.0, 0.0]),
                BayerPos::G2 => Rgb([0.0, image.get_pixel(x, y).0[0], 0.0]),
                // _ => Rgb([0.0, image.get_pixel(x, y).0[0], 0.0]),

            };

            out.put_pixel(x, y, new_pixel);
        }
    }
    out
}

