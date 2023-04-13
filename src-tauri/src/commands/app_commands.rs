use tauri::command;
use crate::libs::gif_creator::capture_area;

#[command]
pub fn take_gif(x: i32, y: i32, width: i32, height: i32) {
    capture_area(x, y, width, height)
}