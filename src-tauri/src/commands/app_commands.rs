use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use image::ColorType::Rgb8;
use image::{ImageBuffer, ImageFormat};
use log::info;
use tauri::command;
use crate::libs::gif_creator::capture_screen;

fn bytes_to_png(bytes: &[u8], width: u32, height: u32) -> Result<(), Box<dyn Error>> {
    let img_buf = ImageBuffer::<image::Rgba<u8>, _>::from_vec(width, height, bytes.to_vec())
        .ok_or("Failed to create ImageBuffer")?;

    let file = File::create("image.png")?;
    let ref mut buffer_writer = BufWriter::new(file);

    img_buf.write_to(buffer_writer, ImageFormat::Png)?;

    Ok(())
}

#[command]
pub fn take_gif(x: i32, y: i32, width: i32, height: i32) {
    match capture_screen(x, y, width, height) {
        Ok(buffer_image) => {
            info!("{:?}",buffer_image);
            match bytes_to_png(&buffer_image, width as u32, height as u32) {
                Ok(()) => {
                    println!("Image saved as PNG");
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
            info!("ok")
        }
        Err(_) => {}
    };
}