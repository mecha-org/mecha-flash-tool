use colored::Colorize;

#[derive(Debug)]
pub struct Script {
    pub commands: Vec<String>,
}

impl Script {
    /// Create a new runnable script from a file
    pub fn new(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).unwrap();
        let commands = contents
            .lines()
            .map(|s| {
                if s.starts_with('#') {
                    return String::new();
                }
                s.to_string()
            })
            .collect();
        Self { commands }
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
        Self { commands }
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
        Self { commands }
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
