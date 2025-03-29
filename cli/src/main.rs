use clap::Parser;
use inquire::ui::{RenderConfig, Styled};
use mechaflt::commands::{self, Cli, Commands};

fn main() {
    let render_config = get_render_config();
    inquire::set_global_render_config(render_config);
    let cli = Cli::parse();

    match &cli.command {
        Commands::Devices => commands::handle_devices(),
        Commands::Flash { image } => commands::handle_flash(image),
        Commands::Script { script } => commands::handle_script(script),
        Commands::Shell => commands::handle_shell(),
    }
}

fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new(">");
    render_config
}
