use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "flux-usb-writer",
    version,
    about = "Write ISO images to USB drives with progress."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List removable USB-like block devices
    List,
    /// Write an ISO file to a target device
    Write {
        /// Path to ISO file
        #[arg(short, long)]
        iso: PathBuf,
        /// Target device path (e.g. /dev/sdb or \\\\.\\PhysicalDrive1)
        #[arg(short, long)]
        device: PathBuf,
        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },
}
