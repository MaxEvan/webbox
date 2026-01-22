# WebBox — AI-Optimized Development Plan (Updated)

## Project Metadata

```yaml
project_name: WebBox
description: A macOS app that generates standalone web app wrappers from any URL
bundle_identifiers:
  generator: io.sublimesolutions.webbox
  generated_apps: io.sublimesolutions.webbox.<appname>
tech_stack:
  framework: Tauri v2
  frontend: React + TypeScript
  backend: Rust
  styling: Tailwind CSS
target_platform: macOS (initial), cross-platform (future)
```

---

## Project Structure

Create this exact directory structure:

```
webbox/
├── apps/
│   ├── generator/                 # The WebBox generator app (Tauri + React)
│   │   ├── src/                   # React frontend
│   │   │   ├── components/
│   │   │   │   ├── Form.tsx
│   │   │   │   ├── SuccessView.tsx
│   │   │   │   └── ProgressBar.tsx
│   │   │   ├── hooks/
│   │   │   │   └── useGenerateApp.ts
│   │   │   ├── App.tsx
│   │   │   ├── main.tsx
│   │   │   └── index.css
│   │   ├── src-tauri/             # Rust backend
│   │   │   ├── src/
│   │   │   │   ├── main.rs
│   │   │   │   ├── commands/
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── generate.rs
│   │   │   │   │   ├── icon.rs
│   │   │   │   │   └── plist.rs
│   │   │   │   └── lib.rs
│   │   │   ├── resources/
│   │   │   │   └── Template.app/  # Pre-built template (added after Phase 3)
│   │   │   ├── Cargo.toml
│   │   │   └── tauri.conf.json
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   ├── vite.config.ts
│   │   ├── tailwind.config.js
│   │   └── postcss.config.js
│   │
│   └── template/                  # The template app (built once, embedded)
│       ├── src/                   # Minimal frontend (loading screen only)
│       │   ├── index.html
│       │   ├── main.ts
│       │   └── styles.css
│       ├── src-tauri/
│       │   ├── src/
│       │   │   ├── main.rs
│       │   │   └── lib.rs
│       │   ├── resources/
│       │   │   ├── config.json
│       │   │   └── notification-bridge.js
│       │   ├── Cargo.toml
│       │   └── tauri.conf.json
│       └── package.json
│
├── package.json                   # Workspace root
├── pnpm-workspace.yaml
├── .gitignore
└── README.md
```

---

## Phase 1: Project Initialization

### Task 1.1: Create Root Project Structure

**Commands to execute:**

```bash
mkdir -p webbox
cd webbox
```

**Create `package.json`:**

```json
{
  "name": "webbox-monorepo",
  "private": true,
  "scripts": {
    "dev:generator": "pnpm --filter generator dev",
    "dev:template": "pnpm --filter template dev",
    "build:generator": "pnpm --filter generator build",
    "build:template": "pnpm --filter template build"
  }
}
```

**Create `pnpm-workspace.yaml`:**

```yaml
packages:
  - 'apps/*'
  - 'packages/*'
```

**Create `.gitignore`:**

```gitignore
node_modules/
dist/
target/
.DS_Store
*.log
*.app
*.dmg
```

**Success criteria:**
- [ ] Root directory exists with `package.json`, `pnpm-workspace.yaml`, `.gitignore`

---

### Task 1.2: Initialize Template App

**Commands to execute:**

```bash
cd webbox/apps
pnpm create tauri-app template --template vanilla-ts --manager pnpm
cd template
```

**Modify `apps/template/src-tauri/tauri.conf.json`:**

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Template",
  "identifier": "io.sublimesolutions.webbox.template",
  "version": "1.0.0",
  "build": {
    "beforeBuildCommand": "pnpm build",
    "beforeDevCommand": "pnpm dev",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  },
  "app": {
    "withGlobalTauri": true,
    "windows": [],
    "security": {
      "csp": null,
      "dangerousRemoteDomainIpcAccess": [
        {
          "domain": "*",
          "enableTauriAPI": true,
          "windows": ["main"],
          "plugins": ["notification"]
        }
      ]
    }
  },
  "bundle": {
    "active": true,
    "targets": ["app"],
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "resources": ["resources/*"],
    "macOS": {
      "minimumSystemVersion": "10.15"
    }
  }
}
```

**Create `apps/template/src-tauri/resources/config.json`:**

```json
{
  "name": "Template",
  "url": "https://example.com"
}
```

**Success criteria:**
- [ ] Template app directory exists at `apps/template`
- [ ] `tauri.conf.json` has correct bundle identifier `io.sublimesolutions.webbox.template`
- [ ] `config.json` placeholder exists in resources

---

### Task 1.3: Initialize Generator App with Tailwind CSS

**Commands to execute:**

```bash
cd webbox/apps
pnpm create tauri-app generator --template react-ts --manager pnpm
cd generator
pnpm add -D tailwindcss postcss autoprefixer
pnpm dlx tailwindcss init -p
```

**Create `apps/generator/tailwind.config.js`:**

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'app-bg': '#1a1a2e',
        'app-secondary': '#16213e',
        'app-border': '#333',
        'app-primary': '#646cff',
        'app-primary-hover': '#535bf2',
        'app-error': '#ff6b6b',
        'app-success': '#51cf66',
      },
    },
  },
  plugins: [],
}
```

**Create `apps/generator/postcss.config.js`:**

```javascript
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

**Replace `apps/generator/src/index.css`:**

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  body {
    @apply bg-app-bg text-gray-100 min-h-screen antialiased;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
  }
}

@layer components {
  .input-field {
    @apply w-full px-4 py-3 bg-app-secondary border border-app-border rounded-lg 
           text-gray-100 text-base transition-colors duration-200
           placeholder:text-gray-500
           focus:outline-none focus:border-app-primary
           disabled:opacity-60 disabled:cursor-not-allowed;
  }

  .btn {
    @apply px-6 py-3 bg-app-secondary border border-app-border rounded-lg
           text-gray-100 text-base cursor-pointer transition-all duration-200
           hover:bg-app-border
           disabled:opacity-60 disabled:cursor-not-allowed disabled:hover:bg-app-secondary;
  }

  .btn-primary {
    @apply bg-app-primary border-app-primary
           hover:bg-app-primary-hover hover:border-app-primary-hover;
  }

  .btn-secondary {
    @apply bg-transparent;
  }
}
```

**Modify `apps/generator/src-tauri/tauri.conf.json`:**

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "WebBox",
  "identifier": "io.sublimesolutions.webbox",
  "version": "1.0.0",
  "build": {
    "beforeBuildCommand": "pnpm build",
    "beforeDevCommand": "pnpm dev",
    "devUrl": "http://localhost:5173",
    "frontendDist": "../dist"
  },
  "app": {
    "withGlobalTauri": true,
    "windows": [
      {
        "title": "WebBox",
        "width": 600,
        "height": 700,
        "minWidth": 500,
        "minHeight": 600,
        "resizable": true,
        "fullscreen": false,
        "center": true
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "active": true,
    "targets": ["app", "dmg"],
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "resources": ["resources/**/*"],
    "macOS": {
      "minimumSystemVersion": "10.15"
    }
  }
}
```

**Success criteria:**
- [ ] Generator app directory exists at `apps/generator`
- [ ] `tauri.conf.json` has bundle identifier `io.sublimesolutions.webbox`
- [ ] Tailwind CSS configured with `tailwind.config.js` and `postcss.config.js`
- [ ] Custom colors defined in Tailwind config
- [ ] Base styles and component classes in `index.css`

---

## Phase 2: Template App Core Functionality

### Task 2.1: Template Rust Backend — Config Loading and URL Navigation

**Replace `apps/template/src-tauri/src/main.rs`:**

```rust
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
```

**Update `apps/template/src-tauri/Cargo.toml`:**

```toml
[package]
name = "template"
version = "1.0.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-notification = "2"
tauri-plugin-shell = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
url = "2"
open = "5"

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

**Success criteria:**
- [ ] Template loads `config.json` at startup
- [ ] Window title is set from config
- [ ] External links open in default browser
- [ ] Compiles without errors

---

### Task 2.2: Notification Bridge JavaScript

**Create `apps/template/src-tauri/resources/notification-bridge.js`:**

```javascript
(function() {
  const OriginalNotification = window.Notification;
  
  class TauriNotification {
    constructor(title, options = {}) {
      this.title = title;
      this.body = options.body || '';
      this.icon = options.icon || '';
      this.tag = options.tag || '';
      this.onclick = null;
      this.onclose = null;
      this.onerror = null;
      this.onshow = null;
      
      this._show();
    }
    
    async _show() {
      try {
        if (window.__TAURI__?.core?.invoke) {
          await window.__TAURI__.core.invoke('show_notification', {
            title: this.title,
            body: this.body
          });
          if (this.onshow) this.onshow();
        }
      } catch (error) {
        console.error('Failed to show notification:', error);
        if (this.onerror) this.onerror(error);
      }
    }
    
    close() {
      if (this.onclose) this.onclose();
    }
    
    static get permission() {
      return 'granted';
    }
    
    static async requestPermission() {
      return 'granted';
    }
  }
  
  window.Notification = TauriNotification;
  
  console.log('[WebBox] Notification bridge initialized');
})();
```

**Success criteria:**
- [ ] `notification-bridge.js` exists in resources
- [ ] Script overrides `window.Notification`

---

### Task 2.3: Build Template and Verify

**Commands to execute:**

```bash
cd webbox/apps/template
pnpm install
pnpm tauri build --release
```

**Verify built app location:**
```
apps/template/src-tauri/target/release/bundle/macos/Template.app
```

**Success criteria:**
- [ ] `Template.app` builds without errors
- [ ] App loads configured URL when tested manually

---

## Phase 3: Generator App Backend

### Task 3.1: Generator Rust Commands Structure

**Create `apps/generator/src-tauri/src/commands/mod.rs`:**

```rust
pub mod generate;
pub mod icon;
pub mod plist;

pub use generate::*;
pub use icon::*;
pub use plist::*;
```

**Create `apps/generator/src-tauri/src/commands/icon.rs`:**

```rust
use std::path::Path;
use std::process::Command;
use std::fs;

pub fn create_icns(source_image: &Path, output_path: &Path) -> Result<(), String> {
    let temp_iconset = output_path.parent()
        .ok_or("Invalid output path")?
        .join("AppIcon.iconset");
    
    fs::create_dir_all(&temp_iconset)
        .map_err(|e| format!("Failed to create iconset directory: {}", e))?;
    
    let sizes = [
        (16, "icon_16x16.png"),
        (32, "icon_16x16@2x.png"),
        (32, "icon_32x32.png"),
        (64, "icon_32x32@2x.png"),
        (128, "icon_128x128.png"),
        (256, "icon_128x128@2x.png"),
        (256, "icon_256x256.png"),
        (512, "icon_256x256@2x.png"),
        (512, "icon_512x512.png"),
        (1024, "icon_512x512@2x.png"),
    ];
    
    for (size, filename) in sizes.iter() {
        let output_file = temp_iconset.join(filename);
        
        let status = Command::new("sips")
            .args([
                "-z", &size.to_string(), &size.to_string(),
                source_image.to_str().unwrap(),
                "--out", output_file.to_str().unwrap()
            ])
            .status()
            .map_err(|e| format!("Failed to run sips: {}", e))?;
        
        if !status.success() {
            return Err(format!("sips failed for size {}", size));
        }
    }
    
    let status = Command::new("iconutil")
        .args([
            "-c", "icns",
            temp_iconset.to_str().unwrap(),
            "-o", output_path.to_str().unwrap()
        ])
        .status()
        .map_err(|e| format!("Failed to run iconutil: {}", e))?;
    
    if !status.success() {
        return Err("iconutil failed".to_string());
    }
    
    fs::remove_dir_all(&temp_iconset).ok();
    
    Ok(())
}
```

**Create `apps/generator/src-tauri/src/commands/plist.rs`:**

```rust
use std::path::Path;
use std::process::Command;

pub fn update_plist(
    plist_path: &Path,
    app_name: &str,
    bundle_id: &str,
) -> Result<(), String> {
    let plist_str = plist_path.to_str().ok_or("Invalid plist path")?;
    
    let updates = [
        ("CFBundleName", app_name),
        ("CFBundleDisplayName", app_name),
        ("CFBundleIdentifier", bundle_id),
    ];
    
    for (key, value) in updates.iter() {
        let status = Command::new("/usr/libexec/PlistBuddy")
            .args(["-c", &format!("Set :{} {}", key, value), plist_str])
            .status()
            .map_err(|e| format!("Failed to run PlistBuddy: {}", e))?;
        
        if !status.success() {
            return Err(format!("Failed to update {} in plist", key));
        }
    }
    
    Ok(())
}
```

**Create `apps/generator/src-tauri/src/commands/generate.rs`:**

```rust
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};
use tauri::Manager;

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
    
    let resource_dir = app.path().resource_dir()
        .map_err(|e| user_friendly_error(&format!("Failed to get resource dir: {}", e)))?;
    let template_path = resource_dir.join("Template.app");
    
    if !template_path.exists() {
        return Err("Template.app not found. The application may be corrupted.".to_string());
    }
    
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
    let icon_dest = temp_app_path.join("Contents/Resources/AppIcon.icns");
    
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
```

**Update `apps/generator/src-tauri/src/main.rs`:**

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use commands::{generate_app, open_in_finder, launch_app};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            generate_app,
            open_in_finder,
            launch_app
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

**Update `apps/generator/src-tauri/Cargo.toml`:**

```toml
[package]
name = "webbox"
version = "1.0.0"
edition = "2021"

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-dialog = "2"
tauri-plugin-fs = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
uuid = { version = "1", features = ["v4"] }

[features]
default = ["custom-protocol"]
custom-protocol = ["tauri/custom-protocol"]
```

**Success criteria:**
- [ ] All command files created in `src/commands/`
- [ ] `generate_app` command compiles
- [ ] User-friendly error messages implemented

---

### Task 3.2: Embed Template in Generator

**After building the template (Task 2.3), copy it to generator resources:**

```bash
mkdir -p webbox/apps/generator/src-tauri/resources
cp -R webbox/apps/template/src-tauri/target/release/bundle/macos/Template.app \
      webbox/apps/generator/src-tauri/resources/
```

**Success criteria:**
- [ ] `Template.app` exists in `apps/generator/src-tauri/resources/`

---

## Phase 4: Generator App Frontend with Tailwind CSS

### Task 4.1: Install Frontend Dependencies

**Commands:**

```bash
cd webbox/apps/generator
pnpm add @tauri-apps/api @tauri-apps/plugin-dialog @tauri-apps/plugin-fs
```

---

### Task 4.2: Create React Components

**Create `apps/generator/src/components/ProgressBar.tsx`:**

```tsx
interface ProgressBarProps {
  progress: number;
  step: string;
}

export function ProgressBar({ progress, step }: ProgressBarProps) {
  return (
    <div className="flex flex-col gap-2">
      <div className="h-2 bg-app-secondary rounded-full overflow-hidden">
        <div
          className="h-full bg-app-primary transition-all duration-300 ease-out"
          style={{ width: `${progress}%` }}
        />
      </div>
      <span className="text-sm text-gray-400">{step}</span>
    </div>
  );
}
```

**Create `apps/generator/src/components/SuccessView.tsx`:**

```tsx
interface SuccessViewProps {
  appPath: string;
  onOpenFinder: () => void;
  onLaunch: () => void;
  onReset: () => void;
}

export function SuccessView({ appPath, onOpenFinder, onLaunch, onReset }: SuccessViewProps) {
  return (
    <div className="text-center">
      {/* Success Icon */}
      <div className="w-16 h-16 bg-app-success rounded-full flex items-center justify-center text-3xl mx-auto mb-6">
        ✓
      </div>

      <h2 className="text-2xl font-semibold mb-2">App Created!</h2>
      
      <p className="text-gray-400 text-sm break-all mb-6 px-4">
        {appPath}
      </p>

      {/* Action Buttons */}
      <div className="flex gap-3 justify-center mb-4">
        <button onClick={onOpenFinder} className="btn">
          Show in Finder
        </button>
        <button onClick={onLaunch} className="btn btn-primary">
          Launch App
        </button>
      </div>

      <button onClick={onReset} className="btn btn-secondary">
        Create Another
      </button>
    </div>
  );
}
```

**Create `apps/generator/src/components/Form.tsx`:**

```tsx
import { useState, useEffect } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { homeDir } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";
import { ProgressBar } from "./ProgressBar";

interface FormProps {
  onSubmit: (data: { name: string; url: string; iconPath: string; outputDir: string }) => void;
  isGenerating: boolean;
  progress: { step: string; progress: number } | null;
  error: string | null;
}

interface FormErrors {
  name?: string;
  url?: string;
  icon?: string;
  output?: string;
}

export function Form({ onSubmit, isGenerating, progress, error }: FormProps) {
  const [appName, setAppName] = useState("");
  const [url, setUrl] = useState("");
  const [iconPath, setIconPath] = useState("");
  const [outputDir, setOutputDir] = useState("");
  const [errors, setErrors] = useState<FormErrors>({});

  // Set default output directory
  useEffect(() => {
    homeDir().then((home) => {
      if (home) {
        setOutputDir(`${home}Applications`);
      }
    });
  }, []);

  const validateForm = (): boolean => {
    const newErrors: FormErrors = {};

    if (!appName.trim()) {
      newErrors.name = "App name is required";
    } else if (!/^[a-zA-Z0-9\s-]+$/.test(appName)) {
      newErrors.name = "Only letters, numbers, spaces, and hyphens allowed";
    }

    if (!url.trim()) {
      newErrors.url = "URL is required";
    } else {
      try {
        const testUrl = url.startsWith("http") ? url : `https://${url}`;
        const parsed = new URL(testUrl);
        if (!parsed.hostname.includes(".")) {
          newErrors.url = "Please enter a valid URL";
        }
      } catch {
        newErrors.url = "Please enter a valid URL";
      }
    }

    if (!iconPath) {
      newErrors.icon = "Please select an icon image";
    }

    if (!outputDir) {
      newErrors.output = "Please select an output directory";
    }

    setErrors(newErrors);
    return Object.keys(newErrors).length === 0;
  };

  const handleSelectIcon = async () => {
    const selected = await open({
      multiple: false,
      filters: [
        {
          name: "Images",
          extensions: ["png", "jpg", "jpeg", "webp"],
        },
      ],
    });

    if (selected && typeof selected === "string") {
      setIconPath(selected);
      setErrors((prev) => ({ ...prev, icon: undefined }));
    }
  };

  const handleSelectOutput = async () => {
    const selected = await open({
      directory: true,
      multiple: false,
      defaultPath: outputDir,
    });

    if (selected && typeof selected === "string") {
      setOutputDir(selected);
      setErrors((prev) => ({ ...prev, output: undefined }));
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!validateForm()) return;

    let normalizedUrl = url.trim();
    if (!normalizedUrl.startsWith("http://") && !normalizedUrl.startsWith("https://")) {
      normalizedUrl = `https://${normalizedUrl}`;
    }

    onSubmit({
      name: appName.trim(),
      url: normalizedUrl,
      iconPath,
      outputDir,
    });
  };

  const iconPreviewSrc = iconPath ? convertFileSrc(iconPath) : null;

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-6">
      {/* App Name */}
      <div className="flex flex-col gap-2">
        <label htmlFor="appName" className="text-sm font-medium">
          App Name
        </label>
        <input
          id="appName"
          type="text"
          value={appName}
          onChange={(e) => {
            setAppName(e.target.value);
            setErrors((prev) => ({ ...prev, name: undefined }));
          }}
          placeholder="e.g., Notion"
          disabled={isGenerating}
          className="input-field"
        />
        {errors.name && <span className="text-app-error text-sm">{errors.name}</span>}
      </div>

      {/* URL */}
      <div className="flex flex-col gap-2">
        <label htmlFor="url" className="text-sm font-medium">
          Website URL
        </label>
        <input
          id="url"
          type="text"
          value={url}
          onChange={(e) => {
            setUrl(e.target.value);
            setErrors((prev) => ({ ...prev, url: undefined }));
          }}
          placeholder="e.g., notion.so"
          disabled={isGenerating}
          className="input-field"
        />
        {errors.url && <span className="text-app-error text-sm">{errors.url}</span>}
      </div>

      {/* Icon */}
      <div className="flex flex-col gap-2">
        <label className="text-sm font-medium">App Icon</label>
        <div className="flex items-center gap-3">
          <button
            type="button"
            onClick={handleSelectIcon}
            disabled={isGenerating}
            className="btn"
          >
            {iconPath ? "Change Icon" : "Select Icon"}
          </button>
          {iconPreviewSrc && (
            <div className="flex items-center gap-2">
              <img
                src={iconPreviewSrc}
                alt="Icon preview"
                className="w-10 h-10 rounded-lg object-cover border border-app-border"
              />
              <span className="text-gray-400 text-sm truncate max-w-[150px]">
                {iconPath.split("/").pop()}
              </span>
            </div>
          )}
        </div>
        {errors.icon && <span className="text-app-error text-sm">{errors.icon}</span>}
      </div>

      {/* Output Directory */}
      <div className="flex flex-col gap-2">
        <label className="text-sm font-medium">Output Location</label>
        <div className="flex items-center gap-3">
          <button
            type="button"
            onClick={handleSelectOutput}
            disabled={isGenerating}
            className="btn"
          >
            Change
          </button>
          <span className="text-gray-400 text-sm truncate flex-1">{outputDir}</span>
        </div>
        {errors.output && <span className="text-app-error text-sm">{errors.output}</span>}
      </div>

      {/* Progress */}
      {isGenerating && progress && (
        <ProgressBar progress={progress.progress} step={progress.step} />
      )}

      {/* Error Message */}
      {error && (
        <div className="bg-app-error/10 border border-app-error rounded-lg p-4 text-sm">
          <strong>Error:</strong> {error}
        </div>
      )}

      {/* Submit Button */}
      <button
        type="submit"
        disabled={isGenerating}
        className="btn btn-primary w-full mt-2"
      >
        {isGenerating ? "Creating..." : "Create App"}
      </button>
    </form>
  );
}
```

**Create `apps/generator/src/hooks/useGenerateApp.ts`:**

```typescript
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

interface GenerateProgress {
  step: string;
  progress: number;
}

interface GenerateResult {
  success: boolean;
  path?: string;
  error?: string;
}

export function useGenerateApp() {
  const [isGenerating, setIsGenerating] = useState(false);
  const [progress, setProgress] = useState<GenerateProgress | null>(null);
  const [result, setResult] = useState<GenerateResult | null>(null);

  useEffect(() => {
    const unlisten = listen<GenerateProgress>("generate-progress", (event) => {
      setProgress(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  const generate = async (data: {
    name: string;
    url: string;
    iconPath: string;
    outputDir: string;
  }) => {
    setIsGenerating(true);
    setResult(null);
    setProgress({ step: "Starting...", progress: 0 });

    try {
      const outputPath = await invoke<string>("generate_app", {
        request: {
          name: data.name,
          url: data.url,
          icon_path: data.iconPath,
          output_dir: data.outputDir,
        },
      });

      setResult({ success: true, path: outputPath });
    } catch (error) {
      setResult({ success: false, error: String(error) });
    } finally {
      setIsGenerating(false);
      setProgress(null);
    }
  };

  const reset = () => {
    setResult(null);
    setProgress(null);
  };

  const openInFinder = async () => {
    if (result?.path) {
      await invoke("open_in_finder", { path: result.path });
    }
  };

  const launchApp = async () => {
    if (result?.path) {
      await invoke("launch_app", { path: result.path });
    }
  };

  return {
    isGenerating,
    progress,
    result,
    generate,
    reset,
    openInFinder,
    launchApp,
  };
}
```

**Replace `apps/generator/src/App.tsx`:**

```tsx
import { Form } from "./components/Form";
import { SuccessView } from "./components/SuccessView";
import { useGenerateApp } from "./hooks/useGenerateApp";

function App() {
  const {
    isGenerating,
    progress,
    result,
    generate,
    reset,
    openInFinder,
    launchApp,
  } = useGenerateApp();

  return (
    <div className="max-w-lg mx-auto px-6 py-10">
      {/* Header */}
      <header className="text-center mb-10">
        <h1 className="text-3xl font-bold mb-2">WebBox</h1>
        <p className="text-gray-400">Turn any website into a native Mac app</p>
      </header>

      {/* Main Content */}
      {result?.success ? (
        <SuccessView
          appPath={result.path!}
          onOpenFinder={openInFinder}
          onLaunch={launchApp}
          onReset={reset}
        />
      ) : (
        <Form
          onSubmit={generate}
          isGenerating={isGenerating}
          progress={progress}
          error={result?.error || null}
        />
      )}
    </div>
  );
}

export default App;
```

**Update `apps/generator/src/main.tsx`:**

```tsx
import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
```

**Success criteria:**
- [ ] All components use Tailwind CSS classes
- [ ] Custom component classes defined in `index.css`
- [ ] Form validation works
- [ ] Progress bar displays during generation
- [ ] Success view shows after completion

---

### Task 4.3: Configure Tauri Capabilities

**Create `apps/generator/src-tauri/capabilities/default.json`:**

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Default capabilities for WebBox",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "core:event:default",
    "core:event:allow-emit",
    "core:event:allow-listen",
    "dialog:default",
    "dialog:allow-open",
    "fs:default",
    "fs:allow-read",
    "fs:allow-write",
    "fs:allow-exists",
    "fs:allow-mkdir",
    "fs:allow-remove",
    "fs:allow-rename",
    "fs:allow-copy-file",
    {
      "identifier": "fs:scope",
      "allow": ["$HOME/**", "$TEMP/**", "$RESOURCE/**"]
    }
  ]
}
```

**Success criteria:**
- [ ] Capabilities file created
- [ ] File system permissions configured

---

## Phase 5: Integration Testing & Build

### Task 5.1: Test Generator in Development Mode

**Commands:**

```bash
cd webbox/apps/generator
pnpm tauri dev
```

**Manual test checklist:**
- [ ] App opens without errors
- [ ] Tailwind styles render correctly
- [ ] Can enter app name
- [ ] Can enter URL
- [ ] Can select icon file
- [ ] Icon preview displays
- [ ] Can change output directory
- [ ] Form validation shows errors
- [ ] "Create App" generates an app
- [ ] Progress indicator shows
- [ ] Success view displays
- [ ] "Show in Finder" works
- [ ] "Launch App" works
- [ ] Generated app loads correct URL
- [ ] Generated app has correct icon and title

---

### Task 5.2: Build Generator for Distribution

**Commands:**

```bash
cd webbox/apps/generator
pnpm tauri build --release
```

**Output location:**
```
apps/generator/src-tauri/target/release/bundle/macos/WebBox.app
apps/generator/src-tauri/target/release/bundle/dmg/WebBox_1.0.0_aarch64.dmg
```

**Success criteria:**
- [ ] `WebBox.app` builds without errors
- [ ] `Template.app` embedded in `WebBox.app/Contents/Resources/`
- [ ] App runs on clean Mac without dev tools

---

## Phase 6: Polish

### Task 6.1: Add WebBox App Icon

**Create or source a WebBox app icon and place in:**
```
apps/generator/src-tauri/icons/
├── 32x32.png
├── 128x128.png
├── 128x128@2x.png
├── icon.icns
└── icon.ico
```

**Success criteria:**
- [ ] WebBox has distinctive icon in Dock, Finder, Spotlight

---

## Completion Checklist

### Phase 1: Project Setup
- [ ] Monorepo structure created
- [ ] Template app initialized
- [ ] Generator app initialized with Tailwind CSS

### Phase 2: Template App
- [ ] Loads URL from config.json
- [ ] Window title from config
- [ ] External links open in browser
- [ ] Notification bridge works
- [ ] Built as release

### Phase 3: Generator Backend
- [ ] `generate_app` command works
- [ ] Icon conversion works
- [ ] Plist modification works
- [ ] Code signing works
- [ ] Template.app embedded

### Phase 4: Generator Frontend
- [ ] Tailwind CSS configured and working
- [ ] Form component with validation
- [ ] Progress bar component
- [ ] Success view component
- [ ] Custom hook for generation logic

### Phase 5: Integration
- [ ] End-to-end generation works
- [ ] Release build works

### Phase 6: Polish
- [ ] WebBox has its own icon
- [ ] DMG installer created

---

## Notes for AI Execution

1. **Execute tasks sequentially** — each phase depends on the previous
2. **Test after each major task** — run `pnpm tauri dev` to verify
3. **File paths are exact** — use the specified paths
4. **Tailwind CSS is required** — all styling uses Tailwind utility classes
5. **Custom classes in `index.css`** — use `@layer components` for reusable styles
6. **macOS tools are assumed** — `sips`, `iconutil`, `PlistBuddy`, `codesign` are built-in
7. **Bundle identifier is `io.sublimesolutions.webbox`** — do not change
8. **Generated app identifiers follow pattern** — `io.sublimesolutions.webbox.<appname>`
9. **No external dependencies for users** — everything needed is bundled
10. **Tauri v2 syntax** — use v2 APIs, not v1