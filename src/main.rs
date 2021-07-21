#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

pub mod ui;
pub mod deloop;
#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use eframe::egui::Vec2;

lazy_static! {
    static ref BASE_PATH: String =
        "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Loop Hero\\".to_string();
    static ref INI_FILE: String = "variables.ini".to_string();
}
fn main() -> Result<()> {
 let app = ui::App::default();
 let mut native_options = eframe::NativeOptions::default();
// native_options.decorated = false;
native_options.drag_and_drop_support = false;
native_options.initial_window_size = Some(Vec2::new(500.0,600.0));
native_options.resizable = false;
 eframe::run_native(Box::new(app), native_options);
}

