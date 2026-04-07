mod cli;
mod device;
mod writer;
mod progress;

use anyhow::Result;
use cli::{Cli, Commands};
use clap::Parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => {
            let devices = device::list_devices()?;
            if devices.is_empty() {
                println!("No removable devices detected.");
            } else {
                println!("Available removable devices:");
                for d in devices {
                    println!(
                        "- {path}  ({size}, {model})",
                        path = d.path.display(),
                        size = d.human_size,
                        model = d.model.unwrap_or_else(|| "unknown".to_string())
                    );
                }
            }
        }
        Commands::Write { iso, device, force } => {
            device::confirm_device(&device, force)?;
            writer::write_iso_to_device(&iso, &device)?;
        }
    }

    Ok(())
}
