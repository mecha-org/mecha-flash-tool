use clap::Parser;
use mechaflt::commands::{self, Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let mut nt_handler = uuu_rs::notification::NotificationHandler::new();
    uuu_rs::notification::register_notification_callback(&mut nt_handler);

    match &cli.command {
        Commands::Devices => commands::handle_devices(),
        Commands::Flash { image } => commands::handle_flash(image),
        Commands::Script { script } => commands::handle_script(script),
        Commands::Shell => commands::handle_shell(),
    }
}

