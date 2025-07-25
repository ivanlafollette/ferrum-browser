#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

/// A simple command exposed to the Svelte frontend.  This is a placeholder for
/// future AI‑powered interactions.  It currently returns a friendly greeting.
#[tauri::command]
fn greet(name: String) -> String {
    format!("Hello, {}! Welcome to Ferrum.", name)
}

fn main() {
    // Build the Tauri application.  The builder pattern lets us register commands
    // that can be invoked from the frontend and customise other options.
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running Ferrum app");
}
