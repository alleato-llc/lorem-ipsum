// Prevents an extra console window on Windows in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lorem_core::{GeneratedText, GeneratorOptions, ThemeInfo};

#[tauri::command]
fn generate(options: GeneratorOptions) -> GeneratedText {
    lorem_core::generate(&options)
}

#[tauri::command]
fn themes() -> Vec<ThemeInfo> {
    lorem_core::list_themes()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![generate, themes])
        .run(tauri::generate_context!())
        .expect("error while running lorem-gui");
}
