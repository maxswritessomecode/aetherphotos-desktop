use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_shell::process::CommandChild;

struct SidecarState(Mutex<Option<CommandChild>>);

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            use tauri_plugin_shell::ShellExt;
            
            // Spawn the sidecar photos-backend
            let sidecar = app.shell().sidecar("photos-backend").unwrap();
            let (_rx, child) = sidecar.spawn().unwrap();
            
            // Store the child handle in app state
            app.manage(SidecarState(Mutex::new(Some(child))));
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app_handle, event| {
            match event {
                tauri::RunEvent::ExitRequested { .. } | tauri::RunEvent::Exit => {
                    let state = app_handle.state::<SidecarState>();
                    let mut child = state.0.lock().unwrap();
                    if let Some(process) = child.take() {
                        let _ = process.kill(); // Terminate the sidecar cleanly
                    }
                }
                _ => {}
            }
        });
}
