mod raw;
mod color;
mod demosaic;
mod luma_ops;
mod asc_cdl;

use std::fs::File;
use std::io::BufWriter;
use image::{DynamicImage, ImageOutputFormat};

fn main() {
    pollster::block_on(run());
}

async fn run() {
    let raw_data = raw::get_raw_data("./data/iso100-z6-burnt.NEF");

    let raw_img = DynamicImage::ImageLuma16(raw_data.buffer).to_luma16();
    let raw_img = luma_ops::black_point(&raw_img, raw_data.min, raw_data.max);
    let raw_img = luma_ops::white_balance(&raw_img, &raw_data.cfa, 1.646, 1.0, 1.408);
    let raw_img = luma_ops::exposure(&raw_img, 0.7);

    let img =  demosaic::rgb_demos(&raw_img, &raw_data.cfa);

    let img = asc_cdl::asc_combined(&img, 1.0, -0.01, 1.0);

    let img =  color::to_srgb(&img);

    // crate an RGB8 from it and save to jpg
    let img8b = DynamicImage::ImageRgb32F(img).to_rgb8();
    let mut f = BufWriter::new(File::create("test.bmp").unwrap());
    img8b.write_to(&mut f, ImageOutputFormat::Bmp).unwrap();
}
