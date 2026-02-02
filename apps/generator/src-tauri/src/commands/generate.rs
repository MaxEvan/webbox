use std::fs;
use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};
use tauri::{Manager, Emitter};

use super::icon::create_icns;
use super::plist::update_plist;

#[derive(Deserialize)]
pub struct GenerateAppRequest {
    pub name: String,
    pub url: String,
    pub icon_path: String,
    pub output_dir: String,
}

#[derive(Serialize, Clone)]
pub struct GenerateProgress {
    pub step: String,
    pub progress: u8,
}

fn user_friendly_error(error: &str) -> String {
    if error.contains("permission denied") || error.contains("Permission denied") {
        return "Permission denied. Try choosing a different output location.".to_string();
    }
    if error.contains("No such file") {
        return "The selected icon file could not be found.".to_string();
    }
    if error.contains("sips") {
        return "Failed to process the icon image. Please try a different image (PNG recommended).".to_string();
    }
    if error.contains("iconutil") {
        return "Failed to create app icon. Please try a different image.".to_string();
    }
    error.to_string()
}

#[tauri::command]
pub async fn generate_app(
    app: tauri::AppHandle,
    request: GenerateAppRequest,
) -> Result<String, String> {
    let window = app.get_webview_window("main").ok_or("No main window")?;

    let emit_progress = |step: &str, progress: u8| {
        window.emit("generate-progress", GenerateProgress {
            step: step.to_string(),
            progress,
        }).ok();
    };

    emit_progress("Preparing...", 10);

    // Try to find Template.app in multiple locations:
    // 1. Production: resource_dir/Template.app
    // 2. Development: src-tauri/resources/Template.app
    let template_path = {
        let resource_dir = app.path().resource_dir()
            .map_err(|e| user_friendly_error(&format!("Failed to get resource dir: {}", e)))?;

        // Production path (bundled app)
        let prod_path = resource_dir.join("Template.app");
        if prod_path.exists() {
            prod_path
        } else {
            // Development path - look for resources relative to the executable
            let dev_path = resource_dir.join("resources/Template.app");
            if dev_path.exists() {
                dev_path
            } else {
                // Try relative to CARGO_MANIFEST_DIR for dev builds
                let manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                    .join("resources/Template.app");
                if manifest_path.exists() {
                    manifest_path
                } else {
                    return Err(format!(
                        "Template.app not found. Checked:\n- {}\n- {}\n- {}",
                        prod_path.display(),
                        dev_path.display(),
                        manifest_path.display()
                    ));
                }
            }
        }
    };

    let safe_name: String = request.name
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-')
        .collect();
    let safe_name = safe_name.trim().to_string();

    if safe_name.is_empty() {
        return Err("Invalid app name. Please use letters, numbers, spaces, or hyphens.".to_string());
    }

    let temp_dir = std::env::temp_dir().join(format!("webbox-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&temp_dir)
        .map_err(|e| user_friendly_error(&format!("Failed to create temp directory: {}", e)))?;

    let app_name = format!("{}.app", safe_name);
    let temp_app_path = temp_dir.join(&app_name);

    emit_progress("Copying template...", 20);

    let status = Command::new("cp")
        .args(["-R", template_path.to_str().unwrap(), temp_app_path.to_str().unwrap()])
        .status()
        .map_err(|e| user_friendly_error(&format!("Failed to copy template: {}", e)))?;

    if !status.success() {
        return Err("Failed to copy template. Please try again.".to_string());
    }

    emit_progress("Processing icon...", 40);

    let icon_source = PathBuf::from(&request.icon_path);
    let icon_dest = temp_app_path.join("Contents/Resources/icon.icns");

    create_icns(&icon_source, &icon_dest).map_err(|e| user_friendly_error(&e))?;

    emit_progress("Writing configuration...", 60);

    let config_path = temp_app_path.join("Contents/Resources/config.json");
    let config = serde_json::json!({
        "name": request.name,
        "url": request.url
    });
    fs::write(&config_path, serde_json::to_string_pretty(&config).unwrap())
        .map_err(|e| user_friendly_error(&format!("Failed to write config: {}", e)))?;

    emit_progress("Updating app metadata...", 70);

    let plist_path = temp_app_path.join("Contents/Info.plist");
    let bundle_id = format!(
        "io.sublimesolutions.webbox.{}",
        safe_name.to_lowercase().replace(" ", "-")
    );
    update_plist(&plist_path, &safe_name, &bundle_id).map_err(|e| user_friendly_error(&e))?;

    emit_progress("Signing app...", 80);

    let status = Command::new("codesign")
        .args(["--force", "--deep", "--sign", "-", temp_app_path.to_str().unwrap()])
        .status()
        .map_err(|e| user_friendly_error(&format!("Failed to sign app: {}", e)))?;

    if !status.success() {
        return Err("Failed to sign app. Please try again.".to_string());
    }

    emit_progress("Moving to destination...", 90);

    let output_path = PathBuf::from(&request.output_dir).join(&app_name);

    if output_path.exists() {
        fs::remove_dir_all(&output_path)
            .map_err(|e| user_friendly_error(&format!("Failed to remove existing app: {}", e)))?;
    }

    fs::rename(&temp_app_path, &output_path)
        .or_else(|_| {
            // If rename fails (cross-device), try copy and delete
            let status = Command::new("cp")
                .args(["-R", temp_app_path.to_str().unwrap(), output_path.to_str().unwrap()])
                .status();

            if status.is_ok() && status.unwrap().success() {
                fs::remove_dir_all(&temp_app_path).ok();
                Ok(())
            } else {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Copy failed"))
            }
        })
        .map_err(|e| user_friendly_error(&format!("Failed to move app to destination: {}", e)))?;

    fs::remove_dir_all(&temp_dir).ok();

    emit_progress("Done!", 100);

    Ok(output_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn open_in_finder(path: String) -> Result<(), String> {
    Command::new("open")
        .args(["-R", &path])
        .status()
        .map_err(|e| format!("Failed to open Finder: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn launch_app(path: String) -> Result<(), String> {
    Command::new("open")
        .arg(&path)
        .status()
        .map_err(|e| format!("Failed to launch app: {}", e))?;
    Ok(())
}
