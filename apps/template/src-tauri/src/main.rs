#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use std::fs;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use url::Url;

#[derive(Deserialize, Clone)]
struct AppConfig {
    name: String,
    url: String,
}

fn load_config_from_path(resource_dir: &std::path::Path) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_path = resource_dir.join("config.json");
    let config_str = fs::read_to_string(&config_path)?;
    let config: AppConfig = serde_json::from_str(&config_str)?;
    Ok(config)
}

fn get_base_domain(url_str: &str) -> Option<String> {
    Url::parse(url_str).ok().and_then(|u| u.host_str().map(String::from))
}

#[tauri::command]
async fn show_notification(
    app: tauri::AppHandle,
    title: String,
    body: String,
) -> Result<(), String> {
    use tauri_plugin_notification::NotificationExt;

    app.notification()
        .builder()
        .title(&title)
        .body(&body)
        .show()
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn focus_window(window: tauri::Window) {
    window.set_focus().ok();
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            show_notification,
            focus_window
        ])
        .setup(|app| {
            let resource_dir = app
                .path()
                .resource_dir()
                .expect("failed to get resource dir");

            let config = load_config_from_path(&resource_dir)
                .expect("Failed to load config.json");

            let base_domain = get_base_domain(&config.url);

            if let Some(window) = app.get_webview_window("main") {
                window.close().ok();
            }

            let url = WebviewUrl::External(config.url.parse().expect("Invalid URL"));

            let _window = WebviewWindowBuilder::new(app, "main", url)
                .title(&config.name)
                .inner_size(1200.0, 800.0)
                .min_inner_size(800.0, 600.0)
                .resizable(true)
                .initialization_script(include_str!("../resources/notification-bridge.js"))
                .on_navigation(move |url| {
                    if let Some(ref base) = base_domain {
                        if let Some(nav_domain) = url.host_str() {
                            if nav_domain == base || nav_domain.ends_with(&format!(".{}", base)) {
                                return true;
                            }
                        }
                    }

                    if let Ok(url_string) = url.to_string().parse::<String>() {
                        let _ = open::that(&url_string);
                    }

                    false
                })
                .build()
                .expect("Failed to create window");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
