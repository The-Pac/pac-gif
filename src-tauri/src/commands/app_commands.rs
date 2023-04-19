use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufWriter;

use log::info;
use tauri::command;

use image::{ImageBuffer, ImageFormat};

use crate::libs::gif_creator::capture_screen;

#[command]
pub fn take_gif(x: i32, y: i32, width: i32, height: i32) {
    match capture_screen(x, y, width, height) {
        Ok(buffer_image) => {
            fs::write(format!("capture.png"), buffer_image.buffer()).expect("failed to create image");
        }
        Err(_) => {}
    };
}