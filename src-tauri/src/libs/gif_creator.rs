use std::mem;
use std::mem::size_of;

use winapi::shared::minwindef::{DWORD, UINT};
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::wingdi::{BI_RGB, BitBlt, BitBlt as GdiBitBlt, BITMAPINFO, BITMAPINFOHEADER, CreateCompatibleBitmap, CreateCompatibleDC, CreateSolidBrush, DeleteDC, DeleteObject, DIB_PAL_COLORS, DIB_RGB_COLORS, GetDIBits, SelectObject, SRCCOPY};
use winapi::um::winnt::LONG;
use winapi::um::winuser::{GetDC, GetDesktopWindow, ReleaseDC};

pub fn capture_screen(x: i32, y: i32, width: i32, height: i32) -> Result<Vec<u8>, String> {
    let hwnd = unsafe { GetDesktopWindow() };
    let hsrc = unsafe { GetDC(hwnd) };
    let hmem = unsafe { CreateCompatibleDC(hsrc) };
    let hbmp = unsafe { CreateCompatibleBitmap(hsrc, width, height) };
    let hbrush = unsafe { CreateSolidBrush(0xFFFFFF) };
    let oldbmp = unsafe { SelectObject(hmem, hbmp as *mut _) };
    let oldbrush = unsafe { SelectObject(hmem, hbrush as *mut _) };

    let result = unsafe { BitBlt(hmem, 0, 0, width, height, hsrc, x, y, SRCCOPY) };
    if result == 0 {
        let error_code = unsafe { GetLastError() };
        return Err(format!("BitBlt failed with error code {}", error_code).into());
    }

    let mut bmp_info: BITMAPINFO = unsafe { mem::zeroed() };
    bmp_info.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as DWORD;
    bmp_info.bmiHeader.biWidth = width as LONG;
    bmp_info.bmiHeader.biHeight = -(height as LONG);
    bmp_info.bmiHeader.biPlanes = 1;
    bmp_info.bmiHeader.biBitCount = 24;
    bmp_info.bmiHeader.biCompression = BI_RGB;

    let bmp_size = ((width * 3 + 3) / 4) * 4 * height;
    let mut bmp_data = vec![0; bmp_size as usize];

    let gdi_result = unsafe { GdiBitBlt(hmem, 0, 0, width as i32, height as i32, hsrc, x as i32, y as i32, SRCCOPY) };
    if gdi_result == 0 {
        let error_code = unsafe { GetLastError() };
        return Err(format!("GdiBitBlt failed with error code {}", error_code));
    }
    let result = unsafe {
        GetDIBits(
            hmem,
            hbmp,
            0,
            height as UINT,
            bmp_data.as_mut_ptr() as *mut _,
            &mut bmp_info,
            DIB_PAL_COLORS,
        )
    };
    if result == 0 {
        let error_code = unsafe { GetLastError() };
        return Err(format!("GetDIBits failed with error code {}", error_code).into());
    }

    unsafe {
        SelectObject(hmem, oldbrush as *mut _);
        SelectObject(hmem, oldbmp as *mut _);
        DeleteObject(hbrush as *mut _);
        DeleteObject(hbmp as *mut _);
        DeleteDC(hmem);
        ReleaseDC(hwnd, hsrc);
    }

    Ok(bmp_data)
}
