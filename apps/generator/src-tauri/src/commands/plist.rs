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
