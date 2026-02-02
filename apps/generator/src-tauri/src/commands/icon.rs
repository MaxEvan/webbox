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
