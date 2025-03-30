use crate::script::Script;
use clap::{Parser, Subcommand};
use colored::Colorize;
use inquire::{Confirm, Text};
use serde::Deserialize;

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

/// Manifest file structure (name, hash)
#[derive(Deserialize, Debug)]
struct Component {
    name: String,
}
#[derive(Deserialize, Debug)]
struct Manifest {
    kernel: Component,
    rootfs: Component,
    bootloader: Component,
    dtb: Component,
    initramfs: Component,
    script: Component,
}

pub fn handle_devices() {
    uuu_rs::devices::print_devices();
}

pub fn handle_flash(image: &str) {
    // Checck if the file exists
    if !std::path::Path::new(image).exists() {
        println!("{} does not exist.", image.red().bold());
        return;
    }

    // Setup prompts
    let ans = Confirm::new("Is the device in SERIAL mode?")
        .with_default(true)
        .with_help_message(
            "Make sure to connect to the device's HOST USB port. Learn more [mecha.so/...]",
        )
        .prompt()
        .unwrap();
    if !ans {
        println!(
            "{}",
            "Please connect the device in SERIAL mode and try again.".cyan()
        );
        return;
    }

    println!("Searching for device...");
    uuu_rs::devices::print_devices();
    let devices = uuu_rs::devices::get_devices();
    if devices.is_empty() {
        println!("{}", "Check the device connection and try again.".red());
        return;
    }

    // Setup a temporary directory
    let temp_dir = tempfile::tempdir().unwrap_or_else(|e| {
        println!(
            "{}",
            format!("Failed to create temporary directory: {}", e)
                .red()
                .bold()
        );
        std::process::exit(1);
    });
    let temp_path = temp_dir.path();

    // Extract the package to the temporary directory
    println!("{}", "Extracting package...".green());
    let file = std::fs::File::open(image).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();
    archive.extract(temp_path).unwrap();

    // Check if the package contains the necessary files
    let manifest = temp_path.join("manifest.yml");
    if !manifest.exists() {
        println!(
            "{}",
            "The package does not contain a manifest.yml file.".red()
        );
        return;
    }

    // Verify the manifest
    let f = std::fs::File::open(&manifest).unwrap();
    let manifest: Manifest = serde_yml::from_reader(f).unwrap();
    // TODO: Verify the hashes of the files

    // Flash the image
    println!("{}", "Flashing image...".green());

    let current_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_path).unwrap();

    let script = Script::new(&manifest.script.name);
    let script_status = script.run();
    match script_status {
        Ok(()) => {
            println!("Script executed successfully");
        }
        Err(e) => {
            println!("Error: {}", e);
            println!("Script execution aborted.");
        }
    }

    std::env::set_current_dir(current_dir).unwrap();
    temp_dir.close().unwrap();
}

pub fn handle_script(script: &str) {
    // Checck if the file exists
    if !std::path::Path::new(script).exists() {
        println!("{} does not exist.", script);
        return;
    }

    // Setup prompts
    let ans = Confirm::new("Is the device in SERIAL mode?")
        .with_default(true)
        .with_help_message(
            "Make sure to connect to the device's HOST USB port. Learn more [mecha.so/...]",
        )
        .prompt()
        .unwrap();
    if !ans {
        println!("Please connect the device in SERIAL mode and try again.");
        return;
    }

    println!("Searching for device...");
    uuu_rs::devices::print_devices();
    let devices = uuu_rs::devices::get_devices();
    if devices.is_empty() {
        return;
    }

    let ans = Confirm::new(format!("Do you want to run the script: {}", script).as_str())
        .with_default(true)
        .prompt()
        .unwrap();
    if !ans {
        println!("Aborted.");
        return;
    }

    println!("Running script...");

    // Run the script
    let script = Script::new(script);
    let script_status = script.run();
    match script_status {
        Ok(()) => {
            println!("{}", "Script executed successfully".green());
        }
        Err(e) => {
            println!("Error: {}", e);
            println!("Script execution aborted.");
        }
    }
}

pub fn handle_shell() {
    println!("Enter command on prompt, or type 'exit' to quit");
    loop {
        let cmd: String = Text::new("").prompt().unwrap();
        match cmd.as_str() {
            "exit" | "quit" => break,
            "" => {}
            _ => {
                let cmd_status = uuu_rs::run_command(&cmd);
                match cmd_status {
                    Ok(()) => {
                        println!("Command executed successfully");
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
        }
    }
}
