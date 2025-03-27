use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List connected USB devices
    Devices,
    /// Flash an image to a device
    Flash {
        /// Path to the image to flash
        image: String,
    },
    /// Run a script
    Script {
        /// Path to the script
        script: String,
    },
    /// Interactive shell
    Shell,
}
