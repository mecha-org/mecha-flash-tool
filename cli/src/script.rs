use colored::Colorize;

#[derive(Debug)]
pub struct Script {
    pub commands: Vec<String>,
    pub uuu_version: Option<String>,
}

impl Script {
    /// Create a new runnable script from a file
    pub fn new(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).unwrap();
        let mut uuu_version = None;
        let commands = contents
            .lines()
            .filter(|s| {
                !s.starts_with("uuu_version") && !s.starts_with('#') && !s.trim().is_empty()
            })
            .map(|s| s.to_string())
            .collect();
        Self {
            commands,
            uuu_version,
        }
    }

    /// Use the image to flash the device
    pub fn with_image(self, image: &str) -> Self {
        let commands = self
            .commands
            .iter()
            .map(|s| {
                if s.contains("_image") {
                    s.replace("_image", image)
                } else {
                    s.to_string()
                }
            })
            .collect();
        let uuu_version = self.uuu_version;
        Self {
            commands,
            uuu_version,
        }
    }

    /// Use the bootloader to flash the device
    pub fn with_bootloader(self, bootloader: &str) -> Self {
        let commands = self
            .commands
            .iter()
            .map(|s| {
                if s.contains("_flash.bin") {
                    s.replace("_flash.bin", bootloader)
                } else {
                    s.to_string()
                }
            })
            .collect();
        let uuu_version = self.uuu_version;
        Self {
            commands,
            uuu_version,
        }
    }

    /// Run the script
    pub fn run(&self) -> Result<(), String> {
        for command in &self.commands {
            println!("> {}", command.cyan());
            let cmd_status = uuu_rs::run_command(command);
            match cmd_status {
                Ok(()) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}
