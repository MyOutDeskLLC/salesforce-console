// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod salesforce;
mod state;
use tauri::{Manager, State};


fn main() {
    let app = tauri::Builder::default()
        .manage(state::AppState(Default::default()))
        .invoke_handler(tauri::generate_handler![
            salesforce::login,
            salesforce::query,
        ])
        .build(tauri::generate_context!()).expect("error while building tauri application");
    let window = app.get_window("main").unwrap();
    window.set_title("Salesforce Console").unwrap();
    app.run(|_app_handle, _run_event| ());
}

