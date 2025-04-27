
mod libs;
mod commands;
mod models;

use log::{error, info, LevelFilter};
use simple_logger::SimpleLogger;
use tauri::{generate_context, generate_handler, Manager, Window};
use windows::Win32::UI::WindowsAndMessaging::{SetWindowLongW, GetWindowLongW, GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_TRANSPARENT};


use crate::commands::app_commands::take_gif;



/*#[cfg(target_os = "windows")]
unsafe fn make_click_through(window: &Window) {
  if let Ok(hwnd) = window.hwnd() {
    let style = GetWindowLongW(hwnd.into_param(), GWL_EXSTYLE) as u32;

    let new_style = style | WS_EX_LAYERED.0 | WS_EX_TRANSPARENT.0;

    SetWindowLongW(hwnd.into_param(), GWL_EXSTYLE, new_style as i32);
  } else {
    eprintln!("Erreur lors de la récupération du handle de la fenêtre.");
  }
}*/

fn main() {
  SimpleLogger::new().with_level(LevelFilter::Info).with_colors(true).init().unwrap();
  info!("-Starting Tauri App-");
  let tauri = tauri::Builder::default()/*.setup(|app| {
    let window = app.get_window("main").unwrap();
    #[cfg(target_os = "windows")]
    unsafe {
      make_click_through(&window);
    }

    Ok(())
  })*/;
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