use std::{mem, ptr};
use std::error::Error;
use std::io::Write;
use winapi::um::winuser::{GetClientRect, GetDC, ReleaseDC, ShowWindow, SW_HIDE, SW_RESTORE};
use winapi::shared::windef::{HDC, HWND, RECT};
use winapi::um::wingdi::{SRCCOPY, StretchDIBits};


pub fn capture_area(x: i32, y: i32, width: i32, height: i32) -> Result<Image, ()> {
    // Obtenir le handle de la fenêtre active
    let window = unsafe { winapi::um::winuser::GetForegroundWindow() };

    // Obtenir le contexte de périphérique de la fenêtre
    let hdc: HDC = unsafe { GetDC(window) };

    // Obtenir les dimensions de la fenêtre
    let mut rect: RECT = unsafe { mem::zeroed() };
    unsafe { GetClientRect(window, &mut rect) };

    // Créer un tampon pour stocker les données de l'image
    let size = (rect.right - rect.left) * (rect.bottom - rect.top) * 4;
    let mut buffer: Vec<u8> = Vec::with_capacity(size as usize);

    // Copier les données de l'image dans le tampon
    let result = unsafe {
        StretchDIBits(
            hdc,
            0,
            0,
            width,
            height,
            0,
            0,
            width,
            height,
            ptr::null_mut(),
            &mut buffer,
            0,
            SRCCOPY,
        )
    };

    // Libérer le contexte de périphérique de la fenêtre
    unsafe { ReleaseDC(window, hdc) };

    // Vérifier si la capture d'écran a réussi
    if result == 0 {
        return Err("Impossible de prendre la capture d'écran".into());
    }

    // Écrire les données de l'image dans un fichier
    let mut file = std::fs::File::create("screenshot.bmp")?;
    let bmp_header = create_bmp_header(&rect)?;
    file.write_all(&bmp_header)?;
    file.write_all(&buffer)?;


    Ok(())
}

fn create_bmp_header(rect: &RECT) -> Result<Vec<u8>, Box<dyn Error>> {
    let file_size = (rect.right - rect.left) * (rect.bottom - rect.top) * 4 + 54;
    let bmp_header = vec![
        b'B', b'M', // Magic number
        (file_size & 0xFF) as u8, // Taille du fichier en octets (1)
        (file_size >> 8 & 0xFF) as u8, // Taille du fichier en octets (2)
        (file_size >> 16 & 0xFF) as u8, // Taille du fichier en octets (3)
        (file_size >> 24 & 0xFF) as u8, // Taille du fichier en octets (4)
        0, 0, 0, 0, // Réservé
        54, 0, 0, 0, // Offset des données de l'image
        40, 0, 0,
        (rect.right - rect.left) as u8, // Taille de l'en-tête BITMAPINFOHEADER (1)
        (rect.right - rect.left >> 8) as u8, // Taille de l'en-tête BITMAPINFOHEADER (2)
        (rect.right - rect.left >> 16) as u8, // Taille de l'en-tête BITMAPINFOHEADER (3)
        (rect.right - rect.left >> 24) as u8, // Taille de l'en-tête BITMAPINFOHEADER (4)
        (rect.bottom - rect.top) as u8, // Largeur de l'image (1)
        (rect.bottom - rect.top >> 8) as u8, // Largeur de l'image (2)
        (rect.bottom - rect.top >> 16) as u8, // Largeur de l'image (3)
        (rect.bottom - rect.top >> 24) as u8, // Largeur de l'image (4)
        1, 0, // Nombre de plans (1)
        0, 0, // Nombre de plans (2)
        24, 0, // Nombre de bits par pixel
        0, 0, 0, 0, // Compression
        0, 0, 0, 0, // Taille des données de l'image
        0, 0, 0, 0, // Résolution horizontale
        0, 0, 0, 0, // Résolution verticale
        0, 0, 0, 0, // Nombre de couleurs dans la palette
        0, 0, 0, 0, // Nombre de couleurs importantes
    ];
    Ok(bmp_header)
}




