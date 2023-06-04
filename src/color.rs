use image::{Rgb, Rgb32FImage};

pub enum CFAPattern {
    Bayer(u32, u32), // RGBG + offsets
    Xtrans(u32, u32),
    Mono,
}

#[derive(Debug)]
pub enum Color{
    Red,
    Green,
    Blue,
    Grey,
}

#[derive(Debug)]
pub enum BayerPos{ R, G, B, G2 }

impl CFAPattern {
    pub fn get_col(&self, row: u32, col: u32) -> Color {
        match self {
            CFAPattern::Bayer(c, r) => {
                let bayer_top = (row + r) % 2 == 0;
                let bayer_left = (col + c) % 2 == 0;
                if bayer_top && bayer_left {
                    Color::Red
                } else if !bayer_top && !bayer_left {
                    Color::Blue
                } else {
                    Color::Green
                }
            },
            _ => Color::Grey,
        }
    }

    pub fn get_bayer_pos(&self, row: u32, col: u32) -> BayerPos {
        match self {
            CFAPattern::Bayer(c, r) => {
                let bayer_top = (row + r) % 2 == 0;
                let bayer_left = (col + c) % 2 == 0;
                if bayer_top && bayer_left {
                    BayerPos::R
                } else if !bayer_top && bayer_left {
                    BayerPos::G
                } else if !bayer_top && !bayer_left {
                    BayerPos::B
                } else {
                    BayerPos::G2
                }
            },
            _ => BayerPos::R,
        }
    }
}

fn linear_to_srgb(x: f32) -> f32 {
    if x <= 0.0 {
        0.0
    } else if x < 0.0031308 {
        12.92 * x
    } else if x < 1.0 {
        (1.055 * x).powf(1.0 / 2.4) - 0.055
    } else {
        1.0
    }
}

fn srgb_to_linear(x: f32) -> f32 {
    if x <= 0.0 {
        0.0
    } else if x < 0.04045 {
        x / 12.92
    } else if x < 1.0 {
        ((x + 0.055) / 1.055).powf(2.4)
    } else {
        1.0
    }
}

pub fn pixel_to_srgb(pix: &Rgb<f32>) -> Rgb<f32> {
    Rgb([
        linear_to_srgb(pix.0[0]),
        linear_to_srgb(pix.0[1]),
        linear_to_srgb(pix.0[2]),
    ])
}

pub fn to_srgb(image: &Rgb32FImage) -> Rgb32FImage {

    let (width, height) = image.dimensions();
    let mut out = Rgb32FImage::new(width, height);
    for (x, y, pix) in image.enumerate_pixels() {
        out.put_pixel(x, y, pixel_to_srgb(pix));
    }
    out
}
