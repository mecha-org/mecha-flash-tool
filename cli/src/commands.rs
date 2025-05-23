use crate::script::Script;
use clap::{Parser, Subcommand};
use colored::Colorize;
use inquire::{
    Confirm, InquireError, Text,
    ui::{Color, RenderConfig, Styled},
};
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

/// Manifest schema
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    version: String,
    size: u64,
    sha2: String,
}

/// Machine information schema
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct MachineInfo {
    name: String,
    r#gen: String,
    rev: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Packages {
    linux: Package,
    rootfs: Package,
    uboot: Package,
    dtb: Package,
    mfgtools: Package,
    script: Package,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Manifest {
    id: String,
    version: String,
    channel: String,
    created_at: String,
    description: String,
    url: String,

    machine: MachineInfo,
    packages: Packages,
}

pub fn handle_devices() {
    uuu_rs::devices::print_devices();
}

fn _setup_prompts() -> bool {
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
        return false;
    }

    println!("Searching for device...");
    uuu_rs::devices::print_devices();
    let devices = uuu_rs::devices::get_devices();
    if devices.is_empty() {
        println!("{}", "Check the device connection and try again.".red());
        return false;
    }

    true
}

pub fn handle_flash(image: &str) {
    // Checck if the file exists
    if !std::path::Path::new(image).exists() {
        println!("{} does not exist.", image.red().bold());
        return;
    }

    // Setup prompt
    if !_setup_prompts() {
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
    // TODO: Fix cleanup during force exit
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

    // TODO: Verify the manifest
    // Check if all components are present
    let components = vec![
        &manifest.packages.linux,
        &manifest.packages.rootfs,
        &manifest.packages.uboot,
        &manifest.packages.dtb,
        &manifest.packages.mfgtools,
        &manifest.packages.script,
    ];

    for component in components {
        if !std::path::Path::new(&temp_path.join(&component.name)).exists() {
            println!(
                "{}",
                format!("The package does not contain {}.", component.name).red()
            );
            return;
        } else {
            println!(
                "{}",
                format!("Found {}: {}", component.name, component.version).green()
            );
        }
    }

    // Flash the image
    println!("{}", "Flashing image...".green());

    let current_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_path).unwrap();

    // Run the flashing script
    let script = Script::new(&manifest.packages.script.name)
        .with_image(&manifest.packages.rootfs.name)
        .with_bootloader(&manifest.packages.uboot.name);
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
    // Check if the file exists
    if !std::path::Path::new(script).exists() {
        println!("{} does not exist.", script);
        return;
    }

    // Setup prompt
    if !_setup_prompts() {
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

    let mut last_command_succeeded = true; // Start with default/success state

    loop {
        let prompt_style = if last_command_succeeded {
            Styled::new(">").with_fg(Color::LightGreen)
        } else {
            Styled::new(">").with_fg(Color::LightRed)
        };
        let render_config = RenderConfig::default().with_prompt_prefix(prompt_style);

        let cmd_result = Text::new("").with_render_config(render_config).prompt();
        let cmd = match cmd_result {
            Ok(c) => c,
            Err(InquireError::OperationCanceled) => {
                println!("Operation cancelled.");
                break; // Exit if user cancels (Ctrl+C)
            }
            Err(InquireError::OperationInterrupted) => {
                println!("Operation interrupted.");
                break; // Exit if user interrupts (Ctrl+D often)
            }
            Err(e) => {
                eprintln!("Prompt error: {}", e);
                last_command_succeeded = false;
                continue;
            }
        };

        match cmd.trim() {
            "exit" | "quit" => break,
            "" => {}
            command_to_run => {
                let cmd_status = uuu_rs::run_command(command_to_run);
                match cmd_status {
                    Ok(()) => {
                        last_command_succeeded = true;
                    }
                    Err(e) => {
                        last_command_succeeded = false;
                        println!("Error: {}", e);
                    }
                }
            }
        }
    }
    println!("Exiting shell.");
}
