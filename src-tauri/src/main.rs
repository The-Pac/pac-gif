// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod libs;
mod commands;

use log::{error, info, LevelFilter};
use simple_logger::SimpleLogger;
use tauri::{generate_context, generate_handler};
use crate::commands::app_commands::take_gif;


fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).with_colors(true).init().unwrap();
    info!("-Starting Tauri App-");
    let tauri = tauri::Builder::default();
    match tauri.invoke_handler(generate_handler![
        take_gif
        ])
        .run(generate_context!()) {
        Ok(_) => {
            info!("-Tauri App start successfully-")
        }
        Err(_) => {
            error!("-Tauri App start failed-")
        }
    }
}
