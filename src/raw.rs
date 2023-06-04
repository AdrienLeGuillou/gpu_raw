extern crate libraw_sys as libraw;

use std::ffi::CString;
use std::slice;
use image::{ImageBuffer,Luma};
use crate::color::CFAPattern;

// TODO ------------------------------------------------------------------------
// - get_bayer_pixels
//      - return colors(f32)
//      - close prcs


pub struct RawData {
    pub buffer: ImageBuffer<Luma<u16>, Vec<u16>>,
    pub width: usize,
    pub height: usize,
    pub min: u16,
    pub max: u16,
    pub cfa: CFAPattern,
}


pub fn get_raw_data(file_path: &str) -> RawData {
    let prcs = open_processor(file_path);
    let (width, height) = get_raw_dims(prcs);
    let (min, max) = get_min_max_raw(prcs);
    let raw_data = get_raw_slice(prcs).to_vec();
    let buffer = ImageBuffer::from_raw(width as u32, height as u32, raw_data).unwrap();
    close_processor(prcs);

    RawData {
        buffer,
        width,
        height,
        min,
        max,
        cfa: CFAPattern::Bayer(0, 0),
    }
}

fn normalize_pixel(val: u16, min: usize, max: usize) -> f32 {
    let val = f32::from(val);
    (val  - min as f32) / (max) as f32
}

fn open_processor(file_path: &str) -> *mut libraw::libraw_data_t {
    let raw_file = CString::new(file_path).expect("no fail");

    unsafe {
        let raw_prcsr = libraw::libraw_init(0);
        libraw::libraw_open_file(raw_prcsr, raw_file.as_ptr());
        libraw::libraw_unpack(raw_prcsr);
        raw_prcsr
    }
}

fn close_processor(prcs: *mut libraw::libraw_data_t) {
    unsafe {
        libraw::libraw_close(prcs);
    }
}

fn get_raw_slice<'a>(prcs: *mut libraw::libraw_data_t) -> &'a [u16] {
    unsafe {
        let (w, h) = get_raw_dims(prcs);
        slice::from_raw_parts((*prcs).rawdata.raw_image, w * h)
    }
}

fn get_raw_dims(prcs: *mut libraw::libraw_data_t) -> (usize, usize) {
    unsafe {
        (libraw::libraw_get_raw_width(prcs) as usize,
        libraw::libraw_get_raw_height(prcs) as usize)
    }
}

fn get_min_max_raw(prcs: *mut libraw::libraw_data_t) -> (u16, u16) {
    unsafe {
        ((*prcs).color.black as u16, (*prcs).color.maximum as u16)
    }
}

// pub fn get_index_col(prcs: *mut libraw::libraw_data_t,
//                      index: usize, width: usize, val: f32) -> Color {
//     let coord = ind2coord(index, width);
//     let colv = unsafe {
//         libraw::libraw_COLOR(prcs, coord.row as i32, coord.col as i32)
//     };
//     Color::new(val, colv)
// }

fn print_raw_content(prcs: *mut libraw::libraw_data_t) {
    unsafe {
        println!("----: {:?}", (*prcs).idata.filters);

        // for r in 0..10 {
        //     println!("col {r},0: {:?}", libraw::libraw_COLOR(prcs, r, 0));
        //     println!("ord {r},0: {:?}", get_coord_col(prcs, (r as usize, 0)));
        //     let ind = coord2ind((r as usize, 0), width);
        //     println!("index {r},0: {:?}", ind);
        //     println!("ind: {:?}", get_ind_col(prcs, ind, width));
        // }
    }
}
