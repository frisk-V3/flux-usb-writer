use crate::progress::WriterProgress;
use anyhow::{Context, Result};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn write_iso_to_device(iso_path: &Path, device_path: &Path) -> Result<()> {
    let mut iso = File::open(iso_path)
        .with_context(|| format!("Failed to open ISO: {}", iso_path.display()))?;

    let iso_size = iso
        .metadata()
        .with_context(|| "Failed to get ISO metadata")?
        .len();

    println!("ISO: {} ({} bytes)", iso_path.display(), iso_size);
    println!("Target device: {}", device_path.display());

    // Linux: /dev/sdX をそのまま開く
    // Windows: \\.\PhysicalDriveN をそのまま開く（PathBuf 経由で渡される前提）
    let mut dev = File::options()
        .write(true)
        .open(device_path)
        .with_context(|| format!("Failed to open device: {}", device_path.display()))?;

    let progress = WriterProgress::new(iso_size);

    const BUF_SIZE: usize = 1024 * 1024;
    let mut buf = vec![0u8; BUF_SIZE];
    let mut written: u64 = 0;

    loop {
        let n = iso.read(&mut buf)?;
        if n == 0 {
            break;
        }
        dev.write_all(&buf[..n])?;
        written += n as u64;
        progress.inc(n as u64);
    }

    dev.flush()?;
    progress.finish();

    println!("Wrote {} bytes to {}", written, device_path.display());
    Ok(())
}
