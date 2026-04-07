use anyhow::{anyhow, Context, Result};
use humansize::{format_size, BINARY};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub human_size: String,
    pub model: Option<String>,
}

#[cfg(target_os = "linux")]
pub fn list_devices() -> Result<Vec<DeviceInfo>> {
    let mut result = Vec::new();

    for entry in fs::read_dir("/sys/block")? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();

        // sdX, nvmeXnY などを対象にするが、ここでは sdX 系を優先
        if !name.starts_with("sd") {
            continue;
        }

        let removable = fs::read_to_string(entry.path().join("removable"))
            .unwrap_or_else(|_| "0".to_string())
            .trim()
            .to_string();

        if removable != "1" {
            continue;
        }

        let size_sectors = fs::read_to_string(entry.path().join("size"))
            .unwrap_or_else(|_| "0".to_string())
            .trim()
            .parse::<u64>()
            .unwrap_or(0);

        let size_bytes = size_sectors * 512;
        let human_size = format_size(size_bytes, BINARY);

        let dev_path = PathBuf::from(format!("/dev/{}", name));

        let model = fs::read_to_string(entry.path().join("device/model"))
            .ok()
            .map(|s| s.trim().to_string());

        result.push(DeviceInfo {
            path: dev_path,
            size_bytes,
            human_size,
            model,
        });
    }

    Ok(result)
}

#[cfg(target_os = "windows")]
pub fn list_devices() -> Result<Vec<DeviceInfo>> {
    // 本気でやるなら WinAPI で列挙するが、
    // ここでは「手動指定前提」のため空リストを返す。
    Ok(Vec::new())
}

pub fn confirm_device(device: &Path, force: bool) -> Result<()> {
    if !device.exists() {
        return Err(anyhow!("Device path does not exist: {}", device.display()));
    }

    if force {
        return Ok(());
    }

    println!("WARNING: This will ERASE all data on: {}", device.display());
    println!("Type 'YES' to continue:");

    use std::io::{self, Read};
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if input.trim() == "YES" {
        Ok(())
    } else {
        Err(anyhow!("Aborted by user."))
    }
}
