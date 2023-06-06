use image::{Rgb, Rgb32FImage};

fn asc_cdl_f32(x: f32, s: f32, o: f32, p: f32) -> f32 {
    (x * s + o).max(0.0).powf(p)
}

fn asc_cdl_pixel(pix: &Rgb<f32>, s: f32, o: f32, p: f32)
        -> Rgb<f32> {
    Rgb([
        asc_cdl_f32(pix.0[0], s, o, p),
        asc_cdl_f32(pix.0[1], s, o, p),
        asc_cdl_f32(pix.0[2], s, o, p),
    ])
}


pub fn asc_combined(image: &Rgb32FImage, slope: f32, offset: f32, power: f32)
        -> Rgb32FImage {
    let (width, height) = image.dimensions();
    let mut out = Rgb32FImage::new(width, height);
    for (x, y, pix) in image.enumerate_pixels() {
        out.put_pixel(x, y, asc_cdl_pixel(pix, slope, offset, power));
    }
    out
}
