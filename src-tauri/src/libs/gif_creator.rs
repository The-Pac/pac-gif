use std::ffi::c_void;
use std::io::Error;
use std::mem;
use std::mem::size_of;
use std::ptr;
use std::ptr::null_mut;
use winapi::shared::windef::{HBITMAP, HGDIOBJ};
use winapi::um::wingdi::{BI_RGB, BitBlt, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, DIB_RGB_COLORS, GetDeviceCaps, GetDIBits, SelectObject, SRCCOPY};
use winapi::um::winuser::{GetDC, GetDesktopWindow, GetWindowDC, ReleaseDC};

pub fn capture_screen(x: i32, y: i32, width: i32, height: i32) -> Result<Vec<u8>, Error> {
    let mut bmp_info: BITMAPINFO = unsafe { mem::zeroed() };
    bmp_info.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
    bmp_info.bmiHeader.biWidth = width;
    bmp_info.bmiHeader.biHeight = height;
    bmp_info.bmiHeader.biPlanes = 1;
    bmp_info.bmiHeader.biBitCount = 32;
    bmp_info.bmiHeader.biCompression = BI_RGB;

    let hdc_screen = unsafe { GetDC(null_mut()) };
    let hdc = unsafe { CreateCompatibleDC(hdc_screen) };
    let hbitmap = unsafe { CreateCompatibleBitmap(hdc_screen, width, height) };
    let hbitmap_old = unsafe { SelectObject(hdc, hbitmap as *mut _) };

    unsafe {
        BitBlt(hdc, 0, 0, width, height, hdc_screen, x, y, SRCCOPY);
    }

    let mut buffer: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);
    let result = unsafe {
        let buffer_ptr = buffer.as_mut_ptr() as *mut c_void;
        let hbitmap_obj = hbitmap as HBITMAP;
        let ret = GetDIBits(hdc, hbitmap_obj, 0, height as u32, buffer_ptr, &mut bmp_info, DIB_RGB_COLORS);
        if ret == 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    };
    if let Err(e) = result {
        return Err(e);
    }

    let stride = (width * 4) as usize;
    buffer.resize(stride * height as usize, 0);

    unsafe {
        SelectObject(hdc, hbitmap_old as *mut _);
        DeleteObject(hbitmap as HGDIOBJ);
        DeleteDC(hdc);
        ReleaseDC(null_mut(), hdc_screen);
    }

    Ok(buffer)
}
