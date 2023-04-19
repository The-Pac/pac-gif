use std::mem;
use std::mem::size_of;

use log::error;
use png::{BitDepth, ColorType, Encoder, EncodingError};
use winapi::shared::minwindef::{DWORD, UINT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::wingdi::{BI_RGB, BitBlt, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleBitmap, CreateCompatibleDC, CreateSolidBrush, DeleteDC, DeleteObject, DIB_PAL_COLORS, DIB_RGB_COLORS, DSTINVERT, GetDIBits, RGBQUAD, SelectObject, SRCCOPY, StretchBlt, StretchDIBits};
use winapi::um::winnt::LONG;
use winapi::um::winuser::{GetDC, GetDesktopWindow, ReleaseDC};

use crate::models::image_object::ImageObject;

#[allow(unsafe_code)]
pub fn capture_screen(x: i32, y: i32, width: i32, height: i32) -> Result<ImageObject, String> {
    let hwnd = unsafe { GetDesktopWindow() };
    let hsrc = unsafe { GetDC(hwnd) };
    let hmem = unsafe { CreateCompatibleDC(hsrc) };
    let hbmp = unsafe { CreateCompatibleBitmap(hsrc, width, height) };
    let oldbmp = unsafe { SelectObject(hmem, hbmp as *mut _) };

    let result = unsafe {
        StretchBlt(
            hmem,
            0,
            0,
            width,
            height,
            hsrc,
            x,
            y,
            width,
            height,
            SRCCOPY)
    };
    if result == 0 {
        let error_code = unsafe { GetLastError() };
        return Err(format!("BitBlt failed with error code {}", error_code).into());
    }

    let mut bitmap_info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 24,
            biCompression: BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [RGBQUAD {
            rgbBlue: 0,
            rgbGreen: 0,
            rgbRed: 0,
            rgbReserved: 0,
        }; 1],
    };

    let mut data = vec![0u8; (width * height) as usize * 4];

    let is_fail = unsafe {
        GetDIBits(
            hmem,
            hbmp,
            0,
            height as u32,
            data.as_mut_ptr() as *mut _,
            &mut bitmap_info,
            DIB_RGB_COLORS,
        ) == 0
    };
    if is_fail {
        let error_code = unsafe { GetLastError() };
        return Err(format!("GetDIBits failed with error code {}", error_code).into());
    }

    unsafe {
        SelectObject(hmem, oldbmp as *mut _);
        DeleteObject(hbmp as *mut _);
        DeleteDC(hmem);
        ReleaseDC(hwnd, hsrc);
    }

    let mut chunks: Vec<Vec<u8>> = data
        .chunks(width as usize * 4)
        .map(|x| x.to_vec())
        .collect();

    chunks.reverse();

    match ImageObject::from_bgra(
        chunks.concat(),
        bitmap.bmWidth as u32,
        bitmap.bmHeight as u32,
        bitmap.bmWidthBytes as usize,
    ) {
        Ok(image) => {
            Ok(image)
        }
        Err(error) => {
            error!("{}", error);
            Err("Failed to convert the image".to_string())
        }
    }
}