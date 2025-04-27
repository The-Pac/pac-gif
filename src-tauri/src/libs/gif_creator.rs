extern crate image;
extern crate winapi;

use std::path::Path;
use winapi::um::wingdi::*;
use winapi::um::winuser::*;
use winapi::shared::windef::{HWND, HDC, HBITMAP};
use gif::{Encoder, Frame, Repeat};
use std::fs::File;
use std::time::Instant;
use log::info;
use rayon::prelude::*;

const CAPTURE_GIF_DURATION_MILLISECOND: u16 = 10;

pub fn capture_screen(x: i32, y: i32, width: i32, height: i32) -> Vec<u8> {
    unsafe {
        let hwnd: HWND = GetDesktopWindow();
        let hdc_screen: HDC = GetDC(hwnd);
        let hdc_mem: HDC = CreateCompatibleDC(hdc_screen);
        let hbitmap: HBITMAP = CreateCompatibleBitmap(hdc_screen, width, height);
        SelectObject(hdc_mem, hbitmap as _);

        BitBlt(
            hdc_mem,
            0,
            0,
            width,
            height,
            hdc_screen,
            x,
            y,
            SRCCOPY,
        );

        let mut bmp_info: BITMAP = std::mem::zeroed();
        GetObjectW(hbitmap as _, std::mem::size_of::<BITMAP>() as i32, &mut bmp_info as *mut _ as *mut _);

        let width = bmp_info.bmWidth;
        let height = bmp_info.bmHeight;
        let bmp_size = (width * height * 4) as usize;
        let mut bmp_pixels: Vec<u8> = vec![0; bmp_size];

        let bmp_info_header = BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        };

        GetDIBits(
            hdc_screen,
            hbitmap,
            0,
            height as u32,
            bmp_pixels.as_mut_ptr() as *mut _,
            &bmp_info_header as *const _ as *mut _,
            DIB_RGB_COLORS,
        );

        DeleteObject(hbitmap as _);
        DeleteDC(hdc_mem);
        ReleaseDC(hwnd, hdc_screen);

        bmp_pixels
    }
}

pub fn save_as_gif(frames: Vec<Vec<u8>>, width: u16, height: u16, path: &str) {
    let start_time = Instant::now();

    let output_file = File::create(Path::new(path)).unwrap();
    let mut encoder = Encoder::new(output_file, width, height, &[]).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();

    let frame_data_buffers: Vec<Frame> = frames
        .into_par_iter()
        .enumerate()
        .map(|(i, mut frame_data)| {
            let mut frame = Frame::from_rgba_speed(width, height, frame_data.as_mut_slice(), 15);
            info!("Frame {} encoded", i);
            frame.delay = CAPTURE_GIF_DURATION_MILLISECOND;
            frame
        })
        .collect();

    for frame in frame_data_buffers {
        encoder.write_frame(&frame).unwrap();
    }

    let duration = start_time.elapsed();
    println!("GIF encodé en {:?}", duration);
}