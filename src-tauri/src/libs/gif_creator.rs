use std::mem;
use windows::Win32::Graphics::Gdi::{BI_RGB, BITMAP, BITMAPINFO, CreatedHDC, BITMAPINFOHEADER, DIB_RGB_COLORS, GetDIBits, GetObjectW, RGBQUAD, SelectObject, SetStretchBltMode, SRCCOPY, STRETCH_HALFTONE, StretchBlt};
use crate::{Image};

pub fn capture_area(x: i32, y: i32, width: i32, height: i32) -> Result<Image, ()> {
    let monitor_info_exw = get_monitor_info_exw_from_id(display_id)?;

    let sz_device = monitor_info_exw.szDevice;
    let sz_device_ptr = sz_device.as_ptr();

    let dcw_drop_box = drop_box!(
    CreatedHDC,
    unsafe {
      CreateDCW(
        PCWSTR(sz_device_ptr),
        PCWSTR(sz_device_ptr),
        PCWSTR(ptr::null()),
        None,
      )
    },
    |dcw| unsafe { DeleteDC(dcw) }
  );

    let compatible_dc_drop_box = drop_box!(
    CreatedHDC,
    unsafe { CreateCompatibleDC(*dcw_drop_box) },
    |compatible_dc| unsafe { DeleteDC(compatible_dc) }
  );

    let h_bitmap_drop_box = drop_box!(
    HBITMAP,
    unsafe { CreateCompatibleBitmap(*dcw_drop_box, width, height) },
    |h_bitmap| unsafe { DeleteObject(h_bitmap) }
  );

    unsafe {
        SelectObject(*compatible_dc_drop_box, *h_bitmap_drop_box);
        SetStretchBltMode(*dcw_drop_box, STRETCH_HALFTONE);
    };

    unsafe {
        StretchBlt(
            *compatible_dc_drop_box,
            0,
            0,
            width,
            height,
            *dcw_drop_box,
            x,
            y,
            width,
            height,
            SRCCOPY,
        )
            .ok()?;
    };

    let mut bitmap_info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [RGBQUAD::default(); 1],
    };

    let data = vec![0u8; (width * height) as usize * 4];
    let buf_prt = data.as_ptr() as *mut _;

    let is_success = unsafe {
        GetDIBits(
            *compatible_dc_drop_box,
            *h_bitmap_drop_box,
            0,
            height as u32,
            Some(buf_prt),
            &mut bitmap_info,
            DIB_RGB_COLORS,
        ) == 0
    };

    if is_success {
        Err(())
    }

    let mut bitmap = BITMAP::default();
    let bitmap_ptr = <*mut _>::cast(&mut bitmap);

    unsafe {
        GetObjectW(
            *h_bitmap_drop_box,
            mem::size_of::<BITMAP>() as i32,
            Some(bitmap_ptr),
        );
    }

    let mut chunks: Vec<Vec<u8>> = data
        .chunks(width as usize * 4)
        .map(|x| x.to_vec())
        .collect();

    chunks.reverse();

    let image = Image::from_bgra(
        chunks.concat(),
        bitmap.bmWidth as u32,
        bitmap.bmHeight as u32,
        bitmap.bmWidthBytes as usize,
    )?;

    Ok(image)
}