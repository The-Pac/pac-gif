use std::time::Duration;
use log::info;
use tauri::command;


use crate::libs::gif_creator::{capture_screen,save_as_gif};

#[command]
pub fn take_gif(x: i32, y: i32, width: i32, height: i32) {
    let mut frames = Vec::new();
    for i in 0..60 {
        let frame_data = capture_screen(x, y, width, height);
        frames.push(frame_data);
        std::thread::sleep(Duration::from_millis(100));
        info!("Gif frame {}",i);
    }

    save_as_gif(frames, width as u16, height as u16, "C:/Users/pac/Downloads/capture.gif");
}