use crate::progress::WriterProgress;
use anyhow::{Context, Result};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub fn write_iso_to_device(iso_path: &Path, device_path: &Path) -> Result<()> {
    let mut iso = File::open(iso_path)
        .with_context(|| format!("Failed to open ISO: {}", iso_path.display()))?;

    let iso_size = iso.metadata()?.len();

    println!("ISO: {} ({} bytes)", iso_path.display(), iso_size);
    println!("Target device: {}", device_path.display());

    // ★ create() を使わない（truncate される）
    let mut dev = OpenOptions::new()
        .write(true)
        .read(true)
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
