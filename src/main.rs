mod app;
mod config;
mod event;
mod package;
mod pacman;
mod ui;

use app::App;
use config::load_config;
use event::{AppEvent, EventHandler};
use std::time::Duration;

fn handle_update() -> anyhow::Result<()> {
    println!("==> Updating Glare...");
    let status = std::process::Command::new("bash")
        .arg("-c")
        .arg("curl -sSL https://raw.githubusercontent.com/TattvaOrg/Glare/main/install.sh | bash")
        .status()?;

    if !status.success() {
        anyhow::bail!("Update failed with exit code: {:?}", status.code());
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        match args[1].as_str() {
            "-v" | "--version" => {
                println!("glare {}", env!("CARGO_PKG_VERSION"));
                return Ok(());
            }
            "-h" | "--help" => {
                println!("glare {}", env!("CARGO_PKG_VERSION"));
                println!("A lightweight system state manager TUI for Arch Linux\n");
                println!("USAGE:");
                println!("    glare [OPTIONS] [COMMAND]\n");
                println!("OPTIONS:");
                println!("    -h, --help       Print help information");
                println!("    -v, --version    Print version information\n");
                println!("COMMANDS:");
                println!("    update           Update glare to the latest release");
                return Ok(());
            }
            "update" => {
                return handle_update();
            }
            unknown => {
                eprintln!("error: unrecognized argument or command '{}'\n", unknown);
                eprintln!("Usage: glare [OPTIONS] [COMMAND]");
                eprintln!("For more information, try '--help'.");
                std::process::exit(1);
            }
        }
    }

    // Load config
    let config = load_config();

    // Initialize terminal
    let mut terminal = ratatui::init();
    terminal.clear()?;

    // Create app
    let mut app = App::new(config)?;

    // Create event handler
    let events = EventHandler::new(Duration::from_millis(250));

    // Main loop
    while !app.should_quit {
        // Render
        terminal.draw(|frame| {
            ui::render(frame, &mut app);
        })?;

        // Handle events
        match events.next()? {
            AppEvent::Key(key) => app.handle_key(key),
            AppEvent::Tick => {}
        }
    }

    // Restore terminal
    ratatui::restore();

    Ok(())
}
